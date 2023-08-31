extern crate socketcan;

use log::{debug, error, warn};
use embedded_can::{ExtendedId, Frame as EmbeddedFrame, Id, StandardId};
use socketcan::{CanFilter, CanFrame, CanSocket, Socket};
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

#[cxx::bridge(namespace = "org::jcan")]
pub mod ffi {

    #[cxx_name = "Frame"]
    #[derive(Clone)]
    pub struct JFrame {
        id: u32,
        data: Vec<u8>,
    }

    extern "Rust" {
        // #[cxx_name = "Bus"]
        type JBus;

        // #[cxx_name = "new_bus"]
        fn new_jbus() -> Result<Box<JBus>>;

        fn set_id_filter(self: &mut JBus, allowed: Vec<u32>) -> Result<()>;
        fn set_id_filter_mask(self: &mut JBus, allowed: u32, allowed_mask: u32) -> Result<()>;

        fn open(self: &mut JBus, interface: String, tx_queue_len: u16, rx_queue_len: u16) -> Result<()>;
        fn is_open(self: &JBus) -> bool;

        fn receive_from_thread_buffer(self: &mut JBus) -> Result<Vec<JFrame>>;

        fn send(self: &mut JBus, frame: JFrame) -> Result<()>;
        fn receive(self: &mut JBus) -> Result<JFrame>;

        #[cxx_name = "new_frame"]
        fn new_jframe(id: u32, data: Vec<u8>) -> Result<JFrame>;
        fn get_id(self: &JFrame) -> u32;
        fn get_data(self: &JFrame) -> Vec<u8>;
        fn set_id(self: &mut JFrame, id: u32) -> Result<()>;
        fn set_data(self: &mut JFrame, data: Vec<u8>) -> Result<()>;
        fn to_string(self: &JFrame) -> String;

    }

    unsafe extern "C++" {
        include!("jcan/include/callback.h");
        type Bus;
        // fn hello() -> Result<()>;
        // fn hello_bus() -> Result<()>;
    }
}

pub struct JBus {
    // Stores can filters which are passed to either socket, or the socket opened by the spin thread
    filters: Vec<CanFilter>,

    // Setup a MPSC channel which is consumed by the main thread calling bus.spin()
    // The spin_handle is the producer for this channel.
    spin_recv_tx: Option<mpsc::SyncSender<ffi::JFrame>>,
    spin_recv_rx: Option<mpsc::Receiver<ffi::JFrame>>,

    // Setup a MPSC channel which is consumed by the spin thread, sending frames onto the socket.
    spin_send_tx: Option<mpsc::SyncSender<ffi::JFrame>>,

    // The threads are stored in a vector, so they can be joined when the bus is dropped
    spin_handle: Option<thread::JoinHandle<Result<(), std::io::Error>>>,

    // ARC Boolean to communicate if the socket is open, or if we should close it
    socket_opened: Arc<std::sync::atomic::AtomicBool>,
}

