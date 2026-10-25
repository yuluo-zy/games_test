#define SHADER_VARIANT_DEPTH_ONLY 0
#line 1 "shaders/final_post.raster.hlsl"








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
#line 9 "shaders/final_post.raster.hlsl"
            
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
#line 10 "shaders/final_post.raster.hlsl"

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
#line 11 "shaders/final_post.raster.hlsl"

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
#line 12 "shaders/final_post.raster.hlsl"

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
#line 13 "shaders/final_post.raster.hlsl"

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
#line 14 "shaders/final_post.raster.hlsl"

#line 1 "shaders/inc/image.hlsl"




#ifndef TINYGLADE_INC_IMAGE_HLSL
#define TINYGLADE_INC_IMAGE_HLSL

#line 1 "shaders/inc/curve.hlsl"















#define frexp incb_frexp

struct FrexpStructType
{
    float3 mantissa;
    int3 exponent;
};

float3 incb_frexp(float3 x, out int3 exponent)
{
    uint3 bits = asuint(x);
    int3 biased = int3(int((bits.x >> 23u) & 0xFFu),
                       int((bits.y >> 23u) & 0xFFu),
                       int((bits.z >> 23u) & 0xFFu));
    int3 e = biased - int3(127, 127, 127);
    float3 m = asfloat((bits & 0x807FFFFFu) | 0x3F800000u);

    
    bool3 big = (m >= 1.0f) | (m <= -1.0f);
    m = lerp(m, m * 0.5f, float3(big));
    e += int3(big);

    
    bool3 special = (biased == 0) | (biased == 0xFF);
    exponent = select(special, int3(0, 0, 0), e);
    return select(special, x, m);
}

float3 curve(float3 x)
{
    
    
    
    FrexpStructType f;
    int3 e;
    f.mantissa = frexp(x, e);
    f.exponent = e;
    return x + exp2(float3(f.exponent) - float3(8.0f, 8.0f, 7.0f));
}
#line 8 "shaders/inc/image.hlsl"



float2 texture_dims(Texture2D<float4> tex)
{
    uint w, h;
    tex.GetDimensions(w, h);
    return float2(w, h);
}


float sample_mask(Texture2D<float4> mask_tex, SamplerState samp, float2 uv)
{
    return mask_tex.SampleLevel(samp, uv, 0.0f).r;
}

#endif 
#line 15 "shaders/final_post.raster.hlsl"

#line 16 "shaders/final_post.raster.hlsl"

#line 1 "shaders/inc/bicubic.hlsl"




#ifndef TINYGLADE_INC_BICUBIC_HLSL
#define TINYGLADE_INC_BICUBIC_HLSL




float4 bicubic_weights(float f)
{
    float t = f;
    float t_half = t * 0.5f;
    float w0 = t * (-0.5f + t * (1.0f - t_half));
    float w1 = t * (0.5f + t * (2.0f - t_half));
    float one_minus_t = 1.0f - t;
    float w3 = one_minus_t * (-0.5f + t_half);
    float w2 = (1.0f + one_minus_t * (-2.5f + t * (2.0f - t_half))) + w3;
    return float4(w0, w1, w2, w3);
}


float4 bicubic_sample(Texture2D<float4> tex, SamplerState samp, float2 uv, float2 texel_size)
{
    float2 p = uv * (float2(1.0f, 1.0f) / texel_size) - 0.5f;
    float2 f = frac(p);
    float2 base = (p - f) * texel_size;
    float4 wx = bicubic_weights(f.x);
    float4 wy = bicubic_weights(f.y);

    float4 c0 = wy.x * tex.SampleLevel(samp, base + float2(-1.0f, -1.0f) * texel_size, 0.0f);
    float4 c1 = wy.y * tex.SampleLevel(samp, base + float2( 0.0f, -1.0f) * texel_size, 0.0f);
    float4 c2 = wy.z * tex.SampleLevel(samp, base + float2( 1.0f, -1.0f) * texel_size, 0.0f);
    float4 c3 = wy.w * tex.SampleLevel(samp, base + float2( 2.0f, -1.0f) * texel_size, 0.0f);
    float4 c01 = wy.x * tex.SampleLevel(samp, base + float2(-1.0f,  0.0f) * texel_size, 0.0f);
    float4 c11 = wy.y * tex.SampleLevel(samp, base + float2( 0.0f,  0.0f) * texel_size, 0.0f);
    float4 c21 = wy.z * tex.SampleLevel(samp, base + float2( 1.0f,  0.0f) * texel_size, 0.0f);
    float4 c31 = wy.w * tex.SampleLevel(samp, base + float2( 2.0f,  0.0f) * texel_size, 0.0f);
    float4 c02 = wy.x * tex.SampleLevel(samp, base + float2(-1.0f,  1.0f) * texel_size, 0.0f);
    float4 c12 = wy.y * tex.SampleLevel(samp, base + float2( 0.0f,  1.0f) * texel_size, 0.0f);
    float4 c22 = wy.z * tex.SampleLevel(samp, base + float2( 1.0f,  1.0f) * texel_size, 0.0f);
    float4 c32 = wy.w * tex.SampleLevel(samp, base + float2( 2.0f,  1.0f) * texel_size, 0.0f);
    float4 c03 = wy.x * tex.SampleLevel(samp, base + float2(-1.0f,  2.0f) * texel_size, 0.0f);
    float4 c13 = wy.y * tex.SampleLevel(samp, base + float2( 0.0f,  2.0f) * texel_size, 0.0f);
    float4 c23 = wy.z * tex.SampleLevel(samp, base + float2( 1.0f,  2.0f) * texel_size, 0.0f);
    float4 c33 = wy.w * tex.SampleLevel(samp, base + float2( 2.0f,  2.0f) * texel_size, 0.0f);

    float4 col0 = (c0 + c1 + c2 + c3) * wx.x
                + (c01 + c11 + c21 + c31) * wx.y
                + (c02 + c12 + c22 + c32) * wx.z
                + (c03 + c13 + c23 + c33) * wx.w;
    return col0;
}

