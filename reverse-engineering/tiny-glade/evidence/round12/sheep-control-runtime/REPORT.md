# Complete SheepControl runtime candidate

Evidence-only; production and Cargo remain central-owner work. Original B RNE
SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.

## Observed original contract

- `SheepControl::new@1416f0ea0/759B` and
  `Commands::spawn<SheepControl>@1412e23e0/756B` establish a Component. The
  original ten SystemParams include `Query<&mut SheepControl>`, not ResMut.
- Runtime `1416f11a0/4929B`, snapshot apply `1416f24f0/773B`, and four GUI
  closures `1416ed110/4343B`, `1416ee210/503B`, `1416ee410/2736B`,
  `1416eeec0/1255B` were reviewed against original bytes and fresh decompilation.
  No PDB void() placeholder was treated as an ABI.
- Original 80-byte component owns separate Option<Vec<40-byte SheepSnapshot>>
  at +0 and +18, wire/solid Entity at +30/+38, selected SheepId at +40 and
  PickingMode byte +48. Constructor sets both snapshots None, selected 0,
  picking None. The native nonempty Vec rows are the canonical snapshot type.
- First snapshot is the continuously applied freeze. Second is the manual
  Take/Restore/Save/Load snapshot. The typed producer is original
  Vec::from_iter<QueryIter<...> mapped by capture_sheep_snapshot>, not a
  guessed Vec<Entity> or opaque payload.
- Picking values have actual GUI writers: 0=None, 1=Set goal, 2=Set position.
  Selection uses Sheep+20, whose original births initialize 0/1/2. The current
  canonical name `Sheep.pet_target` is retained; Entity bits are never used.

## Complete frame policy restored

1. Fetch the unique controller or panic, then apply an existing freeze by query
   order to the shorter of snapshot/live sheep. Empty Some remains meaningful.
2. Set requested freeze to the initial Option state. Original global
   DEVELOPER_HOTKEYS_ENABLED plus KeyU just-pressed toggles that request.
   Public-symbol lookup identifies B global143634cd8; this reuses the current
   canonical developer_hotkeys_enabled owner. The original AppMode parameter
   is present but unused.
3. Hidden/disabled debug UI queues Transform removal for both real debug
   entities. Enabled UI without an Egui context leaves them unchanged.
4. A real Egui `Sheep control` window edits the Freeze checkbox, manual snapshot
   actions and selected sheep. Next sheep uses wrapping add modulo3. It shows
   selected movement, inserts the selected sheep's full Transform on the wire
   marker, and places the half-scale solid marker at an existing goal +Y.
   Missing goal removes the solid marker Transform.
5. Original selection spheres register radius1.25 at position+(0,0.8,0), names
   `sheep_selection_{SheepId}`, original extra low bits0xe01 and occlusion true.
   Phase4 continuing drag only moves X/Z when original scalar
   sqrt((dz²+dx²)+dy²) is strictly greater than0.01; height stays unchanged.
   Click chooses SheepId. No terrain hit supplies the original zero position.
6. A pending point action registers `sheep_pick_point`, radius0.25 at cursor,
   original extra low bits0x101 and no world occlusion. Clicked Set position
   changes X/Z only. Set goal uses cursor.x0z, sets a present goal and Walking
   {timeout0, goal_change_timeoutMAX, force_reach_goal=true}. Missing selected
   sheep cancels picking. Successful action resets picking and refreshes an
   existing freeze; completed direct drags also refresh that freeze.
7. Apply requested freeze: capture only when newly frozen, retain an existing
   freeze, or drop when released. Unit unfreeze events are consumed last,
   clearing after the current frame's restore/edit/capture.

Manual Restore applies to live sheep and clones into the first snapshot only
when already frozen. It never starts freezing on its own. Taking a snapshot
only changes the second owner. Manual Load applies to live sheep and replaces
only the second owner, preserving the original distinction from Restore.

The original file UI is included: rfd0.10, `json` filter, directory
`tmp/sheep_snapshots/`, pretty canonical JSON. Save attempts recursive directory
creation, then unwraps canonicalize; file write failure logs. Load canonicalizes
without directory creation, then ignores read/parse failure. Successful Load
applies and retains the typed snapshot. Cancellation leaves state unchanged.
No fake filesystem/dialog owner or empty GUI shell replaces those branches.

