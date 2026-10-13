#define SHADER_VARIANT_DEPTH_ONLY 0
#define TGV_b903e43ffb3da915 1
#line 1 "shaders/nani_roof_floor.raster.hlsl"


#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))





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
#line 9 "shaders/nani_roof_floor.raster.hlsl"

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
#line 10 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/inc/current_view.hlsl"





#ifndef TINYGLADE_INC_CURRENT_VIEW_HLSL
#define TINYGLADE_INC_CURRENT_VIEW_HLSL

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
#line 9 "shaders/inc/current_view.hlsl"


uint current_view_index;


ViewConstants GetCurrentViewConstants()
{
    return view_constants[current_view_index];
}

#endif 
#line 11 "shaders/nani_roof_floor.raster.hlsl"

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
#line 12 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/inc/ray_cone.hlsl"







#ifndef TINYGLADE_INC_RAY_CONE_HLSL
#define TINYGLADE_INC_RAY_CONE_HLSL



struct RayCone
{
    float3 origin;
    float3 direction;
    float spread; 
};


float ray_cone_half_width(RayCone cone, float t)
{
    return t * cone.spread;
}



float ray_cone_mip(float2 footprint_world, float2 world_per_texel)
{
    float2 footprint_texels = max(footprint_world / max(world_per_texel, 1e-6f.xx), 1.0f.xx);
    return log2(max(footprint_texels.x, footprint_texels.y));
}

#endif 
#line 13 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/inc/math_const.hlsl"





#ifndef TINYGLADE_INC_MATH_CONST_HLSL
#define TINYGLADE_INC_MATH_CONST_HLSL

static const float PI = 3.14159265358979323846264338327950288f;
static const float TWO_PI = 6.28318530717958647692528676655900577f;
static const float HALF_PI = 1.57079632679489661923132169163975144f;
static const float INV_PI = 0.31830988618379067153776752674502872f;
static const float INV_TWO_PI = 0.15915494309189533576888376337251436f;

static const float SQRT_TWO = 1.41421356237309504880168872420969808f;
static const float INV_SQRT_TWO = 0.70710678118654752440084436210484904f;

static const float GOLDEN_RATIO = 1.61803398874989484820458683436563812f;
static const float INV_GOLDEN_RATIO = 0.61803398874989484820458683436563812f;

static const float EPSILON = 1.0e-6f;
static const float FLT_EPSILON = 1.19209290e-7f;

#endif 
#line 14 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/inc/utils.hlsl"





#ifndef TINYGLADE_INC_UTILS_HLSL
#define TINYGLADE_INC_UTILS_HLSL

#line 1 "shaders/inc/math_const.hlsl"





#ifndef TINYGLADE_INC_MATH_CONST_HLSL
#define TINYGLADE_INC_MATH_CONST_HLSL

static const float PI = 3.14159265358979323846264338327950288f;
static const float TWO_PI = 6.28318530717958647692528676655900577f;
static const float HALF_PI = 1.57079632679489661923132169163975144f;
static const float INV_PI = 0.31830988618379067153776752674502872f;
static const float INV_TWO_PI = 0.15915494309189533576888376337251436f;

static const float SQRT_TWO = 1.41421356237309504880168872420969808f;
static const float INV_SQRT_TWO = 0.70710678118654752440084436210484904f;

static const float GOLDEN_RATIO = 1.61803398874989484820458683436563812f;
static const float INV_GOLDEN_RATIO = 0.61803398874989484820458683436563812f;

static const float EPSILON = 1.0e-6f;
static const float FLT_EPSILON = 1.19209290e-7f;

#endif 
#line 9 "shaders/inc/utils.hlsl"







#ifndef TINYGLADE_AFFINE3PACKED
#define TINYGLADE_AFFINE3PACKED
struct Affine3Packed
{
    float data[12];
};
#endif



float4x4 affine_basis(float3 forward)
{
    float3 f = normalize(forward);
    float3 up = float3(0.0f, 1.0f, 0.0f);
    float3 right = cross(f, up);
    float4x4 m = float4x4(
        float4(f, 0.0f),
        float4(up, 0.0f),
        float4(right, 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f));
    return transpose(m);
}


float4x4 affine_translation(float3 pos)
{
    return float4x4(
        float4(1.0f, 0.0f, 0.0f, pos.x),
        float4(0.0f, 1.0f, 0.0f, pos.y),
        float4(0.0f, 0.0f, 1.0f, pos.z),
        float4(0.0f, 0.0f, 0.0f, 1.0f));
}


float4x4 affine_scale(float3 s)
{
    return float4x4(
        float4(s.x, 0.0f, 0.0f, 0.0f),
        float4(0.0f, s.y, 0.0f, 0.0f),
        float4(0.0f, 0.0f, s.z, 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f));
}



Affine3Packed pack_affine(float4x4 m)
{
    Affine3Packed o;
    o.data[0] = m[0].x; o.data[1] = m[1].x; o.data[2] = m[2].x;
    o.data[3] = m[0].y; o.data[4] = m[1].y; o.data[5] = m[2].y;
    o.data[6] = m[0].z; o.data[7] = m[1].z; o.data[8] = m[2].z;
    o.data[9] = m[0].w; o.data[10] = m[1].w; o.data[11] = m[2].w;
    return o;
}


float4x4 unpack_affine(Affine3Packed a)
{
    float4x4 m;
    m[0] = float4(a.data[0], a.data[3], a.data[6], a.data[9]);
    m[1] = float4(a.data[1], a.data[4], a.data[7], a.data[10]);
    m[2] = float4(a.data[2], a.data[5], a.data[8], a.data[11]);
    m[3] = float4(0.0f, 0.0f, 0.0f, 1.0f);
    return m;
}


Affine3Packed compose_affine_packed(float3 pos, float3 forward, float3 scale)
{
    return pack_affine(mul(affine_translation(pos), mul(affine_basis(forward), affine_scale(scale))));
}

#endif 
#line 15 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/nani/inc/user.hlsl"





#ifndef TINYGLADE_NANI_INC_USER_HLSL
#define TINYGLADE_NANI_INC_USER_HLSL

#line 1 "shaders/inc/hsv.hlsl"










float3 rgb2hsv(float3 c)
{
    float4 K = float4(0.0f, -1.0f / 3.0f, 2.0f / 3.0f, -1.0f);
    float4 p = lerp(float4(c.zy, K.wz), float4(c.yz, K.xy), step(c.z, c.y));
    float4 q = lerp(float4(p.xyw, c.x), float4(c.x, p.yzx), step(p.x, c.x));
    float d = q.x - min(q.w, q.y);
    float e = 1.0e-10f;
    return float3(abs(q.z + (q.w - q.y) / (6.0f * d + e)), d / (q.x + e), q.x);
}

float3 hsv2rgb(float3 c)
{
    float3 p = abs(float3(abs(c.x - 3.0f) - 1.0f, 2.0f - abs(c.x - 2.0f), 2.0f - abs(c.x - 4.0f)));
    p = clamp(p, 0.0f, 1.0f);
    return lerp(float3(1.0f, 1.0f, 1.0f), p, c.y) * c.z;
}
#line 9 "shaders/nani/inc/user.hlsl"

#line 1 "shaders/stairs_floor.inc.hlsl"








#ifndef TINYGLADE_STAIRS_FLOOR_INC_HLSL
#define TINYGLADE_STAIRS_FLOOR_INC_HLSL


struct NaniTrimeshChunk
{
    uint64_t gpu_address;
    uint vertex_index_count;
    uint vertex_byte_offset;
    float4 bounding_sphere;
};


struct StairsFloorInstanceData
{
    NaniTrimeshChunk mesh;
    uint color_id;
    uint pad0;
    uint pad1;
    uint pad2;
};

#endif 
#line 10 "shaders/nani/inc/user.hlsl"

#line 1 "shaders/wall/inc/brick_vs_out.hlsl"




#ifndef TINYGLADE_WALL_INC_BRICK_VS_OUT_HLSL
#define TINYGLADE_WALL_INC_BRICK_VS_OUT_HLSL

#line 1 "shaders/wall/inc/brick_instance_flags.hlsl"




#ifndef TINYGLADE_WALL_INC_BRICK_INSTANCE_FLAGS_HLSL
#define TINYGLADE_WALL_INC_BRICK_INSTANCE_FLAGS_HLSL

#line 1 "shaders/wall/inc/brick_instance.hlsl"




#ifndef TINYGLADE_WALL_INC_BRICK_INSTANCE_HLSL
#define TINYGLADE_WALL_INC_BRICK_INSTANCE_HLSL

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
#line 8 "shaders/wall/inc/brick_instance.hlsl"


#endif 
#line 8 "shaders/wall/inc/brick_instance_flags.hlsl"


#endif 
#line 8 "shaders/wall/inc/brick_vs_out.hlsl"


#endif 
#line 11 "shaders/nani/inc/user.hlsl"


#endif 
#line 16 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/roof_material_id.inc.hlsl"









#ifndef TINYGLADE_ROOF_MATERIAL_ID_INC_HLSL
#define TINYGLADE_ROOF_MATERIAL_ID_INC_HLSL


struct RoofFloorInstanceData
{
    float3 position;
    uint wall_id;
    float2 basis0_xz;
    float2 scale;
    float roof_animation;
    uint shape;
    uint double_sided;
    uint is_roof_bottom;
    uint material_id;
    uint override_color_id;
    uint pattern_id;
    uint pad_1;
};




void get_roof_material_params(uint material_id, out float3 albedo, out float roughness)
{
    albedo = float3(1.0f, 1.0f, 1.0f);
    roughness = 1.0f;
}

#endif 
#line 17 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/roof/stone_floor.inc.hlsl"






#ifndef TINYGLADE_ROOF_STONE_FLOOR_HLSL
#define TINYGLADE_ROOF_STONE_FLOOR_HLSL




float stone_floor_march_height(float3 origin, float2 dir, Texture2D height_map, SamplerState s,
                               float mip_bias, float height_scale)
{
    float hit = 1.0f;
    float t_lo = 0.0f;
    float t_hi = 1.0f;
    float prev = origin.z;
    for (uint i = 1u; i < 3u; i++)
    {
        float2 p = origin.xy + (dir * t_hi);
        float h = height_map.SampleLevel(s, p, mip_bias).x - 1.0f;
        float d = origin.z - (h * height_scale);
        if (d < 0.0f) break;
        prev = d;
        t_lo = t_hi;
        t_hi = saturate(min(0.9999847f, pow((float(i) + frac(0.0f)) * 0.5f, 2.0f)));
    }
    
    for (uint i = 0u; i < 4u; i++)
    {
        float t = (t_lo + t_hi) * 0.5f;
        float2 p = origin.xy + (dir * t);
        float h = height_map.SampleLevel(s, p, mip_bias).x - 1.0f;
        float d = origin.z - (h * height_scale);
        if (d < 0.0f) { t_hi = t; prev = d; }
        else { t_lo = t; prev = d; }
    }
    return lerp(t_lo, t_hi, prev / (prev + (prev - prev)));
}


void stone_floor_sample(float2 uv, Texture2D normal_map, Texture2D seed_map, Texture2D height_map,
                        Texture2D roughness_map, SamplerState s, float mip_bias,
                        out float3 normal, out float4 seed, out float4 height, out float4 roughness)
{
    normal = (normal_map.SampleLevel(s, uv, mip_bias).xyz * 2.0f) - 1.0f;
    normal.z *= 0.13333332538604736328125f;
    seed = seed_map.SampleBias(s, uv, mip_bias);
    height = height_map.SampleBias(s, uv, mip_bias);
    roughness = roughness_map.SampleBias(s, uv, mip_bias);
}

#endif 
#line 18 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/roof/wooden_floor.inc.hlsl"





#ifndef TINYGLADE_ROOF_WOODEN_FLOOR_HLSL
#define TINYGLADE_ROOF_WOODEN_FLOOR_HLSL


void wooden_floor_sample(float2 uv, Texture2D normal_map, Texture2D seed_map, Texture2D height_map,
                         Texture2D roughness_map, SamplerState s, float mip_bias,
                         out float3 normal, out float4 seed, out float4 height, out float4 roughness)
{
    normal = (normal_map.SampleLevel(s, uv, mip_bias).xyz * 2.0f) - 1.0f;
    seed = seed_map.SampleBias(s, uv, mip_bias);
    height = height_map.SampleBias(s, uv, mip_bias);
    roughness = roughness_map.SampleBias(s, uv, mip_bias);
}

#endif 
#line 19 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/roof/dormer_hole.inc.hlsl"







#ifndef TINYGLADE_ROOF_DORMER_HOLE_HLSL
#define TINYGLADE_ROOF_DORMER_HOLE_HLSL


struct DormerHole
{
    float3 position;
    uint wall_id;
    float3 size;
    float2 pad0;
    float2 normal;   
    float k;
    float a;
};




bool is_inside_dormer_hole(float3 world_pos, DormerHole d)
{
    float3 basis = float3(d.normal.x, 0.0f, d.normal.y);
    float3x3 basis_mat = transpose(float3x3(cross(basis, float3(0.0f, 1.0f, 0.0f)),
                                            float3(0.0f, 1.0f, 0.0f), basis));
    float3 b0 = mul(basis_mat, float3(1.0f, 0.0f, 0.0f));
    float3 b1 = mul(basis_mat, float3(0.0f, 1.0f, 0.0f));
    float3 b2 = mul(basis_mat, float3(0.0f, 0.0f, 1.0f));
    float3 n = cross(b0, b1);
    float3 inv = (1.0f / dot(b2, n)).xxx;
    float3 p = mul(world_pos - d.position, float3x3(cross(b1, b2) * inv, cross(b2, b0) * inv, n * inv)) / d.size;
    float w = clamp((p.y * d.k) + d.a, 0.0f, 1.0f);
    bool inside = abs(p.x) < lerp(0.5f, 0.0f, (w * w) * w)
               && ((d.k == 1.0f) ? (p.y < 0.5f) && (p.y > -0.5f) : p.y < 0.7f)
               && abs(p.z) < 0.5f;
    return inside;
}

#endif 
#line 20 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/inc/perlin.hlsl"












#line 1 "shaders/inc/hash.hlsl"







#ifndef TINYGLADE_INC_HASH_HLSL
#define TINYGLADE_INC_HASH_HLSL

#line 1 "shaders/inc/math_const.hlsl"





#ifndef TINYGLADE_INC_MATH_CONST_HLSL
#define TINYGLADE_INC_MATH_CONST_HLSL

static const float PI = 3.14159265358979323846264338327950288f;
static const float TWO_PI = 6.28318530717958647692528676655900577f;
static const float HALF_PI = 1.57079632679489661923132169163975144f;
static const float INV_PI = 0.31830988618379067153776752674502872f;
static const float INV_TWO_PI = 0.15915494309189533576888376337251436f;

static const float SQRT_TWO = 1.41421356237309504880168872420969808f;
static const float INV_SQRT_TWO = 0.70710678118654752440084436210484904f;

static const float GOLDEN_RATIO = 1.61803398874989484820458683436563812f;
static const float INV_GOLDEN_RATIO = 0.61803398874989484820458683436563812f;

static const float EPSILON = 1.0e-6f;
static const float FLT_EPSILON = 1.19209290e-7f;

#endif 
#line 11 "shaders/inc/hash.hlsl"



uint hash(uint x)
{
    x += (x << 10u);
    x ^= (x >> 6u);
    x += (x << 3u);
    x ^= (x >> 11u);
    x += (x << 15u);
    return x;
}


float rand01(uint x)
{
    return asfloat((x & 0x7FFFFFu) | 0x3F800000u) - 1.0f;
}


float hash_angle(uint x)
{
    return frac(0.9f + (float(x) * GOLDEN_RATIO)) * TWO_PI;
}


float hash01(float x)
{
    return frac(sin(x) * 43758.5453f);
}

#endif 
#line 13 "shaders/inc/perlin.hlsl"






uint hash(uint2 p)
{
    uint y = p.y;
    y += (y << 10u);
    y ^= (y >> 6u);
    y += (y << 3u);
    y ^= (y >> 11u);
    uint h = ((p.x * 1664525u) + (y + (y << 15u))) + 1013904223u;
    h *= 1664525u;
    h ^= (h >> 11u);
    h ^= (h << 7u) & 2636928640u;
    h ^= (h << 15u) & 4022730752u;
    return h;
}

float incb_perlin_hash01(uint h)
{
    return asfloat(((h ^ (h >> 18u)) & 0x7FFFFFu) | 0x3F800000u) - 1.0f;
}

float perlin(float2 p)
{
    int2 cell = int2(floor(p));
    float2 f = frac(p);
    float2 u = f * f * (3.0f - 2.0f * f);

    float n00 = incb_perlin_hash01(hash(uint2(cell + int2(0, 0))));
    float n10 = incb_perlin_hash01(hash(uint2(cell + int2(1, 0))));
    float n01 = incb_perlin_hash01(hash(uint2(cell + int2(0, 1))));
    float n11 = incb_perlin_hash01(hash(uint2(cell + int2(1, 1))));

    
    return (1.0f - u.y) * ((n00 * (1.0f - u.x)) + (n10 * u.x))
         + (n01 * (1.0f - u.x)) * u.y + (n11 * u.x) * u.y;
}
#line 21 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/inc/parallax_mapping.hlsl"




#ifndef TINYGLADE_INC_PARALLAX_MAPPING_HLSL
#define TINYGLADE_INC_PARALLAX_MAPPING_HLSL




float2 parallax_mapping(Texture2D<float4> height_tex, SamplerState samp,
                        float2 uv, float3 view_dir, float height_scale, uint layers)
{
    float layer_depth = 1.0f / float(layers);
    float current_depth = 1.0f;
    float2 p = view_dir.xy * (height_scale / float(layers));
    float2 delta = p;

    float2 current_uv = uv;
    float height = 1.0f - height_tex.SampleLevel(samp, current_uv, 0.0f).r;
    while (current_depth > 0.0f && height < current_depth)
    {
        current_uv -= delta;
        height = 1.0f - height_tex.SampleLevel(samp, current_uv, 0.0f).r;
        current_depth -= layer_depth;
    }

    
    float2 prev_uv = current_uv + delta;
    float prev_depth = current_depth + layer_depth;
    float prev_height = 1.0f - height_tex.SampleLevel(samp, prev_uv, 0.0f).r;
    float w = (current_depth - height) / max((current_depth - height) - (prev_depth - prev_height), 1.0e-5f);
    return lerp(current_uv, prev_uv, saturate(w));
}

#endif 
#line 22 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/inc/raymarch.hlsl"


#ifndef TINYGLADE_INC_RAYMARCH_HLSL
#define TINYGLADE_INC_RAYMARCH_HLSL





float raymarch_depth_texture(Texture2D depth_tex, SamplerState s, float2 texel_size,
                             float3 ray_start_view, float3 ray_end_view)
{
    float3 delta = ray_end_view - ray_start_view;
    float dist = length(delta);
    float t = 0.0;
    
    float step = max(0.05, dist / 16.0);
    for (uint i = 0u; i < 64u; ++i) {
        t += step / max(dist, 1e-6);
        if (t >= 1.0) {
            return 1.0;
        }
        float3 p = ray_start_view + delta * t;
        if (p.z <= 0.0) {
            return 1.0;
        }
        float2 uv = p.xy * 0.5 + 0.5;
        uv.y = 1.0 - uv.y;
        float sampled_depth = depth_tex.SampleLevel(s, uv, 0.0).x;
        float inv_depth = 1.0 / sampled_depth;
        float my_inv = 1.0 / p.z;
        if (my_inv > inv_depth) {
            
            float lo = max(0.0, t - step / max(dist, 1e-6));
            for (uint j = 0u; j < 8u; ++j) {
                float mid = (lo + t) * 0.5;
                float3 pm = ray_start_view + delta * mid;
                float2 uvm = pm.xy * 0.5 + 0.5;
                uvm.y = 1.0 - uvm.y;
                if (depth_tex.SampleLevel(s, uvm, 0.0).x > 1.0 / pm.z) {
                    t = mid;
                } else {
                    lo = mid;
                }
            }
            return t;
        }
    }
    return 1.0;
}



float raymarch_opacity(Texture2D depth_tex, SamplerState s, float2 texel_size,
                       float3 ray_start_view, float3 ray_end_view, float density)
{
    float3 delta = ray_end_view - ray_start_view;
    float dist = length(delta);
    float acc = 0.0;
    uint steps = 12u;
    for (uint i = 0u; i < steps; ++i) {
        float t = (float(i) + 0.5) / float(steps);
        float3 p = ray_start_view + delta * t;
        if (p.z <= 0.0) {
            break;
        }
        float2 uv = p.xy * 0.5 + 0.5;
        uv.y = 1.0 - uv.y;
        float inv_depth = 1.0 / max(depth_tex.SampleLevel(s, uv, 0.0).x, 1e-6);
        if (inv_depth < 1.0 / max(p.z, 1e-6)) {
            acc += density * (dist / float(steps));
        }
    }
    return saturate(acc);
}

#endif 
#line 23 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/inc/wall_color_ids.hlsl"


#ifndef TINYGLADE_INC_WALL_COLOR_IDS_HLSL
#define TINYGLADE_INC_WALL_COLOR_IDS_HLSL

#line 1 "shaders/inc/reservoir.hlsl"


#ifndef TINYGLADE_INC_RESERVOIR_HLSL
#define TINYGLADE_INC_RESERVOIR_HLSL


struct Reservoir1spp {
    float w_sum;
    uint payload;
    float M;
    float W;
};


struct Reservoir1sppStreamState {
    float p_q_sel; 
    float M_sum;   
};


void reservoir_init(out Reservoir1spp r, out Reservoir1sppStreamState s)
{
    r.w_sum = 0.0;
    r.payload = 0xFFFFFFFFu;
    r.M = 0.0;
    r.W = 0.0;
    s.p_q_sel = 0.0;
    s.M_sum = 0.0;
}




bool reservoir_update(inout Reservoir1spp r, inout Reservoir1sppStreamState s,
                      float w, float target_pdf, uint payload, float u)
{
    s.M_sum += 1.0;
    r.M += 1.0;
    r.w_sum += w;
    if (w / r.w_sum >= u) {
        r.payload = payload;
        s.p_q_sel = target_pdf;
        return true;
    }
    return false;
}


void reservoir_combine_streams(inout Reservoir1sppStreamState dst, Reservoir1sppStreamState src)
{
    dst.M_sum += src.M_sum;
}



void reservoir_resolve(inout Reservoir1spp r, Reservoir1sppStreamState s)
{
    r.M = s.M_sum;
    r.W = r.w_sum / max(1e-8, r.M * s.p_q_sel);
}


uint4 pack_reservoir1spp(Reservoir1spp r)
{
    return uint4(asuint(r.w_sum), r.payload, asuint(r.M), asuint(r.W));
}


Reservoir1spp unpack_reservoir1spp(uint4 v)
{
    Reservoir1spp r;
    r.w_sum = asfloat(v.x);
    r.payload = v.y;
    r.M = asfloat(v.z);
    r.W = asfloat(v.w);
    return r;
}

#endif 
#line 6 "shaders/inc/wall_color_ids.hlsl"



uint get_wall_color_id(ByteAddressBuffer wall_color_ids, uint index)
{
    return wall_color_ids.Load<uint>(index * 4u + 0u);
}

#endif 
#line 24 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/inc/color_library.hlsl"


#ifndef TINYGLADE_INC_COLOR_LIBRARY_HLSL
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL



