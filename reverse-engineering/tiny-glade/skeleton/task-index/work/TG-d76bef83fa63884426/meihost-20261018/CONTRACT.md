# B Taffy layout closure

Original B RNE range perform_layout @0x141e4c6c0, 873 bytes; helpers
make_layout_absolute @0x142374e10, 382 bytes; style setter @0x142376120,
269 bytes; unsigned order comparator in sort helper @0x140fe4110.
Full original instruction slices are retained beside this contract.

TaffyLayout owns pinned taffy 0.3.13 plus a Vec<AnchoredNode> (stride 40).
The records contain unsigned order @0, offset Vec2 @8, node id @16,
reference node @24, gap f32 @32, raw side byte @36. Side 4 sizes the root
to actual WindowSize then computes max-content and makes all descendants
absolute using the supplied offset. Other sides compute max-content, read
the actual reference and child layouts, then position above/below/left/right
(raw sides 0/1/2/3). Gap is multiplied by config.ui.icon_size_mult times
ToolbarSizeMult. Records are sorted by unsigned order before layout.

make_layout_absolute adds parent origin to the actual selected taffy layout,
marks the node dirty, and recurses with its resulting absolute origin.
The dependency LayoutTree API exposes this directly; no private Rust layout
or copied native Taffy struct is required.

Finalizer @0x142340890 reads TaffyNode tail as base size Vec2 @8 and one-frame
size multiplier @16. Result is ((base * node_multiplier) * optional user
icon multiplier) * toolbar multiplier, component-wise max with actual prepared
TextFragment bounds when present. It writes Length sizes through style clone
and set_style, then resets the one-frame multiplier to 1. Existing opaque tail
is decoded safely; this is not an invented default for its missing producer.

The remaining two systems are real layout-to-Ui2dExtent @0x142340d20
(center=location+size*0.5, size copied) and debug_hide_ui @0x14233e960
(when DebugGui.show_2d_ui is false, set only extent.size to zero).
Update registration is set 44, in order prepare_text_fragments → finalizer →
perform_layout → update_drawables → debug_hide_ui → render_text_fragments.
