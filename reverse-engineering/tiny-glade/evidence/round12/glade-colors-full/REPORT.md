# GladeColorCustomization: complete original wire checkpoint

Candidate only. No production writes or Cargo execution. `candidate-checkpoint.json` records raw-byte source/proposed SHA256 for all ten files; `production-review.patch` passed `git apply --check` and every candidate passed rustfmt parsing. Shared source files must be merged as narrow hunks, not whole-file replacements.

## Original authority

Original RNE SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03. `native_gcc_full.py` executes the original Default 1408c5600/82B, JSON serializer 14085ee90/880B, Value serializer 14085f200/844B, Value deserializer 1415a0130/511B and field visitor 14085f550/634B. Only allocator and CRT boundaries are hosted; game/serde control flow executes original instructions. `original-gcc-full-fixtures.json` contains all raw results.

The original fifteen keys are freeform_wall, building, roof, building_wall_style, freehand_ty, halftimber, lantern_light, fences, window_light, flag, windows, gates, flat_rim_style, lantern_brightness, stairs. Colors are ColorId; flag is a color, windows is Option<ColorId>, fences is plural. Wall style, freehand type and flat rim serialize named enums. Stairs owns stairs_color, ladder_color, railing_ty and magic_color, rather than one color.

## Evidence and exact defaults

192 native cases cover 3 wall styles x 2 arch types x 2 rim styles x 4 railing styles x 4 window cases (None, Some(0), Some(8), Some(MAX)). Each original JSON result was independently passed through original Value serialization and original Value deserialization, then original JSON serialized again. Original field visitor resolves all fifteen names; singular fence and unknown names are ignored as unknown fields.

Original default JSON:
```json
{"freeform_wall":0,"building":8,"roof":0,"building_wall_style":"StoneWithSkirts","freehand_ty":"FollowTerrain","halftimber":0,"lantern_light":0,"fences":0,"window_light":0,"flag":0,"windows":null,"gates":0,"flat_rim_style":"Rim","lantern_brightness":1.0,"stairs":{"stairs_color":100003,"ladder_color":0,"railing_ty":"None","magic_color":0}}
```

Nineteen missing-field probes mutate an original owned Value map key and execute the original deserializer. Only windows, flat_rim_style, lantern_brightness, stairs and nested magic_color have defaults. The remaining eleven top-level fields and three required nested stairs fields return original errors. Empty GCC map also errors. No blanket serde(default), invalid-enum swallowing or wrong-type fallback is introduced.

## Candidate interfaces and consumers

Canonical types: `inner_wall_state::{WallStyle,ArchType}`, `roof_shape::FlatRoofRimStyle`, `decorator::StairRailingType`, `glade_color_customization::StairCustomization`. Existing WallStyle wrapper and established compatibility deserializer are retained; serialization emits original names. Enum native_tag/TryFrom preserve the existing numeric gameplay boundary. Draw wall, wall color selection, circle/rectangle roof construction and the load initial-colors projection are migrated together.

The new oracle test module checks actual original JSON/default and missing-field vectors plus typed field values and rejection of the previously incorrect schema. Rust candidate tests have not been compiled or executed; root owns the build checkpoint. T16 UI producer integration and session/load/save App lifecycle require separate owner validation. This checkpoint proves original schema/default policy and supplies source candidates; it does not claim complete startup/GPU behavior.
