#line 1 "shaders/lut/water_noise.cs.hlsl"












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
#line 13 "shaders/lut/water_noise.cs.hlsl"

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
#line 14 "shaders/lut/water_noise.cs.hlsl"

#line 1 "shaders/inc/bicubic.hlsl"




#ifndef TINYGLADE_INC_BICUBIC_HLSL
#define TINYGLADE_INC_BICUBIC_HLSL




float4 bicubic_weights(float f)
{
    float t = f;
    float t_half = t * 0.5f;
    float w0 = t * (-0.5f + t * (1.0f - t_half));
    float w1 = t * (0.5f + t * (2.0f - t_half));
    float one_minus_t = 1.0f - t;
    float w3 = one_minus_t * (-0.5f + t_half);
    float w2 = (1.0f + one_minus_t * (-2.5f + t * (2.0f - t_half))) + w3;
    return float4(w0, w1, w2, w3);
}


float4 bicubic_sample(Texture2D<float4> tex, SamplerState samp, float2 uv, float2 texel_size)
{
    float2 p = uv * (float2(1.0f, 1.0f) / texel_size) - 0.5f;
    float2 f = frac(p);
    float2 base = (p - f) * texel_size;
    float4 wx = bicubic_weights(f.x);
    float4 wy = bicubic_weights(f.y);

    float4 c0 = wy.x * tex.SampleLevel(samp, base + float2(-1.0f, -1.0f) * texel_size, 0.0f);
    float4 c1 = wy.y * tex.SampleLevel(samp, base + float2( 0.0f, -1.0f) * texel_size, 0.0f);
    float4 c2 = wy.z * tex.SampleLevel(samp, base + float2( 1.0f, -1.0f) * texel_size, 0.0f);
    float4 c3 = wy.w * tex.SampleLevel(samp, base + float2( 2.0f, -1.0f) * texel_size, 0.0f);
    float4 c01 = wy.x * tex.SampleLevel(samp, base + float2(-1.0f,  0.0f) * texel_size, 0.0f);
    float4 c11 = wy.y * tex.SampleLevel(samp, base + float2( 0.0f,  0.0f) * texel_size, 0.0f);
    float4 c21 = wy.z * tex.SampleLevel(samp, base + float2( 1.0f,  0.0f) * texel_size, 0.0f);
    float4 c31 = wy.w * tex.SampleLevel(samp, base + float2( 2.0f,  0.0f) * texel_size, 0.0f);
    float4 c02 = wy.x * tex.SampleLevel(samp, base + float2(-1.0f,  1.0f) * texel_size, 0.0f);
    float4 c12 = wy.y * tex.SampleLevel(samp, base + float2( 0.0f,  1.0f) * texel_size, 0.0f);
    float4 c22 = wy.z * tex.SampleLevel(samp, base + float2( 1.0f,  1.0f) * texel_size, 0.0f);
    float4 c32 = wy.w * tex.SampleLevel(samp, base + float2( 2.0f,  1.0f) * texel_size, 0.0f);
    float4 c03 = wy.x * tex.SampleLevel(samp, base + float2(-1.0f,  2.0f) * texel_size, 0.0f);
    float4 c13 = wy.y * tex.SampleLevel(samp, base + float2( 0.0f,  2.0f) * texel_size, 0.0f);
    float4 c23 = wy.z * tex.SampleLevel(samp, base + float2( 1.0f,  2.0f) * texel_size, 0.0f);
    float4 c33 = wy.w * tex.SampleLevel(samp, base + float2( 2.0f,  2.0f) * texel_size, 0.0f);

    float4 col0 = (c0 + c1 + c2 + c3) * wx.x
                + (c01 + c11 + c21 + c31) * wx.y
                + (c02 + c12 + c22 + c32) * wx.z
                + (c03 + c13 + c23 + c33) * wx.w;
    return col0;
}

#endif 
#line 15 "shaders/lut/water_noise.cs.hlsl"

#line 1 "shaders/inc/image.hlsl"




#ifndef TINYGLADE_INC_IMAGE_HLSL
#define TINYGLADE_INC_IMAGE_HLSL

#line 1 "shaders/inc/curve.hlsl"















#define frexp incb_frexp

struct FrexpStructType
{
    float3 mantissa;
    int3 exponent;
};

float3 incb_frexp(float3 x, out int3 exponent)
{
    uint3 bits = asuint(x);
    int3 biased = int3(int((bits.x >> 23u) & 0xFFu),
                       int((bits.y >> 23u) & 0xFFu),
                       int((bits.z >> 23u) & 0xFFu));
    int3 e = biased - int3(127, 127, 127);
    float3 m = asfloat((bits & 0x807FFFFFu) | 0x3F800000u);

    
    bool3 big = (m >= 1.0f) | (m <= -1.0f);
    m = lerp(m, m * 0.5f, float3(big));
    e += int3(big);

    
    bool3 special = (biased == 0) | (biased == 0xFF);
    exponent = select(special, int3(0, 0, 0), e);
    return select(special, x, m);
}

