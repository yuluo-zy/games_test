#define TGV_0 1
#line 1 "shaders/garden/spawn_bushes_buildings.cs.hlsl"


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
#line 6 "shaders/garden/spawn_bushes_buildings.cs.hlsl"

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
#line 7 "shaders/garden/spawn_bushes_buildings.cs.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 8 "shaders/garden/spawn_bushes_buildings.cs.hlsl"

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
#line 9 "shaders/garden/spawn_bushes_buildings.cs.hlsl"

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
#line 10 "shaders/garden/spawn_bushes_buildings.cs.hlsl"

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
#line 11 "shaders/garden/spawn_bushes_buildings.cs.hlsl"

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
#line 13 "shaders/garden/spawn_bushes_buildings.cs.hlsl"

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
#line 14 "shaders/garden/spawn_bushes_buildings.cs.hlsl"

#line 1 "shaders/buildings/building_mask.inc.hlsl"


#ifndef TINYGLADE_BUILDING_MASK_HLSL
#define TINYGLADE_BUILDING_MASK_HLSL



struct BuildingShape
{
    float2 center;
    float2 dims;
    float2 orientation;
    int shape_id;
    int pad0;
};



bool is_building_circle(float2 world_xz, float2 center, float radius)
{
    return distance(world_xz, center) < radius;
}




bool is_building_box(float2 world_xz, float2 center, float2 dims, float2 orientation)
{
    float2 local = mul(world_xz - center,
                       float2x2(float2(orientation.x, -orientation.y),
                                float2(orientation.y, orientation.x)));
    float half_w = (dims.x + 1.0f) * 0.5f;
    float half_h = (dims.y + 1.0f) * 0.5f;
    return abs(local.x) <= half_w && abs(local.y) <= half_h;
}


bool is_building(BuildingShape building, float2 world_xz)
{
    if (building.shape_id == 0)
    {
        return is_building_circle(world_xz, building.center, building.dims.x + 0.75f);
    }
    else if (building.shape_id == 1)
    {
        return is_building_box(world_xz, building.center, building.dims, building.orientation);
    }
    return false;
}



float get_building_mask(BuildingShape building, float2 world_xz)
{
    return is_building(building, world_xz) ? 1.0f : 0.0f;
}

#endif 
#line 15 "shaders/garden/spawn_bushes_buildings.cs.hlsl"

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
#line 16 "shaders/garden/spawn_bushes_buildings.cs.hlsl"

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
#line 17 "shaders/garden/spawn_bushes_buildings.cs.hlsl"

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
#line 18 "shaders/garden/spawn_bushes_buildings.cs.hlsl"


cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 garden_mask_dims;
    uint2 buildings_mask_dims;
    uint2 buildings_mask_non_blurred_dims;
    int2 grid_dims;
    uint2 path_mask_dims;
    uint2 rocky_terrain_dims;
    uint2 water_mask_dims;
    uint2 foliage_exclusion_mask_dims;
};

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);
Texture2D<float4> garden_mask : register(t17, space0);
Texture2D<float4> buildings_mask : register(t18, space0);
Texture2D<float4> buildings_mask_non_blurred : register(t19, space0);
Texture2D<float4> path_mask : register(t22, space0);
Texture2D<float4> rocky_terrain : register(t23, space0);
Texture2D<float4> water_mask : register(t24, space0);
Texture2D<float4> foliage_exclusion_mask : register(t25, space0);
RWStructuredBuffer<BushXform> __alias_bush_buffer : register(u20, space0);
RWStructuredBuffer<FlowerXform> __alias_bush_flowers : register(u21, space0);



