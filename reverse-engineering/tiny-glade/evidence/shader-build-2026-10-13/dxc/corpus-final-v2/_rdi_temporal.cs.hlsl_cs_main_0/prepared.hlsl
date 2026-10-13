#line 1 "shaders/rdi/temporal.cs.hlsl"














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
#line 15 "shaders/rdi/temporal.cs.hlsl"

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
#line 16 "shaders/rdi/temporal.cs.hlsl"

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
#line 17 "shaders/rdi/temporal.cs.hlsl"

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
#line 18 "shaders/rdi/temporal.cs.hlsl"

#line 1 "shaders/inc/reservoir.hlsl"


#ifndef TINYGLADE_INC_RESERVOIR_HLSL
#define TINYGLADE_INC_RESERVOIR_HLSL


struct Reservoir1spp {
    float w_sum;
    uint payload;
    float M;
    float W;
};


struct Reservoir1sppStreamState {
    float p_q_sel; 
    float M_sum;   
};


void reservoir_init(out Reservoir1spp r, out Reservoir1sppStreamState s)
{
    r.w_sum = 0.0;
    r.payload = 0xFFFFFFFFu;
    r.M = 0.0;
    r.W = 0.0;
    s.p_q_sel = 0.0;
    s.M_sum = 0.0;
}




bool reservoir_update(inout Reservoir1spp r, inout Reservoir1sppStreamState s,
                      float w, float target_pdf, uint payload, float u)
{
    s.M_sum += 1.0;
    r.M += 1.0;
    r.w_sum += w;
    if (w / r.w_sum >= u) {
        r.payload = payload;
        s.p_q_sel = target_pdf;
        return true;
    }
    return false;
}


void reservoir_combine_streams(inout Reservoir1sppStreamState dst, Reservoir1sppStreamState src)
{
    dst.M_sum += src.M_sum;
}



void reservoir_resolve(inout Reservoir1spp r, Reservoir1sppStreamState s)
{
    r.M = s.M_sum;
    r.W = r.w_sum / max(1e-8, r.M * s.p_q_sel);
}


uint4 pack_reservoir1spp(Reservoir1spp r)
{
    return uint4(asuint(r.w_sum), r.payload, asuint(r.M), asuint(r.W));
}


Reservoir1spp unpack_reservoir1spp(uint4 v)
{
    Reservoir1spp r;
    r.w_sum = asfloat(v.x);
    r.payload = v.y;
    r.M = asfloat(v.z);
    r.W = asfloat(v.w);
    return r;
}

#endif 
#line 19 "shaders/rdi/temporal.cs.hlsl"



#line 1 "shaders/inc/history_valid.hlsl"












#ifndef TINYGLADE_INC_HISTORY_VALID_HLSL
#define TINYGLADE_INC_HISTORY_VALID_HLSL

struct FrexpScalarType {
    float mantissa;
    float exponent;
};

FrexpScalarType incb_frexp_scalar(float v)
{
    FrexpScalarType r;
    uint bits = asuint(v);
    int biased = int((bits >> 23u) & 0xFFu);
    int e = biased - 127;
    float m = asfloat((bits & 0x807FFFFFu) | 0x3F800000u);
    bool big = (m >= 1.0f) | (m <= -1.0f);
    m = lerp(m, m * 0.5f, big);
    e += big ? 1 : 0;
    bool special = (biased == 0) | (biased == 0xFF);
    r.exponent = special ? 0.0f : float(e);
    r.mantissa = special ? v : m;
    return r;
}



bool history_depth_valid(float current_depth, float history_depth, float epsilon)
{
    FrexpScalarType c = incb_frexp_scalar(current_depth);
    float scale = exp2(c.exponent);
    return abs(current_depth - history_depth) * scale < epsilon;
}






bool is_history_valid(bool reprojected_available, uint history_valid_flag, uint frame_index)
{
    return reprojected_available && (history_valid_flag != 0u) && (frame_index != 0u);
}

