mod player;
mod game;
mod game_state;
mod hud;
mod enemy;
mod menu;

use std::ptr::null;
use raylib::prelude::*;

use game::Game;

fn main() {
    let mut game = Game::start();

    while !game.should_close() {
        game.update();
        game.render();
    }
}