## Original execution and independent review

`native_runtime.py` maps the unchanged original image and executes the entire
runtime with actual dense Query memory for three sheep and one controller.

- 32 no-context fixtures cover None, Some(empty), one row, three rows × developer
  gate × KeyU × trailing unfreeze. Actual original snapshot apply and capture
  execute. Original hidden branch writes two24-byte deferred commands.
- 48 additional fixtures execute the original picking path: 36 combinations
  of mode/selected ID/frozen/no-terrain, and12 drag/click/threshold/NaN cases.
  They retain original query iteration, picking branches and snapshot updates.
  Generic Egui window new/show are bridged and the GUI content callback is not
  invoked. Std formatting and the separately restored UiState registration
  boundary are bridged with fixture widget IDs. These48 are consumer-policy
  execution, not real mouse→picker→Egui acceptance or complete native GUI runs.
- Allocation/free and CRT boundaries are bridged; key lookup bridges Bevy's
  generic HashMap lookup. No SheepControl branch, capture or apply helper is
  replaced by a model. Fixtures are original outputs, not a Rust comparison.

`../history/sheep-abi-independent-review/REPORT.md` is a separate agent's fresh
original-byte and typed-producer audit. Its30 original slice fixtures cover
picking writes, selected ID wrap/modulo3 and manual snapshot replacement/free.
It independently confirms Restore's conditional frozen-snapshot clone. The
reviewer also checked this frozen source's snapshot and picking ownership and
found no inversion. This independent scope does not include file dialogs.

The dedicated persistent Ghidra worker is `sheep-control-runtime`. Its GUI job
prepared bounded original ranges with exact byte SHA and returned all four
decompilations; `gui-result.json` reports ok/passed, identity B patched SHA
cf048e27..., no ABI prototypes changed. Reachable Ghidra bodies exclude SEH
cleanup ranges; the complete PDB spans remain in the independent raw asm.
No modification is claimed synchronized back to the historical database.

## Integration and tests

Four proposed files are frozen by `candidate-manifest.json` and `candidate.patch`:
the full controller source with tests, two original fixture files and the sole
Cargo addition `rfd.workspace = true`. Workspace already pins original rfd0.10.
No production change or Cargo/rustc/LLVM work was run by this agent. Rustfmt
syntax parsed the actual combined candidate through --emit stdout.

The four runnable Rust tests cover eight actual ten-parameter ECS frames with
nonempty snapshots and late unfreeze, all48 actual picking-consumer fixtures,
conditional manual→frozen clone, and a real Egui window registering widgets
and inserting the wire/goal Transform on two real debug entities. The actual
system test avoids changing process-global developer state under parallel
tests; its KeyU/gate32 original fixtures remain separate native evidence.

Required companion: wall_gpu_chain's startup/constructor/DTO migration. It owns
load_game, startup_sheep and external snapshot tests, including removal of old
Res/ResMut registration. The public contract is new(wireframe,solid), public
snapshot, Component-only ownership. It also owns the narrow canonical capture
predicate correction `goal_tag != 0` → `goal_tag == 1`, confirmed by native
capture. PendingSheepSnapshot is only a one-transfer loading DTO, never a
concurrent live controller.

Actual system test setup requires AppMode, CursorTerrainRaycast, UiState,
DebugGui, ButtonInput<KeyCode>, and Events<CmdUnfreezeSheep>. EguiContext is
optional. Exactly one real controller and two real debug entities must exist;
fake Entity::from_raw(1/2) can target a sheep when the hidden branch removes
Transform. Internal tests create real debug entities first. The old standalone
apply_sheep_snapshot system adapter is replaced with the original helper ABI
(&mut SheepQuery, &SheepControlSnapshot); there is no Res shadow adapter.

Unknown/unverified: central Rust type checking/tests, native Egui clicks, real
OS dialogs/file failure injection, and the complete physical mouse→picker→UI
update→sheep chain. This restores those source paths, without claiming they
have already passed integration. T13 and generic dependency reimplementation
remain excluded.
