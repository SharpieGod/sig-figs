use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Clone, Copy)]
pub struct SigFigNum {
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
    pub fn rounded(&self) -> f64 {
        if self.lsd < 0 {
            let scale = 10f64.powi(-self.lsd);
            (self.value * scale).round_ties_even() / scale
        } else {
            let scale = 10f64.powi(self.lsd);
            (self.value / scale).round_ties_even() * scale
        }
    }

    pub fn get_sf(&self) -> u32 {
        // lsd = self.log10().trunc() as i32 - sig_figs as i32 + 1;
        (self.value.abs().log10().floor() as i32 + 1 - self.lsd as i32) as u32
    }
}

pub trait SigFigable {
    fn sf(&self, sig_figs: u32) -> SigFigNum;
    fn lsd(&self, lsd: i32) -> SigFigNum;
    fn perfect(&self) -> SigFigNum;
}

impl SigFigable for f64 {
    fn sf(&self, sig_figs: u32) -> SigFigNum {
        let lsd = self.abs().log10().floor() as i32 - sig_figs as i32 + 1;

        SigFigNum { value: *self, lsd }
    }

    fn lsd(&self, lsd: i32) -> SigFigNum {
        SigFigNum { value: *self, lsd }
    }

    fn perfect(&self) -> SigFigNum {
        SigFigNum {
            value: *self,
            lsd: -9999999,
        }
    }
}

// Arithmetic

impl Add for SigFigNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        (self.value + rhs.value).lsd(self.lsd.max(rhs.lsd))
    }
}

impl Sub for SigFigNum {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        (self.value - rhs.value).lsd(self.lsd.max(rhs.lsd))
    }
}

impl Mul for SigFigNum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        (self.value * rhs.value).sf(self.get_sf().min(rhs.get_sf()))
    }
}
impl Div for SigFigNum {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        (self.value / rhs.value).sf(self.get_sf().min(rhs.get_sf()))
    }
}

impl Neg for SigFigNum {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            value: -self.value,
            lsd: self.lsd,
        }
    }
}

macro_rules! same_sf {
    ($($x:ident),*) => {
        $(
            pub fn $x(&self) -> Self {
                Self {
                    value: self.value.$x(),
                    lsd: self.lsd,
                }
            }
        )*
    };
}

impl SigFigNum {
    same_sf!(sqrt, sin, cos, acos, asin, tan, atan);

    pub fn powf(&self, pow: f64) -> Self {
        Self {
            value: self.value.powf(pow),
            lsd: self.lsd,
        }
    }

    pub fn powi(&self, pow: i32) -> Self {
        Self {
            value: self.value.powi(pow),
            lsd: self.lsd,
        }
    }
}