struct ColorLibrary {
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






#ifndef TINYGLADE_COLOR_LIBRARY_REGISTER
#define TINYGLADE_COLOR_LIBRARY_REGISTER b14
#endif
cbuffer color_library : register(TINYGLADE_COLOR_LIBRARY_REGISTER, space0) {
    ColorLibrary color_library;
};

#endif 
#line 25 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/inc/wall_color_customization.hlsl"


#ifndef TINYGLADE_INC_WALL_COLOR_CUSTOMIZATION_HLSL
#define TINYGLADE_INC_WALL_COLOR_CUSTOMIZATION_HLSL



float3 get_wall_custom_color(uint color_id)
{
    if (color_id == 0u) {
        return color_library.wall_1.xyz;
    } else if (color_id == 1u) {
        return color_library.wall_2.xyz;
    } else if (color_id == 2u) {
        return color_library.wall_bricks.xyz;
    }
    return color_library.wall_moss.xyz;
}



float3 get_wall_default_color(uint brick_hash)
{
    float t = float(brick_hash & 0xFFFFu) / 65535.0;
    return lerp(color_library.wall_1.xyz, color_library.wall_2.xyz, t);
}

#endif 
#line 26 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/inc/highlight_color.hlsl"


#ifndef TINYGLADE_INC_HIGHLIGHT_COLOR_HLSL
#define TINYGLADE_INC_HIGHLIGHT_COLOR_HLSL





#line 1 "shaders/inc/color_library.hlsl"


#ifndef TINYGLADE_INC_COLOR_LIBRARY_HLSL
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL



struct ColorLibrary {
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






#ifndef TINYGLADE_COLOR_LIBRARY_REGISTER
#define TINYGLADE_COLOR_LIBRARY_REGISTER b14
#endif
cbuffer color_library : register(TINYGLADE_COLOR_LIBRARY_REGISTER, space0) {
    ColorLibrary color_library;
};

#endif 
#line 10 "shaders/inc/highlight_color.hlsl"




float3 get_highlight_color(uint glade_theme_id)
{
    float3 c;
    if (glade_theme_id == 0u) {
        c = color_library.temp_color_1.xyz;
    } else if (glade_theme_id == 1u) {
        c = color_library.temp_color_2.xyz;
    } else {
        c = color_library.temp_color_3.xyz;
    }
    return c;
}

#endif 
#line 27 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/inc/material/solid_material.inc.hlsl"


#ifndef TINYGLADE_INC_MATERIAL_SOLID_MATERIAL_HLSL
#define TINYGLADE_INC_MATERIAL_SOLID_MATERIAL_HLSL


struct SolidMaterialParams {
    float3 albedo;
    float roughness;
    uint flags; 
};


SolidMaterialParams init_solid_material(float3 albedo, float roughness, uint flags)
{
    SolidMaterialParams p;
    p.albedo = albedo;
    p.roughness = saturate(roughness);
    p.flags = flags;
    return p;
}

#endif 
#line 28 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/inc/pack_unpack.hlsl"




#ifndef TINYGLADE_INC_PACK_UNPACK_HLSL
#define TINYGLADE_INC_PACK_UNPACK_HLSL


uint pack_rgba8(float4 c)
{
    uint r = uint(clamp(c.x, 0.0f, 1.0f) * 255.0f + 0.5f);
    uint g = uint(clamp(c.y, 0.0f, 1.0f) * 255.0f + 0.5f);
    uint b = uint(clamp(c.z, 0.0f, 1.0f) * 255.0f + 0.5f);
    uint a = uint(clamp(c.w, 0.0f, 1.0f) * 255.0f + 0.5f);
    return (a << 24u) | (b << 16u) | (g << 8u) | r;
}


uint pack_rgb8(float3 c)
{
    uint r = uint(clamp(c.x, 0.0f, 1.0f) * 255.0f + 0.5f);
    uint g = uint(clamp(c.y, 0.0f, 1.0f) * 255.0f + 0.5f);
    uint b = uint(clamp(c.z, 0.0f, 1.0f) * 255.0f + 0.5f);
    return (b << 16u) | (g << 8u) | r;
}


float3 unpack_rgb8(uint p)
{
    float3 c;
    c.r = float(p & 0xFFu) * (1.0f / 255.0f);
    c.g = float((p >> 8u) & 0xFFu) * (1.0f / 255.0f);
    c.b = float((p >> 16u) & 0xFFu) * (1.0f / 255.0f);
    return c;
}


float4 unpack_rgba8(uint p)
{
    return float4(unpack_rgb8(p), float((p >> 24u) & 0xFFu) * (1.0f / 255.0f));
}


float3 unpack_normal_rgb8(uint p)
{
    return unpack_rgb8(p) * 2.0f - 1.0f;
}

#endif 
#line 29 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/inc/material_eval.inc.hlsl"


#ifndef TINYGLADE_INC_MATERIAL_EVAL_HLSL
#define TINYGLADE_INC_MATERIAL_EVAL_HLSL



struct MaterialEvalData {
    float3 albedo;
    float3 normal;
    float roughness;
    float opacity;
    float3 emissive;
};



float3 unpack_gbuffer_normal(uint packed_normal)
{
    float3 n = float3(float(packed_normal & 255u),
                      float((packed_normal >> 8u) & 255u),
                      float((packed_normal >> 16u) & 255u)) * (1.0 / 255.0);
    n = n * 2.0 - 1.0;
    return normalize(n);
}


void init_material_eval_data(out MaterialEvalData d)
{
    d.albedo = float3(1.0, 1.0, 1.0);
    d.normal = float3(0.0, 1.0, 0.0);
    d.roughness = 1.0;
    d.opacity = 1.0;
    d.emissive = float3(0.0, 0.0, 0.0);
}

#endif 
#line 30 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/inc/brdf.hlsl"


#ifndef TINYGLADE_INC_BRDF_HLSL
#define TINYGLADE_INC_BRDF_HLSL



float distribution_ggx(float NdotH, float alphaSq)
{
    float d = NdotH * NdotH * (alphaSq - 1.0) + 1.0;
    return alphaSq / (3.14159265358979323846 * d * d);
}


float visibility_smith_ggx(float NdotL, float NdotV, float alphaSq)
{
    float l2 = NdotL * NdotL;
    float v2 = NdotV * NdotV;
    return (2.0 * NdotL * NdotV) /
        (NdotL * sqrt((1.0 - alphaSq) * v2 + alphaSq) +
         NdotV * sqrt((1.0 - alphaSq) * l2 + alphaSq));
}


float3 fresnel_schlick(float cosTheta, float3 F0)
{
    float c = 1.0 - saturate(cosTheta);
    return F0 + (1.0 - F0) * (c * c * c * c * c);
}



float diffuse_ggx(float NdotL, float NdotV, float NdotH, float roughness)
{
    float r2 = roughness * roughness;
    float f = (2.0 * r2 * NdotH * NdotH) - 0.5;
    float omL = 1.0 - saturate(NdotL);
    float omV = 1.0 - saturate(NdotV);
    return ((1.0 + f * pow(omL, 5.0)) * (1.0 + f * pow(omV, 5.0))) * (1.0 / 3.14159265358979323846);
}


float3 cook_torrance_specular(float NdotL, float NdotV, float NdotH, float HdotL, float roughness)
{
    float alphaSq = roughness * roughness;
    alphaSq = alphaSq * alphaSq; 
    float3 F = fresnel_schlick(HdotL, float3(0.04, 0.04, 0.04));
    float D = distribution_ggx(NdotH, alphaSq);
    float G = visibility_smith_ggx(NdotL, NdotV, alphaSq);
    return (F * D * G) / max(4.0 * NdotV * NdotL, 1e-6);
}


float3 evaluate_brdf(float3 albedo, float roughness, float3 N, float3 V, float3 L)
{
    float NdotL = saturate(dot(N, L));
    float NdotV = saturate(dot(N, V));
    float3 H = normalize(L + V);
    float NdotH = saturate(dot(N, H));
    float HdotL = saturate(dot(H, L));
    float3 diffuse = albedo * diffuse_ggx(NdotL, NdotV, NdotH, roughness);
    float3 specular = cook_torrance_specular(NdotL, NdotV, NdotH, HdotL, roughness);
    return diffuse + specular;
}

#endif 
#line 31 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/inc/math.hlsl"





#ifndef TINYGLADE_INC_MATH_HLSL
#define TINYGLADE_INC_MATH_HLSL


float clamp01(float v)
{
    return clamp(v, 0.0f, 1.0f);
}

float3 clamp01(float3 v)
{
    return clamp(v, 0.0f, 1.0f);
}


float length_squared(float3 v)
{
    return dot(v, v);
}


float3 safe_normalize(float3 v)
{
    float len_sq = dot(v, v);
    float len = sqrt(len_sq);
    return len > 1.0e-6f ? v / len : float3(0.0f, 0.0f, 0.0f);
}


float safe_rcp(float v)
{
    return 1.0f / max(abs(v), 1.0e-6f);
}


float rcp_keep_sign(float v)
{
    return sign(v) * safe_rcp(v);
}

#endif 
#line 32 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/inc/lighting.hlsl"


#ifndef TINYGLADE_INC_LIGHTING_HLSL
#define TINYGLADE_INC_LIGHTING_HLSL

#line 1 "shaders/inc/sun.inc.hlsl"


#ifndef TINYGLADE_INC_SUN_HLSL
#define TINYGLADE_INC_SUN_HLSL

#line 1 "shaders/inc/color_library.hlsl"


#ifndef TINYGLADE_INC_COLOR_LIBRARY_HLSL
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL



struct ColorLibrary {
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






#ifndef TINYGLADE_COLOR_LIBRARY_REGISTER
#define TINYGLADE_COLOR_LIBRARY_REGISTER b14
#endif
cbuffer color_library : register(TINYGLADE_COLOR_LIBRARY_REGISTER, space0) {
    ColorLibrary color_library;
};

#endif 
#line 6 "shaders/inc/sun.inc.hlsl"



float3 get_sun_color()
{
    return sun_color.xyz;
}

float3 get_sky_low_color()
{
    return sky_low_color.xyz;
}

float3 get_sky_high_color()
{
    return sky_high_color.xyz;
}

float3 get_sun_direction()
{
    return dir_to_sun;
}


float3 get_sky_color(float3 day_low, float3 day_high, float3 dusk_low, float3 dusk_high)
{
    float t = saturate(time_of_day);
    float3 low = lerp(day_low, dusk_low, t);
    float3 high = lerp(day_high, dusk_high, t);
    return low;
}

#endif 
#line 6 "shaders/inc/lighting.hlsl"

#line 1 "shaders/inc/brdf.hlsl"


#ifndef TINYGLADE_INC_BRDF_HLSL
#define TINYGLADE_INC_BRDF_HLSL



float distribution_ggx(float NdotH, float alphaSq)
{
    float d = NdotH * NdotH * (alphaSq - 1.0) + 1.0;
    return alphaSq / (3.14159265358979323846 * d * d);
}


float visibility_smith_ggx(float NdotL, float NdotV, float alphaSq)
{
    float l2 = NdotL * NdotL;
    float v2 = NdotV * NdotV;
    return (2.0 * NdotL * NdotV) /
        (NdotL * sqrt((1.0 - alphaSq) * v2 + alphaSq) +
         NdotV * sqrt((1.0 - alphaSq) * l2 + alphaSq));
}


float3 fresnel_schlick(float cosTheta, float3 F0)
{
    float c = 1.0 - saturate(cosTheta);
    return F0 + (1.0 - F0) * (c * c * c * c * c);
}



float diffuse_ggx(float NdotL, float NdotV, float NdotH, float roughness)
{
    float r2 = roughness * roughness;
    float f = (2.0 * r2 * NdotH * NdotH) - 0.5;
    float omL = 1.0 - saturate(NdotL);
    float omV = 1.0 - saturate(NdotV);
    return ((1.0 + f * pow(omL, 5.0)) * (1.0 + f * pow(omV, 5.0))) * (1.0 / 3.14159265358979323846);
}


float3 cook_torrance_specular(float NdotL, float NdotV, float NdotH, float HdotL, float roughness)
{
    float alphaSq = roughness * roughness;
    alphaSq = alphaSq * alphaSq; 
    float3 F = fresnel_schlick(HdotL, float3(0.04, 0.04, 0.04));
    float D = distribution_ggx(NdotH, alphaSq);
    float G = visibility_smith_ggx(NdotL, NdotV, alphaSq);
    return (F * D * G) / max(4.0 * NdotV * NdotL, 1e-6);
}


float3 evaluate_brdf(float3 albedo, float roughness, float3 N, float3 V, float3 L)
{
    float NdotL = saturate(dot(N, L));
    float NdotV = saturate(dot(N, V));
    float3 H = normalize(L + V);
    float NdotH = saturate(dot(N, H));
    float HdotL = saturate(dot(H, L));
    float3 diffuse = albedo * diffuse_ggx(NdotL, NdotV, NdotH, roughness);
    float3 specular = cook_torrance_specular(NdotL, NdotV, NdotH, HdotL, roughness);
    return diffuse + specular;
}

#endif 
#line 7 "shaders/inc/lighting.hlsl"




float3 evaluate_sun_light(float3 albedo, float roughness, float3 normal, float3 view_dir,
                          float3 sun_dir, float shadow)
{
    float NdotL = saturate(dot(normal, sun_dir));
    float NdotV = saturate(dot(normal, view_dir));
    float3 H = normalize(view_dir + sun_dir);
    float NdotH = saturate(dot(normal, H));
    float HdotL = saturate(dot(H, sun_dir));
    float3 diffuse = albedo * diffuse_ggx(NdotL, NdotV, NdotH, roughness);
    float3 specular = cook_torrance_specular(NdotL, NdotV, NdotH, HdotL, roughness);
    return get_sun_color() * (diffuse + specular) * NdotL * shadow;
}



float3 evaluate_sky_ambient(float3 normal)
{
    float up = saturate(normal.y) * 0.5 + 0.5;
    float3 sky = lerp(get_sky_low_color(), get_sky_high_color(), up);
    return sky * color_library.indirect_light_tint.xyz;
}


float3 evaluate_lighting(float3 albedo, float roughness, float3 normal, float3 view_dir,
                        float3 world_pos, float shadow, float ao)
{
    float3 sun = evaluate_sun_light(albedo, roughness, normal, view_dir, get_sun_direction(), shadow);
    float3 ambient = evaluate_sky_ambient(normal) * albedo * ao;
    return sun + ambient;
}

#endif 
#line 33 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/inc/sun.inc.hlsl"


#ifndef TINYGLADE_INC_SUN_HLSL
#define TINYGLADE_INC_SUN_HLSL

#line 1 "shaders/inc/color_library.hlsl"


#ifndef TINYGLADE_INC_COLOR_LIBRARY_HLSL
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL



struct ColorLibrary {
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






#ifndef TINYGLADE_COLOR_LIBRARY_REGISTER
#define TINYGLADE_COLOR_LIBRARY_REGISTER b14
#endif
cbuffer color_library : register(TINYGLADE_COLOR_LIBRARY_REGISTER, space0) {
    ColorLibrary color_library;
};

#endif 
#line 6 "shaders/inc/sun.inc.hlsl"



float3 get_sun_color()
{
    return sun_color.xyz;
}

float3 get_sky_low_color()
{
    return sky_low_color.xyz;
}

float3 get_sky_high_color()
{
    return sky_high_color.xyz;
}

float3 get_sun_direction()
{
    return dir_to_sun;
}


float3 get_sky_color(float3 day_low, float3 day_high, float3 dusk_low, float3 dusk_high)
{
    float t = saturate(time_of_day);
    float3 low = lerp(day_low, dusk_low, t);
    float3 high = lerp(day_high, dusk_high, t);
    return low;
}

#endif 
#line 34 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/inc/color_library.hlsl"


#ifndef TINYGLADE_INC_COLOR_LIBRARY_HLSL
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL



struct ColorLibrary {
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






#ifndef TINYGLADE_COLOR_LIBRARY_REGISTER
#define TINYGLADE_COLOR_LIBRARY_REGISTER b14
#endif
cbuffer color_library : register(TINYGLADE_COLOR_LIBRARY_REGISTER, space0) {
    ColorLibrary color_library;
};

#endif 
#line 35 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/inc/color/srgb.hlsl"




#ifndef TINYGLADE_INC_COLOR_SRGB_HLSL
#define TINYGLADE_INC_COLOR_SRGB_HLSL

static const float SRGB_ALPHA = 0.04045f;
static const float SRGB_OFFSET = 0.055f;
static const float SRGB_GAMMA = 2.4f;


float srgb_to_linear(float c)
{
    float r;
    if (c <= SRGB_ALPHA)
        r = c * 0.077399380505084991455078125f; 
    else
        r = pow((c + SRGB_OFFSET) * 0.947867333889007568359375f, SRGB_GAMMA); 
    return r;
}

float3 srgb_to_linear(float3 c)
{
    return float3(srgb_to_linear(c.r), srgb_to_linear(c.g), srgb_to_linear(c.b));
}

float4 srgb_to_linear(float4 c)
{
    return float4(srgb_to_linear(c.rgb), c.a);
}


float linear_to_srgb(float c)
{
    float r;
    if (c <= 0.0031308f)
        r = c * 12.92f;
    else
        r = 1.055f * pow(c, 1.0f / SRGB_GAMMA) - SRGB_OFFSET;
    return r;
}

float3 linear_to_srgb(float3 c)
{
    return float3(linear_to_srgb(c.r), linear_to_srgb(c.g), linear_to_srgb(c.b));
}

float4 linear_to_srgb(float4 c)
{
    return float4(linear_to_srgb(c.rgb), c.a);
}

#endif 
#line 36 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 37 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/inc/hash.hlsl"







#ifndef TINYGLADE_INC_HASH_HLSL
#define TINYGLADE_INC_HASH_HLSL

#line 1 "shaders/inc/math_const.hlsl"





#ifndef TINYGLADE_INC_MATH_CONST_HLSL
#define TINYGLADE_INC_MATH_CONST_HLSL

static const float PI = 3.14159265358979323846264338327950288f;
static const float TWO_PI = 6.28318530717958647692528676655900577f;
static const float HALF_PI = 1.57079632679489661923132169163975144f;
static const float INV_PI = 0.31830988618379067153776752674502872f;
static const float INV_TWO_PI = 0.15915494309189533576888376337251436f;

static const float SQRT_TWO = 1.41421356237309504880168872420969808f;
static const float INV_SQRT_TWO = 0.70710678118654752440084436210484904f;

static const float GOLDEN_RATIO = 1.61803398874989484820458683436563812f;
static const float INV_GOLDEN_RATIO = 0.61803398874989484820458683436563812f;

static const float EPSILON = 1.0e-6f;
static const float FLT_EPSILON = 1.19209290e-7f;

#endif 
#line 11 "shaders/inc/hash.hlsl"



uint hash(uint x)
{
    x += (x << 10u);
    x ^= (x >> 6u);
    x += (x << 3u);
    x ^= (x >> 11u);
    x += (x << 15u);
    return x;
}


float rand01(uint x)
{
    return asfloat((x & 0x7FFFFFu) | 0x3F800000u) - 1.0f;
}


float hash_angle(uint x)
{
    return frac(0.9f + (float(x) * GOLDEN_RATIO)) * TWO_PI;
}


float hash01(float x)
{
    return frac(sin(x) * 43758.5453f);
}

#endif 
#line 38 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/gi/gi_settings.inc.hlsl"














#ifndef TINYGLADE_GI_SETTINGS_INC_HLSL
#define TINYGLADE_GI_SETTINGS_INC_HLSL




static const uint2 hi_px_subpixels[4] = {
    uint2(0u, 0u),
    uint2(0u, 1u),
    uint2(1u, 1u),
    uint2(1u, 0u),
};



static const float GI_MIN_HALF_NORMAL_Z = 0.3f;          
static const float GI_DEPTH_WEIGHT_SCALE = 80.0f;        
static const float GI_DEPTH_WEIGHT_MAX = 30.0f;
static const float GI_NORMAL_DOT_DAMP = -0.9f;           
static const float GI_FINAL_WEIGHT_NORM = 0.01f;         

#endif 
#line 39 "shaders/nani_roof_floor.raster.hlsl"

#line 1 "shaders/inc/sh.hlsl"










float4 sh_basis(float3 dir)
{
    return float4(0.2820949852466583251953125f, 0.48860299587249755859375f * dir);
}

float sh_eval(float4 sh, float3 dir)
{
    return dot(sh, sh_basis(dir));
}
#line 40 "shaders/nani_roof_floor.raster.hlsl"


uint highlighted_wall_id;
uint highlight_mode;

struct InstanceData
{
    Affine3Packed xform;
    uint material_id;
};

StructuredBuffer<uint> instance_indirection_buffer : register(t14);
Texture2D<float4> dirthpath_heightmap : register(t15);
StructuredBuffer<InstanceData> nani_instance_data : register(t16);
ByteAddressBuffer dormer_holes : register(t18);
ByteAddressBuffer dormer_hole_count : register(t19);
ByteAddressBuffer halftimber_color_ids : register(t20);
Texture2DArray<float4> halftimber_colors : register(t21);
Texture2D<float4> wooden_floor_normal : register(t22);
Texture2D<float4> wooden_floor_height : register(t23);
Texture2D<float4> wooden_floor_seed : register(t24);
Texture2D<float4> wooden_floor_roughness : register(t25);
Texture2D<float4> wooden_floor_normal_2 : register(t26);
Texture2D<float4> wooden_floor_height_2 : register(t27);
Texture2D<float4> wooden_floor_seed_2 : register(t28);
Texture2D<float4> wooden_floor_roughness_2 : register(t29);
Texture2D<float4> wooden_floor_normal_3 : register(t30);
Texture2D<float4> wooden_floor_height_3 : register(t31);
Texture2D<float4> wooden_floor_seed_3 : register(t32);
Texture2D<float4> wooden_floor_roughness_3 : register(t33);
ByteAddressBuffer wall_color_ids : register(t34);
Texture2DArray<float4> brick_colors : register(t35);
Texture2D<float4> stone_floor_normal : register(t37);
Texture2D<float4> stone_floor_seed : register(t38);
Texture2D<float4> stone_floor_height : register(t39);
Texture2D<float4> stone_floor_roughness : register(t40);
Texture2D<float4> stone_floor_seed_2 : register(t41);
Texture2D<float4> stone_floor_normal_2 : register(t42);
Texture2D<float4> stone_floor_height_2 : register(t43);
Texture2D<float4> stone_floor_roughness_2 : register(t44);
Texture2D<float4> stone_floor_seed_3 : register(t45);
Texture2D<float4> stone_floor_normal_3 : register(t46);
Texture2D<float4> stone_floor_height_3 : register(t47);
Texture2D<float4> stone_floor_roughness_3 : register(t48);

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    nointerpolation uint C4 : TEXCOORD4;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    uint idx = instance_indirection_buffer[vin.instance_id];
    Affine3Packed packed = nani_instance_data[idx].xform;
    float3x4 xform = transpose(float4x3(float3(packed.data[0], packed.data[1], packed.data[2]),
                                        float3(packed.data[3], packed.data[4], packed.data[5]),
                                        float3(packed.data[6], packed.data[7], packed.data[8]),
                                        float3(packed.data[9], packed.data[10], packed.data[11])));
    float3 world = mul(xform, float4(vin.Vertex_Position, 1.0f));
    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(world, 1.0f));
    o.C1 = mul(xform, float4(0.0f, 1.0f, 0.0f, 0.0f));
    o.C2 = world;
    o.C4 = idx;
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    nointerpolation uint C4 : TEXCOORD4;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    InstanceData inst = nani_instance_data[pin.C4];
    float2 floor_uv = pin.C2.xz * 0.2f;
    float3 normal = normalize(pin.C1);

