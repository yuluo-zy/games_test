#define PERM_wave32 1
#define PERM_sharpen 0
#define TGV_b303f36f0dbdd5f6 1
#line 1 "shaders/tgsr.cs.hlsl"


#if !((defined(TGV_3ee6a953408e4c4c) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)) || (defined(TGV_7a0af18450cdbcd1) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)) || (defined(TGV_b303f36f0dbdd5f6) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)) || (defined(TGV_f7efabb81dfe256b) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))






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
#line 10 "shaders/tgsr.cs.hlsl"

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
#line 11 "shaders/tgsr.cs.hlsl"

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
#line 12 "shaders/tgsr.cs.hlsl"

#line 13 "shaders/tgsr.cs.hlsl"

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
#line 14 "shaders/tgsr.cs.hlsl"

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
#line 15 "shaders/tgsr.cs.hlsl"

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
#line 16 "shaders/tgsr.cs.hlsl"

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
#line 17 "shaders/tgsr.cs.hlsl"

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
#line 18 "shaders/tgsr.cs.hlsl"

#line 1 "shaders/inc/unjitter_taa.hlsl"


#ifndef TINYGLADE_INC_UNJITTER_TAA_HLSL
#define TINYGLADE_INC_UNJITTER_TAA_HLSL


float2 unjitter_taa_pixel(float2 jittered_pixel, float2 current_offset_pixels)
{
    return jittered_pixel - current_offset_pixels;
}


float2 unjitter_taa_uv(float2 jittered_uv, float2 current_offset_pixels, float2 render_target_dims)
{
    return jittered_uv - (current_offset_pixels / render_target_dims);
}



float2 taa_history_uv(float2 unjittered_uv, float2 reprojection_offset)
{
    return unjittered_uv + reprojection_offset;
}

#endif 
#line 19 "shaders/tgsr.cs.hlsl"

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
#line 20 "shaders/tgsr.cs.hlsl"

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
#line 21 "shaders/tgsr.cs.hlsl"

#line 1 "shaders/tgsr_common.inc.hlsl"



#ifndef TINYGLADE_TGSR_COMMON_INC_HLSL
#define TINYGLADE_TGSR_COMMON_INC_HLSL

#endif 
#line 22 "shaders/tgsr.cs.hlsl"

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
#line 23 "shaders/tgsr.cs.hlsl"

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
#line 24 "shaders/tgsr.cs.hlsl"


cbuffer $Globals : register(b13, space0)
{
    uint2 reprojected_history_tex_lum_dims;
    uint2 reprojected_history_tex_chroma_dims;
    uint2 color_tex_dims;
    uint2 blurred_luminance_tex_dims;
    uint2 history_tex_dims;
    uint2 reprojection_tex_dims;
    uint2 aux_tex_dims;
    uint2 history_output_tex_dims;
    uint2 output_tex_dims;
    uint2 aux_output_tex_dims;
    uint bypass_neighborhood_clamping;
    float sharpen_amount;
    uint history_valid;
};

Texture2D<float4> reprojected_history_tex_lum : register(t14, space0);
Texture2D<float4> reprojected_history_tex_chroma : register(t15, space0);
Texture2D<float4> color_tex : register(t16, space0);
Texture2D<float4> blurred_luminance_tex : register(t17, space0);
Texture2D<float4> reprojection_tex : register(t19, space0);
Texture2D<float4> aux_tex : register(t20, space0);
[[vk::ext_capability(56)]] RWTexture2D<float3> history_output_tex : register(u21, space0);
[[vk::ext_capability(56)]] RWTexture2D<float3> output_tex : register(u22, space0);
[[vk::ext_capability(56)]] RWTexture2D<unorm float2> aux_output_tex : register(u23, space0);

groupshared float tile_luminance[8][8];

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_id : SV_DispatchThreadID, uint3 local_id : SV_GroupThreadID)
{
    float2 out_dims = float2(output_tex_dims);
    float2 uv = (float2(int2(dispatch_id.xy)) + 0.5f) / out_dims;
    bool bypass = bypass_neighborhood_clamping != 0u;
    float tier = float(bypass ? 16 : 4);

    float4 reprojection = reprojection_tex.SampleLevel(sampler_nnc, uv, 0.0f);
    float2 motion = reprojection.xy;
    bool history_available = (history_valid != 0u) && (frame_index != 0u);
    float2 aux = history_available
        ? aux_tex.SampleLevel(sampler_lnc, uv + motion, 0.0f).xy : float2(0.0f, 0.0f);
    float aux_tier = aux.y * tier;

    float3 reproj_hist = float3(reprojected_history_tex_lum.Load(int3(dispatch_id.xy, 0)).x,
        reprojected_history_tex_chroma.Load(int3(dispatch_id.xy, 0)).xy);

    float2 scale = float2(int2(color_tex_dims)) / out_dims;
    int2 current_px = int2((float2(int2(dispatch_id.xy)) + 0.5f) * scale);
    float2 current_pos = (float2(current_px) + 0.5f
        + (view_constants[0].sample_offset_pixels * float2(1.0f, -1.0f))) / scale;
    float sigma_wide = ((-10.0f) * scale.x) * min(4.0f * scale.x, 1.0f);
    float sigma_narrow = (-1.0f) * (scale.x * scale.x);

    float3 sum_sq = float3(0.0f, 0.0f, 0.0f);
    float sum_w1 = 0.0f;
    float3 sum_c = float3(0.0f, 0.0f, 0.0f);
    float sum_w2 = 0.0f;
    float4 sum_c2 = float4(0.0f, 0.0f, 0.0f, 0.0f);
    [loop]
    for (int dy = -1; dy <= 1; ++dy)
    {
        [loop]
        for (int dx = -1; dx <= 1; ++dx)
        {
            float4 c = float4(rgb_to_ycbcr(
                color_tex.Load(int3(uint2(current_px + int2(dx, dy)), 0)).xyz), 1.0f);
            float2 d = (current_pos + (float2(float(dx), float(dy)) / scale))
                - (float2(int2(dispatch_id.xy)) + 0.5f);
            float dist = dot(d, d);
            float w_narrow = exp2(dist * sigma_narrow);
            float w_wide = exp2(dist * sigma_wide);
            sum_w1 += w_narrow;
            sum_sq += (c.xyz * c.xyz) * w_narrow;
            sum_c += c.xyz * w_narrow;
            sum_w2 += w_wide;
            sum_c2 += c * w_wide;
        }
    }

    float3 mean = sum_c / sum_w1;
    float3 wide_result = sum_c2.xyz;

    float blur_sum = 0.0f;
    float blur_weight = 0.0f;
    [loop]
    for (int by = -1; by <= 1; ++by)
    {
        [loop]
        for (int bx = -1; bx <= 1; ++bx)
        {
            float w = exp(float((bx * bx) + (by * by)) * (-0.75f));
            blur_sum += reprojected_history_tex_lum.SampleLevel(sampler_lnc,
                uv + ((float2(float(bx), float(by)) * 2.0f) / float2(color_tex_dims)), 0.0f).x * w;
            blur_weight += w;
        }
    }
    float hist_lum_blur = clamp(sqrt(blur_sum / blur_weight), 0.0f, 1.0f);
    float hist_lum_blur_sq = hist_lum_blur * hist_lum_blur;
    float current_lum = blurred_luminance_tex.SampleLevel(sampler_lnc, uv, 0.0f).x;
    float current_lum_sq = current_lum * current_lum;

    float wide_lum_blur;
    float wide_hist_blur;
    if (WaveActiveMax(WaveGetLaneIndex()) >= 31u)
    {
        wide_lum_blur = lerp(current_lum_sq, WaveReadLaneAt(current_lum_sq, WaveGetLaneIndex() ^ 16u), 0.5f);
        wide_hist_blur = lerp(hist_lum_blur_sq, WaveReadLaneAt(hist_lum_blur_sq, WaveGetLaneIndex() ^ 16u), 0.5f);
    }
    else
    {
        wide_lum_blur = current_lum_sq;
        wide_hist_blur = hist_lum_blur_sq;
    }
    float lum_delta = lerp(wide_hist_blur, WaveReadLaneAt(wide_hist_blur, WaveGetLaneIndex() ^ 2u), 0.5f);
    float lum_delta2 = lerp(wide_lum_blur, WaveReadLaneAt(wide_lum_blur, WaveGetLaneIndex() ^ 2u), 0.5f);

    float3 std = sqrt(max(float3(0.0f, 0.0f, 0.0f), (sum_sq / sum_w1) - (mean * mean))) * 2.0f;
    float3 clamp_min = (mean - std) - 0.001f;
    float3 clamp_max = (mean + std) + 0.001f;
    float2 reproj_dims = float2(reprojection_tex_dims);

    float hist_sum_sq = 0.0f;
    float hist_sum = 0.0f;
    float3 below_count = float3(0.0f, 0.0f, 0.0f);
    float3 above_count = float3(0.0f, 0.0f, 0.0f);
    float hist_count = 0.0f;
    [loop]
    for (int hy = -1; hy <= 1; ++hy)
    {
        [loop]
        for (int hx = -1; hx <= 1; ++hx)
        {
            uint2 sample_px = dispatch_id.xy + uint2(hx, hy);
            float lum = reprojected_history_tex_lum.Load(int3(sample_px, 0)).x;
            float3 sample = float3(lum, 0.5f, reprojected_history_tex_chroma.Load(int3(sample_px, 0)).y);
            hist_sum_sq += lum * lum;
            hist_sum += lum;
            below_count += float3(sample.x < clamp_min.x, sample.y < clamp_min.y, sample.z < clamp_min.z) * 1.0f;
            above_count += float3(sample.x > clamp_max.x, sample.y > clamp_max.y, sample.z > clamp_max.z) * 1.0f;
            hist_count += 1.0f;
        }
    }
    float hist_mean = hist_sum / hist_count;
    float hist_std = sqrt(max(0.0f, (hist_sum_sq / hist_count) - (hist_mean * hist_mean)));
    float4 gather_alpha = reprojection_tex.GatherAlpha(sampler_lnc, uv);
    float history_inconsistency = clamp((20.0f * max(0.0f, hist_std)) / max(9.99999993922529e-09f, hist_mean), 0.0f, 1.0f);
    float2 outside_frac = (above_count / hist_count).xz + (below_count / hist_count).xz;

    float lum_delta_abs = abs(lum_delta - lum_delta2);
    float lum_delta_norm = (2.0f * lum_delta_abs) / max(9.9999997473787516e-06f, lum_delta2 + lum_delta);
    float mean_lum = mean.x;

    float reproj_zero_count = 0.0f;
    float reproj_count = 0.0f;
    [loop]
    for (int ry = -1; ry <= 1; ++ry)
    {
        [loop]
        for (int rx = -1; rx <= 1; ++rx)
        {
            reproj_zero_count += float(reprojection_tex.SampleLevel(sampler_nnc,
                uv - ((reproj_dims * 0.5f) * float2(float(rx), float(ry))), 0.0f).z == 0.0f);
            reproj_count += 1.0f;
        }
    }

    float2 motion_px = motion * out_dims;
    float2 motion_abs = abs(motion_px);
    float4 gather_blue = reprojection_tex.GatherBlue(sampler_lnc, uv);
    float reproj_blue_term = (((3.0f * clamp(1.0f - ((2.5f * hist_std) / hist_mean), 0.0f, 1.0f)) * lum_delta_abs)
        / (0.5f * max(9.9999997473787516e-05f, lum_delta2 + lum_delta)))
        * float(min(min(gather_blue.x, gather_blue.y), min(gather_blue.z, gather_blue.w)) == 1.0f);

    float reproj_occlusion = sqrt(reproj_zero_count / reproj_count)
        * clamp(max(motion_abs.x, motion_abs.y), 0.0f, 1.0f);
    float blend_factor =
        ((((aux.x * 2.0f) - 0.5f) + clamp(1.0f - history_inconsistency, 0.0f, 1.0f))
            + max(outside_frac.x, outside_frac.y)
            + ((lum_delta_norm * lum_delta_norm) * max(0.0f, 1.0f
                - ((((-3.0f) * abs(mean_lum - lum_delta2)) + (2.0f * abs((sum_c2.x / sum_w2) - mean_lum))) / lum_delta2)))
            + reproj_occlusion)
        + ((reproj_blue_term * reproj_blue_term) * reproj_blue_term);

    float color_blend;
    float motion_blend;
    if (!bypass)
    {
        float alpha_mask = clamp(1.0f - min(min(gather_alpha.x, gather_alpha.y), min(gather_alpha.z, gather_alpha.w)), 0.0f, 1.0f);
        if (alpha_mask < 0.75f)
        {
            color_blend = blend_factor + 1.0f;
            motion_blend = min(aux_tier, 0.5f);
        }
        else
        {
            color_blend = blend_factor;
            motion_blend = aux_tier;
        }
        float motion_factor = motion_blend * history_inconsistency;
        float2 motion_norm = abs(motion_px * 0.03125f);
        color_blend = min(motion_factor, lerp(motion_factor, 1.0f, clamp(max(motion_norm.x, motion_norm.y), 0.0f, 1.0f)));
    }
    else
    {
        color_blend = aux_tier;
        motion_blend = blend_factor;
    }

    float final_blend = min(color_blend, 1.0f);
    float3 clamp_range = std * exp2((7.0f * clamp(1.0f - color_blend, 0.0f, 1.0f)) - 0.5f);
    float3 clamped_hist = bypass ? reproj_hist : clamp(reproj_hist, mean - clamp_range, mean + clamp_range);

    float blend_t = min(4.0f, color_blend);
    float total_weight = sum_w2 + blend_t;
    
    float3 blended = lerp(wide_result / max(9.9999997473787516e-06f, sum_w2).xxx, clamped_hist,
        (blend_t / max(9.9999997473787516e-06f, total_weight)).xxx);
    float blended_lum = blended.x;

    bool valid = (history_valid != 0u) && (frame_index != 0u);
    float3 color;
    if (!valid)
    {
        color = max(float3(0.0f, 0.0f, 0.0f),
            ycbcr_to_rgb(wide_result / max(9.9999997473787516e-06f, sum_w2).xxx));
    }
    else
    {
        color = max(float3(0.0f, 0.0f, 0.0f), ycbcr_to_rgb(blended));
    }

    history_output_tex[dispatch_id.xy] = curve(color);

    tile_luminance[local_id.x][local_id.y] = blended_lum;
    GroupMemoryBarrierWithGroupSync();

    uint2 right = local_id.xy + uint2(1u, 0u);
    uint2 left = local_id.xy - uint2(1u, 0u);
    float x_accum = 0.0f;
    float x_weight = 0.0f;
    if (all(bool4(right.x < 8u, right.y < 8u, left.x < 8u, left.y < 8u)))
    {
        float w = max(0.0f, 1.0f - (6.0f
            * (abs(blended_lum - tile_luminance[right.x][right.y]) + abs(blended_lum - tile_luminance[left.x][left.y]))));
        float w2 = min(w, (sharpen_amount * w) * 1.25f);
        x_accum = w2 * (tile_luminance[right.x][right.y] + tile_luminance[left.x][left.y]);
        x_weight = w2 * 2.0f;
    }

    uint2 up = local_id.xy + uint2(0u, 1u);
    uint2 down = local_id.xy - uint2(0u, 1u);
    float y_accum;
    float y_weight;
    if (all(bool4(up.x < 8u, up.y < 8u, down.x < 8u, down.y < 8u)))
    {
        float w = max(0.0f, 1.0f - (6.0f
            * (abs(blended_lum - tile_luminance[up.x][up.y]) + abs(blended_lum - tile_luminance[down.x][down.y]))));
        float w2 = min(w, (sharpen_amount * w) * 1.25f);
        y_accum = (x_accum + (tile_luminance[up.x][up.y] * w2)) + (tile_luminance[down.x][down.y] * w2);
        y_weight = x_weight + (w2 * 2.0f);
    }
    else
    {
        y_accum = x_accum;
        y_weight = x_weight;
    }

    float3 sharpened = color * max(0.0f, max(0.0f,
        (blended_lum * (y_weight + 1.0f)) - y_accum) / max(9.9999997473787516e-06f, blended_lum));
    output_tex[dispatch_id.xy] = curve(sharpened);

    aux_output_tex[dispatch_id.xy] = float2(
        clamp(final_blend + (2.0f * clamp(reprojection.w * 1000.0f, 0.0f, 1.0f)), 0.0f, 2.0f) * 0.5f,
        total_weight / tier);
}

