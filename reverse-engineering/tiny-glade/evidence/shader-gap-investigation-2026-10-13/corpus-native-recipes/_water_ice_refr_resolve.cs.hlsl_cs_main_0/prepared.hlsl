#define TGV_0 1
#line 1 "shaders/water/ice_refr_resolve.cs.hlsl"


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
#line 18 "shaders/water/ice_refr_resolve.cs.hlsl"

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
#line 19 "shaders/water/ice_refr_resolve.cs.hlsl"

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
#line 20 "shaders/water/ice_refr_resolve.cs.hlsl"

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
#line 21 "shaders/water/ice_refr_resolve.cs.hlsl"

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
#line 22 "shaders/water/ice_refr_resolve.cs.hlsl"

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
#line 23 "shaders/water/ice_refr_resolve.cs.hlsl"

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
#line 24 "shaders/water/ice_refr_resolve.cs.hlsl"

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
#line 25 "shaders/water/ice_refr_resolve.cs.hlsl"

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
#line 26 "shaders/water/ice_refr_resolve.cs.hlsl"

#line 1 "shaders/inc/brdf.hlsl"


#ifndef TINYGLADE_INC_BRDF_HLSL
#define TINYGLADE_INC_BRDF_HLSL



float distribution_ggx(float NdotH, float alphaSq)
{
    float d = NdotH * NdotH * (alphaSq - 1.0) + 1.0;
    return alphaSq / (3.14159265358979323846 * d * d);
}


float visibility_smith_ggx(float NdotL, float NdotV, float alphaSq)
{
    float l2 = NdotL * NdotL;
    float v2 = NdotV * NdotV;
    return (2.0 * NdotL * NdotV) /
        (NdotL * sqrt((1.0 - alphaSq) * v2 + alphaSq) +
         NdotV * sqrt((1.0 - alphaSq) * l2 + alphaSq));
}


float3 fresnel_schlick(float cosTheta, float3 F0)
{
    float c = 1.0 - saturate(cosTheta);
    return F0 + (1.0 - F0) * (c * c * c * c * c);
}



float diffuse_ggx(float NdotL, float NdotV, float NdotH, float roughness)
{
    float r2 = roughness * roughness;
    float f = (2.0 * r2 * NdotH * NdotH) - 0.5;
    float omL = 1.0 - saturate(NdotL);
    float omV = 1.0 - saturate(NdotV);
    return ((1.0 + f * pow(omL, 5.0)) * (1.0 + f * pow(omV, 5.0))) * (1.0 / 3.14159265358979323846);
}


float3 cook_torrance_specular(float NdotL, float NdotV, float NdotH, float HdotL, float roughness)
{
    float alphaSq = roughness * roughness;
    alphaSq = alphaSq * alphaSq; 
    float3 F = fresnel_schlick(HdotL, float3(0.04, 0.04, 0.04));
    float D = distribution_ggx(NdotH, alphaSq);
    float G = visibility_smith_ggx(NdotL, NdotV, alphaSq);
    return (F * D * G) / max(4.0 * NdotV * NdotL, 1e-6);
}


float3 evaluate_brdf(float3 albedo, float roughness, float3 N, float3 V, float3 L)
{
    float NdotL = saturate(dot(N, L));
    float NdotV = saturate(dot(N, V));
    float3 H = normalize(L + V);
    float NdotH = saturate(dot(N, H));
    float HdotL = saturate(dot(H, L));
    float3 diffuse = albedo * diffuse_ggx(NdotL, NdotV, NdotH, roughness);
    float3 specular = cook_torrance_specular(NdotL, NdotV, NdotH, HdotL, roughness);
    return diffuse + specular;
}

#endif 
#line 27 "shaders/water/ice_refr_resolve.cs.hlsl"

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
#line 28 "shaders/water/ice_refr_resolve.cs.hlsl"

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
#line 29 "shaders/water/ice_refr_resolve.cs.hlsl"

#line 1 "shaders/water/ice_shared.inc.hlsl"



















#ifndef TINYGLADE_WATER_ICE_SHARED_HLSL
#define TINYGLADE_WATER_ICE_SHARED_HLSL

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
#line 23 "shaders/water/ice_shared.inc.hlsl"



