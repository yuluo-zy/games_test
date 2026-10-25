// ==RECONSTRUCTED== shaders/paths/combine_path_exclusions.cs.hlsl | target: _paths_combine_path_exclusions.cs.hlsl_cs_main_0.bin | evidence: spirv-cross + SPIR-V disasm (clamp of sum of 4 path-mask texel loads -> paths_exclusion; no samplers, texel Load only; gid is int3 -> uint2 bitcast, coordinate reused for fetch+write; storage image R16f -> RWTexture2D<half>) | bindings: path_mask t14, cobblestone1_mask t15, cobblestone2_mask t16, cobblestone3_mask t17, paths_exclusion u18 | status: O (sum/clamp/store); residual codegen diff (int3 construct vs direct uint2 coord) = dxc-version artifact

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/math.hlsl"
#include "inc/math_const.hlsl"
#include "inc/consts.hlsl"
#include "inc/utils.hlsl"
#include "inc/hash.hlsl"
#include "inc/texture_utils.hlsl"

Texture2D<float4> path_mask : register(t14, space0);
Texture2D<float4> cobblestone1_mask : register(t15, space0);
Texture2D<float4> cobblestone2_mask : register(t16, space0);
Texture2D<float4> cobblestone3_mask : register(t17, space0);
RWTexture2D<half> paths_exclusion : register(u18, space0);

[numthreads(8, 8, 1)]
void cs_main(int3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    uint2 p = uint2(gl_GlobalInvocationID.xy);
    paths_exclusion[p] = clamp(path_mask.Load(int3(p, 0)).x
                              + cobblestone1_mask.Load(int3(p, 0)).x
                              + cobblestone2_mask.Load(int3(p, 0)).x
                              + cobblestone3_mask.Load(int3(p, 0)).x,
                              0.0f, 1.0f);
}
