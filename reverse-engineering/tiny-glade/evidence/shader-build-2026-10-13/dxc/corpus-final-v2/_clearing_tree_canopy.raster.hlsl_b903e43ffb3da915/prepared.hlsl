#define SHADER_VARIANT_DEPTH_ONLY 0
#line 1 "shaders/clearing/tree_canopy.raster.hlsl"












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
#line 13 "shaders/clearing/tree_canopy.raster.hlsl"

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
#line 14 "shaders/clearing/tree_canopy.raster.hlsl"

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
#line 15 "shaders/clearing/tree_canopy.raster.hlsl"

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
#line 16 "shaders/clearing/tree_canopy.raster.hlsl"

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
#line 17 "shaders/clearing/tree_canopy.raster.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 18 "shaders/clearing/tree_canopy.raster.hlsl"

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
#line 19 "shaders/clearing/tree_canopy.raster.hlsl"



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





cbuffer color_library : register(b14, space0) {
    ColorLibrary color_library;
};

#endif 
#line 23 "shaders/clearing/tree_canopy.raster.hlsl"

cbuffer color_library : register(b14, space0)
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
#line 64 "shaders/clearing/tree_canopy.raster.hlsl"

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
#line 65 "shaders/clearing/tree_canopy.raster.hlsl"

#line 1 "shaders/clearing/canopy_vert.inc.hlsl"







#ifndef TINYGLADE_CLEARING_CANOPY_VERT_INC_HLSL
#define TINYGLADE_CLEARING_CANOPY_VERT_INC_HLSL



float3 canopy_vertex(float3 local_pos, float growth, float2 bend, float sway)
{
    return local_pos * growth + float3(bend.x * sway, 0.0f, bend.y * sway);
}

#endif 
#line 66 "shaders/clearing/tree_canopy.raster.hlsl"

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
#line 67 "shaders/clearing/tree_canopy.raster.hlsl"

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
#line 68 "shaders/clearing/tree_canopy.raster.hlsl"

#line 1 "shaders/clearing/inc/instanced_placement.hlsl"











#ifndef TINYGLADE_CLEARING_INC_INSTANCED_PLACEMENT_HLSL
#define TINYGLADE_CLEARING_INC_INSTANCED_PLACEMENT_HLSL


struct PlacementData
{
    float3 position;
    float scale;
    float rotation_angle;
    int id;
    uint custom0;
    uint custom1;
    uint anim_t_packed;
    float2 bend_vector;
    int pad0;
};




