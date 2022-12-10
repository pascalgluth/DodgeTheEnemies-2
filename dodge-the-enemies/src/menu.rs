use raylib::prelude::*;

pub struct Menu {
    window_width:i32,
    window_height:i32,
    menu_index:i32,
    on_start:Option<fn()>,
    on_quit:Option<fn()>,
}

impl Menu {
    pub fn new(width:i32, height:i32) -> Menu {
        Menu {
            window_width: width,
            window_height: height,
            menu_index: 0,
            on_start: None,
            on_quit: None,
        }
    }
}

impl Menu {
    pub fn set_callback(&mut self, on_start:fn(), on_quit:fn()) {
        self.on_start? = on_start;
        self.on_quit? = on_quit;
    }

    pub fn update(&mut self, delta_time:f32, rl: &RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_UP) {
            if self.menu_index > 0 {
                self.menu_index -= 1;
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
            if self.menu_index < 2 {
                self.menu_index += 1;
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
            match self.menu_index {
                0 => self.on_start(),
                2 => self.on_quit(),
                _ => { }
            }
        }
    }

    pub fn render(&mut self, gfx: &mut RaylibDrawHandle) {
        gfx.draw_text("Dodge the Enemies",
                      self.window_width/2-measure_text("Dodge the Enemies", 80)/2,
                      200,
                      80, Color::SKYBLUE);

        match self.menu_index {
            0 => {
                gfx.draw_text("Start",
                              self.window_width/2-measure_text("Start", 50)/2,
                              500,
                              50, Color::BLUE);
                gfx.draw_text("Settings",
                              self.window_width/2-measure_text("Settings", 50)/2,
                              600,
                              50, Color::WHITE);
                gfx.draw_text("Quit",
                              self.window_width/2-measure_text("Quit", 50)/2,
                              700,
                              50, Color::WHITE);
            },
            1 => {
                gfx.draw_text("Start",
                              self.window_width/2-measure_text("Start", 50)/2,
                              500,
                              50, Color::WHITE);
                gfx.draw_text("Settings",
                              self.window_width/2-measure_text("Settings", 50)/2,
                              600,
                              50, Color::BLUE);
                gfx.draw_text("Quit",
                              self.window_width/2-measure_text("Quit", 50)/2,
                              700,
                              50, Color::WHITE);
            },
            2 => {
                gfx.draw_text("Start",
                              self.window_width/2-measure_text("Start", 50)/2,
                              500,
                              50, Color::WHITE);
                gfx.draw_text("Settings",
                              self.window_width/2-measure_text("Settings", 50)/2,
                              600,
                              50, Color::WHITE);
                gfx.draw_text("Quit",
                              self.window_width/2-measure_text("Quit", 50)/2,
                              700,
                              50, Color::BLUE);
            },
            _ => {

            }
        }
    }
}