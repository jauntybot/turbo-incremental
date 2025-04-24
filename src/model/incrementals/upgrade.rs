use super::*;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Upgrade {
    pub name: String,
    pub description: String,
    pub cost: (Resources, u32),
    pub unlocks: Vec<usize>, // Which index of the upgrade tree this upgrade leads to
    pub limited: bool,

    // Drawing variables
    pub entry: Btn,
    pub buy_button: Btn,
    pub tooltip: Bounds,
    pub hovered: bool,
}

impl Upgrade {
    pub fn init(&mut self, pop_up: Bounds, index: usize) {
        self.entry.clickable = false;
        self.entry.fixed = true;
        self.entry.bounds = pop_up.inset(4);
        self.entry.bounds = self.entry.bounds.height(18);

        self.buy_button.interactable = false;
        self.buy_button.bounds = self.buy_button.bounds.height(13);
        self.buy_button.bounds = self.buy_button.bounds.width(20);
        
        self.tooltip = self.tooltip.width(128);
        self.tooltip = self.tooltip.height(96);
        self.array(pop_up, index);
    }

    pub fn array(&mut self, bounds: Bounds, index: usize) {
        self.entry.bounds = self.entry.bounds.position(
            bounds.x() + 4,
            16 + bounds.y() + 4 + index as i32 * (self.entry.bounds.h() as i32) as i32,
        );

        self.buy_button.bounds = self.buy_button.bounds.anchor_right(&self.entry.bounds);
        self.buy_button.bounds = self.buy_button.bounds.translate_x(-48);
        self.buy_button.bounds = self.buy_button.bounds.anchor_center_y(&self.entry.bounds);

        self.tooltip = self.tooltip.position(
            if bounds.x() >= 320 { bounds.left() - self.tooltip.w() as i32 - 4 } else { bounds.right() + 4 },
            bounds.center_y() - 48,
        );
    }


    pub fn update(&mut self, resources: u32) {
        self.entry.update();
        self.hovered = self.entry.state == BtnState::Hovered;

        if resources >= self.cost.1 {
            self.buy_button.interactable = true;
        } else {
            self.buy_button.interactable = false;
        }

        self.buy_button.update();
    }

    pub fn on_click(&self) -> bool {
        self.buy_button.on_click()
    }

    pub fn draw(&self, ) {
        self.entry.draw();
        text!("{}", self.name; fixed = true, x = self.entry.bounds.x() + 4, y = self.entry.bounds.center_y() - 4);
        
        self.buy_button.draw();

        let sprite = format!("{}", self.cost.0);
        sprite!(&sprite, fixed = true, x = self.entry.bounds.right() - 42, y = self.entry.bounds.center_y() - 8, wh = (16, 16), color = 0xffffffff);
        text!("{}", self.cost.1; fixed = true, x = self.entry.bounds.right() as i32 - 26, y = self.entry.bounds.center_y() - 4);

        if self.hovered {
            rect!(fixed = true, xy = self.tooltip.xy(), wh = self.tooltip.wh(), border_size = 1, border_radius = 4, color = 0x222034ff, border_color = 0xffffffff);
            text!("{}", self.description; fixed = true, xy = (self.tooltip.x() + 4, self.tooltip.y() + 4), color = 0xffffffff);
        }
    }
}

pub static EXOPLANET_UPGRADES: Lazy<Vec<Upgrade>> = Lazy::new(|| vec![
    Upgrade {
        name: "FIELD SCANNER".to_string(),
        description: "Increase the amount of research gained by pressing the Exoplanet by 100%.".to_string(),
        cost: (Resources::Research, 10),
        entry: Btn::new("".to_string(), Bounds::new(-320, -320, 0, 0), true, 0),
        limited: true,
        unlocks: vec![1],
        buy_button: Btn::buy(),
        tooltip: Bounds::new(-320, -320, 0, 0),
        hovered: false,
    },
    Upgrade {
        name: "FIELD SCANNER 02".to_string(),
        description: "Increase the amount of research gained by pressing the Exoplanet by 100%.".to_string(),
        cost: (Resources::Research, 10),
        entry: Btn::new("".to_string(), Bounds::new(-320, -320, 0, 0), true, 0),
        limited: true,
        unlocks: vec![2],
        buy_button: Btn::buy(),
        tooltip: Bounds::new(-320, -320, 0, 0),
        hovered: false,
    },
    Upgrade {
        name: "FIELD SCANNER 03".to_string(),
        description: "Increase the amount of research gained by pressing the Exoplanet by 100%.".to_string(),
        cost: (Resources::Research, 10),
        entry: Btn::new("".to_string(), Bounds::new(-320, -320, 0, 0), true, 0),
        limited: true,
        unlocks: vec![3],
        buy_button: Btn::buy(),
        tooltip: Bounds::new(-320, -320, 0, 0),
        hovered: false,
    },
    Upgrade {
        name: "FIELD SCANNER 04".to_string(),
        description: "Increase the amount of research gained by pressing the Exoplanet by 100%.".to_string(),
        cost: (Resources::Research, 10),
        entry: Btn::new("".to_string(), Bounds::new(-320, -320, 0, 0), true, 0),
        limited: true,
        unlocks: vec![],
        buy_button: Btn::buy(),
        tooltip: Bounds::new(-320, -320, 0, 0),
        hovered: false,
    },
    Upgrade {
        name: "DEPLOY SURVEY DRONE".to_string(),
        description: "Assign a DRONE to collect RESEARCH.".to_string(),
        cost: (Resources::Drones, 1),
        entry: Btn::new("".to_string(), Bounds::new(-320, -320, 0, 0), true, 0),
        limited: false,
        unlocks: vec![],
        buy_button: Btn::buy(),
        tooltip: Bounds::new(-320, -320, 0, 0),
        hovered: false,
    },
    Upgrade {
        name: "ADV. SENSORS 01".to_string(),
        description: "Increase the amount of research gained by pressing the Exoplanet by 100%.".to_string(),
        cost: (Resources::Metals, 10),
        entry: Btn::new("".to_string(), Bounds::new(-320, -320, 0, 0), true, 0),
        limited: true,
        unlocks: vec![6],
        buy_button: Btn::buy(),
        tooltip: Bounds::new(-320, -320, 0, 0),
        hovered: false,
    },
    Upgrade {
        name: "ADV. SENSORS 02".to_string(),
        description: "Increase the amount of research gained by pressing the Exoplanet by 100%.".to_string(),
        cost: (Resources::Metals, 10),
        entry: Btn::new("".to_string(), Bounds::new(-320, -320, 0, 0), true, 0),
        limited: true,
        unlocks: vec![7],
        buy_button: Btn::buy(),
        tooltip: Bounds::new(-320, -320, 0, 0),
        hovered: false,
    },
    Upgrade {
        name: "ADV. SENSORS 03".to_string(),
        description: "Increase the amount of research gained by pressing the Exoplanet by 100%.".to_string(),
        cost: (Resources::Metals, 10),
        entry: Btn::new("".to_string(), Bounds::new(-320, -320, 0, 0), true, 0),
        limited: true,
        unlocks: vec![],
        buy_button: Btn::buy(),
        tooltip: Bounds::new(-320, -320, 0, 0),
        hovered: false,
    },
]);

