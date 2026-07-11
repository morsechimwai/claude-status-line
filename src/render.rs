pub fn fmt_tokens(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}m", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{}k", n / 1_000)
    } else {
        n.to_string()
    }
}

/// Human-readable remaining duration for a positive number of seconds.
/// `4d 6h` / `2h 15m` / `45m` / `<1m`. Callers pass only positive durations.
pub fn fmt_countdown(secs: i64) -> String {
    if secs < 60 {
        "<1m".to_string()
    } else if secs < 3600 {
        format!("{}m", secs / 60)
    } else if secs < 86_400 {
        format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
    } else {
        format!("{}d {}h", secs / 86_400, (secs % 86_400) / 3600)
    }
}

pub struct Style {
    pub track: u8,
    pub dim: u8,
    pub good: u8,
    pub warn: u8,
    pub crit: u8,
    pub warn_at: u8,
    pub crit_at: u8,
    pub width: usize,
    pub filled: String,
    pub empty: String,
}

/// Threshold color for a bar at `pct`: good below `warn_at`, warn up to
/// `crit_at`, crit at or above `crit_at`.
pub fn fill_color(pct: u8, s: &Style) -> u8 {
    if pct >= s.crit_at {
        s.crit
    } else if pct >= s.warn_at {
        s.warn
    } else {
        s.good
    }
}

pub fn filled_cells(pct: u8, width: usize) -> usize {
    (pct.min(100) as usize * width + 50) / 100
}

