use std::slice;

#[derive(Debug)]
pub enum Button {
    LeftSpecial,

    RightNorth,
    RightSouth,
    RightEast,
    RightWest,

    RightSpecial,

    L1,
    R1,
    L2,
    R2,
}

impl Button {
    pub(super) fn to_evdev_button(&self) -> input_linux::Key {
        use input_linux::Key::*;
        use Button::*;

        match &self {
            LeftSpecial => ButtonStart,

            RightNorth => ButtonNorth,
            RightSouth => ButtonSouth,
            RightEast => ButtonEast,
            RightWest => ButtonWest,

            RightSpecial => ButtonSelect,

            L1 => ButtonTL,
            R1 => ButtonTR,
            L2 => ButtonTL2,
            R2 => ButtonTR2,
        }
    }

    pub(super) fn all_buttons() -> slice::Iter<'static, Self> {
        use Button::*;
        [
            RightSouth,
            RightEast,
            RightWest,
            RightNorth,
            LeftSpecial,
            RightSpecial,
            L1,
            R1,
            L2,
            R2,
        ]
        .iter()
    }
}
