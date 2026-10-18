
void __cdecl
country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_rectangle2d
          (void)

{
  ulonglong uVar1;
  longlong lVar2;
  longlong lVar3;
  ulonglong uVar4;
  longlong in_R8;
  ulonglong uVar5;
  undefined1 local_11;
  undefined1 *local_10;
  
  get_cells_from_rectangle2d<country_core::systems::wall::wall_state::wall_spatial_hash::impl_4::query_with_rectangle2d::closure_env_0>
            ();
  local_10 = &local_11;
  if (1 < *(ulonglong *)(in_R8 + 0x10)) {
    if (*(ulonglong *)(in_R8 + 0x10) < 0x15) {
      core::slice::sort::shared::smallsort::
      insertion_sort_shift_left<ref_<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment>,core::slice::impl_0::sort_unstable_by::closure_env_0<ref_<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment>,country_core::systems::wall::wall_state::wall_spatial_hash::impl_4::query_with_circle::closure_env_1>_>
                ();
      uVar1 = *(ulonglong *)(in_R8 + 0x10);
    }
    else {
      core::slice::sort::unstable::
      ipnsort<ref_<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment>,core::slice::impl_0::sort_unstable_by::closure_env_0<ref_<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment>,country_core::systems::wall::wall_state::wall_spatial_hash::impl_4::query_with_circle::closure_env_1>_>
                ();
      uVar1 = *(ulonglong *)(in_R8 + 0x10);
    }
    if (1 < uVar1) {
      lVar2 = *(longlong *)(in_R8 + 8);
      uVar4 = 1;
      do {
        uVar5 = uVar4;
        if (*(int *)(*(longlong *)(lVar2 + uVar4 * 8) + 0x30) ==
            *(int *)(*(longlong *)(lVar2 + -8 + uVar4 * 8) + 0x30)) {
          while (uVar5 = uVar5 + 1, uVar5 < uVar1) {
            lVar3 = *(longlong *)(lVar2 + uVar5 * 8);
            if (*(int *)(lVar3 + 0x30) != *(int *)(*(longlong *)(lVar2 + -8 + uVar4 * 8) + 0x30)) {
              *(longlong *)(lVar2 + uVar4 * 8) = lVar3;
              uVar4 = uVar4 + 1;
            }
          }
          *(ulonglong *)(in_R8 + 0x10) = uVar4;
          return;
        }
        uVar4 = uVar4 + 1;
      } while (uVar1 != uVar4);
    }
  }
  return;
}

