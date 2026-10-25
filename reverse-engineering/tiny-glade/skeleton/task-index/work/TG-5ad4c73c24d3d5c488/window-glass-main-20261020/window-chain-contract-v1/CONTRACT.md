# Window lighting → canonical component → glass update (original B)

Status: design only. Main-chain completion, source compilation, GPU/present and original runtime equivalence are not accepted by this packet.

## Identity and bounded evidence

Original B RNE f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03, PDB2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911. Four complete PDB procedure byte ranges are bound by native-pack-v1: window_lights1407b5980/3333, glass142075070/4529, retain141268bd0/599, lerp140c904e0/81. Main symbols include unwind funclets; Ghidra's shorter body does not replace these byte ranges. Private worker decomp resultV2 succeeds; earlier shared BufferMgr failure is retained, not treated as evidence. No database signatures were modified.

Typed initializer14002a250 proves the real six system parameters: ResMut<GlobalWindowLightsControl>, Query<&WindowLight>, Res<TimeOfDay>, Res<GameTime>, Res<ActiveSession>, EventReader<LightWindowCmd>. Glass initializer1420673f0 proves Query<(NaniActiveInstance,Transform,WindowLight,ColorId,Option<SemiTransparent>,Option<AlbedoMap>)>, Res<GlobalWindowLightsControl>, Res<AssetTextureLibrary>, Res<GpuWorld>. Source-path type observations and assembly addresses remain separate from a reconstructed Rust ABI.

## Canonical owned values

Restore missing core/resources/window_lights.rs, declared once in resources/mod.rs. GlobalWindowLightsControl has one std::collections::HashMap<usize,LightInfo> and bool lights_on; original std RandomState map retain symbol proves owner and usize key. No local cycle_t, mirror ECS state or second map. LightInfo has brightness:f32, timer:f32, actual_on:u8, target_on:u8. WindowLight is one Component {key:usize,decorator_id:u32}; observed Win64 stride16, key+0/id+8. Safe named fields do not assert a Rust C ABI. Existing builder input u64 is losslessly converted on supported64-bit hosts (checked usize conversion), not truncated. Existing raw12byte WindowLight must be replaced/reexported, keeping same first64bit value and decoratorid. Existing old WindowLightState import can reexport LightInfo; no duplicate type.

Original Component required-components full1776B is derive scaffolding, not1776B of game logic. The only registered descriptor name is country_core::resources::window_lights::WindowLight at142c38039/51B; enforces recursion and self registration, with no additional required-component insertion. Reuse Component derive.

Global default observed in actual DecoratorPlugin build inline source17: empty std map, RandomState initialization, lights_on=false. LightInfo's native or_insert explicitly writes all four fields; for newly queried windows brightness=timer=0 and actual=target=lights_on. Do not use old timer1 default for insertion.

## Exact system effects and order

1. Drain every unread LightWindowCmd, keep only the last payloadu32. EventInstance stride16, payload+8. Restore canonical event from unit to tupleu32; no actual current Rust writer exists in all crates. Original color-menu writer/UI restoration remains separate; a restored event reader does not establish the UI chain.
2. Read real TimeOfDay.time_of_day. lights_on=(olden?0.44:0.51)<=t &&t<0.72. Always write the global flag, preserving resource change tick even for an empty map.
3. Enumerate all existing std map values in their actual current iteration order, before new insertions or removals. For each target!=lights_on, local pinned fastrand1.8 Rng seed=frame_count(u32 widened to64)+ordinal(u64), timer=2*Rng.f32(), target=lights_on. Preserve skipped entries' ordinal contribution. Direct call140caf380 is symbolized to fastrand1.8 Rng::f32/gen_u64, and pinned source confirms with_seed writes seed directly. No global RNG, key sorting or new persistent RNG.
4. Iterate actual Query<&WindowLight>, append every key to a transient Vec. Insert only absent states with the above four explicit values. If latest event payload==WindowLight.decorator_id, set brightness=1 and target=1 only. Do not change actual, timer or earlier-event windows. Same key shared by multiple entities intentionally shares one light state.
5. Retain exactly keys present in that live-key Vec. Full599B retain closure is Vec<usize>.contains; no timer/on predicate, positional mirror, HashSet or fabricated entity collection.
6. For each retained value, only when initial timer>0: timer=MAXSS(timer-delta,0). Implement explicit `remaining>0 ? remaining : 0.0` to preserve second-operand NaN/signedzero behavior. If resulting timer==0, actual=target. Initial zero/negative/NaN must not copy target. Then brightness=lerp(brightness, actual?1:0, GameTime.delta,3.0).

