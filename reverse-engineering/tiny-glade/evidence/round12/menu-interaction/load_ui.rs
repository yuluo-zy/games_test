//! Staged B toolbar::menu::load::ui and all modal/backup transactions.
//! This file is NOT registered or compiled. See REPORT.md for owner contracts.
//! Native original15632B@141e1fe80 and closures are bound in ../render/load.
use super::load::ThumbnailScrollContent;
use crate::mei::{
    core::{MeiContext, NodeContext},
    layout::{Justify, LayoutStyle, LengthPercentage as Length},
    widgets::{LabelContent, MenuButtonContent, PillButtonContent},
};
use bevy_ecs::prelude::*;
use bevy_state::state::NextState;
use country_core::{
    app_loading_state::AppLoadingState,
    game_config::GameConfig,
    resources::{
        glade_settings::GladeSettings,
        gui::{EscMenuState, LoadMenuBackupState, LoadMenuState, NextEscMenuState},
        history::{History, HistoryIdentity},
        session::{
            ActiveSession, GladeType, NextSession, SavedSessionList, SessionId, SessionMeta,
        },
    },
    systems::{
        audio::PlayMenuUiSound,
        save::{
            load_completion_checker::is_session_ok_to_load,
            load_game::{ParsedGameData, parse_saved_game_data},
            save_backup_list::{BackupSaveFile, BackupSaveFileKind, SaveBackupList},
            save_file_identity::{
                derive_fallback_save_file_identity, try_approx_parse_save_file_identity,
            },
            save_game::{AutoSaveState, DeleteSessionCmd},
        },
    },
};
use glam::Vec2;
use std::{
    fs::File,
    path::{Path, PathBuf},
    sync::OnceLock,
    time::{Duration, Instant},
};

const PAPER_TINT: u32 = 0x7b5a51a0;
#[derive(Default)]
pub struct LocalState {
    error: Option<SaveLoadError>,
}
pub struct SaveLoadError {
    path: PathBuf,
    message: String,
    expires: Instant,
}
impl LocalState {
    fn set_error(&mut self, path: PathBuf, error: impl std::fmt::Display) {
        self.error = Some(SaveLoadError {
            path,
            message: error.to_string(),
            expires: Instant::now() + Duration::from_secs(2),
        });
    }
}
fn meta(id: SessionId) -> SessionMeta {
    SessionMeta::try_load_from_path(id.save_folder_path().join("meta.json")).unwrap_or_default()
}
fn parse(path: &Path, metadata: &SessionMeta) -> anyhow::Result<ParsedGameData> {
    let settings = GladeSettings::load(metadata.assets_root());
    parse_saved_game_data(path, &settings)
}
fn queue_load(
    id: SessionId,
    data: ParsedGameData,
    commands: &mut Commands,
    next_loading: &mut NextState<AppLoadingState>,
    next: &mut NextEscMenuState,
) {
    commands.insert_resource(NextSession::from_parsed(id, data));
    // Native is tag2. Canonical AppLoadingState names tag2 Reload, not Loading.
    next_loading.set(AppLoadingState::Reload);
    next.set(EscMenuState::Closed);
}

