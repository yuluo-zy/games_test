#define SHADER_VARIANT_DEPTH_ONLY 1
#define TGV_f0adff76827e73ff 1
#line 1 "shaders/grass_lod1.raster.hlsl"


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
#line 13 "shaders/grass_lod1.raster.hlsl"

#define GRASS_LOD1
#line 1 "shaders/grass.raster.hlsl"


#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))












#ifndef TINYGLADE_GRASS_RASTER_HLSL
#define TINYGLADE_GRASS_RASTER_HLSL

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
#line 19 "shaders/grass.raster.hlsl"

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
#line 20 "shaders/grass.raster.hlsl"

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
#line 21 "shaders/grass.raster.hlsl"

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
#line 22 "shaders/grass.raster.hlsl"

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
#line 23 "shaders/grass.raster.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 24 "shaders/grass.raster.hlsl"

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
#line 25 "shaders/grass.raster.hlsl"

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
#line 26 "shaders/grass.raster.hlsl"

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
#line 27 "shaders/grass.raster.hlsl"

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
#line 28 "shaders/grass.raster.hlsl"

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
#line 29 "shaders/grass.raster.hlsl"

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
#line 30 "shaders/grass.raster.hlsl"

#line 1 "shaders/inc/grass_blade_data.hlsl"


#ifndef TINYGLADE_INC_GRASS_BLADE_DATA_HLSL
#define TINYGLADE_INC_GRASS_BLADE_DATA_HLSL



struct GrassBladeDataPacked {
    float3 position; 
    uint color;      
    uint packed0;    
    uint packed1;    
    uint packed2;    
    uint pad;
};


float3 grass_blade_color(GrassBladeDataPacked b)
{
    return float3(float(b.color & 255u) * (1.0 / 255.0),
                  float((b.color >> 8u) & 255u) * (1.0 / 255.0),
                  float((b.color >> 16u) & 255u) * (1.0 / 255.0));
}


float2 grass_blade_widths(GrassBladeDataPacked b)
{
    return float2(f16tof32(b.packed1 & 0xffffu), f16tof32((b.packed1 >> 16u) & 0xffffu));
}


float grass_blade_bend_xz(GrassBladeDataPacked b)
{
    return f16tof32((b.packed2 >> 16u) & 0xffffu);
}


float grass_blade_dir_x(GrassBladeDataPacked b)
{
    return float((b.packed0 >> 24u) & 255u) * (2.0 / 255.0) - 1.0;
}

#endif 
#line 31 "shaders/grass.raster.hlsl"

#line 1 "shaders/inc/atomics.hlsl"




#ifndef TINYGLADE_INC_ATOMICS_HLSL
#define TINYGLADE_INC_ATOMICS_HLSL


uint atomic_append(RWByteAddressBuffer counter_buffer, uint counter_byte_offset)
{
    uint previous;
    counter_buffer.InterlockedAdd(counter_byte_offset, 1u, previous);
    return previous;
}


uint atomic_add(RWByteAddressBuffer buffer, uint byte_offset, uint value)
{
    uint previous;
    buffer.InterlockedAdd(byte_offset, value, previous);
    return previous;
}

#endif 
#line 32 "shaders/grass.raster.hlsl"

#line 1 "shaders/inc/lod.hlsl"


#ifndef TINYGLADE_INC_LOD_HLSL
#define TINYGLADE_INC_LOD_HLSL


struct MeshLod {
    uint first_index;
    uint index_count;
    float bounding_sphere_radius;
    uint pad0;
};



uint select_lod(float distance_sq, float lod0_distance_sq, float lod1_distance_sq)
{
    if (distance_sq < lod0_distance_sq) {
        return 0u;
    }
    if (distance_sq < lod1_distance_sq) {
        return 1u;
    }
    return 2u;
}


uint select_grass_lod(float distance_sq, float lod1_distance_sq)
{
    return (distance_sq < lod1_distance_sq) ? 0u : 1u;
}

#endif 
#line 33 "shaders/grass.raster.hlsl"


Texture2D<float4> terrain_heightmap : register(t15, space0);
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
#line 36 "shaders/grass.raster.hlsl"

#line 1 "shaders/inc/material/grass_material.inc.hlsl"


#ifndef TINYGLADE_INC_MATERIAL_GRASS_MATERIAL_HLSL
#define TINYGLADE_INC_MATERIAL_GRASS_MATERIAL_HLSL

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
#line 6 "shaders/inc/material/grass_material.inc.hlsl"




struct GrassMaterialParams {
    float3 base_color;
    float3 tip_color;
    float roughness;
    float translucency;
};

GrassMaterialParams init_grass_material(float3 base_color, float3 tip_color, float roughness)
{
    GrassMaterialParams p;
    p.base_color = base_color;
    p.tip_color = tip_color;
    p.roughness = saturate(roughness);
    p.translucency = 1.0;
    return p;
}

#endif 
#line 37 "shaders/grass.raster.hlsl"

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
#line 38 "shaders/grass.raster.hlsl"

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
#line 39 "shaders/grass.raster.hlsl"

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
#line 40 "shaders/grass.raster.hlsl"

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
#line 41 "shaders/grass.raster.hlsl"

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
#line 42 "shaders/grass.raster.hlsl"

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
#line 43 "shaders/grass.raster.hlsl"

#line 1 "shaders/inc/sh.hlsl"










float4 sh_basis(float3 dir)
{
    return float4(0.2820949852466583251953125f, 0.48860299587249755859375f * dir);
}

float sh_eval(float4 sh, float3 dir)
{
    return dot(sh, sh_basis(dir));
}
#line 44 "shaders/grass.raster.hlsl"




float time;
uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
uint grass_buffer_size;

StructuredBuffer<GrassBladeDataPacked> grass_blade_buffer_data : register(t17, space0);


float3 blade_color_rgb(GrassBladeDataPacked b)
{
    return float3(float(b.color & 255u) * (1.0f / 255.0f),
                  float((b.color >> 8u) & 255u) * (1.0f / 255.0f),
                  float((b.color >> 16u) & 255u) * (1.0f / 255.0f));
}


