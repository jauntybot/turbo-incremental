use super::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub enum Event {
    DroneDepotUnlockable,
    UnlockDroneDepot,
    MinesUnlockable,
    PowerPlantUnlockable,
}

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct EventManager {
    events: Vec<Event>,
}

impl EventManager {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    // Add an event to the queue
    pub fn trigger(&mut self, event: Event) {
        self.events.push(event);
    }

    // Process all events in the queue
    pub fn process_events<F>(&mut self, mut handler: F)
    where
        F: FnMut(&Event),
    {
        for event in &self.events {
            handler(event);
        }
        self.events.clear(); // Clear the queue after processing
    }
}