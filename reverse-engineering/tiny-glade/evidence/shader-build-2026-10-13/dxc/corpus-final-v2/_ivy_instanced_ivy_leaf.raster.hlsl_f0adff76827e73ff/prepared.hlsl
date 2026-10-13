#define SHADER_VARIANT_DEPTH_ONLY 1
#line 1 "shaders/ivy/instanced_ivy_leaf.raster.hlsl"


























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
#line 27 "shaders/ivy/instanced_ivy_leaf.raster.hlsl"

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
#line 28 "shaders/ivy/instanced_ivy_leaf.raster.hlsl"

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
#line 29 "shaders/ivy/instanced_ivy_leaf.raster.hlsl"





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
    float3 position : TEXCOORD0;
    float3 normal : TEXCOORD1;
    float2 uv : TEXCOORD2;
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
