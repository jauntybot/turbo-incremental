use std::f32::MIN;

use super::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub enum DroneMode {
    Survey,
    Mining,
    Shipping,
    Conduit,
}

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Drone {
    pub pos: (f32, f32),
    pub target_pos: (f32, f32),
    pub front: bool,
    pub interval: f32,
    pub mode: DroneMode,
    timer: f32,
    phase: f32,
    angle: f32,

    pub level: u32,
    pub speed: u32,

    scan: Option<Scan>,
    asteroid_id: u32, 
    pub cargo: Vec<(Resources, u64)>,
    pub on_site: bool,

    wander_progress: f32,
    wander_forward: bool,
}

impl Drone {
    pub fn new(mode: DroneMode, level: u32, speed: u32, target_pos: (i32, i32)) -> Self {
        Drone {
            pos: ((DEPOT_BOX.0 + DEPOT_BOX.2/2) as f32, (DEPOT_BOX.1 + DEPOT_BOX.3/2) as f32), // Position of drone depot
            target_pos: (target_pos.0 as f32, target_pos.1 as f32),
            front: true,
            interval: match mode {
                DroneMode::Survey => 800.,
                DroneMode::Mining => 500.,
                DroneMode::Shipping => 300.,
                DroneMode::Conduit => 400.,
            },
            mode,
            timer: 0.,
            phase: (rand() as f32 % 101.) / 100., 
            angle: 0.,

            level,
            speed,

            scan: None,
            asteroid_id: 0,
            cargo: vec![],
            on_site: false,

            wander_progress: 0.,
            wander_forward: false,
        }
    }

    pub fn conduit(&mut self, nebula: &mut NebulaStorm) -> bool {
        let bounds = (640.0, 208., 64., 64.0);

        if self.on_site {
            self.wander( 200.0);
            self.timer += 1.;
            if self.timer >= self.interval {
                self.timer = 0.;
                self.target_pos = nebula.get_drone_pos();
                nebula.generate_drone_lightning(self.pos, 15);
                return true;
            }
        } else {
            self.target_pos = ((PLANT_BOX.0 + PLANT_BOX.2/2) as f32, (PLANT_BOX.1 + PLANT_BOX.3/2) as f32);
            self.on_site = self.follow(0.1); 
        }
        false
    }

    pub fn wander(&mut self, min_duration: f32) -> bool {
        // Arc parameters
        let center = (640.0 + 240. + 64., -240. - 64.);
        let base_radius = 240. + self.phase * 120.;
        let arc_start = 0.35; // radians, adjust as needed
        let arc_end = 4.0;   // radians, adjust as needed
        let speed = 0.008;   // adjust for desired speed

        // Initialize direction if not present
        if self.wander_progress.is_nan() {
            self.wander_progress = 0.0;
            self.wander_forward = true;
        }

        // Update progress along the arc
        if self.wander_forward {
            self.wander_progress += speed;
            if self.wander_progress >= 1.0 {
                self.wander_progress = 1.0;
                self.wander_forward = false;
            }
        } else {
            self.wander_progress -= speed;
            if self.wander_progress <= 0.0 {
                self.wander_progress = 0.0;
                self.wander_forward = true;
            }
        }

        // Interpolate angle along the arc
        let angle = arc_start + (arc_end - arc_start) * self.wander_progress;
        self.target_pos = (
            center.0 + base_radius * angle.cos(),
            center.1 + base_radius * angle.sin(),
        );

        self.follow(0.1);
        false
    }

