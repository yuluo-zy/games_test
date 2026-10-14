// ==RECONSTRUCTED== shaders/nani/draw_lists.inc.hlsl
// 依据:nani cull/fill bin 的原始逐 bin 调试类型:
// GpuFilteredInstance{raw}, GpuFilteredInstanceMeta{raw},
// NaniDrawListInfo{global_instance_offset};nani bin 的 DrawIndexedIndirectCommand
// 携带成员名 index_count/instance_count/first_index/vertex_offset/first_instance
// (与 wall/grass 链使用的 draw_indirect_types.hlsl 名称不同)。
// 状态:O (布局逐字)

#ifndef TINYGLADE_NANI_DRAW_LISTS_HLSL
#define TINYGLADE_NANI_DRAW_LISTS_HLSL

// 一条原始打包的过滤后实例索引(网格*64 + 网格内顶点/实例)。
struct GpuFilteredInstance
{
    uint raw;
};

// 逐实例元数据(绘制列表内的原始实例索引)。
struct GpuFilteredInstanceMeta
{
    uint raw;
};

// cull pass 读取的逐绘制列表信息(绘制列表基准偏移)。
struct NaniDrawListInfo
{
    uint global_instance_offset;
};

// nani 绘制列表链使用的 Vulkan DrawIndexedIndirectCommand。
struct DrawIndexedIndirectCommand
{
    uint index_count;
    uint instance_count;
    uint first_index;
    uint vertex_offset;
    uint first_instance;
};

#endif // TINYGLADE_NANI_DRAW_LISTS_HLSL
