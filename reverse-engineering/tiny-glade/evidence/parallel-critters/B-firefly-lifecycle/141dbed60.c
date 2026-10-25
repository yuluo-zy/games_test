
void __cdecl
alloc::vec::spec_from_iter::impl$0::
from_iter<country_core::systems::render_loop::resources::critters::FireflySsbo,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple_<ref_<utils::components::transform::Transform>,ref_<country_core::resources::critters::Firefly>_>,tuple_<>_>,system_critters::firefly::fireflies_send_to_gpu::closure_env_0>_>
          (void)

{
  longlong lVar1;
  longlong lVar2;
  undefined8 uVar3;
  longlong lVar4;
  undefined8 *extraout_RAX;
  ulonglong *in_RCX;
  longlong in_RDX;
  longlong lVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  int local_fc;
  undefined8 local_f8;
  undefined8 uStack_f0;
  uint local_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined4 uStack_d8;
  char local_78;
  ulonglong local_50;
  
  core::iter::adapters::map::impl$2::
  next<country_core::systems::render_loop::resources::critters::FireflySsbo,bevy_ecs::query::iter::QueryIter<tuple_<ref_<utils::components::transform::Transform>,ref_<country_core::resources::critters::Firefly>_>,tuple_<>_>,system_critters::firefly::fireflies_send_to_gpu::closure_env_0>
            ();
  if ((local_e8 & 1) == 0) {
    *in_RCX = 0;
    in_RCX[1] = 4;
    in_RCX[2] = 0;
    return;
  }
  lVar1 = *(longlong *)(in_RDX + 0x20);
  lVar2 = *(longlong *)(in_RDX + 0x28);
  if (*(char *)(in_RDX + 0x70) == '\0') {
    if (lVar1 != lVar2) {
      lVar5 = 0;
      uVar7 = 0;
      do {
        uVar6 = (ulonglong)*(uint *)(lVar1 + uVar7 * 4);
        if (*(ulonglong *)(*(longlong *)(in_RDX + 0x10) + 0x10) <= uVar6) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_bounds_check();
        }
        lVar5 = lVar5 + *(longlong *)
                         (*(longlong *)(*(longlong *)(in_RDX + 0x10) + 8) + 0x58 + uVar6 * 0xa0);
        uVar7 = uVar7 + 1;
      } while ((ulonglong)(lVar2 - lVar1) >> 2 != uVar7);
      goto LAB_141dbee57;
    }
  }
  else if (lVar1 != lVar2) {
    lVar5 = 0;
    uVar7 = 0;
    do {
      uVar6 = (ulonglong)*(uint *)(lVar1 + uVar7 * 4);
      if (*(ulonglong *)(*(longlong *)(in_RDX + 8) + 0x10) <= uVar6) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_bounds_check();
      }
      lVar5 = lVar5 + *(longlong *)
                       (*(longlong *)(*(longlong *)(in_RDX + 8) + 8) + 0x10 + uVar6 * 0x48);
      uVar7 = uVar7 + 1;
    } while ((ulonglong)(lVar2 - lVar1) >> 2 != uVar7);
    goto LAB_141dbee57;
  }
  lVar5 = 0;
LAB_141dbee57:
  uVar6 = ((lVar5 + *(longlong *)(in_RDX + 0x60)) - *(longlong *)(in_RDX + 0x68)) + 1;
  uVar7 = 0xffffffffffffffff;
  if (uVar6 != 0) {
    uVar7 = uVar6;
  }
  uVar6 = 4;
  if (4 < uVar7) {
    uVar6 = uVar7;
  }
  if (((uVar7 >> 0x3c == 0) && (uVar6 << 4 < 0x7ffffffffffffffd)) &&
     (std::alloc::__default_lib_allocator::__rust_alloc(), extraout_RAX != (undefined8 *)0x0)) {
    *extraout_RAX = CONCAT44(uStack_e0,uStack_e4);
    extraout_RAX[1] = CONCAT44(uStack_d8,uStack_dc);
    local_50 = 1;
    uVar3 = *(undefined8 *)(in_RDX + 0x70);
    lVar1 = *(longlong *)(in_RDX + 8);
    lVar2 = *(longlong *)(in_RDX + 0x10);
    lVar5 = *(longlong *)(in_RDX + 0x20);
    lVar4 = *(longlong *)(in_RDX + 0x28);
    do {
      core::iter::adapters::map::impl$2::
      next<country_core::systems::render_loop::resources::critters::FireflySsbo,bevy_ecs::query::iter::QueryIter<tuple_<ref_<utils::components::transform::Transform>,ref_<country_core::resources::critters::Firefly>_>,tuple_<>_>,system_critters::firefly::fireflies_send_to_gpu::closure_env_0>
                ();
      if (local_fc != 1) {
        in_RCX[2] = local_50;
        *in_RCX = uVar6;
        in_RCX[1] = (ulonglong)extraout_RAX;
        return;
      }
      if (local_50 == uVar6) {
        local_78 = (char)uVar3;
        if (local_78 == '\0') {
          if (lVar5 != lVar4) {
            uVar7 = 0;
            do {
              if (*(ulonglong *)(lVar2 + 0x10) <= (ulonglong)*(uint *)(lVar5 + uVar7 * 4)) {
                    /* WARNING: Subroutine does not return */
                core::panicking::panic_bounds_check();
              }
              uVar7 = uVar7 + 1;
            } while ((ulonglong)(lVar4 - lVar5) >> 2 != uVar7);
          }
        }
        else if (lVar5 != lVar4) {
          uVar7 = 0;
          do {
            if (*(ulonglong *)(lVar1 + 0x10) <= (ulonglong)*(uint *)(lVar5 + uVar7 * 4)) {
                    /* WARNING: Subroutine does not return */
              core::panicking::panic_bounds_check();
            }
            uVar7 = uVar7 + 1;
          } while ((ulonglong)(lVar4 - lVar5) >> 2 != uVar7);
        }
        raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      extraout_RAX[local_50 * 2] = local_f8;
      (extraout_RAX + local_50 * 2)[1] = uStack_f0;
      local_50 = local_50 + 1;
    } while( true );
  }
                    /* WARNING: Subroutine does not return */
  raw_vec::handle_error();
}

