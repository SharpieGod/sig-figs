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
        let r = (self.x * self.x + self.y * self.y).sqrt();

        let theta = (self.y / self.x).atan() * (180. / PI).perfect();

        PolarCoordinates { r, theta }
    }
}

fn main() {
    let data = vec![
        // Graysen
        (1.11, 6.07, 1.79),
        (1.05, 3.44, 1.75),
        (0.93, 3.95, 1.75),
        // Darko
        (1.20, 5.18, 1.84),
        (1.09, 4.76, 1.86),
        (1.20, 6.71, 1.84),
        // Steven
        (0.63, 3.51, 1.84),
        (0.66, 3.53, 1.86),
        (0.63, 3.53, 1.84),
        // Daichi
        (1.00, 4.10, 1.78),
        (0.83, 4.21, 1.79),
        (0.91, 2.38, 1.75),
        (1.83, 3.91, 1.71),
    ]
    .iter()
    .enumerate()
    .map(|(i, &(t, dx, dy))| (i, t.lsd(-1), dx.lsd(-2), -dy.lsd(-2)))
    .collect::<Vec<_>>();

    for (i, t, dx, dy) in data {
        let v0 = calc_initial_velocity(t, dx, dy);

        println!(
            "Trial {:>2}:\nv0= [{},{}]\n{} m/s {}°\n",
            i + 1,
            v0.x,
            v0.y,
            v0.r_theta().r,
            v0.r_theta().theta
        );
    }
}

fn calc_initial_velocity(t: SigFigNum, dx: SigFigNum, dy: SigFigNum) -> Vector2 {
    let gravity = -9.80.sf(3);

    let v0y = (dy - (gravity * t * t) / 2.perfect()) / t;
    let v0x = dx / t;

    Vector2::new(v0x, v0y)
}
