#define SHADER_VARIANT_DEPTH_ONLY 1
#define TGV_f0adff76827e73ff 1
#line 1 "shaders/nani_balcony_door.raster.hlsl"


#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))





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
#line 9 "shaders/nani_balcony_door.raster.hlsl"

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
#line 10 "shaders/nani_balcony_door.raster.hlsl"

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
#line 11 "shaders/nani_balcony_door.raster.hlsl"

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
#line 12 "shaders/nani_balcony_door.raster.hlsl"

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
#line 13 "shaders/nani_balcony_door.raster.hlsl"

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
#line 14 "shaders/nani_balcony_door.raster.hlsl"

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
#line 15 "shaders/nani_balcony_door.raster.hlsl"

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
#line 16 "shaders/nani_balcony_door.raster.hlsl"

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
#line 17 "shaders/nani_balcony_door.raster.hlsl"

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
#line 18 "shaders/nani_balcony_door.raster.hlsl"

#line 1 "shaders/inc/wall_distortion.hlsl"







#ifndef TINYGLADE_INC_WALL_DISTORTION_HLSL
#define TINYGLADE_INC_WALL_DISTORTION_HLSL



float2 wall_distortion_uv(float2 uv, float2 world_pos, float time)
{
    float2 wave = float2(sin(world_pos.x * 0.125f + time * 0.5f),
                         cos(world_pos.y * 0.125f + time * 0.5f));
    return uv + wave * 0.0025f;
}

#endif 
#line 19 "shaders/nani_balcony_door.raster.hlsl"

#line 1 "shaders/inc/window_color_customization.hlsl"


#ifndef TINYGLADE_INC_WINDOW_COLOR_CUSTOMIZATION_HLSL
#define TINYGLADE_INC_WINDOW_COLOR_CUSTOMIZATION_HLSL


float3 sample_window_color(Texture2D window_colors, SamplerState s, uint color_id)
{
    float2 uv = float2((float(color_id) + 0.5) / 16.0, 0.5);
    return window_colors.SampleLevel(s, uv, 0.0).xyz;
}


float3 get_window_color(Texture2D window_colors, SamplerState s, uint color_id, float seed)
{
    if (color_id == 0xFFFFFFFFu) {
        return color_library.wall_1.xyz;
    }
    return sample_window_color(window_colors, s, color_id);
}

#endif 
#line 20 "shaders/nani_balcony_door.raster.hlsl"

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
#line 21 "shaders/nani_balcony_door.raster.hlsl"

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
#line 22 "shaders/nani_balcony_door.raster.hlsl"

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
#line 23 "shaders/nani_balcony_door.raster.hlsl"

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
#line 24 "shaders/nani_balcony_door.raster.hlsl"

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
#line 25 "shaders/nani_balcony_door.raster.hlsl"

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
#line 26 "shaders/nani_balcony_door.raster.hlsl"

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
#line 27 "shaders/nani_balcony_door.raster.hlsl"

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
#line 28 "shaders/nani_balcony_door.raster.hlsl"

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
#line 29 "shaders/nani_balcony_door.raster.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 30 "shaders/nani_balcony_door.raster.hlsl"

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
#line 31 "shaders/nani_balcony_door.raster.hlsl"

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
#line 32 "shaders/nani_balcony_door.raster.hlsl"

#line 1 "shaders/inc/sh.hlsl"










float4 sh_basis(float3 dir)
{
    return float4(0.2820949852466583251953125f, 0.48860299587249755859375f * dir);
}

float sh_eval(float4 sh, float3 dir)
{
    return dot(sh, sh_basis(dir));
}
#line 33 "shaders/nani_balcony_door.raster.hlsl"


int bool_render_shadows;

struct InstanceData
{
    Affine3Packed xform;
    uint is_semitransparent;
    uint color_id;
    uint pad0;
    uint pad1;
};

Texture2DArray<float4> window_colors : register(t14);
StructuredBuffer<uint> instance_indirection_buffer : register(t15);
StructuredBuffer<InstanceData> nani_instance_data : register(t16);

uint hash3(uint3 p)
{
    p.y *= 1664525u;
    uint yz = p.z;
    yz += (yz << 10u);
    yz ^= (yz >> 6u);
    yz += (yz << 3u);
    yz ^= (yz >> 11u);
    p.y += yz + (yz << 15u);
    p.y += 1013904223u;
    p.y *= 1664525u;
    uint y = p.y ^ (p.y >> 11u);
    y = y ^ ((y << 7u) & 2636928640u);
    y = y ^ ((y << 15u) & 4022730752u);
    p.x *= 1664525u;
    p.x += y ^ (y >> 18u);
    p.x += 1013904223u;
    p.x *= 1664525u;
    uint x = p.x ^ (p.x >> 11u);
    x = x ^ ((x << 7u) & 2636928640u);
    x = x ^ ((x << 15u) & 4022730752u);
    x = x ^ (x >> 18u);
    x += (x << 10u);
    x ^= (x >> 6u);
    x += (x << 3u);
    x ^= (x >> 11u);
    return x;
}

