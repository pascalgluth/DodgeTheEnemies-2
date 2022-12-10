use raylib::misc::AsF32;
use raylib::prelude::*;

use crate::game::Game;
use crate::player::Player;

pub struct Hud {
    window_width:i32,
    window_height:i32,
}

impl Hud {
    pub fn new(width:i32, height:i32) -> Hud {
        Hud {
            window_width: width,
            window_height: height,
        }
    }
}

impl Hud {
    pub fn update(&self) {

    }

    pub fn render(&self, gfx: &mut RaylibDrawHandle, player: &Player, level:i32) {
        gfx.draw_text(format!("Health: {}", player.health).as_str(), 10, self.window_height-40, 30, Color::WHITE);
        gfx.draw_text(format!("Level: {}", level).as_str(), self.window_width-measure_text("Level: 00", 30)-5, self.window_height-40, 30, Color::WHITE);
    }
}