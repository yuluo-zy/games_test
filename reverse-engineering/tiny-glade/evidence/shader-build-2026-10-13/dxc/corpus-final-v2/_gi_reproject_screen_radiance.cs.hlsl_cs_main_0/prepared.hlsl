#line 1 "shaders/gi/reproject_screen_radiance.cs.hlsl"














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
#line 15 "shaders/gi/reproject_screen_radiance.cs.hlsl"

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
#line 16 "shaders/gi/reproject_screen_radiance.cs.hlsl"

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
#line 17 "shaders/gi/reproject_screen_radiance.cs.hlsl"

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
#line 18 "shaders/gi/reproject_screen_radiance.cs.hlsl"

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
#line 19 "shaders/gi/reproject_screen_radiance.cs.hlsl"

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
#line 20 "shaders/gi/reproject_screen_radiance.cs.hlsl"

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
#line 21 "shaders/gi/reproject_screen_radiance.cs.hlsl"

#line 1 "shaders/gi/gi_settings.inc.hlsl"














#ifndef TINYGLADE_GI_SETTINGS_INC_HLSL
#define TINYGLADE_GI_SETTINGS_INC_HLSL




static const uint2 hi_px_subpixels[4] = {
    uint2(0u, 0u),
    uint2(0u, 1u),
    uint2(1u, 1u),
    uint2(1u, 0u),
};



static const float GI_MIN_HALF_NORMAL_Z = 0.3f;          
static const float GI_DEPTH_WEIGHT_SCALE = 80.0f;        
static const float GI_DEPTH_WEIGHT_MAX = 30.0f;
static const float GI_NORMAL_DOT_DAMP = -0.9f;           
static const float GI_FINAL_WEIGHT_NORM = 0.01f;         

#endif 
#line 22 "shaders/gi/reproject_screen_radiance.cs.hlsl"

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
#line 23 "shaders/gi/reproject_screen_radiance.cs.hlsl"


cbuffer $Globals : register(b13, space0)
{
    uint history_valid;
    uint2 half_depth_tex_dims;
    uint2 half_view_normal_tex_dims;
    uint2 reprojection_tex_dims;
    uint2 history_tex_dims;
    uint2 temporal_stats_tex_dims;
    uint2 output_tex_dims;
    uint2 temporal_stats_out_tex_dims;
    uint allow_non_determinism;
};

