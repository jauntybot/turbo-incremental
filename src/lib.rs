mod model;
pub use model::*;

turbo::init!(
    struct GameState {
        player: Player,
        event_manager: EventManager,
        exoplanet: Exoplanet,
        drone_depot: DroneDepot,
        asteroid_field: AsteroidField,
        asteroid_mines: AsteroidMines,
        nebula_storm: NebulaStorm,
        power_plant: PowerPlant,
        jumpgate: Jumpgate,
        research_complex: ResearchComplex,
    } = GameState::new()
);

impl GameState {
    pub fn new() -> Self {
        GameState {  
            player: Player::load(),
            event_manager: EventManager::new(),
            exoplanet: Exoplanet::load(),
            drone_depot: DroneDepot::load(),
            asteroid_field: AsteroidField::new(250, 20),
            asteroid_mines: AsteroidMines::load(),
            nebula_storm: NebulaStorm::new(),
            power_plant: PowerPlant::load(),
            jumpgate: Jumpgate::load(),
            research_complex: ResearchComplex::load(),
        }
    }
}

// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec
turbo::go! ({
    let mut state = GameState::load();
    let mut sfx = SFX.lock().unwrap();

    for x in -1..=1 {
        for y in -1..=1 {
            sprite!("bg", xy = (x * 640, -80 + y * 640));
        }
    }
    //sprite!("bg", xy = (0, -80));
    rect!(xy = (0, 0), wh = (640, 480), border_size = 1, color = 0xffffff00, border_color = 0xffffffff);
    if state.event_manager.dialogue.is_none() {
        state.player.update();
    }

    state.exoplanet.update(&mut state.player, &mut state.event_manager);
    state.exoplanet.draw();
    if state.drone_depot.unlockable {
        state.drone_depot.update(&mut state.player, &mut state.event_manager);
    }
    state.drone_depot.draw();
    state.asteroid_field.update();
    state.asteroid_field.draw();
    if state.asteroid_mines.unlockable {
        state.asteroid_mines.update(&mut state.player, &mut state.event_manager, &mut state.asteroid_field);
        state.asteroid_mines.draw();
    }
    state.nebula_storm.update();
    //state.nebula_storm.draw();
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

    sfx.update();


    // Event subscribers
    state.event_manager.process_events(|event| {
        state.exoplanet.handle_event(event);
        state.drone_depot.handle_event(event);
        state.asteroid_mines.handle_event(event);
        state.power_plant.handle_event(event);
        state.jumpgate.handle_event(event);
        state.research_complex.handle_event(event);
    });
    
    state.player.draw();

    state.exoplanet.draw_ui();
    state.drone_depot.draw_ui();
    state.asteroid_mines.draw_ui();
    state.power_plant.draw_ui();

    sfx.draw();

    if state.event_manager.dialogue.is_some() {
        state.event_manager.update();
    }

    state.save();
});