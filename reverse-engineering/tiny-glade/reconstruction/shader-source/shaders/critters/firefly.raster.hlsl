// ==RECONSTRUCTED== shaders/critters/firefly.raster.hlsl | target: _critters_firefly.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main | evidence: spirv-cross + SPIR-V access/type inspection (instance StructuredBuffer<Firefly> 16B stride, terrain-heightmap snap to world y, visibility-faded dual-tone glow) | bindings: sampler_default s9, $Globals b13, terrain_heightmap t16, firefly_buffer t18, frame_constants set1/b0 | status: R (reconstruction)

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/current_view.hlsl"
Texture2D<float4> terrain_heightmap : register(t16);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"

// INC-issue workaround: original samplers.hlsl declared s0..s12 (13 samplers);
// inc/samplers.hlsl carries only s0..s4,s9..s12, so the missing s5..s8 are
// declared here to reproduce the original $Globals binding (set0/b13).

struct Firefly
{
    float3 position;
    uint pad;
};

StructuredBuffer<Firefly> firefly_buffer : register(t18);

int bool_render_shadows;
uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
float firefly_visibility;

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    float3 firefly_pos = firefly_buffer[vin.instance_id].position;
    float3 world_pos = (vin.Vertex_Position * 0.039999999105930328369140625f + firefly_pos)
        + float3(0.0f,
                 lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, firefly_pos.xz * 0.0076923076994717121124267578125f.xx + 0.5f.xx, 0.0f).x),
                 0.0f);
    o.position = mul(view_constants[current_view_index].world_to_sample, float4(world_pos, 1.0f));
    return o;
}

struct PsOut
{
    float4 color : SV_Target0;
};

PsOut ps_main()
{
    PsOut o;
    float vis = firefly_visibility;
    float3 low = float3(0.449999988079071044921875f, 0.449999988079071044921875f, 0.02249999903142452239990234375f) * sky_high_color.xyz;
    float3 high = float3(1.0f, 1.0f, 0.0500000007450580596923828125f) * ev_mult;
    o.color = float4(lerp(low, high, vis * vis), 1.0f);
    return o;
}
