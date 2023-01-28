use crate::{
    human::{Couple, MakeChildError, Stage},
    Human, World,
};

use self::history::Entry;

pub mod history;

#[derive(Debug)]
pub enum Step {
    MakingFamiles,
}

#[derive(Debug)]
pub struct Simulator {
    pub world: World,
    pub history: Vec<history::Entry>,
}

impl Simulator {
    pub fn new(world: World) -> Self {
        Self {
            world,
            history: Vec::new(),
        }
    }

    #[inline]
    pub fn make_child(
        &mut self,
        Couple { mother, father }: Couple,
    ) -> Result<Human, MakeChildError> {
        self.world.year += 2;

        mother.make_child(&father)
    }

    pub fn increment_people_stages(&mut self) {
        for people in self.world.peoples.iter_mut() {
            match people.stage {
                Stage::Fetus => {
                    people.stage = Stage::Child;
                    self.history.push(Entry::AdvanceStage {
                        from: Stage::Fetus,
                        to: Stage::Child,
                    });
                }
                Stage::Child => {
                    self.history.push(Entry::AdvanceStage {
                        from: Stage::Child,
                        to: Stage::Adult,
                    });
                    people.stage = Stage::Adult;
                }
                _ => {}
            }
        }
    }

    #[inline]
    fn mark(&mut self, entry: Entry) {
        self.history.push(entry);
    }

    pub fn step(&mut self) -> Option<()> {
        if let Some(couple) = self.world.find_couple() {
            match self.make_child(couple) {
                Err(MakeChildError::CoupleHaveAMinor) => {
                    self.mark(Entry::FailedToCreateCoupleBecauseMinor)
                }
                Err(MakeChildError::InvalidCouple) => {
                    self.mark(Entry::FailedToCreateCoupleBecauseGenderEqual)
                }
                Ok(child) => {
                    self.world.peoples.push(child);
                    self.mark(Entry::NewChild);
                }
            }
        }

        self.increment_people_stages();

        self.world.year += 10;
        Some(())
    }
}
