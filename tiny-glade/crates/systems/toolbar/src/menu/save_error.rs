//! 原B menu/save_error.rs：真实异步落盘错误、恢复提示与备份信息。
//! 原141ec0ce0及三个slot闭包见toolbar-repair-consumers-native.json；沿用当前存档线程owner。
use crate::mei::{
    core::{MeiContext, NodeContext},
    widgets::LabelContent,
};
use bevy_ecs::prelude::*;
use country_core::{
    resources::{gui::EscMenuState, session::ActiveSession},
    systems::save::{
        save_backup_list::SaveBackupList,
        save_thread::{AutoSaveError, SaveThreadHandle},
    },
};
use std::{sync::OnceLock, time::Instant};

pub fn ui(
    mut mei: ResMut<MeiContext>,
    mut worker: ResMut<SaveThreadHandle>,
    menu: Res<EscMenuState>,
    session: Res<ActiveSession>,
    mut last_failure: Local<Option<Instant>>,
) {
    // 当前重构把错误接收端放在SaveThreadHandle；此处读取同一真实owner，
    // 不把AutoSaveState的发送失败Option误当成磁盘失败列表。
    worker.error_state.drain_pending();
    build(
        &mut mei,
        worker.error_state.errors(),
        &menu,
        || session.id.backup_folder_path(),
        &mut last_failure,
        Instant::now(),
    );
}

fn build(
    mei: &mut MeiContext,
    errors: &[(std::path::PathBuf, AutoSaveError)],
    menu: &EscMenuState,
    backup_directory: impl FnOnce() -> std::path::PathBuf,
    last_failure: &mut Option<Instant>,
    now: Instant,
) {
    if let Some((path, error)) = errors.first() {
        *last_failure = Some(now);
        let (message, advice) = match error {
            AutoSaveError::StorageFull(error) => (
                format!("{}: {error}", lang_lib::lookup("SaveError.StorageFull")),
                "SaveError.Advice.FreeUpSpace",
            ),
            AutoSaveError::Io(error) => (error.clone(), "SaveError.Advice.Generic"),
        };
        let root = mei.safe_frame("save_error", false);
        root.ignore_input();
        let view = root.template("menu/save_error");
        view.with_slot("internal_error", |slot| {
            label(slot, message, 15., 0x7b0808ff, tg_text::Align::Center);
            label(
                slot,
                format!(
                    "{}: {}",
                    lang_lib::lookup("SaveError.AffectedFile"),
                    path.to_string_lossy()
                ),
                15.,
                0x7b0808ff,
                tg_text::Align::Center,
            );
        });
        view.with_slot("plan", |slot| {
            label(
                slot,
                format!(
                    "1. {}.\n2. {}.\n3. {}.\n4. {}.",
                    lang_lib::lookup("SaveError.DontPanic"),
                    lang_lib::lookup("SaveError.KeepTheGameRunning"),
                    lang_lib::lookup(advice),
                    lang_lib::lookup("SaveError.WillSave")
                ),
                19.,
                0x7b5a51ff,
                tg_text::Align::Left,
            );
        });
        view.with_slot("backups", |slot| {
            // 原每帧重新枚举，失败视为空迭代，不伪造备份或修改存档文件。
            let mut backups = SaveBackupList::iter_backup_files(&backup_directory())
                .map(|files| files.collect::<Vec<_>>())
                .unwrap_or_default();
            // 原闭包PDB内联std::time::SystemTime::cmp，比较完整时间而非截断整秒。
            backups.sort_by(|a, b| b.modified.cmp(&a.modified));
            let mut text = format!(
                "{}.",
                lang_lib::lookup("SaveError.GladeHasBackups")
                    .replace("{count}", &backups.len().to_string())
            );
            if let Some(backup) = backups.first() {
                static FORMAT: OnceLock<
                    Vec<time::format_description::BorrowedFormatItem<'static>>,
                > = OnceLock::new();
                let format = FORMAT.get_or_init(|| {
                    time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")
                        .expect("原备份日期格式")
                });
                let date = utils::local_time::system_time_to_local_time(backup.modified)
                    .format(format)
                    .expect("原备份日期格式化");
                text.push_str(&format!(
                    "\n{}: {date}",
                    lang_lib::lookup("SaveError.MostRecentBackup")
                ));
            }
            label(slot, text, 13., 0x7b5a51ff, tg_text::Align::Center);
        });
    } else if menu.is_closed() {
        if let Some(last) = *last_failure {
            let elapsed = now.saturating_duration_since(last);
            // 原先比较整秒>14，再以f32秒数/15做独立两次边界裁决。
            if elapsed.as_secs() < 15 {
                let root = mei.safe_frame("save_error_resolved", false);
                root.ignore_input();
                root.node_mut().local_opacity = 1. - (elapsed.as_secs_f32() / 15.).clamp(0., 1.);
                root.template("menu/save_error_resolved");
            }
        }
    }
}