/// Original has12 system params and a process-wide DEV_OVERRIDE_IS_DEMO_MODE.
/// Source adapter reads canonical per-App GameConfig as the13th param.
/// Original registration is set36+in_state(Game), distinct from GUI39.
#[allow(clippy::too_many_arguments)]
pub fn ui(
    active: Res<ActiveSession>,
    history: Res<History>,
    auto_save: Res<AutoSaveState>,
    mut current: ResMut<EscMenuState>,
    mut next: ResMut<NextEscMenuState>,
    mut mei: ResMut<MeiContext>,
    mut next_loading: ResMut<NextState<AppLoadingState>>,
    mut commands: Commands,
    sessions: Res<SavedSessionList>,
    mut local: Local<LocalState>,
    mut sounds: EventWriter<PlayMenuUiSound>,
    mut deletes: EventWriter<DeleteSessionCmd>,
    config: Res<GameConfig>,
) {
    let EscMenuState::Load(state) = &mut *current else {
        return;
    };
    let demo = config.dev.demo;
    let input_disabled = state.warning_session.is_some()
        || state.switch_glade_type.is_some()
        || state.backups.is_some();
    let root = mei.fullscreen("menu screen");
    let scroll = root.thumbnail_scroll(
        "builds",
        sessions.sessions.len() + 1,
        state.thumbnail_metrics.clone(),
    );
    if !input_disabled {
        scroll.node_mut().flags |= 4;
    }
    let (hovered, clicked, position, navigation_boundary) = {
        let content = scroll.get_content_mut_impl::<ThumbnailScrollContent>(true);
        if !content.initialized {
            content.scroll = 1. + content.gap_fraction / content.cell_fraction;
        }
        content.scroll_limits = Some(f32::from_bits(0x3e851eb8));
        content.navigation_speed = 10.;
        content.input_disabled = input_disabled;
        (
            content.hovered.filter(|i| *i <= sessions.sessions.len()),
            content.clicked.filter(|i| *i <= sessions.sessions.len()),
            content.hovered_position,
            content.navigation_boundary,
        )
    };
    if state.warning_session.is_none() && state.switch_glade_type.is_none() {
        let previous_hover = state.hovered_index;
        state.hovered_index = hovered;
        if let Some(clicked) = clicked {
            if clicked == 0 {
                next.set(EscMenuState::NewSession);
                sounds.write(PlayMenuUiSound { kind: 7 });
            } else {
                let selected = &sessions.sessions[clicked - 1];
                if (demo && clicked > 3)
                    || is_session_ok_to_load(selected.id, &auto_save.unsaved_files).deny_load()
                {
                    sounds.write(PlayMenuUiSound { kind: 12 });
                } else if selected.id == active.id {
                    next.set(EscMenuState::Closed);
                    sounds.write(PlayMenuUiSound { kind: 8 });
                } else {
                    let path = selected.id.save_folder_path().join("history.json");
                    match parse(&path, &meta(selected.id)) {
                        Ok(data) => {
                            queue_load(
                                selected.id,
                                data,
                                &mut commands,
                                &mut next_loading,
                                &mut next,
                            );
                            sounds.write(PlayMenuUiSound { kind: 8 });
                        }
                        Err(error) => local.set_error(path, error),
                    }
                }
            }
        }
        if let Some(index) = hovered.filter(|i| *i > 0) {
            let selected = &sessions.sessions[index - 1];
            if demo && index > 3 {
                warning(&mut mei, "notindemo", position, "🔒️ DEMO", 50., false);
            } else if !is_session_ok_to_load(selected.id, &auto_save.unsaved_files).is_ok() {
                warning(&mut mei, "couldnt_load", position, "⚠", 200., true);
            }
            if !demo && !input_disabled {
                contextual(
                    &mut mei,
                    position,
                    index,
                    state,
                    &sessions.sessions,
                    navigation_boundary,
                );
            }
            let path = selected.id.save_folder_path().join("history.json");
            display_error(&mut mei, &mut local, &path, position, true);
        }
        if hovered.is_some() && hovered != previous_hover {
            sounds.write(PlayMenuUiSound { kind: 0 });
        }
    }
    // No early click return: original still creates this frame's overlays.
    if let Some(id) = state.warning_session {
        let modal = mei
            .fullscreen("demo save warning")
            .template("menu/delete_warning");
        modal.with_slot("answer", |slot| {
            if button(&slot, &lang_lib::lookup("Continue.Nope"), true)
                .node()
                .used()
            {
                state.warning_session = None;
            }
            if button(&slot, &lang_lib::lookup("Continue.Yep"), false)
                .node()
                .used()
            {
                deletes.write(DeleteSessionCmd { session: id });
                state.warning_session = None;
            }
        });
    }
    if let Some(id) = state.switch_glade_type {
        let metadata = meta(id);
        let modal = mei
            .fullscreen("switch glade type menu")
            .template("menu/switch_glade");
        modal.with_slot("answer", |slot| {
            for (theme, key) in [
                (GladeType::Summer, "New.Glade.Summer"),
                (GladeType::Autumn, "New.Glade.Autumn"),
                (GladeType::Olden, "New.Glade.Olden"),
                (GladeType::Flowery, "New.Glade.Flowery"),
                (GladeType::Winter, "New.Glade.Winter"),
            ] {
                let label = lang_lib::lookup(key);
                let label = if metadata.glade == theme {
                    format!("-{label}-")
                } else {
                    label.to_string()
                };
                let node = button(&slot, &label, false);
                if metadata.glade == theme {
                    node.content_mut::<MenuButtonContent>().color = 0xc7b7a7ff;
                }
                if node.node().used() {
                    state.switch_glade_type = None;
                    let mut updated = metadata.clone();
                    updated.glade = theme;
                    let path = id.save_folder_path().join("history.json");
                    match parse(&path, &updated) {
                        Ok(data) => {
                            let _ = updated.write_to_disk(id);
                            queue_load(id, data, &mut commands, &mut next_loading, &mut next);
                            sounds.write(PlayMenuUiSound { kind: 8 });
                        }
                        Err(error) => local.set_error(path, error),
                    }
                }
            }
            if button(&slot, &lang_lib::lookup("Continue.Nope"), true)
                .node()
                .used()
            {
                state.switch_glade_type = None;
            }
        });
    }
    if let Some(backup_state) = &mut state.backups {
        let close = backup_overlay(
            &mut mei,
            backup_state,
            &mut local,
            history.identity(),
            &auto_save,
            &mut commands,
            &mut next_loading,
            &mut next,
        );
        if close {
            state.backups = None;
        }
    }
    if !input_disabled {
        let back = mei.safe_frame("back", false).template("menu/back");
        back.with_slot("button", |slot| {
            let node = slot.pill_button("menu/back", "menu/back");
            node.content_mut::<PillButtonContent>().action = 18;
            node.node_mut().set_sounds(0, 9);
            if node.node().used() {
                next.set(EscMenuState::Main);
            }
        });
    }
}

