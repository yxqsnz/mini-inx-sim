use std::iter::repeat_with;

use crate::Human;

#[derive(Clone, Debug)]
pub struct World {
    pub peoples: Vec<Human>,
}

impl World {
    pub fn new(human_count: usize, chara_count: usize) -> World {
        World {
            peoples: repeat_with(|| Human::random(chara_count))
                .take(human_count)
                .collect(),
        }
    }
}
