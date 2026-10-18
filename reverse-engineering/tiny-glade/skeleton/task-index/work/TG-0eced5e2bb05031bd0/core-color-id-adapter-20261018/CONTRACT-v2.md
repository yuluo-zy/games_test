# Same-chain CPU color table initialization repair v2

V1 frozen39 inputs unchanged. Use candidate-v2.rs / candidate-v2.patch for SetWallColor source; the other three source candidates and integration-color-id.patch stay v1.

The current shared CPU registration runs init_color_ids_buffer each frame; its unconditional Commands.insert_resource recreates all three actual tables, losing prior colors and loaded session data. Root authorizes this necessary same-chain integration adapter. V2 adds Option<Res<WallColorIds>>, Option<Res<GateColorIds>>, Option<Res<HalftimberColorIds>> to the same existing init function, and inserts each table only when absent. No mirror initialized flag, separate owner, new resource model or run_once that overwrites already supplied tables. Presence is the existing resource's authority. GPU buffer initialization remains the explicitly unresolved original dependency; this patch does not pretend to create GPU resources.

Actual World/Schedule regressions: missing all tables -> same-frame init -> real setter -> second frame no newevent keeps color12; preloaded values in allthree survive init; missing onlyGate creates that table without touching Wall/Half. The chain lets existing Bevy deferred Commands installation complete before the real setter. Root compiles/runs; this worker does not execute Cargo/native.

The original B leaf here creates the three tables during init_color_ids_buffer142290750; it is initialization work. This v2 does not claim to have recovered original global startup scheduler placement: it safely adapts the current manual repeated CPU registration. No unrelated domain or color algorithm is rewritten.
