use rand::random;

use crate::Human;

use super::merger::merge_genes;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Couple {
    pub mother: Human,
    pub father: Human,
    pub children: Vec<Human>,
    pub descedent: Option<Box<Couple>>,
    pub descedent_count: u64,
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

            ..Default::default()
        };

        self.children.push(child);
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MakeChildError {
    InvalidCouple,
}
