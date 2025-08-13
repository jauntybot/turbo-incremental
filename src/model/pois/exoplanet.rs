use super::*;


pub const PLANET_BOX: (i32, i32, i32, i32) = (271, 151, 98, 98);


#[turbo::serialize]
pub struct Exoplanet {
    pub scanner_level: u32,
    collecting: bool,

    pub station: Station,

    pub hitbox: Bounds,
    pop_up: PopUp,
    hovered: bool,

    clicked_at: usize,
    scans: Vec<Scan>,
    collect_interval: usize,

    assigned: bool,

    avail_upgrades: Vec<Upgrade>,
}
impl Exoplanet {
    pub fn load(player: &Player) -> Self {
        let hitbox = Bounds::new(PLANET_BOX.0, PLANET_BOX.1, PLANET_BOX.2, PLANET_BOX.3);
        let pop_up =  PopUp::new("EXOPLANET".to_string(), DroneMode::Survey);
        let mut station = Station::new_drone(POIType::Exoplanet, &mut DroneStats::new(20., 1.0, 400., 1.0), player);
        station.unlockable = true;
        station.unlocked = true;
        Exoplanet {
            scanner_level: 1,
            collecting: false,

            station, 
            
            hitbox,
            pop_up: pop_up.clone(),
            hovered: false,

            clicked_at: 0,
            scans: vec![],
            collect_interval: 20,

            assigned: false,

            avail_upgrades: vec![EXOPLANET_UPGRADES[0].clone().init(pop_up.panel, 0)],
        }
    }

    pub fn update(&mut self, player: &mut Player, event_manager: &mut EventManager) {
        let p = pointer::world();
        let rp = p.xy();

        // Hover check
        if event_manager.dialogue.is_none() {
            let was_hovered = self.hovered;
            self.hovered = 
                (player.hovered_poi.is_none() || player.hovered_poi == Some(POIType::Exoplanet))
                && self.hitbox.intersects_xy(rp) 
                || (self.hovered && (self.pop_up.inspecting() || self.pop_up.hovered())); 
            if !self.hovered && was_hovered { player.hovered_poi = None; }
        } else {
            self.hovered = false;
        }

        // Update pop up position and buttons, apply upgrades
        if self.hovered {
            player.hovered_poi = Some(POIType::Exoplanet);
            // Pop up returns upgrade player clicks
            if let Some(upgrade) = self.pop_up.update(self.hitbox, &self.station, &mut self.avail_upgrades, &EXOPLANET_UPGRADES, &player.resources) {
                self.upgrade(&upgrade, event_manager);
                player.purchase_upgrade(&upgrade);
            }
        }

        // Produce Resources
        let mut produced = (Resources::Research, 0);
        
        if event_manager.dialogue.is_none() {
            // Initial click
            if self.hovered && self.hitbox.intersects_xy(rp) 
                && p.just_pressed() && !self.collecting {
                self.collecting = true;
            }
            // Manually produce resources every 30 ticks
            if self.collecting && turbo::time::tick() - self.clicked_at >= self.collect_interval {
                if !self.hitbox.intersects_xy(rp) || p.released() { self.collecting = false; }
                else {
                    self.clicked_at = turbo::time::tick();
                    produced.1 += self.manual_produce();
                    player.scan();
                }
            }
        }

        produced.1 += self.update_drones(&player);
        
        // Update collection numbers
        self.station.update_collections();
        
        player.collect(produced);
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::UnlockDroneDepot => {
                let index = if self.avail_upgrades.len() > 0 && self.avail_upgrades[0].name.starts_with("FIELD") { 1 } else { 0 };
                Upgrade::add_upgrade(&mut self.avail_upgrades, &EXOPLANET_UPGRADES, 1, self.pop_up.panel);
                if self.station.innovation {
                    let upgrade = EXOPLANET_UPGRADES[4].clone().init(self.pop_up.panel, 1);
                    self.avail_upgrades.insert(index + 1, upgrade);
                }
            }
            Event::UnlockPowerPlant => {
                Upgrade::add_upgrade(&mut self.avail_upgrades, &EXOPLANET_UPGRADES, 3, self.pop_up.panel);
            }
            Event::AdvDronesResearched => {
                self.station.drone_stats.mult += 1.;
            }
            Event::BaseUpgrade { amount } => {
                self.station.drone_stats.base += amount;
            }
            Event::RecallUpgrade => {
                self.station.drone_stats.recall = true;
            }
            Event::InnovationUpgrade => {
                self.station.innovation = true;
                if self.avail_upgrades.iter().any(|u| u.name.starts_with("DEPLOY")) {
                    let upgrade = EXOPLANET_UPGRADES[4].clone().init(self.pop_up.panel, 1);
                    let index = if self.avail_upgrades[0].name.starts_with("FIELD") && self.avail_upgrades.len() > 1 { 2 } else { 1 };
                    self.avail_upgrades.insert(index, upgrade);
                }
            }
            _ => {}
        }
    }
    
    pub fn draw(&self) {
        let bob =  f32::sin((turbo::time::tick() as f32 + 20.0) / 40.0) * 1.5;
        let bob_box = self.hitbox.translate_y(bob);
        
        if self.station.drone_stats.amped > 1.0 {
            sprite!(
                "amp_aura_2", 
                xy = (bob_box.x() - 11, bob_box.y() - 11),
                rotation = -(time::tick() as f32 / 10. % 360.),
                color = 0xffffffff,
            );
            sprite!(
                "amp_aura_3", 
                xy = (bob_box.x() - 11, bob_box.y() - 11),
                rotation = time::tick() as f32 / 10. % 360.,
                color = 0xffffffff,
            );
        }

        // Draw backside drones
        self.station.draw_back();
        
        // aura
        let o = (turbo::time::tick() as i32/20)%3;
        circ!(xy = bob_box.translate(-8 + o, -8 + o).xy(), diameter = 114 - o*2, color = 0x6c6c8066);
        let o = ((turbo::time::tick() as i32/20)+2)%3;
        circ!(xy = bob_box.translate(-29 + o, -29 + o).xy(), diameter = 156 - o*2, color = 0x38375366);
        
        // main GFX
        if self.hovered {
            sprite!("exoplanet_hovered", xy = bob_box.translate(-1, -1).xy());
        }
        sprite!("exoplanet", xy = bob_box.xy());
        
        // Draw drones        
        self.station.draw_front();
    }


    pub fn draw_ui(&self) {
        // pop up
        if self.hovered {
            self.pop_up.draw(&self.station, &self.avail_upgrades);
        }
    }

}

