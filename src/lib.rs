#[allow(unused_imports)]
extern crate socketcan;
extern crate pyo3;

#[allow(unused_imports)]

use std::ffi::*;
use std::slice;

use embedded_can::{blocking::Can, Frame as EmbeddedFrame, Id, StandardId, ExtendedId};
use pyo3::exceptions::{PyOSError};
use pyo3::prelude::*;
use pyo3::types::{PyModule};
use pyo3::{PyResult};
use socketcan::{CanFrame, CanSocket, Frame, Socket, CanSocketOpenError, CanError};


/* RUST NATIVE FUNCTIONS */
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

pub fn open(iface: &str) -> Result<CanSocket, CanSocketOpenError> {
    CanSocket::open(iface)
}

pub fn send(socket: &mut CanSocket, id: u32, data: &[u8]) -> Result<(), CanSocketOpenError> {
    // If id is 11 bits, use StandardId, otherwise use ExtendedId
    let frame_id : Id = match id.try_into() {
        Ok(id) => embedded_can::Id::Standard(StandardId::new(id).unwrap()),
        Err(_) => embedded_can::Id::Extended(ExtendedId::new(id).unwrap()),
    };

    let write_frame = CanFrame::new(frame_id, data)
        .expect("Failed to create CAN frame");

    socket
        .transmit(&write_frame)
        .expect("Failed to transmit frame");

    Ok(())
}

pub fn receive(socket: &mut CanSocket) -> Result<CanFrame, CanError> {
    let frame = socket.receive()?;
    Ok(frame)
}

// Create custom Python exception to handle CanSocket errors
pyo3::create_exception!(jorzacan, CanSocketError, PyOSError);


#[pyclass]
#[pyo3{name = "Bus"}]
pub struct JorzaBus {
    socket: CanSocket,
}

#[pyclass]
#[pyo3{name = "Frame"}]
pub struct JorzaFrame {
    frame: CanFrame,
}

#[pymethods]
impl JorzaFrame {
    #[new]
    fn new(id: u32, data: &[u8]) -> PyResult<Self> {
        // If id is 11 bits, use StandardId, otherwise use ExtendedId
        let frame_id : Id = match id.try_into() {
            Ok(id) => embedded_can::Id::Standard(StandardId::new(id).unwrap()),
            Err(_) => embedded_can::Id::Extended(ExtendedId::new(id).unwrap()),
        };

        let frame = CanFrame::new(frame_id, data)
            .expect("Failed to create CAN frame");

        Ok(Self { frame })
    }

    fn id(&self) -> u32 {
        get_raw_id(&self.frame.id())
    }

    fn data(&self) -> Vec<u8> {
        self.frame.data().to_vec()
    }

    fn dlc(&self) -> u8 {
        self.frame.dlc().try_into().unwrap()
    }

    fn __str__(&self) -> String {
        frame_to_string(&self.frame)
    }
}


#[pymethods]
impl JorzaBus {

    #[new]
    fn new(iface: &str) -> PyResult<Self> {
        // Open CAN socket, return error as CanSocketError to variable
        //let socket = CanSocket::open(iface).map_err(|e| CanSocketError::new_err(e.to_string()))?;
        // same as above, but uses the public open() function
        let socket = open(iface).map_err(|e| CanSocketError::new_err(e.to_string()))?;

        // Returns new instance of Bus, with socket variable set to the opened socket
        Ok(Self { socket })
    }

    fn send_raw(&mut self, id: u32, data: &[u8]) -> PyResult<()> {
        // Send CAN frame, return error as CanSocketError to variable
        let _ = send(&mut self.socket, id, data).map_err(|e| CanSocketError::new_err(e.to_string()))?;

        Ok(())
    }

    fn send(&mut self, frame: &JorzaFrame) -> PyResult<()> {
        let _ = send(&mut self.socket, frame.id(), &frame.data()).map_err(|e| CanSocketError::new_err(e.to_string()))?;
        Ok(())
    }

