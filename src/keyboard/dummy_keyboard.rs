use super::Keyboard;

/// Used exclusively for testing components that rely on a keyboard
/// but the dependency on the Window is not a good fit for testing.
#[allow(dead_code)] // only used from within #[cfg(test)] modules
pub(crate) struct DummyKeyboard {
    pub curr_keydowns: Vec<u8>,
}

impl Keyboard for DummyKeyboard {
    fn update_state(&mut self, keys: &[u8]) {
        self.curr_keydowns = keys.into();
    }

    fn get_current_keydowns(&self) -> &[u8] {
        &self.curr_keydowns
    }
}

#[allow(dead_code)] // only used from within #[cfg(test)] modules
impl DummyKeyboard {
    pub fn initialise() -> Self {
        Self {
            curr_keydowns: vec![],
        }
    }
}