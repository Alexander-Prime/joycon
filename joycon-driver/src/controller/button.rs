use has::Has;

use self::ControllerButton::*;

pub enum ControllerButton {
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

impl ControllerButton {
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

pub struct ButtonFrame {
    y: bool,
    x: bool,
    b: bool,
    a: bool,
    right_sr: bool,
    right_sl: bool,
    r: bool,
    zr: bool,

    minus: bool,
    plus: bool,
    cr: bool,
    cl: bool,
    home: bool,
    capture: bool,

    down: bool,
    up: bool,
    right: bool,
    left: bool,
    left_sr: bool,
    left_sl: bool,
    l: bool,
    zl: bool,
}

impl<'a> From<&'a [u8]> for ButtonFrame {
    fn from(buf: &[u8]) -> ButtonFrame {
        ButtonFrame {
            y: buf.has(Y),
            x: buf.has(X),
            b: buf.has(B),
            a: buf.has(A),
            right_sr: buf.has(RightSr),
            right_sl: buf.has(RightSl),
            r: buf.has(R),
            zr: buf.has(Zr),

            minus: buf.has(Minus),
            plus: buf.has(Plus),
            cr: buf.has(Cr),
            cl: buf.has(Cl),
            home: buf.has(Home),
            capture: buf.has(Capture),

            down: buf.has(Down),
            up: buf.has(Up),
            right: buf.has(Right),
            left: buf.has(Left),
            left_sr: buf.has(LeftSr),
            left_sl: buf.has(LeftSl),
            l: buf.has(L),
            zl: buf.has(Zl),
        }
    }
}

impl ButtonFrame {
    pub fn new() -> ButtonFrame {
        ButtonFrame {
            y: false,
            x: false,
            b: false,
            a: false,
            right_sr: false,
            right_sl: false,
            r: false,
            zr: false,

            minus: false,
            plus: false,
            cr: false,
            cl: false,
            home: false,
            capture: false,

            down: false,
            up: false,
            right: false,
            left: false,
            left_sr: false,
            left_sl: false,
            l: false,
            zl: false,
        }
    }
}

impl<'a> Has<ControllerButton> for ButtonFrame {
    fn has(&self, btn: ControllerButton) -> bool {
        match btn {
            Y => self.y,
            X => self.x,
            B => self.b,
            A => self.a,
            RightSr => self.right_sr,
            RightSl => self.right_sl,
            R => self.r,
            Zr => self.zr,

            Minus => self.minus,
            Plus => self.plus,
            Cr => self.cr,
            Cl => self.cl,
            Home => self.home,
            Capture => self.capture,

            Down => self.down,
            Up => self.up,
            Right => self.right,
            Left => self.left,
            LeftSr => self.left_sr,
            LeftSl => self.left_sl,
            L => self.l,
            Zl => self.zl,
        }
    }
}

impl<'a> Has<ControllerButton> for &'a [u8] {
    fn has(&self, btn: ControllerButton) -> bool {
        self[btn.byte_offset()] & btn.bit_mask() > 0
    }
}
