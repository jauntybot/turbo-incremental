use super::*;
use once_cell::sync::Lazy;

#[turbo::serialize]
pub struct PopUp {
    pub hitbox: Bounds,
    pub panel: Bounds,
    pub drone_hitbox: Bounds,
    pub drone_panel: Bounds,
    pub hovered: bool,
    drone_inspect: Btn,
    unassaign: Btn,
    inspecting: bool,
    title: String,
    pub drones: u32,
    drone_resource: String,
    drone_mode: DroneMode,
    fab: bool,
}
impl PopUp {
    pub fn new(title: String, drone_mode: DroneMode) -> Self {
        let hitbox = Bounds::new(-336, -320, 240, 106);
        let panel = hitbox
            .inset(10);
        let mut drone_inspect = Btn::new("".to_string(), Bounds::new(0, 0, 0, 0), false, 0);
        drone_inspect.clickable = false;
        Self {
            hitbox,
            panel,
            hovered: false,
            drone_hitbox: hitbox,
            drone_panel: panel,
            drone_inspect,
            unassaign: Btn::new("RECALL".to_string(), Bounds::new(0, 0, 0, 0), true, 1),
            inspecting: false,
            title,
            drones: 0,
            drone_resource: match drone_mode {
                DroneMode::Survey => "RESEARCH".to_string(),
                DroneMode::Mining => "METALS".to_string(),
                DroneMode::Conduit => "POWER".to_string(),
                DroneMode::Research => "RESEARCH".to_string(),
                DroneMode::Shipping => "METALS".to_string(),
                _ => "RESEARCH".to_string(),
            },
            drone_mode,
            fab: false,
        }
    }

    pub fn new_fab(title: String, drone_mode: DroneMode) -> Self {
        let hitbox = Bounds::new(-336, -320, 240, 130);
        let panel = hitbox
            .inset(10);
        let mut drone_inspect = Btn::new("".to_string(), Bounds::new(0, 0, 0, 0), true, 0);
        drone_inspect.clickable = false;
        Self {
            hitbox,
            panel,
            drone_hitbox: hitbox,
            drone_panel: panel,
            drone_inspect,
            unassaign: Btn::new("RECALL".to_string(), Bounds::new(0, 0, 0, 0), true, 1),
            inspecting: false,
            hovered: false,
            title,
            drones: 0,
            drone_resource: match drone_mode {
                DroneMode::Survey => "RESEARCH".to_string(),
                DroneMode::Mining => "METALS".to_string(),
                DroneMode::Conduit => "POWER".to_string(),
                DroneMode::Research => "RESEARCH".to_string(),
                DroneMode::Shipping => "METALS".to_string(),
                _ => "RESEARCH".to_string(),
            },
            drone_mode,
            fab: true,
        }
    }

    pub fn hovered(&mut self) -> bool {
        let p = pointer::screen().xy();
        self.hovered = self.hitbox.intersects_xy((p.0 as i32, p.1 as i32));
        self.hovered
    }

    pub fn inspecting(&mut self) -> bool {
        if self.drones > 0 {
            let p = pointer::screen().xy();
            self.inspecting = self.drone_inspect.bounds.intersects_xy((p.0 as i32, p.1 as i32)) || (self.inspecting && self.drone_hitbox.intersects_xy((p.0 as i32, p.1 as i32)));
        } else {
            self.inspecting = false;
        }
        self.inspecting
    }

