#define TGV_0 1
#line 1 "shaders/garden/marching_squares.cs.hlsl"


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
#line 6 "shaders/garden/marching_squares.cs.hlsl"

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
#line 7 "shaders/garden/marching_squares.cs.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 8 "shaders/garden/marching_squares.cs.hlsl"

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
#line 9 "shaders/garden/marching_squares.cs.hlsl"

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
#line 10 "shaders/garden/marching_squares.cs.hlsl"

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
#line 11 "shaders/garden/marching_squares.cs.hlsl"

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
#line 12 "shaders/garden/marching_squares.cs.hlsl"

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
#line 14 "shaders/garden/marching_squares.cs.hlsl"


struct GardenStone
{
    Affine3Packed transform;
    float4 color;
};

static const uint2 CORNER[4] = {
    uint2(0u, 0u), uint2(1u, 0u), uint2(1u, 1u), uint2(0u, 1u)
};

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
    uint2 garden_mask_dims;
    uint2 path_mask_dims;
    uint2 water_mask_dims;
    uint2 rocky_terrain_dims;
    uint2 foliage_exclusion_mask_dims;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 contouring_grid_dims;
};

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);
Texture2D<float4> garden_mask : register(t15, space0);
Texture2D<float4> path_mask : register(t16, space0);
Texture2D<float4> water_mask : register(t17, space0);
Texture2D<float4> rocky_terrain : register(t18, space0);
Texture2D<float4> foliage_exclusion_mask : register(t19, space0);
RWStructuredBuffer<GardenStone> __alias_instance_buffer : register(u22, space0);



RWStructuredBuffer<uint> _rda_trigger[] : register(u100, space0);

groupshared uint valid_garden_samples;
groupshared float2 group_ws_pos[9][9];
groupshared float group_garden_mask[9][9];
groupshared float group_path_mask[9][9];
groupshared bool group_discard_mask[9][9];


float garden_iso(float2 ws)
{
    float2 uv = ws * 0.0076923076994717121124267578125f + 0.5f;
    return garden_mask.SampleLevel(sampler_default, uv, 0.0f).y - 0.4f;
}


float2 bisect_iso(float2 a, float2 b)
{
    float fa = garden_iso(a);
    float fb = garden_iso(b);
    float2 p0 = a;
    float2 p1 = b;
    [unroll]
    for (uint i = 0u; i < 2u; i++)
    {
        float2 m = (p0 + p1) * 0.5f;
        float fm = garden_iso(m);
        bool left = (fm * fa) < 0.0f;
        fa = left ? fa : fm;
        fb = left ? fm : fb;
        p0 = left ? p0 : m;
        p1 = left ? m : p1;
    }
    return lerp(p1, p0, clamp(fb / (fb - fa), 0.0f, 1.0f).xx);
}


void place_stones(float2 a, float2 b, uint grid_id)
{
    float2 p0 = float3(a.x, 0.4f, a.y).xz;
    float2 p1 = float3(b.x, 0.4f, b.y).xz;
    float dist = distance(p0, p1);
    if (dist < 0.4f)
        return;

    uint n = uint(max(dist * 2.5f, 1.0f));
    for (uint i = 0u; i <= n; i++)
    {
        float2 t = ((float(i) / float(n)) + (float(i + 1u) / float(n))) * 0.5f;
        float2 pos = lerp(p0, p1, t.xx);
        int seed = int(grid_id) + int(i) * 100;

        float rx = rand01(hash(uint(seed ^ 2346)));
        float ry = rand01(hash(uint(seed ^ 3459)));
        float rz = rand01(hash(uint(seed ^ 234)));
        float3 dir = normalize(float3(rx * 2.0f - 1.0f, (ry * 2.0f - 1.0f) * 0.2f, rz * 2.0f - 1.0f));
        float3 up = dir.y < 0.0f ? float3(0.0f, -1.0f, 0.0f) : float3(0.0f, 1.0f, 0.0f);

        float sw = lerp(0.8f, 1.5f, rand01(hash(uint(seed ^ 346))));
        float3 scale = float3(sw, lerp(0.8f, 1.3f, rand01(hash(uint(seed ^ 346)))), sw) * 0.75f;

        float4x4 basis = transpose(float4x4(
            float4(dir, 0.0f),
            float4(up, 0.0f),
            float4(cross(dir, up), 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f)));
        float4x4 s = float4x4(
            float4(scale.x, 0.0f, 0.0f, 0.0f),
            float4(0.0f, scale.y, 0.0f, 0.0f),
            float4(0.0f, 0.0f, scale.z, 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f));

        float height = get_terrain_height(terrain_heightmap, sampler_default, pos);
        float4x4 m = mul(
            affine_translation(float3(pos.x, height, pos.y)),
            mul(basis, s));

        uint base;
        InterlockedAdd(__alias_draw_commands[uint(draw_indirect_buffer_offset)].instanceCount, 1u, base);

        float c0 = rand01(hash(uint((pos.x + pos.y) * 10.0f + 640.0f)));
        float c1 = rand01(hash(uint((pos.x + pos.y) * 10.0f + 987.0f)));
        GardenStone stone;
        stone.transform = pack_affine(m);
        stone.color = float4(c0, c1, 0.0f, 0.0f);
        __alias_instance_buffer[base] = stone;
    }
}

