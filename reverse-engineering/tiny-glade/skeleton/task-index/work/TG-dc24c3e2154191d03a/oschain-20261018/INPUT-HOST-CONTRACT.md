# Original B input → current desktop host candidate

No production or Cargo changes. B RNE/PDB identities remain the project oracle. Full exact native functions and hashes are in native-input-manifest.json; registration full bodies are in TG-40ff43c62ef8791c53/oschain-20261018/host-edit-save-audit/native-registration-manifest.json.

## Registration and current-architecture adaptation

Original backend helper140942c80/763B: set1 chain_ignore_deferred contains commander_update, (control_input_replay, record_and_replay_input_for_trailers), advance_input_replay_frame. reset_input_state is set3. Set5 helper140986ad0/1738B contains ((translate_input_to_cursor_moved,translate_input),input_record_replay,calculate_input_delta,(update_mod_key_state,update_cursor_position)), outer chain_ignore_deferred.

Current InputFrame intentionally owns derived ActionSetStates rather than original raw ActionSetValues. Preserve that architecture: the host's set5 projection must run cursor translation → keyboard/mouse translation → calculate_input_delta → current input_record_replay → (mod_key,update_cursor_position). This ordering difference is an explicit adapter; mounting the native raw-value order over this state protocol would record stale states. Current frame_applied makes the original set1 advance consume only the previous completed frame. seek starts at frame0 directly instead of a native index-minus-one sentinel.

Expose begin_input_frame as an exclusive World adapter so the host can reset the retained private capture policy/suppression before its set1 control/time operations. Do not expose or duplicate CursorCapturePolicy. Do not mount add_input_cpu_consumers in addition to existing host mode/history/time registrations: it contains a different standalone set graph.

register_input_events now uses a contains-resource guard and the real Bevy EventRegistry registration, instead of merely creating Events<T>. Bevy0.16 SubApp::add_event skips registration when Events already exists; therefore the old subsequent host add_event calls did not repair custom event maintenance. The candidate removes the obsolete ModKeyConfig and independent BindingsTable resource initializers and registers the canonical DecoratorOpEvent from its actual owner.

## Binding and gesture restoration

Original KeyBindings serialize140831740/2464B has forty named fields. The first36 and final experimental_camera use Option<bevy_input::keyboard::KeyCode>; three intervening mouse fields use Option<bevy_input::mouse::MouseButton>. Native default14089b110 writes37×12B keyboard options and3×4B mouse options, not40 key/action records. The 0xc3 value is the None niche, not a real unbound keycode. The candidate retains current GameProfile/array architecture with37 keyboard options +3 mouse options and a serde view using dependency enums. country-core's existing bevy_input=0.16.0 needs its serialize feature enabled by the root integrator.

GameProfile is the single configured binding authority; translate_input projects its current value each frame through the existing numeric desktop facade. It no longer reads an independently initialized default table. Some(Left) is a valid mouse binding. Unsupported native scan-code keys and Other mouse numbers colliding with standard numeric codes report an explicit adapter error instead of silently aliasing a different key. These are remaining desktop-facade limitations, not original reconstruction claims.

Original focus gesture1417cd959..a14 reads configured mouse_focus_edit, resets distance on just-press, adds last motion only while held, and tests accumulated distance<3 on release; release-frame motion is not added. Native screenshot1417cdffc..1417ce050 writes digital29 only without Steam and without the Shift modifier. The old generic target table also wrote it unconditionally and bypassed that gate; this duplicate was removed.

Original mod_key140af9c00/506B reads digital15 and digital39, maintains Local previous AppMode/toggle, and resets on mode or canonical DecoratorOpEvent. A mode change does not advance that event reader because of the original short circuit. Candidate is in TG-d38cf1f5d0c9224c56/oschain-20261018 and requires the actual DecoratorOpEvent identity from the other owner. No duplicate event is introduced.

## Sources and checks

Candidates: input_record_replay.rs and integration input/mod.rs in this directory; translate_input.rs under TG-1a60f9e1bb3e2c0662/oschain-20261018; game_profile.rs under boot-authorized TG-d8c2f2c59e98716b0b/main-chain-boot-20261018/oschain-bindings-candidate. Separate manifests freeze exact source/candidate hashes. Existing input_pipeline fixture gets an explicit GameProfile; this is fixture setup, not a new production default that masks missing profile loading.

Added source tests cover profile remapping reaching the action table, configured Left, screenshot gate ownership, lossless named binding serialization, unsupported native input rejection and real EventRegistry rotation/idempotence. Tests were not executed here; root owns unified Cargo validation.

Remaining: Steam device actions still require the existing real producer; legacy profile hint glyph caches are not restored as live binding projections by this input patch; normal canvas/tool/camera/transaction systems remain separate host registration work. This candidate closes device translation and replay boundaries, not the entire editing/save chain by itself.
