use super::*;

pub const PLANT_BOX: (i32, i32, i32, i32) = (576, 64, 64, 74);

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct PowerPlant {
    pub drones: Vec<Drone>,

    station: Station,

    pub drone_level: u32,
    pub drone_speed: u32,

    pub unlockable: bool,
    unlocked: bool,

    pub hitbox: Bounds,
    pop_up: PopUp,
    hovered: bool,

    clicked_at: usize,
    collections: Vec<Collection>,
    collect_interval: usize,

    avail_upgrades: Vec<Upgrade>,
}
impl PowerPlant {
    pub fn load() -> Self {
        let hitbox = Bounds::new(PLANT_BOX.0, PLANT_BOX.1, PLANT_BOX.2, PLANT_BOX.3);
        let pop_up =  PopUp::new("POWER PLANT".to_string(), Resources::Power);
        PowerPlant {
            drones: vec![],
            station: Station {
                drone_base: 20.,
                drone_eff: 1.0,
                drone_speed: 600.,
            },
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

    pub fn update(&mut self, player: &mut Player, event_manager: &mut EventManager, nebula: &mut NebulaStorm) {
        // Update pop up position and buttons, apply upgrades
        if self.hovered {
            if let Some(upgrade) = self.pop_up.update(self.hitbox, &self.station, &mut self.avail_upgrades, &POWER_UPGRADES, &player.resources) {
                self.upgrade(&upgrade, event_manager);
                player.upgrade(&upgrade);
            }
        }

        // Hover check
        let p = pointer();
        let rp = p.xy();
        if event_manager.dialogue.is_none() {
            self.hovered = self.hitbox.intersects_xy(rp) || (self.hovered && self.pop_up.hovered()); 
        } else {
            self.hovered = false;
        }
        // Produce Resources
        let mut produced = (Resources::Power, 0);
        
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

        for drone in self.drones.iter_mut() {
            if drone.conduit(nebula) {
                let amount =  ((1.0 + self.drone_level as f32 * 0.9).round() * 12.) as u64;
                produced.1 += amount;
                self.collections.push(
                    Collection::new(
                        nebula.bolts[nebula.bolts.len() - 1].segments[nebula.bolts[nebula.bolts.len() - 1].segments.len() - 1].end,
                        (Resources::Power, amount)
                    )
                );
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
            Event::PowerPlantUnlockable => {
                self.unlockable = true;
                Upgrade::add_upgrade(&mut self.avail_upgrades, &POWER_UPGRADES, 0, self.pop_up.panel);
            }
            _ => {}
        }
    }

    pub fn draw(&self) {
        let mut bob_box = self.hitbox;
        if self.unlocked {
            let bob =  f32::sin(tick() as f32 / 20.0 + 5.0) * 1.5;
            bob_box = self.hitbox.translate_y(bob);
        }


        // Draw drones
        for drone in self.drones.iter() {
            if !drone.front {
                drone.draw();
            }
        }
        
        if !self.unlocked { 
            sprite!("plant_locked_outline", xy = bob_box.xy());
        }
        // outline
        if self.hovered {
            sprite!("plant_hovered", xy = bob_box.xy());
        }
        // main GFX
        sprite!("plant", xy = bob_box.xy());

        // Draw drones
        for drone in self.drones.iter() {
            if drone.front {
                drone.draw();
            }
        }

        if !self.unlocked { 
            sprite!("plant_locked", xy = bob_box.xy());
            text!("LOCKED", xy = bob_box.translate(-15, 17).center(), color = 0xffffffff);   
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


impl POI for PowerPlant {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_station(&self) -> &Station {
        &self.station
    }

    fn produce(&mut self) -> u64 {
        let mut produced = 0;

        produced
    }

    fn upgrade(&mut self, upgrade: &Upgrade, event_manager: &mut EventManager) {
        if upgrade.name == "CONSTRUCT" {
            self.unlocked = true;
            event_manager.trigger(Event::UnlockPowerPlant);
        } else if upgrade.name.starts_with("DEPLOY") {
            let xy = self.hitbox.translate(self.hitbox.w()/2,self.hitbox.h()/2).xy();
            self.drones.push(Drone::new(DroneMode::Conduit, self.drone_level, self.drone_speed, xy));
            self.pop_up.drones += 1;
            if self.drones.len() == 1 {
                event_manager.trigger(Event::LateGame);
            }
        }
        else if upgrade.name.starts_with("REFLECT") {
            self.drone_level += 1;
            for drone in self.drones.iter_mut() {
                drone.level += 1;
            }
        }
        else if upgrade.name.starts_with("ARC") {
            self.drone_speed += 1;
            for drone in self.drones.iter_mut() {
                drone.speed += 1;
            }
        }
    }
}