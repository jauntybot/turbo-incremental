use super::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Player {
    pub resources: Vec<(Resources, u64)>,
    pub xy: (f32, f32),
    target_pos: (f32, f32),
    dir: f32,

    pub camera: CameraCtrl,

    scans: Vec<Scan>,
    prestiged: bool,
    jumping: bool,
    jump_timer: u32,
    gate_aligned: bool,
}

impl Player {
    pub fn load() -> Self {
        Player {
            resources: vec![
                // (Resources::Research, 4000000000),
                // (Resources::Metals, 4000000000),
                // (Resources::Power, 4000000000),
            ],
            xy: (320., 600.),
            target_pos: (0., 0.),
            dir: 0.,

            camera: CameraCtrl::load(),
            scans: vec![],
            prestiged: false,
            jumping: false,
            jump_timer: 0,
            gate_aligned: false,
        }
    } 

    pub fn update(&mut self, event_manager: &mut EventManager) {
        if !self.jumping {
            self.target_pos = camera::xy();
            
            self.xy.0 = self.xy.0 as f32 + (self.target_pos.0 - self.xy.0) as f32 * 0.1;
            self.xy.1 = self.xy.1 as f32 + (self.target_pos.1 - self.xy.1) as f32 * 0.1;
            
            let dx = self.target_pos.0 - self.xy.0;
            let dy = self.target_pos.1 - self.xy.1;
            let angle = dy.atan2(dx).to_degrees(); // Angle in radians

            // Map the angle to a direction (0-7 for 8 directions)
            self.dir = angle + 90.;
            
            self.camera.update();
            self.camera.update_cam();

            self.scans.retain_mut(|scan| {
                scan.update(self.xy)
            });

        } else {
            self.jump(event_manager);
        }
    }

    pub fn jump(&mut self, event_manager: &mut EventManager) {
        if !self.gate_aligned {
            //log!("aligning");
            self.target_pos = ((GATE_BOX.0 + GATE_BOX.2/2) as f32, (GATE_BOX.1 - 16) as f32);
            self.xy.0 = self.xy.0 as f32 + (self.target_pos.0 - self.xy.0) as f32 * 0.1;
            self.xy.1 = self.xy.1 as f32 + (self.target_pos.1 - self.xy.1) as f32 * 0.1;
            let distance_to_target = (
                self.target_pos.0 - self.xy.0,
                self.target_pos.1 - self.xy.1,
            );
            if distance_to_target.0.abs() < 1.0 {
                self.gate_aligned = true;
            }
        } else {
            self.jump_timer += 1;
            self.xy.1 += self.jump_timer as f32 / 1.;
            if self.jump_timer == 150 {
                event_manager.trigger(Event::EndGame);
                self.jump_timer += 100;
            }
        }
    }

