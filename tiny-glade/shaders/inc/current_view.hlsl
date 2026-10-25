// ============================================================
// inc/current_view.hlsl —— 当前相机/视图选择
// 用途:从 frame_constants.view_constants[5] 选出当前激活的相机视图矩阵。
//   `current_view_index` 是着色器全局变量(落在 per-shader 的 $Globals UBO 里)。
// 入口:无(纯头文件;依赖 frame_constants.hlsl)
// 关键绑定:current_view_index 位于 $Globals 成员 0(178 个 bin 消费)
// ============================================================
// ==RECONSTRUCTED== shaders/inc/current_view.hlsl | evidence: raw-per-bin.jsonl `current_view_index` at $Globals member 0 in 178 bins (exactly the current_view.hlsl consumer set; e.g. _billboard_forest uses frame_constants.view_constants[_Globals_current_view_index].world_to_sample); include-graph edge current_view.hlsl -> frame_constants.hlsl | status: O (global current_view_index); O/I (GetCurrentViewConstants — accessor inferred from the inlined view_constants[current_view_index] access chains at use sites)
//
// Selects the active camera/view out of frame_constants.view_constants[5].
// `current_view_index` is a shader-global (lands in the per-shader $Globals UBO).

#ifndef TINYGLADE_INC_CURRENT_VIEW_HLSL
#define TINYGLADE_INC_CURRENT_VIEW_HLSL

#include "frame_constants.hlsl"

// 当前激活的视图索引(着色器全局变量)。
uint current_view_index;

// 当前激活相机的视图矩阵。
ViewConstants GetCurrentViewConstants()
{
    return view_constants[current_view_index];
}

#endif // TINYGLADE_INC_CURRENT_VIEW_HLSL
