use super::*;

pub trait POI {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn manual_produce(&mut self) -> u32 {
        return 0;
    }

    fn produce(&mut self) -> u32 {
        return 0;
    }

    fn upgrade(&mut self, upgrade: &Upgrade, event_manager: &mut EventManager) {}
}