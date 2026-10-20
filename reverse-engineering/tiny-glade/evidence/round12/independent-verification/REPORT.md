# Round12 independent verification

Verifier is not an implementation owner. Production source, databases, the
installation and Cargo target were read only. No Cargo command was run and
T13 was excluded. Findings below distinguish direct observations from their
interpretation and the unverified remainder.

## Audio completion

**Observed.** Fresh original RNE SHA256 is
`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.
`audio-channel-caller.asm` independently shows `EDX=1` at140a557d9 followed by
the call to1408d9d30. The latter's PDB identity is the std synchronous-channel
constructor; its zero-capacity branch is separate at1408d9d63. The actual game
selects the positive capacity branch. It is not a zero-capacity handoff.

`audio-recv.asm` shows the promise set to None at140a57121 before dispatch.
It passes `R8D=1_000_000_000` and accepts only `AL=2` at140a57190; its other
result branches reach unwrap panic. `array-recv.asm` at141679f7d and
`array-send.asm` at14167bc2f compare that incoming value and jump over the
clock/deadline comparison on equality. The delayed loader's140a52442..5246b
accepts success2 and the odd disconnect value1, drops the sender and returns;
timeout0 enters panic. No duration addition occurs in these caller paths.

**Inference supported by those observations.** Restoring
`std::sync::mpsc::sync_channel(1)`, blocking std send/recv, taking the receiver
once, panicking on receive disconnect and ignoring producer disconnect matches
the observed game-owned policy. The nanosecond niche denotes no deadline.
The current implementation reuses std; it does not reimplement channel internals.

**Executed independent verification.** `audio_lifecycle.rs` compiles the current
production `audio_resources.rs`, `audio_instance.rs` and `audio_utils.rs` unchanged
with direct rustc and cached original workspace Bevy/log dependencies. It ran
**43 tests, 43 passed, 0 failed, 0 ignored, 1.27s**. Four independent tests add:

-128 producers finish and drop before a receiver delayed over1.25s consumes;
-16 receivers remain blocked over1.25s before actual producer completion;
-128 receiver cancellations and128 producer failures preserve one-shot ownership;
-two competing receivers yield exactly one payload and one already-consumed None.

The original four completion regression tests and the directly included audio
files' existing tests ran in the same executable. Actual command, source/dependency
hashes and the result are in `verification.json`; full output is `audio-lifecycle.log`.
The first compile correctly failed because Bevy's derive requires
`CARGO_MANIFEST_DIR`; setting it to the actual country-core manifest directory
resolved the compile without any production change.

**Unknown / not upgraded.** No native FMOD initialization, bank loading, audible
output, actual worker launch order or full-App audio interaction was executed by
this verifier. The author's native-audio ignored test still needs explicit central
execution. These43 tests do not replace the central real-App acceptance.

## Wall saved-history wire

**Observed.** Both complete5944B serializer instances were independently read
from RNE. Jump tables1429bb2fc and142c5badc agree on these operations:

| Tag | Name from actual string operand | Actual payload evidence |
|---|---|---|
|9|ChangeStyle|Second instance calls141583920, PDB `WallStyle` serializer|
|10|ChangePlasterPeelAmount|Calls141585160 for f32 before+10/after+14|
|12|ChangeSupports|Bool helper141588150, before+10/after+11; actual keys `before_removed_supports` and `after_removed_supports`|
|16|ChangeSkirt|Bool helper141588150, before+10/after+11; actual keys before/after|

This independently supports the current `SavedWallStyle`/bool/f32/bool source
declarations and the dedicated support-field wrapper. Style must not be changed
back to an arbitrary integer or skirt to WallStyle on the basis of a stale table.

**Unknown.** This is a wire/static contract review, not a runtime replay-side-effect
or exhaustive original-history acceptance. The separate deprecated variants and
all other history domains are outside this bounded review.

## Save migration flags and snapshot consumer

**Observed.** Full original `parse_saved_game_data`14275B and `load_game`15561B
were independently extracted, avoiding truncated decompilation assumptions.

-Version<20 history decorator records select subtype56..58 and transform
  color+30 with the nine-way mapping, setting stack+2f0. Wrapper+d receives it
  at141472eab. In the successful wrapper's second word this is bit40.
-Version<21 selects subtype58, writes color999 and sets stack+304.
  Wrapper+e receives it at141472eaf, becoming bit48.
-The actual decorator serializer subtracts56 and names56 CottageWindow,
  57 GothicWindow and58 ArrowSlit. The two original log strings independently
  identify bit48 as arrow-slits and bit40 as windows.
-Load first tests bit48 at14146eb4e, traverses0x28 outer/0x58 inner rows and
  writes999 to+30 only where subtype+52==58.
-Bit40 at14146ec9c maps the16B rows' color+8. **That is not its whole body**:
  the exits at14146ed8d and14146edb8 target the cold block14146f6d8. There,
  14146f741 tests decorator subtype+52 minus56<=2;14146f751 reads color+30 and
  14146f733 writes its mapped value back. Thus snapshot windows really do
  migrate both the gate-color table and the three decorator window types.
-After snapshot apply1417a9c50,14146f7eb tests bit40, writes the selected cursor
  atHistory+68 and clears the stored snapshot length atHistory+28 on that bit.

**Adjudication.** Current flag names, bit placement, ArrowSlit-first order and
the extra window Decorators snapshot loop agree with those original branches.
Do not swap the names and do not delete the latter loop. An early verifier
concern based only on ec9c..ee0a was withdrawn after following the branch to
the actual cold block; no production code was altered in response.

**Unknown.** This does not validate all legacy terrain/enclosure migrations,
full parse failure handling or actual load→replay→render interaction. Those need
their own central tests and original input coverage.

## Roof support producer evidence boundary

**Observed independently.** A new raw PDB scan covered4894 modules,
6980730 symbol records and132599 IPI records. It found the same sole method
identity0xd9e3 for the support closure, two source checksums/declarations at
line95 in modules906/2245, no physical producer PROC and no matching INLINESITE
or physical C13 source line. `roof-pdb-independent.json` contains this fresh
scan rather than importing the owner's result.

Fresh complete2357B `add_roof_systems` at14062fcb0 calls its three-item visual
tuple helper140ddffb0 at140630039 with `DL=29`. That1131B helper records
three items, whose PDB signature is tile assembly, stone floor and spires.
It provides no fourth support-placement system.

The exact SSBO identity occurs three times in all original PE sections:
1412c272c,1412cb52b and142c2e320. Fresh constructor asm independently shows
None at1412c2670 and count0 at1412c2678; the typed constructor requests
0xa00000 bytes and its record length multiplies by80. The draw reads the
dense object's count. This is not a Sparse/Nani per-roof allocation contract.

**Inference.** These observations support keeping the retained initially-empty
channel without inventing a producer or substituting wall lintels.

**Unknown.** A raw exact-identity scan and absent CodeView bodies do not prove
global runtime unreachability. Anonymous/indirect mutation is still possible.
No live object-count watchpoint or complete alias/dataflow proof was performed.
The owner report's existing cautious boundary is warranted; “B has no producer”
would be too strong. No nonempty support-beam GPU acceptance is asserted.

## Efficiency lessons independently exercised

Use a direct-rustc test harness over unchanged production files when a narrow
CPU owner can be isolated through genuine dependency interfaces; keep the full
App and GPU tests as separate acceptance gates. In this case the focused compile
took under2s and execution under1.3s, without competing for Cargo's lock or
rebuilding country-core. Preserve exact source and dependency fingerprints.

Follow every successor of the relevant basic block before concluding a field
has no second consumer. A bounded range can be positive evidence, but cannot
justify a negative claim when a jump exits it. The migration cold-block check
above prevented an incorrect simplification. PDB function lengths and fresh
original bytes remain authoritative over names or a decompiler's contiguous view.
