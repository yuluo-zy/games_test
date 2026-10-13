#define PERM_write 0
#define TGV_8faee3cc8c252c04 1
#line 1 "shaders/light_binning/bin_lights_to_cells.cs.hlsl"


#if !((defined(TGV_14dca8ba9c11c197) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)) || (defined(TGV_8faee3cc8c252c04) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))




















#ifndef BIN_LIGHTS_FULL
#define BIN_LIGHTS_FULL 1
#endif

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
#line 28 "shaders/light_binning/bin_lights_to_cells.cs.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 29 "shaders/light_binning/bin_lights_to_cells.cs.hlsl"

#line 1 "shaders/inc/lanterns.hlsl"


#ifndef TINYGLADE_INC_LANTERNS_HLSL
#define TINYGLADE_INC_LANTERNS_HLSL


struct LanternPacked {
    float3 position;
    uint intensity__color_id; 
};


float3 get_lantern_position(ByteAddressBuffer lantern_buf, uint index)
{
    return lantern_buf.Load<float3>(index * 16u + 0u);
}


float get_lantern_intensity(ByteAddressBuffer lantern_buf, uint index)
{
    return f16tof32(lantern_buf.Load<uint>(index * 16u + 12u) & 0xffffu);
}


uint get_lantern_color_id(ByteAddressBuffer lantern_buf, uint index)
{
    return lantern_buf.Load<uint>(index * 16u + 12u) >> 16u;
}


float3 get_lantern_color(uint color_id)
{
    if (color_id == 0u) {
        return float3(0.7919800281524658203125, 0.63358402252197265625, 0.07919800281524658203125) / max(1e-10, 0.7919800281524658203125);
    } else if (color_id == 1u) {
        return float3(0.0, 0.1583960056304931640625, 0.7919800281524658203125) / max(1e-10, 0.215240001678466796875);
    } else if (color_id == 2u) {
        return float3(0.1583960056304931640625, 0.7919800281524658203125, 0.1583960056304931640625) / max(1e-10, 0.772159993648529052734375);
    } else if (color_id == 3u) {
        return float3(0.23759402334690093994140625, 0.1583960056304931640625, 0.7919800281524658203125) / max(1e-10, 0.279020011425018310546875);
    }
    return float3(0.7919800281524658203125, 0.23759402334690093994140625, 0.07919800281524658203125) / max(1e-10, 0.4343800246715545654296875);
}



float3 evaluate_lantern_light(float3 lantern_position, float lantern_intensity, uint color_id,
                              float3 sample_pos, float3 normal)
{
    float3 to_light = lantern_position - sample_pos;
    float dist2 = dot(to_light, to_light);
    float dist = length(to_light);
    float3 l = to_light / max(dist, 1e-6);
    float3 color = get_lantern_color(color_id) * 4.0 * lantern_intensity;
    float atten = (1.0 / (0.3359999954700469970703125 + dist2)) *
                  smoothstep(36.0 * lantern_intensity, 16.0 * lantern_intensity, dist2);
    float ndl = max(0.0, dot(normal, l));
    float3 up = to_light + float3(0.0, 0.1500000059604644775390625, 0.0);
    float up_atten = smoothstep(-0.5, 0.20000000298023223876953125, up.y / max(length(up), 1e-6));
    return color * ndl * atten * up_atten;
}

#endif 
#line 30 "shaders/light_binning/bin_lights_to_cells.cs.hlsl"

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
#line 31 "shaders/light_binning/bin_lights_to_cells.cs.hlsl"

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
#line 32 "shaders/light_binning/bin_lights_to_cells.cs.hlsl"

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
#line 33 "shaders/light_binning/bin_lights_to_cells.cs.hlsl"

#line 1 "shaders/inc/math.hlsl"





#ifndef TINYGLADE_INC_MATH_HLSL
#define TINYGLADE_INC_MATH_HLSL


float clamp01(float v)
{
    return clamp(v, 0.0f, 1.0f);
}

float3 clamp01(float3 v)
{
    return clamp(v, 0.0f, 1.0f);
}


float length_squared(float3 v)
{
    return dot(v, v);
}


