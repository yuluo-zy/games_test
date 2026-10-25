#define SHADER_VARIANT_DEPTH_ONLY 1
#define TGV_f0adff76827e73ff 1
#line 1 "shaders/ivy/instanced_ivy_leaf.raster.hlsl"


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
#line 30 "shaders/ivy/instanced_ivy_leaf.raster.hlsl"

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
#line 31 "shaders/ivy/instanced_ivy_leaf.raster.hlsl"

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
#line 32 "shaders/ivy/instanced_ivy_leaf.raster.hlsl"





int is_flower;
float4x4 model_to_world;
uint2 ivy_leaf_color_dims;
uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
uint2 path_mask_dims;
int bool_render_shadows;

struct IvyLeaf
{
    float3 position;
    float size;
    float3 forward_floattor;
    int seed;
    float3 branch_dir;
    float arch_height;
};

StructuredBuffer<IvyLeaf> ivy_leaf_buffer : register(t16, space0);
Texture2D<float4> terrain_heightmap : register(t17, space0);
Texture2D<float4> path_mask : register(t19, space0);
Texture2D<float4> ivy_leaf_color : register(t15, space0);

struct VSInput
{
    [[vk::location(0)]] float3 position: Vertex_Position;
    [[vk::location(1)]] float3 normal: Vertex_Normal;
    [[vk::location(2)]] float2 uv: Vertex_UV;
    uint instance : SV_InstanceID;
};

struct VSOutput
{
    float4 position : SV_Position;
    float3 world_pos : TEXCOORD1;
    float3 normal : TEXCOORD2;
    float2 uv : TEXCOORD4;
};

VSOutput vs_main(VSInput input)
{
    VSOutput output;
    float4 position;
    float3 world_pos;
    float3 normal;
    float2 uv;
    switch (0u)
    {
    default:
        float3 pos = ivy_leaf_buffer[input.instance].position;
    float h = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default,
                 pos.xz * (0.0076923076994717121124267578125f).xx + 0.5f.xx, 0.0f).x);
    float arch_height = ivy_leaf_buffer[input.instance].arch_height;
    float size = ivy_leaf_buffer[input.instance].size;
    float base = arch_height - min(h, 0.0f);
    float3 vp = (input.position * 0.75f) * size;
    float3 scaled = vp;
    if (glade_theme_id == 4u)
    {
        scaled = vp * 0.85f;
    }
    bool flower = (is_flower == 1);

    float3 bdir = ivy_leaf_buffer[input.instance].branch_dir;
    float3 vx, vy, vz, voff;
    if (flower)
    {
        float3 fwd = ivy_leaf_buffer[input.instance].forward_floattor;
        float3 up_cross = cross(float3(0.0f, 1.0f, 0.0f), fwd);
        vz = up_cross;
        vy = fwd;
        vx = cross(up_cross, fwd);
        voff = scaled + (fwd * 0.1f);
    }
    else
    {
        float3 fwd = ivy_leaf_buffer[input.instance].forward_floattor;
        float3 w = normalize(cross(fwd, normalize(lerp(bdir, float3(0.0f, 1.0f, 0.0f),
                       lerp(0.5f, 1.0f, abs(bdir.y)).xxx))));
        vz = fwd;
        vy = normalize(cross(w, fwd));
        vx = w;
        voff = scaled;
    }
    float4x4 rot = transpose(float4x4(float4(vx, 0.0f), float4(vy, 0.0f), float4(vz, 0.0f),
                                      float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float3 offset_pos = mul(rot, float4(voff, 1.0f)).xyz;
    float3 anchor = mul(rot, float4(0.0f, 0.0f, 0.0f, 1.0f)).xyz + pos;
    float path = path_mask.SampleLevel(sampler_llb,
                 anchor.xz * (0.0076923076994717121124267578125f).xx + 0.5f.xx, 0.0f).x;
    bool over_path = path > 0.1f;
    if (over_path)
    {
        float y = anchor.y;
        float n = perlin(anchor.xz * 0.6f + y * 0.6f);
        float t = clamp(base - y, 0.0f, 1.0f);
        float d = (1.0f - exp(-5.0f * path)) * base * (t * t * clamp(smoothstep(0.0f, 0.3f, n), 0.8f, 1.0f));
        if (y < d)
        {
            position = asfloat(0x7fbfffffu).xxxx;
            world_pos = 0.0f.xxx;
            normal = 0.0f.xxx;
            uv = 0.0f.xx;
            break;
        }
    }
    float3 lifted = pos;
    lifted.y = pos.y + h;
    bool at_base = flower ? (pos.y < 0.9f) : false;
    float3 world = (at_base ? float3(0.0f, 0.0f, 0.0f) : offset_pos) + lifted;
    uint alt_seed;
    float2 disp = clamp((perlin3(world * 0.4f, over_path, alt_seed) - 0.5f) * 2.5f, -0.5f, 0.5f);
    world += float3(disp.x, 0.0f, disp.y) * 0.06f;

    uint jh1 = uint(ivy_leaf_buffer[input.instance].seed) ^ 2342304u;
    jh1 += (jh1 << 10u);
    jh1 ^= (jh1 >> 6u);
    jh1 += (jh1 << 3u);
    jh1 ^= (jh1 >> 11u);
    jh1 += (jh1 << 15u);
    uint jh2 = uint(ivy_leaf_buffer[input.instance].seed) ^ 9344565u;
    jh2 += (jh2 << 10u);
    jh2 ^= (jh2 >> 6u);
    jh2 += (jh2 << 3u);
    jh2 ^= (jh2 >> 11u);
    jh2 += (jh2 << 15u);
    float2 jitter = float2(asfloat((jh1 & 0x7FFFFFu) | 0x3F800000u) - 1.0f,
                           asfloat((jh2 & 0x7FFFFFu) | 0x3F800000u) - 1.0f);

    position = mul(view_constants[current_view_index].world_to_sample, float4(world, 1.0f));
    world_pos = world;
    normal = mul(rot, float4(input.normal, 0.0f)).xyz;
    uv = input.uv * (flower ? 0.3f : 0.1f) + jitter;
    break;
    }
    output.position = position;
    output.world_pos = world_pos;
    output.normal = normal;
    output.uv = uv;
    return output;
}

struct PSInput
{
    float3 normal : TEXCOORD2;
    float2 uv : TEXCOORD4;
    float4 position : SV_Position;
};

