use super::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct CameraCtrl {
    pub zoom_tick: usize,
    pub dragging: bool,
    pub last_pointer_pos: (f32, f32),
    pub pos: (f32, f32),
    pub velocity: (f32, f32), 
}
impl CameraCtrl {
    pub fn load() -> Self {
        camera::set_xy(320, 240);
        CameraCtrl {
            zoom_tick: 0,
            dragging: false,
            last_pointer_pos: (0., 0.),
            pos: (320., 200.),
            velocity: (0., 0.), // Initialize velocity to zero
        }
    }

    pub fn update(&mut self) {
        let gp = gamepad(0);
        let p = pointer();
        let move_speed = 3.;

        let mut moved = false;
        if gp.left.pressed() { //&& camera::x() > 0.0 {
            self.pos.0 -= move_speed;
            moved = true;
        } 
        if gp.right.pressed() { //&& camera::x() < 640. {
            self.pos.0 += move_speed;
            moved = true;
        } 
        if gp.up.pressed() { //&& camera::y() > 0.0{
            self.pos.1 -= move_speed;
            moved = true;
        } 
        if gp.down.pressed() { //&& camera::y() < 480. {
            self.pos.1 += move_speed;
            moved = true;
        }

        // Handle pointer input for panning
        let pp = p.xy_fixed();
        let damping = 0.4;
        
        if p.just_pressed() {
            self.dragging = true;
            self.last_pointer_pos = (pp.0 as f32, pp.1 as f32);
            self.velocity = (0., 0.); // Reset velocity when dragging starts
        } else if p.pressed() && self.dragging {
            let dx = pp.0 as f32 - self.last_pointer_pos.0;
            let dy = pp.1 as f32 - self.last_pointer_pos.1;

            // Update velocity based on pointer movement
            self.velocity.0 += -dx;
            self.velocity.1 += -dy;

            self.last_pointer_pos = (pp.0 as f32, pp.1 as f32);
        } else if p.released() {
            self.dragging = false;
        }

        // Apply velocity to the camera position
        self.pos.0 += self.velocity.0;
        self.pos.1 += self.velocity.1;
        // Apply damping to gradually reduce velocity
        self.velocity.0 = self.velocity.0 * damping;
        self.velocity.1 = self.velocity.0 * damping;

        if self.velocity.0 >= 0.2 || self.velocity.1 >= 0.2 
            || self.velocity.0 <= -0.2 || self.velocity.1 <= -0.2 {
            moved = true;
        }

        if moved {
            self.pos = (
                self.pos.0.clamp(0., 640.), 
                self.pos.1.clamp(0., 400.)
            );
        }

        // zoom
        if (gp.a.pressed() || p.scroll_delta().1 < 0) && camera::zoom() < 4.0 && (tick() - self.zoom_tick) > 5 {
            camera::move_zoom(1.0);
            if camera::zoom() >= 3.0 { camera::set_zoom(4.0); } 
            self.zoom_tick = tick();
        } else if (gp.b.pressed() || p.scroll_delta().1 > 0) && camera::zoom() > 0.5 && (tick() - self.zoom_tick) > 5 {
            camera::move_zoom(-1.0);
            if camera::zoom() <= 1.0 { camera::set_zoom(1.0); }
            if camera::zoom() == 3.0 { camera::set_zoom(2.0); }
            self.zoom_tick = tick();
        }

    }

    pub fn update_cam(&self) {
        camera::set_xy(self.pos.0, self.pos.1);
    }

}