#endif 
#line 17 "shaders/final_post.raster.hlsl"

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
#line 18 "shaders/final_post.raster.hlsl"

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
#line 19 "shaders/final_post.raster.hlsl"

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
#line 20 "shaders/final_post.raster.hlsl"

#line 1 "shaders/inc/grading_lut.hlsl"


#ifndef TINYGLADE_INC_GRADING_LUT_HLSL
#define TINYGLADE_INC_GRADING_LUT_HLSL


float3 sample_grading_lut(Texture3D lut, SamplerState s, float3 color)
{
    return lut.SampleLevel(s, saturate(color), 0.0).xyz;
}


float3 apply_tony_mc_mapface(Texture3D lut, SamplerState s, float3 color)
{
    return sample_grading_lut(lut, s, color);
}


float3 apply_grading_lut(Texture3D lut, SamplerState s, float3 color, float amount)
{
    float3 graded = sample_grading_lut(lut, s, color);
    return lerp(color, graded, saturate(amount));
}

#endif 
#line 21 "shaders/final_post.raster.hlsl"

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
#line 22 "shaders/final_post.raster.hlsl"

#line 1 "shaders/inc/color/ycbcr.hlsl"





#ifndef TINYGLADE_INC_COLOR_YCBCR_HLSL
#define TINYGLADE_INC_COLOR_YCBCR_HLSL

float3 rgb_to_ycbcr(float3 rgb)
{
    return mul(
        float3x3(
            float3( 0.2126f,  0.7152f,  0.0722f),
            float3(-0.1146f, -0.3854f,  0.5f),
            float3( 0.5f,   -0.4542f, -0.0458f)),
        rgb);
}

float3 ycbcr_to_rgb(float3 ycbcr)
{
    
    return mul(
        float3x3(
            float3(1.0f,        1.0f,       1.0f),
            float3(0.0f,       -0.18732f,   1.8556f),
            float3(1.5748f,    -0.46812f,   0.0f)),
        ycbcr);
}

#endif 
#line 23 "shaders/final_post.raster.hlsl"

#line 1 "shaders/inc/color/xyz.hlsl"




#ifndef TINYGLADE_INC_COLOR_XYZ_HLSL
#define TINYGLADE_INC_COLOR_XYZ_HLSL

static const float3 XYZ_D65_WHITE = float3(0.95047f, 1.0f, 1.08883f);

float3 srgb_xyz_to_xyz(float3 c)
{
    return mul(
        float3x3(
            float3(0.4124564f, 0.3575761f, 0.1804375f),
            float3(0.2126729f, 0.7151522f, 0.0721750f),
            float3(0.0193339f, 0.1191920f, 0.9503041f)),
        c);
}

float3 xyz_to_srgb_xyz(float3 xyz)
{
    return mul(
        float3x3(
            float3( 3.2404542f, -1.5371385f, -0.4985314f),
            float3(-0.9692660f,  1.8760108f,  0.0415560f),
            float3( 0.0556434f, -0.2040259f,  1.0572252f)),
        xyz);
}

#endif 
#line 24 "shaders/final_post.raster.hlsl"

#line 1 "shaders/inc/color/luv.hlsl"




#ifndef TINYGLADE_INC_COLOR_LUV_HLSL
#define TINYGLADE_INC_COLOR_LUV_HLSL

#line 1 "shaders/inc/color/xyz.hlsl"




#ifndef TINYGLADE_INC_COLOR_XYZ_HLSL
#define TINYGLADE_INC_COLOR_XYZ_HLSL

static const float3 XYZ_D65_WHITE = float3(0.95047f, 1.0f, 1.08883f);

float3 srgb_xyz_to_xyz(float3 c)
{
    return mul(
        float3x3(
            float3(0.4124564f, 0.3575761f, 0.1804375f),
            float3(0.2126729f, 0.7151522f, 0.0721750f),
            float3(0.0193339f, 0.1191920f, 0.9503041f)),
        c);
}

float3 xyz_to_srgb_xyz(float3 xyz)
{
    return mul(
        float3x3(
            float3( 3.2404542f, -1.5371385f, -0.4985314f),
            float3(-0.9692660f,  1.8760108f,  0.0415560f),
            float3( 0.0556434f, -0.2040259f,  1.0572252f)),
        xyz);
}

#endif 
#line 8 "shaders/inc/color/luv.hlsl"


static const float LUV_EPSILON = 216.0f / 24389.0f;  
static const float LUV_KAPPA = 24389.0f / 27.0f;     


static const float LUV_UN = (4.0f * XYZ_D65_WHITE.x) / (XYZ_D65_WHITE.x + 15.0f * XYZ_D65_WHITE.y + 3.0f * XYZ_D65_WHITE.z);
static const float LUV_VN = (9.0f * XYZ_D65_WHITE.y) / (XYZ_D65_WHITE.x + 15.0f * XYZ_D65_WHITE.y + 3.0f * XYZ_D65_WHITE.z);

