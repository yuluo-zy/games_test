#pragma once
// ==RECONSTRUCTED== shaders/inc/gaussian_rand.hlsl | evidence: _generate_grass.cs.hlsl_cs_main_0 inlined gaussian distribution (log(1-u*u), c=4.5472841+0.5*L, sqrt(sqrt(c*c-L*50/7)-c)); 12 consumer bins | status: I
//
// Observation (from _generate_grass inlined use-site, two independent hash seeds):
//   u   = (rand01(hash(seed_b)) * 2) - 1                 // [-1,1]
//   L   = log(1 - u*u)
//   c   = 4.5472841 + 0.5*L
//   ret = (rand01(hash(seed_a)) * 1.5 + 0.5) + (sqrt(sqrt(c*c - L*7.142857) - c) * sign(u)) * 0.3
// hash(uint)/rand01(uint) come from hash.hlsl (INC-A, reconstructed).  Include order in
// _generate_grass puts hash.hlsl before gaussian_rand.hlsl so no include is added here.
// Function name/signature not embedded -> "gaussian_rand(uint,uint)" Inference.

float gaussian_rand(uint seed_a, uint seed_b)
{
    float u = (rand01(hash(seed_b)) * 2.0f) - 1.0f;
    float L = log(1.0f - u * u);
    float c = 4.5472841f + 0.5f * L;
    float r = (rand01(hash(seed_a)) * 1.5f) + 0.5f;
    return r + (sqrt(sqrt((c * c) - (L * 7.142857f)) - c) * sign(u)) * 0.3f;
}
