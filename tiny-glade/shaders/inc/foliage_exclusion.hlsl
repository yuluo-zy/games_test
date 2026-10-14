// ============================================================
// inc/foliage_exclusion.hlsl —— 植被排除掩码
// 用途:采样 foliage_exclusion_mask(已清理/雕刻区域的矩形掩码),
//   抑制植被生长。
//   消费方:_clearing_meadow_plant、_generate_grass、_buildings_building_mask。
// 入口:无(纯头文件)
// 关键绑定:foliage_exclusion_mask 由消费方声明
// ============================================================
// ==RECONSTRUCTED== shaders/inc/foliage_exclusion.hlsl | evidence: consumers _clearing_meadow_plant/_generate_grass/_buildings_building_mask sample foliage_exclusion_mask (rectangular mask of cleared/sculpted regions) to suppress vegetation; include-graph edge buildings/building_mask.inc.hlsl -> foliage_exclusion.hlsl | status: O (role/mask); O/I (function names/body Inference)

#ifndef TINYGLADE_INC_FOLIAGE_EXCLUSION_HLSL
#define TINYGLADE_INC_FOLIAGE_EXCLUSION_HLSL

// 掩码采样缩放与地形高度图的世界映射一致。
static const float FOLIAGE_EXCLUSION_SAMPLE_SCALE = 0.0076923076994717121124267578125;

// 采样植被排除掩码:>0 表示该处植被被挖除。
float get_foliage_exclusion(Texture2D foliage_exclusion_mask, SamplerState s, float2 world_xz)
{
    float2 uv = world_xz * FOLIAGE_EXCLUSION_SAMPLE_SCALE + 0.5;
    return foliage_exclusion_mask.SampleLevel(s, uv, 0.0).x;
}

// 该世界坐标的植被是否位于排除区域内(掩码值 > threshold)。
bool is_foliage_excluded(Texture2D foliage_exclusion_mask, SamplerState s, float2 world_xz, float threshold)
{
    return get_foliage_exclusion(foliage_exclusion_mask, s, world_xz) > threshold;
}

#endif // TINYGLADE_INC_FOLIAGE_EXCLUSION_HLSL
