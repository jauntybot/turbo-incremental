use super::*;

pub static COMPLEX_BOX: (i32, i32, i32, i32) = (448, 352, 64, 64);

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ResearchComplex {
    pub drones: Vec<Drone>,
    pub drone_level: u32,
    pub drone_speed: u32,

    pub hitbox: Bounds,
    pub pop_up: PopUp,
    pub hovered: bool,
    
    pub clicked_at: usize,
    pub collections: Vec<Collection>,
    pub collect_interval: usize,
    
    pub unlockable: bool,
    pub unlocked: bool,
    
    pub avail_upgrades: Vec<Upgrade>,
}

impl ResearchComplex {
    pub fn load() -> Self {
        let hitbox = Bounds::new(COMPLEX_BOX.0, COMPLEX_BOX.1, COMPLEX_BOX.2, COMPLEX_BOX.3);   
        let pop_up =  PopUp::new("RESEARCH COMPLEX".to_string());
        ResearchComplex { 
            drones: vec![],
            drone_level: 0,
            drone_speed: 0,

            hitbox,
            pop_up: pop_up.clone(),
            hovered: false,

            clicked_at: 0,
            collections: vec![],
            collect_interval: 20,

            unlockable: false,
            unlocked: false,

            avail_upgrades: vec![],
        }
    }

    pub fn update(&mut self, player: &mut Player, event_manager: &mut EventManager) {
        let p = pointer();
        let rp = p.relative_position();
        
        // Hover check
        if event_manager.dialogue.is_none() {
            self.hovered = self.hitbox.intersects_xy(rp) || (self.hovered && self.pop_up.hovered()); 
        } else {
            self.hovered = false;
        }

        // Update pop up position and buttons, apply upgrades
        if self.hovered {
            // Pop up returns upgrade player clicks
            if let Some(upgrade) = self.pop_up.update(self.hitbox, &mut self.avail_upgrades, &COMPLEX_UPGRADES, &player.resources) {
                self.upgrade(&upgrade, event_manager);
                player.upgrade(&upgrade, self);
            }
        }

        // Produce Resources
        let mut produced = (Resources::Research, 0);
        
        // Initial click
        if self.hovered && self.hitbox.intersects_xy(rp) 
            && p.just_pressed() && tick() - self.clicked_at >= self.collect_interval {
            self.clicked_at = tick();
            produced.1 += self.manual_produce();
            player.scan();
        }
        // Manually produce resources every 30 ticks
        if tick() - self.clicked_at >= self.collect_interval {
            if self.hitbox.intersects_xy(rp) && p.pressed() {
                produced.1 += self.manual_produce();
                player.scan();
            }
            self.clicked_at = tick();
        }
        //if p.released() { self.clicked_at = 0; }

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
            Event::LateGame => {
                self.unlockable = true;
                Upgrade::add_upgrade(&mut self.avail_upgrades, &COMPLEX_UPGRADES, 0, self.pop_up.panel);
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
        sprite!("mines", xy = self.hitbox.xy());

        for drone in self.drones.iter() {
            if drone.front {
                drone.draw();
            }
        }
        
        if !self.unlocked { 
            rect!(xy = self.hitbox.translate(-32, -6).center(), wh = (64, 12), color = 0x222034ff);
            text!("LOCKED", xy = self.hitbox.translate(-15,-3).center(), color = 0xffffffff);       
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

impl POI for ResearchComplex {

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    
    fn produce(&mut self) -> u64 {
        return 0;
    }
    
    fn upgrade(&mut self, upgrade: &Upgrade, event_manager: &mut EventManager) {

    }
}