use super::*;

#[turbo::serialize]
pub struct Player {
    pub resources: Vec<(Resources, u64)>,
    hitbox: Bounds,
    pos: (f32, f32),
    target_pos: (f32, f32),
    dir: f32,

    pub station: Station,

    pub camera: CameraCtrl,

    scans: Vec<Scan>,
    jumping: bool,
    jump_timer: u32,
    gate_aligned: bool,

    pub prestige_prog: u64,
    pub prestige_index: u32,
    pub prestige_limit: u64,
    pub prestige_earned: u64,
    pop_up: PopUp,
    hovered: bool,
    pub hovered_poi: Option<POIType>,
    pub avail_upgrades: Vec<Upgrade>,
    
    purchased: usize,
    pub expertise: f32,

    pub drone_recall: bool,
    pub resourceful: bool,
    pub innovation: bool,
    resource_cloning: bool,

    fin: bool,
}

impl Player {
    pub fn new() -> Self {
        let mut station = Station::new(POIType::ResearchProbe, DroneStats::new(20., 1.0, 100., 1.0));
        station.unlockable = true;
        Player {
            resources: vec![],
            hitbox: Bounds::new(320., 600., 16, 16),
            pos: (320., 600.),
            target_pos: (0., 0.),
            dir: 0.,

            station,

            camera: CameraCtrl::load(),
            scans: vec![],
            jumping: false,
            jump_timer: 0,
            gate_aligned: false,

            prestige_prog: 0,
            prestige_index: 0,
            prestige_limit: CostFormula::Exponential{factor: 1.5}.calculate_cost(vec![(Resources::Prestige, 80_000)], 0)[0].1,
            prestige_earned: 0,
            pop_up: PopUp::new("RESEARCH PROBE".to_string(), DroneMode::Survey),
            hovered: false,
            hovered_poi: None,
            avail_upgrades: vec![],
            purchased: 0,

            expertise: 0.,

            drone_recall: false,
            resourceful: false,
            innovation: false,
            resource_cloning: false,

            fin: false,
        }
    }

    pub fn update(&mut self, event_manager: &mut EventManager) {
        if !self.jumping {
            self.target_pos = (camera::x() - 8., camera::y() - 8.);

            let dx = (self.target_pos.0 - self.pos.0) * 0.1;
            let dy = (self.target_pos.1 - self.pos.1) * 0.1;

            self.pos.0 += dx;
            self.pos.1 += dy;
            self.hitbox = self.hitbox.position( self.pos.0 as i32, self.pos.1 as i32);

            let target_angle = dy.atan2(dx).to_degrees() + 90.;

            // Smoothly (interpolate self.dir toward-8) target_angle
            let mut delta = target_angle - self.dir;
            // Wrap delta to [-180, 180] for shortest rotation
            if delta > 180.0 { delta -= 360.0; }
            if delta < -180.0 { delta += 360.0; }

            // Only rotate if distance is significant
            if dx.abs() > 1.0 || dy.abs() > 1.0 {
                self.dir += delta * 0.2; // 0.2 controls smoothness (increase for snappier)
            } else {
                // Find the nearest 90-degree angle
                let mut target_dir = ((self.dir / 90.0).round() * 90.0) % 360.0;

                // Wrap 360 to 0 if above 315
                if target_dir >= 315.0 {
                    target_dir = 0.0;
                }

                // Calculate shortest rotation delta
                let mut delta = target_dir - self.dir;
                if delta > 180.0 { delta -= 360.0; }
                if delta < -180.0 { delta += 360.0; }

                // Lerp self.dir toward target_dir
                self.dir += delta * 0.2;
            }
            
            self.camera.update();
            self.camera.update_cam();

            self.scans.retain_mut(|scan| {
                scan.update((self.pos.0 + 8., self.pos.1 + 8.))
            });

            if event_manager.dialogue.is_none() {
                let was_hovered = self.hovered;
                self.hovered = 
                    self.station.unlocked 
                    && (self.hovered_poi.is_none() || self.hovered_poi == Some(POIType::ResearchProbe))
                    && (self.hitbox.intersects_xy(pointer::world().xy()) 
                    || (self.hovered && self.pop_up.hovered())); 
                if !self.hovered && was_hovered { self.hovered_poi = None; }
            } else {
                self.hovered = false;
            }

            if self.hovered {
                self.hovered_poi = Some(POIType::ResearchProbe);
                // Pop up returns upgrade player clicks
                if let Some(upgrade) = self.pop_up.update(self.hitbox, &self.station, &mut self.avail_upgrades, &PROBE_UPGRADES, &self.resources) {
                    self.purchase_upgrade(&upgrade);
                    self.upgrade(&upgrade, event_manager);
                }
            }

        } else {
            if self.fin {
                event_manager.trigger(Event::ResetGame);
                self.jumping = false;
                self.fin = false;
            } else {
                self.jump(event_manager);
            }
        }

        if self.prestige_prog >= self.prestige_limit && self.prestige_index < 12 {
            self.prestige_earned += 1;
            self.prestige_index += 1;
            if self.prestige_index < 12 {
                self.prestige_limit = CostFormula::Exponential{factor: 1.5}.calculate_cost(vec![(Resources::Prestige, 80_000)], self.prestige_index)[0].1;
            } else {
                self.prestige_limit = 0;
            }
            self.prestige_prog = 0;
        }

        if self.resource_cloning && time::tick() % 600 == 0 {
            for r in self.resources.iter_mut() {
                if r.0 == Resources::Prestige || r.0 == Resources::Drones { continue; } 
                r.1 += (r.1 as f64 * 0.01).round() as u64;
            }
        }

        //self.dev_cheat(event_manager);
    }

