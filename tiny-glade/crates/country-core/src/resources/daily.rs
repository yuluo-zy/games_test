//! B 版每日/每周主题的权威状态（原文件 `resources/daily.rs`）。
use super::session::SessionMeta;
use bevy_ecs::resource::Resource;
use serde::Deserialize;
use std::{
    path::PathBuf,
    time::{Duration, SystemTime},
};

/// `credits.json` 的三个可选字段；缺字段、null 与未知字段遵循原 serde 合同。
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
pub struct DailyThemeCredits {
    pub author: Option<String>,
    pub inspiration: Option<String>,
    pub theme_idea: Option<String>,
}

/// 语言表与元数据已解析、历史路径存在的活动；历史内容由会话加载器解析。
#[derive(Clone, Debug)]
pub struct DailyThemeInfo {
    pub languages: Vec<String>,
    pub history_path: PathBuf,
    pub meta: SessionMeta,
    pub credits: Option<DailyThemeCredits>,
    pub start_time: SystemTime,
    pub end_time: SystemTime,
}
impl DailyThemeInfo {
    /// 0x14083c240：未知 locale 取英语；已知 locale 缺失时不再回退英语。
    pub fn get_name(&self, locale: &str) -> &str {
        const LANGUAGES: [&str; 14] = [
            "en-US", "de-DE", "es-ES", "fr-FR", "it-IT", "ja-JP", "ko-KR", "pl-PL", "pt-BR",
            "ru-RU", "sv-SE", "tr-TR", "uk-UA", "zh-CN",
        ];
        let index = LANGUAGES
            .iter()
            .position(|&language| language == locale)
            .unwrap_or(0);
        self.languages
            .get(index)
            .map(String::as_str)
            .unwrap_or("Error: No event localization found")
    }
    /// 0x14083c440：原阈值为五天，逆序时间区间不会成为每周主题。
    pub fn is_weekly(&self) -> bool {
        self.end_time
            .duration_since(self.start_time)
            .is_ok_and(|duration| duration >= Duration::from_secs(432_000))
    }
}

/// 原外层成功值和三种错误/等待分支；不能合并“下载中”和“无活动”。
#[derive(Default, Debug)]
pub enum DailyThemeState {
    #[default]
    Downloading,
    NoCurrentTheme,
    DownloadError(String),
    Ready(DailyThemeInfo),
}
#[derive(Resource, Default, Debug)]
pub struct CurrentDaily(pub DailyThemeState);

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::UNIX_EPOCH;
    #[test]
    fn full_native_name_fixture_matches_all_locale_and_length_cases() {
        let rows: Vec<(String, usize, String)> = ron::from_str(include_str!(
            "../../../../tests/fixtures/native/toolbar-daily-name.ron"
        ))
        .unwrap();
        assert_eq!(rows.len(), 256);
        for (locale, length, expected) in rows {
            let info = DailyThemeInfo {
                languages: (0..length).map(|index| format!("Theme{index}")).collect(),
                history_path: PathBuf::new(),
                meta: SessionMeta::default(),
                credits: None,
                start_time: UNIX_EPOCH,
                end_time: UNIX_EPOCH,
            };
            assert_eq!(
                info.get_name(&locale),
                expected,
                "locale={locale} len={length}"
            );
        }
    }
    #[test]
    fn names_follow_native_locale_positions_and_missing_behavior() {
        let mut info = DailyThemeInfo {
            languages: (0..14).map(|index| index.to_string()).collect(),
            history_path: PathBuf::new(),
            meta: SessionMeta::default(),
            credits: None,
            start_time: UNIX_EPOCH,
            end_time: UNIX_EPOCH,
        };
        for (locale, index) in [
            ("en-US", 0),
            ("de-DE", 1),
            ("ja-JP", 5),
            ("ko-KR", 6),
            ("pt-BR", 8),
            ("zh-CN", 13),
            ("no-NO", 0),
            ("unknown", 0),
        ] {
            assert_eq!(info.get_name(locale), index.to_string());
        }
        info.languages.truncate(1);
        assert_eq!(info.get_name("zh-CN"), "Error: No event localization found");
        assert_eq!(info.get_name("no-NO"), "0");
        info.languages.clear();
        assert_eq!(info.get_name("en-US"), "Error: No event localization found");
    }
    #[test]
    fn full_native_weekly_fixture_preserves_filetime_boundaries() {
        let rows: Vec<(u64, u64, bool)> = ron::from_str(include_str!(
            "../../../../tests/fixtures/native/toolbar-daily-weekly.ron"
        ))
        .unwrap();
        assert_eq!(rows.len(), 29);
        // Windows 原 duration_since 用有符号时间值比较；raw 高位不能当作
        // macOS 正向 Unix 时间。epoch 平移不影响这里只比较的时间间隔。
        let time = |bits: u64| {
            let ticks = bits as i64;
            let absolute = ticks.unsigned_abs();
            let duration = Duration::new(
                absolute / 10_000_000,
                ((absolute % 10_000_000) * 100) as u32,
            );
            if ticks < 0 {
                UNIX_EPOCH - duration
            } else {
                UNIX_EPOCH + duration
            }
        };
        for (start, end, expected) in rows {
            let info = DailyThemeInfo {
                languages: vec![],
                history_path: PathBuf::new(),
                meta: SessionMeta::default(),
                credits: None,
                start_time: time(start),
                end_time: time(end),
            };
            assert_eq!(info.is_weekly(), expected, "start={start} end={end}");
        }
    }
    #[test]
    fn weekly_boundary_and_credits_schema_match_native() {
        let mut info = DailyThemeInfo {
            languages: vec![],
            history_path: PathBuf::new(),
            meta: SessionMeta::default(),
            credits: None,
            start_time: UNIX_EPOCH,
            end_time: UNIX_EPOCH + Duration::from_secs(431_999),
        };
        assert!(!info.is_weekly());
        info.end_time += Duration::from_secs(1);
        assert!(info.is_weekly());
        info.start_time = info.end_time + Duration::from_secs(1);
        assert!(!info.is_weekly());
        assert_eq!(
            serde_json::from_str::<DailyThemeCredits>(r#"{"author":null,"extra":1}"#).unwrap(),
            DailyThemeCredits::default()
        );
        let credits: DailyThemeCredits =
            serde_json::from_str(r#"{"author":"a","inspiration":"b","theme_idea":"c"}"#).unwrap();
        assert_eq!(credits.author.as_deref(), Some("a"));
        assert_eq!(credits.inspiration.as_deref(), Some("b"));
        assert_eq!(credits.theme_idea.as_deref(), Some("c"));
        assert!(
            serde_json::from_str::<DailyThemeCredits>(r#"{"author":"a","author":"b"}"#).is_err()
        );
        assert!(serde_json::from_str::<DailyThemeCredits>(r#"{"theme_idea":1}"#).is_err());
    }
}
