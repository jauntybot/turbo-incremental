use super::*;

pub static COMPLEX_BOX: (i32, i32, i32, i32) = (490, 300, 64, 64);

#[turbo::serialize]
pub struct ResearchComplex {
    pub station: Station,
    

    pub hitbox: Bounds,
    pub pop_up: PopUp,
    pub project: PopUp,
    pub hovered: bool,
    

    pub avail_upgrades: Vec<Upgrade>,
    pub drone_upgrades: Vec<Upgrade>,

    pub active_project: Option<Upgrade>,
    projects_completed: usize,
}

impl ResearchComplex {
    pub fn load(player: &Player) -> Self {
        let hitbox = Bounds::new(COMPLEX_BOX.0, COMPLEX_BOX.1, COMPLEX_BOX.2, COMPLEX_BOX.3);   
        let pop_up =  PopUp::new("RESEARCH COMPLEX".to_string(), DroneMode::Research);
        let project = PopUp::new_fab("RESEARCH PROJECT".to_string(), DroneMode::Research);
        let station = Station::new_drone(POIType::ResearchComplex, &mut DroneStats::new(35., 1.0,  450., 1.0), player);

        ResearchComplex { 

            station,

            hitbox,
            pop_up,
            project,
            hovered: false,

            avail_upgrades: vec![],
            drone_upgrades: vec![],

            active_project: None,
            projects_completed: 0,
        }
    }

