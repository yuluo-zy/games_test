// ==RECONSTRUCTED== shaders/primary_rt_debug.cs.hlsl | F-D (手写段)
// 用途: 主光线追踪调试通道。每像素发射一条主光线, 沿软件 BVH (rt_bl_bvh / rt_triangles,
//   groupshared rt_lds_stack 保存深度超过 4 的被压栈节点) 求交, 并对视觉地形高度图做软件
//   步进求交; 最多 3 次反弹; 天空 / 地形 / 网格分别用 GGX 阳光 + PCSS 软阴影着色。
//   最终写入时, 若无相机重投影 (clip_to_prev_clip[3].xyz 长度 <= 0.001) 则把新结果按
//   25% 比例 lerp 进上一帧输出 (时序累积), 否则直接覆盖。
// 变体: 单变体 cs_main, 整文件手写 (无 TGV 守卫)。
// 绑定: sampler_lnc s0 / sampler_llc s3 / sampler_default s9 / sampler_shadow_nearest s11 /
//   sampler_shadow_linear s12, frame_constants set1/b0, $Globals b13,
//   terrain_heightmap t14, visual_terrain_heightmap t15, blue_noise_texture t16,
//   terrain_lighting_texture t17, color_library b18, shadow_map t19,
//   wall_color_ids t21, rt_triangle_meta t22, rt_bl_bvh t23, rt_triangles t24,
//   output_tex u25, 工作组尺寸 [8,8,1], 入口 cs_main。
// ==RECONSTRUCTED== shaders/primary_rt_debug.cs.hlsl | F-D
// Evidence: _primary_rt_debug.cs.hlsl_cs_main_0.bin (SPIR-V, module 0).
//   bindings: sampler_lnc s0, sampler_llc s3, sampler_default s9,
//   sampler_shadow_nearest s11, sampler_shadow_linear s12, ViewConstants set1/b0
//   (frame_constants), $Globals b13, terrain_heightmap t14, visual_terrain_heightmap
//   t15, blue_noise_texture t16, terrain_lighting_texture t17, color_library b18,
//   shadow_map t19, wall_color_ids t21, rt_triangle_meta t22, rt_bl_bvh t23,
//   rt_triangles t24, output_tex u25. LocalSize [8,8,1], entry cs_main.
//   spirv-cross: primary ray-tracing debug pass. Three bounces; each bounce traces
//   the ray against the software BVH (rt_bl_bvh / rt_triangles, groupshared
//   rt_lds_stack), ray-marches the visual terrain heightmap, and shades sky /
//   terrain / mesh with a GGX sun + PCSS shadow term. The final write lerps the
//   accumulated radiance into the previous output when no camera reprojection is
//   active (clip_to_prev_clip[3].xyz length <= 0.001).
// Status: O (structure from disassembly); I (helper names inlined).

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
// current_view.hlsl is guard-skipped: the current_view_index global is declared
// inside $Globals below (it is the first $Globals member in the target).
#define TINYGLADE_INC_CURRENT_VIEW_HLSL
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/rt.hlsl"
// color_library.hlsl is guard-skipped; the cbuffer is redeclared below under the
// original binding name `color_library` with flat members (target layout).
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL
#include "inc/color_library.hlsl"
#include "inc/hash.hlsl"

// Bindless descriptor array declared only to reproduce the target module's
// RuntimeDescriptorArray capability + SPV_EXT_descriptor_indexing extension
// (the original declaration is unused here and was dropped by spirv-cross).
Texture2D<float4> rt_debug_unused_tex_array[];

cbuffer color_library : register(b18, space0)
{
    float4 temp_color_1;
    float4 temp_color_2;
    float4 temp_color_3;
    float4 roof_color_1;
    float4 roof_color_2;
    float4 sky_low_day;
    float4 sky_high_day;
    float4 sun_tint_day;
    float4 sky_low_dusk;
    float4 sky_high_dusk;
    float4 sun_tint_dusk;
    float4 indirect_light_tint;
    float4 fence_color_1;
    float4 fence_color_2;
    float4 flower_color_1;
    float4 flower_color_2;
    float4 canopy_color_1;
    float4 canopy_color_2;
    float4 grass_1;
    float4 grass_2;
    float4 path;
    float4 sky_base;
    float4 sky_horizon;
    float4 sky_clouds_1;
    float4 sky_clouds_2;
    float4 terrain_1;
    float4 terrain_2;
    float4 terrain_flattened;
    float4 wall_1;
    float4 wall_2;
    float4 wall_moss;
    float4 wall_bricks;
    float4 ivy_branch;
    float4 ivy_leaf_1;
    float4 ivy_leaf_2;
    float4 terrain_highlight;
    float4 night_tint;
};

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 blue_noise_texture_dims;
    uint2 terrain_lighting_texture_dims;
    uint2 brdf_fg_lut_dims;
    uint rt_bvh_node_count;
    uint rt_triangle_count;
    uint2 output_tex_dims;
};

ByteAddressBuffer wall_color_ids : register(t21, space0);
ByteAddressBuffer rt_triangle_meta : register(t22, space0);
ByteAddressBuffer rt_bl_bvh : register(t23, space0);
ByteAddressBuffer rt_triangles : register(t24, space0);
Texture2D<float4> terrain_heightmap : register(t14, space0);
Texture2D<float4> visual_terrain_heightmap : register(t15, space0);
Texture2D<float4> blue_noise_texture : register(t16, space0);
Texture2D<float4> terrain_lighting_texture : register(t17, space0);
Texture2DArray<float4> shadow_map : register(t19, space0);
RWTexture2D<float3> output_tex : register(u25, space0);

// Module-scope statics corresponding to the target's OpUndef values: the initial
// accumulated-hit normal (overwritten on the first real hit) and the unused
// second components of the PCSS accumulators.
static float3 undefined_normal;
static float3 undefined_f3;
static float undefined_f;

// Decode the per-slot "xor key" byte pattern from a packed child-slot word.
// 解码子节点槽位打包字中的 "xor key" 字节模式 (把 4 个 8 位 key 展开到各自槽位)。
uint decode_slot_xor_key(uint word, uint slot_key)
{
    return (word ^ (slot_key & ((((word & (word << 1u)) & 269488144u) >> 4u) * 255u))) & 522133279u;
}

// Decode the per-slot triangle-count bytes from a packed child-slot word.
// 解码子节点槽位打包字中的三角形计数字节 (每槽 8 位)。
uint decode_slot_tri_count(uint word)
{
    return (word >> 5u) & 117901063u;
}

// Unpack half16 -> float (triangle edge encoding).
// half16 -> float 解包 (三角形边以半精度打包)。
float2 unpack_half2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

// Ray-direction octant keys and the 13 wall palette colors.
// 空栈常量, 以及 13 种墙调色板颜色 (供 wall_color_ids 索引)。
static const uint2 EMPTY_STACK[4] = { uint2(0u, 0u), uint2(0u, 0u), uint2(0u, 0u), uint2(0u, 0u) };
static const float3 WALL_COLORS[13] = {
    float3(0.63f, 0.37f, 0.17f), float3(0.56f, 0.37f, 0.14f), float3(0.27f, 0.22f, 0.13f),
    float3(0.38f, 0.33f, 0.21f), float3(0.69f, 0.22f, 0.13f), float3(0.71f, 0.39f, 0.10f),
    float3(0.15f, 0.36f, 0.35f), float3(0.26f, 0.26f, 0.17f), float3(0.47f, 0.47f, 0.47f),
    float3(0.25f, 0.30f, 0.20f), float3(0.41f, 0.40f, 0.32f), float3(0.285f, 0.2475f, 0.1575f),
    float3(0.41f, 0.40f, 0.32f)
};

