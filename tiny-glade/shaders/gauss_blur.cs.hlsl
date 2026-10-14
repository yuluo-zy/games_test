// 高斯模糊计算着色器(可分离,单方向)。
// 变体 b9dd0465835cd591 / dc5fd7e4b9bf9935 来自 _gauss_blur.cs.hlsl_cs_main_<hash>.bin
// (spirv-cross 提升,行为与原版一致;本文件仅做变量语义化与中文注释,代码不变)。
//
// 算法:对每个输出像素,在 is_horizontal 指定方向上以 sigma 计算高斯半径
// radius = ceil(sigma * 2),遍历 [-radius, radius] 的偏移,按高斯权重
// w(t) = exp(-t^2 / (2*sigma^2)) 累加邻域颜色,最后除以权重和得到模糊结果。
// 越界采样点以固定颜色 0.09433962404727935791015625 (= 24/255) 参与累加,
// 等价于把图像边界外的像素视为该常量色。两个变体仅输出格式不同:
// b9dd 输出 RGBA(float4),dc5f 输出单通道 (unorm float)。

#if defined(TGV_b9dd0465835cd591) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// 变体 b9dd:RGBA 输出。lifted from _gauss_blur.cs.hlsl_cs_main_b9dd0465835cd591.bin
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
cbuffer $Globals : register(b13, space0)
{
    uint2 img_in_dims : packoffset(c0);   // 输入尺寸
    uint2 img_out_dims : packoffset(c0.z); // 输出尺寸
    uint is_horizontal : packoffset(c1);   // 1=水平模糊,0=垂直模糊
    float sigma : packoffset(c1.y);        // 高斯核标准差
};

Texture2D<float4> img_in : register(t14, space0);
RWTexture2D<float4> img_out : register(u15, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    // 高斯半径:sigma*2 向上取整,遍历窗口为 [-radius, radius]。
    int radius = int(ceil(sigma * 2.0f));
    int neg_radius = -radius;
    float4 color_acc;   // 颜色加权累加器
    float weight_acc;   // 权重累加器
    color_acc = 0.0f.xxxx;
    weight_acc = 0.0f;
    float4 next_color;  // 循环迭代后的新累加值
    float next_weight;
    for (int i = neg_radius; i <= radius; color_acc = next_color, weight_acc = next_weight, i++)
    {
        int2 sample_px;  // 采样像素坐标(本像素 + 偏移)
        if (!(is_horizontal != 0u))
        {
            sample_px = int3(gl_GlobalInvocationID).xy + int2(0, i);
        }
        else
        {
            sample_px = int3(gl_GlobalInvocationID).xy + int2(i, 0);
        }
        uint2 sample_uv = uint2(sample_px);
        float4 sample = img_in.Load(int3(sample_uv, 0u));
        // 越界判定:坐标小于 0 或 >= 输入尺寸。
        bool out_of_bounds;
        if (!any(bool2(sample_px.x < int2(0, 0).x, sample_px.y < int2(0, 0).y)))
        {
            out_of_bounds = any(bool2(sample_uv.x >= img_in_dims.x, sample_uv.y >= img_in_dims.y));
        }
        else
        {
            out_of_bounds = true;
        }
        float4 clamped_sample;  // 越界时强制为边界常量色
        if (out_of_bounds)
        {
            float4 clamped = sample;
            clamped.x = 0.09433962404727935791015625f;
            clamped_sample = clamped;
        }
        else
        {
            clamped_sample = sample;
        }
        float i_f = float(i);
        float weight = exp(((-i_f) * i_f) / ((2.0f * sigma) * sigma));
        next_color = color_acc + (clamped_sample * weight);
        next_weight = weight_acc + weight;
    }
    img_out[uint2(int3(gl_GlobalInvocationID).xy)] = color_acc / weight_acc.xxxx;
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_dc5fd7e4b9bf9935) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// 变体 dc5f:单通道输出。lifted from _gauss_blur.cs.hlsl_cs_main_dc5fd7e4b9bf9935.bin
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
cbuffer $Globals : register(b13, space0)
{
    uint2 img_in_dims : packoffset(c0);
    uint2 img_out_dims : packoffset(c0.z);
    uint is_horizontal : packoffset(c1);
    float sigma : packoffset(c1.y);
};

Texture2D<float4> img_in : register(t14, space0);
RWTexture2D<unorm float> img_out : register(u15, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    // 高斯半径:sigma*2 向上取整。
    int radius = int(ceil(sigma * 2.0f));
    int neg_radius = -radius;
    float color_acc;   // 颜色(红通道)加权累加器
    float weight_acc;  // 权重累加器
    color_acc = 0.0f;
    weight_acc = 0.0f;
    float next_color;  // 循环迭代后的新累加值
    float next_weight;
    for (int i = neg_radius; i <= radius; color_acc = next_color, weight_acc = next_weight, i++)
    {
        int2 sample_px;  // 采样像素坐标
        if (!(is_horizontal != 0u))
        {
            sample_px = int3(gl_GlobalInvocationID).xy + int2(0, i);
        }
        else
        {
            sample_px = int3(gl_GlobalInvocationID).xy + int2(i, 0);
        }
        uint2 sample_uv = uint2(sample_px);
        // 越界判定;越界处内联使用边界常量色 0.09433962404727935791015625f。
        bool out_of_bounds;
        if (!any(bool2(sample_px.x < int2(0, 0).x, sample_px.y < int2(0, 0).y)))
        {
            out_of_bounds = any(bool2(sample_uv.x >= img_in_dims.x, sample_uv.y >= img_in_dims.y));
        }
        else
        {
            out_of_bounds = true;
        }
        float i_f = float(i);
        float weight = exp(((-i_f) * i_f) / ((2.0f * sigma) * sigma));
        next_color = color_acc + ((out_of_bounds ? 0.09433962404727935791015625f : img_in.Load(int3(sample_uv, 0u)).x) * weight);
        next_weight = weight_acc + weight;
    }
    img_out[uint2(int3(gl_GlobalInvocationID).xy)] = color_acc / weight_acc;
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