    float3 albedo;
    float3 normal_map;
    float seed_val;
    uint variant = inst.material_id & 3u;
    if (inst.material_id < 3u)
    {
        float height_lod;
        float rough;
        float3 nm;
        float sd;
        if (variant == 0u)
        {
            height_lod = stone_floor_height.CalculateLevelOfDetail(sampler_llr, floor_uv) + global_mip_bias;
            rough = stone_floor_roughness.SampleBias(sampler_llr, floor_uv, height_lod).x;
            nm = stone_floor_normal.SampleBias(sampler_llr, floor_uv, height_lod).xyz * 2.0f - 1.0f.xxx;
            sd = stone_floor_seed.SampleBias(sampler_llr, floor_uv, height_lod).x;
        }
        else if (variant == 1u)
        {
            height_lod = stone_floor_height_2.CalculateLevelOfDetail(sampler_llr, floor_uv) + global_mip_bias;
            rough = stone_floor_roughness_2.SampleBias(sampler_llr, floor_uv, height_lod).x;
            nm = stone_floor_normal_2.SampleBias(sampler_llr, floor_uv, height_lod).xyz * 2.0f - 1.0f.xxx;
            sd = stone_floor_seed_2.SampleBias(sampler_llr, floor_uv, height_lod).x;
        }
        else
        {
            height_lod = stone_floor_height_3.CalculateLevelOfDetail(sampler_llr, floor_uv) + global_mip_bias;
            rough = stone_floor_roughness_3.SampleBias(sampler_llr, floor_uv, height_lod).x;
            nm = stone_floor_normal_3.SampleBias(sampler_llr, floor_uv, height_lod).xyz * 2.0f - 1.0f.xxx;
            sd = stone_floor_seed_3.SampleBias(sampler_llr, floor_uv, height_lod).x;
        }
        albedo = lerp(0.8f.xxx, 0.9f.xxx, rough);
        normal_map = nm;
        seed_val = sd;
        albedo *= 0.9f + seed_val * 0.2f;
    }
    else
    {
        uint wv = variant;
        float height_lod;
        float rough;
        float3 nm;
        float sd;
        if (wv == 0u)
        {
            height_lod = wooden_floor_height.CalculateLevelOfDetail(sampler_llr, floor_uv) + global_mip_bias;
            rough = wooden_floor_roughness.SampleBias(sampler_llr, floor_uv, height_lod).x;
            nm = wooden_floor_normal.SampleBias(sampler_llr, floor_uv, height_lod).xyz * 2.0f - 1.0f.xxx;
            sd = wooden_floor_seed.SampleBias(sampler_llr, floor_uv, height_lod).x;
        }
        else if (wv == 1u)
        {
            height_lod = wooden_floor_height_2.CalculateLevelOfDetail(sampler_llr, floor_uv) + global_mip_bias;
            rough = wooden_floor_roughness_2.SampleBias(sampler_llr, floor_uv, height_lod).x;
            nm = wooden_floor_normal_2.SampleBias(sampler_llr, floor_uv, height_lod).xyz * 2.0f - 1.0f.xxx;
            sd = wooden_floor_seed_2.SampleBias(sampler_llr, floor_uv, height_lod).x;
        }
        else
        {
            height_lod = wooden_floor_height_3.CalculateLevelOfDetail(sampler_llr, floor_uv) + global_mip_bias;
            rough = wooden_floor_roughness_3.SampleBias(sampler_llr, floor_uv, height_lod).x;
            nm = wooden_floor_normal_3.SampleBias(sampler_llr, floor_uv, height_lod).xyz * 2.0f - 1.0f.xxx;
            sd = wooden_floor_seed_3.SampleBias(sampler_llr, floor_uv, height_lod).x;
        }
        albedo = lerp(0.6f.xxx, 0.8f.xxx, rough);
        normal_map = nm;
        seed_val = sd;
        albedo *= 0.9f + seed_val * 0.2f;
    }
    normal = normalize(normal + (normal_map * 0.4f));
    uint wall_id = nani_instance_data[pin.C4].material_id & 0xFFFFu;
    int timber_color_id = int(halftimber_color_ids.Load<uint>(wall_id * 4 + 0));
    float3 timber = halftimber_colors.SampleBias(sampler_llr, float3(floor_uv, float(timber_color_id)), global_mip_bias).xyz;
    albedo = lerp(albedo, timber, 0.3f);
    float3 brick = brick_colors.SampleBias(sampler_llr, float3(floor_uv, float(wall_color_ids.Load<uint>(wall_id * 4 + 0))), global_mip_bias).xyz;
    albedo = lerp(albedo, brick, 0.1f);
    uint dormer_cell = uint(int(floor(floor_uv.x * 8.0f)) + 15 * int(floor(floor_uv.y * 8.0f)));
    uint dormer_count = min(128u, dormer_hole_count.Load<uint>(dormer_cell * 4 + 0));
    for (uint i = 0u; i < dormer_count; i++)
    {
        uint hole_idx = uint(int(uint(dormer_cell * 50) + i));
        if (dormer_holes.Load<uint>(hole_idx * 48 + 12) == wall_id)
        {
            discard;
        }
    }
    if (glade_theme_id == 4u)
    {
        float path = dirthpath_heightmap.SampleBias(sampler_llr, pin.C2.xz * 0.05f, global_mip_bias).x;
        albedo = lerp(albedo, 0.7f.xxx, (path * smoothstep(-0.1f, 0.0f, pin.C2.y) * clamp(normal.y, 0.0f, 1.0f)).xxx);
    }
    if (highlight_mode == 1u && wall_id == highlighted_wall_id)
    {
        albedo = lerp(albedo, float3(0.7f, 0.21f, 0.0f), 0.8f.xxx);
    }
    float3 packed_normal = (normal * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = uint(clamp(srgb_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(srgb_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(srgb_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u);
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 3422552064u;
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = o.gbuffer.y | 117440512u;
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct ViewConstants
{
    column_major float4x4 view_to_clip;
    column_major float4x4 clip_to_view;
    column_major float4x4 view_to_sample;
    column_major float4x4 sample_to_view;
    column_major float4x4 world_to_view;
    column_major float4x4 view_to_world;
    column_major float4x4 world_to_sample;
    column_major float4x4 clip_to_prev_clip;
    column_major float4x4 prev_view_to_prev_clip;
    column_major float4x4 prev_clip_to_prev_view;
    column_major float4x4 prev_world_to_prev_view;
    column_major float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
};

struct InstanceData
{
    float3 position;
    uint wall_id;
    float2 basis0_xz;
    float2 scale;
    float roof_animation;
    uint shape;
    uint double_sided;
    uint is_roof_bottom;
    uint material_id;
    uint override_color_id;
    uint pattern_id;
    uint pad_1;
};

cbuffer frame_constants : register(b0, space1)
{
    float4 frame_constants_sun_color : packoffset(c0);
    float4 frame_constants_sky_low_color : packoffset(c1);
    float4 frame_constants_sky_high_color : packoffset(c2);
    float4 frame_constants_fog_tint_thickness : packoffset(c3);
    float4 frame_constants_fake_light_color : packoffset(c4);
    float4 frame_constants_fake_light_dir : packoffset(c5);
    float4 frame_constants_fake_light2_color : packoffset(c6);
    float4 frame_constants_fake_light2_dir : packoffset(c7);
    uint frame_constants_frame_index : packoffset(c8);
    float frame_constants_delta_time_seconds : packoffset(c8.y);
    float frame_constants_ev_mult : packoffset(c8.z);
    uint frame_constants_glade_theme_id : packoffset(c8.w);
    uint2 frame_constants_inner_rendering_resolution : packoffset(c9);
    uint frame_constants_game_time_sec : packoffset(c9.z);
    float frame_constants_game_time_frac : packoffset(c9.w);
    float3 frame_constants_editing_highlight_color : packoffset(c10);
    float frame_constants_time_of_day : packoffset(c10.w);
    float3 frame_constants_dir_to_sun : packoffset(c11);
    uint frame_constants_camera_idx : packoffset(c11.w);
    float3 frame_constants_dir_to_moon : packoffset(c12);
    uint frame_constants_render_overrides : packoffset(c12.w);
    float frame_constants_global_mip_bias : packoffset(c13);
    uint frame_constants_highlighted_transient_stair_edge_id : packoffset(c13.y);
    uint frame_constants_pad1 : packoffset(c13.z);
    uint frame_constants_pad2 : packoffset(c13.w);
    float4 frame_constants_debug_tweakers : packoffset(c14);
    ViewConstants frame_constants_view_constants[5] : packoffset(c15);
};

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index : packoffset(c0);
    uint2 dirthpath_heightmap_dims : packoffset(c0.y);
    uint highlighted_wall_id : packoffset(c0.w);
    uint highlight_mode : packoffset(c1);
    uint2 wooden_floor_normal_dims : packoffset(c1.y);
    uint2 wooden_floor_height_dims : packoffset(c2);
    uint2 wooden_floor_seed_dims : packoffset(c2.z);
    uint2 wooden_floor_roughness_dims : packoffset(c3);
    uint2 wooden_floor_normal_2_dims : packoffset(c3.z);
    uint2 wooden_floor_height_2_dims : packoffset(c4);
    uint2 wooden_floor_seed_2_dims : packoffset(c4.z);
    uint2 wooden_floor_roughness_2_dims : packoffset(c5);
    uint2 wooden_floor_normal_3_dims : packoffset(c5.z);
    uint2 wooden_floor_height_3_dims : packoffset(c6);
    uint2 wooden_floor_seed_3_dims : packoffset(c6.z);
    uint2 wooden_floor_roughness_3_dims : packoffset(c7);
    uint2 stone_floor_normal_dims : packoffset(c7.z);
    uint2 stone_floor_seed_dims : packoffset(c8);
    uint2 stone_floor_height_dims : packoffset(c8.z);
    uint2 stone_floor_roughness_dims : packoffset(c9);
    uint2 stone_floor_seed_2_dims : packoffset(c9.z);
    uint2 stone_floor_normal_2_dims : packoffset(c10);
    uint2 stone_floor_height_2_dims : packoffset(c10.z);
    uint2 stone_floor_roughness_2_dims : packoffset(c11);
    uint2 stone_floor_seed_3_dims : packoffset(c11.z);
    uint2 stone_floor_normal_3_dims : packoffset(c12);
    uint2 stone_floor_height_3_dims : packoffset(c12.z);
    uint2 stone_floor_roughness_3_dims : packoffset(c13);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static uint out_var_C4;
static uint out_var_C5;
static uint out_var_C6;
static uint out_var_C7;
static uint out_var_C8;
static float3x3 out_var_C9;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] nointerpolation uint out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] nointerpolation uint out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] nointerpolation uint out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] nointerpolation uint out_var_C7 : TEXCOORD7;
    [[vk::location(8)]] nointerpolation uint out_var_C8 : TEXCOORD8;
    [[vk::location(9)]] float3x3 out_var_C9 : TEXCOORD9;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float3 _141 = float3(nani_instance_data.Load<float2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16), 0.0f).xzy;
    float3 _143 = float3(-nani_instance_data.Load<float2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16).y, 0.0f, nani_instance_data.Load<float2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16).x);
    float4 _148 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    bool _157 = (nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 44) != 0u) ? true : ((nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0) - (_148.xyz / _148.w.xxx)).y > 0.0f);
    float3 _161;
    if (_157)
    {
        _161 = _141 * (-1.0f);
    }
    else
    {
        _161 = _141;
    }
    bool3 _162 = _157.xxx;
    float3 _163 = float3(_162.x ? float3(-0.0f, -1.0f, -0.0f).x : float3(0.0f, 1.0f, 0.0f).x, _162.y ? float3(-0.0f, -1.0f, -0.0f).y : float3(0.0f, 1.0f, 0.0f).y, _162.z ? float3(-0.0f, -1.0f, -0.0f).z : float3(0.0f, 1.0f, 0.0f).z);
    float2 _165 = in_var_Vertex_Position.xz * nani_instance_data.Load<float2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 24);
    float3 _168 = mul(_165, float2x3(_161, _143)) + nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0);
    float _176 = 0.001953125f * (floor(_168.y * 512.0f) + frac(float(nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 12)) * 1.61803400516510009765625f));
    float3 _177 = _168;
    _177.y = _176;
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_168.x, _176, _168.z, 1.0f));
    out_var_C1 = _163;
    out_var_C2 = _177;
    out_var_C3 = _165;
    out_var_C4 = instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0);
    out_var_C5 = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 44);
    out_var_C6 = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 48);
    out_var_C7 = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52);
    out_var_C8 = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 56);
    out_var_C9 = transpose(float3x3(_161, _143, _163));
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
    stage_output.out_var_C7 = out_var_C7;
    stage_output.out_var_C8 = out_var_C8;
    stage_output.out_var_C9 = out_var_C9;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct ViewConstants
{
    column_major float4x4 view_to_clip;
    column_major float4x4 clip_to_view;
    column_major float4x4 view_to_sample;
    column_major float4x4 sample_to_view;
    column_major float4x4 world_to_view;
    column_major float4x4 view_to_world;
    column_major float4x4 world_to_sample;
    column_major float4x4 clip_to_prev_clip;
    column_major float4x4 prev_view_to_prev_clip;
    column_major float4x4 prev_clip_to_prev_view;
    column_major float4x4 prev_world_to_prev_view;
    column_major float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
};

struct InstanceData
{
    float3 position;
    uint wall_id;
    float2 basis0_xz;
    float2 scale;
    float roof_animation;
    uint shape;
    uint double_sided;
    uint is_roof_bottom;
    uint material_id;
    uint override_color_id;
    uint pattern_id;
    uint pad_1;
};

struct DormerHole
{
    float3 position;
    int wall_id;
    float3 size;
    int pad0;
    float2 normal;
    float k;
    float a;
};

static bool _225;

cbuffer frame_constants : register(b0, space1)
{
    float4 frame_constants_sun_color : packoffset(c0);
    float4 frame_constants_sky_low_color : packoffset(c1);
    float4 frame_constants_sky_high_color : packoffset(c2);
    float4 frame_constants_fog_tint_thickness : packoffset(c3);
    float4 frame_constants_fake_light_color : packoffset(c4);
    float4 frame_constants_fake_light_dir : packoffset(c5);
    float4 frame_constants_fake_light2_color : packoffset(c6);
    float4 frame_constants_fake_light2_dir : packoffset(c7);
    uint frame_constants_frame_index : packoffset(c8);
    float frame_constants_delta_time_seconds : packoffset(c8.y);
    float frame_constants_ev_mult : packoffset(c8.z);
    uint frame_constants_glade_theme_id : packoffset(c8.w);
    uint2 frame_constants_inner_rendering_resolution : packoffset(c9);
    uint frame_constants_game_time_sec : packoffset(c9.z);
    float frame_constants_game_time_frac : packoffset(c9.w);
    float3 frame_constants_editing_highlight_color : packoffset(c10);
    float frame_constants_time_of_day : packoffset(c10.w);
    float3 frame_constants_dir_to_sun : packoffset(c11);
    uint frame_constants_camera_idx : packoffset(c11.w);
    float3 frame_constants_dir_to_moon : packoffset(c12);
    uint frame_constants_render_overrides : packoffset(c12.w);
    float frame_constants_global_mip_bias : packoffset(c13);
    uint frame_constants_highlighted_transient_stair_edge_id : packoffset(c13.y);
    uint frame_constants_pad1 : packoffset(c13.z);
    uint frame_constants_pad2 : packoffset(c13.w);
    float4 frame_constants_debug_tweakers : packoffset(c14);
    ViewConstants frame_constants_view_constants[5] : packoffset(c15);
};

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index : packoffset(c0);
    uint2 dirthpath_heightmap_dims : packoffset(c0.y);
    uint highlighted_wall_id : packoffset(c0.w);
    uint highlight_mode : packoffset(c1);
    uint2 wooden_floor_normal_dims : packoffset(c1.y);
    uint2 wooden_floor_height_dims : packoffset(c2);
    uint2 wooden_floor_seed_dims : packoffset(c2.z);
    uint2 wooden_floor_roughness_dims : packoffset(c3);
    uint2 wooden_floor_normal_2_dims : packoffset(c3.z);
    uint2 wooden_floor_height_2_dims : packoffset(c4);
    uint2 wooden_floor_seed_2_dims : packoffset(c4.z);
    uint2 wooden_floor_roughness_2_dims : packoffset(c5);
    uint2 wooden_floor_normal_3_dims : packoffset(c5.z);
    uint2 wooden_floor_height_3_dims : packoffset(c6);
    uint2 wooden_floor_seed_3_dims : packoffset(c6.z);
    uint2 wooden_floor_roughness_3_dims : packoffset(c7);
    uint2 stone_floor_normal_dims : packoffset(c7.z);
    uint2 stone_floor_seed_dims : packoffset(c8);
    uint2 stone_floor_height_dims : packoffset(c8.z);
    uint2 stone_floor_roughness_dims : packoffset(c9);
    uint2 stone_floor_seed_2_dims : packoffset(c9.z);
    uint2 stone_floor_normal_2_dims : packoffset(c10);
    uint2 stone_floor_height_2_dims : packoffset(c10.z);
    uint2 stone_floor_roughness_2_dims : packoffset(c11);
    uint2 stone_floor_seed_3_dims : packoffset(c11.z);
    uint2 stone_floor_normal_3_dims : packoffset(c12);
    uint2 stone_floor_height_3_dims : packoffset(c12.z);
    uint2 stone_floor_roughness_3_dims : packoffset(c13);
};

ByteAddressBuffer nani_instance_data : register(t16, space0);
ByteAddressBuffer dormer_holes : register(t18, space0);
ByteAddressBuffer dormer_hole_count : register(t19, space0);
ByteAddressBuffer halftimber_color_ids : register(t20, space0);
ByteAddressBuffer wall_color_ids : register(t34, space0);
SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> dirthpath_heightmap : register(t15, space0);
Texture2DArray<float4> halftimber_colors : register(t21, space0);
Texture2D<float4> wooden_floor_normal : register(t22, space0);
Texture2D<float4> wooden_floor_height : register(t23, space0);
Texture2D<float4> wooden_floor_seed : register(t24, space0);
Texture2D<float4> wooden_floor_roughness : register(t25, space0);
Texture2D<float4> wooden_floor_normal_2 : register(t26, space0);
Texture2D<float4> wooden_floor_height_2 : register(t27, space0);
Texture2D<float4> wooden_floor_seed_2 : register(t28, space0);
Texture2D<float4> wooden_floor_roughness_2 : register(t29, space0);
Texture2D<float4> wooden_floor_normal_3 : register(t30, space0);
Texture2D<float4> wooden_floor_height_3 : register(t31, space0);
Texture2D<float4> wooden_floor_seed_3 : register(t32, space0);
Texture2D<float4> wooden_floor_roughness_3 : register(t33, space0);
Texture2DArray<float4> brick_colors : register(t35, space0);
Texture2D<float4> stone_floor_normal : register(t37, space0);
Texture2D<float4> stone_floor_seed : register(t38, space0);
Texture2D<float4> stone_floor_height : register(t39, space0);
Texture2D<float4> stone_floor_roughness : register(t40, space0);
Texture2D<float4> stone_floor_seed_2 : register(t41, space0);
Texture2D<float4> stone_floor_normal_2 : register(t42, space0);
Texture2D<float4> stone_floor_height_2 : register(t43, space0);
Texture2D<float4> stone_floor_roughness_2 : register(t44, space0);
Texture2D<float4> stone_floor_seed_3 : register(t45, space0);
Texture2D<float4> stone_floor_normal_3 : register(t46, space0);
Texture2D<float4> stone_floor_height_3 : register(t47, space0);
Texture2D<float4> stone_floor_roughness_3 : register(t48, space0);

static float4 gl_FragCoord;
static float gl_FragDepth;
static float3 in_var_C1;
static float3 in_var_C2;
static float2 in_var_C3;
static uint in_var_C4;
static uint in_var_C6;
static uint in_var_C7;
static uint in_var_C8;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 in_var_C3 : TEXCOORD3;
    [[vk::location(4)]] nointerpolation uint in_var_C4 : TEXCOORD4;
    [[vk::location(6)]] nointerpolation uint in_var_C6 : TEXCOORD6;
    [[vk::location(7)]] nointerpolation uint in_var_C7 : TEXCOORD7;
    [[vk::location(8)]] nointerpolation uint in_var_C8 : TEXCOORD8;
    float4 gl_FragCoord : SV_Position;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
    float gl_FragDepth : SV_DepthLessEqual;
};