float3 safe_normalize(float3 v)
{
    float len_sq = dot(v, v);
    float len = sqrt(len_sq);
    return len > 1.0e-6f ? v / len : float3(0.0f, 0.0f, 0.0f);
}


float safe_rcp(float v)
{
    return 1.0f / max(abs(v), 1.0e-6f);
}


float rcp_keep_sign(float v)
{
    return sign(v) * safe_rcp(v);
}

#endif 
#line 34 "shaders/light_binning/bin_lights_to_cells.cs.hlsl"

#line 1 "shaders/inc/color/srgb.hlsl"




#ifndef TINYGLADE_INC_COLOR_SRGB_HLSL
#define TINYGLADE_INC_COLOR_SRGB_HLSL

static const float SRGB_ALPHA = 0.04045f;
static const float SRGB_OFFSET = 0.055f;
static const float SRGB_GAMMA = 2.4f;


float srgb_to_linear(float c)
{
    float r;
    if (c <= SRGB_ALPHA)
        r = c * 0.077399380505084991455078125f; 
    else
        r = pow((c + SRGB_OFFSET) * 0.947867333889007568359375f, SRGB_GAMMA); 
    return r;
}

float3 srgb_to_linear(float3 c)
{
    return float3(srgb_to_linear(c.r), srgb_to_linear(c.g), srgb_to_linear(c.b));
}

float4 srgb_to_linear(float4 c)
{
    return float4(srgb_to_linear(c.rgb), c.a);
}


float linear_to_srgb(float c)
{
    float r;
    if (c <= 0.0031308f)
        r = c * 12.92f;
    else
        r = 1.055f * pow(c, 1.0f / SRGB_GAMMA) - SRGB_OFFSET;
    return r;
}

float3 linear_to_srgb(float3 c)
{
    return float3(linear_to_srgb(c.r), linear_to_srgb(c.g), linear_to_srgb(c.b));
}

float4 linear_to_srgb(float4 c)
{
    return float4(linear_to_srgb(c.rgb), c.a);
}

#endif 
#line 35 "shaders/light_binning/bin_lights_to_cells.cs.hlsl"

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
#line 36 "shaders/light_binning/bin_lights_to_cells.cs.hlsl"

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
#line 37 "shaders/light_binning/bin_lights_to_cells.cs.hlsl"

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
#line 38 "shaders/light_binning/bin_lights_to_cells.cs.hlsl"

#line 1 "shaders/inc/pack_unpack.hlsl"




#ifndef TINYGLADE_INC_PACK_UNPACK_HLSL
#define TINYGLADE_INC_PACK_UNPACK_HLSL


uint pack_rgba8(float4 c)
{
    uint r = uint(clamp(c.x, 0.0f, 1.0f) * 255.0f + 0.5f);
    uint g = uint(clamp(c.y, 0.0f, 1.0f) * 255.0f + 0.5f);
    uint b = uint(clamp(c.z, 0.0f, 1.0f) * 255.0f + 0.5f);
    uint a = uint(clamp(c.w, 0.0f, 1.0f) * 255.0f + 0.5f);
    return (a << 24u) | (b << 16u) | (g << 8u) | r;
}


uint pack_rgb8(float3 c)
{
    uint r = uint(clamp(c.x, 0.0f, 1.0f) * 255.0f + 0.5f);
    uint g = uint(clamp(c.y, 0.0f, 1.0f) * 255.0f + 0.5f);
    uint b = uint(clamp(c.z, 0.0f, 1.0f) * 255.0f + 0.5f);
    return (b << 16u) | (g << 8u) | r;
}


float3 unpack_rgb8(uint p)
{
    float3 c;
    c.r = float(p & 0xFFu) * (1.0f / 255.0f);
    c.g = float((p >> 8u) & 0xFFu) * (1.0f / 255.0f);
    c.b = float((p >> 16u) & 0xFFu) * (1.0f / 255.0f);
    return c;
}


float4 unpack_rgba8(uint p)
{
    return float4(unpack_rgb8(p), float((p >> 24u) & 0xFFu) * (1.0f / 255.0f));
}


float3 unpack_normal_rgb8(uint p)
{
    return unpack_rgb8(p) * 2.0f - 1.0f;
}