// Implements JBus methods
impl JBus {
    // Opens with the socket opened in a background thread - the spin thread
    pub fn open(&mut self, interface: String, tx_queue_size: u16, rx_queue_size: u16) -> Result<(), std::io::Error> {
        // Check if already open
        if self.is_open() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Bus already open",
            ));
        }

        // Create a channel to handle received (inbound) frames
        let (tx, rx) = mpsc::sync_channel(rx_queue_size.into());
        // Store chanel variables
        self.spin_recv_tx = Some(tx);
        self.spin_recv_rx = Some(rx);

        // Create a channel to handle sent (outbound) frames
        let (sendtx, sendrx) = mpsc::sync_channel(tx_queue_size.into());
        // Store channel variables
        self.spin_send_tx = Some(sendtx);

        // Clone the filters list
        let filters = self.filters.clone();
        // Clone the tx channel
        let tx = self.spin_recv_tx.clone().unwrap();
        
        // We will create a Mutex aroud a bool, so we can check if the socket is open
        // The mutex is important because the operation is happening in a new thread - so the main thread
        // needs to wait until the mutex lock is released.
        let socket_error = Arc::new(std::sync::atomic::AtomicBool::new(false));

        // Clone the socket_opened mutex
        let socket_opened_clone = self.socket_opened.clone();
        // Clone the socket_error mutex
        let socket_error_clone = socket_error.clone();

        // Create a thread
        self.spin_handle = Some(thread::Builder::new().name("jcan_spin_thread".to_string()).spawn(move || {

            // Open the socket, and handle the following errors with additional information
            // LookupError(ENODEV) - No such device
            // LookupError(EPERM) - Operation not permitted
            // LookupError(EACCES) - Permission denied
            // LookupError(EBUSY) - Device or resource busy
            let socket = match CanSocket::open(&interface) {
                Ok(s) => {
                    // Set the atomicbool to True
                    socket_opened_clone.store(true, std::sync::atomic::Ordering::Relaxed);
                    s
                },
                Err(e) => {
                    // If the socket fails to open, we will print the error 
                    error!("{}",e.to_string());
                    // Set the socket error atomicbool to True
                    socket_error_clone.store(true, std::sync::atomic::Ordering::Relaxed);
                    return Err(e);
                }
            };

            // Spin thread is actually two threads in one - the receive thread, and the send thread.
            // Each thread has its own loop, which is broken when the socket is closed.
            // The receive thread is responsible for reading frames from the socket, and sending them to the channel.
            // The send thread is responsible for reading frames from the channel, and sending them to the socket.
            // The two threads share the socket via an Arc, so they can both access it.
            // The socket is thread-safe, and can handle multiple threads reading and writing (exclusively) to it.

            // IF the filters list is not empty, set the filters on the socket
            if !filters.is_empty() {
                socket.set_filter_drop_all()?;
                socket.set_filters(&filters)?;
            }

            // Set the socket to blocking operation (reduces CPU usage caused by polling)
            socket.set_nonblocking(false)?;

            // Wrap socket in Arc
            let socket = Arc::new(socket);
            // make a copy of the socket, which will be used for the receive thread
            let socket_recv = Arc::clone(&socket);
            // make a copy of the socket, which will be used for the send thread
            let socket_send = Arc::clone(&socket);

            // Outgoing frames to be sent
            let sendrx = sendrx;

            // Create a thread to handle received frames
            let recv_thread = thread::Builder::new().name("jcan_recv_thread".to_string()).spawn(move || {
                loop {
                    // Read frames
                    match socket_recv.read_frame() {
                        Ok(frame) => {
                            // Convert the CanFrame to a JFrame using into
                            let frame: ffi::JFrame = frame.into();

                            // Send the frame to the channel
                            match tx.send(frame) {
                                Ok(_) => {
                                    // All good!
                                    debug!("jcan_recv_thread queued a frame for next spin()");
                                }
                                Err(e) => {
                                    // This error can only occur if there are no receivers
                                    // If this happens, the main thread has closed, and we should also close
                                    warn!("jcan_recv_thread failed to queue frame: {}",e);
                                }
                            }
                        }
                        Err(e) => {
                            // If the error is not one of
                            // - WouldBlock - The socket is non-blocking, and there are no frames to read
                            // - Error 105 - buffer overflow
                            // Then we will break the loop, and close the thread. 
                            // Otherwise, we will just log the error in a nicely formatted way

                            match e.kind() {
                                std::io::ErrorKind::WouldBlock => {
                                    // Do nothing, repeat loop
                                },
                                // Check if the error kind (of type Os error) is in the list of 
                                // 19 (network down) or 100 (no such device)
                                // This means the socket is closed, break the loop.
                                // Check the os error code
                                _ => {
                                    match e.raw_os_error() {
                                        Some(19) => {
                                            // Network is down
                                            error!("jcan_recv_thread detected network down: {:?}",e);
                                            break;
                                        }
                                        Some(6) => {
                                            // No such device
                                            error!("jcan_recv_thread detected no such device: {:?}",e);
                                            break;
                                        }
                                        Some(100) => {
                                            // No such device
                                            error!("jcan_recv_thread detected no such device: {:?}",e);
                                            break;
                                        }
                                        _ => {
                                            // Any other error, break
                                            warn!("jcan_recv_thread ignored an error: {:?}",e);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            })?;

            // Create a thread to handle sent frames
            let send_thread = thread::Builder::new().name("jcan_send_thread".to_string()).spawn(move || {
                loop {
                    // Blocks until we have something to send
                    match sendrx.recv() {
                        Ok(frame) => {
                            // Convert the JFrame to a CanFrame using into
                            let frame: CanFrame = frame.into();
                            // Write the frame to the socket
                            match socket_send.write_frame(&frame) {
                                Ok(_) => {
                                    // All good!
                                    debug!("jcan_send_thread sent frame: {:?}",frame);
                                }
                                Err(e) => {
                                    // If the error states that the socket closed (OS error 19 or 100)
                                    // Then we will break the loop, and close the thread.
                                    // Otherwise, we will just log the error in a nicely formatted way
                                    match e.raw_os_error() {
                                        Some(6) => {
                                            // Network is down
                                            error!("jcan_send_thread detected network down: {:?}",e);
                                            break;
                                        }
                                        Some(19) => {
                                            // Network is down
                                            error!("jcan_send_thread detected network down: {:?}",e);
                                            break;
                                        }
                                        Some(100) => {
                                            // No such device
                                            error!("jcan_send_thread detected no such device: {:?}",e);
                                            break;
                                        }
                                        _ => {
                                            // Any other error, break
                                            warn!("jcan_send_thread ignored an error: {:?}",e);
                                        }
                                    }
                                }
                            }
                        }
                        // Any error probably means the channel has been closed, so we close the thread
                        Err(e) => {
                            debug!("jcan_send_thread ignored a recv() error on it's MPSC queue: {:?}", e);
                        }
                    }
                }
            })?;
            
            // Join the threads
            recv_thread.join().unwrap();
            send_thread.join().unwrap();

            // Ok
            Ok(())
        })?);

        // Check if spin_handle is finished - this would indicate a thread abort
        debug!("spin_handle: {:?}",self.spin_handle.as_ref().unwrap());

        // Wait until either the socket opened is True, or the socket error is True
        while !self.socket_opened.load(Ordering::Relaxed) && !socket_error.load(Ordering::Relaxed) {
            // Sleep for 1ms
            debug!("Waiting for socket_open or socket_error to be true");
            thread::sleep(Duration::from_millis(10));
        }

        // Check if the socket opened
        if self.socket_opened.load(Ordering::Relaxed) {
            // Ok
            Ok(())
        } else {
            // Error
            Err(std::io::Error::new(std::io::ErrorKind::Other,"Error opening bus",))
        }
    }

    // Check if the thread_handle is not empty.
    // If thread_handle is not empty, we assumed we have been opened
    pub fn is_open(&self) -> bool {
        self.spin_handle.is_some() && self.socket_opened.load(Ordering::Relaxed)
    }

    // Close the bus
    pub fn close(&mut self) -> Result<(), std::io::Error> {
        // Check if we are open
        if !self.is_open() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Cannot close, bus has not been opened",
            ));
        }

        // Get socket_opened arc
        let socket_opened = self.socket_opened.clone();
        // Set socket_opened to false
        socket_opened.store(false, Ordering::Relaxed);

        // Ok
        Ok(())
    }

    // Blocks until a frame is received. Behind the scenes, uses a channel to receive frames via spin thread.
    // WARNING: This will prevent frames from being handled in the callbacks.
    pub fn receive(&mut self) -> Result<ffi::JFrame, std::io::Error> {
        // Check if we are open
        if !self.is_open() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Cannot receive, bus has not been opened",
            ));
        }

        // Clone the spin_recv_rx channel
        let rx = self.spin_recv_rx.as_ref().unwrap().clone();
        // Receive a frame
        let frame = rx.recv().unwrap();
        // Return the frame
        return Ok(frame);
    }

    // Blocks until a frame is sent
    pub fn send(&mut self, frame: ffi::JFrame) -> Result<(), std::io::Error> {
        // Check if we are open
        if !self.is_open() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Cannot send, bus has not been opened",
            ));
        }

        // Make clone of the channel
        let tx = self.spin_send_tx.clone().unwrap();

        // Send the frame - this will BLOCK until the frame is sent onto the internal queue.
        // An error is ONLY raised if the reciever has been dropped. 

        // Send the frame
        match tx.send(frame) {
            Ok(_) => {
                // All good!
                debug!("queued outgiong frame for transmission");
                Ok(())
            }
            Err(e) => {
                // Error message
                error!("send error, jcan_send_thread crashed: {:?}",e);
                // Close the bus so that the other methods stop doing tstuff
                // This is done by the close method
                self.close()?;
                Ok(())
            }
        }
    }

    // Set the list of IDs that will be received
    // This filter guarantees that all the ALLOWED frames will be received
    pub fn set_id_filter(&mut self, allowed: Vec<u32>) -> Result<(), std::io::Error> {
        // If we are open, return an error
        if self.is_open() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Set frame ID filter before opening bus",
            ));
        }

        // Create a vector of CanFilters
        let mut filters = Vec::new();
        // Loop through the allowed IDs
        for id in allowed {
            // Create a CanFilter for each ID, for STANDARD IDs only.
            let filter = CanFilter::new(id, 0x7FF);
            // Push it to the vector
            filters.push(filter);
        }

        // Set the filter so it can be used during socket open
        self.filters = filters.clone();

        Ok(())
    }

    // Directly sets the ID filter via mask
    pub fn set_id_filter_mask(&mut self, allowed: u32, allowed_mask: u32) -> Result<(), std::io::Error> {
        // If we are open, return an error
        if self.is_open() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Set frame ID filter before opening bus",
            ));
        }

        // Create a vector of CanFilters
        let mut filters = Vec::new();

        // Filters will allow an incoming packed if it passes the condition
        // receive_id & mask == filtered_id & mask
        // By setting a filtered_id of 0xFFFF, and a restrictive mask (of, say, 0x3),
        // The filter will accept ALL frames that have a binary '1' in the lower two bits.
        let filter = CanFilter::new(allowed, allowed_mask);

        filters.push(filter);

        // Set the filter so it can be used during socket open
        self.filters = filters.clone();

        Ok(())
    }

    // bus.spin() is the consumer of the mpsc channel (rx), and is what calls the callbacks!
    pub fn receive_from_thread_buffer(&mut self) -> Result<Vec<ffi::JFrame>, std::io::Error> {
        // Check if we are open
        if !self.is_open() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Bus not open",
            ));
        }

        // We know that the self.callbacks list will not change while we are spinning,
        // because we are not allowed to register callbacks after the bus is opened.

        // Make a copy of the frames we have received
        // This prevents spin() from running for a very long time, if many frames keep being received
        let frames = self
            .spin_recv_rx
            .as_ref()
            .unwrap()
            .try_iter()
            .collect::<Vec<ffi::JFrame>>();

        Ok(frames)
    }
}