#endif

#if defined(TGV_3ee6a953408e4c4c) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"


float3 incb_frexp(float3 x, out int3 exponent)
{
    
    uint3 bits0 = asuint(x);
    bool3 denorm = ((bits0 & 0x7F800000u) == 0u) & ((bits0 & 0x7FFFFFFFu) != 0u);
    float3 xs = select(denorm, x * 16777216.0f, x);
    uint3 bits = asuint(xs);
    int3 biased = int3(int((bits.x >> 23u) & 0xFFu), int((bits.y >> 23u) & 0xFFu), int((bits.z >> 23u) & 0xFFu));
    int3 e = biased - int3(127, 127, 127) - select(denorm, int3(24, 24, 24), int3(0, 0, 0));
    float3 m = asfloat((bits & 0x807FFFFFu) | 0x3F800000u);
    bool3 big = (m >= 1.0f) | (m <= -1.0f);
    m = lerp(m, m * 0.5f, float3(big));
    e += int3(big);
    bool3 special = (biased == 0) | (biased == 0xFF);
    exponent = select(special, int3(0, 0, 0), e);
    return select(special, xs, m);
}
#define frexp incb_frexp
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

struct FrexpStructType
{
    float3 mantissa;
    int3 exponent;
};

static float _156;
static float _162;

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
    uint2 _Globals_reprojected_history_tex_lum_dims : packoffset(c0);
    uint2 _Globals_reprojected_history_tex_chroma_dims : packoffset(c0.z);
    uint2 _Globals_color_tex_dims : packoffset(c1);
    uint2 _Globals_blurred_luminance_tex_dims : packoffset(c1.z);
    uint2 _Globals_history_tex_dims : packoffset(c2);
    uint2 _Globals_reprojection_tex_dims : packoffset(c2.z);
    uint2 _Globals_aux_tex_dims : packoffset(c3);
    uint2 _Globals_history_output_tex_dims : packoffset(c3.z);
    uint2 _Globals_output_tex_dims : packoffset(c4);
    uint2 _Globals_aux_output_tex_dims : packoffset(c4.z);
    uint _Globals_bypass_neighborhood_clamping : packoffset(c5);
    float _Globals_sharpen_amount : packoffset(c5.y);
    uint _Globals_history_valid : packoffset(c5.z);
};

SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_nnc : register(s2, space0);
Texture2D<float4> reprojected_history_tex_lum : register(t14, space0);
Texture2D<float4> reprojected_history_tex_chroma : register(t15, space0);
Texture2D<float4> color_tex : register(t16, space0);
Texture2D<float4> blurred_luminance_tex : register(t17, space0);
Texture2D<float4> reprojection_tex : register(t19, space0);
Texture2D<float4> aux_tex : register(t20, space0);
RWTexture2D<float3> history_output_tex : register(u21, space0);
RWTexture2D<float3> output_tex : register(u22, space0);
RWTexture2D<unorm float2> aux_output_tex : register(u23, space0);

static uint3 gl_LocalInvocationID;
static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_LocalInvocationID : SV_GroupThreadID;
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

groupshared float tile_luminance[8][8];

