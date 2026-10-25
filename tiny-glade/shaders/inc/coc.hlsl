// ============================================================
// inc/coc.hlsl —— 弥散圆(CoC)与自动对焦参数
// 用途:定义自动对焦参数布局与有符号弥散圆公式,供景深 pass 使用。
//   消费方:_dof_autofocus、_dof_fullres、_critters_firefly_glow。
// 入口:无(纯头文件)
// 关键绑定:DofFocus 64B 与 dof_autofocus 的 SSBO 布局一致
// ============================================================
// ==RECONSTRUCTED== shaders/inc/coc.hlsl | evidence: raw-per-bin (_dof_autofocus/_dof_fullres/_critters_firefly_glow) DofFocus{focus0,focus1,lock_bias0,lock_bias1,lock_pos_ws,is_locked,interp_lock_pos_ws,use_tilt_shift,interp_lock_bias0,interp_lock_bias1,focus_scale,pad2} 64B + DofFocusReadback{lock_pos_ws,lock_reponse}; dof_autofocus stores focus fields at 0/4/16/28/32/56 | status: O (struct layouts); O/I (coc formula Inference)
// MERGE (dof-aa review inc-fixes #1/#2, 2026-10-13): ring_samples + gaussian_weights removed
// (the bins embed ring_samples once per gather consumer as the consumer's own groupshared;
// consumers keep their dof_ring_samples naming). DofFocusReadback member renamed
// focus_lock_request -> lock_reponse (original typo per bin OpMemberName; the $Globals
// member focus_lock_request is separate and correct).

#ifndef TINYGLADE_INC_COC_HLSL
#define TINYGLADE_INC_COC_HLSL

// 每相机自动对焦参数(64 字节,与 dof_autofocus 的 SSBO 布局一致)。
struct DofFocus {
    float focus0;
    float focus1;
    float lock_bias0;
    float lock_bias1;
    float3 lock_pos_ws;
    uint is_locked;
    float3 interp_lock_pos_ws;
    uint use_tilt_shift;
    float interp_lock_bias0;
    float interp_lock_bias1;
    float focus_scale;
    uint pad2;
};

// CPU 对焦锁定请求的读回结构。
struct DofFocusReadback {
    float3 lock_pos_ws;
    uint lock_reponse; // bin 成员名沿用原版拼写 `lock_reponse`
};

// 有符号弥散圆(归一化 [0,1] 像素):焦点后方为正、前方为负。
// 随 1/深度误差线性增长。
float compute_coc(float depth, float focus_dist, float focus_scale)
{
    float coc = (focus_dist - depth) / max(depth, 1e-6);
    return clamp(coc * focus_scale, -1.0, 1.0);
}

#endif // TINYGLADE_INC_COC_HLSL
