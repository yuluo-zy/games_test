#pragma once
// ==RECONSTRUCTED== shaders/inc/sh.hlsl | evidence: _gi_eval_screen_radiance.cs.hlsl_cs_main_0 inlined SH band-0+1 basis (float4(0.2820949852466583251953125f, dir * 0.48860299587249755859375f)); include graph 146 bins | status: U
//
// Observation (from _gi_eval_screen_radiance inlined use-site):
//   float4(0.2820949852466583251953125f, ray_dir.xyz * 0.48860299587249755859375f)  == Y00, band-1 x/y/z
//   Used to accumulate 4-coefficient SH per RGB channel (output = 3 float4).
// Only band 0+1 is observed; any band-2 (9-coefficient) evaluation used by lighting.hlsl (INC-C)
// is an Unknown here.  Function names not embedded -> sh_basis/sh_eval are Inference (alt: evalSH,
// shEval, etc.).

float4 sh_basis(float3 dir)
{
    return float4(0.2820949852466583251953125f, 0.48860299587249755859375f * dir);
}

float sh_eval(float4 sh, float3 dir)
{
    return dot(sh, sh_basis(dir));
}
