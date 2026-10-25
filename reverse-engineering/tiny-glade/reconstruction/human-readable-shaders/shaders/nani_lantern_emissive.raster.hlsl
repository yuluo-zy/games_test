// ==RECONSTRUCTED== shaders/nani_lantern_emissive.raster.hlsl | B nani raster | bins: _nani_lantern_emissive.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// 【文件用途】Nani 灯笼:VS 转发半打包的灯笼位置/强度 + 旗标;
//   PS 对半透明灯笼做抖动剔除、按调色板 id 给玻璃上色、打包 G-Buffer,
//   并向自发光目标(emissive target)输出一个点光源光晕。
// 【入口】vs_main / ps_main(无 TGV 变体差异;两个 bin 结构一致)。
// 【绑定】instance_indirection_buffer t15、nani_instance_data t16、
//   $Globals b14(bool_render_shadows)、view/frame 常量来自 inc/ 头文件。
// 重写说明:本文件为手写重建段,仅补充中文注释,代码与算术逐字保留。

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "nani/inc/user.hlsl"
#include "inc/perlin.hlsl"
#include "inc/lanterns.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"

// 是否渲染阴影(由 CPU 每帧设置;用于阴影通道剔除半透明灯笼)
int bool_render_shadows;

// 每实例记录:仿射变换 + 半打包的灯光局部位置/强度 + 旗标与调色板 id
struct InstanceData
{
    Affine3Packed xform;                 // 实例变换(打包的 3x4 仿射矩阵)
    uint2 light_position_ls_intensity;   // 灯光局部位置(半精度打包)+ 强度
    uint flags_color_id;                 // 低位 = 调色板 id,高位 = 旗标位
    uint pad0;
};

StructuredBuffer<uint> instance_indirection_buffer : register(t15);
StructuredBuffer<InstanceData> nani_instance_data : register(t16);

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 Vertex_Normal: Vertex_Normal;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;  // 顶点色(逐顶点)
    float3 C1 : TEXCOORD1;  // 世界空间法线
    float3 C2 : TEXCOORD2;  // 世界空间位置
    float3 C3 : TEXCOORD3;  // 世界空间灯光位置
    nointerpolation uint C4 : TEXCOORD4;  // 旗标 + 调色板 id
    nointerpolation float C5 : TEXCOORD5; // 灯光强度
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    // 经间接索引取得实例记录
    uint idx = instance_indirection_buffer[vin.instance_id];
    Affine3Packed packed = nani_instance_data[idx].xform;
    // 解包 3x4 仿射矩阵(行优先打包),转置回列优先矩阵
    float3x4 xform = transpose(float4x3(float3(packed.data[0], packed.data[1], packed.data[2]),
                                        float3(packed.data[3], packed.data[4], packed.data[5]),
                                        float3(packed.data[6], packed.data[7], packed.data[8]),
                                        float3(packed.data[9], packed.data[10], packed.data[11])));
    // 顶点位置变换到世界空间
    float3 world = mul(xform, float4(vin.Vertex_Position, 1.0f));
    // 解包灯光局部位置:两个 uint 各含两个半精度分量
    uint2 half_packed = nani_instance_data[idx].light_position_ls_intensity;
    float3 light_local = float3(f16tof32(half_packed.x & 0xFFFFu),
                                f16tof32((half_packed.x >> 16u) & 0xFFFFu),
                                f16tof32(half_packed.y & 0xFFFFu));
    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(world, 1.0f));
    o.C0 = vin.Vertex_Color;
    o.C1 = mul(xform, float4(vin.Vertex_Normal, 0.0f));
    o.C2 = world;
    o.C3 = mul(xform, float4(light_local, 1.0f));
    o.C4 = nani_instance_data[idx].flags_color_id;
    // 强度打包在 half_packed.y 的高半
    o.C5 = f16tof32((half_packed.y >> 16u) & 0xFFFFu);
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    nointerpolation uint C4 : TEXCOORD4;
    nointerpolation float C5 : TEXCOORD5;
    float4 position : SV_Position;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;     // 打包法线 + 反照率
    float3 emissive : SV_Target1;   // 灯光光晕
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    // 阴影通道下对半透明灯笼做亚像素抖动剔除(flag 位 31 + bool_render_shadows)
    bool dither = (pin.C4 & 2147483648u) != 0u && bool_render_shadows != 0;
    if (dither)
    {
        // 4 帧轮换的亚像素采样模式
        static const uint2 hi_px_subpixels[4] =
        {
            uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u)
        };
        uint2 sub = hi_px_subpixels[frame_index & 3u];
        uint2 cell = uint2(pin.position.xy) & uint2(1u, 1u);
        if (all(cell != sub))
        {
            discard;
        }
    }

    // 取调色板 id(低 16 位),选择灯笼玻璃颜色(归一化到 RGB)
    uint color_id = pin.C4 & 65535u;
    float3 color;
    if (color_id == 0u)
    {
        color = float3(0.79198f, 0.633584f, 0.079198f) / max(1e-10f, 0.79198f);
    }
    else if (color_id == 1u)
    {
        color = float3(0.0f, 0.158396f, 0.79198f) / max(1e-10f, 0.21524f);
    }
    else if (color_id == 2u)
    {
        color = float3(0.158396f, 0.79198f, 0.158396f) / max(1e-10f, 0.77216f);
    }
    else if (color_id == 3u)
    {
        color = float3(0.237594f, 0.158396f, 0.79198f) / max(1e-10f, 0.27902f);
    }
    else
    {
        color = float3(0.79198f, 0.237594f, 0.079198f) / max(1e-10f, 0.43438f);
    }

    // 打包法线与反照率到 G-Buffer(各 8 位)
    float3 packed_normal = (normalize(pin.C1) * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(pin.C0, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = uint(clamp(srgb_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(srgb_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(srgb_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u);
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 3422552064u;
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = o.gbuffer.y | 117440512u;  // 标记为半透明/特殊材质

    // 光晕:屏幕空间距离衰减的高斯型衰减(双指数,50 与 200 系数)
    float4 cam_vw = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_pos = cam_vw.xyz / cam_vw.w.xxx;
    float3 to_light = normalize(pin.C2 - cam_pos);
    // 灯位置相对视线方向的垂直分量 -> 距离的平方
    float3 perp = pin.C3 - (cam_pos + (to_light * dot(pin.C3 - cam_pos, to_light)));
    float dist2 = dot(perp, perp);
    float3 glow = (color * (1.4f * pin.C5)) * (3.0f * ((0.8f * exp2((-50.0f) * dist2)) + (2.0f * exp2((-200.0f) * dist2))));
    // 仅当 flag 位 30 置位时输出光晕
    o.emissive = (pin.C4 & 1073741824u) != 0u ? glow : 0.0f.xxx;
    return o;
}
