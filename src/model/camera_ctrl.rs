use super::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct CameraCtrl {
    pub zoom_tick: usize,
}
impl CameraCtrl {
    pub fn update(&mut self) {
        let gp = gamepad(0);
        let p = pointer();
        let move_speed = 3;

        if gp.left.pressed() && camera::x() > 0.0 {
            camera::move_x(-move_speed);
        } else if gp.right.pressed() && camera::x() < 640. {
            camera::move_x(move_speed);
        } 

        if gp.up.pressed() && camera::y() > 0.0{
            camera::move_y(-move_speed);
        } else if gp.down.pressed() && camera::y() < 640. {
            camera::move_y(move_speed);
        }

        if gp.a.just_pressed() || gp.b.just_pressed() {
            self.zoom_tick = tick();
        }
        if gp.a.pressed() && camera::zoom() < 4.0 && (tick() - self.zoom_tick) % 5 == 0 {
            camera::move_zoom(1.0);
            if camera::zoom() >= 3.0 { camera::set_zoom(4.0); } 
        } else if gp.b.pressed() && camera::zoom() > 1.0 && (tick() - self.zoom_tick) % 5 == 0 {
            camera::move_zoom(-1.0);
            if camera::zoom() < 1.0 { camera::set_zoom(1.0); }
            if camera::zoom() == 3.0 { camera::set_zoom(2.0); }
        }
    }

}