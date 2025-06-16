use super::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Player {
    pub resources: Vec<(Resources, u64)>,
    hitbox: Bounds,
    target_pos: (f32, f32),
    dir: f32,

    pub camera: CameraCtrl,

    scans: Vec<Scan>,
    prestiged: bool,
    jumping: bool,
    jump_timer: u32,
    gate_aligned: bool,

    pub prestige_prog: u64,
    pub prestige_index: u32,
    pub prestige_limit: u64,
    pub prestige_earned: u64,
    pop_up: PopUp,
    hovered: bool,
    pub hovered_else: bool,
    pub avail_upgrades: Vec<Upgrade>,
}

impl Player {
    pub fn load(prestiged: bool, prestige_earned: u64, prestige_prog: u64, prestige_index: u32, avail_upgrades: Vec<Upgrade>) -> Self {
//        let hitbox = Bounds::new(xy)
        Player {
            resources: vec![
                (Resources::Research, 4000000000),
                (Resources::Drones, 4000000000),
                (Resources::Metals, 4000000000),
                (Resources::Power, 4000000000),
                (Resources::Prestige, prestige_earned),
            ],
            hitbox: Bounds::new(320., 600., 16, 16),
            target_pos: (0., 0.),
            dir: 0.,

            camera: CameraCtrl::load(),
            scans: vec![],
            prestiged,
            jumping: false,
            jump_timer: 0,
            gate_aligned: false,

            prestige_prog,
            prestige_index,
            prestige_limit: CostFormula::Exponential.calculate_cost(vec![(Resources::Prestige, 200_000)], prestige_index)[0].1,
            prestige_earned: 0,
            pop_up: PopUp::new("RESEARCH PROBE".to_string(), Resources::Prestige),
            hovered: false,
            hovered_else: false,
            avail_upgrades,
        }
    } 

    pub fn update(&mut self, event_manager: &mut EventManager) {
        self.hovered_else = false;
        if !self.jumping {
            self.target_pos = camera::xy();
            
            self.hitbox = self.hitbox.position(
                (self.hitbox.xy().0 as f32 + (self.target_pos.0 - self.hitbox.xy().0 as f32) * 0.1) as i32,
                (self.hitbox.xy().1 as f32 + (self.target_pos.1 - self.hitbox.xy().1 as f32) * 0.1) as i32
            );
            
            if event_manager.dialogue.is_none() {
                self.hovered = self.prestiged && !self.hovered_else && (self.hitbox.intersects_xy(pointer().xy()) || (self.hovered && self.pop_up.hovered())); 
            } else {
                self.hovered = false;
            }

            if self.hovered {
                // Pop up returns upgrade player clicks
                if let Some(upgrade) = self.pop_up.update(self.hitbox, &Station{drone_base: 20.,drone_eff: 1.0,drone_speed: 2000.,}, &mut self.avail_upgrades, &PROBE_UPGRADES, &self.resources) {
                    self.upgrade(&upgrade);
                }
            }

            let dx = self.target_pos.0 - self.hitbox.xy().0 as f32;
            let dy = self.target_pos.1 - self.hitbox.xy().1 as f32;
            let angle = dy.atan2(dx).to_degrees(); // Angle in radians
            let distance_to_target = (
                self.target_pos.0 - self.hitbox.xy().0 as f32,
                self.target_pos.1 - self.hitbox.xy().1 as f32,
            );
            if distance_to_target.0.abs() > 0.01 && distance_to_target.1.abs() > 0.01 {
                self.dir = angle + 90.;
            }
            
            self.camera.update();
            self.camera.update_cam();

            self.scans.retain_mut(|scan| {
                scan.update((self.hitbox.x() as f32, self.hitbox.y() as f32))
            });
            
            if self.hovered {
                // Pop up returns upgrade player clicks
                if let Some(upgrade) = self.pop_up.update(self.hitbox, &Station{drone_base: 20.,drone_eff: 1.0,drone_speed: 2000.,}, &mut self.avail_upgrades, &GATE_UPGRADES, &self.resources) {
                    self.upgrade(&upgrade);
                }
            }

        } else {
            self.jump(event_manager);
        }

        if self.prestige_prog >= self.prestige_limit {
            self.prestige_earned += 1;
            self.prestige_index += 1;
            self.prestige_limit = CostFormula::Exponential.calculate_cost(vec![(Resources::Prestige, 200_000)], self.prestige_index)[0].1;
            self.prestige_prog = 0;
        }
    }

