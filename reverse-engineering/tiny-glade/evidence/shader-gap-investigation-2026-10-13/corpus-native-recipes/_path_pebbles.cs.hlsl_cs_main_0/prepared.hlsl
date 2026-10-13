#define TGV_0 1
#line 1 "shaders/path_pebbles.cs.hlsl"


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
#line 6 "shaders/path_pebbles.cs.hlsl"

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
#line 7 "shaders/path_pebbles.cs.hlsl"

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
#line 8 "shaders/path_pebbles.cs.hlsl"

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
#line 9 "shaders/path_pebbles.cs.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 10 "shaders/path_pebbles.cs.hlsl"

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
#line 11 "shaders/path_pebbles.cs.hlsl"

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
#line 12 "shaders/path_pebbles.cs.hlsl"

#line 1 "shaders/inc/gaussian_rand.hlsl"












float gaussian_rand(uint seed_a, uint seed_b)
{
    float u = (rand01(hash(seed_b)) * 2.0f) - 1.0f;
    float L = log(1.0f - u * u);
    float c = 4.5472841f + 0.5f * L;
    float r = (rand01(hash(seed_a)) * 1.5f) + 0.5f;
    return r + (sqrt(sqrt((c * c) - (L * 7.142857f)) - c) * sign(u)) * 0.3f;
}
#line 13 "shaders/path_pebbles.cs.hlsl"

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
#line 14 "shaders/path_pebbles.cs.hlsl"

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
#line 15 "shaders/path_pebbles.cs.hlsl"

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
#line 16 "shaders/path_pebbles.cs.hlsl"




[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);
[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
RWStructuredBuffer<Affine3Packed> __alias_pebble_placement_buffer : register(u20, space0);

Texture2D<float4> terrain_heightmap : register(t15, space0);
Texture2D<float4> path_mask : register(t17, space0);
Texture2D<float4> rocky_terrain : register(t18, space0);
Texture2D<float4> water_mask : register(t19, space0);

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 path_mask_dims;
    uint2 rocky_terrain_dims;
    uint2 water_mask_dims;
    uint pebble_buffer_size;
    int scatter_grid_dim;
};



#line 1 "shaders/inc/scatter.hlsl"











float2 scatter_point(uint index)
{
    int grid = scatter_grid_dim;
    int area = grid * grid;
    uint cell = index % uint(area);
    float g = float(grid);
    float2 grid_pos = (float2(float(cell % uint(grid)), float(cell / uint(grid))) / g - 0.5f) * 104.0f;
    float2 jitter = float2(rand01(hash(index)),
                           rand01(hash(index ^ 188053769u))) * 6.3f;
    return grid_pos + jitter;
}



float2 scatter_cell_point(int2 cell, int seed)
{
    float2 p = float2(cell) * 2.5f;
    p += (frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f)
        * float(uint(seed) & 65535u)) + 0.5f) - 0.5f) * 2.5f;
    return p;
}
#line 46 "shaders/path_pebbles.cs.hlsl"

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
#line 47 "shaders/path_pebbles.cs.hlsl"

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
#line 48 "shaders/path_pebbles.cs.hlsl"

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
#line 49 "shaders/path_pebbles.cs.hlsl"

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
#line 50 "shaders/path_pebbles.cs.hlsl"

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
#line 51 "shaders/path_pebbles.cs.hlsl"

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
#line 52 "shaders/path_pebbles.cs.hlsl"

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
#line 53 "shaders/path_pebbles.cs.hlsl"

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
#line 54 "shaders/path_pebbles.cs.hlsl"


