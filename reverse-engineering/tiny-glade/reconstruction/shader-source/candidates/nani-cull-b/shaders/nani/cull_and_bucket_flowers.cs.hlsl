// ==RECONSTRUCTED== shaders/nani/cull_and_bucket_flowers.cs.hlsl | evidence: raw disasm of
// _nani_cull_and_bucket_flowers.cs.hlsl_cs_main_8a7d2f81e72c8eeb.bin: unpack FlowerXform affine,
// sphere center = translation, radius = max-basis-length * atlas radius; frustum + HZB cull;
// filtered append | status: O (algorithm/bindings); I (hzb constants)

#line 1 "shaders/nani/cull_and_bucket_flowers.cs.hlsl"

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "draw_lists.inc.hlsl"
#include "cull_and_bucket.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "atlas_part_info.inc.hlsl"
#include "garden/defs.inc.hlsl"

StructuredBuffer<MeshAtlasPartInfo[64]> atlas_part_info : register(t13, space0);
StructuredBuffer<NaniDrawListInfo> draw_list_info : register(t20, space0);
#if !defined(TINYGLADE_NANI_NO_HZB)
Texture2D<float> hzb_tex : register(t21, space0);
#endif

StructuredBuffer<FlowerXform> instance_buffer : register(t23, space0);
StructuredBuffer<uint4> dispatch_args_buffer : register(t24, space0);


#if !defined(TINYGLADE_NANI_NO_HZB)
cbuffer $Globals : register(b22, space0)
{
    uint2 hzb_tex_inner_dims;
};
#endif


struct PushConstants
{
    uint world_subset;
    uint draw_list_index_base;
};

[[vk::push_constant]]
PushConstants push_constants;

[numthreads(64, 1, 1)]
void cs_main(uint3 thread_id : SV_DispatchThreadID)
{
    if (dispatch_args_buffer[0].x == 0xDEADBEEFu)
    {
        return;
    }

    uint global_instance_offset = draw_list_info[push_constants.draw_list_index_base].global_instance_offset;
    uint global_instance_index = thread_id.x + global_instance_offset;

    FlowerXform xform = instance_buffer[global_instance_index];
    float4x4 m;
    m[0] = float4(xform.transform.data[0], xform.transform.data[3], xform.transform.data[6], xform.transform.data[9]);
    m[1] = float4(xform.transform.data[1], xform.transform.data[4], xform.transform.data[7], xform.transform.data[10]);
    m[2] = float4(xform.transform.data[2], xform.transform.data[5], xform.transform.data[8], xform.transform.data[11]);
    m[3] = float4(0.0f, 0.0f, 0.0f, 1.0f);
    float3 center = m[3].xyz;

    float3 basis_x = m[0].xyz;
    float3 basis_y = m[1].xyz;
    float3 basis_z = m[2].xyz;
    float scale = sqrt(max(max(dot(basis_x, basis_x), dot(basis_y, basis_y)), dot(basis_z, basis_z)));
    uint mesh_id = global_instance_index % 64u;
    float radius = scale * atlas_part_info[push_constants.world_subset][mesh_id].bounding_sphere_radius;

    float4 view_pos = mul(float4(center, 1.0f), view_constants[0].world_to_view);
    float view_z = view_pos.z;

    uint bucket = clamp(uint(clamp((32.0f - view_z) * 0.0034285f, 0.0f, 1.0f) * 1024.0f), 0u, 1023u);

    Frustum f = extract_frustum(view_constants[0].world_to_view);
    bool visible = cull_frustum_side_planes(f, view_pos.xyz, radius);

#if !defined(TINYGLADE_NANI_NO_HZB)
    if (visible)
    {
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
