#define TGV_0 1
#line 1 "shaders/water/spawn_lilypads.cs.hlsl"


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
#line 6 "shaders/water/spawn_lilypads.cs.hlsl"

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
#line 7 "shaders/water/spawn_lilypads.cs.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 8 "shaders/water/spawn_lilypads.cs.hlsl"

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
#line 9 "shaders/water/spawn_lilypads.cs.hlsl"

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
#line 10 "shaders/water/spawn_lilypads.cs.hlsl"

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
#line 11 "shaders/water/spawn_lilypads.cs.hlsl"

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
#line 12 "shaders/water/spawn_lilypads.cs.hlsl"

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
#line 13 "shaders/water/spawn_lilypads.cs.hlsl"

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
#line 14 "shaders/water/spawn_lilypads.cs.hlsl"

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
#line 15 "shaders/water/spawn_lilypads.cs.hlsl"

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
#line 16 "shaders/water/spawn_lilypads.cs.hlsl"


struct LilypadXform
{
    Affine3Packed xform;
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

float band_falloff(float d, float inv_half_width)
{
    float t = d * inv_half_width;
    return 1.0f - (t * t) * (3.0f - d * (2.0f * inv_half_width));
}

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
    uint2 water_mask_dims;
    uint2 path_mask_dims;
    uint2 garden_mask_dims;
    uint2 animation_dims;
    int2 grid_dims;
};

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);
Texture2D<float4> water_mask : register(t15, space0);
Texture2D<float4> path_mask : register(t16, space0);
Texture2D<float4> garden_mask : register(t17, space0);
RWTexture2D<half> animation : register(u18, space0);
RWStructuredBuffer<LilypadXform> __alias_lilypad_buffer : register(u19, space0);
RWStructuredBuffer<LilypadXform> __alias_lilypad_flower_buffer : register(u20, space0);



