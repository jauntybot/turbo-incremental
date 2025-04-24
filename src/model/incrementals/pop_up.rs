use super::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct PopUp {
    hitbox: Bounds,
    pub panel: Bounds,
    title: String,
    pub drones: u32,
}
impl PopUp {
    pub fn new(title: String) -> Self {
        Self {
            hitbox: Bounds::new(-320, -320, 196, 96),
            panel: Bounds::new(-320,-320,0,0),
            title,
            drones: 0,
        }
    }

    pub fn hovered(&self) -> bool {
        let p = pointer().fixed_position();
        self.hitbox.intersects_xy((p.0 as i32, p.1 as i32))
    }



    pub fn update(&mut self, anchor: Bounds, upgrades: &mut Vec<Upgrade>, upgrade_list: &Vec<Upgrade>, resources: &Vec<(Resources, u32)>) -> Option<Upgrade> {
        // Set position of fixed pop up bounds based on camera xyz
        let z = camera::z() as i32;
        let offset = (anchor.w() as i32 * z, anchor.h() as i32/2 * z - self.hitbox.h() as i32/2);
        self.hitbox = self.hitbox.position(
            offset.0 + 320 + z * anchor.x() - camera::x() as i32 * z,
            offset.1 + 240 + z * anchor.y() - camera::y() as i32 * z 
        );
        self.panel = self.hitbox.inset(10);
        self.panel = self.panel.adjust_height(4);
        for i in 0..upgrades.len() {
            upgrades[i].init(self.panel, i);
        }

        // Update upgrade buttons
        let mut upgraded = None;
        for (_, upgrade) in upgrades.iter_mut().enumerate() {
            // Pass the players current resource value for the upgrade
            let default_resource = (upgrade.cost.0.clone(), 0);
            let resource = 
                resources.iter().find(|r| r.0 == upgrade.cost.0)
                .unwrap_or(&default_resource);
            upgrade.update(resource.1);
            // Player purchases the upgrade
            if upgrade.on_click() {
                upgraded = Some(upgrade.clone());
            }
        }
    
        if let Some(upgrade) = &upgraded {
            if upgrade.limited { // Flag for removal
                upgrades.remove(upgrades.iter().position(|u| u == upgrade).unwrap());
            }
            // Push next level upgrade to avail_upgrades
            if upgrade.unlocks.len() > 0 {
                for index in 0..upgrade.unlocks.len() {
                    if upgrade_list.len() as u32 >= upgrade.unlocks[index] as u32 {
                        upgrades.push(upgrade_list[upgrade.unlocks[index]].clone());
                    }
                }
            }
        }
        upgraded
    }

    pub fn draw(&self, upgrades: &Vec<Upgrade>) {
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
            xy = (self.panel.x() + 4, self.panel.y() + 6), 
            font = "large",
            color = 0xffffffff
        );  

        if self.drones > 0 {
            let d = format!("{}", self.drones);
            text!(
                &d, 
                fixed = true, 
                xy = (self.panel.right() - 22, self.panel.y() + 5), 
                font = "large",
                color = 0xffffffff
            );
            sprite!("DRONES", fixed = true, xy = (self.panel.right() - 40, self.panel.y() + 1), w = 16, h = 16, color = 0xffffffff);
        }
        rect!(
            fixed = true, 
            x = self.panel.left() + 4,
            y = self.panel.top() + 18,
            wh = (self.panel.w() - 8, 1), 
            color = 0xffffffff,
        );

        for upgrade in upgrades.iter() {
            upgrade.draw();
        }
    }

}