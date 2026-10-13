// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/nani_cloth.raster.hlsl | B nani raster | bins: _nani_cloth.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// ============================================================
// Nani 布料着色器(用于旗帜/遮阳篷等织物挂布)。
//   顶点阶段 vs_main:
//     - 用两层二维风场晶格 + 竖直正弦波使布料摆动,底边按实例做下垂偏移;
//     - 逐实例读出仿射变换(xform)与世界坐标。
//   像素阶段 ps_main:
//     - 按 color_id 调色板选基色,并做逐实例 HSV 色相/饱和度/明度抖动;
//     - 可选混合"摩天轮图案"(ferris 纹理),采样绑定无索引法线贴图;
//     - 最后打包 G-Buffer(法线字 + 反照率字)。
//   绑定: instance_indirection_buffer(t14)、nani_instance_data(t15)、ferris(t16)。
//   变体: TGV_f0adff76827e73ff(仅像素阶段使用空 frag_main 占位)。
// ============================================================

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "nani/inc/user.hlsl"
#include "inc/perlin.hlsl"
#include "inc/wind.hlsl"
#include "inc/hsv.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"

uint2 ferris_dims;

struct InstanceData
{
    Affine3Packed xform;
    float hem_offset_low;
    float hem_offset_high;
    uint color_id;
    uint albedo_texture;
};

StructuredBuffer<uint> instance_indirection_buffer : register(t14);
StructuredBuffer<InstanceData> nani_instance_data : register(t15);
Texture2D<float4> ferris : register(t16);

// 对 uint2 做整数哈希,得到伪随机 32 位值(混合了移位/异或/乘加)。
uint hash2(uint2 p)
{
    uint y = p.y;
    y += (y << 10u);
    y ^= (y >> 6u);
    y += (y << 3u);
    y ^= (y >> 11u);
    p.x = (p.x * 1664525u) + (y + (y << 15u)) + 1013904223u;
    p.x *= 1664525u;
    uint x = p.x ^ (p.x >> 11u);
    x = x ^ ((x << 7u) & 2636928640u);
    x = x ^ ((x << 15u) & 4022730752u);
    return x;
}

// 把哈希值映射到 [0,1):用尾数位伪装成浮点后减 1.0。
float hash2_rand(uint h)
{
    return asfloat(((h ^ (h >> 18u)) & 8388607u) | 1065353216u) - 1.0f;
}

// 2D 值噪声:对四个晶格角点取哈希并做平滑插值,结果在 [-1,1]。
float noise2(float2 p)
{
    int2 cell = int2(floor(p));
    float2 fr = frac(p);
    float2 f = fr * fr * (3.0f.xx - (fr * 2.0f));
    float n = 0.0f;
    n += hash2_rand(hash2(uint2(cell))) * (1.0f - f.x) * (1.0f - f.y);
    n += hash2_rand(hash2(uint2(cell + int2(1, 0)))) * f.x * (1.0f - f.y);
    n += hash2_rand(hash2(uint2(cell + int2(0, 1)))) * (1.0f - f.x) * f.y;
    n += hash2_rand(hash2(uint2(cell + int2(1, 1)))) * f.x * f.y;
    return n;
}

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 Vertex_UV: Vertex_UV;
    [[vk::location(4)]] float3 tangent: tangent;
    [[vk::location(5)]] float3 bitangent: bitangent;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    nointerpolation uint C0 : TEXCOORD0;
    nointerpolation int C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float3 C4 : TEXCOORD4;
    float2 C5 : TEXCOORD5;
    float3x3 C6 : TEXCOORD6;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    // 实例化间接寻址:先从间接缓冲取真实实例下标,再读该实例的仿射变换。
    uint idx = instance_indirection_buffer[vin.instance_id];
    Affine3Packed packed = nani_instance_data[idx].xform;
    float3x4 xform = transpose(float4x3(float3(packed.data[0], packed.data[1], packed.data[2]),
                                        float3(packed.data[3], packed.data[4], packed.data[5]),
                                        float3(packed.data[6], packed.data[7], packed.data[8]),
                                        float3(packed.data[9], packed.data[10], packed.data[11])));
    // 变换原点(实例锚点)与全局动画时间。
    float3 origin = mul(xform, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float time = game_time_sec + game_time_frac;

    // 以 color_id 派生抖动种子。
    uint seed = uint(nani_instance_data[idx].color_id);
    seed = hash(seed);
    float flutter_seed = hash2_rand(seed);

    // 两层风场晶格:一层随世界 XZ 平移(布料/屋顶摆动),一层随时间与原点位置变化。
    float2 wind_a = (origin.xz * 0.1f) + float2(time, 0.0f) * 1.2f;
    float flutter_a = noise2(wind_a);
    float2 flutter_b = float2(time + origin.x, origin.y);
    float flutter_b_noise = noise2(flutter_b);

    // 竖直方向的正弦波相位;叠加噪声后只在顶点高度为负(下方)时才生效。
    float phase = sin(fmod(time * 1.8f, 6.2831855f) + game_time_frac * 1.8f + vin.Vertex_Position.y + flutter_seed * 0.5f);
    float z_flutter = phase + flutter_b_noise;
    z_flutter *= smoothstep(0.0f, -1.0f, vin.Vertex_Position.y);

    // 阵风调制:用第一层噪声驱动一个正弦包络,再缩放位移幅度。
    float gust = sin(((flutter_a * 3.14f) - 1.57f + 0.3f) * 0.7f) * lerp(0.7f, 1.0f, flutter_seed) * 0.2f;
    float displacement = (z_flutter * lerp(0.3f, 1.0f, gust)) * lerp(0.7f, 1.0f, flutter_seed) * 0.2f;
    float new_z = vin.Vertex_Position.z + displacement;

    // 用带位移的 Z 重建世界坐标,并按实例在底边处做垂向(下摆)偏移。
    float3 world = mul(xform, float4(vin.Vertex_Position.xy, new_z, 1.0f));
    world.y += lerp(nani_instance_data[idx].hem_offset_low, nani_instance_data[idx].hem_offset_high,
        (vin.Vertex_Position.x * 1.4285715f) + 0.5f);

    // 世界空间法线与裁剪坐标;透传色/切线标架供像素阶段使用。
    float3 normal_world = mul(xform, float4(vin.Vertex_Normal, 0.0f));
    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(world, 1.0f));
    o.C0 = idx;
    o.C1 = int((origin.x + origin.y) + origin.z);
    o.C2 = vin.Vertex_Color;
    o.C3 = normal_world;
    o.C4 = world;
    o.C5 = float2(vin.Vertex_UV.x, 1.0f - vin.Vertex_UV.y);
    o.C6 = transpose(float3x3(normalize(mul(xform, float4(vin.tangent, 0.0f))),
                              normalize(mul(xform, float4(vin.bitangent, 0.0f))),
                              normalize(normal_world)));
    return o;
}

