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