fn button(parent: &NodeContext, text: &str, cancel: bool) -> NodeContext {
    let node = parent.menu_button(text);
    let content = node.content_mut::<MenuButtonContent>();
    content.color = PAPER_TINT;
    if cancel {
        content.action = 18;
    }
    node
}
fn contextual(
    mei: &mut MeiContext,
    position: Vec2,
    index: usize,
    state: &mut LoadMenuState,
    sessions: &[country_core::resources::session::SavedSession],
    boundary: bool,
) {
    let root = mei.new_at_pos(
        "delete",
        position + Vec2::new(0., 250.),
        Some(Vec2::new(200., 30.)),
        Vec2::new(0., -1.),
        false,
    );
    let hovered = root.node().state_bits & 1 != 0;
    let view = root.template("other/hint_text_centered");
    view.with_slot("text", |slot| {
        let size = if hovered { 65. } else { 60. };
        let id = sessions[index - 1].id;
        if index != 1 {
            let b = slot.pill_button("delete_button", "menu/delete");
            let c = b.content_mut::<PillButtonContent>();
            c.strength = 0.6;
            b.node_mut().flags |= 0x10;
            // Native bit8 requests default focus. It does NOT disable input.
            if boundary {
                b.node_mut().flags |= 0x18;
            }
            b.node_mut().style.size = Length::absolute(size);
            b.node_mut().style.min = Length::absolute(size);
            if b.node().used() {
                state.warning_session = Some(id);
            }
        }
        let b = slot.pill_button("switch_glade", "menu/switch_glade");
        b.content_mut::<PillButtonContent>().strength = 0.6;
        b.node_mut().flags |= 0x10;
        if index == 1 && boundary {
            b.node_mut().flags |= 0x18;
        }
        b.node_mut().style.size = Length::absolute(size);
        b.node_mut().style.min = Length::absolute(size);
        if b.node().used() {
            state.switch_glade_type = Some(id);
        }
        let b = slot.pill_button("view_backups", "menu/glade_backups");
        b.content_mut::<PillButtonContent>().strength = 0.6;
        b.node_mut().flags |= 0x10;
        b.node_mut().style.size = Length::absolute(size);
        b.node_mut().style.min = Length::absolute(size);
        if b.node().used() {
            state.backups = Some(LoadMenuBackupState {
                session: id,
                page: 0,
            });
        }
    });
}
fn label(
    parent: &NodeContext,
    _name: &str,
    text: String,
    metrics: f32,
    weight: u16,
    color: u32,
) -> NodeContext {
    let node = parent.label(text.clone());
    let content = node.content_mut::<LabelContent>();
    content.set_text(text);
    content.color = color;
    content
        .text
        .set_metrics(metrics)
        .expect("original label metrics");
    if weight != 0 {
        content.text.set_weight(tg_text::Weight(weight));
    }
    node
}
fn warning(
    mei: &mut MeiContext,
    name: &str,
    position: Vec2,
    text: &str,
    metrics: f32,
    _warning: bool,
) {
    let view = mei
        .new_at_pos(
            name,
            position + Vec2::new(-100., 0.),
            Some(Vec2::new(200., 30.)),
            Vec2::splat(-1.),
            false,
        )
        .template("other/hint_text_centered");
    view.with_slot("text", |slot| {
        let node = label(&slot, name, text.to_string(), metrics, 800, u32::MAX);
        node.node_mut().flags |= 2;
    });
}
fn display_error(
    mei: &mut MeiContext,
    local: &mut LocalState,
    path: &Path,
    position: Vec2,
    row_hovered: bool,
) {
    let Some(error) = local.error.as_mut().filter(|error| error.path == path) else {
        return;
    };
    let now = Instant::now();
    if error.expires <= now {
        local.error = None;
        return;
    }
    let frame = mei.new_at_pos("loading error", position, None, Vec2::ZERO, false);
    draw_error(&frame, &error.message);
    if row_hovered || frame.node().state_bits & 1 != 0 {
        error.expires = error.expires.max(now + Duration::from_millis(200));
    }
}

