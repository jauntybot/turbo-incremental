use super::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Collection {
    pub is_active: bool,
    created_at: usize,
    pos: (f32, f32),
    value: (Resources, u32),
}

impl Collection {
    pub fn new(pos: (f32, f32), value: (Resources, u32)) -> Self {
        Self {
            is_active: true,
            created_at: tick(),
            pos,
            value,
        }
    }

    pub fn update(&mut self) {
        // Calculate the lifetime progress (0.0 to 1.0)
        let lifetime = 35; // Total lifetime in ticks
        let elapsed = tick() - self.created_at;
        let progress = (elapsed as f32 / (lifetime as f32 - 10.)).min(1.0);

        // Gradually reduce the vertical movement speed
        let speed = 1.5 * (1.0 - progress); // Slows down as progress approaches 1.0
        self.pos.1 -= speed;

        // Deactivate the collection after its lifetime
        if elapsed > lifetime {
            self.is_active = false;
        }
    }

    pub fn draw(&self) {
        // Drawing logic for the collection
        let sprite = format!("{}", self.value.0);

        sprite!(&sprite, xy = self.pos, wh = (16, 16));
        text!("{}", self.value.1; font = "large", x = self.pos.0 + 16., y = self.pos.1 + 4., color = 0xffffffff);
    }
}