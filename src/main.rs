mod input;
mod joystick;
mod queue;

use std::cmp::{max, min};

use input::{Error, MiceEvent};

const MULTIPLICATION_FACTOR: i32 = 15;

fn main() -> Result<(), Error> {
    let joy = joystick::Joystick::new()?;
    eprintln!("Device created: {:?}", joy.device_path()?);

    // std::thread::sleep(std::time::Duration::from_secs(5));

    // joy.button_press(joystick::Button::RightSouth, true)?;
    // joy.synchronise()?;
    // std::thread::sleep(std::time::Duration::from_secs(1));
    // joy.button_press(joystick::Button::RightSouth, false)?;
    // joy.synchronise()?;
    // std::thread::sleep(std::time::Duration::from_secs(1));

    // joy.button_press(joystick::Button::RightEast, true)?;
    // joy.synchronise()?;
    // std::thread::sleep(std::time::Duration::from_secs(1));
    // joy.button_press(joystick::Button::RightEast, false)?;
    // joy.synchronise()?;
    // std::thread::sleep(std::time::Duration::from_secs(1));

    // joy.button_press(joystick::Button::RightWest, true)?;
    // joy.synchronise()?;
    // std::thread::sleep(std::time::Duration::from_secs(1));
    // joy.button_press(joystick::Button::RightWest, false)?;
    // joy.synchronise()?;
    // std::thread::sleep(std::time::Duration::from_secs(1));

    // joy.button_press(joystick::Button::RightNorth, true)?;
    // joy.synchronise()?;
    // std::thread::sleep(std::time::Duration::from_secs(1));
    // joy.button_press(joystick::Button::RightNorth, false)?;
    // joy.synchronise()?;
    // std::thread::sleep(std::time::Duration::from_secs(1));

    // joy.button_press(joystick::Button::RightSouth, false)?;
    // joy.synchronise()?;

    // joy.move_axis(joystick::Axis::X, 500)?;
    // joy.synchronise()?;
    // std::thread::sleep(std::time::Duration::from_secs(1));
    // joy.move_axis(joystick::Axis::X, 250)?;
    // joy.synchronise()?;
    // std::thread::sleep(std::time::Duration::from_secs(1));

    // let rx = miceevent::listen()?;
    // let (tx, rx) = std::sync::mpsc::channel();
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
