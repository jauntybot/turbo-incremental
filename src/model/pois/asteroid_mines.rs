use super::*;

pub const MINES_BOX: (i32, i32, i32, i32) = (128, 0, 64, 64);

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct AsteroidMines {
    pub drones: Vec<Drone>,
    pub drone_level: u32,
    pub drone_speed: u32,

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
        let hitbox = Bounds::new(128, 0, 64, 64);
        let pop_up =  PopUp::new("ASTEROID MINES".to_string());
        AsteroidMines {
            drones: vec![],
            drone_level: 0,
            drone_speed: 0,

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

    pub fn update(&mut self, player: &mut Player, event_manager: &mut EventManager, field: &mut AsteroidField,) {
        let p = pointer();
        let rp = p.xy();
        
        // Hover check
        if event_manager.dialogue.is_none() {
            self.hovered = self.hitbox.intersects_xy(rp) || (self.hovered && self.pop_up.hovered()); 
        } else {
            self.hovered = false;
        }
        
        // Update pop up position and buttons, apply upgrades
        if self.hovered {
            // Pop up returns upgrade player clicks
            if let Some(upgrade) = self.pop_up.update(self.hitbox, &mut self.avail_upgrades, &MINES_UPGRADES, &player.resources) {
                self.upgrade(&upgrade, event_manager);
                player.upgrade(&upgrade, self);
            }
        }
        
        // Produce Resources
        let mut produced = (Resources::Metals, 0);
        // Initial click
        if self.hovered && p.just_pressed() && tick() - self.clicked_at >= self.collect_interval {
            self.clicked_at = tick();
        }
        // Manually produce resources every 30 ticks
        if tick() - self.clicked_at >= self.collect_interval {
            if self.hitbox.intersects_xy(rp) && p.pressed() {
                produced.1 += self.manual_produce();
            }
            self.clicked_at = tick();
        }

        // Produce based on drone update
        for drone in self.drones.iter_mut() {
            if drone.update_mining(field) {
                let amount =  ((1. + self.drone_level as f32 * 1.2) * 15.).round() as u64;
                produced.1 += amount;
                self.collections.push(Collection::new(drone.pos, (Resources::Metals, amount)));
            }
        }

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
                Upgrade::add_upgrade(&mut self.avail_upgrades, &MINES_UPGRADES, 0, self.pop_up.panel);
            }
            Event::UnlockPowerPlant => {
                Upgrade::add_upgrade(&mut self.avail_upgrades, &MINES_UPGRADES, 3, self.pop_up.panel);
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

        // if self.hovered {
        //     rect!(
        //         x = self.hitbox.x() - 1, 
        //         y = self.hitbox.y() - 1, 
        //         wh = (self.hitbox.w() + 2, self.hitbox.w() + 2), 
        //         border_radius = 4,
        //         color = 0xffffffff
        //     ); 
        // }
        if !self.unlocked { 
            sprite!("mines_locked_outline", xy = self.hitbox.xy());
        }
        // outline
        if self.hovered {
            sprite!("mines_hovered", xy = self.hitbox.xy());
        }
        // main GFX
        sprite!("mines", xy = self.hitbox.xy());

        for drone in self.drones.iter() {
            if drone.front {
                drone.draw();
            }
        }

        if !self.unlocked { 
            sprite!("mines_locked", xy = self.hitbox.xy());
            text!("LOCKED", xy = self.hitbox.translate(-16,2).center(), color = 0xffffffff);       
        }

        // Draw collection numbers
        for collection in self.collections.iter() {
            collection.draw();
        }
        
    }

    pub fn draw_ui(&self) {
        // pop up
        if self.hovered {
            self.pop_up.draw(&self.avail_upgrades);
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

    fn produce(&mut self) -> u64 {
        0
    }

    fn upgrade(&mut self, upgrade: &Upgrade, event_manager: &mut EventManager) {
        if upgrade.name == "CONSTRUCT" {
            self.unlocked = true;
        } else if upgrade.name.starts_with("DEPLOY") {
            let xy = self.hitbox.translate(self.hitbox.w()/2,self.hitbox.h()/2).xy();
            self.drones.push(Drone::new(DroneMode::Mining, self.drone_level, self.drone_speed, xy));
            self.pop_up.drones += 1;
            if self.drones.len() == 1 {
                event_manager.trigger(Event::PowerPlantUnlockable);
            }
        } else if upgrade.name.starts_with("DRILL") {
            self.drone_level += 1;
            for drone in self.drones.iter_mut() {
                drone.level += 1;
            }   
        } else if upgrade.name.starts_with("ADV.") {
            self.drone_speed += 1;
            for drone in self.drones.iter_mut() {
                drone.speed += 1;
            }   
        }
    }
}