#endif 
#line 22 "shaders/rdi/temporal.cs.hlsl"




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
#line 26 "shaders/rdi/temporal.cs.hlsl"

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
#line 27 "shaders/rdi/temporal.cs.hlsl"

#line 1 "shaders/inc/halfres.hlsl"




#ifndef TINYGLADE_INC_HALFRES_HLSL
#define TINYGLADE_INC_HALFRES_HLSL


int2 to_halfres_pixel(int2 full_pixel)
{
    return full_pixel >> 1;
}


int2 from_halfres_pixel(int2 half_pixel)
{
    return half_pixel << 1;
}


uint2 halfres_dims(uint2 full_dims)
{
    return (full_dims + 1u) >> 1;
}

#endif 
#line 28 "shaders/rdi/temporal.cs.hlsl"

#line 1 "shaders/inc/hammersley.hlsl"










float radicalInverse_VdC(uint bits)
{
    bits = (bits << 16u) | (bits >> 16u);
    bits = ((bits & 0x55555555u) << 1u) | ((bits & 0xAAAAAAAAu) >> 1u);
    bits = ((bits & 0x33333333u) << 2u) | ((bits & 0xCCCCCCCCu) >> 2u);
    bits = ((bits & 0x0F0F0F0Fu) << 4u) | ((bits & 0xF0F0F0F0u) >> 4u);
    bits = ((bits & 0x00FF00FFu) << 8u) | ((bits & 0xFF00FF00u) >> 8u);
    return float(bits) * 2.3283064365386963e-10f;
}

float2 hammersley(uint i, uint N)
{
    return float2(float(i) / float(N), radicalInverse_VdC(i));
}
#line 29 "shaders/rdi/temporal.cs.hlsl"

#line 1 "shaders/inc/morton.hlsl"








uint morton_encode(uint2 p)
{
    uint x = p.x & 0xFFFFu;
    x = (x | (x << 8u)) & 0x00FF00FFu;
    x = (x | (x << 4u)) & 0x0F0F0F0Fu;
    x = (x | (x << 2u)) & 0x33333333u;
    x = (x | (x << 1u)) & 0x55555555u;
    uint y = p.y & 0xFFFFu;
    y = (y | (y << 8u)) & 0x00FF00FFu;
    y = (y | (y << 4u)) & 0x0F0F0F0Fu;
    y = (y | (y << 2u)) & 0x33333333u;
    y = (y | (y << 1u)) & 0x55555555u;
    return x | (y << 1u);
}

uint2 morton_decode(uint m)
{
    uint x = m & 0x55555555u;
    x = (x ^ (x >> 1u)) & 0x33333333u;
    x = (x ^ (x >> 2u)) & 0x0F0F0F0Fu;
    x = (x ^ (x >> 4u)) & 0x00FF00FFu;
    x = (x ^ (x >> 8u)) & 0xFFFFu;
    uint y = (m >> 1u) & 0x55555555u;
    y = (y ^ (y >> 1u)) & 0x33333333u;
    y = (y ^ (y >> 2u)) & 0x0F0F0F0Fu;
    y = (y ^ (y >> 4u)) & 0x00FF00FFu;
    y = (y ^ (y >> 8u)) & 0xFFFFu;
    return uint2(x, y);
}
#line 30 "shaders/rdi/temporal.cs.hlsl"

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
#line 31 "shaders/rdi/temporal.cs.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 32 "shaders/rdi/temporal.cs.hlsl"

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
#line 33 "shaders/rdi/temporal.cs.hlsl"

#line 1 "shaders/inc/raymarch.hlsl"


#ifndef TINYGLADE_INC_RAYMARCH_HLSL
#define TINYGLADE_INC_RAYMARCH_HLSL





