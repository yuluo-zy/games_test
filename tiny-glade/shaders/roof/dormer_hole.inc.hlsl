// ==RECONSTRUCTED== shaders/roof/dormer_hole.inc.hlsl
// 依据:F-V | 内嵌于 nani_instanced_roof.raster(2) + nani_roof_floor.raster(2) + nani_stairs_floor.raster(2)
// 老虎窗洞记录 + 遮挡辅助。DormerHole 布局/字段名取自消费方 OpMemberName 表
// (position f3, wall_id u, size f3, pad0 f2, normal f2, k, a;48 字节记录)。
// 遮挡数学根据 _nani_roof_floor ps_main 的内联使用点重建:
// 处于朝向老虎窗包围盒内(按 size 缩放、k/a 圆滑)的片元被丢弃。

#ifndef TINYGLADE_ROOF_DORMER_HOLE_HLSL
#define TINYGLADE_ROOF_DORMER_HOLE_HLSL

// 一条老虎窗洞记录(48 字节,存放于 `dormer_holes` 字节寻址缓冲)。
struct DormerHole
{
    float3 position;   // 世界空间位置
    uint wall_id;      // 所属墙体 id
    float3 size;       // 洞口尺寸
    float2 pad0;       // 对齐填充
    float2 normal;     // basis0_xz,洞口法线(xz)
    float k;           // 圆滑系数
    float a;           // 圆滑偏移
};

// 判断世界空间片元是否落在老虎窗洞内。根据内联使用点重建:
// 片元投影进老虎窗局部坐标系(法线取自 `normal`),按 size 缩放,
// 落在圆滑老虎窗轮廓内则判为洞内。
bool is_inside_dormer_hole(float3 world_pos, DormerHole d)
{
    float3 basis = float3(d.normal.x, 0.0f, d.normal.y);
    float3x3 basis_mat = transpose(float3x3(cross(basis, float3(0.0f, 1.0f, 0.0f)),
                                            float3(0.0f, 1.0f, 0.0f), basis));
    float3 b0 = mul(basis_mat, float3(1.0f, 0.0f, 0.0f));
    float3 b1 = mul(basis_mat, float3(0.0f, 1.0f, 0.0f));
    float3 b2 = mul(basis_mat, float3(0.0f, 0.0f, 1.0f));
    float3 n = cross(b0, b1);
    float3 inv = (1.0f / dot(b2, n)).xxx;
    float3 p = mul(world_pos - d.position, float3x3(cross(b1, b2) * inv, cross(b2, b0) * inv, n * inv)) / d.size;
    float w = clamp((p.y * d.k) + d.a, 0.0f, 1.0f);
    bool inside = abs(p.x) < lerp(0.5f, 0.0f, (w * w) * w)
               && ((d.k == 1.0f) ? (p.y < 0.5f) && (p.y > -0.5f) : p.y < 0.7f)
               && abs(p.z) < 0.5f;
    return inside;
}

#endif // TINYGLADE_ROOF_DORMER_HOLE_HLSL
