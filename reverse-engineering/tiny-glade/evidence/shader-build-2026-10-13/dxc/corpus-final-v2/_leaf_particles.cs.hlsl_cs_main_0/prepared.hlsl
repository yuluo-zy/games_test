#line 1 "shaders/leaf_particles.cs.hlsl"








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
#line 9 "shaders/leaf_particles.cs.hlsl"

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
#line 10 "shaders/leaf_particles.cs.hlsl"

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
#line 11 "shaders/leaf_particles.cs.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 12 "shaders/leaf_particles.cs.hlsl"

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
#line 13 "shaders/leaf_particles.cs.hlsl"

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
#line 14 "shaders/leaf_particles.cs.hlsl"

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
#line 15 "shaders/leaf_particles.cs.hlsl"

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
#line 16 "shaders/leaf_particles.cs.hlsl"

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
#line 17 "shaders/leaf_particles.cs.hlsl"

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
#line 18 "shaders/leaf_particles.cs.hlsl"

#line 1 "shaders/water/water_object_push.inc.hlsl"





















#ifndef TINYGLADE_WATER_OBJECT_PUSH_HLSL
#define TINYGLADE_WATER_OBJECT_PUSH_HLSL

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
#line 25 "shaders/water/water_object_push.inc.hlsl"

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
#line 26 "shaders/water/water_object_push.inc.hlsl"

#line 27 "shaders/water/water_object_push.inc.hlsl"

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
#line 28 "shaders/water/water_object_push.inc.hlsl"

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
#line 29 "shaders/water/water_object_push.inc.hlsl"




