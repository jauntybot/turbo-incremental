use turbo::*;
mod model;
pub use model::*;


// Provide the required 5 arguments to the macro, e.g. (update, draw, init, event, name)
// Replace these with the actual function names or values as required by your project.
#[turbo::game]
struct GameState {
    manager: GameManager,
    player: Player,
    vignette: Vignette,
    event_manager: EventManager,
    exoplanet: Exoplanet,
    drone_depot: DroneDepot,
    asteroid_mines: AsteroidMines,
    power_plant: PowerPlant,
    jumpgate: Jumpgate,
    research_complex: ResearchComplex,
    drone_amp: DroneAmp,
}
impl GameState {
    pub fn new() -> Self {
        GameState::load_local()
    }

    pub fn create(prestiged: bool, player: &mut Player, manager: GameManager) -> Self {
        player.reset_jump();
        player.station.unlocked = prestiged;
        if prestiged {
            let prestige =  if let Some(r) = player.resources.iter().find(|(res, _)| *res == Resources::Prestige) {
                r.1 + player.prestige_earned
            } else {
                player.prestige_earned
            };
            player.prestige_earned = 0;
            let mut resources = vec![(Resources::Prestige, prestige)];
            if player.resourceful {
                for r in player.resources.iter() {
                    if r.0 == Resources::Prestige {
                        continue;
                    }
                    let mut saved = r.clone();
                    saved.1 = (saved.1 as f32 * 0.1) as u64;
                    resources.push(saved);
                }
            }
            player.resources = resources;
        }
        // player.resources.push((Resources::Research, 4000000000));
        // player.resources.push((Resources::Drones, 4000000000));
        // player.resources.push((Resources::Metals, 4000000000));
        // player.resources.push((Resources::Power, 4000000000));

        let mut state = GameState {  
            manager,
            player: player.clone(),
            vignette: Vignette::new(),
            event_manager: EventManager::new(),
            exoplanet: Exoplanet::load(player),
            drone_depot: DroneDepot::load(player),
            asteroid_mines: AsteroidMines::load(player),
            power_plant: PowerPlant::load(player),
            jumpgate: Jumpgate::load(),
            research_complex: ResearchComplex::load(player),
            drone_amp: DroneAmp::load(),
        };
        state.vignette.fade = false;
        state.save_local();
        state
    }

    pub fn save_local(&self) {
        let data = borsh::to_vec(self);
        if let Ok(d) = data {
            let _ = local::save(&d);
        } else {
            log!("error saving");
        }
    }

    pub fn load_local() -> GameState {
        let data = local::load().unwrap_or_else(|_| vec![]);
        let mut state: Result<GameState, std::io::Error> = borsh::from_slice(&data);
        if let Err(_) = state {
            state = Ok(GameState::create(false, &mut Player::new(), GameManager::new()));
        }
        if let Ok(mut s) = state {
            s.manager.options = false;
            s.vignette.fade = false;
            s.vignette.fade_prog = 255.;
            return s
        } else {
            log!("error loading game state");
            return GameState::create(false, &mut Player::new(), GameManager::new());
        }
    }
    