    pub fn update(&mut self, anchor: Bounds, station: &Station, upgrades: &mut Vec<Upgrade>, upgrade_list: &Lazy<Vec<Upgrade>>, resources: &Vec<(Resources, u64)>) -> Option<Upgrade> {
        let mut upgraded = None;
        // Size based on available upgrades
        let mut height = upgrades.iter().map(|u| u.cost.len() as i32).sum::<i32>().max(upgrades.len() as i32);
        if self.fab && station.unlocked {
            height += 2;
            if height == 2 { height += 1; }
        }
        if height <= 0 { height = 1; }
        self.hitbox = self.hitbox.height(48 + height * 20);
        // Set position of fixed pop up bounds based on camera xyz
        let z = camera::z() as i32;
        let offset = (anchor.w() as i32 * z, anchor.h() as i32/2 * z - self.hitbox.h() as i32/2);
        self.hitbox = self.hitbox.position(
            offset.0 + screen().w() as i32/2 + z * anchor.x() - camera::x() as i32 * z,
            offset.1 + screen().h() as i32/2 + z * anchor.y() - camera::y() as i32 * z 
        );
        // if self.hitbox.y() < -10 {
        //     self.hitbox = self.hitbox.position(self.hitbox.x(), -10);
        // }
        // if self.hitbox.y() + 38 + height * 20 > 360 {
        //     self.hitbox = self.hitbox.position(self.hitbox.x(), 360 - 38 - height * 20);
        // }
        // Flip pop up to the left of POI if it goes off screen right
        if self.hitbox.x() + self.hitbox.w() as i32 > 640 {
            self.hitbox = self.hitbox.position(
                -(self.hitbox.w() as i32) + 320 + z * anchor.x() - camera::x() as i32 * z,
                self.hitbox.y()
            );
        }
        // Scale and position panel based on hitbox
        self.panel = self.panel
            .height(28 + height * 20)
            .position(
                self.hitbox.x() + 10,
                self.hitbox.center_y() - self.panel.h() as i32/2
            );
        if self.drones > 0 {
            if station.drone_stats.recall {
                self.unassaign.bounds = self.drone_panel
                    .width(36)
                    .height(16)
                    .position(
                        self.panel.right() - 80,
                        self.panel.y() + 3
                    );
                self.unassaign.update();
                if self.unassaign.on_click() {
                    upgraded = Some(UNASSIGN.clone());
                }
            }
            self.drone_inspect.bounds = self.panel
                .width(39)
                .height(16)
                .position(
                    self.panel.right() - 42,
                    self.panel.y() + 3,
                );
            self.drone_inspect.update();
            if self.inspecting { self.drone_inspect.state = BtnState::Hovered; }
            // Position drone hitbox based on panel
                self.drone_hitbox = self.hitbox
                    .width(130)
                    .height(116)
                    .position(
                        self.hitbox.x() + self.hitbox.w() as i32 - 17,
                        self.hitbox.y()
                    );
                // Flip drone inspect to the left of pop up panel if it goes off screen right
                if self.drone_hitbox.x() + self.drone_hitbox.w() as i32 > 650 {
                    self.drone_hitbox = self.drone_hitbox.position(
                        self.hitbox.x() + self.hitbox.w() as i32 - self.drone_hitbox.w() as i32 - 46,
                        self.drone_hitbox.y()
                    );
                }
                self.drone_panel = self.drone_hitbox
                    .width(self.drone_hitbox.w() - 20)
                    .height(self.drone_hitbox.h() - 20)
                    .position(
                        self.drone_hitbox.x() + 10,
                        self.drone_hitbox.y() + 10
                    );
            }


        let mut d = if self.fab && station.unlocked { 2 } else { 0 };
        
        for i in 0..upgrades.len() {
            if upgrades[i].level < upgrades[i].max_level {
                upgrades[i].array(self.panel, d);
                d+=1;
            }
        }

        // Update upgrade buttons
        if !self.inspecting {
            
            let mut index = 0;
            for i in 0..upgrades.len() {
                let upgrade = &mut upgrades[i];
                // Pass the players current resource value for the upgrade
                upgrade.update(resources);
                // Player purchases the upgrade
                if upgrade.on_click() {
                    upgraded = Some(upgrade.clone());
                    index = i;
                }
            }

            let mut maxed = false;
            if let Some(upgrade) = &mut upgraded {
                if upgrades.contains(upgrade) {
                    maxed = upgrades[index].next_level();
                    
                    if let UpgradeType::Purchase { btn: _ } = &mut upgrade.u_type {
                        // Push next level upgrade to avail_upgrades
                        if upgrade.unlocks.len() > 0 {
                            for i in 0..upgrade.unlocks.len() {
                                Upgrade::add_upgrade(upgrades, &upgrade_list, upgrade.unlocks[i], self.panel);
                            }
                            upgrades[index].unlocks = vec![]; // Clear unlocks after applying upgrade
                        }
                    }
                }
            }
            if maxed {
                upgrades.remove(index);
            }
        }
        upgraded
    }

    pub fn draw(&self, station: &Station, upgrades: &Vec<Upgrade>) {
        self.draw_pop_up(station, upgrades);
        self.draw_inspector(station);
    }

