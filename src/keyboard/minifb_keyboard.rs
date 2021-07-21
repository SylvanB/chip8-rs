use std::{
    cell::RefCell,
    convert::{TryFrom, TryInto},
    error::Error,
    rc::Rc,
};

use minifb::{Key, Window};

use super::Keyboard;

pub(crate) struct MiniFbKeyboard<'keyboard> {
    window: &'keyboard Rc<RefCell<Window>>,
    current_keydowns: Vec<u8>,
}

impl<'a> Keyboard for MiniFbKeyboard<'a> {
    fn update_state(&mut self, keys: &[u8]) {
        // TODO: maybe update the current pressed keys more intelligently...
        self.current_keydowns = keys.into();
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
