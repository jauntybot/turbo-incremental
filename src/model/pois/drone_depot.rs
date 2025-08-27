use super::*;

pub const DEPOT_BOX: (i32, i32, i32, i32) = (160, 284, 64, 64);

#[turbo::serialize]
pub struct DroneDepot {
    pub station: Station,

    hitbox: Bounds,
    pop_up: PopUp,
    hovered: bool,

    fabricator_unlockable: bool,
    fabricator_unlocked: bool,
    fabricator_enabled: bool,
    fabricator: PopUp,
    fab_prog: u64,
    fab_level: u32,
    fab_limit: u64,

    ad_interval: usize,
    ad_counter: usize,

    avail_upgrades: Vec<Upgrade>,
    fab_upgrades: Vec<Upgrade>,
}
impl DroneDepot {
    pub fn load(player: &Player) -> Self {
        let hitbox = Bounds::new(DEPOT_BOX.0, DEPOT_BOX.1, DEPOT_BOX.2, DEPOT_BOX.3);
        let pop_up =  PopUp::new("DRONE DEPOT".to_string(), DroneMode::Shipping);
        let fabricator =  PopUp::new_fab("FABRICATOR".to_string(), DroneMode::Shipping);
        let station = Station::new_drone(POIType::DroneDepot, &mut DroneStats::new(10., 1.0, 600., 1.0), player);

        DroneDepot {
            station,

            fabricator_unlockable: false,
            fabricator_unlocked: false,
            fabricator_enabled: true,

            fabricator,
            fab_prog: 0,
            fab_level: 0,
            fab_limit: 164,

            hitbox,
            pop_up,
            hovered: false,

            ad_interval: 120 * 60,
            ad_counter: 0,

            avail_upgrades: vec![],
            fab_upgrades: vec![],
        }
        
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::DroneDepotUnlockable => {
                self.station.unlockable = true;
                Upgrade::add_upgrade(&mut self.avail_upgrades, &DEPOT_UPGRADES, 0, self.pop_up.panel);
            }
            Event::FabricatorUnlockable => {
                self.fabricator_unlockable = true;
                Upgrade::add_upgrade(&mut self.avail_upgrades, &DEPOT_UPGRADES, 2, self.pop_up.panel);
            }
            Event::AdvDronesResearched => {
                self.station.drone_stats.mult += 1.;
            }
            Event::RecallUpgrade => {
                self.station.drone_stats.recall = true;
            }
            Event::BaseUpgrade { amount } => {
                self.station.drone_stats.base += amount;
            }
            Event::InnovationUpgrade => {
                self.station.innovation = true;
                if self.fab_upgrades.iter().any(|u| u.name.starts_with("DEPLOY")) {
                    let upgrade = DEPOT_UPGRADES[6].clone().init(self.pop_up.panel, 1);
                    self.fab_upgrades.insert(1, upgrade);
                }
            }
            _ => {}
        }
    }

    pub fn update(&mut self, player: &mut Player, event_manager: &mut EventManager) {
        let p = pointer::world();
        let rp = p.xy();

        // Hover check
        if event_manager.dialogue.is_none() && self.station.unlockable {
            let was_hovered = self.hovered;
            self.hovered = 
                (player.hovered_poi.is_none() || player.hovered_poi == Some(POIType::DroneDepot))
                && self.hitbox.intersects_xy(rp) 
                || (self.hovered && (self.pop_up.inspecting() || self.pop_up.hovered()))
                || (self.fabricator_unlocked && self.hovered && (self.fabricator.inspecting() || self.fabricator.hovered()));
            if !self.hovered && was_hovered { player.hovered_poi = None; }
        } else {
            self.hovered = false;
        }

        // CRAZY GAMES AD REWARD
        if self.station.unlocked {
            if self.ad_counter >= self.ad_interval {
                self.avail_upgrades[1].tooltip = WrapBox::new("Watch an ad to recieve 3 DRONES".to_string(), 0);
            } else {
                self.ad_counter += 1;
                self.avail_upgrades[1].tooltip = WrapBox::new(format!("Available in {} \n Watch an ad to recieve 3 DRONES", Numbers::time((self.ad_interval - self.ad_counter) as u64)), 0);
            }
            self.avail_upgrades[1].cost[0].1 = (self.ad_interval - self.ad_counter) as u64;
        }

        // Update pop up position and buttons, apply upgrades
        if self.hovered {
            player.hovered_poi = Some(POIType::DroneDepot);
            let z = camera::z() as i32;
            let mut offset = if self.fabricator_unlocked && self.fabricator_enabled { self.hitbox.translate_y(-(self.pop_up.panel.h() as i32/2 + 1) * 1/z) } else { self.hitbox };
            if let Some(upgrade) = self.pop_up.update(offset, &self.station, &mut self.avail_upgrades, &DEPOT_UPGRADES, &player.resources) {
                self.upgrade(&upgrade, event_manager);
                player.purchase_upgrade(&upgrade);
                if upgrade.name.starts_with("DRONE") {
                    player.collect((Resources::Drones, 1));
                } else if upgrade.name.starts_with("SPONSORED") {
                    turbo::events::emit("rewarded_ad", "");
                    player.collect((Resources::Drones, 3));
                }
            }
            
            if self.fabricator_unlocked &&self.fabricator_enabled {
                offset = self.hitbox.translate_y((self.fabricator.panel.h() as i32/2 + 1) * 1/z);
                if let Some(upgrade) = self.fabricator.update(offset, &self.station, &mut self.fab_upgrades, &DEPOT_UPGRADES, &player.resources) {
                    self.upgrade(&upgrade, event_manager);
                    player.purchase_upgrade(&upgrade);
                }
            }
        }

        // Produce based on drone update
        for drone in self.station.drones.iter_mut() {
            if let Some(prod) = drone.shipping(&self.station.drone_stats, self.fabricator_enabled) {
                if !drone.on_site && !drone.cargo.is_empty() {
                    player.remove((Resources::Metals, prod.1));
                    self.station.collections.push(Collection::new_detail(drone.pos, (Resources::Metals, prod.1), false));
                } else {
                    self.fab_prog += prod.1;
                    if self.fab_prog >= self.fab_limit {
                        self.fab_level += 1;
                        self.fab_limit = CostFormula::Exponential{factor: 1.25}.calculate_cost(vec![(Resources::Metals, 164)], self.fab_level)[0].1;
                        self.fab_prog = 0;
                        player.collect((Resources::Drones, 1));
                        self.station.collections.push(Collection::new((self.hitbox.center_x() as f32, self.hitbox.center_y() as f32), (Resources::Drones, 1),));
                    } 
                }
            } else if !drone.cargo.is_empty() {

            }
        }

        // Update collection numbers
        self.station.update_collections();
    }

    pub fn draw(&self) {
        let mut bob_box = self.hitbox;
        if self.station.unlocked {
            let bob =  f32::sin(turbo::time::tick() as f32 / 20.0) * 1.5;
            bob_box = self.hitbox.translate_y(bob);
        }

        self.station.draw_back();

        if !self.station.unlocked { 
            sprite!("depot_locked_outline", xy = bob_box.xy());
        }
        if self.fabricator_unlockable && !self.fabricator_unlocked {
            sprite!("fab_locked_outline", xy = bob_box.xy());
        }
        // outline
        if self.hovered {
            sprite!("depot_hovered", xy = bob_box.xy());
            if self.fabricator_unlockable {
                sprite!("fab_hovered", xy = bob_box.xy());
            }
        }
        // main GFX
        sprite!("depot", xy = bob_box.xy());
        if self.fabricator_unlocked {
            sprite!("fab", xy = bob_box.xy());
        }


        if !self.station.unlocked { 
            sprite!("depot_locked", xy = bob_box.xy());
            text!("LOCKED", xy = bob_box.translate(-15,-4).center(), color = 0xffffffff);       
        }
        if self.fabricator_unlockable && !self.fabricator_unlocked {
            sprite!("fab_locked", xy = bob_box.xy());
            text!("LOCKED", xy = bob_box.translate(-15,17).center(), color = 0xffffffff);       
        }
        
        // Draw drones
        self.station.draw_front();
    }

    
    pub fn draw_ui(&self) {
        // pop up
        if self.hovered {
            self.pop_up.draw(&self.station, &self.avail_upgrades);
            if self.fabricator_unlocked && self.fabricator_enabled {
                self.fabricator.draw_fabricator(&self.station, &self.fab_upgrades, self.fab_prog, self.fab_limit);
            }
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

    fn get_station(&self) -> &Station {
        &self.station
    }

    fn update_drones(&mut self, player: &Player) -> u64 { 0 }

    fn upgrade(&mut self, upgrade: &Upgrade, event_manager: &mut EventManager) {
        if upgrade.name == "CONSTRUCT" {
            self.station.unlocked = true;
            event_manager.trigger(Event::UnlockDroneDepot);
        } else if upgrade.name == "CONSTRUCT FABRICATOR" {
            self.fabricator_unlocked = true;
            Upgrade::add_upgrade(&mut self.fab_upgrades, &DEPOT_UPGRADES, 3, self.fabricator.panel);
            Upgrade::add_upgrade(&mut self.fab_upgrades, &DEPOT_UPGRADES, 4, self.fabricator.panel);
            Upgrade::add_upgrade(&mut self.fab_upgrades, &DEPOT_UPGRADES, 5, self.fabricator.panel);
            if self.station.innovation {
                let upgrade = DEPOT_UPGRADES[6].clone().init(self.pop_up.panel, 1);
                self.fab_upgrades.insert(1, upgrade);
            }
        } else if upgrade.name.starts_with("SPONSORED") {
            self.ad_counter = 0;
            if self.ad_interval < 240 * 60 {
                self.ad_interval += 30 * 60;
            }
        } else if upgrade.name.starts_with("DEPLOY") {
            let xy = self.hitbox.translate(self.hitbox.w()/2,self.hitbox.h()/2).xy();
            self.station.deploy_drone(DroneMode::Shipping, xy);
            self.fabricator.drones += 1;
        } else if upgrade.name.starts_with("UNASSIGN") {
            if self.station.drones.len() == 0 { return; }
            self.station.drones.remove(0);
            self.fabricator.drones -= 1;
            event_manager.trigger(Event::RecallDrone);
        } else if upgrade.name.starts_with("CARGO") {
            self.station.drone_stats.eff += 0.90;
        } else if upgrade.name.starts_with("PLASMA") {
            self.station.drone_stats.speed *= 0.96;
        } else if upgrade.name.starts_with("ADV.") {
            self.station.drone_stats.base += 5.;
        } else if upgrade.name.starts_with("DISABLE") || upgrade.name.starts_with("ENABLE") {
            if let Some(t) = self.avail_upgrades.iter_mut().find(|u| u.name == upgrade.name) {
                self.fabricator_enabled = !self.fabricator_enabled;
                let name = format!("{} FABRICATOR", if !self.fabricator_enabled { "ENABLE" } else { "DISABLE" });
                t.name = name;
            }
        }
    }
}