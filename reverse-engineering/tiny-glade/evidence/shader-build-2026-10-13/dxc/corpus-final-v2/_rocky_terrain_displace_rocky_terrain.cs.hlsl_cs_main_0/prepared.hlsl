#line 1 "shaders/rocky_terrain/displace_rocky_terrain.cs.hlsl"
























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
#line 25 "shaders/rocky_terrain/displace_rocky_terrain.cs.hlsl"

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
#line 26 "shaders/rocky_terrain/displace_rocky_terrain.cs.hlsl"

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
#line 27 "shaders/rocky_terrain/displace_rocky_terrain.cs.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 28 "shaders/rocky_terrain/displace_rocky_terrain.cs.hlsl"

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
#line 29 "shaders/rocky_terrain/displace_rocky_terrain.cs.hlsl"

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
#line 30 "shaders/rocky_terrain/displace_rocky_terrain.cs.hlsl"

#line 1 "shaders/rocky_terrain/inc_mesh.inc.hlsl"

















#ifndef TINYGLADE_ROCKY_TERRAIN_INC_MESH_HLSL
#define TINYGLADE_ROCKY_TERRAIN_INC_MESH_HLSL


struct Vertex
{
    float3 rest_position;
    float cell_bby;
    float3 position;
    int cell_id;
    float3 dir_to_centroid;
    int is_corner;
};




struct Triangle
{
    int3 neighbours;
    int is_top;
    float3 normal;
    float pad1;
};

#endif 
#line 31 "shaders/rocky_terrain/displace_rocky_terrain.cs.hlsl"

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
#line 32 "shaders/rocky_terrain/displace_rocky_terrain.cs.hlsl"

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
#line 33 "shaders/rocky_terrain/displace_rocky_terrain.cs.hlsl"

Texture2D<float4> terrain_heightmap : register(t16, space0);
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
#line 35 "shaders/rocky_terrain/displace_rocky_terrain.cs.hlsl"

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
#line 36 "shaders/rocky_terrain/displace_rocky_terrain.cs.hlsl"

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
#line 37 "shaders/rocky_terrain/displace_rocky_terrain.cs.hlsl"

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
#line 38 "shaders/rocky_terrain/displace_rocky_terrain.cs.hlsl"

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
#line 39 "shaders/rocky_terrain/displace_rocky_terrain.cs.hlsl"





RWStructuredBuffer<Vertex> __alias_vertices : register(u13, space0);
RWStructuredBuffer<Triangle> __alias_triangles : register(u14, space0);

cbuffer $Globals : register(b15, space0)
{
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 rocky_terrain_dims;
    uint2 path_mask_dims;
    uint2 water_mask_dims;
};

struct PushConstants
{
    uint triangle_count;
};
[[vk::push_constant]]
PushConstants push_constants;

Texture2D<float4> rocky_terrain : register(t18, space0);
Texture2D<float4> path_mask : register(t19, space0);
Texture2D<float4> water_mask : register(t20, space0);


static const float TERRAIN_UV_SCALE = 0.0076923076994717121124267578125f;




float sample_rocky_terrain_channel(float2 uv, int channel)
{
    if (uv.x < 0.0f || uv.x > 1.0f || uv.y < 0.0f || uv.y > 1.0f)
    {
        return 0.0f;
    }
    float4 t = rocky_terrain.SampleLevel(sampler_default, uv, 0.0f);
    return (channel == 0) ? t.x : ((channel == 1) ? t.y : t.z);
}




float value_noise(float2 p)
{
    int2 cell = int2(floor(p));
    float2 f = frac(p);
    float2 u = f * f * (3.0f - 2.0f * f);

    float n00 = incb_perlin_hash01(hash(uint2(cell + int2(0, 0))));
    float n10 = incb_perlin_hash01(hash(uint2(cell + int2(1, 0))));
    float n01 = incb_perlin_hash01(hash(uint2(cell + int2(0, 1))));
    float n11 = incb_perlin_hash01(hash(uint2(cell + int2(1, 1))));

    return (((1.0f - u.y) * ((n00 * (1.0f - u.x)) + (n10 * u.x)))
          + (((n01 * (1.0f - u.x)) * u.y) + ((n11 * u.x) * u.y)));
}