    fn receive(&mut self) -> PyResult<JorzaFrame> {
        let frame = receive(&mut self.socket).map_err(|e| CanSocketError::new_err(e.to_string()))?;
        Ok(JorzaFrame { frame: frame })
    }
}


#[pymodule]
fn jorzacan(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("CanSocketError", _py.get_type::<CanSocketError>())?;
    m.add_class::<JorzaFrame>()?;
    m.add_class::<JorzaBus>()?;

    Ok(())
}


/* Below are C++ functions which define the FFI for the JorzaBus/JorzaFrame */
/*
This is similar to the below
#[no_mangle]
pub unsafe extern "C" fn unic_langid_new() -> *mut LanguageIdentifier {
    let langid = LanguageIdentifier::default();
    Box::into_raw(Box::new(langid))
}

#[no_mangle]
pub unsafe extern "C" fn unic_langid_destroy(langid: *mut LanguageIdentifier) {
    drop(Box::from_raw(langid));
}

#[no_mangle]
pub unsafe extern "C" fn unic_langid_as_string(
    langid: &mut LanguageIdentifier,
    ret_val: &mut nsACString,
) {
    ret_val.assign(&langid.to_string());
}

*/

// new jorza_bus handler
#[no_mangle]
pub unsafe extern "C" fn jorzacan_bus_new(iface: *const c_char) -> *mut JorzaBus {
    let iface = CStr::from_ptr(iface).to_str().unwrap();
    let bus = JorzaBus::new(iface).unwrap();
    Box::into_raw(Box::new(bus))
}

// destroy jorza_bus handler
#[no_mangle]
pub unsafe extern "C" fn jorzacan_bus_destroy(bus: *mut JorzaBus) {
    drop(Box::from_raw(bus));
}

// new jorza_frame handler
#[no_mangle]
pub unsafe extern "C" fn jorzacan_frame_new(id: u32, data: *const u8, len: usize) -> *mut JorzaFrame {
    let data = slice::from_raw_parts(data, len);
    let frame = JorzaFrame::new(id, data).unwrap();
    Box::into_raw(Box::new(frame))
}

// destroy jorza_frame handler
#[no_mangle]
pub unsafe extern "C" fn jorzacan_frame_destroy(frame: *mut JorzaFrame) {
    drop(Box::from_raw(frame));
}

// send_raw handler
#[no_mangle]
pub unsafe extern "C" fn jorzacan_bus_send_raw(bus: &mut JorzaBus, id: u32, data: *const u8, len: usize) {
    let data = slice::from_raw_parts(data, len);
    bus.send_raw(id, data).unwrap();
}

// send handler
#[no_mangle]
pub unsafe extern "C" fn jorzacan_bus_send(bus: &mut JorzaBus, frame: &JorzaFrame) {
    bus.send(frame).unwrap();
}

// receive handler
#[no_mangle]
pub unsafe extern "C" fn jorzacan_bus_receive(bus: &mut JorzaBus) -> *mut JorzaFrame {
    let frame = bus.receive().unwrap();
    Box::into_raw(Box::new(frame))
}

// JorzaFrame id, data, dlc, __str__ handlers
#[no_mangle]
pub unsafe extern "C" fn jorzacan_frame_id(frame: &JorzaFrame) -> u32 {
    frame.id()
}

// frame data is u8, length given by the dlc
#[no_mangle]
pub unsafe extern "C" fn jorzacan_frame_data(frame: &JorzaFrame, data: *mut u8) {
    let data = slice::from_raw_parts_mut(data, frame.dlc() as usize);
    data.copy_from_slice(&frame.data());
}

#[no_mangle]
pub unsafe extern "C" fn jorzacan_frame_dlc(frame: &JorzaFrame) -> u8 {
    frame.dlc()
}

// JorzaFrame str handler allocates memory for the string, and returns a pointer to it
#[no_mangle]
pub unsafe extern "C" fn jorzacan_frame_str(frame: &JorzaFrame) -> *mut c_char {
    let frame_str = frame.__str__() + "\0";
    let c_str = CString::new(frame_str).unwrap();
    c_str.into_raw()
}
