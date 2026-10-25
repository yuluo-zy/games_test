// ==RECONSTRUCTED== shaders/rocky_terrain/stairs/place_stairs.inc.hlsl
// KIND: include_only | embedded only in _rocky_terrain_stairs_stairs.cs.hlsl_cs_main_0.bin
// 【文件用途】楼梯生成:沿等高线网格单元的一条等离线线段放置一个楼梯台阶(以及概率性的碎石)。
//   线段两端点来自 inc/find_root.hlsl(2 次迭代符号变化二分)在两条穿越边上的定位,
//   每个端点提升为 float3(x, height, z)。按顺序的剔除测试:
//     mid 处 rocky_terrain.x < 0.01;两端都不贴近路径(smoothstep(0.075,0.125,path_mask.x) < 0.01);
//     线段长度 < 0.2。随后楼梯变换为 translation(pos - (0,0.5,0)) * basis(forward = normalize(p1-p0), up = +Y)
//     * scale(len,1,0.8),pos = 中点 - 坡度偏移(cross 归一化梯度帧向量,按 lerp(0,0.3,clamp(rocky.x+rocky.z,0,1)) 缩放)。
//   记录以 __alias_draw_commands[draw_indirect_buffer_offset].instanceCount 作为槽位追加。
//   另外以概率 rand01(hash(seed ^ 23784)) > 0.85 放置一个碎石(seed = uint((p0.x+p0.y+p0.z)*100),
//   缩放 lerp(0.2,0.4,rand01(hash(seed ^ 90455))),位置 lerp(p0,p1,rand01(hash(seed ^ 3845))) - offset),
//   在 pebble_indirect_draw_idx 命令的 pebble_buffer_offset + 槽位处追加。
// 【状态】O(算法逐字取自内联使用点);I(函数名/签名——已内联)
// 本头文件重写只添加中文注释,代码逐字保留。

#pragma once
#ifndef TINYGLADE_STAIRS_PLACE_STAIRS_INC_HLSL
#define TINYGLADE_STAIRS_PLACE_STAIRS_INC_HLSL

#include "inc/find_root.hlsl"

// 沿等高线网格单元的一条等离线线段放置一个楼梯台阶(及概率性碎石):
// 线段在高度 h 处穿越边 (e0a,e0b) 与边 (e1a,e1b),端点用 find_root 定位。
void place_stairs(float2 e0a, float2 e0b, float2 e1a, float2 e1b, float h)
{
    // 用符号变化二分定位两条穿越边在高度 h 处的交点
    float2 pa = find_root(e0a, e0b, h);
    float2 pb = find_root(e1a, e1b, h);
    // 端点提升为三维:y 取该等高层高度 h
    float3 p0 = float3(pa.x, h, pa.y);
    float3 p1 = float3(pb.x, h, pb.y);

    // 线段中点
    float3 mid = lerp(p0, p1, 0.5f);
    // 剔除:中点处岩石采样过低(不在岩石地形上)则不放台阶
    if (sample_rocky(mid.xz).x < 0.01f)
        return;

    // 剔除:两端点都不贴近路径(smoothstep 后 path_mask < 0.01)则不放台阶
    bool not_near_path0 = smoothstep(0.075f, 0.125f, path_mask.SampleLevel(sampler_llb, p0.xz * ROCKY_TERRAIN_SAMPLE_SCALE + 0.5f, 0.0f).x) < 0.01f;
    bool not_near_path1 = smoothstep(0.075f, 0.125f, path_mask.SampleLevel(sampler_llb, p1.xz * ROCKY_TERRAIN_SAMPLE_SCALE + 0.5f, 0.0f).x) < 0.01f;
    if (not_near_path0 && not_near_path1)
        return;

    // 剔除:线段过短(<0.2)则不放台阶
    float dist = distance(p0, p1);
    if (dist < 0.2f)
        return;

    // 坡度偏移:用高度图 Gather 近似地形梯度,叉乘归一化梯度帧向量,
    // 偏移量随岩石密度(rock.x + rock.z)在 [0,0.3] 内插值。
    float4 rock = sample_rocky(mid.xz);
    float4 g = terrain_heightmap.GatherRed(sampler_default, mid.xz * ROCKY_TERRAIN_SAMPLE_SCALE + 0.5f, int2(0, 0));
    float2 grad = ((float2((g.z + g.y) - (g.w + g.x), (g.x + g.y) - (g.w + g.z)) * 0.5f) * 26.5f) / (130.0f / float2(terrain_heightmap_dims));
    float3 offset = cross(normalize(float3(0.0f, grad.y, 1.0f)), normalize(float3(1.0f, grad.x, 0.0f))) * lerp(0.0f, 0.3f, clamp(rock.x + rock.z, 0.0f, 1.0f));
    // 台阶中心位置 = 中点 - 坡度偏移
    float3 pos = mid - offset;
    // 台阶朝向 = 线段方向(已归一化)
    float3 forward = normalize(p1 - p0);
    // 由端点坐标构造随机种子
    uint seed = uint((p0.x + p0.y + p0.z) * 100.0f);

    // 仅旋转基(forward 已归一化,内联构造 —— affine_basis 会重新归一化并多一次 normalize,
    // 与原始 bin 不一致,故内联)。
    float4x4 basis = transpose(float4x4(
        float4(forward, 0.0f),
        float4(0.0f, 1.0f, 0.0f, 0.0f),
        float4(cross(forward, float3(0.0f, 1.0f, 0.0f)), 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f)));
    // 最终变换:T(pos - (0,0.5,0)) * R * S(沿前向长度 dist,高 1,宽 0.8)
    float4x4 m = mul(affine_translation(pos - float3(0.0f, 0.5f, 0.0f)), mul(basis, affine_scale(float3(dist, 1.0f, 0.8f))));

    // 追加台阶实例:以楼梯 indirect draw 的 instanceCount 原子自增取得槽位
    uint slot;
    InterlockedAdd(__alias_draw_commands[uint(draw_indirect_buffer_offset)].instanceCount, 1u, slot);
    __alias_stairs_buffer[slot].transform = pack_affine(m);

    // 概率性附带一个碎石(概率 > 0.85)
    if (rand01(hash(seed ^ 23784u)) > 0.85f)
    {
        // 碎石缩放与位置各自由独立随机数决定
        float pebble_scale = lerp(0.2f, 0.4f, rand01(hash(seed ^ 90455u)));
        float t = rand01(hash(seed ^ 3845u));
        float3 pebble_pos = lerp(p0, p1, t) - offset;
        // 碎石追加到独立的 pebble indirect draw,槽位在 pebble_buffer_offset 之后
        uint pebble_slot;
        InterlockedAdd(__alias_draw_commands[pebble_indirect_draw_idx].instanceCount, 1u, pebble_slot);
        float4x4 pm = mul(affine_translation(pebble_pos), mul(basis, affine_scale(float3(pebble_scale, pebble_scale, pebble_scale))));
        __alias_stairs_buffer[pebble_buffer_offset + pebble_slot].transform = pack_affine(pm);
    }
}

#endif // TINYGLADE_STAIRS_PLACE_STAIRS_INC_HLSL
