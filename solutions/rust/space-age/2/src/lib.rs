// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s as f64)
    }
}

impl Duration {
    fn as_seconds(&self) -> f64 {
        self.0
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64; // 惑星の公転周期（地球年に対する比率）

    fn years_during(d: &Duration) -> f64 {
        d.as_seconds() / 31_557_600.0 / Self::ORBITAL_PERIOD
    }
}

macro_rules! define_planets {
    ($name: ident, $period: expr) => {
        pub struct $name;

        impl Planet for $name {
            const ORBITAL_PERIOD: f64 = $period;
        }
    };
}

define_planets!(Earth, 1.0);
define_planets!(Mercury, 0.2408467);
define_planets!(Venus, 0.61519726);
define_planets!(Mars, 1.8808158);
define_planets!(Jupiter, 11.862615);
define_planets!(Saturn, 29.447498);
define_planets!(Uranus, 84.016846);
define_planets!(Neptune, 164.79132);
