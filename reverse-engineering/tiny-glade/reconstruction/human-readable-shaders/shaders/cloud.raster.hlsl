// PATCHED: modules lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
// === 中文说明 ===
// 云朵渲染:以相机为中心的外壳上的实例化公告板。片元着色器按实例象限选取 4 张云图
// 之一,在 7 个纹素偏移处采样,结合太阳/天空光解析光照,并用昼夜窗口控制亮度。
// 入口: vs_main (顶点) / ps_main (片元)。变体: TGV_b903e43ffb3da915 与
// TGV_f0adff76827e73ff;vs/ps 因 PATCHED 原因分别使用手写段或 lifted 段。
// 绑定: cloud1..cloud4 t14..t17, $Globals b13, frame_constants set1/b0。
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/cloud.raster.hlsl | F-E | bins: _cloud.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Clouds: instanced billboards on a shell around the camera. The ps samples one
// of 4 cloud textures (by instance quarter) at 7 texel offsets, resolves light
// against the sun/sky, and gates with the day/night window.

#include "inc/common.hlsl"            // samplers (sampler_lnc s0)
#include "inc/math.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/simplex_noise.hlsl"
#include "inc/color_library.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint2 framebuffer_dims;
    uint2 cloud1_dims;
    uint2 cloud2_dims;
    uint2 cloud3_dims;
    uint2 cloud4_dims;
};

Texture2D<float4> cloud1 : register(t14, space0);
Texture2D<float4> cloud2 : register(t15, space0);
Texture2D<float4> cloud3 : register(t16, space0);
Texture2D<float4> cloud4 : register(t17, space0);

struct VsOut
{
    float2 uv : TEXCOORD0;
    nointerpolation uint instance : TEXCOORD1;
    float3 world_pos : TEXCOORD2;
    float3 right : TEXCOORD3;
    float3 facing : TEXCOORD4;
    float4 position : SV_Position;
};

VsOut vs_main(uint vertex_id : SV_VertexID, uint instance_id : SV_InstanceID)
{
    VsOut o;

    // 公告板四边形顶点位置:由顶点 ID 展开为 (-0.5,-1)..(0.5,1)。
    float2 quad = (float2(float(vertex_id & 1u), float((vertex_id >> 1u) & 1u)) - 0.5f) * float2(1.0f, 2.0f);

    // 用实例 ID 派生哈希链,生成随机数。
    uint seed = (instance_id * 12u) + 25u;
    uint h1 = hash(seed);
    uint h2 = hash(h1 + (h1 << 10u));
    uint h3 = hash(h2 + (h2 << 10u));
    uint h4 = hash(h3 + (h3 << 10u));

    // 云朵环绕相机的朝向角(带帧号微小的飘动)与大小。
    float fi = float(instance_id);
    float angle = (frac(0.3f + fi * GOLDEN_RATIO) * TWO_PI)
                + ((float(frame_index) * 5e-5f) * lerp(1.0f, 0.07f, rand01(h2)));

    float3 facing = -float3(cos(angle), 0.0f, sin(angle));
    float t = 1.0f - fi * 0.01666667f;
    float3 center = (-facing) * (10.0f * lerp(400.0f, 800.0f, t * t));

    // 按主题选择云层基准高度,并叠加随机垂直偏移。
    float height;
    if (glade_theme_id == 1u || glade_theme_id == 2u) height = -600.0f;
    else if (glade_theme_id == 3u) height = -500.0f;
    else height = -200.0f;
    if (glade_theme_id == 4u) height = -300.0f;
    center.y += height + 2000.0f * (rand01(h1) * rand01(h1));

    // 公告板尺寸:主题 4 较扁;按类型(实例象限)缩放。
    float2 quad_dims = (glade_theme_id == 4u) ? float2(1050.0f, 525.0f) : float2(1400.0f, 700.0f);
    uint type = int(frac(fi * 0.25f) * 4.0f);
    if (type == 1u) quad_dims *= 1.2f;
    else if (type == 2u) quad_dims *= 0.6f;
    else if (type == 3u) quad_dims *= 1.2f;

    // 构造世界空间四边形。
    float3 right = float3(facing.z, 0.0f, -facing.x);
    float3 world = center + (right * (quad_dims.x * quad.x)) + (float3(0.0f, 1.0f, 0.0f) * (quad_dims.y * quad.y));

    // UV:按顶点 ID 映射,一半实例水平翻转并镜像 right(朝向变化)。
    float2 uv = float2(float(vertex_id & 1u), float(1u - ((vertex_id >> 1u) & 1u)));
    if (rand01(h4) > 0.5f)
    {
        uv.x = 1.0f - float(vertex_id & 1u);
        right = -right;
    }

    float4 clip = mul(view_constants[current_view_index].world_to_sample, float4(world, 1.0f));
    // 超过 60 个实例的公告板投到 NaN,使其不渲染(保留顶点数)。
    if (instance_id > 60u)
        clip = float4(asfloat(0x7fbfffffu).xxxx);

    o.position = clip;
    o.uv = uv;
    o.instance = instance_id;
    o.world_pos = world;
    o.right = right;
    o.facing = facing;
    return o;
}

