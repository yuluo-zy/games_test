#define TGV_0 1
#line 1 "shaders/taa.cs.hlsl"


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
#line 8 "shaders/taa.cs.hlsl"

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
#line 9 "shaders/taa.cs.hlsl"

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
#line 10 "shaders/taa.cs.hlsl"

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
#line 11 "shaders/taa.cs.hlsl"

#line 12 "shaders/taa.cs.hlsl"

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
#line 13 "shaders/taa.cs.hlsl"

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
#line 14 "shaders/taa.cs.hlsl"

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
#line 15 "shaders/taa.cs.hlsl"

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
#line 16 "shaders/taa.cs.hlsl"

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
#line 17 "shaders/taa.cs.hlsl"

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
#line 18 "shaders/taa.cs.hlsl"

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
#line 19 "shaders/taa.cs.hlsl"


cbuffer $Globals : register(b13, space0)
{
    uint2 color_tex_dims;
    uint2 history_tex_dims;
    uint2 reprojection_tex_dims;
    uint2 history_output_tex_dims;
    uint2 output_tex_dims;
    uint2 temp_debug_tex_dims;
    uint bypass_neighborhood_clamping;
    float sharpen_amount;
    uint history_valid;
};

Texture2D<float4> color_tex : register(t14, space0);
Texture2D<float4> history_tex : register(t15, space0);
Texture2D<float4> reprojection_tex : register(t16, space0);
[[vk::ext_capability(49)]] RWTexture2D<float3> history_output_tex : register(u17, space0);
[[vk::ext_capability(49)]] RWTexture2D<float3> output_tex : register(u18, space0);

