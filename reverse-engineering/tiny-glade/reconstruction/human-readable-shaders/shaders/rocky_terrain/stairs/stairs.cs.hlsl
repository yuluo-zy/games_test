// PATCHED: modules rs.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
// 【文件用途】楼梯生成计算着色器。对地形高度场做逐单元 marching-squares 等高线提取:
//   等高线网格单元只有至少一个角点在岩石上(rocky_terrain.x clamp>0.2)、贴近路径
//   (smoothstep(path_mask.x)>0.99)且不在水中(water_mask.y<0.5)时才保留。对每个楼梯
//   等高层(lerp(0.8,10,level/16)):由 4 位角点状态掩码选择穿越边(marching_squares.inc),
//   find_root(inc/find_root.hlsl,经 place_stairs.inc)把每条边二分到目标高度,
//   place_stairs.inc 把一个 Stair{Affine3Packed} 写入 __alias_stairs_buffer 并递增
//   __alias_draw_commands[draw_indirect_buffer_offset].instanceCount;碎石复用该缓冲,
//   位于 pebble_buffer_offset,使用 pebble_indirect_draw_idx 命令。
// 【入口】cs_main | local_size [8,8,1] | gid uint3 (SV_DispatchThreadID)
// 【绑定】sampler_llb s4、sampler_default s9、__alias_draw_commands u13(SSBO)、
//   $Globals b14(UBO)、terrain_heightmap t15、rocky_terrain t17、path_mask t18、
//   water_mask t19、__alias_stairs_buffer u20(SSBO)。
// 【变体】TGV_0 + compute 阶段:使用下方从 oracle bin 提起(转译)的 comp_main;
//   否则使用上方手写重建段。两个实现行为一致。
// 【状态】O(算法);残留 do-while/phi/switch + int3-Load 代码生成差异为 dxc 版本产物;
//   OpSource/OpString/OpLine 数量漂移来自重建 inc 层的额外 include 边(inc/* 只读)。
// 重写说明:手写段补充中文注释;提起段(TGV_0 内)保持代码逐字,仅添加块级中文注释。
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/rocky_terrain/stairs/stairs.cs.hlsl
// target: _rocky_terrain_stairs_stairs.cs.hlsl_cs_main_0.bin
// evidence: spirv-cross (_rocky_terrain_stairs_stairs.cs.hlsl_cs_main_0_0.hlsl.cross) + SPIR-V disasm:
//   per-cell marching-squares contouring over the terrain height field. A contouring-grid cell is
//   kept only when at least one corner sits on rock (rocky_terrain.x clamp>0.2), near a path
//   (smoothstep(path_mask.x)>0.99) and off water (water_mask.y<0.5). For each stair iso-level
//   (lerp(0.8,10,level/16)) the 4-bit corner-state mask selects the crossing edges (marching_squares.inc),
//   find_root (inc/find_root.hlsl, via place_stairs.inc) bisects each edge to the target height, and
//   place_stairs.inc writes a Stair{Affine3Packed} into __alias_stairs_buffer and bumps
//   __alias_draw_commands[draw_indirect_buffer_offset].instanceCount; pebbles reuse the buffer at
//   pebble_buffer_offset and the pebble_indirect_draw_idx command. Binding/entry/LocalSize verbatim;
//   the two SSBOs are plain RWStructuredBuffer (target module emits only OpCapability Shader, no
//   SPV_EXT_descriptor_indexing, so NO [[vk::ext_capability(5302)]] here — verified from the bin).
// bindings: sampler_llb s4, sampler_default s9, __alias_draw_commands u13 (SSBO), $Globals b14 (UBO),
//   terrain_heightmap t15, rocky_terrain t17, path_mask t18, water_mask t19, __alias_stairs_buffer u20 (SSBO)
// entry: cs_main | local_size [8,8,1] | gid uint3 (SV_DispatchThreadID)
// status: O (algorithm); residual do-while/phi/switch + int3-Load codegen = dxc-version artifacts;
//   OpSource/OpString/OpLine count drift = reconstructed inc-layer extra include edges (inc/* read-only)

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/draw_indirect.hlsl"
#include "inc/draw_indirect_types.hlsl"
Texture2D<float4> terrain_heightmap : register(t15, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/uv.hlsl"
#include "inc/rocky_terrain.inc.hlsl"
#include "inc/texture_utils.hlsl"
#include "paths/path_mask_sampler.inc.hlsl"
#include "water/water_mask_sampler.inc.hlsl"

struct Stair
{
    Affine3Packed transform;
};

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);
RWStructuredBuffer<Stair> __alias_stairs_buffer : register(u20, space0);

Texture2D<float4> rocky_terrain : register(t17, space0);
Texture2D<float4> path_mask : register(t18, space0);
Texture2D<float4> water_mask : register(t19, space0);

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 rocky_terrain_dims;
    uint2 path_mask_dims;
    uint2 water_mask_dims;
    uint2 contouring_grid_dims;
    uint pebble_indirect_draw_idx;
    uint pebble_buffer_offset;
};

// terrain_height(float2) is now provided by inc/terrain_heightmap.inc.hlsl
// (adjudicated inc-fix #3); it samples the global terrain_heightmap texture
// declared below at register t15.

// Bounds-checked rocky-terrain sample; outside the [0,1]^2 uv domain it reads 0.
// 带边界检查的岩石采样:越出 [0,1]^2 uv 域时读 0。
float4 sample_rocky(float2 world_xz)
{
    float2 uv = world_xz * ROCKY_TERRAIN_SAMPLE_SCALE + 0.5f;
    if (uv.x < 0.0f || uv.x > 1.0f || uv.y < 0.0f || uv.y > 1.0f)
        return 0.0f;
    return rocky_terrain.SampleLevel(sampler_default, uv, 0.0f);
}

// Bounds-checked water-mask sample; outside the [0,1]^2 uv domain it reads 0.
// 带边界检查的水掩码采样:越出 [0,1]^2 uv 域时读 0。
float water_mask_value(float2 world_xz)
{
    float2 uv = world_xz * ROCKY_TERRAIN_SAMPLE_SCALE + 0.5f;
    if (uv.x < 0.0f || uv.x > 1.0f || uv.y < 0.0f || uv.y > 1.0f)
        return 0.0f;
    return water_mask.SampleLevel(sampler_default, uv, 0.0f).y;
}

// The three stair include files (this shader's contouring logic), dependency
// chain: isolines.inc -> marching_squares.inc -> place_stairs.inc -> inc/find_root.hlsl.
#include "isolines.inc.hlsl"

[numthreads(8, 8, 1)]
// 等高线网格主入口:先剔除没有角点落在"岩石 + 路径 + 无水"区域的单元,
// 再按单元四角高度调用 place_stair_isolines 生成楼梯台阶。
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    if (gl_GlobalInvocationID.x > contouring_grid_dims.x ||
        gl_GlobalInvocationID.y > contouring_grid_dims.y)
        return;

    float2 grid_dims = float2(contouring_grid_dims);
    float2 center = (float2(gl_GlobalInvocationID.x + 0.5f, gl_GlobalInvocationID.y + 0.5f) / grid_dims - 0.5f) * 130.0f;
    float2 half = 65.0f / grid_dims;

    float2 corners[4];
    corners[0] = center + float2(-half.x, -half.y);
    corners[1] = center + float2(half.x, -half.y);
    corners[2] = center + float2(half.x, half.y);
    corners[3] = center + float2(-half.x, half.y);

    int eligible = 0;
    for (int i = 0; i < 4; i++)
    {
        float rocky = clamp(sample_rocky(corners[i]).x, 0.0f, 1.0f);
        bool near_path = smoothstep(0.075f, 0.125f, path_mask.SampleLevel(sampler_llb, corners[i] * ROCKY_TERRAIN_SAMPLE_SCALE + 0.5f, 0.0f).x) > 0.99f;
        bool off_water = water_mask_value(corners[i]) < 0.5f;
        if (rocky > 0.2f && near_path && off_water)
            eligible++;
    }
// 没有任何角点满足"岩石 + 路径 + 无水"条件,整个单元不生成楼梯
    if (eligible == 0)
        return;

    float4 corner_heights;
    for (int i = 0; i < 4; i++)
        corner_heights[i] = terrain_height(corners[i]);

// 对该单元按各等高层(0.8..10)放置楼梯台阶
    place_stair_isolines(corners, corner_heights);
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _rocky_terrain_stairs_stairs.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct DrawIndexedIndirectCommand
{
    uint count;
    uint instanceCount;
    uint firstIndex;
    uint vertexOffset;
    uint firstInstance;
};

struct Affine3Packed
{
    float data[12];
};

struct Stair
{
    Affine3Packed transform;
};

static uint v132;

RWByteAddressBuffer __alias_draw_commands : register(u13, space0);
cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset : packoffset(c0);
    uint2 terrain_heightmap_dims : packoffset(c0.y);
    uint2 visual_terrain_heightmap_dims : packoffset(c1);
    uint2 rocky_terrain_dims : packoffset(c1.z);
    uint2 path_mask_dims : packoffset(c2);
    uint2 water_mask_dims : packoffset(c2.z);
    uint2 contouring_grid_dims : packoffset(c3);
    uint pebble_indirect_draw_idx : packoffset(c3.z);
    uint pebble_buffer_offset : packoffset(c3.w);
};