Lerp81B calls thunk142923ec0, IAT143193130, api-ms-win-crt-math-l1-1-0.dll!expf: `(b-a)*(1.0-(-dt*rate).exp())+a` with separate f32 operations/noFMA. Restore a single shared utils helper by appending to existing feaa source; preserve all old math and7tests. Old numeric u32 cast/sin/0.05/timerbits were false reconstruction. Cross-platform exp library lastbit parity remains a native differential limitation, not proof of bit-identical execution.

Manual command quirk is intentional: a timer0 actualOFF state flashed to brightness1/targetON then fades toward actualOFF until a later native timer transition. Do not repair it into immediate-on behavior.

## Real glass consumer

Add actual ECS updater to existing Nani systems and mapped-input admission (one marker+Transform+WindowLight+ColorId). Reuse existing build_window_glass, same owned subset, InstanceKey, write_mapped_update and same texture library. Read brightness via control.lights.get(&light.key).map_or(0.0,...): original XMM9 starts0 and only loads on a key match, so this is original behavior, not a fallback invented for an unsupported mapping. SemiTransparent is optional. Absent AlbedoMap maps None; present texture name must resolve with existing asset API and panic when absent, then use actual bindless index. Do not modify the existing matrix/packing algorithm or allocate a per-frame row Vec. Add the updater to the actual backend44 unordered update tuple; retain maintain→ApplyDeferred→updates→flush sequence.

## Actual App mount and lifecycle boundary

Original decorator window_lights is tuple8 member6 in rawset0x24=36, independent from set25 nineteen-member pipeline. Use existing App Update GameSystemSet36, ordinary in_set; do not infer sibling order from tuple position, and do not mount old incomplete aggregate decorator configuration. Current actual numeric sets are ordered with ordinary before and auto deferred sync; this lets set25 Commands become visible before36, with set44 after36. Register real canonical global resource and Event lifecycle in actual App.

Original decorators_on_glade_unload9451B contains global-control Default reset inline at+2945/88B. Before implementing an App unload reset fragment, collect exact reset stores/tick/drop and original OnExit registration; otherwise keep lifecycle explicitly pending and do not claim two-session behavior. Do not replace the full unload function with map.clear or claim the rest of9451B recovered. Source design can be split into a complete per-frame chain and later exact lifecycle/App mount if reset proof remains incomplete.

## Proposed validation (not executed)

Actual same-World ECS tests: current TimeOfDay rather than removed cycle field; empty-world flag tick; query creation and disappearance; shared keys; stable-frame/actual-map ordinal RNG and no global RNG consumption; last unread matching event; flash quirk; timer positive/zero/negative/NaN/deltaNaN; existing finite brightness+exp progression; typed builder produced component queried by window_lights; real same-subset glass bytes plus optional/present/missing asset behavior; event lifecycle; unload then second-session key reuse when exact reset fragment admitted. Tests must execute actual production functions and owned current types. No cfg(test) core device imports from dependent crates, no fake fullApp fixtures, no old core rlib. Current complete core build remains blocked by missing source; added tests unrun until honestly runnable. Shared helper can be tested separately without calling that whole-core proof.

## Remaining acceptance gates

Independent review of this original evidence/source design; exact default/reset and App source delta; canonical builder and Nani all-type presence changes; original rendering algebra remains existing reviewed implementation; WindowLight event actual color-menu producer still absent; actual geometry/windows, mesh schema loader, full world GPU/WSI/present, interaction and save closures still pending. No progress percentage inferred from evidence or function size.
