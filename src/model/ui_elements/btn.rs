use super::*;

#[turbo::serialize]
#[derive(PartialEq, Eq)]
pub enum BtnState {
    Disabled,
    Normal,
    Hovered,
    Pressed,
}
impl BtnState {
    pub fn colors(&self, index: u32) -> (u32, u32, u32) {
        match index {
            // Upgrade buttons
            0 => match self {
                BtnState::Disabled => (0x9badb7ff, 0x847e87ff, 0xffffffff),
                BtnState::Normal => (0x1f122bff, 0x1f122bff, 0xffffffff),
                BtnState::Hovered => (0x847e87ff, 0x847e87ff, 0xffffffff),
                BtnState::Pressed => (0x847e87ff, 0xffffffff, 0x9badb7ff),
            },
            // Text or icon buttons
            _ => match self {
                BtnState::Disabled => (0x1f122bff, 0x1f122bff, 0x847e87ff),
                BtnState::Normal => (0x1f122bff, 0xffffffff, 0xffffffff),
                BtnState::Hovered => (0xffffffff, 0x1f122bff, 0x1f122bff),
                BtnState::Pressed => (0xffffffff, 0xffffffff, 0x1f122bff),
            }
        }
    }

}

#[turbo::serialize]
#[derive(PartialEq)]
pub struct Btn {
    pub bounds: Bounds,
    pub state: BtnState,
    pub string: String,
    text: bool,
    pub interactable: bool,
    pub clickable: bool,
    pub colors_index: u32,
    pub fixed: bool,
    font: String,
}

impl Btn {
    pub fn new(string: String, bounds: Bounds, text: bool, colors_index: u32) -> Btn {
        Self {
            bounds,
            state: BtnState::Normal,
            string,
            text,
            interactable: true,
            clickable: true,
            colors_index,
            fixed: true,
            font: "medium".to_string(),
        }
    }

    pub fn buy() -> Btn {
        Self {
            bounds: Bounds::new(0, 0, 0, 0),
            state: BtnState::Normal,
            string: "+".to_string(),
            text: false,
            interactable: false,
            clickable: true,
            colors_index: 1,
            fixed: true,
            font: "medium".to_string(),
        }
    }

    pub fn on_click(&self) -> bool {
        let p = pointer::screen();
        let pp = if self.fixed { p.xy() } else { p.xy() };

        return self.interactable 
            && self.clickable 
            && self.bounds.intersects_xy(pp) 
            && p.just_pressed();
    }

    pub fn update(&mut self) {
        let p = pointer::screen();
        let pp = if self.fixed { p.xy() } else { p.xy() };

        if self.interactable {
            if self.bounds.intersects_xy(p.xy()) {
                self.state = BtnState::Hovered;
                if self.clickable && p.just_pressed() {
                    self.state = BtnState::Pressed;
                } else if self.clickable && p.pressed() {
                    self.state = BtnState::Pressed;
                }
            } else {
                self.state = BtnState::Normal;
            }
        } else {
            self.state = BtnState::Disabled;
        }
    }

    pub fn draw(&self) {
        let colors = self.state.colors(self.colors_index);

        rect!(
            fixed = self.fixed, 
            xy = self.bounds.xy(), 
            wh = self.bounds.wh(), 
            border_radius = 2, 
            border_size = 1,
            color = colors.0,
            border_color = colors.1
        );
        
        if self.text {
            text!(
                &self.string,
                fixed = self.fixed, 
                x = self.bounds.center_x() as f32 - self.string.len() as f32 * 2.5, 
                y = self.bounds.center_y() - 4,
                color = colors.2,
                font = "medium",
            );
        } else {
            sprite!(
                &self.string,
                fixed = self.fixed, 
                xy = self.bounds.xy(),
                color = colors.2
            )
        }
    }
}