[numthreads(64, 1, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    if (gl_GlobalInvocationID.x >= push_constants.triangle_count)
    {
        return;
    }

    uint triangle_base = gl_GlobalInvocationID.x * 3u;

    
    
    bool displace = false;
    for (uint v = 0; v < 3; ++v)
    {
        float2 uv = (__alias_vertices[triangle_base + v].rest_position * 65.0f).xz
                    * TERRAIN_UV_SCALE + 0.5f;
        float r_z = sample_rocky_terrain_channel(uv, 2);
        float r_x = sample_rocky_terrain_channel(uv, 0);
        float r_y = sample_rocky_terrain_channel(uv, 1);
        float region = clamp(smoothstep(0.1f, 0.2f, r_x + r_y)
                           + smoothstep(0.0f, 0.2f, water_mask.SampleLevel(sampler_llc, uv, 0.0f).y), 0.0f, 1.0f);
        if (region > 0.0f || r_z > 0.0f)
        {
            displace = true;
            break;
        }
    }

    if (!displace)
    {
        Vertex v0 = __alias_vertices[triangle_base];
        v0.position = asfloat(0x7fbfffffu).xxx;
        __alias_vertices[triangle_base] = v0;
        return;
    }

    for (uint vi = 0; vi < 3; ++vi)
    {
        uint vertex_id = triangle_base + vi;
        Vertex vert = __alias_vertices[vertex_id];

        
        uint cell_hash = hash(uint(vert.cell_id));
        float cell_rand = asfloat((cell_hash & 0x7FFFFFu) | 0x3F800000u) - 1.0f;

        
        
        float3 jittered = vert.rest_position
                        + (vert.dir_to_centroid * 0.001f * lerp(-3.0f, 8.0f, cell_rand));
        float3 pos = jittered * 65.0f;
        float2 uv0 = (pos.xz * TERRAIN_UV_SCALE) + 0.5f;

        
        float h = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, uv0, 0.0f).x);
        float3 moved = pos;
        moved.y = h;
        if (abs(pos.x) > 65.0f || abs(pos.z) > 65.0f)
        {
            moved.y = h - 2.0f;
        }

        float2 uv1 = (moved.xz * TERRAIN_UV_SCALE) + 0.5f;
        float r_x1 = sample_rocky_terrain_channel(uv1, 0);
        float r_y1 = sample_rocky_terrain_channel(uv1, 1);
        float rock_region = clamp(smoothstep(0.1f, 0.2f, r_x1 + r_y1)
                                + smoothstep(0.0f, 0.2f, water_mask.SampleLevel(sampler_llc, uv1, 0.0f).y), 0.0f, 1.0f);
        float r_y2 = sample_rocky_terrain_channel(uv1, 1);
        float path_t = smoothstep(0.1f, 0.15f, path_mask.SampleLevel(sampler_llb, uv1, 0.0f).x);
        float r_z0 = sample_rocky_terrain_channel(uv0, 2);
        float water_y = water_mask.SampleLevel(sampler_llc, uv1, 0.0f).y;
        float wd = abs(water_y - 0.5f);
        float water_smooth;
        if (wd > 1.0f)
        {
            water_smooth = 0.0f;
        }
        else
        {
            water_smooth = 1.0f - ((wd * wd) * (3.0f - (2.0f * wd)));
        }

        
        float noise = value_noise(moved.xz * 0.2f);

        
        
        float3 result = moved;
        result.y = moved.y
            + lerp(-0.4f, 0.2f, clamp(rock_region + r_z0 - 10.0f * path_t, 0.0f, 1.0f))
            - smoothstep(0.5f, 0.6f, water_y) * 0.2f
            - (water_smooth * noise * (1.0f - smoothstep(0.3f, 0.5f,
                lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, uv1, 0.0f).x))));

        float3 displaced;
        if (rock_region > 0.0f)
        {
            
            
            float4 gather = terrain_heightmap.GatherRed(sampler_default, uv0, int2(0, 0));
            float2 scale = 130.0f / float2(terrain_heightmap_dims);
            float2 grad = (float2((gather.z + gather.y) - (gather.w + gather.x),
                                  (gather.x + gather.y) - (gather.w + gather.z)) * 0.5f) * 26.5f / scale;
            float3 normal = cross(normalize(float3(0.0f, grad.y, 1.0f)),
                                  normalize(float3(1.0f, grad.x, 0.0f)));
            float4 rt = rocky_terrain.SampleLevel(sampler_default, uv0, 0.0f);
            float rock_t = rt.x;

            
            
            float3 tilted = result
                + (normal * lerp(-0.3f, 0.1f * lerp(0.0f, 3.0f, cell_rand), vert.cell_bby)
                          * rock_region * (1.0f - path_t))
                - (normal * path_t);

            
            
            if ((vert.is_corner == 0) && (path_t == 0.0f))
            {
                float4 g1 = terrain_heightmap.GatherRed(sampler_default, uv0, int2(0, 0));
                float4 g2 = terrain_heightmap.GatherRed(sampler_default,
                    (tilted.xz * TERRAIN_UV_SCALE) + 0.5f, int2(0, 0));
                float grad1_len = length(((float2((g1.z + g1.y) - (g1.w + g1.x),
                                                  (g1.x + g1.y) - (g1.w + g1.z)) * 0.5f) * 26.5f) / scale);
                float grad2_len = length(((float2((g2.z + g2.y) - (g2.w + g2.x),
                                                  (g2.x + g2.y) - (g2.w + g2.z)) * 0.5f) * 26.5f) / scale);

                float2 fbm_pos = (jittered.xz * 0.5f) + (rock_t * 20.0f).xx;
                float fbm_sum = 0.0f;
                float amp = 0.5f;
                for (int oct = 0; oct < 1; ++oct)
                {
                    fbm_sum += amp * (-1.0f + 2.0f * value_noise(fbm_pos));
                    amp *= 0.55f;
                    fbm_pos = mul(float2x2(float2(0.8f, -0.6f), float2(0.6f, 0.8f)), fbm_pos * 1.9f);
                }

                displaced = tilted
                    + (normal * lerp(-0.2f, 1.0f - smoothstep(4.0f, 7.0f, grad2_len),
                                     clamp((fbm_sum + 1.0f) * 0.5f, 0.0f, 1.0f))
                       * smoothstep(0.0f, 0.3f, rock_t) * grad1_len) * 0.3f
                    + (normal * r_y2);
            }
            else
            {
                displaced = tilted;
            }
        }
        else
        {
            displaced = result;
        }

        vert.position = displaced;
        __alias_vertices[vertex_id] = vert;
    }

    
    Triangle tri = __alias_triangles[gl_GlobalInvocationID.x];
    float3 p0 = __alias_vertices[triangle_base].position;
    float3 p1 = __alias_vertices[triangle_base + 1u].position;
    float3 p2 = __alias_vertices[triangle_base + 2u].position;
    tri.normal = normalize(cross(p2 - p0, p1 - p0));
    __alias_triangles[gl_GlobalInvocationID.x] = tri;
}
