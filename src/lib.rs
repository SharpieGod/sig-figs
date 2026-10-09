use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Clone, Copy)]
pub struct SigFigNum {
    value: f64,
    lsd: i32,
}

impl Display for SigFigNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rounded = self.rounded();

        if self.lsd > 0 {
            let sf = self.get_sf();
            let exponent = self.lsd + sf as i32 - 1;
            let mantissa = rounded / 10f64.powi(exponent);
            let decimals = (sf as i32 - 1).max(0) as usize;
            write!(f, "{:.*}e{}", decimals, mantissa, exponent)
        } else if self.lsd == 0 && rounded % 10.0 == 0.0 {
            write!(f, "{:.0}.", rounded)
        } else {
            let decimals = (-self.lsd).clamp(0, 17) as usize;
            write!(f, "{:.*}", decimals, rounded)
        }
    }
}

// enum LSD {
//     TenToThe(i32),
//     Perfect,
// }
// TODO: Add LSD instead of 9999 perfect

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
        let rounded = self.rounded().abs();
        if rounded == 0.0 {
            return 1;
        }
        (rounded.log10().floor() as i32 + 1 - self.lsd) as u32
    }
}

pub trait SigFigable {
    fn sf(&self, sig_figs: u32) -> SigFigNum;
    fn lsd(&self, lsd: i32) -> SigFigNum;
    fn pf(&self) -> SigFigNum;
}

impl SigFigable for f64 {
    fn sf(&self, sig_figs: u32) -> SigFigNum {
        let lsd = self.abs().log10().floor() as i32 - sig_figs as i32 + 1;

        SigFigNum { value: *self, lsd }
    }

    fn lsd(&self, lsd: i32) -> SigFigNum {
        SigFigNum { value: *self, lsd }
    }

    fn pf(&self) -> SigFigNum {
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
    ($($x:ident),+) => {
        $(
            pub fn $x(&self) -> Self {
              self.value.$x().sf(self.get_sf())
            }
        )+
    };
}

macro_rules! impl_integers {
    ($($x:ty),+) => {
        $(
            impl SigFigable for $x {
                fn sf(&self, sf: u32) -> SigFigNum {
                    (*self as f64).sf(sf)
                }

                fn lsd(&self, lsd: i32) -> SigFigNum {
                    (*self as f64).lsd(lsd)
                }

                fn pf(&self) -> SigFigNum {
                    (*self as f64).pf()
                }
            }
        )+
    };
}

impl_integers!(i8, i16, i32, i64, u8, u16, u32, u64);

impl SigFigNum {
    same_sf!(sqrt, sin, cos, acos, asin, tan, atan);

    pub fn pow(&self, pow: SigFigNum) -> Self {
        self.value
            .powf(pow.value)
            .sf(self.get_sf().min(pow.get_sf()))
    }
}
