// 全局光照(GI)半分辨率设置与常量。
// 半分辨率亚像素偏移表 `hi_px_subpixels` 由 _gi_* 消费端的调试名钉定;
// 其余常量值为推断(消费端内联,仅亚像素表可核)。

#ifndef TINYGLADE_GI_SETTINGS_INC_HLSL
#define TINYGLADE_GI_SETTINGS_INC_HLSL

// 半分辨率亚像素偏移(逐帧抖动相位)。用 `hi_px_subpixels[frame_index & 3]`
// 偏移全分辨率纹素读取,使其命中匹配的半分辨率采样,保证与相机抖动一致的
// 半分辨率读取。
static const uint2 hi_px_subpixels[4] = {
    uint2(0u, 0u),
    uint2(0u, 1u),
    uint2(1u, 1u),
    uint2(1u, 0u),
};

// GI 屏幕空间设置常量(推断;消费端内联)。
static const float GI_MIN_HALF_NORMAL_Z = 0.3f;          // max(0.3, normal.z) 权重下限
static const float GI_DEPTH_WEIGHT_SCALE = 80.0f;        // exp2(-min(30, 80*depth_err))
static const float GI_DEPTH_WEIGHT_MAX = 30.0f;
static const float GI_NORMAL_DOT_DAMP = -0.9f;           // dot^2 * -0.9 + 1
static const float GI_FINAL_WEIGHT_NORM = 0.01f;         // 0.01 / (dot^2*-0.9+1)^2

#endif // TINYGLADE_GI_SETTINGS_INC_HLSL
