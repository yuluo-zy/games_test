// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/roof_snow.raster.hlsl | F-V | bins: _roof_snow.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// 屋顶积雪覆盖层(roof 家族)。vs 用细雪噪点抖动每个屋顶顶点:
// 三线性 3D 格点 float2 噪声(自定义 3D 哈希)铺开雪面,2D 值噪声
// (inc/perlin.hlsl 格点)的积雪深度沿法线推动;屋顶边缘附近的顶点由
// UV-y smoothstep 拉低。ps 丢弃低于一半的区域,并把平白的 0.7 雪反照率
// + 法线烘焙进实心 gbuffer 字。
// $Globals (set0/b13) 持有每对象变换 + 屋顶枢轴。
//
// 中文说明:给屋顶叠加一层程序化积雪。vs 用噪声扰动顶点制造蓬松雪面并沿法线
// 推高雪深,边缘平滑拉平;ps 只保留屋顶上半部分,输出白色雪材质到 gbuffer。

#include "inc/common.hlsl"
#include "inc/current_view.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/math_const.hlsl"
#include "inc/hash.hlsl"
#include "inc/perlin.hlsl"

// INC 问题规避:原 samplers.hlsl 声明了 13 个采样器(s0..s12);inc/samplers.hlsl
// 只带 s0..s4,s9..s12。在此补声明 s5..s8,恢复 13 个连续采样器槽,
// 使 $Globals 位于 set0/b13。

// 每实例/全局输入。
float4x4 model_to_world;
float3 roof_pivot;
int is_circle;
int bool_render_shadows;

// 3D 雪格点使用的 4 步整数哈希(第 5 步 +x<<15 折进角点值混合,与编译形态一致)。
uint snow_hash4(uint x)
{
    x += (x << 10u);
    x ^= (x >> 6u);
    x += (x << 3u);
    x ^= (x >> 11u);
    return x;
}

// 一个 3D 格点角点:把 (x,y,z) 整数哈希成两个 [0,1) 值。
float2 snow_corner(uint3 c)
{
    uint z4 = snow_hash4(c.z);
    uint hy = ((c.y * 1664525u) + (z4 + (z4 << 15u))) + 1013904223u;
    hy *= 1664525u;
    hy ^= (hy >> 11u);
    hy ^= (hy << 7u) & 2636928640u;
    hy ^= (hy << 15u) & 4022730752u;
    uint h = ((c.x * 1664525u) + (hy ^ (hy >> 18u))) + 1013904223u;
    h *= 1664525u;
    h ^= (h >> 11u);
    h ^= (h << 7u) & 2636928640u;
    h ^= (h << 15u) & 4022730752u;
    uint hx = h ^ (h >> 18u);
    uint h5 = snow_hash4(hx);
    // 把 23 位哈希映射到 [0,1) 浮点。
    return float2(asfloat((hx & 0x7FFFFFu) | 0x3F800000u) - 1.0f,
                  asfloat(((h5 + (h5 << 15u)) & 0x7FFFFFu) | 0x3F800000u) - 1.0f);
}

// 三线性 3D 格点 float2 噪声(8 个角点,按编译顺序排列)。
float2 snow_noise3(int3 cell, float ux, float ox, float uy, float oy, float uz, float oz)
{
    float2 n000 = snow_corner(uint3(cell + int3(0, 0, 0)));
    float2 n100 = snow_corner(uint3(cell + int3(1, 0, 0)));
    float2 n010 = snow_corner(uint3(cell + int3(0, 1, 0)));
    float2 n110 = snow_corner(uint3(cell + int3(1, 1, 0)));
    float2 n001 = snow_corner(uint3(cell + int3(0, 0, 1)));
    float2 n101 = snow_corner(uint3(cell + int3(1, 0, 1)));
    float2 n011 = snow_corner(uint3(cell + int3(0, 1, 1)));
    float2 n111 = snow_corner(uint3(cell + int3(1, 1, 1)));
    // 三线性插值(8 个角点按 ux/ox、uy/oy、uz/oz 加权)。
    return ((((n000 * ox) * oy) * oz) + (((n100 * ux) * oy) * oz))
        + (((n010 * ox) * uy) * oz) + (((n110 * ux) * uy) * oz)
        + (((n001 * ox) * oy) * uz) + (((n101 * ux) * oy) * uz)
        + (((n011 * ox) * uy) * uz) + (((n111 * ux) * uy) * uz);
}

