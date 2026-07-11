use ccstatus::{cache, config, input, render};
use chrono::Local;
use std::io::Read;
use std::io::Write;

/// A rate-limit row resolved from either live data or the cache.
struct Resolved {
    pct: Option<u8>,
    resets_at: Option<i64>,
}

fn clamp_pct(p: f64) -> u8 {
    p.max(0.0).min(100.0).floor() as u8
}

/// Resolve one rate-limit window: prefer live, else fall back to cache.
fn resolve_window(
    live: Option<&input::Window>,
    cached: Option<cache::CachedWindow>,
    now: i64,
) -> Resolved {
    if let Some(w) = live {
        if let Some(p) = w.used_percentage {
            return Resolved { pct: Some(clamp_pct(p)), resets_at: w.resets_at };
        }
    }
    match cached {
        Some(c) => {
            let rolled_over = c.resets_at.map(|r| r <= now).unwrap_or(false);
            let pct = if rolled_over { 0 } else { clamp_pct(c.used_percentage) };
            Resolved { pct: Some(pct), resets_at: c.resets_at }
        }
        None => Resolved { pct: None, resets_at: None },
    }
}

fn window_to_cache(live: Option<&input::Window>) -> Option<cache::CachedWindow> {
    live.and_then(|w| {
        w.used_percentage.map(|p| cache::CachedWindow {
            used_percentage: p,
            resets_at: w.resets_at,
        })
    })
}

/// A rate-limit gauge row: percentage bar plus a "resets in <countdown>" value,
/// or "--" when there is no future reset time to show.
fn resolved_row(label: &str, r: &Resolved, style: &render::Style, now_ts: i64) -> String {
    let value = match r.resets_at {
        Some(reset) if reset > now_ts => {
            format!("resets in {}", render::fmt_countdown(reset - now_ts))
        }
        _ => "--".to_string(),
    };
    render::row(label, r.pct, &value, style)
}

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().read_to_string(&mut buf);

    let inp = input::parse(&buf);
    let cfg = config::Config::load();
    let style = cfg.style();
    let now = Local::now();
    let now_ts = now.timestamp();

    let mut lines: Vec<String> = Vec::new();

    if cfg.layout.model_header {
        lines.push(render::header(&inp.model.display_name));
    }

    if cfg.rows.context {
        let size = inp.context_window.context_window_size as u64;
        let input_tokens = inp.context_window.total_input_tokens as u64;
        let output_tokens = inp.context_window.total_output_tokens as u64;
        let pct = clamp_pct(inp.context_window.used_percentage);
        let value = format!(
            "↑{} ↓{} / {}",
            render::fmt_tokens(input_tokens),
            render::fmt_tokens(output_tokens),
            render::fmt_tokens(size),
        );
        lines.push(render::row(&cfg.labels.context, Some(pct), &value, &style));
    }

    let cached = cache::load();
    let five = resolve_window(
        inp.rate_limits.five_hour.as_ref(),
        cached.as_ref().and_then(|c| c.five_hour.clone()),
        now_ts,
    );
    let seven = resolve_window(
        inp.rate_limits.seven_day.as_ref(),
        cached.as_ref().and_then(|c| c.seven_day.clone()),
        now_ts,
    );

    // Merge live into the loaded cache: carry forward a window that has no
    // live data (or no real percentage) this run instead of nulling it.
    let five_live = window_to_cache(inp.rate_limits.five_hour.as_ref());
    let seven_live = window_to_cache(inp.rate_limits.seven_day.as_ref());
    if five_live.is_some() || seven_live.is_some() {
        cache::store(&cache::CachedUsage {
            five_hour: five_live.or_else(|| cached.as_ref().and_then(|c| c.five_hour.clone())),
            seven_day: seven_live.or_else(|| cached.as_ref().and_then(|c| c.seven_day.clone())),
        });
    }

    if cfg.rows.current {
        lines.push(resolved_row(&cfg.labels.current, &five, &style, now_ts));
    }
    if cfg.rows.weekly {
        lines.push(resolved_row(&cfg.labels.weekly, &seven, &style, now_ts));
    }

    if !lines.is_empty() {
        let _ = writeln!(std::io::stdout(), "{}", lines.join("\n"));
    }
}
