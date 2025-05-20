use super::*;

pub const DEPOT_BOX: (i32, i32, i32, i32) = (128, 320, 64, 64);

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct DroneDepot {
    pub drones: Vec<Drone>,
    pub drone_level: u32,
    drone_speed: u32,

    pub unlockable: bool,
    pub unlocked: bool,

    hitbox: Bounds,
    pop_up: PopUp,
    hovered: bool,

    fabricator_unlocked: bool,
    power_plant_unlocked: bool,
    fabricator: PopUp,
    fab_prog: u64,
    fab_level: u32,
    fab_limit: u64,

    clicked_at: usize,
    collections: Vec<Collection>,
    collect_interval: usize,
    

    avail_upgrades: Vec<Upgrade>,
    fab_upgrades: Vec<Upgrade>,
}
impl DroneDepot {
    pub fn load() -> Self {
        let hitbox = Bounds::new(DEPOT_BOX.0, DEPOT_BOX.1, DEPOT_BOX.2, DEPOT_BOX.3);
        let pop_up =  PopUp::new("DRONE DEPOT".to_string());
        let fabricator =  PopUp::new("FABRICATOR".to_string());
        let anim = animation::get("drone_locked");
        anim.use_sprite("vignette");
        DroneDepot {
            drones: vec![],
            drone_level: 0,
            drone_speed: 0,

            unlockable: false,
            unlocked: false,

            fabricator_unlocked: false,
            power_plant_unlocked: false,
            fabricator,
            fab_prog: 0,
            fab_level: 0,
            fab_limit: 500,

            hitbox,
            pop_up,
            hovered: false,

            clicked_at: 0,
            collections: vec![],
            collect_interval: 30,

            avail_upgrades: vec![],
            fab_upgrades: vec![],
        }
        
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::DroneDepotUnlockable => {
                self.unlockable = true;
                Upgrade::add_upgrade(&mut self.avail_upgrades, &DEPOT_UPGRADES, 0, self.pop_up.panel);
            }
            Event::PowerPlantUnlockable => {
                Upgrade::add_upgrade(&mut self.avail_upgrades, &DEPOT_UPGRADES, 2, self.pop_up.panel);
            }
            Event::UnlockPowerPlant => {
                self.power_plant_unlocked = true;
                if self.fabricator_unlocked {
                    Upgrade::add_upgrade(&mut self.fab_upgrades, &DEPOT_UPGRADES, 5, self.fabricator.panel);
                }
            }
            _ => {}
        }
    }

    pub fn update(&mut self, player: &mut Player, event_manager: &mut EventManager) {
        let p = pointer();
        let rp = p.xy();
        if event_manager.dialogue.is_none() {
            self.hovered = 
                self.hitbox.intersects_xy(rp) 
                || (self.hovered && self.pop_up.hovered()) 
                || (self.fabricator_unlocked && self.hovered && self.fabricator.hovered()); 
        } else {
            self.hovered = false;
        }

        // Update pop up position and buttons, apply upgrades
        if self.hovered {
            let z = camera::z() as i32;
            let mut offset = if self.fabricator_unlocked { self.hitbox.translate_y(-(self.pop_up.panel.h() as i32/2 + 1) * 1/z) } else { self.hitbox };
            if let Some(upgrade) = self.pop_up.update(offset, &mut self.avail_upgrades, &DEPOT_UPGRADES, &player.resources) {
                self.upgrade(&upgrade, event_manager);
                player.upgrade(&upgrade, self);
            }
            
            if self.fabricator_unlocked {
                offset = self.hitbox.translate_y((self.fabricator.panel.h() as i32/2 + 1) * 1/z);
                if let Some(upgrade) = self.fabricator.update_fabricator(offset, &mut self.fab_upgrades, &DEPOT_UPGRADES, &player.resources) {
                    self.upgrade(&upgrade, event_manager);
                    player.upgrade(&upgrade, self);
                }
            }
        }
        // Produce Resources
        
        // Initial click
        // if self.hovered && p.just_pressed() && tick() - self.clicked_at >= self.collect_interval {
        //     self.clicked_at = tick();
        // }
        // // Manually produce resources every 30 ticks
        // if tick() - self.clicked_at >= self.collect_interval {
        //     if self.hitbox.intersects_xy(rp) && p.pressed() {
        //         produced.1 += self.manual_produce();
        //     }
        //     self.clicked_at = tick();
        // }

        // Produce based on drone update
        for drone in self.drones.iter_mut() {
            if let Some(prod) = drone.shipping() {
                if !drone.on_site && !drone.cargo.is_empty() {
                    player.remove((Resources::Metals, prod.1));
                    self.collections.push(Collection::new_detail(drone.pos, (Resources::Metals, prod.1), false));
                } else {
                    self.fab_prog += prod.1;
                    if self.fab_prog >= self.fab_limit {
                        self.fab_level += 1;
                        self.fab_limit = CostFormula::Exponential.calculate_cost(vec![(Resources::Metals, 500)], self.fab_level)[0].1;
                        self.fab_prog = 0;
                        player.collect((Resources::Drones, 1));
                        self.collections.push(Collection::new((self.hitbox.center_x() as f32, self.hitbox.center_y() as f32), (Resources::Drones, 1),));
                    } 
                }
            } else if !drone.cargo.is_empty() {

            }
        }

        // Update collection numbers
        self.collections.retain_mut(|collection| {
            collection.update();
            collection.is_active // Keep only active collections
        }); 
    }

    pub fn draw(&self) {
        if !self.unlocked { 
            sprite!("depot_locked_outline", xy = self.hitbox.xy());
        }
        // outline
        if self.hovered {
            sprite!("depot_hovered", xy = self.hitbox.xy());
            if self.fabricator_unlocked {
                sprite!("fab_hovered", xy = self.hitbox.xy());
            }
        }
        // main GFX
        sprite!("depot", xy = self.hitbox.xy());
        if self.fabricator_unlocked {
            sprite!("fab", xy = self.hitbox.xy());
        }

        for drone in self.drones.iter() {
            if drone.front {
                drone.draw();
            }
        }

        if !self.unlocked { 
            sprite!("depot_locked", xy = self.hitbox.xy());
            text!("LOCKED", xy = self.hitbox.translate(-16,-4).center(), color = 0xffffffff);       
        }
        
        // Draw drones
        for drone in self.drones.iter() {
            if drone.front {
                drone.draw();
            }
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
            if self.fabricator_unlocked {
                self.fabricator.draw_fabricator(&self.fab_upgrades, self.fab_prog, self.fab_limit);
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

    fn produce(&mut self) -> u64 {
        return 0;
    }

    fn upgrade(&mut self, upgrade: &Upgrade, event_manager: &mut EventManager) {
        if upgrade.name == "CONSTRUCT" {
            self.unlocked = true;
            event_manager.trigger(Event::UnlockDroneDepot);
        } else if upgrade.name == "CONSTRUCT FABRICATOR" {
            self.fabricator_unlocked = true;
            Upgrade::add_upgrade(&mut self.fab_upgrades, &DEPOT_UPGRADES, 3, self.fabricator.panel);
            Upgrade::add_upgrade(&mut self.fab_upgrades, &DEPOT_UPGRADES, 4, self.fabricator.panel);
            if self.power_plant_unlocked {
                Upgrade::add_upgrade(&mut self.fab_upgrades, &DEPOT_UPGRADES, 5, self.fabricator.panel);
            }
        } else if upgrade.name.starts_with("DEPLOY") {
            let xy = self.hitbox.translate(self.hitbox.w()/2,self.hitbox.h()/2).xy();
            self.drones.push(Drone::new(DroneMode::Shipping, self.drone_level, self.drone_speed, xy));
            self.fabricator.drones += 1;
        } else if upgrade.name.starts_with("CARGO") {
            self.drone_level += 1;
            for drone in self.drones.iter_mut() {
                drone.level += 1;
            }
        } else if upgrade.name.starts_with("PLASMA") {
            self.drone_speed += 1;
            for drone in self.drones.iter_mut() {
                drone.speed += 1;
            }
        }
    }
}