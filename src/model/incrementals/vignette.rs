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
            depot: vec![Cloud::new((320, 200), 320, 240), Cloud::new((DEPOT_BOX.0-16, DEPOT_BOX.1-16), 48, 0)],
            mines: vec![Cloud::new((320, 200), 480, 320),],
            clouds: vec![Cloud::new((320, 200), 780, 480)],
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::DroneDepotUnlockable => {
                self.depot.remove(1);
                self.depot[0].fade_ranges.push((110., 200.));
            }
            Event::MinesUnlockable => {
                self.depot[0].fade_ranges[0] = (110., 315.);
                self.mines[0].fade_ranges.push((180., 280.));
                self.clouds[0].fade_ranges.push((190., 250.));
            }
            Event::PowerPlantUnlockable => {
                self.depot.clear();
                self.mines[0].fade_ranges[0] = (180., 10.);
                self.clouds[0].fade_ranges[0].1 = 360.;
            }
            Event::LateGame => {
                self.mines[0].fade_ranges.push((40., 140.));
                self.clouds[0].fade_ranges.push((60., 120.));
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
    }
}