uint pack_half2x16(float2 value)
{
    uint2 packed = f32tof16(value);
    return packed.x | (packed.y << 16u);
}


float2 unpack_half2x16(uint value)
{
    return f16tof32(uint2(value & 0xFFFFu, value >> 16u));
}





#define frexp ice_frexp

struct FrexpStructType
{
    float3 mantissa;
    int3 exponent;
};

float3 ice_frexp(float3 x, out int3 exponent)
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

#endif 
#line 30 "shaders/water/ice_refr_resolve.cs.hlsl"


cbuffer $Globals : register(b13, space0)
{
    uint2 depth_tex_dims;
    uint2 refr_hit_tex_dims;
    uint2 refr_hit_radiance_tex_dims;
    uint2 refl_refr_radiance_out_tex_dims;
};

Texture2D<float4> depth_tex : register(t14, space0);
Texture2D<uint4> refr_hit_tex : register(t15, space0);
Texture2D<float4> refr_hit_radiance_tex : register(t16, space0);



[[vk::image_format("r11g11b10f")]] RWTexture2D<float3> refl_refr_radiance_out_tex : register(u17, space0);



float3 swap_yz(float3 v)
{
    return mul(v, float3x3(float3(1.0f, -0.0f, 0.0f),
                          float3(-0.0f, 0.0f, 1.0f),
                          float3(-0.0f, -1.0f, 0.0f)));
}



float normal_z_from_xy(float2 xy)
{
    return asfloat(532487669u + (asint(max(0.0f, 1.0f - dot(xy, xy))) >> 1));
}