    fn draw_pop_up(&self, station: &Station, upgrades: &Vec<Upgrade>) {
               // Invisible hitbox for mouse hover detection
        // rect!(
        //     fixed = true, 
        //     xy = self.hitbox.xy(), 
        //     wh = self.hitbox.wh(), 
        //     color = 0xffffff33,
        // );
        // rect!(
        //     fixed = true, 
        //     xy = self.drone_hitbox.xy(), 
        //     wh = self.drone_hitbox.wh(), 
        //     color = 0xffffff33,
        // );
        rect!(
            fixed = true, 
            xy = self.panel.xy(), 
            wh = self.panel.wh(), 
            border_radius = 4,
            border_size = 1,
            color = 0x1f122bff,
            border_color = 0xffffffff,
        );
        
        text!(
            &self.title, 
            fixed = true, 
            xy = (self.panel.x() + 6, self.panel.y() + 7), 
            font = "large",
            color = 0xffffffff
        );  

        if self.drones > 0 {
            if station.drone_stats.recall {
                self.unassaign.draw();
            }

            self.drone_inspect.draw();
            let d = format!("{}", self.drones);
            text!(
                &d, 
                fixed = true, 
                xy = (self.panel.right() - 22, self.panel.y() + 7), 
                font = "large",
                color = 0xffffffff
            );
            sprite!("DRONES", fixed = true, xy = (self.panel.right() - 40, self.panel.y() + 3), w = 16, h = 16, color = 0xffffffff);
        }
        rect!(
            fixed = true, 
            x = self.panel.left() + 4,
            y = self.panel.top() + 21,
            wh = (self.panel.w() - 8, 1), 
            color = 0xffffffff,
        );

        for upgrade in upgrades.iter() {
            if upgrade.level < upgrade.max_level {
                upgrade.draw();
            }
        }
    }

