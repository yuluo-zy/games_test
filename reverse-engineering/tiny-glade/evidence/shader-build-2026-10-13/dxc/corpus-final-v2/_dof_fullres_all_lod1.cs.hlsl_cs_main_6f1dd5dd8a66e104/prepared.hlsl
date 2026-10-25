#line 1 "shaders/dof/fullres_all_lod1.cs.hlsl"




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
#line 5 "shaders/dof/fullres_all_lod1.cs.hlsl"

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
#line 6 "shaders/dof/fullres_all_lod1.cs.hlsl"

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
#line 7 "shaders/dof/fullres_all_lod1.cs.hlsl"

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
#line 8 "shaders/dof/fullres_all_lod1.cs.hlsl"

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
#line 9 "shaders/dof/fullres_all_lod1.cs.hlsl"

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
#line 10 "shaders/dof/fullres_all_lod1.cs.hlsl"

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
#line 11 "shaders/dof/fullres_all_lod1.cs.hlsl"

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
#line 12 "shaders/dof/fullres_all_lod1.cs.hlsl"

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
#line 13 "shaders/dof/fullres_all_lod1.cs.hlsl"

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
#line 14 "shaders/dof/fullres_all_lod1.cs.hlsl"

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
#line 15 "shaders/dof/fullres_all_lod1.cs.hlsl"

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
#line 16 "shaders/dof/fullres_all_lod1.cs.hlsl"

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
#line 17 "shaders/dof/fullres_all_lod1.cs.hlsl"

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
#line 18 "shaders/dof/fullres_all_lod1.cs.hlsl"

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
#line 19 "shaders/dof/fullres_all_lod1.cs.hlsl"


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
StructuredBuffer<DofTileOffsets> tile_offsets_buffer : register(t20, space0);
StructuredBuffer<uint2> tile_coords_buffer : register(t21, space0);
Texture2D<float4> color_tex : register(t15, space0);
Texture2D<float4> coc_tiles_dilated_tex : register(t16, space0);
Texture2D<float4> coc_tex : register(t18, space0);
RWTexture2D<half4> img_output : register(u19, space0);

groupshared uint dof_ring_samples[64][3];

uint PackRingPair(float2 v)
{
    return f32tof16(v.x) | (f32tof16(v.y) << 16u);
}

float2 UnpackRingPair(uint v)
{
    return float2(f16tof32(v & 0xFFFFu), f16tof32(v >> 16u));
}

uint2 TileOrigin(uint3 group_id, uint3 dispatch_id, uint offset_index)
{
    return (dispatch_id.xy & uint2(7u, 7u))
        + uint2(8u, 8u) * tile_coords_buffer[tile_offsets_buffer[0].offsets[offset_index] + group_id.x];
}

float2 BokehDir(float2 dir, float2 uv)
{
    if (bokeh_anisotropy > 0.0f)
    {
        float4 v = mul(view_constants[0].clip_to_view, float4((uv - 0.5f) * float2(2.0f, -2.0f), 0.0f, 1.0f));
        float2 p = v.xy / v.z;
        p.y = -p.y;
        float2 n = normalize(p);
        float2x2 rot = float2x2(n, float2(-n.y, n.x));
        float a = lerp(0.699999988079071f, 1.0f, exp((-4.0f * bokeh_anisotropy) * dot(p, p)));
        return mul(transpose(rot), mul(rot, dir) * float2(a, 1.0f / a));
    }
    return dir;
}

void SampleRing(uint thread, float2 uv, float2 dir_px, float ring_offset, float coc_step, float coc_limit,
                float lerp_factor, inout float coc_acc, inout float min_abs_coc, inout float max_abs_coc)
{
    float pending = 0.0f;
    [unroll]
    for (uint k = 0u; k < 5u; ++k)
    {
        float coc = clamp(coc_tex.SampleLevel(sampler_nnc,
            uv + dir_px * ((float(int(k - 2u)) + ring_offset) * coc_step), 0.0f).x, -coc_limit, coc_limit);
        float coc_abs = abs(coc);
        coc_acc = lerp(coc_acc, max(coc_acc, coc_abs), lerp_factor);
        min_abs_coc = min(min_abs_coc, coc_abs);
        max_abs_coc = max(max_abs_coc, coc_abs);
        if ((k & 1u) == 0u)
        {
            pending = coc;
        }
        else
        {
            dof_ring_samples[thread][k >> 1u] =
                PackRingPair(float2(pending, 0.0f)) | (PackRingPair(float2(coc, 0.0f)) << 16u);
        }
    }
    dof_ring_samples[thread][2u] = PackRingPair(float2(pending, 0.0f));
}

