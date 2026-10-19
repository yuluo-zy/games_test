# Solid selector None projection: active branch and successful-history boundary

This addendum supplements frozen BODY/Eq artifacts; it does not modify them. It does not accept arbitrary raw selector injection or hand-written previous maps as reachable successful application states.

## Actual builder gates

The complete B persistent builder `0x1412a9b30/5949B` is retained at `TG-bdac7a16d3c26c954b/nani-gpu-counts-20261019/nani-maintain-20261020/1412a9b30-full-B-rne.asm`. Plain albedo reads selector at `0x1412a9fd7`, compares9 at `9fdf`, and jumps to unwrap failure at `9fe3`; `spawn_empty` is later at `9ff1`. Emissive reads it at `0x1412aa5ea`, compares9 at `5f2`, jumps to failure at `5f6`; its spawn is later at `aa604`. The other shader branches in that complete body do not read this selector.

Current integrated builder SHA `59d915f2270c25781281b62a8afce9e1fe43bb1a4bc27b49ce2c5392991a00a9` has the same two active branches at source lines531 and555. Each executes `NaniSolidTexture::try_from(self.solid_texture.unwrap()).expect(...)` before `spawn_nani`; therefore both None and Some9 fail before spawning. In other branches the selector is unused. The three separate tag2 build gates in that source also now agree with the Eq projection for BrickColor/WallFlag/Smoke.

The actual clutter metadata decoder returns None for9 and checked canonical Some0..8 otherwise; the actual generator uses that Option's `encoded()` value rather than introducing raw9. This narrow producer fact is independently useful but is not extrapolated to every unfinished legacy generator.

## Why the active selector failure is not hidden by equality

For a successful previous active-albedo builder, the selector must be Some0..8. Replacing it with nativeNone9 makes the exact Eq projection unequal, so the actual build reaches its original failure. A previous successful active-albedo None/9 cannot have come from this build. In a branch that does not read the selector, None and raw9 produce identical build output, so equality of that native None representation does not hide a rejection in that branch.

The unchanged-Arc optimization relies on the actual Arc snapshot: current/previous share immutable allocations, while the real `insert_for` allocates fresh state when that allocation is shared. The public safe `get_mut` uses Arc::make_mut. Arbitrary unsafe mutation, explicit manual Prev map injection, or copying a failed intermediate state is outside this successful-history claim.

## Required snapshot and failure ordering

06's frozen `decorator-phase-audit/CONTRACT.md` proves original member14's second branch is `chain_ignore_deferred(fullvisual, middle visual consumers, copy_previous)`. The snapshot follows fullvisual and those consumers; it is not an early copy or a cleanup-only substitute. The actual reconstructed host must preserve that chain and fail-stop behavior: **a fullvisual panic/error must unwind/terminate the phase before middle/copy can run**. It must not catch the failure and continue or publish a successful next-frame snapshot.

The locked Bevy0.16 single-threaded executor directly supports this condition: `single_threaded.rs:164..166` catches a system panic only to immediately resume_unwind it, before advancing the system loop. Workspace bevy_ecs currently declares only std, while multi_threaded on bevy_tasks alone does not select the ECS executor. The central integrator must still confirm the resolved bevy_ecs features and actual final schedule executor before accepting this as an installed host fact. If another executor/error handler permits subsequent consumers/copy after failure, this invariant is not sufficient and the adapter must be revisited. This addendum does not invent an exception-swallowing recovery path or claim the currently incomplete installer already mounts member14.

All runtime tests and the resolved-feature/executor integration check remain root-owned and are not run here. Native behavior, current canonical input domain, source implementation and successful runtime history remain separately qualified.
