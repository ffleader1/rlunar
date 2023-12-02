use super::lunar_datetime::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Zodiac {
    Rat,
    Buffalo,
    Tiger,
    Cat,
    Dragon,
    Snake,
    Horse,
    Goat,
    Monkey,
    Chicken,
    Dog,
    Pig,
}

impl Zodiac {
    pub fn as_zodiac(&self) -> EarthlyBranch {
        match self {
            Zodiac::Rat => { EarthlyBranch::EB1 }
            Zodiac::Buffalo => { EarthlyBranch::EB2 }
            Zodiac::Tiger => { EarthlyBranch::EB3 }
            Zodiac::Cat => { EarthlyBranch::EB4 }
            Zodiac::Dragon => { EarthlyBranch::EB5 }
            Zodiac::Snake => { EarthlyBranch::EB6 }
            Zodiac::Horse => { EarthlyBranch::EB7 }
            Zodiac::Goat => { EarthlyBranch::EB8 }
            Zodiac::Monkey => { EarthlyBranch::EB9 }
            Zodiac::Chicken => { EarthlyBranch::EB10 }
            Zodiac::Dog => { EarthlyBranch::EB11 }
            Zodiac::Pig => { EarthlyBranch::EB12 }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Element {
    Metal,
    Wood,
    Water,
    Fire,
    Earth,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum YinYang {
    Yin,
    // negative
    Yang, // positive
}

impl YinYang {
    pub const NEGATIVE: YinYang = YinYang::Yin;
    pub const POSITIVE: YinYang = YinYang::Yang;
}