uint2 ps_main(PSInput input) : SV_Target0
{
    if (!(bool_render_shadows == 1))
    {
        uint2 checker = uint2(input.position.xy) & uint2(1u, 1u);
        if ((frame_index & 3u) == (checker.x + checker.y * 2u))
        {
            discard;
        }
    }
    float3 packed_normal = input.normal * 0.5f + 0.5f.xxx;
    uint n = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
           + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
           + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
           + 4278190080u;
    float3 sc = sqrt(clamp(ivy_leaf_color.Sample(sampler_llr, input.uv).xyz * 1.1f, 0.0f.xxx, 1.0f.xxx));
    uint m = uint(clamp(sc.x, 0.0f, 1.0f) * 255.0f + 0.5f)
           + (uint(clamp(sc.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
           + (uint(clamp(sc.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
           + 1291845632u;
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

struct IvyLeaf
{
    float3 position;
    float size;
    float3 forward_floattor;
    int seed;
    float3 branch_dir;
    float arch_height;
};

static uint _140;

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
    int is_flower : packoffset(c0.y);
    column_major float4x4 model_to_world : packoffset(c1);
    uint2 ivy_leaf_color_dims : packoffset(c5);
    uint2 terrain_heightmap_dims : packoffset(c5.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c6);
    uint2 path_mask_dims : packoffset(c6.z);
    int bool_render_shadows : packoffset(c7);
};

ByteAddressBuffer ivy_leaf_buffer : register(t16, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t17, space0);
Texture2D<float4> path_mask : register(t19, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C4;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float2 in_var_Vertex_UV: Vertex_UV;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float4 _1131;
    float3 _1132;
    float3 _1133;
    float2 _1134;
    do
    {
        float3 _168 = ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 0);
        float _177 = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_168.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
        float _181 = ivy_leaf_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 44) - min(_177, 0.0f);
        float3 _185 = (in_var_Vertex_Position * 0.75f) * ivy_leaf_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 12);
        float3 _192;
        if (frame_constants_glade_theme_id == 4u)
        {
            _192 = _185 * 0.85000002384185791015625f;
        }
        else
        {
            _192 = _185;
        }
        bool _195 = is_flower == 1;
        float3 _220;
        float3 _221;
        float3 _222;
        float3 _223;
        if (_195)
        {
            float3 _216 = cross(float3(0.0f, 1.0f, 0.0f), ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16));
            _220 = _216;
            _221 = ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16);
            _222 = cross(_216, ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16));
            _223 = _192 + (ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16) * 0.100000001490116119384765625f);
        }
        else
        {
            float3 _211 = normalize(cross(ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16), normalize(lerp(ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 32), float3(0.0f, 1.0f, 0.0f), lerp(0.5f, 1.0f, abs(ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 32).y)).xxx))));
            _220 = ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16);
            _221 = normalize(cross(_211, ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16)));
            _222 = _211;
            _223 = _192;
        }
        float4x4 _237 = transpose(float4x4(float4(_222, 0.0f), float4(_221, 0.0f), float4(_220, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
        float3 _243 = mul(_237, float4(_223, 1.0f)).xyz;
        float3 _246 = mul(_237, float4(0.0f, 0.0f, 0.0f, 1.0f)).xyz + _168;
        float4 _253 = path_mask.SampleLevel(sampler_llb, (_246.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
        float _254 = _253.x;
        bool _255 = _254 > 0.100000001490116119384765625f;
        if (_255)
        {
            float _266 = _246.y;
            float2 _269 = (float2(_246.xz) * 0.60000002384185791015625f) + (_266 * 0.60000002384185791015625f).xx;
            int2 _271 = int2(floor(_269));
            float2 _272 = frac(_269);
            uint2 _273 = uint2(_271);
            uint _275 = _273.y;
            uint _277 = _275 + (_275 << 10u);
            uint _279 = _277 ^ (_277 >> 6u);
            uint _281 = _279 + (_279 << 3u);
            uint _283 = _281 ^ (_281 >> 11u);
            uint _289 = (((_273.x * 1664525u) + (_283 + (_283 << 15u))) + 1013904223u) * 1664525u;
            uint _291 = _289 ^ (_289 >> 11u);
            uint _294 = _291 ^ ((_291 << 7u) & 2636928640u);
            uint _297 = _294 ^ ((_294 << 15u) & 4022730752u);
            uint2 _305 = uint2(_271 + int2(1, 0));
            uint _307 = _305.y;
            uint _309 = _307 + (_307 << 10u);
            uint _311 = _309 ^ (_309 >> 6u);
            uint _313 = _311 + (_311 << 3u);
            uint _315 = _313 ^ (_313 >> 11u);
            uint _321 = (((_305.x * 1664525u) + (_315 + (_315 << 15u))) + 1013904223u) * 1664525u;
            uint _323 = _321 ^ (_321 >> 11u);
            uint _326 = _323 ^ ((_323 << 7u) & 2636928640u);
            uint _329 = _326 ^ ((_326 << 15u) & 4022730752u);
            uint2 _337 = uint2(_271 + int2(0, 1));
            uint _339 = _337.y;
            uint _341 = _339 + (_339 << 10u);
            uint _343 = _341 ^ (_341 >> 6u);
            uint _345 = _343 + (_343 << 3u);
            uint _347 = _345 ^ (_345 >> 11u);
            uint _353 = (((_337.x * 1664525u) + (_347 + (_347 << 15u))) + 1013904223u) * 1664525u;
            uint _355 = _353 ^ (_353 >> 11u);
            uint _358 = _355 ^ ((_355 << 7u) & 2636928640u);
            uint _361 = _358 ^ ((_358 << 15u) & 4022730752u);
            uint2 _369 = uint2(_271 + int2(1, 1));
            uint _371 = _369.y;
            uint _373 = _371 + (_371 << 10u);
            uint _375 = _373 ^ (_373 >> 6u);
            uint _377 = _375 + (_375 << 3u);
            uint _379 = _377 ^ (_377 >> 11u);
            uint _385 = (((_369.x * 1664525u) + (_379 + (_379 << 15u))) + 1013904223u) * 1664525u;
            uint _387 = _385 ^ (_385 >> 11u);
            uint _390 = _387 ^ ((_387 << 7u) & 2636928640u);
            uint _393 = _390 ^ ((_390 << 15u) & 4022730752u);
            float2 _403 = (_272 * _272) * (3.0f.xx - (_272 * 2.0f));
            float _404 = _403.x;
            float _405 = 1.0f - _404;
            float _407 = _403.y;
            float _421 = clamp(_181 - _266, 0.0f, 1.0f);
            if (_266 < (((1.0f - exp((-5.0f) * _254)) * _181) * ((_421 * _421) * clamp(smoothstep(0.0f, 0.300000011920928955078125f, (((1.0f - _407) * (((asfloat(((_297 ^ (_297 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _405) + ((asfloat(((_329 ^ (_329 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _404))) + (((asfloat(((_361 ^ (_361 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _405) * _407)) + (((asfloat(((_393 ^ (_393 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _404) * _407)), 0.800000011920928955078125f, 1.0f))))
            {
                _1131 = asfloat(0x7fbfffffu          ).xxxx;
                _1132 = 0.0f.xxx;
                _1133 = 0.0f.xxx;
                _1134 = 0.0f.xx;
                break;
            }
        }
        bool _430 = _255 ? true : false;
        bool _437;
        if (_195)
        {
            _437 = _168.y < 0.89999997615814208984375f;
        }
        else
        {
            _437 = false;
        }
        bool3 _438 = _437.xxx;
        float3 _442 = _168;
        _442.y = _168.y + _177;
        float3 _443 = float3(_438.x ? 0.0f.xxx.x : _243.x, _438.y ? 0.0f.xxx.y : _243.y, _438.z ? 0.0f.xxx.z : _243.z) + _442;
        float3 _444 = _443 * 0.4000000059604644775390625f;
        int3 _446 = int3(floor(_444));
        float3 _447 = frac(_444);
        uint3 _448 = uint3(_446);
        uint _451 = _448.z;
        uint _453 = _451 + (_451 << 10u);
        uint _455 = _453 ^ (_453 >> 6u);
        uint _457 = _455 + (_455 << 3u);
        uint _459 = _457 ^ (_457 >> 11u);
        bool _462 = _430 ? _430 : true;
        uint _463 = _430 ? (_255 ? 1664525u : _140) : 1664525u;
        uint _464 = _430 ? (_255 ? 1013904223u : _140) : 1013904223u;
        uint _468 = (((_448.y * _463) + (_459 + (_459 << 15u))) + _464) * _463;
        uint _470 = _468 ^ (_468 >> 11u);
        uint _473 = _470 ^ ((_470 << 7u) & 2636928640u);
        uint _476 = _473 ^ ((_473 << 15u) & 4022730752u);
        bool _479 = _462 ? _462 : true;
        uint _480 = _462 ? _463 : 1664525u;
        uint _481 = _462 ? _464 : 1013904223u;
        uint _485 = (((_448.x * _480) + (_476 ^ (_476 >> 18u))) + _481) * _480;
        uint _487 = _485 ^ (_485 >> 11u);
        uint _490 = _487 ^ ((_487 << 7u) & 2636928640u);
        uint _493 = _490 ^ ((_490 << 15u) & 4022730752u);
        uint _495 = _493 ^ (_493 >> 18u);
        uint _497 = _495 + (_495 << 10u);
        uint _499 = _497 ^ (_497 >> 6u);
        uint _501 = _499 + (_499 << 3u);
        uint _503 = _501 ^ (_501 >> 11u);
        uint _506 = _430 ? (_255 ? 8388607u : _140) : 8388607u;
        uint _507 = _430 ? (_255 ? 1065353216u : _140) : 1065353216u;
        uint _512 = _462 ? _506 : 8388607u;
        uint _513 = _462 ? _507 : 1065353216u;
        uint3 _520 = uint3(_446 + int3(1, 0, 0));
        uint _523 = _520.z;
        uint _525 = _523 + (_523 << 10u);
        uint _527 = _525 ^ (_525 >> 6u);
        uint _529 = _527 + (_527 << 3u);
        uint _531 = _529 ^ (_529 >> 11u);
        bool _534 = _479 ? _479 : true;
        uint _535 = _479 ? _480 : 1664525u;
        uint _536 = _479 ? _481 : 1013904223u;
        uint _540 = (((_520.y * _535) + (_531 + (_531 << 15u))) + _536) * _535;
        uint _542 = _540 ^ (_540 >> 11u);
        uint _545 = _542 ^ ((_542 << 7u) & 2636928640u);
        uint _548 = _545 ^ ((_545 << 15u) & 4022730752u);
        bool _551 = _534 ? _534 : true;
        uint _552 = _534 ? _535 : 1664525u;
        uint _553 = _534 ? _536 : 1013904223u;
        uint _557 = (((_520.x * _552) + (_548 ^ (_548 >> 18u))) + _553) * _552;
        uint _559 = _557 ^ (_557 >> 11u);
        uint _562 = _559 ^ ((_559 << 7u) & 2636928640u);
        uint _565 = _562 ^ ((_562 << 15u) & 4022730752u);
        uint _567 = _565 ^ (_565 >> 18u);
        uint _569 = _567 + (_567 << 10u);
        uint _571 = _569 ^ (_569 >> 6u);
        uint _573 = _571 + (_571 << 3u);
        uint _575 = _573 ^ (_573 >> 11u);
        uint _578 = _479 ? _512 : 8388607u;
        uint _579 = _479 ? _513 : 1065353216u;
        uint _584 = _534 ? _578 : 8388607u;
        uint _585 = _534 ? _579 : 1065353216u;
        uint3 _592 = uint3(_446 + int3(0, 1, 0));
        uint _595 = _592.z;
        uint _597 = _595 + (_595 << 10u);
        uint _599 = _597 ^ (_597 >> 6u);
        uint _601 = _599 + (_599 << 3u);
        uint _603 = _601 ^ (_601 >> 11u);
        bool _606 = _551 ? _551 : true;
        uint _607 = _551 ? _552 : 1664525u;
        uint _608 = _551 ? _553 : 1013904223u;
        uint _612 = (((_592.y * _607) + (_603 + (_603 << 15u))) + _608) * _607;
        uint _614 = _612 ^ (_612 >> 11u);
        uint _617 = _614 ^ ((_614 << 7u) & 2636928640u);
        uint _620 = _617 ^ ((_617 << 15u) & 4022730752u);
        bool _623 = _606 ? _606 : true;
        uint _624 = _606 ? _607 : 1664525u;
        uint _625 = _606 ? _608 : 1013904223u;
        uint _629 = (((_592.x * _624) + (_620 ^ (_620 >> 18u))) + _625) * _624;
        uint _631 = _629 ^ (_629 >> 11u);
        uint _634 = _631 ^ ((_631 << 7u) & 2636928640u);
        uint _637 = _634 ^ ((_634 << 15u) & 4022730752u);
        uint _639 = _637 ^ (_637 >> 18u);
        uint _641 = _639 + (_639 << 10u);
        uint _643 = _641 ^ (_641 >> 6u);
        uint _645 = _643 + (_643 << 3u);
        uint _647 = _645 ^ (_645 >> 11u);
        uint _650 = _551 ? _584 : 8388607u;
        uint _651 = _551 ? _585 : 1065353216u;
        uint _656 = _606 ? _650 : 8388607u;
        uint _657 = _606 ? _651 : 1065353216u;
        uint3 _664 = uint3(_446 + int3(1, 1, 0));
        uint _667 = _664.z;
        uint _669 = _667 + (_667 << 10u);
        uint _671 = _669 ^ (_669 >> 6u);
        uint _673 = _671 + (_671 << 3u);
        uint _675 = _673 ^ (_673 >> 11u);
        bool _678 = _623 ? _623 : true;
        uint _679 = _623 ? _624 : 1664525u;
        uint _680 = _623 ? _625 : 1013904223u;
        uint _684 = (((_664.y * _679) + (_675 + (_675 << 15u))) + _680) * _679;
        uint _686 = _684 ^ (_684 >> 11u);
        uint _689 = _686 ^ ((_686 << 7u) & 2636928640u);
        uint _692 = _689 ^ ((_689 << 15u) & 4022730752u);
        bool _695 = _678 ? _678 : true;
        uint _696 = _678 ? _679 : 1664525u;
        uint _697 = _678 ? _680 : 1013904223u;
        uint _701 = (((_664.x * _696) + (_692 ^ (_692 >> 18u))) + _697) * _696;
        uint _703 = _701 ^ (_701 >> 11u);
        uint _706 = _703 ^ ((_703 << 7u) & 2636928640u);
        uint _709 = _706 ^ ((_706 << 15u) & 4022730752u);
        uint _711 = _709 ^ (_709 >> 18u);
        uint _713 = _711 + (_711 << 10u);
        uint _715 = _713 ^ (_713 >> 6u);
        uint _717 = _715 + (_715 << 3u);
        uint _719 = _717 ^ (_717 >> 11u);
        uint _722 = _623 ? _656 : 8388607u;
        uint _723 = _623 ? _657 : 1065353216u;
        uint _728 = _678 ? _722 : 8388607u;
        uint _729 = _678 ? _723 : 1065353216u;
        uint3 _736 = uint3(_446 + int3(0, 0, 1));
        uint _739 = _736.z;
        uint _741 = _739 + (_739 << 10u);
        uint _743 = _741 ^ (_741 >> 6u);
        uint _745 = _743 + (_743 << 3u);
        uint _747 = _745 ^ (_745 >> 11u);
        bool _750 = _695 ? _695 : true;
        uint _751 = _695 ? _696 : 1664525u;
        uint _752 = _695 ? _697 : 1013904223u;
        uint _756 = (((_736.y * _751) + (_747 + (_747 << 15u))) + _752) * _751;
        uint _758 = _756 ^ (_756 >> 11u);
        uint _761 = _758 ^ ((_758 << 7u) & 2636928640u);
        uint _764 = _761 ^ ((_761 << 15u) & 4022730752u);
        bool _767 = _750 ? _750 : true;
        uint _768 = _750 ? _751 : 1664525u;
        uint _769 = _750 ? _752 : 1013904223u;
        uint _773 = (((_736.x * _768) + (_764 ^ (_764 >> 18u))) + _769) * _768;
        uint _775 = _773 ^ (_773 >> 11u);
        uint _778 = _775 ^ ((_775 << 7u) & 2636928640u);
        uint _781 = _778 ^ ((_778 << 15u) & 4022730752u);
        uint _783 = _781 ^ (_781 >> 18u);
        uint _785 = _783 + (_783 << 10u);
        uint _787 = _785 ^ (_785 >> 6u);
        uint _789 = _787 + (_787 << 3u);
        uint _791 = _789 ^ (_789 >> 11u);
        uint _794 = _695 ? _728 : 8388607u;
        uint _795 = _695 ? _729 : 1065353216u;
        uint _800 = _750 ? _794 : 8388607u;
        uint _801 = _750 ? _795 : 1065353216u;
        uint3 _808 = uint3(_446 + int3(1, 0, 1));
        uint _811 = _808.z;
        uint _813 = _811 + (_811 << 10u);
        uint _815 = _813 ^ (_813 >> 6u);
        uint _817 = _815 + (_815 << 3u);
        uint _819 = _817 ^ (_817 >> 11u);
        bool _822 = _767 ? _767 : true;
        uint _823 = _767 ? _768 : 1664525u;
        uint _824 = _767 ? _769 : 1013904223u;
        uint _828 = (((_808.y * _823) + (_819 + (_819 << 15u))) + _824) * _823;
        uint _830 = _828 ^ (_828 >> 11u);
        uint _833 = _830 ^ ((_830 << 7u) & 2636928640u);
        uint _836 = _833 ^ ((_833 << 15u) & 4022730752u);
        bool _839 = _822 ? _822 : true;
        uint _840 = _822 ? _823 : 1664525u;
        uint _841 = _822 ? _824 : 1013904223u;
        uint _845 = (((_808.x * _840) + (_836 ^ (_836 >> 18u))) + _841) * _840;
        uint _847 = _845 ^ (_845 >> 11u);
        uint _850 = _847 ^ ((_847 << 7u) & 2636928640u);
        uint _853 = _850 ^ ((_850 << 15u) & 4022730752u);
        uint _855 = _853 ^ (_853 >> 18u);
        uint _857 = _855 + (_855 << 10u);
        uint _859 = _857 ^ (_857 >> 6u);
        uint _861 = _859 + (_859 << 3u);
        uint _863 = _861 ^ (_861 >> 11u);
        uint _866 = _767 ? _800 : 8388607u;
        uint _867 = _767 ? _801 : 1065353216u;
        uint _872 = _822 ? _866 : 8388607u;
        uint _873 = _822 ? _867 : 1065353216u;
        uint3 _880 = uint3(_446 + int3(0, 1, 1));
        uint _883 = _880.z;
        uint _885 = _883 + (_883 << 10u);
        uint _887 = _885 ^ (_885 >> 6u);
        uint _889 = _887 + (_887 << 3u);
        uint _891 = _889 ^ (_889 >> 11u);
        bool _894 = _839 ? _839 : true;
        uint _895 = _839 ? _840 : 1664525u;
        uint _896 = _839 ? _841 : 1013904223u;
        uint _900 = (((_880.y * _895) + (_891 + (_891 << 15u))) + _896) * _895;
        uint _902 = _900 ^ (_900 >> 11u);
        uint _905 = _902 ^ ((_902 << 7u) & 2636928640u);
        uint _908 = _905 ^ ((_905 << 15u) & 4022730752u);
        bool _911 = _894 ? _894 : true;
        uint _912 = _894 ? _895 : 1664525u;
        uint _913 = _894 ? _896 : 1013904223u;
        uint _917 = (((_880.x * _912) + (_908 ^ (_908 >> 18u))) + _913) * _912;
        uint _919 = _917 ^ (_917 >> 11u);
        uint _922 = _919 ^ ((_919 << 7u) & 2636928640u);
        uint _925 = _922 ^ ((_922 << 15u) & 4022730752u);
        uint _927 = _925 ^ (_925 >> 18u);
        uint _929 = _927 + (_927 << 10u);
        uint _931 = _929 ^ (_929 >> 6u);
        uint _933 = _931 + (_931 << 3u);
        uint _935 = _933 ^ (_933 >> 11u);
        uint _938 = _839 ? _872 : 8388607u;
        uint _939 = _839 ? _873 : 1065353216u;
        uint _944 = _894 ? _938 : 8388607u;
        uint _945 = _894 ? _939 : 1065353216u;
        uint3 _952 = uint3(_446 + int3(1, 1, 1));
        uint _955 = _952.z;
        uint _957 = _955 + (_955 << 10u);
        uint _959 = _957 ^ (_957 >> 6u);
        uint _961 = _959 + (_959 << 3u);
        uint _963 = _961 ^ (_961 >> 11u);
        bool _966 = _911 ? _911 : true;
        uint _967 = _911 ? _912 : 1664525u;
        uint _968 = _911 ? _913 : 1013904223u;
        uint _972 = (((_952.y * _967) + (_963 + (_963 << 15u))) + _968) * _967;
        uint _974 = _972 ^ (_972 >> 11u);
        uint _977 = _974 ^ ((_974 << 7u) & 2636928640u);
        uint _980 = _977 ^ ((_977 << 15u) & 4022730752u);
        uint _983 = _966 ? _967 : 1664525u;
        uint _988 = (((_952.x * _983) + (_980 ^ (_980 >> 18u))) + (_966 ? _968 : 1013904223u)) * _983;
        uint _990 = _988 ^ (_988 >> 11u);
        uint _993 = _990 ^ ((_990 << 7u) & 2636928640u);
        uint _996 = _993 ^ ((_993 << 15u) & 4022730752u);
        uint _998 = _996 ^ (_996 >> 18u);
        uint _1000 = _998 + (_998 << 10u);
        uint _1002 = _1000 ^ (_1000 >> 6u);
        uint _1004 = _1002 + (_1002 << 3u);
        uint _1006 = _1004 ^ (_1004 >> 11u);
        uint _1009 = _911 ? _944 : 8388607u;
        uint _1010 = _911 ? _945 : 1065353216u;
        bool _1015 = _966 ? _966 : true;
        uint _1016 = _966 ? _1009 : 8388607u;
        uint _1017 = _966 ? _1010 : 1065353216u;
        float3 _1026 = (_447 * _447) * (3.0f.xxx - (_447 * 2.0f));
        float _1027 = _1026.x;
        float _1028 = 1.0f - _1027;
        float _1030 = _1026.y;
        float _1031 = 1.0f - _1030;
        float _1033 = _1026.z;
        float _1034 = 1.0f - _1033;
        float2 _1047 = (((((float2(asfloat((_495 & _506) | _507) - 1.0f, asfloat(((_503 + (_503 << 15u)) & _512) | _513) - 1.0f) * _1028) * _1031) * _1034) + (((float2(asfloat((_567 & _578) | _579) - 1.0f, asfloat(((_575 + (_575 << 15u)) & _584) | _585) - 1.0f) * _1027) * _1031) * _1034)) + (((float2(asfloat((_639 & _650) | _651) - 1.0f, asfloat(((_647 + (_647 << 15u)) & _656) | _657) - 1.0f) * _1028) * _1030) * _1034)) + (((float2(asfloat((_711 & _722) | _723) - 1.0f, asfloat(((_719 + (_719 << 15u)) & _728) | _729) - 1.0f) * _1027) * _1030) * _1034);
        float2 _1063 = (((_1047 + (((float2(asfloat((_783 & _794) | _795) - 1.0f, asfloat(((_791 + (_791 << 15u)) & _800) | _801) - 1.0f) * _1028) * _1031) * _1033)) + (((float2(asfloat((_855 & _866) | _867) - 1.0f, asfloat(((_863 + (_863 << 15u)) & _872) | _873) - 1.0f) * _1027) * _1031) * _1033)) + (((float2(asfloat((_927 & _938) | _939) - 1.0f, asfloat(((_935 + (_935 << 15u)) & _944) | _945) - 1.0f) * _1028) * _1030) * _1033)) + (((float2(asfloat((_998 & _1009) | _1010) - 1.0f, asfloat(((_1006 + (_1006 << 15u)) & _1016) | _1017) - 1.0f) * _1027) * _1030) * _1033);
        float2 _1066 = clamp((_1063 - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
        float3 _1071 = _443 + (float3(_1066.x, 0.0f, _1066.y) * 0.0599999986588954925537109375f);
        uint _1092 = uint(ivy_leaf_buffer.Load<int>(uint(gl_InstanceIndex) * 48 + 28) ^ 2342304);
        uint _1094 = _1092 + (_1092 << 10u);
        uint _1096 = _1094 ^ (_1094 >> 6u);
        uint _1098 = _1096 + (_1096 << 3u);
        uint _1100 = _1098 ^ (_1098 >> 11u);
        bool _1103 = _1015 ? _1015 : true;
        uint _1104 = _1015 ? _1016 : 8388607u;
        uint _1105 = _1015 ? _1017 : 1065353216u;
        uint _1112 = uint(ivy_leaf_buffer.Load<int>(uint(gl_InstanceIndex) * 48 + 28) ^ 9344565);
        uint _1114 = _1112 + (_1112 << 10u);
        uint _1116 = _1114 ^ (_1114 >> 6u);
        uint _1118 = _1116 + (_1116 << 3u);
        uint _1120 = _1118 ^ (_1118 >> 11u);
        _1131 = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_1071, 1.0f));
        _1132 = _1071;
        _1133 = mul(_237, float4(in_var_Vertex_Normal, 0.0f)).xyz;
        _1134 = (in_var_Vertex_UV * (_195 ? 0.300000011920928955078125f : 0.100000001490116119384765625f)) + float2(asfloat(((_1100 + (_1100 << 15u)) & _1104) | _1105) - 1.0f, asfloat(((_1120 + (_1120 << 15u)) & (_1103 ? _1104 : 8388607u)) | (_1103 ? _1105 : 1065353216u)) - 1.0f);
        break;
    } while(false);
    gl_Position = _1131;
    out_var_C1 = _1132;
    out_var_C2 = _1133;
    out_var_C4 = _1134;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
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
    int is_flower : packoffset(c0.y);
    column_major float4x4 model_to_world : packoffset(c1);
    uint2 ivy_leaf_color_dims : packoffset(c5);
    uint2 terrain_heightmap_dims : packoffset(c5.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c6);
    uint2 path_mask_dims : packoffset(c6.z);
    int bool_render_shadows : packoffset(c7);
};

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> ivy_leaf_color : register(t15, space0);

static float4 gl_FragCoord;
static float3 in_var_C2;
static float2 in_var_C4;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(4)]] float2 in_var_C4 : TEXCOORD4;
    float4 gl_FragCoord : SV_Position;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    if (!((bool_render_shadows == 1) ? true : false))
    {
        uint2 _105 = uint2(gl_FragCoord.xy) & uint2(1u, 1u);
        if ((frame_constants_frame_index & 3u) == (_105.x + (_105.y * 2u)))
        {
            discard;
        }
    }
    float3 _123 = (in_var_C2 * 0.5f) + 0.5f.xxx;
    float3 _145 = sqrt(clamp(ivy_leaf_color.Sample(sampler_llr, in_var_C4).xyz * 1.10000002384185791015625f, 0.0f.xxx, 1.0f.xxx));
    out_var_SV_TARGET0 = uint2(((uint((clamp(_123.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_123.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_123.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u, ((uint((clamp(_145.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_145.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_145.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 1291845632u);
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C2 = stage_input.in_var_C2;
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

struct IvyLeaf
{
    float3 position;
    float size;
    float3 forward_floattor;
    int seed;
    float3 branch_dir;
    float arch_height;
};

static uint _140;

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
    int is_flower : packoffset(c0.y);
    column_major float4x4 model_to_world : packoffset(c1);
    uint2 ivy_leaf_color_dims : packoffset(c5);
    uint2 terrain_heightmap_dims : packoffset(c5.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c6);
    uint2 path_mask_dims : packoffset(c6.z);
    int bool_render_shadows : packoffset(c7);
};

ByteAddressBuffer ivy_leaf_buffer : register(t16, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t17, space0);
Texture2D<float4> path_mask : register(t19, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C4;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float2 in_var_Vertex_UV: Vertex_UV;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float4 _1131;
    float3 _1132;
    float3 _1133;
    float2 _1134;
    do
    {
        float3 _168 = ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 0);
        float _177 = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_168.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
        float _181 = ivy_leaf_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 44) - min(_177, 0.0f);
        float3 _185 = (in_var_Vertex_Position * 0.75f) * ivy_leaf_buffer.Load<float>(uint(gl_InstanceIndex) * 48 + 12);
        float3 _192;
        if (frame_constants_glade_theme_id == 4u)
        {
            _192 = _185 * 0.85000002384185791015625f;
        }
        else
        {
            _192 = _185;
        }
        bool _195 = is_flower == 1;
        float3 _220;
        float3 _221;
        float3 _222;
        float3 _223;
        if (_195)
        {
            float3 _216 = cross(float3(0.0f, 1.0f, 0.0f), ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16));
            _220 = _216;
            _221 = ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16);
            _222 = cross(_216, ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16));
            _223 = _192 + (ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16) * 0.100000001490116119384765625f);
        }
        else
        {
            float3 _211 = normalize(cross(ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16), normalize(lerp(ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 32), float3(0.0f, 1.0f, 0.0f), lerp(0.5f, 1.0f, abs(ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 32).y)).xxx))));
            _220 = ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16);
            _221 = normalize(cross(_211, ivy_leaf_buffer.Load<float3>(uint(gl_InstanceIndex) * 48 + 16)));
            _222 = _211;
            _223 = _192;
        }
        float4x4 _237 = transpose(float4x4(float4(_222, 0.0f), float4(_221, 0.0f), float4(_220, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
        float3 _243 = mul(_237, float4(_223, 1.0f)).xyz;
        float3 _246 = mul(_237, float4(0.0f, 0.0f, 0.0f, 1.0f)).xyz + _168;
        float4 _253 = path_mask.SampleLevel(sampler_llb, (_246.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
        float _254 = _253.x;
        bool _255 = _254 > 0.100000001490116119384765625f;
        if (_255)
        {
            float _266 = _246.y;
            float2 _269 = (float2(_246.xz) * 0.60000002384185791015625f) + (_266 * 0.60000002384185791015625f).xx;
            int2 _271 = int2(floor(_269));
            float2 _272 = frac(_269);
            uint2 _273 = uint2(_271);
            uint _275 = _273.y;
            uint _277 = _275 + (_275 << 10u);
            uint _279 = _277 ^ (_277 >> 6u);
            uint _281 = _279 + (_279 << 3u);
            uint _283 = _281 ^ (_281 >> 11u);
            uint _289 = (((_273.x * 1664525u) + (_283 + (_283 << 15u))) + 1013904223u) * 1664525u;
            uint _291 = _289 ^ (_289 >> 11u);
            uint _294 = _291 ^ ((_291 << 7u) & 2636928640u);
            uint _297 = _294 ^ ((_294 << 15u) & 4022730752u);
            uint2 _305 = uint2(_271 + int2(1, 0));
            uint _307 = _305.y;
            uint _309 = _307 + (_307 << 10u);
            uint _311 = _309 ^ (_309 >> 6u);
            uint _313 = _311 + (_311 << 3u);
            uint _315 = _313 ^ (_313 >> 11u);
            uint _321 = (((_305.x * 1664525u) + (_315 + (_315 << 15u))) + 1013904223u) * 1664525u;
            uint _323 = _321 ^ (_321 >> 11u);
            uint _326 = _323 ^ ((_323 << 7u) & 2636928640u);
            uint _329 = _326 ^ ((_326 << 15u) & 4022730752u);
            uint2 _337 = uint2(_271 + int2(0, 1));
            uint _339 = _337.y;
            uint _341 = _339 + (_339 << 10u);
            uint _343 = _341 ^ (_341 >> 6u);
            uint _345 = _343 + (_343 << 3u);
            uint _347 = _345 ^ (_345 >> 11u);
            uint _353 = (((_337.x * 1664525u) + (_347 + (_347 << 15u))) + 1013904223u) * 1664525u;
            uint _355 = _353 ^ (_353 >> 11u);
            uint _358 = _355 ^ ((_355 << 7u) & 2636928640u);
            uint _361 = _358 ^ ((_358 << 15u) & 4022730752u);
            uint2 _369 = uint2(_271 + int2(1, 1));
            uint _371 = _369.y;
            uint _373 = _371 + (_371 << 10u);
            uint _375 = _373 ^ (_373 >> 6u);
            uint _377 = _375 + (_375 << 3u);
            uint _379 = _377 ^ (_377 >> 11u);
            uint _385 = (((_369.x * 1664525u) + (_379 + (_379 << 15u))) + 1013904223u) * 1664525u;
            uint _387 = _385 ^ (_385 >> 11u);
            uint _390 = _387 ^ ((_387 << 7u) & 2636928640u);
            uint _393 = _390 ^ ((_390 << 15u) & 4022730752u);
            float2 _403 = (_272 * _272) * (3.0f.xx - (_272 * 2.0f));
            float _404 = _403.x;
            float _405 = 1.0f - _404;
            float _407 = _403.y;
            float _421 = clamp(_181 - _266, 0.0f, 1.0f);
            if (_266 < (((1.0f - exp((-5.0f) * _254)) * _181) * ((_421 * _421) * clamp(smoothstep(0.0f, 0.300000011920928955078125f, (((1.0f - _407) * (((asfloat(((_297 ^ (_297 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _405) + ((asfloat(((_329 ^ (_329 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _404))) + (((asfloat(((_361 ^ (_361 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _405) * _407)) + (((asfloat(((_393 ^ (_393 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _404) * _407)), 0.800000011920928955078125f, 1.0f))))
            {
                _1131 = asfloat(0x7fbfffffu          ).xxxx;
                _1132 = 0.0f.xxx;
                _1133 = 0.0f.xxx;
                _1134 = 0.0f.xx;
                break;
            }
        }
        bool _430 = _255 ? true : false;
        bool _437;
        if (_195)
        {
            _437 = _168.y < 0.89999997615814208984375f;
        }
        else
        {
            _437 = false;
        }
        bool3 _438 = _437.xxx;
        float3 _442 = _168;
        _442.y = _168.y + _177;
        float3 _443 = float3(_438.x ? 0.0f.xxx.x : _243.x, _438.y ? 0.0f.xxx.y : _243.y, _438.z ? 0.0f.xxx.z : _243.z) + _442;
        float3 _444 = _443 * 0.4000000059604644775390625f;
        int3 _446 = int3(floor(_444));
        float3 _447 = frac(_444);
        uint3 _448 = uint3(_446);
        uint _451 = _448.z;
        uint _453 = _451 + (_451 << 10u);
        uint _455 = _453 ^ (_453 >> 6u);
        uint _457 = _455 + (_455 << 3u);
        uint _459 = _457 ^ (_457 >> 11u);
        bool _462 = _430 ? _430 : true;
        uint _463 = _430 ? (_255 ? 1664525u : _140) : 1664525u;
        uint _464 = _430 ? (_255 ? 1013904223u : _140) : 1013904223u;
        uint _468 = (((_448.y * _463) + (_459 + (_459 << 15u))) + _464) * _463;
        uint _470 = _468 ^ (_468 >> 11u);
        uint _473 = _470 ^ ((_470 << 7u) & 2636928640u);
        uint _476 = _473 ^ ((_473 << 15u) & 4022730752u);
        bool _479 = _462 ? _462 : true;
        uint _480 = _462 ? _463 : 1664525u;
        uint _481 = _462 ? _464 : 1013904223u;
        uint _485 = (((_448.x * _480) + (_476 ^ (_476 >> 18u))) + _481) * _480;
        uint _487 = _485 ^ (_485 >> 11u);
        uint _490 = _487 ^ ((_487 << 7u) & 2636928640u);
        uint _493 = _490 ^ ((_490 << 15u) & 4022730752u);
        uint _495 = _493 ^ (_493 >> 18u);
        uint _497 = _495 + (_495 << 10u);
        uint _499 = _497 ^ (_497 >> 6u);
        uint _501 = _499 + (_499 << 3u);
        uint _503 = _501 ^ (_501 >> 11u);
        uint _506 = _430 ? (_255 ? 8388607u : _140) : 8388607u;
        uint _507 = _430 ? (_255 ? 1065353216u : _140) : 1065353216u;
        uint _512 = _462 ? _506 : 8388607u;
        uint _513 = _462 ? _507 : 1065353216u;
        uint3 _520 = uint3(_446 + int3(1, 0, 0));
        uint _523 = _520.z;
        uint _525 = _523 + (_523 << 10u);
        uint _527 = _525 ^ (_525 >> 6u);
        uint _529 = _527 + (_527 << 3u);
        uint _531 = _529 ^ (_529 >> 11u);
        bool _534 = _479 ? _479 : true;
        uint _535 = _479 ? _480 : 1664525u;
        uint _536 = _479 ? _481 : 1013904223u;
        uint _540 = (((_520.y * _535) + (_531 + (_531 << 15u))) + _536) * _535;
        uint _542 = _540 ^ (_540 >> 11u);
        uint _545 = _542 ^ ((_542 << 7u) & 2636928640u);
        uint _548 = _545 ^ ((_545 << 15u) & 4022730752u);
        bool _551 = _534 ? _534 : true;
        uint _552 = _534 ? _535 : 1664525u;
        uint _553 = _534 ? _536 : 1013904223u;
        uint _557 = (((_520.x * _552) + (_548 ^ (_548 >> 18u))) + _553) * _552;
        uint _559 = _557 ^ (_557 >> 11u);
        uint _562 = _559 ^ ((_559 << 7u) & 2636928640u);
        uint _565 = _562 ^ ((_562 << 15u) & 4022730752u);
        uint _567 = _565 ^ (_565 >> 18u);
        uint _569 = _567 + (_567 << 10u);
        uint _571 = _569 ^ (_569 >> 6u);
        uint _573 = _571 + (_571 << 3u);
        uint _575 = _573 ^ (_573 >> 11u);
        uint _578 = _479 ? _512 : 8388607u;
        uint _579 = _479 ? _513 : 1065353216u;
        uint _584 = _534 ? _578 : 8388607u;
        uint _585 = _534 ? _579 : 1065353216u;
        uint3 _592 = uint3(_446 + int3(0, 1, 0));
        uint _595 = _592.z;
        uint _597 = _595 + (_595 << 10u);
        uint _599 = _597 ^ (_597 >> 6u);
        uint _601 = _599 + (_599 << 3u);
        uint _603 = _601 ^ (_601 >> 11u);
        bool _606 = _551 ? _551 : true;
        uint _607 = _551 ? _552 : 1664525u;
        uint _608 = _551 ? _553 : 1013904223u;
        uint _612 = (((_592.y * _607) + (_603 + (_603 << 15u))) + _608) * _607;
        uint _614 = _612 ^ (_612 >> 11u);
        uint _617 = _614 ^ ((_614 << 7u) & 2636928640u);
        uint _620 = _617 ^ ((_617 << 15u) & 4022730752u);
        bool _623 = _606 ? _606 : true;
        uint _624 = _606 ? _607 : 1664525u;
        uint _625 = _606 ? _608 : 1013904223u;
        uint _629 = (((_592.x * _624) + (_620 ^ (_620 >> 18u))) + _625) * _624;
        uint _631 = _629 ^ (_629 >> 11u);
        uint _634 = _631 ^ ((_631 << 7u) & 2636928640u);
        uint _637 = _634 ^ ((_634 << 15u) & 4022730752u);
        uint _639 = _637 ^ (_637 >> 18u);
        uint _641 = _639 + (_639 << 10u);
        uint _643 = _641 ^ (_641 >> 6u);
        uint _645 = _643 + (_643 << 3u);
        uint _647 = _645 ^ (_645 >> 11u);
        uint _650 = _551 ? _584 : 8388607u;
        uint _651 = _551 ? _585 : 1065353216u;
        uint _656 = _606 ? _650 : 8388607u;
        uint _657 = _606 ? _651 : 1065353216u;
        uint3 _664 = uint3(_446 + int3(1, 1, 0));
        uint _667 = _664.z;
        uint _669 = _667 + (_667 << 10u);
        uint _671 = _669 ^ (_669 >> 6u);
        uint _673 = _671 + (_671 << 3u);
        uint _675 = _673 ^ (_673 >> 11u);
        bool _678 = _623 ? _623 : true;
        uint _679 = _623 ? _624 : 1664525u;
        uint _680 = _623 ? _625 : 1013904223u;
        uint _684 = (((_664.y * _679) + (_675 + (_675 << 15u))) + _680) * _679;
        uint _686 = _684 ^ (_684 >> 11u);
        uint _689 = _686 ^ ((_686 << 7u) & 2636928640u);
        uint _692 = _689 ^ ((_689 << 15u) & 4022730752u);
        bool _695 = _678 ? _678 : true;
        uint _696 = _678 ? _679 : 1664525u;
        uint _697 = _678 ? _680 : 1013904223u;
        uint _701 = (((_664.x * _696) + (_692 ^ (_692 >> 18u))) + _697) * _696;
        uint _703 = _701 ^ (_701 >> 11u);
        uint _706 = _703 ^ ((_703 << 7u) & 2636928640u);
        uint _709 = _706 ^ ((_706 << 15u) & 4022730752u);
        uint _711 = _709 ^ (_709 >> 18u);
        uint _713 = _711 + (_711 << 10u);
        uint _715 = _713 ^ (_713 >> 6u);
        uint _717 = _715 + (_715 << 3u);
        uint _719 = _717 ^ (_717 >> 11u);
        uint _722 = _623 ? _656 : 8388607u;
        uint _723 = _623 ? _657 : 1065353216u;
        uint _728 = _678 ? _722 : 8388607u;
        uint _729 = _678 ? _723 : 1065353216u;
        uint3 _736 = uint3(_446 + int3(0, 0, 1));
        uint _739 = _736.z;
        uint _741 = _739 + (_739 << 10u);
        uint _743 = _741 ^ (_741 >> 6u);
        uint _745 = _743 + (_743 << 3u);
        uint _747 = _745 ^ (_745 >> 11u);
        bool _750 = _695 ? _695 : true;
        uint _751 = _695 ? _696 : 1664525u;
        uint _752 = _695 ? _697 : 1013904223u;
        uint _756 = (((_736.y * _751) + (_747 + (_747 << 15u))) + _752) * _751;
        uint _758 = _756 ^ (_756 >> 11u);
        uint _761 = _758 ^ ((_758 << 7u) & 2636928640u);
        uint _764 = _761 ^ ((_761 << 15u) & 4022730752u);
        bool _767 = _750 ? _750 : true;
        uint _768 = _750 ? _751 : 1664525u;
        uint _769 = _750 ? _752 : 1013904223u;
        uint _773 = (((_736.x * _768) + (_764 ^ (_764 >> 18u))) + _769) * _768;
        uint _775 = _773 ^ (_773 >> 11u);
        uint _778 = _775 ^ ((_775 << 7u) & 2636928640u);
        uint _781 = _778 ^ ((_778 << 15u) & 4022730752u);
        uint _783 = _781 ^ (_781 >> 18u);
        uint _785 = _783 + (_783 << 10u);
        uint _787 = _785 ^ (_785 >> 6u);
        uint _789 = _787 + (_787 << 3u);
        uint _791 = _789 ^ (_789 >> 11u);
        uint _794 = _695 ? _728 : 8388607u;
        uint _795 = _695 ? _729 : 1065353216u;
        uint _800 = _750 ? _794 : 8388607u;
        uint _801 = _750 ? _795 : 1065353216u;
        uint3 _808 = uint3(_446 + int3(1, 0, 1));
        uint _811 = _808.z;
        uint _813 = _811 + (_811 << 10u);
        uint _815 = _813 ^ (_813 >> 6u);
        uint _817 = _815 + (_815 << 3u);
        uint _819 = _817 ^ (_817 >> 11u);
        bool _822 = _767 ? _767 : true;
        uint _823 = _767 ? _768 : 1664525u;
        uint _824 = _767 ? _769 : 1013904223u;
        uint _828 = (((_808.y * _823) + (_819 + (_819 << 15u))) + _824) * _823;
        uint _830 = _828 ^ (_828 >> 11u);
        uint _833 = _830 ^ ((_830 << 7u) & 2636928640u);
        uint _836 = _833 ^ ((_833 << 15u) & 4022730752u);
        bool _839 = _822 ? _822 : true;
        uint _840 = _822 ? _823 : 1664525u;
        uint _841 = _822 ? _824 : 1013904223u;
        uint _845 = (((_808.x * _840) + (_836 ^ (_836 >> 18u))) + _841) * _840;
        uint _847 = _845 ^ (_845 >> 11u);
        uint _850 = _847 ^ ((_847 << 7u) & 2636928640u);
        uint _853 = _850 ^ ((_850 << 15u) & 4022730752u);
        uint _855 = _853 ^ (_853 >> 18u);
        uint _857 = _855 + (_855 << 10u);
        uint _859 = _857 ^ (_857 >> 6u);
        uint _861 = _859 + (_859 << 3u);
        uint _863 = _861 ^ (_861 >> 11u);
        uint _866 = _767 ? _800 : 8388607u;
        uint _867 = _767 ? _801 : 1065353216u;
        uint _872 = _822 ? _866 : 8388607u;
        uint _873 = _822 ? _867 : 1065353216u;
        uint3 _880 = uint3(_446 + int3(0, 1, 1));
        uint _883 = _880.z;
        uint _885 = _883 + (_883 << 10u);
        uint _887 = _885 ^ (_885 >> 6u);
        uint _889 = _887 + (_887 << 3u);
        uint _891 = _889 ^ (_889 >> 11u);
        bool _894 = _839 ? _839 : true;
        uint _895 = _839 ? _840 : 1664525u;
        uint _896 = _839 ? _841 : 1013904223u;
        uint _900 = (((_880.y * _895) + (_891 + (_891 << 15u))) + _896) * _895;
        uint _902 = _900 ^ (_900 >> 11u);
        uint _905 = _902 ^ ((_902 << 7u) & 2636928640u);
        uint _908 = _905 ^ ((_905 << 15u) & 4022730752u);
        bool _911 = _894 ? _894 : true;
        uint _912 = _894 ? _895 : 1664525u;
        uint _913 = _894 ? _896 : 1013904223u;
        uint _917 = (((_880.x * _912) + (_908 ^ (_908 >> 18u))) + _913) * _912;
        uint _919 = _917 ^ (_917 >> 11u);
        uint _922 = _919 ^ ((_919 << 7u) & 2636928640u);
        uint _925 = _922 ^ ((_922 << 15u) & 4022730752u);
        uint _927 = _925 ^ (_925 >> 18u);
        uint _929 = _927 + (_927 << 10u);
        uint _931 = _929 ^ (_929 >> 6u);
        uint _933 = _931 + (_931 << 3u);
        uint _935 = _933 ^ (_933 >> 11u);
        uint _938 = _839 ? _872 : 8388607u;
        uint _939 = _839 ? _873 : 1065353216u;
        uint _944 = _894 ? _938 : 8388607u;
        uint _945 = _894 ? _939 : 1065353216u;
        uint3 _952 = uint3(_446 + int3(1, 1, 1));
        uint _955 = _952.z;
        uint _957 = _955 + (_955 << 10u);
        uint _959 = _957 ^ (_957 >> 6u);
        uint _961 = _959 + (_959 << 3u);
        uint _963 = _961 ^ (_961 >> 11u);
        bool _966 = _911 ? _911 : true;
        uint _967 = _911 ? _912 : 1664525u;
        uint _968 = _911 ? _913 : 1013904223u;
        uint _972 = (((_952.y * _967) + (_963 + (_963 << 15u))) + _968) * _967;
        uint _974 = _972 ^ (_972 >> 11u);
        uint _977 = _974 ^ ((_974 << 7u) & 2636928640u);
        uint _980 = _977 ^ ((_977 << 15u) & 4022730752u);
        uint _983 = _966 ? _967 : 1664525u;
        uint _988 = (((_952.x * _983) + (_980 ^ (_980 >> 18u))) + (_966 ? _968 : 1013904223u)) * _983;
        uint _990 = _988 ^ (_988 >> 11u);
        uint _993 = _990 ^ ((_990 << 7u) & 2636928640u);
        uint _996 = _993 ^ ((_993 << 15u) & 4022730752u);
        uint _998 = _996 ^ (_996 >> 18u);
        uint _1000 = _998 + (_998 << 10u);
        uint _1002 = _1000 ^ (_1000 >> 6u);
        uint _1004 = _1002 + (_1002 << 3u);
        uint _1006 = _1004 ^ (_1004 >> 11u);
        uint _1009 = _911 ? _944 : 8388607u;
        uint _1010 = _911 ? _945 : 1065353216u;
        bool _1015 = _966 ? _966 : true;
        uint _1016 = _966 ? _1009 : 8388607u;
        uint _1017 = _966 ? _1010 : 1065353216u;
        float3 _1026 = (_447 * _447) * (3.0f.xxx - (_447 * 2.0f));
        float _1027 = _1026.x;
        float _1028 = 1.0f - _1027;
        float _1030 = _1026.y;
        float _1031 = 1.0f - _1030;
        float _1033 = _1026.z;
        float _1034 = 1.0f - _1033;
        float2 _1047 = (((((float2(asfloat((_495 & _506) | _507) - 1.0f, asfloat(((_503 + (_503 << 15u)) & _512) | _513) - 1.0f) * _1028) * _1031) * _1034) + (((float2(asfloat((_567 & _578) | _579) - 1.0f, asfloat(((_575 + (_575 << 15u)) & _584) | _585) - 1.0f) * _1027) * _1031) * _1034)) + (((float2(asfloat((_639 & _650) | _651) - 1.0f, asfloat(((_647 + (_647 << 15u)) & _656) | _657) - 1.0f) * _1028) * _1030) * _1034)) + (((float2(asfloat((_711 & _722) | _723) - 1.0f, asfloat(((_719 + (_719 << 15u)) & _728) | _729) - 1.0f) * _1027) * _1030) * _1034);
        float2 _1063 = (((_1047 + (((float2(asfloat((_783 & _794) | _795) - 1.0f, asfloat(((_791 + (_791 << 15u)) & _800) | _801) - 1.0f) * _1028) * _1031) * _1033)) + (((float2(asfloat((_855 & _866) | _867) - 1.0f, asfloat(((_863 + (_863 << 15u)) & _872) | _873) - 1.0f) * _1027) * _1031) * _1033)) + (((float2(asfloat((_927 & _938) | _939) - 1.0f, asfloat(((_935 + (_935 << 15u)) & _944) | _945) - 1.0f) * _1028) * _1030) * _1033)) + (((float2(asfloat((_998 & _1009) | _1010) - 1.0f, asfloat(((_1006 + (_1006 << 15u)) & _1016) | _1017) - 1.0f) * _1027) * _1030) * _1033);
        float2 _1066 = clamp((_1063 - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
        float3 _1071 = _443 + (float3(_1066.x, 0.0f, _1066.y) * 0.0599999986588954925537109375f);
        uint _1092 = uint(ivy_leaf_buffer.Load<int>(uint(gl_InstanceIndex) * 48 + 28) ^ 2342304);
        uint _1094 = _1092 + (_1092 << 10u);
        uint _1096 = _1094 ^ (_1094 >> 6u);
        uint _1098 = _1096 + (_1096 << 3u);
        uint _1100 = _1098 ^ (_1098 >> 11u);
        bool _1103 = _1015 ? _1015 : true;
        uint _1104 = _1015 ? _1016 : 8388607u;
        uint _1105 = _1015 ? _1017 : 1065353216u;
        uint _1112 = uint(ivy_leaf_buffer.Load<int>(uint(gl_InstanceIndex) * 48 + 28) ^ 9344565);
        uint _1114 = _1112 + (_1112 << 10u);
        uint _1116 = _1114 ^ (_1114 >> 6u);
        uint _1118 = _1116 + (_1116 << 3u);
        uint _1120 = _1118 ^ (_1118 >> 11u);
        _1131 = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_1071, 1.0f));
        _1132 = _1071;
        _1133 = mul(_237, float4(in_var_Vertex_Normal, 0.0f)).xyz;
        _1134 = (in_var_Vertex_UV * (_195 ? 0.300000011920928955078125f : 0.100000001490116119384765625f)) + float2(asfloat(((_1100 + (_1100 << 15u)) & _1104) | _1105) - 1.0f, asfloat(((_1120 + (_1120 << 15u)) & (_1103 ? _1104 : 8388607u)) | (_1103 ? _1105 : 1065353216u)) - 1.0f);
        break;
    } while(false);
    gl_Position = _1131;
    out_var_C1 = _1132;
    out_var_C2 = _1133;
    out_var_C4 = _1134;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
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
    int is_flower : packoffset(c0.y);
    column_major float4x4 model_to_world : packoffset(c1);
    uint2 ivy_leaf_color_dims : packoffset(c5);
    uint2 terrain_heightmap_dims : packoffset(c5.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c6);
    uint2 path_mask_dims : packoffset(c6.z);
    int bool_render_shadows : packoffset(c7);
};


static float4 gl_FragCoord;
struct SPIRV_Cross_Input
{
    float4 gl_FragCoord : SV_Position;
};

void frag_main()
{
    if (!((bool_render_shadows == 1) ? true : false))
    {
        uint2 _77 = uint2(gl_FragCoord.xy) & uint2(1u, 1u);
        if ((frame_constants_frame_index & 3u) == (_77.x + (_77.y * 2u)))
        {
            discard;
        }
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
