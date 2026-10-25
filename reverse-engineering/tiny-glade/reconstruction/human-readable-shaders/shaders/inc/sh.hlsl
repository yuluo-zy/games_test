#pragma once
// ======================================================================
// inc/sh.hlsl —— 球谐(SH)基函数
// ======================================================================
// 来源:_gi_eval_screen_radiance.cs.hlsl_cs_main_0 内联的 SH band-0+1 基
//       (float4(0.2820949852466583251953125f, dir * 0.48860299587249755859375f));
//       include 图 146 个 bin。
// 说明:只观测到 band 0+1;任何 band-2(9 系数)求值在此为 Unknown。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/sh.hlsl | evidence: _gi_eval_screen_radiance.cs.hlsl_cs_main_0 inlined SH band-0+1 basis (float4(0.2820949852466583251953125f, dir * 0.48860299587249755859375f)); include graph 146 bins | status: U
//
// Observation (from _gi_eval_screen_radiance inlined use-site):
//   float4(0.2820949852466583251953125f, ray_dir.xyz * 0.48860299587249755859375f)  == Y00, band-1 x/y/z
//   Used to accumulate 4-coefficient SH per RGB channel (output = 3 float4).
// Only band 0+1 is observed; any band-2 (9-coefficient) evaluation used by lighting.hlsl (INC-C)
// is an Unknown here.  Function names not embedded -> sh_basis/sh_eval are Inference (alt: evalSH,
// shEval, etc.).

// band-0+1 SH 基:常数项 Y00 + band-1 x/y/z。
float4 sh_basis(float3 dir)
{
    return float4(0.2820949852466583251953125f, 0.48860299587249755859375f * dir);
}

// 用 4 系数 SH 在方向 dir 上求值。
float sh_eval(float4 sh, float3 dir)
{
    return dot(sh, sh_basis(dir));
}