void comp_main()
{
    float2 _171 = float2(_Globals_output_tex_dims);
    float2 _173 = float2(int2(gl_GlobalInvocationID.xy)) + 0.5f.xx;
    float2 _174 = _173 / _171;
    bool _177 = _Globals_bypass_neighborhood_clamping != 0u;
    float _179 = float(_177 ? 16 : 4);
    float4 _183 = reprojection_tex.SampleLevel(sampler_nnc, _174, 0.0f);
    float2 _184 = _183.xy;
    bool _188 = _Globals_history_valid != 0u;
    bool _195;
    if (_188)
    {
        _195 = !(0u == frame_constants_frame_index);
    }
    else
    {
        _195 = false;
    }
    float2 _204;
    if (_195)
    {
        _204 = aux_tex.SampleLevel(sampler_lnc, _174 + _184, 0.0f).xy;
    }
    else
    {
        _204 = 0.0f.xx;
    }
    float _208 = _204.y * _179;
    float3 _216 = float3(reprojected_history_tex_lum.Load(int3(gl_GlobalInvocationID.xy, 0u)).x, reprojected_history_tex_chroma.Load(int3(gl_GlobalInvocationID.xy, 0u)).xy);
    float2 _224 = float2(int2(_Globals_color_tex_dims)) / _171;
    int2 _226 = int2(_173 * _224);
    float2 _231 = ((float2(_226) + 0.5f.xx) + (frame_constants_view_constants[0u].sample_offset_pixels * float2(1.0f, -1.0f))) / _224;
    float _232 = _224.x;
    float _236 = ((-10.0f) * _232) * min(4.0f * _232, 1.0f);
    float _238 = (-1.0f) * (_232 * _232);
    float3 _240;
    float _243;
    float3 _245;
    float _247;
    float4 _249;
    int _251;
    _240 = 0.0f.xxx;
    _243 = 0.0f;
    _245 = 0.0f.xxx;
    _247 = 0.0f;
    _249 = 0.0f.xxxx;
    _251 = -1;
    float3 _241;
    float _244;
    float3 _246;
    float _248;
    float4 _250;
    for (; _251 <= 1; _240 = _241, _243 = _244, _245 = _246, _247 = _248, _249 = _250, _251++)
    {
        _244 = _243;
        _241 = _240;
        _246 = _245;
        _248 = _247;
        _250 = _249;
        for (int _263 = -1; _263 <= 1; )
        {
            float4 _281 = float4(mul(float3x3(float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f), float3(-0.114600002765655517578125f, -0.385399997234344482421875f, 0.5f), float3(0.5f, -0.4541999995708465576171875f, -0.0458000004291534423828125f)), color_tex.Load(int3(uint2(_226 + int2(_263, _251)), 0u)).xyz), _156);
            float2 _282 = (_231 + (float2(float(_263), float(_251)) / _224)) - _173;
            float _283 = dot(_282, _282);
            float _285 = exp2(_283 * _238);
            float _287 = exp2(_283 * _236);
            float3 _289 = _281.xyz;
            _244 += _285;
            _241 += ((_289 * _289) * _285);
            _246 += (_289 * _285);
            _248 += _287;
            _250 += (_281 * _287);
            _263++;
            continue;
        }
    }
    float3 _293 = _243.xxx;
    float3 _294 = _245 / _293;
    float3 _296 = _249.xyz;
    float _298;
    float _301;
    int _303;
    _298 = 0.0f;
    _301 = 0.0f;
    _303 = -1;
    float _299;
    float _302;
    for (; _303 <= 1; _298 = _299, _301 = _302, _303++)
    {
        _299 = _298;
        _302 = _301;
        for (int _312 = -1; _312 <= 1; )
        {
            float _321 = exp(float((_312 * _312) + (_303 * _303)) * (-0.75f));
            _299 += (reprojected_history_tex_lum.SampleLevel(sampler_lnc, _174 + ((float2(float(_312), float(_303)) * 2.0f) / float2(_Globals_color_tex_dims)), 0.0f).x * _321);
            _302 += _321;
            _312++;
            continue;
        }
    }
    float _336 = clamp(sqrt(_298 / _301), 0.0f, 1.0f);
    float _337 = _336 * _336;
    float4 _341 = blurred_luminance_tex.SampleLevel(sampler_lnc, _174, 0.0f);
    float _342 = _341.x;
    float _343 = _342 * _342;
    float _354;
    float _355;
    if (WaveActiveMax(WaveGetLaneIndex()) >= 31u)
    {
        uint _349 = WaveGetLaneIndex() ^ 16u;
        _354 = lerp(_343, WaveReadLaneAt(_343, _349), 0.5f);
        _355 = lerp(_337, WaveReadLaneAt(_337, _349), 0.5f);
    }
    else
    {
        _354 = _343;
        _355 = _337;
    }
    uint _356 = WaveGetLaneIndex() ^ 2u;
    float _357 = WaveReadLaneAt(_355, _356);
    float _358 = lerp(_355, _357, 0.5f);
    float _359 = WaveReadLaneAt(_354, _356);
    float _360 = lerp(_354, _359, 0.5f);
    float3 _364 = sqrt(max(0.0f.xxx, (_240 / _293) - (_294 * _294)));
    float2 _368 = 0.5f.xx / float2(_Globals_reprojection_tex_dims);
    float3 _369 = _364 * 2.0f;
    float3 _371 = (_294 + _369) + 0.001000000047497451305389404296875f.xxx;
    float3 _373 = (_294 - _369) - 0.001000000047497451305389404296875f.xxx;
    float _375;
    float _378;
    float3 _380;
    float3 _382;
    float _384;
    int _386;
    _375 = 0.0f;
    _378 = 0.0f;
    _380 = 0.0f.xxx;
    _382 = 0.0f.xxx;
    _384 = 0.0f;
    _386 = -1;
    float _376;
    float _379;
    float3 _381;
    float3 _383;
    float _385;
    for (; _386 <= 1; _375 = _376, _378 = _379, _380 = _381, _382 = _383, _384 = _385, _386++)
    {
        _376 = _375;
        _379 = _378;
        _381 = _380;
        _383 = _382;
        _385 = _384;
        for (int _398 = -1; _398 <= 1; )
        {
            uint2 _404 = gl_GlobalInvocationID.xy + uint2(int2(_398, _386));
            float4 _405 = reprojected_history_tex_lum.Load(int3(_404, 0u));
            float _406 = _405.x;
            float3 _409 = float3(_406, _162, reprojected_history_tex_chroma.Load(int3(_404, 0u)).y);
            _376 += (_406 * _406);
            _379 += _406;
            _381 += (select(bool3(_409.x < _373.x, _409.y < _373.y, _409.z < _373.z), float3(1.0f, 1.0f, 1.0f), float3(0.0f, 0.0f, 0.0f)) * 1.0f);
            _383 += (select(bool3(_409.x > _371.x, _409.y > _371.y, _409.z > _371.z), float3(1.0f, 1.0f, 1.0f), float3(0.0f, 0.0f, 0.0f)) * 1.0f);
            _385 += 1.0f;
            _398++;
            continue;
        }
    }
    float3 _417 = _384.xxx;
    float _420 = _378 / _384;
    float _425 = sqrt(max(0.0f, (_375 / _384) - (_420 * _420)));
    float4 _427 = reprojection_tex.GatherAlpha(sampler_lnc, _174);
    float _441 = clamp((20.0f * max(0.0f, _425)) / max(9.9999999392252902907785028219223e-09f, _420), 0.0f, 1.0f);
    float2 _448 = (_382 / _417).xz + (_380 / _417).xz;
    float _454 = abs(_358 - _360);
    float _458 = (2.0f * _454) / max(9.9999997473787516355514526367188e-06f, _360 + _358);
    float _460 = _294.x;
    float _476;
    float _479;
    int _481;
    _476 = 0.0f;
    _479 = 0.0f;
    _481 = -1;
    float _477;
    float _480;
    for (; _481 <= 1; _476 = _477, _479 = _480, _481++)
    {
        _477 = _476;
        _480 = _479;
        for (int _490 = -1; _490 <= 1; )
        {
            _477 += ((reprojection_tex.SampleLevel(sampler_nnc, _174 - ((_368 * 1.0f) * float2(float(_490), float(_481))), 0.0f).z == 0.0f) ? 1.0f : 0.0f);
            _480 += 1.0f;
            _490++;
            continue;
        }
    }
    float2 _507 = _184 * _171;
    float2 _508 = abs(_507);
    float4 _526 = reprojection_tex.GatherBlue(sampler_lnc, _174);
    float _536 = (((3.0f * clamp(1.0f - ((2.5f * _425) / _420), 0.0f, 1.0f)) * _454) / (0.5f * max(9.9999997473787516355514526367188e-05f, _358 + _360))) * ((min(min(_526.x, _526.y), min(_526.z, _526.w)) == 1.0f) ? 1.0f : 0.0f);
    float _539 = ((((((_204.x * 2.0f) - 0.5f) + clamp(1.0f - _441, 0.0f, 1.0f)) + max(_448.x, _448.y)) + ((_458 * _458) * max(0.0f, 1.0f - ((((-3.0f) * abs(_460 - _360)) + (2.0f * abs((_249.x / _247) - _460))) / _360)))) + (sqrt(_476 / _479) * clamp(max(_508.x, _508.y), 0.0f, 1.0f))) + ((_536 * _536) * _536);
    bool _540 = !_177;
    float _559;
    float _560;
    if (_540)
    {
        float _548;
        float _549;
        if (clamp(1.0f - min(min(_427.x, _427.y), min(_427.z, _427.w)), 0.0f, 1.0f) < 0.75f)
        {
            _548 = _539 + 1.0f;
            _549 = min(_208, 0.5f);
        }
        else
        {
            _548 = _539;
            _549 = _208;
        }
        float _550 = _549 * _441;
        float2 _552 = abs(_507 * 0.03125f.xx);
        _559 = min(_550, lerp(_550, 1.0f, clamp(max(_552.x, _552.y), 0.0f, 1.0f)));
        _560 = _548;
    }
    else
    {
        _559 = _208;
        _560 = _539;
    }
    float _566 = min(_560, 1.0f);
    float3 _572 = _364 * exp2((7.0f * clamp(1.0f - _560, 0.0f, 1.0f)) - 0.5f);
    float3 _579;
    if (_540)
    {
        _579 = clamp(_216, _294 - _572, _294 + _572);
    }
    else
    {
        _579 = _216;
    }
    float _580 = min(4.0f, _559);
    float _581 = _247 + _580;
    float3 _584 = _247.xxx;
    float3 _587 = lerp(_296 / _584, _579, (_580 / max(9.9999997473787516355514526367188e-06f, _581)).xxx);
    float _588 = _587.x;
    bool _597;
    if (_188)
    {
        _597 = !(0u == frame_constants_frame_index);
    }
    else
    {
        _597 = false;
    }
    float3 _606;
    if (!_597)
    {
        _606 = max(0.0f.xxx, mul(float3x3(float3(1.0f, 0.0f, 1.574800014495849609375f), float3(1.0f, -0.1872999966144561767578125f, -0.468100011348724365234375f), float3(1.0f, 1.85559999942779541015625f, 0.0f)), _296 / max(9.9999997473787516355514526367188e-06f, _247).xxx));
    }
    else
    {
        _606 = max(0.0f.xxx, mul(float3x3(float3(1.0f, 0.0f, 1.574800014495849609375f), float3(1.0f, -0.1872999966144561767578125f, -0.468100011348724365234375f), float3(1.0f, 1.85559999942779541015625f, 0.0f)), _587));
    }
    FrexpStructType _607;
    { int3 _fe; _607.mantissa = frexp(_606, _fe); _607.exponent = _fe; }
    history_output_tex[gl_GlobalInvocationID.xy] = _606 + exp2(float3(_607.exponent) - float3(8.0f, 8.0f, 7.0f));
    tile_luminance[gl_LocalInvocationID.x][gl_LocalInvocationID.y] = _588;
    GroupMemoryBarrierWithGroupSync();
    uint2 _617 = gl_LocalInvocationID.xy + uint2(1u, 0u);
    uint2 _618 = gl_LocalInvocationID.xy - uint2(1u, 0u);
    uint _619 = _617.x;
    uint _620 = _617.y;
    uint _621 = _618.x;
    uint _622 = _618.y;
    uint4 _623 = uint4(_619, _620, _621, _622);
    float _648;
    float _649;
    if (all(bool4(_623.x < uint4(8u, 8u, 8u, 8u).x, _623.y < uint4(8u, 8u, 8u, 8u).y, _623.z < uint4(8u, 8u, 8u, 8u).z, _623.w < uint4(8u, 8u, 8u, 8u).w)))
    {
        float _639 = max(0.0f, 1.0f - (6.0f * (abs(_588 - tile_luminance[_619][_620]) + abs(_588 - tile_luminance[_621][_622]))));
        float _644 = min(_639, (_Globals_sharpen_amount * _639) * 1.25f);
        _648 = _644 * (tile_luminance[_619][_620] + tile_luminance[_621][_622]);
        _649 = _644 * 2.0f;
    }
    else
    {
        _648 = 0.0f;
        _649 = 0.0f;
    }
    uint2 _650 = gl_LocalInvocationID.xy + uint2(0u, 1u);
    uint2 _651 = gl_LocalInvocationID.xy - uint2(0u, 1u);
    uint _652 = _650.x;
    uint _653 = _650.y;
    uint _654 = _651.x;
    uint _655 = _651.y;
    uint4 _656 = uint4(_652, _653, _654, _655);
    float _684;
    float _685;
    if (all(bool4(_656.x < uint4(8u, 8u, 8u, 8u).x, _656.y < uint4(8u, 8u, 8u, 8u).y, _656.z < uint4(8u, 8u, 8u, 8u).z, _656.w < uint4(8u, 8u, 8u, 8u).w)))
    {
        float _672 = max(0.0f, 1.0f - (6.0f * (abs(_588 - tile_luminance[_652][_653]) + abs(_588 - tile_luminance[_654][_655]))));
        float _677 = min(_672, (_Globals_sharpen_amount * _672) * 1.25f);
        _684 = (_648 + (tile_luminance[_652][_653] * _677)) + (tile_luminance[_654][_655] * _677);
        _685 = _649 + (_677 * 2.0f);
    }
    else
    {
        _684 = _648;
        _685 = _649;
    }
    float3 _693 = _606 * max(0.0f, max(0.0f, (_588 * (_685 + 1.0f)) - _684) / max(9.9999997473787516355514526367188e-06f, _588));
    FrexpStructType _694;
    { int3 _fe; _694.mantissa = frexp(_693, _fe); _694.exponent = _fe; }
    output_tex[gl_GlobalInvocationID.xy] = _693 + exp2(float3(_694.exponent) - float3(8.0f, 8.0f, 7.0f));
    aux_output_tex[gl_GlobalInvocationID.xy] = float2(clamp(_566 + (2.0f * clamp(_183.w * 1000.0f, 0.0f, 1.0f)), 0.0f, 2.0f) * 0.5f, _581 / _179);
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_LocalInvocationID = stage_input.gl_LocalInvocationID;
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_7a0af18450cdbcd1) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"


