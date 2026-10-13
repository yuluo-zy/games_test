#define SHADER_VARIANT_DEPTH_ONLY 1
#define TGV_f0adff76827e73ff 1
#line 1 "shaders/star.raster.hlsl"


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
#line 9 "shaders/star.raster.hlsl"
            
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
#line 10 "shaders/star.raster.hlsl"

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
#line 11 "shaders/star.raster.hlsl"

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
#line 12 "shaders/star.raster.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 13 "shaders/star.raster.hlsl"

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
#line 14 "shaders/star.raster.hlsl"

#line 1 "shaders/inc/hammersley.hlsl"










float radicalInverse_VdC(uint bits)
{
    bits = (bits << 16u) | (bits >> 16u);
    bits = ((bits & 0x55555555u) << 1u) | ((bits & 0xAAAAAAAAu) >> 1u);
    bits = ((bits & 0x33333333u) << 2u) | ((bits & 0xCCCCCCCCu) >> 2u);
    bits = ((bits & 0x0F0F0F0Fu) << 4u) | ((bits & 0xF0F0F0F0u) >> 4u);
    bits = ((bits & 0x00FF00FFu) << 8u) | ((bits & 0xFF00FF00u) >> 8u);
    return float(bits) * 2.3283064365386963e-10f;
}

float2 hammersley(uint i, uint N)
{
    return float2(float(i) / float(N), radicalInverse_VdC(i));
}
#line 15 "shaders/star.raster.hlsl"

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
#line 16 "shaders/star.raster.hlsl"

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
#line 17 "shaders/star.raster.hlsl"

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
#line 18 "shaders/star.raster.hlsl"

#line 1 "shaders/inc/simplex_noise.hlsl"











float incb_mod289(float x)
{
    return fmod(x, 289.0f);
}

float3 incb_mod289(float3 x)
{
    return fmod(x, 289.0f);
}

float incb_permute(float x)
{
    return fmod((x * 34.0f + 1.0f) * x, 289.0f);
}

float3 incb_permute(float3 x)
{
    return fmod((x * 34.0f + 1.0f) * x, 289.0f);
}

float snoise(float2 v)
{
    const float2 C = float2(0.211324865405187f, 0.366025403784439f); 
    const float2 Cz = float2(-0.577350269189626f, -0.577350269189626f); 

    float2 i = floor(v + dot(v, C.yy));
    float2 x0 = v - i + dot(i, C.xx);

    float2 g = step(x0.yx, x0.xy);
    float2 l = 1.0f - g;
    float2 i1 = min(g.xy, l.yx);
    float2 i2 = max(g.xy, l.yx);

    float2 x1 = x0 - i1 + C.xx;
    float2 x2 = x0 - i2 + Cz;

    float i4x = incb_mod289(i.x);
    float i4y = incb_mod289(i.y);

    float3 p0 = incb_permute(incb_permute(i4y + float3(0.0f, i1.y, 1.0f)) + i4x + float3(0.0f, i1.x, 1.0f));

    float3 m = max(0.5f - float3(dot(x0, x0), dot(x1, x1), dot(x2, x2)), 0.0f);
    m = m * m;
    m = m * m;

    float3 n = frac(incb_permute(p0) * (1.0f / 41.0f)) * 2.0f;
    n = n - 1.0f;
    float3 s = abs(n) - 0.5f;
    float3 t = n - floor(n + 0.5f);

    float3 grad = float3(t.x * x0.x + s.x * x0.y,
                         t.y * x1.x + s.y * x1.y,
                         t.z * x2.x + s.z * x2.y);

    return 130.0f * dot(m * (1.79284291400159f - (dot(t, t) + dot(s, s)) * 0.85373472095314f), grad);
}
#line 19 "shaders/star.raster.hlsl"

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
#line 20 "shaders/star.raster.hlsl"

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
#line 21 "shaders/star.raster.hlsl"

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
#line 22 "shaders/star.raster.hlsl"

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
#line 23 "shaders/star.raster.hlsl"


cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint2 framebuffer_dims;
    uint2 star_texture_dims;
};

Texture2D<float4> star_texture : register(t14, space0);

struct VsOut
{
    float2 uv : TEXCOORD0;
    nointerpolation uint instance : TEXCOORD1;
    float3 world_pos : TEXCOORD2;
    float3 right : TEXCOORD3;
    float3 facing : TEXCOORD4;
    float4 position : SV_Position;
};