uint pack_normal8(float3 n)
{
    return (uint(clamp(n.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(n.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(n.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u));
}

#if !defined(GRASS_LOD1)

struct VS_OUT
{
    float3 C0 : TEXCOORD0;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float3 C4 : TEXCOORD4;
    float3 C5 : TEXCOORD5;
    float3 C6 : TEXCOORD6;
    float2 C7 : TEXCOORD7;
    float C8 : TEXCOORD8;
    float3 C9 : TEXCOORD9;
    float4 position : SV_Position;
};

VS_OUT vs_main(uint vertex_id : SV_VertexID, uint instance_id : SV_InstanceID)
{
    GrassBladeDataPacked blade = grass_blade_buffer_data[instance_id];
    float3 blade_pos = blade.position;
    float3 albedo = blade_color_rgb(blade);
    float3 albedo2 = albedo * albedo;
    float blend = float(blade.packed0 & 255u) * (1.0f / 255.0f);
    float2 widths = grass_blade_widths(blade);
    float width_bottom = widths.x;

    float v = (float(vertex_id & 1u) * 2.0f) - 1.0f;
    float t = clamp(float(vertex_id & 0xFFFFFFFEu) / (exp2((1.0f - blend) * 2.3219280242919921875f) * 2.0f), 0.0f, 1.0f);
    float skew = lerp(clamp(2.5f * (1.0f - (t * t)), 0.0f, 1.0f) * 0.04f, t * 0.135f, blend);
    float blade_h = t * lerp(1.0f, 1.1f, blend);
    float2 st = float2(v, blade_h);
    float3 s = float3(v * skew, blade_h, 0.0f);

    
    float4 gathered = terrain_heightmap.GatherRed(sampler_default, (blade_pos.xz * (1.0f / 130.0f)) + 0.5f, int2(0, 0));
    float2 grad = ((float2((gathered.z + gathered.y) - (gathered.w + gathered.x),
                           (gathered.x + gathered.y) - (gathered.w + gathered.z)) * 0.5f) * 26.5f)
        / (130.0f / float2(terrain_heightmap_dims));
    float bend_amt = width_bottom * 0.2f;
    float3 blade_scale = float3(min(1.0f, width_bottom * 0.8f) * 0.36f, bend_amt, 1.0f) * 2.5f;

    float dist_scale = max(30.0f / (((-1.0f) / mul(view_constants[0].world_to_view, float4(blade_pos, 1.0f)).z)
        * ((0.5f * view_constants[0].view_to_clip[1].y) * float(clamp(inner_rendering_resolution.y, 1080u, 2160u)))), 1.0f);
    float bend_offset = blade_pos.y - ((dist_scale - 1.0f) * blade_scale.y);
    float3 ground_pos = blade_pos;
    ground_pos.y = bend_offset;
    float wind_scale = blade_scale.y * dist_scale;

    
    float3 axis = normalize(float3((float((blade.packed0 >> 24u) & 255u) * (2.0f / 255.0f)) - 1.0f, 0.0f, grass_blade_bend_xz(blade)));
    float4x4 basis = transpose(float4x4(float4(normalize(cross(axis, float3(0.0f, 1.0f, 0.0f))), 0.0f),
                                         float4(0.0f, 1.0f, 0.0f, 0.0f),
                                         float4(axis, 0.0f),
                                         float4(0.0f, 0.0f, 0.0f, 1.0f)));

    float4 cam_pos = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float h_clamped = clamp(blade_h, 0.0f, 1.0f);
    float curve = 3.0f * ((0.03f * widths.y) * lerp(1.0f, 2.0f, abs(normalize((cam_pos.xyz / cam_pos.w) - ground_pos).y)));
    float curve_factor = curve * (h_clamped * h_clamped);
    s.z = curve_factor;

    float3 blade_geom;
    if (glade_theme_id == 4u)
    {
        blade_geom = s;
        blade_geom.x = v * skew * 0.5f;
    }
    else
    {
        blade_geom = s;
    }

    
    float bend_angle = float((blade.packed0 >> 16u) & 255u) * (-0.0061568631790578365325927734375f);
    float sina = sin(bend_angle);
    float cosa = cos(bend_angle);
    float oc = 1.0f - cosa;
    float axis_x = 1.0f;
    float axis_y = 0.0f;
    float axis_z = 0.0f;
    float t_oc_x = oc * axis_x;
    float t_oc_xy = t_oc_x * axis_y;
    float t_z_sin = axis_z * sina;
    float t_oc_z = oc * axis_z;
    float t_oc_zx = t_oc_z * axis_x;
    float t_y_sin = axis_y * sina;
    float t_oc_y = oc * axis_y;
    float t_oc_yz = t_oc_y * axis_z;
    float t_x_sin = axis_x * sina;
    float4x4 twist = transpose(float4x4(float4((t_oc_x * axis_x) + cosa, t_oc_xy - t_z_sin, t_oc_zx + t_y_sin, 0.0f),
                                        float4(t_oc_xy + t_z_sin, (t_oc_y * axis_y) + cosa, t_oc_yz - t_x_sin, 0.0f),
                                        float4(t_oc_zx - t_y_sin, t_oc_yz + t_x_sin, (t_oc_z * axis_z) + cosa, 0.0f),
                                        float4(0.0f, 0.0f, 0.0f, 1.0f)));

    float3 blade_normal = normalize(cross(float3(1.0f, 0.0f, 0.0f), float3(0.0f, 1e-5f * wind_scale, (curve * ((h_clamped + 0.0001f) * (h_clamped + 0.0001f))) - curve_factor)));
    blade_normal.x += 0.3f * v;

    float4 rotated = mul(basis, mul(twist, float4(normalize(blade_normal), 0.0f)));
    float3 rotated_xyz = rotated.xyz;
    float4 world_offset = mul(basis, mul(twist, mul(float4x4(float4(blade_scale.x * (float((blade.packed0 >> 8u) & 255u) * (1.0f / 255.0f)) * dist_scale, 0.0f, 0.0f, 0.0f),
                                                          float4(0.0f, wind_scale, 0.0f, 0.0f),
                                                          float4(0.0f, 0.0f, blade_scale.z, 0.0f),
                                                          float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(blade_geom, 1.0f)))) + float4(blade_pos.x, bend_offset, blade_pos.z, 0.0f);

    
    float2 wind_uv = (ground_pos.xz * 0.1f) + (float2(time, 0.0f) * 1.2f);
    int2 wind_cell = int2(floor(wind_uv));
    float2 wind_frac = frac(wind_uv);
    float2 wind_smooth = wind_frac * wind_frac * (3.0f - (2.0f * wind_frac));
    float wind_fx = wind_smooth.x;
    float wind_fy = wind_smooth.y;
    float wind_n00 = incb_perlin_hash01(hash(uint2(wind_cell + int2(0, 0))));
    float wind_n10 = incb_perlin_hash01(hash(uint2(wind_cell + int2(1, 0))));
    float wind_n01 = incb_perlin_hash01(hash(uint2(wind_cell + int2(0, 1))));
    float wind_n11 = incb_perlin_hash01(hash(uint2(wind_cell + int2(1, 1))));
    float wind_noise = ((1.0f - wind_fy) * ((wind_n00 * (1.0f - wind_fx)) + (wind_n10 * wind_fx)))
        + ((wind_n01 * (1.0f - wind_fx)) * wind_fy)
        + ((wind_n11 * wind_fx) * wind_fy);
    float wind = (smoothstep(-0.2f, 0.5f, bend_amt) * (blade_h * blade_h))
        * (sin((wind_noise * 3.14f) - 1.57f + 0.3f)) * 0.7f * grass_blade_bend_xz(blade);
    rotated_xyz.y -= wind * 0.7f;
    world_offset.x -= wind * 0.25f;

    VS_OUT o;
    o.position = mul(view_constants[current_view_index].world_to_sample, world_offset);
    o.C0 = lerp(albedo2 * 0.8f, albedo2, (blade_h * blade_h).xxx);
    o.C2 = cross(normalize(float3(0.0f, grad.y, 1.0f)), normalize(float3(1.0f, grad.x, 0.0f)));
    o.C3 = world_offset.xyz;
    o.C4 = ground_pos;
    o.C5 = normalize(rotated_xyz);
    o.C6 = axis;
    o.C7 = st;
    o.C8 = blend;
    o.C9 = float3(st * float2(skew, 1.0f), skew);
    return o;
}

struct PS_IN
{
    float3 C0 : TEXCOORD0;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float3 C5 : TEXCOORD5;
    float2 C7 : TEXCOORD7;
    float C8 : TEXCOORD8;
    float3 C9 : TEXCOORD9;
};

struct PS_OUT
{
    uint2 target : SV_Target0;
};

PS_OUT ps_main(PS_IN i)
{
    PS_OUT o;
    float4 cam_pos = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    if (abs(i.C9.x / i.C9.z) < lerp((-0.8f) / i.C8, 1.0f, i.C7.y))
    {
        discard;
    }
    float3 view_normal;
    if (dot(normalize((cam_pos.xyz / cam_pos.w) - i.C3), i.C5) <= 0.0f)
    {
        view_normal = -i.C5;
    }
    else
    {
        view_normal = i.C5;
    }
    float3 n = normalize(view_normal);
    float3 albedo = (i.C0 * 0.55f) * lerp(0.7f, 1.1f, i.C7.y);
    float3 albedo_out;
    if (glade_theme_id == 4u)
    {
        albedo_out = lerp(0.9f.xxx, albedo * 0.5f, pow(i.C7.y, 0.5f).xxx);
    }
    else
    {
        albedo_out = albedo;
    }
    float3 octa = n / ((abs(n.x) + abs(n.y)) + abs(n.z));
    if (octa.z < 0.0f)
    {
        float2 xy = (1.0f - abs(octa.yx)) * ((step(0.0f, octa.xy) * 2.0f) - 1.0f);
        octa = float3(xy.x, xy.y, octa.z);
    }
    float2 octa_uv = (octa.xy * 0.5f) + 0.5f;
    float3 packed_n = (normalize(i.C2) * 0.5f) + 0.5f;
    float3 albedo_sqrt = sqrt(clamp(albedo_out, 0.0f, 1.0f));
    o.target = uint2(pack_normal8(packed_n) + (uint(clamp(octa_uv.x, 0.0f, 1.0f) * 255.0f + 0.5f) << 24u),
                     pack_normal8(albedo_sqrt) + (uint(clamp(octa_uv.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 24u));
    return o;
}

#endif 

#endif 

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

struct GrassBladeDataPacked
{
    float3 position;
    uint color;
    uint packed0;
    uint packed1;
    uint packed2;
    uint pad;
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
    float _Globals_time : packoffset(c0.y);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c0.z);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c1);
    uint _Globals_grass_buffer_size : packoffset(c1.z);
};

ByteAddressBuffer grass_blade_buffer_data : register(t17, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t15, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static float3 out_var_C0;
static float3 out_var_C2;
static float3 out_var_C3;
static float3 out_var_C4;
static float3 out_var_C5;
static float3 out_var_C6;
static float2 out_var_C7;
static float out_var_C8;
static float3 out_var_C9;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float3 out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float3 out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] float2 out_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float out_var_C8 : TEXCOORD8;
    [[vk::location(9)]] float3 out_var_C9 : TEXCOORD9;
    float4 gl_Position : SV_Position;
};

uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void vert_main()
{
    float3 _168 = grass_blade_buffer_data.Load<float3>(uint(gl_InstanceIndex) * 32 + 0);
    float3 _182 = float3(float(grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 12) & 255u) * 0.0039215688593685626983642578125f, float((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 12) >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 12) >> 16u) & 255u) * 0.0039215688593685626983642578125f);
    float3 _183 = _182 * _182;
    float _188 = float(grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 16) & 255u) * 0.0039215688593685626983642578125f;
    float2 _204 = spvUnpackHalf2x16(grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 20) & 65535u);
    float _205 = _204.x;
    float _226 = (float(uint(gl_VertexIndex) & 1u) * 2.0f) - 1.0f;
    float _230 = clamp(float(uint(gl_VertexIndex) & 4294967294u) / (exp2((1.0f - _188) * 2.3219280242919921875f) * 2.0f), 0.0f, 1.0f);
    float _237 = lerp(clamp(2.5f * (1.0f - (_230 * _230)), 0.0f, 1.0f) * 0.039999999105930328369140625f, _230 * 0.13500000536441802978515625f, _188);
    float _239 = _230 * lerp(1.0f, 1.10000002384185791015625f, _188);
    float2 _240 = float2(_226, _239);
    float _241 = _226 * _237;
    float3 _242 = float3(_241, _239, 0.0f);
    float4 _249 = terrain_heightmap.GatherRed(sampler_default, (_168.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, int2(0, 0));
    float _254 = _249.z;
    float _255 = _249.y;
    float _257 = _249.w;
    float _258 = _249.x;
    float2 _267 = ((float2((_254 + _255) - (_257 + _258), (_258 + _255) - (_257 + _254)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(_Globals_terrain_heightmap_dims));
    float _278 = _205 * 0.20000000298023223876953125f;
    float3 _280 = float3(min(1.0f, _205 * 0.800000011920928955078125f) * 0.3599999845027923583984375f, _278, 1.0f) * 2.5f;
    float _303 = max(30.0f / (((-1.0f) / mul(frame_constants_view_constants[0u].world_to_view, float4(_168, 1.0f)).z) * ((0.5f * frame_constants_view_constants[0u].view_to_clip[1].y) * float(clamp(frame_constants_inner_rendering_resolution.y, 1080u, 2160u)))), 1.0f);
    float _306 = _280.y;
    float _308 = _168.y - ((_303 - 1.0f) * _306);
    float3 _309 = _168;
    _309.y = _308;
    float _310 = _306 * _303;
    float3 _316 = float3((float((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 16) >> 24u) & 255u) * 0.007843137718737125396728515625f) - 1.0f, 0.0f, spvUnpackHalf2x16((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 24) >> 16u) & 65535u).x);
    float3 _317 = normalize(_316);
    float4x4 _329 = transpose(float4x4(float4(normalize(cross(_317, float3(0.0f, 1.0f, 0.0f))), 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(_317, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float4 _335 = mul(frame_constants_view_constants[_Globals_current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float _346 = clamp(_239, 0.0f, 1.0f);
    float _347 = 3.0f * ((0.02999999932944774627685546875f * spvUnpackHalf2x16((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 20) >> 16u) & 65535u).x) * lerp(1.0f, 2.0f, abs(normalize((_335.xyz / _335.w.xxx) - _309).y)));
    float _349 = _347 * (_346 * _346);
    float _350 = _346 + 0.00010001659393310546875f;
    _242.z = _349;
    float3 _362;
    if (frame_constants_glade_theme_id == 4u)
    {
        float3 _361 = _242;
        _361.x = _241 * 0.5f;
        _362 = _361;
    }
    else
    {
        _362 = _242;
    }
    float _363 = float((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 16) >> 16u) & 255u) * (-0.0061568631790578365325927734375f);
    float3 _364 = normalize(float3(1.0f, 0.0f, 0.0f));
    float _365 = sin(_363);
    float _366 = cos(_363);
    float _367 = 1.0f - _366;
    float _368 = _364.x;
    float _369 = _367 * _368;
    float _372 = _364.y;
    float _373 = _369 * _372;
    float _374 = _364.z;
    float _375 = _374 * _365;
    float _377 = _367 * _374;
    float _378 = _377 * _368;
    float _379 = _372 * _365;
    float _382 = _367 * _372;
    float _385 = _382 * _374;
    float _386 = _368 * _365;
    float4x4 _396 = transpose(float4x4(float4((_369 * _368) + _366, _373 - _375, _378 + _379, 0.0f), float4(_373 + _375, (_382 * _372) + _366, _385 - _386, 0.0f), float4(_378 - _379, _385 + _386, (_377 * _374) + _366, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float3 _400 = normalize(cross(float3(1.0f, 0.0f, 0.0f), float3(0.0f, 9.9999997473787516355514526367188e-05f * _310, (_347 * (_350 * _350)) - _349)));
    _400.x = _400.x + (0.300000011920928955078125f * _226);
    float4 _411 = mul(_329, mul(_396, float4(normalize(_400), 0.0f)));
    float3 _412 = _411.xyz;
    float4 _421 = mul(_329, mul(_396, mul(float4x4(float4((_280.x * (float((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 16) >> 8u) & 255u) * 0.0039215688593685626983642578125f)) * _303, 0.0f, 0.0f, 0.0f), float4(0.0f, _310, 0.0f, 0.0f), float4(0.0f, 0.0f, _280.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(_362, 1.0f)))) + float4(_168.x, _308, _168.z, 0.0f);
    float2 _428 = (_309.xz * 0.100000001490116119384765625f) + (float2(_Globals_time, 0.0f) * 1.2000000476837158203125f);
    int2 _430 = int2(floor(_428));
    float2 _431 = frac(_428);
    uint2 _432 = uint2(_430);
    uint _434 = _432.y;
    uint _436 = _434 + (_434 << 10u);
    uint _438 = _436 ^ (_436 >> 6u);
    uint _440 = _438 + (_438 << 3u);
    uint _442 = _440 ^ (_440 >> 11u);
    uint _448 = (((_432.x * 1664525u) + (_442 + (_442 << 15u))) + 1013904223u) * 1664525u;
    uint _450 = _448 ^ (_448 >> 11u);
    uint _453 = _450 ^ ((_450 << 7u) & 2636928640u);
    uint _456 = _453 ^ ((_453 << 15u) & 4022730752u);
    uint2 _464 = uint2(_430 + int2(1, 0));
    uint _466 = _464.y;
    uint _468 = _466 + (_466 << 10u);
    uint _470 = _468 ^ (_468 >> 6u);
    uint _472 = _470 + (_470 << 3u);
    uint _474 = _472 ^ (_472 >> 11u);
    uint _480 = (((_464.x * 1664525u) + (_474 + (_474 << 15u))) + 1013904223u) * 1664525u;
    uint _482 = _480 ^ (_480 >> 11u);
    uint _485 = _482 ^ ((_482 << 7u) & 2636928640u);
    uint _488 = _485 ^ ((_485 << 15u) & 4022730752u);
    uint2 _496 = uint2(_430 + int2(0, 1));
    uint _498 = _496.y;
    uint _500 = _498 + (_498 << 10u);
    uint _502 = _500 ^ (_500 >> 6u);
    uint _504 = _502 + (_502 << 3u);
    uint _506 = _504 ^ (_504 >> 11u);
    uint _512 = (((_496.x * 1664525u) + (_506 + (_506 << 15u))) + 1013904223u) * 1664525u;
    uint _514 = _512 ^ (_512 >> 11u);
    uint _517 = _514 ^ ((_514 << 7u) & 2636928640u);
    uint _520 = _517 ^ ((_517 << 15u) & 4022730752u);
    uint2 _528 = uint2(_430 + int2(1, 1));
    uint _530 = _528.y;
    uint _532 = _530 + (_530 << 10u);
    uint _534 = _532 ^ (_532 >> 6u);
    uint _536 = _534 + (_534 << 3u);
    uint _538 = _536 ^ (_536 >> 11u);
    uint _544 = (((_528.x * 1664525u) + (_538 + (_538 << 15u))) + 1013904223u) * 1664525u;
    uint _546 = _544 ^ (_544 >> 11u);
    uint _549 = _546 ^ ((_546 << 7u) & 2636928640u);
    uint _552 = _549 ^ ((_549 << 15u) & 4022730752u);
    float2 _562 = (_431 * _431) * (3.0f.xx - (_431 * 2.0f));
    float _563 = _562.x;
    float _564 = 1.0f - _563;
    float _566 = _562.y;
    float _578 = _239 * _239;
    float _586 = (((smoothstep(-0.20000000298023223876953125f, 0.5f, _278) * _578) * (sin((((((1.0f - _566) * (((asfloat(((_456 ^ (_456 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _564) + ((asfloat(((_488 ^ (_488 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _563))) + (((asfloat(((_520 ^ (_520 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _564) * _566)) + (((asfloat(((_552 ^ (_552 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _563) * _566)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * spvUnpackHalf2x16(grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 24) & 65535u).x;
    _412.y = _411.y - (_586 * 0.699999988079071044921875f);
    _421.x = _421.x - (_586 * 0.25f);
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, _421);
    out_var_C0 = lerp(_183 * 0.800000011920928955078125f, _183, _578.xxx);
    out_var_C2 = cross(normalize(float3(0.0f, _267.y, 1.0f)), normalize(float3(1.0f, _267.x, 0.0f)));
    out_var_C3 = _421.xyz;
    out_var_C4 = _309;
    out_var_C5 = normalize(_412);
    out_var_C6 = _316;
    out_var_C7 = _240;
    out_var_C8 = _188;
    out_var_C9 = float3(_240 * float2(_237, 1.0f), _237);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
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
    float _Globals_time : packoffset(c0.y);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c0.z);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c1);
    uint _Globals_grass_buffer_size : packoffset(c1.z);
};


static float3 in_var_C0;
static float3 in_var_C2;
static float3 in_var_C3;
static float3 in_var_C5;
static float2 in_var_C7;
static float in_var_C8;
static float3 in_var_C9;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_C0 : TEXCOORD0;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
    [[vk::location(5)]] float3 in_var_C5 : TEXCOORD5;
    [[vk::location(7)]] float2 in_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float in_var_C8 : TEXCOORD8;
    [[vk::location(9)]] float3 in_var_C9 : TEXCOORD9;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    float4 _107 = mul(frame_constants_view_constants[_Globals_current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    if (abs(in_var_C9.x / in_var_C9.z) < lerp((-0.800000011920928955078125f) / in_var_C8, 1.0f, in_var_C7.y))
    {
        discard;
    }
    float3 _129;
    if (dot(normalize((_107.xyz / _107.w.xxx) - in_var_C3), in_var_C5) <= 0.0f)
    {
        _129 = -in_var_C5;
    }
    else
    {
        _129 = in_var_C5;
    }
    float3 _130 = normalize(_129);
    float3 _133 = (in_var_C0 * 0.550000011920928955078125f) * lerp(0.699999988079071044921875f, 1.10000002384185791015625f, in_var_C7.y);
    float3 _143;
    if (frame_constants_glade_theme_id == 4u)
    {
        _143 = lerp(0.89999997615814208984375f.xxx, _133 * 0.5f, pow(in_var_C7.y, 0.5f).xxx);
    }
    else
    {
        _143 = _133;
    }
    float3 _153 = _130 / ((abs(_130.x) + abs(_130.y)) + abs(_130.z)).xxx;
    float3 _167;
    if (_153.z < 0.0f)
    {
        float2 _165 = (1.0f.xx - abs(_153.yx)) * ((step(0.0f.xx, _153.xy) * 2.0f) - 1.0f.xx);
        _167 = float3(_165.x, _165.y, _153.z);
    }
    else
    {
        _167 = _153;
    }
    float2 _170 = (_167.xy * 0.5f) + 0.5f.xx;
    float3 _172 = (normalize(in_var_C2) * 0.5f) + 0.5f.xxx;
    float3 _200 = sqrt(clamp(_143, 0.0f.xxx, 1.0f.xxx));
    out_var_SV_TARGET0 = uint2(((uint((clamp(_172.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_172.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_172.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + (uint((clamp(_170.x, 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u), ((uint((clamp(_200.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_200.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_200.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + (uint((clamp(_170.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u));
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C0 = stage_input.in_var_C0;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C5 = stage_input.in_var_C5;
    in_var_C7 = stage_input.in_var_C7;
    in_var_C8 = stage_input.in_var_C8;
    in_var_C9 = stage_input.in_var_C9;
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

struct GrassBladeDataPacked
{
    float3 position;
    uint color;
    uint packed0;
    uint packed1;
    uint packed2;
    uint pad;
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
    float _Globals_time : packoffset(c0.y);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c0.z);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c1);
    uint _Globals_grass_buffer_size : packoffset(c1.z);
};

ByteAddressBuffer grass_blade_buffer_data : register(t17, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t15, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static float3 out_var_C0;
static float3 out_var_C2;
static float3 out_var_C3;
static float3 out_var_C4;
static float3 out_var_C5;
static float3 out_var_C6;
static float2 out_var_C7;
static float out_var_C8;
static float3 out_var_C9;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float3 out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float3 out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] float2 out_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float out_var_C8 : TEXCOORD8;
    [[vk::location(9)]] float3 out_var_C9 : TEXCOORD9;
    float4 gl_Position : SV_Position;
};

uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void vert_main()
{
    float3 _168 = grass_blade_buffer_data.Load<float3>(uint(gl_InstanceIndex) * 32 + 0);
    float3 _182 = float3(float(grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 12) & 255u) * 0.0039215688593685626983642578125f, float((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 12) >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 12) >> 16u) & 255u) * 0.0039215688593685626983642578125f);
    float3 _183 = _182 * _182;
    float _188 = float(grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 16) & 255u) * 0.0039215688593685626983642578125f;
    float2 _204 = spvUnpackHalf2x16(grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 20) & 65535u);
    float _205 = _204.x;
    float _226 = (float(uint(gl_VertexIndex) & 1u) * 2.0f) - 1.0f;
    float _230 = clamp(float(uint(gl_VertexIndex) & 4294967294u) / (exp2((1.0f - _188) * 2.3219280242919921875f) * 2.0f), 0.0f, 1.0f);
    float _237 = lerp(clamp(2.5f * (1.0f - (_230 * _230)), 0.0f, 1.0f) * 0.039999999105930328369140625f, _230 * 0.13500000536441802978515625f, _188);
    float _239 = _230 * lerp(1.0f, 1.10000002384185791015625f, _188);
    float2 _240 = float2(_226, _239);
    float _241 = _226 * _237;
    float3 _242 = float3(_241, _239, 0.0f);
    float4 _249 = terrain_heightmap.GatherRed(sampler_default, (_168.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, int2(0, 0));
    float _254 = _249.z;
    float _255 = _249.y;
    float _257 = _249.w;
    float _258 = _249.x;
    float2 _267 = ((float2((_254 + _255) - (_257 + _258), (_258 + _255) - (_257 + _254)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(_Globals_terrain_heightmap_dims));
    float _278 = _205 * 0.20000000298023223876953125f;
    float3 _280 = float3(min(1.0f, _205 * 0.800000011920928955078125f) * 0.3599999845027923583984375f, _278, 1.0f) * 2.5f;
    float _303 = max(30.0f / (((-1.0f) / mul(frame_constants_view_constants[0u].world_to_view, float4(_168, 1.0f)).z) * ((0.5f * frame_constants_view_constants[0u].view_to_clip[1].y) * float(clamp(frame_constants_inner_rendering_resolution.y, 1080u, 2160u)))), 1.0f);
    float _306 = _280.y;
    float _308 = _168.y - ((_303 - 1.0f) * _306);
    float3 _309 = _168;
    _309.y = _308;
    float _310 = _306 * _303;
    float3 _316 = float3((float((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 16) >> 24u) & 255u) * 0.007843137718737125396728515625f) - 1.0f, 0.0f, spvUnpackHalf2x16((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 24) >> 16u) & 65535u).x);
    float3 _317 = normalize(_316);
    float4x4 _329 = transpose(float4x4(float4(normalize(cross(_317, float3(0.0f, 1.0f, 0.0f))), 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(_317, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float4 _335 = mul(frame_constants_view_constants[_Globals_current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float _346 = clamp(_239, 0.0f, 1.0f);
    float _347 = 3.0f * ((0.02999999932944774627685546875f * spvUnpackHalf2x16((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 20) >> 16u) & 65535u).x) * lerp(1.0f, 2.0f, abs(normalize((_335.xyz / _335.w.xxx) - _309).y)));
    float _349 = _347 * (_346 * _346);
    float _350 = _346 + 0.00010001659393310546875f;
    _242.z = _349;
    float3 _362;
    if (frame_constants_glade_theme_id == 4u)
    {
        float3 _361 = _242;
        _361.x = _241 * 0.5f;
        _362 = _361;
    }
    else
    {
        _362 = _242;
    }
    float _363 = float((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 16) >> 16u) & 255u) * (-0.0061568631790578365325927734375f);
    float3 _364 = normalize(float3(1.0f, 0.0f, 0.0f));
    float _365 = sin(_363);
    float _366 = cos(_363);
    float _367 = 1.0f - _366;
    float _368 = _364.x;
    float _369 = _367 * _368;
    float _372 = _364.y;
    float _373 = _369 * _372;
    float _374 = _364.z;
    float _375 = _374 * _365;
    float _377 = _367 * _374;
    float _378 = _377 * _368;
    float _379 = _372 * _365;
    float _382 = _367 * _372;
    float _385 = _382 * _374;
    float _386 = _368 * _365;
    float4x4 _396 = transpose(float4x4(float4((_369 * _368) + _366, _373 - _375, _378 + _379, 0.0f), float4(_373 + _375, (_382 * _372) + _366, _385 - _386, 0.0f), float4(_378 - _379, _385 + _386, (_377 * _374) + _366, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float3 _400 = normalize(cross(float3(1.0f, 0.0f, 0.0f), float3(0.0f, 9.9999997473787516355514526367188e-05f * _310, (_347 * (_350 * _350)) - _349)));
    _400.x = _400.x + (0.300000011920928955078125f * _226);
    float4 _411 = mul(_329, mul(_396, float4(normalize(_400), 0.0f)));
    float3 _412 = _411.xyz;
    float4 _421 = mul(_329, mul(_396, mul(float4x4(float4((_280.x * (float((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 16) >> 8u) & 255u) * 0.0039215688593685626983642578125f)) * _303, 0.0f, 0.0f, 0.0f), float4(0.0f, _310, 0.0f, 0.0f), float4(0.0f, 0.0f, _280.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(_362, 1.0f)))) + float4(_168.x, _308, _168.z, 0.0f);
    float2 _428 = (_309.xz * 0.100000001490116119384765625f) + (float2(_Globals_time, 0.0f) * 1.2000000476837158203125f);
    int2 _430 = int2(floor(_428));
    float2 _431 = frac(_428);
    uint2 _432 = uint2(_430);
    uint _434 = _432.y;
    uint _436 = _434 + (_434 << 10u);
    uint _438 = _436 ^ (_436 >> 6u);
    uint _440 = _438 + (_438 << 3u);
    uint _442 = _440 ^ (_440 >> 11u);
    uint _448 = (((_432.x * 1664525u) + (_442 + (_442 << 15u))) + 1013904223u) * 1664525u;
    uint _450 = _448 ^ (_448 >> 11u);
    uint _453 = _450 ^ ((_450 << 7u) & 2636928640u);
    uint _456 = _453 ^ ((_453 << 15u) & 4022730752u);
    uint2 _464 = uint2(_430 + int2(1, 0));
    uint _466 = _464.y;
    uint _468 = _466 + (_466 << 10u);
    uint _470 = _468 ^ (_468 >> 6u);
    uint _472 = _470 + (_470 << 3u);
    uint _474 = _472 ^ (_472 >> 11u);
    uint _480 = (((_464.x * 1664525u) + (_474 + (_474 << 15u))) + 1013904223u) * 1664525u;
    uint _482 = _480 ^ (_480 >> 11u);
    uint _485 = _482 ^ ((_482 << 7u) & 2636928640u);
    uint _488 = _485 ^ ((_485 << 15u) & 4022730752u);
    uint2 _496 = uint2(_430 + int2(0, 1));
    uint _498 = _496.y;
    uint _500 = _498 + (_498 << 10u);
    uint _502 = _500 ^ (_500 >> 6u);
    uint _504 = _502 + (_502 << 3u);
    uint _506 = _504 ^ (_504 >> 11u);
    uint _512 = (((_496.x * 1664525u) + (_506 + (_506 << 15u))) + 1013904223u) * 1664525u;
    uint _514 = _512 ^ (_512 >> 11u);
    uint _517 = _514 ^ ((_514 << 7u) & 2636928640u);
    uint _520 = _517 ^ ((_517 << 15u) & 4022730752u);
    uint2 _528 = uint2(_430 + int2(1, 1));
    uint _530 = _528.y;
    uint _532 = _530 + (_530 << 10u);
    uint _534 = _532 ^ (_532 >> 6u);
    uint _536 = _534 + (_534 << 3u);
    uint _538 = _536 ^ (_536 >> 11u);
    uint _544 = (((_528.x * 1664525u) + (_538 + (_538 << 15u))) + 1013904223u) * 1664525u;
    uint _546 = _544 ^ (_544 >> 11u);
    uint _549 = _546 ^ ((_546 << 7u) & 2636928640u);
    uint _552 = _549 ^ ((_549 << 15u) & 4022730752u);
    float2 _562 = (_431 * _431) * (3.0f.xx - (_431 * 2.0f));
    float _563 = _562.x;
    float _564 = 1.0f - _563;
    float _566 = _562.y;
    float _578 = _239 * _239;
    float _586 = (((smoothstep(-0.20000000298023223876953125f, 0.5f, _278) * _578) * (sin((((((1.0f - _566) * (((asfloat(((_456 ^ (_456 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _564) + ((asfloat(((_488 ^ (_488 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _563))) + (((asfloat(((_520 ^ (_520 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _564) * _566)) + (((asfloat(((_552 ^ (_552 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _563) * _566)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * spvUnpackHalf2x16(grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 24) & 65535u).x;
    _412.y = _411.y - (_586 * 0.699999988079071044921875f);
    _421.x = _421.x - (_586 * 0.25f);
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, _421);
    out_var_C0 = lerp(_183 * 0.800000011920928955078125f, _183, _578.xxx);
    out_var_C2 = cross(normalize(float3(0.0f, _267.y, 1.0f)), normalize(float3(1.0f, _267.x, 0.0f)));
    out_var_C3 = _421.xyz;
    out_var_C4 = _309;
    out_var_C5 = normalize(_412);
    out_var_C6 = _316;
    out_var_C7 = _240;
    out_var_C8 = _188;
    out_var_C9 = float3(_240 * float2(_237, 1.0f), _237);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
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
static float2 in_var_C7;
static float in_var_C8;
static float3 in_var_C9;

struct SPIRV_Cross_Input
{
    [[vk::location(7)]] float2 in_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float in_var_C8 : TEXCOORD8;
    [[vk::location(9)]] float3 in_var_C9 : TEXCOORD9;
};

void frag_main()
{
    if (abs(in_var_C9.x / in_var_C9.z) < lerp((-0.800000011920928955078125f) / in_var_C8, 1.0f, in_var_C7.y))
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C7 = stage_input.in_var_C7;
    in_var_C8 = stage_input.in_var_C8;
    in_var_C9 = stage_input.in_var_C9;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
#line 15 "shaders/grass_lod1.raster.hlsl"


struct LOD1_VS_OUT
{
    float3 C0 : TEXCOORD0;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float3 C4 : TEXCOORD4;
    float3 C5 : TEXCOORD5;
    float3 C6 : TEXCOORD6;
    float2 C7 : TEXCOORD7;
    float C8 : TEXCOORD8;
    float3 C9 : TEXCOORD9;
    float4 position : SV_Position;
};

LOD1_VS_OUT vs_main(uint vertex_id : SV_VertexID)
{
    LOD1_VS_OUT o;

    float2 st;
    uint tri = vertex_id % 3u;
    if (tri == 0u)
    {
        st = float2(-1.0f, 1.1f);
    }
    else if (tri == 1u)
    {
        st = float2(0.0f, 1.0f);
    }
    else
    {
        st = float2(1.0f, 1.0f);
    }
    float3 blade_geom = float3(st.x * 0.135f, st.y, 0.0f);

    uint blade_index = (grass_buffer_size - 1u) - (vertex_id / 3u);
    GrassBladeDataPacked blade = grass_blade_buffer_data[blade_index];
    float3 blade_pos = blade.position;
    float3 albedo = blade_color_rgb(blade);
    float3 albedo2 = albedo * albedo;
    float2 widths = grass_blade_widths(blade);
    float width_bottom = widths.x;

    float4 gathered = terrain_heightmap.GatherRed(sampler_default, (blade_pos.xz * (1.0f / 130.0f)) + 0.5f, int2(0, 0));
    float2 grad = ((float2((gathered.z + gathered.y) - (gathered.w + gathered.x),
                           (gathered.x + gathered.y) - (gathered.w + gathered.z)) * 0.5f) * 26.5f)
        / (130.0f / float2(terrain_heightmap_dims));
    float bend_amt = width_bottom * 0.2f;
    float3 blade_scale = float3(min(1.0f, width_bottom * 0.8f) * 0.36f, bend_amt, 1.0f) * 2.5f;

    float dist_scale = max(30.0f / (((-1.0f) / mul(view_constants[0].world_to_view, float4(blade_pos, 1.0f)).z)
        * ((0.5f * view_constants[0].view_to_clip[1].y) * float(clamp(inner_rendering_resolution.y, 1080u, 2160u)))), 1.0f);
    float bend_offset = blade_pos.y - ((dist_scale - 1.0f) * blade_scale.y);
    float3 ground_pos = blade_pos;
    ground_pos.y = bend_offset;
    float wind_scale = blade_scale.y * dist_scale;

    float3 axis = normalize(float3((float((blade.packed0 >> 24u) & 255u) * (2.0f / 255.0f)) - 1.0f, 0.0f, grass_blade_bend_xz(blade)));
    float4x4 basis = transpose(float4x4(float4(normalize(cross(axis, float3(0.0f, 1.0f, 0.0f))), 0.0f),
                                         float4(0.0f, 1.0f, 0.0f, 0.0f),
                                         float4(axis, 0.0f),
                                         float4(0.0f, 0.0f, 0.0f, 1.0f)));

    float4 cam_pos = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float h_clamped = clamp(st.y, 0.0f, 1.0f);
    float curve = 3.0f * ((0.03f * widths.y) * lerp(1.0f, 2.0f, abs(normalize((cam_pos.xyz / cam_pos.w) - ground_pos).y)));
    float curve_factor = curve * (h_clamped * h_clamped);
    blade_geom.z = curve_factor;

    float3 blade_geom2;
    if (glade_theme_id == 4u)
    {
        blade_geom2 = blade_geom;
        blade_geom2.x = st.x * 0.0675f;
    }
    else
    {
        blade_geom2 = blade_geom;
    }

    
    float bend_angle = float((blade.packed0 >> 16u) & 255u) * (-0.0061568631790578365325927734375f);
    float sina = sin(bend_angle);
    float cosa = cos(bend_angle);
    float oc = 1.0f - cosa;
    float axis_x = 1.0f;
    float axis_y = 0.0f;
    float axis_z = 0.0f;
    float t_oc_x = oc * axis_x;
    float t_oc_xy = t_oc_x * axis_y;
    float t_z_sin = axis_z * sina;
    float t_oc_z = oc * axis_z;
    float t_oc_zx = t_oc_z * axis_x;
    float t_y_sin = axis_y * sina;
    float t_oc_y = oc * axis_y;
    float t_oc_yz = t_oc_y * axis_z;
    float t_x_sin = axis_x * sina;
    float4x4 twist = transpose(float4x4(float4((t_oc_x * axis_x) + cosa, t_oc_xy - t_z_sin, t_oc_zx + t_y_sin, 0.0f),
                                        float4(t_oc_xy + t_z_sin, (t_oc_y * axis_y) + cosa, t_oc_yz - t_x_sin, 0.0f),
                                        float4(t_oc_zx - t_y_sin, t_oc_yz + t_x_sin, (t_oc_z * axis_z) + cosa, 0.0f),
                                        float4(0.0f, 0.0f, 0.0f, 1.0f)));

    float3 blade_normal = normalize(cross(float3(1.0f, 0.0f, 0.0f), float3(0.0f, 1e-5f * wind_scale, (curve * ((h_clamped + 0.0001f) * (h_clamped + 0.0001f))) - curve_factor)));
    blade_normal.x += 0.3f * st.x;

    float4 rotated = mul(basis, mul(twist, float4(normalize(blade_normal), 0.0f)));
    float3 rotated_xyz = rotated.xyz;
    float4 world_offset = mul(basis, mul(twist, mul(float4x4(float4(blade_scale.x * (float((blade.packed0 >> 8u) & 255u) * (1.0f / 255.0f)) * dist_scale, 0.0f, 0.0f, 0.0f),
                                                          float4(0.0f, wind_scale, 0.0f, 0.0f),
                                                          float4(0.0f, 0.0f, blade_scale.z, 0.0f),
                                                          float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(blade_geom2, 1.0f)))) + float4(blade_pos.x, bend_offset, blade_pos.z, 0.0f);

    
    float2 wind_uv = (ground_pos.xz * 0.1f) + (float2(time, 0.0f) * 1.2f);
    int2 wind_cell = int2(floor(wind_uv));
    float2 wind_frac = frac(wind_uv);
    float2 wind_smooth = wind_frac * wind_frac * (3.0f - (2.0f * wind_frac));
    float wind_fx = wind_smooth.x;
    float wind_fy = wind_smooth.y;
    float wind_n00 = incb_perlin_hash01(hash(uint2(wind_cell + int2(0, 0))));
    float wind_n10 = incb_perlin_hash01(hash(uint2(wind_cell + int2(1, 0))));
    float wind_n01 = incb_perlin_hash01(hash(uint2(wind_cell + int2(0, 1))));
    float wind_n11 = incb_perlin_hash01(hash(uint2(wind_cell + int2(1, 1))));
    float wind_noise = ((1.0f - wind_fy) * ((wind_n00 * (1.0f - wind_fx)) + (wind_n10 * wind_fx)))
        + ((wind_n01 * (1.0f - wind_fx)) * wind_fy)
        + ((wind_n11 * wind_fx) * wind_fy);
    float wind = (smoothstep(-0.2f, 0.5f, bend_amt) * (st.y * st.y))
        * (sin((wind_noise * 3.14f) - 1.57f + 0.3f)) * 0.7f * grass_blade_bend_xz(blade);
    rotated_xyz.y -= wind * 0.7f;
    world_offset.x -= wind * 0.25f;

    o.position = mul(view_constants[current_view_index].world_to_sample, world_offset);
    o.C0 = lerp(albedo2 * 0.8f, albedo2, (st.y * st.y).xxx);
    o.C2 = cross(normalize(float3(0.0f, grad.y, 1.0f)), normalize(float3(1.0f, grad.x, 0.0f)));
    o.C3 = world_offset.xyz;
    o.C4 = ground_pos;
    o.C5 = normalize(rotated_xyz);
    o.C6 = axis;
    o.C7 = st;
    o.C8 = 1.0f;
    o.C9 = float3(st * float2(st.y, 1.0f), st.y);
    return o;
}

struct LOD1_PS_IN
{
    float3 C0 : TEXCOORD0;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float3 C5 : TEXCOORD5;
    float2 C7 : TEXCOORD7;
    float3 C9 : TEXCOORD9;
};

struct LOD1_PS_OUT
{
    uint2 target : SV_Target0;
};

LOD1_PS_OUT ps_main(LOD1_PS_IN i)
{
    LOD1_PS_OUT o;
    float4 cam_pos = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    if (abs(i.C9.x / i.C9.z) < lerp(-0.8f, 1.0f, i.C7.y))
    {
        discard;
    }
    float3 view_normal;
    if (dot(normalize((cam_pos.xyz / cam_pos.w) - i.C3), i.C5) <= 0.0f)
    {
        view_normal = -i.C5;
    }
    else
    {
        view_normal = i.C5;
    }
    float3 n = normalize(view_normal);
    float3 albedo = (i.C0 * 0.55f) * lerp(0.7f, 1.1f, i.C7.y);
    float3 albedo_out;
    if (glade_theme_id == 4u)
    {
        albedo_out = lerp(0.9f.xxx, albedo * 0.5f, pow(i.C7.y, 0.5f).xxx);
    }
    else
    {
        albedo_out = albedo;
    }
    float3 octa = n / ((abs(n.x) + abs(n.y)) + abs(n.z));
    if (octa.z < 0.0f)
    {
        float2 xy = (1.0f - abs(octa.yx)) * ((step(0.0f, octa.xy) * 2.0f) - 1.0f);
        octa = float3(xy.x, xy.y, octa.z);
    }
    float2 octa_uv = (octa.xy * 0.5f) + 0.5f;
    float3 packed_n = (normalize(i.C2) * 0.5f) + 0.5f;
    float3 albedo_sqrt = sqrt(clamp(albedo_out, 0.0f, 1.0f));
    o.target = uint2(pack_normal8(packed_n) + (uint(clamp(octa_uv.x, 0.0f, 1.0f) * 255.0f + 0.5f) << 24u),
                     pack_normal8(albedo_sqrt) + (uint(clamp(octa_uv.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 24u));
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

struct GrassBladeDataPacked
{
    float3 position;
    uint color;
    uint packed0;
    uint packed1;
    uint packed2;
    uint pad;
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
    float _Globals_time : packoffset(c0.y);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c0.z);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c1);
    uint _Globals_grass_buffer_size : packoffset(c1.z);
};

ByteAddressBuffer grass_blade_buffer_data : register(t17, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t15, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static float3 out_var_C0;
static float3 out_var_C2;
static float3 out_var_C3;
static float3 out_var_C4;
static float3 out_var_C5;
static float3 out_var_C6;
static float2 out_var_C7;
static float out_var_C8;
static float3 out_var_C9;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float3 out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float3 out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] float2 out_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float out_var_C8 : TEXCOORD8;
    [[vk::location(9)]] float3 out_var_C9 : TEXCOORD9;
    float4 gl_Position : SV_Position;
};

uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void vert_main()
{
    uint _168 = uint(gl_VertexIndex) % 3u;
    float2 _176;
    if (_168 == 0u)
    {
        _176 = float2(-1.0f, 1.10000002384185791015625f);
    }
    else
    {
        bool2 _174 = (_168 == 1u).xx;
        _176 = float2(_174.x ? 0.0f.xx.x : 1.0f.xx.x, _174.y ? 0.0f.xx.y : 1.0f.xx.y);
    }
    float3 _180 = float3(_176.x * 0.13500000536441802978515625f, _176.y, 0.0f);
    uint _185 = (_Globals_grass_buffer_size - 1u) - (uint(gl_VertexIndex) / 3u);
    float3 _187 = grass_blade_buffer_data.Load<float3>(_185 * 32 + 0);
    float3 _201 = float3(float(grass_blade_buffer_data.Load<uint>(_185 * 32 + 12) & 255u) * 0.0039215688593685626983642578125f, float((grass_blade_buffer_data.Load<uint>(_185 * 32 + 12) >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((grass_blade_buffer_data.Load<uint>(_185 * 32 + 12) >> 16u) & 255u) * 0.0039215688593685626983642578125f);
    float3 _202 = _201 * _201;
    float2 _220 = spvUnpackHalf2x16(grass_blade_buffer_data.Load<uint>(_185 * 32 + 20) & 65535u);
    float _221 = _220.x;
    float4 _241 = terrain_heightmap.GatherRed(sampler_default, (_187.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, int2(0, 0));
    float _246 = _241.z;
    float _247 = _241.y;
    float _249 = _241.w;
    float _250 = _241.x;
    float2 _259 = ((float2((_246 + _247) - (_249 + _250), (_250 + _247) - (_249 + _246)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(_Globals_terrain_heightmap_dims));
    float _270 = _221 * 0.20000000298023223876953125f;
    float3 _272 = float3(min(1.0f, _221 * 0.800000011920928955078125f) * 0.3599999845027923583984375f, _270, 1.0f) * 2.5f;
    float _295 = max(30.0f / (((-1.0f) / mul(frame_constants_view_constants[0u].world_to_view, float4(_187, 1.0f)).z) * ((0.5f * frame_constants_view_constants[0u].view_to_clip[1].y) * float(clamp(frame_constants_inner_rendering_resolution.y, 1080u, 2160u)))), 1.0f);
    float _298 = _272.y;
    float _300 = _187.y - ((_295 - 1.0f) * _298);
    float3 _301 = _187;
    _301.y = _300;
    float _302 = _298 * _295;
    float3 _308 = float3((float((grass_blade_buffer_data.Load<uint>(_185 * 32 + 16) >> 24u) & 255u) * 0.007843137718737125396728515625f) - 1.0f, 0.0f, spvUnpackHalf2x16((grass_blade_buffer_data.Load<uint>(_185 * 32 + 24) >> 16u) & 65535u).x);
    float3 _309 = normalize(_308);
    float4x4 _321 = transpose(float4x4(float4(normalize(cross(_309, float3(0.0f, 1.0f, 0.0f))), 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(_309, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float4 _327 = mul(frame_constants_view_constants[_Globals_current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float _338 = clamp(_176.y, 0.0f, 1.0f);
    float _339 = 3.0f * ((0.02999999932944774627685546875f * spvUnpackHalf2x16((grass_blade_buffer_data.Load<uint>(_185 * 32 + 20) >> 16u) & 65535u).x) * lerp(1.0f, 2.0f, abs(normalize((_327.xyz / _327.w.xxx) - _301).y)));
    float _341 = _339 * (_338 * _338);
    float _342 = _338 + 0.00010001659393310546875f;
    _180.z = _341;
    float3 _354;
    if (frame_constants_glade_theme_id == 4u)
    {
        float3 _353 = _180;
        _353.x = _176.x * 0.067500002682209014892578125f;
        _354 = _353;
    }
    else
    {
        _354 = _180;
    }
    float _355 = float((grass_blade_buffer_data.Load<uint>(_185 * 32 + 16) >> 16u) & 255u) * (-0.0061568631790578365325927734375f);
    float3 _356 = normalize(float3(1.0f, 0.0f, 0.0f));
    float _357 = sin(_355);
    float _358 = cos(_355);
    float _359 = 1.0f - _358;
    float _360 = _356.x;
    float _361 = _359 * _360;
    float _364 = _356.y;
    float _365 = _361 * _364;
    float _366 = _356.z;
    float _367 = _366 * _357;
    float _369 = _359 * _366;
    float _370 = _369 * _360;
    float _371 = _364 * _357;
    float _374 = _359 * _364;
    float _377 = _374 * _366;
    float _378 = _360 * _357;
    float4x4 _388 = transpose(float4x4(float4((_361 * _360) + _358, _365 - _367, _370 + _371, 0.0f), float4(_365 + _367, (_374 * _364) + _358, _377 - _378, 0.0f), float4(_370 - _371, _377 + _378, (_369 * _366) + _358, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float3 _392 = normalize(cross(float3(1.0f, 0.0f, 0.0f), float3(0.0f, 9.9999997473787516355514526367188e-05f * _302, (_339 * (_342 * _342)) - _341)));
    _392.x = _392.x + (0.300000011920928955078125f * _176.x);
    float4 _403 = mul(_321, mul(_388, float4(normalize(_392), 0.0f)));
    float3 _404 = _403.xyz;
    float4 _413 = mul(_321, mul(_388, mul(float4x4(float4((_272.x * (float((grass_blade_buffer_data.Load<uint>(_185 * 32 + 16) >> 8u) & 255u) * 0.0039215688593685626983642578125f)) * _295, 0.0f, 0.0f, 0.0f), float4(0.0f, _302, 0.0f, 0.0f), float4(0.0f, 0.0f, _272.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(_354, 1.0f)))) + float4(_187.x, _300, _187.z, 0.0f);
    float2 _420 = (_301.xz * 0.100000001490116119384765625f) + (float2(_Globals_time, 0.0f) * 1.2000000476837158203125f);
    int2 _422 = int2(floor(_420));
    float2 _423 = frac(_420);
    uint2 _424 = uint2(_422);
    uint _426 = _424.y;
    uint _428 = _426 + (_426 << 10u);
    uint _430 = _428 ^ (_428 >> 6u);
    uint _432 = _430 + (_430 << 3u);
    uint _434 = _432 ^ (_432 >> 11u);
    uint _440 = (((_424.x * 1664525u) + (_434 + (_434 << 15u))) + 1013904223u) * 1664525u;
    uint _442 = _440 ^ (_440 >> 11u);
    uint _445 = _442 ^ ((_442 << 7u) & 2636928640u);
    uint _448 = _445 ^ ((_445 << 15u) & 4022730752u);
    uint2 _456 = uint2(_422 + int2(1, 0));
    uint _458 = _456.y;
    uint _460 = _458 + (_458 << 10u);
    uint _462 = _460 ^ (_460 >> 6u);
    uint _464 = _462 + (_462 << 3u);
    uint _466 = _464 ^ (_464 >> 11u);
    uint _472 = (((_456.x * 1664525u) + (_466 + (_466 << 15u))) + 1013904223u) * 1664525u;
    uint _474 = _472 ^ (_472 >> 11u);
    uint _477 = _474 ^ ((_474 << 7u) & 2636928640u);
    uint _480 = _477 ^ ((_477 << 15u) & 4022730752u);
    uint2 _488 = uint2(_422 + int2(0, 1));
    uint _490 = _488.y;
    uint _492 = _490 + (_490 << 10u);
    uint _494 = _492 ^ (_492 >> 6u);
    uint _496 = _494 + (_494 << 3u);
    uint _498 = _496 ^ (_496 >> 11u);
    uint _504 = (((_488.x * 1664525u) + (_498 + (_498 << 15u))) + 1013904223u) * 1664525u;
    uint _506 = _504 ^ (_504 >> 11u);
    uint _509 = _506 ^ ((_506 << 7u) & 2636928640u);
    uint _512 = _509 ^ ((_509 << 15u) & 4022730752u);
    uint2 _520 = uint2(_422 + int2(1, 1));
    uint _522 = _520.y;
    uint _524 = _522 + (_522 << 10u);
    uint _526 = _524 ^ (_524 >> 6u);
    uint _528 = _526 + (_526 << 3u);
    uint _530 = _528 ^ (_528 >> 11u);
    uint _536 = (((_520.x * 1664525u) + (_530 + (_530 << 15u))) + 1013904223u) * 1664525u;
    uint _538 = _536 ^ (_536 >> 11u);
    uint _541 = _538 ^ ((_538 << 7u) & 2636928640u);
    uint _544 = _541 ^ ((_541 << 15u) & 4022730752u);
    float2 _554 = (_423 * _423) * (3.0f.xx - (_423 * 2.0f));
    float _555 = _554.x;
    float _556 = 1.0f - _555;
    float _558 = _554.y;
    float _570 = _176.y * _176.y;
    float _578 = (((smoothstep(-0.20000000298023223876953125f, 0.5f, _270) * _570) * (sin((((((1.0f - _558) * (((asfloat(((_448 ^ (_448 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _556) + ((asfloat(((_480 ^ (_480 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _555))) + (((asfloat(((_512 ^ (_512 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _556) * _558)) + (((asfloat(((_544 ^ (_544 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _555) * _558)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * spvUnpackHalf2x16(grass_blade_buffer_data.Load<uint>(_185 * 32 + 24) & 65535u).x;
    _404.y = _403.y - (_578 * 0.699999988079071044921875f);
    _413.x = _413.x - (_578 * 0.25f);
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, _413);
    out_var_C0 = lerp(_202 * 0.800000011920928955078125f, _202, _570.xxx);
    out_var_C2 = cross(normalize(float3(0.0f, _259.y, 1.0f)), normalize(float3(1.0f, _259.x, 0.0f)));
    out_var_C3 = _413.xyz;
    out_var_C4 = _301;
    out_var_C5 = normalize(_404);
    out_var_C6 = _308;
    out_var_C7 = _176;
    out_var_C8 = 1.0f;
    out_var_C9 = float3(_176 * float2(_176.y, 1.0f), _176.y);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
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
    float _Globals_time : packoffset(c0.y);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c0.z);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c1);
    uint _Globals_grass_buffer_size : packoffset(c1.z);
};


static float3 in_var_C0;
static float3 in_var_C2;
static float3 in_var_C3;
static float3 in_var_C5;
static float2 in_var_C7;
static float3 in_var_C9;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_C0 : TEXCOORD0;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
    [[vk::location(5)]] float3 in_var_C5 : TEXCOORD5;
    [[vk::location(7)]] float2 in_var_C7 : TEXCOORD7;
    [[vk::location(9)]] float3 in_var_C9 : TEXCOORD9;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    float4 _105 = mul(frame_constants_view_constants[_Globals_current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    if (abs(in_var_C9.x / in_var_C9.z) < lerp(-0.800000011920928955078125f, 1.0f, in_var_C7.y))
    {
        discard;
    }
    float3 _126;
    if (dot(normalize((_105.xyz / _105.w.xxx) - in_var_C3), in_var_C5) <= 0.0f)
    {
        _126 = -in_var_C5;
    }
    else
    {
        _126 = in_var_C5;
    }
    float3 _127 = normalize(_126);
    float3 _130 = (in_var_C0 * 0.550000011920928955078125f) * lerp(0.699999988079071044921875f, 1.10000002384185791015625f, in_var_C7.y);
    float3 _140;
    if (frame_constants_glade_theme_id == 4u)
    {
        _140 = lerp(0.89999997615814208984375f.xxx, _130 * 0.5f, pow(in_var_C7.y, 0.5f).xxx);
    }
    else
    {
        _140 = _130;
    }
    float3 _150 = _127 / ((abs(_127.x) + abs(_127.y)) + abs(_127.z)).xxx;
    float3 _164;
    if (_150.z < 0.0f)
    {
        float2 _162 = (1.0f.xx - abs(_150.yx)) * ((step(0.0f.xx, _150.xy) * 2.0f) - 1.0f.xx);
        _164 = float3(_162.x, _162.y, _150.z);
    }
    else
    {
        _164 = _150;
    }
    float2 _167 = (_164.xy * 0.5f) + 0.5f.xx;
    float3 _169 = (normalize(in_var_C2) * 0.5f) + 0.5f.xxx;
    float3 _197 = sqrt(clamp(_140, 0.0f.xxx, 1.0f.xxx));
    out_var_SV_TARGET0 = uint2(((uint((clamp(_169.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_169.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_169.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + (uint((clamp(_167.x, 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u), ((uint((clamp(_197.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_197.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_197.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + (uint((clamp(_167.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u));
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C0 = stage_input.in_var_C0;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C5 = stage_input.in_var_C5;
    in_var_C7 = stage_input.in_var_C7;
    in_var_C9 = stage_input.in_var_C9;
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

struct GrassBladeDataPacked
{
    float3 position;
    uint color;
    uint packed0;
    uint packed1;
    uint packed2;
    uint pad;
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
    float _Globals_time : packoffset(c0.y);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c0.z);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c1);
    uint _Globals_grass_buffer_size : packoffset(c1.z);
};

ByteAddressBuffer grass_blade_buffer_data : register(t17, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t15, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static float3 out_var_C0;
static float3 out_var_C2;
static float3 out_var_C3;
static float3 out_var_C4;
static float3 out_var_C5;
static float3 out_var_C6;
static float2 out_var_C7;
static float out_var_C8;
static float3 out_var_C9;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float3 out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float3 out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] float2 out_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float out_var_C8 : TEXCOORD8;
    [[vk::location(9)]] float3 out_var_C9 : TEXCOORD9;
    float4 gl_Position : SV_Position;
};

uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void vert_main()
{
    uint _168 = uint(gl_VertexIndex) % 3u;
    float2 _176;
    if (_168 == 0u)
    {
        _176 = float2(-1.0f, 1.10000002384185791015625f);
    }
    else
    {
        bool2 _174 = (_168 == 1u).xx;
        _176 = float2(_174.x ? 0.0f.xx.x : 1.0f.xx.x, _174.y ? 0.0f.xx.y : 1.0f.xx.y);
    }
    float3 _180 = float3(_176.x * 0.13500000536441802978515625f, _176.y, 0.0f);
    uint _185 = (_Globals_grass_buffer_size - 1u) - (uint(gl_VertexIndex) / 3u);
    float3 _187 = grass_blade_buffer_data.Load<float3>(_185 * 32 + 0);
    float3 _201 = float3(float(grass_blade_buffer_data.Load<uint>(_185 * 32 + 12) & 255u) * 0.0039215688593685626983642578125f, float((grass_blade_buffer_data.Load<uint>(_185 * 32 + 12) >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((grass_blade_buffer_data.Load<uint>(_185 * 32 + 12) >> 16u) & 255u) * 0.0039215688593685626983642578125f);
    float3 _202 = _201 * _201;
    float2 _220 = spvUnpackHalf2x16(grass_blade_buffer_data.Load<uint>(_185 * 32 + 20) & 65535u);
    float _221 = _220.x;
    float4 _241 = terrain_heightmap.GatherRed(sampler_default, (_187.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, int2(0, 0));
    float _246 = _241.z;
    float _247 = _241.y;
    float _249 = _241.w;
    float _250 = _241.x;
    float2 _259 = ((float2((_246 + _247) - (_249 + _250), (_250 + _247) - (_249 + _246)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(_Globals_terrain_heightmap_dims));
    float _270 = _221 * 0.20000000298023223876953125f;
    float3 _272 = float3(min(1.0f, _221 * 0.800000011920928955078125f) * 0.3599999845027923583984375f, _270, 1.0f) * 2.5f;
    float _295 = max(30.0f / (((-1.0f) / mul(frame_constants_view_constants[0u].world_to_view, float4(_187, 1.0f)).z) * ((0.5f * frame_constants_view_constants[0u].view_to_clip[1].y) * float(clamp(frame_constants_inner_rendering_resolution.y, 1080u, 2160u)))), 1.0f);
    float _298 = _272.y;
    float _300 = _187.y - ((_295 - 1.0f) * _298);
    float3 _301 = _187;
    _301.y = _300;
    float _302 = _298 * _295;
    float3 _308 = float3((float((grass_blade_buffer_data.Load<uint>(_185 * 32 + 16) >> 24u) & 255u) * 0.007843137718737125396728515625f) - 1.0f, 0.0f, spvUnpackHalf2x16((grass_blade_buffer_data.Load<uint>(_185 * 32 + 24) >> 16u) & 65535u).x);
    float3 _309 = normalize(_308);
    float4x4 _321 = transpose(float4x4(float4(normalize(cross(_309, float3(0.0f, 1.0f, 0.0f))), 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(_309, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float4 _327 = mul(frame_constants_view_constants[_Globals_current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float _338 = clamp(_176.y, 0.0f, 1.0f);
    float _339 = 3.0f * ((0.02999999932944774627685546875f * spvUnpackHalf2x16((grass_blade_buffer_data.Load<uint>(_185 * 32 + 20) >> 16u) & 65535u).x) * lerp(1.0f, 2.0f, abs(normalize((_327.xyz / _327.w.xxx) - _301).y)));
    float _341 = _339 * (_338 * _338);
    float _342 = _338 + 0.00010001659393310546875f;
    _180.z = _341;
    float3 _354;
    if (frame_constants_glade_theme_id == 4u)
    {
        float3 _353 = _180;
        _353.x = _176.x * 0.067500002682209014892578125f;
        _354 = _353;
    }
    else
    {
        _354 = _180;
    }
    float _355 = float((grass_blade_buffer_data.Load<uint>(_185 * 32 + 16) >> 16u) & 255u) * (-0.0061568631790578365325927734375f);
    float3 _356 = normalize(float3(1.0f, 0.0f, 0.0f));
    float _357 = sin(_355);
    float _358 = cos(_355);
    float _359 = 1.0f - _358;
    float _360 = _356.x;
    float _361 = _359 * _360;
    float _364 = _356.y;
    float _365 = _361 * _364;
    float _366 = _356.z;
    float _367 = _366 * _357;
    float _369 = _359 * _366;
    float _370 = _369 * _360;
    float _371 = _364 * _357;
    float _374 = _359 * _364;
    float _377 = _374 * _366;
    float _378 = _360 * _357;
    float4x4 _388 = transpose(float4x4(float4((_361 * _360) + _358, _365 - _367, _370 + _371, 0.0f), float4(_365 + _367, (_374 * _364) + _358, _377 - _378, 0.0f), float4(_370 - _371, _377 + _378, (_369 * _366) + _358, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float3 _392 = normalize(cross(float3(1.0f, 0.0f, 0.0f), float3(0.0f, 9.9999997473787516355514526367188e-05f * _302, (_339 * (_342 * _342)) - _341)));
    _392.x = _392.x + (0.300000011920928955078125f * _176.x);
    float4 _403 = mul(_321, mul(_388, float4(normalize(_392), 0.0f)));
    float3 _404 = _403.xyz;
    float4 _413 = mul(_321, mul(_388, mul(float4x4(float4((_272.x * (float((grass_blade_buffer_data.Load<uint>(_185 * 32 + 16) >> 8u) & 255u) * 0.0039215688593685626983642578125f)) * _295, 0.0f, 0.0f, 0.0f), float4(0.0f, _302, 0.0f, 0.0f), float4(0.0f, 0.0f, _272.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(_354, 1.0f)))) + float4(_187.x, _300, _187.z, 0.0f);
    float2 _420 = (_301.xz * 0.100000001490116119384765625f) + (float2(_Globals_time, 0.0f) * 1.2000000476837158203125f);
    int2 _422 = int2(floor(_420));
    float2 _423 = frac(_420);
    uint2 _424 = uint2(_422);
    uint _426 = _424.y;
    uint _428 = _426 + (_426 << 10u);
    uint _430 = _428 ^ (_428 >> 6u);
    uint _432 = _430 + (_430 << 3u);
    uint _434 = _432 ^ (_432 >> 11u);
    uint _440 = (((_424.x * 1664525u) + (_434 + (_434 << 15u))) + 1013904223u) * 1664525u;
    uint _442 = _440 ^ (_440 >> 11u);
    uint _445 = _442 ^ ((_442 << 7u) & 2636928640u);
    uint _448 = _445 ^ ((_445 << 15u) & 4022730752u);
    uint2 _456 = uint2(_422 + int2(1, 0));
    uint _458 = _456.y;
    uint _460 = _458 + (_458 << 10u);
    uint _462 = _460 ^ (_460 >> 6u);
    uint _464 = _462 + (_462 << 3u);
    uint _466 = _464 ^ (_464 >> 11u);
    uint _472 = (((_456.x * 1664525u) + (_466 + (_466 << 15u))) + 1013904223u) * 1664525u;
    uint _474 = _472 ^ (_472 >> 11u);
    uint _477 = _474 ^ ((_474 << 7u) & 2636928640u);
    uint _480 = _477 ^ ((_477 << 15u) & 4022730752u);
    uint2 _488 = uint2(_422 + int2(0, 1));
    uint _490 = _488.y;
    uint _492 = _490 + (_490 << 10u);
    uint _494 = _492 ^ (_492 >> 6u);
    uint _496 = _494 + (_494 << 3u);
    uint _498 = _496 ^ (_496 >> 11u);
    uint _504 = (((_488.x * 1664525u) + (_498 + (_498 << 15u))) + 1013904223u) * 1664525u;
    uint _506 = _504 ^ (_504 >> 11u);
    uint _509 = _506 ^ ((_506 << 7u) & 2636928640u);
    uint _512 = _509 ^ ((_509 << 15u) & 4022730752u);
    uint2 _520 = uint2(_422 + int2(1, 1));
    uint _522 = _520.y;
    uint _524 = _522 + (_522 << 10u);
    uint _526 = _524 ^ (_524 >> 6u);
    uint _528 = _526 + (_526 << 3u);
    uint _530 = _528 ^ (_528 >> 11u);
    uint _536 = (((_520.x * 1664525u) + (_530 + (_530 << 15u))) + 1013904223u) * 1664525u;
    uint _538 = _536 ^ (_536 >> 11u);
    uint _541 = _538 ^ ((_538 << 7u) & 2636928640u);
    uint _544 = _541 ^ ((_541 << 15u) & 4022730752u);
    float2 _554 = (_423 * _423) * (3.0f.xx - (_423 * 2.0f));
    float _555 = _554.x;
    float _556 = 1.0f - _555;
    float _558 = _554.y;
    float _570 = _176.y * _176.y;
    float _578 = (((smoothstep(-0.20000000298023223876953125f, 0.5f, _270) * _570) * (sin((((((1.0f - _558) * (((asfloat(((_448 ^ (_448 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _556) + ((asfloat(((_480 ^ (_480 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _555))) + (((asfloat(((_512 ^ (_512 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _556) * _558)) + (((asfloat(((_544 ^ (_544 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _555) * _558)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * spvUnpackHalf2x16(grass_blade_buffer_data.Load<uint>(_185 * 32 + 24) & 65535u).x;
    _404.y = _403.y - (_578 * 0.699999988079071044921875f);
    _413.x = _413.x - (_578 * 0.25f);
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, _413);
    out_var_C0 = lerp(_202 * 0.800000011920928955078125f, _202, _570.xxx);
    out_var_C2 = cross(normalize(float3(0.0f, _259.y, 1.0f)), normalize(float3(1.0f, _259.x, 0.0f)));
    out_var_C3 = _413.xyz;
    out_var_C4 = _301;
    out_var_C5 = normalize(_404);
    out_var_C6 = _308;
    out_var_C7 = _176;
    out_var_C8 = 1.0f;
    out_var_C9 = float3(_176 * float2(_176.y, 1.0f), _176.y);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
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
static float2 in_var_C7;
static float3 in_var_C9;

struct SPIRV_Cross_Input
{
    [[vk::location(7)]] float2 in_var_C7 : TEXCOORD7;
    [[vk::location(9)]] float3 in_var_C9 : TEXCOORD9;
};

void frag_main()
{
    if (abs(in_var_C9.x / in_var_C9.z) < lerp(-0.800000011920928955078125f, 1.0f, in_var_C7.y))
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C7 = stage_input.in_var_C7;
    in_var_C9 = stage_input.in_var_C9;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
