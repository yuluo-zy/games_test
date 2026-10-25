// ==RECONSTRUCTED== shaders/wall/inc/draw_lists.hlsl | evidence: raw-per-bin OpMemberName of
// _wall_generate_draw_lists / _wall_wall_brick_lod{0,1} bins: InstancedWallData{transform
// (Affine3Packed), sin_deform_y (f4), global_arch_height_vals (f3), seed, source_id,
// wallspace_x_range (f2), flags} — 96B record | status: O (layout verbatim); I (helpers)
//
// 墙体绘制列表:每段墙的实例记录(变换、正弦变形、拱高、随机种子、来源、
// 墙空间 x 范围与标志位),共 96 字节,由墙体绘制列表构建器与砖墙光栅使用。

#ifndef TINYGLADE_WALL_INC_DRAW_LISTS_HLSL
#define TINYGLADE_WALL_INC_DRAW_LISTS_HLSL

#ifndef TINYGLADE_AFFINE3PACKED
#define TINYGLADE_AFFINE3PACKED
struct Affine3Packed
{
    float data[12];
};
#endif

// 墙体实例记录。
struct InstancedWallData
{
    Affine3Packed transform;
    float4 sin_deform_y;
    float3 global_arch_height_vals;
    uint seed;
    uint source_id;
    float2 wallspace_x_range;
    uint flags;
};

#endif // TINYGLADE_WALL_INC_DRAW_LISTS_HLSL