impl POI for Exoplanet {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    
    fn get_station(&self) -> &Station {
        &self.station
    }


    fn update_drones(&mut self, player: &Player) -> u64 {
        let mut produced = 0;
        let drone_stats = self.station.drone_stats.clone();
        let mut drones = std::mem::take(&mut self.station.drones);

        for drone in drones.iter_mut() {
            if drone.update(&drone_stats, self as &mut dyn POI) {
                let amount = drone_stats.produce();
                produced += amount;
                self.station.new_collect(drone.pos, (Resources::Research, amount));
            }
        }

        self.station.drones = drones;
        produced
    }

    fn manual_produce(&mut self) -> u64 {
        let pp = pointer::world().xy();
        let pos = (pp.0 as f32 + 5., pp.1 as f32 - 5.);
        self.station.collections.push(Collection::new(pos, (Resources::Research, self.scanner_level as u64)));
        return self.scanner_level as u64;
    }

    fn upgrade(&mut self, upgrade: &Upgrade, event_manager: &mut EventManager) {
        if upgrade.name.starts_with("FIELD") {
            self.scanner_level += 1;
            if self.scanner_level == 3 {
                event_manager.trigger(Event::DroneDepotUnlockable);
            }
        } else if upgrade.name.starts_with("DEPLOY") {
            let xy = self.hitbox.translate(self.hitbox.w()/2,self.hitbox.h()/2).xy();
            self.station.deploy_drone(DroneMode::Survey, xy);
            self.pop_up.drones += 1;
            if !self.assigned {
                event_manager.trigger(Event::MinesUnlockable);
                self.assigned = true;
            }
        } else if upgrade.name == ("UNASSIGN") {
            if self.station.drones.len() == 0 { return; }
            self.station.drones.remove(0);
            self.pop_up.drones -= 1;
            event_manager.trigger(Event::RecallDrone);
        } else if upgrade.name.starts_with("ADV.") {
            self.station.drone_stats.eff += 0.85;
        } else if upgrade.name.starts_with("BIO") {
            self.station.drone_stats.speed *= 0.95;
        } else if upgrade.name.starts_with("DATA") {
            self.station.drone_stats.base += 3.;
        }
    }
}