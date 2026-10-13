#define TGV_0 1
#line 1 "shaders/dof/autofocus.cs.hlsl"


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
#line 9 "shaders/dof/autofocus.cs.hlsl"

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
#line 10 "shaders/dof/autofocus.cs.hlsl"

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
#line 11 "shaders/dof/autofocus.cs.hlsl"

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
#line 12 "shaders/dof/autofocus.cs.hlsl"

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
#line 13 "shaders/dof/autofocus.cs.hlsl"

#line 1 "shaders/inc/utils.hlsl"





#ifndef TINYGLADE_INC_UTILS_HLSL
#define TINYGLADE_INC_UTILS_HLSL

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
#line 9 "shaders/inc/utils.hlsl"







#ifndef TINYGLADE_AFFINE3PACKED
#define TINYGLADE_AFFINE3PACKED
struct Affine3Packed
{
    float data[12];
};
#endif



float4x4 affine_basis(float3 forward)
{
    float3 f = normalize(forward);
    float3 up = float3(0.0f, 1.0f, 0.0f);
    float3 right = cross(f, up);
    float4x4 m = float4x4(
        float4(f, 0.0f),
        float4(up, 0.0f),
        float4(right, 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f));
    return transpose(m);
}


float4x4 affine_translation(float3 pos)
{
    return float4x4(
        float4(1.0f, 0.0f, 0.0f, pos.x),
        float4(0.0f, 1.0f, 0.0f, pos.y),
        float4(0.0f, 0.0f, 1.0f, pos.z),
        float4(0.0f, 0.0f, 0.0f, 1.0f));
}


float4x4 affine_scale(float3 s)
{
    return float4x4(
        float4(s.x, 0.0f, 0.0f, 0.0f),
        float4(0.0f, s.y, 0.0f, 0.0f),
        float4(0.0f, 0.0f, s.z, 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f));
}



Affine3Packed pack_affine(float4x4 m)
{
    Affine3Packed o;
    o.data[0] = m[0].x; o.data[1] = m[1].x; o.data[2] = m[2].x;
    o.data[3] = m[0].y; o.data[4] = m[1].y; o.data[5] = m[2].y;
    o.data[6] = m[0].z; o.data[7] = m[1].z; o.data[8] = m[2].z;
    o.data[9] = m[0].w; o.data[10] = m[1].w; o.data[11] = m[2].w;
    return o;
}


float4x4 unpack_affine(Affine3Packed a)
{
    float4x4 m;
    m[0] = float4(a.data[0], a.data[3], a.data[6], a.data[9]);
    m[1] = float4(a.data[1], a.data[4], a.data[7], a.data[10]);
    m[2] = float4(a.data[2], a.data[5], a.data[8], a.data[11]);
    m[3] = float4(0.0f, 0.0f, 0.0f, 1.0f);
    return m;
}


Affine3Packed compose_affine_packed(float3 pos, float3 forward, float3 scale)
{
    return pack_affine(mul(affine_translation(pos), mul(affine_basis(forward), affine_scale(scale))));
}

#endif 
#line 14 "shaders/dof/autofocus.cs.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 15 "shaders/dof/autofocus.cs.hlsl"

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
#line 16 "shaders/dof/autofocus.cs.hlsl"

#line 1 "shaders/inc/bitonic_sort.hlsl"


#ifndef TINYGLADE_INC_BITONIC_SORT_HLSL
#define TINYGLADE_INC_BITONIC_SORT_HLSL



uint2 bitonic_sort_pair(uint lane, uint j, uint k)
{
    uint i = lane & (j - 1u);
    uint m = 2u * (lane - i);
    uint a = m + i;
    uint b = (j == (k >> 1u)) ? (m + ((2u * j) - 1u) - i) : (m + j + i);
    return uint2(a, b);
}




