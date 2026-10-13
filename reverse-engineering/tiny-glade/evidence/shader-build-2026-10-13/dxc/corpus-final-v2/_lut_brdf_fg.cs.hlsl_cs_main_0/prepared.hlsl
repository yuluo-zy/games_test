#line 1 "shaders/lut/brdf_fg.cs.hlsl"









#line 1 "shaders/inc/common.hlsl"





#ifndef TINYGLADE_INC_COMMON_HLSL
#define TINYGLADE_INC_COMMON_HLSL

#line 1 "shaders/inc/samplers.hlsl"







#ifndef TINYGLADE_INC_SAMPLERS_HLSL
#define TINYGLADE_INC_SAMPLERS_HLSL

#line 1 "shaders/inc/debug_utils.hlsl"





#ifndef TINYGLADE_INC_DEBUG_UTILS_HLSL
#define TINYGLADE_INC_DEBUG_UTILS_HLSL

#line 1 "shaders/inc/tracked_buffer.hlsl"






#ifndef TINYGLADE_INC_TRACKED_BUFFER_HLSL
#define TINYGLADE_INC_TRACKED_BUFFER_HLSL




uint tracked_buffer_reserve(RWByteAddressBuffer counter, uint counter_offset)
{
    uint previous;
    counter.InterlockedAdd(counter_offset, 1u, previous);
    return previous;
}


void tracked_buffer_store_float(RWByteAddressBuffer buffer, uint base_byte_offset, uint element_index, float value)
{
    buffer.Store<float>(base_byte_offset + element_index * 4u, value);
}

#endif 
#line 9 "shaders/inc/debug_utils.hlsl"



float3 debug_heatmap(float v)
{
    float t = clamp(v, 0.0f, 1.0f);
    return lerp(float3(0.0f, 0.0f, 1.0f), float3(1.0f, 0.0f, 0.0f), t);
}

#endif 
#line 11 "shaders/inc/samplers.hlsl"


SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_llc : register(s3, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_s5 : register(s5, space0);
SamplerState sampler_s6 : register(s6, space0);
SamplerState sampler_s7 : register(s7, space0);
SamplerState sampler_s8 : register(s8, space0);
SamplerState sampler_default : register(s9, space0);
SamplerState sampler_mipmap : register(s10, space0);
SamplerState sampler_shadow_nearest : register(s11, space0);
SamplerComparisonState sampler_shadow_linear : register(s12, space0);

#endif 
#line 9 "shaders/inc/common.hlsl"


#endif 
#line 10 "shaders/lut/brdf_fg.cs.hlsl"

#line 1 "shaders/inc/hammersley.hlsl"










float radicalInverse_VdC(uint bits)
{
    bits = (bits << 16u) | (bits >> 16u);
    bits = ((bits & 0x55555555u) << 1u) | ((bits & 0xAAAAAAAAu) >> 1u);
    bits = ((bits & 0x33333333u) << 2u) | ((bits & 0xCCCCCCCCu) >> 2u);
    bits = ((bits & 0x0F0F0F0Fu) << 4u) | ((bits & 0xF0F0F0F0u) >> 4u);
    bits = ((bits & 0x00FF00FFu) << 8u) | ((bits & 0xFF00FF00u) >> 8u);
    return float(bits) * 2.3283064365386963e-10f;
}

float2 hammersley(uint i, uint N)
{
    return float2(float(i) / float(N), radicalInverse_VdC(i));
}
#line 11 "shaders/lut/brdf_fg.cs.hlsl"

#line 1 "shaders/inc/math.hlsl"





#ifndef TINYGLADE_INC_MATH_HLSL
#define TINYGLADE_INC_MATH_HLSL


float clamp01(float v)
{
    return clamp(v, 0.0f, 1.0f);
}

float3 clamp01(float3 v)
{
    return clamp(v, 0.0f, 1.0f);
}


float length_squared(float3 v)
{
    return dot(v, v);
}


float3 safe_normalize(float3 v)
{
    float len_sq = dot(v, v);
    float len = sqrt(len_sq);
    return len > 1.0e-6f ? v / len : float3(0.0f, 0.0f, 0.0f);
}


float safe_rcp(float v)
{
    return 1.0f / max(abs(v), 1.0e-6f);
}


float rcp_keep_sign(float v)
{
    return sign(v) * safe_rcp(v);
}

#endif 
#line 12 "shaders/lut/brdf_fg.cs.hlsl"

#line 1 "shaders/inc/math_const.hlsl"





#ifndef TINYGLADE_INC_MATH_CONST_HLSL
#define TINYGLADE_INC_MATH_CONST_HLSL

static const float PI = 3.14159265358979323846264338327950288f;
static const float TWO_PI = 6.28318530717958647692528676655900577f;
static const float HALF_PI = 1.57079632679489661923132169163975144f;
static const float INV_PI = 0.31830988618379067153776752674502872f;
static const float INV_TWO_PI = 0.15915494309189533576888376337251436f;

static const float SQRT_TWO = 1.41421356237309504880168872420969808f;
static const float INV_SQRT_TWO = 0.70710678118654752440084436210484904f;

static const float GOLDEN_RATIO = 1.61803398874989484820458683436563812f;
static const float INV_GOLDEN_RATIO = 0.61803398874989484820458683436563812f;

static const float EPSILON = 1.0e-6f;
static const float FLT_EPSILON = 1.19209290e-7f;

#endif 
#line 13 "shaders/lut/brdf_fg.cs.hlsl"

#line 1 "shaders/inc/brdf.hlsl"


#ifndef TINYGLADE_INC_BRDF_HLSL
#define TINYGLADE_INC_BRDF_HLSL



float distribution_ggx(float NdotH, float alphaSq)
{
    float d = NdotH * NdotH * (alphaSq - 1.0) + 1.0;
    return alphaSq / (3.14159265358979323846 * d * d);
}


float visibility_smith_ggx(float NdotL, float NdotV, float alphaSq)
{
    float l2 = NdotL * NdotL;
    float v2 = NdotV * NdotV;
    return (2.0 * NdotL * NdotV) /
        (NdotL * sqrt((1.0 - alphaSq) * v2 + alphaSq) +
         NdotV * sqrt((1.0 - alphaSq) * l2 + alphaSq));
}


float3 fresnel_schlick(float cosTheta, float3 F0)
{
    float c = 1.0 - saturate(cosTheta);
    return F0 + (1.0 - F0) * (c * c * c * c * c);
}



float diffuse_ggx(float NdotL, float NdotV, float NdotH, float roughness)
{
    float r2 = roughness * roughness;
    float f = (2.0 * r2 * NdotH * NdotH) - 0.5;
    float omL = 1.0 - saturate(NdotL);
    float omV = 1.0 - saturate(NdotV);
    return ((1.0 + f * pow(omL, 5.0)) * (1.0 + f * pow(omV, 5.0))) * (1.0 / 3.14159265358979323846);
}


float3 cook_torrance_specular(float NdotL, float NdotV, float NdotH, float HdotL, float roughness)
{
    float alphaSq = roughness * roughness;
    alphaSq = alphaSq * alphaSq; 
    float3 F = fresnel_schlick(HdotL, float3(0.04, 0.04, 0.04));
    float D = distribution_ggx(NdotH, alphaSq);
    float G = visibility_smith_ggx(NdotL, NdotV, alphaSq);
    return (F * D * G) / max(4.0 * NdotV * NdotL, 1e-6);
}


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

#endif 
#line 14 "shaders/lut/brdf_fg.cs.hlsl"


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
