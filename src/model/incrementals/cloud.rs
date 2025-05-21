use super::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Cloud {
    pub center: (i32, i32),
    pub radius: u32,
    size_range: (u32, u32),
    rings: Vec<Vec<Circle>>, // Vec of rings, each containing circles
    pub fade_ranges: Vec<(f32, f32)>, 
    pub fade: bool,
}

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Circle {
    pub pos: (f32, f32),
    pub size: u32,
    color: u32,
    pub angle: f32, // Current angle along the ring
    pub speed: f32, // Wobble speed
    pub wobble_phase: f32, // Phase of the sine wave for wobble
    pub wobble_amplitude: f32, // Amplitude of the wobble
}

impl Cloud {
    pub fn new(center: (i32, i32), radius: u32, start: u32) -> Self {
        let mut rings = Vec::new();
        let size_range = (16, 72); // Larger minimum size for inner rings

        // Determine the number of rings based on the radius and circle size
        let mut current_radius = start; // Start with the largest circle size
        while current_radius <= radius {
            let circumference = 2.0 * std::f32::consts::PI * current_radius as f32;

            // Scale circle size inversely with radius (inner rings have larger circles)
            let circle_size = 32;

            let num_circles = 1 + (circumference / circle_size as f32).floor() as usize;

            // Create circles for this ring
            let mut ring = Vec::new();
            for i in 0..num_circles {
                let angle = (i as f32 / num_circles as f32) * std::f32::consts::TAU;
                let pos = (
                    center.0 as f32 + current_radius as f32 * angle.cos(),
                    center.1 as f32 + current_radius as f32 * angle.sin(),
                );

                // Scale wobble amplitude based on radius (inner rings wobble less)
                let wobble_amplitude = 0.1 + (current_radius as f32 / radius as f32) * 0.04;
                let color = if rand()%2==0 { 0x1a1229ff } else { 0x1f122bff };

                ring.push(Circle {
                    pos,
                    size: size_range.0 + (size_range.1 as f32 * ((rand()as f32%100.)/100.)) as u32,
                    color,
                    angle,
                    speed: 0.1 + (rand() as f32%100.)/100. * 0.04, // Random wobble speed
                    wobble_phase: (rand() as f32%100.)/100. * std::f32::consts::TAU, // Random wobble phase
                    wobble_amplitude,
                });
            }
            rings.push(ring);

            // Move to the next ring
            current_radius += size_range.0;
        }

        Cloud {
            center,
            radius,
            size_range,
            rings,
            fade_ranges: vec![],
            fade: false,
        }
    }

    pub fn update(&mut self) {
        for (ring_index, ring) in self.rings.iter_mut().enumerate() {
            for (circle_index, circle) in ring.iter_mut().enumerate() {
                // Constant linear speed for all rings
                let ring_radius = ((circle.pos.0 - self.center.0 as f32).powi(2)
                    + (circle.pos.1 - self.center.1 as f32).powi(2))
                    .sqrt();
                let normalized_speed = circle.speed / ring_radius.max(1.0);
                circle.angle += normalized_speed;
                if circle.angle > std::f32::consts::TAU {
                    circle.angle -= std::f32::consts::TAU;
                }

                let base_x = self.center.0 as f32 + ring_radius * circle.angle.cos();
                let base_y = self.center.1 as f32 + ring_radius * circle.angle.sin();

                // Compute the wobble/orbit offset around this base position
                let wobble_radius = circle.wobble_amplitude; // or 2.0
                let speed = 0.02; // adjust this for speed only
                let phase = tick() as f32 * speed + circle_index as f32 * 0.4 + ring_index as f32 * 0.8 + circle.angle * 2.0;
                circle.pos = (
                    base_x + wobble_radius * phase.cos(),
                    base_y + wobble_radius * phase.sin(),
                );
                //text!("{}", phase; x = circle.pos.0, y = circle.pos.1 + 10.);
            }
        }
    }

    pub fn draw(&self) {
        for i in 0..self.rings.len() {
            for circle in self.rings[self.rings.len() - i - 1].iter() {
                // Default alpha
                let mut alpha = 100;

                // Check if the circle's angle is within any fade range
                for &(start, end) in &self.fade_ranges {
                    let angle_deg = circle.angle.to_degrees();
                    let start_deg = start.to_degrees();
                    let end_deg = end.to_degrees();

                    // Normalize the angle to the range [0, 360)
                    let normalized_angle = if angle_deg < 0.0 {
                        angle_deg + 360.0
                    } else {
                        angle_deg
                    };

                    // Handle circular wrapping of fade ranges
                    let in_range = if start_deg <= end_deg {
                        // Normal case: start and end are in order
                        normalized_angle.to_degrees() >= start_deg && normalized_angle.to_degrees() <= end_deg
                    } else {
                        // Wrapped case: end is before start (e.g., 350° to 10°)
                        //normalized_angle.to_degrees() >= start_deg && normalized_angle.to_degrees() <= end_deg
                        normalized_angle.to_degrees() >= start_deg || normalized_angle.to_degrees() <= end_deg
                    };

                    if in_range {
                        // Smooth fade using cosine interpolation
                        let fade_range = 30.0; // Half of the fade range (15 degrees on each side)
                        let distance_to_start = if normalized_angle.to_degrees() >= start_deg {
                            (normalized_angle.to_degrees() - start_deg).abs()
                        } else {
                            (normalized_angle.to_degrees() + 360.0 - start_deg).abs()
                        };
                        let distance_to_end = if normalized_angle.to_degrees() <= end_deg {
                            (end_deg - normalized_angle.to_degrees()).abs()
                        } else {
                            (end_deg + 360.0 - normalized_angle.to_degrees()).abs()
                        };

                        // Calculate fade factor based on the closest boundary
                        let fade_factor_start = (1.0 - (distance_to_start / fade_range).min(1.0)).powi(2); // Quadratic easing
                        let fade_factor_end = (1.0 - (distance_to_end / fade_range).min(1.0)).powi(2);   // Quadratic easing
                        let fade_factor = fade_factor_start.max(fade_factor_end);

                        // Interpolate alpha between 50 and 255
                        alpha = (fade_factor * (100.0)) as u8;
                        break;
                    }
                }

                // Combine the alpha with the circle's color
                let color = 0x000000 | alpha as u32;

                circ!(
                    xy = circle.pos,
                    size = circle.size,
                    color = color,
                );
                //text!("{}", alpha; xy = circle.pos);
            }
        }
    }
}
