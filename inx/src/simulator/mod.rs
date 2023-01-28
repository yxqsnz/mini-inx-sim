use std::mem::swap;

use crate::{
    human::{Couple, MakeChildError, Stage},
    util::extract_two,
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

    pub fn increment_people_stages(&mut self) {
        let mut couples = self.world.couples.clone();

        for couple in &mut couples {
            for people in couple.children.iter_mut() {
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
    }

    #[inline]
    fn mark(&mut self, entry: Entry) {
        self.history.push(entry);
    }

    pub fn prepare(&mut self) {
        while self.world.find_couple().is_some() {}
        self.basic_childs();
    }

    fn basic_childs(&mut self) {
        let mut couples = self.world.couples.clone();

        for couple in couples.iter_mut() {
            self.world.year += 2;

            match couple.make_child().and(couple.make_child()) {
                Err(MakeChildError::CoupleHaveAMinor) => {
                    self.mark(Entry::FailedToCreateCoupleBecauseMinor)
                }
                Err(MakeChildError::InvalidCouple) => {
                    self.mark(Entry::FailedToCreateCoupleBecauseGenderEqual)
                }
                Ok(()) => {
                    self.mark(Entry::NewChild);
                }
            }
        }

        self.world.couples = couples;
    }

    pub fn step(&mut self) -> Option<()> {
        self.increment_people_stages();

        self.build_generations();

        self.world.year += 10;
        Some(())
    }

    fn build_generation(a: &mut Couple, b: &mut Couple) -> Option<()> {
        let (f1, f2) = extract_two(&a.children, &b.children)?;

        let mut sim_a = Simulator::new(World {
            peoples: f1.to_owned(),
            couples: vec![],
            year: 0,
        });

        let mut sim_b = Simulator::new(World {
            peoples: f2.to_owned(),
            couples: vec![],
            year: 0,
        });

        sim_a.prepare();
        sim_b.prepare();

        for _ in 0..=2 {
            sim_a.step();
            sim_b.step();
        }

        let aa = sim_a
            .world
            .couples
            .into_iter()
            .map(|x| x.children)
            .flatten();

        a.descedent = Some(Box::new(Couple {
            mother: f1[0].to_owned(),
            father: f1[1].to_owned(),
            children: aa.collect(),
            descedent: None,
        }));

        let bb = sim_b
            .world
            .couples
            .into_iter()
            .map(|x| x.children)
            .flatten();

        b.descedent = Some(Box::new(Couple {
            mother: f2[0].to_owned(),
            father: f2[1].to_owned(),
            children: bb.collect(),
            descedent: None,
        }));

        Some(())
    }

    fn build_generations(&mut self) {
        let mut chunks = self.world.couples.chunks(2);
        let mut target = vec![];

        while let Some(chunk) = chunks.next() {
            if chunk.len() != 2 {
                continue;
            }

            let mut a = chunk[0].to_owned();
            let mut b = chunk[0].to_owned();

            Self::build_generation(&mut a, &mut b);

            target.push(a);
            target.push(b);
        }

        self.world.couples = target;
    }
}
