use super::*;


pub const PLANET_BOX: (i32, i32, i32, i32) = (274, 154, 98, 98);


#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Exoplanet {
    pub drones: Vec<Drone>,
    pub scanner_level: u32,
    collecting: bool,

    pub station: Station,

    pub hitbox: Bounds,
    pop_up: PopUp,
    hovered: bool,

    clicked_at: usize,
    collections: Vec<Collection>,
    scans: Vec<Scan>,
    collect_interval: usize,

    assigned: bool,

    avail_upgrades: Vec<Upgrade>,
}
impl Exoplanet {
    pub fn load() -> Self {
        let hitbox = Bounds::new(PLANET_BOX.0, PLANET_BOX.1, PLANET_BOX.2, PLANET_BOX.3);
        let pop_up =  PopUp::new("EXOPLANET".to_string(), Resources::Research);
        Exoplanet {
            drones: vec![],
            scanner_level: 1,
            collecting: false,

            station: Station {
                drone_base: 20.,
                drone_eff: 1.0,
                drone_speed: 800.,
            },

            hitbox,
            pop_up: pop_up.clone(),
            hovered: false,

            clicked_at: 0,
            collections: vec![],
            scans: vec![],
            collect_interval: 20,

            assigned: false,

            avail_upgrades: vec![EXOPLANET_UPGRADES[0].clone().init(pop_up.panel, 0)],
        }
    }

    pub fn update(&mut self, player: &mut Player, event_manager: &mut EventManager) {
        let p = pointer();
        let rp = p.xy();
        
        // Hover check
        if event_manager.dialogue.is_none() {
            self.hovered = self.hitbox.intersects_xy(rp) || (self.hovered && (self.pop_up.inspecting() || self.pop_up.hovered())); 
            if self.hovered { player.hovered_else = true; }
        } else {
            self.hovered = false;
        }

        // Update pop up position and buttons, apply upgrades
        if self.hovered {
            // Pop up returns upgrade player clicks
            if let Some(upgrade) = self.pop_up.update(self.hitbox, &self.station, &mut self.avail_upgrades, &EXOPLANET_UPGRADES, &player.resources) {
                self.upgrade(&upgrade, event_manager);
                player.upgrade(&upgrade);
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
            if self.collecting && tick() - self.clicked_at >= self.collect_interval {
                if !self.hitbox.intersects_xy(rp) || p.released() { self.collecting = false; }
                else {
                    self.clicked_at = tick();
                    produced.1 += self.manual_produce();
                    player.scan();
                }
            }
        }

        produced.1 += self.produce();
        
        // Update collection numbers
        self.collections.retain_mut(|collection| {
            collection.update();
            collection.is_active // Keep only active collections
        }); 
        
        player.collect(produced);
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::UnlockDroneDepot => {
                Upgrade::add_upgrade(&mut self.avail_upgrades, &EXOPLANET_UPGRADES, 1, self.pop_up.panel);
            }
            Event::UnlockPowerPlant => {
                Upgrade::add_upgrade(&mut self.avail_upgrades, &EXOPLANET_UPGRADES, 3, self.pop_up.panel);
            }
            _ => {}
        }
    }
    
    pub fn draw(&self) {
        let bob =  f32::sin((tick() as f32 + 20.0) / 40.0) * 1.5;
        let mut bob_box = self.hitbox.translate_y(bob);
        // Draw backside drones
        for drone in self.drones.iter() {
            if !drone.front {
                drone.draw();
            }
        }

        // outline
        // main GFX
        let o = (tick() as i32/20)%3;
        circ!(xy = bob_box.translate(-8 + o, -8 + o).xy(), diameter = 114 - o*2, color = 0x6c6c8066);
        let o = ((tick() as i32/20)+2)%3;
        circ!(xy = bob_box.translate(-29 + o, -29 + o).xy(), diameter = 156 - o*2, color = 0x38375366);
        
        if self.hovered {
            sprite!("exoplanet_hovered", xy = bob_box.translate(-1, -1).xy());
        }
        sprite!("exoplanet", xy = bob_box.xy());
        // Draw drones
        for drone in self.drones.iter() {
            drone.draw_scan();
            if drone.front {
                drone.draw();
            }
        }
        
        // Draw collection numbers
        for collection in self.collections.iter() {
            collection.draw();
        }
        
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

    fn manual_produce(&mut self) -> u64 {
        let pp = pointer().xy();
        let pos = (pp.0 as f32 + 5., pp.1 as f32 - 5.);
        self.collections.push(Collection::new(pos, (Resources::Research, self.scanner_level as u64)));
        return self.scanner_level as u64;
    } 

    fn produce(&mut self) -> u64 {
        let mut produced = 0;
        for drone in self.drones.iter_mut() {
            if drone.survey(&self.station) {
                let amount =  (self.station.drone_eff * self.station.drone_base) as u64;
                produced += amount;
                self.collections.push(Collection::new(drone.pos, (Resources::Research, amount)));
            }
        }
        produced
    }

    fn upgrade(&mut self, upgrade: &Upgrade, event_manager: &mut EventManager) {
        if upgrade.name.starts_with("FIELD") {
            self.scanner_level += 1;
            if self.scanner_level == 3 {
                event_manager.trigger(Event::DroneDepotUnlockable);
            }
        } else if upgrade.name.starts_with("DEPLOY") {
            let xy = self.hitbox.translate(self.hitbox.w()/2,self.hitbox.h()/2).xy();
            self.drones.push(Drone::new(DroneMode::Survey, self.station.drone_eff as u32, self.station.drone_speed as u32, xy));
            self.pop_up.drones += 1;
            if !self.assigned {
                event_manager.trigger(Event::MinesUnlockable);
                self.assigned = true;
            }
        } else if upgrade.name.starts_with("UNASSIGN") {
            if self.drones.len() == 0 { return; }
            self.drones.remove(0);
            self.pop_up.drones -= 1;
        } else if upgrade.name.starts_with("ADV.") {
            self.station.drone_eff += 0.8;
            for drone in self.drones.iter_mut() {
                drone.level += 1;
            }
        } else if upgrade.name.starts_with("BIO") {
            self.station.drone_speed *= 0.95;
            for drone in self.drones.iter_mut() {
                drone.speed += 1;
            }
        }
    }
}