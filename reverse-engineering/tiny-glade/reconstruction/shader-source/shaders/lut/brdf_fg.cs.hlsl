// ==RECONSTRUCTED== shaders/lut/brdf_fg.cs.hlsl | F-D
// Evidence: _lut_brdf_fg.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: output_tex u14 (float2), LocalSize [8,8,1], entry cs_main.
//   spirv-cross: split-sum BRDF LUT (Karis). x = NdotV in (0,1) small-offset, y =
//   roughness; 1024 Hammersley samples of the GGX importance-sampled half-vector;
//   accumulates Fresnel (F) and Geometry (G) split-sum terms; output = (F,G)/1024.
//   Uses inc/hammersley.hlsl radicalInverse_VdC for the sample rotation.
// Status: O (structure from disassembly; hammersley via inc layer).

#include "inc/common.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/math.hlsl"
#include "inc/math_const.hlsl"
#include "inc/brdf.hlsl"

RWTexture2D<float2> output_tex : register(u14, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    float NdotV = float(dispatchThreadID.x) * 0.01585714332759380340576171875f + 0.001000000047497451305389404296875f;
    float roughness = max(9.9999997473787516355514526367188e-06f, float(dispatchThreadID.y) * 0.01587301678955554962158203125f);

    float Vx = sqrt(1.0f - NdotV * NdotV);
    float3 V = float3(Vx, 0.0f, NdotV);

    float F = 0.0f;
    float G = 0.0f;

    for (uint i = 0u; i < 1024u; ++i)
    {
        uint sample_index = i + 1u;

        float3 H = normalize(float3(roughness * Vx, 0.0f, NdotV));
        float3 T;
        if (H.z < 0.99989998340606689453125f)
        {
            T = normalize(cross(float3(0.0f, 0.0f, 1.0f), H));
        }
        else
        {
            T = float3(1.0f, 0.0f, 0.0f);
        }
        float3 B = cross(H, T);

        float u1 = sqrt(float(sample_index) * 0.0009765625f);
        float phi = radicalInverse_VdC(sample_index) * 6.283185482025146484375f;

        float r1 = u1 * cos(phi);
        float r1Sq = 1.0f - r1 * r1;
        float HdotV = 0.5f * (1.0f + H.z);
        float HdotV2 = ((1.0f - HdotV) * sqrt(r1Sq)) + (HdotV * (u1 * sin(phi)));

        float3 L = (T * r1) + (B * HdotV2) + (H * sqrt(max(0.0f, r1Sq - HdotV2 * HdotV2)));
        float3 N = normalize(float3(roughness * L.x, roughness * L.y, max(0.0f, L.z)));
        float3 R = reflect(-V, N);

        bool invalid = (N.z <= 9.9999997473787516355514526367188e-06f) || (R.z <= 9.9999997473787516355514526367188e-06f) || (NdotV <= 9.9999997473787516355514526367188e-06f);
        if (invalid)
        {
            continue;
        }

        float r2 = roughness * roughness;
        float Rz = R.z;
        float NdotR = max(0.0f, 1.0f - dot(N, R));
        float NdotR2 = NdotR * NdotR;

        float denom = (Rz * sqrt((((-0.001000000047497451305389404296875f - NdotV) * r2 + NdotV) * NdotV + r2)) + (NdotV * sqrt(((-Rz * r2 + Rz) * Rz + r2))));
        float norm = 2.0f / (1.0f + sqrt(1.0f + r2 * ((1.0f - NdotV * NdotV) / (NdotV * NdotV))));
        float vis = ((2.0f * Rz * NdotV) / denom) / norm;

        F += vis;

        float3 H2 = normalize(V + R);
        float NdotH2 = max(0.0f, 1.0f - dot(H2, R));
        float NdotH22 = NdotH2 * NdotH2;
        float denom2 = (Rz * sqrt((((-0.001000000047497451305389404296875f - NdotV) * r2 + NdotV) * NdotV + r2)) + (NdotV * sqrt(((-Rz * r2 + Rz) * Rz + r2))));
        G += ((2.0f * Rz * NdotV) / denom2) / norm;
    }

    output_tex[dispatchThreadID.xy] = float2(F, G) * 0.0009765625f;
}
