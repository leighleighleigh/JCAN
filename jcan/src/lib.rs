extern crate socketcan;

use cxx::private::UniquePtrTarget;
use embedded_can::{Frame as EmbeddedFrame, Id, StandardId, ExtendedId};
use socketcan::{CanFrame, CanSocket, Socket, CanFilter};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use cxx::UniquePtr;

#[cxx::bridge(namespace = "org::jcan")]
pub mod ffi {

    #[cxx_name = "Frame"]
    #[derive(Clone)]
    pub struct JFrame {
        id: u32,
        data: Vec<u8>,
    }


    extern "Rust" {
        #[cxx_name = "Bus"]
        type JBus;
        #[cxx_name = "open_bus"]
        fn new_jbus(interface: String) -> Result<Box<JBus>>;
        fn receive(self: &mut JBus) -> Result<JFrame>;
        fn receive_with_id(self: &mut JBus, id: u32) -> Result<JFrame>;
        fn send(self: &mut JBus, frame: JFrame) -> Result<()>;
        fn new_jframe(id: u32, data: Vec<u8>) -> Result<JFrame>;
        fn to_string(self: &JFrame) -> String;
        fn set_id_filter(self: &mut JBus, allowed: Vec<u32>) -> Result<()>;
        fn clear_id_filter(self: &mut JBus) -> Result<()>;
        fn receive_nonblocking(self: &mut JBus) -> Result<Vec<JFrame>>;
    }

    unsafe extern "C++" {
        include!("jcan/src/callback.h");
        type JCANFrameCallback;
        // fn new_jcanframecallback(callback: fn(JFrame)) -> UniquePtr<JCANFrameCallback>;
        // fn execute_callback(callback: &JCANFrameCallback, frame: JFrame);
    }
}

pub struct JBus {
    socket: CanSocket,
    // Setup a MPSC channel which is consumed by the main thread calling bus.spin()
    // Multiple threads can send to the channel, each listening for a specific ID
    // Having these threads sit in the background mean we don't waste CPU cycles
    // These are optional, and are only created if the user calls bus.spin()
    tx : Option<mpsc::Sender<(u32, ffi::JFrame)>>,
    rx : Option<mpsc::Receiver<(u32, ffi::JFrame)>>,

    // The threads are stored in a vector, so they can be joined when the bus is dropped
    thread_handle: Vec<thread::JoinHandle<()>>,

    // Callbacks functions are stored in a vector, so they can be called when a frame is received
    // The first element of the tuple is the ID, the second is the callback function
    // The callback function is an Opaque type, which surrounds a C++ functor. This is because we cannot
    // directly pass function pointers from C++ to Rust, so we have to wrap them in a functor.
    // The function signature is void(*)(Frame)
    callbacks: Vec<(u32, ffi::JCANFrameCallback)>,
}

// Implements JBus methods
impl JBus {
    // Blocks until a frame is received
    pub fn receive(&mut self) -> Result<ffi::JFrame, std::io::Error> {
        let frame = self.socket.read_frame()?;
        // Convert the CanFrame to a JFrame using into
        let frame: ffi::JFrame = frame.into();
        Ok(frame)
    }
    
    // Unlike bus.set_filter, this method operates on a single ID, and doesn't prevent other IDs from being received
    pub fn receive_with_id(&mut self, id: u32) -> Result<ffi::JFrame, std::io::Error> {
        loop {
            let frame = self.receive()?;
            if frame.id == id {
                return Ok(frame);
            }
        }
    }

    // Non-blocking receive, returns a vector of frames, if any are available!
    pub fn receive_nonblocking(&mut self) -> Result<Vec<ffi::JFrame>, std::io::Error> {
        // Set self to non-blocking
        self.socket.set_nonblocking(true)?;
        // Create a vector to store frames
        let mut frames = Vec::new();
        // Loop until we get an error
        loop {
            match self.receive() {
                Ok(frame) => frames.push(frame),
                Err(e) => {
                    // If the error is a WouldBlock, we're done
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        break;
                    }
                    // Otherwise, return the error
                    return Err(e);
                }
            }
        }
        // Set self back to blocking
        self.socket.set_nonblocking(false)?;
        // Return the vector of frames
        Ok(frames)
    }

    // Blocks until a frame is sent
    pub fn send(&mut self, frame: ffi::JFrame) -> Result<(), std::io::Error> {
        // Convert frame into CanFrame
        let frame: CanFrame = frame.into(); 
        // Send it!
        self.socket.write_frame(&frame)?;
        Ok(())
    }

    // Set the list of IDs that will be received
    // This filter guarantees that all the ALLOWED frames will be received
    pub fn set_id_filter(&mut self, allowed: Vec<u32>) -> Result<(), std::io::Error> {
        // Create a vector of CanFilters
        let mut filters = Vec::new();
        // Loop through the allowed IDs
        for id in allowed {
            // Create a CanFilter for each ID, for STANDARD IDs only.
            let filter = CanFilter::new(id, 0x7FF);
            // Push it to the vector
            filters.push(filter);
        }
        // Disable the default filter
        self.socket.filter_drop_all()?;
        // Set the filter
        self.socket.set_filters(&filters)?;
        // Set the filter
        Ok(())
    }

    // Clear the list of IDs that will be received
    // This means ALL frames will be received
    pub fn clear_id_filter(&mut self) -> Result<(), std::io::Error> {
        // Disable the default filter
        self.socket.filter_accept_all()?;
        // Set the filter
        Ok(())
    }

}


// Public 'builder' method used to create C++ instances of the opaque JBus type
pub fn new_jbus(interface: String) -> Result<Box<JBus>, std::io::Error> {
    // Open and map to Error if it fails
    let socket = CanSocket::open(&interface).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(Box::new(JBus {
        socket,
        tx: None,
        rx: None,
        thread_handle: Vec::new(),
    }))
}

// Builder for jframe, used to create C++ instances of the opaque JFrame type
// Takes in a u32 id, and a Vec<u8> data
pub fn new_jframe(id: u32, data: Vec<u8>) -> Result<ffi::JFrame, std::io::Error> {
    // Check if data is empty (zero length)
    if data.len() == 0 {
        // Print error message that shows N = 0 bytes
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Data length cannot be 0 bytes"));
    }

    // Check if data is too long
    if data.len() > 8 {
        // Print error message that shows N > 8 bytes
        return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Data length {} > 8 bytes", data.len())));
    }
    Ok(ffi::JFrame {
        id,
        data,
    })
}

// Implement a to_string method for JFrame, returning String, used for C++ and Rust
impl ffi::JFrame {
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("ID: 0x{:X} Data: ", self.id));
        for byte in self.data.iter() {
            s.push_str(&format!("{:X} ", byte));
        }
        s
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
        CanFrame::new(
            id,
            self.data.as_slice(),
        ).unwrap()
    }
}
