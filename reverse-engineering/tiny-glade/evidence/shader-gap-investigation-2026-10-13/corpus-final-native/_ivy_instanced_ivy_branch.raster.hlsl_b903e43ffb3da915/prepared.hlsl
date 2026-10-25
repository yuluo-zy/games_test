#define SHADER_VARIANT_DEPTH_ONLY 0
#define TGV_b903e43ffb3da915 1
#line 1 "shaders/ivy/instanced_ivy_branch.raster.hlsl"


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
#line 30 "shaders/ivy/instanced_ivy_branch.raster.hlsl"

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
#line 31 "shaders/ivy/instanced_ivy_branch.raster.hlsl"

#line 1 "shaders/ivy/arch_noise_discard.inc.hlsl"





















#ifndef TINYGLADE_IVY_ARCH_NOISE_DISCARD_HLSL
#define TINYGLADE_IVY_ARCH_NOISE_DISCARD_HLSL

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
#line 25 "shaders/ivy/arch_noise_discard.inc.hlsl"



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


float hash01(uint h)
{
    return asfloat(((h ^ (h >> 18u)) & 0x7FFFFFu) | 0x3F800000u) - 1.0f;
}


float perlin(float2 p)
{
    int2 cell = int2(floor(p));
    float2 f = frac(p);
    float2 u = f * f * (3.0f - 2.0f * f);
    float n00 = hash01(hash(uint2(cell + int2(0, 0))));
    float n10 = hash01(hash(uint2(cell + int2(1, 0))));
    float n01 = hash01(hash(uint2(cell + int2(0, 1))));
    float n11 = hash01(hash(uint2(cell + int2(1, 1))));
    return (1.0f - u.y) * (n00 * (1.0f - u.x) + n10 * u.x)
         + u.y * (n01 * (1.0f - u.x) + n11 * u.x);
}



uint hash3(uint3 p)
{
    uint h = hash(uint2(p.y, p.z));
    h = ((p.x * 1664525u) + (h ^ (h >> 18u)) + 1013904223u) * 1664525u;
    h ^= (h >> 11u);
    h ^= (h << 7u) & 2636928640u;
    h ^= (h << 15u) & 4022730752u;
    return h;
}




float2 hash3_01(uint3 cell, bool masked, uint seed)
{
    uint s1 = masked ? 1664525u : seed;
    uint s2 = masked ? 1013904223u : seed;
    uint mask = masked ? 8388607u : seed;
    uint off = masked ? 1065353216u : seed;

    uint hz = cell.z;
    hz += (hz << 10u);
    hz ^= (hz >> 6u);
    hz += (hz << 3u);
    hz ^= (hz >> 11u);
    uint h = ((cell.y * s1) + (hz + (hz << 15u)) + s2) * s1;
    h ^= (h >> 11u);
    h ^= (h << 7u) & 2636928640u;
    h ^= (h << 15u) & 4022730752u;
    uint hx = ((cell.x * s1) + (h ^ (h >> 18u)) + s2) * s1;
    hx ^= (hx >> 11u);
    hx ^= (hx << 7u) & 2636928640u;
    hx ^= (hx << 15u) & 4022730752u;

    float2 v;
    v.x = asfloat(((hx ^ (hx >> 18u)) & mask) | off) - 1.0f;
    uint h2 = hx ^ (hx >> 18u);
    h2 += (h2 << 10u);
    h2 ^= (h2 >> 6u);
    h2 += (h2 << 3u);
    h2 ^= (h2 >> 11u);
    h2 += (h2 << 15u);
    v.y = asfloat((h2 & mask) | off) - 1.0f;
    return v;
}


float2 perlin3(float3 p, bool masked, uint seed)
{
    int3 cell = int3(floor(p));
    float3 f = frac(p);
    float3 u = f * f * (3.0f - 2.0f * f);

    float2 n = hash3_01(uint3(cell + int3(0, 0, 0)), masked, seed)
        * (1.0f - u.x) * (1.0f - u.y) * (1.0f - u.z);
    n += hash3_01(uint3(cell + int3(1, 0, 0)), masked, seed)
        * u.x * (1.0f - u.y) * (1.0f - u.z);
    n += hash3_01(uint3(cell + int3(0, 1, 0)), masked, seed)
        * (1.0f - u.x) * u.y * (1.0f - u.z);
    n += hash3_01(uint3(cell + int3(1, 1, 0)), masked, seed)
        * u.x * u.y * (1.0f - u.z);
    n += hash3_01(uint3(cell + int3(0, 0, 1)), masked, seed)
        * (1.0f - u.x) * (1.0f - u.y) * u.z;
    n += hash3_01(uint3(cell + int3(1, 0, 1)), masked, seed)
        * u.x * (1.0f - u.y) * u.z;
    n += hash3_01(uint3(cell + int3(0, 1, 1)), masked, seed)
        * (1.0f - u.x) * u.y * u.z;
    n += hash3_01(uint3(cell + int3(1, 1, 1)), masked, seed)
        * u.x * u.y * u.z;
    return n;
}





bool arch_noise_discard(float3 pos, float height, float arch_height, float path)
{
    float base = arch_height - min(height, 0.0f);
    float fade = (1.0f - exp(-5.0f * path)) * base;
    float n = perlin(pos.xz * 0.6f + pos.y * 0.6f);
    float t = clamp(base - pos.y, 0.0f, 1.0f);
    return pos.y < fade * (t * t * clamp(smoothstep(0.0f, 0.3f, n), 0.8f, 1.0f));
}

#endif 
#line 32 "shaders/ivy/instanced_ivy_branch.raster.hlsl"





float4x4 model_to_world;
uint2 ivy_branch_color_dims;
uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
uint2 path_mask_dims;

struct IvySegment
{
    float3 start_pos;
    float start_thickness;
    float3 end_pos;
    float end_thickness;
    float3 wall_normal;
    float arch_height;
};

StructuredBuffer<IvySegment> ivy_buffer : register(t14, space0);
Texture2D<float4> terrain_heightmap : register(t16, space0);
Texture2D<float4> path_mask : register(t18, space0);
Texture2D<float4> ivy_branch_color : register(t15, space0);

struct VSInput
{
    [[vk::location(0)]] float3 position: Vertex_Position;
    uint instance : SV_InstanceID;
};

struct VSOutput
{
    float4 position : SV_Position;
    float3 normal : TEXCOORD1;
    float2 uv : TEXCOORD2;
    float3 world_pos : TEXCOORD3;
};

