// ==RECONSTRUCTED== shaders/fsr_copy_history.cs.hlsl | B (dof-aa review, 2026-10-13): B winner (input_tex[id.xy] texel-fetch indexing matches OpImageFetch + OpVectorShuffle; A's Load() adds opcodes).
//   A plain input->output copy: output_tex[id.xy] = input_tex[id.xy].

#include "inc/common.hlsl"

Texture2D<float4> input_tex : register(t14, space0);
RWTexture2D<float4> output_tex : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    output_tex[gl_GlobalInvocationID.xy] = input_tex[gl_GlobalInvocationID.xy];
}