    pub fn jump(&mut self, event_manager: &mut EventManager) {
        if !self.gate_aligned {
            //log!("aligning");
            self.target_pos = ((GATE_BOX.0 + GATE_BOX.2/2) as f32, (GATE_BOX.1 - 16) as f32);
            self.hitbox = self.hitbox.position(
                (self.hitbox.xy().0 as f32 + (self.target_pos.0 - self.hitbox.xy().0 as f32) * 0.1) as i32,
                (self.hitbox.xy().1 as f32 + (self.target_pos.1 - self.hitbox.xy().1 as f32) * 0.1) as i32
            );
            
            let distance_to_target = (
                self.target_pos.0 - self.hitbox.xy().0 as f32,
                self.target_pos.1 - self.hitbox.xy().1 as f32,
            );
            if distance_to_target.0.abs() < 1.0 {
                self.gate_aligned = true;
            }
        } else {
            self.jump_timer += 1;

            // Rubber band up (decelerating motion)
            if self.jump_timer <= 50 {
                self.hitbox = self.hitbox.translate_y(((50 - self.jump_timer) as f32 * 0.15) as i32); // Move down faster as time progresses
                self.dir += (180.0 - self.dir) * 0.1;
            }
            // Slingshot down (accelerating motion)
            else if self.jump_timer <= 150 {
                self.hitbox = self.hitbox.translate_y((self.jump_timer - 50) as f32 * 0.5);
            }

            if self.hitbox.xy().1 as f32 >= (GATE_BOX.1 + GATE_BOX.3/2 - 2) as f32 {
                self.hitbox = self.hitbox.translate_y(400);
            }

            // Trigger the end game event after the motion completes
            if self.jump_timer == 150 {
                event_manager.trigger(Event::EndGame);
                self.jump_timer += 100; // Prevent further updates
            }
        }
    }

    pub fn collect(&mut self, resource: (Resources, u64)) {
        self.prestige_prog += resource.1;
        // Append value to exisiting resource
        let mut found = false;
        for i in 0..self.resources.len() {
            if self.resources[i].0 == resource.0 {
                self.resources[i].1 += resource.1;
                found = true;
                break;
            }
        }
        // Append resrouce and value
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

    pub fn upgrade(&mut self, upgrade: &Upgrade) {
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

    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::Prestige => {
                self.jumping = true;
                if !self.prestiged {
                    self.avail_upgrades.push(PROBE_UPGRADES[0].clone());
                    self.avail_upgrades.push(PROBE_UPGRADES[1].clone());
                }
            }
            _ => {}
        }
    }

    pub fn scan(&mut self) {
        let pp = pointer().xy();
        let pos = (pp.0 as f32 + 5., pp.1 as f32 - 5.);
        self.scans.push(Scan::new((self.hitbox.x() as f32, self.hitbox.y() as f32), pos));
    }

    pub fn draw(&self) {
        for scan in self.scans.iter() {
            scan.draw();
        }
        // rect!( 
        //     xy = (self.hitbox.xy().0 - 8., self.hitbox.xy().1 - 8.),
        //     wh = (16, 16),
        // );
        if self.hovered {
            sprite!(
            "player_hovered", 
            xy = (self.hitbox.xy().0 - 9, self.hitbox.xy().1 - 9),
            rotation = self.dir,
        );
        }

        sprite!(
            "player", 
            xy = (self.hitbox.xy().0 - 8, self.hitbox.xy().1 - 8),
            rotation = self.dir,
        );
        text!("{}", self.prestige_prog; xy = (self.hitbox.xy().0 - 8, self.hitbox.xy().1 - 8),);

        if self.jumping && self.jump_timer >= 60 {
            let anim = animation::get("jump");
            anim.use_sprite("jump");
            anim.set_repeat(0);
            anim.set_fill_forwards(true);

            // Draw the scan effect
            sprite!(animation_key = "jump", xy = (GATE_BOX.0, GATE_BOX.1 - 64));
        }

        PlayerDisplay::draw(&self.resources);
    }

    pub fn draw_ui(&self) { 
        // pop up
        if self.hovered {
            self.pop_up.draw(&Station{ drone_base: 20.,
            drone_eff: 1.0,
            drone_speed: 0.,}, &self.avail_upgrades);
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::load(false, 0, 0, 0, vec![])
    }
}

impl POI for Player {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_station(&self) -> &Station {
        &Station {
            drone_base: 20.,
            drone_eff: 1.0,
            drone_speed: 0.,
        }
    }

    fn manual_produce(&mut self) -> u64 {
        return 0;
    }

    fn produce(&mut self) -> u64 {
        return 0;
    }

    fn upgrade(&mut self, upgrade: &Upgrade, event_manager: &mut EventManager) {}
}

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct PlayerDisplay {}
impl PlayerDisplay {
    pub fn draw(resources: &Vec<(Resources, u64)>) {
        let vp = Bounds::new(0, 0, 640, 400);
        let wh = (64, resources.len() as i32 * 24 + 20);
        let xy = (0, vp.bottom() - wh.1);

        rect!(fixed = true, x = xy.0, y = xy.1, w = wh.0, h = wh.1, border_radius = 4, border_size = 1, color = 0x1f122bff, border_color = 0xffffffff);
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
    key: String,
}

impl Scan {
    pub fn new(origin: (f32, f32), pos: (f32, f32)) -> Self {
        Self {
            origin,
            pos,
            radius: 0.0,
            lifetime: 40.,
            key: rand().to_string(),
        }
    }

    pub fn update(&mut self, origin: (f32, f32)) -> bool {
        self.lifetime -= 1.;
        self.origin = origin;
        if self.lifetime <= -20. {
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

                sprite!(
                    "scan_line",
                    xy = dash_start,
                );
            }
        }
        let anim = animation::get(&self.key);
        anim.use_sprite("scan");
        anim.set_repeat(0);

        // Draw the scan effect
        sprite!(animation_key = &self.key, xy = (self.pos.0 - 8., self.pos.1 - 8.));
    }

}