float3 curve(float3 x)
{
    
    
    
    FrexpStructType f;
    int3 e;
    f.mantissa = frexp(x, e);
    f.exponent = e;
    return x + exp2(float3(f.exponent) - float3(8.0f, 8.0f, 7.0f));
}
#line 8 "shaders/inc/image.hlsl"



float2 texture_dims(Texture2D<float4> tex)
{
    uint w, h;
    tex.GetDimensions(w, h);
    return float2(w, h);
}


float sample_mask(Texture2D<float4> mask_tex, SamplerState samp, float2 uv)
{
    return mask_tex.SampleLevel(samp, uv, 0.0f).r;
}

#endif 
#line 16 "shaders/lut/water_noise.cs.hlsl"


cbuffer $Globals : register(b13, space0)
{
    uint2 blue_noise_texture_dims;
    uint2 output_tex_dims;
};

Texture2D<float4> blue_noise_texture : register(t14, space0);
RWTexture2D<unorm float4> output_tex : register(u15, space0);

float4 sample_noise_bicubic(float2 p, float2 texel)
{
    float2 f = floor(p - 0.5f);
    float2 t = p - (f + 0.5f);
    float2 t2 = t * t;
    float2 t_half = t * 0.5f;
    float2 t15 = t * 1.5f;

    float2 w0 = t * (-0.5f + t * (1.0f - t_half));
    float2 w1 = t * (0.5f + t * (2.0f - t15));
    float2 w0b = t2 * (-0.5f + t_half);
    float2 w2 = (1.0f + t2 * (-2.5f + t15)) + w1;

    float2 p0 = (f + (-0.5f).xx) * texel;
    float2 p2 = (f + 2.5f.xx) * texel;
    float2 p1 = (f + 0.5f.xx + (w1 / w2)) * texel;

    float4 s00 = blue_noise_texture.SampleLevel(sampler_llr, p0, 0.0f);
    float4 s10 = blue_noise_texture.SampleLevel(sampler_llr, float2(p1.x, p0.y), 0.0f);
    float4 s20 = blue_noise_texture.SampleLevel(sampler_llr, float2(p2.x, p0.y), 0.0f);
    float4 s01 = blue_noise_texture.SampleLevel(sampler_llr, float2(p0.x, p1.y), 0.0f);
    float4 s11 = blue_noise_texture.SampleLevel(sampler_llr, p1, 0.0f);
    float4 s21 = blue_noise_texture.SampleLevel(sampler_llr, float2(p2.x, p1.y), 0.0f);
    float4 s02 = blue_noise_texture.SampleLevel(sampler_llr, float2(p0.x, p2.y), 0.0f);
    float4 s12 = blue_noise_texture.SampleLevel(sampler_llr, float2(p1.x, p2.y), 0.0f);
    float4 s22 = blue_noise_texture.SampleLevel(sampler_llr, p2, 0.0f);

    return (s00 * w0.x * w0.y)
        + (s10 * w2.x * w0.y)
        + (s20 * w0b.x * w0.y)
        + (s01 * w0.x * w2.y)
        + (s11 * w2.x * w2.y)
        + (s21 * w0b.x * w2.y)
        + (s02 * w0.x * w0b.y)
        + (s12 * w2.x * w0b.y)
        + (s22 * w0b.x * w0b.y);
}

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    float2 out_dims = float2(output_tex_dims);
    uint bn_w;
    uint bn_h;
    uint bn_elements;
    blue_noise_texture.GetDimensions(0u, bn_w, bn_h, bn_elements);
    float2 bn_dims = float2(bn_w, bn_h);
    float2 texel = 1.0f / bn_dims;

    float2 p = ((float2(int3(dispatchThreadID).xy) + 0.5f) / out_dims) * bn_dims;
    float2 pr = ((float2(int3(dispatchThreadID).xy + int2(1, 0)) + 0.5f) / out_dims) * bn_dims;
    float2 pl = ((float2(int3(dispatchThreadID).xy - int2(1, 0)) + 0.5f) / out_dims) * bn_dims;
    float2 pu = ((float2(int3(dispatchThreadID).xy + int2(0, 1)) + 0.5f) / out_dims) * bn_dims;
    float2 pd = ((float2(int3(dispatchThreadID).xy - int2(0, 1)) + 0.5f) / out_dims) * bn_dims;

    float4 center = sample_noise_bicubic(p, texel);
    float4 right = sample_noise_bicubic(pr, texel);
    float4 left = sample_noise_bicubic(pl, texel);
    float4 up = sample_noise_bicubic(pu, texel);
    float4 down = sample_noise_bicubic(pd, texel);

    output_tex[uint2(int3(dispatchThreadID).xy)] = float4(center.x, (right.x - left.x) * (-0.3333333432674407958984375f) + 0.5f, (up.x - down.x) * (-0.3333333432674407958984375f) + 0.5f, 1.0f);
}
