mod error;
mod mouse;

pub use error::Error;

use std::{io::Read, sync::mpsc::Receiver};

pub struct MiceEvent {
    pub x: i32,
    pub y: i32,
    pub left: bool,
    pub right: bool,
    pub middle: bool,
}

pub struct KbdEvent {
    pub key: u8,
}

impl MiceEvent {
    pub fn listen() -> Result<Receiver<MiceEvent>, error::Error> {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut mice = std::fs::File::open("/dev/input/mice")?;
        let mut mice_buf = [0; 4];

        std::thread::spawn(move || loop {
            mice.read(&mut mice_buf)
                .expect("Mice device is not readable");

            let left = mice_buf[0] & 0x1 != 0;
            let right = mice_buf[0] & 0x2 != 0;
            let middle = mice_buf[0] & 0x4 != 0;

            let x = mice_buf[1] as i8;
            let y = mice_buf[2] as i8;

            let m = MiceEvent {
                x: x as i32,
                y: y as i32,
                left,
                right,
                middle,
            };
            tx.send(m).unwrap();
        });

        Ok(rx)
    }
}
