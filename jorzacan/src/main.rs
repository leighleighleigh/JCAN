
extern crate socketcan;
use embedded_can::{blocking::Can, Frame as EmbeddedFrame, Id, StandardId, ExtendedId};
use ffi::{JorzaError, JorzaBus};
use socketcan::{CanFrame, CanSocket, CanSocketOpenError, CanError, Socket};

pub struct JorzaSocket {
    socket: CanSocket,
}

#[cxx::bridge(namespace = "org::jorzacan")]
mod ffi {
    
    pub struct JorzaBus {
        iface: String,
        socketptr: Box<JorzaSocket>,
    }

    pub struct JorzaFrame {
        // Same fields as CanFrame
        id: u32,
        data: Vec<u8>,
        dlc: u8,
    }

    #[derive(Debug)]
    pub struct JorzaError {
        message: String,
    }

    extern "Rust" {
        type JorzaSocket;
        fn jorzabus_open(interface: &str) -> Result<JorzaBus>;
        fn receive(self: &mut JorzaBus) -> Result<JorzaFrame>;
    }

    unsafe extern "C++" {
    }
}

// Opens CanSocket 
pub fn jorzabus_open(interface: &str) -> Result<JorzaBus, JorzaError> {
    // Builds a new JorzaBus
    let mut bus = JorzaBus {
        iface: interface.to_string(),
        socketptr: Box::new(JorzaSocket { socket: CanSocket::open(interface).map_err(|e| JorzaError { message: e.to_string() })? }),
    };
    Ok((bus))
}


// Implement public method for reading on the bus
impl ffi::JorzaBus {
    // Gets JorzaFrames
    pub fn receive(&mut self) -> Result<ffi::JorzaFrame, JorzaError> {
        // Get frame from socket
        let frame = self.socketptr.socket.read_frame().map_err(|e| JorzaError { message: e.to_string() })?;

        // Convert to JorzaFrame
        // First convert id to u32, requiring try_into
        let id = match frame.id() {
            Id::Standard(id) => id.as_raw().into(),
            Id::Extended(id) => id.as_raw().into(),
        };

        let data = frame.data().to_vec();
        let dlc : u8 = frame.dlc().try_into().unwrap();

        // Return frame, or error 
        Ok(ffi::JorzaFrame { id, data, dlc })
    }
}

// Implement Display for JorzaFrame
impl std::fmt::Display for ffi::JorzaFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "JorzaFrame {{ id: {}, data: {:?}, dlc: {} }}", self.id, self.data, self.dlc)
    }
}

// Implement Display for JorzaError
impl std::fmt::Display for JorzaError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "JorzaError {{ message: {} }}", self.message)
    }
}

pub fn main () {
    let mut bus = jorzabus_open("vcan0").unwrap();
    let frame = bus.receive().unwrap();
    println!("Frame: {}", frame);
}