RWStructuredBuffer<uint> _rda_trigger[] : register(u100, space0);

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
    if (garden_mask.Load(int3(uint2(uv * float2(path_mask_dims)), 0u)).y > 0.3f)
        return;

    float bmx = buildings_mask.SampleLevel(sampler_default, uv, 0.0f).x;
    if (buildings_mask_non_blurred.SampleLevel(sampler_default, uv, 0.0f).x > 0.999989986419677734375f)
        return;

    if (bmx > 0.0f && perlin(world_pos) > 0.5f)
    {
        float height = get_terrain_height(terrain_heightmap, sampler_default, world_pos);
        float3 pos = float3(world_pos.x, glade_theme_id == 4u ? height - 0.5f : height, world_pos.y);

        float band1 = band_falloff_garden(abs(buildings_mask.SampleLevel(sampler_default, uv, 0.0f).y - 0.4f), 3.3333332538604736328125f);
        float fs = lerp(0.8f, 1.2f, rand01(hash(id.x + id.y)));
        float3 s0 = fs.xxx * lerp(0.2f, 1.0f, sqrt(band1));
        float4x4 s = float4x4(
            float4(min(s0.x, 0.75f), 0.0f, 0.0f, 0.0f),
            float4(0.0f, min(s0.y * fs, 1.2f), 0.0f, 0.0f),
            float4(0.0f, 0.0f, min(s0.z, 0.75f), 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f));
        float3 s3 = float3(min(s0.x, 0.75f), min(s0.y * fs, 1.2f), min(s0.z, 0.75f)) * 0.75f;

        float band2 = band_falloff_garden(abs(bmx - 0.4f), 3.3333332538604736328125f);
        if (min((fs.xxx * lerp(0.2f, 1.0f, sqrt(band2))).x, 0.75f) * 0.75f < 0.3f)
            return;

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

        if (rand01(hash((id.x * 2u) + (id.y / 5u) + 123u)) > 0.7f)
        {
            uint flower_base;
            InterlockedAdd(__alias_draw_commands[11u].instanceCount, 1u, flower_base);
            FlowerXform f;
            f.transform = pack_affine(m);
            f.color = float3(-1.0f, -1.0f, -1.0f);
            f.flower_type = 2u;
            __alias_bush_flowers[flower_base] = f;
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

static float _147;

RWByteAddressBuffer __alias_draw_commands : register(u13, space0);
cbuffer $Globals : register(b14, space0)
{
    int _Globals_draw_indirect_buffer_offset : packoffset(c0);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c0.y);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c1);
    uint2 _Globals_garden_mask_dims : packoffset(c1.z);
    uint2 _Globals_buildings_mask_dims : packoffset(c2);
    uint2 _Globals_buildings_mask_non_blurred_dims : packoffset(c2.z);
    int2 _Globals_grid_dims : packoffset(c3);
    uint2 _Globals_path_mask_dims : packoffset(c3.z);
    uint2 _Globals_rocky_terrain_dims : packoffset(c4);
    uint2 _Globals_water_mask_dims : packoffset(c4.z);
    uint2 _Globals_foliage_exclusion_mask_dims : packoffset(c5);
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

RWByteAddressBuffer __alias_bush_buffer : register(u20, space0);
RWByteAddressBuffer __alias_bush_flowers : register(u21, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t15, space0);
Texture2D<float4> garden_mask : register(t17, space0);
Texture2D<float4> buildings_mask : register(t18, space0);
Texture2D<float4> buildings_mask_non_blurred : register(t19, space0);
Texture2D<float4> path_mask : register(t22, space0);
Texture2D<float4> rocky_terrain : register(t23, space0);
Texture2D<float4> water_mask : register(t24, space0);
Texture2D<float4> foliage_exclusion_mask : register(t25, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    do
    {
        uint2 _191;
        uint _165 = (gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) + gl_GlobalInvocationID.x;
        uint _167 = _165 + (_165 << 10u);
        uint _169 = _167 ^ (_167 >> 6u);
        uint _171 = _169 + (_169 << 3u);
        uint _173 = _171 ^ (_171 >> 11u);
        float _179 = asfloat(((_173 + (_173 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        float2 _182 = ((((float2(gl_GlobalInvocationID.xy) + 0.5f.xx) / float2(_Globals_grid_dims)) - 0.5f.xx) * 130.0f.xx) + lerp(-1.0f, 1.0f, _179).xx;
        float2 _184 = (_182 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        bool _252;
        do
        {
            _191 = uint2(_184 * float2(_Globals_path_mask_dims));
            float4 _209 = terrain_heightmap.GatherRed(sampler_default, _184, int2(0, 0));
            float _214 = _209.z;
            float _215 = _209.y;
            float _217 = _209.w;
            float _218 = _209.x;
            bool _239;
            if (!(path_mask.Load(int3(_191, 0u)).x > 0.20000000298023223876953125f))
            {
                _239 = (int(rocky_terrain.Load(int3(_191, 0u)).x > 0.0f) & int(length(((float2((_214 + _215) - (_217 + _218), (_218 + _215) - (_217 + _214)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(_Globals_terrain_heightmap_dims))) > 0.89999997615814208984375f)) != 0;
            }
            else
            {
                _239 = true;
            }
            bool _244;
            if (!_239)
            {
                _244 = water_mask.Load(int3(_191, 0u)).y > 0.300000011920928955078125f;
            }
            else
            {
                _244 = true;
            }
            bool _249;
            if (!_244)
            {
                _249 = foliage_exclusion_mask.SampleLevel(sampler_llb, _184, 0.0f).x > 0.0f;
            }
            else
            {
                _249 = true;
            }
            if (_249)
            {
                _252 = true;
                break;
            }
            _252 = false;
            break;
        } while(false);
        if (_252)
        {
            break;
        }
        if (garden_mask.Load(int3(_191, 0u)).y > 0.300000011920928955078125f)
        {
            break;
        }
        float4 _263 = buildings_mask.SampleLevel(sampler_default, _184, 0.0f);
        float _264 = _263.x;
        if (buildings_mask_non_blurred.SampleLevel(sampler_default, _184, 0.0f).x > 0.999989986419677734375f)
        {
            break;
        }
        bool _425;
        if (_264 > 0.0f)
        {
            int2 _277 = int2(floor(_182));
            float2 _278 = frac(_182);
            uint2 _279 = uint2(_277);
            uint _281 = _279.y;
            uint _283 = _281 + (_281 << 10u);
            uint _285 = _283 ^ (_283 >> 6u);
            uint _287 = _285 + (_285 << 3u);
            uint _289 = _287 ^ (_287 >> 11u);
            uint _295 = (((_279.x * 1664525u) + (_289 + (_289 << 15u))) + 1013904223u) * 1664525u;
            uint _297 = _295 ^ (_295 >> 11u);
            uint _300 = _297 ^ ((_297 << 7u) & 2636928640u);
            uint _303 = _300 ^ ((_300 << 15u) & 4022730752u);
            uint2 _311 = uint2(_277 + int2(1, 0));
            uint _313 = _311.y;
            uint _315 = _313 + (_313 << 10u);
            uint _317 = _315 ^ (_315 >> 6u);
            uint _319 = _317 + (_317 << 3u);
            uint _321 = _319 ^ (_319 >> 11u);
            uint _327 = (((_311.x * 1664525u) + (_321 + (_321 << 15u))) + 1013904223u) * 1664525u;
            uint _329 = _327 ^ (_327 >> 11u);
            uint _332 = _329 ^ ((_329 << 7u) & 2636928640u);
            uint _335 = _332 ^ ((_332 << 15u) & 4022730752u);
            uint2 _343 = uint2(_277 + int2(0, 1));
            uint _345 = _343.y;
            uint _347 = _345 + (_345 << 10u);
            uint _349 = _347 ^ (_347 >> 6u);
            uint _351 = _349 + (_349 << 3u);
            uint _353 = _351 ^ (_351 >> 11u);
            uint _359 = (((_343.x * 1664525u) + (_353 + (_353 << 15u))) + 1013904223u) * 1664525u;
            uint _361 = _359 ^ (_359 >> 11u);
            uint _364 = _361 ^ ((_361 << 7u) & 2636928640u);
            uint _367 = _364 ^ ((_364 << 15u) & 4022730752u);
            uint2 _375 = uint2(_277 + int2(1, 1));
            uint _377 = _375.y;
            uint _379 = _377 + (_377 << 10u);
            uint _381 = _379 ^ (_379 >> 6u);
            uint _383 = _381 + (_381 << 3u);
            uint _385 = _383 ^ (_383 >> 11u);
            uint _391 = (((_375.x * 1664525u) + (_385 + (_385 << 15u))) + 1013904223u) * 1664525u;
            uint _393 = _391 ^ (_391 >> 11u);
            uint _396 = _393 ^ ((_393 << 7u) & 2636928640u);
            uint _399 = _396 ^ ((_396 << 15u) & 4022730752u);
            float2 _409 = (_278 * _278) * (3.0f.xx - (_278 * 2.0f));
            float _410 = _409.x;
            float _411 = 1.0f - _410;
            float _413 = _409.y;
            _425 = ((((1.0f - _413) * (((asfloat(((_303 ^ (_303 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _411) + ((asfloat(((_335 ^ (_335 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _410))) + (((asfloat(((_367 ^ (_367 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _411) * _413)) + (((asfloat(((_399 ^ (_399 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _410) * _413)) > 0.5f;
        }
        else
        {
            _425 = false;
        }
        if (_425)
        {
            float3 _428 = _182.xxy;
            float _435 = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_182.xy * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
            _428.y = _435;
            float3 _444;
            if (frame_constants_glade_theme_id == 4u)
            {
                float3 _443 = _428;
                _443.y = _435 - 0.5f;
                _444 = _443;
            }
            else
            {
                _444 = _428;
            }
            float _458;
            do
            {
                float _448 = abs(_263.y - 0.4000000059604644775390625f);
                if (_448 > 0.300000011920928955078125f)
                {
                    _458 = 0.0f;
                    break;
                }
                float _452 = _448 * 3.3333332538604736328125f;
                _458 = 1.0f - ((_452 * _452) * (3.0f - (_448 * 6.666666507720947265625f)));
                break;
            } while(false);
            uint _459 = gl_GlobalInvocationID.x + gl_GlobalInvocationID.y;
            uint _461 = _459 + (_459 << 10u);
            uint _463 = _461 ^ (_461 >> 6u);
            uint _465 = _463 + (_463 << 3u);
            uint _467 = _465 ^ (_465 >> 11u);
            float _474 = lerp(0.800000011920928955078125f, 1.2000000476837158203125f, asfloat(((_467 + (_467 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
            float3 _475 = _474.xxx;
            float3 _478 = _475 * lerp(0.20000000298023223876953125f, 1.0f, sqrt(_458));
            float _480 = _478.y * _474;
            float3 _485 = float3(min(_478.x, 0.75f), _480, min(_478.z, 0.75f));
            _485.y = min(_480, 1.2000000476837158203125f);
            float3 _488 = _485 * 0.75f;
            float _502;
            do
            {
                float _492 = abs(_264 - 0.4000000059604644775390625f);
                if (_492 > 0.300000011920928955078125f)
                {
                    _502 = 0.0f;
                    break;
                }
                float _496 = _492 * 3.3333332538604736328125f;
                _502 = 1.0f - ((_496 * _496) * (3.0f - (_492 * 6.666666507720947265625f)));
                break;
            } while(false);
            if ((float3(min((_475 * lerp(0.20000000298023223876953125f, 1.0f, sqrt(_502))).x, 0.75f), _147, _147) * 0.75f).x < 0.300000011920928955078125f)
            {
                break;
            }
            float _530 = 3.1400001049041748046875f * _179;
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
            float4x4 _564 = mul(mul(float4x4(float4(1.0f, 0.0f, 0.0f, _444.x), float4(0.0f, 1.0f, 0.0f, _444.y), float4(0.0f, 0.0f, 1.0f, _444.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(float4x4(float4(1.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 1.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4x4(float4(_488.x, 0.0f, 0.0f, 0.0f), float4(0.0f, _488.y, 0.0f, 0.0f), float4(0.0f, 0.0f, _488.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), transpose(float4x4(float4((_536 * _535) + _533, _540 - _542, _545 + _546, 0.0f), float4(_540 + _542, (_549 * _539) + _533, _552 - _553, 0.0f), float4(_545 - _546, _552 + _553, (_544 * _541) + _533, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
            uint _578;
            __alias_draw_commands.InterlockedAdd(204, 1u, _578);
            uint _580 = uint(int(_578));
            float _582[12] = { _564[0].x, _564[1].x, _564[2].x, _564[0].y, _564[1].y, _564[2].y, _564[0].z, _564[1].z, _564[2].z, _564[0].w, _564[1].w, _564[2].w };
            Affine3Packed _583 = { _582 };
            BushXform _584 = { _583 };
            [unroll]
            for (int _2ident = 0; _2ident < 12; _2ident++)
            {
                __alias_bush_buffer.Store<float>(_2ident * 4 + _580 * 48 + 0, _584.transform.data[_2ident]);
            }
            uint _588 = ((gl_GlobalInvocationID.x * 2u) + (gl_GlobalInvocationID.y / 5u)) + 123u;
            uint _590 = _588 + (_588 << 10u);
            uint _592 = _590 ^ (_590 >> 6u);
            uint _594 = _592 + (_592 << 3u);
            uint _596 = _594 ^ (_594 >> 11u);
            if ((asfloat(((_596 + (_596 << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.699999988079071044921875f)
            {
                uint _607;
                __alias_draw_commands.InterlockedAdd(224, 1u, _607);
                uint _609 = uint(int(_607));
                FlowerXform _611 = { _583, (-1.0f).xxx, 2u };
                [unroll]
                for (int _3ident = 0; _3ident < 12; _3ident++)
                {
                    __alias_bush_flowers.Store<float>(_3ident * 4 + _609 * 64 + 0, _611.transform.data[_3ident]);
                }
                __alias_bush_flowers.Store<float3>(_609 * 64 + 48, _611.color);
                __alias_bush_flowers.Store<uint>(_609 * 64 + 60, _611.flower_type);
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
