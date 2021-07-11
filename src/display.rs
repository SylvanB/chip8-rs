pub(crate) struct Display {
    screen: [[u8; 32]; 64],
}

impl Display {
    pub fn initialise() -> Self {
        Self {
            screen: [[0; 32]; 64],
        }
    }

    pub fn clear_screen(&mut self) {
        self.screen = [[0; 32]; 64];
        Display::redraw();
    }

    fn redraw() {}
}
