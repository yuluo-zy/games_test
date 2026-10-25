#define SHADER_VARIANT_DEPTH_ONLY 1
#define TGV_f0adff76827e73ff 1
#line 1 "shaders/clearing/meadow_flower.raster.hlsl"


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
#line 14 "shaders/clearing/meadow_flower.raster.hlsl"

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
#line 15 "shaders/clearing/meadow_flower.raster.hlsl"

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
#line 16 "shaders/clearing/meadow_flower.raster.hlsl"

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
#line 17 "shaders/clearing/meadow_flower.raster.hlsl"

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
#line 18 "shaders/clearing/meadow_flower.raster.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 19 "shaders/clearing/meadow_flower.raster.hlsl"

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
#line 20 "shaders/clearing/meadow_flower.raster.hlsl"

#line 1 "shaders/paths/path_mask_sampler.inc.hlsl"


#ifndef TINYGLADE_PATHS_PATH_MASK_SAMPLER_HLSL
#define TINYGLADE_PATHS_PATH_MASK_SAMPLER_HLSL

#line 1 "shaders/inc/texture_utils.hlsl"




#ifndef TINYGLADE_INC_TEXTURE_UTILS_HLSL
#define TINYGLADE_INC_TEXTURE_UTILS_HLSL


float4 sample_lod_bias(Texture2D<float4> tex, SamplerState samp, float2 uv, float mip_bias)
{
    return tex.SampleBias(samp, uv, mip_bias);
}


float4 sample_lod(Texture2D<float4> tex, SamplerState samp, float2 uv, float lod)
{
    return tex.SampleLevel(samp, uv, lod);
}


float4 load_texel(Texture2D<float4> tex, int2 pixel)
{
    return tex.Load(int3(pixel, 0));
}

#endif 
#line 6 "shaders/paths/path_mask_sampler.inc.hlsl"



static const float PATH_MASK_SAMPLE_SCALE = 0.0076923076994717121124267578125f;




float get_path_mask(Texture2D<float4> path_mask, SamplerState s, float2 world_xz)
{
    float2 uv = world_xz * PATH_MASK_SAMPLE_SCALE + 0.5f;
    return path_mask.SampleLevel(s, uv, 0.0f).x;
}



bool is_path(Texture2D<float4> path_mask, SamplerState s, float2 world_xz, float threshold)
{
    return get_path_mask(path_mask, s, world_xz) > threshold;
}

#endif 
#line 21 "shaders/clearing/meadow_flower.raster.hlsl"

#line 1 "shaders/inc/texture_utils.hlsl"




#ifndef TINYGLADE_INC_TEXTURE_UTILS_HLSL
#define TINYGLADE_INC_TEXTURE_UTILS_HLSL


float4 sample_lod_bias(Texture2D<float4> tex, SamplerState samp, float2 uv, float mip_bias)
{
    return tex.SampleBias(samp, uv, mip_bias);
}


float4 sample_lod(Texture2D<float4> tex, SamplerState samp, float2 uv, float lod)
{
    return tex.SampleLevel(samp, uv, lod);
}


float4 load_texel(Texture2D<float4> tex, int2 pixel)
{
    return tex.Load(int3(pixel, 0));
}

#endif 
#line 22 "shaders/clearing/meadow_flower.raster.hlsl"


Texture2D<float4> terrain_heightmap : register(t20, space0);
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
#line 25 "shaders/clearing/meadow_flower.raster.hlsl"

#line 1 "shaders/inc/rocky_terrain.inc.hlsl"


#ifndef TINYGLADE_INC_ROCKY_TERRAIN_HLSL
#define TINYGLADE_INC_ROCKY_TERRAIN_HLSL


static const float ROCKY_TERRAIN_SAMPLE_SCALE = 0.0076923076994717121124267578125;



float get_rocky_terrain_height(Texture2D rocky_terrain, SamplerState s, float2 world_xz)
{
    float2 uv = world_xz * ROCKY_TERRAIN_SAMPLE_SCALE + 0.5;
    float t = rocky_terrain.SampleLevel(s, uv, 0.0).x;
    return lerp(-2.5, 24.0, t);
}



bool is_rocky_terrain(Texture2D rocky_terrain, SamplerState s, float2 world_xz, float threshold)
{
    float2 uv = world_xz * ROCKY_TERRAIN_SAMPLE_SCALE + 0.5;
    return rocky_terrain.SampleLevel(s, uv, 0.0).x > threshold;
}

#endif 
#line 26 "shaders/clearing/meadow_flower.raster.hlsl"

#line 1 "shaders/water/water_mask_sampler.inc.hlsl"










#ifndef TINYGLADE_WATER_WATER_MASK_SAMPLER_HLSL
#define TINYGLADE_WATER_WATER_MASK_SAMPLER_HLSL

#line 1 "shaders/inc/texture_utils.hlsl"




