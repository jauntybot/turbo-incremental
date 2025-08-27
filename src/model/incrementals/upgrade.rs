use super::*;
use once_cell::sync::Lazy;

#[turbo::serialize]
#[derive(PartialEq)]
pub struct Upgrade {
    pub name: String,
    pub description: String,
    pub cost: Vec<(Resources, u64)>,
    pub unlocks: Vec<usize>, // Which index of the upgrade tree this upgrade leads to
    pub level: u32,
    pub max_level: u32,

    // Drawing variables
    pub entry: Btn,
    pub u_type: UpgradeType,
    pub tooltip: WrapBox,
    pub hovered: bool,
    pub display_lvl: bool,

    // Function to calculate the cost of the upgrade based on level
    pub base_cost: Vec<(Resources, u64)>,
    pub cost_formula: CostFormula,
}

#[turbo::serialize]
#[derive(PartialEq)]
pub enum UpgradeType {
    Purchase { btn: Btn },
    Toggle { toggle: Toggle },
}

#[turbo::serialize]
#[derive(PartialEq)]
pub enum CostFormula {
    None,
    Linear { factor: f32 },
    Compounding {factor: f32},
    Exponential {factor: f32},
}
impl CostFormula {
    pub fn calculate_cost(&self, base_cost: Vec<(Resources, u64)>, n: u32) -> Vec<(Resources, u64)> {
        match self {
            CostFormula::None => {
                base_cost
            }
            CostFormula::Linear { factor } => {
                let mut new_cost = vec![];
                for cost in base_cost.iter() {
                    let prod = (cost.1 as f32 + factor * n as f32) as u64;
                    new_cost.push((cost.0.clone(), prod));
                }
                new_cost
            }
            CostFormula::Compounding { factor } => {
                let mut new_cost = vec![];
                for cost in base_cost.iter() {
                    let prod = cost.1 * (2u64.pow(n));
                    new_cost.push((cost.0.clone(), prod));
                }
                new_cost
            }
            CostFormula::Exponential { factor} => {
                let mut new_cost = vec![];
                for cost in base_cost.iter() {
                    let prod= (cost.1 as f32 * factor.powf(n as f32)) as u64;
                    // if n <= 5 {
                    //     prod = (cost.1 as f32 * (1.07 as f32).powf(n as f32)) as u64;
                    // } else if n <= 20 {
                    //     prod = (cost.1 as f32 * (1.1 as f32).powf(n as f32)) as u64;
                    // } else if n <= 40 {
                    //     prod = (cost.1 as f32 * (1.15 as f32).powf(n as f32)) as u64;
                    // } else {
                    //     prod = (cost.1 as f32 * (1.2 as f32).powf(n as f32)) as u64;
                    // } 

                    new_cost.push((cost.0.clone(), prod));
                }
                new_cost
            }
        }
    }
}

impl Upgrade {
    pub fn add_upgrade(mut_list: &mut Vec<Upgrade>, upgrade_list: &Lazy<Vec<Upgrade>>, index: usize, pop_up: Bounds) {
        if index < upgrade_list.len() {
            let upgrade = upgrade_list[index].clone()
                .init(pop_up, mut_list.len());
            mut_list.push(upgrade);
        }
    }

    pub fn init(&mut self, pop_up: Bounds, index: usize) -> Self {
        if let UpgradeType::Purchase { btn } = &mut self.u_type {
            self.cost = self.base_cost.clone();
        } else if !self.base_cost.is_empty(){
            self.cost = self.base_cost.clone();
            self.base_cost[0].1 = 0;
        }
        let h = 
            if self.cost.len() > 0 { self.cost.len() as i32 * 20 }
            else { 20 };
        self.entry.clickable = false;
        self.entry.fixed = true;
        self.entry.bounds = pop_up
            .inset(4)
            .height(h);

        if let UpgradeType::Purchase { btn } = &mut self.u_type {
            btn.interactable = false;
            btn.bounds = btn.bounds
                .height(15)
                .width(15);
        } else if let UpgradeType::Toggle { toggle } = &mut self.u_type {
            toggle.bounds = toggle.bounds
                .height(14)
                .width(21);
        }

        self.tooltip = WrapBox::new(self.description.clone(), 0);

        self.array(pop_up, index);
        return self.clone();
    }

