#define SHADER_VARIANT_DEPTH_ONLY 0
#define TGV_b903e43ffb3da915 1
#line 1 "shaders/lilypad.raster.hlsl"


#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))


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
#line 13 "shaders/lilypad.raster.hlsl"

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
#line 14 "shaders/lilypad.raster.hlsl"

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
#line 15 "shaders/lilypad.raster.hlsl"

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
#line 16 "shaders/lilypad.raster.hlsl"

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
#line 17 "shaders/lilypad.raster.hlsl"

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
#line 18 "shaders/lilypad.raster.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 19 "shaders/lilypad.raster.hlsl"

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
#line 20 "shaders/lilypad.raster.hlsl"

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
#line 21 "shaders/lilypad.raster.hlsl"

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
#line 22 "shaders/lilypad.raster.hlsl"

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
#line 23 "shaders/lilypad.raster.hlsl"

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
#line 24 "shaders/lilypad.raster.hlsl"

Texture2D<float4> water_noise_texture : register(t15);
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
#line 26 "shaders/lilypad.raster.hlsl"

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
#line 27 "shaders/lilypad.raster.hlsl"

Texture2D<float4> terrain_heightmap;
#line 1 "shaders/inc/terrain_heightmap.inc.hlsl"







#ifndef TINYGLADE_INC_TERRAIN_HEIGHTMAP_HLSL
#define TINYGLADE_INC_TERRAIN_HEIGHTMAP_HLSL


static const float TERRAIN_WORLD_HALF_EXTENT = 65.0;
static const float TERRAIN_HEIGHT_MIN = -2.5;
static const float TERRAIN_HEIGHT_MAX = 24.0;
static const float TERRAIN_SAMPLE_SCALE = 0.0076923076994717121124267578125; 


float2 terrain_heightmap_uv(float2 world_xz)
{
    return world_xz * TERRAIN_SAMPLE_SCALE + 0.5;
}


float get_terrain_height(Texture2D terrain_heightmap, SamplerState s, float2 world_xz)
{
    float t = terrain_heightmap.SampleLevel(s, terrain_heightmap_uv(world_xz), 0.0).x;
    return lerp(TERRAIN_HEIGHT_MIN, TERRAIN_HEIGHT_MAX, t);
}










#ifndef TINYGLADE_TERRAIN_HEIGHTMAP_NO_GLOBAL
float terrain_height(float2 world_xz)
{
    return get_terrain_height(terrain_heightmap, sampler_default, world_xz);
}
#endif


float3 get_terrain_normal(Texture2D terrain_heightmap, SamplerState s, float2 world_xz, float2 heightmap_dims)
{
    float4 g = terrain_heightmap.GatherRed(s, terrain_heightmap_uv(world_xz), int2(0, 0));
    float2 grad = (float2((g.z + g.y) - (g.w + g.x), (g.x + g.y) - (g.w + g.z)) * 0.5) * 26.5;
    grad /= (130.0 / max(heightmap_dims, 1e-6));
    float3 right = normalize(float3(1.0, grad.x, 0.0));
    float3 fwd = normalize(float3(0.0, grad.y, 1.0));
    return normalize(cross(fwd, right));
}

#endif 
#line 29 "shaders/lilypad.raster.hlsl"



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
#line 32 "shaders/lilypad.raster.hlsl"

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
#line 33 "shaders/lilypad.raster.hlsl"

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
#line 34 "shaders/lilypad.raster.hlsl"

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
#line 35 "shaders/lilypad.raster.hlsl"

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
#line 36 "shaders/lilypad.raster.hlsl"

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
#line 37 "shaders/lilypad.raster.hlsl"

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
#line 38 "shaders/lilypad.raster.hlsl"

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
#line 39 "shaders/lilypad.raster.hlsl"

#line 1 "shaders/inc/sh.hlsl"










float4 sh_basis(float3 dir)
{
    return float4(0.2820949852466583251953125f, 0.48860299587249755859375f * dir);
}

float sh_eval(float4 sh, float3 dir)
{
    return dot(sh, sh_basis(dir));
}
#line 40 "shaders/lilypad.raster.hlsl"

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
#line 41 "shaders/lilypad.raster.hlsl"







uint2 water_noise_texture_dims;
uint2 grass_object_push_dims;
uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
uint2 lilypad_normal_dims;
uint2 lilypad_alpha_dims;
int bool_render_shadows;


struct Reed
{
    Affine3Packed xform;
};

StructuredBuffer<Reed> instance_buffer : register(t17);

Texture2D<float4> grass_object_push : register(t16);
Texture2D<float4> lilypad_alpha : register(t21);

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(2)]] float3 Vertex_Color: Vertex_Color;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
    float C5 : TEXCOORD5;
    float4 position : SV_Position;
};