VSOutput vs_main(VSInput input)
{
    VSOutput output;
    float4 position;
    float3 normal;
    float2 uv;
    float3 world_pos;
    switch (0u)
    {
    default:
        float3 start = ivy_buffer[input.instance].start_pos;
        float3 end = ivy_buffer[input.instance].end_pos;
        float2 start_uv = start.xz * (0.0076923076994717121124267578125f).xx + 0.5f.xx;
        float hstart = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, start_uv, 0.0f).x);
        float2 end_uv = end.xz * (0.0076923076994717121124267578125f).xx + 0.5f.xx;
        float hend = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, end_uv, 0.0f).x);
        float start_thick = ivy_buffer[input.instance].start_thickness;
        float end_thick = ivy_buffer[input.instance].end_thickness;
        float3 wall_normal = ivy_buffer[input.instance].wall_normal;
        float arch_height = ivy_buffer[input.instance].arch_height;
        float base = arch_height - min(hstart, 0.0f);
        float path = path_mask.SampleLevel(sampler_llb, start_uv, 0.0f).x;
        bool over_path = path > 0.1f;

        float t0, t1;
        if (over_path)
        {
            float fade = (1.0f - exp(-5.0f * path)) * base;
            float n = perlin(start.xz * 0.6f + start.y * 0.6f);
            float t = clamp(base - start.y, 0.0f, 1.0f);
            float tt = t * t;
            float d = fade * (tt * clamp(smoothstep(0.0f, 0.3f, n), 0.8f, 1.0f));
            if (start.y < d)
            {
                position = asfloat(0x7fbfffffu).xxxx;
                normal = 0.0f.xxx;
                uv = 0.0f.xx;
                world_pos = 0.0f.xxx;
                break;
            }
            else if (start.y < fade)
            {
                float k = 1.0f - tt;
                t0 = min(start_thick * k, 0.2f);
                t1 = min(end_thick * k, 0.2f);
            }
            else
            {
                t0 = start_thick;
                t1 = end_thick;
            }
        }
        else
        {
            t0 = start_thick;
            t1 = end_thick;
        }

        float3 s = start;
        s.y = start.y + hstart;
        float3 e = end;
        e.y = end.y + hend;
        float3 dir = normalize(e - s);
        float3 side = normalize(cross(dir, wall_normal));
        float3 vpos;
        if (input.position.y < 0.1f)
        {
            float2 sc = input.position.xz * lerp(0.01679999940097332000732421875f, 0.083999998867511749267578125f, t0);
            vpos = float3(sc.x, input.position.y, sc.y);
        }
        else
        {
            float2 sc = input.position.xz * lerp(0.01679999940097332000732421875f, 0.083999998867511749267578125f, t1);
            vpos = float3(sc.x, input.position.y, sc.y);
        }
        float4x4 rot = transpose(float4x4(float4(side, 0.0f), float4(dir, 0.0f),
                                          float4(normalize(cross(side, dir)), 0.0f),
                                          float4(0.0f, 0.0f, 0.0f, 1.0f)));
        float4x4 trans = float4x4(float4(1.0f, 0.0f, 0.0f, start.x),
                                  float4(0.0f, 1.0f, 0.0f, s.y),
                                  float4(0.0f, 0.0f, 1.0f, start.z),
                                  float4(0.0f, 0.0f, 0.0f, 1.0f));
        float3 local = mul(trans, mul(rot, float4(vpos.x, vpos.y * (distance(s, e) * 1.1f), vpos.z, 1.0f))).xyz;
        uint alt_seed;
        float2 disp = clamp((perlin3(local * 0.4f, over_path, alt_seed) - 0.5f) * 2.5f, -0.5f, 0.5f);
        float3 world = mul(model_to_world, float4(local + (float3(disp.x, 0.0f, disp.y) * 0.06f), 1.0f)).xyz;
        position = mul(view_constants[current_view_index].world_to_sample, float4(world, 1.0f));
        normal = mul(rot, float4(normalize(float3(input.position.x, 0.0f, input.position.z)), 0.0f)).xyz;
        uv = float2(end.x + e.y * 0.5f, end.z + e.y * 0.5f) * 0.2f;
        world_pos = world;
        break;
    }
    output.position = position;
    output.normal = normal;
    output.uv = uv;
    output.world_pos = world_pos;
    return output;
}

struct PSInput
{
    float3 normal : TEXCOORD1;
    float2 uv : TEXCOORD2;
    float3 world_pos : TEXCOORD3;
};

