//! 原 B 新建菜单（141eb9690）与 resolve_new_glade；完整请求由 startup_session 消费。
use super::daily_welcome::ShowDailyWelcomeCmd;
use crate::mei::{
    core::{MeiContext, NodeContext},
    widgets::{ImageContent, LabelContent, MenuButtonContent},
};
use bevy_ecs::prelude::*;
use bevy_state::state::NextState;
use country_core::{
    app_loading_state::AppLoadingState,
    game_config::GameConfig,
    game_profile::{GameProfile, SaveGameProfileCmd},
    resources::{
        daily::{CurrentDaily, DailyThemeState},
        gui::{EscMenuState, NextEscMenuState},
        session::{ActiveSession, GladeType, NextSession, SavedSessionList, SessionMeta},
    },
    systems::audio::music::{music_player::MusicEvent, music_system::MusicAudioState},
};
use std::time::SystemTime;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NewGlade {
    Summer,
    Olden,
    Autumn,
    Flowery,
    Winter,
    Daily,
}
impl NewGlade {
    const ALL: [Self; 6] = [
        Self::Summer,
        Self::Olden,
        Self::Autumn,
        Self::Flowery,
        Self::Winter,
        Self::Daily,
    ];
    fn theme(self) -> Option<GladeType> {
        Some(match self {
            Self::Summer => GladeType::Summer,
            Self::Olden => GladeType::Olden,
            Self::Autumn => GladeType::Autumn,
            Self::Flowery => GladeType::Flowery,
            Self::Winter => GladeType::Winter,
            Self::Daily => return None,
        })
    }
    fn key(self, weekly: bool) -> &'static str {
        match self {
            Self::Summer => "New.Glade.Summer",
            Self::Olden => "New.Glade.Olden",
            Self::Autumn => "New.Glade.Autumn",
            Self::Flowery => "New.Glade.Flowery",
            Self::Winter => "New.Glade.Winter",
            Self::Daily if weekly => "New.Glade.Weekly",
            Self::Daily => "New.Glade.Daily",
        }
    }
}

/// 原 Local：保留悬停选择和计时；背景路径由真正的前帧 ImageContent 拥有。
pub struct HoveredGlade {
    selected: Option<NewGlade>,
    elapsed: f32,
}
impl Default for HoveredGlade {
    fn default() -> Self {
        Self {
            selected: None,
            elapsed: 0.,
        }
    }
}
#[derive(Default)]
pub struct DemoSaveLimitWarning(bool);

/// 每日请求复制完整元数据和活动历史；未下载、缺失和失败均不能构造请求。
pub fn resolve_new_glade(choice: NewGlade, daily: &CurrentDaily) -> Option<NextSession> {
    if let Some(glade) = choice.theme() {
        Some(NextSession::new_game(
            SessionMeta {
                glade,
                ..Default::default()
            },
            None,
        ))
    } else if let DailyThemeState::Ready(info) = &daily.0 {
        Some(NextSession::new_game(
            info.meta.clone(),
            Some(info.history_path.clone()),
        ))
    } else {
        None
    }
}

