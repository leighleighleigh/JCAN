// Separate library for the Python wrappers, using 
// pyo3 and the maturin build system.
// This is built separately to the main library, so that
// the libjcan.a file isn't bloated with python junk.
// It also helps the cxx system not get confused.
extern crate pyo3;
extern crate jcan;

use std::collections::HashMap;

use pyo3::exceptions::{PyOSError, PyRuntimeError, PyValueError, PyTypeError};
use pyo3::prelude::*;
use pyo3::types::{PyModule, PyList};
use pyo3::{PyResult};

use jcan::*;

#[pyclass]
#[pyo3{name = "Bus"}]
struct PyJBus {
    bus: JBus,
    // Make a HashMap of callbacks, where the u32 key 
    // ,is the ID of the frame being handled by the callback 
    callbacks: HashMap<u32, PyObject>,
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
    fn new() -> PyResult<Self> {
        // Unbox the result of new_jbus, and return it
        let bus = new_jbus().map_err(|e| {
            PyOSError::new_err(format!("Error creating Bus: {}", e))
        })?;

        // bus is a Box<JBus>, so we need to dereference it
        Ok(PyJBus {
            bus: *bus,
            callbacks: HashMap::new(),
        })
    }

    // Implement the open method for the PyJBus
    fn open(&mut self, interface: String) -> PyResult<()> {
        self.bus.open(interface).map_err(|e| {
            PyOSError::new_err(format!("Error opening bus: {}", e))
        })?;
        Ok(())
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

    // Implement the send method for the PyJBus
    fn send(&mut self, frame: PyJFrame) -> PyResult<()> {
        self.bus.send(frame.frame).map_err(|e| {
            PyOSError::new_err(format!("Error sending frame: {}", e))
        })?;
        Ok(())
    }

    // Implement set_id_filter for the PyJBus, which takes a list of IDs
    fn set_id_filter(&mut self, allowed_ids: Vec<u32>) -> PyResult<()> {
        self.bus.set_id_filter(allowed_ids).map_err(|e| {
            PyOSError::new_err(format!("Error setting filter: {}", e))
        })?;
        Ok(())
    }

    fn set_id_filter_mask(&mut self, allowed: u32, allowed_mask: u32) -> PyResult<()> {
        self.bus.set_id_filter_mask(allowed, allowed_mask).map_err(|e| {
            PyOSError::new_err(format!("Error setting filter mask: {}", e))
        })?;
        Ok(())
    }

    // Receive many will return a list of buffered frames from the receive thread
    fn receive_from_thread_buffer(&mut self) -> PyResult<Vec<PyJFrame>> {
        let frames = self.bus.receive_from_thread_buffer().map_err(|e| {
            PyOSError::new_err(format!("Error receiving frames: {}", e))
        })?;

        Ok(frames.into_iter().map(|f| PyJFrame {
            frame: f,
        }).collect())
    }

    // Implement the add_callback method for the PyJBus, which takes a function
    // and adds it to the list of callbacks
    fn add_callback(&mut self, frame_id: u32, callback: PyObject) -> PyResult<()> {
        // Check that the callback takes a single argument
        let _gil = Python::with_gil(|py| {
            let args = callback.getattr(py, "__code__").map_err(|e| {
                PyRuntimeError::new_err(format!("Error calling __code__ on callback: {}", e))
            }).expect("Failed to get __code__").getattr(py, "co_argcount").map_err(|e| {
                PyRuntimeError::new_err(format!("Error getting co_argcount on callback: {}", e))
            }).expect("Failed to read co_argcount").extract::<u32>(py).map_err(|e| {
                PyRuntimeError::new_err(format!("Error extracting co_argcount from callback: {}", e))
            })?;

            if args < 1 {
                return Err(PyValueError::new_err("Callback provided must take atleast one positional argument"));
            }
            Ok(())
        })?;

        // Store the callback in the HashMap
        self.callbacks.insert(frame_id, callback);

        Ok(())
    }

    // Implement the spin() method, which first calls the underlying receive_from_thread_buffer to retrieve all the frames
    // ,before calling the appropriate callback functions
    fn spin(&mut self) -> PyResult<()> {
        let frames = self.receive_from_thread_buffer()?;
        let _gil = Python::with_gil(|py| {
            for frame in frames {
                // Lookup the callback function for the frame, given its ID
                // If no callback is found, ignore the frame
                let callback = match self.callbacks.get(&frame.frame.id) {
                    Some(c) => c,
                    None => continue,
                };

                // Call the callback function with the frame as an argument
                // If an error occurs, return it
                callback.call1(py, (frame.clone(),)).map_err(|e| {
                    PyRuntimeError::new_err(format!("Error calling callback: {}", e))
                }).expect("Error calling callback");
            }
        });
        Ok(())
    }
}

// Implement the 'new' method for the PyJFrame, which makes a cal to new_jframe
#[pymethods]
impl PyJFrame {
    #[new]
    fn new(id: u32, data: &PyList) -> PyResult<Self> {
        // We use a PyList here, so that the user can pass in a list of integers OR floats.
        // We will cast them to uint8's here, or raise an error if the type was wildly wrong (such as str)
        let data: Vec<u8> = data.iter().map(|x| {
            // If x is PyLong or PyFloat, we can convert it to a U8 (in some kind of way)
            // First try extract to u8 ,then to f32->u8, else raise error
            if let Ok(x) = x.extract::<u8>() {
                x
            } else if let Ok(x) = x.extract::<f32>() {
                x as u8
            } else {
                // If x is not PyLong or PyFloat, we cannot convert it to a u8
                // Raise a TypeError
                panic!("{}",PyTypeError::new_err("Data must be a list of integers or floats"));
            }
        }).collect();

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

impl From<PyJFrame> for ffi::JFrame {
    fn from(py_frame: PyJFrame) -> Self {
        // Unbox the PyJFrame, and return the JFrame
        py_frame.frame
    }
}

impl From<ffi::JFrame> for PyJFrame {
    fn from(frame: ffi::JFrame) -> Self {
        PyJFrame {
            frame,
        }
    }
}

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
