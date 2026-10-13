#line 1 "shaders/paths/path_mask_visual.cs.hlsl"


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
#line 3 "shaders/paths/path_mask_visual.cs.hlsl"

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
#line 4 "shaders/paths/path_mask_visual.cs.hlsl"

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
#line 5 "shaders/paths/path_mask_visual.cs.hlsl"

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
#line 6 "shaders/paths/path_mask_visual.cs.hlsl"

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
#line 7 "shaders/paths/path_mask_visual.cs.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 8 "shaders/paths/path_mask_visual.cs.hlsl"

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
#line 9 "shaders/paths/path_mask_visual.cs.hlsl"

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
#line 10 "shaders/paths/path_mask_visual.cs.hlsl"

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
#line 11 "shaders/paths/path_mask_visual.cs.hlsl"

Texture2D<float4> terrain_heightmap : register(t14, space0);
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
#line 13 "shaders/paths/path_mask_visual.cs.hlsl"

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
#line 14 "shaders/paths/path_mask_visual.cs.hlsl"

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
#line 15 "shaders/paths/path_mask_visual.cs.hlsl"

#line 1 "shaders/inc/fbm_noise.hlsl"






#line 7 "shaders/inc/fbm_noise.hlsl"


float fbm(float2 p)
{
    const float2x2 rot = float2x2(0.8f, -0.6f, 0.6f, 0.8f);
    float value = 0.0f;
    float amp = 0.5f;
    float2 pos = p;
    [unroll]
    for (int i = 0; i < 2; i++)
    {
        value += amp * (-1.0f + 2.0f * perlin(pos));
        amp *= 0.55f;
        pos = mul(rot, pos * 1.9f);
    }
    return value;
}
#line 16 "shaders/paths/path_mask_visual.cs.hlsl"

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
#line 17 "shaders/paths/path_mask_visual.cs.hlsl"

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
#line 18 "shaders/paths/path_mask_visual.cs.hlsl"




Texture2D<float4> water_mask : register(t16, space0);
RWTexture2D<half4> paths_visual : register(u17, space0);
RWTexture2D<half> dirt_mask : register(u18, space0);
Texture2D<float4> path_mask_blurred : register(t19, space0);
Texture2D<float4> cobblestone1_mask_blurred : register(t20, space0);
Texture2D<float4> cobblestone2_mask_blurred : register(t21, space0);
Texture2D<float4> cobblestone3_mask_blurred : register(t22, space0);

cbuffer $Globals : register(b13, space0)
{
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 water_mask_dims;
    uint2 paths_visual_dims;
    uint2 dirt_mask_dims;
    uint2 path_mask_blurred_dims;
    uint2 cobblestone1_mask_blurred_dims;
    uint2 cobblestone2_mask_blurred_dims;
    uint2 cobblestone3_mask_blurred_dims;
    uint2 rocky_terrain_dims;
};






float vnoise_fbm(float2 pos)
{
    const float2x2 rot = float2x2(float2(0.800000011920928955078125f, -0.60000002384185791015625f),
                                  float2(0.60000002384185791015625f, 0.800000011920928955078125f));
    float value = 0.0f;
    float amp = 0.5f;
    for (int i = 0; i < 2; i++)
    {
        int2 cell = int2(floor(pos));
        float2 f = frac(pos);
        float2 u = (f * f) * (3.0f.xx - (f * 2.0f));

        float n00 = incb_perlin_hash01(hash(uint2(cell + int2(0, 0))));
        float n10 = incb_perlin_hash01(hash(uint2(cell + int2(1, 0))));
        float n01 = incb_perlin_hash01(hash(uint2(cell + int2(0, 1))));
        float n11 = incb_perlin_hash01(hash(uint2(cell + int2(1, 1))));

        float x0 = 1.0f - u.x;
        float y0 = 1.0f - u.y;
        float n = (y0 * ((n00 * x0) + (n10 * u.x)))
                + ((n01 * x0) * u.y)
                + ((n11 * u.x) * u.y);

        value += amp * ((-1.0f) + (2.0f * n));
        amp *= 0.550000011920928955078125f;
        pos = mul(rot, pos * 1.89999997615814208984375f);
    }
    return value;
}

