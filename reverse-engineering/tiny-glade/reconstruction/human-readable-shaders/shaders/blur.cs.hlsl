// ==RECONSTRUCTED== shaders/blur.cs.hlsl | F-D
// Evidence: _blur.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: $Globals b13 (img_in_dims/img_out_dims), img_in t14, img_out u15,
//   LocalSize [8,8,1], entry cs_main.
//   spirv-cross: radial blur, 30 angular steps x 10 radial taps, output red channel
//   scaled by 1/285 (0.00350877). dims = float2(int2(img_in_dims)); uv = gid/dims.
// Status: O (structure from disassembly).
//
// === 中文说明 ===
// 径向模糊(radial blur):以像素为中心,沿 30 个角度方向、每个方向 10 个径向台阶
//   采样叠加,得到沿径向拖尾的模糊效果。入口 cs_main,工作组 8x8x1,无 TGV_ 变体。
// 绑定: img_in t14 输入纹理, img_out u15 输出 UAV, $Globals b13 输入/输出尺寸。

#include "inc/common.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 img_in_dims;
    uint2 img_out_dims;
};

Texture2D<float4> img_in : register(t14, space0);
RWTexture2D<float4> img_out : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    int2 dims_i = int2(img_in_dims);
    float2 dims = float2(dims_i);
    float dx = float(dims_i.x);
    float dy = float(dims_i.y);

    // 归一化采样坐标:线程位置 / 图像尺寸。
    float2 uv = float2(float(int3(dispatchThreadID).x) / dx, float(int3(dispatchThreadID).y) / dy);
    uint2 pixel = uint2(int3(dispatchThreadID).xy);

    // 径向模糊:以中心像素为起点,沿 30 个角度、每个角度 10 个径向台阶迭代累加。
    float4 acc = img_in[pixel];
    for (float angle = 0.0f; angle < 6.283185482025146484375f; angle += 0.2094395160675048828125f)
    {
        float4 sum = acc;
        for (float t = 0.100000001490116119384765625f; t <= 1.0f; t += 0.100000001490116119384765625f)
        {
            // 采样方向 (cos, sin),半径 7 个纹素,按台阶 t 线性外推。
            float2 sample_uv = uv + float2(cos(angle), sin(angle)) * (7.0f / dims) * t;
            sum += img_in[uint2(int2(int(sample_uv.x * dx), int(sample_uv.y * dy)))];
        }
        acc = sum;
    }

    // 累积结果乘 1/285(0.00350877)归一化,输出 R 通道。
    img_out[pixel] = float4((acc * 0.00350877200253307819366455078125f.xxxx).xxx, 1.0f);
}