float2 corner_rand(uint3 cell)
{
    uint h = hash3(cell);
    float r1 = asfloat((h & 8388607u) | 1065353216u) - 1.0f;
    h = h + (h << 10u);
    h = h ^ (h >> 6u);
    h = h + (h << 3u);
    h = h ^ (h >> 11u);
    float r2 = asfloat(((h + (h << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    return float2(r1, r2);
}

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 Vertex_UV: Vertex_UV;
    [[vk::location(4)]] int is_metal_part: is_metal_part;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    nointerpolation int C4 : TEXCOORD4;
    nointerpolation int C5 : TEXCOORD5;
    nointerpolation uint C6 : TEXCOORD6;
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

    
    float3 scaled = world * 0.4f;
    int3 cell = int3(floor(scaled));
    float3 fr = frac(scaled);
    float3 f = fr * fr * (3.0f.xxx - (fr * 2.0f));
    float2 n = float2(0.0f, 0.0f);
    n += corner_rand(uint3(cell)) * (1.0f - f.x) * (1.0f - f.y) * (1.0f - f.z);
    n += corner_rand(uint3(cell + int3(1, 0, 0))) * f.x * (1.0f - f.y) * (1.0f - f.z);
    n += corner_rand(uint3(cell + int3(0, 1, 0))) * (1.0f - f.x) * f.y * (1.0f - f.z);
    n += corner_rand(uint3(cell + int3(1, 1, 0))) * f.x * f.y * (1.0f - f.z);
    n += corner_rand(uint3(cell + int3(0, 0, 1))) * (1.0f - f.x) * (1.0f - f.y) * f.z;
    n += corner_rand(uint3(cell + int3(1, 0, 1))) * f.x * (1.0f - f.y) * f.z;
    n += corner_rand(uint3(cell + int3(0, 1, 1))) * (1.0f - f.x) * f.y * f.z;
    n += corner_rand(uint3(cell + int3(1, 1, 1))) * f.x * f.y * f.z;
    float2 offset = clamp((n - 0.5f.xx) * 2.5f, -0.5f.xx, 0.5f.xx);
    float3 displaced = world + (float3(offset.x, 0.0f, offset.y) * 0.06f);

    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(displaced, 1.0f));
    o.C0 = vin.Vertex_Color;
    o.C1 = mul(xform, float4(vin.Vertex_Normal, 0.0f));
    o.C2 = displaced;
    o.C3 = vin.Vertex_UV;
    o.C4 = vin.is_metal_part;
    o.C5 = int(nani_instance_data[idx].color_id);
    o.C6 = nani_instance_data[idx].is_semitransparent;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    nointerpolation int C4 : TEXCOORD4;
    nointerpolation int C5 : TEXCOORD5;
    nointerpolation uint C6 : TEXCOORD6;
    float4 position : SV_Position;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    bool dither = pin.C6 != 0u && bool_render_shadows != 0;
    if (dither)
    {
        static const uint2 hi_px_subpixels[4] =
        {
            uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u)
        };
        uint2 sub = hi_px_subpixels[frame_index & 3u];
        uint2 cell = uint2(pin.position.xy) & uint2(1u, 1u);
        if (all(cell != sub))
        {
            discard;
        }
    }

    float3 albedo;
    if (pin.C4 != 1)
    {
        uint seed = uint(pin.C5);
        seed = hash(seed);
        float rnd = asfloat((seed & 8388607u) | 1065353216u) - 1.0f;
        float3 door_color = window_colors.Sample(sampler_llr, float3((pin.C3 + rnd.xx) + (pin.C2.xz * 0.1f), float(pin.C5))).xyz;
        if (pin.C5 == 0)
        {
            door_color = door_color * 0.7f;
        }
        albedo = pin.C5 == 0 ? lerp(door_color, pin.C0, 0.3f.xxx) : door_color;
    }
    else
    {
        albedo = pin.C0;
    }

    float3 packed_normal = (normalize(pin.C1) * 0.5f) + 0.5f.xxx;
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

struct Affine3Packed
{
    float data[12];
};

struct InstanceData
{
    Affine3Packed xform;
    uint is_semitransparent;
    int color_id;
    uint pad0;
    uint pad1;
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
    int bool_render_shadows : packoffset(c0.y);
};

ByteAddressBuffer instance_indirection_buffer : register(t15, space0);
ByteAddressBuffer nani_instance_data : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static int in_var_is_metal_part;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static int out_var_C4;
static int out_var_C5;
static uint out_var_C6;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(4)]] int in_var_is_metal_part: is_metal_part;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] nointerpolation int out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] nointerpolation int out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] nointerpolation uint out_var_C6 : TEXCOORD6;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    Affine3Packed _142;
    [unroll]
    for (int _2ident = 0; _2ident < 12; _2ident++)
    {
        _142.data[_2ident] = nani_instance_data.Load<float>(_2ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0);
    }
    float3x4 _165 = transpose(float4x3(float3(_142.data[0], _142.data[1], _142.data[2]), float3(_142.data[3], _142.data[4], _142.data[5]), float3(_142.data[6], _142.data[7], _142.data[8]), float3(_142.data[9], _142.data[10], _142.data[11])));
    float3 _170 = mul(_165, float4(in_var_Vertex_Position, 1.0f));
    float3 _171 = _170 * 0.4000000059604644775390625f;
    int3 _173 = int3(floor(_171));
    float3 _174 = frac(_171);
    uint3 _175 = uint3(_173);
    uint _178 = _175.z;
    uint _180 = _178 + (_178 << 10u);
    uint _182 = _180 ^ (_180 >> 6u);
    uint _184 = _182 + (_182 << 3u);
    uint _186 = _184 ^ (_184 >> 11u);
    uint _192 = (((_175.y * 1664525u) + (_186 + (_186 << 15u))) + 1013904223u) * 1664525u;
    uint _194 = _192 ^ (_192 >> 11u);
    uint _197 = _194 ^ ((_194 << 7u) & 2636928640u);
    uint _200 = _197 ^ ((_197 << 15u) & 4022730752u);
    uint _206 = (((_175.x * 1664525u) + (_200 ^ (_200 >> 18u))) + 1013904223u) * 1664525u;
    uint _208 = _206 ^ (_206 >> 11u);
    uint _211 = _208 ^ ((_208 << 7u) & 2636928640u);
    uint _214 = _211 ^ ((_211 << 15u) & 4022730752u);
    uint _216 = _214 ^ (_214 >> 18u);
    uint _218 = _216 + (_216 << 10u);
    uint _220 = _218 ^ (_218 >> 6u);
    uint _222 = _220 + (_220 << 3u);
    uint _224 = _222 ^ (_222 >> 11u);
    uint3 _237 = uint3(_173 + int3(1, 0, 0));
    uint _240 = _237.z;
    uint _242 = _240 + (_240 << 10u);
    uint _244 = _242 ^ (_242 >> 6u);
    uint _246 = _244 + (_244 << 3u);
    uint _248 = _246 ^ (_246 >> 11u);
    uint _254 = (((_237.y * 1664525u) + (_248 + (_248 << 15u))) + 1013904223u) * 1664525u;
    uint _256 = _254 ^ (_254 >> 11u);
    uint _259 = _256 ^ ((_256 << 7u) & 2636928640u);
    uint _262 = _259 ^ ((_259 << 15u) & 4022730752u);
    uint _268 = (((_237.x * 1664525u) + (_262 ^ (_262 >> 18u))) + 1013904223u) * 1664525u;
    uint _270 = _268 ^ (_268 >> 11u);
    uint _273 = _270 ^ ((_270 << 7u) & 2636928640u);
    uint _276 = _273 ^ ((_273 << 15u) & 4022730752u);
    uint _278 = _276 ^ (_276 >> 18u);
    uint _280 = _278 + (_278 << 10u);
    uint _282 = _280 ^ (_280 >> 6u);
    uint _284 = _282 + (_282 << 3u);
    uint _286 = _284 ^ (_284 >> 11u);
    uint3 _299 = uint3(_173 + int3(0, 1, 0));
    uint _302 = _299.z;
    uint _304 = _302 + (_302 << 10u);
    uint _306 = _304 ^ (_304 >> 6u);
    uint _308 = _306 + (_306 << 3u);
    uint _310 = _308 ^ (_308 >> 11u);
    uint _316 = (((_299.y * 1664525u) + (_310 + (_310 << 15u))) + 1013904223u) * 1664525u;
    uint _318 = _316 ^ (_316 >> 11u);
    uint _321 = _318 ^ ((_318 << 7u) & 2636928640u);
    uint _324 = _321 ^ ((_321 << 15u) & 4022730752u);
    uint _330 = (((_299.x * 1664525u) + (_324 ^ (_324 >> 18u))) + 1013904223u) * 1664525u;
    uint _332 = _330 ^ (_330 >> 11u);
    uint _335 = _332 ^ ((_332 << 7u) & 2636928640u);
    uint _338 = _335 ^ ((_335 << 15u) & 4022730752u);
    uint _340 = _338 ^ (_338 >> 18u);
    uint _342 = _340 + (_340 << 10u);
    uint _344 = _342 ^ (_342 >> 6u);
    uint _346 = _344 + (_344 << 3u);
    uint _348 = _346 ^ (_346 >> 11u);
    uint3 _361 = uint3(_173 + int3(1, 1, 0));
    uint _364 = _361.z;
    uint _366 = _364 + (_364 << 10u);
    uint _368 = _366 ^ (_366 >> 6u);
    uint _370 = _368 + (_368 << 3u);
    uint _372 = _370 ^ (_370 >> 11u);
    uint _378 = (((_361.y * 1664525u) + (_372 + (_372 << 15u))) + 1013904223u) * 1664525u;
    uint _380 = _378 ^ (_378 >> 11u);
    uint _383 = _380 ^ ((_380 << 7u) & 2636928640u);
    uint _386 = _383 ^ ((_383 << 15u) & 4022730752u);
    uint _392 = (((_361.x * 1664525u) + (_386 ^ (_386 >> 18u))) + 1013904223u) * 1664525u;
    uint _394 = _392 ^ (_392 >> 11u);
    uint _397 = _394 ^ ((_394 << 7u) & 2636928640u);
    uint _400 = _397 ^ ((_397 << 15u) & 4022730752u);
    uint _402 = _400 ^ (_400 >> 18u);
    uint _404 = _402 + (_402 << 10u);
    uint _406 = _404 ^ (_404 >> 6u);
    uint _408 = _406 + (_406 << 3u);
    uint _410 = _408 ^ (_408 >> 11u);
    uint3 _423 = uint3(_173 + int3(0, 0, 1));
    uint _426 = _423.z;
    uint _428 = _426 + (_426 << 10u);
    uint _430 = _428 ^ (_428 >> 6u);
    uint _432 = _430 + (_430 << 3u);
    uint _434 = _432 ^ (_432 >> 11u);
    uint _440 = (((_423.y * 1664525u) + (_434 + (_434 << 15u))) + 1013904223u) * 1664525u;
    uint _442 = _440 ^ (_440 >> 11u);
    uint _445 = _442 ^ ((_442 << 7u) & 2636928640u);
    uint _448 = _445 ^ ((_445 << 15u) & 4022730752u);
    uint _454 = (((_423.x * 1664525u) + (_448 ^ (_448 >> 18u))) + 1013904223u) * 1664525u;
    uint _456 = _454 ^ (_454 >> 11u);
    uint _459 = _456 ^ ((_456 << 7u) & 2636928640u);
    uint _462 = _459 ^ ((_459 << 15u) & 4022730752u);
    uint _464 = _462 ^ (_462 >> 18u);
    uint _466 = _464 + (_464 << 10u);
    uint _468 = _466 ^ (_466 >> 6u);
    uint _470 = _468 + (_468 << 3u);
    uint _472 = _470 ^ (_470 >> 11u);
    uint3 _485 = uint3(_173 + int3(1, 0, 1));
    uint _488 = _485.z;
    uint _490 = _488 + (_488 << 10u);
    uint _492 = _490 ^ (_490 >> 6u);
    uint _494 = _492 + (_492 << 3u);
    uint _496 = _494 ^ (_494 >> 11u);
    uint _502 = (((_485.y * 1664525u) + (_496 + (_496 << 15u))) + 1013904223u) * 1664525u;
    uint _504 = _502 ^ (_502 >> 11u);
    uint _507 = _504 ^ ((_504 << 7u) & 2636928640u);
    uint _510 = _507 ^ ((_507 << 15u) & 4022730752u);
    uint _516 = (((_485.x * 1664525u) + (_510 ^ (_510 >> 18u))) + 1013904223u) * 1664525u;
    uint _518 = _516 ^ (_516 >> 11u);
    uint _521 = _518 ^ ((_518 << 7u) & 2636928640u);
    uint _524 = _521 ^ ((_521 << 15u) & 4022730752u);
    uint _526 = _524 ^ (_524 >> 18u);
    uint _528 = _526 + (_526 << 10u);
    uint _530 = _528 ^ (_528 >> 6u);
    uint _532 = _530 + (_530 << 3u);
    uint _534 = _532 ^ (_532 >> 11u);
    uint3 _547 = uint3(_173 + int3(0, 1, 1));
    uint _550 = _547.z;
    uint _552 = _550 + (_550 << 10u);
    uint _554 = _552 ^ (_552 >> 6u);
    uint _556 = _554 + (_554 << 3u);
    uint _558 = _556 ^ (_556 >> 11u);
    uint _564 = (((_547.y * 1664525u) + (_558 + (_558 << 15u))) + 1013904223u) * 1664525u;
    uint _566 = _564 ^ (_564 >> 11u);
    uint _569 = _566 ^ ((_566 << 7u) & 2636928640u);
    uint _572 = _569 ^ ((_569 << 15u) & 4022730752u);
    uint _578 = (((_547.x * 1664525u) + (_572 ^ (_572 >> 18u))) + 1013904223u) * 1664525u;
    uint _580 = _578 ^ (_578 >> 11u);
    uint _583 = _580 ^ ((_580 << 7u) & 2636928640u);
    uint _586 = _583 ^ ((_583 << 15u) & 4022730752u);
    uint _588 = _586 ^ (_586 >> 18u);
    uint _590 = _588 + (_588 << 10u);
    uint _592 = _590 ^ (_590 >> 6u);
    uint _594 = _592 + (_592 << 3u);
    uint _596 = _594 ^ (_594 >> 11u);
    uint3 _609 = uint3(_173 + int3(1, 1, 1));
    uint _612 = _609.z;
    uint _614 = _612 + (_612 << 10u);
    uint _616 = _614 ^ (_614 >> 6u);
    uint _618 = _616 + (_616 << 3u);
    uint _620 = _618 ^ (_618 >> 11u);
    uint _626 = (((_609.y * 1664525u) + (_620 + (_620 << 15u))) + 1013904223u) * 1664525u;
    uint _628 = _626 ^ (_626 >> 11u);
    uint _631 = _628 ^ ((_628 << 7u) & 2636928640u);
    uint _634 = _631 ^ ((_631 << 15u) & 4022730752u);
    uint _640 = (((_609.x * 1664525u) + (_634 ^ (_634 >> 18u))) + 1013904223u) * 1664525u;
    uint _642 = _640 ^ (_640 >> 11u);
    uint _645 = _642 ^ ((_642 << 7u) & 2636928640u);
    uint _648 = _645 ^ ((_645 << 15u) & 4022730752u);
    uint _650 = _648 ^ (_648 >> 18u);
    uint _652 = _650 + (_650 << 10u);
    uint _654 = _652 ^ (_652 >> 6u);
    uint _656 = _654 + (_654 << 3u);
    uint _658 = _656 ^ (_656 >> 11u);
    float3 _673 = (_174 * _174) * (3.0f.xxx - (_174 * 2.0f));
    float _674 = _673.x;
    float _675 = 1.0f - _674;
    float _677 = _673.y;
    float _678 = 1.0f - _677;
    float _680 = _673.z;
    float _681 = 1.0f - _680;
    float2 _698 = ((((((float2(asfloat((_216 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_224 + (_224 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _675) * _678) * _681) + (((float2(asfloat((_278 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_286 + (_286 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _674) * _678) * _681)) + (((float2(asfloat((_340 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_348 + (_348 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _675) * _677) * _681)) + (((float2(asfloat((_402 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_410 + (_410 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _674) * _677) * _681)) + (((float2(asfloat((_464 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_472 + (_472 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _675) * _678) * _680);
    float2 _713 = clamp(((((_698 + (((float2(asfloat((_526 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_534 + (_534 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _674) * _678) * _680)) + (((float2(asfloat((_588 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_596 + (_596 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _675) * _677) * _680)) + (((float2(asfloat((_650 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_658 + (_658 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _674) * _677) * _680)) - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 _718 = _170 + (float3(_713.x, 0.0f, _713.y) * 0.0599999986588954925537109375f);
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_718, 1.0f));
    out_var_C0 = in_var_Vertex_Color;
    out_var_C1 = mul(_165, float4(in_var_Vertex_Normal, 0.0f));
    out_var_C2 = _718;
    out_var_C3 = in_var_Vertex_UV;
    out_var_C4 = in_var_is_metal_part;
    out_var_C5 = nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52);
    out_var_C6 = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 48);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_is_metal_part = stage_input.in_var_is_metal_part;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
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

struct Affine3Packed
{
    float data[12];
};

struct InstanceData
{
    Affine3Packed xform;
    uint is_semitransparent;
    int color_id;
    uint pad0;
    uint pad1;
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
    int bool_render_shadows : packoffset(c0.y);
};

ByteAddressBuffer instance_indirection_buffer : register(t15, space0);
ByteAddressBuffer nani_instance_data : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static int in_var_is_metal_part;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static int out_var_C4;
static int out_var_C5;
static uint out_var_C6;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(4)]] int in_var_is_metal_part: is_metal_part;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] nointerpolation int out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] nointerpolation int out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] nointerpolation uint out_var_C6 : TEXCOORD6;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    Affine3Packed _142;
    [unroll]
    for (int _2ident = 0; _2ident < 12; _2ident++)
    {
        _142.data[_2ident] = nani_instance_data.Load<float>(_2ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0);
    }
    float3x4 _165 = transpose(float4x3(float3(_142.data[0], _142.data[1], _142.data[2]), float3(_142.data[3], _142.data[4], _142.data[5]), float3(_142.data[6], _142.data[7], _142.data[8]), float3(_142.data[9], _142.data[10], _142.data[11])));
    float3 _170 = mul(_165, float4(in_var_Vertex_Position, 1.0f));
    float3 _171 = _170 * 0.4000000059604644775390625f;
    int3 _173 = int3(floor(_171));
    float3 _174 = frac(_171);
    uint3 _175 = uint3(_173);
    uint _178 = _175.z;
    uint _180 = _178 + (_178 << 10u);
    uint _182 = _180 ^ (_180 >> 6u);
    uint _184 = _182 + (_182 << 3u);
    uint _186 = _184 ^ (_184 >> 11u);
    uint _192 = (((_175.y * 1664525u) + (_186 + (_186 << 15u))) + 1013904223u) * 1664525u;
    uint _194 = _192 ^ (_192 >> 11u);
    uint _197 = _194 ^ ((_194 << 7u) & 2636928640u);
    uint _200 = _197 ^ ((_197 << 15u) & 4022730752u);
    uint _206 = (((_175.x * 1664525u) + (_200 ^ (_200 >> 18u))) + 1013904223u) * 1664525u;
    uint _208 = _206 ^ (_206 >> 11u);
    uint _211 = _208 ^ ((_208 << 7u) & 2636928640u);
    uint _214 = _211 ^ ((_211 << 15u) & 4022730752u);
    uint _216 = _214 ^ (_214 >> 18u);
    uint _218 = _216 + (_216 << 10u);
    uint _220 = _218 ^ (_218 >> 6u);
    uint _222 = _220 + (_220 << 3u);
    uint _224 = _222 ^ (_222 >> 11u);
    uint3 _237 = uint3(_173 + int3(1, 0, 0));
    uint _240 = _237.z;
    uint _242 = _240 + (_240 << 10u);
    uint _244 = _242 ^ (_242 >> 6u);
    uint _246 = _244 + (_244 << 3u);
    uint _248 = _246 ^ (_246 >> 11u);
    uint _254 = (((_237.y * 1664525u) + (_248 + (_248 << 15u))) + 1013904223u) * 1664525u;
    uint _256 = _254 ^ (_254 >> 11u);
    uint _259 = _256 ^ ((_256 << 7u) & 2636928640u);
    uint _262 = _259 ^ ((_259 << 15u) & 4022730752u);
    uint _268 = (((_237.x * 1664525u) + (_262 ^ (_262 >> 18u))) + 1013904223u) * 1664525u;
    uint _270 = _268 ^ (_268 >> 11u);
    uint _273 = _270 ^ ((_270 << 7u) & 2636928640u);
    uint _276 = _273 ^ ((_273 << 15u) & 4022730752u);
    uint _278 = _276 ^ (_276 >> 18u);
    uint _280 = _278 + (_278 << 10u);
    uint _282 = _280 ^ (_280 >> 6u);
    uint _284 = _282 + (_282 << 3u);
    uint _286 = _284 ^ (_284 >> 11u);
    uint3 _299 = uint3(_173 + int3(0, 1, 0));
    uint _302 = _299.z;
    uint _304 = _302 + (_302 << 10u);
    uint _306 = _304 ^ (_304 >> 6u);
    uint _308 = _306 + (_306 << 3u);
    uint _310 = _308 ^ (_308 >> 11u);
    uint _316 = (((_299.y * 1664525u) + (_310 + (_310 << 15u))) + 1013904223u) * 1664525u;
    uint _318 = _316 ^ (_316 >> 11u);
    uint _321 = _318 ^ ((_318 << 7u) & 2636928640u);
    uint _324 = _321 ^ ((_321 << 15u) & 4022730752u);
    uint _330 = (((_299.x * 1664525u) + (_324 ^ (_324 >> 18u))) + 1013904223u) * 1664525u;
    uint _332 = _330 ^ (_330 >> 11u);
    uint _335 = _332 ^ ((_332 << 7u) & 2636928640u);
    uint _338 = _335 ^ ((_335 << 15u) & 4022730752u);
    uint _340 = _338 ^ (_338 >> 18u);
    uint _342 = _340 + (_340 << 10u);
    uint _344 = _342 ^ (_342 >> 6u);
    uint _346 = _344 + (_344 << 3u);
    uint _348 = _346 ^ (_346 >> 11u);
    uint3 _361 = uint3(_173 + int3(1, 1, 0));
    uint _364 = _361.z;
    uint _366 = _364 + (_364 << 10u);
    uint _368 = _366 ^ (_366 >> 6u);
    uint _370 = _368 + (_368 << 3u);
    uint _372 = _370 ^ (_370 >> 11u);
    uint _378 = (((_361.y * 1664525u) + (_372 + (_372 << 15u))) + 1013904223u) * 1664525u;
    uint _380 = _378 ^ (_378 >> 11u);
    uint _383 = _380 ^ ((_380 << 7u) & 2636928640u);
    uint _386 = _383 ^ ((_383 << 15u) & 4022730752u);
    uint _392 = (((_361.x * 1664525u) + (_386 ^ (_386 >> 18u))) + 1013904223u) * 1664525u;
    uint _394 = _392 ^ (_392 >> 11u);
    uint _397 = _394 ^ ((_394 << 7u) & 2636928640u);
    uint _400 = _397 ^ ((_397 << 15u) & 4022730752u);
    uint _402 = _400 ^ (_400 >> 18u);
    uint _404 = _402 + (_402 << 10u);
    uint _406 = _404 ^ (_404 >> 6u);
    uint _408 = _406 + (_406 << 3u);
    uint _410 = _408 ^ (_408 >> 11u);
    uint3 _423 = uint3(_173 + int3(0, 0, 1));
    uint _426 = _423.z;
    uint _428 = _426 + (_426 << 10u);
    uint _430 = _428 ^ (_428 >> 6u);
    uint _432 = _430 + (_430 << 3u);
    uint _434 = _432 ^ (_432 >> 11u);
    uint _440 = (((_423.y * 1664525u) + (_434 + (_434 << 15u))) + 1013904223u) * 1664525u;
    uint _442 = _440 ^ (_440 >> 11u);
    uint _445 = _442 ^ ((_442 << 7u) & 2636928640u);
    uint _448 = _445 ^ ((_445 << 15u) & 4022730752u);
    uint _454 = (((_423.x * 1664525u) + (_448 ^ (_448 >> 18u))) + 1013904223u) * 1664525u;
    uint _456 = _454 ^ (_454 >> 11u);
    uint _459 = _456 ^ ((_456 << 7u) & 2636928640u);
    uint _462 = _459 ^ ((_459 << 15u) & 4022730752u);
    uint _464 = _462 ^ (_462 >> 18u);
    uint _466 = _464 + (_464 << 10u);
    uint _468 = _466 ^ (_466 >> 6u);
    uint _470 = _468 + (_468 << 3u);
    uint _472 = _470 ^ (_470 >> 11u);
    uint3 _485 = uint3(_173 + int3(1, 0, 1));
    uint _488 = _485.z;
    uint _490 = _488 + (_488 << 10u);
    uint _492 = _490 ^ (_490 >> 6u);
    uint _494 = _492 + (_492 << 3u);
    uint _496 = _494 ^ (_494 >> 11u);
    uint _502 = (((_485.y * 1664525u) + (_496 + (_496 << 15u))) + 1013904223u) * 1664525u;
    uint _504 = _502 ^ (_502 >> 11u);
    uint _507 = _504 ^ ((_504 << 7u) & 2636928640u);
    uint _510 = _507 ^ ((_507 << 15u) & 4022730752u);
    uint _516 = (((_485.x * 1664525u) + (_510 ^ (_510 >> 18u))) + 1013904223u) * 1664525u;
    uint _518 = _516 ^ (_516 >> 11u);
    uint _521 = _518 ^ ((_518 << 7u) & 2636928640u);
    uint _524 = _521 ^ ((_521 << 15u) & 4022730752u);
    uint _526 = _524 ^ (_524 >> 18u);
    uint _528 = _526 + (_526 << 10u);
    uint _530 = _528 ^ (_528 >> 6u);
    uint _532 = _530 + (_530 << 3u);
    uint _534 = _532 ^ (_532 >> 11u);
    uint3 _547 = uint3(_173 + int3(0, 1, 1));
    uint _550 = _547.z;
    uint _552 = _550 + (_550 << 10u);
    uint _554 = _552 ^ (_552 >> 6u);
    uint _556 = _554 + (_554 << 3u);
    uint _558 = _556 ^ (_556 >> 11u);
    uint _564 = (((_547.y * 1664525u) + (_558 + (_558 << 15u))) + 1013904223u) * 1664525u;
    uint _566 = _564 ^ (_564 >> 11u);
    uint _569 = _566 ^ ((_566 << 7u) & 2636928640u);
    uint _572 = _569 ^ ((_569 << 15u) & 4022730752u);
    uint _578 = (((_547.x * 1664525u) + (_572 ^ (_572 >> 18u))) + 1013904223u) * 1664525u;
    uint _580 = _578 ^ (_578 >> 11u);
    uint _583 = _580 ^ ((_580 << 7u) & 2636928640u);
    uint _586 = _583 ^ ((_583 << 15u) & 4022730752u);
    uint _588 = _586 ^ (_586 >> 18u);
    uint _590 = _588 + (_588 << 10u);
    uint _592 = _590 ^ (_590 >> 6u);
    uint _594 = _592 + (_592 << 3u);
    uint _596 = _594 ^ (_594 >> 11u);
    uint3 _609 = uint3(_173 + int3(1, 1, 1));
    uint _612 = _609.z;
    uint _614 = _612 + (_612 << 10u);
    uint _616 = _614 ^ (_614 >> 6u);
    uint _618 = _616 + (_616 << 3u);
    uint _620 = _618 ^ (_618 >> 11u);
    uint _626 = (((_609.y * 1664525u) + (_620 + (_620 << 15u))) + 1013904223u) * 1664525u;
    uint _628 = _626 ^ (_626 >> 11u);
    uint _631 = _628 ^ ((_628 << 7u) & 2636928640u);
    uint _634 = _631 ^ ((_631 << 15u) & 4022730752u);
    uint _640 = (((_609.x * 1664525u) + (_634 ^ (_634 >> 18u))) + 1013904223u) * 1664525u;
    uint _642 = _640 ^ (_640 >> 11u);
    uint _645 = _642 ^ ((_642 << 7u) & 2636928640u);
    uint _648 = _645 ^ ((_645 << 15u) & 4022730752u);
    uint _650 = _648 ^ (_648 >> 18u);
    uint _652 = _650 + (_650 << 10u);
    uint _654 = _652 ^ (_652 >> 6u);
    uint _656 = _654 + (_654 << 3u);
    uint _658 = _656 ^ (_656 >> 11u);
    float3 _673 = (_174 * _174) * (3.0f.xxx - (_174 * 2.0f));
    float _674 = _673.x;
    float _675 = 1.0f - _674;
    float _677 = _673.y;
    float _678 = 1.0f - _677;
    float _680 = _673.z;
    float _681 = 1.0f - _680;
    float2 _698 = ((((((float2(asfloat((_216 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_224 + (_224 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _675) * _678) * _681) + (((float2(asfloat((_278 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_286 + (_286 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _674) * _678) * _681)) + (((float2(asfloat((_340 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_348 + (_348 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _675) * _677) * _681)) + (((float2(asfloat((_402 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_410 + (_410 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _674) * _677) * _681)) + (((float2(asfloat((_464 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_472 + (_472 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _675) * _678) * _680);
    float2 _713 = clamp(((((_698 + (((float2(asfloat((_526 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_534 + (_534 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _674) * _678) * _680)) + (((float2(asfloat((_588 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_596 + (_596 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _675) * _677) * _680)) + (((float2(asfloat((_650 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_658 + (_658 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _674) * _677) * _680)) - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 _718 = _170 + (float3(_713.x, 0.0f, _713.y) * 0.0599999986588954925537109375f);
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_718, 1.0f));
    out_var_C0 = in_var_Vertex_Color;
    out_var_C1 = mul(_165, float4(in_var_Vertex_Normal, 0.0f));
    out_var_C2 = _718;
    out_var_C3 = in_var_Vertex_UV;
    out_var_C4 = in_var_is_metal_part;
    out_var_C5 = nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52);
    out_var_C6 = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 48);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_is_metal_part = stage_input.in_var_is_metal_part;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
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

static const uint2 _75[4] = { uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u) };

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
    int bool_render_shadows : packoffset(c0.y);
};


static float4 gl_FragCoord;
static uint in_var_C6;

struct SPIRV_Cross_Input
{
    [[vk::location(6)]] nointerpolation uint in_var_C6 : TEXCOORD6;
    float4 gl_FragCoord : SV_Position;
};

void frag_main()
{
    bool _85;
    if (in_var_C6 != 0u)
    {
        _85 = bool_render_shadows != 0;
    }
    else
    {
        _85 = false;
    }
    if (_85)
    {
        uint2 _90 = uint2(gl_FragCoord.xy) & uint2(1u, 1u);
        uint _93 = frame_constants_frame_index & 3u;
        if (all(bool2(_90.x != _75[_93].x, _90.y != _75[_93].y)))
        {
            discard;
        }
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C6 = stage_input.in_var_C6;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
