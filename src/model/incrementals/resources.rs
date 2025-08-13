#[turbo::serialize]
#[derive(PartialEq, Eq)]
pub enum Resources {
    Research,
    Drones,
    Metals,
    Power,
    Prestige,
}

impl Resources {
    pub fn description(&self) -> String {
        match self {
            Resources::Research => "RESEARCH. Scientific data about the Exoplanet.".to_string(),
            Resources::Drones => "DRONES. Autonomous workers assigned to gather resources.".to_string(),
            Resources::Metals => "METALS. Crafting components for advanced tech.".to_string(), 
            Resources::Power => "POWER. Energy for amplifying other systems.".to_string(),
            Resources::Prestige => "PRESTIGE. Experience earned to upgrade the research probe.".to_string(),
        }
    }
}

impl std::fmt::Display for Resources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Resources::Research => "RESEARCH",
            Resources::Drones => "DRONES",
            Resources::Metals => "METALS",
            Resources::Power => "POWER",
            Resources::Prestige => "PRESTIGE",
        };
        write!(f, "{}", name)
    }
}