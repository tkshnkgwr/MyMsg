//! # CLI 時間・スケジュール計算サブモジュール (`cli/time.rs`)
//!
//! 単位付き時間（10s, 30m, 1h, 30分 等）、時刻指定（HH:MM / HH:MM:SS）、
//! 翌日繰り越しを含むスケジュール待機秒数の算出を提供します。

use chrono::{Local, NaiveTime, Timelike};

/// 単位付き時間（10s, 30m, 1h, 10秒, 30分, 1時間 等）または純粋な秒数を秒数に変換します。
pub fn parse_duration_to_seconds(input: &str) -> Option<u64> {
    let clean = input.trim().to_lowercase();
    if clean.is_empty() {
        return None;
    }
    if let Ok(secs) = clean.parse::<u64>() {
        return Some(secs);
    }
    let unit_multipliers = [
        ("s", 1u64),
        ("m", 60),
        ("h", 3600),
        ("秒", 1),
        ("分", 60),
        ("時間", 3600),
    ];
    for (suffix, mult) in unit_multipliers {
        let parsed = clean
            .strip_suffix(suffix)
            .and_then(|num_str| num_str.trim().parse::<u64>().ok());
        if let Some(num) = parsed {
            return Some(num * mult);
        }
    }
    None
}

/// インターバル指定文字列（30m, 1h, 300, 30分 など）を解析し、秒数を返します（1秒以上有効）。
pub fn parse_interval_to_seconds(input: &str) -> Option<u64> {
    parse_duration_to_seconds(input).filter(|&s| s > 0)
}

/// 時刻文字列（HH:MM または HH:MM:SS）を解析して NaiveTime を返します。
pub fn parse_time_str(input: &str) -> Option<NaiveTime> {
    let clean = input.trim();
    if !clean.contains(':') {
        return None;
    }
    let parts: Vec<&str> = clean.split(':').collect();
    if parts.len() == 2 || parts.len() == 3 {
        let h_opt = parts[0].trim().parse::<u32>().ok();
        let m_opt = parts[1].trim().parse::<u32>().ok();
        let s_opt = if parts.len() == 3 {
            parts[2].trim().parse::<u32>().ok()
        } else {
            Some(0)
        };

        match (h_opt, m_opt, s_opt) {
            (Some(h), Some(m), Some(s)) if h < 24 && m < 60 && s < 60 => {
                NaiveTime::from_hms_opt(h, m, s)
            }
            _ => None,
        }
    } else {
        None
    }
}

/// 指定時刻文字列（HH:MM または HH:MM:SS）を解析し、NaiveTime を返します。
/// カンマ等による複数時刻指定は許可されず、None を返します。
pub fn parse_at_time(input: &str) -> Option<NaiveTime> {
    let clean = input.trim();
    if clean.contains(',') {
        return None;
    }
    parse_time_str(clean)
}

/// 基準時刻（now）から指定時刻（target）までの待機秒数を算出します。
/// 本日の未来（1秒以上先）であればその差分秒数、過去であれば翌日同時刻までの秒数を返します。
pub fn calculate_at_wait(now: NaiveTime, target: NaiveTime) -> u64 {
    let now_secs = now.num_seconds_from_midnight() as i64;
    let target_secs = target.num_seconds_from_midnight() as i64;
    let diff = if target_secs > now_secs {
        target_secs - now_secs
    } else {
        (86400 + target_secs) - now_secs
    };
    diff as u64
}

/// 遅延指定文字列（秒数, 単位付き, または HH:MM / HH:MM:SS 時刻）を解析し、待機秒数を返します。
pub fn parse_delay_to_seconds(input: &str) -> u64 {
    let now = Local::now().time();
    parse_delay_with_reference(input, now)
}

/// 基準時刻（now）を用いて遅延秒数を算出する内部関数（テスト・検証用）
pub fn parse_delay_with_reference(input: &str, now: NaiveTime) -> u64 {
    let clean = input.trim();
    if clean.is_empty() || clean == "0" {
        return 0;
    }

    // 1. 単位付きまたは純粋な秒数
    if let Some(secs) = parse_duration_to_seconds(clean) {
        return clamp_delay_seconds(secs);
    }

    // 2. 時刻指定 (HH:MM または HH:MM:SS)
    if let Some(target_time) = parse_time_str(clean) {
        let now_secs = now.num_seconds_from_midnight() as i64;
        let target_secs = target_time.num_seconds_from_midnight() as i64;
        let diff = if target_secs >= now_secs {
            target_secs - now_secs
        } else {
            (86400 + target_secs) - now_secs
        };
        return clamp_delay_seconds(diff as u64);
    }

    0
}

