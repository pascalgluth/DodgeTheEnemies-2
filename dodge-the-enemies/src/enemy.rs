use std::borrow::Borrow;
use raylib::prelude::*;
use rand::Rng;

pub struct Enemy {
    pub x_pos:i32,
    pub y_pos:i32,
    pub radius:i32,
    dx:i32,
    dy:i32,
    window_width:i32,
    window_height:i32,
    audio:RaylibAudio,
    bounce_sound:raylib::audio::Sound
}

impl Enemy {
    pub fn new(width:i32, height:i32, max_speed:i32) -> Enemy {
        let mut rng = rand::thread_rng();
        Enemy {
            x_pos: rng.gen_range::<i32, _>(0..width),
            y_pos: rng.gen_range::<i32, _>(0..height),
            radius: 35,
            dx: rng.gen_range::<i32, _>(-max_speed..max_speed),
            dy: rng.gen_range::<i32, _>(-max_speed..max_speed),
            window_width: width,
            window_height: height,
            audio: raylib::audio::RaylibAudio::init_audio_device(),
            bounce_sound: raylib::audio::Sound::load_sound("C:\\Users\\pasca\\Desktop\\Dev\\DodgeTheEnemies-2\\dodge-the-enemies\\target\\debug\\bounce.wav").unwrap(),
        }
    }
}

impl Enemy {
    pub fn update(&mut self, delta_time:f32) {
        let mut mov_x = self.dx as f32 * (delta_time * 100.0);
        let mut mov_y = self.dy as f32 * (delta_time * 100.0);

        if self.x_pos + (mov_x as i32) > self.window_width - self.radius || self.x_pos + (mov_x as i32) < self.radius {
            self.dx = -self.dx;
            self.audio.play_sound(self.bounce_sound.borrow());
        }
        if self.y_pos + (mov_y as i32) > self.window_height - self.radius || self.y_pos + (mov_y as i32) < self.radius {
            self.dy = -self.dy;
            self.audio.play_sound(self.bounce_sound.borrow());
        }

        mov_x = self.dx as f32 * (delta_time * 100.0);
        mov_y = self.dy as f32 * (delta_time * 100.0);

        self.x_pos += mov_x as i32;
        self.y_pos += mov_y as i32;
    }

    pub fn render(&mut self, gfx: &mut RaylibDrawHandle) {
        gfx.draw_circle(self.x_pos, self.y_pos, self.radius as f32, Color::RED);
    }
}