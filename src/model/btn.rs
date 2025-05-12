use super::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub enum BtnState {
    Disabled,
    Normal,
    Hovered,
    Pressed,
}
impl BtnState {
    pub fn colors(&self, index: u32) -> (u32, u32, u32) {
        match index {
            0 => match self {
                BtnState::Disabled => (0x9badb7ff, 0x847e87ff, 0xffffffff),
                BtnState::Normal => (0x222034ff, 0x222034ff, 0xffffffff),
                BtnState::Hovered => (0x847e87ff, 0x847e87ff, 0xffffffff),
                BtnState::Pressed => (0x847e87ff, 0xffffffff, 0x9badb7ff),
            },
            _ => match self {
                BtnState::Disabled => (0x222034ff, 0x222034ff, 0x847e87ff),
                BtnState::Normal => (0x222034ff, 0xffffffff, 0xffffffff),
                BtnState::Hovered => (0xffffffff, 0x222034ff, 0x222034ff),
                BtnState::Pressed => (0xffffffff, 0xffffffff, 0x222034ff),
            }
        }
    }

}

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
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
        let p = pointer();
        let pp = if self.fixed { p.fixed_position() } else { p.relative_position() };

        return self.interactable 
            && self.clickable 
            && self.bounds.intersects_xy(pp) 
            && p.just_pressed();
    }

    pub fn update(&mut self) {
        let p = pointer();
        let pp = if self.fixed { p.fixed_position() } else { p.relative_position() };

        if self.interactable {
            if self.clickable && self.bounds.intersects_xy(pp) && p.pressed(){
                self.state = BtnState::Pressed;
            } else if self.bounds.intersects_xy(pp) {
                self.state = BtnState::Hovered;
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
                x = self.bounds.center_x() - self.string.len() as i32 * 2 - 1, 
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