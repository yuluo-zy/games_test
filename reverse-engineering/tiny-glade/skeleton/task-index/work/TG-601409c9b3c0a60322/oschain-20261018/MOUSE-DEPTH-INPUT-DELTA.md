# B CPU mouse depth + actual input frame delta

Candidate only, not compiled. B oracle identity retained in TG-a544/oschain mouse-depth-native-manifest.json; native readback140a1a480 full1390B and constructor140940e10..e30 in full898B insert_resources.

CPU56B is six semantic groups, not the 64B GPU SSBO. Copy depth finite independently then four finite xyz groups in source order; bad records retain old group. DisableInput3 returns before any mapped access. Auto/Records write three values into the actual just-recorded InputStream frame; Replay restores the same three values from the actual current frame; view depth uses inverse projection after override. Bounds failures preserve the previously copied valid prefix, no invented empty mapping.

Current refactored versioned JSON format changes2→3 to carry those exact three values. Old incomplete files are rejected, not zero-filled. This is a current API adapter, not a claim to restore native208B file ABI. Frozen input batch remains untouched; apply only this manifest's incremental input_record_replay patch after that batch.

Host must export resources::read_mouse_depth_value, insert MouseDepthValue::new exactly with backend resource constructor and register full original Update10 tuple under gameplay-active. No Last/PostUpdate insertion. Camera initialization and ray producer are separate same-chain candidates.

Tests: original_finite_groups_keep_the_previous_values_independently; short_mapping_panics_after_writing_the_actual_valid_prefix; replay_overrides_only_the_three_original_recorded_values; file_round_trip_keeps_every_frame_cursor_and_action now includes nonzero depth/positions; incomplete_depth_recordings_are_rejected_instead_of_filled.
