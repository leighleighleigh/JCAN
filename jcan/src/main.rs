// Rust example of using JCAN.
// This helps to test the C++ bindings.

extern crate jcan;

use jcan::{*, ffi::JFrame};


// extern "C" fn on_frame(frame: &ffi::JFrame) {
//     println!("Received frame: {}", frame);
// }

// // extern "C" fn on_special_frame(frame: &ffi::JFrame) {
// //     println!("Received special frame: {}", frame);
// // }

fn main()
{
    println!("Hello, world!");
    let mut bus = new_jbus().unwrap();

    // Register a frame handler callback
    // let cb = FrameCallback(on_frame);

    // Register the callback
    // bus.on_receive(cb).expect("Failed to register callback.");
    // bus.on_receive_id(0x123, FrameCallback(on_special_frame)).expect("Failed to register callback.");

    // Open the bus in async mode
    bus.open("vcan0".to_string()).expect("Failed to open bus.");


    loop {
        // Print and wait a bit
        println!("Spinning...");
        let frames = bus.receive_many().expect("Failed to spin bus.");

        // Print received frames
        for frame in frames {
            println!("Received frame: {}", frame);
        }

        // Build frame
        let frame = JFrame::build(0x123, [0x10,0x20].to_vec()).unwrap();
        println!("{}", frame);
        // Send frame
        bus.send(frame).expect("Failed to send frame.");

        std::thread::sleep(std::time::Duration::from_millis(1000));
        // // receive frame (blocking)
        // let frame = bus.receive().expect("Failed to receive frame.");
        // println!("{}", frame);
    }
}