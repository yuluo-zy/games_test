# paths / lang-lib / tabloid product repairs

These modules now use independent product behavior. Historical native matching
reports do not validate the changed APIs or lifecycle policies. Existing shared
workspace/member/lock changes made by other work are not part of this repair.

## API migration

- `paths::initialize_paths` returns an error on repeated initialization before
  touching another directory. `try_paths` returns an initialization error;
  convenience getters still require initialization. Relative input paths become
  absolute. Empty explicit paths are errors. `installed` uses the executable's
  parent directory. Overrides remain parent directories with `Tiny Glade` appended.
- Platform defaults are Windows Saved Games (then USERPROFILE/Saved Games),
  macOS HOME/Library/Application Support, and Linux absolute XDG_DATA_HOME
  (otherwise HOME/.local/share), each with `Tiny Glade` appended. File paths may
  contain non-Unicode characters. `preference_root_path` honors an initialized
  override and otherwise resolves the platform default.
- `lang_lib::lookup`, `game_locale`, and library getters return `Arc<str>`; use
  `value.as_ref()` for a borrowed view and `value.to_string()` at String APIs.
  `try_get` returns `Option<Arc<str>>`; constructor entries use owned `Arc<str>`.
  `lang_pack_path` now returns `Result<PathBuf>` and rejects invalid identifiers.
- Call `lang_lib::configure(absolute_asset_directory, fallback_enabled)` before
  the first load or path query to explicitly configure the global service.
  Otherwise the first such call captures cwd/assets/lang and whether
  TG_DISABLE_LANG_FALLBACK is absent. Later environment changes do not reconfigure
  the service. Pure lookup before loading returns the key without capturing cwd.
  Independent `Localization` instances support injected assets and isolated tests.
- Language files accept an empty file or one string-to-string mapping. Multi-document
  and schema errors return errors. Requested-pack failure can load en-US; failure
  to prepare the complete primary/fallback snapshot preserves the old snapshot.
  Missing markers are owned and cached per snapshot, capped at 1024 keys; excess
  keys are not retained or logged. Returned text survives reload only as long as
  callers retain it. Snapshot/library/cache ownership is normally reclaimed.
- `Localization::load_locale` does not save a preference. `load_locale_and_save`
  accepts its preference directory; the global `load_locale` uses the paths root.
  Failure to resolve a preference directory also leaves loading available; a
  preference read without an available directory loads the default language.
  Save failure logs after releasing the loading lock and does not roll back a
  successfully loaded language. Preferences use a same-directory create-new temp
  file, file synchronization, and rename replacement. This is not a guarantee
  against every power-loss scenario. Reads trim and validate the stored identifier.
- `tabloid::State` requires independent `Clone` semantics. `publish` returns
  `Result<(), PublishError>`; callers must handle sequence exhaustion. Taken
  `PublicationReader`s are immutable Arc snapshots carrying no read lock. Cloned
  subscribers still share one consumption boundary. Late subscribers receive
  only future changes, and pending subscribers accumulate subsequent deltas.
  Publication clones each distinct pending batch once per publish and applies
  the delta once per distinct batch. Large, growing pending states therefore have
  a copy cost; use bounded/compact states and consume batches promptly.
  State construction/Clone/apply_to must not panic or reenter the channel while
  subscription locks are held. Subscribers are weakly registered, so dropping the
  last external subscriber immediately releases its pending state.

## Validation

The shared workspace currently has other work changing membership/dependencies.
Use the disposable verification workspace for reproducible checks including the
actual GUI package; its first check resolves its own temporary lock. The script
copies current source and seeds that lock from the shared lock, and never edits
production source or locks. Its manifest is limited to these four packages.

```sh
TG_VERIFY_DIR=$(mktemp -d)
python3 scripts/prepare_foundation_workspace.py "$TG_VERIFY_DIR"
cargo check --manifest-path "$TG_VERIFY_DIR/Cargo.toml" -p tg-iced --all-targets
cargo test --locked --manifest-path "$TG_VERIFY_DIR/Cargo.toml" -p paths -p lang-lib -p tabloid --all-targets
cargo clippy --locked --manifest-path "$TG_VERIFY_DIR/Cargo.toml" -p paths -p lang-lib -p tabloid --all-targets -- -D warnings
cargo fmt --manifest-path "$TG_VERIFY_DIR/Cargo.toml" -p paths -p lang-lib -p tabloid -p tg-iced --check
cargo run --locked --manifest-path "$TG_VERIFY_DIR/Cargo.toml" -p lang-lib --example review_missing_cache
cargo run --locked --release --manifest-path "$TG_VERIFY_DIR/Cargo.toml" -p tabloid --example review_publication_cost
```

Tests cover initialization/retry/concurrency in isolated processes, fixed paths
across cwd changes, platform policies and non-Unicode paths, invalid YAML/locales,
transaction failure and concurrent language replacement, preference roundtrips
and injected write/rename failures, logger reentry, snapshot reclamation,
subscriber abandonment, sequence exhaustion and concurrent consumption.

The GitHub Actions foundation-libraries workflow runs library regression and GUI
compilation on Windows/macOS/Linux. A configured workflow is not an executed
platform result; local macOS results and remaining execution limits are recorded
in the delivery response.

Local delivery results (macOS, Rust 1.98.1): 31 library tests passed, plus one
portable GUI-consumer test. The final preference-directory boundary passed its
incremental language regression. All-targets Clippy for the three libraries and
portable GUI source, actual native-GUI all-targets compilation, and formatting
passed. Cached 10,000 missing lookups made zero allocations; 1,000 complete pack
lifetimes retained zero bytes. Fixed/growing publication probes passed with
1/16/64 subscribers: each shared pending batch was copied/applied once, and
10,000 abandoned subscriptions caused no subsequent delta applications.
Windows/Linux runtime execution and Rust 1.86 execution are configured in CI but
were not run locally. No native equivalence or visual GUI acceptance is claimed.
