#[allow(unused_imports)]
extern crate socketcan;
extern crate pyo3;

#[allow(unused_imports)]

use embedded_can::{blocking::Can, Frame as EmbeddedFrame, Id, StandardId, ExtendedId};
use pyo3::exceptions::{PyOSError};
use pyo3::prelude::*;
use pyo3::types::{PyModule};
use pyo3::{PyResult};
use socketcan::{CanFrame, CanSocket, Frame, Socket, CanSocketOpenError};

// Create custom Python exception to handle CanSocket errors
pyo3::create_exception!(jorzacan, CanSocketError, PyOSError);

// Public function open() to open CAN socket
pub fn open(iface: &str) -> Result<CanSocket, CanSocketOpenError> {
    CanSocket::open(iface)
}

pub fn get_raw_id(id: &Id) -> u32 {
    match id {
        Id::Standard(id) => id.as_raw() as u32,
        Id::Extended(id) => id.as_raw(),
    }
}

pub fn frame_to_string<F: Frame>(f: &F) -> String {
    let id = get_raw_id(&f.id());
    let data_string = f
        .data()
        .iter()
        .fold(String::from(""), |a, b| format!("{} {:02x}", a, b));

    format!("{:08X}  [{}] {}", id, f.dlc(), data_string)
}

#[pymodule]
fn jorzacan(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyclass]
    struct PyCanSocket {
        socket: CanSocket,
    }

    #[pymethods]
    impl PyCanSocket {
        #[new]
        fn new(iface: &str) -> PyResult<Self> {
            // Open CAN socket, return error as CanSocketError to variable
            //let socket = CanSocket::open(iface).map_err(|e| CanSocketError::new_err(e.to_string()))?;
            // same as above, but uses the public open() function
            let socket = open(iface).map_err(|e| CanSocketError::new_err(e.to_string()))?;

            // Returns new instance of PyCanSocket, with socket variable set to the opened socket
            Ok(Self { socket })
        }

        fn send(&mut self, id: u32, data: &[u8]) -> PyResult<()> {
            // If id is 11 bits, use StandardId, otherwise use ExtendedId

            let frame_id : Id = match id.try_into() {
                Ok(id) => embedded_can::Id::Standard(StandardId::new(id).unwrap()),
                Err(_) => embedded_can::Id::Extended(ExtendedId::new(id).unwrap()),
            };

            let write_frame = CanFrame::new(frame_id, data)
                .expect("Failed to create CAN frame");

            self.socket
                .transmit(&write_frame)
                .expect("Failed to transmit frame");

            Ok(())
        }

        fn receive(&mut self) -> PyResult<String> {
            let frame = self.socket.receive();

            if let Ok(frame) = frame {
                Ok(frame_to_string(&frame))
            } else {
                Ok("".to_string())
            }
        }
    }

    m.add("CanSocketError", _py.get_type::<CanSocketError>())?;
    m.add_class::<PyCanSocket>()?;

    Ok(())
}