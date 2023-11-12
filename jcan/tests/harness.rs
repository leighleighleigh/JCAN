// For constructing frames
extern crate jcan;
use jcan::{ffi::JFrame};

// For running tests
use std::process::Command;
use serde::{Deserialize};
use serde_json;
use std::sync::Mutex;
use lazy_static::lazy_static;

#[derive(Debug, Deserialize)]
pub struct Link {
    ifname: String,
}

lazy_static! {
  static ref TEST_MUTEX: Mutex<()> = Mutex::new(());
}

#[allow(dead_code)]
pub fn add_vcan() -> String {
  // Grab the TEST_MUTEX to prevent threads getting the interfaces muddled
  let _guard = TEST_MUTEX.lock().unwrap();

  // Add a new vcan interface (vcan<N>)
  assert!(Command::new("sudo").args(&["ip","link","add","type","vcan"]).status().unwrap().success());

  // Get a list of virtual CAN interfaces, as JSON format.
  let ifs = Command::new("sudo").args(&["ip","-j","link","show","type","vcan"]).output().expect("Failed to list vcan links");

  let if_json = match String::from_utf8(ifs.stdout) {
      Ok(v) => v,
      Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
  };
  
  // Decode JSON into a Vector of Link objects
  let ifs_obj : Vec<Link> = serde_json::from_str(&if_json).expect("Failed to parse JSON");
  // Get the name of the last link available
  let iface = &ifs_obj.last().unwrap().ifname;

  println!("Created {}",iface);
  assert!(Command::new("sudo").args(&["ip","link","set","up",&iface.to_string()]).status().unwrap().success());
  
  iface.to_string()
}

#[allow(dead_code)]
pub fn del_vcan(iface : String) {
  assert!(Command::new("sudo").args(&["ip","link","del",&iface]).status().unwrap().success());
  println!("Deleted {}",iface);
}

#[allow(dead_code)]
pub fn cansend_vcan(iface : String, frame : &JFrame) {
  // Uses the string representation of JFrame, e.g 123#AABBCCDD
  assert!(Command::new("sudo").args(&["cansend", &iface, &frame.to_string()]).status().unwrap().success());
}