struct PsIn
{
    nointerpolation uint C0 : TEXCOORD0;
    nointerpolation int C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float3 C4 : TEXCOORD4;
    float2 C5 : TEXCOORD5;
    float3x3 C6 : TEXCOORD6;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    // 调色板:color_id 对 7 取模选择布料的基色。
    uint color_id = uint(nani_instance_data[pin.C0].color_id);
    uint palette_idx = color_id % 7u;
    float3 base_color;
    if (palette_idx == 0u)
    {
        base_color = 0.7f.xxx;
    }
    else if (palette_idx == 1u)
    {
        base_color = float3(0.1137f, 0.15f, 0.0675f);
    }
    else if (palette_idx == 2u)
    {
        base_color = float3(0.049765f, 0.132287f, 0.11814f);
    }
    else if (palette_idx == 3u)
    {
        base_color = float3(0.173464f, 0.166082f, 0.064536f);
    }
    else
    {
        base_color = palette_idx == 4u ? float3(0.450245f, 0.265458f, 0.085742f) : 0.5f.xxx;
    }

    // RGB -> HSV:显式按通道大小排序,得到最大/最小通道与色相偏移量。
    float4 a;
    if (base_color.y < base_color.z)
    {
        a = float4(base_color.zy, -1.0f, 0.666667f);
    }
    else
    {
        a = float4(base_color.yz, 0.0f, -0.333333f);
    }
    float4 b;
    if (base_color.x < a.x)
    {
        b = float4(a.xyw, base_color.x);
    }
    else
    {
        b = float4(base_color.x, a.yzx);
    }
    // 通道极差 delta;以实例位置哈希对色相、饱和度、明度分别做随机抖动。
    float delta = b.x - min(b.w, b.y);
    uint seed_a = hash(uint(pin.C1) ^ 7230u);
    float hue = clamp(abs(((b.w - b.y) / ((6.0f * delta) + 1e-10f)) + b.z) + lerp(-0.1f, 0.05f, hash2_rand(seed_a)), 0.1f, 0.9f) * 6.0f;
    uint seed_b = hash(uint(pin.C1) ^ 890348u);
    // 用抖动后的色相重建 RGB,并按 delta 调整饱和度、按亮度调整明度。
    float3 retinted = ((clamp(float3(abs(hue - 3.0f) - 1.0f, 2.0f - abs(hue - 2.0f), 2.0f - abs(hue - 4.0f)), 0.0f.xxx, 1.0f.xxx) - 1.0f.xxx) * (delta / (b.x + 1e-10f)) + 1.0f.xxx) * (b.x + lerp(0.0f, 0.1f, hash2_rand(seed_b)));

    // color_id 末位为 1 的实例:叠加"摩天轮"图案(采样 ferris,亮度受 alpha 调制)。
    if ((color_id % 10u) == 1u)
    {
        float4 ferris_sample = ferris.Sample(sampler_llr, pin.C5 * 2.0f);
        retinted = lerp(retinted, pow(ferris_sample.xyz, 1.3f.xxx), (ferris_sample.w * 0.6f).xxx);
    }

    // 采样绑定无索引的法线贴图,转换到世界空间并确保朝向相机。
    uint tex_idx = nani_instance_data[pin.C0].albedo_texture;
    float3 normal_map = bindless_textures_dyn[NonUniformResourceIndex(tex_idx)].Sample(sampler_llr, pin.C5).xyz * 2.0f - 1.0f.xxx;
    float3 normal = normalize(mul(pin.C6, normal_map));
    float4 cam_vw = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_pos = cam_vw.xyz / cam_vw.w.xxx;
    // 若法线背离相机则翻转,保证双面渲染下光照方向正确。
    if (dot(normalize(cam_pos - pin.C4), normal) <= 0.0f)
    {
        normal = -normal;
    }

    // 打包 G-Buffer:法线压到 [0,1] 再 8bit 量化,反照率做 sqrt(线性→sRGB) 后 8bit 量化。
    float3 packed_normal = (normalize(normal) * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(retinted, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = uint(clamp(srgb_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(srgb_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(srgb_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u);
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 3422552064u;
    o.gbuffer = uint2(normal_word, albedo_word);
    // 高位标志:指示该像素来自 Nani 布料/特殊材质。
    o.gbuffer.y = o.gbuffer.y | 117440512u;
    return o;
}

#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _nani_cloth.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