    pub fn collect(&mut self, resource: (Resources, u64)) {
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

    pub fn remove(&mut self, resource: (Resources, u64)) {
        for i in 0..self.resources.len() {
            if self.resources[i].0 == resource.0 {
                if self.resources[i].1 >= resource.1 {
                    self.resources[i].1 -= resource.1;
                } else {
                    self.resources.remove(i);
                }
                break;
            }
        }
    }

    pub fn upgrade(&mut self, upgrade: &Upgrade, poi: &mut dyn POI) {
        // Determine if the player has sufficent resources
        let mut found = false;
        for cost in upgrade.cost.iter() {
            for i in 0..self.resources.len() {
                if self.resources[i].0 == cost.0 {
                    // Subtract resources from player
                    if self.resources[i].1 >= cost.1 {
                        self.resources[i].1 -= cost.1;
                        found = true;
                    // Exit loops early when found
                    } else { break; }
                    break;
                }
            }
        }
            
        if !found {
            return;
        }
        // Match the concrete type of `poi`
        if let Some(_) = poi.as_any_mut().downcast_mut::<Exoplanet>() {
            if upgrade.name.starts_with("FIELD") {
                //exoplanet.scanner_level += 1;
            }
        } else if let Some(_) = poi.as_any_mut().downcast_mut::<DroneDepot>() {
            if upgrade.name == "DRONE SHIPMENT" {
                self.collect((Resources::Drones, 1));
            }
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::Prestige => {
                self.jumping = true;
            }
            _ => {}
        }
    }

    pub fn scan(&mut self) {
        let pp = pointer().xy();
        let pos = (pp.0 as f32 + 5., pp.1 as f32 - 5.);
        self.scans.push(Scan::new(self.xy, pos));
    }

    pub fn draw(&self) {
        for scan in self.scans.iter() {
            scan.draw();
        }
        // rect!( 
        //     xy = (self.xy.0 - 8., self.xy.1 - 8.),
        //     wh = (16, 16),
        // );

        sprite!(
            "player", 
            xy = (self.xy.0 - 8., self.xy.1 - 8.),
            rotation = self.dir,
        );

        PlayerDisplay::draw(&self.resources);
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::load()
    }
}

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct PlayerDisplay {}
impl PlayerDisplay {
    pub fn draw(resources: &Vec<(Resources, u64)>) {
        let vp = Bounds::new(0, 0, 640, 400);
        let wh = (64, resources.len() as i32 * 24 + 20);
        let xy = (0, vp.bottom() - wh.1);

        rect!(fixed = true, x = xy.0, y = xy.1, w = wh.0, h = wh.1, border_radius = 4, border_size = 1, color = 0x222034ff, border_color = 0xffffffff);
        text!("RESOURCES", fixed = true, x = xy.0 + 4, y = xy.1 + 6, color = 0xffffffff);
        rect!(fixed = true, x = xy.0 + 4, y = xy.1 + 18, w = wh.0 - 8, h = 1, color = 0xffffffff);

        for i in 0..resources.len() {
            let h = 24;
            let bb = Bounds::new(xy.0, 20 + xy.1 + i as i32 * h, wh.0, h);
            let mut button = Btn::new("".to_string(), bb.inset(2), true, 0);
            button.clickable = false;
            button.update();
            button.draw();
            if button.state == BtnState::Hovered {
                let mut desc = WrapBox::new(resources[i].0.description(), 0);
                desc.update(button.bounds, 6);
                desc.draw();
            }

            let t = format!("{}", resources[i].0);
            sprite!(&t, fixed = true, x = bb.x() + 4, y = bb.center_y() - 8, wh = (16, 16), color = 0xffffffff);
            //text!(&t, fixed = true, x = bb.ctuenter_x() - t.len() as i32/2 * 5, y = bb.top() + 4, color = 0xffffffff);
            let t = Numbers::format(resources[i].1);
            text!(&t, fixed = true, x = bb.left() + 24, y = bb.center_y() - 4, color = 0xffffffff);
        }
    }
}

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Scan {
    pub origin: (f32, f32),
    pub pos: (f32, f32),
    pub radius: f32,
    pub lifetime: f32,
}

impl Scan {
    pub fn new(origin: (f32, f32), pos: (f32, f32)) -> Self {
        Self {
            origin,
            pos,
            radius: 0.0,
            lifetime: 40.,
        }
    }

    pub fn update(&mut self, origin: (f32, f32)) -> bool {
        self.lifetime -= 1.;
        self.origin = origin;
        if self.lifetime <= 0. {
            return false;
        }
        if self.lifetime <= 30. {
            self.radius += 0.5 * self.lifetime / 40.;
        }
        true
    }

    pub fn draw(&self) {
        let mut start = self.origin;
        let mut end = self.pos;
        if self.lifetime >= 35.0 {
            let t = ((40.0 - self.lifetime) / 5.0).clamp(0.0, 1.0); // Clamp t to [0, 1]
            end = (
                self.origin.0 + (self.pos.0 - self.origin.0) * t,
                self.origin.1 + (self.pos.1 - self.origin.1) * t,
            );
        } else if self.lifetime < 30.0 && self.lifetime >= 20.0 {
            let t = ((30.0 - self.lifetime) / 5.0).clamp(0.0, 1.0); // Clamp t to [0, 1]
            start = (
                self.origin.0 + (self.pos.0 - self.origin.0) * t,
                self.origin.1 + (self.pos.1 - self.origin.1) * t,
            );
        }
        if self.lifetime >= 20. {
            // Dashed line logic
            let dash_length = 2.0; // Length of each dash
            let gap_length = 2.0; // Length of each gap
            let total_length = dash_length + gap_length;

            let dx = end.0 - start.0;
            let dy = end.1 - start.1;
            let line_length = (dx * dx + dy * dy).sqrt();

            let num_dashes = (line_length / total_length).ceil() as usize;
            let unit_dx = dx / line_length;
            let unit_dy = dy / line_length;

            let mut current_start = start;

            for i in 0..num_dashes {
                let dash_start = (
                    current_start.0 + unit_dx * total_length * i as f32,
                    current_start.1 + unit_dy * total_length * i as f32,
                );
                let dash_end = (
                    dash_start.0 + unit_dx * dash_length,
                    dash_start.1 + unit_dy * dash_length,
                );

                // Ensure the dash does not extend beyond the end point
                if (dash_end.0 - start.0).hypot(dash_end.1 - start.1) > line_length {
                    break;
                }

                circ!(
                    xy = dash_start,
                    size = 2,
                    color = 0x99e550ff,
                );
            }
        }

        // Draw the scan effect
        circ!(x = self.pos.0 - self.radius, y = self.pos.1 - self.radius, diameter = self.radius*2., border_size = 1, color = 0x99e55000, border_color = 0x99e550ff);
        circ!(x = self.pos.0 - self.radius/2., y = self.pos.1 - self.radius/2., diameter = self.radius, border_size = 1, color = 0x99e55000, border_color = 0x99e550ff);
    }

}