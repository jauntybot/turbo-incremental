mod model;
pub use model::*;

turbo::init!(
    struct GameState {
        button: Btn,
    } = GameState::new()
);

impl GameState {
    pub fn new() -> Self {
        let viewport = viewport();
        GameState {  
            button: Btn::new("TEST".to_string(), viewport.center_x() - 20, viewport.center_y() - 6, 40, 12, true),
        }
    }
}

// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec
turbo::go! ({
    let mut state = GameState::load();

    sprite!("bg");

    state.button.update();
    if state.button.on_click() {

    }
    state.button.draw();

    state.save();
});