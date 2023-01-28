use rand::{distributions::Standard, prelude::Distribution, Rng};

#[derive(Debug, Clone)]
pub struct Characteristic {
    pub is_dominant: bool,
    pub kind: Kind,
}

#[derive(Debug, Clone)]
pub enum EyeColor {
    Blue,
    Brown,
    Green,
}

#[derive(Debug, Clone)]
pub enum SkinColor {
    White,
    Black,
}

#[derive(Debug, Clone)]
pub enum HairColor {
    White,
    Black,
}

#[derive(Debug, Clone)]
pub enum Kind {
    SkinColor(SkinColor),
    EyeColor(EyeColor),
    HairColor(HairColor),
}

impl Distribution<EyeColor> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EyeColor {
        match rng.gen_range(0..2) {
            0 => EyeColor::Blue,
            1 => EyeColor::Brown,
            _ => EyeColor::Green,
        }
    }
}

impl Distribution<SkinColor> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SkinColor {
        match rng.gen_range(0..1) {
            0 => SkinColor::Black,
            _ => SkinColor::White,
        }
    }
}

impl Distribution<HairColor> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HairColor {
        match rng.gen_range(0..1) {
            0 => HairColor::Black,
            _ => HairColor::White,
        }
    }
}

impl Distribution<Kind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Kind {
        match rng.gen_range(0..2) {
            0 => Kind::EyeColor(rng.gen()),
            1 => Kind::HairColor(rng.gen()),
            _ => Kind::SkinColor(rng.gen()),
        }
    }
}

impl Distribution<Characteristic> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Characteristic {
        Characteristic {
            is_dominant: rng.gen(),
            kind: rng.gen(),
        }
    }
}
