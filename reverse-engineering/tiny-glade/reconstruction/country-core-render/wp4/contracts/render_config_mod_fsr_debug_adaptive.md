# WP4 Contract: `country_core::render::render_config` / `render::mod` / `fsr_plugin` / `debug_info_buffers` / `adaptive_render_state`

## `render_config` (source_sha256 `2511e6da…`, 24 procedures, modules 9/34/74/80/…/504/516)

Types from PDB symbols: `FullscreenConfig`, `UpscalingPreset`, `VideoSettings`, plus serde-serialized types `MonitorMatch`, `DisplayConfig`, `UpscalingMode`, `LastKnownGraphicsDevice`, `VideoMode`, `ImageQualityPreset` (observed in public-symbols / enriched lists).

- `FullscreenConfig { mode: FullscreenMode, resolution: (u32, u32) }` (I from `resolution`).
  `FullscreenConfig::resolution(&self) -> Option<(u32,u32)>` (0x140968ad0, 26B): `(mode == Fullscreen).then_some(self.resolution)`; discriminant stored as `iVar1==1`, pair copied on Some. O.
- `UpscalingPreset::tgsr_resolution_scale` (0x140968bb0, 16B): trivial — returns the TGSR scale for the preset (O: empty body; exact value unknown → typed gap).
- `VideoSettings::upgrade` — inline host only (compiled=0): migration of saved settings to current schema. U.
- `VideoSettings`, `MonitorMatch`, `DisplayConfig`, `UpscalingMode`, `LastKnownGraphicsDevice`, `VideoMode`, `ImageQualityPreset` — serde RON (de)serialization + `enum_iterator::Sequence` (next/previous for settings cycling) confirmed by PDB hosts. Layouts U (typed gaps).

## `render::mod` (source_sha256 `825e8980…`, 3 procs)

- `AvailableGraphicsDevices` — `Vec<AvailableGraphicsDevice>` (160 B/element, O `from_preference_order`).
- `AvailableGraphicsDevices::from_preference_order(devices)` (0x140987cb0, 280B): reverses the preference list in place (pairwise 0xa0-byte element swaps). O. Consumer: device selection at startup (I).
- `impl$0::deref` / `impl$1::deref` (0x140987b30-ish): `Res<GraphicsDeviceResource>` deref to `rhapsody::raw::device::GraphicsDevice`. O (bevy resource Deref).
- `GraphicsDeviceResource` — bevy resource wrapper around `rhapsody::raw::device::GraphicsDevice` (O: deref hosts; reused rhapsody).

## `fsr_plugin` (source_sha256 `50fdaace…`, 1 proc, module 480)

- Single procedure; PDB names surface absent in symbol dump (enriched empty). Likely a bevy plugin wiring FSR (reused `fsr`/`fsr-sys`) history/reactive compute passes (I from `evidence/render-passes/INDEX.md` W52 note: FSR history/reactive only). No standalone business proc recovered → typed gap.

## `debug_info_buffers` (source_sha256 `2256d3b8…`, 0 procs, module 480)

- No procedures overlap this file (all logic inlined). PDB symbol surface absent. U — typed gap; module retained with empty body + doc.

## `adaptive_render_state` (source_sha256 `39a05efa…`, 0 procs, modules 194/240)

- No procedures; participates in `render_config`-adjacent enums (module 194/240 shared with render_config). Likely adaptive-quality state (resolution scale lerp). U — typed gap; module retained with doc + placeholder struct.

## Typed gaps
- Exact enum discriminants / serde field layout for VideoSettings & friends (would need RON file samples + settings consumers).
- `tgsr_resolution_scale` numeric return; `VideoSettings::upgrade` body.
- `AdaptiveRenderState` struct fields; `debug_info_buffers` contents; FSR plugin wiring.
