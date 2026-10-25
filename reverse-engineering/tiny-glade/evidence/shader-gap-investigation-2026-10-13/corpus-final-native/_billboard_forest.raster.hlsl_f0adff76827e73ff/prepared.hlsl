#define SHADER_VARIANT_DEPTH_ONLY 1
#define TGV_f0adff76827e73ff 1
#line 1 "shaders/billboard_forest.raster.hlsl"


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
#line 13 "shaders/billboard_forest.raster.hlsl"

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
#line 14 "shaders/billboard_forest.raster.hlsl"

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
#line 15 "shaders/billboard_forest.raster.hlsl"

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
#line 16 "shaders/billboard_forest.raster.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 17 "shaders/billboard_forest.raster.hlsl"

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
#line 18 "shaders/billboard_forest.raster.hlsl"

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
#line 19 "shaders/billboard_forest.raster.hlsl"

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
#line 20 "shaders/billboard_forest.raster.hlsl"

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
#line 21 "shaders/billboard_forest.raster.hlsl"

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
#line 22 "shaders/billboard_forest.raster.hlsl"



#define TINYGLADE_INC_COLOR_LIBRARY_HLSL
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
#line 26 "shaders/billboard_forest.raster.hlsl"

cbuffer color_library : register(b18, space0)
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
#line 67 "shaders/billboard_forest.raster.hlsl"

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
#line 68 "shaders/billboard_forest.raster.hlsl"

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
#line 69 "shaders/billboard_forest.raster.hlsl"

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
#line 70 "shaders/billboard_forest.raster.hlsl"

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
#line 71 "shaders/billboard_forest.raster.hlsl"

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
#line 72 "shaders/billboard_forest.raster.hlsl"

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
#line 73 "shaders/billboard_forest.raster.hlsl"

#line 1 "shaders/inc/sh.hlsl"










float4 sh_basis(float3 dir)
{
    return float4(0.2820949852466583251953125f, 0.48860299587249755859375f * dir);
}

float sh_eval(float4 sh, float3 dir)
{
    return dot(sh, sh_basis(dir));
}
#line 74 "shaders/billboard_forest.raster.hlsl"







uint2 framebuffer_dims;
uint2 cloud1_dims;
uint2 cloud2_dims;
uint2 cloud3_dims;
uint2 cloud4_dims;

Texture2D<float4> cloud1 : register(t14, space0);
Texture2D<float4> cloud2 : register(t15, space0);
Texture2D<float4> cloud3 : register(t16, space0);
Texture2D<float4> cloud4 : register(t17, space0);

struct VS_OUT
{
    float2 C0 : TEXCOORD0;
    nointerpolation uint C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float3 C4 : TEXCOORD4;
    float4 position : SV_Position;
};

VS_OUT vs_main(uint vertex_id : SV_VertexID, uint instance_id : SV_InstanceID)
{
    VS_OUT o;
    float vx = float(vertex_id & 1u);
    float vy = float((vertex_id >> 1u) & 1u);
    float2 quad = (float2(vx, vy) - 0.5f) * float2(1.0f, 2.0f);
    uint seed = (instance_id * 12u) + 25u;
    float inst = float(instance_id);

    float angle = hash_angle(seed);
    float2 facing = -float2(cos(angle), sin(angle));
    float2 facing_perp = float2(facing.y, -facing.x);
    float3 dir = float3(facing.x, 0.0f, facing.y);
    float dist_f = 1.0f - (inst * 0.002f);
    float dist2 = dist_f * dist_f;
    float3 base = (-dir) * lerp(450.0f, 550.0f, dist2);
    base.y -= lerp(35.0f, 45.0f, dist2);

    uint h1 = hash(seed);
    uint h2 = hash(h1);
    uint h3 = hash(h2);
    uint h4 = hash(h3);
    float2 size = float2(36.0f, 18.0f) * lerp(0.9f, 1.2f, rand01(h4));
    int type = int(frac(inst * 0.25f) * 4.0f);
    float2 size2;
    if (type == 1)
    {
        size2 = size * 1.2f;
        size2.y *= 0.7f;
    }
    else if (type == 2)
    {
        size2 = size * 0.6f;
    }
    else if (type == 3)
    {
        size2 = size * 1.0f;
        size2.y *= 0.9f;
    }
    else
    {
        size2 = size;
    }

    uint h5 = hash(h4);
    uint h6 = hash(h5);
    uint h7 = hash(h6);
    float3 pos = base
        + (float3(facing.y, 0.0f, -facing.x) * ((size2.x * quad.x) * lerp(1.5f, 2.0f, rand01(h6))))
        + (float3(0.0f, lerp(1.5f, 2.5f, rand01(h5)), 0.0f) * (size2.y * quad.y));

    float2 uv_base = float2(vx, 1.0f - vy);
    float2 uv_out;
    float2 facing_out;
    if (rand01(h7) > 0.5f)
    {
        uv_out = float2(1.0f - vx, uv_base.y);
        facing_out = -facing_perp;
    }
    else
    {
        uv_out = uv_base;
        facing_out = facing_perp;
    }

    float4 clip = mul(view_constants[current_view_index].world_to_sample, float4(pos, 1.0f));
    if (instance_id > 500u)
    {
        clip = asfloat(0x7fbfffffu);
    }
    o.position = clip;
    o.C0 = uv_out;
    o.C1 = instance_id;
    o.C2 = pos;
    o.C3 = float3(facing_out.x, 0.0f, facing_out.y);
    o.C4 = dir;
    return o;
}

