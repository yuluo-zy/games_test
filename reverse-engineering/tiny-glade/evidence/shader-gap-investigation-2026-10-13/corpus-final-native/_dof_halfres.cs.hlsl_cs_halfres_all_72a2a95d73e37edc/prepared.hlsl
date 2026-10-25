#define PERM_quality 1
#define TGV_cs_halfres_all_72a2a95d73e37edc 1
#line 1 "shaders/dof/halfres.cs.hlsl"


#if !((defined(TGV_cs_halfres_6f1dd5dd8a66e104) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)) || (defined(TGV_cs_halfres_72a2a95d73e37edc) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)) || (defined(TGV_cs_halfres_all_6f1dd5dd8a66e104) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)) || (defined(TGV_cs_halfres_all_72a2a95d73e37edc) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))







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
#line 11 "shaders/dof/halfres.cs.hlsl"

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
#line 12 "shaders/dof/halfres.cs.hlsl"

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
#line 13 "shaders/dof/halfres.cs.hlsl"

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
#line 14 "shaders/dof/halfres.cs.hlsl"

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
#line 15 "shaders/dof/halfres.cs.hlsl"

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
#line 16 "shaders/dof/halfres.cs.hlsl"

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
#line 17 "shaders/dof/halfres.cs.hlsl"

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
#line 18 "shaders/dof/halfres.cs.hlsl"

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
#line 19 "shaders/dof/halfres.cs.hlsl"

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
#line 20 "shaders/dof/halfres.cs.hlsl"

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
#line 21 "shaders/dof/halfres.cs.hlsl"

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
#line 22 "shaders/dof/halfres.cs.hlsl"

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
#line 23 "shaders/dof/halfres.cs.hlsl"

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
#line 24 "shaders/dof/halfres.cs.hlsl"

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
#line 25 "shaders/dof/halfres.cs.hlsl"


cbuffer $Globals : register(b13, space0)
{
    float bokeh_anisotropy;
    uint2 blue_noise_texture_dims;
    uint2 color_tex_dims;
    uint2 coc_tiles_dilated_tex_dims;
    uint2 depth_tex_dims;
    uint2 coc_tex_dims;
    uint2 img_output_dims;
};

Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> color_tex : register(t15, space0);
Texture2D<float4> coc_tiles_dilated_tex : register(t16, space0);
Texture2D<float4> coc_tex : register(t18, space0);
RWTexture2D<half4> img_output : register(u19, space0);
StructuredBuffer<DofTileOffsets> tile_offsets_buffer : register(t20, space0);
StructuredBuffer<uint2> tile_coords_buffer : register(t21, space0);

groupshared uint dof_ring_samples[64][9];

void halfres_core(uint offsets_slot,
                  bool occlude_gate,
                  uint3 gl_WorkGroupID,
                  uint3 gl_GlobalInvocationID,
                  uint gl_LocalInvocationIndex)
{
    uint2 pixel = (gl_GlobalInvocationID.xy & uint2(7u, 7u)) + (uint2(8u, 8u) * tile_coords_buffer[tile_offsets_buffer[0].offsets[offsets_slot] + gl_WorkGroupID.x]);
    float2 uv = (float2(int2(pixel)) + 0.5f) / float2(img_output_dims);

    float coc_limit = min(float(color_tex_dims.y) * 0.0416666679084300994873046875f, coc_tiles_dilated_tex.SampleLevel(sampler_nnc, uv, 0.0f).x);

    
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
            float4 ctv = mul(view_constants[0].clip_to_view, float4((uv - 0.5f) * float2(2.0f, -2.0f), 0.0f, 1.0f));
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

        float coc0 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((-8.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float m = abs(coc0);
        float w = lerp(weight_accum, max(weight_accum, m), lerp_gain);
        float coc1 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((-7.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc1)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][0] = dof_pack_half2(float2(coc0, 0.0f)) | (dof_pack_half2(float2(coc1, 0.0f)) << 16u);

        float coc2 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((-6.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc2)), lerp_gain);
        float coc3 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((-5.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc3)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][1] = dof_pack_half2(float2(coc2, 0.0f)) | (dof_pack_half2(float2(coc3, 0.0f)) << 16u);

        float coc4 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((-4.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc4)), lerp_gain);
        float coc5 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((-3.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc5)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][2] = dof_pack_half2(float2(coc4, 0.0f)) | (dof_pack_half2(float2(coc5, 0.0f)) << 16u);

        float coc6 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((-2.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc6)), lerp_gain);
        float coc7 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((-1.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc7)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][3] = dof_pack_half2(float2(coc6, 0.0f)) | (dof_pack_half2(float2(coc7, 0.0f)) << 16u);

        float coc8 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * (dither_y * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc8)), lerp_gain);
        float coc9 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((1.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc9)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][4] = dof_pack_half2(float2(coc8, 0.0f)) | (dof_pack_half2(float2(coc9, 0.0f)) << 16u);

        float coc10 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((2.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc10)), lerp_gain);
        float coc11 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((3.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc11)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][5] = dof_pack_half2(float2(coc10, 0.0f)) | (dof_pack_half2(float2(coc11, 0.0f)) << 16u);

        float coc12 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((4.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc12)), lerp_gain);
        float coc13 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((5.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc13)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][6] = dof_pack_half2(float2(coc12, 0.0f)) | (dof_pack_half2(float2(coc13, 0.0f)) << 16u);

        float coc14 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((6.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc14)), lerp_gain);
        float coc15 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((7.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc15)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][7] = dof_pack_half2(float2(coc14, 0.0f)) | (dof_pack_half2(float2(coc15, 0.0f)) << 16u);

        float coc16 = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * ((8.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
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
            float coc_at_resolved = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (step_px * resolved), 0.0f).x, neg_coc, coc_limit);
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
                if (occlude_gate)
                {
                    float abs_resolved = abs(coc_at_resolved);
                    if (abs_resolved < 8.0f)
                    {
                        continue;
                    }
                    ring_color = slot_color;
                    ring_weight = slot_span * smoothstep(8.0f, 11.0f, abs_resolved);
                }
                else
                {
                    ring_color = slot_color;
                    ring_weight = slot_span;
                }
            }
            else
            {
                if (occlude_gate)
                {
                    float abs_hi = abs(hi_coc);
                    if (abs_hi < 8.0f)
                    {
                        continue;
                    }
                    float w2 = slot_span * smoothstep(8.0f, 11.0f, abs_hi);
                    float depth_gate = smoothstep(0.0f, 30.0f, hi_coc) * smoothstep(0.2f * coc_limit, 0.0f, lo_coc);
                    ring_color = slot_color + (w2 * depth_gate);
                    ring_weight = w2 * (1.0f - depth_gate);
                }
                else
                {
                    float w2 = slot_span;
                    float depth_gate = smoothstep(0.0f, 30.0f, hi_coc) * smoothstep(0.2f * coc_limit, 0.0f, lo_coc);
                    ring_color = slot_color + (w2 * depth_gate);
                    ring_weight = w2 * (1.0f - depth_gate);
                }
            }
            depth_weight = ring_color.w;
            slot_color = slot_color + (color_tex.SampleLevel(sampler_nnc, uv + (step_px * (close_to_edge ? resolved : lo_edge)), min(occlusion, occ_wide)) * ring_weight);
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
            float2 bg_uv = uv + (((float2(cos(phi) * r, sin(phi) * r) * 1.0f) / float2(color_tex_dims)) * weight_accum);
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
    img_output[pixel] = max(0.0f, final_color / depth_weight);
}

[numthreads(8, 8, 1)]
void cs_halfres(uint3 gl_WorkGroupID : SV_GroupID,
                uint3 gl_GlobalInvocationID : SV_DispatchThreadID,
                uint gl_LocalInvocationIndex : SV_GroupIndex)
{
    halfres_core(4u, true, gl_WorkGroupID, gl_GlobalInvocationID, gl_LocalInvocationIndex);
}

[numthreads(8, 8, 1)]
void cs_halfres_all(uint3 gl_WorkGroupID : SV_GroupID,
                    uint3 gl_GlobalInvocationID : SV_DispatchThreadID,
                    uint gl_LocalInvocationIndex : SV_GroupIndex)
{
    halfres_core(5u, false, gl_WorkGroupID, gl_GlobalInvocationID, gl_LocalInvocationIndex);
}

#endif

#if defined(TGV_cs_halfres_6f1dd5dd8a66e104) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)

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
};

