use rand::{distributions::Standard, prelude::Distribution, Rng};

#[derive(Debug, Clone, PartialOrd, Eq, Hash, Ord, Default, PartialEq)]
pub struct Gene {
    pub dominant: bool,
    pub kind: Kind,
}

#[derive(Debug, Clone, PartialOrd, Eq, Hash, Ord, PartialEq, Default)]
pub enum EyeColor {
    #[default]
    Blue,
    Brown,
    Green,
}

#[derive(Debug, Clone, PartialOrd, Eq, Hash, Ord, PartialEq, Default)]
pub enum SkinColor {
    #[default]
    White,
    Black,
}

#[derive(Debug, Clone, PartialOrd, Eq, Hash, Ord, Default, PartialEq)]
pub enum HairColor {
    #[default]
    White,
    Black,
    RedHead,
}

#[derive(Debug, Clone, PartialOrd, Eq, Hash, Ord, PartialEq)]
pub enum Kind {
    SkinColor(SkinColor),
    EyeColor(EyeColor),
    HairColor(HairColor),
}

impl Default for Kind {
    fn default() -> Self {
        Self::SkinColor(Default::default())
    }
}

impl Distribution<EyeColor> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EyeColor {
        match rng.gen_range(0..=2) {
            0 => EyeColor::Blue,
            1 => EyeColor::Brown,
            _ => EyeColor::Green,
        }
    }
}

impl Distribution<SkinColor> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SkinColor {
        match rng.gen_range(0..=1) {
            0 => SkinColor::Black,
            _ => SkinColor::White,
        }
    }
}

impl Distribution<HairColor> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HairColor {
        match rng.gen_range(0..=2) {
            0 => HairColor::Black,
            1 => HairColor::RedHead,
            _ => HairColor::White,
        }
    }
}

impl Distribution<Kind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Kind {
        match rng.gen_range(0..=2) {
            0 => Kind::EyeColor(rng.gen()),
            1 => Kind::HairColor(rng.gen()),
            _ => Kind::SkinColor(rng.gen()),
        }
    }
}

impl Distribution<Gene> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Gene {
        Gene {
            dominant: rng.gen(),
            kind: rng.gen(),
        }
    }
}
