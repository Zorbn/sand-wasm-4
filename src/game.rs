use crate::{particle_type::*, wasm4::*};

const SCREEN_SIZE: usize = 160;
const CHUNK_SIZE: usize = 160;
// The chunk bitmap is 2BPP so 4 pixels fit in a byte.
const CHUNK_BITMAP_LENGTH: usize = CHUNK_SIZE * CHUNK_SIZE / 4;
const AIR_COLOR: u8 = 0;
const PARTICLE_TYPES: [ParticleType; 2] = [
    // Air:
    ParticleType {
        color: AIR_COLOR,
        has_gravity: false,
    },
    // Sand:
    ParticleType {
        color: 1,
        has_gravity: true,
    },
];

// TODO: Add brushes that follow the 1BPP format.

pub struct Game {
    particles: [u8; CHUNK_BITMAP_LENGTH],
}

impl Game {
    pub const fn new() -> Self {
        Self {
            particles: [0x0; CHUNK_BITMAP_LENGTH],
        }
    }

    pub fn start(&mut self) {
        self.set_pixel(80, 0, 1);
        self.set_pixel(81, 0, 1);
        self.set_pixel(82, 0, 1);
        self.set_pixel(83, 0, 1);
        self.set_pixel(84, 0, 1);
    }

    pub fn update(&mut self, mouse: u8, mouse_x: i16, mouse_y: i16) {
        self.process_input(mouse, mouse_x, mouse_y);

        for y in (0..(CHUNK_SIZE - 1)).rev() {
            for x in 0..CHUNK_SIZE {
                let pixel = self.get_pixel(x, y);
                let particle_type = &PARTICLE_TYPES[pixel as usize];
                match particle_type.has_gravity {
                    true => {
                        let below_pixel = self.get_pixel(x, y + 1);
                        if below_pixel == AIR_COLOR {
                            self.set_pixel(x, y, AIR_COLOR);
                            self.set_pixel(x, y + 1, particle_type.color);
                        }
                    }
                    false => {}
                }
            }
        }

        self.draw();
    }

    fn process_input(&mut self, mouse: u8, mouse_x: i16, mouse_y: i16) {
        if mouse_x < 0
            || mouse_x >= CHUNK_SIZE as i16
            || mouse_y < 0
            || mouse_y >= CHUNK_SIZE as i16
        {
            return;
        }

        if mouse & MOUSE_LEFT != 0 {
            self.set_pixel(mouse_x as usize, mouse_y as usize, 1);
        } else if mouse & MOUSE_RIGHT != 0 {
            self.set_pixel(mouse_x as usize, mouse_y as usize, 0);
        }
    }

    fn draw(&mut self) {
        unsafe { *DRAW_COLORS = 0x4321 }
        Game::mem_blit(
            &self.particles,
            0,
            0,
            CHUNK_SIZE,
            CHUNK_SIZE,
        );
        unsafe { *DRAW_COLORS = 2 }
        text("hi", 0, 0);
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: u8) {
        // The byte index into the framebuffer that contains (x, y)
        let i = (y * CHUNK_SIZE + x) >> 2;

        // Calculate the bits within the byte that corresponds to our position
        // let shift = (3 - (x as u8 & 0b11)) << 1;
        let shift = (x as u8 & 0b11) << 1;
        let mask = 0b11 << shift;

        let color = color & 0b11;
        self.particles[i] = (color << shift) | (self.particles[i] & !mask);
    }

    fn get_pixel(&mut self, x: usize, y: usize) -> u8 {
        // The byte index into the framebuffer that contains (x, y)
        let i = (y * CHUNK_SIZE + x) >> 2;

        // Calculate the bits within the byte that corresponds to our position
        // let shift = (3 - (x as u8 & 0b11)) << 1;
        let shift = (x as u8 & 0b11) << 1;
        let mask = 0b11 << shift;

        (self.particles[i] & mask) >> shift
    }

    // Blit a 2BPP sprite to the screen by using memcpy on each row.
    fn mem_blit(sprite: &[u8], x: i32, y: i32, width: usize, height: usize) {
        let dst_x = x.max(0) as usize;
        let src_x = (dst_x as i32 - x) as usize;
        let dst_y = y.max(0) as usize;
        let src_y = (dst_y as i32 - y) as usize;
        let visible_width = ((width - src_x + dst_x).min(SCREEN_SIZE) - dst_x) >> 2;
        let visible_height = (height - src_y + dst_y).min(SCREEN_SIZE) - dst_y;

        let framebuffer = unsafe { &mut *FRAMEBUFFER };

        for iy in 0..visible_height {
            let src_i = ((src_y + iy) * width + src_x) >> 2;
            let dst_i = ((dst_y + iy) * SCREEN_SIZE + dst_x) >> 2;
            framebuffer[dst_i..(dst_i+visible_width)].copy_from_slice(&sprite[src_i..(src_i+visible_width)]);
        }
    }
}