fn draw_error(parent: &NodeContext, message: &str) {
    let view = parent.template("menu/backup_load_error");
    view.with_slot("header", |slot| {
        label(
            &slot,
            "header",
            format!("⚠️ {}", lang_lib::lookup("New.ErrorLoadingSave")),
            48.,
            0,
            u32::MAX,
        );
    });
    view.with_slot("error", |slot| {
        let node = label(
            &slot,
            "error",
            format!("Context: {message}"),
            28.,
            0,
            0xffffffe0,
        );
        let text = &mut node.content_mut::<LabelContent>().text;
        text.set_max_width(700.)
            .expect("native backup error max-width");
        text.set_align(tg_text::Align::Left);
    });
}
fn backup_label(backup: &BackupSaveFile) -> String {
    static FORMAT: OnceLock<Vec<time::format_description::BorrowedFormatItem<'static>>> =
        OnceLock::new();
    let description = FORMAT.get_or_init(|| {
        time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")
            .expect("native static date format")
    });
    let date = utils::local_time::system_time_to_local_time(backup.modified)
        .format(description)
        .expect("original backup date formatter unwrap");
    date
}

#[allow(clippy::too_many_arguments)]
fn backup_overlay(
    mei: &mut MeiContext,
    state: &mut LoadMenuBackupState,
    local: &mut LocalState,
    identity: HistoryIdentity,
    auto_save: &AutoSaveState,
    commands: &mut Commands,
    next_loading: &mut NextState<AppLoadingState>,
    next: &mut NextEscMenuState,
) -> bool {
    let mut backups: Vec<_> =
        SaveBackupList::iter_backup_files(&state.session.backup_folder_path())
            .into_iter()
            .flatten()
            .collect();
    // Native comparator ignores subsecond modified precision; stable sorting.
    backups.sort_by_key(|b| {
        std::cmp::Reverse(time::OffsetDateTime::from(b.modified).unix_timestamp())
    });
    let maximum = backups.len() >> 3;
    state.page = state.page.min(maximum);
    let root = mei
        .fullscreen("glade backups menu")
        .template("menu/load_backup");
    root.with_slot("header", |slot| {
        label(
            &slot,
            "backup_header",
            lang_lib::lookup("New.LoadGladeBackup").to_string(),
            48.,
            0,
            PAPER_TINT,
        );
    });
    let mut close = false;
    root.with_slot("list", |slot| {
        slot.node_mut().flags |= 4;
        let start = state.page * 8;
        for backup in &backups[start..(start + 8).min(backups.len())] {
            let text = backup_label(backup);
            let row = slot.child(&text);
            row.layout(LayoutStyle {
                gap: Length::absolute(10.),
                swizzle: 4,
                ..Default::default()
            });
            let node = button(&row, &text, false);
            let row_hovered = row.node().state_bits & 1 != 0;
            if node.node().used() {
                match restore_backup(state.session, backup, &backups, identity, auto_save) {
                    Ok(Some(data)) => {
                        close = true;
                        queue_load(state.session, data, commands, next_loading, next);
                        // Native backup closure does not emit a menu sound.
                    }
                    Ok(None) => {}
                    Err((path, message)) => local.set_error(path, message),
                }
            }
            if let Some(error) = local.error.as_mut().filter(|e| e.path == backup.path) {
                let now = Instant::now();
                if error.expires <= now {
                    local.error = None;
                } else {
                    let frame = mei.new_relative(
                        "save load error",
                        row.id,
                        None,
                        Vec2::new(0., -20.),
                        Vec2::new(0., 1.),
                        Vec2::new(-0., -1.),
                        true,
                    );
                    frame.node_mut().local_z = 100.;
                    if row_hovered || frame.node().state_bits & 1 != 0 {
                        error.expires = error.expires.max(now + Duration::from_millis(200));
                    }
                    draw_error(&frame, &error.message);
                }
            }
            let icon = match backup.kind {
                BackupSaveFileKind::Auto => "⌚️",
                BackupSaveFileKind::Manual => "",
                BackupSaveFileKind::Recovered => "💥",
                BackupSaveFileKind::Other => "🦕",
                BackupSaveFileKind::Corrupted => "🤕",
            };
            label(&row, "kind", icon.to_string(), 32., 0, u32::MAX);
        }
    });
    root.with_slot("footer", |slot| {
        if maximum != 0 {
            let pagination = slot.child("pagination");
            pagination.layout(LayoutStyle {
                gap: Length::absolute(10.),
                size: Length::absolute(120.),
                swizzle: 4,
                justify: Justify(1),
                ..Default::default()
            });
            // Exact native order: decrement, dynamic label, increment.
            if button(&pagination, "←", false).node().used() {
                state.page = state.page.saturating_sub(1);
            }
            let page_label = format!("{}/{}", state.page + 1, maximum + 1);
            label(
                &pagination,
                &page_label,
                page_label.clone(),
                48.,
                700,
                PAPER_TINT,
            );
            if button(&pagination, "→", false).node().used() && state.page < maximum {
                state.page += 1;
            }
        }
        if button(&slot, &lang_lib::lookup("Cancel"), true)
            .node()
            .used()
        {
            close = true;
        }
    });
    close
}

