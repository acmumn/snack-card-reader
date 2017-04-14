use num::FromPrimitive;
use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Key {
    Esc = 1,
    Num1 = 2,
    Num2 = 3,
    Num3 = 4,
    Num4 = 5,
    Num5 = 6,
    Num6 = 7,
    Num7 = 8,
    Num8 = 9,
    Num9 = 10,
    Num0 = 11,
    Minus = 12,
    Equal = 13,
    Backspace = 14,
    Tab = 15,
    Q = 16,
    W = 17,
    E = 18,
    R = 19,
    T = 20,
    Y = 21,
    U = 22,
    I = 23,
    O = 24,
    P = 25,
    LeftBrace = 26,
    RightBrace = 27,
    Enter = 28,
    LeftCtrl = 29,
    A = 30,
    S = 31,
    D = 32,
    F = 33,
    G = 34,
    H = 35,
    J = 36,
    K = 37,
    L = 38,
    Semicolon = 39,
    Apostrophe = 40,
    Grave = 41,
    LeftShift = 42,
    Backslash = 43,
    Z = 44,
    X = 45,
    C = 46,
    V = 47,
    B = 48,
    N = 49,
    M = 50,
    Comma = 51,
    Dot = 52,
    Slash = 53,
    RightShift = 54,
    KPAsterisk = 55,
    LeftAlt = 56,
    Space = 57,
    CapsLock = 58,
    F1 = 59,
    F2 = 60,
    F3 = 61,
    F4 = 62,
    F5 = 63,
    F6 = 64,
    F7 = 65,
    F8 = 66,
    F9 = 67,
    F10 = 68,
    NumLock = 69,
    ScrollLock = 70,
    KP7 = 71,
    KP8 = 72,
    KP9 = 73,
    KPMinus = 74,
    KP4 = 75,
    KP5 = 76,
    KP6 = 77,
    KPPlus = 78,
    KP1 = 79,
    KP2 = 80,
    KP3 = 81,
    KP0 = 82,
    KPDot = 83,
}

impl Key {
    pub fn to_str(&self) -> &'static str {
        match *self {
            Key::Esc => "Esc",
            Key::Num1 => "1",
            Key::Num2 => "2",
            Key::Num3 => "3",
            Key::Num4 => "4",
            Key::Num5 => "5",
            Key::Num6 => "6",
            Key::Num7 => "7",
            Key::Num8 => "8",
            Key::Num9 => "9",
            Key::Num0 => "0",
            Key::Minus => "-",
            Key::Equal => "=",
            Key::Backspace => "Backspace",
            Key::Tab => "Tab",
            Key::Q => "q",
            Key::W => "w",
            Key::E => "e",
            Key::R => "r",
            Key::T => "t",
            Key::Y => "y",
            Key::U => "u",
            Key::I => "i",
            Key::O => "o",
            Key::P => "p",
            Key::LeftBrace => "[",
            Key::RightBrace => "]",
            Key::Enter => "Enter",
            Key::LeftCtrl => "LeftCtrl",
            Key::A => "a",
            Key::S => "s",
            Key::D => "d",
            Key::F => "f",
            Key::G => "g",
            Key::H => "h",
            Key::J => "j",
            Key::K => "k",
            Key::L => "l",
            Key::Semicolon => ";",
            Key::Apostrophe => "'",
            Key::Grave => "`",
            Key::LeftShift => "LeftShift",
            Key::Backslash => "\\",
            Key::Z => "z",
            Key::X => "x",
            Key::C => "c",
            Key::V => "v",
            Key::B => "b",
            Key::N => "n",
            Key::M => "m",
            Key::Comma => ",",
            Key::Dot => ".",
            Key::Slash => "/",
            Key::RightShift => "RightShift",
            Key::KPAsterisk => "*",
            Key::LeftAlt => "LeftAlt",
            Key::Space => " ",
            Key::CapsLock => "CapsLock",
            Key::F1 => "F1",
            Key::F2 => "F2",
            Key::F3 => "F3",
            Key::F4 => "F4",
            Key::F5 => "F5",
            Key::F6 => "F6",
            Key::F7 => "F7",
            Key::F8 => "F8",
            Key::F9 => "F9",
            Key::F10 => "F10",
            Key::NumLock => "NumLock",
            Key::ScrollLock => "ScrollLock",
            Key::KP7 => "7",
            Key::KP8 => "8",
            Key::KP9 => "9",
            Key::KPMinus => "-",
            Key::KP4 => "4",
            Key::KP5 => "5",
            Key::KP6 => "6",
            Key::KPPlus => "+",
            Key::KP1 => "1",
            Key::KP2 => "2",
            Key::KP3 => "3",
            Key::KP0 => "0",
            Key::KPDot => ".",
        }
    }
}

impl Display for Key {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "{}", self.to_str())
    }
}