[numthreads(64, 1, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    int index = int(gl_GlobalInvocationID.x);
    int grid = scatter_grid_dim;
    int area = grid * grid;
    int cell = (index % area);
    float g = float(grid);
    float2 grid_pos = (float2(float((cell % grid)) / g, float(cell / grid) / g) - 0.5f) * 104.0f;

    uint iu = uint(index);
    float2 jitter = float2(rand01(hash(iu)), rand01(hash(iu ^ 188053769u))) * 6.3f;

    float px = grid_pos.x + jitter.x;
    float pz = grid_pos.y + jitter.y;

    float h = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (float3(px, 0.0f, pz).xz * 0.0076923076994717121124267578125f + 0.5f), 0.0f).x);
    float2 mask_uv = (float3(px, h, pz).xz * 0.0076923076994717121124267578125f) + 0.5f;

    if (path_mask.SampleLevel(sampler_llb, mask_uv, 0.0f).x < 0.9f)
        return;

    bool in_bounds = !(mask_uv.x < 0.0f) && !(mask_uv.x > 1.0f) && !(mask_uv.y < 0.0f) && !(mask_uv.y > 1.0f);

    float rocky = 0.0f;
    if (in_bounds)
        rocky = rocky_terrain.SampleLevel(sampler_default, mask_uv, 0.0f).x;

    float water = 0.0f;
    if (in_bounds)
        water = water_mask.SampleLevel(sampler_default, mask_uv, 0.0f).y;

    if (rocky > 0.0f || water > 0.5f)
        return;

    int seed = (int(px) ^ (345 + int(pz))) ^ 2378;

    float scale = lerp(0.4f, 1.2f, rand01(hash(uint(seed)))) * 1.2f;

    float angle = (1.0f - asfloat((hash(uint(seed)) & 0x7FFFFFu) | 0x3F800000u)) * 6.283185482025146484375f;
    float2 dir = normalize(float2(cos(angle), sin(angle)));

    float angle2 = (1.0f - asfloat((hash(uint(seed - 235)) & 0x7FFFFFu) | 0x3F800000u)) * 6.283185482025146484375f;
    float2 dir2 = normalize(float2(cos(angle2), sin(angle2)));

    float tilt_slope = rand01(hash(uint(seed + 2345))) * 0.5f;
    float3 tilt = normalize(float3(dir2.x * tilt_slope, 1.0f, dir2.y * tilt_slope));

    uint slot;
    InterlockedAdd(__alias_draw_commands[uint(draw_indirect_buffer_offset)].instanceCount, 1u, slot);

    float3 f = float3(dir.x, 0.0f, dir.y);
    float3 r = cross(f, tilt);
    float4x4 basis = float4x4(float4(r, 0.0f), float4(tilt, 0.0f), float4(f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f));
    float4x4 scale_m = float4x4(float4(scale, 0.0f, 0.0f, 0.0f), float4(0.0f, scale, 0.0f, 0.0f), float4(0.0f, 0.0f, scale, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f));
    float4x4 trans = float4x4(float4(1.0f, 0.0f, 0.0f, px), float4(0.0f, 1.0f, 0.0f, h), float4(0.0f, 0.0f, 1.0f, pz), float4(0.0f, 0.0f, 0.0f, 1.0f));

    float4x4 m = mul(mul(scale_m, transpose(basis)), trans);
    __alias_pebble_placement_buffer[slot] = pack_affine(m);
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

static float3 _103;

RWByteAddressBuffer __alias_draw_commands : register(u13, space0);
cbuffer $Globals : register(b14, space0)
{
    int _Globals_draw_indirect_buffer_offset : packoffset(c0);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c0.y);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c1);
    uint2 _Globals_path_mask_dims : packoffset(c1.z);
    uint2 _Globals_rocky_terrain_dims : packoffset(c2);
    uint2 _Globals_water_mask_dims : packoffset(c2.z);
    uint _Globals_pebble_buffer_size : packoffset(c3);
    int _Globals_scatter_grid_dim : packoffset(c3.y);
};

