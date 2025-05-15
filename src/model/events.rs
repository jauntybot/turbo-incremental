use super::*;
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub enum Event {
    StartGame,
    SaveGame,
    ResetGame,
    DroneDepotUnlockable,
    UnlockDroneDepot,
    MinesUnlockable,
    PowerPlantUnlockable,
    UnlockPowerPlant,
    LateGame,
    Prestige,
    EndGame,
}

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct EventManager {
    events: Vec<Event>,
    pub dialogue: Option<Dialogue>,
}

impl EventManager {
    pub fn new() -> Self {
        Self { 
            events: Vec::new(),
            dialogue: Some(CUTSCENES[0].clone().start()),
        }
    }

    // Add an event to the queue
    pub fn trigger(&mut self, event: Event) {
        self.events.push(event);
    }

    // Process all events in the queue
    pub fn process_events<F>(&mut self, mut handler: F) -> bool
    where
        F: FnMut(&Event),
    {
        let mut save = false;
        for i in 0..self.events.len() {
            let event = &self.events[i];
            if let Some(dialogue) = &mut self.dialogue {
                if dialogue.event_broadcast <= 0 {
                    handler(event);
                    self.events.remove(i);
                    save = true
                }
            } else {
                match event {
                    Event::StartGame => { 
                        self.dialogue = Some(CUTSCENES[0].clone().start()); 
                    }
                    Event::DroneDepotUnlockable => { 
                        self.dialogue = Some(CUTSCENES[1].clone().start()); 
                    }
                    Event::MinesUnlockable => { 
                        self.dialogue = Some(CUTSCENES[2].clone().start()); 
                    }
                    Event::PowerPlantUnlockable => { 
                        self.dialogue = Some(CUTSCENES[3].clone().start()); 
                    }
                    Event::LateGame => { 
                        self.dialogue = Some(CUTSCENES[4].clone().start()); 
                    }
                    Event::Prestige => {
                        self.dialogue = Some(CUTSCENES[5].clone().start());
                    }
                    _ => {
                        handler(event);
                        self.events.remove(i);
                        save = true
                    }
                }
            }
        }
        save
    }

    pub fn update(&mut self, player: &mut Player) {
        if let Some(dialogue) = &mut self.dialogue {
            if !dialogue.update(player) {
                self.dialogue = None;
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Dialogue {
    pub messages: Vec<String>,
    pub camera_pos: Vec<((i32, i32), i32)>,
    pub event_broadcast: i32,
    pub d_box: DialogueBox,
}
impl Dialogue {
    pub fn start(&mut self) -> Self {
        self.d_box.set_message(self.messages[0].clone());
        for pos in self.camera_pos.iter_mut() {
            if pos.1 == 0 {
                self.d_box.tween(pos.0);
            }
        }
        return self.clone();
    }

    pub fn next(&mut self) -> bool {
        // Remove the first message from the queue
        self.messages.remove(0);
        self.event_broadcast -= 1;
        if self.messages.len() <= 0 {
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

    pub fn update(&mut self, player: &mut Player) -> bool {
        if self.d_box.update(player) {
            return self.next();
        }

        self.d_box.draw();
        true
    }
}

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
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
        let panel = Bounds::new(224, 400-64, 192, 64);
        let btn = Bounds::new(320, 240, 64, 24)
            .anchor_bottom(&panel)
            .anchor_right(&panel);
        Self { 
            panel,
            typed_message: String::new(),
            message: String::new(),
            tween: (None, None),
            prompt: false,
            confirm: Btn::new("CONFIRM".to_string(), btn, true, 1),
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
                player.camera.pos.0 = x;
                player.camera.last_pointer_pos.0 = x;
                camera::set_x(x);
            }
            if let Some(ref mut ytween) = self.tween.1 {
                let y = ytween.get();
                player.camera.pos.1 = y;
                player.camera.last_pointer_pos.1 = y;
                camera::set_y(y);
            }
            player.camera.velocity = (0,0);
        }

        let p = pointer();
        if !self.prompt {
            if p.intersects_fixed(self.panel.x(), self.panel.y(), self.panel.w(), self.panel.h()) && p.just_pressed() {
                return true;
            }
        } else {
            self.confirm.update();
            self.cancel.update();
            if self.confirm.on_click() {

            }
        }
        
        false
    }

    pub fn draw(&self) {
        // Drawing
        rect!(
            fixed = true, 
            xy = self.panel.xy(), 
            wh = self.panel.wh(), 
            border_radius = 4,
            border_size = 1,
            color = 0x222034ff,
            border_color = 0xffffffff,
        );

        rect!(
            fixed = true, 
            xy = (self.panel.x() + 7, self.panel.y() + 7),
            wh = (50, 50),
            border_radius = 4,
            border_size = 1,
            color = 0x222034ff,
            border_color = 0xffffffff,
        );
        sprite!(
            "turbi",
            fixed = true,
            xy = (self.panel.x() + 8, self.panel.y() + 8),
            wh = (48, 48)
        );

        let lines = WrapBox::split_text(self.typed_message.clone(), 24);
        for i in 0..lines.len() {
            text!("{}", lines[i]; fixed = true, xy = (self.panel.x() + 68, self.panel.y() + 8 + i as i32 * 10), color = 0xffffffff);
        }

        text!("[TAP TO CONTINUE]", fixed = true, xy = (self.panel.x() + 78, self.panel.y() + self.panel.h() as i32 - 10), font = "small", color = 0x847e87ff);
    }
}