    fn dev_cheat(&mut self, event_manager: &mut EventManager) {
        if gamepad::get(0).start.just_pressed() {
            if !self.station.unlocked {
                Upgrade::add_upgrade(&mut self.avail_upgrades, &PROBE_UPGRADES, 0, self.pop_up.panel);
                Upgrade::add_upgrade(&mut self.avail_upgrades, &PROBE_UPGRADES, 1, self.pop_up.panel);
                Upgrade::add_upgrade(&mut self.avail_upgrades, &PROBE_UPGRADES, 2, self.pop_up.panel);
                self.station.unlocked = true;
            }
            // event_manager.trigger(Event::DroneDepotUnlockable);
            // event_manager.trigger(Event::MinesUnlockable);
            // event_manager.trigger(Event::PowerPlantUnlockable);
            // event_manager.trigger(Event::LateGame);
            // event_manager.trigger(Event::FabricatorUnlockable);
            // event_manager.trigger(Event::AmpUnlockable);
            self.resources.clear();
            self.resources.push((Resources::Prestige, 60));
            self.resources.push((Resources::Research, 400000));
            self.resources.push((Resources::Drones, 400));
            self.resources.push((Resources::Metals, 400000));
            self.resources.push((Resources::Power, 400000));

            //event_manager.trigger(Event::Simulacrum);
        }

    }

