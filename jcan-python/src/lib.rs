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
    // callbacks: Vec<(u32, PyFrameCallback)>,
}

#[pyclass]
#[pyo3{name = "Frame"}]
#[derive(Clone)]
struct PyJFrame {
    frame: ffi::JFrame,
}


// struct PyFrameCallbackManager {
//     callbacks: Vec<(u32, cb(f: ffi::JFrame))>,
// }

// Implement the 'new' method for the PyJBus, which makes a call to new_jbus
#[pymethods]
impl PyJBus {
    #[new]
    fn new(interface: String) -> PyResult<Self> {
        // Unbox the result of new_jbus, and return it
        let bus = new_jbus().map_err(|e| {
            PyOSError::new_err(format!("Error opening bus: {}", e))
        })?;

        // bus is a Box<JBus>, so we need to dereference it
        Ok(PyJBus {
            bus: *bus,
            // callbacks: Vec::new(),
        })
    }

    // Implement the open method for the PyJBus
    fn open(&mut self, interface: String) -> PyResult<()> {
        self.bus.open(interface).map_err(|e| {
            PyOSError::new_err(format!("Error opening bus: {}", e))
        })?;
        Ok(())
    }

    // // C-exported version of the pycallable_to_framecallback method
    // // This is not turned into a pymethod
    // #[inline]
    // extern "C" fn _on_recv_handler_(&self, frame: &ffi::JFrame) {
    //     // For each callback matching this frame's ID, or 0 (all IDs), call the callback
    //     for (id, callback) in self.callbacks.iter() {
    //         if *id == 0 || *id == frame.id {
    //             // Convert the frame to a PyJFrame, and call the callback
    //             let pyframe = PyJFrame {
    //                 frame: *frame,
    //             };
    //             let gil = Python::with_gil(|py| {
    //                 callback.callback.call1(py, (pyframe, )).unwrap();
    //             });
    //         }
    //     }
    // }

    // // Implement the on_receive method.
    // fn on_receive(&mut self, callback: Py<PyAny>) -> PyResult<()> {
    //     self.on_receive_id(0, callback)
    // }

    // // Implement the on_receive_id method. 
    // // The function signature is replaced so that it only accepts python Callable's
    // fn on_receive_id(&mut self, id: u32, callback: Py<PyAny>) -> PyResult<()> {
    //     // Store the callback in a vector, so that it doesn't get dropped
    //     self.callbacks.push((id, PyFrameCallback {
    //         callback,
    //     }));

    //     // If there's already a callback with this ID, don't make the bus call
    //     let mut found = false;
    //     for (cid, _) in self.callbacks.iter() {
    //         if *cid == id {
    //             found = true;
    //             break;
    //         }
    //     }

    //     if !found {
    //         // Call the on_receive method on the JBus
    //         self.bus.on_receive_id(id, FrameCallback(self._on_recv_handler_)).map_err(|e| {
    //             PyOSError::new_err(format!("Error setting on_receive callback: {}", e))
    //         })?;
    //     }
    //     Ok(())
    // }



    // Implement the receive method for the PyJBus
    fn receive(&mut self) -> PyResult<PyJFrame> {
        let frame = self.bus.receive().map_err(|e| {
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

    // Implement set_id_filter for the PyJBus, which takes a list of IDs
    fn set_id_filter(&mut self, ids: Vec<u32>) -> PyResult<()> {
        self.bus.set_id_filter(ids).map_err(|e| {
            PyOSError::new_err(format!("Error setting ID filter: {}", e))
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

    // Implement the get_id/set_id methods, as a property getter/setter
    #[getter]
    fn get_id(&self) -> PyResult<u32> {
        Ok(self.frame.get_id())
    }

    #[setter]
    fn set_id(&mut self, id: u32) -> PyResult<()> {
        self.frame.set_id(id).map_err(|e| {
            PyOSError::new_err(format!("Error setting ID: {}", e))
        })?;

        Ok(())
    }

    // Implement the get_data/set_data methods, as a property getter/setter
    #[getter]
    fn get_data(&self) -> PyResult<Vec<u8>> {
        Ok(self.frame.get_data())
    }

    #[setter]
    fn set_data(&mut self, data: Vec<u8>) -> PyResult<()> {
        self.frame.set_data(data).map_err(|e| {
            PyOSError::new_err(format!("Error setting data: {}", e))
        })?;

        Ok(())
    }
    


}

// Implement From trait for ffi::JFrame -> PyJFrame
impl From<ffi::JFrame> for PyJFrame {
    fn from(frame: ffi::JFrame) -> Self {
        PyJFrame {
            frame,
        }
    }
}

// Implement From train for PyJFrame -> PyOject
impl From<PyJFrame> for PyObject {
    fn from(frame: PyJFrame) -> Self {
        let gil = Python::with_gil(|py|{
            let pyframe = PyJFrame {
                frame: frame.frame,
            };
            pyframe.into_py(py)
        });
        gil
    }
}

#[pymodule]
fn jcan_python(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyJBus>()?;
    m.add_class::<PyJFrame>()?;

    Ok(())
}
