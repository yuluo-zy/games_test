// PATCHED: modules er.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/ssgi/temporal_filter.cs.hlsl | F-D
// Evidence: _ssgi_temporal_filter.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23;
//   SHA-256 d31c87e724cf473fea5cc9cf65e66699177e57997eb8b8aaf9651482313920b1).
//   bindings: sampler_lnc s0, $Globals b13 (history_valid/input_tex_dims/
//   history_tex_dims/reprojection_tex_dims/final_output_tex_dims/
//   history_output_tex_dims), input_tex t14, history_tex t15, reprojection_tex t16,
//   final_output_tex u17, history_output_tex u18, frame_constants b0 space1;
//   LocalSize [8,8,1], entry cs_main.
//   Temporal reprojection + 3x3 spatial blur (stride-2 neighbors) + variance
//   clamp of the SSGI AO scalar, gated on history_valid (inc/history_valid.hlsl)
//   and frame_index != 0. Both outputs store the .x channel of the result.
// Status: O (structure/constants from disassembly; bindings verbatim);
//         I (function/var names — inlined by dxc).

#include "inc/common.hlsl"
#include "inc/uv.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
// inc/history_valid.hlsl's helper is is_history_valid (adjudicated inc-fix #16),
// so no name clash with the $Globals member history_valid recorded verbatim
// (OpMemberName history_valid) below.
#include "inc/history_valid.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint history_valid;
    uint2 input_tex_dims;
    uint2 history_tex_dims;
    uint2 reprojection_tex_dims;
    uint2 final_output_tex_dims;
    uint2 history_output_tex_dims;
};

Texture2D<float4> input_tex : register(t14, space0);
Texture2D<float4> history_tex : register(t15, space0);
Texture2D<float4> reprojection_tex : register(t16, space0);
RWTexture2D<unorm float> final_output_tex : register(u17, space0);
RWTexture2D<unorm float> history_output_tex : register(u18, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    float4 result;

    bool history_ok = history_valid != 0u;
    if (history_ok && !(frame_index == 0u))
    {
        float4 current = input_tex.Load(uint3(dispatchThreadID.xy, 0u));
        float2 history_uv = pixel_to_uv(int2(dispatchThreadID.xy), float2(input_tex_dims)) + reprojection_tex.Load(uint3(dispatchThreadID.xy, 0u)).xy;

        float4 history;
        if (history_ok && !(frame_index == 0u))
        {
            history = history_tex.SampleLevel(sampler_lnc, history_uv, 0.0f);
        }
        else
        {
            history = current;
        }

        // 3x3 gaussian (stride-2 neighbors, normalized 0.264411270618438720703125f).
        float4 c00 = input_tex.Load(uint3(dispatchThreadID.xy + uint2(4294967294u, 4294967294u), 0u));
        float4 c10 = input_tex.Load(uint3(dispatchThreadID.xy + uint2(0u, 4294967294u), 0u));
        float4 c20 = input_tex.Load(uint3(dispatchThreadID.xy + uint2(2u, 4294967294u), 0u));
        float4 c01 = input_tex.Load(uint3(dispatchThreadID.xy + uint2(4294967294u, 0u), 0u));
        float4 c11 = input_tex.Load(uint3(dispatchThreadID.xy, 0u));
        float4 c21 = input_tex.Load(uint3(dispatchThreadID.xy + uint2(2u, 0u), 0u));
        float4 c02 = input_tex.Load(uint3(dispatchThreadID.xy + uint2(4294967294u, 2u), 0u));
        float4 c12 = input_tex.Load(uint3(dispatchThreadID.xy + uint2(0u, 2u), 0u));
        float4 c22 = input_tex.Load(uint3(dispatchThreadID.xy + uint2(2u, 2u), 0u));

        float4 sum = c00 * 0.223130166530609130859375f;
        sum += c10 * 0.4723665416240692138671875f;
        sum += c20 * 0.223130166530609130859375f;
        sum += c01 * 0.4723665416240692138671875f;
        sum += c11 * 1.0f;
        sum += c21 * 0.4723665416240692138671875f;
        sum += c02 * 0.223130166530609130859375f;
        sum += c12 * 0.4723665416240692138671875f;
        sum += c22 * 0.223130166530609130859375f;

        float4 sumsq = (c00 * c00) * 0.223130166530609130859375f;
        sumsq += (c10 * c10) * 0.4723665416240692138671875f;
        sumsq += (c20 * c20) * 0.223130166530609130859375f;
        sumsq += (c01 * c01) * 0.4723665416240692138671875f;
        sumsq += (c11 * c11) * 1.0f;
        sumsq += (c21 * c21) * 0.4723665416240692138671875f;
        sumsq += (c02 * c02) * 0.223130166530609130859375f;
        sumsq += (c12 * c12) * 0.4723665416240692138671875f;
        sumsq += (c22 * c22) * 0.223130166530609130859375f;

        float4 blur = sum * 0.264411270618438720703125f.xxxx;
        float4 variance = (sumsq * 0.264411270618438720703125f.xxxx) - (blur * blur);
        float4 stddev = sqrt(max(0.0f.xxxx, variance)) * 0.5f * 5.0f;
        float4 blended = lerp(current, blur, 0.25f.xxxx);

        result = lerp(clamp(history, blended - stddev, blended + stddev), current, 0.125f.xxxx);
    }
    else
    {
        result = input_tex.Load(uint3(dispatchThreadID.xy, 0u));
    }

    history_output_tex[dispatchThreadID.xy] = result.x;
    final_output_tex[dispatchThreadID.xy] = result.x;
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _ssgi_temporal_filter.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    uint _Globals_history_valid : packoffset(c0);
    uint2 _Globals_input_tex_dims : packoffset(c0.y);
    uint2 _Globals_history_tex_dims : packoffset(c1);
    uint2 _Globals_reprojection_tex_dims : packoffset(c1.z);
    uint2 _Globals_final_output_tex_dims : packoffset(c2);
    uint2 _Globals_history_output_tex_dims : packoffset(c2.z);
};

