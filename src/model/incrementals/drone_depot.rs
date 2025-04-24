use super::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct DroneDepot {
    pub drones: u32,

    pub unlockable: bool,
    pub unlocked: bool,

    hitbox: Bounds,
    pop_up: PopUp,
    hovered: bool,

    clicked_at: usize,

    avail_upgrades: Vec<Upgrade>,
}
impl DroneDepot {
    pub fn load() -> Self {
        let hitbox = Bounds::new(192, 416, 64, 64);
        let pop_up =  PopUp::new("DRONE DEPOT".to_string());
        DroneDepot {
            drones: 0,
            
            unlockable: false,
            unlocked: false,

            hitbox,
            pop_up,
            hovered: false,

            clicked_at: 0,

            avail_upgrades: vec![],
        }
        
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::DroneDepotUnlockable => {
                self.unlockable = true;
                self.avail_upgrades.push(DEPOT_UPGRADES[0].clone());
            }
            _ => {}
        }
    }

    pub fn update(&mut self, player: &mut Player, event_manager: &mut EventManager) {
        let p = pointer();
        let pp = p.relative_position();
        self.hovered = self.hitbox.intersects_xy(pp) || (self.hovered && self.pop_up.hovered()); 

        // Update pop up position and buttons, apply upgrades
        if self.hovered {
            if let Some(upgrade) = self.pop_up.update(self.hitbox, &mut self.avail_upgrades, &DEPOT_UPGRADES, &player.resources) {
                self.upgrade(&upgrade, event_manager);
                player.upgrade(&upgrade, self);
            }
        }

        //player.collect(produced);
    }

    pub fn draw(&self) {
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
            color = 0xdf7126ff
        );
        if !self.unlocked { 
            text!("LOCKED", xy = (self.hitbox.x() + 4, self.hitbox.y() + 4), color = 0xffffffff);       
        }
        
        // pop up
        if self.hovered {
            self.pop_up.draw(&self.avail_upgrades);
        }
    }
}

impl POI for DroneDepot {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn produce(&mut self) -> u32 {
        return 0;
    }

    fn upgrade(&mut self, upgrade: &Upgrade, event_manager: &mut EventManager) {
        if upgrade.name == "CONSTRUCT" {
            self.unlocked = true;
            event_manager.trigger(Event::UnlockDroneDepot);
        }
    }
}