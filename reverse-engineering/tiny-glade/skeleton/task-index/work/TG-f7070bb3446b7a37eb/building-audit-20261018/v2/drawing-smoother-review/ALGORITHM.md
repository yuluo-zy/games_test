# Independent B DrawingInputSmoother audit

Read-only support for terrain-curve owner TG-e4029726f0fdb1a782. No production edits or Cargo. Original B SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03, matching PDB 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911. Procedure 0x14145c350, PDB1606:55688, size858 includes its unwind cleanup; main entry returns at0x14145c5d1. This is resampling and filtering based on the last accepted point, with no acceleration or elapsed-time integrator.

## ABI and existing resource identities

| Location | Input/output |
|---|---|
| RCX | sret RawInputSmoothed, a Vec<Vec2> header cap/pointer/len |
| RDX | &mut DrawingInputSmoother: Option<Vec2> with u32 tag+00 and previous accepted XZ f32s+04/+08 |
| R8 | &Vec3 current world point, x/y/z |
| R9 | &GameTime, native ticks_60 u64+18 |
| entry stack+28 | &MainCamera |
| entry stack+30 | &WindowSize, height u32+04 |

Both real draw_wall callers 0x140a7d32d/0x140a7d479 use this ABI. They pass separate current/continued smoothing states, the same sampled Vec3, GameTime, camera, window. The original draw FunctionSystem tuple names these exact existing resource identities; GameTime is tuple field+60. Current project already reexports GameClock as GameTime, including ticks_60. Do not replace the sample count with delta_seconds or invent a parallel time resource.

## Exact operations

1. Compute view z as f32 `((camera[88]*x + camera[98]*y) + camera[a8]*z) + camera[b8]` in that operation order; flip its sign bit. Execute original `utils::geometry::ws_to_ss`0x14127c950: `((0.5 / depth) * camera[14]) * float(window.height)`. These are separate f32 operations, no FMA or perspective clamp. Its result is pixel scale for a world distance.
2. Let `n=time.ticks_60`. Keep immutable interpolation start = previous accepted XZ when Some, else current XZ. Keep mutable previous accepted XZ separately.
3. For unsigned `i=1..=n`, convert i and n to f32, compute `t=i/n` as f32, then each component `candidate=(1.0-t)*start + t*current_xz` with separate f32 multiply/add in the shown order.
4. If previous is None, accept unconditionally. Otherwise distance = sqrt((candidate.x-prev.x)^2+(candidate.y-prev.y)^2), with separate f32 arithmetic. Accept only if **distance > f32(0.01)** and **distance * pixel_scale > f32(3.0)**. Both comparisons are strict; NaN/unordered skips acceptance.
5. Acceptance pushes one Vec2 and updates the persistent previous accepted state. Rejection changes neither. The interpolation start remains the entry state even after acceptance. Return exactly accepted points.

Constants: 1.0 bits3f800000 @142925904, .01 bits3c23d70a @142960674, 3.0 bits40400000 @1429cd078, .5 bits3f000000 @142925870. No forced current endpoint. With n0, output empty and state unchanged. With None and n>0, the first candidate is currentXZ; later candidates repeat and are suppressed. The last accepted state carries across frames, including all rejected intermediate samples. The native uint-to-f32 helper correctly handles high-bit u64 values; the probe bounds n to16 to avoid unbounded work and does not claim extreme-count execution coverage.

## Native fixtures

`native_smoother_probe.py` executes the full original entry, actual ws_to_ss and actual Vec growth. No game leaf is hooked; only existing allocator/memory library boundaries are bridged. 140 independent finite/infinity/depth/strict-threshold/sample-count cases match a separately written, explicitly f32-rounded reference **bit for bit**, for output points and persistent state. Eight cross-call sequence steps preserve prior accepted state across zero ticks and tiny displacements. Seven NaN cases verify acceptance counts and preserved state without forcing arithmetic NaN sign/payload across CPUs.

Fixtures retain exact input/output bits. Consumers should compare non-NaN values strictly; for arithmetic NaN validate NaN presence/branch behavior unless the target CPU semantics are explicitly controlled. Full wall draw/history/render integration remains the owner task, not evidence supplied by this leaf audit.
