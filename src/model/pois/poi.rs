use super::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Station {
    pub drone_base: f32,
    pub drone_eff: f32,
    pub drone_speed: f32,
}

pub trait POI {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn get_station(&self) -> &Station;

    fn manual_produce(&mut self) -> u64 {
        return 0;
    }

    fn produce(&mut self) -> u64 {
        return 0;
    }

    fn prod_rate(drones: u32, drone_base: u32, drone_eff: u32, drone_speed: u32) -> u64 {
        0
    }

    fn upgrade(&mut self, upgrade: &Upgrade, event_manager: &mut EventManager) {}
}

