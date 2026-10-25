// ============================================================
// inc/draw_indirect_types.hlsl —— 间接绘制命令结构
// 用途:定义 Vulkan 兼容的间接绘制命令结构。
//   消费方:_generate_grass、_wall_generate_draw_lists、_garden_*、
//   _nani_draw_indexed_indirect_to_dispatch 等。
// 入口:无(纯头文件)
// 关键布局:DrawIndexedIndirectCommand 步长 0x14,DrawIndirectCommand 步长 0x10
// ============================================================
// ==RECONSTRUCTED== shaders/inc/draw_indirect_types.hlsl | evidence: raw-per-bin (_generate_grass/_wall_generate_draw_lists/_garden_*/_nani_draw_indexed_indirect_to_dispatch) OpMemberName DrawIndexedIndirectCommand{count,instanceCount,firstIndex,vertexOffset,firstInstance} stride 0x14, DrawIndirectCommand{vertexCount,instanceCount,firstVertex,firstInstance} stride 0x10 | status: O

#ifndef TINYGLADE_INC_DRAW_INDIRECT_TYPES_HLSL
#define TINYGLADE_INC_DRAW_INDIRECT_TYPES_HLSL

// Vulkan 兼容的索引间接绘制命令(步长 0x14)。字段与顺序和嵌入的
// OpMemberName 表完全一致。
struct DrawIndexedIndirectCommand {
    uint count;
    uint instanceCount;
    uint firstIndex;
    uint vertexOffset;
    uint firstInstance;
};

// Vulkan 兼容的非索引间接绘制命令(步长 0x10)。
struct DrawIndirectCommand {
    uint vertexCount;
    uint instanceCount;
    uint firstVertex;
    uint firstInstance;
};

#endif // TINYGLADE_INC_DRAW_INDIRECT_TYPES_HLSL