// 软件光线遍历栈: 深度超过 4 时把待访问节点压入 groupshared (每 64 线程一段)。
groupshared uint2 rt_lds_stack[256];

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID,
             uint gl_LocalInvocationIndex : SV_GroupIndex)
{
    // 当前像素坐标; sample_pos 为 [-1,1]x[-1,1] 的采样空间位置。
    int2 pixel = int2(gl_GlobalInvocationID.xy);
    float2 sample_pos = (((float2(pixel) + 0.5f.xx) / float2(output_tex_dims)) - 0.5f.xx) * float2(2.0f, -2.0f);
    float sample_x = sample_pos.x;

    // 由远平面反投影得到射线原点 (世界坐标)。
    float4 world_far = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view, float4(sample_x, sample_pos.y, 1.0f, 1.0f)));

    // 累积辐射; w=100000 作为 "未命中" 距离哨兵。
    float4 radiance = float4(0.0f, 0.0f, 0.0f, 100000.0f);
    // 射线方向 (相机 -> 世界)。
    float3 ray_dir = normalize(mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view, float4(sample_x, sample_pos.y, 0.0f, 1.0f))).xyz);
    float3 throughput = float3(1.0f, 1.0f, 1.0f);
    float3 accumulated_normal = undefined_normal;
    float3 ray_origin = world_far.xyz / world_far.w.xxx;

    float4 result;
    // 反弹计数, 最多 3 次。
    uint bounce = 0u;
    for (;;)
    {
        if (bounce < 3u)
        {
            // 蓝噪声采样得到本反弹随机数 rnd (分量用于采样方向/半球)。
            float4 rnd = ((blue_noise_texture.Load(int3((gl_GlobalInvocationID.xy + uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float((frame_index + 1234567u) + (bounce * 137u))) + 0.5f.xx) * 256.0f.xx)) % uint2(256u, 256u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;

            float3 hit_normal;
            uint hit_kind;
            uint hit_tri_index;
            float hit_dist;
            do
            {
                // 局部节点栈 (深度 <=4 用局部数组, 更深存 groupshared)。
                uint2 ray_stack[4] = EMPTY_STACK;
                // 按射线方向符号决定子节点索引翻转 (BVH 按八分体排序)。
                bool rd_x_neg = ray_dir.x < 0.0f;
                bool rd_y_neg = ray_dir.y < 0.0f;
                bool rd_z_neg = ray_dir.z < 0.0f;
                // 由方向八分体生成槽位 key (每字节一份)。
                uint slot_key = (7u - uint(((rd_x_neg ? 4 : 0) | (rd_y_neg ? 2 : 0)) | int(rd_z_neg))) * 16843009u;
                // 根节点引用: 高 8 位 0x80 表示 "仍有待访问子节点位"。
                uint2 node_ref = uint2(0u, 2147483648u);
                float3 best_normal = accumulated_normal;
                // 迄今最近命中距离 (初始为 FLT_MAX)。
                float best_dist = 3.4028234663852885981170418348452e+38f;
                int stack_depth = 0;
                uint iter_count = 0u;
                int best_tri = -1;

                [loop]
                for (;;)
                {
                    if (iter_count < 1024u)
                    {
                        uint2 child_ref;
                        uint2 stack_entry;
                        int push_depth;
                        if ((node_ref.y & 4278190080u) != 0u)
                        {
                        // 取最高的子节点位, 并从剩余掩码中清除。
                            uint child_bit = firstbithigh(node_ref.y);
                            uint child_mask = node_ref.y & (~(1u << (child_bit & 31u)));
                            uint2 pushed = node_ref;
                            pushed.y = child_mask;
                            int depth_after;
                            do
                            {
                                if (stack_depth >= 4)
                                {
                                    rt_lds_stack[gl_LocalInvocationIndex + uint((stack_depth - 4) * 64)] = pushed;
                                    depth_after = stack_depth + 1;
                                    break;
                                }
                                ray_stack[stack_depth] = pushed;
                                depth_after = stack_depth + 1;
                                break;
                            } while (false);

                        // 加载 80 字节打包 BVH 节点。
                            PackedBlBvhNode node_data;
                            [unroll]
                            for (int node_i = 0; node_i < 5; node_i++)
                            {
                                node_data.data[node_i] = rt_bl_bvh.Load<uint4>(node_i * 16 + (node_ref.x + countbits(node_ref.y & (~(4294967295u << (((child_bit - 24u) ^ (slot_key & 255u)) & 31u))))) * 80 + 0);
                            }

                            // 子节点包围盒尺寸倒数 (SLT 编码), 用于快速 slab 测试。
                            float3 extent_inv = float3(asfloat(((node_data.data[0].w >> 0u) & 255u) << 23u) / ray_dir.x,
                                                       asfloat(((node_data.data[0].w >> 8u) & 255u) << 23u) / ray_dir.y,
                                                       asfloat(((node_data.data[0].w >> 16u) & 255u) << 23u) / ray_dir.z);
                            float3 rel = (asfloat(node_data.data[0].xyz) - ray_origin) / ray_dir;

                            // 解码槽位 A 的 xor key 与三角计数。
                            uint slot_xor = decode_slot_xor_key(node_data.data[1].z, slot_key);
                            uint slot_tri = decode_slot_tri_count(node_data.data[1].z);
                            uint child_a_lo0 = rd_x_neg ? node_data.data[2].z : node_data.data[2].x;
                            uint child_a_hi0 = rd_x_neg ? node_data.data[2].x : node_data.data[2].z;
                            uint child_a_lo1 = rd_y_neg ? node_data.data[3].z : node_data.data[3].x;
                            uint child_a_hi1 = rd_y_neg ? node_data.data[3].x : node_data.data[3].z;
                            uint child_a_lo2 = rd_z_neg ? node_data.data[4].z : node_data.data[4].x;
                            uint child_a_hi2 = rd_z_neg ? node_data.data[4].x : node_data.data[4].z;

                            uint hit_bits = 0u;
                            {
                                float3 lo = (float3(float((child_a_lo0 >> 0u) & 255u), float((child_a_lo1 >> 0u) & 255u), float((child_a_lo2 >> 0u) & 255u)) * extent_inv) + rel;
                                float3 hi = (float3(float((child_a_hi0 >> 0u) & 255u), float((child_a_hi1 >> 0u) & 255u), float((child_a_hi2 >> 0u) & 255u)) * extent_inv) + rel;
                                if (max(lo.x, max(lo.y, max(lo.z, 9.9999997473787516355514526367188e-05f))) <= min(hi.x, min(hi.y, min(hi.z, best_dist))))
                                {
                                    hit_bits = 0u | (((slot_tri >> 0u) & 255u) << (((slot_xor >> 0u) & 255u) & 31u));
                                }
                            }
                            {
                                float3 lo = (float3(float((child_a_lo0 >> 8u) & 255u), float((child_a_lo1 >> 8u) & 255u), float((child_a_lo2 >> 8u) & 255u)) * extent_inv) + rel;
                                float3 hi = (float3(float((child_a_hi0 >> 8u) & 255u), float((child_a_hi1 >> 8u) & 255u), float((child_a_hi2 >> 8u) & 255u)) * extent_inv) + rel;
                                if (max(lo.x, max(lo.y, max(lo.z, 9.9999997473787516355514526367188e-05f))) <= min(hi.x, min(hi.y, min(hi.z, best_dist))))
                                {
                                    hit_bits |= (((slot_tri >> 8u) & 255u) << (((slot_xor >> 8u) & 255u) & 31u));
                                }
                            }
                            {
                                float3 lo = (float3(float((child_a_lo0 >> 16u) & 255u), float((child_a_lo1 >> 16u) & 255u), float((child_a_lo2 >> 16u) & 255u)) * extent_inv) + rel;
                                float3 hi = (float3(float((child_a_hi0 >> 16u) & 255u), float((child_a_hi1 >> 16u) & 255u), float((child_a_hi2 >> 16u) & 255u)) * extent_inv) + rel;
                                if (max(lo.x, max(lo.y, max(lo.z, 9.9999997473787516355514526367188e-05f))) <= min(hi.x, min(hi.y, min(hi.z, best_dist))))
                                {
                                    hit_bits |= (((slot_tri >> 16u) & 255u) << (((slot_xor >> 16u) & 255u) & 31u));
                                }
                            }
                            {
                                float3 lo = (float3(float((child_a_lo0 >> 24u) & 255u), float((child_a_lo1 >> 24u) & 255u), float((child_a_lo2 >> 24u) & 255u)) * extent_inv) + rel;
                                float3 hi = (float3(float((child_a_hi0 >> 24u) & 255u), float((child_a_hi1 >> 24u) & 255u), float((child_a_hi2 >> 24u) & 255u)) * extent_inv) + rel;
                                if (max(lo.x, max(lo.y, max(lo.z, 9.9999997473787516355514526367188e-05f))) <= min(hi.x, min(hi.y, min(hi.z, best_dist))))
                                {
                                    hit_bits |= (((slot_tri >> 24u) & 255u) << (((slot_xor >> 24u) & 255u) & 31u));
                                }
                            }

                            uint slot_xor_b = decode_slot_xor_key(node_data.data[1].w, slot_key);
                            uint slot_tri_b = decode_slot_tri_count(node_data.data[1].w);
                            uint child_b_lo0 = rd_x_neg ? node_data.data[2].w : node_data.data[2].y;
                            uint child_b_hi0 = rd_x_neg ? node_data.data[2].y : node_data.data[2].w;
                            uint child_b_lo1 = rd_y_neg ? node_data.data[3].w : node_data.data[3].y;
                            uint child_b_hi1 = rd_y_neg ? node_data.data[3].y : node_data.data[3].w;
                            uint child_b_lo2 = rd_z_neg ? node_data.data[4].w : node_data.data[4].y;
                            uint child_b_hi2 = rd_z_neg ? node_data.data[4].y : node_data.data[4].w;

                            {
                                float3 lo = (float3(float((child_b_lo0 >> 0u) & 255u), float((child_b_lo1 >> 0u) & 255u), float((child_b_lo2 >> 0u) & 255u)) * extent_inv) + rel;
                                float3 hi = (float3(float((child_b_hi0 >> 0u) & 255u), float((child_b_hi1 >> 0u) & 255u), float((child_b_hi2 >> 0u) & 255u)) * extent_inv) + rel;
                                if (max(lo.x, max(lo.y, max(lo.z, 9.9999997473787516355514526367188e-05f))) <= min(hi.x, min(hi.y, min(hi.z, best_dist))))
                                {
                                    hit_bits |= (((slot_tri_b >> 0u) & 255u) << (((slot_xor_b >> 0u) & 255u) & 31u));
                                }
                            }
                            {
                                float3 lo = (float3(float((child_b_lo0 >> 8u) & 255u), float((child_b_lo1 >> 8u) & 255u), float((child_b_lo2 >> 8u) & 255u)) * extent_inv) + rel;
                                float3 hi = (float3(float((child_b_hi0 >> 8u) & 255u), float((child_b_hi1 >> 8u) & 255u), float((child_b_hi2 >> 8u) & 255u)) * extent_inv) + rel;
                                if (max(lo.x, max(lo.y, max(lo.z, 9.9999997473787516355514526367188e-05f))) <= min(hi.x, min(hi.y, min(hi.z, best_dist))))
                                {
                                    hit_bits |= (((slot_tri_b >> 8u) & 255u) << (((slot_xor_b >> 8u) & 255u) & 31u));
                                }
                            }
                            {
                                float3 lo = (float3(float((child_b_lo0 >> 16u) & 255u), float((child_b_lo1 >> 16u) & 255u), float((child_b_lo2 >> 16u) & 255u)) * extent_inv) + rel;
                                float3 hi = (float3(float((child_b_hi0 >> 16u) & 255u), float((child_b_hi1 >> 16u) & 255u), float((child_b_hi2 >> 16u) & 255u)) * extent_inv) + rel;
                                if (max(lo.x, max(lo.y, max(lo.z, 9.9999997473787516355514526367188e-05f))) <= min(hi.x, min(hi.y, min(hi.z, best_dist))))
                                {
                                    hit_bits |= (((slot_tri_b >> 16u) & 255u) << (((slot_xor_b >> 16u) & 255u) & 31u));
                                }
                            }
                            {
                                float3 lo = (float3(float((child_b_lo0 >> 24u) & 255u), float((child_b_lo1 >> 24u) & 255u), float((child_b_lo2 >> 24u) & 255u)) * extent_inv) + rel;
                                float3 hi = (float3(float((child_b_hi0 >> 24u) & 255u), float((child_b_hi1 >> 24u) & 255u), float((child_b_hi2 >> 24u) & 255u)) * extent_inv) + rel;
                                if (max(lo.x, max(lo.y, max(lo.z, 9.9999997473787516355514526367188e-05f))) <= min(hi.x, min(hi.y, min(hi.z, best_dist))))
                                {
                                    hit_bits |= (((slot_tri_b >> 24u) & 255u) << (((slot_xor_b >> 24u) & 255u) & 31u));
                                }
                            }

                            uint2 next_ref = uint2(node_data.data[1].x, child_mask);
                            next_ref.y = (hit_bits & 4278190080u) | ((node_data.data[0].w >> 24u) & 255u);
                            child_ref = next_ref;
                            stack_entry = uint2(node_data.data[1].y, hit_bits & 16777215u);
                            push_depth = depth_after;
                        }
                        else
                        {
                            child_ref = uint2(0u, 0u);
                            stack_entry = node_ref;
                            push_depth = stack_depth;
                        }

                        // 处理本节点携带的三角形栈 (把命中的槽位三角逐个求交)。
                        uint2 tri_stack = stack_entry;
                        float3 tri_normal = best_normal;
                        float tri_dist = best_dist;
                        int tri_idx = best_tri;
                        for (;;)
                        {
                            uint stack_bits = tri_stack.y;
                            if (stack_bits != 0u)
                            {
                                uint stack_bit = firstbithigh(stack_bits);
                                uint2 tri_entry = tri_stack;
                                tri_entry.y = stack_bits & (~(1u << (stack_bit & 31u)));
                                uint tri_addr = tri_stack.x + stack_bit;

                                // 加载打包三角形: 3 顶点 (float) + 3 半精度边。
                                PackedTriangle tri;
                                [unroll]
                                for (int tri_i = 0; tri_i < 3; tri_i++)
                                {
                                    tri.v[tri_i] = rt_triangles.Load<float>(tri_i * 4 + tri_addr * 24 + 0);
                                }
                                [unroll]
                                for (int tri_e = 0; tri_e < 3; tri_e++)
                                {
                                    tri.e[tri_e] = rt_triangles.Load<uint>(tri_e * 4 + tri_addr * 24 + 12);
                                }

                                float3 edge_x = float3(unpack_half2x16((tri.e[0] >> 16u) & 65535u).x, unpack_half2x16((tri.e[1] >> 16u) & 65535u).x, unpack_half2x16((tri.e[2] >> 16u) & 65535u).x);
                                float3 edge_y = float3(unpack_half2x16(tri.e[0] & 65535u).x, unpack_half2x16(tri.e[1] & 65535u).x, unpack_half2x16(tri.e[2] & 65535u).x);

                                float3 test_normal;
                                float test_dist;
                                int test_tri;
                                do
                                {
                                // Moller-Trumbore 求交: 计算行列式 / 距离。
                                    float3 c1 = cross(ray_dir, edge_y);
                                    float det = dot(edge_x, c1);
                                    float3 s = ray_origin - float3(tri.v[0], tri.v[1], tri.v[2]);
                                    float3 c2 = cross(s, edge_x);
                                    float dist = dot(edge_y, c2) / det;

                                    bool closer;
                                    if (dist >= 0.0f)
                                    {
                                        closer = dist < tri_dist;
                                    }
                                    else
                                    {
                                        closer = false;
                                    }
                                    if (closer)
                                    {
                                // 重心坐标检查 (含小 epsilon) 是否命中。
                                        float3 bary_num;
                                        bary_num.x = dot(s, c1);
                                        bary_num.y = dot(ray_dir, c2);
                                        float2 bary = bary_num.xy / det.xx;
                                        float3 bary3 = float3(bary.x, bary.y, bary_num.z);
                                        bary3.z = (1.0f - bary.x) - bary.y;
                                        if (all(bool3(bary3.x >= (-9.9999997473787516355514526367188e-05f).xxx.x, bary3.y >= (-9.9999997473787516355514526367188e-05f).xxx.y, bary3.z >= (-9.9999997473787516355514526367188e-05f).xxx.z)))
                                        {
                                            test_dist = dist;
                                            test_normal = cross(edge_x, edge_y);
                                            test_tri = int(tri_addr);
                                            break;
                                        }
                                    }
                                    test_dist = tri_dist;
                                    test_normal = tri_normal;
                                    test_tri = tri_idx;
                                    break;
                                } while (false);

                                tri_stack = tri_entry;
                                tri_normal = test_normal;
                                tri_dist = test_dist;
                                tri_idx = test_tri;
                                continue;
                            }
                            else
                            {
                                break;
                            }
                        }

                        uint2 next_node;
                        int next_depth;
                        // 若子节点无更多子项, 则弹栈或结束本次遍历。
                        if ((child_ref.y & 4278190080u) == 0u)
                        {
                            if (push_depth == 0)
                            {
                                hit_normal = tri_normal;
                                hit_dist = tri_dist;
                                best_tri = tri_idx;
                                break;
                            }
                            int popped;
                            uint2 popped_entry;
                            do
                            {
                                popped = push_depth - 1;
                                if (popped >= 4)
                                {
                                    popped_entry = rt_lds_stack[gl_LocalInvocationIndex + uint((push_depth - 5) * 64)];
                                    break;
                                }
                                popped_entry = ray_stack[popped];
                                break;
                            } while (false);
                            next_node = popped_entry;
                            next_depth = popped;
                        }
                        else
                        {
                            next_node = child_ref;
                            next_depth = push_depth;
                        }
                        node_ref = next_node;
                        best_dist = tri_dist;
                        stack_depth = next_depth;
                        iter_count++;
                        best_normal = tri_normal;
                        best_tri = tri_idx;
                        continue;
                    }
                    else
                    {
                        hit_normal = best_normal;
                        hit_dist = best_dist;
                        break;
                    }
                }

                // 判定是否命中三角形 (best_tri >= 0 且距离 < 100000)。
                bool hit;
                if (best_tri != (-1))
                {
                    hit = hit_dist < 100000.0f;
                }
                else
                {
                    hit = false;
                }
                if (hit)
                {
                    hit_normal = hit_normal;
                    hit_kind = 3u;
                    hit_tri_index = uint(best_tri);
                    hit_dist = hit_dist;
                    break;
                }
                hit_normal = float3(0.0f, 1.0f, 0.0f);
                hit_kind = 0u;
                hit_tri_index = 0u;
                hit_dist = 100000.0f;
                break;
            } while (false);

            // 与地形高度图求交 (软件光线步进 + 二分)。
            uint terrain_kind;
            float terrain_dist;
            do
            {
                // 地形包围盒 [-65,65]x[0,24]x[-65,65] 的 slab 求交。
                float3 box_t0 = (float3(65.0f, 24.0f, 65.0f) - ray_origin) / ray_dir;
                float3 box_t1 = (float3(-65.0f, 0.0f, -65.0f) - ray_origin) / ray_dir;
                float3 box_min = min(box_t0, box_t1);
                float3 box_max = max(box_t0, box_t1);
                float t_enter = max(max(box_min.x, box_min.y), box_min.z);
                float t_exit = min(min(box_max.x, box_max.y), box_max.z);
                bool box_hit;
                if (t_enter <= t_exit)
                {
                    box_hit = t_exit >= 0.0f;
                }
                else
                {
                    box_hit = false;
                }
                if (!box_hit)
                {
                    terrain_kind = hit_kind;
                    terrain_dist = hit_dist;
                    break;
                }

                float t_lo = max(t_enter, 0.0f);
                // 在包围盒内 32 步步进地形。
                float t_step = (t_exit - t_lo) * 0.03125f;
                float t_cur = t_lo;
                float t_cross;
                uint step_count = 0u;
                bool stepped;
                for (;;)
                {
                    stepped = step_count < 32u;
                    if (stepped)
                    {
                        t_cross = t_cur + t_step;
                        float3 p = ray_origin + (ray_dir * t_cross);
                        if (p.y < lerp(-2.5f, 24.0f, visual_terrain_heightmap.SampleLevel(sampler_lnc, (p.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x))
                        {
                            break;
                        }
                        step_count++;
                        t_cur = t_cross;
                        continue;
                    }
                    else
                    {
                        t_cross = t_exit;
                        break;
                    }
                }
                bool below;
                if (stepped)
                {
                    below = t_cur < hit_dist;
                }
                else
                {
                    below = false;
                }
                bool terrain_hit;
                if (below)
                {
                    terrain_hit = t_cur < t_cross;
                }
                else
                {
                    terrain_hit = false;
                }
                if (terrain_hit)
                {
                    float t_hi = t_cross;
                    float t_lo2 = t_cur;
                    // 二分 8 次精确定位地形表面。
                    for (uint bisect = 0u; bisect < 8u; )
                    {
                        float t_mid = (t_lo2 + t_hi) * 0.5f;
                        float3 p = ray_origin + (ray_dir * t_mid);
                        bool under = p.y < lerp(-2.5f, 24.0f, visual_terrain_heightmap.SampleLevel(sampler_lnc, (p.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                        t_hi = under ? t_mid : t_hi;
                        t_lo2 = under ? t_lo2 : t_mid;
                        bisect++;
                        continue;
                    }
                    if (t_lo2 < hit_dist)
                    {
                        float2 uv = ((ray_origin + (ray_dir * t_lo2)).xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                        // 精细地形 (地表高于视觉地形 0.1) 记为 1, 其余泥土记为 2。
                        terrain_kind = ((lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, uv, 0.0f).x) + 0.100000001490116119384765625f) > lerp(-2.5f, 24.0f, visual_terrain_heightmap.SampleLevel(sampler_lnc, uv, 0.0f).x)) ? 1u : 2u;
                        terrain_dist = t_lo2;
                        break;
                    }
                    else
                    {
                        terrain_kind = hit_kind;
                        terrain_dist = hit_dist;
                        break;
                    }
                }
                else
                {
                    terrain_kind = hit_kind;
                    terrain_dist = hit_dist;
                    break;
                }
            } while (false);

            // 无地形命中: 进入天空 / 夜间着色。
            if (terrain_kind == 0u)
            {
                // 上半球: 天空渐变 + 假光源光晕。
                if (ray_dir.y > 0.0f)
                {
                    // 按方向混合晨昏渐变 (蓝 -> 橙 -> 白)。
                    float sun_dot = dot(ray_dir, fake_light_dir.xyz);
                    float3 light_ramp = lerp(lerp(float3(0.20000000298023223876953125f, 0.4000000059604644775390625f, 1.0f), float3(1.0f, 0.4000000059604644775390625f, 0.20000000298023223876953125f), ((sun_dot * 0.5f) + 0.5f).xxx), float3(1.0f, 1.0f, 1.0f), (ray_dir.y * ray_dir.y).xxx);
                    float3 fake_light = ((fake_light_color.xyz * pow(max(0.0f, sun_dot), 12.0f)) + (fake_light2_color.xyz * pow(max(0.0f, dot(ray_dir, fake_light2_dir.xyz)), 12.0f))) * 3.0f;
                    float3 tint_rcp = max(float3(0.0f, 0.0f, 0.0f), float3(1.0f, 1.0f, 1.0f) - (indirect_light_tint.xyz * 0.920000016689300537109375f));
                    float3 luma = float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f);
                    float3 sky_gradient = lerp(sky_low_color.xyz * 1.0f, sky_high_color.xyz, pow(clamp(1.0f - pow(1.0f - clamp(ray_dir.y + 0.20000000298023223876953125f, 0.0f, 1.0f), 14.0f), 0.0f, 1.0f), 0.64999997615814208984375f).xxx) * 1.099999904632568359375f;
                    float3 sky_term = sky_gradient * (light_ramp / dot(light_ramp, luma).xxx);
                    float3 fake_term = max(float3(0.0f, 0.0f, 0.0f), lerp((tint_rcp / dot(tint_rcp, luma).xxx) * dot(fake_light, luma), fake_light, 0.854700863361358642578125f.xxx) * 0.833333313465118408203125f.xxx);
                    float3 sky_add = sky_term + fake_term;
                    float4 sky_light = float4(radiance.xyz + (throughput * sky_add), radiance.w);
                    result = float4(sky_light.x, sky_light.y, sky_light.z, radiance.w);
                    break;
                }
                else
                {
                    // 夜间主题: 草绿色环境光按日夜平滑曲线调节。
                    if (glade_theme_id == 2u)
                    {
                        float3 night = radiance.xyz + ((((lerp(grass_1.xyz, grass_2.xyz, 0.1500000059604644775390625f.xxx) * 1.2000000476837158203125f) * 0.550000011920928955078125f) * 5.0f) * (1.0f - (smoothstep(0.4830000102519989013671875f, 0.50499999523162841796875f, time_of_day) - smoothstep(0.839999973773956298828125f, 0.910000026226043701171875f, time_of_day))));
                        result = float4(night.x, night.y, night.z, radiance.w);
                        break;
                    }
                    result = radiance;
                    break;
                }
            }
            else
            {
                float3 hit_pos = ray_origin + (ray_dir * terrain_dist);
                // 命中地形: 读取地表光照纹理并散射新方向 (最后一次反弹直接取光照)。
                bool sharp_terrain = terrain_kind == 1u;
                bool is_terrain;
                if (sharp_terrain)
                {
                    is_terrain = true;
                }
                else
                {
                    is_terrain = terrain_kind == 2u;
                }

                if (is_terrain)
                {
                    float4 new_radiance;
                    float3 new_dir;
                    float3 new_throughput;
                    float3 new_origin;
                    if ((bounce + 1u) == 3u)
                    {
                        float3 lit = radiance.xyz + (throughput * terrain_lighting_texture.SampleLevel(sampler_llc, (hit_pos.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).xyz);
                        result = float4(lit.x, lit.y, lit.z, radiance.w);
                        break;
                    }
                    else
                    {
                        float3 albedo;
                        if (sharp_terrain)
                        {
                        // 草皮反照率 (主题 4 混入 50% 灰)。
                            float3 grass = (lerp(grass_1.xyz, grass_2.xyz, 0.1500000059604644775390625f.xxx) * 1.2000000476837158203125f) * 0.550000011920928955078125f;
                            if (glade_theme_id == 4u)
                            {
                                albedo = lerp(grass, float3(0.699999988079071044921875f, 0.699999988079071044921875f, 0.699999988079071044921875f), 0.5f.xxx);
                            }
                            else
                            {
                                albedo = grass;
                            }
                        }
                        else
                        {
                            albedo = float3(0.122000001370906829833984375f, 0.122000001370906829833984375f, 0.08900000154972076416015625f);
                        }

                        // 太阳方向 (相机 2 视锥)。
                        float3 sun_dir = mul(view_constants[2].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz;
                        float3 to_view = -ray_dir;
                        bool3 override_albedo = ((render_overrides & 1u) != 0u).xxx;
                        float3 albedo_eff = float3(override_albedo.x ? 0.5f.xxx.x : albedo.x, override_albedo.y ? 0.5f.xxx.y : albedo.y, override_albedo.z ? 0.5f.xxx.z : albedo.z);
                        float3 neg_sun = -sun_dir;

                        // PCSS 软阴影: 先 5 次采样估计平均遮挡深度, 再按半影宽度做 PCF。
                        float2 shadow_factor;
                        do
                        {
                            // 命中点投影到太阳相机; 超出范围则无阴影。
                            float4 sun_clip = mul(view_constants[2].world_to_sample, float4(hit_pos, 1.0f));
                            if (sun_clip.z > 1.0f)
                            {
                                shadow_factor = 0.0f.xx;
                                break;
                            }
                            float2 uv0 = sun_clip.xy;
                            float2 uv0_abs = abs(uv0);
                            bool outer = any(bool2(uv0_abs.x > 0.9833333492279052734375f.xx.x, uv0_abs.y > 0.9833333492279052734375f.xx.y));
                            float2 uv1 = outer ? uv0 * 0.5f : uv0;
                            float texel_scale = outer ? 5.0f : 10.0f;
                            float sample_scale = outer ? 0.5f : 1.0f;
                            float2 uv1_abs = abs(uv1);
                            float2 margin = 1.0f.xx - (0.000833333353511989116668701171875f.xx * (outer ? 10.0f : 20.0f));
                            bool inner_out = any(bool2(uv1_abs.x > margin.x, uv1_abs.y > margin.y));
                            float texel_s2;
                            float2 uv2;
                            float sample_s2;
                            if (inner_out)
                            {
                                texel_s2 = outer ? 1.4285714626312255859375f : 2.857142925262451171875f;
                                uv2 = uv1 * 0.2857142984867095947265625f;
                                sample_s2 = outer ? 0.14285714924335479736328125f : 0.2857142984867095947265625f;
                            }
                            else
                            {
                                texel_s2 = texel_scale;
                                uv2 = uv1;
                                sample_s2 = sample_scale;
                            }
                            float cascade = float(inner_out ? 2u : uint(outer));
                            uint cascade_idx = 2u + uint(cascade);
                            float cos_term = neg_sun.y;
                            float4 biased_clip = float4(uv2.x, uv2.y, sun_clip.z, sun_clip.w) + mul(view_constants[cascade_idx].world_to_sample, float4((float3(0.0f, 1.0f, 0.0f) * (0.066666670143604278564453125f * clamp(1.0f - cos_term, 0.0f, 1.0f))) / sample_s2.xxx, 0.0f));
                            float2 shadow_uv = (biased_clip.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                            float3 shadow_sample_pos = float3(shadow_uv.x, shadow_uv.y, biased_clip.z);
                            float bias = max((inner_out ? (outer ? 0.2100000083446502685546875f : 0.10500000417232513427734375f) : (outer ? 0.60000002384185791015625f : 0.300000011920928955078125f)) * (1.0f - abs(cos_term)), 0.02500000037252902984619140625f);
                            float spiral_seed = float((uint((pixel.x * 11) + (pixel.y * 7)) + (frame_index * 5u)) % 64u) * 1.61803400516510009765625f;
                            uint2 pixel_u = uint2(pixel);
                            float frame_f = 5.588237762451171875f * float(frame_index % 64u);
                            float hash_v = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(pixel_u.x) + frame_f)) + (0.005837149918079376220703125f * (float(pixel_u.y) + frame_f))));

                            float4 light_dir_proj = mul(view_constants[cascade_idx].world_to_sample, float4(0.0f, 1.0f, 0.0f, 0.0f));

                            float blocker_depth = 0.0f;
                            int blocker_count = 0;
                            for (int tap = 0; tap < 5; blocker_depth = blocker_depth, blocker_count = blocker_count, tap++)
                            {
                                float tap_f = float(tap);
                                float angle = (tap_f + spiral_seed) * 2.3999631404876708984375f;
                                float3 off_pos = shadow_sample_pos + float3((((float2(cos(angle), sin(angle)) * (tap_f + hash_v)) * 0.20000000298023223876953125f.xx) * texel_s2) * 0.000833333353511989116668701171875f.xx, bias * (-0.0009765625f));
                                float4 shadow_tap = shadow_map.SampleLevel(sampler_shadow_nearest, float3(off_pos.xy, cascade), 0.0f);
                                float shadow_depth = shadow_tap.x;
                                if (off_pos.z > shadow_depth)
                                {
                                    blocker_depth = blocker_depth + shadow_depth;
                                    blocker_count = blocker_count + 1;
                                }
                            }
                            float avg_blocker = blocker_depth / float(blocker_count);
                            if (blocker_count == 0)
                            {
                                shadow_factor = 0.0f.xx;
                                break;
                            }
                            // 半影宽度随遮挡深度 / 级联分辨率变化。
                            float penumbra_uv = max(((biased_clip.z - avg_blocker) * ((inner_out ? (outer ? 24.380954742431640625f : 48.76190948486328125f) : (outer ? 85.33333587646484375f : 170.6666717529296875f)) * pow(10.0f, smoothstep(0.0f, 0.25f, fog_tint_thickness.w)))) / avg_blocker, 0.0f);
                            float filter_scale = (penumbra_uv / pow(pow(penumbra_uv / (texel_s2 - 1.0f), 5.0f) + 1.0f, 0.20000000298023223876953125f)) + 1.0f;

                            float ldp_z = light_dir_proj.z;
                            float2 slope_bias = (((light_dir_proj.xy * float(int(sign(ldp_z)))) / max(0.1500000059604644775390625f, abs(ldp_z)).xx) * 0.0009765625f.xx) * 2.0f;

                            // PCF 10 抽头螺旋采样累加阴影因子。
                            float2 pcf_acc = float2(0.0f, undefined_f);
                            for (int tap2 = 0; tap2 < 10; tap2++)
                            {
                                float tap2_f = float(tap2);
                                float angle2 = (tap2_f + spiral_seed) * 2.3999631404876708984375f;
                                float2 off2 = ((float2(cos(angle2), sin(angle2)) * (tap2_f + hash_v)) * 0.100000001490116119384765625f.xx) * filter_scale;
                                float3 cmp_pos = shadow_sample_pos + float3(off2 * 0.000833333353511989116668701171875f.xx, bias * (-0.0009765625f));
                                pcf_acc += float2(1.0f - shadow_map.SampleCmpLevelZero(sampler_shadow_linear, float3(cmp_pos.xy, cascade), cmp_pos.z + min(0.0f, dot(slope_bias, off2))), undefined_f);
                            }
                            shadow_factor = pcf_acc * 0.100000001490116119384765625f;
                            break;
                        } while (false);

                        // 地形视为水平平面, 计算 GGX 光照。
                        float3 geom_normal = normalize(float3(0.0f, 1.0f, 0.0f));
                        float ndl = clamp(dot(geom_normal, sun_dir), 0.0f, 1.0f);
                        float ndh = clamp(dot(sun_dir, normalize(sun_dir + to_view)), 0.0f, 1.0f);
                        float ndh_term = ((1.28000009059906005859375f * ndh) * ndh) + (-0.5f);
                        float one_minus_ndl = 1.0f - ndl;
                        float om_ndl2 = one_minus_ndl * one_minus_ndl;
                        float one_minus_ndv = 1.0f - max(dot(geom_normal, to_view), 9.9999997473787516355514526367188e-05f);
                        float om_ndv2 = one_minus_ndv * one_minus_ndv;

                        // 地形镜面高光 (单侧剔除)。
                        float3 terrain_spec;
                        do
                        {
                            float sun_up = sun_dir.y;
                            bool two_sided;
                            if (sun_up <= 0.0f)
                            {
                                two_sided = true;
                            }
                            else
                            {
                                two_sided = to_view.y <= 0.0f;
                            }
                            if (two_sided)
                            {
                                terrain_spec = float3(0.0f, 0.0f, 0.0f);
                                break;
                            }
                            float3 h = normalize(to_view + sun_dir);
                            float h_up = h.y;
                            float om_dh = max(0.0f, 1.0f - dot(h, sun_dir));
                            float om_dh2 = om_dh * om_dh;
                            float v_up = to_view.y;
                            float h2_term = ((h_up * h_up) * (-0.5903999805450439453125f)) + 1.0f;
                            terrain_spec = ((lerp(float3(0.039999999105930328369140625f, 0.039999999105930328369140625f, 0.039999999105930328369140625f), float3(1.0f, 1.0f, 1.0f), ((om_dh2 * om_dh2) * om_dh).xxx) * (((2.0f * sun_up) * v_up) / ((sun_up * sqrt((((v_up * (-0.4096000492572784423828125f)) + v_up) * v_up) + 0.4096000492572784423828125f)) + (v_up * sqrt((((sun_up * (-0.4096000492572784423828125f)) + sun_up) * sun_up) + 0.4096000492572784423828125f))))) * (0.4096000492572784423828125f / ((3.1415927410125732421875f * h2_term) * h2_term))) / ((4.0f * v_up) * sun_up).xxx;
                            break;
                        } while (false);

                        float3 sun_light = radiance.xyz + (throughput * ((((sun_color.xyz * ((albedo_eff * (((1.0f + (ndh_term * ((om_ndl2 * om_ndl2) * one_minus_ndl))) * (1.0f + (ndh_term * ((om_ndv2 * om_ndv2) * one_minus_ndv)))) * 0.3183098733425140380859375f)) + terrain_spec)) * ndl) * 1.0f) * (1.0f - shadow_factor.x)));
                        new_radiance = float4(sun_light.x, sun_light.y, sun_light.z, radiance.w);
                        new_throughput = throughput * float3(override_albedo.x ? 0.5f.xxx.x : albedo_eff.x, override_albedo.y ? 0.5f.xxx.y : albedo_eff.y, override_albedo.z ? 0.5f.xxx.z : albedo_eff.z);
                        new_origin = hit_pos + float3(0.0f, 0.001000000047497451305389404296875f, 0.0f);
                        float rnd1 = 1.0f - (2.0f * rnd.x);
                        float rnd2 = sqrt(max(0.0f, 1.0f - (rnd1 * rnd1)));
                        float rnd_ang = 6.283185482025146484375f * rnd.y;
                        new_dir = normalize(float3(0.0f, 1.0f, 0.0f) + float3(cos(rnd_ang) * rnd2, sin(rnd_ang) * rnd2, rnd1));
                    }
                    radiance = new_radiance;
                    ray_dir = new_dir;
                    throughput = new_throughput;
                    ray_origin = new_origin;
                }
                else
                {
                    float4 new_radiance2;
                    float3 new_dir2;
                    float3 new_throughput2;
                    float3 new_origin2;
                    if (terrain_kind == 3u)
                    {
                        // 网格命中: 背面 (法线朝射线) 剔除。
                        if (dot(hit_normal, ray_dir) > 0.0f)
                        {
                            result = radiance;
                            break;
                        }
                        float3 albedo2;
                        do
                        {
                            // 墙: 经 wall_color_ids 索引到调色板颜色。
                            if ((rt_triangle_meta.Load<uint>(hit_tri_index * 4 + 0) & 16777216u) != 0u)
                            {
                                int wall_id = int(wall_color_ids.Load<uint>((rt_triangle_meta.Load<uint>(hit_tri_index * 4 + 0) & 16777215u) * 4 + 0));
                                float3 wall_color;
                                do
                                {
                                    if (uint(wall_id) < 13u)
                                    {
                                        wall_color = WALL_COLORS[wall_id];
                                        break;
                                    }
                                    else
                                    {
                                        wall_color = float3(1.0f, 0.0f, 1.0f);
                                        break;
                                    }
                                } while (false);
                                albedo2 = wall_color;
                                break;
                            }
                            else
                            {
                                // 非墙: 打包 RGB 反照率 (srgb 解包)。
                                uint packed_rgb = rt_triangle_meta.Load<uint>(hit_tri_index * 4 + 0) & 16777215u;
                                float3 rgb = float3(float(packed_rgb & 255u) * 0.0039215688593685626983642578125f, float((packed_rgb >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((packed_rgb >> 16u) & 255u) * 0.0039215688593685626983642578125f);
                                albedo2 = rgb * rgb;
                                break;
                            }
                        } while (false);

                        // 网格 PCSS 阴影 (与地形相同的级联 / 半影逻辑)。
                        float3 sun_dir2 = mul(view_constants[2].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz;
                        float3 to_view2 = -ray_dir;
                        float3 tri_norm0 = normalize(hit_normal);
                        bool3 override_albedo2 = ((render_overrides & 1u) != 0u).xxx;
                        float3 albedo2_eff = float3(override_albedo2.x ? 0.5f.xxx.x : albedo2.x, override_albedo2.y ? 0.5f.xxx.y : albedo2.y, override_albedo2.z ? 0.5f.xxx.z : albedo2.z);

                        float2 shadow_factor2;
                        do
                        {
                            float4 sun_clip2 = mul(view_constants[2].world_to_sample, float4(hit_pos, 1.0f));
                            if (sun_clip2.z > 1.0f)
                            {
                                shadow_factor2 = 0.0f.xx;
                                break;
                            }
                            float2 u0 = sun_clip2.xy;
                            float2 u0a = abs(u0);
                            bool outer2 = any(bool2(u0a.x > 0.9833333492279052734375f.xx.x, u0a.y > 0.9833333492279052734375f.xx.y));
                            float2 u1 = outer2 ? u0 * 0.5f : u0;
                            float texel2 = outer2 ? 5.0f : 10.0f;
                            float sample2 = outer2 ? 0.5f : 1.0f;
                            float2 u1a = abs(u1);
                            float2 margin2 = 1.0f.xx - (0.000833333353511989116668701171875f.xx * (outer2 ? 10.0f : 20.0f));
                            bool inner2 = any(bool2(u1a.x > margin2.x, u1a.y > margin2.y));
                            float texel3;
                            float2 u2;
                            float sample3;
                            if (inner2)
                            {
                                texel3 = outer2 ? 1.4285714626312255859375f : 2.857142925262451171875f;
                                u2 = u1 * 0.2857142984867095947265625f;
                                sample3 = outer2 ? 0.14285714924335479736328125f : 0.2857142984867095947265625f;
                            }
                            else
                            {
                                texel3 = texel2;
                                u2 = u1;
                                sample3 = sample2;
                            }
                            float cascade2 = float(inner2 ? 2u : uint(outer2));
                            uint cascade2_idx = 2u + uint(cascade2);
                            float ndl2 = dot(tri_norm0, -sun_dir2);
                            float4 biased2 = float4(u2.x, u2.y, sun_clip2.z, sun_clip2.w) + mul(view_constants[cascade2_idx].world_to_sample, float4((tri_norm0 * (0.066666670143604278564453125f * clamp(1.0f - ndl2, 0.0f, 1.0f))) / sample3.xxx, 0.0f));
                            float2 shadow_uv2 = (biased2.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                            float3 shadow_pos2 = float3(shadow_uv2.x, shadow_uv2.y, biased2.z);
                            float bias2 = max((inner2 ? (outer2 ? 0.2100000083446502685546875f : 0.10500000417232513427734375f) : (outer2 ? 0.60000002384185791015625f : 0.300000011920928955078125f)) * (1.0f - abs(ndl2)), 0.02500000037252902984619140625f);
                            float spiral2 = float((uint((pixel.x * 11) + (pixel.y * 7)) + (frame_index * 5u)) % 64u) * 1.61803400516510009765625f;
                            uint2 pixel_u2 = uint2(pixel);
                            float frame_f2 = 5.588237762451171875f * float(frame_index % 64u);
                            float hash2 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(pixel_u2.x) + frame_f2)) + (0.005837149918079376220703125f * (float(pixel_u2.y) + frame_f2))));
                            float4 light_dir_proj2 = mul(view_constants[cascade2_idx].world_to_sample, float4(tri_norm0, 0.0f));
                            float blocker2 = 0.0f;
                            int blocker_count2 = 0;
                            for (int tap3 = 0; tap3 < 5; blocker2 = blocker2, blocker_count2 = blocker_count2, tap3++)
                            {
                                float tap3_f = float(tap3);
                                float angle3 = (tap3_f + spiral2) * 2.3999631404876708984375f;
                                float3 off3 = shadow_pos2 + float3((((float2(cos(angle3), sin(angle3)) * (tap3_f + hash2)) * 0.20000000298023223876953125f.xx) * texel3) * 0.000833333353511989116668701171875f.xx, bias2 * (-0.0009765625f));
                                float4 tap_shadow = shadow_map.SampleLevel(sampler_shadow_nearest, float3(off3.xy, cascade2), 0.0f);
                                float td = tap_shadow.x;
                                if (off3.z > td)
                                {
                                    blocker2 = blocker2 + td;
                                    blocker_count2 = blocker_count2 + 1;
                                }
                            }
                            float avg2 = blocker2 / float(blocker_count2);
                            if (blocker_count2 == 0)
                            {
                                shadow_factor2 = 0.0f.xx;
                                break;
                            }
                            float penumbra2 = max(((biased2.z - avg2) * ((inner2 ? (outer2 ? 24.380954742431640625f : 48.76190948486328125f) : (outer2 ? 85.33333587646484375f : 170.6666717529296875f)) * pow(10.0f, smoothstep(0.0f, 0.25f, fog_tint_thickness.w)))) / avg2, 0.0f);
                            float filter2 = (penumbra2 / pow(pow(penumbra2 / (texel3 - 1.0f), 5.0f) + 1.0f, 0.20000000298023223876953125f)) + 1.0f;
                            float ldp2 = light_dir_proj2.z;
                            float2 slope2 = (((light_dir_proj2.xy * float(int(sign(ldp2)))) / max(0.1500000059604644775390625f, abs(ldp2)).xx) * 0.0009765625f.xx) * 2.0f;
                            float2 pcf2 = float2(0.0f, undefined_f);
                            for (int tap4 = 0; tap4 < 10; tap4++)
                            {
                                float tap4_f = float(tap4);
                                float angle4 = (tap4_f + spiral2) * 2.3999631404876708984375f;
                                float2 off4 = ((float2(cos(angle4), sin(angle4)) * (tap4_f + hash2)) * 0.100000001490116119384765625f.xx) * filter2;
                                float3 cmp2 = shadow_pos2 + float3(off4 * 0.000833333353511989116668701171875f.xx, bias2 * (-0.0009765625f));
                                pcf2 += float2(1.0f - shadow_map.SampleCmpLevelZero(sampler_shadow_linear, float3(cmp2.xy, cascade2), cmp2.z + min(0.0f, dot(slope2, off4))), undefined_f);
                            }
                            shadow_factor2 = pcf2 * 0.100000001490116119384765625f;
                            break;
                        } while (false);

                        float3 tri_norm_n = normalize(tri_norm0);
                        float ndl3 = clamp(dot(tri_norm_n, sun_dir2), 0.0f, 1.0f);
                        float ndh3 = clamp(dot(sun_dir2, normalize(sun_dir2 + to_view2)), 0.0f, 1.0f);
                        float ndh3_term = ((1.28000009059906005859375f * ndh3) * ndh3) + (-0.5f);
                        float om_nl = 1.0f - ndl3;
                        float om_nl2 = om_nl * om_nl;
                        float om_nv = 1.0f - max(dot(tri_norm_n, to_view2), 9.9999997473787516355514526367188e-05f);
                        float om_nv2 = om_nv * om_nv;

                        // 网格镜面高光 (单侧剔除)。
                        float3 mesh_spec;
                        do
                        {
                            float ndotl = dot(tri_norm0, sun_dir2);
                            bool mesh_two_sided;
                            if (ndotl <= 0.0f)
                            {
                                mesh_two_sided = true;
                            }
                            else
                            {
                                mesh_two_sided = dot(tri_norm0, to_view2) <= 0.0f;
                            }
                            if (mesh_two_sided)
                            {
                                mesh_spec = float3(0.0f, 0.0f, 0.0f);
                                break;
                            }
                            float3 h2 = normalize(to_view2 + sun_dir2);
                            float ndh2 = dot(tri_norm0, h2);
                            float om_dh2v = max(0.0f, 1.0f - dot(h2, sun_dir2));
                            float om_dh2v2 = om_dh2v * om_dh2v;
                            float ndv = dot(tri_norm0, to_view2);
                            float h2_term = ((ndh2 * ndh2) * (-0.5903999805450439453125f)) + 1.0f;
                            mesh_spec = ((lerp(float3(0.039999999105930328369140625f, 0.039999999105930328369140625f, 0.039999999105930328369140625f), float3(1.0f, 1.0f, 1.0f), ((om_dh2v2 * om_dh2v2) * om_dh2v).xxx) * (((2.0f * ndotl) * ndv) / ((ndotl * sqrt((((ndv * (-0.4096000492572784423828125f)) + ndv) * ndv) + 0.4096000492572784423828125f)) + (ndv * sqrt((((ndotl * (-0.4096000492572784423828125f)) + ndotl) * ndotl) + 0.4096000492572784423828125f))))) * (0.4096000492572784423828125f / ((3.1415927410125732421875f * h2_term) * h2_term))) / ((4.0f * ndv) * ndotl).xxx;
                            break;
                        } while (false);

                        float3 mesh_light = radiance.xyz + (throughput * ((((sun_color.xyz * ((albedo2_eff * (((1.0f + (ndh3_term * ((om_nl2 * om_nl2) * om_nl))) * (1.0f + (ndh3_term * ((om_nv2 * om_nv2) * om_nv)))) * 0.3183098733425140380859375f)) + mesh_spec)) * ndl3) * 1.0f) * (1.0f - shadow_factor2.x)));
                        float rnd1b = 1.0f - (2.0f * rnd.x);
                        float rnd2b = sqrt(max(0.0f, 1.0f - (rnd1b * rnd1b)));
                        float rnd_ang2 = 6.283185482025146484375f * rnd.y;
                        new_radiance2 = float4(mesh_light.x, mesh_light.y, mesh_light.z, radiance.w);
                        new_dir2 = normalize(tri_norm0 + float3(cos(rnd_ang2) * rnd2b, sin(rnd_ang2) * rnd2b, rnd1b));
                        new_throughput2 = throughput * float3(override_albedo2.x ? 0.5f.xxx.x : albedo2_eff.x, override_albedo2.y ? 0.5f.xxx.y : albedo2_eff.y, override_albedo2.z ? 0.5f.xxx.z : albedo2_eff.z);
                        new_origin2 = hit_pos + (tri_norm0 * 0.001000000047497451305389404296875f);
                    }
                    else
                    {
                        new_radiance2 = radiance;
                        new_dir2 = ray_dir;
                        new_throughput2 = throughput;
                        new_origin2 = ray_origin;
                    }
                    radiance = new_radiance2;
                    ray_dir = new_dir2;
                    throughput = new_throughput2;
                    ray_origin = new_origin2;
                }
            }
            bounce++;
            continue;
        }
        else
        {
            result = radiance;
            break;
        }
    }

    // 有相机重投影: 直接写结果; 否则把新结果按 25% 比例 lerp 进上一帧输出。
    if (length(view_constants[0].clip_to_prev_clip[3].xyz) > 0.001000000047497451305389404296875f)
    {
        output_tex[gl_GlobalInvocationID.xy] = result.xyz;
    }
    else
    {
        output_tex[gl_GlobalInvocationID.xy] = lerp(output_tex[gl_GlobalInvocationID.xy].xyz, result.xyz, 0.25f.xxx);
    }
}
