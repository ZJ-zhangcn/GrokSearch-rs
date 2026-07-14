//! Local clock injection for time-sensitive search queries.
//! Port of GuDaStudio/GrokSearch `get_local_time_info` / `_needs_time_context`.

use chrono::{Datelike, Local, Timelike};

/// Chinese weekday labels (Monday = 0), matching the Python fork.
const WEEKDAYS_CN: [&str; 7] = [
    "星期一",
    "星期二",
    "星期三",
    "星期四",
    "星期五",
    "星期六",
    "星期日",
];

/// Compact local-time block prepended to user queries that need temporal context.
pub fn local_time_context() -> String {
    let now = Local::now();
    let weekday = WEEKDAYS_CN[now.weekday().num_days_from_monday() as usize];
    let tz = now.format("%Z").to_string();
    let tz = if tz.is_empty() {
        "Local".to_string()
    } else {
        tz
    };
    format!(
        "[Current Time Context]\n- Date: {} ({})\n- Time: {:02}:{:02}:{:02}\n- Timezone: {}\n",
        now.format("%Y-%m-%d"),
        weekday,
        now.hour(),
        now.minute(),
        now.second(),
        tz
    )
}

/// Whether `query` looks time-relative (今天 / today / 最新 …).
pub fn needs_time_context(query: &str) -> bool {
    const CN: &[&str] = &[
        "当前",
        "现在",
        "今天",
        "明天",
        "昨天",
        "本周",
        "上周",
        "下周",
        "这周",
        "本月",
        "上月",
        "下月",
        "这个月",
        "今年",
        "去年",
        "明年",
        "最新",
        "最近",
        "近期",
        "刚刚",
        "刚才",
        "实时",
        "即时",
        "目前",
        "今日",
        "今夜",
        "今晚",
        "今早",
        "午前",
        "午后",
    ];
    const EN: &[&str] = &[
        "current",
        "now",
        "today",
        "tomorrow",
        "yesterday",
        "this week",
        "last week",
        "next week",
        "this month",
        "last month",
        "next month",
        "this year",
        "last year",
        "next year",
        "latest",
        "recent",
        "recently",
        "just now",
        "real-time",
        "realtime",
        "up-to-date",
    ];

    for kw in CN {
        if query.contains(kw) {
            return true;
        }
    }
    let lower = query.to_ascii_lowercase();
    for kw in EN {
        if lower.contains(kw) {
            return true;
        }
    }
    false
}

/// Prepend time context when the query is time-sensitive; otherwise return `query` unchanged.
pub fn maybe_inject_time_context(query: &str) -> String {
    if needs_time_context(query) {
        format!("{}{}", local_time_context(), query)
    } else {
        query.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_chinese_today() {
        assert!(needs_time_context("上海浦东今天天气"));
        assert!(needs_time_context("最新消息"));
        assert!(!needs_time_context("什么是 HTTP"));
    }

    #[test]
    fn detects_english_today() {
        assert!(needs_time_context("weather today in Shanghai"));
        assert!(!needs_time_context("history of TCP"));
    }

    #[test]
    fn inject_prefixes_when_needed() {
        let out = maybe_inject_time_context("浦东今天天气");
        assert!(out.starts_with("[Current Time Context]"));
        assert!(out.contains("浦东今天天气"));
        assert_eq!(maybe_inject_time_context("什么是 Rust"), "什么是 Rust");
    }
}
