#line 1 "shaders/read_mouse_depth_value.cs.hlsl"












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
#line 13 "shaders/read_mouse_depth_value.cs.hlsl"

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
#line 14 "shaders/read_mouse_depth_value.cs.hlsl"

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
#line 15 "shaders/read_mouse_depth_value.cs.hlsl"

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
#line 16 "shaders/read_mouse_depth_value.cs.hlsl"


cbuffer $Globals : register(b13, space0)
{
    uint2 depth_map_dims;
    float2 mouse_position_screenspace;
    float2 window_size;
    float filter_box_y_frac;
};

Texture2D<float4> depth_map : register(t14, space0);
RWByteAddressBuffer out_buffer : register(u15, space0);

groupshared float2 group_depths[256];

[numthreads(16, 16, 1)]
void cs_main(uint3 groupThreadID : SV_GroupThreadID, uint groupIndex : SV_GroupIndex)
{
    float2 mouse_uv = mouse_position_screenspace / window_size;
    float2 dims = float2(depth_map_dims);
    uint2 max_px = depth_map_dims - uint2(1u, 1u);

    uint2 mouse_px = uint2(int2(clamp(uint2(int2(mouse_uv * dims)), uint2(0u, 0u), max_px)));
    float mouse_depth = depth_map.Load(int3(mouse_px, 0)).x;
    float4 mouse_world = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view, float4((((float2(int2(clamp(mouse_px, uint2(0u, 0u), max_px))) + 0.5f) / dims) - 0.5f) * float2(2.0f, -2.0f), mouse_depth, 1.0f)));

    uint2 center_px = uint2(int2(clamp(uint2(int2(0.5f.xx * dims)), uint2(0u, 0u), max_px)));
    float center_depth = depth_map.Load(int3(center_px, 0)).x;
    float4 center_world = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view, float4((((float2(int2(clamp(center_px, uint2(0u, 0u), max_px))) + 0.5f) / dims) - 0.5f) * float2(2.0f, -2.0f), center_depth, 1.0f)));

    if (groupIndex == 0u)
    {
        out_buffer.Store<float4>(0, float4(mouse_world.xyz / mouse_world.w, mouse_depth));
        out_buffer.Store<float4>(16, float4(center_world.xyz / center_world.w, center_depth));
    }

    float2 sample_offset = ((float(depth_map_dims.y) * filter_box_y_frac).xx / dims) * (((float2(groupThreadID.xy) + 0.5f.xx) * 0.0625f.xx) - 0.5f.xx);
    group_depths[groupIndex] = float2(depth_map.Load(int3(uint2(int2(clamp(uint2(int2((mouse_uv + sample_offset) * dims)), uint2(0u, 0u), max_px))), 0)).x, depth_map.Load(int3(uint2(int2(clamp(uint2(int2((0.5f + sample_offset) * dims)), uint2(0u, 0u), max_px))), 0)).x);

    GroupMemoryBarrierWithGroupSync();

    uint sort_size = 2u << (firstbithigh(255u) & 31u);
    for (uint stride = 2u; stride <= sort_size; stride *= 2u)
    {
        uint half = stride >> 1u;
        for (uint step = half; step > 0u; step = step >> 1u)
        {
            uint lane = groupIndex & (step - 1u);
            uint a = 2u * (groupIndex - lane);
            uint b = a + lane;
            uint c;
            if (step == half)
            {
                c = (a + ((2u * step) - 1u)) - lane;
            }
            else
            {
                c = (a + step) + lane;
            }
            if (c < 256u && b < 256u)
            {
                float2 va = group_depths[b];
                float2 vb = group_depths[c];
                bool2 cmp = bool2(va.x <= vb.x, va.y <= vb.y);
                group_depths[b] = float2(cmp.x ? va.x : vb.x, cmp.y ? va.y : vb.y);
                group_depths[c] = float2(cmp.x ? vb.x : va.x, cmp.y ? vb.y : va.y);
            }
            GroupMemoryBarrierWithGroupSync();
        }
    }

    if (groupIndex == 0u)
    {
        float clamped_mouse = clamp(mouse_depth, group_depths[179u].x, group_depths[230u].x);
        float4 mouse_world_clamped = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view, float4((((float2(mouse_px) + 0.5f) / dims) - 0.5f) * float2(2.0f, -2.0f), clamped_mouse, 1.0f)));
        out_buffer.Store<float4>(32, float4(mouse_world_clamped.xyz / mouse_world_clamped.w, clamped_mouse));

        float clamped_center = clamp(center_depth, group_depths[179u].y, group_depths[230u].y);
        float4 center_world_clamped = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view, float4((((float2(center_px) + 0.5f) / dims) - 0.5f) * float2(2.0f, -2.0f), clamped_center, 1.0f)));
        out_buffer.Store<float4>(48, float4(center_world_clamped.xyz / center_world_clamped.w, clamped_center));
    }
}
