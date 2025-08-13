use super::*;

#[turbo::serialize]
#[derive(PartialEq)]
pub struct Toggle {
    pub bounds: Bounds,
    pub state: BtnState,
    pub value: bool,
    pub interactable: bool,
    pub clickable: bool,
}
impl Toggle {
    pub fn new(bounds: Bounds, value: bool) -> Self {
        Self {
            bounds,
            state: BtnState::Normal,
            value,
            interactable: true,
            clickable: true,
        }
    }

    pub fn on_click(&self) -> bool {
        let p = pointer::screen();
        let pp = p.xy();

        return self.interactable 
            && self.clickable 
            && self.bounds.intersects_xy(pp) 
            && p.just_pressed();
    }

    pub fn update(&mut self) {
        let p = pointer::screen();

        if self.interactable {
            if self.bounds.intersects_xy(p.xy()) {
                self.state = BtnState::Hovered;
                if self.clickable && p.just_pressed() {
                    self.state = BtnState::Pressed;
                    self.value = !self.value;
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
        let bg_color;
        let line_color;

        match self.state {
            BtnState::Disabled => {
                bg_color = if self.value { 0x847e87ff } else { 0x1f122bff };
                line_color = 0x847e87ff;
            }
            BtnState::Normal => {
                bg_color = if self.value { 0x847e87ff } else { 0x1f122bff };
                line_color = 0xffffffff;
            }
            BtnState::Hovered => {
                bg_color =  0xffffffff;
                line_color = 0x1f122bff;
            }
            BtnState::Pressed => {
                bg_color = if self.value { 0x847e87ff } else { 0x1f122bff };
                line_color = 0xffffffff;
            }
        }
        rect!(
            fixed = true,
            xy = (self.bounds.x() + self.bounds.h() as i32 / 2, self.bounds.y()),
            wh = (self.bounds.w() - self.bounds.h() + 2, self.bounds.h()),
            color = line_color,
        );
        circ!(
            fixed = true,
            xy = self.bounds.xy(),
            size = self.bounds.h(),
            border_size = 1,
            border_color = line_color,
            color = bg_color,
        );
        circ!(
            fixed = true,
            xy = (self.bounds.x() + self.bounds.w() as i32 - self.bounds.h() as i32, self.bounds.y()),
            size = self.bounds.h(),
            border_size = 1,
            border_color = line_color,
            color = bg_color,
        );

        rect!(
            fixed = true,
            xy = (self.bounds.x() + self.bounds.h() as i32 / 2 - 1, self.bounds.y() + 1),
            wh = (self.bounds.w() - self.bounds.h() + 2, self.bounds.h() - 2),
            color = bg_color,
        );

        let offset = if self.value { self.bounds.w() as i32 - self.bounds.h() as i32 } else { 0 };
        circ!(
            fixed = true,
            xy = (self.bounds.x() + offset + 1, self.bounds.y() + 1),
            size = self.bounds.h() - 2,
            border_size = 1,
            color = line_color,
        );

    }
}