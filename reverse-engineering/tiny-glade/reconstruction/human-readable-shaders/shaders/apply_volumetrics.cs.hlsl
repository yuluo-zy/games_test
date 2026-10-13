// ==RECONSTRUCTED== shaders/apply_volumetrics.cs.hlsl
// Evidence: _apply_volumetrics.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: sampler_shadow_linear s12, $Globals b13 (current_view_index/depth_tex_dims/
//   global_density_scale/global_size_scale/blue_noise_texture_dims/color_tex_dims),
//   shadow_map t14 (2DArray), depth_tex t16, blue_noise_texture t17,
//   color_tex u18 (float3), frame_constants set1/b0.
//   LocalSize [8,8,1], entry cs_main.
//   spirv-cross + disassembly: screen-space volumetric scattering. Reconstructs the ray
//   from the camera through the depth buffer, integrates 6 exponentially-spaced samples
//   along the ray, applies a shadow-map comparison term, two quadratic fake-light
//   occlusion checks and a theme-selected radial density falloff, and accumulates
//   sun/sky/fake-light inscatter against an exponentially decaying transmittance.
// Status: O (structure from disassembly; fake-light occlusion helpers kept as early-return
//   functions to reproduce the switch-shaped exits).
// NOTE: embedded set lists inc/current_view.hlsl (declares the `current_view_index` global)
// and inc/blue_noise.hlsl (declares `blue_noise_texture` without a register); both collide
// with the explicit $Globals member / t17 registration below, so they are omitted and
// their bodies inlined.
//
// === 中文说明 ===
// 屏幕空间体积散射(雾气)计算。入口 cs_main,工作组 8x8x1,无 TGV_ 变体。
// 绑定: shadow_map t14 (Texture2DArray), depth_tex t16, blue_noise_texture t17,
//   color_tex u18 (RWTexture2D<float3>), $Globals b13, frame_constants set1/b0。
// 算法: 由深度缓冲重建相机->表面射线,沿射线积分 6 个指数间隔采样点;
//   每个采样点做阴影贴图比较、两个假光二次遮挡测试和按主题选取的径向密度衰减,
//   并累加太阳/天空/假光的散射光,透过率按指数衰减。

#include "inc/common.hlsl"
#include "inc/uv.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/math.hlsl"
#include "inc/shadow.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/cascades.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/perlin.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint2 depth_tex_dims;
    float global_density_scale;
    float global_size_scale;
    uint2 blue_noise_texture_dims;
    uint2 color_tex_dims;
};

Texture2DArray<float4> shadow_map : register(t14, space0);
Texture2D<float4> depth_tex : register(t16, space0);
Texture2D<float4> blue_noise_texture : register(t17, space0);
RWTexture2D<float3> color_tex : register(u18, space0);

