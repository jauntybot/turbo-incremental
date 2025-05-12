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
}

// The singleton instance
pub static SFX: Lazy<Mutex<Sfx>> = Lazy::new(|| Mutex::new(Sfx::new()));
impl Sfx {
    // Private constructor
    fn new() -> Self {
        let menu_bounds = Bounds::new(576, 336, 64, 120);
        Sfx {
            sfx: true,
            music: true,
            menu_button: Btn::new("gear".to_string(), Bounds::new(640-24,456,24,24), false, 1),
            menu: false,
            menu_bounds,
            music_toggle: Btn::new("toggle".to_string(), Bounds::new(menu_bounds.x()+8, menu_bounds.y()+8,16,16), false, 1),
            sfx_toggle: Btn::new("toggle".to_string(), Bounds::new(menu_bounds.x()+8, menu_bounds.y()+36,16,16), false, 1),
        }
    }

    // Example function that can be called on the singleton
    pub fn play_sound(sound_id: u32) {
        let mut instance = SFX.lock().unwrap();
    }

    pub fn update(&mut self) {
        self.menu_button.update();
        if self.menu_button.on_click() {
            self.menu = !self.menu
        }
        if self.menu {
            self.music_toggle.update();
            if self.music_toggle.on_click() {
                self.music = !self.music;
                self.music_toggle.string = if self.music { "toggle".to_string() } else { "".to_string() };
            }
            self.sfx_toggle.update();
            if self.sfx_toggle.on_click() {
                self.sfx = !self.sfx;
                self.sfx_toggle.string = if self.sfx { "toggle".to_string() } else { "".to_string() };
            }
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
            self.sfx_toggle.draw();
            self.music_toggle.draw();
            text!(
                "MUSIC",
                fixed = true,
                xy = (self.music_toggle.bounds.x() + self.music_toggle.bounds.w() as i32 + 6, self.music_toggle.bounds.center_y() - 4),
            );
            text!(
                "SFX",
                fixed = true,
                xy = (self.sfx_toggle.bounds.x() + self.sfx_toggle.bounds.w() as i32 + 6, self.sfx_toggle.bounds.center_y() - 4),
            );
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