    pub fn update(&mut self, player: &mut Player, event_manager: &mut EventManager) {
        let p = pointer::world();
        let rp = p.xy();

        // Hover check
        if event_manager.dialogue.is_none() && self.station.unlockable {
            let was_hovered = self.hovered;
            self.hovered = 
                (player.hovered_poi.is_none() || player.hovered_poi == Some(POIType::ResearchComplex))
                && self.hitbox.intersects_xy(rp) 
                || (self.hovered && self.pop_up.hovered()) 
                || (self.hovered && (self.project.inspecting() || self.project.hovered())
            ); 
            if !self.hovered && was_hovered { player.hovered_poi = None }
        } else {
            self.hovered = false;
        }

        // Produce Resources
        let mut produced = (Resources::Research, 0);

        produced.1 +=  self.update_drones(&player);

        let mut researched = None;
        if let Some(project) = &mut self.active_project {
            project.base_cost[0].1 += produced.1;
            if project.base_cost[0].1 >= project.cost[0].1 {
                if project.unlocks.len() > 0 {
                    for i in 0..project.unlocks.len() {
                        Upgrade::add_upgrade(&mut self.avail_upgrades, &COMPLEX_UPGRADES, project.unlocks[i], self.pop_up.panel);
                    }
                    project.unlocks = vec![]; // Clear unlocks after applying upgrade
                }
                researched = Some(project.clone());
 
            }
        }
        if let Some(project) = &mut researched {
            if let Some(index) = self.avail_upgrades.iter().position(|p| p.name == project.name) {
                self.avail_upgrades.remove(index);
            }
            project.name.replace_range(0..9, "");
            self.upgrade(&project, event_manager);
            self.active_project = None;
        }
        
        // Update pop up position and buttons, apply upgrades
        if self.hovered {
            player.hovered_poi = Some(POIType::ResearchComplex);
            let z = camera::z() as i32;
            let mut offset = self.hitbox;
            if self.station.unlocked {
                offset = self.hitbox.translate_y(-(self.pop_up.panel.h() as i32/2 + 1) * 1/z);
            }
            // Pop up returns upgrade player clicks
            if let Some(upgrade) = self.pop_up.update(offset, &self.station, &mut self.avail_upgrades, &COMPLEX_UPGRADES, &player.resources) {
                self.upgrade(&upgrade, event_manager);
                if upgrade.name == "CONSTRUCT" {
                    player.purchase_upgrade(&upgrade);
                }
            }
            if self.station.unlocked {
                offset = self.hitbox.translate_y((self.project.panel.h() as i32/2 + 1) * 1/z);
                if let Some(upgrade) = self.project.update(offset, &self.station, &mut self.drone_upgrades, &COMPLEX_UPGRADES, &player.resources) {
                    self.upgrade(&upgrade, event_manager);
                    player.purchase_upgrade(&upgrade);
                }
            }
        }

        // Update collection numbers
        self.station.update_collections();
        
        player.collect(produced);
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::LateGame => {
                self.station.unlockable = true;
                Upgrade::add_upgrade(&mut self.avail_upgrades, &COMPLEX_UPGRADES, 0, self.pop_up.panel);
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
                if self.avail_upgrades.iter().any(|u| u.name.starts_with("DEPLOY")) {
                    let upgrade = COMPLEX_UPGRADES[2].clone().init(self.pop_up.panel, 1);
                    self.drone_upgrades.insert(1, upgrade);
                }
            }
            _ => {}
        }
    }

    pub fn draw(&self) {
        let mut bob_box = self.hitbox;
        if self.station.unlocked {
            let bob =  f32::sin(turbo::time::tick() as f32 / 30.0) * 1.5;
            bob_box = self.hitbox.translate_y(bob);
        }
        self.station.draw_back();

        if !self.station.unlocked { 
            sprite!("complex_locked_outline", xy = bob_box.xy());
        }
        // outline
        if self.hovered {
            sprite!("complex_hovered", xy = bob_box.xy());
        }

        // main GFX
        sprite!("complex", xy = bob_box.xy());

        self.station.draw_front();

        if !self.station.unlocked { 
            sprite!("complex_locked", xy = bob_box.xy());
            text!("LOCKED", xy = bob_box.translate(-15,-4).center(), color = 0xffffffff);  
        }     
    }

    pub fn draw_ui(&mut self) {
        // pop up
        if self.hovered {
            self.pop_up.draw(&self.station, &self.avail_upgrades);
            if self.station.unlocked {
                let mut prog = 0;
                let mut limit = 0;
                if let Some(project) = &self.active_project {
                    limit = project.cost[0].1;
                    prog = project.base_cost[0].1;
                }
                self.project.draw_project(self, prog, limit);
            }
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

    fn get_station(&self) -> &Station {
        &self.station
    }
    
    fn update_drones(&mut self, player: &Player) -> u64 {
        let mut produced = 0;
        let drone_stats = self.station.drone_stats.clone();
        let mut drones = std::mem::take(&mut self.station.drones);

        for drone in drones.iter_mut() {
            if drone.update(&drone_stats, self as &mut dyn POI) && self.active_project.is_some() {
                let amount = drone_stats.produce();
                produced += drone_stats.produce();
                self.station.new_collect(drone.pos, (Resources::Research, amount));
            }
        }

        self.station.drones = drones;
        produced
    }
    
    fn upgrade(&mut self, upgrade: &Upgrade, event_manager: &mut EventManager) {
        log!("{}", upgrade.name);
        if upgrade.name == "CONSTRUCT" {
            self.station.unlocked = true;
            Upgrade::add_upgrade(&mut &mut self.drone_upgrades, &COMPLEX_UPGRADES, 1, self.project.panel);
            //Upgrade::add_upgrade(&mut &mut self.drone_upgrades, &COMPLEX_UPGRADES, 2, self.project.panel);
            Upgrade::add_upgrade(&mut &mut self.drone_upgrades, &COMPLEX_UPGRADES, 3, self.project.panel);
            Upgrade::add_upgrade(&mut &mut self.drone_upgrades, &COMPLEX_UPGRADES, 4, self.project.panel);
            if self.station.innovation {
                let upgrade = COMPLEX_UPGRADES[2].clone().init(self.pop_up.panel, 1);
                self.drone_upgrades.insert(1, upgrade);
            }
            event_manager.trigger(Event::ResearchComplexBuilt);
        } else if upgrade.name.starts_with("DEPLOY") {
            let xy = self.hitbox.translate(self.hitbox.w()/2,self.hitbox.h()/2).xy();
            self.station.deploy_drone(DroneMode::Research,xy);
            self.project.drones += 1;
        } else if upgrade.name.starts_with("UNASSIGN") {
            if self.station.drones.len() == 0 { return; }
            self.station.drones.remove(0);
            self.project.drones -= 1;
            event_manager.trigger(Event::RecallDrone);
        } else if upgrade.name.starts_with("MACHINE") {
            self.station.drone_stats.eff += 0.70;
        } else if upgrade.name.starts_with("RAPID") {
            self.station.drone_stats.speed *= 0.96;
        } else if upgrade.name == ("ADV. DATABASE") {
            self.station.drone_stats.base += 4.;
        } else if upgrade.name.starts_with("RESEARCH") {
            // Copy the active upgrade back to the avail upgrades
            if let Some(project) = &mut self.active_project {
                if let Some(i) = self.avail_upgrades.iter().position(|u| u.name == project.name) {
                    self.avail_upgrades[i].base_cost[0].1 = project.base_cost[0].1;
                } 
            }
            // Loop through avail upgrades
            let mut toggle_off = false;
            for i in 0..self.avail_upgrades.len() {
                let u = self.avail_upgrades[i].clone();
                if let UpgradeType::Toggle { toggle } = &mut self.avail_upgrades[i].u_type {
                    toggle.value = false;
                    // If this avail upgrade is the upgrade passed
                    if &u.name == &upgrade.name {
                        // Check if there's an active upgrade
                        if let Some(project) = &self.active_project {
                            if &u.name == &project.name {
                                toggle_off = true;
                                continue;
                            } 
                        }
                        toggle.value = true;
                        self.active_project = Some(u);
                    }
                }
            }
            if toggle_off {
                self.active_project = None;
            }
        }
        else if upgrade.name == "FABRICATOR" {
            self.projects_completed += 1;
            event_manager.trigger(Event::FabricatorUnlockable);
            if self.projects_completed == 3 {
                Upgrade::add_upgrade(&mut self.avail_upgrades, &COMPLEX_UPGRADES, 8, self.pop_up.panel);   
            }
        }
        else if upgrade.name == "DRONE AMP" {
            self.projects_completed += 1;
            event_manager.trigger(Event::AmpUnlockable);
            if self.projects_completed == 3 {
                Upgrade::add_upgrade(&mut self.avail_upgrades, &COMPLEX_UPGRADES, 8, self.pop_up.panel);   
            }
        }
        else if upgrade.name == "ADV. DRONES" {
            self.projects_completed += 1;
            event_manager.trigger(Event::AdvDronesResearched);
            if self.projects_completed == 3 {
                Upgrade::add_upgrade(&mut self.avail_upgrades, &COMPLEX_UPGRADES, 8, self.pop_up.panel);   
            }
            log!("event triggered");
        } 
        else if upgrade.name == "SIMULACRUM" {
            event_manager.trigger(Event::Simulacrum);
        }
    }
}