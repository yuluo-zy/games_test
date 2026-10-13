# Paths / language / theme definitions: B source-module recovery

This is behavior reconstruction, not author source. Original game files and dependency
implementations are unchanged. Source path and named procedure inventory are recorded
in `procedures.json` and LLVM `module-*.txt`; emitted CodeView `void()` is not used as
a Rust ABI. Earlier `.c` files are temporary ReVa project evidence, not a claim that
those overlays were migrated. New persistent worker is `foundation-paths-language`;
`persistent-context.json` verifies the B patched SHA and its independent project.

## crates/paths/src/lib.rs

Original module 637 supplies `config`, `saves`, `backup_saves`, `tmp`,
`deleted_saves`, `installed`, `standard_app_root_path`,
`find_standard_app_root_path`, `initialize_paths`. Module 1834 supplies the emitted
`ensure_path<PathBuf>` and its inlined `create_dir_all`. Addresses/sizes are in the
inventory. Each is mapped to the same-named reconstruction function.

Observed: Windows KnownFolder selector 0x6c (SavedGames); failure reads USERPROFILE
as Unicode with context `Reading environment variable USERPROFILE`, then appends
`Saved Games`. Both paths append `Tiny Glade`. Explicit override also appends
`Tiny Glade`. User folder is `Steam/{u64}`. Creation is sequential:
`saves`, `beta-saves`, `tmp`, `deleted-saves`, `backup-saves`, account root. Earlier
folders remain after a later error. `config` is account root; installation is
`current_dir()`, observed after directory creation; standard root is retained
separately. Accessors panic before initialization. Reinitialization panics.

The inlined private error context of `ensure_path` is `Creating directory {path}`,
confirmed by byte-hashed persistent query of module 3260 at 0x142131410 and the
format-piece table 0x142fe66b0 -> 19 bytes at 0x142fe6698. We inspect only the game
message/composition and reuse std/anyhow instead of recovering their internals.

Synthetic boundaries: safe immutable `Paths` storage and `initialize_at` make policy
callable/testing possible. Original leaked globals/ABI are not reconstructed.
`beta_saves` helper is synthetic (B emitted no standalone getter). Other platforms
are an explicit unsupported boundary; their rules are not inferred from Windows.
Original declaration names/attributes, all non-emitted feature branches and the
global diagnostic strings are not guaranteed identical. Full application startup
is not wired, but lang-lib's real recovered persistence consumer uses the path API.

## crates/lang-lib/src/{lib.rs,language_library.rs}

This package was handed to a new owner after initial implementation; do not use the
following as its final acceptance. Original 6 root procedures and 2 LanguageLibrary
procedures are emitted; `try_get`/`locale` are PDB-confirmed inline methods.

Observed candidate policy: first YAML document, flat string -> string map, empty
document list gives empty map; map/string schema mismatch panics (unwrap), scanner
and file errors return Result. Text and locale are leaked to preserve returned
references across library replacement. Missing keys use one global mutex cache,
log on first miss, and stable `?_{key}` text. `lookup` uses selected try_get then
default library get; absent library/fallback returns input key. Uninitialized
`game_locale` returns en-US. Packs are cwd-relative `assets/lang/{locale}.yml`.
Requested failure falls back to en-US; selected library is replaced before default
fallback load. Any present TG_DISABLE_LANG_FALLBACK disables fallback replacement
without clearing the preceding value. Failed fallback load panics. `load_locale`
persists the requested value (including fallback case) in standard root's
last_used_locale; write failure only logs. `load_last_used_locale` queries standard
root afresh, uses en-US if root/file fails, and passes file text without trimming.

Candidate checks at handoff: check, 2 unit tests and all-target strict Clippy pass;
15 real B locale assets parsed, zh-CN/en-US replacement and missing locale fallback
exercised by `verify_original_assets`, plus separate disabled-fallback process.
No original native function was executed. CSV/difference/dev utilities and full
feature coverage remain unknown; no Bevy ECS startup/game UI wiring claimed.

## crates/theme-defs/src/lib.rs

B's source checksum is f792fea4a30df098d9e57558500523d4b44ef61474088eb18472da8260e481f3.
No independent theme-defs compilation module or emitted standalone procedure was
found. Genuine consumers: ThemeFetcher::run (module 259, 0x140618310, 24789 PDB
bytes) and tiny_glade::theme_fetcher_plugin::update_daily_theme (module 170,
0x1403d1750, 4247 PDB bytes). Dependency generic code only establishes payload type
ownership; it is not counted as restored business implementation.

Observed: IPI scope 0x3F72/clone 0x3F73 and eq 0x3F76 represent EventDataError
operations. Their producer distinguishes two unit states and owned text. Initial
per-event data cache is pending; unmatched active interval is the no-active-event
state; download/extraction/validation failure carries text. Variant names and
Display wording in Rust are inferred, not recovered. No serde enum/wire shape is
asserted.

Observed: ActiveEvent clone 0x3F78 and eq 0x3F7A include path ownership/eq and TWO
std::time::SystemTime equality calls (IPI 0x3F7C, parent std::time::impl$29, nesting
0x3F7E Windows time eq), then PathBuf eq 0x291C. The timestamps are NOT raw Unix
integers in this type; ThemeFetcher selects protocol intervals using signed Unix
seconds, then supplies SystemTime values. The reconstructed field names
`data_folder`, `start_time`, `end_time` are semantic names. An earlier copied byte
was not proven meaningful and was removed instead of inventing a business flag.

Synthetic helper `is_active_interval` exposes the producer's half-open comparison;
it is not an original standalone function. Clone/equality reuse std PathBuf and
SystemTime. Unknown: author's exact field and error-variant names, unused serde
forms, Display wording, exact startup/plugin transport wiring and original native
behavior acceptance. Therefore theme-defs is implemented_candidate_partial, not
fully verified. There are no invented theme defaults or fabricated theme presets.

## Validation limits

`validation.json` owns current paths/theme candidate check/test/Clippy/fmt and
direct dependency identities. Native Windows KnownFolder is exercised read-only;
the path creation fixture is confined to a process-specific temporary directory.
These checks are useful candidate and dependency tests, not original-machine
differentials. Workspace transitive lock drift is listed for the integration owner;
it is not silently hidden or repaired outside this agent's file ownership.