#define BITONIC_SORT_UINT(name, lane, size) \
    { \
        uint _b_sort_k_max = 2u << (firstbithigh((size) - 1u) & 31u); \
        for (uint _b_sort_k = 2u; _b_sort_k <= _b_sort_k_max; _b_sort_k *= 2u) { \
            uint _b_sort_j_start = _b_sort_k >> 1u; \
            for (uint _b_sort_j = _b_sort_j_start; _b_sort_j > 0u; _b_sort_j >>= 1u) { \
                uint2 _b_sort_ab = bitonic_sort_pair((lane), _b_sort_j, _b_sort_k); \
                if (_b_sort_ab.x < (size) && _b_sort_ab.y < (size)) { \
                    if (name[_b_sort_ab.x] > name[_b_sort_ab.y]) { \
                        uint _b_sort_tmp = name[_b_sort_ab.x]; \
                        name[_b_sort_ab.x] = name[_b_sort_ab.y]; \
                        name[_b_sort_ab.y] = _b_sort_tmp; \
                    } \
                } \
                GroupMemoryBarrierWithGroupSync(); \
            } \
        } \
    }

#endif 
#line 17 "shaders/dof/autofocus.cs.hlsl"

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
#line 18 "shaders/dof/autofocus.cs.hlsl"


cbuffer $Globals : register(b13, space0)
{
    uint2 depth_tex_dims;
    float2 point_uv;
    uint use_dual_focus;
    uint focus_lock_request;
    uint focus_is_locked;
    float focus_box_height;
    uint use_tilt_shift;
    float aperture_size;
};

RWStructuredBuffer<DofFocus> __alias_autofocus_buf : register(u15, space0);
RWStructuredBuffer<DofFocusReadback> __alias_focus_readback_buf : register(u16, space0);
Texture2D<float4> depth_tex : register(t14, space0);

