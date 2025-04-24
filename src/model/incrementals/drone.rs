use super::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub enum DroneMode {
    Survey,
    Mining,
    Conduit,
}

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Drone {
    pub pos: (f32, f32),
    target_pos: (f32, f32),
    x_tween: Tween<f32>,
    y_tween: Tween<f32>,
    pub front: bool,
    pub interval: u32,
    pub mode: DroneMode,
    timer: u32,
    phase: f32,

    pub level: u32,
}
impl Drone {
    pub fn new(mode: DroneMode, level: u32) -> Self {
        Drone {
            pos: (224.,448.), // Position of drone depot
            target_pos: (224.,448.),
            x_tween: Tween::new(224.),
            y_tween: Tween::new(448.),
            front: true,
            interval: match mode {
                DroneMode::Survey => 400,
                DroneMode::Mining => 400,
                DroneMode::Conduit => 400,
            },
            mode,
            timer: 0,
            phase: (rand() as f32 % 100.) / 100., 

            level,
        }
    }

    pub fn update(&mut self) -> bool {
        match self.mode {
            DroneMode::Survey => {
                // Calculate the angle based on the timer and interval
                let angle = ((self.timer as f32 / self.interval as f32) + self.phase) * std::f32::consts::TAU; // TAU = 2 * PI

                // Define the ellipse dimensions
                let center = (320.0, 320.0 ); // Center of the ellipse
                let radius_x = 100.0; // Horizontal radius
                let radius_y = 25.0;  // Vertical radius

                // Oscillation factor (sinusoidal oscillation between 0.75 and 1.0)
                let raw_oscillation = ((self.timer as f32 / self.interval as f32) * std::f32::consts::TAU).sin();
                let oscillation = 0.25 + 0.75 * (0.5 + 0.5 * raw_oscillation); // Oscillates between 0.75 and 1.0

                // Calculate the position on the ellipse with oscillation
                self.pos.0 = center.0 + radius_x * angle.cos();
                self.pos.1 = center.1 + radius_y * oscillation * angle.sin();

                self.front = self.pos.1 >= center.1;

                self.timer += 1;
            },
            DroneMode::Mining => {  
                // Define the start and bounds for the random target
                let home = (160.0, 160.0);
                let bounds = (0.0, 0.0, 64.0, 64.0);

                // Generate a random target position if not already set
                if self.timer <= self.interval/4 && self.target_pos == home {
                    let target_x = ((rand() as f32 % 100.) / 100.) * bounds.2 + bounds.0;
                    let target_y = ((rand() as f32 % 100.) / 100.) * bounds.3 + bounds.1;
                    self.pos = home; // Reset to start position
                    self.target_pos = (target_x, target_y); // Set the random target
                    self.timer = 0; // Reset the timer
                    
                    self.x_tween = Tween::new(self.pos.0, );
                    self.y_tween = Tween::new(self.pos.1, );
                    self.x_tween.end = self.target_pos.0;
                    self.x_tween.duration = self.interval as usize / 4;
                    self.y_tween.end = self.target_pos.1;
                    self.y_tween.duration = self.interval as usize / 4;
                } else if self.timer >= self.interval / 2 && self.target_pos != home {
                    self.pos = self.target_pos;

                    self.target_pos = home;

                    self.x_tween = Tween::new(self.pos.0, );
                    self.y_tween = Tween::new(self.pos.1, );
                    self.x_tween.end = self.target_pos.0;
                    self.x_tween.duration = self.interval as usize / 4;
                    self.y_tween.end = self.target_pos.1;
                    self.y_tween.duration = self.interval as usize / 4;
                }

                self.pos.0 = self.x_tween.get();
                self.pos.1 = self.y_tween.get();
                
                self.timer += 1;
            },
            DroneMode::Conduit => {
                // Define the start and bounds for the random target
                let home = (480., 160.0);
                let bounds = (576.0, 0.0, 64.0, 64.0);

                // Generate a random target position if not already set
                if self.timer <= self.interval/4 && self.target_pos == home {
                    let target_x = ((rand() as f32 % 100.) / 100.) * bounds.2 + bounds.0;
                    let target_y = ((rand() as f32 % 100.) / 100.) * bounds.3 + bounds.1;
                    self.pos = home; // Reset to start position
                    self.target_pos = (target_x, target_y); // Set the random target
                    self.timer = 0; // Reset the timer
                    
                    self.x_tween = Tween::new(self.pos.0, );
                    self.y_tween = Tween::new(self.pos.1, );
                    self.x_tween.end = self.target_pos.0;
                    self.x_tween.duration = self.interval as usize / 4;
                    self.y_tween.end = self.target_pos.1;
                    self.y_tween.duration = self.interval as usize / 4;
                } else if self.timer >= self.interval / 2 && self.target_pos != home {
                    self.pos = self.target_pos;

                    self.target_pos = home;

                    self.x_tween = Tween::new(self.pos.0, );
                    self.y_tween = Tween::new(self.pos.1, );
                    self.x_tween.end = self.target_pos.0;
                    self.x_tween.duration = self.interval as usize / 4;
                    self.y_tween.end = self.target_pos.1;
                    self.y_tween.duration = self.interval as usize / 4;
                }

                self.pos.0 = self.x_tween.get();
                self.pos.1 = self.y_tween.get();

                self.timer += 1;
            },
        }
        if self.timer >= self.interval {
            self.timer = 0;
            return true;
        }
        false
    }

    pub fn draw(&self) {
        circ!(xy = self.pos, diameter = 4, color = 0xdf7126ff);
    }
}