use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

macro_rules! ratio {
        ($rng:ident:
            _ = $r:expr,
            $xa:tt / $ya:tt = $aexpr:expr,
            $($x:tt / $y:tt = $e:expr,)+ ) => {
        if $rng.gen_ratio($xa, $ya) {
            $aexpr
        } $(
            else if $rng.gen_ratio($x, $y) {
                $e
            }
        )+ else {
            $r
        }
    };
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
    Dark,
    Black,
}

#[derive(Debug, Clone, PartialOrd, Eq, Hash, Ord, Default, PartialEq)]
pub enum HairColor {
    #[default]
    Black,
    Red,
    Brown,
    Bloud,
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
        match rng.gen_range(0..=3) {
            0 => HairColor::Black,
            1 => HairColor::Red,
            2 => HairColor::Bloud,
            _ => HairColor::Brown,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Characteristics {
    pub eye_color: EyeColor,
    pub hair_color: HairColor,
    pub skin_color: SkinColor,
}

impl Distribution<Characteristics> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Characteristics {
        Characteristics {
            eye_color: rng.gen(),
            hair_color: rng.gen(),
            skin_color: rng.gen(),
        }
    }
}

#[must_use]
pub fn merge_skin(a: &SkinColor, b: &SkinColor) -> SkinColor {
    use SkinColor::{Black, Dark, White};

    match (a, b) {
        (White, Black) | (Black, White) => Dark,
        (White, White) => White,
        (Dark, Dark) => Dark,
        (Dark, White) | (White, Dark) => White,
        (Dark | Black, Black) | (Black, Dark) => Black,
    }
}

#[must_use]
pub fn merge_eyes(a: &EyeColor, b: &EyeColor) -> EyeColor {
    use EyeColor::{Blue, Brown, Green};
    let mut rng = thread_rng();
    // Brown (2P):
    // 75% -> Brown
    // 18.8% -> Green
    // 6.3% Blue
    // Blue (2P)
    // 99% -> Blue
    // 1% -> Green
    // 0% -> Brown
    // Green (2P)
    // 75% Green
    // 25% Blue
    // 0% -> Brown

    match (a, b) {
        (Brown, Brown) => match rng.gen_range(0..100) {
            0..=74 => Brown,
            75..=93 => Green,
            94..=100 => Blue,
            _ => unreachable!(),
        },

        (Blue, Blue) => match rng.gen_range(0..100) {
            0..=98 => Blue,
            99..=100 => Green,
            _ => unreachable!(),
        },

        (Green, Green) => match rng.gen_range(0..100) {
            0..=74 => Green,
            75..=100 => Blue,
            _ => unreachable!(),
        },

        (Brown, Blue) | (Blue, Brown) => match rng.gen_range(0..100) {
            0..=48 => Brown,
            49..=100 => Blue,
            _ => unreachable!(),
        },

        (Blue, Green) | (Green, Blue) => match rng.gen_range(0..100) {
            0..=48 => Green,
            49..=100 => Blue,
            _ => unreachable!(),
        },

        (Brown, Green) | (Green, Brown) => match rng.gen_range(0..100) {
            0..=49 => Brown,
            50..=86 => Green,
            87..=100 => Blue,
            _ => unreachable!(),
        },
    }
}

#[must_use]
pub fn merge_hair(a: &HairColor, b: &HairColor) -> HairColor {
    use HairColor::{Black, Bloud, Brown, Red};

    let mut rng = thread_rng();
    match (a, b) {
        (Black, Black) => {
            if rng.gen_ratio(3, 100) {
                Bloud
            } else if rng.gen_ratio(6, 100) {
                Brown
            } else if rng.gen_ratio(2, 100) {
                Red
            } else {
                Black
            }
        }

        (Red, Red) => ratio! {
            rng:
                _ = Red,
                7 / 100 = Bloud,
                10 / 100 = Black,
                32 / 100 = Brown,
        },

        (Brown, Brown) => ratio! {
            rng:
                _ = Brown,
                8 / 100 = Bloud,
                6 / 100 = Black,
                3 / 100 = Red,
        },

        (Bloud, Bloud) => ratio! {
            rng:
                _ = Bloud,
                5 / 100 = Brown,
                2 / 100 = Red,
                6 / 100 = Black,
        },

        (Black, Red) | (Red, Black) => ratio! {
            rng:
                _ = Black,
                3 / 100 = Bloud,
                7 / 100 = Brown,
                6 / 100 = Red,
        },

        (Brown, Red) | (Red, Brown) => ratio! {
            rng:
              _ = Brown,
              10 / 100 = Bloud,
              23 / 100 = Red,
              6 / 100 = Black,
        },

        (Black, Brown) | (Black, Bloud) => ratio! {
            rng:
                _ = Black,
                8 / 100 = Bloud,
                7 / 100 = Brown,
                3 / 100 = Red,
        },

        (Red, Bloud) | (Bloud, Red) => ratio! {
            rng:
                _ = Red,
                6 / 100 = Black,
                36 / 100 = Brown,
                13 / 100 = Bloud,
        },

        (Brown, Black) | (Bloud, Black) => ratio! {
            rng:
                _ = Black,
                8 / 100 = Bloud,
                6 / 100 = Brown,
                3 / 100 = Red,
        },
        (Brown, Bloud) | (Bloud, Brown) => ratio! {
            rng:
                _ = Brown,
                23 / 100 = Bloud,
                3 / 100 = Red,
                7 / 100 = Black,
        },
    }
}

impl Characteristics {
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            eye_color: merge_eyes(&self.eye_color, &other.eye_color),
            skin_color: merge_skin(&self.skin_color, &other.skin_color),
            hair_color: merge_hair(&self.hair_color, &other.hair_color),
        }
    }
}
