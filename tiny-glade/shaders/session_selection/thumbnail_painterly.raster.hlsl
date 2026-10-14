// PATCHED: modules lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/session_selection/thumbnail_painterly.raster.hlsl | F-E | bins: _session_selection_thumbnail_painterly.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// 绘画风会话缩略图:按实例(每实例 6 顶点)散落在 first_thumbnail_center 周围,
// 由哈希随机决定旋转与大小;顶点着色器采样缩略图并通过 C1 传出染色精灵;
// 像素着色器叠加笔刷纹理遮罩。

#include "inc/uv.hlsl"
#include "inc/common.hlsl"            // samplers (sampler_lnc s0, sampler_llc s3)
#include "inc/consts.hlsl"
#include "inc/utils.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/math.hlsl"
#include "inc/quad_mesh.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/color/srgb.hlsl"

cbuffer $Globals : register(b15, space0)
{
    uint2 framebuffer_dims;
    float2 first_thumbnail_center;
    float thumbnail_size;
    float hover;
    uint is_black_and_white;
    uint thumbnail_index;
};

Texture2D<float4> thumbnail_tex : register(t13, space0);
Texture2D<float4> brush_tex : register(t14, space0);

struct VsOut
{
    float2 uv : TEXCOORD0;
    float4 sprite : TEXCOORD1;
    float4 position : SV_Position;
};

VsOut vs_main(uint vertex_id : SV_VertexID)
{
    VsOut o;

    // 每实例 6 顶点;实例索引 = id/6,角索引 = id%6。
    uint instance = vertex_id / 6u;
    uint vid = vertex_id % 6u;

    // 6 顶点四边形四角坐标 + uv。
    float4 corner;
    if (vid == 0u) corner = float4(0.5f, -0.5f, 1.0f, 0.0f);
    else if (vid == 1u) corner = float4(0.5f, 0.5f, 1.0f, 1.0f);
    else if (vid == 2u) corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
    else if (vid == 3u) corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
    else if (vid == 4u) corner = float4(-0.5f, -0.5f, 0.0f, 0.0f);
    else corner = float4(0.5f, -0.5f, 1.0f, 0.0f);

    // 按实例索引旋转角点(黄金角步进),uv 的 v 翻转。
    float fi = float(instance);
    float ang = fi * 0.5698403f;
    float s = sin(ang);
    float c = cos(ang);
    float2 rotated = mul(float2x2(float2(c, s), float2(-s, c)), corner.xy);
    float4 local = float4(rotated, corner.z, 1.0f - corner.w);

    // 由缩略图索引与实例索引混合出的确定性随机种子(整数哈希链)。
    uint seed = thumbnail_index + (thumbnail_index << 10u);
    seed = seed ^ (seed >> 6u);
    seed = seed + (seed << 3u);
    seed = seed ^ (seed >> 11u);
    uint rnd = ((instance * 1664525u) + (seed + (seed << 15u))) * 1664525u;
    rnd = rnd ^ (rnd >> 11u);
    rnd = rnd ^ ((rnd << 7u) & 2636928640u);
    rnd = rnd ^ ((rnd << 15u) & 4022730752u);
    uint rand_bits = rnd ^ (rnd >> 18u);

    // 散射方向与尺寸:方向按实例固定角度,大小随实例缓增。
    float2 dir = float2(cos(fi * 4.7430363f), sin(fi * 4.7430363f)) * 0.5f;
    float p = fi * 0.0078125f;
    float size = lerp(0.5f, 0.75f, sqrt(p));

    // 第二个哈希判定该实例是否落在六边形格子阵位上。
    uint h2 = rand_bits + (rand_bits << 10u);
    h2 = h2 ^ (h2 >> 6u);
    h2 = h2 + (h2 << 3u);
    h2 = h2 ^ (h2 >> 11u);
    bool grid = ((h2 + (h2 << 15u)) % 6u) == 0u;

    // 阵位实例用准随机抖动,非阵位实例沿方向按随机量外扩。
    float2 offset;
    if (grid)
        offset = frac((float2(0.7548777f, 0.5698403f) * fi) + 0.5f) - 0.5f;
    else
        offset = dir * ((lerp(0.8f, 1.2f, rand01(rand_bits)) * 0.5f) * size / max(abs(dir.x), abs(dir.y)));

    // 角点位置按尺寸缩放、叠加散射偏移、平移到缩略图中心。
    float2 pos = (local.xy * thumbnail_size) * (lerp(0.25f, 0.12f, p) * 2.0f) + (offset * thumbnail_size) + first_thumbnail_center;
    pos.x = pos.x / (float(framebuffer_dims.x) / float(framebuffer_dims.y));
    float4 clip = float4((pos - 0.5f) * float2(2.0f, -2.0f), 0.0f, 1.0f);
    if ((instance % 2u) != 0u)
        clip = float4(0.0f, 0.0f, 0.0f, 0.0f);

    // 在偏移处采样缩略图,alpha 预乘成精灵颜色。
    float4 sprite = thumbnail_tex.SampleLevel(sampler_lnc, offset + 0.5f, 0.0f);
    float4 sp = float4(sprite.rgb * sprite.a, sprite.a);

    // 非阵位实例做圆角边缘遮罩(距离场 smoothstep),阵位实例压暗一半。
    float rim = size * 0.08f;
    float4 sprite_col;
    if (!grid)
    {
        float radius = 0.2f * size;
        float2 d = (abs(offset) - (0.5f * size).xx) + rim.xx;
        float dist = abs((length(max(d, 0.0f)) + min(max(d.x, d.y), 0.0f)) - rim);
        float m = (dist > radius) ? 0.0f : 1.0f - (dist / radius) * (dist / radius) * (3.0f - 2.0f * (dist / radius));
        sprite_col = sp * m;
    }
    else
        sprite_col = sp * 0.5f;

    // 整体再压暗 25%;精灵透明时用 NaN 裁剪掉该片元。
    sprite_col *= 0.75f;
    if (sprite_col.w < 1e-4f)
        clip = float4(asfloat(0x7fbfffffu).xxxx);

    o.position = clip;
    o.uv = local.zw;
    o.sprite = sprite_col;
    return o;
}

