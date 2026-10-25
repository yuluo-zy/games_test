// ============================================================
// inc/bitonic_sort.hlsl —— 双调排序辅助
// 用途:对 groupshared 中的 uint 数组做升序双调排序(64 元素规模)。
//   消费方:_dof_autofocus、_read_mouse_depth_value。
// 入口:无(纯头文件;宏 BITONIC_SORT_UINT 在使用点展开)
// 关键算法:比较-交换对索引按 k/j 级数推导,见 bitonic_sort_pair
// ============================================================
// ==RECONSTRUCTED== shaders/inc/bitonic_sort.hlsl | evidence: spirv-cross (_dof_autofocus/_read_mouse_depth_value) ascending 64-element bitonic sort over groupshared uint (k/j loop, pair indices a=m+i, b=j==k/2?(m+2j-1-i):(m+j+i), swap when data[a]>data[b]); include-graph edge bitonic_sort.hlsl -> coc.hlsl | status: O (algorithm); I (function names)

#ifndef TINYGLADE_INC_BITONIC_SORT_HLSL
#define TINYGLADE_INC_BITONIC_SORT_HLSL

// 计算本 lane 在阶段 j / k 的"比较-交换"对 (a, b)。
// 索引公式与 dof_autofocus 中观察到的逐项一致。
uint2 bitonic_sort_pair(uint lane, uint j, uint k)
{
    uint i = lane & (j - 1u);
    uint m = 2u * (lane - i);
    uint a = m + i;
    uint b = (j == (k >> 1u)) ? (m + ((2u * j) - 1u) - i) : (m + j + i);
    return uint2(a, b);
}

// 对 2 的幂大小的 groupshared uint 数组做升序双调排序。
// 消费方在每个阶段之间使用完整组同步屏障。
// `name` 为数组标识符,`lane` 为 lane 索引,`size` 为元素个数。
#define BITONIC_SORT_UINT(name, lane, size) \
    { \
        uint _b_sort_k_max = 2u << (firstbithigh((size) - 1u) & 31u); \
        for (uint _b_sort_k = 2u; _b_sort_k <= _b_sort_k_max; _b_sort_k *= 2u) { \
            uint _b_sort_j_start = _b_sort_k >> 1u; \
            for (uint _b_sort_j = _b_sort_j_start; _b_sort_j > 0u; _b_sort_j >>= 1u) { \
                uint2 _b_sort_ab = bitonic_sort_pair((lane), _b_sort_j, _b_sort_k); \
                if (_b_sort_ab.x < (size) && _b_sort_ab.y < (size)) { \
                    if (name[_b_sort_ab.x] > name[_b_sort_ab.y]) { \
                        uint _b_sort_tmp = name[_b_sort_ab.x]; \
                        name[_b_sort_ab.x] = name[_b_sort_ab.y]; \
                        name[_b_sort_ab.y] = _b_sort_tmp; \
                    } \
                } \
                GroupMemoryBarrierWithGroupSync(); \
            } \
        } \
    }

#endif // TINYGLADE_INC_BITONIC_SORT_HLSL
