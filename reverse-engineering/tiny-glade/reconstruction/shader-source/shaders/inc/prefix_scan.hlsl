// ==RECONSTRUCTED== shaders/inc/prefix_scan.hlsl | evidence: spirv-cross (_light_binning_cell_prefix_sum/_nani_histogram_prefix_sum/_grass_tiles_prefix) Hillis-Steele inclusive scan over groupshared uint (step s: i=(lane>>s)<<(s+1)+(1<<s)+(lane&((1<<s)-1)), shared[i]+=shared[i-(1<<s)]), 2 elements per lane in the light-binning variant; consumers declare their own shared_data1x/shared_data2x scratch arrays | status: O (algorithm); I (macro names)

#ifndef TINYGLADE_INC_PREFIX_SCAN_HLSL
#define TINYGLADE_INC_PREFIX_SCAN_HLSL

// Inclusive Hillis-Steele prefix sum over a power-of-two sized groupshared
// uint array. `shared` is the array identifier, `lane` the lane index and
// `element_count` the element count (one element per lane for the 1x variant,
// two per lane for the 2x variant). The consumer places its own barriers
// around the load/store phases; this macro emits the per-step barriers.
#define PREFIX_SCAN_UINT(shared, lane, element_count) \
    { \
        for (uint _ps_s = 0u; (1u << _ps_s) < (element_count); ++_ps_s) { \
            GroupMemoryBarrierWithGroupSync(); \
            uint _ps_d = 1u << _ps_s; \
            uint _ps_base = ((lane) >> _ps_s) << (_ps_s + 1u); \
            uint _ps_i = _ps_base + _ps_d + ((lane) & (_ps_d - 1u)); \
            if (_ps_i < (element_count)) { \
                shared[_ps_i] += shared[_ps_i - _ps_d]; \
            } \
        } \
        GroupMemoryBarrierWithGroupSync(); \
    }

// Load/scan/store for the "two elements per lane" prefix pass (light-binning,
// nani histogram/prefix): each lane sums two buffer words, then writes back
// exclusive prefix values and the block total. `shared` is the caller's
// groupshared uint array (size >= block_words*2).
#define PREFIX_SCAN_2_PER_LANE(inout_buf, subtotals_buf, shared, lane, group_id, block_words, value_clamp) \
    { \
        uint _ps2_idx0 = 4u * ((2u * (lane)) + ((group_id) * (block_words))); \
        uint _ps2_idx1 = _ps2_idx0 + 1u; \
        uint _ps2_v0 = min((inout_buf).Load<uint>(_ps2_idx0 * 4u + 0u), (value_clamp)); \
        uint _ps2_v1 = min((inout_buf).Load<uint>(_ps2_idx1 * 4u + 0u), (value_clamp)); \
        (shared)[(lane) * 2u] = _ps2_v0; \
        (shared)[(lane) * 2u + 1u] = _ps2_v1; \
        GroupMemoryBarrierWithGroupSync(); \
        PREFIX_SCAN_UINT((shared), (lane), (block_words) * 2u); \
        uint _ps2_ex0 = ((lane) == 0u) ? 0u : (shared)[(lane) * 2u - 1u]; \
        uint _ps2_ex1 = (shared)[(lane) * 2u]; \
        (inout_buf).Store<uint>(_ps2_idx0 * 4u + 0u, _ps2_ex0); \
        (inout_buf).Store<uint>(_ps2_idx1 * 4u + 0u, _ps2_ex1); \
        if ((lane) == 0u) { \
            (subtotals_buf).Store<uint>((group_id) * 4u + 0u, (shared)[(block_words) * 2u - 1u]); \
        } \
    }

#endif // TINYGLADE_INC_PREFIX_SCAN_HLSL
