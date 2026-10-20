# ui_nav_update next recovery evidence (read-only, no implementation)

Original B 14233eed0, PDB body6017B, exact end142340651. Original run_unsafe141ea3650/vtable::name identifies six canonical SystemParams (see ui-nav-signature.txt):

1. Query<(Entity,Option<&core::Ui2dExtent>,Option<&core::UiInteractionState>,&core::UiNav)>
2. Query<(Entity,&toolbar::TaffyNode,&core::UiNav)>
3. Res<ActionSetStates>
4. Res<TaffyLayout>
5. ResMut<UiNavState>
6. EventWriter<UiClickEvent>

Canonical UiNav belongs in country_core::components, not a second toolbar component identity. Current typed four-slot links preserve the known original tags and payload, but full tag dispatch must be recovered from the bounded original control flow.

The old all-b4 decompilation is unreliable after its indirect jump: it includes fake reads/writes/in/out instructions in read-only14304de/e0 data outside the procedure. The exact-bounded original objdump is ui-nav-original-bounded.asm; do not translate the corrupted pseudo-C switch as an original game algorithm. Resolve the actual jump tables and callees first.

Full original entry executed for80 empty-query scenarios via native Bevy query protocol (same world harness used for focus verification), native-nav-empty-query-gates.json + probe-nav-gates.py. Both queries are genuinely empty; no navigation/business helper is hooked/replaced. These establish only entry gates:

- current focus=Mei(tag2): clear staged focus tag, preserve current Mei; no click.
- a stored Toolbar(tag1) focus survives idle; Confirm19 or an uncancelled directional action restores staged->current and clears staged.
- simultaneous opposite left/right cancels movement and does not restore staged focus.
- Cancel18 uses just_released, copies current toolbar focus into staged then clears current.
- Direction slots20left/21right/22down/23up are read at native offsets a0/a8/b0/b8; opposing just_pressed inputs cancel each other. Native DigitalActionState::just_pressed is called, so repeating timer semantics must survive.

Remaining substantial algorithm: visible eligible root selection (optional state and extent), current-entity validation, UiNav variant dispatch, candidate ordering, nearest/spatial selection, Taffy hierarchy ascent/anchored cross-root traversal, subtree iteration and click emission. An implementation containing only the above gates would still be partial. Main dependency bodies include141ea8c60 iter_subtree plus its closure; proper reconstruction needs full native route fixtures over real node trees and component combinations.

No production or exported test was changed for this read-only navigation analysis.
