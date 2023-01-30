use egui::Color32;
use inx::{
    human::{EyeColor, Gender, HairColor, SkinColor},
    Human,
};

use crate::tree::CustomRender;

pub fn human(human: &Human) -> CustomRender {
    let human = human.clone();
    Box::new(move |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;

            match human.characteristics.skin_color {
                SkinColor::Black => ui.colored_label(Color32::BROWN, "É preto/a"),
                SkinColor::Dark => ui.colored_label(Color32::BROWN, "É pardo/a"),
                SkinColor::White => ui.colored_label(Color32::WHITE, "É branco/a"),
            };

            ui.label(" também tem ");

            match human.characteristics.hair_color {
                HairColor::Black => ui.colored_label(Color32::BROWN, "Cabelo Preto"),
                HairColor::Brown => ui.colored_label(Color32::BROWN, "Cabelo Marrom"),
                HairColor::Red => ui.colored_label(Color32::LIGHT_RED, "Cabelo Ruivo"),
                HairColor::Bloud => ui.colored_label(Color32::GRAY, "Cabelo Loiro"),
            };

            ui.label(" , ");

            match human.characteristics.eye_color {
                EyeColor::Blue => ui.colored_label(Color32::LIGHT_BLUE, "Olho Azul"),
                EyeColor::Brown => ui.colored_label(Color32::BROWN, "Olho Marrom"),
                EyeColor::Green => ui.colored_label(Color32::GREEN, "Olho Verde"),
            };

            ui.label(" e por fim ");

            match human.gender {
                Gender::Male => ui.colored_label(Color32::BLUE, "Homem"),
                Gender::Female => ui.colored_label(Color32::RED, "Mulher"),
            };
        });
    })
}

pub fn human_text(human: &Human) -> String {
    let mut res = String::new();

    match human.characteristics.skin_color {
        SkinColor::Black => res.push_str("É preto/a"),
        SkinColor::Dark => res.push_str("É pardo/a"),
        SkinColor::White => res.push_str("É branco/a"),
    };

    res += " também tem ";

    match human.characteristics.hair_color {
        HairColor::Black => res += "Cabelo Preto",
        HairColor::Brown => res += "Cabelo Marrom",
        HairColor::Red => res += "Cabelo Ruivo",
        HairColor::Bloud => res += "Cabelo Loiro",
    }

    res += ", ";

    match human.characteristics.eye_color {
        EyeColor::Blue => res += "Olho Azul",
        EyeColor::Brown => res += "Olho Marrom",
        EyeColor::Green => res += "Olho Verde",
    };

    res
}
