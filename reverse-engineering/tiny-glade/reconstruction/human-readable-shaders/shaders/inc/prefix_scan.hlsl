// ======================================================================
// inc/prefix_scan.hlsl —— groupshared 前缀和(Hillis–Steele)宏公共头文件
// ======================================================================
// 用途:被 _light_binning_cell_prefix_sum / _nani_histogram_prefix_sum /
//       _grass_tiles_prefix 使用;消费者自行声明 shared_data1x/shared_data2x
//       暂存数组。
// 算法:对 groupshared uint 做包含式 Hillis–Steele 扫描(步长 s:
//       i=(lane>>s)<<(s+1)+(1<<s)+(lane&((1<<s)-1)),shared[i]+=shared[i-(1<<s)]),
//       light-binning 变体每个 lane 处理 2 个元素。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/prefix_scan.hlsl | evidence: spirv-cross (_light_binning_cell_prefix_sum/_nani_histogram_prefix_sum/_grass_tiles_prefix) Hillis-Steele inclusive scan over groupshared uint (step s: i=(lane>>s)<<(s+1)+(1<<s)+(lane&((1<<s)-1)), shared[i]+=shared[i-(1<<s)]), 2 elements per lane in the light-binning variant; consumers declare their own shared_data1x/shared_data2x scratch arrays | status: O (algorithm); I (macro names)

#ifndef TINYGLADE_INC_PREFIX_SCAN_HLSL
#define TINYGLADE_INC_PREFIX_SCAN_HLSL

// 对 2 的幂大小的 groupshared uint 数组做包含式 Hillis–Steele 前缀和。
// `shared` 是数组标识符,`lane` 是线程索引,`element_count` 是元素个数
// (1x 变体每 lane 一个,2x 变体每 lane 两个)。消费者在自己的 load/store
// 阶段之间放置屏障;本宏输出每步的屏障。
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

// "每 lane 两个元素"前缀 pass 的加载/扫描/写回(light-binning、nani
// histogram/prefix):每个 lane 先求和两个缓冲字,再写回排他前缀值与块总和。
// `shared` 是调用方的 groupshared uint 数组(大小 >= block_words*2)。
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