// Builder for JBus, used to create C++ instances of the opaque JBus type
// Takes in a String interface
pub fn new_jbus() -> Result<Box<JBus>, std::io::Error> {
    // Initialise the logger, if it hasn't already been initialised
    // This is done by the first call to new_jbus()
    match env_logger::builder().filter_level(log::LevelFilter::Warn).try_init() {
        Ok(_) => {}
        Err(_) => {
            warn!("env_logger already initialised");
        }
    }

    // Create a new JBus
    let jbus = JBus {
        filters: Vec::new(),
        // callbacks: Vec::new(),
        spin_handle: None,
        spin_recv_tx: None,
        spin_recv_rx: None,
        spin_send_tx: None,
        socket_opened: Arc::new(std::sync::atomic::AtomicBool::new(false)),
    };

    // Return the JBus
    Ok(Box::new(jbus))
}

// Builder for jframe, used to create C++ instances of the shared JFrame type
pub fn new_jframe(id: u32, data: Vec<u8>) -> Result<ffi::JFrame, std::io::Error> {
    let frame = ffi::JFrame::build(id, data)?;
    Ok(frame)
}

impl ffi::JFrame {
    pub fn build(id: u32, data: Vec<u8>) -> Result<ffi::JFrame, std::io::Error> {
        let mut frame = ffi::JFrame {
            id: 0,
            data: Vec::new(),
        };

        // Set id and data
        frame.set_id(id)?;
        frame.set_data(data)?;

        Ok(frame)
    }

