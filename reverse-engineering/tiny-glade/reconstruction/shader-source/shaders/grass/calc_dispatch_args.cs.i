#line 1 "reconstruction/shader-source/shaders/grass/calc_dispatch_args.cs.hlsl"
#line 10 "reconstruction/shader-source/shaders/grass/calc_dispatch_args.cs.hlsl"
#line 1 "./reconstruction/shader-source/shaders/inc/common.hlsl"








#line 1 "./reconstruction/shader-source/shaders/inc/samplers.hlsl"
#line 11 "./reconstruction/shader-source/shaders/inc/samplers.hlsl"
#line 1 "./reconstruction/shader-source/shaders/inc/debug_utils.hlsl"








#line 1 "./reconstruction/shader-source/shaders/inc/tracked_buffer.hlsl"
#line 13 "./reconstruction/shader-source/shaders/inc/tracked_buffer.hlsl"
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
#line 10 "./reconstruction/shader-source/shaders/inc/debug_utils.hlsl"


float3 debug_heatmap(float v)
{
    float t = clamp(v, 0.0f, 1.0f);
    return lerp(float3(0.0f, 0.0f, 1.0f), float3(1.0f, 0.0f, 0.0f), t);
}
#line 12 "./reconstruction/shader-source/shaders/inc/samplers.hlsl"

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
#line 10 "./reconstruction/shader-source/shaders/inc/common.hlsl"
#line 11 "reconstruction/shader-source/shaders/grass/calc_dispatch_args.cs.hlsl"
#line 1 "./reconstruction/shader-source/shaders/grass/grass_common.inc.hlsl"
#line 14 "./reconstruction/shader-source/shaders/grass/grass_common.inc.hlsl"
#line 1 "./reconstruction/shader-source/shaders/inc/consts.hlsl"
#line 11 "./reconstruction/shader-source/shaders/inc/consts.hlsl"
static const float RENDER_SCALE = 1.0f;
#line 15 "./reconstruction/shader-source/shaders/grass/grass_common.inc.hlsl"




struct GrassTilePacked
{
    uint raw;
};




struct GrassBundlePacked
{
    uint raw;
};
#line 12 "reconstruction/shader-source/shaders/grass/calc_dispatch_args.cs.hlsl"

RWByteAddressBuffer __alias_grass_dispatch_args_buffer : register(u13, space0);
ByteAddressBuffer bundle_grasslet_index_buffer : register(t14, space0);
ByteAddressBuffer tile_bundle_subtotal_buffer : register(t15, space0);

[numthreads(1, 1, 1)]
void cs_main()
{
    uint total = tile_bundle_subtotal_buffer.Load<uint>(1024);
    uint last_grasslet;
    if (total == 0u)
    {
        last_grasslet = 0u;
    }
    else
    {
        last_grasslet = bundle_grasslet_index_buffer.Load<uint>((total - 1u) * 4 + 0);
    }
    __alias_grass_dispatch_args_buffer.Store<uint4>(0, uint4(last_grasslet, 1u, 1u, 0u));
    __alias_grass_dispatch_args_buffer.Store<uint4>(16, uint4((total + 63u) / 64u, 1u, 1u, total));
}
