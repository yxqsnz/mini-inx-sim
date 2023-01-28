mod characteristic;
use std::iter::repeat_with;

pub use characteristic::*;
use rand::{distributions::Standard, prelude::Distribution, random, Rng};

#[derive(Debug, Clone)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone)]
pub struct Human {
    pub gender: Gender,
    pub characteristics: Vec<Characteristic>,
}

impl Human {
    pub fn random(chara_count: usize) -> Self {
        Self {
            gender: random(),
            characteristics: repeat_with(random).take(chara_count).collect(),
        }
    }
}

impl Distribution<Gender> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Gender {
        match rng.gen_range(0..1) {
            0 => Gender::Male,
            _ => Gender::Female,
        }
    }
}
