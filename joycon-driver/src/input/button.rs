use crate::device::id::Product;

use self::Button::*;

#[derive(Copy, Clone)]
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

    Sl,
    Sr,

    North,
    East,
    West,
    South,
}

impl Button {
    pub fn is_real(&self) -> bool {
        match self {
            Sl | Sr | North | East | West | South => false,
            _ => true,
        }
    }

    pub fn to_real(&self, product: Product) -> Option<Button> {
        match product {
            Product::JoyConL => match self {
                Sl => Some(LeftSl),
                Sr => Some(LeftSr),
                North => Some(Right),
                East => Some(Down),
                West => Some(Up),
                South => Some(Left),
                _ => Some(*self),
            },
            Product::JoyConR => match self {
                Sl => Some(RightSl),
                Sr => Some(RightSr),
                North => Some(Y),
                East => Some(X),
                West => Some(B),
                South => Some(A),
                _ => Some(*self),
            },
            Product::ProController => match self {
                Sl => None,
                Sr => None,
                North => Some(X),
                East => Some(A),
                West => Some(Y),
                South => Some(B),
                _ => Some(*self),
            },
            _ => None,
        }
    }
}

impl From<Button> for u32 {
    fn from(button: Button) -> u32 {
        match button {
            Y => 1,
            X => 1 << 1,
            B => 1 << 2,
            A => 1 << 3,
            RightSr => 1 << 4,
            RightSl => 1 << 5,
            R => 1 << 6,
            Zr => 1 << 7,

            Minus => 1 << 8,
            Plus => 1 << 9,
            Cr => 1 << 10,
            Cl => 1 << 11,
            Home => 1 << 12,
            Capture => 1 << 13,
            // Bit 14 is unused
            // Bit 15 is called "Charging grip"?
            Down => 1 << 16,
            Up => 1 << 17,
            Right => 1 << 18,
            Left => 1 << 19,
            LeftSr => 1 << 20,
            LeftSl => 1 << 21,
            L => 1 << 22,
            Zl => 1 << 23,

            _ => 0, // Not a real button
        }
    }
}
