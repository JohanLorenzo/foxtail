use chrono::{DateTime, TimeZone, Utc};
use std::fs;
use std::io;
use std::path::Path;

const TIMESTAMP_FORMAT: &str = "%Y-%m-%dT%H:%M:%SZ";

pub fn write_usage_snapshot(raw: &str, path: &Path) {
    let value: serde_json::Value = match serde_json::from_str(raw) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("foxtail: --usage-snapshot: invalid JSON: {}", e);
            return;
        }
    };

    let json = match snapshot_json(&value, Utc::now()) {
        Some(j) => j,
        None => return,
    };

    if let Err(e) = write_atomic(path, &json) {
        eprintln!("foxtail: --usage-snapshot: {}: {}", path.display(), e);
    }
}

pub(crate) fn snapshot_json(value: &serde_json::Value, now: DateTime<Utc>) -> Option<String> {
    let limits = value.get("rate_limits")?;
    let (five_pct, five_reset) = window_fields(limits.get("five_hour"));
    let (week_pct, week_reset) = window_fields(limits.get("seven_day"));
    Some(format!(
        "{{\"five_hour_pct\":{},\"five_hour_resets_at\":{},\"weekly_pct\":{},\"weekly_resets_at\":{},\"updated_at\":\"{}\"}}",
        five_pct,
        five_reset,
        week_pct,
        week_reset,
        now.format(TIMESTAMP_FORMAT)
    ))
}

fn window_fields(window: Option<&serde_json::Value>) -> (String, String) {
    let percentage = window
        .and_then(|w| w.get("used_percentage"))
        .and_then(|p| p.as_f64())
        .filter(|p| p.is_finite())
        .map_or("null".to_string(), |p| p.to_string());
    let resets_at = window
        .and_then(|w| w.get("resets_at"))
        .and_then(|r| r.as_i64())
        .and_then(|secs| Utc.timestamp_opt(secs, 0).single())
        .map_or("null".to_string(), |t| {
            format!("\"{}\"", t.format(TIMESTAMP_FORMAT))
        });
    (percentage, resets_at)
}

pub(crate) fn write_atomic(path: &Path, text: &str) -> io::Result<()> {
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    fs::create_dir_all(parent)?;
    let file_name = path
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "path has no file name"))?
        .to_string_lossy();
    let tmp = parent.join(format!("{}.{}.tmp", file_name, std::process::id()));
    if let Err(e) = fs::write(&tmp, text).and_then(|_| fs::rename(&tmp, path)) {
        let _ = fs::remove_file(&tmp);
        return Err(e);
    }
    Ok(())
}