groupshared uint group_focus0_dists[64];
groupshared uint group_focus1_dists[64];

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_id : SV_DispatchThreadID, uint local_index : SV_GroupIndex)
{
    float2 local_uv = (float2(dispatch_id.xy) * 0.14285714924335479736f) - 0.5f;
    GroupMemoryBarrierWithGroupSync();
    bool not_first_frame = (frame_index != 0u);

    float focus0 = __alias_autofocus_buf[camera_idx].focus0;
    float focus1 = __alias_autofocus_buf[camera_idx].focus1;
    float lock_bias0 = __alias_autofocus_buf[camera_idx].lock_bias0;
    float lock_bias1 = __alias_autofocus_buf[camera_idx].lock_bias1;
    float3 lock_pos_ws = __alias_autofocus_buf[camera_idx].lock_pos_ws;
    float3 interp_lock_pos_ws = __alias_autofocus_buf[camera_idx].interp_lock_pos_ws;
    float interp_lock_bias0 = __alias_autofocus_buf[camera_idx].interp_lock_bias0;
    float interp_lock_bias1 = __alias_autofocus_buf[camera_idx].interp_lock_bias1;
    uint pad2 = __alias_autofocus_buf[camera_idx].pad2;

    float clamped_focus0 = clamp(focus0, 0.0f, 100000.0f);
    float clamped_focus1 = clamp(focus1, 0.0f, 100000.0f);

    float2 focus_box_extent = float2(1.0f / (view_constants[0].view_to_clip[1].y / view_constants[0].view_to_clip[0].x), 1.0f)
        * focus_box_height;
    bool dual_focus = use_dual_focus != 0u;

    float3 focus0_point_ws;
    if (dual_focus)
    {
        float2 focus0_uv = point_uv + ((local_uv * focus_box_extent) * 0.2f);
        float4 w = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view,
            float4((focus0_uv - 0.5f) * float2(2.0f, -2.0f),
                   depth_tex.Load(int3(uint2(int2(focus0_uv * float2(depth_tex_dims))), 0)).x, 1.0f)));
        focus0_point_ws = w.xyz / w.w;
    }
    else
    {
        float2 focus0_uv = point_uv + (local_uv * focus_box_extent);
        float4 w = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view,
            float4((focus0_uv - 0.5f) * float2(2.0f, -2.0f),
                   depth_tex.Load(int3(uint2(int2(focus0_uv * float2(depth_tex_dims))), 0)).x, 1.0f)));
        focus0_point_ws = w.xyz / w.w;
    }

    float3 focus1_point_ws;
    if (dual_focus)
    {
        float2 focus1_uv = local_uv * focus_box_extent;
        float4 w = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view,
            float4(focus1_uv * float2(2.0f, -2.0f),
                   depth_tex.Load(int3(uint2(int2((0.5f + focus1_uv) * float2(depth_tex_dims))), 0)).x, 1.0f)));
        focus1_point_ws = w.xyz / w.w;
    }
    else
    {
        focus1_point_ws = focus0_point_ws;
    }

    bool tilt_shift = use_tilt_shift != 0u;
    float3 p0 = focus0_point_ws;
    p0.y *= (tilt_shift ? 0.0f : 1.0f);
    float4 cam_h = mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_pos = cam_h.xyz / cam_h.w;
    float3 cam_dir = normalize(mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, -1.0f, 0.0f)).xyz);
    float3 p1 = focus1_point_ws;
    p1.y *= (tilt_shift ? 0.0f : 1.0f);

    group_focus0_dists[local_index] = asuint(dot(p0 - cam_pos, cam_dir));
    group_focus1_dists[local_index] = asuint(dot(p1 - cam_pos, cam_dir));
    GroupMemoryBarrierWithGroupSync();

    BITONIC_SORT_UINT(group_focus0_dists, local_index, 64);
    BITONIC_SORT_UINT(group_focus1_dists, local_index, 64);

    if (local_index == 0u)
    {
        float focus0_min = asfloat(group_focus0_dists[0]);
        float focus0_max = asfloat(group_focus0_dists[63]);
        float focus0_median = asfloat(group_focus0_dists[32]);
        float focus1_median = asfloat(group_focus1_dists[32]);

        float near_focus;
        float far_focus;
        if (tilt_shift)
        {
            bool not_dual = !dual_focus;
            near_focus = not_dual ? focus0_max : focus1_median;
            far_focus = not_dual ? focus0_min : focus0_median;
        }
        else
        {
            near_focus = focus1_median;
            far_focus = focus0_median;
        }

        float target_focus0;
        float target_focus1;
        float target_bias0;
        float target_bias1;
        float3 target_lock_pos;
        if (focus_lock_request == camera_idx)
        {
            float4 w = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view,
                float4((point_uv - 0.5f) * float2(2.0f, -2.0f),
                       depth_tex.Load(int3(uint2(int2(point_uv * float2(depth_tex_dims))), 0)).x, 1.0f)));
            float3 lock_point = w.xyz / w.w;
            float3 lock_point_f = lock_point;
            lock_point_f.y *= (tilt_shift ? 0.0f : 1.0f);
            float lock_dist = dot(lock_point_f - cam_pos, cam_dir);
            target_focus0 = max(clamped_focus0, clamped_focus1) - lock_dist;
            target_focus1 = max(far_focus, near_focus) - lock_dist;
            target_bias0 = min(clamped_focus0, clamped_focus1) - lock_dist;
            target_bias1 = min(far_focus, near_focus) - lock_dist;
            target_lock_pos = lock_point;
        }
        else
        {
            target_focus0 = interp_lock_bias1;
            target_focus1 = lock_bias1;
            target_bias0 = interp_lock_bias0;
            target_bias1 = lock_bias0;
            target_lock_pos = lock_pos_ws;
        }

        float out_focus0;
        float out_focus1;
        float3 out_lock_pos;
        float out_bias0;
        float out_bias1;
        if (focus_is_locked != 0u)
        {
            float lerp_fast = 1.0f - exp(delta_time_seconds * -8.0f);
            float3 lerped_lock_pos = not_first_frame
                ? lerp(interp_lock_pos_ws, target_lock_pos, lerp_fast) : target_lock_pos;
            float lerped_bias0 = not_first_frame
                ? lerp(target_bias0, target_bias1, lerp_fast) : target_bias1;
            float lerped_focus0 = not_first_frame
                ? lerp(target_focus0, target_focus1, lerp_fast) : target_focus1;

            float3 lock_point_f = target_lock_pos;
            lock_point_f.y *= (tilt_shift ? 0.0f : 1.0f);
            float lock_dist = dot(lock_point_f - cam_pos, cam_dir);
            out_focus0 = lock_dist + lerped_bias0;
            out_focus1 = lock_dist + lerped_focus0;
            out_lock_pos = lerped_lock_pos;
            out_bias0 = lerped_bias0;
            out_bias1 = lerped_focus0;
        }
        else
        {
            if (not_first_frame)
            {
                float lerp_fast = 1.0f - exp(delta_time_seconds * -8.0f);
                float lerp_slow = 1.0f - exp(delta_time_seconds * -4.0f);
                float inv = 1.0f / max(9.9999997473787516e-06f, clamped_focus0);
                float inv_clamped = 1.0f / max(9.9999997473787516e-06f,
                    clamp(clamped_focus0, focus0_min, focus0_max));
                float inv_lerped = 1.0f / max(9.9999997473787516e-06f, lerp(inv, inv_clamped, lerp_fast));
                float inv_target = 1.0f / max(9.9999997473787516e-06f, far_focus);
                out_focus0 = 1.0f / max(9.9999997473787516e-06f, lerp(inv_lerped, inv_target, lerp_slow));

                float inv1 = 1.0f / max(9.9999997473787516e-06f, clamped_focus1);
                float inv1_clamped = 1.0f / max(9.9999997473787516e-06f,
                    clamp(clamped_focus1, asfloat(group_focus1_dists[0]), asfloat(group_focus1_dists[63])));
                float inv1_lerped = 1.0f / max(9.9999997473787516e-06f, lerp(inv1, inv1_clamped, lerp_fast));
                float inv1_target = 1.0f / max(9.9999997473787516e-06f, near_focus);
                out_focus1 = 1.0f / max(9.9999997473787516e-06f, lerp(inv1_lerped, inv1_target, lerp_slow));
            }
            else
            {
                out_focus0 = far_focus;
                out_focus1 = near_focus;
            }
            out_lock_pos = interp_lock_pos_ws;
            out_bias0 = target_bias0;
            out_bias1 = target_focus0;
        }

        DofFocus result_focus;
        result_focus.focus0 = out_focus0;
        result_focus.focus1 = out_focus1;
        result_focus.lock_bias0 = target_bias1;
        result_focus.lock_bias1 = target_focus1;
        result_focus.lock_pos_ws = target_lock_pos;
        result_focus.is_locked = focus_is_locked;
        result_focus.interp_lock_pos_ws = out_lock_pos;
        result_focus.use_tilt_shift = use_tilt_shift;
        result_focus.interp_lock_bias0 = out_bias0;
        result_focus.interp_lock_bias1 = out_bias1;
        result_focus.focus_scale = aperture_size;
        result_focus.pad2 = pad2;
        __alias_autofocus_buf[camera_idx] = result_focus;

        DofFocusReadback readback;
        readback.lock_pos_ws = target_lock_pos;
        readback.lock_reponse = focus_lock_request;
        __alias_focus_readback_buf[0] = readback;
    }
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

