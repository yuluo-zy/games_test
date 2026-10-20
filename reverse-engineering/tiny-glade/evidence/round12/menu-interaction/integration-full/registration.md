# Load UI production registration contract

Original B MenuPlugin::build14068dee6 passes in_state(Game=1) to helper140ff4d40;14068e29e supplies GameSystemSet36. Restore `toolbar::menu::load_ui::ui.in_set(GameSystemSet(36)).run_if(in_state(AppLoadingState::Game))` in the actual gameplay App. This is separate from Escape GUI39 and follows MEI pre6/before post44. Do not add a minimal-world fallback or remove the real systems to make checks pass.

Required canonical owners, in original12-param order: Res<ActiveSession>, Res<History>, Res<AutoSaveState>, ResMut<EscMenuState>, ResMut<NextEscMenuState>, ResMut<toolbar::MeiContext>, ResMut<NextState<AppLoadingState>>, Commands, Res<SavedSessionList>, Local<toolbar::menu::load_ui::LocalState>, EventWriter<PlayMenuUiSound>, EventWriter<DeleteSessionCmd>. The source-level13th parameter is canonical per-App Res<GameConfig> for the original demo-mode global. Do not install a second session list/history/menu/Time/scroll metrics resource.

Successful parses transfer the real ParsedGameData owner through NextSession::from_parsed, then Reload2, NextClosed; the startup/session owner must consume that same Box without reopening history.json. The original UI parse branch does not issue path-upgrade backup a second time. All parser migrations and four wrapper flags belong to the existing save owner.

Retain the same LoadMenuState.thumbnail_metrics Arc used by ThumbnailScrollContent and T11. The actual TextureHost must load menu/delete, menu/switch_glade, menu/glade_backups and menu/paper-bg in addition to existing T11 assets, from the original installation. Post44 real text/shape rendering consumes the nonempty installed templates and owning MenuButton/Pill/Label contents.

DeleteSessionCmd must reach the canonical delete_session_system (last unread only), then canonical refresh_session_list_system on DeleteSessionCmd or SavingGameComplete. Files are renamed to deleted-saves with collision reroll; do not substitute recursive deletion. Source already has these consumers; central registration must preserve their order relative to the new producer.

Toolbar Cargo needs existing locked direct edges anyhow.workspace=true, utils.workspace=true, time.workspace=true and bevy_state0.16.0 with the same std/bevy_app features as tiny-glade. No package identity/version changes. AutoSaveState::create_backup_of receiver was changed to&self only after coordination with save_migration_full; send/reply semantics are untouched.

Native6 also enters the original gate; its variant/producer remains unresolved. Only canonical Load0/1 is represented. Do not count that unknown branch or complete original GUI runtime as passed. All source changes remain pending central compile/independent real input/session/save validation. T13 is excluded and unrelated Load functionality is preserved.
