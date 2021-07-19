use super::Keyboard;

/// Used exclusively for testing components that rely on a keyboard
/// but the dependancy on the Window is not a good fit for testing.
pub(crate) struct DummyKeyboard {
    pub curr_keydowns: Vec<u8>,
}

impl Keyboard for DummyKeyboard {
    fn update_state(&mut self) {
        // Do nothing
        // During testting it is intended that the developer will manipulate the keybords state
        // externally. This is done by accessing curr_keydowns.
    }

    fn get_current_keydowns(&self) -> &Vec<u8> {
        &self.curr_keydowns
    }
}

impl DummyKeyboard {
    pub fn initialise() -> Self {
        Self {
            curr_keydowns: vec![],
        }
    }

    fn get_current_keydowns(&self) -> &Vec<u8> {
        &self.curr_keydowns
    }
}
