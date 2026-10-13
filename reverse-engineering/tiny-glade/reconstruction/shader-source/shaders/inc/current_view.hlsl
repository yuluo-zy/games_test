// ==RECONSTRUCTED== shaders/inc/current_view.hlsl | evidence: raw-per-bin.jsonl `current_view_index` at $Globals member 0 in 178 bins (exactly the current_view.hlsl consumer set; e.g. _billboard_forest uses frame_constants.view_constants[_Globals_current_view_index].world_to_sample); include-graph edge current_view.hlsl -> frame_constants.hlsl | status: O (global current_view_index); O/I (GetCurrentViewConstants — accessor inferred from the inlined view_constants[current_view_index] access chains at use sites)
//
// Selects the active camera/view out of frame_constants.view_constants[5].
// `current_view_index` is a shader-global (lands in the per-shader $Globals UBO).

#ifndef TINYGLADE_INC_CURRENT_VIEW_HLSL
#define TINYGLADE_INC_CURRENT_VIEW_HLSL

#include "frame_constants.hlsl"

uint current_view_index;

// Current view matrices for the active camera.
ViewConstants GetCurrentViewConstants()
{
    return view_constants[current_view_index];
}

#endif // TINYGLADE_INC_CURRENT_VIEW_HLSL
