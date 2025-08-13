use super::*;

pub const MINES_BOX: (i32, i32, i32, i32) = (128, 0, 64, 64);

#[turbo::serialize]
pub struct AsteroidMines {
    pub station: Station,

    pub asteroid_field: AsteroidField,

    hitbox: Bounds,
    pop_up: PopUp,
    hovered: bool,

    clicked_at: usize,
    collections: Vec<Collection>,
    collect_interval: usize,

    avail_upgrades: Vec<Upgrade>,
}

impl AsteroidMines {
    pub fn load(player: &Player) -> Self {
        let hitbox = Bounds::new(128, 0, 64, 64);
        let pop_up =  PopUp::new("ASTEROID MINES".to_string(), DroneMode::Mining);
        let mut station = Station::new_drone(POIType::AsteroidMines, &mut DroneStats::new(32., 1.0, 500., 1.0), player);

        AsteroidMines {
            station,

            asteroid_field: AsteroidField::new(),

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
        let p = pointer::world();
        let rp = p.xy();

        self.asteroid_field.update();

        // Hover check
        if event_manager.dialogue.is_none() && self.station.unlockable {
            let was_hovered = self.hovered;
            self.hovered = 
                (player.hovered_poi.is_none() || player.hovered_poi == Some(POIType::AsteroidMines))
                && self.hitbox.intersects_xy(rp) 
                || (self.hovered && (self.pop_up.inspecting() || self.pop_up.hovered())); 
            if !self.hovered && was_hovered { player.hovered_poi = None; }
        } else {
            self.hovered = false;
        }
        
        // Update pop up position and buttons, apply upgrades
        if self.hovered {
            player.hovered_poi = Some(POIType::AsteroidMines);
            // Pop up returns upgrade player clicks
            if let Some(upgrade) = self.pop_up.update(self.hitbox, &self.station, &mut self.avail_upgrades, &MINES_UPGRADES, &player.resources) {
                self.upgrade(&upgrade, event_manager);
                player.purchase_upgrade(&upgrade);
            }
        }
        
        // Produce Resources
        let mut produced = (Resources::Metals, 0);
        // Initial click
        if self.hovered && p.just_pressed() && turbo::time::tick() - self.clicked_at >= self.collect_interval {
            self.clicked_at = turbo::time::tick();
        }
        // Manually produce resources every 30 ticks
        if turbo::time::tick() - self.clicked_at >= self.collect_interval {
            if self.hitbox.intersects_xy(rp) && p.pressed() {
                produced.1 += self.manual_produce();
            }
            self.clicked_at = turbo::time::tick();
        }

        // Produce based on drone update
        produced.1 += self.update_drones(&player);

        // Update collection numbers
        self.station.update_collections();

        if self.station.unlocked {
            player.collect(produced);
        }
    }


    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::MinesUnlockable => {
                self.station.unlockable = true;
                Upgrade::add_upgrade(&mut self.avail_upgrades, &MINES_UPGRADES, 0, self.pop_up.panel);
            }
            Event::UnlockPowerPlant => {
                Upgrade::add_upgrade(&mut self.avail_upgrades, &MINES_UPGRADES, 3, self.pop_up.panel);
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
                    let upgrade = MINES_UPGRADES[4].clone().init(self.pop_up.panel, 1);
                    self.avail_upgrades.insert(1, upgrade);
                }
            }
            _ => {}
        }
    }
    
    pub fn draw(&self) {
        self.asteroid_field.draw();

        let mut bob_box = self.hitbox;
        if self.station.unlocked {
            let bob =  f32::sin(turbo::time::tick() as f32 / 35.0 + 20.0) * 1.5;
            bob_box = self.hitbox.translate_y(bob);
        }

        if self.station.drone_stats.amped > 1.0 {
            sprite!(
                "amp_aura_0", 
                xy = (bob_box.x() - 8, bob_box.y() - 4),
                rotation = -(time::tick() as f32 / 10. % 360.),
            );
            sprite!(
                "amp_aura_1", 
                xy = (bob_box.x() - 8, bob_box.y() - 4),
                rotation = time::tick() as f32 / 10. % 360.,
            );
        }

        self.station.draw_back();

        if !self.station.unlocked { 
            sprite!("mines_locked_outline", xy = bob_box.xy());
        }
        // outline
        if self.hovered {
            sprite!("mines_hovered", xy = bob_box.xy());
        }
        // main GFX
        sprite!("mines", xy = bob_box.xy());

        if !self.station.unlocked { 
            sprite!("mines_locked", xy = bob_box.xy());
            text!("LOCKED", xy = bob_box.translate(-15,2).center(), color = 0xffffffff);       
        }

        // Draw collection numbers
        self.station.draw_front(); 
    }

    pub fn draw_ui(&self) {
        // pop up
        if self.hovered {
            self.pop_up.draw(&self.station, &self.avail_upgrades);
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
                self.station.new_collect(drone.pos, (Resources::Metals, amount));
            }
        }

        self.station.drones = drones;
        produced
    }

    fn upgrade(&mut self, upgrade: &Upgrade, event_manager: &mut EventManager) {
        if upgrade.name == "CONSTRUCT" {
            self.station.unlocked = true;
            if self.station.innovation {
                let upgrade = MINES_UPGRADES[4].clone().init(self.pop_up.panel, 1);
                self.avail_upgrades.insert(1, upgrade);
            }
        } else if upgrade.name.starts_with("DEPLOY") {
            let xy = self.hitbox.translate(self.hitbox.w()/2,self.hitbox.h()/2).xy();
            self.station.deploy_drone(DroneMode::Mining,xy);
            self.pop_up.drones += 1;
            if self.station.drones.len() == 1 {
                event_manager.trigger(Event::PowerPlantUnlockable);
            }
        } else if upgrade.name.starts_with("UNASSIGN") {
            if self.station.drones.len() == 0 { return; }
            self.station.drones.remove(0);
            self.pop_up.drones -= 1;
            event_manager.trigger(Event::RecallDrone);
        } else if upgrade.name.starts_with("DRILL") {
            self.station.drone_stats.eff += 0.8;
        } else if upgrade.name.starts_with("ADV.") {
            self.station.drone_stats.speed *= 0.92;
        }
        else if upgrade.name.starts_with("ORE") {
            self.station.drone_stats.base += 5.;
        }
    }
}