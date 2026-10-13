// ==RECONSTRUCTED== shaders/water/spawn_waterwheels.cs.hlsl | evidence: spirv-cross + raw disasm of _water_spawn_waterwheels.cs.hlsl_cs_main_0.bin — entry is EMPTY (only LocalSize 64,1,1 + cs_main; no bindings, no body ops); header carries OpCapability RuntimeDescriptorArray + SPV_EXT_descriptor_indexing + GLSL.std.450 import, which the original frame_constants.hlsl emitted via a dead unbounded-array declaration (verified 307/307 bins: frame_constants included <=> RDA) | status: O (empty body + capabilities verified from SPIR-V); I (why: waterwheel spawn happens CPU-side, this GPU pass is a stub; RDA/import reproduced via a folding construct)

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/draw_indirect.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/perlin.hlsl"

// The original frame_constants.hlsl carries an unbounded-array declaration that
// emits RuntimeDescriptorArray + SPV_EXT_descriptor_indexing even when dead.
// The reconstructed inc/ layer omits it; reproduce it here (dxc drops the decl).
RWStructuredBuffer<uint> _rda_trigger[] : register(u100, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 id : SV_DispatchThreadID)
{
    // Constant-folded intrinsic that preserves the original module's
    // GLSL.std.450 import (the shipped module is otherwise empty).
    float _dead = smoothstep(0.0f, 1.0f, 0.5f);
}
