mod gene;
pub mod merger;
pub use gene::*;
use rand::{distributions::Standard, prelude::Distribution, random, Rng};
use std::iter::repeat_with;
mod couple;
pub use couple::*;

use crate::util::most_frequent;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct VisibleCharacteristics {
    pub eye_color: EyeColor,
    pub hair_color: HairColor,
    pub skin_color: SkinColor,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Human {
    pub stage: Stage,
    pub gender: Gender,
    pub genes: Vec<Gene>,
    pub visible_characteristics: VisibleCharacteristics,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Gender {
    #[default]
    Male,
    Female,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Stage {
    #[default]
    Fetus,
    Child,
    Adult,
}

impl Stage {
    pub const fn minor(&self) -> bool {
        matches!(self, Stage::Fetus | Stage::Child)
    }
}

impl Human {
    pub fn random(gene_amount: usize) -> Self {
        Self {
            gender: random(),
            stage: Stage::Adult,
            genes: repeat_with(random).take(gene_amount).collect(),
            visible_characteristics: VisibleCharacteristics::default(),
        }
    }

    pub fn compute_visible_characteristics(&mut self) {
        todo!()
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
