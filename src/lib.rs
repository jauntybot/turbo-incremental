mod model;
use std::fmt::Error;

pub use model::*;

turbo::init!(
    struct GameState {
        player: Player,
        vignette: Vignette,
        event_manager: EventManager,
        exoplanet: Exoplanet,
        drone_depot: DroneDepot,
        asteroid_field: AsteroidField,
        asteroid_mines: AsteroidMines,
        nebula_storm: NebulaStorm,
        power_plant: PowerPlant,
        jumpgate: Jumpgate,
        research_complex: ResearchComplex,
    } = GameState::load_local()
);

impl GameState {
    pub fn new(prestiged: bool, prestige_earned: u64, prestige_prog: u64, prestige_index: u32, avail_upgrades: Vec<Upgrade>) -> Self {
        let mut state = GameState {  
            player: Player::load(prestiged, prestige_earned, prestige_prog, prestige_index, avail_upgrades),
            vignette: Vignette::new(),
            event_manager: EventManager::new(),
            exoplanet: Exoplanet::load(),
            drone_depot: DroneDepot::load(),
            asteroid_field: AsteroidField::new(),
            asteroid_mines: AsteroidMines::load(),
            nebula_storm: NebulaStorm::new(),
            power_plant: PowerPlant::load(),
            jumpgate: Jumpgate::load(),
            research_complex: ResearchComplex::load(),
        };
        state.vignette.fade = false;
        state.save_local();
        state
    }

    pub fn save_local(&self) {
        let data = self.try_to_vec();
        if let Ok(d) = data {
            let _ = local::save(&d);
        } else {
            log!("error saving");
        }
    }

    pub fn load_local() -> GameState {
        let data = local::load().unwrap_or_else(|_| vec![]);
        let mut state = GameState::try_from_slice(&data).unwrap_or_else(|_| GameState::new(false, 0, 0, 0, vec![]));
        state.vignette.fade = false;
        state.vignette.fade_prog = 255.;
        state
    }

}

// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec
turbo::go! ({
    let mut state = GameState::load();
    let mut sfx = GLOBAL.lock().unwrap();
    
    
    for x in -1..=1 {
        for y in -1..=1 {
            sprite!("bg", xy = (x * 640, -80 + y * 640));
        }
    }
    // text!("pos: ({}, {}), target: ({}, {}), last: ({}, {})", state.player.camera.pos.0, state.player.camera.pos.1, camera::x(), camera::y(), state.player.camera.last_pointer_pos.0, state.player.camera.last_pointer_pos.1; fixed = true, y = 28);
    //rect!(xy = (-320, -200), wh = (1280, 800), border_size = 1, color = 0xffffff00, border_color = 0xffffffff);

    if state.event_manager.dialogue.is_none() {
        state.player.update(&mut state.event_manager);
    } else {
        state.player.camera.update_cam(); // Only update the camera
    }
    state.asteroid_field.update();
    if state.asteroid_mines.unlockable {
        state.asteroid_field.draw();
    }
    state.nebula_storm.update();
    if state.power_plant.unlockable {
        state.nebula_storm.draw();
    }

    state.exoplanet.update(&mut state.player, &mut state.event_manager);
    state.exoplanet.draw();
    if state.asteroid_mines.unlockable {
        state.asteroid_mines.update(&mut state.player, &mut state.event_manager, &mut state.asteroid_field);
        state.asteroid_mines.draw();
    }
    if state.drone_depot.unlockable {
        state.drone_depot.update(&mut state.player, &mut state.event_manager);
        state.drone_depot.draw();
    }
    if state.power_plant.unlockable {
        state.power_plant.update(&mut state.player, &mut state.event_manager, &mut state.nebula_storm);
        state.power_plant.draw();
    }
    if state.jumpgate.unlockable {
        state.jumpgate.update(&mut state.player, &mut state.event_manager);
        state.jumpgate.draw();
    }
    if state.research_complex.unlockable {
        state.research_complex.update(&mut state.player, &mut state.event_manager);
        state.research_complex.draw();
    }

    // Event subscribers
    let mut prestige = false;
    let mut reset = false;
    let mut save = false;

    state.event_manager.process_events(|event| {
        state.player.handle_event(event);
        state.vignette.handle_event(event);
        state.exoplanet.handle_event(event);
        state.drone_depot.handle_event(event);
        state.asteroid_mines.handle_event(event);
        state.power_plant.handle_event(event);
        state.jumpgate.handle_event(event);
        state.research_complex.handle_event(event);
        match event {
            Event::ResetGame => {
                reset = true;
            }
            Event::SaveGame => {
                save = true;
            }
            Event::EndGame => {
                state.vignette.fade = true;
                prestige = true;
            }
            _ => {}
        }
    });

    if prestige {
        let leftover = state.player.resources
            .iter()
            .find(|(res, _)| *res == Resources::Prestige)
            .map(|(_, x)| *x)
            .unwrap_or(0);
        state = GameState::new(true, leftover + state.player.prestige_earned, state.player.prestige_prog, state.player.prestige_index, state.player.avail_upgrades.clone());
    }
    if reset {
        state = GameState::new(false, 0, 0, 0, vec![]);
    }
    if save {
        state.save_local();
    }
    
    sfx.update(&mut state.event_manager);
    state.vignette.update();

    // Drawing
    state.vignette.draw();
    state.player.draw();
    if tick() > 100 {
        state.event_manager.update(&mut state.player);
    }
    
    state.exoplanet.draw_ui();
    state.drone_depot.draw_ui();
    state.asteroid_mines.draw_ui();
    state.power_plant.draw_ui();
    state.jumpgate.draw_ui();
    state.player.draw_ui();

    sfx.draw();

    if sfx.autosave && tick() % 1000 == 0 {
        state.save_local();
    }
    if tick() < 200 {
        //camera::set_xy(320, 296);
        let alpha = if tick() >= 150 {
            255 - ((tick() - 150) * 255 / 50) as u8 // Linear fade from 255 to 0
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

    state.save();
});