groupshared float tile_luminance[8][8];

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_id : SV_DispatchThreadID, uint3 local_id : SV_GroupThreadID)
{
    float2 out_dims = float2(output_tex_dims);
    float2 center = float2(int2(dispatch_id.xy)) + 0.5f;
    float2 reproj_px = ((center / out_dims)
        + reprojection_tex.Load(int3(uint2((float2(dispatch_id.xy) + 0.5f)
            * float2(color_tex_dims / output_tex_dims)), 0)).xy) * out_dims;

    
    float2 base = floor(reproj_px - 0.5f);
    float2 base_c = base + 0.5f;
    float2 frac = reproj_px - base_c;
    float2 h2 = frac * 0.5f;
    float2 w0 = frac * ((-0.5f) + (frac * (1.0f - h2)));
    float2 frac2 = frac * frac;
    float2 frac15 = frac * 1.5f;
    float2 w2 = frac * (0.5f + (frac * (2.0f - frac15)));
    float2 w3 = frac2 * ((-0.5f) + h2);
    float2 w1 = (1.0f + (frac2 * ((-2.5f) + frac15))) + w2;
    float2 left_uv = (base + (-0.5f)) / out_dims;
    float2 right_uv = (base + 2.5f) / out_dims;
    float2 center_uv = (base_c + (w2 / w1)) / out_dims;

    float3 hist = (
        (history_tex.SampleLevel(sampler_default, float2(center_uv.x, left_uv.y), 0.0f).xyz * w1.x * w0.y)
        + (history_tex.SampleLevel(sampler_default, float2(left_uv.x, center_uv.y), 0.0f).xyz * w0.x * w1.y)
        + (history_tex.SampleLevel(sampler_default, center_uv, 0.0f).xyz * w1.x * w1.y)
        + (history_tex.SampleLevel(sampler_default, float2(right_uv.x, center_uv.y), 0.0f).xyz * w3.x * w1.y)
        + (history_tex.SampleLevel(sampler_default, float2(center_uv.x, right_uv.y), 0.0f).xyz * w1.x * w3.y)
    ) / ((((w1.x * w0.y) + (w0.x * w1.y)) + (w1.x * w1.y)) + ((w3.x * w1.y) + (w1.x * w3.y)));

    float3 hist_ycbcr = rgb_to_ycbcr(hist);

    float2 scale = float2(int2(color_tex_dims)) / out_dims;
    int2 current_px = int2(center * scale);
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
            float2 d = (current_pos + (float2(float(dx), float(dy)) / scale)) - center;
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

    float3 mean = sum_c / max(9.9999997473787516e-06f, sum_w1);
    float3 clamp_center = sum_c2.xyz / max(9.9999997473787516e-06f, sum_w2);
    float3 std = sqrt(max(float3(0.0f, 0.0f, 0.0f), (sum_sq / sum_w1) - (mean * mean)));
    float3 clamp_range = (std * lerp(0.5f, 1.0f,
        smoothstep(-0.1f, 0.3f, std.x / (mean.x + 9.9999997473787516e-06f)))) * 1.0f;
    float3 clamp_min = mean - clamp_range;
    float3 clamp_max = mean + clamp_range;

    float blend;
    float3 clamped_ycbcr;
    if (bypass_neighborhood_clamping != 0u)
    {
        blend = 0.125f;
        clamped_ycbcr = hist_ycbcr;
    }
    else
    {
        blend = 0.125f * lerp(0.2f, 1.0f, smoothstep(0.0f, 2.0f,
            min(abs(hist_ycbcr.x - clamp_min.x), abs(hist_ycbcr.x - clamp_max.x))
            / max(max(hist_ycbcr.x, mean.x), 9.9999997473787516e-06f)));
        clamped_ycbcr = clamp(hist_ycbcr, clamp_min, clamp_max);
    }

    bool history_available = (history_valid != 0u) && (frame_index != 0u);
    float3 final_color;
    if (!history_available)
    {
        final_color = max(float3(0.0f, 0.0f, 0.0f), ycbcr_to_rgb(clamp_center));
    }
    else
    {
        final_color = max(float3(0.0f, 0.0f, 0.0f), ycbcr_to_rgb(lerp(clamped_ycbcr, clamp_center, blend)));
    }

    history_output_tex[dispatch_id.xy] = curve(final_color);

    float3 out_color;
    if (sharpen_amount > 0.0f)
    {
        float lum = dot(final_color, float3(0.2126f, 0.7152f, 0.0722f));
        tile_luminance[local_id.x][local_id.y] = lum;
        GroupMemoryBarrierWithGroupSync();

        uint2 right = local_id.xy + uint2(1u, 0u);
        uint2 left = local_id.xy - uint2(1u, 0u);
        float x_accum = 0.0f;
        float x_weight = 0.0f;
        if (all(bool4(right.x < 8u, right.y < 8u, left.x < 8u, left.y < 8u)))
        {
            float w = max(0.0f, 1.0f - (6.0f
                * (abs(lum - tile_luminance[right.x][right.y]) + abs(lum - tile_luminance[left.x][left.y]))));
            float w2 = min(w, (sharpen_amount * w) * 1.25f);
            x_accum = w2 * (tile_luminance[right.x][right.y] + tile_luminance[left.x][left.y]);
            x_weight = w2 * 2.0f;
        }

        uint2 up = local_id.xy + uint2(0u, 1u);
        uint2 down = local_id.xy - uint2(0u, 1u);
        float y_accum = 0.0f;
        float y_weight = 0.0f;
        if (all(bool4(up.x < 8u, up.y < 8u, down.x < 8u, down.y < 8u)))
        {
            float w = max(0.0f, 1.0f - (6.0f
                * (abs(lum - tile_luminance[up.x][up.y]) + abs(lum - tile_luminance[down.x][down.y]))));
            float w2 = min(w, (sharpen_amount * w) * 1.25f);
            y_accum = (x_accum + (tile_luminance[up.x][up.y] * w2)) + (tile_luminance[down.x][down.y] * w2);
            y_weight = x_weight + (w2 * 2.0f);
        }
        else
        {
            y_accum = x_accum;
            y_weight = x_weight;
        }

        out_color = final_color * max(0.0f, max(0.0f,
            (lum * (y_weight + 1.0f)) - y_accum) / max(9.9999997473787516e-06f, lum));
    }
    else
    {
        out_color = final_color;
    }

    output_tex[dispatch_id.xy] = out_color;
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)

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
    uint2 color_tex_dims : packoffset(c0);
    uint2 history_tex_dims : packoffset(c0.z);
    uint2 reprojection_tex_dims : packoffset(c1);
    uint2 history_output_tex_dims : packoffset(c1.z);
    uint2 output_tex_dims : packoffset(c2);
    uint2 temp_debug_tex_dims : packoffset(c2.z);
    uint bypass_neighborhood_clamping : packoffset(c3);
    float sharpen_amount : packoffset(c3.y);
    uint history_valid : packoffset(c3.z);
};

