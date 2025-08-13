use super::*;

#[turbo::serialize]
#[derive(PartialEq)]
pub enum DroneMode {
    Survey,
    Mining,
    Shipping,
    Conduit,
    Research,
}

#[turbo::serialize]
pub struct Drone {
    pub pos: (f32, f32),
    pub target_pos: (f32, f32),
    pub front: bool,

    pub mode: DroneMode,
   
    timer: f32,
    pub on_site: bool,
    
    phase: f32,
    angle: f32,

    scan: Option<Scan>,

    asteroid_id: u32, 
    pub cargo: Vec<(Resources, u64)>,

    wander_progress: f32,
    wander_forward: bool,
}

impl Drone {
    pub fn new(mode: DroneMode, target_pos: (i32, i32)) -> Self {
        Drone {
            pos: ((DEPOT_BOX.0 + DEPOT_BOX.2/2) as f32, (DEPOT_BOX.1 + DEPOT_BOX.3/2) as f32), // Position of drone depot
            target_pos: (target_pos.0 as f32, target_pos.1 as f32),
            front: true,

            mode,
            timer: 0.,
            on_site: false,

            phase: random::f32(), 
            angle: 0.,

            scan: None,
            asteroid_id: 0,
            cargo: vec![],

            wander_progress: 0.,
            wander_forward: false,
        }
    }

    pub fn update(&mut self, stats: &DroneStats, poi: &mut dyn POI) -> bool {
        match self.mode {
            DroneMode::Survey => self.survey(stats, poi),
            DroneMode::Mining => {
                if let Some(mines) = poi.as_any_mut().downcast_mut::<AsteroidMines>() {
                    return self.update_mining(stats, mines)
                }
                return false
            },
            DroneMode::Shipping => false,
            DroneMode::Conduit => {
                if let Some(plant) = poi.as_any_mut().downcast_mut::<PowerPlant>() {
                    return self.conduit(stats, plant)
                }
                return false
            }
            DroneMode::Research => self.survey(stats, poi),
        }
    }

    pub fn conduit(&mut self, stats: &DroneStats, plant: &mut PowerPlant) -> bool {
        let nebula = &mut plant.nebula;

        if self.on_site {
            self.wander(stats);
            self.timer += 1.;
            if self.timer >= stats.interval * (stats.speed/stats.amped) {
                self.timer = 0.;
                self.target_pos = nebula.get_drone_pos();
                nebula.generate_drone_lightning(self.pos, 15);
                return true;
            }
        } else {
            self.target_pos = ((PLANT_BOX.0 + PLANT_BOX.2/2) as f32, (PLANT_BOX.1 + PLANT_BOX.3/2) as f32);
            self.on_site = self.follow(stats); 
        }
        false
    }

    pub fn wander(&mut self, stats: &DroneStats) -> bool {
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

        self.follow(stats);
        false
    }