float3 xyz_to_luv(float3 xyz)
{
    float y_ratio = xyz.y / XYZ_D65_WHITE.y;
    float L = y_ratio > LUV_EPSILON
        ? 116.0f * pow(y_ratio, 1.0f / 3.0f) - 16.0f
        : LUV_KAPPA * y_ratio;

    float denom = xyz.x + 15.0f * xyz.y + 3.0f * xyz.z;
    float u_p = 4.0f * xyz.x / max(denom, 1.0e-5f);
    float v_p = 9.0f * xyz.y / max(denom, 1.0e-5f);

    return float3(L, 13.0f * L * (u_p - LUV_UN), 13.0f * L * (v_p - LUV_VN));
}

float3 luv_to_xyz(float3 luv)
{
    float L = luv.x;
    float u = luv.y;
    float v = luv.z;

    float u_p = (L > 8.0f ? u / (13.0f * L) : 0.0f) + LUV_UN;
    float v_p = (L > 8.0f ? v / (13.0f * L) : 0.0f) + LUV_VN;

    float Y = L > 8.0f
        ? XYZ_D65_WHITE.y * pow((L + 16.0f) / 116.0f, 3.0f)
        : XYZ_D65_WHITE.y * (L / LUV_KAPPA);

    float denom = 4.0f * v_p;
    float X = Y * (9.0f * u_p) / max(denom, 1.0e-5f);
    float Z = Y * (12.0f - 3.0f * u_p - 20.0f * v_p) / max(denom, 1.0e-5f);

    return float3(X, Y, Z);
}

#endif 
#line 25 "shaders/final_post.raster.hlsl"

#line 1 "shaders/inc/color/helmholtz_kohlrausch.hlsl"





#ifndef TINYGLADE_INC_COLOR_HELMHOLTZ_KOHLRAUSCH_HLSL
#define TINYGLADE_INC_COLOR_HELMHOLTZ_KOHLRAUSCH_HLSL

#line 1 "shaders/inc/color/luv.hlsl"




#ifndef TINYGLADE_INC_COLOR_LUV_HLSL
#define TINYGLADE_INC_COLOR_LUV_HLSL

#line 1 "shaders/inc/color/xyz.hlsl"




#ifndef TINYGLADE_INC_COLOR_XYZ_HLSL
#define TINYGLADE_INC_COLOR_XYZ_HLSL

static const float3 XYZ_D65_WHITE = float3(0.95047f, 1.0f, 1.08883f);

float3 srgb_xyz_to_xyz(float3 c)
{
    return mul(
        float3x3(
            float3(0.4124564f, 0.3575761f, 0.1804375f),
            float3(0.2126729f, 0.7151522f, 0.0721750f),
            float3(0.0193339f, 0.1191920f, 0.9503041f)),
        c);
}

float3 xyz_to_srgb_xyz(float3 xyz)
{
    return mul(
        float3x3(
            float3( 3.2404542f, -1.5371385f, -0.4985314f),
            float3(-0.9692660f,  1.8760108f,  0.0415560f),
            float3( 0.0556434f, -0.2040259f,  1.0572252f)),
        xyz);
}

#endif 
#line 8 "shaders/inc/color/luv.hlsl"


static const float LUV_EPSILON = 216.0f / 24389.0f;  
static const float LUV_KAPPA = 24389.0f / 27.0f;     


static const float LUV_UN = (4.0f * XYZ_D65_WHITE.x) / (XYZ_D65_WHITE.x + 15.0f * XYZ_D65_WHITE.y + 3.0f * XYZ_D65_WHITE.z);
static const float LUV_VN = (9.0f * XYZ_D65_WHITE.y) / (XYZ_D65_WHITE.x + 15.0f * XYZ_D65_WHITE.y + 3.0f * XYZ_D65_WHITE.z);

float3 xyz_to_luv(float3 xyz)
{
    float y_ratio = xyz.y / XYZ_D65_WHITE.y;
    float L = y_ratio > LUV_EPSILON
        ? 116.0f * pow(y_ratio, 1.0f / 3.0f) - 16.0f
        : LUV_KAPPA * y_ratio;

    float denom = xyz.x + 15.0f * xyz.y + 3.0f * xyz.z;
    float u_p = 4.0f * xyz.x / max(denom, 1.0e-5f);
    float v_p = 9.0f * xyz.y / max(denom, 1.0e-5f);

    return float3(L, 13.0f * L * (u_p - LUV_UN), 13.0f * L * (v_p - LUV_VN));
}

float3 luv_to_xyz(float3 luv)
{
    float L = luv.x;
    float u = luv.y;
    float v = luv.z;

    float u_p = (L > 8.0f ? u / (13.0f * L) : 0.0f) + LUV_UN;
    float v_p = (L > 8.0f ? v / (13.0f * L) : 0.0f) + LUV_VN;

    float Y = L > 8.0f
        ? XYZ_D65_WHITE.y * pow((L + 16.0f) / 116.0f, 3.0f)
        : XYZ_D65_WHITE.y * (L / LUV_KAPPA);

    float denom = 4.0f * v_p;
    float X = Y * (9.0f * u_p) / max(denom, 1.0e-5f);
    float Z = Y * (12.0f - 3.0f * u_p - 20.0f * v_p) / max(denom, 1.0e-5f);

    return float3(X, Y, Z);
}

#endif 
#line 9 "shaders/inc/color/helmholtz_kohlrausch.hlsl"




float helmholtz_kohlrausch_lightness(float3 luv)
{
    float L = luv.x;
    float chroma = length(luv.yz);
    return L + (0.3f - 0.15f * L) * chroma;
}


float helmholtz_kohlrausch_perceived_lightness(float3 linear_rgb)
{
    float3 luv = xyz_to_luv(srgb_xyz_to_xyz(linear_rgb));
    return helmholtz_kohlrausch_lightness(luv);
}