#ifndef TINYGLADE_INC_TEXTURE_UTILS_HLSL
#define TINYGLADE_INC_TEXTURE_UTILS_HLSL


float4 sample_lod_bias(Texture2D<float4> tex, SamplerState samp, float2 uv, float mip_bias)
{
    return tex.SampleBias(samp, uv, mip_bias);
}


float4 sample_lod(Texture2D<float4> tex, SamplerState samp, float2 uv, float lod)
{
    return tex.SampleLevel(samp, uv, lod);
}


float4 load_texel(Texture2D<float4> tex, int2 pixel)
{
    return tex.Load(int3(pixel, 0));
}

#endif 
#line 14 "shaders/water/water_mask_sampler.inc.hlsl"



static const float WATER_MASK_SAMPLE_SCALE = 0.0076923076994717121124267578125f;


float get_water_mask(Texture2D<float4> water_mask, SamplerState s, float2 world_xz)
{
    float2 uv = world_xz * WATER_MASK_SAMPLE_SCALE + 0.5f;
    return water_mask.SampleLevel(s, uv, 0.0f).y;
}


bool is_water(Texture2D<float4> water_mask, SamplerState s, float2 world_xz, float threshold)
{
    return get_water_mask(water_mask, s, world_xz) > threshold;
}

#endif 
#line 27 "shaders/clearing/meadow_flower.raster.hlsl"

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
#line 28 "shaders/clearing/meadow_flower.raster.hlsl"

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
#line 29 "shaders/clearing/meadow_flower.raster.hlsl"

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
#line 30 "shaders/clearing/meadow_flower.raster.hlsl"

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
#line 31 "shaders/clearing/meadow_flower.raster.hlsl"



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
#line 35 "shaders/clearing/meadow_flower.raster.hlsl"

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
#line 36 "shaders/clearing/meadow_flower.raster.hlsl"

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
#line 37 "shaders/clearing/meadow_flower.raster.hlsl"

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
#line 38 "shaders/clearing/meadow_flower.raster.hlsl"

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
#line 39 "shaders/clearing/meadow_flower.raster.hlsl"

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
#line 40 "shaders/clearing/meadow_flower.raster.hlsl"

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
#line 41 "shaders/clearing/meadow_flower.raster.hlsl"

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
#line 42 "shaders/clearing/meadow_flower.raster.hlsl"




float4x4 model_to_world;
float time;
uint2 path_mask_dims;
uint2 albedo_map_dims;
uint2 normal_map_dims;
int has_normal_map;
uint2 alpha_map_dims;
uint2 leafy_greens_dims;
uint2 grass_object_push_dims;
uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
uint2 rocky_terrain_dims;
uint2 water_mask_dims;
int bool_render_shadows;

Texture2D<float4> path_mask : register(t14, space0);
Texture2D<float4> grass_object_push : register(t19, space0);
Texture2D<float4> rocky_terrain : register(t22, space0);
Texture2D<float4> water_mask : register(t23, space0);
Texture2D<float4> albedo_map : register(t15, space0);
Texture2D<float4> normal_map : register(t16, space0);
Texture2D<float4> alpha_map : register(t17, space0);
Texture2D<float4> leafy_greens : register(t18, space0);

struct VS_IN
{
    [[vk::location(0)]] float3 vertex_position: Vertex_Position;
    [[vk::location(1)]] float3 vertex_normal: Vertex_Normal;
    [[vk::location(2)]] float2 vertex_uv: Vertex_UV;
    [[vk::location(3)]] float3 flower_pivot: flower_pivot;
    [[vk::location(4)]] int stem: stem;
    [[vk::location(5)]] float wind_bend: wind_bend;
};

struct VS_OUT
{
    float2 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    nointerpolation int C5 : TEXCOORD5;
    float2 C6 : TEXCOORD6;
    float3x3 C7 : TEXCOORD7;
    float4 position : SV_Position;
};

