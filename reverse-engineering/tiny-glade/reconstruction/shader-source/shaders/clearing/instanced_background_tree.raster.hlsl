// ==RECONSTRUCTED== shaders/clearing/instanced_background_tree.raster.hlsl
// Evidence: _clearing_instanced_background_tree.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin.
//   bindings: $Globals b13 (current_view_index/model_to_world), color_library b14,
//   placement_buffer t15, instance_indirection_buffer t16, frame_constants set1/b0.
//   vs_main: per-instance PlacementData lookup via instance_indirection_buffer, rotation basis
//   + translate/scale placement transform, id-hash canopy color. ps_main: render_overrides debug
//   color override, sun-dir dot lighting, distance fade, sky ambient blend.
//   Status: O (structure verbatim).

#include "inc/common.hlsl"
#include "inc/current_view.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"

// color_library.hlsl is guard-skipped; flat-member cbuffer at set0/b14.
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL
#include "inc/color_library.hlsl"
cbuffer color_library : register(b14, space0)
{
    float4 temp_color_1;
    float4 temp_color_2;
    float4 temp_color_3;
    float4 roof_color_1;
    float4 roof_color_2;
    float4 sky_low_day;
    float4 sky_high_day;
    float4 sun_tint_day;
    float4 sky_low_dusk;
    float4 sky_high_dusk;
    float4 sun_tint_dusk;
    float4 indirect_light_tint;
    float4 fence_color_1;
    float4 fence_color_2;
    float4 flower_color_1;
    float4 flower_color_2;
    float4 canopy_color_1;
    float4 canopy_color_2;
    float4 grass_1;
    float4 grass_2;
    float4 path;
    float4 sky_base;
    float4 sky_horizon;
    float4 sky_clouds_1;
    float4 sky_clouds_2;
    float4 terrain_1;
    float4 terrain_2;
    float4 terrain_flattened;
    float4 wall_1;
    float4 wall_2;
    float4 wall_moss;
    float4 wall_bricks;
    float4 ivy_branch;
    float4 ivy_leaf_1;
    float4 ivy_leaf_2;
    float4 terrain_highlight;
    float4 night_tint;
};
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/math.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "clearing/inc/instanced_placement.hlsl"
#include "inc/pack_unpack.hlsl"

// $Globals member (implicit cbuffer at set0/b13): current_view_index from inc.
float4x4 model_to_world;

ByteAddressBuffer placement_buffer : register(t15, space0);
ByteAddressBuffer instance_indirection_buffer : register(t16, space0);

struct VS_IN
{
    float3 vertex_position : TEXCOORD0;
    float3 vertex_normal : TEXCOORD1;
};

struct VS_OUT
{
    float C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float4 position : SV_Position;
};

VS_OUT vs_main(VS_IN i, uint instance_id : SV_InstanceID)
{
    VS_OUT o;
    uint idx = uint(int(instance_indirection_buffer.Load<uint>(instance_id * 4u + 0)));
    float rot = radians(placement_buffer.Load<float>(idx * 48u + 16));
    float3 fwd = normalize(float3(cos(rot), 0.0f, sin(rot)));
    float4x4 basis = transpose(float4x4(float4(fwd, 0.0f),
                                        float4(0.0f, 1.0f, 0.0f, 0.0f),
                                        float4(normalize(cross(fwd, float3(0.0f, 1.0f, 0.0f))), 0.0f),
                                        float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float3 pos = placement_buffer.Load<float3>(idx * 48u + 0);
    float scale = placement_buffer.Load<float>(idx * 48u + 12);
    float4 world = mul(float4x4(float4(1.0f, 0.0f, 0.0f, pos.x),
                                float4(0.0f, 1.0f, 0.0f, pos.y),
                                float4(0.0f, 0.0f, 1.0f, pos.z),
                                float4(0.0f, 0.0f, 0.0f, 1.0f)),
                       mul(basis, mul(float4x4(float4(scale, 0.0f, 0.0f, 0.0f),
                                               float4(0.0f, scale, 0.0f, 0.0f),
                                               float4(0.0f, 0.0f, scale, 0.0f),
                                               float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(i.vertex_position, 1.0f))));
    int pid = placement_buffer.Load<int>(idx * 48u + 20);
    uint h = hash(uint(pid));
    o.position = mul(view_constants[current_view_index].world_to_sample, mul(model_to_world, world));
    o.C0 = world.y;
    o.C1 = mul(basis, float4(i.vertex_normal, 0.0f)).xyz;
    o.C2 = lerp(canopy_color_1.xyz, canopy_color_2.xyz, rand01(h).xxx);
    return o;
}

struct PS_IN
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
};

struct PS_OUT
{
    float4 target : SV_Target0;
};

PS_OUT ps_main(PS_IN i, float4 frag_coord : SV_Position)
{
    PS_OUT o;
    bool3 over = ((render_overrides & 1u) != 0u);
    float3 c2 = select(over, 0.5f.xxx, i.C2);
    float3 lit = (c2 * 0.19098593294620513916015625f)
        * lerp(0.35f, 0.9f, smoothstep(-0.2f, 0.8f, dot(i.C1, mul(view_constants[2].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz)).xxx);
    float3 ambient = lerp(sky_low_color.xyz, sky_high_color.xyz, 0.5f) * 0.2f;
    float fade = smoothstep(0.07f, 0.3f, 0.0005f / frag_coord.w);
    float alpha = clamp((fade * fade) * 2.0f, 0.0f, 1.0f);
    o.target = float4(lerp(lit * sun_color.xyz, (pow(ambient, 1.0f)) * 0.8f, alpha) * 1.2f, 1.0f);
    return o;
}