void frag_main()
{
    bool _266;
    if (1u == highlight_mode)
    {
        _266 = nani_instance_data.Load<uint>(in_var_C4 * 64 + 12) == highlighted_wall_id;
    }
    else
    {
        _266 = false;
    }
    bool _273;
    if (2u == highlight_mode)
    {
        _273 = nani_instance_data.Load<uint>(in_var_C4 * 64 + 12) == highlighted_wall_id;
    }
    else
    {
        _273 = false;
    }
    float2 _277;
    int _274 = int(nani_instance_data.Load<uint>(in_var_C4 * 64 + 12));
    bool _372;
    do
    {
        _277 = in_var_C2.xz;
        float2 _279 = (_277 + 65.0f.xx) * 0.1153846085071563720703125f.xx;
        int _287 = int(floor(_279.x)) + (15 * int(floor(_279.y)));
        uint _291 = min(128u, dormer_hole_count.Load<uint>(uint(_287) * 4 + 0));
        bool _369;
        bool _370;
        uint _293 = 0u;
        for (;;)
        {
            if (_293 < _291)
            {
                uint _303 = uint(int(uint(_287 * 50) + _293));
                if (_274 == dormer_holes.Load<int>(_303 * 48 + 12))
                {
                    float3 _319 = float3(dormer_holes.Load<float>(_303 * 48 + 32), 0.0f, dormer_holes.Load<float>(_303 * 48 + 36));
                    float3x3 _323 = transpose(float3x3(cross(_319, float3(0.0f, 1.0f, 0.0f)), float3(0.0f, 1.0f, 0.0f), _319));
                    float3 _324 = mul(_323, float3(1.0f, 0.0f, 0.0f));
                    float3 _325 = mul(_323, float3(0.0f, 1.0f, 0.0f));
                    float3 _326 = mul(_323, float3(0.0f, 0.0f, 1.0f));
                    float3 _329 = cross(_324, _325);
                    float3 _332 = (1.0f / dot(_326, _329)).xxx;
                    float3 _340 = mul(in_var_C2 - dormer_holes.Load<float3>(_303 * 48 + 0), float3x3(cross(_325, _326) * _332, cross(_326, _324) * _332, _329 * _332)) / dormer_holes.Load<float3>(_303 * 48 + 16);
                    float _341 = _340.y;
                    float _344 = clamp((_341 * dormer_holes.Load<float>(_303 * 48 + 40)) + dormer_holes.Load<float>(_303 * 48 + 44), 0.0f, 1.0f);
                    bool _360;
                    if (abs(_340.x) < lerp(0.5f, 0.0f, (_344 * _344) * _344))
                    {
                        bool _359;
                        if (_341 < ((dormer_holes.Load<float>(_303 * 48 + 40) == 1.0f) ? 0.5f : 0.699999988079071044921875f))
                        {
                            _359 = _341 > (-0.5f);
                        }
                        else
                        {
                            _359 = false;
                        }
                        _360 = _359;
                    }
                    else
                    {
                        _360 = false;
                    }
                    bool _366;
                    if (_360)
                    {
                        _366 = abs(_340.z) < 0.5f;
                    }
                    else
                    {
                        _366 = false;
                    }
                    if (_366)
                    {
                        _369 = true;
                        _370 = true;
                        break;
                    }
                }
                _293++;
                continue;
            }
            else
            {
                _369 = _225;
                _370 = false;
                break;
            }
        }
        if (_370)
        {
            _372 = _369;
            break;
        }
        _372 = false;
        break;
    } while(false);
    if (_372)
    {
        discard;
    }
    int _376 = int(in_var_C6);
    bool _382;
    if (!(_376 == 0))
    {
        _382 = _376 == 2;
    }
    else
    {
        _382 = true;
    }
    float3 _2781;
    float _2782;
    float _2783;
    float _2784;
    float3 _2785;
    if (_382)
    {
        bool _1521;
        if (!(_376 == 2))
        {
            _1521 = _376 == 3;
        }
        else
        {
            _1521 = true;
        }
        int _1525;
        if (_1521)
        {
            _1525 = int(in_var_C7);
        }
        else
        {
            _1525 = int(wall_color_ids.Load<uint>(nani_instance_data.Load<uint>(in_var_C4 * 64 + 12) * 4 + 0));
        }
        float2 _1526 = in_var_C3 * 0.20000000298023223876953125f;
        float _2771;
        float _2772;
        float _2773;
        float3 _2774;
        float3 _2775;
        float _2776;
        if (in_var_C8 == 0u)
        {
            float3 _2449;
            uint2 _2365 = uint2(gl_FragCoord.yx);
            float _2372 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
            float _2382 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_2365.x) + _2372)) + (0.005837149918079376220703125f * (float(_2365.y) + _2372))));
            float _2385_tmp = stone_floor_height.CalculateLevelOfDetail(sampler_llr, _1526);
            float2 _2385 = _2385_tmp.xx;
            float _2389 = _2385.x + frame_constants_global_mip_bias;
            float3 _2390 = ddx(in_var_C2);
            float3 _2391 = ddy(in_var_C2);
            float2 _2393 = ddx(_1526);
            float2 _2394 = ddy(_1526);
            float2 _2395 = -_2394;
            float3 _2396 = cross(in_var_C1, -_2391);
            float3 _2397 = cross(_2390, in_var_C1);
            float _2398 = _2393.x;
            float _2399 = _2395.y;
            float _2401 = _2393.y;
            float _2402 = _2395.x;
            float _2405 = 1.0f / ((_2398 * _2399) - (_2401 * _2402));
            float3x3 _2415 = transpose(float3x3(((_2396 * _2398) + (_2397 * _2402)) * _2405, ((_2396 * _2401) + (_2397 * _2399)) * _2405, in_var_C1));
            float4 _2420 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
            float3 _2426 = mul(_2415, float3(1.0f, 0.0f, 0.0f));
            float3 _2427 = mul(_2415, float3(0.0f, 1.0f, 0.0f));
            float3 _2428 = mul(_2415, float3(0.0f, 0.0f, 1.0f));
            float3 _2431 = cross(_2426, _2427);
            float3 _2434 = (1.0f / dot(_2428, _2431)).xxx;
            float3x3 _2438 = float3x3(cross(_2427, _2428) * _2434, cross(_2428, _2426) * _2434, _2431 * _2434);
            float3 _2439 = mul(_2438, in_var_C2 - (_2420.xyz / _2420.w.xxx));
            float3 _2445 = float3(_1526, 0.0f);
            float _2558;
            do
            {
                _2449 = (_2445 + (_2439 * ((-0.07500000298023223876953125f) / _2439.z))) - _2445;
                float _2451 = 1.0f / max(1.0f, 2.0f);
                float _2454 = lerp(0.0f, 1.0f, pow(_2382 * _2451, 2.0f));
                float3 _2456 = _2445 + (_2449 * _2454);
                float _2464 = _2456.z - ((stone_floor_height.SampleLevel(sampler_llr, _2456.xy, _2389).x - 1.0f) * 0.07500000298023223876953125f);
                bool _2466 = (_2464 < 0.0f) ? true : false;
                float _2503;
                float _2504;
                float _2505;
                bool _2506;
                float _2507;
                if (_2466)
                {
                    _2503 = _2464;
                    _2504 = 0.0f;
                    _2505 = _2454;
                    _2506 = _2466;
                    _2507 = 0.0f;
                }
                else
                {
                    float _2474;
                    float _2478;
                    _2474 = _2464;
                    _2478 = _2454;
                    float _2475;
                    bool _2477;
                    float _2479;
                    float _2500;
                    float _2501;
                    bool _2502;
                    uint _2471 = 1u;
                    bool _2476 = _2466;
                    for (;;)
                    {
                        if (_2471 < 3u)
                        {
                            _2479 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_2471) + _2382) * _2451, 2.0f)));
                            float3 _2489 = _2445 + (_2449 * _2479);
                            _2475 = _2489.z - ((stone_floor_height.SampleLevel(sampler_llr, _2489.xy, _2389).x - 1.0f) * 0.07500000298023223876953125f);
                            _2477 = (_2475 < 0.0f) ? true : false;
                            if (_2477)
                            {
                                _2500 = _2475;
                                _2501 = _2479;
                                _2502 = _2477;
                                break;
                            }
                            _2471++;
                            _2474 = _2475;
                            _2476 = _2477;
                            _2478 = _2479;
                            continue;
                        }
                        else
                        {
                            _2500 = 0.0f;
                            _2501 = 1.0f;
                            _2502 = _2476;
                            break;
                        }
                    }
                    _2503 = _2500;
                    _2504 = _2474;
                    _2505 = _2501;
                    _2506 = _2502;
                    _2507 = _2478;
                }
                if (_2506)
                {
                    float _2512;
                    float _2515;
                    float _2517;
                    float _2519;
                    _2512 = _2507;
                    _2515 = _2503;
                    _2517 = _2504;
                    _2519 = _2505;
                    for (uint _2521 = 0u; _2521 < 4u; )
                    {
                        float _2525 = _2512 + _2519;
                        float _2526 = _2525 * 0.5f;
                        float3 _2528 = _2445 + (_2449 * _2526);
                        float _2536 = _2528.z - ((stone_floor_height.SampleLevel(sampler_llr, _2528.xy, _2389).x - 1.0f) * 0.07500000298023223876953125f);
                        bool _2538 = (_2536 < 0.0f) ? true : false;
                        _2512 = _2538 ? _2512 : _2526;
                        _2515 = _2538 ? _2536 : _2515;
                        _2517 = _2538 ? _2517 : _2536;
                        _2519 = _2538 ? _2526 : _2519;
                        _2521++;
                        continue;
                    }
                    float _2542 = lerp(_2512, _2519, _2517 / (_2517 + (-_2515)));
                    float3 _2544 = _2445 + (_2449 * _2542);
                    _2558 = ((abs(_2544.z - ((stone_floor_height.SampleLevel(sampler_llr, _2544.xy, _2389).x - 1.0f) * 0.07500000298023223876953125f)) < (_2517 * 0.89999997615814208984375f)) ? true : false) ? _2542 : _2519;
                    break;
                }
                else
                {
                    _2558 = _2507;
                    break;
                }
                break; 
            } while(false);
            float3 _2559 = _2449 * _2558;
            float2 _2561 = _1526 + _2559.xy;
            float4 _2573 = mul(frame_constants_view_constants[current_view_index].view_to_clip, mul(frame_constants_view_constants[current_view_index].world_to_view, float4(in_var_C2 + mul(_2415, _2559), 1.0f)));
            float3 _2582 = (stone_floor_normal.SampleLevel(sampler_llr, _2561, _2389).xyz * 2.0f) - 1.0f.xxx;
            _2582.z = _2582.z * 0.13333332538604736328125f;
            float4 _2590 = stone_floor_seed.SampleBias(sampler_llr, _2561, frame_constants_global_mip_bias);
            float _2591 = _2590.z;
            float _2592 = _2590.x;
            int _2601 = int(trunc((_2592 * 32.0f) - 0.5f) + trunc((_2590.y * 32.0f) - 0.5f));
            uint _2603 = uint(_2601 ^ 7993234);
            uint _2605 = _2603 + (_2603 << 10u);
            uint _2607 = _2605 ^ (_2605 >> 6u);
            uint _2609 = _2607 + (_2607 << 3u);
            uint _2611 = _2609 ^ (_2609 >> 11u);
            uint _2620 = uint(_2601 ^ 9730541);
            uint _2622 = _2620 + (_2620 << 10u);
            uint _2624 = _2622 ^ (_2622 >> 6u);
            uint _2626 = _2624 + (_2624 << 3u);
            uint _2628 = _2626 ^ (_2626 >> 11u);
            float _2634 = asfloat(((_2628 + (_2628 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
            float4 _2643 = stone_floor_height.SampleBias(sampler_llr, _2561, frame_constants_global_mip_bias);
            float4 _2646 = stone_floor_roughness.SampleBias(sampler_llr, _2561, frame_constants_global_mip_bias);
            float _2647 = _2646.x;
            float2 _2650 = (((_277 * 0.100000001490116119384765625f) + float2((asfloat(((_2611 + (_2611 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 0.300000011920928955078125f, _2634 * 0.300000011920928955078125f)) + (0.5f * _2591).xx) + (_2647 * 0.100000001490116119384765625f).xx;
            bool _2656;
            if (!(_1525 == 0))
            {
                _2656 = _1525 == 4;
            }
            else
            {
                _2656 = true;
            }
            bool _2661;
            if (!_2656)
            {
                _2661 = _1525 == 10;
            }
            else
            {
                _2661 = true;
            }
            bool _2666;
            if (!_2661)
            {
                _2666 = _1525 == 12;
            }
            else
            {
                _2666 = true;
            }
            float3 _2758;
            if (_2666)
            {
                _2758 = brick_colors.Sample(sampler_llr, float3(_2650, 0.0f)).xyz * 0.89999997615814208984375f;
            }
            else
            {
                bool _2675;
                if (!(_1525 == 1))
                {
                    _2675 = _1525 == 5;
                }
                else
                {
                    _2675 = true;
                }
                float3 _2749;
                if (_2675)
                {
                    _2749 = brick_colors.Sample(sampler_llr, float3(_2650, 1.0f)).xyz;
                }
                else
                {
                    bool _2684;
                    if (!(_1525 == 2))
                    {
                        _2684 = _1525 == 7;
                    }
                    else
                    {
                        _2684 = true;
                    }
                    float3 _2741;
                    if (_2684)
                    {
                        _2741 = (brick_colors.Sample(sampler_llr, float3(_2650, 2.0f)).xyz * 1.2999999523162841796875f) * lerp(1.0f, 1.2999999523162841796875f, _2591);
                    }
                    else
                    {
                        bool _2693;
                        if (!(_1525 == 3))
                        {
                            _2693 = _1525 == 6;
                        }
                        else
                        {
                            _2693 = true;
                        }
                        bool _2698;
                        if (!_2693)
                        {
                            _2698 = _1525 == 8;
                        }
                        else
                        {
                            _2698 = true;
                        }
                        bool _2703;
                        if (!_2698)
                        {
                            _2703 = _1525 == 9;
                        }
                        else
                        {
                            _2703 = true;
                        }
                        float3 _2730;
                        if (_2703)
                        {
                            _2730 = (brick_colors.Sample(sampler_llr, float3(_2650, 3.0f)).xyz * 0.89999997615814208984375f) * lerp(1.0f, 1.10000002384185791015625f, _2591);
                        }
                        else
                        {
                            float3 _2719;
                            if (_1525 == 11)
                            {
                                _2719 = brick_colors.Sample(sampler_llr, float3(_2650, 4.0f)).xyz * lerp(1.0f, 1.39999997615814208984375f, _2591);
                            }
                            else
                            {
                                _2719 = 0.0f.xxx;
                            }
                            _2730 = _2719;
                        }
                        _2741 = _2730;
                    }
                    _2749 = _2741;
                }
                _2758 = _2749;
            }
            _2771 = smoothstep(0.20000000298023223876953125f, 0.75f, 1.0f - _2643.x);
            _2772 = (_2573.xyz / _2573.w.xxx).z;
            _2773 = lerp(0.75f, 1.0f, _2647) * lerp(0.800000011920928955078125f, 1.0f, _2634);
            _2774 = normalize(mul(_2582, _2438));
            _2775 = (_2758 * 0.800000011920928955078125f) * lerp(0.699999988079071044921875f, 1.0f, stone_floor_height.SampleBias(sampler_llr, _2561, frame_constants_global_mip_bias).x);
            _2776 = lerp(0.0f, 0.96875f, _2592);
        }
        else
        {
            float _2354;
            float _2355;
            float _2356;
            float3 _2357;
            float3 _2358;
            float _2359;
            if (in_var_C8 == 1u)
            {
                float3 _2032;
                uint2 _1948 = uint2(gl_FragCoord.yx);
                float _1955 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
                float _1965 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_1948.x) + _1955)) + (0.005837149918079376220703125f * (float(_1948.y) + _1955))));
                float _1968_tmp = stone_floor_height_2.CalculateLevelOfDetail(sampler_llr, _1526);
                float2 _1968 = _1968_tmp.xx;
                float _1972 = _1968.x + frame_constants_global_mip_bias;
                float3 _1973 = ddx(in_var_C2);
                float3 _1974 = ddy(in_var_C2);
                float2 _1976 = ddx(_1526);
                float2 _1977 = ddy(_1526);
                float2 _1978 = -_1977;
                float3 _1979 = cross(in_var_C1, -_1974);
                float3 _1980 = cross(_1973, in_var_C1);
                float _1981 = _1976.x;
                float _1982 = _1978.y;
                float _1984 = _1976.y;
                float _1985 = _1978.x;
                float _1988 = 1.0f / ((_1981 * _1982) - (_1984 * _1985));
                float3x3 _1998 = transpose(float3x3(((_1979 * _1981) + (_1980 * _1985)) * _1988, ((_1979 * _1984) + (_1980 * _1982)) * _1988, in_var_C1));
                float4 _2003 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
                float3 _2009 = mul(_1998, float3(1.0f, 0.0f, 0.0f));
                float3 _2010 = mul(_1998, float3(0.0f, 1.0f, 0.0f));
                float3 _2011 = mul(_1998, float3(0.0f, 0.0f, 1.0f));
                float3 _2014 = cross(_2009, _2010);
                float3 _2017 = (1.0f / dot(_2011, _2014)).xxx;
                float3x3 _2021 = float3x3(cross(_2010, _2011) * _2017, cross(_2011, _2009) * _2017, _2014 * _2017);
                float3 _2022 = mul(_2021, in_var_C2 - (_2003.xyz / _2003.w.xxx));
                float3 _2028 = float3(_1526, 0.0f);
                float _2141;
                do
                {
                    _2032 = (_2028 + (_2022 * ((-0.0500000007450580596923828125f) / _2022.z))) - _2028;
                    float _2034 = 1.0f / max(1.0f, 2.0f);
                    float _2037 = lerp(0.0f, 1.0f, pow(_1965 * _2034, 2.0f));
                    float3 _2039 = _2028 + (_2032 * _2037);
                    float _2047 = _2039.z - ((stone_floor_height_2.SampleLevel(sampler_llr, _2039.xy, _1972).x - 1.0f) * 0.0500000007450580596923828125f);
                    bool _2049 = (_2047 < 0.0f) ? true : false;
                    float _2086;
                    float _2087;
                    float _2088;
                    bool _2089;
                    float _2090;
                    if (_2049)
                    {
                        _2086 = _2047;
                        _2087 = 0.0f;
                        _2088 = _2037;
                        _2089 = _2049;
                        _2090 = 0.0f;
                    }
                    else
                    {
                        float _2057;
                        float _2061;
                        _2057 = _2047;
                        _2061 = _2037;
                        float _2058;
                        bool _2060;
                        float _2062;
                        float _2083;
                        float _2084;
                        bool _2085;
                        uint _2054 = 1u;
                        bool _2059 = _2049;
                        for (;;)
                        {
                            if (_2054 < 3u)
                            {
                                _2062 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_2054) + _1965) * _2034, 2.0f)));
                                float3 _2072 = _2028 + (_2032 * _2062);
                                _2058 = _2072.z - ((stone_floor_height_2.SampleLevel(sampler_llr, _2072.xy, _1972).x - 1.0f) * 0.0500000007450580596923828125f);
                                _2060 = (_2058 < 0.0f) ? true : false;
                                if (_2060)
                                {
                                    _2083 = _2058;
                                    _2084 = _2062;
                                    _2085 = _2060;
                                    break;
                                }
                                _2054++;
                                _2057 = _2058;
                                _2059 = _2060;
                                _2061 = _2062;
                                continue;
                            }
                            else
                            {
                                _2083 = 0.0f;
                                _2084 = 1.0f;
                                _2085 = _2059;
                                break;
                            }
                        }
                        _2086 = _2083;
                        _2087 = _2057;
                        _2088 = _2084;
                        _2089 = _2085;
                        _2090 = _2061;
                    }
                    if (_2089)
                    {
                        float _2095;
                        float _2098;
                        float _2100;
                        float _2102;
                        _2095 = _2090;
                        _2098 = _2086;
                        _2100 = _2087;
                        _2102 = _2088;
                        for (uint _2104 = 0u; _2104 < 4u; )
                        {
                            float _2108 = _2095 + _2102;
                            float _2109 = _2108 * 0.5f;
                            float3 _2111 = _2028 + (_2032 * _2109);
                            float _2119 = _2111.z - ((stone_floor_height_2.SampleLevel(sampler_llr, _2111.xy, _1972).x - 1.0f) * 0.0500000007450580596923828125f);
                            bool _2121 = (_2119 < 0.0f) ? true : false;
                            _2095 = _2121 ? _2095 : _2109;
                            _2098 = _2121 ? _2119 : _2098;
                            _2100 = _2121 ? _2100 : _2119;
                            _2102 = _2121 ? _2109 : _2102;
                            _2104++;
                            continue;
                        }
                        float _2125 = lerp(_2095, _2102, _2100 / (_2100 + (-_2098)));
                        float3 _2127 = _2028 + (_2032 * _2125);
                        _2141 = ((abs(_2127.z - ((stone_floor_height_2.SampleLevel(sampler_llr, _2127.xy, _1972).x - 1.0f) * 0.0500000007450580596923828125f)) < (_2100 * 0.89999997615814208984375f)) ? true : false) ? _2125 : _2102;
                        break;
                    }
                    else
                    {
                        _2141 = _2090;
                        break;
                    }
                    break; 
                } while(false);
                float3 _2142 = _2032 * _2141;
                float2 _2144 = _1526 + _2142.xy;
                float4 _2156 = mul(frame_constants_view_constants[current_view_index].view_to_clip, mul(frame_constants_view_constants[current_view_index].world_to_view, float4(in_var_C2 + mul(_1998, _2142), 1.0f)));
                float3 _2165 = (stone_floor_normal_2.SampleLevel(sampler_llr, _2144, _1972).xyz * 2.0f) - 1.0f.xxx;
                _2165.z = _2165.z * 0.199999988079071044921875f;
                float4 _2173 = stone_floor_seed_2.SampleBias(sampler_llr, _2144, frame_constants_global_mip_bias);
                float _2174 = _2173.z;
                float _2175 = _2173.x;
                int _2184 = int(trunc((_2175 * 32.0f) - 0.5f) + trunc((_2173.y * 32.0f) - 0.5f));
                uint _2186 = uint(_2184 ^ 7993234);
                uint _2188 = _2186 + (_2186 << 10u);
                uint _2190 = _2188 ^ (_2188 >> 6u);
                uint _2192 = _2190 + (_2190 << 3u);
                uint _2194 = _2192 ^ (_2192 >> 11u);
                uint _2203 = uint(_2184 ^ 9730541);
                uint _2205 = _2203 + (_2203 << 10u);
                uint _2207 = _2205 ^ (_2205 >> 6u);
                uint _2209 = _2207 + (_2207 << 3u);
                uint _2211 = _2209 ^ (_2209 >> 11u);
                float _2217 = asfloat(((_2211 + (_2211 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
                float4 _2226 = stone_floor_height_2.SampleBias(sampler_llr, _2144, frame_constants_global_mip_bias);
                float4 _2229 = stone_floor_roughness_2.SampleBias(sampler_llr, _2144, frame_constants_global_mip_bias);
                float _2230 = _2229.x;
                float2 _2233 = (((_277 * 0.100000001490116119384765625f) + float2((asfloat(((_2194 + (_2194 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 0.300000011920928955078125f, _2217 * 0.300000011920928955078125f)) + (0.5f * _2174).xx) + (_2230 * 0.100000001490116119384765625f).xx;
                bool _2239;
                if (!(_1525 == 0))
                {
                    _2239 = _1525 == 4;
                }
                else
                {
                    _2239 = true;
                }
                bool _2244;
                if (!_2239)
                {
                    _2244 = _1525 == 10;
                }
                else
                {
                    _2244 = true;
                }
                bool _2249;
                if (!_2244)
                {
                    _2249 = _1525 == 12;
                }
                else
                {
                    _2249 = true;
                }
                float3 _2341;
                if (_2249)
                {
                    _2341 = brick_colors.Sample(sampler_llr, float3(_2233, 0.0f)).xyz * 0.89999997615814208984375f;
                }
                else
                {
                    bool _2258;
                    if (!(_1525 == 1))
                    {
                        _2258 = _1525 == 5;
                    }
                    else
                    {
                        _2258 = true;
                    }
                    float3 _2332;
                    if (_2258)
                    {
                        _2332 = brick_colors.Sample(sampler_llr, float3(_2233, 1.0f)).xyz;
                    }
                    else
                    {
                        bool _2267;
                        if (!(_1525 == 2))
                        {
                            _2267 = _1525 == 7;
                        }
                        else
                        {
                            _2267 = true;
                        }
                        float3 _2324;
                        if (_2267)
                        {
                            _2324 = (brick_colors.Sample(sampler_llr, float3(_2233, 2.0f)).xyz * 1.2999999523162841796875f) * lerp(1.0f, 1.2999999523162841796875f, _2174);
                        }
                        else
                        {
                            bool _2276;
                            if (!(_1525 == 3))
                            {
                                _2276 = _1525 == 6;
                            }
                            else
                            {
                                _2276 = true;
                            }
                            bool _2281;
                            if (!_2276)
                            {
                                _2281 = _1525 == 8;
                            }
                            else
                            {
                                _2281 = true;
                            }
                            bool _2286;
                            if (!_2281)
                            {
                                _2286 = _1525 == 9;
                            }
                            else
                            {
                                _2286 = true;
                            }
                            float3 _2313;
                            if (_2286)
                            {
                                _2313 = (brick_colors.Sample(sampler_llr, float3(_2233, 3.0f)).xyz * 0.89999997615814208984375f) * lerp(1.0f, 1.10000002384185791015625f, _2174);
                            }
                            else
                            {
                                float3 _2302;
                                if (_1525 == 11)
                                {
                                    _2302 = brick_colors.Sample(sampler_llr, float3(_2233, 4.0f)).xyz * lerp(1.0f, 1.39999997615814208984375f, _2174);
                                }
                                else
                                {
                                    _2302 = 0.0f.xxx;
                                }
                                _2313 = _2302;
                            }
                            _2324 = _2313;
                        }
                        _2332 = _2324;
                    }
                    _2341 = _2332;
                }
                _2354 = smoothstep(0.300000011920928955078125f, 0.5f, 1.0f - _2226.x);
                _2355 = (_2156.xyz / _2156.w.xxx).z;
                _2356 = lerp(0.75f, 1.0f, _2230) * lerp(0.800000011920928955078125f, 1.0f, _2217);
                _2357 = normalize(mul(_2165, _2021));
                _2358 = (_2341 * 0.800000011920928955078125f) * lerp(0.699999988079071044921875f, 1.0f, stone_floor_height_2.SampleBias(sampler_llr, _2144, frame_constants_global_mip_bias).x);
                _2359 = lerp(0.0f, 0.96875f, _2175);
            }
            else
            {
                float3 _1624;
                uint2 _1540 = uint2(gl_FragCoord.yx);
                float _1547 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
                float _1557 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_1540.x) + _1547)) + (0.005837149918079376220703125f * (float(_1540.y) + _1547))));
                float _1560_tmp = stone_floor_height_3.CalculateLevelOfDetail(sampler_llr, _1526);
                float2 _1560 = _1560_tmp.xx;
                float _1564 = _1560.x + frame_constants_global_mip_bias;
                float3 _1565 = ddx(in_var_C2);
                float3 _1566 = ddy(in_var_C2);
                float2 _1568 = ddx(_1526);
                float2 _1569 = ddy(_1526);
                float2 _1570 = -_1569;
                float3 _1571 = cross(in_var_C1, -_1566);
                float3 _1572 = cross(_1565, in_var_C1);
                float _1573 = _1568.x;
                float _1574 = _1570.y;
                float _1576 = _1568.y;
                float _1577 = _1570.x;
                float _1580 = 1.0f / ((_1573 * _1574) - (_1576 * _1577));
                float3x3 _1590 = transpose(float3x3(((_1571 * _1573) + (_1572 * _1577)) * _1580, ((_1571 * _1576) + (_1572 * _1574)) * _1580, in_var_C1));
                float4 _1595 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
                float3 _1601 = mul(_1590, float3(1.0f, 0.0f, 0.0f));
                float3 _1602 = mul(_1590, float3(0.0f, 1.0f, 0.0f));
                float3 _1603 = mul(_1590, float3(0.0f, 0.0f, 1.0f));
                float3 _1606 = cross(_1601, _1602);
                float3 _1609 = (1.0f / dot(_1603, _1606)).xxx;
                float3x3 _1613 = float3x3(cross(_1602, _1603) * _1609, cross(_1603, _1601) * _1609, _1606 * _1609);
                float3 _1614 = mul(_1613, in_var_C2 - (_1595.xyz / _1595.w.xxx));
                float3 _1620 = float3(_1526, 0.0f);
                float _1733;
                do
                {
                    _1624 = (_1620 + (_1614 * ((-0.0500000007450580596923828125f) / _1614.z))) - _1620;
                    float _1626 = 1.0f / max(1.0f, 2.0f);
                    float _1629 = lerp(0.0f, 1.0f, pow(_1557 * _1626, 2.0f));
                    float3 _1631 = _1620 + (_1624 * _1629);
                    float _1639 = _1631.z - ((stone_floor_height_3.SampleLevel(sampler_llr, _1631.xy, _1564).x - 1.0f) * 0.0500000007450580596923828125f);
                    bool _1641 = (_1639 < 0.0f) ? true : false;
                    float _1678;
                    float _1679;
                    float _1680;
                    bool _1681;
                    float _1682;
                    if (_1641)
                    {
                        _1678 = _1639;
                        _1679 = 0.0f;
                        _1680 = _1629;
                        _1681 = _1641;
                        _1682 = 0.0f;
                    }
                    else
                    {
                        float _1649;
                        float _1653;
                        _1649 = _1639;
                        _1653 = _1629;
                        float _1650;
                        bool _1652;
                        float _1654;
                        float _1675;
                        float _1676;
                        bool _1677;
                        uint _1646 = 1u;
                        bool _1651 = _1641;
                        for (;;)
                        {
                            if (_1646 < 3u)
                            {
                                _1654 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_1646) + _1557) * _1626, 2.0f)));
                                float3 _1664 = _1620 + (_1624 * _1654);
                                _1650 = _1664.z - ((stone_floor_height_3.SampleLevel(sampler_llr, _1664.xy, _1564).x - 1.0f) * 0.0500000007450580596923828125f);
                                _1652 = (_1650 < 0.0f) ? true : false;
                                if (_1652)
                                {
                                    _1675 = _1650;
                                    _1676 = _1654;
                                    _1677 = _1652;
                                    break;
                                }
                                _1646++;
                                _1649 = _1650;
                                _1651 = _1652;
                                _1653 = _1654;
                                continue;
                            }
                            else
                            {
                                _1675 = 0.0f;
                                _1676 = 1.0f;
                                _1677 = _1651;
                                break;
                            }
                        }
                        _1678 = _1675;
                        _1679 = _1649;
                        _1680 = _1676;
                        _1681 = _1677;
                        _1682 = _1653;
                    }
                    if (_1681)
                    {
                        float _1687;
                        float _1690;
                        float _1692;
                        float _1694;
                        _1687 = _1682;
                        _1690 = _1678;
                        _1692 = _1679;
                        _1694 = _1680;
                        for (uint _1696 = 0u; _1696 < 4u; )
                        {
                            float _1700 = _1687 + _1694;
                            float _1701 = _1700 * 0.5f;
                            float3 _1703 = _1620 + (_1624 * _1701);
                            float _1711 = _1703.z - ((stone_floor_height_3.SampleLevel(sampler_llr, _1703.xy, _1564).x - 1.0f) * 0.0500000007450580596923828125f);
                            bool _1713 = (_1711 < 0.0f) ? true : false;
                            _1687 = _1713 ? _1687 : _1701;
                            _1690 = _1713 ? _1711 : _1690;
                            _1692 = _1713 ? _1692 : _1711;
                            _1694 = _1713 ? _1701 : _1694;
                            _1696++;
                            continue;
                        }
                        float _1717 = lerp(_1687, _1694, _1692 / (_1692 + (-_1690)));
                        float3 _1719 = _1620 + (_1624 * _1717);
                        _1733 = ((abs(_1719.z - ((stone_floor_height_3.SampleLevel(sampler_llr, _1719.xy, _1564).x - 1.0f) * 0.0500000007450580596923828125f)) < (_1692 * 0.89999997615814208984375f)) ? true : false) ? _1717 : _1694;
                        break;
                    }
                    else
                    {
                        _1733 = _1682;
                        break;
                    }
                    break; 
                } while(false);
                float3 _1734 = _1624 * _1733;
                float2 _1736 = _1526 + _1734.xy;
                float4 _1748 = mul(frame_constants_view_constants[current_view_index].view_to_clip, mul(frame_constants_view_constants[current_view_index].world_to_view, float4(in_var_C2 + mul(_1590, _1734), 1.0f)));
                float3 _1757 = (stone_floor_normal_3.SampleLevel(sampler_llr, _1736, _1564).xyz * 2.0f) - 1.0f.xxx;
                _1757.z = _1757.z * 0.199999988079071044921875f;
                float4 _1765 = stone_floor_seed_3.SampleBias(sampler_llr, _1736, frame_constants_global_mip_bias);
                float _1766 = _1765.z;
                float _1767 = _1765.x;
                int _1776 = int(trunc((_1767 * 32.0f) - 0.5f) + trunc((_1765.y * 32.0f) - 0.5f));
                uint _1778 = uint(_1776 ^ 7993234);
                uint _1780 = _1778 + (_1778 << 10u);
                uint _1782 = _1780 ^ (_1780 >> 6u);
                uint _1784 = _1782 + (_1782 << 3u);
                uint _1786 = _1784 ^ (_1784 >> 11u);
                uint _1795 = uint(_1776 ^ 9730541);
                uint _1797 = _1795 + (_1795 << 10u);
                uint _1799 = _1797 ^ (_1797 >> 6u);
                uint _1801 = _1799 + (_1799 << 3u);
                uint _1803 = _1801 ^ (_1801 >> 11u);
                float _1809 = asfloat(((_1803 + (_1803 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
                float2 _1816 = ((_277 * 0.100000001490116119384765625f) + float2((asfloat(((_1786 + (_1786 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 0.300000011920928955078125f, _1809 * 0.300000011920928955078125f)) + (0.5f * _1766).xx;
                float4 _1818 = stone_floor_height_3.SampleBias(sampler_llr, _1736, frame_constants_global_mip_bias);
                float4 _1821 = stone_floor_roughness_3.SampleBias(sampler_llr, _1736, frame_constants_global_mip_bias);
                bool _1828;
                if (!(_1525 == 0))
                {
                    _1828 = _1525 == 4;
                }
                else
                {
                    _1828 = true;
                }
                bool _1833;
                if (!_1828)
                {
                    _1833 = _1525 == 10;
                }
                else
                {
                    _1833 = true;
                }
                bool _1838;
                if (!_1833)
                {
                    _1838 = _1525 == 12;
                }
                else
                {
                    _1838 = true;
                }
                float3 _1930;
                if (_1838)
                {
                    _1930 = brick_colors.Sample(sampler_llr, float3(_1816, 0.0f)).xyz * 0.89999997615814208984375f;
                }
                else
                {
                    bool _1847;
                    if (!(_1525 == 1))
                    {
                        _1847 = _1525 == 5;
                    }
                    else
                    {
                        _1847 = true;
                    }
                    float3 _1921;
                    if (_1847)
                    {
                        _1921 = brick_colors.Sample(sampler_llr, float3(_1816, 1.0f)).xyz;
                    }
                    else
                    {
                        bool _1856;
                        if (!(_1525 == 2))
                        {
                            _1856 = _1525 == 7;
                        }
                        else
                        {
                            _1856 = true;
                        }
                        float3 _1913;
                        if (_1856)
                        {
                            _1913 = (brick_colors.Sample(sampler_llr, float3(_1816, 2.0f)).xyz * 1.2999999523162841796875f) * lerp(1.0f, 1.2999999523162841796875f, _1766);
                        }
                        else
                        {
                            bool _1865;
                            if (!(_1525 == 3))
                            {
                                _1865 = _1525 == 6;
                            }
                            else
                            {
                                _1865 = true;
                            }
                            bool _1870;
                            if (!_1865)
                            {
                                _1870 = _1525 == 8;
                            }
                            else
                            {
                                _1870 = true;
                            }
                            bool _1875;
                            if (!_1870)
                            {
                                _1875 = _1525 == 9;
                            }
                            else
                            {
                                _1875 = true;
                            }
                            float3 _1902;
                            if (_1875)
                            {
                                _1902 = (brick_colors.Sample(sampler_llr, float3(_1816, 3.0f)).xyz * 0.89999997615814208984375f) * lerp(1.0f, 1.10000002384185791015625f, _1766);
                            }
                            else
                            {
                                float3 _1891;
                                if (_1525 == 11)
                                {
                                    _1891 = brick_colors.Sample(sampler_llr, float3(_1816, 4.0f)).xyz * lerp(1.0f, 1.39999997615814208984375f, _1766);
                                }
                                else
                                {
                                    _1891 = 0.0f.xxx;
                                }
                                _1902 = _1891;
                            }
                            _1913 = _1902;
                        }
                        _1921 = _1913;
                    }
                    _1930 = _1921;
                }
                _2354 = smoothstep(0.300000011920928955078125f, 1.0f, 1.0f - _1818.x);
                _2355 = (_1748.xyz / _1748.w.xxx).z;
                _2356 = lerp(0.75f, 1.0f, _1821.x) * lerp(0.800000011920928955078125f, 1.0f, _1809);
                _2357 = normalize(mul(_1757, _1613));
                _2358 = (_1930 * 0.800000011920928955078125f) * lerp(0.699999988079071044921875f, 1.0f, stone_floor_height_3.SampleBias(sampler_llr, _1736, frame_constants_global_mip_bias).x);
                _2359 = lerp(0.0f, 0.96875f, _1767);
            }
            _2771 = _2354;
            _2772 = _2355;
            _2773 = _2356;
            _2774 = _2357;
            _2775 = _2358;
            _2776 = _2359;
        }
        if ((_2776 * _2776) >= nani_instance_data.Load<float>(in_var_C4 * 64 + 32))
        {
            discard;
        }
        _2781 = _2774;
        _2782 = _2773;
        _2783 = _2772;
        _2784 = _2771;
        _2785 = _2775;
    }
    else
    {
        bool _391;
        if (!(_376 == 1))
        {
            _391 = _376 == 3;
        }
        else
        {
            _391 = true;
        }
        float3 _1508;
        float _1509;
        float _1510;
        float _1511;
        float3 _1512;
        if (_391)
        {
            bool _402;
            if (!(_376 == 2))
            {
                _402 = _376 == 3;
            }
            else
            {
                _402 = true;
            }
            int _406;
            if (_402)
            {
                _406 = int(in_var_C7);
            }
            else
            {
                _406 = int(halftimber_color_ids.Load<uint>(nani_instance_data.Load<uint>(in_var_C4 * 64 + 12) * 4 + 0));
            }
            float2 _407 = in_var_C3 * 0.20000000298023223876953125f;
            float _1499;
            float _1500;
            float _1501;
            float3 _1502;
            float3 _1503;
            float _1504;
            if (in_var_C8 == 0u)
            {
                float3 _1229;
                uint2 _1145 = uint2(gl_FragCoord.yx);
                float _1152 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
                float _1162 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_1145.x) + _1152)) + (0.005837149918079376220703125f * (float(_1145.y) + _1152))));
                float _1165_tmp = wooden_floor_height.CalculateLevelOfDetail(sampler_llr, _407);
                float2 _1165 = _1165_tmp.xx;
                float _1169 = _1165.x + frame_constants_global_mip_bias;
                float3 _1170 = ddx(in_var_C2);
                float3 _1171 = ddy(in_var_C2);
                float2 _1173 = ddx(_407);
                float2 _1174 = ddy(_407);
                float2 _1175 = -_1174;
                float3 _1176 = cross(in_var_C1, -_1171);
                float3 _1177 = cross(_1170, in_var_C1);
                float _1178 = _1173.x;
                float _1179 = _1175.y;
                float _1181 = _1173.y;
                float _1182 = _1175.x;
                float _1185 = 1.0f / ((_1178 * _1179) - (_1181 * _1182));
                float3x3 _1195 = transpose(float3x3(((_1176 * _1178) + (_1177 * _1182)) * _1185, ((_1176 * _1181) + (_1177 * _1179)) * _1185, in_var_C1));
                float4 _1200 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
                float3 _1206 = mul(_1195, float3(1.0f, 0.0f, 0.0f));
                float3 _1207 = mul(_1195, float3(0.0f, 1.0f, 0.0f));
                float3 _1208 = mul(_1195, float3(0.0f, 0.0f, 1.0f));
                float3 _1211 = cross(_1206, _1207);
                float3 _1214 = (1.0f / dot(_1208, _1211)).xxx;
                float3x3 _1218 = float3x3(cross(_1207, _1208) * _1214, cross(_1208, _1206) * _1214, _1211 * _1214);
                float3 _1219 = mul(_1218, in_var_C2 - (_1200.xyz / _1200.w.xxx));
                float3 _1225 = float3(_407, 0.0f);
                float _1338;
                do
                {
                    _1229 = (_1225 + (_1219 * ((-0.07500000298023223876953125f) / _1219.z))) - _1225;
                    float _1231 = 1.0f / max(1.0f, 2.0f);
                    float _1234 = lerp(0.0f, 1.0f, pow(_1162 * _1231, 2.0f));
                    float3 _1236 = _1225 + (_1229 * _1234);
                    float _1244 = _1236.z - ((wooden_floor_height.SampleLevel(sampler_llr, _1236.xy, _1169).x - 1.0f) * 0.07500000298023223876953125f);
                    bool _1246 = (_1244 < 0.0f) ? true : false;
                    float _1283;
                    float _1284;
                    float _1285;
                    bool _1286;
                    float _1287;
                    if (_1246)
                    {
                        _1283 = _1244;
                        _1284 = 0.0f;
                        _1285 = _1234;
                        _1286 = _1246;
                        _1287 = 0.0f;
                    }
                    else
                    {
                        float _1254;
                        float _1258;
                        _1254 = _1244;
                        _1258 = _1234;
                        float _1255;
                        bool _1257;
                        float _1259;
                        float _1280;
                        float _1281;
                        bool _1282;
                        uint _1251 = 1u;
                        bool _1256 = _1246;
                        for (;;)
                        {
                            if (_1251 < 3u)
                            {
                                _1259 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_1251) + _1162) * _1231, 2.0f)));
                                float3 _1269 = _1225 + (_1229 * _1259);
                                _1255 = _1269.z - ((wooden_floor_height.SampleLevel(sampler_llr, _1269.xy, _1169).x - 1.0f) * 0.07500000298023223876953125f);
                                _1257 = (_1255 < 0.0f) ? true : false;
                                if (_1257)
                                {
                                    _1280 = _1255;
                                    _1281 = _1259;
                                    _1282 = _1257;
                                    break;
                                }
                                _1251++;
                                _1254 = _1255;
                                _1256 = _1257;
                                _1258 = _1259;
                                continue;
                            }
                            else
                            {
                                _1280 = 0.0f;
                                _1281 = 1.0f;
                                _1282 = _1256;
                                break;
                            }
                        }
                        _1283 = _1280;
                        _1284 = _1254;
                        _1285 = _1281;
                        _1286 = _1282;
                        _1287 = _1258;
                    }
                    if (_1286)
                    {
                        float _1292;
                        float _1295;
                        float _1297;
                        float _1299;
                        _1292 = _1287;
                        _1295 = _1283;
                        _1297 = _1284;
                        _1299 = _1285;
                        for (uint _1301 = 0u; _1301 < 4u; )
                        {
                            float _1305 = _1292 + _1299;
                            float _1306 = _1305 * 0.5f;
                            float3 _1308 = _1225 + (_1229 * _1306);
                            float _1316 = _1308.z - ((wooden_floor_height.SampleLevel(sampler_llr, _1308.xy, _1169).x - 1.0f) * 0.07500000298023223876953125f);
                            bool _1318 = (_1316 < 0.0f) ? true : false;
                            _1292 = _1318 ? _1292 : _1306;
                            _1295 = _1318 ? _1316 : _1295;
                            _1297 = _1318 ? _1297 : _1316;
                            _1299 = _1318 ? _1306 : _1299;
                            _1301++;
                            continue;
                        }
                        float _1322 = lerp(_1292, _1299, _1297 / (_1297 + (-_1295)));
                        float3 _1324 = _1225 + (_1229 * _1322);
                        _1338 = ((abs(_1324.z - ((wooden_floor_height.SampleLevel(sampler_llr, _1324.xy, _1169).x - 1.0f) * 0.07500000298023223876953125f)) < (_1297 * 0.89999997615814208984375f)) ? true : false) ? _1322 : _1299;
                        break;
                    }
                    else
                    {
                        _1338 = _1287;
                        break;
                    }
                    break; 
                } while(false);
                float3 _1339 = _1229 * _1338;
                float2 _1341 = _407 + _1339.xy;
                float4 _1353 = mul(frame_constants_view_constants[current_view_index].view_to_clip, mul(frame_constants_view_constants[current_view_index].world_to_view, float4(in_var_C2 + mul(_1195, _1339), 1.0f)));
                float3 _1362 = (wooden_floor_normal.SampleLevel(sampler_llr, _1341, _1169).xyz * 2.0f) - 1.0f.xxx;
                _1362.z = _1362.z * 0.199999988079071044921875f;
                float4 _1373 = wooden_floor_seed.SampleBias(sampler_llr, _1341, frame_constants_global_mip_bias);
                float _1374 = _1373.z;
                float _1376 = _1373.x;
                int _1380 = int(trunc((_1376 * 32.0f) - 0.5f));
                float2 _1387 = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(uint(_1380))) + 0.5f.xx) + (float2(_1341.yx) * 0.87999999523162841796875f);
                float _1388 = _1373.y;
                float _1397;
                if (0.003130800090730190277099609375f >= _1388)
                {
                    _1397 = 12.9200000762939453125f * _1388;
                }
                else
                {
                    _1397 = (1.05499994754791259765625f * pow(_1388, 0.4166666567325592041015625f)) - 0.054999999701976776123046875f;
                }
                float _1400 = round(_1397 * 8.0f) * 0.785398185253143310546875f;
                float _1401 = _1387.x;
                float _1402 = cos(_1400);
                float _1404 = _1387.y;
                float _1405 = sin(_1400);
                float2 _1411 = float2((_1401 * _1402) - (_1404 * _1405), (_1401 * _1405) + (_1404 * _1402));
                float2 _1415;
                if (_1374 > 0.5f)
                {
                    _1415 = _1411 + 0.20000000298023223876953125f.xx;
                }
                else
                {
                    _1415 = _1411;
                }
                float4 _1422 = halftimber_colors.SampleBias(sampler_llr, float3(_1415, float(_406)), frame_constants_global_mip_bias);
                float3 _1423 = _1422.xyz;
                bool _1429;
                if (!(_406 == 2))
                {
                    _1429 = _406 == 8;
                }
                else
                {
                    _1429 = true;
                }
                bool _1434;
                if (!_1429)
                {
                    _1434 = _406 == 9;
                }
                else
                {
                    _1434 = true;
                }
                float3 _1440;
                if (_1434)
                {
                    _1440 = lerp(_1423, dot(_1423, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)).xxx, 0.300000011920928955078125f.xxx);
                }
                else
                {
                    _1440 = _1423;
                }
                float4 _1443 = halftimber_colors.SampleBias(sampler_llr, float3(_1415, 0.0f), frame_constants_global_mip_bias);
                float3 _1444 = _1443.xyz;
                float _1466;
                float3 _1467;
                if (_406 == 0)
                {
                    _1466 = _1374;
                    _1467 = _1444 * 1.2000000476837158203125f;
                }
                else
                {
                    bool _1455;
                    if (!(_406 == 9))
                    {
                        _1455 = _406 == 3;
                    }
                    else
                    {
                        _1455 = true;
                    }
                    bool _1460;
                    if (!_1455)
                    {
                        _1460 = _406 == 4;
                    }
                    else
                    {
                        _1460 = true;
                    }
                    float _1464;
                    if (_1460)
                    {
                        _1464 = _1374 * 0.5f;
                    }
                    else
                    {
                        _1464 = _1374 * 0.20000000298023223876953125f;
                    }
                    _1466 = _1464;
                    _1467 = _1444;
                }
                uint _1480 = uint(_1380 ^ 9730541);
                uint _1482 = _1480 + (_1480 << 10u);
                uint _1484 = _1482 ^ (_1482 >> 6u);
                uint _1486 = _1484 + (_1484 << 3u);
                uint _1488 = _1486 ^ (_1486 >> 11u);
                float _1496 = lerp(0.75f, 1.0f, wooden_floor_roughness.SampleBias(sampler_llr, _1341, frame_constants_global_mip_bias).x) * lerp(0.800000011920928955078125f, 1.0f, asfloat(((_1488 + (_1488 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
                _1499 = smoothstep(0.5f, 1.0f, _1496);
                _1500 = (_1353.xyz / _1353.w.xxx).z;
                _1501 = _1496;
                _1502 = normalize(mul(_1362, _1218));
                _1503 = lerp(_1440 * lerp(0.699999988079071044921875f, 1.0f, wooden_floor_height.SampleBias(sampler_llr, _1341, frame_constants_global_mip_bias).x), _1467, _1466.xxx);
                _1504 = lerp(0.0f, 0.96875f, _1376);
            }
            else
            {
                float _1134;
                float _1135;
                float _1136;
                float3 _1137;
                float3 _1138;
                float _1139;
                if (in_var_C8 == 1u)
                {
                    float3 _864;
                    uint2 _780 = uint2(gl_FragCoord.yx);
                    float _787 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
                    float _797 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_780.x) + _787)) + (0.005837149918079376220703125f * (float(_780.y) + _787))));
                    float _800_tmp = wooden_floor_height_2.CalculateLevelOfDetail(sampler_llr, _407);
                    float2 _800 = _800_tmp.xx;
                    float _804 = _800.x + frame_constants_global_mip_bias;
                    float3 _805 = ddx(in_var_C2);
                    float3 _806 = ddy(in_var_C2);
                    float2 _808 = ddx(_407);
                    float2 _809 = ddy(_407);
                    float2 _810 = -_809;
                    float3 _811 = cross(in_var_C1, -_806);
                    float3 _812 = cross(_805, in_var_C1);
                    float _813 = _808.x;
                    float _814 = _810.y;
                    float _816 = _808.y;
                    float _817 = _810.x;
                    float _820 = 1.0f / ((_813 * _814) - (_816 * _817));
                    float3x3 _830 = transpose(float3x3(((_811 * _813) + (_812 * _817)) * _820, ((_811 * _816) + (_812 * _814)) * _820, in_var_C1));
                    float4 _835 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
                    float3 _841 = mul(_830, float3(1.0f, 0.0f, 0.0f));
                    float3 _842 = mul(_830, float3(0.0f, 1.0f, 0.0f));
                    float3 _843 = mul(_830, float3(0.0f, 0.0f, 1.0f));
                    float3 _846 = cross(_841, _842);
                    float3 _849 = (1.0f / dot(_843, _846)).xxx;
                    float3x3 _853 = float3x3(cross(_842, _843) * _849, cross(_843, _841) * _849, _846 * _849);
                    float3 _854 = mul(_853, in_var_C2 - (_835.xyz / _835.w.xxx));
                    float3 _860 = float3(_407, 0.0f);
                    float _973;
                    do
                    {
                        _864 = (_860 + (_854 * ((-0.07500000298023223876953125f) / _854.z))) - _860;
                        float _866 = 1.0f / max(1.0f, 2.0f);
                        float _869 = lerp(0.0f, 1.0f, pow(_797 * _866, 2.0f));
                        float3 _871 = _860 + (_864 * _869);
                        float _879 = _871.z - ((wooden_floor_height_2.SampleLevel(sampler_llr, _871.xy, _804).x - 1.0f) * 0.07500000298023223876953125f);
                        bool _881 = (_879 < 0.0f) ? true : false;
                        float _918;
                        float _919;
                        float _920;
                        bool _921;
                        float _922;
                        if (_881)
                        {
                            _918 = _879;
                            _919 = 0.0f;
                            _920 = _869;
                            _921 = _881;
                            _922 = 0.0f;
                        }
                        else
                        {
                            float _889;
                            float _893;
                            _889 = _879;
                            _893 = _869;
                            float _890;
                            bool _892;
                            float _894;
                            float _915;
                            float _916;
                            bool _917;
                            uint _886 = 1u;
                            bool _891 = _881;
                            for (;;)
                            {
                                if (_886 < 3u)
                                {
                                    _894 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_886) + _797) * _866, 2.0f)));
                                    float3 _904 = _860 + (_864 * _894);
                                    _890 = _904.z - ((wooden_floor_height_2.SampleLevel(sampler_llr, _904.xy, _804).x - 1.0f) * 0.07500000298023223876953125f);
                                    _892 = (_890 < 0.0f) ? true : false;
                                    if (_892)
                                    {
                                        _915 = _890;
                                        _916 = _894;
                                        _917 = _892;
                                        break;
                                    }
                                    _886++;
                                    _889 = _890;
                                    _891 = _892;
                                    _893 = _894;
                                    continue;
                                }
                                else
                                {
                                    _915 = 0.0f;
                                    _916 = 1.0f;
                                    _917 = _891;
                                    break;
                                }
                            }
                            _918 = _915;
                            _919 = _889;
                            _920 = _916;
                            _921 = _917;
                            _922 = _893;
                        }
                        if (_921)
                        {
                            float _927;
                            float _930;
                            float _932;
                            float _934;
                            _927 = _922;
                            _930 = _918;
                            _932 = _919;
                            _934 = _920;
                            for (uint _936 = 0u; _936 < 4u; )
                            {
                                float _940 = _927 + _934;
                                float _941 = _940 * 0.5f;
                                float3 _943 = _860 + (_864 * _941);
                                float _951 = _943.z - ((wooden_floor_height_2.SampleLevel(sampler_llr, _943.xy, _804).x - 1.0f) * 0.07500000298023223876953125f);
                                bool _953 = (_951 < 0.0f) ? true : false;
                                _927 = _953 ? _927 : _941;
                                _930 = _953 ? _951 : _930;
                                _932 = _953 ? _932 : _951;
                                _934 = _953 ? _941 : _934;
                                _936++;
                                continue;
                            }
                            float _957 = lerp(_927, _934, _932 / (_932 + (-_930)));
                            float3 _959 = _860 + (_864 * _957);
                            _973 = ((abs(_959.z - ((wooden_floor_height_2.SampleLevel(sampler_llr, _959.xy, _804).x - 1.0f) * 0.07500000298023223876953125f)) < (_932 * 0.89999997615814208984375f)) ? true : false) ? _957 : _934;
                            break;
                        }
                        else
                        {
                            _973 = _922;
                            break;
                        }
                        break; 
                    } while(false);
                    float3 _974 = _864 * _973;
                    float2 _976 = _407 + _974.xy;
                    float4 _988 = mul(frame_constants_view_constants[current_view_index].view_to_clip, mul(frame_constants_view_constants[current_view_index].world_to_view, float4(in_var_C2 + mul(_830, _974), 1.0f)));
                    float3 _997 = (wooden_floor_normal_2.SampleLevel(sampler_llr, _976, _804).xyz * 2.0f) - 1.0f.xxx;
                    _997.z = _997.z * 0.199999988079071044921875f;
                    float4 _1008 = wooden_floor_seed_2.SampleBias(sampler_llr, _976, frame_constants_global_mip_bias);
                    float _1009 = _1008.z;
                    float _1011 = _1008.x;
                    int _1015 = int(trunc((_1011 * 32.0f) - 0.5f));
                    float2 _1022 = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(uint(_1015))) + 0.5f.xx) + (float2(_976.yx) * 0.87999999523162841796875f);
                    float _1023 = _1008.y;
                    float _1032;
                    if (0.003130800090730190277099609375f >= _1023)
                    {
                        _1032 = 12.9200000762939453125f * _1023;
                    }
                    else
                    {
                        _1032 = (1.05499994754791259765625f * pow(_1023, 0.4166666567325592041015625f)) - 0.054999999701976776123046875f;
                    }
                    float _1035 = round(_1032 * 8.0f) * 0.785398185253143310546875f;
                    float _1036 = _1022.x;
                    float _1037 = cos(_1035);
                    float _1039 = _1022.y;
                    float _1040 = sin(_1035);
                    float2 _1046 = float2((_1036 * _1037) - (_1039 * _1040), (_1036 * _1040) + (_1039 * _1037));
                    float2 _1050;
                    if (_1009 > 0.5f)
                    {
                        _1050 = _1046 + 0.20000000298023223876953125f.xx;
                    }
                    else
                    {
                        _1050 = _1046;
                    }
                    float4 _1057 = halftimber_colors.SampleBias(sampler_llr, float3(_1050, float(_406)), frame_constants_global_mip_bias);
                    float3 _1058 = _1057.xyz;
                    bool _1064;
                    if (!(_406 == 2))
                    {
                        _1064 = _406 == 8;
                    }
                    else
                    {
                        _1064 = true;
                    }
                    bool _1069;
                    if (!_1064)
                    {
                        _1069 = _406 == 9;
                    }
                    else
                    {
                        _1069 = true;
                    }
                    float3 _1075;
                    if (_1069)
                    {
                        _1075 = lerp(_1058, dot(_1058, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)).xxx, 0.300000011920928955078125f.xxx);
                    }
                    else
                    {
                        _1075 = _1058;
                    }
                    float4 _1078 = halftimber_colors.SampleBias(sampler_llr, float3(_1050, 0.0f), frame_constants_global_mip_bias);
                    float3 _1079 = _1078.xyz;
                    float _1101;
                    float3 _1102;
                    if (_406 == 0)
                    {
                        _1101 = _1009;
                        _1102 = _1079 * 1.2000000476837158203125f;
                    }
                    else
                    {
                        bool _1090;
                        if (!(_406 == 9))
                        {
                            _1090 = _406 == 3;
                        }
                        else
                        {
                            _1090 = true;
                        }
                        bool _1095;
                        if (!_1090)
                        {
                            _1095 = _406 == 4;
                        }
                        else
                        {
                            _1095 = true;
                        }
                        float _1099;
                        if (_1095)
                        {
                            _1099 = _1009 * 0.5f;
                        }
                        else
                        {
                            _1099 = _1009 * 0.20000000298023223876953125f;
                        }
                        _1101 = _1099;
                        _1102 = _1079;
                    }
                    uint _1115 = uint(_1015 ^ 9730541);
                    uint _1117 = _1115 + (_1115 << 10u);
                    uint _1119 = _1117 ^ (_1117 >> 6u);
                    uint _1121 = _1119 + (_1119 << 3u);
                    uint _1123 = _1121 ^ (_1121 >> 11u);
                    float _1131 = lerp(0.75f, 1.0f, wooden_floor_roughness_2.SampleBias(sampler_llr, _976, frame_constants_global_mip_bias).x) * lerp(0.800000011920928955078125f, 1.0f, asfloat(((_1123 + (_1123 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
                    _1134 = smoothstep(0.550000011920928955078125f, 1.0f, _1131);
                    _1135 = (_988.xyz / _988.w.xxx).z;
                    _1136 = _1131;
                    _1137 = normalize(mul(_997, _853));
                    _1138 = lerp(_1075 * lerp(0.699999988079071044921875f, 1.0f, wooden_floor_height_2.SampleBias(sampler_llr, _976, frame_constants_global_mip_bias).x), _1102, _1101.xxx);
                    _1139 = lerp(0.0f, 0.96875f, _1011);
                }
                else
                {
                    float3 _505;
                    uint2 _421 = uint2(gl_FragCoord.yx);
                    float _428 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
                    float _438 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_421.x) + _428)) + (0.005837149918079376220703125f * (float(_421.y) + _428))));
                    float _441_tmp = wooden_floor_height_3.CalculateLevelOfDetail(sampler_llr, _407);
                    float2 _441 = _441_tmp.xx;
                    float _445 = _441.x + frame_constants_global_mip_bias;
                    float3 _446 = ddx(in_var_C2);
                    float3 _447 = ddy(in_var_C2);
                    float2 _449 = ddx(_407);
                    float2 _450 = ddy(_407);
                    float2 _451 = -_450;
                    float3 _452 = cross(in_var_C1, -_447);
                    float3 _453 = cross(_446, in_var_C1);
                    float _454 = _449.x;
                    float _455 = _451.y;
                    float _457 = _449.y;
                    float _458 = _451.x;
                    float _461 = 1.0f / ((_454 * _455) - (_457 * _458));
                    float3x3 _471 = transpose(float3x3(((_452 * _454) + (_453 * _458)) * _461, ((_452 * _457) + (_453 * _455)) * _461, in_var_C1));
                    float4 _476 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
                    float3 _482 = mul(_471, float3(1.0f, 0.0f, 0.0f));
                    float3 _483 = mul(_471, float3(0.0f, 1.0f, 0.0f));
                    float3 _484 = mul(_471, float3(0.0f, 0.0f, 1.0f));
                    float3 _487 = cross(_482, _483);
                    float3 _490 = (1.0f / dot(_484, _487)).xxx;
                    float3x3 _494 = float3x3(cross(_483, _484) * _490, cross(_484, _482) * _490, _487 * _490);
                    float3 _495 = mul(_494, in_var_C2 - (_476.xyz / _476.w.xxx));
                    float3 _501 = float3(_407, 0.0f);
                    float _614;
                    do
                    {
                        _505 = (_501 + (_495 * ((-0.07500000298023223876953125f) / _495.z))) - _501;
                        float _507 = 1.0f / max(1.0f, 2.0f);
                        float _510 = lerp(0.0f, 1.0f, pow(_438 * _507, 2.0f));
                        float3 _512 = _501 + (_505 * _510);
                        float _520 = _512.z - ((wooden_floor_height_3.SampleLevel(sampler_llr, _512.xy, _445).x - 1.0f) * 0.07500000298023223876953125f);
                        bool _522 = (_520 < 0.0f) ? true : false;
                        float _559;
                        float _560;
                        float _561;
                        bool _562;
                        float _563;
                        if (_522)
                        {
                            _559 = _520;
                            _560 = 0.0f;
                            _561 = _510;
                            _562 = _522;
                            _563 = 0.0f;
                        }
                        else
                        {
                            float _530;
                            float _534;
                            _530 = _520;
                            _534 = _510;
                            float _531;
                            bool _533;
                            float _535;
                            float _556;
                            float _557;
                            bool _558;
                            uint _527 = 1u;
                            bool _532 = _522;
                            for (;;)
                            {
                                if (_527 < 3u)
                                {
                                    _535 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_527) + _438) * _507, 2.0f)));
                                    float3 _545 = _501 + (_505 * _535);
                                    _531 = _545.z - ((wooden_floor_height_3.SampleLevel(sampler_llr, _545.xy, _445).x - 1.0f) * 0.07500000298023223876953125f);
                                    _533 = (_531 < 0.0f) ? true : false;
                                    if (_533)
                                    {
                                        _556 = _531;
                                        _557 = _535;
                                        _558 = _533;
                                        break;
                                    }
                                    _527++;
                                    _530 = _531;
                                    _532 = _533;
                                    _534 = _535;
                                    continue;
                                }
                                else
                                {
                                    _556 = 0.0f;
                                    _557 = 1.0f;
                                    _558 = _532;
                                    break;
                                }
                            }
                            _559 = _556;
                            _560 = _530;
                            _561 = _557;
                            _562 = _558;
                            _563 = _534;
                        }
                        if (_562)
                        {
                            float _568;
                            float _571;
                            float _573;
                            float _575;
                            _568 = _563;
                            _571 = _559;
                            _573 = _560;
                            _575 = _561;
                            for (uint _577 = 0u; _577 < 4u; )
                            {
                                float _581 = _568 + _575;
                                float _582 = _581 * 0.5f;
                                float3 _584 = _501 + (_505 * _582);
                                float _592 = _584.z - ((wooden_floor_height_3.SampleLevel(sampler_llr, _584.xy, _445).x - 1.0f) * 0.07500000298023223876953125f);
                                bool _594 = (_592 < 0.0f) ? true : false;
                                _568 = _594 ? _568 : _582;
                                _571 = _594 ? _592 : _571;
                                _573 = _594 ? _573 : _592;
                                _575 = _594 ? _582 : _575;
                                _577++;
                                continue;
                            }
                            float _598 = lerp(_568, _575, _573 / (_573 + (-_571)));
                            float3 _600 = _501 + (_505 * _598);
                            _614 = ((abs(_600.z - ((wooden_floor_height_3.SampleLevel(sampler_llr, _600.xy, _445).x - 1.0f) * 0.07500000298023223876953125f)) < (_573 * 0.89999997615814208984375f)) ? true : false) ? _598 : _575;
                            break;
                        }
                        else
                        {
                            _614 = _563;
                            break;
                        }
                        break; 
                    } while(false);
                    float3 _615 = _505 * _614;
                    float2 _617 = _407 + _615.xy;
                    float4 _629 = mul(frame_constants_view_constants[current_view_index].view_to_clip, mul(frame_constants_view_constants[current_view_index].world_to_view, float4(in_var_C2 + mul(_471, _615), 1.0f)));
                    float3 _638 = (wooden_floor_normal_3.SampleLevel(sampler_llr, _617, _445).xyz * 2.0f) - 1.0f.xxx;
                    _638.z = _638.z * 0.199999988079071044921875f;
                    float4 _649 = wooden_floor_seed_3.SampleBias(sampler_llr, _617, frame_constants_global_mip_bias);
                    float _650 = _649.z;
                    float _652 = _649.x;
                    int _656 = int(trunc((_652 * 32.0f) - 0.5f));
                    float2 _663 = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(uint(_656))) + 0.5f.xx) + (float2(_617.yx) * 0.87999999523162841796875f);
                    float _664 = _649.y;
                    float _673;
                    if (0.003130800090730190277099609375f >= _664)
                    {
                        _673 = 12.9200000762939453125f * _664;
                    }
                    else
                    {
                        _673 = (1.05499994754791259765625f * pow(_664, 0.4166666567325592041015625f)) - 0.054999999701976776123046875f;
                    }
                    float _676 = round(_673 * 8.0f) * 0.785398185253143310546875f;
                    float _677 = _663.x;
                    float _678 = cos(_676);
                    float _680 = _663.y;
                    float _681 = sin(_676);
                    float2 _687 = float2((_677 * _678) - (_680 * _681), (_677 * _681) + (_680 * _678));
                    float2 _691;
                    if (_650 > 0.5f)
                    {
                        _691 = _687 + 0.20000000298023223876953125f.xx;
                    }
                    else
                    {
                        _691 = _687;
                    }
                    float4 _698 = halftimber_colors.SampleBias(sampler_llr, float3(_691, float(_406)), frame_constants_global_mip_bias);
                    float3 _699 = _698.xyz;
                    bool _705;
                    if (!(_406 == 2))
                    {
                        _705 = _406 == 8;
                    }
                    else
                    {
                        _705 = true;
                    }
                    bool _710;
                    if (!_705)
                    {
                        _710 = _406 == 9;
                    }
                    else
                    {
                        _710 = true;
                    }
                    float3 _716;
                    if (_710)
                    {
                        _716 = lerp(_699, dot(_699, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)).xxx, 0.300000011920928955078125f.xxx);
                    }
                    else
                    {
                        _716 = _699;
                    }
                    float4 _719 = halftimber_colors.SampleBias(sampler_llr, float3(_691, 0.0f), frame_constants_global_mip_bias);
                    float3 _720 = _719.xyz;
                    float _742;
                    float3 _743;
                    if (_406 == 0)
                    {
                        _742 = _650;
                        _743 = _720 * 1.2000000476837158203125f;
                    }
                    else
                    {
                        bool _731;
                        if (!(_406 == 9))
                        {
                            _731 = _406 == 3;
                        }
                        else
                        {
                            _731 = true;
                        }
                        bool _736;
                        if (!_731)
                        {
                            _736 = _406 == 4;
                        }
                        else
                        {
                            _736 = true;
                        }
                        float _740;
                        if (_736)
                        {
                            _740 = _650 * 0.5f;
                        }
                        else
                        {
                            _740 = _650 * 0.20000000298023223876953125f;
                        }
                        _742 = _740;
                        _743 = _720;
                    }
                    uint _756 = uint(_656 ^ 9730541);
                    uint _758 = _756 + (_756 << 10u);
                    uint _760 = _758 ^ (_758 >> 6u);
                    uint _762 = _760 + (_760 << 3u);
                    uint _764 = _762 ^ (_762 >> 11u);
                    float _772 = lerp(0.75f, 1.0f, wooden_floor_roughness_3.SampleBias(sampler_llr, _617, frame_constants_global_mip_bias).x) * lerp(0.800000011920928955078125f, 1.0f, asfloat(((_764 + (_764 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
                    _1134 = smoothstep(0.5f, 1.0f, _772);
                    _1135 = (_629.xyz / _629.w.xxx).z;
                    _1136 = _772;
                    _1137 = normalize(mul(_638, _494));
                    _1138 = lerp(_716 * lerp(0.699999988079071044921875f, 1.0f, wooden_floor_height_3.SampleBias(sampler_llr, _617, frame_constants_global_mip_bias).x), _743, _742.xxx);
                    _1139 = lerp(0.0f, 0.96875f, _652);
                }
                _1499 = _1134;
                _1500 = _1135;
                _1501 = _1136;
                _1502 = _1137;
                _1503 = _1138;
                _1504 = _1139;
            }
            if (_1504 >= nani_instance_data.Load<float>(in_var_C4 * 64 + 32))
            {
                discard;
            }
            _1508 = _1502;
            _1509 = _1501;
            _1510 = _1500;
            _1511 = _1499;
            _1512 = _1503;
        }
        else
        {
            _1508 = 0.0f.xxx;
            _1509 = 0.800000011920928955078125f;
            _1510 = gl_FragCoord.z;
            _1511 = 0.0f;
            _1512 = 0.0f.xxx;
        }
        _2781 = _1508;
        _2782 = _1509;
        _2783 = _1510;
        _2784 = _1511;
        _2785 = _1512;
    }
    bool3 _2786 = _266.xxx;
    float3 _2787 = float3(_2786.x ? float3(0.800000011920928955078125f, 0.519999980926513671875f, 0.4000000059604644775390625f).x : _2785.x, _2786.y ? float3(0.800000011920928955078125f, 0.519999980926513671875f, 0.4000000059604644775390625f).y : _2785.y, _2786.z ? float3(0.800000011920928955078125f, 0.519999980926513671875f, 0.4000000059604644775390625f).z : _2785.z);
    bool _2789 = int(_273) == 1;
    float3 _2793;
    if (_2789)
    {
        _2793 = lerp(_2787, 0.5f.xxx, 0.100000001490116119384765625f.xxx);
    }
    else
    {
        _2793 = _2787;
    }
    float3 _2819;
    if (frame_constants_glade_theme_id == 4u)
    {
        float4 _2809 = dirthpath_heightmap.SampleBias(sampler_llr, (in_var_C3 * 0.0199999995529651641845703125f) + (_277 * 0.0500000007450580596923828125f), frame_constants_global_mip_bias);
        float _2810 = _2809.x;
        _2819 = lerp(_2793, 0.699999988079071044921875f.xxx, ((smoothstep(0.0f, 0.20000000298023223876953125f, _2784 * lerp(0.0f, 0.5f, _2810)) * 0.60000002384185791015625f) + (_2810 * 0.20000000298023223876953125f)).xxx);
    }
    else
    {
        _2819 = _2793;
    }
    float3 _2821 = (_2781 * 0.5f) + 0.5f.xxx;
    float3 _2848 = sqrt(clamp(_2819, 0.0f.xxx, 1.0f.xxx));
    uint _2867 = (uint((clamp(_2848.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_2848.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_2848.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 _2868 = uint2(((uint((clamp(_2821.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_2821.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_2821.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + (uint((clamp(_2782, 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u), _2867);
    _2868.y = _2867 | (((((7u | (uint(_266 ? true : (_2789 ? true : false)) << 3u)) | 0u) | 0u) | 0u) << 24u);
    out_var_SV_TARGET0 = _2868;
    gl_FragDepth = _2783;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C4 = stage_input.in_var_C4;
    in_var_C6 = stage_input.in_var_C6;
    in_var_C7 = stage_input.in_var_C7;
    in_var_C8 = stage_input.in_var_C8;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_FragDepth = gl_FragDepth;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct ViewConstants
{
    column_major float4x4 view_to_clip;
    column_major float4x4 clip_to_view;
    column_major float4x4 view_to_sample;
    column_major float4x4 sample_to_view;
    column_major float4x4 world_to_view;
    column_major float4x4 view_to_world;
    column_major float4x4 world_to_sample;
    column_major float4x4 clip_to_prev_clip;
    column_major float4x4 prev_view_to_prev_clip;
    column_major float4x4 prev_clip_to_prev_view;
    column_major float4x4 prev_world_to_prev_view;
    column_major float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
};

struct InstanceData
{
    float3 position;
    uint wall_id;
    float2 basis0_xz;
    float2 scale;
    float roof_animation;
    uint shape;
    uint double_sided;
    uint is_roof_bottom;
    uint material_id;
    uint override_color_id;
    uint pattern_id;
    uint pad_1;
};

cbuffer frame_constants : register(b0, space1)
{
    float4 frame_constants_sun_color : packoffset(c0);
    float4 frame_constants_sky_low_color : packoffset(c1);
    float4 frame_constants_sky_high_color : packoffset(c2);
    float4 frame_constants_fog_tint_thickness : packoffset(c3);
    float4 frame_constants_fake_light_color : packoffset(c4);
    float4 frame_constants_fake_light_dir : packoffset(c5);
    float4 frame_constants_fake_light2_color : packoffset(c6);
    float4 frame_constants_fake_light2_dir : packoffset(c7);
    uint frame_constants_frame_index : packoffset(c8);
    float frame_constants_delta_time_seconds : packoffset(c8.y);
    float frame_constants_ev_mult : packoffset(c8.z);
    uint frame_constants_glade_theme_id : packoffset(c8.w);
    uint2 frame_constants_inner_rendering_resolution : packoffset(c9);
    uint frame_constants_game_time_sec : packoffset(c9.z);
    float frame_constants_game_time_frac : packoffset(c9.w);
    float3 frame_constants_editing_highlight_color : packoffset(c10);
    float frame_constants_time_of_day : packoffset(c10.w);
    float3 frame_constants_dir_to_sun : packoffset(c11);
    uint frame_constants_camera_idx : packoffset(c11.w);
    float3 frame_constants_dir_to_moon : packoffset(c12);
    uint frame_constants_render_overrides : packoffset(c12.w);
    float frame_constants_global_mip_bias : packoffset(c13);
    uint frame_constants_highlighted_transient_stair_edge_id : packoffset(c13.y);
    uint frame_constants_pad1 : packoffset(c13.z);
    uint frame_constants_pad2 : packoffset(c13.w);
    float4 frame_constants_debug_tweakers : packoffset(c14);
    ViewConstants frame_constants_view_constants[5] : packoffset(c15);
};

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index : packoffset(c0);
    uint2 dirthpath_heightmap_dims : packoffset(c0.y);
    uint highlighted_wall_id : packoffset(c0.w);
    uint highlight_mode : packoffset(c1);
    uint2 wooden_floor_normal_dims : packoffset(c1.y);
    uint2 wooden_floor_height_dims : packoffset(c2);
    uint2 wooden_floor_seed_dims : packoffset(c2.z);
    uint2 wooden_floor_roughness_dims : packoffset(c3);
    uint2 wooden_floor_normal_2_dims : packoffset(c3.z);
    uint2 wooden_floor_height_2_dims : packoffset(c4);
    uint2 wooden_floor_seed_2_dims : packoffset(c4.z);
    uint2 wooden_floor_roughness_2_dims : packoffset(c5);
    uint2 wooden_floor_normal_3_dims : packoffset(c5.z);
    uint2 wooden_floor_height_3_dims : packoffset(c6);
    uint2 wooden_floor_seed_3_dims : packoffset(c6.z);
    uint2 wooden_floor_roughness_3_dims : packoffset(c7);
    uint2 stone_floor_normal_dims : packoffset(c7.z);
    uint2 stone_floor_seed_dims : packoffset(c8);
    uint2 stone_floor_height_dims : packoffset(c8.z);
    uint2 stone_floor_roughness_dims : packoffset(c9);
    uint2 stone_floor_seed_2_dims : packoffset(c9.z);
    uint2 stone_floor_normal_2_dims : packoffset(c10);
    uint2 stone_floor_height_2_dims : packoffset(c10.z);
    uint2 stone_floor_roughness_2_dims : packoffset(c11);
    uint2 stone_floor_seed_3_dims : packoffset(c11.z);
    uint2 stone_floor_normal_3_dims : packoffset(c12);
    uint2 stone_floor_height_3_dims : packoffset(c12.z);
    uint2 stone_floor_roughness_3_dims : packoffset(c13);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static uint out_var_C4;
static uint out_var_C5;
static uint out_var_C6;
static uint out_var_C7;
static uint out_var_C8;
static float3x3 out_var_C9;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] nointerpolation uint out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] nointerpolation uint out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] nointerpolation uint out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] nointerpolation uint out_var_C7 : TEXCOORD7;
    [[vk::location(8)]] nointerpolation uint out_var_C8 : TEXCOORD8;
    [[vk::location(9)]] float3x3 out_var_C9 : TEXCOORD9;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float3 _141 = float3(nani_instance_data.Load<float2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16), 0.0f).xzy;
    float3 _143 = float3(-nani_instance_data.Load<float2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16).y, 0.0f, nani_instance_data.Load<float2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16).x);
    float4 _148 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    bool _157 = (nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 44) != 0u) ? true : ((nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0) - (_148.xyz / _148.w.xxx)).y > 0.0f);
    float3 _161;
    if (_157)
    {
        _161 = _141 * (-1.0f);
    }
    else
    {
        _161 = _141;
    }
    bool3 _162 = _157.xxx;
    float3 _163 = float3(_162.x ? float3(-0.0f, -1.0f, -0.0f).x : float3(0.0f, 1.0f, 0.0f).x, _162.y ? float3(-0.0f, -1.0f, -0.0f).y : float3(0.0f, 1.0f, 0.0f).y, _162.z ? float3(-0.0f, -1.0f, -0.0f).z : float3(0.0f, 1.0f, 0.0f).z);
    float2 _165 = in_var_Vertex_Position.xz * nani_instance_data.Load<float2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 24);
    float3 _168 = mul(_165, float2x3(_161, _143)) + nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0);
    float _176 = 0.001953125f * (floor(_168.y * 512.0f) + frac(float(nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 12)) * 1.61803400516510009765625f));
    float3 _177 = _168;
    _177.y = _176;
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_168.x, _176, _168.z, 1.0f));
    out_var_C1 = _163;
    out_var_C2 = _177;
    out_var_C3 = _165;
    out_var_C4 = instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0);
    out_var_C5 = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 44);
    out_var_C6 = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 48);
    out_var_C7 = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52);
    out_var_C8 = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 56);
    out_var_C9 = transpose(float3x3(_161, _143, _163));
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
    stage_output.out_var_C7 = out_var_C7;
    stage_output.out_var_C8 = out_var_C8;
    stage_output.out_var_C9 = out_var_C9;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct ViewConstants
{
    column_major float4x4 view_to_clip;
    column_major float4x4 clip_to_view;
    column_major float4x4 view_to_sample;
    column_major float4x4 sample_to_view;
    column_major float4x4 world_to_view;
    column_major float4x4 view_to_world;
    column_major float4x4 world_to_sample;
    column_major float4x4 clip_to_prev_clip;
    column_major float4x4 prev_view_to_prev_clip;
    column_major float4x4 prev_clip_to_prev_view;
    column_major float4x4 prev_world_to_prev_view;
    column_major float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
};

