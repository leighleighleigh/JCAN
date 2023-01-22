extern crate socketcan;

use embedded_can::{Frame as EmbeddedFrame, Id, StandardId, ExtendedId};
use socketcan::{CanFrame, CanSocket, Socket};


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
    }

    unsafe extern "C++" {
    }
}

pub struct JBus {
    socket: CanSocket,
}

// Implements JBus methods
impl JBus {

    // Blocks until a frame is received
    pub fn receive(&mut self) -> Result<ffi::JFrame, std::io::Error> {
        let frame = self.socket.read_frame()?;
        // Convert the embedded_can::Frame to a JFrame
        let frame = ffi::JFrame {
            // Need to map Id to Extended or Standard, then convert to u32 using as_raw()
            id: match frame.id() {
                Id::Standard(id) => id.as_raw().into(),
                Id::Extended(id) => id.as_raw(),
            },
            data: frame.data().to_vec(),
        };
        Ok(frame)
    }
    
    // Keeps receiving frames, returning the first frame with the given id
    pub fn receive_with_id(&mut self, id: u32) -> Result<ffi::JFrame, std::io::Error> {
        loop {
            let frame = self.receive()?;
            if frame.id == id {
                return Ok(frame);
            }
        }
    }

    // Blocks until a frame is sent
    pub fn send(&mut self, frame: ffi::JFrame) -> Result<(), std::io::Error> {
        // First check if id needs to be Standard or Extended
        let id = if frame.id > 0x7FF {
            Id::Extended(ExtendedId::new(frame.id).unwrap())
        } else {
            Id::Standard(StandardId::new(frame.id as u16).unwrap())
        };

        // Convert the JFrame to a CanFrame
        let frame = CanFrame::new(
            id,
            frame.data.as_slice(),
        ).unwrap();

        // Send it!
        self.socket.write_frame(&frame)?;
        Ok(())
    }
}


// Public 'builder' method used to create C++ instances of the opaque JBus type
pub fn new_jbus(interface: String) -> Result<Box<JBus>, std::io::Error> {
    // Open and map to Error if it fails
    let socket = CanSocket::open(&interface).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(Box::new(JBus {
        socket,
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