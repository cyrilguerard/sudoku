#[macro_use]
extern crate lazy_static;

use crate::game::Game;

mod board;
mod game;
mod generator;
mod input;
mod render;
mod solver;

pub fn play() {
    Game::new().play();
}