VS_OUT vs_main(VS_IN i)
{
    VS_OUT o;
    float4 wp;
    float2 wind_uv;
    float3 world_pos;
    float3 normal;
    int stem_out;
    float2 uv_out;
    float3x3 tbn;
    do
    {
        float2 mask_uv = (i.flower_pivot.xz * (1.0f / 130.0f)) + 0.5f;
        bool in_bounds = !((mask_uv.x < 0.0f) || (mask_uv.x > 1.0f) || (mask_uv.y < 0.0f) || (mask_uv.y > 1.0f));
        float rocky = in_bounds ? rocky_terrain.SampleLevel(sampler_default, mask_uv, 0.0f).x : 0.0f;
        if ((smoothstep(0.075f, 0.125f, path_mask.SampleLevel(sampler_llb, mask_uv, 0.0f).x) > 0.1f) || (rocky > 0.1f))
        {
            wp = asfloat(0x7fbfffffu);
            wind_uv = 0.0f;
            world_pos = 0.0f;
            normal = 0.0f;
            stem_out = 0;
            uv_out = 0.0f;
            tbn = float3x3(0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f);
            break;
        }
        bool water_reject;
        if (!((smoothstep(0.075f, 0.125f, path_mask.SampleLevel(sampler_llb, mask_uv, 0.0f).x) > 0.1f) || (rocky > 0.1f)))
        {
            float water = in_bounds ? water_mask.SampleLevel(sampler_default, mask_uv, 0.0f).y : 0.0f;
            water_reject = water > 0.25f;
        }
        else
        {
            water_reject = true;
        }
        if (water_reject)
        {
            wp = asfloat(0x7fbfffffu);
            wind_uv = 0.0f;
            world_pos = 0.0f;
            normal = 0.0f;
            stem_out = 0;
            uv_out = 0.0f;
            tbn = float3x3(0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f);
            break;
        }
        float4 v = float4(i.vertex_position, 1.0f);
        v.y = i.vertex_position.y + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, mask_uv, 0.0f).x);
        float2 wind_xy = (i.flower_pivot.xz * 0.1f) + float2(time, 0.0f) * 1.2f;
        int2 wind_cell = int2(floor(wind_xy));
        float2 wind_frac = frac(wind_xy);
        float2 wind_smooth = wind_frac * wind_frac * (3.0f - (2.0f * wind_frac));
        float wind_n00 = incb_perlin_hash01(hash(uint2(wind_cell + int2(0, 0))));
        float wind_n10 = incb_perlin_hash01(hash(uint2(wind_cell + int2(1, 0))));
        float wind_n01 = incb_perlin_hash01(hash(uint2(wind_cell + int2(0, 1))));
        float wind_n11 = incb_perlin_hash01(hash(uint2(wind_cell + int2(1, 1))));
        float wind_noise = ((1.0f - wind_smooth.y) * ((wind_n00 * (1.0f - wind_smooth.x)) + (wind_n10 * wind_smooth.x)))
            + ((wind_n01 * (1.0f - wind_smooth.x)) * wind_smooth.y)
            + ((wind_n11 * wind_smooth.x) * wind_smooth.y);
        v.x = i.vertex_position.x - (i.wind_bend * sin((wind_noise * 3.14f) - 1.57f + 0.3f)) * 0.175f;
        float4 push = grass_object_push.SampleLevel(sampler_default, mask_uv, 0.0f);
        float4 v2;
        if (clamp(push.z, 0.0f, 1.0f) > 0.0f)
        {
            float2 pushed = v.xz + ((push.xy * pow(i.wind_bend, 0.5f)) * 0.5f);
            v2 = float4(pushed.x, v.y, pushed.y, v.w);
        }
        else
        {
            v2 = v;
        }
        wp = mul(view_constants[current_view_index].world_to_sample, mul(model_to_world, float4(v2.xyz, 1.0f)));
        wind_uv = wind_xy;
        world_pos = v2.xyz;
        normal = i.vertex_normal;
        stem_out = i.stem;
        uv_out = i.vertex_uv;
        tbn = transpose(float3x3(normalize(mul(model_to_world, float4(1.0f, 0.0f, 0.0f, 0.0f)).xyz),
                                 normalize(mul(model_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz),
                                 normalize(mul(model_to_world, float4(0.0f, 1.0f, 0.0f, 0.0f)).xyz)));
        break;
    } while (false);
    o.position = wp;
    o.C1 = wind_uv;
    o.C2 = world_pos;
    o.C3 = normal;
    o.C5 = stem_out;
    o.C6 = uv_out;
    o.C7 = tbn;
    return o;
}


