# Timelapse state producer, direct B evidence

Original TimelapseState is a byte enum, not ReplayBuffer emptiness or DragState. Predicate14092e6b0 is `byte != 2`. The historical chapter's `0=idle` label is inconsistent with this predicate and the actual writers.

The sole relative CALL to App::insert_resource<TimelapseState>1401295c0 in original .text is1405a1471. Immediately before it,1405a146f executes `mov dl,2`;1405a146c supplies the App pointer in RCX. Therefore original run_game explicitly initializes tag2. See timelapse-init-original.asm; PDB procs.tsv identifies the exact insert_resource generic at1401295c0. This is a real initialization, not an invented default to pass the menu resource gate.

start_timelapse140a8a310 has actual params EventReader<StartTimelapseReplayCmd>, History, MetaCommander, ActiveSession, AppMode, RestoreAppTo, PlayUiSound writer, ReplaySpeedMultiplierOverride, TimelapseState. It takes the last unread start message and calls check_if_can_switch140af9e00. Its successful inlined closure records AppMode/ActiveSession into RestoreAppTo, pushes a MetaCommand through140b59be0, enables the replay-speed override, and writes state0. Ref: retained ui-input/decomp/decomp-0x140af9e00.txt plus camera-time/decomp/decomp-0x140a8a310.txt. Full MetaCommander schema/producer integration remains required; state0 must not be used as inactive.

finish_timelapse140a8a460 takes InterruptTimelapse, TimelapseReplayFinished writer, ReplaySpeedMultiplierOverride, TimelapseState, MetaCommander and ActiveSession. If state!=2, an unread Interrupt and even state cause a session MetaCommand to be queued and state=1. When the MetaCommander queue is empty, the speed override discriminator is set0 and Finished is emitted carrying the current state byte. cleanup_timelapse140a8a5d0 consumes Finished and writes state2. The byte controls real transitions; naming enum variants may improve design, but fabricated idle0 or a permanently inactive resource would hide missing producers.

Read-only contract delivered to root/startup_chain. No extra core source mutation after the round12 freeze.