RWByteAddressBuffer __alias_stairs_buffer : register(u20, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t15, space0);
Texture2D<float4> rocky_terrain : register(t17, space0);
Texture2D<float4> path_mask : register(t18, space0);
Texture2D<float4> water_mask : register(t19, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

// 【提起段】以下为从 _rocky_terrain_stairs_stairs.cs.hlsl_cs_main_0.bin 经
//   spirv-cross 提起的行为精确代码;名字是 SPIR-V id(_NN 为 SSA 临时变量)。
//   结构 = 手写 cs_main + isolines/marching_squares/place_stairs/find_root 全部内联:
//   外层 do-while 内的 if(v165) 是单元越界检查;对每个等高线单元计算四角高度
//   (v150) 与等高层窗口(v339/v344),然后按 v355 层号循环;循环内 v364 是 4 位角点
//   状态掩码,随后按掩码进入各 case(v405=1/14、v414=2/13、v423=3/12、v432=4/11、
//   v389==5 鞍形、v445=6/9、v454=7/8、v389==10 鞍形)。每个 case 内部结构相同:
//   两条穿越边的 find_root 二分(for 循环 2 趟)得到端点 -> 提升为 float3 ->
//   取中点 -> 剔除测试(岩石 < 0.01、两端都不贴近路径、长度 < 0.2)
//   -> 高度图 GatherRed 梯度 + cross 偏移 -> T(pos-0.5y)*R*S 变换 -> InterlockedAdd
//   取槽位写 Stair -> 概率碎石(>0.85)。v346/v349/v351/v353 是跨层循环传递的
//   哈希随机状态。
void comp_main()
{
    do
    {
// 单元越界检查:gid.x/y 超出 contouring_grid_dims 则跳过本单元
        bool v165;
        if (!(gl_GlobalInvocationID.x > contouring_grid_dims.x))
        {
            v165 = gl_GlobalInvocationID.y > contouring_grid_dims.y;
        }
        else
        {
            v165 = true;
        }
        if (v165)
        {
            break;
        }
        float v170 = float(contouring_grid_dims.x);
        float v176 = float(contouring_grid_dims.y);
        float2 v181 = (float2((float(gl_GlobalInvocationID.x) + 0.5f) / v170, (float(gl_GlobalInvocationID.y) + 0.5f) / v176) - 0.5f.xx) * 130.0f.xx;
        float2 v182 = 65.0f.xx / float2(v170, v176);
        float v183 = v182.x;
        float v184 = -v183;
        float v185 = v182.y;
        float v186 = -v185;
        float2 v149[4];
        v149[0] = v181 + float2(v184, v186);
        v149[1] = v181 + float2(v183, v186);
        v149[2] = v181 + v182;
        v149[3] = v181 + float2(v184, v185);
        int v199;
        v199 = 0;
        int v200;
        for (int v202 = 0; v202 < 4; v199 = v200, v202++)
        {
            float v240;
            do
            {
                float2 v212 = (v149[v202] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                float v213 = v212.x;
                bool v219;
                if (!(v213 < 0.0f))
                {
                    v219 = v213 > 1.0f;
                }
                else
                {
                    v219 = true;
                }
                bool v225;
                if (!v219)
                {
                    v225 = v212.y < 0.0f;
                }
                else
                {
                    v225 = true;
                }
                bool v231;
                if (!v225)
                {
                    v231 = v212.y > 1.0f;
                }
                else
                {
                    v231 = true;
                }
                if (v231)
                {
                    v240 = 0.0f;
                    break;
                }
                else
                {
                    v240 = rocky_terrain.SampleLevel(sampler_default, v212, 0.0f).x;
                    break;
                }
                break; // unreachable workaround
            } while(false);
            float v282;
            do
            {
                float2 v254 = (v149[v202] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                float v255 = v254.x;
                bool v261;
                if (!(v255 < 0.0f))
                {
                    v261 = v255 > 1.0f;
                }
                else
                {
                    v261 = true;
                }
                bool v267;
                if (!v261)
                {
                    v267 = v254.y < 0.0f;
                }
                else
                {
                    v267 = true;
                }
                bool v273;
                if (!v267)
                {
                    v273 = v254.y > 1.0f;
                }
                else
                {
                    v273 = true;
                }
                if (v273)
                {
                    v282 = 0.0f;
                    break;
                }
                else
                {
                    v282 = water_mask.SampleLevel(sampler_default, v254, 0.0f).y;
                    break;
                }
                break; // unreachable workaround
            } while(false);
            if (((int(clamp(v240, 0.0f, 1.0f) > 0.20000000298023223876953125f) & int(smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (v149[v202] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) > 0.9900000095367431640625f)) & int(v282 < 0.5f)) != 0)
            {
                v200 = v199 + 1;
            }
            else
            {
                v200 = v199;
            }
        }
        if (v199 == 0)
        {
            break;
        }
        float4 v150 = 0.0f.xxxx;
        for (int v300 = 0; v300 < 4; )
        {
            v150[uint(v300)] = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v149[v300] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
            v300++;
            continue;
        }
// 计算本单元等高层窗口:由四角高度(min/max)映射到层号 [0,16]
        int v339 = min(int(ceil((min(min(v150.x, v150.y), min(v150.z, v150.w)) - 0.900000035762786865234375f) * 1.84782612323760986328125f)), 16);
        int v344 = max(int(floor((max(max(v150.x, v150.y), max(v150.z, v150.w)) + (-0.699999988079071044921875f)) * 1.84782612323760986328125f)), 0);
        uint v347;
        uint v350;
        bool v352;
        bool v354;
        uint v346 = 0u;
        uint v349 = 0u;
        bool v351 = false;
        bool v353 = false;
        int v355 = v339;
// 对等高层窗口内的每一层循环:计算该层高度 v362,生成 4 位角点掩码,分发到 case
        for (; v355 <= v344; v346 = v347, v349 = v350, v351 = v352, v353 = v354, v355++)
        {
            float v362 = lerp(0.800000011920928955078125f, 10.0f, float(v355) * 0.0625f);
            uint v364;
            v364 = 0u;
            uint v365;
            for (int v367 = 0; v367 < 4; v364 = v365, v367++)
            {
                if (lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v149[v367] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) >= v362)
                {
                    v365 = v364 | (8u >> (uint(v367) & 31u));
                }
                else
                {
                    v365 = v364;
                }
            }
            int v389 = int(v364);
            bool v396;
            if (!(v389 == 0))
            {
                v396 = v389 == 15;
            }
            else
            {
                v396 = true;
            }
            if (v396)
            {
                v347 = v346;
                v350 = v349;
                v352 = v351;
                v354 = v353;
            }
            else
            {
                bool v405;
                if (!(v389 == 1))
                {
                    v405 = v389 == 14;
                }
                else
                {
                    v405 = true;
                }
                uint v4587;
                uint v4588;
                bool v4589;
                bool v4590;
// case 1 / 14(单角点 0 及其补):等离线连接边 0-3 与边 3-2
                if (v405)
                {
                    uint v4583;
                    uint v4584;
                    bool v4585;
                    bool v4586;
                    do
                    {
                        float v4180 = -v362;
                        float2 v4191 = (v149[3] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                        float v4198;
                        float v4201;
                        float2 v4203;
                        float2 v4205;
                        v4198 = v4180 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v149[0] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                        v4201 = v4180 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, v4191, 0.0f).x);
                        v4203 = v149[0];
                        v4205 = v149[3];
                        for (uint v4207 = 0u; v4207 < 2u; )
                        {
                            float2 v4211 = v4203 + v4205;
                            float2 v4212 = v4211 * 0.5f;
                            float v4219 = v4180 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v4212 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                            float v4220 = v4219 * v4198;
                            bool v4221 = v4220 < 0.0f;
                            bool2 v4222 = v4221.xx;
                            v4198 = v4221 ? v4198 : v4219;
                            v4201 = v4221 ? v4219 : v4201;
                            v4203 = float2(v4222.x ? v4203.x : v4212.x, v4222.y ? v4203.y : v4212.y);
                            v4205 = float2(v4222.x ? v4212.x : v4205.x, v4222.y ? v4212.y : v4205.y);
                            v4207++;
                            continue;
                        }
                        float2 v4227 = lerp(v4205, v4203, clamp(v4201 / (v4201 - v4198), 0.0f, 1.0f).xx);
                        float v4228 = v4227.x;
                        float3 v4230 = float3(v4228, v362, v4227.y);
                        float v4245;
                        float v4248;
                        float2 v4250;
                        float2 v4252;
                        v4245 = v4180 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, v4191, 0.0f).x);
                        v4248 = v4180 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v149[2] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                        v4250 = v149[3];
                        v4252 = v149[2];
                        for (uint v4254 = 0u; v4254 < 2u; )
                        {
                            float2 v4258 = v4250 + v4252;
                            float2 v4259 = v4258 * 0.5f;
                            float v4266 = v4180 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v4259 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                            float v4267 = v4266 * v4245;
                            bool v4268 = v4267 < 0.0f;
                            bool2 v4269 = v4268.xx;
                            v4245 = v4268 ? v4245 : v4266;
                            v4248 = v4268 ? v4266 : v4248;
                            v4250 = float2(v4269.x ? v4250.x : v4259.x, v4269.y ? v4250.y : v4259.y);
                            v4252 = float2(v4269.x ? v4259.x : v4252.x, v4269.y ? v4259.y : v4252.y);
                            v4254++;
                            continue;
                        }
                        float2 v4283;
                        float v4284;
                        bool v4286;
                        float2 v4274 = lerp(v4252, v4250, clamp(v4248 / (v4248 - v4245), 0.0f, 1.0f).xx);
                        float v4276 = v4274.y;
                        float3 v4277 = float3(v4274.x, v362, v4276);
                        float3 v4278 = lerp(v4230, v4277, 0.5f.xxx);
                        float v4310;
                        do
                        {
                            v4283 = (v4278.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                            v4284 = v4283.x;
                            v4286 = !(v4284 < 0.0f);
                            bool v4290;
                            if (v4286)
                            {
                                v4290 = v4284 > 1.0f;
                            }
                            else
                            {
                                v4290 = true;
                            }
                            bool v4296;
                            if (!v4290)
                            {
                                v4296 = v4283.y < 0.0f;
                            }
                            else
                            {
                                v4296 = true;
                            }
                            bool v4302;
                            if (!v4296)
                            {
                                v4302 = v4283.y > 1.0f;
                            }
                            else
                            {
                                v4302 = true;
                            }
                            if (v4302)
                            {
                                v4310 = 0.0f;
                                break;
                            }
                            else
                            {
                                v4310 = rocky_terrain.SampleLevel(sampler_default, v4283, 0.0f).x;
                                break;
                            }
                            break; // unreachable workaround
                        } while(false);
                        if (v4310 < 0.00999999977648258209228515625f)
                        {
                            v4583 = v346;
                            v4584 = v349;
                            v4585 = v351;
                            v4586 = v353;
                            break;
                        }
                        bool v4334;
                        if (smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (v4230.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f)
                        {
                            v4334 = smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (v4277.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f;
                        }
                        else
                        {
                            v4334 = false;
                        }
                        if (v4334)
                        {
                            v4583 = v346;
                            v4584 = v349;
                            v4585 = v351;
                            v4586 = v353;
                            break;
                        }
                        float v4337 = distance(v4230, v4277);
                        if (v4337 < 0.20000000298023223876953125f)
                        {
                            v4583 = v346;
                            v4584 = v349;
                            v4585 = v351;
                            v4586 = v353;
                            break;
                        }
                        float4 v4342 = terrain_heightmap.GatherRed(sampler_default, v4283, int2(0, 0));
                        float v4347 = v4342.z;
                        float v4348 = v4342.y;
                        float v4350 = v4342.w;
                        float v4351 = v4342.x;
                        float2 v4360 = ((float2((v4347 + v4348) - (v4350 + v4351), (v4351 + v4348) - (v4350 + v4347)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(terrain_heightmap_dims));
                        float v4393;
                        do
                        {
                            bool v4373;
                            if (v4286)
                            {
                                v4373 = v4284 > 1.0f;
                            }
                            else
                            {
                                v4373 = true;
                            }
                            bool v4379;
                            if (!v4373)
                            {
                                v4379 = v4283.y < 0.0f;
                            }
                            else
                            {
                                v4379 = true;
                            }
                            bool v4385;
                            if (!v4379)
                            {
                                v4385 = v4283.y > 1.0f;
                            }
                            else
                            {
                                v4385 = true;
                            }
                            if (v4385)
                            {
                                v4393 = 0.0f;
                                break;
                            }
                            else
                            {
                                v4393 = rocky_terrain.SampleLevel(sampler_default, v4283, 0.0f).x;
                                break;
                            }
                            break; // unreachable workaround
                        } while(false);
                        float v4419;
                        do
                        {
                            bool v4399;
                            if (v4286)
                            {
                                v4399 = v4284 > 1.0f;
                            }
                            else
                            {
                                v4399 = true;
                            }
                            bool v4405;
                            if (!v4399)
                            {
                                v4405 = v4283.y < 0.0f;
                            }
                            else
                            {
                                v4405 = true;
                            }
                            bool v4411;
                            if (!v4405)
                            {
                                v4411 = v4283.y > 1.0f;
                            }
                            else
                            {
                                v4411 = true;
                            }
                            if (v4411)
                            {
                                v4419 = 0.0f;
                                break;
                            }
                            else
                            {
                                v4419 = rocky_terrain.SampleLevel(sampler_default, v4283, 0.0f).z;
                                break;
                            }
                            break; // unreachable workaround
                        } while(false);
                        float3 v4423 = cross(normalize(float3(0.0f, v4360.y, 1.0f)), normalize(float3(1.0f, v4360.x, 0.0f))) * lerp(0.0f, 0.300000011920928955078125f, clamp(v4393 + v4419, 0.0f, 1.0f));
                        float3 v4424 = v4278 - v4423;
                        float3 v4428 = normalize(v4277 - v4230);
                        uint v4433 = uint(((v4228 + v362) + v4276) * 100.0f);
                        float4x4 v4451 = transpose(float4x4(float4(v4428, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(cross(v4428, float3(0.0f, 1.0f, 0.0f)), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
                        float4x4 v4453 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v4424.x), float4(0.0f, 1.0f, 0.0f, v4424.y - 0.5f), float4(0.0f, 0.0f, 1.0f, v4424.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(v4451, float4x4(float4(v4337, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.800000011920928955078125f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                        uint v4458;
                        __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, v4458);
                        uint v4472 = uint(int(v4458));
                        float v4474[12] = { v4453[0].x, v4453[1].x, v4453[2].x, v4453[0].y, v4453[1].y, v4453[2].y, v4453[0].z, v4453[1].z, v4453[2].z, v4453[0].w, v4453[1].w, v4453[2].w };
                        Affine3Packed v4475 = { v4474 };
                        Stair v4476 = { v4475 };
                        [unroll]
                        for (int _20ident = 0; _20ident < 12; _20ident++)
                        {
                            __alias_stairs_buffer.Store<float>(_20ident * 4 + v4472 * 48 + 0, v4476.transform.data[_20ident]);
                        }
                        uint v4477 = v4433 ^ 23784u;
                        uint v4479 = v4477 + (v4477 << 10u);
                        uint v4481 = v4479 ^ (v4479 >> 6u);
                        uint v4483 = v4481 + (v4481 << 3u);
                        uint v4485 = v4483 ^ (v4483 >> 11u);
                        bool v4488 = v353 ? v353 : true;
                        uint v4489 = v353 ? v349 : 8388607u;
                        bool v4490 = v351 ? v351 : true;
                        uint v4491 = v351 ? v346 : 1065353216u;
                        uint v4579;
                        uint v4580;
                        bool v4581;
                        bool v4582;
                        if ((asfloat(((v4485 + (v4485 << 15u)) & v4489) | v4491) - 1.0f) > 0.85000002384185791015625f)
                        {
                            uint v4499 = v4433 ^ 90455u;
                            uint v4501 = v4499 + (v4499 << 10u);
                            uint v4503 = v4501 ^ (v4501 >> 6u);
                            uint v4505 = v4503 + (v4503 << 3u);
                            uint v4507 = v4505 ^ (v4505 >> 11u);
                            bool v4510 = v4488 ? v4488 : true;
                            uint v4511 = v4488 ? v4489 : 8388607u;
                            bool v4512 = v4490 ? v4490 : true;
                            uint v4513 = v4490 ? v4491 : 1065353216u;
                            float v4518 = lerp(0.20000000298023223876953125f, 0.4000000059604644775390625f, asfloat(((v4507 + (v4507 << 15u)) & v4511) | v4513) - 1.0f);
                            uint v4519 = v4433 ^ 3845u;
                            uint v4521 = v4519 + (v4519 << 10u);
                            uint v4523 = v4521 ^ (v4521 >> 6u);
                            uint v4525 = v4523 + (v4523 << 3u);
                            uint v4527 = v4525 ^ (v4525 >> 11u);
                            uint v4531 = v4510 ? v4511 : 8388607u;
                            uint v4533 = v4512 ? v4513 : 1065353216u;
                            float3 v4540 = lerp(v4230, v4277, (asfloat(((v4527 + (v4527 << 15u)) & v4531) | v4533) - 1.0f).xxx) - v4423;
                            uint v4544;
                            __alias_draw_commands.InterlockedAdd(pebble_indirect_draw_idx * 20 + 4, 1u, v4544);
                            uint v4549 = pebble_buffer_offset + uint(int(v4544));
                            float4x4 v4562 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v4540.x), float4(0.0f, 1.0f, 0.0f, v4540.y), float4(0.0f, 0.0f, 1.0f, v4540.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(v4451, float4x4(float4(v4518, 0.0f, 0.0f, 0.0f), float4(0.0f, v4518, 0.0f, 0.0f), float4(0.0f, 0.0f, v4518, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                            float v4576[12] = { v4562[0].x, v4562[1].x, v4562[2].x, v4562[0].y, v4562[1].y, v4562[2].y, v4562[0].z, v4562[1].z, v4562[2].z, v4562[0].w, v4562[1].w, v4562[2].w };
                            Affine3Packed v4577 = { v4576 };
                            Stair v4578 = { v4577 };
                            [unroll]
                            for (int _21ident = 0; _21ident < 12; _21ident++)
                            {
                                __alias_stairs_buffer.Store<float>(_21ident * 4 + v4549 * 48 + 0, v4578.transform.data[_21ident]);
                            }
                            v4579 = v4533;
                            v4580 = v4531;
                            v4581 = v4512 ? v4512 : true;
                            v4582 = v4510 ? v4510 : true;
                        }
                        else
                        {
                            v4579 = v4491;
                            v4580 = v4489;
                            v4581 = v4490;
                            v4582 = v4488;
                        }
                        v4583 = v4579;
                        v4584 = v4580;
                        v4585 = v4581;
                        v4586 = v4582;
                        break;
                    } while(false);
                    v4587 = v4583;
                    v4588 = v4584;
                    v4589 = v4585;
                    v4590 = v4586;
                }
                else
                {
                    bool v414;
                    if (!(v389 == 2))
                    {
                        v414 = v389 == 13;
                    }
                    else
                    {
                        v414 = true;
                    }
                    uint v4172;
                    uint v4173;
                    bool v4174;
                    bool v4175;
// case 2 / 13(单角点 1 及其补):等离线连接边 1-2 与边 3-2
                    if (v414)
                    {
                        uint v4168;
                        uint v4169;
                        bool v4170;
                        bool v4171;
                        do
                        {
                            float v3765 = -v362;
                            float2 v3776 = (v149[2] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                            float v3783;
                            float v3786;
                            float2 v3788;
                            float2 v3790;
                            v3783 = v3765 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v149[1] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                            v3786 = v3765 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, v3776, 0.0f).x);
                            v3788 = v149[1];
                            v3790 = v149[2];
                            for (uint v3792 = 0u; v3792 < 2u; )
                            {
                                float2 v3796 = v3788 + v3790;
                                float2 v3797 = v3796 * 0.5f;
                                float v3804 = v3765 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v3797 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                float v3805 = v3804 * v3783;
                                bool v3806 = v3805 < 0.0f;
                                bool2 v3807 = v3806.xx;
                                v3783 = v3806 ? v3783 : v3804;
                                v3786 = v3806 ? v3804 : v3786;
                                v3788 = float2(v3807.x ? v3788.x : v3797.x, v3807.y ? v3788.y : v3797.y);
                                v3790 = float2(v3807.x ? v3797.x : v3790.x, v3807.y ? v3797.y : v3790.y);
                                v3792++;
                                continue;
                            }
                            float2 v3812 = lerp(v3790, v3788, clamp(v3786 / (v3786 - v3783), 0.0f, 1.0f).xx);
                            float v3813 = v3812.x;
                            float3 v3815 = float3(v3813, v362, v3812.y);
                            float v3830;
                            float v3833;
                            float2 v3835;
                            float2 v3837;
                            v3830 = v3765 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v149[3] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                            v3833 = v3765 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, v3776, 0.0f).x);
                            v3835 = v149[3];
                            v3837 = v149[2];
                            for (uint v3839 = 0u; v3839 < 2u; )
                            {
                                float2 v3843 = v3835 + v3837;
                                float2 v3844 = v3843 * 0.5f;
                                float v3851 = v3765 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v3844 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                float v3852 = v3851 * v3830;
                                bool v3853 = v3852 < 0.0f;
                                bool2 v3854 = v3853.xx;
                                v3830 = v3853 ? v3830 : v3851;
                                v3833 = v3853 ? v3851 : v3833;
                                v3835 = float2(v3854.x ? v3835.x : v3844.x, v3854.y ? v3835.y : v3844.y);
                                v3837 = float2(v3854.x ? v3844.x : v3837.x, v3854.y ? v3844.y : v3837.y);
                                v3839++;
                                continue;
                            }
                            float2 v3868;
                            float v3869;
                            bool v3871;
                            float2 v3859 = lerp(v3837, v3835, clamp(v3833 / (v3833 - v3830), 0.0f, 1.0f).xx);
                            float v3861 = v3859.y;
                            float3 v3862 = float3(v3859.x, v362, v3861);
                            float3 v3863 = lerp(v3815, v3862, 0.5f.xxx);
                            float v3895;
                            do
                            {
                                v3868 = (v3863.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                v3869 = v3868.x;
                                v3871 = !(v3869 < 0.0f);
                                bool v3875;
                                if (v3871)
                                {
                                    v3875 = v3869 > 1.0f;
                                }
                                else
                                {
                                    v3875 = true;
                                }
                                bool v3881;
                                if (!v3875)
                                {
                                    v3881 = v3868.y < 0.0f;
                                }
                                else
                                {
                                    v3881 = true;
                                }
                                bool v3887;
                                if (!v3881)
                                {
                                    v3887 = v3868.y > 1.0f;
                                }
                                else
                                {
                                    v3887 = true;
                                }
                                if (v3887)
                                {
                                    v3895 = 0.0f;
                                    break;
                                }
                                else
                                {
                                    v3895 = rocky_terrain.SampleLevel(sampler_default, v3868, 0.0f).x;
                                    break;
                                }
                                break; // unreachable workaround
                            } while(false);
                            if (v3895 < 0.00999999977648258209228515625f)
                            {
                                v4168 = v346;
                                v4169 = v349;
                                v4170 = v351;
                                v4171 = v353;
                                break;
                            }
                            bool v3919;
                            if (smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (v3815.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f)
                            {
                                v3919 = smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (v3862.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f;
                            }
                            else
                            {
                                v3919 = false;
                            }
                            if (v3919)
                            {
                                v4168 = v346;
                                v4169 = v349;
                                v4170 = v351;
                                v4171 = v353;
                                break;
                            }
                            float v3922 = distance(v3815, v3862);
                            if (v3922 < 0.20000000298023223876953125f)
                            {
                                v4168 = v346;
                                v4169 = v349;
                                v4170 = v351;
                                v4171 = v353;
                                break;
                            }
                            float4 v3927 = terrain_heightmap.GatherRed(sampler_default, v3868, int2(0, 0));
                            float v3932 = v3927.z;
                            float v3933 = v3927.y;
                            float v3935 = v3927.w;
                            float v3936 = v3927.x;
                            float2 v3945 = ((float2((v3932 + v3933) - (v3935 + v3936), (v3936 + v3933) - (v3935 + v3932)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(terrain_heightmap_dims));
                            float v3978;
                            do
                            {
                                bool v3958;
                                if (v3871)
                                {
                                    v3958 = v3869 > 1.0f;
                                }
                                else
                                {
                                    v3958 = true;
                                }
                                bool v3964;
                                if (!v3958)
                                {
                                    v3964 = v3868.y < 0.0f;
                                }
                                else
                                {
                                    v3964 = true;
                                }
                                bool v3970;
                                if (!v3964)
                                {
                                    v3970 = v3868.y > 1.0f;
                                }
                                else
                                {
                                    v3970 = true;
                                }
                                if (v3970)
                                {
                                    v3978 = 0.0f;
                                    break;
                                }
                                else
                                {
                                    v3978 = rocky_terrain.SampleLevel(sampler_default, v3868, 0.0f).x;
                                    break;
                                }
                                break; // unreachable workaround
                            } while(false);
                            float v4004;
                            do
                            {
                                bool v3984;
                                if (v3871)
                                {
                                    v3984 = v3869 > 1.0f;
                                }
                                else
                                {
                                    v3984 = true;
                                }
                                bool v3990;
                                if (!v3984)
                                {
                                    v3990 = v3868.y < 0.0f;
                                }
                                else
                                {
                                    v3990 = true;
                                }
                                bool v3996;
                                if (!v3990)
                                {
                                    v3996 = v3868.y > 1.0f;
                                }
                                else
                                {
                                    v3996 = true;
                                }
                                if (v3996)
                                {
                                    v4004 = 0.0f;
                                    break;
                                }
                                else
                                {
                                    v4004 = rocky_terrain.SampleLevel(sampler_default, v3868, 0.0f).z;
                                    break;
                                }
                                break; // unreachable workaround
                            } while(false);
                            float3 v4008 = cross(normalize(float3(0.0f, v3945.y, 1.0f)), normalize(float3(1.0f, v3945.x, 0.0f))) * lerp(0.0f, 0.300000011920928955078125f, clamp(v3978 + v4004, 0.0f, 1.0f));
                            float3 v4009 = v3863 - v4008;
                            float3 v4013 = normalize(v3862 - v3815);
                            uint v4018 = uint(((v3813 + v362) + v3861) * 100.0f);
                            float4x4 v4036 = transpose(float4x4(float4(v4013, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(cross(v4013, float3(0.0f, 1.0f, 0.0f)), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
                            float4x4 v4038 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v4009.x), float4(0.0f, 1.0f, 0.0f, v4009.y - 0.5f), float4(0.0f, 0.0f, 1.0f, v4009.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(v4036, float4x4(float4(v3922, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.800000011920928955078125f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                            uint v4043;
                            __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, v4043);
                            uint v4057 = uint(int(v4043));
                            float v4059[12] = { v4038[0].x, v4038[1].x, v4038[2].x, v4038[0].y, v4038[1].y, v4038[2].y, v4038[0].z, v4038[1].z, v4038[2].z, v4038[0].w, v4038[1].w, v4038[2].w };
                            Affine3Packed v4060 = { v4059 };
                            Stair v4061 = { v4060 };
                            [unroll]
                            for (int _22ident = 0; _22ident < 12; _22ident++)
                            {
                                __alias_stairs_buffer.Store<float>(_22ident * 4 + v4057 * 48 + 0, v4061.transform.data[_22ident]);
                            }
                            uint v4062 = v4018 ^ 23784u;
                            uint v4064 = v4062 + (v4062 << 10u);
                            uint v4066 = v4064 ^ (v4064 >> 6u);
                            uint v4068 = v4066 + (v4066 << 3u);
                            uint v4070 = v4068 ^ (v4068 >> 11u);
                            bool v4073 = v353 ? v353 : true;
                            uint v4074 = v353 ? v349 : 8388607u;
                            bool v4075 = v351 ? v351 : true;
                            uint v4076 = v351 ? v346 : 1065353216u;
                            uint v4164;
                            uint v4165;
                            bool v4166;
                            bool v4167;
                            if ((asfloat(((v4070 + (v4070 << 15u)) & v4074) | v4076) - 1.0f) > 0.85000002384185791015625f)
                            {
                                uint v4084 = v4018 ^ 90455u;
                                uint v4086 = v4084 + (v4084 << 10u);
                                uint v4088 = v4086 ^ (v4086 >> 6u);
                                uint v4090 = v4088 + (v4088 << 3u);
                                uint v4092 = v4090 ^ (v4090 >> 11u);
                                bool v4095 = v4073 ? v4073 : true;
                                uint v4096 = v4073 ? v4074 : 8388607u;
                                bool v4097 = v4075 ? v4075 : true;
                                uint v4098 = v4075 ? v4076 : 1065353216u;
                                float v4103 = lerp(0.20000000298023223876953125f, 0.4000000059604644775390625f, asfloat(((v4092 + (v4092 << 15u)) & v4096) | v4098) - 1.0f);
                                uint v4104 = v4018 ^ 3845u;
                                uint v4106 = v4104 + (v4104 << 10u);
                                uint v4108 = v4106 ^ (v4106 >> 6u);
                                uint v4110 = v4108 + (v4108 << 3u);
                                uint v4112 = v4110 ^ (v4110 >> 11u);
                                uint v4116 = v4095 ? v4096 : 8388607u;
                                uint v4118 = v4097 ? v4098 : 1065353216u;
                                float3 v4125 = lerp(v3815, v3862, (asfloat(((v4112 + (v4112 << 15u)) & v4116) | v4118) - 1.0f).xxx) - v4008;
                                uint v4129;
                                __alias_draw_commands.InterlockedAdd(pebble_indirect_draw_idx * 20 + 4, 1u, v4129);
                                uint v4134 = pebble_buffer_offset + uint(int(v4129));
                                float4x4 v4147 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v4125.x), float4(0.0f, 1.0f, 0.0f, v4125.y), float4(0.0f, 0.0f, 1.0f, v4125.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(v4036, float4x4(float4(v4103, 0.0f, 0.0f, 0.0f), float4(0.0f, v4103, 0.0f, 0.0f), float4(0.0f, 0.0f, v4103, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                float v4161[12] = { v4147[0].x, v4147[1].x, v4147[2].x, v4147[0].y, v4147[1].y, v4147[2].y, v4147[0].z, v4147[1].z, v4147[2].z, v4147[0].w, v4147[1].w, v4147[2].w };
                                Affine3Packed v4162 = { v4161 };
                                Stair v4163 = { v4162 };
                                [unroll]
                                for (int _23ident = 0; _23ident < 12; _23ident++)
                                {
                                    __alias_stairs_buffer.Store<float>(_23ident * 4 + v4134 * 48 + 0, v4163.transform.data[_23ident]);
                                }
                                v4164 = v4118;
                                v4165 = v4116;
                                v4166 = v4097 ? v4097 : true;
                                v4167 = v4095 ? v4095 : true;
                            }
                            else
                            {
                                v4164 = v4076;
                                v4165 = v4074;
                                v4166 = v4075;
                                v4167 = v4073;
                            }
                            v4168 = v4164;
                            v4169 = v4165;
                            v4170 = v4166;
                            v4171 = v4167;
                            break;
                        } while(false);
                        v4172 = v4168;
                        v4173 = v4169;
                        v4174 = v4170;
                        v4175 = v4171;
                    }
                    else
                    {
                        bool v423;
                        if (!(v389 == 3))
                        {
                            v423 = v389 == 12;
                        }
                        else
                        {
                            v423 = true;
                        }
                        uint v3757;
                        uint v3758;
                        bool v3759;
                        bool v3760;
// case 3 / 12(两相邻角点 0,1 及其补):等离线连接边 0-3 与边 1-2
                        if (v423)
                        {
                            uint v3753;
                            uint v3754;
                            bool v3755;
                            bool v3756;
                            do
                            {
                                float v3347 = -v362;
                                float v3365;
                                float v3368;
                                float2 v3370;
                                float2 v3372;
                                v3365 = v3347 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v149[0] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                v3368 = v3347 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v149[3] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                v3370 = v149[0];
                                v3372 = v149[3];
                                for (uint v3374 = 0u; v3374 < 2u; )
                                {
                                    float2 v3378 = v3370 + v3372;
                                    float2 v3379 = v3378 * 0.5f;
                                    float v3386 = v3347 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v3379 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                    float v3387 = v3386 * v3365;
                                    bool v3388 = v3387 < 0.0f;
                                    bool2 v3389 = v3388.xx;
                                    v3365 = v3388 ? v3365 : v3386;
                                    v3368 = v3388 ? v3386 : v3368;
                                    v3370 = float2(v3389.x ? v3370.x : v3379.x, v3389.y ? v3370.y : v3379.y);
                                    v3372 = float2(v3389.x ? v3379.x : v3372.x, v3389.y ? v3379.y : v3372.y);
                                    v3374++;
                                    continue;
                                }
                                float2 v3394 = lerp(v3372, v3370, clamp(v3368 / (v3368 - v3365), 0.0f, 1.0f).xx);
                                float v3395 = v3394.x;
                                float3 v3397 = float3(v3395, v362, v3394.y);
                                float v3415;
                                float v3418;
                                float2 v3420;
                                float2 v3422;
                                v3415 = v3347 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v149[1] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                v3418 = v3347 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v149[2] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                v3420 = v149[1];
                                v3422 = v149[2];
                                for (uint v3424 = 0u; v3424 < 2u; )
                                {
                                    float2 v3428 = v3420 + v3422;
                                    float2 v3429 = v3428 * 0.5f;
                                    float v3436 = v3347 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v3429 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                    float v3437 = v3436 * v3415;
                                    bool v3438 = v3437 < 0.0f;
                                    bool2 v3439 = v3438.xx;
                                    v3415 = v3438 ? v3415 : v3436;
                                    v3418 = v3438 ? v3436 : v3418;
                                    v3420 = float2(v3439.x ? v3420.x : v3429.x, v3439.y ? v3420.y : v3429.y);
                                    v3422 = float2(v3439.x ? v3429.x : v3422.x, v3439.y ? v3429.y : v3422.y);
                                    v3424++;
                                    continue;
                                }
                                float2 v3453;
                                float v3454;
                                bool v3456;
                                float2 v3444 = lerp(v3422, v3420, clamp(v3418 / (v3418 - v3415), 0.0f, 1.0f).xx);
                                float v3446 = v3444.y;
                                float3 v3447 = float3(v3444.x, v362, v3446);
                                float3 v3448 = lerp(v3397, v3447, 0.5f.xxx);
                                float v3480;
                                do
                                {
                                    v3453 = (v3448.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                    v3454 = v3453.x;
                                    v3456 = !(v3454 < 0.0f);
                                    bool v3460;
                                    if (v3456)
                                    {
                                        v3460 = v3454 > 1.0f;
                                    }
                                    else
                                    {
                                        v3460 = true;
                                    }
                                    bool v3466;
                                    if (!v3460)
                                    {
                                        v3466 = v3453.y < 0.0f;
                                    }
                                    else
                                    {
                                        v3466 = true;
                                    }
                                    bool v3472;
                                    if (!v3466)
                                    {
                                        v3472 = v3453.y > 1.0f;
                                    }
                                    else
                                    {
                                        v3472 = true;
                                    }
                                    if (v3472)
                                    {
                                        v3480 = 0.0f;
                                        break;
                                    }
                                    else
                                    {
                                        v3480 = rocky_terrain.SampleLevel(sampler_default, v3453, 0.0f).x;
                                        break;
                                    }
                                    break; // unreachable workaround
                                } while(false);
                                if (v3480 < 0.00999999977648258209228515625f)
                                {
                                    v3753 = v346;
                                    v3754 = v349;
                                    v3755 = v351;
                                    v3756 = v353;
                                    break;
                                }
                                bool v3504;
                                if (smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (v3397.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f)
                                {
                                    v3504 = smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (v3447.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f;
                                }
                                else
                                {
                                    v3504 = false;
                                }
                                if (v3504)
                                {
                                    v3753 = v346;
                                    v3754 = v349;
                                    v3755 = v351;
                                    v3756 = v353;
                                    break;
                                }
                                float v3507 = distance(v3397, v3447);
                                if (v3507 < 0.20000000298023223876953125f)
                                {
                                    v3753 = v346;
                                    v3754 = v349;
                                    v3755 = v351;
                                    v3756 = v353;
                                    break;
                                }
                                float4 v3512 = terrain_heightmap.GatherRed(sampler_default, v3453, int2(0, 0));
                                float v3517 = v3512.z;
                                float v3518 = v3512.y;
                                float v3520 = v3512.w;
                                float v3521 = v3512.x;
                                float2 v3530 = ((float2((v3517 + v3518) - (v3520 + v3521), (v3521 + v3518) - (v3520 + v3517)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(terrain_heightmap_dims));
                                float v3563;
                                do
                                {
                                    bool v3543;
                                    if (v3456)
                                    {
                                        v3543 = v3454 > 1.0f;
                                    }
                                    else
                                    {
                                        v3543 = true;
                                    }
                                    bool v3549;
                                    if (!v3543)
                                    {
                                        v3549 = v3453.y < 0.0f;
                                    }
                                    else
                                    {
                                        v3549 = true;
                                    }
                                    bool v3555;
                                    if (!v3549)
                                    {
                                        v3555 = v3453.y > 1.0f;
                                    }
                                    else
                                    {
                                        v3555 = true;
                                    }
                                    if (v3555)
                                    {
                                        v3563 = 0.0f;
                                        break;
                                    }
                                    else
                                    {
                                        v3563 = rocky_terrain.SampleLevel(sampler_default, v3453, 0.0f).x;
                                        break;
                                    }
                                    break; // unreachable workaround
                                } while(false);
                                float v3589;
                                do
                                {
                                    bool v3569;
                                    if (v3456)
                                    {
                                        v3569 = v3454 > 1.0f;
                                    }
                                    else
                                    {
                                        v3569 = true;
                                    }
                                    bool v3575;
                                    if (!v3569)
                                    {
                                        v3575 = v3453.y < 0.0f;
                                    }
                                    else
                                    {
                                        v3575 = true;
                                    }
                                    bool v3581;
                                    if (!v3575)
                                    {
                                        v3581 = v3453.y > 1.0f;
                                    }
                                    else
                                    {
                                        v3581 = true;
                                    }
                                    if (v3581)
                                    {
                                        v3589 = 0.0f;
                                        break;
                                    }
                                    else
                                    {
                                        v3589 = rocky_terrain.SampleLevel(sampler_default, v3453, 0.0f).z;
                                        break;
                                    }
                                    break; // unreachable workaround
                                } while(false);
                                float3 v3593 = cross(normalize(float3(0.0f, v3530.y, 1.0f)), normalize(float3(1.0f, v3530.x, 0.0f))) * lerp(0.0f, 0.300000011920928955078125f, clamp(v3563 + v3589, 0.0f, 1.0f));
                                float3 v3594 = v3448 - v3593;
                                float3 v3598 = normalize(v3447 - v3397);
                                uint v3603 = uint(((v3395 + v362) + v3446) * 100.0f);
                                float4x4 v3621 = transpose(float4x4(float4(v3598, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(cross(v3598, float3(0.0f, 1.0f, 0.0f)), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
                                float4x4 v3623 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v3594.x), float4(0.0f, 1.0f, 0.0f, v3594.y - 0.5f), float4(0.0f, 0.0f, 1.0f, v3594.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(v3621, float4x4(float4(v3507, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.800000011920928955078125f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                uint v3628;
                                __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, v3628);
                                uint v3642 = uint(int(v3628));
                                float v3644[12] = { v3623[0].x, v3623[1].x, v3623[2].x, v3623[0].y, v3623[1].y, v3623[2].y, v3623[0].z, v3623[1].z, v3623[2].z, v3623[0].w, v3623[1].w, v3623[2].w };
                                Affine3Packed v3645 = { v3644 };
                                Stair v3646 = { v3645 };
                                [unroll]
                                for (int _24ident = 0; _24ident < 12; _24ident++)
                                {
                                    __alias_stairs_buffer.Store<float>(_24ident * 4 + v3642 * 48 + 0, v3646.transform.data[_24ident]);
                                }
                                uint v3647 = v3603 ^ 23784u;
                                uint v3649 = v3647 + (v3647 << 10u);
                                uint v3651 = v3649 ^ (v3649 >> 6u);
                                uint v3653 = v3651 + (v3651 << 3u);
                                uint v3655 = v3653 ^ (v3653 >> 11u);
                                bool v3658 = v353 ? v353 : true;
                                uint v3659 = v353 ? v349 : 8388607u;
                                bool v3660 = v351 ? v351 : true;
                                uint v3661 = v351 ? v346 : 1065353216u;
                                uint v3749;
                                uint v3750;
                                bool v3751;
                                bool v3752;
                                if ((asfloat(((v3655 + (v3655 << 15u)) & v3659) | v3661) - 1.0f) > 0.85000002384185791015625f)
                                {
                                    uint v3669 = v3603 ^ 90455u;
                                    uint v3671 = v3669 + (v3669 << 10u);
                                    uint v3673 = v3671 ^ (v3671 >> 6u);
                                    uint v3675 = v3673 + (v3673 << 3u);
                                    uint v3677 = v3675 ^ (v3675 >> 11u);
                                    bool v3680 = v3658 ? v3658 : true;
                                    uint v3681 = v3658 ? v3659 : 8388607u;
                                    bool v3682 = v3660 ? v3660 : true;
                                    uint v3683 = v3660 ? v3661 : 1065353216u;
                                    float v3688 = lerp(0.20000000298023223876953125f, 0.4000000059604644775390625f, asfloat(((v3677 + (v3677 << 15u)) & v3681) | v3683) - 1.0f);
                                    uint v3689 = v3603 ^ 3845u;
                                    uint v3691 = v3689 + (v3689 << 10u);
                                    uint v3693 = v3691 ^ (v3691 >> 6u);
                                    uint v3695 = v3693 + (v3693 << 3u);
                                    uint v3697 = v3695 ^ (v3695 >> 11u);
                                    uint v3701 = v3680 ? v3681 : 8388607u;
                                    uint v3703 = v3682 ? v3683 : 1065353216u;
                                    float3 v3710 = lerp(v3397, v3447, (asfloat(((v3697 + (v3697 << 15u)) & v3701) | v3703) - 1.0f).xxx) - v3593;
                                    uint v3714;
                                    __alias_draw_commands.InterlockedAdd(pebble_indirect_draw_idx * 20 + 4, 1u, v3714);
                                    uint v3719 = pebble_buffer_offset + uint(int(v3714));
                                    float4x4 v3732 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v3710.x), float4(0.0f, 1.0f, 0.0f, v3710.y), float4(0.0f, 0.0f, 1.0f, v3710.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(v3621, float4x4(float4(v3688, 0.0f, 0.0f, 0.0f), float4(0.0f, v3688, 0.0f, 0.0f), float4(0.0f, 0.0f, v3688, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                    float v3746[12] = { v3732[0].x, v3732[1].x, v3732[2].x, v3732[0].y, v3732[1].y, v3732[2].y, v3732[0].z, v3732[1].z, v3732[2].z, v3732[0].w, v3732[1].w, v3732[2].w };
                                    Affine3Packed v3747 = { v3746 };
                                    Stair v3748 = { v3747 };
                                    [unroll]
                                    for (int _25ident = 0; _25ident < 12; _25ident++)
                                    {
                                        __alias_stairs_buffer.Store<float>(_25ident * 4 + v3719 * 48 + 0, v3748.transform.data[_25ident]);
                                    }
                                    v3749 = v3703;
                                    v3750 = v3701;
                                    v3751 = v3682 ? v3682 : true;
                                    v3752 = v3680 ? v3680 : true;
                                }
                                else
                                {
                                    v3749 = v3661;
                                    v3750 = v3659;
                                    v3751 = v3660;
                                    v3752 = v3658;
                                }
                                v3753 = v3749;
                                v3754 = v3750;
                                v3755 = v3751;
                                v3756 = v3752;
                                break;
                            } while(false);
                            v3757 = v3753;
                            v3758 = v3754;
                            v3759 = v3755;
                            v3760 = v3756;
                        }
                        else
                        {
                            bool v432;
                            if (!(v389 == 4))
                            {
                                v432 = v389 == 11;
                            }
                            else
                            {
                                v432 = true;
                            }
                            uint v3339;
                            uint v3340;
                            bool v3341;
                            bool v3342;
// case 4 / 11(单角点 3 及其补):等离线连接边 0-1 与边 1-2
                            if (v432)
                            {
                                uint v3335;
                                uint v3336;
                                bool v3337;
                                bool v3338;
                                do
                                {
                                    float v2932 = -v362;
                                    float2 v2943 = (v149[1] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                    float v2950;
                                    float v2953;
                                    float2 v2955;
                                    float2 v2957;
                                    v2950 = v2932 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v149[0] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                    v2953 = v2932 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, v2943, 0.0f).x);
                                    v2955 = v149[0];
                                    v2957 = v149[1];
                                    for (uint v2959 = 0u; v2959 < 2u; )
                                    {
                                        float2 v2963 = v2955 + v2957;
                                        float2 v2964 = v2963 * 0.5f;
                                        float v2971 = v2932 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v2964 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                        float v2972 = v2971 * v2950;
                                        bool v2973 = v2972 < 0.0f;
                                        bool2 v2974 = v2973.xx;
                                        v2950 = v2973 ? v2950 : v2971;
                                        v2953 = v2973 ? v2971 : v2953;
                                        v2955 = float2(v2974.x ? v2955.x : v2964.x, v2974.y ? v2955.y : v2964.y);
                                        v2957 = float2(v2974.x ? v2964.x : v2957.x, v2974.y ? v2964.y : v2957.y);
                                        v2959++;
                                        continue;
                                    }
                                    float2 v2979 = lerp(v2957, v2955, clamp(v2953 / (v2953 - v2950), 0.0f, 1.0f).xx);
                                    float v2980 = v2979.x;
                                    float3 v2982 = float3(v2980, v362, v2979.y);
                                    float v2997;
                                    float v3000;
                                    float2 v3002;
                                    float2 v3004;
                                    v2997 = v2932 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, v2943, 0.0f).x);
                                    v3000 = v2932 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v149[2] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                    v3002 = v149[1];
                                    v3004 = v149[2];
                                    for (uint v3006 = 0u; v3006 < 2u; )
                                    {
                                        float2 v3010 = v3002 + v3004;
                                        float2 v3011 = v3010 * 0.5f;
                                        float v3018 = v2932 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v3011 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                        float v3019 = v3018 * v2997;
                                        bool v3020 = v3019 < 0.0f;
                                        bool2 v3021 = v3020.xx;
                                        v2997 = v3020 ? v2997 : v3018;
                                        v3000 = v3020 ? v3018 : v3000;
                                        v3002 = float2(v3021.x ? v3002.x : v3011.x, v3021.y ? v3002.y : v3011.y);
                                        v3004 = float2(v3021.x ? v3011.x : v3004.x, v3021.y ? v3011.y : v3004.y);
                                        v3006++;
                                        continue;
                                    }
                                    float2 v3035;
                                    float v3036;
                                    bool v3038;
                                    float2 v3026 = lerp(v3004, v3002, clamp(v3000 / (v3000 - v2997), 0.0f, 1.0f).xx);
                                    float v3028 = v3026.y;
                                    float3 v3029 = float3(v3026.x, v362, v3028);
                                    float3 v3030 = lerp(v2982, v3029, 0.5f.xxx);
                                    float v3062;
                                    do
                                    {
                                        v3035 = (v3030.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                        v3036 = v3035.x;
                                        v3038 = !(v3036 < 0.0f);
                                        bool v3042;
                                        if (v3038)
                                        {
                                            v3042 = v3036 > 1.0f;
                                        }
                                        else
                                        {
                                            v3042 = true;
                                        }
                                        bool v3048;
                                        if (!v3042)
                                        {
                                            v3048 = v3035.y < 0.0f;
                                        }
                                        else
                                        {
                                            v3048 = true;
                                        }
                                        bool v3054;
                                        if (!v3048)
                                        {
                                            v3054 = v3035.y > 1.0f;
                                        }
                                        else
                                        {
                                            v3054 = true;
                                        }
                                        if (v3054)
                                        {
                                            v3062 = 0.0f;
                                            break;
                                        }
                                        else
                                        {
                                            v3062 = rocky_terrain.SampleLevel(sampler_default, v3035, 0.0f).x;
                                            break;
                                        }
                                        break; // unreachable workaround
                                    } while(false);
                                    if (v3062 < 0.00999999977648258209228515625f)
                                    {
                                        v3335 = v346;
                                        v3336 = v349;
                                        v3337 = v351;
                                        v3338 = v353;
                                        break;
                                    }
                                    bool v3086;
                                    if (smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (v2982.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f)
                                    {
                                        v3086 = smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (v3029.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f;
                                    }
                                    else
                                    {
                                        v3086 = false;
                                    }
                                    if (v3086)
                                    {
                                        v3335 = v346;
                                        v3336 = v349;
                                        v3337 = v351;
                                        v3338 = v353;
                                        break;
                                    }
                                    float v3089 = distance(v2982, v3029);
                                    if (v3089 < 0.20000000298023223876953125f)
                                    {
                                        v3335 = v346;
                                        v3336 = v349;
                                        v3337 = v351;
                                        v3338 = v353;
                                        break;
                                    }
                                    float4 v3094 = terrain_heightmap.GatherRed(sampler_default, v3035, int2(0, 0));
                                    float v3099 = v3094.z;
                                    float v3100 = v3094.y;
                                    float v3102 = v3094.w;
                                    float v3103 = v3094.x;
                                    float2 v3112 = ((float2((v3099 + v3100) - (v3102 + v3103), (v3103 + v3100) - (v3102 + v3099)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(terrain_heightmap_dims));
                                    float v3145;
                                    do
                                    {
                                        bool v3125;
                                        if (v3038)
                                        {
                                            v3125 = v3036 > 1.0f;
                                        }
                                        else
                                        {
                                            v3125 = true;
                                        }
                                        bool v3131;
                                        if (!v3125)
                                        {
                                            v3131 = v3035.y < 0.0f;
                                        }
                                        else
                                        {
                                            v3131 = true;
                                        }
                                        bool v3137;
                                        if (!v3131)
                                        {
                                            v3137 = v3035.y > 1.0f;
                                        }
                                        else
                                        {
                                            v3137 = true;
                                        }
                                        if (v3137)
                                        {
                                            v3145 = 0.0f;
                                            break;
                                        }
                                        else
                                        {
                                            v3145 = rocky_terrain.SampleLevel(sampler_default, v3035, 0.0f).x;
                                            break;
                                        }
                                        break; // unreachable workaround
                                    } while(false);
                                    float v3171;
                                    do
                                    {
                                        bool v3151;
                                        if (v3038)
                                        {
                                            v3151 = v3036 > 1.0f;
                                        }
                                        else
                                        {
                                            v3151 = true;
                                        }
                                        bool v3157;
                                        if (!v3151)
                                        {
                                            v3157 = v3035.y < 0.0f;
                                        }
                                        else
                                        {
                                            v3157 = true;
                                        }
                                        bool v3163;
                                        if (!v3157)
                                        {
                                            v3163 = v3035.y > 1.0f;
                                        }
                                        else
                                        {
                                            v3163 = true;
                                        }
                                        if (v3163)
                                        {
                                            v3171 = 0.0f;
                                            break;
                                        }
                                        else
                                        {
                                            v3171 = rocky_terrain.SampleLevel(sampler_default, v3035, 0.0f).z;
                                            break;
                                        }
                                        break; // unreachable workaround
                                    } while(false);
                                    float3 v3175 = cross(normalize(float3(0.0f, v3112.y, 1.0f)), normalize(float3(1.0f, v3112.x, 0.0f))) * lerp(0.0f, 0.300000011920928955078125f, clamp(v3145 + v3171, 0.0f, 1.0f));
                                    float3 v3176 = v3030 - v3175;
                                    float3 v3180 = normalize(v3029 - v2982);
                                    uint v3185 = uint(((v2980 + v362) + v3028) * 100.0f);
                                    float4x4 v3203 = transpose(float4x4(float4(v3180, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(cross(v3180, float3(0.0f, 1.0f, 0.0f)), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
                                    float4x4 v3205 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v3176.x), float4(0.0f, 1.0f, 0.0f, v3176.y - 0.5f), float4(0.0f, 0.0f, 1.0f, v3176.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(v3203, float4x4(float4(v3089, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.800000011920928955078125f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                    uint v3210;
                                    __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, v3210);
                                    uint v3224 = uint(int(v3210));
                                    float v3226[12] = { v3205[0].x, v3205[1].x, v3205[2].x, v3205[0].y, v3205[1].y, v3205[2].y, v3205[0].z, v3205[1].z, v3205[2].z, v3205[0].w, v3205[1].w, v3205[2].w };
                                    Affine3Packed v3227 = { v3226 };
                                    Stair v3228 = { v3227 };
                                    [unroll]
                                    for (int _26ident = 0; _26ident < 12; _26ident++)
                                    {
                                        __alias_stairs_buffer.Store<float>(_26ident * 4 + v3224 * 48 + 0, v3228.transform.data[_26ident]);
                                    }
                                    uint v3229 = v3185 ^ 23784u;
                                    uint v3231 = v3229 + (v3229 << 10u);
                                    uint v3233 = v3231 ^ (v3231 >> 6u);
                                    uint v3235 = v3233 + (v3233 << 3u);
                                    uint v3237 = v3235 ^ (v3235 >> 11u);
                                    bool v3240 = v353 ? v353 : true;
                                    uint v3241 = v353 ? v349 : 8388607u;
                                    bool v3242 = v351 ? v351 : true;
                                    uint v3243 = v351 ? v346 : 1065353216u;
                                    uint v3331;
                                    uint v3332;
                                    bool v3333;
                                    bool v3334;
                                    if ((asfloat(((v3237 + (v3237 << 15u)) & v3241) | v3243) - 1.0f) > 0.85000002384185791015625f)
                                    {
                                        uint v3251 = v3185 ^ 90455u;
                                        uint v3253 = v3251 + (v3251 << 10u);
                                        uint v3255 = v3253 ^ (v3253 >> 6u);
                                        uint v3257 = v3255 + (v3255 << 3u);
                                        uint v3259 = v3257 ^ (v3257 >> 11u);
                                        bool v3262 = v3240 ? v3240 : true;
                                        uint v3263 = v3240 ? v3241 : 8388607u;
                                        bool v3264 = v3242 ? v3242 : true;
                                        uint v3265 = v3242 ? v3243 : 1065353216u;
                                        float v3270 = lerp(0.20000000298023223876953125f, 0.4000000059604644775390625f, asfloat(((v3259 + (v3259 << 15u)) & v3263) | v3265) - 1.0f);
                                        uint v3271 = v3185 ^ 3845u;
                                        uint v3273 = v3271 + (v3271 << 10u);
                                        uint v3275 = v3273 ^ (v3273 >> 6u);
                                        uint v3277 = v3275 + (v3275 << 3u);
                                        uint v3279 = v3277 ^ (v3277 >> 11u);
                                        uint v3283 = v3262 ? v3263 : 8388607u;
                                        uint v3285 = v3264 ? v3265 : 1065353216u;
                                        float3 v3292 = lerp(v2982, v3029, (asfloat(((v3279 + (v3279 << 15u)) & v3283) | v3285) - 1.0f).xxx) - v3175;
                                        uint v3296;
                                        __alias_draw_commands.InterlockedAdd(pebble_indirect_draw_idx * 20 + 4, 1u, v3296);
                                        uint v3301 = pebble_buffer_offset + uint(int(v3296));
                                        float4x4 v3314 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v3292.x), float4(0.0f, 1.0f, 0.0f, v3292.y), float4(0.0f, 0.0f, 1.0f, v3292.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(v3203, float4x4(float4(v3270, 0.0f, 0.0f, 0.0f), float4(0.0f, v3270, 0.0f, 0.0f), float4(0.0f, 0.0f, v3270, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                        float v3328[12] = { v3314[0].x, v3314[1].x, v3314[2].x, v3314[0].y, v3314[1].y, v3314[2].y, v3314[0].z, v3314[1].z, v3314[2].z, v3314[0].w, v3314[1].w, v3314[2].w };
                                        Affine3Packed v3329 = { v3328 };
                                        Stair v3330 = { v3329 };
                                        [unroll]
                                        for (int _27ident = 0; _27ident < 12; _27ident++)
                                        {
                                            __alias_stairs_buffer.Store<float>(_27ident * 4 + v3301 * 48 + 0, v3330.transform.data[_27ident]);
                                        }
                                        v3331 = v3285;
                                        v3332 = v3283;
                                        v3333 = v3264 ? v3264 : true;
                                        v3334 = v3262 ? v3262 : true;
                                    }
                                    else
                                    {
                                        v3331 = v3243;
                                        v3332 = v3241;
                                        v3333 = v3242;
                                        v3334 = v3240;
                                    }
                                    v3335 = v3331;
                                    v3336 = v3332;
                                    v3337 = v3333;
                                    v3338 = v3334;
                                    break;
                                } while(false);
                                v3339 = v3335;
                                v3340 = v3336;
                                v3341 = v3337;
                                v3342 = v3338;
                            }
                            else
                            {
                                uint v2924;
                                uint v2925;
                                bool v2926;
                                bool v2927;
// case 5(鞍形):两条不相连的等离线线段,分别放置台阶(两段各一次完整逻辑)
                                if (v389 == 5)
                                {
                                    float2 v2114;
                                    float v2115;
                                    float2 v2126;
                                    float2 v2166;
                                    float2 v2173;
                                    uint v2518;
                                    uint v2519;
                                    bool v2520;
                                    bool v2521;
                                    do
                                    {
                                        v2114 = v149[1];
                                        v2115 = -v362;
                                        float2 v2117 = (v149[0] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                        v2126 = (v2114 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                        float v2133;
                                        float v2136;
                                        float2 v2138;
                                        float2 v2140;
                                        v2133 = v2115 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, v2117, 0.0f).x);
                                        v2136 = v2115 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, v2126, 0.0f).x);
                                        v2138 = v149[0];
                                        v2140 = v2114;
                                        for (uint v2142 = 0u; v2142 < 2u; )
                                        {
                                            float2 v2146 = v2138 + v2140;
                                            float2 v2147 = v2146 * 0.5f;
                                            float v2154 = v2115 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v2147 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                            float v2155 = v2154 * v2133;
                                            bool v2156 = v2155 < 0.0f;
                                            bool2 v2157 = v2156.xx;
                                            v2133 = v2156 ? v2133 : v2154;
                                            v2136 = v2156 ? v2154 : v2136;
                                            v2138 = float2(v2157.x ? v2138.x : v2147.x, v2157.y ? v2138.y : v2147.y);
                                            v2140 = float2(v2157.x ? v2147.x : v2140.x, v2157.y ? v2147.y : v2140.y);
                                            v2142++;
                                            continue;
                                        }
                                        float2 v2162 = lerp(v2140, v2138, clamp(v2136 / (v2136 - v2133), 0.0f, 1.0f).xx);
                                        float v2163 = v2162.x;
                                        float3 v2165 = float3(v2163, v362, v2162.y);
                                        v2166 = v149[3];
                                        v2173 = (v2166 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                        float v2180;
                                        float v2183;
                                        float2 v2185;
                                        float2 v2187;
                                        v2180 = v2115 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, v2117, 0.0f).x);
                                        v2183 = v2115 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, v2173, 0.0f).x);
                                        v2185 = v149[0];
                                        v2187 = v2166;
                                        for (uint v2189 = 0u; v2189 < 2u; )
                                        {
                                            float2 v2193 = v2185 + v2187;
                                            float2 v2194 = v2193 * 0.5f;
                                            float v2201 = v2115 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v2194 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                            float v2202 = v2201 * v2180;
                                            bool v2203 = v2202 < 0.0f;
                                            bool2 v2204 = v2203.xx;
                                            v2180 = v2203 ? v2180 : v2201;
                                            v2183 = v2203 ? v2201 : v2183;
                                            v2185 = float2(v2204.x ? v2185.x : v2194.x, v2204.y ? v2185.y : v2194.y);
                                            v2187 = float2(v2204.x ? v2194.x : v2187.x, v2204.y ? v2194.y : v2187.y);
                                            v2189++;
                                            continue;
                                        }
                                        float2 v2218;
                                        float v2219;
                                        bool v2221;
                                        float2 v2209 = lerp(v2187, v2185, clamp(v2183 / (v2183 - v2180), 0.0f, 1.0f).xx);
                                        float v2211 = v2209.y;
                                        float3 v2212 = float3(v2209.x, v362, v2211);
                                        float3 v2213 = lerp(v2165, v2212, 0.5f.xxx);
                                        float v2245;
                                        do
                                        {
                                            v2218 = (v2213.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                            v2219 = v2218.x;
                                            v2221 = !(v2219 < 0.0f);
                                            bool v2225;
                                            if (v2221)
                                            {
                                                v2225 = v2219 > 1.0f;
                                            }
                                            else
                                            {
                                                v2225 = true;
                                            }
                                            bool v2231;
                                            if (!v2225)
                                            {
                                                v2231 = v2218.y < 0.0f;
                                            }
                                            else
                                            {
                                                v2231 = true;
                                            }
                                            bool v2237;
                                            if (!v2231)
                                            {
                                                v2237 = v2218.y > 1.0f;
                                            }
                                            else
                                            {
                                                v2237 = true;
                                            }
                                            if (v2237)
                                            {
                                                v2245 = 0.0f;
                                                break;
                                            }
                                            else
                                            {
                                                v2245 = rocky_terrain.SampleLevel(sampler_default, v2218, 0.0f).x;
                                                break;
                                            }
                                            break; // unreachable workaround
                                        } while(false);
                                        if (v2245 < 0.00999999977648258209228515625f)
                                        {
                                            v2518 = v346;
                                            v2519 = v349;
                                            v2520 = v351;
                                            v2521 = v353;
                                            break;
                                        }
                                        bool v2269;
                                        if (smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (v2165.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f)
                                        {
                                            v2269 = smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (v2212.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f;
                                        }
                                        else
                                        {
                                            v2269 = false;
                                        }
                                        if (v2269)
                                        {
                                            v2518 = v346;
                                            v2519 = v349;
                                            v2520 = v351;
                                            v2521 = v353;
                                            break;
                                        }
                                        float v2272 = distance(v2165, v2212);
                                        if (v2272 < 0.20000000298023223876953125f)
                                        {
                                            v2518 = v346;
                                            v2519 = v349;
                                            v2520 = v351;
                                            v2521 = v353;
                                            break;
                                        }
                                        float4 v2277 = terrain_heightmap.GatherRed(sampler_default, v2218, int2(0, 0));
                                        float v2282 = v2277.z;
                                        float v2283 = v2277.y;
                                        float v2285 = v2277.w;
                                        float v2286 = v2277.x;
                                        float2 v2295 = ((float2((v2282 + v2283) - (v2285 + v2286), (v2286 + v2283) - (v2285 + v2282)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(terrain_heightmap_dims));
                                        float v2328;
                                        do
                                        {
                                            bool v2308;
                                            if (v2221)
                                            {
                                                v2308 = v2219 > 1.0f;
                                            }
                                            else
                                            {
                                                v2308 = true;
                                            }
                                            bool v2314;
                                            if (!v2308)
                                            {
                                                v2314 = v2218.y < 0.0f;
                                            }
                                            else
                                            {
                                                v2314 = true;
                                            }
                                            bool v2320;
                                            if (!v2314)
                                            {
                                                v2320 = v2218.y > 1.0f;
                                            }
                                            else
                                            {
                                                v2320 = true;
                                            }
                                            if (v2320)
                                            {
                                                v2328 = 0.0f;
                                                break;
                                            }
                                            else
                                            {
                                                v2328 = rocky_terrain.SampleLevel(sampler_default, v2218, 0.0f).x;
                                                break;
                                            }
                                            break; // unreachable workaround
                                        } while(false);
                                        float v2354;
                                        do
                                        {
                                            bool v2334;
                                            if (v2221)
                                            {
                                                v2334 = v2219 > 1.0f;
                                            }
                                            else
                                            {
                                                v2334 = true;
                                            }
                                            bool v2340;
                                            if (!v2334)
                                            {
                                                v2340 = v2218.y < 0.0f;
                                            }
                                            else
                                            {
                                                v2340 = true;
                                            }
                                            bool v2346;
                                            if (!v2340)
                                            {
                                                v2346 = v2218.y > 1.0f;
                                            }
                                            else
                                            {
                                                v2346 = true;
                                            }
                                            if (v2346)
                                            {
                                                v2354 = 0.0f;
                                                break;
                                            }
                                            else
                                            {
                                                v2354 = rocky_terrain.SampleLevel(sampler_default, v2218, 0.0f).z;
                                                break;
                                            }
                                            break; // unreachable workaround
                                        } while(false);
                                        float3 v2358 = cross(normalize(float3(0.0f, v2295.y, 1.0f)), normalize(float3(1.0f, v2295.x, 0.0f))) * lerp(0.0f, 0.300000011920928955078125f, clamp(v2328 + v2354, 0.0f, 1.0f));
                                        float3 v2359 = v2213 - v2358;
                                        float3 v2363 = normalize(v2212 - v2165);
                                        uint v2368 = uint(((v2163 + v362) + v2211) * 100.0f);
                                        float4x4 v2386 = transpose(float4x4(float4(v2363, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(cross(v2363, float3(0.0f, 1.0f, 0.0f)), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
                                        float4x4 v2388 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v2359.x), float4(0.0f, 1.0f, 0.0f, v2359.y - 0.5f), float4(0.0f, 0.0f, 1.0f, v2359.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(v2386, float4x4(float4(v2272, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.800000011920928955078125f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                        uint v2393;
                                        __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, v2393);
                                        uint v2407 = uint(int(v2393));
                                        float v2409[12] = { v2388[0].x, v2388[1].x, v2388[2].x, v2388[0].y, v2388[1].y, v2388[2].y, v2388[0].z, v2388[1].z, v2388[2].z, v2388[0].w, v2388[1].w, v2388[2].w };
                                        Affine3Packed v2410 = { v2409 };
                                        Stair v2411 = { v2410 };
                                        [unroll]
                                        for (int _28ident = 0; _28ident < 12; _28ident++)
                                        {
                                            __alias_stairs_buffer.Store<float>(_28ident * 4 + v2407 * 48 + 0, v2411.transform.data[_28ident]);
                                        }
                                        uint v2412 = v2368 ^ 23784u;
                                        uint v2414 = v2412 + (v2412 << 10u);
                                        uint v2416 = v2414 ^ (v2414 >> 6u);
                                        uint v2418 = v2416 + (v2416 << 3u);
                                        uint v2420 = v2418 ^ (v2418 >> 11u);
                                        bool v2423 = v353 ? v353 : true;
                                        uint v2424 = v353 ? v349 : 8388607u;
                                        bool v2425 = v351 ? v351 : true;
                                        uint v2426 = v351 ? v346 : 1065353216u;
                                        uint v2514;
                                        uint v2515;
                                        bool v2516;
                                        bool v2517;
                                        if ((asfloat(((v2420 + (v2420 << 15u)) & v2424) | v2426) - 1.0f) > 0.85000002384185791015625f)
                                        {
                                            uint v2434 = v2368 ^ 90455u;
                                            uint v2436 = v2434 + (v2434 << 10u);
                                            uint v2438 = v2436 ^ (v2436 >> 6u);
                                            uint v2440 = v2438 + (v2438 << 3u);
                                            uint v2442 = v2440 ^ (v2440 >> 11u);
                                            bool v2445 = v2423 ? v2423 : true;
                                            uint v2446 = v2423 ? v2424 : 8388607u;
                                            bool v2447 = v2425 ? v2425 : true;
                                            uint v2448 = v2425 ? v2426 : 1065353216u;
                                            float v2453 = lerp(0.20000000298023223876953125f, 0.4000000059604644775390625f, asfloat(((v2442 + (v2442 << 15u)) & v2446) | v2448) - 1.0f);
                                            uint v2454 = v2368 ^ 3845u;
                                            uint v2456 = v2454 + (v2454 << 10u);
                                            uint v2458 = v2456 ^ (v2456 >> 6u);
                                            uint v2460 = v2458 + (v2458 << 3u);
                                            uint v2462 = v2460 ^ (v2460 >> 11u);
                                            uint v2466 = v2445 ? v2446 : 8388607u;
                                            uint v2468 = v2447 ? v2448 : 1065353216u;
                                            float3 v2475 = lerp(v2165, v2212, (asfloat(((v2462 + (v2462 << 15u)) & v2466) | v2468) - 1.0f).xxx) - v2358;
                                            uint v2479;
                                            __alias_draw_commands.InterlockedAdd(pebble_indirect_draw_idx * 20 + 4, 1u, v2479);
                                            uint v2484 = pebble_buffer_offset + uint(int(v2479));
                                            float4x4 v2497 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v2475.x), float4(0.0f, 1.0f, 0.0f, v2475.y), float4(0.0f, 0.0f, 1.0f, v2475.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(v2386, float4x4(float4(v2453, 0.0f, 0.0f, 0.0f), float4(0.0f, v2453, 0.0f, 0.0f), float4(0.0f, 0.0f, v2453, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                            float v2511[12] = { v2497[0].x, v2497[1].x, v2497[2].x, v2497[0].y, v2497[1].y, v2497[2].y, v2497[0].z, v2497[1].z, v2497[2].z, v2497[0].w, v2497[1].w, v2497[2].w };
                                            Affine3Packed v2512 = { v2511 };
                                            Stair v2513 = { v2512 };
                                            [unroll]
                                            for (int _29ident = 0; _29ident < 12; _29ident++)
                                            {
                                                __alias_stairs_buffer.Store<float>(_29ident * 4 + v2484 * 48 + 0, v2513.transform.data[_29ident]);
                                            }
                                            v2514 = v2468;
                                            v2515 = v2466;
                                            v2516 = v2447 ? v2447 : true;
                                            v2517 = v2445 ? v2445 : true;
                                        }
                                        else
                                        {
                                            v2514 = v2426;
                                            v2515 = v2424;
                                            v2516 = v2425;
                                            v2517 = v2423;
                                        }
                                        v2518 = v2514;
                                        v2519 = v2515;
                                        v2520 = v2516;
                                        v2521 = v2517;
                                        break;
                                    } while(false);
                                    uint v2920;
                                    uint v2921;
                                    bool v2922;
                                    bool v2923;
                                    do
                                    {
                                        float2 v2531 = (v149[2] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                        float v2538;
                                        float v2541;
                                        float2 v2543;
                                        float2 v2545;
                                        v2538 = v2115 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, v2126, 0.0f).x);
                                        v2541 = v2115 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, v2531, 0.0f).x);
                                        v2543 = v2114;
                                        v2545 = v149[2];
                                        for (uint v2547 = 0u; v2547 < 2u; )
                                        {
                                            float2 v2551 = v2543 + v2545;
                                            float2 v2552 = v2551 * 0.5f;
                                            float v2559 = v2115 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v2552 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                            float v2560 = v2559 * v2538;
                                            bool v2561 = v2560 < 0.0f;
                                            bool2 v2562 = v2561.xx;
                                            v2538 = v2561 ? v2538 : v2559;
                                            v2541 = v2561 ? v2559 : v2541;
                                            v2543 = float2(v2562.x ? v2543.x : v2552.x, v2562.y ? v2543.y : v2552.y);
                                            v2545 = float2(v2562.x ? v2552.x : v2545.x, v2562.y ? v2552.y : v2545.y);
                                            v2547++;
                                            continue;
                                        }
                                        float2 v2567 = lerp(v2545, v2543, clamp(v2541 / (v2541 - v2538), 0.0f, 1.0f).xx);
                                        float v2568 = v2567.x;
                                        float3 v2570 = float3(v2568, v362, v2567.y);
                                        float v2582;
                                        float v2585;
                                        float2 v2587;
                                        float2 v2589;
                                        v2582 = v2115 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, v2173, 0.0f).x);
                                        v2585 = v2115 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, v2531, 0.0f).x);
                                        v2587 = v2166;
                                        v2589 = v149[2];
                                        for (uint v2591 = 0u; v2591 < 2u; )
                                        {
                                            float2 v2595 = v2587 + v2589;
                                            float2 v2596 = v2595 * 0.5f;
                                            float v2603 = v2115 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v2596 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                            float v2604 = v2603 * v2582;
                                            bool v2605 = v2604 < 0.0f;
                                            bool2 v2606 = v2605.xx;
                                            v2582 = v2605 ? v2582 : v2603;
                                            v2585 = v2605 ? v2603 : v2585;
                                            v2587 = float2(v2606.x ? v2587.x : v2596.x, v2606.y ? v2587.y : v2596.y);
                                            v2589 = float2(v2606.x ? v2596.x : v2589.x, v2606.y ? v2596.y : v2589.y);
                                            v2591++;
                                            continue;
                                        }
                                        float2 v2620;
                                        float v2621;
                                        bool v2623;
                                        float2 v2611 = lerp(v2589, v2587, clamp(v2585 / (v2585 - v2582), 0.0f, 1.0f).xx);
                                        float v2613 = v2611.y;
                                        float3 v2614 = float3(v2611.x, v362, v2613);
                                        float3 v2615 = lerp(v2570, v2614, 0.5f.xxx);
                                        float v2647;
                                        do
                                        {
                                            v2620 = (v2615.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                            v2621 = v2620.x;
                                            v2623 = !(v2621 < 0.0f);
                                            bool v2627;
                                            if (v2623)
                                            {
                                                v2627 = v2621 > 1.0f;
                                            }
                                            else
                                            {
                                                v2627 = true;
                                            }
                                            bool v2633;
                                            if (!v2627)
                                            {
                                                v2633 = v2620.y < 0.0f;
                                            }
                                            else
                                            {
                                                v2633 = true;
                                            }
                                            bool v2639;
                                            if (!v2633)
                                            {
                                                v2639 = v2620.y > 1.0f;
                                            }
                                            else
                                            {
                                                v2639 = true;
                                            }
                                            if (v2639)
                                            {
                                                v2647 = 0.0f;
                                                break;
                                            }
                                            else
                                            {
                                                v2647 = rocky_terrain.SampleLevel(sampler_default, v2620, 0.0f).x;
                                                break;
                                            }
                                            break; // unreachable workaround
                                        } while(false);
                                        if (v2647 < 0.00999999977648258209228515625f)
                                        {
                                            v2920 = v2518;
                                            v2921 = v2519;
                                            v2922 = v2520;
                                            v2923 = v2521;
                                            break;
                                        }
                                        bool v2671;
                                        if (smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (v2570.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f)
                                        {
                                            v2671 = smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (v2614.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f;
                                        }
                                        else
                                        {
                                            v2671 = false;
                                        }
                                        if (v2671)
                                        {
                                            v2920 = v2518;
                                            v2921 = v2519;
                                            v2922 = v2520;
                                            v2923 = v2521;
                                            break;
                                        }
                                        float v2674 = distance(v2570, v2614);
                                        if (v2674 < 0.20000000298023223876953125f)
                                        {
                                            v2920 = v2518;
                                            v2921 = v2519;
                                            v2922 = v2520;
                                            v2923 = v2521;
                                            break;
                                        }
                                        float4 v2679 = terrain_heightmap.GatherRed(sampler_default, v2620, int2(0, 0));
                                        float v2684 = v2679.z;
                                        float v2685 = v2679.y;
                                        float v2687 = v2679.w;
                                        float v2688 = v2679.x;
                                        float2 v2697 = ((float2((v2684 + v2685) - (v2687 + v2688), (v2688 + v2685) - (v2687 + v2684)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(terrain_heightmap_dims));
                                        float v2730;
                                        do
                                        {
                                            bool v2710;
                                            if (v2623)
                                            {
                                                v2710 = v2621 > 1.0f;
                                            }
                                            else
                                            {
                                                v2710 = true;
                                            }
                                            bool v2716;
                                            if (!v2710)
                                            {
                                                v2716 = v2620.y < 0.0f;
                                            }
                                            else
                                            {
                                                v2716 = true;
                                            }
                                            bool v2722;
                                            if (!v2716)
                                            {
                                                v2722 = v2620.y > 1.0f;
                                            }
                                            else
                                            {
                                                v2722 = true;
                                            }
                                            if (v2722)
                                            {
                                                v2730 = 0.0f;
                                                break;
                                            }
                                            else
                                            {
                                                v2730 = rocky_terrain.SampleLevel(sampler_default, v2620, 0.0f).x;
                                                break;
                                            }
                                            break; // unreachable workaround
                                        } while(false);
                                        float v2756;
                                        do
                                        {
                                            bool v2736;
                                            if (v2623)
                                            {
                                                v2736 = v2621 > 1.0f;
                                            }
                                            else
                                            {
                                                v2736 = true;
                                            }
                                            bool v2742;
                                            if (!v2736)
                                            {
                                                v2742 = v2620.y < 0.0f;
                                            }
                                            else
                                            {
                                                v2742 = true;
                                            }
                                            bool v2748;
                                            if (!v2742)
                                            {
                                                v2748 = v2620.y > 1.0f;
                                            }
                                            else
                                            {
                                                v2748 = true;
                                            }
                                            if (v2748)
                                            {
                                                v2756 = 0.0f;
                                                break;
                                            }
                                            else
                                            {
                                                v2756 = rocky_terrain.SampleLevel(sampler_default, v2620, 0.0f).z;
                                                break;
                                            }
                                            break; // unreachable workaround
                                        } while(false);
                                        float3 v2760 = cross(normalize(float3(0.0f, v2697.y, 1.0f)), normalize(float3(1.0f, v2697.x, 0.0f))) * lerp(0.0f, 0.300000011920928955078125f, clamp(v2730 + v2756, 0.0f, 1.0f));
                                        float3 v2761 = v2615 - v2760;
                                        float3 v2765 = normalize(v2614 - v2570);
                                        uint v2770 = uint(((v2568 + v362) + v2613) * 100.0f);
                                        float4x4 v2788 = transpose(float4x4(float4(v2765, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(cross(v2765, float3(0.0f, 1.0f, 0.0f)), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
                                        float4x4 v2790 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v2761.x), float4(0.0f, 1.0f, 0.0f, v2761.y - 0.5f), float4(0.0f, 0.0f, 1.0f, v2761.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(v2788, float4x4(float4(v2674, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.800000011920928955078125f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                        uint v2795;
                                        __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, v2795);
                                        uint v2809 = uint(int(v2795));
                                        float v2811[12] = { v2790[0].x, v2790[1].x, v2790[2].x, v2790[0].y, v2790[1].y, v2790[2].y, v2790[0].z, v2790[1].z, v2790[2].z, v2790[0].w, v2790[1].w, v2790[2].w };
                                        Affine3Packed v2812 = { v2811 };
                                        Stair v2813 = { v2812 };
                                        [unroll]
                                        for (int _30ident = 0; _30ident < 12; _30ident++)
                                        {
                                            __alias_stairs_buffer.Store<float>(_30ident * 4 + v2809 * 48 + 0, v2813.transform.data[_30ident]);
                                        }
                                        uint v2814 = v2770 ^ 23784u;
                                        uint v2816 = v2814 + (v2814 << 10u);
                                        uint v2818 = v2816 ^ (v2816 >> 6u);
                                        uint v2820 = v2818 + (v2818 << 3u);
                                        uint v2822 = v2820 ^ (v2820 >> 11u);
                                        bool v2825 = v2521 ? v2521 : true;
                                        uint v2826 = v2521 ? v2519 : 8388607u;
                                        bool v2827 = v2520 ? v2520 : true;
                                        uint v2828 = v2520 ? v2518 : 1065353216u;
                                        uint v2916;
                                        uint v2917;
                                        bool v2918;
                                        bool v2919;
                                        if ((asfloat(((v2822 + (v2822 << 15u)) & v2826) | v2828) - 1.0f) > 0.85000002384185791015625f)
                                        {
                                            uint v2836 = v2770 ^ 90455u;
                                            uint v2838 = v2836 + (v2836 << 10u);
                                            uint v2840 = v2838 ^ (v2838 >> 6u);
                                            uint v2842 = v2840 + (v2840 << 3u);
                                            uint v2844 = v2842 ^ (v2842 >> 11u);
                                            bool v2847 = v2825 ? v2825 : true;
                                            uint v2848 = v2825 ? v2826 : 8388607u;
                                            bool v2849 = v2827 ? v2827 : true;
                                            uint v2850 = v2827 ? v2828 : 1065353216u;
                                            float v2855 = lerp(0.20000000298023223876953125f, 0.4000000059604644775390625f, asfloat(((v2844 + (v2844 << 15u)) & v2848) | v2850) - 1.0f);
                                            uint v2856 = v2770 ^ 3845u;
                                            uint v2858 = v2856 + (v2856 << 10u);
                                            uint v2860 = v2858 ^ (v2858 >> 6u);
                                            uint v2862 = v2860 + (v2860 << 3u);
                                            uint v2864 = v2862 ^ (v2862 >> 11u);
                                            uint v2868 = v2847 ? v2848 : 8388607u;
                                            uint v2870 = v2849 ? v2850 : 1065353216u;
                                            float3 v2877 = lerp(v2570, v2614, (asfloat(((v2864 + (v2864 << 15u)) & v2868) | v2870) - 1.0f).xxx) - v2760;
                                            uint v2881;
                                            __alias_draw_commands.InterlockedAdd(pebble_indirect_draw_idx * 20 + 4, 1u, v2881);
                                            uint v2886 = pebble_buffer_offset + uint(int(v2881));
                                            float4x4 v2899 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v2877.x), float4(0.0f, 1.0f, 0.0f, v2877.y), float4(0.0f, 0.0f, 1.0f, v2877.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(v2788, float4x4(float4(v2855, 0.0f, 0.0f, 0.0f), float4(0.0f, v2855, 0.0f, 0.0f), float4(0.0f, 0.0f, v2855, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                            float v2913[12] = { v2899[0].x, v2899[1].x, v2899[2].x, v2899[0].y, v2899[1].y, v2899[2].y, v2899[0].z, v2899[1].z, v2899[2].z, v2899[0].w, v2899[1].w, v2899[2].w };
                                            Affine3Packed v2914 = { v2913 };
                                            Stair v2915 = { v2914 };
                                            [unroll]
                                            for (int _31ident = 0; _31ident < 12; _31ident++)
                                            {
                                                __alias_stairs_buffer.Store<float>(_31ident * 4 + v2886 * 48 + 0, v2915.transform.data[_31ident]);
                                            }
                                            v2916 = v2870;
                                            v2917 = v2868;
                                            v2918 = v2849 ? v2849 : true;
                                            v2919 = v2847 ? v2847 : true;
                                        }
                                        else
                                        {
                                            v2916 = v2828;
                                            v2917 = v2826;
                                            v2918 = v2827;
                                            v2919 = v2825;
                                        }
                                        v2920 = v2916;
                                        v2921 = v2917;
                                        v2922 = v2918;
                                        v2923 = v2919;
                                        break;
                                    } while(false);
                                    v2924 = v2920;
                                    v2925 = v2921;
                                    v2926 = v2922;
                                    v2927 = v2923;
                                }
                                else
                                {
                                    bool v445;
// case 6 / 9(两对角角点 0,2 及其补):等离线连接边 0-1 与边 3-2
                                    if (!(v389 == 6))
                                    {
                                        v445 = v389 == 9;
                                    }
                                    else
                                    {
                                        v445 = true;
                                    }
                                    uint v2107;
                                    uint v2108;
                                    bool v2109;
                                    bool v2110;
                                    if (v445)
                                    {
                                        uint v2103;
                                        uint v2104;
                                        bool v2105;
                                        bool v2106;
                                        do
                                        {
                                            float v1697 = -v362;
                                            float v1715;
                                            float v1718;
                                            float2 v1720;
                                            float2 v1722;
                                            v1715 = v1697 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v149[0] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                            v1718 = v1697 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v149[1] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                            v1720 = v149[0];
                                            v1722 = v149[1];
                                            for (uint v1724 = 0u; v1724 < 2u; )
                                            {
                                                float2 v1728 = v1720 + v1722;
                                                float2 v1729 = v1728 * 0.5f;
                                                float v1736 = v1697 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v1729 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                                float v1737 = v1736 * v1715;
                                                bool v1738 = v1737 < 0.0f;
                                                bool2 v1739 = v1738.xx;
                                                v1715 = v1738 ? v1715 : v1736;
                                                v1718 = v1738 ? v1736 : v1718;
                                                v1720 = float2(v1739.x ? v1720.x : v1729.x, v1739.y ? v1720.y : v1729.y);
                                                v1722 = float2(v1739.x ? v1729.x : v1722.x, v1739.y ? v1729.y : v1722.y);
                                                v1724++;
                                                continue;
                                            }
                                            float2 v1744 = lerp(v1722, v1720, clamp(v1718 / (v1718 - v1715), 0.0f, 1.0f).xx);
                                            float v1745 = v1744.x;
                                            float3 v1747 = float3(v1745, v362, v1744.y);
                                            float v1765;
                                            float v1768;
                                            float2 v1770;
                                            float2 v1772;
                                            v1765 = v1697 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v149[3] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                            v1768 = v1697 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v149[2] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                            v1770 = v149[3];
                                            v1772 = v149[2];
                                            for (uint v1774 = 0u; v1774 < 2u; )
                                            {
                                                float2 v1778 = v1770 + v1772;
                                                float2 v1779 = v1778 * 0.5f;
                                                float v1786 = v1697 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v1779 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                                float v1787 = v1786 * v1765;
                                                bool v1788 = v1787 < 0.0f;
                                                bool2 v1789 = v1788.xx;
                                                v1765 = v1788 ? v1765 : v1786;
                                                v1768 = v1788 ? v1786 : v1768;
                                                v1770 = float2(v1789.x ? v1770.x : v1779.x, v1789.y ? v1770.y : v1779.y);
                                                v1772 = float2(v1789.x ? v1779.x : v1772.x, v1789.y ? v1779.y : v1772.y);
                                                v1774++;
                                                continue;
                                            }
                                            float2 v1803;
                                            float v1804;
                                            bool v1806;
                                            float2 v1794 = lerp(v1772, v1770, clamp(v1768 / (v1768 - v1765), 0.0f, 1.0f).xx);
                                            float v1796 = v1794.y;
                                            float3 v1797 = float3(v1794.x, v362, v1796);
                                            float3 v1798 = lerp(v1747, v1797, 0.5f.xxx);
                                            float v1830;
                                            do
                                            {
                                                v1803 = (v1798.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                                v1804 = v1803.x;
                                                v1806 = !(v1804 < 0.0f);
                                                bool v1810;
                                                if (v1806)
                                                {
                                                    v1810 = v1804 > 1.0f;
                                                }
                                                else
                                                {
                                                    v1810 = true;
                                                }
                                                bool v1816;
                                                if (!v1810)
                                                {
                                                    v1816 = v1803.y < 0.0f;
                                                }
                                                else
                                                {
                                                    v1816 = true;
                                                }
                                                bool v1822;
                                                if (!v1816)
                                                {
                                                    v1822 = v1803.y > 1.0f;
                                                }
                                                else
                                                {
                                                    v1822 = true;
                                                }
                                                if (v1822)
                                                {
                                                    v1830 = 0.0f;
                                                    break;
                                                }
                                                else
                                                {
                                                    v1830 = rocky_terrain.SampleLevel(sampler_default, v1803, 0.0f).x;
                                                    break;
                                                }
                                                break; // unreachable workaround
                                            } while(false);
                                            if (v1830 < 0.00999999977648258209228515625f)
                                            {
                                                v2103 = v346;
                                                v2104 = v349;
                                                v2105 = v351;
                                                v2106 = v353;
                                                break;
                                            }
                                            bool v1854;
                                            if (smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (v1747.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f)
                                            {
                                                v1854 = smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (v1797.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f;
                                            }
                                            else
                                            {
                                                v1854 = false;
                                            }
                                            if (v1854)
                                            {
                                                v2103 = v346;
                                                v2104 = v349;
                                                v2105 = v351;
                                                v2106 = v353;
                                                break;
                                            }
                                            float v1857 = distance(v1747, v1797);
                                            if (v1857 < 0.20000000298023223876953125f)
                                            {
                                                v2103 = v346;
                                                v2104 = v349;
                                                v2105 = v351;
                                                v2106 = v353;
                                                break;
                                            }
                                            float4 v1862 = terrain_heightmap.GatherRed(sampler_default, v1803, int2(0, 0));
                                            float v1867 = v1862.z;
                                            float v1868 = v1862.y;
                                            float v1870 = v1862.w;
                                            float v1871 = v1862.x;
                                            float2 v1880 = ((float2((v1867 + v1868) - (v1870 + v1871), (v1871 + v1868) - (v1870 + v1867)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(terrain_heightmap_dims));
                                            float v1913;
                                            do
                                            {
                                                bool v1893;
                                                if (v1806)
                                                {
                                                    v1893 = v1804 > 1.0f;
                                                }
                                                else
                                                {
                                                    v1893 = true;
                                                }
                                                bool v1899;
                                                if (!v1893)
                                                {
                                                    v1899 = v1803.y < 0.0f;
                                                }
                                                else
                                                {
                                                    v1899 = true;
                                                }
                                                bool v1905;
                                                if (!v1899)
                                                {
                                                    v1905 = v1803.y > 1.0f;
                                                }
                                                else
                                                {
                                                    v1905 = true;
                                                }
                                                if (v1905)
                                                {
                                                    v1913 = 0.0f;
                                                    break;
                                                }
                                                else
                                                {
                                                    v1913 = rocky_terrain.SampleLevel(sampler_default, v1803, 0.0f).x;
                                                    break;
                                                }
                                                break; // unreachable workaround
                                            } while(false);
                                            float v1939;
                                            do
                                            {
                                                bool v1919;
                                                if (v1806)
                                                {
                                                    v1919 = v1804 > 1.0f;
                                                }
                                                else
                                                {
                                                    v1919 = true;
                                                }
                                                bool v1925;
                                                if (!v1919)
                                                {
                                                    v1925 = v1803.y < 0.0f;
                                                }
                                                else
                                                {
                                                    v1925 = true;
                                                }
                                                bool v1931;
                                                if (!v1925)
                                                {
                                                    v1931 = v1803.y > 1.0f;
                                                }
                                                else
                                                {
                                                    v1931 = true;
                                                }
                                                if (v1931)
                                                {
                                                    v1939 = 0.0f;
                                                    break;
                                                }
                                                else
                                                {
                                                    v1939 = rocky_terrain.SampleLevel(sampler_default, v1803, 0.0f).z;
                                                    break;
                                                }
                                                break; // unreachable workaround
                                            } while(false);
                                            float3 v1943 = cross(normalize(float3(0.0f, v1880.y, 1.0f)), normalize(float3(1.0f, v1880.x, 0.0f))) * lerp(0.0f, 0.300000011920928955078125f, clamp(v1913 + v1939, 0.0f, 1.0f));
                                            float3 v1944 = v1798 - v1943;
                                            float3 v1948 = normalize(v1797 - v1747);
                                            uint v1953 = uint(((v1745 + v362) + v1796) * 100.0f);
                                            float4x4 v1971 = transpose(float4x4(float4(v1948, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(cross(v1948, float3(0.0f, 1.0f, 0.0f)), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
                                            float4x4 v1973 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v1944.x), float4(0.0f, 1.0f, 0.0f, v1944.y - 0.5f), float4(0.0f, 0.0f, 1.0f, v1944.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(v1971, float4x4(float4(v1857, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.800000011920928955078125f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                            uint v1978;
                                            __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, v1978);
                                            uint v1992 = uint(int(v1978));
                                            float v1994[12] = { v1973[0].x, v1973[1].x, v1973[2].x, v1973[0].y, v1973[1].y, v1973[2].y, v1973[0].z, v1973[1].z, v1973[2].z, v1973[0].w, v1973[1].w, v1973[2].w };
                                            Affine3Packed v1995 = { v1994 };
                                            Stair v1996 = { v1995 };
                                            [unroll]
                                            for (int _32ident = 0; _32ident < 12; _32ident++)
                                            {
                                                __alias_stairs_buffer.Store<float>(_32ident * 4 + v1992 * 48 + 0, v1996.transform.data[_32ident]);
                                            }
                                            uint v1997 = v1953 ^ 23784u;
                                            uint v1999 = v1997 + (v1997 << 10u);
                                            uint v2001 = v1999 ^ (v1999 >> 6u);
                                            uint v2003 = v2001 + (v2001 << 3u);
                                            uint v2005 = v2003 ^ (v2003 >> 11u);
                                            bool v2008 = v353 ? v353 : true;
                                            uint v2009 = v353 ? v349 : 8388607u;
                                            bool v2010 = v351 ? v351 : true;
                                            uint v2011 = v351 ? v346 : 1065353216u;
                                            uint v2099;
                                            uint v2100;
                                            bool v2101;
                                            bool v2102;
                                            if ((asfloat(((v2005 + (v2005 << 15u)) & v2009) | v2011) - 1.0f) > 0.85000002384185791015625f)
                                            {
                                                uint v2019 = v1953 ^ 90455u;
                                                uint v2021 = v2019 + (v2019 << 10u);
                                                uint v2023 = v2021 ^ (v2021 >> 6u);
                                                uint v2025 = v2023 + (v2023 << 3u);
                                                uint v2027 = v2025 ^ (v2025 >> 11u);
                                                bool v2030 = v2008 ? v2008 : true;
                                                uint v2031 = v2008 ? v2009 : 8388607u;
                                                bool v2032 = v2010 ? v2010 : true;
                                                uint v2033 = v2010 ? v2011 : 1065353216u;
                                                float v2038 = lerp(0.20000000298023223876953125f, 0.4000000059604644775390625f, asfloat(((v2027 + (v2027 << 15u)) & v2031) | v2033) - 1.0f);
                                                uint v2039 = v1953 ^ 3845u;
                                                uint v2041 = v2039 + (v2039 << 10u);
                                                uint v2043 = v2041 ^ (v2041 >> 6u);
                                                uint v2045 = v2043 + (v2043 << 3u);
                                                uint v2047 = v2045 ^ (v2045 >> 11u);
                                                uint v2051 = v2030 ? v2031 : 8388607u;
                                                uint v2053 = v2032 ? v2033 : 1065353216u;
                                                float3 v2060 = lerp(v1747, v1797, (asfloat(((v2047 + (v2047 << 15u)) & v2051) | v2053) - 1.0f).xxx) - v1943;
                                                uint v2064;
                                                __alias_draw_commands.InterlockedAdd(pebble_indirect_draw_idx * 20 + 4, 1u, v2064);
                                                uint v2069 = pebble_buffer_offset + uint(int(v2064));
                                                float4x4 v2082 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v2060.x), float4(0.0f, 1.0f, 0.0f, v2060.y), float4(0.0f, 0.0f, 1.0f, v2060.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(v1971, float4x4(float4(v2038, 0.0f, 0.0f, 0.0f), float4(0.0f, v2038, 0.0f, 0.0f), float4(0.0f, 0.0f, v2038, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                                float v2096[12] = { v2082[0].x, v2082[1].x, v2082[2].x, v2082[0].y, v2082[1].y, v2082[2].y, v2082[0].z, v2082[1].z, v2082[2].z, v2082[0].w, v2082[1].w, v2082[2].w };
                                                Affine3Packed v2097 = { v2096 };
                                                Stair v2098 = { v2097 };
                                                [unroll]
                                                for (int _33ident = 0; _33ident < 12; _33ident++)
                                                {
                                                    __alias_stairs_buffer.Store<float>(_33ident * 4 + v2069 * 48 + 0, v2098.transform.data[_33ident]);
                                                }
                                                v2099 = v2053;
                                                v2100 = v2051;
                                                v2101 = v2032 ? v2032 : true;
                                                v2102 = v2030 ? v2030 : true;
                                            }
                                            else
                                            {
                                                v2099 = v2011;
                                                v2100 = v2009;
                                                v2101 = v2010;
                                                v2102 = v2008;
                                            }
                                            v2103 = v2099;
                                            v2104 = v2100;
                                            v2105 = v2101;
                                            v2106 = v2102;
                                            break;
                                        } while(false);
                                        v2107 = v2103;
                                        v2108 = v2104;
                                        v2109 = v2105;
                                        v2110 = v2106;
                                    }
                                    else
                                    {
                                        bool v454;
// case 7 / 8(三角点情形及其补):等离线连接边 0-1 与边 0-3
                                        if (!(v389 == 7))
                                        {
                                            v454 = v389 == 8;
                                        }
                                        else
                                        {
                                            v454 = true;
                                        }
                                        uint v1689;
                                        uint v1690;
                                        bool v1691;
                                        bool v1692;
                                        if (v454)
                                        {
                                            uint v1685;
                                            uint v1686;
                                            bool v1687;
                                            bool v1688;
                                            do
                                            {
                                                float v1282 = -v362;
                                                float2 v1284 = (v149[0] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                                float v1300;
                                                float v1303;
                                                float2 v1305;
                                                float2 v1307;
                                                v1300 = v1282 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, v1284, 0.0f).x);
                                                v1303 = v1282 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v149[1] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                                v1305 = v149[0];
                                                v1307 = v149[1];
                                                for (uint v1309 = 0u; v1309 < 2u; )
                                                {
                                                    float2 v1313 = v1305 + v1307;
                                                    float2 v1314 = v1313 * 0.5f;
                                                    float v1321 = v1282 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v1314 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                                    float v1322 = v1321 * v1300;
                                                    bool v1323 = v1322 < 0.0f;
                                                    bool2 v1324 = v1323.xx;
                                                    v1300 = v1323 ? v1300 : v1321;
                                                    v1303 = v1323 ? v1321 : v1303;
                                                    v1305 = float2(v1324.x ? v1305.x : v1314.x, v1324.y ? v1305.y : v1314.y);
                                                    v1307 = float2(v1324.x ? v1314.x : v1307.x, v1324.y ? v1314.y : v1307.y);
                                                    v1309++;
                                                    continue;
                                                }
                                                float2 v1329 = lerp(v1307, v1305, clamp(v1303 / (v1303 - v1300), 0.0f, 1.0f).xx);
                                                float v1330 = v1329.x;
                                                float3 v1332 = float3(v1330, v362, v1329.y);
                                                float v1347;
                                                float v1350;
                                                float2 v1352;
                                                float2 v1354;
                                                v1347 = v1282 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, v1284, 0.0f).x);
                                                v1350 = v1282 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v149[3] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                                v1352 = v149[0];
                                                v1354 = v149[3];
                                                for (uint v1356 = 0u; v1356 < 2u; )
                                                {
                                                    float2 v1360 = v1352 + v1354;
                                                    float2 v1361 = v1360 * 0.5f;
                                                    float v1368 = v1282 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v1361 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                                    float v1369 = v1368 * v1347;
                                                    bool v1370 = v1369 < 0.0f;
                                                    bool2 v1371 = v1370.xx;
                                                    v1347 = v1370 ? v1347 : v1368;
                                                    v1350 = v1370 ? v1368 : v1350;
                                                    v1352 = float2(v1371.x ? v1352.x : v1361.x, v1371.y ? v1352.y : v1361.y);
                                                    v1354 = float2(v1371.x ? v1361.x : v1354.x, v1371.y ? v1361.y : v1354.y);
                                                    v1356++;
                                                    continue;
                                                }
                                                float2 v1385;
                                                float v1386;
                                                bool v1388;
                                                float2 v1376 = lerp(v1354, v1352, clamp(v1350 / (v1350 - v1347), 0.0f, 1.0f).xx);
                                                float v1378 = v1376.y;
                                                float3 v1379 = float3(v1376.x, v362, v1378);
                                                float3 v1380 = lerp(v1332, v1379, 0.5f.xxx);
                                                float v1412;
                                                do
                                                {
                                                    v1385 = (v1380.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                                    v1386 = v1385.x;
                                                    v1388 = !(v1386 < 0.0f);
                                                    bool v1392;
                                                    if (v1388)
                                                    {
                                                        v1392 = v1386 > 1.0f;
                                                    }
                                                    else
                                                    {
                                                        v1392 = true;
                                                    }
                                                    bool v1398;
                                                    if (!v1392)
                                                    {
                                                        v1398 = v1385.y < 0.0f;
                                                    }
                                                    else
                                                    {
                                                        v1398 = true;
                                                    }
                                                    bool v1404;
                                                    if (!v1398)
                                                    {
                                                        v1404 = v1385.y > 1.0f;
                                                    }
                                                    else
                                                    {
                                                        v1404 = true;
                                                    }
                                                    if (v1404)
                                                    {
                                                        v1412 = 0.0f;
                                                        break;
                                                    }
                                                    else
                                                    {
                                                        v1412 = rocky_terrain.SampleLevel(sampler_default, v1385, 0.0f).x;
                                                        break;
                                                    }
                                                    break; // unreachable workaround
                                                } while(false);
                                                if (v1412 < 0.00999999977648258209228515625f)
                                                {
                                                    v1685 = v346;
                                                    v1686 = v349;
                                                    v1687 = v351;
                                                    v1688 = v353;
                                                    break;
                                                }
                                                bool v1436;
                                                if (smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (v1332.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f)
                                                {
                                                    v1436 = smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (v1379.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f;
                                                }
                                                else
                                                {
                                                    v1436 = false;
                                                }
                                                if (v1436)
                                                {
                                                    v1685 = v346;
                                                    v1686 = v349;
                                                    v1687 = v351;
                                                    v1688 = v353;
                                                    break;
                                                }
                                                float v1439 = distance(v1332, v1379);
                                                if (v1439 < 0.20000000298023223876953125f)
                                                {
                                                    v1685 = v346;
                                                    v1686 = v349;
                                                    v1687 = v351;
                                                    v1688 = v353;
                                                    break;
                                                }
                                                float4 v1444 = terrain_heightmap.GatherRed(sampler_default, v1385, int2(0, 0));
                                                float v1449 = v1444.z;
                                                float v1450 = v1444.y;
                                                float v1452 = v1444.w;
                                                float v1453 = v1444.x;
                                                float2 v1462 = ((float2((v1449 + v1450) - (v1452 + v1453), (v1453 + v1450) - (v1452 + v1449)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(terrain_heightmap_dims));
                                                float v1495;
                                                do
                                                {
                                                    bool v1475;
                                                    if (v1388)
                                                    {
                                                        v1475 = v1386 > 1.0f;
                                                    }
                                                    else
                                                    {
                                                        v1475 = true;
                                                    }
                                                    bool v1481;
                                                    if (!v1475)
                                                    {
                                                        v1481 = v1385.y < 0.0f;
                                                    }
                                                    else
                                                    {
                                                        v1481 = true;
                                                    }
                                                    bool v1487;
                                                    if (!v1481)
                                                    {
                                                        v1487 = v1385.y > 1.0f;
                                                    }
                                                    else
                                                    {
                                                        v1487 = true;
                                                    }
                                                    if (v1487)
                                                    {
                                                        v1495 = 0.0f;
                                                        break;
                                                    }
                                                    else
                                                    {
                                                        v1495 = rocky_terrain.SampleLevel(sampler_default, v1385, 0.0f).x;
                                                        break;
                                                    }
                                                    break; // unreachable workaround
                                                } while(false);
                                                float v1521;
                                                do
                                                {
                                                    bool v1501;
                                                    if (v1388)
                                                    {
                                                        v1501 = v1386 > 1.0f;
                                                    }
                                                    else
                                                    {
                                                        v1501 = true;
                                                    }
                                                    bool v1507;
                                                    if (!v1501)
                                                    {
                                                        v1507 = v1385.y < 0.0f;
                                                    }
                                                    else
                                                    {
                                                        v1507 = true;
                                                    }
                                                    bool v1513;
                                                    if (!v1507)
                                                    {
                                                        v1513 = v1385.y > 1.0f;
                                                    }
                                                    else
                                                    {
                                                        v1513 = true;
                                                    }
                                                    if (v1513)
                                                    {
                                                        v1521 = 0.0f;
                                                        break;
                                                    }
                                                    else
                                                    {
                                                        v1521 = rocky_terrain.SampleLevel(sampler_default, v1385, 0.0f).z;
                                                        break;
                                                    }
                                                    break; // unreachable workaround
                                                } while(false);
                                                float3 v1525 = cross(normalize(float3(0.0f, v1462.y, 1.0f)), normalize(float3(1.0f, v1462.x, 0.0f))) * lerp(0.0f, 0.300000011920928955078125f, clamp(v1495 + v1521, 0.0f, 1.0f));
                                                float3 v1526 = v1380 - v1525;
                                                float3 v1530 = normalize(v1379 - v1332);
                                                uint v1535 = uint(((v1330 + v362) + v1378) * 100.0f);
                                                float4x4 v1553 = transpose(float4x4(float4(v1530, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(cross(v1530, float3(0.0f, 1.0f, 0.0f)), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
                                                float4x4 v1555 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v1526.x), float4(0.0f, 1.0f, 0.0f, v1526.y - 0.5f), float4(0.0f, 0.0f, 1.0f, v1526.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(v1553, float4x4(float4(v1439, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.800000011920928955078125f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                                uint v1560;
                                                __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, v1560);
                                                uint v1574 = uint(int(v1560));
                                                float v1576[12] = { v1555[0].x, v1555[1].x, v1555[2].x, v1555[0].y, v1555[1].y, v1555[2].y, v1555[0].z, v1555[1].z, v1555[2].z, v1555[0].w, v1555[1].w, v1555[2].w };
                                                Affine3Packed v1577 = { v1576 };
                                                Stair v1578 = { v1577 };
                                                [unroll]
                                                for (int _34ident = 0; _34ident < 12; _34ident++)
                                                {
                                                    __alias_stairs_buffer.Store<float>(_34ident * 4 + v1574 * 48 + 0, v1578.transform.data[_34ident]);
                                                }
                                                uint v1579 = v1535 ^ 23784u;
                                                uint v1581 = v1579 + (v1579 << 10u);
                                                uint v1583 = v1581 ^ (v1581 >> 6u);
                                                uint v1585 = v1583 + (v1583 << 3u);
                                                uint v1587 = v1585 ^ (v1585 >> 11u);
                                                bool v1590 = v353 ? v353 : true;
                                                uint v1591 = v353 ? v349 : 8388607u;
                                                bool v1592 = v351 ? v351 : true;
                                                uint v1593 = v351 ? v346 : 1065353216u;
                                                uint v1681;
                                                uint v1682;
                                                bool v1683;
                                                bool v1684;
                                                if ((asfloat(((v1587 + (v1587 << 15u)) & v1591) | v1593) - 1.0f) > 0.85000002384185791015625f)
                                                {
                                                    uint v1601 = v1535 ^ 90455u;
                                                    uint v1603 = v1601 + (v1601 << 10u);
                                                    uint v1605 = v1603 ^ (v1603 >> 6u);
                                                    uint v1607 = v1605 + (v1605 << 3u);
                                                    uint v1609 = v1607 ^ (v1607 >> 11u);
                                                    bool v1612 = v1590 ? v1590 : true;
                                                    uint v1613 = v1590 ? v1591 : 8388607u;
                                                    bool v1614 = v1592 ? v1592 : true;
                                                    uint v1615 = v1592 ? v1593 : 1065353216u;
                                                    float v1620 = lerp(0.20000000298023223876953125f, 0.4000000059604644775390625f, asfloat(((v1609 + (v1609 << 15u)) & v1613) | v1615) - 1.0f);
                                                    uint v1621 = v1535 ^ 3845u;
                                                    uint v1623 = v1621 + (v1621 << 10u);
                                                    uint v1625 = v1623 ^ (v1623 >> 6u);
                                                    uint v1627 = v1625 + (v1625 << 3u);
                                                    uint v1629 = v1627 ^ (v1627 >> 11u);
                                                    uint v1633 = v1612 ? v1613 : 8388607u;
                                                    uint v1635 = v1614 ? v1615 : 1065353216u;
                                                    float3 v1642 = lerp(v1332, v1379, (asfloat(((v1629 + (v1629 << 15u)) & v1633) | v1635) - 1.0f).xxx) - v1525;
                                                    uint v1646;
                                                    __alias_draw_commands.InterlockedAdd(pebble_indirect_draw_idx * 20 + 4, 1u, v1646);
                                                    uint v1651 = pebble_buffer_offset + uint(int(v1646));
                                                    float4x4 v1664 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v1642.x), float4(0.0f, 1.0f, 0.0f, v1642.y), float4(0.0f, 0.0f, 1.0f, v1642.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(v1553, float4x4(float4(v1620, 0.0f, 0.0f, 0.0f), float4(0.0f, v1620, 0.0f, 0.0f), float4(0.0f, 0.0f, v1620, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                                    float v1678[12] = { v1664[0].x, v1664[1].x, v1664[2].x, v1664[0].y, v1664[1].y, v1664[2].y, v1664[0].z, v1664[1].z, v1664[2].z, v1664[0].w, v1664[1].w, v1664[2].w };
                                                    Affine3Packed v1679 = { v1678 };
                                                    Stair v1680 = { v1679 };
                                                    [unroll]
                                                    for (int _35ident = 0; _35ident < 12; _35ident++)
                                                    {
                                                        __alias_stairs_buffer.Store<float>(_35ident * 4 + v1651 * 48 + 0, v1680.transform.data[_35ident]);
                                                    }
                                                    v1681 = v1635;
                                                    v1682 = v1633;
                                                    v1683 = v1614 ? v1614 : true;
                                                    v1684 = v1612 ? v1612 : true;
                                                }
                                                else
                                                {
                                                    v1681 = v1593;
                                                    v1682 = v1591;
                                                    v1683 = v1592;
                                                    v1684 = v1590;
                                                }
                                                v1685 = v1681;
                                                v1686 = v1682;
                                                v1687 = v1683;
                                                v1688 = v1684;
                                                break;
                                            } while(false);
                                            v1689 = v1685;
                                            v1690 = v1686;
                                            v1691 = v1687;
                                            v1692 = v1688;
                                        }
                                        else
                                        {
                                            uint v1274;
                                            uint v1275;
                                            bool v1276;
                                            bool v1277;
// case 10(另一鞍形):两条不相连的等离线线段,分别放置台阶
                                            if (v389 == 10)
                                            {
                                                float2 v463;
                                                float v465;
                                                float2 v467;
                                                float2 v516;
                                                float2 v523;
                                                uint v868;
                                                uint v869;
                                                bool v870;
                                                bool v871;
                                                do
                                                {
                                                    v463 = v149[0];
                                                    v465 = -v362;
                                                    v467 = (v463 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                                    float2 v476 = (v149[1] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                                    float v483;
                                                    float v486;
                                                    float2 v488;
                                                    float2 v490;
                                                    v483 = v465 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, v467, 0.0f).x);
                                                    v486 = v465 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, v476, 0.0f).x);
                                                    v488 = v463;
                                                    v490 = v149[1];
                                                    for (uint v492 = 0u; v492 < 2u; )
                                                    {
                                                        float2 v496 = v488 + v490;
                                                        float2 v497 = v496 * 0.5f;
                                                        float v504 = v465 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v497 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                                        float v505 = v504 * v483;
                                                        bool v506 = v505 < 0.0f;
                                                        bool2 v507 = v506.xx;
                                                        v483 = v506 ? v483 : v504;
                                                        v486 = v506 ? v504 : v486;
                                                        v488 = float2(v507.x ? v488.x : v497.x, v507.y ? v488.y : v497.y);
                                                        v490 = float2(v507.x ? v497.x : v490.x, v507.y ? v497.y : v490.y);
                                                        v492++;
                                                        continue;
                                                    }
                                                    float2 v512 = lerp(v490, v488, clamp(v486 / (v486 - v483), 0.0f, 1.0f).xx);
                                                    float v513 = v512.x;
                                                    float3 v515 = float3(v513, v362, v512.y);
                                                    v516 = v149[2];
                                                    v523 = (v516 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                                    float v530;
                                                    float v533;
                                                    float2 v535;
                                                    float2 v537;
                                                    v530 = v465 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, v476, 0.0f).x);
                                                    v533 = v465 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, v523, 0.0f).x);
                                                    v535 = v149[1];
                                                    v537 = v516;
                                                    for (uint v539 = 0u; v539 < 2u; )
                                                    {
                                                        float2 v543 = v535 + v537;
                                                        float2 v544 = v543 * 0.5f;
                                                        float v551 = v465 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v544 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                                        float v552 = v551 * v530;
                                                        bool v553 = v552 < 0.0f;
                                                        bool2 v554 = v553.xx;
                                                        v530 = v553 ? v530 : v551;
                                                        v533 = v553 ? v551 : v533;
                                                        v535 = float2(v554.x ? v535.x : v544.x, v554.y ? v535.y : v544.y);
                                                        v537 = float2(v554.x ? v544.x : v537.x, v554.y ? v544.y : v537.y);
                                                        v539++;
                                                        continue;
                                                    }
                                                    float2 v568;
                                                    float v569;
                                                    bool v571;
                                                    float2 v559 = lerp(v537, v535, clamp(v533 / (v533 - v530), 0.0f, 1.0f).xx);
                                                    float v561 = v559.y;
                                                    float3 v562 = float3(v559.x, v362, v561);
                                                    float3 v563 = lerp(v515, v562, 0.5f.xxx);
                                                    float v595;
                                                    do
                                                    {
                                                        v568 = (v563.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                                        v569 = v568.x;
                                                        v571 = !(v569 < 0.0f);
                                                        bool v575;
                                                        if (v571)
                                                        {
                                                            v575 = v569 > 1.0f;
                                                        }
                                                        else
                                                        {
                                                            v575 = true;
                                                        }
                                                        bool v581;
                                                        if (!v575)
                                                        {
                                                            v581 = v568.y < 0.0f;
                                                        }
                                                        else
                                                        {
                                                            v581 = true;
                                                        }
                                                        bool v587;
                                                        if (!v581)
                                                        {
                                                            v587 = v568.y > 1.0f;
                                                        }
                                                        else
                                                        {
                                                            v587 = true;
                                                        }
                                                        if (v587)
                                                        {
                                                            v595 = 0.0f;
                                                            break;
                                                        }
                                                        else
                                                        {
                                                            v595 = rocky_terrain.SampleLevel(sampler_default, v568, 0.0f).x;
                                                            break;
                                                        }
                                                        break; // unreachable workaround
                                                    } while(false);
                                                    if (v595 < 0.00999999977648258209228515625f)
                                                    {
                                                        v868 = v346;
                                                        v869 = v349;
                                                        v870 = v351;
                                                        v871 = v353;
                                                        break;
                                                    }
                                                    bool v619;
                                                    if (smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (v515.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f)
                                                    {
                                                        v619 = smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (v562.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f;
                                                    }
                                                    else
                                                    {
                                                        v619 = false;
                                                    }
                                                    if (v619)
                                                    {
                                                        v868 = v346;
                                                        v869 = v349;
                                                        v870 = v351;
                                                        v871 = v353;
                                                        break;
                                                    }
                                                    float v622 = distance(v515, v562);
                                                    if (v622 < 0.20000000298023223876953125f)
                                                    {
                                                        v868 = v346;
                                                        v869 = v349;
                                                        v870 = v351;
                                                        v871 = v353;
                                                        break;
                                                    }
                                                    float4 v627 = terrain_heightmap.GatherRed(sampler_default, v568, int2(0, 0));
                                                    float v632 = v627.z;
                                                    float v633 = v627.y;
                                                    float v635 = v627.w;
                                                    float v636 = v627.x;
                                                    float2 v645 = ((float2((v632 + v633) - (v635 + v636), (v636 + v633) - (v635 + v632)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(terrain_heightmap_dims));
                                                    float v678;
                                                    do
                                                    {
                                                        bool v658;
                                                        if (v571)
                                                        {
                                                            v658 = v569 > 1.0f;
                                                        }
                                                        else
                                                        {
                                                            v658 = true;
                                                        }
                                                        bool v664;
                                                        if (!v658)
                                                        {
                                                            v664 = v568.y < 0.0f;
                                                        }
                                                        else
                                                        {
                                                            v664 = true;
                                                        }
                                                        bool v670;
                                                        if (!v664)
                                                        {
                                                            v670 = v568.y > 1.0f;
                                                        }
                                                        else
                                                        {
                                                            v670 = true;
                                                        }
                                                        if (v670)
                                                        {
                                                            v678 = 0.0f;
                                                            break;
                                                        }
                                                        else
                                                        {
                                                            v678 = rocky_terrain.SampleLevel(sampler_default, v568, 0.0f).x;
                                                            break;
                                                        }
                                                        break; // unreachable workaround
                                                    } while(false);
                                                    float v704;
                                                    do
                                                    {
                                                        bool v684;
                                                        if (v571)
                                                        {
                                                            v684 = v569 > 1.0f;
                                                        }
                                                        else
                                                        {
                                                            v684 = true;
                                                        }
                                                        bool v690;
                                                        if (!v684)
                                                        {
                                                            v690 = v568.y < 0.0f;
                                                        }
                                                        else
                                                        {
                                                            v690 = true;
                                                        }
                                                        bool v696;
                                                        if (!v690)
                                                        {
                                                            v696 = v568.y > 1.0f;
                                                        }
                                                        else
                                                        {
                                                            v696 = true;
                                                        }
                                                        if (v696)
                                                        {
                                                            v704 = 0.0f;
                                                            break;
                                                        }
                                                        else
                                                        {
                                                            v704 = rocky_terrain.SampleLevel(sampler_default, v568, 0.0f).z;
                                                            break;
                                                        }
                                                        break; // unreachable workaround
                                                    } while(false);
                                                    float3 v708 = cross(normalize(float3(0.0f, v645.y, 1.0f)), normalize(float3(1.0f, v645.x, 0.0f))) * lerp(0.0f, 0.300000011920928955078125f, clamp(v678 + v704, 0.0f, 1.0f));
                                                    float3 v709 = v563 - v708;
                                                    float3 v713 = normalize(v562 - v515);
                                                    uint v718 = uint(((v513 + v362) + v561) * 100.0f);
                                                    float4x4 v736 = transpose(float4x4(float4(v713, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(cross(v713, float3(0.0f, 1.0f, 0.0f)), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
                                                    float4x4 v738 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v709.x), float4(0.0f, 1.0f, 0.0f, v709.y - 0.5f), float4(0.0f, 0.0f, 1.0f, v709.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(v736, float4x4(float4(v622, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.800000011920928955078125f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                                    uint v743;
                                                    __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, v743);
                                                    uint v757 = uint(int(v743));
                                                    float v759[12] = { v738[0].x, v738[1].x, v738[2].x, v738[0].y, v738[1].y, v738[2].y, v738[0].z, v738[1].z, v738[2].z, v738[0].w, v738[1].w, v738[2].w };
                                                    Affine3Packed v760 = { v759 };
                                                    Stair v761 = { v760 };
                                                    [unroll]
                                                    for (int _36ident = 0; _36ident < 12; _36ident++)
                                                    {
                                                        __alias_stairs_buffer.Store<float>(_36ident * 4 + v757 * 48 + 0, v761.transform.data[_36ident]);
                                                    }
                                                    uint v762 = v718 ^ 23784u;
                                                    uint v764 = v762 + (v762 << 10u);
                                                    uint v766 = v764 ^ (v764 >> 6u);
                                                    uint v768 = v766 + (v766 << 3u);
                                                    uint v770 = v768 ^ (v768 >> 11u);
                                                    bool v773 = v353 ? v353 : true;
                                                    uint v774 = v353 ? v349 : 8388607u;
                                                    bool v775 = v351 ? v351 : true;
                                                    uint v776 = v351 ? v346 : 1065353216u;
                                                    uint v864;
                                                    uint v865;
                                                    bool v866;
                                                    bool v867;
                                                    if ((asfloat(((v770 + (v770 << 15u)) & v774) | v776) - 1.0f) > 0.85000002384185791015625f)
                                                    {
                                                        uint v784 = v718 ^ 90455u;
                                                        uint v786 = v784 + (v784 << 10u);
                                                        uint v788 = v786 ^ (v786 >> 6u);
                                                        uint v790 = v788 + (v788 << 3u);
                                                        uint v792 = v790 ^ (v790 >> 11u);
                                                        bool v795 = v773 ? v773 : true;
                                                        uint v796 = v773 ? v774 : 8388607u;
                                                        bool v797 = v775 ? v775 : true;
                                                        uint v798 = v775 ? v776 : 1065353216u;
                                                        float v803 = lerp(0.20000000298023223876953125f, 0.4000000059604644775390625f, asfloat(((v792 + (v792 << 15u)) & v796) | v798) - 1.0f);
                                                        uint v804 = v718 ^ 3845u;
                                                        uint v806 = v804 + (v804 << 10u);
                                                        uint v808 = v806 ^ (v806 >> 6u);
                                                        uint v810 = v808 + (v808 << 3u);
                                                        uint v812 = v810 ^ (v810 >> 11u);
                                                        uint v816 = v795 ? v796 : 8388607u;
                                                        uint v818 = v797 ? v798 : 1065353216u;
                                                        float3 v825 = lerp(v515, v562, (asfloat(((v812 + (v812 << 15u)) & v816) | v818) - 1.0f).xxx) - v708;
                                                        uint v829;
                                                        __alias_draw_commands.InterlockedAdd(pebble_indirect_draw_idx * 20 + 4, 1u, v829);
                                                        uint v834 = pebble_buffer_offset + uint(int(v829));
                                                        float4x4 v847 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v825.x), float4(0.0f, 1.0f, 0.0f, v825.y), float4(0.0f, 0.0f, 1.0f, v825.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(v736, float4x4(float4(v803, 0.0f, 0.0f, 0.0f), float4(0.0f, v803, 0.0f, 0.0f), float4(0.0f, 0.0f, v803, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                                        float v861[12] = { v847[0].x, v847[1].x, v847[2].x, v847[0].y, v847[1].y, v847[2].y, v847[0].z, v847[1].z, v847[2].z, v847[0].w, v847[1].w, v847[2].w };
                                                        Affine3Packed v862 = { v861 };
                                                        Stair v863 = { v862 };
                                                        [unroll]
                                                        for (int _37ident = 0; _37ident < 12; _37ident++)
                                                        {
                                                            __alias_stairs_buffer.Store<float>(_37ident * 4 + v834 * 48 + 0, v863.transform.data[_37ident]);
                                                        }
                                                        v864 = v818;
                                                        v865 = v816;
                                                        v866 = v797 ? v797 : true;
                                                        v867 = v795 ? v795 : true;
                                                    }
                                                    else
                                                    {
                                                        v864 = v776;
                                                        v865 = v774;
                                                        v866 = v775;
                                                        v867 = v773;
                                                    }
                                                    v868 = v864;
                                                    v869 = v865;
                                                    v870 = v866;
                                                    v871 = v867;
                                                    break;
                                                } while(false);
                                                uint v1270;
                                                uint v1271;
                                                bool v1272;
                                                bool v1273;
                                                do
                                                {
                                                    float2 v881 = (v149[3] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                                    float v888;
                                                    float v891;
                                                    float2 v893;
                                                    float2 v895;
                                                    v888 = v465 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, v467, 0.0f).x);
                                                    v891 = v465 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, v881, 0.0f).x);
                                                    v893 = v463;
                                                    v895 = v149[3];
                                                    for (uint v897 = 0u; v897 < 2u; )
                                                    {
                                                        float2 v901 = v893 + v895;
                                                        float2 v902 = v901 * 0.5f;
                                                        float v909 = v465 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v902 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                                        float v910 = v909 * v888;
                                                        bool v911 = v910 < 0.0f;
                                                        bool2 v912 = v911.xx;
                                                        v888 = v911 ? v888 : v909;
                                                        v891 = v911 ? v909 : v891;
                                                        v893 = float2(v912.x ? v893.x : v902.x, v912.y ? v893.y : v902.y);
                                                        v895 = float2(v912.x ? v902.x : v895.x, v912.y ? v902.y : v895.y);
                                                        v897++;
                                                        continue;
                                                    }
                                                    float2 v917 = lerp(v895, v893, clamp(v891 / (v891 - v888), 0.0f, 1.0f).xx);
                                                    float v918 = v917.x;
                                                    float3 v920 = float3(v918, v362, v917.y);
                                                    float v932;
                                                    float v935;
                                                    float2 v937;
                                                    float2 v939;
                                                    v932 = v465 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, v881, 0.0f).x);
                                                    v935 = v465 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, v523, 0.0f).x);
                                                    v937 = v149[3];
                                                    v939 = v516;
                                                    for (uint v941 = 0u; v941 < 2u; )
                                                    {
                                                        float2 v945 = v937 + v939;
                                                        float2 v946 = v945 * 0.5f;
                                                        float v953 = v465 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v946 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                                        float v954 = v953 * v932;
                                                        bool v955 = v954 < 0.0f;
                                                        bool2 v956 = v955.xx;
                                                        v932 = v955 ? v932 : v953;
                                                        v935 = v955 ? v953 : v935;
                                                        v937 = float2(v956.x ? v937.x : v946.x, v956.y ? v937.y : v946.y);
                                                        v939 = float2(v956.x ? v946.x : v939.x, v956.y ? v946.y : v939.y);
                                                        v941++;
                                                        continue;
                                                    }
                                                    float2 v970;
                                                    float v971;
                                                    bool v973;
                                                    float2 v961 = lerp(v939, v937, clamp(v935 / (v935 - v932), 0.0f, 1.0f).xx);
                                                    float v963 = v961.y;
                                                    float3 v964 = float3(v961.x, v362, v963);
                                                    float3 v965 = lerp(v920, v964, 0.5f.xxx);
                                                    float v997;
                                                    do
                                                    {
                                                        v970 = (v965.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                                        v971 = v970.x;
                                                        v973 = !(v971 < 0.0f);
                                                        bool v977;
                                                        if (v973)
                                                        {
                                                            v977 = v971 > 1.0f;
                                                        }
                                                        else
                                                        {
                                                            v977 = true;
                                                        }
                                                        bool v983;
                                                        if (!v977)
                                                        {
                                                            v983 = v970.y < 0.0f;
                                                        }
                                                        else
                                                        {
                                                            v983 = true;
                                                        }
                                                        bool v989;
                                                        if (!v983)
                                                        {
                                                            v989 = v970.y > 1.0f;
                                                        }
                                                        else
                                                        {
                                                            v989 = true;
                                                        }
                                                        if (v989)
                                                        {
                                                            v997 = 0.0f;
                                                            break;
                                                        }
                                                        else
                                                        {
                                                            v997 = rocky_terrain.SampleLevel(sampler_default, v970, 0.0f).x;
                                                            break;
                                                        }
                                                        break; // unreachable workaround
                                                    } while(false);
                                                    if (v997 < 0.00999999977648258209228515625f)
                                                    {
                                                        v1270 = v868;
                                                        v1271 = v869;
                                                        v1272 = v870;
                                                        v1273 = v871;
                                                        break;
                                                    }
                                                    bool v1021;
                                                    if (smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (v920.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f)
                                                    {
                                                        v1021 = smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (v964.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f;
                                                    }
                                                    else
                                                    {
                                                        v1021 = false;
                                                    }
                                                    if (v1021)
                                                    {
                                                        v1270 = v868;
                                                        v1271 = v869;
                                                        v1272 = v870;
                                                        v1273 = v871;
                                                        break;
                                                    }
                                                    float v1024 = distance(v920, v964);
                                                    if (v1024 < 0.20000000298023223876953125f)
                                                    {
                                                        v1270 = v868;
                                                        v1271 = v869;
                                                        v1272 = v870;
                                                        v1273 = v871;
                                                        break;
                                                    }
                                                    float4 v1029 = terrain_heightmap.GatherRed(sampler_default, v970, int2(0, 0));
                                                    float v1034 = v1029.z;
                                                    float v1035 = v1029.y;
                                                    float v1037 = v1029.w;
                                                    float v1038 = v1029.x;
                                                    float2 v1047 = ((float2((v1034 + v1035) - (v1037 + v1038), (v1038 + v1035) - (v1037 + v1034)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(terrain_heightmap_dims));
                                                    float v1080;
                                                    do
                                                    {
                                                        bool v1060;
                                                        if (v973)
                                                        {
                                                            v1060 = v971 > 1.0f;
                                                        }
                                                        else
                                                        {
                                                            v1060 = true;
                                                        }
                                                        bool v1066;
                                                        if (!v1060)
                                                        {
                                                            v1066 = v970.y < 0.0f;
                                                        }
                                                        else
                                                        {
                                                            v1066 = true;
                                                        }
                                                        bool v1072;
                                                        if (!v1066)
                                                        {
                                                            v1072 = v970.y > 1.0f;
                                                        }
                                                        else
                                                        {
                                                            v1072 = true;
                                                        }
                                                        if (v1072)
                                                        {
                                                            v1080 = 0.0f;
                                                            break;
                                                        }
                                                        else
                                                        {
                                                            v1080 = rocky_terrain.SampleLevel(sampler_default, v970, 0.0f).x;
                                                            break;
                                                        }
                                                        break; // unreachable workaround
                                                    } while(false);
                                                    float v1106;
                                                    do
                                                    {
                                                        bool v1086;
                                                        if (v973)
                                                        {
                                                            v1086 = v971 > 1.0f;
                                                        }
                                                        else
                                                        {
                                                            v1086 = true;
                                                        }
                                                        bool v1092;
                                                        if (!v1086)
                                                        {
                                                            v1092 = v970.y < 0.0f;
                                                        }
                                                        else
                                                        {
                                                            v1092 = true;
                                                        }
                                                        bool v1098;
                                                        if (!v1092)
                                                        {
                                                            v1098 = v970.y > 1.0f;
                                                        }
                                                        else
                                                        {
                                                            v1098 = true;
                                                        }
                                                        if (v1098)
                                                        {
                                                            v1106 = 0.0f;
                                                            break;
                                                        }
                                                        else
                                                        {
                                                            v1106 = rocky_terrain.SampleLevel(sampler_default, v970, 0.0f).z;
                                                            break;
                                                        }
                                                        break; // unreachable workaround
                                                    } while(false);
                                                    float3 v1110 = cross(normalize(float3(0.0f, v1047.y, 1.0f)), normalize(float3(1.0f, v1047.x, 0.0f))) * lerp(0.0f, 0.300000011920928955078125f, clamp(v1080 + v1106, 0.0f, 1.0f));
                                                    float3 v1111 = v965 - v1110;
                                                    float3 v1115 = normalize(v964 - v920);
                                                    uint v1120 = uint(((v918 + v362) + v963) * 100.0f);
                                                    float4x4 v1138 = transpose(float4x4(float4(v1115, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(cross(v1115, float3(0.0f, 1.0f, 0.0f)), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
                                                    float4x4 v1140 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v1111.x), float4(0.0f, 1.0f, 0.0f, v1111.y - 0.5f), float4(0.0f, 0.0f, 1.0f, v1111.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(v1138, float4x4(float4(v1024, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.800000011920928955078125f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                                    uint v1145;
                                                    __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, v1145);
                                                    uint v1159 = uint(int(v1145));
                                                    float v1161[12] = { v1140[0].x, v1140[1].x, v1140[2].x, v1140[0].y, v1140[1].y, v1140[2].y, v1140[0].z, v1140[1].z, v1140[2].z, v1140[0].w, v1140[1].w, v1140[2].w };
                                                    Affine3Packed v1162 = { v1161 };
                                                    Stair v1163 = { v1162 };
                                                    [unroll]
                                                    for (int _38ident = 0; _38ident < 12; _38ident++)
                                                    {
                                                        __alias_stairs_buffer.Store<float>(_38ident * 4 + v1159 * 48 + 0, v1163.transform.data[_38ident]);
                                                    }
                                                    uint v1164 = v1120 ^ 23784u;
                                                    uint v1166 = v1164 + (v1164 << 10u);
                                                    uint v1168 = v1166 ^ (v1166 >> 6u);
                                                    uint v1170 = v1168 + (v1168 << 3u);
                                                    uint v1172 = v1170 ^ (v1170 >> 11u);
                                                    bool v1175 = v871 ? v871 : true;
                                                    uint v1176 = v871 ? v869 : 8388607u;
                                                    bool v1177 = v870 ? v870 : true;
                                                    uint v1178 = v870 ? v868 : 1065353216u;
                                                    uint v1266;
                                                    uint v1267;
                                                    bool v1268;
                                                    bool v1269;
                                                    if ((asfloat(((v1172 + (v1172 << 15u)) & v1176) | v1178) - 1.0f) > 0.85000002384185791015625f)
                                                    {
                                                        uint v1186 = v1120 ^ 90455u;
                                                        uint v1188 = v1186 + (v1186 << 10u);
                                                        uint v1190 = v1188 ^ (v1188 >> 6u);
                                                        uint v1192 = v1190 + (v1190 << 3u);
                                                        uint v1194 = v1192 ^ (v1192 >> 11u);
                                                        bool v1197 = v1175 ? v1175 : true;
                                                        uint v1198 = v1175 ? v1176 : 8388607u;
                                                        bool v1199 = v1177 ? v1177 : true;
                                                        uint v1200 = v1177 ? v1178 : 1065353216u;
                                                        float v1205 = lerp(0.20000000298023223876953125f, 0.4000000059604644775390625f, asfloat(((v1194 + (v1194 << 15u)) & v1198) | v1200) - 1.0f);
                                                        uint v1206 = v1120 ^ 3845u;
                                                        uint v1208 = v1206 + (v1206 << 10u);
                                                        uint v1210 = v1208 ^ (v1208 >> 6u);
                                                        uint v1212 = v1210 + (v1210 << 3u);
                                                        uint v1214 = v1212 ^ (v1212 >> 11u);
                                                        uint v1218 = v1197 ? v1198 : 8388607u;
                                                        uint v1220 = v1199 ? v1200 : 1065353216u;
                                                        float3 v1227 = lerp(v920, v964, (asfloat(((v1214 + (v1214 << 15u)) & v1218) | v1220) - 1.0f).xxx) - v1110;
                                                        uint v1231;
                                                        __alias_draw_commands.InterlockedAdd(pebble_indirect_draw_idx * 20 + 4, 1u, v1231);
                                                        uint v1236 = pebble_buffer_offset + uint(int(v1231));
                                                        float4x4 v1249 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, v1227.x), float4(0.0f, 1.0f, 0.0f, v1227.y), float4(0.0f, 0.0f, 1.0f, v1227.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(v1138, float4x4(float4(v1205, 0.0f, 0.0f, 0.0f), float4(0.0f, v1205, 0.0f, 0.0f), float4(0.0f, 0.0f, v1205, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                                        float v1263[12] = { v1249[0].x, v1249[1].x, v1249[2].x, v1249[0].y, v1249[1].y, v1249[2].y, v1249[0].z, v1249[1].z, v1249[2].z, v1249[0].w, v1249[1].w, v1249[2].w };
                                                        Affine3Packed v1264 = { v1263 };
                                                        Stair v1265 = { v1264 };
                                                        [unroll]
                                                        for (int _39ident = 0; _39ident < 12; _39ident++)
                                                        {
                                                            __alias_stairs_buffer.Store<float>(_39ident * 4 + v1236 * 48 + 0, v1265.transform.data[_39ident]);
                                                        }
                                                        v1266 = v1220;
                                                        v1267 = v1218;
                                                        v1268 = v1199 ? v1199 : true;
                                                        v1269 = v1197 ? v1197 : true;
                                                    }
                                                    else
                                                    {
                                                        v1266 = v1178;
                                                        v1267 = v1176;
                                                        v1268 = v1177;
                                                        v1269 = v1175;
                                                    }
                                                    v1270 = v1266;
                                                    v1271 = v1267;
                                                    v1272 = v1268;
                                                    v1273 = v1269;
                                                    break;
                                                } while(false);
                                                v1274 = v1270;
                                                v1275 = v1271;
                                                v1276 = v1272;
                                                v1277 = v1273;
                                            }
                                            else
                                            {
                                                v1274 = v346;
                                                v1275 = v349;
                                                v1276 = v351;
                                                v1277 = v353;
                                            }
                                            v1689 = v1274;
                                            v1690 = v1275;
                                            v1691 = v1276;
                                            v1692 = v1277;
                                        }
                                        v2107 = v1689;
                                        v2108 = v1690;
                                        v2109 = v1691;
                                        v2110 = v1692;
                                    }
                                    v2924 = v2107;
                                    v2925 = v2108;
                                    v2926 = v2109;
                                    v2927 = v2110;
                                }
                                v3339 = v2924;
                                v3340 = v2925;
                                v3341 = v2926;
                                v3342 = v2927;
                            }
                            v3757 = v3339;
                            v3758 = v3340;
                            v3759 = v3341;
                            v3760 = v3342;
                        }
                        v4172 = v3757;
                        v4173 = v3758;
                        v4174 = v3759;
                        v4175 = v3760;
                    }
                    v4587 = v4172;
                    v4588 = v4173;
                    v4589 = v4174;
                    v4590 = v4175;
                }
                v347 = v4587;
                v350 = v4588;
                v352 = v4589;
                v354 = v4590;
            }
        }
        break;
    } while(false);
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
