// Separate library for the Python wrappers, using 
// pyo3 and the maturin build system.
// This is built separately to the main library, so that
// the libjorzacan.a file isn't bloated with python junk.
// It also helps the cxx system not get confused.
extern crate pyo3;
extern crate jorzacan;

use pyo3::exceptions::{PyOSError};
use pyo3::prelude::*;
use pyo3::types::{PyModule};
use pyo3::{PyResult};

use jorzacan::*;

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
fn pyjorzacan(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyJorzaBus>()?;
    m.add_class::<PyJorzaFrame>()?;

    Ok(())
}