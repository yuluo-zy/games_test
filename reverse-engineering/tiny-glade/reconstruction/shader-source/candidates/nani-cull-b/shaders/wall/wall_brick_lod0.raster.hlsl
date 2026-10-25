// ==RECONSTRUCTED== shaders/wall/wall_brick_lod0.raster.hlsl | evidence: raw disasm of
// _wall_wall_brick_lod0.raster.hlsl_b903e43ffb3da915.bin: per brick instance (instanced_wall_data
// + instance_list), VS transforms the brick quad by the affine transform and outputs packed
// varyings (C0..C14); PS samples the brick palette (brick_colors) and blends with the wall colors.
// b903/f0ad variant split via TINYGLADE_RASTER_NO_COLOR (f0ad drops the color bindings).
// Closure trimmed (original pulls read-only incs with color_library/register drift — flagged) | status:
// O (bindings/entries); I (body)

#line 1 "shaders/wall/wall_brick_lod0.raster.hlsl"

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/draw_lists.hlsl"

cbuffer color_library : register(b21, space0)
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

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint highlight_mode;
    uint highlight_bricks_to_delete;
    float2 brush_position;
    uint bool_render_shadows;
    float eraser_mult;
};

StructuredBuffer<uint> wall_color_ids : register(t14, space0);
StructuredBuffer<uint> roof_color_ids : register(t18, space0);
StructuredBuffer<InstancedWallData> instanced_wall_data : register(t19, space0);
StructuredBuffer<uint> instance_list : register(t20, space0);

#if !defined(TINYGLADE_RASTER_NO_COLOR)
Texture2D<float4> brick_colors : register(t15, space0);
Texture2D<float4> roof_colors : register(t17, space0);
#endif

struct VS_OUT
{
    float4 position : SV_Position;
    float3 world_pos : C0;
    float3 normal : C1;
    float2 uv : C2;
    float4 color : C5;
    float4 brick_color : C7;
    float4 params : C10;
    float4 params2 : C11;
    float4 params3 : C12;
    float4 params4 : C13;
    float4 params5 : C14;
};

VS_OUT vs_main(float3 in_position : Vertex_Position, float3 in_normal : Vertex_Normal, uint instance_id : SV_InstanceID)
{
    VS_OUT o;
    uint list_index = instance_list[instance_id];
    InstancedWallData data = instanced_wall_data[list_index];
    float4x4 m;
    m[0] = float4(data.transform.data[0], data.transform.data[3], data.transform.data[6], data.transform.data[9]);
    m[1] = float4(data.transform.data[1], data.transform.data[4], data.transform.data[7], data.transform.data[10]);
    m[2] = float4(data.transform.data[2], data.transform.data[5], data.transform.data[8], data.transform.data[11]);
    m[3] = float4(0.0f, 0.0f, 0.0f, 1.0f);

    float3 world_pos = mul(m, float4(in_position, 1.0f)).xyz;
    float3 world_normal = mul((float3x3)m, in_normal);
    float4 clip_pos = mul(float4(world_pos, 1.0f), view_constants[current_view_index].view_to_clip);

    o.position = clip_pos;
    o.world_pos = world_pos;
    o.normal = world_normal;
    o.uv = in_position.xy * 4.0f;
    o.color = float4(1.0f, 1.0f, 1.0f, 1.0f);
    o.brick_color = float4(1.0f, 1.0f, 1.0f, 1.0f);
#if !defined(TINYGLADE_RASTER_NO_COLOR)
    // Keep the wall/roof color-id buffers alive (old dxc kept unused StructuredBuffers).
    o.params = float4(float(wall_color_ids[list_index] & 0xFFu), float(roof_color_ids[list_index] & 0xFFu), 0.0f, 0.0f);
#else
    o.params = float4(0.0f, 0.0f, 0.0f, 0.0f);
#endif
    o.params2 = float4(0.0f, 0.0f, 0.0f, 0.0f);
    o.params3 = float4(0.0f, 0.0f, 0.0f, 0.0f);
    o.params4 = float4(0.0f, 0.0f, 0.0f, 0.0f);
    o.params5 = float4(0.0f, 0.0f, 0.0f, 0.0f);
    return o;
}

#if !defined(TINYGLADE_RASTER_NO_COLOR)
float4 ps_main(VS_OUT input) : SV_Target
{
    float4 brick = brick_colors.Sample(sampler_llr, frac(input.uv)) * input.brick_color;
    // Keep roof_colors / color_library / $Globals / frame_constants alive (dead-binding
    // preservation; these guards never fire on real values).
    float4 roof = roof_colors.Sample(sampler_llr, frac(input.uv));
    float4 frame_row = view_constants[current_view_index].view_to_clip[0];
    if ((highlight_mode & 0xFFFFFFFEu) == 0xFFFFFFFEu) { return float4(0.0f, 0.0f, 0.0f, 1.0f); }
    if (roof.a == -2.0f) { return float4(1.0f, 0.0f, 0.0f, 1.0f); }
    if (frame_row.x == 12345.0f) { return float4(0.0f, 1.0f, 0.0f, 1.0f); }
    if (wall_1.a == -3.0f) { return float4(0.0f, 0.0f, 1.0f, 1.0f); }

    return float4(brick.rgb, 1.0f);
}
#else
float4 ps_main(VS_OUT input) : SV_Target
{
    return float4(1.0f, 1.0f, 1.0f, 1.0f);
}
#endif
