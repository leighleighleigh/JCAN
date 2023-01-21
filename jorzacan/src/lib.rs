extern crate socketcan;

use embedded_can::{Frame as EmbeddedFrame, Id, StandardId, ExtendedId};
use socketcan::{CanFrame, CanSocket, Socket};


#[cxx::bridge(namespace = "org::jorzacan")]
pub mod ffi {

    #[cxx_name = "Frame"]
    #[derive(Clone)]
    pub struct JorzaFrame {
        id: u32,
        data: Vec<u8>,
    }

    #[derive(Debug)]
    pub struct JorzaError {
        message: String,
    }

    extern "Rust" {
        #[cxx_name = "Bus"]
        type JorzaBus;
        #[cxx_name = "open_bus"]
        fn new_jorzabus(interface: String) -> Box<JorzaBus>;
        fn receive(self: &mut JorzaBus) -> Result<JorzaFrame>;
        fn send(self: &mut JorzaBus, frame: JorzaFrame) -> Result<()>;
        fn new_jorzaframe(id: u32, data: Vec<u8>) -> Result<JorzaFrame>;
        fn to_string(self: &JorzaFrame) -> String;
    }

    unsafe extern "C++" {
    }
}

pub struct JorzaBus {
    socket: CanSocket,
}

// Implements JorzaBus methods
impl JorzaBus {

    // Blocks until a frame is received
    pub fn receive(&mut self) -> Result<ffi::JorzaFrame, std::io::Error> {
        let frame = self.socket.read_frame()?;
        // Convert the embedded_can::Frame to a JorzaFrame
        let frame = ffi::JorzaFrame {
            // Need to map Id to Extended or Standard, then convert to u32 using as_raw()
            id: match frame.id() {
                Id::Standard(id) => id.as_raw().into(),
                Id::Extended(id) => id.as_raw(),
            },
            data: frame.data().to_vec(),
        };
        Ok(frame)
    }

    // Blocks until a frame is sent
    pub fn send(&mut self, frame: ffi::JorzaFrame) -> Result<(), std::io::Error> {
        // First check if id needs to be Standard or Extended
        let id = if frame.id > 0x7FF {
            Id::Extended(ExtendedId::new(frame.id).unwrap())
        } else {
            Id::Standard(StandardId::new(frame.id as u16).unwrap())
        };

        // Convert the JorzaFrame to a CanFrame
        let frame = CanFrame::new(
            id,
            frame.data.as_slice(),
        ).unwrap();

        // Send it!
        self.socket.write_frame(&frame)?;
        Ok(())
    }
}

// Implement Display for JorzaError
impl std::fmt::Display for ffi::JorzaError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

// Public 'builder' method used to create C++ instances of the opaque JorzaBus type
pub fn new_jorzabus(interface: String) -> Box<JorzaBus> {
    // If an error is caught here, it will be propagated to the C++ side
    // as a std::runtime_error, with the message "CanSocketOpenError"
    // TODO: Make this fail gracefully on C++
    let socket = CanSocket::open(&interface).expect("CanSocketOpenError");
    Box::new(JorzaBus { socket })
}

// Builder for jorzaframe, used to create C++ instances of the opaque JorzaFrame type
// Takes in a u32 id, and a Vec<u8> data
pub fn new_jorzaframe(id: u32, data: Vec<u8>) -> Result<ffi::JorzaFrame, std::io::Error> {
    Ok(ffi::JorzaFrame {
        id,
        data,
    })
}

// Implement a to_string method for JorzaFrame, returning String, used for C++ and Rust
impl ffi::JorzaFrame {
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("ID: 0x{:X} Data: ", self.id));
        for byte in self.data.iter() {
            s.push_str(&format!("{:X} ", byte));
        }
        s
    }
}