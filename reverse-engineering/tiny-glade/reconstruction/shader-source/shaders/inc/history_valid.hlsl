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

struct FrexpScalarType {
    float mantissa;
    float exponent;
};

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

// True when current and history depth agree within a scale-relative epsilon.
// Used to reject reprojected samples that land on different geometry.
bool history_depth_valid(float current_depth, float history_depth, float epsilon)
{
    FrexpScalarType c = incb_frexp_scalar(current_depth);
    float scale = exp2(c.exponent);
    return abs(current_depth - history_depth) * scale < epsilon;
}

// Convenience wrapper used by reprojection consumers: a sample is valid when
// the reprojection motion vector is non-zero and a valid history exists.
// Named is_history_valid (adjudicated inc-fix #16, RECOVERY_TRACKER #16): the
// earlier name `history_valid` collided with the $Globals member of the same
// name that several consumers embed verbatim (OpMemberName history_valid).
bool is_history_valid(bool reprojected_available, uint history_valid_flag, uint frame_index)
{
    return reprojected_available && (history_valid_flag != 0u) && (frame_index != 0u);
}

#endif // TINYGLADE_INC_HISTORY_VALID_HLSL
