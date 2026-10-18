# B MEI content producer interface

Oracle is original B tiny-glade.rne SHA256
f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03.
Fresh decompilation and full original mei_post_update instructions are retained
in the text work pack TG-c80526daecad7e4117/meihost-20261018.

At 0x140688941..0x14068899f the +0x60 content virtual method receives:
TextRenderer, graphics device, AssetTextureLibrary, mutable node state byte,
the actual action set, logical window dimensions (height exactly 1000),
Steam handles, scale, actual cursor and elapsed time. The returned optional
16-byte intrinsic size comprises two {percentage bool/float} values; absent
has niche tag 2. A present value is copied to node+0x60 by 0x1406889ba.
It is not a z flag. LabelContent's method is named prepare_layout, not render.

At 0x140689f1a..0x140689f82 the +0x68 method receives content plus a 48-byte
render context: TextRenderer pointer @0; center=position+size*0.5 @8;
extent @16; opacity @24; pixels_per_point @28; cursor @32; elapsed @40;
state byte @44. The 0.5 literal @0x1429258e0 was independently read.
The safe Rust context uses references and semantic fields; no Windows ABI claim.

LabelContent vtable @0x142baf0b8 links +0x60 to 0x14067e510 and +0x68 to
0x14067e560. FrameContent vtable @0x142baf450 links 0x140680540/0x140680590.
Their original producer implementations are restored in mei/widgets.rs rather
than fabricating a generic TextFragment-as-widget implementation. Generic
NodeContent defaults remain empty exactly for absent/default content.

Current node topology/layout/template construction is still a separate
consumer closure. Correct intrinsic sizes and actual packets do not prove
the existing approximate custom MEI layout or missing templates equivalent.

Fresh node-new-decomp.json additionally proves Node+0x60..0x70 is zeroed for a new node; the former 8-byte NodeState was a wrong projection of the preceding style justify byte. The canonical field is [LengthPercentage;2], initialized to zero absolute lengths. Native controller gate 0x14068881c..0x140688847 exposes the Steam client only when SteamInput exists and input_mode.is_controller().
