pub mod dummy_keyboard;
pub mod minifb_keyboard;

pub(crate) trait Keyboard {
    fn update_state(&mut self, keys: &[u8]);

    fn get_current_keydowns(&self) -> &[u8];
}
