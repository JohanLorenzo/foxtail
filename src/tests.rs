use super::*;
use chrono::TimeZone;
use std::collections::HashMap;

#[test]
fn test_abbreviate_path_simple() {
    assert_eq!(abbreviate_path("/home/user/project"), "/h/u/project");
}

#[test]
fn test_hsl_percent_parsing() {
    let spec = ColorSpec::Hsl("hsl(30, 100%, 50%)".to_string());
    let rgb = parse_color(&spec).unwrap();
    assert_eq!(rgb.0, rgb.0); // basic sanity
}

#[test]
fn test_warning_message_customization_plain() {
    let d = StatusInput {
        cwd: "/tmp".to_string(),
        model: Model {
            display_name: "M".to_string(),
        },
        workspace: Workspace {
            current_dir: "/tmp".to_string(),
        },
        cost: Cost {
            total_cost_usd: 0.0,
            total_duration_ms: 30_000,
            total_lines_added: 0,
            total_lines_removed: 0,
        },
        context_window: ContextWindow {
            context_window_size: 200_000,
            used_percentage: 42.0,
            remaining_percentage: 58.0,
            current_usage: None,
        },
        rate_limits: None,
    };
    let cfg = Config {
        warning_message: Some("warn {used_pct}% {used_k}k/{total_k}k".to_string()),
        ..Default::default()
    };
    let out = render_warning(&d, &cfg, (0, 0, 0), false, false);
    assert!(out.contains("warn 42.0% 84k/200k"));
}

#[test]
fn test_abbreviate_path_hidden_dirs() {
    assert_eq!(abbreviate_path("/home/user/.config/app"), "/h/u/.c/app");
    assert_eq!(abbreviate_path("/home/.local/.cache/data"), "/h/.l/.c/data");
}

#[test]
fn test_abbreviate_path_single_component() {
    assert_eq!(abbreviate_path("/project"), "/project");
    assert_eq!(abbreviate_path("project"), "project");
}

#[test]
fn test_abbreviate_path_root() {
    assert_eq!(abbreviate_path("/"), "/");
}

#[test]
fn test_format_duration_ms_seconds() {
    assert_eq!(format_duration_ms(5000), "5s");
    assert_eq!(format_duration_ms(45000), "45s");
}

#[test]
fn test_format_duration_ms_minutes() {
    assert_eq!(format_duration_ms(60000), "1m");
    assert_eq!(format_duration_ms(125000), "2m");
    assert_eq!(format_duration_ms(3540000), "59m");
}

#[test]
fn test_format_duration_ms_hours() {
    assert_eq!(format_duration_ms(3600000), "1h0m");
    assert_eq!(format_duration_ms(5400000), "1h30m");
    assert_eq!(format_duration_ms(7200000), "2h0m");
}

#[test]
fn test_format_duration_ms_days() {
    assert_eq!(format_duration_ms(86400000), "1d0h");
    assert_eq!(format_duration_ms(90000000), "1d1h");
}

#[test]
fn test_get_symbol_with_emojis() {
    let config = Config {
        enable_git: true,
        use_emojis: true,
        warn_high_initial_context_threshold: 20.0,
        symbols: HashMap::new(),
        enable_background: false,
        threshold_green: 100,
        threshold_yellow: 500,
        threshold_orange: 1000,
        colors: None,
        visual: None,
        format: default_format(),
        light_background: None,
        warning_message: None,
        limit5h_reset: default_limit5h_reset(),
        limit7d_reset: default_limit7d_reset(),
    };

    assert_eq!(get_symbol("cwd", &config), "📁 ");
    assert_eq!(get_symbol("time", &config), "⏱ ");
    assert_eq!(get_symbol("context", &config), "🧠 ");
    assert_eq!(get_symbol("git_warning", &config), " ⚠");
    assert_eq!(get_symbol("git_clean", &config), "✓ ");
    assert_eq!(get_symbol("delta", &config), "Σ ");
    assert_eq!(get_symbol("cost", &config), "💰 $");
    assert_eq!(get_symbol("cache", &config), "🗄 ");
}

#[test]
fn test_get_symbol_without_emojis() {
    let config = Config::default();

    assert_eq!(get_symbol("cwd", &config), "cwd:");
    assert_eq!(get_symbol("time", &config), "");
    assert_eq!(get_symbol("context", &config), "ctx:");
    assert_eq!(get_symbol("git_warning", &config), "");
    assert_eq!(get_symbol("git_clean", &config), "");
    assert_eq!(get_symbol("delta", &config), "Δ ");
    assert_eq!(get_symbol("cost", &config), "$");
    assert_eq!(get_symbol("cache", &config), "cache:");
}

