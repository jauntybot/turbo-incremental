use super::*;

#[turbo::serialize]
pub struct DroneStats {
    pub base: f32,
    pub eff: f32,
    pub interval: f32,
    pub speed: f32,

    pub mult: f32,
    pub amped: f32,

    pub recall: bool
}
impl DroneStats {
    pub fn new(base: f32, eff: f32, interval: f32, speed: f32) -> Self {
        DroneStats {
            base,
            eff,
            interval,
            speed,

            mult: 1.0,
            amped: 1.0,

            recall: false,
        }
    }

    pub fn factored(&mut self, player: &Player) -> DroneStats {
        let mut stats = self.clone();
        //stats.base *= player.expertise;
        stats
    }

    pub fn produce(&self) -> u64 {
        (self.base * self.eff * self.mult) as u64
    }
}

#[turbo::serialize]
pub struct Station {
    pub poi_type: POIType,
    pub drones: Vec<Drone>,

    pub drone_stats: DroneStats,

    pub unlockable: bool,
    pub unlocked: bool,
    pub innovation: bool,

    pub collections: Vec<Collection>,
}

impl Station {
    pub fn new(poi_type: POIType, stats: DroneStats) -> Self {
        Station {
            poi_type,
            drones: vec![],

            drone_stats: stats,

            unlockable: false,
            unlocked: false,
            innovation: false,

            collections: vec![],
        }
    }

    pub fn new_drone(poi_type: POIType, stats: &mut DroneStats, player: &Player) -> Self {
        stats.recall = player.drone_recall;
        stats.base += player.expertise;
        Station {
            poi_type, 
            drones: vec![],

            drone_stats: stats.clone(),

            unlockable: false,
            unlocked: false,
            innovation: player.innovation,

            collections: vec![],
        }
    }

    pub fn deploy_drone(&mut self, mode: DroneMode, target_pos: (i32, i32)) {
        let drone = Drone::new(mode, target_pos);
        self.drones.push(drone);
    }

    pub fn new_collect(&mut self, pos: (f32, f32), value: (Resources, u64)) {
        self.collections.push(Collection::new(pos, value));
    }

    pub fn update_collections(&mut self) {
        self.collections.retain_mut(|collection| {
            collection.update()
        });
    }

    pub fn draw_back(&self) {
        for drone in self.drones.iter() {
            if !drone.front {
                drone.draw();
            }
        }

    }
    
    pub fn draw_front(&self) {
        for drone in self.drones.iter() {
            if drone.front {
                drone.draw();
            }
            drone.draw_scan();
        }

        for collection in self.collections.iter() {
            collection.draw();
        }
    }
}

#[turbo::serialize]
#[derive(PartialEq)]
pub enum POIType {
    Exoplanet,
    DroneDepot,
    AsteroidMines,
    PowerPlant,
    ResearchComplex,
    Jumpgate,
    ResearchProbe,
    DroneAmp,
    Fabricator,
}

pub trait POI {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn get_station(&self) -> &Station;

    fn update_drones(&mut self, player: &Player) -> u64;

    fn manual_produce(&mut self) -> u64 {
        return 0;
    }

    fn prod_rate(&self) -> u64 {
        let station = self.get_station();
        (station.drone_stats.base * station.drone_stats.eff) as u64
    }

    fn upgrade(&mut self, upgrade: &Upgrade, event_manager: &mut EventManager) {}
}