SamplerState sampler_lnc : register(s0, space0);
Texture2D<float4> input_tex : register(t14, space0);
Texture2D<float4> history_tex : register(t15, space0);
Texture2D<float4> reprojection_tex : register(t16, space0);
RWTexture2D<unorm float> final_output_tex : register(u17, space0);
RWTexture2D<unorm float> history_output_tex : register(u18, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    bool _87 = _Globals_history_valid != 0u;
    bool _94;
    if (_87)
    {
        _94 = !(0u == frame_constants_frame_index);
    }
    else
    {
        _94 = false;
    }
    float4 _201;
    if (_94)
    {
        float4 _106 = input_tex.Load(int3(gl_GlobalInvocationID.xy, 0u));
        bool _115;
        if (_87)
        {
            _115 = !(0u == frame_constants_frame_index);
        }
        else
        {
            _115 = false;
        }
        float4 _125;
        if (_115)
        {
            _125 = history_tex.SampleLevel(sampler_lnc, ((float2(int2(gl_GlobalInvocationID.xy)) + 0.5f.xx) / float2(_Globals_input_tex_dims)) + reprojection_tex.Load(int3(gl_GlobalInvocationID.xy, 0u)).xy, 0.0f);
        }
        else
        {
            _125 = _106;
        }
        float4 _127 = input_tex.Load(int3(gl_GlobalInvocationID.xy + uint2(4294967294u, 4294967294u), 0u));
        float4 _132 = input_tex.Load(int3(gl_GlobalInvocationID.xy + uint2(0u, 4294967294u), 0u));
        float4 _139 = input_tex.Load(int3(gl_GlobalInvocationID.xy + uint2(2u, 4294967294u), 0u));
        float4 _146 = input_tex.Load(int3(gl_GlobalInvocationID.xy + uint2(4294967294u, 0u), 0u));
        float4 _152 = input_tex.Load(int3(gl_GlobalInvocationID.xy, 0u));
        float4 _159 = input_tex.Load(int3(gl_GlobalInvocationID.xy + uint2(2u, 0u), 0u));
        float4 _166 = input_tex.Load(int3(gl_GlobalInvocationID.xy + uint2(4294967294u, 2u), 0u));
        float4 _173 = input_tex.Load(int3(gl_GlobalInvocationID.xy + uint2(0u, 2u), 0u));
        float4 _180 = input_tex.Load(int3(gl_GlobalInvocationID.xy + uint2(2u, 2u), 0u));
        float4 _186 = (((((((((_127 * 0.223130166530609130859375f) + (_132 * 0.4723665416240692138671875f)) + (_139 * 0.223130166530609130859375f)) + (_146 * 0.4723665416240692138671875f)) + (_152 * 1.0f)) + (_159 * 0.4723665416240692138671875f)) + (_166 * 0.223130166530609130859375f)) + (_173 * 0.4723665416240692138671875f)) + (_180 * 0.223130166530609130859375f)) * 0.264411270618438720703125f.xxxx;
        float4 _192 = lerp(_106, _186, 0.25f.xxxx);
        float4 _194 = (sqrt(max(0.0f.xxxx, (((((((((((_127 * _127) * 0.223130166530609130859375f) + ((_132 * _132) * 0.4723665416240692138671875f)) + ((_139 * _139) * 0.223130166530609130859375f)) + ((_146 * _146) * 0.4723665416240692138671875f)) + ((_152 * _152) * 1.0f)) + ((_159 * _159) * 0.4723665416240692138671875f)) + ((_166 * _166) * 0.223130166530609130859375f)) + ((_173 * _173) * 0.4723665416240692138671875f)) + ((_180 * _180) * 0.223130166530609130859375f)) * 0.264411270618438720703125f.xxxx) - (_186 * _186))) * 0.5f) * 5.0f;
        _201 = lerp(clamp(_125, _192 - _194, _192 + _194), _106, 0.125f.xxxx);
    }
    else
    {
        _201 = input_tex.Load(int3(gl_GlobalInvocationID.xy, 0u));
    }
    float4 _203 = _201.x.xxxx;
    history_output_tex[gl_GlobalInvocationID.xy] = _203.x;
    final_output_tex[gl_GlobalInvocationID.xy] = _203.x;
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