// 假光的二次光柱遮挡测试:水平距离太远时返回 1.0(受光),否则判断射线是否
// 高于 18 单位的光柱高度。提前返回结构复现反汇编中的 switch 形退出。
float fake_light_occlusion(float2 xz, float height, float2 light_dir_xz, float light_dir_y,
                           float dist2, bool far_enough)
{
    if (far_enough)
    {
        return 1.0f;
    }

    // 求解射线与光柱 (半径 40 = sqrt(1600)) 的二次交点参数 t。
    float a = dot(light_dir_xz, light_dir_xz);
    float b = dot(xz, light_dir_xz);
    float t = ((-2.0f * b) + sqrt(((2.0f * b) * (2.0f * b)) - ((4.0f * a) * (dist2 - 1600.0f)))) / (2.0f * a);
    bool valid = (t == t) && (t >= 0.0f);
    if (valid)
    {
        // 交点处高度是否高于 18 单位:是则未遮挡(受光),否则被遮挡。
        return float((height + (light_dir_y * t)) > 18.0f);
    }
    else
    {
        return 1.0f;
    }
}

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    // 由像素坐标构造裁剪空间 UV(把 0..dims 映射到 [-1,1]x[1,-1])。
    float2 clip_uv = (((float2(int2(gl_GlobalInvocationID.xy)) + 0.5f) / float2(depth_tex_dims)) - 0.5f) * float2(2.0f, -2.0f);
    float depth = depth_tex.Load(int3(gl_GlobalInvocationID.xy, 0u)).x;
    // 用当前视图的 sample_to_view + view_to_world 重建世界坐标与视线方向。
    float4 world_pos = mul(view_constants[0].view_to_world,
                           mul(view_constants[0].sample_to_view, float4(clip_uv.x, clip_uv.y, depth, 1.0f)));
    float3 view_dir = -normalize(mul(view_constants[0].view_to_world,
        mul(view_constants[0].sample_to_view, float4(clip_uv.x, clip_uv.y, 0.0f, 1.0f))).xyz);
    float3 existing_color = color_tex[gl_GlobalInvocationID.xy].xyz;

    // 雾厚度接近 0 时不处理,直接返回。
    if (fog_tint_thickness.w <= 9.9999997473787516355514526367188e-06f)
    {
        return;
    }

    // 相机位置与相机到重建表面的射线。
    float4 camera_view = mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 camera_pos = camera_view.xyz / camera_view.w;
    float3 ray = (world_pos.xyz / world_pos.w) - camera_pos;
    float ray_len = length(ray);
    // 射线终点:超过 1024 则截断(把终点拉回视距范围内)。
    float3 ray_end = (ray * (min(1024.0f, ray_len) / max(9.9999996826552253889678874634872e-21f, ray_len))) + camera_pos;

    // 太阳方向 / 相位函数 / 密度参数。
    float4 sun_v = mul(view_constants[2].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f));
    // Henyey-Greenstein 相位函数近似,前向散射偏置 -1.3。
    float phase = 0.045955993235111236572265625f / pow(1.4224998950958251953125f - (dot(view_dir, sun_v.xyz) * (-1.2999999523162841796875f)), 1.5f);
    float density = (0.012500000186264514923095703125f * global_density_scale) * fog_tint_thickness.w;
    float density_scatter = density * 0.4000000059604644775390625f;
    float density_extinct = density * 1.2000000476837158203125f;
    float size_near = 30.0f * global_size_scale;
    float size_far = 300.0f * global_size_scale;
    // 地面消光:太阳高度越低,水平透过率越小。
    float ground_extinction = exp((density * (-36.0f)) / max(0.300000011920928955078125f, sun_v.y));

    // 当前活动视图下的相机与射线。
    float4 camera_current = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 camera_pos_cur = camera_current.xyz / camera_current.w;
    float3 ray_dir = ray_end - camera_pos_cur;
    float ray_dist = length(ray_dir);

    // 蓝噪声旋转抖动:按帧号旋转一个蓝噪声纹素偏移,降低带状伪影。
    uint2 noise_offset = uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(frame_index)) + 0.5f) * 256.0f);
    float4 noise = ((blue_noise_texture.Load(int3((gl_GlobalInvocationID.xy + uint2(13u, 13u) + noise_offset) % uint2(256u, 256u), 0u)) * 255.0f) * 0.00390625f) + 0.001953125f;
    float noise_val = noise.x;

    // 相机与射线终点在太阳视图下的投影,用于后续采样点的插值。
    float4 cam_proj = mul(view_constants[2].world_to_sample, float4(camera_pos_cur, 1.0f));
    float4 ray_end_proj = mul(view_constants[2].world_to_sample, float4(ray_end, 1.0f));

    // 沿射线积分 6 个指数间隔采样点。
    float3 total_light = 0.0f;
    float transmittance = ground_extinction;
    float prev_t = 0.0f;
    for (int i = 0; i < 6; ++i)
    {
        // 采样位置按视角仰角调整指数(低视角时采样更靠近平面的幂)。
        float sample_t = pow((float(i) + noise_val) * 0.16666667163372039794921875f, clamp(1.0f + (-2.0f * view_dir.y), 0.5f, 2.0f));
        float dt = sample_t - prev_t;
        float mid_t = lerp(prev_t, sample_t, 0.4000000059604644775390625f);
        float3 ray_pos = camera_pos_cur + (ray_dir * mid_t);
        float4 ray_proj = lerp(cam_proj, ray_end_proj, mid_t);

        // 阴影贴图比较项:投影移出 [-1,1] 时跳过(视为受光)。
        float2 proj_uv = ray_proj.xy;
        float2 clamped_uv = clamp(proj_uv, -1.0f, 1.0f);
        bool out_of_bounds = any(proj_uv != clamped_uv);
        float shadow_term;
        if (out_of_bounds)
        {
            shadow_term = 1.0f;
        }
        else
        {
            shadow_term = shadow_map.SampleCmpLevelZero(sampler_shadow_linear,
                float3((proj_uv * float2(0.5f, -0.5f)) + 0.5f, 0.0f), ray_proj.z);
        }

        // 两个假光的遮挡测试,共享水平距离平方 dist2。
        float2 xz = ray_pos.xz;
        float dist2 = dot(xz, xz);
        bool far_enough = dist2 > 1600.0f;
        float fake1 = fake_light_occlusion(xz, ray_pos.y, fake_light_dir.xz, fake_light_dir.y, dist2, far_enough);
        float fake2 = fake_light_occlusion(xz, ray_pos.y, fake_light2_dir.xz, fake_light2_dir.y, dist2, far_enough);

        // 径向密度衰减:相对 (size_far, size_near) 归一化的高斯衰减,并按主题微调。
        float3 scaled_pos = ray_pos / float2(size_far, size_near).xyx;
        float radial_density = 0.60000002384185791015625f * exp2(-dot(scaled_pos, scaled_pos));
        float xz_len = length(xz);
        float local_density;
        if (glade_theme_id == 2u)
        {
            local_density = min(radial_density * lerp(1.0f, 3.0f, smoothstep(60.0f, 200.0f, length(xz / global_size_scale.xx))), 0.800000011920928955078125f);
        }
        else if (glade_theme_id == 3u)
        {
            local_density = radial_density * lerp(1.0f, 8.0f, smoothstep(30.0f, 200.0f, length(xz / global_size_scale.xx)));
        }
        else if (glade_theme_id == 4u)
        {
            local_density = radial_density * lerp(1.0f, 4.0f, smoothstep(100.0f, 300.0f, xz_len));
        }
        else
        {
            local_density = radial_density * lerp(1.0f, 2.0f, smoothstep(100.0f, 300.0f, xz_len));
        }
        float density_used = max(local_density, 0.0f);

        // 散射光累加:太阳(带阴影)/ 天空 / 两个假光,透过率按指数衰减。
        float3 inscatter = (((((((((sun_color.xyz * shadow_term) * phase) * 4.0f)
            + (sky_high_color.xyz * lerp(4.0f, 8.0f, 1.0f - exp(-0.00999999977648258209228515625f * xz_len))))
            * ground_extinction)
            + ((fake_light_color.xyz * fake1) + (fake_light2_color.xyz * fake2)) * 0.3183098733425140380859375f)
            * (density_scatter * density_used))
            * ((1.0f - exp(-max(1.0000000116860974230803549289703e-07f, ((ray_dist * dt) * density_extinct) * density_used))) / max(9.9999996826552253889678874634872e-21f, density_extinct * density_used)))
            * transmittance);
        total_light += inscatter;
        transmittance *= exp(((-ray_dist * dt) * density_extinct) * density_used);
    }

    // 输出:散射光 + 原色乘以最终透过率。
    color_tex[gl_GlobalInvocationID.xy] = (total_light * fog_tint_thickness.xyz) + (existing_color * transmittance);
}
