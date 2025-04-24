use super::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct AsteroidMines {
    pub drones: Vec<Drone>,
    pub drone_level: u32,

    pub unlockable: bool,
    unlocked: bool,

    hitbox: Bounds,
    pop_up: PopUp,
    hovered: bool,

    clicked_at: usize,
    collections: Vec<Collection>,
    collect_interval: usize,

    avail_upgrades: Vec<Upgrade>,
}

impl AsteroidMines {
    pub fn load() -> Self {
        let hitbox = Bounds::new(128, 128, 64, 64);
        let pop_up =  PopUp::new("ASTEROID MINES".to_string());
        AsteroidMines {
            drones: vec![],
            drone_level: 1,

            unlockable: false,
            unlocked: false,

            hitbox,
            pop_up,
            hovered: false,

            clicked_at: 0,
            collections: vec![],
            collect_interval: 30,

            avail_upgrades: vec![],
        }
    }

    pub fn update(&mut self, player: &mut Player, event_manager: &mut EventManager) {
        // Update pop up position and buttons, apply upgrades
        if self.hovered {
            if let Some(upgrade) = self.pop_up.update(self.hitbox, &mut self.avail_upgrades, &MINES_UPGRADES, &player.resources) {
                self.upgrade(&upgrade, event_manager);
                player.upgrade(&upgrade, self);
            }
        }

        // Produce Resources
        let mut produced = (Resources::Metals, 0);

        let p = pointer();
        let rp = p.relative_position();
        self.hovered = self.hitbox.intersects_xy(rp) || (self.hovered && self.pop_up.hovered()); 
        
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

        produced.1 += self.produce();
        //log!("Produced: {:?}", self.produce());
        
        // Update collection numbers
        self.collections.retain_mut(|collection| {
            collection.update();
            collection.is_active // Keep only active collections
        }); 

        if self.unlocked {
            player.collect(produced);
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::MinesUnlockable => {
                self.unlockable = true;
                self.avail_upgrades.push(MINES_UPGRADES[0].clone());
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
            rect!(
                x = self.hitbox.x() - 1, 
                y = self.hitbox.y() - 1, 
                wh = (self.hitbox.w() + 2, self.hitbox.w() + 2), 
                border_radius = 4,
                color = 0xffffffff
            ); 
        }

        // main GFX
        rect!(
            xy = self.hitbox.xy(), 
            wh = self.hitbox.wh(), 
            border_radius = 4,
            color = 0xbab32dff
        );

        for drone in self.drones.iter() {
            if drone.front {
                drone.draw();
            }
        }

        if !self.unlocked { 
            text!("LOCKED", xy = (self.hitbox.x() + 4, self.hitbox.y() + 4), color = 0xffffffff);       
        }

        if self.hovered {
            // pop up
            self.pop_up.draw(&self.avail_upgrades);
        }

        // Draw collection numbers
        for collection in self.collections.iter() {
            collection.draw();
        }
        
    }

}

impl POI for AsteroidMines {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn produce(&mut self) -> u32 {
        let mut produced = 0;
        for drone in self.drones.iter_mut() {
            if drone.update() {
                let amount =  self.drone_level * 2;
                produced += amount;
                self.collections.push(Collection::new(drone.pos, (Resources::Metals, amount)));
            }
        }
        produced
    }

    fn upgrade(&mut self, upgrade: &Upgrade, event_manager: &mut EventManager) {
        if upgrade.name == "CONSTRUCT" {
            self.unlocked = true;
        } else if upgrade.name.starts_with("DEPLOY") {
            self.drones.push(Drone::new(DroneMode::Mining, self.drone_level));
            self.pop_up.drones += 1;
            if self.drones.len() == 1 {
                event_manager.trigger(Event::PowerPlantUnlockable);
            }
        }
    }
}