use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
};

struct SigFigNum {
    value: f64,
    lsd: i32,
}

impl Debug for SigFigNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ {} to the {}s }}",
            self.value,
            10f64.powi(self.lsd as i32)
        )
    }
}

impl SigFigNum {
    fn rounded(&self) -> f64 {
        (self.value / 10f64.powi(self.lsd)).round_ties_even() * 10f64.powi(self.lsd)
    }

    fn get_sf(&self) -> u32 {
        // lsd = self.log10().trunc() as i32 - sig_figs as i32 + 1;
        (self.value.log10().floor() as i32 + 1 - self.lsd as i32) as u32
    }
}

trait SigFigable {
    fn sf(&self, sig_figs: u32) -> SigFigNum;
    fn lsd(&self, lsd: i32) -> SigFigNum;
}

impl SigFigable for f64 {
    fn sf(&self, sig_figs: u32) -> SigFigNum {
        let lsd = self.log10().floor() as i32 - sig_figs as i32 + 1;

        SigFigNum { value: *self, lsd }
    }

    fn lsd(&self, lsd: i32) -> SigFigNum {
        SigFigNum { value: *self, lsd }
    }
}

// Arithmetic

impl Add for SigFigNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        (self.value + rhs.value).lsd(self.lsd.min(rhs.lsd))
    }
}

impl Sub for SigFigNum {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        (self.value - rhs.value).lsd(self.lsd.min(rhs.lsd))
    }
}

// impl Mul for SigFigNum {
//     type Output = Self;

//     fn mul(self, rhs: Self) -> Self::Output {
//         (self.value * rhs.value).sf()
//     }
// }

fn main() {
    println!("{:?}", 0.15f64.sf(2))
}
