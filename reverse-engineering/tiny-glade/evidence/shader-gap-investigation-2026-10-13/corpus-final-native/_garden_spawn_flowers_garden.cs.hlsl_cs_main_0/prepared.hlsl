#define TGV_0 1
#line 1 "shaders/garden/spawn_flowers_garden.cs.hlsl"


#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))


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
#line 6 "shaders/garden/spawn_flowers_garden.cs.hlsl"

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
#line 7 "shaders/garden/spawn_flowers_garden.cs.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 8 "shaders/garden/spawn_flowers_garden.cs.hlsl"

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
#line 9 "shaders/garden/spawn_flowers_garden.cs.hlsl"

#line 1 "shaders/inc/draw_indirect.hlsl"


#ifndef TINYGLADE_INC_DRAW_INDIRECT_HLSL
#define TINYGLADE_INC_DRAW_INDIRECT_HLSL

#line 1 "shaders/inc/draw_indirect_types.hlsl"


#ifndef TINYGLADE_INC_DRAW_INDIRECT_TYPES_HLSL
#define TINYGLADE_INC_DRAW_INDIRECT_TYPES_HLSL



struct DrawIndexedIndirectCommand {
    uint count;
    uint instanceCount;
    uint firstIndex;
    uint vertexOffset;
    uint firstInstance;
};


struct DrawIndirectCommand {
    uint vertexCount;
    uint instanceCount;
    uint firstVertex;
    uint firstInstance;
};

#endif 
#line 6 "shaders/inc/draw_indirect.hlsl"



static const uint DRAW_INDEXED_INDIRECT_STRIDE_BYTES = 20;




uint increment_draw_indexed_indirect_instance_count(
    RWByteAddressBuffer draw_commands,
    uint command_base_offset,
    uint cmd_index,
    uint n)
{
    uint previous;
    draw_commands.InterlockedAdd((command_base_offset + cmd_index) * 20u + 4u, n, previous);
    return previous;
}



uint reserve_draw_indexed_indirect_instances(
    RWByteAddressBuffer draw_commands,
    uint command_base_offset,
    uint cmd_index,
    uint n)
{
    return increment_draw_indexed_indirect_instance_count(draw_commands, command_base_offset, cmd_index, n);
}

#endif 
#line 10 "shaders/garden/spawn_flowers_garden.cs.hlsl"

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
#line 11 "shaders/garden/spawn_flowers_garden.cs.hlsl"

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
#line 13 "shaders/garden/spawn_flowers_garden.cs.hlsl"

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
#line 14 "shaders/garden/spawn_flowers_garden.cs.hlsl"

#line 1 "shaders/garden/defs.inc.hlsl"


#ifndef TINYGLADE_GARDEN_DEFS_HLSL
#define TINYGLADE_GARDEN_DEFS_HLSL



struct BushXform
{
    Affine3Packed transform;
};

struct FlowerXform
{
    Affine3Packed transform;
    float3 color;
    uint flower_type;
};

#endif 
#line 15 "shaders/garden/spawn_flowers_garden.cs.hlsl"

#line 1 "shaders/garden/placement_utils.inc.hlsl"


#ifndef TINYGLADE_GARDEN_PLACEMENT_UTILS_HLSL
#define TINYGLADE_GARDEN_PLACEMENT_UTILS_HLSL





struct Flower03Xform
{
    float4 position;
};


float2 grid_world_pos(float2 id, int2 grid_dims)
{
    return ((float2(id) + 0.5f) / float2(grid_dims) - 0.5f) * 130.0f;
}


float2 placement_uv(float2 world_xz)
{
    return world_xz * 0.0076923076994717121124267578125f + 0.5f;
}



float4x4 rotate_axis(float3 axis, float angle)
{
    float c = cos(angle);
    float s = sin(angle);
    float omc = 1.0f - c;
    float x = axis.x;
    float y = axis.y;
    float z = axis.z;
    return transpose(float4x4(
        float4(omc * x * x + c, omc * x * y - s * z, omc * x * z + s * y, 0.0f),
        float4(omc * x * y + s * z, omc * y * y + c, omc * y * z - s * x, 0.0f),
        float4(omc * x * z - s * y, omc * y * z + s * x, omc * z * z + c, 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f)));
}


float4x4 rotate_y(float angle)
{
    return rotate_axis(float3(0.0f, 1.0f, 0.0f), angle);
}



