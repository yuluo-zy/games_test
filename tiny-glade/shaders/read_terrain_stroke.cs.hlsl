// ==RECONSTRUCTED== shaders/read_terrain_stroke.cs.hlsl | 证据: spirv-cross 还原
//   _read_terrain_stroke.cs.hlsl_cs_main_0.bin(push constant 查询位置 -> stroke_indices
//   采样 -> 把 terrain_stroke_index 写入 out_buffer 第 16 字节)| 状态: O
//   (绑定/入口/工作组尺寸/流程); I(命名)
//   用途: 读取鼠标/查询点所在的地形笔刷(stroke)索引,供 CPU 回读。把世界 xz
//   坐标按 1/130 缩放换算到高度图 UV,采样 stroke_indices 得到笔刷索引。
//   绑定: push_constants (query_position_xz), $Globals b13 (stroke_indices_dims),
//   out_buffer u15, stroke_indices t14, 工作组尺寸 [1,1,1], 入口 cs_main。

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"

// 世界 xz -> 高度图 uv(1/130 世界缩放)。
static const float TERRAIN_SAMPLE_SCALE = 0.0076923076994717121124267578125f;

// CPU 推送的查询世界位置。
struct PushConstants
{
    float2 query_position_xz;
};
[[vk::push_constant]]
PushConstants push_constants;

// 笔刷索引纹理尺寸。
cbuffer $Globals : register(b13, space0)
{
    uint2 stroke_indices_dims;
};

// 回读输出:查询点命中结果。
struct CsOut
{
    float4 mouse_raycast_position;
    int terrain_stroke_index;
};

RWStructuredBuffer<CsOut> out_buffer : register(u15, space0);
// 每个纹素记录一个地形笔刷索引。
Texture2D<int4> stroke_indices : register(t14, space0);

[numthreads(1, 1, 1)]
void cs_main()
{
    // 世界坐标 -> 高度图 UV([0,1],y 轴世界 z)。
    float3 query = (float3(push_constants.query_position_xz.x, 0.0f, push_constants.query_position_xz.y) * TERRAIN_SAMPLE_SCALE) + 0.5f;
    // 换算到纹素坐标并采样,取得该位置的笔刷索引。
    int2 pixel = int2(int(query.x * float(stroke_indices_dims.x)), int(query.z * float(stroke_indices_dims.y)));
    out_buffer[0].terrain_stroke_index = stroke_indices.Load(int3(pixel, 0)).x;
}
