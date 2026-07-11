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
    /// The Claude-brand fill color for filled bar cells.
    pub fill: u8,
    pub width: usize,
    pub filled: String,
    pub empty: String,
    /// Render the bar with hi-res braille dots (2 sub-columns per cell) instead
    /// of block glyphs. Each cell rests on a baseline so empty cells stay visible.
    pub braille: bool,
}

pub fn filled_cells(pct: u8, width: usize) -> usize {
    (pct.min(100) as usize * width + 50) / 100
}

/// Render a bar of length `bar_pct` with an explicit fill color. Splitting the
/// color out keeps the door open for per-row coloring; today every row uses the
/// brand fill.
pub fn bar_colored(bar_pct: u8, color: u8, s: &Style) -> String {
    if s.braille {
        return braille_bar(bar_pct, color, s);
    }
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

/// Hi-res braille bar. Each cell is a braille glyph (base U+2800) with two
/// horizontal sub-columns, so `width` cells give `2*width` steps of resolution.
/// Dots 7,8 (`0xC0`) are always lit as a baseline; the left sub-column adds dots
/// 1,2,3 (`0x07`) and the right adds 4,5,6 (`0x38`). A cell with any lit column
/// is drawn in the fill color, otherwise the track color.
fn braille_bar(bar_pct: u8, color: u8, s: &Style) -> String {
    let subcols = 2 * s.width;
    let filled = (bar_pct.min(100) as usize * subcols + 50) / 100;
    let fill_c = format!("\x1b[38;5;{}m", color);
    let track_c = format!("\x1b[38;5;{}m", s.track);
    let mut out = String::new();
    for cell in 0..s.width {
        let left = 2 * cell < filled;
        let right = 2 * cell + 1 < filled;
        let mut bits: u32 = 0xC0; // baseline dots 7,8
        if left {
            bits |= 0x07; // dots 1,2,3
        }
        if right {
            bits |= 0x38; // dots 4,5,6
        }
        let glyph = char::from_u32(0x2800 + bits).unwrap_or('⣀');
        out.push_str(if left || right { &fill_c } else { &track_c });
        out.push(glyph);
    }
    out.push_str("\x1b[0m");
    out
}

/// Brand-colored bar: length tracks `pct`, fill is the brand color.
pub fn bar(pct: u8, s: &Style) -> String {
    bar_colored(pct, s.fill, s)
}

/// Bold model-name header line, with an optional dim plan/tier label appended
/// (e.g. "Opus 4.8 (1M context)  Max (20x)").
pub fn header(model: &str, plan: &str, s: &Style) -> String {
    if plan.is_empty() {
        format!("\x1b[1m{model}\x1b[0m")
    } else {
        format!("\x1b[1m{model}\x1b[0m  \x1b[38;5;{dim}m{plan}\x1b[0m", dim = s.dim)
    }
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
            fill: 68,
            width: 12,
            filled: "#".into(),
            empty: ".".into(),
            braille: false,
        }
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
    fn bar_uses_brand_fill() {
        let s = test_style();
        let b = bar(50, &s);
        assert!(b.starts_with("\x1b[38;5;68m")); // brand blue fill
        assert_eq!(b.matches('#').count(), 6);
        assert_eq!(b.matches('.').count(), 6);
        assert!(b.ends_with("\x1b[0m"));
    }

    #[test]
    fn braille_bar_renders_dots() {
        let mut s = test_style();
        s.braille = true;
        // 100% -> every cell full (⣿); 0% -> every cell baseline (⣀).
        assert_eq!(bar(100, &s).matches('⣿').count(), 12);
        assert_eq!(bar(0, &s).matches('⣀').count(), 12);
        // 50% -> half full cells, half baseline; orange fill leads.
        let mid = bar(50, &s);
        assert_eq!(mid.matches('⣿').count(), 6);
        assert_eq!(mid.matches('⣀').count(), 6);
        assert!(mid.starts_with("\x1b[38;5;68m"));
        assert!(mid.ends_with("\x1b[0m"));
    }

    #[test]
    fn header_is_bold() {
        let s = test_style();
        assert_eq!(header("Opus 4.8", "", &s), "\x1b[1mOpus 4.8\x1b[0m");
        let with_plan = header("Opus 4.8", "Max (20x)", &s);
        assert!(with_plan.starts_with("\x1b[1mOpus 4.8\x1b[0m"));
        assert!(with_plan.contains("Max (20x)"));
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
}
