use super::*;

pub const GATE_BOX: (i32, i32, i32, i32) = (288, 432, 64, 64);

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Jumpgate {
    pub drones: Vec<Drone>,

    station: Station,

    pub drone_level: u32,
    pub drone_speed: u32,

    pub hitbox: Bounds,
    pub pop_up: PopUp,
    pub hovered: bool,
    
    pub clicked_at: usize,
    pub collections: Vec<Collection>,
    pub collect_interval: usize,

    earn: u64,
    limit: u64,
    prog: u64,
    
    pub unlockable: bool,
    pub unlocked: bool,
    
    pub avail_upgrades: Vec<Upgrade>,
}

impl Jumpgate {
    pub fn load() -> Self {
        let hitbox = Bounds::new(GATE_BOX.0, GATE_BOX.1, GATE_BOX.2, GATE_BOX.3);
        let pop_up =  PopUp::new_fab("JUMPGATE".to_string(), Resources::Prestige);
        Jumpgate {
            drones: vec![],
          
            station: Station {
                drone_base: 20.,
                drone_eff: 1.0,
                drone_speed: 600.,
            },

            drone_level: 0,
            drone_speed: 0,

            hitbox,
            pop_up: pop_up.clone(),
            hovered: false,

            clicked_at: 0,
            collections: vec![],
            collect_interval: 20,

            earn: 0,
            limit: 0,
            prog: 0,

            unlockable: false,
            unlocked: false,

            avail_upgrades: vec![],
        }
    }

    pub fn update(&mut self, player: &mut Player, event_manager: &mut EventManager) {
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
            if self.unlocked {
                if let Some(upgrade) = self.pop_up.update_fabricator(self.hitbox, &self.station, &mut self.avail_upgrades, &GATE_UPGRADES, &player.resources) {
                    self.upgrade(&upgrade, event_manager);
                    player.upgrade(&upgrade);
                }
            } else {
                if let Some(upgrade) = self.pop_up.update(self.hitbox, &self.station, &mut self.avail_upgrades, &GATE_UPGRADES, &player.resources) {
                    self.upgrade(&upgrade, event_manager);
                    player.upgrade(&upgrade);
                }
            }
        }

        // Produce Resources
        let mut produced = (Resources::Research, 0);
        produced.1 += self.produce();
        
        // Update collection numbers
        self.collections.retain_mut(|collection| {
            collection.update();
            collection.is_active // Keep only active collections
        }); 
        
        player.collect(produced);

        self.limit = player.prestige_limit;
        self.earn = player.prestige_earned;
        self.prog = player.prestige_prog;
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::LateGame => {
                self.unlockable = true;
                Upgrade::add_upgrade(&mut self.avail_upgrades, &GATE_UPGRADES, 0, self.pop_up.panel);
            }
            _ => {}
        }
    }

    pub fn draw(&self) {
        let mut bob_box = self.hitbox;
        if self.unlocked {
            let bob =  f32::sin(tick() as f32 / 25.0 + 10.0) * 1.5;
            bob_box = self.hitbox.translate_y(bob);
        }

        // Draw backside drones
        for drone in self.drones.iter() {
            if !drone.front {
                drone.draw();
            }
        }
        
        if !self.unlocked { 
            sprite!("gate_locked_outline", xy = bob_box.xy());
        }
        // outline
        if self.hovered {
            sprite!("gate_hovered", xy = bob_box.xy());
        }

        // main GFX
        sprite!("gate", xy = bob_box.xy());

        for drone in self.drones.iter() {
            if drone.front {
                drone.draw();
            }
        }

        if !self.unlocked { 
            sprite!("gate_locked", xy = bob_box.xy());
            text!("LOCKED", xy = bob_box.translate(-15,-4).center(), color = 0xffffffff);  
        }
    }

    pub fn draw_ui(&self) { 
        
        if self.hovered {
            // pop up
            if self.unlocked {
                self.pop_up.draw_jumpgate(&self.station, &self.avail_upgrades, self.earn, self.prog, self.limit);
            } else {
                self.pop_up.draw(&self.station, &self.avail_upgrades);
            }
        }

        // Draw collection numbers
        for collection in self.collections.iter() {
            collection.draw();
        }
    }

}

impl POI for Jumpgate {
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
        return 0;
    }

    fn upgrade(&mut self, upgrade: &Upgrade, event_manager: &mut EventManager) {
        if upgrade.name == "CONSTRUCT" {
            self.unlocked = true;
        } else if upgrade.name.starts_with("JUMP") {
            event_manager.trigger(Event::Prestige);
        }
    }
}