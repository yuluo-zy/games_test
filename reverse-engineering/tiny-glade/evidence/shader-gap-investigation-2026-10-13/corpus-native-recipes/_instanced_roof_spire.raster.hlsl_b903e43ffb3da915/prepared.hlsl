#define SHADER_VARIANT_DEPTH_ONLY 0
#define TGV_b903e43ffb3da915 1
#line 1 "shaders/instanced_roof_spire.raster.hlsl"


#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)))


















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
#line 22 "shaders/instanced_roof_spire.raster.hlsl"

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
#line 23 "shaders/instanced_roof_spire.raster.hlsl"

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
#line 24 "shaders/instanced_roof_spire.raster.hlsl"

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
#line 25 "shaders/instanced_roof_spire.raster.hlsl"

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
#line 26 "shaders/instanced_roof_spire.raster.hlsl"

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
#line 27 "shaders/instanced_roof_spire.raster.hlsl"

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
#line 28 "shaders/instanced_roof_spire.raster.hlsl"

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
#line 29 "shaders/instanced_roof_spire.raster.hlsl"

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
#line 30 "shaders/instanced_roof_spire.raster.hlsl"

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
#line 31 "shaders/instanced_roof_spire.raster.hlsl"

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
#line 32 "shaders/instanced_roof_spire.raster.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 33 "shaders/instanced_roof_spire.raster.hlsl"

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
#line 34 "shaders/instanced_roof_spire.raster.hlsl"

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
#line 35 "shaders/instanced_roof_spire.raster.hlsl"

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
#line 36 "shaders/instanced_roof_spire.raster.hlsl"

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
#line 37 "shaders/instanced_roof_spire.raster.hlsl"

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
#line 38 "shaders/instanced_roof_spire.raster.hlsl"

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
#line 39 "shaders/instanced_roof_spire.raster.hlsl"

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
#line 40 "shaders/instanced_roof_spire.raster.hlsl"

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
#line 41 "shaders/instanced_roof_spire.raster.hlsl"

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
#line 42 "shaders/instanced_roof_spire.raster.hlsl"

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
#line 43 "shaders/instanced_roof_spire.raster.hlsl"

#line 1 "shaders/inc/sh.hlsl"










float4 sh_basis(float3 dir)
{
    return float4(0.2820949852466583251953125f, 0.48860299587249755859375f * dir);
}

float sh_eval(float4 sh, float3 dir)
{
    return dot(sh, sh_basis(dir));
}
#line 44 "shaders/instanced_roof_spire.raster.hlsl"



struct RoofSpire
{
    float4 position;
    float roof_animation_t;
    float roof_profile;
    float radius;
    int pad1;
};

ByteAddressBuffer instanced_data : register(t15, space0);




float2 spire_cell_hash01_2(uint3 cell)
{
    uint z = cell.z;
    z += (z << 10u);
    z ^= (z >> 6u);
    z += (z << 3u);
    z ^= (z >> 11u);

    uint h1 = ((cell.y * 1664525u) + (z + (z << 15u))) + 1013904223u;
    h1 *= 1664525u;
    h1 ^= (h1 >> 11u);
    h1 ^= (h1 << 7u) & 2636928640u;
    h1 ^= (h1 << 15u) & 4022730752u;

    uint h2 = ((cell.x * 1664525u) + (h1 ^ (h1 >> 18u))) + 1013904223u;
    h2 *= 1664525u;
    h2 ^= (h2 >> 11u);
    h2 ^= (h2 << 7u) & 2636928640u;
    h2 ^= (h2 << 15u) & 4022730752u;

    uint xv = h2 ^ (h2 >> 18u);
    uint yv = xv;
    yv += (yv << 10u);
    yv ^= (yv >> 6u);
    yv += (yv << 3u);
    yv ^= (yv >> 11u);

    return float2(asfloat((xv & 0x7FFFFFu) | 0x3F800000u) - 1.0f,
                  asfloat(((yv + (yv << 15u)) & 0x7FFFFFu) | 0x3F800000u) - 1.0f);
}



