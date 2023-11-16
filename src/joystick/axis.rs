use std::slice;

#[derive(Debug)]
pub enum Axis {
    X,
    Y,
    Z,
    RX,
    RY,
    RZ,
    Hat0X,
    Hat0Y,
}

impl Axis {
    pub(super) fn to_evdev_axis(&self) -> input_linux::AbsoluteAxis {
        use Axis::*;

        match &self {
            X => input_linux::AbsoluteAxis::X,
            Y => input_linux::AbsoluteAxis::Y,
            Z => input_linux::AbsoluteAxis::Z,
            RX => input_linux::AbsoluteAxis::RX,
            RY => input_linux::AbsoluteAxis::RY,
            RZ => input_linux::AbsoluteAxis::RZ,
            Hat0X => input_linux::AbsoluteAxis::Hat0X,
            Hat0Y => input_linux::AbsoluteAxis::Hat0Y,
        }
    }

    pub(super) fn all_axes() -> slice::Iter<'static, Self> {
        use Axis::*;
        [X, Y, Z, RX, RY, RZ, Hat0X, Hat0Y].iter()
    }
}
