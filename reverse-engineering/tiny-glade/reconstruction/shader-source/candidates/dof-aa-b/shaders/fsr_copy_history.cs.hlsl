// ==SKELETON==
// SOURCE PATH : shaders/fsr_copy_history.cs.hlsl
// KIND        : compute_main
// COMPILED AS MAIN IN (1 bins):
//   _fsr_copy_history.cs.hlsl_cs_main_0.bin
//     file_sha256  : c30be4b6bad050394ee109ccab9b5508002ce8124719e5cd4fa6916f9e652359
//     pass/stage   : _fsr_copy_history (fsr_fxaa) / cs
//     entry        : cs:cs_main
//     variant      : 0
//     local_size   : [8, 8, 1]
//     names        : input_tex, output_tex, cs_main
//
// ==RECONSTRUCTED== shaders/fsr_copy_history.cs.hlsl
// Evidence: _fsr_copy_history.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23, bound 30).
//   bindings: input_tex t14 (SampledImage 2D), output_tex u15 (StorageImage 2D);
//   LocalSize [8,8,1], entry cs_main.  spirv-cross + disassembly: a plain
//   input->output copy: output_tex[id.xy] = input_tex.Load(int3(id.xy, 0)).
// Status: O (structure from disassembly; trivial copy).

#include "inc/common.hlsl"

Texture2D<float4> input_tex : register(t14, space0);
RWTexture2D<float4> output_tex : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    output_tex[gl_GlobalInvocationID.xy] = input_tex[gl_GlobalInvocationID.xy];
}
