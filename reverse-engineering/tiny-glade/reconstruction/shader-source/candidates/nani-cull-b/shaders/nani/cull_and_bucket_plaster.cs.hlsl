// ==RECONSTRUCTED== shaders/nani/cull_and_bucket_plaster.cs.hlsl | evidence: raw disasm of
// _nani_cull_and_bucket_plaster.cs.hlsl_cs_main_8a7d2f81e72c8eeb.bin: per PlasterInstanceData,
// sphere from instance_buffer.bounding_sphere; frustum + HZB cull; filtered append.
// PhysicalStorageBufferAddresses declared via ext_capability (physical read approximated, see
// base cull status) | status: O (algorithm/bindings); U (physical read approximation)

#line 1 "shaders/nani/cull_and_bucket_plaster.cs.hlsl"

#include "inc/cull_and_bucket_owned_custom.hlsl"
#include "plaster.inc.hlsl"

// Local instance record (64B, same layout as plaster.inc PlasterInstanceData; the address
// member is kept as uint2 so the module does not emit the Int64 capability the original lacked).
struct PlasterInstanceCullData
{
    float4 bounding_sphere;
    uint wall_id;
    float wall_length;
    float blend_uv_seam;
    uint quad_count;
    uint2 verts_gpu_address;
    uint row_stride;
    float peel_strength;
};

StructuredBuffer<PlasterInstanceCullData> instance_buffer : register(t23, space0);
#if !defined(TINYGLADE_NANI_NO_HZB)
Texture2D<float> hzb_tex : register(t21, space0);
#endif



#if !defined(TINYGLADE_NANI_NO_HZB)
cbuffer $Globals : register(b22, space0)
{
    uint2 hzb_tex_inner_dims;
};
#endif


struct PushConstants
{
    uint2 instance_buffer_addr;
    uint world_subset;
    uint draw_list_index_base;
    uint input_instance_count;
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

    PlasterInstanceCullData data = instance_buffer[global_instance_index];
    float4 sphere = data.bounding_sphere;
    float3 center = sphere.xyz;
    float radius = sphere.w;
    uint mesh_id = global_instance_index % 64u;

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
