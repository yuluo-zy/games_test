# Load UI restoration handoff (read-only batch, 2026-10-19)

This batch does **not** modify production source or run Cargo. T12 is already integrated; T13 is excluded by the user. T11 thumbnail decode/upload/refresh/card consumer exists, but the actual full Load interaction remains unimplemented. Do not count the renderer host as the Load GUI being complete.

## Evidence identity and navigation

`identity.json` binds every bounded function byte range to the original unmodified RNE (`workspace/input/tiny-glade.exe`, SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`). `extract.py` generates bounded LLVM disassembly, `call-targets.json`, and `rip-data-windows.json`. Calls resolved by a single PDB name can be ICF aliases: e.g. `140a9fa70` is the readiness `deny_load` predicate despite the first PDB symbol being a graphics predicate. Do not trust the unrelated first name as the business operation.

- `load-ui.asm`: **15632 bytes**, `141e1fe80` original `toolbar::menu::load::ui`; actual function includes exception cleanup through `141e23b90`. Old preserved decomp header says 12354 bytes and `void()`: neither is the true function/ABI boundary.
- `load-system.asm`: 2245 bytes, `141e11780`, complete 12-system-parameter specialization.
- `contextual.asm` / `contextual-slot.asm`: `141e23d20` 451B / `140fc6be0` 2267B.
- `backup-list-slot.asm`: `140fc3840`, 11861B; contains the full recovery filesystem transaction, not just rows.
- `backup-footer-slot.asm`: `140fc66a0`, 844B; delegates arrow/page contents into `140f9b5c0` (nested closure remains to decode).
- `back-slot.asm`: `140fb8600`, 1858B, inlined closure9 plus template selection.
- `readiness-warning-slot.asm` / `demo-warning-slot.asm`: `140fb9a80` / `140fbe1a0`, 1679B each.
- `backup-header-slot.asm`: `140fbb410`, 1068B.
- `display-error.asm` / `error-title-slot.asm` / `display-error-slot.asm`: `141e23b90`110B / `140fbc5b0`1321B / `140fc69f0`484B.
- `load-meta.asm`: `141e23c00`, **273B**, including cleanup; preserved decomp says156B.
- `backup-sort-comparator.asm`: `140fe41e0`203B; descending modified-time seconds; do not reverse std sort itself.
- `load-ui-nonempty.txt`: old retained main decomp with blank lines removed (2435 lines); convenient only for navigation; constants and ownership conclusions below come from native code.
- Original decomp: `tool-usage/jobs/toolbar-c/decomp/0x141e1fe80-ui.txt`, same directory has load-meta, display-error, contextual.
- Original UI templates: `ljxsj_92385/Tiny Glade/assets/ui/menu/{delete_warning,switch_glade,load_backup,backup_load_error,back}.ron` and `assets/ui/other/hint_text_centered.ron`.
- Original plugin registration: `../menu-build.asm`.

## Exact system contract and registration

Original order:

```rust
fn ui(
    active: Res<ActiveSession>,
    history: Res<History>,
    auto_save: Res<AutoSaveState>,
    current: ResMut<EscMenuState>,
    next_menu: ResMut<NextEscMenuState>,
    mei: ResMut<MeiContext>,
    next_loading: ResMut<NextState<AppLoadingState>>,
    commands: Commands,
    sessions: Res<SavedSessionList>,
    local: Local<toolbar::menu::load::LocalState>,
    sounds: EventWriter<PlayMenuUiSound>,
    deletes: EventWriter<DeleteSessionCmd>,
)
```

PDB run_if specialization `140ff4d40` contains that exact signature. MenuPlugin::build calls it at `14068dee8` with `dl=1` at `14068dee6`: `in_state(AppLoadingState::Game)`. This menu tuple joins set **36**: `14068e29e mov r8b,0x24`, `14068e2a1 call in_set`. It belongs with the existing T12 UI set36 after MEI pre6, before MEI post44; this is native registration proof, not a guessed set.

Main UI early gate at `141e20128..13f`: wrapping `(native_tag-2)>=9 || (native_tag-2)==4`, accepting Load niches0/1 and native6. Canonical safe `EscMenuState::Load` should be used; native6 naming has not been established, so don't add a guessed enum variant.

## State and shared ownership

Canonical `country-core/resources/gui.rs` already provides `LoadMenuState`: hovered Option<usize> at native0, backups Option(session,page) at10, shared thumbnail metrics Arc at30, delete-confirm session Option at38/39, switch-theme session Option at49/4a. Shared widget is already `toolbar/menu/load.rs::ThumbnailScrollContent`. **Keep its same Arc** so frame metrics and card rendering see actual user movement.

Native LocalState is an optional owned SaveLoadError. Layout observation: path at0..20, String at20..38, Instant at38..48 (Windows PathBuf owns a 32B WTF8 representation). Use typed `PathBuf`, `String`, `Instant`, not raw layout/u64. Local persists across leaving/reopening menu; drop/replace previous owned values exactly.

## Full main tree and interaction algorithm

1. Fullscreen root ID **`"menu screen"`**, then thumbnail_scroll ID **`"builds"`**, count `sessions.len()+1`, current state's metrics Arc (`141e20264..299`). Slot0 is new glade; slots1..N are sessions0..N-1.
2. `input_disabled = warning_session.is_some() || switch_glade_type.is_some() || backups.is_some()` (`141e20246..256`). Root scroll gets default-navigation flag4 when enabled. If scroll content not initialized, set its initial scroll to `1 + gap/cell_size`, centering saved-slot1 (`141e203e5..402`). Set scroll_limits Some(f32 bits **3e851eb8**, .26), navigation speed **10**, and content input_disabled. Keep complete wheel/drag/inertia/bounds widget already restored.
3. If neither delete nor switch modal: remember previous hover and widget navigation_boundary; update hovered from content.hovered filtered `index<=sessions.len()`; click is filtered identically. Slot0 sets next menu **NewSession** and sound **7**. Session click allows first3 saved slots in demo, all slots outside demo. Recompute readiness from actual `auto_save.unsaved_files`; if deny_load(tag1), or demo restriction, sound **12** and keep menu.
4. Accepted clicked current active ID: no disk parse/new loading request; still close menu and emit sound8. Different ID: load SessionMeta from selected folder/meta.json with `unwrap_or_default`, `metadata.assets_root()` -> `GladeSettings::load`, selected history.json -> **parse_saved_game_data(path,&settings)**. On success transfer the real parsed owner into NextSession variant3, set `NextState<AppLoadingState>::Loading`, close menu, sound **8** (`141e20e88`). On parse error store path, formatted error, `now+2s`, stay in current menu.
5. Read hovered save coordinates from widget, only index>0 and valid list index. Recompute readiness; !is_ok displays warning node `"couldnt_load"`, `other/hint_text_centered` template, `"text"` slot. Its label is **U+26A0 WARNING SIGN** (3 UTF8 bytes), metrics200, weight800, flag2; not an ASCII ellipsis (`exact-data.json`). Demo-disallowed hovered save instead node `"notindemo"`, same template/slot, literal **U+1F512 U+FE0F + ` DEMO`**, metrics50. Preserve native position/anchor/size (main asm208af..20925, demo207xx).
6. Outside demo and with input enabled, contextual row is visible. Contextual arguments include hovered index, complete session slice, mutable load state, `hovered_index!=1` (native bool at call stack38), and widget.navigation_boundary (call stack40); **not** generic assumed always-enabled buttons. Extract exact disabled/navigation flag mapping from contextual-slot.asm before coding.
7. For hovered save whose history path equals LocalState.error.path: if expiry>now, `new_at_pos("loading error", hovered coords)` and display full error. Hover extends to `max(old_expiry,now+200ms)` (`141e210c3=200000000ns`), **not +1s**. Expired value is dropped and cleared. Path comparison is typed `Path` component equality, not string hash.
8. When hover changes from prior Some/different to new Some, emit sound **0** (`141e211ce`). None does not produce hover sound.
9. Continue constructing other overlays after queuing transitions, as original does. Do not early-return after click if it suppresses this frame's other original effects.

### Contextual row

`show_contextual_buttons` creates root **`"delete"`** at `(hover.x,hover.y+250)`, anchor `(0,-1)`, size `(200,30)` from original same `other/hint_text_centered` template/textslot. Retained closure does **three** PillButtons:

- `"delete_button"`, image **`menu/delete`**; click sets `warning_session=Some(selected.id)` (`140fc72d5..731c`).
- `"switch_glade"`, image **`menu/switch_glade`**; click sets `switch_glade_type=Some(selected.id)` (`140fc7164..71a7`).
- `"view_backups"`, image **`menu/glade_backups`**; click sets backups=Some{session:selected.id,page:0} (`140fc7430..7478`).

Each content's strength is **.6**, exact button size is **65 when parent hover, 60 otherwise** from constants142bb31c8. Contextual native code applies navigation/disabled flag bits10/18 before size; these depend on the two caller booleans, and must not be collapsed. PillButton click test is state&0x30. Explicitly load these original compiled image textures into the App host: common-assets whitelist does not include them.

### Delete confirmation

If warning_session Some: fullscreen modal root **`"demo save warning"`** (odd native stable ID), template **`menu/delete_warning`**, slot **`answer`**. RON preserves paper-bg, 550x300, heading localized `Menu.Settings.DeleteGlade`, horizontal answer gap100. Add localized buttons `Continue.Nope`, `Continue.Yep`, tint **7b5a51a0**. No has digital action18. State&0x30 No clears only warning; Yes publishes **DeleteSessionCmd(id)** then clears warning (`141e21710..217d4`). Do not remove save files directly from UI.

Existing country-core owner receives `EventReader<DeleteSessionCmd>`, consumes **read().last()** only, then source-folder->paths::deleted_saves()/uuid via rename, rerolls ID if destination exists, logs failure. Session refresh is event-gated with DeleteSessionCmd or SavingGameComplete. Keep this producer→existing consumer→refresh chain and its ordering; no `remove_dir_all` substitute.

### Theme switch confirmation

If switch_glade_type Some: fullscreen ID **`"switch glade type menu"`**, template **`menu/switch_glade`**, answer slot. Metadata `load_session_meta(id).unwrap_or_default()`. Options original order/value: **Summer0, Autumn3, Olden1, Flowery2, Winter4**. Labels `New.Glade.Summer`, `New.Glade.Autumn`, `New.Glade.Olden`, `New.Glade.Flowery`, `New.Glade.Winter`. Existing selected theme is bracketed with native `-X-` formatter and tinted **c7b7a7ff** instead of **7b5a51a0**.

Click clears switch modal **before** disk parse (`141e21f84`). Obtain chosen theme settings, parse selected history with &settings. Failure stores same typed error+2s and leaves load root open. Success clones SessionMeta fields, replaces its glade, `write_to_disk(id)` (original drops write error), queues owned parsed NextSession variant3, Loading, Closed and sound8. Final `Continue.Nope`/action18 only clears switch modal. Preserve each original metadata field; don't rebuild default meta with only theme.

### Backups overlay and restore transaction

If backups Some: fullscreen **`"glade backups menu"`**, template **`menu/load_backup`**. RON paper-bg550x300, padding80/80/70/50, gap30; slots header/list/footer. Header localized **`New.LoadGladeBackup`** with tint7b5a51a0. `SaveBackupList::iter_backup_files(session.backup_folder_path()).into_iter().flatten().collect()`; read_dir error is dropped -> empty list. Stable sort **descending modified seconds** (`backup-sort-comparator.asm`).

Page maximum is **len >> 3** (floor len/8); clamp current page to that maximum, list slice `[page*8..min(page*8+8,len)]`. This permits an empty final page when len is an exact multiple of8: preserve original, do not silently change ceil arithmetic. Footer `"pagination"` horizontal child (gap10, size120 field), nested arrows `>` and `<`, cancel localized **`Cancel`**, action18/tint. Cancel sets local close bool, main clears backups after all slots. Arrow implementation `140f9b5c0` still requires targeted decode; do not substitute an unverified page policy.

Every backup row stable ID derives from its formatted **local** modified timestamp, horizontal layout gap10. Button tint7b5a51a0. Label kind tag0 U+231A U+FE0F, tag1 empty, tag2 U+1F4A5, tag3 U+1F995, tag4 U+1F915, plus date label. Jump table proven in exact-data.json. `utils::local_time::system_time_to_local_time` + locked `time=0.3.36` formatting library; **reuse library**. Format description native table pointer142bb3f08/count11 at142bb4010; BorrowedFormatItem stride24; exact modifier decoding still pending (do not claim a guessed format). Date formatter unwrap is original panic boundary.

Click restoration (`backup-list-slot.asm140fc3e8b..4f4b`):

1. Build selected session current history.json. Load meta/default->assets_root->GladeSettings; parse **chosen backup path with settings**.
2. **Parse failure branch** logs `Error parsing save backup: ...`; selected filename/UTF8 and parent are expected/unwrap boundaries. If filename already starts `corrupted-`, do not prefix again; otherwise try rename to sibling `corrupted-{oldfilename}`. Rename failure is dropped. Store SaveLoadError for correct selected/new path and original parse error, expiry+2s. No loading request. Do not confuse this error branch with the successful backup-current transaction.
3. **Parse success branch** retains parsed owner while protecting current history. Construct target `session.backup_folder_path()/history-manual-{date_time_string_for_filename()}.json`.
4. Iterate all listed backups; skip corrupt(kind4, aliased leaf140ae8320), attempt open each; skip unreadable files. Try approx parse SaveFileIdentity; on parse error log `Backup ... couldn't parse identity: ...`, derive fallback from path. Compare to **History.identity at native+60**. If equivalent backup exists, log skip message and do not create duplicate.
5. Without equivalent backup call **AutoSaveState::create_backup_of(target)** synchronously at140fc4870. Existing API currently takes &mut self while actual UI system param is **Res<AutoSaveState>**. The restore producer must share the existing save-thread sender; fix owner API to &self if no state mutation, not switch UI param to guessed ResMut or use direct file copy. If backup fails: log `Error copying history file to backup: ...`, drop parsed owner and **abort request**. No loading transition.
6. Success/equivalent-existing: close backups via shared local close flag, transfer parsed owner as NextSession variant3 with same selected SessionId, set Loading, close main menu. Preserve ignore_snapshot=false and empty on_load exactly. This branch has no explicit menu sound write in original closure7.
7. Local error display binds matching backup path, 2s expiry, hover(row **or** errorframe) extends max+200ms, style min-width100, template **`menu/backup_load_error`**, slots **`header`**, **`error`**. Header localized `New.ErrorLoadingSave` formatted `Context: ...`; actual error String is shown separately, tintffffffe0, align native0, max-width700, metrics28. Drop/clear expired owned error.

### Main back button

Only if no input-disabling modal from the frame's initial state: safe-frame **`"back"`**, template **`menu/back`**, `"button"` slot, PillButton **`menu/back`**, action18 and native sound word0900. Click returns **Main(tag3)** (`140fb8d01`), unlike LUT gallery which closes. Do not reuse LUT's target state blindly.

## Critical cross-crate contracts for session_owner

**Parsed NextSession variant3 is missing in current source.** Current `NextSessionKind::{None,LoadSavedGame,LoadOnLoad}` values0/1/2 and raw `[u64;9]` payload cannot own UI data. Native producer:

```
141e20bd8 parse_saved_game_data -> return at rbp+25e0 (16B tagged value)
141e20bdd cmp byte [rbp+25ef],2 ; Err discriminator
141e20d5c movups [rbp+25e0] -> [rbp+1d10] ; NextSession+30
141e20d80 movdqu selected ID -> [rbp+1d00] ; +20
141e20dc6 movabs 0x8000000000000003
141e20dd0 mov [rbp+1cf8],rax ; +18 kind
141e20e1a Commands.insert_resource<NextSession>
```

Backup restore same contract: owner->rbp190 at140fc4cc3, id->180 at4cd5, variant3->178 at4d1e..28, insert4d6f. Theme switch also variant3 at141e22557 insertion. Do not treat temporary parser None niche at718/298 as final NextSession kind; it is initialized for safe move/drop. The 16B producer value's complete typed ownership still needs consumer proof in startup/load_game (session_owner assigned). Do not reconstruct a raw pointer payload or reload path instead of consuming it.

**Parser settings argument is missing in current source.** Native call ABI: rcx hidden return, rdx/r8 Path(str pointer,len), **r9=&GladeSettings**:

- ordinary load141e20bce lea r9,[rbp+2510], call141e20bd8;
- theme switch141e2203b lea r9,[rbp+25e0], call141e22042;
- backup140fc401d lea r9,[rbp+d0], call140fc4024.

Current `country-core/systems/save/load_game.rs::parse_saved_game_data(path)` discards this original input entirely; inspect native callee field consumption before extending safe API, then preserve all domain decoding. This batch did not reverse a general library or alter rhapsody.

## Next concentrated patch and acceptance checklist

1. session_owner restores actual typed variant3, parser settings usage, move through startup_session/load_game; no reparsing shortcut.
2. menu_interaction restores full load::ui and typed LocalState/SaveLoadError, all roots/templates/slots/flags; explicitly completes missing arrow nestedclosure and timestamp format decoding above.
3. Add shared MenuButtonContent APIs needed by original tree without overwriting independently updated MEI; reuse original font/label/shape rendering. Real button input must traverse retained MEI capture/bubble phases, not injected fake clicks.
4. Load menu compiled image assets (`menu/delete`, `menu/switch_glade`, `menu/glade_backups`, `menu/paper-bg`) in correct App physical TextureHost and real shader/array ownership. T11 already explicitly installs craft_paper/new_glade_tile/cloth_ferris/brush2/menu_back; common whitelist is insufficient.
5. Register original load UI at GPU startup tuple set36+Game with 12 resources/events actually available, keep session delete/refresh consumer order. Escape→Load entry and back Main transitions must come from real menu producer; snapshot tests alone are insufficient.
6. Tests worth adding next batch: normal/current/demo/restricted/notready/readinessdeny branches; parsed owner moves once; delete emits and existing last-event owner renames/refreshes; page boundaries0,7,8,16; restoration current-equivalent/noequivalent/failedbackup/corruptparse/renamefailure; theme meta preservation; error2s+hover200ms expiry; real GUI metrics Arc + next-frame selected session.
7. Root owns unified Cargo/GPU acceptance. This batch ran **no Cargo and no production edit**. Unknowns above are genuine remaining proof work, not completion claims; no full ready-to-apply patch is emitted because consumer/tag/format/arrow contracts must first close.
