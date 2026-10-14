// ==RECONSTRUCTED== shaders/stairs_floor.inc.hlsl | F-V | embedded in nani_cull_and_bucket_stairs_floor.cs (2) + nani_stairs_floor.raster (2)
// 楼梯地板共享记录类型。字段布局/名称从消费 bin 的 OpMemberName 表按原样恢复:
//   NaniTrimeshChunk: gpu_address (uint64), vertex_index_count, vertex_byte_offset, bounding_sphere.
//   StairsFloorInstanceData: { NaniTrimeshChunk mesh; uint color_id; pad0; pad1; pad2 }.

#ifndef TINYGLADE_STAIRS_FLOOR_INC_HLSL
#define TINYGLADE_STAIRS_FLOOR_INC_HLSL

// GPU 网格块,由楼梯实例通过物理地址(PhysicalStorageBufferAddress)引用。
struct NaniTrimeshChunk
{
    uint64_t gpu_address;
    uint vertex_index_count;
    uint vertex_byte_offset;
    float4 bounding_sphere;
};

// 楼梯地板逐实例记录:网格块 + 配色 id(其余为对齐填充)。
struct StairsFloorInstanceData
{
    NaniTrimeshChunk mesh;
    uint color_id;
    uint pad0;
    uint pad1;
    uint pad2;
};

#endif // TINYGLADE_STAIRS_FLOOR_INC_HLSL
