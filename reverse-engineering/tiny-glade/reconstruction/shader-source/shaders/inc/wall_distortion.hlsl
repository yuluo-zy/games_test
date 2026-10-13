// ==RECONSTRUCTED== shaders/inc/wall_distortion.hlsl | status: Inference (minimal
// behavior-consistent body, filled in P1c integration pass, adjudicated inc-fix #13).
// Embedded by 28 bins (nani roof/ivy/wall-brick surface shaders). The helper name/body
// are not recoverable (inlined); a small procedural UV distortion is the minimal
// consistent role for a wall-material distortion utility. Wave-1 consumers do not
// include this file.

#ifndef TINYGLADE_INC_WALL_DISTORTION_HLSL
#define TINYGLADE_INC_WALL_DISTORTION_HLSL

// Low-frequency procedural distortion applied to wall / plaster / brick UVs so
// the baked material reads non-uniformly across a surface.
float2 wall_distortion_uv(float2 uv, float2 world_pos, float time)
{
    float2 wave = float2(sin(world_pos.x * 0.125f + time * 0.5f),
                         cos(world_pos.y * 0.125f + time * 0.5f));
    return uv + wave * 0.0025f;
}

#endif // TINYGLADE_INC_WALL_DISTORTION_HLSL