/// Render a bar whose length is `bar_pct` and whose fill uses an explicit
/// 256-color. Splitting length from color lets a "remaining" bar show headroom
/// while still coloring by the underlying danger level.
pub fn bar_colored(bar_pct: u8, color: u8, s: &Style) -> String {
    let f = filled_cells(bar_pct, s.width);
    let e = s.width - f;
    let mut out = format!("\x1b[38;5;{}m", color);
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

/// Used/fullness bar: length and threshold color both track `pct`.
pub fn bar(pct: u8, s: &Style) -> String {
    bar_colored(pct, fill_color(pct, s), s)
}

/// Bold model-name header line.
pub fn header(model: &str) -> String {
    format!("\x1b[1m{model}\x1b[0m")
}

/// One aligned gauge row: bold label (padded to 8), bar, right-aligned
/// percentage, then the value — columns line up across rows.
pub fn row(label: &str, pct: Option<u8>, value: &str, s: &Style) -> String {
    let dim = format!("\x1b[38;5;{}m", s.dim);
    let (pdisp, bar_pct) = match pct {
        None => (format!("{:>4}", "--"), 0u8),
        Some(p) => (format!("{:>3}%", p), p),
    };
    format!(
        "\x1b[1m{label:<8}\x1b[0m {barstr}  {dim}{pdisp}\x1b[0m  {dim}{value}\x1b[0m",
        barstr = bar(bar_pct, s),
    )
}

/// A remaining/headroom gauge row: the bar length and the `N% left` label show
/// what is LEFT, while the color still reflects danger (driven by the underlying
/// USED percentage — a short red bar means little left). `used` is 0..=100.
pub fn row_remaining(label: &str, used: Option<u8>, value: &str, s: &Style) -> String {
    let dim = format!("\x1b[38;5;{}m", s.dim);
    let (pdisp, barstr) = match used {
        // Pad to the same 9-char width as "NNN% left" so the value column stays
        // aligned when a sibling row has data and this one does not.
        None => (format!("{:>9}", "--"), bar_colored(0, s.track, s)),
        Some(u) => {
            let u = u.min(100);
            let remaining = 100 - u;
            (
                format!("{:>3}% left", remaining),
                bar_colored(remaining, fill_color(u, s), s),
            )
        }
    };
    format!("\x1b[1m{label:<8}\x1b[0m {barstr}  {dim}{pdisp}\x1b[0m  {dim}{value}\x1b[0m")
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
    fn formats_countdown() {
        assert_eq!(fmt_countdown(30), "<1m");
        assert_eq!(fmt_countdown(45 * 60), "45m");
        assert_eq!(fmt_countdown(2 * 3600 + 15 * 60), "2h 15m");
        assert_eq!(fmt_countdown(4 * 86_400 + 6 * 3600), "4d 6h");
        assert_eq!(fmt_countdown(60), "1m"); // minute boundary
        assert_eq!(fmt_countdown(3600), "1h 0m"); // hour boundary
        assert_eq!(fmt_countdown(86_400), "1d 0h"); // day boundary
    }

    fn test_style() -> Style {
        Style {
            track: 240,
            dim: 245,
            good: 71,
            warn: 179,
            crit: 167,
            warn_at: 50,
            crit_at: 80,
            width: 12,
            filled: "#".into(),
            empty: ".".into(),
        }
    }

    #[test]
    fn threshold_fill_color() {
        let s = test_style();
        assert_eq!(fill_color(10, &s), 71); // good
        assert_eq!(fill_color(49, &s), 71);
        assert_eq!(fill_color(50, &s), 179); // warn at cutoff
        assert_eq!(fill_color(79, &s), 179);
        assert_eq!(fill_color(80, &s), 167); // crit at cutoff
        assert_eq!(fill_color(100, &s), 167);
    }

    #[test]
    fn computes_filled_cells() {
        assert_eq!(filled_cells(0, 12), 0);
        assert_eq!(filled_cells(100, 12), 12);
        assert_eq!(filled_cells(50, 12), 6);
        assert_eq!(filled_cells(99, 12), 12);
        assert_eq!(filled_cells(4, 12), 0);
        assert_eq!(filled_cells(8, 12), 1);
        assert_eq!(filled_cells(200, 12), 12);
    }

    #[test]
    fn bar_uses_threshold_color() {
        let s = test_style();
        let good = bar(30, &s);
        assert!(good.starts_with("\x1b[38;5;71m")); // green fill
        let crit = bar(90, &s);
        assert!(crit.starts_with("\x1b[38;5;167m")); // red fill
        assert_eq!(bar(50, &s).matches('#').count(), 6);
        assert_eq!(bar(50, &s).matches('.').count(), 6);
        assert!(crit.ends_with("\x1b[0m"));
    }

    #[test]
    fn header_is_bold() {
        assert_eq!(header("Opus 4.8"), "\x1b[1mOpus 4.8\x1b[0m");
    }

    #[test]
    fn row_with_pct() {
        let s = test_style();
        let r = row("5h", Some(42), "resets in 2h 15m", &s);
        assert!(r.contains("\x1b[1m5h      \x1b[0m")); // bold label padded to 8
        assert!(r.contains(" 42%")); // right-aligned in width 3 + %
        assert!(r.contains("resets in 2h 15m"));
    }

    #[test]
    fn row_without_pct() {
        let s = test_style();
        let r = row("7d", None, "--", &s);
        assert!(r.contains("  --"));
        assert!(!r.contains('%'));
    }

    #[test]
    fn row_remaining_shows_headroom() {
        let s = test_style();
        // 42% used -> 58% left, still green (danger by used% = 42 < warn_at).
        let r = row_remaining("5h", Some(42), "2h 15m", &s);
        assert!(r.contains("58% left"));
        assert!(r.contains("2h 15m"));
        assert!(r.contains("\x1b[38;5;71m")); // green fill (headroom, low danger)
        assert_eq!(bar_colored(58, 71, &s).matches('#').count(), 7); // 58% of 12 ≈ 7 cells
    }

    #[test]
    fn row_remaining_low_is_red() {
        let s = test_style();
        // 88% used -> 12% left, red (danger by used% = 88 >= crit_at).
        let r = row_remaining("7d", Some(88), "4d 6h", &s);
        assert!(r.contains("12% left"));
        assert!(r.contains("\x1b[38;5;167m")); // red fill = little left + dangerous
    }

    #[test]
    fn row_remaining_without_data() {
        let s = test_style();
        let r = row_remaining("5h", None, "--", &s);
        assert!(r.contains("--"));
        assert!(!r.contains("% left"));
    }

    #[test]
    fn row_remaining_columns_align() {
        let s = test_style();
        // Visible width up to an (empty) value must match with and without data,
        // so the value column lines up across sibling rows.
        let visible = |x: String| -> usize {
            let mut n = 0;
            let mut it = x.chars().peekable();
            while let Some(c) = it.next() {
                if c == '\x1b' {
                    while let Some(d) = it.next() {
                        if d == 'm' {
                            break;
                        }
                    }
                } else {
                    n += 1;
                }
            }
            n
        };
        let with = visible(row_remaining("5h", Some(42), "", &s));
        let without = visible(row_remaining("5h", None, "", &s));
        assert_eq!(with, without, "value column must align with and without data");
    }
}
