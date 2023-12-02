use chrono::{NaiveDate};
use anyhow::Result;
use chrono::{Datelike, DateTime, FixedOffset, Timelike};
use super::lunisolar_datetime::*;
use super::zodiac_and_element::*;
use std::cmp::PartialEq;

#[derive(Clone, Debug, PartialEq)]
pub enum HeavenlyStem {
    HS1,
    HS2,
    HS3,
    HS4,
    HS5,
    HS6,
    HS7,
    HS8,
    HS9,
    HS10,
}

impl HeavenlyStem {
    pub fn from_numeric(num: u32) -> HeavenlyStem {
        match num % 10 {
            0 => HeavenlyStem::HS1,
            1 => HeavenlyStem::HS2,
            2 => HeavenlyStem::HS3,
            3 => HeavenlyStem::HS4,
            4 => HeavenlyStem::HS5,
            5 => HeavenlyStem::HS6,
            6 => HeavenlyStem::HS7,
            7 => HeavenlyStem::HS8,
            8 => HeavenlyStem::HS9,
            9 => HeavenlyStem::HS10,
            _ => panic!("invalid numeric for heavenly stem")
        }
    }

    pub fn to_numeric(&self) -> u32 {
        match self {
            HeavenlyStem::HS1 => 0,
            HeavenlyStem::HS2 => 1,
            HeavenlyStem::HS3 => 2,
            HeavenlyStem::HS4 => 3,
            HeavenlyStem::HS5 => 4,
            HeavenlyStem::HS6 => 5,
            HeavenlyStem::HS7 => 6,
            HeavenlyStem::HS8 => 7,
            HeavenlyStem::HS9 => 8,
            HeavenlyStem::HS10 => 9,
        }
    }

    pub fn get_yinyang_elem(&self) -> (Element, YinYang){
        match self{
            HeavenlyStem::HS1 => {(Element::Wood, YinYang::Yang)}
            HeavenlyStem::HS2 => {(Element::Wood, YinYang::Yin)}
            HeavenlyStem::HS3 => {(Element::Fire, YinYang::Yang)}
            HeavenlyStem::HS4 => {(Element::Fire, YinYang::Yin)}
            HeavenlyStem::HS5 => {(Element::Earth, YinYang::Yang)}
            HeavenlyStem::HS6 => {(Element::Earth, YinYang::Yin)}
            HeavenlyStem::HS7 => {(Element::Metal, YinYang::Yang)}
            HeavenlyStem::HS8 => {(Element::Metal, YinYang::Yin)}
            HeavenlyStem::HS9 => {(Element::Water, YinYang::Yang)}
            HeavenlyStem::HS10 => {(Element::Water, YinYang::Yin)}
        }
    }
    pub fn lunar_y_from_lunisolar_y(year: u32) -> HeavenlyStem {
        HeavenlyStem::from_numeric((year + 6) % 10)
    }

    pub fn lunar_m_from_lunisolar_ym(month: u32, year: u32) -> HeavenlyStem {
        HeavenlyStem::from_numeric(year * 12 + month + 3)
    }

    pub fn lunar_d_from_gregorian_ymd(day: u32, month: u32, year: u32) -> HeavenlyStem {
        let first_date = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();
        let interest_date = NaiveDate::from_ymd_opt(year as i32, month, day).unwrap();
        let duration = interest_date.signed_duration_since(first_date).num_days();
        HeavenlyStem::from_numeric(duration as u32 % 10)
    }

