// ==RECONSTRUCTED== shaders/snow_particles.raster.hlsl | F-V | bins: _snow_particles.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// 雪花粒子:以活动相机为中心的 100 单位周期体积内、面向相机的四边形。
// vs_main 在顶视图模式、粒子过近/超出景深范围、或属性未定义(NaN 位置)时
// 剔除顶点(把位置写 NaN);ps 用蓝噪哈希抖动,然后打包进实心 gbuffer。

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/current_view.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/coc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"

// INC 问题规避:原版 common.hlsl 声明了 13 个采样器(s0..s12);
// inc/samplers.hlsl 只带 s0..s4,s9..s12。在此声明未使用的 s5..s8,
// 以复现原版 $Globals 绑定(set0/b13)。

// 每相机自动对焦数据(DofFocus 记录,64 字节步长,来自 inc/coc.hlsl)。
StructuredBuffer<DofFocus> autofocus_buf : register(t14);

// 与目标内联访问链结构一致的访问辅助。
DofFocus get_autofocus(uint index)
{
    return autofocus_buf[index];
}

ViewConstants get_camera0()
{
    return view_constants[0];
}

// 全局常量($Globals)。
float4x4 model_to_world;
float snow_strength;     // 雪花强度,控制粒子尺寸
int is_photomode;        // 照片模式开关
int is_topdown_mode;     // 顶视图模式开关

// 相机距离剔除阈值:正常 15 米,照片模式下为 0(永不剔除)。
float snow_camera_cutoff()
{
    if (is_photomode != 0)
    {
        return 0.0f;
    }
    else
    {
        return 15.0f;
    }
}

struct VsIn
{
    uint vertex_id : SV_VertexID;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float2 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    nointerpolation uint C2 : TEXCOORD2;
    float4 position : SV_Position;
};

