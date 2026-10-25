// ============================================================
// terrain_editing/calc_highlight_hill_mask.cs.hlsl
// 功能:计算"高亮小山蒙版"——凡是高亮笔划经过的像素写 1,其余写 0。
// 入口:cs_main,8x8 线程/工作组。
// 绑定:stroke_indices (t14, space0)、img_out (u15, space0);
//       $Globals b13 携带 stroke_indices_dims / highlight_stroke_index / img_out_dims。
// ============================================================

// ==RECONSTRUCTED== shaders/terrain_editing/calc_highlight_hill_mask.cs.hlsl | evidence: spirv-cross of _terrain_editing_calc_highlight_hill_mask.cs.hlsl_cs_main_0.bin (fill mask where the highlight stroke passes) | status: O (bindings/entry/LocalSize/flow); I (names)

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 stroke_indices_dims;   // 笔划索引图的尺寸
    int highlight_stroke_index;  // 高亮笔划的索引(-1 表示没有高亮)
    uint2 img_out_dims;          // 输出蒙版尺寸
};

// 每个像素记录它属于哪条笔划的索引图。
Texture2D<int4> stroke_indices : register(t14, space0);
// 高亮蒙版输出(半精度)。
RWTexture2D<half> img_out : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_thread_id : SV_DispatchThreadID)
{
    if (highlight_stroke_index == -1)
    {
        // 没有高亮笔划:整张蒙版清零。
        img_out[uint2(int3(dispatch_thread_id).xy)] = 0.0f;
    }
    else
    {
        int2 dims = int2(stroke_indices_dims);
        // 输出像素在 [0,1] 的 UV(半像素偏移),再居中到 [-0.5, 0.5]。
        float2 uv = (float2(int3(dispatch_thread_id).xy) + 0.5f) / float2(int2(img_out_dims)) - 0.5f;
        // 把 UV 按地形比例(130 米)换算到高度图空间。
        float3 query = ((float2(130.0f, 130.0f) * uv).xyy * 0.0076923076994717121124267578125f.xxx) + 0.5f.xxx;
        // 取整得到笔划索引图的采样像素。
        int2 pixel = int2(int(query.x * float(dims.x)), int(query.z * float(dims.y)));
        // 若该像素属于高亮笔划,则蒙版写 1,否则写 0。
        if (stroke_indices.Load(int3(uint2(pixel), 0)).x == highlight_stroke_index)
        {
            img_out[uint2(int3(dispatch_thread_id).xy)] = 1.0f;
        }
        else
        {
            img_out[uint2(int3(dispatch_thread_id).xy)] = 0.0f;
        }
    }
}