SamplerState sampler_default : register(s9, space0);
Texture2D<float4> color_tex : register(t14, space0);
Texture2D<float4> history_tex : register(t15, space0);
Texture2D<float4> reprojection_tex : register(t16, space0);
RWTexture2D<float3> history_output_tex : register(u17, space0);
RWTexture2D<float3> output_tex : register(u18, space0);

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
    float2 _147 = float2(output_tex_dims);
    float2 _149 = float2(int2(gl_GlobalInvocationID.xy)) + 0.5f.xx;
    float2 _165 = ((_149 / _147) + reprojection_tex.Load(int3(uint2((float2(gl_GlobalInvocationID.xy) + 0.5f.xx) * float2(color_tex_dims / output_tex_dims)), 0u)).xy) * _147;
    float2 _167 = floor(_165 - 0.5f.xx);
    float2 _168 = _167 + 0.5f.xx;
    float2 _169 = _165 - _168;
    float2 _170 = _169 * 0.5f;
    float2 _174 = _169 * ((-0.5f).xx + (_169 * (1.0f.xx - _170)));
    float2 _175 = _169 * _169;
    float2 _176 = _169 * 1.5f;
    float2 _183 = _169 * (0.5f.xx + (_169 * (2.0f.xx - _176)));
    float2 _185 = _175 * ((-0.5f).xx + _170);
    float2 _186 = (1.0f.xx + (_175 * ((-2.5f).xx + _176))) + _183;
    float2 _191 = (_167 + (-0.5f).xx) / _147;
    float2 _192 = (_167 + 2.5f.xx) / _147;
    float2 _193 = (_168 + (_183 / _186)) / _147;
    float _194 = _193.x;
    float _203 = _186.x;
    float _205 = _174.y;
    float _208 = _193.y;
    float _216 = _174.x;
    float _218 = _186.y;
    float _238 = _185.x;
    float _251 = _185.y;
    float3 _265 = (((((((float4(history_tex.SampleLevel(sampler_default, float2(_194, _191.y), 0.0f).xyz, 1.0f) * _203) * _205) + ((float4(history_tex.SampleLevel(sampler_default, float2(_191.x, _208), 0.0f).xyz, 1.0f) * _216) * _218)) + ((float4(history_tex.SampleLevel(sampler_default, _193, 0.0f).xyz, 1.0f) * _203) * _218)) + ((float4(history_tex.SampleLevel(sampler_default, float2(_192.x, _208), 0.0f).xyz, 1.0f) * _238) * _218)) + ((float4(history_tex.SampleLevel(sampler_default, float2(_194, _192.y), 0.0f).xyz, 1.0f) * _203) * _251)) / (((((_203 * _205) + (_216 * _218)) + (_203 * _218)) + (_238 * _218)) + (_203 * _251)).xxxx).xyz;
    float3 _266 = mul(float3x3(float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f), float3(-0.114600002765655517578125f, -0.385399997234344482421875f, 0.5f), float3(0.5f, -0.4541999995708465576171875f, -0.0458000004291534423828125f)), _265);
    float2 _272 = float2(int2(color_tex_dims)) / _147;
    int2 _274 = int2(_149 * _272);
    float2 _279 = ((float2(_274) + 0.5f.xx) + (frame_constants_view_constants[0u].sample_offset_pixels * float2(1.0f, -1.0f))) / _272;
    float _280 = _272.x;
    float _284 = ((-10.0f) * _280) * min(4.0f * _280, 1.0f);
    float _286 = (-1.0f) * (_280 * _280);
    float3 _288;
    float _291;
    float3 _293;
    float _295;
    float4 _297;
    int _299;
    _288 = 0.0f.xxx;
    _291 = 0.0f;
    _293 = 0.0f.xxx;
    _295 = 0.0f;
    _297 = 0.0f.xxxx;
    _299 = -1;
    float3 _289;
    float _292;
    float3 _294;
    float _296;
    float4 _298;
    for (; _299 <= 1; _288 = _289, _291 = _292, _293 = _294, _295 = _296, _297 = _298, _299++)
    {
        _292 = _291;
        _289 = _288;
        _294 = _293;
        _296 = _295;
        _298 = _297;
        for (int _311 = -1; _311 <= 1; )
        {
            float4 _329 = float4(mul(float3x3(float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f), float3(-0.114600002765655517578125f, -0.385399997234344482421875f, 0.5f), float3(0.5f, -0.4541999995708465576171875f, -0.0458000004291534423828125f)), color_tex.Load(int3(uint2(_274 + int2(_311, _299)), 0u)).xyz), 1.0f);
            float2 _330 = (_279 + (float2(float(_311), float(_299)) / _272)) - _149;
            float _331 = dot(_330, _330);
            float _333 = exp2(_331 * _286);
            float _335 = exp2(_331 * _284);
            float3 _337 = _329.xyz;
            _292 += _333;
            _289 += ((_337 * _337) * _333);
            _294 += (_337 * _333);
            _296 += _335;
            _298 += (_329 * _335);
            _311++;
            continue;
        }
    }
    float3 _341 = _291.xxx;
    float3 _342 = _293 / _341;
    float3 _347 = _297.xyz / max(9.9999997473787516355514526367188e-06f, _295).xxx;
    float3 _351 = sqrt(max(0.0f.xxx, (_288 / _341) - (_342 * _342)));
    float _353 = _342.x;
    float3 _359 = (_351 * lerp(0.5f, 1.0f, smoothstep(-0.100000001490116119384765625f, 0.300000011920928955078125f, _351.x / (_353 + 9.9999997473787516355514526367188e-06f)))) * 1.0f;
    float3 _360 = _342 - _359;
    float3 _361 = _342 + _359;
    float _384;
    float3 _385;
    if (!(bypass_neighborhood_clamping != 0u))
    {
        float _370 = _266.x;
        _384 = 0.125f * lerp(0.20000000298023223876953125f, 1.0f, smoothstep(0.0f, 2.0f, min(abs(_370 - _360.x), abs(_370 - _361.x)) / max(max(_370, _353), 9.9999997473787516355514526367188e-06f)));
        _385 = clamp(_266, _360, _361);
    }
    else
    {
        _384 = 0.125f;
        _385 = _266;
    }
    bool _399;
    if (history_valid != 0u)
    {
        _399 = !(0u == frame_constants_frame_index);
    }
    else
    {
        _399 = false;
    }
    float3 _405;
    if (!_399)
    {
        _405 = max(0.0f.xxx, mul(float3x3(float3(1.0f, 0.0f, 1.574800014495849609375f), float3(1.0f, -0.1872999966144561767578125f, -0.468100011348724365234375f), float3(1.0f, 1.85559999942779541015625f, 0.0f)), _347));
    }
    else
    {
        _405 = max(0.0f.xxx, mul(float3x3(float3(1.0f, 0.0f, 1.574800014495849609375f), float3(1.0f, -0.1872999966144561767578125f, -0.468100011348724365234375f), float3(1.0f, 1.85559999942779541015625f, 0.0f)), lerp(_385, _347, _384.xxx)));
    }
    FrexpStructType _406;
    { int3 _fe; _406.mantissa = frexp(_405, _fe); _406.exponent = _fe; }
    history_output_tex[gl_GlobalInvocationID.xy] = _405 + exp2(float3(_406.exponent) - float3(8.0f, 8.0f, 7.0f));
    float3 _495;
    if (sharpen_amount > 0.0f)
    {
        float _418 = dot(_405, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
        tile_luminance[gl_LocalInvocationID.x][gl_LocalInvocationID.y] = _418;
        GroupMemoryBarrierWithGroupSync();
        uint2 _422 = gl_LocalInvocationID.xy + uint2(1u, 0u);
        uint2 _423 = gl_LocalInvocationID.xy - uint2(1u, 0u);
        uint _424 = _422.x;
        uint _425 = _422.y;
        uint _426 = _423.x;
        uint _427 = _423.y;
        uint4 _428 = uint4(_424, _425, _426, _427);
        float _451;
        float _452;
        if (all(bool4(_428.x < uint4(8u, 8u, 8u, 8u).x, _428.y < uint4(8u, 8u, 8u, 8u).y, _428.z < uint4(8u, 8u, 8u, 8u).z, _428.w < uint4(8u, 8u, 8u, 8u).w)))
        {
            float _444 = max(0.0f, 1.0f - (6.0f * (abs(_418 - tile_luminance[_424][_425]) + abs(_418 - tile_luminance[_426][_427]))));
            float _447 = min(_444, (sharpen_amount * _444) * 1.25f);
            _451 = _447 * (tile_luminance[_424][_425] + tile_luminance[_426][_427]);
            _452 = _447 * 2.0f;
        }
        else
        {
            _451 = 0.0f;
            _452 = 0.0f;
        }
        uint2 _453 = gl_LocalInvocationID.xy + uint2(0u, 1u);
        uint2 _454 = gl_LocalInvocationID.xy - uint2(0u, 1u);
        uint _455 = _453.x;
        uint _456 = _453.y;
        uint _457 = _454.x;
        uint _458 = _454.y;
        uint4 _459 = uint4(_455, _456, _457, _458);
        float _485;
        float _486;
        if (all(bool4(_459.x < uint4(8u, 8u, 8u, 8u).x, _459.y < uint4(8u, 8u, 8u, 8u).y, _459.z < uint4(8u, 8u, 8u, 8u).z, _459.w < uint4(8u, 8u, 8u, 8u).w)))
        {
            float _475 = max(0.0f, 1.0f - (6.0f * (abs(_418 - tile_luminance[_455][_456]) + abs(_418 - tile_luminance[_457][_458]))));
            float _478 = min(_475, (sharpen_amount * _475) * 1.25f);
            _485 = (_451 + (tile_luminance[_455][_456] * _478)) + (tile_luminance[_457][_458] * _478);
            _486 = _452 + (_478 * 2.0f);
        }
        else
        {
            _485 = _451;
            _486 = _452;
        }
        _495 = _405 * max(0.0f, max(0.0f, (_418 * (_486 + 1.0f)) - _485) / max(9.9999997473787516355514526367188e-06f, _418));
    }
    else
    {
        _495 = _405;
    }
    output_tex[gl_GlobalInvocationID.xy] = _495;
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
