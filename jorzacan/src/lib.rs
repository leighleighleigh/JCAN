extern crate socketcan;
extern crate pyo3;
use std::ffi::*;

use embedded_can::{blocking::Can, Frame as EmbeddedFrame, Id, StandardId, ExtendedId};
use socketcan::{CanFrame, CanSocket, CanSocketOpenError, CanError, Socket};
use pyo3::exceptions::{PyOSError};
use pyo3::prelude::*;
use pyo3::types::{PyModule};
use pyo3::{PyResult};

#[cxx::bridge(namespace = "org::jorzacan")]
mod ffi {

    #[cxx_name = "Frame"]
    #[derive(Clone)]
    struct JorzaFrame {
        id: u32,
        data: Vec<u8>,
    }

    #[derive(Debug)]
    struct JorzaError {
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


struct JorzaBus {
    socket: CanSocket,
}

// Implements JorzaBus methods
impl JorzaBus {

    // Blocks until a frame is received
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
        };
        Ok(frame)
    }

    // Blocks until a frame is sent
    fn send(&mut self, frame: ffi::JorzaFrame) -> Result<(), std::io::Error> {
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
fn new_jorzabus(interface: String) -> Box<JorzaBus> {
    // If an error is caught here, it will be propagated to the C++ side
    // as a std::runtime_error, with the message "CanSocketOpenError"
    // TODO: Make this fail gracefully on C++
    let socket = CanSocket::open(&interface).expect("CanSocketOpenError");
    Box::new(JorzaBus { socket })
}

// Builder for jorzaframe, used to create C++ instances of the opaque JorzaFrame type
// Takes in a u32 id, and a Vec<u8> data
fn new_jorzaframe(id: u32, data: Vec<u8>) -> Result<ffi::JorzaFrame, std::io::Error> {
    Ok(ffi::JorzaFrame {
        id,
        data,
    })
}

// Implement a to_string method for JorzaFrame, returning String, used for C++ and Rust
impl ffi::JorzaFrame {
    fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("ID: 0x{:X} Data: ", self.id));
        for byte in self.data.iter() {
            s.push_str(&format!("{:X} ", byte));
        }
        s
    }
}


#[pyclass]
#[pyo3{name = "Bus"}]
struct PyJorzaBus {
    bus: JorzaBus,
}

#[pyclass]
#[pyo3{name = "Frame"}]
#[derive(Clone)]
struct PyJorzaFrame {
    frame: ffi::JorzaFrame,
}

// Implement the 'new' method for the PyJorzaBus, which makes a call to new_jorzabus
#[pymethods]
impl PyJorzaBus {
    #[new]
    fn new(interface: String) -> PyResult<Self> {
        Ok(PyJorzaBus {
            bus: *new_jorzabus(interface),
        })
    }

    // Implement the receive method for the PyJorzaBus
    fn receive(&mut self) -> PyResult<PyJorzaFrame> {
        let frame = self.bus.receive().map_err(|e| {
            PyOSError::new_err(format!("Error receiving frame: {}", e))
        })?;
        Ok(PyJorzaFrame {
            frame,
        })
    }

    // Implement the send method for the PyJorzaBus
    fn send(&mut self, frame: PyJorzaFrame) -> PyResult<()> {
        self.bus.send(frame.frame).map_err(|e| {
            PyOSError::new_err(format!("Error sending frame: {}", e))
        })?;
        Ok(())
    }
}

// Implement the 'new' method for the PyJorzaFrame, which makes a cal to new_jorzaframe
#[pymethods]
impl PyJorzaFrame {
    #[new]
    fn new(id: u32, data: Vec<u8>) -> PyResult<Self> {
        Ok(PyJorzaFrame {
            frame: new_jorzaframe(id, data).map_err(|e| {
                PyOSError::new_err(format!("Error creating frame: {}", e))
            })?,
        })
    }

    // Implement string representation using the to_string method
    fn __str__(&self) -> PyResult<String> {
        Ok(self.frame.to_string())
    }
}


#[pymodule]
fn jorzacan(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyJorzaBus>()?;
    m.add_class::<PyJorzaFrame>()?;

    Ok(())
}