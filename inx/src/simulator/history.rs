use crate::human::Stage;

#[derive(Debug)]
pub enum Entry {
    AdvanceStage { from: Stage, to: Stage },
    NewChild,
    FailedToCreateCoupleBecauseMinor,
    FailedToCreateCoupleBecauseGenderEqual,
    GenerationJoining { started :bool }
}
