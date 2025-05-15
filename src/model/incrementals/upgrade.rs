use super::*;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Upgrade {
    pub name: String,
    pub description: String,
    pub cost: Vec<(Resources, u64)>,
    pub unlocks: Vec<usize>, // Which index of the upgrade tree this upgrade leads to
    pub level: u32,
    pub max_level: u32,

    // Drawing variables
    pub entry: Btn,
    pub buy_button: Btn,
    pub tooltip: WrapBox,
    pub hovered: bool,
    pub display_lvl: bool,

    // Function to calculate the cost of the upgrade based on level
    pub base_cost: Vec<(Resources, u64)>,
    pub cost_formula: CostFormula,
}

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub enum CostFormula {
    None,
    Double,
    Exponential,
}
impl CostFormula {
    pub fn calculate_cost(&self, base_cost: Vec<(Resources, u64)>, n: u32) -> Vec<(Resources, u64)> {
        match self {
            CostFormula::None => {
                base_cost
            }
            CostFormula::Double => {
                let mut new_cost = vec![];
                for cost in base_cost.iter() {
                    let prod = cost.1 * (2u64.pow(n));
                    new_cost.push((cost.0.clone(), prod));
                }
                new_cost
            }
            CostFormula::Exponential => {
                let mut new_cost = vec![];
                for cost in base_cost.iter() {
                    let prod = (cost.1 as f64 * (1.2f64).powi(n as i32)).ceil() as u64;
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
            let mut upgrade = upgrade_list[index].clone();
            upgrade.init(pop_up, mut_list.len());
            mut_list.push(upgrade);
        }
    }

    pub fn init(&mut self, pop_up: Bounds, index: usize) -> Self {
        let h = 
            if self.cost.len() > 0 { self.cost.len() as i32 * 20 }
            else { 20 };
        self.entry.clickable = false;
        self.entry.fixed = true;
        self.entry.bounds = pop_up
            .inset(4)
            .height(h);

        self.buy_button.interactable = false;
        self.buy_button.bounds = self.buy_button.bounds
            .height(15)
            .width(15);

        self.tooltip = WrapBox::new(self.description.clone(), 0);

        self.array(pop_up, index);
        return self.clone();
    }

    pub fn array(&mut self, bounds: Bounds, index: usize) {
        self.entry.bounds = self.entry.bounds.position(
            bounds.x() + 4,
            24 + bounds.y() + index as i32 * 20,
        );

        self.buy_button.bounds = self.buy_button.bounds
            .anchor_right(&self.entry.bounds)
            .translate_x(-64)
            .anchor_center_y(&self.entry.bounds);

        self.tooltip.update(self.entry.bounds, 8);
    }


    pub fn update(&mut self, resources: &Vec<(Resources, u64)>) {
        self.entry.update();
        self.hovered = self.entry.state == BtnState::Hovered;

        let mut buyable = false;
        if self.level < self.max_level {
            buyable = true;
            let mut has_resources = true;
            for cost in self.cost.iter() {
                if resources.len() == 0 {
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
        self.buy_button.interactable = buyable;

        self.buy_button.update();
    }

    pub fn on_click(&self) -> bool {
        self.buy_button.on_click()
    }

    pub fn next_level(&mut self) -> bool {
        self.level += 1;
        if self.level >= self.max_level {
            //self.entry.interactable = false;
            self.level = self.max_level;
            self.buy_button.interactable = false;
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
        
        if self.level < self.max_level {
            self.buy_button.draw();
            let mut i = 0;
            for (resource, amount) in self.cost.iter() {
                let sprite = format!("{}", resource);
                sprite!(&sprite, fixed = true, x = self.entry.bounds.right() - 58, y = i * 20 + self.entry.bounds.y() + 2, wh = (16, 16), color = 0xffffffff);
                let abbr = Numbers::format(amount.clone());
                text!("{}", abbr; fixed = true, x = self.entry.bounds.right() as i32 - 38, y = i * 20 + self.entry.bounds.y() + 6);
                i += 1;
            }
        }


        if self.hovered {
            self.tooltip.draw();
        }
    }
}
