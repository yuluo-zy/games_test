#define TGV_0 1
#line 1 "shaders/water/spawn_boats.cs.hlsl"


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
#line 6 "shaders/water/spawn_boats.cs.hlsl"

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
#line 7 "shaders/water/spawn_boats.cs.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 8 "shaders/water/spawn_boats.cs.hlsl"

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
#line 9 "shaders/water/spawn_boats.cs.hlsl"

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
#line 10 "shaders/water/spawn_boats.cs.hlsl"

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
#line 11 "shaders/water/spawn_boats.cs.hlsl"

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
#line 12 "shaders/water/spawn_boats.cs.hlsl"


struct BoatXform
{
    Affine3Packed transform;
};



float4x4 rotate_axis(float3 axis, float angle)
{
    float c = cos(angle);
    float s = sin(angle);
    float omc = 1.0f - c;
    float x = axis.x;
    float y = axis.y;
    float z = axis.z;
    float cxx = omc * x;
    float cyy = omc * y;
    float czz = omc * z;
    return transpose(float4x4(
        float4(cxx * x + c, cxx * y - z * s, czz * x + y * s, 0.0f),
        float4(cxx * y + z * s, cyy * y + c, cyy * z - x * s, 0.0f),
        float4(czz * x - y * s, cyy * z + x * s, czz * z + c, 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f)));
}

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
    uint2 path_mask_dims;
    uint2 water_mask_dims;
    uint2 buildings_mask_dims;
    uint2 grid_dims;
};

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);
Texture2D<float4> path_mask : register(t15, space0);
Texture2D<float4> water_mask : register(t16, space0);
Texture2D<float4> buildings_mask : register(t17, space0);
RWStructuredBuffer<BoatXform> __alias_instance_buffer : register(u18, space0);



RWStructuredBuffer<uint> _rda_trigger[] : register(u100, space0);

groupshared uint best_choice;