float4 ps_main(VsOut vin) : SV_Target0
{
    // 云图类型 = 实例象限。
    uint type = int(frac(float(vin.instance) * 0.25f) * 4.0f);

    // 云图按 3x3 网格分区,取本象限的 1/3 区域,7 个偏移采样点。
    float x0 = vin.uv.x * (1.0f / 3.0f);
    float y0 = vin.uv.y * (1.0f / 3.0f);

    float s0, s1, s2, s3, s4, s5, s6;
    if (type == 1u)
    {
        s0 = cloud2.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0 + 2.0f / 3.0f)).x;
        s1 = cloud2.Sample(sampler_lnc, float2(x0, y0 + 2.0f / 3.0f)).x;
        s2 = cloud2.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0 + 1.0f / 3.0f)).x;
        s3 = cloud2.Sample(sampler_lnc, float2(x0, y0 + 1.0f / 3.0f)).x;
        s4 = cloud2.Sample(sampler_lnc, float2(x0 + 2.0f / 3.0f, y0)).x;
        s5 = cloud2.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0)).x;
        s6 = cloud2.Sample(sampler_lnc, float2(x0, y0)).x;
    }
    else if (type == 2u)
    {
        s0 = cloud3.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0 + 2.0f / 3.0f)).x;
        s1 = cloud3.Sample(sampler_lnc, float2(x0, y0 + 2.0f / 3.0f)).x;
        s2 = cloud3.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0 + 1.0f / 3.0f)).x;
        s3 = cloud3.Sample(sampler_lnc, float2(x0, y0 + 1.0f / 3.0f)).x;
        s4 = cloud3.Sample(sampler_lnc, float2(x0 + 2.0f / 3.0f, y0)).x;
        s5 = cloud3.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0)).x;
        s6 = cloud3.Sample(sampler_lnc, float2(x0, y0)).x;
    }
    else if (type == 3u)
    {
        s0 = cloud4.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0 + 2.0f / 3.0f)).x;
        s1 = cloud4.Sample(sampler_lnc, float2(x0, y0 + 2.0f / 3.0f)).x;
        s2 = cloud4.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0 + 1.0f / 3.0f)).x;
        s3 = cloud4.Sample(sampler_lnc, float2(x0, y0 + 1.0f / 3.0f)).x;
        s4 = cloud4.Sample(sampler_lnc, float2(x0 + 2.0f / 3.0f, y0)).x;
        s5 = cloud4.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0)).x;
        s6 = cloud4.Sample(sampler_lnc, float2(x0, y0)).x;
    }
    else
    {
        s0 = cloud1.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0 + 2.0f / 3.0f)).x;
        s1 = cloud1.Sample(sampler_lnc, float2(x0, y0 + 2.0f / 3.0f)).x;
        s2 = cloud1.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0 + 1.0f / 3.0f)).x;
        s3 = cloud1.Sample(sampler_lnc, float2(x0, y0 + 1.0f / 3.0f)).x;
        s4 = cloud1.Sample(sampler_lnc, float2(x0 + 2.0f / 3.0f, y0)).x;
        s5 = cloud1.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0)).x;
        s6 = cloud1.Sample(sampler_lnc, float2(x0, y0)).x;
    }

    // 中心密度过低直接丢弃。
    if (s4 < 0.001f)
        discard;

    // 世界空间前向与相机位置。
    float3 forward = mul(view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz;
    float4 cam4 = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 camera_world = cam4.xyz / cam4.w;

    // 昼夜窗口:夜晚云彩变暗(两个 smoothstep 的差决定白天比例)。
    float day_night = 1.0f - ((smoothstep(0.483f, 0.505f, time_of_day) - smoothstep(0.84f, 0.91f, time_of_day)) * 0.75f);
    float3 sun_light = sun_color.rgb * 0.1f * day_night;
    float3 sky = sky_high_color.rgb * 2.0f * day_night;

    // 按法线贴片方向分通道散射:right 两个面、up 两个面、facing 正面,
    // 以及面向相机的一层高光。
    float3 col = sky;
    col += sun_light * s6 * pow(max(dot(forward, vin.right), 0.0f), 2.0f);
    col += sun_light * s5 * pow(abs(min(dot(forward, vin.right), 0.0f)), 2.0f);
    col += sun_light * s3 * pow(max(forward.y, 0.0f), 2.0f);
    col += sun_light * s2 * pow(abs(min(forward.y, 0.0f)), 2.0f);
    col += sun_light * s0 * pow(max(dot(forward, vin.facing), 0.0f), 2.0f);
    col += sun_color.rgb * (smoothstep(0.9f, 1.0f, dot(forward, -normalize((camera_world / camera_world.xxx) - vin.world_pos))) * s1 * 0.5f);

    // 主题 3 下透明度减半、颜色变暗。
    float alpha = (glade_theme_id == 3u) ? s4 * 0.5f : s4;
    float3 shaded = (glade_theme_id == 3u) ? col * 0.6f : col;
    return float4(shaded * alpha, alpha);
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _cloud.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// === 中文说明 ===
// 下面是从 oracle bin 直接提升(lift)的 vs_main 段(行为精确,变量名已语义化,
//   _NN 原编号已在注释里说明)。
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct ViewConstants
{
    column_major float4x4 view_to_clip;
    column_major float4x4 clip_to_view;
    column_major float4x4 view_to_sample;
    column_major float4x4 sample_to_view;
    column_major float4x4 world_to_view;
    column_major float4x4 view_to_world;
    column_major float4x4 world_to_sample;
    column_major float4x4 clip_to_prev_clip;
    column_major float4x4 prev_view_to_prev_clip;
    column_major float4x4 prev_clip_to_prev_view;
    column_major float4x4 prev_world_to_prev_view;
    column_major float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
};

cbuffer frame_constants : register(b0, space1)
{
    float4 frame_constants_sun_color : packoffset(c0);
    float4 frame_constants_sky_low_color : packoffset(c1);
    float4 frame_constants_sky_high_color : packoffset(c2);
    float4 frame_constants_fog_tint_thickness : packoffset(c3);
    float4 frame_constants_fake_light_color : packoffset(c4);
    float4 frame_constants_fake_light_dir : packoffset(c5);
    float4 frame_constants_fake_light2_color : packoffset(c6);
    float4 frame_constants_fake_light2_dir : packoffset(c7);
    uint frame_constants_frame_index : packoffset(c8);
    float frame_constants_delta_time_seconds : packoffset(c8.y);
    float frame_constants_ev_mult : packoffset(c8.z);
    uint frame_constants_glade_theme_id : packoffset(c8.w);
    uint2 frame_constants_inner_rendering_resolution : packoffset(c9);
    uint frame_constants_game_time_sec : packoffset(c9.z);
    float frame_constants_game_time_frac : packoffset(c9.w);
    float3 frame_constants_editing_highlight_color : packoffset(c10);
    float frame_constants_time_of_day : packoffset(c10.w);
    float3 frame_constants_dir_to_sun : packoffset(c11);
    uint frame_constants_camera_idx : packoffset(c11.w);
    float3 frame_constants_dir_to_moon : packoffset(c12);
    uint frame_constants_render_overrides : packoffset(c12.w);
    float frame_constants_global_mip_bias : packoffset(c13);
    uint frame_constants_highlighted_transient_stair_edge_id : packoffset(c13.y);
    uint frame_constants_pad1 : packoffset(c13.z);
    uint frame_constants_pad2 : packoffset(c13.w);
    float4 frame_constants_debug_tweakers : packoffset(c14);
    ViewConstants frame_constants_view_constants[5] : packoffset(c15);
};

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index : packoffset(c0);
    uint2 framebuffer_dims : packoffset(c0.y);
    uint2 cloud1_dims : packoffset(c1);
    uint2 cloud2_dims : packoffset(c1.z);
    uint2 cloud3_dims : packoffset(c2);
    uint2 cloud4_dims : packoffset(c2.z);
};


