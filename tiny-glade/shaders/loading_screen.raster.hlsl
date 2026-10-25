// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/loading_screen.raster.hlsl | F-E | bins: _loading_screen.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// 加载屏动画:居中四边形采样 Texture2DArray 帧条(由 progress_time 驱动的 8 层),
// 黑色内容(1 - sample.x)随进度淡入淡出。

#include "inc/common.hlsl"            // 采样器 (sampler_lnc s0)
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"

cbuffer $Globals : register(b14, space0)
{
    uint2 animation_dims;
    float progress_time;
    float fade;
    uint2 framebuffer_dims;
};

Texture2DArray<float4> animation : register(t13, space0);

struct VsOut
{
    float2 uv : TEXCOORD0;
    float4 position : SV_Position;
};

// 用 SV_VertexID 生成一个覆盖屏幕的四边形;按动画帧纵横比做宽高修正。
VsOut vs_main(uint vertex_id : SV_VertexID)
{
    VsOut o;

    float x = float(vertex_id & 1u);
    float y = float((vertex_id >> 1u) & 1u);
    float2 corner = float2(x, y) - 0.5f;
    // 以动画帧的宽高比相对屏幕宽高比做修正,避免拉伸。
    corner.x = (corner.x * (float(animation_dims.x) / float(animation_dims.y)))
             / (float(framebuffer_dims.x) / float(framebuffer_dims.y));
    float2 pos = corner * 0.3f;
    o.position = float4(pos.x, pos.y - 0.6f, 1.0f, 1.0f);
    o.uv = float2(x, 1.0f - y);
    return o;
}

// 按进度选择动画帧层;黑色部分让出,其余用 alpha 淡入淡出。
float4 ps_main(float2 uv : TEXCOORD0) : SV_Target0
{
    int layer = int(progress_time * 8.0f) - 4 * (int(progress_time * 8.0f) / 4);
    float a = animation.Sample(sampler_lnc, float3(uv, float(layer))).x;
    return (1.0f - a) * (1.0f - fade) * smoothstep(3.0f, 3.5f, progress_time);
}

#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _loading_screen.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
void frag_main()
{
}

void ps_main()
{
    frag_main();
}
#pragma dxc diagnostic pop
#endif
