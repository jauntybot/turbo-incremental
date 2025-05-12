use super::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Segment {
    start: (f32, f32),
    pub end: (f32, f32),
    thickness: f32,
    color: u32, // RGBA: we will fade alpha
}

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Bolt {
    pub segments: Vec<Segment>,
    age: f32,
    lifespan: f32,
    dir: f32,
    center: (f32, f32),       // center of arc
    angle_speed: f32,         // radians per second
}

impl Bolt {
    fn center(&self) -> (f32, f32) {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let n = self.segments.len() * 2;
    
        for segment in &self.segments {
            sum_x += segment.start.0 + segment.end.0;
            sum_y += segment.start.1 + segment.end.1;
        }
    
        (sum_x / n as f32, sum_y / n as f32)
    }

    fn update(&mut self) {
        self.age += self.dir / 60.0;
        let num_segments = self.segments.len();
        let segment_duration = GROW_TIME / num_segments as f32;
        let shrink_start = self.lifespan - GROW_TIME * 2.0;
        
        for (i, segment) in self.segments.iter().enumerate() {
            let seg_time_start = i as f32 * segment_duration;
            let seg_time_end = seg_time_start + segment_duration;
        
            let mut draw = false;
            let mut draw_start = segment.start;
            let mut draw_end = segment.end;
        
            // GROW PHASE
            if self.age < GROW_TIME {
                if self.age >= seg_time_start && self.age < seg_time_end {
                    let t = (self.age - seg_time_start) / segment_duration;
                    draw = true;
                    draw_end = (
                        segment.start.0 + (segment.end.0 - segment.start.0) * t,
                        segment.start.1 + (segment.end.1 - segment.start.1) * t,
                    );
                } else if self.age >= seg_time_end {
                    draw = true;
                }
            }
            // FULL PHASE
            else if self.age >= GROW_TIME && self.age <= shrink_start {
                draw = true;
            }
            // SHRINK PHASE (reverse draw order)
            else if self.age > shrink_start {
                let shrink_age = self.age - shrink_start;
                let t = shrink_age / (GROW_TIME * 2.0);
                //log!("{}, {}, {}", shrink_age, GROW_TIME * 4.0, t);
                let seg_index = (t * num_segments as f32).floor() as usize; // Reverse the shrinking direction
            
                if i > seg_index {
                    draw = true;
                } else if i == seg_index {
                    let local_t = (t * num_segments as f32) % 1.0; // Interpolate from start to end
                    draw = true;
                    draw_start = (
                        segment.start.0 + (segment.end.0 - segment.start.0) * local_t,
                        segment.start.1 + (segment.end.1 - segment.start.1) * local_t,
                    );
                }
            }
        
            if draw {
                let alpha = 1.0 - (self.age / self.lifespan);
                let a = (segment.color >> 24) & 0xff;
                let r = (segment.color >> 16) & 0xff;
                let g = (segment.color >> 8) & 0xff;
                let b = segment.color & 0xff;
        
                let faded_color =
                    ((a as f32 * alpha) as u32) << 24 |
                    (r << 16) |
                    (g << 8) |
                    (b);
        
                path!(
                    start = draw_start,
                    end = draw_end,
                    width = segment.thickness,
                    color = faded_color,
                );
                circ!(
                    xy = (draw_start.0 - segment.thickness / 2., draw_start.1 - segment.thickness / 2.),
                    diameter = segment.thickness/2.,
                    color = faded_color,
                );
            }
        }
    }
}

const GROW_TIME: f32 = 0.1;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct NebulaStorm {
    pub bolts: Vec<Bolt>,
    spawn_timer: f32,
    field: FlowFieldArt,
}

impl NebulaStorm {
    pub fn new() -> NebulaStorm {
        NebulaStorm {
            bolts: vec![],
            spawn_timer: 0.0,
            field: FlowFieldArt::new(),
        }
    }

    pub fn update(&mut self) {
        let dt = 1.0 / 60.0; // Simulate 60 FPS time step
        self.spawn_timer += dt;
        let center = (640.0 + 240. + 64., -240. - 64.);

        // Spawn a new bolt every 0.2 seconds
        if self.spawn_timer >= 0.1 {
            self.spawn_timer = 0.0;
            let mut center = center;
            center.0 += rand() as f32 % 64.0; // Randomize x position within bounds
            let radius = 120.0 + (rand() as f32 % 241.0);        // Varying arc radius
            let start_angle = 64. + rand() as f32 % 65.0;             // Anywhere around the circle
            let arc_span = 90.0 + (rand() as f32 % 32.0) - radius/360. * 32.;        // Arc length 15°–30°
            let segments = NebulaStorm::generate_arc_lightning(
                center,
                radius,
                start_angle,
                arc_span,
                15,
                25.0,  // jaggedness
            );

            self.bolts.push(Bolt {
                segments,
                age: 0.0,
                lifespan: 0.8,
                dir: 1.,
                center,
                angle_speed: 0.2,
            });
        }

        // self.field.update();
        // self.field.draw(tick());

        // Draw all bolts
        for bolt in self.bolts.iter_mut() {
            bolt.update();        
        }
        // Remove expired bolts
        self.bolts.retain(|bolt| bolt.age < bolt.lifespan);

    }