    pub fn survey(&mut self, stats: &DroneStats, poi: &dyn POI) -> bool {
        // Calculate the angle based on the timer and interval
        let angle = (self.timer + self.phase) * std::f32::consts::TAU;

        // Define the ellipse dimensions
        let mut center = (0.,0.);
        let mut radius_x = 100.0; // Horizontal radius
        let radius_y = 25.0;  // Vertical radius
        if let Some(_) = poi.as_any().downcast_ref::<ResearchComplex>() {
            radius_x = 34.;
            center = ((COMPLEX_BOX.0 + COMPLEX_BOX.2/2) as f32, (COMPLEX_BOX.1 + COMPLEX_BOX.3/2) as f32);
        } else if let Some(_) = poi.as_any().downcast_ref::<Exoplanet>() {
            center = ((PLANET_BOX.0 + PLANET_BOX.2/2) as f32, (PLANET_BOX.1 + PLANET_BOX.3/2) as f32);
        }

        // Oscillation factor (sinusoidal oscillation between 0.75 and 1.0)
        let raw_oscillation = ((self.timer as f32 / (stats.interval * 2. * (stats.speed/stats.amped))) * std::f32::consts::TAU).sin();
        let oscillation = 0.25 + 0.75 * (0.5 + 0.5 * raw_oscillation); // Oscillates between 0.75 and 1.0

        // Calculate the position on the ellipse with oscillation
        
        self.front = self.pos.1 >= center.1;
        
        if self.on_site {
            self.pos.0 = center.0 + radius_x * angle.cos();
            self.pos.1 = center.1 + radius_y * oscillation * angle.sin();
            
            let orbit_speed = (stats.amped / stats.speed) / (stats.interval * 2.);
            self.timer = (self.timer + orbit_speed) % 1.0;
            
                let threshold = 0.02; // How close to 0.5 or 1.0 to trigger
                let t = self.timer % 1.0;

                let at_half = (t - 0.5).abs() < threshold;
                let at_full = (t - 1.0).abs() < threshold || t < threshold; // handle wrap-around

                if (at_half || at_full) && self.scan.is_none() {
                let mut scan = true;
                if let Some(complex) = poi.as_any().downcast_ref::<ResearchComplex>() {
                    scan = complex.active_project.is_some();
                }
                if scan {
                    let scan = (center.0 + radius_x/3. * angle.cos(), center.1 + radius_x/3. * oscillation * angle.sin()); 
                    self.scan = Some(Scan::new(self.pos, scan));
                    return true;
                }
            }
    
            if let Some(scan) = &mut self.scan {
                if !scan.update(self.pos) {
                    self.scan = None;
                }
            } 
            self.follow(stats);
            false
        } else {
            self.target_pos.0 = center.0 + radius_x * angle.cos();
            self.target_pos.1 = center.1 + radius_y * oscillation * angle.sin();
            if self.follow(stats) {
                self.on_site = true;
            }
            false
        }
    }

    pub fn shipping(&mut self, stats: &DroneStats, enabled: bool) -> Option<(Resources, u64)> {
        // Define the start and bounds for the random target
        let home = ((DEPOT_BOX.0 + DEPOT_BOX.2/2) as f32, (DEPOT_BOX.1 + DEPOT_BOX.3 - 8) as f32);
        let mines = ((MINES_BOX.0 + MINES_BOX.2/2) as f32 -6. - (self.phase * 2.).round() * 8., (MINES_BOX.1 + 2*MINES_BOX.3/3) as f32);
        
        // Fabricating
        if self.on_site {
            let orbit_speed = (stats.amped / stats.speed) / (stats.interval);
            self.timer += orbit_speed;
            let angle = (self.timer + self.phase) * std::f32::consts::TAU;

            self.target_pos = (
                home.0 + (self.phase * 8. + 16.) * angle.sin(),
                home.1 + (self.phase * 8. + 16.) * angle.cos(),
            );
            self.follow(stats);
            // if self.phase % 0.02 == 0.0 {
            //     self.target_pos = (
            //         home.0 + (16. + self.phase * 16.) * angle.sin(),
            //         home.1 + (16. + self.phase * 16.) * angle.cos(),
            //     );
            // } else {
            // }
//            log!("{}", (16. + self.phase * 16.) * angle.sin());
            if self.timer >= 1.0 {
                self.timer = 0.;
                if enabled {
                    let amount = stats.produce()/4;
                    if amount >= self.cargo[0].1 {
                        self.cargo.clear();
                        self.target_pos = mines;
                        self.on_site = false;
                    } else {
                        self.cargo[0].1 -= amount;
                    }
                    return Some((Resources::Metals, amount));
                }
            }
        } else {
            if self.cargo.is_empty() {
                self.target_pos = mines;
            } else {
                self.target_pos = home;
            }
            if self.follow(stats) {
                if !self.cargo.is_empty() {
                    self.on_site = true;
                } else {
                    self.timer += 1.;
                    if self.timer >= (stats.interval / 4.) * stats.speed {
                        self.timer = 0.;
                        let amount = stats.produce();
                        self.cargo.push((Resources::Metals, amount));
                        self.target_pos = home; // Reset target to home after mining
                        return Some((Resources::Metals, amount));
                    }
                }
            }
        }
        None
    }

