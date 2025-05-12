use super::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Collection {
    pub is_active: bool,
    created_at: usize,
    pos: (f32, f32),
    value: (Resources, u64),
    positive: bool,
}

impl Collection {
    pub fn new(pos: (f32, f32), value: (Resources, u64)) -> Self {
        Collection::new_detail(pos, value, true)
    }

    pub fn new_detail(pos: (f32, f32), value: (Resources, u64), positive: bool) -> Self {
        Self {
            is_active: true,
            created_at: tick(),
            pos,
            value,
            positive,
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
        let mut amount = Numbers::format(self.value.1);
        if !self.positive { amount = format!("-{}", amount); }
        let color: u32 = if !self.positive { 0xff0000ff } else { 0xffffffff };
        text!(&amount, font = "large", x = self.pos.0 + 16., y = self.pos.1 + 4., color = color);
    }
}