fn restore_backup(
    id: SessionId,
    backup: &BackupSaveFile,
    all: &[BackupSaveFile],
    identity: HistoryIdentity,
    auto_save: &AutoSaveState,
) -> Result<Option<ParsedGameData>, (PathBuf, String)> {
    let parsed = match parse(&backup.path, &meta(id)) {
        Ok(data) => data,
        Err(error) => {
            log::error!("Error parsing save backup: {error}");
            let name = backup
                .path
                .file_name()
                .expect("original backup file name")
                .to_str()
                .expect("original backup filename UTF8");
            let path = if name.starts_with("corrupted-") {
                backup.path.clone()
            } else {
                let path = backup
                    .path
                    .parent()
                    .expect("original backup parent")
                    .join(format!("corrupted-{name}"));
                let _ = std::fs::rename(&backup.path, &path);
                path
            };
            return Err((path, error.to_string()));
        }
    };
    let target = id.backup_folder_path().join(format!(
        "history-manual-{}.json",
        utils::file::date_time_string_for_filename()
    ));
    let duplicate = all.iter().filter(|b| !b.kind.is_corrupted()).any(|other| {
        let Ok(mut file) = File::open(&other.path) else {
            return false;
        };
        let other_identity = match try_approx_parse_save_file_identity(&mut file) {
            Ok(Some(id)) => id,
            Ok(None) => derive_fallback_save_file_identity(&other.path),
            Err(error) => {
                log::error!(
                    "Backup {} couldn't parse identity: {error}",
                    other.path.display()
                );
                derive_fallback_save_file_identity(&other.path)
            }
        };
        other_identity == identity
    });
    if duplicate {
        log::info!("Current history already has an equivalent backup; skipping duplicate backup");
    } else if let Err(error) = auto_save.create_backup_of(target) {
        log::error!("Error copying history file to backup: {error}");
        // Native does not create a SaveLoadError or queue a load on this branch.
        return Ok(None);
    }
    Ok(Some(parsed))
}
