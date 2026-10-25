// ==RECONSTRUCTED== shaders/inc/bitonic_sort.hlsl | evidence: spirv-cross (_dof_autofocus/_read_mouse_depth_value) ascending 64-element bitonic sort over groupshared uint (k/j loop, pair indices a=m+i, b=j==k/2?(m+2j-1-i):(m+j+i), swap when data[a]>data[b]); include-graph edge bitonic_sort.hlsl -> coc.hlsl | status: O (algorithm); I (function names)

#ifndef TINYGLADE_INC_BITONIC_SORT_HLSL
#define TINYGLADE_INC_BITONIC_SORT_HLSL

// Compute the compare-exchange pair (a, b) for this lane at stage j / k of a
// bitonic merge. Matches the observed index math in dof_autofocus.
uint2 bitonic_sort_pair(uint lane, uint j, uint k)
{
    uint i = lane & (j - 1u);
    uint m = 2u * (lane - i);
    uint a = m + i;
    uint b = (j == (k >> 1u)) ? (m + ((2u * j) - 1u) - i) : (m + j + i);
    return uint2(a, b);
}

// Ascending bitonic sort of a power-of-two sized groupshared uint array.
// Consumers drive it with a full barrier between stages. `name` is the array
// identifier, `lane` the lane index, `size` the element count.
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