[numthreads(8, 8, 1)]
void cs_main(uint3 id : SV_DispatchThreadID, uint local_index : SV_GroupIndex)
{
    if (local_index == 0u)
        best_choice = 0u;
    GroupMemoryBarrierWithGroupSync();

    float2 uv = (float2(id.xy) + 0.5f) / float2(grid_dims);
    float2 world_pos = (uv - 0.5f) * 130.0f;

    float angle = 40.665630340576171875f * float(id.x + id.y * grid_dims.x);
    float2 dir = float2(cos(angle), sin(angle));

    float2 fwd = dir * 0.016923077404499053955078125f;
    float2 side = float2(dir.y, -dir.x) * 0.003076923079788684844970703125f;
    float2 bow = uv + dir * 0.03384615480899810791015625f;

    float min_water = min(min(min(min(min(
        water_mask.SampleLevel(sampler_lnc, uv, 0.0f).x,
        water_mask.SampleLevel(sampler_lnc, uv - fwd + side, 0.0f).x),
        water_mask.SampleLevel(sampler_lnc, uv + fwd + side, 0.0f).x),
        water_mask.SampleLevel(sampler_lnc, uv - fwd - side, 0.0f).x),
        water_mask.SampleLevel(sampler_lnc, uv + fwd - side, 0.0f).x),
        water_mask.SampleLevel(sampler_lnc, uv - fwd * 1.5f, 0.0f).x);

    bool deep = water_mask.SampleLevel(sampler_lnc, bow, 0.0f).x < 0.5f ? (min_water > 0.5f) : false;

    float deep_score = deep ? min_water : 0.0f;
    float score = (abs(water_mask.SampleLevel(sampler_lnc, uv + fwd, 0.0f).w - water_mask.SampleLevel(sampler_lnc, uv - fwd, 0.0f).w) > 0.1f)
        ? 0.0f
        : ((path_mask.Load(int3(uint2(int2(uv * float2(path_mask_dims))), 0u)).x > 0.0f)
            ? 0.0f
            : deep_score * clamp(buildings_mask.SampleLevel(sampler_lnc, uv + dir * 0.06769230841702222824096679688f, 0.0f).y - 0.3f, 0.0f, 1.0f));

    uint packed = uint(clamp(score, 0.0f, 1.0f) * 4294967040.0f) | local_index;
    uint prev;
    InterlockedMax(best_choice, packed, prev);
    GroupMemoryBarrierWithGroupSync();

    bool is_winner = best_choice == packed && score > 0.0f;

    if (is_winner)
    {
        uint seed = id.x * id.y + id.x;
        float rnd = rand01(hash(seed));
        float bob = 0.05f * sin((fmod(float(game_time_sec), TWO_PI) + game_time_frac) + TWO_PI * rnd);
        float yaw = cos(angle);

        float4x4 identity = float4x4(
            float4(1.0f, 0.0f, 0.0f, 0.0f),
            float4(0.0f, 1.0f, 0.0f, 0.0f),
            float4(0.0f, 0.0f, 1.0f, 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f));
        float4x4 scale = float4x4(
            float4(0.85f, 0.0f, 0.0f, 0.0f),
            float4(0.0f, 0.85f, 0.0f, 0.0f),
            float4(0.0f, 0.0f, 0.85f, 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f));
        float4x4 m = mul(mul(mul(
            affine_translation(float3(world_pos.x, -0.68f, world_pos.y)),
            mul(identity, scale)),
            rotate_axis(normalize(normalize(float3(1.0f, 0.0f, 1.0f))), bob)),
            rotate_axis(normalize(float3(0.0f, 1.0f, 0.0f)), yaw));

        uint base;
        InterlockedAdd(__alias_draw_commands[uint(draw_indirect_buffer_offset)].instanceCount, 1u, base);
        __alias_instance_buffer[base] = BoatXform(pack_affine(m));
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

struct Affine3Packed
{
    float data[12];
};

struct BoatXform
{
    Affine3Packed transform;
};

RWByteAddressBuffer __alias_draw_commands : register(u13, space0);
cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset : packoffset(c0);
    uint2 path_mask_dims : packoffset(c0.y);
    uint2 water_mask_dims : packoffset(c1);
    uint2 buildings_mask_dims : packoffset(c1.z);
    uint2 grid_dims : packoffset(c2);
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

RWByteAddressBuffer __alias_instance_buffer : register(u18, space0);
SamplerState sampler_lnc : register(s0, space0);
Texture2D<float4> path_mask : register(t15, space0);
Texture2D<float4> water_mask : register(t16, space0);
Texture2D<float4> buildings_mask : register(t17, space0);

static uint3 gl_GlobalInvocationID;
static uint gl_LocalInvocationIndex;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
    uint gl_LocalInvocationIndex : SV_GroupIndex;
};

groupshared uint best_choice;

void comp_main()
{
    if (0u == gl_LocalInvocationIndex)
    {
        best_choice = 0u;
    }
    GroupMemoryBarrierWithGroupSync();
    float2 _135 = (float2(gl_GlobalInvocationID.xy) + 0.5f.xx) / float2(grid_dims);
    float2 _137 = (_135 - 0.5f.xx) * 130.0f.xx;
    float _150 = 40.665630340576171875f * float(gl_GlobalInvocationID.x + (gl_GlobalInvocationID.y * grid_dims.x));
    float _151 = cos(_150);
    float _152 = sin(_150);
    float2 _153 = float2(_151, _152);
    float2 _158 = _153 * 0.03384615480899810791015625f;
    float2 _167 = _135 + (_153 * (-0.016923077404499053955078125f));
    float2 _168 = float2(_152, -_151) * 0.003076923079788684844970703125f;
    float2 _174 = _135 + (_153 * 0.016923077404499053955078125f);
    float _197 = min(min(min(min(min(water_mask.SampleLevel(sampler_lnc, _135, 0.0f).x, water_mask.SampleLevel(sampler_lnc, _167 + _168, 0.0f).x), water_mask.SampleLevel(sampler_lnc, _174 + _168, 0.0f).x), water_mask.SampleLevel(sampler_lnc, _167 - _168, 0.0f).x), water_mask.SampleLevel(sampler_lnc, _174 - _168, 0.0f).x), water_mask.SampleLevel(sampler_lnc, _135 + ((_153 * (-1.5f)) * 0.016923077404499053955078125f), 0.0f).x);
    bool _202;
    if (water_mask.SampleLevel(sampler_lnc, _135 + _158, 0.0f).x < 0.5f)
    {
        _202 = _197 > 0.5f;
    }
    else
    {
        _202 = false;
    }
    float _203 = _202 ? _197 : 0.0f;
    float _228 = (abs(water_mask.SampleLevel(sampler_lnc, _174, 0.0f).w - water_mask.SampleLevel(sampler_lnc, _167, 0.0f).w) > 0.100000001490116119384765625f) ? 0.0f : ((path_mask.Load(int3(uint2(int2(_135 * float2(path_mask_dims))), 0u)).x > 0.0f) ? 0.0f : (_203 * clamp(buildings_mask.SampleLevel(sampler_lnc, _135 + (_158 * 2.0f), 0.0f).y - 0.300000011920928955078125f, 0.0f, 1.0f)));
    uint _232 = uint(clamp(_228, 0.0f, 1.0f) * 4294967040.0f) | gl_LocalInvocationIndex;
    uint _233;
    InterlockedMax(best_choice, _232, _233);
    GroupMemoryBarrierWithGroupSync();
    bool _239;
    if (best_choice == _232)
    {
        _239 = _228 > 0.0f;
    }
    else
    {
        _239 = false;
    }
    if (_239)
    {
        uint _258 = (gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) + gl_GlobalInvocationID.x;
        uint _260 = _258 + (_258 << 10u);
        uint _262 = _260 ^ (_260 >> 6u);
        uint _264 = _262 + (_262 << 3u);
        uint _266 = _264 ^ (_264 >> 11u);
        float _276 = 0.0500000007450580596923828125f * sin((fmod(float(frame_constants_game_time_sec), 6.283185482025146484375f) + frame_constants_game_time_frac) + (6.283185482025146484375f * (asfloat(((_266 + (_266 << 15u)) & 8388607u) | 1065353216u) - 1.0f)));
        float3 _277 = normalize(normalize(float3(1.0f, 0.0f, 1.0f)));
        float _278 = sin(_276);
        float _279 = cos(_276);
        float _280 = 1.0f - _279;
        float _281 = _277.x;
        float _282 = _280 * _281;
        float _285 = _277.y;
        float _286 = _282 * _285;
        float _287 = _277.z;
        float _288 = _287 * _278;
        float _290 = _280 * _287;
        float _291 = _290 * _281;
        float _292 = _285 * _278;
        float _295 = _280 * _285;
        float _298 = _295 * _287;
        float _299 = _281 * _278;
        float3 _311 = normalize(float3(0.0f, 1.0f, 0.0f));
        float _312 = 1.0f - _151;
        float _313 = _311.x;
        float _314 = _312 * _313;
        float _317 = _311.y;
        float _318 = _314 * _317;
        float _319 = _311.z;
        float _320 = _319 * _152;
        float _322 = _312 * _319;
        float _323 = _322 * _313;
        float _324 = _317 * _152;
        float _327 = _312 * _317;
        float _330 = _327 * _319;
        float _331 = _313 * _152;
        float4x4 _342 = mul(mul(mul(float4x4(float4(1.0f, 0.0f, 0.0f, _137.x), float4(0.0f, 1.0f, 0.0f, -0.680000007152557373046875f), float4(0.0f, 0.0f, 1.0f, _137.y), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(float4x4(float4(1.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 1.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4x4(float4(0.85000002384185791015625f, 0.0f, 0.0f, 0.0f), float4(0.0f, 0.85000002384185791015625f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.85000002384185791015625f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), transpose(float4x4(float4((_282 * _281) + _279, _286 - _288, _291 + _292, 0.0f), float4(_286 + _288, (_295 * _285) + _279, _298 - _299, 0.0f), float4(_291 - _292, _298 + _299, (_290 * _287) + _279, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), transpose(float4x4(float4((_314 * _313) + _151, _318 - _320, _323 + _324, 0.0f), float4(_318 + _320, (_327 * _317) + _151, _330 - _331, 0.0f), float4(_323 - _324, _330 + _331, (_322 * _319) + _151, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
        uint _359;
        __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, _359);
        uint _361 = uint(int(_359));
        float _363[12] = { _342[0].x, _342[1].x, _342[2].x, _342[0].y, _342[1].y, _342[2].y, _342[0].z, _342[1].z, _342[2].z, _342[0].w, _342[1].w, _342[2].w };
        Affine3Packed _364 = { _363 };
        BoatXform _365 = { _364 };
        [unroll]
        for (int _2ident = 0; _2ident < 12; _2ident++)
        {
            __alias_instance_buffer.Store<float>(_2ident * 4 + _361 * 48 + 0, _365.transform.data[_2ident]);
        }
    }
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
