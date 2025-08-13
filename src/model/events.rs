use super::*;
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[turbo::serialize]
#[derive(PartialEq)]
pub enum Event {
    StartGame,
    SaveGame,
    ResetGame,
    Prestige,
    EndGame,
    // Early game progression events
    DroneDepotUnlockable,
    UnlockDroneDepot,
    MinesUnlockable,
    PowerPlantUnlockable,
    UnlockPowerPlant,
    LateGame,
    JumpgateBuilt,
    ResearchComplexBuilt,
    // Research Complex events
    FabricatorUnlockable,
    AmpUnlockable,
    AdvDronesResearched,
    Simulacrum,
    // Prestige events
    BaseUpgrade { amount: f32 },
    RecallUpgrade,
    RecallDrone,
    InnovationUpgrade,
}

#[turbo::serialize]
pub struct EventManager {
    events: Vec<Event>,
    triggered_events: Vec<Event>,
    pub dialogue: Option<Dialogue>,
    over: bool,
}

impl EventManager {
    pub fn new() -> Self {
        Self { 
            events: Vec::new(),
            triggered_events: Vec::new(),
            dialogue: Some(CS_INTRO.clone().start()),
            over: false,
        }
    }

    // Add an event to the queue
    pub fn trigger(&mut self, event: Event) {
        if event != Event::Prestige 
            && event != Event::RecallDrone
            && event != Event::ResetGame 
            && event != Event::SaveGame
            && event != (Event::BaseUpgrade { amount: 15. })
            && self.triggered_events.contains(&event) {
            return; // Event already triggered, do not add again
        }
        self.triggered_events.push(event.clone());
        self.events.push(event);
        turbo::events::emit("stop", "");
        log!("added event");
    }

    // Process all events in the queue
    pub fn process_events<F>(&mut self, mut handler: F)
    where
        F: FnMut(&Event),
    {
        if !self.events.is_empty() {
            let event = &self.events[0];
            if let Some(dialogue) = &mut self.dialogue {
                if dialogue.event_broadcast <= 0 {
                    handler(event);
                    self.events.remove(0);
                    if dialogue.prompt {
                        self.dialogue = None;
                    }
                }
            } else {
                match event {
                    Event::StartGame => { 
                        self.dialogue = Some(CS_INTRO.clone().start()); 
                    }
                    Event::DroneDepotUnlockable => { 
                        self.dialogue = Some(CS_DEPOT_AVAIL.clone().start()); 
                    }
                    Event::MinesUnlockable => { 
                        self.dialogue = Some(CS_MINES_AVAIL.clone().start()); 
                    }
                    Event::PowerPlantUnlockable => { 
                        self.dialogue = Some(CS_PLANT_AVAIL.clone().start()); 
                    }
                    Event::LateGame => { 
                        self.dialogue = Some(CS_LATE_GAME.clone().start()); 
                    }
                    Event::JumpgateBuilt => {
                        self.dialogue = Some(CS_JUMPGATE_BUILT.clone().start()); 
                    }
                    Event::ResearchComplexBuilt => {
                        self.dialogue = Some(CS_COMPLEX_BUILT.clone().start()); 
                    }
                    Event::Prestige => {
                        if self.over {
                            self.events.remove(0);
                            self.over = false;
                            self.dialogue = None;
                            turbo::events::emit("start", "");
                        } else {
                            self.dialogue = Some(CS_PRESTIGE_PROMPT.clone().start());
                        }
                    }
                    Event::ResetGame => {
                        if self.over {
                            self.events.remove(0);
                            self.over = false;
                            self.dialogue = None;
                            turbo::events::emit("start", "");
                        } else {
                            self.dialogue = Some(CS_RESET_PROMPT.clone().start());
                        }
                    }
                    Event::EndGame => {
                        self.dialogue = Some(CS_OUTRO.clone().start());
                    }
                    Event::FabricatorUnlockable => {
                        self.dialogue = Some(CS_FAB_AVAIL.clone().start()); 
                    }
                    Event::AmpUnlockable => {
                        self.dialogue = Some(CS_AMP_AVAIL.clone().start()); 
                    }
                    Event::AdvDronesResearched => {
                        self.dialogue = Some(CS_ADV_DRONES.clone().start()); 
                    }
                    Event::Simulacrum => {
                        self.dialogue = Some(CS_SIMULACRUM.clone().start()); 
                    }
                    _ => {
                        handler(event);
                        self.events.remove(0);
                        turbo::events::emit("start", "");
                    }
                }
            }
        }
        if self.over {
            self.over = false;
            self.dialogue = None;
            turbo::events::emit("start", "");
        }
    }

