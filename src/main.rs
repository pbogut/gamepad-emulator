use joystick::Error;

mod joystick;

fn main() -> Result<(), Error> {
    let joy = joystick::Joystick::new()?;
    std::thread::sleep(std::time::Duration::from_secs(5));
    println!("Press Y");
    joy.button_press(joystick::Button::RightSouth, true)?;
    joy.synchronise()?;

    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("Release Y");
    joy.button_press(joystick::Button::RightSouth, false)?;
    joy.synchronise()?;

    joy.button_press(joystick::Button::RightSouth, false)?;

    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("Bye");
    Ok(())
}
