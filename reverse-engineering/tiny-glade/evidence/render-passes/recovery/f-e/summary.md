# F-E summary — entry/small family recovery

Scope: 32 reconstructed HLSL files (blit/, skydome/cloud/star, ui_2d*, text,
egui, custom_cursor, dashed_line, timer_circle, loading_screen, logo_*,
session_selection/, lut_gallery/, screenshot/, fxaa, capture_panic,
resolve_to_swapchain). All compile with dxc 1.10.2605.37, canonical flags.

## Result
- Bindings + entries + LocalSize: **53/53 modules match** (name, set, binding,
  storage, kind; entry names; numthreads) — the hard gate is fully met.
- +Capabilities: 52/53. One miss (fxaa.cs): the shipped module has
  `StorageImageExtendedFormats` because the original dxc defaulted
  `RWTexture2D<float3>` to ImageFormat=R11fG11fB10f; installed dxc uses Unknown.
  Not expressible in HLSL — toolchain-version artifact.
- Full structural fingerprint: 4 modules (blit_stretch VS, resolve VS,
  resolve_msaa VS, logo_animation VS). The other 48 differ only in the opcode
  histogram, from dxc-version codegen idioms: OpSwitch from if/return helpers,
  shuffle-vs-construct output packing, int2/uint2 bitcast folding, OpSource count
  drift. Every raster bin ships two variants; the f0ad PS is an empty stub
  (unreproducible from one source) — targets use the full b903 variant.

## Status
31 files O, 1 file I (fxaa.inc). Top unknowns: text shape-stream op encoding +
glyph-dither octave mapping; skydome sun-glow constants; star/cloud per-instance
hash-chain depth; painterly thumbnail scatter/rotation tables.

## Inc-layer issues (main agent to adjudicate)
1. `$Globals` binding (b13/b14/b15) needs an explicit `cbuffer $Globals` in each
   main; inc/current_view.hlsl declares `current_view_index` as a plain global
   (lands at b0 and redefines the member). Consumers include frame_constants.hlsl
   directly and declare `$Globals` in-file.
2. `RuntimeDescriptorArray` appears in every shader including frame_constants.hlsl;
   the inc layer declares no bindless array, so each such main re-declares
   `Texture2D<float4> bindless_textures_dyn[] : register(t0, space2)` (unused).
3. inc/color/srgb.hlsl if/return lowers to OpSwitch; shipped shaders use
   if/else-assignment. Consumers inline the per-channel decode.
4. inc/blue_noise.hlsl lacks `register(t14, space0)`; text.raster redeclares it.
