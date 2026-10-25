# Shared menu widgets: original B contract and staged implementation

Original RNE SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`. No production source changed and no Cargo command run by this agent. `menu-widget.patch` is a reviewable candidate, not an accepted runtime result. `rustfmt` parsed/formatted the two complete new Rust files; `git apply --check --ignore-space-change` passed (the existing Windows files have CRLF).

## Integration

Apply `menu-widget.patch` from `D:/game` with `git apply --ignore-space-change`, or integrate its four source changes manually at the owner's freeze point. It adds `mei/widgets/menu_button.rs`, exposes the concrete MenuButtonContent, delegates Label's preparation to the shared text helper, adds a safe `MeiContext::new_relative`, and stages the bounded original-font/MEI input test under toolbar/tests.

Interfaces:

* `NodeContext::menu_button(&str) -> NodeContext`; retained concrete `MenuButtonContent { text, color, always_white, action, pending_text }`, with `set_text(String)`.
* `NodeContext::label(impl Into<String>) -> NodeContext`; retained existing LabelContent, text-derived identity.
* `NodeContext::pill_button(name: &str, image: &str) -> NodeContext`; retained existing PillButtonContent, all its preparation/drawing/input behavior unchanged.
* `MeiContext::new_relative(name, relative_to, requested_size, offset, reference_anchor, anchor, opaque)`; delegates the existing root lifecycle and exact Relative payload consumer.

`source-boundary.json` records before/candidate source text hashes. New files also exist separately; `.staged` files are complete review views of modified files, not files to overwrite concurrent work blindly.

## MenuButtonContent coverage

Native vtable `142baf6d0`, size0x260, alignment8. Full vtable entry evidence is retained by menu-interaction's `menu-button-vtable.json` and `menu-button-vtable-{slot}.asm`.

| Slot | Original behavior | Candidate |
| --- | --- | --- |
| 0,18,20,28,30 | TextFragment drop; type-id; concrete Any references; concrete type-name | Rust owning fields/Drop, Any/downcast, type_name; no ABI imitation |
| 38 | Any supertrait vtable | Rust trait upcast/downcast already used by core |
| 40 | Three-byte leaf returns true | interactive=true |
| 48 | One-byte leaf ret | process_input_action returns unhandled, with no mutation |
| 50 | Three-byte leaf returns false | on_input=false; existing phase adapter delegates it |
| 58 | 45B pre_update: action40 means absent; otherwise original MainActionSet.just_pressed(action), then state OR0x20 | Exact same predicate/state write using canonical MainActionSet |
| 60 | 73B prepare shared physically with LabelContent | Single helper prepares the same owning TextFragment and returns absolute two-axis bounds |
| 68 | 95B draw: state bit1 OR always_white selects white; otherwise stored color; packedRGBA opacity scaling; TextFragment render | Same complete branch and existing canonical text/color implementation |

Constructor140f9dba0 uses native MenuButton font role1, original constant142a16e40=`00004042`=48.0, weight700, colorffffffa0, always_white=false, action40. TextFragment owns font Arc and glyph state; pending_text is the current text API adapter applied before the original prepare boundary. It doesn't replace text measurement or rendering.

`NodeContext::menu_button`14068bfc0 hashes the text, then MenuButtonContent TypeId, then parent ID, obtains/initializes the retained concrete owner, sets text, ORs navigation flag0x20 and sets sounds(0,2). It doesn't invent a separate label/image parameter or substitute LabelContent/PillButtonContent.

## Corrections and reuse

* Original `utils::calculate_hash<(u64,TypeId)>`1405bdf20 reads the value at tuple+0 and only the TypeId's high64 at tuple+0x10. Existing source adapters hashing `(value, type_high)` are correct; passing both native TypeId words would be wrong. `hash-value-typeid.asm` preserves the complete393B body. No standard hashing implementation was recreated.
* `label<&str>`14068bf00 and the original dynamic String pagination140f9b789..140f9b7ea both use LabelContent TypeId `{low=dc603d951b9380b2,high=ce69dbf0aee13da2}`. Different text changes the retained ID; String allocation versus borrowed str does not introduce another widget identity. The source API accepts either.
* Label's constructor remains role0,48,white, with no invented default700. Load caller may set700; icon caller may set32, independently of a date button.
* Generic pill140fa0130 defaults80x60, align2/justify1, nav flag0x20 and sounds(0,2). The caller's contextual65/60 and strength0.6 are caller overrides. The existing full PillButtonContent is reused, not copied or replaced.
* `back_button`1406811c0 is813B, not the early305B truncated range. It starts with generic pill, then sounds(0,13),50x50 and clears nav0x20. Existing source already preserves those semantics; no back_button change needed.

## Bounded acceptance candidate

`original_menu_widget.rs` uses actual original font table and all installed font files (TG_INSTALL_ROOT required; no skip). It constructs a real nonempty MenuButtonContent and executes the actual `mei_pre_update` system for idle, pointer hover, cancel shortcut, mouse click and controller confirm. It checks state0x10/0x20/hover, actual UiState registration path, emitted sounds(0/2), and owner reuse across the native recycle boundary. It then generates actual glyph primitives with that owning TextFragment and invokes the actual content draw for opacity/color output.

This candidate has not been compiled or run. It is limited to pre-update/widget ownership and actual CPU text primitives. The complete Load/Escape UI caller, controller navigation across multiple siblings, prepared layout through mei_post_update, atlas upload/GPU draw and full application interaction still need central integration validation. No success or full-chain claim is made.

## Efficiency lessons confirmed here

1. Read every virtual slot and original getter/constructor, not only the named draw. Tiny leaf functions may have unrelated folded symbol names; bytes determine the widget's real behavior.
2. Confirm shared physical function bodies before duplicating algorithms. Label/MenuButton share prepare; retained owning TextFragment and existing shaping are the correct reuse point.
3. Separate defaults from caller mutations. Pill's80x60/1.0 differs from contextual65/60/0.6 and back50x50. A global approximation would change multiple UI paths.
4. Inspect the actual hash helper. A128-bit TypeId object at a caller does not prove both halves enter Hash; both String and &str can lead to the same concrete widget TypeId.
5. Stage a small integration patch while the central Cargo owner compiles. Preserve source hashes, parse with rustfmt and check applicability; neither these checks nor a prepared test counts as runtime acceptance.