    pub fn get_drone_pos(&self) -> (f32, f32) {
        if self.bolts.is_empty() {
            return (640.0 + 320., 480.0 / 2.0);
        }
        let center = self.bolts[0].center();
        (center.0, center.1)
    }   

    pub fn generate_drone_lightning (
        &mut self,
        origin: (f32, f32),
        segments: usize,
    ) -> (f32, f32) {
        let mut points = Vec::new();
        let target = ((PLANT_BOX.0 + PLANT_BOX.2) as f32 - 4.0, PLANT_BOX.1 as f32 + 4.0);

        for i in 0..segments {
            let t = i as f32 / segments as f32;
            let x = origin.0 * (1.0 - t) + target.0 * t;
            let y = origin.1 * (1.0 - t) + target.1 * t;

            let mut jitter_x = 0.0;
            let mut jitter_y = 0.0;
            if i < segments - 2 {
                jitter_x = (rand() as f32 % 101. / 100. - 0.5) * 25.0;
                jitter_y = (rand() as f32 % 101. / 100. - 0.5) * 25.0;
            }

            points.push((x + jitter_x, y + jitter_y));
        }
        let mid = points.len() / 2;
        let mut segments_vec = vec![];
        for i in 0..points.len() - 1 {
            let dist_from_mid = (i as isize - mid as isize).abs() as f32;
            let thickness = 8.0 - dist_from_mid * 1.5;
            let color = 0xffffffff;
            segments_vec.push(Segment {
                start: points[i],
                end: points[i + 1],
                thickness: thickness.max(1.0),
                color,
            });
        }
        let end = segments_vec[segments_vec.len() - 1].end;
        self.bolts.push(Bolt {
            segments: segments_vec,
            age: 0.0,
            lifespan: 0.8,
            dir: 1.,
            center: origin,
            angle_speed: 0.2,
        });
        end
    }

    fn generate_arc_lightning(
        center: (f32, f32),
        radius: f32,
        start_angle_deg: f32,
        arc_span_deg: f32,
        segments: usize,
        jaggedness: f32,
    ) -> Vec<Segment> {
        let mut points = Vec::new();
    
        for i in 0..=segments {
            let t = i as f32 / segments as f32;
            let angle = (start_angle_deg + arc_span_deg * t).to_radians();
    
            let x = center.0 + radius * angle.cos();
            let y = center.1 + radius * angle.sin();
    
            let jitter_x = ((rand() as f32 % 101. / 100. - 0.5) * jaggedness);
            let jitter_y = ((rand() as f32 % 101. / 100. - 0.5) * jaggedness);
    
            points.push((x + jitter_x, y + jitter_y));
        }
    
        let mid = points.len() / 2;
        let mut segments_vec = vec![];
    
        for i in 0..points.len() - 1 {
            let dist_from_mid = (i as isize - mid as isize).abs() as f32;
            let thickness = 6.0 - dist_from_mid * 1.0;
            let color = 0xb3d3dfff;
    
            let start = points[i];
            let end = points[i + 1];
    
            segments_vec.push(Segment {
                start,
                end,
                thickness: thickness.max(1.0),
                color,
            });
    
            // Branch with 20% chance
            if rand() as f32 % 100. < 20.0 {
                let branch_angle = ((rand() as f32 % 60.0) - 30.0).to_radians(); // ±30° spread
                let branch_length = 30.0 + rand() as f32 % 20.0;
    
                let dx = end.0 - start.0;
                let dy = end.1 - start.1;
                let angle = dy.atan2(dx) + branch_angle;
    
                let bx = end.0 + branch_length * angle.cos();
                let by = end.1 + branch_length * angle.sin();
    
                segments_vec.push(Segment {
                    start: end,
                    end: (bx, by),
                    thickness: 1.0,
                    color: 0x99b3ffff,
                });
            }
        }
    
        segments_vec
    }
}


#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct FlowFieldArt {
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

impl FlowFieldArt {
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