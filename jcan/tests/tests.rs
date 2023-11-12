#![allow(non_snake_case)]

// Import jcan
extern crate jcan;
use jcan::*;

// Import testing harness
mod harness;
use harness::{*};

#[test]
fn test_vcan_harness() {
    let iface = add_vcan();
    del_vcan(iface);
}

#[test]
fn test_bus_open_close() {
    let iface = add_vcan();

    let mut bus = new_jbus().unwrap();
    assert!(bus.open(iface.clone(), 2, 256).is_ok());

    assert!(bus.close().is_ok());
    del_vcan(iface);
}

#[test]
fn test_send() {
    let iface = add_vcan();

    let mut bus = new_jbus().unwrap();
    assert!(bus.open(iface.clone(), 2, 256).is_ok());

    let frame = new_jframe(0x123,(&[0xA,0xB,0xC,0xD]).to_vec()).expect("Failed to build Frame");
    let _ = bus.send(frame);

    assert!(bus.close().is_ok());
    del_vcan(iface);
}

#[test]
fn test_send_receive() {
    let iface = add_vcan();

    let mut bus = new_jbus().unwrap();
    assert!(bus.open(iface.clone(), 2, 256).is_ok());

    // Build a frame to send using 'cansend' tool
    let frame = new_jframe(0x123,(&[0xA,0xA]).to_vec()).expect("Failed to build Frame");
    
    // Send a frame
    cansend_vcan(iface.clone(), &frame);
    
    // Receive a frame, comparing it to the expected one
    let rx = bus.receive().expect("Failed to receive Frame");
    println!("{}",rx);
    assert!(rx == frame);

    assert!(bus.close().is_ok());
    del_vcan(iface);
}

#[test]
fn test_id_filter() {
    let iface = add_vcan();

    let mut bus = new_jbus().unwrap();

    // Apply a filter to only receive frames with
    // ID == 0x210 ONLY!
    bus.set_id_filter((&[ 0x210 ]).to_vec()).expect("Failed to set Frame ID filter");
    bus.open(iface.clone(), 2, 256).expect("Failed to open Bus");

    // Build two frames to send - the first one should be ignored!
    let frame1 = new_jframe(0x123,(&[0xAA,0xBB]).to_vec()).expect("Failed to build Frame");
    let frame2 = new_jframe(0x210,(&[0xCC,0xDD]).to_vec()).expect("Failed to build Frame");
    
    // Send the two frames
    cansend_vcan(iface.clone(), &frame1);
    cansend_vcan(iface.clone(), &frame2);
    
    // Receive a frame, comparing it to the expected one
    let rx = bus.receive().expect("Failed to receive Frame");
    println!("{}",rx);

    assert!(rx == frame2);

    assert!(bus.close().is_ok());
    del_vcan(iface);
}

#[test]
fn test_receive_timeout() {
    let iface = add_vcan();

    let mut bus = new_jbus().unwrap();
    assert!(bus.open(iface.clone(), 2, 256).is_ok());

    // Receive a frame, comparing it to the expected one
    let rx = bus.receive_with_timeout_millis(300);

    assert!(rx.is_err());

    assert!(bus.close().is_ok());
    del_vcan(iface);
}


#[test]
fn test_receive_buffer() {
    // Send 100 frames using cansend, and then receive them.
    // PASS if we received all 100 frames.
    let N = 30u16;
    let iface = add_vcan();

    let mut bus = new_jbus().unwrap();
    bus.open(iface.clone(), 2, 256).expect("Failed to open Bus");

    let frame1 = new_jframe(0x123,(&[0xAA,0xBB]).to_vec()).expect("Failed to build Frame");
    
    for _ in 0..N {
      cansend_vcan(iface.clone(), &frame1);
    }
    
    // Receives from the Vec buffer directly
    let rxs = bus.receive_from_thread_buffer().expect("Failed to receive from thread buffer");
    println!("Received {} frames", rxs.len());

    assert!(rxs.len() == N as usize);
    assert!(bus.close().is_ok());

    del_vcan(iface);
}

#[test]
fn test_receive_buffer_alternate() {
    // Send 100 frames using cansend, and then receive them.
    // PASS if we received all 100 frames.
    // This test uses bus.receive() in a loop, instead of receive_from_thread_buffer().

    let N = 30u16;
    let iface = add_vcan();

    let mut bus = new_jbus().unwrap();
    bus.open(iface.clone(), 2, 256).expect("Failed to open Bus");

    let frame1 = new_jframe(0x123,(&[0xAA,0xBB]).to_vec()).expect("Failed to build Frame");
    
    for _ in 0..N {
      cansend_vcan(iface.clone(), &frame1);
    }
    println!("Sent {} frames", N);
    
    // Receives from the Vec buffer directly
    let mut rxs : Vec<ffi::JFrame> = Vec::new();

    for _ in 0..N {
      let rx = bus.receive_with_timeout_millis(100);
      match rx {
        Ok(r) => rxs.push(r),
        Err(_) => break,
      }
    }
    println!("Received {} frames", rxs.len());

    assert!(rxs.len() == N as usize);
    assert!(bus.close().is_ok());

    del_vcan(iface);
}

#[test]
fn test_receive_buffer_overflow() {
    // Send 500 frames using cansend, dropping 250 of them, because our receive buffer is
    // only 250 frames long.

    let N = 100u16;
    let B = 50u16;

    let iface = add_vcan();

    let mut bus = new_jbus().unwrap();
    bus.open(iface.clone(), 2, B).expect("Failed to open Bus");
  
    let frame1 = new_jframe(0x123,(&[0xAA,0xBB]).to_vec()).expect("Failed to build Frame");
    
    for _ in 0..N {
      cansend_vcan(iface.clone(), &frame1);
    }
    println!("Sent {} frames", N);
    
    // Receives from the Vec buffer directly
    let mut rxs : Vec<ffi::JFrame> = Vec::new();

    for _ in 0..N {
      let rx = bus.receive_with_timeout_millis(100);
      match rx {
        Ok(r) => rxs.push(r),
        Err(_) => break,
      }
    }
    println!("Received {} frames", rxs.len());

    assert!(rxs.len() == B as usize);
    assert!(bus.close().is_ok());

    del_vcan(iface);
}
