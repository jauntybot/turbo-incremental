use super::*;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
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
    drone_resource: Resources,
    fab: bool,
}
impl PopUp {
    pub fn new(title: String, resource: Resources) -> Self {
        let hitbox = Bounds::new(-336, -320, 224, 106);
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
            unassaign: Btn::new("-".to_string(), Bounds::new(0, 0, 0, 0), false, 1),
            inspecting: false,
            title,
            drones: 0,
            drone_resource: resource,
            fab: false,
        }
    }

    pub fn new_fab(title: String, resource: Resources) -> Self {
        let hitbox = Bounds::new(-336, -320, 224, 130);
        let panel = hitbox
            .inset(10);
        let mut drone_inspect = Btn::new("".to_string(), Bounds::new(0, 0, 0, 0), false, 0);
        drone_inspect.clickable = false;
        Self {
            hitbox,
            panel,
            drone_hitbox: hitbox,
            drone_panel: panel,
            drone_inspect,
            unassaign: Btn::new("-".to_string(), Bounds::new(0, 0, 0, 0), false, 1),
            inspecting: false,
            hovered: false,
            title,
            drones: 0,
            drone_resource: resource,
            fab: true,
        }
    }

    pub fn hovered(&mut self) -> bool {
        let p = pointer().xy_fixed();
        self.hovered = self.hitbox.intersects_xy((p.0 as i32, p.1 as i32));
        self.hovered
    }

    pub fn inspecting(&mut self) -> bool {
        if self.drones > 0 {
            let p = pointer().xy_fixed();
            self.inspecting = self.drone_inspect.bounds.intersects_xy((p.0 as i32, p.1 as i32)) || (self.inspecting && self.drone_hitbox.intersects_xy((p.0 as i32, p.1 as i32)));
        } else {
            self.inspecting = false;
        }
        self.inspecting
    }

    pub fn update(&mut self, anchor: Bounds, station: &Station, upgrades: &mut Vec<Upgrade>, upgrade_list: &Lazy<Vec<Upgrade>>, resources: &Vec<(Resources, u64)>) -> Option<Upgrade> {
        let mut upgraded = None;
        // Size based on available upgrades
        let mut height = upgrades.iter().map(|u| u.cost.len() as i32).sum::<i32>();
        if self.fab {
            height += 2;
        }
        if height <= 0 { height = 1; }
        self.hitbox = self.hitbox.height(48 + height * 20);
        // Set position of fixed pop up bounds based on camera xyz
        let z = camera::z() as i32;
        let offset = (anchor.w() as i32 * z, anchor.h() as i32/2 * z - self.hitbox.h() as i32/2);
        self.hitbox = self.hitbox.position(
            offset.0 + 320 + z * anchor.x() - camera::x() as i32 * z,
            offset.1 + 200 + z * anchor.y() - camera::y() as i32 * z 
        );
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
                // Flip pop up to the left of POI if it goes off screen right
                if self.hitbox.x() + self.hitbox.w() as i32 + self.drone_hitbox.w() as i32 > 640 {
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
                self.unassaign.bounds = self.drone_panel
                    .width(15)
                    .height(15)
                    .position(
                        self.drone_panel.left() + 22,
                        self.drone_panel.bottom() - 17
                    );
                self.unassaign.update();
                if self.unassaign.on_click() {
                    upgraded = Some(UNASSGIN.clone());
                }
            }


        let mut d = if self.fab { 2 } else { 0 };
        
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
                maxed = upgrades[index].next_level();
                // Push next level upgrade to avail_upgrades
                if upgrade.unlocks.len() > 0 {
                    for i in 0..upgrade.unlocks.len() {
                        Upgrade::add_upgrade(upgrades, &upgrade_list, upgrade.unlocks[i], self.panel);
                    }
                    upgrades[index].unlocks = vec![]; // Clear unlocks after applying upgrade
                }
            }
            if maxed {
                upgrades.remove(index);
            }
        }
        upgraded
    }

    pub fn update_fabricator(&mut self, anchor: Bounds, station: &Station, upgrades: &mut Vec<Upgrade>, upgrade_list: &Lazy<Vec<Upgrade>>, resources: &Vec<(Resources, u64)>) -> Option<Upgrade> {
        // Update upgrade buttons
        let mut upgraded = self.update(anchor, station, upgrades, upgrade_list, resources);
        
        upgraded
    }

    pub fn draw(&self, station: &Station, upgrades: &Vec<Upgrade>) {
        // Invisible hitbox for mouse hover detection
        rect!(
            fixed = true, 
            xy = self.hitbox.xy(), 
            wh = self.hitbox.wh(), 
            color = 0xffffff33,
        );
        rect!(
            fixed = true, 
            xy = self.drone_hitbox.xy(), 
            wh = self.drone_hitbox.wh(), 
            color = 0xffffff33,
        );
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

        // Drone inspector
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
            text!(
                "SURVEY DRONES",
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
            let t = format!("{}", station.drone_base);
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
            let t = format!("{}%", (station.drone_eff * 100.0).round() as i32);
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
            let t = format!("{:.2}", station.drone_speed / 60.);
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
            self.unassaign.draw();

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
            let t = Numbers::format((self.drones as f32 * ((station.drone_base * station.drone_eff) / (station.drone_speed / 60.))) as u64);
            text!(
                &t,
                fixed = true,
                xy = (anchor.0 + 80 - (t.len() as f32 * 2.5) as i32, anchor.1 + 43),
            );
            let t = self.drone_resource.to_string();
            sprite!(
                &t, 
                fixed = true, 
                xy = (anchor.0 + 60, anchor.1 + 52), 
                wh = (16, 16), 
                color = 0xffffffff,
            );
            text!(
                "/sec.",
                fixed = true,
                xy = (anchor.0 + 78, anchor.1 + 56),
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
        self.draw(station, upgrades);

        rect!(
            fixed = true, 
            x = self.panel.left() + 4,
            y = self.panel.top() + 64,
            wh = (self.panel.w() - 8, 1), 
            color = 0xffffffff,
        );

        let bar = ( 
            self.panel.x() + 4, 24 + self.panel.y(),
            (self.panel.w() as u64 - 8) * fab_prog / fab_limit, 20
        );
        rect!(
            fixed = true, 
            xy = (bar.0, bar.1), 
            wh = (self.panel.w() - 8, bar.3), 
            border_radius = 4,
            border_size = 1,
            color = 0x1f122bff,
            border_color = 0xffffffff,
        );
        rect!(
            fixed = true, 
            xy = (bar.0 + 2, bar.1 + 2), 
            wh = (bar.2 - 4, bar.3 - 4), 
            border_radius = 4,
            border_size = 1,
            color = 0x1f122bff,
            border_color = 0xffffffff,
        );
        rect!(
            fixed = true, 
            xy = (bar.0 + 3, bar.1 + 3), 
            wh = (bar.2 - 6, bar.3 - 6), 
            border_radius = 4,
            border_size = 1,
            color = 0xffffffff,
            border_color = 0x1f122bff,
        );
        let t = if self.drones > 0 { "FABRICATING DRONE..." } else { "ASSIGN MAKER DRONE." };
        text!(&t, fixed = true, xy = (bar.0 + 4, bar.1 + 26), color = 0xffffffff);
        let t = format!("{}/{}", Numbers::format(fab_prog), Numbers::format(fab_limit));
        text!(
            &t, 
            fixed = true, 
            xy = (bar.0 + self.panel.w() as i32 - 8 - t.len() as i32 * 5, bar.1 + 26), 
            color = 0xffffffff
        );
        sprite!(
            "METALS", 
            fixed = true, 
            xy = (bar.0 + self.panel.w() as i32 - 28 - t.len() as i32 * 5, bar.1 + 22), 
            wh = (16, 16), 
        );
    }

    pub fn draw_jumpgate(&self, station: &Station, upgrades: &Vec<Upgrade>, prestige_earn: u64, prestige_prog: u64, prestige_limit: u64) {
        self.draw(station, upgrades);

        rect!(
            fixed = true, 
            x = self.panel.left() + 4,
            y = self.panel.top() + 64,
            wh = (self.panel.w() - 8, 1), 
            color = 0xffffffff,
        );

        let bar = ( 
            self.panel.x() + 4, 24 + self.panel.y(),
            (self.panel.w() as u64 - 8) * prestige_prog / prestige_limit, 20
        );
        rect!(
            fixed = true, 
            xy = (bar.0, bar.1), 
            wh = (self.panel.w() - 8, bar.3), 
            border_radius = 4,
            border_size = 1,
            color = 0x1f122bff,
            border_color = 0xffffffff,
        );
        rect!(
            fixed = true, 
            xy = (bar.0 + 2, bar.1 + 2), 
            wh = (bar.2 - 4, bar.3 - 4), 
            border_radius = 4,
            border_size = 1,
            color = 0x1f122bff,
            border_color = 0xffffffff,
        );
        rect!(
            fixed = true, 
            xy = (bar.0 + 3, bar.1 + 3), 
            wh = (bar.2 - 6, bar.3 - 6), 
            border_radius = 4,
            border_size = 1,
            color = 0xffffffff,
            border_color = 0x1f122bff,
        );
        let t = format!("EARN {}", prestige_earn);
        text!(&t, fixed = true, xy = (bar.0 + 4, bar.1 + 26), color = 0xffffffff);
        sprite!(
            "PRESTIGE",
            fixed = true,
            xy = (bar.0 + 4 + t.len() as i32 * 5, bar.1 + 22),
        );
    }

}