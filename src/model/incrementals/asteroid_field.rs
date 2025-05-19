use super::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Debris {
    pub pos: (f32, f32),
    pub angle: f32,  // Current angle of the debris on the circle
    pub speed: f32,  // Angular speed (radians per frame)
    pub size: f32, // Radius of the circles
    pub lifetime: u32,
    pub timer: u32,
}
impl Debris {
    pub fn new(pos: (f32, f32), size: f32) -> Self {
        Debris {
            pos,
            angle: rand() as f32 % std::f32::consts::TAU, // Random angle in range [0, 2*PI]
            speed: -(((rand() % 101) as f32 / 100.0) * 0.2 + 0.1), // Negative angular speed for clockwise motion
            size: 6.0 + size/3. * ((rand() % 101) as f32 / 100.0), // Random radius in range [960.0, 1280.0]
            lifetime: 30 + (rand() % 30) as u32, // Random lifetime in range [60, 120]
            timer: 0,
        }
    }

    pub fn update(&mut self, anchor: (f32, f32)) -> bool {
        self.timer += 1;
        if self.timer >= self.lifetime {
            return true;
        }
        self.pos = (
            anchor.0 + self.timer as f32 * self.speed * self.angle.cos(),
            anchor.1 - self.timer as f32 * self.speed * self.angle.sin()
        );
        if self.timer >= self.lifetime/2 {
            self.size *= 0.95; // Gradually shrink the debris
        }
        false
    }

    pub fn draw(&self) {
        // Draw the debris as a small circle
        circ!(xy = (self.pos.0 - self.size/2., self.pos.1 - self.size/2.), diameter = self.size, color = 0x555555ff);
        circ!(xy = (self.pos.0 - self.size/2. + 1., self.pos.1 - self.size/2. + 1.), diameter = self.size/2., color = 0x7d7d7dff);
    }
}

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Asteroid {
    pub pos: (f32, f32),
    pub angle: f32,  // Current angle of the asteroid on the circle
    pub speed: f32,  // Angular speed (radians per frame)
    pub radius: f32, // Radius of the circle
    pub size: f32,   // Diameter of the asteroid
    pub id: u32,
    pub drilling: bool,
    debris: Vec<Debris>, // Chunks of the asteroid
    sprite: u32,
    rot: u32,
}

impl Asteroid {
    pub fn new() -> Self {
        // Use the assumed `rand()` function to generate random values
        let angle = std::f32::consts::FRAC_PI_2 + 0.6; // Start at the top middle (90 degrees or π/2 radians)
        let speed = -(((rand() % 101) as f32 / 100.0) * 0.0001 + 0.0001); // Negative angular speed for clockwise motion
        let radius = 1920.0 + 384.0 * ((rand() % 101) as f32 / 100.0); // Random radius in range [960.0, 1280.0]
        let size = 8.0 + ((rand() % 17) as f32); // Random size in range [8.0, 24.0]
        let id = rand() as u32; // Unique ID for the asteroid
        
        Self {
            pos: (-320.0, -320.0),
            angle,
            speed,
            radius,
            size,
            id,
            drilling: false,
            debris: vec![],
            sprite: rand() % 3,
            rot: rand() % 4
        }
    }

    pub fn update(&mut self) {
        // Update the angle based on the angular speed
        self.angle -= self.speed;

        // Keep the angle within the range [0, 2*PI]
        if self.angle > std::f32::consts::TAU {
            self.angle -= std::f32::consts::TAU;
        }
        self.pos = (
            1344.0 + self.radius * self.angle.cos(),
            1344.0 - self.radius * self.angle.sin()
        );

        if self.drilling {
            if self.debris.len() < 10 {
                self.debris.push(Debris::new(self.pos, self.size));
            }
        }
        self.debris.retain_mut(|chunk| !chunk.update(self.pos)); // Remove debris that has reached its lifetime
    }

    pub fn draw(&self, color: usize) {
        // Draw the asteroid as a circle
        //circ!(xy = (self.pos.0 - self.size/2., self.pos.1 - self.size/2.), diameter = self.size, color = 0xaaaaaaff);
        let sprite = format!("stroid_{:02}", self.sprite);
        let c: u32 = {
            match color {
                0 => 0x555555ff,
                1 => 0x949494ff,
                _ => 0xffffffff,
            }
        };
        sprite!(
            &sprite,
            x = self.pos.0 - 12.,
            y = self.pos.1 - 12.,
            color = c,
            rotation = self.rot * 90,
        );
        for chunk in self.debris.iter() {
            chunk.draw();
        }
    }
}



#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct AsteroidField {
    pub asteroids: Vec<Vec<Asteroid>>,
    pub limit: usize,       // Maximum number of asteroids
    pub spawn_interval: u32, // Interval between spawns (in frames)
    pub timer: u32,         // Timer to track spawn intervals
    belt_index: usize,
}

impl AsteroidField {
    pub fn new() -> Self {
        Self {
            asteroids: vec![vec![], vec![], vec![]],
            limit: 350,
            spawn_interval: 5,
            timer: 0,
            belt_index: 0,
        }
    }

    pub fn update(&mut self) {
        // Update existing asteroids
        for belt in self.asteroids.iter_mut() {
            for asteroid in belt.iter_mut() {
                asteroid.update();
            }
        }

        // Remove asteroids that have reached the left middle point (angle = π radians)
        for belt in self.asteroids.iter_mut() {
            belt.retain(|asteroid| asteroid.angle < 2.62);
        }

        // Increment the timer
        self.timer += 1;

        // Spawn new asteroids if below the limit and the interval has passed
        let stroids: usize = self.asteroids.iter().map(|belt| belt.len()).sum();
        if stroids < self.limit && self.timer >= self.spawn_interval {
            self.asteroids[self.belt_index].push(Asteroid::new());
            self.timer = 0; // Reset the timer
            self.belt_index += 1;
            if self.belt_index >= 3 {
                self.belt_index = 0;
            }
        }
    }

    pub fn draw(&self) {
        //rect!(xy = (-320, -240), wh = (640, 480), border_size = 1, color = 0x00000000, border_color = 0x0ffffffff);
        // Draw all asteroids
        
        for i in 0..self.asteroids.len() {
            for asteroid in self.asteroids[2 - i].iter() {
                asteroid.draw(i);
            }
        }
    }
}