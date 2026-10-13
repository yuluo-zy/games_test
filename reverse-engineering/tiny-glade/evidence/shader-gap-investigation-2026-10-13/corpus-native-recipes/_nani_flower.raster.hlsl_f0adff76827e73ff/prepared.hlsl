#define SHADER_VARIANT_DEPTH_ONLY 1
#define TGV_f0adff76827e73ff 1
#line 1 "shaders/nani_flower.raster.hlsl"


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
#line 9 "shaders/nani_flower.raster.hlsl"

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
#line 10 "shaders/nani_flower.raster.hlsl"

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
#line 11 "shaders/nani_flower.raster.hlsl"

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
#line 12 "shaders/nani_flower.raster.hlsl"

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
#line 13 "shaders/nani_flower.raster.hlsl"

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
#line 14 "shaders/nani_flower.raster.hlsl"

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
#line 15 "shaders/nani_flower.raster.hlsl"

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
#line 16 "shaders/nani_flower.raster.hlsl"

#line 1 "shaders/garden/defs.inc.hlsl"


#ifndef TINYGLADE_GARDEN_DEFS_HLSL
#define TINYGLADE_GARDEN_DEFS_HLSL



struct BushXform
{
    Affine3Packed transform;
};

struct FlowerXform
{
    Affine3Packed transform;
    float3 color;
    uint flower_type;
};

#endif 
#line 17 "shaders/nani_flower.raster.hlsl"

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
#line 18 "shaders/nani_flower.raster.hlsl"

#line 1 "shaders/inc/wind.hlsl"


#ifndef TINYGLADE_INC_WIND_HLSL
#define TINYGLADE_INC_WIND_HLSL



float wind_noise(float2 p)
{
    uint2 ip = uint2(int2(p * 0.5));
    float2 fp = frac(p * 0.5);
    float2 u = fp * fp * (3.0 - 2.0 * fp);
    uint h00 = (ip.x * 374761393u) + (ip.y * 668265263u);
    uint h01 = ((ip.x + 1u) * 374761393u) + (ip.y * 668265263u);
    uint h10 = (ip.x * 374761393u) + ((ip.y + 1u) * 668265263u);
    uint h11 = ((ip.x + 1u) * 374761393u) + ((ip.y + 1u) * 668265263u);
    float n00 = float(h00 & 0x00FFFFFFu) / 16777215.0;
    float n01 = float(h01 & 0x00FFFFFFu) / 16777215.0;
    float n10 = float(h10 & 0x00FFFFFFu) / 16777215.0;
    float n11 = float(h11 & 0x00FFFFFFu) / 16777215.0;
    return lerp(lerp(n00, n01, u.x), lerp(n10, n11, u.x), u.y) * 2.0 - 1.0;
}




float3 evaluate_wind(float3 world_pos, float seed, float amplitude)
{
    float t = game_time_sec + game_time_frac;
    float2 xz = world_pos.xz;
    float gust = wind_noise(xz * 0.02 + float2(t * 0.03, 0.0) + float2(seed * 13.0, 0.0));
    float flutter = wind_noise(xz * 0.3 + float2(t * 0.5, seed) );
    float bend = saturate(gust * 0.5 + 0.5) * amplitude;
    
    float3 disp = float3(-gust * bend, flutter * 0.05 * amplitude, 0.0);
    disp *= smoothstep(0.0, 1.0, max(world_pos.y, 0.0));
    return disp;
}

#endif 
#line 19 "shaders/nani_flower.raster.hlsl"

#line 1 "shaders/inc/material/plant_material.inc.hlsl"


#ifndef TINYGLADE_INC_MATERIAL_PLANT_MATERIAL_HLSL
#define TINYGLADE_INC_MATERIAL_PLANT_MATERIAL_HLSL

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
#line 6 "shaders/inc/material/plant_material.inc.hlsl"



struct PlantMaterialParams {
    float3 base_color;
    float3 flower_color;
    float roughness;
    float translucency;
    uint is_flower;
};

PlantMaterialParams init_plant_material(float3 base_color, float3 flower_color, float roughness, uint is_flower)
{
    PlantMaterialParams p;
    p.base_color = base_color;
    p.flower_color = flower_color;
    p.roughness = saturate(roughness);
    p.translucency = 1.0;
    p.is_flower = is_flower;
    return p;
}

