use chrono::{DateTime, Datelike, TimeZone};

pub fn fmt_tokens(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}m", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{}k", n / 1_000)
    } else {
        n.to_string()
    }
}

pub fn fmt_reset<Tz: TimeZone>(reset: Option<DateTime<Tz>>, now: &DateTime<Tz>) -> String
where
    Tz::Offset: std::fmt::Display,
{
    let Some(r) = reset else {
        return "--".to_string();
    };
    let same_day = r.year() == now.year() && r.ordinal() == now.ordinal();
    if same_day {
        r.format("%-I:%M %p").to_string()
    } else {
        r.format("%b %-d, %-I:%M %p").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_tokens() {
        assert_eq!(fmt_tokens(0), "0");
        assert_eq!(fmt_tokens(999), "999");
        assert_eq!(fmt_tokens(1_000), "1k");
        assert_eq!(fmt_tokens(15_500), "15k");
        assert_eq!(fmt_tokens(1_000_000), "1.0m");
        assert_eq!(fmt_tokens(1_500_000), "1.5m");
    }

    #[test]
    fn formats_reset() {
        use chrono::{TimeZone, Utc};
        let now = Utc.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).unwrap();

        let same = Utc.with_ymd_and_hms(2024, 1, 15, 15, 30, 0).unwrap();
        assert_eq!(fmt_reset(Some(same), &now), "3:30 PM");

        let other = Utc.with_ymd_and_hms(2024, 1, 18, 15, 30, 0).unwrap();
        assert_eq!(fmt_reset(Some(other), &now), "Jan 18, 3:30 PM");

        assert_eq!(fmt_reset::<Utc>(None, &now), "--");
    }
}
