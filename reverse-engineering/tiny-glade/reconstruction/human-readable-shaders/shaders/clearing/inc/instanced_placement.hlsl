// ==RECONSTRUCTED== shaders/clearing/inc/instanced_placement.hlsl
// 【文件用途】实例化放置辅助。内嵌于 10 个 bin(instanced_background_tree、meadow_flower、
//   meadow_plant、tree_canopy、tree_trunk x2 变体)。提供 48 字节的 PlacementData 记录
//   (position float3 @0、scale @12、rotation_angle @16、id @20、custom0/1、anim_t_packed、
//   bend_vector @36、pad0)以及背景树 / 草地植物 / 树冠 / 树干顶点着色器使用的放置变换。
//   每个消费者在自己的 set0 绑定上声明各自的字节地址缓冲(placement_buffer / nani_instance_data)
//   并内联读取记录字段;struct 与变换辅助函数位于此处。
// 【状态】O(struct 布局逐字取自 OpMemberName + 字节访问);I(辅助函数签名——变换在使用点内联)。
// 本头文件重写只添加中文注释,代码逐字保留。

#ifndef TINYGLADE_CLEARING_INC_INSTANCED_PLACEMENT_HLSL
#define TINYGLADE_CLEARING_INC_INSTANCED_PLACEMENT_HLSL

// 每实例放置记录,共 48 字节。
struct PlacementData
{
    float3 position;        // 位置 @0
    float scale;            // 均匀缩放 @12
    float rotation_angle;   // 绕 Y 轴旋转角(弧度)@16
    int id;                 // 实例 id @20
    uint custom0;           // 自定义数据 0 @24
    uint custom1;           // 自定义数据 1 @28
    uint anim_t_packed;     // 动画 t(半精度打包)@32
    float2 bend_vector;     // 弯曲向量 @36
    int pad0;               // 填充 @44
};

// 放置变换:T(position) * R(rotation_angle) * S(scale),与内联使用点一致。
// `placement` 是消费者的字节地址缓冲,`index` 是记录索引(48 字节步长)。
float4 placement_world(ByteAddressBuffer placement, uint index, float3 vertex_pos)
{
    // 读取记录:位置 @0、缩放 @12、旋转角 @16
    float3 pos = placement.Load<float3>(index * 48u + 0);
    float scale = placement.Load<float>(index * 48u + 12);
    float rot = radians(placement.Load<float>(index * 48u + 16));
    // 由旋转角构造前向向量与旋转基(forward、up=+Y、side)
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
    // 组合 T * R * S * vertex_pos
    return mul(translate_m, mul(basis, mul(scale_m, float4(vertex_pos, 1.0f))));
}

#endif // TINYGLADE_CLEARING_INC_INSTANCED_PLACEMENT_HLSL
