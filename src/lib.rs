
#![feature(libc)]

extern crate gedcomx_date;
extern crate libc;

use std::ffi::{CStr, CString};

use gedcomx_date::{GedcomxDate, Simple, Range, Recurring, DateTime, Date, Time, DateTimeOrDuration};

trait ToEdtf {
    fn to_edtf(&self) -> String;
}

fn fill_double_figure(input: u32) -> String {
    let input_string = input.to_string();
    if input_string.len() == 1 {
        "0".to_string() + &input_string
    } else {
        input_string
    }
}

fn fill_tz_hours(input: i32) -> String {
    if input > 0 {
        "+".to_string() + &fill_double_figure(input.abs() as u32)
    } else {
        "-".to_string() + &fill_double_figure(input.abs() as u32)
    }
}

impl ToEdtf for Date {
    fn to_edtf(&self) -> String {
        let y = if self.year <= 0 {
            self.year -1
        } else {
            self.year
        };
        y.to_string() +
        &match self.month {
            Some(m) => {
                "-".to_string() + &fill_double_figure(m) +
                &match self.day {
                    Some(d) => "-".to_string() + &fill_double_figure(d),
                    None => "".to_string(),
                }
            }
            None => "".to_string(),
        }
    }
}


impl ToEdtf for Time {
    fn to_edtf(&self) -> String {
        "T".to_string() + &fill_double_figure(self.hours) +
        &match self.minutes {
            Some(m) => {
                ":".to_string() + &fill_double_figure(m) +
                &match self.tz_offset_hours {
                    Some(th) => {
                        fill_tz_hours(th) +
                        &match self.tz_offset_minutes {
                            Some(tm) => ":".to_string() + &fill_double_figure(tm as u32),
                            None => "".to_string(),
                        }
                    }
                    None => "".to_string(),
                }
            }
            None => "".to_string(),
        }
    }
}

impl ToEdtf for DateTime {
    fn to_edtf(&self) -> String {
        let date = self.date;
        let time = self.time;
        date.to_edtf() +
        &match time {
            Some(t) => t.to_edtf(),
            None => "".to_string(),
        }
    }
}

impl ToEdtf for Simple {
    fn to_edtf(&self) -> String {
        let date = self.date;
        let time = self.time;
        let approximate = self.approximate;
        date.to_edtf() +
        &match time {
            Some(t) => t.to_edtf(),
            None => "".to_string(),
        } +
        &match approximate {
            true => "~".to_string(),
            false => "".to_string(),
        }
    }
}

fn get_range_datetime(date: DateTime, approximate: bool) -> String {
    match approximate {
        true => date.to_edtf() + "~",
        false => date.to_edtf()
    }
}

impl ToEdtf for Range {
    fn to_edtf(&self) -> String {
        let start = self.start;
        let end = self.end;
        let approximate = self.approximate;
        match start {
            Some(s) => {
                let start_string = get_range_datetime(s, approximate);
                match end {
                    Some(e) =>
                        match e {
                            DateTimeOrDuration::DateTime(d) => start_string + "/" + &get_range_datetime(d, approximate),
                            DateTimeOrDuration::Duration(_) => "".to_string(),
                        },
                    None => "[".to_string() + &start_string + ",..]",
                }
            },
            None => {
                match end {
                    Some(e) =>
                        match e {
                            DateTimeOrDuration::DateTime(d) => "[..,".to_string() + &get_range_datetime(d, approximate) + "]",
                            DateTimeOrDuration::Duration(_) => "".to_string(),
                        },
                    None => "".to_string(),
                }
            },
        }
    }
}

impl ToEdtf for Recurring {
    fn to_edtf(&self) -> String {
        "recurring dates are not supported yet".to_string()
    }
}

impl ToEdtf for GedcomxDate {
    fn to_edtf(&self) -> String {
        match *self {
            GedcomxDate::Simple(simple) => simple.to_edtf(),
            GedcomxDate::Range(range) => range.to_edtf(),
            GedcomxDate::Recurring(recurring) => recurring.to_edtf(),
        }
    }
}

#[no_mangle]
pub extern fn convert(gedcomx: *const libc::c_char) -> *const libc::c_char {
    let s = unsafe { CStr::from_ptr(gedcomx) };
    let r = conv(s.to_str().unwrap());
    CString::new(r).unwrap().into_raw()
}

pub fn conv(gedcomx: &str) -> String {
    match gedcomx_date::parse(gedcomx) {
        Ok(d) => d.to_edtf(),
        _ => "Parsing error".to_string(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn simple_date() {
        assert_eq!(convert("+2003"), "2003");
        assert_eq!(convert("+2003-03"), "2003-03");
        assert_eq!(convert("+2003-03-29"), "2003-03-29");
        assert_eq!(convert("+2003-03-29T11"), "2003-03-29T11");
        assert_eq!(convert("+2003-03-29T11:32"), "2003-03-29T11:32");
        // TODO: check - 00
        assert_eq!(convert("+2003-03-29T11:32Z"), "2003-03-29T11:32-00:00");
        // TODO: check + 00
        assert_eq!(convert("+2003-03-29T11:32+01"), "2003-03-29T11:32+01:00");
        assert_eq!(convert("+2003-03-29T11:32+01:30"), "2003-03-29T11:32+01:30");
        assert_eq!(convert("A+2003"), "2003~");
    }

    #[test]
    fn range() {
        assert_eq!(convert("+2003/+2004"), "2003/2004");
        assert_eq!(convert("+2003-10/+2004-12"), "2003-10/2004-12");
        assert_eq!(convert("+2003/"), "[2003,..]");
        assert_eq!(convert("/+2003"), "[..,2003]");
        assert_eq!(convert("A+2003/+2004"), "2003~/2004~");
        assert_eq!(convert("A+2003-10/+2004-12"), "2003-10~/2004-12~");
        assert_eq!(convert("A+2003/"), "[2003~,..]");
        assert_eq!(convert("A/+2003"), "[..,2003~]");
    }
}
