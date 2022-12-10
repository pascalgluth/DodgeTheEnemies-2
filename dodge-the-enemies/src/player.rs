use raylib::prelude::*;

use crate::enemy::Enemy;

pub struct Player {
    pub x_pos:i32,
    pub y_pos:i32,
    pub health:i32,
    radius:i32,
    speed:i32,
    window_width:i32,
    window_height:i32,
}

impl Player {
    pub fn new(width:i32, height:i32) -> Player {
        Player {
            x_pos: 0,
            y_pos: 0,
            health: 100,
            radius: 50,
            speed: 10,
            window_width: width,
            window_height: height,
        }
    }
}

impl Player {
    pub fn update(&mut self, delta_time:f32, rl: &mut RaylibHandle) {
        let movement = self.speed as f32 * (delta_time * 100.0);
        if rl.is_key_down(KeyboardKey::KEY_UP) && self.y_pos - movement as i32 - self.radius >= 0 {
            self.y_pos -= movement as i32;
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) && self.x_pos + movement as i32 + self.radius <= self.window_width {
            self.x_pos += movement as i32;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) && self.y_pos + movement as i32 + self.radius <= self.window_height {
            self.y_pos += movement as i32;
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) && self.x_pos - movement as i32 - self.radius >= 0 {
            self.x_pos -= movement as i32;
        }
    }

    pub fn render(&self, gfx: &mut RaylibDrawHandle) {
        gfx.draw_circle(self.x_pos, self.y_pos, self.radius as f32, Color::WHITE);
    }

    pub fn intercepts(&self, enemy: &Enemy) -> bool {
        let distance = f32::sqrt(i32::pow(enemy.x_pos - self.x_pos, 2) as f32 + i32::pow(enemy.y_pos - self.y_pos, 2) as f32) as i32;
        if distance < self.radius + enemy.radius {
            return true;
        }
        return false;
    }
}