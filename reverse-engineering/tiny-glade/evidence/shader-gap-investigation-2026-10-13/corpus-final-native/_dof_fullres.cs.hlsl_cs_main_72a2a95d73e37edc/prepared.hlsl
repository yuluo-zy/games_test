#define PERM_quality 1
#define TGV_72a2a95d73e37edc 1
#line 1 "shaders/dof/fullres.cs.hlsl"


#if !((defined(TGV_6f1dd5dd8a66e104) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)) || (defined(TGV_72a2a95d73e37edc) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))






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
#line 10 "shaders/dof/fullres.cs.hlsl"

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
#line 11 "shaders/dof/fullres.cs.hlsl"

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
#line 12 "shaders/dof/fullres.cs.hlsl"

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
#line 13 "shaders/dof/fullres.cs.hlsl"

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
#line 14 "shaders/dof/fullres.cs.hlsl"

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
#line 15 "shaders/dof/fullres.cs.hlsl"

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
#line 16 "shaders/dof/fullres.cs.hlsl"

#line 1 "shaders/inc/coc.hlsl"







#ifndef TINYGLADE_INC_COC_HLSL
#define TINYGLADE_INC_COC_HLSL


struct DofFocus {
    float focus0;
    float focus1;
    float lock_bias0;
    float lock_bias1;
    float3 lock_pos_ws;
    uint is_locked;
    float3 interp_lock_pos_ws;
    uint use_tilt_shift;
    float interp_lock_bias0;
    float interp_lock_bias1;
    float focus_scale;
    uint pad2;
};


struct DofFocusReadback {
    float3 lock_pos_ws;
    uint lock_reponse; 
};



float compute_coc(float depth, float focus_dist, float focus_scale)
{
    float coc = (focus_dist - depth) / max(depth, 1e-6);
    return clamp(coc * focus_scale, -1.0, 1.0);
}

#endif 
#line 17 "shaders/dof/fullres.cs.hlsl"

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
#line 18 "shaders/dof/fullres.cs.hlsl"

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
#line 19 "shaders/dof/fullres.cs.hlsl"

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
#line 20 "shaders/dof/fullres.cs.hlsl"

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
#line 21 "shaders/dof/fullres.cs.hlsl"

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
#line 22 "shaders/dof/fullres.cs.hlsl"

#line 1 "shaders/dof/tiles.inc.hlsl"









#ifndef TINYGLADE_DOF_TILES_INC_HLSL
#define TINYGLADE_DOF_TILES_INC_HLSL

struct DofTileOffsets
{
    uint offsets[8];
};

struct DofTileOffsetsPacked
{
    uint4 offsets[2];
};

#endif 
#line 23 "shaders/dof/fullres.cs.hlsl"

#line 1 "shaders/dof/upsample_halfres.inc.hlsl"







#ifndef TINYGLADE_DOF_UPSAMPLE_HALFRES_INC_HLSL
#define TINYGLADE_DOF_UPSAMPLE_HALFRES_INC_HLSL


float4 upsample_halfres(Texture2D<float4> src_tex,
                        int4 resolve_offsets[512],
                        uint2 fullres_pixel,
                        uint quadrant)
{
    float4 sum = 0.0f;
    for (uint i = 0u; i < 8u; ++i)
    {
        sum += src_tex[(fullres_pixel / uint2(2u, 2u))
            + uint2(resolve_offsets[(quadrant * 16u) + i].xy)];
    }
    return sum * 0.125f.xxxx;
}

#endif 
#line 24 "shaders/dof/fullres.cs.hlsl"

#line 1 "shaders/dof/multires.inc.hlsl"







#ifndef TINYGLADE_DOF_MULTIRES_INC_HLSL
#define TINYGLADE_DOF_MULTIRES_INC_HLSL



uint dof_pack_half2(float2 v)
{
    uint2 packed = f32tof16(v);
    return packed.x | (packed.y << 16u);
}


float2 dof_unpack_half2(uint value)
{
    return f16tof32(uint2(value & 0xFFFFu, value >> 16u));
}

#endif 
#line 25 "shaders/dof/fullres.cs.hlsl"


cbuffer $Globals : register(b13, space0)
{
    float bokeh_anisotropy;
    uint2 blue_noise_texture_dims;
    uint2 color_tex_dims;
    uint2 coc_tiles_dilated_tex_dims;
    uint2 depth_tex_dims;
    uint2 coc_tex_dims;
    uint2 img_output_dims;
    uint2 halfres_tex_dims;
};

Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> color_tex : register(t15, space0);
Texture2D<float4> coc_tiles_dilated_tex : register(t16, space0);
Texture2D<float4> coc_tex : register(t18, space0);
RWTexture2D<half4> img_output : register(u19, space0);
Texture2D<float4> halfres_tex : register(t20, space0);
cbuffer spatial_resolve_offsets : register(b21, space0)
{
    int4 spatial_resolve_offsets_array[512];
};
StructuredBuffer<DofTileOffsets> tile_offsets_buffer : register(t22, space0);
StructuredBuffer<uint2> tile_coords_buffer : register(t23, space0);

