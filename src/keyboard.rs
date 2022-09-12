pub enum Keyboard {
    None,
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
}

impl Keyboard {
    // TODO : finish implementing this to return keycodes
    pub fn get_keycode(&self) -> Option<u8>{
        match *self {
            Keyboard::Key0 => Some(0x0),
            Keyboard::Key1 => Some(0x1),
            Keyboard::Key2 => Some(0x2),
            Keyboard::Key3 => Some(0x3),
            Keyboard::Key4 => Some(0x4),
            Keyboard::Key5 => Some(0x5),
            Keyboard::Key6 => Some(0x6),
            Keyboard::Key7 => Some(0x7),
            Keyboard::Key8 => Some(0x8),
            Keyboard::Key9 => Some(0x9),
            Keyboard::KeyA => Some(0xa),
            Keyboard::KeyB => Some(0xb),
            Keyboard::KeyC => Some(0xc),
            Keyboard::KeyD => Some(0xd),
            Keyboard::KeyE => Some(0xe),
            Keyboard::KeyF => Some(0xf),
            _ => None
        }
    }
}