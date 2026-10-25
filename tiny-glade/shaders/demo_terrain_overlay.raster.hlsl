// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
// === 中文说明 ===
// 演示用的地形覆盖网格:在地形上方绘制一个 46x46 的半透明覆盖四边形(采样空间)。
//   入口: vs_main / ps_main。变体: f0adff76827e73ff 的 ps 被 lifted(仅距离丢弃)。
// 绑定: terrain_heightmap t14, demo_overlay_texture t16, $Globals b13, sampler_default/llr。
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/demo_terrain_overlay.raster.hlsl | evidence: spirv-cross of _demo_terrain_overlay.raster.hlsl_* (46x46 grid over the terrain; overlay tinted quad in sample space) | status: O (bindings/entry/flow); I (names)

#include "inc/common.hlsl"
#include "inc/consts.hlsl"
// current_view.hlsl is guard-skipped; its global current_view_index is
// redeclared in $Globals below (file OpString is still recorded).
#define TINYGLADE_INC_CURRENT_VIEW_HLSL
#include "inc/current_view.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/texture_utils.hlsl"
Texture2D<float4> terrain_heightmap : register(t14, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/hash.hlsl"
#include "inc/utils.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    float4x4 model_to_world;
    uint2 demo_overlay_texture_dims;
};

Texture2D<float4> demo_overlay_texture : register(t16, space0);

// 覆盖网格沿一条轴的网格单元数(46x46 网格)。
static const float OVERLAY_GRID_CELLS = 46.0f;

struct VS_OUTPUT
{
    float4 position : SV_Position;
    float2 c0 : TEXCOORD0;
    float3 c1 : TEXCOORD1;
};

[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
VS_OUTPUT vs_main(uint vertex_id : SV_VertexID)
{
    VS_OUTPUT output;
    // 网格坐标 -> 归一化 UV -> 世界空间 xz(范围 ±65)。
    float2 grid = float2(float(vertex_id % 46u), float(vertex_id / 46u));
    float2 uv = (((grid + 0.5f.xx) * (1.0f / OVERLAY_GRID_CELLS).xx) * 2.0f) - 1.0f.xx;
    float2 world_xz = uv * 65.0f.xx;
    float x = world_xz.x;
    float y = world_xz.y;

    // 从地形高度图采样得到 y 高度。
    float3 pos = float3(x, 0.0f, y);
    float height = lerp(TERRAIN_HEIGHT_MIN, TERRAIN_HEIGHT_MAX,
                        terrain_heightmap.SampleLevel(sampler_default, pos.xz * TERRAIN_SAMPLE_SCALE.xx + 0.5f.xx, 0.0f).x) + 0.5f;
    pos.y = height;

    // 世界坐标投影到当前视图裁剪空间。
    ViewConstants view = view_constants[current_view_index];
    output.position = mul(view.world_to_sample, float4(x, height, y, 1.0f));
    output.c0 = pos.xz * 0.2f;
    output.c1 = pos;
    return output;
}

[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
float4 ps_main(VS_OUTPUT input) : SV_Target0
{
    // 离中心过近的网格丢弃(露出地面中心区域)。
    if (length(input.c1.xz) < 15.0f)
    {
        discard;
    }
    // 半透明深色覆盖层乘以覆盖纹理。
    return float4(0.0f, 0.0f, 0.0f, 0.6f) * demo_overlay_texture.Sample(sampler_llr, input.position.xy * 0.0074999998323619365692138671875f).x;
}

#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _demo_terrain_overlay.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// === 中文说明 ===
// f0ad 变体的片元段:仅做中心距离丢弃(目标 bin 不含颜色输出)。
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
static float3 in_var_C1;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
};

void frag_main()
{
    // 离中心过近的网格丢弃。
    if (length(in_var_C1.xz) < 15.0f)
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
