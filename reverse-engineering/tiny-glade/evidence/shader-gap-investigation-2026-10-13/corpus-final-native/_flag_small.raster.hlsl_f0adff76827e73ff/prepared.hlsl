#define SHADER_VARIANT_DEPTH_ONLY 1
#define TGV_f0adff76827e73ff 1
#line 1 "shaders/flag_small.raster.hlsl"


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
#line 12 "shaders/flag_small.raster.hlsl"

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
#line 13 "shaders/flag_small.raster.hlsl"

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
#line 14 "shaders/flag_small.raster.hlsl"

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
#line 15 "shaders/flag_small.raster.hlsl"

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
#line 16 "shaders/flag_small.raster.hlsl"

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
#line 17 "shaders/flag_small.raster.hlsl"









Texture2D<float4> border_mask : register(t14, space0);
Texture2D<float4> flag_palettes : register(t15, space0);
Texture2DArray<float4> flag_patterns : register(t16, space0);

float4x4 model_to_world;
int color_id;
int seed;
uint2 border_mask_dims;
uint2 flag_palettes_dims;
int bool_render_shadows;



float flag_noise(float2 p)
{
    int2 cell = int2(floor(p));
    float2 f = frac(p);
    float2 u = f * f * (3.0f - 2.0f * f);
    float n00 = incb_perlin_hash01(hash(uint2(cell + int2(0, 0))));
    float n10 = incb_perlin_hash01(hash(uint2(cell + int2(1, 0))));
    float n01 = incb_perlin_hash01(hash(uint2(cell + int2(0, 1))));
    float n11 = incb_perlin_hash01(hash(uint2(cell + int2(1, 1))));
    return ((1.0f - u.y) * ((n00 * (1.0f - u.x)) + (n10 * u.x)))
        + ((n01 * (1.0f - u.x)) * u.y)
        + ((n11 * u.x) * u.y);
}

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 Vertex_UV: Vertex_UV;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    float4 origin = mul(model_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float t_sec = float(game_time_sec);
    float t = t_sec + game_time_frac;

    
    float gust = flag_noise((origin.xz * 0.1f) + (float2(t, 0.0f) * 1.2f));
    float r0 = rand01(hash(uint(seed)));
    float sway = lerp(0.3f, 1.0f, (sin((gust * PI) - HALF_PI + 0.3f) * 0.7f * lerp(0.7f, 1.0f, r0)) * 0.3f);

    
    float flutter = flag_noise(float2(t + origin.x, origin.y));

    float vertex_x = vin.Vertex_Position.x
        + (sin((fmod(t_sec * 1.8f, TWO_PI) + (game_time_frac * 1.8f)) + (vin.Vertex_Position.y * 1.5f) + (r0 * 0.5f)
            + (flutter * smoothstep(0.0f, -1.0f, vin.Vertex_Position.y))) * sway) * 0.3f;

    float4 wp = mul(model_to_world, float4(vertex_x, vin.Vertex_Position.yz, 1.0f));
    o.position = mul(view_constants[current_view_index].world_to_sample, wp);
    o.C0 = vin.Vertex_Color;
    o.C1 = mul(model_to_world, float4(vin.Vertex_Normal, 0.0f)).xyz;
    o.C2 = wp.xyz;
    o.C3 = (((vin.Vertex_UV * 2.0f) * float2(1.0f, 1.5f)) - 0.5f) * 0.8f + 0.625f;
    o.C4 = vin.Vertex_UV;
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;

    float layer = float(int(round(lerp(9.0f, 11.0f, rand01(hash(uint(seed ^ 234234)))))));
    float4 pattern = flag_patterns.SampleBias(sampler_llr, float3(pin.C3.x, 1.0f - pin.C3.y, layer), global_mip_bias);

    uint cid = uint(color_id);
    float3 albedo;
    if (border_mask.SampleBias(sampler_llc, pin.C4, global_mip_bias).x > 0.89999997615814208984375f)
    {
        albedo = flag_palettes.Load(int3(uint2(1u, cid), 0u)).xyz;
    }
    else
    {
        albedo = lerp(flag_palettes.Load(int3(uint2(0u, cid), 0u)).xyz,
                      flag_palettes.Load(int3(uint2(3u, cid), 0u)).xyz, (pattern.x * 0.5f).xxx);
    }

    float3 nrm;
    float4 origin = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    if (dot(normalize((origin.xyz / origin.w) - pin.C2), pin.C1) <= 0.0f)
    {
        nrm = -pin.C1;
    }
    else
    {
        nrm = pin.C1;
    }

    float3 packed_normal = (normalize(nrm) * 0.5f) + 0.5f;
    float3 albedo_sqrt = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = (uint(clamp(albedo_sqrt.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(albedo_sqrt.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(albedo_sqrt.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u));
    uint normal_word = (uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)) + 4278190080u;
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = albedo_word | 117440512u;
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
    int color_id : packoffset(c5);
    int seed : packoffset(c5.y);
    uint2 border_mask_dims : packoffset(c5.z);
    uint2 flag_palettes_dims : packoffset(c6);
    int bool_render_shadows : packoffset(c6.z);
};


static float4 gl_Position;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static float2 out_var_C4;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float4 _123 = mul(model_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float _132 = float(frame_constants_game_time_sec);
    float _133 = _132 + frame_constants_game_time_frac;
    float2 _136 = (_123.xz * 0.100000001490116119384765625f) + (float2(_133, 0.0f) * 1.2000000476837158203125f);
    int2 _138 = int2(floor(_136));
    float2 _139 = frac(_136);
    uint2 _140 = uint2(_138);
    uint _142 = _140.y;
    uint _144 = _142 + (_142 << 10u);
    uint _146 = _144 ^ (_144 >> 6u);
    uint _148 = _146 + (_146 << 3u);
    uint _150 = _148 ^ (_148 >> 11u);
    uint _156 = (((_140.x * 1664525u) + (_150 + (_150 << 15u))) + 1013904223u) * 1664525u;
    uint _158 = _156 ^ (_156 >> 11u);
    uint _161 = _158 ^ ((_158 << 7u) & 2636928640u);
    uint _164 = _161 ^ ((_161 << 15u) & 4022730752u);
    uint2 _172 = uint2(_138 + int2(1, 0));
    uint _174 = _172.y;
    uint _176 = _174 + (_174 << 10u);
    uint _178 = _176 ^ (_176 >> 6u);
    uint _180 = _178 + (_178 << 3u);
    uint _182 = _180 ^ (_180 >> 11u);
    uint _188 = (((_172.x * 1664525u) + (_182 + (_182 << 15u))) + 1013904223u) * 1664525u;
    uint _190 = _188 ^ (_188 >> 11u);
    uint _193 = _190 ^ ((_190 << 7u) & 2636928640u);
    uint _196 = _193 ^ ((_193 << 15u) & 4022730752u);
    uint2 _204 = uint2(_138 + int2(0, 1));
    uint _206 = _204.y;
    uint _208 = _206 + (_206 << 10u);
    uint _210 = _208 ^ (_208 >> 6u);
    uint _212 = _210 + (_210 << 3u);
    uint _214 = _212 ^ (_212 >> 11u);
    uint _220 = (((_204.x * 1664525u) + (_214 + (_214 << 15u))) + 1013904223u) * 1664525u;
    uint _222 = _220 ^ (_220 >> 11u);
    uint _225 = _222 ^ ((_222 << 7u) & 2636928640u);
    uint _228 = _225 ^ ((_225 << 15u) & 4022730752u);
    uint2 _236 = uint2(_138 + int2(1, 1));
    uint _238 = _236.y;
    uint _240 = _238 + (_238 << 10u);
    uint _242 = _240 ^ (_240 >> 6u);
    uint _244 = _242 + (_242 << 3u);
    uint _246 = _244 ^ (_244 >> 11u);
    uint _252 = (((_236.x * 1664525u) + (_246 + (_246 << 15u))) + 1013904223u) * 1664525u;
    uint _254 = _252 ^ (_252 >> 11u);
    uint _257 = _254 ^ ((_254 << 7u) & 2636928640u);
    uint _260 = _257 ^ ((_257 << 15u) & 4022730752u);
    float2 _270 = (_139 * _139) * (3.0f.xx - (_139 * 2.0f));
    float _271 = _270.x;
    float _272 = 1.0f - _271;
    float _274 = _270.y;
    uint _290 = uint(seed);
    uint _292 = _290 + (_290 << 10u);
    uint _294 = _292 ^ (_292 >> 6u);
    uint _296 = _294 + (_294 << 3u);
    uint _298 = _296 ^ (_296 >> 11u);
    float _304 = asfloat(((_298 + (_298 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float2 _320 = float2(_133 + _123.x, _123.y);
    int2 _322 = int2(floor(_320));
    float2 _323 = frac(_320);
    uint2 _324 = uint2(_322);
    uint _326 = _324.y;
    uint _328 = _326 + (_326 << 10u);
    uint _330 = _328 ^ (_328 >> 6u);
    uint _332 = _330 + (_330 << 3u);
    uint _334 = _332 ^ (_332 >> 11u);
    uint _340 = (((_324.x * 1664525u) + (_334 + (_334 << 15u))) + 1013904223u) * 1664525u;
    uint _342 = _340 ^ (_340 >> 11u);
    uint _345 = _342 ^ ((_342 << 7u) & 2636928640u);
    uint _348 = _345 ^ ((_345 << 15u) & 4022730752u);
    uint2 _356 = uint2(_322 + int2(1, 0));
    uint _358 = _356.y;
    uint _360 = _358 + (_358 << 10u);
    uint _362 = _360 ^ (_360 >> 6u);
    uint _364 = _362 + (_362 << 3u);
    uint _366 = _364 ^ (_364 >> 11u);
    uint _372 = (((_356.x * 1664525u) + (_366 + (_366 << 15u))) + 1013904223u) * 1664525u;
    uint _374 = _372 ^ (_372 >> 11u);
    uint _377 = _374 ^ ((_374 << 7u) & 2636928640u);
    uint _380 = _377 ^ ((_377 << 15u) & 4022730752u);
    uint2 _388 = uint2(_322 + int2(0, 1));
    uint _390 = _388.y;
    uint _392 = _390 + (_390 << 10u);
    uint _394 = _392 ^ (_392 >> 6u);
    uint _396 = _394 + (_394 << 3u);
    uint _398 = _396 ^ (_396 >> 11u);
    uint _404 = (((_388.x * 1664525u) + (_398 + (_398 << 15u))) + 1013904223u) * 1664525u;
    uint _406 = _404 ^ (_404 >> 11u);
    uint _409 = _406 ^ ((_406 << 7u) & 2636928640u);
    uint _412 = _409 ^ ((_409 << 15u) & 4022730752u);
    uint2 _420 = uint2(_322 + int2(1, 1));
    uint _422 = _420.y;
    uint _424 = _422 + (_422 << 10u);
    uint _426 = _424 ^ (_424 >> 6u);
    uint _428 = _426 + (_426 << 3u);
    uint _430 = _428 ^ (_428 >> 11u);
    uint _436 = (((_420.x * 1664525u) + (_430 + (_430 << 15u))) + 1013904223u) * 1664525u;
    uint _438 = _436 ^ (_436 >> 11u);
    uint _441 = _438 ^ ((_438 << 7u) & 2636928640u);
    uint _444 = _441 ^ ((_441 << 15u) & 4022730752u);
    float2 _454 = (_323 * _323) * (3.0f.xx - (_323 * 2.0f));
    float _455 = _454.x;
    float _456 = 1.0f - _455;
    float _458 = _454.y;
    float _474 = in_var_Vertex_Position.x + ((sin((((fmod(_132 * 1.7999999523162841796875f, 6.283185482025146484375f) + (frame_constants_game_time_frac * 1.7999999523162841796875f)) + (in_var_Vertex_Position.y * 1.5f)) + (_304 * 0.5f)) + ((((1.0f - _458) * (((asfloat(((_348 ^ (_348 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _456) + ((asfloat(((_380 ^ (_380 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _455))) + (((asfloat(((_412 ^ (_412 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _456) * _458)) + (((asfloat(((_444 ^ (_444 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _455) * _458))) * smoothstep(0.0f, -1.0f, in_var_Vertex_Position.y)) * (lerp(0.300000011920928955078125f, 1.0f, ((sin((((((1.0f - _274) * (((asfloat(((_164 ^ (_164 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _272) + ((asfloat(((_196 ^ (_196 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _271))) + (((asfloat(((_228 ^ (_228 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _272) * _274)) + (((asfloat(((_260 ^ (_260 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _271) * _274)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f) * 0.699999988079071044921875f) * lerp(0.699999988079071044921875f, 1.0f, _304)) * 0.300000011920928955078125f));
    float4 _477 = mul(model_to_world, float4(_474, in_var_Vertex_Position.yz, 1.0f));
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, _477);
    out_var_C0 = in_var_Vertex_Color;
    out_var_C1 = mul(model_to_world, float4(in_var_Vertex_Normal, 0.0f)).xyz;
    out_var_C2 = _477.xyz;
    out_var_C3 = ((((in_var_Vertex_UV * 2.0f) * float2(1.0f, 1.5f)) - 0.5f.xx) * 0.800000011920928955078125f) + 0.625f.xx;
    out_var_C4 = in_var_Vertex_UV;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
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
    int color_id : packoffset(c5);
    int seed : packoffset(c5.y);
    uint2 border_mask_dims : packoffset(c5.z);
    uint2 flag_palettes_dims : packoffset(c6);
    int bool_render_shadows : packoffset(c6.z);
};

SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_llc : register(s3, space0);
Texture2D<float4> border_mask : register(t14, space0);
Texture2D<float4> flag_palettes : register(t15, space0);
Texture2DArray<float4> flag_patterns : register(t16, space0);

static float3 in_var_C1;
static float3 in_var_C2;
static float2 in_var_C3;
static float2 in_var_C4;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 in_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 in_var_C4 : TEXCOORD4;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    uint _113 = uint(seed ^ 234234);
    uint _115 = _113 + (_113 << 10u);
    uint _117 = _115 ^ (_115 >> 6u);
    uint _119 = _117 + (_117 << 3u);
    uint _121 = _119 ^ (_119 >> 11u);
    float4 _141 = flag_patterns.SampleBias(sampler_llr, float3(in_var_C3.x, 1.0f - in_var_C3.y, float(int(round(lerp(9.0f, 11.0f, asfloat(((_121 + (_121 << 15u)) & 8388607u) | 1065353216u) - 1.0f))))), frame_constants_global_mip_bias);
    uint _150 = uint(color_id);
    float3 _168;
    if (border_mask.SampleBias(sampler_llc, in_var_C4, frame_constants_global_mip_bias).x > 0.89999997615814208984375f)
    {
        _168 = flag_palettes.Load(int3(uint2(1u, _150), 0u)).xyz;
    }
    else
    {
        _168 = lerp(flag_palettes.Load(int3(uint2(0u, _150), 0u)).xyz, flag_palettes.Load(int3(uint2(3u, _150), 0u)).xyz, (_141.x * 0.5f).xxx);
    }
    float4 _173 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 _185;
    if (dot(normalize((_173.xyz / _173.w.xxx) - in_var_C2), in_var_C1) <= 0.0f)
    {
        _185 = -in_var_C1;
    }
    else
    {
        _185 = in_var_C1;
    }
    float3 _188 = (normalize(_185) * 0.5f) + 0.5f.xxx;
    float3 _210 = sqrt(clamp(_168, 0.0f.xxx, 1.0f.xxx));
    uint _229 = (uint((clamp(_210.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_210.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_210.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 _230 = uint2(((uint((clamp(_188.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_188.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_188.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u, _229);
    _230.y = _229 | 117440512u;
    out_var_SV_TARGET0 = _230;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C4 = stage_input.in_var_C4;
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
    int color_id : packoffset(c5);
    int seed : packoffset(c5.y);
    uint2 border_mask_dims : packoffset(c5.z);
    uint2 flag_palettes_dims : packoffset(c6);
    int bool_render_shadows : packoffset(c6.z);
};


static float4 gl_Position;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static float2 out_var_C4;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float4 _123 = mul(model_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float _132 = float(frame_constants_game_time_sec);
    float _133 = _132 + frame_constants_game_time_frac;
    float2 _136 = (_123.xz * 0.100000001490116119384765625f) + (float2(_133, 0.0f) * 1.2000000476837158203125f);
    int2 _138 = int2(floor(_136));
    float2 _139 = frac(_136);
    uint2 _140 = uint2(_138);
    uint _142 = _140.y;
    uint _144 = _142 + (_142 << 10u);
    uint _146 = _144 ^ (_144 >> 6u);
    uint _148 = _146 + (_146 << 3u);
    uint _150 = _148 ^ (_148 >> 11u);
    uint _156 = (((_140.x * 1664525u) + (_150 + (_150 << 15u))) + 1013904223u) * 1664525u;
    uint _158 = _156 ^ (_156 >> 11u);
    uint _161 = _158 ^ ((_158 << 7u) & 2636928640u);
    uint _164 = _161 ^ ((_161 << 15u) & 4022730752u);
    uint2 _172 = uint2(_138 + int2(1, 0));
    uint _174 = _172.y;
    uint _176 = _174 + (_174 << 10u);
    uint _178 = _176 ^ (_176 >> 6u);
    uint _180 = _178 + (_178 << 3u);
    uint _182 = _180 ^ (_180 >> 11u);
    uint _188 = (((_172.x * 1664525u) + (_182 + (_182 << 15u))) + 1013904223u) * 1664525u;
    uint _190 = _188 ^ (_188 >> 11u);
    uint _193 = _190 ^ ((_190 << 7u) & 2636928640u);
    uint _196 = _193 ^ ((_193 << 15u) & 4022730752u);
    uint2 _204 = uint2(_138 + int2(0, 1));
    uint _206 = _204.y;
    uint _208 = _206 + (_206 << 10u);
    uint _210 = _208 ^ (_208 >> 6u);
    uint _212 = _210 + (_210 << 3u);
    uint _214 = _212 ^ (_212 >> 11u);
    uint _220 = (((_204.x * 1664525u) + (_214 + (_214 << 15u))) + 1013904223u) * 1664525u;
    uint _222 = _220 ^ (_220 >> 11u);
    uint _225 = _222 ^ ((_222 << 7u) & 2636928640u);
    uint _228 = _225 ^ ((_225 << 15u) & 4022730752u);
    uint2 _236 = uint2(_138 + int2(1, 1));
    uint _238 = _236.y;
    uint _240 = _238 + (_238 << 10u);
    uint _242 = _240 ^ (_240 >> 6u);
    uint _244 = _242 + (_242 << 3u);
    uint _246 = _244 ^ (_244 >> 11u);
    uint _252 = (((_236.x * 1664525u) + (_246 + (_246 << 15u))) + 1013904223u) * 1664525u;
    uint _254 = _252 ^ (_252 >> 11u);
    uint _257 = _254 ^ ((_254 << 7u) & 2636928640u);
    uint _260 = _257 ^ ((_257 << 15u) & 4022730752u);
    float2 _270 = (_139 * _139) * (3.0f.xx - (_139 * 2.0f));
    float _271 = _270.x;
    float _272 = 1.0f - _271;
    float _274 = _270.y;
    uint _290 = uint(seed);
    uint _292 = _290 + (_290 << 10u);
    uint _294 = _292 ^ (_292 >> 6u);
    uint _296 = _294 + (_294 << 3u);
    uint _298 = _296 ^ (_296 >> 11u);
    float _304 = asfloat(((_298 + (_298 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float2 _320 = float2(_133 + _123.x, _123.y);
    int2 _322 = int2(floor(_320));
    float2 _323 = frac(_320);
    uint2 _324 = uint2(_322);
    uint _326 = _324.y;
    uint _328 = _326 + (_326 << 10u);
    uint _330 = _328 ^ (_328 >> 6u);
    uint _332 = _330 + (_330 << 3u);
    uint _334 = _332 ^ (_332 >> 11u);
    uint _340 = (((_324.x * 1664525u) + (_334 + (_334 << 15u))) + 1013904223u) * 1664525u;
    uint _342 = _340 ^ (_340 >> 11u);
    uint _345 = _342 ^ ((_342 << 7u) & 2636928640u);
    uint _348 = _345 ^ ((_345 << 15u) & 4022730752u);
    uint2 _356 = uint2(_322 + int2(1, 0));
    uint _358 = _356.y;
    uint _360 = _358 + (_358 << 10u);
    uint _362 = _360 ^ (_360 >> 6u);
    uint _364 = _362 + (_362 << 3u);
    uint _366 = _364 ^ (_364 >> 11u);
    uint _372 = (((_356.x * 1664525u) + (_366 + (_366 << 15u))) + 1013904223u) * 1664525u;
    uint _374 = _372 ^ (_372 >> 11u);
    uint _377 = _374 ^ ((_374 << 7u) & 2636928640u);
    uint _380 = _377 ^ ((_377 << 15u) & 4022730752u);
    uint2 _388 = uint2(_322 + int2(0, 1));
    uint _390 = _388.y;
    uint _392 = _390 + (_390 << 10u);
    uint _394 = _392 ^ (_392 >> 6u);
    uint _396 = _394 + (_394 << 3u);
    uint _398 = _396 ^ (_396 >> 11u);
    uint _404 = (((_388.x * 1664525u) + (_398 + (_398 << 15u))) + 1013904223u) * 1664525u;
    uint _406 = _404 ^ (_404 >> 11u);
    uint _409 = _406 ^ ((_406 << 7u) & 2636928640u);
    uint _412 = _409 ^ ((_409 << 15u) & 4022730752u);
    uint2 _420 = uint2(_322 + int2(1, 1));
    uint _422 = _420.y;
    uint _424 = _422 + (_422 << 10u);
    uint _426 = _424 ^ (_424 >> 6u);
    uint _428 = _426 + (_426 << 3u);
    uint _430 = _428 ^ (_428 >> 11u);
    uint _436 = (((_420.x * 1664525u) + (_430 + (_430 << 15u))) + 1013904223u) * 1664525u;
    uint _438 = _436 ^ (_436 >> 11u);
    uint _441 = _438 ^ ((_438 << 7u) & 2636928640u);
    uint _444 = _441 ^ ((_441 << 15u) & 4022730752u);
    float2 _454 = (_323 * _323) * (3.0f.xx - (_323 * 2.0f));
    float _455 = _454.x;
    float _456 = 1.0f - _455;
    float _458 = _454.y;
    float _474 = in_var_Vertex_Position.x + ((sin((((fmod(_132 * 1.7999999523162841796875f, 6.283185482025146484375f) + (frame_constants_game_time_frac * 1.7999999523162841796875f)) + (in_var_Vertex_Position.y * 1.5f)) + (_304 * 0.5f)) + ((((1.0f - _458) * (((asfloat(((_348 ^ (_348 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _456) + ((asfloat(((_380 ^ (_380 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _455))) + (((asfloat(((_412 ^ (_412 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _456) * _458)) + (((asfloat(((_444 ^ (_444 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _455) * _458))) * smoothstep(0.0f, -1.0f, in_var_Vertex_Position.y)) * (lerp(0.300000011920928955078125f, 1.0f, ((sin((((((1.0f - _274) * (((asfloat(((_164 ^ (_164 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _272) + ((asfloat(((_196 ^ (_196 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _271))) + (((asfloat(((_228 ^ (_228 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _272) * _274)) + (((asfloat(((_260 ^ (_260 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _271) * _274)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f) * 0.699999988079071044921875f) * lerp(0.699999988079071044921875f, 1.0f, _304)) * 0.300000011920928955078125f));
    float4 _477 = mul(model_to_world, float4(_474, in_var_Vertex_Position.yz, 1.0f));
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, _477);
    out_var_C0 = in_var_Vertex_Color;
    out_var_C1 = mul(model_to_world, float4(in_var_Vertex_Normal, 0.0f)).xyz;
    out_var_C2 = _477.xyz;
    out_var_C3 = ((((in_var_Vertex_UV * 2.0f) * float2(1.0f, 1.5f)) - 0.5f.xx) * 0.800000011920928955078125f) + 0.625f.xx;
    out_var_C4 = in_var_Vertex_UV;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
void frag_main()
{
}

void ps_main()
{
    frag_main();
}
#pragma dxc diagnostic pop
#endif
