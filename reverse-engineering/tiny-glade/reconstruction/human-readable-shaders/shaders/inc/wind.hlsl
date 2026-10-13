// 风场工具:为旗帜/叶子粒子/花园花/睡莲等顶点弯曲提供阵风噪声+时间包络。
// 函数名与模型为推断(原版内联,依赖 INC-B 的 perlin/simplex 噪声契约)。

#ifndef TINYGLADE_INC_WIND_HLSL
#define TINYGLADE_INC_WIND_HLSL

// 确定性伪随机噪声(位置+种子哈希,退化为值噪声;
// 原版在同一阵风包络上叠加 INC-B 的 perlin/simplex 噪声)。
float wind_noise(float2 p)
{
    uint2 ip = uint2(int2(p * 0.5));
    float2 fp = frac(p * 0.5);
    float2 u = fp * fp * (3.0 - 2.0 * fp);
    uint h00 = (ip.x * 374761393u) + (ip.y * 668265263u);
    uint h01 = ((ip.x + 1u) * 374761393u) + (ip.y * 668265263u);
    uint h10 = (ip.x * 374761393u) + ((ip.y + 1u) * 668265263u);
    uint h11 = ((ip.x + 1u) * 374761393u) + ((ip.y + 1u) * 668265263u);
    float n00 = float(h00 & 0x00FFFFFFu) / 16777215.0;
    float n01 = float(h01 & 0x00FFFFFFu) / 16777215.0;
    float n10 = float(h10 & 0x00FFFFFFu) / 16777215.0;
    float n11 = float(h11 & 0x00FFFFFFu) / 16777215.0;
    return lerp(lerp(n00, n01, u.x), lerp(n10, n11, u.x), u.y) * 2.0 - 1.0;
}

// 对 world_pos 处顶点计算风位移。gust 为缓慢移动的低频包络;
// flutter 为较快的高频项;振幅与方向偏置按顶点种子变化。
float3 evaluate_wind(float3 world_pos, float seed, float amplitude)
{
    float t = game_time_sec + game_time_frac;
    float2 xz = world_pos.xz;
    float gust = wind_noise(xz * 0.02 + float2(t * 0.03, 0.0) + float2(seed * 13.0, 0.0));
    float flutter = wind_noise(xz * 0.3 + float2(t * 0.5, seed) );
    float bend = saturate(gust * 0.5 + 0.5) * amplitude;
    // 朝 -x 方向摆动,外加垂直 flutter。
    float3 disp = float3(-gust * bend, flutter * 0.05 * amplitude, 0.0);
    disp *= smoothstep(0.0, 1.0, max(world_pos.y, 0.0));
    return disp;
}

#endif // TINYGLADE_INC_WIND_HLSL
