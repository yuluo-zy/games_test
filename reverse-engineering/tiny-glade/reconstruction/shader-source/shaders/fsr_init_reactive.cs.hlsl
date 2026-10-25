// ==RECONSTRUCTED== shaders/fsr_init_reactive.cs.hlsl | B (dof-aa review, 2026-10-13: tie; B body). RWTexture2D<half> u14 (R16f) + $Globals b13 {output_tex_dims, value}; write output_tex[id.xy] = value.
//   Fills the FSR reactive-mask buffer with a constant value.

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