uint pack_normal8(float3 n)
{
    return (uint(clamp(n.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(n.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(n.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u));
}

struct PS_IN
{
    float2 C1 : TEXCOORD1;
    float3 C3 : TEXCOORD3;
    nointerpolation int C5 : TEXCOORD5;
    float2 C6 : TEXCOORD6;
    float3x3 C7 : TEXCOORD7;
};

struct PS_OUT
{
    uint2 target : SV_Target0;
};

PS_OUT ps_main(PS_IN i, float4 frag_coord : SV_Position)
{
    PS_OUT o;
    if (alpha_map.Sample(sampler_llr, i.C6).x < 0.5f)
    {
        discard;
    }
    if (bool_render_shadows != 1)
    {
        uint2 pix = uint2(frag_coord.xy) & uint2(1u, 1u);
        if ((frame_index & 3u) == (pix.x + (pix.y * 2u)))
        {
            discard;
        }
    }
    float4 albedo_tex = albedo_map.Sample(sampler_llr, i.C1);
    float3 albedo;
    if (i.C5 == 1)
    {
        albedo = leafy_greens.Sample(sampler_llr, i.C1).xyz;
    }
    else
    {
        albedo = albedo_tex.xyz;
    }
    float3 n;
    if (has_normal_map == 1)
    {
        n = normalize(mul(i.C7, (normal_map.Sample(sampler_llr, i.C6).xyz * 2.0f) - 1.0f));
    }
    else
    {
        n = i.C3;
    }
    float3 packed_n = (n * 0.5f) + 0.5f;
    float3 color = sqrt(clamp(albedo, 0.0f, 1.0f));
    o.target = uint2((pack_normal8(packed_n) + 0x80000000u), (pack_normal8(color) + 0x4D000000u));
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
    column_major float4x4 model_to_world : packoffset(c1);
    float time : packoffset(c5);
    uint2 path_mask_dims : packoffset(c5.y);
    uint2 albedo_map_dims : packoffset(c6);
    uint2 normal_map_dims : packoffset(c6.z);
    int has_normal_map : packoffset(c7);
    uint2 alpha_map_dims : packoffset(c7.y);
    uint2 leafy_greens_dims : packoffset(c8);
    uint2 grass_object_push_dims : packoffset(c8.z);
    uint2 terrain_heightmap_dims : packoffset(c9);
    uint2 visual_terrain_heightmap_dims : packoffset(c9.z);
    uint2 rocky_terrain_dims : packoffset(c10);
    uint2 water_mask_dims : packoffset(c10.z);
    int bool_render_shadows : packoffset(c11);
};

SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> path_mask : register(t14, space0);
Texture2D<float4> grass_object_push : register(t19, space0);
Texture2D<float4> terrain_heightmap : register(t20, space0);
Texture2D<float4> rocky_terrain : register(t22, space0);
Texture2D<float4> water_mask : register(t23, space0);

static float4 gl_Position;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static float3 in_var_flower_pivot;
static int in_var_stem;
static float in_var_wind_bend;
static float2 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static int out_var_C5;
static float2 out_var_C6;
static float3x3 out_var_C7;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(3)]] float3 in_var_flower_pivot: flower_pivot;
    [[vk::location(4)]] int in_var_stem: stem;
    [[vk::location(5)]] float in_var_wind_bend: wind_bend;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float2 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(5)]] nointerpolation int out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float2 out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] float3x3 out_var_C7 : TEXCOORD7;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float4 _448;
    float2 _449;
    float3 _450;
    float3 _451;
    int _452;
    float2 _453;
    float3x3 _454;
    do
    {
        float _166;
        bool _168;
        float2 _159 = (in_var_flower_pivot.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        float _193;
        do
        {
            _166 = _159.x;
            _168 = !(_166 < 0.0f);
            bool _172;
            if (_168)
            {
                _172 = _166 > 1.0f;
            }
            else
            {
                _172 = true;
            }
            bool _178;
            if (!_172)
            {
                _178 = _159.y < 0.0f;
            }
            else
            {
                _178 = true;
            }
            bool _184;
            if (!_178)
            {
                _184 = _159.y > 1.0f;
            }
            else
            {
                _184 = true;
            }
            if (_184)
            {
                _193 = 0.0f;
                break;
            }
            else
            {
                _193 = rocky_terrain.SampleLevel(sampler_default, _159, 0.0f).x;
                break;
            }
            break; 
        } while(false);
        bool _199;
        if (!(smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, _159, 0.0f).x) > 0.100000001490116119384765625f))
        {
            _199 = _193 > 0.100000001490116119384765625f;
        }
        else
        {
            _199 = true;
        }
        bool _231;
        if (!_199)
        {
            float _229;
            do
            {
                bool _208;
                if (_168)
                {
                    _208 = _166 > 1.0f;
                }
                else
                {
                    _208 = true;
                }
                bool _214;
                if (!_208)
                {
                    _214 = _159.y < 0.0f;
                }
                else
                {
                    _214 = true;
                }
                bool _220;
                if (!_214)
                {
                    _220 = _159.y > 1.0f;
                }
                else
                {
                    _220 = true;
                }
                if (_220)
                {
                    _229 = 0.0f;
                    break;
                }
                else
                {
                    _229 = water_mask.SampleLevel(sampler_default, _159, 0.0f).y;
                    break;
                }
                break; 
            } while(false);
            _231 = _229 > 0.25f;
        }
        else
        {
            _231 = true;
        }
        if (_231)
        {
            _448 = asfloat(0x7fbfffffu          ).xxxx;
            _449 = 0.0f.xx;
            _450 = 0.0f.xxx;
            _451 = 0.0f.xxx;
            _452 = 0;
            _453 = 0.0f.xx;
            _454 = float3x3(0.0f.xxx, 0.0f.xxx, 0.0f.xxx);
            break;
        }
        float4 _237 = float4(in_var_Vertex_Position, 1.0f);
        _237.y = in_var_Vertex_Position.y + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _159, 0.0f).x);
        float2 _246 = in_var_flower_pivot.xz * 0.100000001490116119384765625f;
        float2 _251 = _246 + (float2(time, 0.0f) * 1.2000000476837158203125f);
        int2 _253 = int2(floor(_251));
        float2 _254 = frac(_251);
        uint2 _255 = uint2(_253);
        uint _257 = _255.y;
        uint _259 = _257 + (_257 << 10u);
        uint _261 = _259 ^ (_259 >> 6u);
        uint _263 = _261 + (_261 << 3u);
        uint _265 = _263 ^ (_263 >> 11u);
        uint _271 = (((_255.x * 1664525u) + (_265 + (_265 << 15u))) + 1013904223u) * 1664525u;
        uint _273 = _271 ^ (_271 >> 11u);
        uint _276 = _273 ^ ((_273 << 7u) & 2636928640u);
        uint _279 = _276 ^ ((_276 << 15u) & 4022730752u);
        uint2 _287 = uint2(_253 + int2(1, 0));
        uint _289 = _287.y;
        uint _291 = _289 + (_289 << 10u);
        uint _293 = _291 ^ (_291 >> 6u);
        uint _295 = _293 + (_293 << 3u);
        uint _297 = _295 ^ (_295 >> 11u);
        uint _303 = (((_287.x * 1664525u) + (_297 + (_297 << 15u))) + 1013904223u) * 1664525u;
        uint _305 = _303 ^ (_303 >> 11u);
        uint _308 = _305 ^ ((_305 << 7u) & 2636928640u);
        uint _311 = _308 ^ ((_308 << 15u) & 4022730752u);
        uint2 _319 = uint2(_253 + int2(0, 1));
        uint _321 = _319.y;
        uint _323 = _321 + (_321 << 10u);
        uint _325 = _323 ^ (_323 >> 6u);
        uint _327 = _325 + (_325 << 3u);
        uint _329 = _327 ^ (_327 >> 11u);
        uint _335 = (((_319.x * 1664525u) + (_329 + (_329 << 15u))) + 1013904223u) * 1664525u;
        uint _337 = _335 ^ (_335 >> 11u);
        uint _340 = _337 ^ ((_337 << 7u) & 2636928640u);
        uint _343 = _340 ^ ((_340 << 15u) & 4022730752u);
        uint2 _351 = uint2(_253 + int2(1, 1));
        uint _353 = _351.y;
        uint _355 = _353 + (_353 << 10u);
        uint _357 = _355 ^ (_355 >> 6u);
        uint _359 = _357 + (_357 << 3u);
        uint _361 = _359 ^ (_359 >> 11u);
        uint _367 = (((_351.x * 1664525u) + (_361 + (_361 << 15u))) + 1013904223u) * 1664525u;
        uint _369 = _367 ^ (_367 >> 11u);
        uint _372 = _369 ^ ((_369 << 7u) & 2636928640u);
        uint _375 = _372 ^ ((_372 << 15u) & 4022730752u);
        float2 _385 = (_254 * _254) * (3.0f.xx - (_254 * 2.0f));
        float _386 = _385.x;
        float _387 = 1.0f - _386;
        float _389 = _385.y;
        _237.x = in_var_Vertex_Position.x - ((in_var_wind_bend * (sin((((((1.0f - _389) * (((asfloat(((_279 ^ (_279 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _387) + ((asfloat(((_311 ^ (_311 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _386))) + (((asfloat(((_343 ^ (_343 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _387) * _389)) + (((asfloat(((_375 ^ (_375 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _386) * _389)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.17499999701976776123046875f);
        float4 _410 = grass_object_push.SampleLevel(sampler_default, _159, 0.0f);
        float4 _423;
        if (clamp(_410.z, 0.0f, 1.0f) > 0.0f)
        {
            float2 _421 = _237.xz + ((_410.xy * pow(in_var_wind_bend, 0.5f)) * 0.5f);
            _423 = float4(_421.x, _237.y, _421.y, _237.w);
        }
        else
        {
            _423 = _237;
        }
        _448 = mul(frame_constants_view_constants[current_view_index].world_to_sample, mul(model_to_world, float4(_423.xyz, 1.0f)));
        _449 = _246;
        _450 = _423.xyz;
        _451 = in_var_Vertex_Normal;
        _452 = in_var_stem;
        _453 = in_var_Vertex_UV;
        _454 = transpose(float3x3(normalize(mul(model_to_world, float4(1.0f, 0.0f, 0.0f, 0.0f)).xyz), normalize(mul(model_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz), normalize(mul(model_to_world, float4(0.0f, 1.0f, 0.0f, 0.0f)).xyz)));
        break;
    } while(false);
    gl_Position = _448;
    out_var_C1 = _449;
    out_var_C2 = _450;
    out_var_C3 = _451;
    out_var_C5 = _452;
    out_var_C6 = _453;
    out_var_C7 = _454;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_flower_pivot = stage_input.in_var_flower_pivot;
    in_var_stem = stage_input.in_var_stem;
    in_var_wind_bend = stage_input.in_var_wind_bend;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
    stage_output.out_var_C7 = out_var_C7;
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
    uint current_view_index : packoffset(c0);
    column_major float4x4 model_to_world : packoffset(c1);
    float time : packoffset(c5);
    uint2 path_mask_dims : packoffset(c5.y);
    uint2 albedo_map_dims : packoffset(c6);
    uint2 normal_map_dims : packoffset(c6.z);
    int has_normal_map : packoffset(c7);
    uint2 alpha_map_dims : packoffset(c7.y);
    uint2 leafy_greens_dims : packoffset(c8);
    uint2 grass_object_push_dims : packoffset(c8.z);
    uint2 terrain_heightmap_dims : packoffset(c9);
    uint2 visual_terrain_heightmap_dims : packoffset(c9.z);
    uint2 rocky_terrain_dims : packoffset(c10);
    uint2 water_mask_dims : packoffset(c10.z);
    int bool_render_shadows : packoffset(c11);
};

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> albedo_map : register(t15, space0);
Texture2D<float4> normal_map : register(t16, space0);
Texture2D<float4> alpha_map : register(t17, space0);
Texture2D<float4> leafy_greens : register(t18, space0);

static float4 gl_FragCoord;
static float2 in_var_C1;
static float3 in_var_C3;
static int in_var_C5;
static float2 in_var_C6;
static float3x3 in_var_C7;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float2 in_var_C1 : TEXCOORD1;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
    [[vk::location(5)]] nointerpolation int in_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float2 in_var_C6 : TEXCOORD6;
    [[vk::location(7)]] float3x3 in_var_C7 : TEXCOORD7;
    float4 gl_FragCoord : SV_Position;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    if (alpha_map.Sample(sampler_llr, in_var_C6).x < 0.5f)
    {
        discard;
    }
    if (!((bool_render_shadows == 1) ? true : false))
    {
        uint2 _128 = uint2(gl_FragCoord.xy) & uint2(1u, 1u);
        if ((frame_constants_frame_index & 3u) == (_128.x + (_128.y * 2u)))
        {
            discard;
        }
    }
    float4 _141 = albedo_map.Sample(sampler_llr, in_var_C1);
    float3 _150;
    if (in_var_C5 == 1)
    {
        _150 = leafy_greens.Sample(sampler_llr, in_var_C1).xyz;
    }
    else
    {
        _150 = _141.xyz;
    }
    float3 _165;
    if (has_normal_map == 1)
    {
        _165 = normalize(mul(in_var_C7, (normal_map.Sample(sampler_llr, in_var_C6).xyz * 2.0f) - 1.0f.xxx));
    }
    else
    {
        _165 = in_var_C3;
    }
    float3 _167 = (_165 * 0.5f) + 0.5f.xxx;
    float3 _189 = sqrt(clamp(_150, 0.0f.xxx, 1.0f.xxx));
    out_var_SV_TARGET0 = uint2(((uint((clamp(_167.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_167.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_167.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 2147483648u, ((uint((clamp(_189.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_189.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_189.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 1291845632u);
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C1 = stage_input.in_var_C1;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C5 = stage_input.in_var_C5;
    in_var_C6 = stage_input.in_var_C6;
    in_var_C7 = stage_input.in_var_C7;
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
    column_major float4x4 model_to_world : packoffset(c1);
    float time : packoffset(c5);
    uint2 path_mask_dims : packoffset(c5.y);
    uint2 albedo_map_dims : packoffset(c6);
    uint2 normal_map_dims : packoffset(c6.z);
    int has_normal_map : packoffset(c7);
    uint2 alpha_map_dims : packoffset(c7.y);
    uint2 leafy_greens_dims : packoffset(c8);
    uint2 grass_object_push_dims : packoffset(c8.z);
    uint2 terrain_heightmap_dims : packoffset(c9);
    uint2 visual_terrain_heightmap_dims : packoffset(c9.z);
    uint2 rocky_terrain_dims : packoffset(c10);
    uint2 water_mask_dims : packoffset(c10.z);
    int bool_render_shadows : packoffset(c11);
};

SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> path_mask : register(t14, space0);
Texture2D<float4> grass_object_push : register(t19, space0);
Texture2D<float4> terrain_heightmap : register(t20, space0);
Texture2D<float4> rocky_terrain : register(t22, space0);
Texture2D<float4> water_mask : register(t23, space0);

static float4 gl_Position;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static float3 in_var_flower_pivot;
static int in_var_stem;
static float in_var_wind_bend;
static float2 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static int out_var_C5;
static float2 out_var_C6;
static float3x3 out_var_C7;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(3)]] float3 in_var_flower_pivot: flower_pivot;
    [[vk::location(4)]] int in_var_stem: stem;
    [[vk::location(5)]] float in_var_wind_bend: wind_bend;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float2 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(5)]] nointerpolation int out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float2 out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] float3x3 out_var_C7 : TEXCOORD7;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float4 _448;
    float2 _449;
    float3 _450;
    float3 _451;
    int _452;
    float2 _453;
    float3x3 _454;
    do
    {
        float _166;
        bool _168;
        float2 _159 = (in_var_flower_pivot.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        float _193;
        do
        {
            _166 = _159.x;
            _168 = !(_166 < 0.0f);
            bool _172;
            if (_168)
            {
                _172 = _166 > 1.0f;
            }
            else
            {
                _172 = true;
            }
            bool _178;
            if (!_172)
            {
                _178 = _159.y < 0.0f;
            }
            else
            {
                _178 = true;
            }
            bool _184;
            if (!_178)
            {
                _184 = _159.y > 1.0f;
            }
            else
            {
                _184 = true;
            }
            if (_184)
            {
                _193 = 0.0f;
                break;
            }
            else
            {
                _193 = rocky_terrain.SampleLevel(sampler_default, _159, 0.0f).x;
                break;
            }
            break; 
        } while(false);
        bool _199;
        if (!(smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, _159, 0.0f).x) > 0.100000001490116119384765625f))
        {
            _199 = _193 > 0.100000001490116119384765625f;
        }
        else
        {
            _199 = true;
        }
        bool _231;
        if (!_199)
        {
            float _229;
            do
            {
                bool _208;
                if (_168)
                {
                    _208 = _166 > 1.0f;
                }
                else
                {
                    _208 = true;
                }
                bool _214;
                if (!_208)
                {
                    _214 = _159.y < 0.0f;
                }
                else
                {
                    _214 = true;
                }
                bool _220;
                if (!_214)
                {
                    _220 = _159.y > 1.0f;
                }
                else
                {
                    _220 = true;
                }
                if (_220)
                {
                    _229 = 0.0f;
                    break;
                }
                else
                {
                    _229 = water_mask.SampleLevel(sampler_default, _159, 0.0f).y;
                    break;
                }
                break; 
            } while(false);
            _231 = _229 > 0.25f;
        }
        else
        {
            _231 = true;
        }
        if (_231)
        {
            _448 = asfloat(0x7fbfffffu          ).xxxx;
            _449 = 0.0f.xx;
            _450 = 0.0f.xxx;
            _451 = 0.0f.xxx;
            _452 = 0;
            _453 = 0.0f.xx;
            _454 = float3x3(0.0f.xxx, 0.0f.xxx, 0.0f.xxx);
            break;
        }
        float4 _237 = float4(in_var_Vertex_Position, 1.0f);
        _237.y = in_var_Vertex_Position.y + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _159, 0.0f).x);
        float2 _246 = in_var_flower_pivot.xz * 0.100000001490116119384765625f;
        float2 _251 = _246 + (float2(time, 0.0f) * 1.2000000476837158203125f);
        int2 _253 = int2(floor(_251));
        float2 _254 = frac(_251);
        uint2 _255 = uint2(_253);
        uint _257 = _255.y;
        uint _259 = _257 + (_257 << 10u);
        uint _261 = _259 ^ (_259 >> 6u);
        uint _263 = _261 + (_261 << 3u);
        uint _265 = _263 ^ (_263 >> 11u);
        uint _271 = (((_255.x * 1664525u) + (_265 + (_265 << 15u))) + 1013904223u) * 1664525u;
        uint _273 = _271 ^ (_271 >> 11u);
        uint _276 = _273 ^ ((_273 << 7u) & 2636928640u);
        uint _279 = _276 ^ ((_276 << 15u) & 4022730752u);
        uint2 _287 = uint2(_253 + int2(1, 0));
        uint _289 = _287.y;
        uint _291 = _289 + (_289 << 10u);
        uint _293 = _291 ^ (_291 >> 6u);
        uint _295 = _293 + (_293 << 3u);
        uint _297 = _295 ^ (_295 >> 11u);
        uint _303 = (((_287.x * 1664525u) + (_297 + (_297 << 15u))) + 1013904223u) * 1664525u;
        uint _305 = _303 ^ (_303 >> 11u);
        uint _308 = _305 ^ ((_305 << 7u) & 2636928640u);
        uint _311 = _308 ^ ((_308 << 15u) & 4022730752u);
        uint2 _319 = uint2(_253 + int2(0, 1));
        uint _321 = _319.y;
        uint _323 = _321 + (_321 << 10u);
        uint _325 = _323 ^ (_323 >> 6u);
        uint _327 = _325 + (_325 << 3u);
        uint _329 = _327 ^ (_327 >> 11u);
        uint _335 = (((_319.x * 1664525u) + (_329 + (_329 << 15u))) + 1013904223u) * 1664525u;
        uint _337 = _335 ^ (_335 >> 11u);
        uint _340 = _337 ^ ((_337 << 7u) & 2636928640u);
        uint _343 = _340 ^ ((_340 << 15u) & 4022730752u);
        uint2 _351 = uint2(_253 + int2(1, 1));
        uint _353 = _351.y;
        uint _355 = _353 + (_353 << 10u);
        uint _357 = _355 ^ (_355 >> 6u);
        uint _359 = _357 + (_357 << 3u);
        uint _361 = _359 ^ (_359 >> 11u);
        uint _367 = (((_351.x * 1664525u) + (_361 + (_361 << 15u))) + 1013904223u) * 1664525u;
        uint _369 = _367 ^ (_367 >> 11u);
        uint _372 = _369 ^ ((_369 << 7u) & 2636928640u);
        uint _375 = _372 ^ ((_372 << 15u) & 4022730752u);
        float2 _385 = (_254 * _254) * (3.0f.xx - (_254 * 2.0f));
        float _386 = _385.x;
        float _387 = 1.0f - _386;
        float _389 = _385.y;
        _237.x = in_var_Vertex_Position.x - ((in_var_wind_bend * (sin((((((1.0f - _389) * (((asfloat(((_279 ^ (_279 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _387) + ((asfloat(((_311 ^ (_311 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _386))) + (((asfloat(((_343 ^ (_343 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _387) * _389)) + (((asfloat(((_375 ^ (_375 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _386) * _389)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.17499999701976776123046875f);
        float4 _410 = grass_object_push.SampleLevel(sampler_default, _159, 0.0f);
        float4 _423;
        if (clamp(_410.z, 0.0f, 1.0f) > 0.0f)
        {
            float2 _421 = _237.xz + ((_410.xy * pow(in_var_wind_bend, 0.5f)) * 0.5f);
            _423 = float4(_421.x, _237.y, _421.y, _237.w);
        }
        else
        {
            _423 = _237;
        }
        _448 = mul(frame_constants_view_constants[current_view_index].world_to_sample, mul(model_to_world, float4(_423.xyz, 1.0f)));
        _449 = _246;
        _450 = _423.xyz;
        _451 = in_var_Vertex_Normal;
        _452 = in_var_stem;
        _453 = in_var_Vertex_UV;
        _454 = transpose(float3x3(normalize(mul(model_to_world, float4(1.0f, 0.0f, 0.0f, 0.0f)).xyz), normalize(mul(model_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz), normalize(mul(model_to_world, float4(0.0f, 1.0f, 0.0f, 0.0f)).xyz)));
        break;
    } while(false);
    gl_Position = _448;
    out_var_C1 = _449;
    out_var_C2 = _450;
    out_var_C3 = _451;
    out_var_C5 = _452;
    out_var_C6 = _453;
    out_var_C7 = _454;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_flower_pivot = stage_input.in_var_flower_pivot;
    in_var_stem = stage_input.in_var_stem;
    in_var_wind_bend = stage_input.in_var_wind_bend;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
    stage_output.out_var_C7 = out_var_C7;
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

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index : packoffset(c0);
    column_major float4x4 model_to_world : packoffset(c1);
    float time : packoffset(c5);
    uint2 path_mask_dims : packoffset(c5.y);
    uint2 albedo_map_dims : packoffset(c6);
    uint2 normal_map_dims : packoffset(c6.z);
    int has_normal_map : packoffset(c7);
    uint2 alpha_map_dims : packoffset(c7.y);
    uint2 leafy_greens_dims : packoffset(c8);
    uint2 grass_object_push_dims : packoffset(c8.z);
    uint2 terrain_heightmap_dims : packoffset(c9);
    uint2 visual_terrain_heightmap_dims : packoffset(c9.z);
    uint2 rocky_terrain_dims : packoffset(c10);
    uint2 water_mask_dims : packoffset(c10.z);
    int bool_render_shadows : packoffset(c11);
};

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> alpha_map : register(t17, space0);

static float4 gl_FragCoord;
static float2 in_var_C6;

struct SPIRV_Cross_Input
{
    [[vk::location(6)]] float2 in_var_C6 : TEXCOORD6;
    float4 gl_FragCoord : SV_Position;
};

void frag_main()
{
    if (alpha_map.Sample(sampler_llr, in_var_C6).x < 0.5f)
    {
        discard;
    }
    if (!((bool_render_shadows == 1) ? true : false))
    {
        uint2 _99 = uint2(gl_FragCoord.xy) & uint2(1u, 1u);
        if ((frame_constants_frame_index & 3u) == (_99.x + (_99.y * 2u)))
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
