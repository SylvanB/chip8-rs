use std::{cell::RefCell, rc::Rc};

use minifb::{Key, Window};

use super::Keyboard;

pub(crate) struct MiniFbKeyboard<'Keyboard> {
    window: &'Keyboard Rc<RefCell<Window>>,
    current_keydowns: Vec<u8>,
}

impl<'a> Keyboard for MiniFbKeyboard<'a> {
    fn update_state(&mut self) {
        // TODO: maybe update the current pressed keys more intelligently...
        self.current_keydowns = vec![];
        for k in self.window.borrow().get_keys().unwrap().into_iter() {
            match k {
                Key::Key0 => self.current_keydowns.push(0x0),
                Key::Key1 => self.current_keydowns.push(0x1),
                Key::Key2 => self.current_keydowns.push(0x2),
                Key::Key3 => self.current_keydowns.push(0x3),
                Key::Key4 => self.current_keydowns.push(0x4),
                Key::Key5 => self.current_keydowns.push(0x5),
                Key::Key6 => self.current_keydowns.push(0x6),
                Key::Key7 => self.current_keydowns.push(0x7),
                Key::Key8 => self.current_keydowns.push(0x8),
                Key::Key9 => self.current_keydowns.push(0x9),
                Key::A => self.current_keydowns.push(0xA),
                Key::B => self.current_keydowns.push(0xB),
                Key::C => self.current_keydowns.push(0xC),
                Key::D => self.current_keydowns.push(0xD),
                Key::E => self.current_keydowns.push(0xE),
                Key::F => self.current_keydowns.push(0xF),
                _ => continue,
            };
        }
    }

    fn get_current_keydowns(&self) -> &Vec<u8> {
        &self.current_keydowns
    }
}

impl<'a> MiniFbKeyboard<'a> {
    pub fn initialise(window: &'a Rc<RefCell<Window>>) -> Self {
        Self {
            window,
            current_keydowns: vec![],
        }
    }
}
