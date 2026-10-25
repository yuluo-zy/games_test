// ==RECONSTRUCTED== shaders/grass/grass_common.inc.hlsl
// Evidence: embedded in 8 grass compute bins (generate_grass, bundles_prefix,
// calc_dispatch_args, calc_grasslet_bundle_ranges, calc_tiles, tile_bundle_count,
// tile_bundle_gen, tiles_prefix). Include order puts grass_common.inc -> inc/consts.hlsl
// (consts appears immediately after grass_common in every consumer's embedded
// source list; calc_dispatch_args embeds no other file). GrassTilePacked / GrassBundlePacked
// are the raw uint tile / bundle records; OpMemberName "raw" verified in raw-per-bin.
// Status: O (struct layout + include edge verbatim); U (any additional members —
// nothing else is pinned by consumer evidence, so none are invented).

#ifndef TINYGLADE_GRASS_GRASS_COMMON_INC_HLSL
#define TINYGLADE_GRASS_GRASS_COMMON_INC_HLSL

#include "inc/consts.hlsl"

// One grass tile record. Layout (from use sites in calc_tiles/generate_grass):
//   bits 24..31 tile_x, bits 16..23 tile_y, low bits = blade count packed as
//   `(raw & 0xFFFFu) % 512u` (blade_count / the grasslet LOD list).
struct GrassTilePacked
{
    uint raw;
};

// One bundle record written by tile_bundle_gen / read by generate_grass:
//   bits 24..31 bundle tile_x, bits 16..23 bundle tile_y, low bits = packed
//   blade window within the bundle (`((tile_raw & 0xFFFF) % 512 / 128)` etc.).
struct GrassBundlePacked
{
    uint raw;
};

#endif // TINYGLADE_GRASS_GRASS_COMMON_INC_HLSL
