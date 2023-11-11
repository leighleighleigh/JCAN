// Rust example of using JCAN.
// This helps to test the C++ bindings.

extern crate jcan;

use jcan::{*, ffi::JFrame};

fn main()
{
    println!("Hello, world!");
    let mut bus = new_jbus().unwrap();
    bus.open("vcan0".to_string(),2,256).expect("Failed to open bus.");

    let mut run = true;

    while run {
        println!("Spinning...");
        // NOTE: receive_from_thread_buffer() is similar to .spin()
        let frames = bus.receive_from_thread_buffer().expect("Failed to spin bus.");

        for frame in frames {
          println!("Received frame: {}", frame);

          let data = frame.get_data();
          
          #[allow(clippy::single_match)] // shush, Clippy
          match (frame.get_id(),data.len()) {
            (0x123,1) => {
              match data[0] {
                0x42 => {
                  println!("Received quit command!");
                  run = false;     
                },
                0x11 => {
                  println!("Received restart command!");
                  println!("bus.close()...");
                  let _ = bus.close();

                  println!("sleep 3");
                  std::thread::sleep(std::time::Duration::from_millis(3000));

                  println!("bus.open()");
                  bus.open("vcan0".to_string(),256,256).expect("Failed to open bus.");
                },
                _ => {}
              }
            },
            _ => {}
          }
        }

        // Build a frame
        let frame = JFrame::build(0x123, [0x10,0x20].to_vec()).unwrap();
        println!("{}", frame);

        // Send a frame
        bus.send(frame).expect("Failed to send frame.");

        // relax!
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

    let _ = bus.close();
}
