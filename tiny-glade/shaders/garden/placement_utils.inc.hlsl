// ==RECONSTRUCTED== shaders/garden/placement_utils.inc.hlsl
// 依据:_garden_spawn_* + _water_spawn_* 中的内联使用点
// (世界网格 uv: ((float2(id.xy)+0.5)/grid_dims-0.5)*130;放置高度图 uv: world*0.007692307699+0.5;
// yaw 旋转 = transpose(float4x4(rows)) 绕 normalize(0,1,0) 的 Rodrigues;
// compose = mul(mul(translate, mul(identity, scale)), rot);
// Flower03Xform{float4 position} 用于 _garden_spawn_small_flowers_garden)。
// 状态:I (函数/结构名推断——内联、未嵌入);O (算法/常量取自内联 SPIR-V)

#ifndef TINYGLADE_GARDEN_PLACEMENT_UTILS_HLSL
#define TINYGLADE_GARDEN_PLACEMENT_UTILS_HLSL

// Affine3Packed / rand01 / hash / terrain_heightmap 辅助来自消费方先前 include
// (utils.hlsl, hash.hlsl, terrain_heightmap.inc.hlsl)。
// 本文件补充家族共享的放置记录 + 辅助函数。

struct Flower03Xform
{
    float4 position;
};

// 网格分发 id -> 居中的世界 xz(地形范围 130,世界中心 0)。
float2 grid_world_pos(float2 id, int2 grid_dims)
{
    return ((float2(id) + 0.5f) / float2(grid_dims) - 0.5f) * 130.0f;
}

// 世界 xz -> 放置遮罩高度图 uv。
float2 placement_uv(float2 world_xz)
{
    return world_xz * 0.0076923076994717121124267578125f + 0.5f;
}

// 绕任意轴的 Rodrigues 旋转,按放置 pass 嵌入的行主 4x4 转置形式构造
// (与内联数学一致)。
float4x4 rotate_axis(float3 axis, float angle)
{
    float c = cos(angle);
    float s = sin(angle);
    float omc = 1.0f - c;
    float x = axis.x;
    float y = axis.y;
    float z = axis.z;
    return transpose(float4x4(
        float4(omc * x * x + c, omc * x * y - s * z, omc * x * z + s * y, 0.0f),
        float4(omc * x * y + s * z, omc * y * y + c, omc * y * z - s * x, 0.0f),
        float4(omc * x * z - s * y, omc * y * z + s * x, omc * z * z + c, 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f)));
}

// 绕竖直轴的偏航旋转(放置 pass 的常见情形)。
float4x4 rotate_y(float angle)
{
    return rotate_axis(float3(0.0f, 1.0f, 0.0f), angle);
}

// translate * (identity * scale) * rotation —— 放置 pass 生成的精确组合顺序
// (identity 作为显式因子保留,与原编译一致)。
float4x4 placement_transform(float3 pos, float3 scale, float yaw)
{
    float4x4 identity = float4x4(
        float4(1.0f, 0.0f, 0.0f, 0.0f),
        float4(0.0f, 1.0f, 0.0f, 0.0f),
        float4(0.0f, 0.0f, 1.0f, 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f));
    float4x4 s = float4x4(
        float4(scale.x, 0.0f, 0.0f, 0.0f),
        float4(0.0f, scale.y, 0.0f, 0.0f),
        float4(0.0f, 0.0f, scale.z, 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f));
    return mul(mul(affine_translation(pos), mul(identity, s)), rotate_y(yaw));
}

// 在字节/结构化缓冲中写入一条 Flower03Xform 记录(float4 position)。
void store_flower03(RWByteAddressBuffer buffer, uint base, float4 position)
{
    buffer.Store<float4>(base * 16u + 0u, position);
}

#endif // TINYGLADE_GARDEN_PLACEMENT_UTILS_HLSL

// 生成 pass 共享的花园遮罩排除测试。`uv` 为放置高度图 uv;
// px = uv * path_mask_dims。返回该格子是否被排除
// (路径 / 岩石 / 陡坡 / 水面 / 植被排除区)。
bool placement_excluded(
    Texture2D<float4> terrain_heightmap,
    uint2 terrain_heightmap_dims,
    Texture2D<float4> path_mask,
    uint2 path_mask_dims,
    Texture2D<float4> rocky_terrain,
    Texture2D<float4> water_mask,
    Texture2D<float4> foliage_exclusion_mask,
    SamplerState sampler_llb,
    SamplerState sampler_default,
    float2 uv)
{
    uint2 px = uint2(uv * float2(path_mask_dims));
    if (path_mask.Load(int3(px, 0u)).x > 0.2f)
        return true;

    float4 g = terrain_heightmap.GatherRed(sampler_default, uv, int2(0, 0));
    float2 grad = (float2((g.z + g.y) - (g.w + g.x), (g.x + g.y) - (g.w + g.z)) * 0.5f) * 26.5f;
    grad /= (130.0f / max(float2(terrain_heightmap_dims), 1.0e-6f));
    if (rocky_terrain.Load(int3(px, 0u)).x > 0.0f || length(grad) > 0.9f)
        return true;

    if (water_mask.Load(int3(px, 0u)).y > 0.3f)
        return true;

    if (foliage_exclusion_mask.SampleLevel(sampler_llb, uv, 0.0f).x > 0.0f)
        return true;

    return false;
}

// 带状衰减(d==0 时为 1,d >= 1/inv_half_width 时为 0),花园/水面生成器
// 共享的放置评分曲线(1 - t^2*(3 - d*2*inv))。
float band_falloff_garden(float d, float inv_half_width)
{
    float t = d * inv_half_width;
    return 1.0f - (t * t) * (3.0f - d * (2.0f * inv_half_width));
}
