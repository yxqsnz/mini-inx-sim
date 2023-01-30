mod characteristics;
pub use characteristics::*;
use rand::{distributions::Standard, prelude::Distribution, random, Rng};
mod couple;
pub use couple::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Human {
    pub gender: Gender,
    pub characteristics: Characteristics,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Gender {
    #[default]
    Male,
    Female,
}

impl Human {
    pub fn random() -> Self {
        Self {
            gender: random(),
            characteristics: random(),
        }
    }
}

impl Distribution<Gender> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Gender {
        match rng.gen_range(0..=1) {
            0 => Gender::Male,
            _ => Gender::Female,
        }
    }
}
