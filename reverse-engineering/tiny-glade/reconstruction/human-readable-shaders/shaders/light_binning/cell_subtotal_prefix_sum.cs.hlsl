// ==RECONSTRUCTED== shaders/light_binning/cell_subtotal_prefix_sum.cs.hlsl
// 中文说明:
//   灯光装箱:对单一 512 字子块做包含扫描并写回排他前缀。证据来自
//   _light_binning_cell_subtotal_prefix_sum.cs.hlsl_cs_main_0.bin(LocalSize 256)。
//   绑定:push_constants([[vk::push_constant]] {uint range_end})、__alias_inout_buf
//   u13(RWByteAddressBuffer);caps 仅 {Shader}。groupshared shared_data2x[512]。
//   每线程载入 512 字子块的 2 字(range_end 之后补 0),做 9 步 Hillis-Steele
//   包含扫描,再写回排他前缀(同样受 range_end 门控)。

#include "inc/common.hlsl"
#include "inc/prefix_scan.hlsl"

RWByteAddressBuffer __alias_inout_buf : register(u13, space0);

struct PushConstants
{
    uint range_end;
};
[[vk::push_constant]]
PushConstants push_constants;

groupshared uint shared_data2x[512];

[numthreads(256, 1, 1)]
void cs_main(uint3 gl_LocalInvocationID : SV_GroupThreadID)
{
    uint lane = gl_LocalInvocationID.x;
    uint lane2 = 2u * lane;
    uint byte_addr = 4u * lane2;

    // 超出 range_end 的元素补 0(两个元素都超 / 只超一个)。
    bool has_both = (lane2 + 1u) < push_constants.range_end;
    uint2 v;
    if (has_both) {
        v = uint2(__alias_inout_buf.Load<uint>(byte_addr),
                  __alias_inout_buf.Load<uint>(byte_addr + 4u));
    } else {
        if (lane2 < push_constants.range_end) {
            v = uint2(__alias_inout_buf.Load<uint>(byte_addr), 0u);
        } else {
            v = uint2(0u, 0u);
        }
    }
    shared_data2x[lane2] = v.x;
    shared_data2x[lane2 + 1u] = v.y;
    GroupMemoryBarrierWithGroupSync();

    // 512 元素、每线程 2 字的包含扫描,显式展开以镜像原 dxc 输出(每步后同步)。
    shared_data2x[((lane >> 0u) << 1u) + 1u] += shared_data2x[(lane >> 0u) << 1u];
    GroupMemoryBarrierWithGroupSync();
    shared_data2x[((lane >> 1u) << 2u) + 2u + (lane & 1u)] += shared_data2x[((lane >> 1u) << 2u) + 1u];
    GroupMemoryBarrierWithGroupSync();
    shared_data2x[((lane >> 2u) << 3u) + 4u + (lane & 3u)] += shared_data2x[((lane >> 2u) << 3u) + 3u];
    GroupMemoryBarrierWithGroupSync();
    shared_data2x[((lane >> 3u) << 4u) + 8u + (lane & 7u)] += shared_data2x[((lane >> 3u) << 4u) + 7u];
    GroupMemoryBarrierWithGroupSync();
    shared_data2x[((lane >> 4u) << 5u) + 16u + (lane & 15u)] += shared_data2x[((lane >> 4u) << 5u) + 15u];
    GroupMemoryBarrierWithGroupSync();
    shared_data2x[((lane >> 5u) << 6u) + 32u + (lane & 31u)] += shared_data2x[((lane >> 5u) << 6u) + 31u];
    GroupMemoryBarrierWithGroupSync();
    shared_data2x[((lane >> 6u) << 7u) + 64u + (lane & 63u)] += shared_data2x[((lane >> 6u) << 7u) + 63u];
    GroupMemoryBarrierWithGroupSync();
    shared_data2x[((lane >> 7u) << 8u) + 128u + (lane & 127u)] += shared_data2x[((lane >> 7u) << 8u) + 127u];
    GroupMemoryBarrierWithGroupSync();
    shared_data2x[((lane >> 8u) << 9u) + 256u + (lane & 255u)] += shared_data2x[((lane >> 8u) << 9u) + 255u];
    GroupMemoryBarrierWithGroupSync();

    // 排他前缀(包含扫描左移一个元素),受 range_end 门控写回。
    uint ex0 = (lane > 0u) ? shared_data2x[lane2 - 1u] : 0u;
    uint ex1 = shared_data2x[lane2];

    if (has_both) {
        __alias_inout_buf.Store<uint>(byte_addr, ex0);
        __alias_inout_buf.Store<uint>(byte_addr + 4u, ex1);
    } else {
        if (lane2 < push_constants.range_end) {
            __alias_inout_buf.Store<uint>(byte_addr, ex0);
        }
    }
}
