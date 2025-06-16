use super::*;
use once_cell::sync::Lazy;
use turbo::canvas::text_box::TextBox;
use std::sync::Mutex;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Global {
    pub sfx: bool,
    pub music: bool,
    pub options_button: Btn,
    pub options: bool,
    pub info_button: Btn,
    pub info: bool,
    pub menu_bounds: Bounds,
    pub music_toggle: Btn,
    pub sfx_toggle: Btn,
    pub reset_button: Btn,
    pub save_button: Btn,
    pub autosave: bool,
    pub autosave_toggle: Btn,
}

// The singleton instance
pub static GLOBAL: Lazy<Mutex<Global>> = Lazy::new(|| Mutex::new(Global::new()));
impl Global {
    // Private constructor
    fn new() -> Self {
        let menu_bounds = Bounds::new(0, 26, 96, 96);
        let spacing = 24;
        Global {
            sfx: true,
            music: true,
            options_button: Btn::new("gear".to_string(), Bounds::new(0,0,24,24), false, 1),
            options: false,
            info_button: Btn::new("i".to_string(), Bounds::new(26,0,24,24), false, 1),
            info: false,
            menu_bounds,
            music_toggle: Btn::new("toggle".to_string(), Bounds::new(menu_bounds.x()+16, menu_bounds.y()+spacing/3,16,16), false, 1),
            sfx_toggle: Btn::new("toggle".to_string(), Bounds::new(menu_bounds.x()+16, menu_bounds.y()+spacing/3+spacing,16,16), false, 1),
            save_button: Btn::new("SAVE".to_string(), Bounds::new(menu_bounds.x()+12, menu_bounds.y()+6+spacing * 2,72,16), true, 1),
            reset_button: Btn::new("RESET SAVE".to_string(), Bounds::new(menu_bounds.x()+12, menu_bounds.y()+2+spacing * 3,72,16), true, 1),
            autosave: true,
            autosave_toggle: Btn::new("toggle".to_string(), Bounds::new(menu_bounds.x()+16, menu_bounds.y()+spacing/3+spacing,16,16), false, 1),
        }
    }

    // Example function that can be called on the singleton
    pub fn play_sound(sound_id: u32) {
        let mut instance = GLOBAL.lock().unwrap();
    }

    pub fn update(&mut self, event_manager: &mut EventManager) {
        let p = pointer();
        self.options_button.update();
        if self.options_button.on_click() {
            self.options = !self.options
        } else if p.just_pressed() && !p.intersects_fixed(self.menu_bounds.x(), self.menu_bounds.y(), self.menu_bounds.w(), self.menu_bounds.h()) && self.options {
            self.options = false;
        }
        self.info_button.update();
        if self.info_button.on_click() {
            self.info = !self.info
        } else if p.just_pressed() && !p.intersects_fixed(self.menu_bounds.x(), self.menu_bounds.y(), self.menu_bounds.w(), self.menu_bounds.h()) && self.info {
            self.info = false;
        }
        if self.options {
            self.music_toggle.update();
            if self.music_toggle.on_click() {
                self.music = !self.music;
                if !self.music { audio::stop("loop"); }
                self.music_toggle.string = if self.music { "toggle".to_string() } else { "".to_string() };
            }
            // self.sfx_toggle.update();
            // if self.sfx_toggle.on_click() {
            //     self.sfx = !self.sfx;
            //     self.sfx_toggle.string = if self.sfx { "toggle".to_string() } else { "".to_string() };
            // }
            self.save_button.update();
            if self.save_button.on_click() {
                event_manager.trigger(Event::SaveGame);
            }
            self.reset_button.update();
            if self.reset_button.on_click() {
                event_manager.trigger(Event::ResetGame);
                self.options = false;
            }
            self.autosave_toggle.update();
            if self.autosave_toggle.on_click() {
                self.autosave = !self.autosave;
                self.autosave_toggle.string = if self.autosave { "toggle".to_string() } else { "".to_string() };
            }
        }

        if self.music && !audio::is_playing("loop") {
            audio::play("loop");
        }
    }

    pub fn draw(&self) {
        self.options_button.draw();
        self.info_button.draw();
        if self.options {
            rect!( 
                fixed = true,
                xy = self.menu_bounds.xy(),
                wh = self.menu_bounds.wh(),
                border_size = 1,
                border_radius = 2,
                color = 0x1f122bff,
                border_color = 0xffffffff,
            );
            self.music_toggle.draw();
            text!(
                "MUSIC",
                fixed = true,
                xy = (self.music_toggle.bounds.x() + self.music_toggle.bounds.w() as i32 + 6, self.music_toggle.bounds.center_y() - 4),
            );
            // self.sfx_toggle.draw();
            // text!(
            //     "SFX",
            //     fixed = true,
            //     xy = (self.sfx_toggle.bounds.x() + self.sfx_toggle.bounds.w() as i32 + 6, self.sfx_toggle.bounds.center_y() - 4),
            // );
            self.autosave_toggle.draw();
            text!(
                "AUTOSAVE",
                fixed = true,
                xy = (self.autosave_toggle.bounds.x() + self.autosave_toggle.bounds.w() as i32 + 6, self.autosave_toggle.bounds.center_y() - 4),
            );
            self.save_button.draw();
            self.reset_button.draw();
        }
        if self.info {
            rect!( 
                fixed = true,
                xy = self.menu_bounds.xy(),
                wh = self.menu_bounds.wh(),
                border_size = 1,
                border_radius = 2,
                color = 0x1f122bff,
                border_color = 0xffffffff,
            );
            let mut textbox = TextBox::new("alpha v0.2.1");
            textbox.set_fixed(true);
            textbox.set_size(88, 96);
            textbox.set_position(self.menu_bounds.x() + 4, self.menu_bounds.y() + 4);
            textbox.draw();
            let mut textbox = TextBox::new("game by jauntybot");
            textbox.set_fixed(true);
            textbox.set_size(88, 96);
            textbox.set_position(self.menu_bounds.x() + 4, self.menu_bounds.y() + 24);
            textbox.draw();
            let mut textbox = TextBox::new("music by zach jones");
            textbox.set_fixed(true);
            textbox.set_size(88, 96);
            textbox.set_position(self.menu_bounds.x() + 4, self.menu_bounds.y() + 44);
            textbox.draw();
            let mut textbox = TextBox::new("made fast using the Turbo engine");
            textbox.set_fixed(true);
            textbox.set_size(88, 96);
            textbox.set_position(self.menu_bounds.x() + 4, self.menu_bounds.y() + 74);
            textbox.draw();
        }
    }
}

pub enum Sounds {
    Click,
    Collect,
    Upgrade,
    Drone,
    Ship,
    Scan,
    Asteroid,
    Nebula,
    Fabricator,
    PopUp,
    PopDown,
    PopUpClick,
    PopDownClick,
    PopUpHover,
    PopDownHover,
}