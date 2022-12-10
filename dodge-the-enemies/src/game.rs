use std::borrow::{Borrow, BorrowMut};
use raylib::ffi::glad_glBindVertexShaderEXT;
use raylib::prelude::*;

use crate::player::Player;
use crate::game_state::GameState;
use crate::hud::Hud;
use crate::enemy::Enemy;
use crate::menu::Menu;

pub struct Game {
    pub is_running:bool,
    pub rl_handle:RaylibHandle,
    pub rl_thread:RaylibThread,
    pub player:Player,
    pub enemies:Vec<Enemy>,
    level:i32,
    game_state: GameState,
    hud: Hud,
    menu: Menu,
    window_width:i32,
    window_height:i32,
    audio:RaylibAudio,
    elapsed_time:f32,
    grace:f32,
}

impl Game {
    pub fn start() -> Game {
        let width = 1920;
        let height = 1080;

        let (mut rl, thread) = raylib::init()
            .size(1920, 1080)
            .title("Dodge The Enemies")
            .msaa_4x()
            .vsync()
            .build();

        let mut game = Game {
            is_running: true,
            rl_handle: rl,
            rl_thread: thread,
            player: Player::new(width, height),
            game_state: GameState::MENU,
            level: 1,
            hud: Hud::new(width, height),
            menu: Menu::new(width, height),
            enemies: Vec::new(),
            window_width: width,
            window_height: height,
            audio: RaylibAudio::init_audio_device(),
            elapsed_time: 0.0,
            grace: 0.0,
        };

        return game;
    }
}

impl Game {
    pub fn should_close(&self) -> bool {
        return self.rl_handle.window_should_close();
    }

    pub fn update(&mut self) {
        let delta_time = self.rl_handle.get_frame_time();
        self.elapsed_time += delta_time;
        self.grace += delta_time;

        match self.game_state {
            GameState::MENU => {
                self.menu.update(delta_time, self.rl_handle.borrow());
            },
            GameState::PLAYING => {
                if self.elapsed_time >= 30.0 {
                    self.elapsed_time = 0.0;
                    self.level += 1;
                    self.spawn_wave_for_level();
                    self.grace = 0.0;
                }

                self.player.update(delta_time, self.rl_handle.borrow_mut());

                for i in 0..self.enemies.len() {
                    self.enemies[i].update(delta_time);
                    if self.player.intercepts(self.enemies[i].borrow()) && self.grace > 5.0 {
                        self.player.health -= 1;
                    }
                }

                if self.player.health <= 0 {
                    self.enemies.clear();
                    self.game_state = GameState::GAME_OVER;
                }

                self.hud.update();
            },
            GameState::GAME_OVER => {
                if self.rl_handle.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    self.level = 1;
                    self.player.health = 100;
                    self.grace = 0.0;
                    self.spawn_wave_for_level();
                    self.game_state = GameState::PLAYING;
                }
            },
        }

    }

    pub fn render(&mut self) {
        let mut gfx = self.rl_handle.begin_drawing(&self.rl_thread);

        gfx.clear_background(Color::BLACK);

        match self.game_state {
            GameState::MENU => {
                self.menu.render(gfx.borrow_mut());
            },
            GameState::PLAYING => {
                self.player.render(gfx.borrow_mut());

                for i in 0..self.enemies.len() {
                    self.enemies[i].render(gfx.borrow_mut());
                }

                self.hud.render(gfx.borrow_mut(), self.player.borrow(), self.level);
            },
            GameState::GAME_OVER => {
                gfx.draw_text("Game Over",
                              self.window_width/2-measure_text("Game Over", 80)/2,
                              self.window_height/2-120,
                              80, Color::WHITE);
                gfx.draw_text(format!("Level: {}", self.level).as_str(),
                              self.window_width/2-measure_text(format!("Level: {}", self.level).as_str(), 40)/2,
                              self.window_height/2,
                              40, Color::WHITE);
                gfx.draw_text("Press ENTER to restart",
                              self.window_width/2-measure_text("Press ENTER to restart", 30)/2,
                              self.window_height-52,
                              30, Color::WHITE);
            },
        }

        gfx.draw_fps(10, 10);
    }

    fn spawn_wave_for_level(&mut self) {
        self.enemies.clear();
        match self.level {
            1 => {
                for i in 0..5 {
                    self.enemies.push(Enemy::new(self.window_width, self.window_height, 5));
                }
            },
            2 => {
                for i in 0..5 {
                    self.enemies.push(Enemy::new(self.window_width, self.window_height, 10));
                }
            },
            3 => {
                for i in 0..10 {
                    self.enemies.push(Enemy::new(self.window_width, self.window_height, 5));
                }
            },
            4 => {
                for i in 0..10 {
                    self.enemies.push(Enemy::new(self.window_width, self.window_height, 10));
                }
            },
            5 => {
                for i in 0..10 {
                    self.enemies.push(Enemy::new(self.window_width, self.window_height, 20));
                }
            },
            6 => {
                for i in 0..15 {
                    self.enemies.push(Enemy::new(self.window_width, self.window_height, 30));
                }
            }
            _ => {

            }
        }
    }

    fn start_callback(&mut self) {

    }

    fn quit_callback(&mut self) {

    }
}