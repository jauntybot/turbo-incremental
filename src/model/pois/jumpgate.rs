use super::*;

pub const GATE_BOX: (i32, i32, i32, i32) = (288, 432, 64, 64);

#[turbo::serialize]
pub struct Jumpgate {
    pub station: Station,

    pub hitbox: Bounds,
    pub pop_up: PopUp,
    pub hovered: bool,
    
    earn: u64,
    limit: u64,
    prog: u64,
    
    pub avail_upgrades: Vec<Upgrade>,
}

impl Jumpgate {
    pub fn load() -> Self {
        let hitbox = Bounds::new(GATE_BOX.0, GATE_BOX.1, GATE_BOX.2, GATE_BOX.3);
        let pop_up =  PopUp::new_fab("JUMPGATE".to_string(), DroneMode::Survey);
        Jumpgate {
            station: Station::new(POIType::Jumpgate, DroneStats::new(20., 1.0, 300., 1.0)),

            hitbox,
            pop_up: pop_up.clone(),
            hovered: false,

            earn: 0,
            limit: 0,
            prog: 0,

            avail_upgrades: vec![],
        }
    }

    pub fn update(&mut self, player: &mut Player, event_manager: &mut EventManager) {
        let p = pointer::world();
        let rp = p.xy();
        
        // Hover check
        if event_manager.dialogue.is_none() && self.station.unlockable {
            let was_hovered = self.hovered;
            self.hovered = 
                (player.hovered_poi.is_none() || player.hovered_poi == Some(POIType::Jumpgate))
                && self.hitbox.intersects_xy(rp) 
                || (self.hovered && (self.pop_up.inspecting() || self.pop_up.hovered())); 
            if !self.hovered && was_hovered { player.hovered_poi = None; }
        } else {
            self.hovered = false;
        }

        // Update pop up position and buttons, apply upgrades
        if self.hovered {
            player.hovered_poi = Some(POIType::Jumpgate);
            // Pop up returns upgrade player clicks
            if let Some(upgrade) = self.pop_up.update(self.hitbox, &self.station, &mut self.avail_upgrades, &GATE_UPGRADES, &player.resources) {
                self.upgrade(&upgrade, event_manager);
                player.purchase_upgrade(&upgrade);
            }
        }

        self.limit = player.prestige_limit;
        self.earn = player.prestige_earned;
        self.prog = player.prestige_prog;
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::LateGame => {
                self.station.unlockable = true;
                Upgrade::add_upgrade(&mut self.avail_upgrades, &GATE_UPGRADES, 0, self.pop_up.panel);
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
        
        if !self.station.unlocked { 
            sprite!("gate_locked_outline", xy = bob_box.xy());
        }
        // outline
        if self.hovered {
            sprite!("gate_hovered", xy = bob_box.xy());
        }

        // main GFX
        sprite!("gate", xy = bob_box.xy());


        if !self.station.unlocked { 
            sprite!("gate_locked", xy = bob_box.xy());
            text!("LOCKED", xy = bob_box.translate(-15,-4).center(), color = 0xffffffff);  
        }
    }

    pub fn draw_ui(&self) { 
        
        if self.hovered {
            // pop up
            if self.station.unlocked {
                self.pop_up.draw_jumpgate(&self.station, &self.avail_upgrades, self.earn, self.prog, self.limit);
            } else {
                self.pop_up.draw(&self.station, &self.avail_upgrades);
            }
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
    
    fn update_drones(&mut self, player: &Player) -> u64 { 0 }

    fn upgrade(&mut self, upgrade: &Upgrade, event_manager: &mut EventManager) {
        if upgrade.name == "CONSTRUCT" {
            self.station.unlocked = true;
            event_manager.trigger(Event::JumpgateBuilt);
        } else if upgrade.name.starts_with("JUMP") {
            event_manager.trigger(Event::Prestige);
        }
    }
}