use self::Button::*;

pub enum Button {
    Y,
    X,
    B,
    A,
    RightSr,
    RightSl,
    R,
    Zr,

    Minus,
    Plus,
    Cr,
    Cl,
    Home,
    Capture,

    Down,
    Up,
    Right,
    Left,
    LeftSr,
    LeftSl,
    L,
    Zl,
}

impl Button {
    pub fn byte_offset(&self) -> usize {
        match self {
            Y | X | B | A | RightSr | RightSl | R | Zr => 0,
            Minus | Plus | Cr | Cl | Home | Capture => 1,
            Down | Up | Right | Left | LeftSr | LeftSl | L | Zl => 2,
        }
    }

    pub fn bit_mask(&self) -> u8 {
        match self {
            Y | Minus | Down => 0x01,
            X | Plus | Up => 0x02,
            B | Cr | Right => 0x04,
            A | Cl | Left => 0x08,
            RightSr | Home | LeftSr => 0x10,
            RightSl | Capture | LeftSl => 0x20,
            R | L => 0x40,
            Zr | Zl => 0x80,
        }
    }
}