/// 原 UI 的 MusicAudioState 沿用宿主已有 Event 类型，不创建另一份音频状态。
#[allow(clippy::too_many_arguments)]
pub fn ui<E: MusicEvent>(
    _active: Res<ActiveSession>,
    sessions: Res<SavedSessionList>,
    time: Res<bevy_time::Time>,
    menu: Res<EscMenuState>,
    daily: Res<CurrentDaily>,
    mut hover: Local<HoveredGlade>,
    mut next: ResMut<NextEscMenuState>,
    mut mei: ResMut<MeiContext>,
    mut loading: ResMut<NextState<AppLoadingState>>,
    mut commands: Commands,
    mut warning: Local<DemoSaveLimitWarning>,
    mut music: Option<ResMut<MusicAudioState<E>>>,
    mut profile: ResMut<GameProfile>,
    mut save: EventWriter<SaveGameProfileCmd>,
    mut welcome: EventWriter<ShowDailyWelcomeCmd>,
    config: Res<GameConfig>,
) {
    if !matches!(*menu, EscMenuState::NewSession) {
        if let Some(music) = music.as_mut() {
            music.preview_glade = None;
        }
        return;
    }
    if let Some(music) = music.as_mut() {
        music.preview_glade = Some(GladeType::Summer);
    }
    mei.safe_frame("back", false)
        .template("menu/back")
        .with_slot("button", |slot| {
            let back = slot.back_button("back").pill_hotkey(18);
            back.node_mut()
                .set_sounds(0, if warning.0 { 11 } else { 9 });
            if back.node().used() {
                if warning.0 {
                    warning.0 = false;
                } else {
                    next.set(EscMenuState::Main);
                }
            }
        });
    let mut selected = None;
    if warning.0 {
        let modal = mei
            .fullscreen("demo save warning")
            .template("menu/demo_save_warning");
        modal.with_slot("answer", |slot| {
            let no = slot
                .menu_button(&lang_lib::lookup("Continue.Nope"))
                .menu_hotkey(18);
            if no.node().used() {
                warning.0 = false;
            }
            if slot
                .menu_button(&lang_lib::lookup("Continue.Yep"))
                .node()
                .used()
            {
                warning.0 = false;
                selected = Some(NewGlade::Summer);
            }
        });
    } else {
        hover.elapsed += time.delta_secs();
        let root = mei.fullscreen("menu screen");
        let view = root.template("menu/new_session");
        view.with_slot("header", |slot| {
            let text = label(
                slot,
                lang_lib::lookup("Menu.New.Header").to_uppercase(),
                90.,
                tg_text::FontFamily::Label,
                0xffffffff,
            );
            text.content_mut::<LabelContent>()
                .text
                .set_weight(tg_text::Weight(800));
            text.content_mut::<LabelContent>()
                .text
                .set_align(tg_text::Align::Left);
        });
        view.with_slot("save_note", |slot| {
            label(
                slot,
                lang_lib::lookup("Menu.YourGameIsSaved").to_string(),
                30.,
                tg_text::FontFamily::Hand,
                0xffffffe0,
            )
            .content_mut::<LabelContent>()
            .text
            .set_align(tg_text::Align::Left);
        });
        let mut changed_hover = None;
        let weekly = matches!(&daily.0,DailyThemeState::Ready(info) if info.is_weekly());
        view.with_slot("list", |slot| {
            for (index, choice) in NewGlade::ALL.into_iter().enumerate() {
                if choice == NewGlade::Daily {
                    slot.child("daily separator")
                        .template("menu/new_session_separator");
                }
                let button = slot.menu_button(&lang_lib::lookup(choice.key(weekly)));
                let allowed = !config.dev.demo || choice == NewGlade::Summer;
                button
                    .node_mut()
                    .set_sounds(0, if allowed { 2 } else { 12 });
                button.node_mut().with_nav(true);
                if index == 0 {
                    button.node_mut().default_focus();
                }
                button
                    .content_mut::<MenuButtonContent>()
                    .text
                    .set_metrics(60.)
                    .expect("原新建按钮字号");
                button
                    .content_mut::<MenuButtonContent>()
                    .text
                    .set_align(tg_text::Align::Center);
                button
                    .content_mut::<MenuButtonContent>()
                    .text
                    .set_max_width(400.)
                    .expect("原新建按钮宽度");
                if button.node().hover() {
                    changed_hover = Some(choice);
                }
                if button.node().used() && allowed {
                    if config.dev.demo && sessions.sessions.len() >= 3 {
                        warning.0 = true;
                    } else {
                        selected = Some(choice);
                    }
                }
            }
        });
        view.with_slot("back", |slot| {
            let back = slot.menu_button(&lang_lib::lookup("Menu.Back"));
            if back.node().used() && !warning.0 {
                next.set(EscMenuState::Main);
            }
        });
        if let Some(choice) = changed_hover {
            hover.selected = Some(choice);
            hover.elapsed = 0.;
        }
        if let Some(choice) = hover.selected {
            let theme = choice.theme().or_else(|| match &daily.0 {
                DailyThemeState::Ready(info) => Some(info.meta.glade),
                _ => None,
            });
            if let Some(music) = music.as_mut() {
                music.preview_glade = theme;
            }
            if choice == NewGlade::Daily {
                view.with_slot("info_panel", |slot| {
                    daily_info(slot, &daily, config.dev.demo, SystemTime::now())
                });
            } else if config.dev.demo && choice != NewGlade::Summer {
                view.with_slot("info_panel", |slot| {
                    slot.template("menu/new_session_not_in_demo")
                        .context
                        .node_mut()
                        .local_z = 2.;
                });
            }
        }
    }
    // 原两个1000高的BlurExtend背景：新图在底，旧图alpha=(1-4t)*255饱和截断。
    let background = mei.fullscreen_without_block_input("new session background 1");
    background.node_mut().style.expand = 1.;
    // 与 image<&str> 相同的原类型身份；必须在写新路径前取回实际回收内容，
    // 菜单卸载后节点消失时自然回退Summer，不另建跨卸载残留的图片状态。
    use crate::mei::core::{GlobalId, calculate_hash};
    let local = calculate_hash(&(calculate_hash(&("bg", ())), 0x01f6d60c8858b8ffu64));
    let current =
        background.child_with_global_id(GlobalId(calculate_hash(&(background.id.0, local))));
    let content = current.get_content_mut_impl::<ImageContent>(false);
    let previous_path =
        std::mem::replace(&mut content.image, background_path(hover.selected, &daily));
    content.height = 1000.;
    content.fit_parent_height = false;
    content.blur_extend = true;
    let previous_path = if previous_path.is_empty() {
        "new_glade/summer"
    } else {
        &previous_path
    };
    let previous = mei.fullscreen_without_block_input("new session background 2");
    previous.node_mut().local_z = 1.;
    previous.node_mut().style.expand = 1.;
    let image = previous.image("bg", previous_path, 1000.).blur_extend();
    image.content_mut::<ImageContent>().color =
        0xffffff00 | (((1. - hover.elapsed * 4.) * 255.).clamp(0., 255.) as u32);
    if let Some(choice) = selected {
        if let Some(request) = resolve_new_glade(choice, &daily) {
            profile.increment_new_glade_count();
            save.write(SaveGameProfileCmd);
            commands.insert_resource(request);
            loading.set(AppLoadingState::Reload);
            next.set(EscMenuState::Closed);
            if choice == NewGlade::Daily {
                welcome.write(ShowDailyWelcomeCmd);
            }
        }
    }
}

