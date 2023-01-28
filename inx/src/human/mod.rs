mod gene;
pub mod merger;
pub use gene::*;
use rand::{distributions::Standard, prelude::Distribution, random, Rng};
use std::iter::repeat_with;

use crate::util::most_frequent;

use self::merger::merge_genes;

#[derive(Clone, Debug, PartialEq)]
pub struct Couple {
    pub mother: Human,
    pub father: Human,
}

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
    pub parents: Option<Box<Couple>>,
    pub visible_characteristics: VisibleCharacteristics,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Gender {
    #[default]
    Male,
    Female,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MakeChildError {
    InvalidCouple,
    CoupleHaveAMinor,
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

impl Couple {
    pub fn new(mother: Human, father: Human) -> Self {
        Self { mother, father }
    }
}

impl Human {
    pub fn random(gene_amount: usize) -> Self {
        Self {
            gender: random(),
            stage: Stage::Adult,
            genes: repeat_with(random).take(gene_amount).collect(),
            parents: None,
            visible_characteristics: VisibleCharacteristics::default(),
        }
    }

    pub fn make_child(&self, with: &Self) -> Result<Self, MakeChildError> {
        if self.stage.minor() || with.stage.minor() {
            return Err(MakeChildError::CoupleHaveAMinor);
        }

        if self.gender == with.gender {
            return Err(MakeChildError::InvalidCouple);
        }

        Ok(Self {
            stage: Stage::Fetus,
            gender: random(),
            parents: Some(Box::new(Couple::new(self.to_owned(), with.to_owned()))),
            genes: merge_genes(&self.genes, &with.genes),
            visible_characteristics: Default::default(),
        })
    }

    pub fn compute_visible_characteristics(&mut self) {
        todo!()
    }
}

#[test]
fn make_child_minor_fail() {
    let a = Human {
        gender: Gender::Female,
        stage: Stage::Child,
        ..Default::default()
    };

    let b = Human {
        gender: Gender::Male,
        stage: Stage::Child,
        ..Default::default()
    };

    assert_eq!(a.make_child(&b), Err(MakeChildError::CoupleHaveAMinor))
}

impl Distribution<Gender> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Gender {
        match rng.gen_range(0..=1) {
            0 => Gender::Male,
            _ => Gender::Female,
        }
    }
}
