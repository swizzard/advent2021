struct LF(u8);

impl LF {
    fn new_fish() -> Self {
        LF(6)
    }
    fn tick(&mut self) -> bool {
        if self.0 == 0 {
            self.0 = 6;
            true
        } else {
            self.0 -= 1;
            false
        }
    }
}

struct LanternFish(Vec<LF>);

impl LanternFish {
    fn tick(&mut self) {
        let new_fish = self.0.iter_mut().map(|lf| lf.tick()).fold(0, |mut acc, b| {
            if b {
                acc += 1;
            }
            acc
        });
        let mut nf: Vec<LF> = Vec::with_capacity(new_fish);
        nf.fill_with(LF::new_fish);
        self.0.append(&mut nf);
    }
}