Texture2D<float4> half_depth_tex : register(t14, space0);
Texture2D<float4> half_view_normal_tex : register(t15, space0);
Texture2D<float4> reprojection_tex : register(t16, space0);
Texture2D<float4> history_tex : register(t17, space0);
Texture2D<float4> temporal_stats_tex : register(t18, space0);
RWTexture2D<float4> output_tex : register(u19, space0);
RWTexture2D<unorm float4> temporal_stats_out_tex : register(u20, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 globalThreadID : SV_DispatchThreadID)
{
    uint2 xy = globalThreadID.xy;

    float4 n4 = half_view_normal_tex.Load(int3(xy, 0u));
    float3 n = n4.xyz;
    float nw = n4.w;
    float nz = n4.z;

    float3 p = n / (abs(n.x) + abs(n.y) + abs(n.z)).xxx;
    bool flip = p.z < 0.0f;
    float3 q;
    if (flip) {
        float2 s = (1.0f.xx - abs(p.yx)) * ((step(0.0f.xx, p.xy) * 2.0f) - 1.0f.xx);
        q = float3(s.x, s.y, p.z);
    } else {
        q = p;
    }
    float2 reproj_uv = (q.xy * 0.5f) + 0.5f.xx;
    float4 stats = float4(0.0f, reproj_uv.x, reproj_uv.y, 0.0f);

    if (!(history_valid != 0u && frame_index != 0u)) {
        temporal_stats_out_tex[xy] = stats;
        return;
    }

    float2 dims = float2(half_depth_tex_dims);
    float4 reproj = reprojection_tex.Load(int3((xy * uint2(2u, 2u)) + uint2(uint(abs(nw) > 0.75f), uint(nw >= 0.0f)), 0u));
    float2 uv = ((float2(int2(xy)) + 0.5f.xx) / dims) + reproj.xy;
    float rjz = reproj.z;

    float4 stats_out;
    bool updated;
    if (rjz > 0.0f) {
        float2 uv2;
        if (rjz != 1.0f) {
            uint mask = uint((rjz * 15.0f) + 0.5f);
            uint4 sel = uint4(mask, mask, mask, mask) & uint4(1u, 2u, 4u, 8u);
            float4 fsel = float4(sel != uint4(0u, 0u, 0u, 0u));
            uv2 = uv + (((((-0.5f).xx * fsel.x) + (float2(0.5f, -0.5f) * fsel.y)) + (float2(-0.5f, 0.5f) * fsel.z)) + (0.5f.xx * fsel.w)) * 2.0f / dims;
        } else {
            uv2 = uv;
        }
        float2 uv_clamped = clamp(uv2, float2(0.5f / float(half_depth_tex_dims.x), 0.0f), 1.0f.xx - float2(0.5f / float(half_depth_tex_dims.x), 0.0f));

        float4 hist[3] = { 0.0f.xxxx, 0.0f.xxxx, 0.0f.xxxx };
        for (uint c0 = 0u; c0 < 3u; ++c0) {
            hist[c0] = history_tex.SampleLevel(sampler_lnc, (uv_clamped * float2(0.3333333432674407958984375f, 1.0f)) + float2(float(c0) * 0.3333333432674407958984375f, 0.0f), 0.0f);
        }

        float3 q2;
        if (flip) {
            float2 s2 = (1.0f.xx - abs(p.yx)) * ((step(0.0f.xx, p.xy) * 2.0f) - 1.0f.xx);
            q2 = float3(s2.x, s2.y, p.z);
        } else {
            q2 = p;
        }
        float2 uv_q2 = (q2.xy * 0.5f) + 0.5f.xx;
        float4 gv = abs(uv_q2.xxxx - temporal_stats_tex.GatherGreen(sampler_nnc, uv_clamped, int2(0, 0)));
        float4 gb = abs(uv_q2.yyyy - temporal_stats_tex.GatherBlue(sampler_nnc, uv_clamped, int2(0, 0)));
        bool valid = (min(min(gv.x, gv.y), min(gv.z, gv.w)) + min(min(gb.x, gb.y), min(gb.z, gb.w))) < 0.20000000298023223876953125f;

        if (valid) {
            float cnt = clamp((temporal_stats_tex.SampleLevel(sampler_nnc, uv_clamped, 0.0f) * float4(255.0f, 1.0f, 1.0f, 1.0f)).x + 1.0f, 1.0f, 48.0f);
            stats_out = stats;
            stats_out.x = cnt;
            float inv = 1.0f / cnt;
            float4 out_slab[3];
            for (uint c1 = 0u; c1 < 3u; ++c1) {
                float4 v = lerp(hist[c1], output_tex[xy + uint2(c1 * half_depth_tex_dims.x, 0u)], inv.xxxx);
                v.x = max(0.0f, v.x);
                v.yzw = clamp(v.yzw, (-1024.0f).xxx, 1024.0f.xxx);
                out_slab[c1] = v;
            }
            for (uint c2 = 0u; c2 < 3u; ++c2) {
                output_tex[xy + uint2(c2 * half_depth_tex_dims.x, 0u)] = out_slab[c2];
            }
            updated = false;
        } else {
            stats_out = stats;
            updated = true;
        }
    } else {
        stats_out = stats;
        updated = true;
    }

    if (updated) {
        temporal_stats_out_tex[xy] = stats_out;
    }

    float4 result;
    if (updated && allow_non_determinism != 0u) {
        float depth = half_depth_tex.Load(int3(xy, 0u)).x;
        float normal_z = max(0.300000011920928955078125f, nz);
        float4 sum[3] = { 0.0f.xxxx, 0.0f.xxxx, 0.0f.xxxx };
        float4 accw = float4(0.0f, 0.0f, 0.0f, 0.0f);
        float wsum = 0.0f;
        float wsum2 = 0.0f;
        for (int yy = -2; yy <= 2; ++yy) {
            for (int xx = -2; xx <= 2; ++xx) {
                uint2 off = xy + uint2(int2(xx, yy) * int2(2, 2));
                if (any(bool2(off.x >= half_depth_tex_dims.x, off.y >= half_depth_tex_dims.y))) {
                    continue;
                }
                float nd = dot(n, half_view_normal_tex.Load(int3(off, 0u)).xyz);
                float w = exp2(-min(30.0f, 80.0f * (normal_z * abs(1.0f - (half_depth_tex.Load(int3(off, 0u)).x / depth))))) * (0.010000000707805156707763671875f / ((1.0f - 0.89999997615814208984375f * nd * nd) * (1.0f - 0.89999997615814208984375f * nd * nd)));
                float4 ts = temporal_stats_out_tex[off] * float4(255.0f, 1.0f, 1.0f, 1.0f);
                float tw = max(ts.x, 1.0f);
                float4 tsw = ts;
                tsw.x = tw;
                for (uint c3 = 0u; c3 < 3u; ++c3) {
                    sum[c3] += (output_tex[off + uint2(c3 * half_depth_tex_dims.x, 0u)] * w) * tw;
                }
                accw += tsw * w;
                wsum += w;
                wsum2 += w * tw;
            }
        }
        float inv2 = max(9.9999996826552253889678874634872e-21f, wsum2);
        for (uint c4 = 0u; c4 < 3u; ++c4) {
            float4 v = sum[c4] / inv2.xxxx;
            v.x = max(0.0f, v.x);
            v.yzw = clamp(v.yzw, (-1024.0f).xxx, 1024.0f.xxx);
            output_tex[xy + uint2(c4 * half_depth_tex_dims.x, 0u)] = v;
        }
        result = accw * (1.0f / max(9.9999996826552253889678874634872e-21f, wsum));
        result.x = min(result.x, 24.0f);
    } else {
        result = stats_out;
    }

    temporal_stats_out_tex[xy] = result * float4(0.0039215688593685626983642578125f, 1.0f, 1.0f, 1.0f);
}
