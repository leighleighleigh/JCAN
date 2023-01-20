
extern crate socketcan;
use embedded_can::{blocking::Can, Frame as EmbeddedFrame, Id, StandardId, ExtendedId};
use socketcan::{CanFrame, CanSocket, CanSocketOpenError, CanError, Socket};

#[cxx::bridge(namespace = "org::jorzacan")]
mod ffi {

    struct JorzaFrame {
        id: u32,
        data: Vec<u8>,
        dlc: u8,
    }

    #[derive(Debug)]
    struct JorzaError {
        message: String,
    }

    extern "Rust" {
        type JorzaBus;
        fn new_jorzabus(interface: String) -> Box<JorzaBus>;
        fn receive(self: &mut JorzaBus) -> Result<JorzaFrame>;
    }

    unsafe extern "C++" {
    }
}

struct JorzaBus {
    socket: CanSocket,
}

// Implements JorzaBus methods
impl JorzaBus {
    fn receive(&mut self) -> Result<ffi::JorzaFrame, std::io::Error> {
        let frame = self.socket.read_frame()?;
        // Convert the embedded_can::Frame to a JorzaFrame
        let frame = ffi::JorzaFrame {
            // Need to map Id to Extended or Standard, then convert to u32 using as_raw()
            id: match frame.id() {
                Id::Standard(id) => id.as_raw().into(),
                Id::Extended(id) => id.as_raw(),
            },
            data: frame.data().to_vec(),
            dlc: frame.dlc() as u8
        };
        Ok(frame)
    }
}

// Implement Display for JorzaError
impl std::fmt::Display for ffi::JorzaError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

// Public 'builder' method used to create C++ instances of the opaque JorzaBus type
fn new_jorzabus(interface: String) -> Box<JorzaBus> {
    // If an error is caught here, it will be propagated to the C++ side
    // as a std::runtime_error, with the message "CanSocketOpenError"
    // TODO: Make this fail gracefully on C++
    let socket = CanSocket::open(&interface).expect("CanSocketOpenError");
    Box::new(JorzaBus { socket })
}