pub static DEPOT_UPGRADES: Lazy<Vec<Upgrade>> = Lazy::new(|| vec![
    Upgrade {
        name: "CONSTRUCT".to_string(),
        description: "Increase the amount of RESEARCH gained by pressing the Exoplanet by 100%.".to_string(),
        cost: (Resources::Research, 10),
        entry: Btn::new("".to_string(), Bounds::new(-320, -320, 0, 0), true, 0),
        limited: true,
        unlocks: vec![1],
        buy_button: Btn::buy(),
        tooltip: Bounds::new(-320, -320, 0, 0),
        hovered: false,
    },
    Upgrade {
        name: "DRONE SHIPMENT".to_string(),
        description: "Exchange RESEARCH for a DRONE.".to_string(),
        cost: (Resources::Research, 10),
        entry: Btn::new("".to_string(), Bounds::new(-320, -320, 0, 0), true, 0),
        buy_button: Btn::buy(),
        limited: false,
        unlocks: vec![],
        tooltip: Bounds::new(-320, -320, 0, 0),
        hovered: false,
    },
    Upgrade {
        name: "FIELD SCANNER 03".to_string(),
        description: "Increase the amount of RESEARCH gained by pressing the Exoplanet by 100%.".to_string(),
        cost: (Resources::Research, 10),
        entry: Btn::new("".to_string(), Bounds::new(-320, -320, 0, 0), true, 0),
        limited: true,
        unlocks: vec![],
        buy_button: Btn::buy(),
        tooltip: Bounds::new(-320, -320, 0, 0),
        hovered: false,
    },
]);

pub static MINES_UPGRADES: Lazy<Vec<Upgrade>> = Lazy::new(|| vec![
    Upgrade {
        name: "CONSTRUCT".to_string(),
        description: "Construct ASTEROID MINES.".to_string(),
        cost: (Resources::Research, 10),
        entry: Btn::new("".to_string(), Bounds::new(-320, -320, 0, 0), true, 0),
        limited: true,
        unlocks: vec![1],
        buy_button: Btn::buy(),
        tooltip: Bounds::new(-320, -320, 0, 0),
        hovered: false,
    },
    Upgrade {
        name: "DEPLOY MINING DRONE".to_string(),
        description: "Assign a DRONE to collect METALS.".to_string(),
        cost: (Resources::Drones, 1),
        entry: Btn::new("".to_string(), Bounds::new(-320, -320, 0, 0), true, 0),
        limited: false,
        unlocks: vec![],
        buy_button: Btn::buy(),
        tooltip: Bounds::new(-320, -320, 0, 0),
        hovered: false,
    },
]);

pub static POWER_UPGRADES: Lazy<Vec<Upgrade>> = Lazy::new(|| vec![
    Upgrade {
        name: "CONSTRUCT".to_string(),
        description: "Construct POWER PLANT.".to_string(),
        cost: (Resources::Research, 10),
        entry: Btn::new("".to_string(), Bounds::new(-320, -320, 0, 0), true, 0),
        limited: true,
        unlocks: vec![1],
        buy_button: Btn::buy(),
        tooltip: Bounds::new(-320, -320, 0, 0),
        hovered: false,
    },
    Upgrade {
        name: "DEPLOY CONDUIT DRONE".to_string(),
        description: "Assign a DRONE to collect POWER.".to_string(),
        cost: (Resources::Drones, 1),
        entry: Btn::new("".to_string(), Bounds::new(-320, -320, 0, 0), true, 0),
        limited: false,
        unlocks: vec![],
        buy_button: Btn::buy(),
        tooltip: Bounds::new(-320, -320, 0, 0),
        hovered: false,
    },
]);