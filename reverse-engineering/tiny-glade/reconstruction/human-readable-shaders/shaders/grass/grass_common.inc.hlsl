// ==RECONSTRUCTED== shaders/grass/grass_common.inc.hlsl
// Evidence: embedded in 8 grass compute bins (generate_grass, bundles_prefix,
// calc_dispatch_args, calc_grasslet_bundle_ranges, calc_tiles, tile_bundle_count,
// tile_bundle_gen, tiles_prefix). Include order puts grass_common.inc -> inc/consts.hlsl
// (consts appears immediately after grass_common in every consumer's embedded
// source list; calc_dispatch_args embeds no other file). GrassTilePacked / GrassBundlePacked
// are the raw uint tile / bundle records; OpMemberName "raw" verified in raw-per-bin.
// Status: O (struct layout + include edge verbatim); U (any additional members —
// nothing else is pinned by consumer evidence, so none are invented).
//
// 功能:草地系统的共享头文件,定义瓦片/束的打包记录结构。
// 逻辑与原文完全一致,仅补充中文注释。

#ifndef TINYGLADE_GRASS_GRASS_COMMON_INC_HLSL
#define TINYGLADE_GRASS_GRASS_COMMON_INC_HLSL

#include "inc/consts.hlsl"

// 一块草地瓦片记录。布局(依据 calc_tiles/generate_grass 的使用点):
//   24..31 位 = 瓦片 X,16..23 位 = 瓦片 Y,低 16 位 = 草叶数,按
//   `(raw & 0xFFFFu) % 512u` 解出(草叶数 / 草叶 LOD 列表)。
struct GrassTilePacked
{
    uint raw;
};

// 一条束记录,由 tile_bundle_gen 写入 / generate_grass 读取:
//   24..31 位 = 束瓦片 X,16..23 位 = 束瓦片 Y,低 16 位 = 束内草叶窗口
//   (即 `((tile_raw & 0xFFFF) % 512 / 128)` 等)。
struct GrassBundlePacked
{
    uint raw;
};

#endif // TINYGLADE_GRASS_GRASS_COMMON_INC_HLSL