    pub fn survey(&mut self, station: &Station) -> bool {
        // Calculate the angle based on the timer and interval
        let angle = ((self.timer as f32 / station.drone_speed as f32) + self.phase) * std::f32::consts::TAU; // TAU = 2 * PI

        // Define the ellipse dimensions
        let center = ((PLANET_BOX.0 + PLANET_BOX.2/2) as f32, (PLANET_BOX.1 + PLANET_BOX.3/2) as f32); // Center of the ellipse
        let radius_x = 100.0; // Horizontal radius
        let radius_y = 25.0;  // Vertical radius

        // Oscillation factor (sinusoidal oscillation between 0.75 and 1.0)
        let raw_oscillation = ((self.timer as f32 / station.drone_speed as f32) * std::f32::consts::TAU).sin();
        let oscillation = 0.25 + 0.75 * (0.5 + 0.5 * raw_oscillation); // Oscillates between 0.75 and 1.0

        if self.on_site {
            // Calculate the position on the ellipse with oscillation
            self.pos.0 = center.0 + radius_x * angle.cos();
            self.pos.1 = center.1 + radius_y * oscillation * angle.sin();
    
            self.front = self.pos.1 >= center.1;
    
            self.timer += 1.;
            let delta = (self.timer as f32 - station.drone_speed as f32).abs() % (station.drone_speed as f32/2.);
            if delta <= 1.0 && self.scan.is_none() {
                let scan = (center.0 + 32. * angle.cos(), center.1 + 32. * oscillation * angle.sin()); 
                self.scan = Some(Scan::new(self.pos, scan));
                return true;
            }
            if self.timer >= station.drone_speed {
                self.timer = 0.;
            }
    
            if let Some(scan) = &mut self.scan {
                if !scan.update(self.pos) {
                    self.scan = None;
                }
            } 
            false
        } else {
            self.timer = station.drone_speed/2.-1.;
            self.target_pos = (center.0 + radius_x * angle.cos(),
                center.1 + radius_y * oscillation * angle.sin());
            if self.follow(0.1) {
                self.on_site = true;
            }
            false
        }
    }

    pub fn shipping(&mut self) -> Option<(Resources, u64)> {
        // Define the start and bounds for the random target
        let home = ((DEPOT_BOX.0 + DEPOT_BOX.2/2) as f32, (DEPOT_BOX.1 + DEPOT_BOX.3 - 8) as f32);
        let mines = ((MINES_BOX.0 + MINES_BOX.2/2) as f32 -6. - (self.phase * 2.).round() * 8., (MINES_BOX.1 + 2*MINES_BOX.3/3) as f32);
        
        if self.on_site {
            self.timer += 1. * (1. + self.speed as f32 * 0.2);
            let angle = (self.timer / self.interval) * std::f32::consts::TAU; // TAU = 2 * PI
            
            self.target_pos = (
                home.0 + (8. + self.phase * 16.) * angle.sin(),
                home.1 + (8. + self.phase * 16.) * angle.cos(),
            );
            self.follow(0.1);
            // if self.phase % 0.02 == 0.0 {
            //     self.target_pos = (
            //         home.0 + (16. + self.phase * 16.) * angle.sin(),
            //         home.1 + (16. + self.phase * 16.) * angle.cos(),
            //     );
            // } else {
            // }
//            log!("{}", (16. + self.phase * 16.) * angle.sin());
            if self.timer >= self.interval {
                self.timer = 0.;
                let amount = ((1.0 + self.speed as f32 * 0.2) * 10. + (self.level as f32 * 0.75 * 5.)).round() as u64;
                if amount >= self.cargo[0].1 {
                    self.cargo.clear();
                    self.target_pos = mines;
                    self.on_site = false;
                } else {
                    self.cargo[0].1 -= amount;
                }
                return Some((Resources::Metals, amount));
            }
        } else {
            if self.cargo.is_empty() {
                self.target_pos = mines;
            } else {
                self.target_pos = home;
            }
            if self.follow(0.2) {
                if !self.cargo.is_empty() {
                    self.on_site = true;
                } else {
                    self.timer += 1.;
                    if self.timer >= self.interval {
                        self.timer = 0.;
                        let amount = ((1. + self.level as f32 * 0.75) * 32.) as u64;
                        self.cargo.push((Resources::Metals, amount));
                        self.target_pos = home; // Reset target to home after mining
                        return Some((Resources::Metals, amount));
                    }
                }
            }
        }
        None
    }

