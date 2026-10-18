# B replay finish contract (2026-10-18)

B-only implementation, C ownership family retained without asserting equivalence. Native RNE SHA and analysis SHA plus full function-range identity are in native-identity.json. Fresh shared MCP returned the same branches under placeholder void signature; raw machine code establishes register/stack reads. PDB SystemParam signature independently confirms EventReader<HistoryCmd>, PendingHistoryCommands, ReplayBuffer, ActionSetStates and EventWriter<CmdUnfreezeSheep>.

- 0x1408b36c7..376c: when Replay, pending length==1 and front byte0==1 emits unit CmdUnfreezeSheep. This is byte0, not the undo/redo direction byte1. No local fake sheep state is modified.
- 0x1408b3770..3785: nonempty pending or ReplayBuffer active count takes command-scan path and returns without restoration when no stop command exists. The old implementation dropped all these returns.
- 0x1408b38fb / 39fb / 3ab3: if queues empty, undo/redo states16/17 must both be >=3 to finish naturally. Either held still scans commands and otherwise returns.
- Every scan advances only to first HistoryCmd byte1==3; this forced stop restores mode even with work/held inputs. This reads the real HistoryCommand event stream independently of cursor reader, not NewReplayCommandEvent (fourth-byte output).
- Restore AppMode from RestoreAppTo. Tools 6..19 call Toolbar::switch_mode. Editing/CloneClutter select or cycle Editing category using Toolbar::advance_category, without overwriting restored AppMode.
- MetaCommander front tag 0x8000000000000001 consumed only after successful finish in Replay, or immediately when already outside Replay. Nonfinished Replay never reaches that sentinel.

Open: CmdUnfreezeSheep currently lacks a real critter consumer in this workspace; deliver exact unit event identity and registration obligation, not log-as-behavior. Original global ordering and C remain unknown. No runtime original oracle. Root owns compile/test and shared registration due disk limit.
