use super::*;

pub struct ProgressBar {}
impl ProgressBar {
    pub fn draw(bar: Bounds, prog: f32) {
        rect!(
            fixed = true, 
            xy = bar.xy(), 
            wh = bar.wh(), 
            border_radius = 4,
            border_size = 1,
            color = 0x1f122bff,
            border_color = 0xffffffff,
        );
        rect!(
            fixed = true, 
            xy = (bar.x() + 1, bar.y() + 1), 
            wh = (bar.w() as f32 * prog - 2., bar.h() - 2), 
            border_radius = 4,
            border_size = 2,
            color = 0xffffffff,
            border_color = 0x1f122bff,
        );
        
        // Draw diagonal dashes that scroll
        if bar.w() > 10 {
            let (x, y) = (bar.x() + 5, bar.y() + 4);
            let (w, h) = (bar.w() as i32 - 10, bar.h() as i32 - 8);
            let t = turbo::time::tick() as i32 / 4;
            
            for offset in (x-h..x+w).step_by((h as f32 / 1.5) as usize) {
                let scroll = t % (h/2) as i32;
                let mut x0 = offset + scroll;
                let mut y0 = y + h;
                let mut x1 = offset + h + scroll;
                let mut y1 = y;
    
                if x0 < x {
                    let diff = x - x0;
                    x0 = x;
                    y0 -= diff;
                    if y0 >= y + h {
                        y0 = y + h;
                    }
                }
                if x1 > x + w  {
                    let diff = i32::abs(x + w - x1 );
                    x1 = x + w;
                    y1 += diff;
                    if y1 >= y + h {
                        y1 = y + h;
                    }
                }
                
                path!(
                    fixed = true,
                    start = (x0.max(x), y0.min(y+h)), end = (x1.min(x+w), y1.max(y)),
                    size = 2,
                    rounded = true,
                    color = 0x1f122bff,
                );
    
            }
            rect!(
                fixed = true, 
                xy = (bar.x() + 3, bar.y() + 3), 
                wh = (bar.w() as f32 * prog - 6., bar.h() - 6), 
                border_radius = 4,
                border_size = 2,
                color = 0x1f122b00,
                border_color = 0xffffffff,
            );
        }
    }
}