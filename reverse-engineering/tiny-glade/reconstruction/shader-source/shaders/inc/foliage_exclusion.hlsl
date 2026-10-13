// ==RECONSTRUCTED== shaders/inc/foliage_exclusion.hlsl | evidence: consumers _clearing_meadow_plant/_generate_grass/_buildings_building_mask sample foliage_exclusion_mask (rectangular mask of cleared/sculpted regions) to suppress vegetation; include-graph edge buildings/building_mask.inc.hlsl -> foliage_exclusion.hlsl | status: O (role/mask); O/I (function names/body Inference)

#ifndef TINYGLADE_INC_FOLIAGE_EXCLUSION_HLSL
#define TINYGLADE_INC_FOLIAGE_EXCLUSION_HLSL

// Mask sampling scale matches the terrain heightmap world mapping.
static const float FOLIAGE_EXCLUSION_SAMPLE_SCALE = 0.0076923076994717121124267578125;

// Sample the foliage-exclusion mask: >0 means the foliage is carved out.
float get_foliage_exclusion(Texture2D foliage_exclusion_mask, SamplerState s, float2 world_xz)
{
    float2 uv = world_xz * FOLIAGE_EXCLUSION_SAMPLE_SCALE + 0.5;
    return foliage_exclusion_mask.SampleLevel(s, uv, 0.0).x;
}

// True when foliage at this world position is inside an exclusion region.
bool is_foliage_excluded(Texture2D foliage_exclusion_mask, SamplerState s, float2 world_xz, float threshold)
{
    return get_foliage_exclusion(foliage_exclusion_mask, s, world_xz) > threshold;
}

#endif // TINYGLADE_INC_FOLIAGE_EXCLUSION_HLSL