RWByteAddressBuffer __alias_pebble_placement_buffer : register(u20, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t15, space0);
Texture2D<float4> path_mask : register(t17, space0);
Texture2D<float4> rocky_terrain : register(t18, space0);
Texture2D<float4> water_mask : register(t19, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    do
    {
        int _112 = int(gl_GlobalInvocationID.x);
        int _113 = (_112 % (_Globals_scatter_grid_dim * _Globals_scatter_grid_dim));
        float _117 = float(_Globals_scatter_grid_dim);
        float2 _123 = (float2(float((_113 % _Globals_scatter_grid_dim)) / _117, float(_113 / _Globals_scatter_grid_dim) / _117) - 0.5f.xx) * 104.0f.xx;
        uint _126 = uint(_112);
        uint _128 = _126 + (_126 << 10u);
        uint _130 = _128 ^ (_128 >> 6u);
        uint _132 = _130 + (_130 << 3u);
        uint _134 = _132 ^ (_132 >> 11u);
        uint _141 = _126 ^ 188053769u;
        uint _143 = _141 + (_141 << 10u);
        uint _145 = _143 ^ (_143 >> 6u);
        uint _147 = _145 + (_145 << 3u);
        uint _149 = _147 ^ (_147 >> 11u);
        float2 _157 = float2(asfloat(((_134 + (_134 << 15u)) & 8388607u) | 1065353216u) - 1.0f, asfloat(((_149 + (_149 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 6.30000019073486328125f;
        float _159 = _123.x + _157.x;
        float3 _160;
        _160.x = _159;
        float _162 = _123.y + _157.y;
        _160.z = _162;
        float _172 = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_160.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
        float2 _178 = (float3(_159, _172, _162).xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        if (path_mask.SampleLevel(sampler_llb, _178, 0.0f).x < 0.89999997615814208984375f)
        {
            break;
        }
        float _187;
        bool _189;
        float _213;
        do
        {
            _187 = _178.x;
            _189 = !(_187 < 0.0f);
            bool _193;
            if (_189)
            {
                _193 = _187 > 1.0f;
            }
            else
            {
                _193 = true;
            }
            bool _199;
            if (!_193)
            {
                _199 = _178.y < 0.0f;
            }
            else
            {
                _199 = true;
            }
            bool _205;
            if (!_199)
            {
                _205 = _178.y > 1.0f;
            }
            else
            {
                _205 = true;
            }
            if (_205)
            {
                _213 = 0.0f;
                break;
            }
            else
            {
                _213 = rocky_terrain.SampleLevel(sampler_default, _178, 0.0f).x;
                break;
            }
            break; 
        } while(false);
        float _239;
        do
        {
            bool _219;
            if (_189)
            {
                _219 = _187 > 1.0f;
            }
            else
            {
                _219 = true;
            }
            bool _225;
            if (!_219)
            {
                _225 = _178.y < 0.0f;
            }
            else
            {
                _225 = true;
            }
            bool _231;
            if (!_225)
            {
                _231 = _178.y > 1.0f;
            }
            else
            {
                _231 = true;
            }
            if (_231)
            {
                _239 = 0.0f;
                break;
            }
            else
            {
                _239 = water_mask.SampleLevel(sampler_default, _178, 0.0f).y;
                break;
            }
            break; 
        } while(false);
        bool _245;
        if (!(_213 > 0.0f))
        {
            _245 = _239 > 0.5f;
        }
        else
        {
            _245 = true;
        }
        if (_245)
        {
            break;
        }
        int _252 = (int(_159) ^ (345 + int(_162))) ^ 2378;
        uint _253 = uint(_252);
        uint _255 = _253 + (_253 << 10u);
        uint _257 = _255 ^ (_255 >> 6u);
        uint _259 = _257 + (_257 << 3u);
        uint _261 = _259 ^ (_259 >> 11u);
        float _266 = asfloat(((_261 + (_261 << 15u)) & 8388607u) | 1065353216u);
        float _269 = lerp(0.4000000059604644775390625f, 1.2000000476837158203125f, _266 - 1.0f) * 1.2000000476837158203125f;
        float _271 = (1.0f - _266) * 6.280000209808349609375f;
        float2 _275 = normalize(float2(cos(_271), sin(_271)));
        float _276 = _275.x;
        float _277 = _275.y;
        uint _280 = uint(_252 - 235);
        uint _282 = _280 + (_280 << 10u);
        uint _284 = _282 ^ (_282 >> 6u);
        uint _286 = _284 + (_284 << 3u);
        uint _288 = _286 ^ (_286 >> 11u);
        float _295 = (1.0f - asfloat(((_288 + (_288 << 15u)) & 8388607u) | 1065353216u)) * 6.280000209808349609375f;
        float2 _299 = normalize(float2(cos(_295), sin(_295)));
        uint _301 = uint(_252 + 2345);
        uint _303 = _301 + (_301 << 10u);
        uint _305 = _303 ^ (_303 >> 6u);
        uint _307 = _305 + (_305 << 3u);
        uint _309 = _307 ^ (_307 >> 11u);
        float _316 = (asfloat(((_309 + (_309 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 0.5f;
        float3 _322 = normalize(float3(_299.x * _316, 1.0f, _299.y * _316));
        uint _328;
        __alias_draw_commands.InterlockedAdd(uint(_Globals_draw_indirect_buffer_offset) * 20 + 4, 1u, _328);
        uint _330 = uint(int(_328));
        float4x4 _351 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _159), float4(0.0f, 1.0f, 0.0f, _172), float4(0.0f, 0.0f, 1.0f, _162), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(transpose(float4x4(float4(cross(float3(_276, 0.0f, _277), _322), 0.0f), float4(_322, 0.0f), float4(_276, 0.0f, _277, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))), float4x4(float4(_269, 0.0f, 0.0f, 0.0f), float4(0.0f, _269, 0.0f, 0.0f), float4(0.0f, 0.0f, _269, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
        float _365[12] = { _351[0].x, _351[1].x, _351[2].x, _351[0].y, _351[1].y, _351[2].y, _351[0].z, _351[1].z, _351[2].z, _351[0].w, _351[1].w, _351[2].w };
        Affine3Packed _366 = { _365 };
        [unroll]
        for (int _2ident = 0; _2ident < 12; _2ident++)
        {
            __alias_pebble_placement_buffer.Store<float>(_2ident * 4 + _330 * 48 + 0, _366.data[_2ident]);
        }
        break;
    } while(false);
}

[numthreads(64, 1, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
