// ==RECONSTRUCTED== shaders/clearing/inc/canopy_common.inc.hlsl
// 【文件用途】树冠公共辅助函数。内嵌于 6 个 bin(tree_canopy、tree_trunk、deferred canopy light sun sky x2)。
//   提供:由放置记录 anim_t_packed 半精度低半(48 字节记录的第 32 字节)推出的生长年龄,
//   以及树冠/树干顶点着色器共用的树冠放置/旋转基。各消费者自行声明各自的
//   nani_instance_data / instance_indirection_buffer 字节缓冲。
// 【状态】O(放置记录布局 + 生长年龄映射取自使用点);I(辅助函数名)。
// 本头文件重写只添加中文注释,代码逐字保留。

#ifndef TINYGLADE_CLEARING_INC_CANOPY_COMMON_HLSL
#define TINYGLADE_CLEARING_INC_CANOPY_COMMON_HLSL

// 生长年龄 [0,1]:取放置记录 anim_t_packed 的低 16 位(字节 32),按半精度浮点解码。
float canopy_age(ByteAddressBuffer placement, uint index)
{
    return f16tof32(placement.Load<uint>(index * 48u + 32) & 0xFFFFu);
}

// 放置记录的弯曲向量(字节 36..43),用于树冠风摆动。
float2 canopy_bend(ByteAddressBuffer placement, uint index)
{
    return placement.Load<float2>(index * 48u + 36);
}

// 树冠/树干顶点着色器共用的放置变换 (T * R * S)。
float4 canopy_place(ByteAddressBuffer placement, uint index, float3 local_pos)
{
    // 读取记录:位置 @0、缩放 @12、绕 Y 轴旋转角 @16
    float3 pos = placement.Load<float3>(index * 48u + 0);
    float scale = placement.Load<float>(index * 48u + 12);
    float rot = radians(placement.Load<float>(index * 48u + 16));
    // 由旋转角构造前向向量,再构造旋转基(forward、up=+Y、side)
    float3 fwd = normalize(float3(cos(rot), 0.0f, sin(rot)));
    float4x4 basis = transpose(float4x4(float4(fwd, 0.0f),
                                        float4(0.0f, 1.0f, 0.0f, 0.0f),
                                        float4(normalize(cross(fwd, float3(0.0f, 1.0f, 0.0f))), 0.0f),
                                        float4(0.0f, 0.0f, 0.0f, 1.0f)));
    // 均匀缩放矩阵
    float4x4 scale_m = float4x4(float4(scale, 0.0f, 0.0f, 0.0f),
                                float4(0.0f, scale, 0.0f, 0.0f),
                                float4(0.0f, 0.0f, scale, 0.0f),
                                float4(0.0f, 0.0f, 0.0f, 1.0f));
    // 平移矩阵
    float4x4 translate_m = float4x4(float4(1.0f, 0.0f, 0.0f, pos.x),
                                    float4(0.0f, 1.0f, 0.0f, pos.y),
                                    float4(0.0f, 0.0f, 1.0f, pos.z),
                                    float4(0.0f, 0.0f, 0.0f, 1.0f));
    // 组合 T * R * S * local_pos
    return mul(translate_m, mul(basis, mul(scale_m, float4(local_pos, 1.0f))));
}

#endif // TINYGLADE_CLEARING_INC_CANOPY_COMMON_HLSL