#endif 
#line 26 "shaders/final_post.raster.hlsl"

#line 1 "shaders/inc/color/math.hlsl"




#ifndef TINYGLADE_INC_COLOR_MATH_HLSL
#define TINYGLADE_INC_COLOR_MATH_HLSL


float luma(float3 rgb)
{
    return dot(rgb, float3(0.2126f, 0.7152f, 0.0722f));
}

#endif 
#line 27 "shaders/final_post.raster.hlsl"

#line 1 "shaders/inc/upsample/easu.hlsl"





#ifndef TINYGLADE_INC_UPSAMPLE_EASU_HLSL
#define TINYGLADE_INC_UPSAMPLE_EASU_HLSL




float3 easu_sample(Texture2D<float4> tex, SamplerState samp,
                   float2 pos, float2 input_dims, float2 output_dims)
{
    
    float2 scale = input_dims / output_dims;
    float2 src = (pos + 0.5f) * scale - 0.5f;
    float2 base = floor(src);
    float2 t = src - base;               
    float2 t_inv = 1.0f - t;

    float2 pixel = float2(1.0f, 1.0f) / input_dims;

    
    float3 p[4][4];
    for (int j = 0; j < 4; j++)
    {
        for (int i = 0; i < 4; i++)
        {
            float2 c = (base + float2(float(i) - 1.0f, float(j) - 1.0f)) * pixel;
            p[j][i] = tex.SampleLevel(samp, c, 0.0f).xyz;
        }
    }

    
    float l[4][4];
    for (int j = 0; j < 4; j++)
        for (int i = 0; i < 4; i++)
            l[j][i] = p[j][i].y + 0.5f * (p[j][i].x + p[j][i].z);

    
    float g1 = l[1][1] - l[1][0];  
    float g2 = l[1][2] - l[1][1];
    float g3 = l[0][1] - l[0][0];  
    float g4 = l[0][2] - l[0][1];
    float g5 = l[2][1] - l[2][0];
    float g6 = l[2][2] - l[2][1];
    float g7 = l[1][1] - l[0][1];  
    float g8 = l[1][2] - l[0][2];

    
    float b = clamp(abs(g2 - g1) / max(max(abs(g1), abs(g2)), 1.0e-6f), 0.0f, 1.0f);
    float c = clamp(abs(g4 - g3) / max(max(abs(g3), abs(g4)), 1.0e-6f), 0.0f, 1.0f);
    float d = clamp(abs(g6 - g5) / max(max(abs(g5), abs(g6)), 1.0e-6f), 0.0f, 1.0f);
    float e = clamp(abs(g8 - g7) / max(max(abs(g7), abs(g8)), 1.0e-6f), 0.0f, 1.0f);

    
    float2 xw = float2(t_inv.x, t.x);
    float2 yw = float2(t_inv.y, t.y);

    
    float3 result = xw.x * yw.x * p[1][1] + xw.y * yw.x * p[1][2]
                  + xw.x * yw.y * p[2][1] + xw.y * yw.y * p[2][2];

    result += xw.x * yw.x * (p[1][0] * (1.0f - b) + p[0][1] * (1.0f - c));
    result += xw.y * yw.x * (p[1][3] * (1.0f - b) + p[0][2] * (1.0f - d));
    result += xw.x * yw.y * (p[2][0] * (1.0f - e) + p[3][1] * (1.0f - c));
    result += xw.y * yw.y * (p[2][3] * (1.0f - e) + p[3][2] * (1.0f - d));

    return result;
}

#endif 
#line 28 "shaders/final_post.raster.hlsl"

#line 1 "shaders/inc/upsample/lanczos2.hlsl"




#ifndef TINYGLADE_INC_UPSAMPLE_LANCZOS2_HLSL
#define TINYGLADE_INC_UPSAMPLE_LANCZOS2_HLSL

static const float LANCZOS2_TWO = 2.0f;


float lanczos2_kernel(float x)
{
    float ax = abs(x);
    if (ax < 1.0e-6f)
        return 1.0f;
    if (ax >= LANCZOS2_TWO)
        return 0.0f;
    float px = 3.14159265358979323846f * x;
    float px_half = px * 0.5f;
    return (sin(px) / px) * (sin(px_half) / px_half);
}


float3 lanczos2_sample(Texture2D<float4> tex, SamplerState samp,
                       float2 uv, float2 texel_size)
{
    float2 p = uv / texel_size - 0.5f;
    float2 base = floor(p);
    float2 f = p - base;

    float3 result = float3(0.0f, 0.0f, 0.0f);
    float wsum = 0.0f;
    for (int j = -1; j <= 2; j++)
    {
        float wy = lanczos2_kernel(f.y - float(j));
        for (int i = -1; i <= 2; i++)
        {
            float wx = lanczos2_kernel(f.x - float(i));
            float w = wx * wy;
            float2 c = (base + float2(float(i), float(j))) * texel_size;
            result += w * tex.SampleLevel(samp, c, 0.0f).xyz;
            wsum += w;
        }
    }
    return result / max(wsum, 1.0e-6f);
}

#endif 
#line 29 "shaders/final_post.raster.hlsl"

#line 1 "shaders/fullscreen_triangle_vert.inc.hlsl"





#ifndef TINYGLADE_FULLSCREEN_TRIANGLE_VERT_INC_HLSL
#define TINYGLADE_FULLSCREEN_TRIANGLE_VERT_INC_HLSL

struct VsToPs
{
    float4 position : SV_Position;
    float2 uv : TEXCOORD0;
};

