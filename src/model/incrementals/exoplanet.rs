use super::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Exoplanet {
    pub drones: Vec<Drone>,
    pub scanner_level: u32,
    pub drone_level: u32,

    pub hitbox: Bounds,
    pop_up: PopUp,
    hovered: bool,

    clicked_at: usize,
    collections: Vec<Collection>,
    collect_interval: usize,

    avail_upgrades: Vec<Upgrade>,
}
impl Exoplanet {
    pub fn load() -> Self {
        let hitbox = Bounds::new(272, 272, 96, 96);
        let pop_up =  PopUp::new("EXOPLANET".to_string());
        Exoplanet {
            drones: vec![],
            scanner_level: 1,
            drone_level: 1,

            hitbox,
            pop_up,
            hovered: false,

            clicked_at: 0,
            collections: vec![],
            collect_interval: 25,

            avail_upgrades: vec![EXOPLANET_UPGRADES[0].clone()],
        }
    }

    pub fn update(&mut self, player: &mut Player, event_manager: &mut EventManager) {
        // Update pop up position and buttons, apply upgrades
        if self.hovered {
            if let Some(upgrade) = self.pop_up.update(self.hitbox, &mut self.avail_upgrades, &EXOPLANET_UPGRADES, &player.resources) {
                self.upgrade(&upgrade, event_manager);
                player.upgrade(&upgrade, self);
            }
        }

        // Hover check
        let p = pointer();
        let rp = p.relative_position();
        self.hovered = self.hitbox.intersects_xy(rp) || (self.hovered && self.pop_up.hovered()); 
        
        // Produce Resources
        let mut produced = (Resources::Research, 0);
        
        // Initial click
        if self.hitbox.intersects_xy(rp) && p.just_pressed() && tick() - self.clicked_at >= self.collect_interval {
            self.clicked_at = tick();
        }
        // Manually produce resources every 30 ticks
        if tick() - self.clicked_at >= self.collect_interval {
            if self.hitbox.intersects_xy(rp) && p.pressed() {
                produced.1 += self.manual_produce();
            }
            self.clicked_at = tick();
        }

        produced.1 +=  self.produce();
        
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
                self.avail_upgrades.push(EXOPLANET_UPGRADES[4].clone());
            }
            _ => {}
        }
    }
    
    pub fn draw(&self) {
        // Draw backside drones
        for drone in self.drones.iter() {
            if !drone.front {
                drone.draw();
            }
        }

        // outline
        if self.hovered {
            circ!(x = self.hitbox.x() - 1, y = self.hitbox.y() - 1, diameter = 98, color = 0xffffffff); 
        }
        // main GFX
        circ!(xy = self.hitbox.xy(), diameter = self.hitbox.w(), color = 0x5b6ee1ff);
        
        // Draw drones
        for drone in self.drones.iter() {
            if drone.front {
                drone.draw();
            }
        }

        // Pop up
        if self.hovered {
            self.pop_up.draw(&self.avail_upgrades);
        }

        // Draw collection numbers
        for collection in self.collections.iter() {
            collection.draw();
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
    
    fn manual_produce(&mut self) -> u32 {
        let pp = pointer().relative_position();
        let pos = (pp.0 as f32 + 5., pp.1 as f32 - 5.);
        self.collections.push(Collection::new(pos, (Resources::Research, self.scanner_level)));
        return self.scanner_level;
    } 

    fn produce(&mut self) -> u32 {
        let mut produced = 0;
        for drone in self.drones.iter_mut() {
            if drone.update() {
                let amount =  self.drone_level * 20;
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
            self.drones.push(Drone::new(DroneMode::Survey, self.drone_level));
            self.pop_up.drones += 1;
            if self.drones.len() == 1 {
                event_manager.trigger(Event::MinesUnlockable);
                self.avail_upgrades.push(EXOPLANET_UPGRADES[5].clone());
            }
        }
    }
}