[numthreads(8, 8, 1)]
void cs_main(int3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    uint2 p = uint2(gl_GlobalInvocationID.xy);

    
    float2 uv_center = float2(gl_GlobalInvocationID.xy) + 0.5f.xx;
    float2 world = (((uv_center / float2(int2(paths_visual_dims))) * 2.0f) - 1.0f.xx) * 65.0f.xx;
    float2 world_xz = float3(world.x, 0.0f, world.y).xz;

    
    float2 thm_dims = float2(terrain_heightmap_dims);
    float2 grad_scale = 130.0f.xx / thm_dims;
    float4 g0 = terrain_heightmap.GatherRed(sampler_default, uv_center / thm_dims, int2(0, 0));
    float2 grad0 = (float2((g0.z + g0.y) - (g0.w + g0.x), (g0.x + g0.y) - (g0.w + g0.z)) * 0.5f.xx) * 26.5f;
    float offset_scale = lerp(1.0f, 0.0f, clamp(length(grad0 / grad_scale), 0.0f, 1.0f));

    
    
    float2 fbm_base = (world_xz * 0.75f) + 7.0f.xx;

    float n0 = vnoise_fbm(fbm_base);
    float path_cov = path_mask_blurred.SampleLevel(sampler_lnc,
        ((world_xz + (((n0 * 0.5f) * offset_scale) * 1.2000000476837158203125f).xx) * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x;

    float n1 = vnoise_fbm(fbm_base);
    float c1_cov = cobblestone1_mask_blurred.SampleLevel(sampler_lnc,
        ((world_xz + (((n1 * 0.5f) * offset_scale) * 1.2000000476837158203125f).xx) * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x;

    float n2 = vnoise_fbm(fbm_base);
    float c2_cov = cobblestone2_mask_blurred.SampleLevel(sampler_lnc,
        ((world_xz + (((n2 * 0.5f) * offset_scale) * 1.2000000476837158203125f).xx) * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x;

    float n3 = vnoise_fbm(fbm_base);
    float c3_cov = cobblestone3_mask_blurred.SampleLevel(sampler_lnc,
        ((world_xz + (((n3 * 0.5f) * offset_scale) * 1.2000000476837158203125f).xx) * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x;

    
    
    
    
    float2 uv_ws = (world_xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
    float4 g1 = terrain_heightmap.GatherRed(sampler_default, uv_ws, int2(0, 0));
    float2 grad1 = (float2((g1.z + g1.y) - (g1.w + g1.x), (g1.x + g1.y) - (g1.w + g1.z)) * 0.5f.xx) * 26.5f;
    float water = 0.0f;
    if (uv_ws.x >= 0.0f && uv_ws.x <= 1.0f && uv_ws.y >= 0.0f && uv_ws.y <= 1.0f)
    {
        water = water_mask.SampleLevel(sampler_default, uv_ws, 0.0f).y;
    }
    float rocky_water = smoothstep(0.5f, 1.0f, length(grad1 / grad_scale))
                      + smoothstep(0.0f, 0.449999988079071044921875f, water);

    
    
    float4 outv = paths_visual[p];
    outv.x = path_cov;
    paths_visual[p] = outv;

    outv = paths_visual[p];
    outv.y = clamp(c1_cov - rocky_water, 0.0f, 1.0f);
    paths_visual[p] = outv;

    outv = paths_visual[p];
    outv.z = clamp(c2_cov - rocky_water, 0.0f, 1.0f);
    paths_visual[p] = outv;

    outv = paths_visual[p];
    outv.w = clamp(c3_cov - rocky_water, 0.0f, 1.0f);
    paths_visual[p] = outv;

    dirt_mask[p] = max(max(path_cov, c1_cov), max(c2_cov, c3_cov));
}
