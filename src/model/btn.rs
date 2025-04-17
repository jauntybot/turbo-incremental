use super::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub enum BtnState {
    Disabled,
    Normal,
    Hovered,
    Pressed,
}
impl BtnState {
    pub fn colors(&self) -> (u32, u32, u32) {
        match self {
            BtnState::Disabled => (0x9badb7ff, 0x847e87ff, 0xffffffff),
            BtnState::Normal => (0x9badb7ff, 0x847e87ff, 0xffffffff),
            BtnState::Hovered => (0x847e87ff, 0x9badb7ff, 0xffffffff),
            BtnState::Pressed => (0x847e87ff, 0xffffffff, 0x9badb7ff),
        }
    }
}

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Btn {
    pub bounds: Bounds,
    state: BtnState,
    string: String,
    text: bool,
}

impl Btn {
    pub fn new(string: String, x: i32, y: i32, w: i32, h: i32, text: bool) -> Btn {
        Self {
            bounds: Bounds::new(x, y, w, h),
            state: BtnState::Normal,
            string,
            text,
        }
    }

    pub fn on_click(&self) -> bool {
        return self.bounds.pressed();
    }

    pub fn update(&mut self) {
        if self.bounds.pressed() {
            self.state = BtnState::Pressed;
        } else if self.bounds.hovered() {
            self.state = BtnState::Hovered;
        } else {
            self.state = BtnState::Normal;
        }
    }

    pub fn draw(&self) {
        let colors = self.state.colors();

        rect!(xy = self.bounds.xy(), wh = self.bounds.wh(), color = colors.1);
        rect!(xy = self.bounds.inset(1).xy(), wh = self.bounds.inset(1).wh(), color = colors.0);

        if self.text {
            text!(&self.string, x = self.bounds.center_x() - self.string.len() as i32 * 2, y = self.bounds.center_y() - 3);
        }
    }
}