#[test]
fn test_get_symbol_with_custom_overrides() {
    let mut symbols = HashMap::new();
    symbols.insert("cwd".to_string(), "DIR:".to_string());
    symbols.insert("cost".to_string(), "¢".to_string());

    let config = Config {
        enable_git: true,
        use_emojis: true,
        warn_high_initial_context_threshold: 20.0,
        symbols,
        enable_background: false,
        threshold_green: 100,
        threshold_yellow: 500,
        threshold_orange: 1000,
        colors: None,
        visual: None,
        format: default_format(),
        light_background: None,
        warning_message: None,
        limit5h_reset: default_limit5h_reset(),
        limit7d_reset: default_limit7d_reset(),
    };

    assert_eq!(get_symbol("cwd", &config), "DIR:");
    assert_eq!(get_symbol("cost", &config), "¢");

    assert_eq!(get_symbol("time", &config), "⏱ ");
    assert_eq!(get_symbol("context", &config), "🧠 ");
}

#[test]
fn test_get_symbol_unknown_key() {
    let config = Config::default();
    assert_eq!(get_symbol("unknown", &config), "");
}

#[test]
fn test_commit_warning_color_green() {
    let colors = Colors::default();
    let bg_rgb = parse_color(&colors.background).unwrap();
    let color = commit_warning_color(10, bg_rgb, false);
    assert!(format!("{:?}", color).contains("Rgb"));
}

#[test]
fn test_commit_warning_color_yellow() {
    let colors = Colors::default();
    let bg_rgb = parse_color(&colors.background).unwrap();
    let color = commit_warning_color(150, bg_rgb, false);
    assert!(format!("{:?}", color).contains("Rgb"));
}

#[test]
fn test_commit_warning_color_orange() {
    let colors = Colors::default();
    let bg_rgb = parse_color(&colors.background).unwrap();
    let color = commit_warning_color(600, bg_rgb, false);
    assert!(format!("{:?}", color).contains("Rgb"));
}

#[test]
fn test_commit_warning_color_red() {
    let colors = Colors::default();
    let bg_rgb = parse_color(&colors.background).unwrap();
    let color = commit_warning_color(9001, bg_rgb, false);
    let debug_str = format!("{:?}", color);
    assert!(debug_str.contains("Rgb"));
}

#[test]
fn test_config_default() {
    let config = Config::default();
    assert!(config.enable_git);
    assert!(!config.use_emojis);
    assert_eq!(config.warn_high_initial_context_threshold, 20.0);
    assert!(config.symbols.is_empty());
}

