
void __cdecl
alloc::vec::spec_from_iter::impl$0::
from_iter<u64,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple_<bevy_ecs::entity::Entity,ref_<country_core::components::AutoClutter>,enum2_<core::option::Option<ref_<system_clutter::autoclutter::WindowShutter>_>_>,ref_<utils::components::transform::Transform>_>,bevy_ecs::query::filter::Without<country_core::resources::clutter_meshes::ClutterMarkedForDeletion>_>,system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure_env_4>_>
          (void)

{
  longlong lVar1;
  longlong lVar2;
  longlong extraout_RAX;
  undefined1 *puVar3;
  undefined8 *extraout_RAX_00;
  longlong extraout_RAX_01;
  undefined8 *in_RCX;
  ulonglong uVar4;
  undefined1 *in_RDX;
  undefined1 *puVar5;
  longlong lVar6;
  undefined1 *puVar7;
  ulonglong uVar8;
  undefined8 *puVar9;
  undefined1 local_f0 [8];
  longlong local_e8;
  longlong local_e0;
  longlong local_d0;
  longlong local_c8;
  char local_70;
  undefined1 *local_60;
  undefined8 *puStack_58;
  undefined1 *local_50;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  puVar5 = in_RDX;
  core::iter::adapters::map::impl$2::
  next<u64,bevy_ecs::query::iter::QueryIter<tuple_<bevy_ecs::entity::Entity,ref_<country_core::components::AutoClutter>,enum2_<core::option::Option<ref_<system_clutter::autoclutter::WindowShutter>_>_>,ref_<utils::components::transform::Transform>_>,bevy_ecs::query::filter::Without<country_core::resources::clutter_meshes::ClutterMarkedForDeletion>_>,system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure_env_4>
            ();
  if (extraout_RAX == 0) {
    *in_RCX = 0;
    in_RCX[1] = 8;
    in_RCX[2] = 0;
    return;
  }
  lVar1 = *(longlong *)(in_RDX + 0x20);
  lVar2 = *(longlong *)(in_RDX + 0x28);
  if (in_RDX[0x80] == '\0') {
    if (lVar1 != lVar2) {
      lVar6 = 0;
      uVar8 = 0;
      do {
        uVar4 = (ulonglong)*(uint *)(lVar1 + uVar8 * 4);
        if (*(ulonglong *)(*(longlong *)(in_RDX + 0x10) + 0x10) <= uVar4) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        lVar6 = lVar6 + *(longlong *)
                         (*(longlong *)(*(longlong *)(in_RDX + 0x10) + 8) + 0x58 + uVar4 * 0xa0);
        uVar8 = uVar8 + 1;
      } while ((ulonglong)(lVar2 - lVar1) >> 2 != uVar8);
      goto LAB_141ceabc7;
    }
  }
  else if (lVar1 != lVar2) {
    lVar6 = 0;
    uVar8 = 0;
    do {
      uVar4 = (ulonglong)*(uint *)(lVar1 + uVar8 * 4);
      if (*(ulonglong *)(*(longlong *)(in_RDX + 8) + 0x10) <= uVar4) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      lVar6 = lVar6 + *(longlong *)
                       (*(longlong *)(*(longlong *)(in_RDX + 8) + 8) + 0x10 + uVar4 * 0x48);
      uVar8 = uVar8 + 1;
    } while ((ulonglong)(lVar2 - lVar1) >> 2 != uVar8);
    goto LAB_141ceabc7;
  }
  lVar6 = 0;
LAB_141ceabc7:
  puVar7 = (undefined1 *)
           (((lVar6 + *(longlong *)(in_RDX + 0x50)) - *(longlong *)(in_RDX + 0x58)) + 1);
  puVar3 = (undefined1 *)0xffffffffffffffff;
  if (puVar7 != (undefined1 *)0x0) {
    puVar3 = puVar7;
  }
  puVar7 = &DAT_00000004;
  if (&DAT_00000004 < puVar3) {
    puVar7 = puVar3;
  }
  if ((((ulonglong)puVar3 >> 0x3d == 0) && ((ulonglong)((longlong)puVar7 * 8) < 0x7ffffffffffffff9))
     && (std::alloc::__default_lib_allocator::__rust_alloc(), extraout_RAX_00 != (undefined8 *)0x0))
  {
    *extraout_RAX_00 = puVar5;
    local_50 = (undefined1 *)0x1;
    local_60 = puVar7;
    puStack_58 = extraout_RAX_00;
    memcpy(local_f0,in_RDX,0x88);
    puVar5 = (undefined1 *)0x1;
    puVar9 = extraout_RAX_00;
    do {
      core::iter::adapters::map::impl$2::
      next<u64,bevy_ecs::query::iter::QueryIter<tuple_<bevy_ecs::entity::Entity,ref_<country_core::components::AutoClutter>,enum2_<core::option::Option<ref_<system_clutter::autoclutter::WindowShutter>_>_>,ref_<utils::components::transform::Transform>_>,bevy_ecs::query::filter::Without<country_core::resources::clutter_meshes::ClutterMarkedForDeletion>_>,system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure_env_4>
                ();
      if (extraout_RAX_01 != 1) {
        in_RCX[2] = local_50;
        *in_RCX = local_60;
        in_RCX[1] = puStack_58;
        return;
      }
      puVar3 = in_RDX;
      if (puVar5 == local_60) {
        if (local_70 == '\0') {
          if (local_d0 != local_c8) {
            uVar8 = 0;
            do {
              if (*(ulonglong *)(local_e0 + 0x10) <= (ulonglong)*(uint *)(local_d0 + uVar8 * 4)) {
                    // WARNING: Subroutine does not return
                core::panicking::panic_bounds_check();
              }
              uVar8 = uVar8 + 1;
            } while ((ulonglong)(local_c8 - local_d0) >> 2 != uVar8);
          }
        }
        else if (local_d0 != local_c8) {
          uVar8 = 0;
          do {
            if (*(ulonglong *)(local_e8 + 0x10) <= (ulonglong)*(uint *)(local_d0 + uVar8 * 4)) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            uVar8 = uVar8 + 1;
          } while ((ulonglong)(local_c8 - local_d0) >> 2 != uVar8);
        }
        puVar3 = puVar5;
        raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        puVar9 = puStack_58;
      }
      puVar9[(longlong)puVar5] = in_RDX;
      puVar5 = puVar5 + 1;
      in_RDX = puVar3;
      local_50 = puVar5;
    } while( true );
  }
                    // WARNING: Subroutine does not return
  raw_vec::handle_error();
}

