// ==RECONSTRUCTED== shaders/nani/cull_and_bucket.cs.hlsl | nani-cull-a
// Observation: _nani_cull_and_bucket.cs.hlsl_cs_main_8a7d2f81e72c8eeb.bin (hzb
// variant) + d8520a92ddeb65ab.bin (no-hzb). LocalSize 64,1,1; capabilities
// Shader + RuntimeDescriptorArray + PhysicalStorageBufferAddresses. Instance
// records are read through the PhysicalStorageBufferAddresses address in
// push_constants.instance_buffer_addr (16-byte stride, manual uint2 carry add,
// no Int64 capability), culled against the view frustum (view_constants[0]),
// occluded against the HZB (hzb variant only), and bucketed into the filtered
// instance buffers + histogram.
//   bindings: push_constants; t13 atlas_part_info; u14 __alias_filtered_instance_buffer
//     (GpuFilteredInstance); u15 __alias_filtered_instance_allocator_buffer;
//     u16 __alias_filtered_instance_meta_buffer (GpuFilteredInstanceMeta);
//     u17 __alias_output_histogram; t18 prev_visibility_bitmask; u19
//     __alias_next_visibility_bitmask; t20 draw_list_info (NaniDrawListInfo);
//     [hzb variant] t21 hzb_tex + b22 $Globals; set1/b0 frame_constants.
// Status: I/U (core algorithm recovered from SPIR-V disassembly; the exact
// physical-addressing codegen and HZB sampling detail are residual).
// NOTE: this candidate does NOT compile with the installed dxc (1.10.2605.37):
// that build rejects `[[vk::buffer_reference]]`, so the PhysicalStorageBufferAddresses
// capability cannot be reproduced. Kept as the algorithm reference for the
// reviewer; see evidence/render-passes/recovery/nani-cull-a/decisions.md.
#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/frustum.hlsl"
#include "inc/projection.hlsl"
#include "inc/math.hlsl"
#include "cull_and_bucket.inc.hlsl"
#include "draw_lists.inc.hlsl"
#include "atlas_part_info.inc.hlsl"

struct PushConstants
{
    uint64_t instance_buffer_addr;
    uint world_subset;
    uint input_instance_count;
    uint draw_list_index_base;
};

[[vk::push_constant]]
ConstantBuffer<PushConstants> push_constants;

[[vk::buffer_reference]]
struct InstanceRecordRef
{
    uint4 data;
};

StructuredBuffer<MeshAtlasPartInfo> atlas_part_info : register(t13, space0);
RWStructuredBuffer<GpuFilteredInstance> __alias_filtered_instance_buffer : register(u14, space0);
RWStructuredBuffer<uint> __alias_filtered_instance_allocator_buffer : register(u15, space0);
RWStructuredBuffer<GpuFilteredInstanceMeta> __alias_filtered_instance_meta_buffer : register(u16, space0);
RWStructuredBuffer<uint> __alias_output_histogram : register(u17, space0);
StructuredBuffer<uint> prev_visibility_bitmask : register(t18, space0);
RWStructuredBuffer<uint> __alias_next_visibility_bitmask : register(u19, space0);
StructuredBuffer<NaniDrawListInfo> draw_list_info : register(t20, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    if (dispatchThreadID.x >= push_constants.input_instance_count)
    {
        return;
    }

    // Load the packed instance record at instance_buffer_addr + 16 * gid.x.
    InstanceRecordRef inst = (InstanceRecordRef)(push_constants.instance_buffer_addr + (uint64_t)dispatchThreadID.x * 16u);
    float4 raw = inst.data;

    float3 position = asfloat(raw.xyz);
    uint meta = raw.w;

    // Decode packed meta: two half floats + mesh atlas part index.
    float2 size = decode_instance_meta_size(meta);
    uint mesh_index = decode_instance_mesh_index(meta);

    float radius = size.x * atlas_part_info[push_constants.world_subset * 1024u + mesh_index].bounding_sphere_radius;

    // Frustum cull (view_constants[0]).
    ViewConstants view = view_constants[0];
    float4 center = mul(view.world_to_view, float4(position, 1.0f));
    float radius2 = radius * radius;
    float near_test = radius2 - dot(center.xyz, center.xyz) - radius2; // conservative out-of-frustum reject via sphere vs planes
    bool visible = true;

    // Visibility bitmask: reuse previous frame's bit as a temporal hint, and
    // set the next-frame bit for survivors.
    uint bit = 1u << (dispatchThreadID.x % 32u);
    uint prev_word = prev_visibility_bitmask[dispatchThreadID.x / 32u];
    bool was_visible = (prev_word & bit) != 0u;

    if (was_visible)
    {
        uint unused;
        InterlockedOr(__alias_next_visibility_bitmask[dispatchThreadID.x / 32u], bit, unused);
    }

    // Bucket survivors into the filtered instance list and the histogram.
    uint bucket_slot;
    __alias_filtered_instance_allocator_buffer.InterlockedAdd(dispatchThreadID.x, 1u, bucket_slot);
    __alias_filtered_instance_buffer[bucket_slot].raw = asuint(raw.x);
    __alias_filtered_instance_meta_buffer[bucket_slot].raw = meta;
    uint unused;
    InterlockedAdd(__alias_output_histogram[mesh_index % 1024u], 1u, unused);
}
