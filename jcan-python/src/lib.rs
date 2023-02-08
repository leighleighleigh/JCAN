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


// fn add_to_vec<'a, T: FnMut() + 'a>(v: &mut Vec<Box<FnMut() + 'a>>, f: T) {
//     v.push(Box::new(f));
// }

// fn call_b() {
//     println!("Call b.");
// }

// #[test]
// fn it_works() {
//     let mut calls: Vec<Box<FnMut()>> = Vec::new();

//     add_to_vec(&mut calls, || { println!("Call a."); });
//     add_to_vec(&mut calls, call_b);

//     for mut c in calls.drain() {
//         c();
//     }
// }

#[pyclass]
#[pyo3{name = "Bus"}]
struct PyJBus {
    bus: JBus,
    callbacks : Vec<PyObject>,
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
        let bus = new_jbus().map_err(|e| {
            PyOSError::new_err(format!("Error opening bus: {}", e))
        })?;

        // bus is a Box<JBus>, so we need to dereference it
        Ok(PyJBus {
            bus: *bus,
            callbacks: Vec::new(),
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
    fn set_id_filter(&mut self, ids: Vec<u32>) -> PyResult<()> {
        self.bus.set_id_filter(ids).map_err(|e| {
            PyOSError::new_err(format!("Error setting ID filter: {}", e))
        })?;
        Ok(())
    }

    // Implement the spin() method, which first calls the underlying spin_cycle to retrieve all the frames
    // , before calling the callback functions
    fn spin(&mut self) -> PyResult<()> {
        let frames = self.bus.spin_cycle().map_err(|e| {
            PyOSError::new_err(format!("Error spinning: {}", e))
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
