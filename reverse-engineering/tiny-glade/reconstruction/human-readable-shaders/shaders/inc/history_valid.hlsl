// ============================================================
// inc/history_valid.hlsl —— 时间历史有效性门控
// 用途:尺度不变的时间历史有效性判断,拒绝落在不同几何上的重投影样本。
//   消费方:_apply_screen_shadows、_calculate_reprojection_map、_ssgi_temporal_filter。
// 入口:无(纯头文件)
// 裁定说明:2026-10-13 裁定——原版 history_valid.hlsl 并不定义
//   FrexpStructType(其消费方 bin 中无此名);早期草案定义的标量版与
//   curve.hlsl 的 float3/int3 版在共享 TU(taa/tgsr)里冲突,故改名
//   FrexpScalarType 并自包含,使两文件共存。深度有效性主体仍为推断。
// ============================================================
// ==RECONSTRUCTED== shaders/inc/history_valid.hlsl | evidence: raw-per-bin consumer bins
// (_apply_screen_shadows/_calculate_reprojection_map/_ssgi_temporal_filter) scale-invariant
// temporal-history gating; history_valid.hlsl was NOT observed to embed FrexpStructType in its
// consumers (that name appears only in taa/tgsr/water_ice_refl_resolve via curve.hlsl).
// | status: struct+body Inference
//
// Adjudicated 2026-10-13 (main agent, RECOVERY_TRACKER): original history_valid.hlsl evidently
// did NOT define FrexpStructType (name absent from its consumers' bins). INC-C's earlier draft
// defined a scalar FrexpStructType that clashed with curve.hlsl's float3/int3 FrexpStructType in
// shared TUs (taa/tgsr). The scalar helper is renamed FrexpScalarType and made self-contained so
// both files coexist. The depth-validity body remains Inference.

#ifndef TINYGLADE_INC_HISTORY_VALID_HLSL
#define TINYGLADE_INC_HISTORY_VALID_HLSL

// 标量 C 风格 frexp 结果(尾数 + 指数)。
struct FrexpScalarType {
    float mantissa;
    float exponent;
};

// 标量 C 风格 frexp。
FrexpScalarType incb_frexp_scalar(float v)
{
    FrexpScalarType r;
    uint bits = asuint(v);
    int biased = int((bits >> 23u) & 0xFFu);
    int e = biased - 127;
    float m = asfloat((bits & 0x807FFFFFu) | 0x3F800000u);
    bool big = (m >= 1.0f) | (m <= -1.0f);
    m = lerp(m, m * 0.5f, big);
    e += big ? 1 : 0;
    bool special = (biased == 0) | (biased == 0xFF);
    r.exponent = special ? 0.0f : float(e);
    r.mantissa = special ? v : m;
    return r;
}

// 当前深度与历史深度在尺度相关 ε 内一致时为真。
// 用于拒绝落在不同几何上的重投影样本。
bool history_depth_valid(float current_depth, float history_depth, float epsilon)
{
    FrexpScalarType c = incb_frexp_scalar(current_depth);
    float scale = exp2(c.exponent);
    return abs(current_depth - history_depth) * scale < epsilon;
}

// 重投影消费方使用的便捷包装:重投影运动向量非零且存在有效历史时样本有效。
// 命名为 is_history_valid(裁定 inc-fix #16,RECOVERY_TRACKER #16):
// 早期名 `history_valid` 与多个消费方逐字嵌入的 $Globals 成员同名
// (OpMemberName history_valid)冲突。
bool is_history_valid(bool reprojected_available, uint history_valid_flag, uint frame_index)
{
    return reprojected_available && (history_valid_flag != 0u) && (frame_index != 0u);
}

#endif // TINYGLADE_INC_HISTORY_VALID_HLSL