    pub fn update(&mut self, player: &mut Player) {
        if let Some(dialogue) = &mut self.dialogue {
            if !dialogue.update(player) {
                self.over = true;
                self.dialogue = None;
            }
        }
    }

    pub fn draw(&mut self) {
        if let Some(dialogue) = &mut self.dialogue {
            dialogue.draw();
        }
    }
}

impl Default for EventManager {
    fn default() -> Self {
        EventManager::new()
    }
}

#[turbo::serialize]
pub struct Dialogue {
    pub messages: Vec<String>,
    pub camera_pos: Vec<((i32, i32), i32)>,
    pub event_broadcast: i32,
    pub d_box: DialogueBox,
    pub prompt: bool,
}
impl Dialogue {
    pub fn start(&mut self) -> Self {
        self.d_box.set_message(self.messages[0].clone());
        for pos in self.camera_pos.iter_mut() {
            if pos.1 == 0 {
                self.d_box.tween(pos.0);
            }
        }
        self.d_box.prompt = self.prompt;
        return self.clone();
    }

    // Returns false when no next message found - end of dialogue
    pub fn next(&mut self) -> bool {
        // Remove the first message from the queue
        self.messages.remove(0);
        self.event_broadcast -= 1;
        if self.messages.is_empty() {
            false
        } else {
            self.d_box.set_message(self.messages[0].clone());
            // Trigger camera movement
            for pos in self.camera_pos.iter_mut() {
                pos.1 -= 1;
                if pos.1 == 0 {
                    self.d_box.tween(pos.0);
                }
            }
    
            true
        }
    }

    // Returns false when no next message found - end of dialogue
    pub fn update(&mut self, player: &mut Player) -> bool {
        if !self.d_box.prompt {
            if self.d_box.update(player) {
                return self.next();
            }
        } else {
            if let Some(p) = self.d_box.prompt(player) {
                if p {
                    self.event_broadcast -= 1;
                } else {
                    return false
                }
            }
        }
        true
    }

    pub fn draw(&self) {
        self.d_box.draw();
    }
}

#[turbo::serialize]
pub struct DialogueBox {
    pub panel: Bounds,
    //pub button: Btn,
    pub typed_message: String,
    pub message: String,
    pub tween: (Option<Tween<i32>>, Option<Tween<i32>>),
    pub prompt: bool,
    pub confirm: Btn,
    pub cancel: Btn,
}

impl DialogueBox {
    pub fn new() -> Self {
        let panel = Bounds::new(224, 360-64-16, 192, 64);
        let btn = Bounds::new(320, 240, 48, 22)
            .anchor_bottom(&panel)
            .anchor_right(&panel)
            .translate(-16 ,-4);
        Self { 
            panel,
            typed_message: String::new(),
            message: String::new(),
            tween: (None, None),
            prompt: false,
            confirm: Btn::new("CONFIRM".to_string(), btn.translate_x(-56), true, 1),
            cancel: Btn::new("CANCEL".to_string(), btn, true, 1),
        }
    }

