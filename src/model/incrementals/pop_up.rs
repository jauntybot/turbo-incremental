use super::*;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct PopUp {
    pub hitbox: Bounds,
    pub panel: Bounds,
    title: String,
    pub drones: u32,
    fab: bool,
}
impl PopUp {
    pub fn new(title: String) -> Self {
        let hitbox = Bounds::new(-336, -320, 224, 106);
        let panel = hitbox
            .inset(10);
        Self {
            hitbox,
            panel,
            title,
            drones: 0,
            fab: false,
        }
    }

    pub fn new_fab(title: String) -> Self {
        let hitbox = Bounds::new(-336, -320, 224, 130);
        let panel = hitbox
            .inset(10);
        Self {
            hitbox,
            panel,
            title,
            drones: 0,
            fab: true,
        }
    }

    pub fn hovered(&self) -> bool {
        let p = pointer().xy_fixed();
        self.hitbox.intersects_xy((p.0 as i32, p.1 as i32))
    }

    pub fn update(&mut self, anchor: Bounds, upgrades: &mut Vec<Upgrade>, upgrade_list: &Lazy<Vec<Upgrade>>, resources: &Vec<(Resources, u64)>) -> Option<Upgrade> {
        let mut height = upgrades.iter().map(|u| u.cost.len() as i32).sum::<i32>();
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
        self.panel = self.panel
        .height(28 + height * 20)
        .position(
            self.hitbox.x() + 10,
            self.hitbox.center_y() - self.panel.h() as i32/2
        );

        let mut d = 0;
        for i in 0..upgrades.len() {
            if upgrades[i].level < upgrades[i].max_level {
                upgrades[i].array(self.panel, d);
                d+=1;
            }
        }

        // Update upgrade buttons
        let mut upgraded = None;
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
        upgraded
    }

    pub fn update_fabricator(&mut self, anchor: Bounds, upgrades: &mut Vec<Upgrade>, upgrade_list: &Lazy<Vec<Upgrade>>, resources: &Vec<(Resources, u64)>) -> Option<Upgrade> {
        let height = upgrades.iter().map(|u| u.cost.len() as i32).sum::<i32>() + 2;
        self.hitbox = self.hitbox.height(52 + height * 20);
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
        self.panel = self.panel
        .height(32 + height * 20)
        .position(
            self.hitbox.x() + 10,
            self.hitbox.center_y() - self.panel.h() as i32/2
        );

        let mut d = 0;
        for i in 0..upgrades.len() {
            if upgrades[i].level < upgrades[i].max_level {
                upgrades[i].array(self.panel.translate_y(4), d + 2);
                d+=1;
            }
        }

        // Update upgrade buttons
        let mut upgraded = None;
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
        upgraded
    }

    pub fn draw(&self, upgrades: &Vec<Upgrade>) {
        // Invisible hitbox for mouse hover detection
        // rect!(
        //     fixed = true, 
        //     xy = self.hitbox.xy(), 
        //     wh = self.hitbox.wh(), 
        //     color = 0xffffffff,
        // );
        rect!(
            fixed = true, 
            xy = self.panel.xy(), 
            wh = self.panel.wh(), 
            border_radius = 4,
            border_size = 1,
            color = 0x222034ff,
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
            y = self.panel.top() + 20,
            wh = (self.panel.w() - 8, 1), 
            color = 0xffffffff,
        );

        for upgrade in upgrades.iter() {
            if upgrade.level < upgrade.max_level {
                upgrade.draw();
            }
        }
    }


    pub fn draw_fabricator(&self, upgrades: &Vec<Upgrade>, fab_prog: u64, fab_limit: u64) {
        self.draw(upgrades);

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
            color = 0x222034ff,
            border_color = 0xffffffff,
        );
        rect!(
            fixed = true, 
            xy = (bar.0 + 2, bar.1 + 2), 
            wh = (bar.2 - 4, bar.3 - 4), 
            border_radius = 4,
            border_size = 1,
            color = 0x222034ff,
            border_color = 0xffffffff,
        );
        rect!(
            fixed = true, 
            xy = (bar.0 + 3, bar.1 + 3), 
            wh = (bar.2 - 6, bar.3 - 6), 
            border_radius = 4,
            border_size = 1,
            color = 0xffffffff,
            border_color = 0x222034ff,
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

}