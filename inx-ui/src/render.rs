use inx::Human;

pub fn human(human: &Human) -> String {
    format!(
        "( {:?} {:?} {:?} )",
        human.gender, human.characteristics.skin_color, human.characteristics.eye_color,
    )
}