struct InstanceData
{
    float3 position;
    uint wall_id;
    float2 basis0_xz;
    float2 scale;
    float roof_animation;
    uint shape;
    uint double_sided;
    uint is_roof_bottom;
    uint material_id;
    uint override_color_id;
    uint pattern_id;
    uint pad_1;
};

struct DormerHole
{
    float3 position;
    int wall_id;
    float3 size;
    int pad0;
    float2 normal;
    float k;
    float a;
};

static bool _147;

cbuffer frame_constants : register(b0, space1)
{
    float4 frame_constants_sun_color : packoffset(c0);
    float4 frame_constants_sky_low_color : packoffset(c1);
    float4 frame_constants_sky_high_color : packoffset(c2);
    float4 frame_constants_fog_tint_thickness : packoffset(c3);
    float4 frame_constants_fake_light_color : packoffset(c4);
    float4 frame_constants_fake_light_dir : packoffset(c5);
    float4 frame_constants_fake_light2_color : packoffset(c6);
    float4 frame_constants_fake_light2_dir : packoffset(c7);
    uint frame_constants_frame_index : packoffset(c8);
    float frame_constants_delta_time_seconds : packoffset(c8.y);
    float frame_constants_ev_mult : packoffset(c8.z);
    uint frame_constants_glade_theme_id : packoffset(c8.w);
    uint2 frame_constants_inner_rendering_resolution : packoffset(c9);
    uint frame_constants_game_time_sec : packoffset(c9.z);
    float frame_constants_game_time_frac : packoffset(c9.w);
    float3 frame_constants_editing_highlight_color : packoffset(c10);
    float frame_constants_time_of_day : packoffset(c10.w);
    float3 frame_constants_dir_to_sun : packoffset(c11);
    uint frame_constants_camera_idx : packoffset(c11.w);
    float3 frame_constants_dir_to_moon : packoffset(c12);
    uint frame_constants_render_overrides : packoffset(c12.w);
    float frame_constants_global_mip_bias : packoffset(c13);
    uint frame_constants_highlighted_transient_stair_edge_id : packoffset(c13.y);
    uint frame_constants_pad1 : packoffset(c13.z);
    uint frame_constants_pad2 : packoffset(c13.w);
    float4 frame_constants_debug_tweakers : packoffset(c14);
    ViewConstants frame_constants_view_constants[5] : packoffset(c15);
};

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index : packoffset(c0);
    uint2 dirthpath_heightmap_dims : packoffset(c0.y);
    uint highlighted_wall_id : packoffset(c0.w);
    uint highlight_mode : packoffset(c1);
    uint2 wooden_floor_normal_dims : packoffset(c1.y);
    uint2 wooden_floor_height_dims : packoffset(c2);
    uint2 wooden_floor_seed_dims : packoffset(c2.z);
    uint2 wooden_floor_roughness_dims : packoffset(c3);
    uint2 wooden_floor_normal_2_dims : packoffset(c3.z);
    uint2 wooden_floor_height_2_dims : packoffset(c4);
    uint2 wooden_floor_seed_2_dims : packoffset(c4.z);
    uint2 wooden_floor_roughness_2_dims : packoffset(c5);
    uint2 wooden_floor_normal_3_dims : packoffset(c5.z);
    uint2 wooden_floor_height_3_dims : packoffset(c6);
    uint2 wooden_floor_seed_3_dims : packoffset(c6.z);
    uint2 wooden_floor_roughness_3_dims : packoffset(c7);
    uint2 stone_floor_normal_dims : packoffset(c7.z);
    uint2 stone_floor_seed_dims : packoffset(c8);
    uint2 stone_floor_height_dims : packoffset(c8.z);
    uint2 stone_floor_roughness_dims : packoffset(c9);
    uint2 stone_floor_seed_2_dims : packoffset(c9.z);
    uint2 stone_floor_normal_2_dims : packoffset(c10);
    uint2 stone_floor_height_2_dims : packoffset(c10.z);
    uint2 stone_floor_roughness_2_dims : packoffset(c11);
    uint2 stone_floor_seed_3_dims : packoffset(c11.z);
    uint2 stone_floor_normal_3_dims : packoffset(c12);
    uint2 stone_floor_height_3_dims : packoffset(c12.z);
    uint2 stone_floor_roughness_3_dims : packoffset(c13);
};

