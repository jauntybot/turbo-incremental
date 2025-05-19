use super::*;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Vignette {
    pub fade: bool,
    fade_prog: f32,
    stage: u32,
    depot: Vec<Cloud>,
    mines: Vec<Cloud>,
    clouds: Vec<Cloud>,
}
impl Vignette {
    pub fn new() -> Self {
        Vignette {
            fade: true,
            fade_prog: 255.,
            stage: 0,
            depot: vec![Cloud::new((320, 240), 320, 240), Cloud::new((DEPOT_BOX.0-16, DEPOT_BOX.1-16), 48, 0)],
            mines: vec![Cloud::new((320, 240), 480, 320),],
            clouds: vec![Cloud::new((320, 240), 800, 480)],
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::DroneDepotUnlockable => {
                self.depot.clear();
            }
            Event::MinesUnlockable => {
                self.mines.clear();
                self.clouds[0].fade_ranges.push((180., 270.));
            }
            Event::PowerPlantUnlockable => {
                self.clouds[0].fade_ranges[0].1 = 360.;
            }
            _ => {}
        }
    }
    pub fn update(&mut self) {
        if self.fade && self.fade_prog < 255. {
        // Ease-in and ease-out for fading in
        let progress = self.fade_prog as f32 / 255.0;
        let ease = progress.powi(2); // Quadratic ease-out
        self.fade_prog += (2.0 * ease).ceil();
        if self.fade_prog > 255. {
            self.fade_prog = 255.;
        }
        } else if !self.fade && self.fade_prog > 0. {
            // Ease-in and ease-out for fading out
            let progress = self.fade_prog as f32 / 255.0;
            let ease = progress.powi(2); // Quadratic ease-in
            self.fade_prog -= (2.0 * ease).ceil();
            if self.fade_prog > 255. {
                self.fade_prog = 255.;
            }
            if self.fade_prog < 0. {
                self.fade_prog = 0.;
            }
        }

        for cloud in self.clouds.iter_mut() {
            cloud.update();
        }
        for cloud in self.depot.iter_mut() {
            cloud.update();
        }
        for cloud in self.mines.iter_mut() {
            cloud.update();
        }
    }

    pub fn draw(&self) {
        let color = 0x000000 | self.fade_prog as u32;
        rect!(fixed = true, wh = (640, 400), color = color);

        for cloud in self.clouds.iter() {
            cloud.draw();
        }
        for cloud in self.depot.iter() {
            cloud.draw();
        }
        for cloud in self.mines.iter() {
            cloud.draw();
        }

        //circ!(xy = (-280, -360), size = 1200, border_size = 240, border_color = 0x000000BF, color = 0x00000000);
        // sprite!("blobs", xy = (DEPOT_BOX.0, DEPOT_BOX.1), wh = (192, 192), color = 0x000000BF);
        // sprite!("blobs", xy = (DEPOT_BOX.0 - 96, DEPOT_BOX.1 - 96), wh = (192, 192), color = 0x000000BF);
        // sprite!("blobs", xy = (DEPOT_BOX.0 - 480, DEPOT_BOX.1), wh = (640, 640), color = 0x000000BF);
        // sprite!("blobs", xy = (0, DEPOT_BOX.1 + 120), wh = (640, 640), color = 0x000000BF);

    }
}