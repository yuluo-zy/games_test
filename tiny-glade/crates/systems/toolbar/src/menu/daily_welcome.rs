//! 原 B 每日主题欢迎页（141ebedd0）：请求事件重开，游戏时钟淡入，确认/菜单抢占关闭。
use crate::mei::{
    core::MeiContext,
    widgets::{LabelContent, MenuButtonContent},
};
use bevy_ecs::prelude::*;
use country_core::{
    resources::{
        daily::{CurrentDaily, DailyThemeState},
        gui::EscMenuState,
        time::GameTime,
    },
    systems::steam::SteamClientResource,
};

#[derive(Event, Clone, Copy, Debug, Default)]
pub struct ShowDailyWelcomeCmd;
#[derive(Resource, Debug)]
pub struct DailyWelcomeScreenState {
    dismissed: bool,
    elapsed: f32,
}
impl Default for DailyWelcomeScreenState {
    fn default() -> Self {
        Self {
            dismissed: true,
            elapsed: 0.,
        }
    }
}

pub fn ui(
    mut mei: ResMut<MeiContext>,
    mut state: ResMut<DailyWelcomeScreenState>,
    time: Res<GameTime>,
    menu: Res<EscMenuState>,
    mut events: EventReader<ShowDailyWelcomeCmd>,
    daily: Res<CurrentDaily>,
    steam: Option<Res<SteamClientResource>>,
) {
    // 原 SteamDeck 分支在读取事件之前返回；继续沿用宿主已注入的客户端接口。
    if steam
        .as_ref()
        .and_then(|s| s.client.as_ref())
        .is_some_and(|s| s.is_steam_running_on_steam_deck())
    {
        return;
    }
    if state.dismissed {
        // 原关闭分支只消费事件并重置8B状态，然后立即返回。
        // 新建者本帧仅写NextEscMenuState，当前菜单尚是NewSession；不能同帧用它取消欢迎。
        if events.read().count() != 0 {
            *state = DailyWelcomeScreenState {
                dismissed: false,
                elapsed: 0.,
            };
        }
        return;
    }
    if !menu.is_closed() {
        state.dismissed = true;
        return;
    }
    state.elapsed += time.delta;
    let screen = mei
        .safe_frame("welcome", true)
        .template("menu/welcome_daily");
    let value = (state.elapsed - 0.7) / 0.3;
    let value = if value.is_nan() || value >= 0. {
        value
    } else {
        0.
    };
    let value = if value.is_nan() || value <= 1. {
        value
    } else {
        1.
    };
    screen.context.node_mut().local_opacity = (3. - (value + value)) * (value * value);
    screen.with_slot("theme", |slot| {
        let name = match &daily.0 {
            DailyThemeState::Ready(info) => info.get_name(&lang_lib::game_locale()),
            _ => "???",
        };
        let label = super::new_session::label(
            slot,
            name.into(),
            30.,
            tg_text::FontFamily::Label,
            0x7b5a51ff,
        );
        label
            .content_mut::<LabelContent>()
            .text
            .set_weight(tg_text::Weight(700));
        label
            .content_mut::<LabelContent>()
            .text
            .set_align(tg_text::Align::Left);
    });
    screen.with_slot("buttons", |slot| {
        let button = slot
            .menu_button(&format!("{} 🐑", lang_lib::lookup("Welcome.LetsGo")))
            .menu_hotkey(19);
        button
            .content_mut::<MenuButtonContent>()
            .text
            .set_metrics(27.)
            .expect("原每日欢迎按钮字号");
        if button.node().used() {
            state.dismissed = true;
        }
    });
    if let DailyThemeState::Ready(info) = &daily.0 {
        screen.with_slot("intro2", |slot| {
            super::new_session::label(
                slot,
                lang_lib::lookup(if info.is_weekly() {
                    "Theme.Weekly.Welcome.Intro2"
                } else {
                    "Daily.Welcome.Intro2"
                })
                .to_string(),
                17.,
                tg_text::FontFamily::Hand,
                0x7b5a51ff,
            )
            .content_mut::<LabelContent>()
            .text
            .set_max_width(400.)
            .expect("原每日介绍宽度");
        });
        if let Some(credits) = &info.credits {
            for (slot_name, key, value) in [
                ("idea", "Theme.SuggestionBy", &credits.theme_idea),
                ("author", "Daily.Welcome.Author", &credits.author),
                ("author", "Daily.Welcome.Inspiration", &credits.inspiration),
            ] {
                if let Some(value) = value {
                    screen.with_slot(slot_name, |slot| {
                        for text in [lang_lib::lookup(key).to_string(), value.clone()] {
                            let label = super::new_session::label(
                                slot,
                                text,
                                12.,
                                tg_text::FontFamily::Hand,
                                0x7b5a51ff,
                            );
                            label
                                .content_mut::<LabelContent>()
                                .text
                                .set_weight(tg_text::Weight(300));
                        }
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore = "需TG_INSTALL_ROOT原模板和字体；不发送OS输入"]
    fn original_event_fade_confirm_menu_preemption_and_reopen() {
        let installed = std::path::PathBuf::from(
            std::env::var_os("TG_INSTALL_ROOT").expect("需原版TG_INSTALL_ROOT"),
        );
        let mut mei = MeiContext::default();
        let template = crate::mei::config::read_node_config_from_file(
            &installed.join("assets/ui/menu/welcome_daily.ron"),
        )
        .unwrap();
        mei.configs
            .insert("menu/welcome_daily".into(), std::sync::Arc::new(template));
        mei.load_font_table(&installed.join("assets/fonts/table.ron"))
            .unwrap();
        let mut app = bevy_app::App::new();
        app.insert_resource(mei)
            .insert_resource(DailyWelcomeScreenState::default())
            .insert_resource(CurrentDaily(DailyThemeState::Ready(
                country_core::resources::daily::DailyThemeInfo {
                    languages: vec!["Actual theme".into()],
                    history_path: std::path::PathBuf::from("/not-read/history.json"),
                    meta: Default::default(),
                    credits: Some(country_core::resources::daily::DailyThemeCredits {
                        author: Some("Alice".into()),
                        inspiration: Some("Bob".into()),
                        theme_idea: Some("Carol".into()),
                    }),
                    start_time: std::time::UNIX_EPOCH,
                    end_time: std::time::UNIX_EPOCH + std::time::Duration::from_secs(432000),
                },
            )))
            .insert_resource(EscMenuState::Closed)
            .insert_resource(GameTime {
                delta: 0.4,
                ..Default::default()
            })
            .add_event::<ShowDailyWelcomeCmd>()
            .add_systems(bevy_app::Update, ui);
        app.update();
        assert!(app.world().resource::<MeiContext>().nodes.is_empty());
        *app.world_mut().resource_mut::<EscMenuState>() = EscMenuState::NewSession;
        app.world_mut().send_event(ShowDailyWelcomeCmd);
        app.update();
        assert!(!app.world().resource::<DailyWelcomeScreenState>().dismissed);
        assert!(
            app.world().resource::<MeiContext>().nodes.is_empty(),
            "新建命令这帧仅接收，不被旧菜单抢占"
        );
        *app.world_mut().resource_mut::<EscMenuState>() = EscMenuState::Closed;
        app.update();
        assert!(
            app.world()
                .resource::<MeiContext>()
                .nodes
                .values()
                .any(|node| node.local_opacity == 0.)
        );
        for expected in ["Actual theme", "Alice", "Bob", "Carol"] {
            assert!(
                app.world()
                    .resource::<MeiContext>()
                    .nodes
                    .values()
                    .any(|node| {
                        let content: &dyn std::any::Any = node.content.as_ref();
                        content
                            .downcast_ref::<LabelContent>()
                            .is_some_and(|label| label.pending_text.as_deref() == Some(expected))
                    }),
                "每日标题和活动归属必须进入实际标签消费者：{expected}"
            );
        }
        app.world_mut()
            .resource_mut::<MeiContext>()
            .recycle_frame_nodes();
        app.update();
        let opacity = app
            .world()
            .resource::<MeiContext>()
            .nodes
            .values()
            .filter_map(|node| {
                if node.local_opacity < 1. {
                    Some(node.local_opacity)
                } else {
                    None
                }
            })
            .next()
            .unwrap();
        assert!((opacity - 0.2592594).abs() < 0.000001);
        let mei = app.world_mut().resource_mut::<MeiContext>().into_inner();
        mei.nodes
            .values_mut()
            .find(|node| node.content.type_name() == "toolbar::mei::widgets::MenuButtonContent")
            .unwrap()
            .state_bits |= 0x10;
        mei.recycle_frame_nodes();
        app.update();
        assert!(app.world().resource::<DailyWelcomeScreenState>().dismissed);
        // 此CPU树消费者不运行GPU帧尾；按原mei_post_update清除一次性激活位。
        for node in app
            .world_mut()
            .resource_mut::<MeiContext>()
            .nodes
            .values_mut()
        {
            node.state_bits &= !0x30;
        }
        app.world_mut()
            .resource_mut::<MeiContext>()
            .recycle_frame_nodes();
        app.update();
        assert!(app.world().resource::<MeiContext>().nodes.is_empty());
        app.world_mut().send_event(ShowDailyWelcomeCmd);
        app.update();
        assert!(!app.world().resource::<DailyWelcomeScreenState>().dismissed);
        *app.world_mut().resource_mut::<EscMenuState>() = EscMenuState::Main;
        app.world_mut()
            .resource_mut::<MeiContext>()
            .recycle_frame_nodes();
        app.update();
        assert!(app.world().resource::<DailyWelcomeScreenState>().dismissed);
        assert!(app.world().resource::<MeiContext>().nodes.is_empty());
    }
}