float2 spire_noise3(float3 p)
{
    int3 cell = int3(floor(p));
    float3 f = frac(p);
    float3 u = f * f * (3.0f - 2.0f * f);

    float2 n = spire_cell_hash01_2(uint3(cell)) * (1.0f - u.x) * (1.0f - u.y) * (1.0f - u.z);
    n += spire_cell_hash01_2(uint3(cell + int3(1, 0, 0))) * u.x * (1.0f - u.y) * (1.0f - u.z);
    n += spire_cell_hash01_2(uint3(cell + int3(0, 1, 0))) * (1.0f - u.x) * u.y * (1.0f - u.z);
    n += spire_cell_hash01_2(uint3(cell + int3(1, 1, 0))) * u.x * u.y * (1.0f - u.z);
    n += spire_cell_hash01_2(uint3(cell + int3(0, 0, 1))) * (1.0f - u.x) * (1.0f - u.y) * u.z;
    n += spire_cell_hash01_2(uint3(cell + int3(1, 0, 1))) * u.x * (1.0f - u.y) * u.z;
    n += spire_cell_hash01_2(uint3(cell + int3(0, 1, 1))) * (1.0f - u.x) * u.y * u.z;
    n += spire_cell_hash01_2(uint3(cell + int3(1, 1, 1))) * u.x * u.y * u.z;

    return clamp((n - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
}

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float roof_profile_mult: roof_profile_mult;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float C3 : TEXCOORD3;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    uint base = vin.instance_id * 32u;
    float roof_t = instanced_data.Load<float>(base + 16u);
    float roof_profile = instanced_data.Load<float>(base + 20u);
    float radius = instanced_data.Load<float>(base + 24u);

    
    
    bool is_profile = vin.roof_profile_mult > -0.001f;
    float3 scaled_pos = vin.Vertex_Position;
    if (is_profile)
    {
        float s = max(0.6f, (radius * 2.0f) * lerp(0.6f, 1.0f, vin.roof_profile_mult)
            * lerp(0.4f, 1.5f, 1.0f - roof_t));
        scaled_pos = float3(vin.Vertex_Position.x * s, vin.Vertex_Position.y, vin.Vertex_Position.z * s);
    }

    
    float3 world = ((scaled_pos * ((pow(2.0f, (-10.0f) * roof_t)
        * sin(((roof_t * 10.0f) - 0.75f) * 2.094395160675048828125f)) + 1.0f)) * 0.5f)
                 + instanced_data.Load<float4>(base + 0u).xyz;

    
    float3 wpos = world;
    wpos.y = world.y + perlin(world.xz * 0.5f) * 0.3f;
    float2 n3 = spire_noise3(wpos * 0.4f);
    float3 disp = wpos + (float3(n3.x, 0.0f, n3.y) * 0.06f);

    o.position = mul(view_constants[current_view_index].world_to_sample, float4(disp, 1.0f));
    o.C0 = float3(0.2067500054836273193359375f, 0.1769330203533172607421875f, 0.10665000975131988525390625f);
    o.C1 = vin.Vertex_Normal;
    o.C2 = disp;
    o.C3 = float(is_profile);
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float C3 : TEXCOORD3;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float3 face_normal = normalize(cross(ddx(pin.C2), -ddy(pin.C2)));
    float3 normal = normalize(lerp(pin.C1, float3(pin.C1.x, face_normal.y, pin.C1.z), pin.C3));
    float3 albedo = sqrt(clamp(pin.C0 * 0.5f, 0.0f, 1.0f));

    uint normal_word = uint(clamp((normal.x * 0.5f) + 0.5f, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp((normal.y * 0.5f) + 0.5f, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp((normal.z * 0.5f) + 0.5f, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 3003121664u;
    uint albedo_word = uint(clamp(albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 117440512u;
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

struct RoofSpire
{
    float4 position;
    float roof_animation_t;
    float roof_profile;
    float radius;
    int pad1;
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
};

ByteAddressBuffer instanced_data : register(t15, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float in_var_roof_profile_mult;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float out_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float in_var_roof_profile_mult: roof_profile_mult;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float3 _131 = in_var_Vertex_Position;
    bool _141 = in_var_roof_profile_mult > (-0.001000000047497451305389404296875f);
    float3 _159;
    if (_141)
    {
        float _152 = max(0.60000002384185791015625f, ((instanced_data.Load<float>(uint(gl_InstanceIndex) * 32 + 24) * 2.0f) * lerp(0.60000002384185791015625f, 1.0f, in_var_roof_profile_mult)) * lerp(0.4000000059604644775390625f, 1.5f, 1.0f - instanced_data.Load<float>(uint(gl_InstanceIndex) * 32 + 20)));
        float3 _155 = _131;
        _155.x = _131.x * _152;
        _155.z = _131.z * _152;
        _159 = _155;
    }
    else
    {
        _159 = _131;
    }
    float3 _172 = ((_159 * ((pow(2.0f, (-10.0f) * instanced_data.Load<float>(uint(gl_InstanceIndex) * 32 + 16)) * sin(((instanced_data.Load<float>(uint(gl_InstanceIndex) * 32 + 16) * 10.0f) - 0.75f) * 2.094395160675048828125f)) + 1.0f)) * 0.5f) + instanced_data.Load<float4>(uint(gl_InstanceIndex) * 32 + 0).xyz;
    float2 _174 = _172.xz * 0.5f;
    int2 _176 = int2(floor(_174));
    float2 _177 = frac(_174);
    uint2 _178 = uint2(_176);
    uint _180 = _178.y;
    uint _182 = _180 + (_180 << 10u);
    uint _184 = _182 ^ (_182 >> 6u);
    uint _186 = _184 + (_184 << 3u);
    uint _188 = _186 ^ (_186 >> 11u);
    uint _194 = (((_178.x * 1664525u) + (_188 + (_188 << 15u))) + 1013904223u) * 1664525u;
    uint _196 = _194 ^ (_194 >> 11u);
    uint _199 = _196 ^ ((_196 << 7u) & 2636928640u);
    uint _202 = _199 ^ ((_199 << 15u) & 4022730752u);
    uint2 _210 = uint2(_176 + int2(1, 0));
    uint _212 = _210.y;
    uint _214 = _212 + (_212 << 10u);
    uint _216 = _214 ^ (_214 >> 6u);
    uint _218 = _216 + (_216 << 3u);
    uint _220 = _218 ^ (_218 >> 11u);
    uint _226 = (((_210.x * 1664525u) + (_220 + (_220 << 15u))) + 1013904223u) * 1664525u;
    uint _228 = _226 ^ (_226 >> 11u);
    uint _231 = _228 ^ ((_228 << 7u) & 2636928640u);
    uint _234 = _231 ^ ((_231 << 15u) & 4022730752u);
    uint2 _242 = uint2(_176 + int2(0, 1));
    uint _244 = _242.y;
    uint _246 = _244 + (_244 << 10u);
    uint _248 = _246 ^ (_246 >> 6u);
    uint _250 = _248 + (_248 << 3u);
    uint _252 = _250 ^ (_250 >> 11u);
    uint _258 = (((_242.x * 1664525u) + (_252 + (_252 << 15u))) + 1013904223u) * 1664525u;
    uint _260 = _258 ^ (_258 >> 11u);
    uint _263 = _260 ^ ((_260 << 7u) & 2636928640u);
    uint _266 = _263 ^ ((_263 << 15u) & 4022730752u);
    uint2 _274 = uint2(_176 + int2(1, 1));
    uint _276 = _274.y;
    uint _278 = _276 + (_276 << 10u);
    uint _280 = _278 ^ (_278 >> 6u);
    uint _282 = _280 + (_280 << 3u);
    uint _284 = _282 ^ (_282 >> 11u);
    uint _290 = (((_274.x * 1664525u) + (_284 + (_284 << 15u))) + 1013904223u) * 1664525u;
    uint _292 = _290 ^ (_290 >> 11u);
    uint _295 = _292 ^ ((_292 << 7u) & 2636928640u);
    uint _298 = _295 ^ ((_295 << 15u) & 4022730752u);
    float2 _308 = (_177 * _177) * (3.0f.xx - (_177 * 2.0f));
    float _309 = _308.x;
    float _310 = 1.0f - _309;
    float _312 = _308.y;
    _172.y = _172.y + (((((1.0f - _312) * (((asfloat(((_202 ^ (_202 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _310) + ((asfloat(((_234 ^ (_234 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _309))) + (((asfloat(((_266 ^ (_266 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _310) * _312)) + (((asfloat(((_298 ^ (_298 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _309) * _312)) * 0.300000011920928955078125f);
    float3 _327 = _172 * 0.4000000059604644775390625f;
    int3 _329 = int3(floor(_327));
    float3 _330 = frac(_327);
    uint3 _331 = uint3(_329);
    uint _334 = _331.z;
    uint _336 = _334 + (_334 << 10u);
    uint _338 = _336 ^ (_336 >> 6u);
    uint _340 = _338 + (_338 << 3u);
    uint _342 = _340 ^ (_340 >> 11u);
    uint _348 = (((_331.y * 1664525u) + (_342 + (_342 << 15u))) + 1013904223u) * 1664525u;
    uint _350 = _348 ^ (_348 >> 11u);
    uint _353 = _350 ^ ((_350 << 7u) & 2636928640u);
    uint _356 = _353 ^ ((_353 << 15u) & 4022730752u);
    uint _362 = (((_331.x * 1664525u) + (_356 ^ (_356 >> 18u))) + 1013904223u) * 1664525u;
    uint _364 = _362 ^ (_362 >> 11u);
    uint _367 = _364 ^ ((_364 << 7u) & 2636928640u);
    uint _370 = _367 ^ ((_367 << 15u) & 4022730752u);
    uint _372 = _370 ^ (_370 >> 18u);
    uint _374 = _372 + (_372 << 10u);
    uint _376 = _374 ^ (_374 >> 6u);
    uint _378 = _376 + (_376 << 3u);
    uint _380 = _378 ^ (_378 >> 11u);
    uint3 _393 = uint3(_329 + int3(1, 0, 0));
    uint _396 = _393.z;
    uint _398 = _396 + (_396 << 10u);
    uint _400 = _398 ^ (_398 >> 6u);
    uint _402 = _400 + (_400 << 3u);
    uint _404 = _402 ^ (_402 >> 11u);
    uint _410 = (((_393.y * 1664525u) + (_404 + (_404 << 15u))) + 1013904223u) * 1664525u;
    uint _412 = _410 ^ (_410 >> 11u);
    uint _415 = _412 ^ ((_412 << 7u) & 2636928640u);
    uint _418 = _415 ^ ((_415 << 15u) & 4022730752u);
    uint _424 = (((_393.x * 1664525u) + (_418 ^ (_418 >> 18u))) + 1013904223u) * 1664525u;
    uint _426 = _424 ^ (_424 >> 11u);
    uint _429 = _426 ^ ((_426 << 7u) & 2636928640u);
    uint _432 = _429 ^ ((_429 << 15u) & 4022730752u);
    uint _434 = _432 ^ (_432 >> 18u);
    uint _436 = _434 + (_434 << 10u);
    uint _438 = _436 ^ (_436 >> 6u);
    uint _440 = _438 + (_438 << 3u);
    uint _442 = _440 ^ (_440 >> 11u);
    uint3 _455 = uint3(_329 + int3(0, 1, 0));
    uint _458 = _455.z;
    uint _460 = _458 + (_458 << 10u);
    uint _462 = _460 ^ (_460 >> 6u);
    uint _464 = _462 + (_462 << 3u);
    uint _466 = _464 ^ (_464 >> 11u);
    uint _472 = (((_455.y * 1664525u) + (_466 + (_466 << 15u))) + 1013904223u) * 1664525u;
    uint _474 = _472 ^ (_472 >> 11u);
    uint _477 = _474 ^ ((_474 << 7u) & 2636928640u);
    uint _480 = _477 ^ ((_477 << 15u) & 4022730752u);
    uint _486 = (((_455.x * 1664525u) + (_480 ^ (_480 >> 18u))) + 1013904223u) * 1664525u;
    uint _488 = _486 ^ (_486 >> 11u);
    uint _491 = _488 ^ ((_488 << 7u) & 2636928640u);
    uint _494 = _491 ^ ((_491 << 15u) & 4022730752u);
    uint _496 = _494 ^ (_494 >> 18u);
    uint _498 = _496 + (_496 << 10u);
    uint _500 = _498 ^ (_498 >> 6u);
    uint _502 = _500 + (_500 << 3u);
    uint _504 = _502 ^ (_502 >> 11u);
    uint3 _517 = uint3(_329 + int3(1, 1, 0));
    uint _520 = _517.z;
    uint _522 = _520 + (_520 << 10u);
    uint _524 = _522 ^ (_522 >> 6u);
    uint _526 = _524 + (_524 << 3u);
    uint _528 = _526 ^ (_526 >> 11u);
    uint _534 = (((_517.y * 1664525u) + (_528 + (_528 << 15u))) + 1013904223u) * 1664525u;
    uint _536 = _534 ^ (_534 >> 11u);
    uint _539 = _536 ^ ((_536 << 7u) & 2636928640u);
    uint _542 = _539 ^ ((_539 << 15u) & 4022730752u);
    uint _548 = (((_517.x * 1664525u) + (_542 ^ (_542 >> 18u))) + 1013904223u) * 1664525u;
    uint _550 = _548 ^ (_548 >> 11u);
    uint _553 = _550 ^ ((_550 << 7u) & 2636928640u);
    uint _556 = _553 ^ ((_553 << 15u) & 4022730752u);
    uint _558 = _556 ^ (_556 >> 18u);
    uint _560 = _558 + (_558 << 10u);
    uint _562 = _560 ^ (_560 >> 6u);
    uint _564 = _562 + (_562 << 3u);
    uint _566 = _564 ^ (_564 >> 11u);
    uint3 _579 = uint3(_329 + int3(0, 0, 1));
    uint _582 = _579.z;
    uint _584 = _582 + (_582 << 10u);
    uint _586 = _584 ^ (_584 >> 6u);
    uint _588 = _586 + (_586 << 3u);
    uint _590 = _588 ^ (_588 >> 11u);
    uint _596 = (((_579.y * 1664525u) + (_590 + (_590 << 15u))) + 1013904223u) * 1664525u;
    uint _598 = _596 ^ (_596 >> 11u);
    uint _601 = _598 ^ ((_598 << 7u) & 2636928640u);
    uint _604 = _601 ^ ((_601 << 15u) & 4022730752u);
    uint _610 = (((_579.x * 1664525u) + (_604 ^ (_604 >> 18u))) + 1013904223u) * 1664525u;
    uint _612 = _610 ^ (_610 >> 11u);
    uint _615 = _612 ^ ((_612 << 7u) & 2636928640u);
    uint _618 = _615 ^ ((_615 << 15u) & 4022730752u);
    uint _620 = _618 ^ (_618 >> 18u);
    uint _622 = _620 + (_620 << 10u);
    uint _624 = _622 ^ (_622 >> 6u);
    uint _626 = _624 + (_624 << 3u);
    uint _628 = _626 ^ (_626 >> 11u);
    uint3 _641 = uint3(_329 + int3(1, 0, 1));
    uint _644 = _641.z;
    uint _646 = _644 + (_644 << 10u);
    uint _648 = _646 ^ (_646 >> 6u);
    uint _650 = _648 + (_648 << 3u);
    uint _652 = _650 ^ (_650 >> 11u);
    uint _658 = (((_641.y * 1664525u) + (_652 + (_652 << 15u))) + 1013904223u) * 1664525u;
    uint _660 = _658 ^ (_658 >> 11u);
    uint _663 = _660 ^ ((_660 << 7u) & 2636928640u);
    uint _666 = _663 ^ ((_663 << 15u) & 4022730752u);
    uint _672 = (((_641.x * 1664525u) + (_666 ^ (_666 >> 18u))) + 1013904223u) * 1664525u;
    uint _674 = _672 ^ (_672 >> 11u);
    uint _677 = _674 ^ ((_674 << 7u) & 2636928640u);
    uint _680 = _677 ^ ((_677 << 15u) & 4022730752u);
    uint _682 = _680 ^ (_680 >> 18u);
    uint _684 = _682 + (_682 << 10u);
    uint _686 = _684 ^ (_684 >> 6u);
    uint _688 = _686 + (_686 << 3u);
    uint _690 = _688 ^ (_688 >> 11u);
    uint3 _703 = uint3(_329 + int3(0, 1, 1));
    uint _706 = _703.z;
    uint _708 = _706 + (_706 << 10u);
    uint _710 = _708 ^ (_708 >> 6u);
    uint _712 = _710 + (_710 << 3u);
    uint _714 = _712 ^ (_712 >> 11u);
    uint _720 = (((_703.y * 1664525u) + (_714 + (_714 << 15u))) + 1013904223u) * 1664525u;
    uint _722 = _720 ^ (_720 >> 11u);
    uint _725 = _722 ^ ((_722 << 7u) & 2636928640u);
    uint _728 = _725 ^ ((_725 << 15u) & 4022730752u);
    uint _734 = (((_703.x * 1664525u) + (_728 ^ (_728 >> 18u))) + 1013904223u) * 1664525u;
    uint _736 = _734 ^ (_734 >> 11u);
    uint _739 = _736 ^ ((_736 << 7u) & 2636928640u);
    uint _742 = _739 ^ ((_739 << 15u) & 4022730752u);
    uint _744 = _742 ^ (_742 >> 18u);
    uint _746 = _744 + (_744 << 10u);
    uint _748 = _746 ^ (_746 >> 6u);
    uint _750 = _748 + (_748 << 3u);
    uint _752 = _750 ^ (_750 >> 11u);
    uint3 _765 = uint3(_329 + int3(1, 1, 1));
    uint _768 = _765.z;
    uint _770 = _768 + (_768 << 10u);
    uint _772 = _770 ^ (_770 >> 6u);
    uint _774 = _772 + (_772 << 3u);
    uint _776 = _774 ^ (_774 >> 11u);
    uint _782 = (((_765.y * 1664525u) + (_776 + (_776 << 15u))) + 1013904223u) * 1664525u;
    uint _784 = _782 ^ (_782 >> 11u);
    uint _787 = _784 ^ ((_784 << 7u) & 2636928640u);
    uint _790 = _787 ^ ((_787 << 15u) & 4022730752u);
    uint _796 = (((_765.x * 1664525u) + (_790 ^ (_790 >> 18u))) + 1013904223u) * 1664525u;
    uint _798 = _796 ^ (_796 >> 11u);
    uint _801 = _798 ^ ((_798 << 7u) & 2636928640u);
    uint _804 = _801 ^ ((_801 << 15u) & 4022730752u);
    uint _806 = _804 ^ (_804 >> 18u);
    uint _808 = _806 + (_806 << 10u);
    uint _810 = _808 ^ (_808 >> 6u);
    uint _812 = _810 + (_810 << 3u);
    uint _814 = _812 ^ (_812 >> 11u);
    float3 _829 = (_330 * _330) * (3.0f.xxx - (_330 * 2.0f));
    float _830 = _829.x;
    float _831 = 1.0f - _830;
    float _833 = _829.y;
    float _834 = 1.0f - _833;
    float _836 = _829.z;
    float _837 = 1.0f - _836;
    float2 _854 = ((((((float2(asfloat((_372 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_380 + (_380 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _831) * _834) * _837) + (((float2(asfloat((_434 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_442 + (_442 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _830) * _834) * _837)) + (((float2(asfloat((_496 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_504 + (_504 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _831) * _833) * _837)) + (((float2(asfloat((_558 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_566 + (_566 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _830) * _833) * _837)) + (((float2(asfloat((_620 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_628 + (_628 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _831) * _834) * _836);
    float2 _869 = clamp(((((_854 + (((float2(asfloat((_682 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_690 + (_690 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _830) * _834) * _836)) + (((float2(asfloat((_744 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_752 + (_752 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _831) * _833) * _836)) + (((float2(asfloat((_806 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_814 + (_814 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _830) * _833) * _836)) - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 _874 = _172 + (float3(_869.x, 0.0f, _869.y) * 0.0599999986588954925537109375f);
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_874, 1.0f));
    out_var_C0 = float3(0.2067500054836273193359375f, 0.1769330203533172607421875f, 0.10665000975131988525390625f);
    out_var_C1 = in_var_Vertex_Normal;
    out_var_C2 = _874;
    out_var_C3 = float(_141);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_roof_profile_mult = stage_input.in_var_roof_profile_mult;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
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

struct RoofSpire
{
    float4 position;
    float roof_animation_t;
    float roof_profile;
    float radius;
    int pad1;
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
};

ByteAddressBuffer instanced_data : register(t15, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float in_var_roof_profile_mult;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float out_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float in_var_roof_profile_mult: roof_profile_mult;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float3 _131 = in_var_Vertex_Position;
    bool _141 = in_var_roof_profile_mult > (-0.001000000047497451305389404296875f);
    float3 _159;
    if (_141)
    {
        float _152 = max(0.60000002384185791015625f, ((instanced_data.Load<float>(uint(gl_InstanceIndex) * 32 + 24) * 2.0f) * lerp(0.60000002384185791015625f, 1.0f, in_var_roof_profile_mult)) * lerp(0.4000000059604644775390625f, 1.5f, 1.0f - instanced_data.Load<float>(uint(gl_InstanceIndex) * 32 + 20)));
        float3 _155 = _131;
        _155.x = _131.x * _152;
        _155.z = _131.z * _152;
        _159 = _155;
    }
    else
    {
        _159 = _131;
    }
    float3 _172 = ((_159 * ((pow(2.0f, (-10.0f) * instanced_data.Load<float>(uint(gl_InstanceIndex) * 32 + 16)) * sin(((instanced_data.Load<float>(uint(gl_InstanceIndex) * 32 + 16) * 10.0f) - 0.75f) * 2.094395160675048828125f)) + 1.0f)) * 0.5f) + instanced_data.Load<float4>(uint(gl_InstanceIndex) * 32 + 0).xyz;
    float2 _174 = _172.xz * 0.5f;
    int2 _176 = int2(floor(_174));
    float2 _177 = frac(_174);
    uint2 _178 = uint2(_176);
    uint _180 = _178.y;
    uint _182 = _180 + (_180 << 10u);
    uint _184 = _182 ^ (_182 >> 6u);
    uint _186 = _184 + (_184 << 3u);
    uint _188 = _186 ^ (_186 >> 11u);
    uint _194 = (((_178.x * 1664525u) + (_188 + (_188 << 15u))) + 1013904223u) * 1664525u;
    uint _196 = _194 ^ (_194 >> 11u);
    uint _199 = _196 ^ ((_196 << 7u) & 2636928640u);
    uint _202 = _199 ^ ((_199 << 15u) & 4022730752u);
    uint2 _210 = uint2(_176 + int2(1, 0));
    uint _212 = _210.y;
    uint _214 = _212 + (_212 << 10u);
    uint _216 = _214 ^ (_214 >> 6u);
    uint _218 = _216 + (_216 << 3u);
    uint _220 = _218 ^ (_218 >> 11u);
    uint _226 = (((_210.x * 1664525u) + (_220 + (_220 << 15u))) + 1013904223u) * 1664525u;
    uint _228 = _226 ^ (_226 >> 11u);
    uint _231 = _228 ^ ((_228 << 7u) & 2636928640u);
    uint _234 = _231 ^ ((_231 << 15u) & 4022730752u);
    uint2 _242 = uint2(_176 + int2(0, 1));
    uint _244 = _242.y;
    uint _246 = _244 + (_244 << 10u);
    uint _248 = _246 ^ (_246 >> 6u);
    uint _250 = _248 + (_248 << 3u);
    uint _252 = _250 ^ (_250 >> 11u);
    uint _258 = (((_242.x * 1664525u) + (_252 + (_252 << 15u))) + 1013904223u) * 1664525u;
    uint _260 = _258 ^ (_258 >> 11u);
    uint _263 = _260 ^ ((_260 << 7u) & 2636928640u);
    uint _266 = _263 ^ ((_263 << 15u) & 4022730752u);
    uint2 _274 = uint2(_176 + int2(1, 1));
    uint _276 = _274.y;
    uint _278 = _276 + (_276 << 10u);
    uint _280 = _278 ^ (_278 >> 6u);
    uint _282 = _280 + (_280 << 3u);
    uint _284 = _282 ^ (_282 >> 11u);
    uint _290 = (((_274.x * 1664525u) + (_284 + (_284 << 15u))) + 1013904223u) * 1664525u;
    uint _292 = _290 ^ (_290 >> 11u);
    uint _295 = _292 ^ ((_292 << 7u) & 2636928640u);
    uint _298 = _295 ^ ((_295 << 15u) & 4022730752u);
    float2 _308 = (_177 * _177) * (3.0f.xx - (_177 * 2.0f));
    float _309 = _308.x;
    float _310 = 1.0f - _309;
    float _312 = _308.y;
    _172.y = _172.y + (((((1.0f - _312) * (((asfloat(((_202 ^ (_202 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _310) + ((asfloat(((_234 ^ (_234 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _309))) + (((asfloat(((_266 ^ (_266 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _310) * _312)) + (((asfloat(((_298 ^ (_298 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _309) * _312)) * 0.300000011920928955078125f);
    float3 _327 = _172 * 0.4000000059604644775390625f;
    int3 _329 = int3(floor(_327));
    float3 _330 = frac(_327);
    uint3 _331 = uint3(_329);
    uint _334 = _331.z;
    uint _336 = _334 + (_334 << 10u);
    uint _338 = _336 ^ (_336 >> 6u);
    uint _340 = _338 + (_338 << 3u);
    uint _342 = _340 ^ (_340 >> 11u);
    uint _348 = (((_331.y * 1664525u) + (_342 + (_342 << 15u))) + 1013904223u) * 1664525u;
    uint _350 = _348 ^ (_348 >> 11u);
    uint _353 = _350 ^ ((_350 << 7u) & 2636928640u);
    uint _356 = _353 ^ ((_353 << 15u) & 4022730752u);
    uint _362 = (((_331.x * 1664525u) + (_356 ^ (_356 >> 18u))) + 1013904223u) * 1664525u;
    uint _364 = _362 ^ (_362 >> 11u);
    uint _367 = _364 ^ ((_364 << 7u) & 2636928640u);
    uint _370 = _367 ^ ((_367 << 15u) & 4022730752u);
    uint _372 = _370 ^ (_370 >> 18u);
    uint _374 = _372 + (_372 << 10u);
    uint _376 = _374 ^ (_374 >> 6u);
    uint _378 = _376 + (_376 << 3u);
    uint _380 = _378 ^ (_378 >> 11u);
    uint3 _393 = uint3(_329 + int3(1, 0, 0));
    uint _396 = _393.z;
    uint _398 = _396 + (_396 << 10u);
    uint _400 = _398 ^ (_398 >> 6u);
    uint _402 = _400 + (_400 << 3u);
    uint _404 = _402 ^ (_402 >> 11u);
    uint _410 = (((_393.y * 1664525u) + (_404 + (_404 << 15u))) + 1013904223u) * 1664525u;
    uint _412 = _410 ^ (_410 >> 11u);
    uint _415 = _412 ^ ((_412 << 7u) & 2636928640u);
    uint _418 = _415 ^ ((_415 << 15u) & 4022730752u);
    uint _424 = (((_393.x * 1664525u) + (_418 ^ (_418 >> 18u))) + 1013904223u) * 1664525u;
    uint _426 = _424 ^ (_424 >> 11u);
    uint _429 = _426 ^ ((_426 << 7u) & 2636928640u);
    uint _432 = _429 ^ ((_429 << 15u) & 4022730752u);
    uint _434 = _432 ^ (_432 >> 18u);
    uint _436 = _434 + (_434 << 10u);
    uint _438 = _436 ^ (_436 >> 6u);
    uint _440 = _438 + (_438 << 3u);
    uint _442 = _440 ^ (_440 >> 11u);
    uint3 _455 = uint3(_329 + int3(0, 1, 0));
    uint _458 = _455.z;
    uint _460 = _458 + (_458 << 10u);
    uint _462 = _460 ^ (_460 >> 6u);
    uint _464 = _462 + (_462 << 3u);
    uint _466 = _464 ^ (_464 >> 11u);
    uint _472 = (((_455.y * 1664525u) + (_466 + (_466 << 15u))) + 1013904223u) * 1664525u;
    uint _474 = _472 ^ (_472 >> 11u);
    uint _477 = _474 ^ ((_474 << 7u) & 2636928640u);
    uint _480 = _477 ^ ((_477 << 15u) & 4022730752u);
    uint _486 = (((_455.x * 1664525u) + (_480 ^ (_480 >> 18u))) + 1013904223u) * 1664525u;
    uint _488 = _486 ^ (_486 >> 11u);
    uint _491 = _488 ^ ((_488 << 7u) & 2636928640u);
    uint _494 = _491 ^ ((_491 << 15u) & 4022730752u);
    uint _496 = _494 ^ (_494 >> 18u);
    uint _498 = _496 + (_496 << 10u);
    uint _500 = _498 ^ (_498 >> 6u);
    uint _502 = _500 + (_500 << 3u);
    uint _504 = _502 ^ (_502 >> 11u);
    uint3 _517 = uint3(_329 + int3(1, 1, 0));
    uint _520 = _517.z;
    uint _522 = _520 + (_520 << 10u);
    uint _524 = _522 ^ (_522 >> 6u);
    uint _526 = _524 + (_524 << 3u);
    uint _528 = _526 ^ (_526 >> 11u);
    uint _534 = (((_517.y * 1664525u) + (_528 + (_528 << 15u))) + 1013904223u) * 1664525u;
    uint _536 = _534 ^ (_534 >> 11u);
    uint _539 = _536 ^ ((_536 << 7u) & 2636928640u);
    uint _542 = _539 ^ ((_539 << 15u) & 4022730752u);
    uint _548 = (((_517.x * 1664525u) + (_542 ^ (_542 >> 18u))) + 1013904223u) * 1664525u;
    uint _550 = _548 ^ (_548 >> 11u);
    uint _553 = _550 ^ ((_550 << 7u) & 2636928640u);
    uint _556 = _553 ^ ((_553 << 15u) & 4022730752u);
    uint _558 = _556 ^ (_556 >> 18u);
    uint _560 = _558 + (_558 << 10u);
    uint _562 = _560 ^ (_560 >> 6u);
    uint _564 = _562 + (_562 << 3u);
    uint _566 = _564 ^ (_564 >> 11u);
    uint3 _579 = uint3(_329 + int3(0, 0, 1));
    uint _582 = _579.z;
    uint _584 = _582 + (_582 << 10u);
    uint _586 = _584 ^ (_584 >> 6u);
    uint _588 = _586 + (_586 << 3u);
    uint _590 = _588 ^ (_588 >> 11u);
    uint _596 = (((_579.y * 1664525u) + (_590 + (_590 << 15u))) + 1013904223u) * 1664525u;
    uint _598 = _596 ^ (_596 >> 11u);
    uint _601 = _598 ^ ((_598 << 7u) & 2636928640u);
    uint _604 = _601 ^ ((_601 << 15u) & 4022730752u);
    uint _610 = (((_579.x * 1664525u) + (_604 ^ (_604 >> 18u))) + 1013904223u) * 1664525u;
    uint _612 = _610 ^ (_610 >> 11u);
    uint _615 = _612 ^ ((_612 << 7u) & 2636928640u);
    uint _618 = _615 ^ ((_615 << 15u) & 4022730752u);
    uint _620 = _618 ^ (_618 >> 18u);
    uint _622 = _620 + (_620 << 10u);
    uint _624 = _622 ^ (_622 >> 6u);
    uint _626 = _624 + (_624 << 3u);
    uint _628 = _626 ^ (_626 >> 11u);
    uint3 _641 = uint3(_329 + int3(1, 0, 1));
    uint _644 = _641.z;
    uint _646 = _644 + (_644 << 10u);
    uint _648 = _646 ^ (_646 >> 6u);
    uint _650 = _648 + (_648 << 3u);
    uint _652 = _650 ^ (_650 >> 11u);
    uint _658 = (((_641.y * 1664525u) + (_652 + (_652 << 15u))) + 1013904223u) * 1664525u;
    uint _660 = _658 ^ (_658 >> 11u);
    uint _663 = _660 ^ ((_660 << 7u) & 2636928640u);
    uint _666 = _663 ^ ((_663 << 15u) & 4022730752u);
    uint _672 = (((_641.x * 1664525u) + (_666 ^ (_666 >> 18u))) + 1013904223u) * 1664525u;
    uint _674 = _672 ^ (_672 >> 11u);
    uint _677 = _674 ^ ((_674 << 7u) & 2636928640u);
    uint _680 = _677 ^ ((_677 << 15u) & 4022730752u);
    uint _682 = _680 ^ (_680 >> 18u);
    uint _684 = _682 + (_682 << 10u);
    uint _686 = _684 ^ (_684 >> 6u);
    uint _688 = _686 + (_686 << 3u);
    uint _690 = _688 ^ (_688 >> 11u);
    uint3 _703 = uint3(_329 + int3(0, 1, 1));
    uint _706 = _703.z;
    uint _708 = _706 + (_706 << 10u);
    uint _710 = _708 ^ (_708 >> 6u);
    uint _712 = _710 + (_710 << 3u);
    uint _714 = _712 ^ (_712 >> 11u);
    uint _720 = (((_703.y * 1664525u) + (_714 + (_714 << 15u))) + 1013904223u) * 1664525u;
    uint _722 = _720 ^ (_720 >> 11u);
    uint _725 = _722 ^ ((_722 << 7u) & 2636928640u);
    uint _728 = _725 ^ ((_725 << 15u) & 4022730752u);
    uint _734 = (((_703.x * 1664525u) + (_728 ^ (_728 >> 18u))) + 1013904223u) * 1664525u;
    uint _736 = _734 ^ (_734 >> 11u);
    uint _739 = _736 ^ ((_736 << 7u) & 2636928640u);
    uint _742 = _739 ^ ((_739 << 15u) & 4022730752u);
    uint _744 = _742 ^ (_742 >> 18u);
    uint _746 = _744 + (_744 << 10u);
    uint _748 = _746 ^ (_746 >> 6u);
    uint _750 = _748 + (_748 << 3u);
    uint _752 = _750 ^ (_750 >> 11u);
    uint3 _765 = uint3(_329 + int3(1, 1, 1));
    uint _768 = _765.z;
    uint _770 = _768 + (_768 << 10u);
    uint _772 = _770 ^ (_770 >> 6u);
    uint _774 = _772 + (_772 << 3u);
    uint _776 = _774 ^ (_774 >> 11u);
    uint _782 = (((_765.y * 1664525u) + (_776 + (_776 << 15u))) + 1013904223u) * 1664525u;
    uint _784 = _782 ^ (_782 >> 11u);
    uint _787 = _784 ^ ((_784 << 7u) & 2636928640u);
    uint _790 = _787 ^ ((_787 << 15u) & 4022730752u);
    uint _796 = (((_765.x * 1664525u) + (_790 ^ (_790 >> 18u))) + 1013904223u) * 1664525u;
    uint _798 = _796 ^ (_796 >> 11u);
    uint _801 = _798 ^ ((_798 << 7u) & 2636928640u);
    uint _804 = _801 ^ ((_801 << 15u) & 4022730752u);
    uint _806 = _804 ^ (_804 >> 18u);
    uint _808 = _806 + (_806 << 10u);
    uint _810 = _808 ^ (_808 >> 6u);
    uint _812 = _810 + (_810 << 3u);
    uint _814 = _812 ^ (_812 >> 11u);
    float3 _829 = (_330 * _330) * (3.0f.xxx - (_330 * 2.0f));
    float _830 = _829.x;
    float _831 = 1.0f - _830;
    float _833 = _829.y;
    float _834 = 1.0f - _833;
    float _836 = _829.z;
    float _837 = 1.0f - _836;
    float2 _854 = ((((((float2(asfloat((_372 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_380 + (_380 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _831) * _834) * _837) + (((float2(asfloat((_434 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_442 + (_442 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _830) * _834) * _837)) + (((float2(asfloat((_496 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_504 + (_504 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _831) * _833) * _837)) + (((float2(asfloat((_558 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_566 + (_566 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _830) * _833) * _837)) + (((float2(asfloat((_620 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_628 + (_628 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _831) * _834) * _836);
    float2 _869 = clamp(((((_854 + (((float2(asfloat((_682 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_690 + (_690 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _830) * _834) * _836)) + (((float2(asfloat((_744 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_752 + (_752 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _831) * _833) * _836)) + (((float2(asfloat((_806 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_814 + (_814 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _830) * _833) * _836)) - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 _874 = _172 + (float3(_869.x, 0.0f, _869.y) * 0.0599999986588954925537109375f);
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_874, 1.0f));
    out_var_C0 = float3(0.2067500054836273193359375f, 0.1769330203533172607421875f, 0.10665000975131988525390625f);
    out_var_C1 = in_var_Vertex_Normal;
    out_var_C2 = _874;
    out_var_C3 = float(_141);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_roof_profile_mult = stage_input.in_var_roof_profile_mult;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif
