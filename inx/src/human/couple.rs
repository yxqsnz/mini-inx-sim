use rand::random;

use crate::Human;

use super::{merger::merge_genes, Stage};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Couple {
    pub mother: Human,
    pub father: Human,
    pub children: Vec<Human>,
    pub descedent: Option<Box<Couple>>,
}

impl Couple {
    pub fn new(mother: Human, father: Human) -> Self {
        Self {
            mother,
            father,
            ..Default::default()
        }
    }

    pub fn make_child(&mut self) -> Result<(), MakeChildError> {
        if self.father.stage.minor() || self.father.stage.minor() {
            return Err(MakeChildError::CoupleHaveAMinor);
        }

        if self.father.gender == self.mother.gender {
            return Err(MakeChildError::InvalidCouple);
        }

        let child = Human {
            gender: random(),
            genes: merge_genes(
                &self.mother.genes,
                &self.father.genes,
                self.mother.genes.len(),
            ),
            stage: Stage::Fetus,
            ..Default::default()
        };

        self.children.push(child);
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MakeChildError {
    InvalidCouple,
    CoupleHaveAMinor,
}

#[test]
fn make_child_minor_fail() {
    use super::Gender;
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

    let mut couple = Couple::new(a, b);

    assert_eq!(couple.make_child(), Err(MakeChildError::CoupleHaveAMinor))
}
