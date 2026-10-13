// ==RECONSTRUCTED== shaders/stairs_foliage_exclusion.raster.hlsl | target: _stairs_foliage_exclusion.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | evidence: spirv-cross + disasm (VS: mesh_buf vertex slab, position = (v.xz * (2/130, -2/130), 1,1), texcoord0 = v.xyz; PS: discard when world y > terrain height + 1.75, else write white) | bindings: mesh_buf t16 (vs); sampler_lnc s0, visual_terrain_heightmap t15 (ps) | status: O (algorithm); inc-closure debug diffs

#include "inc/common.hlsl"
#include "inc/consts.hlsl"
Texture2D<float4> terrain_heightmap;
#include "inc/terrain_heightmap.inc.hlsl"
// Registerless global for inc/terrain_heightmap.inc.hlsl's terrain_height()
// helper; unused here, so dxc DCEs it (no binding emitted).
#include "inc/uv.hlsl"

ByteAddressBuffer mesh_buf : register(t16, space0);
Texture2D<float4> visual_terrain_heightmap : register(t15, space0);

struct VSOut
{
    float3 c0 : TEXCOORD0;
    float4 pos : SV_Position;
};

VSOut vs_main(uint vertex_id : SV_VertexID)
{
    VSOut o;
    float4 v = mesh_buf.Load<float4>(uint(vertex_id) * 16 + 0);
    o.pos = float4(v.xz * float2(0.015384615398943424224853515625f, -0.015384615398943424224853515625f), 1.0f, 1.0f);
    o.c0 = v.xyz;
    return o;
}

float4 ps_main(float3 c0 : TEXCOORD0) : SV_Target0
{
    if (c0.y > (lerp(-2.5f, 24.0f, visual_terrain_heightmap.SampleLevel(sampler_lnc, (c0.xz * float2(0.0076923076994717121124267578125f, 0.0076923076994717121124267578125f)) + float2(0.5f, 0.5f), 0.0f).x) + 1.75f))
    {
        discard;
    }
    return float4(1.0f, 1.0f, 1.0f, 1.0f);
}
