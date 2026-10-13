// 程序化实例工具:仿射 3x4 变换助手与生成器着色器使用的紧凑变换记录。
// 布局与打包顺序直接取自 _rocky_terrain_stairs_stairs 等生成器消费端的反编译
// (Affine3Packed.data 逐列打包),与 wall/inc/draw_lists.hlsl 中的同名定义
// 布局一致,靠 include 顺序无关的守卫保持幂等。

#ifndef TINYGLADE_INC_UTILS_HLSL
#define TINYGLADE_INC_UTILS_HLSL

#include "math_const.hlsl"

// 3x4 仿射变换,以 12 个 float 打包,列主序:
// data[0..2]=第0列 xyz,data[3..5]=第1列,data[6..8]=第2列,data[9..11]=平移。
#ifndef TINYGLADE_AFFINE3PACKED
#define TINYGLADE_AFFINE3PACKED
struct Affine3Packed
{
    float data[12];
};
#endif

// 由前向方向构造仅含旋转的基矩阵(行向量 [forward; up; cross(forward,up); 0 0 0 1])。
float4x4 affine_basis(float3 forward)
{
    float3 f = normalize(forward);
    float3 up = float3(0.0f, 1.0f, 0.0f);
    float3 right = cross(f, up);
    float4x4 m = float4x4(
        float4(f, 0.0f),
        float4(up, 0.0f),
        float4(right, 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f));
    return transpose(m);
}

// 纯平移仿射矩阵。
float4x4 affine_translation(float3 pos)
{
    return float4x4(
        float4(1.0f, 0.0f, 0.0f, pos.x),
        float4(0.0f, 1.0f, 0.0f, pos.y),
        float4(0.0f, 0.0f, 1.0f, pos.z),
        float4(0.0f, 0.0f, 0.0f, 1.0f));
}

// 纯(可能非均匀)缩放仿射矩阵。
float4x4 affine_scale(float3 s)
{
    return float4x4(
        float4(s.x, 0.0f, 0.0f, 0.0f),
        float4(0.0f, s.y, 0.0f, 0.0f),
        float4(0.0f, 0.0f, s.z, 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f));
}

// 把列主序 4x4(旋转列 0..2 + 平移列 3)打包进 12-float 的 Affine3Packed,丢弃仿射行。
Affine3Packed pack_affine(float4x4 m)
{
    Affine3Packed o;
    o.data[0] = m[0].x; o.data[1] = m[1].x; o.data[2] = m[2].x;
    o.data[3] = m[0].y; o.data[4] = m[1].y; o.data[5] = m[2].y;
    o.data[6] = m[0].z; o.data[7] = m[1].z; o.data[8] = m[2].z;
    o.data[9] = m[0].w; o.data[10] = m[1].w; o.data[11] = m[2].w;
    return o;
}

// 还原列主序 4x4(补回仿射行)。
float4x4 unpack_affine(Affine3Packed a)
{
    float4x4 m;
    m[0] = float4(a.data[0], a.data[3], a.data[6], a.data[9]);
    m[1] = float4(a.data[1], a.data[4], a.data[7], a.data[10]);
    m[2] = float4(a.data[2], a.data[5], a.data[8], a.data[11]);
    m[3] = float4(0.0f, 0.0f, 0.0f, 1.0f);
    return m;
}

// 组合 平移 * 旋转 * 缩放 后打包。
Affine3Packed compose_affine_packed(float3 pos, float3 forward, float3 scale)
{
    return pack_affine(mul(affine_translation(pos), mul(affine_basis(forward), affine_scale(scale))));
}

#endif // TINYGLADE_INC_UTILS_HLSL
