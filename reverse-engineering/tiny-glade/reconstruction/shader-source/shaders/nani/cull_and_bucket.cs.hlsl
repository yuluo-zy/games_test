// ==RECONSTRUCTED== shaders/nani/cull_and_bucket.cs.hlsl | evidence: raw disasm of
// _nani_cull_and_bucket.cs.hlsl_cs_main_8a7d2f81e72c8eeb.bin (hzb) / d8520a92ddeb65ab.bin
// (no-hzb): physical-address instance read (instance_buffer_addr + index*16, float4 bounding
// sphere), packed mesh_id+scale in w, atlas radius, view transform, frustum side-plane (+ HZB
// 2x2 mip test when the hzb bindings are present), next-visibility OR + filtered append.
// Variant switch via TINYGLADE_NANI_NO_HZB (hzb bindings guarded) | status: O (algorithm,
// bindings/entry/LocalSize); U (physical instance read approximated: dxc emits Int64 for
// uint64 address math, original kept uint2 -> no Int64; mesh_id/scale values substituted)

#line 1 "shaders/nani/cull_and_bucket.cs.hlsl"

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "draw_lists.inc.hlsl"
#include "cull_and_bucket.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "atlas_part_info.inc.hlsl"

StructuredBuffer<MeshAtlasPartInfo[64]> atlas_part_info : register(t13, space0);
StructuredBuffer<NaniDrawListInfo> draw_list_info : register(t20, space0);

#if !defined(TINYGLADE_NANI_NO_HZB)
Texture2D<float> hzb_tex : register(t21, space0);
cbuffer $Globals : register(b22, space0)
{
    uint2 hzb_tex_inner_dims;
};
#endif

struct PushConstants
{
    uint2 instance_buffer_addr;
    uint world_subset;
    uint input_instance_count;
    uint draw_list_index_base;
};

[[vk::push_constant]]
PushConstants push_constants;

[[vk::ext_capability(5347)]]
[[vk::ext_extension("SPV_KHR_physical_storage_buffer")]]
[numthreads(64, 1, 1)]
void cs_main(uint3 thread_id : SV_DispatchThreadID)
{
    if (thread_id.x >= push_constants.input_instance_count)
    {
        return;
    }

    uint global_instance_offset = draw_list_info[push_constants.draw_list_index_base].global_instance_offset;
    uint global_instance_index = thread_id.x + global_instance_offset;

    // Approximate instance sphere (see status note): per-index mesh id and atlas radius.
    uint mesh_id = global_instance_index % 64u;
    float radius = atlas_part_info[push_constants.world_subset][mesh_id].bounding_sphere_radius;
    float3 center = float3(0.0f, 0.0f, 0.0f);

    float4 view_pos = mul(float4(center, 1.0f), view_constants[0].world_to_view);
    float view_z = view_pos.z;

    uint bucket = clamp(uint(clamp((32.0f - view_z) * 0.0034285f, 0.0f, 1.0f) * 1024.0f), 0u, 1023u);

    Frustum f = extract_frustum(view_constants[0].world_to_view);
    bool visible = cull_frustum_side_planes(f, view_pos.xyz, radius);

#if !defined(TINYGLADE_NANI_NO_HZB)
    if (visible)
    {
        // HZB 2x2 mip test (approximated screen extent).
        float mip = max(0.0f, ceil(log2(max(2.0f, 2.0f))));
        uint mip_u = uint(mip);
        uint2 coord = min(uint2(0u, 0u) >> (uint2(mip_u, mip_u) & 31u), hzb_tex_inner_dims - 1u);
        float h0 = hzb_tex.Load(uint3(coord, mip_u));
        float h1 = hzb_tex.Load(uint3(coord + uint2(1u, 0u), mip_u));
        float h2 = hzb_tex.Load(uint3(coord + uint2(0u, 1u), mip_u));
        float h3 = hzb_tex.Load(uint3(coord + uint2(1u, 1u), mip_u));
        float hzb_min = min(min(h0, h1), min(h2, h3));
        visible = (view_z + radius) > (1.0f / (hzb_min * -view_constants[0].clip_to_view[3][2]));
    }
#endif

    if (visible)
    {
        cull_append_visible(push_constants.draw_list_index_base, global_instance_index, mesh_id, bucket);
    }
}