groupshared uint dof_ring_samples[64][9];

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_WorkGroupID : SV_GroupID,
             uint3 gl_GlobalInvocationID : SV_DispatchThreadID,
             uint gl_LocalInvocationIndex : SV_GroupIndex)
{
    uint2 pixel = (gl_GlobalInvocationID.xy & uint2(7u, 7u)) + (uint2(8u, 8u) * tile_coords_buffer[tile_offsets_buffer[0].offsets[0] + gl_WorkGroupID.x]);
    float2 pixel_uv = (float2(int2(pixel)) + 0.5f) / float2(img_output_dims);

    
    uint2 in_tile = pixel & uint2(1u, 1u);
    uint quadrant = in_tile.x + (in_tile.y * 2u);
    float4 upsample = upsample_halfres(halfres_tex, spatial_resolve_offsets_array, pixel, quadrant);

    float coc_limit = min(float(color_tex_dims.y) * 0.0416666679084300994873046875f, coc_tiles_dilated_tex.SampleLevel(sampler_nnc, pixel_uv, 0.0f).x);

    float4 dither = ((blue_noise_texture.Load(int3(((pixel + uint2(13u, 13u))
        + uint2(int2(int(frame_index * 59u), int(frame_index * 37u)))) & uint2(255u, 255u), 0u)) * 255.0f)
        * 0.00390625f) + 0.001953125f;

    uint2 hash_pos = uint2(7u, 7u) + pixel;
    float sub = (frac(52.98291778564453125f * frac((0.067110560834407806396484375f * float(hash_pos.x))
        + (0.005837149918079376220703125f * float(hash_pos.y)))) - 0.5f) * 0.5f;
    float sub2 = 0.5f + sub;
    float dither_y = dither.y;
    float coc_step = coc_limit * 0.125f;
    float frame_rand = 5.588237762451171875f * float(frame_index % 64u);
    float tap_seed = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(pixel.x) + frame_rand))
        + (0.005837149918079376220703125f * (float(pixel.y) + frame_rand))));
    bool use_low = sub2 < 0.5f;
    float tap_sub = use_low ? (0.5f - sub) : sub2;
    float tap_sign = use_low ? (-1.0f) : 1.0f;

    float4 color_accum = 0.0f.xxxx;
    float weight_accum = 0.0f;
    float depth_weight = 0.0f;
    float4 ring_color_accum = 0.0f.xxxx;
    uint ring_bits[8];
    ring_bits[0] = 0u; ring_bits[1] = 0u; ring_bits[2] = 0u; ring_bits[3] = 0u;
    ring_bits[4] = 0u; ring_bits[5] = 0u; ring_bits[6] = 0u; ring_bits[7] = 0u;
    float coc_min = 1e10f;
    float coc_max = -1e10f;

    for (uint ring = 0u; ring < 2u; ++ring)
    {
        float angle = (float(ring) + tap_seed) * 1.57079637050628662109375f;
        float2 dir = float2(cos(angle), sin(angle));
        float2 bokeh_dir;
        if (bokeh_anisotropy > 0.0f)
        {
            float4 ctv = mul(view_constants[0].clip_to_view, float4((pixel_uv - 0.5f) * float2(2.0f, -2.0f), 0.0f, 1.0f));
            float2 view_dir = ctv.xy / ctv.z;
            view_dir.y = view_dir.y * (-1.0f);
            float2 norm_dir = normalize(view_dir);
            float2x2 rot = float2x2(norm_dir, float2(-norm_dir.y, norm_dir.x));
            float aniso = lerp(0.7f, 1.0f, exp((-4.0f * bokeh_anisotropy) * dot(view_dir, view_dir)));
            bokeh_dir = mul(transpose(rot), mul(rot, dir) * float2(aniso, 1.0f / aniso));
        }
        else
        {
            bokeh_dir = dir;
        }
        float2 step_px = bokeh_dir * (tap_sign / float2(color_tex_dims));
        float neg_coc = -coc_limit;
        float lerp_gain = dither.x * 0.2f;

        float coc0 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-8.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float m = abs(coc0);
        float w = lerp(weight_accum, max(weight_accum, m), lerp_gain);
        float coc1 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-7.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc1)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][0] = dof_pack_half2(float2(coc0, 0.0f)) | (dof_pack_half2(float2(coc1, 0.0f)) << 16u);

        float coc2 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-6.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc2)), lerp_gain);
        float coc3 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-5.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc3)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][1] = dof_pack_half2(float2(coc2, 0.0f)) | (dof_pack_half2(float2(coc3, 0.0f)) << 16u);

        float coc4 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-4.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc4)), lerp_gain);
        float coc5 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-3.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc5)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][2] = dof_pack_half2(float2(coc4, 0.0f)) | (dof_pack_half2(float2(coc5, 0.0f)) << 16u);

        float coc6 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-2.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc6)), lerp_gain);
        float coc7 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-1.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc7)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][3] = dof_pack_half2(float2(coc6, 0.0f)) | (dof_pack_half2(float2(coc7, 0.0f)) << 16u);

        float coc8 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (dither_y * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc8)), lerp_gain);
        float coc9 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((1.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc9)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][4] = dof_pack_half2(float2(coc8, 0.0f)) | (dof_pack_half2(float2(coc9, 0.0f)) << 16u);

        float coc10 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((2.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc10)), lerp_gain);
        float coc11 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((3.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc11)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][5] = dof_pack_half2(float2(coc10, 0.0f)) | (dof_pack_half2(float2(coc11, 0.0f)) << 16u);

        float coc12 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((4.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc12)), lerp_gain);
        float coc13 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((5.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc13)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][6] = dof_pack_half2(float2(coc12, 0.0f)) | (dof_pack_half2(float2(coc13, 0.0f)) << 16u);

        float coc14 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((6.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc14)), lerp_gain);
        float coc15 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((7.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc15)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][7] = dof_pack_half2(float2(coc14, 0.0f)) | (dof_pack_half2(float2(coc15, 0.0f)) << 16u);

        float coc16 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((8.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        weight_accum = lerp(w, max(w, abs(coc16)), lerp_gain);
        coc_min = min(min(min(min(min(min(min(min(min(coc_min, abs(coc0)), abs(coc2)), abs(coc4)), abs(coc6)), abs(coc8)), abs(coc10)), abs(coc12)), abs(coc14)), abs(coc16));
        coc_max = max(max(max(max(max(max(max(max(max(coc_max, abs(coc0)), abs(coc2)), abs(coc4)), abs(coc6)), abs(coc8)), abs(coc10)), abs(coc12)), abs(coc14)), abs(coc16));
        dof_ring_samples[gl_LocalInvocationIndex][8] = dof_pack_half2(float2(coc16, 0.0f));

        for (uint i = 0u; i < 8u; ++i)
        {
            ring_bits[i] = 0u;
        }
        for (uint tap = 0u; tap < 17u; ++tap)
        {
            uint par = tap & 1u;
            uint half = tap >> 1u;
            float2 slot_coc = float2(dof_unpack_half2(dof_ring_samples[gl_LocalInvocationIndex][half] & 65535u).x,
                                     dof_unpack_half2((dof_ring_samples[gl_LocalInvocationIndex][half] >> 16u) & 65535u).x);
            float tap_coc = (float(int(tap - 8u)) + dither_y) * coc_step;
            for (int slot = 0; slot < 8; ++slot)
            {
                if ((tap_coc / (((float(slot) + tap_sub) * 0.25f) - 1.0f)) >= slot_coc[par])
                {
                    ring_bits[slot] |= (1u << (tap & 31u));
                }
            }
        }

        float4 slot_color = 0.0f.xxxx;
        float slot_weight = 0.0f;
        float slot_depth = 0.0f;
        for (int slot = 0; slot < 8; slot_weight = depth_weight, slot_depth = slot_depth, slot_color = slot_color, ++slot)
        {
            float slot_frac = ((float(slot) + tap_sub) * 0.25f) - 1.0f;
            float abs_frac = abs(slot_frac);
            depth_weight = slot_depth + abs_frac;
            if (ring_bits[slot] == 0u)
            {
                continue;
            }
            bool positive = slot_frac >= 0.0f;
            float lo_coc, hi_coc;
            uint tap_idx;
            if (positive)
            {
                uint low_bit = firstbitlow(ring_bits[slot]);
                uint prev_bit = uint(max(0, (int(low_bit) - 1)));
                uint low_half = low_bit >> 1u;
                float2 a = float2(dof_unpack_half2(dof_ring_samples[gl_LocalInvocationIndex][low_half] & 65535u).x,
                                  dof_unpack_half2((dof_ring_samples[gl_LocalInvocationIndex][low_half] >> 16u) & 65535u).x);
                uint prev_half = prev_bit >> 1u;
                float2 b = float2(dof_unpack_half2(dof_ring_samples[gl_LocalInvocationIndex][prev_half] & 65535u).x,
                                  dof_unpack_half2((dof_ring_samples[gl_LocalInvocationIndex][prev_half] >> 16u) & 65535u).x);
                lo_coc = a[low_bit & 1u];
                hi_coc = b[prev_bit & 1u];
                tap_idx = low_bit;
            }
            else
            {
                uint high_bit = firstbithigh(ring_bits[slot]);
                uint next_bit = min(16u, (high_bit + 1u));
                uint high_half = high_bit >> 1u;
                float2 a = float2(dof_unpack_half2(dof_ring_samples[gl_LocalInvocationIndex][high_half] & 65535u).x,
                                  dof_unpack_half2((dof_ring_samples[gl_LocalInvocationIndex][high_half] >> 16u) & 65535u).x);
                uint next_half = next_bit >> 1u;
                float2 b = float2(dof_unpack_half2(dof_ring_samples[gl_LocalInvocationIndex][next_half] & 65535u).x,
                                  dof_unpack_half2((dof_ring_samples[gl_LocalInvocationIndex][next_half] >> 16u) & 65535u).x);
                lo_coc = a[high_bit & 1u];
                hi_coc = b[next_bit & 1u];
                tap_idx = high_bit;
            }
            float tap_coc = (float(int(tap_idx - 8u)) + dither_y) * coc_step;
            float coc_at_slot = tap_coc / slot_frac;
            float delta = coc_step * (positive ? 1.0f : (-1.0f));
            float lo_edge = tap_coc - delta;
            float hi_edge = hi_coc - (coc_at_slot - (delta / slot_frac));
            float slot_span = abs_frac * float(ring_bits[slot] != 0u);
            float resolved = lerp(lo_edge, tap_coc, clamp(hi_edge / (hi_edge + (-(lo_coc - coc_at_slot))), 0.0f, 1.0f));
            float coc_at_resolved = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * resolved), 0.0f).x, neg_coc, coc_limit);
            float occlusion = log2(max(0.0f, abs(lo_coc) * 0.125f)) * smoothstep(4.0f, 0.0f, coc_max - coc_min);
            float occ_lane = WaveReadLaneAt(occlusion, WaveGetLaneIndex() ^ 1u);
            occlusion = min(occlusion, occ_lane);
            float occ_wide = WaveReadLaneAt(occlusion, WaveGetLaneIndex() ^ 8u);
            if (lo_coc > coc_limit)
            {
                continue;
            }
            float edge_eps = hi_edge * 0.1f;
            bool close_to_edge;
            if (((resolved / slot_frac) - coc_at_resolved) < edge_eps)
            {
                close_to_edge = (coc_at_resolved - lo_coc) < edge_eps;
            }
            else
            {
                close_to_edge = false;
            }
            float4 ring_color;
            float ring_weight;
            
            
            if (close_to_edge)
            {
                float abs_resolved = abs(coc_at_resolved);
                if (abs_resolved >= 11.0f)
                {
                    continue;
                }
                ring_color = slot_color;
                ring_weight = slot_span * smoothstep(11.0f, 8.0f, abs_resolved);
            }
            else
            {
                float abs_hi = abs(hi_coc);
                if (abs_hi >= 11.0f)
                {
                    continue;
                }
                float w2 = slot_span * smoothstep(11.0f, 8.0f, abs_hi);
                float depth_gate = smoothstep(0.0f, 30.0f, hi_coc) * smoothstep(0.2f * coc_limit, 0.0f, lo_coc);
                ring_color = slot_color + (w2 * depth_gate);
                ring_weight = w2 * (1.0f - depth_gate);
            }
            depth_weight = ring_color.w;
            slot_color = slot_color + (color_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (close_to_edge ? resolved : lo_edge)), min(occlusion, occ_wide)) * ring_weight);
        }
        ring_color_accum = ring_color_accum + slot_color;
    }

    float scatter = depth_weight * smoothstep(0.01f * ring_color_accum.w, max(1e-5f, 0.2f * ring_color_accum.w), depth_weight);
    float4 final_color;
    if (scatter > 0.0f)
    {
        float neg_coc = -coc_limit;
        float4 bg_accum = 0.0f.xxxx;
        float max_bg_coc = neg_coc;
        for (uint s = 0u; s < 16u; ++s)
        {
            float2 rnd = frac(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(s)) + 0.5f) + dither.xy);
            float r = sqrt(rnd.x);
            float phi = rnd.y * 6.283185482025146484375f;
            float2 bg_uv = pixel_uv + (((float2(cos(phi) * r, sin(phi) * r) * 1.0f) / float2(color_tex_dims)) * weight_accum);
            float bg_coc = coc_tex.SampleLevel(sampler_nnc, bg_uv, 0.0f).x;
            if (bg_coc > (max_bg_coc - 10.0f))
            {
                bg_accum += color_tex.SampleLevel(sampler_llc, bg_uv, 0.0f);
                max_bg_coc = max(max_bg_coc, bg_coc);
            }
        }
        final_color = ring_color_accum + ((bg_accum / max(1.0f, bg_accum.w)) * scatter);
    }
    else
    {
        final_color = ring_color_accum;
    }

    
    float4 combined = upsample + (final_color / depth_weight);
    img_output[pixel] = float4((combined / max(1e-10f, combined.w)).xyz, 1.0f);
}