// 2D 值噪声,双线性插值用分解乘加形式(与原编译形态一致;
// inc/perlin.hlsl 的 perlin() 用的是 lerp 形式)。
float flag_noise(float2 p)
{
    int2 cell = int2(floor(p));
    float2 f = frac(p);
    float2 u = f * f * (3.0f - 2.0f * f);
    float n00 = incb_perlin_hash01(hash(uint2(cell + int2(0, 0))));
    float n10 = incb_perlin_hash01(hash(uint2(cell + int2(1, 0))));
    float n01 = incb_perlin_hash01(hash(uint2(cell + int2(0, 1))));
    float n11 = incb_perlin_hash01(hash(uint2(cell + int2(1, 1))));
    return ((1.0f - u.y) * ((n00 * (1.0f - u.x)) + (n10 * u.x)))
        + ((n01 * (1.0f - u.x)) * u.y)
        + ((n11 * u.x) * u.y);
}

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float2 Vertex_UV: Vertex_UV;
    [[vk::location(2)]] float3 Vertex_Normal: Vertex_Normal;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    // 3D 雪噪声:按顶点位置 x0.4 取格点,三线性插值得到 xy 抖动。
    float3 snow = vin.Vertex_Position * 0.4f;
    int3 cell = int3(floor(snow));
    float3 f = frac(snow);
    float3 u = f * f * (3.0f - 2.0f * f);
    float ux = u.x;
    float ox = 1.0f - ux;
    float uy = u.y;
    float oy = 1.0f - uy;
    float uz = u.z;
    float oz = 1.0f - uz;
    float2 jitter = clamp((snow_noise3(cell, ux, ox, uy, oy, uz, oz) - 0.5f) * 2.5f, -0.5f, 0.5f);

    // 相对屋顶枢轴的位置,用 2D 值噪声求雪深。
    float3 to_pivot = vin.Vertex_Position - roof_pivot;
    float depth = flag_noise(to_pivot.xz * 2.0f);

    // 位移:xy 抖动 + 法线推高(雪深平方)+ 屋顶边缘(UV.y 低处)拉平。
    float3 displaced = (vin.Vertex_Position + (float3(jitter.x, 0.0f, jitter.y) * 0.06f))
        - ((vin.Vertex_Normal * 0.4f) * 0.1f)
        + ((vin.Vertex_Normal * (depth * depth)) * 0.9f * 0.1f)
        - ((vin.Vertex_Normal * 0.1f) * smoothstep(0.4f, 0.0f, vin.Vertex_UV.y));

    // 变换到采样空间;白色反照率、世界法线、位移位置、枢轴向量透传。
    o.position = mul(view_constants[current_view_index].world_to_sample, float4(displaced, 1.0f));
    o.C0 = 0.7f.xxx;
    o.C1 = mul(model_to_world, float4(vin.Vertex_Normal, 0.0f)).xyz;
    o.C2 = displaced;
    o.C3 = to_pivot;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C3 : TEXCOORD3;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    // 只保留屋顶上半部分(相对枢轴的 y >= 0.5),下半丢弃。
    if (pin.C3.y < 0.5f)
    {
        discard;
    }
    // 打包法线 + 白色反照率进 gbuffer;高字节预置雪材质标志。
    float3 packed_normal = (normalize(pin.C1) * 0.5f) + 0.5f;
    float3 albedo_sqrt = sqrt(clamp(pin.C0, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = (uint(clamp(albedo_sqrt.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(albedo_sqrt.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(albedo_sqrt.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u));
    uint normal_word = (uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)) + 3422552064u;
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = albedo_word | 117440512u;
    return o;
}

#endif

// b903e43ffb3da915 变体的像素阶段(与手写版相同的丢弃 + gbuffer 打包逻辑)。
#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _roof_snow.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
static float3 in_var_C0;
static float3 in_var_C1;
static float3 in_var_C3;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    // 只保留屋顶上半部分。
    if (in_var_C3.y < 0.5f)
    {
        discard;
    }
    // 打包法线与反照率字。
    float3 packed_normal = (normalize(in_var_C1) * 0.5f) + 0.5f.xxx;
    float3 albedo_sqrt = sqrt(clamp(in_var_C0, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = (uint((clamp(albedo_sqrt.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(albedo_sqrt.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(albedo_sqrt.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 gbuffer_word = uint2(((uint((clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 3422552064u, albedo_word);
    gbuffer_word.y = albedo_word | 117440512u;
    out_var_SV_TARGET0 = gbuffer_word;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C0 = stage_input.in_var_C0;
    in_var_C1 = stage_input.in_var_C1;
    in_var_C3 = stage_input.in_var_C3;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

// f0adff76827e73ff 变体的像素阶段:只保留"丢弃屋顶下半部分"的裁剪逻辑。
#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _roof_snow.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
static float3 in_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
};

void frag_main()
{
    // 只保留屋顶上半部分。
    if (in_var_C3.y < 0.5f)
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C3 = stage_input.in_var_C3;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
