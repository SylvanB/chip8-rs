pub(crate) const SCREEN_WIDTH: usize = 64;
pub(crate) const SCREEN_HEIGHT: usize = 32;

#[derive(Debug)]
pub(crate) struct Display {
    pub screen: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

impl Display {
    pub fn initialise() -> Self {
        Self {
            screen: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT],
        }
    }

    pub fn clear_screen(&mut self) {
        self.screen = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT];
    }

    pub fn get_buffer(&self) -> Vec<u32> {
        self.screen
            .iter()
            .flatten()
            .map(|x| if *x { 0 } else { 0xFFFF_FFFF })
            .collect()
    }

    /// Draws a sprite at (x, y) using XOR and wraps around screen edges.
    ///
    /// Returns `true` (sets VF) if any pixel is turned **off**, i.e. an already-on
    /// pixel is XOR'd with a set sprite bit.
    pub fn display_sprite(&mut self, (x, y): (usize, usize), sprite: &[u8]) -> bool {
        let mut did_overwrite = false;

        for (y_offset, spr_row) in sprite.iter().enumerate() {
            for n in 0..8 {
                // Extract the bit at position n of the byte, drawing the most
                // significant bit first.
                let bit = (spr_row & (1 << n)) != 0;
                if !bit {
                    continue;
                }

                let curr_x = (x + (7 - n)) % SCREEN_WIDTH;
                let curr_y = (y + y_offset) % SCREEN_HEIGHT;

                // A collision occurs when an on-pixel is XOR'd with a set bit,
                // turning it off.
                if self.screen[curr_y][curr_x] {
                    did_overwrite = true;
                }

                self.screen[curr_y][curr_x] ^= bit;
            }
        }

        did_overwrite
    }
}

impl DebugDisplay for Display {
    fn view_state(&self) {
        for r in self.screen {
            for x in r {
                let p = if !x { "." } else { "X" };
                print!("{}", p);
            }
            println!();
        }
    }
}

#[allow(dead_code)] // used for debugging and in tests
pub(crate) trait DebugDisplay {
    fn view_state(&self);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set_pixel(display: &mut Display, x: usize, y: usize) {
        display.screen[y][x] = true;
    }

    #[test]
    fn drawing_sprite_sets_pixels() {
        let mut display = Display::initialise();
        display.display_sprite((2, 2), &[0b1100_0000]);

        assert!(display.screen[2][2]);
        assert!(display.screen[2][3]);
        assert!(!display.screen[2][4]);
    }

    #[test]
    fn drawing_sprite_wraps_around_edges() {
        let mut display = Display::initialise();
        // Two rows starting at the bottom-right corner; columns wrap left,
        // the second row wraps to the top.
        display.display_sprite(
            (SCREEN_WIDTH - 1, SCREEN_HEIGHT - 1),
            &[0b1100_0000, 0b1100_0000],
        );

        assert!(display.screen[SCREEN_HEIGHT - 1][SCREEN_WIDTH - 1]);
        assert!(display.screen[SCREEN_HEIGHT - 1][0]); // column wraps
        assert!(display.screen[0][SCREEN_WIDTH - 1]); // row wraps
        assert!(display.screen[0][0]); // both wrap
        assert!(!display.screen[SCREEN_HEIGHT - 1][1]); // column 65 mod 64 = 1, bit 5 not set
    }

    #[test]
    fn no_collision_when_pixel_turned_on() {
        let mut display = Display::initialise();
        let collided = display.display_sprite((0, 0), &[0b1000_0000]);

        assert!(!collided);
        assert!(display.screen[0][0]);
    }

    #[test]
    fn collision_flag_set_when_pixel_turned_off() {
        let mut display = Display::initialise();
        set_pixel(&mut display, 0, 0);

        let collided = display.display_sprite((0, 0), &[0b1000_0000]);

        assert!(collided);
        assert!(!display.screen[0][0]);
    }

    #[test]
    fn xor_drawing_twice_clears_pixel_and_sets_flag() {
        let mut display = Display::initialise();

        assert!(!display.display_sprite((0, 0), &[0b1000_0000]));
        assert!(display.display_sprite((0, 0), &[0b1000_0000]));
        assert!(!display.screen[0][0]);
    }
}
