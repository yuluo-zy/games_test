# Read-only audio registration lifetime review

Reviewed the current `country-core/src/systems/audio/mod.rs` against root's retained `before-audio-registration-lifecycle.rs` and 18-pair manifest. No production edits, Cargo builds, or tests were run. No blocking code defect was found in the 18 registration adapters.

Each initializer takes `Res<AudioCore<S>>` and `Commands`, constructs exactly the State below with `S::Event`, and inserts that one resource. Each condition checks the identical `module::State<S::Event>`, each paired update takes `ResMut<State<S::Event>>`, and each type derives Resource. Only init has the missing-resource condition; update runs every frame. There are no extra side effects in these init functions that must continue running after the owner exists.

| Module | Exact owner used by init, condition and update |
| --- | --- |
| timberframe_audio | TimberframeAudioState<S::Event> |
| forest_audio | ForestAudioState<S::Event> |
| tree_audio | TreeAudioState<S::Event> |
| firstperson_steps | FirstPersonViewAudioState<S::Event> |
| garden_audio | GardenAudioState<S::Event> |
| building_audio | BuildingAudioState<S::Event> |
| decorator_audio | DecoratorAudioState<S::Event> |
| game_loading_audio | GameLoadingAudioState<S::Event> |
| menu_audio | MenuAudioState<S::Event> |
| path_audio | PathAudioState<S::Event> |
| playermade_stairs_audio | StairsAudioState<S::Event> |
| replay_audio | ReplayAudioState<S::Event> |
| sheep_audio | SheepAudioState<S::Event> |
| signifier_audio | SignifierAudioState<S::Event> |
| terrain_stairs_audio | TerrainStairsAudioState<S::Event> |
| trigger_audio | TriggerAudioState<S::Event> |
| wall_audio_fmod | WallAudioState<S::Event> |
| roof_audio | RoofAudioState<S::Event> |

The old unsorted per-frame init entries queued a replacement State on every schedule run. That discarded persistent playback/ring/snapshot ownership at the deferred flush and left the initial update without an explicit dependency on resource creation. The new missing-resource condition preserves the current owner, honors an owner installed by the host, and recreates it when it is genuinely absent. Pair-local `.chain()` establishes init before update and allows the initialization Commands to be committed first.

This Commands conclusion is grounded in the actual locked `bevy_ecs 0.16.0` local source: `schedule/config.rs` documents and implements ordinary `.chain()` with deferred sync edges; `schedule/auto_insert_apply_deferred.rs` detects `has_deferred()` and inserts an ApplyDeferred node; `ScheduleBuildSettings::new()` enables `auto_insert_apply_deferred`. Current production sources do not disable that build setting. A host that explicitly disables the Bevy automatic deferred pass must supply its own flush; this review does not claim `.chain()` overrides such a host setting. The default Schedule used by the existing terrain registration regression has the required setting.

The before/after registration bodies retain the same multiset of 47 generic system references, plus the nongeneric ivy entry. No domain consumer was removed, duplicated or accidentally placed under the initializer's run condition. The remaining direct systems and the two existing water consumers are unchanged. Their external resource dependencies are not established by this patch. Terrain retains its separate tested helper, with init/consumer chain after TerrainCpuStage::Changed; it appears once and was not duplicated among these 18 pairs. The new dependencies are pair-local and introduce no reverse domain edge. They are a necessary current-architecture adapter, not evidence of the original global schedule or complete audio recovery.

`review-evidence.json` records each source hash, exact init body, consumer signature and registration-pattern match, the locked Bevy source hashes, and original/current module hashes. The retained before hash agrees with root's manifest. The current after hash differs from the manifest's recorded `6f42be…`; root was notified to refresh the final after fingerprint. This is an evidence bookkeeping issue; all conclusions here bind the independently captured current hash. The separate monotonic stop_now consumer patch remains a distinct incremental change and does not change these init signatures or owner identities.
