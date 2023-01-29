use crate::{
    generator::build_generation,
    human::{Couple, MakeChildError},
    World,
};

use self::history::Entry;
pub mod generator;
pub mod history;

#[derive(Debug, PartialEq)]
pub enum Step {
    BuildingCouples,
    GenerationgGenerations,
}

#[derive(Debug)]
pub struct Simulator {
    pub world: World,
    pub history: Vec<history::Entry>,
    pub step: Step,
    pub max_generations: u64,
    pub created_generations: u64,
}

impl Simulator {
    pub fn new(world: World, max_generations: u64) -> Self {
        Self {
            world,
            history: Vec::new(),
            max_generations,
            step: Step::BuildingCouples,
            created_generations: 0,
        }
    }

    #[inline]
    fn mark(&mut self, entry: Entry) {
        self.history.push(entry);
    }

    fn make_couple(&mut self) -> Option<Couple> {
        let mut couple = self.world.find_couple()?;
        self.world.year += 2;

        match couple.make_child().and(couple.make_child()) {
            Err(MakeChildError::InvalidCouple) => {
                self.mark(Entry::FailedToCreateCoupleBecauseGenderEqual)
            }
            Ok(()) => {
                self.mark(Entry::NewChild);
            }
        }

        Some(couple)
    }

    pub fn step(&mut self) {
        match self.step {
            Step::BuildingCouples => match self.make_couple() {
                Some(couple) => {
                    self.world.couples.push(couple);
                }
                None => {
                    self.step = Step::GenerationgGenerations;
                }
            },

            Step::GenerationgGenerations => {
                let mut generations = 0;

                self.build_generations(&mut generations);
                self.created_generations += generations;

                self.world.year += 10;
            }
        }
    }

    fn build_generations(&mut self, generation_count: &mut u64) {
        let mut new_couples = vec![];

        for chunk in self.world.couples.chunks(2) {
            if chunk.len() < 2 {
                continue;
            }

            let mut family_a = chunk[0].to_owned();
            let mut family_b = chunk[1].to_owned();

            log::debug!("Family (A): {family_a:#?}");
            log::debug!("Family (B): {family_b:#?}");

            if family_a.descedent_count > self.max_generations
                || family_b.descedent_count > self.max_generations
            {
                continue;
            }

            *generation_count +=
                build_generation(&mut family_a, &mut family_b, self.max_generations);

            new_couples.extend_from_slice(&[family_a, family_b]);
        }

        self.world.couples = new_couples;
    }
}
