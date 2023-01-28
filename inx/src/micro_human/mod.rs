mod characteristic;
pub use characteristic::*;

#[derive(Debug, Clone)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone)]
pub struct MicroHuman {
    pub gender: Gender,
    pub characteristics: Vec<Characteristic>,
}
