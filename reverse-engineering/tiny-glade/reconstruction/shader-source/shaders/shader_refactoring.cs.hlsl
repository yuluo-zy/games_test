// ==RECONSTRUCTED== shaders/shader_refactoring.cs.hlsl | F-D
// Evidence: _shader_refactoring.cs.hlsl_capture_0.bin (entry `capture`, [8,8,1])
//   + _shader_refactoring.cs.hlsl_compare_0.bin (entry `compare`, [8,8,1]).
//   bindings: rendered_tex t14, reference_tex u15 (capture); + feedback_tex u16 (compare).
//   caps: Shader, StorageImageExtendedFormats, StorageImageWriteWithoutFormat.
//   spirv-cross capture: reference_tex[gid.xy] = rendered_tex[gid.xy].xyz;
//   spirv-cross compare: per-component |rendered-reference|>1e-5 selects red flag on
//     top of 8% dimmed (0,1,1) reference tint.
// Status: O (body from disassembly; entry names `capture`/`compare` verbatim).

#include "inc/common.hlsl"

Texture2D<float4> rendered_tex : register(t14, space0);
RWTexture2D<float3> reference_tex : register(u15, space0);
RWTexture2D<float3> feedback_tex : register(u16, space0);

[numthreads(8, 8, 1)]
void capture(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    reference_tex[dispatchThreadID.xy] = rendered_tex[dispatchThreadID.xy].xyz;
}

[numthreads(8, 8, 1)]
void compare(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    float3 reference = reference_tex[dispatchThreadID.xy].xyz;
    float3 diff = abs(rendered_tex[dispatchThreadID.xy].xyz - reference);
    bool any_diff = any(diff > 1.0e-5f);
    feedback_tex[dispatchThreadID.xy] = reference * 0.08f * float3(0.0f, 1.0f, 1.0f)
        + select(bool3(any_diff, any_diff, any_diff), float3(1.0f, 0.0f, 0.0f), float3(0.0f, 0.0f, 0.0f));
}
