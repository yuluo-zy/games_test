// ==RECONSTRUCTED== shaders/blur_pyramid/rev_blur.cs.hlsl | F-D
// 依据:_blur_pyramid_rev_blur.cs.hlsl_cs_main_0.bin(SPIR-V,偏移 23)。
//   绑定:$Globals b16 (output_extent/sample_self), sampler_lnc s3, input_tail_tex t0,
//   input_tex t1, output_tex u2 (float3),LocalSize [8,8,1],入口 cs_main。
//   spirv-cross:在归一化像素中心对 input_tex 做 3x3 SampleLevel 盒滤波,
//   sample_self 时 lerp(tail*1/9, tail, 0.3),否则直接取 tail。
//   注意:此处 sampler_lnc 绑定在 s3(inc/samplers.hlsl 里声明在 s0 —— 共享层分歧;
//   本着色器在观察到的槽位自行声明采样器)。
// 状态:O (结构取自反汇编;成员名逐字)。

cbuffer $Globals : register(b16, space0)
{
    uint2 output_extent;
    uint sample_self;
};

SamplerState sampler_lnc : register(s3, space0);
Texture2D<float4> input_tail_tex : register(t0, space0);
Texture2D<float4> input_tex : register(t1, space0);
RWTexture2D<float3> output_tex : register(u2, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    // 上一级(尾)结果,作为本级的默认输出或混合基准。
    float3 tail = input_tail_tex[dispatchThreadID.xy].xyz;

    // 3x3 邻域盒滤波累加(像素中心 + 偏移归一化到 [0,1])。
    float3 acc = 0.0f;
    for (int j = -1; j <= 1; ++j)
    {
        for (int i = -1; i <= 1; ++i)
        {
            float2 uv = (float2(dispatchThreadID.xy) + 0.5f + float2(float(i), float(j))) / float2(output_extent);
            acc += input_tex.SampleLevel(sampler_lnc, uv, 0.0f).xyz;
        }
    }

    // sample_self 时按 30% 混合滤波均值(均值 = 累加*1/9),否则沿用 tail。
    float3 result;
    if (sample_self != 0u)
    {
        result = lerp(acc * 0.111111111938953399658203125f.xxx, tail, 0.300000011920928955078125f.xxx);
    }
    else
    {
        result = tail;
    }
    output_tex[dispatchThreadID.xy] = result;
}