    pub fn jump(&mut self, event_manager: &mut EventManager) {
        if !self.gate_aligned {
            //log!("aligning");
            self.target_pos = ((GATE_BOX.0 + GATE_BOX.2/2) as f32 - 8., (GATE_BOX.1 - 16) as f32);
            let dx = (self.target_pos.0 - self.pos.0) * 0.1;
            let dy = (self.target_pos.1 - self.pos.1) * 0.1;
            self.pos.0 += dx;
            self.pos.1 += dy;

            self.hitbox = self.hitbox.position(
                self.pos.0 as i32,
                self.pos.1 as i32
            );
            
            let distance_to_target = (
                self.target_pos.0 - self.hitbox.xy().0 as f32,
                self.target_pos.1 - self.hitbox.xy().1 as f32,
            );
            if distance_to_target.0.abs() < 2.0 && distance_to_target.1.abs() < 2.0 {
                self.gate_aligned = true;
                turbo::events::emit("happy_time", "");
            }
            self.dir += (180.0 - self.dir) * 0.1;
        } else {
            self.jump_timer += 1;

            // Rubber band up (decelerating motion)
            if self.jump_timer <= 50 {
                self.hitbox = self.hitbox.translate_y(-((50 - self.jump_timer) as f32 * 0.15) as i32); // Move down faster as time progresses
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

    pub fn reset_jump(&mut self) {
        self.jump_timer = 0;
        self.gate_aligned = false;
        self.jumping = false;
        self.hovered_poi = None;
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

    pub fn purchase_upgrade(&mut self, upgrade: &Upgrade) {
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
            Event::RecallDrone => {
                log!("recalled");
                self.collect((Resources::Drones, 1));
            }
            Event::Prestige => {
                self.jumping = true;
                if !self.station.unlocked {
                    Upgrade::add_upgrade(&mut self.avail_upgrades, &PROBE_UPGRADES, 0, self.pop_up.panel);
                    Upgrade::add_upgrade(&mut self.avail_upgrades, &PROBE_UPGRADES, 1, self.pop_up.panel);
                    Upgrade::add_upgrade(&mut self.avail_upgrades, &PROBE_UPGRADES, 2, self.pop_up.panel);
                }
            },
            Event::Simulacrum => {
                self.fin = true;
                self.jumping = true;
            }
            _ => {}
        }
    }

    pub fn scan(&mut self) {
        let pp = pointer::world().xy();
        let pos = (pp.0 as f32 + 5., pp.1 as f32 - 5.);
        self.scans.push(Scan::new((self.hitbox.x() as f32 + 8., self.hitbox.y() as f32 + 8.), pos));
    }

    pub fn draw(&self) {
        for scan in self.scans.iter() {
            scan.draw();
        }
        // rect!( 
        //     xy = (self.hitbox.xy().0, self.hitbox.xy().1),
        //     wh = (16, 16),
        // );
        if self.hovered {
            sprite!(
            "player_hovered", 
            xy = (self.hitbox.xy().0 - 1, self.hitbox.xy().1 - 1),
            rotation = self.dir,
        );
        }

        sprite!(
            "player", 
            xy = (self.hitbox.xy().0, self.hitbox.xy().1),
            rotation = self.dir,
        );
        //text!("{}", self.hovered_poi.is_some(); xy = (self.hitbox.xy().0 - 8, self.hitbox.xy().1 - 8),);

        if self.jumping && self.jump_timer >= 60 {
            let anim = animation::get("jump");
            anim.use_sprite("jump");
            anim.set_repeat(0);
            anim.set_fill_forwards(true);

            // Draw the scan effect
            sprite!(animation_key = "jump", xy = (GATE_BOX.0, GATE_BOX.1 - 64));
        }

        if !self.jumping {
            self.draw_boundary_fade();
        }

        PlayerDisplay::draw(&self.resources);
    }

    pub fn draw_boundary_fade(&self) {
        let player_pos = self.hitbox.xy();

        // Scene bounds
        let bounds = [(-8, -8), (648, -8), (648, 408), (-8, 408), (-8, -8)];

        let segment_len = 8; // Length of each faded segment
        let max_fade_dist = 80.0;

        for i in 0..bounds.len() - 1 {
            let (x0, y0) = bounds[i];
            let (x1, y1) = bounds[i + 1];

            let dx = x1 as f32 - x0 as f32;
            let dy = y1 as f32 - y0 as f32;
            let edge_len = (dx * dx + dy * dy).sqrt();
            let num_segments = (edge_len / segment_len as f32).ceil() as usize;

            for s in 0..num_segments {
                let t0 = s as f32 / num_segments as f32;
                let t1 = (s + 1) as f32 / num_segments as f32;

                let sx0 = x0 as f32 + dx * t0;
                let sy0 = y0 as f32 + dy * t0;
                let sx1 = x0 as f32 + dx * t1;
                let sy1 = y0 as f32 + dy * t1;

                // Distance from player to segment midpoint
                let mx = (sx0 + sx1) / 2.0;
                let my = (sy0 + sy1) / 2.0;
                let pdx = player_pos.0 as f32 - mx;
                let pdy = player_pos.1 as f32 - my;
                let dist = (pdx * pdx + pdy * pdy).sqrt();

                let alpha = (1.0 - (dist / max_fade_dist).clamp(0.0, 1.0)) * 255.0;
                let color = (0xffffff00) | (alpha.round() as u32);

                path!(
                    start = (sx0.round() as i32, sy0.round() as i32),
                    end = (sx1.round() as i32, sy1.round() as i32),
                    width = 1,
                    color = color,
                );
            }
        }
    }

    pub fn draw_ui(&self) { 
        // pop up
        if self.hovered {
            self.pop_up.draw(&self.station, &self.avail_upgrades);
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::new()
    }
}

impl POI for Player {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn update_drones(&mut self, player: &Player) -> u64 { 0 }

    fn get_station(&self) -> &Station {
        &self.station
    }

    fn manual_produce(&mut self) -> u64 {
        return 0;
    }

    fn upgrade(&mut self, upgrade: &Upgrade, event_manager: &mut EventManager) {
        self.purchased += 1;
        if self.purchased + 2 <= PROBE_UPGRADES.len() {
            Upgrade::add_upgrade(&mut self.avail_upgrades, &PROBE_UPGRADES, 2 + self.purchased, self.pop_up.panel);
        }
        

        if upgrade.name == "EXPERTISE" {
            self.expertise += 15.;
            event_manager.trigger(Event::BaseUpgrade { amount: 15. });
        } else if upgrade.name == "DRONE RECALL" {
            self.drone_recall = true;
            event_manager.trigger(Event::RecallUpgrade);
        } else if upgrade.name == "RESOURCEFUL" {
            self.resourceful = true;
        } else if upgrade.name == "INNOVATION" {
            self.innovation = true;
            event_manager.trigger(Event::InnovationUpgrade);
        } else if upgrade.name == "RESOURCE CLONING" {
            self.resource_cloning = true;
        }
    }
}

#[turbo::serialize]
pub struct PlayerDisplay {}
impl PlayerDisplay {
    pub fn draw(resources: &Vec<(Resources, u64)>) {
        let vp = screen();
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