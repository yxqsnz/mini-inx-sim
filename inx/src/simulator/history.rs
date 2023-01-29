#[derive(Debug)]
pub enum Entry {
    NewChild,

    FailedToCreateCoupleBecauseGenderEqual,
    GenerationJoining { started: bool },
}
