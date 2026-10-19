# Fullvisual value equality and canonical fields, independent source sub-batch

This is the root-authorized implementation of the already reviewed native body design's value-equality gate. It does not implement or register fullvisual, fabricate the three shared dependencies, or claim a running frame. Only `blueprint.rs` in the existing legal claim is a source candidate.

Before is production `c86444c77352782c7572396e16886e55ff2820c56e16de7c2536d927a346b07b`. The patch adds actual `Vec<RawWallHole>` and `Vec<country_core::resources::walls::dormers::DormerHoleSsbo>` fields to the existing blueprint; original empty defaults are kept through the current constructor. It does not create a second DormerHoleSsbo or change core's GPU words/TypeId/constructors. The original 48-byte list comparison projects words 3 and 7 as exact integers, with all ten remaining words interpreted as ordered f32 equality. Its actual producer/upload consumer remains a separate recovery boundary; the existing local decorator stand-in and footprint approximation are not endorsed by this patch.

`visual_builder_value_eq` projects the native helper's actually represented fields, not the current host struct's derived PartialEq. Exact original scope is `0x1412a92a0/2178B`, PDB module1557 record264, byte SHA `aafd835ab4d3195492fde4d4419c6cd857d601213b7b5a19ea63420e3d3af66e`. Complete assembly and identity manifest remain in the frozen sibling `fullvisual-v1/evidence`. NameHash::eq at `0x140c9cd70/10B`, module770 record252, is an exact u64 comparison; the new ten-byte assembly is in this package's evidence directory.

| Current represented field | Original equality projection |
| --- | --- |
| translation, rotation, scale, normal_offset, scale_override | ordered float values, including Option state |
| mesh_name, nani_name | byte-string contents/length |
| normal_map, albedo_map | Option state and exact 64-bit NameHash |
| color_id, window_light, roof_flag_cloth, semi_transparent, balcony_door | actual integer/boolean/Option values |
| gothic_window_brick_deformation | fifteen ordered f32 values, every word in its 60-byte payload |
| brick_color 24-byte payload | two integer words, three ordered f32 words, low tag byte at20; bytes21..23 are padding and ignored |
| clothline_cloth `(u64,u32)` | first64 bits are two ordered f32 values; last32 bits are an exact integer |
| sittable_for_birds `(u64,u32)` | three ordered f32 values, including the last32 bits |
| wall_flag_cloth `(u64,u64,u32)` | first low32 integer, first high32 and both second64 halves ordered f32, final low8 tag; remaining final24 bits padding |
| smoke, solid_texture | exact active payload/tag values, with original None markers respected |
| lantern_light_params | four ordered f32 values and Option state |

The wall-flag payload correspondence is additionally checked against the existing complete native persistent build: `0x1412aad27` takes `self+0x12c`, then `0x1412aad62..72` copies its contiguous20-byte payload. Some older builder comments starting the payload at +0x130 were imprecise; this sub-batch does not modify that owner's file. The current three-tuple serializes that same contiguous20-byte payload in its build, so its exact comparison can be expressed without a new field or invented tag.

Original native None representations are BrickColor tag2, WallFlag tag2, Smoke tag2 and solid selector9. The projection checks those markers before payload, as the original helper does. This is evidence-backed option representation, not a newly defaulted business-off branch. It does not make an otherwise invalid host builder input successful in `build`, nor does it recover unresolved custom setters.

Host-only `decorator_id`, `preview`, `window_glass` and `shader` are not appended to native equality. The complete native helper reads no separate fields for those current convenience values. In fullvisual the actual map ID is explicitly passed into the persistent build, as the original call does. The current custom-setter/native-field mapping remains an existing recovery gap; changing this equality does not claim those setters recovered.

Five new tests call the actual candidate helpers and current WindowVisualEntityBuilder. They cover canonical dormer mixed words, the 60-byte Gothic float payload, BrickColor/WallFlag padding, bird/clothline differing final-word semantics, and actual name/outer-ID distinction. They have not run. `rustfmt --edition 2024` parses and formats the candidate successfully. No Cargo/typecheck/GPU/fullgame result is claimed, and the helpers are not yet wired to a fullvisual function in this sub-batch.
