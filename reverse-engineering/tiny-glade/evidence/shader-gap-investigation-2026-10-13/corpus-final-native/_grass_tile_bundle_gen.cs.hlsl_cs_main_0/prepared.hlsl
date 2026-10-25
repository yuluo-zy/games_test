#line 1 "shaders/grass/tile_bundle_gen.cs.hlsl"












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
#line 13 "shaders/grass/tile_bundle_gen.cs.hlsl"

#line 1 "shaders/grass/grass_common.inc.hlsl"










#ifndef TINYGLADE_GRASS_GRASS_COMMON_INC_HLSL
#define TINYGLADE_GRASS_GRASS_COMMON_INC_HLSL

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 14 "shaders/grass/grass_common.inc.hlsl"





struct GrassTilePacked
{
    uint raw;
};




struct GrassBundlePacked
{
    uint raw;
};

#endif 
#line 14 "shaders/grass/tile_bundle_gen.cs.hlsl"


RWStructuredBuffer<GrassBundlePacked> __alias_bundle_buffer : register(u13, space0);
RWStructuredBuffer<uint> __alias_bundle_grasslet_index_buffer : register(u14, space0);
StructuredBuffer<uint> tile_bundle_prefix_buffer : register(t15, space0);
StructuredBuffer<uint> tile_bundle_subtotal_buffer : register(t16, space0);
StructuredBuffer<GrassTilePacked> tile_buffer : register(t17, space0);
StructuredBuffer<uint> blade_prefix_buffer : register(t18, space0);
StructuredBuffer<uint> blade_prefix_subtotal_buffer : register(t19, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 group_id : SV_GroupID, uint3 dispatch_id : SV_DispatchThreadID)
{
    uint bundle_base = tile_bundle_prefix_buffer[dispatch_id.x] + tile_bundle_subtotal_buffer[group_id.x];
    GrassTilePacked tile = tile_buffer[dispatch_id.x];
    uint tile_x = (tile.raw >> 24u) & 255u;
    uint tile_y = (tile.raw >> 16u) & 255u;
    uint tile_blades = (tile.raw & 0xFFFFu) % 512u;
    uint slots = 64u - ((blade_prefix_buffer[dispatch_id.x] + blade_prefix_subtotal_buffer[group_id.x]) % 64u);
    uint index = 0u;
    for (uint remaining = tile_blades; remaining > 0u; )
    {
        uint take = min(remaining, slots);
        uint bundle_index = bundle_base + index;
        GrassBundlePacked bundle = { ((tile_x << 24u) | (tile_y << 16u)) | (((tile_blades - remaining) * 128u) + take) };
        __alias_bundle_buffer[bundle_index] = bundle;
        __alias_bundle_grasslet_index_buffer[bundle_index] = uint(slots == 64u);
        uint slots_left = slots - take;
        slots = (slots_left == 0u) ? 64u : slots_left;
        index++;
        remaining -= take;
    }
}