[numthreads(8, 8, 1)]
void cs_main(uint3 gtid : SV_DispatchThreadID)
{
    uint4 hit = refr_hit_tex.Load(int3(gtid.xy, 0u));
    if (0u == hit.x)
    {
        return;
    }

    float3 n_local = float3((float((hit.x >> 24u) & 255u) * 0.007843137718737125396728515625f) - 1.0f,
                            (float((hit.x >> 16u) & 255u) * 0.007843137718737125396728515625f) - 1.0f,
                            0.0f);
    n_local.z = normal_z_from_xy(n_local.xy);

    float2 ndc = (((float2(int2(gtid.xy)) + 0.5f) / float2(depth_tex_dims)) - 0.5f) * float2(2.0f, -2.0f);
    float4 view_ray_vs4 = mul(view_constants[0].sample_to_view, float4(ndc, 0.0f, 1.0f));
    float3 view_ray = normalize(mul(view_constants[0].view_to_world, view_ray_vs4).xyz);
    float4 wp4 = mul(view_constants[0].view_to_world,
        mul(view_constants[0].sample_to_view, float4(ndc, depth_tex.Load(int3(gtid.xy, 0u)).x, 1.0f)));
    float3 world_pos = wp4.xyz / wp4.w;

    float3 view_dir = swap_yz(-view_ray);
    float roughness_raylen = length(n_local * unpack_half2x16(hit.x & 0xFFFFu).x);
    float dist = length(mul(view_constants[0].world_to_view, float4(world_pos, 1.0f)).xyz);
    float vdotn = -normalize(view_ray_vs4.xyz).z;
    float blur = max(roughness_raylen,
        (((dist / vdotn) * view_constants[0].clip_to_view[1].y) * 0.20000000298023223876953125f)
            * smoothstep(0.0f, 0.0500000007450580596923828125f * dist, roughness_raylen));
    float proj_scale = (dist * vdotn) * view_constants[0].clip_to_view[1].y;

    float3 refl_dir = swap_yz(-normalize(lerp(float3(0.0f, 1.0f, 0.0f),
        reflect(view_ray, float3(0.0f, 1.0f, 0.0f)), 0.959011614322662353515625f)));
    float3 binorm = normalize(cross(float3(0.0f, 1.0f, 0.0f), refl_dir))
        * max(min((((blur / (blur + dist)) * dist) * vdotn) * 0.0684653222560882568359375f,
            0.100000001490116119384765625f * proj_scale),
            (4.0f / float(refl_refr_radiance_out_tex_dims.y)) * proj_scale);
    float3 tangent = cross(refl_dir, binorm);

    float golden = (float((frame_index * 59u) % 128u) * 1.324717998504638671875f)
        + (dot(float2(gtid.xy & uint2(1u, 1u)), float2(1.0f, 2.0f)) * 0.654508531093597412109375f);

    float4 acc = 0.0f;
    float t = 0.0f;
    for (uint i = 0u; i < 8u; ++i)
    {
        float angle = (float(i) + golden) * 2.3999631404876708984375f;
        float3 sample_pos = world_pos + ((binorm * cos(angle)) + (tangent * sin(angle))) * t;
        float4 s4 = mul(view_constants[0].view_to_sample,
            mul(view_constants[0].world_to_view, float4(sample_pos, 1.0f)));
        uint2 px = uint2(int2(floor((((s4.xyz / s4.w).xy * float2(0.5f, -0.5f)) + 0.5f) * float2(refr_hit_tex_dims))));
        uint4 sample_hit = refr_hit_tex.Load(int3(px, 0u));
        if (sample_hit.x != 0u)
        {
            float3 sn = float3((float((sample_hit.x >> 24u) & 255u) * 0.007843137718737125396728515625f) - 1.0f,
                               (float((sample_hit.x >> 16u) & 255u) * 0.007843137718737125396728515625f) - 1.0f,
                               0.0f);
            sn.z = normal_z_from_xy(sn.xy);
            float3 v = normalize(view_dir + normalize((sn * unpack_half2x16(sample_hit.x & 0xFFFFu).x)
                + swap_yz(sample_pos - world_pos)));
            float weight = 3.1640625820728018879890441894531e-05f / pow((-0.9943749904632568359375f * (v.z * v.z)) + 1.0f, 2.0f);
            acc += float4(refr_hit_radiance_tex.Load(int3(px, 0u)).xyz, 1.0f) * weight;
        }
        t += 0.125f;
    }

    refl_refr_radiance_out_tex[gtid.xy] = (acc.xyz / max(9.9999998245167004418121337039338e-15f, acc.w)).xyz;
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)

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
    uint2 _Globals_depth_tex_dims : packoffset(c0);
    uint2 _Globals_refr_hit_tex_dims : packoffset(c0.z);
    uint2 _Globals_refr_hit_radiance_tex_dims : packoffset(c1);
    uint2 _Globals_refl_refr_radiance_out_tex_dims : packoffset(c1.z);
};

