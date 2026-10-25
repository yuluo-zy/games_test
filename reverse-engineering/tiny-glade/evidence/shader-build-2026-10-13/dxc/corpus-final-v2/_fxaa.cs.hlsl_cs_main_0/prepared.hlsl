#line 1 "shaders/fxaa.cs.hlsl"




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
#line 5 "shaders/fxaa.cs.hlsl"
            
#line 1 "shaders/inc/uv.hlsl"






#ifndef TINYGLADE_INC_UV_HLSL
#define TINYGLADE_INC_UV_HLSL


float2 pixel_to_uv(int2 pixel, float2 dims)
{
    return (float2(pixel) + 0.5f) / dims;
}




float2 uv_to_sample(float2 uv)
{
    return uv - 0.5f;
}


float2 sample_to_clip(float2 sample)
{
    return sample * float2(2.0f, -2.0f);
}



float3 sample_to_view(float4x4 sample_to_view_mat, float2 sample_uv, float depth)
{
    return mul(sample_to_view_mat, float4(sample_uv * float2(2.0f, -2.0f), depth, 1.0f)).xyz;
}


float2 view_to_sample(float4x4 view_to_sample_mat, float3 view_pos)
{
    return mul(view_to_sample_mat, float4(view_pos, 1.0f)).xy;
}


float2 view_to_clip(float4x4 view_to_clip_mat, float3 view_pos)
{
    float4 clip = mul(view_to_clip_mat, float4(view_pos, 1.0f));
    return clip.xy / clip.w;
}


float3 world_to_view(float4x4 world_to_view_mat, float3 world_pos)
{
    return mul(world_to_view_mat, float4(world_pos, 1.0f)).xyz;
}


float2 world_to_sample(float4x4 world_to_sample_mat, float3 world_pos)
{
    return mul(world_to_sample_mat, float4(world_pos, 1.0f)).xy;
}

#endif 
#line 6 "shaders/fxaa.cs.hlsl"

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
#line 7 "shaders/fxaa.cs.hlsl"

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
#line 8 "shaders/fxaa.cs.hlsl"

#line 1 "shaders/fxaa.inc.hlsl"




#ifndef TINYGLADE_FXAA_INC_HLSL
#define TINYGLADE_FXAA_INC_HLSL

static const float FXAA_EDGE_THRESHOLD = 0.166f;
static const float FXAA_SUBPIXEL = 0.75f;


