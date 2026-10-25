// ============================================================
// terrain_editing/inc_terrain_stroke.inc.hlsl
// 功能:地形编辑笔刷笔画(TerrainStroke)的结构定义与距离/影响力函数。
// 消费方:rasterize_terrain_stroke(把笔画光栅化进 img_output /
//         rocky_terrain / terrain_sdg)。
// 说明:笔画的 $Globals 在消费方声明(stroke_width / stroke_infos_device_address /
//        stroke_count);本头依赖 sdf_utils.inc.hlsl。
// ============================================================

// ==RECONSTRUCTED== shaders/terrain_editing/inc_terrain_stroke.inc.hlsl | evidence: include-graph direct edge -> sdf_utils.inc.hlsl; consumed only by rasterize_terrain_stroke (stroke list rasterized into img_output / rocky_terrain / terrain_sdg); $Globals of that pass carries stroke_width / stroke_infos_device_address / stroke_count | status: O (include edge, stroke role); I (struct/body Inference — inlined)

#ifndef TINYGLADE_TERRAIN_EDITING_INC_TERRAIN_STROKE_HLSL
#define TINYGLADE_TERRAIN_EDITING_INC_TERRAIN_STROKE_HLSL

#include "sdf_utils.inc.hlsl"

// 一条地形编辑笔画,由 CPU 写入笔画信息缓冲
// (缓冲设备地址在消费方着色器的 $Globals 中)。
struct TerrainStroke
{
    float2 start_xz; // 笔画起点(世界 xz)
    float2 end_xz;   // 笔画终点(世界 xz)
    float width;     // 笔画宽度(世界单位)
    float falloff;   // 边缘衰减距离
    int kind;        // 笔画类型
    int pad;         // 对齐填充
};

// 世界坐标点相对笔画中心线的带符号距离(世界单位)。
// 笔画被压平到地形平面(xz)。
float stroke_distance(float2 world_xz, TerrainStroke stroke)
{
    return sd_segment(world_xz, stroke.start_xz, stroke.end_xz) - stroke.width * 0.5f;
}

// 归一化笔画影响力:[0,1],按衰减距离线性过渡。
float stroke_influence(float2 world_xz, TerrainStroke stroke)
{
    float d = stroke_distance(world_xz, stroke);
    return 1.0f - clamp(d / max(stroke.falloff, 1e-4f), 0.0f, 1.0f);
}

#endif // TINYGLADE_TERRAIN_EDITING_INC_TERRAIN_STROKE_HLSL
