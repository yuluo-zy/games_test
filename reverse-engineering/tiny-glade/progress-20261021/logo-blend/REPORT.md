# Loading Logo transparency: restore the native UI blend state

## Observed defect

Central GPU execution of `original_loading_gpu` exported
`D:/game/tmp/logo-loading-fixture-960.png`: the packaged green Vermeer background
was visible, but the central Logo quad was a 243×243 black rectangle with alpha
zero at initial Logo progress `[0; 5]`. The full image had 11.38% pure-black pixels.
The previous `any RGB != 0` assertion passed, so it could not detect this defect.
This is a final loading target after the real Vermeer/MEI/Logo passes; the bounded
fixture does not exercise asynchronous gameplay handoff or the welcome menu.

## Original evidence

`probe.py` decodes only PDB-bounded procedures from original B executable SHA-256
`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.
`manifest.json` records the input, boundaries and body/disassembly SHA-256 hashes.
The PDB ledger is `evidence/country-core-audit/data/procs.tsv`.

| Procedure | PDB start / bytes | Observed blend state write |
|---|---|---|
| `ui_pass` | `0x140837c40` / 929 | `0x140837ee2..0x140837eed` |
| `loading_screen_pass` | `0x140837ff0` / 929 | `0x140838292..0x14083829d` |

Both procedures, **after `bind_common`, before calling the draw-list callback**:

```text
movzx eax, byte ptr [rbp + 0x84b]
and al, 0x9f
or al, 0x40
mov byte ptr [rbp + 0x84b], al
```

The packet begins at `[rbp+0x7a0]`; `0x84b-0x7a0=0xab` is byte3 of
`DrawPacket.state` at +0xa8. Thus only blend-mode bits 29–30 become `0b10`.
The default is `0x1ffc00d0` (`raster-default.asm`, native `0x140be6750`).
Existing current `rhapsody::raw::shader::get_vk_blend_mode` decodes mode2 to
enabled ADD, source ONE / destination ONE_MINUS_SRC_ALPHA for color and alpha.
Its existing implementation is preserved. Transparent RGBA=0 therefore preserves
the background. This matches the native state; no shader workaround is required.

`env37.asm` and `env38.asm` additionally show the Logo callbacks bind textures and
progress uniforms, then draw. They contain no blend mutation. The original
startup tuple is unchanged; no invented producer component is added.

## Implementation and regression

- `country-core/.../ui_pass.rs`: replace the erroneous `Plain` mode with `Ui` for
  both native UI list passes and correct the former “no state modification” comment.
- `country-core/.../d_main_support.rs`: `ItemMode::Ui` applies the existing exact
  `RasterState::set_byte3_field_2()` after common bindings, before each callback.
  No depth/cull/topology state is changed; no ordinary 3D pass mode is changed.
- `d_main_tests.rs::ui_and_loading_screen_use_their_lists_and_premultiplied_alpha_state`:
  both actual pass builders retain correct list selection and emit the exact
  native transformed state for every draw (`0x80000080 → 0xc0000080`).
- `original_loading_gpu`: retain all existing assertions; require the actual
  final loading pixels to remain alpha255 at the fixture's initial Logo progress.
  This catches the former transparent black rectangle over an opaque background.

No changes to rhapsody, shader bytecode/HLSL, graphics-host state decoding, clear
colors, Logo meshes, animation timing, background selection or pass order.
No alpha discard, clipping, replacement image or pass bypass was introduced.

## Validation boundary

The subagent performed source/native review only, without Cargo, GPU or GUI runs.
The central agent independently checked both original instruction sequences and
owns the subsequent CPU/GPU verification. A successful headless fixture proves
this composition contract, not a full real-window startup/menu interaction.
The production strict WSI shutdown requirement remains separately tracked.