[numthreads(8, 8, 1)]
void cs_main(uint3 gid : SV_GroupID, uint3 lid : SV_GroupThreadID, uint3 id : SV_DispatchThreadID, uint li : SV_GroupIndex)
{
    float2 grid_dims_f = float2(contouring_grid_dims);

    if (li == 0u)
        valid_garden_samples = 0u;
    GroupMemoryBarrierWithGroupSync();

    for (uint gy = lid.y; gy <= 8u; gy += 8u)
    {
        for (uint gx = lid.x; gx <= 8u; gx += 8u)
        {
            float2 ws = ((float2((gid.xy * uint2(8u, 8u)) + uint2(gx, gy)) / grid_dims_f) - 0.5f) * 130.0f;
            group_ws_pos[gx][gy] = ws;

            float3 w3 = (ws.xxy * 0.0076923076994717121124267578125f) + 0.5f;
            uint2 px = uint2(int2(int(w3.x * float(garden_mask_dims.x)), int(w3.z * float(garden_mask_dims.y))));
            float g = garden_mask.Load(int3(px, 0u)).y;
            group_garden_mask[gx][gy] = g;
            if (g >= 0.4f)
            {
                uint prev;
                InterlockedAdd(valid_garden_samples, 1u, prev);
            }
        }
    }
    GroupMemoryBarrierWithGroupSync();

    if (valid_garden_samples == 0u)
        return;

    for (uint py = lid.y; py <= 8u; py += 8u)
    {
        for (uint px2 = lid.x; px2 <= 8u; px2 += 8u)
        {
            float2 ws = group_ws_pos[px2][py];
            float3 w3 = (ws.xxy * 0.0076923076994717121124267578125f) + 0.5f;
            uint2 ppx = uint2(int2(int(w3.x * float(garden_mask_dims.x)), int(w3.z * float(garden_mask_dims.y))));

            group_path_mask[px2][py] = path_mask.Load(int3(ppx, 0u)).x;

            bool dmask = rocky_terrain.Load(int3(ppx, 0u)).x > 0.0f
                        || water_mask.Load(int3(ppx, 0u)).y > 0.3f
                        || foliage_exclusion_mask.SampleLevel(sampler_llb, ws * 0.0076923076994717121124267578125f + 0.5f, 0.0f).x > 0.0f;
            group_discard_mask[px2][py] = dmask;
        }
    }
    GroupMemoryBarrierWithGroupSync();

    if (float(id.x) > grid_dims_f.x || float(id.y) > grid_dims_f.y)
        return;

    uint config = 0u;
    float2 corner_ws[4];
    bool skip = false;
    [unroll]
    for (int c = 0; c < 4; c++)
    {
        uint2 p = lid.xy + CORNER[c];
        if (group_discard_mask[p.x][p.y])
        {
            skip = true;
            break;
        }
        if (group_garden_mask[p.x][p.y] >= 0.4f && group_path_mask[p.x][p.y] < 0.1f)
            config |= (8u >> (uint(c) & 31u));
        corner_ws[c] = group_ws_pos[p.x][p.y];
    }
    if (skip)
        return;
    if (config == 0u || config == 15u)
        return;

    uint grid_id = (id.x * id.y) + id.x;

    
    
    if (config == 1u || config == 14u)
    {
        float2 a = bisect_iso(corner_ws[0], corner_ws[3]);
        float2 b = bisect_iso(corner_ws[3], corner_ws[2]);
        place_stones(a, b, grid_id);
    }
    else if (config == 2u || config == 13u)
    {
        float2 a = bisect_iso(corner_ws[1], corner_ws[2]);
        float2 b = bisect_iso(corner_ws[2], corner_ws[3]);
        place_stones(a, b, grid_id);
    }
    else if (config == 3u || config == 12u)
    {
        float2 a = bisect_iso(corner_ws[0], corner_ws[1]);
        float2 b = bisect_iso(corner_ws[1], corner_ws[2]);
        place_stones(a, b, grid_id);
    }
    else if (config == 4u || config == 11u)
    {
        float2 a = bisect_iso(corner_ws[1], corner_ws[2]);
        float2 b = bisect_iso(corner_ws[2], corner_ws[3]);
        place_stones(a, b, grid_id);
    }
    else if (config == 5u || config == 10u)
    {
        float2 a = bisect_iso(corner_ws[0], corner_ws[1]);
        float2 b = bisect_iso(corner_ws[1], corner_ws[2]);
        place_stones(a, b, grid_id);
        a = bisect_iso(corner_ws[2], corner_ws[3]);
        b = bisect_iso(corner_ws[3], corner_ws[0]);
        place_stones(a, b, grid_id);
    }
    else if (config == 6u || config == 9u)
    {
        float2 a = bisect_iso(corner_ws[0], corner_ws[3]);
        float2 b = bisect_iso(corner_ws[3], corner_ws[2]);
        place_stones(a, b, grid_id);
        a = bisect_iso(corner_ws[2], corner_ws[1]);
        b = bisect_iso(corner_ws[1], corner_ws[0]);
        place_stones(a, b, grid_id);
    }
    else 
    {
        float2 a = bisect_iso(corner_ws[0], corner_ws[3]);
        float2 b = bisect_iso(corner_ws[3], corner_ws[2]);
        place_stones(a, b, grid_id);
        a = bisect_iso(corner_ws[2], corner_ws[1]);
        b = bisect_iso(corner_ws[1], corner_ws[0]);
        place_stones(a, b, grid_id);
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

struct GardenStone
{
    Affine3Packed transform;
    float4 color;
};

static uint _159;

static const uint2 _164[4] = { uint2(0u, 0u), uint2(1u, 0u), uint2(1u, 1u), uint2(0u, 1u) };

RWByteAddressBuffer __alias_draw_commands : register(u13, space0);
cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset : packoffset(c0);
    uint2 garden_mask_dims : packoffset(c0.y);
    uint2 path_mask_dims : packoffset(c1);
    uint2 water_mask_dims : packoffset(c1.z);
    uint2 rocky_terrain_dims : packoffset(c2);
    uint2 foliage_exclusion_mask_dims : packoffset(c2.z);
    uint2 terrain_heightmap_dims : packoffset(c3);
    uint2 visual_terrain_heightmap_dims : packoffset(c3.z);
    uint2 contouring_grid_dims : packoffset(c4);
};

RWByteAddressBuffer __alias_instance_buffer : register(u22, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> garden_mask : register(t15, space0);
Texture2D<float4> path_mask : register(t16, space0);
Texture2D<float4> water_mask : register(t17, space0);
Texture2D<float4> rocky_terrain : register(t18, space0);
Texture2D<float4> foliage_exclusion_mask : register(t19, space0);
Texture2D<float4> terrain_heightmap : register(t20, space0);

static uint3 gl_WorkGroupID;
static uint3 gl_LocalInvocationID;
static uint3 gl_GlobalInvocationID;
static uint gl_LocalInvocationIndex;
struct SPIRV_Cross_Input
{
    uint3 gl_WorkGroupID : SV_GroupID;
    uint3 gl_LocalInvocationID : SV_GroupThreadID;
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
    uint gl_LocalInvocationIndex : SV_GroupIndex;
};

groupshared uint valid_garden_samples;
groupshared float2 group_ws_pos[9][9];
groupshared float group_garden_mask[9][9];
groupshared float group_path_mask[9][9];
groupshared bool group_discard_mask[9][9];

void comp_main()
{
    do
    {
        float2 _187 = float2(contouring_grid_dims);
        if (0u == gl_LocalInvocationIndex)
        {
            valid_garden_samples = 0u;
        }
        GroupMemoryBarrierWithGroupSync();
        for (uint _193 = gl_LocalInvocationID.y; _193 <= 8u; _193 += 8u)
        {
            for (uint _201 = gl_LocalInvocationID.x; _201 <= 8u; _201 += 8u)
            {
                float2 _215 = ((float2((gl_WorkGroupID.xy * uint2(8u, 8u)) + uint2(_201, _193)) / float2(uint2(_187))) - 0.5f.xx) * 130.0f.xx;
                group_ws_pos[_201][_193] = _215;
                int2 _220 = int2(garden_mask_dims);
                float3 _222 = (_215.xxy * 0.0076923076994717121124267578125f.xxx) + 0.5f.xxx;
                float4 _236 = garden_mask.Load(int3(uint2(int2(int(_222.x * float(_220.x)), int(_222.z * float(_220.y)))), 0u));
                float _237 = _236.y;
                group_garden_mask[_201][_193] = _237;
                if (_237 >= 0.4000000059604644775390625f)
                {
                    uint _242;
                    InterlockedAdd(valid_garden_samples, 1u, _242);
                }
            }
        }
        GroupMemoryBarrierWithGroupSync();
        if (0u == valid_garden_samples)
        {
            break;
        }
        for (uint _248 = gl_LocalInvocationID.y; _248 <= 8u; _248 += 8u)
        {
            for (uint _256 = gl_LocalInvocationID.x; _256 <= 8u; _256 += 8u)
            {
                int2 _267 = int2(garden_mask_dims);
                float3 _269 = (group_ws_pos[_256][_248].xxy * 0.0076923076994717121124267578125f.xxx) + 0.5f.xxx;
                uint2 _288 = uint2(int2(int(_269.x * float(_267.x)), int(_269.z * float(_267.y))));
                group_path_mask[_256][_248] = path_mask.Load(int3(_288, 0u)).x;
                bool _304;
                if (!(rocky_terrain.Load(int3(_288, 0u)).x > 0.0f))
                {
                    _304 = water_mask.Load(int3(_288, 0u)).y > 0.300000011920928955078125f;
                }
                else
                {
                    _304 = true;
                }
                bool _309;
                if (!_304)
                {
                    _309 = foliage_exclusion_mask.SampleLevel(sampler_llb, (group_ws_pos[_256][_248] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x > 0.0f;
                }
                else
                {
                    _309 = true;
                }
                group_discard_mask[_256][_248] = _309;
            }
        }
        GroupMemoryBarrierWithGroupSync();
        bool _322;
        if (!(float(gl_GlobalInvocationID.x) > _187.x))
        {
            _322 = float(gl_GlobalInvocationID.y) > _187.y;
        }
        else
        {
            _322 = true;
        }
        if (_322)
        {
            break;
        }
        uint _326;
        _326 = 0u;
        float2 _176[4];
        uint _327;
        bool _362;
        int _329 = 0;
        [unroll]
        for (;;)
        {
            if (_329 < 4)
            {
                uint2 _336 = gl_LocalInvocationID.xy + _164[_329];
                uint _337 = _336.x;
                uint _338 = _336.y;
                if (group_discard_mask[_337][_338])
                {
                    _362 = true;
                    break;
                }
                if ((int(group_garden_mask[_337][_338] >= 0.4000000059604644775390625f) & int(group_path_mask[_337][_338] < 0.100000001490116119384765625f)) != 0)
                {
                    _327 = _326 | (8u >> (uint(_329) & 31u));
                }
                else
                {
                    _327 = _326;
                }
                _176[_329] = group_ws_pos[_337][_338];
                _326 = _327;
                _329++;
                continue;
            }
            else
            {
                _362 = false;
                break;
            }
        }
        if (_362)
        {
            break;
        }
        int _367 = int((gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) + gl_GlobalInvocationID.x);
        int _368 = int(_326);
        bool _375;
        if (!(_368 == 0))
        {
            _375 = _368 == 15;
        }
        else
        {
            _375 = true;
        }
        if (_375)
        {
        }
        else
        {
            bool _384;
            if (!(_368 == 1))
            {
                _384 = _368 == 14;
            }
            else
            {
                _384 = true;
            }
            if (_384)
            {
                do
                {
                    float2 _401 = (_176[3] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                    float _407;
                    float _410;
                    float2 _412;
                    float2 _414;
                    _407 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_176[0] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                    _410 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, _401, 0.0f).y;
                    _412 = _176[0];
                    _414 = _176[3];
                    for (uint _416 = 0u; _416 < 2u; )
                    {
                        float2 _420 = _412 + _414;
                        float2 _421 = _420 * 0.5f;
                        float _427 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_421 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                        float _428 = _427 * _407;
                        bool _429 = _428 < 0.0f;
                        bool2 _430 = _429.xx;
                        _407 = _429 ? _407 : _427;
                        _410 = _429 ? _427 : _410;
                        _412 = float2(_430.x ? _412.x : _421.x, _430.y ? _412.y : _421.y);
                        _414 = float2(_430.x ? _421.x : _414.x, _430.y ? _421.y : _414.y);
                        _416++;
                        continue;
                    }
                    float2 _435 = lerp(_414, _412, clamp(_410 / (_410 - _407), 0.0f, 1.0f).xx);
                    float2 _439 = float3(_435.x, 0.4000000059604644775390625f, _435.y).xz;
                    float _452;
                    float _455;
                    float2 _457;
                    float2 _459;
                    _452 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, _401, 0.0f).y;
                    _455 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_176[2] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                    _457 = _176[3];
                    _459 = _176[2];
                    for (uint _461 = 0u; _461 < 2u; )
                    {
                        float2 _465 = _457 + _459;
                        float2 _466 = _465 * 0.5f;
                        float _472 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_466 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                        float _473 = _472 * _452;
                        bool _474 = _473 < 0.0f;
                        bool2 _475 = _474.xx;
                        _452 = _474 ? _452 : _472;
                        _455 = _474 ? _472 : _455;
                        _457 = float2(_475.x ? _457.x : _466.x, _475.y ? _457.y : _466.y);
                        _459 = float2(_475.x ? _466.x : _459.x, _475.y ? _466.y : _459.y);
                        _461++;
                        continue;
                    }
                    float2 _480 = lerp(_459, _457, clamp(_455 / (_455 - _452), 0.0f, 1.0f).xx);
                    float2 _484 = float3(_480.x, 0.4000000059604644775390625f, _480.y).xz;
                    float _485 = distance(_439, _484);
                    if (_485 < 0.4000000059604644775390625f)
                    {
                        break;
                    }
                    int _491 = int(max(_485 * 2.5f, 1.0f));
                    uint _493 = 0u;
                    uint _496 = 0u;
                    bool _498 = false;
                    bool _500 = false;
                    int _502 = 0;
                    for (; _502 < (_491 + 1); )
                    {
                        float _508 = float(_491);
                        int _503 = _502 + 1;
                        float2 _515 = lerp(_439, _484, (((float(_502) / _508) + (float(_503) / _508)) * 0.5f).xx);
                        float _516 = _515.x;
                        float _524 = _515.y;
                        int _526 = _367 + (_502 * 100);
                        uint _528 = uint(_526 ^ 2346);
                        uint _530 = _528 + (_528 << 10u);
                        uint _532 = _530 ^ (_530 >> 6u);
                        uint _534 = _532 + (_532 << 3u);
                        uint _536 = _534 ^ (_534 >> 11u);
                        uint _538 = _536 + (_536 << 15u);
                        bool _539 = _500 ? _500 : true;
                        uint _540 = _500 ? _496 : 8388607u;
                        bool _541 = _498 ? _498 : true;
                        uint _542 = _498 ? _493 : 1065353216u;
                        uint _550 = uint(_526 ^ 3459);
                        uint _552 = _550 + (_550 << 10u);
                        uint _554 = _552 ^ (_552 >> 6u);
                        uint _556 = _554 + (_554 << 3u);
                        uint _558 = _556 ^ (_556 >> 11u);
                        bool _561 = _539 ? _539 : true;
                        uint _562 = _539 ? _540 : 8388607u;
                        bool _563 = _541 ? _541 : true;
                        uint _564 = _541 ? _542 : 1065353216u;
                        float _570 = ((asfloat(((_558 + (_558 << 15u)) & _562) | _564) - 1.0f) * 2.0f) - 1.0f;
                        uint _572 = uint(_526 ^ 234);
                        uint _574 = _572 + (_572 << 10u);
                        uint _576 = _574 ^ (_574 >> 6u);
                        uint _578 = _576 + (_576 << 3u);
                        uint _580 = _578 ^ (_578 >> 11u);
                        bool _583 = _561 ? _561 : true;
                        uint _584 = _561 ? _562 : 8388607u;
                        bool _585 = _563 ? _563 : true;
                        uint _586 = _563 ? _564 : 1065353216u;
                        float3 _593 = float3(((asfloat((_538 & _540) | _542) - 1.0f) * 2.0f) - 1.0f, _570, ((asfloat(((_580 + (_580 << 15u)) & _584) | _586) - 1.0f) * 2.0f) - 1.0f);
                        _593.y = _570 * 0.20000000298023223876953125f;
                        float3 _596 = normalize(_593);
                        bool3 _599 = (_596.y < 0.0f).xxx;
                        float3 _600 = float3(_599.x ? float3(0.0f, -1.0f, 0.0f).x : float3(0.0f, 1.0f, 0.0f).x, _599.y ? float3(0.0f, -1.0f, 0.0f).y : float3(0.0f, 1.0f, 0.0f).y, _599.z ? float3(0.0f, -1.0f, 0.0f).z : float3(0.0f, 1.0f, 0.0f).z);
                        uint _603 = uint(_526 ^ 346);
                        uint _605 = _603 + (_603 << 10u);
                        uint _607 = _605 ^ (_605 >> 6u);
                        uint _609 = _607 + (_607 << 3u);
                        uint _611 = _609 ^ (_609 >> 11u);
                        bool _614 = _583 ? _583 : true;
                        uint _615 = _583 ? _584 : 8388607u;
                        bool _616 = _585 ? _585 : true;
                        uint _617 = _585 ? _586 : 1065353216u;
                        bool _624 = _614 ? _614 : true;
                        uint _625 = _614 ? _615 : 8388607u;
                        bool _626 = _616 ? _616 : true;
                        uint _627 = _616 ? _617 : 1065353216u;
                        float2 _633 = lerp(0.800000011920928955078125f, 1.5f, asfloat((_538 & _625) | _627) - 1.0f).xx;
                        float3 _635 = float3(_633.x, lerp(0.800000011920928955078125f, 1.2999999523162841796875f, asfloat(((_611 + (_611 << 15u)) & _615) | _617) - 1.0f).xxx.y, _633.y) * 0.75f;
                        float4x4 _661 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _516), float4(0.0f, 1.0f, 0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_515 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x)), float4(0.0f, 0.0f, 1.0f, _524), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(transpose(float4x4(float4(_596, 0.0f), float4(_600, 0.0f), float4(cross(_596, _600), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))), float4x4(float4(_635.x, 0.0f, 0.0f, 0.0f), float4(0.0f, _635.y, 0.0f, 0.0f), float4(0.0f, 0.0f, _635.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                        uint _678;
                        __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, _678);
                        float _681 = 10.0f * (_516 + _524);
                        uint _683 = uint(_681 + 640.0f);
                        uint _685 = _683 + (_683 << 10u);
                        uint _687 = _685 ^ (_685 >> 6u);
                        uint _689 = _687 + (_687 << 3u);
                        uint _691 = _689 ^ (_689 >> 11u);
                        bool _694 = _624 ? _624 : true;
                        uint _695 = _624 ? _625 : 8388607u;
                        bool _696 = _626 ? _626 : true;
                        uint _697 = _626 ? _627 : 1065353216u;
                        uint _703 = uint(_681 + 987.0f);
                        uint _705 = _703 + (_703 << 10u);
                        uint _707 = _705 ^ (_705 >> 6u);
                        uint _709 = _707 + (_707 << 3u);
                        uint _711 = _709 ^ (_709 >> 11u);
                        uint _497 = _694 ? _695 : 8388607u;
                        uint _494 = _696 ? _697 : 1065353216u;
                        uint _719 = uint(int(_678));
                        float _721[12] = { _661[0].x, _661[1].x, _661[2].x, _661[0].y, _661[1].y, _661[2].y, _661[0].z, _661[1].z, _661[2].z, _661[0].w, _661[1].w, _661[2].w };
                        Affine3Packed _722 = { _721 };
                        GardenStone _723 = { _722, float4(asfloat(((_691 + (_691 << 15u)) & _695) | _697) - 1.0f, asfloat(((_711 + (_711 << 15u)) & _497) | _494) - 1.0f, 0.0f, 0.0f) };
                        [unroll]
                        for (int _10ident = 0; _10ident < 12; _10ident++)
                        {
                            __alias_instance_buffer.Store<float>(_10ident * 4 + _719 * 64 + 0, _723.transform.data[_10ident]);
                        }
                        __alias_instance_buffer.Store<float4>(_719 * 64 + 48, _723.color);
                        _493 = _494;
                        _496 = _497;
                        _498 = _696 ? _696 : true;
                        _500 = _694 ? _694 : true;
                        _502 = _503;
                        continue;
                    }
                    break;
                } while(false);
            }
            else
            {
                bool _729;
                if (!(_368 == 2))
                {
                    _729 = _368 == 13;
                }
                else
                {
                    _729 = true;
                }
                if (_729)
                {
                    do
                    {
                        float2 _746 = (_176[2] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                        float _752;
                        float _755;
                        float2 _757;
                        float2 _759;
                        _752 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_176[1] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                        _755 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, _746, 0.0f).y;
                        _757 = _176[1];
                        _759 = _176[2];
                        for (uint _761 = 0u; _761 < 2u; )
                        {
                            float2 _765 = _757 + _759;
                            float2 _766 = _765 * 0.5f;
                            float _772 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_766 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                            float _773 = _772 * _752;
                            bool _774 = _773 < 0.0f;
                            bool2 _775 = _774.xx;
                            _752 = _774 ? _752 : _772;
                            _755 = _774 ? _772 : _755;
                            _757 = float2(_775.x ? _757.x : _766.x, _775.y ? _757.y : _766.y);
                            _759 = float2(_775.x ? _766.x : _759.x, _775.y ? _766.y : _759.y);
                            _761++;
                            continue;
                        }
                        float2 _780 = lerp(_759, _757, clamp(_755 / (_755 - _752), 0.0f, 1.0f).xx);
                        float2 _784 = float3(_780.x, 0.4000000059604644775390625f, _780.y).xz;
                        float _797;
                        float _800;
                        float2 _802;
                        float2 _804;
                        _797 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_176[3] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                        _800 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, _746, 0.0f).y;
                        _802 = _176[3];
                        _804 = _176[2];
                        for (uint _806 = 0u; _806 < 2u; )
                        {
                            float2 _810 = _802 + _804;
                            float2 _811 = _810 * 0.5f;
                            float _817 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_811 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                            float _818 = _817 * _797;
                            bool _819 = _818 < 0.0f;
                            bool2 _820 = _819.xx;
                            _797 = _819 ? _797 : _817;
                            _800 = _819 ? _817 : _800;
                            _802 = float2(_820.x ? _802.x : _811.x, _820.y ? _802.y : _811.y);
                            _804 = float2(_820.x ? _811.x : _804.x, _820.y ? _811.y : _804.y);
                            _806++;
                            continue;
                        }
                        float2 _825 = lerp(_804, _802, clamp(_800 / (_800 - _797), 0.0f, 1.0f).xx);
                        float2 _829 = float3(_825.x, 0.4000000059604644775390625f, _825.y).xz;
                        float _830 = distance(_784, _829);
                        if (_830 < 0.4000000059604644775390625f)
                        {
                            break;
                        }
                        int _836 = int(max(_830 * 2.5f, 1.0f));
                        uint _838 = 0u;
                        uint _841 = 0u;
                        bool _843 = false;
                        bool _845 = false;
                        int _847 = 0;
                        for (; _847 < (_836 + 1); )
                        {
                            float _853 = float(_836);
                            int _848 = _847 + 1;
                            float2 _860 = lerp(_784, _829, (((float(_847) / _853) + (float(_848) / _853)) * 0.5f).xx);
                            float _861 = _860.x;
                            float _869 = _860.y;
                            int _871 = _367 + (_847 * 100);
                            uint _873 = uint(_871 ^ 2346);
                            uint _875 = _873 + (_873 << 10u);
                            uint _877 = _875 ^ (_875 >> 6u);
                            uint _879 = _877 + (_877 << 3u);
                            uint _881 = _879 ^ (_879 >> 11u);
                            uint _883 = _881 + (_881 << 15u);
                            bool _884 = _845 ? _845 : true;
                            uint _885 = _845 ? _841 : 8388607u;
                            bool _886 = _843 ? _843 : true;
                            uint _887 = _843 ? _838 : 1065353216u;
                            uint _895 = uint(_871 ^ 3459);
                            uint _897 = _895 + (_895 << 10u);
                            uint _899 = _897 ^ (_897 >> 6u);
                            uint _901 = _899 + (_899 << 3u);
                            uint _903 = _901 ^ (_901 >> 11u);
                            bool _906 = _884 ? _884 : true;
                            uint _907 = _884 ? _885 : 8388607u;
                            bool _908 = _886 ? _886 : true;
                            uint _909 = _886 ? _887 : 1065353216u;
                            float _915 = ((asfloat(((_903 + (_903 << 15u)) & _907) | _909) - 1.0f) * 2.0f) - 1.0f;
                            uint _917 = uint(_871 ^ 234);
                            uint _919 = _917 + (_917 << 10u);
                            uint _921 = _919 ^ (_919 >> 6u);
                            uint _923 = _921 + (_921 << 3u);
                            uint _925 = _923 ^ (_923 >> 11u);
                            bool _928 = _906 ? _906 : true;
                            uint _929 = _906 ? _907 : 8388607u;
                            bool _930 = _908 ? _908 : true;
                            uint _931 = _908 ? _909 : 1065353216u;
                            float3 _938 = float3(((asfloat((_883 & _885) | _887) - 1.0f) * 2.0f) - 1.0f, _915, ((asfloat(((_925 + (_925 << 15u)) & _929) | _931) - 1.0f) * 2.0f) - 1.0f);
                            _938.y = _915 * 0.20000000298023223876953125f;
                            float3 _941 = normalize(_938);
                            bool3 _944 = (_941.y < 0.0f).xxx;
                            float3 _945 = float3(_944.x ? float3(0.0f, -1.0f, 0.0f).x : float3(0.0f, 1.0f, 0.0f).x, _944.y ? float3(0.0f, -1.0f, 0.0f).y : float3(0.0f, 1.0f, 0.0f).y, _944.z ? float3(0.0f, -1.0f, 0.0f).z : float3(0.0f, 1.0f, 0.0f).z);
                            uint _948 = uint(_871 ^ 346);
                            uint _950 = _948 + (_948 << 10u);
                            uint _952 = _950 ^ (_950 >> 6u);
                            uint _954 = _952 + (_952 << 3u);
                            uint _956 = _954 ^ (_954 >> 11u);
                            bool _959 = _928 ? _928 : true;
                            uint _960 = _928 ? _929 : 8388607u;
                            bool _961 = _930 ? _930 : true;
                            uint _962 = _930 ? _931 : 1065353216u;
                            bool _969 = _959 ? _959 : true;
                            uint _970 = _959 ? _960 : 8388607u;
                            bool _971 = _961 ? _961 : true;
                            uint _972 = _961 ? _962 : 1065353216u;
                            float2 _978 = lerp(0.800000011920928955078125f, 1.5f, asfloat((_883 & _970) | _972) - 1.0f).xx;
                            float3 _980 = float3(_978.x, lerp(0.800000011920928955078125f, 1.2999999523162841796875f, asfloat(((_956 + (_956 << 15u)) & _960) | _962) - 1.0f).xxx.y, _978.y) * 0.75f;
                            float4x4 _1006 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _861), float4(0.0f, 1.0f, 0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_860 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x)), float4(0.0f, 0.0f, 1.0f, _869), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(transpose(float4x4(float4(_941, 0.0f), float4(_945, 0.0f), float4(cross(_941, _945), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))), float4x4(float4(_980.x, 0.0f, 0.0f, 0.0f), float4(0.0f, _980.y, 0.0f, 0.0f), float4(0.0f, 0.0f, _980.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                            uint _1023;
                            __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, _1023);
                            float _1026 = 10.0f * (_861 + _869);
                            uint _1028 = uint(_1026 + 640.0f);
                            uint _1030 = _1028 + (_1028 << 10u);
                            uint _1032 = _1030 ^ (_1030 >> 6u);
                            uint _1034 = _1032 + (_1032 << 3u);
                            uint _1036 = _1034 ^ (_1034 >> 11u);
                            bool _1039 = _969 ? _969 : true;
                            uint _1040 = _969 ? _970 : 8388607u;
                            bool _1041 = _971 ? _971 : true;
                            uint _1042 = _971 ? _972 : 1065353216u;
                            uint _1048 = uint(_1026 + 987.0f);
                            uint _1050 = _1048 + (_1048 << 10u);
                            uint _1052 = _1050 ^ (_1050 >> 6u);
                            uint _1054 = _1052 + (_1052 << 3u);
                            uint _1056 = _1054 ^ (_1054 >> 11u);
                            uint _842 = _1039 ? _1040 : 8388607u;
                            uint _839 = _1041 ? _1042 : 1065353216u;
                            uint _1064 = uint(int(_1023));
                            float _1066[12] = { _1006[0].x, _1006[1].x, _1006[2].x, _1006[0].y, _1006[1].y, _1006[2].y, _1006[0].z, _1006[1].z, _1006[2].z, _1006[0].w, _1006[1].w, _1006[2].w };
                            Affine3Packed _1067 = { _1066 };
                            GardenStone _1068 = { _1067, float4(asfloat(((_1036 + (_1036 << 15u)) & _1040) | _1042) - 1.0f, asfloat(((_1056 + (_1056 << 15u)) & _842) | _839) - 1.0f, 0.0f, 0.0f) };
                            [unroll]
                            for (int _11ident = 0; _11ident < 12; _11ident++)
                            {
                                __alias_instance_buffer.Store<float>(_11ident * 4 + _1064 * 64 + 0, _1068.transform.data[_11ident]);
                            }
                            __alias_instance_buffer.Store<float4>(_1064 * 64 + 48, _1068.color);
                            _838 = _839;
                            _841 = _842;
                            _843 = _1041 ? _1041 : true;
                            _845 = _1039 ? _1039 : true;
                            _847 = _848;
                            continue;
                        }
                        break;
                    } while(false);
                }
                else
                {
                    bool _1074;
                    if (!(_368 == 3))
                    {
                        _1074 = _368 == 12;
                    }
                    else
                    {
                        _1074 = true;
                    }
                    if (_1074)
                    {
                        do
                        {
                            float _1097;
                            float _1100;
                            float2 _1102;
                            float2 _1104;
                            _1097 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_176[0] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                            _1100 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_176[3] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                            _1102 = _176[0];
                            _1104 = _176[3];
                            for (uint _1106 = 0u; _1106 < 2u; )
                            {
                                float2 _1110 = _1102 + _1104;
                                float2 _1111 = _1110 * 0.5f;
                                float _1117 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_1111 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                float _1118 = _1117 * _1097;
                                bool _1119 = _1118 < 0.0f;
                                bool2 _1120 = _1119.xx;
                                _1097 = _1119 ? _1097 : _1117;
                                _1100 = _1119 ? _1117 : _1100;
                                _1102 = float2(_1120.x ? _1102.x : _1111.x, _1120.y ? _1102.y : _1111.y);
                                _1104 = float2(_1120.x ? _1111.x : _1104.x, _1120.y ? _1111.y : _1104.y);
                                _1106++;
                                continue;
                            }
                            float2 _1125 = lerp(_1104, _1102, clamp(_1100 / (_1100 - _1097), 0.0f, 1.0f).xx);
                            float2 _1129 = float3(_1125.x, 0.4000000059604644775390625f, _1125.y).xz;
                            float _1145;
                            float _1148;
                            float2 _1150;
                            float2 _1152;
                            _1145 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_176[1] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                            _1148 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_176[2] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                            _1150 = _176[1];
                            _1152 = _176[2];
                            for (uint _1154 = 0u; _1154 < 2u; )
                            {
                                float2 _1158 = _1150 + _1152;
                                float2 _1159 = _1158 * 0.5f;
                                float _1165 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_1159 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                float _1166 = _1165 * _1145;
                                bool _1167 = _1166 < 0.0f;
                                bool2 _1168 = _1167.xx;
                                _1145 = _1167 ? _1145 : _1165;
                                _1148 = _1167 ? _1165 : _1148;
                                _1150 = float2(_1168.x ? _1150.x : _1159.x, _1168.y ? _1150.y : _1159.y);
                                _1152 = float2(_1168.x ? _1159.x : _1152.x, _1168.y ? _1159.y : _1152.y);
                                _1154++;
                                continue;
                            }
                            float2 _1173 = lerp(_1152, _1150, clamp(_1148 / (_1148 - _1145), 0.0f, 1.0f).xx);
                            float2 _1177 = float3(_1173.x, 0.4000000059604644775390625f, _1173.y).xz;
                            float _1178 = distance(_1129, _1177);
                            if (_1178 < 0.4000000059604644775390625f)
                            {
                                break;
                            }
                            int _1184 = int(max(_1178 * 2.5f, 1.0f));
                            uint _1186 = 0u;
                            uint _1189 = 0u;
                            bool _1191 = false;
                            bool _1193 = false;
                            int _1195 = 0;
                            for (; _1195 < (_1184 + 1); )
                            {
                                float _1201 = float(_1184);
                                int _1196 = _1195 + 1;
                                float2 _1208 = lerp(_1129, _1177, (((float(_1195) / _1201) + (float(_1196) / _1201)) * 0.5f).xx);
                                float _1209 = _1208.x;
                                float _1217 = _1208.y;
                                int _1219 = _367 + (_1195 * 100);
                                uint _1221 = uint(_1219 ^ 2346);
                                uint _1223 = _1221 + (_1221 << 10u);
                                uint _1225 = _1223 ^ (_1223 >> 6u);
                                uint _1227 = _1225 + (_1225 << 3u);
                                uint _1229 = _1227 ^ (_1227 >> 11u);
                                uint _1231 = _1229 + (_1229 << 15u);
                                bool _1232 = _1193 ? _1193 : true;
                                uint _1233 = _1193 ? _1189 : 8388607u;
                                bool _1234 = _1191 ? _1191 : true;
                                uint _1235 = _1191 ? _1186 : 1065353216u;
                                uint _1243 = uint(_1219 ^ 3459);
                                uint _1245 = _1243 + (_1243 << 10u);
                                uint _1247 = _1245 ^ (_1245 >> 6u);
                                uint _1249 = _1247 + (_1247 << 3u);
                                uint _1251 = _1249 ^ (_1249 >> 11u);
                                bool _1254 = _1232 ? _1232 : true;
                                uint _1255 = _1232 ? _1233 : 8388607u;
                                bool _1256 = _1234 ? _1234 : true;
                                uint _1257 = _1234 ? _1235 : 1065353216u;
                                float _1263 = ((asfloat(((_1251 + (_1251 << 15u)) & _1255) | _1257) - 1.0f) * 2.0f) - 1.0f;
                                uint _1265 = uint(_1219 ^ 234);
                                uint _1267 = _1265 + (_1265 << 10u);
                                uint _1269 = _1267 ^ (_1267 >> 6u);
                                uint _1271 = _1269 + (_1269 << 3u);
                                uint _1273 = _1271 ^ (_1271 >> 11u);
                                bool _1276 = _1254 ? _1254 : true;
                                uint _1277 = _1254 ? _1255 : 8388607u;
                                bool _1278 = _1256 ? _1256 : true;
                                uint _1279 = _1256 ? _1257 : 1065353216u;
                                float3 _1286 = float3(((asfloat((_1231 & _1233) | _1235) - 1.0f) * 2.0f) - 1.0f, _1263, ((asfloat(((_1273 + (_1273 << 15u)) & _1277) | _1279) - 1.0f) * 2.0f) - 1.0f);
                                _1286.y = _1263 * 0.20000000298023223876953125f;
                                float3 _1289 = normalize(_1286);
                                bool3 _1292 = (_1289.y < 0.0f).xxx;
                                float3 _1293 = float3(_1292.x ? float3(0.0f, -1.0f, 0.0f).x : float3(0.0f, 1.0f, 0.0f).x, _1292.y ? float3(0.0f, -1.0f, 0.0f).y : float3(0.0f, 1.0f, 0.0f).y, _1292.z ? float3(0.0f, -1.0f, 0.0f).z : float3(0.0f, 1.0f, 0.0f).z);
                                uint _1296 = uint(_1219 ^ 346);
                                uint _1298 = _1296 + (_1296 << 10u);
                                uint _1300 = _1298 ^ (_1298 >> 6u);
                                uint _1302 = _1300 + (_1300 << 3u);
                                uint _1304 = _1302 ^ (_1302 >> 11u);
                                bool _1307 = _1276 ? _1276 : true;
                                uint _1308 = _1276 ? _1277 : 8388607u;
                                bool _1309 = _1278 ? _1278 : true;
                                uint _1310 = _1278 ? _1279 : 1065353216u;
                                bool _1317 = _1307 ? _1307 : true;
                                uint _1318 = _1307 ? _1308 : 8388607u;
                                bool _1319 = _1309 ? _1309 : true;
                                uint _1320 = _1309 ? _1310 : 1065353216u;
                                float2 _1326 = lerp(0.800000011920928955078125f, 1.5f, asfloat((_1231 & _1318) | _1320) - 1.0f).xx;
                                float3 _1328 = float3(_1326.x, lerp(0.800000011920928955078125f, 1.2999999523162841796875f, asfloat(((_1304 + (_1304 << 15u)) & _1308) | _1310) - 1.0f).xxx.y, _1326.y) * 0.75f;
                                float4x4 _1354 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _1209), float4(0.0f, 1.0f, 0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_1208 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x)), float4(0.0f, 0.0f, 1.0f, _1217), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(transpose(float4x4(float4(_1289, 0.0f), float4(_1293, 0.0f), float4(cross(_1289, _1293), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))), float4x4(float4(_1328.x, 0.0f, 0.0f, 0.0f), float4(0.0f, _1328.y, 0.0f, 0.0f), float4(0.0f, 0.0f, _1328.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                uint _1371;
                                __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, _1371);
                                float _1374 = 10.0f * (_1209 + _1217);
                                uint _1376 = uint(_1374 + 640.0f);
                                uint _1378 = _1376 + (_1376 << 10u);
                                uint _1380 = _1378 ^ (_1378 >> 6u);
                                uint _1382 = _1380 + (_1380 << 3u);
                                uint _1384 = _1382 ^ (_1382 >> 11u);
                                bool _1387 = _1317 ? _1317 : true;
                                uint _1388 = _1317 ? _1318 : 8388607u;
                                bool _1389 = _1319 ? _1319 : true;
                                uint _1390 = _1319 ? _1320 : 1065353216u;
                                uint _1396 = uint(_1374 + 987.0f);
                                uint _1398 = _1396 + (_1396 << 10u);
                                uint _1400 = _1398 ^ (_1398 >> 6u);
                                uint _1402 = _1400 + (_1400 << 3u);
                                uint _1404 = _1402 ^ (_1402 >> 11u);
                                uint _1190 = _1387 ? _1388 : 8388607u;
                                uint _1187 = _1389 ? _1390 : 1065353216u;
                                uint _1412 = uint(int(_1371));
                                float _1414[12] = { _1354[0].x, _1354[1].x, _1354[2].x, _1354[0].y, _1354[1].y, _1354[2].y, _1354[0].z, _1354[1].z, _1354[2].z, _1354[0].w, _1354[1].w, _1354[2].w };
                                Affine3Packed _1415 = { _1414 };
                                GardenStone _1416 = { _1415, float4(asfloat(((_1384 + (_1384 << 15u)) & _1388) | _1390) - 1.0f, asfloat(((_1404 + (_1404 << 15u)) & _1190) | _1187) - 1.0f, 0.0f, 0.0f) };
                                [unroll]
                                for (int _12ident = 0; _12ident < 12; _12ident++)
                                {
                                    __alias_instance_buffer.Store<float>(_12ident * 4 + _1412 * 64 + 0, _1416.transform.data[_12ident]);
                                }
                                __alias_instance_buffer.Store<float4>(_1412 * 64 + 48, _1416.color);
                                _1186 = _1187;
                                _1189 = _1190;
                                _1191 = _1389 ? _1389 : true;
                                _1193 = _1387 ? _1387 : true;
                                _1195 = _1196;
                                continue;
                            }
                            break;
                        } while(false);
                    }
                    else
                    {
                        bool _1422;
                        if (!(_368 == 4))
                        {
                            _1422 = _368 == 11;
                        }
                        else
                        {
                            _1422 = true;
                        }
                        if (_1422)
                        {
                            do
                            {
                                float2 _1439 = (_176[1] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                float _1445;
                                float _1448;
                                float2 _1450;
                                float2 _1452;
                                _1445 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_176[0] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                _1448 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, _1439, 0.0f).y;
                                _1450 = _176[0];
                                _1452 = _176[1];
                                for (uint _1454 = 0u; _1454 < 2u; )
                                {
                                    float2 _1458 = _1450 + _1452;
                                    float2 _1459 = _1458 * 0.5f;
                                    float _1465 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_1459 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                    float _1466 = _1465 * _1445;
                                    bool _1467 = _1466 < 0.0f;
                                    bool2 _1468 = _1467.xx;
                                    _1445 = _1467 ? _1445 : _1465;
                                    _1448 = _1467 ? _1465 : _1448;
                                    _1450 = float2(_1468.x ? _1450.x : _1459.x, _1468.y ? _1450.y : _1459.y);
                                    _1452 = float2(_1468.x ? _1459.x : _1452.x, _1468.y ? _1459.y : _1452.y);
                                    _1454++;
                                    continue;
                                }
                                float2 _1473 = lerp(_1452, _1450, clamp(_1448 / (_1448 - _1445), 0.0f, 1.0f).xx);
                                float2 _1477 = float3(_1473.x, 0.4000000059604644775390625f, _1473.y).xz;
                                float _1490;
                                float _1493;
                                float2 _1495;
                                float2 _1497;
                                _1490 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, _1439, 0.0f).y;
                                _1493 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_176[2] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                _1495 = _176[1];
                                _1497 = _176[2];
                                for (uint _1499 = 0u; _1499 < 2u; )
                                {
                                    float2 _1503 = _1495 + _1497;
                                    float2 _1504 = _1503 * 0.5f;
                                    float _1510 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_1504 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                    float _1511 = _1510 * _1490;
                                    bool _1512 = _1511 < 0.0f;
                                    bool2 _1513 = _1512.xx;
                                    _1490 = _1512 ? _1490 : _1510;
                                    _1493 = _1512 ? _1510 : _1493;
                                    _1495 = float2(_1513.x ? _1495.x : _1504.x, _1513.y ? _1495.y : _1504.y);
                                    _1497 = float2(_1513.x ? _1504.x : _1497.x, _1513.y ? _1504.y : _1497.y);
                                    _1499++;
                                    continue;
                                }
                                float2 _1518 = lerp(_1497, _1495, clamp(_1493 / (_1493 - _1490), 0.0f, 1.0f).xx);
                                float2 _1522 = float3(_1518.x, 0.4000000059604644775390625f, _1518.y).xz;
                                float _1523 = distance(_1477, _1522);
                                if (_1523 < 0.4000000059604644775390625f)
                                {
                                    break;
                                }
                                int _1529 = int(max(_1523 * 2.5f, 1.0f));
                                uint _1531 = 0u;
                                uint _1534 = 0u;
                                bool _1536 = false;
                                bool _1538 = false;
                                int _1540 = 0;
                                for (; _1540 < (_1529 + 1); )
                                {
                                    float _1546 = float(_1529);
                                    int _1541 = _1540 + 1;
                                    float2 _1553 = lerp(_1477, _1522, (((float(_1540) / _1546) + (float(_1541) / _1546)) * 0.5f).xx);
                                    float _1554 = _1553.x;
                                    float _1562 = _1553.y;
                                    int _1564 = _367 + (_1540 * 100);
                                    uint _1566 = uint(_1564 ^ 2346);
                                    uint _1568 = _1566 + (_1566 << 10u);
                                    uint _1570 = _1568 ^ (_1568 >> 6u);
                                    uint _1572 = _1570 + (_1570 << 3u);
                                    uint _1574 = _1572 ^ (_1572 >> 11u);
                                    uint _1576 = _1574 + (_1574 << 15u);
                                    bool _1577 = _1538 ? _1538 : true;
                                    uint _1578 = _1538 ? _1534 : 8388607u;
                                    bool _1579 = _1536 ? _1536 : true;
                                    uint _1580 = _1536 ? _1531 : 1065353216u;
                                    uint _1588 = uint(_1564 ^ 3459);
                                    uint _1590 = _1588 + (_1588 << 10u);
                                    uint _1592 = _1590 ^ (_1590 >> 6u);
                                    uint _1594 = _1592 + (_1592 << 3u);
                                    uint _1596 = _1594 ^ (_1594 >> 11u);
                                    bool _1599 = _1577 ? _1577 : true;
                                    uint _1600 = _1577 ? _1578 : 8388607u;
                                    bool _1601 = _1579 ? _1579 : true;
                                    uint _1602 = _1579 ? _1580 : 1065353216u;
                                    float _1608 = ((asfloat(((_1596 + (_1596 << 15u)) & _1600) | _1602) - 1.0f) * 2.0f) - 1.0f;
                                    uint _1610 = uint(_1564 ^ 234);
                                    uint _1612 = _1610 + (_1610 << 10u);
                                    uint _1614 = _1612 ^ (_1612 >> 6u);
                                    uint _1616 = _1614 + (_1614 << 3u);
                                    uint _1618 = _1616 ^ (_1616 >> 11u);
                                    bool _1621 = _1599 ? _1599 : true;
                                    uint _1622 = _1599 ? _1600 : 8388607u;
                                    bool _1623 = _1601 ? _1601 : true;
                                    uint _1624 = _1601 ? _1602 : 1065353216u;
                                    float3 _1631 = float3(((asfloat((_1576 & _1578) | _1580) - 1.0f) * 2.0f) - 1.0f, _1608, ((asfloat(((_1618 + (_1618 << 15u)) & _1622) | _1624) - 1.0f) * 2.0f) - 1.0f);
                                    _1631.y = _1608 * 0.20000000298023223876953125f;
                                    float3 _1634 = normalize(_1631);
                                    bool3 _1637 = (_1634.y < 0.0f).xxx;
                                    float3 _1638 = float3(_1637.x ? float3(0.0f, -1.0f, 0.0f).x : float3(0.0f, 1.0f, 0.0f).x, _1637.y ? float3(0.0f, -1.0f, 0.0f).y : float3(0.0f, 1.0f, 0.0f).y, _1637.z ? float3(0.0f, -1.0f, 0.0f).z : float3(0.0f, 1.0f, 0.0f).z);
                                    uint _1641 = uint(_1564 ^ 346);
                                    uint _1643 = _1641 + (_1641 << 10u);
                                    uint _1645 = _1643 ^ (_1643 >> 6u);
                                    uint _1647 = _1645 + (_1645 << 3u);
                                    uint _1649 = _1647 ^ (_1647 >> 11u);
                                    bool _1652 = _1621 ? _1621 : true;
                                    uint _1653 = _1621 ? _1622 : 8388607u;
                                    bool _1654 = _1623 ? _1623 : true;
                                    uint _1655 = _1623 ? _1624 : 1065353216u;
                                    bool _1662 = _1652 ? _1652 : true;
                                    uint _1663 = _1652 ? _1653 : 8388607u;
                                    bool _1664 = _1654 ? _1654 : true;
                                    uint _1665 = _1654 ? _1655 : 1065353216u;
                                    float2 _1671 = lerp(0.800000011920928955078125f, 1.5f, asfloat((_1576 & _1663) | _1665) - 1.0f).xx;
                                    float3 _1673 = float3(_1671.x, lerp(0.800000011920928955078125f, 1.2999999523162841796875f, asfloat(((_1649 + (_1649 << 15u)) & _1653) | _1655) - 1.0f).xxx.y, _1671.y) * 0.75f;
                                    float4x4 _1699 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _1554), float4(0.0f, 1.0f, 0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_1553 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x)), float4(0.0f, 0.0f, 1.0f, _1562), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(transpose(float4x4(float4(_1634, 0.0f), float4(_1638, 0.0f), float4(cross(_1634, _1638), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))), float4x4(float4(_1673.x, 0.0f, 0.0f, 0.0f), float4(0.0f, _1673.y, 0.0f, 0.0f), float4(0.0f, 0.0f, _1673.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                    uint _1716;
                                    __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, _1716);
                                    float _1719 = 10.0f * (_1554 + _1562);
                                    uint _1721 = uint(_1719 + 640.0f);
                                    uint _1723 = _1721 + (_1721 << 10u);
                                    uint _1725 = _1723 ^ (_1723 >> 6u);
                                    uint _1727 = _1725 + (_1725 << 3u);
                                    uint _1729 = _1727 ^ (_1727 >> 11u);
                                    bool _1732 = _1662 ? _1662 : true;
                                    uint _1733 = _1662 ? _1663 : 8388607u;
                                    bool _1734 = _1664 ? _1664 : true;
                                    uint _1735 = _1664 ? _1665 : 1065353216u;
                                    uint _1741 = uint(_1719 + 987.0f);
                                    uint _1743 = _1741 + (_1741 << 10u);
                                    uint _1745 = _1743 ^ (_1743 >> 6u);
                                    uint _1747 = _1745 + (_1745 << 3u);
                                    uint _1749 = _1747 ^ (_1747 >> 11u);
                                    uint _1535 = _1732 ? _1733 : 8388607u;
                                    uint _1532 = _1734 ? _1735 : 1065353216u;
                                    uint _1757 = uint(int(_1716));
                                    float _1759[12] = { _1699[0].x, _1699[1].x, _1699[2].x, _1699[0].y, _1699[1].y, _1699[2].y, _1699[0].z, _1699[1].z, _1699[2].z, _1699[0].w, _1699[1].w, _1699[2].w };
                                    Affine3Packed _1760 = { _1759 };
                                    GardenStone _1761 = { _1760, float4(asfloat(((_1729 + (_1729 << 15u)) & _1733) | _1735) - 1.0f, asfloat(((_1749 + (_1749 << 15u)) & _1535) | _1532) - 1.0f, 0.0f, 0.0f) };
                                    [unroll]
                                    for (int _13ident = 0; _13ident < 12; _13ident++)
                                    {
                                        __alias_instance_buffer.Store<float>(_13ident * 4 + _1757 * 64 + 0, _1761.transform.data[_13ident]);
                                    }
                                    __alias_instance_buffer.Store<float4>(_1757 * 64 + 48, _1761.color);
                                    _1531 = _1532;
                                    _1534 = _1535;
                                    _1536 = _1734 ? _1734 : true;
                                    _1538 = _1732 ? _1732 : true;
                                    _1540 = _1541;
                                    continue;
                                }
                                break;
                            } while(false);
                        }
                        else
                        {
                            if (_368 == 5)
                            {
                                float2 _1769;
                                float2 _1779;
                                float2 _1818;
                                float2 _1824;
                                uint _2102;
                                uint _2103;
                                bool _2104;
                                bool _2105;
                                do
                                {
                                    _1769 = _176[1];
                                    float2 _1771 = (_176[0] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                    _1779 = (_1769 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                    float _1785;
                                    float _1788;
                                    float2 _1790;
                                    float2 _1792;
                                    _1785 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, _1771, 0.0f).y;
                                    _1788 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, _1779, 0.0f).y;
                                    _1790 = _176[0];
                                    _1792 = _1769;
                                    for (uint _1794 = 0u; _1794 < 2u; )
                                    {
                                        float2 _1798 = _1790 + _1792;
                                        float2 _1799 = _1798 * 0.5f;
                                        float _1805 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_1799 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                        float _1806 = _1805 * _1785;
                                        bool _1807 = _1806 < 0.0f;
                                        bool2 _1808 = _1807.xx;
                                        _1785 = _1807 ? _1785 : _1805;
                                        _1788 = _1807 ? _1805 : _1788;
                                        _1790 = float2(_1808.x ? _1790.x : _1799.x, _1808.y ? _1790.y : _1799.y);
                                        _1792 = float2(_1808.x ? _1799.x : _1792.x, _1808.y ? _1799.y : _1792.y);
                                        _1794++;
                                        continue;
                                    }
                                    float2 _1813 = lerp(_1792, _1790, clamp(_1788 / (_1788 - _1785), 0.0f, 1.0f).xx);
                                    float2 _1817 = float3(_1813.x, 0.4000000059604644775390625f, _1813.y).xz;
                                    _1818 = _176[3];
                                    _1824 = (_1818 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                    float _1830;
                                    float _1833;
                                    float2 _1835;
                                    float2 _1837;
                                    _1830 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, _1771, 0.0f).y;
                                    _1833 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, _1824, 0.0f).y;
                                    _1835 = _176[0];
                                    _1837 = _1818;
                                    for (uint _1839 = 0u; _1839 < 2u; )
                                    {
                                        float2 _1843 = _1835 + _1837;
                                        float2 _1844 = _1843 * 0.5f;
                                        float _1850 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_1844 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                        float _1851 = _1850 * _1830;
                                        bool _1852 = _1851 < 0.0f;
                                        bool2 _1853 = _1852.xx;
                                        _1830 = _1852 ? _1830 : _1850;
                                        _1833 = _1852 ? _1850 : _1833;
                                        _1835 = float2(_1853.x ? _1835.x : _1844.x, _1853.y ? _1835.y : _1844.y);
                                        _1837 = float2(_1853.x ? _1844.x : _1837.x, _1853.y ? _1844.y : _1837.y);
                                        _1839++;
                                        continue;
                                    }
                                    float2 _1858 = lerp(_1837, _1835, clamp(_1833 / (_1833 - _1830), 0.0f, 1.0f).xx);
                                    float2 _1862 = float3(_1858.x, 0.4000000059604644775390625f, _1858.y).xz;
                                    float _1863 = distance(_1817, _1862);
                                    if (_1863 < 0.4000000059604644775390625f)
                                    {
                                        _2102 = _159;
                                        _2103 = _159;
                                        _2104 = false;
                                        _2105 = false;
                                        break;
                                    }
                                    int _1869 = int(max(_1863 * 2.5f, 1.0f));
                                    uint _1871;
                                    uint _1874;
                                    bool _1876;
                                    bool _1878;
                                    _1871 = _159;
                                    _1874 = _159;
                                    _1876 = false;
                                    _1878 = false;
                                    for (int _1880 = 0; _1880 < (_1869 + 1); )
                                    {
                                        float _1886 = float(_1869);
                                        int _1881 = _1880 + 1;
                                        float2 _1893 = lerp(_1817, _1862, (((float(_1880) / _1886) + (float(_1881) / _1886)) * 0.5f).xx);
                                        float _1894 = _1893.x;
                                        float _1902 = _1893.y;
                                        int _1904 = _367 + (_1880 * 100);
                                        uint _1906 = uint(_1904 ^ 2346);
                                        uint _1908 = _1906 + (_1906 << 10u);
                                        uint _1910 = _1908 ^ (_1908 >> 6u);
                                        uint _1912 = _1910 + (_1910 << 3u);
                                        uint _1914 = _1912 ^ (_1912 >> 11u);
                                        uint _1916 = _1914 + (_1914 << 15u);
                                        bool _1917 = _1878 ? _1878 : true;
                                        uint _1918 = _1878 ? _1874 : 8388607u;
                                        bool _1919 = _1876 ? _1876 : true;
                                        uint _1920 = _1876 ? _1871 : 1065353216u;
                                        uint _1928 = uint(_1904 ^ 3459);
                                        uint _1930 = _1928 + (_1928 << 10u);
                                        uint _1932 = _1930 ^ (_1930 >> 6u);
                                        uint _1934 = _1932 + (_1932 << 3u);
                                        uint _1936 = _1934 ^ (_1934 >> 11u);
                                        bool _1939 = _1917 ? _1917 : true;
                                        uint _1940 = _1917 ? _1918 : 8388607u;
                                        bool _1941 = _1919 ? _1919 : true;
                                        uint _1942 = _1919 ? _1920 : 1065353216u;
                                        float _1948 = ((asfloat(((_1936 + (_1936 << 15u)) & _1940) | _1942) - 1.0f) * 2.0f) - 1.0f;
                                        uint _1950 = uint(_1904 ^ 234);
                                        uint _1952 = _1950 + (_1950 << 10u);
                                        uint _1954 = _1952 ^ (_1952 >> 6u);
                                        uint _1956 = _1954 + (_1954 << 3u);
                                        uint _1958 = _1956 ^ (_1956 >> 11u);
                                        bool _1961 = _1939 ? _1939 : true;
                                        uint _1962 = _1939 ? _1940 : 8388607u;
                                        bool _1963 = _1941 ? _1941 : true;
                                        uint _1964 = _1941 ? _1942 : 1065353216u;
                                        float3 _1971 = float3(((asfloat((_1916 & _1918) | _1920) - 1.0f) * 2.0f) - 1.0f, _1948, ((asfloat(((_1958 + (_1958 << 15u)) & _1962) | _1964) - 1.0f) * 2.0f) - 1.0f);
                                        _1971.y = _1948 * 0.20000000298023223876953125f;
                                        float3 _1974 = normalize(_1971);
                                        bool3 _1977 = (_1974.y < 0.0f).xxx;
                                        float3 _1978 = float3(_1977.x ? float3(0.0f, -1.0f, 0.0f).x : float3(0.0f, 1.0f, 0.0f).x, _1977.y ? float3(0.0f, -1.0f, 0.0f).y : float3(0.0f, 1.0f, 0.0f).y, _1977.z ? float3(0.0f, -1.0f, 0.0f).z : float3(0.0f, 1.0f, 0.0f).z);
                                        uint _1981 = uint(_1904 ^ 346);
                                        uint _1983 = _1981 + (_1981 << 10u);
                                        uint _1985 = _1983 ^ (_1983 >> 6u);
                                        uint _1987 = _1985 + (_1985 << 3u);
                                        uint _1989 = _1987 ^ (_1987 >> 11u);
                                        bool _1992 = _1961 ? _1961 : true;
                                        uint _1993 = _1961 ? _1962 : 8388607u;
                                        bool _1994 = _1963 ? _1963 : true;
                                        uint _1995 = _1963 ? _1964 : 1065353216u;
                                        bool _2002 = _1992 ? _1992 : true;
                                        uint _2003 = _1992 ? _1993 : 8388607u;
                                        bool _2004 = _1994 ? _1994 : true;
                                        uint _2005 = _1994 ? _1995 : 1065353216u;
                                        float2 _2011 = lerp(0.800000011920928955078125f, 1.5f, asfloat((_1916 & _2003) | _2005) - 1.0f).xx;
                                        float3 _2013 = float3(_2011.x, lerp(0.800000011920928955078125f, 1.2999999523162841796875f, asfloat(((_1989 + (_1989 << 15u)) & _1993) | _1995) - 1.0f).xxx.y, _2011.y) * 0.75f;
                                        float4x4 _2039 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _1894), float4(0.0f, 1.0f, 0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_1893 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x)), float4(0.0f, 0.0f, 1.0f, _1902), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(transpose(float4x4(float4(_1974, 0.0f), float4(_1978, 0.0f), float4(cross(_1974, _1978), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))), float4x4(float4(_2013.x, 0.0f, 0.0f, 0.0f), float4(0.0f, _2013.y, 0.0f, 0.0f), float4(0.0f, 0.0f, _2013.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                        uint _2056;
                                        __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, _2056);
                                        float _2059 = 10.0f * (_1894 + _1902);
                                        uint _2061 = uint(_2059 + 640.0f);
                                        uint _2063 = _2061 + (_2061 << 10u);
                                        uint _2065 = _2063 ^ (_2063 >> 6u);
                                        uint _2067 = _2065 + (_2065 << 3u);
                                        uint _2069 = _2067 ^ (_2067 >> 11u);
                                        bool _2072 = _2002 ? _2002 : true;
                                        uint _2073 = _2002 ? _2003 : 8388607u;
                                        bool _2074 = _2004 ? _2004 : true;
                                        uint _2075 = _2004 ? _2005 : 1065353216u;
                                        uint _2081 = uint(_2059 + 987.0f);
                                        uint _2083 = _2081 + (_2081 << 10u);
                                        uint _2085 = _2083 ^ (_2083 >> 6u);
                                        uint _2087 = _2085 + (_2085 << 3u);
                                        uint _2089 = _2087 ^ (_2087 >> 11u);
                                        uint _1875 = _2072 ? _2073 : 8388607u;
                                        uint _1872 = _2074 ? _2075 : 1065353216u;
                                        uint _2097 = uint(int(_2056));
                                        float _2099[12] = { _2039[0].x, _2039[1].x, _2039[2].x, _2039[0].y, _2039[1].y, _2039[2].y, _2039[0].z, _2039[1].z, _2039[2].z, _2039[0].w, _2039[1].w, _2039[2].w };
                                        Affine3Packed _2100 = { _2099 };
                                        GardenStone _2101 = { _2100, float4(asfloat(((_2069 + (_2069 << 15u)) & _2073) | _2075) - 1.0f, asfloat(((_2089 + (_2089 << 15u)) & _1875) | _1872) - 1.0f, 0.0f, 0.0f) };
                                        [unroll]
                                        for (int _14ident = 0; _14ident < 12; _14ident++)
                                        {
                                            __alias_instance_buffer.Store<float>(_14ident * 4 + _2097 * 64 + 0, _2101.transform.data[_14ident]);
                                        }
                                        __alias_instance_buffer.Store<float4>(_2097 * 64 + 48, _2101.color);
                                        _1871 = _1872;
                                        _1874 = _1875;
                                        _1876 = _2074 ? _2074 : true;
                                        _1878 = _2072 ? _2072 : true;
                                        _1880 = _1881;
                                        continue;
                                    }
                                    _2102 = _1871;
                                    _2103 = _1874;
                                    _2104 = _1876;
                                    _2105 = _1878;
                                    break;
                                } while(false);
                                do
                                {
                                    float2 _2114 = (_176[2] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                    float _2120;
                                    float _2123;
                                    float2 _2125;
                                    float2 _2127;
                                    _2120 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, _1779, 0.0f).y;
                                    _2123 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, _2114, 0.0f).y;
                                    _2125 = _1769;
                                    _2127 = _176[2];
                                    for (uint _2129 = 0u; _2129 < 2u; )
                                    {
                                        float2 _2133 = _2125 + _2127;
                                        float2 _2134 = _2133 * 0.5f;
                                        float _2140 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_2134 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                        float _2141 = _2140 * _2120;
                                        bool _2142 = _2141 < 0.0f;
                                        bool2 _2143 = _2142.xx;
                                        _2120 = _2142 ? _2120 : _2140;
                                        _2123 = _2142 ? _2140 : _2123;
                                        _2125 = float2(_2143.x ? _2125.x : _2134.x, _2143.y ? _2125.y : _2134.y);
                                        _2127 = float2(_2143.x ? _2134.x : _2127.x, _2143.y ? _2134.y : _2127.y);
                                        _2129++;
                                        continue;
                                    }
                                    float2 _2148 = lerp(_2127, _2125, clamp(_2123 / (_2123 - _2120), 0.0f, 1.0f).xx);
                                    float2 _2152 = float3(_2148.x, 0.4000000059604644775390625f, _2148.y).xz;
                                    float _2162;
                                    float _2165;
                                    float2 _2167;
                                    float2 _2169;
                                    _2162 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, _1824, 0.0f).y;
                                    _2165 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, _2114, 0.0f).y;
                                    _2167 = _1818;
                                    _2169 = _176[2];
                                    for (uint _2171 = 0u; _2171 < 2u; )
                                    {
                                        float2 _2175 = _2167 + _2169;
                                        float2 _2176 = _2175 * 0.5f;
                                        float _2182 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_2176 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                        float _2183 = _2182 * _2162;
                                        bool _2184 = _2183 < 0.0f;
                                        bool2 _2185 = _2184.xx;
                                        _2162 = _2184 ? _2162 : _2182;
                                        _2165 = _2184 ? _2182 : _2165;
                                        _2167 = float2(_2185.x ? _2167.x : _2176.x, _2185.y ? _2167.y : _2176.y);
                                        _2169 = float2(_2185.x ? _2176.x : _2169.x, _2185.y ? _2176.y : _2169.y);
                                        _2171++;
                                        continue;
                                    }
                                    float2 _2190 = lerp(_2169, _2167, clamp(_2165 / (_2165 - _2162), 0.0f, 1.0f).xx);
                                    float2 _2194 = float3(_2190.x, 0.4000000059604644775390625f, _2190.y).xz;
                                    float _2195 = distance(_2152, _2194);
                                    if (_2195 < 0.4000000059604644775390625f)
                                    {
                                        break;
                                    }
                                    int _2201 = int(max(_2195 * 2.5f, 1.0f));
                                    uint _2203 = _2102;
                                    uint _2206 = _2103;
                                    bool _2208 = _2104;
                                    bool _2210 = _2105;
                                    int _2212 = 0;
                                    for (; _2212 < (_2201 + 1); )
                                    {
                                        float _2218 = float(_2201);
                                        int _2213 = _2212 + 1;
                                        float2 _2225 = lerp(_2152, _2194, (((float(_2212) / _2218) + (float(_2213) / _2218)) * 0.5f).xx);
                                        float _2226 = _2225.x;
                                        float _2234 = _2225.y;
                                        int _2236 = _367 + (_2212 * 100);
                                        uint _2238 = uint(_2236 ^ 2346);
                                        uint _2240 = _2238 + (_2238 << 10u);
                                        uint _2242 = _2240 ^ (_2240 >> 6u);
                                        uint _2244 = _2242 + (_2242 << 3u);
                                        uint _2246 = _2244 ^ (_2244 >> 11u);
                                        uint _2248 = _2246 + (_2246 << 15u);
                                        bool _2249 = _2210 ? _2210 : true;
                                        uint _2250 = _2210 ? _2206 : 8388607u;
                                        bool _2251 = _2208 ? _2208 : true;
                                        uint _2252 = _2208 ? _2203 : 1065353216u;
                                        uint _2260 = uint(_2236 ^ 3459);
                                        uint _2262 = _2260 + (_2260 << 10u);
                                        uint _2264 = _2262 ^ (_2262 >> 6u);
                                        uint _2266 = _2264 + (_2264 << 3u);
                                        uint _2268 = _2266 ^ (_2266 >> 11u);
                                        bool _2271 = _2249 ? _2249 : true;
                                        uint _2272 = _2249 ? _2250 : 8388607u;
                                        bool _2273 = _2251 ? _2251 : true;
                                        uint _2274 = _2251 ? _2252 : 1065353216u;
                                        float _2280 = ((asfloat(((_2268 + (_2268 << 15u)) & _2272) | _2274) - 1.0f) * 2.0f) - 1.0f;
                                        uint _2282 = uint(_2236 ^ 234);
                                        uint _2284 = _2282 + (_2282 << 10u);
                                        uint _2286 = _2284 ^ (_2284 >> 6u);
                                        uint _2288 = _2286 + (_2286 << 3u);
                                        uint _2290 = _2288 ^ (_2288 >> 11u);
                                        bool _2293 = _2271 ? _2271 : true;
                                        uint _2294 = _2271 ? _2272 : 8388607u;
                                        bool _2295 = _2273 ? _2273 : true;
                                        uint _2296 = _2273 ? _2274 : 1065353216u;
                                        float3 _2303 = float3(((asfloat((_2248 & _2250) | _2252) - 1.0f) * 2.0f) - 1.0f, _2280, ((asfloat(((_2290 + (_2290 << 15u)) & _2294) | _2296) - 1.0f) * 2.0f) - 1.0f);
                                        _2303.y = _2280 * 0.20000000298023223876953125f;
                                        float3 _2306 = normalize(_2303);
                                        bool3 _2309 = (_2306.y < 0.0f).xxx;
                                        float3 _2310 = float3(_2309.x ? float3(0.0f, -1.0f, 0.0f).x : float3(0.0f, 1.0f, 0.0f).x, _2309.y ? float3(0.0f, -1.0f, 0.0f).y : float3(0.0f, 1.0f, 0.0f).y, _2309.z ? float3(0.0f, -1.0f, 0.0f).z : float3(0.0f, 1.0f, 0.0f).z);
                                        uint _2313 = uint(_2236 ^ 346);
                                        uint _2315 = _2313 + (_2313 << 10u);
                                        uint _2317 = _2315 ^ (_2315 >> 6u);
                                        uint _2319 = _2317 + (_2317 << 3u);
                                        uint _2321 = _2319 ^ (_2319 >> 11u);
                                        bool _2324 = _2293 ? _2293 : true;
                                        uint _2325 = _2293 ? _2294 : 8388607u;
                                        bool _2326 = _2295 ? _2295 : true;
                                        uint _2327 = _2295 ? _2296 : 1065353216u;
                                        bool _2334 = _2324 ? _2324 : true;
                                        uint _2335 = _2324 ? _2325 : 8388607u;
                                        bool _2336 = _2326 ? _2326 : true;
                                        uint _2337 = _2326 ? _2327 : 1065353216u;
                                        float2 _2343 = lerp(0.800000011920928955078125f, 1.5f, asfloat((_2248 & _2335) | _2337) - 1.0f).xx;
                                        float3 _2345 = float3(_2343.x, lerp(0.800000011920928955078125f, 1.2999999523162841796875f, asfloat(((_2321 + (_2321 << 15u)) & _2325) | _2327) - 1.0f).xxx.y, _2343.y) * 0.75f;
                                        float4x4 _2371 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _2226), float4(0.0f, 1.0f, 0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_2225 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x)), float4(0.0f, 0.0f, 1.0f, _2234), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(transpose(float4x4(float4(_2306, 0.0f), float4(_2310, 0.0f), float4(cross(_2306, _2310), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))), float4x4(float4(_2345.x, 0.0f, 0.0f, 0.0f), float4(0.0f, _2345.y, 0.0f, 0.0f), float4(0.0f, 0.0f, _2345.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                        uint _2388;
                                        __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, _2388);
                                        float _2391 = 10.0f * (_2226 + _2234);
                                        uint _2393 = uint(_2391 + 640.0f);
                                        uint _2395 = _2393 + (_2393 << 10u);
                                        uint _2397 = _2395 ^ (_2395 >> 6u);
                                        uint _2399 = _2397 + (_2397 << 3u);
                                        uint _2401 = _2399 ^ (_2399 >> 11u);
                                        bool _2404 = _2334 ? _2334 : true;
                                        uint _2405 = _2334 ? _2335 : 8388607u;
                                        bool _2406 = _2336 ? _2336 : true;
                                        uint _2407 = _2336 ? _2337 : 1065353216u;
                                        uint _2413 = uint(_2391 + 987.0f);
                                        uint _2415 = _2413 + (_2413 << 10u);
                                        uint _2417 = _2415 ^ (_2415 >> 6u);
                                        uint _2419 = _2417 + (_2417 << 3u);
                                        uint _2421 = _2419 ^ (_2419 >> 11u);
                                        uint _2207 = _2404 ? _2405 : 8388607u;
                                        uint _2204 = _2406 ? _2407 : 1065353216u;
                                        uint _2429 = uint(int(_2388));
                                        float _2431[12] = { _2371[0].x, _2371[1].x, _2371[2].x, _2371[0].y, _2371[1].y, _2371[2].y, _2371[0].z, _2371[1].z, _2371[2].z, _2371[0].w, _2371[1].w, _2371[2].w };
                                        Affine3Packed _2432 = { _2431 };
                                        GardenStone _2433 = { _2432, float4(asfloat(((_2401 + (_2401 << 15u)) & _2405) | _2407) - 1.0f, asfloat(((_2421 + (_2421 << 15u)) & _2207) | _2204) - 1.0f, 0.0f, 0.0f) };
                                        [unroll]
                                        for (int _15ident = 0; _15ident < 12; _15ident++)
                                        {
                                            __alias_instance_buffer.Store<float>(_15ident * 4 + _2429 * 64 + 0, _2433.transform.data[_15ident]);
                                        }
                                        __alias_instance_buffer.Store<float4>(_2429 * 64 + 48, _2433.color);
                                        _2203 = _2204;
                                        _2206 = _2207;
                                        _2208 = _2406 ? _2406 : true;
                                        _2210 = _2404 ? _2404 : true;
                                        _2212 = _2213;
                                        continue;
                                    }
                                    break;
                                } while(false);
                            }
                            else
                            {
                                bool _2439;
                                if (!(_368 == 6))
                                {
                                    _2439 = _368 == 9;
                                }
                                else
                                {
                                    _2439 = true;
                                }
                                if (_2439)
                                {
                                    do
                                    {
                                        float _2462;
                                        float _2465;
                                        float2 _2467;
                                        float2 _2469;
                                        _2462 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_176[0] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                        _2465 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_176[1] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                        _2467 = _176[0];
                                        _2469 = _176[1];
                                        for (uint _2471 = 0u; _2471 < 2u; )
                                        {
                                            float2 _2475 = _2467 + _2469;
                                            float2 _2476 = _2475 * 0.5f;
                                            float _2482 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_2476 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                            float _2483 = _2482 * _2462;
                                            bool _2484 = _2483 < 0.0f;
                                            bool2 _2485 = _2484.xx;
                                            _2462 = _2484 ? _2462 : _2482;
                                            _2465 = _2484 ? _2482 : _2465;
                                            _2467 = float2(_2485.x ? _2467.x : _2476.x, _2485.y ? _2467.y : _2476.y);
                                            _2469 = float2(_2485.x ? _2476.x : _2469.x, _2485.y ? _2476.y : _2469.y);
                                            _2471++;
                                            continue;
                                        }
                                        float2 _2490 = lerp(_2469, _2467, clamp(_2465 / (_2465 - _2462), 0.0f, 1.0f).xx);
                                        float2 _2494 = float3(_2490.x, 0.4000000059604644775390625f, _2490.y).xz;
                                        float _2510;
                                        float _2513;
                                        float2 _2515;
                                        float2 _2517;
                                        _2510 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_176[3] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                        _2513 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_176[2] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                        _2515 = _176[3];
                                        _2517 = _176[2];
                                        for (uint _2519 = 0u; _2519 < 2u; )
                                        {
                                            float2 _2523 = _2515 + _2517;
                                            float2 _2524 = _2523 * 0.5f;
                                            float _2530 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_2524 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                            float _2531 = _2530 * _2510;
                                            bool _2532 = _2531 < 0.0f;
                                            bool2 _2533 = _2532.xx;
                                            _2510 = _2532 ? _2510 : _2530;
                                            _2513 = _2532 ? _2530 : _2513;
                                            _2515 = float2(_2533.x ? _2515.x : _2524.x, _2533.y ? _2515.y : _2524.y);
                                            _2517 = float2(_2533.x ? _2524.x : _2517.x, _2533.y ? _2524.y : _2517.y);
                                            _2519++;
                                            continue;
                                        }
                                        float2 _2538 = lerp(_2517, _2515, clamp(_2513 / (_2513 - _2510), 0.0f, 1.0f).xx);
                                        float2 _2542 = float3(_2538.x, 0.4000000059604644775390625f, _2538.y).xz;
                                        float _2543 = distance(_2494, _2542);
                                        if (_2543 < 0.4000000059604644775390625f)
                                        {
                                            break;
                                        }
                                        int _2549 = int(max(_2543 * 2.5f, 1.0f));
                                        uint _2551 = 0u;
                                        uint _2554 = 0u;
                                        bool _2556 = false;
                                        bool _2558 = false;
                                        int _2560 = 0;
                                        for (; _2560 < (_2549 + 1); )
                                        {
                                            float _2566 = float(_2549);
                                            int _2561 = _2560 + 1;
                                            float2 _2573 = lerp(_2494, _2542, (((float(_2560) / _2566) + (float(_2561) / _2566)) * 0.5f).xx);
                                            float _2574 = _2573.x;
                                            float _2582 = _2573.y;
                                            int _2584 = _367 + (_2560 * 100);
                                            uint _2586 = uint(_2584 ^ 2346);
                                            uint _2588 = _2586 + (_2586 << 10u);
                                            uint _2590 = _2588 ^ (_2588 >> 6u);
                                            uint _2592 = _2590 + (_2590 << 3u);
                                            uint _2594 = _2592 ^ (_2592 >> 11u);
                                            uint _2596 = _2594 + (_2594 << 15u);
                                            bool _2597 = _2558 ? _2558 : true;
                                            uint _2598 = _2558 ? _2554 : 8388607u;
                                            bool _2599 = _2556 ? _2556 : true;
                                            uint _2600 = _2556 ? _2551 : 1065353216u;
                                            uint _2608 = uint(_2584 ^ 3459);
                                            uint _2610 = _2608 + (_2608 << 10u);
                                            uint _2612 = _2610 ^ (_2610 >> 6u);
                                            uint _2614 = _2612 + (_2612 << 3u);
                                            uint _2616 = _2614 ^ (_2614 >> 11u);
                                            bool _2619 = _2597 ? _2597 : true;
                                            uint _2620 = _2597 ? _2598 : 8388607u;
                                            bool _2621 = _2599 ? _2599 : true;
                                            uint _2622 = _2599 ? _2600 : 1065353216u;
                                            float _2628 = ((asfloat(((_2616 + (_2616 << 15u)) & _2620) | _2622) - 1.0f) * 2.0f) - 1.0f;
                                            uint _2630 = uint(_2584 ^ 234);
                                            uint _2632 = _2630 + (_2630 << 10u);
                                            uint _2634 = _2632 ^ (_2632 >> 6u);
                                            uint _2636 = _2634 + (_2634 << 3u);
                                            uint _2638 = _2636 ^ (_2636 >> 11u);
                                            bool _2641 = _2619 ? _2619 : true;
                                            uint _2642 = _2619 ? _2620 : 8388607u;
                                            bool _2643 = _2621 ? _2621 : true;
                                            uint _2644 = _2621 ? _2622 : 1065353216u;
                                            float3 _2651 = float3(((asfloat((_2596 & _2598) | _2600) - 1.0f) * 2.0f) - 1.0f, _2628, ((asfloat(((_2638 + (_2638 << 15u)) & _2642) | _2644) - 1.0f) * 2.0f) - 1.0f);
                                            _2651.y = _2628 * 0.20000000298023223876953125f;
                                            float3 _2654 = normalize(_2651);
                                            bool3 _2657 = (_2654.y < 0.0f).xxx;
                                            float3 _2658 = float3(_2657.x ? float3(0.0f, -1.0f, 0.0f).x : float3(0.0f, 1.0f, 0.0f).x, _2657.y ? float3(0.0f, -1.0f, 0.0f).y : float3(0.0f, 1.0f, 0.0f).y, _2657.z ? float3(0.0f, -1.0f, 0.0f).z : float3(0.0f, 1.0f, 0.0f).z);
                                            uint _2661 = uint(_2584 ^ 346);
                                            uint _2663 = _2661 + (_2661 << 10u);
                                            uint _2665 = _2663 ^ (_2663 >> 6u);
                                            uint _2667 = _2665 + (_2665 << 3u);
                                            uint _2669 = _2667 ^ (_2667 >> 11u);
                                            bool _2672 = _2641 ? _2641 : true;
                                            uint _2673 = _2641 ? _2642 : 8388607u;
                                            bool _2674 = _2643 ? _2643 : true;
                                            uint _2675 = _2643 ? _2644 : 1065353216u;
                                            bool _2682 = _2672 ? _2672 : true;
                                            uint _2683 = _2672 ? _2673 : 8388607u;
                                            bool _2684 = _2674 ? _2674 : true;
                                            uint _2685 = _2674 ? _2675 : 1065353216u;
                                            float2 _2691 = lerp(0.800000011920928955078125f, 1.5f, asfloat((_2596 & _2683) | _2685) - 1.0f).xx;
                                            float3 _2693 = float3(_2691.x, lerp(0.800000011920928955078125f, 1.2999999523162841796875f, asfloat(((_2669 + (_2669 << 15u)) & _2673) | _2675) - 1.0f).xxx.y, _2691.y) * 0.75f;
                                            float4x4 _2719 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _2574), float4(0.0f, 1.0f, 0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_2573 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x)), float4(0.0f, 0.0f, 1.0f, _2582), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(transpose(float4x4(float4(_2654, 0.0f), float4(_2658, 0.0f), float4(cross(_2654, _2658), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))), float4x4(float4(_2693.x, 0.0f, 0.0f, 0.0f), float4(0.0f, _2693.y, 0.0f, 0.0f), float4(0.0f, 0.0f, _2693.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                            uint _2736;
                                            __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, _2736);
                                            float _2739 = 10.0f * (_2574 + _2582);
                                            uint _2741 = uint(_2739 + 640.0f);
                                            uint _2743 = _2741 + (_2741 << 10u);
                                            uint _2745 = _2743 ^ (_2743 >> 6u);
                                            uint _2747 = _2745 + (_2745 << 3u);
                                            uint _2749 = _2747 ^ (_2747 >> 11u);
                                            bool _2752 = _2682 ? _2682 : true;
                                            uint _2753 = _2682 ? _2683 : 8388607u;
                                            bool _2754 = _2684 ? _2684 : true;
                                            uint _2755 = _2684 ? _2685 : 1065353216u;
                                            uint _2761 = uint(_2739 + 987.0f);
                                            uint _2763 = _2761 + (_2761 << 10u);
                                            uint _2765 = _2763 ^ (_2763 >> 6u);
                                            uint _2767 = _2765 + (_2765 << 3u);
                                            uint _2769 = _2767 ^ (_2767 >> 11u);
                                            uint _2555 = _2752 ? _2753 : 8388607u;
                                            uint _2552 = _2754 ? _2755 : 1065353216u;
                                            uint _2777 = uint(int(_2736));
                                            float _2779[12] = { _2719[0].x, _2719[1].x, _2719[2].x, _2719[0].y, _2719[1].y, _2719[2].y, _2719[0].z, _2719[1].z, _2719[2].z, _2719[0].w, _2719[1].w, _2719[2].w };
                                            Affine3Packed _2780 = { _2779 };
                                            GardenStone _2781 = { _2780, float4(asfloat(((_2749 + (_2749 << 15u)) & _2753) | _2755) - 1.0f, asfloat(((_2769 + (_2769 << 15u)) & _2555) | _2552) - 1.0f, 0.0f, 0.0f) };
                                            [unroll]
                                            for (int _16ident = 0; _16ident < 12; _16ident++)
                                            {
                                                __alias_instance_buffer.Store<float>(_16ident * 4 + _2777 * 64 + 0, _2781.transform.data[_16ident]);
                                            }
                                            __alias_instance_buffer.Store<float4>(_2777 * 64 + 48, _2781.color);
                                            _2551 = _2552;
                                            _2554 = _2555;
                                            _2556 = _2754 ? _2754 : true;
                                            _2558 = _2752 ? _2752 : true;
                                            _2560 = _2561;
                                            continue;
                                        }
                                        break;
                                    } while(false);
                                }
                                else
                                {
                                    bool _2787;
                                    if (!(_368 == 7))
                                    {
                                        _2787 = _368 == 8;
                                    }
                                    else
                                    {
                                        _2787 = true;
                                    }
                                    if (_2787)
                                    {
                                        do
                                        {
                                            float2 _2796 = (_176[0] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                            float _2810;
                                            float _2813;
                                            float2 _2815;
                                            float2 _2817;
                                            _2810 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, _2796, 0.0f).y;
                                            _2813 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_176[1] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                            _2815 = _176[0];
                                            _2817 = _176[1];
                                            for (uint _2819 = 0u; _2819 < 2u; )
                                            {
                                                float2 _2823 = _2815 + _2817;
                                                float2 _2824 = _2823 * 0.5f;
                                                float _2830 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_2824 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                                float _2831 = _2830 * _2810;
                                                bool _2832 = _2831 < 0.0f;
                                                bool2 _2833 = _2832.xx;
                                                _2810 = _2832 ? _2810 : _2830;
                                                _2813 = _2832 ? _2830 : _2813;
                                                _2815 = float2(_2833.x ? _2815.x : _2824.x, _2833.y ? _2815.y : _2824.y);
                                                _2817 = float2(_2833.x ? _2824.x : _2817.x, _2833.y ? _2824.y : _2817.y);
                                                _2819++;
                                                continue;
                                            }
                                            float2 _2838 = lerp(_2817, _2815, clamp(_2813 / (_2813 - _2810), 0.0f, 1.0f).xx);
                                            float2 _2842 = float3(_2838.x, 0.4000000059604644775390625f, _2838.y).xz;
                                            float _2855;
                                            float _2858;
                                            float2 _2860;
                                            float2 _2862;
                                            _2855 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, _2796, 0.0f).y;
                                            _2858 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_176[3] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                            _2860 = _176[0];
                                            _2862 = _176[3];
                                            for (uint _2864 = 0u; _2864 < 2u; )
                                            {
                                                float2 _2868 = _2860 + _2862;
                                                float2 _2869 = _2868 * 0.5f;
                                                float _2875 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_2869 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                                float _2876 = _2875 * _2855;
                                                bool _2877 = _2876 < 0.0f;
                                                bool2 _2878 = _2877.xx;
                                                _2855 = _2877 ? _2855 : _2875;
                                                _2858 = _2877 ? _2875 : _2858;
                                                _2860 = float2(_2878.x ? _2860.x : _2869.x, _2878.y ? _2860.y : _2869.y);
                                                _2862 = float2(_2878.x ? _2869.x : _2862.x, _2878.y ? _2869.y : _2862.y);
                                                _2864++;
                                                continue;
                                            }
                                            float2 _2883 = lerp(_2862, _2860, clamp(_2858 / (_2858 - _2855), 0.0f, 1.0f).xx);
                                            float2 _2887 = float3(_2883.x, 0.4000000059604644775390625f, _2883.y).xz;
                                            float _2888 = distance(_2842, _2887);
                                            if (_2888 < 0.4000000059604644775390625f)
                                            {
                                                break;
                                            }
                                            int _2894 = int(max(_2888 * 2.5f, 1.0f));
                                            uint _2896 = 0u;
                                            uint _2899 = 0u;
                                            bool _2901 = false;
                                            bool _2903 = false;
                                            int _2905 = 0;
                                            for (; _2905 < (_2894 + 1); )
                                            {
                                                float _2911 = float(_2894);
                                                int _2906 = _2905 + 1;
                                                float2 _2918 = lerp(_2842, _2887, (((float(_2905) / _2911) + (float(_2906) / _2911)) * 0.5f).xx);
                                                float _2919 = _2918.x;
                                                float _2927 = _2918.y;
                                                int _2929 = _367 + (_2905 * 100);
                                                uint _2931 = uint(_2929 ^ 2346);
                                                uint _2933 = _2931 + (_2931 << 10u);
                                                uint _2935 = _2933 ^ (_2933 >> 6u);
                                                uint _2937 = _2935 + (_2935 << 3u);
                                                uint _2939 = _2937 ^ (_2937 >> 11u);
                                                uint _2941 = _2939 + (_2939 << 15u);
                                                bool _2942 = _2903 ? _2903 : true;
                                                uint _2943 = _2903 ? _2899 : 8388607u;
                                                bool _2944 = _2901 ? _2901 : true;
                                                uint _2945 = _2901 ? _2896 : 1065353216u;
                                                uint _2953 = uint(_2929 ^ 3459);
                                                uint _2955 = _2953 + (_2953 << 10u);
                                                uint _2957 = _2955 ^ (_2955 >> 6u);
                                                uint _2959 = _2957 + (_2957 << 3u);
                                                uint _2961 = _2959 ^ (_2959 >> 11u);
                                                bool _2964 = _2942 ? _2942 : true;
                                                uint _2965 = _2942 ? _2943 : 8388607u;
                                                bool _2966 = _2944 ? _2944 : true;
                                                uint _2967 = _2944 ? _2945 : 1065353216u;
                                                float _2973 = ((asfloat(((_2961 + (_2961 << 15u)) & _2965) | _2967) - 1.0f) * 2.0f) - 1.0f;
                                                uint _2975 = uint(_2929 ^ 234);
                                                uint _2977 = _2975 + (_2975 << 10u);
                                                uint _2979 = _2977 ^ (_2977 >> 6u);
                                                uint _2981 = _2979 + (_2979 << 3u);
                                                uint _2983 = _2981 ^ (_2981 >> 11u);
                                                bool _2986 = _2964 ? _2964 : true;
                                                uint _2987 = _2964 ? _2965 : 8388607u;
                                                bool _2988 = _2966 ? _2966 : true;
                                                uint _2989 = _2966 ? _2967 : 1065353216u;
                                                float3 _2996 = float3(((asfloat((_2941 & _2943) | _2945) - 1.0f) * 2.0f) - 1.0f, _2973, ((asfloat(((_2983 + (_2983 << 15u)) & _2987) | _2989) - 1.0f) * 2.0f) - 1.0f);
                                                _2996.y = _2973 * 0.20000000298023223876953125f;
                                                float3 _2999 = normalize(_2996);
                                                bool3 _3002 = (_2999.y < 0.0f).xxx;
                                                float3 _3003 = float3(_3002.x ? float3(0.0f, -1.0f, 0.0f).x : float3(0.0f, 1.0f, 0.0f).x, _3002.y ? float3(0.0f, -1.0f, 0.0f).y : float3(0.0f, 1.0f, 0.0f).y, _3002.z ? float3(0.0f, -1.0f, 0.0f).z : float3(0.0f, 1.0f, 0.0f).z);
                                                uint _3006 = uint(_2929 ^ 346);
                                                uint _3008 = _3006 + (_3006 << 10u);
                                                uint _3010 = _3008 ^ (_3008 >> 6u);
                                                uint _3012 = _3010 + (_3010 << 3u);
                                                uint _3014 = _3012 ^ (_3012 >> 11u);
                                                bool _3017 = _2986 ? _2986 : true;
                                                uint _3018 = _2986 ? _2987 : 8388607u;
                                                bool _3019 = _2988 ? _2988 : true;
                                                uint _3020 = _2988 ? _2989 : 1065353216u;
                                                bool _3027 = _3017 ? _3017 : true;
                                                uint _3028 = _3017 ? _3018 : 8388607u;
                                                bool _3029 = _3019 ? _3019 : true;
                                                uint _3030 = _3019 ? _3020 : 1065353216u;
                                                float2 _3036 = lerp(0.800000011920928955078125f, 1.5f, asfloat((_2941 & _3028) | _3030) - 1.0f).xx;
                                                float3 _3038 = float3(_3036.x, lerp(0.800000011920928955078125f, 1.2999999523162841796875f, asfloat(((_3014 + (_3014 << 15u)) & _3018) | _3020) - 1.0f).xxx.y, _3036.y) * 0.75f;
                                                float4x4 _3064 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _2919), float4(0.0f, 1.0f, 0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_2918 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x)), float4(0.0f, 0.0f, 1.0f, _2927), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(transpose(float4x4(float4(_2999, 0.0f), float4(_3003, 0.0f), float4(cross(_2999, _3003), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))), float4x4(float4(_3038.x, 0.0f, 0.0f, 0.0f), float4(0.0f, _3038.y, 0.0f, 0.0f), float4(0.0f, 0.0f, _3038.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                                uint _3081;
                                                __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, _3081);
                                                float _3084 = 10.0f * (_2919 + _2927);
                                                uint _3086 = uint(_3084 + 640.0f);
                                                uint _3088 = _3086 + (_3086 << 10u);
                                                uint _3090 = _3088 ^ (_3088 >> 6u);
                                                uint _3092 = _3090 + (_3090 << 3u);
                                                uint _3094 = _3092 ^ (_3092 >> 11u);
                                                bool _3097 = _3027 ? _3027 : true;
                                                uint _3098 = _3027 ? _3028 : 8388607u;
                                                bool _3099 = _3029 ? _3029 : true;
                                                uint _3100 = _3029 ? _3030 : 1065353216u;
                                                uint _3106 = uint(_3084 + 987.0f);
                                                uint _3108 = _3106 + (_3106 << 10u);
                                                uint _3110 = _3108 ^ (_3108 >> 6u);
                                                uint _3112 = _3110 + (_3110 << 3u);
                                                uint _3114 = _3112 ^ (_3112 >> 11u);
                                                uint _2900 = _3097 ? _3098 : 8388607u;
                                                uint _2897 = _3099 ? _3100 : 1065353216u;
                                                uint _3122 = uint(int(_3081));
                                                float _3124[12] = { _3064[0].x, _3064[1].x, _3064[2].x, _3064[0].y, _3064[1].y, _3064[2].y, _3064[0].z, _3064[1].z, _3064[2].z, _3064[0].w, _3064[1].w, _3064[2].w };
                                                Affine3Packed _3125 = { _3124 };
                                                GardenStone _3126 = { _3125, float4(asfloat(((_3094 + (_3094 << 15u)) & _3098) | _3100) - 1.0f, asfloat(((_3114 + (_3114 << 15u)) & _2900) | _2897) - 1.0f, 0.0f, 0.0f) };
                                                [unroll]
                                                for (int _17ident = 0; _17ident < 12; _17ident++)
                                                {
                                                    __alias_instance_buffer.Store<float>(_17ident * 4 + _3122 * 64 + 0, _3126.transform.data[_17ident]);
                                                }
                                                __alias_instance_buffer.Store<float4>(_3122 * 64 + 48, _3126.color);
                                                _2896 = _2897;
                                                _2899 = _2900;
                                                _2901 = _3099 ? _3099 : true;
                                                _2903 = _3097 ? _3097 : true;
                                                _2905 = _2906;
                                                continue;
                                            }
                                            break;
                                        } while(false);
                                    }
                                    else
                                    {
                                        if (_368 == 10)
                                        {
                                            float2 _3132;
                                            float2 _3135;
                                            float2 _3182;
                                            float2 _3188;
                                            uint _3466;
                                            uint _3467;
                                            bool _3468;
                                            bool _3469;
                                            do
                                            {
                                                _3132 = _176[0];
                                                _3135 = (_3132 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                                float2 _3143 = (_176[1] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                                float _3149;
                                                float _3152;
                                                float2 _3154;
                                                float2 _3156;
                                                _3149 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, _3135, 0.0f).y;
                                                _3152 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, _3143, 0.0f).y;
                                                _3154 = _3132;
                                                _3156 = _176[1];
                                                for (uint _3158 = 0u; _3158 < 2u; )
                                                {
                                                    float2 _3162 = _3154 + _3156;
                                                    float2 _3163 = _3162 * 0.5f;
                                                    float _3169 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_3163 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                                    float _3170 = _3169 * _3149;
                                                    bool _3171 = _3170 < 0.0f;
                                                    bool2 _3172 = _3171.xx;
                                                    _3149 = _3171 ? _3149 : _3169;
                                                    _3152 = _3171 ? _3169 : _3152;
                                                    _3154 = float2(_3172.x ? _3154.x : _3163.x, _3172.y ? _3154.y : _3163.y);
                                                    _3156 = float2(_3172.x ? _3163.x : _3156.x, _3172.y ? _3163.y : _3156.y);
                                                    _3158++;
                                                    continue;
                                                }
                                                float2 _3177 = lerp(_3156, _3154, clamp(_3152 / (_3152 - _3149), 0.0f, 1.0f).xx);
                                                float2 _3181 = float3(_3177.x, 0.4000000059604644775390625f, _3177.y).xz;
                                                _3182 = _176[2];
                                                _3188 = (_3182 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                                float _3194;
                                                float _3197;
                                                float2 _3199;
                                                float2 _3201;
                                                _3194 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, _3143, 0.0f).y;
                                                _3197 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, _3188, 0.0f).y;
                                                _3199 = _176[1];
                                                _3201 = _3182;
                                                for (uint _3203 = 0u; _3203 < 2u; )
                                                {
                                                    float2 _3207 = _3199 + _3201;
                                                    float2 _3208 = _3207 * 0.5f;
                                                    float _3214 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_3208 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                                    float _3215 = _3214 * _3194;
                                                    bool _3216 = _3215 < 0.0f;
                                                    bool2 _3217 = _3216.xx;
                                                    _3194 = _3216 ? _3194 : _3214;
                                                    _3197 = _3216 ? _3214 : _3197;
                                                    _3199 = float2(_3217.x ? _3199.x : _3208.x, _3217.y ? _3199.y : _3208.y);
                                                    _3201 = float2(_3217.x ? _3208.x : _3201.x, _3217.y ? _3208.y : _3201.y);
                                                    _3203++;
                                                    continue;
                                                }
                                                float2 _3222 = lerp(_3201, _3199, clamp(_3197 / (_3197 - _3194), 0.0f, 1.0f).xx);
                                                float2 _3226 = float3(_3222.x, 0.4000000059604644775390625f, _3222.y).xz;
                                                float _3227 = distance(_3181, _3226);
                                                if (_3227 < 0.4000000059604644775390625f)
                                                {
                                                    _3466 = _159;
                                                    _3467 = _159;
                                                    _3468 = false;
                                                    _3469 = false;
                                                    break;
                                                }
                                                int _3233 = int(max(_3227 * 2.5f, 1.0f));
                                                uint _3235;
                                                uint _3238;
                                                bool _3240;
                                                bool _3242;
                                                _3235 = _159;
                                                _3238 = _159;
                                                _3240 = false;
                                                _3242 = false;
                                                for (int _3244 = 0; _3244 < (_3233 + 1); )
                                                {
                                                    float _3250 = float(_3233);
                                                    int _3245 = _3244 + 1;
                                                    float2 _3257 = lerp(_3181, _3226, (((float(_3244) / _3250) + (float(_3245) / _3250)) * 0.5f).xx);
                                                    float _3258 = _3257.x;
                                                    float _3266 = _3257.y;
                                                    int _3268 = _367 + (_3244 * 100);
                                                    uint _3270 = uint(_3268 ^ 2346);
                                                    uint _3272 = _3270 + (_3270 << 10u);
                                                    uint _3274 = _3272 ^ (_3272 >> 6u);
                                                    uint _3276 = _3274 + (_3274 << 3u);
                                                    uint _3278 = _3276 ^ (_3276 >> 11u);
                                                    uint _3280 = _3278 + (_3278 << 15u);
                                                    bool _3281 = _3242 ? _3242 : true;
                                                    uint _3282 = _3242 ? _3238 : 8388607u;
                                                    bool _3283 = _3240 ? _3240 : true;
                                                    uint _3284 = _3240 ? _3235 : 1065353216u;
                                                    uint _3292 = uint(_3268 ^ 3459);
                                                    uint _3294 = _3292 + (_3292 << 10u);
                                                    uint _3296 = _3294 ^ (_3294 >> 6u);
                                                    uint _3298 = _3296 + (_3296 << 3u);
                                                    uint _3300 = _3298 ^ (_3298 >> 11u);
                                                    bool _3303 = _3281 ? _3281 : true;
                                                    uint _3304 = _3281 ? _3282 : 8388607u;
                                                    bool _3305 = _3283 ? _3283 : true;
                                                    uint _3306 = _3283 ? _3284 : 1065353216u;
                                                    float _3312 = ((asfloat(((_3300 + (_3300 << 15u)) & _3304) | _3306) - 1.0f) * 2.0f) - 1.0f;
                                                    uint _3314 = uint(_3268 ^ 234);
                                                    uint _3316 = _3314 + (_3314 << 10u);
                                                    uint _3318 = _3316 ^ (_3316 >> 6u);
                                                    uint _3320 = _3318 + (_3318 << 3u);
                                                    uint _3322 = _3320 ^ (_3320 >> 11u);
                                                    bool _3325 = _3303 ? _3303 : true;
                                                    uint _3326 = _3303 ? _3304 : 8388607u;
                                                    bool _3327 = _3305 ? _3305 : true;
                                                    uint _3328 = _3305 ? _3306 : 1065353216u;
                                                    float3 _3335 = float3(((asfloat((_3280 & _3282) | _3284) - 1.0f) * 2.0f) - 1.0f, _3312, ((asfloat(((_3322 + (_3322 << 15u)) & _3326) | _3328) - 1.0f) * 2.0f) - 1.0f);
                                                    _3335.y = _3312 * 0.20000000298023223876953125f;
                                                    float3 _3338 = normalize(_3335);
                                                    bool3 _3341 = (_3338.y < 0.0f).xxx;
                                                    float3 _3342 = float3(_3341.x ? float3(0.0f, -1.0f, 0.0f).x : float3(0.0f, 1.0f, 0.0f).x, _3341.y ? float3(0.0f, -1.0f, 0.0f).y : float3(0.0f, 1.0f, 0.0f).y, _3341.z ? float3(0.0f, -1.0f, 0.0f).z : float3(0.0f, 1.0f, 0.0f).z);
                                                    uint _3345 = uint(_3268 ^ 346);
                                                    uint _3347 = _3345 + (_3345 << 10u);
                                                    uint _3349 = _3347 ^ (_3347 >> 6u);
                                                    uint _3351 = _3349 + (_3349 << 3u);
                                                    uint _3353 = _3351 ^ (_3351 >> 11u);
                                                    bool _3356 = _3325 ? _3325 : true;
                                                    uint _3357 = _3325 ? _3326 : 8388607u;
                                                    bool _3358 = _3327 ? _3327 : true;
                                                    uint _3359 = _3327 ? _3328 : 1065353216u;
                                                    bool _3366 = _3356 ? _3356 : true;
                                                    uint _3367 = _3356 ? _3357 : 8388607u;
                                                    bool _3368 = _3358 ? _3358 : true;
                                                    uint _3369 = _3358 ? _3359 : 1065353216u;
                                                    float2 _3375 = lerp(0.800000011920928955078125f, 1.5f, asfloat((_3280 & _3367) | _3369) - 1.0f).xx;
                                                    float3 _3377 = float3(_3375.x, lerp(0.800000011920928955078125f, 1.2999999523162841796875f, asfloat(((_3353 + (_3353 << 15u)) & _3357) | _3359) - 1.0f).xxx.y, _3375.y) * 0.75f;
                                                    float4x4 _3403 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _3258), float4(0.0f, 1.0f, 0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_3257 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x)), float4(0.0f, 0.0f, 1.0f, _3266), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(transpose(float4x4(float4(_3338, 0.0f), float4(_3342, 0.0f), float4(cross(_3338, _3342), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))), float4x4(float4(_3377.x, 0.0f, 0.0f, 0.0f), float4(0.0f, _3377.y, 0.0f, 0.0f), float4(0.0f, 0.0f, _3377.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                                    uint _3420;
                                                    __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, _3420);
                                                    float _3423 = 10.0f * (_3258 + _3266);
                                                    uint _3425 = uint(_3423 + 640.0f);
                                                    uint _3427 = _3425 + (_3425 << 10u);
                                                    uint _3429 = _3427 ^ (_3427 >> 6u);
                                                    uint _3431 = _3429 + (_3429 << 3u);
                                                    uint _3433 = _3431 ^ (_3431 >> 11u);
                                                    bool _3436 = _3366 ? _3366 : true;
                                                    uint _3437 = _3366 ? _3367 : 8388607u;
                                                    bool _3438 = _3368 ? _3368 : true;
                                                    uint _3439 = _3368 ? _3369 : 1065353216u;
                                                    uint _3445 = uint(_3423 + 987.0f);
                                                    uint _3447 = _3445 + (_3445 << 10u);
                                                    uint _3449 = _3447 ^ (_3447 >> 6u);
                                                    uint _3451 = _3449 + (_3449 << 3u);
                                                    uint _3453 = _3451 ^ (_3451 >> 11u);
                                                    uint _3239 = _3436 ? _3437 : 8388607u;
                                                    uint _3236 = _3438 ? _3439 : 1065353216u;
                                                    uint _3461 = uint(int(_3420));
                                                    float _3463[12] = { _3403[0].x, _3403[1].x, _3403[2].x, _3403[0].y, _3403[1].y, _3403[2].y, _3403[0].z, _3403[1].z, _3403[2].z, _3403[0].w, _3403[1].w, _3403[2].w };
                                                    Affine3Packed _3464 = { _3463 };
                                                    GardenStone _3465 = { _3464, float4(asfloat(((_3433 + (_3433 << 15u)) & _3437) | _3439) - 1.0f, asfloat(((_3453 + (_3453 << 15u)) & _3239) | _3236) - 1.0f, 0.0f, 0.0f) };
                                                    [unroll]
                                                    for (int _18ident = 0; _18ident < 12; _18ident++)
                                                    {
                                                        __alias_instance_buffer.Store<float>(_18ident * 4 + _3461 * 64 + 0, _3465.transform.data[_18ident]);
                                                    }
                                                    __alias_instance_buffer.Store<float4>(_3461 * 64 + 48, _3465.color);
                                                    _3235 = _3236;
                                                    _3238 = _3239;
                                                    _3240 = _3438 ? _3438 : true;
                                                    _3242 = _3436 ? _3436 : true;
                                                    _3244 = _3245;
                                                    continue;
                                                }
                                                _3466 = _3235;
                                                _3467 = _3238;
                                                _3468 = _3240;
                                                _3469 = _3242;
                                                break;
                                            } while(false);
                                            do
                                            {
                                                float2 _3478 = (_176[3] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                                float _3484;
                                                float _3487;
                                                float2 _3489;
                                                float2 _3491;
                                                _3484 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, _3135, 0.0f).y;
                                                _3487 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, _3478, 0.0f).y;
                                                _3489 = _3132;
                                                _3491 = _176[3];
                                                for (uint _3493 = 0u; _3493 < 2u; )
                                                {
                                                    float2 _3497 = _3489 + _3491;
                                                    float2 _3498 = _3497 * 0.5f;
                                                    float _3504 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_3498 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                                    float _3505 = _3504 * _3484;
                                                    bool _3506 = _3505 < 0.0f;
                                                    bool2 _3507 = _3506.xx;
                                                    _3484 = _3506 ? _3484 : _3504;
                                                    _3487 = _3506 ? _3504 : _3487;
                                                    _3489 = float2(_3507.x ? _3489.x : _3498.x, _3507.y ? _3489.y : _3498.y);
                                                    _3491 = float2(_3507.x ? _3498.x : _3491.x, _3507.y ? _3498.y : _3491.y);
                                                    _3493++;
                                                    continue;
                                                }
                                                float2 _3512 = lerp(_3491, _3489, clamp(_3487 / (_3487 - _3484), 0.0f, 1.0f).xx);
                                                float2 _3516 = float3(_3512.x, 0.4000000059604644775390625f, _3512.y).xz;
                                                float _3526;
                                                float _3529;
                                                float2 _3531;
                                                float2 _3533;
                                                _3526 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, _3478, 0.0f).y;
                                                _3529 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, _3188, 0.0f).y;
                                                _3531 = _176[3];
                                                _3533 = _3182;
                                                for (uint _3535 = 0u; _3535 < 2u; )
                                                {
                                                    float2 _3539 = _3531 + _3533;
                                                    float2 _3540 = _3539 * 0.5f;
                                                    float _3546 = (-0.4000000059604644775390625f) + garden_mask.SampleLevel(sampler_default, (_3540 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y;
                                                    float _3547 = _3546 * _3526;
                                                    bool _3548 = _3547 < 0.0f;
                                                    bool2 _3549 = _3548.xx;
                                                    _3526 = _3548 ? _3526 : _3546;
                                                    _3529 = _3548 ? _3546 : _3529;
                                                    _3531 = float2(_3549.x ? _3531.x : _3540.x, _3549.y ? _3531.y : _3540.y);
                                                    _3533 = float2(_3549.x ? _3540.x : _3533.x, _3549.y ? _3540.y : _3533.y);
                                                    _3535++;
                                                    continue;
                                                }
                                                float2 _3554 = lerp(_3533, _3531, clamp(_3529 / (_3529 - _3526), 0.0f, 1.0f).xx);
                                                float2 _3558 = float3(_3554.x, 0.4000000059604644775390625f, _3554.y).xz;
                                                float _3559 = distance(_3516, _3558);
                                                if (_3559 < 0.4000000059604644775390625f)
                                                {
                                                    break;
                                                }
                                                int _3565 = int(max(_3559 * 2.5f, 1.0f));
                                                uint _3567 = _3466;
                                                uint _3570 = _3467;
                                                bool _3572 = _3468;
                                                bool _3574 = _3469;
                                                int _3576 = 0;
                                                for (; _3576 < (_3565 + 1); )
                                                {
                                                    float _3582 = float(_3565);
                                                    int _3577 = _3576 + 1;
                                                    float2 _3589 = lerp(_3516, _3558, (((float(_3576) / _3582) + (float(_3577) / _3582)) * 0.5f).xx);
                                                    float _3590 = _3589.x;
                                                    float _3598 = _3589.y;
                                                    int _3600 = _367 + (_3576 * 100);
                                                    uint _3602 = uint(_3600 ^ 2346);
                                                    uint _3604 = _3602 + (_3602 << 10u);
                                                    uint _3606 = _3604 ^ (_3604 >> 6u);
                                                    uint _3608 = _3606 + (_3606 << 3u);
                                                    uint _3610 = _3608 ^ (_3608 >> 11u);
                                                    uint _3612 = _3610 + (_3610 << 15u);
                                                    bool _3613 = _3574 ? _3574 : true;
                                                    uint _3614 = _3574 ? _3570 : 8388607u;
                                                    bool _3615 = _3572 ? _3572 : true;
                                                    uint _3616 = _3572 ? _3567 : 1065353216u;
                                                    uint _3624 = uint(_3600 ^ 3459);
                                                    uint _3626 = _3624 + (_3624 << 10u);
                                                    uint _3628 = _3626 ^ (_3626 >> 6u);
                                                    uint _3630 = _3628 + (_3628 << 3u);
                                                    uint _3632 = _3630 ^ (_3630 >> 11u);
                                                    bool _3635 = _3613 ? _3613 : true;
                                                    uint _3636 = _3613 ? _3614 : 8388607u;
                                                    bool _3637 = _3615 ? _3615 : true;
                                                    uint _3638 = _3615 ? _3616 : 1065353216u;
                                                    float _3644 = ((asfloat(((_3632 + (_3632 << 15u)) & _3636) | _3638) - 1.0f) * 2.0f) - 1.0f;
                                                    uint _3646 = uint(_3600 ^ 234);
                                                    uint _3648 = _3646 + (_3646 << 10u);
                                                    uint _3650 = _3648 ^ (_3648 >> 6u);
                                                    uint _3652 = _3650 + (_3650 << 3u);
                                                    uint _3654 = _3652 ^ (_3652 >> 11u);
                                                    bool _3657 = _3635 ? _3635 : true;
                                                    uint _3658 = _3635 ? _3636 : 8388607u;
                                                    bool _3659 = _3637 ? _3637 : true;
                                                    uint _3660 = _3637 ? _3638 : 1065353216u;
                                                    float3 _3667 = float3(((asfloat((_3612 & _3614) | _3616) - 1.0f) * 2.0f) - 1.0f, _3644, ((asfloat(((_3654 + (_3654 << 15u)) & _3658) | _3660) - 1.0f) * 2.0f) - 1.0f);
                                                    _3667.y = _3644 * 0.20000000298023223876953125f;
                                                    float3 _3670 = normalize(_3667);
                                                    bool3 _3673 = (_3670.y < 0.0f).xxx;
                                                    float3 _3674 = float3(_3673.x ? float3(0.0f, -1.0f, 0.0f).x : float3(0.0f, 1.0f, 0.0f).x, _3673.y ? float3(0.0f, -1.0f, 0.0f).y : float3(0.0f, 1.0f, 0.0f).y, _3673.z ? float3(0.0f, -1.0f, 0.0f).z : float3(0.0f, 1.0f, 0.0f).z);
                                                    uint _3677 = uint(_3600 ^ 346);
                                                    uint _3679 = _3677 + (_3677 << 10u);
                                                    uint _3681 = _3679 ^ (_3679 >> 6u);
                                                    uint _3683 = _3681 + (_3681 << 3u);
                                                    uint _3685 = _3683 ^ (_3683 >> 11u);
                                                    bool _3688 = _3657 ? _3657 : true;
                                                    uint _3689 = _3657 ? _3658 : 8388607u;
                                                    bool _3690 = _3659 ? _3659 : true;
                                                    uint _3691 = _3659 ? _3660 : 1065353216u;
                                                    bool _3698 = _3688 ? _3688 : true;
                                                    uint _3699 = _3688 ? _3689 : 8388607u;
                                                    bool _3700 = _3690 ? _3690 : true;
                                                    uint _3701 = _3690 ? _3691 : 1065353216u;
                                                    float2 _3707 = lerp(0.800000011920928955078125f, 1.5f, asfloat((_3612 & _3699) | _3701) - 1.0f).xx;
                                                    float3 _3709 = float3(_3707.x, lerp(0.800000011920928955078125f, 1.2999999523162841796875f, asfloat(((_3685 + (_3685 << 15u)) & _3689) | _3691) - 1.0f).xxx.y, _3707.y) * 0.75f;
                                                    float4x4 _3735 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _3590), float4(0.0f, 1.0f, 0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_3589 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x)), float4(0.0f, 0.0f, 1.0f, _3598), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(transpose(float4x4(float4(_3670, 0.0f), float4(_3674, 0.0f), float4(cross(_3670, _3674), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))), float4x4(float4(_3709.x, 0.0f, 0.0f, 0.0f), float4(0.0f, _3709.y, 0.0f, 0.0f), float4(0.0f, 0.0f, _3709.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                                    uint _3752;
                                                    __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, _3752);
                                                    float _3755 = 10.0f * (_3590 + _3598);
                                                    uint _3757 = uint(_3755 + 640.0f);
                                                    uint _3759 = _3757 + (_3757 << 10u);
                                                    uint _3761 = _3759 ^ (_3759 >> 6u);
                                                    uint _3763 = _3761 + (_3761 << 3u);
                                                    uint _3765 = _3763 ^ (_3763 >> 11u);
                                                    bool _3768 = _3698 ? _3698 : true;
                                                    uint _3769 = _3698 ? _3699 : 8388607u;
                                                    bool _3770 = _3700 ? _3700 : true;
                                                    uint _3771 = _3700 ? _3701 : 1065353216u;
                                                    uint _3777 = uint(_3755 + 987.0f);
                                                    uint _3779 = _3777 + (_3777 << 10u);
                                                    uint _3781 = _3779 ^ (_3779 >> 6u);
                                                    uint _3783 = _3781 + (_3781 << 3u);
                                                    uint _3785 = _3783 ^ (_3783 >> 11u);
                                                    uint _3571 = _3768 ? _3769 : 8388607u;
                                                    uint _3568 = _3770 ? _3771 : 1065353216u;
                                                    uint _3793 = uint(int(_3752));
                                                    float _3795[12] = { _3735[0].x, _3735[1].x, _3735[2].x, _3735[0].y, _3735[1].y, _3735[2].y, _3735[0].z, _3735[1].z, _3735[2].z, _3735[0].w, _3735[1].w, _3735[2].w };
                                                    Affine3Packed _3796 = { _3795 };
                                                    GardenStone _3797 = { _3796, float4(asfloat(((_3765 + (_3765 << 15u)) & _3769) | _3771) - 1.0f, asfloat(((_3785 + (_3785 << 15u)) & _3571) | _3568) - 1.0f, 0.0f, 0.0f) };
                                                    [unroll]
                                                    for (int _19ident = 0; _19ident < 12; _19ident++)
                                                    {
                                                        __alias_instance_buffer.Store<float>(_19ident * 4 + _3793 * 64 + 0, _3797.transform.data[_19ident]);
                                                    }
                                                    __alias_instance_buffer.Store<float4>(_3793 * 64 + 48, _3797.color);
                                                    _3567 = _3568;
                                                    _3570 = _3571;
                                                    _3572 = _3770 ? _3770 : true;
                                                    _3574 = _3768 ? _3768 : true;
                                                    _3576 = _3577;
                                                    continue;
                                                }
                                                break;
                                            } while(false);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        break;
    } while(false);
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_WorkGroupID = stage_input.gl_WorkGroupID;
    gl_LocalInvocationID = stage_input.gl_LocalInvocationID;
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