    // .id setter
    pub fn set_id(&mut self, id: u32) -> Result<(), std::io::Error> {
        self.id = id;
        Ok(())
    }

    // .id getter
    pub fn get_id(&self) -> u32 {
        self.id
    }

    // .data setter
    pub fn set_data(&mut self, data: Vec<u8>) -> Result<(), std::io::Error> {
        // Check if data is too long
        if data.len() > 8 {
            // Print error message that shows N > 8 bytes
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Data length {} > 8 bytes", data.len()),
            ));
        }

        self.data = data.clone();
        Ok(())
    }

    // .data getter
    pub fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn to_string(&self) -> String {
        // Prints in the same format as CANDUMP
        // e.g: vcan0  123   [2]  10 20
        let mut s = String::new();
        s.push_str(&format!("0x{:03X}   [{}]  ", self.id, self.data.len()));
        for byte in self.data.iter() {
            s.push_str(&format!("{:02X} ", byte));
        }
        s
    }
}

// Implement Display for JFrame, used for Rust only
impl std::fmt::Display for ffi::JFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Use the to_string() function to print the JFrame
        write!(f, "{}", self.to_string()).unwrap();
        Ok(())
    }
}

// Implement From<CanFrame> for ffi::JFrame
impl From<CanFrame> for ffi::JFrame {
    fn from(frame: CanFrame) -> Self {
        ffi::JFrame {
            id: match frame.id() {
                Id::Standard(id) => id.as_raw().into(),
                Id::Extended(id) => id.as_raw(),
            },
            data: frame.data().to_vec(),
        }
    }
}

// Implement Into<CanFrame> for ffi::JFrame
impl Into<CanFrame> for ffi::JFrame {
    fn into(self) -> CanFrame {
        // First check if id needs to be Standard or Extended
        let id = if self.id > 0x7FF {
            Id::Extended(ExtendedId::new(self.id).unwrap())
        } else {
            Id::Standard(StandardId::new(self.id as u16).unwrap())
        };

        // Convert the JFrame to a CanFrame
        CanFrame::new(id, self.data.as_slice()).unwrap()
    }
}
