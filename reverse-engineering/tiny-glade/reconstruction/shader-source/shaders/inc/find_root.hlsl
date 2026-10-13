#pragma once
// ==RECONSTRUCTED== shaders/inc/find_root.hlsl | evidence: _rocky_terrain_stairs_stairs.cs.hlsl_cs_main_0 inlined bisection root-find (2 iterations of midpoint sampling with sign-change test fm*fa<0, then linear interpolation lerp(b, a, fb/(fb-fa))); include graph 2 bins | status: U
//
// Observation (from _rocky_terrain_stairs_stairs inlined use-site, called with the segment corners
// _149[0]/_149[3] and a target stair height):
//   fa = height(a)-target; fb = height(b)-target; for 2 iters: m = (a+b)/2; fm = height(m)-target;
//   if (fm*fa < 0) b = m else a = m;  return lerp(b, a, clamp(fb/(fb-fa), 0, 1)).
// "height" here is the terrain height sampled via terrain_heightmap (inlined as
// lerp(-2.5,24, SampleLevel(sampler_default, pos/130 + 0.5).x)).  The exact function it calls
// (terrain_height in INC-C's terrain_heightmap.inc.hlsl, per include-graph find_root -> place_stairs.inc)
// is an Unknown dependency; the residual function is externalized below as terrain_height(float2).
// Function name/signature not embedded -> "find_root(float2,float2,float)" Inference.

float2 find_root(float2 p0, float2 p1, float target_height)
{
    float2 a = p0;
    float2 b = p1;
    float fa = terrain_height(a) - target_height;
    float fb = terrain_height(b) - target_height;

    [unroll]
    for (int i = 0; i < 2; i++)
    {
        float2 m = (a + b) * 0.5f;
        float fm = terrain_height(m) - target_height;
        if (fm * fa < 0.0f)
        {
            b = m;
            fb = fm;
        }
        else
        {
            a = m;
            fa = fm;
        }
    }

    float t = clamp(fb / (fb - fa), 0.0f, 1.0f);
    return lerp(b, a, t);
}