float4 placement_world(ByteAddressBuffer placement, uint index, float3 vertex_pos)
{
    float3 pos = placement.Load<float3>(index * 48u + 0);
    float scale = placement.Load<float>(index * 48u + 12);
    float rot = radians(placement.Load<float>(index * 48u + 16));
    float3 fwd = normalize(float3(cos(rot), 0.0f, sin(rot)));
    float4x4 basis = transpose(float4x4(float4(fwd, 0.0f),
                                        float4(0.0f, 1.0f, 0.0f, 0.0f),
                                        float4(normalize(cross(fwd, float3(0.0f, 1.0f, 0.0f))), 0.0f),
                                        float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float4x4 scale_m = float4x4(float4(scale, 0.0f, 0.0f, 0.0f),
                                float4(0.0f, scale, 0.0f, 0.0f),
                                float4(0.0f, 0.0f, scale, 0.0f),
                                float4(0.0f, 0.0f, 0.0f, 1.0f));
    float4x4 translate_m = float4x4(float4(1.0f, 0.0f, 0.0f, pos.x),
                                    float4(0.0f, 1.0f, 0.0f, pos.y),
                                    float4(0.0f, 0.0f, 1.0f, pos.z),
                                    float4(0.0f, 0.0f, 0.0f, 1.0f));
    return mul(translate_m, mul(basis, mul(scale_m, float4(vertex_pos, 1.0f))));
}

#endif 
#line 69 "shaders/clearing/tree_canopy.raster.hlsl"

#line 1 "shaders/clearing/inc/canopy_common.inc.hlsl"








#ifndef TINYGLADE_CLEARING_INC_CANOPY_COMMON_HLSL
#define TINYGLADE_CLEARING_INC_CANOPY_COMMON_HLSL


float canopy_age(ByteAddressBuffer placement, uint index)
{
    return f16tof32(placement.Load<uint>(index * 48u + 32) & 0xFFFFu);
}


float2 canopy_bend(ByteAddressBuffer placement, uint index)
{
    return placement.Load<float2>(index * 48u + 36);
}


float4 canopy_place(ByteAddressBuffer placement, uint index, float3 local_pos)
{
    float3 pos = placement.Load<float3>(index * 48u + 0);
    float scale = placement.Load<float>(index * 48u + 12);
    float rot = radians(placement.Load<float>(index * 48u + 16));
    float3 fwd = normalize(float3(cos(rot), 0.0f, sin(rot)));
    float4x4 basis = transpose(float4x4(float4(fwd, 0.0f),
                                        float4(0.0f, 1.0f, 0.0f, 0.0f),
                                        float4(normalize(cross(fwd, float3(0.0f, 1.0f, 0.0f))), 0.0f),
                                        float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float4x4 scale_m = float4x4(float4(scale, 0.0f, 0.0f, 0.0f),
                                float4(0.0f, scale, 0.0f, 0.0f),
                                float4(0.0f, 0.0f, scale, 0.0f),
                                float4(0.0f, 0.0f, 0.0f, 1.0f));
    float4x4 translate_m = float4x4(float4(1.0f, 0.0f, 0.0f, pos.x),
                                    float4(0.0f, 1.0f, 0.0f, pos.y),
                                    float4(0.0f, 0.0f, 1.0f, pos.z),
                                    float4(0.0f, 0.0f, 0.0f, 1.0f));
    return mul(translate_m, mul(basis, mul(scale_m, float4(local_pos, 1.0f))));
}

#endif 
#line 70 "shaders/clearing/tree_canopy.raster.hlsl"

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
#line 71 "shaders/clearing/tree_canopy.raster.hlsl"



#line 1 "shaders/inc/material/canopy_material.inc.hlsl"


#ifndef TINYGLADE_INC_MATERIAL_CANOPY_MATERIAL_HLSL
#define TINYGLADE_INC_MATERIAL_CANOPY_MATERIAL_HLSL

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
#line 6 "shaders/inc/material/canopy_material.inc.hlsl"




struct CanopyMaterialParams {
    float3 leaf_color;
    float3 flower_color;
    float roughness;
    float alpha;
};

CanopyMaterialParams init_canopy_material(float3 leaf_color, float3 flower_color, float roughness, float alpha)
{
    CanopyMaterialParams p;
    p.leaf_color = leaf_color;
    p.flower_color = flower_color;
    p.roughness = saturate(roughness);
    p.alpha = saturate(alpha);
    return p;
}

#endif 
#line 74 "shaders/clearing/tree_canopy.raster.hlsl"

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
#line 75 "shaders/clearing/tree_canopy.raster.hlsl"

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
#line 76 "shaders/clearing/tree_canopy.raster.hlsl"


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





cbuffer color_library : register(b14, space0) {
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
#line 78 "shaders/clearing/tree_canopy.raster.hlsl"

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
#line 79 "shaders/clearing/tree_canopy.raster.hlsl"

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
#line 80 "shaders/clearing/tree_canopy.raster.hlsl"

#line 1 "shaders/inc/sh.hlsl"










float4 sh_basis(float3 dir)
{
    return float4(0.2820949852466583251953125f, 0.48860299587249755859375f * dir);
}

float sh_eval(float4 sh, float3 dir)
{
    return dot(sh, sh_basis(dir));
}
#line 81 "shaders/clearing/tree_canopy.raster.hlsl"



int is_firstperson_mode;
float wind_strength;
uint2 blue_noise_texture_dims;
float cone_angle_mult;
float max_cone_cull_dist_range;

ByteAddressBuffer nani_instance_data : register(t15, space0);
ByteAddressBuffer instance_indirection_buffer : register(t16, space0);
Texture2D<float4> canopy_alpha_texture : register(t17, space0);
Texture2D<float4> canopy_flowers_color : register(t18, space0);
Texture2D<float4> canopy_flowers_depth : register(t19, space0);
Texture2D<float4> canopy_color : register(t20, space0);
Texture2D<float4> blue_noise_texture : register(t21, space0);

struct VS_IN
{
    float3 vertex_position : TEXCOORD0;
    float3 vertex_color : TEXCOORD1;
    float3 vertex_normal : TEXCOORD2;
    float3 prim_center : TEXCOORD3;
    float3 prim_normals : TEXCOORD4;
    float3 appear_pos : TEXCOORD5;
    float age : TEXCOORD6;
};

struct VS_OUT
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    nointerpolation int C4 : TEXCOORD4;
    nointerpolation int C5 : TEXCOORD5;
    nointerpolation float C6 : TEXCOORD6;
    float3 C7 : TEXCOORD7;
    nointerpolation uint C8 : TEXCOORD8;
    nointerpolation uint C9 : TEXCOORD9;
    nointerpolation float C10 : TEXCOORD10;
    float2 C11 : TEXCOORD11;
    float C12 : TEXCOORD12;
    float4 position : SV_Position;
};

VS_OUT vs_main(VS_IN i, uint instance_id : SV_InstanceID)
{
    VS_OUT o;
    float4 wp;
    float3 canopy_pos;
    float3 leaf_color;
    float3 leaf_normal;
    float3 world_pos;
    int tree_id;
    int leaf_seed;
    float anim_high;
    float3 prim_center;
    bool flower_bool;
    uint part_id;
    float leaf_noise;
    float2 clip_zw;
    float depth_fade;
    do
    {
        uint indir = instance_indirection_buffer.Load<uint>(instance_id * 4u + 0);
        float anim_low = clamp(f16tof32(nani_instance_data.Load<uint>(indir * 48u + 32) & 0xFFFFu), 0.0f, 1.0f);
        float growth = clamp(1.0f - clamp(lerp(-1.0f, 1.0f, anim_low) + (1.0f - i.age), 0.0f, 1.0f), 0.0f, 1.0f);
        float grow_f = 1.0f - ((growth * growth) - (growth * sin(growth * 3.14f)));
        float3 pos = lerp(i.appear_pos, i.vertex_position, grow_f);
        if (grow_f < 0.01f)
        {
            wp = asfloat(0x7fbfffffu);
            canopy_pos = 0.0f;
            leaf_color = 0.0f;
            leaf_normal = 0.0f;
            world_pos = 0.0f;
            tree_id = 0;
            leaf_seed = 0;
            anim_high = 0.0f;
            prim_center = 0.0f;
            flower_bool = false;
            part_id = 0u;
            leaf_noise = 0.0f;
            clip_zw = 0.0f;
            depth_fade = 0.0f;
            break;
        }

        
        float wind_amp = pos.y * 0.03f;
        float t_sec = float(game_time_sec);
        float t_frac = game_time_frac;
        float x_amp = 0.2f * pos.z;
        float sway_x = pos.x + ((wind_amp * sin(fmod(t_sec * 0.823529f, 6.2831855f) + (t_frac * 0.823529f) + (0.2f * pos.x)))
            * sin(fmod(t_sec * 0.893617f, 6.2831855f) + (t_frac * 0.893617f) + x_amp));
        float sway_z = pos.z + ((wind_amp * sin(fmod(t_sec * 0.792453f, 6.2831855f) + (t_frac * 0.792453f) + (0.2f * sway_x)))
            * sin(fmod(t_sec * 0.688525f, 6.2831855f) + (t_frac * 0.688525f) + x_amp));

        uint idx = uint(int(indir));
        float rot = radians(nani_instance_data.Load<float>(idx * 48u + 16));
        float3 fwd = normalize(float3(cos(rot), 0.0f, sin(rot)));
        float4x4 basis = transpose(float4x4(float4(fwd, 0.0f),
                                            float4(0.0f, 1.0f, 0.0f, 0.0f),
                                            float4(normalize(cross(fwd, float3(0.0f, 1.0f, 0.0f))), 0.0f),
                                            float4(0.0f, 0.0f, 0.0f, 1.0f)));
        float px = nani_instance_data.Load<float3>(idx * 48u + 0).x;
        float py = nani_instance_data.Load<float3>(idx * 48u + 0).y;
        float pz = nani_instance_data.Load<float3>(idx * 48u + 0).z;
        float scale = nani_instance_data.Load<float>(idx * 48u + 12);
        float4 prim_world = mul(float4x4(float4(1.0f, 0.0f, 0.0f, px),
                                         float4(0.0f, 1.0f, 0.0f, py),
                                         float4(0.0f, 0.0f, 1.0f, pz),
                                         float4(0.0f, 0.0f, 0.0f, 1.0f)),
                                mul(basis, mul(float4x4(float4(scale, 0.0f, 0.0f, 0.0f),
                                                       float4(0.0f, scale, 0.0f, 0.0f),
                                                       float4(0.0f, 0.0f, scale, 0.0f),
                                                       float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(i.prim_center, 1.0f))));
        float3 prim_center_world = prim_world.xyz;
        float4 prim_view = mul(view_constants[current_view_index].world_to_view, float4(prim_world.x, prim_world.y, prim_world.z, 1.0f));
        if (length(prim_view.xyz) < (((render_overrides & 2u) != 0u) ? 5.0f : -5.0f))
        {
            wp = asfloat(0x7fbfffffu);
            canopy_pos = 0.0f;
            leaf_color = 0.0f;
            leaf_normal = 0.0f;
            world_pos = 0.0f;
            tree_id = 0;
            leaf_seed = 0;
            anim_high = 0.0f;
            prim_center = 0.0f;
            flower_bool = false;
            part_id = 0u;
            leaf_noise = 0.0f;
            clip_zw = 0.0f;
            depth_fade = 0.0f;
            break;
        }

        float4 cam4 = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
        float3 cam = cam4.xyz / cam4.w;
        float3 to_cam = prim_center_world - cam;
        float dist = length(to_cam);
        float facing = dot(mul(basis, float4(i.prim_normals, 0.0f)).xyz, to_cam * (1.0f / dist));
        float facing_falloff = facing - (abs(facing) * clamp((dist - 5.0f) * (-0.2f), 0.0f, 1.0f));
        if (facing_falloff >= 0.3f)
        {
            wp = asfloat(0x7fbfffffu);
            canopy_pos = 0.0f;
            leaf_color = 0.0f;
            leaf_normal = 0.0f;
            world_pos = 0.0f;
            tree_id = 0;
            leaf_seed = 0;
            anim_high = 0.0f;
            prim_center = 0.0f;
            flower_bool = false;
            part_id = 0u;
            leaf_noise = 0.0f;
            clip_zw = 0.0f;
            depth_fade = 0.0f;
            break;
        }
        float facing_blend = 1.0f - smoothstep(0.3f, -0.2f, facing_falloff);

        
        float3 leaf_world = mul(float4x4(float4(1.0f, 0.0f, 0.0f, px),
                                         float4(0.0f, 1.0f, 0.0f, py),
                                         float4(0.0f, 0.0f, 1.0f, pz),
                                         float4(0.0f, 0.0f, 0.0f, 1.0f)),
                                mul(basis, mul(float4x4(float4(scale, 0.0f, 0.0f, 0.0f),
                                                       float4(0.0f, scale, 0.0f, 0.0f),
                                                       float4(0.0f, 0.0f, scale, 0.0f),
                                                       float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(sway_x, pos.y, sway_z, 1.0f)))).xyz;

        float wind = (glade_theme_id == 4u) ? 0.4f : wind_strength;
        float3 leaf_dir = (float3(i.vertex_color.xy, 0.0f) * 2.0f) - 1.0f;
        float freq1 = 3.529416f * wind;
        float freq2 = 1.538462f * wind;
        float leaf_wind = (0.2f * sin(fmod(t_sec * freq1, 6.2831855f) + (t_frac * freq1) + sway_x)) * sin(fmod(t_sec * freq2, 6.2831855f) + (t_frac * freq2) + sway_z);
        float cosw = cos(leaf_wind);
        float sinw = sin(leaf_wind);
        float rand_angle = lerp(-0.5f, 0.5f, 0.3f * frac(52.982918f * frac(dot(float2(0.0671106f, 0.00583715f), prim_world.xz))));
        float3 view_axis = normalize(cam - prim_center_world);
        float sina = sin(rand_angle);
        float cosa = cos(rand_angle);
        float oca = 1.0f - cosa;
        float ax = view_axis.x;
        float ay = view_axis.y;
        float az = view_axis.z;
        float4x4 spin = transpose(float4x4(float4((oca * ax * ax) + cosa, (oca * ax * ay) - (az * sina), (oca * az * ax) + (ay * sina), 0.0f),
                                           float4((oca * ax * ay) + (az * sina), (oca * ay * ay) + cosa, (oca * ay * az) - (ax * sina), 0.0f),
                                           float4((oca * az * ax) - (ay * sina), (oca * ay * az) + (ax * sina), (oca * az * az) + cosa, 0.0f),
                                           float4(0.0f, 0.0f, 0.0f, 1.0f)));
        float grow_scaled = asfloat(nani_instance_data.Load<uint>(indir * 48u + 24));
        float3 billboard_pos = leaf_world
            + (mul(view_constants[current_view_index].view_to_world, float4(mul(float2x2(float2(cosw, sinw), float2(-sinw, cosw)), leaf_dir.xy), leaf_dir.z, 0.0f)).xyz * (grow_scaled * grow_f));
        float3 leaf_out = (mul(spin, float4(billboard_pos - prim_center_world, 1.0f)).xyz * ((1.0f - (facing_blend * facing_blend)) * lerp(0.8f, 1.0f, clamp(prim_view.z * (-0.05f), 0.0f, 1.0f)))) + prim_center_world;
        float2 bend = nani_instance_data.Load<float2>(indir * 48u + 36);
        float2 bent = leaf_out.xz + (bend * smoothstep(3.0f, 20.0f, i.vertex_position.y));
        float4 clip = mul(view_constants[current_view_index].world_to_sample, float4(bent.x, leaf_out.y, bent.y, 1.0f));
        float dist_plane = -view_constants[current_view_index].clip_to_view[3].z;
        float view_dist = (-1.0f) / ((clip.z / clip.w) * dist_plane);
        float depth_fade = smoothstep(1.6f, 3.2f, view_dist);
        float4 clip_final = clip;
        clip_final.z = (1.0f / ((-(view_dist - (1.6f * depth_fade))) * dist_plane)) * clip.w;

        int tree_id = int(uint(nani_instance_data.Load<int>(idx * 48u + 20)));
        int leaf_seed = int(10.0f * ((prim_world.x + prim_world.y) + prim_world.z));
        uint h1 = hash(uint(tree_id + 4));
        uint h2 = hash(uint(leaf_seed));
        float2 leaf_cell = nani_instance_data.Load<float3>(indir * 48u + 0).xz * 0.05f;
        int2 leaf_cell_int = int2(floor(leaf_cell));
        float2 leaf_cell_frac = frac(leaf_cell);
        float2 leaf_smooth = leaf_cell_frac * leaf_cell_frac * (3.0f - (2.0f * leaf_cell_frac));
        float leaf_noise = ((1.0f - leaf_smooth.y) * ((incb_perlin_hash01(hash(uint2(leaf_cell_int + int2(0, 0)))) * (1.0f - leaf_smooth.x)) + (incb_perlin_hash01(hash(uint2(leaf_cell_int + int2(1, 0)))) * leaf_smooth.x)))
            + ((incb_perlin_hash01(hash(uint2(leaf_cell_int + int2(0, 1)))) * (1.0f - leaf_smooth.x)) * leaf_smooth.y)
            + ((incb_perlin_hash01(hash(uint2(leaf_cell_int + int2(1, 1)))) * leaf_smooth.x) * leaf_smooth.y);

        wp = clip_final;
        canopy_pos = leaf_world;
        leaf_color = i.vertex_color;
        leaf_normal = mul(basis, float4(i.vertex_normal, 0.0f)).xyz;
        world_pos = float3(bent.x, leaf_out.y, bent.y);
        tree_id = tree_id;
        leaf_seed = leaf_seed;
        anim_high = f16tof32((nani_instance_data.Load<uint>(indir * 48u + 32) >> 16u) & 0xFFFFu);
        prim_center = prim_center_world;
        flower_bool = ((int(rand01(h1) < 0.4f)) & int(prim_world.y > 3.0f)) & int(rand01(h2) < 0.9f);
        part_id = uint(nani_instance_data.Load<int>(indir * 48u + 44));
        leaf_noise = leaf_noise;
        clip_zw = clip_final.zw;
        depth_fade = depth_fade;
        break;
    } while (false);
    o.position = wp;
    o.C0 = canopy_pos;
    o.C1 = leaf_color;
    o.C2 = leaf_normal;
    o.C3 = world_pos;
    o.C4 = tree_id;
    o.C5 = leaf_seed;
    o.C6 = anim_high;
    o.C7 = prim_center;
    o.C8 = uint(flower_bool);
    o.C9 = part_id;
    o.C10 = leaf_noise;
    o.C11 = clip_zw;
    o.C12 = depth_fade;
    return o;
}

struct PS_IN
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    nointerpolation int C4 : TEXCOORD4;
    nointerpolation int C5 : TEXCOORD5;
    nointerpolation float C6 : TEXCOORD6;
    float3 C7 : TEXCOORD7;
    nointerpolation uint C8 : TEXCOORD8;
    nointerpolation uint C9 : TEXCOORD9;
    nointerpolation float C10 : TEXCOORD10;
    float C12 : TEXCOORD12;
};

struct PS_OUT
{
    uint2 target : SV_Target0;
    float depth : SV_DepthLessEqual;
};

PS_OUT ps_main(PS_IN i, float4 frag_coord : SV_Position)
{
    PS_OUT o;
    bool has_flower = (i.C8 != 0u);
    float3 flower_col;
    float flower_depth;
    float flower_alpha;
    if (has_flower)
    {
        float4 depth_tex = canopy_flowers_depth.SampleBias(sampler_llc, i.C1.xy, global_mip_bias);
        float fd = depth_tex.x;
        flower_col = canopy_flowers_color.SampleBias(sampler_llc, i.C1.xy, global_mip_bias).xyz * lerp(0.5f, 2.0f, fd);
        flower_depth = fd;
        flower_alpha = float(fd > 0.01f);
    }
    else
    {
        flower_col = 0.0f;
        flower_depth = 0.0f;
        flower_alpha = 0.0f;
    }
    float4 alpha_tex = canopy_alpha_texture.SampleBias(sampler_llc, i.C1.xy, global_mip_bias);
    float wind_cone = ((render_overrides & 2u) != 0u) ? i.C6 : 0.0f;
    float4 view4 = mul(view_constants[current_view_index].world_to_view, float4(i.C7, 1.0f));
    float2 cone_range = float2(6.66f, max_cone_cull_dist_range) * view_constants[0].view_to_clip[1].y;
    if ((wind_cone > 0.0f) && (is_firstperson_mode == 0))
    {
        if (-view4.z < cone_range.y)
        {
            float3 p = float3(view4.xy + (i.C1.xy - 0.5f), view4.z);
            p.z = min(view4.z, -10.0f);
            float cone = cone_angle_mult * smoothstep(cone_range.y, cone_range.x, -view4.z);
            if ((-normalize(p * float3(0.75f, 1.0f, 1.0f)).z) > cos((cone * wind_cone) + ((min(1.0f, cone * 16.0f) * 0.075f) * (blue_noise_texture.SampleLevel(sampler_llr, (i.C1.xy * 2.0f) * 4.0f * 0.00390625f, 0.0f).x - 0.5f))))
            {
                discard;
            }
        }
    }

    float alpha_threshold = has_flower ? (alpha_tex.y - 0.3f) : alpha_tex.y;
    bool flower_visible = has_flower && (flower_alpha > 0.0f);
    bool flower_blend = flower_visible && (i.C10 < 0.6f);
    float blend_depth = alpha_threshold;
    bool use_flower;
    if (flower_blend)
    {
        float flower_t = lerp(0.0f, 1.5f, rand01(hash(uint(i.C5))) * flower_depth);
        use_flower = flower_t > alpha_threshold;
        blend_depth = max(alpha_threshold, flower_t);
    }
    else
    {
        use_flower = false;
    }
    if ((use_flower ? 1.0f : alpha_tex.x) < 0.2f)
    {
        discard;
    }

    int detail_kind;
    bool theme24 = (glade_theme_id == 2u) || (glade_theme_id == 4u);
    if (theme24)
    {
        detail_kind = 1;
    }
    else
    {
        if (glade_theme_id == 1u)
        {
            bool part_test;
            if (i.C9 != 1u)
            {
                part_test = (i.C9 == 2u) || (i.C10 > 0.4f);
            }
            else
            {
                part_test = false;
            }
            detail_kind = part_test ? 2 : 0;
        }
        else
        {
            detail_kind = 0;
        }
    }

    float3 detail_color;
    if (detail_kind == 1)
    {
        uint h1 = hash(uint(i.C4 ^ 1128782));
        uint h2 = hash(uint(i.C4 ^ 872));
        uint h3 = hash(uint(i.C4 ^ 11282));
        uint h4 = hash(uint(i.C4 ^ 82));
        uint h5 = hash(uint(i.C5));
        float2 hash2 = float2(rand01(h1), rand01(h2));
        float2 hash3 = float2(rand01(h3), rand01(h4));
        float2 sum = i.C3.yz + i.C3.yx;
        float3 c1 = canopy_color.SampleBias(sampler_llr, hash2 + (i.C1.xy * 0.01f) + (sum * 0.0005f), global_mip_bias).xyz;
        float3 c2 = canopy_color.SampleBias(sampler_llr, hash3 + (i.C1.xy * 0.2f) + (sum * 0.05f), global_mip_bias).xyz * 0.5f;
        detail_color = lerp(c1, c2, lerp(0.1f, 0.5f, rand01(h5)));
    }
    else if (detail_kind == 2)
    {
        bool leaf_part = (i.C9 == 2u);
        uint h1 = hash(uint(i.C4 ^ 1128782));
        uint h2 = hash(uint(i.C4 ^ 872));
        float2 hash2 = float2(rand01(h1), rand01(h2));
        float2 sum = i.C1.xy * 0.2f;
        float2 detail_uv1 = hash2 + sum + (float2(i.C3.yz + i.C3.yx) * 0.005f);
        float2 uv1 = leaf_part ? float2(detail_uv1.x, frac(detail_uv1.y) * 0.6f) : detail_uv1;
        float3 c1 = canopy_color.SampleBias(sampler_llr, uv1, global_mip_bias).xyz;
        uint h3 = hash(uint(i.C4 ^ 11282));
        uint h4 = hash(uint(i.C4 ^ 82));
        float2 hash3 = float2(rand01(h3), rand01(h4));
        float2 detail_uv2 = hash3 + sum + (float2(i.C3.yz + i.C3.yx) * 0.05f);
        float2 uv2 = leaf_part ? float2(detail_uv2.x, frac(uv1.y) * 0.6f) : detail_uv2;
        uint h5 = hash(uint(i.C5 ^ 1128782));
        detail_color = lerp(c1, canopy_color.SampleBias(sampler_llr, uv2, global_mip_bias).xyz, smoothstep(0.0f, 1.0f, rand01(h5)));
    }
    else
    {
        uint h = hash(uint(i.C4 * 1128782));
        float3 c2 = (glade_theme_id == 3u) ? (canopy_color_2.xyz * 0.75f) : canopy_color_2.xyz;
        detail_color = lerp(canopy_color_1.xyz, c2, rand01(h));
    }

    float3 final_albedo = use_flower ? flower_col : detail_color;
    if (glade_theme_id == 4u)
    {
        final_albedo = lerp(final_albedo, 0.7f, smoothstep(0.0f, 1.0f, clamp(i.C2.y, 0.0f, 1.0f)));
    }
    float3 packed_n = (i.C2 * 0.5f) + 0.5f;
    float3 color = sqrt(clamp(final_albedo, 0.0f, 1.0f));
    uint normal_packed = (uint(clamp(packed_n.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(packed_n.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(packed_n.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u))
        + (uint(clamp(clamp(alpha_tex.z - 0.00390625f, 0.0f, 1.0f), 0.0f, 1.0f) * 255.0f + 0.5f) << 24u);
    uint albedo_packed = (uint(clamp(color.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(color.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(color.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u))
        + (uint(clamp(blend_depth, 0.0f, 1.0f) * 255.0f + 0.5f) << 24u);
    o.target = uint2(normal_packed, albedo_packed);
    o.depth = (frag_coord.z / frag_coord.w) / ((1.0f / frag_coord.w) + ((i.C12 * (1.0f - blend_depth)) * 1.6f));
    return o;
}
