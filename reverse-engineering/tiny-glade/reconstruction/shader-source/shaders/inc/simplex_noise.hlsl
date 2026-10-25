#pragma once
// ==RECONSTRUCTED== shaders/inc/simplex_noise.hlsl | evidence: _generate_grass.cs.hlsl_cs_main_0 inlined 2D simplex (skew C = 0.3660254/0.2113249/-0.5773503, mod289 + permute hash, m^4 falloff, 130.0 dot); 38 consumer bins | status: I
//
// Observation (from _generate_grass theme-4 inlined use-site): the Ashima/WebGL-noise 2D simplex
// "snoise" verbatim opcode pattern: i = floor(v + dot(v,C.yy)); x0 = v - i + dot(i,C.xx);
// g = step(x0.yx,x0.xy); i1 = min(g,l.yx); i2 = max(g,l.yx); x1/x2 = ...; i4 = mod(i,289);
// p0 = permute(permute(i4.y + (0,i1.y,1)) + i4.x + (0,i1.x,1)); m = max(0.5-dot..); m=m*m;m=m*m;
// n = frac(permute(p0)*(1/41))*2-1; s = |n|-0.5; t = n - floor(n+0.5);
// return 130*dot(m*(1.79284285-(dot(t,t)+dot(s,s))*0.8537347), grad(x0,x1,x2)).
// Function name not embedded -> "snoise" Inference (alt: simplex_noise).

float incb_mod289(float x)
{
    return fmod(x, 289.0f);
}

float3 incb_mod289(float3 x)
{
    return fmod(x, 289.0f);
}

float incb_permute(float x)
{
    return fmod((x * 34.0f + 1.0f) * x, 289.0f);
}

float3 incb_permute(float3 x)
{
    return fmod((x * 34.0f + 1.0f) * x, 289.0f);
}

float snoise(float2 v)
{
    const float2 C = float2(0.211324865405187f, 0.366025403784439f); // G2, F2
    const float2 Cz = float2(-0.577350269189626f, -0.577350269189626f); // -(2*G2), used for x2

    float2 i = floor(v + dot(v, C.yy));
    float2 x0 = v - i + dot(i, C.xx);

    float2 g = step(x0.yx, x0.xy);
    float2 l = 1.0f - g;
    float2 i1 = min(g.xy, l.yx);
    float2 i2 = max(g.xy, l.yx);

    float2 x1 = x0 - i1 + C.xx;
    float2 x2 = x0 - i2 + Cz;

    float i4x = incb_mod289(i.x);
    float i4y = incb_mod289(i.y);

    float3 p0 = incb_permute(incb_permute(i4y + float3(0.0f, i1.y, 1.0f)) + i4x + float3(0.0f, i1.x, 1.0f));

    float3 m = max(0.5f - float3(dot(x0, x0), dot(x1, x1), dot(x2, x2)), 0.0f);
    m = m * m;
    m = m * m;

    float3 n = frac(incb_permute(p0) * (1.0f / 41.0f)) * 2.0f;
    n = n - 1.0f;
    float3 s = abs(n) - 0.5f;
    float3 t = n - floor(n + 0.5f);

    float3 grad = float3(t.x * x0.x + s.x * x0.y,
                         t.y * x1.x + s.y * x1.y,
                         t.z * x2.x + s.z * x2.y);

    return 130.0f * dot(m * (1.79284291400159f - (dot(t, t) + dot(s, s)) * 0.85373472095314f), grad);
}
