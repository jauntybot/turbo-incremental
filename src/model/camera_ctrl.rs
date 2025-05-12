use super::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct CameraCtrl {
    pub zoom_tick: usize,
    dragging: bool,
    pub last_pointer_pos: (i32, i32),
    velocity: (i32, i32), 
}
impl CameraCtrl {
    pub fn load() -> Self {
        CameraCtrl {
            zoom_tick: 0,
            dragging: false,
            last_pointer_pos: (0, 0),
            velocity: (0, 0), // Initialize velocity to zero
        }
    }

    pub fn update(&mut self) {
        let gp = gamepad(0);
        let p = pointer();
        let move_speed = 3;

        if gp.left.pressed() { //&& camera::x() > 0.0 {
            camera::move_x(-move_speed);
            //if camera::x() < 0.0 { camera::set_x(0.0); }
        } 
        if gp.right.pressed() { //&& camera::x() < 640. {
            camera::move_x(move_speed);
            //if camera::x() > 640.0 { camera::set_x(640.0); }
        } 

        if gp.up.pressed() { //&& camera::y() > 0.0{
            camera::move_y(-move_speed);
            //if camera::y() < 0.0 { camera::set_y(0.0); }
        } 
        if gp.down.pressed() { //&& camera::y() < 480. {
            camera::move_y(move_speed);
            //if camera::y() > 480.0 { camera::set_y(480.0); }
        }

        if gp.a.just_pressed() || gp.b.just_pressed() {
            self.zoom_tick = tick();
        }
        if gp.a.pressed() && camera::zoom() < 4.0 && (tick() - self.zoom_tick) % 5 == 0 {
            camera::move_zoom(1.0);
            if camera::zoom() >= 3.0 { camera::set_zoom(4.0); } 
            self.zoom_tick = tick();
        } else if gp.b.pressed() && camera::zoom() > 1.0 && (tick() - self.zoom_tick) % 5 == 0 {
            camera::move_zoom(-1.0);
            if camera::zoom() <= 1.0 { camera::set_zoom(1.0); }
            if camera::zoom() == 3.0 { camera::set_zoom(2.0); }
            self.zoom_tick = tick();
        }

        // Handle pointer input for panning
        // let pp = p.relative_position();
        // let damping = 0.4;
        
        // if p.just_pressed() {
        //     self.dragging = true;
        //     self.last_pointer_pos = (pp.0, pp.1);
        //     self.velocity = (0, 0); // Reset velocity when dragging starts
        // } else if p.pressed() && self.dragging {
        //     let dx = pp.0 - self.last_pointer_pos.0;
        //     let dy = pp.1 - self.last_pointer_pos.1;

        //     // Update velocity based on pointer movement
        //     self.velocity.0 += -dx;
        //     self.velocity.1 += -dy;

        //     self.last_pointer_pos = (pp.0, pp.1);
        // } else if p.released() {
        //     self.dragging = false;
        // }

        // // Apply velocity to the camera position
        // camera::move_x(self.velocity.0);
        // camera::move_y(self.velocity.1);

        // // Clamp the camera's position to the bounds (0, 0, 640, 480)
        // if camera::x() < 0.0 {
        //     camera::set_x(0.0);
        // } else if camera::x() > 640.0 {
        //     camera::set_x(640.0);
        // }

        // if camera::y() < 0.0 {
        //     camera::set_y(0.0);
        // } else if camera::y() > 480.0 {
        //     camera::set_y(480.0);
        // }

        // // Apply damping to gradually reduce velocity
        // self.velocity.0 = (self.velocity.0 as f32 * damping) as i32;
        // self.velocity.1 = (self.velocity.0 as f32 * damping) as i32;

        // Handle pointer input for zooming
        // if p.scroll_y() > 0.0 && camera::zoom() < 4.0 {
        //     camera::move_zoom(1.0);
        // } else if p.scroll_y() < 0.0 && camera::zoom() > 1.0 {
        //     camera::move_zoom(-1.0);
        // }
    }

}