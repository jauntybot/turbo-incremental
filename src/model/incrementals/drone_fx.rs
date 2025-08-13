use super::*;

pub trait DroneFx {
    fn update(&mut self, origin: (f32, f32)) -> bool;
    fn draw(&self);
}

#[turbo::serialize]
pub struct Scan {
    pub origin: (f32, f32),
    pub pos: (f32, f32),
    pub radius: f32,
    pub lifetime: f32,
    key: String,
}

impl Scan {
    pub fn new(origin: (f32, f32), pos: (f32, f32)) -> Self {
        Self {
            origin,
            pos,
            radius: 0.0,
            lifetime: 40.,
            key: turbo::random::u32().to_string(),
        }
    }

    pub fn update(&mut self, origin: (f32, f32)) -> bool {
        self.lifetime -= 1.;
        self.origin = origin;
        if self.lifetime <= -20. {
            return false;
        }
        if self.lifetime <= 30. {
            self.radius += 0.5 * self.lifetime / 40.;
        }
        true
    }

    pub fn draw(&self) {
        let mut start = self.origin;
        let mut end = self.pos;
        if self.lifetime >= 35.0 {
            let t = ((40.0 - self.lifetime) / 5.0).clamp(0.0, 1.0); // Clamp t to [0, 1]
            end = (
                self.origin.0 + (self.pos.0 - self.origin.0) * t,
                self.origin.1 + (self.pos.1 - self.origin.1) * t,
            );
        } else if self.lifetime < 30.0 && self.lifetime >= 20.0 {
            let t = ((30.0 - self.lifetime) / 5.0).clamp(0.0, 1.0); // Clamp t to [0, 1]
            start = (
                self.origin.0 + (self.pos.0 - self.origin.0) * t,
                self.origin.1 + (self.pos.1 - self.origin.1) * t,
            );
        }
        if self.lifetime >= 20. {
            // Dashed line logic
            let dash_length = 2.0; // Length of each dash
            let gap_length = 2.0; // Length of each gap
            let total_length = dash_length + gap_length;

            let dx = end.0 - start.0;
            let dy = end.1 - start.1;
            let line_length = (dx * dx + dy * dy).sqrt();

            let num_dashes = (line_length / total_length).ceil() as usize;
            let unit_dx = dx / line_length;
            let unit_dy = dy / line_length;

            let current_start = start;

            for i in 0..num_dashes {
                let dash_start = (
                    current_start.0 + unit_dx * total_length * i as f32,
                    current_start.1 + unit_dy * total_length * i as f32,
                );
                let dash_end = (
                    dash_start.0 + unit_dx * dash_length,
                    dash_start.1 + unit_dy * dash_length,
                );

                // Ensure the dash does not extend beyond the end point
                if (dash_end.0 - start.0).hypot(dash_end.1 - start.1) > line_length {
                    break;
                }

                sprite!(
                    "scan_line",
                    xy = dash_start,
                );
            }
        }
        let anim = animation::get(&self.key);
        anim.use_sprite("scan");
        anim.set_repeat(0);

        // Draw the scan effect
        sprite!(animation_key = &self.key, xy = (self.pos.0 - 8., self.pos.1 - 8.));
    }

}

#[turbo::serialize]
pub struct Debris {
    pub origin: (f32, f32),
    pub pos: (f32, f32),
    pub radius: f32,
    pub lifetime: f32,
    key: String,
}