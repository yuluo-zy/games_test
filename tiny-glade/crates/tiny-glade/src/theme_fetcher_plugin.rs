//! 原 `theme_fetcher_plugin`：复用当前 tg-theme 下载状态机，发布语言表、元数据与历史路径已校验的主题。
use anyhow::{Context, Result};
use bevy_ecs::prelude::{Res, ResMut, Resource};
use country_core::resources::daily::{
    CurrentDaily, DailyThemeCredits, DailyThemeInfo, DailyThemeState,
};
use country_core::resources::session::SessionMeta;
use parking_lot::Mutex;
use std::time::Duration;
use tg_theme::{ActiveEvent, ThemeState, ThemeSubscription};

/// 持有订阅即持有 worker 的取消责任；资源卸载时订阅 Drop 通知后台退出。
#[derive(Resource)]
pub struct ActiveEventReceiver(Option<Mutex<ThemeSubscription>>);

/// 0x1403d1750：语言表与元数据解析成功，且历史路径为文件才发布。
/// 原版不读取 daily_theme.json/thumbnail.png，也不凭空补 data_folder。
pub fn load_daily_theme(event: ActiveEvent) -> Result<DailyThemeInfo> {
    let language_bytes =
        std::fs::read(event.data_folder.join("langs.json")).context("Reading langs.json")?;
    let languages =
        serde_json::from_slice::<Vec<String>>(&language_bytes).context("Parsing langs.json")?;
    let meta = SessionMeta::try_load_from_path(event.data_folder.join("meta.json"))
        .context("Loading session meta")?;
    let history_path = event.data_folder.join("history.json");
    anyhow::ensure!(history_path.is_file(), "Missing history.json");
    // credits 是可选附属信息，读取或解析失败不会废弃有效活动。
    let credits = std::fs::read_to_string(event.data_folder.join("credits.json"))
        .ok()
        .and_then(|text| serde_json::from_str::<DailyThemeCredits>(&text).ok());
    Ok(DailyThemeInfo {
        languages,
        history_path,
        meta,
        credits,
        start_time: event.start_time,
        end_time: event.end_time,
    })
}

fn publish_state(state: ThemeState, current: &mut CurrentDaily) {
    // 新通知必须替换旧活动。Pending/Idle 不能保留已失效的旧主题。
    current.0 = match state {
        ThemeState::Idle => DailyThemeState::NoCurrentTheme,
        ThemeState::Pending => DailyThemeState::Downloading,
        ThemeState::Failed(error) => DailyThemeState::DownloadError(error),
        ThemeState::Active(event) => match load_daily_theme(event) {
            Ok(info) => DailyThemeState::Ready(info),
            Err(error) => DailyThemeState::DownloadError(format!("{error:?}")),
        },
    };
}

/// 每帧只做一次非阻塞接收；没有通知时不触碰资源的变更标记。
pub fn update_daily_theme(receiver: Res<ActiveEventReceiver>, mut current: ResMut<CurrentDaily>) {
    if let Some(subscription) = &receiver.0 {
        if let Some(state) = subscription.lock().recv_timeout(Duration::ZERO) {
            publish_state(state, &mut current);
        }
    }
}