uint2 ps_main(PSInput input) : SV_Target0
{
    float3 color = ivy_branch_color.Sample(sampler_llr, input.uv).xyz;
    float3 c;
    if (glade_theme_id == 4u)
    {
        float fade = smoothstep(-0.1f, 0.0f, input.world_pos.y) * smoothstep(0.0f, 0.7f, clamp(input.normal.y, 0.0f, 1.0f));
        c = lerp(lerp(color, 0.7f.xxx, 0.2f.xxx), 0.7f.xxx, fade.xxx);
    }
    else
    {
        c = color;
    }
    float3 packed_normal = input.normal * 0.5f + 0.5f.xxx;
    uint n = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
           + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
           + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
           + 4278190080u;
    float3 sc = sqrt(clamp(c, 0.0f.xxx, 1.0f.xxx));
    uint m = uint(clamp(sc.x, 0.0f, 1.0f) * 255.0f + 0.5f)
           + (uint(clamp(sc.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
           + (uint(clamp(sc.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
           + 436207616u;
    return uint2(n, m);
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

struct IvySegment
{
    float3 start_pos;
    float start_thickness;
    float3 end_pos;
    float end_thickness;
    float3 wall_normal;
    float arch_height;
};

static uint _133;

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
    uint2 ivy_branch_color_dims : packoffset(c5);
    uint2 terrain_heightmap_dims : packoffset(c5.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c6);
    uint2 path_mask_dims : packoffset(c6.z);
};

ByteAddressBuffer ivy_buffer : register(t14, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t16, space0);
Texture2D<float4> path_mask : register(t18, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 out_var_C1;
static float2 out_var_C2;
static float3 out_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float2 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float4 _1095;
    float3 _1096;
    float2 _1097;
    float3 _1098;
    do
    {
        float3 _157 = ivy_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 0);
        float3 _159 = ivy_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16);
        float2 _162 = (_157.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        float _168 = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _162, 0.0f).x);
        float _185 = ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 44) - min(_168, 0.0f);
        float4 _189 = path_mask.SampleLevel(sampler_llb, _162, 0.0f);
        float _190 = _189.x;
        bool _191 = _190 > 0.100000001490116119384765625f;
        float _375;
        float _376;
        if (_191)
        {
            float _197 = (1.0f - exp((-5.0f) * _190)) * _185;
            float2 _205 = (float2(_157.xz) * 0.60000002384185791015625f) + (_157.y * 0.60000002384185791015625f).xx;
            int2 _207 = int2(floor(_205));
            float2 _208 = frac(_205);
            uint2 _209 = uint2(_207);
            uint _211 = _209.y;
            uint _213 = _211 + (_211 << 10u);
            uint _215 = _213 ^ (_213 >> 6u);
            uint _217 = _215 + (_215 << 3u);
            uint _219 = _217 ^ (_217 >> 11u);
            uint _225 = (((_209.x * 1664525u) + (_219 + (_219 << 15u))) + 1013904223u) * 1664525u;
            uint _227 = _225 ^ (_225 >> 11u);
            uint _230 = _227 ^ ((_227 << 7u) & 2636928640u);
            uint _233 = _230 ^ ((_230 << 15u) & 4022730752u);
            uint2 _241 = uint2(_207 + int2(1, 0));
            uint _243 = _241.y;
            uint _245 = _243 + (_243 << 10u);
            uint _247 = _245 ^ (_245 >> 6u);
            uint _249 = _247 + (_247 << 3u);
            uint _251 = _249 ^ (_249 >> 11u);
            uint _257 = (((_241.x * 1664525u) + (_251 + (_251 << 15u))) + 1013904223u) * 1664525u;
            uint _259 = _257 ^ (_257 >> 11u);
            uint _262 = _259 ^ ((_259 << 7u) & 2636928640u);
            uint _265 = _262 ^ ((_262 << 15u) & 4022730752u);
            uint2 _273 = uint2(_207 + int2(0, 1));
            uint _275 = _273.y;
            uint _277 = _275 + (_275 << 10u);
            uint _279 = _277 ^ (_277 >> 6u);
            uint _281 = _279 + (_279 << 3u);
            uint _283 = _281 ^ (_281 >> 11u);
            uint _289 = (((_273.x * 1664525u) + (_283 + (_283 << 15u))) + 1013904223u) * 1664525u;
            uint _291 = _289 ^ (_289 >> 11u);
            uint _294 = _291 ^ ((_291 << 7u) & 2636928640u);
            uint _297 = _294 ^ ((_294 << 15u) & 4022730752u);
            uint2 _305 = uint2(_207 + int2(1, 1));
            uint _307 = _305.y;
            uint _309 = _307 + (_307 << 10u);
            uint _311 = _309 ^ (_309 >> 6u);
            uint _313 = _311 + (_311 << 3u);
            uint _315 = _313 ^ (_313 >> 11u);
            uint _321 = (((_305.x * 1664525u) + (_315 + (_315 << 15u))) + 1013904223u) * 1664525u;
            uint _323 = _321 ^ (_321 >> 11u);
            uint _326 = _323 ^ ((_323 << 7u) & 2636928640u);
            uint _329 = _326 ^ ((_326 << 15u) & 4022730752u);
            float2 _339 = (_208 * _208) * (3.0f.xx - (_208 * 2.0f));
            float _340 = _339.x;
            float _341 = 1.0f - _340;
            float _343 = _339.y;
            float _357 = clamp(_185 - _157.y, 0.0f, 1.0f);
            float _358 = _357 * _357;
            float _373;
            float _374;
            if (_157.y < (_197 * (_358 * clamp(smoothstep(0.0f, 0.300000011920928955078125f, (((1.0f - _343) * (((asfloat(((_233 ^ (_233 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _341) + ((asfloat(((_265 ^ (_265 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _340))) + (((asfloat(((_297 ^ (_297 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _341) * _343)) + (((asfloat(((_329 ^ (_329 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _340) * _343)), 0.800000011920928955078125f, 1.0f))))
            {
                _1095 = asfloat(0x7fbfffffu          ).xxxx;
                _1096 = 0.0f.xxx;
                _1097 = 0.0f.xx;
                _1098 = 0.0f.xxx;
                break;
            }
            else
            {
                if (_157.y < _197)
                {
                    float _368 = 1.0f - _358;
                    _373 = min(ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 12) * _368, 0.20000000298023223876953125f);
                    _374 = min(ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 28) * _368, 0.20000000298023223876953125f);
                }
                else
                {
                    _373 = ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 12);
                    _374 = ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 28);
                }
            }
            _375 = _373;
            _376 = _374;
        }
        else
        {
            _375 = ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 12);
            _376 = ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 28);
        }
        bool _379 = _191 ? true : false;
        float _383 = _157.y + _168;
        float3 _384 = _157;
        _384.y = _383;
        float _386 = _159.y + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_159.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
        float3 _387 = _159;
        _387.y = _386;
        float3 _390 = normalize(_387 - _384);
        float3 _392 = normalize(cross(_390, ivy_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 32)));
        float3 _408;
        if (in_var_Vertex_Position.y < 0.100000001490116119384765625f)
        {
            float2 _406 = in_var_Vertex_Position.xz * lerp(0.01679999940097332000732421875f, 0.083999998867511749267578125f, _375);
            _408 = float3(_406.x, in_var_Vertex_Position.y, _406.y);
        }
        else
        {
            float2 _402 = in_var_Vertex_Position.xz * lerp(0.01679999940097332000732421875f, 0.083999998867511749267578125f, _376);
            _408 = float3(_402.x, in_var_Vertex_Position.y, _402.y);
        }
        float4x4 _435 = transpose(float4x4(float4(_392, 0.0f), float4(_390, 0.0f), float4(normalize(cross(_392, _390)), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
        float3 _441 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _157.x), float4(0.0f, 1.0f, 0.0f, _383), float4(0.0f, 0.0f, 1.0f, _157.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(_435, float4(_408.x, _408.y * (distance(_384, _387) * 1.10000002384185791015625f), _408.z, 1.0f))).xyz;
        float3 _442 = _441 * 0.4000000059604644775390625f;
        int3 _444 = int3(floor(_442));
        float3 _445 = frac(_442);
        uint3 _446 = uint3(_444);
        uint _449 = _446.z;
        uint _451 = _449 + (_449 << 10u);
        uint _453 = _451 ^ (_451 >> 6u);
        uint _455 = _453 + (_453 << 3u);
        uint _457 = _455 ^ (_455 >> 11u);
        bool _460 = _379 ? _379 : true;
        uint _461 = _379 ? (_191 ? 1664525u : _133) : 1664525u;
        uint _462 = _379 ? (_191 ? 1013904223u : _133) : 1013904223u;
        uint _466 = (((_446.y * _461) + (_457 + (_457 << 15u))) + _462) * _461;
        uint _468 = _466 ^ (_466 >> 11u);
        uint _471 = _468 ^ ((_468 << 7u) & 2636928640u);
        uint _474 = _471 ^ ((_471 << 15u) & 4022730752u);
        bool _477 = _460 ? _460 : true;
        uint _478 = _460 ? _461 : 1664525u;
        uint _479 = _460 ? _462 : 1013904223u;
        uint _483 = (((_446.x * _478) + (_474 ^ (_474 >> 18u))) + _479) * _478;
        uint _485 = _483 ^ (_483 >> 11u);
        uint _488 = _485 ^ ((_485 << 7u) & 2636928640u);
        uint _491 = _488 ^ ((_488 << 15u) & 4022730752u);
        uint _493 = _491 ^ (_491 >> 18u);
        uint _495 = _493 + (_493 << 10u);
        uint _497 = _495 ^ (_495 >> 6u);
        uint _499 = _497 + (_497 << 3u);
        uint _501 = _499 ^ (_499 >> 11u);
        uint _504 = _379 ? (_191 ? 8388607u : _133) : 8388607u;
        uint _505 = _379 ? (_191 ? 1065353216u : _133) : 1065353216u;
        uint _510 = _460 ? _504 : 8388607u;
        uint _511 = _460 ? _505 : 1065353216u;
        uint3 _518 = uint3(_444 + int3(1, 0, 0));
        uint _521 = _518.z;
        uint _523 = _521 + (_521 << 10u);
        uint _525 = _523 ^ (_523 >> 6u);
        uint _527 = _525 + (_525 << 3u);
        uint _529 = _527 ^ (_527 >> 11u);
        bool _532 = _477 ? _477 : true;
        uint _533 = _477 ? _478 : 1664525u;
        uint _534 = _477 ? _479 : 1013904223u;
        uint _538 = (((_518.y * _533) + (_529 + (_529 << 15u))) + _534) * _533;
        uint _540 = _538 ^ (_538 >> 11u);
        uint _543 = _540 ^ ((_540 << 7u) & 2636928640u);
        uint _546 = _543 ^ ((_543 << 15u) & 4022730752u);
        bool _549 = _532 ? _532 : true;
        uint _550 = _532 ? _533 : 1664525u;
        uint _551 = _532 ? _534 : 1013904223u;
        uint _555 = (((_518.x * _550) + (_546 ^ (_546 >> 18u))) + _551) * _550;
        uint _557 = _555 ^ (_555 >> 11u);
        uint _560 = _557 ^ ((_557 << 7u) & 2636928640u);
        uint _563 = _560 ^ ((_560 << 15u) & 4022730752u);
        uint _565 = _563 ^ (_563 >> 18u);
        uint _567 = _565 + (_565 << 10u);
        uint _569 = _567 ^ (_567 >> 6u);
        uint _571 = _569 + (_569 << 3u);
        uint _573 = _571 ^ (_571 >> 11u);
        uint _576 = _477 ? _510 : 8388607u;
        uint _577 = _477 ? _511 : 1065353216u;
        uint _582 = _532 ? _576 : 8388607u;
        uint _583 = _532 ? _577 : 1065353216u;
        uint3 _590 = uint3(_444 + int3(0, 1, 0));
        uint _593 = _590.z;
        uint _595 = _593 + (_593 << 10u);
        uint _597 = _595 ^ (_595 >> 6u);
        uint _599 = _597 + (_597 << 3u);
        uint _601 = _599 ^ (_599 >> 11u);
        bool _604 = _549 ? _549 : true;
        uint _605 = _549 ? _550 : 1664525u;
        uint _606 = _549 ? _551 : 1013904223u;
        uint _610 = (((_590.y * _605) + (_601 + (_601 << 15u))) + _606) * _605;
        uint _612 = _610 ^ (_610 >> 11u);
        uint _615 = _612 ^ ((_612 << 7u) & 2636928640u);
        uint _618 = _615 ^ ((_615 << 15u) & 4022730752u);
        bool _621 = _604 ? _604 : true;
        uint _622 = _604 ? _605 : 1664525u;
        uint _623 = _604 ? _606 : 1013904223u;
        uint _627 = (((_590.x * _622) + (_618 ^ (_618 >> 18u))) + _623) * _622;
        uint _629 = _627 ^ (_627 >> 11u);
        uint _632 = _629 ^ ((_629 << 7u) & 2636928640u);
        uint _635 = _632 ^ ((_632 << 15u) & 4022730752u);
        uint _637 = _635 ^ (_635 >> 18u);
        uint _639 = _637 + (_637 << 10u);
        uint _641 = _639 ^ (_639 >> 6u);
        uint _643 = _641 + (_641 << 3u);
        uint _645 = _643 ^ (_643 >> 11u);
        uint _648 = _549 ? _582 : 8388607u;
        uint _649 = _549 ? _583 : 1065353216u;
        uint _654 = _604 ? _648 : 8388607u;
        uint _655 = _604 ? _649 : 1065353216u;
        uint3 _662 = uint3(_444 + int3(1, 1, 0));
        uint _665 = _662.z;
        uint _667 = _665 + (_665 << 10u);
        uint _669 = _667 ^ (_667 >> 6u);
        uint _671 = _669 + (_669 << 3u);
        uint _673 = _671 ^ (_671 >> 11u);
        bool _676 = _621 ? _621 : true;
        uint _677 = _621 ? _622 : 1664525u;
        uint _678 = _621 ? _623 : 1013904223u;
        uint _682 = (((_662.y * _677) + (_673 + (_673 << 15u))) + _678) * _677;
        uint _684 = _682 ^ (_682 >> 11u);
        uint _687 = _684 ^ ((_684 << 7u) & 2636928640u);
        uint _690 = _687 ^ ((_687 << 15u) & 4022730752u);
        bool _693 = _676 ? _676 : true;
        uint _694 = _676 ? _677 : 1664525u;
        uint _695 = _676 ? _678 : 1013904223u;
        uint _699 = (((_662.x * _694) + (_690 ^ (_690 >> 18u))) + _695) * _694;
        uint _701 = _699 ^ (_699 >> 11u);
        uint _704 = _701 ^ ((_701 << 7u) & 2636928640u);
        uint _707 = _704 ^ ((_704 << 15u) & 4022730752u);
        uint _709 = _707 ^ (_707 >> 18u);
        uint _711 = _709 + (_709 << 10u);
        uint _713 = _711 ^ (_711 >> 6u);
        uint _715 = _713 + (_713 << 3u);
        uint _717 = _715 ^ (_715 >> 11u);
        uint _720 = _621 ? _654 : 8388607u;
        uint _721 = _621 ? _655 : 1065353216u;
        uint _726 = _676 ? _720 : 8388607u;
        uint _727 = _676 ? _721 : 1065353216u;
        uint3 _734 = uint3(_444 + int3(0, 0, 1));
        uint _737 = _734.z;
        uint _739 = _737 + (_737 << 10u);
        uint _741 = _739 ^ (_739 >> 6u);
        uint _743 = _741 + (_741 << 3u);
        uint _745 = _743 ^ (_743 >> 11u);
        bool _748 = _693 ? _693 : true;
        uint _749 = _693 ? _694 : 1664525u;
        uint _750 = _693 ? _695 : 1013904223u;
        uint _754 = (((_734.y * _749) + (_745 + (_745 << 15u))) + _750) * _749;
        uint _756 = _754 ^ (_754 >> 11u);
        uint _759 = _756 ^ ((_756 << 7u) & 2636928640u);
        uint _762 = _759 ^ ((_759 << 15u) & 4022730752u);
        bool _765 = _748 ? _748 : true;
        uint _766 = _748 ? _749 : 1664525u;
        uint _767 = _748 ? _750 : 1013904223u;
        uint _771 = (((_734.x * _766) + (_762 ^ (_762 >> 18u))) + _767) * _766;
        uint _773 = _771 ^ (_771 >> 11u);
        uint _776 = _773 ^ ((_773 << 7u) & 2636928640u);
        uint _779 = _776 ^ ((_776 << 15u) & 4022730752u);
        uint _781 = _779 ^ (_779 >> 18u);
        uint _783 = _781 + (_781 << 10u);
        uint _785 = _783 ^ (_783 >> 6u);
        uint _787 = _785 + (_785 << 3u);
        uint _789 = _787 ^ (_787 >> 11u);
        uint _792 = _693 ? _726 : 8388607u;
        uint _793 = _693 ? _727 : 1065353216u;
        uint _798 = _748 ? _792 : 8388607u;
        uint _799 = _748 ? _793 : 1065353216u;
        uint3 _806 = uint3(_444 + int3(1, 0, 1));
        uint _809 = _806.z;
        uint _811 = _809 + (_809 << 10u);
        uint _813 = _811 ^ (_811 >> 6u);
        uint _815 = _813 + (_813 << 3u);
        uint _817 = _815 ^ (_815 >> 11u);
        bool _820 = _765 ? _765 : true;
        uint _821 = _765 ? _766 : 1664525u;
        uint _822 = _765 ? _767 : 1013904223u;
        uint _826 = (((_806.y * _821) + (_817 + (_817 << 15u))) + _822) * _821;
        uint _828 = _826 ^ (_826 >> 11u);
        uint _831 = _828 ^ ((_828 << 7u) & 2636928640u);
        uint _834 = _831 ^ ((_831 << 15u) & 4022730752u);
        bool _837 = _820 ? _820 : true;
        uint _838 = _820 ? _821 : 1664525u;
        uint _839 = _820 ? _822 : 1013904223u;
        uint _843 = (((_806.x * _838) + (_834 ^ (_834 >> 18u))) + _839) * _838;
        uint _845 = _843 ^ (_843 >> 11u);
        uint _848 = _845 ^ ((_845 << 7u) & 2636928640u);
        uint _851 = _848 ^ ((_848 << 15u) & 4022730752u);
        uint _853 = _851 ^ (_851 >> 18u);
        uint _855 = _853 + (_853 << 10u);
        uint _857 = _855 ^ (_855 >> 6u);
        uint _859 = _857 + (_857 << 3u);
        uint _861 = _859 ^ (_859 >> 11u);
        uint _864 = _765 ? _798 : 8388607u;
        uint _865 = _765 ? _799 : 1065353216u;
        uint _870 = _820 ? _864 : 8388607u;
        uint _871 = _820 ? _865 : 1065353216u;
        uint3 _878 = uint3(_444 + int3(0, 1, 1));
        uint _881 = _878.z;
        uint _883 = _881 + (_881 << 10u);
        uint _885 = _883 ^ (_883 >> 6u);
        uint _887 = _885 + (_885 << 3u);
        uint _889 = _887 ^ (_887 >> 11u);
        bool _892 = _837 ? _837 : true;
        uint _893 = _837 ? _838 : 1664525u;
        uint _894 = _837 ? _839 : 1013904223u;
        uint _898 = (((_878.y * _893) + (_889 + (_889 << 15u))) + _894) * _893;
        uint _900 = _898 ^ (_898 >> 11u);
        uint _903 = _900 ^ ((_900 << 7u) & 2636928640u);
        uint _906 = _903 ^ ((_903 << 15u) & 4022730752u);
        bool _909 = _892 ? _892 : true;
        uint _910 = _892 ? _893 : 1664525u;
        uint _911 = _892 ? _894 : 1013904223u;
        uint _915 = (((_878.x * _910) + (_906 ^ (_906 >> 18u))) + _911) * _910;
        uint _917 = _915 ^ (_915 >> 11u);
        uint _920 = _917 ^ ((_917 << 7u) & 2636928640u);
        uint _923 = _920 ^ ((_920 << 15u) & 4022730752u);
        uint _925 = _923 ^ (_923 >> 18u);
        uint _927 = _925 + (_925 << 10u);
        uint _929 = _927 ^ (_927 >> 6u);
        uint _931 = _929 + (_929 << 3u);
        uint _933 = _931 ^ (_931 >> 11u);
        uint _936 = _837 ? _870 : 8388607u;
        uint _937 = _837 ? _871 : 1065353216u;
        uint _942 = _892 ? _936 : 8388607u;
        uint _943 = _892 ? _937 : 1065353216u;
        uint3 _950 = uint3(_444 + int3(1, 1, 1));
        uint _953 = _950.z;
        uint _955 = _953 + (_953 << 10u);
        uint _957 = _955 ^ (_955 >> 6u);
        uint _959 = _957 + (_957 << 3u);
        uint _961 = _959 ^ (_959 >> 11u);
        bool _964 = _909 ? _909 : true;
        uint _965 = _909 ? _910 : 1664525u;
        uint _966 = _909 ? _911 : 1013904223u;
        uint _970 = (((_950.y * _965) + (_961 + (_961 << 15u))) + _966) * _965;
        uint _972 = _970 ^ (_970 >> 11u);
        uint _975 = _972 ^ ((_972 << 7u) & 2636928640u);
        uint _978 = _975 ^ ((_975 << 15u) & 4022730752u);
        uint _981 = _964 ? _965 : 1664525u;
        uint _986 = (((_950.x * _981) + (_978 ^ (_978 >> 18u))) + (_964 ? _966 : 1013904223u)) * _981;
        uint _988 = _986 ^ (_986 >> 11u);
        uint _991 = _988 ^ ((_988 << 7u) & 2636928640u);
        uint _994 = _991 ^ ((_991 << 15u) & 4022730752u);
        uint _996 = _994 ^ (_994 >> 18u);
        uint _998 = _996 + (_996 << 10u);
        uint _1000 = _998 ^ (_998 >> 6u);
        uint _1002 = _1000 + (_1000 << 3u);
        uint _1004 = _1002 ^ (_1002 >> 11u);
        uint _1007 = _909 ? _942 : 8388607u;
        uint _1008 = _909 ? _943 : 1065353216u;
        float3 _1023 = (_445 * _445) * (3.0f.xxx - (_445 * 2.0f));
        float _1024 = _1023.x;
        float _1025 = 1.0f - _1024;
        float _1027 = _1023.y;
        float _1028 = 1.0f - _1027;
        float _1030 = _1023.z;
        float _1031 = 1.0f - _1030;
        float2 _1044 = (((((float2(asfloat((_493 & _504) | _505) - 1.0f, asfloat(((_501 + (_501 << 15u)) & _510) | _511) - 1.0f) * _1025) * _1028) * _1031) + (((float2(asfloat((_565 & _576) | _577) - 1.0f, asfloat(((_573 + (_573 << 15u)) & _582) | _583) - 1.0f) * _1024) * _1028) * _1031)) + (((float2(asfloat((_637 & _648) | _649) - 1.0f, asfloat(((_645 + (_645 << 15u)) & _654) | _655) - 1.0f) * _1025) * _1027) * _1031)) + (((float2(asfloat((_709 & _720) | _721) - 1.0f, asfloat(((_717 + (_717 << 15u)) & _726) | _727) - 1.0f) * _1024) * _1027) * _1031);
        float2 _1060 = (((_1044 + (((float2(asfloat((_781 & _792) | _793) - 1.0f, asfloat(((_789 + (_789 << 15u)) & _798) | _799) - 1.0f) * _1025) * _1028) * _1030)) + (((float2(asfloat((_853 & _864) | _865) - 1.0f, asfloat(((_861 + (_861 << 15u)) & _870) | _871) - 1.0f) * _1024) * _1028) * _1030)) + (((float2(asfloat((_925 & _936) | _937) - 1.0f, asfloat(((_933 + (_933 << 15u)) & _942) | _943) - 1.0f) * _1025) * _1027) * _1030)) + (((float2(asfloat((_996 & _1007) | _1008) - 1.0f, asfloat(((_1004 + (_1004 << 15u)) & (_964 ? _1007 : 8388607u)) | (_964 ? _1008 : 1065353216u)) - 1.0f) * _1024) * _1027) * _1030);
        float2 _1063 = clamp((_1060 - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
        float4 _1075 = mul(model_to_world, float4(_441 + (float3(_1063.x, 0.0f, _1063.y) * 0.0599999986588954925537109375f), 1.0f));
        float _1088 = _386 * 0.5f;
        _1095 = mul(frame_constants_view_constants[current_view_index].world_to_sample, _1075);
        _1096 = mul(_435, float4(normalize(float3(in_var_Vertex_Position.x, 0.0f, in_var_Vertex_Position.z)), 0.0f)).xyz;
        _1097 = float2(_159.x + _1088, _159.z + _1088) * 0.20000000298023223876953125f;
        _1098 = _1075.xyz;
        break;
    } while(false);
    gl_Position = _1095;
    out_var_C1 = _1096;
    out_var_C2 = _1097;
    out_var_C3 = _1098;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
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

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> ivy_branch_color : register(t15, space0);

static float3 in_var_C1;
static float2 in_var_C2;
static float3 in_var_C3;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float2 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    float4 _90 = ivy_branch_color.Sample(sampler_llr, in_var_C2);
    float3 _91 = _90.xyz;
    float3 _106;
    if (frame_constants_glade_theme_id == 4u)
    {
        _106 = lerp(lerp(_91, 0.699999988079071044921875f.xxx, 0.20000000298023223876953125f.xxx), 0.699999988079071044921875f.xxx, (smoothstep(-0.100000001490116119384765625f, 0.0f, in_var_C3.y) * smoothstep(0.0f, 0.699999988079071044921875f, clamp(in_var_C1.y, 0.0f, 1.0f))).xxx);
    }
    else
    {
        _106 = _91;
    }
    float3 _108 = (in_var_C1 * 0.5f) + 0.5f.xxx;
    float3 _130 = sqrt(clamp(_106, 0.0f.xxx, 1.0f.xxx));
    out_var_SV_TARGET0 = uint2(((uint((clamp(_108.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_108.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_108.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u, ((uint((clamp(_130.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_130.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_130.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 436207616u);
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
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

struct IvySegment
{
    float3 start_pos;
    float start_thickness;
    float3 end_pos;
    float end_thickness;
    float3 wall_normal;
    float arch_height;
};

static uint _133;

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
    uint2 ivy_branch_color_dims : packoffset(c5);
    uint2 terrain_heightmap_dims : packoffset(c5.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c6);
    uint2 path_mask_dims : packoffset(c6.z);
};

ByteAddressBuffer ivy_buffer : register(t14, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t16, space0);
Texture2D<float4> path_mask : register(t18, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 out_var_C1;
static float2 out_var_C2;
static float3 out_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float2 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float4 _1095;
    float3 _1096;
    float2 _1097;
    float3 _1098;
    do
    {
        float3 _157 = ivy_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 0);
        float3 _159 = ivy_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16);
        float2 _162 = (_157.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        float _168 = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _162, 0.0f).x);
        float _185 = ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 44) - min(_168, 0.0f);
        float4 _189 = path_mask.SampleLevel(sampler_llb, _162, 0.0f);
        float _190 = _189.x;
        bool _191 = _190 > 0.100000001490116119384765625f;
        float _375;
        float _376;
        if (_191)
        {
            float _197 = (1.0f - exp((-5.0f) * _190)) * _185;
            float2 _205 = (float2(_157.xz) * 0.60000002384185791015625f) + (_157.y * 0.60000002384185791015625f).xx;
            int2 _207 = int2(floor(_205));
            float2 _208 = frac(_205);
            uint2 _209 = uint2(_207);
            uint _211 = _209.y;
            uint _213 = _211 + (_211 << 10u);
            uint _215 = _213 ^ (_213 >> 6u);
            uint _217 = _215 + (_215 << 3u);
            uint _219 = _217 ^ (_217 >> 11u);
            uint _225 = (((_209.x * 1664525u) + (_219 + (_219 << 15u))) + 1013904223u) * 1664525u;
            uint _227 = _225 ^ (_225 >> 11u);
            uint _230 = _227 ^ ((_227 << 7u) & 2636928640u);
            uint _233 = _230 ^ ((_230 << 15u) & 4022730752u);
            uint2 _241 = uint2(_207 + int2(1, 0));
            uint _243 = _241.y;
            uint _245 = _243 + (_243 << 10u);
            uint _247 = _245 ^ (_245 >> 6u);
            uint _249 = _247 + (_247 << 3u);
            uint _251 = _249 ^ (_249 >> 11u);
            uint _257 = (((_241.x * 1664525u) + (_251 + (_251 << 15u))) + 1013904223u) * 1664525u;
            uint _259 = _257 ^ (_257 >> 11u);
            uint _262 = _259 ^ ((_259 << 7u) & 2636928640u);
            uint _265 = _262 ^ ((_262 << 15u) & 4022730752u);
            uint2 _273 = uint2(_207 + int2(0, 1));
            uint _275 = _273.y;
            uint _277 = _275 + (_275 << 10u);
            uint _279 = _277 ^ (_277 >> 6u);
            uint _281 = _279 + (_279 << 3u);
            uint _283 = _281 ^ (_281 >> 11u);
            uint _289 = (((_273.x * 1664525u) + (_283 + (_283 << 15u))) + 1013904223u) * 1664525u;
            uint _291 = _289 ^ (_289 >> 11u);
            uint _294 = _291 ^ ((_291 << 7u) & 2636928640u);
            uint _297 = _294 ^ ((_294 << 15u) & 4022730752u);
            uint2 _305 = uint2(_207 + int2(1, 1));
            uint _307 = _305.y;
            uint _309 = _307 + (_307 << 10u);
            uint _311 = _309 ^ (_309 >> 6u);
            uint _313 = _311 + (_311 << 3u);
            uint _315 = _313 ^ (_313 >> 11u);
            uint _321 = (((_305.x * 1664525u) + (_315 + (_315 << 15u))) + 1013904223u) * 1664525u;
            uint _323 = _321 ^ (_321 >> 11u);
            uint _326 = _323 ^ ((_323 << 7u) & 2636928640u);
            uint _329 = _326 ^ ((_326 << 15u) & 4022730752u);
            float2 _339 = (_208 * _208) * (3.0f.xx - (_208 * 2.0f));
            float _340 = _339.x;
            float _341 = 1.0f - _340;
            float _343 = _339.y;
            float _357 = clamp(_185 - _157.y, 0.0f, 1.0f);
            float _358 = _357 * _357;
            float _373;
            float _374;
            if (_157.y < (_197 * (_358 * clamp(smoothstep(0.0f, 0.300000011920928955078125f, (((1.0f - _343) * (((asfloat(((_233 ^ (_233 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _341) + ((asfloat(((_265 ^ (_265 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _340))) + (((asfloat(((_297 ^ (_297 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _341) * _343)) + (((asfloat(((_329 ^ (_329 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _340) * _343)), 0.800000011920928955078125f, 1.0f))))
            {
                _1095 = asfloat(0x7fbfffffu          ).xxxx;
                _1096 = 0.0f.xxx;
                _1097 = 0.0f.xx;
                _1098 = 0.0f.xxx;
                break;
            }
            else
            {
                if (_157.y < _197)
                {
                    float _368 = 1.0f - _358;
                    _373 = min(ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 12) * _368, 0.20000000298023223876953125f);
                    _374 = min(ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 28) * _368, 0.20000000298023223876953125f);
                }
                else
                {
                    _373 = ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 12);
                    _374 = ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 28);
                }
            }
            _375 = _373;
            _376 = _374;
        }
        else
        {
            _375 = ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 12);
            _376 = ivy_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 28);
        }
        bool _379 = _191 ? true : false;
        float _383 = _157.y + _168;
        float3 _384 = _157;
        _384.y = _383;
        float _386 = _159.y + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_159.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
        float3 _387 = _159;
        _387.y = _386;
        float3 _390 = normalize(_387 - _384);
        float3 _392 = normalize(cross(_390, ivy_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 32)));
        float3 _408;
        if (in_var_Vertex_Position.y < 0.100000001490116119384765625f)
        {
            float2 _406 = in_var_Vertex_Position.xz * lerp(0.01679999940097332000732421875f, 0.083999998867511749267578125f, _375);
            _408 = float3(_406.x, in_var_Vertex_Position.y, _406.y);
        }
        else
        {
            float2 _402 = in_var_Vertex_Position.xz * lerp(0.01679999940097332000732421875f, 0.083999998867511749267578125f, _376);
            _408 = float3(_402.x, in_var_Vertex_Position.y, _402.y);
        }
        float4x4 _435 = transpose(float4x4(float4(_392, 0.0f), float4(_390, 0.0f), float4(normalize(cross(_392, _390)), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
        float3 _441 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _157.x), float4(0.0f, 1.0f, 0.0f, _383), float4(0.0f, 0.0f, 1.0f, _157.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(_435, float4(_408.x, _408.y * (distance(_384, _387) * 1.10000002384185791015625f), _408.z, 1.0f))).xyz;
        float3 _442 = _441 * 0.4000000059604644775390625f;
        int3 _444 = int3(floor(_442));
        float3 _445 = frac(_442);
        uint3 _446 = uint3(_444);
        uint _449 = _446.z;
        uint _451 = _449 + (_449 << 10u);
        uint _453 = _451 ^ (_451 >> 6u);
        uint _455 = _453 + (_453 << 3u);
        uint _457 = _455 ^ (_455 >> 11u);
        bool _460 = _379 ? _379 : true;
        uint _461 = _379 ? (_191 ? 1664525u : _133) : 1664525u;
        uint _462 = _379 ? (_191 ? 1013904223u : _133) : 1013904223u;
        uint _466 = (((_446.y * _461) + (_457 + (_457 << 15u))) + _462) * _461;
        uint _468 = _466 ^ (_466 >> 11u);
        uint _471 = _468 ^ ((_468 << 7u) & 2636928640u);
        uint _474 = _471 ^ ((_471 << 15u) & 4022730752u);
        bool _477 = _460 ? _460 : true;
        uint _478 = _460 ? _461 : 1664525u;
        uint _479 = _460 ? _462 : 1013904223u;
        uint _483 = (((_446.x * _478) + (_474 ^ (_474 >> 18u))) + _479) * _478;
        uint _485 = _483 ^ (_483 >> 11u);
        uint _488 = _485 ^ ((_485 << 7u) & 2636928640u);
        uint _491 = _488 ^ ((_488 << 15u) & 4022730752u);
        uint _493 = _491 ^ (_491 >> 18u);
        uint _495 = _493 + (_493 << 10u);
        uint _497 = _495 ^ (_495 >> 6u);
        uint _499 = _497 + (_497 << 3u);
        uint _501 = _499 ^ (_499 >> 11u);
        uint _504 = _379 ? (_191 ? 8388607u : _133) : 8388607u;
        uint _505 = _379 ? (_191 ? 1065353216u : _133) : 1065353216u;
        uint _510 = _460 ? _504 : 8388607u;
        uint _511 = _460 ? _505 : 1065353216u;
        uint3 _518 = uint3(_444 + int3(1, 0, 0));
        uint _521 = _518.z;
        uint _523 = _521 + (_521 << 10u);
        uint _525 = _523 ^ (_523 >> 6u);
        uint _527 = _525 + (_525 << 3u);
        uint _529 = _527 ^ (_527 >> 11u);
        bool _532 = _477 ? _477 : true;
        uint _533 = _477 ? _478 : 1664525u;
        uint _534 = _477 ? _479 : 1013904223u;
        uint _538 = (((_518.y * _533) + (_529 + (_529 << 15u))) + _534) * _533;
        uint _540 = _538 ^ (_538 >> 11u);
        uint _543 = _540 ^ ((_540 << 7u) & 2636928640u);
        uint _546 = _543 ^ ((_543 << 15u) & 4022730752u);
        bool _549 = _532 ? _532 : true;
        uint _550 = _532 ? _533 : 1664525u;
        uint _551 = _532 ? _534 : 1013904223u;
        uint _555 = (((_518.x * _550) + (_546 ^ (_546 >> 18u))) + _551) * _550;
        uint _557 = _555 ^ (_555 >> 11u);
        uint _560 = _557 ^ ((_557 << 7u) & 2636928640u);
        uint _563 = _560 ^ ((_560 << 15u) & 4022730752u);
        uint _565 = _563 ^ (_563 >> 18u);
        uint _567 = _565 + (_565 << 10u);
        uint _569 = _567 ^ (_567 >> 6u);
        uint _571 = _569 + (_569 << 3u);
        uint _573 = _571 ^ (_571 >> 11u);
        uint _576 = _477 ? _510 : 8388607u;
        uint _577 = _477 ? _511 : 1065353216u;
        uint _582 = _532 ? _576 : 8388607u;
        uint _583 = _532 ? _577 : 1065353216u;
        uint3 _590 = uint3(_444 + int3(0, 1, 0));
        uint _593 = _590.z;
        uint _595 = _593 + (_593 << 10u);
        uint _597 = _595 ^ (_595 >> 6u);
        uint _599 = _597 + (_597 << 3u);
        uint _601 = _599 ^ (_599 >> 11u);
        bool _604 = _549 ? _549 : true;
        uint _605 = _549 ? _550 : 1664525u;
        uint _606 = _549 ? _551 : 1013904223u;
        uint _610 = (((_590.y * _605) + (_601 + (_601 << 15u))) + _606) * _605;
        uint _612 = _610 ^ (_610 >> 11u);
        uint _615 = _612 ^ ((_612 << 7u) & 2636928640u);
        uint _618 = _615 ^ ((_615 << 15u) & 4022730752u);
        bool _621 = _604 ? _604 : true;
        uint _622 = _604 ? _605 : 1664525u;
        uint _623 = _604 ? _606 : 1013904223u;
        uint _627 = (((_590.x * _622) + (_618 ^ (_618 >> 18u))) + _623) * _622;
        uint _629 = _627 ^ (_627 >> 11u);
        uint _632 = _629 ^ ((_629 << 7u) & 2636928640u);
        uint _635 = _632 ^ ((_632 << 15u) & 4022730752u);
        uint _637 = _635 ^ (_635 >> 18u);
        uint _639 = _637 + (_637 << 10u);
        uint _641 = _639 ^ (_639 >> 6u);
        uint _643 = _641 + (_641 << 3u);
        uint _645 = _643 ^ (_643 >> 11u);
        uint _648 = _549 ? _582 : 8388607u;
        uint _649 = _549 ? _583 : 1065353216u;
        uint _654 = _604 ? _648 : 8388607u;
        uint _655 = _604 ? _649 : 1065353216u;
        uint3 _662 = uint3(_444 + int3(1, 1, 0));
        uint _665 = _662.z;
        uint _667 = _665 + (_665 << 10u);
        uint _669 = _667 ^ (_667 >> 6u);
        uint _671 = _669 + (_669 << 3u);
        uint _673 = _671 ^ (_671 >> 11u);
        bool _676 = _621 ? _621 : true;
        uint _677 = _621 ? _622 : 1664525u;
        uint _678 = _621 ? _623 : 1013904223u;
        uint _682 = (((_662.y * _677) + (_673 + (_673 << 15u))) + _678) * _677;
        uint _684 = _682 ^ (_682 >> 11u);
        uint _687 = _684 ^ ((_684 << 7u) & 2636928640u);
        uint _690 = _687 ^ ((_687 << 15u) & 4022730752u);
        bool _693 = _676 ? _676 : true;
        uint _694 = _676 ? _677 : 1664525u;
        uint _695 = _676 ? _678 : 1013904223u;
        uint _699 = (((_662.x * _694) + (_690 ^ (_690 >> 18u))) + _695) * _694;
        uint _701 = _699 ^ (_699 >> 11u);
        uint _704 = _701 ^ ((_701 << 7u) & 2636928640u);
        uint _707 = _704 ^ ((_704 << 15u) & 4022730752u);
        uint _709 = _707 ^ (_707 >> 18u);
        uint _711 = _709 + (_709 << 10u);
        uint _713 = _711 ^ (_711 >> 6u);
        uint _715 = _713 + (_713 << 3u);
        uint _717 = _715 ^ (_715 >> 11u);
        uint _720 = _621 ? _654 : 8388607u;
        uint _721 = _621 ? _655 : 1065353216u;
        uint _726 = _676 ? _720 : 8388607u;
        uint _727 = _676 ? _721 : 1065353216u;
        uint3 _734 = uint3(_444 + int3(0, 0, 1));
        uint _737 = _734.z;
        uint _739 = _737 + (_737 << 10u);
        uint _741 = _739 ^ (_739 >> 6u);
        uint _743 = _741 + (_741 << 3u);
        uint _745 = _743 ^ (_743 >> 11u);
        bool _748 = _693 ? _693 : true;
        uint _749 = _693 ? _694 : 1664525u;
        uint _750 = _693 ? _695 : 1013904223u;
        uint _754 = (((_734.y * _749) + (_745 + (_745 << 15u))) + _750) * _749;
        uint _756 = _754 ^ (_754 >> 11u);
        uint _759 = _756 ^ ((_756 << 7u) & 2636928640u);
        uint _762 = _759 ^ ((_759 << 15u) & 4022730752u);
        bool _765 = _748 ? _748 : true;
        uint _766 = _748 ? _749 : 1664525u;
        uint _767 = _748 ? _750 : 1013904223u;
        uint _771 = (((_734.x * _766) + (_762 ^ (_762 >> 18u))) + _767) * _766;
        uint _773 = _771 ^ (_771 >> 11u);
        uint _776 = _773 ^ ((_773 << 7u) & 2636928640u);
        uint _779 = _776 ^ ((_776 << 15u) & 4022730752u);
        uint _781 = _779 ^ (_779 >> 18u);
        uint _783 = _781 + (_781 << 10u);
        uint _785 = _783 ^ (_783 >> 6u);
        uint _787 = _785 + (_785 << 3u);
        uint _789 = _787 ^ (_787 >> 11u);
        uint _792 = _693 ? _726 : 8388607u;
        uint _793 = _693 ? _727 : 1065353216u;
        uint _798 = _748 ? _792 : 8388607u;
        uint _799 = _748 ? _793 : 1065353216u;
        uint3 _806 = uint3(_444 + int3(1, 0, 1));
        uint _809 = _806.z;
        uint _811 = _809 + (_809 << 10u);
        uint _813 = _811 ^ (_811 >> 6u);
        uint _815 = _813 + (_813 << 3u);
        uint _817 = _815 ^ (_815 >> 11u);
        bool _820 = _765 ? _765 : true;
        uint _821 = _765 ? _766 : 1664525u;
        uint _822 = _765 ? _767 : 1013904223u;
        uint _826 = (((_806.y * _821) + (_817 + (_817 << 15u))) + _822) * _821;
        uint _828 = _826 ^ (_826 >> 11u);
        uint _831 = _828 ^ ((_828 << 7u) & 2636928640u);
        uint _834 = _831 ^ ((_831 << 15u) & 4022730752u);
        bool _837 = _820 ? _820 : true;
        uint _838 = _820 ? _821 : 1664525u;
        uint _839 = _820 ? _822 : 1013904223u;
        uint _843 = (((_806.x * _838) + (_834 ^ (_834 >> 18u))) + _839) * _838;
        uint _845 = _843 ^ (_843 >> 11u);
        uint _848 = _845 ^ ((_845 << 7u) & 2636928640u);
        uint _851 = _848 ^ ((_848 << 15u) & 4022730752u);
        uint _853 = _851 ^ (_851 >> 18u);
        uint _855 = _853 + (_853 << 10u);
        uint _857 = _855 ^ (_855 >> 6u);
        uint _859 = _857 + (_857 << 3u);
        uint _861 = _859 ^ (_859 >> 11u);
        uint _864 = _765 ? _798 : 8388607u;
        uint _865 = _765 ? _799 : 1065353216u;
        uint _870 = _820 ? _864 : 8388607u;
        uint _871 = _820 ? _865 : 1065353216u;
        uint3 _878 = uint3(_444 + int3(0, 1, 1));
        uint _881 = _878.z;
        uint _883 = _881 + (_881 << 10u);
        uint _885 = _883 ^ (_883 >> 6u);
        uint _887 = _885 + (_885 << 3u);
        uint _889 = _887 ^ (_887 >> 11u);
        bool _892 = _837 ? _837 : true;
        uint _893 = _837 ? _838 : 1664525u;
        uint _894 = _837 ? _839 : 1013904223u;
        uint _898 = (((_878.y * _893) + (_889 + (_889 << 15u))) + _894) * _893;
        uint _900 = _898 ^ (_898 >> 11u);
        uint _903 = _900 ^ ((_900 << 7u) & 2636928640u);
        uint _906 = _903 ^ ((_903 << 15u) & 4022730752u);
        bool _909 = _892 ? _892 : true;
        uint _910 = _892 ? _893 : 1664525u;
        uint _911 = _892 ? _894 : 1013904223u;
        uint _915 = (((_878.x * _910) + (_906 ^ (_906 >> 18u))) + _911) * _910;
        uint _917 = _915 ^ (_915 >> 11u);
        uint _920 = _917 ^ ((_917 << 7u) & 2636928640u);
        uint _923 = _920 ^ ((_920 << 15u) & 4022730752u);
        uint _925 = _923 ^ (_923 >> 18u);
        uint _927 = _925 + (_925 << 10u);
        uint _929 = _927 ^ (_927 >> 6u);
        uint _931 = _929 + (_929 << 3u);
        uint _933 = _931 ^ (_931 >> 11u);
        uint _936 = _837 ? _870 : 8388607u;
        uint _937 = _837 ? _871 : 1065353216u;
        uint _942 = _892 ? _936 : 8388607u;
        uint _943 = _892 ? _937 : 1065353216u;
        uint3 _950 = uint3(_444 + int3(1, 1, 1));
        uint _953 = _950.z;
        uint _955 = _953 + (_953 << 10u);
        uint _957 = _955 ^ (_955 >> 6u);
        uint _959 = _957 + (_957 << 3u);
        uint _961 = _959 ^ (_959 >> 11u);
        bool _964 = _909 ? _909 : true;
        uint _965 = _909 ? _910 : 1664525u;
        uint _966 = _909 ? _911 : 1013904223u;
        uint _970 = (((_950.y * _965) + (_961 + (_961 << 15u))) + _966) * _965;
        uint _972 = _970 ^ (_970 >> 11u);
        uint _975 = _972 ^ ((_972 << 7u) & 2636928640u);
        uint _978 = _975 ^ ((_975 << 15u) & 4022730752u);
        uint _981 = _964 ? _965 : 1664525u;
        uint _986 = (((_950.x * _981) + (_978 ^ (_978 >> 18u))) + (_964 ? _966 : 1013904223u)) * _981;
        uint _988 = _986 ^ (_986 >> 11u);
        uint _991 = _988 ^ ((_988 << 7u) & 2636928640u);
        uint _994 = _991 ^ ((_991 << 15u) & 4022730752u);
        uint _996 = _994 ^ (_994 >> 18u);
        uint _998 = _996 + (_996 << 10u);
        uint _1000 = _998 ^ (_998 >> 6u);
        uint _1002 = _1000 + (_1000 << 3u);
        uint _1004 = _1002 ^ (_1002 >> 11u);
        uint _1007 = _909 ? _942 : 8388607u;
        uint _1008 = _909 ? _943 : 1065353216u;
        float3 _1023 = (_445 * _445) * (3.0f.xxx - (_445 * 2.0f));
        float _1024 = _1023.x;
        float _1025 = 1.0f - _1024;
        float _1027 = _1023.y;
        float _1028 = 1.0f - _1027;
        float _1030 = _1023.z;
        float _1031 = 1.0f - _1030;
        float2 _1044 = (((((float2(asfloat((_493 & _504) | _505) - 1.0f, asfloat(((_501 + (_501 << 15u)) & _510) | _511) - 1.0f) * _1025) * _1028) * _1031) + (((float2(asfloat((_565 & _576) | _577) - 1.0f, asfloat(((_573 + (_573 << 15u)) & _582) | _583) - 1.0f) * _1024) * _1028) * _1031)) + (((float2(asfloat((_637 & _648) | _649) - 1.0f, asfloat(((_645 + (_645 << 15u)) & _654) | _655) - 1.0f) * _1025) * _1027) * _1031)) + (((float2(asfloat((_709 & _720) | _721) - 1.0f, asfloat(((_717 + (_717 << 15u)) & _726) | _727) - 1.0f) * _1024) * _1027) * _1031);
        float2 _1060 = (((_1044 + (((float2(asfloat((_781 & _792) | _793) - 1.0f, asfloat(((_789 + (_789 << 15u)) & _798) | _799) - 1.0f) * _1025) * _1028) * _1030)) + (((float2(asfloat((_853 & _864) | _865) - 1.0f, asfloat(((_861 + (_861 << 15u)) & _870) | _871) - 1.0f) * _1024) * _1028) * _1030)) + (((float2(asfloat((_925 & _936) | _937) - 1.0f, asfloat(((_933 + (_933 << 15u)) & _942) | _943) - 1.0f) * _1025) * _1027) * _1030)) + (((float2(asfloat((_996 & _1007) | _1008) - 1.0f, asfloat(((_1004 + (_1004 << 15u)) & (_964 ? _1007 : 8388607u)) | (_964 ? _1008 : 1065353216u)) - 1.0f) * _1024) * _1027) * _1030);
        float2 _1063 = clamp((_1060 - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
        float4 _1075 = mul(model_to_world, float4(_441 + (float3(_1063.x, 0.0f, _1063.y) * 0.0599999986588954925537109375f), 1.0f));
        float _1088 = _386 * 0.5f;
        _1095 = mul(frame_constants_view_constants[current_view_index].world_to_sample, _1075);
        _1096 = mul(_435, float4(normalize(float3(in_var_Vertex_Position.x, 0.0f, in_var_Vertex_Position.z)), 0.0f)).xyz;
        _1097 = float2(_159.x + _1088, _159.z + _1088) * 0.20000000298023223876953125f;
        _1098 = _1075.xyz;
        break;
    } while(false);
    gl_Position = _1095;
    out_var_C1 = _1096;
    out_var_C2 = _1097;
    out_var_C3 = _1098;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
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
