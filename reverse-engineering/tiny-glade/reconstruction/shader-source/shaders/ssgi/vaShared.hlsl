// ==RECONSTRUCTED== shaders/ssgi/vaShared.hlsl | F-D
// Evidence: _ssgi_ssgi.cs.hlsl_cs_main_0.bin (embedded source path
//   shaders/ssgi/vaShared.hlsl; inlined use-sites: the depth->view -Z
//   reconstruction `-1.0f / (depth * -clip_to_view[3][2])` appears in the
//   4-tap depth gather and both per-slice sample taps).
// Status: O (algorithm from inlined SPIR-V); I (function names — inlined).
//
// Shared view-space reconstruction helpers for the SSGI/GTAO passes. All
// helpers are static and inlined into the consumer; this header is
// self-contained (includes nothing).

#ifndef TINYGLADE_SSGI_VASHARED_HLSL
#define TINYGLADE_SSGI_VASHARED_HLSL

// Convert a [0,1] depth texel to the view-space -Z distance. `clip_to_view_32`
// is the clip_to_view[3][2] matrix element (the constant column of the inverse
// projection that maps NDC depth to view-space Z).
float va_depth_to_view_z(float clip_to_view_32, float depth)
{
    return (-1.0f) / (depth * (-clip_to_view_32));
}

#endif // TINYGLADE_SSGI_VASHARED_HLSL
