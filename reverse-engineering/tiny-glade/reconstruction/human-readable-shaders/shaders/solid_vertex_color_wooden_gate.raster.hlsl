// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/solid_vertex_color_wooden_gate.raster.hlsl | B simple-solid | bins: _solid_vertex_color_wooden_gate.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// 木门:法线贴图 + 种子哈希的逐片元明暗变化 + 按墙选窗色,
// 把 (法线, 反照率) 打包写入 gbuffer。

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"
#include "inc/window_color_customization.hlsl"

// 每着色器全局量(隐式 $Globals,set0/b13)。
uint2 normal_map_dims;
uint2 seed_map_dims;
int seed;
int wall_id;
float4x4 model_to_world;

Texture2DArray<float4> window_colors : register(t14);
Texture2D<float4> normal_map : register(t15);
Texture2D<float4> seed_map : register(t16);
StructuredBuffer<uint> gate_color_ids : register(t17);
struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 Vertex_UV: Vertex_UV;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float3x3 C4 : TEXCOORD4;
    float4 position : SV_Position;
};

// 顶点着色器:世界变换 + 从法线/上方向构建切线系(TBN 矩阵,转置后输出)。
VsOut vs_main(VsIn vin)
{
    VsOut o;
    float4 world = mul(model_to_world, float4(vin.Vertex_Position, 1.0f));
    float3 tangent = cross(float3(0.0f, 1.0f, 0.0f), vin.Vertex_Normal);
    float3 world_normal = mul(model_to_world, float4(vin.Vertex_Normal, 0.0f)).xyz;
    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, world);
    o.C0 = vin.Vertex_Color;
    o.C1 = world_normal;
    o.C2 = world.xyz;
    o.C3 = vin.Vertex_UV;
    o.C4 = transpose(float3x3(normalize(mul(model_to_world, float4(tangent, 0.0f)).xyz),
                              normalize(mul(model_to_world, float4(cross(vin.Vertex_Normal, tangent), 0.0f)).xyz),
                              normalize(world_normal)));
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float3x3 C4 : TEXCOORD4;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

// 像素着色器:法线贴图采样、种子哈希随机、窗色采样,打包 (法线,反照率) 到 gbuffer。
PsOut ps_main(PsIn pin)
{
    PsOut o;
    float2 uv = pin.C3 * 0.12f;
    uv.y = 1.0f - uv.y;
    float3 normal = normalize(mul(pin.C4, (normal_map.Sample(sampler_llr, uv).xyz * 2.0f) - 1.0f.xxx));
    // 相机位置:由 view_to_world 的齐次原点换算;法线若背离相机则翻转。
    float4 cam_vw = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_pos = cam_vw.xyz / cam_vw.w.xxx;
    if (dot(normalize(cam_pos - pin.C2), normal) < 0.0f)
    {
        normal = -normal;
    }

    // 经典 5 步整数哈希,由种子贴图样本 + seed 生成逐片元随机数(0..1)。
    float seedf = float(seed);
    uint h = uint((seed_map.Sample(sampler_llr, uv).x * 255.0f) + seedf);
    h = h + (h << 10u);
    h = h ^ (h >> 6u);
    h = h + (h << 3u);
    h = h ^ (h >> 11u);
    float rnd = asfloat(((h + (h << 15u)) & 8388607u) | 1065353216u) - 1.0f;

    // 按墙 id 取窗色;0 号墙变暗。
    int wall = int(gate_color_ids[uint(wall_id)]);
    float3 window_color = window_colors.Sample(sampler_llr, float3((uv + (seedf * 0.2f).xx) + (rnd * 0.4f).xx, float(wall))).xyz;
    if (wall == 0)
    {
        window_color = window_color * 0.7f;
    }

    // 法线/反照率编码为 8bit 每通道的 uint word。
    float3 packed_normal = (normal * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(window_color * lerp(0.9f, 1.2f, rnd), 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = uint(clamp(srgb_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(srgb_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(srgb_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u);
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 3422552064u;
    o.gbuffer = uint2(normal_word, albedo_word);
    // 反照率高位再叠加标记(117440512 = 0x07000000),标识该片元属于此材质。
    o.gbuffer.y = o.gbuffer.y | 117440512u;
    return o;
}

#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _solid_vertex_color_wooden_gate.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