#endif

#if defined(TGV_6f1dd5dd8a66e104) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)

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

struct DofTileOffsets
{
    uint offsets[8];
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
    float bokeh_anisotropy : packoffset(c0);
    uint2 blue_noise_texture_dims : packoffset(c0.y);
    uint2 color_tex_dims : packoffset(c1);
    uint2 coc_tiles_dilated_tex_dims : packoffset(c1.z);
    uint2 depth_tex_dims : packoffset(c2);
    uint2 coc_tex_dims : packoffset(c2.z);
    uint2 img_output_dims : packoffset(c3);
    uint2 halfres_tex_dims : packoffset(c3.z);
};

cbuffer spatial_resolve_offsets : register(b21, space0)
{
    int4 spatial_resolve_offsets_spatial_resolve_offsets_array[512] : packoffset(c0);
};

ByteAddressBuffer tile_offsets_buffer : register(t22, space0);
ByteAddressBuffer tile_coords_buffer : register(t23, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_llc : register(s3, space0);
Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> color_tex : register(t15, space0);
Texture2D<float4> coc_tiles_dilated_tex : register(t16, space0);
Texture2D<float4> coc_tex : register(t18, space0);
RWTexture2D<float4> img_output : register(u19, space0);
Texture2D<float4> halfres_tex : register(t20, space0);

static uint3 gl_WorkGroupID;
static uint3 gl_GlobalInvocationID;
static uint gl_LocalInvocationIndex;
struct SPIRV_Cross_Input
{
    uint3 gl_WorkGroupID : SV_GroupID;
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
    uint gl_LocalInvocationIndex : SV_GroupIndex;
};

groupshared uint ring_samples[64][9];

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
    uint2 _201 = (gl_GlobalInvocationID.xy & uint2(7u, 7u)) + (uint2(8u, 8u) * tile_coords_buffer.Load<uint2>((tile_offsets_buffer.Load<uint>(0) + gl_WorkGroupID.x) * 8 + 0));
    uint2 _202 = _201 & uint2(1u, 1u);
    uint _206 = _202.x + (_202.y * 2u);
    float4 _208;
    _208 = 0.0f.xxxx;
    for (uint _211 = 0u; _211 < 8u; )
    {
        _208 += halfres_tex.Load(int3((_201 / uint2(2u, 2u)) + uint2(spatial_resolve_offsets_spatial_resolve_offsets_array[(_206 * 16u) + _211].xy), 0u));
        _211++;
        continue;
    }
    float2 _232 = (float2(int2(_201)) + 0.5f.xx) / float2(img_output_dims);
    float _243 = min(float(color_tex_dims.y) * 0.0416666679084300994873046875f, coc_tiles_dilated_tex.SampleLevel(sampler_nnc, _232, 0.0f).x);
    float4 _259 = ((blue_noise_texture.Load(int3(((_201 + uint2(13u, 13u)) + uint2(int2(int(frame_constants_frame_index * 59u), int(frame_constants_frame_index * 37u)))) & uint2(255u, 255u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
    uint2 _260 = uint2(7u, 7u) + _201;
    float _272 = (frac(52.98291778564453125f * frac((0.067110560834407806396484375f * float(_260.x)) + (0.005837149918079376220703125f * float(_260.y)))) - 0.5f) * 0.5f;
    float _273 = 0.5f + _272;
    float _274 = _259.y;
    float _275 = _243 * 0.125f;
    float _280 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
    float _290 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_201.x) + _280)) + (0.005837149918079376220703125f * (float(_201.y) + _280))));
    bool _291 = _273 < 0.5f;
    float _295;
    if (_291)
    {
        _295 = 0.5f - _272;
    }
    else
    {
        _295 = _273;
    }
    float _296 = _291 ? (-1.0f) : 1.0f;
    float4 _298;
    float _301;
    float _303;
    float _305;
    _298 = 0.0f.xxxx;
    _301 = 0.0f;
    _303 = 0.0f;
    _305 = 0.0f;
    float4 _299;
    float _302;
    float _310;
    float _312;
    uint _183[8];
    float _304;
    float _306;
    uint _307 = 0u;
    float _309 = 10000000000.0f;
    float _311 = -10000000000.0f;
    [loop]
    for (; _307 < 2u; _298 = _299, _301 = _302, _303 = _304, _305 = _306, _307++, _309 = _310, _311 = _312)
    {
        float _318 = (float(_307) + _290) * 1.57079637050628662109375f;
        float2 _321 = float2(cos(_318), sin(_318));
        float2 _359;
        if (bokeh_anisotropy > 0.0f)
        {
            float4 _334 = mul(frame_constants_view_constants[0u].clip_to_view, float4((_232 - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f));
            float2 _338 = _334.xy / _334.z.xx;
            _338.y = _338.y * (-1.0f);
            float2 _342 = normalize(_338);
            float2x2 _347 = float2x2(_342, float2(-_342.y, _342.x));
            float _352 = lerp(0.699999988079071044921875f, 1.0f, exp(((-4.0f) * bokeh_anisotropy) * dot(_338, _338)));
            _359 = mul(transpose(_347), mul(_347, _321) * float2(_352, 1.0f / _352));
        }
        else
        {
            _359 = _321;
        }
        float2 _364 = _359 * (_296.xx / float2(color_tex_dims));
        float _373 = -_243;
        float _374 = clamp(coc_tex.SampleLevel(sampler_nnc, _232 + (_364 * (((-8.0f) + _274) * _275)), 0.0f).x, _373, _243);
        float _375 = abs(_374);
        float _378 = _259.x * 0.20000000298023223876953125f;
        float _379 = lerp(_301, max(_301, _375), _378);
        float _391 = clamp(coc_tex.SampleLevel(sampler_nnc, _232 + (_364 * (((-7.0f) + _274) * _275)), 0.0f).x, _373, _243);
        float _394 = lerp(_379, max(_379, abs(_391)), _378);
        ring_samples[gl_LocalInvocationIndex][0u] = spvPackHalf2x16(float2(_374, 0.0f)) | (spvPackHalf2x16(float2(_391, 0.0f)) << 16u);
        float _407 = clamp(coc_tex.SampleLevel(sampler_nnc, _232 + (_364 * (((-6.0f) + _274) * _275)), 0.0f).x, _373, _243);
        float _408 = abs(_407);
        float _410 = lerp(_394, max(_394, _408), _378);
        float _422 = clamp(coc_tex.SampleLevel(sampler_nnc, _232 + (_364 * (((-5.0f) + _274) * _275)), 0.0f).x, _373, _243);
        float _425 = lerp(_410, max(_410, abs(_422)), _378);
        ring_samples[gl_LocalInvocationIndex][1u] = spvPackHalf2x16(float2(_407, 0.0f)) | (spvPackHalf2x16(float2(_422, 0.0f)) << 16u);
        float _438 = clamp(coc_tex.SampleLevel(sampler_nnc, _232 + (_364 * (((-4.0f) + _274) * _275)), 0.0f).x, _373, _243);
        float _439 = abs(_438);
        float _441 = lerp(_425, max(_425, _439), _378);
        float _453 = clamp(coc_tex.SampleLevel(sampler_nnc, _232 + (_364 * (((-3.0f) + _274) * _275)), 0.0f).x, _373, _243);
        float _456 = lerp(_441, max(_441, abs(_453)), _378);
        ring_samples[gl_LocalInvocationIndex][2u] = spvPackHalf2x16(float2(_438, 0.0f)) | (spvPackHalf2x16(float2(_453, 0.0f)) << 16u);
        float _469 = clamp(coc_tex.SampleLevel(sampler_nnc, _232 + (_364 * (((-2.0f) + _274) * _275)), 0.0f).x, _373, _243);
        float _470 = abs(_469);
        float _472 = lerp(_456, max(_456, _470), _378);
        float _484 = clamp(coc_tex.SampleLevel(sampler_nnc, _232 + (_364 * (((-1.0f) + _274) * _275)), 0.0f).x, _373, _243);
        float _487 = lerp(_472, max(_472, abs(_484)), _378);
        ring_samples[gl_LocalInvocationIndex][3u] = spvPackHalf2x16(float2(_469, 0.0f)) | (spvPackHalf2x16(float2(_484, 0.0f)) << 16u);
        float _499 = clamp(coc_tex.SampleLevel(sampler_nnc, _232 + (_364 * (_274 * _275)), 0.0f).x, _373, _243);
        float _500 = abs(_499);
        float _502 = lerp(_487, max(_487, _500), _378);
        float _514 = clamp(coc_tex.SampleLevel(sampler_nnc, _232 + (_364 * ((1.0f + _274) * _275)), 0.0f).x, _373, _243);
        float _517 = lerp(_502, max(_502, abs(_514)), _378);
        ring_samples[gl_LocalInvocationIndex][4u] = spvPackHalf2x16(float2(_499, 0.0f)) | (spvPackHalf2x16(float2(_514, 0.0f)) << 16u);
        float _530 = clamp(coc_tex.SampleLevel(sampler_nnc, _232 + (_364 * ((2.0f + _274) * _275)), 0.0f).x, _373, _243);
        float _531 = abs(_530);
        float _533 = lerp(_517, max(_517, _531), _378);
        float _545 = clamp(coc_tex.SampleLevel(sampler_nnc, _232 + (_364 * ((3.0f + _274) * _275)), 0.0f).x, _373, _243);
        float _548 = lerp(_533, max(_533, abs(_545)), _378);
        ring_samples[gl_LocalInvocationIndex][5u] = spvPackHalf2x16(float2(_530, 0.0f)) | (spvPackHalf2x16(float2(_545, 0.0f)) << 16u);
        float _561 = clamp(coc_tex.SampleLevel(sampler_nnc, _232 + (_364 * ((4.0f + _274) * _275)), 0.0f).x, _373, _243);
        float _562 = abs(_561);
        float _564 = lerp(_548, max(_548, _562), _378);
        float _576 = clamp(coc_tex.SampleLevel(sampler_nnc, _232 + (_364 * ((5.0f + _274) * _275)), 0.0f).x, _373, _243);
        float _579 = lerp(_564, max(_564, abs(_576)), _378);
        ring_samples[gl_LocalInvocationIndex][6u] = spvPackHalf2x16(float2(_561, 0.0f)) | (spvPackHalf2x16(float2(_576, 0.0f)) << 16u);
        float _592 = clamp(coc_tex.SampleLevel(sampler_nnc, _232 + (_364 * ((6.0f + _274) * _275)), 0.0f).x, _373, _243);
        float _593 = abs(_592);
        float _595 = lerp(_579, max(_579, _593), _378);
        float _607 = clamp(coc_tex.SampleLevel(sampler_nnc, _232 + (_364 * ((7.0f + _274) * _275)), 0.0f).x, _373, _243);
        float _610 = lerp(_595, max(_595, abs(_607)), _378);
        ring_samples[gl_LocalInvocationIndex][7u] = spvPackHalf2x16(float2(_592, 0.0f)) | (spvPackHalf2x16(float2(_607, 0.0f)) << 16u);
        float _623 = clamp(coc_tex.SampleLevel(sampler_nnc, _232 + (_364 * ((8.0f + _274) * _275)), 0.0f).x, _373, _243);
        float _624 = abs(_623);
        _302 = lerp(_610, max(_610, _624), _378);
        _310 = min(min(min(min(min(min(min(min(min(_309, _375), _408), _439), _470), _500), _531), _562), _593), _624);
        _312 = max(max(max(max(max(max(max(max(max(_311, _375), _408), _439), _470), _500), _531), _562), _593), _624);
        ring_samples[gl_LocalInvocationIndex][8u] = spvPackHalf2x16(float2(_623, 0.0f));
        for (uint _630 = 0u; _630 < 8u; )
        {
            _183[_630] = 0u;
            _630++;
            continue;
        }
        for (uint _637 = 0u; _637 < 17u; _637++)
        {
            uint _645 = _637 & 1u;
            uint _646 = _637 >> 1u;
            float2 _184 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_646] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_646] >> 16u) & 65535u).x);
            float _661 = (float(int(_637 - 8u)) + _274) * _275;
            for (int _663 = 0; _663 < 8; _663++)
            {
                if ((_661 / (((float(_663) + _295) * 0.25f) - 1.0f)) >= _184[_645])
                {
                    _183[_663] |= (1u << (_637 & 31u));
                }
            }
        }
        float4 _686;
        _306 = _305;
        _304 = _303;
        _686 = 0.0f.xxxx;
        float _685;
        float _683;
        float4 _687;
        for (int _688 = 0; _688 < 8; _306 = _683, _304 = _685, _686 = _687, _688++)
        {
            float _696 = ((float(_688) + _295) * 0.25f) - 1.0f;
            float _697 = abs(_696);
            _685 = _304 + _697;
            if (_183[_688] == 0u)
            {
                _683 = _306;
                _687 = _686;
                continue;
            }
            bool _703 = _696 >= 0.0f;
            float _771;
            float _772;
            uint _773;
            if (_703)
            {
                uint _738 = firstbitlow(_183[_688]);
                uint _742 = uint(max(0, (int(_738) - 1)));
                uint _744 = _738 >> 1u;
                float2 _185 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_744] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_744] >> 16u) & 65535u).x);
                uint _758 = _742 >> 1u;
                float2 _186 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_758] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_758] >> 16u) & 65535u).x);
                _771 = _185[_738 & 1u];
                _772 = _186[_742 & 1u];
                _773 = _738;
            }
            else
            {
                uint _707 = firstbithigh(_183[_688]);
                uint _709 = min(16u, (_707 + 1u));
                uint _711 = _707 >> 1u;
                float2 _187 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_711] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_711] >> 16u) & 65535u).x);
                uint _725 = _709 >> 1u;
                float2 _188 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_725] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_725] >> 16u) & 65535u).x);
                _771 = _187[_707 & 1u];
                _772 = _188[_709 & 1u];
                _773 = _707;
            }
            float _779 = (float(int(_773 - 8u)) + _274) * _275;
            float _780 = _779 / _696;
            float _781 = _275 * (_703 ? 1.0f : (-1.0f));
            float _784 = _779 - _781;
            float _785 = _772 - (_780 - (_781 / _696));
            float _789 = _697 * ((_183[_688] != 0u) ? 1.0f : 0.0f);
            float _794 = lerp(_784, _779, clamp(_785 / (_785 + (-(_771 - _780))), 0.0f, 1.0f));
            float _800 = clamp(coc_tex.SampleLevel(sampler_nnc, _232 + (_364 * _794), 0.0f).x, _373, _243);
            float _808 = log2(max(0.0f, abs(_771) * 0.125f)) * smoothstep(4.0f, 0.0f, _312 - _310);
            float _811 = WaveReadLaneAt(_808, WaveGetLaneIndex() ^ 1u);
            float _812 = min(_808, _811);
            float _814 = WaveReadLaneAt(_812, WaveGetLaneIndex() ^ 8u);
            if (_771 > _243)
            {
                _683 = _306;
                _687 = _686;
                continue;
            }
            float _820 = _785 * 0.100000001490116119384765625f;
            bool _826;
            if (((_794 / _696) - _800) < _820)
            {
                _826 = (_800 - _771) < _820;
            }
            else
            {
                _826 = false;
            }
            float _850;
            float _851;
            if (_826)
            {
                float _844 = abs(_800);
                if (_844 >= 11.0f)
                {
                    _683 = _306;
                    _687 = _686;
                    continue;
                }
                _850 = _306;
                _851 = _789 * smoothstep(11.0f, 8.0f, _844);
            }
            else
            {
                float _830 = abs(_772);
                if (_830 >= 11.0f)
                {
                    _683 = _306;
                    _687 = _686;
                    continue;
                }
                float _835 = _789 * smoothstep(11.0f, 8.0f, _830);
                float _839 = smoothstep(0.0f, 30.0f, _772) * smoothstep(0.20000000298023223876953125f * _243, 0.0f, _771);
                _850 = _306 + (_835 * _839);
                _851 = _835 * (1.0f - _839);
            }
            _683 = _850;
            _687 = _686 + (color_tex.SampleLevel(sampler_nnc, _232 + (_364 * (_826 ? _794 : _784)), min(_812, _814)) * _851);
        }
        _299 = _298 + _686;
    }
    float _865 = _305 * smoothstep(0.00999999977648258209228515625f * _298.w, max(9.9999997473787516355514526367188e-06f, 0.20000000298023223876953125f * _298.w), _305);
    float4 _926;
    [branch]
    if (int(_865 > 0.0f) != 0)
    {
        float _872 = -_243;
        float4 _874;
        _874 = 0.0f.xxxx;
        float4 _875;
        float _878;
        float _877 = _872;
        uint _879 = 0u;
        for (; _879 < 16u; _874 = _875, _877 = _878, _879++)
        {
            float2 _890 = frac(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(_879)) + 0.5f.xx) + _259.xy);
            float _892 = sqrt(_890.x);
            float _894 = _890.y * 6.283185482025146484375f;
            float2 _905 = _232 + (((float2(cos(_894) * _892, sin(_894) * _892) * 1.0f) / float2(color_tex_dims)) * _301);
            float4 _908 = coc_tex.SampleLevel(sampler_nnc, _905, 0.0f);
            float _909 = _908.x;
            if (_909 > (_877 - 10.0f))
            {
                _875 = _874 + color_tex.SampleLevel(sampler_llc, _905, 0.0f);
                _878 = max(_877, _909);
            }
            else
            {
                _875 = _874;
                _878 = _877;
            }
        }
        _926 = _298 + ((_874 / max(1.0f, _874.w).xxxx) * _865);
    }
    else
    {
        _926 = _298;
    }
    float4 _929 = (_208 * 0.125f.xxxx) + (_926 / _303.xxxx);
    img_output[_201] = float4((_929 / max(1.0000000133514319600180897396058e-10f, _929.w).xxxx).xyz, 1.0f);
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_WorkGroupID = stage_input.gl_WorkGroupID;
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_72a2a95d73e37edc) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)

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