    pub fn tween(&mut self, target: (i32, i32)) {
        let mut xtween = Tween::new(camera::x() as i32); 
        let mut ytween = Tween::new(camera::y() as i32);
        xtween.set(target.0);
        ytween.set(target.1);
        xtween.duration((target.0 - camera::x() as i32).abs() as usize / 4);
        ytween.duration((target.1 - camera::y() as i32).abs() as usize / 4);
        xtween.set_ease(Easing::EaseOutCubic);
        ytween.set_ease(Easing::EaseOutCubic);
        self.tween = (Some(xtween), Some(ytween));
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
        self.typed_message.clear();
        self.typed_message.push_str(&self.message);
    }

    pub fn update(&mut self, player: &mut Player) -> bool {
        if self.tween.0.is_some() || self.tween.1.is_some() {
            if let Some(ref mut xtween) = self.tween.0 {
                let x = xtween.get();
                player.camera.pos.0 = x as f32;
            }
            if let Some(ref mut ytween) = self.tween.1 {
                let y = ytween.get();
                player.camera.pos.1 = y as f32;
            }
        }
        
        let p = pointer::screen();
        if p.intersects(self.panel.x(), self.panel.y(), self.panel.w(), self.panel.h()) && p.just_pressed() {
            player.camera.velocity = (0.,0.);
            player.camera.last_pointer_pos = (0.,0.);
            player.camera.dragging = false;
            return true;
        }
        
        false
    }
    
    pub fn prompt(&mut self, player: &mut Player) -> Option<bool> {
        self.confirm.update();
        self.cancel.update();
        if self.confirm.on_click() {
            player.camera.velocity = (0.,0.);
            player.camera.last_pointer_pos = (0.,0.);
            player.camera.dragging = false;
            return Some(true);
        }
        if self.cancel.on_click() {
            player.camera.velocity = (0.,0.);
            player.camera.last_pointer_pos = (0.,0.);
            player.camera.dragging = false;
            return Some(false);
        }
        None
    }

    pub fn draw(&self) {
        // Drawing
        for i in 0..2 {
            let mut o =  2 + ((time::tick() as f32 / 5. + (i as f32 * 10.)) % 21.) as i32;
            //log!("{} o: {}", i, o);
            let alpha = if o >= 10 {
                // Fade from 0 (at o=6) to 255 (at o=11)
                let t = (o - 10) as f32 / 9.;
                ((1.0 - t) * 255.0).round() as u32
            } else {
                255
            };
            let faded_color = (0xffffff00) | alpha;

            rect!(
                fixed = true, 
                xy = (self.panel.x() - o/2, self.panel.y() - o/2), 
                wh = (self.panel.w() + o as u32, self.panel.h() + o as u32), 
                border_radius = 4,
                border_size = 1,
                color = 0x1f122b00,
                border_color = faded_color,
            );
        }

        rect!(
            fixed = true, 
            xy = self.panel.xy(), 
            wh = self.panel.wh(), 
            border_radius = 4,
            border_size = 1,
            color = 0x1f122bff,
            border_color = 0xffffffff,
        );

        rect!(
            fixed = true, 
            xy = (self.panel.x() + 7, self.panel.y() + 7),
            wh = (50, 50),
            border_radius = 4,
            border_size = 1,
            color = 0x1f122bff,
            border_color = 0xffffffff,
        );
        sprite!(
            "turbi",
            fixed = true,
            xy = (self.panel.x() + 8, self.panel.y() + 8),
            wh = (48, 48), 
        );

        let lines = WrapBox::split_text(self.typed_message.clone(), 24);
        for i in 0..lines.len() {
            text!("{}", lines[i]; fixed = true, xy = (self.panel.x() + 68, self.panel.y() + 8 + i as i32 * 10), color = 0xffffffff);
        }

        if !self.prompt {
            text!("[TAP TO CONTINUE]", fixed = true, xy = (self.panel.x() + 78, self.panel.y() + self.panel.h() as i32 - 10), font = "small", color = 0x847e87ff);
        } else {
            self.confirm.draw();
            self.cancel.draw();
        }
    }
}
