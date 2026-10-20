# Escape GUI integration handoff

Root owns shared run_game and platform bootstrap. This agent did not run Cargo.

Add `crate::gui::gui_escape_menu::do_gui_escape_menu` to the actual GUI39 tuple/schedule. It reads `State<AppLoadingState>` itself for the main-menu shortcut and intentionally still builds applicable UI while Splash/Reload. Do not add a blanket Game/loading/session gate. Keep the native current menu owner until its actual advance boundary; do not make writes to Next immediately mutate Current.

The system consumes the canonical resources already installed by mode/history/save/MEI: MainActionSet, AppMode, State<AppLoadingState>, EscMenuState, PhotomodeScreen, GameConfig, GameProfile, NextEscMenuState, MeiContext, RestoreAppTo, TimelapseState, SavedSessionList. It produces real HistoryCommand, EscapeMenuJustOpened, SaveGameCmd, PlayMenuUiSound, SaveGameProfileCmd, RequestAppModeChange; deferred exit inserts the original ShutdownGame resource. No parallel resource is introduced.

Install the actual SteamOverlayOpenResources owner with the existing SteamClientResource.client and browser callback; forward its successful browser-open command to the canonical OS owner. A default owner with no injected opener is an explicit missing platform chain, not a completed link button. Original dependency webbrowser is already locked; root must verify exact original version and integrate its host dependency centrally. UI does not itself open any URL until the real widget is used.

`original_escape_menu.rs` has four bounded, nonempty installed-template candidates and has not been run: actual keyboard27 replay/save/capture events, canonical Running/Restoring interruption and Photo preservation, actual MEI pointer Settings and deferred Quit, actual cancel18 Back. Root must compile/run and independently judge these. Tests do not claim full original runtime or GPU behavior. A real App screenshot opportunity still independently validates consumer behavior.