struct DofFocus
{
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

struct DofFocusReadback
{
    float3 lock_pos_ws;
    uint lock_reponse;
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
    float2 _Globals_point_uv : packoffset(c0.z);
    uint _Globals_use_dual_focus : packoffset(c1);
    uint _Globals_focus_lock_request : packoffset(c1.y);
    uint _Globals_focus_is_locked : packoffset(c1.z);
    float _Globals_focus_box_height : packoffset(c1.w);
    uint _Globals_use_tilt_shift : packoffset(c2);
    float _Globals_aperture_size : packoffset(c2.y);
};

RWByteAddressBuffer __alias_autofocus_buf : register(u15, space0);
RWByteAddressBuffer __alias_focus_readback_buf : register(u16, space0);
Texture2D<float4> depth_tex : register(t14, space0);

static uint3 gl_GlobalInvocationID;
static uint gl_LocalInvocationIndex;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
    uint gl_LocalInvocationIndex : SV_GroupIndex;
};

groupshared uint group_focus0_dists[64];
groupshared uint group_focus1_dists[64];

void comp_main()
{
    float2 _119 = (float2(gl_GlobalInvocationID.xy) * 0.14285714924335479736328125f.xx) - 0.5f.xx;
    GroupMemoryBarrierWithGroupSync();
    bool _123 = !(0u == frame_constants_frame_index);
    float _128 = __alias_autofocus_buf.Load<float>(frame_constants_camera_idx * 64 + 0);
    float _130 = __alias_autofocus_buf.Load<float>(frame_constants_camera_idx * 64 + 4);
    float _132 = __alias_autofocus_buf.Load<float>(frame_constants_camera_idx * 64 + 8);
    float _134 = __alias_autofocus_buf.Load<float>(frame_constants_camera_idx * 64 + 12);
    float3 _136 = __alias_autofocus_buf.Load<float3>(frame_constants_camera_idx * 64 + 16);
    float3 _138 = __alias_autofocus_buf.Load<float3>(frame_constants_camera_idx * 64 + 32);
    float _140 = __alias_autofocus_buf.Load<float>(frame_constants_camera_idx * 64 + 48);
    float _142 = __alias_autofocus_buf.Load<float>(frame_constants_camera_idx * 64 + 52);
    uint _144 = __alias_autofocus_buf.Load<uint>(frame_constants_camera_idx * 64 + 60);
    float _145 = clamp(_128, 0.0f, 100000.0f);
    float _146 = clamp(_130, 0.0f, 100000.0f);
    float2 _156 = float2(1.0f / (frame_constants_view_constants[0u].view_to_clip[1].y / frame_constants_view_constants[0u].view_to_clip[0].x), 1.0f) * _Globals_focus_box_height;
    bool _159 = _Globals_use_dual_focus != 0u;
    float3 _220;
    if (_159)
    {
        float2 _167 = _Globals_point_uv + ((_119 * _156) * 0.20000000298023223876953125f);
        float4 _187 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4((_167 - 0.5f.xx) * float2(2.0f, -2.0f), depth_tex.Load(int3(uint2(int2(_167 * float2(_Globals_depth_tex_dims))), 0u)).x, 1.0f)));
        _220 = _187.xyz / _187.w.xxx;
    }
    else
    {
        float2 _195 = _Globals_point_uv + (_119 * _156);
        float4 _215 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4((_195 - 0.5f.xx) * float2(2.0f, -2.0f), depth_tex.Load(int3(uint2(int2(_195 * float2(_Globals_depth_tex_dims))), 0u)).x, 1.0f)));
        _220 = _215.xyz / _215.w.xxx;
    }
    float3 _249;
    if (_159)
    {
        float2 _224 = _119 * _156;
        float4 _244 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(_224 * float2(2.0f, -2.0f), depth_tex.Load(int3(uint2(int2((0.5f.xx + _224) * float2(_Globals_depth_tex_dims))), 0u)).x, 1.0f)));
        _249 = _244.xyz / _244.w.xxx;
    }
    else
    {
        _249 = _220;
    }
    bool _252 = _Globals_use_tilt_shift != 0u;
    float _254 = float(_252 ? 0 : 1);
    float3 _257 = _220;
    _257.y = _220.y * _254;
    float4 _260 = mul(frame_constants_view_constants[0u].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 _264 = _260.xyz / _260.w.xxx;
    float3 _268 = normalize(mul(frame_constants_view_constants[0u].view_to_world, float4(0.0f, 0.0f, -1.0f, 0.0f)).xyz);
    float3 _272 = _249;
    _272.y = _249.y * _254;
    group_focus0_dists[gl_LocalInvocationIndex] = asuint(dot(_257 - _264, _268));
    group_focus1_dists[gl_LocalInvocationIndex] = asuint(dot(_272 - _264, _268));
    GroupMemoryBarrierWithGroupSync();
    uint _281 = 2u << (firstbithigh(63u) & 31u);
    for (uint _283 = 2u; _283 <= _281; _283 *= 2u)
    {
        uint _289 = _283 >> 1u;
        for (uint _291 = _289; _291 > 0u; _291 = _291 >> 1u)
        {
            uint _298 = gl_LocalInvocationIndex & (_291 - 1u);
            uint _300 = 2u * (gl_LocalInvocationIndex - _298);
            uint _301 = _300 + _298;
            uint _312;
            if (_291 == _289)
            {
                _312 = (_300 + ((2u * _291) - 1u)) - _298;
            }
            else
            {
                _312 = (_300 + _291) + _298;
            }
            bool _317;
            if (_312 < 64u)
            {
                _317 = _301 < 64u;
            }
            else
            {
                _317 = false;
            }
            if (_317)
            {
                if (group_focus0_dists[_301] > group_focus0_dists[_312])
                {
                    uint _327 = group_focus0_dists[_301];
                    group_focus0_dists[_301] = group_focus0_dists[_312];
                    group_focus0_dists[_312] = _327;
                }
            }
            GroupMemoryBarrierWithGroupSync();
        }
    }
    for (uint _330 = 2u; _330 <= _281; _330 *= 2u)
    {
        uint _336 = _330 >> 1u;
        for (uint _338 = _336; _338 > 0u; _338 = _338 >> 1u)
        {
            uint _345 = gl_LocalInvocationIndex & (_338 - 1u);
            uint _347 = 2u * (gl_LocalInvocationIndex - _345);
            uint _348 = _347 + _345;
            uint _359;
            if (_338 == _336)
            {
                _359 = (_347 + ((2u * _338) - 1u)) - _345;
            }
            else
            {
                _359 = (_347 + _338) + _345;
            }
            bool _364;
            if (_359 < 64u)
            {
                _364 = _348 < 64u;
            }
            else
            {
                _364 = false;
            }
            if (_364)
            {
                if (group_focus1_dists[_348] > group_focus1_dists[_359])
                {
                    uint _374 = group_focus1_dists[_348];
                    group_focus1_dists[_348] = group_focus1_dists[_359];
                    group_focus1_dists[_359] = _374;
                }
            }
            GroupMemoryBarrierWithGroupSync();
        }
    }
    if (0u == gl_LocalInvocationIndex)
    {
        float _381 = asfloat(group_focus0_dists[0]);
        float _384 = asfloat(group_focus0_dists[63u]);
        float _393 = asfloat(group_focus0_dists[32u]);
        float _396 = asfloat(group_focus1_dists[32u]);
        float _402;
        float _403;
        if (_252)
        {
            bool _399 = !_159;
            _402 = _399 ? _384 : _396;
            _403 = _399 ? _381 : _393;
        }
        else
        {
            _402 = _396;
            _403 = _393;
        }
        float _450;
        float _451;
        float _452;
        float _453;
        float3 _454;
        if (_Globals_focus_lock_request == frame_constants_camera_idx)
        {
            float4 _432 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4((_Globals_point_uv - 0.5f.xx) * float2(2.0f, -2.0f), depth_tex.Load(int3(uint2(int2(_Globals_point_uv * float2(_Globals_depth_tex_dims))), 0u)).x, 1.0f)));
            float3 _436 = _432.xyz / _432.w.xxx;
            float3 _439 = _436;
            _439.y = _436.y * _254;
            float _441 = dot(_439 - _264, _268);
            _450 = max(_145, _146) - _441;
            _451 = max(_403, _402) - _441;
            _452 = min(_145, _146) - _441;
            _453 = min(_403, _402) - _441;
            _454 = _436;
        }
        else
        {
            _450 = _142;
            _451 = _134;
            _452 = _140;
            _453 = _132;
            _454 = _136;
        }
        float _539;
        float _540;
        float3 _541;
        float _542;
        float _543;
        if (_Globals_focus_is_locked != 0u)
        {
            float3 _469;
            if (_123)
            {
                _469 = lerp(_138, _454, (1.0f - exp(frame_constants_delta_time_seconds * (-8.0f))).xxx);
            }
            else
            {
                _469 = _454;
            }
            float _479;
            if (_123)
            {
                _479 = lerp(_452, _453, 1.0f - exp(frame_constants_delta_time_seconds * (-8.0f)));
            }
            else
            {
                _479 = _453;
            }
            float _489;
            if (_123)
            {
                _489 = lerp(_450, _451, 1.0f - exp(frame_constants_delta_time_seconds * (-8.0f)));
            }
            else
            {
                _489 = _451;
            }
            float3 _492 = _454;
            _492.y = _454.y * _254;
            float _494 = dot(_492 - _264, _268);
            _539 = _494 + _479;
            _540 = _494 + _489;
            _541 = _469;
            _542 = _479;
            _543 = _489;
        }
        else
        {
            float _537;
            float _538;
            if (_123)
            {
                float _505 = 1.0f - exp(frame_constants_delta_time_seconds * (-8.0f));
                float _522 = 1.0f - exp(frame_constants_delta_time_seconds * (-4.0f));
                _537 = 1.0f / max(9.9999997473787516355514526367188e-06f, lerp(1.0f / max(9.9999997473787516355514526367188e-06f, 1.0f / max(9.9999997473787516355514526367188e-06f, lerp(1.0f / max(9.9999997473787516355514526367188e-06f, _145), 1.0f / max(9.9999997473787516355514526367188e-06f, clamp(_145, _381, _384)), _505))), 1.0f / max(9.9999997473787516355514526367188e-06f, _403), _522));
                _538 = 1.0f / max(9.9999997473787516355514526367188e-06f, lerp(1.0f / max(9.9999997473787516355514526367188e-06f, 1.0f / max(9.9999997473787516355514526367188e-06f, lerp(1.0f / max(9.9999997473787516355514526367188e-06f, _146), 1.0f / max(9.9999997473787516355514526367188e-06f, clamp(_146, asfloat(group_focus1_dists[0]), asfloat(group_focus1_dists[63u]))), _505))), 1.0f / max(9.9999997473787516355514526367188e-06f, _402), _522));
            }
            else
            {
                _537 = _403;
                _538 = _402;
            }
            _539 = _537;
            _540 = _538;
            _541 = _138;
            _542 = _452;
            _543 = _450;
        }
        DofFocus _544 = { _539, _540, _453, _451, _454, _Globals_focus_is_locked, _541, _Globals_use_tilt_shift, _542, _543, _Globals_aperture_size, _144 };
        __alias_autofocus_buf.Store<float>(frame_constants_camera_idx * 64 + 0, _544.focus0);
        __alias_autofocus_buf.Store<float>(frame_constants_camera_idx * 64 + 4, _544.focus1);
        __alias_autofocus_buf.Store<float>(frame_constants_camera_idx * 64 + 8, _544.lock_bias0);
        __alias_autofocus_buf.Store<float>(frame_constants_camera_idx * 64 + 12, _544.lock_bias1);
        __alias_autofocus_buf.Store<float3>(frame_constants_camera_idx * 64 + 16, _544.lock_pos_ws);
        __alias_autofocus_buf.Store<uint>(frame_constants_camera_idx * 64 + 28, _544.is_locked);
        __alias_autofocus_buf.Store<float3>(frame_constants_camera_idx * 64 + 32, _544.interp_lock_pos_ws);
        __alias_autofocus_buf.Store<uint>(frame_constants_camera_idx * 64 + 44, _544.use_tilt_shift);
        __alias_autofocus_buf.Store<float>(frame_constants_camera_idx * 64 + 48, _544.interp_lock_bias0);
        __alias_autofocus_buf.Store<float>(frame_constants_camera_idx * 64 + 52, _544.interp_lock_bias1);
        __alias_autofocus_buf.Store<float>(frame_constants_camera_idx * 64 + 56, _544.focus_scale);
        __alias_autofocus_buf.Store<uint>(frame_constants_camera_idx * 64 + 60, _544.pad2);
        DofFocusReadback _546 = { _454, _Globals_focus_lock_request };
        __alias_focus_readback_buf.Store<float3>(0, _546.lock_pos_ws);
        __alias_focus_readback_buf.Store<uint>(12, _546.lock_reponse);
    }
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