VsToPs vs_main(uint vertex_id : SV_VertexID)
{
    VsToPs o;

    if (vertex_id == 0u)
    {
        o.position = float4(-1.0f, -1.0f, 0.0f, 1.0f);
        o.uv = float2(0.0f, 1.0f);
    }
    else if (vertex_id == 1u)
    {
        o.position = float4(3.0f, -1.0f, 0.0f, 1.0f);
        o.uv = float2(2.0f, 1.0f);
    }
    else
    {
        o.position = float4(-1.0f, 3.0f, 0.0f, 1.0f);
        o.uv = float2(0.0f, -1.0f);
    }

    return o;
}

#endif 
#line 30 "shaders/final_post.raster.hlsl"


cbuffer $Globals : register(b13, space0)
{
    uint2 input_tex_dims;
    uint2 blur_pyramid_tex_dims;
    uint2 grading_lut_tex_dims;
    uint2 icc_lut_dims;
    uint2 blue_noise_texture_dims;
    uint is_input_valid;
    uint use_icc;
    float sharpen_amount;
    uint upsampling_mode;
    uint grading_flags;
    float grading_strength;
    float grading_contrast;
    float grading_saturation;
    float vignette_amount;
    float distortion_amount;
    float ca_amount;
    float gamma_shift;
    uint2 framebuffer_dims;
};

Texture2D<float4> input_tex : register(t14, space0);
Texture2D<float4> blur_pyramid_tex : register(t15, space0);
Texture3D<float4> grading_lut_tex : register(t16, space0);
Texture3D<float4> icc_lut : register(t17, space0);
Texture2D<float4> blue_noise_texture : register(t18, space0);
ByteAddressBuffer shadow_mask_mean : register(t19, space0);
Texture3D<float4> tony_mc_mapface_lut : register(t20, space0);



static const float3 debug_ramp[3] = {
    float3(0.8631572723388671875f, 0.06301002204418182373046875f, 0.15896083414554595947265625f),
    float3(0.9911022186279296875f, 0.6444797515869140625f, 0.13286833465099334716796875f),
    float3(0.02028855867683887481689453125f, 0.304987370967864990234375f, 0.520995616912841796875f),
};



float3 bicubic_upsample(int2 pixel, float2 output_dims)
{
    float2 input_dims = float2(input_tex_dims);
    float2 uv = (float2(pixel) + 0.5f.xx) / output_dims;
    float2 src = uv * input_dims;
    float2 base = floor(src - 0.5f.xx);
    float2 f = src - (base + 0.5f.xx);

    float2 w0 = f * (-0.5f + (f * (1.0f - (f * 0.5f))));
    float2 w1 = f * (0.5f + (f * (2.0f - (f * 1.5f))));
    float2 w2 = (1.0f + ((f * f) * (-2.5f + (f * 1.5f)))) + w1;
    float2 w3 = (f * f) * (-0.5f + (f * 0.5f));

    float2 c_left = (base - 0.5f.xx) / input_dims;
    float2 c_right = (base + 2.5f.xx) / input_dims;
    float2 c_center = (base + 0.5f.xx + (w1 / w2)) / input_dims;

    float3 t1 = input_tex.SampleLevel(sampler_lnc, float2(c_center.x, c_left.y), 0.0f).xyz;
    float3 t2 = input_tex.SampleLevel(sampler_lnc, float2(c_left.x, c_center.y), 0.0f).xyz;
    float3 t3 = input_tex.SampleLevel(sampler_lnc, c_center, 0.0f).xyz;
    float3 t4 = input_tex.SampleLevel(sampler_lnc, float2(c_right.x, c_center.y), 0.0f).xyz;
    float3 t5 = input_tex.SampleLevel(sampler_lnc, float2(c_center.x, c_right.y), 0.0f).xyz;

    float3 num = (t1 * (w2.x * w0.y)) + (t2 * (w0.x * w2.y)) + (t3 * (w2.x * w2.y)) +
                 (t4 * (w3.x * w2.y)) + (t5 * (w2.x * w3.y));
    float den = (w2.x * w0.y) + (w0.x * w2.y) + (w2.x * w2.y) + (w3.x * w2.y) + (w2.x * w3.y);
    return max(0.0f.xxx, num / den.xxx);
}



float easu_tap_weight(float2 off, float2 g2n, float2 perp, float2 span, float len_inv_recip, float len_inv)
{
    float2 v = float2(dot(off, g2n), dot(off, perp)) * span;
    float t = min(dot(v, v), len_inv);
    float a = (0.4000000059604644775390625f * t) - 1.0f;
    float b = (len_inv_recip * t) - 1.0f;
    return ((1.5625f * (a * a)) - 0.5625f) * (b * b);
}



