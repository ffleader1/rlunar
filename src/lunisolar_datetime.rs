use anyhow::{Result, bail};
use chrono::{Datelike, DateTime, FixedOffset, Timelike};

const MIN_YEAR: i32 = 1900;
const MAX_YEAR: i32 = 2100;

#[derive(Clone, Debug)]
pub struct NaiveLunisolarDateTime {
    pub hour_lunisolar: u32,
    pub minute_lunisolar: u32,
    pub day_lunisolar: u32,
    pub month_lunisolar: u32,
    pub year_lunisolar: u32,
}
impl NaiveLunisolarDateTime {
    pub fn new_from_datetime(datetime: DateTime<FixedOffset>) -> Result<NaiveLunisolarDateTime>{
        let hour = datetime.hour();
        let minute = datetime.minute();

        let (day, month, year) = NaiveLunisolarDateTime::datetime_to_lunisolar(datetime)?;

        Ok(NaiveLunisolarDateTime{
            hour_lunisolar: hour,
            minute_lunisolar: minute,
            day_lunisolar: day,
            month_lunisolar: month,
            year_lunisolar: year,
        })
    }

    fn datetime_to_lunisolar(datetime: DateTime<FixedOffset>) -> Result<(u32, u32, u32)> {
        let day = datetime.day();
        let month = datetime.month();
        let year_i32 = datetime.year();
        if year_i32 < MIN_YEAR || year_i32 > MAX_YEAR {
            bail!("year should be between {} and {}", MIN_YEAR, MAX_YEAR);
        }

        let time_zone = datetime.timezone().local_minus_utc() as f64 / 3600_f64;
        let (d, m, y) = NaiveLunisolarDateTime::convert_solar_to_lunisolar(day, month, year_i32, time_zone);
        Ok((d, m, y as u32))
    }
    fn jd_from_date(dd: u32, mm: u32, yy: i32) -> i32 {
        let a = ((14 - mm) / 12) as i32;
        let y = yy + 4800 - a;
        let m = mm + 12 * a as u32 - 3;
        let jd = dd as i32 + ((153 * m + 2) / 5) as i32 + 365 * y + (y / 4) - (y / 100) + (y / 400) as i32 - 32045;

        if jd < 2299161 {
            return dd as i32 + ((153 * m + 2) / 5) as i32 + 365 * y + (y / 4) - 32083;
        }

        jd
    }

    #[allow(dead_code)]
    fn jd_to_date(jd: i32) -> (u32, u32, i32) {
        // Check if the Julian day is after October 4, 1582 (the date of the Gregorian calendar switch)
        let is_gregorian = jd > 2299160;

        return if is_gregorian {
            // Gregorian calendar
            let a = jd + 32044;
            let b = ((4 * a + 3) / 146097) as i32;
            let c = a - ((b * 146097) / 4) as i32;

            let d = ((4 * c + 3) / 1461) as i32;
            let e = c - ((1461 * d) / 4) as i32;
            let m = ((5 * e + 2) / 153) as i32;
            let day = e - ((153 * m + 2) / 5) + 1;
            let month = m + 3 - 12 * ((m / 10) as i32);
            let year = b * 100 + d - 4800 + ((m / 10) as i32);

            (day as u32, month as u32, year)
        } else {
            // Julian calendar
            let a = jd + 32082;
            let b = 0;
            let c = a;

            let d = ((4 * c + 3) / 1461) as i32;
            let e = c - ((1461 * d) / 4) as i32;
            let m = ((5 * e + 2) / 153) as i32;
            let day = e - ((153 * m + 2) / 5) + 1;
            let month = m + 3 - 12 * ((m / 10) as i32);
            let year = b * 100 + d - 4800 + ((m / 10) as i32);

            (day as u32, month as u32, year)
        };
    }

