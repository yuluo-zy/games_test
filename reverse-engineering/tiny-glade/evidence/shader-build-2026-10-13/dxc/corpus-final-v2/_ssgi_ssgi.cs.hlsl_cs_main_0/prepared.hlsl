#line 1 "shaders/ssgi/ssgi.cs.hlsl"













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
#line 14 "shaders/ssgi/ssgi.cs.hlsl"

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
#line 15 "shaders/ssgi/ssgi.cs.hlsl"

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
#line 16 "shaders/ssgi/ssgi.cs.hlsl"

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
#line 17 "shaders/ssgi/ssgi.cs.hlsl"

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
#line 18 "shaders/ssgi/ssgi.cs.hlsl"

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
#line 19 "shaders/ssgi/ssgi.cs.hlsl"

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
#line 20 "shaders/ssgi/ssgi.cs.hlsl"

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
#line 21 "shaders/ssgi/ssgi.cs.hlsl"

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
#line 22 "shaders/ssgi/ssgi.cs.hlsl"

#line 1 "shaders/ssgi/XeGTAO.hlsl"












#ifndef TINYGLADE_SSGI_XEGTAO_HLSL
#define TINYGLADE_SSGI_XEGTAO_HLSL




float XeGTAO_Internal_FastACos(float abs_x)
{
    return ((-0.15658299624919891357421875f * abs_x) + 1.57079601287841796875f)
        * asfloat(532487669 + (asint(1.0f - abs_x) >> 1));
}



float XeGTAO_Internal_ProjectedNormalLen(float projected_normal_len)
{
    return lerp(projected_normal_len, 1.0f, 0.0500000007450580596923828125f);
}

#endif 
#line 23 "shaders/ssgi/ssgi.cs.hlsl"

#line 1 "shaders/ssgi/vaShared.hlsl"











#ifndef TINYGLADE_SSGI_VASHARED_HLSL
#define TINYGLADE_SSGI_VASHARED_HLSL




float va_depth_to_view_z(float clip_to_view_32, float depth)
{
    return (-1.0f) / (depth * (-clip_to_view_32));
}

#endif 
#line 24 "shaders/ssgi/ssgi.cs.hlsl"


cbuffer $Globals : register(b13, space0)
{
    uint2 depth_tex_dims;
    uint2 half_view_normal_tex_dims;
    uint2 output_tex_dims;
};

