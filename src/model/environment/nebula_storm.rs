use super::*;

#[turbo::serialize]
pub struct Segment {
    pub start: (f32, f32),
    pub end: (f32, f32),
    pub thickness: f32,
    pub direction: (f32, f32),
    pub color: u32, // RGBA: we will fade alpha
    
}

#[turbo::serialize]
pub struct Bolt {
    pub segments: Vec<Segment>,
    age: f32,
    lifespan: f32,
    dir: f32,
    center: (f32, f32),       // center of arc
    angle_speed: f32,         // radians per second
    draw: bool,
    draw_segments: Vec<Segment>,
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

    pub fn update(&mut self) {
        self.age += self.dir / 60.0;
        self.draw_segments.clear();

        let num_segments = self.segments.len();
        let segment_duration = GROW_TIME / num_segments as f32;
        let shrink_start = self.lifespan - GROW_TIME * 2.0;

        for (i, segment) in self.segments.iter().enumerate() {
            let seg_time_start = i as f32 * segment_duration;
            let seg_time_end = seg_time_start + segment_duration;

            let mut draw = false;
            let mut draw_start = segment.start;
            let mut draw_end = segment.end;

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
            } else if self.age >= GROW_TIME && self.age <= shrink_start {
                draw = true;
            } else if self.age > shrink_start {
                let shrink_age = self.age - shrink_start;
                let t = shrink_age / (GROW_TIME * 2.0);
                let seg_index = (t * num_segments as f32).floor() as usize;

                if i > seg_index {
                    draw = true;
                } else if i == seg_index {
                    let local_t = (t * num_segments as f32) % 1.0;
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

                self.draw_segments.push(Segment {
                    start: draw_start,
                    end: draw_end,
                    thickness: segment.thickness,
                    direction: segment.direction,
                    color: faded_color,
                });
            }
        }
    }

   pub fn draw(&self) {
        for seg in &self.draw_segments {
            path!(
                start = seg.start,
                end = seg.end,
                width = seg.thickness,
                color = seg.color,
            );
            circ!(
                xy = (seg.start.0 - seg.thickness / 2., seg.start.1 - seg.thickness / 2.),
                diameter = seg.thickness / 2.,
                color = seg.color,
            );
        }
    }
}

const GROW_TIME: f32 = 0.1;

#[turbo::serialize]
pub struct NebulaStorm {
    pub bolts: Vec<Bolt>,
    spawn_timer: f32,
    bg: Cloud,
}

impl NebulaStorm {
    pub fn new() -> NebulaStorm {
        let mut bg = Cloud::new((640 + 240 + 64, -240 - 64), 320, 0, 0x83b3b000);
        bg.fade_ranges.push((180., 90.));
        NebulaStorm {
            bolts: vec![],
            spawn_timer: 0.0,
            bg,
        }
    }

    pub fn update(&mut self) {
        self.bg.update();


        let dt = 1.0 / 60.0; // Simulate 60 FPS time step
        self.spawn_timer += dt;
        let center = (640.0 + 240., -120. - 64.);

        // Spawn a new bolt every 0.2 seconds
        if self.spawn_timer >= 0.1 {
            self.spawn_timer = 0.0;
            let mut center = center;
            center.0 += random::f32() * 64.0; // Randomize x position within bounds
            let radius = 50.0 + random::f32() * 200.0;        // Varying arc radius
            let start_angle = 45. + random::f32() * 120.0;             // Anywhere around the circle
            let arc_span = 90.0 + random::f32() * 64.0 - radius/360. * 32.;        // Arc length 15°–30°
            let segments = NebulaStorm::generate_arc_lightning(
                center,
                radius,
                start_angle,
                arc_span,
                15,
                25.0,  // jaggedness
            );

            self.bolts.push(Bolt {
                segments: segments,
                age: 0.0,
                lifespan: 0.8,
                dir: 1.,
                center,
                angle_speed: 0.2,
                draw: false,
                draw_segments: vec![],
            });
        }
        
        // Remove expired bolts
        self.bolts.retain_mut(|bolt| {
            bolt.update();
            bolt.age < bolt.lifespan
        });

    }

    pub fn draw(&mut self) {
        self.bg.draw();
        // Draw all bolts
        for bolt in self.bolts.iter() {
            bolt.draw();        
        }

    }

    pub fn get_drone_pos(&self) -> (f32, f32) {
        let mut center = (640.0 + 240., -120. - 64.);
        center.0 += random::f32() * 64.0; // Randomize x position within bounds
        let radius = 50.0 + random::f32() * 200.0;        // Varying arc radius
        let start_angle = 45. + random::f32() * 120.0;             // Anywhere around the circle
        let arc_span = 90.0 + random::f32() * 64.0 - radius/360. * 32.;       

        // Pick a random position along the arc
        let random_t = random::f32(); // 0.0 to 1.0
        let angle = (start_angle + arc_span * random_t).to_radians();
        
        let x = center.0 + radius * angle.cos();
        let y = center.1 + radius * angle.sin();
        
        (x, y)
    }   

    pub fn generate_drone_lightning (
        &mut self,
        origin: (f32, f32),
        segments: usize,
    ) -> (f32, f32) {
        let mut points = Vec::new();
        let target = ((PLANT_BOX.0 + PLANT_BOX.2) as f32 - 20.0, PLANT_BOX.1 as f32 + 20.0);

        for i in 0..segments {
            let t = i as f32 / segments as f32;
            let x = origin.0 * (1.0 - t) + target.0 * t;
            let y = origin.1 * (1.0 - t) + target.1 * t;

            let mut jitter_x = 0.0;
            let mut jitter_y = 0.0;
            if i < segments - 4 {
                jitter_x = (random::f32() - 0.5) * 25.0;
                jitter_y = (random::f32() - 0.5) * 25.0;
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
                direction: (0.0, 0.0),
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
            draw: false,
            draw_segments: vec![],
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
    
            let jitter_x = (random::f32() - 0.5) * jaggedness;
            let jitter_y = (random::f32() - 0.5) * jaggedness;
    
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
                direction: (0.0, 0.0),
                color,
            });
    
            // Branch with 20% chance
            if random::f32() <= 0.20 {
                let branch_angle = ((random::f32() * 60.0) - 30.0).to_radians(); // ±30° spread
                let branch_length = 30.0 + random::f32() * 20.0;
    
                let dx = end.0 - start.0;
                let dy = end.1 - start.1;
                let angle = dy.atan2(dx) + branch_angle;
    
                let bx = end.0 + branch_length * angle.cos();
                let by = end.1 + branch_length * angle.sin();
    
                segments_vec.push(Segment {
                    start: end,
                    end: (bx, by),
                    thickness: 1.0,
                    direction: (0.0, 0.0),
                    color: 0x99b3ffff,
                });
            }
        }
    
        segments_vec
    }
}
