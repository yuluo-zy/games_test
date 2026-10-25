// ==RECONSTRUCTED== shaders/lut/brdf_fg.cs.hlsl | F-D
// 中文说明:
//   GGX BRDF 的 F+G 分割求和 LUT 生成器。生成 split-sum 积分查找表:
//   对 (NdotV, alpha=roughness^2) 网格上的每点,用 1024 个 Hammersley 样本做
//   Monte-Carlo 积分,输出 (F 积分项, G 积分项)。本文件为纯转译(spirv-cross),
//   变量名已语义化(NdotV/alpha/importance 采样方向/反射方向等),算术与控制流
//   与原转译版逐字一致。
#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// 转译自 _lut_brdf_fg.cs.hlsl_cs_main_0.bin(spirv-cross;行为精确)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
RWTexture2D<float2> output_tex : register(u14, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    // 像素 x -> NdotV(y 轴为 alpha=roughness^2),带 0.001 偏置避免退化的余弦角。
    float v_raw = float(gl_GlobalInvocationID.x) * 0.01585714332759380340576171875f;
    float ndv = v_raw + 0.001000000047497451305389404296875f;
    float alpha = max(9.9999997473787516355514526367188e-06f, float(gl_GlobalInvocationID.y) * 0.01587301678955554962158203125f);
    float ndv2 = ndv * ndv;
    float sin2 = 1.0f - ndv2;
    float sin_v = sqrt(sin2);
    // 视线方向 V = (sinθv, 0, cosθv)。
    float3 view_dir = float3(sin_v, 0.0f, ndv);
    float sum_g;
    float sum_f;
    sum_g = 0.0f;
    sum_f = 0.0f;
    uint i1;
    float next_g;
    float next_f;
    // 1024 个 Hammersley 样本的 Monte-Carlo 积分。
    for (uint i = 0u; i < 1024u; sum_g = next_g, sum_f = next_f, i = i1)
    {
        i1 = i + 1u;
        // Van der Corput 基数逆序(位反转)序列。
        uint bit_rev = (i1 << 16u) | (i1 >> 16u);
        uint sw1 = ((bit_rev & 1431655765u) << 1u) | ((bit_rev & 2863311530u) >> 1u);
        uint sw2 = ((sw1 & 858993459u) << 2u) | ((sw1 & 3435973836u) >> 2u);
        uint sw3 = ((sw2 & 252645135u) << 4u) | ((sw2 & 4042322160u) >> 4u);
        float3 f_term;
        float3 light_out;
        do
        {
            // GGX 重要性采样:由 alpha 拉伸的视线得到半向量 H。
            float3 half_vec = normalize(float3(alpha * sin_v, 0.0f, ndv));
            float half_z = half_vec.z;
            float3 tangent;
            if (half_z < 0.99989998340606689453125f)
            {
                tangent = normalize(cross(float3(0.0f, 0.0f, 1.0f), half_vec));
            }
            else
            {
                tangent = float3(1.0f, 0.0f, 0.0f);
            }
            // 球坐标半径/方位角(Hammersley 低差异)。
            float radius = sqrt(float(i1) * 0.0009765625f);
            float phi = float(((sw3 & 16711935u) << 8u) | ((sw3 & 4278255360u) >> 8u)) * 1.4629181199765639576071407645941e-09f;
            float cos_r = radius * cos(phi);
            float k = 0.5f * (1.0f + half_z);
            float one_minus_cos2 = 1.0f - (cos_r * cos_r);
            float sin_r = ((1.0f - k) * sqrt(one_minus_cos2)) + (k * (radius * sin(phi)));
            float3 sample_dir = ((tangent * cos_r) + (cross(half_vec, tangent) * sin_r)) + (half_vec * sqrt(max(0.0f, one_minus_cos2 - (sin_r * sin_r))));
            // 重要性采样后的微平面法线 H。
            float3 h_ndf = normalize(float3(alpha * sample_dir.x, alpha * sample_dir.y, max(0.0f, sample_dir.z)));
            // 反射视线得到出射光方向 L。
            float3 light_dir = reflect(-view_dir, h_ndf);
            bool flag_bad_h;
            if (!(h_ndf.z <= 9.9999997473787516355514526367188e-06f))
            {
                flag_bad_h = light_dir.z <= 9.9999997473787516355514526367188e-06f;
            }
            else
            {
                flag_bad_h = true;
            }
            bool flag_bad_ndv;
            if (!flag_bad_h)
            {
                flag_bad_ndv = ndv <= 9.9999997473787516355514526367188e-06f;
            }
            else
            {
                flag_bad_ndv = true;
            }
            // 地平线以下/退化角:贡献记 0。
            if (flag_bad_ndv)
            {
                f_term = 0.0f.xxx;
                light_out = float3(0.0f, 0.0f, -1.0f);
                break;
            }
            float ndh = max(0.0f, 1.0f - dot(h_ndf, light_dir));
            float ndh2 = ndh * ndh;
            float alpha2 = alpha * alpha;
            float light_z = light_dir.z;
            // F(菲涅尔)项 * G(几何)项 integrand。
            f_term = lerp(1.0f.xxx, 1.0f.xxx, ((ndh2 * ndh2) * ndh).xxx) * ((((2.0f * light_z) * ndv) / ((light_z * sqrt((((((-0.001000000047497451305389404296875f) - v_raw) * alpha2) + ndv) * ndv) + alpha2)) + (ndv * sqrt(((((-light_z) * alpha2) + light_z) * light_z) + alpha2)))) / (2.0f / (1.0f + sqrt(1.0f + (alpha2 * (sin2 / ndv2))))));
            light_out = light_dir;
            break;
        } while(false);
        // 出射方向有效时,累加 G(几何)与 F-G 差值(F 项)。
        if (light_out.z > 9.9999999747524270787835121154785e-07f)
        {
            float3 g_term;
            do
            {
                bool flag_low;
                if (!(light_out.z <= 0.0f))
                {
                    flag_low = ndv <= 0.0f;
                }
                else
                {
                    flag_low = true;
                }
                if (flag_low)
                {
                    g_term = 0.0f.xxx;
                    break;
                }
                float alpha2b = alpha * alpha;
                float ndh2b = max(0.0f, 1.0f - dot(normalize(view_dir + light_out), light_out));
                float ndh2b2 = ndh2b * ndh2b;
                g_term = lerp(0.0f.xxx, 1.0f.xxx, ((ndh2b2 * ndh2b2) * ndh2b).xxx) * ((((2.0f * light_out.z) * ndv) / ((light_out.z * sqrt((((((-0.001000000047497451305389404296875f) - v_raw) * alpha2b) + ndv) * ndv) + alpha2b)) + (ndv * sqrt(((((-light_out.z) * alpha2b) + light_out.z) * light_out.z) + alpha2b)))) / (2.0f / (1.0f + sqrt(1.0f + (alpha2b * (sin2 / ndv2))))));
                break;
            } while(false);
            next_g = sum_g + g_term.x;
            next_f = sum_f + (f_term.x - g_term.x);
        }
        else
        {
            next_g = sum_g;
            next_f = sum_f;
        }
    }
    // 除以样本数得到均值:输出 (F, G)。
    output_tex[gl_GlobalInvocationID.xy] = float2(sum_f, sum_g) * 0.0009765625f.xx;
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
