use super::*;

pub const PLANT_BOX: (i32, i32, i32, i32) = (548, 72, 64, 64);

#[turbo::serialize]
pub struct PowerPlant {
    pub station: Station,

    pub nebula: NebulaStorm,

    pub hitbox: Bounds,
    pop_up: PopUp,
    hovered: bool,

    clicked_at: usize,
    collect_interval: usize,

    avail_upgrades: Vec<Upgrade>,
}
impl PowerPlant {
    pub fn load(player: &Player) -> Self {
        let hitbox = Bounds::new(PLANT_BOX.0, PLANT_BOX.1, PLANT_BOX.2, PLANT_BOX.3);
        let pop_up =  PopUp::new("POWER PLANT".to_string(), DroneMode::Conduit);
        let mut station = Station::new_drone(POIType::PowerPlant, &mut DroneStats::new(26., 1.0, 750., 1.0), player);

        PowerPlant {
            station,

            nebula: NebulaStorm::new(),

            hitbox,
            pop_up,
            hovered: false,

            clicked_at: 0,
            collect_interval: 30,

            avail_upgrades: vec![],
        }
    }

    pub fn update(&mut self, player: &mut Player, event_manager: &mut EventManager) {
        self.nebula.update();
        
        // Hover check
        let p = pointer::world();
        let rp = p.xy();
        if event_manager.dialogue.is_none() && self.station.unlockable {
            let was_hovered = self.hovered;
            self.hovered = 
                (player.hovered_poi.is_none() || player.hovered_poi == Some(POIType::PowerPlant))
                && self.hitbox.intersects_xy(rp) 
                || (self.hovered && (self.pop_up.inspecting() || self.pop_up.hovered())); 
            if !self.hovered && was_hovered { player.hovered_poi = None; }
        } else {
            self.hovered = false;
        }

        // Update pop up position and buttons, apply upgrades
        if self.hovered {
            player.hovered_poi = Some(POIType::PowerPlant);
            if let Some(upgrade) = self.pop_up.update(self.hitbox, &self.station, &mut self.avail_upgrades, &POWER_UPGRADES, &player.resources) {
                self.upgrade(&upgrade, event_manager);
                player.purchase_upgrade(&upgrade);
            }
        }

        // Produce Resources
        let mut produced = (Resources::Power, 0);

        produced.1 += self.update_drones(&player);
                
        if self.station.unlocked {
            player.collect(produced);
        }

        self.station.update_collections();
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::PowerPlantUnlockable => {
                self.station.unlockable = true;
                Upgrade::add_upgrade(&mut self.avail_upgrades, &POWER_UPGRADES, 0, self.pop_up.panel);
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
                    let upgrade = POWER_UPGRADES[4].clone().init(self.pop_up.panel, 1);
                    self.avail_upgrades.insert(1, upgrade);
                }
            }
            _ => {}
        }
    }

    pub fn draw(&mut self) {
        let mut bob_box = self.hitbox;
        if self.station.unlocked {
            let bob =  f32::sin(turbo::time::tick() as f32 / 20.0 + 5.0) * 1.5;
            bob_box = self.hitbox.translate_y(bob);
        }

        self.nebula.draw();
        
        if self.station.drone_stats.amped > 1.0 {
            sprite!(
                "amp_aura_0", 
                xy = (bob_box.x() - 8, bob_box.y()),
                rotation = -(time::tick() as f32 / 10. % 360.),
                color = 0xffffffff,
            );
            sprite!(
                "amp_aura_1", 
                xy = (bob_box.x() - 8, bob_box.y()),
                rotation = time::tick() as f32 / 10. % 360.,
                color = 0xffffffff,
            );
        }
        
        // Draw drones
        self.station.draw_back();
        
        if !self.station.unlocked { 
            sprite!("plant_locked_outline", xy = bob_box.xy());
        }
        // outline
        if self.hovered {
            sprite!("plant_hovered", xy = bob_box.xy());
        }
        // main GFX
        sprite!("plant", xy = bob_box.xy());

        if !self.station.unlocked { 
            sprite!("plant_locked", xy = bob_box.xy());
            text!("LOCKED", xy = bob_box.translate(-15, 12).center(), color = 0xffffffff);   
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

    fn update_drones(&mut self, player: &Player) -> u64 {
        let mut produced = 0;
        let drone_stats = self.station.drone_stats.clone();
        let mut drones = std::mem::take(&mut self.station.drones);

        for drone in drones.iter_mut() {
            if drone.update(&drone_stats, self as &mut dyn POI) {
                let amount = drone_stats.produce();
                produced += amount;
                self.station.new_collect((PLANT_BOX.0 as f32 + 12. + random::f32() * 12., PLANT_BOX.1 as f32), (Resources::Power, amount));
            }
        }

        self.station.drones = drones;
        produced
    }

    fn upgrade(&mut self, upgrade: &Upgrade, event_manager: &mut EventManager) {
        if upgrade.name == "CONSTRUCT" {
            self.station.unlocked = true;
            event_manager.trigger(Event::UnlockPowerPlant);
            if self.station.innovation {
                let upgrade = POWER_UPGRADES[4].clone().init(self.pop_up.panel, 1);
                self.avail_upgrades.insert(1, upgrade);
            }
        } else if upgrade.name.starts_with("DEPLOY") {
            let xy = self.hitbox.translate(self.hitbox.w()/2,self.hitbox.h()/2).xy();
            self.station.deploy_drone(DroneMode::Conduit,xy);
            self.pop_up.drones += 1;
            if self.station.drones.len() == 1 {
                event_manager.trigger(Event::LateGame);
            }
        }
        else if upgrade.name.starts_with("UNASSIGN") {
            if self.station.drones.len() == 0 { return; }
            self.station.drones.remove(0);
            self.pop_up.drones -= 1;
            event_manager.trigger(Event::RecallDrone);
        }
        else if upgrade.name.starts_with("REFLECT") {
            self.station.drone_stats.eff += 0.85;
        }
        else if upgrade.name.starts_with("ARC") {
            self.station.drone_stats.speed *= 0.94;
        }
         else if upgrade.name.starts_with("POWER") {
            self.station.drone_stats.base += 4.;
        }
    }
}