static const float WATER_PUSH_SCALE[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float WATER_PUSH_WEIGHT[4] = { 2.0f, 2.0f, 0.5f, 0.2f };
static const uint WATER_PUSH_SEED[4] = { 17u, 18u, 20u, 21u };






float3 water_object_push(float2 origin_xz, float time)
{
    if (glade_theme_id == 4u)
    {
        return 0.0f;
    }
    float2 jitter = 0.0f;
    for (uint oct = 0u; oct < 4u; oct++)
    {
        float seed_f = float(WATER_PUSH_SEED[oct]);
        int cell = int(floor(seed_f));
        float f = frac(seed_f);
        uint c0 = uint(cell);
        uint c1 = uint(cell + 1);
        float u = f * f * (3.0f - 2.0f * f);
        float2 a = float2(incb_perlin_hash01(hash(uint2(c0, c0))), rand01(hash(c0 ^ (c0 >> 18u))));
        float2 b = float2(incb_perlin_hash01(hash(uint2(c1, c1))), rand01(hash(c1 ^ (c1 >> 18u))));
        jitter += (normalize(lerp(a, b, u) - 0.5f) * time * 3.0f) * WATER_PUSH_WEIGHT[oct];
    }
    return float3(jitter, 0.0f);
}

#endif 
#line 19 "shaders/leaf_particles.cs.hlsl"



Texture2D water_noise_texture : register(t15);
#line 1 "shaders/water/waves.inc.hlsl"


















#ifndef TINYGLADE_WATER_WAVES_HLSL
#define TINYGLADE_WATER_WAVES_HLSL

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
#line 22 "shaders/water/waves.inc.hlsl"

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
#line 23 "shaders/water/waves.inc.hlsl"




static const float WAVE_SCALE[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float WAVE_WEIGHT[4] = { 2.0f, 2.0f, 0.5f, 0.2f };
static const uint WAVE_SEED[4] = { 17u, 18u, 20u, 21u };




float2 waves_hash01_2(uint cell)
{
    uint y = cell;
    y += (y << 10u);
    y ^= (y >> 6u);
    y += (y << 3u);
    y ^= (y >> 11u);
    uint h = ((cell * 1664525u) + (y + (y << 15u))) + 1013904223u;
    h *= 1664525u;
    h ^= (h >> 11u);
    h ^= (h << 7u) & 2636928640u;
    h ^= (h << 15u) & 4022730752u;
    uint hx = h ^ (h >> 18u);
    uint hy = hx;
    hy += (hy << 10u);
    hy ^= (hy >> 6u);
    hy += (hy << 3u);
    hy ^= (hy >> 11u);
    return float2(asfloat((hx & 0x7FFFFFu) | 0x3F800000u) - 1.0f,
                  asfloat(((hy + (hy << 15u)) & 0x7FFFFFu) | 0x3F800000u) - 1.0f);
}




float2 water_wave_offset(uint octave, float time)
{
    float seed = float(WAVE_SEED[octave]);
    int cell = int(floor(seed));
    float f = frac(seed);
    uint c0 = uint(cell);
    uint c1 = uint(cell + 1);
    float u = f * f * (3.0f - 2.0f * f);
    float2 d0 = waves_hash01_2(c0);
    float2 d1 = waves_hash01_2(c1);
    return (normalize(lerp(d0, d1, u)) - 0.5f) * time * 3.0f;
}


float3 water_wave_displacement(float3 world_xz, float time)
{
    float3 acc = 0.0f;
    for (uint i = 0u; i < 4u; ++i)
    {
        float2 r = water_wave_offset(i, time);
        float angle = float(i + 1u) * 10.16640758514404296875f;
        float2x2 rot = float2x2(float2(cos(angle), sin(angle)), float2(-sin(angle), cos(angle)));
        float3 sample_v = (water_noise_texture.Sample(sampler_llr,
            mul(rot, (world_xz.xz * WAVE_SCALE[i]) + r) * 0.01200000010430812835693359375f).xyz
            * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f);
        acc = float3(acc.x, acc.yz + mul(((sample_v * WAVE_WEIGHT[i])
            * float2(1.0f, WAVE_SCALE[i] * 0.01200000010430812835693359375f).xyy).yz, rot));
    }
    return acc;
}



float waves_ripple_ring(float2 mid, float2 vel, float2 p, float time)
{
    float2 d = mid - p;
    float len_d = length(d);
    float ring = len_d + dot(d, vel);
    float amp = max(len_d, ring);
    return sin((ring * 16.3649997711181640625f) - (time * 6.0f)) / (1.0f + ((amp * amp) * (amp * amp)));
}





float3 water_ripple_displacement(ByteAddressBuffer ripple_source_buffer, float3 world_xz, float time)
{
    float3 acc = 0.0f;
    uint count = min(32u, ripple_source_buffer.Load<uint>(512));
    for (uint i = 0u; i < count; ++i)
    {
        uint4 rec = ripple_source_buffer.Load<uint4>(i * 16u + 0u);
        float2 a = float2(f16tof32(rec.x & 0xFFFFu), f16tof32((rec.x >> 16u) & 0xFFFFu));
        float2 b = float2(f16tof32(rec.y & 0xFFFFu), f16tof32((rec.y >> 16u) & 0xFFFFu));
        float2 mid = lerp(a, b, 0.5f);
        float2 vel = (b - a) * 3.0f;
        float w1 = waves_ripple_ring(mid, vel, world_xz.xz, time);
        float w2 = waves_ripple_ring(mid, vel, world_xz.xz - float2(0.00999999977648258209228515625f, 0.0f), time);
        float w3 = waves_ripple_ring(mid, vel, world_xz.xz - float2(0.0f, 0.00999999977648258209228515625f), time);
        acc += float3(w1, (w2 - w1) * 0.5f, (w3 - w1) * 0.5f) * asfloat(rec.z);
    }
    return acc;
}

#endif 
#line 23 "shaders/leaf_particles.cs.hlsl"

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
#line 24 "shaders/leaf_particles.cs.hlsl"


struct LeafParticle
{
    float3 position;
    float rotation;
    float scale;
    float3 up_axis;
};

struct ParticleEmitter
{
    float3 position;
    float pad0;
};

uint2 water_noise_texture_dims;
uint2 grass_object_push_dims;
int particle_emitter_count;
int total_particle_count;

RWByteAddressBuffer __alias_leaf_particle_buffer : register(u17);
RWByteAddressBuffer __alias_particle_emitter_buffer : register(u18);

Texture2D grass_object_push : register(t16);




Texture2D unused_t13 : register(t13);

static const float leaf_octave_scales[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float leaf_octave_weights[4] = { 2.0f, 2.0f, 0.5f, 0.2f };
static const uint leaf_octave_seeds[4] = { 17u, 18u, 20u, 21u };


float leaf_noise(float2 p)
{
    return perlin(p);
}

[numthreads(64, 1, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    if (gl_GlobalInvocationID.x >= uint(total_particle_count))
    {
        return;
    }

    uint stride = gl_GlobalInvocationID.x * 32u;
    LeafParticle p;
    p.position = __alias_leaf_particle_buffer.Load<float3>(stride + 0u);
    p.rotation = __alias_leaf_particle_buffer.Load<float>(stride + 12u);
    p.scale = __alias_leaf_particle_buffer.Load<float>(stride + 16u);
    p.up_axis = __alias_leaf_particle_buffer.Load<float3>(stride + 20u);

    if (particle_emitter_count == 0)
    {
        __alias_leaf_particle_buffer.Store<float3>(stride + 0u, float3(0.0f, 0.0f, 0.0f));
        __alias_leaf_particle_buffer.Store<float>(stride + 12u, p.rotation);
        __alias_leaf_particle_buffer.Store<float>(stride + 16u, 0.0f);
        __alias_leaf_particle_buffer.Store<float3>(stride + 20u, p.up_axis);
        return;
    }

    float3 out_pos;
    float out_rotation;
    float out_scale;
    float3 out_up;

    if (p.position.y > 0.05f)
    {
        
        float t = float(frame_index);
        float3 dir = normalize(p.position);
        float gust_x = (sin((t * 0.058823529f) + p.position.x) * sin((t * 0.025641026f) + p.position.z)) + 1.0f;
        float3 drift = (dir * 0.3f)
                     + ((float3(1.0f, 0.0f, 0.0f) * gust_x * 2.0f) * 0.4f) * ((dir.x + 1.0f) * 0.5f);

        float2 noise_uv = ((p.position.xy + p.position.zy) + float(gl_GlobalInvocationID.x).xx) * 0.2f;
        float nx = leaf_noise(noise_uv + 234.0f);
        float ny = leaf_noise(noise_uv + 452.0f);
        float nz = leaf_noise(noise_uv + 99452.0f);

        float3 vel = -float3(drift.x + lerp(-0.5f, 0.5f, nx),
                             lerp(0.4f, 3.0f, ny * ny) * 0.7f,
                             drift.z + lerp(-0.5f, 0.5f, nz));
        float3 disp = vel * delta_time_seconds * 1.5f;

        float3 moved = p.position + disp;
        moved.y = max(moved.y, 0.05f);

        out_pos = moved;
        out_rotation = p.rotation + (length(disp) * 3.0f) * lerp(0.5f, 1.0f, rand01(hash(gl_GlobalInvocationID.x ^ 345346u)));
        out_scale = p.scale;
        out_up = p.up_axis;
    }
    else
    {
        
        float ground_y = p.position.y - (delta_time_seconds * 0.0125f);
        float3 ground_pos = p.position;
        ground_pos.y = ground_y;
        float4 grass = grass_object_push.SampleLevel(sampler_default, (ground_pos.xz * 0.0076923f) + 0.5f, 0.0f);

        float time = float(game_time_sec) + game_time_frac;

        if (glade_theme_id == 4u)
        {
            out_pos = ground_pos;
            out_rotation = p.rotation;
            out_scale = p.scale;
            out_up = p.up_axis;
        }
        else
        {
            
            float2 wind_dir[4];
            [unroll]
            for (uint o = 0u; o < 4u; o++)
            {
                float2 seed_p = float2(leaf_octave_seeds[o], leaf_octave_seeds[o]);
                int2 cell = int2(floor(seed_p));
                float2 f = frac(seed_p);
                float2 u = f * f * (3.0f - 2.0f * f);
                float n00 = incb_perlin_hash01(hash(uint2(cell + int2(0, 0))));
                float n10 = incb_perlin_hash01(hash(uint2(cell + int2(1, 0))));
                float n01 = incb_perlin_hash01(hash(uint2(cell + int2(0, 1))));
                float n11 = incb_perlin_hash01(hash(uint2(cell + int2(1, 1))));
                float2 noise_dir = float2(lerp(n00, n10, u.x), lerp(n01, n11, u.x));
                wind_dir[o] = normalize(noise_dir - 0.5f) * time * 3.0f;
            }

            
            float3 flow = float3(0.0f, 0.0f, 0.0f);
            [unroll]
            for (uint i = 0u; i < 1u; i++)
            {
                float ang = float(i + 1u) * 10.16640758f;
                float c = cos(ang);
                float s = sin(ang);
                float2x2 rot = float2x2(c, s, -s, c);
                float2 sample_uv = mul(rot, (ground_pos.xz * leaf_octave_scales[i]) + wind_dir[i]) * 0.012f;
                float3 v = water_noise_texture.SampleLevel(sampler_llr, sample_uv, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)
                         + float3(-0.5f, -1.0f, -1.0f);
                v *= leaf_octave_weights[i];
                float3 v_scaled = v * float3(1.0f, leaf_octave_scales[i] * 0.012f, leaf_octave_scales[i] * 0.012f);
                float2 move = mul(v_scaled.yz, rot);
                flow.yz += move;
                flow = float3(flow.x, flow.y, flow.z);
            }

            float2 wind_disp = ground_pos.xz + ((((float3(grass.x, 0.0f, grass.y) * float(grass.z > 0.0f)) * 0.35f)
                           + (flow.yxz * float3(2.0f, 0.01f, 2.0f))).xz * 0.1f);
            float3 settled = float3(wind_disp.x, ground_y, wind_disp.y);

            if (ground_y < -0.03f)
            {
                uint respawn_idx = (gl_GlobalInvocationID.x ^ (345345u * (frame_index + 1u))) % uint(particle_emitter_count);
                float3 emitter = __alias_particle_emitter_buffer.Load<float3>(respawn_idx * 16u + 0u);
                float3 jitter = float3(lerp(-0.5f, 0.5f, rand01(hash(gl_GlobalInvocationID.x ^ 10945u))),
                                       lerp(-0.5f, 0.5f, rand01(hash(gl_GlobalInvocationID.x ^ 49234u))),
                                       lerp(-0.5f, 0.5f, rand01(hash(gl_GlobalInvocationID.x ^ 23458u))));
                out_pos = emitter + jitter;
                out_rotation = p.rotation;
                out_scale = lerp(0.6f, 1.0f, rand01(hash(gl_GlobalInvocationID.x))) * 0.8f;
                out_up = normalize(float3(lerp(-1.0f, 1.0f, rand01(hash(gl_GlobalInvocationID.x ^ 2345346u))),
                                          2.0f,
                                          lerp(-1.0f, 1.0f, rand01(hash(gl_GlobalInvocationID.x ^ 10945u)))));
            }
            else
            {
                out_pos = settled;
                out_rotation = p.rotation;
                out_scale = p.scale - (delta_time_seconds * 0.05f);
                out_up = float3(0.0f, 1.0f, 0.0f);
            }
        }
    }

    __alias_leaf_particle_buffer.Store<float3>(stride + 0u, out_pos);
    __alias_leaf_particle_buffer.Store<float>(stride + 12u, out_rotation);
    __alias_leaf_particle_buffer.Store<float>(stride + 16u, out_scale);
    __alias_leaf_particle_buffer.Store<float3>(stride + 20u, out_up);
}