    fn draw_inspector(&self, station: &Station) {
        if self.inspecting {
            rect!(
                fixed = true, 
                xy = self.drone_panel.xy(), 
                wh = self.drone_panel.wh(), 
                border_radius = 4,
                border_size = 1,
                color = 0x1f122bff,
                border_color = 0xffffffff,
            );
            let t = match self.drone_mode {
                DroneMode::Survey => "SURVEY",
                DroneMode::Mining => "MINING",
                DroneMode::Conduit => "CONDUIT",
                DroneMode::Research => "RESEARCH",
                DroneMode::Shipping => "SHIPPING",
                _ => "SURVEY"
            };
            text!(
                "{} DRONES", &t;
                fixed = true,
                xy = (self.drone_panel.left() + 4, self.drone_panel.top() + 4),
            );
            rect!(
                fixed = true, 
                xy = (self.drone_panel.left() + 3, self.drone_panel.top() + 15), 
                wh = (self.drone_panel.w() - 6, 1), 
                color = 0xffffffff,
            );

            let anchor = (self.drone_panel.left() + 2, self.drone_panel.top() + 20);
            // BOXES AND TITLES
            for i in 0..=2 {
                let t = match i {
                    0 => "BASE",
                    1 => "EFF.",
                    _ => "SPD.",
                };
                text!(
                    &t,
                    fixed = true,
                    xy = (anchor.0 + 6 + i * 37, anchor.1),
                );
                rect!(
                    fixed = true, 
                    xy = self.drone_panel.position(anchor.0 + i * 36, anchor.1 + 10).xy(), 
                    wh = (33, 15), 
                    border_radius = 4,
                    border_size = 1,
                    color = 0x1f122bff,
                    border_color = 0xffffffff,
                );
            }
            // BASE
            let t = format!("{}", station.drone_stats.base);
            text!(
                &t,
                fixed = true,
                xy = (anchor.0 + 17 - (t.len() as f32 * 2.5) as i32, anchor.1 + 13),
            );
            for x in -1..=1 {
                for y in -1..=1 {
                    if x == 0 && y == 0 { continue; }
                    text!(
                        "X",
                        fixed = true,
                        xy = (anchor.0 + 31 + x, anchor.1 + 13 + y),
                        font = "large",
                        color = 0x1f122bff
                    );
                }

            }
            text!(
                "X",
                fixed = true,
                xy = (anchor.0 + 31, anchor.1 + 13),
                font = "large",
            );
            // EFF
            let t = format!("{}%", (station.drone_stats.eff * station.drone_stats.mult * 100.0).round() as i32);
            text!(
                &t,
                fixed = true,
                xy = (anchor.0 + 53 - (t.len() as f32 * 2.5) as i32, anchor.1 + 13),
            );
            for x in -1..=1 {
                for y in -1..=1 {
                    if x == 0 && y == 0 { continue; }
                    text!(
                        "/",
                        fixed = true,
                        xy = (anchor.0 + 67 + x, anchor.1 + 12 + y),
                        font = "large",
                        color = 0x1f122bff
                    );
                }
            }
            text!(
                "/",
                fixed = true,
                xy = (anchor.0 + 67, anchor.1 + 12),
                font = "large",
            );

            // SPEED
            let interval = if self.drone_mode == DroneMode::Mining { station.drone_stats.interval + 300. } else { station.drone_stats.interval };
            let t = format!("{:.2}", ((interval * station.drone_stats.speed)/station.drone_stats.amped)/60.);
            text!(
                &t,
                fixed = true,
                xy = (anchor.0 + 88 - (t.len() as f32 * 2.5) as i32, anchor.1 + 13),
            );

            // Line break
            text!(
                "COUNT",
                fixed = true,
                xy = (anchor.0 + 16, anchor.1 + 28),
            );
            rect!(
                fixed = true, 
                xy = (anchor.0 + 7, anchor.1 + 38), 
                wh = (47, 18), 
                border_radius = 4,
                border_size = 1,
                color = 0x1f122bff,
                border_color = 0xffffffff,
            );
            sprite!(
                "DRONES", 
                fixed = true, 
                xy = (anchor.0 + 10, anchor.1 + 39),
                wh = (16, 16), 
                color = 0xffffffff,
            );
            text!(
                "{}", self.drones;
                fixed = true,
                xy = (anchor.0 + 28, anchor.1 + 43),
                font = "large",
            );
            for x in -1..=1 {
                for y in -1..=1 {
                    if x == 0 && y == 0 { continue; }
                    text!(
                        "X",
                        fixed = true,
                        xy = (anchor.0 + 2 + x, anchor.1 + 43 + y),
                        font = "large",
                        color = 0x1f122bff
                    );
                }
            }
            text!(
                "X",
                fixed = true,
                xy = (anchor.0 + 2, anchor.1 + 43),
                font = "large",
            );

            rect!(
                fixed = true, 
                xy = (anchor.0 + 57, anchor.1 + 38), 
                wh = (48, 35), 
                border_radius = 4,
                border_size = 1,
                color = 0x1f122bff,
                border_color = 0xffffffff,
            );

            text!(
                "PROD.",
                fixed = true,
                xy = (anchor.0 + 70, anchor.1 + 28),
            );
            
            let n = (self.drones as f32 * ((station.drone_stats.base * station.drone_stats.eff * station.drone_stats.mult) / (((interval * station.drone_stats.speed)/station.drone_stats.amped)/60.))) * 60.;
            let t = Numbers::format(n as u64);
            text!(
                &t,
                fixed = true,
                xy = (anchor.0 + 81 - (t.len() as f32 * 2.5) as i32, anchor.1 + 43),
            );
            let t = self.drone_resource.to_string();
            sprite!(
                &t, 
                fixed = true, 
                xy = (anchor.0 + 61, anchor.1 + 52), 
                wh = (16, 16), 
                color = 0xffffffff,
            );
            text!(
                "/min.",
                fixed = true,
                xy = (anchor.0 + 79, anchor.1 + 56),
            );

            for x in -1..=1 {
                for y in -1..=1 {
                    if x == 0 && y == 0 { continue; }
                    text!(
                        "=",
                        fixed = true,
                        xy = (anchor.0 + 52 + x, anchor.1 + 42 + y),
                        font = "large",
                        color = 0x1f122bff
                    );
                }
            }
            text!(
                "=",
                fixed = true,
                xy = (anchor.0 + 52, anchor.1 + 42),
                font = "large",
            );
        }
    }


