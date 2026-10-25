// ============================================================
// inc/cascades.hlsl —— 阴影级联选择
// 用途:把世界坐标投影进太阳光阴影级联,选出级联切片与阴影 UV。
//   消费方:_apply_screen_shadows、_terrain_lighting_texture。
// 入口:无(纯头文件)
// 关键绑定:view_constants(帧常量中 2+slice 的太阳视图投影矩阵)
// 关键算法:级联边界测试 0.9833 与 1-0.000833*2*scale,UV 重映射 *0.5 或 *0.285714
// ============================================================
// ==RECONSTRUCTED== shaders/inc/cascades.hlsl | evidence: spirv-cross (_apply_screen_shadows/_terrain_lighting_texture) cascade selection against frame_constants.view_constants[2..4] (slice=0|1|2 via world_to_sample border tests 0.9833 and 1-0.000833*2*scale, remap *0.5 or *0.285714, view index 2+slice); include-graph edge shadow.hlsl -> cascades.hlsl | status: O (constants/math); O/I (struct + function names Inference)

#ifndef TINYGLADE_INC_CASCADES_HLSL
#define TINYGLADE_INC_CASCADES_HLSL

// 世界坐标投影进太阳阴影级联后的结果。
struct ShadowCascadeSelection {
    int slice;          // 在太阳背后/所有级联之外时为 -1
    float2 shadow_uv;   // 最终阴影贴图 UV(0..1)
    float texel_scale;  // 10(内层)或 5(外层)——边界余量的纹素宽度
    float sample_scale; // 1.0、0.5 或 1/3.5 —— UV 重映射系数
};

// 用 view_constants[2 + slice] 里的太阳视图投影矩阵为 world_pos 选择级联。
// 与观察到的原版逐操作一致。
ShadowCascadeSelection select_shadow_cascade(float3 world_pos)
{
    ShadowCascadeSelection sel;
    sel.slice = -1;
    sel.shadow_uv = float2(0.0, 0.0);
    sel.texel_scale = 10.0;
    sel.sample_scale = 1.0;

    float4 c0 = mul(view_constants[2].world_to_sample, float4(world_pos, 1.0));
    if (c0.z > 1.0) {
        return sel; // 在太阳背后
    }
    float2 uv = c0.xy;
    bool c0_out = any(abs(uv) > 0.983333349f);
    float texel_scale = c0_out ? 5.0 : 10.0;
    float sample_scale = c0_out ? 0.5 : 1.0;
    uint slice = c0_out ? 1u : 0u;
    if (c0_out) {
        uv *= 0.5;
    }
    bool c1_out = any(abs(uv) > (1.0 - (0.00083333335f * (2.0 * texel_scale))));
    if (c1_out) {
        texel_scale = 10.0 * 0.2857142984867095947265625f;
        uv *= 0.2857142984867095947265625f;
        sample_scale = sample_scale * 0.2857142984867095947265625f;
        slice = 2u;
    }
    // 重映射到 [0,1] 阴影贴图 UV(y 翻转)。
    float4 final_uv = mul(view_constants[2 + slice].world_to_sample, float4(world_pos, 1.0));
    sel.slice = int(slice);
    sel.shadow_uv = float2(final_uv.x, -final_uv.y) * 0.5 + 0.5;
    sel.texel_scale = texel_scale;
    sel.sample_scale = sample_scale;
    return sel;
}

#endif // TINYGLADE_INC_CASCADES_HLSL
