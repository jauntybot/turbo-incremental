mod model;
pub use model::*;

turbo::init!(
    struct GameState {
        player: Player,
        event_manager: EventManager,
        exoplanet: Exoplanet,
        drone_depot: DroneDepot,
        asteroid_mines: AsteroidMines,
        power_plant: PowerPlant,
    } = GameState::new()
);

impl GameState {
    pub fn new() -> Self {
        let viewport = viewport();
        GameState {  
            player: Player::load(),
            event_manager: EventManager::new(),
            exoplanet: Exoplanet::load(),
            drone_depot: DroneDepot::load(),
            asteroid_mines: AsteroidMines::load(),
            power_plant: PowerPlant::load(),
        }
    }
}

// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec
turbo::go! ({
    let mut state = GameState::load();

    sprite!("bg");
    state.player.update();

    state.exoplanet.update(&mut state.player, &mut state.event_manager);
    state.exoplanet.draw();
    if state.drone_depot.unlockable {
        state.drone_depot.update(&mut state.player, &mut state.event_manager);
        state.drone_depot.draw();
    }
    if state.asteroid_mines.unlockable {
        state.asteroid_mines.update(&mut state.player, &mut state.event_manager);
        state.asteroid_mines.draw();
    }
    if state.power_plant.unlockable {
        state.power_plant.update(&mut state.player, &mut state.event_manager);
        state.power_plant.draw();
    }
    
    // Event subscribers
    state.event_manager.process_events(|event| {
        state.exoplanet.handle_event(event);
        state.drone_depot.handle_event(event);
        state.asteroid_mines.handle_event(event);
        state.power_plant.handle_event(event);
    });
    
    state.player.draw();

    state.save();
});