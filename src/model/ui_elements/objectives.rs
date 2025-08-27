use super::*;


#[turbo::serialize]
pub struct Objective {
    pub description: String,
    textbox: WrapBox,
}

impl Objective {

    pub fn new() -> Self {
        let bounds = Bounds::new(500, 0, 140, 36);
        let mut o = Objective {
            description: "CURRENT OBJECTIVE: \n Collect RESEARCH by scanning the EXOPLANET.".to_string(),
            textbox: WrapBox::new_bounds("CURRENT OBJECTIVE \n Collect RESEARCH by scanning the EXOPLANET.".to_string(), bounds, 0),
        };
        o.textbox.update_text(o.description.clone());
        o
    
    }

    pub fn handle_event(&mut self, event: &Event) {
        self.description = "CURRENT OBJECTIVE: \n".to_string();
        match event {
            Event::StartGame => {
                self.description += "Collect RESEARCH by scanning the EXOPLANET";
                self.textbox.update_text(self.description.clone());
            }
            Event::DroneDepotUnlockable => {
                self.description += "Construct DRONE DEPOT";
                self.textbox.update_text(self.description.clone());
            }
            Event::UnlockDroneDepot => {
                self.description += "Purchase DRONE SHIPMENT and then DEPLOY SURVEY DRONE";
                self.textbox.update_text(self.description.clone());
            }
            Event::MinesUnlockable => {
                self.description += "Construct ASTEROID MINES then DEPLOY MINING DRONE";
                self.textbox.update_text(self.description.clone());
            }
            Event::PowerPlantUnlockable => {
                self.description += "Construct POWER PLANT then DEPLOY CONDUIT DRONE";
                self.textbox.update_text(self.description.clone());
            }
            Event::LateGame => {
                self.description += "Construct JUMPGATE to prestige";
                self.textbox.update_text(self.description.clone());
            }
            Event::JumpgateBuilt => {
                self.description += "Use the JUMPGATE to prestige and upgrade the RESEARCH PROBE";
                self.textbox.update_text(self.description.clone());
            }
            Event::ResearchComplexBuilt => {
                self.description += "Research new projects to complete in this sector";
                self.textbox.update_text(self.description.clone());
            }
            _ => {}
        }
    }


    pub fn draw(&self) {

        self.textbox.draw();
    }
}