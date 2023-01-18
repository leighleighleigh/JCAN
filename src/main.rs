//
// read_blocking.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 05 2022
//

// Allow unused imports 
#[allow(unused_imports)]

use anyhow::Context;
use clap::Parser;

use embedded_can::{blocking::Can, Frame as EmbeddedFrame, Id, StandardId};
use socketcan::{CanFrame, CanSocket, Frame, Socket, CanSocketOpenError};

// open/frame_to_string/get_raw_id are imported from this crate, defined in the lib.rs file
use jorzacan::*;



#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// CAN interface
    #[clap(value_parser)]
    interface: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let can_interface = args.interface;
    
    let mut socket = open(&can_interface).with_context(|| format!("Failed to open socket on interface {}", can_interface))?;
    
    loop
    {

        let frame = socket.receive();

        if let Ok(frame) = frame {
            println!("{}", frame_to_string(&frame));
        }

        let write_frame = CanFrame::new(StandardId::new(0x1f1).unwrap(), &[1, 2, 3, 4])
            .expect("Failed to create CAN frame");

        socket
            .transmit(&write_frame)
            .expect("Failed to transmit frame");
        
    }
    //Ok(())
}

