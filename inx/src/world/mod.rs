use std::iter::repeat_with;

use crate::{
    human::{Couple, Gender},
    Human,
};

#[derive(Clone, Debug)]
pub struct World {
    pub peoples: Vec<Human>,
    pub year: u64,
}

impl World {
    pub fn new(population: usize, gene_count: usize) -> World {
        World {
            year: 0,
            peoples: repeat_with(|| Human::random(gene_count))
                .take(population)
                .collect(),
        }
    }

    pub fn find_couple(&mut self) -> Option<Couple> {
        let peoples = &mut self.peoples;

        let father_position = peoples.iter().position(|p| p.gender == Gender::Male)?;
        let father = peoples.remove(father_position);

        let mother_position = peoples.iter().position(|p| p.gender == Gender::Female)?;
        let mother = peoples.remove(mother_position);

        Some(Couple { mother, father })
    }
}