    fn get_new_moon_day(k_int: i32, time_zone: f64) -> i32 {
        const DR: f64 = std::f64::consts::PI / 180.0;
        let k = k_int as f64;
        let t = k / 1236.85; // Time in Julian centuries from 1900 January 0.5
        let t2 = t * t;
        let t3 = t2 * t;

        let jd1 = 2415020.75933 + 29.53058868 * k + 0.0001178 * t2 - 0.000000155 * t3;
        let jd1 = jd1
            + 0.00033 * f64::sin(166.56 + 132.87 * t - 0.009173 * t2) * DR; // Mean new moon

        let m = 359.2242 + 29.10535608 * k - 0.0000333 * t2 - 0.00000347 * t3; // Sun's mean anomaly
        let mpr = 306.0253 + 385.81691806 * k + 0.0107306 * t2 + 0.00001236 * t3; // Moon's mean anomaly
        let f = 21.2964 + 390.67050646 * k - 0.0016528 * t2 - 0.00000239 * t3; // Moon's argument of latitude

        let c1 = (0.1734 - 0.000393 * t) * f64::sin(m * DR) + 0.0021 * f64::sin(2.0 * DR * m);
        let c1 = c1
            - 0.4068 * f64::sin(mpr * DR)
            + 0.0161 * f64::sin(DR * 2.0 * mpr);
        let c1 = c1 - 0.0004 * f64::sin(DR * 3.0 * mpr);
        let c1 = c1
            + 0.0104 * f64::sin(DR * 2.0 * f)
            - 0.0051 * f64::sin(DR * (m + mpr));
        let c1 = c1
            - 0.0074 * f64::sin(DR * (m - mpr))
            + 0.0004 * f64::sin(DR * (2.0 * f + m));
        let c1 = c1
            - 0.0004 * f64::sin(DR * (2.0 * f - m))
            - 0.0006 * f64::sin(DR * (2.0 * f + mpr));
        let c1 = c1 + 0.0010 * f64::sin(DR * (2.0 * f - mpr)) + 0.0005 * f64::sin(DR * (2.0 * mpr + m));

        let deltat = if t < -11.0 {
            0.001 + 0.000839 * t + 0.0002261 * t2 - 0.00000845 * t3 - 0.000000081 * t * t3
        } else {
            -0.000278 + 0.000265 * t + 0.000262 * t2
        };

        let jd_new = jd1 + c1 - deltat;

        (jd_new + 0.5 + time_zone / 24_f64) as i32
    }

    fn get_sun_longitude(jdn: f64, time_zone: f64) -> f64 {
        const DR: f64 = std::f64::consts::PI / 180.0;

        let t = (jdn - 2451545.5 - time_zone / 24.0) / 36525.0; // Time in Julian centuries from 2000-01-01 12:00:00 GMT
        let t2 = t * t;

        let m = 357.52910 + 35999.05030 * t - 0.0001559 * t2 - 0.00000048 * t * t2; // mean anomaly, degree


        let l0 = 280.46645 + 36000.76983 * t + 0.0003032 * t2; // mean longitude, degree


        let dl = (1.914600 - 0.004817 * t - 0.000014 * t2) * f64::sin(DR * m);
        let dl = dl + (0.019993 - 0.000101 * t) * f64::sin(DR * 2.0 * m) + 0.000290 * f64::sin(DR * 3.0 * m);

        let l = l0 + dl; // true longitude, degree
        let l = l * DR;
        let l = l - std::f64::consts::PI * 2.0 * ((l / (std::f64::consts::PI * 2.0)).floor()); // Normalize to (0, 2*PI)

        l / std::f64::consts::PI * 6.0
    }

    fn get_lunisolar_month_11(yy: i32, time_zone: f64) -> i32 {
        let off = NaiveLunisolarDateTime::jd_from_date(31, 12, yy) as f64 - 2415021_f64;
        let k = (off / 29.530588853) as i32;
        let mut nm = NaiveLunisolarDateTime::get_new_moon_day(k, time_zone);
        let sun_long = NaiveLunisolarDateTime::get_sun_longitude(nm as f64, time_zone); // sun longitude at local midnight

        if sun_long >= 9.0 {
            nm = NaiveLunisolarDateTime::get_new_moon_day(k - 1, time_zone);
        }

        nm
    }

    fn get_leap_month_offset(a11: f64, time_zone: f64) -> i32 {
        let k = ((a11 - 2415021.076998695) / 29.530588853 + 0.5) as i32;
        let mut last:i32;
        let mut i = 1; // We start with the month following lunisolar month 11

        let mut arc = NaiveLunisolarDateTime::get_sun_longitude(NaiveLunisolarDateTime::get_new_moon_day(k + i, time_zone) as f64, time_zone) as i32;

        loop {
            last = arc;
            i += 1;
            arc = NaiveLunisolarDateTime::get_sun_longitude(NaiveLunisolarDateTime::get_new_moon_day(k + i, time_zone) as f64, time_zone) as i32;

            if arc == last || i == 14 {
                break;
            }
        }

        i - 1
    }