#endif 
#line 39 "shaders/light_binning/bin_lights_to_cells.cs.hlsl"

#line 1 "shaders/light_binning/grid.inc.hlsl"












#ifndef TINYGLADE_LIGHT_BINNING_GRID_INC_HLSL
#define TINYGLADE_LIGHT_BINNING_GRID_INC_HLSL


static const float3 GRID_WORLD_SCALE = float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f);
static const float3 GRID_WORLD_OFFSET = float3(16.0f, 2.0f, 16.0f);
static const uint3 GRID_DIMS = uint3(32u, 16u, 32u);
static const float3 GRID_CELL_SIZE = float3(4.0625f, 8.125f, 4.0625f);


uint3 grid_world_pos_to_cell(float3 world_pos)
{
    return uint3(clamp(floor(world_pos * GRID_WORLD_SCALE + GRID_WORLD_OFFSET),
                       float3(0.0f, 0.0f, 0.0f), float3(GRID_DIMS) - 1.0f));
}



uint grid_cell_index(uint3 cell)
{
    return 32u * (32u * cell.y + cell.z) + cell.x;
}


float3 grid_cell_min_world(uint3 cell)
{
    return (float3(cell) - GRID_WORLD_OFFSET) * GRID_CELL_SIZE;
}

float3 grid_cell_max_world(uint3 cell)
{
    return (float3(cell) - GRID_WORLD_OFFSET + float3(1.0f, 1.0f, 1.0f)) * GRID_CELL_SIZE;
}



float grid_point_to_cell_dist2(float3 p, float r2, uint3 cell)
{
    float3 mn = grid_cell_min_world(cell);
    float3 mx = grid_cell_max_world(cell);
    float d2 = r2;
    [unroll]
    for (uint i = 0u; i < 3u; ++i) {
        if (p[i] < mn[i]) {
            float e = p[i] - mn[i];
            d2 -= e * e;
        } else if (p[i] > mx[i]) {
            float e = p[i] - mx[i];
            d2 -= e * e;
        }
    }
    return d2;
}

#endif 
#line 40 "shaders/light_binning/bin_lights_to_cells.cs.hlsl"


StructuredBuffer<LanternPacked> lantern_buf : register(t13, space0);

cbuffer $Globals : register(b14, space0)
{
    uint lantern_count;
};

RWByteAddressBuffer __alias_indirection_grid : register(u15, space0);
#if BIN_LIGHTS_FULL
RWByteAddressBuffer __alias_light_indices : register(u16, space0);
#endif



uint wang_hash(uint x)
{
    x ^= (x >> 11u);
    x ^= (x << 7u) & 2636928640u;
    x ^= (x << 15u) & 4022730752u;
    x ^= (x >> 18u);
    return x;
}

