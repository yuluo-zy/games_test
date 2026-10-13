// ==RECONSTRUCTED== shaders/copy_depth_to_r.cs.hlsl | F-D
// Evidence: _copy_depth_to_r.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: input_tex t14/space0 (SampledImage 2D), output_tex u15/space0
//   (StorageImage r32f), LocalSize [8,8,1], entry cs_main.
//   spirv-cross: output_tex[gid.xy] = input_tex.Load(int3(gid.xy,0)).x.
// Status: O (body verbatim from disassembly; names from OpName).

#include "inc/common.hlsl"

Texture2D<float4> input_tex : register(t14, space0);
RWTexture2D<float> output_tex : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    uint2 pixel = dispatchThreadID.xy;
    output_tex[pixel] = input_tex[pixel].x;
}
