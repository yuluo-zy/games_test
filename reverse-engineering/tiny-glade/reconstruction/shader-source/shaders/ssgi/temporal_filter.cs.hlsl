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
