use super::*;

#[turbo::serialize]
pub struct Slider {
    pub bounds: Bounds,
    pub state: BtnState,
    pub value: f32,
}

impl Slider {
    pub fn new(bounds: Bounds, value: f32) -> Self {
        Self { 
            bounds, 
            state: BtnState::Normal,
            value, 
        }
    }

    pub fn update(&mut self) {
        let p = pointer::screen();

        if self.bounds.intersects_xy(p.xy()) {
            self.state = BtnState::Hovered;
            if p.just_pressed() || p.pressed() {
                self.state = BtnState::Pressed;
                self.value = ((p.xy().0 - self.bounds.x()) as f32 / self.bounds.w() as f32)
                    .clamp(0.0, 1.0);
                self.value = (self.value / 0.05).round() * 0.05;
            }
        } else {
            self.state = BtnState::Normal;
        }
    }

    pub fn draw(&self) {
        let bg_color;
        let line_color;
        let fill_color;
        match self.state {
            BtnState::Disabled => {
                bg_color = 0x847e87ff;
                line_color = 0xffffffff;
                fill_color = 0x1f122bff;
            }
            BtnState::Normal => {
                bg_color = 0x847e87ff;
                line_color = 0xffffffff;
                fill_color = 0x1f122bff;
            }
            BtnState::Hovered => {
                bg_color =  0xffffffff;
                line_color = 0x847e87ff;
                fill_color = 0xffffffff;
            }
            BtnState::Pressed => {
                bg_color = 0x847e87ff;
                line_color = 0xffffffff;
                fill_color = 0x1f122bff;
            }
        }
        // Draw the background
        rect!(
            fixed = true,
            xy = (self.bounds.x(), self.bounds.center_y()),
            wh = (self.bounds.w(), 2),
            color = bg_color,
        );

        rect!(
            fixed = true,
            xy = (self.bounds.x() + (self.value * self.bounds.w() as f32) as i32 - self.bounds.w() as i32 / 16, self.bounds.center_y() - self.bounds.h() as i32/2),
            wh = (self.bounds.w() / 8, self.bounds.h()),
            border_radius = 2,
            color = fill_color,
            border_color = line_color,
            border_size = 1,
        );
    }
}