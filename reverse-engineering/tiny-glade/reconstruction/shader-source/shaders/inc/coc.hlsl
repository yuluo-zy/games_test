// ==RECONSTRUCTED== shaders/inc/coc.hlsl | evidence: raw-per-bin (_dof_autofocus/_dof_fullres/_critters_firefly_glow) DofFocus{focus0,focus1,lock_bias0,lock_bias1,lock_pos_ws,is_locked,interp_lock_pos_ws,use_tilt_shift,interp_lock_bias0,interp_lock_bias1,focus_scale,pad2} 64B + DofFocusReadback{lock_pos_ws,lock_reponse}; dof_autofocus stores focus fields at 0/4/16/28/32/56 | status: O (struct layouts); O/I (coc formula Inference)
// MERGE (dof-aa review inc-fixes #1/#2, 2026-10-13): ring_samples + gaussian_weights removed
// (the bins embed ring_samples once per gather consumer as the consumer's own groupshared;
// consumers keep their dof_ring_samples naming). DofFocusReadback member renamed
// focus_lock_request -> lock_reponse (original typo per bin OpMemberName; the $Globals
// member focus_lock_request is separate and correct).

#ifndef TINYGLADE_INC_COC_HLSL
#define TINYGLADE_INC_COC_HLSL

// Per-camera autofocus parameters (64 bytes, matches the dof_autofocus SSBO).
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

// CPU readback of a focus-lock request.
struct DofFocusReadback {
    float3 lock_pos_ws;
    uint lock_reponse; // bin member name is the original typo `lock_reponse`
};

// Signed circle-of-confusion (in normalized [0,1] pixels): positive behind
// focus, negative in front. Grows linearly with 1/depth error.
float compute_coc(float depth, float focus_dist, float focus_scale)
{
    float coc = (focus_dist - depth) / max(depth, 1e-6);
    return clamp(coc * focus_scale, -1.0, 1.0);
}

#endif // TINYGLADE_INC_COC_HLSL
