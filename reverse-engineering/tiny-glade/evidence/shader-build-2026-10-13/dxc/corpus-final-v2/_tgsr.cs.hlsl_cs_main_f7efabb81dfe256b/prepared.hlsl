#line 1 "shaders/tgsr.cs.hlsl"






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
#line 7 "shaders/tgsr.cs.hlsl"

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
#line 8 "shaders/tgsr.cs.hlsl"

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
#line 9 "shaders/tgsr.cs.hlsl"

#line 10 "shaders/tgsr.cs.hlsl"

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
#line 11 "shaders/tgsr.cs.hlsl"

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
#line 12 "shaders/tgsr.cs.hlsl"

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
#line 13 "shaders/tgsr.cs.hlsl"

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
#line 14 "shaders/tgsr.cs.hlsl"

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
#line 15 "shaders/tgsr.cs.hlsl"

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
#line 16 "shaders/tgsr.cs.hlsl"

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
#line 17 "shaders/tgsr.cs.hlsl"

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
#line 18 "shaders/tgsr.cs.hlsl"

#line 1 "shaders/tgsr_common.inc.hlsl"



#ifndef TINYGLADE_TGSR_COMMON_INC_HLSL
#define TINYGLADE_TGSR_COMMON_INC_HLSL

#endif 
#line 19 "shaders/tgsr.cs.hlsl"

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
#line 20 "shaders/tgsr.cs.hlsl"

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
#line 21 "shaders/tgsr.cs.hlsl"


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
