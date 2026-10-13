#line 1 "shaders/calculate_reprojection_map.cs.hlsl"













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
#line 14 "shaders/calculate_reprojection_map.cs.hlsl"

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
#line 15 "shaders/calculate_reprojection_map.cs.hlsl"

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
#line 16 "shaders/calculate_reprojection_map.cs.hlsl"

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
#line 17 "shaders/calculate_reprojection_map.cs.hlsl"

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
#line 18 "shaders/calculate_reprojection_map.cs.hlsl"

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
#line 19 "shaders/calculate_reprojection_map.cs.hlsl"

#line 1 "shaders/inc/bilinear.hlsl"




#ifndef TINYGLADE_INC_BILINEAR_HLSL
#define TINYGLADE_INC_BILINEAR_HLSL

struct BilinearCorners
{
    int2 p00;
    int2 p10;
    int2 p01;
    int2 p11;
};



BilinearCorners bilinear_corners(int2 base, uint2 dims)
{
    BilinearCorners c;
    c.p00 = base;
    c.p10 = base + int2(1, 0);
    c.p01 = base + int2(0, 1);
    c.p11 = base + int2(1, 1);
    return c;
}


uint bilinear_corners_mask(BilinearCorners c, uint2 dims)
{
    uint mask = 0u;
    int2 p[4] = { c.p00, c.p10, c.p01, c.p11 };
    int2 d = int2(dims);
    for (int i = 0; i < 4; i++)
    {
        if (p[i].x >= 0 && p[i].y >= 0 && p[i].x < d.x && p[i].y < d.y)
            mask |= (1u << i);
    }
    return mask;
}

#endif 
#line 20 "shaders/calculate_reprojection_map.cs.hlsl"

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
#line 21 "shaders/calculate_reprojection_map.cs.hlsl"


cbuffer $Globals : register(b13, space0)
{
    uint2 depth_tex_dims;
    uint2 prev_depth_tex_dims;
    uint history_valid;
    uint2 velocity_tex_dims;
    uint2 output_tex_dims;
};

Texture2D<float4> depth_tex : register(t14, space0);
Texture2D<float4> prev_depth_tex : register(t15, space0);
Texture2D<float4> velocity_tex : register(t16, space0);
RWTexture2D<snorm float4> output_tex : register(u17, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    float2 uv = (float2(int2(dispatchThreadID.xy)) + 0.5f) / float2(depth_tex_dims);

    
    if (depth_tex.Load(int3(dispatchThreadID.xy, 0)).x == 0.0f)
    {
        float4 clip_pos = float4((uv - 0.5f) * float2(2.0f, -2.0f), 0.0f, 1.0f);
        float4 prev_clip = mul(view_constants[0].clip_to_prev_clip, mul(view_constants[0].view_to_clip, mul(view_constants[0].clip_to_view, clip_pos)));
        float2 prev_uv = ((prev_clip.xy / prev_clip.w) * float2(0.5f, -0.5f)) + 0.5f;
        float2 clamped = clamp(prev_uv, 0.0f, 1.0f);
        output_tex[dispatchThreadID.xy] = float4(prev_uv - ((clip_pos.xy * float2(0.5f, -0.5f)) + 0.5f), float(all(clamped == prev_uv)), 1.0f);
    }
    else
    {
        float depth = depth_tex.Load(int3(dispatchThreadID.xy, 0)).x;
        float4 view_pos = mul(view_constants[0].clip_to_view, float4((uv - 0.5f) * float2(2.0f, -2.0f), depth, 1.0f));
        float w = view_pos.w;
        float view_z = view_pos.z / w;
        float4 view_pos_n = view_pos / w;

        float4 vel = mul(view_constants[0].world_to_view, float4(velocity_tex.Load(int3(uint2(dispatchThreadID.xy), 0)).xyz, 0.0f));
        float3 prev_pos = view_pos_n.xyz - vel.xyz;

        float4 prev_clip = mul(view_constants[0].clip_to_prev_clip, mul(view_constants[0].view_to_clip, float4(prev_pos.x, prev_pos.y, prev_pos.z, view_pos_n.w)));
        float2 motion = floor(((((prev_clip.xy / prev_clip.w) * float2(0.5f, -0.5f)) + 0.5f) - uv) * 32767.0f + 0.5f) * 3.0518509447574615478515625e-05f;

        bool use_history;
        if (history_valid != 0u)
        {
            use_history = !(0u == frame_index);
        }
        else
        {
            use_history = false;
        }

        float validity;
        float confidence;
        if (use_history)
        {
            float4 prev_view_pos = mul(view_constants[0].prev_clip_to_prev_view, prev_clip);
            float2 out_dims = float2(output_tex_dims);
            float2 tap_pos = trunc(((uv + motion) + ((view_constants[0].sample_offset_clip - view_constants[0].prev_sample_offset_clip) * 0.5f) * float2(1.0f, -1.0f)) * out_dims - 0.5f);

            float4 prev_depth_gathered = (1.0f / (prev_depth_tex.GatherRed(sampler_nnc, (tap_pos + 1.0f) / out_dims).wzxy * (-view_constants[0].prev_clip_to_prev_view[3].z))) - (prev_view_pos / prev_view_pos.w).z;
            float4 neg = -prev_depth_gathered;

            float view_dir_z = -normalize(view_pos.xyz / view_pos.w).z;
            float4 in_range = step(abs(prev_depth_gathered * min(-0.20000000298023223876953125f, 1.0f)), ((view_z * (-0.004999999888241291046142578125f)) / view_dir_z));
            float4 threshold = ((view_z * (-0.0003000000142492353916168212890625f)) / view_dir_z);
            float4 reject_neg = step(neg, threshold);
            float4 reject_pos = step(-neg, threshold);

            int2 tap = int2(tap_pos);
            bool in_bounds_00 = all(tap >= int2(0, 0)) && all(uint2(tap) < output_tex_dims);
            int2 tap_x = tap + int2(1, 0);
            bool in_bounds_10 = all(tap_x >= int2(0, 0)) && all(uint2(tap_x) < output_tex_dims);
            int2 tap_y = tap + int2(0, 1);
            bool in_bounds_01 = all(tap_y >= int2(0, 0)) && all(uint2(tap_y) < output_tex_dims);
            int2 tap_xy = tap + int2(1, 1);
            bool in_bounds_11 = all(tap_xy >= int2(0, 0)) && all(uint2(tap_xy) < output_tex_dims);

            bool rejected = (int(any(reject_pos != 0.0f)) > 0) ? (int(any(reject_neg != 0.0f)) > 0) : false;
            validity = float(rejected);
            confidence = dot(float4(in_range.x * float(in_bounds_00), in_range.y * float(in_bounds_10), in_range.z * float(in_bounds_01), in_range.w * float(in_bounds_11)), float4(1.0f, 2.0f, 4.0f, 8.0f)) * 0.066666670143604278564453125f;
        }
        else
        {
            validity = 1.0f;
            confidence = 1.0f;
        }

        output_tex[dispatchThreadID.xy] = float4(motion, confidence, (1.0f - validity) + length((vel.xz * (((-0.5f) / view_z) * view_constants[0].view_to_clip[1].y)) * float2(view_constants[0].view_to_clip[1].y / view_constants[0].view_to_clip[0].x, 1.0f)));
    }
}