float raymarch_depth_texture(Texture2D depth_tex, SamplerState s, float2 texel_size,
                             float3 ray_start_view, float3 ray_end_view)
{
    float3 delta = ray_end_view - ray_start_view;
    float dist = length(delta);
    float t = 0.0;
    
    float step = max(0.05, dist / 16.0);
    for (uint i = 0u; i < 64u; ++i) {
        t += step / max(dist, 1e-6);
        if (t >= 1.0) {
            return 1.0;
        }
        float3 p = ray_start_view + delta * t;
        if (p.z <= 0.0) {
            return 1.0;
        }
        float2 uv = p.xy * 0.5 + 0.5;
        uv.y = 1.0 - uv.y;
        float sampled_depth = depth_tex.SampleLevel(s, uv, 0.0).x;
        float inv_depth = 1.0 / sampled_depth;
        float my_inv = 1.0 / p.z;
        if (my_inv > inv_depth) {
            
            float lo = max(0.0, t - step / max(dist, 1e-6));
            for (uint j = 0u; j < 8u; ++j) {
                float mid = (lo + t) * 0.5;
                float3 pm = ray_start_view + delta * mid;
                float2 uvm = pm.xy * 0.5 + 0.5;
                uvm.y = 1.0 - uvm.y;
                if (depth_tex.SampleLevel(s, uvm, 0.0).x > 1.0 / pm.z) {
                    t = mid;
                } else {
                    lo = mid;
                }
            }
            return t;
        }
    }
    return 1.0;
}



float raymarch_opacity(Texture2D depth_tex, SamplerState s, float2 texel_size,
                       float3 ray_start_view, float3 ray_end_view, float density)
{
    float3 delta = ray_end_view - ray_start_view;
    float dist = length(delta);
    float acc = 0.0;
    uint steps = 12u;
    for (uint i = 0u; i < steps; ++i) {
        float t = (float(i) + 0.5) / float(steps);
        float3 p = ray_start_view + delta * t;
        if (p.z <= 0.0) {
            break;
        }
        float2 uv = p.xy * 0.5 + 0.5;
        uv.y = 1.0 - uv.y;
        float inv_depth = 1.0 / max(depth_tex.SampleLevel(s, uv, 0.0).x, 1e-6);
        if (inv_depth < 1.0 / max(p.z, 1e-6)) {
            acc += density * (dist / float(steps));
        }
    }
    return saturate(acc);
}

#endif 
#line 34 "shaders/rdi/temporal.cs.hlsl"

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
#line 35 "shaders/rdi/temporal.cs.hlsl"

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
#line 36 "shaders/rdi/temporal.cs.hlsl"





Texture2D<float4> rdi_temporal_unused_tex_array[];

cbuffer $Globals : register(b14, space0)
{
    uint2 blue_noise_texture_dims;
    uint history_valid;
    uint2 reprojection_tex_dims;
    uint2 half_depth_tex_dims;
    uint2 half_view_normal_tex_dims;
    uint2 rdi_temporal_history_tex_dims;
    uint2 rdi_temporal_out_tex_dims;
    uint lantern_count;
};

StructuredBuffer<LanternPacked> lantern_buf : register(t13, space0);
StructuredBuffer<uint> light_indirection_grid : register(t21, space0);
StructuredBuffer<uint> grid_light_indices : register(t22, space0);
Texture2D<float4> blue_noise_texture : register(t15, space0);
Texture2D<float4> reprojection_tex : register(t16, space0);
Texture2D<float4> half_depth_tex : register(t17, space0);
Texture2D<float4> half_view_normal_tex : register(t18, space0);
Texture2D<uint4> rdi_temporal_history_tex : register(t19, space0);
RWTexture2D<uint2> rdi_temporal_out_tex : register(u20, space0);



static uint undefined_uint;
static float undefined_float;


float2 unpack_half2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

uint pack_half2x16(float2 value)
{
    uint2 packed = f32tof16(value);
    return packed.x | (packed.y << 16);
}


