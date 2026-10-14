// ==RECONSTRUCTED== shaders/plaster.inc.hlsl | F-V | 内嵌于 nani_cull_and_bucket_plaster.cs (2) + nani_plaster.raster (2)
// 用途: 灰泥墙共享记录类型。成员布局/名称逐字取自消费方二进制 OpMemberName 表:
//   PlasterInstanceData: bounding_sphere (float4), wall_id (uint), wall_length (float),
//     blend_uv_seam (float), quad_count (uint), verts_gpu_address (uint64),
//     row_stride (uint), peel_strength (float)  (64 字节紧凑布局)。
// 函数体在本 pass 未知(物理寻址消费方;nani 家族细化)。

#ifndef TINYGLADE_PLASTER_INC_HLSL
#define TINYGLADE_PLASTER_INC_HLSL

// 每面墙的灰泥四边形记录(光栅化器把每个实例展开成 quad_count 个四边形)。
struct PlasterInstanceData
{
    float4 bounding_sphere;
    uint wall_id;
    float wall_length;
    float blend_uv_seam;
    uint quad_count;
    uint64_t verts_gpu_address;
    uint row_stride;
    float peel_strength;
};

#endif // TINYGLADE_PLASTER_INC_HLSL