ByteAddressBuffer tile_offsets_buffer : register(t20, space0);
ByteAddressBuffer tile_coords_buffer : register(t21, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_llc : register(s3, space0);
Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> color_tex : register(t15, space0);
Texture2D<float4> coc_tiles_dilated_tex : register(t16, space0);
Texture2D<float4> coc_tex : register(t18, space0);
RWTexture2D<float4> img_output : register(u19, space0);

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
    uint2 _189 = (gl_GlobalInvocationID.xy & uint2(7u, 7u)) + (uint2(8u, 8u) * tile_coords_buffer.Load<uint2>((tile_offsets_buffer.Load<uint>(16) + gl_WorkGroupID.x) * 8 + 0));
    float2 _196 = (float2(int2(_189)) + 0.5f.xx) / float2(img_output_dims);
    float _207 = min(float(color_tex_dims.y) * 0.0416666679084300994873046875f, coc_tiles_dilated_tex.SampleLevel(sampler_nnc, _196, 0.0f).x);
    float4 _223 = ((blue_noise_texture.Load(int3(((_189 + uint2(13u, 13u)) + uint2(int2(int(frame_constants_frame_index * 59u), int(frame_constants_frame_index * 37u)))) & uint2(255u, 255u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
    uint2 _224 = uint2(7u, 7u) + _189;
    float _236 = (frac(52.98291778564453125f * frac((0.067110560834407806396484375f * float(_224.x)) + (0.005837149918079376220703125f * float(_224.y)))) - 0.5f) * 0.5f;
    float _237 = 0.5f + _236;
    float _238 = _223.y;
    float _239 = _207 * 0.125f;
    float _244 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
    float _254 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_189.x) + _244)) + (0.005837149918079376220703125f * (float(_189.y) + _244))));
    bool _255 = _237 < 0.5f;
    float _259;
    if (_255)
    {
        _259 = 0.5f - _236;
    }
    else
    {
        _259 = _237;
    }
    float _260 = _255 ? (-1.0f) : 1.0f;
    float4 _262;
    float _265;
    float _267;
    float _269;
    _262 = 0.0f.xxxx;
    _265 = 0.0f;
    _267 = 0.0f;
    _269 = 0.0f;
    float4 _263;
    float _266;
    float _274;
    float _276;
    uint _171[8];
    float _268;
    float _270;
    uint _271 = 0u;
    float _273 = 10000000000.0f;
    float _275 = -10000000000.0f;
    [loop]
    for (; _271 < 2u; _262 = _263, _265 = _266, _267 = _268, _269 = _270, _271++, _273 = _274, _275 = _276)
    {
        float _282 = (float(_271) + _254) * 1.57079637050628662109375f;
        float2 _285 = float2(cos(_282), sin(_282));
        float2 _323;
        if (bokeh_anisotropy > 0.0f)
        {
            float4 _298 = mul(frame_constants_view_constants[0u].clip_to_view, float4((_196 - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f));
            float2 _302 = _298.xy / _298.z.xx;
            _302.y = _302.y * (-1.0f);
            float2 _306 = normalize(_302);
            float2x2 _311 = float2x2(_306, float2(-_306.y, _306.x));
            float _316 = lerp(0.699999988079071044921875f, 1.0f, exp(((-4.0f) * bokeh_anisotropy) * dot(_302, _302)));
            _323 = mul(transpose(_311), mul(_311, _285) * float2(_316, 1.0f / _316));
        }
        else
        {
            _323 = _285;
        }
        float2 _328 = _323 * (_260.xx / float2(color_tex_dims));
        float _337 = -_207;
        float _338 = clamp(coc_tex.SampleLevel(sampler_nnc, _196 + (_328 * (((-8.0f) + _238) * _239)), 0.0f).x, _337, _207);
        float _339 = abs(_338);
        float _342 = _223.x * 0.20000000298023223876953125f;
        float _343 = lerp(_265, max(_265, _339), _342);
        float _355 = clamp(coc_tex.SampleLevel(sampler_nnc, _196 + (_328 * (((-7.0f) + _238) * _239)), 0.0f).x, _337, _207);
        float _358 = lerp(_343, max(_343, abs(_355)), _342);
        ring_samples[gl_LocalInvocationIndex][0u] = spvPackHalf2x16(float2(_338, 0.0f)) | (spvPackHalf2x16(float2(_355, 0.0f)) << 16u);
        float _371 = clamp(coc_tex.SampleLevel(sampler_nnc, _196 + (_328 * (((-6.0f) + _238) * _239)), 0.0f).x, _337, _207);
        float _372 = abs(_371);
        float _374 = lerp(_358, max(_358, _372), _342);
        float _386 = clamp(coc_tex.SampleLevel(sampler_nnc, _196 + (_328 * (((-5.0f) + _238) * _239)), 0.0f).x, _337, _207);
        float _389 = lerp(_374, max(_374, abs(_386)), _342);
        ring_samples[gl_LocalInvocationIndex][1u] = spvPackHalf2x16(float2(_371, 0.0f)) | (spvPackHalf2x16(float2(_386, 0.0f)) << 16u);
        float _402 = clamp(coc_tex.SampleLevel(sampler_nnc, _196 + (_328 * (((-4.0f) + _238) * _239)), 0.0f).x, _337, _207);
        float _403 = abs(_402);
        float _405 = lerp(_389, max(_389, _403), _342);
        float _417 = clamp(coc_tex.SampleLevel(sampler_nnc, _196 + (_328 * (((-3.0f) + _238) * _239)), 0.0f).x, _337, _207);
        float _420 = lerp(_405, max(_405, abs(_417)), _342);
        ring_samples[gl_LocalInvocationIndex][2u] = spvPackHalf2x16(float2(_402, 0.0f)) | (spvPackHalf2x16(float2(_417, 0.0f)) << 16u);
        float _433 = clamp(coc_tex.SampleLevel(sampler_nnc, _196 + (_328 * (((-2.0f) + _238) * _239)), 0.0f).x, _337, _207);
        float _434 = abs(_433);
        float _436 = lerp(_420, max(_420, _434), _342);
        float _448 = clamp(coc_tex.SampleLevel(sampler_nnc, _196 + (_328 * (((-1.0f) + _238) * _239)), 0.0f).x, _337, _207);
        float _451 = lerp(_436, max(_436, abs(_448)), _342);
        ring_samples[gl_LocalInvocationIndex][3u] = spvPackHalf2x16(float2(_433, 0.0f)) | (spvPackHalf2x16(float2(_448, 0.0f)) << 16u);
        float _463 = clamp(coc_tex.SampleLevel(sampler_nnc, _196 + (_328 * (_238 * _239)), 0.0f).x, _337, _207);
        float _464 = abs(_463);
        float _466 = lerp(_451, max(_451, _464), _342);
        float _478 = clamp(coc_tex.SampleLevel(sampler_nnc, _196 + (_328 * ((1.0f + _238) * _239)), 0.0f).x, _337, _207);
        float _481 = lerp(_466, max(_466, abs(_478)), _342);
        ring_samples[gl_LocalInvocationIndex][4u] = spvPackHalf2x16(float2(_463, 0.0f)) | (spvPackHalf2x16(float2(_478, 0.0f)) << 16u);
        float _494 = clamp(coc_tex.SampleLevel(sampler_nnc, _196 + (_328 * ((2.0f + _238) * _239)), 0.0f).x, _337, _207);
        float _495 = abs(_494);
        float _497 = lerp(_481, max(_481, _495), _342);
        float _509 = clamp(coc_tex.SampleLevel(sampler_nnc, _196 + (_328 * ((3.0f + _238) * _239)), 0.0f).x, _337, _207);
        float _512 = lerp(_497, max(_497, abs(_509)), _342);
        ring_samples[gl_LocalInvocationIndex][5u] = spvPackHalf2x16(float2(_494, 0.0f)) | (spvPackHalf2x16(float2(_509, 0.0f)) << 16u);
        float _525 = clamp(coc_tex.SampleLevel(sampler_nnc, _196 + (_328 * ((4.0f + _238) * _239)), 0.0f).x, _337, _207);
        float _526 = abs(_525);
        float _528 = lerp(_512, max(_512, _526), _342);
        float _540 = clamp(coc_tex.SampleLevel(sampler_nnc, _196 + (_328 * ((5.0f + _238) * _239)), 0.0f).x, _337, _207);
        float _543 = lerp(_528, max(_528, abs(_540)), _342);
        ring_samples[gl_LocalInvocationIndex][6u] = spvPackHalf2x16(float2(_525, 0.0f)) | (spvPackHalf2x16(float2(_540, 0.0f)) << 16u);
        float _556 = clamp(coc_tex.SampleLevel(sampler_nnc, _196 + (_328 * ((6.0f + _238) * _239)), 0.0f).x, _337, _207);
        float _557 = abs(_556);
        float _559 = lerp(_543, max(_543, _557), _342);
        float _571 = clamp(coc_tex.SampleLevel(sampler_nnc, _196 + (_328 * ((7.0f + _238) * _239)), 0.0f).x, _337, _207);
        float _574 = lerp(_559, max(_559, abs(_571)), _342);
        ring_samples[gl_LocalInvocationIndex][7u] = spvPackHalf2x16(float2(_556, 0.0f)) | (spvPackHalf2x16(float2(_571, 0.0f)) << 16u);
        float _587 = clamp(coc_tex.SampleLevel(sampler_nnc, _196 + (_328 * ((8.0f + _238) * _239)), 0.0f).x, _337, _207);
        float _588 = abs(_587);
        _266 = lerp(_574, max(_574, _588), _342);
        _274 = min(min(min(min(min(min(min(min(min(_273, _339), _372), _403), _434), _464), _495), _526), _557), _588);
        _276 = max(max(max(max(max(max(max(max(max(_275, _339), _372), _403), _434), _464), _495), _526), _557), _588);
        ring_samples[gl_LocalInvocationIndex][8u] = spvPackHalf2x16(float2(_587, 0.0f));
        for (uint _594 = 0u; _594 < 8u; )
        {
            _171[_594] = 0u;
            _594++;
            continue;
        }
        for (uint _601 = 0u; _601 < 17u; _601++)
        {
            uint _609 = _601 & 1u;
            uint _610 = _601 >> 1u;
            float2 _172 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_610] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_610] >> 16u) & 65535u).x);
            float _625 = (float(int(_601 - 8u)) + _238) * _239;
            for (int _627 = 0; _627 < 8; _627++)
            {
                if ((_625 / (((float(_627) + _259) * 0.25f) - 1.0f)) >= _172[_609])
                {
                    _171[_627] |= (1u << (_601 & 31u));
                }
            }
        }
        float4 _650;
        _270 = _269;
        _268 = _267;
        _650 = 0.0f.xxxx;
        float _649;
        float _647;
        float4 _651;
        for (int _652 = 0; _652 < 8; _270 = _647, _268 = _649, _650 = _651, _652++)
        {
            float _660 = ((float(_652) + _259) * 0.25f) - 1.0f;
            float _661 = abs(_660);
            _649 = _268 + _661;
            if (_171[_652] == 0u)
            {
                _647 = _270;
                _651 = _650;
                continue;
            }
            bool _667 = _660 >= 0.0f;
            float _735;
            float _736;
            uint _737;
            if (_667)
            {
                uint _702 = firstbitlow(_171[_652]);
                uint _706 = uint(max(0, (int(_702) - 1)));
                uint _708 = _702 >> 1u;
                float2 _173 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_708] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_708] >> 16u) & 65535u).x);
                uint _722 = _706 >> 1u;
                float2 _174 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_722] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_722] >> 16u) & 65535u).x);
                _735 = _173[_702 & 1u];
                _736 = _174[_706 & 1u];
                _737 = _702;
            }
            else
            {
                uint _671 = firstbithigh(_171[_652]);
                uint _673 = min(16u, (_671 + 1u));
                uint _675 = _671 >> 1u;
                float2 _175 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_675] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_675] >> 16u) & 65535u).x);
                uint _689 = _673 >> 1u;
                float2 _176 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_689] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_689] >> 16u) & 65535u).x);
                _735 = _175[_671 & 1u];
                _736 = _176[_673 & 1u];
                _737 = _671;
            }
            float _743 = (float(int(_737 - 8u)) + _238) * _239;
            float _744 = _743 / _660;
            float _745 = _239 * (_667 ? 1.0f : (-1.0f));
            float _748 = _743 - _745;
            float _749 = _736 - (_744 - (_745 / _660));
            float _753 = _661 * ((_171[_652] != 0u) ? 1.0f : 0.0f);
            float _758 = lerp(_748, _743, clamp(_749 / (_749 + (-(_735 - _744))), 0.0f, 1.0f));
            float _764 = clamp(coc_tex.SampleLevel(sampler_nnc, _196 + (_328 * _758), 0.0f).x, _337, _207);
            float _772 = log2(max(0.0f, abs(_735) * 0.125f)) * smoothstep(4.0f, 0.0f, _276 - _274);
            float _775 = WaveReadLaneAt(_772, WaveGetLaneIndex() ^ 1u);
            float _776 = min(_772, _775);
            float _778 = WaveReadLaneAt(_776, WaveGetLaneIndex() ^ 8u);
            if (_735 > _207)
            {
                _647 = _270;
                _651 = _650;
                continue;
            }
            float _784 = _749 * 0.100000001490116119384765625f;
            bool _790;
            if (((_758 / _660) - _764) < _784)
            {
                _790 = (_764 - _735) < _784;
            }
            else
            {
                _790 = false;
            }
            float _814;
            float _815;
            if (_790)
            {
                float _808 = abs(_764);
                if (_808 < 8.0f)
                {
                    _647 = _270;
                    _651 = _650;
                    continue;
                }
                _814 = _270;
                _815 = _753 * smoothstep(8.0f, 11.0f, _808);
            }
            else
            {
                float _794 = abs(_736);
                if (_794 < 8.0f)
                {
                    _647 = _270;
                    _651 = _650;
                    continue;
                }
                float _799 = _753 * smoothstep(8.0f, 11.0f, _794);
                float _803 = smoothstep(0.0f, 30.0f, _736) * smoothstep(0.20000000298023223876953125f * _207, 0.0f, _735);
                _814 = _270 + (_799 * _803);
                _815 = _799 * (1.0f - _803);
            }
            _647 = _814;
            _651 = _650 + (color_tex.SampleLevel(sampler_nnc, _196 + (_328 * (_790 ? _758 : _748)), min(_776, _778)) * _815);
        }
        _263 = _262 + _650;
    }
    float _829 = _269 * smoothstep(0.00999999977648258209228515625f * _262.w, max(9.9999997473787516355514526367188e-06f, 0.20000000298023223876953125f * _262.w), _269);
    float4 _890;
    [branch]
    if (int(_829 > 0.0f) != 0)
    {
        float _836 = -_207;
        float4 _838;
        _838 = 0.0f.xxxx;
        float4 _839;
        float _842;
        float _841 = _836;
        uint _843 = 0u;
        for (; _843 < 16u; _838 = _839, _841 = _842, _843++)
        {
            float2 _854 = frac(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(_843)) + 0.5f.xx) + _223.xy);
            float _856 = sqrt(_854.x);
            float _858 = _854.y * 6.283185482025146484375f;
            float2 _869 = _196 + (((float2(cos(_858) * _856, sin(_858) * _856) * 1.0f) / float2(color_tex_dims)) * _265);
            float4 _872 = coc_tex.SampleLevel(sampler_nnc, _869, 0.0f);
            float _873 = _872.x;
            if (_873 > (_841 - 10.0f))
            {
                _839 = _838 + color_tex.SampleLevel(sampler_llc, _869, 0.0f);
                _842 = max(_841, _873);
            }
            else
            {
                _839 = _838;
                _842 = _841;
            }
        }
        _890 = _262 + ((_838 / max(1.0f, _838.w).xxxx) * _829);
    }
    else
    {
        _890 = _262;
    }
    img_output[_189] = max(0.0f.xxxx, _890 / _267.xxxx);
}

