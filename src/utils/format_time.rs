use gloo::console::{self};
use math::round;

pub fn calculate_timer(time_in_seconds: usize) -> [String; 3] {
    let hours = round::floor(time_in_seconds as f64 / 3600.0, 0);
    let minutes = round::floor((time_in_seconds as f64 - hours * 3600.0) / 60.0, 0);
    let seconds = round::floor(
        (time_in_seconds as f64 - hours * 3600.0) - minutes * 60.0,
        0,
    );

    let formated_hours = if hours < 10.0 {
        format!("0{hours}")
    } else {
        hours.to_string()
    };

    let formated_minutes = if minutes < 10.0 {
        format!("0{minutes}")
    } else {
        minutes.to_string()
    };

    let formated_seconds = if seconds < 10.0 {
        format!("0{seconds}")
    } else {
        seconds.to_string()
    };

    [formated_hours, formated_minutes, formated_seconds]
}

#[test]
fn it_returns_hours_min_sec_values() {
    // 60s -> 00:01:00
    assert_eq!(
        calculate_timer(60),
        ["00".to_string(), "01".to_string(), "00".to_string()]
    );
    // 130s -> 00:03:10
    assert_eq!(
        calculate_timer(130),
        ["00".to_string(), "02".to_string(), "10".to_string()]
    );
    // 50_000s -> 13:53:20
    assert_eq!(
        calculate_timer(50_000),
        ["13".to_string(), "53".to_string(), "20".to_string()]
    );
}

pub fn fix_two_digits(num: u32) -> String {
    if num < 10 {
        return format!("0{}", num.to_string());
    }
    num.to_string()
}

#[test]
fn it_returns_fix_two_digits() {
    assert_eq!(fix_two_digits(60), "60".to_string());
    assert_eq!(fix_two_digits(10), "10".to_string());
    assert_eq!(fix_two_digits(8), "08".to_string());
}

pub fn convert_time_to_am_pm(hour: u32) -> String {
    let mut part_of_day = "".to_string();
    let mut hours_display = 0;

    if hour < 1 {
        hours_display = hour + 12;
    }

    if hour >= 1 && hour < 12 {
        hours_display = hour;
    }

    if hour >= 12 && hour < 13 {
        hours_display = hour;
    }

    if hour >= 13 && hour < 24 {
        hours_display = hour - 12;
    }

    if hour >= 12 {
        part_of_day = "PM".to_string();
    };

    if hour < 12 {
        part_of_day = "AM".to_string();
    };

    todo!("recieve a Date and return a string in the format '10:30 PM'");
    return format!("{} {}", hours_display, part_of_day);
}

#[test]
fn it_convert_24h_time_to_am_pm() {
    assert_eq!(convert_time_to_am_pm(0), "12 AM".to_string());
    assert_eq!(convert_time_to_am_pm(1), "1 AM".to_string());
    assert_eq!(convert_time_to_am_pm(10), "10 AM".to_string());
    assert_eq!(convert_time_to_am_pm(12), "12 PM".to_string());
    assert_eq!(convert_time_to_am_pm(13), "1 PM".to_string());
    assert_eq!(convert_time_to_am_pm(20), "8 PM".to_string());
}
