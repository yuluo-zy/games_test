// ==SKELETON==
// SOURCE PATH : shaders/fsr_init_reactive.cs.hlsl
// KIND        : compute_main
// COMPILED AS MAIN IN (1 bins):
//   _fsr_init_reactive.cs.hlsl_cs_main_0.bin
//     file_sha256  : ca1a1d4ff7aaef297c8d107f73f25b40bb764e5da19fb6d4461d6fec9728556a
//     pass/stage   : _fsr_init_reactive (fsr_fxaa) / cs
//     entry        : cs:cs_main
//     variant      : 0
//     local_size   : [8, 8, 1]
//     names        : $Globals, output_tex, cs_main
//
// ==RECONSTRUCTED== shaders/fsr_init_reactive.cs.hlsl
// Evidence: _fsr_init_reactive.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23, bound 31).
//   bindings: $Globals b13 {output_tex_dims (uint2), value (float)},
//   output_tex u14 (StorageImage 2D); caps Shader + StorageImageExtendedFormats
//   (R16f storage image -> RWTexture2D<half>); LocalSize [8,8,1], entry cs_main.
//   spirv-cross + disassembly: output_tex[id.xy] = value (no conversion opcode in
//   the target stream; R16f write).  Status: O (structure from disassembly).

#include "inc/common.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 output_tex_dims;
    float value;
};

RWTexture2D<half> output_tex : register(u14, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    output_tex[gl_GlobalInvocationID.xy] = value;
}
