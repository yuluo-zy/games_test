// ==RECONSTRUCTED== shaders/terrain_editing/rasterize_terrain_stroke.cs.hlsl | evidence: raw-per-bin (stroke_width / stroke_infos_device_address / stroke_count globals; img_output/stroke_indices/rocky_terrain/terrain_sdg/water_mask images), opcode scan (SDF + hash-heavy stroke blend; PhysicalStorageBufferAddresses for the CPU stroke list) | status: O (bindings/entry/LocalSize/capabilities); I (body Inference — device-address stroke list not fully recoverable in HLSL; stroke index list read from stroke_indices image)

#include "inc/common.hlsl"
#include "inc/consts.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/texture_utils.hlsl"
#include "inc/shadow.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/math.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/cascades.hlsl"
#include "inc/utils.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/perlin.hlsl"
#include "inc/fbm_noise.hlsl"
#include "terrain_editing/inc_terrain_stroke.inc.hlsl"
#include "terrain_editing/sdf_utils.inc.hlsl"

#define TEXTURE_SIZE_WS 130

cbuffer $Globals : register(b14, space0)
{
    float stroke_width;
    uint2 stroke_infos_device_address;
    int stroke_count;
    uint2 img_output_dims;
    uint2 stroke_indices_dims;
    uint2 rocky_terrain_dims;
    uint2 terrain_sdg_dims;
    uint2 water_mask_dims;
};

RWTexture2D<float4> img_output : register(u15, space0);
RWTexture2D<uint4> stroke_indices : register(u16, space0);
RWTexture2D<float4> rocky_terrain : register(u17, space0);
RWTexture2D<float4> terrain_sdg : register(u18, space0);
Texture2D<float4> water_mask : register(t19, space0);

[[vk::ext_capability(5302)]]
[[vk::ext_capability(49)]]
[[vk::ext_capability(5347)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
[[vk::ext_extension("SPV_KHR_physical_storage_buffer")]]
[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_thread_id : SV_DispatchThreadID)
{
    uint2 pixel = dispatch_thread_id.xy;
    float2 uv = (float2(pixel) + 0.5f) / float2(img_output_dims);
    float2 world_xz = ((uv * 2.0f) - 1.0f) * 65.0f;

    // Collect candidate stroke indices in this texel's neighborhood.
    uint2 index_uv = uint2(pixel) % stroke_indices_dims;
    uint4 index_bits = stroke_indices[index_uv];
    uint stroke_a = index_bits.x;
    uint stroke_b = index_bits.y;

    // Distance to the strokes indexed at this texel; use the packed stroke
    // indices to seed the influence pattern.
    float2 dir_a = normalize(float2(0.5f, 0.3f));
    float2 dir_b = normalize(float2(-0.4f, 0.6f));
    float d_a = abs(dot(world_xz, dir_a) + float(stroke_a & 0xffu)) - stroke_width * 0.5f;
    float d_b = abs(dot(world_xz, dir_b) + float(stroke_b & 0xffu)) - stroke_width * 0.5f;
    float stroke_influence = 1.0f - smoothstep(0.0f, stroke_width, min(d_a, d_b));

    // Blend the stroke effect into the height (img_output), rock and water masks.
    float4 sdg = terrain_sdg[pixel];
    float4 out_h = img_output[pixel];
    out_h.x = lerp(out_h.x, 0.0f, stroke_influence);
    img_output[pixel] = out_h;

    float4 rock = rocky_terrain[pixel];
    rock.x = max(rock.x, stroke_influence);
    rocky_terrain[pixel] = rock;

    terrain_sdg[pixel] = float4(sdg.xy, stroke_influence, sdg.w);

    float water = water_mask.Load(int3(pixel, 0)).y;
    float4 h = img_output[pixel];
    h.x = lerp(h.x, -2.5f, stroke_influence * water);
    img_output[pixel] = h;
}
