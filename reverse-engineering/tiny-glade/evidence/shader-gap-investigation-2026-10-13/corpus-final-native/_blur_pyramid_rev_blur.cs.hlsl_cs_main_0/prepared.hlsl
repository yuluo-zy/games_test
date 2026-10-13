#line 1 "shaders/blur_pyramid/rev_blur.cs.hlsl"










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
    float3 tail = input_tail_tex[dispatchThreadID.xy].xyz;

    float3 acc = 0.0f;
    for (int j = -1; j <= 1; ++j)
    {
        for (int i = -1; i <= 1; ++i)
        {
            float2 uv = (float2(dispatchThreadID.xy) + 0.5f + float2(float(i), float(j))) / float2(output_extent);
            acc += input_tex.SampleLevel(sampler_lnc, uv, 0.0f).xyz;
        }
    }

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