Texture2D<float4> depth_tex : register(t14, space0);
Texture2D<uint4> refr_hit_tex : register(t15, space0);
Texture2D<float4> refr_hit_radiance_tex : register(t16, space0);
RWTexture2D<float3> refl_refr_radiance_out_tex : register(u17, space0);

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
        uint4 _116 = refr_hit_tex.Load(int3(gl_GlobalInvocationID.xy, 0u));
        uint _117 = _116.x;
        if (0u == _117)
        {
            break;
        }
        float3 _134 = float3((float((_117 >> 24u) & 255u) * 0.007843137718737125396728515625f) - 1.0f, (float((_117 >> 16u) & 255u) * 0.007843137718737125396728515625f) - 1.0f, 0.0f);
        float2 _135 = _134.xy;
        _134.z = asfloat(532487669 + (asint(max(0.0f, 1.0f - dot(_135, _135))) >> 1));
        float2 _160 = (((float2(int2(gl_GlobalInvocationID.xy)) + 0.5f.xx) / float2(_Globals_depth_tex_dims)) - 0.5f.xx) * float2(2.0f, -2.0f);
        float _161 = _160.x;
        float4 _164 = mul(frame_constants_view_constants[0u].sample_to_view, float4(_161, _160.y, 0.0f, 1.0f));
        float4 _168 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(_161, _160.y, depth_tex.Load(int3(gl_GlobalInvocationID.xy, 0u)).x, 1.0f)));
        float3 _172 = _168.xyz / _168.w.xxx;
        float3 _183 = normalize(normalize(mul(frame_constants_view_constants[0u].view_to_world, _164).xyz));
        float3 _185 = mul(-_183, float3x3(float3(1.0f, -0.0f, 0.0f), float3(-0.0f, 0.0f, 1.0f), float3(-0.0f, -1.0f, 0.0f)));
        float _186 = length(_134 * spvUnpackHalf2x16(_117 & 65535u).x);
        float _187 = length(mul(frame_constants_view_constants[0u].world_to_view, float4(_172, 1.0f)).xyz);
        float _191 = -normalize(_164.xyz).z;
        float _201 = max(_186, (((_187 / _191) * frame_constants_view_constants[0u].clip_to_view[1].y) * 0.20000000298023223876953125f) * smoothstep(0.0f, 0.0500000007450580596923828125f * _187, _186));
        float _208 = (_187 * _191) * frame_constants_view_constants[0u].clip_to_view[1].y;
        float3 _221 = reflect(-normalize(lerp(float3(0.0f, 1.0f, 0.0f), reflect(_183, float3(0.0f, 1.0f, 0.0f)), 0.959011614322662353515625f.xxx)), float3(0.0f, 1.0f, 0.0f));
        float3 _224 = normalize(cross(float3(0.0f, 1.0f, 0.0f), _221)) * max(min((((_201 / (_201 + _187)) * _187) * _191) * 0.0684653222560882568359375f, 0.100000001490116119384765625f * _208), (4.0f / float(_Globals_refl_refr_radiance_out_tex_dims.y)) * _208);
        float3 _225 = cross(_221, _224);
        float _236 = (float((frame_constants_frame_index * 59u) % 128u) * 1.324717998504638671875f) + (dot(float2(gl_GlobalInvocationID.xy & uint2(1u, 1u)), float2(1.0f, 2.0f)) * 0.654508531093597412109375f);
        float4 _238;
        _238 = 0.0f.xxxx;
        float4 _239;
        float _241 = 0.0f;
        int _243 = 0;
        for (; uint(_243) < 8u; _238 = _239, _241 += 0.125f, _243++)
        {
            float _251 = (float(_243) + _236) * 2.3999631404876708984375f;
            float3 _258 = _172 + (((_224 * cos(_251)) + (_225 * sin(_251))) * _241);
            float4 _266 = mul(frame_constants_view_constants[0u].view_to_sample, mul(frame_constants_view_constants[0u].world_to_view, float4(_258, 1.0f)));
            uint2 _280 = uint2(int2(floor((((_266.xyz / _266.w.xxx).xy * float2(0.5f, -0.5f)) + 0.5f.xx) * float2(_Globals_refr_hit_tex_dims))));
            uint4 _281 = refr_hit_tex.Load(int3(_280, 0u));
            uint _282 = _281.x;
            if (_282 != 0u)
            {
                float3 _299 = float3((float((_282 >> 24u) & 255u) * 0.007843137718737125396728515625f) - 1.0f, (float((_282 >> 16u) & 255u) * 0.007843137718737125396728515625f) - 1.0f, 0.0f);
                float2 _300 = _299.xy;
                _299.z = asfloat(532487669 + (asint(max(0.0f, 1.0f - dot(_300, _300))) >> 1));
                float3 _315 = normalize(_185 + normalize((_299 * spvUnpackHalf2x16(_282 & 65535u).x) + mul(_258 - _172, float3x3(float3(1.0f, -0.0f, 0.0f), float3(-0.0f, 0.0f, 1.0f), float3(-0.0f, -1.0f, 0.0f)))));
                float _316 = _315.z;
                float _319 = ((_316 * _316) * (-0.9943749904632568359375f)) + 1.0f;
                _239 = _238 + (float4(refr_hit_radiance_tex.Load(int3(_280, 0u)).xyz, 1.0f) * (3.1640625820728018879890441894531e-05f / (_319 * _319)));
            }
            else
            {
                _239 = _238;
            }
        }
        refl_refr_radiance_out_tex[gl_GlobalInvocationID.xy] = (_238.xyz / max(9.9999998245167004418121337039338e-15f, _238.w).xxx).xyz;
        break;
    } while(false);
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
