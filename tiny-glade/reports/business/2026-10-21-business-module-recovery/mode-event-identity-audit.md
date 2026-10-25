# Mode-change event identity audit

Status: direct native evidence reviewed; complete eight-file candidate staged, production sources frozen. Rustfmt syntax parsing passed; candidate not mounted, compiled, or executed. Owner: toolbar_lifecycle.

## Proven defect

`mode_manager::send_mode_just_changed_event` writes `AppModeChanged { current, previous }`. Toolbar, onboarding, and color readers subscribe to the distinct `resources::events::ModeJustChanged { tool, _gap }`. There is no production writer or bridge for the latter event. Thus the actual publisher cannot update toolbar mode-button selected/animation state, close the TOD/color menus, or notify those tutorial consumers. Hand-written ModeJustChanged events in existing unit tests hide this integration break.

## Native payload identity

Direct source: `reverse-engineering/tiny-glade/evidence/country-core-deep/ui-input/decomp/decomp-0x140a875e0.txt`.

- `puVar2 = *param_1` is the current mode; `puVar3 = *param_2` is the previous-mode resource.
- Compare both complete AppMode values with native equality.
- On difference, write `puVar2[0..2]` at event offset 0; write `puVar3[0..2]` at offset 0x10; write EventId at offset 0x20. EventInstance stride is 0x28.
- Finally assign `puVar3[0..2] = puVar2[0..2]`. This assignment fixes current/previous direction independently of guessed names.
- Therefore business payload is **current AppMode + previous AppMode**, each native 16 bytes. No u32-only bridge, zero padding, or parallel event is appropriate.
- Existing ui-input/DIGEST.md labels the two modes backwards and describes `current := prev`; the actual final assignment disproves that terminology. Existing resources/events.rs claims tag at +8; the producer and direct consumer dereference place current native tag at event +0.

Proposed canonical contract: retain the existing semantic `mode_manager::AppModeChanged` with both full typed values; `resources::events` re-exports it as `ModeJustChanged`. Rust TypeId then matches across both names. Current compiler-private AppMode layout is deliberately not assumed to equal native layout. Existing history/replay and photomode readers keep both semantic values untouched.

## Consumer migration inventory

| File | Required change |
| --- | --- |
| country-core/src/resources/events.rs | Replace wrong independent struct with canonical re-export and corrected evidence comment |
| systems/toolbar/src/toolbar_button.rs | TOD last event `.tool` becomes `.current.native_tag()`; mode/category consumers use event presence and require no algorithm change; migrate embedded test literal |
| systems/color/src/autoclose_color_menu.rs | Last event `.tool` becomes `.current.native_tag()`; preserve latest-event and escape ordering |
| systems/onboarding/src/hint_focused_edit.rs | Current native tag 1/2, with latest event rather than first |
| systems/onboarding/src/hint_clutter_focus_edit.rs | Current native tag 3, with latest event rather than first |
| systems/onboarding/src/exponential_back_off.rs | Mode reader current native tag 1/2, with latest event rather than first; other unrelated unknown reader remains unknown |
| systems/toolbar/tests/tb_e_toolbar_buttons.rs | Update obsolete old payload construction; narrow consumer test is not physical integration evidence |
| systems/toolbar/tests/tod_focus_event_order.rs | Publish actual AppMode transitions through real publisher, preserving the two-buffer ordering cases |

Registrations under either spelling become the same Events resource. Production host has both spellings registered. Checked fixed dependency bevy_app 0.16.0 `src/sub_app.rs:339`: `add_event<T>` calls EventRegistry only when `Events<T>` does not already exist, so both spellings are idempotent after the alias and do not cause double frame maintenance. Independently, an earlier plain `init_resource<Events<T>>` can suppress registry insertion; that pre-existing ordering concern remains the host event-runtime owner's responsibility. No host mutation is proposed in the frozen window.

## Additional original algorithm correction

The three onboarding readers currently call `.read().next()`. Direct native files below all compute the last unread record as `data + (remaining - 1) * 0x28`, preferring the newer event buffer and falling back to the older one, then advance the reader across the full batch:

- `systems-recovery/groupB/decomp/decomp-0x14227da70.txt`: focused edit, `*piVar5 - 1U < 2`.
- `systems-recovery/groupB/decomp/decomp-0x141c3d890.txt`: clutter focus, `*piVar6 == 3`.
- `systems-recovery/groupB/decomp/decomp-0x142284800.txt`: backoff B, `*piVar11 - 1U < 2`.

All three require `.read().last()`; a mere field rename would retain incorrect batch semantics and cause stale tutorial completion/backoff actions. No other event reader is changed without its own evidence.

## Verification contract

The required integration test uses real MouseButtonInput through the canonical input/UI chain, actual mode request consumer, actual publisher, then original toolbar selected/animation consumer. It must not send ModeJustChanged by hand. Separate narrow tests should retain both full AppMode values (including entity/decorator payload), avoid publishing unchanged modes, and cover latest-event behavior across both Bevy event buffers. CPU success does not establish window input, visible render, complete save/history, or the 60 FPS minimum / 120 FPS qualification standard.

## Staged delivery

`mode-event-migration.patch` compares each complete final candidate against its live source. `mode-event-migration.json` records live SHA256, intermediate candidate SHA256, and final SHA256. The focused/clutter onboarding files include both the earlier canonical PlayUiSound migration and this event migration. These two final files supersede the sound-only candidates; **do not overwrite them by reapplying an old whole-file sound candidate**. Other seven sound-only files are unchanged. No live file was modified, and the staging script verified all eight live SHA values after writing.

Regression candidates:

- Existing toolbar mode-button integration test now uses the real publisher for notification, retaining its explicit click-consumer boundary.
- New toolbar test checks identical TypeId, complete high-bit wall/decorator payload in both directions, and unchanged-mode suppression.
- Existing TOD latest-unread-across-two-buffers test now changes actual AppMode and invokes the real publisher; no raw tool event is forged.
- New onboarding test passes real published target→Path and Path→target transitions to both focused/clutter consumers across two buffers, checking no stale hint and no repeated consumption.
- New backoff test passes real published transitions and checks cooldown remains 800 for stale qualifying modes, increases to 1200 once for the latest qualifying mode, and remains 1200 on another consumer run.

Queued commands for the central compile window (not executed by this owner):

```text
cargo test -p toolbar --test tb_e_toolbar_buttons --test tod_focus_event_order --locked --offline
cargo test -p system-onboarding --lib canonical_mode_ --locked --offline
```

The physical-input candidate maintained by toolbar_input is separate. The migrated embedded toolbar unit test remains a narrow consumer-only test and is not claimed as publisher or physical-input evidence.
