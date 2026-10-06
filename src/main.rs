use std::f64::consts::PI;

use sig_figs::{self, SigFigNum, SigFigable};

#[derive(Debug)]
struct Vector2 {
    x: SigFigNum,
    y: SigFigNum,
}

#[derive(Debug)]
struct PolarCoordinates {
    r: SigFigNum,
    theta: SigFigNum,
}

impl Vector2 {
    fn new(x: SigFigNum, y: SigFigNum) -> Self {
        Self { x, y }
    }

    fn r_theta(&self) -> PolarCoordinates {
        let r = (self.x.powi(2) + self.y.powi(2)).sqrt();
        let theta = (self.y / self.x).atan() * (180. / PI).perfect();

        PolarCoordinates { r, theta }
    }
}

fn main() {
    let v = Vector2::new(1.0.sf(2), 1.0.sf(2));

    println!("{}", v.r_theta().r.rounded());
}
