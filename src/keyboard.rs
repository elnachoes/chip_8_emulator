#[derive(Debug, Clone, Copy)]
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
    // pub fn get_keycode(&self) -> Option<u8>{
    //     match *self {
    //         Keyboard::Key0 => Some(0x0),
    //         Keyboard::Key1 => Some(0x1),
    //         Keyboard::Key2 => Some(0x2),
    //         Keyboard::Key3 => Some(0x3),
    //         Keyboard::Key4 => Some(0x4),
    //         Keyboard::Key5 => Some(0x5),
    //         Keyboard::Key6 => Some(0x6),
    //         Keyboard::Key7 => Some(0x7),
    //         Keyboard::Key8 => Some(0x8),
    //         Keyboard::Key9 => Some(0x9),
    //         Keyboard::KeyA => Some(0xA),
    //         Keyboard::KeyB => Some(0xB),
    //         Keyboard::KeyC => Some(0xC),
    //         Keyboard::KeyD => Some(0xD),
    //         Keyboard::KeyE => Some(0xE),
    //         Keyboard::KeyF => Some(0xF),
    //         _ => Some(0x0)
    //     }
    // }

    pub fn get_keycode(&self) -> u8{
        match *self {
            Keyboard::Key0 => 0x0,
            Keyboard::Key1 => 0x1,
            Keyboard::Key2 => 0x2,
            Keyboard::Key3 => 0x3,
            Keyboard::Key4 => 0x4,
            Keyboard::Key5 => 0x5,
            Keyboard::Key6 => 0x6,
            Keyboard::Key7 => 0x7,
            Keyboard::Key8 => 0x8,
            Keyboard::Key9 => 0x9,
            Keyboard::KeyA => 0xA,
            Keyboard::KeyB => 0xB,
            Keyboard::KeyC => 0xC,
            Keyboard::KeyD => 0xD,
            Keyboard::KeyE => 0xE,
            Keyboard::KeyF => 0xF,
            _ => 0xff 
        }
    }
}