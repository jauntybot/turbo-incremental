use super::*;
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Sfx {
    pub sfx: bool,
    pub music: bool,
    pub menu_button: Btn,
    pub menu: bool,
    pub menu_bounds: Bounds,
    pub music_toggle: Btn,
    pub sfx_toggle: Btn,
    pub reset_button: Btn,
    pub save_button: Btn,
    pub autosave: bool,
    pub autosave_toggle: Btn,
}

// The singleton instance
pub static SFX: Lazy<Mutex<Sfx>> = Lazy::new(|| Mutex::new(Sfx::new()));
impl Sfx {
    // Private constructor
    fn new() -> Self {
        let menu_bounds = Bounds::new(0, 26, 80, 120);
        let spacing = 24;
        Sfx {
            sfx: true,
            music: true,
            menu_button: Btn::new("gear".to_string(), Bounds::new(0,0,24,24), false, 1),
            menu: false,
            menu_bounds,
            music_toggle: Btn::new("toggle".to_string(), Bounds::new(menu_bounds.x()+spacing/3, menu_bounds.y()+spacing/3,16,16), false, 1),
            sfx_toggle: Btn::new("toggle".to_string(), Bounds::new(menu_bounds.x()+spacing/3, menu_bounds.y()+spacing/3+spacing,16,16), false, 1),
            save_button: Btn::new("SAVE".to_string(), Bounds::new(menu_bounds.x()+spacing/3, menu_bounds.y()+6+spacing * 3,64,16), true, 1),
            reset_button: Btn::new("RESET SAVE".to_string(), Bounds::new(menu_bounds.x()+spacing/3, menu_bounds.y()+2+spacing * 4,64,16), true, 1),
            autosave: true,
            autosave_toggle: Btn::new("toggle".to_string(), Bounds::new(menu_bounds.x()+spacing/3, menu_bounds.y()+spacing/3+spacing * 2,16,16), false, 1),
        }
    }

    // Example function that can be called on the singleton
    pub fn play_sound(sound_id: u32) {
        let mut instance = SFX.lock().unwrap();
    }

    pub fn update(&mut self, state: &mut GameState) {
        let p = pointer();
        self.menu_button.update();
        if self.menu_button.on_click() {
            self.menu = !self.menu
        } else if p.just_pressed() && !p.intersects_fixed(self.menu_bounds.x(), self.menu_bounds.y(), self.menu_bounds.w(), self.menu_bounds.h()) && self.menu {
            self.menu = false;
        }
        if self.menu {
            self.music_toggle.update();
            if self.music_toggle.on_click() {
                self.music = !self.music;
                if !self.music { audio::stop("droplet"); }
                self.music_toggle.string = if self.music { "toggle".to_string() } else { "".to_string() };
            }
            self.sfx_toggle.update();
            if self.sfx_toggle.on_click() {
                self.sfx = !self.sfx;
                self.sfx_toggle.string = if self.sfx { "toggle".to_string() } else { "".to_string() };
            }
            self.save_button.update();
            if self.save_button.on_click() {
                state.save_local();
            }
            self.reset_button.update();
            if self.reset_button.on_click() {
                self.menu = false;
                *state = GameState::new();
            }
            self.autosave_toggle.update();
            if self.autosave_toggle.on_click() {
                self.autosave = !self.autosave;
                self.autosave_toggle.string = if self.autosave { "toggle".to_string() } else { "".to_string() };
            }
        }

        if self.music && !audio::is_playing("droplet") {
            //audio::play("droplet");
        }
    }

    pub fn draw(&self) {
        self.menu_button.draw();
        if self.menu {
            rect!( 
                fixed = true,
                xy = self.menu_bounds.xy(),
                wh = self.menu_bounds.wh(),
                border_size = 1,
                border_radius = 2,
                color = 0x222034ff,
                border_color = 0xffffffff,
            );
            self.music_toggle.draw();
            text!(
                "MUSIC",
                fixed = true,
                xy = (self.music_toggle.bounds.x() + self.music_toggle.bounds.w() as i32 + 6, self.music_toggle.bounds.center_y() - 4),
            );
            self.sfx_toggle.draw();
            text!(
                "SFX",
                fixed = true,
                xy = (self.sfx_toggle.bounds.x() + self.sfx_toggle.bounds.w() as i32 + 6, self.sfx_toggle.bounds.center_y() - 4),
            );
            self.autosave_toggle.draw();
            text!(
                "AUTOSAVE",
                fixed = true,
                xy = (self.music_toggle.bounds.x() + self.autosave_toggle.bounds.w() as i32 + 6, self.autosave_toggle.bounds.center_y() - 4),
            );
            self.save_button.draw();
            self.reset_button.draw();
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