// Separate library for the Python wrappers, using 
// pyo3 and the maturin build system.
// This is built separately to the main library, so that
// the libjcan.a file isn't bloated with python junk.
// It also helps the cxx system not get confused.
extern crate pyo3;
extern crate jcan;

use pyo3::exceptions::{PyOSError};
use pyo3::prelude::*;
use pyo3::types::{PyModule};
use pyo3::{PyResult};

use jcan::*;

#[pyclass]
#[pyo3{name = "Bus"}]
struct PyJBus {
    bus: JBus,
}

#[pyclass]
#[pyo3{name = "Frame"}]
#[derive(Clone)]
struct PyJFrame {
    frame: ffi::JFrame,
}

// Implement the 'new' method for the PyJBus, which makes a call to new_jbus
#[pymethods]
impl PyJBus {
    #[new]
    fn new(interface: String) -> PyResult<Self> {
        // Unbox the result of new_jbus, and return it
        let bus = new_jbus(interface).map_err(|e| {
            PyOSError::new_err(format!("Error opening bus: {}", e))
        })?;

        // bus is a Box<JBus>, so we need to dereference it
        Ok(PyJBus {
            bus: *bus,
        })
    }

    // Implement the receive method for the PyJBus
    fn receive(&mut self) -> PyResult<PyJFrame> {
        let frame = self.bus.receive().map_err(|e| {
            PyOSError::new_err(format!("Error receiving frame: {}", e))
        })?;
        Ok(PyJFrame {
            frame,
        })
    }

    // Implement the receive_with_id method for the PyJBus
    fn receive_with_id(&mut self, id: u32) -> PyResult<PyJFrame> {
        let frame = self.bus.receive_with_id(id).map_err(|e| {
            PyOSError::new_err(format!("Error receiving frame: {}", e))
        })?;
        Ok(PyJFrame {
            frame,
        })
    }

    // Implement the send method for the PyJBus
    fn send(&mut self, frame: PyJFrame) -> PyResult<()> {
        self.bus.send(frame.frame).map_err(|e| {
            PyOSError::new_err(format!("Error sending frame: {}", e))
        })?;
        Ok(())
    }
}

// Implement the 'new' method for the PyJFrame, which makes a cal to new_jframe
#[pymethods]
impl PyJFrame {
    #[new]
    fn new(id: u32, data: Vec<u8>) -> PyResult<Self> {
        Ok(PyJFrame {
            frame: new_jframe(id, data).map_err(|e| {
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
fn jcan_python(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyJBus>()?;
    m.add_class::<PyJFrame>()?;

    Ok(())
}
