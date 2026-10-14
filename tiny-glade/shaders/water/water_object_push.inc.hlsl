// ==RECONSTRUCTED== shaders/water/water_object_push.inc.hlsl
// 中文说明:
//   水面物体(莲叶/芦苇/莲花/叶片粒子)共用的"随波推送"位移头文件,被 7 个 bin
//   (_leaf_particles/_lilypad/_lilypad_flower/_reed 等)嵌入。消费者会先包含本
//   文件再声明 water_noise_texture,再包含 waves.inc.hlsl(它引用该纹理)。
//   消费者内联复用了同一套 {2,4,8,12}/{2,2,0.5,0.2}/{17,18,20,21} 波形表——
//   证据表明波形表 + 一个水面物体位移辅助函数最初就住在这里。本文件提供与此
//   一致的最小自包含辅助函数(主题 4 提前返回 + 每倍频旋转哈希抖动);
//   消费者仍保留各自内联的位移主体。

#ifndef TINYGLADE_WATER_OBJECT_PUSH_HLSL
#define TINYGLADE_WATER_OBJECT_PUSH_HLSL

#include "inc/math.hlsl"
#include "inc/math_const.hlsl"
#include "inc/perlin.hlsl"
#include "inc/hash.hlsl"
#include "inc/frame_constants.hlsl"

// 水面消费者(lilypad / reed / lilypad_flower / leaf_particles)与 waves.inc.hlsl
// 共享的波形表:每倍频的缩放、权重与哈希种子。
static const float WATER_PUSH_SCALE[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float WATER_PUSH_WEIGHT[4] = { 2.0f, 2.0f, 0.5f, 0.2f };
static const uint WATER_PUSH_SEED[4] = { 17u, 18u, 20u, 21u };

// 水面物体原点受到的"随波推送"位移。Glade 主题 4(秋/雪)关闭水面运动;
// 否则物体按每倍频旋转哈希抖动以时间为尺度被推动(与消费者内联的
// 双分量格点一致)。调用方把返回的 xz 偏移叠加到物体位置上。
float3 water_object_push(float2 origin_xz, float time)
{
    if (glade_theme_id == 4u)
    {
        return 0.0f;
    }
    float2 jitter = 0.0f;
    for (uint oct = 0u; oct < 4u; oct++)
    {
        float seed_f = float(WATER_PUSH_SEED[oct]);
        int cell = int(floor(seed_f));
        float f = frac(seed_f);
        uint c0 = uint(cell);
        uint c1 = uint(cell + 1);
        float u = f * f * (3.0f - 2.0f * f);
        float2 a = float2(incb_perlin_hash01(hash(uint2(c0, c0))), rand01(hash(c0 ^ (c0 >> 18u))));
        float2 b = float2(incb_perlin_hash01(hash(uint2(c1, c1))), rand01(hash(c1 ^ (c1 >> 18u))));
        jitter += (normalize(lerp(a, b, u) - 0.5f) * time * 3.0f) * WATER_PUSH_WEIGHT[oct];
    }
    return float3(jitter, 0.0f);
}

#endif // TINYGLADE_WATER_OBJECT_PUSH_HLSL