    fn convert_solar_to_lunisolar(dd: u32, mm: u32, yy: i32, time_zone: f64) -> (u32, u32, i32) {
        let day_number = NaiveLunisolarDateTime::jd_from_date(dd, mm, yy);
        let k = ((day_number as f64 - 2415021.076998695) / 29.530588853) as i32;
        let mut month_start = NaiveLunisolarDateTime::get_new_moon_day(k + 1, time_zone);

        if month_start > day_number {
            month_start = NaiveLunisolarDateTime::get_new_moon_day(k, time_zone);
        }

        let mut a11 = NaiveLunisolarDateTime::get_lunisolar_month_11(yy, time_zone);
        let mut b11 = a11;

        let mut lunisolar_year :i32;
        let mut lunisolar_month :i32;
        let lunisolar_day = day_number - month_start + 1;
        // let mut lunisolar_leap: bool;

        if a11 >= month_start {
            lunisolar_year = yy;
            a11 = NaiveLunisolarDateTime::get_lunisolar_month_11(yy - 1, time_zone);
        } else {
            lunisolar_year = yy + 1;
            b11 = NaiveLunisolarDateTime::get_lunisolar_month_11(yy + 1, time_zone);
        }


        let diff = ((month_start - a11) as f64 / 29.0) as i32;

        lunisolar_month = diff + 11;

        if b11 - a11 > 365 {
            let leap_month_diff = NaiveLunisolarDateTime::get_leap_month_offset(a11 as f64, time_zone) as i32;

            if diff >= leap_month_diff {
                lunisolar_month = diff + 10;
                // lunisolar_leap = diff == leap_month_diff;
            }
        }

        if lunisolar_month > 12 {
            lunisolar_month -= 12;
        }

        if lunisolar_month >= 11 && diff < 4 {
            lunisolar_year -= 1;
        }

        (lunisolar_day as u32, lunisolar_month as u32, lunisolar_year)
    }

    #[allow(dead_code)]
    fn convert_lunisolar_to_solar(lunisolar_day: u32, lunisolar_month: u32, lunisolar_year: i32, lunisolar_leap: bool, time_zone: f64) -> (u32, u32, i32) {
        let a11: i32;
        // let b11: i32;

        if lunisolar_month < 11 {
            a11 = NaiveLunisolarDateTime::get_lunisolar_month_11(lunisolar_year - 1, time_zone);
            //b11 = NaiveLunisolarDateTime::get_lunisolar_month_11(lunisolar_year, time_zone);
        } else {
            a11 = NaiveLunisolarDateTime::get_lunisolar_month_11(lunisolar_year, time_zone);
           //b11 = NaiveLunisolarDateTime::get_lunisolar_month_11(lunisolar_year + 1, time_zone);
        }

        let mut off = lunisolar_month as i32 - 11;
        if off < 0 {
            off += 12;
        }

        let leap_off = NaiveLunisolarDateTime::get_leap_month_offset(a11 as f64, time_zone) as i32;
        let mut leap_month = leap_off - 2;

        if leap_month < 0 {
            leap_month += 12;
        }

        if lunisolar_leap && lunisolar_month != leap_month as u32 {
            return (0, 0, 0);
        } else if lunisolar_leap || off >= leap_off {
            off += 1;
        }

        let k = ((a11 as f64 - 2415021.076998695) / 29.530588853 + 0.5) as i32;
        let month_start = NaiveLunisolarDateTime::get_new_moon_day(k + off, time_zone);

        let solar_day = month_start + lunisolar_day as i32 - 1;
        NaiveLunisolarDateTime::jd_to_date(solar_day)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;
    use chrono::{NaiveTime, NaiveDate, NaiveDateTime};


    #[rstest]
    #[case(2011, 1, 9, 7, 2010, 12, 6)]
    #[case(1925, 9, 3, 7, 1925, 7, 16)]
    #[case(1964, 4, 16, 7, 1964, 3, 5)]
    #[case(1890, 3, 2, 7, 0, 0, 0)]
    #[case(2007, 2, 17, 7, 2007, 1, 1)]
    #[case(2007, 2, 17, 8, 2006, 12, 30)]
    fn test_datetime_to_lunisolar(#[case] yyyy: i32, #[case] mm: u32, #[case] dd: u32, #[case] time_zone: i64,
                                  #[case] expected_yyyy: u32, #[case] expected_mm: u32, #[case] expected_dd: u32) {
        // let yyyy = 1996;
        // let mm = 4;
        // let dd = 21;
        let hour = 3;
        let min = 30;
        // let time_zone = 7;

        let date = NaiveDate::from_ymd_opt(yyyy, mm, dd).unwrap();
        let time = NaiveTime::from_hms_opt(hour, min, 0).unwrap();
        let naive_datetime = NaiveDateTime::new(date, time);

        let offset = FixedOffset::east_opt(chrono::Duration::hours(time_zone).num_seconds() as i32).unwrap();
        let datetime = naive_datetime.and_local_timezone(offset).unwrap();


        match NaiveLunisolarDateTime::datetime_to_lunisolar(datetime) {
            Ok((day, month, year)) => {
                assert_eq!(day, expected_dd);
                assert_eq!(month, expected_mm);
                assert_eq!(year, expected_yyyy);
            }
            Err(e) => {
                if expected_yyyy != 0 {
                    panic!("Error {}", e);
                }
            }
        };
    }
}