float4x4 placement_transform(float3 pos, float3 scale, float yaw)
{
    float4x4 identity = float4x4(
        float4(1.0f, 0.0f, 0.0f, 0.0f),
        float4(0.0f, 1.0f, 0.0f, 0.0f),
        float4(0.0f, 0.0f, 1.0f, 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f));
    float4x4 s = float4x4(
        float4(scale.x, 0.0f, 0.0f, 0.0f),
        float4(0.0f, scale.y, 0.0f, 0.0f),
        float4(0.0f, 0.0f, scale.z, 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f));
    return mul(mul(affine_translation(pos), mul(identity, s)), rotate_y(yaw));
}


void store_flower03(RWByteAddressBuffer buffer, uint base, float4 position)
{
    buffer.Store<float4>(base * 16u + 0u, position);
}

#endif 




bool placement_excluded(
    Texture2D<float4> terrain_heightmap,
    uint2 terrain_heightmap_dims,
    Texture2D<float4> path_mask,
    uint2 path_mask_dims,
    Texture2D<float4> rocky_terrain,
    Texture2D<float4> water_mask,
    Texture2D<float4> foliage_exclusion_mask,
    SamplerState sampler_llb,
    SamplerState sampler_default,
    float2 uv)
{
    uint2 px = uint2(uv * float2(path_mask_dims));
    if (path_mask.Load(int3(px, 0u)).x > 0.2f)
        return true;

    float4 g = terrain_heightmap.GatherRed(sampler_default, uv, int2(0, 0));
    float2 grad = (float2((g.z + g.y) - (g.w + g.x), (g.x + g.y) - (g.w + g.z)) * 0.5f) * 26.5f;
    grad /= (130.0f / max(float2(terrain_heightmap_dims), 1.0e-6f));
    if (rocky_terrain.Load(int3(px, 0u)).x > 0.0f || length(grad) > 0.9f)
        return true;

    if (water_mask.Load(int3(px, 0u)).y > 0.3f)
        return true;

    if (foliage_exclusion_mask.SampleLevel(sampler_llb, uv, 0.0f).x > 0.0f)
        return true;

    return false;
}



float band_falloff_garden(float d, float inv_half_width)
{
    float t = d * inv_half_width;
    return 1.0f - (t * t) * (3.0f - d * (2.0f * inv_half_width));
}
#line 16 "shaders/garden/spawn_flowers_garden.cs.hlsl"


cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 garden_mask_dims;
    uint2 buildings_mask_non_blurred_dims;
    int2 grid_dims;
    uint2 path_mask_dims;
    uint2 rocky_terrain_dims;
    uint2 water_mask_dims;
    uint2 foliage_exclusion_mask_dims;
};

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);
Texture2D<float4> garden_mask : register(t17, space0);
Texture2D<float4> buildings_mask_non_blurred : register(t18, space0);
Texture2D<float4> path_mask : register(t20, space0);
Texture2D<float4> rocky_terrain : register(t21, space0);
Texture2D<float4> water_mask : register(t22, space0);
Texture2D<float4> foliage_exclusion_mask : register(t23, space0);
RWStructuredBuffer<FlowerXform> __alias_flower_buffer : register(u19, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 id : SV_DispatchThreadID)
{
    float2 base = ((float2(id.xy) + 0.5f) / float2(grid_dims) - 0.5f) * 130.0f;
    float rnd0 = rand01(hash(id.x * id.y + id.x + 100u));
    float2 p0 = base + lerp(-0.75f, 0.75f, rnd0).xx;
    uint seed1 = uint((rnd0 * 100.0f) - 34.0f);
    float2 p1 = base + lerp(-0.75f, 0.75f, rand01(hash(seed1))).xx;

    int2 gid = int2(id.xy);

    
    {
        float2 uv = placement_uv(p0);
        if (placement_excluded(terrain_heightmap, terrain_heightmap_dims, path_mask, path_mask_dims,
                               rocky_terrain, water_mask, foliage_exclusion_mask,
                               sampler_llb, sampler_default, uv))
            return;
        if (buildings_mask_non_blurred.SampleLevel(sampler_default, uv, 0.0f).x > 0.5f)
            return;

        float gy = garden_mask.SampleLevel(sampler_default, uv, 0.0f).y;
        float band = band_falloff_garden(abs(gy - 0.6f), 5.0f);
        bool full = smoothstep(0.9f, 1.0f, garden_mask.SampleLevel(sampler_default, uv, 0.0f).y) > 0.0f;

        if (perlin(p0 * 0.5f) * 2.0f > 0.6f && gy > 0.425f)
        {
            float3 pos = float3(p0.x, full ? 1.1f : 0.5f, p0.y);
            float fs = lerp(0.8f, 1.2f, rand01(hash(gid.x + gid.y)));
            float3 scale = fs.xxx * lerp(0.5f, 1.0f, band + (full ? 1.5f : 0.0f));
            float3 s3 = float3(min(scale.x, 0.75f), (scale.y * 1.2f) * fs, min(scale.z, 0.75f)) * 1.5f;

            float4x4 m = placement_transform(pos, s3, 3.14f * rand01(hash(uint(gid.x * gid.y + gid.x))));

            uint base;
            InterlockedAdd(__alias_draw_commands[11u].instanceCount, 1u, base);
            float h = get_terrain_height(terrain_heightmap, sampler_default, p0);
            Affine3Packed packed = pack_affine(m);
            packed.data[11] += h;
            FlowerXform f;
            f.transform = packed;
            f.color = full ? float3(1.0f, 1.0f, 1.0f) : float3(-1.0f, -1.0f, -1.0f);
            f.flower_type = 0u;
            __alias_flower_buffer[base] = f;
        }
    }

    
    {
        float2 uv = placement_uv(p1);
        if (placement_excluded(terrain_heightmap, terrain_heightmap_dims, path_mask, path_mask_dims,
                               rocky_terrain, water_mask, foliage_exclusion_mask,
                               sampler_llb, sampler_default, uv))
            return;
        if (buildings_mask_non_blurred.SampleLevel(sampler_default, uv, 0.0f).x > 0.5f)
            return;

        float gy = garden_mask.SampleLevel(sampler_default, uv, 0.0f).y;
        float band = band_falloff_garden(abs(gy - 0.65f), 3.3333332538604736328125f);
        bool full = smoothstep(0.9f, 1.0f, garden_mask.SampleLevel(sampler_default, uv, 0.0f).y) > 0.0f;

        if (perlin((p1 * 0.5f) + 100.0f) * 2.0f > 0.6f && gy > 0.425f)
        {
            float3 pos = float3(p1.x, full ? 1.1f : 0.5f, p1.y);
            float fs = lerp(0.8f, 1.2f, rand01(hash(gid.x + gid.y)));
            float3 scale = fs.xxx * lerp(0.5f, 1.0f, band + (full ? 1.5f : 0.0f));
            float3 s3 = float3(min(scale.x, 0.75f), (scale.y * 1.2f) * fs, min(scale.z, 0.75f)) * 1.5f;

            float4x4 m = placement_transform(pos, s3, 3.14f * rand01(hash(uint(gid.x * gid.y + gid.x))));

            uint base;
            InterlockedAdd(__alias_draw_commands[11u].instanceCount, 1u, base);
            float h = get_terrain_height(terrain_heightmap, sampler_default, p1);
            Affine3Packed packed = pack_affine(m);
            packed.data[11] += h;
            FlowerXform f;
            f.transform = packed;
            f.color = float3(-1.0f, -1.0f, -1.0f);
            f.flower_type = 1u;
            __alias_flower_buffer[base] = f;
        }
    }
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct DrawIndexedIndirectCommand
{
    uint count;
    uint instanceCount;
    uint firstIndex;
    uint vertexOffset;
    uint firstInstance;
};

struct Affine3Packed
{
    float data[12];
};

struct FlowerXform
{
    Affine3Packed transform;
    float3 color;
    uint flower_type;
};

static uint _119;

RWByteAddressBuffer __alias_draw_commands : register(u13, space0);
cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset : packoffset(c0);
    uint2 terrain_heightmap_dims : packoffset(c0.y);
    uint2 visual_terrain_heightmap_dims : packoffset(c1);
    uint2 garden_mask_dims : packoffset(c1.z);
    uint2 buildings_mask_non_blurred_dims : packoffset(c2);
    int2 grid_dims : packoffset(c2.z);
    uint2 path_mask_dims : packoffset(c3);
    uint2 rocky_terrain_dims : packoffset(c3.z);
    uint2 water_mask_dims : packoffset(c4);
    uint2 foliage_exclusion_mask_dims : packoffset(c4.z);
};

