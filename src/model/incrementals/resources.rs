use super::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub enum Resources {
    Research,
    Drones,
    Metals,
    Power,
}

impl std::fmt::Display for Resources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Resources::Research => "RESEARCH",
            Resources::Drones => "DRONES",
            Resources::Metals => "METALS",
            Resources::Power => "POWER",
        };
        write!(f, "{}", name)
    }
}