ByteAddressBuffer nani_instance_data : register(t16, space0);
ByteAddressBuffer dormer_holes : register(t18, space0);
ByteAddressBuffer dormer_hole_count : register(t19, space0);
SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> wooden_floor_height : register(t23, space0);
Texture2D<float4> wooden_floor_seed : register(t24, space0);
Texture2D<float4> wooden_floor_height_2 : register(t27, space0);
Texture2D<float4> wooden_floor_seed_2 : register(t28, space0);
Texture2D<float4> wooden_floor_height_3 : register(t31, space0);
Texture2D<float4> wooden_floor_seed_3 : register(t32, space0);
Texture2D<float4> stone_floor_seed : register(t38, space0);
Texture2D<float4> stone_floor_height : register(t39, space0);
Texture2D<float4> stone_floor_seed_2 : register(t41, space0);
Texture2D<float4> stone_floor_height_2 : register(t43, space0);
Texture2D<float4> stone_floor_seed_3 : register(t45, space0);
Texture2D<float4> stone_floor_height_3 : register(t47, space0);

static float4 gl_FragCoord;
static float3 in_var_C1;
static float3 in_var_C2;
static float2 in_var_C3;
static uint in_var_C4;
static uint in_var_C6;
static uint in_var_C8;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 in_var_C3 : TEXCOORD3;
    [[vk::location(4)]] nointerpolation uint in_var_C4 : TEXCOORD4;
    [[vk::location(6)]] nointerpolation uint in_var_C6 : TEXCOORD6;
    [[vk::location(8)]] nointerpolation uint in_var_C8 : TEXCOORD8;
    float4 gl_FragCoord : SV_Position;
};

