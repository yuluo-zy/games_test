# WP4 Contract: `country_core::render::view_constants` / `frame_constants` / `directional_light` / `atmosphere` / `load_texture`

## `view_constants` (source_sha256 `3c00f8e4…`, 2 procs)

- `ViewConstants` — 0x320+ bytes (O: `ViewportConstantBuilder::build` writes to +0x31c). Layout (O): +0x00..0x7c two 4x4 matrices (view + something), +0x80..+0xfc matrices (proj/inverse view), +0x100..+0x17c view-proj matrices, +0x180..+0x1fc more, +0x200 u32 x/y (pixel offset), +0x210 viewport size u32 pair, +0x304..+0x31c scale factors.
  Accessor hosts (inline): `builder()`, `eye_position()`, `prev_eye_position()`, `fov_y_radians()`, `near_plane()` (I: trivial field reads).
- `ViewportConstantBuilder` (0x220+ bytes): holds eye pos, matrices, pixel offset, viewport size.
- `ViewConstants::set_pixel_offset(&mut self, pixel_offset:(f32,f32), viewport_size:(u32,u32))` (0x140b18af0, 1131B): ndc_offset = offset*2/viewport; rebuilds projection columns (shear on x/y translation), inverse proj and view-proj accordingly; stores offset + viewport. O.
- `ViewportConstantBuilder::build(&self) -> ViewConstants` (0x140b18f60, 2339B): full matrix chain — proj×view, inverse matrices, prev view-proj (temporal reprojection), pixel-offset application; writes the ~0x320-byte struct. O. (Exact matrix set and interleaved 0x40-stride stores noted; precise per-column mapping is heavy and left as Inferred in source.)
- `builder()` (inline host, 4 sites). I.

Descriptor-UBO contract (from `evidence/render-passes/INDEX.md`): `ViewConstants`/`FrameConstants` are written to the `frame_constants` UBO (set1/b0).

## `frame_constants` (source_sha256 `abf41da9…`, 1 proc)

- `FrameConstants` — UBO payload written by the game each frame: main view + sun cascade views. Observed members from PDB names: `main_view()`, `sun_view()`, `should_reset_temporal()`, `RenderViewIndex::{is_shadow_view, sun_cascade}` + a `TREE_SEE_THROUGH` fmt const.
- `FrameConstants::should_reset_temporal` (0x140976880, 11B): trivial accessor returning the reset flag (O: empty body = register pass-through).
- `RenderViewIndex` — u32 view slot index; `sun_cascade()` returns cascade id; `is_shadow_view()` true for shadow views (I from names).
- `impl$15::fmt TREE_SEE_THROUGH` — display string const for the tree-see-through toggle. O/I.

## `directional_light` (source_sha256 `69166bac…`, 2 procs)

`DirectionalLight` (~0x78 bytes, O from `look_at`):
- +0x00 view-basis Mat4 (rotation columns + translation row, w=1)
- +0x40 position Vec3 (+w reused as packed pair)
- +0x50 target Vec3 / +0x60 (1.0 + target)
- +0x70 distance f32

- `look_at(&mut self, eye: Vec3, target: Vec3)` (0x140b18550, 725B): builds look-at basis (right/up/forward normalized), translation = -dot(basis, eye-ish), stores position + target + distance=|eye-target|. O.
- `set_direction_towards_light(&mut self, direction: Vec3)` (0x140b18830, 695B): position += normalize(direction)*distance; rebuilds look-at basis toward target; stores new position. O.
- `translate`, `update`, `view_to_clip` — inline hosts (U bodies; names O).

## `atmosphere` (source_sha256 `8ba1e09e…`, 3 procs)

`SunSkyColors` — 48 bytes = 3×Vec4 (O: `lerp` reads/writes [0..11] f32).
- `lerp(&self, other: &SunSkyColors, t: f32) -> SunSkyColors` (0x140a97650, 87B): per-component `self*(1-t) + other*t`. O.
- serde `Serialize` host (0x140322a10). O.

## `load_texture` (source_sha256 `a31916ce…`, 1 proc)

- `load_compiled_texture_image(path: &Path, …) -> Result<Texture, _>` (0x140917860, 2323B):
  1. puffin scope `"load_compiled_texture_image"`.
  2. Build path: `Path::to_path_buf` + join + `with_extension` (compiled-image path).
  3. Log debug; `std::fs::OpenOptions::open` (panic on io error), `memmap2` mmap + file_len.
  4. Validate header: aligned length, magic `0x48502183`, field +4 == 8, tail mips u16 < 0xff.
  5. `rhapsody::raw::texture::TextureDesc::new().sampled().mip_levels(..).array_elements(..)`; `ustr::Ustr::from(path)`; `GraphicsDevice::create_texture<&[u8]>` (reuse); unwrap.
  6. Return 0x40-byte `Texture` (rhapsody::raw::texture::Texture, reused). O.
- The exact compiled-image header field meaning (len-0x14 / len-0xc / len-8) is O-bytewise but U-semantics.

## Typed gaps
- `ViewConstants` per-matrix column mapping; `FrameConstants` full UBO field list (uses shader reflection / catalog conventions).
- `DirectionalLight::translate/update/view_to_clip` bodies.
- Compiled-texture magic/header format spec.
