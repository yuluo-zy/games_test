# Independent review of decorator interaction candidates

Review scope: `decorator-interaction/widget-and-handle.patch` and current
`ui_systems/move_ops.rs`, against the retained original B disassembly, actual
canonical destination setter, WidgetParams constructor and geometry methods.
Read-only review; this worker did not edit those files. No separate Rust/UI
runtime execution is claimed by this review.

## Changed widget and handle logic

No blocking discrepancy found within the changed logic.

- B1412823c0 stores the fourth argument from XMM3 at volume+4, with tag5 at
  volume+0. The candidate correctly uses f32 distance; the former u32 layer
  interpretation was wrong.
- B14128241c..428 ORs the constructor's existing flags with bit1 and conditional
  bit3. The candidate preserves bit0, producing3/11 with the actual constructor.
  It correctly preserves the default world-occlusion byte rather than forcing
  false. The actual UiSounds 96-byte owner is retained.
- B14128c938..96e reads the handle's u64 at0 and u8 at8, then writes exactly8
  and1 bytes to the zero-key SipHasher13. The candidate's canonical tuple and
  two Hasher writes match. It does not use a string key or its terminator.
- Only defined volume payload fields are reconstructed; the original leaves
  inactive enum/padding bytes unspecified. Zeroing inactive logical fields is
  not evidence that every native padding byte equals zero.

## calculate_move_dst

No blocking discrepancy found within this restored helper's valid-candidate
domain. The helper remains dependent on its actual intent producer and published
wall state; this review does not close those callers.

- Original tag0/2 writes coordinate bytes0..7, yaw8..11 and destination tag+22.
  Tag3 writes two IDs0..7, coordinate8..15, yaw10..13 and tag4. The canonical
  `DecoratorInfo::set_destination` preserves the remaining original bytes.
- The wall path unwraps a Wall storage address and existing public wall, calls
  get_wall_attachment_height, computes coordinate.y minus that height, obtains
  world position through the actual curve transform, and writes bytes0..22.
  The current adapter performs those same operations and copies platform,
  corner, bottom-door and side/compound fields. The original height helper uses
  the u input; its incidental XMM2 value is not a required extra y argument.
- Missing wall/storage lookup is a failure, not an invented default. No default
  height, identity, or zero position was found in this helper.
- The available native move fixtures hook the height and world-position callees.
  They prove the destination write protocol, not the shared height/curve library
  algorithms or the complete user move interaction.

## Existing unrelated gap in move_ops.rs

`show_decorator_remove_signifier` still describes/uses the old approximate
Mat3-to-four-floats adapter and marker0, and writes a raw Arc pointer through
SignifierFactory before dropping its local Arc. Those pre-existing statements
are not corrected by calculate_move_dst. Its lifetime/type marker and original
quaternion payload need separate recovery before calling that signifier chain
complete. This is not a blocker to the reviewed widget/type patch itself.