fn background_path(choice: Option<NewGlade>, daily: &CurrentDaily) -> String {
    match choice {
        Some(NewGlade::Daily) => match &daily.0 {
            DailyThemeState::Ready(info) => {
                format!("new_glade/daily_{}", info.meta.glade.assets_folder())
            }
            _ => "new_glade/daily_unavailable".into(),
        },
        Some(choice) => format!(
            "new_glade/{}",
            choice.theme().expect("普通主题").assets_folder()
        ),
        None => "new_glade/summer".into(),
    }
}
pub(super) fn label(
    slot: NodeContext,
    text: String,
    size: f32,
    font: tg_text::FontFamily,
    color: u32,
) -> NodeContext {
    let node = slot.label(text);
    let content = node.content_mut::<LabelContent>();
    content.color = color;
    content.text.set_metrics(size).expect("原菜单字号");
    content.text.set_family(font).expect("原菜单字体");
    content.text.set_align(tg_text::Align::Center);
    node
}
fn daily_info(slot: NodeContext, daily: &CurrentDaily, demo: bool, now: SystemTime) {
    let view = match &daily.0 {
        _ if demo => {
            slot.template("menu/new_session_daily_not_in_demo")
                .context
                .node_mut()
                .local_z = 2.;
            return;
        }
        DailyThemeState::Downloading => {
            let v = slot.template("menu/new_session_daily_downloading");
            v.context.node_mut().local_z = 2.;
            v.with_slot("reason", |s| {
                label(
                    s,
                    String::new(),
                    32.,
                    tg_text::FontFamily::Label,
                    0xffffffff,
                )
                .content_mut::<LabelContent>()
                .text
                .set_max_width(800.)
                .expect("原说明宽度");
            });
            return;
        }
        DailyThemeState::NoCurrentTheme | DailyThemeState::DownloadError(_) => {
            let v = slot.template("menu/new_session_daily_missing");
            let reason = match &daily.0 {
                DailyThemeState::DownloadError(error) => {
                    format!("{}: {error}", lang_lib::lookup("Daily.DownloadError"))
                }
                _ => lang_lib::lookup("Daily.NoCurrentTheme").to_string(),
            };
            v.context.node_mut().local_z = 2.;
            v.with_slot("reason", |s| {
                label(s, reason, 32., tg_text::FontFamily::Label, 0xffffffff)
                    .content_mut::<LabelContent>()
                    .text
                    .set_max_width(800.)
                    .expect("原错误说明宽度");
            });
            return;
        }
        DailyThemeState::Ready(info) => {
            let v = slot.template(if info.meta.is_xmas2024() {
                "menu/new_session_daily_info_xmas"
            } else {
                "menu/new_session_daily_info"
            });
            v.with_slot("theme_kind", |s| {
                label(
                    s,
                    lang_lib::lookup(if info.is_weekly() {
                        "New.Glade.ThisWeeksTheme"
                    } else {
                        "Daily.TodaysTheme"
                    })
                    .to_string(),
                    56.,
                    tg_text::FontFamily::Hand,
                    0xffffffff,
                );
            });
            v.with_slot("theme_name", |s| {
                label(
                    s,
                    info.get_name(&lang_lib::game_locale()).into(),
                    90.,
                    tg_text::FontFamily::Label,
                    0xffffffff,
                )
                .content_mut::<LabelContent>()
                .text
                .set_max_width(600.)
                .expect("原主题名宽度");
            });
            v.with_slot("next_theme_in", |s| {
                label(
                    s,
                    countdown(
                        info.end_time
                            .duration_since(now)
                            .unwrap_or_default()
                            .as_secs(),
                    ),
                    32.,
                    tg_text::FontFamily::Hand,
                    0xffffffff,
                )
                .content_mut::<LabelContent>()
                .text
                .set_max_width(750.)
                .expect("原倒计时宽度");
            });
            v
        }
    };
    view.context.node_mut().local_z = 2.;
}
fn countdown(seconds: u64) -> String {
    let mut text = format!("{}s", seconds % 60);
    for (value, unit) in [
        (seconds / 60 % 60, "min"),
        (seconds / 3600 % 24, "h"),
        (seconds / 86400, "d"),
    ] {
        if value != 0 {
            text = format!("{value}{unit} {text}");
        }
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::{
        daily::DailyThemeInfo,
        image_formation::ImageFormationConfig,
        session::{NextSessionKind, SessionId, SessionMutator},
    };
    use std::{
        path::PathBuf,
        sync::Arc,
        time::{Duration, UNIX_EPOCH},
    };

    // 本测试不安装音频资源；不可构造事件类型确保没有模拟音频成功路径。
    enum AbsentMusicEvent {}
    impl MusicEvent for AbsentMusicEvent {
        type Error = std::convert::Infallible;
        fn start(&mut self) -> Result<(), Self::Error> {
            match *self {}
        }
        fn stop(
            &mut self,
            _: country_core::systems::audio::music::music_player::StopMode,
        ) -> Result<(), Self::Error> {
            match *self {}
        }
        fn set_paused(&mut self, _: bool) -> Result<(), Self::Error> {
            match *self {}
        }
        fn paused(&self) -> Result<bool, Self::Error> {
            match *self {}
        }
        fn playback_state(
            &self,
        ) -> Result<country_core::resources::audio_instance::PlaybackState, Self::Error> {
            match *self {}
        }
        fn timeline_position(&self) -> Result<i32, Self::Error> {
            match *self {}
        }
        fn install_beat_callback(&mut self) -> Result<(), Self::Error> {
            match *self {}
        }
    }
    fn daily() -> CurrentDaily {
        CurrentDaily(DailyThemeState::Ready(DailyThemeInfo {
            languages: vec!["Actual theme".into()],
            history_path: PathBuf::from("/theme/content/history.json"),
            meta: SessionMeta {
                glade: GladeType::Winter,
                mutators: vec![SessionMutator::Xmas2024],
                image: Some(ImageFormationConfig::default_third(true)),
            },
            credits: None,
            start_time: UNIX_EPOCH,
            end_time: UNIX_EPOCH + Duration::from_secs(604800),
        }))
    }
    #[test]
    fn every_choice_owns_all_request_data_and_unavailable_daily_cannot_start() {
        let daily = daily();
        for choice in NewGlade::ALL {
            let request = resolve_new_glade(choice, &daily).unwrap();
            assert_eq!(request.kind, NextSessionKind::Create);
            let data = request.new_session_options().unwrap();
            if choice == NewGlade::Daily {
                let DailyThemeState::Ready(info) = &daily.0 else {
                    unreachable!()
                };
                assert_eq!(data.metadata, info.meta);
                assert_eq!(data.history_path.as_ref(), Some(&info.history_path));
            } else {
                assert_eq!(data.metadata.glade, choice.theme().unwrap());
                assert!(data.metadata.image.is_none());
                assert!(data.metadata.mutators.is_empty());
                assert!(data.history_path.is_none());
            }
        }
        for state in [
            DailyThemeState::Downloading,
            DailyThemeState::NoCurrentTheme,
            DailyThemeState::DownloadError("broken package".into()),
        ] {
            assert!(resolve_new_glade(NewGlade::Daily, &CurrentDaily(state)).is_none());
        }
        assert_eq!(countdown(0), "0s");
        assert_eq!(countdown(3600), "1h 0s");
        assert_eq!(countdown(90061), "1d 1h 1min 1s");
    }
    pub(super) fn templates(names: &[&str]) -> MeiContext {
        let installed =
            PathBuf::from(std::env::var_os("TG_INSTALL_ROOT").expect("需原版TG_INSTALL_ROOT"));
        let mut mei = MeiContext::default();
        for name in names {
            let config = crate::mei::config::read_node_config_from_file(
                &installed.join("assets/ui").join(format!("{name}.ron")),
            )
            .unwrap();
            mei.configs.insert((*name).into(), Arc::new(config));
        }
        mei.load_font_table(&installed.join("assets/fonts/table.ron"))
            .unwrap();
        mei
    }
    fn activate(app: &mut bevy_app::App, key: &str, bit: u8) {
        let text = lang_lib::lookup(key);
        let mei = app.world_mut().resource_mut::<MeiContext>().into_inner();
        for node in mei.nodes.values_mut() {
            node.state_bits &= !0x30;
        }
        let button = mei
            .nodes
            .values_mut()
            .find(|node| {
                let content: &dyn std::any::Any = node.content.as_ref();
                content
                    .downcast_ref::<MenuButtonContent>()
                    .is_some_and(|button| button.pending_text.as_deref() == Some(text.as_ref()))
            })
            .expect("实际菜单按钮");
        button.state_bits |= bit;
        mei.recycle_frame_nodes();
        app.update();
    }
    fn next_frame(app: &mut bevy_app::App) {
        let mei = app.world_mut().resource_mut::<MeiContext>().into_inner();
        for node in mei.nodes.values_mut() {
            node.state_bits &= !0x30;
        }
        mei.recycle_frame_nodes();
        app.update();
    }
    #[test]
    #[ignore = "需TG_INSTALL_ROOT原模板和字体；不发送OS输入"]
    fn original_background_unload_and_demo_save_limit_cancel_confirm() {
        let mei = templates(&[
            "menu/new_session",
            "menu/back",
            "menu/new_session_separator",
            "menu/new_session_not_in_demo",
            "menu/demo_save_warning",
        ]);
        let mut app = bevy_app::App::new();
        app.insert_resource(mei)
            .insert_resource(CurrentDaily::default())
            .insert_resource(EscMenuState::NewSession)
            .insert_resource(NextEscMenuState::default())
            .insert_resource(NextState::<AppLoadingState>::default())
            .insert_resource(GameProfile::default())
            .insert_resource(GameConfig::default())
            .insert_resource(SavedSessionList::default())
            .insert_resource(bevy_time::Time::<()>::default())
            .insert_resource(ActiveSession::new(SessionId::new(), SessionMeta::default()))
            .add_event::<SaveGameProfileCmd>()
            .add_event::<ShowDailyWelcomeCmd>()
            .add_systems(bevy_app::Update, ui::<AbsentMusicEvent>);
        fn image(app: &bevy_app::App, root: &str) -> String {
            let mei = app.world().resource::<MeiContext>();
            let root = mei
                .nodes
                .get(&crate::mei::core::GlobalId::new(root))
                .unwrap();
            let node = mei.nodes.get(&root.children[0]).unwrap();
            let content: &dyn std::any::Any = node.content.as_ref();
            content
                .downcast_ref::<ImageContent>()
                .unwrap()
                .image
                .clone()
        }
        app.update();
        activate(&mut app, "New.Glade.Winter", 1);
        assert_eq!(image(&app, "new session background 1"), "new_glade/winter");
        assert_eq!(image(&app, "new session background 2"), "new_glade/summer");
        next_frame(&mut app);
        assert_eq!(image(&app, "new session background 2"), "new_glade/winter");
        *app.world_mut().resource_mut::<EscMenuState>() = EscMenuState::Main;
        next_frame(&mut app);
        assert!(app.world().resource::<MeiContext>().nodes.is_empty());
        *app.world_mut().resource_mut::<EscMenuState>() = EscMenuState::NewSession;
        next_frame(&mut app);
        assert_eq!(image(&app, "new session background 1"), "new_glade/winter");
        assert_eq!(
            image(&app, "new session background 2"),
            "new_glade/summer",
            "旧内容卸载后从原默认背景开始"
        );
        app.world_mut().resource_mut::<GameConfig>().dev.demo = true;
        activate(&mut app, "New.Glade.Winter", 0x10);
        assert!(!app.world().contains_resource::<NextSession>());
        app.world_mut().resource_mut::<SavedSessionList>().sessions = (0..3)
            .map(|_| country_core::resources::session::SavedSession {
                id: SessionId::new(),
                modified: UNIX_EPOCH,
                readiness: Default::default(),
                screenshot_exists: false,
                loading_background_exists: false,
            })
            .collect();
        activate(&mut app, "New.Glade.Summer", 0x10);
        assert!(!app.world().contains_resource::<NextSession>());
        next_frame(&mut app);
        activate(&mut app, "Continue.Nope", 0x10);
        assert!(!app.world().contains_resource::<NextSession>());
        next_frame(&mut app);
        activate(&mut app, "New.Glade.Summer", 0x10);
        next_frame(&mut app);
        activate(&mut app, "Continue.Yep", 0x10);
        let request = app.world().resource::<NextSession>();
        assert_eq!(
            request.new_session_options().unwrap().metadata.glade,
            GladeType::Summer
        );
        assert_eq!(app.world().resource::<GameProfile>().new_glade_count, 1);
        assert_eq!(
            app.world().resource::<Events<SaveGameProfileCmd>>().len(),
            1
        );
        assert!(
            app.world()
                .resource::<Events<ShowDailyWelcomeCmd>>()
                .is_empty()
        );
    }
    #[test]
    #[ignore = "需TG_INSTALL_ROOT原模板和字体；不发送OS输入"]
    fn original_template_click_publishes_owned_daily_request_reload_profile_and_welcome() {
        use country_core::{
            resources::{
                glade_settings::GladeSettings,
                session::{NewSessionStartedCmd, SessionLoadedCmd},
            },
            systems::{
                mode_manager::AppMode,
                save::{
                    load_completion_checker::DisplayCorruptedSaveFileWarning,
                    load_game::{LoadGameManager, LoadGameSource},
                    save_game::AutoSaveState,
                },
            },
        };
        let installed = PathBuf::from(std::env::var_os("TG_INSTALL_ROOT").expect("需原版安装目录"));
        let account = tempfile::tempdir().unwrap();
        paths::initialize_paths_with_installation(0, Some(account.path()), &installed)
            .expect("该验收需独立测试进程中的临时账号");
        let mut theme = daily();
        let DailyThemeState::Ready(info) = &mut theme.0 else {
            unreachable!()
        };
        info.history_path = installed.join("assets/starting-builds/full/intro/history.json");
        assert!(info.history_path.is_file());
        let expected_meta = info.meta.clone();
        let expected_path = info.history_path.clone();
        let mei = templates(&[
            "menu/new_session",
            "menu/back",
            "menu/new_session_separator",
            "menu/new_session_daily_info",
            "menu/new_session_daily_info_xmas",
            "menu/new_session_daily_downloading",
            "menu/new_session_daily_missing",
        ]);
        let mut app = bevy_app::App::new();
        app.insert_resource(mei)
            .insert_resource(CurrentDaily::default())
            .insert_resource(EscMenuState::NewSession)
            .insert_resource(NextEscMenuState::default())
            .insert_resource(NextState::<AppLoadingState>::default())
            .insert_resource(GameProfile::default())
            .insert_resource(GameConfig::default())
            .insert_resource(SavedSessionList::default())
            .insert_resource(bevy_time::Time::<()>::default())
            .insert_resource(ActiveSession::new(SessionId::new(), SessionMeta::default()))
            .add_event::<SaveGameProfileCmd>()
            .add_event::<ShowDailyWelcomeCmd>()
            .add_systems(bevy_app::Update, ui::<AbsentMusicEvent>);
        // Drive each unavailable state through the real menu producer and
        // attempt its actual Daily button; no request or profile mutation may
        // be emitted before a complete theme is ready.
        for state in [DailyThemeState::Downloading, DailyThemeState::NoCurrentTheme,
            DailyThemeState::DownloadError("fixture download failure".into())] {
            app.world_mut().resource_mut::<CurrentDaily>().0 = state;
            app.update();
            let key = lang_lib::lookup("New.Glade.Daily");
            let mei = app.world_mut().resource_mut::<MeiContext>().into_inner();
            let button = mei.nodes.values_mut().find(|node| {
                let any: &dyn std::any::Any = node.content.as_ref();
                any.downcast_ref::<MenuButtonContent>()
                    .is_some_and(|button| button.pending_text.as_deref() == Some(key.as_ref()))
            }).expect("原每日主题按钮");
            button.state_bits |= 0x10;
            mei.recycle_frame_nodes();
            app.update();
            assert!(!app.world().contains_resource::<NextSession>());
            assert_eq!(app.world().resource::<GameProfile>().new_glade_count, 0);
            assert!(!matches!(*app.world().resource::<NextState<AppLoadingState>>(), NextState::Pending(AppLoadingState::Reload)));
            assert!(app.world().resource::<Events<ShowDailyWelcomeCmd>>().is_empty());
            assert!(app.world().resource::<Events<SaveGameProfileCmd>>().is_empty());
            app.world_mut().resource_mut::<MeiContext>().recycle_frame_nodes();
        }
        app.insert_resource(theme);
        app.update();
        let key = lang_lib::lookup("New.Glade.Weekly");
        let mei = app.world_mut().resource_mut::<MeiContext>().into_inner();
        let button = mei
            .nodes
            .values_mut()
            .find(|node| {
                ({
                    let any: &dyn std::any::Any = node.content.as_ref();
                    any
                })
                .downcast_ref::<MenuButtonContent>()
                .is_some_and(|button| button.pending_text.as_deref() == Some(key.as_ref()))
            })
            .expect("原每周主题按钮");
        button.state_bits |= 0x10;
        mei.recycle_frame_nodes();
        app.update();
        let request = app.world().resource::<NextSession>();
        assert_eq!(request.kind, NextSessionKind::Create);
        assert_eq!(
            request.new_session_options().unwrap().metadata.mutators,
            [SessionMutator::Xmas2024]
        );
        assert_eq!(app.world().resource::<GameProfile>().new_glade_count, 1);
        assert!(matches!(
            app.world().resource::<NextEscMenuState>().0,
            Some(EscMenuState::Closed)
        ));
        assert!(matches!(
            *app.world().resource::<NextState<AppLoadingState>>(),
            NextState::Pending(AppLoadingState::Reload)
        ));
        assert_eq!(
            app.world().resource::<Events<SaveGameProfileCmd>>().len(),
            1
        );
        assert_eq!(
            app.world().resource::<Events<ShowDailyWelcomeCmd>>().len(),
            1
        );
        // 在同一个World消费真实菜单请求；保留原历史文件，不伪造解析结果或改用户存档。
        app.insert_resource(AutoSaveState::default())
            .insert_resource(LoadGameManager::default())
            .insert_resource(AppMode::default())
            .insert_resource(GladeSettings::default())
            .add_event::<NewSessionStartedCmd>()
            .add_event::<SessionLoadedCmd>()
            .add_event::<DisplayCorruptedSaveFileWarning>();
        let mut startup = Schedule::default();
        startup.add_systems(country_core::startup::startup_session::load_game_session);
        startup.run(app.world_mut());
        assert!(!app.world().contains_resource::<NextSession>());
        let active = app.world().resource::<ActiveSession>();
        assert_eq!(active.meta, expected_meta);
        assert_eq!(
            SessionMeta::try_load_from_path(active.id.save_folder_path().join("meta.json"))
                .unwrap(),
            expected_meta
        );
        let manager = app.world().resource::<LoadGameManager>();
        assert!(matches!(&manager.source,LoadGameSource::Path(path) if *path==expected_path));
        assert!(manager.options.as_ref().unwrap().new_session);
        assert_eq!(
            app.world().resource::<Events<NewSessionStartedCmd>>().len(),
            1
        );
        assert!(
            app.world()
                .resource::<Events<DisplayCorruptedSaveFileWarning>>()
                .is_empty()
        );
    }
}