[numthreads(64, 1, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    uint lantern_idx = gl_GlobalInvocationID.x;
    if (lantern_idx >= lantern_count) {
        return;
    }

    float3 lantern_pos = lantern_buf[lantern_idx].position;
    float intensity = f16tof32(lantern_buf[lantern_idx].intensity__color_id & 0xFFFFu);
    if (intensity <= 0.0f) {
        return;
    }

    
    
    float radius = 6.0f * asfloat(532487669 + (asint(intensity) >> 1));

    
    uint3 cell_min = uint3(clamp(floor((lantern_pos - radius) * GRID_WORLD_SCALE + GRID_WORLD_OFFSET),
                                 0.0f, float3(GRID_DIMS) - 1.0f));
    uint3 cell_max = uint3(clamp(ceil((lantern_pos + radius) * GRID_WORLD_SCALE + GRID_WORLD_OFFSET),
                                 0.0f, float3(GRID_DIMS)));

#if BIN_LIGHTS_FULL
    
    uint hash_state = wang_hash(((lantern_idx * 1664525u) + hash(frame_index) + 1013904223u) * 1664525u);
#endif

    for (uint z = cell_min.z; z < cell_max.z; z++) {
        for (uint y = cell_min.y; y < cell_max.y; y++) {
            for (uint x = cell_min.x; x < cell_max.x; x++) {
                uint3 cell = uint3(x, y, z);
                float3 aabb_min = grid_cell_min_world(cell);
                float3 aabb_max = grid_cell_max_world(cell);

                
                float d2 = radius * radius;
                [unroll]
                for (uint i = 0u; i < 3u; ++i) {
                    if (lantern_pos[i] < aabb_min[i]) {
                        float e = lantern_pos[i] - aabb_min[i];
                        d2 -= e * e;
                    } else if (lantern_pos[i] > aabb_max[i]) {
                        float e = lantern_pos[i] - aabb_max[i];
                        d2 -= e * e;
                    }
                }

                if (d2 > 0.0f) {
                    uint cell_idx = grid_cell_index(cell);
#if BIN_LIGHTS_FULL
                    uint prev;
                    __alias_indirection_grid.InterlockedAdd(cell_idx * 4u, 1u << 20u, prev);
                    uint start = prev & 0xFFFFFu;
                    uint count = prev >> 20u;
                    if (count < 64u) {
                        __alias_light_indices.Store<uint>((start + count) * 4u, lantern_idx);
                    } else {
                        
                        
                        float u = asfloat((hash_state & 0x7FFFFFu) | 0x3F800000u) - 1.0f;
                        hash_state = hash(hash_state);
                        if (u < (64.0f / float(count + 1u))) {
                            __alias_light_indices.Store<uint>((start + (hash_state % 64u)) * 4u, lantern_idx);
                            hash_state = hash(hash_state);
                        }
                    }
#else
                    __alias_indirection_grid.InterlockedAdd(cell_idx * 4u, 1u);
#endif
                }
            }
        }
    }
}

#endif

#if defined(TGV_14dca8ba9c11c197) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)

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

struct LanternPacked
{
    float3 position;
    uint intensity_color_id;
};

static uint _86;

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

ByteAddressBuffer lantern_buf : register(t13, space0);
cbuffer $Globals : register(b14, space0)
{
    uint lantern_count : packoffset(c0);
};

RWByteAddressBuffer __alias_indirection_grid : register(u15, space0);
RWByteAddressBuffer __alias_light_indices : register(u16, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void comp_main()
{
    do
    {
        if (gl_GlobalInvocationID.x >= lantern_count)
        {
            break;
        }
        float2 _121 = spvUnpackHalf2x16(lantern_buf.Load<uint>(gl_GlobalInvocationID.x * 16 + 12) & 65535u);
        float _122 = _121.x;
        if (_122 <= 0.0f)
        {
            break;
        }
        precise float _130 = 6.0f * asfloat(532487669 + (asint(_122) >> 1));
        float3 _131 = _130.xxx;
        uint3 _138 = uint3(clamp(floor(((lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0) - _131) * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(31.0f, 15.0f, 31.0f)));
        uint3 _143 = uint3(clamp(ceil(((lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0) + _131) * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(32.0f, 16.0f, 32.0f)));
        uint _147 = frame_constants_frame_index + (frame_constants_frame_index << 10u);
        uint _149 = _147 ^ (_147 >> 6u);
        uint _151 = _149 + (_149 << 3u);
        uint _153 = _151 ^ (_151 >> 11u);
        uint _159 = (((gl_GlobalInvocationID.x * 1664525u) + (_153 + (_153 << 15u))) + 1013904223u) * 1664525u;
        uint _161 = _159 ^ (_159 >> 11u);
        uint _164 = _161 ^ ((_161 << 7u) & 2636928640u);
        uint _167 = _164 ^ ((_164 << 15u) & 4022730752u);
        uint _169 = _167 ^ (_167 >> 18u);
        uint _170 = _138.z;
        uint _176;
        uint _178;
        bool _180;
        bool _182;
        uint _184;
        uint _172 = _170;
        uint _175 = 0u;
        uint _177 = 0u;
        bool _179 = false;
        bool _181 = false;
        uint _183 = _169;
        for (; _172 < _143.z; _172++, _175 = _176, _177 = _178, _179 = _180, _181 = _182, _183 = _184)
        {
            uint _189 = _138.y;
            _176 = _175;
            _178 = _177;
            _180 = _179;
            _182 = _181;
            _184 = _183;
            uint _194;
            uint _195;
            bool _196;
            bool _197;
            uint _198;
            for (uint _191 = _189; _191 < _143.y; _191++, _176 = _194, _178 = _195, _180 = _196, _182 = _197, _184 = _198)
            {
                uint _203 = _138.x;
                _194 = _176;
                _195 = _178;
                _196 = _180;
                _197 = _182;
                _198 = _184;
                uint _205;
                uint _207;
                bool _208;
                bool _209;
                uint _210;
                for (uint _211 = _203; _211 < _143.x; _194 = _205, _195 = _207, _196 = _208, _197 = _209, _198 = _210, _211++)
                {
                    float3 _218 = float3(uint3(_211, _191, _172));
                    float3 _220 = (_218 - float3(16.0f, 2.0f, 16.0f)) * float3(4.0625f, 8.125f, 4.0625f);
                    float3 _222 = (_218 + float3(-15.0f, -1.0f, -15.0f)) * float3(4.0625f, 8.125f, 4.0625f);
                    precise float _223 = _130 * _130;
                    float _225 = _220.x;
                    float _241;
                    if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).x < _225)
                    {
                        float _238 = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).x - _225;
                        _241 = _223 - (_238 * _238);
                    }
                    else
                    {
                        float _230 = _222.x;
                        float _237;
                        if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).x > _230)
                        {
                            float _234 = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).x - _230;
                            _237 = _223 - (_234 * _234);
                        }
                        else
                        {
                            _237 = _223;
                        }
                        _241 = _237;
                    }
                    float _243 = _220.y;
                    float _259;
                    if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).y < _243)
                    {
                        float _256 = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).y - _243;
                        _259 = _241 - (_256 * _256);
                    }
                    else
                    {
                        float _248 = _222.y;
                        float _255;
                        if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).y > _248)
                        {
                            float _252 = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).y - _248;
                            _255 = _241 - (_252 * _252);
                        }
                        else
                        {
                            _255 = _241;
                        }
                        _259 = _255;
                    }
                    float _261 = _220.z;
                    float _277;
                    if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).z < _261)
                    {
                        float _274 = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).z - _261;
                        _277 = _259 - (_274 * _274);
                    }
                    else
                    {
                        float _266 = _222.z;
                        float _273;
                        if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).z > _266)
                        {
                            float _270 = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).z - _266;
                            _273 = _259 - (_270 * _270);
                        }
                        else
                        {
                            _273 = _259;
                        }
                        _277 = _273;
                    }
                    if (_277 > 0.0f)
                    {
                        uint _286;
                        __alias_indirection_grid.InterlockedAdd(((32u * ((32u * _191) + _172)) + _211) * 4 + 0, 1048576u, _286);
                        uint _287 = _286 & 1048575u;
                        uint _288 = _286 >> 20u;
                        uint _333;
                        uint _334;
                        bool _335;
                        bool _336;
                        uint _337;
                        if (_288 < 64u)
                        {
                            __alias_light_indices.Store<uint>((_287 + _288) * 4 + 0, gl_GlobalInvocationID.x);
                            _333 = _194;
                            _334 = _195;
                            _335 = _196;
                            _336 = _197;
                            _337 = _198;
                        }
                        else
                        {
                            uint _294 = _198 + (_198 << 10u);
                            uint _296 = _294 ^ (_294 >> 6u);
                            uint _298 = _296 + (_296 << 3u);
                            uint _300 = _298 ^ (_298 >> 11u);
                            uint _302 = _300 + (_300 << 15u);
                            uint _304 = _197 ? _195 : 8388607u;
                            uint _306 = _196 ? _194 : 1065353216u;
                            uint _330;
                            if ((asfloat((_198 & _304) | _306) - 1.0f) < (64.0f / float(_288 + 1u)))
                            {
                                uint _318 = _302 + (_302 << 10u);
                                uint _320 = _318 ^ (_318 >> 6u);
                                uint _322 = _320 + (_320 << 3u);
                                uint _324 = _322 ^ (_322 >> 11u);
                                __alias_light_indices.Store<uint>((_287 + (_302 % 64u)) * 4 + 0, gl_GlobalInvocationID.x);
                                _330 = _324 + (_324 << 15u);
                            }
                            else
                            {
                                _330 = _302;
                            }
                            _333 = _306;
                            _334 = _304;
                            _335 = _196 ? _196 : true;
                            _336 = _197 ? _197 : true;
                            _337 = _330;
                        }
                        _205 = _333;
                        _207 = _334;
                        _208 = _335;
                        _209 = _336;
                        _210 = _337;
                    }
                    else
                    {
                        _205 = _194;
                        _207 = _195;
                        _208 = _196;
                        _209 = _197;
                        _210 = _198;
                    }
                }
            }
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