[numthreads(8, 8, 1)]
void cs_halfres(SPIRV_Cross_Input stage_input)
{
    gl_WorkGroupID = stage_input.gl_WorkGroupID;
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_cs_halfres_72a2a95d73e37edc) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)

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
};

ByteAddressBuffer tile_offsets_buffer : register(t20, space0);
ByteAddressBuffer tile_coords_buffer : register(t21, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_llc : register(s3, space0);
Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> color_tex : register(t15, space0);
Texture2D<float4> coc_tiles_dilated_tex : register(t16, space0);
Texture2D<float4> coc_tex : register(t18, space0);
RWTexture2D<float4> img_output : register(u19, space0);

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
    uint2 _187 = (gl_GlobalInvocationID.xy & uint2(7u, 7u)) + (uint2(8u, 8u) * tile_coords_buffer.Load<uint2>((tile_offsets_buffer.Load<uint>(16) + gl_WorkGroupID.x) * 8 + 0));
    float2 _194 = (float2(int2(_187)) + 0.5f.xx) / float2(img_output_dims);
    float _205 = min(float(color_tex_dims.y) * 0.0416666679084300994873046875f, coc_tiles_dilated_tex.SampleLevel(sampler_nnc, _194, 0.0f).x);
    float4 _221 = ((blue_noise_texture.Load(int3(((_187 + uint2(13u, 13u)) + uint2(int2(int(frame_constants_frame_index * 59u), int(frame_constants_frame_index * 37u)))) & uint2(255u, 255u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
    float _222 = _221.z;
    float _223 = _221.y;
    float _224 = _205 * 0.125f;
    float _225 = _221.x;
    bool _226 = _222 < 0.5f;
    float _230;
    if (_226)
    {
        _230 = 1.0f - _222;
    }
    else
    {
        _230 = _222;
    }
    float _231 = _226 ? (-1.0f) : 1.0f;
    float4 _233;
    float _236;
    float _238;
    float _240;
    _233 = 0.0f.xxxx;
    _236 = 0.0f;
    _238 = 0.0f;
    _240 = 0.0f;
    float4 _234;
    float _237;
    float _245;
    float _247;
    uint _169[12];
    float _239;
    float _241;
    uint _242 = 0u;
    float _244 = 10000000000.0f;
    float _246 = -10000000000.0f;
    [loop]
    for (; _242 < 4u; _233 = _234, _236 = _237, _238 = _239, _240 = _241, _242++, _244 = _245, _246 = _247)
    {
        float _253 = (float(_242) + _225) * 0.785398185253143310546875f;
        float2 _256 = float2(cos(_253), sin(_253));
        float2 _294;
        if (bokeh_anisotropy > 0.0f)
        {
            float4 _269 = mul(frame_constants_view_constants[0u].clip_to_view, float4((_194 - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f));
            float2 _273 = _269.xy / _269.z.xx;
            _273.y = _273.y * (-1.0f);
            float2 _277 = normalize(_273);
            float2x2 _282 = float2x2(_277, float2(-_277.y, _277.x));
            float _287 = lerp(0.699999988079071044921875f, 1.0f, exp(((-4.0f) * bokeh_anisotropy) * dot(_273, _273)));
            _294 = mul(transpose(_282), mul(_282, _256) * float2(_287, 1.0f / _287));
        }
        else
        {
            _294 = _256;
        }
        float2 _299 = _294 * (_231.xx / float2(color_tex_dims));
        float _308 = -_205;
        float _309 = clamp(coc_tex.SampleLevel(sampler_nnc, _194 + (_299 * (((-8.0f) + _223) * _224)), 0.0f).x, _308, _205);
        float _310 = abs(_309);
        float _312 = _225 * 0.20000000298023223876953125f;
        float _313 = lerp(_236, max(_236, _310), _312);
        float _325 = clamp(coc_tex.SampleLevel(sampler_nnc, _194 + (_299 * (((-7.0f) + _223) * _224)), 0.0f).x, _308, _205);
        float _328 = lerp(_313, max(_313, abs(_325)), _312);
        ring_samples[gl_LocalInvocationIndex][0u] = spvPackHalf2x16(float2(_309, 0.0f)) | (spvPackHalf2x16(float2(_325, 0.0f)) << 16u);
        float _341 = clamp(coc_tex.SampleLevel(sampler_nnc, _194 + (_299 * (((-6.0f) + _223) * _224)), 0.0f).x, _308, _205);
        float _342 = abs(_341);
        float _344 = lerp(_328, max(_328, _342), _312);
        float _356 = clamp(coc_tex.SampleLevel(sampler_nnc, _194 + (_299 * (((-5.0f) + _223) * _224)), 0.0f).x, _308, _205);
        float _359 = lerp(_344, max(_344, abs(_356)), _312);
        ring_samples[gl_LocalInvocationIndex][1u] = spvPackHalf2x16(float2(_341, 0.0f)) | (spvPackHalf2x16(float2(_356, 0.0f)) << 16u);
        float _372 = clamp(coc_tex.SampleLevel(sampler_nnc, _194 + (_299 * (((-4.0f) + _223) * _224)), 0.0f).x, _308, _205);
        float _373 = abs(_372);
        float _375 = lerp(_359, max(_359, _373), _312);
        float _387 = clamp(coc_tex.SampleLevel(sampler_nnc, _194 + (_299 * (((-3.0f) + _223) * _224)), 0.0f).x, _308, _205);
        float _390 = lerp(_375, max(_375, abs(_387)), _312);
        ring_samples[gl_LocalInvocationIndex][2u] = spvPackHalf2x16(float2(_372, 0.0f)) | (spvPackHalf2x16(float2(_387, 0.0f)) << 16u);
        float _403 = clamp(coc_tex.SampleLevel(sampler_nnc, _194 + (_299 * (((-2.0f) + _223) * _224)), 0.0f).x, _308, _205);
        float _404 = abs(_403);
        float _406 = lerp(_390, max(_390, _404), _312);
        float _418 = clamp(coc_tex.SampleLevel(sampler_nnc, _194 + (_299 * (((-1.0f) + _223) * _224)), 0.0f).x, _308, _205);
        float _421 = lerp(_406, max(_406, abs(_418)), _312);
        ring_samples[gl_LocalInvocationIndex][3u] = spvPackHalf2x16(float2(_403, 0.0f)) | (spvPackHalf2x16(float2(_418, 0.0f)) << 16u);
        float _433 = clamp(coc_tex.SampleLevel(sampler_nnc, _194 + (_299 * (_223 * _224)), 0.0f).x, _308, _205);
        float _434 = abs(_433);
        float _436 = lerp(_421, max(_421, _434), _312);
        float _448 = clamp(coc_tex.SampleLevel(sampler_nnc, _194 + (_299 * ((1.0f + _223) * _224)), 0.0f).x, _308, _205);
        float _451 = lerp(_436, max(_436, abs(_448)), _312);
        ring_samples[gl_LocalInvocationIndex][4u] = spvPackHalf2x16(float2(_433, 0.0f)) | (spvPackHalf2x16(float2(_448, 0.0f)) << 16u);
        float _464 = clamp(coc_tex.SampleLevel(sampler_nnc, _194 + (_299 * ((2.0f + _223) * _224)), 0.0f).x, _308, _205);
        float _465 = abs(_464);
        float _467 = lerp(_451, max(_451, _465), _312);
        float _479 = clamp(coc_tex.SampleLevel(sampler_nnc, _194 + (_299 * ((3.0f + _223) * _224)), 0.0f).x, _308, _205);
        float _482 = lerp(_467, max(_467, abs(_479)), _312);
        ring_samples[gl_LocalInvocationIndex][5u] = spvPackHalf2x16(float2(_464, 0.0f)) | (spvPackHalf2x16(float2(_479, 0.0f)) << 16u);
        float _495 = clamp(coc_tex.SampleLevel(sampler_nnc, _194 + (_299 * ((4.0f + _223) * _224)), 0.0f).x, _308, _205);
        float _496 = abs(_495);
        float _498 = lerp(_482, max(_482, _496), _312);
        float _510 = clamp(coc_tex.SampleLevel(sampler_nnc, _194 + (_299 * ((5.0f + _223) * _224)), 0.0f).x, _308, _205);
        float _513 = lerp(_498, max(_498, abs(_510)), _312);
        ring_samples[gl_LocalInvocationIndex][6u] = spvPackHalf2x16(float2(_495, 0.0f)) | (spvPackHalf2x16(float2(_510, 0.0f)) << 16u);
        float _526 = clamp(coc_tex.SampleLevel(sampler_nnc, _194 + (_299 * ((6.0f + _223) * _224)), 0.0f).x, _308, _205);
        float _527 = abs(_526);
        float _529 = lerp(_513, max(_513, _527), _312);
        float _541 = clamp(coc_tex.SampleLevel(sampler_nnc, _194 + (_299 * ((7.0f + _223) * _224)), 0.0f).x, _308, _205);
        float _544 = lerp(_529, max(_529, abs(_541)), _312);
        ring_samples[gl_LocalInvocationIndex][7u] = spvPackHalf2x16(float2(_526, 0.0f)) | (spvPackHalf2x16(float2(_541, 0.0f)) << 16u);
        float _557 = clamp(coc_tex.SampleLevel(sampler_nnc, _194 + (_299 * ((8.0f + _223) * _224)), 0.0f).x, _308, _205);
        float _558 = abs(_557);
        _237 = lerp(_544, max(_544, _558), _312);
        _245 = min(min(min(min(min(min(min(min(min(_244, _310), _342), _373), _404), _434), _465), _496), _527), _558);
        _247 = max(max(max(max(max(max(max(max(max(_246, _310), _342), _373), _404), _434), _465), _496), _527), _558);
        ring_samples[gl_LocalInvocationIndex][8u] = spvPackHalf2x16(float2(_557, 0.0f));
        for (uint _564 = 0u; _564 < 12u; )
        {
            _169[_564] = 0u;
            _564++;
            continue;
        }
        for (uint _571 = 0u; _571 < 17u; _571++)
        {
            uint _579 = _571 & 1u;
            uint _580 = _571 >> 1u;
            float2 _170 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_580] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_580] >> 16u) & 65535u).x);
            float _595 = (float(int(_571 - 8u)) + _223) * _224;
            for (int _597 = 0; _597 < 12; _597++)
            {
                if ((_595 / (((float(_597) + _230) * 0.16666667163372039794921875f) - 1.0f)) >= _170[_579])
                {
                    _169[_597] |= (1u << (_571 & 31u));
                }
            }
        }
        float4 _620;
        _241 = _240;
        _239 = _238;
        _620 = 0.0f.xxxx;
        float _619;
        float _617;
        float4 _621;
        for (int _622 = 0; _622 < 12; _241 = _617, _239 = _619, _620 = _621, _622++)
        {
            float _630 = ((float(_622) + _230) * 0.16666667163372039794921875f) - 1.0f;
            float _631 = abs(_630);
            _619 = _239 + _631;
            if (_169[_622] == 0u)
            {
                _617 = _241;
                _621 = _620;
                continue;
            }
            bool _637 = _630 >= 0.0f;
            float _705;
            float _706;
            uint _707;
            if (_637)
            {
                uint _672 = firstbitlow(_169[_622]);
                uint _676 = uint(max(0, (int(_672) - 1)));
                uint _678 = _672 >> 1u;
                float2 _171 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_678] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_678] >> 16u) & 65535u).x);
                uint _692 = _676 >> 1u;
                float2 _172 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_692] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_692] >> 16u) & 65535u).x);
                _705 = _171[_672 & 1u];
                _706 = _172[_676 & 1u];
                _707 = _672;
            }
            else
            {
                uint _641 = firstbithigh(_169[_622]);
                uint _643 = min(16u, (_641 + 1u));
                uint _645 = _641 >> 1u;
                float2 _173 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_645] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_645] >> 16u) & 65535u).x);
                uint _659 = _643 >> 1u;
                float2 _174 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_659] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_659] >> 16u) & 65535u).x);
                _705 = _173[_641 & 1u];
                _706 = _174[_643 & 1u];
                _707 = _641;
            }
            float _713 = (float(int(_707 - 8u)) + _223) * _224;
            float _714 = _713 / _630;
            float _715 = _224 * (_637 ? 1.0f : (-1.0f));
            float _718 = _713 - _715;
            float _719 = _706 - (_714 - (_715 / _630));
            float _723 = _631 * ((_169[_622] != 0u) ? 1.0f : 0.0f);
            float _728 = lerp(_718, _713, clamp(_719 / (_719 + (-(_705 - _714))), 0.0f, 1.0f));
            float _734 = clamp(coc_tex.SampleLevel(sampler_nnc, _194 + (_299 * _728), 0.0f).x, _308, _205);
            float _742 = log2(max(0.0f, abs(_705) * 0.125f)) * smoothstep(4.0f, 0.0f, _247 - _245);
            float _745 = WaveReadLaneAt(_742, WaveGetLaneIndex() ^ 1u);
            float _746 = min(_742, _745);
            float _748 = WaveReadLaneAt(_746, WaveGetLaneIndex() ^ 8u);
            if (_705 > _205)
            {
                _617 = _241;
                _621 = _620;
                continue;
            }
            float _754 = _719 * 0.100000001490116119384765625f;
            bool _760;
            if (((_728 / _630) - _734) < _754)
            {
                _760 = (_734 - _705) < _754;
            }
            else
            {
                _760 = false;
            }
            float _784;
            float _785;
            if (_760)
            {
                float _778 = abs(_734);
                if (_778 < 8.0f)
                {
                    _617 = _241;
                    _621 = _620;
                    continue;
                }
                _784 = _241;
                _785 = _723 * smoothstep(8.0f, 11.0f, _778);
            }
            else
            {
                float _764 = abs(_706);
                if (_764 < 8.0f)
                {
                    _617 = _241;
                    _621 = _620;
                    continue;
                }
                float _769 = _723 * smoothstep(8.0f, 11.0f, _764);
                float _773 = smoothstep(0.0f, 30.0f, _706) * smoothstep(0.20000000298023223876953125f * _205, 0.0f, _705);
                _784 = _241 + (_769 * _773);
                _785 = _769 * (1.0f - _773);
            }
            _617 = _784;
            _621 = _620 + (color_tex.SampleLevel(sampler_nnc, _194 + (_299 * (_760 ? _728 : _718)), min(_746, _748)) * _785);
        }
        _234 = _233 + _620;
    }
    float _799 = _240 * smoothstep(0.00999999977648258209228515625f * _233.w, max(9.9999997473787516355514526367188e-06f, 0.20000000298023223876953125f * _233.w), _240);
    float4 _860;
    [branch]
    if (int(_799 > 0.0f) != 0)
    {
        float _806 = -_205;
        float4 _808;
        _808 = 0.0f.xxxx;
        float4 _809;
        float _812;
        float _811 = _806;
        uint _813 = 0u;
        for (; _813 < 16u; _808 = _809, _811 = _812, _813++)
        {
            float2 _824 = frac(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(_813)) + 0.5f.xx) + _221.xy);
            float _826 = sqrt(_824.x);
            float _828 = _824.y * 6.283185482025146484375f;
            float2 _839 = _194 + (((float2(cos(_828) * _826, sin(_828) * _826) * 1.0f) / float2(color_tex_dims)) * _236);
            float4 _842 = coc_tex.SampleLevel(sampler_nnc, _839, 0.0f);
            float _843 = _842.x;
            if (_843 > (_811 - 10.0f))
            {
                _809 = _808 + color_tex.SampleLevel(sampler_llc, _839, 0.0f);
                _812 = max(_811, _843);
            }
            else
            {
                _809 = _808;
                _812 = _811;
            }
        }
        _860 = _233 + ((_808 / max(1.0f, _808.w).xxxx) * _799);
    }
    else
    {
        _860 = _233;
    }
    img_output[_187] = max(0.0f.xxxx, _860 / _238.xxxx);
}