/// 遅延秒数を安全な範囲（0〜86400秒 = 最大24時間）にクランプします。
pub fn clamp_delay_seconds(delay: u64) -> u64 {
    delay.min(86400)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp_delay_seconds() {
        assert_eq!(clamp_delay_seconds(0), 0);
        assert_eq!(clamp_delay_seconds(30), 30);
        assert_eq!(clamp_delay_seconds(86400), 86400);
        assert_eq!(clamp_delay_seconds(99999), 86400); // 24時間に制限
    }

    #[test]
    fn test_parse_delay_with_reference() {
        let now = NaiveTime::from_hms_opt(10, 50, 0).unwrap();

        // 1. 秒数指定
        assert_eq!(parse_delay_with_reference("0", now), 0);
        assert_eq!(parse_delay_with_reference("60", now), 60);
        assert_eq!(parse_delay_with_reference("300", now), 300);

        // 2. 単位指定
        assert_eq!(parse_delay_with_reference("10s", now), 10);
        assert_eq!(parse_delay_with_reference("5m", now), 300);
        assert_eq!(parse_delay_with_reference("2h", now), 7200);
        assert_eq!(parse_delay_with_reference("30秒", now), 30);
        assert_eq!(parse_delay_with_reference("10分", now), 600);
        assert_eq!(parse_delay_with_reference("1時間", now), 3600);

        // 3. 当日の後刻指定 (10:50 -> 11:00 = 10分 = 600秒)
        assert_eq!(parse_delay_with_reference("11:00", now), 600);
        // 秒付き指定 (10:50:00 -> 10:50:30 = 30秒)
        assert_eq!(parse_delay_with_reference("10:50:30", now), 30);

        // 4. 翌日の同時刻指定 (10:50 -> 10:00 = 23時間10分 = 83400秒)
        assert_eq!(parse_delay_with_reference("10:00", now), 83400);
    }

    #[test]
    fn test_parse_duration_and_interval() {
        assert_eq!(parse_duration_to_seconds("30"), Some(30));
        assert_eq!(parse_duration_to_seconds("10s"), Some(10));
        assert_eq!(parse_duration_to_seconds("30m"), Some(1800));
        assert_eq!(parse_duration_to_seconds("1h"), Some(3600));
        assert_eq!(parse_duration_to_seconds("10秒"), Some(10));
        assert_eq!(parse_duration_to_seconds("30分"), Some(1800));
        assert_eq!(parse_duration_to_seconds("1時間"), Some(3600));
        assert_eq!(parse_duration_to_seconds("invalid"), None);

        assert_eq!(parse_interval_to_seconds("30m"), Some(1800));
        assert_eq!(parse_interval_to_seconds("0"), None);
        assert_eq!(parse_interval_to_seconds("0s"), None);
    }

    #[test]
    fn test_parse_at_time() {
        assert_eq!(
            parse_at_time("15:30"),
            Some(NaiveTime::from_hms_opt(15, 30, 0).unwrap())
        );
        assert_eq!(
            parse_at_time("09:00:15"),
            Some(NaiveTime::from_hms_opt(9, 0, 15).unwrap())
        );
        // 前後空白トリム
        assert_eq!(
            parse_at_time("  15:30  "),
            Some(NaiveTime::from_hms_opt(15, 30, 0).unwrap())
        );
        // 複数時刻（カンマ区切り）は拒絶される
        assert_eq!(parse_at_time("09:00,12:00"), None);
        assert_eq!(parse_at_time("invalid"), None);
        assert_eq!(parse_at_time(""), None);
        assert_eq!(parse_at_time("25:00"), None); // 24時間を超える無効時刻
        assert_eq!(parse_at_time("12:60"), None); // 60分以上の無効時刻
    }

    #[test]
    fn test_calculate_at_wait() {
        let target = NaiveTime::from_hms_opt(12, 0, 0).unwrap();

        // 1. 午前10:00 -> 12:00 (2時間 = 7200秒)
        let now1 = NaiveTime::from_hms_opt(10, 0, 0).unwrap();
        assert_eq!(calculate_at_wait(now1, target), 7200);

        // 2. 午後14:00 -> 翌日12:00 (10時間 + 12時間 = 22時間 = 79200秒)
        let now2 = NaiveTime::from_hms_opt(14, 0, 0).unwrap();
        assert_eq!(calculate_at_wait(now2, target), 79200);

        // 3. ちょうど12:00 -> 翌日12:00 (24時間 = 86400秒)
        let now3 = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
        assert_eq!(calculate_at_wait(now3, target), 86400);
    }
}