VsOut vs_main(uint vertex_id : SV_VertexID, uint instance_id : SV_InstanceID)
{
    VsOut o;

    float2 quad = (float2(float(vertex_id & 1u), float((vertex_id >> 1u) & 1u)) - 0.5f) * float2(1.0f, 2.0f);

    uint seed = (instance_id * 12u) + 25u;
    uint h1 = hash(seed);
    uint h2 = hash(h1 + (h1 << 10u));
    uint h3 = hash(h2 + (h2 << 10u));

    float fi = float(instance_id);
    float angle = (frac(0.3f + fi * GOLDEN_RATIO) * TWO_PI)
                + ((float(frame_index) * 1e-5f) * lerp(1.0f, 0.07f, rand01(h2)));

    float3 facing = -float3(cos(angle), 0.0f, sin(angle));
    float t = 1.0f - fi * 0.0001666667f;
    float3 center = (-facing) * (10.0f * lerp(800.0f, 2200.0f, t * t));
    center.y = (center.y + 10000.0f * (rand01(h1) * rand01(h1))) - 1000.0f;

    float2 size = (float2(2.0f, 1.0f) * (lerp(1.0f, 0.2f, smoothstep(0.7f, 1.0f, rand01(h1))) * 10.0f)) * 3.0f
                * rand01(hash(h3));
    float3 right = float3(facing.z, 0.0f, -facing.x);

    float3 world = center + (right * (size.x * quad.x)) + (float3(0.0f, 1.0f, 0.0f) * (size.y * quad.y));

    float4 clip = mul(view_constants[current_view_index].world_to_sample, float4(world, 1.0f));
    if (instance_id > 6000u)
        clip = float4(asfloat(0x7fbfffffu).xxxx);

    o.position = clip;
    o.uv = float2(float(vertex_id & 1u), float(1u - ((vertex_id >> 1u) & 1u)));
    o.instance = instance_id;
    o.world_pos = world;
    o.right = right;
    o.facing = facing;
    return o;
}

float4 ps_main(VsOut vin) : SV_Target0
{
    float3 camera_world = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f)).xyz;
    float3 view_dir = normalize(camera_world - vin.world_pos);

    
    float d_night = abs(time_of_day - 0.9f);
    float night = (d_night > 0.1f) ? 0.0f : 1.0f - (d_night * 10.0f) * (d_night * 10.0f) * (3.0f - d_night * 20.0f);
    float d_dusk = abs(time_of_day - 0.45f);
    float dusk = (d_dusk > 0.15f) ? 0.0f : 1.0f - (d_dusk * 6.6667f) * (d_dusk * 6.6667f) * (3.0f - d_dusk * 13.333f);

    float occlusion = smoothstep(-1.0f, 1.0f, dot(view_dir, -mul(view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz));
    float visible = smoothstep(0.4f, 0.6f, time_of_day) - smoothstep(0.8f, 0.92f, time_of_day);

    float star = star_texture.Sample(sampler_lnc, vin.uv).x * (1.0f - occlusion * (night + dusk)) * visible;
    return float4(star.xxx * (10.0f * ev_mult), star);
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
    uint2 _Globals_framebuffer_dims : packoffset(c0.y);
    uint2 _Globals_star_texture_dims : packoffset(c1);
};


