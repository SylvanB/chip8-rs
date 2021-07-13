#[derive(Debug)]
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
        Display::draw();
    }

    fn draw() {}
}

impl DebugDisplay for Display {
    fn view_state(&self) {
        for r in self.screen {
            for x in r {
                print!("{:02x}", x);
            }
            println!("");
        }
    }
}

pub(crate) trait DebugDisplay {
    fn view_state(&self);
}
