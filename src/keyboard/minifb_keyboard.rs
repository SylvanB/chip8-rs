use minifb::Window;

use super::Keyboard;

pub(crate) struct MiniFbKeyboard {
    current_keydowns: Vec<u8>,
}

impl Keyboard for MiniFbKeyboard {
    fn update_state(&mut self, keys: &[u8]) {
        self.current_keydowns = keys.into();
    }

    fn get_current_keydowns(&self) -> &[u8] {
        &self.current_keydowns
    }
}

impl MiniFbKeyboard {
    pub fn initialise(_window: &Window) -> Self {
        Self {
            current_keydowns: vec![],
        }
    }
}