    pub fn draw_fabricator(&self, station: &Station, upgrades: &Vec<Upgrade>, fab_prog: u64, fab_limit: u64) {
        self.draw_pop_up(station, upgrades);
        
        let bar = Bounds::new( 
            self.panel.x() + 4, 25 + self.panel.y(),
            self.panel.w() as u64 - 8, 18
        );
        ProgressBar::draw(bar, fab_prog as f32 / fab_limit as f32);

        rect!(
            fixed = true, 
            x = self.panel.left() + 4,
            y = self.panel.top() + 61,
            wh = (self.panel.w() - 8, 1), 
            color = 0xffffffff,
        );

        let t = if self.drones > 0 { "FABRICATING DRONE..." } else { "ASSIGN MAKER DRONE." };
        text!(&t, fixed = true, xy = (bar.x() + 4, bar.y() + 23), color = 0xffffffff);
        let t = format!("{}/{}", Numbers::format(fab_prog), Numbers::format(fab_limit));
        text!(
            &t, 
            fixed = true, 
            xy = (bar.x() + self.panel.w() as i32 - 8 - t.len() as i32 * 5, bar.y() + 23), 
            color = 0xffffffff
        );
        sprite!(
            "METALS", 
            fixed = true, 
            xy = (bar.x() + self.panel.w() as i32 - 28 - t.len() as i32 * 5, bar.y() + 19), 
            wh = (16, 16), 
        );
        self.draw_inspector(station);
    }

    pub fn draw_jumpgate(&self, station: &Station, upgrades: &Vec<Upgrade>, prestige_earn: u64, prestige_prog: u64, prestige_limit: u64) {
        self.draw_pop_up(station, upgrades);
        
        let bar = Bounds::new( 
            self.panel.x() + 4, 25 + self.panel.y(),
            self.panel.w() as u64 - 8, 18
        );
        ProgressBar::draw(bar, prestige_prog as f32 / prestige_limit as f32);

        
        rect!(
            fixed = true, 
            x = self.panel.left() + 4,
            y = self.panel.top() + 61,
            wh = (self.panel.w() - 8, 1), 
            color = 0xffffffff,
        );

        if prestige_limit != 0 {
            text!("COLLECTING RESOURCES...", fixed = true, xy = (bar.x() + 4, bar.y() + 23), color = 0xffffffff);
        } else {
            text!("MAX LEVEL PRESTIGE", fixed = true, xy = (bar.x() + 4, bar.y() + 23), color = 0xffffffff);
        }
        let t = format!("EARN {}", prestige_earn);
        text!(&t, fixed = true, xy = (bar.x() + bar.w() as i32 - 20 - t.len() as i32 * 5, bar.y() + 23), color = 0xffffffff);
        sprite!(
            "PRESTIGE",
            fixed = true,
            xy = (bar.x() + bar.w() as i32 - 20, bar.y() + 19),
        );

        self.draw_inspector(station);
    }

    pub fn draw_project(&self, poi: &dyn POI, fab_prog: u64, fab_limit: u64) {
        if let Some(complex) = poi.as_any().downcast_ref::<ResearchComplex>() {
            self.draw_pop_up(&complex.station, &complex.drone_upgrades);
            
            let bar = Bounds::new( 
                self.panel.x() + 4, 25 + self.panel.y(),
                self.panel.w() as u64 - 8, 18
            );
            ProgressBar::draw(bar, fab_prog as f32 / fab_limit as f32);
    
            rect!(
                fixed = true, 
                x = self.panel.left() + 4,
                y = self.panel.top() + 61,
                wh = (self.panel.w() - 8, 1), 
                color = 0xffffffff,
            );
    
            
            let t = 
                if let Some(project) = &complex.active_project {
                    let mut name = project.name.clone();
                    name.replace_range(0..9, "");
                    if self.drones > 0 { format!("RESEARCHING {}", name) } else { "ASSIGN RESEARCH DRONE.".to_string() }
                } else {
                    "ASSIGN A RESEARCH PROJECT.".to_string()
                };
            text!(&t, fixed = true, xy = (bar.x() + 4, bar.y() + 23), color = 0xffffffff);
            let t = format!("{}/{}", Numbers::format(fab_prog), Numbers::format(fab_limit));
            text!(
                &t, 
                fixed = true, 
                xy = (bar.x() + self.panel.w() as i32 - 8 - t.len() as i32 * 5, bar.y() + 23), 
                color = 0xffffffff
            );
            sprite!(
                "RESEARCH", 
                fixed = true, 
                xy = (bar.x() + self.panel.w() as i32 - 28 - t.len() as i32 * 5, bar.y() + 19), 
                wh = (16, 16), 
            );

            self.draw_inspector(&complex.station);
        }
    }

}