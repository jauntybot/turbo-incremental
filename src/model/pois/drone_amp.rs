use std::vec;

use super::*;
pub const AMP_BOX: (i32, i32, i32, i32) = (396, 0, 64, 64);

#[turbo::serialize] 
pub struct DroneAmp {
    pub station: Station,

    pub hitbox: Bounds,
    pub pop_up: PopUp,
    pub hovered: bool,
    
    dir: f32,
    target_dir: f32,

    timer: u32,

    pub avail_upgrades: Vec<Upgrade>,
    pub active_amp: Option<Upgrade>,
}


impl DroneAmp {
    pub fn load() -> Self {
        let hitbox = Bounds::new(AMP_BOX.0, AMP_BOX.1, AMP_BOX.2, AMP_BOX.3);
        let pop_up =  PopUp::new("DRONE AMP".to_string(), DroneMode::Conduit);
        DroneAmp {
            station: Station::new(POIType::DroneAmp, DroneStats::new(20., 1.0, 90., 1.0)),

            hitbox,
            pop_up: pop_up.clone(),
            hovered: false,

            dir: 90.,
            target_dir: 90.,

            timer: 0,

            avail_upgrades: vec![],
            active_amp: None,
        }
    }

    pub fn update(&mut self, player: &mut Player, event_manager: &mut EventManager,
        planet: &mut Exoplanet, mines: &mut AsteroidMines, plant: &mut PowerPlant) {
        let p = pointer::world();
        let rp = p.xy();
        
        // Hover check
        if event_manager.dialogue.is_none() && self.station.unlockable {
            let was_hovered = self.hovered;
            self.hovered = 
                (player.hovered_poi.is_none() || player.hovered_poi == Some(POIType::DroneAmp))
                && self.hitbox.intersects_xy(rp) 
                || (self.hovered && (self.pop_up.inspecting() || self.pop_up.hovered())); 
            if !self.hovered && was_hovered { player.hovered_poi = None; }
        } else {
            self.hovered = false;
        }

        // Update pop up position and buttons, apply upgrades
        if self.hovered {
            player.hovered_poi = Some(POIType::DroneAmp);
            // Pop up returns upgrade player clicks
            if let Some(upgrade) = self.pop_up.update(self.hitbox, &self.station, &mut self.avail_upgrades, &AMP_UPGRADES, &player.resources) {
                self.upgrade(&upgrade, event_manager);
                player.purchase_upgrade(&upgrade);
            }
        }

        let lerp_speed = 0.1;
        self.dir += (self.target_dir - self.dir) * lerp_speed;

        let mut too_poor = false;
        if let Some(amp) = &mut self.active_amp {
            self.timer += 1;
            if let Some (power) = player.resources.iter().find(|r| r.0 == Resources::Power) {
                // Not enough power, shut off
                if power.1 < amp.cost[0].1 {
                    too_poor = true;
                // Siphon power
                } else if self.timer >= self.station.drone_stats.interval as u32 {
                    player.remove(amp.cost[0].clone());
                    self.station.collections.push(Collection::new_detail((self.hitbox.x() as f32, self.hitbox.y() as f32), amp.cost[0].clone(), false));
                    self.timer = 0;
                }
            }
            match amp.name.as_str() {
                "AMP SURVEY DRONES" => {
                    mines.station.drone_stats.amped = 1.0;
                    plant.station.drone_stats.amped = 1.0;
                    self.target_dir = 33.;
                    if (self.dir - self.target_dir).abs() < 1.0 {
                        planet.station.drone_stats.amped = 2.0;
                    }
                }
                "AMP MINING DRONES" => {
                    self.target_dir = 90.;
                    planet.station.drone_stats.amped = 1.0;
                    plant.station.drone_stats.amped = 1.0;
                    if (self.dir - self.target_dir).abs() < 1.0 {
                        mines.station.drone_stats.amped = 2.0;
                    }
                }
                "AMP CONDUIT DRONES" => {
                    self.target_dir = -60.;
                    planet.station.drone_stats.amped = 1.0;
                    mines.station.drone_stats.amped = 1.0;
                    if (self.dir - self.target_dir).abs() < 1.0 {
                        plant.station.drone_stats.amped = 2.0;
                    }
                }
                _ => {}
            }
        } else {
            planet.station.drone_stats.amped = 1.0;
            mines.station.drone_stats.amped = 1.0;
            plant.station.drone_stats.amped = 1.0;
            self.target_dir = 0.
        }
        if too_poor {
            self.active_amp = None;
            self.timer = 0;
        }

        self.station.update_collections();
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::AmpUnlockable => {
                self.station.unlockable = true;
                Upgrade::add_upgrade(&mut self.avail_upgrades, &AMP_UPGRADES, 0, self.pop_up.panel);
            }
            _ => {}
        }
    }

    pub fn draw(&self) {
        let mut bob_box = self.hitbox;
        if self.station.unlocked {
            let bob =  f32::sin(turbo::time::tick() as f32 / 25.0 + 10.0) * 1.5;
            bob_box = self.hitbox.translate_y(bob);
        }

        // Draw backside drones
        self.station.draw_back();
        
        if !self.station.unlocked { 
            sprite!("amp_locked_outline", xy = bob_box.xy());
        }
        // outline
        if self.hovered {
            let s = if self.active_amp.is_some() { "amp_hovered" } else { "amp_off_hovered" };
            sprite!(&s, xy = bob_box.xy(), rotation = self.dir);
        }

        // main GFX
        let s = if self.active_amp.is_some() { "amp" } else { "amp_off" };
        sprite!(&s, xy = bob_box.xy(), rotation = self.dir);

        if let Some(amp) = &self.active_amp {

            // Draw beam
            if (self.dir - self.target_dir).abs() < 1.0 {
                let count = if amp.name == "AMP MINING DRONES" { 8 } else { 5 };
                for l in 0..=1 {
                    let dir_rad = (self.dir + 90.).to_radians();
                    let offset_x = dir_rad.cos() * 32.0;
                    let offset_y = dir_rad.sin() * 32.0;
                    let mut beam_pos = (bob_box.x() as f32 + 16., bob_box.y() as f32 + 16.);
                    for i in 1..=count {
                        beam_pos.0 += offset_x;
                        beam_pos.1 += offset_y;
                        let sprite = format!("amp_beam_{}", l * 2 + (i + l) % 2);
                        sprite!(
                            &sprite,
                            xy = (beam_pos.0 as i32, beam_pos.1 as i32),
                            rotation = self.dir,
                            color = 0xffffff33,
                        );
                        if l == 1 {
                            sprite!(
                                "amp_beam_4",
                                xy = (beam_pos.0 as i32, beam_pos.1 as i32),
                                rotation = self.dir,
                                color = 0xffffffff,
                            );
                        }
                    }
                }
            }
        }

        if !self.station.unlocked { 
            sprite!("amp_locked", xy = bob_box.xy());
            text!("LOCKED", xy = bob_box.translate(-15,-8).center(), color = 0xffffffff);  
        }

        self.station.draw_front();
    }

    pub fn draw_ui(&self) { 
        
        if self.hovered {
            // pop up
            self.pop_up.draw(&self.station, &self.avail_upgrades);
        }
    }
}


impl POI for DroneAmp {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_station(&self) -> &Station {
        &self.station
    }
    
    fn update_drones(&mut self, player: &Player) -> u64 { 0 }

    fn upgrade(&mut self, upgrade: &Upgrade, event_manager: &mut EventManager) {
        if upgrade.name == "CONSTRUCT" {
            self.station.unlocked = true;
        } else if upgrade.name.starts_with("AMP") {
            // Loop through avail upgrades
            for u in self.avail_upgrades.iter_mut() {
                if let UpgradeType::Toggle { toggle } = &mut u.u_type {
                    toggle.value = false;
                    // If this avail upgrade is the upgrade passed
                    if u.name == upgrade.name {
                        // Check if there's an active upgrade
                        if let Some(active) = &mut self.active_amp {
                            if upgrade.name == active.name {
                                self.active_amp = None;
                            } else {
                                self.active_amp = Some(upgrade.clone());
                                toggle.value = true;
                            }
                        } else {
                            self.active_amp = Some(upgrade.clone());
                            toggle.value = true;
                        }
                    }
                }
            }
        }
    }
}