    pub fn update_mining(&mut self, stats: &DroneStats, mines: &mut AsteroidMines) -> bool {
        let field = &mut mines.asteroid_field;

        // Deposit metals in depot
        if self.on_site {
            self.timer += 1.0;
            if self.timer >= stats.interval * (stats.speed/stats.amped) / 4. {
                self.timer = 0.;
                self.cargo.clear();
                if let Some(asteroid) = {
                    let matching_asteroids: Vec<_> = field
                        .asteroids[0]
                        .iter()
                        .filter(|a| a.angle < 2.3 && a.radius < 2040.0)
                        .collect();
            
                    if !matching_asteroids.is_empty() {
                        let random_index = random::u32() as usize % matching_asteroids.len();
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
        // Mine asteroids
        } else {
            let done = self.follow(stats);
            if self.cargo.is_empty() && done {
                // Active mining
                if let Some(asteroid) = field.asteroids[0].iter_mut().find(|a| a.id == self.asteroid_id) {
                    self.timer += 1.;
                    asteroid.drilling = true; // Start drilling animation
                    self.target_pos = asteroid.pos;
                    if self.timer >= stats.interval * (stats.speed/stats.amped) {
                        self.timer = 0.;
                        self.cargo.push((Resources::Metals, 0));
                        self.target_pos = (15.0 + MINES_BOX.0 as f32 + random::f32() * 33., 0.0); // Reset target to home after mining
                        asteroid.drilling = false; // Stop drilling animation
                        
                    }
                // Assign new target asteroid
                } else if let Some(asteroid) = {
                    let matching_asteroids: Vec<_> = field
                        .asteroids[0]
                        .iter()
                        .filter(|a| a.angle < 2.3 && a.radius < 2040.0)
                        .collect();
            
                    if !matching_asteroids.is_empty() {
                        let random_index = random::u32() as usize % matching_asteroids.len();
                        Some(matching_asteroids[random_index])
                    } else {
                        None
                    }
                } {
                    self.asteroid_id = asteroid.id;
                    self.target_pos = asteroid.pos;
                }
            // If drone has no cargo, but is not at its target position
            } else if self.cargo.is_empty() && !done {
                // Set target pos if target asteroid
                if let Some(asteroid) = field.asteroids[0].iter().find(|a| a.id == self.asteroid_id) {
                    self.target_pos = asteroid.pos;
                // Assign new target asteroid
                } else if let Some(asteroid) = {
                    let matching_asteroids: Vec<_> = field
                    .asteroids[0]
                    .iter()
                    .filter(|a| a.angle < 2.3 && a.radius < 2040.0)
                        .collect();
                    
                    if !matching_asteroids.is_empty() {
                        let random_index = random::u32() as usize % matching_asteroids.len();
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

    pub fn follow(&mut self, stats: &DroneStats) -> bool {
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
        self.pos.0 += normalized.0/(stats.speed/stats.amped);
        self.pos.1 += normalized.1/(stats.speed/stats.amped);

        // Check if the drone has reached the target position
        let distance_to_target = (
            self.target_pos.0 - self.pos.0,
            self.target_pos.1 - self.pos.1,
        );
        if distance_to_target.0.abs() <= 1./(stats.speed/stats.amped) && distance_to_target.1.abs() <= 1./(stats.speed/stats.amped) {
            self.pos = self.target_pos; // Snap to the target position
            return true; // Indicate that the drone has reached the target
        }

        false
    }


    pub fn draw(&self) {
        rect!(
            xy = (self.pos.0-1., self.pos.1+(turbo::time::tick()as f32/2.%10.)*0.5), 
            wh = (2, 1),
            color = 0xffc247ff, 
        );
        sprite!("drone", xy = (self.pos.0 - 2., self.pos.1 -2.), wh = (4, 4));
        let distance_to_target = (
            self.target_pos.0 - self.pos.0,
            self.target_pos.1 - self.pos.1,
        );
     
    }

    pub fn draw_scan(&self) {
        if let Some(scan) = &self.scan {
            scan.draw();
        }
    }
}