struct PS_IN
{
    float2 C0 : TEXCOORD0;
    nointerpolation uint C1 : TEXCOORD1;
    float3 C3 : TEXCOORD3;
    float3 C4 : TEXCOORD4;
};

struct PS_OUT
{
    uint2 target : SV_Target0;
};

PS_OUT ps_main(PS_IN i)
{
    PS_OUT o;
    int type = int(frac(float(i.C1) * 0.25f) * 4.0f);
    float s;
    float t;
    float u;
    float v;
    float w;
    if (type == 1)
    {
        float sx = i.C0.x * 0.3333333432674407958984375f;
        float sy = i.C0.y * 0.3333333432674407958984375f;
        s = cloud2.Sample(sampler_lnc, float2(sx, sy + 0.666666686534881591796875f)).x;
        t = cloud2.Sample(sampler_lnc, float2(sx, sy + 0.3333333432674407958984375f)).x;
        u = cloud2.Sample(sampler_lnc, float2(sx + 0.666666686534881591796875f, sy)).x;
        v = cloud2.Sample(sampler_lnc, float2(sx + 0.3333333432674407958984375f, sy)).x;
        w = cloud2.Sample(sampler_lnc, float2(sx, sy)).x;
    }
    else if (type == 2)
    {
        float sx = i.C0.x * 0.3333333432674407958984375f;
        float sy = i.C0.y * 0.3333333432674407958984375f;
        s = cloud3.Sample(sampler_lnc, float2(sx, sy + 0.666666686534881591796875f)).x;
        t = cloud3.Sample(sampler_lnc, float2(sx, sy + 0.3333333432674407958984375f)).x;
        u = cloud3.Sample(sampler_lnc, float2(sx + 0.666666686534881591796875f, sy)).x;
        v = cloud3.Sample(sampler_lnc, float2(sx + 0.3333333432674407958984375f, sy)).x;
        w = cloud3.Sample(sampler_lnc, float2(sx, sy)).x;
    }
    else if (type == 3)
    {
        float sx = i.C0.x * 0.3333333432674407958984375f;
        float sy = i.C0.y * 0.3333333432674407958984375f;
        s = cloud4.Sample(sampler_lnc, float2(sx, sy + 0.666666686534881591796875f)).x;
        t = cloud4.Sample(sampler_lnc, float2(sx, sy + 0.3333333432674407958984375f)).x;
        u = cloud4.Sample(sampler_lnc, float2(sx + 0.666666686534881591796875f, sy)).x;
        v = cloud4.Sample(sampler_lnc, float2(sx + 0.3333333432674407958984375f, sy)).x;
        w = cloud4.Sample(sampler_lnc, float2(sx, sy)).x;
    }
    else
    {
        float sx = i.C0.x * 0.3333333432674407958984375f;
        float sy = i.C0.y * 0.3333333432674407958984375f;
        s = cloud1.Sample(sampler_lnc, float2(sx, sy + 0.666666686534881591796875f)).x;
        t = cloud1.Sample(sampler_lnc, float2(sx, sy + 0.3333333432674407958984375f)).x;
        u = cloud1.Sample(sampler_lnc, float2(sx + 0.666666686534881591796875f, sy)).x;
        v = cloud1.Sample(sampler_lnc, float2(sx + 0.3333333432674407958984375f, sy)).x;
        w = cloud1.Sample(sampler_lnc, float2(sx, sy)).x;
    }

    float w2 = w * w;
    float v2 = v * v;
    float v3 = (v2 * v2) * v;
    float u2 = u * u;
    float t2 = t * t;
    float s2 = s * s;
    float3 n = normalize(float3(((w2 * w2) * w) - (v3 * v3), (t2 * t2) * t, 1.0f - ((s2 * s2) * s)));
    if (smoothstep(0.0f, 0.5f, (u2 * u2) * u) < 0.5f)
    {
        discard;
    }

    float3 albedo;
    if (glade_theme_id == 2u)
    {
        albedo = pow(canopy_color_1.xyz, 1.5f) * 0.9f;
    }
    else if (glade_theme_id == 4u)
    {
        albedo = lerp(pow(canopy_color_1.xyz, 1.5f), 1.0f, 0.5f);
    }
    else
    {
        albedo = float3(0.024f, 0.054f, 0.018f);
    }

    float3 world_n = (normalize((i.C3 * n.x) + (float3(0.0f, 1.0f, 0.0f) * n.y) + (i.C4 * n.z)) * 0.5f) + 0.5f;
    float3 color = sqrt(clamp(lerp(albedo * 0.5f, sky_low_color.xyz * (1.0f - ((smoothstep(0.483f, 0.505f, time_of_day) - smoothstep(0.84f, 0.91f, time_of_day)) * 0.75f)), 0.02f), 0.0f, 1.0f));
    uint albedo_packed = (uint(clamp(color.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(color.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(color.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u));
    uint normal_packed = (uint(clamp(world_n.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(world_n.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(world_n.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)) + 0xFF000000u;
    o.target = uint2(normal_packed, albedo_packed | 0x46000000u);
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
    uint2 framebuffer_dims : packoffset(c0.y);
    uint2 cloud1_dims : packoffset(c1);
    uint2 cloud2_dims : packoffset(c1.z);
    uint2 cloud3_dims : packoffset(c2);
    uint2 cloud4_dims : packoffset(c2.z);
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
    float _113 = float(uint(gl_VertexIndex) & 1u);
    uint _115 = (uint(gl_VertexIndex) >> 1u) & 1u;
    float2 _119 = (float2(_113, float(_115)) - 0.5f.xx) * float2(1.0f, 2.0f);
    uint _121 = (uint(gl_InstanceIndex) * 12u) + 25u;
    uint _123 = _121 + (_121 << 10u);
    uint _125 = _123 ^ (_123 >> 6u);
    uint _127 = _125 + (_125 << 3u);
    uint _129 = _127 ^ (_127 >> 11u);
    uint _131 = _129 + (_129 << 15u);
    float _132 = float(uint(gl_InstanceIndex));
    uint _137 = _131 + (_131 << 10u);
    uint _139 = _137 ^ (_137 >> 6u);
    uint _141 = _139 + (_139 << 3u);
    uint _143 = _141 ^ (_141 >> 11u);
    uint _145 = _143 + (_143 << 15u);
    uint _147 = _145 + (_145 << 10u);
    uint _149 = _147 ^ (_147 >> 6u);
    uint _151 = _149 + (_149 << 3u);
    uint _153 = _151 ^ (_151 >> 11u);
    uint _155 = _153 + (_153 << 15u);
    uint _157 = _155 + (_155 << 10u);
    uint _159 = _157 ^ (_157 >> 6u);
    uint _161 = _159 + (_159 << 3u);
    uint _163 = _161 ^ (_161 >> 11u);
    uint _165 = _163 + (_163 << 15u);
    float _166 = frac(0.89999997615814208984375f + (_132 * 1.61803400516510009765625f)) * 6.283185482025146484375f;
    float2 _170 = -float2(cos(_166), sin(_166));
    float _171 = _170.y;
    float _172 = _170.x;
    float _173 = -_172;
    float2 _174 = float2(_171, _173);
    float3 _175 = float3(_172, 0.0f, _171);
    float _178 = 1.0f - (_132 * 0.00200000009499490261077880859375f);
    float _179 = _178 * _178;
    float3 _181 = (-_175) * lerp(450.0f, 550.0f, _179);
    _181.y = _181.y - lerp(35.0f, 45.0f, _179);
    uint _187 = _165 + (_165 << 10u);
    uint _189 = _187 ^ (_187 >> 6u);
    uint _191 = _189 + (_189 << 3u);
    uint _193 = _191 ^ (_191 >> 11u);
    uint _195 = _193 + (_193 << 15u);
    float2 _201 = float2(36.0f, 18.0f) * lerp(0.89999997615814208984375f, 1.2000000476837158203125f, asfloat((_165 & 8388607u) | 1065353216u) - 1.0f);
    int _205 = int(frac(_132 * 0.25f) * 4.0f);
    float2 _228;
    if (_205 == 1)
    {
        float2 _210 = _201 * 1.2000000476837158203125f;
        _210.y = _210.y * 0.699999988079071044921875f;
        _228 = _210;
    }
    else
    {
        float2 _227;
        if (_205 == 2)
        {
            _227 = _201 * 0.60000002384185791015625f;
        }
        else
        {
            float2 _226;
            if (_205 == 3)
            {
                float2 _222 = _201 * 1.0f;
                _222.y = _222.y * 0.89999997615814208984375f;
                _226 = _222;
            }
            else
            {
                _226 = _201;
            }
            _227 = _226;
        }
        _228 = _227;
    }
    uint _230 = _195 + (_195 << 10u);
    uint _232 = _230 ^ (_230 >> 6u);
    uint _234 = _232 + (_232 << 3u);
    uint _236 = _234 ^ (_234 >> 11u);
    uint _238 = _236 + (_236 << 15u);
    uint _245 = _238 + (_238 << 10u);
    uint _247 = _245 ^ (_245 >> 6u);
    uint _249 = _247 + (_247 << 3u);
    uint _251 = _249 ^ (_249 >> 11u);
    float3 _271 = _181 + ((float3(_171, 0.0f, _173) * ((_228.x * _119.x) * lerp(1.5f, 2.0f, asfloat((_238 & 8388607u) | 1065353216u) - 1.0f))) + (float3(0.0f, lerp(1.5f, 2.5f, asfloat((_195 & 8388607u) | 1065353216u) - 1.0f), 0.0f) * (_228.y * _119.y)));
    float2 _274 = float2(_113, float(1u - _115));
    float2 _285;
    float2 _286;
    if ((asfloat(((_251 + (_251 << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.5f)
    {
        float2 _283 = _274;
        _283.x = 1.0f - _113;
        _285 = -_174;
        _286 = _283;
    }
    else
    {
        _285 = _174;
        _286 = _274;
    }
    float4 _296 = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_271, 1.0f));
    bool4 _301 = (uint(gl_InstanceIndex) > 500u).xxxx;
    gl_Position = float4(_301.x ? asfloat(0x7fbfffffu          ).xxxx.x : _296.x, _301.y ? asfloat(0x7fbfffffu          ).xxxx.y : _296.y, _301.z ? asfloat(0x7fbfffffu          ).xxxx.z : _296.z, _301.w ? asfloat(0x7fbfffffu          ).xxxx.w : _296.w);
    out_var_C0 = _286;
    out_var_C1 = uint(gl_InstanceIndex);
    out_var_C2 = _271;
    out_var_C3 = float3(_285.x, 0.0f, _285.y);
    out_var_C4 = _175;
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

cbuffer color_library : register(b18, space0)
{
    float4 color_library_temp_color_1 : packoffset(c0);
    float4 color_library_temp_color_2 : packoffset(c1);
    float4 color_library_temp_color_3 : packoffset(c2);
    float4 color_library_roof_color_1 : packoffset(c3);
    float4 color_library_roof_color_2 : packoffset(c4);
    float4 color_library_sky_low_day : packoffset(c5);
    float4 color_library_sky_high_day : packoffset(c6);
    float4 color_library_sun_tint_day : packoffset(c7);
    float4 color_library_sky_low_dusk : packoffset(c8);
    float4 color_library_sky_high_dusk : packoffset(c9);
    float4 color_library_sun_tint_dusk : packoffset(c10);
    float4 color_library_indirect_light_tint : packoffset(c11);
    float4 color_library_fence_color_1 : packoffset(c12);
    float4 color_library_fence_color_2 : packoffset(c13);
    float4 color_library_flower_color_1 : packoffset(c14);
    float4 color_library_flower_color_2 : packoffset(c15);
    float4 color_library_canopy_color_1 : packoffset(c16);
    float4 color_library_canopy_color_2 : packoffset(c17);
    float4 color_library_grass_1 : packoffset(c18);
    float4 color_library_grass_2 : packoffset(c19);
    float4 color_library_path : packoffset(c20);
    float4 color_library_sky_base : packoffset(c21);
    float4 color_library_sky_horizon : packoffset(c22);
    float4 color_library_sky_clouds_1 : packoffset(c23);
    float4 color_library_sky_clouds_2 : packoffset(c24);
    float4 color_library_terrain_1 : packoffset(c25);
    float4 color_library_terrain_2 : packoffset(c26);
    float4 color_library_terrain_flattened : packoffset(c27);
    float4 color_library_wall_1 : packoffset(c28);
    float4 color_library_wall_2 : packoffset(c29);
    float4 color_library_wall_moss : packoffset(c30);
    float4 color_library_wall_bricks : packoffset(c31);
    float4 color_library_ivy_branch : packoffset(c32);
    float4 color_library_ivy_leaf_1 : packoffset(c33);
    float4 color_library_ivy_leaf_2 : packoffset(c34);
    float4 color_library_terrain_highlight : packoffset(c35);
    float4 color_library_night_tint : packoffset(c36);
};

SamplerState sampler_lnc : register(s0, space0);
Texture2D<float4> cloud1 : register(t14, space0);
Texture2D<float4> cloud2 : register(t15, space0);
Texture2D<float4> cloud3 : register(t16, space0);
Texture2D<float4> cloud4 : register(t17, space0);

static float2 in_var_C0;
static uint in_var_C1;
static float3 in_var_C3;
static float3 in_var_C4;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float2 in_var_C0 : TEXCOORD0;
    [[vk::location(1)]] nointerpolation uint in_var_C1 : TEXCOORD1;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 in_var_C4 : TEXCOORD4;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    int _118 = int(frac(float(in_var_C1) * 0.25f) * 4.0f);
    float _261;
    float _262;
    float _263;
    float _264;
    float _265;
    if (_118 == 1)
    {
        float _126 = in_var_C0.x * 0.3333333432674407958984375f;
        float _128 = in_var_C0.y * 0.3333333432674407958984375f;
        _261 = cloud2.Sample(sampler_lnc, float2(_126, _128 + 0.666666686534881591796875f)).x;
        _262 = cloud2.Sample(sampler_lnc, float2(_126, _128 + 0.3333333432674407958984375f)).x;
        _263 = cloud2.Sample(sampler_lnc, float2(_126 + 0.666666686534881591796875f, _128)).x;
        _264 = cloud2.Sample(sampler_lnc, float2(_126 + 0.3333333432674407958984375f, _128)).x;
        _265 = cloud2.Sample(sampler_lnc, float2(_126, _128)).x;
    }
    else
    {
        float _256;
        float _257;
        float _258;
        float _259;
        float _260;
        if (_118 == 2)
        {
            float _160 = in_var_C0.x * 0.3333333432674407958984375f;
            float _162 = in_var_C0.y * 0.3333333432674407958984375f;
            _256 = cloud3.Sample(sampler_lnc, float2(_160, _162 + 0.666666686534881591796875f)).x;
            _257 = cloud3.Sample(sampler_lnc, float2(_160, _162 + 0.3333333432674407958984375f)).x;
            _258 = cloud3.Sample(sampler_lnc, float2(_160 + 0.666666686534881591796875f, _162)).x;
            _259 = cloud3.Sample(sampler_lnc, float2(_160 + 0.3333333432674407958984375f, _162)).x;
            _260 = cloud3.Sample(sampler_lnc, float2(_160, _162)).x;
        }
        else
        {
            float _251;
            float _252;
            float _253;
            float _254;
            float _255;
            if (_118 == 3)
            {
                float _194 = in_var_C0.x * 0.3333333432674407958984375f;
                float _196 = in_var_C0.y * 0.3333333432674407958984375f;
                _251 = cloud4.Sample(sampler_lnc, float2(_194, _196 + 0.666666686534881591796875f)).x;
                _252 = cloud4.Sample(sampler_lnc, float2(_194, _196 + 0.3333333432674407958984375f)).x;
                _253 = cloud4.Sample(sampler_lnc, float2(_194 + 0.666666686534881591796875f, _196)).x;
                _254 = cloud4.Sample(sampler_lnc, float2(_194 + 0.3333333432674407958984375f, _196)).x;
                _255 = cloud4.Sample(sampler_lnc, float2(_194, _196)).x;
            }
            else
            {
                float _224 = in_var_C0.x * 0.3333333432674407958984375f;
                float _226 = in_var_C0.y * 0.3333333432674407958984375f;
                _251 = cloud1.Sample(sampler_lnc, float2(_224, _226 + 0.666666686534881591796875f)).x;
                _252 = cloud1.Sample(sampler_lnc, float2(_224, _226 + 0.3333333432674407958984375f)).x;
                _253 = cloud1.Sample(sampler_lnc, float2(_224 + 0.666666686534881591796875f, _226)).x;
                _254 = cloud1.Sample(sampler_lnc, float2(_224 + 0.3333333432674407958984375f, _226)).x;
                _255 = cloud1.Sample(sampler_lnc, float2(_224, _226)).x;
            }
            _256 = _251;
            _257 = _252;
            _258 = _253;
            _259 = _254;
            _260 = _255;
        }
        _261 = _256;
        _262 = _257;
        _263 = _258;
        _264 = _259;
        _265 = _260;
    }
    float _266 = _265 * _265;
    float _269 = _264 * _264;
    float _271 = (_269 * _269) * _264;
    float _272 = _263 * _263;
    float _275 = _262 * _262;
    float _278 = _261 * _261;
    float3 _285 = normalize(float3(((_266 * _266) * _265) - (_271 * _271), (_275 * _275) * _262, 1.0f - ((_278 * _278) * _261)));
    if (smoothstep(0.0f, 0.5f, (_272 * _272) * _263) < 0.5f)
    {
        discard;
    }
    float3 _320;
    if (frame_constants_glade_theme_id == 2u)
    {
        _320 = pow(color_library_canopy_color_1.xyz, 1.5f.xxx) * 0.89999997615814208984375f;
    }
    else
    {
        float3 _319;
        if (frame_constants_glade_theme_id == 4u)
        {
            _319 = lerp(pow(color_library_canopy_color_1.xyz, 1.5f.xxx), 1.0f.xxx, 0.5f.xxx);
        }
        else
        {
            _319 = float3(0.0240000002086162567138671875f, 0.053999997675418853759765625f, 0.01800000108778476715087890625f);
        }
        _320 = _319;
    }
    float3 _335 = (normalize(((in_var_C3 * _285.x) + (float3(0.0f, 1.0f, 0.0f) * _285.y)) + (in_var_C4 * _285.z)) * 0.5f) + 0.5f.xxx;
    float3 _357 = sqrt(clamp(lerp(_320 * 0.5f, frame_constants_sky_low_color.xyz * (1.0f - ((smoothstep(0.4830000102519989013671875f, 0.50499999523162841796875f, frame_constants_time_of_day) - smoothstep(0.839999973773956298828125f, 0.910000026226043701171875f, frame_constants_time_of_day)) * 0.75f)), 0.0199999995529651641845703125f.xxx), 0.0f.xxx, 1.0f.xxx));
    uint _376 = (uint((clamp(_357.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_357.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_357.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 _377 = uint2(((uint((clamp(_335.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_335.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_335.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u, _376);
    _377.y = _376 | 1174405120u;
    out_var_SV_TARGET0 = _377;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C0 = stage_input.in_var_C0;
    in_var_C1 = stage_input.in_var_C1;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C4 = stage_input.in_var_C4;
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
    uint2 framebuffer_dims : packoffset(c0.y);
    uint2 cloud1_dims : packoffset(c1);
    uint2 cloud2_dims : packoffset(c1.z);
    uint2 cloud3_dims : packoffset(c2);
    uint2 cloud4_dims : packoffset(c2.z);
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
    float _113 = float(uint(gl_VertexIndex) & 1u);
    uint _115 = (uint(gl_VertexIndex) >> 1u) & 1u;
    float2 _119 = (float2(_113, float(_115)) - 0.5f.xx) * float2(1.0f, 2.0f);
    uint _121 = (uint(gl_InstanceIndex) * 12u) + 25u;
    uint _123 = _121 + (_121 << 10u);
    uint _125 = _123 ^ (_123 >> 6u);
    uint _127 = _125 + (_125 << 3u);
    uint _129 = _127 ^ (_127 >> 11u);
    uint _131 = _129 + (_129 << 15u);
    float _132 = float(uint(gl_InstanceIndex));
    uint _137 = _131 + (_131 << 10u);
    uint _139 = _137 ^ (_137 >> 6u);
    uint _141 = _139 + (_139 << 3u);
    uint _143 = _141 ^ (_141 >> 11u);
    uint _145 = _143 + (_143 << 15u);
    uint _147 = _145 + (_145 << 10u);
    uint _149 = _147 ^ (_147 >> 6u);
    uint _151 = _149 + (_149 << 3u);
    uint _153 = _151 ^ (_151 >> 11u);
    uint _155 = _153 + (_153 << 15u);
    uint _157 = _155 + (_155 << 10u);
    uint _159 = _157 ^ (_157 >> 6u);
    uint _161 = _159 + (_159 << 3u);
    uint _163 = _161 ^ (_161 >> 11u);
    uint _165 = _163 + (_163 << 15u);
    float _166 = frac(0.89999997615814208984375f + (_132 * 1.61803400516510009765625f)) * 6.283185482025146484375f;
    float2 _170 = -float2(cos(_166), sin(_166));
    float _171 = _170.y;
    float _172 = _170.x;
    float _173 = -_172;
    float2 _174 = float2(_171, _173);
    float3 _175 = float3(_172, 0.0f, _171);
    float _178 = 1.0f - (_132 * 0.00200000009499490261077880859375f);
    float _179 = _178 * _178;
    float3 _181 = (-_175) * lerp(450.0f, 550.0f, _179);
    _181.y = _181.y - lerp(35.0f, 45.0f, _179);
    uint _187 = _165 + (_165 << 10u);
    uint _189 = _187 ^ (_187 >> 6u);
    uint _191 = _189 + (_189 << 3u);
    uint _193 = _191 ^ (_191 >> 11u);
    uint _195 = _193 + (_193 << 15u);
    float2 _201 = float2(36.0f, 18.0f) * lerp(0.89999997615814208984375f, 1.2000000476837158203125f, asfloat((_165 & 8388607u) | 1065353216u) - 1.0f);
    int _205 = int(frac(_132 * 0.25f) * 4.0f);
    float2 _228;
    if (_205 == 1)
    {
        float2 _210 = _201 * 1.2000000476837158203125f;
        _210.y = _210.y * 0.699999988079071044921875f;
        _228 = _210;
    }
    else
    {
        float2 _227;
        if (_205 == 2)
        {
            _227 = _201 * 0.60000002384185791015625f;
        }
        else
        {
            float2 _226;
            if (_205 == 3)
            {
                float2 _222 = _201 * 1.0f;
                _222.y = _222.y * 0.89999997615814208984375f;
                _226 = _222;
            }
            else
            {
                _226 = _201;
            }
            _227 = _226;
        }
        _228 = _227;
    }
    uint _230 = _195 + (_195 << 10u);
    uint _232 = _230 ^ (_230 >> 6u);
    uint _234 = _232 + (_232 << 3u);
    uint _236 = _234 ^ (_234 >> 11u);
    uint _238 = _236 + (_236 << 15u);
    uint _245 = _238 + (_238 << 10u);
    uint _247 = _245 ^ (_245 >> 6u);
    uint _249 = _247 + (_247 << 3u);
    uint _251 = _249 ^ (_249 >> 11u);
    float3 _271 = _181 + ((float3(_171, 0.0f, _173) * ((_228.x * _119.x) * lerp(1.5f, 2.0f, asfloat((_238 & 8388607u) | 1065353216u) - 1.0f))) + (float3(0.0f, lerp(1.5f, 2.5f, asfloat((_195 & 8388607u) | 1065353216u) - 1.0f), 0.0f) * (_228.y * _119.y)));
    float2 _274 = float2(_113, float(1u - _115));
    float2 _285;
    float2 _286;
    if ((asfloat(((_251 + (_251 << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.5f)
    {
        float2 _283 = _274;
        _283.x = 1.0f - _113;
        _285 = -_174;
        _286 = _283;
    }
    else
    {
        _285 = _174;
        _286 = _274;
    }
    float4 _296 = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_271, 1.0f));
    bool4 _301 = (uint(gl_InstanceIndex) > 500u).xxxx;
    gl_Position = float4(_301.x ? asfloat(0x7fbfffffu          ).xxxx.x : _296.x, _301.y ? asfloat(0x7fbfffffu          ).xxxx.y : _296.y, _301.z ? asfloat(0x7fbfffffu          ).xxxx.z : _296.z, _301.w ? asfloat(0x7fbfffffu          ).xxxx.w : _296.w);
    out_var_C0 = _286;
    out_var_C1 = uint(gl_InstanceIndex);
    out_var_C2 = _271;
    out_var_C3 = float3(_285.x, 0.0f, _285.y);
    out_var_C4 = _175;
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
SamplerState sampler_lnc : register(s0, space0);
Texture2D<float4> cloud1 : register(t14, space0);
Texture2D<float4> cloud2 : register(t15, space0);
Texture2D<float4> cloud3 : register(t16, space0);
Texture2D<float4> cloud4 : register(t17, space0);

static float2 in_var_C0;
static uint in_var_C1;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float2 in_var_C0 : TEXCOORD0;
    [[vk::location(1)]] nointerpolation uint in_var_C1 : TEXCOORD1;
};

void frag_main()
{
    int _68 = int(frac(float(in_var_C1) * 0.25f) * 4.0f);
    float _127;
    if (_68 == 1)
    {
        _127 = cloud2.Sample(sampler_lnc, float2((in_var_C0.x * 0.3333333432674407958984375f) + 0.666666686534881591796875f, in_var_C0.y * 0.3333333432674407958984375f)).x;
    }
    else
    {
        float _126;
        if (_68 == 2)
        {
            _126 = cloud3.Sample(sampler_lnc, float2((in_var_C0.x * 0.3333333432674407958984375f) + 0.666666686534881591796875f, in_var_C0.y * 0.3333333432674407958984375f)).x;
        }
        else
        {
            float _125;
            if (_68 == 3)
            {
                _125 = cloud4.Sample(sampler_lnc, float2((in_var_C0.x * 0.3333333432674407958984375f) + 0.666666686534881591796875f, in_var_C0.y * 0.3333333432674407958984375f)).x;
            }
            else
            {
                _125 = cloud1.Sample(sampler_lnc, float2((in_var_C0.x * 0.3333333432674407958984375f) + 0.666666686534881591796875f, in_var_C0.y * 0.3333333432674407958984375f)).x;
            }
            _126 = _125;
        }
        _127 = _126;
    }
    float _128 = _127 * _127;
    if (smoothstep(0.0f, 0.5f, (_128 * _128) * _127) < 0.5f)
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C0 = stage_input.in_var_C0;
    in_var_C1 = stage_input.in_var_C1;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
