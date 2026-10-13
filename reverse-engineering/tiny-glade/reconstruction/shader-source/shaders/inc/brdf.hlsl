// ==RECONSTRUCTED== shaders/inc/brdf.hlsl | evidence: spirv-cross (_deferred_solid_light_sun_sky PS) exact opcodes — GGX D=alphaSq/(PI*(NdotH^2*(alphaSq-1)+1)^2) with alphaSq=roughness^4, Hammon correlated G=(2*NdotL*NdotV)/(NdotL*sqrt((1-a)*NdotV^2+a)+NdotV*sqrt((1-a)*NdotL^2+a)), Schlick F=lerp(0.04,1,pow(1-HdotL,5)), diffuse=(1+(2*r^2*NdotH^2-0.5)*(1-NdotL)^5)*(1+...NdotV...)/PI | status: O (function names Inference: inlined)

#ifndef TINYGLADE_INC_BRDF_HLSL
#define TINYGLADE_INC_BRDF_HLSL

// GGX normal distribution. alphaSq is the squared roughness-alpha
// (roughness^4 for roughness in [0,1]), matching the observed D term.
float distribution_ggx(float NdotH, float alphaSq)
{
    float d = NdotH * NdotH * (alphaSq - 1.0) + 1.0;
    return alphaSq / (3.14159265358979323846 * d * d);
}

// Smith visibility (height-correlated, Hammon's approximation).
float visibility_smith_ggx(float NdotL, float NdotV, float alphaSq)
{
    float l2 = NdotL * NdotL;
    float v2 = NdotV * NdotV;
    return (2.0 * NdotL * NdotV) /
        (NdotL * sqrt((1.0 - alphaSq) * v2 + alphaSq) +
         NdotV * sqrt((1.0 - alphaSq) * l2 + alphaSq));
}

// Schlick Fresnel.
float3 fresnel_schlick(float cosTheta, float3 F0)
{
    float c = 1.0 - saturate(cosTheta);
    return F0 + (1.0 - F0) * (c * c * c * c * c);
}

// GGX-diffuse term observed in the deferred pass: Fresnel-weighted energy
// preserving diffuse using NdotH and roughness.
float diffuse_ggx(float NdotL, float NdotV, float NdotH, float roughness)
{
    float r2 = roughness * roughness;
    float f = (2.0 * r2 * NdotH * NdotH) - 0.5;
    float omL = 1.0 - saturate(NdotL);
    float omV = 1.0 - saturate(NdotV);
    return ((1.0 + f * pow(omL, 5.0)) * (1.0 + f * pow(omV, 5.0))) * (1.0 / 3.14159265358979323846);
}

// Combined Cook-Torrance specular BRDF (dielectric F0 = 0.04).
float3 cook_torrance_specular(float NdotL, float NdotV, float NdotH, float HdotL, float roughness)
{
    float alphaSq = roughness * roughness;
    alphaSq = alphaSq * alphaSq; // roughness^4
    float3 F = fresnel_schlick(HdotL, float3(0.04, 0.04, 0.04));
    float D = distribution_ggx(NdotH, alphaSq);
    float G = visibility_smith_ggx(NdotL, NdotV, alphaSq);
    return (F * D * G) / max(4.0 * NdotV * NdotL, 1e-6);
}

// Full direct-light BRDF: GGX specular plus GGX-diffuse times albedo.
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