#endif 
#line 20 "shaders/nani_flower.raster.hlsl"

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
#line 21 "shaders/nani_flower.raster.hlsl"

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
#line 22 "shaders/nani_flower.raster.hlsl"

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
#line 23 "shaders/nani_flower.raster.hlsl"

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
#line 24 "shaders/nani_flower.raster.hlsl"

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
#line 25 "shaders/nani_flower.raster.hlsl"

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
#line 26 "shaders/nani_flower.raster.hlsl"

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
#line 27 "shaders/nani_flower.raster.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 28 "shaders/nani_flower.raster.hlsl"

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
#line 29 "shaders/nani_flower.raster.hlsl"

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
#line 30 "shaders/nani_flower.raster.hlsl"

#line 1 "shaders/inc/sh.hlsl"










float4 sh_basis(float3 dir)
{
    return float4(0.2820949852466583251953125f, 0.48860299587249755859375f * dir);
}

float sh_eval(float4 sh, float3 dir)
{
    return dot(sh, sh_basis(dir));
}
#line 31 "shaders/nani_flower.raster.hlsl"


uint2 grass_object_push_dims;
int bool_render_shadows;

StructuredBuffer<uint> instance_indirection_buffer : register(t14);
StructuredBuffer<FlowerXform> nani_instance_data : register(t15);
Texture2D<float4> grass_object_push : register(t16);

uint hash2(uint2 p)
{
    uint y = p.y;
    y += (y << 10u);
    y ^= (y >> 6u);
    y += (y << 3u);
    y ^= (y >> 11u);
    p.x = (p.x * 1664525u) + (y + (y << 15u)) + 1013904223u;
    p.x *= 1664525u;
    uint x = p.x ^ (p.x >> 11u);
    x = x ^ ((x << 7u) & 2636928640u);
    x = x ^ ((x << 15u) & 4022730752u);
    return x;
}

