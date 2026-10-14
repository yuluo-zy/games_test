// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/rocky_terrain/rocky_terrain.raster.hlsl | F-R | bins: _rocky_terrain_rocky_terrain.raster.hlsl_b903e43ffb3da915.bin, _rocky_terrain_rocky_terrain.raster.hlsl_f0adff76827e73ff.bin | entry vs:vs_main (vs_6_4) ps:ps_main (ps_6_4)
// 【文件用途】岩石地形网格栅格化通道。顶点着色器按 gl_VertexIndex 索引位移网格
//   SSBO(每三角形 3 个顶点):输出位移后的位置、三角形 id(vertexIndex/3)、
//   重心角点权重(vertexIndex%3 的 one-hot)与该角点的 cell_id。自上而下
//   (force_top_down_render) 投影用 xz/65 重映射平面坐标、高度用 clamp((y+2.5)/26.5),
//   服务于游戏的自上而下工具视图。像素着色器做每三角形的裂缝修复
//   (triangle_adjacency.inc.hlsl):在三角形最近边附近,着色法线向邻接三角形
//   面法线混合,混合带由哈希驱动的单纯形噪声扰动;随后做三平面岩石纹理细节采样
//   (在 world-uv/30 上做 3 次 SampleBias,带 cell_id 抖动,按亮度+法线权重混合),
//   并把 RGB8 反照率 + RGB8 法线打包进 G-Buffer。
// 【绑定】$Globals b13、vertices t15(StructuredBuffer<Vertex>)、
//   triangles t16(StructuredBuffer<Triangle>)、frame_constants b0/space1(vs)
//   | sampler_mipmap s10、vertices t15、triangles t16、rocky_terrain_texture
//   t18、frame_constants b0/space1(ps)。能力:RuntimeDescriptorArray
//   (强制,SSBO 上的 SPV_EXT_descriptor_indexing)+ 两个模块都带 Shader。
// 【证据】spirv-cross 反编译 _rocky_terrain_rocky_terrain.raster.hlsl_b903e43ffb3da915.bin
//   (workspace/f-r-scratch/modules/_rocky_terrain_rocky_terrain.raster.hlsl_b903e43ffb3da915_{0,1}.hlsl.cross),
//   数学/顺序逐字转写(Vertex.position 在字节 16 读取、cell_id 在字节 28;
//   Triangle.neighbours/is_top/normal 在 0/12/16)。f0ad 变体仅在 PS 上不同
//   (空的 ps_main,无绑定)——本文件以 b903 变体为目标。
// 【状态】O(算法/顺序/字面量取自 .cross);inc 层 OpSource/OpString
//   + dxc 版本 opcode 差异是预期的(已记录,不修复)。
// 重写说明:手写重建段补充中文注释;转译/提起段(TGV_f0adff... 内)逐字保留。

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/math.hlsl"
#include "inc/color_library.hlsl"
#include "rocky_terrain/inc_mesh.inc.hlsl"
#include "terrain_editing/inc_highlight_active_terrain_chunk.inc.hlsl"
#include "inc/simplex_noise.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/texture_blend.hlsl"
#include "rocky_terrain/triangle_adjacency.inc.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"

