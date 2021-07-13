pub(crate) const SCREEN_WIDTH: usize = 64;
pub(crate) const SCREEN_HEIGHT: usize = 32;

#[derive(Debug)]
pub(crate) struct Display {
    screen: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

impl Display {
    pub fn initialise() -> Self {
        Self {
            screen: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT],
        }
    }

    pub fn clear_screen(&mut self) {
        self.screen = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT];
        Display::draw();
    }

    // TODO: Write tests for this
    pub fn display_sprite(&mut self, location: (&u8, &u8), sprite: &[u8]) -> bool {
        let (x, y) = location;

        let mut did_overwrite = false;
        let mut y_offset = 0;

        for spr_row in sprite {
            for n in 0..8 {
                // calculate if we need to wrap around
                // part of the sprite
                let mut curr_y = *y as usize + y_offset;
                // In order to render the pixels in the correct order
                // we must print the most significant byte to the display first
                let mut curr_x = *x as usize + (8 - n);
                if curr_y >= SCREEN_HEIGHT {
                    curr_y %= SCREEN_HEIGHT;
                }

                if curr_x >= SCREEN_WIDTH {
                    curr_x %= SCREEN_WIDTH;
                }

                // extract the bit at position n of the byte
                let bit = (spr_row & (1 << n)) != 0;

                // indicate if the setting of the new pixel will
                // overwrite the previous pixel (i.e change state)
                if self.screen[curr_y][curr_x] != bit {
                    did_overwrite = true;
                }

                self.screen[curr_y][curr_x] ^= bit;
            }
            y_offset += 1;
        }

        did_overwrite
    }

    fn draw() {}
}

impl DebugDisplay for Display {
    fn view_state(&self) {
        for r in self.screen {
            for x in r {
                let p = if !x { "x" } else { " " };
                print!("{}", p);
            }
            println!("");
        }
    }
}

pub(crate) trait DebugDisplay {
    fn view_state(&self);
}
