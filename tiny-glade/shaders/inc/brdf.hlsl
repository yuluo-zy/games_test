// ============================================================
// inc/brdf.hlsl —— 双向反射分布函数(BRDF)核心
// 用途:延迟光照中计算 GGX 高光 + 能量守恒漫反射。
//   消费方:_deferred_*_light_sun_sky 等。
// 入口:无(纯头文件)
// 关键算法:GGX D、Hammon 高度相关 G、Schlick F、GGX 漫反射
// ============================================================
// ==RECONSTRUCTED== shaders/inc/brdf.hlsl | evidence: spirv-cross (_deferred_solid_light_sun_sky PS) exact opcodes — GGX D=alphaSq/(PI*(NdotH^2*(alphaSq-1)+1)^2) with alphaSq=roughness^4, Hammon correlated G=(2*NdotL*NdotV)/(NdotL*sqrt((1-a)*NdotV^2+a)+NdotV*sqrt((1-a)*NdotL^2+a)), Schlick F=lerp(0.04,1,pow(1-HdotL,5)), diffuse=(1+(2*r^2*NdotH^2-0.5)*(1-NdotL)^5)*(1+...NdotV...)/PI | status: O (function names Inference: inlined)

#ifndef TINYGLADE_INC_BRDF_HLSL
#define TINYGLADE_INC_BRDF_HLSL

// GGX 法线分布项。alphaSq 为平方后的粗糙度 α(roughness∈[0,1] 时即 roughness^4),
//   与观察到的 D 项一致。
float distribution_ggx(float NdotH, float alphaSq)
{
    float d = NdotH * NdotH * (alphaSq - 1.0) + 1.0;
    return alphaSq / (3.14159265358979323846 * d * d);
}

// Smith 可见性项(Hammon 高度相关近似)。
float visibility_smith_ggx(float NdotL, float NdotV, float alphaSq)
{
    float l2 = NdotL * NdotL;
    float v2 = NdotV * NdotV;
    return (2.0 * NdotL * NdotV) /
        (NdotL * sqrt((1.0 - alphaSq) * v2 + alphaSq) +
         NdotV * sqrt((1.0 - alphaSq) * l2 + alphaSq));
}

// Schlick 菲涅尔项。
float3 fresnel_schlick(float cosTheta, float3 F0)
{
    float c = 1.0 - saturate(cosTheta);
    return F0 + (1.0 - F0) * (c * c * c * c * c);
}

// 在延迟 pass 中观察到的 GGX 漫反射项:用 NdotH 与 roughness 做
//   菲涅尔加权能量守恒漫反射。
float diffuse_ggx(float NdotL, float NdotV, float NdotH, float roughness)
{
    float r2 = roughness * roughness;
    float f = (2.0 * r2 * NdotH * NdotH) - 0.5;
    float omL = 1.0 - saturate(NdotL);
    float omV = 1.0 - saturate(NdotV);
    return ((1.0 + f * pow(omL, 5.0)) * (1.0 + f * pow(omV, 5.0))) * (1.0 / 3.14159265358979323846);
}

// 组合的 Cook-Torrance 高光 BRDF(电介质 F0 = 0.04)。
float3 cook_torrance_specular(float NdotL, float NdotV, float NdotH, float HdotL, float roughness)
{
    float alphaSq = roughness * roughness;
    alphaSq = alphaSq * alphaSq; // roughness^4
    float3 F = fresnel_schlick(HdotL, float3(0.04, 0.04, 0.04));
    float D = distribution_ggx(NdotH, alphaSq);
    float G = visibility_smith_ggx(NdotL, NdotV, alphaSq);
    return (F * D * G) / max(4.0 * NdotV * NdotL, 1e-6);
}

// 完整直接光 BRDF:GGX 高光 + GGX 漫反射 × 反照率。
float3 evaluate_brdf(float3 albedo, float roughness, float3 N, float3 V, float3 L)
{
    float NdotL = saturate(dot(N, L));
    float NdotV = saturate(dot(N, V));
    float3 H = normalize(L + V);
    float NdotH = saturate(dot(N, H));
    float HdotL = saturate(dot(H, L));
    float3 diffuse = albedo * diffuse_ggx(NdotL, NdotV, NdotH, roughness);
    float3 specular = cook_torrance_specular(NdotL, NdotV, NdotH, HdotL, roughness);
    return diffuse + specular;
}

#endif // TINYGLADE_INC_BRDF_HLSL
