// ==RECONSTRUCTED== shaders/water/waves.inc.hlsl
// 中文说明:
//   水面波浪位移辅助头文件,水面与水瀑布片元着色器共享。被 _leaf_particles/
//   _lilypad/_lilypad_flower/_reed/_water_plane/_waterfall(共 11 bin)嵌入。
//   | 状态:O(4 倍频旋转水噪声位移 + 涟漪源求和来自内联使用点);I(函数名)
//   合并注记:采用 A 的波形表 {2,4,8,12}/{2,2,0.5,0.2}/{17,18,20,21} +
//   2 格点 smoothstep 混合 + 涟漪阻尼公式;B 的标量双分量 hash01(x 来自 h,
//   y 来自 h+(h<<15),故 x != y)用于每倍频偏移(复刻 .xx 是错误的);补充了
//   bin 的 3 相中心差分涟漪法线(sin(w1),(sin(w2)-sin(w1))*0.5,
//   (sin(w3)-sin(w1))*0.5),+0.01 偏移——两个候选都没抓住它。

#ifndef TINYGLADE_WATER_WAVES_HLSL
#define TINYGLADE_WATER_WAVES_HLSL

#include "inc/math.hlsl"
#include "inc/math_const.hlsl"

// 消费者观察到的倍频缩放/权重/种子(_246={2,4,8,12} 缩放,
// _247={2,2,0.5,0.2} 权重,_248={17,18,20,21} 种子)。
static const float WAVE_SCALE[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float WAVE_WEIGHT[4] = { 2.0f, 2.0f, 0.5f, 0.2f };
static const uint WAVE_SEED[4] = { 17u, 18u, 20u, 21u };

// 格点的标量双分量 hash01(bin:x 来自雪崩哈希 h,y 来自 h 的再哈希链 +<<15,
// 所以 x != y)。格点哈希即 perlin 2D 格点哈希,取 p.x == p.y == cell。
float2 waves_hash01_2(uint cell)
{
    uint y = cell;
    y += (y << 10u);
    y ^= (y >> 6u);
    y += (y << 3u);
    y ^= (y >> 11u);
    uint h = ((cell * 1664525u) + (y + (y << 15u))) + 1013904223u;
    h *= 1664525u;
    h ^= (h >> 11u);
    h ^= (h << 7u) & 2636928640u;
    h ^= (h << 15u) & 4022730752u;
    uint hx = h ^ (h >> 18u);
    uint hy = hx;
    hy += (hy << 10u);
    hy ^= (hy >> 6u);
    hy += (hy << 3u);
    hy ^= (hy >> 11u);
    return float2(asfloat((hx & 0x7FFFFFu) | 0x3F800000u) - 1.0f,
                  asfloat(((hy + (hy << 15u)) & 0x7FFFFFu) | 0x3F800000u) - 1.0f);
}

// 每倍频旋转水噪声采样坐标偏移。`octave` 选择种子表;偏移是双分量 hash01
// 向量的 2 格点 smoothstep 混合,以时间为尺度(镜像 bin 的
// (normalize(lerp(...)) - 0.5) * time * 3)。
float2 water_wave_offset(uint octave, float time)
{
    float seed = float(WAVE_SEED[octave]);
    int cell = int(floor(seed));
    float f = frac(seed);
    uint c0 = uint(cell);
    uint c1 = uint(cell + 1);
    float u = f * f * (3.0f - 2.0f * f);
    float2 d0 = waves_hash01_2(c0);
    float2 d1 = waves_hash01_2(c1);
    return (normalize(lerp(d0, d1, u)) - 0.5f) * time * 3.0f;
}

// 累加 4 个旋转水噪声倍频得到一个位移向量。
float3 water_wave_displacement(float3 world_xz, float time)
{
    float3 acc = 0.0f;
    for (uint i = 0u; i < 4u; ++i)
    {
        float2 r = water_wave_offset(i, time);
        float angle = float(i + 1u) * 10.16640758514404296875f;
        float2x2 rot = float2x2(float2(cos(angle), sin(angle)), float2(-sin(angle), cos(angle)));
        float3 sample_v = (water_noise_texture.Sample(sampler_llr,
            mul(rot, (world_xz.xz * WAVE_SCALE[i]) + r) * 0.01200000010430812835693359375f).xyz
            * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f);
        acc = float3(acc.x, acc.yz + mul(((sample_v * WAVE_WEIGHT[i])
            * float2(1.0f, WAVE_SCALE[i] * 0.01200000010430812835693359375f).xyy).yz, rot));
    }
    return acc;
}

// 单个涟漪源在世界 xz 点的环带衰减:
// sin((ring*16.365) - time*6) / (1 + max(len,ring)^4),ring = len + dot(d, vel)。
float waves_ripple_ring(float2 mid, float2 vel, float2 p, float time)
{
    float2 d = mid - p;
    float len_d = length(d);
    float ring = len_d + dot(d, vel);
    float amp = max(len_d, ring);
    return sin((ring * 16.3649997711181640625f) - (time * 6.0f)) / (1.0f + ((amp * amp) * (amp * amp)));
}

// 从字节寻址涟漪缓冲区汇总 `world_xz` 附近的涟漪源位移(每条记录:
// half2 起点,half2 终点,float 振幅)。返回 3 相中心差分法线
// (sin(w1),(sin(w2)-sin(w1))*0.5,(sin(w3)-sin(w1))*0.5),在 +0.01 世界 xz 偏移采样。
float3 water_ripple_displacement(ByteAddressBuffer ripple_source_buffer, float3 world_xz, float time)
{
    float3 acc = 0.0f;
    uint count = min(32u, ripple_source_buffer.Load<uint>(512));
    for (uint i = 0u; i < count; ++i)
    {
        uint4 rec = ripple_source_buffer.Load<uint4>(i * 16u + 0u);
        float2 a = float2(f16tof32(rec.x & 0xFFFFu), f16tof32((rec.x >> 16u) & 0xFFFFu));
        float2 b = float2(f16tof32(rec.y & 0xFFFFu), f16tof32((rec.y >> 16u) & 0xFFFFu));
        float2 mid = lerp(a, b, 0.5f);
        float2 vel = (b - a) * 3.0f;
        float w1 = waves_ripple_ring(mid, vel, world_xz.xz, time);
        float w2 = waves_ripple_ring(mid, vel, world_xz.xz - float2(0.00999999977648258209228515625f, 0.0f), time);
        float w3 = waves_ripple_ring(mid, vel, world_xz.xz - float2(0.0f, 0.00999999977648258209228515625f), time);
        acc += float3(w1, (w2 - w1) * 0.5f, (w3 - w1) * 0.5f) * asfloat(rec.z);
    }
    return acc;
}

#endif // TINYGLADE_WATER_WAVES_HLSL
