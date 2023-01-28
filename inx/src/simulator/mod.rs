use rand::random;

use crate::{
    human::{Couple, MakeChildError, Stage},
    Human, World,
};

use self::history::Entry;

pub mod history;

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
    pub fn make_child(&mut self, Couple { mother, father }: Couple) {
        self.world.year += 2;

        match mother.make_child(&father) {
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

    fn walk(&mut self) {
        if let Some(couple) = self.world.find_couple(random()) {
            self.make_child(couple);
        }
    }

    pub fn step(&mut self) -> Option<()> {
        self.increment_people_stages();

        if random::<bool>() {
            self.walk();
        }

        self.world.year += 10;
        Some(())
    }
}