    pub fn lunar_h_from_gregorian_hymd(hour: u32, day: u32, month: u32, year: u32) -> HeavenlyStem {
        let lunar_date = HeavenlyStem::lunar_d_from_gregorian_ymd(day, month, year);
        let mut hour = hour + 1;
        if hour >= 24 {
            hour = 0
        };
        hour /= 2;
        HeavenlyStem::from_numeric(hour + lunar_date.to_numeric() * 2)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EarthlyBranch {
    EB1,
    EB2,
    EB3,
    EB4,
    EB5,
    EB6,
    EB7,
    EB8,
    EB9,
    EB10,
    EB11,
    EB12,
}

impl EarthlyBranch {
    pub fn from_numeric(num: u32) -> EarthlyBranch {
        match num % 12 {
            0 => EarthlyBranch::EB1,
            1 => EarthlyBranch::EB2,
            2 => EarthlyBranch::EB3,
            3 => EarthlyBranch::EB4,
            4 => EarthlyBranch::EB5,
            5 => EarthlyBranch::EB6,
            6 => EarthlyBranch::EB7,
            7 => EarthlyBranch::EB8,
            8 => EarthlyBranch::EB9,
            9 => EarthlyBranch::EB10,
            10 => EarthlyBranch::EB11,
            11 => EarthlyBranch::EB12,
            _ => panic!("invalid numeric for heavenly stem")
        }
    }

    pub fn to_numeric(&self) -> u32 {
        match self {
            EarthlyBranch::EB1 => 0,
            EarthlyBranch::EB2 => 1,
            EarthlyBranch::EB3 => 2,
            EarthlyBranch::EB4 => 3,
            EarthlyBranch::EB5 => 4,
            EarthlyBranch::EB6 => 5,
            EarthlyBranch::EB7 => 6,
            EarthlyBranch::EB8 => 7,
            EarthlyBranch::EB9 => 8,
            EarthlyBranch::EB10 => 9,
            EarthlyBranch::EB11 => 10,
            EarthlyBranch::EB12 => 11,
        }
    }

    pub fn get_yinyang_elem(&self) -> (Element, YinYang){
        match self{
            EarthlyBranch::EB1 => {(Element::Water, YinYang::Yang)}
            EarthlyBranch::EB2 => {(Element::Earth, YinYang::Yin)}
            EarthlyBranch::EB3 => {(Element::Wood, YinYang::Yang)}
            EarthlyBranch::EB4 => {(Element::Wood, YinYang::Yin)}
            EarthlyBranch::EB5 => {(Element::Earth, YinYang::Yang)}
            EarthlyBranch::EB6 => {(Element::Fire, YinYang::Yin)}
            EarthlyBranch::EB7 => {(Element::Fire, YinYang::Yang)}
            EarthlyBranch::EB8 => {(Element::Earth, YinYang::Yin)}
            EarthlyBranch::EB9 => {(Element::Metal, YinYang::Yang)}
            EarthlyBranch::EB10 => {(Element::Fire, YinYang::Yin)}
            EarthlyBranch::EB11 => {(Element::Earth, YinYang::Yang)}
            EarthlyBranch::EB12 => {(Element::Water, YinYang::Yin)}
        }
    }
    pub fn as_zodiac(&self) -> Zodiac{
        match self{
            EarthlyBranch::EB1 => {Zodiac::Rat}
            EarthlyBranch::EB2 => {Zodiac::Buffalo}
            EarthlyBranch::EB3 => {Zodiac::Tiger}
            EarthlyBranch::EB4 => {Zodiac::Cat}
            EarthlyBranch::EB5 => {Zodiac::Dragon}
            EarthlyBranch::EB6 => {Zodiac::Snake}
            EarthlyBranch::EB7 => {Zodiac::Horse}
            EarthlyBranch::EB8 => {Zodiac::Goat}
            EarthlyBranch::EB9 => {Zodiac::Monkey}
            EarthlyBranch::EB10 => {Zodiac::Chicken}
            EarthlyBranch::EB11 => {Zodiac::Dog}
            EarthlyBranch::EB12 => {Zodiac::Pig}
        }
    }
    pub fn lunar_y_from_lunisolar_y(year: u32) -> EarthlyBranch {
        EarthlyBranch::from_numeric((year + 8) % 12)
    }

    pub fn lunar_m_from_lunisolar_m(month: u32) -> EarthlyBranch {
        EarthlyBranch::from_numeric((month + 1) % 12)
    }

    pub fn lunar_d_from_gregorian_ymd(day: u32, month: u32, year: u32) -> EarthlyBranch {
        let first_date = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();
        let interest_date = NaiveDate::from_ymd_opt(year as i32, month, day).unwrap();
        let duration = interest_date.signed_duration_since(first_date).num_days();
        EarthlyBranch::from_numeric((duration + 10) as u32 % 12)
    }

    pub fn lunar_h_from_gregorian_h(hour: u32) -> EarthlyBranch {
        EarthlyBranch::from_numeric( ((hour+1)/2 )%12)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LunarFormat {
    pub stem: HeavenlyStem,
    pub branch: EarthlyBranch,
}

impl LunarFormat {
    pub fn new(stem: HeavenlyStem, branch: EarthlyBranch) -> LunarFormat{
        LunarFormat{
            stem, branch
        }
    }
}

// impl PartialEq for LunarFormat{
//     fn eq(&self, other: &Self) -> bool {
//         self.stem == other.stem && self.branch == other.branch
//     }
//
//     fn ne(&self, other: &Self) -> bool {
//         self.stem != other.stem || self.branch != other.branch
//     }
// }
#[derive(Clone, Debug)]
pub struct NaiveLunarDateTime {
    pub hour_lunar: LunarFormat,
    pub date_lunar: LunarFormat,
    pub month_lunar: LunarFormat,
    pub year_lunar: LunarFormat,
}

impl NaiveLunarDateTime {
    pub fn new_from_datetime(datetime: DateTime<FixedOffset>) -> Result<NaiveLunarDateTime>{
        let naive_lunisolar = NaiveLunisolarDateTime::new_from_datetime(datetime.clone())?;
        let hour = datetime.hour();
        let day = datetime.day();
        let month = datetime.month();
        let year =  datetime.year() as u32;
        let hour_hs =  HeavenlyStem::lunar_h_from_gregorian_hymd(hour.clone(), day.clone(), month.clone(), year.clone());
        let hour_eb =  EarthlyBranch::lunar_h_from_gregorian_h(hour.clone());

        let day_hs =  HeavenlyStem::lunar_d_from_gregorian_ymd(day.clone(), month.clone(), year.clone());
        let day_eb =  EarthlyBranch::lunar_d_from_gregorian_ymd(day.clone(), month.clone(), year.clone());

        let month_hs =  HeavenlyStem::lunar_m_from_lunisolar_ym(naive_lunisolar.month_lunisolar.clone(), naive_lunisolar.year_lunisolar.clone());
        let month_eb =  EarthlyBranch::lunar_m_from_lunisolar_m(naive_lunisolar.month_lunisolar.clone());

        let year_hs =  HeavenlyStem::lunar_y_from_lunisolar_y( naive_lunisolar.year_lunisolar.clone());
        let year_eb =  EarthlyBranch::lunar_y_from_lunisolar_y(naive_lunisolar.year_lunisolar.clone());

        Ok(NaiveLunarDateTime{
            hour_lunar: LunarFormat { stem: hour_hs, branch: hour_eb },
            date_lunar: LunarFormat { stem: day_hs, branch: day_eb },
            month_lunar: LunarFormat { stem: month_hs, branch: month_eb },
            year_lunar: LunarFormat { stem: year_hs, branch: year_eb },
        })

    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_cal_year_from_lunisolar_y() {
        assert_eq!(HeavenlyStem::lunar_y_from_lunisolar_y(1996), HeavenlyStem::HS3);
        assert_eq!(EarthlyBranch::lunar_y_from_lunisolar_y(1996), EarthlyBranch::EB1)
    }

    #[test]
    fn test_cal_month_from_lunisolar_ym() {
        assert_eq!(HeavenlyStem::lunar_m_from_lunisolar_ym(3, 1996), HeavenlyStem::HS9);
        assert_eq!(EarthlyBranch::lunar_m_from_lunisolar_m(3), EarthlyBranch::EB5);
    }

    #[test]
    fn test_cal_date_from_gregorian_ymd() {
        assert_eq!(HeavenlyStem::lunar_d_from_gregorian_ymd(1, 1, 1900), HeavenlyStem::HS1);
        assert_eq!(HeavenlyStem::lunar_d_from_gregorian_ymd(2, 1, 1950), HeavenlyStem::HS4);
        assert_eq!(HeavenlyStem::lunar_d_from_gregorian_ymd(21, 4, 1996), HeavenlyStem::HS5);
        assert_eq!(EarthlyBranch::lunar_d_from_gregorian_ymd(21, 4, 1996), EarthlyBranch::EB1);
    }

    #[test]
    fn test_cal_time_from_gregorian_hymd() {
        assert_eq!(HeavenlyStem::lunar_h_from_gregorian_hymd(3, 21, 4, 1996), HeavenlyStem::HS1);
        assert_eq!(HeavenlyStem::lunar_h_from_gregorian_hymd(10, 9, 11, 1999), HeavenlyStem::HS8);
        assert_eq!(EarthlyBranch::lunar_h_from_gregorian_h(10), EarthlyBranch::EB6);
    }
}
