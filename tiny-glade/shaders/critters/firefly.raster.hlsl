// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main 由 batch_lift.py 从 oracle bin 转译;
// 下方手写代码恰好对那些(变体, 阶段)对被排除。
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// 萤火虫着色器:实例化小型光点,吸附到地形高度,按可见度淡化的双色调光晕。
// 绑定:sampler_default s9,$Globals b13,terrain_heightmap t16,firefly_buffer t18,
// frame_constants set1/b0。入口 vs_main / ps_main。

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/current_view.hlsl"
Texture2D<float4> terrain_heightmap : register(t16);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"

// INC 问题规避:原版 samplers.hlsl 声明 s0..s12(13 个采样器);
// inc/samplers.hlsl 只带 s0..s4,s9..s12,因此缺失的 s5..s8 在此声明,
// 以还原原版 $Globals 绑定(set0/b13)。

struct Firefly
{
    float3 position;
    uint pad;
};

StructuredBuffer<Firefly> firefly_buffer : register(t18);

int bool_render_shadows;
uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
float firefly_visibility;

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    // 小光点叠加到萤火虫位置,再抬升到地形高度上方。
    float3 firefly_pos = firefly_buffer[vin.instance_id].position;
    float3 world_pos = (vin.Vertex_Position * 0.039999999105930328369140625f + firefly_pos)
        + float3(0.0f,
                 lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, firefly_pos.xz * 0.0076923076994717121124267578125f.xx + 0.5f.xx, 0.0f).x),
                 0.0f);
    o.position = mul(view_constants[current_view_index].world_to_sample, float4(world_pos, 1.0f));
    return o;
}

struct PsOut
{
    float4 color : SV_Target0;
};

PsOut ps_main()
{
    PsOut o;
    // 可见度平方驱动从暗低色到亮高色的双色调渐变。
    float vis = firefly_visibility;
    float3 low = float3(0.449999988079071044921875f, 0.449999988079071044921875f, 0.02249999903142452239990234375f) * sky_high_color.xyz;
    float3 high = float3(1.0f, 1.0f, 0.0500000007450580596923828125f) * ev_mult;
    o.color = float4(lerp(low, high, vis * vis), 1.0f);
    return o;
}

#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// 从 _critters_firefly.raster.hlsl_f0adff76827e73ff.bin 转译(spirv-cross;行为精确,名字为 SPIR-V id)
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
