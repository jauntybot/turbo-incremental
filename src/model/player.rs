use super::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Player {
    pub resources: Vec<(Resources, u32)>,
    pub xy: (f32, f32),
    target_pos: (f32, f32),

    camera: CameraCtrl,
}

impl Player {
    pub fn load() -> Self {
        Player {
            resources: vec![
                (Resources::Research, 0),
            ],
            xy: (320., 0.),
            target_pos: (0., 0.),

            camera: CameraCtrl { zoom_tick: 0 },
        }
    } 

    pub fn update(&mut self) {
        self.target_pos = camera::xy();
        self.xy.0 = self.xy.0 as f32 + (self.target_pos.0 - self.xy.0) as f32 * 0.1;
        self.xy.1 = self.xy.1 as f32 + (self.target_pos.1 - self.xy.1) as f32 * 0.1;
        
        self.camera.update();
    }

    pub fn collect(&mut self, resource: (Resources, u32)) {
        let mut found = false;
        for i in 0..self.resources.len() {
            if self.resources[i].0 == resource.0 {
                self.resources[i].1 += resource.1;
                found = true;
                break;
            }
        }
        if !found {
            self.resources.push(resource);
        }   
    }

    pub fn upgrade(&mut self, upgrade: &Upgrade, poi: &mut dyn POI) {
        // Determine if the player has sufficent resources
        let mut found = false;
        for i in 0..self.resources.len() {
            if self.resources[i].0 == upgrade.cost.0 {
                // Subtract resources from player
                if self.resources[i].1 >= upgrade.cost.1 {
                    self.resources[i].1 -= upgrade.cost.1;
                    found = true;
                // Exit loops early when found
                } else { break; }
                break;
            }
        }
        if !found {
            return;
        }
        // Match the concrete type of `poi`
        if let Some(exoplanet) = poi.as_any_mut().downcast_mut::<Exoplanet>() {
            if upgrade.name.starts_with("FIELD") {
                //exoplanet.scanner_level += 1;
            }
        } else if let Some(depot) = poi.as_any_mut().downcast_mut::<DroneDepot>() {
            if upgrade.name == "DRONE SHIPMENT" {
                self.collect((Resources::Drones, 1));
            }
        }
    }

    pub fn draw(&self) {

        circ!(x = self.xy.0 - 8., y = self.xy.1 - 8., diameter = 16, color = 0xdf7126ff);

        PlayerDisplay::draw(&self.resources);
    }
}


#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct PlayerDisplay {}
impl PlayerDisplay {
    pub fn draw(resources: &Vec<(Resources, u32)>) {
        let vp = Bounds::new(0, 0, 640, 480);
        let wh = (64, resources.len() as i32 * 24 + 20);
        let xy = (0, vp.bottom() - wh.1);

        rect!(fixed = true, x = xy.0, y = xy.1, w = wh.0, h = wh.1, border_radius = 4, border_size = 1, color = 0x222034ff, border_color = 0xffffffff);
        text!("RESOURCES", fixed = true, x = xy.0 + 4, y = xy.1 + 6, color = 0xffffffff);
        rect!(fixed = true, x = xy.0 + 4, y = xy.1 + 18, w = wh.0 - 8, h = 1, color = 0xffffffff);

        for i in 0..resources.len() {
            let h = 24;
            let bb = Bounds::new(xy.0, 20 + xy.1 + i as i32 * h, wh.0, h);
            //rect!(fixed = true, x = bb.x() + 1, y = bb.y() + 1, w = bb.w() - 2, h = bb.h() - 2, color = 0x847e87ff);
           
            let t = format!("{}", resources[i].0);
            sprite!(&t, fixed = true, x = bb.x() + 4, y = bb.center_y() - 8, wh = (16, 16), color = 0xffffffff);
            //text!(&t, fixed = true, x = bb.ctuenter_x() - t.len() as i32/2 * 5, y = bb.top() + 4, color = 0xffffffff);
            let t = Numbers::format(resources[i].1);
            text!(&t, fixed = true, x = bb.left() + 24, y = bb.center_y() - 4, color = 0xffffffff);
        }
    }
}