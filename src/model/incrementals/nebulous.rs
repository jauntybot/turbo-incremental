use super::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Nebulous {
    center: (f32, f32),
    counter: u8,
    k: i32,
    flow_array: Vec<(f32, f32, u32)>, // (x, y, h, s, b)
    start_col: f32,
    rez1: f32,
    rez2: f32,
    gap: f32,
    len: f32,
    start_vary: f32,
    radius_max: f32,
    radius_min: f32,

    segments: Vec<Segment>,
}

const BASE_COLOR: u32 = 0x4b77bc33;


impl Nebulous {
    pub fn new() -> Self {
        Self {
            center: (944.0, -304.0),
            counter: 0,
            k: 24,
            flow_array: vec![],
            start_col: rand() as f32 % 361.0,
            rez1: 0.006,
            rez2: 0.005,
            gap: 4.0,
            len: 10.0,
            start_vary: 25.0,

            segments: vec![],

            radius_max: 480.0,
            radius_min: 20.0,
        }
    }

    pub fn update(&mut self) {
        
        if self.counter%120 == 0 {
            self.make_array();
            self.make_segments();
        }

        for segment in self.segments.iter_mut() {
            let angle = (segment.start.1 - self.center.1).atan2(segment.start.0 - self.center.0);
            let angle_offset = angle + (perlin_noise(segment.start.0 * self.rez2, segment.start.1 * self.rez2) - 0.5) * std::f32::consts::PI / 2.0;
            let dx = angle_offset.cos() / self.len;
            let dy = angle_offset.sin() / self.len;
            
            segment.start.0 += dx;
            segment.start.1 += dy;
            segment.end.0 += dx;
            segment.end.1 += dy;
        }
        
        self.counter += 1;  
    }

    fn make_array(&mut self) {
        self.flow_array.clear();
        let angle_start = std::f32::consts::FRAC_PI_2;

        for r in (self.radius_min as usize..=self.radius_max as usize).step_by(self.gap as usize) {
            for a in (0..=90).step_by(6) {
                let angle = angle_start + (a as f32 * std::f32::consts::PI / 180.0);
                let radius = r as f32;

                let x = self.center.0 + radius * angle.cos() * 2. + rand() as f32 % self.start_vary - self.start_vary / 2.0;
                let y = self.center.1 + radius * angle.sin() * 2. + rand() as f32 % self.start_vary - self.start_vary / 2.0;

                let color = animated_gradient_color(x, y, tick() as f32, self.center, self.radius_max);
                self.flow_array.push((x, y, color));
            }
        }
    }

    fn make_segments(&mut self) {
        if self.segments.len() > self.flow_array.len() * 5 {
            for i in 0..self.flow_array.len() + 1 {
                self.segments.remove(i);
            }
        }
        for i in 0..self.flow_array.len() {
            let (x, y, color) = self.flow_array[i];
            
            let dx = x - self.center.0;
            let dy = y - self.center.1;
            let base_angle = dy.atan2(dx);
            let ang = base_angle + (perlin_noise(x * self.rez1, y * self.rez1) - 0.2) * 1.7 * std::f32::consts::PI;

            let new_x = ang.cos() * self.len + x;
            let new_y = ang.sin() * self.len + y;

            self.segments.push(Segment {
                start: (x, y),
                end: (new_x, new_y),
                thickness: 8 as f32 + rand() as f32 % 40.,
                direction: (0., 0.),
                color: color as u32,
            });

            self.flow_array[i].0 = new_x;
            self.flow_array[i].1 = new_y;
        }
    }

    pub fn draw(&mut self, tick: usize) {
        for i in 0..self.segments.len()-1 {
            let index = self.segments.len() - 1 - i;
            let segment = &mut self.segments[index];
            segment.color = animated_gradient_color(segment.start.0, segment.start.1, tick as f32, self.center, self.radius_max);
            let s = format!("cloud_{}", segment.thickness);
            // path!(
            //     start = segment.start,
            //     end = segment.end,
            //     size = segment.thickness * 4.,
            //     color = segment.color,
            // );
            circ!(
                xy = segment.start,
                size = segment.thickness,
                color = segment.color,
            );
            // sprite!(
            //     &s,
            //     xy = segment.start,
            //     color = segment.color,
            // );
        }
    }

    fn vary_rgb(hex: u32, range: i8) -> u32 {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
    
        let vary = |channel: u8| -> u8 {
            let val = rand() as i8 % (2 * range + 1) - range;
            channel.saturating_add_signed(val)
        };
    
        let new_r = vary(r);
        let new_g = vary(g);
        let new_b = vary(b);
    
        (255 << 24) | ((new_r as u32) << 16) | ((new_g as u32) << 8) | (new_b as u32)
    }
}

fn animated_gradient_color(x: f32, y: f32, time: f32, center: (f32, f32), radius: f32) -> u32 {
    let pink = u32_to_rgba(0x942630ff);   // RGB for pink
    let purple = u32_to_rgba(0x4b77bcff);   // RGB for purple

    // Use Perlin noise to compute a blend factor in [0.0, 1.0]
    let noise_val = perlin_noise(x * 0.01 + time * 0.02, y * 0.01 + time * 0.02);
    let t = (noise_val * 0.5 + 0.5).clamp(0.0, 1.0); // Map to [0, 1]

    // Calculate the distance from the center
    let dx = x - center.0;
    let dy = y - center.1;
    let distance = (dx * dx + dy * dy).sqrt();

    // Map the distance to the alpha range [120, 0]
    let alpha_start = radius / 2.0; // Start fading halfway
    let alpha = if distance > alpha_start {
        let fade_t = ((distance - alpha_start) / (radius - alpha_start)).clamp(0.0, 1.0);
        (120.0 * (1.0 - fade_t)).round() as u8
    } else {
        120
    };

    // Interpolate RGB values
    let r = (pink.0 as f32 * (1.0 - t) + purple.0 as f32 * t).round() as u8;
    let g = (pink.1 as f32 * (1.0 - t) + purple.1 as f32 * t).round() as u8;
    let b = (pink.2 as f32 * (1.0 - t) + purple.2 as f32 * t).round() as u8;

    rgba_to_u32(r, g, b, alpha)
}

fn rgba_to_u32(r: u8, g: u8, b: u8, a: u8) -> u32 {
    ((r as u32) << 24) | ((g as u32) << 16) | (b as u32) << 8 | (a as u32)
}

fn u32_to_rgba(color: u32) -> (u8, u8, u8, u8) {
    let r = ((color >> 24) & 0xFF) as u8;
    let g = ((color >> 16) & 0xFF) as u8;
    let b = ((color >> 8) & 0xFF) as u8;
    let a = (color & 0xFF) as u8;
    (r, g, b, a)
}