#if defined(TGV_8faee3cc8c252c04) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct LanternPacked
{
    float3 position;
    uint intensity_color_id;
};

ByteAddressBuffer lantern_buf : register(t13, space0);
cbuffer $Globals : register(b14, space0)
{
    uint lantern_count : packoffset(c0);
};

RWByteAddressBuffer __alias_indirection_grid : register(u15, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void comp_main()
{
    do
    {
        if (gl_GlobalInvocationID.x >= lantern_count)
        {
            break;
        }
        float2 _88 = spvUnpackHalf2x16(lantern_buf.Load<uint>(gl_GlobalInvocationID.x * 16 + 12) & 65535u);
        float _89 = _88.x;
        if (_89 <= 0.0f)
        {
            break;
        }
        precise float _97 = 6.0f * asfloat(532487669 + (asint(_89) >> 1));
        float3 _98 = _97.xxx;
        uint3 _105 = uint3(clamp(floor(((lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0) - _98) * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(31.0f, 15.0f, 31.0f)));
        uint3 _110 = uint3(clamp(ceil(((lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0) + _98) * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(32.0f, 16.0f, 32.0f)));
        uint _111 = _105.z;
        for (uint _113 = _111; _113 < _110.z; _113++)
        {
            uint _120 = _105.y;
            for (uint _122 = _120; _122 < _110.y; _122++)
            {
                uint _129 = _105.x;
                for (uint _131 = _129; _131 < _110.x; _131++)
                {
                    float3 _139 = float3(uint3(_131, _122, _113));
                    float3 _141 = (_139 - float3(16.0f, 2.0f, 16.0f)) * float3(4.0625f, 8.125f, 4.0625f);
                    float3 _143 = (_139 + float3(-15.0f, -1.0f, -15.0f)) * float3(4.0625f, 8.125f, 4.0625f);
                    precise float _144 = _97 * _97;
                    float _146 = _141.x;
                    float _162;
                    if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).x < _146)
                    {
                        float _151 = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).x - _146;
                        _162 = _144 - (_151 * _151);
                    }
                    else
                    {
                        float _154 = _143.x;
                        float _161;
                        if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).x > _154)
                        {
                            float _158 = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).x - _154;
                            _161 = _144 - (_158 * _158);
                        }
                        else
                        {
                            _161 = _144;
                        }
                        _162 = _161;
                    }
                    float _164 = _141.y;
                    float _180;
                    if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).y < _164)
                    {
                        float _169 = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).y - _164;
                        _180 = _162 - (_169 * _169);
                    }
                    else
                    {
                        float _172 = _143.y;
                        float _179;
                        if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).y > _172)
                        {
                            float _176 = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).y - _172;
                            _179 = _162 - (_176 * _176);
                        }
                        else
                        {
                            _179 = _162;
                        }
                        _180 = _179;
                    }
                    float _182 = _141.z;
                    float _198;
                    if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).z < _182)
                    {
                        float _187 = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).z - _182;
                        _198 = _180 - (_187 * _187);
                    }
                    else
                    {
                        float _190 = _143.z;
                        float _197;
                        if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).z > _190)
                        {
                            float _194 = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).z - _190;
                            _197 = _180 - (_194 * _194);
                        }
                        else
                        {
                            _197 = _180;
                        }
                        _198 = _197;
                    }
                    if (_198 > 0.0f)
                    {
                        uint _207;
                        __alias_indirection_grid.InterlockedAdd(((32u * ((32u * _122) + _113)) + _131) * 4 + 0, 1u, _207);
                    }
                }
            }
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
