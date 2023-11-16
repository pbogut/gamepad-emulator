mod input;
mod joystick;
mod queue;

use std::cmp::{max, min};

use input::{Error, MiceEvent};

const MULTIPLICATION_FACTOR: i32 = 15;

fn main() -> Result<(), Error> {
    let joy = joystick::Joystick::new()?;
    eprintln!("Device created: {:?}", joy.device_path()?);

    let rx_mice = MiceEvent::listen()?;

    let mut queue_x = queue::Queue::new();
    let mut queue_y = queue::Queue::new();

    loop {
        match rx_mice.try_recv() {
            Ok(m) => {
                queue_x.push(m.x);
                queue_y.push(m.y);
                handle_mouse_motion(&joy, queue_x.sum(), queue_y.sum())?;
            }
            Err(_) => {
                queue_x.push(0);
                queue_y.push(0);
                handle_mouse_motion(&joy, queue_x.sum(), queue_y.sum())?;
            }
        }
        joy.synchronise()?;
    }
}

fn handle_mouse_motion(joy: &joystick::Joystick, x: i32, y: i32) -> Result<(), Error> {
    let x = max(-512, min(512, x / MULTIPLICATION_FACTOR));
    let y = max(-512, min(512, y / MULTIPLICATION_FACTOR));
    joy.move_axis(joystick::Axis::RX, x)?;
    joy.move_axis(joystick::Axis::RY, -y)?;
    Ok(())
}