static const float3 LUMA = float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f);

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    do
    {
        float depth = half_depth_tex.Load(int3(gl_GlobalInvocationID.xy, 0u)).x;
        float2 half_dims = float2(half_depth_tex_dims);
        float2 uv = (float2(int2(gl_GlobalInvocationID.xy)) + 0.5f.xx) / half_dims;

        
        uint seed = frame_index + 192192u;
        uint h1 = seed;
        h1 += (h1 << 10u);
        h1 ^= (h1 >> 6u);
        h1 += (h1 << 3u);
        h1 ^= (h1 >> 11u);
        uint a1 = (((gl_GlobalInvocationID.y * 1664525u) + (h1 + (h1 << 15u))) + 1013904223u) * 1664525u;
        a1 ^= (a1 >> 11u);
        a1 ^= ((a1 << 7u) & 2636928640u);
        a1 ^= ((a1 << 15u) & 4022730752u);
        uint b1 = (((gl_GlobalInvocationID.x * 1664525u) + (a1 ^ (a1 >> 18u))) + 1013904223u) * 1664525u;
        b1 ^= (b1 >> 11u);
        b1 ^= ((b1 << 7u) & 2636928640u);
        b1 ^= ((b1 << 15u) & 4022730752u);
        uint rng0 = b1 ^ (b1 >> 18u);

        float2 sample_pos = (uv - 0.5f.xx) * float2(2.0f, -2.0f);
        float sample_x = sample_pos.x;
        float4 view_pos_h = float4(sample_x, sample_pos.y, depth, 1.0f);
        float4 world_pos_h = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view, view_pos_h));

        if (((-1.0f) / (depth * (-view_constants[0].clip_to_view[3].z))) > 2000.0f)
        {
            rdi_temporal_out_tex[gl_GlobalInvocationID.xy] = uint2(0u, 0u);
            break;
        }

        float3 world_normal = mul(view_constants[0].view_to_world, float4(half_view_normal_tex.Load(int3(gl_GlobalInvocationID.xy, 0u)).xyz, 0.0f)).xyz;
        float3 world_pos = world_pos_h.xyz / world_pos_h.w.xxx;

        float4 rnd = ((blue_noise_texture.Load(int3((gl_GlobalInvocationID.xy + uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(frame_index)) + 0.5f.xx) * 256.0f.xx)) % uint2(256u, 256u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;

        uint3 cell = uint3(clamp(floor((world_pos * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(31.0f, 15.0f, 31.0f)));
        uint cell_index = (32u * ((32u * cell.y) + cell.z)) + cell.x;
        uint grid_entry = light_indirection_grid[cell_index];
        uint grid_start = grid_entry & 1048575u;
        uint grid_count = min(64u, grid_entry >> 20u);

        uint rng_state = rng0;
        rng_state += (rng_state << 10u);
        rng_state ^= (rng_state >> 6u);
        rng_state += (rng_state << 3u);
        rng_state ^= (rng_state >> 11u);
        rng_state += (rng_state << 15u);

        uint rand_exp = undefined_uint;
        uint rand_mant = undefined_uint;
        bool exp_seen = false;
        bool mant_seen = false;
        float acc_w = 0.0f;
        uint sel_payload = 0u;
        float m_sum = 0.0f;
        float w_sum = 0.0f;

        for (uint candidate = 0u; candidate < min(2u, grid_count); candidate++)
        {
            uint entry_index = grid_start + ((rng0 + candidate) % grid_count);
            uint lantern_index = grid_light_indices[entry_index];
            float lantern_intensity = unpack_half2x16(lantern_buf[lantern_index].intensity__color_id & 65535u).x;
            uint lantern_color_id = lantern_buf[lantern_index].intensity__color_id >> 16u;
            float3 to_light = lantern_buf[lantern_index].position - world_pos;
            float dist2 = dot(to_light, to_light);
            float3 up_vec = to_light + float3(0.0f, 0.1500000059604644775390625f, 0.0f);

            float3 lantern_color;
            if (lantern_color_id == 0u)
            {
                lantern_color = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
            }
            else if (lantern_color_id == 1u)
            {
                lantern_color = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
            }
            else if (lantern_color_id == 2u)
            {
                lantern_color = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
            }
            else if (lantern_color_id == 3u)
            {
                lantern_color = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
            }
            else
            {
                lantern_color = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
            }

            float w = dot(((lantern_color * 4.0f) * lantern_intensity) * ((max(0.0f, dot(world_normal, to_light / length(to_light).xxx)) * ((1.0f / (0.3359999954700469970703125f + dist2)) * smoothstep(36.0f * lantern_intensity, 16.0f * lantern_intensity, dist2))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (up_vec / length(up_vec).xxx).y)), LUMA);

            uint next_rng = rng_state;
            next_rng += (next_rng << 10u);
            next_rng ^= (next_rng >> 6u);
            next_rng += (next_rng << 3u);
            next_rng ^= (next_rng >> 11u);
            next_rng += (next_rng << 15u);

            bool mant_seen_new = mant_seen ? mant_seen : true;
            uint mant_new = mant_seen ? rand_mant : 8388607u;
            bool exp_seen_new = exp_seen ? exp_seen : true;
            uint exp_new = exp_seen ? rand_exp : 1065353216u;

            float next_m_sum = m_sum + 1.0f;
            float target_w = w * (float(grid_count) / float(lantern_count));
            float next_w_sum = w_sum + target_w;
            uint next_payload;
            float next_acc_w;
            if ((target_w / next_w_sum) >= (asfloat((rng_state & mant_new) | exp_new) - 1.0f))
            {
                next_payload = grid_light_indices[entry_index];
                next_acc_w = w;
            }
            else
            {
                next_payload = sel_payload;
                next_acc_w = acc_w;
            }

            m_sum = next_m_sum;
            rand_mant = mant_new;
            rand_exp = exp_new;
            mant_seen = mant_seen_new;
            exp_seen = exp_seen_new;
            rng_state = next_rng;
            sel_payload = next_payload;
            acc_w = next_acc_w;
            w_sum = next_w_sum;
        }

        bool occluded;
        if (w_sum > 0.0f)
        {
            bool valid;
            do
            {
                float3 to_sel = lantern_buf[sel_payload].position - world_pos;
                float sel_dist = length(to_sel);
                if (sel_dist > 0.5f)
                {
                    float3 view_hit = view_pos_h.xyz;
                    float4 reproj = mul(view_constants[0].view_to_sample, mul(view_constants[0].world_to_view, float4((world_pos + ((to_sel * (sel_dist - 0.5f)) / sel_dist.xxx)) + (world_normal * (0.0199999995529651641845703125f * rnd.y)), 1.0f)));
                    float3 reproj_ndc = reproj.xyz / reproj.w.xxx;
                    float4 seg = view_pos_h + (float4(reproj_ndc - view_hit, 0.0f) * float(int(sign(reproj_ndc.z))));
                    float seg_w = seg.w;
                    float3 seg_ndc = (seg / (float((seg_w >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(seg_w))).xxxx).xyz;
                    float3 seg_dir = seg_ndc - view_hit;
                    bool3 neg = bool3(seg_dir.x < 0.0f.xxx.x, seg_dir.y < 0.0f.xxx.y, seg_dir.z < 0.0f.xxx.z);
                    float3 t_enter = (float3(neg.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, neg.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, neg.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - view_hit) / seg_dir;
                    float3 a = view_hit + (seg_dir * max(0.0f, max(t_enter.x, t_enter.y)));
                    float3 seg_dir2 = seg_ndc - a;
                    bool3 neg2 = bool3(seg_dir2.x >= 0.0f.xxx.x, seg_dir2.y >= 0.0f.xxx.y, seg_dir2.z >= 0.0f.xxx.z);
                    float3 t_exit = (float3(neg2.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, neg2.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, neg2.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - a) / seg_dir2;
                    float3 b = a + (seg_dir2 * min(1.0f, min(min(t_exit.x, t_exit.y), t_exit.z)));
                    float rnd_x = rnd.x;

                    bool hit;
                    float hit_t;
                    float hit_scale;
                    do
                    {
                        float eps = 0.800000011920928955078125f * view_constants[0].clip_to_view[3].z;
                        uint step_count = uint(int(max(2u, min(4u, uint(int(floor(length((((b.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((a.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * half_dims))))))));

                        float d_hi;
                        float d_lo;
                        float t_eps;
                        float t_first;
                        float t_prev;
                        bool crossed;
                        do
                        {
                            float3 seg3 = b - a;
                            float step_scale;
                            if (b.z > 9.5367431640625e-07f)
                            {
                                step_scale = float(step_count);
                            }
                            else
                            {
                                step_scale = max(1.0f, float(step_count) - 1.0f);
                            }
                            float step_inv = 1.0f / step_scale;

                            if (step_count > 0u)
                            {
                                float t0 = lerp(0.0f, 1.0f, pow(rnd_x * step_inv, 1.0f));
                                float3 p0 = a + (seg3 * t0);
                                float2 uv0 = (p0.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                float inv_z0 = 1.0f / p0.z;
                                float inv_d_hi = 1.0f / half_depth_tex.SampleLevel(sampler_llc, uv0, 0.0f).x;
                                float inv_d_lo = 1.0f / half_depth_tex.SampleLevel(sampler_nnc, uv0, 0.0f).x;
                                float err_pos = (max(inv_d_hi, inv_d_lo) * 1.00000202655792236328125f) - inv_z0;
                                float err_neg = inv_z0 - min(inv_d_hi, inv_d_lo);
                                bool crossed0 = (err_pos < 0.0f) ? (err_neg < eps) : false;

                                if (crossed0)
                                {
                                    d_hi = err_pos;
                                    d_lo = err_neg;
                                    t_eps = eps;
                                    t_first = t0;
                                    t_prev = 0.0f;
                                    crossed = crossed0;
                                }
                                else
                                {
                                    float prev_t = t0;
                                    bool prev_crossed = crossed0;
                                    uint step_i = 1u;
                                    bool found_step;
                                    float f_d_hi;
                                    float f_d_lo;
                                    float f_t_eps;
                                    float f_t_first;
                                    for (;;)
                                    {
                                        if (step_i < step_count)
                                        {
                                            float t1 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(step_i) + rnd_x) * step_inv, 1.0f)));
                                            float3 p1 = a + (seg3 * t1);
                                            float2 uv1 = (p1.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                            float inv_z1 = 1.0f / p1.z;
                                            float inv_d_hi1 = 1.0f / half_depth_tex.SampleLevel(sampler_llc, uv1, 0.0f).x;
                                            float inv_d_lo1 = 1.0f / half_depth_tex.SampleLevel(sampler_nnc, uv1, 0.0f).x;
                                            float err_pos1 = (max(inv_d_hi1, inv_d_lo1) * 1.00000202655792236328125f) - inv_z1;
                                            float err_neg1 = inv_z1 - min(inv_d_hi1, inv_d_lo1);
                                            bool crossed1 = (err_pos1 < 0.0f) ? (err_neg1 < eps) : false;
                                            if (crossed1)
                                            {
                                                f_d_hi = err_pos1;
                                                f_d_lo = err_neg1;
                                                f_t_eps = eps;
                                                f_t_first = t1;
                                                found_step = true;
                                                break;
                                            }
                                            step_i++;
                                            prev_crossed = crossed1;
                                            prev_t = t1;
                                            continue;
                                        }
                                        else
                                        {
                                            f_d_hi = 0.0f;
                                            f_d_lo = 0.0f;
                                            f_t_eps = 0.0f;
                                            f_t_first = 1.0f;
                                            found_step = prev_crossed;
                                            break;
                                        }
                                    }
                                    d_hi = f_d_hi;
                                    d_lo = f_d_lo;
                                    t_eps = f_t_eps;
                                    t_first = f_t_first;
                                    t_prev = prev_t;
                                    crossed = found_step;
                                }
                            }
                            else
                            {
                                d_hi = undefined_float;
                                d_lo = undefined_float;
                                t_eps = undefined_float;
                                t_first = 1.0f;
                                t_prev = 0.0f;
                                crossed = false;
                            }
                        } while (false);

                        bool occluded_here;
                        if (crossed)
                        {
                            bool cond;
                            if (d_lo < t_eps)
                            {
                                cond = d_hi < t_eps;
                            }
                            else
                            {
                                cond = false;
                            }
                            occluded_here = cond;
                        }
                        else
                        {
                            occluded_here = false;
                        }
                        if (occluded_here)
                        {
                            hit = true;
                            hit_t = t_first;
                            hit_scale = d_lo / view_constants[0].clip_to_view[3].z;
                            break;
                        }
                        hit = false;
                        hit_t = t_prev;
                        hit_scale = 0.0f;
                        break;
                    } while (false);

                    if (hit)
                    {
                        float4 hit_world = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view, float4(lerp(a, b, hit_t.xxx), 1.0f)));
                        float3 to_light2 = ((hit_world.xyz / hit_world.w.xxx) + ((-normalize(mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view, float4(sample_x, sample_pos.y, 0.0f, 1.0f))).xyz)) * hit_scale)) - lantern_buf[sel_payload].position;
                        bool occluded2;
                        if (abs(to_light2.y) > 0.5f)
                        {
                            occluded2 = true;
                        }
                        else
                        {
                            float2 xz = to_light2.xz;
                            occluded2 = dot(xz, xz) > 0.136900007724761962890625f;
                        }
                        if (occluded2)
                        {
                            valid = false;
                            break;
                        }
                    }
                }
                valid = true;
                break;
            } while (false);
            occluded = !valid;
        }
        else
        {
            occluded = false;
        }

        uint payload_out = occluded ? 0u : sel_payload;
        float w_sum_out = occluded ? 0.0f : w_sum;

        bool have_history;
        if (history_valid != 0u)
        {
            have_history = frame_index != 0u;
        }
        else
        {
            have_history = false;
        }

        uint payload_final;
        float w_sum_final;
        float acc_w_final;
        float m_sum_final;
        if (have_history)
        {
            uint payload_h = payload_out;
            float w_sum_h = w_sum_out;
            float acc_w_h = acc_w;
            float m_sum_h = m_sum;
            uint rand_exp_h = rand_exp;
            uint rand_mant_h = rand_mant;
            bool exp_seen_h = exp_seen;
            bool mant_seen_h = mant_seen;
            uint rng_h = rng_state;

            for (uint hist_i = 0u; hist_i < 4u; hist_i++)
            {
                uint rng_1 = rng_h;
                rng_1 += (rng_1 << 10u);
                rng_1 ^= (rng_1 >> 6u);
                rng_1 += (rng_1 << 3u);
                rng_1 ^= (rng_1 >> 11u);
                rng_1 += (rng_1 << 15u);

                bool mant_seen_1 = mant_seen_h ? mant_seen_h : true;
                uint mant_1 = mant_seen_h ? rand_mant_h : 8388607u;
                bool exp_seen_1 = exp_seen_h ? exp_seen_h : true;
                uint exp_1 = exp_seen_h ? rand_exp_h : 1065353216u;

                uint rng_2 = rng_1;
                rng_2 += (rng_2 << 10u);
                rng_2 ^= (rng_2 >> 6u);
                rng_2 += (rng_2 << 3u);
                rng_2 ^= (rng_2 >> 11u);
                rng_2 += (rng_2 << 15u);

                bool mant_seen_2 = mant_seen_1 ? mant_seen_1 : true;
                uint mant_2 = mant_seen_1 ? mant_1 : 8388607u;
                bool exp_seen_2 = exp_seen_1 ? exp_seen_1 : true;
                uint exp_2 = exp_seen_1 ? exp_1 : 1065353216u;

                bool first = 0u == hist_i;
                int2 jitter;
                if (first)
                {
                    jitter = int2(0, 0);
                }
                else
                {
                    jitter = int2(((float2(asfloat((rng_h & mant_1) | exp_1) - 1.0f, asfloat((rng_1 & mant_2) | exp_2) - 1.0f) * 2.0f) - 1.0f.xx) * 12.0f);
                }

                uint4 hist_texel = rdi_temporal_history_tex.Load(int3(uint2(jitter) + uint2((uv + reprojection_tex.Load(int3(gl_GlobalInvocationID.xy * uint2(2u, 2u), 0u)).xy) * float2(rdi_temporal_history_tex_dims)), 0u));
                uint hist_payload = hist_texel.x;
                uint hist_packed = hist_texel.y;
                float2 hist_weights = max(0.0f.xx, float2(unpack_half2x16(hist_packed & 65535u).x, unpack_half2x16((hist_packed >> 16u) & 65535u).x));
                float hist_w_sum = min(hist_weights.x, float(first ? 100 : 5));

                float hist_intensity = unpack_half2x16(lantern_buf[hist_payload].intensity__color_id & 65535u).x;
                uint hist_color_id = lantern_buf[hist_payload].intensity__color_id >> 16u;
                float3 to_hist = lantern_buf[hist_payload].position - world_pos;
                float hist_dist2 = dot(to_hist, to_hist);
                float3 up_hist = to_hist + float3(0.0f, 0.1500000059604644775390625f, 0.0f);

                float3 hist_color;
                if (hist_color_id == 0u)
                {
                    hist_color = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                }
                else if (hist_color_id == 1u)
                {
                    hist_color = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                }
                else if (hist_color_id == 2u)
                {
                    hist_color = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                }
                else if (hist_color_id == 3u)
                {
                    hist_color = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
                }
                else
                {
                    hist_color = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
                }

                float hist_w = dot(((hist_color * 4.0f) * hist_intensity) * ((max(0.0f, dot(world_normal, to_hist / length(to_hist).xxx)) * ((1.0f / (0.3359999954700469970703125f + hist_dist2)) * smoothstep(36.0f * hist_intensity, 16.0f * hist_intensity, hist_dist2))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (up_hist / length(up_hist).xxx).y)), LUMA);

                uint rng_3 = rng_2;
                rng_3 += (rng_3 << 10u);
                rng_3 ^= (rng_3 >> 6u);
                rng_3 += (rng_3 << 3u);
                rng_3 ^= (rng_3 >> 11u);
                rng_3 += (rng_3 << 15u);

                bool mant_seen_3 = mant_seen_2 ? mant_seen_2 : true;
                uint mant_3 = mant_seen_2 ? mant_2 : 8388607u;
                bool exp_seen_3 = exp_seen_2 ? exp_seen_2 : true;
                uint exp_3 = exp_seen_2 ? exp_2 : 1065353216u;

                float next_m_sum_h = m_sum_h + hist_w_sum;
                float w_h = (hist_w * hist_weights.y) * hist_w_sum;
                float next_w_sum_h = w_sum_h + w_h;
                uint next_payload_h;
                float next_acc_w_h;
                if ((w_h / next_w_sum_h) >= (asfloat((rng_2 & mant_3) | exp_3) - 1.0f))
                {
                    next_payload_h = hist_payload;
                    next_acc_w_h = hist_w;
                }
                else
                {
                    next_payload_h = payload_h;
                    next_acc_w_h = acc_w_h;
                }

                m_sum_h = next_m_sum_h;
                rand_mant_h = mant_3;
                rand_exp_h = exp_3;
                mant_seen_h = mant_seen_3;
                exp_seen_h = exp_seen_3;
                rng_h = rng_3;
                payload_h = next_payload_h;
                w_sum_h = next_w_sum_h;
                acc_w_h = next_acc_w_h;
            }
            payload_final = payload_h;
            w_sum_final = w_sum_h;
            acc_w_final = acc_w_h;
            m_sum_final = m_sum_h;
        }
        else
        {
            payload_final = payload_out;
            w_sum_final = w_sum_out;
            acc_w_final = acc_w;
            m_sum_final = m_sum;
        }

        rdi_temporal_out_tex[gl_GlobalInvocationID.xy] = uint2(payload_final, pack_half2x16(float2(m_sum_final, 0.0f)) | (pack_half2x16(float2(min(w_sum_final / max(9.9999999392252902907785028219223e-09f, m_sum_final * acc_w_final), 1.0f), 0.0f)) << 16u));
        break;
    } while (false);
}