    pub fn update_mining(&mut self, field: &mut AsteroidField) -> bool {
        if self.on_site {
            self.timer += 1.0 * (1.0 + self.speed as f32 * 0.5);
            if self.timer >= self.interval / 4. {
                self.timer = 0.;
                self.cargo.clear();
                if let Some(asteroid) = {
                    let matching_asteroids: Vec<_> = field
                        .asteroids[0]
                        .iter()
                        .filter(|a| a.angle < 2.3 && a.radius < 2040.0)
                        .collect();
            
                    if !matching_asteroids.is_empty() {
                        let random_index = (rand() as usize) % matching_asteroids.len();
                        Some(matching_asteroids[random_index])
                    } else {
                        None
                    }
                } {
                    self.asteroid_id = asteroid.id;
                    self.target_pos = asteroid.pos;
                }
                self.on_site = false;
                return true;
            }
        } else {
            let done = self.follow(0.15);
            if self.cargo.is_empty() && done {
                if let Some(asteroid) = field.asteroids[0].iter_mut().find(|a| a.id == self.asteroid_id) {
                    // Active mining
                    self.timer += 1. * (1.0 + self.speed as f32 * 0.15);
                    asteroid.drilling = true; // Start drilling animation
                    self.target_pos = asteroid.pos;
                    if self.timer >= self.interval {
                        self.timer = 0.;
                        self.cargo.push((Resources::Metals, 0));
                        self.target_pos = (15.0 + (MINES_BOX.0 + rand() as i32 % 33) as f32, 0.0); // Reset target to home after mining
                        asteroid.drilling = false; // Stop drilling animation
                        
                    }
                } else if let Some(asteroid) = {
                    let matching_asteroids: Vec<_> = field
                        .asteroids[0]
                        .iter()
                        .filter(|a| a.angle < 2.3 && a.radius < 2040.0)
                        .collect();
            
                    if !matching_asteroids.is_empty() {
                        let random_index = (rand() as usize) % matching_asteroids.len();
                        Some(matching_asteroids[random_index])
                    } else {
                        None
                    }
                } {
                    self.asteroid_id = asteroid.id;
                    self.target_pos = asteroid.pos;
                }
            } else if self.cargo.is_empty() && !done {
                if let Some(asteroid) = field.asteroids[0].iter_mut().find(|a| a.id == self.asteroid_id) {
                    self.target_pos = asteroid.pos;
                } else if let Some(asteroid) = {
                    let matching_asteroids: Vec<_> = field
                    .asteroids[0]
                    .iter()
                    .filter(|a| a.angle < 2.3 && a.radius < 2040.0)
                        .collect();
                    
                    if !matching_asteroids.is_empty() {
                        let random_index = (rand() as usize) % matching_asteroids.len();
                        Some(matching_asteroids[random_index])
                    } else {
                        None
                    }
                } {
                    self.asteroid_id = asteroid.id;
                    self.target_pos = asteroid.pos;
                }
            } else if done {
                self.on_site = true;
            }
        }
        false
    }

    pub fn follow(&mut self, speed_mult: f32) -> bool {
        // Calculate the direction vector
        let direction = (
            self.target_pos.0 - self.pos.0,
            self.target_pos.1 - self.pos.1,
        );

        // Calculate the magnitude of the direction vector
        let magnitude = (direction.0.powi(2) + direction.1.powi(2)).sqrt();

        // Normalize the direction vector
        let normalized = if magnitude != 0.0 {
            (direction.0 / magnitude, direction.1 / magnitude)
        } else {
            (0.0, 0.0) // If magnitude is zero, no movement
        };

        // Update the drone's position based on its speed
        self.pos.0 += normalized.0 * (1.0 + 0.1 * self.speed as f32 * speed_mult);
        self.pos.1 += normalized.1 * (1.0 + 0.1 * self.speed as f32 * speed_mult);

        // Check if the drone has reached the target position
        let distance_to_target = (
            self.target_pos.0 - self.pos.0,
            self.target_pos.1 - self.pos.1,
        );
        if distance_to_target.0.abs() < 1.0 * (1.0 + 0.1 * self.speed as f32 * speed_mult) && distance_to_target.1.abs() < 1.0 * (1.0 + 0.1 * self.speed as f32){
            self.pos = self.target_pos; // Snap to the target position
            return true; // Indicate that the drone has reached the target
        }

        false
    }


    pub fn draw(&self) {
        rect!(
            xy = (self.pos.0-1., self.pos.1+(tick()as f32/2.%10.)*0.5), 
            wh = (2, 1),
            color = 0xffc247ff, 
        );
        sprite!("drone", xy = (self.pos.0 - 2., self.pos.1 -2.), wh = (4, 4));

    }

    pub fn draw_scan(&self) {
        if let Some(scan) = &self.scan {
            scan.draw();
        }
    }
}