float3 easu_upsample(int2 pixel, float2 output_dims)
{
    float2 input_dims = float2(input_tex_dims);
    float4 coef = float4(input_dims.x / output_dims.x, input_dims.y / output_dims.y,
                         (0.5f * input_dims.x) / output_dims.x - 0.5f, (0.5f * input_dims.y) / output_dims.y - 0.5f);
    float4 div_dims = float4(1.0f, 1.0f, 1.0f, -1.0f) / input_dims.xyxy;
    float4 add_dims = float4(-1.0f, 2.0f, 1.0f, 2.0f) / input_dims.xyxy;

    float2 src = (float2(pixel) * coef.xy) + coef.zw;
    float2 base = floor(src);
    float2 f = src - base;

    float2 t0 = (base * div_dims.xy) + div_dims.zw;
    float2 t1 = t0 + add_dims.xy;
    float2 t2 = t0 + add_dims.zw;
    float2 t3 = t0 + (float4(0.0f, 4.0f, 0.0f, 0.0f) / input_dims.xyxy).xy;
    float4 tap_off = float4(-0.5f, 0.5f, -0.5f, 0.5f) * div_dims.xxyy;
    float2 off_xw = float2(tap_off.x, tap_off.w);
    float2 off_yw = float2(tap_off.y, tap_off.w);
    float2 off_yz = float2(tap_off.y, tap_off.z);
    float2 off_xz = float2(tap_off.x, tap_off.z);

    float4 a1 = input_tex.SampleLevel(sampler_lnc, t0 + off_xw, 0.0f);
    float4 b1 = input_tex.SampleLevel(sampler_lnc, t0 + off_yw, 0.0f);
    float4 c1 = input_tex.SampleLevel(sampler_lnc, t1 + off_xw, 0.0f);
    float4 c2 = input_tex.SampleLevel(sampler_lnc, t1 + off_yw, 0.0f);
    float4 c3 = input_tex.SampleLevel(sampler_lnc, t1 + off_yz, 0.0f);
    float4 c4 = input_tex.SampleLevel(sampler_lnc, t1 + off_xz, 0.0f);
    float4 d1 = input_tex.SampleLevel(sampler_lnc, t2 + off_xw, 0.0f);
    float4 d2 = input_tex.SampleLevel(sampler_lnc, t2 + off_yw, 0.0f);
    float4 d3 = input_tex.SampleLevel(sampler_lnc, t2 + off_yz, 0.0f);
    float4 d4 = input_tex.SampleLevel(sampler_lnc, t2 + off_xz, 0.0f);
    float4 e1 = input_tex.SampleLevel(sampler_lnc, t3 + off_yz, 0.0f);
    float4 e2 = input_tex.SampleLevel(sampler_lnc, t3 + off_xz, 0.0f);

    float la1 = a1.y + (0.5f * (a1.x + a1.z));
    float lb1 = b1.y + (0.5f * (b1.x + b1.z));
    float lc1 = c1.y + (0.5f * (c1.x + c1.z));
    float lc2 = c2.y + (0.5f * (c2.x + c2.z));
    float lc3 = c3.y + (0.5f * (c3.x + c3.z));
    float lc4 = c4.y + (0.5f * (c4.x + c4.z));
    float ld1 = d1.y + (0.5f * (d1.x + d1.z));
    float ld2 = d2.y + (0.5f * (d2.x + d2.z));
    float ld3 = d3.y + (0.5f * (d3.x + d3.z));
    float ld4 = d4.y + (0.5f * (d4.x + d4.z));
    float le1 = e1.y + (0.5f * (e1.x + e1.z));
    float le2 = e2.y + (0.5f * (e2.x + e2.z));

    float fx = f.x;
    float fy = f.y;
    float fx1 = 1.0f - fx;
    float fy1 = 1.0f - fy;
    float w00 = fx1 * fy1;
    float w01 = fx * fy1;
    float w10 = fx1 * fy;
    float w11 = fx * fy;

    float g0 = abs(ld4 - lc3);
    float g1 = ld4 - ld1;
    float e_c = clamp(abs(g1) / max(g0, abs(lc3 - ld1)), 0.0f, 1.0f);
    float g2 = abs(lc2 - lc3);
    float g3 = lc2 - la1;
    float e_b = clamp(abs(g3) / max(g2, abs(lc3 - la1)), 0.0f, 1.0f);
    float g4 = ld3 - lc3;
    float e_d = clamp(abs(g4) / max(abs(ld3 - ld4), g0), 0.0f, 1.0f);
    float g5 = abs(ld1 - ld4);
    float g6 = ld1 - lb1;
    float e_f = clamp(abs(g6) / max(g5, abs(ld4 - lb1)), 0.0f, 1.0f);
    float g7 = abs(ld1 - lc2);
    float g8 = ld1 - lc1;
    float e_a = clamp(abs(g8) / max(g7, abs(lc2 - lc1)), 0.0f, 1.0f);
    float g9 = le2 - lc3;
    float e_g = clamp(abs(g9) / max(abs(le2 - lc2), g2), 0.0f, 1.0f);
    float g10 = ld2 - lc2;
    float gx = (g1 * w00) + (g4 * w01) + (g8 * w10) + (g10 * w11);
    float e_e = clamp(abs(g10) / max(abs(ld2 - ld1), g7), 0.0f, 1.0f);
    float g11 = le1 - ld4;
    float gy = (g3 * w00) + (g6 * w01) + (g9 * w10) + (g11 * w11);
    float e_h = clamp(abs(g11) / max(abs(le1 - ld1), g5), 0.0f, 1.0f);

    float2 gv = float2(gx, gy);
    float g_len2 = dot(gv, gv);
    bool is_zero = g_len2 < 3.0517578125e-05f;
    float2 g2n = float2(is_zero ? 1.0f : gx, gy) * (is_zero ? 1.0f : rsqrt(g_len2)).xx;

    float e_avg = ((w00 * ((e_c * e_c) + (e_b * e_b))) + (w01 * ((e_d * e_d) + (e_f * e_f))) +
                   (w10 * ((e_a * e_a) + (e_g * e_g))) + (w11 * ((e_e * e_e) + (e_h * e_h)))) * 0.5f;
    float e_avg2 = e_avg * e_avg;
    float2 span = float2(1.0f + (((dot(g2n, g2n) / max(abs(g2n.x), abs(g2n.y))) - 1.0f) * e_avg2), 1.0f - (0.5f * e_avg2));
    float len_inv_recip = 0.5f - (0.2899999916553497314453125f * e_avg2);
    float len_inv = 1.0f / len_inv_recip;
    float2 perp = float2(-g2n.y, g2n.x);

    float w_a1 = easu_tap_weight(float2(0.0f, -1.0f) - f, g2n, perp, span, len_inv_recip, len_inv);
    float w_b1 = easu_tap_weight(float2(1.0f, -1.0f) - f, g2n, perp, span, len_inv_recip, len_inv);
    float w_c1 = easu_tap_weight(float2(-1.0f, 1.0f) - f, g2n, perp, span, len_inv_recip, len_inv);
    float w_c2 = easu_tap_weight(float2(0.0f, 1.0f) - f, g2n, perp, span, len_inv_recip, len_inv);
    float w_c3 = easu_tap_weight(-f, g2n, perp, span, len_inv_recip, len_inv);
    float w_c4 = easu_tap_weight(float2(-1.0f, 0.0f) - f, g2n, perp, span, len_inv_recip, len_inv);
    float w_d1 = easu_tap_weight(1.0f.xx - f, g2n, perp, span, len_inv_recip, len_inv);
    float w_d2 = easu_tap_weight(float2(2.0f, 1.0f) - f, g2n, perp, span, len_inv_recip, len_inv);
    float w_d3 = easu_tap_weight(float2(2.0f, 0.0f) - f, g2n, perp, span, len_inv_recip, len_inv);
    float w_d4 = easu_tap_weight(float2(1.0f, 0.0f) - f, g2n, perp, span, len_inv_recip, len_inv);
    float w_e1 = easu_tap_weight(float2(1.0f, 2.0f) - f, g2n, perp, span, len_inv_recip, len_inv);
    float w_e2 = easu_tap_weight(float2(0.0f, 2.0f) - f, g2n, perp, span, len_inv_recip, len_inv);

    float w_sum = w_a1 + w_b1 + w_c1 + w_c2 + w_c3 + w_c4 + w_d1 + w_d2 + w_d3 + w_d4 + w_e1 + w_e2;
    float3 weighted = a1.xyz * w_a1 + b1.xyz * w_b1 + c1.xyz * w_c1 + c2.xyz * w_c2 + c3.xyz * w_c3 + c4.xyz * w_c4 + d1.xyz * w_d1 + d2.xyz * w_d2 + d3.xyz * w_d3 + d4.xyz * w_d4 + e1.xyz * w_e1 + e2.xyz * w_e2;
    float3 result = max(min(min(c3.xyz, d4.xyz), min(c2.xyz, d1.xyz)), weighted / w_sum.xxx);
    return min(max(max(c3.xyz, d4.xyz), max(c2.xyz, d1.xyz)), result);
}



