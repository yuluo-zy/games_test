#define TGV_0 1
#line 1 "shaders/read_mouse_depth_value.cs.hlsl"


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
#line 16 "shaders/read_mouse_depth_value.cs.hlsl"

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
#line 17 "shaders/read_mouse_depth_value.cs.hlsl"

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
#line 18 "shaders/read_mouse_depth_value.cs.hlsl"

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
#line 19 "shaders/read_mouse_depth_value.cs.hlsl"


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
    uint2 _Globals_depth_map_dims : packoffset(c0);
    float2 _Globals_mouse_position_screenspace : packoffset(c0.z);
    float2 _Globals_window_size : packoffset(c1);
    float _Globals_filter_box_y_frac : packoffset(c1.z);
};

RWByteAddressBuffer out_buffer : register(u15, space0);
Texture2D<float4> depth_map : register(t14, space0);

static uint3 gl_LocalInvocationID;
static uint gl_LocalInvocationIndex;
struct SPIRV_Cross_Input
{
    uint3 gl_LocalInvocationID : SV_GroupThreadID;
    uint gl_LocalInvocationIndex : SV_GroupIndex;
};

groupshared float2 group_depths[256];

void comp_main()
{
    float2 _93 = _Globals_mouse_position_screenspace / _Globals_window_size;
    float2 _96 = float2(_Globals_depth_map_dims);
    uint2 _100 = _Globals_depth_map_dims - uint2(1u, 1u);
    int2 _102 = int2(clamp(uint2(int2(_93 * _96)), uint2(0u, 0u), _100));
    uint2 _103 = uint2(_102);
    float4 _105 = depth_map.Load(int3(_103, 0u));
    float _106 = _105.x;
    float4 _122 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4((((float2(int2(clamp(_103, uint2(0u, 0u), _100))) + 0.5f.xx) / _96) - 0.5f.xx) * float2(2.0f, -2.0f), _106, 1.0f)));
    int2 _135 = int2(clamp(uint2(int2(0.5f.xx * _96)), uint2(0u, 0u), _100));
    uint2 _136 = uint2(_135);
    float4 _137 = depth_map.Load(int3(_136, 0u));
    float _138 = _137.x;
    float4 _150 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4((((float2(int2(clamp(_136, uint2(0u, 0u), _100))) + 0.5f.xx) / _96) - 0.5f.xx) * float2(2.0f, -2.0f), _138, 1.0f)));
    bool _159 = 0u == gl_LocalInvocationIndex;
    if (_159)
    {
        out_buffer.Store<float4>(0, float4(_122.xyz / _122.w.xxx, _106));
        out_buffer.Store<float4>(16, float4(_150.xyz / _150.w.xxx, _138));
    }
    float2 _176 = ((float(_Globals_depth_map_dims.y) * _Globals_filter_box_y_frac).xx / _96) * (((float2(gl_LocalInvocationID.xy) + 0.5f.xx) * 0.0625f.xx) - 0.5f.xx);
    group_depths[gl_LocalInvocationIndex] = float2(depth_map.Load(int3(uint2(int2(clamp(uint2(int2((_93 + _176) * _96)), uint2(0u, 0u), _100))), 0u)).x, depth_map.Load(int3(uint2(int2(clamp(uint2(int2((0.5f.xx + _176) * _96)), uint2(0u, 0u), _100))), 0u)).x);
    GroupMemoryBarrierWithGroupSync();
    uint _199 = 2u << (firstbithigh(255u) & 31u);
    for (uint _201 = 2u; _201 <= _199; _201 *= 2u)
    {
        uint _207 = _201 >> 1u;
        for (uint _209 = _207; _209 > 0u; _209 = _209 >> 1u)
        {
            uint _216 = gl_LocalInvocationIndex & (_209 - 1u);
            uint _218 = 2u * (gl_LocalInvocationIndex - _216);
            uint _219 = _218 + _216;
            uint _230;
            if (_209 == _207)
            {
                _230 = (_218 + ((2u * _209) - 1u)) - _216;
            }
            else
            {
                _230 = (_218 + _209) + _216;
            }
            bool _235;
            if (_230 < 256u)
            {
                _235 = _219 < 256u;
            }
            else
            {
                _235 = false;
            }
            if (_235)
            {
                float2 _239 = group_depths[_219];
                float2 _241 = group_depths[_230];
                bool2 _242 = bool2(_239.x <= _241.x, _239.y <= _241.y);
                group_depths[_219] = float2(_242.x ? _239.x : _241.x, _242.y ? _239.y : _241.y);
                group_depths[_230] = float2(_242.x ? _241.x : _239.x, _242.y ? _241.y : _239.y);
            }
            GroupMemoryBarrierWithGroupSync();
        }
    }
    if (_159)
    {
        float _251 = clamp(_106, group_depths[179u].x, group_depths[230u].x);
        float4 _261 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4((((float2(_102) + 0.5f.xx) / _96) - 0.5f.xx) * float2(2.0f, -2.0f), _251, 1.0f)));
        out_buffer.Store<float4>(32, float4(_261.xyz / _261.w.xxx, _251));
        float _275 = clamp(_138, group_depths[179u].y, group_depths[230u].y);
        float4 _285 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4((((float2(_135) + 0.5f.xx) / _96) - 0.5f.xx) * float2(2.0f, -2.0f), _275, 1.0f)));
        out_buffer.Store<float4>(48, float4(_285.xyz / _285.w.xxx, _275));
    }
}

[numthreads(16, 16, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_LocalInvocationID = stage_input.gl_LocalInvocationID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
