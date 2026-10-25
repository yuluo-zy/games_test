# Raster implementation increment

Status: partial module recovery; not full terrain/water/history or core completion.

Implemented:
- Six owned raster resources now expose borrowed read/write views, named stream validation, sequential writes and typed mapped staging uploads. Missing entries panic after their already-written prefix; extra entries are ignored. ECS-facing writes/uploads preserve prefix change ticks.
- The tile hash now uses original dependency ahash 0.8.11, rather than a local reproduction. Public RandomState seeds are derived from the observed fallback AHasher state; they are not claimed as author seed literals.
- draw_raster implements begin/continue/commit, decorator cull allowance, cursor start position, resampling, double dirty-map swap, scratch preview, staging publication and the original three caller-owned event closures. The common DebugHistoryMsg and NotifyHintSystemToolWasJustUsed event owners are real Bevy queues.
- RasterStroke owns the Arc point slice. record_raster_history detaches the active curve, resets radius/mode and adds the caller's edit variant to the existing HistoryTransactions owner. It preserves input/dirty maps/blend fields.
- GameTime aliases the existing GameClock Resource; GUI and roof consumers share one type/clock.

Corrections against older chapters:
- Native command tags 0/1 begin, 2 continues, >=3 commits (142260b42..60).
- Threshold at 1429cd060 is 0.001, not 0.0005; ucomiss+jbe skips NaN too. Native distance sums y*y+x*x before z*z.
- Both preview and commit stage selector 1; commit copies scratch into composite first. The commit tail is an inlined copy_to_staging, despite Claude calling it unresolved/no call.
- Begin stores the tag as blend flags, +4 as strength, +8 as radius, +12 as draw byte. Commit emits debug -> tool-specific history callback -> tool-used before mutating/publishing raster state.
- History reset Vec has cap=0, pointer=4, len=0. Pointer 4 is Vec3's empty sentinel, not capacity 4.
- NamedRasterData is an owned logical stream record, not the original 40-byte borrowed ABI.

Evidence: inspect_native.py checks original B SHA256 before bounded reads; native-identity.json and saved .asm record the ranges. parameter-types.json maps six init_state callees to exact PDB resource names. Existing terrain-water decompilation supplies the complete control/data flow; contradictory chapter summaries were not copied.

Claude: repaired helper draw-control.result.json reports success with inherited configuration and disabled tools. Its tag/control summary was useful; its commit-tail claim was rejected against native staging operations. Earlier validation response misread slice count and SBORROW; those guesses were not implemented.

Verification so far: 29 existing world_raster/world_raster_defs tests passed after dependency replacement and stream changes; two GUI ECS tests passed; all-targets check passed after initial state-machine integration. Final targeted state-machine tests and integrated review are recorded separately after execution.

Remaining: concrete Path/Cobblestone/Water/Garden input producers and full history dispatch/replay; original global scheduling; legacy renderer raw RasterFlags/type unification and GPU consumers. Generic caller callbacks are required, not default no-ops. DecoratorArchivist shares its existing owner but its full culling/archival lifecycle remains partial.
