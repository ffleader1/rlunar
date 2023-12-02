mod lunar_datetime;
mod lunisolar_datetime;
mod zodiac_and_element;
mod localization;

use lunar_datetime::*;

use lunisolar_datetime::*;
use chrono::prelude::*;
use chrono::{NaiveTime, NaiveDate};
use anyhow::Result;


#[derive(Clone, Debug)]
pub struct LunarDateTimeObject {
    pub lunisolar_datetime: NaiveLunisolarDateTime,
    pub lunar_datetime: NaiveLunarDateTime,
    pub gregorian_datetime: DateTime<FixedOffset>
}

impl LunarDateTimeObject {
    fn new_from_gregorian_datetime(dd: u32, mm:u32, yyyy:i32, hour:u32, min:u32, time_zone: i64) -> Result<LunarDateTimeObject>{
        let date = NaiveDate::from_ymd_opt(yyyy, mm, dd).unwrap();
        let time = NaiveTime::from_hms_opt(hour, min, 0).unwrap();
        let naive_datetime = NaiveDateTime::new(date, time);

        let offset = FixedOffset::east_opt(chrono::Duration::hours(time_zone).num_seconds() as i32).unwrap();
        let gregorian_datetime = naive_datetime.and_local_timezone(offset).unwrap();

        let lunisolar_datetime = NaiveLunisolarDateTime::new_from_datetime(gregorian_datetime.clone())?;
        let lunar_datetime = NaiveLunarDateTime::new_from_datetime(gregorian_datetime.clone())?;

        Ok(LunarDateTimeObject {
            lunisolar_datetime,
            lunar_datetime,
            gregorian_datetime,
        })
    }
}


#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[rstest]
    #[case(2011, 1, 9, 10, 25, 7,
    LunarFormat{stem: HeavenlyStem::HS_CANH, branch: EarthlyBranch:: EB_TIGER},
    LunarFormat{stem: HeavenlyStem::HS_KY, branch: EarthlyBranch:: EB_BUFFALO})]

    fn test_lunisolar_object(#[case] yyyy: i32, #[case] mm: u32, #[case] dd: u32,
                             #[case] hour: u32, #[case] min: u32,#[case] time_zone: i64,
                             #[case] expect_lunar_year: LunarFormat, #[case] expect_lunar_month: LunarFormat) {
        match LunarDateTimeObject::new_from_gregorian_datetime(dd, mm, yyyy, hour, min, time_zone){
            Ok(lo) =>{
                assert_eq!(lo.lunar_datetime.year_lunar, expect_lunar_year);
                assert_eq!(lo.lunar_datetime.month_lunar, expect_lunar_month);
            }
            Err (e) => {
                println!("Error {:?}", e)
            }

        }

    }
}