/// 原注册使用 standard_app_root/cas-cache，初始化失败只记录错误并保留空活动。
pub fn build(app: &mut bevy_app::App) {
    let subscription = tg_theme::ThemeClient::new(tg_theme::Config::new(
        paths::standard_app_root_path().join("cas-cache"),
    ))
    .and_then(tg_theme::ThemeClient::spawn);
    let subscription = match subscription {
        Ok(subscription) => Some(Mutex::new(subscription)),
        Err(error) => {
            log::error!("Error initializing theme fetcher: {error:?}");
            None
        }
    };
    app.init_resource::<CurrentDaily>()
        .insert_resource(ActiveEventReceiver(subscription))
        .add_systems(bevy_app::Update, update_daily_theme);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{path::PathBuf, time::UNIX_EPOCH};

    struct ThemeFolder(PathBuf);
    impl ThemeFolder {
        fn new() -> Self {
            static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
            let folder = std::env::temp_dir().join(format!(
                "tiny-glade-daily-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            ));
            std::fs::create_dir_all(&folder).unwrap();
            Self(folder)
        }
        fn event(&self) -> ActiveEvent {
            ActiveEvent {
                data_folder: self.0.clone(),
                start_time: UNIX_EPOCH,
                end_time: UNIX_EPOCH + Duration::from_secs(432_000),
            }
        }
        fn valid(&self) {
            std::fs::write(self.0.join("langs.json"), r#"["A theme","Ein Thema"]"#).unwrap();
            SessionMeta::default()
                .write_to_path(self.0.join("meta.json"))
                .unwrap();
            std::fs::write(self.0.join("history.json"), b"{}").unwrap();
        }
    }
    impl Drop for ThemeFolder {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn actual_package_requires_language_meta_history_and_ignores_bad_optional_credits() {
        let folder = ThemeFolder::new();
        assert!(
            format!("{:?}", load_daily_theme(folder.event()).unwrap_err())
                .contains("Reading langs.json")
        );
        std::fs::write(
            folder.0.join("langs.json"),
            r#"{"title":"invented schema"}"#,
        )
        .unwrap();
        assert!(
            format!("{:?}", load_daily_theme(folder.event()).unwrap_err())
                .contains("Parsing langs.json")
        );
        std::fs::write(folder.0.join("langs.json"), r#"["Theme"]"#).unwrap();
        assert!(
            format!("{:?}", load_daily_theme(folder.event()).unwrap_err())
                .contains("Loading session meta")
        );
        SessionMeta::default()
            .write_to_path(folder.0.join("meta.json"))
            .unwrap();
        assert_eq!(
            load_daily_theme(folder.event()).unwrap_err().to_string(),
            "Missing history.json"
        );
        folder.valid();
        let valid = load_daily_theme(folder.event()).unwrap();
        assert_eq!(valid.get_name("de-DE"), "Ein Thema");
        assert!(valid.is_weekly());
        assert_eq!(valid.history_path, folder.0.join("history.json"));
        assert_eq!(valid.meta, SessionMeta::default());
        assert!(valid.credits.is_none());
        let mut image = country_core::resources::image_formation::ImageFormationConfig::default();
        image.params_18[1] = 0.7;
        let metadata = SessionMeta {
            glade: country_core::resources::session::GladeType::Winter,
            mutators: vec![country_core::resources::session::SessionMutator::Xmas2024],
            image: Some(image),
        };
        metadata.write_to_path(folder.0.join("meta.json")).unwrap();
        assert_eq!(load_daily_theme(folder.event()).unwrap().meta, metadata);
        std::fs::write(folder.0.join("credits.json"), "malformed").unwrap();
        assert!(load_daily_theme(folder.event()).unwrap().credits.is_none());
        std::fs::write(
            folder.0.join("credits.json"),
            r#"{"author":"Artist","inspiration":"Photo","theme_idea":"Garden"}"#,
        )
        .unwrap();
        let credits = load_daily_theme(folder.event()).unwrap().credits.unwrap();
        assert_eq!(credits.author.as_deref(), Some("Artist"));
        assert_eq!(credits.inspiration.as_deref(), Some("Photo"));
        assert_eq!(credits.theme_idea.as_deref(), Some("Garden"));
        std::fs::remove_file(folder.0.join("history.json")).unwrap();
        std::fs::create_dir(folder.0.join("history.json")).unwrap();
        assert!(load_daily_theme(folder.event()).is_err());
    }
    #[test]
    fn notifications_replace_stale_theme_and_recover_after_error() {
        let folder = ThemeFolder::new();
        folder.valid();
        let mut current = CurrentDaily::default();
        publish_state(ThemeState::Active(folder.event()), &mut current);
        assert!(matches!(current.0, DailyThemeState::Ready(_)));
        publish_state(ThemeState::Failed("download failed".into()), &mut current);
        assert!(
            matches!(&current.0, DailyThemeState::DownloadError(error) if error == "download failed")
        );
        publish_state(ThemeState::Pending, &mut current);
        assert!(matches!(current.0, DailyThemeState::Downloading));
        publish_state(ThemeState::Active(folder.event()), &mut current);
        assert!(matches!(current.0, DailyThemeState::Ready(_)));
        std::fs::remove_file(folder.0.join("langs.json")).unwrap();
        publish_state(ThemeState::Active(folder.event()), &mut current);
        assert!(matches!(current.0, DailyThemeState::DownloadError(_)));
        publish_state(ThemeState::Idle, &mut current);
        assert!(matches!(current.0, DailyThemeState::NoCurrentTheme));
    }
    #[test]
    fn actual_subscription_publishes_to_ecs_and_resource_drop_cancels_worker() {
        use bevy_ecs::system::RunSystemOnce;
        struct EmptySchedule(std::sync::mpsc::Sender<()>);
        impl Drop for EmptySchedule {
            fn drop(&mut self) {
                let _ = self.0.send(());
            }
        }
        impl tg_theme::http::Transport for EmptySchedule {
            fn get(&mut self, _: &str, _: Option<&str>) -> Result<tg_theme::http::Response> {
                let bytes = br#"{"events":[]}"#.to_vec();
                Ok(tg_theme::http::Response {
                    status: 200,
                    date: None,
                    age: None,
                    etag: None,
                    cache_control: Some("max-age=100".into()),
                    content_length: Some(bytes.len() as u64),
                    body: Box::new(std::io::Cursor::new(bytes)),
                })
            }
        }
        let folder = ThemeFolder::new();
        let (finished, exit) = std::sync::mpsc::channel();
        let subscription = tg_theme::ThemeClient::with_transport(
            tg_theme::Config::new(&folder.0),
            EmptySchedule(finished),
        )
        .unwrap()
        .spawn()
        .unwrap();
        let mut world = bevy_ecs::world::World::new();
        // A distinct initial value ensures the worker's Idle notification is
        // actually received; the default is already NoCurrentTheme.
        world.insert_resource(CurrentDaily(DailyThemeState::DownloadError("fixture sentinel".into())));
        world.insert_resource(ActiveEventReceiver(Some(Mutex::new(subscription))));
        let deadline = std::time::Instant::now() + Duration::from_secs(2);
        loop {
            world.run_system_once(update_daily_theme).unwrap();
            if matches!(
                world.resource::<CurrentDaily>().0,
                DailyThemeState::NoCurrentTheme
            ) {
                break;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "主题 worker 未发布无活动状态"
            );
            std::thread::sleep(Duration::from_millis(1));
        }
        world.remove_resource::<ActiveEventReceiver>();
        exit.recv_timeout(Duration::from_secs(2))
            .expect("移除资源应取消并唤醒空闲 worker");
    }
    #[test]
    fn absent_notification_does_not_mark_resource_changed() {
        use bevy_ecs::{change_detection::DetectChanges, system::RunSystemOnce};
        let mut world = bevy_ecs::world::World::new();
        world.insert_resource(CurrentDaily::default());
        world.insert_resource(ActiveEventReceiver(None));
        world.clear_trackers();
        world.run_system_once(update_daily_theme).unwrap();
        assert!(
            !world
                .get_resource_ref::<CurrentDaily>()
                .unwrap()
                .is_changed()
        );
    }
}
