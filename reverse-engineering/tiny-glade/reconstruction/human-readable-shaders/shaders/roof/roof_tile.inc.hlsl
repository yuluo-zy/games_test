// ==RECONSTRUCTED== shaders/roof/roof_tile.inc.hlsl
// 依据:F-V | 内嵌于 nani_cull_and_bucket_roof_tile.cs(2), nani_instanced_roof.raster(2), roof_tiles_gravity.cs
// 屋顶族共享的瓦片记录类型 + 辅助函数(实例化屋顶栅格化与重力模拟)。
// 成员布局与字段名逐字取自消费方 bin 的 OpMemberName / raw-per-bin.jsonl:
//   RoofShingle   64B: base_position f3 +0, current_height f +12, clipping_plane_ u2 +16,
//                     scale_t u2 +24, rotation_quat_ u2 +32, roof_id u +40,
//                     velocity_rotation u +44, animation_t f +48, seed i +52,
//                     is_circle i +56, is_main_roof_body i +60。
//   PushConstants (cbuffer 'push_constants'): first_instance, instance_count,
//                     wall_height_delta, roof_pivot_y, animation_t。
// 辅助函数名(pack_half2x16 / unpack_half2x16 / quat_to_matrix)为推断
// (在消费方内联;仅类型/全局变量带 OpName)。

#ifndef TINYGLADE_ROOF_ROOF_TILE_HLSL
#define TINYGLADE_ROOF_ROOF_TILE_HLSL

// 单条屋顶瓦片实例记录(也用作 `instanced_data` / `__alias_instanced_data`
// StructuredBuffer 的元素类型)。
struct RoofShingle
{
    float3 base_position;      // 基准位置
    float current_height;      // 当前高度
    uint2 clipping_plane_;     // 裁剪平面参数
    uint2 scale_t;             // 缩放与时间参数
    uint2 rotation_quat_;      // 打包的四元数
    uint roof_id;              // 所属屋顶 id
    uint velocity_rotation;    // 旋转速度(打包)
    float animation_t;         // 动画时间
    int seed;                  // 随机种子
    int is_circle;             // 是否圆形瓦片
    int is_main_roof_body;     // 是否主体屋顶
};

// 屋顶族 push constants(绑定为着色器 push-constant 块)。
struct RoofPushConstants
{
    uint first_instance;
    uint instance_count;
    float wall_height_delta;
    float roof_pivot_y;
    float animation_t;
};

// 把两个半精度浮点打包进一个 uint(低位 .x,高位 .y)。
uint pack_half2x16(float2 value)
{
    uint2 packed = f32tof16(value);
    return packed.x | (packed.y << 16);
}

// 把一个 half2x16 uint 解包成两个浮点。
float2 unpack_half2x16(uint value)
{
    return f16tof32(uint2(value & 0xFFFFu, value >> 16));
}

// 四元数 (x,y,z,w) -> 3x4 旋转+平移矩阵(列向量)。
float3x4 quat_to_matrix(float4 q, float3 translation)
{
    float x = q.x;
    float y = q.y;
    float z = q.z;
    float w = q.w;
    float x2 = x + x;
    float y2 = y + y;
    float z2 = z + z;
    float xx = x * x2;
    float xy = x * y2;
    float xz = x * z2;
    float yy = y * y2;
    float yz = y * z2;
    float zz = z * z2;
    float wx = w * x2;
    float wy = w * y2;
    float wz = w * z2;
    return float3x4(
        float4(1.0f - (yy + zz), xy + wz, xz - wy, translation.x),
        float4(xy - wz, 1.0f - (xx + zz), yz + wx, translation.y),
        float4(xz + wy, yz - wx, 1.0f - (xx + yy), translation.z));
}

#endif // TINYGLADE_ROOF_ROOF_TILE_HLSL