static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static float2 out_var_C0;
static uint out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static float3 out_var_C4;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float2 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] nointerpolation uint out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float _103 = float(uint(gl_VertexIndex) & 1u);
    uint _105 = (uint(gl_VertexIndex) >> 1u) & 1u;
    float2 _109 = (float2(_103, float(_105)) - 0.5f.xx) * float2(1.0f, 2.0f);
    uint _111 = (uint(gl_InstanceIndex) * 12u) + 25u;
    uint _113 = _111 + (_111 << 10u);
    uint _115 = _113 ^ (_113 >> 6u);
    uint _117 = _115 + (_115 << 3u);
    uint _119 = _117 ^ (_117 >> 11u);
    uint _121 = _119 + (_119 << 15u);
    float _122 = float(uint(gl_InstanceIndex));
    uint _127 = _121 + (_121 << 10u);
    uint _129 = _127 ^ (_127 >> 6u);
    uint _131 = _129 + (_129 << 3u);
    uint _133 = _131 ^ (_131 >> 11u);
    uint _135 = _133 + (_133 << 15u);
    float _139 = asfloat((_121 & 8388607u) | 1065353216u) - 1.0f;
    uint _141 = _135 + (_135 << 10u);
    uint _143 = _141 ^ (_141 >> 6u);
    uint _145 = _143 + (_143 << 3u);
    uint _147 = _145 ^ (_145 >> 11u);
    uint _149 = _147 + (_147 << 15u);
    uint _155 = _149 + (_149 << 10u);
    uint _157 = _155 ^ (_155 >> 6u);
    uint _159 = _157 + (_157 << 3u);
    uint _161 = _159 ^ (_159 >> 11u);
    float _171 = (frac(0.300000011920928955078125f + (_122 * 1.61803400516510009765625f)) * 6.283185482025146484375f) + ((float(frame_constants_frame_index) * 9.9999997473787516355514526367188e-06f) * lerp(1.0f, 0.070000000298023223876953125f, asfloat((_135 & 8388607u) | 1065353216u) - 1.0f));
    float2 _175 = -float2(cos(_171), sin(_171));
    float _176 = _175.y;
    float _177 = _175.x;
    float3 _179 = float3(_177, 0.0f, _176);
    float _182 = 1.0f - (_122 * 0.00016666666488163173198699951171875f);
    float3 _186 = (-_179) * (10.0f * lerp(800.0f, 2200.0f, _182 * _182));
    _186.y = (_186.y + (10000.0f * (_139 * _139))) - 1000.0f;
    float2 _202 = ((float2(2.0f, 1.0f) * (lerp(1.0f, 0.20000000298023223876953125f, smoothstep(0.699999988079071044921875f, 1.0f, _139)) * 10.0f)) * 3.0f) * (asfloat(((_161 + (_161 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
    float3 _203 = float3(_176, 0.0f, -_177);
    float3 _213 = _186 + ((_203 * (_202.x * _109.x)) + (float3(0.0f, 1.0f, 0.0f) * (_202.y * _109.y)));
    float4 _226 = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_213, 1.0f));
    bool4 _228 = (uint(gl_InstanceIndex) > 6000u).xxxx;
    gl_Position = float4(_228.x ? asfloat(0x7fbfffffu          ).xxxx.x : _226.x, _228.y ? asfloat(0x7fbfffffu          ).xxxx.y : _226.y, _228.z ? asfloat(0x7fbfffffu          ).xxxx.z : _226.z, _228.w ? asfloat(0x7fbfffffu          ).xxxx.w : _226.w);
    out_var_C0 = float2(_103, float(1u - _105));
    out_var_C1 = uint(gl_InstanceIndex);
    out_var_C2 = _213;
    out_var_C3 = _203;
    out_var_C4 = _179;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
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

cbuffer $Globals : register(b13, space0)
{
    uint _Globals_current_view_index : packoffset(c0);
    uint2 _Globals_framebuffer_dims : packoffset(c0.y);
    uint2 _Globals_star_texture_dims : packoffset(c1);
};

SamplerState sampler_lnc : register(s0, space0);
Texture2D<float4> star_texture : register(t14, space0);

static float2 in_var_C0;
static float3 in_var_C2;
static float4 out_var_SV_TARGET;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float2 in_var_C0 : TEXCOORD0;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
};

struct SPIRV_Cross_Output
{
    float4 out_var_SV_TARGET : SV_Target0;
};

void frag_main()
{
    float4 _94 = mul(frame_constants_view_constants[_Globals_current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float _121;
    do
    {
        float _111 = abs(frame_constants_time_of_day - 0.89999997615814208984375f);
        if (_111 > 0.100000001490116119384765625f)
        {
            _121 = 0.0f;
            break;
        }
        float _115 = _111 * 10.0f;
        _121 = 1.0f - ((_115 * _115) * (3.0f - (_111 * 20.0f)));
        break;
    } while(false);
    float _135;
    do
    {
        float _125 = abs(frame_constants_time_of_day - 0.449999988079071044921875f);
        if (_125 > 0.1500000059604644775390625f)
        {
            _135 = 0.0f;
            break;
        }
        float _129 = _125 * 6.666666507720947265625f;
        _135 = 1.0f - ((_129 * _129) * (3.0f - (_125 * 13.33333301544189453125f)));
        break;
    } while(false);
    float4 _152 = 1.0f.xxxx * ((star_texture.Sample(sampler_lnc, in_var_C0).x * (1.0f - (smoothstep(-1.0f, 1.0f, dot(normalize((_94.xyz / _94.w.xxx) - in_var_C2), -mul(frame_constants_view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz)) * (_121 + _135)))) * (smoothstep(0.4000000059604644775390625f, 0.60000002384185791015625f, frame_constants_time_of_day) - smoothstep(0.800000011920928955078125f, 0.920000016689300537109375f, frame_constants_time_of_day)));
    float3 _157 = _152.xyz * (10.0f * frame_constants_ev_mult);
    out_var_SV_TARGET = float4(_157.x, _157.y, _157.z, _152.w);
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C0 = stage_input.in_var_C0;
    in_var_C2 = stage_input.in_var_C2;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET = out_var_SV_TARGET;
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
    uint2 _Globals_framebuffer_dims : packoffset(c0.y);
    uint2 _Globals_star_texture_dims : packoffset(c1);
};


static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static float2 out_var_C0;
static uint out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static float3 out_var_C4;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float2 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] nointerpolation uint out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float _103 = float(uint(gl_VertexIndex) & 1u);
    uint _105 = (uint(gl_VertexIndex) >> 1u) & 1u;
    float2 _109 = (float2(_103, float(_105)) - 0.5f.xx) * float2(1.0f, 2.0f);
    uint _111 = (uint(gl_InstanceIndex) * 12u) + 25u;
    uint _113 = _111 + (_111 << 10u);
    uint _115 = _113 ^ (_113 >> 6u);
    uint _117 = _115 + (_115 << 3u);
    uint _119 = _117 ^ (_117 >> 11u);
    uint _121 = _119 + (_119 << 15u);
    float _122 = float(uint(gl_InstanceIndex));
    uint _127 = _121 + (_121 << 10u);
    uint _129 = _127 ^ (_127 >> 6u);
    uint _131 = _129 + (_129 << 3u);
    uint _133 = _131 ^ (_131 >> 11u);
    uint _135 = _133 + (_133 << 15u);
    float _139 = asfloat((_121 & 8388607u) | 1065353216u) - 1.0f;
    uint _141 = _135 + (_135 << 10u);
    uint _143 = _141 ^ (_141 >> 6u);
    uint _145 = _143 + (_143 << 3u);
    uint _147 = _145 ^ (_145 >> 11u);
    uint _149 = _147 + (_147 << 15u);
    uint _155 = _149 + (_149 << 10u);
    uint _157 = _155 ^ (_155 >> 6u);
    uint _159 = _157 + (_157 << 3u);
    uint _161 = _159 ^ (_159 >> 11u);
    float _171 = (frac(0.300000011920928955078125f + (_122 * 1.61803400516510009765625f)) * 6.283185482025146484375f) + ((float(frame_constants_frame_index) * 9.9999997473787516355514526367188e-06f) * lerp(1.0f, 0.070000000298023223876953125f, asfloat((_135 & 8388607u) | 1065353216u) - 1.0f));
    float2 _175 = -float2(cos(_171), sin(_171));
    float _176 = _175.y;
    float _177 = _175.x;
    float3 _179 = float3(_177, 0.0f, _176);
    float _182 = 1.0f - (_122 * 0.00016666666488163173198699951171875f);
    float3 _186 = (-_179) * (10.0f * lerp(800.0f, 2200.0f, _182 * _182));
    _186.y = (_186.y + (10000.0f * (_139 * _139))) - 1000.0f;
    float2 _202 = ((float2(2.0f, 1.0f) * (lerp(1.0f, 0.20000000298023223876953125f, smoothstep(0.699999988079071044921875f, 1.0f, _139)) * 10.0f)) * 3.0f) * (asfloat(((_161 + (_161 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
    float3 _203 = float3(_176, 0.0f, -_177);
    float3 _213 = _186 + ((_203 * (_202.x * _109.x)) + (float3(0.0f, 1.0f, 0.0f) * (_202.y * _109.y)));
    float4 _226 = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_213, 1.0f));
    bool4 _228 = (uint(gl_InstanceIndex) > 6000u).xxxx;
    gl_Position = float4(_228.x ? asfloat(0x7fbfffffu          ).xxxx.x : _226.x, _228.y ? asfloat(0x7fbfffffu          ).xxxx.y : _226.y, _228.z ? asfloat(0x7fbfffffu          ).xxxx.z : _226.z, _228.w ? asfloat(0x7fbfffffu          ).xxxx.w : _226.w);
    out_var_C0 = float2(_103, float(1u - _105));
    out_var_C1 = uint(gl_InstanceIndex);
    out_var_C2 = _213;
    out_var_C3 = _203;
    out_var_C4 = _179;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
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
