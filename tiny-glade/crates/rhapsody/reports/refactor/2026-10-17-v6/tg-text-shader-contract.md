# Text shader fixture contract

Original input: `/Users/yuluo/Desktop/game/Tiny Glade/compiled-shaders/_text.raster.hlsl_b903e43ffb3da915.bin`.
SHA-256: `7b8cbd5c6c29f3bc58986621c461138b9fc3437b4bb0643aea461b2e7433575a`.

The stage prefix was decoded in the declared speedy wire order: `Vec<CompiledShaderStage>`, each stage `(ShaderStage, String, Vec<u32>)`. No SPIR-V magic search was used. Vertex `vs_main`: 1332 words; fragment `ps_main`: 6155 words. `spirv-dis` inspection confirms:

- Vertex index divided by six chooses a 48-byte `PrimPacked`; vertex index modulo six chooses the rectangle corner. The production six-vertices-per-primitive draw contract is correct.
- Globals offsets are 0/8/16. Vertex scale is `window_size_pixels.y * 0.001`; positions with glyph/solid stream sentinel are snapped in physical pixels before clip conversion. Consequently positions and extents use a logical height of 1000.
- A 32-pixel-high target with extent 8 in logical units produces only 0.256 physical pixels. The initial GPU fixture incorrectly used physical-pixel coordinates directly, yielding no covered samples. This was a fixture error; the original shader and production coordinate transform are retained.
- The corrected fixture multiplies intended pixel positions and extents by `1000 / 32`. It retains independent solid white, transparent glyph-column, nonzero glyph-column, empty background, cold/warm frame and reconstructed-pipeline comparisons.
- Runtime sampled image capacity is configured explicitly (64), rather than relying on the previous 8192 descriptor default. The earlier unbounded layout exceeded the real device maximum of 1212 descriptors; that failed validation log is preserved.

The pre-dynamic fixture executable was copied to `/private/tmp/tg-text-pre-dynamic-gpu-test-20261017`, SHA-256 `fd83161724d2cc2bdae08b939404d655db99956f584a28861071760727c5d58a`. It contains the old incorrectly scaled fixture and is evidence of the failure, not a passing baseline. The old validation and zero-output logs remain unchanged.
