# Integration owner only

registration-only.patch is a five-init additive hunk for the existing register_history_events. It is not applied here because the original history/mod file family is owned by main-chain-boot-20261018. init_resource preserves already-inserted real input resources. Both readers reuse Events<HistoryCommand>; the old ReplayFocusEvent may remain for compatibility but no longer carries production replay commands.

AppModeChanged must be produced after the real mode transition and before replay_mode reads it. Existing backend/mode-manager owns send_mode_just_changed_event + PreviousAppMode. Do not add a second independent mode state. This patch does not assert original global schedule edges or wire the missing sheep/signifier consumers.

Base history.rs SHA-256: 8f8db712c90f526f3a4856a759c53b3447d063763362ad70e11843030095574b