[numthreads(8, 8, 1)]
void cs_main(uint3 group_id : SV_GroupID, uint3 dispatch_id : SV_DispatchThreadID, uint local_index : SV_GroupIndex)
{
    uint2 pixel = TileOrigin(group_id, dispatch_id, 2u);
    float2 uv = (float2(int2(pixel)) + 0.5f) / float2(img_output_dims);
    float coc_limit = min(float(color_tex_dims.y) * 0.041666667908430099f,
        coc_tiles_dilated_tex.SampleLevel(sampler_nnc, uv, 0.0f).x);
    float4 noise = ((blue_noise_texture.Load(int3((pixel + uint2(13u, 13u)
        + uint2(int(frame_index * 59u), int(frame_index * 37u))) & uint2(255u, 255u), 0)) * 255.0f)
        * 0.00390625f) + 0.001953125f;
    uint2 jitter_coord = uint2(7u, 7u) + pixel;
    float ring_jitter = (frac(52.982917785644531f * frac((0.067110560834407806f * float(jitter_coord.x))
        + (0.005837149918079376f * float(jitter_coord.y)))) - 0.5f) * 0.5f;
    float ring_phase = 0.5f + ring_jitter;
    float ring_offset = noise.y;
    float coc_step = coc_limit * 0.5f;
    float spoke_seed = 5.588237762451172f * float(frame_index % 64u);
    float spoke_rot = frac(52.982917785644531f * frac((0.067110560834407806f * (float(pixel.x) + spoke_seed))
        + (0.005837149918079376f * (float(pixel.y) + spoke_seed))));
    bool flip = ring_phase < 0.5f;
    float mask_phase = flip ? (0.5f - ring_jitter) : ring_phase;
    float spoke_sign = flip ? -1.0f : 1.0f;

    float4 color_acc = float4(0.0f, 0.0f, 0.0f, 0.0f);
    float coc_acc = 0.0f;
    float coc_weight = 0.0f;
    float weight_acc = 0.0f;
    float min_abs_coc = 1.0e10f;
    float max_abs_coc = -1.0e10f;

    [loop]
    for (uint spoke = 0u; spoke < 2u; ++spoke)
    {
        float angle = (float(spoke) + spoke_rot) * 1.5707963705062866f;
        float2 dir = BokehDir(float2(cos(angle), sin(angle)), uv);
        float2 dir_px = dir * (spoke_sign / float2(color_tex_dims));
        SampleRing(local_index, uv, dir_px, ring_offset, coc_step, coc_limit, noise.x * 0.2f,
            coc_acc, min_abs_coc, max_abs_coc);

        uint mask[2];
        [unroll]
        for (uint mi = 0u; mi < 2u; ++mi)
        {
            mask[mi] = 0u;
        }
        [loop]
        for (uint j = 0u; j < 5u; ++j)
        {
            uint lane_bit = j & 1u;
            uint pair = j >> 1u;
            float2 pair_coc = UnpackRingPair(dof_ring_samples[local_index][pair]);
            float ring_dist = (float(int(j - 2u)) + ring_offset) * coc_step;
            [unroll]
            for (uint b = 0u; b < 2u; ++b)
            {
                if ((ring_dist / ((float(b) + mask_phase) - 1.0f)) >= pair_coc[lane_bit])
                {
                    mask[b] |= (1u << (j & 31u));
                }
            }
        }

        float4 spoke_color = float4(0.0f, 0.0f, 0.0f, 0.0f);
        [loop]
        for (uint bin = 0u; bin < 2u; ++bin)
        {
            float mask_dist = (float(bin) + mask_phase) - 1.0f;
            float abs_dist = abs(mask_dist);
            coc_weight += abs_dist;
            if (mask[bin] == 0u)
            {
                continue;
            }
            bool positive = mask_dist >= 0.0f;
            float coc_a;
            float coc_b;
            uint crossing;
            if (positive)
            {
                crossing = firstbitlow(mask[bin]);
                uint prev = uint(max(0, (int(crossing) - 1)));
                float2 c1 = UnpackRingPair(dof_ring_samples[local_index][crossing >> 1u]);
                float2 c2 = UnpackRingPair(dof_ring_samples[local_index][prev >> 1u]);
                coc_a = c1[crossing & 1u];
                coc_b = c2[prev & 1u];
            }
            else
            {
                crossing = firstbithigh(mask[bin]);
                uint next = min(4u, crossing + 1u);
                float2 c1 = UnpackRingPair(dof_ring_samples[local_index][crossing >> 1u]);
                float2 c2 = UnpackRingPair(dof_ring_samples[local_index][next >> 1u]);
                coc_a = c1[crossing & 1u];
                coc_b = c2[next & 1u];
            }
            float ring_dist = (float(int(crossing - 2u)) + ring_offset) * coc_step;
            float dist_ratio = ring_dist / mask_dist;
            float step = coc_step * (positive ? 1.0f : -1.0f);
            float candidate = ring_dist - step;
            float cross_weight = coc_b - (dist_ratio - (step / mask_dist));
            float sample_weight = abs_dist * float(mask[bin] != 0u);
            float coc_pos = lerp(candidate, ring_dist,
                clamp(cross_weight / (cross_weight - (coc_a - dist_ratio)), 0.0f, 1.0f));
            float coc_check = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (dir_px * coc_pos), 0.0f).x,
                -coc_limit, coc_limit);
            float mip = log2(max(0.0f, abs(coc_a) * 0.125f)) * smoothstep(4.0f, 0.0f, max_abs_coc - min_abs_coc);
            float mip_x = WaveReadLaneAt(mip, WaveGetLaneIndex() ^ 1u);
            mip = min(mip, mip_x);
            float mip_y = WaveReadLaneAt(mip, WaveGetLaneIndex() ^ 8u);
            if (coc_a > coc_limit)
            {
                continue;
            }
            float epsilon = cross_weight * 0.1f;
            bool near = false;
            if (((coc_pos / mask_dist) - coc_check) < epsilon)
            {
                near = (coc_check - coc_a) < epsilon;
            }
            float add_weight;
            float add_color_weight;
            
            if (near)
            {
                add_weight = weight_acc;
                add_color_weight = sample_weight;
            }
            else
            {
                float s = smoothstep(0.0f, 30.0f, coc_b) * smoothstep(0.2f * coc_limit, 0.0f, coc_a);
                add_weight = weight_acc + (sample_weight * s);
                add_color_weight = sample_weight * (1.0f - s);
            }
            weight_acc = add_weight;
            spoke_color += color_tex.SampleLevel(sampler_nnc,
                uv + (dir_px * (near ? coc_pos : candidate)), min(mip, mip_y)) * add_color_weight;
        }
        color_acc += spoke_color;
    }

    float scatter = weight_acc * smoothstep(0.01f * color_acc.w,
        max(9.9999997473787516e-06f, 0.2f * color_acc.w), weight_acc);
    if (scatter > 0.0f)
    {
        float min_coc = -coc_limit;
        float4 disk_acc = float4(0.0f, 0.0f, 0.0f, 0.0f);
        [loop]
        for (uint d = 0u; d < 16u; ++d)
        {
            float2 disc = frac(frac((float2(0.7548776865005493f, 0.5698403120040894f) * float(d)) + 0.5f) + noise.xy);
            float r = sqrt(disc.x);
            float theta = disc.y * 6.2831854820251465f;
            float2 sample_uv = uv + (((float2(cos(theta) * r, sin(theta) * r) * 1.0f) / float2(color_tex_dims)) * coc_acc);
            float4 c = coc_tex.SampleLevel(sampler_nnc, sample_uv, 0.0f);
            if (c.x > (min_coc - 10.0f))
            {
                disk_acc += color_tex.SampleLevel(sampler_llc, sample_uv, 0.0f);
                min_coc = max(min_coc, c.x);
            }
        }
        color_acc += (disk_acc / max(1.0f, disk_acc.w)) * scatter;
    }

    img_output[pixel] = max(float4(0.0f, 0.0f, 0.0f, 0.0f), color_acc / coc_weight);
}
