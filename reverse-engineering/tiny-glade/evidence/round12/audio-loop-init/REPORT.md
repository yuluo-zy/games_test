# Canonical AudioLoop startup restoration candidate

Evidence-only delivery; no production files changed, no Cargo/rustc/codegen run.
Original B RNE SHA256: `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.

## Observed

Original `AudioCore::instantiate_loop@140a59220/459B` increments descriptor
`count@+10`, collects every element of `0..count.wrapping_add(1)`, retains the
first element, probes its actual `loop` parameter, copies descriptor `+18` into
the stop-delay field, and initializes started/noloop false. It returns an error
on instance-creation failure, after releasing the successfully created prefix.
Startup callers unwrap that result. The collect/iterator implementations are
dependencies, used only as original executable evidence, not reconstructed.

`../firstperson-full/native_init.py` was rerun: complete original constructor,
collect and iterator execute for descriptor counts 0/1/3/7 and produce actual
1/2/4/8 entries, delay 50, all three flags zero with the Studio disabled. Only
allocator/Vec growth/CRT bridges are installed. These four executed cases do
not validate FMOD or active/error creation.

`verify_callers.py` independently extracts actual constant stores/LEAs at all
68 direct constructor call sites in these 13 original initializer bodies:

| Initializer | Direct sites | Descriptor policy |
| --- | ---: | --- |
| water | 4 | count 3, delay 50 |
| wall | 10 | count 3, delay 50 |
| path | 5 | count 3, delay 50 |
| signifier | 1 | count 3, delay 50 |
| timberframe | 3 | count 3, delay 50 |
| garden | 4 | count 3, delay 50 |
| forest | 1, repeated for 50 slots | count 3, delay 50 |
| roof | 3 | count 3, delay 50 |
| playermade stairs | 6 | count 3, delay 50 |
| decorator | 16 | count 3, delay 50 |
| terrain | 4 | count 3, delay 50 |
| tree | 4 | count 3, delay 50 |
| building | 7 | count 3, delay 50 |

`native-caller-descriptors.json` records every event name, actual call address,
full original initializer span and byte SHA; zero unresolved descriptors.
These are static call sites, not 68 separately executed constructors.

`verify_inline_policy.py` separately proves the inlined contracts:

- Sheep constructors at `140b6a92c` and `1414d9b82`: **Sheep/Pet**, count 3,
  delay 50. The current Baah event belongs to a different startup producer.
- Terrain-stairs constructor at `1417e3389`: **StairDraw_LoopTrem**, count 3,
  delay 50. The separate OneShot event remains the one-shot event.
- Water ambience inline constructor: range 0..4, delay 50, noloop false.
- All **five** path rings overwrite returned owner `+36` with 1, at
  `1409d0987/a1f/ab7/b64/c05`. The fifth store copies into rbp+150 then sets
  rbp+186; it must not be missed by inspecting only the immediate return slot.
- The decorator initializer's sixteenth actual name is
  **Cosmetics/MoveBuilding**, not another Lantern/Amb.

The first-person inlined count-3/delay-50 callers were independently captured
in `../firstperson-full`; its prior local workaround is removed in this patch.
The independent agent's separate static constructor/collect/iterator audit is
`../independent-startup-reload/audio-loop-policy/evidence.json`. It agrees on
count+1, retained first entry, delay and actual parameter probe. It is a second
static audit, not another executed corpus.

## Candidate and ownership

`candidate.patch` / `candidate-manifest.json` contain 13 narrow source files.
`AudioLoopDesc` now represents name, extra-instance count and stop delay.
`try_instantiate_loop` propagates creation failures and lets collection release
the owned prefix; the existing startup adapter unwraps before publishing the
resource. The first actual instance supplies parameter capability; a failed
probe logs and retains false.

`AudioLoopPlayer` now owns one complete canonical `AudioLoop`, preserving its
cursor, stop timer, delay, parameter capability and noloop flag instead of
discarding those fields into a Vec. Existing request positioning/volume order
is retained. The wall/building/stairs access changes reference that same owner.
The existing AudioLoop lifecycle algorithms are unchanged: the only edit in
`utils/audio_utils.rs` adds the original `noloop` flag adaptor.

`forest_audio.rs` is deliberately absent. Its full 50-loop/FIFO/map candidate
belongs to history_wire_fix and depends on this canonical producer: 50 rings
times 4 actual events = 200 events. No shadow forest owner is introduced.
No rhapsody or generic dependency implementation is changed.

First-person patch base is the prior full candidate (exact SHA in the manifest),
not the older production file. Integrate firstperson-full first or apply the
one constructor hunk manually to its already merged version.

## Validation and limits

All 13 proposed files passed rustfmt syntax parsing through `--emit stdout`,
which neither mutates files nor compiles. Source bases and candidate SHA256s
are frozen in the manifest. No Rust test has been compiled or run by this agent.

The candidate includes meaningful runnable Rust tests for actual canonical
initialization with a recording Studio backend: all four native disabled count
cases; actual first-event probe true/false/error; four actual creation failures
with exact prefix release and no premature probe; actual Path/Sheep/Stairs/
Decorator state constructors with 92 recorded event creations, correct names,
four events per ring, delay and every path noloop flag; and player delayed stop
strictly after 50 ms with rotation. Prior first-person real ECS/native fixture
tests remain unchanged except delegation to the canonical producer.

Unknown: central type checking/test execution, real FMOD execution and real
backend creation-error injection remain unverified. This delivery does not
claim that all startup systems, reload lifecycle, editing/history or GPU chains
have passed. Active/error branch expectations additionally rely on direct
original CFG, and are not promoted into the four disabled native executions.

## Reproduction

Use Python313 for `verify_callers.py`, `verify_inline_policy.py`, and
`../firstperson-full/native_init.py`. `build_candidate.py` regenerates candidate
files from the current base and prior firstperson-full candidate; do not rerun
it blindly after another agent updates production. The frozen manifest is the
integration boundary. The native scripts assert the original full-file SHA.
