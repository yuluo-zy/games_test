# Real DecoratorIdGen propagation (minimal existing helper change)

Base: final Snapshot61+testv2+WallIdv3 source SHA8508fceddfca138c2621a95d19f08ee6e125d6b8b5b0622b69225a4074210835. Candidate only in work; source owner/root applies. No save file, save thread, OS file or host source is written.

Original B restore projection evidence in frozen69 restore-pack/restore-store-pack proves R8=&mut DecoratorIdGen, reserve at140a20f28..f42 before each child insert. The independently reviewed owned storage adds restore_snapshot_with_id_gen; old restore_snapshot intentionally lacks a real generator argument and is compatibility only.

Two existing snapshot helpers now require &mutDecoratorIdGen: init_game_state_from_snapshot→apply_loaded_snapshot→restore_snapshot_with_id_gen. The same mutable reference flows through, with no default/new generator. The existing real Terrain application test is updated to pass a pre-reserved caller-owned generator and verify that its next allocation remains43. Nonempty typed storage restoration/reservation is separately tested by frozen69. No fake nonempty JSON decode test is introduced.

## Actual current caller map and boundaries

- run_game.rs::install_history_and_wall_runtime initializes one authoritative DecoratorIdGen and DeferredDecoratorOpEvents, together with actual DecoratorStorage. It does not call the snapshot apply helpers.
- snapshot.rs::init_game_state_from_snapshot is the only non-test caller of apply_loaded_snapshot; current repository has no non-test caller of init_game_state_from_snapshot. A future/actual loader must pass the existing runtime resource, never make a temporary local generator.
- run_game registers load_game_system in its existing numbered schedule. try_load_game_system state Loading calls require_no_valid_snapshot. That function returns an explicit error when a real snapshot file opens; no complete chunk decode/apply has been silently added. No-snapshot history replay uses its separate actual World path.
- snapshot61-v3 DecoratorsSectionJson::into_storage_snapshot explicitly returns Err because the legacy DecoratorInfo DTO is not native HistoryState wire. Thus this helper API propagation removes the generator-loss bug when a real decoder is connected; it does not make nonempty snapshot JSON/file loading accepted. Actual nonempty ID reservation proof is frozen69's typed storage test, authored not executed by this agent.

Validation: rustfmt parse and isolated patch exact only; no Cargo, no target execution. Root will compile centrally. No original execution retry or permission bypass.