static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static float2 out_var_C0;
static uint out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static float3 out_var_C4;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float2 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] nointerpolation uint out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    // 顶点 ID 拆分出公告板象限坐标 (quad_x, quad_y)。
    float quad_x = float(uint(gl_VertexIndex) & 1u);
    uint quad_y = (uint(gl_VertexIndex) >> 1u) & 1u;
    // 公告板位置:(-0.5..0.5, -1..1)。
    float2 quad_pos = (float2(quad_x, float(quad_y)) - 0.5f.xx) * float2(1.0f, 2.0f);
    // 实例种子与 Wang 哈希链:得到 h1 (seed)、h2 (rand)、h3、h4。
    uint seed = (uint(gl_InstanceIndex) * 12u) + 25u;
    uint hash_a = seed + (seed << 10u);
    uint hash_b = hash_a ^ (hash_a >> 6u);
    uint hash_c = hash_b + (hash_b << 3u);
    uint hash_d = hash_c ^ (hash_c >> 11u);
    uint h1 = hash_d + (hash_d << 15u);
    float instance_f = float(uint(gl_InstanceIndex));
    uint hash_e = h1 + (h1 << 10u);
    uint hash_f = hash_e ^ (hash_e >> 6u);
    uint hash_g = hash_f + (hash_f << 3u);
    uint hash_h = hash_g ^ (hash_g >> 11u);
    uint h2 = hash_h + (hash_h << 15u);
    // rand01(h1):取 h1 低 23 位凑成 [0,1) 浮点数。
    float rand_h1 = asfloat((h1 & 8388607u) | 1065353216u) - 1.0f;
    uint hash_i = h2 + (h2 << 10u);
    uint hash_j = hash_i ^ (hash_i >> 6u);
    uint hash_k = hash_j + (hash_j << 3u);
    uint hash_l = hash_k ^ (hash_k >> 11u);
    uint h3 = hash_l + (hash_l << 15u);
    uint hash_m = h3 + (h3 << 10u);
    uint hash_n = hash_m ^ (hash_m >> 6u);
    uint hash_o = hash_n + (hash_n << 3u);
    uint hash_p = hash_o ^ (hash_o >> 11u);
    // h4 的中间值;完整 h4 = h4_partial + (h4_partial << 15u),在翻转测试里内联完成最后一步。
    uint h4_partial = hash_p;
    // 云朵朝向角:黄金比例分布 + 帧号微小飘动(幅度由 rand(h2) 控制)。
    float angle = (frac(0.300000011920928955078125f + (instance_f * 1.61803400516510009765625f)) * 6.283185482025146484375f) + ((float(frame_constants_frame_index) * 4.9999998736893758177757263183594e-05f) * lerp(1.0f, 0.070000000298023223876953125f, asfloat((h2 & 8388607u) | 1065353216u) - 1.0f));
    // facing_neg = -(cos, sin),拆出分量。
    float2 facing_neg = -float2(cos(angle), sin(angle));
    float facing_z = facing_neg.y;
    float facing_x = facing_neg.x;
    float right_z = -facing_x;
    float2 right = float2(facing_z, right_z);
    float3 facing = float3(facing_x, 0.0f, facing_z);
    // 云朵距相机距离的衰减系数 t。
    float t = 1.0f - (instance_f * 0.01666666753590106964111328125f);
    // 公告板中心:沿朝向的反方向,半径 10 * lerp(400, 800, t^2)。
    float3 center = (-facing) * (10.0f * lerp(400.0f, 800.0f, t * t));
    // 主题 1/2 高度 -600;主题 3 高度 -500;其他 -200。
    bool is_theme_1_or_2;
    if (!(frame_constants_glade_theme_id == 2u))
    {
        is_theme_1_or_2 = frame_constants_glade_theme_id == 1u;
    }
    else
    {
        is_theme_1_or_2 = true;
    }
    float base_height;
    if (is_theme_1_or_2)
    {
        base_height = -600.0f;
    }
    else
    {
        base_height = (frame_constants_glade_theme_id == 3u) ? (-500.0f) : (-200.0f);
    }
    bool is_theme_4 = frame_constants_glade_theme_id == 4u;
    center.y = center.y + ((is_theme_4 ? (-300.0f) : base_height) + (2000.0f * (rand_h1 * rand_h1)));
    // 公告板尺寸:主题 4 为 1050x525,否则 1400x700。
    bool2 theme4_mask = is_theme_4.xx;
    float2 quad_dims = float2(theme4_mask.x ? float2(1050.0f, 525.0f).x : float2(1400.0f, 700.0f).x, theme4_mask.y ? float2(1050.0f, 525.0f).y : float2(1400.0f, 700.0f).y);
    // 云图类型:实例序号的四象限。
    int type = int(frac(instance_f * 0.25f) * 4.0f);
    float2 scaled_quad_dims;
    if (type == 1)
    {
        scaled_quad_dims = quad_dims * 1.2000000476837158203125f;
    }
    else
    {
        float2 _quad_dims;
        if (type == 2)
        {
            _quad_dims = quad_dims * 0.60000002384185791015625f;
        }
        else
        {
            float2 _quad_dims2;
            if (type == 3)
            {
                _quad_dims2 = quad_dims * 1.2000000476837158203125f;
            }
            else
            {
                _quad_dims2 = quad_dims;
            }
            _quad_dims = _quad_dims2;
        }
        scaled_quad_dims = _quad_dims;
    }
    // 世界坐标四边形。
    float3 world = center + ((float3(facing_z, 0.0f, right_z) * (scaled_quad_dims.x * quad_pos.x)) + (float3(0.0f, 1.0f, 0.0f) * (scaled_quad_dims.y * quad_pos.y)));
    float2 uv = float2(quad_x, float(1u - quad_y));
    float2 flip_right;
    float2 flip_uv;
    // 约一半实例水平翻转 UV 与朝向。
    if ((asfloat(((h4_partial + (h4_partial << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.5f)
    {
        float2 _uv = uv;
        _uv.x = 1.0f - quad_x;
        flip_right = -right;
        flip_uv = _uv;
    }
    else
    {
        flip_right = right;
        flip_uv = uv;
    }
    // 投影到当前视图裁剪空间。
    float4 clip = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(world, 1.0f));
    // 实例号 > 60 的云朵位置置为 NaN(不渲染)。
    bool4 far_instance = (uint(gl_InstanceIndex) > 60u).xxxx;
    gl_Position = float4(far_instance.x ? asfloat(0x7fbfffffu /* nan */).xxxx.x : clip.x, far_instance.y ? asfloat(0x7fbfffffu /* nan */).xxxx.y : clip.y, far_instance.z ? asfloat(0x7fbfffffu /* nan */).xxxx.z : clip.z, far_instance.w ? asfloat(0x7fbfffffu /* nan */).xxxx.w : clip.w);
    out_var_C0 = flip_uv;
    out_var_C1 = uint(gl_InstanceIndex);
    out_var_C2 = world;
    out_var_C3 = float3(flip_right.x, 0.0f, flip_right.y);
    out_var_C4 = facing;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _cloud.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// === 中文说明 ===
// 与 b903 变体的 vs 段内容一致(变量名已语义化),仅变体守卫不同。
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct ViewConstants
{
    column_major float4x4 view_to_clip;
    column_major float4x4 clip_to_view;
    column_major float4x4 view_to_sample;
    column_major float4x4 sample_to_view;
    column_major float4x4 world_to_view;
    column_major float4x4 view_to_world;
    column_major float4x4 world_to_sample;
    column_major float4x4 clip_to_prev_clip;
    column_major float4x4 prev_view_to_prev_clip;
    column_major float4x4 prev_clip_to_prev_view;
    column_major float4x4 prev_world_to_prev_view;
    column_major float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
};

cbuffer frame_constants : register(b0, space1)
{
    float4 frame_constants_sun_color : packoffset(c0);
    float4 frame_constants_sky_low_color : packoffset(c1);
    float4 frame_constants_sky_high_color : packoffset(c2);
    float4 frame_constants_fog_tint_thickness : packoffset(c3);
    float4 frame_constants_fake_light_color : packoffset(c4);
    float4 frame_constants_fake_light_dir : packoffset(c5);
    float4 frame_constants_fake_light2_color : packoffset(c6);
    float4 frame_constants_fake_light2_dir : packoffset(c7);
    uint frame_constants_frame_index : packoffset(c8);
    float frame_constants_delta_time_seconds : packoffset(c8.y);
    float frame_constants_ev_mult : packoffset(c8.z);
    uint frame_constants_glade_theme_id : packoffset(c8.w);
    uint2 frame_constants_inner_rendering_resolution : packoffset(c9);
    uint frame_constants_game_time_sec : packoffset(c9.z);
    float frame_constants_game_time_frac : packoffset(c9.w);
    float3 frame_constants_editing_highlight_color : packoffset(c10);
    float frame_constants_time_of_day : packoffset(c10.w);
    float3 frame_constants_dir_to_sun : packoffset(c11);
    uint frame_constants_camera_idx : packoffset(c11.w);
    float3 frame_constants_dir_to_moon : packoffset(c12);
    uint frame_constants_render_overrides : packoffset(c12.w);
    float frame_constants_global_mip_bias : packoffset(c13);
    uint frame_constants_highlighted_transient_stair_edge_id : packoffset(c13.y);
    uint frame_constants_pad1 : packoffset(c13.z);
    uint frame_constants_pad2 : packoffset(c13.w);
    float4 frame_constants_debug_tweakers : packoffset(c14);
    ViewConstants frame_constants_view_constants[5] : packoffset(c15);
};

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index : packoffset(c0);
    uint2 framebuffer_dims : packoffset(c0.y);
    uint2 cloud1_dims : packoffset(c1);
    uint2 cloud2_dims : packoffset(c1.z);
    uint2 cloud3_dims : packoffset(c2);
    uint2 cloud4_dims : packoffset(c2.z);
};


static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static float2 out_var_C0;
static uint out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static float3 out_var_C4;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float2 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] nointerpolation uint out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    // 顶点 ID 拆分出公告板象限坐标 (quad_x, quad_y)。
    float quad_x = float(uint(gl_VertexIndex) & 1u);
    uint quad_y = (uint(gl_VertexIndex) >> 1u) & 1u;
    // 公告板位置:(-0.5..0.5, -1..1)。
    float2 quad_pos = (float2(quad_x, float(quad_y)) - 0.5f.xx) * float2(1.0f, 2.0f);
    // 实例种子与 Wang 哈希链:得到 h1 (seed)、h2 (rand)、h3、h4。
    uint seed = (uint(gl_InstanceIndex) * 12u) + 25u;
    uint hash_a = seed + (seed << 10u);
    uint hash_b = hash_a ^ (hash_a >> 6u);
    uint hash_c = hash_b + (hash_b << 3u);
    uint hash_d = hash_c ^ (hash_c >> 11u);
    uint h1 = hash_d + (hash_d << 15u);
    float instance_f = float(uint(gl_InstanceIndex));
    uint hash_e = h1 + (h1 << 10u);
    uint hash_f = hash_e ^ (hash_e >> 6u);
    uint hash_g = hash_f + (hash_f << 3u);
    uint hash_h = hash_g ^ (hash_g >> 11u);
    uint h2 = hash_h + (hash_h << 15u);
    // rand01(h1):取 h1 低 23 位凑成 [0,1) 浮点数。
    float rand_h1 = asfloat((h1 & 8388607u) | 1065353216u) - 1.0f;
    uint hash_i = h2 + (h2 << 10u);
    uint hash_j = hash_i ^ (hash_i >> 6u);
    uint hash_k = hash_j + (hash_j << 3u);
    uint hash_l = hash_k ^ (hash_k >> 11u);
    uint h3 = hash_l + (hash_l << 15u);
    uint hash_m = h3 + (h3 << 10u);
    uint hash_n = hash_m ^ (hash_m >> 6u);
    uint hash_o = hash_n + (hash_n << 3u);
    uint hash_p = hash_o ^ (hash_o >> 11u);
    // h4 的中间值;完整 h4 = h4_partial + (h4_partial << 15u),在翻转测试里内联完成最后一步。
    uint h4_partial = hash_p;
    // 云朵朝向角:黄金比例分布 + 帧号微小飘动(幅度由 rand(h2) 控制)。
    float angle = (frac(0.300000011920928955078125f + (instance_f * 1.61803400516510009765625f)) * 6.283185482025146484375f) + ((float(frame_constants_frame_index) * 4.9999998736893758177757263183594e-05f) * lerp(1.0f, 0.070000000298023223876953125f, asfloat((h2 & 8388607u) | 1065353216u) - 1.0f));
    // facing_neg = -(cos, sin),拆出分量。
    float2 facing_neg = -float2(cos(angle), sin(angle));
    float facing_z = facing_neg.y;
    float facing_x = facing_neg.x;
    float right_z = -facing_x;
    float2 right = float2(facing_z, right_z);
    float3 facing = float3(facing_x, 0.0f, facing_z);
    // 云朵距相机距离的衰减系数 t。
    float t = 1.0f - (instance_f * 0.01666666753590106964111328125f);
    // 公告板中心:沿朝向的反方向,半径 10 * lerp(400, 800, t^2)。
    float3 center = (-facing) * (10.0f * lerp(400.0f, 800.0f, t * t));
    // 主题 1/2 高度 -600;主题 3 高度 -500;其他 -200。
    bool is_theme_1_or_2;
    if (!(frame_constants_glade_theme_id == 2u))
    {
        is_theme_1_or_2 = frame_constants_glade_theme_id == 1u;
    }
    else
    {
        is_theme_1_or_2 = true;
    }
    float base_height;
    if (is_theme_1_or_2)
    {
        base_height = -600.0f;
    }
    else
    {
        base_height = (frame_constants_glade_theme_id == 3u) ? (-500.0f) : (-200.0f);
    }
    bool is_theme_4 = frame_constants_glade_theme_id == 4u;
    center.y = center.y + ((is_theme_4 ? (-300.0f) : base_height) + (2000.0f * (rand_h1 * rand_h1)));
    // 公告板尺寸:主题 4 为 1050x525,否则 1400x700。
    bool2 theme4_mask = is_theme_4.xx;
    float2 quad_dims = float2(theme4_mask.x ? float2(1050.0f, 525.0f).x : float2(1400.0f, 700.0f).x, theme4_mask.y ? float2(1050.0f, 525.0f).y : float2(1400.0f, 700.0f).y);
    // 云图类型:实例序号的四象限。
    int type = int(frac(instance_f * 0.25f) * 4.0f);
    float2 scaled_quad_dims;
    if (type == 1)
    {
        scaled_quad_dims = quad_dims * 1.2000000476837158203125f;
    }
    else
    {
        float2 _quad_dims;
        if (type == 2)
        {
            _quad_dims = quad_dims * 0.60000002384185791015625f;
        }
        else
        {
            float2 _quad_dims2;
            if (type == 3)
            {
                _quad_dims2 = quad_dims * 1.2000000476837158203125f;
            }
            else
            {
                _quad_dims2 = quad_dims;
            }
            _quad_dims = _quad_dims2;
        }
        scaled_quad_dims = _quad_dims;
    }
    // 世界坐标四边形。
    float3 world = center + ((float3(facing_z, 0.0f, right_z) * (scaled_quad_dims.x * quad_pos.x)) + (float3(0.0f, 1.0f, 0.0f) * (scaled_quad_dims.y * quad_pos.y)));
    float2 uv = float2(quad_x, float(1u - quad_y));
    float2 flip_right;
    float2 flip_uv;
    // 约一半实例水平翻转 UV 与朝向。
    if ((asfloat(((h4_partial + (h4_partial << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.5f)
    {
        float2 _uv = uv;
        _uv.x = 1.0f - quad_x;
        flip_right = -right;
        flip_uv = _uv;
    }
    else
    {
        flip_right = right;
        flip_uv = uv;
    }
    // 投影到当前视图裁剪空间。
    float4 clip = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(world, 1.0f));
    // 实例号 > 60 的云朵位置置为 NaN(不渲染)。
    bool4 far_instance = (uint(gl_InstanceIndex) > 60u).xxxx;
    gl_Position = float4(far_instance.x ? asfloat(0x7fbfffffu /* nan */).xxxx.x : clip.x, far_instance.y ? asfloat(0x7fbfffffu /* nan */).xxxx.y : clip.y, far_instance.z ? asfloat(0x7fbfffffu /* nan */).xxxx.z : clip.z, far_instance.w ? asfloat(0x7fbfffffu /* nan */).xxxx.w : clip.w);
    out_var_C0 = flip_uv;
    out_var_C1 = uint(gl_InstanceIndex);
    out_var_C2 = world;
    out_var_C3 = float3(flip_right.x, 0.0f, flip_right.y);
    out_var_C4 = facing;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _cloud.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// === 中文说明 ===
// f0ad 变体的片元段:只做云图中心密度采样与 discard(该变体的 ps 在目标 bin 中
//   比手写 ps 更精简,仅包含 type 选择 + 一个采样 + 低密度丢弃)。
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
SamplerState sampler_lnc : register(s0, space0);
Texture2D<float4> cloud1 : register(t14, space0);
Texture2D<float4> cloud2 : register(t15, space0);
Texture2D<float4> cloud3 : register(t16, space0);
Texture2D<float4> cloud4 : register(t17, space0);

static float2 in_var_C0;
static uint in_var_C1;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float2 in_var_C0 : TEXCOORD0;
    [[vk::location(1)]] nointerpolation uint in_var_C1 : TEXCOORD1;
};

void frag_main()
{
    // 云图类型 = 实例象限。
    int type = int(frac(float(in_var_C1) * 0.25f) * 4.0f);
    // 按类型选图并采样中心点 (s0)。
    float s0;
    if (type == 1)
    {
        s0 = cloud2.Sample(sampler_lnc, float2((in_var_C0.x * 0.3333333432674407958984375f) + 0.666666686534881591796875f, in_var_C0.y * 0.3333333432674407958984375f)).x;
    }
    else
    {
        float _s0;
        if (type == 2)
        {
            _s0 = cloud3.Sample(sampler_lnc, float2((in_var_C0.x * 0.3333333432674407958984375f) + 0.666666686534881591796875f, in_var_C0.y * 0.3333333432674407958984375f)).x;
        }
        else
        {
            float _s0b;
            if (type == 3)
            {
                _s0b = cloud4.Sample(sampler_lnc, float2((in_var_C0.x * 0.3333333432674407958984375f) + 0.666666686534881591796875f, in_var_C0.y * 0.3333333432674407958984375f)).x;
            }
            else
            {
                _s0b = cloud1.Sample(sampler_lnc, float2((in_var_C0.x * 0.3333333432674407958984375f) + 0.666666686534881591796875f, in_var_C0.y * 0.3333333432674407958984375f)).x;
            }
            _s0 = _s0b;
        }
        s0 = _s0;
    }
    // 密度过低丢弃。
    if (s0 < 0.001000000047497451305389404296875f)
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C0 = stage_input.in_var_C0;
    in_var_C1 = stage_input.in_var_C1;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