float3 water_displacement(float2 origin_xz, float t)
{
    if (glade_theme_id == 4u)
    {
        return 0.0f;
    }
    float2 jitter[4];
    static const float wave_scales[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
    static const float wave_amps[4] = { 2.0f, 2.0f, 0.5f, 0.2f };
    static const uint wave_seeds[4] = { 17u, 18u, 20u, 21u };
    uint jit_a = 0u;
    uint jit_b = 0u;
    bool jit_c = false;
    bool jit_d = false;
    for (uint oct = 0u; oct < 4u; oct++)
    {
        float seed_f = float(wave_seeds[oct]);
        int cell = int(floor(seed_f));
        float f = frac(seed_f);
        uint s = uint(cell);
        uint s5 = s + (s << 10u);
        s5 ^= (s5 >> 6u);
        s5 += (s5 << 3u);
        s5 ^= (s5 >> 11u);
        bool use_a = jit_d ? jit_d : true;
        uint A = jit_d ? jit_b : 1664525u;
        bool use_b = jit_c ? jit_c : true;
        uint B = jit_c ? jit_a : 1013904223u;
        uint h0 = ((s * A) + (s5 + (s5 << 15u)) + B) * A;
        h0 ^= (h0 >> 11u);
        h0 ^= ((h0 << 7u) & 2636928640u);
        h0 ^= ((h0 << 15u) & 4022730752u);
        uint s1 = s + 1u;
        uint s1_5 = s1 + (s1 << 10u);
        s1_5 ^= (s1_5 >> 6u);
        s1_5 += (s1_5 << 3u);
        s1_5 ^= (s1_5 >> 11u);
        uint A1 = use_a ? A : 1664525u;
        uint B1 = use_b ? B : 1013904223u;
        uint h1 = ((s1 * A1) + (s1_5 + (s1_5 << 15u)) + B1) * A1;
        h1 ^= (h1 >> 11u);
        h1 ^= ((h1 << 7u) & 2636928640u);
        h1 ^= ((h1 << 15u) & 4022730752u);
        float u = f * f * (3.0f - 2.0f * f);
        float2 a = float2(incb_perlin_hash01(h0), rand01(hash(h0 ^ (h0 >> 18u))));
        float2 b = float2(incb_perlin_hash01(h1), rand01(hash(h1 ^ (h1 >> 18u))));
        jitter[oct] = normalize(((a * (1.0f - u)) + (b * u)) - 0.5f) * t * 3.0f;
        jit_a = B1;
        jit_b = A1;
        jit_c = use_b ? use_b : true;
        jit_d = use_a ? use_a : true;
    }
    float3 acc = 0.0f;
    for (uint wave = 0u; wave < 1u; wave++)
    {
        float angle = float(wave + 1u) * (GOLDEN_RATIO * TWO_PI);
        float2x2 rot = float2x2(float2(cos(angle), sin(angle)), float2(-sin(angle), cos(angle)));
        float2 wave_pos = mul(rot, (origin_xz * wave_scales[wave]) + jitter[wave]) * 0.012f;
        float3 sample = (water_noise_texture.SampleLevel(sampler_llr, wave_pos, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f))
                      + float3(-0.5f, -1.0f, -1.0f);
        float3 disp = sample * wave_amps[wave] * float3(1.0f, wave_scales[wave] * 0.012f, wave_scales[wave] * 0.012f);
        acc = float3(acc.x, acc.yz + mul(disp.yz, rot));
    }
    return acc;
}

VsOut vs_main(VsIn vin)
{
    VsOut o;

    Affine3Packed xform = instance_buffer[vin.instance_id].xform;
    float3x4 m = transpose(float4x3(float3(xform.data[0], xform.data[1], xform.data[2]),
                                    float3(xform.data[3], xform.data[4], xform.data[5]),
                                    float3(xform.data[6], xform.data[7], xform.data[8]),
                                    float3(xform.data[9], xform.data[10], xform.data[11])));

    float3 pos = mul(m, float4(vin.Vertex_Position, 1.0f));
    float3 origin = mul(m, float4(0.0f, 0.0f, 0.0f, 1.0f));

    uint seed = uint((((origin.x * 20.0f) - (origin.z * 20.0f)) + 2000.0f) + vin.Vertex_Position.x + vin.Vertex_Position.z);
    float randv = rand01(hash(seed));

    pos.y = pos.y + lerp(-0.025f, 0.0f, randv);
    float2 origin_xz = origin.xz;
    float4 grass_push = grass_object_push.SampleLevel(sampler_default, (origin_xz * 0.007692307699471712f) + 0.5f, 0.0f);

    float3 water_disp = water_displacement(origin_xz, float(game_time_sec) + game_time_frac);

    float2 disp2 = pos.xz + (((float3(grass_push.x, 0.0f, grass_push.y) * float(clamp(grass_push.z, 0.0f, 1.0f) > 0.0f)) * 0.35f) + (water_disp.yxz * float3(2.0f, 0.01f, 2.0f))).xz;
    float3 wpos = float3(disp2.x, pos.y, disp2.y);

    float wobble_k = vin.Vertex_Position.y + 0.5f;
    float2 wobble_pos = (origin_xz * 0.1f) + (float2(float(game_time_sec) + game_time_frac, 0.0f) * 1.2f);
    int2 wobble_cell = int2(floor(wobble_pos));
    float2 wobble_frac = frac(wobble_pos);
    float2 wobble_u = wobble_frac * wobble_frac * (3.0f - 2.0f * wobble_frac);
    float n00 = incb_perlin_hash01(hash(uint2(wobble_cell)));
    float n10 = incb_perlin_hash01(hash(uint2(wobble_cell + int2(1, 0))));
    float n01 = incb_perlin_hash01(hash(uint2(wobble_cell + int2(0, 1))));
    float n11 = incb_perlin_hash01(hash(uint2(wobble_cell + int2(1, 1))));
    float wobble_noise = (1.0f - wobble_u.y) * ((n00 * (1.0f - wobble_u.x)) + (n10 * wobble_u.x))
                       + (n01 * (1.0f - wobble_u.x)) * wobble_u.y
                       + (n11 * wobble_u.x) * wobble_u.y;
    float wobble_rand = lerp(0.7f, 1.0f, rand01(hash(uint(int(randv)))));
    wpos.x = wpos.x - ((wobble_k * wobble_k) * sin((wobble_noise * PI) - HALF_PI + 0.3f) * 0.7f * (glade_theme_id == 4u ? 0.025f : 0.05f)) * wobble_rand;

    float3 color_pow = lerp(1.2f, 2.0f, randv).xxx;
    float3 vertex_color = pow(vin.Vertex_Color, color_pow);
    float3 color;
    if (glade_theme_id == 4u)
    {
        if (vin.Vertex_Color.y > 0.05f)
        {
            color = pow(float3(0.15f, 0.09f, 0.03f), color_pow);
        }
        else
        {
            color = vertex_color;
        }
    }
    else
    {
        color = vertex_color;
    }

    o.position = mul(GetCurrentViewConstants().world_to_sample, float4(wpos.x, pos.y, disp2.y, 1.0f));
    o.C1 = float3(0.0f, 1.0f, 0.0f);
    o.C2 = wpos;
    o.C3 = color;
    o.C4 = (vin.Vertex_Position.xz * 1.5f) + 0.5f;
    o.C5 = randv;
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
    float C5 : TEXCOORD5;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    if (lilypad_alpha.SampleBias(sampler_llr, pin.C4, global_mip_bias).x < lerp(0.0f, 0.9f, pin.C5))
    {
        discard;
    }
    bool snow_branch = false;
    if (glade_theme_id == 4u)
    {
        snow_branch = pin.C2.y > 0.0f;
    }
    float3 albedo = pin.C3;
    if (snow_branch)
    {
        albedo = lerp(pin.C3, 0.7f.xxx, smoothstep(-0.2f, 1.0f, clamp(pin.C1.y, 0.0f, 1.0f)).xxx);
    }
    float3 packed_normal = (pin.C1 * 0.5f) + 0.5f;
    float3 linear_albedo = sqrt(clamp(albedo, 0.0f, 1.0f));
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 4278190080u;
    uint albedo_word = uint(clamp(linear_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(linear_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(linear_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 4278190080u;
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

struct Reed
{
    Affine3Packed xform;
};

static uint _155;

static const float _157[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float _158[4] = { 2.0f, 2.0f, 0.5f, 0.20000000298023223876953125f };
static const uint _159[4] = { 17u, 18u, 20u, 21u };

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
    uint2 _Globals_water_noise_texture_dims : packoffset(c0.y);
    uint2 _Globals_grass_object_push_dims : packoffset(c1);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c1.z);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c2);
    uint2 _Globals_lilypad_normal_dims : packoffset(c2.z);
    uint2 _Globals_lilypad_alpha_dims : packoffset(c3);
    int _Globals_bool_render_shadows : packoffset(c3.z);
};

ByteAddressBuffer instance_buffer : register(t17, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> water_noise_texture : register(t15, space0);
Texture2D<float4> grass_object_push : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static float2 out_var_C4;
static float out_var_C5;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(2)]] float3 in_var_Vertex_Color: Vertex_Color;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float out_var_C5 : TEXCOORD5;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    bool _259;
    Affine3Packed _186;
    [unroll]
    for (int _3ident = 0; _3ident < 12; _3ident++)
    {
        _186.data[_3ident] = instance_buffer.Load<float>(_3ident * 4 + uint(gl_InstanceIndex) * 48 + 0);
    }
    float3x4 _205 = transpose(float4x3(float3(_186.data[0], _186.data[1], _186.data[2]), float3(_186.data[3], _186.data[4], _186.data[5]), float3(_186.data[6], _186.data[7], _186.data[8]), float3(_186.data[9], _186.data[10], _186.data[11])));
    float3 _210 = mul(_205, float4(in_var_Vertex_Position, 1.0f));
    float3 _211 = mul(_205, float4(0.0f, 0.0f, 0.0f, 1.0f));
    uint _220 = uint(((((_211.x * 20.0f) - (_211.z * 20.0f)) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint _222 = _220 + (_220 << 10u);
    uint _224 = _222 ^ (_222 >> 6u);
    uint _226 = _224 + (_224 << 3u);
    uint _228 = _226 ^ (_226 >> 11u);
    float _234 = asfloat(((_228 + (_228 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float _237 = _210.y + lerp(-0.02500000037252902984619140625f, 0.0f, _234);
    float3 _238 = _210;
    _238.y = _237;
    float2 _239 = _211.xz;
    float4 _245 = grass_object_push.SampleLevel(sampler_default, (_239 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    uint _437;
    uint _438;
    bool _439;
    bool _440;
    float3 _441;
    do
    {
        _259 = frame_constants_glade_theme_id == 4u;
        if (_259)
        {
            _437 = _155;
            _438 = _155;
            _439 = false;
            _440 = false;
            _441 = 0.0f.xxx;
            break;
        }
        float _267 = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        uint _272;
        uint _274;
        bool _276;
        bool _278;
        _272 = _155;
        _274 = _155;
        _276 = false;
        _278 = false;
        float2 _181[4];
        for (uint _269 = 0u; _269 < 4u; )
        {
            float _284 = float(_159[_269]);
            int _286 = int(floor(_284));
            float _287 = frac(_284);
            uint _288 = uint(_286);
            uint _290 = _288 + (_288 << 10u);
            uint _292 = _290 ^ (_290 >> 6u);
            uint _294 = _292 + (_292 << 3u);
            uint _296 = _294 ^ (_294 >> 11u);
            bool _299 = _278 ? _278 : true;
            uint _300 = _278 ? _274 : 1664525u;
            bool _301 = _276 ? _276 : true;
            uint _302 = _276 ? _272 : 1013904223u;
            uint _306 = (((_288 * _300) + (_296 + (_296 << 15u))) + _302) * _300;
            uint _308 = _306 ^ (_306 >> 11u);
            uint _311 = _308 ^ ((_308 << 7u) & 2636928640u);
            uint _314 = _311 ^ ((_311 << 15u) & 4022730752u);
            uint _316 = _314 ^ (_314 >> 18u);
            uint _318 = _316 + (_316 << 10u);
            uint _320 = _318 ^ (_318 >> 6u);
            uint _322 = _320 + (_320 << 3u);
            uint _324 = _322 ^ (_322 >> 11u);
            uint _337 = uint(_286 + 1);
            uint _339 = _337 + (_337 << 10u);
            uint _341 = _339 ^ (_339 >> 6u);
            uint _343 = _341 + (_341 << 3u);
            uint _345 = _343 ^ (_343 >> 11u);
            uint _275 = _299 ? _300 : 1664525u;
            uint _273 = _301 ? _302 : 1013904223u;
            uint _351 = (((_337 * _275) + (_345 + (_345 << 15u))) + _273) * _275;
            uint _353 = _351 ^ (_351 >> 11u);
            uint _356 = _353 ^ ((_353 << 7u) & 2636928640u);
            uint _359 = _356 ^ ((_356 << 15u) & 4022730752u);
            uint _361 = _359 ^ (_359 >> 18u);
            uint _363 = _361 + (_361 << 10u);
            uint _365 = _363 ^ (_363 >> 6u);
            uint _367 = _365 + (_365 << 3u);
            uint _369 = _367 ^ (_367 >> 11u);
            float _384 = (_287 * _287) * (3.0f - (2.0f * _287));
            _181[_269] = (normalize(((float2(asfloat((_316 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_324 + (_324 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - _384)) + (float2(asfloat((_361 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_369 + (_369 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _384)) - 0.5f.xx) * _267) * 3.0f;
            _269++;
            _272 = _273;
            _274 = _275;
            _276 = _301 ? _301 : true;
            _278 = _299 ? _299 : true;
            continue;
        }
        float3 _395;
        _395 = 0.0f.xxx;
        for (uint _398 = 0u; _398 < 1u; )
        {
            uint _399 = _398 + 1u;
            float _403 = float(_399) * 10.16640758514404296875f;
            float _404 = cos(_403);
            float _405 = sin(_403);
            float2x2 _409 = float2x2(float2(_404, _405), float2(-_405, _404));
            float2 _436 = _395.yz + mul(((((water_noise_texture.SampleLevel(sampler_llr, mul(_409, (_239 * _157[_398]) + _181[_398]) * 0.01200000010430812835693359375f, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * _158[_398]) * float2(1.0f, _157[_398] * 0.01200000010430812835693359375f).xyy).yz, _409);
            _395 = float3(_395.x, _436.x, _436.y);
            _398 = _399;
            continue;
        }
        _437 = _272;
        _438 = _274;
        _439 = _276;
        _440 = _278;
        _441 = _395;
        break;
    } while(false);
    float2 _447 = _238.xz + (((float3(_245.x, 0.0f, _245.y) * ((clamp(_245.z, 0.0f, 1.0f) > 0.0f) ? 1.0f : 0.0f)) * 0.3499999940395355224609375f) + (_441.yxz * float3(2.0f, 0.00999999977648258209228515625f, 2.0f))).xz;
    float3 _448 = float3(_447.x, _238.y, _447.y);
    float _450 = in_var_Vertex_Position.y + 0.5f;
    float2 _461 = (_239 * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 _463 = int2(floor(_461));
    float2 _464 = frac(_461);
    uint2 _465 = uint2(_463);
    uint _467 = _465.y;
    uint _469 = _467 + (_467 << 10u);
    uint _471 = _469 ^ (_469 >> 6u);
    uint _473 = _471 + (_471 << 3u);
    uint _475 = _473 ^ (_473 >> 11u);
    bool _478 = _440 ? _440 : true;
    uint _479 = _440 ? _438 : 1664525u;
    bool _480 = _439 ? _439 : true;
    uint _481 = _439 ? _437 : 1013904223u;
    uint _485 = (((_465.x * _479) + (_475 + (_475 << 15u))) + _481) * _479;
    uint _487 = _485 ^ (_485 >> 11u);
    uint _490 = _487 ^ ((_487 << 7u) & 2636928640u);
    uint _493 = _490 ^ ((_490 << 15u) & 4022730752u);
    uint2 _501 = uint2(_463 + int2(1, 0));
    uint _503 = _501.y;
    uint _505 = _503 + (_503 << 10u);
    uint _507 = _505 ^ (_505 >> 6u);
    uint _509 = _507 + (_507 << 3u);
    uint _511 = _509 ^ (_509 >> 11u);
    bool _514 = _478 ? _478 : true;
    uint _515 = _478 ? _479 : 1664525u;
    bool _516 = _480 ? _480 : true;
    uint _517 = _480 ? _481 : 1013904223u;
    uint _521 = (((_501.x * _515) + (_511 + (_511 << 15u))) + _517) * _515;
    uint _523 = _521 ^ (_521 >> 11u);
    uint _526 = _523 ^ ((_523 << 7u) & 2636928640u);
    uint _529 = _526 ^ ((_526 << 15u) & 4022730752u);
    uint2 _537 = uint2(_463 + int2(0, 1));
    uint _539 = _537.y;
    uint _541 = _539 + (_539 << 10u);
    uint _543 = _541 ^ (_541 >> 6u);
    uint _545 = _543 + (_543 << 3u);
    uint _547 = _545 ^ (_545 >> 11u);
    uint _551 = _514 ? _515 : 1664525u;
    uint _553 = _516 ? _517 : 1013904223u;
    uint _557 = (((_537.x * _551) + (_547 + (_547 << 15u))) + _553) * _551;
    uint _559 = _557 ^ (_557 >> 11u);
    uint _562 = _559 ^ ((_559 << 7u) & 2636928640u);
    uint _565 = _562 ^ ((_562 << 15u) & 4022730752u);
    uint2 _573 = uint2(_463 + int2(1, 1));
    uint _575 = _573.y;
    uint _577 = _575 + (_575 << 10u);
    uint _579 = _577 ^ (_577 >> 6u);
    uint _581 = _579 + (_579 << 3u);
    uint _583 = _581 ^ (_581 >> 11u);
    uint _586 = (_514 ? _514 : true) ? _551 : 1664525u;
    uint _591 = (((_573.x * _586) + (_583 + (_583 << 15u))) + ((_516 ? _516 : true) ? _553 : 1013904223u)) * _586;
    uint _593 = _591 ^ (_591 >> 11u);
    uint _596 = _593 ^ ((_593 << 7u) & 2636928640u);
    uint _599 = _596 ^ ((_596 << 15u) & 4022730752u);
    float2 _609 = (_464 * _464) * (3.0f.xx - (_464 * 2.0f));
    float _610 = _609.x;
    float _611 = 1.0f - _610;
    float _613 = _609.y;
    uint _631 = uint(int(_234));
    uint _633 = _631 + (_631 << 10u);
    uint _635 = _633 ^ (_633 >> 6u);
    uint _637 = _635 + (_635 << 3u);
    uint _639 = _637 ^ (_637 >> 11u);
    float _650 = _447.x - (((((_450 * _450) * (sin((((((1.0f - _613) * (((asfloat(((_493 ^ (_493 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _611) + ((asfloat(((_529 ^ (_529 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _610))) + (((asfloat(((_565 ^ (_565 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _611) * _613)) + (((asfloat(((_599 ^ (_599 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _610) * _613)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * (_259 ? 0.02500000037252902984619140625f : 0.0500000007450580596923828125f)) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((_639 + (_639 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    _448.x = _650;
    float3 _660 = lerp(1.2000000476837158203125f, 2.0f, _234).xxx;
    float3 _661 = pow(in_var_Vertex_Color, _660);
    float3 _673;
    if (_259)
    {
        float3 _672;
        if (in_var_Vertex_Color.y > 0.0500000007450580596923828125f)
        {
            _672 = pow(float3(0.1500000059604644775390625f, 0.0900000035762786865234375f, 0.0300000011920928955078125f), _660);
        }
        else
        {
            _672 = _661;
        }
        _673 = _672;
    }
    else
    {
        _673 = _661;
    }
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_650, _237, _447.y, 1.0f));
    out_var_C1 = float3(0.0f, 1.0f, 0.0f);
    out_var_C2 = _448;
    out_var_C3 = _673;
    out_var_C4 = (in_var_Vertex_Position.xz * 1.5f) + 0.5f.xx;
    out_var_C5 = _234;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
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

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> lilypad_alpha : register(t21, space0);

static float3 in_var_C1;
static float3 in_var_C2;
static float3 in_var_C3;
static float2 in_var_C4;
static float in_var_C5;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2: Vertex_Color;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 in_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float in_var_C5 : TEXCOORD5;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    if (lilypad_alpha.SampleBias(sampler_llr, in_var_C4, frame_constants_global_mip_bias).x < lerp(0.0f, 0.89999997615814208984375f, in_var_C5))
    {
        discard;
    }
    bool _112;
    if (frame_constants_glade_theme_id == 4u)
    {
        _112 = in_var_C2.y > 0.0f;
    }
    else
    {
        _112 = false;
    }
    float3 _120;
    if (_112)
    {
        _120 = lerp(in_var_C3, 0.699999988079071044921875f.xxx, smoothstep(-0.20000000298023223876953125f, 1.0f, clamp(in_var_C1.y, 0.0f, 1.0f)).xxx);
    }
    else
    {
        _120 = in_var_C3;
    }
    float3 _122 = (in_var_C1 * 0.5f) + 0.5f.xxx;
    float3 _144 = sqrt(clamp(_120, 0.0f.xxx, 1.0f.xxx));
    out_var_SV_TARGET0 = uint2(((uint((clamp(_122.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_122.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_122.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u, ((uint((clamp(_144.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_144.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_144.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u);
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C4 = stage_input.in_var_C4;
    in_var_C5 = stage_input.in_var_C5;
    frag_main();
    SPIRV_Cross_Output stage_output;
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

struct Affine3Packed
{
    float data[12];
};

struct Reed
{
    Affine3Packed xform;
};

static uint _155;

static const float _157[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float _158[4] = { 2.0f, 2.0f, 0.5f, 0.20000000298023223876953125f };
static const uint _159[4] = { 17u, 18u, 20u, 21u };

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
    uint2 _Globals_water_noise_texture_dims : packoffset(c0.y);
    uint2 _Globals_grass_object_push_dims : packoffset(c1);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c1.z);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c2);
    uint2 _Globals_lilypad_normal_dims : packoffset(c2.z);
    uint2 _Globals_lilypad_alpha_dims : packoffset(c3);
    int _Globals_bool_render_shadows : packoffset(c3.z);
};

ByteAddressBuffer instance_buffer : register(t17, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> water_noise_texture : register(t15, space0);
Texture2D<float4> grass_object_push : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static float2 out_var_C4;
static float out_var_C5;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(2)]] float3 in_var_Vertex_Color: Vertex_Color;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float out_var_C5 : TEXCOORD5;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    bool _259;
    Affine3Packed _186;
    [unroll]
    for (int _3ident = 0; _3ident < 12; _3ident++)
    {
        _186.data[_3ident] = instance_buffer.Load<float>(_3ident * 4 + uint(gl_InstanceIndex) * 48 + 0);
    }
    float3x4 _205 = transpose(float4x3(float3(_186.data[0], _186.data[1], _186.data[2]), float3(_186.data[3], _186.data[4], _186.data[5]), float3(_186.data[6], _186.data[7], _186.data[8]), float3(_186.data[9], _186.data[10], _186.data[11])));
    float3 _210 = mul(_205, float4(in_var_Vertex_Position, 1.0f));
    float3 _211 = mul(_205, float4(0.0f, 0.0f, 0.0f, 1.0f));
    uint _220 = uint(((((_211.x * 20.0f) - (_211.z * 20.0f)) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint _222 = _220 + (_220 << 10u);
    uint _224 = _222 ^ (_222 >> 6u);
    uint _226 = _224 + (_224 << 3u);
    uint _228 = _226 ^ (_226 >> 11u);
    float _234 = asfloat(((_228 + (_228 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float _237 = _210.y + lerp(-0.02500000037252902984619140625f, 0.0f, _234);
    float3 _238 = _210;
    _238.y = _237;
    float2 _239 = _211.xz;
    float4 _245 = grass_object_push.SampleLevel(sampler_default, (_239 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    uint _437;
    uint _438;
    bool _439;
    bool _440;
    float3 _441;
    do
    {
        _259 = frame_constants_glade_theme_id == 4u;
        if (_259)
        {
            _437 = _155;
            _438 = _155;
            _439 = false;
            _440 = false;
            _441 = 0.0f.xxx;
            break;
        }
        float _267 = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        uint _272;
        uint _274;
        bool _276;
        bool _278;
        _272 = _155;
        _274 = _155;
        _276 = false;
        _278 = false;
        float2 _181[4];
        for (uint _269 = 0u; _269 < 4u; )
        {
            float _284 = float(_159[_269]);
            int _286 = int(floor(_284));
            float _287 = frac(_284);
            uint _288 = uint(_286);
            uint _290 = _288 + (_288 << 10u);
            uint _292 = _290 ^ (_290 >> 6u);
            uint _294 = _292 + (_292 << 3u);
            uint _296 = _294 ^ (_294 >> 11u);
            bool _299 = _278 ? _278 : true;
            uint _300 = _278 ? _274 : 1664525u;
            bool _301 = _276 ? _276 : true;
            uint _302 = _276 ? _272 : 1013904223u;
            uint _306 = (((_288 * _300) + (_296 + (_296 << 15u))) + _302) * _300;
            uint _308 = _306 ^ (_306 >> 11u);
            uint _311 = _308 ^ ((_308 << 7u) & 2636928640u);
            uint _314 = _311 ^ ((_311 << 15u) & 4022730752u);
            uint _316 = _314 ^ (_314 >> 18u);
            uint _318 = _316 + (_316 << 10u);
            uint _320 = _318 ^ (_318 >> 6u);
            uint _322 = _320 + (_320 << 3u);
            uint _324 = _322 ^ (_322 >> 11u);
            uint _337 = uint(_286 + 1);
            uint _339 = _337 + (_337 << 10u);
            uint _341 = _339 ^ (_339 >> 6u);
            uint _343 = _341 + (_341 << 3u);
            uint _345 = _343 ^ (_343 >> 11u);
            uint _275 = _299 ? _300 : 1664525u;
            uint _273 = _301 ? _302 : 1013904223u;
            uint _351 = (((_337 * _275) + (_345 + (_345 << 15u))) + _273) * _275;
            uint _353 = _351 ^ (_351 >> 11u);
            uint _356 = _353 ^ ((_353 << 7u) & 2636928640u);
            uint _359 = _356 ^ ((_356 << 15u) & 4022730752u);
            uint _361 = _359 ^ (_359 >> 18u);
            uint _363 = _361 + (_361 << 10u);
            uint _365 = _363 ^ (_363 >> 6u);
            uint _367 = _365 + (_365 << 3u);
            uint _369 = _367 ^ (_367 >> 11u);
            float _384 = (_287 * _287) * (3.0f - (2.0f * _287));
            _181[_269] = (normalize(((float2(asfloat((_316 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_324 + (_324 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - _384)) + (float2(asfloat((_361 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_369 + (_369 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _384)) - 0.5f.xx) * _267) * 3.0f;
            _269++;
            _272 = _273;
            _274 = _275;
            _276 = _301 ? _301 : true;
            _278 = _299 ? _299 : true;
            continue;
        }
        float3 _395;
        _395 = 0.0f.xxx;
        for (uint _398 = 0u; _398 < 1u; )
        {
            uint _399 = _398 + 1u;
            float _403 = float(_399) * 10.16640758514404296875f;
            float _404 = cos(_403);
            float _405 = sin(_403);
            float2x2 _409 = float2x2(float2(_404, _405), float2(-_405, _404));
            float2 _436 = _395.yz + mul(((((water_noise_texture.SampleLevel(sampler_llr, mul(_409, (_239 * _157[_398]) + _181[_398]) * 0.01200000010430812835693359375f, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * _158[_398]) * float2(1.0f, _157[_398] * 0.01200000010430812835693359375f).xyy).yz, _409);
            _395 = float3(_395.x, _436.x, _436.y);
            _398 = _399;
            continue;
        }
        _437 = _272;
        _438 = _274;
        _439 = _276;
        _440 = _278;
        _441 = _395;
        break;
    } while(false);
    float2 _447 = _238.xz + (((float3(_245.x, 0.0f, _245.y) * ((clamp(_245.z, 0.0f, 1.0f) > 0.0f) ? 1.0f : 0.0f)) * 0.3499999940395355224609375f) + (_441.yxz * float3(2.0f, 0.00999999977648258209228515625f, 2.0f))).xz;
    float3 _448 = float3(_447.x, _238.y, _447.y);
    float _450 = in_var_Vertex_Position.y + 0.5f;
    float2 _461 = (_239 * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 _463 = int2(floor(_461));
    float2 _464 = frac(_461);
    uint2 _465 = uint2(_463);
    uint _467 = _465.y;
    uint _469 = _467 + (_467 << 10u);
    uint _471 = _469 ^ (_469 >> 6u);
    uint _473 = _471 + (_471 << 3u);
    uint _475 = _473 ^ (_473 >> 11u);
    bool _478 = _440 ? _440 : true;
    uint _479 = _440 ? _438 : 1664525u;
    bool _480 = _439 ? _439 : true;
    uint _481 = _439 ? _437 : 1013904223u;
    uint _485 = (((_465.x * _479) + (_475 + (_475 << 15u))) + _481) * _479;
    uint _487 = _485 ^ (_485 >> 11u);
    uint _490 = _487 ^ ((_487 << 7u) & 2636928640u);
    uint _493 = _490 ^ ((_490 << 15u) & 4022730752u);
    uint2 _501 = uint2(_463 + int2(1, 0));
    uint _503 = _501.y;
    uint _505 = _503 + (_503 << 10u);
    uint _507 = _505 ^ (_505 >> 6u);
    uint _509 = _507 + (_507 << 3u);
    uint _511 = _509 ^ (_509 >> 11u);
    bool _514 = _478 ? _478 : true;
    uint _515 = _478 ? _479 : 1664525u;
    bool _516 = _480 ? _480 : true;
    uint _517 = _480 ? _481 : 1013904223u;
    uint _521 = (((_501.x * _515) + (_511 + (_511 << 15u))) + _517) * _515;
    uint _523 = _521 ^ (_521 >> 11u);
    uint _526 = _523 ^ ((_523 << 7u) & 2636928640u);
    uint _529 = _526 ^ ((_526 << 15u) & 4022730752u);
    uint2 _537 = uint2(_463 + int2(0, 1));
    uint _539 = _537.y;
    uint _541 = _539 + (_539 << 10u);
    uint _543 = _541 ^ (_541 >> 6u);
    uint _545 = _543 + (_543 << 3u);
    uint _547 = _545 ^ (_545 >> 11u);
    uint _551 = _514 ? _515 : 1664525u;
    uint _553 = _516 ? _517 : 1013904223u;
    uint _557 = (((_537.x * _551) + (_547 + (_547 << 15u))) + _553) * _551;
    uint _559 = _557 ^ (_557 >> 11u);
    uint _562 = _559 ^ ((_559 << 7u) & 2636928640u);
    uint _565 = _562 ^ ((_562 << 15u) & 4022730752u);
    uint2 _573 = uint2(_463 + int2(1, 1));
    uint _575 = _573.y;
    uint _577 = _575 + (_575 << 10u);
    uint _579 = _577 ^ (_577 >> 6u);
    uint _581 = _579 + (_579 << 3u);
    uint _583 = _581 ^ (_581 >> 11u);
    uint _586 = (_514 ? _514 : true) ? _551 : 1664525u;
    uint _591 = (((_573.x * _586) + (_583 + (_583 << 15u))) + ((_516 ? _516 : true) ? _553 : 1013904223u)) * _586;
    uint _593 = _591 ^ (_591 >> 11u);
    uint _596 = _593 ^ ((_593 << 7u) & 2636928640u);
    uint _599 = _596 ^ ((_596 << 15u) & 4022730752u);
    float2 _609 = (_464 * _464) * (3.0f.xx - (_464 * 2.0f));
    float _610 = _609.x;
    float _611 = 1.0f - _610;
    float _613 = _609.y;
    uint _631 = uint(int(_234));
    uint _633 = _631 + (_631 << 10u);
    uint _635 = _633 ^ (_633 >> 6u);
    uint _637 = _635 + (_635 << 3u);
    uint _639 = _637 ^ (_637 >> 11u);
    float _650 = _447.x - (((((_450 * _450) * (sin((((((1.0f - _613) * (((asfloat(((_493 ^ (_493 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _611) + ((asfloat(((_529 ^ (_529 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _610))) + (((asfloat(((_565 ^ (_565 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _611) * _613)) + (((asfloat(((_599 ^ (_599 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _610) * _613)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * (_259 ? 0.02500000037252902984619140625f : 0.0500000007450580596923828125f)) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((_639 + (_639 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    _448.x = _650;
    float3 _660 = lerp(1.2000000476837158203125f, 2.0f, _234).xxx;
    float3 _661 = pow(in_var_Vertex_Color, _660);
    float3 _673;
    if (_259)
    {
        float3 _672;
        if (in_var_Vertex_Color.y > 0.0500000007450580596923828125f)
        {
            _672 = pow(float3(0.1500000059604644775390625f, 0.0900000035762786865234375f, 0.0300000011920928955078125f), _660);
        }
        else
        {
            _672 = _661;
        }
        _673 = _672;
    }
    else
    {
        _673 = _661;
    }
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_650, _237, _447.y, 1.0f));
    out_var_C1 = float3(0.0f, 1.0f, 0.0f);
    out_var_C2 = _448;
    out_var_C3 = _673;
    out_var_C4 = (in_var_Vertex_Position.xz * 1.5f) + 0.5f.xx;
    out_var_C5 = _234;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
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

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> lilypad_alpha : register(t21, space0);

static float2 in_var_C4;
static float in_var_C5;

struct SPIRV_Cross_Input
{
    [[vk::location(4)]] float2 in_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float in_var_C5 : TEXCOORD5;
};

void frag_main()
{
    if (lilypad_alpha.SampleBias(sampler_llr, in_var_C4, frame_constants_global_mip_bias).x < lerp(0.0f, 0.89999997615814208984375f, in_var_C5))
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C4 = stage_input.in_var_C4;
    in_var_C5 = stage_input.in_var_C5;
    frag_main();
}
#pragma dxc diagnostic pop
#endif

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"

template<typename T> T spv_fmsub(T a, T b, float c) { precise T p = a * c; precise T q = b * c; precise T r = p - q; return r; }
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

struct Reed
{
    Affine3Packed xform;
};

static uint _155;

static const float _157[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float _158[4] = { 2.0f, 2.0f, 0.5f, 0.20000000298023223876953125f };
static const uint _159[4] = { 17u, 18u, 20u, 21u };

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
    uint2 _Globals_water_noise_texture_dims : packoffset(c0.y);
    uint2 _Globals_grass_object_push_dims : packoffset(c1);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c1.z);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c2);
    uint2 _Globals_lilypad_normal_dims : packoffset(c2.z);
    uint2 _Globals_lilypad_alpha_dims : packoffset(c3);
    int _Globals_bool_render_shadows : packoffset(c3.z);
};

ByteAddressBuffer instance_buffer : register(t17, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> water_noise_texture : register(t15, space0);
Texture2D<float4> grass_object_push : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static float2 out_var_C4;
static float out_var_C5;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(2)]] float3 in_var_Vertex_Color: Vertex_Color;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float out_var_C5 : TEXCOORD5;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    bool _259;
    Affine3Packed _186;
    [unroll]
    for (int _3ident = 0; _3ident < 12; _3ident++)
    {
        _186.data[_3ident] = instance_buffer.Load<float>(_3ident * 4 + uint(gl_InstanceIndex) * 48 + 0);
    }
    float3x4 _205 = transpose(float4x3(float3(_186.data[0], _186.data[1], _186.data[2]), float3(_186.data[3], _186.data[4], _186.data[5]), float3(_186.data[6], _186.data[7], _186.data[8]), float3(_186.data[9], _186.data[10], _186.data[11])));
    float3 _210 = mul(_205, float4(in_var_Vertex_Position, 1.0f));
    float3 _211 = mul(_205, float4(0.0f, 0.0f, 0.0f, 1.0f));
    uint _220 = uint(((spv_fmsub(_211.x, _211.z, 20.0f) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint _222 = _220 + (_220 << 10u);
    uint _224 = _222 ^ (_222 >> 6u);
    uint _226 = _224 + (_224 << 3u);
    uint _228 = _226 ^ (_226 >> 11u);
    float _234 = asfloat(((_228 + (_228 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float _237 = _210.y + lerp(-0.02500000037252902984619140625f, 0.0f, _234);
    float3 _238 = _210;
    _238.y = _237;
    float2 _239 = _211.xz;
    float4 _245 = grass_object_push.SampleLevel(sampler_default, (_239 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    uint _437;
    uint _438;
    bool _439;
    bool _440;
    float3 _441;
    do
    {
        _259 = frame_constants_glade_theme_id == 4u;
        if (_259)
        {
            _437 = _155;
            _438 = _155;
            _439 = false;
            _440 = false;
            _441 = 0.0f.xxx;
            break;
        }
        float _267 = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        uint _272;
        uint _274;
        bool _276;
        bool _278;
        _272 = _155;
        _274 = _155;
        _276 = false;
        _278 = false;
        float2 _181[4];
        for (uint _269 = 0u; _269 < 4u; )
        {
            float _284 = float(_159[_269]);
            int _286 = int(floor(_284));
            float _287 = frac(_284);
            uint _288 = uint(_286);
            uint _290 = _288 + (_288 << 10u);
            uint _292 = _290 ^ (_290 >> 6u);
            uint _294 = _292 + (_292 << 3u);
            uint _296 = _294 ^ (_294 >> 11u);
            bool _299 = _278 ? _278 : true;
            uint _300 = _278 ? _274 : 1664525u;
            bool _301 = _276 ? _276 : true;
            uint _302 = _276 ? _272 : 1013904223u;
            uint _306 = (((_288 * _300) + (_296 + (_296 << 15u))) + _302) * _300;
            uint _308 = _306 ^ (_306 >> 11u);
            uint _311 = _308 ^ ((_308 << 7u) & 2636928640u);
            uint _314 = _311 ^ ((_311 << 15u) & 4022730752u);
            uint _316 = _314 ^ (_314 >> 18u);
            uint _318 = _316 + (_316 << 10u);
            uint _320 = _318 ^ (_318 >> 6u);
            uint _322 = _320 + (_320 << 3u);
            uint _324 = _322 ^ (_322 >> 11u);
            uint _337 = uint(_286 + 1);
            uint _339 = _337 + (_337 << 10u);
            uint _341 = _339 ^ (_339 >> 6u);
            uint _343 = _341 + (_341 << 3u);
            uint _345 = _343 ^ (_343 >> 11u);
            uint _275 = _299 ? _300 : 1664525u;
            uint _273 = _301 ? _302 : 1013904223u;
            uint _351 = (((_337 * _275) + (_345 + (_345 << 15u))) + _273) * _275;
            uint _353 = _351 ^ (_351 >> 11u);
            uint _356 = _353 ^ ((_353 << 7u) & 2636928640u);
            uint _359 = _356 ^ ((_356 << 15u) & 4022730752u);
            uint _361 = _359 ^ (_359 >> 18u);
            uint _363 = _361 + (_361 << 10u);
            uint _365 = _363 ^ (_363 >> 6u);
            uint _367 = _365 + (_365 << 3u);
            uint _369 = _367 ^ (_367 >> 11u);
            float _384 = (_287 * _287) * (3.0f - (2.0f * _287));
            _181[_269] = (normalize(((float2(asfloat((_316 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_324 + (_324 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - _384)) + (float2(asfloat((_361 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_369 + (_369 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _384)) - 0.5f.xx) * _267) * 3.0f;
            _269++;
            _272 = _273;
            _274 = _275;
            _276 = _301 ? _301 : true;
            _278 = _299 ? _299 : true;
            continue;
        }
        float3 _395;
        _395 = 0.0f.xxx;
        for (uint _398 = 0u; _398 < 1u; )
        {
            uint _399 = _398 + 1u;
            float _403 = float(_399) * 10.16640758514404296875f;
            float _404 = cos(_403);
            float _405 = sin(_403);
            float2x2 _409 = float2x2(float2(_404, _405), float2(-_405, _404));
            float2 _436 = _395.yz + mul(((((water_noise_texture.SampleLevel(sampler_llr, mul(_409, (_239 * _157[_398]) + _181[_398]) * 0.01200000010430812835693359375f, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * _158[_398]) * float2(1.0f, _157[_398] * 0.01200000010430812835693359375f).xyy).yz, _409);
            _395 = float3(_395.x, _436.x, _436.y);
            _398 = _399;
            continue;
        }
        _437 = _272;
        _438 = _274;
        _439 = _276;
        _440 = _278;
        _441 = _395;
        break;
    } while(false);
    float2 _447 = _238.xz + (((float3(_245.x, 0.0f, _245.y) * ((clamp(_245.z, 0.0f, 1.0f) > 0.0f) ? 1.0f : 0.0f)) * 0.3499999940395355224609375f) + (_441.yxz * float3(2.0f, 0.00999999977648258209228515625f, 2.0f))).xz;
    float3 _448 = float3(_447.x, _238.y, _447.y);
    float _450 = in_var_Vertex_Position.y + 0.5f;
    float2 _461 = (_239 * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 _463 = int2(floor(_461));
    float2 _464 = frac(_461);
    uint2 _465 = uint2(_463);
    uint _467 = _465.y;
    uint _469 = _467 + (_467 << 10u);
    uint _471 = _469 ^ (_469 >> 6u);
    uint _473 = _471 + (_471 << 3u);
    uint _475 = _473 ^ (_473 >> 11u);
    bool _478 = _440 ? _440 : true;
    uint _479 = _440 ? _438 : 1664525u;
    bool _480 = _439 ? _439 : true;
    uint _481 = _439 ? _437 : 1013904223u;
    uint _485 = (((_465.x * _479) + (_475 + (_475 << 15u))) + _481) * _479;
    uint _487 = _485 ^ (_485 >> 11u);
    uint _490 = _487 ^ ((_487 << 7u) & 2636928640u);
    uint _493 = _490 ^ ((_490 << 15u) & 4022730752u);
    uint2 _501 = uint2(_463 + int2(1, 0));
    uint _503 = _501.y;
    uint _505 = _503 + (_503 << 10u);
    uint _507 = _505 ^ (_505 >> 6u);
    uint _509 = _507 + (_507 << 3u);
    uint _511 = _509 ^ (_509 >> 11u);
    bool _514 = _478 ? _478 : true;
    uint _515 = _478 ? _479 : 1664525u;
    bool _516 = _480 ? _480 : true;
    uint _517 = _480 ? _481 : 1013904223u;
    uint _521 = (((_501.x * _515) + (_511 + (_511 << 15u))) + _517) * _515;
    uint _523 = _521 ^ (_521 >> 11u);
    uint _526 = _523 ^ ((_523 << 7u) & 2636928640u);
    uint _529 = _526 ^ ((_526 << 15u) & 4022730752u);
    uint2 _537 = uint2(_463 + int2(0, 1));
    uint _539 = _537.y;
    uint _541 = _539 + (_539 << 10u);
    uint _543 = _541 ^ (_541 >> 6u);
    uint _545 = _543 + (_543 << 3u);
    uint _547 = _545 ^ (_545 >> 11u);
    uint _551 = _514 ? _515 : 1664525u;
    uint _553 = _516 ? _517 : 1013904223u;
    uint _557 = (((_537.x * _551) + (_547 + (_547 << 15u))) + _553) * _551;
    uint _559 = _557 ^ (_557 >> 11u);
    uint _562 = _559 ^ ((_559 << 7u) & 2636928640u);
    uint _565 = _562 ^ ((_562 << 15u) & 4022730752u);
    uint2 _573 = uint2(_463 + int2(1, 1));
    uint _575 = _573.y;
    uint _577 = _575 + (_575 << 10u);
    uint _579 = _577 ^ (_577 >> 6u);
    uint _581 = _579 + (_579 << 3u);
    uint _583 = _581 ^ (_581 >> 11u);
    uint _586 = (_514 ? _514 : true) ? _551 : 1664525u;
    uint _591 = (((_573.x * _586) + (_583 + (_583 << 15u))) + ((_516 ? _516 : true) ? _553 : 1013904223u)) * _586;
    uint _593 = _591 ^ (_591 >> 11u);
    uint _596 = _593 ^ ((_593 << 7u) & 2636928640u);
    uint _599 = _596 ^ ((_596 << 15u) & 4022730752u);
    float2 _609 = (_464 * _464) * (3.0f.xx - (_464 * 2.0f));
    float _610 = _609.x;
    float _611 = 1.0f - _610;
    float _613 = _609.y;
    uint _631 = uint(int(_234));
    uint _633 = _631 + (_631 << 10u);
    uint _635 = _633 ^ (_633 >> 6u);
    uint _637 = _635 + (_635 << 3u);
    uint _639 = _637 ^ (_637 >> 11u);
    float _650 = _447.x - (((((_450 * _450) * (sin((((((1.0f - _613) * (((asfloat(((_493 ^ (_493 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _611) + ((asfloat(((_529 ^ (_529 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _610))) + (((asfloat(((_565 ^ (_565 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _611) * _613)) + (((asfloat(((_599 ^ (_599 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _610) * _613)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * (_259 ? 0.02500000037252902984619140625f : 0.0500000007450580596923828125f)) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((_639 + (_639 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    _448.x = _650;
    float3 _660 = lerp(1.2000000476837158203125f, 2.0f, _234).xxx;
    float3 _661 = pow(in_var_Vertex_Color, _660);
    float3 _673;
    if (_259)
    {
        float3 _672;
        if (in_var_Vertex_Color.y > 0.0500000007450580596923828125f)
        {
            _672 = pow(float3(0.1500000059604644775390625f, 0.0900000035762786865234375f, 0.0300000011920928955078125f), _660);
        }
        else
        {
            _672 = _661;
        }
        _673 = _672;
    }
    else
    {
        _673 = _661;
    }
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_650, _237, _447.y, 1.0f));
    out_var_C1 = float3(0.0f, 1.0f, 0.0f);
    out_var_C2 = _448;
    out_var_C3 = _673;
    out_var_C4 = (in_var_Vertex_Position.xz * 1.5f) + 0.5f.xx;
    out_var_C5 = _234;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
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

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> lilypad_alpha : register(t21, space0);

static float3 in_var_C1;
static float3 in_var_C2;
static float3 in_var_C3;
static float2 in_var_C4;
static float in_var_C5;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2: Vertex_Color;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 in_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float in_var_C5 : TEXCOORD5;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    if (lilypad_alpha.SampleBias(sampler_llr, in_var_C4, frame_constants_global_mip_bias).x < lerp(0.0f, 0.89999997615814208984375f, in_var_C5))
    {
        discard;
    }
    bool _112;
    if (frame_constants_glade_theme_id == 4u)
    {
        _112 = in_var_C2.y > 0.0f;
    }
    else
    {
        _112 = false;
    }
    float3 _120;
    if (_112)
    {
        _120 = lerp(in_var_C3, 0.699999988079071044921875f.xxx, smoothstep(-0.20000000298023223876953125f, 1.0f, clamp(in_var_C1.y, 0.0f, 1.0f)).xxx);
    }
    else
    {
        _120 = in_var_C3;
    }
    float3 _122 = (in_var_C1 * 0.5f) + 0.5f.xxx;
    float3 _144 = sqrt(clamp(_120, 0.0f.xxx, 1.0f.xxx));
    out_var_SV_TARGET0 = uint2(((uint((clamp(_122.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_122.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_122.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u, ((uint((clamp(_144.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_144.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_144.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u);
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C4 = stage_input.in_var_C4;
    in_var_C5 = stage_input.in_var_C5;
    frag_main();
    SPIRV_Cross_Output stage_output;
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

template<typename T> T spv_fmsub(T a, T b, float c) { precise T p = a * c; precise T q = b * c; precise T r = p - q; return r; }
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

struct Reed
{
    Affine3Packed xform;
};

static uint _155;

static const float _157[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float _158[4] = { 2.0f, 2.0f, 0.5f, 0.20000000298023223876953125f };
static const uint _159[4] = { 17u, 18u, 20u, 21u };

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
    uint2 _Globals_water_noise_texture_dims : packoffset(c0.y);
    uint2 _Globals_grass_object_push_dims : packoffset(c1);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c1.z);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c2);
    uint2 _Globals_lilypad_normal_dims : packoffset(c2.z);
    uint2 _Globals_lilypad_alpha_dims : packoffset(c3);
    int _Globals_bool_render_shadows : packoffset(c3.z);
};

ByteAddressBuffer instance_buffer : register(t17, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> water_noise_texture : register(t15, space0);
Texture2D<float4> grass_object_push : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static float2 out_var_C4;
static float out_var_C5;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(2)]] float3 in_var_Vertex_Color: Vertex_Color;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float out_var_C5 : TEXCOORD5;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    bool _259;
    Affine3Packed _186;
    [unroll]
    for (int _3ident = 0; _3ident < 12; _3ident++)
    {
        _186.data[_3ident] = instance_buffer.Load<float>(_3ident * 4 + uint(gl_InstanceIndex) * 48 + 0);
    }
    float3x4 _205 = transpose(float4x3(float3(_186.data[0], _186.data[1], _186.data[2]), float3(_186.data[3], _186.data[4], _186.data[5]), float3(_186.data[6], _186.data[7], _186.data[8]), float3(_186.data[9], _186.data[10], _186.data[11])));
    float3 _210 = mul(_205, float4(in_var_Vertex_Position, 1.0f));
    float3 _211 = mul(_205, float4(0.0f, 0.0f, 0.0f, 1.0f));
    uint _220 = uint(((spv_fmsub(_211.x, _211.z, 20.0f) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint _222 = _220 + (_220 << 10u);
    uint _224 = _222 ^ (_222 >> 6u);
    uint _226 = _224 + (_224 << 3u);
    uint _228 = _226 ^ (_226 >> 11u);
    float _234 = asfloat(((_228 + (_228 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float _237 = _210.y + lerp(-0.02500000037252902984619140625f, 0.0f, _234);
    float3 _238 = _210;
    _238.y = _237;
    float2 _239 = _211.xz;
    float4 _245 = grass_object_push.SampleLevel(sampler_default, (_239 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    uint _437;
    uint _438;
    bool _439;
    bool _440;
    float3 _441;
    do
    {
        _259 = frame_constants_glade_theme_id == 4u;
        if (_259)
        {
            _437 = _155;
            _438 = _155;
            _439 = false;
            _440 = false;
            _441 = 0.0f.xxx;
            break;
        }
        float _267 = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        uint _272;
        uint _274;
        bool _276;
        bool _278;
        _272 = _155;
        _274 = _155;
        _276 = false;
        _278 = false;
        float2 _181[4];
        for (uint _269 = 0u; _269 < 4u; )
        {
            float _284 = float(_159[_269]);
            int _286 = int(floor(_284));
            float _287 = frac(_284);
            uint _288 = uint(_286);
            uint _290 = _288 + (_288 << 10u);
            uint _292 = _290 ^ (_290 >> 6u);
            uint _294 = _292 + (_292 << 3u);
            uint _296 = _294 ^ (_294 >> 11u);
            bool _299 = _278 ? _278 : true;
            uint _300 = _278 ? _274 : 1664525u;
            bool _301 = _276 ? _276 : true;
            uint _302 = _276 ? _272 : 1013904223u;
            uint _306 = (((_288 * _300) + (_296 + (_296 << 15u))) + _302) * _300;
            uint _308 = _306 ^ (_306 >> 11u);
            uint _311 = _308 ^ ((_308 << 7u) & 2636928640u);
            uint _314 = _311 ^ ((_311 << 15u) & 4022730752u);
            uint _316 = _314 ^ (_314 >> 18u);
            uint _318 = _316 + (_316 << 10u);
            uint _320 = _318 ^ (_318 >> 6u);
            uint _322 = _320 + (_320 << 3u);
            uint _324 = _322 ^ (_322 >> 11u);
            uint _337 = uint(_286 + 1);
            uint _339 = _337 + (_337 << 10u);
            uint _341 = _339 ^ (_339 >> 6u);
            uint _343 = _341 + (_341 << 3u);
            uint _345 = _343 ^ (_343 >> 11u);
            uint _275 = _299 ? _300 : 1664525u;
            uint _273 = _301 ? _302 : 1013904223u;
            uint _351 = (((_337 * _275) + (_345 + (_345 << 15u))) + _273) * _275;
            uint _353 = _351 ^ (_351 >> 11u);
            uint _356 = _353 ^ ((_353 << 7u) & 2636928640u);
            uint _359 = _356 ^ ((_356 << 15u) & 4022730752u);
            uint _361 = _359 ^ (_359 >> 18u);
            uint _363 = _361 + (_361 << 10u);
            uint _365 = _363 ^ (_363 >> 6u);
            uint _367 = _365 + (_365 << 3u);
            uint _369 = _367 ^ (_367 >> 11u);
            float _384 = (_287 * _287) * (3.0f - (2.0f * _287));
            _181[_269] = (normalize(((float2(asfloat((_316 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_324 + (_324 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - _384)) + (float2(asfloat((_361 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_369 + (_369 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _384)) - 0.5f.xx) * _267) * 3.0f;
            _269++;
            _272 = _273;
            _274 = _275;
            _276 = _301 ? _301 : true;
            _278 = _299 ? _299 : true;
            continue;
        }
        float3 _395;
        _395 = 0.0f.xxx;
        for (uint _398 = 0u; _398 < 1u; )
        {
            uint _399 = _398 + 1u;
            float _403 = float(_399) * 10.16640758514404296875f;
            float _404 = cos(_403);
            float _405 = sin(_403);
            float2x2 _409 = float2x2(float2(_404, _405), float2(-_405, _404));
            float2 _436 = _395.yz + mul(((((water_noise_texture.SampleLevel(sampler_llr, mul(_409, (_239 * _157[_398]) + _181[_398]) * 0.01200000010430812835693359375f, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * _158[_398]) * float2(1.0f, _157[_398] * 0.01200000010430812835693359375f).xyy).yz, _409);
            _395 = float3(_395.x, _436.x, _436.y);
            _398 = _399;
            continue;
        }
        _437 = _272;
        _438 = _274;
        _439 = _276;
        _440 = _278;
        _441 = _395;
        break;
    } while(false);
    float2 _447 = _238.xz + (((float3(_245.x, 0.0f, _245.y) * ((clamp(_245.z, 0.0f, 1.0f) > 0.0f) ? 1.0f : 0.0f)) * 0.3499999940395355224609375f) + (_441.yxz * float3(2.0f, 0.00999999977648258209228515625f, 2.0f))).xz;
    float3 _448 = float3(_447.x, _238.y, _447.y);
    float _450 = in_var_Vertex_Position.y + 0.5f;
    float2 _461 = (_239 * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 _463 = int2(floor(_461));
    float2 _464 = frac(_461);
    uint2 _465 = uint2(_463);
    uint _467 = _465.y;
    uint _469 = _467 + (_467 << 10u);
    uint _471 = _469 ^ (_469 >> 6u);
    uint _473 = _471 + (_471 << 3u);
    uint _475 = _473 ^ (_473 >> 11u);
    bool _478 = _440 ? _440 : true;
    uint _479 = _440 ? _438 : 1664525u;
    bool _480 = _439 ? _439 : true;
    uint _481 = _439 ? _437 : 1013904223u;
    uint _485 = (((_465.x * _479) + (_475 + (_475 << 15u))) + _481) * _479;
    uint _487 = _485 ^ (_485 >> 11u);
    uint _490 = _487 ^ ((_487 << 7u) & 2636928640u);
    uint _493 = _490 ^ ((_490 << 15u) & 4022730752u);
    uint2 _501 = uint2(_463 + int2(1, 0));
    uint _503 = _501.y;
    uint _505 = _503 + (_503 << 10u);
    uint _507 = _505 ^ (_505 >> 6u);
    uint _509 = _507 + (_507 << 3u);
    uint _511 = _509 ^ (_509 >> 11u);
    bool _514 = _478 ? _478 : true;
    uint _515 = _478 ? _479 : 1664525u;
    bool _516 = _480 ? _480 : true;
    uint _517 = _480 ? _481 : 1013904223u;
    uint _521 = (((_501.x * _515) + (_511 + (_511 << 15u))) + _517) * _515;
    uint _523 = _521 ^ (_521 >> 11u);
    uint _526 = _523 ^ ((_523 << 7u) & 2636928640u);
    uint _529 = _526 ^ ((_526 << 15u) & 4022730752u);
    uint2 _537 = uint2(_463 + int2(0, 1));
    uint _539 = _537.y;
    uint _541 = _539 + (_539 << 10u);
    uint _543 = _541 ^ (_541 >> 6u);
    uint _545 = _543 + (_543 << 3u);
    uint _547 = _545 ^ (_545 >> 11u);
    uint _551 = _514 ? _515 : 1664525u;
    uint _553 = _516 ? _517 : 1013904223u;
    uint _557 = (((_537.x * _551) + (_547 + (_547 << 15u))) + _553) * _551;
    uint _559 = _557 ^ (_557 >> 11u);
    uint _562 = _559 ^ ((_559 << 7u) & 2636928640u);
    uint _565 = _562 ^ ((_562 << 15u) & 4022730752u);
    uint2 _573 = uint2(_463 + int2(1, 1));
    uint _575 = _573.y;
    uint _577 = _575 + (_575 << 10u);
    uint _579 = _577 ^ (_577 >> 6u);
    uint _581 = _579 + (_579 << 3u);
    uint _583 = _581 ^ (_581 >> 11u);
    uint _586 = (_514 ? _514 : true) ? _551 : 1664525u;
    uint _591 = (((_573.x * _586) + (_583 + (_583 << 15u))) + ((_516 ? _516 : true) ? _553 : 1013904223u)) * _586;
    uint _593 = _591 ^ (_591 >> 11u);
    uint _596 = _593 ^ ((_593 << 7u) & 2636928640u);
    uint _599 = _596 ^ ((_596 << 15u) & 4022730752u);
    float2 _609 = (_464 * _464) * (3.0f.xx - (_464 * 2.0f));
    float _610 = _609.x;
    float _611 = 1.0f - _610;
    float _613 = _609.y;
    uint _631 = uint(int(_234));
    uint _633 = _631 + (_631 << 10u);
    uint _635 = _633 ^ (_633 >> 6u);
    uint _637 = _635 + (_635 << 3u);
    uint _639 = _637 ^ (_637 >> 11u);
    float _650 = _447.x - (((((_450 * _450) * (sin((((((1.0f - _613) * (((asfloat(((_493 ^ (_493 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _611) + ((asfloat(((_529 ^ (_529 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _610))) + (((asfloat(((_565 ^ (_565 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _611) * _613)) + (((asfloat(((_599 ^ (_599 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _610) * _613)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * (_259 ? 0.02500000037252902984619140625f : 0.0500000007450580596923828125f)) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((_639 + (_639 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    _448.x = _650;
    float3 _660 = lerp(1.2000000476837158203125f, 2.0f, _234).xxx;
    float3 _661 = pow(in_var_Vertex_Color, _660);
    float3 _673;
    if (_259)
    {
        float3 _672;
        if (in_var_Vertex_Color.y > 0.0500000007450580596923828125f)
        {
            _672 = pow(float3(0.1500000059604644775390625f, 0.0900000035762786865234375f, 0.0300000011920928955078125f), _660);
        }
        else
        {
            _672 = _661;
        }
        _673 = _672;
    }
    else
    {
        _673 = _661;
    }
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_650, _237, _447.y, 1.0f));
    out_var_C1 = float3(0.0f, 1.0f, 0.0f);
    out_var_C2 = _448;
    out_var_C3 = _673;
    out_var_C4 = (in_var_Vertex_Position.xz * 1.5f) + 0.5f.xx;
    out_var_C5 = _234;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
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

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> lilypad_alpha : register(t21, space0);

static float2 in_var_C4;
static float in_var_C5;

struct SPIRV_Cross_Input
{
    [[vk::location(4)]] float2 in_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float in_var_C5 : TEXCOORD5;
};

void frag_main()
{
    if (lilypad_alpha.SampleBias(sampler_llr, in_var_C4, frame_constants_global_mip_bias).x < lerp(0.0f, 0.89999997615814208984375f, in_var_C5))
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C4 = stage_input.in_var_C4;
    in_var_C5 = stage_input.in_var_C5;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
