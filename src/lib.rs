#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
mod game;
mod particle_type;

use game::Game;
use wasm4::*;

#[rustfmt::skip]
const SMILEY: [u8; 8] = [
    0b11000011,
    0b10000001,
    0b00100100,
    0b00100100,
    0b00000000,
    0b00100100,
    0b10011001,
    0b11000011,
];

static mut GAME: Game = Game::new();

#[no_mangle]
unsafe fn start() {
    GAME.start();
}

#[no_mangle]
unsafe fn update() {
    GAME.update(*MOUSE_BUTTONS, *MOUSE_X, *MOUSE_Y);
}
