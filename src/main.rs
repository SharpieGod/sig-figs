use std::f64::consts::PI;

use sig_figs::{self, SigFigNum, SigFigable};

#[derive(Debug, Clone, Copy)]
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

        let theta = (self.y / self.x).atan() * (180. / PI).pf();

        PolarCoordinates { r, theta }
    }
}

fn main() {
    let v = 0.1.sf(6);

    let dt = 86400.0.sf(6);

    let gamma = 1.pf() / (1.pf() - v * v).sqrt();

    println!("{gamma}");
    let dt0 = dt / gamma;

    println!("{}", dt0);
}

fn probe() {
    let t = 1.09.sf(8);
    let height = (2.0200 * 10.0f64.powf(7.)).pf();
    let c = (2.998 * 10.0f64.powf(8.)).sf(7);

    let d_signal = t * c;
    let d_groud = (d_signal * d_signal - height * height).sqrt();

    println!("{t}: {}", 10.0f64.powf(d_groud.lsd as f64));
}

fn satelites() {
    let times = vec![0.06854.sf(4), 0.06742.sf(4), 0.06768.sf(4)];
    let height = (2.0200 * 10.0f64.powf(7.)).sf(5);
    let c = (2.998 * 10.0f64.powf(8.)).sf(4);

    for t in times {
        let d_signal = t * c;
        let d_groud = (d_signal * d_signal - height * height).sqrt();

        println!("{t}: {d_signal} {d_groud}");
    }
}

fn bonus() {
    let m = 670.sf(2);
    let theta = 35.sf(2) * (PI / 180.).pf();
    let mew = 0.22.sf(2);
    let fa = 11550.sf(4);
    let base = 85.sf(2);

    let fg = m * 9.80.sf(3);
    let fgx = -fg * (theta).sin();
    let fgy = fg * (theta).cos();

    let a = (fgx + (-fgy * mew) + fa) / m;

    dbg!(a);

    // cos(theta) = base/d
    let vf = (2.pf() * a * base / theta.cos()).sqrt();

    dbg!(vf);
    let v0 = vf;

    let v0x = v0 * theta.cos();
    let v0y = v0 * theta.sin();

    // tan(theta) = drop/base
    let drop = -base * theta.tan();

    let g = -9.80.sf(3);

    let vfy = -(v0y * v0y + 2.pf() * g * drop).sqrt();
    let vfx = v0x;

    let vf = Vector2::new(vfx, vfy);

    dbg!(vf);
    println!("{:?} {:?}", vf.r_theta().r, vf.r_theta().theta);
}