fn label(node: NodeContext, text: String, size: f32, color: u32, align: tg_text::Align) {
    let label = node.label(text);
    let content = label.content_mut::<LabelContent>();
    content.color = color;
    content.text.set_align(align);
    content
        .text
        .set_family(tg_text::FontFamily::Hand)
        .expect("原保存错误字体");
    content.text.set_metrics(size).expect("原保存错误字号");
    content
        .text
        .set_max_width(450.)
        .expect("原保存错误文字最大宽度");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        path::{Path, PathBuf},
        sync::Arc,
        time::Duration,
    };
    fn texts(mei: &MeiContext) -> Vec<(String, u32)> {
        mei.nodes
            .values()
            .filter_map(|node| {
                let any: &dyn std::any::Any = node.content.as_ref();
                any.downcast_ref::<LabelContent>()
                    .and_then(|label| label.pending_text.clone().map(|text| (text, label.color)))
            })
            .collect()
    }
    #[test]
    fn healthy_frames_do_not_resolve_or_read_backup_paths() {
        let mut mei = MeiContext::default();
        let mut last = None;
        build(
            &mut mei,
            &[],
            &EscMenuState::Closed,
            || panic!("健康帧不应计算备份路径或读取磁盘"),
            &mut last,
            Instant::now(),
        );
        assert!(mei.nodes.is_empty());
    }

    fn real_mei() -> MeiContext {
        let install = PathBuf::from(std::env::var_os("TG_INSTALL_ROOT").unwrap());
        let mut mei = MeiContext::default();
        mei.load_font_table(&install.join("assets/fonts/table.ron"))
            .unwrap();
        for name in ["menu/save_error", "menu/save_error_resolved"] {
            mei.configs.insert(
                name.into(),
                Arc::new(
                    crate::mei::config::read_node_config_from_file(
                        &install.join(format!("assets/ui/{name}.ron")),
                    )
                    .unwrap(),
                ),
            );
        }
        mei
    }

    #[test]
    #[ignore = "需TG_INSTALL_ROOT原模板和字体表；不发送OS输入"]
    fn real_templates_follow_async_error_recovery_menu_gate_and_deadline() {
        let mut mei = real_mei();
        let (sender, mut errors) =
            country_core::systems::save::save_thread::AutosaveErrorState::channel();
        let now = Instant::now();
        let mut last = None;
        let absent = Path::new("/nonexistent-toolbar-review-backups");
        sender
            .send(vec![(
                PathBuf::from("history.json"),
                AutoSaveError::StorageFull("disk error".into()),
            )])
            .unwrap();
        errors.drain_pending();
        build(
            &mut mei,
            errors.errors(),
            &EscMenuState::Main,
            || absent.to_owned(),
            &mut last,
            now,
        );
        assert_eq!(last, Some(now));
        let labels = texts(&mei);
        assert!(
            labels
                .iter()
                .any(|(s, c)| s.contains("disk error") && *c == 0x7b0808ff)
        );
        assert!(labels.iter().any(|(s, _)| s.contains("history.json")));
        assert!(
            labels
                .iter()
                .any(|(s, _)| s.contains(&*lang_lib::lookup("SaveError.Advice.FreeUpSpace")))
        );
        assert!(labels.iter().any(|(s, _)| s
            == &format!(
                "{}.",
                lang_lib::lookup("SaveError.GladeHasBackups").replace("{count}", "0")
            )));
        // 持续失败每一帧重置时间，恢复提示从最后失败帧计时。
        mei.recycle_frame_nodes();
        build(
            &mut mei,
            errors.errors(),
            &EscMenuState::Main,
            || absent.to_owned(),
            &mut last,
            now + Duration::from_secs(10),
        );
        assert_eq!(last, Some(now + Duration::from_secs(10)));
        sender.send(Vec::new()).unwrap();
        errors.drain_pending();
        mei.recycle_frame_nodes();
        build(
            &mut mei,
            errors.errors(),
            &EscMenuState::Main,
            || absent.to_owned(),
            &mut last,
            now + Duration::from_secs(11),
        );
        assert!(mei.nodes.is_empty(), "原恢复提示在菜单打开时隐藏");
        mei.recycle_frame_nodes();
        build(
            &mut mei,
            errors.errors(),
            &EscMenuState::Closed,
            || absent.to_owned(),
            &mut last,
            now + Duration::from_millis(17500),
        );
        assert!(mei.nodes.values().any(|node| node.local_opacity == 0.5));
        assert!(!mei.nodes.is_empty());
        mei.recycle_frame_nodes();
        build(
            &mut mei,
            errors.errors(),
            &EscMenuState::Closed,
            || absent.to_owned(),
            &mut last,
            now + Duration::from_millis(24999),
        );
        assert!(!mei.nodes.is_empty(), "14.999秒仍属提示时段");
        mei.recycle_frame_nodes();
        build(
            &mut mei,
            errors.errors(),
            &EscMenuState::Closed,
            || absent.to_owned(),
            &mut last,
            now + Duration::from_secs(25),
        );
        assert!(mei.nodes.is_empty());
        sender
            .send(vec![(
                PathBuf::from("other.json"),
                AutoSaveError::Io("permission denied".into()),
            )])
            .unwrap();
        errors.drain_pending();
        build(
            &mut mei,
            errors.errors(),
            &EscMenuState::Closed,
            || absent.to_owned(),
            &mut last,
            now + Duration::from_secs(26),
        );
        assert_eq!(last, Some(now + Duration::from_secs(26)));
        assert!(
            texts(&mei)
                .iter()
                .any(|(s, _)| s.contains(&*lang_lib::lookup("SaveError.Advice.Generic")))
        );
    }

    #[test]
    #[ignore = "需TG_INSTALL_ROOT原模板和字体表；独立进程内临时账号，不发送OS输入"]
    fn actual_ecs_adapter_drains_latest_snapshot_renders_backups_and_recovers() {
        const CHILD: &str = "TG_SAVE_ERROR_ADAPTER_CHILD";
        if std::env::var_os(CHILD).is_none() {
            // paths是进程全局OnceLock：子进程隔离，不能改变其他测试/用户的账号目录。
            let result = std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "menu::save_error::tests::actual_ecs_adapter_drains_latest_snapshot_renders_backups_and_recovers", "--ignored", "--nocapture"])
                .env(CHILD, "1").output().unwrap();
            assert!(
                result.status.success(),
                "{}\n{}",
                String::from_utf8_lossy(&result.stdout),
                String::from_utf8_lossy(&result.stderr)
            );
            return;
        }
        let account = tempfile::tempdir().unwrap();
        let install = PathBuf::from(std::env::var_os("TG_INSTALL_ROOT").unwrap());
        paths::initialize_paths_with_installation(0, Some(account.path()), &install).unwrap();
        lang_lib::configure(install.join("assets/lang"), true).unwrap();
        lang_lib::load_locale("en-US").unwrap();
        let session = ActiveSession::new(
            country_core::resources::session::SessionId::new(),
            Default::default(),
        );
        let backup_path = session.id.backup_folder_path();
        std::fs::create_dir_all(&backup_path).unwrap();
        let old = std::time::SystemTime::UNIX_EPOCH + Duration::from_secs(1_704_067_200);
        let latest = old + Duration::from_secs(172_800);
        for (name, modified) in [
            ("history-auto-0.json", old),
            ("history-manual-1.bak", latest),
        ] {
            let file = std::fs::File::create(backup_path.join(name)).unwrap();
            file.set_modified(modified).unwrap();
        }
        std::fs::write(backup_path.join("ignored.txt"), b"not a backup").unwrap();
        std::fs::create_dir(backup_path.join("directory.json")).unwrap();
        let (sender, error_state) =
            country_core::systems::save::save_thread::AutosaveErrorState::channel();
        let worker = SaveThreadHandle::spawn();
        let SaveThreadHandle {
            join,
            command_sender,
            ..
        } = worker;
        let mut world = World::new();
        world.insert_resource(real_mei());
        world.insert_resource(session);
        world.insert_resource(EscMenuState::Closed);
        world.insert_resource(SaveThreadHandle {
            join,
            command_sender,
            error_state,
        });
        let mut schedule = Schedule::default();
        schedule.add_systems(ui);
        schedule.run(&mut world);
        assert!(
            world.resource::<MeiContext>().nodes.is_empty(),
            "从未失败时不提示恢复"
        );
        sender
            .send(vec![(
                PathBuf::from("old.json"),
                AutoSaveError::Io("superseded".into()),
            )])
            .unwrap();
        sender
            .send(vec![
                (
                    PathBuf::from("history.json"),
                    AutoSaveError::Io("write failed".into()),
                ),
                (
                    PathBuf::from("second.json"),
                    AutoSaveError::Io("only first error should display".into()),
                ),
            ])
            .unwrap();
        schedule.run(&mut world);
        let labels = texts(world.resource::<MeiContext>());
        assert!(labels.iter().any(|(text, _)| text == "write failed"));
        assert!(
            !labels
                .iter()
                .any(|(text, _)| text.contains("superseded") || text.contains("only first error"))
        );
        let latest = utils::local_time::system_time_to_local_time(latest);
        let expected_date = format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            latest.year(),
            latest.month() as u8,
            latest.day(),
            latest.hour(),
            latest.minute(),
            latest.second()
        );
        let expected = format!(
            "{}.\n{}: {}",
            lang_lib::lookup("SaveError.GladeHasBackups").replace("{count}", "2"),
            lang_lib::lookup("SaveError.MostRecentBackup"),
            expected_date
        );
        assert!(
            labels
                .iter()
                .any(|(text, color)| text == &expected && *color == 0x7b5a51ff),
            "备份必须按真实目录计数并显示最新本地时间: {labels:?}"
        );
        sender.send(Vec::new()).unwrap();
        world.resource_mut::<MeiContext>().recycle_frame_nodes();
        *world.resource_mut::<EscMenuState>() = EscMenuState::Main;
        schedule.run(&mut world);
        assert!(world.resource::<SaveThreadHandle>().error_state.is_ok());
        assert!(world.resource::<MeiContext>().nodes.is_empty());
        *world.resource_mut::<EscMenuState>() = EscMenuState::Closed;
        schedule.run(&mut world);
        let mei = world.resource::<MeiContext>();
        assert!(
            !mei.nodes.is_empty(),
            "关闭菜单后在同一Local生命周期显示恢复提示"
        );
        assert!(
            !texts(mei)
                .iter()
                .any(|(text, _)| text.contains("write failed"))
        );
        let SaveThreadHandle {
            join,
            command_sender,
            ..
        } = world.remove_resource::<SaveThreadHandle>().unwrap();
        drop(command_sender);
        join.join().unwrap();
        // 继续通过真正存档线程、同一接收端和同一ui Local跑完整失败/恢复链。
        // 不注入错误文字：OpenFile目标会话目录缺失，ShadowFile真实创建失败。
        world.insert_resource(SaveThreadHandle::spawn());
        let session_id = world.resource::<ActiveSession>().id;
        assert!(!session_id.save_folder_path().exists());
        use country_core::systems::save::save_game::{
            AsyncSaveCmd, GameSaveSections, SerializedJson,
        };
        let sections = GameSaveSections {
            values: ["27", "17", "0", "{}", "{}", "[]", "{}", "-1", "-1", "[]"]
                .map(|value| SerializedJson::from_raw_string(value.to_owned())),
        };
        world
            .resource::<SaveThreadHandle>()
            .command_sender
            .send(AsyncSaveCmd::OpenFile {
                session: session_id,
                sections,
                edits: Vec::new(),
            })
            .unwrap();
        *world.resource_mut::<EscMenuState>() = EscMenuState::Main;
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            world.resource_mut::<MeiContext>().recycle_frame_nodes();
            schedule.run(&mut world);
            if !world.resource::<SaveThreadHandle>().error_state.is_ok() {
                break;
            }
            assert!(Instant::now() < deadline, "真实存档线程未报告创建失败");
            std::thread::sleep(Duration::from_millis(10));
        }
        let error_text = texts(world.resource::<MeiContext>());
        assert!(
            error_text
                .iter()
                .any(|(text, _)| text.contains("Create temp file")),
            "页面须展示真实下层I/O上下文而非测试预设文字: {error_text:?}"
        );
        assert!(
            error_text
                .iter()
                .any(|(text, _)| text.contains("history.json"))
        );
        std::fs::create_dir_all(session_id.save_folder_path()).unwrap();
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            world.resource_mut::<MeiContext>().recycle_frame_nodes();
            schedule.run(&mut world);
            if world.resource::<SaveThreadHandle>().error_state.is_ok() {
                break;
            }
            assert!(Instant::now() < deadline, "真实重试成功后未发布恢复快照");
            std::thread::sleep(Duration::from_millis(10));
        }
        assert!(
            !std::fs::read(session_id.save_folder_path().join("history.json"))
                .unwrap()
                .is_empty()
        );
        assert!(
            world.resource::<MeiContext>().nodes.is_empty(),
            "菜单抢占恢复提示"
        );
        *world.resource_mut::<EscMenuState>() = EscMenuState::Closed;
        schedule.run(&mut world);
        assert!(!world.resource::<MeiContext>().nodes.is_empty());
        let SaveThreadHandle {
            join,
            command_sender,
            ..
        } = world.remove_resource::<SaveThreadHandle>().unwrap();
        drop(command_sender);
        join.join().unwrap();
    }
}
