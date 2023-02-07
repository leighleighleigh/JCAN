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

    // Implement receive_any, which returns a list of frames
    fn receive_nonblocking(&mut self) -> PyResult<Vec<PyJFrame>> {
        let frames = self.bus.receive_nonblocking().map_err(|e| {
            PyOSError::new_err(format!("Error receiving frames: {}", e))
        })?;
        let mut py_frames = Vec::new();
        for frame in frames {
            py_frames.push(PyJFrame {
                frame,
            });
        }
        Ok(py_frames)
    }

    // Implement the send method for the PyJBus
    fn send(&mut self, frame: PyJFrame) -> PyResult<()> {
        self.bus.send(frame.frame).map_err(|e| {
            PyOSError::new_err(format!("Error sending frame: {}", e))
        })?;
        Ok(())
    }

    // Implement set_id_filter for the PyJBus, which takes a list of IDs
    fn set_id_filter(&mut self, ids: Vec<u32>) -> PyResult<()> {
        self.bus.set_id_filter(ids).map_err(|e| {
            PyOSError::new_err(format!("Error setting ID filter: {}", e))
        })?;
        Ok(())
    }

    // Implement clear_id_filter, which takes no arguments
    fn clear_id_filter(&mut self) -> PyResult<()> {
        self.bus.clear_id_filter().map_err(|e| {
            PyOSError::new_err(format!("Error clearing ID filter: {}", e))
        })?;
        Ok(())
    }
}

// Implement the 'new' method for the PyJFrame, which makes a cal to new_jframe
#[pymethods]
impl PyJFrame {
    #[new]
    fn new(id: u32, data: Vec<u8>) -> PyResult<Self> {
        // First build a JFrame from the id and data, using new _jframe.
        // This method runs some data validation, so we need to use it.
        let frame = new_jframe(id, data).map_err(|e| {
            PyOSError::new_err(format!("Error creating frame: {}", e))
        })?;

        // Then convert JFrame to PyJFrame, using the From<> trait
        Ok(frame.into())
    }

    // Implement string representation using the to_string method
    fn __str__(&self) -> PyResult<String> {
        Ok(self.frame.to_string())
    }

    // Implement the id property
    #[getter]
    fn id(&self) -> PyResult<u32> {
        Ok(self.frame.id)
    }

    // Implement the data property
    #[getter]
    fn data(&self) -> PyResult<Vec<u8>> {
        // Cannot return this since Vec<u8> doesn't implent Copy
        // Ok(self.frame.data)
        Ok(self.frame.data.clone())
    }
}

// Implement conversion from PyJFrame to JFrame
impl From<PyJFrame> for ffi::JFrame {
    fn from(py_frame: PyJFrame) -> Self {
        // Unbox the PyJFrame, and return the JFrame
        py_frame.frame
    }
}

// Implement conversion from JFrame to PyJFrame
impl From<ffi::JFrame> for PyJFrame {
    fn from(frame: ffi::JFrame) -> Self {
        PyJFrame {
            frame,
        }
    }
}

#[pymodule]
fn jcan_python(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyJBus>()?;
    m.add_class::<PyJFrame>()?;

    Ok(())
}
