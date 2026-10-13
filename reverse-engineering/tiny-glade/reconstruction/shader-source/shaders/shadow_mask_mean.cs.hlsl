// ==RECONSTRUCTED== shaders/shadow_mask_mean.cs.hlsl | F-D
// Evidence: _shadow_mask_mean.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: __alias_shadow_mask_histogram u13 (RWByteAddressBuffer),
//   __alias_shadow_mask_mean u14, frame_constants set1/b0, LocalSize [256,1,1],
//   entry cs_main.
//   spirv-cross: each lane converts a shadow-mask histogram bucket into a weighted
//   (value = lane/255, weight = count) pair and clears the histogram; a tree
//   reduction computes the mean; the running mean is smoothed with
//   exp(-5*dt) and stored back.
// Status: O (structure from disassembly).

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/math_const.hlsl"

RWByteAddressBuffer __alias_shadow_mask_histogram : register(u13, space0);
RWByteAddressBuffer __alias_shadow_mask_mean : register(u14, space0);

groupshared float2 thread_vals[256];

[numthreads(256, 1, 1)]
void cs_main(uint groupIndex : SV_GroupIndex)
{
    thread_vals[groupIndex] = float2(float(groupIndex) * 0.0039215688593685626983642578125f, 1.0f) * float(__alias_shadow_mask_histogram.Load<uint>(groupIndex * 4 + 0));
    __alias_shadow_mask_histogram.Store<uint>(groupIndex * 4 + 0, 0u);

    float mean;
    float old_mean;
    if (!(0u == frame_index))
    {
        GroupMemoryBarrierWithGroupSync();
        for (uint stride = 128u; stride > 0u; stride = stride >> 1u)
        {
            if (groupIndex < stride)
            {
                thread_vals[groupIndex] += thread_vals[groupIndex + stride];
            }
            GroupMemoryBarrierWithGroupSync();
        }
        mean = thread_vals[0].x / thread_vals[0].y;
        old_mean = clamp(__alias_shadow_mask_mean.Load<float>(0), 0.0f, 1.0f);
    }
    else
    {
        mean = 0.5f;
        old_mean = 0.5f;
    }

    if (groupIndex == 0u)
    {
        __alias_shadow_mask_mean.Store<float>(0, lerp(old_mean, mean, clamp(1.0f - exp(delta_time_seconds * (-5.0f)), 0.0f, 1.0f)));
    }
}