float hash2_rand(uint h)
{
    return asfloat(((h ^ (h >> 18u)) & 8388607u) | 1065353216u) - 1.0f;
}

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float wind_bend: wind_bend;
    [[vk::location(4)]] float3 autumn_color: autumn_color;
    [[vk::location(5)]] float3 flowery_color: flowery_color;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    uint idx = instance_indirection_buffer[vin.instance_id];
    Affine3Packed packed = nani_instance_data[idx].transform;
    float3x4 xform = transpose(float4x3(float3(packed.data[0], packed.data[1], packed.data[2]),
                                        float3(packed.data[3], packed.data[4], packed.data[5]),
                                        float3(packed.data[6], packed.data[7], packed.data[8]),
                                        float3(packed.data[9], packed.data[10], packed.data[11])));
    float3 origin = mul(xform, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 world = mul(xform, float4(vin.Vertex_Position, 1.0f));

    uint origin_hash = uint((origin.x * 20.0f) - (origin.z * 20.0f) + 2000.0f);
    origin_hash = hash(origin_hash);
    float color_rand = hash2_rand(origin_hash);

    bool theme_a = glade_theme_id == 4u;
    bool theme_b = glade_theme_id == 1u;
    float3 base_color;
    float color_var;
    if (theme_a)
    {
        base_color = lerp(vin.autumn_color, 0.7f.xxx, 0.5f.xxx);
        color_var = color_rand * 0.5f;
    }
    else if (theme_b)
    {
        base_color = vin.flowery_color;
        color_var = color_rand * 0.5f;
    }
    else if (glade_theme_id == 3u && nani_instance_data[idx].flower_type == 0u)
    {
        base_color = vin.Vertex_Color.z > 0.2f
            ? float3(0.21f, 0.21f, 0.35f)
            : vin.Vertex_Color;
        color_var = color_rand;
    }
    else
    {
        base_color = vin.Vertex_Color;
        color_var = color_rand;
    }

    float3 shade_color;
    if (nani_instance_data[idx].color.x > -1.0f)
    {
        shade_color = theme_a
            ? float3(0.207063f, 0.164317f, 0.291863f)
            : (theme_b
                ? float3(0.310594f, 0.246475f, 0.437794f)
                : (glade_theme_id == 3u
                    ? float3(0.310594f, 0.246475f, 0.437794f)
                    : float3(0.126791f, 0.305342f, 0.495884f)));
    }
    else
    {
        shade_color = base_color;
    }

    float2 wind_uv = (origin.xz * 0.1f) + float2(game_time_sec + game_time_frac, 0.0f) * 1.2f;
    int2 wind_cell = int2(floor(wind_uv));
    float2 wind_frac = frac(wind_uv);
    float2 wind_smooth = wind_frac * wind_frac * (3.0f.xx - (wind_frac * 2.0f));
    float n00 = hash2_rand(hash2(uint2(wind_cell))) * (1.0f - wind_smooth.x) * (1.0f - wind_smooth.y);
    float n10 = hash2_rand(hash2(uint2(wind_cell + int2(1, 0)))) * wind_smooth.x * (1.0f - wind_smooth.y);
    float n01 = hash2_rand(hash2(uint2(wind_cell + int2(0, 1)))) * (1.0f - wind_smooth.x) * wind_smooth.y;
    float n11 = hash2_rand(hash2(uint2(wind_cell + int2(1, 1)))) * wind_smooth.x * wind_smooth.y;
    float wind_noise = (n00 + n10) + (n01 + n11);
    world.x -= (vin.wind_bend * vin.wind_bend) * (sin((wind_noise * 3.14f) - 1.57f + 0.3f) * 0.0735f);

    float4 grass_push = grass_object_push.SampleLevel(sampler_default, (origin.xz * 0.0076923077f) + 0.5f.xx, 0.0f);
    float push_amount = clamp(grass_push.z, 0.0f, 1.0f);
    float3 displaced = world;
    if (push_amount > 0.0f)
    {
        float2 displaced_xz = world.xz + ((grass_push.xy * push_amount) * vin.wind_bend);
        displaced = float3(displaced_xz.x, world.y, displaced_xz.y);
    }

    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(displaced, 1.0f));
    o.C0 = pow(shade_color, lerp(1.0f, 2.5f, color_var).xxx);
    o.C1 = mul(xform, float4(vin.Vertex_Normal, 0.0f));
    o.C2 = displaced;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float3 normal = normalize(pin.C1);
    float3 albedo;
    if (glade_theme_id == 4u)
    {
        albedo = lerp(pin.C0, 0.7f.xxx, smoothstep(0.5f, 1.0f, clamp(normal.y, 0.0f, 1.0f) * 0.8f));
    }
    else
    {
        albedo = pin.C0;
    }
    float3 packed_normal = (normal * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = uint(clamp(srgb_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(srgb_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(srgb_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 855638016u;
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 1711276032u;
    o.gbuffer = uint2(normal_word, albedo_word);
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

struct FlowerXform
{
    Affine3Packed transform;
    float3 color;
    uint flower_type;
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
    uint _Globals_current_view_index : packoffset(c0);
    uint2 _Globals_grass_object_push_dims : packoffset(c0.y);
    int _Globals_bool_render_shadows : packoffset(c0.w);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> grass_object_push : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float in_var_wind_bend;
static float3 in_var_autumn_color;
static float3 in_var_flowery_color;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float in_var_wind_bend: wind_bend;
    [[vk::location(4)]] float3 in_var_autumn_color: autumn_color;
    [[vk::location(5)]] float3 in_var_flowery_color: flowery_color;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    Affine3Packed _174;
    [unroll]
    for (int _1ident = 0; _1ident < 12; _1ident++)
    {
        _174.data[_1ident] = nani_instance_data.Load<float>(_1ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0);
    }
    float3x4 _193 = transpose(float4x3(float3(_174.data[0], _174.data[1], _174.data[2]), float3(_174.data[3], _174.data[4], _174.data[5]), float3(_174.data[6], _174.data[7], _174.data[8]), float3(_174.data[9], _174.data[10], _174.data[11])));
    float3 _194 = mul(_193, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 _199 = mul(_193, float4(in_var_Vertex_Position, 1.0f));
    uint _206 = uint(((_194.x * 20.0f) - (_194.z * 20.0f)) + 2000.0f);
    uint _208 = _206 + (_206 << 10u);
    uint _210 = _208 ^ (_208 >> 6u);
    uint _212 = _210 + (_210 << 3u);
    uint _214 = _212 ^ (_212 >> 11u);
    float _220 = asfloat(((_214 + (_214 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    bool _224 = !(frame_constants_glade_theme_id == 2u);
    bool _228;
    if (_224)
    {
        _228 = frame_constants_glade_theme_id == 4u;
    }
    else
    {
        _228 = true;
    }
    float3 _259;
    float _260;
    if (_228)
    {
        float3 _257;
        if (frame_constants_glade_theme_id == 4u)
        {
            _257 = lerp(in_var_autumn_color, 0.699999988079071044921875f.xxx, 0.5f.xxx);
        }
        else
        {
            _257 = in_var_autumn_color;
        }
        _259 = _257;
        _260 = _220 * 0.5f;
    }
    else
    {
        float3 _251;
        float _252;
        if (frame_constants_glade_theme_id == 1u)
        {
            _251 = in_var_flowery_color;
            _252 = _220 * 0.5f;
        }
        else
        {
            bool _242;
            if (nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 60) == 0u)
            {
                _242 = frame_constants_glade_theme_id == 3u;
            }
            else
            {
                _242 = false;
            }
            float3 _249;
            if (_242)
            {
                bool3 _247 = (in_var_Vertex_Color.z > 0.20000000298023223876953125f).xxx;
                _249 = float3(_247.x ? float3(0.2100000083446502685546875f, 0.2100000083446502685546875f, 0.3499999940395355224609375f).x : in_var_Vertex_Color.x, _247.y ? float3(0.2100000083446502685546875f, 0.2100000083446502685546875f, 0.3499999940395355224609375f).y : in_var_Vertex_Color.y, _247.z ? float3(0.2100000083446502685546875f, 0.2100000083446502685546875f, 0.3499999940395355224609375f).z : in_var_Vertex_Color.z);
            }
            else
            {
                _249 = in_var_Vertex_Color;
            }
            _251 = _249;
            _252 = _220;
        }
        _259 = _251;
        _260 = _252;
    }
    float3 _282;
    if (nani_instance_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 48) > (-1.0f))
    {
        bool _269;
        if (_224)
        {
            _269 = frame_constants_glade_theme_id == 4u;
        }
        else
        {
            _269 = true;
        }
        float3 _281;
        if (_269)
        {
            _281 = float3(0.207062780857086181640625f, 0.16431677341461181640625f, 0.291862964630126953125f);
        }
        else
        {
            float3 _280;
            if (frame_constants_glade_theme_id == 1u)
            {
                _280 = float3(0.3105941712856292724609375f, 0.246475160121917724609375f, 0.4377944469451904296875f);
            }
            else
            {
                bool3 _278 = (frame_constants_glade_theme_id == 3u).xxx;
                _280 = float3(_278.x ? float3(0.3105941712856292724609375f, 0.246475160121917724609375f, 0.4377944469451904296875f).x : float3(0.12679146230220794677734375f, 0.30534231662750244140625f, 0.4958840310573577880859375f).x, _278.y ? float3(0.3105941712856292724609375f, 0.246475160121917724609375f, 0.4377944469451904296875f).y : float3(0.12679146230220794677734375f, 0.30534231662750244140625f, 0.4958840310573577880859375f).y, _278.z ? float3(0.3105941712856292724609375f, 0.246475160121917724609375f, 0.4377944469451904296875f).z : float3(0.12679146230220794677734375f, 0.30534231662750244140625f, 0.4958840310573577880859375f).z);
            }
            _281 = _280;
        }
        _282 = _281;
    }
    else
    {
        _282 = _259;
    }
    float2 _283 = _194.xz;
    float2 _293 = (_283 * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 _295 = int2(floor(_293));
    float2 _296 = frac(_293);
    uint2 _297 = uint2(_295);
    uint _299 = _297.y;
    uint _301 = _299 + (_299 << 10u);
    uint _303 = _301 ^ (_301 >> 6u);
    uint _305 = _303 + (_303 << 3u);
    uint _307 = _305 ^ (_305 >> 11u);
    uint _313 = (((_297.x * 1664525u) + (_307 + (_307 << 15u))) + 1013904223u) * 1664525u;
    uint _315 = _313 ^ (_313 >> 11u);
    uint _318 = _315 ^ ((_315 << 7u) & 2636928640u);
    uint _321 = _318 ^ ((_318 << 15u) & 4022730752u);
    uint2 _329 = uint2(_295 + int2(1, 0));
    uint _331 = _329.y;
    uint _333 = _331 + (_331 << 10u);
    uint _335 = _333 ^ (_333 >> 6u);
    uint _337 = _335 + (_335 << 3u);
    uint _339 = _337 ^ (_337 >> 11u);
    uint _345 = (((_329.x * 1664525u) + (_339 + (_339 << 15u))) + 1013904223u) * 1664525u;
    uint _347 = _345 ^ (_345 >> 11u);
    uint _350 = _347 ^ ((_347 << 7u) & 2636928640u);
    uint _353 = _350 ^ ((_350 << 15u) & 4022730752u);
    uint2 _361 = uint2(_295 + int2(0, 1));
    uint _363 = _361.y;
    uint _365 = _363 + (_363 << 10u);
    uint _367 = _365 ^ (_365 >> 6u);
    uint _369 = _367 + (_367 << 3u);
    uint _371 = _369 ^ (_369 >> 11u);
    uint _377 = (((_361.x * 1664525u) + (_371 + (_371 << 15u))) + 1013904223u) * 1664525u;
    uint _379 = _377 ^ (_377 >> 11u);
    uint _382 = _379 ^ ((_379 << 7u) & 2636928640u);
    uint _385 = _382 ^ ((_382 << 15u) & 4022730752u);
    uint2 _393 = uint2(_295 + int2(1, 1));
    uint _395 = _393.y;
    uint _397 = _395 + (_395 << 10u);
    uint _399 = _397 ^ (_397 >> 6u);
    uint _401 = _399 + (_399 << 3u);
    uint _403 = _401 ^ (_401 >> 11u);
    uint _409 = (((_393.x * 1664525u) + (_403 + (_403 << 15u))) + 1013904223u) * 1664525u;
    uint _411 = _409 ^ (_409 >> 11u);
    uint _414 = _411 ^ ((_411 << 7u) & 2636928640u);
    uint _417 = _414 ^ ((_414 << 15u) & 4022730752u);
    float2 _427 = (_296 * _296) * (3.0f.xx - (_296 * 2.0f));
    float _428 = _427.x;
    float _429 = 1.0f - _428;
    float _431 = _427.y;
    _199.x = _199.x - (((in_var_wind_bend * in_var_wind_bend) * (sin((((((1.0f - _431) * (((asfloat(((_321 ^ (_321 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _429) + ((asfloat(((_353 ^ (_353 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _428))) + (((asfloat(((_385 ^ (_385 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _429) * _431)) + (((asfloat(((_417 ^ (_417 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _428) * _431)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.073499999940395355224609375f);
    float4 _457 = grass_object_push.SampleLevel(sampler_default, (_283 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    float _459 = clamp(_457.z, 0.0f, 1.0f);
    float3 _469;
    if (_459 > 0.0f)
    {
        float2 _467 = _199.xz + ((_457.xy * _459) * in_var_wind_bend);
        _469 = float3(_467.x, _199.y, _467.y);
    }
    else
    {
        _469 = _199;
    }
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_469, 1.0f));
    out_var_C0 = pow(_282, lerp(1.0f, 2.5f, _260).xxx);
    out_var_C1 = mul(_193, float4(in_var_Vertex_Normal, 0.0f));
    out_var_C2 = _469;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_wind_bend = stage_input.in_var_wind_bend;
    in_var_autumn_color = stage_input.in_var_autumn_color;
    in_var_flowery_color = stage_input.in_var_flowery_color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
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

struct FlowerXform
{
    Affine3Packed transform;
    float3 color;
    uint flower_type;
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
    uint _Globals_current_view_index : packoffset(c0);
    uint2 _Globals_grass_object_push_dims : packoffset(c0.y);
    int _Globals_bool_render_shadows : packoffset(c0.w);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> grass_object_push : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float in_var_wind_bend;
static float3 in_var_autumn_color;
static float3 in_var_flowery_color;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float in_var_wind_bend: wind_bend;
    [[vk::location(4)]] float3 in_var_autumn_color: autumn_color;
    [[vk::location(5)]] float3 in_var_flowery_color: flowery_color;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    Affine3Packed _174;
    [unroll]
    for (int _1ident = 0; _1ident < 12; _1ident++)
    {
        _174.data[_1ident] = nani_instance_data.Load<float>(_1ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0);
    }
    float3x4 _193 = transpose(float4x3(float3(_174.data[0], _174.data[1], _174.data[2]), float3(_174.data[3], _174.data[4], _174.data[5]), float3(_174.data[6], _174.data[7], _174.data[8]), float3(_174.data[9], _174.data[10], _174.data[11])));
    float3 _194 = mul(_193, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 _199 = mul(_193, float4(in_var_Vertex_Position, 1.0f));
    uint _206 = uint(((_194.x * 20.0f) - (_194.z * 20.0f)) + 2000.0f);
    uint _208 = _206 + (_206 << 10u);
    uint _210 = _208 ^ (_208 >> 6u);
    uint _212 = _210 + (_210 << 3u);
    uint _214 = _212 ^ (_212 >> 11u);
    float _220 = asfloat(((_214 + (_214 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    bool _224 = !(frame_constants_glade_theme_id == 2u);
    bool _228;
    if (_224)
    {
        _228 = frame_constants_glade_theme_id == 4u;
    }
    else
    {
        _228 = true;
    }
    float3 _259;
    float _260;
    if (_228)
    {
        float3 _257;
        if (frame_constants_glade_theme_id == 4u)
        {
            _257 = lerp(in_var_autumn_color, 0.699999988079071044921875f.xxx, 0.5f.xxx);
        }
        else
        {
            _257 = in_var_autumn_color;
        }
        _259 = _257;
        _260 = _220 * 0.5f;
    }
    else
    {
        float3 _251;
        float _252;
        if (frame_constants_glade_theme_id == 1u)
        {
            _251 = in_var_flowery_color;
            _252 = _220 * 0.5f;
        }
        else
        {
            bool _242;
            if (nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 60) == 0u)
            {
                _242 = frame_constants_glade_theme_id == 3u;
            }
            else
            {
                _242 = false;
            }
            float3 _249;
            if (_242)
            {
                bool3 _247 = (in_var_Vertex_Color.z > 0.20000000298023223876953125f).xxx;
                _249 = float3(_247.x ? float3(0.2100000083446502685546875f, 0.2100000083446502685546875f, 0.3499999940395355224609375f).x : in_var_Vertex_Color.x, _247.y ? float3(0.2100000083446502685546875f, 0.2100000083446502685546875f, 0.3499999940395355224609375f).y : in_var_Vertex_Color.y, _247.z ? float3(0.2100000083446502685546875f, 0.2100000083446502685546875f, 0.3499999940395355224609375f).z : in_var_Vertex_Color.z);
            }
            else
            {
                _249 = in_var_Vertex_Color;
            }
            _251 = _249;
            _252 = _220;
        }
        _259 = _251;
        _260 = _252;
    }
    float3 _282;
    if (nani_instance_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 48) > (-1.0f))
    {
        bool _269;
        if (_224)
        {
            _269 = frame_constants_glade_theme_id == 4u;
        }
        else
        {
            _269 = true;
        }
        float3 _281;
        if (_269)
        {
            _281 = float3(0.207062780857086181640625f, 0.16431677341461181640625f, 0.291862964630126953125f);
        }
        else
        {
            float3 _280;
            if (frame_constants_glade_theme_id == 1u)
            {
                _280 = float3(0.3105941712856292724609375f, 0.246475160121917724609375f, 0.4377944469451904296875f);
            }
            else
            {
                bool3 _278 = (frame_constants_glade_theme_id == 3u).xxx;
                _280 = float3(_278.x ? float3(0.3105941712856292724609375f, 0.246475160121917724609375f, 0.4377944469451904296875f).x : float3(0.12679146230220794677734375f, 0.30534231662750244140625f, 0.4958840310573577880859375f).x, _278.y ? float3(0.3105941712856292724609375f, 0.246475160121917724609375f, 0.4377944469451904296875f).y : float3(0.12679146230220794677734375f, 0.30534231662750244140625f, 0.4958840310573577880859375f).y, _278.z ? float3(0.3105941712856292724609375f, 0.246475160121917724609375f, 0.4377944469451904296875f).z : float3(0.12679146230220794677734375f, 0.30534231662750244140625f, 0.4958840310573577880859375f).z);
            }
            _281 = _280;
        }
        _282 = _281;
    }
    else
    {
        _282 = _259;
    }
    float2 _283 = _194.xz;
    float2 _293 = (_283 * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 _295 = int2(floor(_293));
    float2 _296 = frac(_293);
    uint2 _297 = uint2(_295);
    uint _299 = _297.y;
    uint _301 = _299 + (_299 << 10u);
    uint _303 = _301 ^ (_301 >> 6u);
    uint _305 = _303 + (_303 << 3u);
    uint _307 = _305 ^ (_305 >> 11u);
    uint _313 = (((_297.x * 1664525u) + (_307 + (_307 << 15u))) + 1013904223u) * 1664525u;
    uint _315 = _313 ^ (_313 >> 11u);
    uint _318 = _315 ^ ((_315 << 7u) & 2636928640u);
    uint _321 = _318 ^ ((_318 << 15u) & 4022730752u);
    uint2 _329 = uint2(_295 + int2(1, 0));
    uint _331 = _329.y;
    uint _333 = _331 + (_331 << 10u);
    uint _335 = _333 ^ (_333 >> 6u);
    uint _337 = _335 + (_335 << 3u);
    uint _339 = _337 ^ (_337 >> 11u);
    uint _345 = (((_329.x * 1664525u) + (_339 + (_339 << 15u))) + 1013904223u) * 1664525u;
    uint _347 = _345 ^ (_345 >> 11u);
    uint _350 = _347 ^ ((_347 << 7u) & 2636928640u);
    uint _353 = _350 ^ ((_350 << 15u) & 4022730752u);
    uint2 _361 = uint2(_295 + int2(0, 1));
    uint _363 = _361.y;
    uint _365 = _363 + (_363 << 10u);
    uint _367 = _365 ^ (_365 >> 6u);
    uint _369 = _367 + (_367 << 3u);
    uint _371 = _369 ^ (_369 >> 11u);
    uint _377 = (((_361.x * 1664525u) + (_371 + (_371 << 15u))) + 1013904223u) * 1664525u;
    uint _379 = _377 ^ (_377 >> 11u);
    uint _382 = _379 ^ ((_379 << 7u) & 2636928640u);
    uint _385 = _382 ^ ((_382 << 15u) & 4022730752u);
    uint2 _393 = uint2(_295 + int2(1, 1));
    uint _395 = _393.y;
    uint _397 = _395 + (_395 << 10u);
    uint _399 = _397 ^ (_397 >> 6u);
    uint _401 = _399 + (_399 << 3u);
    uint _403 = _401 ^ (_401 >> 11u);
    uint _409 = (((_393.x * 1664525u) + (_403 + (_403 << 15u))) + 1013904223u) * 1664525u;
    uint _411 = _409 ^ (_409 >> 11u);
    uint _414 = _411 ^ ((_411 << 7u) & 2636928640u);
    uint _417 = _414 ^ ((_414 << 15u) & 4022730752u);
    float2 _427 = (_296 * _296) * (3.0f.xx - (_296 * 2.0f));
    float _428 = _427.x;
    float _429 = 1.0f - _428;
    float _431 = _427.y;
    _199.x = _199.x - (((in_var_wind_bend * in_var_wind_bend) * (sin((((((1.0f - _431) * (((asfloat(((_321 ^ (_321 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _429) + ((asfloat(((_353 ^ (_353 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _428))) + (((asfloat(((_385 ^ (_385 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _429) * _431)) + (((asfloat(((_417 ^ (_417 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _428) * _431)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.073499999940395355224609375f);
    float4 _457 = grass_object_push.SampleLevel(sampler_default, (_283 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    float _459 = clamp(_457.z, 0.0f, 1.0f);
    float3 _469;
    if (_459 > 0.0f)
    {
        float2 _467 = _199.xz + ((_457.xy * _459) * in_var_wind_bend);
        _469 = float3(_467.x, _199.y, _467.y);
    }
    else
    {
        _469 = _199;
    }
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_469, 1.0f));
    out_var_C0 = pow(_282, lerp(1.0f, 2.5f, _260).xxx);
    out_var_C1 = mul(_193, float4(in_var_Vertex_Normal, 0.0f));
    out_var_C2 = _469;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_wind_bend = stage_input.in_var_wind_bend;
    in_var_autumn_color = stage_input.in_var_autumn_color;
    in_var_flowery_color = stage_input.in_var_flowery_color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
void frag_main()
{
}

void ps_main()
{
    frag_main();
}
#pragma dxc diagnostic pop
#endif