// 原始 dxc 构建对结构化缓冲 SSBO 生成 SPV_EXT_descriptor_indexing +
// RuntimeDescriptorArray;这里强制以匹配目标模块。
[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
StructuredBuffer<Vertex> vertices : register(t15, space0);
[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
StructuredBuffer<Triangle> triangles : register(t16, space0);

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;        // 当前视图索引
    uint2 water_mask_dims;          // 水掩码尺寸
    uint2 rocky_terrain_texture_dims;  // 岩石纹理尺寸
    uint2 highlight_active_hill_dims;  // 高亮激活山丘尺寸
    uint force_top_down_render;     // 是否强制自上而下投影
};

Texture2D<float4> rocky_terrain_texture : register(t18, space0);

struct vs_main_output
{
    float3 c0 : TEXCOORD0;                 // 世界位置
    nointerpolation int c1 : TEXCOORD1;    // 三角形 id
    float3 c2 : TEXCOORD2;                 // 重心角点权重(one-hot)
    nointerpolation int c3 : TEXCOORD3;    // 角点 cell_id
    float4 position : SV_Position;
};

vs_main_output vs_main(uint vertex_id : SV_VertexID)
{
    // 读取位移后的顶点位置
    float3 position = vertices[vertex_id].position;

    float4 clip_pos;
    if (force_top_down_render != 0u)
    {
        // 自上而下投影:平面坐标缩放 1/65,高度映射到 [0,1]
        clip_pos = float4(position.xz * float2(0.015384615398943424224853515625f, -0.015384615398943424224853515625f),
                           clamp((position.y - (-2.5f)) * 0.0377358496189117431640625f, 0.0f, 1.0f),
                           1.0f);
    }
    else
    {
        // 常规透视投影
        clip_pos = mul(view_constants[current_view_index].world_to_sample, float4(position, 1.0f));
    }

    vs_main_output output;
    output.position = clip_pos;
    output.c0 = position;
    output.c1 = int(vertex_id / 3u);  // 每 3 个顶点一个三角形
    // 重心角点权重:角点索引 = vertex_id % 3,置 1
    float3 corner_weight = 0.0f;
    corner_weight[vertex_id % 3u] = 1.0f;
    output.c2 = corner_weight;
    output.c3 = vertices[vertex_id].cell_id;
    return output;
}

struct ps_main_input
{
    float3 c0 : TEXCOORD0;
    nointerpolation int c1 : TEXCOORD1;
    float3 c2 : TEXCOORD2;
    nointerpolation int c3 : TEXCOORD3;
};

uint2 ps_main(ps_main_input input) : SV_Target0
{
    uint tri = uint(input.c1);

    // 裂缝修复邻接:由插值后的重心权重取最近三角形边,以及该边对侧的
    // 邻接三角形(边界上为 -1)。
    uint edge = nearest_edge_index(input.c2);
    int neighbour = triangles[tri].neighbours[edge];
    uint neighbour_u = uint(neighbour);

    // 片元到最近边的距离(仅当该边存在邻接三角形时计算;下面的混合仍会
    // 作用于越界读取,与原始内联完全一致)。
    float edge_distance;
    if (neighbour != -1)
    {
        float3 edge_start = vertices[tri * 3u + edge].position;
        float3 edge_end = vertices[tri * 3u + (edge + 1u) % 3u].position;
        edge_distance = distance_to_edge(input.c0, edge_start, edge_end);
    }
    else
    {
        edge_distance = -1.0f;
    }

    // 两侧都是"脊顶"三角形时混合带更窄(0.2),否则较宽(0.08)
    bool both_top = (triangles[tri].is_top == 1) && (triangles[neighbour_u].is_top == 1);
    float blend_width = both_top ? 0.20000000298023223876953125f : 0.07999999821186065673828125f;

    // 哈希驱动的 2D 值/梯度噪声(Ashima permute + 归一化常量、m^4 衰减),
    // 作用于单元随机的世界位置;它扰动边距离,使裂缝修复混合带不是一条直线。
    float2 noise_p = float2(input.c0.x, input.c0.z)
                   + (float(input.c3) * 0.100000001490116119384765625f + input.c0.y * 0.5f).xx;
    float2 i = floor(noise_p + dot(noise_p, 0.3660254180431365966796875f.xx));
    float2 x0 = (noise_p - i) + dot(i, 0.211324870586395263671875f.xx);
    float2 i1 = (x0.x > x0.y) ? float2(1.0f, 0.0f) : float2(0.0f, 1.0f);
    float4 c = x0.xyxy + float4(0.211324870586395263671875f, 0.211324870586395263671875f,
                                -0.57735025882720947265625f, -0.57735025882720947265625f);
    float2 x1 = c.xy - i1;
    float4 x12 = float4(x1.x, x1.y, c.z, c.w);
    // 格点取模后构造梯度随机值
    float2 i_mod = fmod(i, 289.0f.xx);
    float3 p0a = i_mod.y.xxx + float3(0.0f, i1.y, 1.0f);
    float3 p0 = fmod((p0a * 34.0f + 1.0f) * p0a, 289.0f) + i_mod.x.xxx + float3(0.0f, i1.x, 1.0f);
    float3 m_fall = max(0.5f - float3(dot(x0, x0), dot(x1, x1), dot(x12.zw, x12.zw)), 0.0f);
    float3 m = m_fall * m_fall;
    float3 g_raw = frac(fmod((p0 * 34.0f + 1.0f) * p0, 289.0f) * 0.024390242993831634521484375f) * 2.0f;
    float3 g = g_raw - 1.0f;
    float3 s = abs(g) - 0.5f;
    float3 a0 = g - floor(g_raw - 0.5f);
    float3 grad;
    grad.x = a0.x * x0.x + s.x * x0.y;
    grad.y = a0.y * x1.x + s.y * x1.y;
    grad.z = a0.z * x12.z + s.z * x12.w;
    float noise = 130.0f * dot((m * m) * (1.792842864990234375f - ((a0 * a0 + s * s) * 0.8537347316741943359375f)), grad);

    // 在共享边附近把着色法线向邻接面法线混合。
    // 噪声扰动后的边距离决定混合带起点;两个 smoothstep 组合得到混合系数。
    float edge_noise = edge_distance + noise * (both_top ? 0.5f : 0.25f);
    float t0 = smoothstep(0.0f, blend_width, edge_noise);
    float t1 = smoothstep(0.0f, blend_width * 0.5f, edge_distance);
    float blend = (1.0f - min(t0, t1)) * 0.5f;
    float3 n_raw = lerp(triangles[tri].normal, triangles[neighbour_u].normal, blend);

    // 岩石纹理的三平面细节采样:世界位置缩放 1/30,三个轴各采样一次,
    // 按亮度 + 法线权重混合。
    float3 world_uv = input.c0 * 0.0333333350718021392822265625f;
    float jitter_v = rand01(hash(uint(input.c3)));
    float2 jitter = float2(jitter_v, jitter_v);
    float3 sx = rocky_terrain_texture.SampleBias(sampler_mipmap, world_uv.zy + jitter, global_mip_bias).xyz;
    float3 sy = rocky_terrain_texture.SampleBias(sampler_mipmap, world_uv.xy + jitter, global_mip_bias).xyz;
    float3 sz = rocky_terrain_texture.SampleBias(sampler_mipmap, world_uv.xz + jitter, global_mip_bias).xyz;

    // 用法线绝对值 + 亮度为三个轴向采样赋权,再做归一化加权混合
    float3 abs_normal = pow(abs(n_raw), 1.0f);
    const float3 luminance = float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f);
    float lum_sy = dot(sy, luminance);
    float w0 = dot(sx, luminance) + abs_normal.x;
    float w1 = lum_sy + abs_normal.z;
    float w2 = lum_sy + abs_normal.y;
    // 减去最大权重使主导轴向占优,剩余权重归一化
    float max_w = max(max(w0, w1), w2) - 0.20000000298023223876953125f;
    float c0 = max(w0 - max_w, 0.0f);
    float c1 = max(w1 - max_w, 0.0f);
    float c2 = max(w2 - max_w, 0.0f);
    float3 rock = ((sx * c0) + (sy * c1) + (sz * c2)) / (c0 + c1 + c2) * 1.10000002384185791015625f;

    // 主题 id == 4 时按朝上程度与高度把岩石色向灰色淡化(模拟积雪/高光)
    float3 n = normalize(n_raw);
    float3 rock_color;
    if (glade_theme_id == 4u)
    {
        rock_color = lerp(rock, 0.699999988079071044921875f,
                          smoothstep(0.5f, 1.0f, clamp(n.y, 0.0f, 1.0f))
                          * smoothstep(-0.100000001490116119384765625f, 0.0f, input.c0.y));
    }
    else
    {
        rock_color = rock;
    }

    // 打包到 G-Buffer:法线 (RGB8) + 反照率 (RGB8)
    float3 packed_normal = n * 0.5f + 0.5f;
    float3 sqrt_color = sqrt(clamp(rock_color, 0.0f, 1.0f));

    uint color_packed = (uint((clamp(sqrt_color.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
                       + (uint((clamp(sqrt_color.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u))
                       + (uint((clamp(sqrt_color.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);

    uint normal_packed = (uint((clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
                        + (uint((clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u))
                        + (uint((clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)
                        + 3422552064u;

    uint2 result = uint2(normal_packed, color_packed);
    result.y = color_packed | 369098752u;  // 材质位标记
    return result;
}

#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// 提起自 _rocky_terrain_rocky_terrain.raster.hlsl_f0adff76827e73ff.bin(spirv-cross;行为完全一致,名字是 SPIR-V id)
// 【说明】f0ad 变体的 PS 为空函数(该变体不产生像素输出),此段逐字保留。
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
void frag_main()
{
}

void ps_main()
{
    frag_main();
}
#pragma dxc diagnostic pop
#endif
