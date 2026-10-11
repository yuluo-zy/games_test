
void __cdecl
alloc::vec::spec_from_iter::impl$0::
from_iter<country_core::resources::roof_tile_ssbo::RoofSpireSsbo,core::iter::adapters::filter_map::FilterMap<bevy_ecs::query::iter::QueryIter<tuple_<ref_<country_core::resources::roofs::roof_shape::Roof>,ref_<country_core::resources::roofs::roof_animation::RoofAnimation>_>,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>_>,system_roof::visual::place_spires::place_roof_spires::closure_env_0>_>
          (void)

{
  longlong lVar1;
  longlong lVar2;
  undefined8 uVar3;
  longlong lVar4;
  longlong lVar5;
  undefined8 *extraout_RAX;
  undefined8 *in_RCX;
  longlong in_RDX;
  ulonglong uVar6;
  byte local_108;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  ulonglong local_d8;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  char local_68;
  longlong local_50;
  
  core::iter::adapters::filter_map::impl$2::
  next<country_core::resources::roof_tile_ssbo::RoofSpireSsbo,bevy_ecs::query::iter::QueryIter<tuple_<ref_<country_core::resources::roofs::roof_shape::Roof>,ref_<country_core::resources::roofs::roof_animation::RoofAnimation>_>,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>_>,system_roof::visual::place_spires::place_roof_spires::closure_env_0>
            ();
  if ((local_d8 & 1) == 0) {
    *in_RCX = 0;
    in_RCX[1] = 0x10;
    in_RCX[2] = 0;
  }
  else {
    lVar1 = *(longlong *)(in_RDX + 0x20);
    lVar2 = *(longlong *)(in_RDX + 0x28);
    if (*(char *)(in_RDX + 0x70) == '\0') {
      if (lVar1 != lVar2) {
        uVar6 = 0;
        do {
          if (*(ulonglong *)(*(longlong *)(in_RDX + 0x10) + 0x10) <=
              (ulonglong)*(uint *)(lVar1 + uVar6 * 4)) {
                    /* WARNING: Subroutine does not return */
            core::panicking::panic_bounds_check();
          }
          uVar6 = uVar6 + 1;
        } while ((ulonglong)(lVar2 - lVar1) >> 2 != uVar6);
      }
    }
    else if (lVar1 != lVar2) {
      uVar6 = 0;
      do {
        if (*(ulonglong *)(*(longlong *)(in_RDX + 8) + 0x10) <=
            (ulonglong)*(uint *)(lVar1 + uVar6 * 4)) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_bounds_check();
        }
        uVar6 = uVar6 + 1;
      } while ((ulonglong)(lVar2 - lVar1) >> 2 != uVar6);
    }
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      raw_vec::handle_error();
    }
    extraout_RAX[2] = local_b8;
    extraout_RAX[3] = uStack_b0;
    *extraout_RAX = local_c8;
    extraout_RAX[1] = uStack_c0;
    local_50 = 1;
    uVar3 = *(undefined8 *)(in_RDX + 0x70);
    lVar1 = *(longlong *)(in_RDX + 8);
    lVar2 = *(longlong *)(in_RDX + 0x10);
    lVar4 = *(longlong *)(in_RDX + 0x20);
    lVar5 = *(longlong *)(in_RDX + 0x28);
    while (core::iter::adapters::filter_map::impl$2::
           next<country_core::resources::roof_tile_ssbo::RoofSpireSsbo,bevy_ecs::query::iter::QueryIter<tuple_<ref_<country_core::resources::roofs::roof_shape::Roof>,ref_<country_core::resources::roofs::roof_animation::RoofAnimation>_>,bevy_ecs::query::filter::With<system_roof::utils::components::RoofSpire>_>,system_roof::visual::place_spires::place_roof_spires::closure_env_0>
                     (), (local_108 & 1) != 0) {
      if (local_50 == 4) {
        local_68 = (char)uVar3;
        if (local_68 == '\0') {
          if (lVar4 != lVar5) {
            uVar6 = 0;
            do {
              if (*(ulonglong *)(lVar2 + 0x10) <= (ulonglong)*(uint *)(lVar4 + uVar6 * 4)) {
                    /* WARNING: Subroutine does not return */
                core::panicking::panic_bounds_check();
              }
              uVar6 = uVar6 + 1;
            } while ((ulonglong)(lVar5 - lVar4) >> 2 != uVar6);
          }
        }
        else if (lVar4 != lVar5) {
          uVar6 = 0;
          do {
            if (*(ulonglong *)(lVar1 + 0x10) <= (ulonglong)*(uint *)(lVar4 + uVar6 * 4)) {
                    /* WARNING: Subroutine does not return */
              core::panicking::panic_bounds_check();
            }
            uVar6 = uVar6 + 1;
          } while ((ulonglong)(lVar5 - lVar4) >> 2 != uVar6);
        }
        raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      extraout_RAX[local_50 * 4 + 2] = local_e8;
      (extraout_RAX + local_50 * 4 + 2)[1] = uStack_e0;
      extraout_RAX[local_50 * 4] = local_f8;
      (extraout_RAX + local_50 * 4)[1] = uStack_f0;
      local_50 = local_50 + 1;
    }
    in_RCX[2] = local_50;
    *in_RCX = 4;
    in_RCX[1] = extraout_RAX;
  }
  return;
}