RWByteAddressBuffer __alias_flower_buffer : register(u19, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t15, space0);
Texture2D<float4> garden_mask : register(t17, space0);
Texture2D<float4> buildings_mask_non_blurred : register(t18, space0);
Texture2D<float4> path_mask : register(t20, space0);
Texture2D<float4> rocky_terrain : register(t21, space0);
Texture2D<float4> water_mask : register(t22, space0);
Texture2D<float4> foliage_exclusion_mask : register(t23, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    float2 _204;
    float2 _228;
    float2 _152 = (((float2(gl_GlobalInvocationID.xy) + 0.5f.xx) / float2(grid_dims)) - 0.5f.xx) * 130.0f.xx;
    uint _157 = ((gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) + gl_GlobalInvocationID.x) + 100u;
    uint _159 = _157 + (_157 << 10u);
    uint _161 = _159 ^ (_159 >> 6u);
    uint _163 = _161 + (_161 << 3u);
    uint _165 = _163 ^ (_163 >> 11u);
    float _171 = asfloat(((_165 + (_165 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float2 _174 = _152 + lerp(-0.75f, 0.75f, _171).xx;
    uint _177 = uint((_171 * 100.0f) - 34.0f);
    uint _179 = _177 + (_177 << 10u);
    uint _181 = _179 ^ (_179 >> 6u);
    uint _183 = _181 + (_181 << 3u);
    uint _185 = _183 ^ (_183 >> 11u);
    float2 _194 = _152 + lerp(-0.75f, 0.75f, asfloat(((_185 + (_185 << 15u)) & 8388607u) | 1065353216u) - 1.0f).xx;
    int2 _195 = int2(gl_GlobalInvocationID.xy);
    uint _602;
    uint _603;
    bool _604;
    do
    {
        float2 _201;
        bool _267;
        do
        {
            _201 = (_174 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
            _204 = float2(path_mask_dims);
            uint2 _206 = uint2(_201 * _204);
            float4 _224 = terrain_heightmap.GatherRed(sampler_default, _201, int2(0, 0));
            _228 = 130.0f.xx / float2(terrain_heightmap_dims);
            float _229 = _224.z;
            float _230 = _224.y;
            float _232 = _224.w;
            float _233 = _224.x;
            bool _254;
            if (!(path_mask.Load(int3(_206, 0u)).x > 0.20000000298023223876953125f))
            {
                _254 = (int(rocky_terrain.Load(int3(_206, 0u)).x > 0.0f) & int(length(((float2((_229 + _230) - (_232 + _233), (_233 + _230) - (_232 + _229)) * 0.5f.xx) * 26.5f) / _228) > 0.89999997615814208984375f)) != 0;
            }
            else
            {
                _254 = true;
            }
            bool _259;
            if (!_254)
            {
                _259 = water_mask.Load(int3(_206, 0u)).y > 0.300000011920928955078125f;
            }
            else
            {
                _259 = true;
            }
            bool _264;
            if (!_259)
            {
                _264 = foliage_exclusion_mask.SampleLevel(sampler_llb, _201, 0.0f).x > 0.0f;
            }
            else
            {
                _264 = true;
            }
            if (_264)
            {
                _267 = true;
                break;
            }
            _267 = false;
            break;
        } while(false);
        if (_267)
        {
            _602 = _119;
            _603 = _119;
            _604 = false;
            break;
        }
        if (buildings_mask_non_blurred.SampleLevel(sampler_default, _201, 0.0f).x > 0.5f)
        {
            _602 = _119;
            _603 = _119;
            _604 = false;
            break;
        }
        float4 _279 = garden_mask.SampleLevel(sampler_default, _201, 0.0f);
        float _280 = _279.y;
        float _294;
        do
        {
            float _284 = abs(_280 - 0.60000002384185791015625f);
            if (_284 > 0.20000000298023223876953125f)
            {
                _294 = 0.0f;
                break;
            }
            float _288 = _284 * 5.0f;
            _294 = 1.0f - ((_288 * _288) * (3.0f - (_284 * 10.0f)));
            break;
        } while(false);
        float _298 = smoothstep(0.89999997615814208984375f, 1.0f, garden_mask.SampleLevel(sampler_default, _201, 0.0f).y);
        float2 _301 = _174 * 0.5f;
        int2 _303 = int2(floor(_301));
        float2 _304 = frac(_301);
        uint2 _305 = uint2(_303);
        uint _307 = _305.y;
        uint _309 = _307 + (_307 << 10u);
        uint _311 = _309 ^ (_309 >> 6u);
        uint _313 = _311 + (_311 << 3u);
        uint _315 = _313 ^ (_313 >> 11u);
        uint _321 = (((_305.x * 1664525u) + (_315 + (_315 << 15u))) + 1013904223u) * 1664525u;
        uint _323 = _321 ^ (_321 >> 11u);
        uint _326 = _323 ^ ((_323 << 7u) & 2636928640u);
        uint _329 = _326 ^ ((_326 << 15u) & 4022730752u);
        uint2 _337 = uint2(_303 + int2(1, 0));
        uint _339 = _337.y;
        uint _341 = _339 + (_339 << 10u);
        uint _343 = _341 ^ (_341 >> 6u);
        uint _345 = _343 + (_343 << 3u);
        uint _347 = _345 ^ (_345 >> 11u);
        uint _353 = (((_337.x * 1664525u) + (_347 + (_347 << 15u))) + 1013904223u) * 1664525u;
        uint _355 = _353 ^ (_353 >> 11u);
        uint _358 = _355 ^ ((_355 << 7u) & 2636928640u);
        uint _361 = _358 ^ ((_358 << 15u) & 4022730752u);
        uint2 _369 = uint2(_303 + int2(0, 1));
        uint _371 = _369.y;
        uint _373 = _371 + (_371 << 10u);
        uint _375 = _373 ^ (_373 >> 6u);
        uint _377 = _375 + (_375 << 3u);
        uint _379 = _377 ^ (_377 >> 11u);
        uint _385 = (((_369.x * 1664525u) + (_379 + (_379 << 15u))) + 1013904223u) * 1664525u;
        uint _387 = _385 ^ (_385 >> 11u);
        uint _390 = _387 ^ ((_387 << 7u) & 2636928640u);
        uint _393 = _390 ^ ((_390 << 15u) & 4022730752u);
        uint2 _401 = uint2(_303 + int2(1, 1));
        uint _403 = _401.y;
        uint _405 = _403 + (_403 << 10u);
        uint _407 = _405 ^ (_405 >> 6u);
        uint _409 = _407 + (_407 << 3u);
        uint _411 = _409 ^ (_409 >> 11u);
        uint _417 = (((_401.x * 1664525u) + (_411 + (_411 << 15u))) + 1013904223u) * 1664525u;
        uint _419 = _417 ^ (_417 >> 11u);
        uint _422 = _419 ^ ((_419 << 7u) & 2636928640u);
        uint _425 = _422 ^ ((_422 << 15u) & 4022730752u);
        float2 _435 = (_304 * _304) * (3.0f.xx - (_304 * 2.0f));
        float _436 = _435.x;
        float _437 = 1.0f - _436;
        float _439 = _435.y;
        if ((int((((((1.0f - _439) * (((asfloat(((_329 ^ (_329 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _437) + ((asfloat(((_361 ^ (_361 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _436))) + (((asfloat(((_393 ^ (_393 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _437) * _439)) + (((asfloat(((_425 ^ (_425 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _436) * _439)) * 2.0f) > 0.60000002384185791015625f) & int(_280 > 0.425000011920928955078125f)) != 0)
        {
            float3 _459 = _174.xxy;
            _459.y = 0.5f;
            int _461 = _195.x;
            int _462 = _195.y;
            uint _464 = uint(_461 + _462);
            uint _466 = _464 + (_464 << 10u);
            uint _468 = _466 ^ (_466 >> 6u);
            uint _470 = _468 + (_468 << 3u);
            uint _472 = _470 ^ (_470 >> 11u);
            float _479 = lerp(0.800000011920928955078125f, 1.2000000476837158203125f, asfloat(((_472 + (_472 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
            float3 _482 = _479.xxx * lerp(0.5f, 1.0f, _294 + (_298 * 1.5f));
            float3 _491 = float3(min(_482.x, 0.75f), (_482.y * 1.2000000476837158203125f) * _479, min(_482.z, 0.75f)) * 1.5f;
            bool _492 = _298 > 0.0f;
            float3 _496;
            if (_492)
            {
                float3 _495 = _459;
                _495.y = 1.10000002384185791015625f;
                _496 = _495;
            }
            else
            {
                _496 = _459;
            }
            uint _515 = uint((_461 * _462) + _461);
            uint _517 = _515 + (_515 << 10u);
            uint _519 = _517 ^ (_517 >> 6u);
            uint _521 = _519 + (_519 << 3u);
            uint _523 = _521 ^ (_521 >> 11u);
            float _530 = 3.1400001049041748046875f * (asfloat(((_523 + (_523 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
            float3 _531 = normalize(float3(0.0f, 1.0f, 0.0f));
            float _532 = sin(_530);
            float _533 = cos(_530);
            float _534 = 1.0f - _533;
            float _535 = _531.x;
            float _536 = _534 * _535;
            float _539 = _531.y;
            float _540 = _536 * _539;
            float _541 = _531.z;
            float _542 = _541 * _532;
            float _544 = _534 * _541;
            float _545 = _544 * _535;
            float _546 = _539 * _532;
            float _549 = _534 * _539;
            float _552 = _549 * _541;
            float _553 = _535 * _532;
            float4x4 _564 = mul(mul(float4x4(float4(1.0f, 0.0f, 0.0f, _496.x), float4(0.0f, 1.0f, 0.0f, _496.y), float4(0.0f, 0.0f, 1.0f, _496.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(float4x4(float4(1.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 1.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4x4(float4(_491.x, 0.0f, 0.0f, 0.0f), float4(0.0f, _491.y, 0.0f, 0.0f), float4(0.0f, 0.0f, _491.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), transpose(float4x4(float4((_536 * _535) + _533, _540 - _542, _545 + _546, 0.0f), float4(_540 + _542, (_549 * _539) + _533, _552 - _553, 0.0f), float4(_545 - _546, _552 + _553, (_544 * _541) + _533, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
            bool3 _592 = _492.xxx;
            uint _595;
            __alias_draw_commands.InterlockedAdd(224, 1u, _595);
            uint _597 = uint(int(_595));
            float _599[12] = { _564[0].x, _564[1].x, _564[2].x, _564[0].y, _564[1].y, _564[2].y, _564[0].z, _564[1].z, _564[2].z, _564[0].w, _564[1].w + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (mul(transpose(float4x3(float3(_564[0].x, _564[1].x, _564[2].x), float3(_564[0].y, _564[1].y, _564[2].y), float3(_564[0].z, _564[1].z, _564[2].z), float3(_564[0].w, _564[1].w, _564[2].w))), float4(0.0f, 0.0f, 0.0f, 1.0f)).xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x), _564[2].w };
            Affine3Packed _600 = { _599 };
            FlowerXform _601 = { _600, float3(_592.x ? 1.0f.xxx.x : (-1.0f).xxx.x, _592.y ? 1.0f.xxx.y : (-1.0f).xxx.y, _592.z ? 1.0f.xxx.z : (-1.0f).xxx.z), 0u };
            [unroll]
            for (int _4ident = 0; _4ident < 12; _4ident++)
            {
                __alias_flower_buffer.Store<float>(_4ident * 4 + _597 * 64 + 0, _601.transform.data[_4ident]);
            }
            __alias_flower_buffer.Store<float3>(_597 * 64 + 48, _601.color);
            __alias_flower_buffer.Store<uint>(_597 * 64 + 60, _601.flower_type);
        }
        _602 = 1013904223u;
        _603 = 1664525u;
        _604 = true;
        break;
    } while(false);
    do
    {
        float2 _610;
        bool _662;
        do
        {
            _610 = (_194 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
            uint2 _612 = uint2(_610 * _204);
            float4 _623 = terrain_heightmap.GatherRed(sampler_default, _610, int2(0, 0));
            float _624 = _623.z;
            float _625 = _623.y;
            float _627 = _623.w;
            float _628 = _623.x;
            bool _649;
            if (!(path_mask.Load(int3(_612, 0u)).x > 0.20000000298023223876953125f))
            {
                _649 = (int(rocky_terrain.Load(int3(_612, 0u)).x > 0.0f) & int(length(((float2((_624 + _625) - (_627 + _628), (_628 + _625) - (_627 + _624)) * 0.5f.xx) * 26.5f) / _228) > 0.89999997615814208984375f)) != 0;
            }
            else
            {
                _649 = true;
            }
            bool _654;
            if (!_649)
            {
                _654 = water_mask.Load(int3(_612, 0u)).y > 0.300000011920928955078125f;
            }
            else
            {
                _654 = true;
            }
            bool _659;
            if (!_654)
            {
                _659 = foliage_exclusion_mask.SampleLevel(sampler_llb, _610, 0.0f).x > 0.0f;
            }
            else
            {
                _659 = true;
            }
            if (_659)
            {
                _662 = true;
                break;
            }
            _662 = false;
            break;
        } while(false);
        if (_662)
        {
            break;
        }
        if (buildings_mask_non_blurred.SampleLevel(sampler_default, _610, 0.0f).x > 0.5f)
        {
            break;
        }
        float4 _674 = garden_mask.SampleLevel(sampler_default, _610, 0.0f);
        float _675 = _674.y;
        float _689;
        do
        {
            float _679 = abs(_675 - 0.650000035762786865234375f);
            if (_679 > 0.300000011920928955078125f)
            {
                _689 = 0.0f;
                break;
            }
            float _683 = _679 * 3.3333332538604736328125f;
            _689 = 1.0f - ((_683 * _683) * (3.0f - (_679 * 6.666666507720947265625f)));
            break;
        } while(false);
        float _693 = smoothstep(0.89999997615814208984375f, 1.0f, garden_mask.SampleLevel(sampler_default, _610, 0.0f).y);
        float2 _697 = (_194 * 0.5f) + 100.0f.xx;
        int2 _699 = int2(floor(_697));
        float2 _700 = frac(_697);
        uint2 _701 = uint2(_699);
        uint _703 = _701.y;
        uint _705 = _703 + (_703 << 10u);
        uint _707 = _705 ^ (_705 >> 6u);
        uint _709 = _707 + (_707 << 3u);
        uint _711 = _709 ^ (_709 >> 11u);
        bool _714 = _604 ? _604 : true;
        uint _715 = _604 ? _603 : 1664525u;
        uint _716 = _604 ? _602 : 1013904223u;
        uint _720 = (((_701.x * _715) + (_711 + (_711 << 15u))) + _716) * _715;
        uint _722 = _720 ^ (_720 >> 11u);
        uint _725 = _722 ^ ((_722 << 7u) & 2636928640u);
        uint _728 = _725 ^ ((_725 << 15u) & 4022730752u);
        uint2 _736 = uint2(_699 + int2(1, 0));
        uint _738 = _736.y;
        uint _740 = _738 + (_738 << 10u);
        uint _742 = _740 ^ (_740 >> 6u);
        uint _744 = _742 + (_742 << 3u);
        uint _746 = _744 ^ (_744 >> 11u);
        bool _749 = _714 ? _714 : true;
        uint _750 = _714 ? _715 : 1664525u;
        uint _751 = _714 ? _716 : 1013904223u;
        uint _755 = (((_736.x * _750) + (_746 + (_746 << 15u))) + _751) * _750;
        uint _757 = _755 ^ (_755 >> 11u);
        uint _760 = _757 ^ ((_757 << 7u) & 2636928640u);
        uint _763 = _760 ^ ((_760 << 15u) & 4022730752u);
        uint2 _771 = uint2(_699 + int2(0, 1));
        uint _773 = _771.y;
        uint _775 = _773 + (_773 << 10u);
        uint _777 = _775 ^ (_775 >> 6u);
        uint _779 = _777 + (_777 << 3u);
        uint _781 = _779 ^ (_779 >> 11u);
        bool _784 = _749 ? _749 : true;
        uint _785 = _749 ? _750 : 1664525u;
        uint _786 = _749 ? _751 : 1013904223u;
        uint _790 = (((_771.x * _785) + (_781 + (_781 << 15u))) + _786) * _785;
        uint _792 = _790 ^ (_790 >> 11u);
        uint _795 = _792 ^ ((_792 << 7u) & 2636928640u);
        uint _798 = _795 ^ ((_795 << 15u) & 4022730752u);
        uint2 _806 = uint2(_699 + int2(1, 1));
        uint _808 = _806.y;
        uint _810 = _808 + (_808 << 10u);
        uint _812 = _810 ^ (_810 >> 6u);
        uint _814 = _812 + (_812 << 3u);
        uint _816 = _814 ^ (_814 >> 11u);
        uint _819 = _784 ? _785 : 1664525u;
        uint _824 = (((_806.x * _819) + (_816 + (_816 << 15u))) + (_784 ? _786 : 1013904223u)) * _819;
        uint _826 = _824 ^ (_824 >> 11u);
        uint _829 = _826 ^ ((_826 << 7u) & 2636928640u);
        uint _832 = _829 ^ ((_829 << 15u) & 4022730752u);
        float2 _842 = (_700 * _700) * (3.0f.xx - (_700 * 2.0f));
        float _843 = _842.x;
        float _844 = 1.0f - _843;
        float _846 = _842.y;
        if ((int((((((1.0f - _846) * (((asfloat(((_728 ^ (_728 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _844) + ((asfloat(((_763 ^ (_763 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _843))) + (((asfloat(((_798 ^ (_798 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _844) * _846)) + (((asfloat(((_832 ^ (_832 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _843) * _846)) * 2.0f) > 0.60000002384185791015625f) & int(_675 > 0.425000011920928955078125f)) != 0)
        {
            float3 _866 = _194.xxy;
            _866.y = 0.5f;
            int _868 = _195.x;
            int _869 = _195.y;
            uint _871 = uint(_868 + _869);
            uint _873 = _871 + (_871 << 10u);
            uint _875 = _873 ^ (_873 >> 6u);
            uint _877 = _875 + (_875 << 3u);
            uint _879 = _877 ^ (_877 >> 11u);
            float _886 = lerp(0.800000011920928955078125f, 1.2000000476837158203125f, asfloat(((_879 + (_879 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
            float3 _889 = _886.xxx * lerp(0.5f, 1.0f, _689 + (_693 * 1.5f));
            float3 _898 = float3(min(_889.x, 0.75f), (_889.y * 1.2000000476837158203125f) * _886, min(_889.z, 0.75f)) * 1.5f;
            float3 _903;
            if (_693 > 0.0f)
            {
                float3 _902 = _866;
                _902.y = 1.10000002384185791015625f;
                _903 = _902;
            }
            else
            {
                _903 = _866;
            }
            uint _922 = uint((_868 * _869) + _868);
            uint _924 = _922 + (_922 << 10u);
            uint _926 = _924 ^ (_924 >> 6u);
            uint _928 = _926 + (_926 << 3u);
            uint _930 = _928 ^ (_928 >> 11u);
            float _937 = 3.1400001049041748046875f * (asfloat(((_930 + (_930 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
            float3 _938 = normalize(float3(0.0f, 1.0f, 0.0f));
            float _939 = sin(_937);
            float _940 = cos(_937);
            float _941 = 1.0f - _940;
            float _942 = _938.x;
            float _943 = _941 * _942;
            float _946 = _938.y;
            float _947 = _943 * _946;
            float _948 = _938.z;
            float _949 = _948 * _939;
            float _951 = _941 * _948;
            float _952 = _951 * _942;
            float _953 = _946 * _939;
            float _956 = _941 * _946;
            float _959 = _956 * _948;
            float _960 = _942 * _939;
            float4x4 _971 = mul(mul(float4x4(float4(1.0f, 0.0f, 0.0f, _903.x), float4(0.0f, 1.0f, 0.0f, _903.y), float4(0.0f, 0.0f, 1.0f, _903.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(float4x4(float4(1.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 1.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4x4(float4(_898.x, 0.0f, 0.0f, 0.0f), float4(0.0f, _898.y, 0.0f, 0.0f), float4(0.0f, 0.0f, _898.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), transpose(float4x4(float4((_943 * _942) + _940, _947 - _949, _952 + _953, 0.0f), float4(_947 + _949, (_956 * _946) + _940, _959 - _960, 0.0f), float4(_952 - _953, _959 + _960, (_951 * _948) + _940, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
            uint _1000;
            __alias_draw_commands.InterlockedAdd(224, 1u, _1000);
            uint _1002 = uint(int(_1000));
            float _1004[12] = { _971[0].x, _971[1].x, _971[2].x, _971[0].y, _971[1].y, _971[2].y, _971[0].z, _971[1].z, _971[2].z, _971[0].w, _971[1].w + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (mul(transpose(float4x3(float3(_971[0].x, _971[1].x, _971[2].x), float3(_971[0].y, _971[1].y, _971[2].y), float3(_971[0].z, _971[1].z, _971[2].z), float3(_971[0].w, _971[1].w, _971[2].w))), float4(0.0f, 0.0f, 0.0f, 1.0f)).xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x), _971[2].w };
            Affine3Packed _1005 = { _1004 };
            FlowerXform _1006 = { _1005, (-1.0f).xxx, 1u };
            [unroll]
            for (int _5ident = 0; _5ident < 12; _5ident++)
            {
                __alias_flower_buffer.Store<float>(_5ident * 4 + _1002 * 64 + 0, _1006.transform.data[_5ident]);
            }
            __alias_flower_buffer.Store<float3>(_1002 * 64 + 48, _1006.color);
            __alias_flower_buffer.Store<uint>(_1002 * 64 + 60, _1006.flower_type);
        }
        break;
    } while(false);
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
