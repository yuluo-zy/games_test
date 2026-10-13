#define TGV_0 1
#line 1 "shaders/garden/spawn_bushes_garden.cs.hlsl"


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
#line 6 "shaders/garden/spawn_bushes_garden.cs.hlsl"

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
#line 7 "shaders/garden/spawn_bushes_garden.cs.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 8 "shaders/garden/spawn_bushes_garden.cs.hlsl"

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
#line 9 "shaders/garden/spawn_bushes_garden.cs.hlsl"

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
#line 10 "shaders/garden/spawn_bushes_garden.cs.hlsl"

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
#line 11 "shaders/garden/spawn_bushes_garden.cs.hlsl"

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
#line 13 "shaders/garden/spawn_bushes_garden.cs.hlsl"

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
#line 14 "shaders/garden/spawn_bushes_garden.cs.hlsl"

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
#line 15 "shaders/garden/spawn_bushes_garden.cs.hlsl"

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
#line 16 "shaders/garden/spawn_bushes_garden.cs.hlsl"


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
Texture2D<float4> path_mask : register(t21, space0);
Texture2D<float4> rocky_terrain : register(t22, space0);
Texture2D<float4> water_mask : register(t23, space0);
Texture2D<float4> foliage_exclusion_mask : register(t24, space0);
RWStructuredBuffer<BushXform> __alias_bush_buffer : register(u19, space0);
RWStructuredBuffer<FlowerXform> __alias_bush_flowers : register(u20, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 id : SV_DispatchThreadID)
{
    float rnd = rand01(hash(id.x * id.y + id.x));
    float2 world_pos = ((float2(id.xy) + 0.5f) / float2(grid_dims) - 0.5f) * 130.0f + lerp(-1.0f, 1.0f, rnd).xx;

    float2 uv = placement_uv(world_pos);
    if (placement_excluded(terrain_heightmap, terrain_heightmap_dims, path_mask, path_mask_dims,
                           rocky_terrain, water_mask, foliage_exclusion_mask,
                           sampler_llb, sampler_default, uv))
        return;

    if (buildings_mask_non_blurred.SampleLevel(sampler_default, uv, 0.0f).x > 0.5f)
        return;

    float g = garden_mask.SampleLevel(sampler_default, uv, 0.0f).y + 0.005f;
    if (g > 0.5f)
    {
        float height = get_terrain_height(terrain_heightmap, sampler_default, world_pos);
        float3 pos = float3(world_pos.x, height, world_pos.y);

        float bush_scale = lerp(0.8f, 1.2f, rand01(hash(id.x + id.y)));
        float3 scale = bush_scale.xxx * lerp(0.3f, 0.8f, sqrt(clamp(
            smoothstep(0.45f, 1.0f, g) + smoothstep(0.6f, 1.0f, g) * 0.2f, 0.0f, 1.0f)));
        float4x4 s = float4x4(
            float4(min(scale.x, 0.75f), 0.0f, 0.0f, 0.0f),
            float4(0.0f, min(scale.y * bush_scale, 1.2f), 0.0f, 0.0f),
            float4(0.0f, 0.0f, min(scale.z, 0.75f), 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f));

        float4x4 identity = float4x4(
            float4(1.0f, 0.0f, 0.0f, 0.0f),
            float4(0.0f, 1.0f, 0.0f, 0.0f),
            float4(0.0f, 0.0f, 1.0f, 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f));

        float4x4 m = mul(mul(
            affine_translation(pos),
            mul(identity, s)),
            rotate_y(3.14f * rnd));

        uint bush_base;
        InterlockedAdd(__alias_draw_commands[10u].instanceCount, 1u, bush_base);
        __alias_bush_buffer[bush_base] = BushXform(pack_affine(m));

        if (rand01(hash((id.x * 2u) + (id.y / 5u) + 1233u)) > 0.3f && g > 0.8f)
        {
            uint flower_base;
            InterlockedAdd(__alias_draw_commands[11u].instanceCount, 1u, flower_base);
            FlowerXform ff;
            ff.transform = pack_affine(m);
            ff.color = float3(-1.0f, -1.0f, -1.0f);
            ff.flower_type = 2u;
            __alias_bush_flowers[flower_base] = ff;
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

RWByteAddressBuffer __alias_draw_commands : register(u13, space0);
cbuffer $Globals : register(b14, space0)
{
    int _Globals_draw_indirect_buffer_offset : packoffset(c0);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c0.y);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c1);
    uint2 _Globals_garden_mask_dims : packoffset(c1.z);
    uint2 _Globals_buildings_mask_non_blurred_dims : packoffset(c2);
    int2 _Globals_grid_dims : packoffset(c2.z);
    uint2 _Globals_path_mask_dims : packoffset(c3);
    uint2 _Globals_rocky_terrain_dims : packoffset(c3.z);
    uint2 _Globals_water_mask_dims : packoffset(c4);
    uint2 _Globals_foliage_exclusion_mask_dims : packoffset(c4.z);
};

RWByteAddressBuffer __alias_bush_buffer : register(u19, space0);
RWByteAddressBuffer __alias_bush_flowers : register(u20, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t15, space0);
Texture2D<float4> garden_mask : register(t17, space0);
Texture2D<float4> buildings_mask_non_blurred : register(t18, space0);
Texture2D<float4> path_mask : register(t21, space0);
Texture2D<float4> rocky_terrain : register(t22, space0);
Texture2D<float4> water_mask : register(t23, space0);
Texture2D<float4> foliage_exclusion_mask : register(t24, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    do
    {
        float2 _161;
        uint _140 = (gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) + gl_GlobalInvocationID.x;
        uint _142 = _140 + (_140 << 10u);
        uint _144 = _142 ^ (_142 >> 6u);
        uint _146 = _144 + (_144 << 3u);
        uint _148 = _146 ^ (_146 >> 11u);
        float _154 = asfloat(((_148 + (_148 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        float2 _157 = ((((float2(gl_GlobalInvocationID.xy) + 0.5f.xx) / float2(_Globals_grid_dims)) - 0.5f.xx) * 130.0f.xx) + lerp(-1.0f, 1.0f, _154).xx;
        bool _227;
        do
        {
            _161 = (_157 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
            uint2 _166 = uint2(_161 * float2(_Globals_path_mask_dims));
            float4 _184 = terrain_heightmap.GatherRed(sampler_default, _161, int2(0, 0));
            float _189 = _184.z;
            float _190 = _184.y;
            float _192 = _184.w;
            float _193 = _184.x;
            bool _214;
            if (!(path_mask.Load(int3(_166, 0u)).x > 0.20000000298023223876953125f))
            {
                _214 = (int(rocky_terrain.Load(int3(_166, 0u)).x > 0.0f) & int(length(((float2((_189 + _190) - (_192 + _193), (_193 + _190) - (_192 + _189)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(_Globals_terrain_heightmap_dims))) > 0.89999997615814208984375f)) != 0;
            }
            else
            {
                _214 = true;
            }
            bool _219;
            if (!_214)
            {
                _219 = water_mask.Load(int3(_166, 0u)).y > 0.300000011920928955078125f;
            }
            else
            {
                _219 = true;
            }
            bool _224;
            if (!_219)
            {
                _224 = foliage_exclusion_mask.SampleLevel(sampler_llb, _161, 0.0f).x > 0.0f;
            }
            else
            {
                _224 = true;
            }
            if (_224)
            {
                _227 = true;
                break;
            }
            _227 = false;
            break;
        } while(false);
        if (_227)
        {
            break;
        }
        if (buildings_mask_non_blurred.SampleLevel(sampler_default, _161, 0.0f).x > 0.5f)
        {
            break;
        }
        float _241 = garden_mask.SampleLevel(sampler_default, _161, 0.0f).y + 0.004999999888241291046142578125f;
        if (_241 > 0.5f)
        {
            uint _257 = gl_GlobalInvocationID.x + gl_GlobalInvocationID.y;
            uint _259 = _257 + (_257 << 10u);
            uint _261 = _259 ^ (_259 >> 6u);
            uint _263 = _261 + (_261 << 3u);
            uint _265 = _263 ^ (_263 >> 11u);
            float _272 = lerp(0.800000011920928955078125f, 1.2000000476837158203125f, asfloat(((_265 + (_265 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
            float3 _276 = _272.xxx * lerp(0.300000011920928955078125f, 0.800000011920928955078125f, sqrt(clamp(smoothstep(0.449999988079071044921875f, 1.0f, _241) + (smoothstep(0.60000002384185791015625f, 1.0f, _241) * 0.20000000298023223876953125f), 0.0f, 1.0f)));
            float _296 = 3.1400001049041748046875f * _154;
            float3 _297 = normalize(float3(0.0f, 1.0f, 0.0f));
            float _298 = sin(_296);
            float _299 = cos(_296);
            float _300 = 1.0f - _299;
            float _301 = _297.x;
            float _302 = _300 * _301;
            float _305 = _297.y;
            float _306 = _302 * _305;
            float _307 = _297.z;
            float _308 = _307 * _298;
            float _310 = _300 * _307;
            float _311 = _310 * _301;
            float _312 = _305 * _298;
            float _315 = _300 * _305;
            float _318 = _315 * _307;
            float _319 = _301 * _298;
            float4x4 _330 = mul(mul(float4x4(float4(1.0f, 0.0f, 0.0f, _157.x), float4(0.0f, 1.0f, 0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_157.xy * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x)), float4(0.0f, 0.0f, 1.0f, _157.y), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(float4x4(float4(1.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 1.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4x4(float4(min(_276.x, 0.75f), 0.0f, 0.0f, 0.0f), float4(0.0f, min(_276.y * _272, 1.2000000476837158203125f), 0.0f, 0.0f), float4(0.0f, 0.0f, min(_276.z, 0.75f), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), transpose(float4x4(float4((_302 * _301) + _299, _306 - _308, _311 + _312, 0.0f), float4(_306 + _308, (_315 * _305) + _299, _318 - _319, 0.0f), float4(_311 - _312, _318 + _319, (_310 * _307) + _299, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
            uint _344;
            __alias_draw_commands.InterlockedAdd(204, 1u, _344);
            uint _346 = uint(int(_344));
            float _348[12] = { _330[0].x, _330[1].x, _330[2].x, _330[0].y, _330[1].y, _330[2].y, _330[0].z, _330[1].z, _330[2].z, _330[0].w, _330[1].w, _330[2].w };
            Affine3Packed _349 = { _348 };
            BushXform _350 = { _349 };
            [unroll]
            for (int _4ident = 0; _4ident < 12; _4ident++)
            {
                __alias_bush_buffer.Store<float>(_4ident * 4 + _346 * 48 + 0, _350.transform.data[_4ident]);
            }
            uint _354 = ((gl_GlobalInvocationID.x * 2u) + (gl_GlobalInvocationID.y / 5u)) + 1233u;
            uint _356 = _354 + (_354 << 10u);
            uint _358 = _356 ^ (_356 >> 6u);
            uint _360 = _358 + (_358 << 3u);
            uint _362 = _360 ^ (_360 >> 11u);
            bool _373;
            if ((asfloat(((_362 + (_362 << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.300000011920928955078125f)
            {
                _373 = _241 > 0.800000011920928955078125f;
            }
            else
            {
                _373 = false;
            }
            if (_373)
            {
                uint _377;
                __alias_draw_commands.InterlockedAdd(224, 1u, _377);
                uint _379 = uint(int(_377));
                FlowerXform _381 = { _349, (-1.0f).xxx, 2u };
                [unroll]
                for (int _5ident = 0; _5ident < 12; _5ident++)
                {
                    __alias_bush_flowers.Store<float>(_5ident * 4 + _379 * 64 + 0, _381.transform.data[_5ident]);
                }
                __alias_bush_flowers.Store<float3>(_379 * 64 + 48, _381.color);
                __alias_bush_flowers.Store<uint>(_379 * 64 + 60, _381.flower_type);
            }
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
