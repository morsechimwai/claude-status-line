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

pub struct Style {
    pub fill: u8,
    pub track: u8,
    pub dim: u8,
    pub width: usize,
    pub filled: String,
    pub empty: String,
}

pub fn filled_cells(pct: u8, width: usize) -> usize {
    (pct.min(100) as usize * width + 50) / 100
}

pub fn bar(pct: u8, s: &Style) -> String {
    let f = filled_cells(pct, s.width);
    let e = s.width - f;
    let mut out = format!("\x1b[38;5;{}m", s.fill);
    for _ in 0..f {
        out.push_str(&s.filled);
    }
    out.push_str(&format!("\x1b[38;5;{}m", s.track));
    for _ in 0..e {
        out.push_str(&s.empty);
    }
    out.push_str("\x1b[0m");
    out
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

    fn test_style() -> Style {
        Style { fill: 173, track: 240, dim: 245, width: 12, filled: "#".into(), empty: ".".into() }
    }

    #[test]
    fn computes_filled_cells() {
        assert_eq!(filled_cells(0, 12), 0);
        assert_eq!(filled_cells(100, 12), 12);
        assert_eq!(filled_cells(50, 12), 6);
        assert_eq!(filled_cells(99, 12), 12);
        assert_eq!(filled_cells(4, 12), 0);   // rounds down
        assert_eq!(filled_cells(8, 12), 1);   // rounds up
        assert_eq!(filled_cells(200, 12), 12); // clamped
    }

    #[test]
    fn bar_glyph_counts() {
        let s = test_style();
        let b = bar(50, &s);
        assert_eq!(b.matches('#').count(), 6);
        assert_eq!(b.matches('.').count(), 6);
        assert!(b.starts_with("\x1b[38;5;173m"));
        assert!(b.ends_with("\x1b[0m"));
    }
}
