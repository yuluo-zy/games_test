#pragma once
// ======================================================================
// inc/simplex_noise.hlsl —— 2D Simplex 噪声
// ======================================================================
// 来源:_generate_grass.cs.hlsl_cs_main_0 内联的 2D simplex("snoise"),
//       Ashima/WebGL-noise 的逐 opcode 版本(38 个消费者 bin)。
// 核心:skew 常数 C = 0.3660254/0.2113249/-0.5773503,mod289 + permute 哈希,
//       m^4 衰减,130.0 点积输出。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/simplex_noise.hlsl | evidence: _generate_grass.cs.hlsl_cs_main_0 inlined 2D simplex (skew C = 0.3660254/0.2113249/-0.5773503, mod289 + permute hash, m^4 falloff, 130.0 dot); 38 consumer bins | status: I
//
// Observation (from _generate_grass theme-4 inlined use-site): the Ashima/WebGL-noise 2D simplex
// "snoise" verbatim opcode pattern: i = floor(v + dot(v,C.yy)); x0 = v - i + dot(i,C.xx);
// g = step(x0.yx,x0.xy); i1 = min(g,l.yx); i2 = max(g,l.yx); x1/x2 = ...; i4 = mod(i,289);
// p0 = permute(permute(i4.y + (0,i1.y,1)) + i4.x + (0,i1.x,1)); m = max(0.5-dot..); m=m*m;m=m*m;
// n = frac(permute(p0)*(1/41))*2-1; s = |n|-0.5; t = n - floor(n+0.5);
// return 130*dot(m*(1.79284285-(dot(t,t)+dot(s,s))*0.8537347), grad(x0,x1,x2)).
// Function name not embedded -> "snoise" Inference (alt: simplex_noise).

// 对 289 取模的辅助函数(标量/向量重载)。
float incb_mod289(float x)
{
    return fmod(x, 289.0f);
}

float3 incb_mod289(float3 x)
{
    return fmod(x, 289.0f);
}

// permute 哈希(标量/向量重载)。
float incb_permute(float x)
{
    return fmod((x * 34.0f + 1.0f) * x, 289.0f);
}

float3 incb_permute(float3 x)
{
    return fmod((x * 34.0f + 1.0f) * x, 289.0f);
}

// 2D simplex 噪声,输出大致在 [-1, 1]。
float snoise(float2 v)
{
    const float2 C = float2(0.211324865405187f, 0.366025403784439f); // G2, F2
    const float2 Cz = float2(-0.577350269189626f, -0.577350269189626f); // -(2*G2),用于 x2

    // 三角形网格:把输入坐标歪斜到整数格,再取回原坐标。
    float2 i = floor(v + dot(v, C.yy));
    float2 x0 = v - i + dot(i, C.xx);

    // 根据位置决定两个三角形顶点 i1/i2 的偏移。
    float2 g = step(x0.yx, x0.xy);
    float2 l = 1.0f - g;
    float2 i1 = min(g.xy, l.yx);
    float2 i2 = max(g.xy, l.yx);

    // 三个角到顶点的相对坐标。
    float2 x1 = x0 - i1 + C.xx;
    float2 x2 = x0 - i2 + Cz;

    // 顶点哈希。
    float i4x = incb_mod289(i.x);
    float i4y = incb_mod289(i.y);

    float3 p0 = incb_permute(incb_permute(i4y + float3(0.0f, i1.y, 1.0f)) + i4x + float3(0.0f, i1.x, 1.0f));

    // 每个顶点的贡献衰减:m = max(0.5 - d^2, 0),再平方两次(m^4)。
    float3 m = max(0.5f - float3(dot(x0, x0), dot(x1, x1), dot(x2, x2)), 0.0f);
    m = m * m;
    m = m * m;

    // 从哈希构造方向伪随机数 n ∈ [-1, 1],再拆成 s(带符号)与 t。
    float3 n = frac(incb_permute(p0) * (1.0f / 41.0f)) * 2.0f;
    n = n - 1.0f;
    float3 s = abs(n) - 0.5f;
    float3 t = n - floor(n + 0.5f);

    // 三个角各自的梯度贡献。
    float3 grad = float3(t.x * x0.x + s.x * x0.y,
                         t.y * x1.x + s.y * x1.y,
                         t.z * x2.x + s.z * x2.y);

    // 衰减权 x 归一化后求和,放大 130 倍输出。
    return 130.0f * dot(m * (1.79284291400159f - (dot(t, t) + dot(s, s)) * 0.85373472095314f), grad);
}