#[test]
fn test_cost_log_path() {
    use crate::cost_log::log_path;

    let v: serde_json::Value =
        serde_json::from_str(r#"{"session_id":"abc","transcript_path":"/p/proj/abc.jsonl"}"#)
            .unwrap();
    assert_eq!(
        log_path(&v).unwrap().to_str().unwrap(),
        "/p/proj/abc.cost.jsonl"
    );

    let v: serde_json::Value = serde_json::from_str(r#"{"session_id":"abc"}"#).unwrap();
    assert!(log_path(&v).is_none());
}

#[test]
fn test_cost_log_line() {
    use crate::cost_log::log_line;

    let v: serde_json::Value = serde_json::from_str(
        r#"{"cost":{"total_cost_usd":0.123},"context_window":{"used_percentage":1.0}}"#,
    )
    .unwrap();
    let line = log_line(&v, "2026-09-13T09:20:45.521Z").unwrap();
    assert_eq!(line, "{\"t\":\"2026-09-13T09:20:45.521Z\",\"c\":0.123}\n");
    // Only the two fields of interest are kept.
    let parsed: serde_json::Value = serde_json::from_str(&line).unwrap();
    assert_eq!(parsed.as_object().unwrap().len(), 2);

    // An integral cost must still serialize as valid JSON.
    let v: serde_json::Value = serde_json::from_str(r#"{"cost":{"total_cost_usd":2.0}}"#).unwrap();
    assert_eq!(log_line(&v, "T").unwrap(), "{\"t\":\"T\",\"c\":2}\n");

    // No cost means nothing to log.
    let v: serde_json::Value = serde_json::from_str(r#"{"session_id":"abc"}"#).unwrap();
    assert!(log_line(&v, "T").is_none());
}

fn input_with_limits(rate_limits: Option<RateLimits>) -> StatusInput {
    StatusInput {
        cwd: "/tmp".to_string(),
        model: Model {
            display_name: "M".to_string(),
        },
        workspace: Workspace {
            current_dir: "/tmp".to_string(),
        },
        cost: Cost {
            total_cost_usd: 0.0,
            total_duration_ms: 120_000,
            total_lines_added: 0,
            total_lines_removed: 0,
        },
        context_window: ContextWindow {
            context_window_size: 200_000,
            used_percentage: 10.0,
            remaining_percentage: 90.0,
            current_usage: None,
        },
        rate_limits,
    }
}

fn render_limits_plain(d: &StatusInput, cfg: &Config) -> String {
    render_formatted(
        "{limit5h} | {limit7d}",
        d,
        cfg,
        &Colors::default(),
        (0, 0, 0),
        false,
        false,
    )
}

fn now_secs() -> i64 {
    chrono::Utc::now().timestamp()
}

#[test]
fn test_rate_limits_optional_in_payload() {
    let base = r#"{"cwd":"/","model":{"display_name":"M"},"workspace":{"current_dir":"/"},
        "cost":{"total_cost_usd":0.0,"total_duration_ms":1,"total_lines_added":0,"total_lines_removed":0},
        "context_window":{"context_window_size":200000,"used_percentage":1.0,"remaining_percentage":99.0}"#;
    let without: StatusInput = serde_json::from_str(&format!("{}}}", base)).unwrap();
    assert!(without.rate_limits.is_none());

    let one_window = format!(
        r#"{},"rate_limits":{{"five_hour":{{"used_percentage":23.5,"resets_at":1738425600}}}}}}"#,
        base
    );
    let one: StatusInput = serde_json::from_str(&one_window).unwrap();
    let limits = one.rate_limits.unwrap();
    assert_eq!(limits.five_hour.unwrap().resets_at, 1738425600);
    assert!(limits.seven_day.is_none());
}

fn countdown_cfg() -> Config {
    Config {
        limit5h_reset: "countdown".to_string(),
        limit7d_reset: "countdown".to_string(),
        ..Default::default()
    }
}

#[test]
fn test_limits_both_windows() {
    let now = now_secs();
    let d = input_with_limits(Some(RateLimits {
        five_hour: Some(RateLimitWindow {
            used_percentage: 23.0,
            resets_at: now + 2 * 3600 + 10 * 60 + 30,
        }),
        seven_day: Some(RateLimitWindow {
            used_percentage: 41.0,
            resets_at: now + 3 * 86400 + 30,
        }),
    }));
    assert_eq!(
        render_limits_plain(&d, &countdown_cfg()),
        "5h:23% (2h10m) | 7d:41% (3d0h)"
    );
}

#[test]
fn test_limits_one_window() {
    let d = input_with_limits(Some(RateLimits {
        five_hour: None,
        seven_day: Some(RateLimitWindow {
            used_percentage: 41.0,
            resets_at: now_secs() + 3 * 86400 + 30,
        }),
    }));
    assert_eq!(render_limits_plain(&d, &countdown_cfg()), "7d:41% (3d0h)");
}

#[test]
fn test_limits_absent() {
    let d = input_with_limits(None);
    assert_eq!(render_limits_plain(&d, &Config::default()), "");
}

#[test]
fn test_limits_expired_window_dropped() {
    let now = now_secs();
    let d = input_with_limits(Some(RateLimits {
        five_hour: Some(RateLimitWindow {
            used_percentage: 99.0,
            resets_at: now - 60,
        }),
        seven_day: Some(RateLimitWindow {
            used_percentage: 41.0,
            resets_at: now + 3 * 86400 + 30,
        }),
    }));
    assert_eq!(render_limits_plain(&d, &countdown_cfg()), "7d:41% (3d0h)");
}

#[test]
fn test_limits_percentage_rounded() {
    let d = input_with_limits(Some(RateLimits {
        five_hour: Some(RateLimitWindow {
            used_percentage: 23.5,
            resets_at: now_secs() + 600,
        }),
        seven_day: None,
    }));
    assert!(render_limits_plain(&d, &countdown_cfg()).starts_with("5h:24% ("));
}

#[test]
fn test_limits_percentage_clamped() {
    let d = input_with_limits(Some(RateLimits {
        five_hour: Some(RateLimitWindow {
            used_percentage: 130.0,
            resets_at: now_secs() + 600,
        }),
        seven_day: None,
    }));
    assert!(render_limits_plain(&d, &countdown_cfg()).starts_with("5h:100% ("));
}

#[test]
fn test_limits_emoji_and_custom_symbols() {
    let window = || RateLimitWindow {
        used_percentage: 23.0,
        resets_at: now_secs() + 2 * 3600 + 10 * 60 + 30,
    };
    let d = input_with_limits(Some(RateLimits {
        five_hour: Some(window()),
        seven_day: None,
    }));
    let emoji_cfg = Config {
        use_emojis: true,
        ..countdown_cfg()
    };
    assert_eq!(render_limits_plain(&d, &emoji_cfg), "🕔 23% (2h10m)");

    let mut symbols = HashMap::new();
    symbols.insert("limit5h".to_string(), "5H=".to_string());
    let custom_cfg = Config {
        symbols,
        ..countdown_cfg()
    };
    assert_eq!(render_limits_plain(&d, &custom_cfg), "5H=23% (2h10m)");
}

#[test]
fn test_limits_colored_output_uses_gradient() {
    let d = input_with_limits(Some(RateLimits {
        five_hour: Some(RateLimitWindow {
            used_percentage: 95.0,
            resets_at: now_secs() + 600,
        }),
        seven_day: None,
    }));
    let out = render_formatted(
        "{limit5h}",
        &d,
        &Config::default(),
        &Colors::default(),
        (0, 0, 0),
        false,
        true,
    );
    assert!(out.contains("\u{1b}["));
    assert!(out.contains("5h:95%"));
}

fn utc_ts(y: i32, mo: u32, d: u32, h: u32, mi: u32, s: u32) -> i64 {
    chrono::Utc
        .with_ymd_and_hms(y, mo, d, h, mi, s)
        .unwrap()
        .timestamp()
}

fn plus2() -> chrono::FixedOffset {
    chrono::FixedOffset::east_opt(2 * 3600).unwrap()
}

fn limit_at(resets_at: i64) -> RateLimitWindow {
    RateLimitWindow {
        used_percentage: 23.0,
        resets_at,
    }
}

fn limit_text(style: ResetStyle, sym: &str, now: i64, resets_at: i64) -> String {
    let tz = plus2();
    let display = LimitDisplay {
        sym: sym.to_string(),
        marker: "↻",
        style,
        tz: &tz,
    };
    render_limit(
        Some(&limit_at(resets_at)),
        &display,
        now,
        None,
        (0, 0, 0),
        false,
        false,
    )
}

#[test]
fn test_limit_symbols_distinct_icons_without_labels() {
    let emoji = Config {
        use_emojis: true,
        ..Default::default()
    };
    assert_eq!(get_symbol("limit5h", &emoji), "🕔 ");
    assert_eq!(get_symbol("limit7d", &emoji), "📅 ");
    assert_eq!(get_symbol("limit5h", &Config::default()), "5h:");
    assert_eq!(get_symbol("limit7d", &Config::default()), "7d:");
}

#[test]
fn test_limit_reset_marker_symbol_and_override() {
    let emoji = Config {
        use_emojis: true,
        ..Default::default()
    };
    assert_eq!(get_symbol("limit_reset", &emoji), "↻");
    assert_eq!(get_symbol("limit_reset", &Config::default()), "↻");
    let mut symbols = HashMap::new();
    symbols.insert("limit_reset".to_string(), "->".to_string());
    let custom = Config {
        symbols,
        ..Default::default()
    };
    assert_eq!(get_symbol("limit_reset", &custom), "->");
}

#[test]
fn test_limit_reset_style_defaults_and_parsing() {
    let cfg = Config::default();
    assert_eq!(cfg.reset_style("limit5h"), ResetStyle::Clock);
    assert_eq!(cfg.reset_style("limit7d"), ResetStyle::None);

    let parsed: Config =
        toml::from_str("limit5h_reset = \"weekday\"\nlimit7d_reset = \"countdown\"").unwrap();
    assert_eq!(parsed.reset_style("limit5h"), ResetStyle::Weekday);
    assert_eq!(parsed.reset_style("limit7d"), ResetStyle::Countdown);
}

#[test]
fn test_limit_reset_style_invalid_falls_back_to_default() {
    let cfg = Config {
        limit5h_reset: "bogus".to_string(),
        limit7d_reset: "".to_string(),
        ..Default::default()
    };
    assert_eq!(cfg.reset_style("limit5h"), ResetStyle::Clock);
    assert_eq!(cfg.reset_style("limit7d"), ResetStyle::None);
}

#[test]
fn test_limit_clock_same_local_day() {
    let now = utc_ts(2026, 10, 17, 12, 0, 0);
    let reset = utc_ts(2026, 10, 17, 14, 10, 30);
    assert_eq!(
        limit_text(ResetStyle::Clock, "5h:", now, reset),
        "5h:23% \u{21bb}16:10"
    );
}

#[test]
fn test_limit_clock_other_day_prefixes_weekday() {
    let now = utc_ts(2026, 10, 17, 12, 0, 0);
    let reset = utc_ts(2026, 10, 20, 14, 30, 0);
    assert_eq!(
        limit_text(ResetStyle::Clock, "7d:", now, reset),
        "7d:23% \u{21bb}Tue 16:30"
    );
}

#[test]
fn test_limit_clock_uses_local_day_not_utc_day() {
    let now = utc_ts(2026, 10, 17, 21, 0, 0);
    let reset = utc_ts(2026, 10, 17, 22, 30, 0);
    assert_eq!(
        limit_text(ResetStyle::Clock, "5h:", now, reset),
        "5h:23% \u{21bb}Sun 00:30"
    );
}

#[test]
fn test_limit_weekday_other_day() {
    let now = utc_ts(2026, 10, 17, 12, 0, 0);
    let reset = utc_ts(2026, 10, 20, 14, 30, 0);
    assert_eq!(
        limit_text(ResetStyle::Weekday, "7d:", now, reset),
        "7d:23% \u{21bb}Tue"
    );
}

#[test]
fn test_limit_weekday_today_shows_clock_time() {
    let now = utc_ts(2026, 10, 17, 12, 0, 0);
    let reset = utc_ts(2026, 10, 17, 14, 10, 30);
    assert_eq!(
        limit_text(ResetStyle::Weekday, "5h:", now, reset),
        "5h:23% \u{21bb}16:10"
    );
}

#[test]
fn test_limit_none_shows_no_reset() {
    let now = utc_ts(2026, 10, 17, 12, 0, 0);
    let reset = utc_ts(2026, 10, 20, 14, 30, 0);
    assert_eq!(limit_text(ResetStyle::None, "📅 ", now, reset), "📅 23%");
}

#[test]
fn test_limit_countdown_keeps_parentheses() {
    let now = utc_ts(2026, 10, 17, 12, 0, 0);
    let reset = now + 2 * 3600 + 10 * 60 + 30;
    assert_eq!(
        limit_text(ResetStyle::Countdown, "5h:", now, reset),
        "5h:23% (2h10m)"
    );
}

#[test]
fn test_limit_expired_window_renders_nothing_for_any_style() {
    let now = utc_ts(2026, 10, 17, 12, 0, 0);
    assert_eq!(limit_text(ResetStyle::Clock, "5h:", now, now - 1), "");
}

#[test]
fn test_limit_custom_marker() {
    let tz = plus2();
    let display = LimitDisplay {
        sym: "5h:".to_string(),
        marker: "->",
        style: ResetStyle::Clock,
        tz: &tz,
    };
    let now = utc_ts(2026, 10, 17, 12, 0, 0);
    let out = render_limit(
        Some(&limit_at(utc_ts(2026, 10, 17, 14, 10, 0))),
        &display,
        now,
        None,
        (0, 0, 0),
        false,
        false,
    );
    assert_eq!(out, "5h:23% ->16:10");
}

#[test]
fn test_limits_default_config_clock_for_5h_nothing_for_7d() {
    let now = now_secs();
    let d = input_with_limits(Some(RateLimits {
        five_hour: Some(limit_at(now + 600)),
        seven_day: Some(RateLimitWindow {
            used_percentage: 60.0,
            resets_at: now + 3 * 86400,
        }),
    }));
    let out = render_limits_plain(&d, &Config::default());
    assert!(out.starts_with("5h:23% \u{21bb}"), "{out}");
    assert!(out.ends_with(" | 7d:60%"), "{out}");
}

#[test]
fn test_limits_custom_marker_via_symbols_in_render() {
    let mut symbols = HashMap::new();
    symbols.insert("limit_reset".to_string(), ">>".to_string());
    let cfg = Config {
        symbols,
        ..Default::default()
    };
    let d = input_with_limits(Some(RateLimits {
        five_hour: Some(limit_at(now_secs() + 600)),
        seven_day: None,
    }));
    assert!(render_limits_plain(&d, &cfg).starts_with("5h:23% >>"));
}