RWStructuredBuffer<uint> _rda_trigger[] : register(u100, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 id : SV_DispatchThreadID)
{
    if (id.x >= animation_dims.x || id.y >= animation_dims.y)
        return;

    float rx = rand01(hash(id.x));
    float ry = rand01(hash(id.y));
    float2 world_pos = ((float2(id.xy) + 0.5f) / float2(grid_dims) - 0.5f) * 130.0f
        + (lerp(-0.25f, 0.25f, (rx + ry) * 0.5f) * 2.0f).xx;

    float2 uv = world_pos * 0.0076923076994717121124267578125f + 0.5f;
    bool in_bounds = uv.x >= 0.0f && uv.x <= 1.0f && uv.y >= 0.0f && uv.y <= 1.0f;
    float water = in_bounds ? water_mask.SampleLevel(sampler_default, uv, 0.0f).y : 0.0f;

    float garden = smoothstep(0.4f, 0.5f, smoothstep(0.0f, 0.5f, garden_mask.SampleLevel(sampler_default, uv, 0.0f).y));
    animation[id.xy] = lerp(animation[id.xy].x, float(garden > 0.0f), 1.5f * delta_time_seconds);

    bool theme2 = glade_theme_id == 2u;
    float water_band;
    if (theme2)
    {
        water_band = band_falloff(abs((water + perlin(world_pos * 1.83f) * 0.3f) - 0.662f), 2.5445291996002197265625f);
    }
    else
    {
        water_band = band_falloff(abs(water - 0.45f), 3.3333332538604736328125f);
    }

    float path = path_mask.SampleLevel(sampler_llb, uv, 0.0f).x;
    float path_band = band_falloff(abs(path - 0.2f), 5.0f);

    float pad_score = water_band + path_band * smoothstep(0.6f, 0.8f, water);
    float bloom_score = garden * smoothstep(0.4f, 0.5f, water);

    if (pad_score <= 0.0f)
        return;

    bool perlin_gate = theme2
        ? perlin(world_pos) > 0.5f
        : (bloom_score > 0.0f && perlin(world_pos * 2.0f) > 0.5f);

    if (!perlin_gate)
        return;

    float3 pos = float3(world_pos.x, 0.0f, world_pos.y);
    if (glade_theme_id == 4u)
        pos.y = -0.1f;

    float rand = rand01(hash(id.x + id.y));
    float3 scale = (lerp(0.8f, theme2 ? 1.3f : 2.0f, rand)
                     * lerp(0.5f, 1.0f, clamp(pad_score + bloom_score, 0.0f, 1.0f))
                     * lerp(1.0f, 0.75f, bloom_score)).xxx;

    float2 dir2 = float2(rand01(hash(id.x ^ 234u)) * 2.0f - 1.0f, rand01(hash(id.x ^ 8634u)) * 2.0f - 1.0f);
    float3 dir = normalize(float3(dir2.x, 0.0f, dir2.y));
    float3 up = dir.y < 0.0f ? float3(0.0f, -1.0f, 0.0f) : float3(0.0f, 1.0f, 0.0f);
    float4x4 basis = transpose(float4x4(
        float4(dir, 0.0f),
        float4(up, 0.0f),
        float4(cross(dir, up), 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f)));

    float4x4 m = mul(mul(
        affine_translation(pos),
        mul(basis, float4x4(
            float4(scale.x, 0.0f, 0.0f, 0.0f),
            float4(0.0f, scale.y, 0.0f, 0.0f),
            float4(0.0f, 0.0f, scale.z, 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f)))),
        rotate_axis(normalize(float3(0.0f, 1.0f, 0.0f)), 3.14f * rand));

    uint base;
    InterlockedAdd(__alias_draw_commands[uint(draw_indirect_buffer_offset)].instanceCount, 1u, base);
    __alias_lilypad_buffer[base] = LilypadXform(pack_affine(m));

    if (bloom_score > 0.7f && rand01(hash(id.x + id.y + 345u)) > 0.2f)
    {
        float grow = clamp(1.0f - smoothstep(rand01(hash((10u * id.x) + 45u + id.y * 10u)) * 0.2f - 0.2f, 1.0f, animation[id.xy].x), 0.0f, 1.0f);
        float pop = (grow * grow * grow) - (grow * sin(grow * 3.14f) * 1.5f);
        float3 flower_scale = scale * (bloom_score * 0.8f) * (1.0f - pop);
        if (theme2)
            flower_scale *= 2.0f;

        float jx = rand01(hash((id.x + id.y * 2u) + 345u));
        float jy = rand01(hash(id.x * 10u + id.y * 345u));
        float3 fpos = float3(
            pos.x + lerp(-0.3f, 0.3f, jx),
            (pos.y - pop * 0.3f) + jy * 0.1f,
            pos.z + lerp(-0.3f, 0.3f, rand01(hash((id.x + id.y * 3u) - 345u))));

        float4x4 fm = mul(
            affine_translation(fpos),
            mul(basis, float4x4(
                float4(flower_scale.x, 0.0f, 0.0f, 0.0f),
                float4(0.0f, flower_scale.y, 0.0f, 0.0f),
                float4(0.0f, 0.0f, flower_scale.z, 0.0f),
                float4(0.0f, 0.0f, 0.0f, 1.0f))));

        uint fbase;
        InterlockedAdd(__alias_draw_commands[15u].instanceCount, 1u, fbase);
        __alias_lilypad_flower_buffer[fbase] = LilypadXform(pack_affine(fm));
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

struct LilypadXform
{
    Affine3Packed xform;
};

static uint _137;

RWByteAddressBuffer __alias_draw_commands : register(u13, space0);
cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset : packoffset(c0);
    uint2 water_mask_dims : packoffset(c0.y);
    uint2 path_mask_dims : packoffset(c1);
    uint2 garden_mask_dims : packoffset(c1.z);
    uint2 animation_dims : packoffset(c2);
    int2 grid_dims : packoffset(c2.z);
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

RWByteAddressBuffer __alias_lilypad_buffer : register(u19, space0);
RWByteAddressBuffer __alias_lilypad_flower_buffer : register(u20, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> water_mask : register(t15, space0);
Texture2D<float4> path_mask : register(t16, space0);
Texture2D<float4> garden_mask : register(t17, space0);
RWTexture2D<float> animation : register(u18, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    do
    {
        if (any(bool2(gl_GlobalInvocationID.xy.x >= animation_dims.x, gl_GlobalInvocationID.xy.y >= animation_dims.y)))
        {
            break;
        }
        float2 _209;
        uint _172 = gl_GlobalInvocationID.x + (gl_GlobalInvocationID.x << 10u);
        uint _174 = _172 ^ (_172 >> 6u);
        uint _176 = _174 + (_174 << 3u);
        uint _178 = _176 ^ (_176 >> 11u);
        uint _187 = gl_GlobalInvocationID.y + (gl_GlobalInvocationID.y << 10u);
        uint _189 = _187 ^ (_187 >> 6u);
        uint _191 = _189 + (_189 << 3u);
        uint _193 = _191 ^ (_191 >> 11u);
        float2 _205 = ((((float2(gl_GlobalInvocationID.xy) + 0.5f.xx) / float2(grid_dims)) - 0.5f.xx) * 130.0f.xx) + (lerp(-0.25f, 0.25f, ((asfloat(((_178 + (_178 << 15u)) & 8388607u) | 1065353216u) - 1.0f) + (asfloat(((_193 + (_193 << 15u)) & 8388607u) | 1065353216u) - 1.0f)) * 0.5f) * 2.0f).xx;
        float _237;
        do
        {
            _209 = (_205 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
            bool _216;
            if (!(_209.x < 0.0f))
            {
                _216 = _209.x > 1.0f;
            }
            else
            {
                _216 = true;
            }
            bool _222;
            if (!_216)
            {
                _222 = _209.y < 0.0f;
            }
            else
            {
                _222 = true;
            }
            bool _228;
            if (!_222)
            {
                _228 = _209.y > 1.0f;
            }
            else
            {
                _228 = true;
            }
            if (_228)
            {
                _237 = 0.0f;
                break;
            }
            else
            {
                _237 = water_mask.SampleLevel(sampler_default, _209, 0.0f).y;
                break;
            }
            break; 
        } while(false);
        float _249 = smoothstep(0.4000000059604644775390625f, 0.5f, smoothstep(0.0f, 0.5f, garden_mask.SampleLevel(sampler_default, _209, 0.0f).y));
        animation[gl_GlobalInvocationID.xy] = lerp(animation[gl_GlobalInvocationID.xy].xxxx.x, ((_249 > 0.0f) ? 1.0f : 0.0f), 1.5f * frame_constants_delta_time_seconds);
        bool _262 = frame_constants_glade_theme_id == 2u;
        float _445;
        if (_262)
        {
            float2 _280 = _205 * 1.83000004291534423828125f;
            int2 _282 = int2(floor(_280));
            float2 _283 = frac(_280);
            uint2 _284 = uint2(_282);
            uint _286 = _284.y;
            uint _288 = _286 + (_286 << 10u);
            uint _290 = _288 ^ (_288 >> 6u);
            uint _292 = _290 + (_290 << 3u);
            uint _294 = _292 ^ (_292 >> 11u);
            uint _300 = (((_284.x * 1664525u) + (_294 + (_294 << 15u))) + 1013904223u) * 1664525u;
            uint _302 = _300 ^ (_300 >> 11u);
            uint _305 = _302 ^ ((_302 << 7u) & 2636928640u);
            uint _308 = _305 ^ ((_305 << 15u) & 4022730752u);
            uint2 _316 = uint2(_282 + int2(1, 0));
            uint _318 = _316.y;
            uint _320 = _318 + (_318 << 10u);
            uint _322 = _320 ^ (_320 >> 6u);
            uint _324 = _322 + (_322 << 3u);
            uint _326 = _324 ^ (_324 >> 11u);
            uint _332 = (((_316.x * 1664525u) + (_326 + (_326 << 15u))) + 1013904223u) * 1664525u;
            uint _334 = _332 ^ (_332 >> 11u);
            uint _337 = _334 ^ ((_334 << 7u) & 2636928640u);
            uint _340 = _337 ^ ((_337 << 15u) & 4022730752u);
            uint2 _348 = uint2(_282 + int2(0, 1));
            uint _350 = _348.y;
            uint _352 = _350 + (_350 << 10u);
            uint _354 = _352 ^ (_352 >> 6u);
            uint _356 = _354 + (_354 << 3u);
            uint _358 = _356 ^ (_356 >> 11u);
            uint _364 = (((_348.x * 1664525u) + (_358 + (_358 << 15u))) + 1013904223u) * 1664525u;
            uint _366 = _364 ^ (_364 >> 11u);
            uint _369 = _366 ^ ((_366 << 7u) & 2636928640u);
            uint _372 = _369 ^ ((_369 << 15u) & 4022730752u);
            uint2 _380 = uint2(_282 + int2(1, 1));
            uint _382 = _380.y;
            uint _384 = _382 + (_382 << 10u);
            uint _386 = _384 ^ (_384 >> 6u);
            uint _388 = _386 + (_386 << 3u);
            uint _390 = _388 ^ (_388 >> 11u);
            uint _396 = (((_380.x * 1664525u) + (_390 + (_390 << 15u))) + 1013904223u) * 1664525u;
            uint _398 = _396 ^ (_396 >> 11u);
            uint _401 = _398 ^ ((_398 << 7u) & 2636928640u);
            uint _404 = _401 ^ ((_401 << 15u) & 4022730752u);
            float2 _414 = (_283 * _283) * (3.0f.xx - (_283 * 2.0f));
            float _415 = _414.x;
            float _416 = 1.0f - _415;
            float _418 = _414.y;
            float _444;
            do
            {
                float _434 = abs((_237 + (((((1.0f - _418) * (((asfloat(((_308 ^ (_308 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _416) + ((asfloat(((_340 ^ (_340 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _415))) + (((asfloat(((_372 ^ (_372 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _416) * _418)) + (((asfloat(((_404 ^ (_404 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _415) * _418)) * 0.300000011920928955078125f)) - 0.662000000476837158203125f);
                if (_434 > 0.39300000667572021484375f)
                {
                    _444 = 0.0f;
                    break;
                }
                float _438 = _434 * 2.5445291996002197265625f;
                _444 = 1.0f - ((_438 * _438) * (3.0f - (_434 * 5.089058399200439453125f)));
                break;
            } while(false);
            _445 = _444;
        }
        else
        {
            float _279;
            do
            {
                float _269 = abs(_237 - 0.449999988079071044921875f);
                if (_269 > 0.300000011920928955078125f)
                {
                    _279 = 0.0f;
                    break;
                }
                float _273 = _269 * 3.3333332538604736328125f;
                _279 = 1.0f - ((_273 * _273) * (3.0f - (_269 * 6.666666507720947265625f)));
                break;
            } while(false);
            _445 = _279;
        }
        uint _446 = _262 ? 1013904223u : _137;
        uint _447 = _262 ? 1664525u : _137;
        bool _448 = _262 ? true : false;
        float _462;
        do
        {
            float _452 = abs(path_mask.SampleLevel(sampler_llb, _209, 0.0f).x - 0.20000000298023223876953125f);
            if (_452 > 0.20000000298023223876953125f)
            {
                _462 = 0.0f;
                break;
            }
            float _456 = _452 * 5.0f;
            _462 = 1.0f - ((_456 * _456) * (3.0f - (_452 * 10.0f)));
            break;
        } while(false);
        float _465 = _445 + (_462 * smoothstep(0.60000002384185791015625f, 0.800000011920928955078125f, _237));
        float _467 = _249 * smoothstep(0.4000000059604644775390625f, 0.5f, _237);
        uint _632;
        uint _633;
        bool _634;
        bool _635;
        if (_465 > 0.0f)
        {
            int2 _472 = int2(floor(_205));
            float2 _473 = frac(_205);
            uint2 _474 = uint2(_472);
            uint _476 = _474.y;
            uint _478 = _476 + (_476 << 10u);
            uint _480 = _478 ^ (_478 >> 6u);
            uint _482 = _480 + (_480 << 3u);
            uint _484 = _482 ^ (_482 >> 11u);
            bool _487 = _448 ? _448 : true;
            uint _488 = _448 ? _447 : 1664525u;
            uint _489 = _448 ? _446 : 1013904223u;
            uint _493 = (((_474.x * _488) + (_484 + (_484 << 15u))) + _489) * _488;
            uint _495 = _493 ^ (_493 >> 11u);
            uint _498 = _495 ^ ((_495 << 7u) & 2636928640u);
            uint _501 = _498 ^ ((_498 << 15u) & 4022730752u);
            uint2 _509 = uint2(_472 + int2(1, 0));
            uint _511 = _509.y;
            uint _513 = _511 + (_511 << 10u);
            uint _515 = _513 ^ (_513 >> 6u);
            uint _517 = _515 + (_515 << 3u);
            uint _519 = _517 ^ (_517 >> 11u);
            bool _522 = _487 ? _487 : true;
            uint _523 = _487 ? _488 : 1664525u;
            uint _524 = _487 ? _489 : 1013904223u;
            uint _528 = (((_509.x * _523) + (_519 + (_519 << 15u))) + _524) * _523;
            uint _530 = _528 ^ (_528 >> 11u);
            uint _533 = _530 ^ ((_530 << 7u) & 2636928640u);
            uint _536 = _533 ^ ((_533 << 15u) & 4022730752u);
            uint2 _544 = uint2(_472 + int2(0, 1));
            uint _546 = _544.y;
            uint _548 = _546 + (_546 << 10u);
            uint _550 = _548 ^ (_548 >> 6u);
            uint _552 = _550 + (_550 << 3u);
            uint _554 = _552 ^ (_552 >> 11u);
            bool _557 = _522 ? _522 : true;
            uint _558 = _522 ? _523 : 1664525u;
            uint _559 = _522 ? _524 : 1013904223u;
            uint _563 = (((_544.x * _558) + (_554 + (_554 << 15u))) + _559) * _558;
            uint _565 = _563 ^ (_563 >> 11u);
            uint _568 = _565 ^ ((_565 << 7u) & 2636928640u);
            uint _571 = _568 ^ ((_568 << 15u) & 4022730752u);
            uint2 _579 = uint2(_472 + int2(1, 1));
            uint _581 = _579.y;
            uint _583 = _581 + (_581 << 10u);
            uint _585 = _583 ^ (_583 >> 6u);
            uint _587 = _585 + (_585 << 3u);
            uint _589 = _587 ^ (_587 >> 11u);
            uint _593 = _557 ? _558 : 1664525u;
            uint _594 = _557 ? _559 : 1013904223u;
            uint _598 = (((_579.x * _593) + (_589 + (_589 << 15u))) + _594) * _593;
            uint _600 = _598 ^ (_598 >> 11u);
            uint _603 = _600 ^ ((_600 << 7u) & 2636928640u);
            uint _606 = _603 ^ ((_603 << 15u) & 4022730752u);
            float2 _616 = (_473 * _473) * (3.0f.xx - (_473 * 2.0f));
            float _617 = _616.x;
            float _618 = 1.0f - _617;
            float _620 = _616.y;
            _632 = _594;
            _633 = _593;
            _634 = _557 ? _557 : true;
            _635 = ((((1.0f - _620) * (((asfloat(((_501 ^ (_501 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _618) + ((asfloat(((_536 ^ (_536 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _617))) + (((asfloat(((_571 ^ (_571 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _618) * _620)) + (((asfloat(((_606 ^ (_606 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _617) * _620)) > 0.5f;
        }
        else
        {
            _632 = _446;
            _633 = _447;
            _634 = _448;
            _635 = false;
        }
        bool _804;
        if (!_635)
        {
            bool _803;
            if (_467 > 0.0f)
            {
                float2 _642 = _205 * 2.0f;
                int2 _644 = int2(floor(_642));
                float2 _645 = frac(_642);
                uint2 _646 = uint2(_644);
                uint _648 = _646.y;
                uint _650 = _648 + (_648 << 10u);
                uint _652 = _650 ^ (_650 >> 6u);
                uint _654 = _652 + (_652 << 3u);
                uint _656 = _654 ^ (_654 >> 11u);
                bool _659 = _634 ? _634 : true;
                uint _660 = _634 ? _633 : 1664525u;
                uint _661 = _634 ? _632 : 1013904223u;
                uint _665 = (((_646.x * _660) + (_656 + (_656 << 15u))) + _661) * _660;
                uint _667 = _665 ^ (_665 >> 11u);
                uint _670 = _667 ^ ((_667 << 7u) & 2636928640u);
                uint _673 = _670 ^ ((_670 << 15u) & 4022730752u);
                uint2 _681 = uint2(_644 + int2(1, 0));
                uint _683 = _681.y;
                uint _685 = _683 + (_683 << 10u);
                uint _687 = _685 ^ (_685 >> 6u);
                uint _689 = _687 + (_687 << 3u);
                uint _691 = _689 ^ (_689 >> 11u);
                bool _694 = _659 ? _659 : true;
                uint _695 = _659 ? _660 : 1664525u;
                uint _696 = _659 ? _661 : 1013904223u;
                uint _700 = (((_681.x * _695) + (_691 + (_691 << 15u))) + _696) * _695;
                uint _702 = _700 ^ (_700 >> 11u);
                uint _705 = _702 ^ ((_702 << 7u) & 2636928640u);
                uint _708 = _705 ^ ((_705 << 15u) & 4022730752u);
                uint2 _716 = uint2(_644 + int2(0, 1));
                uint _718 = _716.y;
                uint _720 = _718 + (_718 << 10u);
                uint _722 = _720 ^ (_720 >> 6u);
                uint _724 = _722 + (_722 << 3u);
                uint _726 = _724 ^ (_724 >> 11u);
                bool _729 = _694 ? _694 : true;
                uint _730 = _694 ? _695 : 1664525u;
                uint _731 = _694 ? _696 : 1013904223u;
                uint _735 = (((_716.x * _730) + (_726 + (_726 << 15u))) + _731) * _730;
                uint _737 = _735 ^ (_735 >> 11u);
                uint _740 = _737 ^ ((_737 << 7u) & 2636928640u);
                uint _743 = _740 ^ ((_740 << 15u) & 4022730752u);
                uint2 _751 = uint2(_644 + int2(1, 1));
                uint _753 = _751.y;
                uint _755 = _753 + (_753 << 10u);
                uint _757 = _755 ^ (_755 >> 6u);
                uint _759 = _757 + (_757 << 3u);
                uint _761 = _759 ^ (_759 >> 11u);
                uint _764 = _729 ? _730 : 1664525u;
                uint _769 = (((_751.x * _764) + (_761 + (_761 << 15u))) + (_729 ? _731 : 1013904223u)) * _764;
                uint _771 = _769 ^ (_769 >> 11u);
                uint _774 = _771 ^ ((_771 << 7u) & 2636928640u);
                uint _777 = _774 ^ ((_774 << 15u) & 4022730752u);
                float2 _787 = (_645 * _645) * (3.0f.xx - (_645 * 2.0f));
                float _788 = _787.x;
                float _789 = 1.0f - _788;
                float _791 = _787.y;
                _803 = ((((1.0f - _791) * (((asfloat(((_673 ^ (_673 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _789) + ((asfloat(((_708 ^ (_708 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _788))) + (((asfloat(((_743 ^ (_743 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _789) * _791)) + (((asfloat(((_777 ^ (_777 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _788) * _791)) > 0.5f;
            }
            else
            {
                _803 = false;
            }
            _804 = _803;
        }
        else
        {
            _804 = true;
        }
        if (_804)
        {
            int _811 = int((gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) + gl_GlobalInvocationID.x);
            float3 _813 = _205.xxy;
            _813.y = 0.0f;
            float3 _819;
            if (frame_constants_glade_theme_id == 4u)
            {
                float3 _818 = _813;
                _818.y = -0.100000001490116119384765625f;
                _819 = _818;
            }
            else
            {
                _819 = _813;
            }
            uint _820 = gl_GlobalInvocationID.x + gl_GlobalInvocationID.y;
            uint _822 = _820 + (_820 << 10u);
            uint _824 = _822 ^ (_822 >> 6u);
            uint _826 = _824 + (_824 << 3u);
            uint _828 = _826 ^ (_826 >> 11u);
            float _834 = asfloat(((_828 + (_828 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
            float3 _840 = (lerp(0.800000011920928955078125f, _262 ? 1.2999999523162841796875f : 2.0f, _834) * lerp(0.5f, 1.0f, clamp(_465 + _467, 0.0f, 1.0f))).xxx * lerp(1.0f, 0.75f, _467);
            uint _842 = uint(_811 ^ 234);
            uint _844 = _842 + (_842 << 10u);
            uint _846 = _844 ^ (_844 >> 6u);
            uint _848 = _846 + (_846 << 3u);
            uint _850 = _848 ^ (_848 >> 11u);
            uint _858 = uint(_811 ^ 8634);
            uint _860 = _858 + (_858 << 10u);
            uint _862 = _860 ^ (_860 >> 6u);
            uint _864 = _862 + (_862 << 3u);
            uint _866 = _864 ^ (_864 >> 11u);
            float3 _874 = normalize(float3(asfloat(((_850 + (_850 << 15u)) & 8388607u) | 1065353216u) - 1.0f, 0.0f, asfloat(((_866 + (_866 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
            float4x4 _899 = transpose(float4x4(float4(_874, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(cross(_874, float3(0.0f, 1.0f, 0.0f)), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
            float _902 = 3.1400001049041748046875f * _834;
            float3 _903 = normalize(float3(0.0f, 1.0f, 0.0f));
            float _904 = sin(_902);
            float _905 = cos(_902);
            float _906 = 1.0f - _905;
            float _907 = _903.x;
            float _908 = _906 * _907;
            float _911 = _903.y;
            float _912 = _908 * _911;
            float _913 = _903.z;
            float _914 = _913 * _904;
            float _916 = _906 * _913;
            float _917 = _916 * _907;
            float _918 = _911 * _904;
            float _921 = _906 * _911;
            float _924 = _921 * _913;
            float _925 = _907 * _904;
            float4x4 _936 = mul(mul(float4x4(float4(1.0f, 0.0f, 0.0f, _819.x), float4(0.0f, 1.0f, 0.0f, _819.y), float4(0.0f, 0.0f, 1.0f, _819.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(_899, float4x4(float4(_840.x, 0.0f, 0.0f, 0.0f), float4(0.0f, _840.y, 0.0f, 0.0f), float4(0.0f, 0.0f, _840.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), transpose(float4x4(float4((_908 * _907) + _905, _912 - _914, _917 + _918, 0.0f), float4(_912 + _914, (_921 * _911) + _905, _924 - _925, 0.0f), float4(_917 - _918, _924 + _925, (_916 * _913) + _905, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
            uint _953;
            __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, _953);
            uint _955 = uint(int(_953));
            float _957[12] = { _936[0].x, _936[1].x, _936[2].x, _936[0].y, _936[1].y, _936[2].y, _936[0].z, _936[1].z, _936[2].z, _936[0].w, _936[1].w, _936[2].w };
            Affine3Packed _958 = { _957 };
            LilypadXform _959 = { _958 };
            [unroll]
            for (int _4ident = 0; _4ident < 12; _4ident++)
            {
                __alias_lilypad_buffer.Store<float>(_4ident * 4 + _955 * 48 + 0, _959.xform.data[_4ident]);
            }
            uint _962 = _820 + 345u;
            uint _964 = _962 + (_962 << 10u);
            uint _966 = _964 ^ (_964 >> 6u);
            uint _968 = _966 + (_966 << 3u);
            uint _970 = _968 ^ (_968 >> 11u);
            if ((int(_467 > 0.699999988079071044921875f) & int((asfloat(((_970 + (_970 << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.20000000298023223876953125f)) != 0)
            {
                uint _984;
                __alias_draw_commands.InterlockedAdd(304, 1u, _984);
                uint _987 = (10u * _820) + 45u;
                uint _989 = _987 + (_987 << 10u);
                uint _991 = _989 ^ (_989 >> 6u);
                uint _993 = _991 + (_991 << 3u);
                uint _995 = _993 ^ (_993 >> 11u);
                float _1009 = clamp(1.0f - smoothstep(((asfloat(((_995 + (_995 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 0.20000000298023223876953125f) - 0.20000000298023223876953125f, 1.0f, animation[gl_GlobalInvocationID.xy].xxxx.x), 0.0f, 1.0f);
                float _1016 = ((_1009 * _1009) * _1009) - ((_1009 * sin(_1009 * 3.1400001049041748046875f)) * 1.5f);
                float3 _1020 = (_840 * (_467 * 0.800000011920928955078125f)) * (1.0f - _1016);
                float3 _1024;
                if (_262)
                {
                    _1024 = _1020 * 2.0f;
                }
                else
                {
                    _1024 = _1020;
                }
                uint _1029 = (gl_GlobalInvocationID.x + (gl_GlobalInvocationID.y * 2u)) + 345u;
                uint _1031 = _1029 + (_1029 << 10u);
                uint _1033 = _1031 ^ (_1031 >> 6u);
                uint _1035 = _1033 + (_1033 << 3u);
                uint _1037 = _1035 ^ (_1035 >> 11u);
                uint _1048 = (gl_GlobalInvocationID.x + (gl_GlobalInvocationID.y * 3u)) - 345u;
                uint _1050 = _1048 + (_1048 << 10u);
                uint _1052 = _1050 ^ (_1050 >> 6u);
                uint _1054 = _1052 + (_1052 << 3u);
                uint _1056 = _1054 ^ (_1054 >> 11u);
                uint _1067 = (gl_GlobalInvocationID.x * 10u) + (gl_GlobalInvocationID.y * 345u);
                uint _1069 = _1067 + (_1067 << 10u);
                uint _1071 = _1069 ^ (_1069 >> 6u);
                uint _1073 = _1071 + (_1071 << 3u);
                uint _1075 = _1073 ^ (_1073 >> 11u);
                float4x4 _1096 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _819.x + lerp(-0.300000011920928955078125f, 0.300000011920928955078125f, asfloat(((_1037 + (_1037 << 15u)) & 8388607u) | 1065353216u) - 1.0f)), float4(0.0f, 1.0f, 0.0f, (_819.y - (_1016 * 0.300000011920928955078125f)) + ((asfloat(((_1075 + (_1075 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 0.100000001490116119384765625f)), float4(0.0f, 0.0f, 1.0f, _819.z + lerp(-0.300000011920928955078125f, 0.300000011920928955078125f, asfloat(((_1056 + (_1056 << 15u)) & 8388607u) | 1065353216u) - 1.0f)), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(_899, float4x4(float4(_1024.x, 0.0f, 0.0f, 0.0f), float4(0.0f, _1024.y, 0.0f, 0.0f), float4(0.0f, 0.0f, _1024.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                uint _1109 = uint(int(_984));
                float _1111[12] = { _1096[0].x, _1096[1].x, _1096[2].x, _1096[0].y, _1096[1].y, _1096[2].y, _1096[0].z, _1096[1].z, _1096[2].z, _1096[0].w, _1096[1].w, _1096[2].w };
                Affine3Packed _1112 = { _1111 };
                LilypadXform _1113 = { _1112 };
                [unroll]
                for (int _5ident = 0; _5ident < 12; _5ident++)
                {
                    __alias_lilypad_flower_buffer.Store<float>(_5ident * 4 + _1109 * 48 + 0, _1113.xform.data[_5ident]);
                }
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
