use std::iter::repeat_with;

use crate::{
    human::{Couple, Gender},
    Human,
};

#[derive(Clone, Debug, Default)]
pub struct World {
    pub peoples: Vec<Human>,
    pub couples: Vec<Couple>,
    pub year: u64,
}

impl World {
    pub fn new(population: usize, gene_count: usize) -> Self {
        World {
            year: 0,
            couples: vec![],
            peoples: repeat_with(|| Human::random(gene_count))
                .take(population)
                .collect(),
        }
    }

    pub fn with_peoples(peoples: Vec<Human>) -> Self {
        Self {
            peoples,
            ..Default::default()
        }
    }

    pub fn find_couple(&mut self) -> Option<Couple> {
        let father_position = self.peoples.iter().position(|p| p.gender == Gender::Male)?;
        let father = self.peoples.remove(father_position);

        let mother_position = self
            .peoples
            .iter()
            .position(|p| p.gender == Gender::Female)?;

        let mother = self.peoples.remove(mother_position);

        Some(Couple::new(mother, father))
    }
}