[numthreads(8, 8, 1)]
void cs_halfres(SPIRV_Cross_Input stage_input)
{
    gl_WorkGroupID = stage_input.gl_WorkGroupID;
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_cs_halfres_all_6f1dd5dd8a66e104) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)

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
};

ByteAddressBuffer tile_offsets_buffer : register(t20, space0);
ByteAddressBuffer tile_coords_buffer : register(t21, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_llc : register(s3, space0);
Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> color_tex : register(t15, space0);
Texture2D<float4> coc_tiles_dilated_tex : register(t16, space0);
Texture2D<float4> coc_tex : register(t18, space0);
RWTexture2D<float4> img_output : register(u19, space0);

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
    uint2 _188 = (gl_GlobalInvocationID.xy & uint2(7u, 7u)) + (uint2(8u, 8u) * tile_coords_buffer.Load<uint2>((tile_offsets_buffer.Load<uint>(20) + gl_WorkGroupID.x) * 8 + 0));
    float2 _195 = (float2(int2(_188)) + 0.5f.xx) / float2(img_output_dims);
    float _206 = min(float(color_tex_dims.y) * 0.0416666679084300994873046875f, coc_tiles_dilated_tex.SampleLevel(sampler_nnc, _195, 0.0f).x);
    float4 _222 = ((blue_noise_texture.Load(int3(((_188 + uint2(13u, 13u)) + uint2(int2(int(frame_constants_frame_index * 59u), int(frame_constants_frame_index * 37u)))) & uint2(255u, 255u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
    uint2 _223 = uint2(7u, 7u) + _188;
    float _235 = (frac(52.98291778564453125f * frac((0.067110560834407806396484375f * float(_223.x)) + (0.005837149918079376220703125f * float(_223.y)))) - 0.5f) * 0.5f;
    float _236 = 0.5f + _235;
    float _237 = _222.y;
    float _238 = _206 * 0.125f;
    float _243 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
    float _253 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_188.x) + _243)) + (0.005837149918079376220703125f * (float(_188.y) + _243))));
    bool _254 = _236 < 0.5f;
    float _258;
    if (_254)
    {
        _258 = 0.5f - _235;
    }
    else
    {
        _258 = _236;
    }
    float _259 = _254 ? (-1.0f) : 1.0f;
    float4 _261;
    float _264;
    float _266;
    float _268;
    _261 = 0.0f.xxxx;
    _264 = 0.0f;
    _266 = 0.0f;
    _268 = 0.0f;
    float4 _262;
    float _265;
    float _273;
    float _275;
    uint _170[8];
    float _267;
    float _269;
    uint _270 = 0u;
    float _272 = 10000000000.0f;
    float _274 = -10000000000.0f;
    [loop]
    for (; _270 < 2u; _261 = _262, _264 = _265, _266 = _267, _268 = _269, _270++, _272 = _273, _274 = _275)
    {
        float _281 = (float(_270) + _253) * 1.57079637050628662109375f;
        float2 _284 = float2(cos(_281), sin(_281));
        float2 _322;
        if (bokeh_anisotropy > 0.0f)
        {
            float4 _297 = mul(frame_constants_view_constants[0u].clip_to_view, float4((_195 - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f));
            float2 _301 = _297.xy / _297.z.xx;
            _301.y = _301.y * (-1.0f);
            float2 _305 = normalize(_301);
            float2x2 _310 = float2x2(_305, float2(-_305.y, _305.x));
            float _315 = lerp(0.699999988079071044921875f, 1.0f, exp(((-4.0f) * bokeh_anisotropy) * dot(_301, _301)));
            _322 = mul(transpose(_310), mul(_310, _284) * float2(_315, 1.0f / _315));
        }
        else
        {
            _322 = _284;
        }
        float2 _327 = _322 * (_259.xx / float2(color_tex_dims));
        float _336 = -_206;
        float _337 = clamp(coc_tex.SampleLevel(sampler_nnc, _195 + (_327 * (((-8.0f) + _237) * _238)), 0.0f).x, _336, _206);
        float _338 = abs(_337);
        float _341 = _222.x * 0.20000000298023223876953125f;
        float _342 = lerp(_264, max(_264, _338), _341);
        float _354 = clamp(coc_tex.SampleLevel(sampler_nnc, _195 + (_327 * (((-7.0f) + _237) * _238)), 0.0f).x, _336, _206);
        float _357 = lerp(_342, max(_342, abs(_354)), _341);
        ring_samples[gl_LocalInvocationIndex][0u] = spvPackHalf2x16(float2(_337, 0.0f)) | (spvPackHalf2x16(float2(_354, 0.0f)) << 16u);
        float _370 = clamp(coc_tex.SampleLevel(sampler_nnc, _195 + (_327 * (((-6.0f) + _237) * _238)), 0.0f).x, _336, _206);
        float _371 = abs(_370);
        float _373 = lerp(_357, max(_357, _371), _341);
        float _385 = clamp(coc_tex.SampleLevel(sampler_nnc, _195 + (_327 * (((-5.0f) + _237) * _238)), 0.0f).x, _336, _206);
        float _388 = lerp(_373, max(_373, abs(_385)), _341);
        ring_samples[gl_LocalInvocationIndex][1u] = spvPackHalf2x16(float2(_370, 0.0f)) | (spvPackHalf2x16(float2(_385, 0.0f)) << 16u);
        float _401 = clamp(coc_tex.SampleLevel(sampler_nnc, _195 + (_327 * (((-4.0f) + _237) * _238)), 0.0f).x, _336, _206);
        float _402 = abs(_401);
        float _404 = lerp(_388, max(_388, _402), _341);
        float _416 = clamp(coc_tex.SampleLevel(sampler_nnc, _195 + (_327 * (((-3.0f) + _237) * _238)), 0.0f).x, _336, _206);
        float _419 = lerp(_404, max(_404, abs(_416)), _341);
        ring_samples[gl_LocalInvocationIndex][2u] = spvPackHalf2x16(float2(_401, 0.0f)) | (spvPackHalf2x16(float2(_416, 0.0f)) << 16u);
        float _432 = clamp(coc_tex.SampleLevel(sampler_nnc, _195 + (_327 * (((-2.0f) + _237) * _238)), 0.0f).x, _336, _206);
        float _433 = abs(_432);
        float _435 = lerp(_419, max(_419, _433), _341);
        float _447 = clamp(coc_tex.SampleLevel(sampler_nnc, _195 + (_327 * (((-1.0f) + _237) * _238)), 0.0f).x, _336, _206);
        float _450 = lerp(_435, max(_435, abs(_447)), _341);
        ring_samples[gl_LocalInvocationIndex][3u] = spvPackHalf2x16(float2(_432, 0.0f)) | (spvPackHalf2x16(float2(_447, 0.0f)) << 16u);
        float _462 = clamp(coc_tex.SampleLevel(sampler_nnc, _195 + (_327 * (_237 * _238)), 0.0f).x, _336, _206);
        float _463 = abs(_462);
        float _465 = lerp(_450, max(_450, _463), _341);
        float _477 = clamp(coc_tex.SampleLevel(sampler_nnc, _195 + (_327 * ((1.0f + _237) * _238)), 0.0f).x, _336, _206);
        float _480 = lerp(_465, max(_465, abs(_477)), _341);
        ring_samples[gl_LocalInvocationIndex][4u] = spvPackHalf2x16(float2(_462, 0.0f)) | (spvPackHalf2x16(float2(_477, 0.0f)) << 16u);
        float _493 = clamp(coc_tex.SampleLevel(sampler_nnc, _195 + (_327 * ((2.0f + _237) * _238)), 0.0f).x, _336, _206);
        float _494 = abs(_493);
        float _496 = lerp(_480, max(_480, _494), _341);
        float _508 = clamp(coc_tex.SampleLevel(sampler_nnc, _195 + (_327 * ((3.0f + _237) * _238)), 0.0f).x, _336, _206);
        float _511 = lerp(_496, max(_496, abs(_508)), _341);
        ring_samples[gl_LocalInvocationIndex][5u] = spvPackHalf2x16(float2(_493, 0.0f)) | (spvPackHalf2x16(float2(_508, 0.0f)) << 16u);
        float _524 = clamp(coc_tex.SampleLevel(sampler_nnc, _195 + (_327 * ((4.0f + _237) * _238)), 0.0f).x, _336, _206);
        float _525 = abs(_524);
        float _527 = lerp(_511, max(_511, _525), _341);
        float _539 = clamp(coc_tex.SampleLevel(sampler_nnc, _195 + (_327 * ((5.0f + _237) * _238)), 0.0f).x, _336, _206);
        float _542 = lerp(_527, max(_527, abs(_539)), _341);
        ring_samples[gl_LocalInvocationIndex][6u] = spvPackHalf2x16(float2(_524, 0.0f)) | (spvPackHalf2x16(float2(_539, 0.0f)) << 16u);
        float _555 = clamp(coc_tex.SampleLevel(sampler_nnc, _195 + (_327 * ((6.0f + _237) * _238)), 0.0f).x, _336, _206);
        float _556 = abs(_555);
        float _558 = lerp(_542, max(_542, _556), _341);
        float _570 = clamp(coc_tex.SampleLevel(sampler_nnc, _195 + (_327 * ((7.0f + _237) * _238)), 0.0f).x, _336, _206);
        float _573 = lerp(_558, max(_558, abs(_570)), _341);
        ring_samples[gl_LocalInvocationIndex][7u] = spvPackHalf2x16(float2(_555, 0.0f)) | (spvPackHalf2x16(float2(_570, 0.0f)) << 16u);
        float _586 = clamp(coc_tex.SampleLevel(sampler_nnc, _195 + (_327 * ((8.0f + _237) * _238)), 0.0f).x, _336, _206);
        float _587 = abs(_586);
        _265 = lerp(_573, max(_573, _587), _341);
        _273 = min(min(min(min(min(min(min(min(min(_272, _338), _371), _402), _433), _463), _494), _525), _556), _587);
        _275 = max(max(max(max(max(max(max(max(max(_274, _338), _371), _402), _433), _463), _494), _525), _556), _587);
        ring_samples[gl_LocalInvocationIndex][8u] = spvPackHalf2x16(float2(_586, 0.0f));
        for (uint _593 = 0u; _593 < 8u; )
        {
            _170[_593] = 0u;
            _593++;
            continue;
        }
        for (uint _600 = 0u; _600 < 17u; _600++)
        {
            uint _608 = _600 & 1u;
            uint _609 = _600 >> 1u;
            float2 _171 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_609] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_609] >> 16u) & 65535u).x);
            float _624 = (float(int(_600 - 8u)) + _237) * _238;
            for (int _626 = 0; _626 < 8; _626++)
            {
                if ((_624 / (((float(_626) + _258) * 0.25f) - 1.0f)) >= _171[_608])
                {
                    _170[_626] |= (1u << (_600 & 31u));
                }
            }
        }
        float4 _649;
        _269 = _268;
        _267 = _266;
        _649 = 0.0f.xxxx;
        float _648;
        float _646;
        float4 _650;
        for (int _651 = 0; _651 < 8; _269 = _646, _267 = _648, _649 = _650, _651++)
        {
            float _659 = ((float(_651) + _258) * 0.25f) - 1.0f;
            float _660 = abs(_659);
            _648 = _267 + _660;
            if (_170[_651] == 0u)
            {
                _646 = _269;
                _650 = _649;
                continue;
            }
            bool _666 = _659 >= 0.0f;
            float _734;
            float _735;
            uint _736;
            if (_666)
            {
                uint _701 = firstbitlow(_170[_651]);
                uint _705 = uint(max(0, (int(_701) - 1)));
                uint _707 = _701 >> 1u;
                float2 _172 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_707] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_707] >> 16u) & 65535u).x);
                uint _721 = _705 >> 1u;
                float2 _173 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_721] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_721] >> 16u) & 65535u).x);
                _734 = _172[_701 & 1u];
                _735 = _173[_705 & 1u];
                _736 = _701;
            }
            else
            {
                uint _670 = firstbithigh(_170[_651]);
                uint _672 = min(16u, (_670 + 1u));
                uint _674 = _670 >> 1u;
                float2 _174 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_674] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_674] >> 16u) & 65535u).x);
                uint _688 = _672 >> 1u;
                float2 _175 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_688] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_688] >> 16u) & 65535u).x);
                _734 = _174[_670 & 1u];
                _735 = _175[_672 & 1u];
                _736 = _670;
            }
            float _742 = (float(int(_736 - 8u)) + _237) * _238;
            float _743 = _742 / _659;
            float _744 = _238 * (_666 ? 1.0f : (-1.0f));
            float _747 = _742 - _744;
            float _748 = _735 - (_743 - (_744 / _659));
            float _752 = _660 * ((_170[_651] != 0u) ? 1.0f : 0.0f);
            float _757 = lerp(_747, _742, clamp(_748 / (_748 + (-(_734 - _743))), 0.0f, 1.0f));
            float _763 = clamp(coc_tex.SampleLevel(sampler_nnc, _195 + (_327 * _757), 0.0f).x, _336, _206);
            float _771 = log2(max(0.0f, abs(_734) * 0.125f)) * smoothstep(4.0f, 0.0f, _275 - _273);
            float _774 = WaveReadLaneAt(_771, WaveGetLaneIndex() ^ 1u);
            float _775 = min(_771, _774);
            float _777 = WaveReadLaneAt(_775, WaveGetLaneIndex() ^ 8u);
            if (_734 > _206)
            {
                _646 = _269;
                _650 = _649;
                continue;
            }
            float _783 = _748 * 0.100000001490116119384765625f;
            bool _789;
            if (((_757 / _659) - _763) < _783)
            {
                _789 = (_763 - _734) < _783;
            }
            else
            {
                _789 = false;
            }
            float _801;
            float _802;
            if (_789)
            {
                _801 = _269;
                _802 = _752;
            }
            else
            {
                float _796 = smoothstep(0.0f, 30.0f, _735) * smoothstep(0.20000000298023223876953125f * _206, 0.0f, _734);
                _801 = _269 + (_752 * _796);
                _802 = _752 * (1.0f - _796);
            }
            _646 = _801;
            _650 = _649 + (color_tex.SampleLevel(sampler_nnc, _195 + (_327 * (_789 ? _757 : _747)), min(_775, _777)) * _802);
        }
        _262 = _261 + _649;
    }
    float _816 = _268 * smoothstep(0.00999999977648258209228515625f * _261.w, max(9.9999997473787516355514526367188e-06f, 0.20000000298023223876953125f * _261.w), _268);
    float4 _877;
    [branch]
    if (int(_816 > 0.0f) != 0)
    {
        float _823 = -_206;
        float4 _825;
        _825 = 0.0f.xxxx;
        float4 _826;
        float _829;
        float _828 = _823;
        uint _830 = 0u;
        for (; _830 < 16u; _825 = _826, _828 = _829, _830++)
        {
            float2 _841 = frac(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(_830)) + 0.5f.xx) + _222.xy);
            float _843 = sqrt(_841.x);
            float _845 = _841.y * 6.283185482025146484375f;
            float2 _856 = _195 + (((float2(cos(_845) * _843, sin(_845) * _843) * 1.0f) / float2(color_tex_dims)) * _264);
            float4 _859 = coc_tex.SampleLevel(sampler_nnc, _856, 0.0f);
            float _860 = _859.x;
            if (_860 > (_828 - 10.0f))
            {
                _826 = _825 + color_tex.SampleLevel(sampler_llc, _856, 0.0f);
                _829 = max(_828, _860);
            }
            else
            {
                _826 = _825;
                _829 = _828;
            }
        }
        _877 = _261 + ((_825 / max(1.0f, _825.w).xxxx) * _816);
    }
    else
    {
        _877 = _261;
    }
    img_output[_188] = max(0.0f.xxxx, _877 / _266.xxxx);
}

