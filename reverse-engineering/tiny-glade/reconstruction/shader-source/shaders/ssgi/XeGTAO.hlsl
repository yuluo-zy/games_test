// ==RECONSTRUCTED== shaders/ssgi/XeGTAO.hlsl | F-D
// Evidence: _ssgi_ssgi.cs.hlsl_cs_main_0.bin (embedded source path
//   shaders/ssgi/XeGTAO.hlsl; inlined use-sites: the fast-acos horizon-angle
//   reconstruction `((-0.15658299624919891357421875f * x) + 1.57079601287841796875f)
//   * asfloat(532487669 + (asint(1.0f - x) >> 1))` appears once per horizon slice
//   plus twice at integration, matching the classic XeGTAO angle recovery).
// Status: O (algorithm/constants from inlined SPIR-V); I (function names — inlined).
//
// Horizon-based ambient-occlusion math (Intel GTAO lineage) shared by the SSGI
// pass. All helpers are static and inlined into the consumer; this header is
// self-contained (includes nothing).

#ifndef TINYGLADE_SSGI_XEGTAO_HLSL
#define TINYGLADE_SSGI_XEGTAO_HLSL

// Fast approximation of acos(x) for |x| <= 1, built from a linear correction
// plus an integer bit trick (asfloat(0x1FBD1A55... + (asint(1-x) >> 1))). The
// caller passes |x|; the sign handling is done at the call site.
float XeGTAO_Internal_FastACos(float abs_x)
{
    return ((-0.15658299624919891357421875f * abs_x) + 1.57079601287841796875f)
        * asfloat(532487669 + (asint(1.0f - abs_x) >> 1));
}

// Weighted length of the projected normal used to scale the AO contribution of
// a slice: strong for grazing normals, saturating toward 1.0 for head-on.
float XeGTAO_Internal_ProjectedNormalLen(float projected_normal_len)
{
    return lerp(projected_normal_len, 1.0f, 0.0500000007450580596923828125f);
}

#endif // TINYGLADE_SSGI_XEGTAO_HLSL