// 顶点阶段:生成面向相机的雪花四边形。
VsOut vs_main(VsIn vin)
{
    VsOut o;
    // 顶视图模式:整个粒子集合都剔除。
    if (is_topdown_mode == 1)
    {
        o.position = asfloat(0x7FBFFFFFu).xxxx;
        return o;
    }

    // 四边形内角点 UV(以中心为原点)。
    float2 quad_uv = float2(float(vin.vertex_id & 1u), float((vin.vertex_id >> 1u) & 1u)) - 0.5f;
    int id = int(vin.instance_id);

    // 每实例随机下落速度(0.3..1.2)与 100 单位体积内的随机偏移。
    float fall = lerp(0.3f, 1.2f, rand01(hash(uint(id ^ 5678))));
    float3 offset = float3(rand01(hash(uint(id ^ 23475))), rand01(hash(uint(id ^ 35934))), rand01(hash(uint(id ^ 45845)))) * 100.0f;
    // 飘落:随游戏时间向下、向左漂移。
    float drift = ((float(game_time_sec) + game_time_frac) * 2.0f) * fall;
    offset.y -= drift;
    offset.x -= drift;

    // 相机 0 的世界位置与前向,用于景深/移轴计算。
    float4 cam0_vw = mul(get_camera0().view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam0_pos = cam0_vw.xyz / cam0_vw.w.xxx;
    float3 cam0_fwd = normalize(mul(get_camera0().view_to_world, float4(0.0f, 0.0f, -1.0f, 0.0f)).xyz);

    // 景深远界:由对焦尺度与视锥垂直半角推出。
    float dof_scale = 5.0f / (get_autofocus(camera_idx).focus_scale * get_camera0().view_to_clip[1].y);
    bool tilt = get_autofocus(camera_idx).use_tilt_shift != 0u;
    float dof_far = dof_scale;
    if (tilt)
    {
        dof_far = dof_scale * max(0.01f, length(cam0_fwd.xz));
    }

    // 焦点范围:由 focus0/focus1 的倒数推近/远平面。
    float inv_f0 = 1.0f / get_autofocus(camera_idx).focus0;
    float inv_f1 = 1.0f / get_autofocus(camera_idx).focus1;
    float focus_half = max(0.001f, max(inv_f0, inv_f1)) + 9.9999997473787516e-06f;
    float focus_near = max(0.001f, min(inv_f0, inv_f1)) - 9.9999997473787516e-06f;
    // 对焦距离。
    float dof_focus = 1.0f / ((dof_far + focus_half) - min(0.01f, 0.05f * (focus_half - focus_near)));
    float focus_dist = dof_focus;
    if (tilt)
    {
        // 移轴模式下按相机俯仰把焦点投影到水平面。
        focus_dist = (dof_focus + (cam0_pos.y * cam0_fwd.y)) / ((cam0_fwd.x * cam0_fwd.x) + (cam0_fwd.z * cam0_fwd.z));
    }

    // 当前相机朝向的 100 单位体积:粒子在其中周期包裹。
    float4 cur_vw = mul(GetCurrentViewConstants().view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_pos = cur_vw.xyz / cur_vw.w.xxx;
    float3 cam_fwd = normalize(mul(GetCurrentViewConstants().view_to_world, float4(0.0f, 0.0f, -1.0f, 0.0f)).xyz);
    // 体积框:焦点前方 25 米处居中,边长 100。
    float3 box_min = cam_pos + (cam_fwd * (focus_dist + 25.0f)) - 50.0f;
    float3 box_size = (cam_pos + (cam_fwd * (focus_dist + 25.0f)) + 50.0f) - box_min;
    // 把随机偏移周期折叠进体积框内。
    float3 world = frac((offset - box_min) / box_size) * box_size + box_min;

    // 距离相机过近(小于剔除阈值)的粒子剔除。
    if (distance(world, cam_pos) < snow_camera_cutoff())
    {
        o.position = asfloat(0x7FBFFFFFu).xxxx;
        return o;
    }

    // 计算该粒子的散焦圆(CoC)以做景深剔除。
    inv_f0 = 1.0f / get_autofocus(camera_idx).focus0;
    inv_f1 = 1.0f / get_autofocus(camera_idx).focus1;
    float near_plane = max(0.001f, max(inv_f0, inv_f1)) + 9.9999997473787516e-06f;
    float far_plane = max(0.001f, min(inv_f0, inv_f1)) - 9.9999997473787516e-06f;
    tilt = get_autofocus(camera_idx).use_tilt_shift != 0u;
    // 世界位置沿相机前向的深度(移轴时忽略 y)。
    float3 depth_pos = world;
    depth_pos.y = world.y * float(tilt ? 0 : 1);
    float depth = 1.0f / max(1.00000001335143196e-10f, dot(depth_pos - cam0_pos, cam0_fwd));
    float plane_range = near_plane - far_plane;
    // 平滑过渡带宽度。
    float edge_w = min(0.01f, 0.05f * plane_range);
    // CoC:近侧/远侧/带内三段插值。
    float coc;
    if (depth > near_plane)
    {
        coc = (depth - near_plane) + edge_w;
    }
    else if (depth < far_plane)
    {
        coc = (depth - far_plane) - edge_w;
    }
    else
    {
        coc = (-edge_w) + ((2.0f * edge_w * (depth - far_plane)) / plane_range);
    }
    // 移轴时把 CoC 摊回水平方向。
    float coc_world = coc;
    if (tilt)
    {
        coc_world = coc / max(0.01f, length(cam0_fwd.xz));
    }

    // 散焦圆半径超过 20 像素的粒子剔除(景深外模糊太大,直接省略)。
    if (abs(((-coc_world) * get_autofocus(camera_idx).focus_scale) * get_camera0().view_to_clip[1].y) > 20.0f)
    {
        o.position = asfloat(0x7FBFFFFFu).xxxx;
        return o;
    }

    // 粒子尺寸 = 0.16 * 雪花强度;用当前相机 right/up 轴展开四边形。
    float scale = 0.16f * snow_strength;
    float3 right = normalize(mul(GetCurrentViewConstants().view_to_world, float4(1.0f, 0.0f, 0.0f, 0.0f)).xyz);
    float3 up = normalize(mul(GetCurrentViewConstants().view_to_world, float4(0.0f, 1.0f, 0.0f, 0.0f)).xyz);
    float3 quad = (up * quad_uv.y * scale) + (right * quad_uv.x * scale) + world;

    // 世界 -> 采样空间,透传 UV、世界位置与实例 id。
    o.position = mul(GetCurrentViewConstants().world_to_sample, float4(quad, 1.0f));
    o.C0 = quad_uv;
    o.C1 = quad;
    o.C2 = vin.instance_id;
    return o;
}

struct PsIn
{
    float2 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float4 position : SV_Position;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

// 片段阶段:蓝噪抖动 + 打包 gbuffer。
PsOut ps_main(PsIn pin)
{
    PsOut o;
    // 重新计算景深近/远平面与 CoC(与 vs 一致)。
    float3 world_pos = pin.C1;
    float inv_f0 = 1.0f / autofocus_buf[camera_idx].focus0;
    float inv_f1 = 1.0f / autofocus_buf[camera_idx].focus1;
    float near_plane = max(0.001f, max(inv_f0, inv_f1)) + 9.9999997473787516e-06f;
    float far_plane = max(0.001f, min(inv_f0, inv_f1)) - 9.9999997473787516e-06f;
    bool tilt = autofocus_buf[camera_idx].use_tilt_shift != 0u;
    float3 depth_pos = world_pos;
    depth_pos.y = world_pos.y * float(tilt ? 0 : 1);
    float4 cam0_vw = mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam0_pos = cam0_vw.xyz / cam0_vw.w.xxx;
    float3 cam0_fwd = normalize(mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, -1.0f, 0.0f)).xyz);
    float depth = 1.0f / max(1.00000001335143196e-10f, dot(depth_pos - cam0_pos, cam0_fwd));
    float plane_range = near_plane - far_plane;
    float edge_w = min(0.01f, 0.05f * plane_range);
    float coc;
    if (depth > near_plane)
    {
        coc = (depth - near_plane) + edge_w;
    }
    else if (depth < far_plane)
    {
        coc = (depth - far_plane) - edge_w;
    }
    else
    {
        coc = (-edge_w) + ((2.0f * edge_w * (depth - far_plane)) / plane_range);
    }
    float coc_world = coc;
    if (tilt)
    {
        coc_world = coc / max(0.01f, length(cam0_fwd.xz));
    }

    // 当前相机位置,用于距离衰减。
    float4 cur_cam_vw = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cur_cam_pos = cur_cam_vw.xyz / cur_cam_vw.w.xxx;

    float cutoff = snow_camera_cutoff();
    // 屏幕空间蓝噪抖动阈值(黄金角序列哈希)。
    uint2 pixel = uint2(pin.position.xy);
    float dither = frac(52.98291778564453125f * frac((0.0671105608344078f * float(pixel.x)) + (0.0058371499180793762f * float(pixel.y))));
    // 联合阈值:CoC 越大越可能剔除,离相机越远越可能剔除。
    if (smoothstep(0.75f, 1.0f, max(abs(((-coc_world) * autofocus_buf[camera_idx].focus_scale) * view_constants[0].view_to_clip[1].y) * 0.05f, 2.0f - (distance(world_pos, cur_cam_pos) / cutoff))) > dither)
    {
        discard;
    }
    // 四边形外区域剔除。
    if (length(pin.C0) > 0.5f)
    {
        discard;
    }

    // 固定法线(指向相机 2 的 +z 方向),压缩到 [0,1] 并打包。
    float3 snow_normal = (mul(view_constants[2].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz * 0.5f) + 0.5f;
    uint normal_word = uint(clamp(snow_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(snow_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(snow_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 4278190080u;
    // gbuffer:法线字 + 预置的反照率/材质字(雪:白,alpha 标志位)。
    o.gbuffer = uint2(normal_word, 14013909u);
    o.gbuffer.y = 1188419029u;
    return o;
}
