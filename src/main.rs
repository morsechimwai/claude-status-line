use ccstatus::{cache, config, input, render};
use chrono::{DateTime, Local, TimeZone};
use std::io::Read;

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
        return Resolved {
            pct: Some(clamp_pct(w.used_percentage.unwrap_or(0.0))),
            resets_at: w.resets_at,
        };
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
    live.map(|w| cache::CachedWindow {
        used_percentage: w.used_percentage.unwrap_or(0.0),
        resets_at: w.resets_at,
    })
}

fn resolved_row(
    label: &str,
    r: &Resolved,
    style: &render::Style,
    now: &DateTime<Local>,
) -> String {
    let dt = r.resets_at.and_then(|e| Local.timestamp_opt(e, 0).single());
    let value = render::fmt_reset(dt, now);
    render::row(label, r.pct, &value, style)
}

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().read_to_string(&mut buf);

    let inp = input::parse(&buf);
    let cfg = config::Config::load();
    let style = cfg.style();
    let now = Local::now();

    let mut lines: Vec<String> = Vec::new();

    if cfg.rows.context {
        let size = inp.context_window.context_window_size as u64;
        let used = (inp.context_window.total_input_tokens
            + inp.context_window.total_output_tokens) as u64;
        let pct = clamp_pct(inp.context_window.used_percentage);
        let value = format!("{}/{}", render::fmt_tokens(used), render::fmt_tokens(size));
        lines.push(render::row(&inp.model.display_name, Some(pct), &value, &style));
    }

    let cached = cache::load();
    let five = resolve_window(
        inp.rate_limits.five_hour.as_ref(),
        cached.as_ref().and_then(|c| c.five_hour.clone()),
        now.timestamp(),
    );
    let seven = resolve_window(
        inp.rate_limits.seven_day.as_ref(),
        cached.as_ref().and_then(|c| c.seven_day.clone()),
        now.timestamp(),
    );

    let live_present =
        inp.rate_limits.five_hour.is_some() || inp.rate_limits.seven_day.is_some();
    if live_present {
        // Merge live into the loaded cache: carry forward a window that has no
        // live data this run instead of nulling it.
        cache::store(&cache::CachedUsage {
            five_hour: window_to_cache(inp.rate_limits.five_hour.as_ref())
                .or_else(|| cached.as_ref().and_then(|c| c.five_hour.clone())),
            seven_day: window_to_cache(inp.rate_limits.seven_day.as_ref())
                .or_else(|| cached.as_ref().and_then(|c| c.seven_day.clone())),
        });
    }

    if cfg.rows.current {
        lines.push(resolved_row(&cfg.labels.current, &five, &style, &now));
    }
    if cfg.rows.weekly {
        lines.push(resolved_row(&cfg.labels.weekly, &seven, &style, &now));
    }

    if !lines.is_empty() {
        println!("{}", lines.join("\n"));
    }
}
