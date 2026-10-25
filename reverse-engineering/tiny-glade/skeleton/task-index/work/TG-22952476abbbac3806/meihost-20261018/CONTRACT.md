# B LabelContent and FrameContent

Original B RNE SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03.
Vtable identities are proven by Label/Frame type_name entries, native sizes
0x260/0x30, and actual +0x60/+0x68 consumers in mei_post_update.

Label prepare_layout 0x14067e510 calls its actual TextFragment::prepare with
the context renderer, then returns bounds as two absolute Length values.
Label render 0x14067e560 reads color at +0x258 (after the 600-byte fragment),
scales it by context opacity with utils::scale_u32_packed_srgba_color,
then calls TextFragment::render(center, extent, tint, renderer).

Frame prepare_layout 0x140680540 clears cached RawShapeStream then appends
the true ShapeStream with AssetTextureLibrary image_name_to_index resolver;
returns two absolute zero Lengths. Frame render 0x140680590 checks original
ShapeStream nonempty, converts context center to min corner, and submits
TextRenderer::rect(min, extent, opacity, actual raw shapes). Empty frames
submit no rectangle, rather than drawing a fabricated white box.

Public fields are safe ownership adapters for known native producer values;
no default label/font/shape content is generated. Other widget types and
template instantiation are outside this bounded closure and remain missing.
