use chrono::{Local, DateTime};
use chrono::{Datelike, Timelike};
use astro::coords::*;
use astro::time::*;

pub fn eq_to_altaz(target : EqPoint, lat : f64, long : f64) -> (f64,f64)
{
    let localtime : DateTime<Local> = Local::now();
    let gregorian : Date = Date {
        year : localtime.year() as i16,
        month : localtime.month() as u8,
        decimal_day : localtime.day() as f64 + (localtime.hour() as f64)/24.0 + (localtime.minute() as f64)/(24.0*60.0) + (localtime.second() as f64)/(24.0*60.0*60.0) + (localtime.nanosecond() as f64)/(24.0*60.0*60.0*1_000_000_000.0),
        cal_type : CalType::Gregorian
    };
    let jday = julian_day(&gregorian);
    let localsidereal = mn_sidr(jday);
    let hour_angle = hr_angl_frm_loc_sidr(localsidereal, target.asc);
    let alt = alt_frm_eq(hour_angle, target.dec, lat);
    let az = az_frm_eq(hour_angle, target.dec, lat);
    (alt,az)
}