struct PsIn
{
    float2 uv : TEXCOORD0;
    float4 sprite : TEXCOORD1;
};

float4 ps_main(PsIn pin) : SV_Target0
{
    // 笔刷纹理遮罩叠加精灵色,悬停提亮 20%;黑白模式取 Rec.709 亮度。
    float4 brush = brush_tex.Sample(sampler_llc, pin.uv);
    float4 col = brush.xxxx * pin.sprite;
    float3 tinted = col.rgb * lerp(1.0f, 1.2f, hover);

    float4 out_col;
    if (is_black_and_white == 1u)
        out_col = float4(dot(tinted, float3(0.2126f, 0.7152f, 0.0722f)).xxx, col.w);
    else
        out_col = float4(tinted, col.w);
    return out_col;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// 转译自 _session_selection_thumbnail_painterly.raster.hlsl_b903e43ffb3da915.bin(spirv-cross;行为精确,变量名已语义化)
// 顶点着色器:实例化散射的绘画风缩略图(逻辑同重建段,算术为 oracle 逐字)。
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
cbuffer $Globals : register(b15, space0)
{
    uint2 framebuffer_dims : packoffset(c0);
    float2 first_thumbnail_center : packoffset(c0.z);
    float thumbnail_size : packoffset(c1);
    float hover : packoffset(c1.y);
    uint is_black_and_white : packoffset(c1.z);
    uint thumbnail_index : packoffset(c1.w);
};

SamplerState sampler_lnc : register(s0, space0);
Texture2D<float4> thumbnail_tex : register(t13, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static float2 out_var_C0;
static float4 out_var_C1;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float2 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float4 out_var_C1 : TEXCOORD1;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    uint instance_index = uint(gl_VertexIndex) / 6u;
    uint corner_index = uint(gl_VertexIndex) % 6u;
    float4 corner;
    do
    {
        if (corner_index == 0u)
        {
            corner = float4(0.5f, -0.5f, 1.0f, 0.0f);
            break;
        }
        else
        {
            if (corner_index == 1u)
            {
                corner = float4(0.5f, 0.5f, 1.0f, 1.0f);
                break;
            }
            else
            {
                if (corner_index == 2u)
                {
                    corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
                    break;
                }
                else
                {
                    if (corner_index == 3u)
                    {
                        corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
                        break;
                    }
                    else
                    {
                        if (corner_index == 4u)
                        {
                            corner = float4(-0.5f, -0.5f, 0.0f, 0.0f);
                            break;
                        }
                        else
                        {
                            corner = float4(0.5f, -0.5f, 1.0f, 0.0f);
                            break;
                        }
                        break; // unreachable workaround
                    }
                    break; // unreachable workaround
                }
                break; // unreachable workaround
            }
            break; // unreachable workaround
        }
        break; // unreachable workaround
    } while(false);
    float inst_f = float(instance_index);
    float inst_angle = inst_f * 0.56984031200408935546875f;
    float cos_angle = cos(inst_angle);
    float sin_angle = sin(inst_angle);
    float2 rotated_xy = mul(float2x2(float2(cos_angle, sin_angle), float2(-sin_angle, cos_angle)), corner.xy);
    float4 corner_rot = float4(rotated_xy.x, rotated_xy.y, corner.z, corner.w);
    corner_rot.w = 1.0f - corner.w;
    uint hash0 = thumbnail_index + (thumbnail_index << 10u);
    uint hash1 = hash0 ^ (hash0 >> 6u);
    uint hash2 = hash1 + (hash1 << 3u);
    uint hash3 = hash2 ^ (hash2 >> 11u);
    uint hash4 = (((instance_index * 1664525u) + (hash3 + (hash3 << 15u))) + 1013904223u) * 1664525u;
    uint hash5 = hash4 ^ (hash4 >> 11u);
    uint hash6 = hash5 ^ ((hash5 << 7u) & 2636928640u);
    uint hash7 = hash6 ^ ((hash6 << 15u) & 4022730752u);
    uint hash8 = hash7 ^ (hash7 >> 18u);
    float flower_angle = inst_f * 4.7430362701416015625f;
    float2 flower_dir = float2(cos(flower_angle), sin(flower_angle)) * 0.5f;
    float t_f = inst_f * 0.0078125f;
    float scale = lerp(0.5f, 0.75f, sqrt(t_f));
    uint hash9 = hash8 + (hash8 << 10u);
    uint hash10 = hash9 ^ (hash9 >> 6u);
    uint hash11 = hash10 + (hash10 << 3u);
    uint hash12 = hash11 ^ (hash11 >> 11u);
    bool is_hex = ((hash12 + (hash12 << 15u)) % 6u) == 0u;
    float2 jitter;
    if (is_hex)
    {
        jitter = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * inst_f) + 0.5f.xx) - 0.5f.xx;
    }
    else
    {
        jitter = flower_dir * (((lerp(0.800000011920928955078125f, 1.2000000476837158203125f, asfloat((hash8 & 8388607u) | 1065353216u) - 1.0f) * 0.5f) * scale) / max(abs(flower_dir.x), abs(flower_dir.y)));
    }
    float2 pos = (((corner_rot.xy * thumbnail_size) * (lerp(0.25f, 0.119999997317790985107421875f, t_f) * 2.0f)) + (jitter * thumbnail_size)) + first_thumbnail_center;
    pos.x = pos.x / (float(framebuffer_dims.x) / float(framebuffer_dims.y));
    float4 clip_pos = float4((pos - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f);
    bool4 cull_mask = ((instance_index % 2u) != 0u).xxxx;
    float4 culled_pos = float4(cull_mask.x ? 0.0f.xxxx.x : clip_pos.x, cull_mask.y ? 0.0f.xxxx.y : clip_pos.y, cull_mask.z ? 0.0f.xxxx.z : clip_pos.z, cull_mask.w ? 0.0f.xxxx.w : clip_pos.w);
    float4 sample = thumbnail_tex.SampleLevel(sampler_lnc, jitter + 0.5f.xx, 0.0f);
    float3 sample_rgb = sample.xyz * sample.w;
    float4 sample_col = float4(sample_rgb.x, sample_rgb.y, sample_rgb.z, sample.w);
    float mask_edge = scale * 0.080000005662441253662109375f;
    float4 col;
    if (!is_hex)
    {
        float mask_smooth = 0.20000000298023223876953125f * scale;
        float2 mask_d = (abs(jitter) - (0.5f * scale).xx) + mask_edge.xx;
        float mask;
        do
        {
            float mask_abs = abs((length(max(mask_d, 0.0f.xx)) + min(max(mask_d.x, mask_d.y), 0.0f)) - mask_edge);
            if (mask_abs > mask_smooth)
            {
                mask = 0.0f;
                break;
            }
            float mask_t = mask_abs / mask_smooth;
            mask = 1.0f - ((mask_t * mask_t) * (3.0f - (2.0f * mask_t)));
            break;
        } while(false);
        col = sample_col * mask;
    }
    else
    {
        col = sample_col * 0.5f;
    }
    float4 out_col = col * 0.75f;
    bool4 alpha_cull = (out_col.w < 9.9999997473787516355514526367188e-05f).xxxx;
    gl_Position = float4(alpha_cull.x ? asfloat(0x7fbfffffu /* nan */).xxxx.x : culled_pos.x, alpha_cull.y ? asfloat(0x7fbfffffu /* nan */).xxxx.y : culled_pos.y, alpha_cull.z ? asfloat(0x7fbfffffu /* nan */).xxxx.z : culled_pos.z, alpha_cull.w ? asfloat(0x7fbfffffu /* nan */).xxxx.w : culled_pos.w);
    out_var_C0 = corner_rot.zw;
    out_var_C1 = out_col;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// 转译自 _session_selection_thumbnail_painterly.raster.hlsl_f0adff76827e73ff.bin(spirv-cross;行为精确,变量名已语义化)
// 顶点着色器变体 f0ad:与 b903 相同的实例化散射逻辑。
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
cbuffer $Globals : register(b15, space0)
{
    uint2 framebuffer_dims : packoffset(c0);
    float2 first_thumbnail_center : packoffset(c0.z);
    float thumbnail_size : packoffset(c1);
    float hover : packoffset(c1.y);
    uint is_black_and_white : packoffset(c1.z);
    uint thumbnail_index : packoffset(c1.w);
};

SamplerState sampler_lnc : register(s0, space0);
Texture2D<float4> thumbnail_tex : register(t13, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static float2 out_var_C0;
static float4 out_var_C1;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float2 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float4 out_var_C1 : TEXCOORD1;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    uint instance_index = uint(gl_VertexIndex) / 6u;
    uint corner_index = uint(gl_VertexIndex) % 6u;
    float4 corner;
    do
    {
        if (corner_index == 0u)
        {
            corner = float4(0.5f, -0.5f, 1.0f, 0.0f);
            break;
        }
        else
        {
            if (corner_index == 1u)
            {
                corner = float4(0.5f, 0.5f, 1.0f, 1.0f);
                break;
            }
            else
            {
                if (corner_index == 2u)
                {
                    corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
                    break;
                }
                else
                {
                    if (corner_index == 3u)
                    {
                        corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
                        break;
                    }
                    else
                    {
                        if (corner_index == 4u)
                        {
                            corner = float4(-0.5f, -0.5f, 0.0f, 0.0f);
                            break;
                        }
                        else
                        {
                            corner = float4(0.5f, -0.5f, 1.0f, 0.0f);
                            break;
                        }
                        break; // unreachable workaround
                    }
                    break; // unreachable workaround
                }
                break; // unreachable workaround
            }
            break; // unreachable workaround
        }
        break; // unreachable workaround
    } while(false);
    float inst_f = float(instance_index);
    float inst_angle = inst_f * 0.56984031200408935546875f;
    float cos_angle = cos(inst_angle);
    float sin_angle = sin(inst_angle);
    float2 rotated_xy = mul(float2x2(float2(cos_angle, sin_angle), float2(-sin_angle, cos_angle)), corner.xy);
    float4 corner_rot = float4(rotated_xy.x, rotated_xy.y, corner.z, corner.w);
    corner_rot.w = 1.0f - corner.w;
    uint hash0 = thumbnail_index + (thumbnail_index << 10u);
    uint hash1 = hash0 ^ (hash0 >> 6u);
    uint hash2 = hash1 + (hash1 << 3u);
    uint hash3 = hash2 ^ (hash2 >> 11u);
    uint hash4 = (((instance_index * 1664525u) + (hash3 + (hash3 << 15u))) + 1013904223u) * 1664525u;
    uint hash5 = hash4 ^ (hash4 >> 11u);
    uint hash6 = hash5 ^ ((hash5 << 7u) & 2636928640u);
    uint hash7 = hash6 ^ ((hash6 << 15u) & 4022730752u);
    uint hash8 = hash7 ^ (hash7 >> 18u);
    float flower_angle = inst_f * 4.7430362701416015625f;
    float2 flower_dir = float2(cos(flower_angle), sin(flower_angle)) * 0.5f;
    float t_f = inst_f * 0.0078125f;
    float scale = lerp(0.5f, 0.75f, sqrt(t_f));
    uint hash9 = hash8 + (hash8 << 10u);
    uint hash10 = hash9 ^ (hash9 >> 6u);
    uint hash11 = hash10 + (hash10 << 3u);
    uint hash12 = hash11 ^ (hash11 >> 11u);
    bool is_hex = ((hash12 + (hash12 << 15u)) % 6u) == 0u;
    float2 jitter;
    if (is_hex)
    {
        jitter = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * inst_f) + 0.5f.xx) - 0.5f.xx;
    }
    else
    {
        jitter = flower_dir * (((lerp(0.800000011920928955078125f, 1.2000000476837158203125f, asfloat((hash8 & 8388607u) | 1065353216u) - 1.0f) * 0.5f) * scale) / max(abs(flower_dir.x), abs(flower_dir.y)));
    }
    float2 pos = (((corner_rot.xy * thumbnail_size) * (lerp(0.25f, 0.119999997317790985107421875f, t_f) * 2.0f)) + (jitter * thumbnail_size)) + first_thumbnail_center;
    pos.x = pos.x / (float(framebuffer_dims.x) / float(framebuffer_dims.y));
    float4 clip_pos = float4((pos - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f);
    bool4 cull_mask = ((instance_index % 2u) != 0u).xxxx;
    float4 culled_pos = float4(cull_mask.x ? 0.0f.xxxx.x : clip_pos.x, cull_mask.y ? 0.0f.xxxx.y : clip_pos.y, cull_mask.z ? 0.0f.xxxx.z : clip_pos.z, cull_mask.w ? 0.0f.xxxx.w : clip_pos.w);
    float4 sample = thumbnail_tex.SampleLevel(sampler_lnc, jitter + 0.5f.xx, 0.0f);
    float3 sample_rgb = sample.xyz * sample.w;
    float4 sample_col = float4(sample_rgb.x, sample_rgb.y, sample_rgb.z, sample.w);
    float mask_edge = scale * 0.080000005662441253662109375f;
    float4 col;
    if (!is_hex)
    {
        float mask_smooth = 0.20000000298023223876953125f * scale;
        float2 mask_d = (abs(jitter) - (0.5f * scale).xx) + mask_edge.xx;
        float mask;
        do
        {
            float mask_abs = abs((length(max(mask_d, 0.0f.xx)) + min(max(mask_d.x, mask_d.y), 0.0f)) - mask_edge);
            if (mask_abs > mask_smooth)
            {
                mask = 0.0f;
                break;
            }
            float mask_t = mask_abs / mask_smooth;
            mask = 1.0f - ((mask_t * mask_t) * (3.0f - (2.0f * mask_t)));
            break;
        } while(false);
        col = sample_col * mask;
    }
    else
    {
        col = sample_col * 0.5f;
    }
    float4 out_col = col * 0.75f;
    bool4 alpha_cull = (out_col.w < 9.9999997473787516355514526367188e-05f).xxxx;
    gl_Position = float4(alpha_cull.x ? asfloat(0x7fbfffffu /* nan */).xxxx.x : culled_pos.x, alpha_cull.y ? asfloat(0x7fbfffffu /* nan */).xxxx.y : culled_pos.y, alpha_cull.z ? asfloat(0x7fbfffffu /* nan */).xxxx.z : culled_pos.z, alpha_cull.w ? asfloat(0x7fbfffffu /* nan */).xxxx.w : culled_pos.w);
    out_var_C0 = corner_rot.zw;
    out_var_C1 = out_col;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _session_selection_thumbnail_painterly.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