impl FromPrimitive for Key {
    fn from_i64(n: i64) -> Option<Key> {
        match n {
            1 => Some(Key::Esc),
            2 => Some(Key::Num1),
            3 => Some(Key::Num2),
            4 => Some(Key::Num3),
            5 => Some(Key::Num4),
            6 => Some(Key::Num5),
            7 => Some(Key::Num6),
            8 => Some(Key::Num7),
            9 => Some(Key::Num8),
            10 => Some(Key::Num9),
            11 => Some(Key::Num0),
            12 => Some(Key::Minus),
            13 => Some(Key::Equal),
            14 => Some(Key::Backspace),
            15 => Some(Key::Tab),
            16 => Some(Key::Q),
            17 => Some(Key::W),
            18 => Some(Key::E),
            19 => Some(Key::R),
            20 => Some(Key::T),
            21 => Some(Key::Y),
            22 => Some(Key::U),
            23 => Some(Key::I),
            24 => Some(Key::O),
            25 => Some(Key::P),
            26 => Some(Key::LeftBrace),
            27 => Some(Key::RightBrace),
            28 => Some(Key::Enter),
            29 => Some(Key::LeftCtrl),
            30 => Some(Key::A),
            31 => Some(Key::S),
            32 => Some(Key::D),
            33 => Some(Key::F),
            34 => Some(Key::G),
            35 => Some(Key::H),
            36 => Some(Key::J),
            37 => Some(Key::K),
            38 => Some(Key::L),
            39 => Some(Key::Semicolon),
            40 => Some(Key::Apostrophe),
            41 => Some(Key::Grave),
            42 => Some(Key::LeftShift),
            43 => Some(Key::Backslash),
            44 => Some(Key::Z),
            45 => Some(Key::X),
            46 => Some(Key::C),
            47 => Some(Key::V),
            48 => Some(Key::B),
            49 => Some(Key::N),
            50 => Some(Key::M),
            51 => Some(Key::Comma),
            52 => Some(Key::Dot),
            53 => Some(Key::Slash),
            54 => Some(Key::RightShift),
            55 => Some(Key::KPAsterisk),
            56 => Some(Key::LeftAlt),
            57 => Some(Key::Space),
            58 => Some(Key::CapsLock),
            59 => Some(Key::F1),
            60 => Some(Key::F2),
            61 => Some(Key::F3),
            62 => Some(Key::F4),
            63 => Some(Key::F5),
            64 => Some(Key::F6),
            65 => Some(Key::F7),
            66 => Some(Key::F8),
            67 => Some(Key::F9),
            68 => Some(Key::F10),
            69 => Some(Key::NumLock),
            70 => Some(Key::ScrollLock),
            71 => Some(Key::KP7),
            72 => Some(Key::KP8),
            73 => Some(Key::KP9),
            74 => Some(Key::KPMinus),
            75 => Some(Key::KP4),
            76 => Some(Key::KP5),
            77 => Some(Key::KP6),
            78 => Some(Key::KPPlus),
            79 => Some(Key::KP1),
            80 => Some(Key::KP2),
            81 => Some(Key::KP3),
            82 => Some(Key::KP0),
            83 => Some(Key::KPDot),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Key> {
        match n {
            1 => Some(Key::Esc),
            2 => Some(Key::Num1),
            3 => Some(Key::Num2),
            4 => Some(Key::Num3),
            5 => Some(Key::Num4),
            6 => Some(Key::Num5),
            7 => Some(Key::Num6),
            8 => Some(Key::Num7),
            9 => Some(Key::Num8),
            10 => Some(Key::Num9),
            11 => Some(Key::Num0),
            12 => Some(Key::Minus),
            13 => Some(Key::Equal),
            14 => Some(Key::Backspace),
            15 => Some(Key::Tab),
            16 => Some(Key::Q),
            17 => Some(Key::W),
            18 => Some(Key::E),
            19 => Some(Key::R),
            20 => Some(Key::T),
            21 => Some(Key::Y),
            22 => Some(Key::U),
            23 => Some(Key::I),
            24 => Some(Key::O),
            25 => Some(Key::P),
            26 => Some(Key::LeftBrace),
            27 => Some(Key::RightBrace),
            28 => Some(Key::Enter),
            29 => Some(Key::LeftCtrl),
            30 => Some(Key::A),
            31 => Some(Key::S),
            32 => Some(Key::D),
            33 => Some(Key::F),
            34 => Some(Key::G),
            35 => Some(Key::H),
            36 => Some(Key::J),
            37 => Some(Key::K),
            38 => Some(Key::L),
            39 => Some(Key::Semicolon),
            40 => Some(Key::Apostrophe),
            41 => Some(Key::Grave),
            42 => Some(Key::LeftShift),
            43 => Some(Key::Backslash),
            44 => Some(Key::Z),
            45 => Some(Key::X),
            46 => Some(Key::C),
            47 => Some(Key::V),
            48 => Some(Key::B),
            49 => Some(Key::N),
            50 => Some(Key::M),
            51 => Some(Key::Comma),
            52 => Some(Key::Dot),
            53 => Some(Key::Slash),
            54 => Some(Key::RightShift),
            55 => Some(Key::KPAsterisk),
            56 => Some(Key::LeftAlt),
            57 => Some(Key::Space),
            58 => Some(Key::CapsLock),
            59 => Some(Key::F1),
            60 => Some(Key::F2),
            61 => Some(Key::F3),
            62 => Some(Key::F4),
            63 => Some(Key::F5),
            64 => Some(Key::F6),
            65 => Some(Key::F7),
            66 => Some(Key::F8),
            67 => Some(Key::F9),
            68 => Some(Key::F10),
            69 => Some(Key::NumLock),
            70 => Some(Key::ScrollLock),
            71 => Some(Key::KP7),
            72 => Some(Key::KP8),
            73 => Some(Key::KP9),
            74 => Some(Key::KPMinus),
            75 => Some(Key::KP4),
            76 => Some(Key::KP5),
            77 => Some(Key::KP6),
            78 => Some(Key::KPPlus),
            79 => Some(Key::KP1),
            80 => Some(Key::KP2),
            81 => Some(Key::KP3),
            82 => Some(Key::KP0),
            83 => Some(Key::KPDot),
            _ => None,
        }
    }
}