    // This is where your main game loop code goes
    // The stuff in this block will run ~60x per sec
    pub fn update(&mut self) {
        clear(0x140b1dff);
        for x in -2..=1 {
            for y in -1..=1 {
                sprite!("bg", xy = ((time::tick() as i32 / 20) % 480 + x * 480, -80 + y * 480));
            }
        }
        for x in -2..=1 {
            for y in -1..=1 {
                sprite!("fg", xy = ((time::tick() as i32 / 30) % 480 + x * 480, -80 + y * 480));
            }
        }
        // text!("pos: ({}, {}), target: ({}, {}), last: ({}, {})", state.player.camera.pos.0, state.player.camera.pos.1, camera::x(), camera::y(), state.player.camera.last_pointer_pos.0, state.player.camera.last_pointer_pos.1; fixed = true, y = 28);
        //rect!(xy = (-320, -200), wh = (1280, 800), border_size = 1, color = 0xffffff00, border_color = 0xffffffff);
    
        if self.event_manager.dialogue.is_none() {
            self.player.update(&mut self.event_manager);
        } else {
            self.player.camera.update_cam(); // Only update the camera
            self.player.hovered_poi = None;
        }

        
        self.drone_amp.update(&mut self.player, &mut self.event_manager,
            &mut self.exoplanet, &mut self.asteroid_mines, &mut self.power_plant);
        if self.drone_amp.get_station().unlockable {
            self.drone_amp.draw();
        }
        // EXOPLANET
        self.exoplanet.update(&mut self.player, &mut self.event_manager);
        self.exoplanet.draw();
        // ASTEROID MINES
        self.asteroid_mines.update(&mut self.player, &mut self.event_manager);
        if self.asteroid_mines.get_station().unlockable {
            self.asteroid_mines.draw();
        }
        // DRONE DEPOT
        self.drone_depot.update(&mut self.player, &mut self.event_manager);
        if self.drone_depot.get_station().unlockable {
            self.drone_depot.draw();
        }
        // POWER PLANT
        self.power_plant.update(&mut self.player, &mut self.event_manager);
        if self.power_plant.get_station().unlockable {
            self.power_plant.draw();
        }

        self.jumpgate.update(&mut self.player, &mut self.event_manager);
        if self.jumpgate.get_station().unlockable {
            self.jumpgate.draw();
        }

        self.research_complex.update(&mut self.player, &mut self.event_manager);
        if self.research_complex.get_station().unlockable {
            self.research_complex.draw();
        }
    
        // Event subscribers
        let mut prestige = false;
        let mut reset = false;
        let mut save = false;
    
        self.event_manager.update(&mut self.player);
        self.event_manager.process_events(|event| {
            self.player.handle_event(event);
            self.vignette.handle_event(event);
            self.exoplanet.handle_event(event);
            self.drone_depot.handle_event(event);
            self.asteroid_mines.handle_event(event);
            self.power_plant.handle_event(event);
            self.jumpgate.handle_event(event);
            self.research_complex.handle_event(event);
            self.drone_amp.handle_event(event);
            match event {
                Event::ResetGame => {
                    reset = true;
                }
                Event::SaveGame => {
                    save = true;
                }
                Event::EndGame => {
                    self.vignette.fade = true;
                    prestige = true;
                }
                _ => {}
            }
        });
    
        if prestige {
            events::emit("midgame_ad", "");
            *self = GameState::create(true, &mut self.player, self.manager.clone());
        }
        if reset {
            *self = GameState::create(false, &mut Player::new(), self.manager.clone());
        }
        if save {
            self.save_local();
        }
        
        self.manager.update(&mut self.event_manager);
        self.vignette.update();
    
        // Drawing
        self.vignette.draw();
        self.player.draw();
        if turbo::time::tick() > 100 {
            self.event_manager.draw();
        }
        
        self.exoplanet.draw_ui();
        self.drone_depot.draw_ui();
        self.asteroid_mines.draw_ui();
        self.power_plant.draw_ui();
        self.jumpgate.draw_ui();
        self.research_complex.draw_ui();
        self.drone_amp.draw_ui();
        self.player.draw_ui();
    
        self.manager.draw();
    
        if self.manager.autosave && turbo::time::tick() % 1000 == 0 {
            self.save_local();
        }
        if time::tick() < 200 {
            //camera::set_xy(320, 296);
            let alpha = if turbo::time::tick() >= 150 {
                255 - ((turbo::time::tick() - 150) * 255 / 50) as u8 // Linear fade from 255 to 0
            } else {
                255
            };
            let color = 0xFFFFFF00 | alpha as u32; // Place alpha in the lowest byte
            // sprite!(
            //     "coolmath",
            //     fixed = true,
            //     xy = (0, 0),
            //     color = color
            // );
        }

        // rect!(
        //     fixed = true,
        //     xy = (320, 0),
        //     wh = (1, 360),
        // );
        // rect!(
        //     fixed = true,
        //     xy = (0, 180),
        //     wh = (640, 1),
        // );
    }
}