Texture2D<float4> depth_tex : register(t14, space0);
Texture2D<float4> half_view_normal_tex : register(t15, space0);
RWTexture2D<half> output_tex : register(u16, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    float2 output_res = float2(output_tex_dims);
    float2 uv = pixel_to_uv(int2(dispatchThreadID.xy), output_res);

    
    
    uint pixel_index = 0u;
    uint sfc_x = dispatchThreadID.x;
    uint sfc_y = dispatchThreadID.y;
    for (uint mask = 32u; mask > 0u; mask /= 2u)
    {
        uint xb = (sfc_x & mask) > 0u ? 1u : 0u;
        uint yb = (sfc_y & mask) > 0u ? 1u : 0u;
        pixel_index += (mask * mask) * ((3u * xb) ^ yb);
        if (yb == 0u)
        {
            if (xb == 1u)
            {
                uint prev_y = sfc_y;
                sfc_y = 63u - sfc_x;
                sfc_x = 63u - prev_y;
            }
        }
    }

    float2 jitter = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(pixel_index + (288u * (frame_index % 64u)))) + 0.5f);
    float jitter_x = jitter.x;
    float jitter_y = jitter.y;

    
    float4 view_homo = mul(view_constants[0].sample_to_view, float4(uv_to_sample(uv) * float2(2.0f, -2.0f), depth_tex.Load(uint3(dispatchThreadID.xy, 0u)).x, 1.0f));
    float view_z = (view_homo.xyz / view_homo.w).z;
    float focal = (-0.5f / view_z) * view_constants[0].view_to_clip[1][1];

    float2 inv_res = 1.0f / output_res;
    float2 view_to_sample_inv = float2(1.0f / view_constants[0].view_to_sample[0][0], 1.0f / view_constants[0].view_to_sample[1][1]);
    float2 clip_pix_scale = view_to_sample_inv * float2(2.0f, -2.0f);
    float2 clip_pix_bias = view_to_sample_inv * float2(-1.0f, 1.0f);

    float3 view_normal = normalize(half_view_normal_tex.SampleLevel(sampler_nnc, uv, 0.0f).xyz * float3(1.0f, 1.0f, -1.0f));

    float2 pixel_center_uv = (float2(dispatchThreadID.xy) + 0.5f) * inv_res;

    float4 gathered = depth_tex.GatherRed(sampler_nnc, float2(dispatchThreadID.xy) * inv_res);
    for (uint g = 0u; g < 4u; g++)
    {
        gathered[g] = va_depth_to_view_z(view_constants[0].clip_to_view[3][2], gathered[g]);
    }

    float view_depth = gathered.y * 0.99919998645782470703125f;

    float3 view_pos;
    view_pos.xy = ((clip_pix_scale * pixel_center_uv) + clip_pix_bias) * view_depth;
    view_pos.z = view_depth;
    float3 view_dir = normalize(-view_pos);

    float radius_pixels = (-80.0f) * focal;
    float distance_fade = ((0.11249999701976776123046875f / focal) / (0.012500000186264514923095703125f / focal)) + 1.0f;

    float ao_radius = (0.125f / focal) / ((float2(view_depth, view_depth) * (clip_pix_scale * inv_res)).x);
    float radius_dist = 1.2999999523162841796875f / ao_radius;

    float ao = clamp((10.0f - ao_radius) * 0.00999999977648258209228515625f, 0.0f, 1.0f) * 0.5f;

    
    float3 sample_pos;

    for (float slice = 0.0f; slice < 1.0f; slice += 1.0f)
    {
        float angle = (slice + jitter_x) * 3.1415927410125732421875f;
        float cos_angle = cos(angle);
        float sin_angle = sin(angle);

        float2 slice_dir = float2(cos_angle, -sin_angle) * ao_radius;
        float3 slice_dir3 = float3(cos_angle, sin_angle, 0.0f);
        float3 h_proj = slice_dir3 - (view_dir * dot(slice_dir3, view_dir));
        float3 h_perp = normalize(cross(h_proj, view_dir));
        float3 n_proj = view_normal - (h_perp * dot(view_normal, h_perp));
        float n_len = length(n_proj);
        float ndv = clamp(dot(n_proj, view_dir) / n_len, 0.0f, 1.0f);
        float abs_ndv = abs(ndv);
        float sign_factor = float(int(sign(dot(h_proj, n_proj))));
        float horizon0 = sign_factor * XeGTAO_Internal_FastACos(abs_ndv);

        float cos_h2 = cos(horizon0 + 1.57079637050628662109375f);
        float cos_h1 = cos(horizon0 - 1.57079637050628662109375f);
        float h1 = cos_h1;
        float h2 = cos_h2;

        [unroll]
        for (float tap = 0.0f; tap < 3.0f; tap += 1.0f)
        {
            float2 sample_offset = slice_dir * (pow((tap + frac(jitter_y + ((slice + (tap * 3.0f)) * 0.61803400516510009765625f))) * 0.3333333432674407958984375f, 2.0f) + radius_dist);
            float mip_level = clamp(log2(length(sample_offset)), 0.0f, 0.0f);
            float2 rounded = round(sample_offset) * inv_res;

            float2 sample_uv_a = pixel_center_uv + rounded;
            float view_z_a = va_depth_to_view_z(view_constants[0].clip_to_view[3][2], depth_tex.SampleLevel(sampler_nnc, sample_uv_a, mip_level).x);
            sample_pos.xy = ((clip_pix_scale * sample_uv_a) + clip_pix_bias) * view_z_a;
            sample_pos.z = view_z_a;
            float3 delta_a = sample_pos - view_pos;
            float len_a = length(delta_a);

            float2 sample_uv_b = pixel_center_uv - rounded;
            float view_z_b = va_depth_to_view_z(view_constants[0].clip_to_view[3][2], depth_tex.SampleLevel(sampler_nnc, sample_uv_b, mip_level).x);
            sample_pos.xy = ((clip_pix_scale * sample_uv_b) + clip_pix_bias) * view_z_b;
            sample_pos.z = view_z_b;
            float3 delta_b = sample_pos - view_pos;
            float len_b = length(delta_b);

            h1 = max(h1, lerp(cos_h1, dot(delta_b / len_b, view_dir), clamp((len_b * radius_pixels) + distance_fade, 0.0f, 1.0f)));
            h2 = max(h2, lerp(cos_h2, dot(delta_a / len_a, view_dir), clamp((len_a * radius_pixels) + distance_fade, 0.0f, 1.0f)));
        }

        
        float abs_h1 = abs(h1);
        float acos_h1 = XeGTAO_Internal_FastACos(abs_h1);
        float horizon_angle_neg = (h1 >= 0.0f) ? acos_h1 : (3.14159297943115234375f - acos_h1);

        float abs_h2 = abs(h2);
        float acos_h2 = XeGTAO_Internal_FastACos(abs_h2);
        float horizon_angle_pos = (h2 >= 0.0f) ? acos_h2 : (3.14159297943115234375f - acos_h2);

        float half_w2 = horizon_angle_neg * (-2.0f);
        float sin_horizon0 = sin(horizon0);
        float half_w1 = 2.0f * horizon_angle_pos;

        ao += (lerp(n_len, 1.0f, 0.0500000007450580596923828125f) * (0.25f * (((ndv + (half_w2 * sin_horizon0)) - cos(half_w2 - horizon0)) + ((ndv + (half_w1 * sin_horizon0)) - cos(half_w1 - horizon0)))));
    }

    output_tex[dispatchThreadID.xy] = max(0.02999999932944774627685546875f, pow(ao, 1.0f));
}
