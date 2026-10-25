// ==RECONSTRUCTED== shaders/rocky_terrain/displace_rocky_terrain.cs.hlsl | F-R | bin: _rocky_terrain_displace_rocky_terrain.cs.hlsl_cs_main_0.bin | entry cs:cs_main | LocalSize [64,1,1] | profile cs_6_4
// 【文件用途】岩石地形顶点/三角形位移计算。一个线程负责一个三角形
//   (gid.x:顶点 [3x, 3x+1, 3x+2] 即三角形 x)。当 3 个顶点都不落在岩石/水区域内时,
//   把第一个顶点写回一个 NaN 位置;否则对每个顶点做位移:
//   地形高度采样(超出 130 单位范围时下降 2.0)、岩石/路径/水掩码侵蚀、
//   每顶点值噪声(perlin,0.2 世界尺度)、对非角点、非路径顶点的
//   采集梯度"带状"位移(由 1 倍频 fbm 调制),最后用位移后的位置重算三角形法线。
// 【绑定】push_constants(推送常量)、sampler_llc s3、sampler_llb s4、
//   sampler_default s9、__alias_vertices u13(RWStructuredBuffer<Vertex>)、
//   __alias_triangles u14、$Globals b15、terrain_heightmap t16、
//   rocky_terrain t18、path_mask t19、water_mask t20
// 【证据】spirv-cross 反编译 _rocky_terrain_displace_rocky_terrain.cs.hlsl_cs_main_0.bin
//   (workspace/f-r-scratch/modules/_rocky_terrain_displace_rocky_terrain.cs.hlsl_cs_main_0_0.hlsl.cross),
//   数学/顺序逐字转写。spirv-cross 的 phi 传递的每顶点噪声"旋转"状态可归约为
//   标准 perlin 常量:select 链(1664525/1013904223 种子、0x7FFFFF 尾数掩码、0x3F800000 底数)
//   使初始默认值在 3 个顶点与 1 倍频 fbm 中不变地传播,因此格点与 inc/perlin.hlsl 的
//   hash(uint2) 一致。
// 【状态】O(算法/顺序/字面量取自 .cross);残留的 do-while/phi 与 opcode 数量差异
//   是 dxc 版本产物(已记录,不修复)。
// 重写说明:手写重建段,仅补充中文注释,代码与算术逐字保留。

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "rocky_terrain/inc_mesh.inc.hlsl"
#include "inc/perlin.hlsl"
#include "inc/fbm_noise.hlsl"
Texture2D<float4> terrain_heightmap : register(t16, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/uv.hlsl"
#include "inc/rocky_terrain.inc.hlsl"
#include "inc/texture_utils.hlsl"
#include "paths/path_mask_sampler.inc.hlsl"

// 顶点/三角形记录布局来自 rocky_terrain/inc_mesh.inc.hlsl
// (重建版;此前该 inc 还是骨架时在此内联)。
//
RWStructuredBuffer<Vertex> __alias_vertices : register(u13, space0);
RWStructuredBuffer<Triangle> __alias_triangles : register(u14, space0);

cbuffer $Globals : register(b15, space0)
{
    uint2 terrain_heightmap_dims;        // 地形高度图尺寸
    uint2 visual_terrain_heightmap_dims; // 视觉地形高度图尺寸
    uint2 rocky_terrain_dims;            // 岩石纹理尺寸
    uint2 path_mask_dims;                // 路径掩码尺寸
    uint2 water_mask_dims;               // 水掩码尺寸
};

struct PushConstants
{
    uint triangle_count;    // 需处理的三角形总数
};
[[vk::push_constant]]
PushConstants push_constants;

Texture2D<float4> rocky_terrain : register(t18, space0);
Texture2D<float4> path_mask : register(t19, space0);
Texture2D<float4> water_mask : register(t20, space0);

// 世界 xz -> 掩码 uv(1/130 世界尺度,居中于 .5)。
static const float TERRAIN_UV_SCALE = 0.0076923076994717121124267578125f;

// 带边界检查的岩石通道采样:当 uv 越出 [0,1]^2 时目标返回 0.0
// (.cross 中 do/while(false) 守卫;那里 z/x/y 分别采样,此处合并为一次守卫采样)。
float sample_rocky_terrain_channel(float2 uv, int channel)
{
    if (uv.x < 0.0f || uv.x > 1.0f || uv.y < 0.0f || uv.y > 1.0f)
    {
        return 0.0f;
    }
    float4 t = rocky_terrain.SampleLevel(sampler_default, uv, 0.0f);
    return (channel == 0) ? t.x : ((channel == 1) ? t.y : t.z);
}

// 2D 值噪声格点,逐字转写自目标的格点:每角点 hash(uint2)
// (perlin 旋转常量)、smoothstep 曲线,以及 ((1-w)*blend0) + ((n01*(1-u))*w + (n11*u)*w) 分组。
float value_noise(float2 p)
{
    int2 cell = int2(floor(p));
    float2 f = frac(p);
    // smoothstep 曲线
    float2 u = f * f * (3.0f - 2.0f * f);

    // 四个角点的随机值
    float n00 = incb_perlin_hash01(hash(uint2(cell + int2(0, 0))));
    float n10 = incb_perlin_hash01(hash(uint2(cell + int2(1, 0))));
    float n01 = incb_perlin_hash01(hash(uint2(cell + int2(0, 1))));
    float n11 = incb_perlin_hash01(hash(uint2(cell + int2(1, 1))));

    // 双线性插值
    return (((1.0f - u.y) * ((n00 * (1.0f - u.x)) + (n10 * u.x)))
          + (((n01 * (1.0f - u.x)) * u.y) + ((n11 * u.x) * u.y)));
}

[numthreads(64, 1, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    // 越界线程直接返回
    if (gl_GlobalInvocationID.x >= push_constants.triangle_count)
    {
        return;
    }

    // 本线程的三角形顶点基址(每个三角形 3 个连续顶点)
    uint triangle_base = gl_GlobalInvocationID.x * 3u;

    // 提前退出:若三角形 3 个顶点都不在岩石或水区域内,
    // 把第一个顶点写回一个 NaN 位置。
    bool displace = false;
    for (uint v = 0; v < 3; ++v)
    {
        // 顶点静止位置放大 65 倍 -> 世界坐标,再映射到掩码 uv
        float2 uv = (__alias_vertices[triangle_base + v].rest_position * 65.0f).xz
                    * TERRAIN_UV_SCALE + 0.5f;
        float r_z = sample_rocky_terrain_channel(uv, 2);
        float r_x = sample_rocky_terrain_channel(uv, 0);
        float r_y = sample_rocky_terrain_channel(uv, 1);
        // 岩石区域 = smoothstep 岩石强度 + 水掩码,钳制到 [0,1]
        float region = clamp(smoothstep(0.1f, 0.2f, r_x + r_y)
                           + smoothstep(0.0f, 0.2f, water_mask.SampleLevel(sampler_llc, uv, 0.0f).y), 0.0f, 1.0f);
        if (region > 0.0f || r_z > 0.0f)
        {
            displace = true;
            break;
        }
    }

    // 无任何顶点需要位移:写回 NaN 位置并结束
    if (!displace)
    {
        Vertex v0 = __alias_vertices[triangle_base];
        v0.position = asfloat(0x7fbfffffu).xxx;  // 0x7FBFFFFF = 最大有限 float 附近的 NaN 位型
        __alias_vertices[triangle_base] = v0;
        return;
    }

    // 对三角形的每个顶点做位移
    for (uint vi = 0; vi < 3; ++vi)
    {
        uint vertex_id = triangle_base + vi;
        Vertex vert = __alias_vertices[vertex_id];

        // 对顶点单元 id 做 5 步 wang hash -> 每顶点 [0,1) 随机数
        uint cell_hash = hash(uint(vert.cell_id));
        float cell_rand = asfloat((cell_hash & 0x7FFFFFu) | 0x3F800000u) - 1.0f;

        // 静止位置向/离单元质心抖动(小系数 0.001,幅度 lerp(-3,8,rand)),
        // 再放大 65 倍进入 130 单位世界空间。
        float3 jittered = vert.rest_position
                        + (vert.dir_to_centroid * 0.001f * lerp(-3.0f, 8.0f, cell_rand));
        float3 pos = jittered * 65.0f;
        float2 uv0 = (pos.xz * TERRAIN_UV_SCALE) + 0.5f;

        // 基础地形高度,映射到 [-2.5, 24];超出 130 单位范围时下降 2.0
        float h = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, uv0, 0.0f).x);
        float3 moved = pos;
        moved.y = h;
        if (abs(pos.x) > 65.0f || abs(pos.z) > 65.0f)
        {
            moved.y = h - 2.0f;
        }

        // 位移后位置处的岩石/水/路径掩码采样
        float2 uv1 = (moved.xz * TERRAIN_UV_SCALE) + 0.5f;
        float r_x1 = sample_rocky_terrain_channel(uv1, 0);
        float r_y1 = sample_rocky_terrain_channel(uv1, 1);
        float rock_region = clamp(smoothstep(0.1f, 0.2f, r_x1 + r_y1)
                                + smoothstep(0.0f, 0.2f, water_mask.SampleLevel(sampler_llc, uv1, 0.0f).y), 0.0f, 1.0f);
        float r_y2 = sample_rocky_terrain_channel(uv1, 1);
        float path_t = smoothstep(0.1f, 0.15f, path_mask.SampleLevel(sampler_llb, uv1, 0.0f).x);
        float r_z0 = sample_rocky_terrain_channel(uv0, 2);
        // 水的平滑过渡:到水线(0.5)的距离经 smoothstep 平滑
        float water_y = water_mask.SampleLevel(sampler_llc, uv1, 0.0f).y;
        float wd = abs(water_y - 0.5f);
        float water_smooth;
        if (wd > 1.0f)
        {
            water_smooth = 0.0f;
        }
        else
        {
            water_smooth = 1.0f - ((wd * wd) * (3.0f - (2.0f * wd)));
        }

        // 每顶点值噪声,0.2 世界尺度
        float noise = value_noise(moved.xz * 0.2f);

        // 组合侵蚀、路径压平与水平滑,由地形坡度调制(平坦区获得更多噪声)
        float3 result = moved;
        result.y = moved.y
            + lerp(-0.4f, 0.2f, clamp(rock_region + r_z0 - 10.0f * path_t, 0.0f, 1.0f))
            - smoothstep(0.5f, 0.6f, water_y) * 0.2f
            - (water_smooth * noise * (1.0f - smoothstep(0.3f, 0.5f,
                lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, uv1, 0.0f).x))));

        float3 displaced;
        if (rock_region > 0.0f)
        {
            // 地形梯度 + 世界法线:在基础映射 uv 上采集中心差分
            float4 gather = terrain_heightmap.GatherRed(sampler_default, uv0, int2(0, 0));
            float2 scale = 130.0f / float2(terrain_heightmap_dims);
            float2 grad = (float2((gather.z + gather.y) - (gather.w + gather.x),
                                  (gather.x + gather.y) - (gather.w + gather.z)) * 0.5f) * 26.5f / scale;
            float3 normal = cross(normalize(float3(0.0f, grad.y, 1.0f)),
                                  normalize(float3(1.0f, grad.x, 0.0f)));
            float4 rt = rocky_terrain.SampleLevel(sampler_default, uv0, 0.0f);
            float rock_t = rt.x;

            // 在岩石区域内沿地形法线滑动顶点,并沿路径向下拉
            float3 tilted = result
                + (normal * lerp(-0.3f, 0.1f * lerp(0.0f, 3.0f, cell_rand), vert.cell_bby)
                          * rock_region * (1.0f - path_t))
                - (normal * path_t);

            // 非角点且远离路径的顶点获得额外的带状法线偏移,由抖动位置的 1 倍频 fbm 调制
            if ((vert.is_corner == 0) && (path_t == 0.0f))
            {
                // 两处采集:原地与倾斜位置,求梯度长度
                float4 g1 = terrain_heightmap.GatherRed(sampler_default, uv0, int2(0, 0));
                float4 g2 = terrain_heightmap.GatherRed(sampler_default,
                    (tilted.xz * TERRAIN_UV_SCALE) + 0.5f, int2(0, 0));
                float grad1_len = length(((float2((g1.z + g1.y) - (g1.w + g1.x),
                                                  (g1.x + g1.y) - (g1.w + g1.z)) * 0.5f) * 26.5f) / scale);
                float grad2_len = length(((float2((g2.z + g2.y) - (g2.w + g2.x),
                                                  (g2.x + g2.y) - (g2.w + g2.z)) * 0.5f) * 26.5f) / scale);

                // 1 倍频 fbm(oct=0):偏移位置 = 抖动 xz*0.5 + 岩石强度*20
                float2 fbm_pos = (jittered.xz * 0.5f) + (rock_t * 20.0f).xx;
                float fbm_sum = 0.0f;
                float amp = 0.5f;
                for (int oct = 0; oct < 1; ++oct)
                {
                    fbm_sum += amp * (-1.0f + 2.0f * value_noise(fbm_pos));
                    amp *= 0.55f;
                    fbm_pos = mul(float2x2(float2(0.8f, -0.6f), float2(0.6f, 0.8f)), fbm_pos * 1.9f);
                }

                // 位移 = 倾斜位置 + 沿法线的带状偏移(梯度 1 长度 * 梯度 2 平滑衰减 * fbm)
                displaced = tilted
                    + (normal * lerp(-0.2f, 1.0f - smoothstep(4.0f, 7.0f, grad2_len),
                                     clamp((fbm_sum + 1.0f) * 0.5f, 0.0f, 1.0f))
                       * smoothstep(0.0f, 0.3f, rock_t) * grad1_len) * 0.3f
                    + (normal * r_y2);
            }
            else
            {
                displaced = tilted;
            }
        }
        else
        {
            displaced = result;
        }

        // 写回位移后的位置
        vert.position = displaced;
        __alias_vertices[vertex_id] = vert;
    }

    // 用位移后的位置重算三角形法线
    Triangle tri = __alias_triangles[gl_GlobalInvocationID.x];
    float3 p0 = __alias_vertices[triangle_base].position;
    float3 p1 = __alias_vertices[triangle_base + 1u].position;
    float3 p2 = __alias_vertices[triangle_base + 2u].position;
    tri.normal = normalize(cross(p2 - p0, p1 - p0));
    __alias_triangles[gl_GlobalInvocationID.x] = tri;
}
