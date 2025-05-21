use super::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct WrapBox {
    pub bounds: Bounds,
    pub lines: Vec<String>,
    pub colors_index: u32,
    pub fixed: bool,
}

impl WrapBox {
    pub fn new(text: String, colors_index: u32) -> WrapBox {
        let max_line_length = 20;
        let lines = WrapBox::split_text(text, max_line_length);
        let bounds = Bounds::new(-320, -320, 112, lines.len() * 10 + 6);
        Self {
            bounds,
            lines,
            colors_index,
            fixed: false,
        }
    }

    pub fn split_text(text: String, max_line_length: usize) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current_line = String::new();
        for word in text.split_whitespace() {
            if current_line.len() + word.len() + 1 > max_line_length {
                // Push the current line and start a new one
                lines.push(current_line.trim_start().to_string());
                current_line = String::new();
            }
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }
        // Push the last line if it exists
        if !current_line.is_empty() {
            lines.push(current_line.trim_start().to_string());
        }
        lines
    }

    pub fn update(&mut self, bounds: Bounds, x_offset: i32) {
        self.bounds = self.bounds.position(
            if bounds.x() >= 320 { bounds.left() - self.bounds.w() as i32 - x_offset } else { bounds.right() + x_offset },
            bounds.center_y() - self.bounds.h() as i32 / 2,
        );
        if self.bounds.y() + self.bounds.h() as i32 >= 400 {
            self.bounds = self.bounds.position(
                self.bounds.x(), 
                400 - self.bounds.h()
            );
        }
    }

    pub fn draw(&self) {
        rect!(fixed = true, xy = self.bounds.xy(), wh = self.bounds.wh(), border_size = 1, border_radius = 4, color = 0x1f122bff, border_color = 0xffffffff);
        for i in 0..self.lines.len() {
            text!("{}", self.lines[i]; fixed = true, xy = (self.bounds.x() + 4, self.bounds.y() + 4 + i as i32 * 10), color = 0xffffffff);
        }
    }
}