void frag_main()
{
    int _169 = int(nani_instance_data.Load<uint>(in_var_C4 * 64 + 12));
    bool _267;
    do
    {
        float2 _174 = (in_var_C2.xz + 65.0f.xx) * 0.1153846085071563720703125f.xx;
        int _182 = int(floor(_174.x)) + (15 * int(floor(_174.y)));
        uint _186 = min(128u, dormer_hole_count.Load<uint>(uint(_182) * 4 + 0));
        bool _264;
        bool _265;
        uint _188 = 0u;
        for (;;)
        {
            if (_188 < _186)
            {
                uint _198 = uint(int(uint(_182 * 50) + _188));
                if (_169 == dormer_holes.Load<int>(_198 * 48 + 12))
                {
                    float3 _214 = float3(dormer_holes.Load<float>(_198 * 48 + 32), 0.0f, dormer_holes.Load<float>(_198 * 48 + 36));
                    float3x3 _218 = transpose(float3x3(cross(_214, float3(0.0f, 1.0f, 0.0f)), float3(0.0f, 1.0f, 0.0f), _214));
                    float3 _219 = mul(_218, float3(1.0f, 0.0f, 0.0f));
                    float3 _220 = mul(_218, float3(0.0f, 1.0f, 0.0f));
                    float3 _221 = mul(_218, float3(0.0f, 0.0f, 1.0f));
                    float3 _224 = cross(_219, _220);
                    float3 _227 = (1.0f / dot(_221, _224)).xxx;
                    float3 _235 = mul(in_var_C2 - dormer_holes.Load<float3>(_198 * 48 + 0), float3x3(cross(_220, _221) * _227, cross(_221, _219) * _227, _224 * _227)) / dormer_holes.Load<float3>(_198 * 48 + 16);
                    float _236 = _235.y;
                    float _239 = clamp((_236 * dormer_holes.Load<float>(_198 * 48 + 40)) + dormer_holes.Load<float>(_198 * 48 + 44), 0.0f, 1.0f);
                    bool _255;
                    if (abs(_235.x) < lerp(0.5f, 0.0f, (_239 * _239) * _239))
                    {
                        bool _254;
                        if (_236 < ((dormer_holes.Load<float>(_198 * 48 + 40) == 1.0f) ? 0.5f : 0.699999988079071044921875f))
                        {
                            _254 = _236 > (-0.5f);
                        }
                        else
                        {
                            _254 = false;
                        }
                        _255 = _254;
                    }
                    else
                    {
                        _255 = false;
                    }
                    bool _261;
                    if (_255)
                    {
                        _261 = abs(_235.z) < 0.5f;
                    }
                    else
                    {
                        _261 = false;
                    }
                    if (_261)
                    {
                        _264 = true;
                        _265 = true;
                        break;
                    }
                }
                _188++;
                continue;
            }
            else
            {
                _264 = _147;
                _265 = false;
                break;
            }
        }
        if (_265)
        {
            _267 = _264;
            break;
        }
        _267 = false;
        break;
    } while(false);
    if (_267)
    {
        discard;
    }
    int _270 = int(in_var_C6);
    bool _276;
    if (!(_270 == 0))
    {
        _276 = _270 == 2;
    }
    else
    {
        _276 = true;
    }
    if (_276)
    {
        float2 _914 = in_var_C3 * 0.20000000298023223876953125f;
        float _1536;
        if (in_var_C8 == 0u)
        {
            float3 _1419;
            uint2 _1335 = uint2(gl_FragCoord.yx);
            float _1342 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
            float _1352 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_1335.x) + _1342)) + (0.005837149918079376220703125f * (float(_1335.y) + _1342))));
            float _1355_tmp = stone_floor_height.CalculateLevelOfDetail(sampler_llr, _914);
            float2 _1355 = _1355_tmp.xx;
            float _1359 = _1355.x + frame_constants_global_mip_bias;
            float3 _1360 = ddx(in_var_C2);
            float3 _1361 = ddy(in_var_C2);
            float2 _1363 = ddx(_914);
            float2 _1364 = ddy(_914);
            float2 _1365 = -_1364;
            float3 _1366 = cross(in_var_C1, -_1361);
            float3 _1367 = cross(_1360, in_var_C1);
            float _1368 = _1363.x;
            float _1369 = _1365.y;
            float _1371 = _1363.y;
            float _1372 = _1365.x;
            float _1375 = 1.0f / ((_1368 * _1369) - (_1371 * _1372));
            float3x3 _1385 = transpose(float3x3(((_1366 * _1368) + (_1367 * _1372)) * _1375, ((_1366 * _1371) + (_1367 * _1369)) * _1375, in_var_C1));
            float4 _1390 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
            float3 _1396 = mul(_1385, float3(1.0f, 0.0f, 0.0f));
            float3 _1397 = mul(_1385, float3(0.0f, 1.0f, 0.0f));
            float3 _1398 = mul(_1385, float3(0.0f, 0.0f, 1.0f));
            float3 _1401 = cross(_1396, _1397);
            float3 _1404 = (1.0f / dot(_1398, _1401)).xxx;
            float3 _1409 = mul(float3x3(cross(_1397, _1398) * _1404, cross(_1398, _1396) * _1404, _1401 * _1404), in_var_C2 - (_1390.xyz / _1390.w.xxx));
            float3 _1415 = float3(_914, 0.0f);
            float _1528;
            do
            {
                _1419 = (_1415 + (_1409 * ((-0.07500000298023223876953125f) / _1409.z))) - _1415;
                float _1421 = 1.0f / max(1.0f, 2.0f);
                float _1424 = lerp(0.0f, 1.0f, pow(_1352 * _1421, 2.0f));
                float3 _1426 = _1415 + (_1419 * _1424);
                float _1434 = _1426.z - ((stone_floor_height.SampleLevel(sampler_llr, _1426.xy, _1359).x - 1.0f) * 0.07500000298023223876953125f);
                bool _1436 = (_1434 < 0.0f) ? true : false;
                float _1473;
                float _1474;
                float _1475;
                bool _1476;
                float _1477;
                if (_1436)
                {
                    _1473 = _1434;
                    _1474 = 0.0f;
                    _1475 = _1424;
                    _1476 = _1436;
                    _1477 = 0.0f;
                }
                else
                {
                    float _1444;
                    float _1448;
                    _1444 = _1434;
                    _1448 = _1424;
                    float _1445;
                    bool _1447;
                    float _1449;
                    float _1470;
                    float _1471;
                    bool _1472;
                    uint _1441 = 1u;
                    bool _1446 = _1436;
                    for (;;)
                    {
                        if (_1441 < 3u)
                        {
                            _1449 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_1441) + _1352) * _1421, 2.0f)));
                            float3 _1459 = _1415 + (_1419 * _1449);
                            _1445 = _1459.z - ((stone_floor_height.SampleLevel(sampler_llr, _1459.xy, _1359).x - 1.0f) * 0.07500000298023223876953125f);
                            _1447 = (_1445 < 0.0f) ? true : false;
                            if (_1447)
                            {
                                _1470 = _1445;
                                _1471 = _1449;
                                _1472 = _1447;
                                break;
                            }
                            _1441++;
                            _1444 = _1445;
                            _1446 = _1447;
                            _1448 = _1449;
                            continue;
                        }
                        else
                        {
                            _1470 = 0.0f;
                            _1471 = 1.0f;
                            _1472 = _1446;
                            break;
                        }
                    }
                    _1473 = _1470;
                    _1474 = _1444;
                    _1475 = _1471;
                    _1476 = _1472;
                    _1477 = _1448;
                }
                if (_1476)
                {
                    float _1482;
                    float _1485;
                    float _1487;
                    float _1489;
                    _1482 = _1477;
                    _1485 = _1473;
                    _1487 = _1474;
                    _1489 = _1475;
                    for (uint _1491 = 0u; _1491 < 4u; )
                    {
                        float _1495 = _1482 + _1489;
                        float _1496 = _1495 * 0.5f;
                        float3 _1498 = _1415 + (_1419 * _1496);
                        float _1506 = _1498.z - ((stone_floor_height.SampleLevel(sampler_llr, _1498.xy, _1359).x - 1.0f) * 0.07500000298023223876953125f);
                        bool _1508 = (_1506 < 0.0f) ? true : false;
                        _1482 = _1508 ? _1482 : _1496;
                        _1485 = _1508 ? _1506 : _1485;
                        _1487 = _1508 ? _1487 : _1506;
                        _1489 = _1508 ? _1496 : _1489;
                        _1491++;
                        continue;
                    }
                    float _1512 = lerp(_1482, _1489, _1487 / (_1487 + (-_1485)));
                    float3 _1514 = _1415 + (_1419 * _1512);
                    _1528 = ((abs(_1514.z - ((stone_floor_height.SampleLevel(sampler_llr, _1514.xy, _1359).x - 1.0f) * 0.07500000298023223876953125f)) < (_1487 * 0.89999997615814208984375f)) ? true : false) ? _1512 : _1489;
                    break;
                }
                else
                {
                    _1528 = _1477;
                    break;
                }
                break; 
            } while(false);
            _1536 = lerp(0.0f, 0.96875f, stone_floor_seed.SampleBias(sampler_llr, _914 + (_1419 * _1528).xy, frame_constants_global_mip_bias).x);
        }
        else
        {
            float _1331;
            if (in_var_C8 == 1u)
            {
                float3 _1214;
                uint2 _1130 = uint2(gl_FragCoord.yx);
                float _1137 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
                float _1147 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_1130.x) + _1137)) + (0.005837149918079376220703125f * (float(_1130.y) + _1137))));
                float _1150_tmp = stone_floor_height_2.CalculateLevelOfDetail(sampler_llr, _914);
                float2 _1150 = _1150_tmp.xx;
                float _1154 = _1150.x + frame_constants_global_mip_bias;
                float3 _1155 = ddx(in_var_C2);
                float3 _1156 = ddy(in_var_C2);
                float2 _1158 = ddx(_914);
                float2 _1159 = ddy(_914);
                float2 _1160 = -_1159;
                float3 _1161 = cross(in_var_C1, -_1156);
                float3 _1162 = cross(_1155, in_var_C1);
                float _1163 = _1158.x;
                float _1164 = _1160.y;
                float _1166 = _1158.y;
                float _1167 = _1160.x;
                float _1170 = 1.0f / ((_1163 * _1164) - (_1166 * _1167));
                float3x3 _1180 = transpose(float3x3(((_1161 * _1163) + (_1162 * _1167)) * _1170, ((_1161 * _1166) + (_1162 * _1164)) * _1170, in_var_C1));
                float4 _1185 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
                float3 _1191 = mul(_1180, float3(1.0f, 0.0f, 0.0f));
                float3 _1192 = mul(_1180, float3(0.0f, 1.0f, 0.0f));
                float3 _1193 = mul(_1180, float3(0.0f, 0.0f, 1.0f));
                float3 _1196 = cross(_1191, _1192);
                float3 _1199 = (1.0f / dot(_1193, _1196)).xxx;
                float3 _1204 = mul(float3x3(cross(_1192, _1193) * _1199, cross(_1193, _1191) * _1199, _1196 * _1199), in_var_C2 - (_1185.xyz / _1185.w.xxx));
                float3 _1210 = float3(_914, 0.0f);
                float _1323;
                do
                {
                    _1214 = (_1210 + (_1204 * ((-0.0500000007450580596923828125f) / _1204.z))) - _1210;
                    float _1216 = 1.0f / max(1.0f, 2.0f);
                    float _1219 = lerp(0.0f, 1.0f, pow(_1147 * _1216, 2.0f));
                    float3 _1221 = _1210 + (_1214 * _1219);
                    float _1229 = _1221.z - ((stone_floor_height_2.SampleLevel(sampler_llr, _1221.xy, _1154).x - 1.0f) * 0.0500000007450580596923828125f);
                    bool _1231 = (_1229 < 0.0f) ? true : false;
                    float _1268;
                    float _1269;
                    float _1270;
                    bool _1271;
                    float _1272;
                    if (_1231)
                    {
                        _1268 = _1229;
                        _1269 = 0.0f;
                        _1270 = _1219;
                        _1271 = _1231;
                        _1272 = 0.0f;
                    }
                    else
                    {
                        float _1239;
                        float _1243;
                        _1239 = _1229;
                        _1243 = _1219;
                        float _1240;
                        bool _1242;
                        float _1244;
                        float _1265;
                        float _1266;
                        bool _1267;
                        uint _1236 = 1u;
                        bool _1241 = _1231;
                        for (;;)
                        {
                            if (_1236 < 3u)
                            {
                                _1244 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_1236) + _1147) * _1216, 2.0f)));
                                float3 _1254 = _1210 + (_1214 * _1244);
                                _1240 = _1254.z - ((stone_floor_height_2.SampleLevel(sampler_llr, _1254.xy, _1154).x - 1.0f) * 0.0500000007450580596923828125f);
                                _1242 = (_1240 < 0.0f) ? true : false;
                                if (_1242)
                                {
                                    _1265 = _1240;
                                    _1266 = _1244;
                                    _1267 = _1242;
                                    break;
                                }
                                _1236++;
                                _1239 = _1240;
                                _1241 = _1242;
                                _1243 = _1244;
                                continue;
                            }
                            else
                            {
                                _1265 = 0.0f;
                                _1266 = 1.0f;
                                _1267 = _1241;
                                break;
                            }
                        }
                        _1268 = _1265;
                        _1269 = _1239;
                        _1270 = _1266;
                        _1271 = _1267;
                        _1272 = _1243;
                    }
                    if (_1271)
                    {
                        float _1277;
                        float _1280;
                        float _1282;
                        float _1284;
                        _1277 = _1272;
                        _1280 = _1268;
                        _1282 = _1269;
                        _1284 = _1270;
                        for (uint _1286 = 0u; _1286 < 4u; )
                        {
                            float _1290 = _1277 + _1284;
                            float _1291 = _1290 * 0.5f;
                            float3 _1293 = _1210 + (_1214 * _1291);
                            float _1301 = _1293.z - ((stone_floor_height_2.SampleLevel(sampler_llr, _1293.xy, _1154).x - 1.0f) * 0.0500000007450580596923828125f);
                            bool _1303 = (_1301 < 0.0f) ? true : false;
                            _1277 = _1303 ? _1277 : _1291;
                            _1280 = _1303 ? _1301 : _1280;
                            _1282 = _1303 ? _1282 : _1301;
                            _1284 = _1303 ? _1291 : _1284;
                            _1286++;
                            continue;
                        }
                        float _1307 = lerp(_1277, _1284, _1282 / (_1282 + (-_1280)));
                        float3 _1309 = _1210 + (_1214 * _1307);
                        _1323 = ((abs(_1309.z - ((stone_floor_height_2.SampleLevel(sampler_llr, _1309.xy, _1154).x - 1.0f) * 0.0500000007450580596923828125f)) < (_1282 * 0.89999997615814208984375f)) ? true : false) ? _1307 : _1284;
                        break;
                    }
                    else
                    {
                        _1323 = _1272;
                        break;
                    }
                    break; 
                } while(false);
                _1331 = lerp(0.0f, 0.96875f, stone_floor_seed_2.SampleBias(sampler_llr, _914 + (_1214 * _1323).xy, frame_constants_global_mip_bias).x);
            }
            else
            {
                float3 _1010;
                uint2 _926 = uint2(gl_FragCoord.yx);
                float _933 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
                float _943 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_926.x) + _933)) + (0.005837149918079376220703125f * (float(_926.y) + _933))));
                float _946_tmp = stone_floor_height_3.CalculateLevelOfDetail(sampler_llr, _914);
                float2 _946 = _946_tmp.xx;
                float _950 = _946.x + frame_constants_global_mip_bias;
                float3 _951 = ddx(in_var_C2);
                float3 _952 = ddy(in_var_C2);
                float2 _954 = ddx(_914);
                float2 _955 = ddy(_914);
                float2 _956 = -_955;
                float3 _957 = cross(in_var_C1, -_952);
                float3 _958 = cross(_951, in_var_C1);
                float _959 = _954.x;
                float _960 = _956.y;
                float _962 = _954.y;
                float _963 = _956.x;
                float _966 = 1.0f / ((_959 * _960) - (_962 * _963));
                float3x3 _976 = transpose(float3x3(((_957 * _959) + (_958 * _963)) * _966, ((_957 * _962) + (_958 * _960)) * _966, in_var_C1));
                float4 _981 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
                float3 _987 = mul(_976, float3(1.0f, 0.0f, 0.0f));
                float3 _988 = mul(_976, float3(0.0f, 1.0f, 0.0f));
                float3 _989 = mul(_976, float3(0.0f, 0.0f, 1.0f));
                float3 _992 = cross(_987, _988);
                float3 _995 = (1.0f / dot(_989, _992)).xxx;
                float3 _1000 = mul(float3x3(cross(_988, _989) * _995, cross(_989, _987) * _995, _992 * _995), in_var_C2 - (_981.xyz / _981.w.xxx));
                float3 _1006 = float3(_914, 0.0f);
                float _1119;
                do
                {
                    _1010 = (_1006 + (_1000 * ((-0.0500000007450580596923828125f) / _1000.z))) - _1006;
                    float _1012 = 1.0f / max(1.0f, 2.0f);
                    float _1015 = lerp(0.0f, 1.0f, pow(_943 * _1012, 2.0f));
                    float3 _1017 = _1006 + (_1010 * _1015);
                    float _1025 = _1017.z - ((stone_floor_height_3.SampleLevel(sampler_llr, _1017.xy, _950).x - 1.0f) * 0.0500000007450580596923828125f);
                    bool _1027 = (_1025 < 0.0f) ? true : false;
                    float _1064;
                    float _1065;
                    float _1066;
                    bool _1067;
                    float _1068;
                    if (_1027)
                    {
                        _1064 = _1025;
                        _1065 = 0.0f;
                        _1066 = _1015;
                        _1067 = _1027;
                        _1068 = 0.0f;
                    }
                    else
                    {
                        float _1035;
                        float _1039;
                        _1035 = _1025;
                        _1039 = _1015;
                        float _1036;
                        bool _1038;
                        float _1040;
                        float _1061;
                        float _1062;
                        bool _1063;
                        uint _1032 = 1u;
                        bool _1037 = _1027;
                        for (;;)
                        {
                            if (_1032 < 3u)
                            {
                                _1040 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_1032) + _943) * _1012, 2.0f)));
                                float3 _1050 = _1006 + (_1010 * _1040);
                                _1036 = _1050.z - ((stone_floor_height_3.SampleLevel(sampler_llr, _1050.xy, _950).x - 1.0f) * 0.0500000007450580596923828125f);
                                _1038 = (_1036 < 0.0f) ? true : false;
                                if (_1038)
                                {
                                    _1061 = _1036;
                                    _1062 = _1040;
                                    _1063 = _1038;
                                    break;
                                }
                                _1032++;
                                _1035 = _1036;
                                _1037 = _1038;
                                _1039 = _1040;
                                continue;
                            }
                            else
                            {
                                _1061 = 0.0f;
                                _1062 = 1.0f;
                                _1063 = _1037;
                                break;
                            }
                        }
                        _1064 = _1061;
                        _1065 = _1035;
                        _1066 = _1062;
                        _1067 = _1063;
                        _1068 = _1039;
                    }
                    if (_1067)
                    {
                        float _1073;
                        float _1076;
                        float _1078;
                        float _1080;
                        _1073 = _1068;
                        _1076 = _1064;
                        _1078 = _1065;
                        _1080 = _1066;
                        for (uint _1082 = 0u; _1082 < 4u; )
                        {
                            float _1086 = _1073 + _1080;
                            float _1087 = _1086 * 0.5f;
                            float3 _1089 = _1006 + (_1010 * _1087);
                            float _1097 = _1089.z - ((stone_floor_height_3.SampleLevel(sampler_llr, _1089.xy, _950).x - 1.0f) * 0.0500000007450580596923828125f);
                            bool _1099 = (_1097 < 0.0f) ? true : false;
                            _1073 = _1099 ? _1073 : _1087;
                            _1076 = _1099 ? _1097 : _1076;
                            _1078 = _1099 ? _1078 : _1097;
                            _1080 = _1099 ? _1087 : _1080;
                            _1082++;
                            continue;
                        }
                        float _1103 = lerp(_1073, _1080, _1078 / (_1078 + (-_1076)));
                        float3 _1105 = _1006 + (_1010 * _1103);
                        _1119 = ((abs(_1105.z - ((stone_floor_height_3.SampleLevel(sampler_llr, _1105.xy, _950).x - 1.0f) * 0.0500000007450580596923828125f)) < (_1078 * 0.89999997615814208984375f)) ? true : false) ? _1103 : _1080;
                        break;
                    }
                    else
                    {
                        _1119 = _1068;
                        break;
                    }
                    break; 
                } while(false);
                _1331 = lerp(0.0f, 0.96875f, stone_floor_seed_3.SampleBias(sampler_llr, _914 + (_1010 * _1119).xy, frame_constants_global_mip_bias).x);
            }
            _1536 = _1331;
        }
        if ((_1536 * _1536) >= nani_instance_data.Load<float>(in_var_C4 * 64 + 32))
        {
            discard;
        }
    }
    else
    {
        bool _285;
        if (!(_270 == 1))
        {
            _285 = _270 == 3;
        }
        else
        {
            _285 = true;
        }
        if (_285)
        {
            float2 _288 = in_var_C3 * 0.20000000298023223876953125f;
            float _910;
            if (in_var_C8 == 0u)
            {
                float3 _793;
                uint2 _709 = uint2(gl_FragCoord.yx);
                float _716 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
                float _726 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_709.x) + _716)) + (0.005837149918079376220703125f * (float(_709.y) + _716))));
                float _729_tmp = wooden_floor_height.CalculateLevelOfDetail(sampler_llr, _288);
                float2 _729 = _729_tmp.xx;
                float _733 = _729.x + frame_constants_global_mip_bias;
                float3 _734 = ddx(in_var_C2);
                float3 _735 = ddy(in_var_C2);
                float2 _737 = ddx(_288);
                float2 _738 = ddy(_288);
                float2 _739 = -_738;
                float3 _740 = cross(in_var_C1, -_735);
                float3 _741 = cross(_734, in_var_C1);
                float _742 = _737.x;
                float _743 = _739.y;
                float _745 = _737.y;
                float _746 = _739.x;
                float _749 = 1.0f / ((_742 * _743) - (_745 * _746));
                float3x3 _759 = transpose(float3x3(((_740 * _742) + (_741 * _746)) * _749, ((_740 * _745) + (_741 * _743)) * _749, in_var_C1));
                float4 _764 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
                float3 _770 = mul(_759, float3(1.0f, 0.0f, 0.0f));
                float3 _771 = mul(_759, float3(0.0f, 1.0f, 0.0f));
                float3 _772 = mul(_759, float3(0.0f, 0.0f, 1.0f));
                float3 _775 = cross(_770, _771);
                float3 _778 = (1.0f / dot(_772, _775)).xxx;
                float3 _783 = mul(float3x3(cross(_771, _772) * _778, cross(_772, _770) * _778, _775 * _778), in_var_C2 - (_764.xyz / _764.w.xxx));
                float3 _789 = float3(_288, 0.0f);
                float _902;
                do
                {
                    _793 = (_789 + (_783 * ((-0.07500000298023223876953125f) / _783.z))) - _789;
                    float _795 = 1.0f / max(1.0f, 2.0f);
                    float _798 = lerp(0.0f, 1.0f, pow(_726 * _795, 2.0f));
                    float3 _800 = _789 + (_793 * _798);
                    float _808 = _800.z - ((wooden_floor_height.SampleLevel(sampler_llr, _800.xy, _733).x - 1.0f) * 0.07500000298023223876953125f);
                    bool _810 = (_808 < 0.0f) ? true : false;
                    float _847;
                    float _848;
                    float _849;
                    bool _850;
                    float _851;
                    if (_810)
                    {
                        _847 = _808;
                        _848 = 0.0f;
                        _849 = _798;
                        _850 = _810;
                        _851 = 0.0f;
                    }
                    else
                    {
                        float _818;
                        float _822;
                        _818 = _808;
                        _822 = _798;
                        float _819;
                        bool _821;
                        float _823;
                        float _844;
                        float _845;
                        bool _846;
                        uint _815 = 1u;
                        bool _820 = _810;
                        for (;;)
                        {
                            if (_815 < 3u)
                            {
                                _823 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_815) + _726) * _795, 2.0f)));
                                float3 _833 = _789 + (_793 * _823);
                                _819 = _833.z - ((wooden_floor_height.SampleLevel(sampler_llr, _833.xy, _733).x - 1.0f) * 0.07500000298023223876953125f);
                                _821 = (_819 < 0.0f) ? true : false;
                                if (_821)
                                {
                                    _844 = _819;
                                    _845 = _823;
                                    _846 = _821;
                                    break;
                                }
                                _815++;
                                _818 = _819;
                                _820 = _821;
                                _822 = _823;
                                continue;
                            }
                            else
                            {
                                _844 = 0.0f;
                                _845 = 1.0f;
                                _846 = _820;
                                break;
                            }
                        }
                        _847 = _844;
                        _848 = _818;
                        _849 = _845;
                        _850 = _846;
                        _851 = _822;
                    }
                    if (_850)
                    {
                        float _856;
                        float _859;
                        float _861;
                        float _863;
                        _856 = _851;
                        _859 = _847;
                        _861 = _848;
                        _863 = _849;
                        for (uint _865 = 0u; _865 < 4u; )
                        {
                            float _869 = _856 + _863;
                            float _870 = _869 * 0.5f;
                            float3 _872 = _789 + (_793 * _870);
                            float _880 = _872.z - ((wooden_floor_height.SampleLevel(sampler_llr, _872.xy, _733).x - 1.0f) * 0.07500000298023223876953125f);
                            bool _882 = (_880 < 0.0f) ? true : false;
                            _856 = _882 ? _856 : _870;
                            _859 = _882 ? _880 : _859;
                            _861 = _882 ? _861 : _880;
                            _863 = _882 ? _870 : _863;
                            _865++;
                            continue;
                        }
                        float _886 = lerp(_856, _863, _861 / (_861 + (-_859)));
                        float3 _888 = _789 + (_793 * _886);
                        _902 = ((abs(_888.z - ((wooden_floor_height.SampleLevel(sampler_llr, _888.xy, _733).x - 1.0f) * 0.07500000298023223876953125f)) < (_861 * 0.89999997615814208984375f)) ? true : false) ? _886 : _863;
                        break;
                    }
                    else
                    {
                        _902 = _851;
                        break;
                    }
                    break; 
                } while(false);
                _910 = lerp(0.0f, 0.96875f, wooden_floor_seed.SampleBias(sampler_llr, _288 + (_793 * _902).xy, frame_constants_global_mip_bias).x);
            }
            else
            {
                float _705;
                if (in_var_C8 == 1u)
                {
                    float3 _588;
                    uint2 _504 = uint2(gl_FragCoord.yx);
                    float _511 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
                    float _521 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_504.x) + _511)) + (0.005837149918079376220703125f * (float(_504.y) + _511))));
                    float _524_tmp = wooden_floor_height_2.CalculateLevelOfDetail(sampler_llr, _288);
                    float2 _524 = _524_tmp.xx;
                    float _528 = _524.x + frame_constants_global_mip_bias;
                    float3 _529 = ddx(in_var_C2);
                    float3 _530 = ddy(in_var_C2);
                    float2 _532 = ddx(_288);
                    float2 _533 = ddy(_288);
                    float2 _534 = -_533;
                    float3 _535 = cross(in_var_C1, -_530);
                    float3 _536 = cross(_529, in_var_C1);
                    float _537 = _532.x;
                    float _538 = _534.y;
                    float _540 = _532.y;
                    float _541 = _534.x;
                    float _544 = 1.0f / ((_537 * _538) - (_540 * _541));
                    float3x3 _554 = transpose(float3x3(((_535 * _537) + (_536 * _541)) * _544, ((_535 * _540) + (_536 * _538)) * _544, in_var_C1));
                    float4 _559 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
                    float3 _565 = mul(_554, float3(1.0f, 0.0f, 0.0f));
                    float3 _566 = mul(_554, float3(0.0f, 1.0f, 0.0f));
                    float3 _567 = mul(_554, float3(0.0f, 0.0f, 1.0f));
                    float3 _570 = cross(_565, _566);
                    float3 _573 = (1.0f / dot(_567, _570)).xxx;
                    float3 _578 = mul(float3x3(cross(_566, _567) * _573, cross(_567, _565) * _573, _570 * _573), in_var_C2 - (_559.xyz / _559.w.xxx));
                    float3 _584 = float3(_288, 0.0f);
                    float _697;
                    do
                    {
                        _588 = (_584 + (_578 * ((-0.07500000298023223876953125f) / _578.z))) - _584;
                        float _590 = 1.0f / max(1.0f, 2.0f);
                        float _593 = lerp(0.0f, 1.0f, pow(_521 * _590, 2.0f));
                        float3 _595 = _584 + (_588 * _593);
                        float _603 = _595.z - ((wooden_floor_height_2.SampleLevel(sampler_llr, _595.xy, _528).x - 1.0f) * 0.07500000298023223876953125f);
                        bool _605 = (_603 < 0.0f) ? true : false;
                        float _642;
                        float _643;
                        float _644;
                        bool _645;
                        float _646;
                        if (_605)
                        {
                            _642 = _603;
                            _643 = 0.0f;
                            _644 = _593;
                            _645 = _605;
                            _646 = 0.0f;
                        }
                        else
                        {
                            float _613;
                            float _617;
                            _613 = _603;
                            _617 = _593;
                            float _614;
                            bool _616;
                            float _618;
                            float _639;
                            float _640;
                            bool _641;
                            uint _610 = 1u;
                            bool _615 = _605;
                            for (;;)
                            {
                                if (_610 < 3u)
                                {
                                    _618 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_610) + _521) * _590, 2.0f)));
                                    float3 _628 = _584 + (_588 * _618);
                                    _614 = _628.z - ((wooden_floor_height_2.SampleLevel(sampler_llr, _628.xy, _528).x - 1.0f) * 0.07500000298023223876953125f);
                                    _616 = (_614 < 0.0f) ? true : false;
                                    if (_616)
                                    {
                                        _639 = _614;
                                        _640 = _618;
                                        _641 = _616;
                                        break;
                                    }
                                    _610++;
                                    _613 = _614;
                                    _615 = _616;
                                    _617 = _618;
                                    continue;
                                }
                                else
                                {
                                    _639 = 0.0f;
                                    _640 = 1.0f;
                                    _641 = _615;
                                    break;
                                }
                            }
                            _642 = _639;
                            _643 = _613;
                            _644 = _640;
                            _645 = _641;
                            _646 = _617;
                        }
                        if (_645)
                        {
                            float _651;
                            float _654;
                            float _656;
                            float _658;
                            _651 = _646;
                            _654 = _642;
                            _656 = _643;
                            _658 = _644;
                            for (uint _660 = 0u; _660 < 4u; )
                            {
                                float _664 = _651 + _658;
                                float _665 = _664 * 0.5f;
                                float3 _667 = _584 + (_588 * _665);
                                float _675 = _667.z - ((wooden_floor_height_2.SampleLevel(sampler_llr, _667.xy, _528).x - 1.0f) * 0.07500000298023223876953125f);
                                bool _677 = (_675 < 0.0f) ? true : false;
                                _651 = _677 ? _651 : _665;
                                _654 = _677 ? _675 : _654;
                                _656 = _677 ? _656 : _675;
                                _658 = _677 ? _665 : _658;
                                _660++;
                                continue;
                            }
                            float _681 = lerp(_651, _658, _656 / (_656 + (-_654)));
                            float3 _683 = _584 + (_588 * _681);
                            _697 = ((abs(_683.z - ((wooden_floor_height_2.SampleLevel(sampler_llr, _683.xy, _528).x - 1.0f) * 0.07500000298023223876953125f)) < (_656 * 0.89999997615814208984375f)) ? true : false) ? _681 : _658;
                            break;
                        }
                        else
                        {
                            _697 = _646;
                            break;
                        }
                        break; 
                    } while(false);
                    _705 = lerp(0.0f, 0.96875f, wooden_floor_seed_2.SampleBias(sampler_llr, _288 + (_588 * _697).xy, frame_constants_global_mip_bias).x);
                }
                else
                {
                    float3 _384;
                    uint2 _300 = uint2(gl_FragCoord.yx);
                    float _307 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
                    float _317 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_300.x) + _307)) + (0.005837149918079376220703125f * (float(_300.y) + _307))));
                    float _320_tmp = wooden_floor_height_3.CalculateLevelOfDetail(sampler_llr, _288);
                    float2 _320 = _320_tmp.xx;
                    float _324 = _320.x + frame_constants_global_mip_bias;
                    float3 _325 = ddx(in_var_C2);
                    float3 _326 = ddy(in_var_C2);
                    float2 _328 = ddx(_288);
                    float2 _329 = ddy(_288);
                    float2 _330 = -_329;
                    float3 _331 = cross(in_var_C1, -_326);
                    float3 _332 = cross(_325, in_var_C1);
                    float _333 = _328.x;
                    float _334 = _330.y;
                    float _336 = _328.y;
                    float _337 = _330.x;
                    float _340 = 1.0f / ((_333 * _334) - (_336 * _337));
                    float3x3 _350 = transpose(float3x3(((_331 * _333) + (_332 * _337)) * _340, ((_331 * _336) + (_332 * _334)) * _340, in_var_C1));
                    float4 _355 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
                    float3 _361 = mul(_350, float3(1.0f, 0.0f, 0.0f));
                    float3 _362 = mul(_350, float3(0.0f, 1.0f, 0.0f));
                    float3 _363 = mul(_350, float3(0.0f, 0.0f, 1.0f));
                    float3 _366 = cross(_361, _362);
                    float3 _369 = (1.0f / dot(_363, _366)).xxx;
                    float3 _374 = mul(float3x3(cross(_362, _363) * _369, cross(_363, _361) * _369, _366 * _369), in_var_C2 - (_355.xyz / _355.w.xxx));
                    float3 _380 = float3(_288, 0.0f);
                    float _493;
                    do
                    {
                        _384 = (_380 + (_374 * ((-0.07500000298023223876953125f) / _374.z))) - _380;
                        float _386 = 1.0f / max(1.0f, 2.0f);
                        float _389 = lerp(0.0f, 1.0f, pow(_317 * _386, 2.0f));
                        float3 _391 = _380 + (_384 * _389);
                        float _399 = _391.z - ((wooden_floor_height_3.SampleLevel(sampler_llr, _391.xy, _324).x - 1.0f) * 0.07500000298023223876953125f);
                        bool _401 = (_399 < 0.0f) ? true : false;
                        float _438;
                        float _439;
                        float _440;
                        bool _441;
                        float _442;
                        if (_401)
                        {
                            _438 = _399;
                            _439 = 0.0f;
                            _440 = _389;
                            _441 = _401;
                            _442 = 0.0f;
                        }
                        else
                        {
                            float _409;
                            float _413;
                            _409 = _399;
                            _413 = _389;
                            float _410;
                            bool _412;
                            float _414;
                            float _435;
                            float _436;
                            bool _437;
                            uint _406 = 1u;
                            bool _411 = _401;
                            for (;;)
                            {
                                if (_406 < 3u)
                                {
                                    _414 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_406) + _317) * _386, 2.0f)));
                                    float3 _424 = _380 + (_384 * _414);
                                    _410 = _424.z - ((wooden_floor_height_3.SampleLevel(sampler_llr, _424.xy, _324).x - 1.0f) * 0.07500000298023223876953125f);
                                    _412 = (_410 < 0.0f) ? true : false;
                                    if (_412)
                                    {
                                        _435 = _410;
                                        _436 = _414;
                                        _437 = _412;
                                        break;
                                    }
                                    _406++;
                                    _409 = _410;
                                    _411 = _412;
                                    _413 = _414;
                                    continue;
                                }
                                else
                                {
                                    _435 = 0.0f;
                                    _436 = 1.0f;
                                    _437 = _411;
                                    break;
                                }
                            }
                            _438 = _435;
                            _439 = _409;
                            _440 = _436;
                            _441 = _437;
                            _442 = _413;
                        }
                        if (_441)
                        {
                            float _447;
                            float _450;
                            float _452;
                            float _454;
                            _447 = _442;
                            _450 = _438;
                            _452 = _439;
                            _454 = _440;
                            for (uint _456 = 0u; _456 < 4u; )
                            {
                                float _460 = _447 + _454;
                                float _461 = _460 * 0.5f;
                                float3 _463 = _380 + (_384 * _461);
                                float _471 = _463.z - ((wooden_floor_height_3.SampleLevel(sampler_llr, _463.xy, _324).x - 1.0f) * 0.07500000298023223876953125f);
                                bool _473 = (_471 < 0.0f) ? true : false;
                                _447 = _473 ? _447 : _461;
                                _450 = _473 ? _471 : _450;
                                _452 = _473 ? _452 : _471;
                                _454 = _473 ? _461 : _454;
                                _456++;
                                continue;
                            }
                            float _477 = lerp(_447, _454, _452 / (_452 + (-_450)));
                            float3 _479 = _380 + (_384 * _477);
                            _493 = ((abs(_479.z - ((wooden_floor_height_3.SampleLevel(sampler_llr, _479.xy, _324).x - 1.0f) * 0.07500000298023223876953125f)) < (_452 * 0.89999997615814208984375f)) ? true : false) ? _477 : _454;
                            break;
                        }
                        else
                        {
                            _493 = _442;
                            break;
                        }
                        break; 
                    } while(false);
                    _705 = lerp(0.0f, 0.96875f, wooden_floor_seed_3.SampleBias(sampler_llr, _288 + (_384 * _493).xy, frame_constants_global_mip_bias).x);
                }
                _910 = _705;
            }
            if (_910 >= nani_instance_data.Load<float>(in_var_C4 * 64 + 32))
            {
                discard;
            }
        }
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C4 = stage_input.in_var_C4;
    in_var_C6 = stage_input.in_var_C6;
    in_var_C8 = stage_input.in_var_C8;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