    pub fn array(&mut self, bounds: Bounds, index: usize) {
        self.entry.bounds = self.entry.bounds.position(
            bounds.x() + 4,
            24 + bounds.y() + index as i32 * 20,
        );

        if let UpgradeType::Purchase { btn } = &mut self.u_type {
            btn.bounds = btn.bounds
                .anchor_right(&self.entry.bounds)
                .translate_x(-68)
                .anchor_center_y(&self.entry.bounds);
        } else if let UpgradeType::Toggle { toggle } = &mut self.u_type {
            toggle.bounds = toggle.bounds
                .anchor_right(&self.entry.bounds)
                .translate_x(-65)
                .anchor_center_y(&self.entry.bounds);
        }

        self.tooltip.update(self.entry.bounds, 8);
    }


    pub fn update(&mut self, resources: &Vec<(Resources, u64)>) {
        self.entry.update();
        self.hovered = self.entry.state == BtnState::Hovered;

        if let UpgradeType::Purchase { btn } = &mut self.u_type {
            let mut buyable = false;
            if self.level < self.max_level {
                buyable = true;
                let mut has_resources = true;
                for cost in self.cost.iter() {
                    if cost.0 == Resources::Ad {
                        buyable = cost.1 <= 0;
                    } else if resources.len() == 0 {
                        buyable = false;
                    } else {
                        let mut found = false;
                        for resource in resources.iter() {
                            if resource.0 == cost.0 {
                                found = true;
                                if resource.1 < cost.1 {
                                    buyable = false;
                                }
                            } 
                        }
                        if !found {
                            has_resources = false;
                        }
                    }
                }
                if !has_resources {
                    buyable = false;
                }
            }
            btn.interactable = buyable;
            btn.update();
        } else if let UpgradeType::Toggle { toggle } = &mut self.u_type {
            toggle.interactable = true;
            toggle.update();
        }
    }

    pub fn on_click(&mut self) -> bool {
        match &mut self.u_type {
            UpgradeType::Purchase { btn } => {
                btn.on_click()
            } 
            UpgradeType::Toggle { toggle } => {
                toggle.on_click()
            }
        }
    }

    pub fn next_level(&mut self) -> bool {
        if let UpgradeType::Toggle { .. } = self.u_type {
            return false; // Toggle upgrades don't level up
        }
        self.level += 1;
        if self.level >= self.max_level {
            //self.entry.interactable = false;
            self.level = self.max_level;
            if let UpgradeType::Purchase { btn } = &mut self.u_type {
                btn.interactable = false;
            }
            return true;
        } else {
            self.cost = self.cost_formula.calculate_cost(self.base_cost.clone(), self.level);
        }
        false
    }

    pub fn draw(&self, ) {
        self.entry.draw();
        let mut t = format!("{}", self.name);
        if self.display_lvl {
            t = format!("{} LVL {}", self.name, self.level + 1);
        }
        text!(&t, fixed = true, x = self.entry.bounds.x() + 4, y = self.entry.bounds.center_y() - 4);
        
        if let UpgradeType::Purchase { btn } = &self.u_type {
            btn.draw();
        } else if let UpgradeType::Toggle { toggle } = &self.u_type {
            toggle.draw();
        }

        let mut i = 0;
        let mut o = 0;
        for (resource, amount) in self.cost.iter() {
            let sprite = format!("{}", resource);
            sprite!(&sprite, fixed = true, x = self.entry.bounds.right() - 58, y = i * 20 + self.entry.bounds.y() + 2, wh = (16, 16), color = 0xffffffff);
            let abbr = if resource != &Resources::Ad { 
                Numbers::format(amount.clone())
            } else {
                if amount == &0 {
                    o = 3;
                    "WATCH AD".to_string()
                } else {
                    Numbers::time(amount.clone())
                }
            };
            text!("{}", abbr; fixed = true, x = self.entry.bounds.right() as i32 - 38 - o, y = i * 20 + self.entry.bounds.y() + 6);
            i += 1;
        }
        


        if self.hovered {
            self.tooltip.draw();
        }
    }
}