float3 incb_frexp(float3 x, out int3 exponent)
{
    
    uint3 bits0 = asuint(x);
    bool3 denorm = ((bits0 & 0x7F800000u) == 0u) & ((bits0 & 0x7FFFFFFFu) != 0u);
    float3 xs = select(denorm, x * 16777216.0f, x);
    uint3 bits = asuint(xs);
    int3 biased = int3(int((bits.x >> 23u) & 0xFFu), int((bits.y >> 23u) & 0xFFu), int((bits.z >> 23u) & 0xFFu));
    int3 e = biased - int3(127, 127, 127) - select(denorm, int3(24, 24, 24), int3(0, 0, 0));
    float3 m = asfloat((bits & 0x807FFFFFu) | 0x3F800000u);
    bool3 big = (m >= 1.0f) | (m <= -1.0f);
    m = lerp(m, m * 0.5f, float3(big));
    e += int3(big);
    bool3 special = (biased == 0) | (biased == 0xFF);
    exponent = select(special, int3(0, 0, 0), e);
    return select(special, xs, m);
}
#define frexp incb_frexp
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

struct FrexpStructType
{
    float3 mantissa;
    int3 exponent;
};

static float _141;
static float _145;

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
    uint2 _Globals_reprojected_history_tex_lum_dims : packoffset(c0);
    uint2 _Globals_reprojected_history_tex_chroma_dims : packoffset(c0.z);
    uint2 _Globals_color_tex_dims : packoffset(c1);
    uint2 _Globals_blurred_luminance_tex_dims : packoffset(c1.z);
    uint2 _Globals_history_tex_dims : packoffset(c2);
    uint2 _Globals_reprojection_tex_dims : packoffset(c2.z);
    uint2 _Globals_aux_tex_dims : packoffset(c3);
    uint2 _Globals_history_output_tex_dims : packoffset(c3.z);
    uint2 _Globals_output_tex_dims : packoffset(c4);
    uint2 _Globals_aux_output_tex_dims : packoffset(c4.z);
    uint _Globals_bypass_neighborhood_clamping : packoffset(c5);
    float _Globals_sharpen_amount : packoffset(c5.y);
    uint _Globals_history_valid : packoffset(c5.z);
};

SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_nnc : register(s2, space0);
Texture2D<float4> reprojected_history_tex_lum : register(t14, space0);
Texture2D<float4> reprojected_history_tex_chroma : register(t15, space0);
Texture2D<float4> color_tex : register(t16, space0);
Texture2D<float4> blurred_luminance_tex : register(t17, space0);
Texture2D<float4> reprojection_tex : register(t19, space0);
Texture2D<float4> aux_tex : register(t20, space0);
RWTexture2D<float3> history_output_tex : register(u21, space0);
RWTexture2D<float3> output_tex : register(u22, space0);
RWTexture2D<unorm float2> aux_output_tex : register(u23, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    float2 _152 = float2(_Globals_output_tex_dims);
    float2 _154 = float2(int2(gl_GlobalInvocationID.xy)) + 0.5f.xx;
    float2 _155 = _154 / _152;
    bool _158 = _Globals_bypass_neighborhood_clamping != 0u;
    float _160 = float(_158 ? 16 : 4);
    float4 _164 = reprojection_tex.SampleLevel(sampler_nnc, _155, 0.0f);
    float2 _165 = _164.xy;
    bool _169 = _Globals_history_valid != 0u;
    bool _176;
    if (_169)
    {
        _176 = !(0u == frame_constants_frame_index);
    }
    else
    {
        _176 = false;
    }
    float2 _185;
    if (_176)
    {
        _185 = aux_tex.SampleLevel(sampler_lnc, _155 + _165, 0.0f).xy;
    }
    else
    {
        _185 = 0.0f.xx;
    }
    float _189 = _185.y * _160;
    float3 _197 = float3(reprojected_history_tex_lum.Load(int3(gl_GlobalInvocationID.xy, 0u)).x, reprojected_history_tex_chroma.Load(int3(gl_GlobalInvocationID.xy, 0u)).xy);
    float2 _205 = float2(int2(_Globals_color_tex_dims)) / _152;
    int2 _207 = int2(_154 * _205);
    float2 _212 = ((float2(_207) + 0.5f.xx) + (frame_constants_view_constants[0u].sample_offset_pixels * float2(1.0f, -1.0f))) / _205;
    float _213 = _205.x;
    float _217 = ((-10.0f) * _213) * min(4.0f * _213, 1.0f);
    float _219 = (-1.0f) * (_213 * _213);
    float3 _221;
    float _224;
    float3 _226;
    float _228;
    float4 _230;
    int _232;
    _221 = 0.0f.xxx;
    _224 = 0.0f;
    _226 = 0.0f.xxx;
    _228 = 0.0f;
    _230 = 0.0f.xxxx;
    _232 = -1;
    float3 _222;
    float _225;
    float3 _227;
    float _229;
    float4 _231;
    for (; _232 <= 1; _221 = _222, _224 = _225, _226 = _227, _228 = _229, _230 = _231, _232++)
    {
        _225 = _224;
        _222 = _221;
        _227 = _226;
        _229 = _228;
        _231 = _230;
        for (int _244 = -1; _244 <= 1; )
        {
            float4 _262 = float4(mul(float3x3(float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f), float3(-0.114600002765655517578125f, -0.385399997234344482421875f, 0.5f), float3(0.5f, -0.4541999995708465576171875f, -0.0458000004291534423828125f)), color_tex.Load(int3(uint2(_207 + int2(_244, _232)), 0u)).xyz), _141);
            float2 _263 = (_212 + (float2(float(_244), float(_232)) / _205)) - _154;
            float _264 = dot(_263, _263);
            float _266 = exp2(_264 * _219);
            float _268 = exp2(_264 * _217);
            float3 _270 = _262.xyz;
            _225 += _266;
            _222 += ((_270 * _270) * _266);
            _227 += (_270 * _266);
            _229 += _268;
            _231 += (_262 * _268);
            _244++;
            continue;
        }
    }
    float3 _274 = _224.xxx;
    float3 _275 = _226 / _274;
    float _279;
    float _282;
    int _284;
    _279 = 0.0f;
    _282 = 0.0f;
    _284 = -1;
    float _280;
    float _283;
    for (; _284 <= 1; _279 = _280, _282 = _283, _284++)
    {
        _280 = _279;
        _283 = _282;
        for (int _293 = -1; _293 <= 1; )
        {
            float _302 = exp(float((_293 * _293) + (_284 * _284)) * (-0.75f));
            _280 += (reprojected_history_tex_lum.SampleLevel(sampler_lnc, _155 + ((float2(float(_293), float(_284)) * 2.0f) / float2(_Globals_color_tex_dims)), 0.0f).x * _302);
            _283 += _302;
            _293++;
            continue;
        }
    }
    float _317 = clamp(sqrt(_279 / _282), 0.0f, 1.0f);
    float _318 = _317 * _317;
    float4 _322 = blurred_luminance_tex.SampleLevel(sampler_lnc, _155, 0.0f);
    float _323 = _322.x;
    float _324 = _323 * _323;
    float _335;
    float _336;
    if (WaveActiveMax(WaveGetLaneIndex()) >= 31u)
    {
        uint _330 = WaveGetLaneIndex() ^ 16u;
        _335 = lerp(_324, WaveReadLaneAt(_324, _330), 0.5f);
        _336 = lerp(_318, WaveReadLaneAt(_318, _330), 0.5f);
    }
    else
    {
        _335 = _324;
        _336 = _318;
    }
    uint _337 = WaveGetLaneIndex() ^ 2u;
    float _338 = WaveReadLaneAt(_336, _337);
    float _339 = lerp(_336, _338, 0.5f);
    float _340 = WaveReadLaneAt(_335, _337);
    float _341 = lerp(_335, _340, 0.5f);
    float3 _345 = sqrt(max(0.0f.xxx, (_221 / _274) - (_275 * _275)));
    float2 _349 = 0.5f.xx / float2(_Globals_reprojection_tex_dims);
    float3 _350 = _345 * 2.0f;
    float3 _352 = (_275 + _350) + 0.001000000047497451305389404296875f.xxx;
    float3 _354 = (_275 - _350) - 0.001000000047497451305389404296875f.xxx;
    float _356;
    float _359;
    float3 _361;
    float3 _363;
    float _365;
    int _367;
    _356 = 0.0f;
    _359 = 0.0f;
    _361 = 0.0f.xxx;
    _363 = 0.0f.xxx;
    _365 = 0.0f;
    _367 = -1;
    float _357;
    float _360;
    float3 _362;
    float3 _364;
    float _366;
    for (; _367 <= 1; _356 = _357, _359 = _360, _361 = _362, _363 = _364, _365 = _366, _367++)
    {
        _357 = _356;
        _360 = _359;
        _362 = _361;
        _364 = _363;
        _366 = _365;
        for (int _379 = -1; _379 <= 1; )
        {
            uint2 _385 = gl_GlobalInvocationID.xy + uint2(int2(_379, _367));
            float4 _386 = reprojected_history_tex_lum.Load(int3(_385, 0u));
            float _387 = _386.x;
            float3 _390 = float3(_387, _145, reprojected_history_tex_chroma.Load(int3(_385, 0u)).y);
            _357 += (_387 * _387);
            _360 += _387;
            _362 += (select(bool3(_390.x < _354.x, _390.y < _354.y, _390.z < _354.z), float3(1.0f, 1.0f, 1.0f), float3(0.0f, 0.0f, 0.0f)) * 1.0f);
            _364 += (select(bool3(_390.x > _352.x, _390.y > _352.y, _390.z > _352.z), float3(1.0f, 1.0f, 1.0f), float3(0.0f, 0.0f, 0.0f)) * 1.0f);
            _366 += 1.0f;
            _379++;
            continue;
        }
    }
    float3 _398 = _365.xxx;
    float _401 = _359 / _365;
    float _406 = sqrt(max(0.0f, (_356 / _365) - (_401 * _401)));
    float4 _408 = reprojection_tex.GatherAlpha(sampler_lnc, _155);
    float _422 = clamp((20.0f * max(0.0f, _406)) / max(9.9999999392252902907785028219223e-09f, _401), 0.0f, 1.0f);
    float2 _429 = (_363 / _398).xz + (_361 / _398).xz;
    float _435 = abs(_339 - _341);
    float _439 = (2.0f * _435) / max(9.9999997473787516355514526367188e-06f, _341 + _339);
    float _441 = _275.x;
    float _457;
    float _460;
    int _462;
    _457 = 0.0f;
    _460 = 0.0f;
    _462 = -1;
    float _458;
    float _461;
    for (; _462 <= 1; _457 = _458, _460 = _461, _462++)
    {
        _458 = _457;
        _461 = _460;
        for (int _471 = -1; _471 <= 1; )
        {
            _458 += ((reprojection_tex.SampleLevel(sampler_nnc, _155 - ((_349 * 1.0f) * float2(float(_471), float(_462))), 0.0f).z == 0.0f) ? 1.0f : 0.0f);
            _461 += 1.0f;
            _471++;
            continue;
        }
    }
    float2 _488 = _165 * _152;
    float2 _489 = abs(_488);
    float4 _507 = reprojection_tex.GatherBlue(sampler_lnc, _155);
    float _517 = (((3.0f * clamp(1.0f - ((2.5f * _406) / _401), 0.0f, 1.0f)) * _435) / (0.5f * max(9.9999997473787516355514526367188e-05f, _339 + _341))) * ((min(min(_507.x, _507.y), min(_507.z, _507.w)) == 1.0f) ? 1.0f : 0.0f);
    float _520 = ((((((_185.x * 2.0f) - 0.5f) + clamp(1.0f - _422, 0.0f, 1.0f)) + max(_429.x, _429.y)) + ((_439 * _439) * max(0.0f, 1.0f - ((((-3.0f) * abs(_441 - _341)) + (2.0f * abs((_230.x / _228) - _441))) / _341)))) + (sqrt(_457 / _460) * clamp(max(_489.x, _489.y), 0.0f, 1.0f))) + ((_517 * _517) * _517);
    bool _521 = !_158;
    float _540;
    float _541;
    if (_521)
    {
        float _529;
        float _530;
        if (clamp(1.0f - min(min(_408.x, _408.y), min(_408.z, _408.w)), 0.0f, 1.0f) < 0.75f)
        {
            _529 = _520 + 1.0f;
            _530 = min(_189, 0.5f);
        }
        else
        {
            _529 = _520;
            _530 = _189;
        }
        float _531 = _530 * _422;
        float2 _533 = abs(_488 * 0.03125f.xx);
        _540 = min(_531, lerp(_531, 1.0f, clamp(max(_533.x, _533.y), 0.0f, 1.0f)));
        _541 = _529;
    }
    else
    {
        _540 = _189;
        _541 = _520;
    }
    float3 _553 = _345 * exp2((7.0f * clamp(1.0f - _541, 0.0f, 1.0f)) - 0.5f);
    float3 _560;
    if (_521)
    {
        _560 = clamp(_197, _275 - _553, _275 + _553);
    }
    else
    {
        _560 = _197;
    }
    float _561 = min(4.0f, _540);
    float _562 = _228 + _561;
    bool _577;
    if (_169)
    {
        _577 = !(0u == frame_constants_frame_index);
    }
    else
    {
        _577 = false;
    }
    float3 _586;
    if (!_577)
    {
        _586 = max(0.0f.xxx, mul(float3x3(float3(1.0f, 0.0f, 1.574800014495849609375f), float3(1.0f, -0.1872999966144561767578125f, -0.468100011348724365234375f), float3(1.0f, 1.85559999942779541015625f, 0.0f)), _230.xyz / max(9.9999997473787516355514526367188e-06f, _228).xxx));
    }
    else
    {
        _586 = max(0.0f.xxx, mul(float3x3(float3(1.0f, 0.0f, 1.574800014495849609375f), float3(1.0f, -0.1872999966144561767578125f, -0.468100011348724365234375f), float3(1.0f, 1.85559999942779541015625f, 0.0f)), lerp(_230.xyz / _228.xxx, _560, (_561 / max(9.9999997473787516355514526367188e-06f, _562)).xxx)));
    }
    FrexpStructType _587;
    { int3 _fe; _587.mantissa = frexp(_586, _fe); _587.exponent = _fe; }
    float3 _592 = _586 + exp2(float3(_587.exponent) - float3(8.0f, 8.0f, 7.0f));
    history_output_tex[gl_GlobalInvocationID.xy] = _592;
    output_tex[gl_GlobalInvocationID.xy] = _592;
    aux_output_tex[gl_GlobalInvocationID.xy] = float2(clamp(min(_541, 1.0f) + (2.0f * clamp(_164.w * 1000.0f, 0.0f, 1.0f)), 0.0f, 2.0f) * 0.5f, _562 / _160);
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b303f36f0dbdd5f6) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"


float3 incb_frexp(float3 x, out int3 exponent)
{
    
    uint3 bits0 = asuint(x);
    bool3 denorm = ((bits0 & 0x7F800000u) == 0u) & ((bits0 & 0x7FFFFFFFu) != 0u);
    float3 xs = select(denorm, x * 16777216.0f, x);
    uint3 bits = asuint(xs);
    int3 biased = int3(int((bits.x >> 23u) & 0xFFu), int((bits.y >> 23u) & 0xFFu), int((bits.z >> 23u) & 0xFFu));
    int3 e = biased - int3(127, 127, 127) - select(denorm, int3(24, 24, 24), int3(0, 0, 0));
    float3 m = asfloat((bits & 0x807FFFFFu) | 0x3F800000u);
    bool3 big = (m >= 1.0f) | (m <= -1.0f);
    m = lerp(m, m * 0.5f, float3(big));
    e += int3(big);
    bool3 special = (biased == 0) | (biased == 0xFF);
    exponent = select(special, int3(0, 0, 0), e);
    return select(special, xs, m);
}
#define frexp incb_frexp
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

struct FrexpStructType
{
    float3 mantissa;
    int3 exponent;
};

static float _141;
static float _145;

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
    uint2 _Globals_reprojected_history_tex_lum_dims : packoffset(c0);
    uint2 _Globals_reprojected_history_tex_chroma_dims : packoffset(c0.z);
    uint2 _Globals_color_tex_dims : packoffset(c1);
    uint2 _Globals_blurred_luminance_tex_dims : packoffset(c1.z);
    uint2 _Globals_history_tex_dims : packoffset(c2);
    uint2 _Globals_reprojection_tex_dims : packoffset(c2.z);
    uint2 _Globals_aux_tex_dims : packoffset(c3);
    uint2 _Globals_history_output_tex_dims : packoffset(c3.z);
    uint2 _Globals_output_tex_dims : packoffset(c4);
    uint2 _Globals_aux_output_tex_dims : packoffset(c4.z);
    uint _Globals_bypass_neighborhood_clamping : packoffset(c5);
    float _Globals_sharpen_amount : packoffset(c5.y);
    uint _Globals_history_valid : packoffset(c5.z);
};

SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_nnc : register(s2, space0);
Texture2D<float4> reprojected_history_tex_lum : register(t14, space0);
Texture2D<float4> reprojected_history_tex_chroma : register(t15, space0);
Texture2D<float4> color_tex : register(t16, space0);
Texture2D<float4> blurred_luminance_tex : register(t17, space0);
Texture2D<float4> reprojection_tex : register(t19, space0);
Texture2D<float4> aux_tex : register(t20, space0);
RWTexture2D<float3> history_output_tex : register(u21, space0);
RWTexture2D<float3> output_tex : register(u22, space0);
RWTexture2D<unorm float2> aux_output_tex : register(u23, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    float2 _152 = float2(_Globals_output_tex_dims);
    float2 _154 = float2(int2(gl_GlobalInvocationID.xy)) + 0.5f.xx;
    float2 _155 = _154 / _152;
    bool _158 = _Globals_bypass_neighborhood_clamping != 0u;
    float _160 = float(_158 ? 16 : 4);
    float4 _164 = reprojection_tex.SampleLevel(sampler_nnc, _155, 0.0f);
    float2 _165 = _164.xy;
    bool _169 = _Globals_history_valid != 0u;
    bool _176;
    if (_169)
    {
        _176 = !(0u == frame_constants_frame_index);
    }
    else
    {
        _176 = false;
    }
    float2 _185;
    if (_176)
    {
        _185 = aux_tex.SampleLevel(sampler_lnc, _155 + _165, 0.0f).xy;
    }
    else
    {
        _185 = 0.0f.xx;
    }
    float _189 = _185.y * _160;
    float3 _197 = float3(reprojected_history_tex_lum.Load(int3(gl_GlobalInvocationID.xy, 0u)).x, reprojected_history_tex_chroma.Load(int3(gl_GlobalInvocationID.xy, 0u)).xy);
    float2 _205 = float2(int2(_Globals_color_tex_dims)) / _152;
    int2 _207 = int2(_154 * _205);
    float2 _212 = ((float2(_207) + 0.5f.xx) + (frame_constants_view_constants[0u].sample_offset_pixels * float2(1.0f, -1.0f))) / _205;
    float _213 = _205.x;
    float _217 = ((-10.0f) * _213) * min(4.0f * _213, 1.0f);
    float _219 = (-1.0f) * (_213 * _213);
    float3 _221;
    float _224;
    float3 _226;
    float _228;
    float4 _230;
    int _232;
    _221 = 0.0f.xxx;
    _224 = 0.0f;
    _226 = 0.0f.xxx;
    _228 = 0.0f;
    _230 = 0.0f.xxxx;
    _232 = -1;
    float3 _222;
    float _225;
    float3 _227;
    float _229;
    float4 _231;
    for (; _232 <= 1; _221 = _222, _224 = _225, _226 = _227, _228 = _229, _230 = _231, _232++)
    {
        _225 = _224;
        _222 = _221;
        _227 = _226;
        _229 = _228;
        _231 = _230;
        for (int _244 = -1; _244 <= 1; )
        {
            float4 _262 = float4(mul(float3x3(float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f), float3(-0.114600002765655517578125f, -0.385399997234344482421875f, 0.5f), float3(0.5f, -0.4541999995708465576171875f, -0.0458000004291534423828125f)), color_tex.Load(int3(uint2(_207 + int2(_244, _232)), 0u)).xyz), _141);
            float2 _263 = (_212 + (float2(float(_244), float(_232)) / _205)) - _154;
            float _264 = dot(_263, _263);
            float _266 = exp2(_264 * _219);
            float _268 = exp2(_264 * _217);
            float3 _270 = _262.xyz;
            _225 += _266;
            _222 += ((_270 * _270) * _266);
            _227 += (_270 * _266);
            _229 += _268;
            _231 += (_262 * _268);
            _244++;
            continue;
        }
    }
    float3 _274 = _224.xxx;
    float3 _275 = _226 / _274;
    float _279;
    float _282;
    int _284;
    _279 = 0.0f;
    _282 = 0.0f;
    _284 = -1;
    float _280;
    float _283;
    for (; _284 <= 1; _279 = _280, _282 = _283, _284++)
    {
        _280 = _279;
        _283 = _282;
        for (int _293 = -1; _293 <= 1; )
        {
            float _302 = exp(float((_293 * _293) + (_284 * _284)) * (-0.75f));
            _280 += (reprojected_history_tex_lum.SampleLevel(sampler_lnc, _155 + ((float2(float(_293), float(_284)) * 2.0f) / float2(_Globals_color_tex_dims)), 0.0f).x * _302);
            _283 += _302;
            _293++;
            continue;
        }
    }
    float _317 = clamp(sqrt(_279 / _282), 0.0f, 1.0f);
    float _318 = _317 * _317;
    float4 _322 = blurred_luminance_tex.SampleLevel(sampler_lnc, _155, 0.0f);
    float _323 = _322.x;
    float _324 = _323 * _323;
    float _335;
    float _336;
    if (WaveActiveMax(WaveGetLaneIndex()) >= 31u)
    {
        uint _330 = WaveGetLaneIndex() ^ 16u;
        _335 = lerp(_324, WaveReadLaneAt(_324, _330), 0.5f);
        _336 = lerp(_318, WaveReadLaneAt(_318, _330), 0.5f);
    }
    else
    {
        _335 = _324;
        _336 = _318;
    }
    uint _337 = WaveGetLaneIndex() ^ 2u;
    float _338 = WaveReadLaneAt(_336, _337);
    float _339 = lerp(_336, _338, 0.5f);
    float _340 = WaveReadLaneAt(_335, _337);
    float _341 = lerp(_335, _340, 0.5f);
    float3 _345 = sqrt(max(0.0f.xxx, (_221 / _274) - (_275 * _275)));
    float2 _349 = 0.5f.xx / float2(_Globals_reprojection_tex_dims);
    float3 _350 = _345 * 2.0f;
    float3 _352 = (_275 + _350) + 0.001000000047497451305389404296875f.xxx;
    float3 _354 = (_275 - _350) - 0.001000000047497451305389404296875f.xxx;
    float _356;
    float _359;
    float3 _361;
    float3 _363;
    float _365;
    int _367;
    _356 = 0.0f;
    _359 = 0.0f;
    _361 = 0.0f.xxx;
    _363 = 0.0f.xxx;
    _365 = 0.0f;
    _367 = -1;
    float _357;
    float _360;
    float3 _362;
    float3 _364;
    float _366;
    for (; _367 <= 1; _356 = _357, _359 = _360, _361 = _362, _363 = _364, _365 = _366, _367++)
    {
        _357 = _356;
        _360 = _359;
        _362 = _361;
        _364 = _363;
        _366 = _365;
        for (int _379 = -1; _379 <= 1; )
        {
            uint2 _385 = gl_GlobalInvocationID.xy + uint2(int2(_379, _367));
            float4 _386 = reprojected_history_tex_lum.Load(int3(_385, 0u));
            float _387 = _386.x;
            float3 _390 = float3(_387, _145, reprojected_history_tex_chroma.Load(int3(_385, 0u)).y);
            _357 += (_387 * _387);
            _360 += _387;
            _362 += (select(bool3(_390.x < _354.x, _390.y < _354.y, _390.z < _354.z), float3(1.0f, 1.0f, 1.0f), float3(0.0f, 0.0f, 0.0f)) * 1.0f);
            _364 += (select(bool3(_390.x > _352.x, _390.y > _352.y, _390.z > _352.z), float3(1.0f, 1.0f, 1.0f), float3(0.0f, 0.0f, 0.0f)) * 1.0f);
            _366 += 1.0f;
            _379++;
            continue;
        }
    }
    float3 _398 = _365.xxx;
    float _401 = _359 / _365;
    float _406 = sqrt(max(0.0f, (_356 / _365) - (_401 * _401)));
    float4 _408 = reprojection_tex.GatherAlpha(sampler_lnc, _155);
    float _422 = clamp((20.0f * max(0.0f, _406)) / max(9.9999999392252902907785028219223e-09f, _401), 0.0f, 1.0f);
    float2 _429 = (_363 / _398).xz + (_361 / _398).xz;
    float _435 = abs(_339 - _341);
    float _439 = (2.0f * _435) / max(9.9999997473787516355514526367188e-06f, _341 + _339);
    float _441 = _275.x;
    float _457;
    float _460;
    int _462;
    _457 = 0.0f;
    _460 = 0.0f;
    _462 = -1;
    float _458;
    float _461;
    for (; _462 <= 1; _457 = _458, _460 = _461, _462++)
    {
        _458 = _457;
        _461 = _460;
        for (int _471 = -1; _471 <= 1; )
        {
            _458 += ((reprojection_tex.SampleLevel(sampler_nnc, _155 - ((_349 * 1.0f) * float2(float(_471), float(_462))), 0.0f).z == 0.0f) ? 1.0f : 0.0f);
            _461 += 1.0f;
            _471++;
            continue;
        }
    }
    float2 _488 = _165 * _152;
    float2 _489 = abs(_488);
    float4 _507 = reprojection_tex.GatherBlue(sampler_lnc, _155);
    float _517 = (((3.0f * clamp(1.0f - ((2.5f * _406) / _401), 0.0f, 1.0f)) * _435) / (0.5f * max(9.9999997473787516355514526367188e-05f, _339 + _341))) * ((min(min(_507.x, _507.y), min(_507.z, _507.w)) == 1.0f) ? 1.0f : 0.0f);
    float _520 = ((((((_185.x * 2.0f) - 0.5f) + clamp(1.0f - _422, 0.0f, 1.0f)) + max(_429.x, _429.y)) + ((_439 * _439) * max(0.0f, 1.0f - ((((-3.0f) * abs(_441 - _341)) + (2.0f * abs((_230.x / _228) - _441))) / _341)))) + (sqrt(_457 / _460) * clamp(max(_489.x, _489.y), 0.0f, 1.0f))) + ((_517 * _517) * _517);
    bool _521 = !_158;
    float _540;
    float _541;
    if (_521)
    {
        float _529;
        float _530;
        if (clamp(1.0f - min(min(_408.x, _408.y), min(_408.z, _408.w)), 0.0f, 1.0f) < 0.75f)
        {
            _529 = _520 + 1.0f;
            _530 = min(_189, 0.5f);
        }
        else
        {
            _529 = _520;
            _530 = _189;
        }
        float _531 = _530 * _422;
        float2 _533 = abs(_488 * 0.03125f.xx);
        _540 = min(_531, lerp(_531, 1.0f, clamp(max(_533.x, _533.y), 0.0f, 1.0f)));
        _541 = _529;
    }
    else
    {
        _540 = _189;
        _541 = _520;
    }
    float3 _553 = _345 * exp2((7.0f * clamp(1.0f - _541, 0.0f, 1.0f)) - 0.5f);
    float3 _560;
    if (_521)
    {
        _560 = clamp(_197, _275 - _553, _275 + _553);
    }
    else
    {
        _560 = _197;
    }
    float _561 = min(4.0f, _540);
    float _562 = _228 + _561;
    bool _577;
    if (_169)
    {
        _577 = !(0u == frame_constants_frame_index);
    }
    else
    {
        _577 = false;
    }
    float3 _586;
    if (!_577)
    {
        _586 = max(0.0f.xxx, mul(float3x3(float3(1.0f, 0.0f, 1.574800014495849609375f), float3(1.0f, -0.1872999966144561767578125f, -0.468100011348724365234375f), float3(1.0f, 1.85559999942779541015625f, 0.0f)), _230.xyz / max(9.9999997473787516355514526367188e-06f, _228).xxx));
    }
    else
    {
        _586 = max(0.0f.xxx, mul(float3x3(float3(1.0f, 0.0f, 1.574800014495849609375f), float3(1.0f, -0.1872999966144561767578125f, -0.468100011348724365234375f), float3(1.0f, 1.85559999942779541015625f, 0.0f)), lerp(_230.xyz / _228.xxx, _560, (_561 / max(9.9999997473787516355514526367188e-06f, _562)).xxx)));
    }
    FrexpStructType _587;
    { int3 _fe; _587.mantissa = frexp(_586, _fe); _587.exponent = _fe; }
    float3 _592 = _586 + exp2(float3(_587.exponent) - float3(8.0f, 8.0f, 7.0f));
    history_output_tex[gl_GlobalInvocationID.xy] = _592;
    output_tex[gl_GlobalInvocationID.xy] = _592;
    aux_output_tex[gl_GlobalInvocationID.xy] = float2(clamp(min(_541, 1.0f) + (2.0f * clamp(_164.w * 1000.0f, 0.0f, 1.0f)), 0.0f, 2.0f) * 0.5f, _562 / _160);
}

