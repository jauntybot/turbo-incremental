use super::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Cloud {
    pub center: (i32, i32),
    pub radius: u32,
    size_range: (u32, u32),
    rings: Vec<Vec<Circle>>, // Vec of rings, each containing circles
    pub fade_ranges: Vec<(f32, f32)>, 
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
        let size_range = (8, 32); // Larger minimum size for inner rings

        // Determine the number of rings based on the radius and circle size
        let mut current_radius = start; // Start with the largest circle size
        while current_radius <= radius {
            let circumference = 2.0 * std::f32::consts::PI * current_radius as f32;

            // Scale circle size inversely with radius (inner rings have larger circles)
            let circle_size = size_range.1.max(size_range.0 + (radius - current_radius) / 10);

            let num_circles = 1 + (circumference / circle_size as f32).floor() as usize/2;

            // Create circles for this ring
            let mut ring = Vec::new();
            for i in 0..num_circles {
                let angle = (i as f32 / num_circles as f32) * std::f32::consts::TAU + (rand() as f32%100.)/100. * std::f32::consts::TAU;
                let pos = (
                    center.0 as f32 + current_radius as f32 * angle.cos(),
                    center.1 as f32 + current_radius as f32 * angle.sin(),
                );

                // Scale wobble amplitude based on radius (inner rings wobble less)
                let wobble_amplitude = 0.1 + (current_radius as f32 / radius as f32) * 0.4;
                let color = if rand()%2==0 { 0x1a1229ff } else { 0x140b1dff };

                ring.push(Circle {
                    pos,
                    size: circle_size,
                    color,
                    angle,
                    speed: 0.001 + (rand() as f32%100.)/100. * 0.002, // Random wobble speed
                    wobble_phase: (rand() as f32%100.)/100. * std::f32::consts::TAU, // Random wobble phase
                    wobble_amplitude,
                });
            }
            rings.push(ring);

            // Move to the next ring
            current_radius += 1;
        }

        Cloud {
            center,
            radius,
            size_range,
            rings,
            fade_ranges: vec![],
        }
    }

    pub fn update(&mut self) {
        //log!("{}", self.fade_ranges.len());
        for (ring_index, ring) in self.rings.iter_mut().enumerate() {
            for circle in ring.iter_mut() {
                // Wobble the circle along its ring
                let normalized_speed = circle.speed / (self.radius as f32 / ring_index as f32);
                circle.angle += normalized_speed;
                if circle.angle > std::f32::consts::TAU {
                    circle.angle -= std::f32::consts::TAU;
                }

                // Apply wobble using a sine wave
                circle.wobble_phase += 0.1; // Increment wobble phase
                if circle.wobble_phase > std::f32::consts::TAU {
                    circle.wobble_phase -= std::f32::consts::TAU;
                }

                // Inner rings wobble less (scale wobble amplitude by ring index)
                let wobble_offset = circle.wobble_amplitude * circle.wobble_phase.sin();

                // Update the circle's position based on its angle and wobble
                let distance = ((circle.pos.0 - self.center.0 as f32).powi(2)
                    + (circle.pos.1 - self.center.1 as f32).powi(2))
                    .sqrt();
                circle.pos = (
                    self.center.0 as f32 + (distance + wobble_offset) * circle.angle.cos(),
                    self.center.1 as f32 + (distance + wobble_offset) * circle.angle.sin(),
                );
            }
        }
        let clouds: usize = self.rings.iter().map(|belt| belt.len()).sum();
        //log!("{}", clouds);
    }

    pub fn draw(&self) {
        for i in 0..self.rings.len() {
            for circle in self.rings[self.rings.len() - i - 1].iter() {
                // Default alpha
                let mut alpha = 50;

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
                        normalized_angle.to_degrees() >= start_deg || normalized_angle.to_degrees() <= end_deg
                    };

                    if in_range {
                        // Smooth fade using cosine interpolation
                        let fade_range = 550.0; // Half of the fade range (15 degrees on each side)
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
                        alpha = (fade_factor * (50.0)) as u8;
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
            }
        }
    }
}
