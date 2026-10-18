# B MetaCommander main-chain candidate

Claim TG-b85d349cabeedc081f, owner oschain-20261018. Candidate only, production untouched, no Cargo. B evidence identity is f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03. Complete commander13539B is reused from TG-40ff43c62ef8791c53/oschain-20261018/host-edit-save-audit/14091f180.asm and bound by commander-save-original-manifest.json; newly extracted native wrappers/serde are in meta-reload-original-manifest.json and meta-serde-fields.json here.

## Original dispatch, distinct from misleading old Rust comments

visit_str@140168e00/964B plus visit_enum@140168b60/658B and dispatch table14295e6c8 establish exactly:0 LoadSession;1 WaitHistoryReplay;2 WaitAsync;3 WaitAsyncMaxFrames;4 ResetTime;5 WaitFrames;6 FixTimestep;7 DisableInput;8 StartInputReplay;9 Screenshot;10 Freeze;11 Unfreeze;12 StartProfiling;13 FinishProfiling;14 ReportGpuUtilization;15 ReportMemoryUse;16 FloodWithDeadlyNeurotoxin;17 SaveAndQuit;18 QuitWithoutSaving;19 QuitProcess. The previous FloatEvent/CreateDirectory interpretations were not original semantics.

Serde fields: LoadSession{id,ignore_snapshot}; FixTimestep{delta_seconds}; StartInputReplay{path,skip_last_commands}; Screenshot{file}; StartProfiling{id}; FinishProfiling{id,output}; both report variants{file}; QuitProcess{status}. An integer payload cannot be turned into an owned path by casting/dereferencing.

Original main-loop dispatch is a loop, not one command/frame:14091f360 begins dequeue,140920923 finishes/drops and returns to that loop. WaitHistoryReplay140920c4f retains front and returns. WaitFrames14091fd28 consumes0 and continues; nonzero140920d37 decrements/requeues and returns. WaitAsync2 checks RtWorld structural dirty@+38, each DirtyCow dirty bit and RtBvhBuilderGlueResource::is_idle; busy becomes WaitAsyncMaxFrames300. MaxFrames3 busy nonzero decrements/requeues; idle or expired0 continues. Current canonical RtWorld::has_pending_changes() is the parent's read-only projection of those actual authority bits; pending_upload is not substituted.

ResetTime14091fe80 zeros elapsed/delta/frame_count/total_secs/ticks/frac, sets ran1/paused0 and RenderTemporalState.frame_index0. Freeze/Unfreeze only change GameTime.paused. FixTimestep constructs Duration::from_secs_f32 and queues TimeUpdateStrategy::ManualDuration (native tag2). Invalid duration panics after command consumption. DisableInput queues a new empty InputRecordReplayState with modeNone (native niche3), rather than preserving recording bytes.

SaveAndQuit14091fd55 writes a unit SaveGameCmd (event instance stride8@14091fd92), then queues ShutdownGame insertion@14091fdbc; QuitWithoutSaving goes directly to that insertion. QuitProcess140921079 exits with payload status as i32.

## LoadSession and state lifecycle

Raw tag0: UUID16 bytes@+8/+10, fenceArc@+18, ignore_snapshot byte@+20. The old payloads[3] fence check was off by one word. Fresh140920d85 checks AppMode13/14; those modes requeue the exact original and return. Other modes:140920f05 creates fence;140920f27 writes NextState byte2=Reload;140920f5c/f63 copy UUID;140920f6a copies ignore_snapshot;140920f86 appends owned fence;140920fa7 inserts canonical NextSession(kindLoadOnLoad);140921007 requeues original with fence and returns. Later14091f450 reads fenceblocked; blocked preserves exact UUID/ignore and returns; unblocked drops fence and continues. No active/latest-session fallback exists in this branch.

The candidate preserves the existing seven-word public carrier for on-load compatibility. It owns fences in the commander, uses the integer only as an identity key, never creates Arc from a pointer, and never reads a native pointer. `MetaCommand::load_session` packs/unpacks the canonical Uuid bytes. This is a current Rust ownership adapter, not a native ABI reimplementation.

Actual host state reset is independently proven: add_wall_systems@140493585 tailjumps SubApp::add_systems<PostUpdate,(initiate_game_shutdown,closure(State,NextState))>@140327ae0. run_unsafe closure@1403bf630/483B has1403bf6d1 `cmp State,2` and1403bf6db `NextState=1`. candidate-host-reload.patch adds this exact Reload→Pending(Game) consumer. The same add_wall_systems@14049356f..574 supplies dl1 to OnExit<AppLoadingState>@140327c20, binding walls_on_glade_unload@1402b1cb0; that actual cleanup is a separate necessary recovery under TG-f5d3ee34cc0589d842. A state-only toggle is not a complete reload.

## Current integration boundary

candidate-meta_commander.rs implements0/1/2/3/4/5/6/7/10/11/17/18/19 and loops through immediate outcomes. It removes the prior silent Handled catchalls. Owned-path/recording/profiler/critters cases8/9/12..16 now report their precise missing consumer and retain queue front. They are not counted as recovered. StartInputReplay skip_last_commands is not silently mapped to the current recording API's rewind: these meanings require its canonical owner to align.

Production save's current `SaveGameCmd {data:GameSaveData}` is still not the original unit Event, so this candidate intentionally awaits the canonical save-owner correction before compiling/integration. Original set1 is commander→(control_input_replay,record_and_replay_input_for_trailers)→advance_input_replay_frame, chain_ignore_deferred. Do not mount the commander alone and label full set1 complete. Tests in the candidate cover exact UUID/fence retention, unsafe foreign fence rejection, FixTimestep/tag17/18 identity and screenshot retention (not a fake empty-directory action).
