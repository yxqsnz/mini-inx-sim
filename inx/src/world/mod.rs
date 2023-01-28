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

    pub fn find_couple(&mut self, prefer: bool) -> Option<Couple> {
        let peoples = &mut self.peoples;
        let pred = |p: &&Human| {
            if prefer {
                p.parents.is_some()
            } else {
                true
            }
        };

        let father_position = peoples
            .iter()
            .filter(pred)
            .position(|p| p.gender == Gender::Male)?;
        let father = peoples[father_position].to_owned();

        if father.parents.is_none() {
            peoples.remove(father_position);
        }

        let mother_position = peoples
            .iter()
            .filter(pred)
            .position(|p| p.gender == Gender::Female)?;

        let mother = peoples[mother_position].to_owned();

        if mother.parents.is_none() {
            peoples.remove(mother_position);
        }

        Some(Couple { mother, father })
    }
}