[numthreads(8, 8, 1)]
void cs_halfres_all(SPIRV_Cross_Input stage_input)
{
    gl_WorkGroupID = stage_input.gl_WorkGroupID;
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_cs_halfres_all_72a2a95d73e37edc) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)

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
};

ByteAddressBuffer tile_offsets_buffer : register(t20, space0);
ByteAddressBuffer tile_coords_buffer : register(t21, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_llc : register(s3, space0);
Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> color_tex : register(t15, space0);
Texture2D<float4> coc_tiles_dilated_tex : register(t16, space0);
Texture2D<float4> coc_tex : register(t18, space0);
RWTexture2D<float4> img_output : register(u19, space0);

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
    uint2 _186 = (gl_GlobalInvocationID.xy & uint2(7u, 7u)) + (uint2(8u, 8u) * tile_coords_buffer.Load<uint2>((tile_offsets_buffer.Load<uint>(20) + gl_WorkGroupID.x) * 8 + 0));
    float2 _193 = (float2(int2(_186)) + 0.5f.xx) / float2(img_output_dims);
    float _204 = min(float(color_tex_dims.y) * 0.0416666679084300994873046875f, coc_tiles_dilated_tex.SampleLevel(sampler_nnc, _193, 0.0f).x);
    float4 _220 = ((blue_noise_texture.Load(int3(((_186 + uint2(13u, 13u)) + uint2(int2(int(frame_constants_frame_index * 59u), int(frame_constants_frame_index * 37u)))) & uint2(255u, 255u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
    float _221 = _220.z;
    float _222 = _220.y;
    float _223 = _204 * 0.125f;
    float _224 = _220.x;
    bool _225 = _221 < 0.5f;
    float _229;
    if (_225)
    {
        _229 = 1.0f - _221;
    }
    else
    {
        _229 = _221;
    }
    float _230 = _225 ? (-1.0f) : 1.0f;
    float4 _232;
    float _235;
    float _237;
    float _239;
    _232 = 0.0f.xxxx;
    _235 = 0.0f;
    _237 = 0.0f;
    _239 = 0.0f;
    float4 _233;
    float _236;
    float _244;
    float _246;
    uint _168[12];
    float _238;
    float _240;
    uint _241 = 0u;
    float _243 = 10000000000.0f;
    float _245 = -10000000000.0f;
    [loop]
    for (; _241 < 4u; _232 = _233, _235 = _236, _237 = _238, _239 = _240, _241++, _243 = _244, _245 = _246)
    {
        float _252 = (float(_241) + _224) * 0.785398185253143310546875f;
        float2 _255 = float2(cos(_252), sin(_252));
        float2 _293;
        if (bokeh_anisotropy > 0.0f)
        {
            float4 _268 = mul(frame_constants_view_constants[0u].clip_to_view, float4((_193 - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f));
            float2 _272 = _268.xy / _268.z.xx;
            _272.y = _272.y * (-1.0f);
            float2 _276 = normalize(_272);
            float2x2 _281 = float2x2(_276, float2(-_276.y, _276.x));
            float _286 = lerp(0.699999988079071044921875f, 1.0f, exp(((-4.0f) * bokeh_anisotropy) * dot(_272, _272)));
            _293 = mul(transpose(_281), mul(_281, _255) * float2(_286, 1.0f / _286));
        }
        else
        {
            _293 = _255;
        }
        float2 _298 = _293 * (_230.xx / float2(color_tex_dims));
        float _307 = -_204;
        float _308 = clamp(coc_tex.SampleLevel(sampler_nnc, _193 + (_298 * (((-8.0f) + _222) * _223)), 0.0f).x, _307, _204);
        float _309 = abs(_308);
        float _311 = _224 * 0.20000000298023223876953125f;
        float _312 = lerp(_235, max(_235, _309), _311);
        float _324 = clamp(coc_tex.SampleLevel(sampler_nnc, _193 + (_298 * (((-7.0f) + _222) * _223)), 0.0f).x, _307, _204);
        float _327 = lerp(_312, max(_312, abs(_324)), _311);
        ring_samples[gl_LocalInvocationIndex][0u] = spvPackHalf2x16(float2(_308, 0.0f)) | (spvPackHalf2x16(float2(_324, 0.0f)) << 16u);
        float _340 = clamp(coc_tex.SampleLevel(sampler_nnc, _193 + (_298 * (((-6.0f) + _222) * _223)), 0.0f).x, _307, _204);
        float _341 = abs(_340);
        float _343 = lerp(_327, max(_327, _341), _311);
        float _355 = clamp(coc_tex.SampleLevel(sampler_nnc, _193 + (_298 * (((-5.0f) + _222) * _223)), 0.0f).x, _307, _204);
        float _358 = lerp(_343, max(_343, abs(_355)), _311);
        ring_samples[gl_LocalInvocationIndex][1u] = spvPackHalf2x16(float2(_340, 0.0f)) | (spvPackHalf2x16(float2(_355, 0.0f)) << 16u);
        float _371 = clamp(coc_tex.SampleLevel(sampler_nnc, _193 + (_298 * (((-4.0f) + _222) * _223)), 0.0f).x, _307, _204);
        float _372 = abs(_371);
        float _374 = lerp(_358, max(_358, _372), _311);
        float _386 = clamp(coc_tex.SampleLevel(sampler_nnc, _193 + (_298 * (((-3.0f) + _222) * _223)), 0.0f).x, _307, _204);
        float _389 = lerp(_374, max(_374, abs(_386)), _311);
        ring_samples[gl_LocalInvocationIndex][2u] = spvPackHalf2x16(float2(_371, 0.0f)) | (spvPackHalf2x16(float2(_386, 0.0f)) << 16u);
        float _402 = clamp(coc_tex.SampleLevel(sampler_nnc, _193 + (_298 * (((-2.0f) + _222) * _223)), 0.0f).x, _307, _204);
        float _403 = abs(_402);
        float _405 = lerp(_389, max(_389, _403), _311);
        float _417 = clamp(coc_tex.SampleLevel(sampler_nnc, _193 + (_298 * (((-1.0f) + _222) * _223)), 0.0f).x, _307, _204);
        float _420 = lerp(_405, max(_405, abs(_417)), _311);
        ring_samples[gl_LocalInvocationIndex][3u] = spvPackHalf2x16(float2(_402, 0.0f)) | (spvPackHalf2x16(float2(_417, 0.0f)) << 16u);
        float _432 = clamp(coc_tex.SampleLevel(sampler_nnc, _193 + (_298 * (_222 * _223)), 0.0f).x, _307, _204);
        float _433 = abs(_432);
        float _435 = lerp(_420, max(_420, _433), _311);
        float _447 = clamp(coc_tex.SampleLevel(sampler_nnc, _193 + (_298 * ((1.0f + _222) * _223)), 0.0f).x, _307, _204);
        float _450 = lerp(_435, max(_435, abs(_447)), _311);
        ring_samples[gl_LocalInvocationIndex][4u] = spvPackHalf2x16(float2(_432, 0.0f)) | (spvPackHalf2x16(float2(_447, 0.0f)) << 16u);
        float _463 = clamp(coc_tex.SampleLevel(sampler_nnc, _193 + (_298 * ((2.0f + _222) * _223)), 0.0f).x, _307, _204);
        float _464 = abs(_463);
        float _466 = lerp(_450, max(_450, _464), _311);
        float _478 = clamp(coc_tex.SampleLevel(sampler_nnc, _193 + (_298 * ((3.0f + _222) * _223)), 0.0f).x, _307, _204);
        float _481 = lerp(_466, max(_466, abs(_478)), _311);
        ring_samples[gl_LocalInvocationIndex][5u] = spvPackHalf2x16(float2(_463, 0.0f)) | (spvPackHalf2x16(float2(_478, 0.0f)) << 16u);
        float _494 = clamp(coc_tex.SampleLevel(sampler_nnc, _193 + (_298 * ((4.0f + _222) * _223)), 0.0f).x, _307, _204);
        float _495 = abs(_494);
        float _497 = lerp(_481, max(_481, _495), _311);
        float _509 = clamp(coc_tex.SampleLevel(sampler_nnc, _193 + (_298 * ((5.0f + _222) * _223)), 0.0f).x, _307, _204);
        float _512 = lerp(_497, max(_497, abs(_509)), _311);
        ring_samples[gl_LocalInvocationIndex][6u] = spvPackHalf2x16(float2(_494, 0.0f)) | (spvPackHalf2x16(float2(_509, 0.0f)) << 16u);
        float _525 = clamp(coc_tex.SampleLevel(sampler_nnc, _193 + (_298 * ((6.0f + _222) * _223)), 0.0f).x, _307, _204);
        float _526 = abs(_525);
        float _528 = lerp(_512, max(_512, _526), _311);
        float _540 = clamp(coc_tex.SampleLevel(sampler_nnc, _193 + (_298 * ((7.0f + _222) * _223)), 0.0f).x, _307, _204);
        float _543 = lerp(_528, max(_528, abs(_540)), _311);
        ring_samples[gl_LocalInvocationIndex][7u] = spvPackHalf2x16(float2(_525, 0.0f)) | (spvPackHalf2x16(float2(_540, 0.0f)) << 16u);
        float _556 = clamp(coc_tex.SampleLevel(sampler_nnc, _193 + (_298 * ((8.0f + _222) * _223)), 0.0f).x, _307, _204);
        float _557 = abs(_556);
        _236 = lerp(_543, max(_543, _557), _311);
        _244 = min(min(min(min(min(min(min(min(min(_243, _309), _341), _372), _403), _433), _464), _495), _526), _557);
        _246 = max(max(max(max(max(max(max(max(max(_245, _309), _341), _372), _403), _433), _464), _495), _526), _557);
        ring_samples[gl_LocalInvocationIndex][8u] = spvPackHalf2x16(float2(_556, 0.0f));
        for (uint _563 = 0u; _563 < 12u; )
        {
            _168[_563] = 0u;
            _563++;
            continue;
        }
        for (uint _570 = 0u; _570 < 17u; _570++)
        {
            uint _578 = _570 & 1u;
            uint _579 = _570 >> 1u;
            float2 _169 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_579] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_579] >> 16u) & 65535u).x);
            float _594 = (float(int(_570 - 8u)) + _222) * _223;
            for (int _596 = 0; _596 < 12; _596++)
            {
                if ((_594 / (((float(_596) + _229) * 0.16666667163372039794921875f) - 1.0f)) >= _169[_578])
                {
                    _168[_596] |= (1u << (_570 & 31u));
                }
            }
        }
        float4 _619;
        _240 = _239;
        _238 = _237;
        _619 = 0.0f.xxxx;
        float _618;
        float _616;
        float4 _620;
        for (int _621 = 0; _621 < 12; _240 = _616, _238 = _618, _619 = _620, _621++)
        {
            float _629 = ((float(_621) + _229) * 0.16666667163372039794921875f) - 1.0f;
            float _630 = abs(_629);
            _618 = _238 + _630;
            if (_168[_621] == 0u)
            {
                _616 = _240;
                _620 = _619;
                continue;
            }
            bool _636 = _629 >= 0.0f;
            float _704;
            float _705;
            uint _706;
            if (_636)
            {
                uint _671 = firstbitlow(_168[_621]);
                uint _675 = uint(max(0, (int(_671) - 1)));
                uint _677 = _671 >> 1u;
                float2 _170 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_677] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_677] >> 16u) & 65535u).x);
                uint _691 = _675 >> 1u;
                float2 _171 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_691] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_691] >> 16u) & 65535u).x);
                _704 = _170[_671 & 1u];
                _705 = _171[_675 & 1u];
                _706 = _671;
            }
            else
            {
                uint _640 = firstbithigh(_168[_621]);
                uint _642 = min(16u, (_640 + 1u));
                uint _644 = _640 >> 1u;
                float2 _172 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_644] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_644] >> 16u) & 65535u).x);
                uint _658 = _642 >> 1u;
                float2 _173 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_658] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_658] >> 16u) & 65535u).x);
                _704 = _172[_640 & 1u];
                _705 = _173[_642 & 1u];
                _706 = _640;
            }
            float _712 = (float(int(_706 - 8u)) + _222) * _223;
            float _713 = _712 / _629;
            float _714 = _223 * (_636 ? 1.0f : (-1.0f));
            float _717 = _712 - _714;
            float _718 = _705 - (_713 - (_714 / _629));
            float _722 = _630 * ((_168[_621] != 0u) ? 1.0f : 0.0f);
            float _727 = lerp(_717, _712, clamp(_718 / (_718 + (-(_704 - _713))), 0.0f, 1.0f));
            float _733 = clamp(coc_tex.SampleLevel(sampler_nnc, _193 + (_298 * _727), 0.0f).x, _307, _204);
            float _741 = log2(max(0.0f, abs(_704) * 0.125f)) * smoothstep(4.0f, 0.0f, _246 - _244);
            float _744 = WaveReadLaneAt(_741, WaveGetLaneIndex() ^ 1u);
            float _745 = min(_741, _744);
            float _747 = WaveReadLaneAt(_745, WaveGetLaneIndex() ^ 8u);
            if (_704 > _204)
            {
                _616 = _240;
                _620 = _619;
                continue;
            }
            float _753 = _718 * 0.100000001490116119384765625f;
            bool _759;
            if (((_727 / _629) - _733) < _753)
            {
                _759 = (_733 - _704) < _753;
            }
            else
            {
                _759 = false;
            }
            float _771;
            float _772;
            if (_759)
            {
                _771 = _240;
                _772 = _722;
            }
            else
            {
                float _766 = smoothstep(0.0f, 30.0f, _705) * smoothstep(0.20000000298023223876953125f * _204, 0.0f, _704);
                _771 = _240 + (_722 * _766);
                _772 = _722 * (1.0f - _766);
            }
            _616 = _771;
            _620 = _619 + (color_tex.SampleLevel(sampler_nnc, _193 + (_298 * (_759 ? _727 : _717)), min(_745, _747)) * _772);
        }
        _233 = _232 + _619;
    }
    float _786 = _239 * smoothstep(0.00999999977648258209228515625f * _232.w, max(9.9999997473787516355514526367188e-06f, 0.20000000298023223876953125f * _232.w), _239);
    float4 _847;
    [branch]
    if (int(_786 > 0.0f) != 0)
    {
        float _793 = -_204;
        float4 _795;
        _795 = 0.0f.xxxx;
        float4 _796;
        float _799;
        float _798 = _793;
        uint _800 = 0u;
        for (; _800 < 16u; _795 = _796, _798 = _799, _800++)
        {
            float2 _811 = frac(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(_800)) + 0.5f.xx) + _220.xy);
            float _813 = sqrt(_811.x);
            float _815 = _811.y * 6.283185482025146484375f;
            float2 _826 = _193 + (((float2(cos(_815) * _813, sin(_815) * _813) * 1.0f) / float2(color_tex_dims)) * _235);
            float4 _829 = coc_tex.SampleLevel(sampler_nnc, _826, 0.0f);
            float _830 = _829.x;
            if (_830 > (_798 - 10.0f))
            {
                _796 = _795 + color_tex.SampleLevel(sampler_llc, _826, 0.0f);
                _799 = max(_798, _830);
            }
            else
            {
                _796 = _795;
                _799 = _798;
            }
        }
        _847 = _232 + ((_795 / max(1.0f, _795.w).xxxx) * _786);
    }
    else
    {
        _847 = _232;
    }
    img_output[_186] = max(0.0f.xxxx, _847 / _237.xxxx);
}

[numthreads(8, 8, 1)]
void cs_halfres_all(SPIRV_Cross_Input stage_input)
{
    gl_WorkGroupID = stage_input.gl_WorkGroupID;
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