float2 final_post_sample_uv(float2 view_uv, float2 corner_uv, float r2, float corner_r2, float2 clip_scale, float amp)
{
    float2 distorted_pt = view_uv + (view_uv * (amp * r2));
    float2 distorted_corner = corner_uv + (corner_uv * (amp * corner_r2));
    float2 ndc = mul(view_constants[0].view_to_clip, float4(distorted_pt * clip_scale, 0.0f, 1.0f)).xy /
                 mul(view_constants[0].view_to_clip, float4(distorted_corner * clip_scale, 0.0f, 1.0f)).xy;
    return (ndc * float2(0.5f, -0.5f)) + 0.5f.xx;
}

float4 ps_main(float4 position : SV_Position) : SV_Target0
{
    int2 pixel = int2(position.xy);
    float2 output_dims = float2(framebuffer_dims);
    float2 uv = (float2(pixel) + 0.5f.xx) / output_dims;

    
    
    float3 upsampled;
    if (upsampling_mode == 0u)
    {
        upsampled = input_tex.SampleLevel(sampler_nnc, uv, 0.0f).xyz;
    }
    else if (upsampling_mode == 1u)
    {
        upsampled = input_tex.SampleLevel(sampler_lnc, uv, 0.0f).xyz;
    }
    else if (upsampling_mode == 2u)
    {
        upsampled = bicubic_upsample(pixel, output_dims);
    }
    else if (upsampling_mode == 3u)
    {
        upsampled = easu_upsample(pixel, output_dims);
    }
    else
    {
        float4 g = input_tex.GatherGreen(sampler_lnc, uv, int2(0, 0));
        float g_min = min(min(g.x, g.y), min(g.z, g.w));
        float g_max = max(max(g.x, g.y), max(g.z, g.w));
        if (WaveActiveAnyTrue((abs(g_min - g_max) / max(9.9999996826552253889678874634872e-21f, g_min + g_max)) > 0.5f))
        {
            upsampled = easu_upsample(pixel, output_dims);
        }
        else
        {
            upsampled = bicubic_upsample(pixel, output_dims);
        }
    }

    float2 blur_dims = float2(blur_pyramid_tex_dims);
    float blur_extent = max(blur_dims.x, blur_dims.y);
    float3 blur_mid = (blur_pyramid_tex.SampleLevel(sampler_lnc, uv, log2(0.0500000007450580596923828125f * blur_extent)).xyz * 0.119999997317790985107421875f) * 0.25f;

    float distortion = max(0.0f, distortion_amount);
    float3 blended;
    if (distortion > 0.0f)
    {
        float2 clip_scale = float2(view_constants[0].view_to_clip[1].y / view_constants[0].view_to_clip[0].x, 1.0f);
        float2 view_uv = mul(view_constants[0].clip_to_view, float4((uv - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f)).xy / clip_scale;
        float r2 = dot(view_uv, view_uv);
        float2 corner_uv = mul(view_constants[0].clip_to_view, float4(1.0f, 1.0f, 0.0f, 1.0f)).xy / clip_scale;
        float corner_r2 = dot(corner_uv, corner_uv);

        float ca_plus = distortion * (1.0f + ca_amount);
        float ca_minus = distortion * (1.0f - ca_amount);
        blended = float3(input_tex.SampleLevel(sampler_lnc, final_post_sample_uv(view_uv, corner_uv, r2, corner_r2, clip_scale, ca_plus), 0.0f).x,
                         input_tex.SampleLevel(sampler_lnc, final_post_sample_uv(view_uv, corner_uv, r2, corner_r2, clip_scale, distortion), 0.0f).y,
                         input_tex.SampleLevel(sampler_lnc, final_post_sample_uv(view_uv, corner_uv, r2, corner_r2, clip_scale, ca_minus), 0.0f).z);
    }
    else
    {
        float3 blur_low = blur_pyramid_tex.SampleLevel(sampler_llc, uv, log2(0.001000000047497451305389404296875f * blur_extent)).xyz * 0.119999997317790985107421875f;
        blended = max((((upsampled + blur_low) * 0.917431175708770751953125f.xxx) - lerp(blur_mid, dot(blur_mid, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)).xxx, 0.5f.xxx)), 0.0f.xxx);
    }

    float2 centered = uv - 0.5f.xx;
    float r2 = 3.5f * dot(centered, centered);
    float3 tonemapped = pow(max(0.0f.xxx, (blended * lerp(1.0f, 0.569999992847442626953125f, smoothstep(0.300000011920928955078125f, 1.0f, clamp(shadow_mask_mean.Load<float>(0), 0.0f, 1.0f)))) * exp((-vignette_amount) * (r2 * r2))),
                           max(0.0f, 1.10000002384185791015625f + gamma_shift).xxx) * 1.60000002384185791015625f;

    float3 mapface = max(0.0f.xxx, tony_mc_mapface_lut.SampleLevel(sampler_lnc, ((tonemapped / (tonemapped + 1.0f.xxx)) * 0.979166686534881591796875f) + 0.010416666977107524871826171875f.xxx, 0.0f).xyz);
    float3 base_color = (is_input_valid != 0u) ? mapface : 0.180000007152557373046875f.xxx;

    float3 graded_sat = clamp(lerp(dot(base_color, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)).xxx, base_color, grading_saturation.xxx), 0.0f.xxx, 1.0f.xxx);
    float graded_luma = dot(graded_sat, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
    float3 graded = graded_sat * (pow(graded_luma, grading_contrast) / max(9.9999997473787516355514526367188e-06f, graded_luma));

    float3 final_graded;
    if ((1u & grading_flags) != 0u)
    {
        if ((2u & grading_flags) != 0u)
        {
            if ((pixel.x < 256) && (pixel.y < 16))
            {
                float3 ramp = float3(float(pixel.x % 16), float(pixel.x / 16), float(15 - (pixel.y % 16))) * 0.066666670143604278564453125f;
                final_graded = srgb_to_linear(ramp);
            }
            else
            {
                final_graded = graded;
            }
        }
        else
        {
            final_graded = graded;
        }
    }
    else
    {
        float3 lut_uv = linear_to_srgb(graded) * (float(grading_lut_tex_dims.x - 1u).xxx / float(grading_lut_tex_dims.x).xxx) + (0.5f.xxx / float(grading_lut_tex_dims.x).xxx);
        float3 lut_color = srgb_to_linear(grading_lut_tex.SampleLevel(sampler_lnc, lut_uv, 0.0f).xyz);
        final_graded = lerp(graded, lut_color, grading_strength.xxx);
    }

    float dither = (blue_noise_texture.Load(int3(uint2((pixel + int2(int(frame_index * 59u), int(frame_index * 37u))) & int2(255, 255)), 0u)).x * 2.0f) - 1.0f;
    float3 dithered = clamp(final_graded + (clamp(final_graded * 512.0f, 0.0f.xxx, 1.0f.xxx) * ((max(-1.0f, dither * rsqrt(abs(dither))) - float(int(sign(dither)))) * 0.0039215688593685626983642578125f)), 0.0f.xxx, 1.0f.xxx);

    float3 output_color;
    if (use_icc != 0u)
    {
        int2 debug_pos = int2(pixel.x, int((framebuffer_dims.y - 1u) - uint(pixel.y))) - int2(24, 18);
        bool in_region = (debug_pos.x >= 0) && (debug_pos.y >= 0) && (uint(debug_pos.x) < 88u) && (uint(debug_pos.y) < 24u);
        float3 icc_input = dithered;
        if (in_region)
        {
            uint block = uint(debug_pos.x) / 32u;
            uint col = uint(debug_pos.x) % 32u;
            if (col < 24u)
            {
                if ((col == 0u) || (debug_pos.y == 0))
                {
                    icc_input = debug_ramp[block] * (0.4199999868869781494140625f / max(debug_ramp[block].x, max(debug_ramp[block].y, debug_ramp[block].z)));
                }
                else
                {
                    icc_input = debug_ramp[block];
                }
            }
        }
        float3 icc_uv = linear_to_srgb(icc_input) * 0.96875f.xxx + 0.015625f.xxx;
        output_color = srgb_to_linear(icc_lut.SampleLevel(sampler_lnc, icc_uv, 0.0f).xyz);
    }
    else
    {
        output_color = dithered;
    }

    return float4(output_color, 1.0f);
}
