use rdev::{Button, Key};

pub fn mouse_button_code_convert(rdev_button: Button) -> Result<u32, Button> {
    match rdev_button {
        Button::Left => Ok(0),
        Button::Middle => Ok(2),
        Button::Right => Ok(1),
        Button::Unknown(code) => match code {
            8 | 19 => Ok(3),
            9 | 20 => Ok(4),
            _ => Err(rdev_button),
        },
    }
}

pub fn keycode_convert(rdev_key: Key) -> Result<u32, Key> {
    match rdev_key {
        Key::Backspace => Ok(8),
        Key::Tab => Ok(9),
        Key::Return => Ok(13),
        Key::Pause => Ok(19),
        Key::CapsLock => Ok(20),
        Key::Escape => Ok(27),
        Key::Space => Ok(32),
        Key::PageUp => Ok(33),
        Key::PageDown => Ok(34),
        Key::End => Ok(35),
        Key::Home => Ok(36),
        Key::LeftArrow => Ok(37),
        Key::UpArrow => Ok(38),
        Key::RightArrow => Ok(39),
        Key::DownArrow => Ok(40),
        Key::PrintScreen => Ok(44),
        Key::Insert => Ok(45),
        Key::Delete => Ok(46),
        Key::Num0 => Ok(48),
        Key::Num1 => Ok(49),
        Key::Num2 => Ok(50),
        Key::Num3 => Ok(51),
        Key::Num4 => Ok(52),
        Key::Num5 => Ok(53),
        Key::Num6 => Ok(54),
        Key::Num7 => Ok(55),
        Key::Num8 => Ok(56),
        Key::Num9 => Ok(57),
        Key::KeyA => Ok(65),
        Key::KeyB => Ok(66),
        Key::KeyC => Ok(67),
        Key::KeyD => Ok(68),
        Key::KeyE => Ok(69),
        Key::KeyF => Ok(70),
        Key::KeyG => Ok(71),
        Key::KeyH => Ok(72),
        Key::KeyI => Ok(73),
        Key::KeyJ => Ok(74),
        Key::KeyK => Ok(75),
        Key::KeyL => Ok(76),
        Key::KeyM => Ok(77),
        Key::KeyN => Ok(78),
        Key::KeyO => Ok(79),
        Key::KeyP => Ok(80),
        Key::KeyQ => Ok(81),
        Key::KeyR => Ok(82),
        Key::KeyS => Ok(83),
        Key::KeyT => Ok(84),
        Key::KeyU => Ok(85),
        Key::KeyV => Ok(86),
        Key::KeyW => Ok(87),
        Key::KeyX => Ok(88),
        Key::KeyY => Ok(89),
        Key::KeyZ => Ok(90),
        Key::MetaLeft => Ok(91),
        Key::MetaRight => Ok(92),
        Key::Kp0 => Ok(96),
        Key::Kp1 => Ok(97),
        Key::Kp2 => Ok(98),
        Key::Kp3 => Ok(99),
        Key::Kp4 => Ok(100),
        Key::Kp5 => Ok(101),
        Key::Kp6 => Ok(102),
        Key::Kp7 => Ok(103),
        Key::Kp8 => Ok(104),
        Key::Kp9 => Ok(105),
        Key::KpMultiply => Ok(106),
        Key::KpPlus => Ok(107),
        Key::KpMinus => Ok(109),
        Key::KpDelete => Ok(110),
        Key::KpDivide => Ok(111),
        Key::F1 => Ok(112),
        Key::F2 => Ok(113),
        Key::F3 => Ok(114),
        Key::F4 => Ok(115),
        Key::F5 => Ok(116),
        Key::F6 => Ok(117),
        Key::F7 => Ok(118),
        Key::F8 => Ok(119),
        Key::F9 => Ok(120),
        Key::F10 => Ok(121),
        Key::F11 => Ok(122),
        Key::F12 => Ok(123),
        Key::ScrollLock => Ok(145),
        Key::ShiftLeft => Ok(160),
        Key::ShiftRight => Ok(161),
        Key::ControlLeft => Ok(162),
        Key::ControlRight => Ok(163),
        Key::Alt => Ok(164),
        Key::AltGr => Ok(165),
        Key::SemiColon => Ok(186),
        Key::Equal => Ok(187),
        Key::Comma => Ok(188),
        Key::Minus => Ok(189),
        Key::Dot => Ok(190),
        Key::Slash => Ok(191),
        Key::BackQuote => Ok(192),
        Key::LeftBracket => Ok(219),
        Key::BackSlash => Ok(220),
        Key::RightBracket => Ok(221),
        Key::Quote => Ok(222),
        Key::NumLock => Ok(144),
        Key::KpReturn => Ok(1025),
        // Menu
        Key::Unknown(135) => Ok(93),
        _ => Err(rdev_key),
    }
}