struct DofTileOffsets
{
    uint offsets[8];
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
    float bokeh_anisotropy : packoffset(c0);
    uint2 blue_noise_texture_dims : packoffset(c0.y);
    uint2 color_tex_dims : packoffset(c1);
    uint2 coc_tiles_dilated_tex_dims : packoffset(c1.z);
    uint2 depth_tex_dims : packoffset(c2);
    uint2 coc_tex_dims : packoffset(c2.z);
    uint2 img_output_dims : packoffset(c3);
    uint2 halfres_tex_dims : packoffset(c3.z);
};

cbuffer spatial_resolve_offsets : register(b21, space0)
{
    int4 spatial_resolve_offsets_spatial_resolve_offsets_array[512] : packoffset(c0);
};

ByteAddressBuffer tile_offsets_buffer : register(t22, space0);
ByteAddressBuffer tile_coords_buffer : register(t23, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_llc : register(s3, space0);
Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> color_tex : register(t15, space0);
Texture2D<float4> coc_tiles_dilated_tex : register(t16, space0);
Texture2D<float4> coc_tex : register(t18, space0);
RWTexture2D<float4> img_output : register(u19, space0);
Texture2D<float4> halfres_tex : register(t20, space0);

static uint3 gl_WorkGroupID;
static uint3 gl_GlobalInvocationID;
static uint gl_LocalInvocationIndex;
struct SPIRV_Cross_Input
{
    uint3 gl_WorkGroupID : SV_GroupID;
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
    uint gl_LocalInvocationIndex : SV_GroupIndex;
};

groupshared uint ring_samples[64][9];

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
    uint2 _199 = (gl_GlobalInvocationID.xy & uint2(7u, 7u)) + (uint2(8u, 8u) * tile_coords_buffer.Load<uint2>((tile_offsets_buffer.Load<uint>(0) + gl_WorkGroupID.x) * 8 + 0));
    uint2 _200 = _199 & uint2(1u, 1u);
    uint _204 = _200.x + (_200.y * 2u);
    float4 _206;
    _206 = 0.0f.xxxx;
    for (uint _209 = 0u; _209 < 8u; )
    {
        _206 += halfres_tex.Load(int3((_199 / uint2(2u, 2u)) + uint2(spatial_resolve_offsets_spatial_resolve_offsets_array[(_204 * 16u) + _209].xy), 0u));
        _209++;
        continue;
    }
    float2 _230 = (float2(int2(_199)) + 0.5f.xx) / float2(img_output_dims);
    float _241 = min(float(color_tex_dims.y) * 0.0416666679084300994873046875f, coc_tiles_dilated_tex.SampleLevel(sampler_nnc, _230, 0.0f).x);
    float4 _257 = ((blue_noise_texture.Load(int3(((_199 + uint2(13u, 13u)) + uint2(int2(int(frame_constants_frame_index * 59u), int(frame_constants_frame_index * 37u)))) & uint2(255u, 255u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
    float _258 = _257.z;
    float _259 = _257.y;
    float _260 = _241 * 0.125f;
    float _261 = _257.x;
    bool _262 = _258 < 0.5f;
    float _266;
    if (_262)
    {
        _266 = 1.0f - _258;
    }
    else
    {
        _266 = _258;
    }
    float _267 = _262 ? (-1.0f) : 1.0f;
    float4 _269;
    float _272;
    float _274;
    float _276;
    _269 = 0.0f.xxxx;
    _272 = 0.0f;
    _274 = 0.0f;
    _276 = 0.0f;
    float4 _270;
    float _273;
    float _281;
    float _283;
    uint _181[12];
    float _275;
    float _277;
    uint _278 = 0u;
    float _280 = 10000000000.0f;
    float _282 = -10000000000.0f;
    [loop]
    for (; _278 < 4u; _269 = _270, _272 = _273, _274 = _275, _276 = _277, _278++, _280 = _281, _282 = _283)
    {
        float _289 = (float(_278) + _261) * 0.785398185253143310546875f;
        float2 _292 = float2(cos(_289), sin(_289));
        float2 _330;
        if (bokeh_anisotropy > 0.0f)
        {
            float4 _305 = mul(frame_constants_view_constants[0u].clip_to_view, float4((_230 - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f));
            float2 _309 = _305.xy / _305.z.xx;
            _309.y = _309.y * (-1.0f);
            float2 _313 = normalize(_309);
            float2x2 _318 = float2x2(_313, float2(-_313.y, _313.x));
            float _323 = lerp(0.699999988079071044921875f, 1.0f, exp(((-4.0f) * bokeh_anisotropy) * dot(_309, _309)));
            _330 = mul(transpose(_318), mul(_318, _292) * float2(_323, 1.0f / _323));
        }
        else
        {
            _330 = _292;
        }
        float2 _335 = _330 * (_267.xx / float2(color_tex_dims));
        float _344 = -_241;
        float _345 = clamp(coc_tex.SampleLevel(sampler_nnc, _230 + (_335 * (((-8.0f) + _259) * _260)), 0.0f).x, _344, _241);
        float _346 = abs(_345);
        float _348 = _261 * 0.20000000298023223876953125f;
        float _349 = lerp(_272, max(_272, _346), _348);
        float _361 = clamp(coc_tex.SampleLevel(sampler_nnc, _230 + (_335 * (((-7.0f) + _259) * _260)), 0.0f).x, _344, _241);
        float _364 = lerp(_349, max(_349, abs(_361)), _348);
        ring_samples[gl_LocalInvocationIndex][0u] = spvPackHalf2x16(float2(_345, 0.0f)) | (spvPackHalf2x16(float2(_361, 0.0f)) << 16u);
        float _377 = clamp(coc_tex.SampleLevel(sampler_nnc, _230 + (_335 * (((-6.0f) + _259) * _260)), 0.0f).x, _344, _241);
        float _378 = abs(_377);
        float _380 = lerp(_364, max(_364, _378), _348);
        float _392 = clamp(coc_tex.SampleLevel(sampler_nnc, _230 + (_335 * (((-5.0f) + _259) * _260)), 0.0f).x, _344, _241);
        float _395 = lerp(_380, max(_380, abs(_392)), _348);
        ring_samples[gl_LocalInvocationIndex][1u] = spvPackHalf2x16(float2(_377, 0.0f)) | (spvPackHalf2x16(float2(_392, 0.0f)) << 16u);
        float _408 = clamp(coc_tex.SampleLevel(sampler_nnc, _230 + (_335 * (((-4.0f) + _259) * _260)), 0.0f).x, _344, _241);
        float _409 = abs(_408);
        float _411 = lerp(_395, max(_395, _409), _348);
        float _423 = clamp(coc_tex.SampleLevel(sampler_nnc, _230 + (_335 * (((-3.0f) + _259) * _260)), 0.0f).x, _344, _241);
        float _426 = lerp(_411, max(_411, abs(_423)), _348);
        ring_samples[gl_LocalInvocationIndex][2u] = spvPackHalf2x16(float2(_408, 0.0f)) | (spvPackHalf2x16(float2(_423, 0.0f)) << 16u);
        float _439 = clamp(coc_tex.SampleLevel(sampler_nnc, _230 + (_335 * (((-2.0f) + _259) * _260)), 0.0f).x, _344, _241);
        float _440 = abs(_439);
        float _442 = lerp(_426, max(_426, _440), _348);
        float _454 = clamp(coc_tex.SampleLevel(sampler_nnc, _230 + (_335 * (((-1.0f) + _259) * _260)), 0.0f).x, _344, _241);
        float _457 = lerp(_442, max(_442, abs(_454)), _348);
        ring_samples[gl_LocalInvocationIndex][3u] = spvPackHalf2x16(float2(_439, 0.0f)) | (spvPackHalf2x16(float2(_454, 0.0f)) << 16u);
        float _469 = clamp(coc_tex.SampleLevel(sampler_nnc, _230 + (_335 * (_259 * _260)), 0.0f).x, _344, _241);
        float _470 = abs(_469);
        float _472 = lerp(_457, max(_457, _470), _348);
        float _484 = clamp(coc_tex.SampleLevel(sampler_nnc, _230 + (_335 * ((1.0f + _259) * _260)), 0.0f).x, _344, _241);
        float _487 = lerp(_472, max(_472, abs(_484)), _348);
        ring_samples[gl_LocalInvocationIndex][4u] = spvPackHalf2x16(float2(_469, 0.0f)) | (spvPackHalf2x16(float2(_484, 0.0f)) << 16u);
        float _500 = clamp(coc_tex.SampleLevel(sampler_nnc, _230 + (_335 * ((2.0f + _259) * _260)), 0.0f).x, _344, _241);
        float _501 = abs(_500);
        float _503 = lerp(_487, max(_487, _501), _348);
        float _515 = clamp(coc_tex.SampleLevel(sampler_nnc, _230 + (_335 * ((3.0f + _259) * _260)), 0.0f).x, _344, _241);
        float _518 = lerp(_503, max(_503, abs(_515)), _348);
        ring_samples[gl_LocalInvocationIndex][5u] = spvPackHalf2x16(float2(_500, 0.0f)) | (spvPackHalf2x16(float2(_515, 0.0f)) << 16u);
        float _531 = clamp(coc_tex.SampleLevel(sampler_nnc, _230 + (_335 * ((4.0f + _259) * _260)), 0.0f).x, _344, _241);
        float _532 = abs(_531);
        float _534 = lerp(_518, max(_518, _532), _348);
        float _546 = clamp(coc_tex.SampleLevel(sampler_nnc, _230 + (_335 * ((5.0f + _259) * _260)), 0.0f).x, _344, _241);
        float _549 = lerp(_534, max(_534, abs(_546)), _348);
        ring_samples[gl_LocalInvocationIndex][6u] = spvPackHalf2x16(float2(_531, 0.0f)) | (spvPackHalf2x16(float2(_546, 0.0f)) << 16u);
        float _562 = clamp(coc_tex.SampleLevel(sampler_nnc, _230 + (_335 * ((6.0f + _259) * _260)), 0.0f).x, _344, _241);
        float _563 = abs(_562);
        float _565 = lerp(_549, max(_549, _563), _348);
        float _577 = clamp(coc_tex.SampleLevel(sampler_nnc, _230 + (_335 * ((7.0f + _259) * _260)), 0.0f).x, _344, _241);
        float _580 = lerp(_565, max(_565, abs(_577)), _348);
        ring_samples[gl_LocalInvocationIndex][7u] = spvPackHalf2x16(float2(_562, 0.0f)) | (spvPackHalf2x16(float2(_577, 0.0f)) << 16u);
        float _593 = clamp(coc_tex.SampleLevel(sampler_nnc, _230 + (_335 * ((8.0f + _259) * _260)), 0.0f).x, _344, _241);
        float _594 = abs(_593);
        _273 = lerp(_580, max(_580, _594), _348);
        _281 = min(min(min(min(min(min(min(min(min(_280, _346), _378), _409), _440), _470), _501), _532), _563), _594);
        _283 = max(max(max(max(max(max(max(max(max(_282, _346), _378), _409), _440), _470), _501), _532), _563), _594);
        ring_samples[gl_LocalInvocationIndex][8u] = spvPackHalf2x16(float2(_593, 0.0f));
        for (uint _600 = 0u; _600 < 12u; )
        {
            _181[_600] = 0u;
            _600++;
            continue;
        }
        for (uint _607 = 0u; _607 < 17u; _607++)
        {
            uint _615 = _607 & 1u;
            uint _616 = _607 >> 1u;
            float2 _182 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_616] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_616] >> 16u) & 65535u).x);
            float _631 = (float(int(_607 - 8u)) + _259) * _260;
            for (int _633 = 0; _633 < 12; _633++)
            {
                if ((_631 / (((float(_633) + _266) * 0.16666667163372039794921875f) - 1.0f)) >= _182[_615])
                {
                    _181[_633] |= (1u << (_607 & 31u));
                }
            }
        }
        float4 _656;
        _277 = _276;
        _275 = _274;
        _656 = 0.0f.xxxx;
        float _655;
        float _653;
        float4 _657;
        for (int _658 = 0; _658 < 12; _277 = _653, _275 = _655, _656 = _657, _658++)
        {
            float _666 = ((float(_658) + _266) * 0.16666667163372039794921875f) - 1.0f;
            float _667 = abs(_666);
            _655 = _275 + _667;
            if (_181[_658] == 0u)
            {
                _653 = _277;
                _657 = _656;
                continue;
            }
            bool _673 = _666 >= 0.0f;
            float _741;
            float _742;
            uint _743;
            if (_673)
            {
                uint _708 = firstbitlow(_181[_658]);
                uint _712 = uint(max(0, (int(_708) - 1)));
                uint _714 = _708 >> 1u;
                float2 _183 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_714] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_714] >> 16u) & 65535u).x);
                uint _728 = _712 >> 1u;
                float2 _184 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_728] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_728] >> 16u) & 65535u).x);
                _741 = _183[_708 & 1u];
                _742 = _184[_712 & 1u];
                _743 = _708;
            }
            else
            {
                uint _677 = firstbithigh(_181[_658]);
                uint _679 = min(16u, (_677 + 1u));
                uint _681 = _677 >> 1u;
                float2 _185 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_681] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_681] >> 16u) & 65535u).x);
                uint _695 = _679 >> 1u;
                float2 _186 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_695] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_695] >> 16u) & 65535u).x);
                _741 = _185[_677 & 1u];
                _742 = _186[_679 & 1u];
                _743 = _677;
            }
            float _749 = (float(int(_743 - 8u)) + _259) * _260;
            float _750 = _749 / _666;
            float _751 = _260 * (_673 ? 1.0f : (-1.0f));
            float _754 = _749 - _751;
            float _755 = _742 - (_750 - (_751 / _666));
            float _759 = _667 * ((_181[_658] != 0u) ? 1.0f : 0.0f);
            float _764 = lerp(_754, _749, clamp(_755 / (_755 + (-(_741 - _750))), 0.0f, 1.0f));
            float _770 = clamp(coc_tex.SampleLevel(sampler_nnc, _230 + (_335 * _764), 0.0f).x, _344, _241);
            float _778 = log2(max(0.0f, abs(_741) * 0.125f)) * smoothstep(4.0f, 0.0f, _283 - _281);
            float _781 = WaveReadLaneAt(_778, WaveGetLaneIndex() ^ 1u);
            float _782 = min(_778, _781);
            float _784 = WaveReadLaneAt(_782, WaveGetLaneIndex() ^ 8u);
            if (_741 > _241)
            {
                _653 = _277;
                _657 = _656;
                continue;
            }
            float _790 = _755 * 0.100000001490116119384765625f;
            bool _796;
            if (((_764 / _666) - _770) < _790)
            {
                _796 = (_770 - _741) < _790;
            }
            else
            {
                _796 = false;
            }
            float _820;
            float _821;
            if (_796)
            {
                float _814 = abs(_770);
                if (_814 >= 11.0f)
                {
                    _653 = _277;
                    _657 = _656;
                    continue;
                }
                _820 = _277;
                _821 = _759 * smoothstep(11.0f, 8.0f, _814);
            }
            else
            {
                float _800 = abs(_742);
                if (_800 >= 11.0f)
                {
                    _653 = _277;
                    _657 = _656;
                    continue;
                }
                float _805 = _759 * smoothstep(11.0f, 8.0f, _800);
                float _809 = smoothstep(0.0f, 30.0f, _742) * smoothstep(0.20000000298023223876953125f * _241, 0.0f, _741);
                _820 = _277 + (_805 * _809);
                _821 = _805 * (1.0f - _809);
            }
            _653 = _820;
            _657 = _656 + (color_tex.SampleLevel(sampler_nnc, _230 + (_335 * (_796 ? _764 : _754)), min(_782, _784)) * _821);
        }
        _270 = _269 + _656;
    }
    float _835 = _276 * smoothstep(0.00999999977648258209228515625f * _269.w, max(9.9999997473787516355514526367188e-06f, 0.20000000298023223876953125f * _269.w), _276);
    float4 _896;
    [branch]
    if (int(_835 > 0.0f) != 0)
    {
        float _842 = -_241;
        float4 _844;
        _844 = 0.0f.xxxx;
        float4 _845;
        float _848;
        float _847 = _842;
        uint _849 = 0u;
        for (; _849 < 16u; _844 = _845, _847 = _848, _849++)
        {
            float2 _860 = frac(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(_849)) + 0.5f.xx) + _257.xy);
            float _862 = sqrt(_860.x);
            float _864 = _860.y * 6.283185482025146484375f;
            float2 _875 = _230 + (((float2(cos(_864) * _862, sin(_864) * _862) * 1.0f) / float2(color_tex_dims)) * _272);
            float4 _878 = coc_tex.SampleLevel(sampler_nnc, _875, 0.0f);
            float _879 = _878.x;
            if (_879 > (_847 - 10.0f))
            {
                _845 = _844 + color_tex.SampleLevel(sampler_llc, _875, 0.0f);
                _848 = max(_847, _879);
            }
            else
            {
                _845 = _844;
                _848 = _847;
            }
        }
        _896 = _269 + ((_844 / max(1.0f, _844.w).xxxx) * _835);
    }
    else
    {
        _896 = _269;
    }
    float4 _899 = (_206 * 0.125f.xxxx) + (_896 / _274.xxxx);
    img_output[_199] = float4((_899 / max(1.0000000133514319600180897396058e-10f, _899.w).xxxx).xyz, 1.0f);
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_WorkGroupID = stage_input.gl_WorkGroupID;
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