[numthreads(8, 4, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f7efabb81dfe256b) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"


float3 incb_frexp(float3 x, out int3 exponent)
{
    
    uint3 bits0 = asuint(x);
    bool3 denorm = ((bits0 & 0x7F800000u) == 0u) & ((bits0 & 0x7FFFFFFFu) != 0u);
    float3 xs = select(denorm, x * 16777216.0f, x);
    uint3 bits = asuint(xs);
    int3 biased = int3(int((bits.x >> 23u) & 0xFFu), int((bits.y >> 23u) & 0xFFu), int((bits.z >> 23u) & 0xFFu));
    int3 e = biased - int3(127, 127, 127) - select(denorm, int3(24, 24, 24), int3(0, 0, 0));
    float3 m = asfloat((bits & 0x807FFFFFu) | 0x3F800000u);
    bool3 big = (m >= 1.0f) | (m <= -1.0f);
    m = lerp(m, m * 0.5f, float3(big));
    e += int3(big);
    bool3 special = (biased == 0) | (biased == 0xFF);
    exponent = select(special, int3(0, 0, 0), e);
    return select(special, xs, m);
}
#define frexp incb_frexp
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

struct FrexpStructType
{
    float3 mantissa;
    int3 exponent;
};

static float _157;
static float _163;

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
    uint2 _Globals_reprojected_history_tex_lum_dims : packoffset(c0);
    uint2 _Globals_reprojected_history_tex_chroma_dims : packoffset(c0.z);
    uint2 _Globals_color_tex_dims : packoffset(c1);
    uint2 _Globals_blurred_luminance_tex_dims : packoffset(c1.z);
    uint2 _Globals_history_tex_dims : packoffset(c2);
    uint2 _Globals_reprojection_tex_dims : packoffset(c2.z);
    uint2 _Globals_aux_tex_dims : packoffset(c3);
    uint2 _Globals_history_output_tex_dims : packoffset(c3.z);
    uint2 _Globals_output_tex_dims : packoffset(c4);
    uint2 _Globals_aux_output_tex_dims : packoffset(c4.z);
    uint _Globals_bypass_neighborhood_clamping : packoffset(c5);
    float _Globals_sharpen_amount : packoffset(c5.y);
    uint _Globals_history_valid : packoffset(c5.z);
};

SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_nnc : register(s2, space0);
Texture2D<float4> reprojected_history_tex_lum : register(t14, space0);
Texture2D<float4> reprojected_history_tex_chroma : register(t15, space0);
Texture2D<float4> color_tex : register(t16, space0);
Texture2D<float4> blurred_luminance_tex : register(t17, space0);
Texture2D<float4> reprojection_tex : register(t19, space0);
Texture2D<float4> aux_tex : register(t20, space0);
RWTexture2D<float3> history_output_tex : register(u21, space0);
RWTexture2D<float3> output_tex : register(u22, space0);
RWTexture2D<unorm float2> aux_output_tex : register(u23, space0);

static uint3 gl_LocalInvocationID;
static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_LocalInvocationID : SV_GroupThreadID;
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

groupshared float tile_luminance[8][4];

