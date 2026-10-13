#define SHADER_VARIANT_DEPTH_ONLY 1
#line 1 "shaders/wall/wall_brick_lod0.raster.hlsl"








#line 1 "shaders/wall/wall_brick_lod0.raster.hlsl"

#line 1 "shaders/inc/common.hlsl"





#ifndef TINYGLADE_INC_COMMON_HLSL
#define TINYGLADE_INC_COMMON_HLSL

#line 1 "shaders/inc/samplers.hlsl"







#ifndef TINYGLADE_INC_SAMPLERS_HLSL
#define TINYGLADE_INC_SAMPLERS_HLSL

#line 1 "shaders/inc/debug_utils.hlsl"





#ifndef TINYGLADE_INC_DEBUG_UTILS_HLSL
#define TINYGLADE_INC_DEBUG_UTILS_HLSL

#line 1 "shaders/inc/tracked_buffer.hlsl"






#ifndef TINYGLADE_INC_TRACKED_BUFFER_HLSL
#define TINYGLADE_INC_TRACKED_BUFFER_HLSL




uint tracked_buffer_reserve(RWByteAddressBuffer counter, uint counter_offset)
{
    uint previous;
    counter.InterlockedAdd(counter_offset, 1u, previous);
    return previous;
}


void tracked_buffer_store_float(RWByteAddressBuffer buffer, uint base_byte_offset, uint element_index, float value)
{
    buffer.Store<float>(base_byte_offset + element_index * 4u, value);
}

#endif 
#line 9 "shaders/inc/debug_utils.hlsl"



float3 debug_heatmap(float v)
{
    float t = clamp(v, 0.0f, 1.0f);
    return lerp(float3(0.0f, 0.0f, 1.0f), float3(1.0f, 0.0f, 0.0f), t);
}

#endif 
#line 11 "shaders/inc/samplers.hlsl"


SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_llc : register(s3, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_s5 : register(s5, space0);
SamplerState sampler_s6 : register(s6, space0);
SamplerState sampler_s7 : register(s7, space0);
SamplerState sampler_s8 : register(s8, space0);
SamplerState sampler_default : register(s9, space0);
SamplerState sampler_mipmap : register(s10, space0);
SamplerState sampler_shadow_nearest : register(s11, space0);
SamplerComparisonState sampler_shadow_linear : register(s12, space0);

#endif 
#line 9 "shaders/inc/common.hlsl"


#endif 
#line 11 "shaders/wall/wall_brick_lod0.raster.hlsl"

#line 1 "shaders/inc/debug_utils.hlsl"





#ifndef TINYGLADE_INC_DEBUG_UTILS_HLSL
#define TINYGLADE_INC_DEBUG_UTILS_HLSL

#line 1 "shaders/inc/tracked_buffer.hlsl"






#ifndef TINYGLADE_INC_TRACKED_BUFFER_HLSL
#define TINYGLADE_INC_TRACKED_BUFFER_HLSL




uint tracked_buffer_reserve(RWByteAddressBuffer counter, uint counter_offset)
{
    uint previous;
    counter.InterlockedAdd(counter_offset, 1u, previous);
    return previous;
}


void tracked_buffer_store_float(RWByteAddressBuffer buffer, uint base_byte_offset, uint element_index, float value)
{
    buffer.Store<float>(base_byte_offset + element_index * 4u, value);
}

#endif 
#line 9 "shaders/inc/debug_utils.hlsl"



float3 debug_heatmap(float v)
{
    float t = clamp(v, 0.0f, 1.0f);
    return lerp(float3(0.0f, 0.0f, 1.0f), float3(1.0f, 0.0f, 0.0f), t);
}

#endif 
#line 12 "shaders/wall/wall_brick_lod0.raster.hlsl"

#line 1 "shaders/inc/frame_constants.hlsl"








#ifndef TINYGLADE_INC_FRAME_CONSTANTS_HLSL
#define TINYGLADE_INC_FRAME_CONSTANTS_HLSL

#line 1 "shaders/inc/uv.hlsl"






#ifndef TINYGLADE_INC_UV_HLSL
#define TINYGLADE_INC_UV_HLSL


float2 pixel_to_uv(int2 pixel, float2 dims)
{
    return (float2(pixel) + 0.5f) / dims;
}




float2 uv_to_sample(float2 uv)
{
    return uv - 0.5f;
}


float2 sample_to_clip(float2 sample)
{
    return sample * float2(2.0f, -2.0f);
}



float3 sample_to_view(float4x4 sample_to_view_mat, float2 sample_uv, float depth)
{
    return mul(sample_to_view_mat, float4(sample_uv * float2(2.0f, -2.0f), depth, 1.0f)).xyz;
}


float2 view_to_sample(float4x4 view_to_sample_mat, float3 view_pos)
{
    return mul(view_to_sample_mat, float4(view_pos, 1.0f)).xy;
}


float2 view_to_clip(float4x4 view_to_clip_mat, float3 view_pos)
{
    float4 clip = mul(view_to_clip_mat, float4(view_pos, 1.0f));
    return clip.xy / clip.w;
}


float3 world_to_view(float4x4 world_to_view_mat, float3 world_pos)
{
    return mul(world_to_view_mat, float4(world_pos, 1.0f)).xyz;
}


float2 world_to_sample(float4x4 world_to_sample_mat, float3 world_pos)
{
    return mul(world_to_sample_mat, float4(world_pos, 1.0f)).xy;
}

#endif 
#line 12 "shaders/inc/frame_constants.hlsl"










Texture2D<float4> bindless_textures_dyn[] : register(t0, space2);


struct ViewConstants
{
    float4x4 view_to_clip;
    float4x4 clip_to_view;
    float4x4 view_to_sample;
    float4x4 sample_to_view;
    float4x4 world_to_view;
    float4x4 view_to_world;
    float4x4 world_to_sample;
    float4x4 clip_to_prev_clip;
    float4x4 prev_view_to_prev_clip;
    float4x4 prev_clip_to_prev_view;
    float4x4 prev_world_to_prev_view;
    float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
};

cbuffer frame_constants : register(b0, space1)
{
    float4 sun_color;
    float4 sky_low_color;
    float4 sky_high_color;
    float4 fog_tint_thickness;
    float4 fake_light_color;
    float4 fake_light_dir;
    float4 fake_light2_color;
    float4 fake_light2_dir;
    uint frame_index;
    float delta_time_seconds;
    float ev_mult;
    uint glade_theme_id;
    uint2 inner_rendering_resolution;
    uint game_time_sec;
    float game_time_frac;
    float3 editing_highlight_color;
    float time_of_day;
    float3 dir_to_sun;
    uint camera_idx;
    float3 dir_to_moon;
    uint render_overrides;
    float global_mip_bias;
    uint highlighted_transient_stair_edge_id;
    uint pad1;
    uint pad2;
    float4 debug_tweakers;
    ViewConstants view_constants[5];
};

#endif 
#line 13 "shaders/wall/wall_brick_lod0.raster.hlsl"

#line 1 "shaders/wall/inc/draw_lists.hlsl"





#ifndef TINYGLADE_WALL_INC_DRAW_LISTS_HLSL
#define TINYGLADE_WALL_INC_DRAW_LISTS_HLSL

#ifndef TINYGLADE_AFFINE3PACKED
#define TINYGLADE_AFFINE3PACKED
struct Affine3Packed
{
    float data[12];
};
#endif


struct InstancedWallData
{
    Affine3Packed transform;
    float4 sin_deform_y;
    float3 global_arch_height_vals;
    uint seed;
    uint source_id;
    float2 wallspace_x_range;
    uint flags;
};

#endif 
#line 14 "shaders/wall/wall_brick_lod0.raster.hlsl"


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