float4 fxaa(Texture2D<float4> tex, SamplerState samp, float2 uv, float2 rcp, float4 color_center)
{
    float4 lumaNW = tex.GatherGreen(samp, uv, int2(-1, -1));
    float4 lumaNE = tex.GatherGreen(samp, uv);

    float lumaN = lumaNE.x;
    float lumaC = color_center.y;
    float lumaS = lumaNE.z;
    float lumaW = lumaNW.z;
    float lumaE = lumaNW.x;

    float lumaMax = max(max(lumaN, lumaE), max(lumaS, max(lumaC, lumaW)));
    float lumaMin = min(min(lumaN, lumaE), min(lumaS, min(lumaC, lumaW)));
    float lumaRange = lumaMax - lumaMin;

    if (lumaRange < max(0.0f, lumaMax * FXAA_EDGE_THRESHOLD))
        return color_center;

    float4 sampleB = tex.SampleLevel(samp, uv, 0.0f, int2(1, -1));
    float4 sampleC = tex.SampleLevel(samp, uv, 0.0f, int2(-1, 1));
    float lumaSW2 = lumaNW.z;
    float lumaSE2 = lumaNW.x;

    
    float lumaGradVert = (-2.0f * lumaC) + lumaSW2 + sampleC.y;
    float lumaGradHoriz = (-2.0f * lumaC) + lumaSE2 + sampleB.y;

    float gradVert = abs(lumaGradHoriz) + (abs(lumaGradVert + lumaSE2) * 2.0f) + abs(lumaGradHoriz + lumaNE.w);
    float gradHoriz = abs(lumaGradVert) + (abs(lumaGradHoriz + lumaSW2) * 2.0f) + abs(lumaGradVert + lumaNE.w);
    bool horizontal = gradHoriz >= gradVert;

    
    float2 dir;
    float lumaEdgeN;
    float lumaEdgeE;
    if (horizontal)
    {
        dir = float2(rcp.x, 0.0f);
        lumaEdgeN = lumaSE2;
        lumaEdgeE = lumaNE.x;
    }
    else
    {
        dir = float2(0.0f, rcp.y);
        lumaEdgeN = lumaSW2;
        lumaEdgeE = lumaNE.z;
    }

    
    float lumaAvg = (lumaN + lumaS + lumaE + lumaW + lumaC) * 0.083333336f;
    float subpixel = clamp(abs(lumaAvg - lumaC) / lumaRange, 0.0f, 1.0f);
    float subpixelTap = horizontal ? 0.0f : rcp.x;

    
    float2 pix = uv;
    if (horizontal)
        pix.y = uv.y + dir.y * 0.5f;
    else
        pix.x = uv.x + dir.x * 0.5f;

    float2 neg = pix - dir * subpixelTap;
    float2 pos = pix + dir * subpixelTap;

    float lumaTap = max(abs(lumaEdgeN - lumaC), abs(lumaEdgeE - lumaC)) * 0.25f;
    float lumaCenter = ((horizontal ? lumaEdgeE : lumaEdgeN) + lumaC) * 0.5f;
    float subpixelSmooth = ((-2.0f * subpixel) + 3.0f) * (subpixel * subpixel);
    bool lumaBelow = (lumaC - lumaCenter) < 0.0f;

    float lumaL = tex.SampleLevel(samp, neg, 0.0f).y - lumaCenter;
    float lumaR = tex.SampleLevel(samp, pos, 0.0f).y - lumaCenter;

    
    bool doneL = !(abs(lumaL) >= lumaTap);
    bool doneR = !(abs(lumaR) >= lumaTap);
    float2 negL = doneL ? neg - dir * (subpixelTap * 1.5f) : neg;
    float2 posR = doneR ? pos + dir * (subpixelTap * 1.5f) : pos;

    lumaL = doneL ? tex.SampleLevel(samp, negL, 0.0f).y - lumaCenter : lumaL;
    lumaR = doneR ? tex.SampleLevel(samp, posR, 0.0f).y - lumaCenter : lumaR;
    doneL = !(abs(lumaL) >= lumaTap);
    doneR = !(abs(lumaR) >= lumaTap);
    negL = doneL ? negL - dir * (subpixelTap * 2.0f) : negL;
    posR = doneR ? posR + dir * (subpixelTap * 2.0f) : posR;

    lumaL = doneL ? tex.SampleLevel(samp, negL, 0.0f).y - lumaCenter : lumaL;
    lumaR = doneR ? tex.SampleLevel(samp, posR, 0.0f).y - lumaCenter : lumaR;
    doneL = !(abs(lumaL) >= lumaTap);
    doneR = !(abs(lumaR) >= lumaTap);
    negL = doneL ? negL - dir * (subpixelTap * 4.0f) : negL;
    posR = doneR ? posR + dir * (subpixelTap * 4.0f) : posR;

    lumaL = doneL ? tex.SampleLevel(samp, negL, 0.0f).y - lumaCenter : lumaL;
    lumaR = doneR ? tex.SampleLevel(samp, posR, 0.0f).y - lumaCenter : lumaR;
    doneL = !(abs(lumaL) >= lumaTap);
    doneR = !(abs(lumaR) >= lumaTap);
    negL = doneL ? negL - dir * (subpixelTap * 12.0f) : negL;
    posR = doneR ? posR + dir * (subpixelTap * 12.0f) : posR;

    lumaL = doneL ? tex.SampleLevel(samp, negL, 0.0f).y - lumaCenter : lumaL;
    lumaR = doneR ? tex.SampleLevel(samp, posR, 0.0f).y - lumaCenter : lumaR;

    
    float spanL = horizontal ? pix.x - negL.x : pix.y - negL.y;
    float spanR = horizontal ? posR.x - pix.x : posR.y - pix.y;
    float spanMax = max(spanL, spanR);

    float blend = 0.0f;
    if (spanL < spanR)
    {
        if ((lumaL < 0.0f) != lumaBelow)
            blend = (min(spanL, spanR) * (-1.0f / (spanR + spanL))) + 0.5f;
    }
    else
    {
        if ((lumaR < 0.0f) != lumaBelow)
            blend = (min(spanL, spanR) * (-1.0f / (spanR + spanL))) + 0.5f;
    }
    blend = max(blend, subpixelSmooth * subpixelSmooth * FXAA_SUBPIXEL);

    float2 final_uv = uv;
    if (horizontal)
        final_uv.y = uv.y + blend * dir.y;
    else
        final_uv.x = uv.x + blend * dir.x;

    return float4(tex.SampleLevel(samp, final_uv, 0.0f).xyz, 0.0f);
}

#endif 
#line 9 "shaders/fxaa.cs.hlsl"


cbuffer $Globals : register(b13, space0)
{
    uint2 input_tex_dims;
    uint2 output_tex_dims;
};

Texture2D<float4> input_tex : register(t14, space0);
RWTexture2D<float3> output_tex : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_id : SV_DispatchThreadID)
{
    float2 dims = float2(input_tex_dims);
    float2 uv = (float2(int2(dispatch_id.xy)) + 0.5f) / dims;
    float2 rcp = 1.0f / dims;

    float4 center = input_tex.SampleLevel(sampler_lnc, uv, 0.0f);
    float4 result = fxaa(input_tex, sampler_lnc, uv, rcp, center);

    output_tex[dispatch_id.xy] = result.xyz;
}