void comp_main()
{
    float2 _172 = float2(_Globals_output_tex_dims);
    float2 _174 = float2(int2(gl_GlobalInvocationID.xy)) + 0.5f.xx;
    float2 _175 = _174 / _172;
    bool _178 = _Globals_bypass_neighborhood_clamping != 0u;
    float _180 = float(_178 ? 16 : 4);
    float4 _184 = reprojection_tex.SampleLevel(sampler_nnc, _175, 0.0f);
    float2 _185 = _184.xy;
    bool _189 = _Globals_history_valid != 0u;
    bool _196;
    if (_189)
    {
        _196 = !(0u == frame_constants_frame_index);
    }
    else
    {
        _196 = false;
    }
    float2 _205;
    if (_196)
    {
        _205 = aux_tex.SampleLevel(sampler_lnc, _175 + _185, 0.0f).xy;
    }
    else
    {
        _205 = 0.0f.xx;
    }
    float _209 = _205.y * _180;
    float3 _217 = float3(reprojected_history_tex_lum.Load(int3(gl_GlobalInvocationID.xy, 0u)).x, reprojected_history_tex_chroma.Load(int3(gl_GlobalInvocationID.xy, 0u)).xy);
    float2 _225 = float2(int2(_Globals_color_tex_dims)) / _172;
    int2 _227 = int2(_174 * _225);
    float2 _232 = ((float2(_227) + 0.5f.xx) + (frame_constants_view_constants[0u].sample_offset_pixels * float2(1.0f, -1.0f))) / _225;
    float _233 = _225.x;
    float _237 = ((-10.0f) * _233) * min(4.0f * _233, 1.0f);
    float _239 = (-1.0f) * (_233 * _233);
    float3 _241;
    float _244;
    float3 _246;
    float _248;
    float4 _250;
    int _252;
    _241 = 0.0f.xxx;
    _244 = 0.0f;
    _246 = 0.0f.xxx;
    _248 = 0.0f;
    _250 = 0.0f.xxxx;
    _252 = -1;
    float3 _242;
    float _245;
    float3 _247;
    float _249;
    float4 _251;
    for (; _252 <= 1; _241 = _242, _244 = _245, _246 = _247, _248 = _249, _250 = _251, _252++)
    {
        _245 = _244;
        _242 = _241;
        _247 = _246;
        _249 = _248;
        _251 = _250;
        for (int _264 = -1; _264 <= 1; )
        {
            float4 _282 = float4(mul(float3x3(float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f), float3(-0.114600002765655517578125f, -0.385399997234344482421875f, 0.5f), float3(0.5f, -0.4541999995708465576171875f, -0.0458000004291534423828125f)), color_tex.Load(int3(uint2(_227 + int2(_264, _252)), 0u)).xyz), _157);
            float2 _283 = (_232 + (float2(float(_264), float(_252)) / _225)) - _174;
            float _284 = dot(_283, _283);
            float _286 = exp2(_284 * _239);
            float _288 = exp2(_284 * _237);
            float3 _290 = _282.xyz;
            _245 += _286;
            _242 += ((_290 * _290) * _286);
            _247 += (_290 * _286);
            _249 += _288;
            _251 += (_282 * _288);
            _264++;
            continue;
        }
    }
    float3 _294 = _244.xxx;
    float3 _295 = _246 / _294;
    float3 _297 = _250.xyz;
    float _299;
    float _302;
    int _304;
    _299 = 0.0f;
    _302 = 0.0f;
    _304 = -1;
    float _300;
    float _303;
    for (; _304 <= 1; _299 = _300, _302 = _303, _304++)
    {
        _300 = _299;
        _303 = _302;
        for (int _313 = -1; _313 <= 1; )
        {
            float _322 = exp(float((_313 * _313) + (_304 * _304)) * (-0.75f));
            _300 += (reprojected_history_tex_lum.SampleLevel(sampler_lnc, _175 + ((float2(float(_313), float(_304)) * 2.0f) / float2(_Globals_color_tex_dims)), 0.0f).x * _322);
            _303 += _322;
            _313++;
            continue;
        }
    }
    float _337 = clamp(sqrt(_299 / _302), 0.0f, 1.0f);
    float _338 = _337 * _337;
    float4 _342 = blurred_luminance_tex.SampleLevel(sampler_lnc, _175, 0.0f);
    float _343 = _342.x;
    float _344 = _343 * _343;
    float _355;
    float _356;
    if (WaveActiveMax(WaveGetLaneIndex()) >= 31u)
    {
        uint _350 = WaveGetLaneIndex() ^ 16u;
        _355 = lerp(_344, WaveReadLaneAt(_344, _350), 0.5f);
        _356 = lerp(_338, WaveReadLaneAt(_338, _350), 0.5f);
    }
    else
    {
        _355 = _344;
        _356 = _338;
    }
    uint _357 = WaveGetLaneIndex() ^ 2u;
    float _358 = WaveReadLaneAt(_356, _357);
    float _359 = lerp(_356, _358, 0.5f);
    float _360 = WaveReadLaneAt(_355, _357);
    float _361 = lerp(_355, _360, 0.5f);
    float3 _365 = sqrt(max(0.0f.xxx, (_241 / _294) - (_295 * _295)));
    float2 _369 = 0.5f.xx / float2(_Globals_reprojection_tex_dims);
    float3 _370 = _365 * 2.0f;
    float3 _372 = (_295 + _370) + 0.001000000047497451305389404296875f.xxx;
    float3 _374 = (_295 - _370) - 0.001000000047497451305389404296875f.xxx;
    float _376;
    float _379;
    float3 _381;
    float3 _383;
    float _385;
    int _387;
    _376 = 0.0f;
    _379 = 0.0f;
    _381 = 0.0f.xxx;
    _383 = 0.0f.xxx;
    _385 = 0.0f;
    _387 = -1;
    float _377;
    float _380;
    float3 _382;
    float3 _384;
    float _386;
    for (; _387 <= 1; _376 = _377, _379 = _380, _381 = _382, _383 = _384, _385 = _386, _387++)
    {
        _377 = _376;
        _380 = _379;
        _382 = _381;
        _384 = _383;
        _386 = _385;
        for (int _399 = -1; _399 <= 1; )
        {
            uint2 _405 = gl_GlobalInvocationID.xy + uint2(int2(_399, _387));
            float4 _406 = reprojected_history_tex_lum.Load(int3(_405, 0u));
            float _407 = _406.x;
            float3 _410 = float3(_407, _163, reprojected_history_tex_chroma.Load(int3(_405, 0u)).y);
            _377 += (_407 * _407);
            _380 += _407;
            _382 += (select(bool3(_410.x < _374.x, _410.y < _374.y, _410.z < _374.z), float3(1.0f, 1.0f, 1.0f), float3(0.0f, 0.0f, 0.0f)) * 1.0f);
            _384 += (select(bool3(_410.x > _372.x, _410.y > _372.y, _410.z > _372.z), float3(1.0f, 1.0f, 1.0f), float3(0.0f, 0.0f, 0.0f)) * 1.0f);
            _386 += 1.0f;
            _399++;
            continue;
        }
    }
    float3 _418 = _385.xxx;
    float _421 = _379 / _385;
    float _426 = sqrt(max(0.0f, (_376 / _385) - (_421 * _421)));
    float4 _428 = reprojection_tex.GatherAlpha(sampler_lnc, _175);
    float _442 = clamp((20.0f * max(0.0f, _426)) / max(9.9999999392252902907785028219223e-09f, _421), 0.0f, 1.0f);
    float2 _449 = (_383 / _418).xz + (_381 / _418).xz;
    float _455 = abs(_359 - _361);
    float _459 = (2.0f * _455) / max(9.9999997473787516355514526367188e-06f, _361 + _359);
    float _461 = _295.x;
    float _477;
    float _480;
    int _482;
    _477 = 0.0f;
    _480 = 0.0f;
    _482 = -1;
    float _478;
    float _481;
    for (; _482 <= 1; _477 = _478, _480 = _481, _482++)
    {
        _478 = _477;
        _481 = _480;
        for (int _491 = -1; _491 <= 1; )
        {
            _478 += ((reprojection_tex.SampleLevel(sampler_nnc, _175 - ((_369 * 1.0f) * float2(float(_491), float(_482))), 0.0f).z == 0.0f) ? 1.0f : 0.0f);
            _481 += 1.0f;
            _491++;
            continue;
        }
    }
    float2 _508 = _185 * _172;
    float2 _509 = abs(_508);
    float4 _527 = reprojection_tex.GatherBlue(sampler_lnc, _175);
    float _537 = (((3.0f * clamp(1.0f - ((2.5f * _426) / _421), 0.0f, 1.0f)) * _455) / (0.5f * max(9.9999997473787516355514526367188e-05f, _359 + _361))) * ((min(min(_527.x, _527.y), min(_527.z, _527.w)) == 1.0f) ? 1.0f : 0.0f);
    float _540 = ((((((_205.x * 2.0f) - 0.5f) + clamp(1.0f - _442, 0.0f, 1.0f)) + max(_449.x, _449.y)) + ((_459 * _459) * max(0.0f, 1.0f - ((((-3.0f) * abs(_461 - _361)) + (2.0f * abs((_250.x / _248) - _461))) / _361)))) + (sqrt(_477 / _480) * clamp(max(_509.x, _509.y), 0.0f, 1.0f))) + ((_537 * _537) * _537);
    bool _541 = !_178;
    float _560;
    float _561;
    if (_541)
    {
        float _549;
        float _550;
        if (clamp(1.0f - min(min(_428.x, _428.y), min(_428.z, _428.w)), 0.0f, 1.0f) < 0.75f)
        {
            _549 = _540 + 1.0f;
            _550 = min(_209, 0.5f);
        }
        else
        {
            _549 = _540;
            _550 = _209;
        }
        float _551 = _550 * _442;
        float2 _553 = abs(_508 * 0.03125f.xx);
        _560 = min(_551, lerp(_551, 1.0f, clamp(max(_553.x, _553.y), 0.0f, 1.0f)));
        _561 = _549;
    }
    else
    {
        _560 = _209;
        _561 = _540;
    }
    float _567 = min(_561, 1.0f);
    float3 _573 = _365 * exp2((7.0f * clamp(1.0f - _561, 0.0f, 1.0f)) - 0.5f);
    float3 _580;
    if (_541)
    {
        _580 = clamp(_217, _295 - _573, _295 + _573);
    }
    else
    {
        _580 = _217;
    }
    float _581 = min(4.0f, _560);
    float _582 = _248 + _581;
    float3 _585 = _248.xxx;
    float3 _588 = lerp(_297 / _585, _580, (_581 / max(9.9999997473787516355514526367188e-06f, _582)).xxx);
    float _589 = _588.x;
    bool _598;
    if (_189)
    {
        _598 = !(0u == frame_constants_frame_index);
    }
    else
    {
        _598 = false;
    }
    float3 _607;
    if (!_598)
    {
        _607 = max(0.0f.xxx, mul(float3x3(float3(1.0f, 0.0f, 1.574800014495849609375f), float3(1.0f, -0.1872999966144561767578125f, -0.468100011348724365234375f), float3(1.0f, 1.85559999942779541015625f, 0.0f)), _297 / max(9.9999997473787516355514526367188e-06f, _248).xxx));
    }
    else
    {
        _607 = max(0.0f.xxx, mul(float3x3(float3(1.0f, 0.0f, 1.574800014495849609375f), float3(1.0f, -0.1872999966144561767578125f, -0.468100011348724365234375f), float3(1.0f, 1.85559999942779541015625f, 0.0f)), _588));
    }
    FrexpStructType _608;
    { int3 _fe; _608.mantissa = frexp(_607, _fe); _608.exponent = _fe; }
    history_output_tex[gl_GlobalInvocationID.xy] = _607 + exp2(float3(_608.exponent) - float3(8.0f, 8.0f, 7.0f));
    tile_luminance[gl_LocalInvocationID.x][gl_LocalInvocationID.y] = _589;
    GroupMemoryBarrierWithGroupSync();
    uint2 _618 = gl_LocalInvocationID.xy + uint2(1u, 0u);
    uint2 _619 = gl_LocalInvocationID.xy - uint2(1u, 0u);
    uint _620 = _618.x;
    uint _621 = _618.y;
    uint _622 = _619.x;
    uint _623 = _619.y;
    uint4 _624 = uint4(_620, _621, _622, _623);
    float _649;
    float _650;
    if (all(bool4(_624.x < uint4(8u, 4u, 8u, 4u).x, _624.y < uint4(8u, 4u, 8u, 4u).y, _624.z < uint4(8u, 4u, 8u, 4u).z, _624.w < uint4(8u, 4u, 8u, 4u).w)))
    {
        float _640 = max(0.0f, 1.0f - (6.0f * (abs(_589 - tile_luminance[_620][_621]) + abs(_589 - tile_luminance[_622][_623]))));
        float _645 = min(_640, (_Globals_sharpen_amount * _640) * 1.25f);
        _649 = _645 * (tile_luminance[_620][_621] + tile_luminance[_622][_623]);
        _650 = _645 * 2.0f;
    }
    else
    {
        _649 = 0.0f;
        _650 = 0.0f;
    }
    uint2 _651 = gl_LocalInvocationID.xy + uint2(0u, 1u);
    uint2 _652 = gl_LocalInvocationID.xy - uint2(0u, 1u);
    uint _653 = _651.x;
    uint _654 = _651.y;
    uint _655 = _652.x;
    uint _656 = _652.y;
    uint4 _657 = uint4(_653, _654, _655, _656);
    float _685;
    float _686;
    if (all(bool4(_657.x < uint4(8u, 4u, 8u, 4u).x, _657.y < uint4(8u, 4u, 8u, 4u).y, _657.z < uint4(8u, 4u, 8u, 4u).z, _657.w < uint4(8u, 4u, 8u, 4u).w)))
    {
        float _673 = max(0.0f, 1.0f - (6.0f * (abs(_589 - tile_luminance[_653][_654]) + abs(_589 - tile_luminance[_655][_656]))));
        float _678 = min(_673, (_Globals_sharpen_amount * _673) * 1.25f);
        _685 = (_649 + (tile_luminance[_653][_654] * _678)) + (tile_luminance[_655][_656] * _678);
        _686 = _650 + (_678 * 2.0f);
    }
    else
    {
        _685 = _649;
        _686 = _650;
    }
    float3 _694 = _607 * max(0.0f, max(0.0f, (_589 * (_686 + 1.0f)) - _685) / max(9.9999997473787516355514526367188e-06f, _589));
    FrexpStructType _695;
    { int3 _fe; _695.mantissa = frexp(_694, _fe); _695.exponent = _fe; }
    output_tex[gl_GlobalInvocationID.xy] = _694 + exp2(float3(_695.exponent) - float3(8.0f, 8.0f, 7.0f));
    aux_output_tex[gl_GlobalInvocationID.xy] = float2(clamp(_567 + (2.0f * clamp(_184.w * 1000.0f, 0.0f, 1.0f)), 0.0f, 2.0f) * 0.5f, _582 / _180);
}

[numthreads(8, 4, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_LocalInvocationID = stage_input.gl_LocalInvocationID;
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
