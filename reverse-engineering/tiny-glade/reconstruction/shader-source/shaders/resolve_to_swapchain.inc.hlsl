// ==RECONSTRUCTED== shaders/resolve_to_swapchain.inc.hlsl | F-E | evidence: blit/resolve_to_swapchain.raster + blit/resolve_msaa_to_swapchain.raster + resolve_to_swapchain.cs modules — shared flip: `transform_image & 1` selects y-flip of the sample coordinate against dims.y; the three consumers then Load/average and linear_to_srgb (color/srgb.hlsl); include set (frame_constants/uv/ray_cone/math_const) present in all three bins though unused at codegen | status: O (transform_image global member 0 of $Globals + flip math from inlined use-sites); I (helper name — inlined; dims global name differs per consumer: framebuffer_dims vs output_tex_dims, so it stays in the main file)
//
// Shared logic for resolving the HDR framebuffer to the swapchain: coordinate
// transform flag + per-pixel sample coordinate (before MSAA resolve / encode).

#ifndef TINYGLADE_RESOLVE_TO_SWAPCHAIN_INC_HLSL
#define TINYGLADE_RESOLVE_TO_SWAPCHAIN_INC_HLSL

#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/color/srgb.hlsl"

// Apply the swapchain orientation transform to a pixel coordinate. `transform`
// is the engine-set transform_image flag (member 0 of the per-shader $Globals
// cbuffer, declared in the consuming main file).
int2 resolve_to_swapchain_coord(int2 pixel, uint2 dims, uint transform)
{
    if ((transform & 1u) != 0u)
    {
        pixel.y = int(dims.y - 1u - uint(pixel.y));
    }
    return pixel;
}

#endif // TINYGLADE_RESOLVE_TO_SWAPCHAIN_INC_HLSL
