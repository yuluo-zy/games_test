
/* WARNING: Removing unreachable block (ram,0x000141243f76) */

void __cdecl
system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::
emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env_2>
          (void)

{
  undefined4 *extraout_RAX;
  undefined4 *extraout_RAX_00;
  undefined4 *puVar1;
  longlong in_RCX;
  ulonglong *puVar2;
  ulonglong uVar3;
  ulonglong *in_R9;
  ulonglong uVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  longlong local_110;
  ulonglong *local_108;
  longlong local_100;
  longlong local_f8;
  ulonglong local_98;
  
  uVar4 = in_R9[1];
  if (uVar4 == *in_R9) {
    return;
  }
  if (uVar4 == 0xffffffffffffffff) {
                    /* WARNING: Subroutine does not return */
    core::slice::index::slice_end_index_overflow_fail();
  }
  uVar5 = uVar4 + 1;
  uVar6 = *in_R9;
  if ((char)in_R9[2] != '\0') {
    uVar6 = uVar5;
  }
  uVar3 = uVar5 - uVar6;
  if (uVar5 < uVar6) {
                    /* WARNING: Subroutine does not return */
    core::slice::index::slice_index_order_fail();
  }
  if (*(ulonglong *)(in_RCX + 8) <= uVar4) {
                    /* WARNING: Subroutine does not return */
    core::slice::index::slice_end_index_len_fail();
  }
  if (*(ulonglong *)(in_RCX + 0x18) <= uVar4) {
                    /* WARNING: Subroutine does not return */
    core::slice::index::slice_end_index_len_fail();
  }
  if (*(ulonglong *)(in_RCX + 0x28) <= uVar4) {
                    /* WARNING: Subroutine does not return */
    core::slice::index::slice_end_index_len_fail();
  }
  if (uVar3 < 2) {
    return;
  }
  ::utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>();
  if (local_f8 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  country_core::resources::walls::public_wall_state::PublicWallState::u_space();
  if ((local_100 == 0) || (local_100 == 1)) goto LAB_1412440d6;
  uVar4 = local_108[1];
  if (uVar4 == 0xffffffffffffffff) {
LAB_141244168:
                    /* WARNING: Subroutine does not return */
    core::slice::index::slice_end_index_overflow_fail();
  }
  uVar5 = *local_108;
  if (uVar4 + 1 < uVar5) {
LAB_14124418d:
                    /* WARNING: Subroutine does not return */
    core::slice::index::slice_index_order_fail();
  }
  if (uVar3 <= uVar4) {
LAB_141244176:
                    /* WARNING: Subroutine does not return */
    core::slice::index::slice_end_index_len_fail();
  }
  if (uVar4 < uVar5) {
    uVar5 = 0;
  }
  else {
    if (uVar4 - uVar5 == -1) {
LAB_141243dab:
                    /* WARNING: Subroutine does not return */
      core::option::expect_failed();
    }
    uVar5 = (uVar4 - uVar5) + 1;
  }
  uVar6 = uVar5 * 4;
  if ((uVar5 >> 0x3e != 0) || (0x7ffffffffffffffc < uVar6)) {
LAB_1412441b1:
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error();
  }
  if (uVar6 == 0) {
    puVar1 = (undefined4 *)&DAT_00000004;
    local_98 = 0;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    puVar1 = extraout_RAX;
    local_98 = uVar5;
    if (extraout_RAX == (undefined4 *)0x0) goto LAB_1412441b1;
  }
  if (uVar5 < 2) {
    if (uVar5 != 0) goto LAB_141243e79;
  }
  else {
    memset(puVar1,0,uVar6 - 4);
    puVar1 = puVar1 + (uVar5 - 1);
LAB_141243e79:
    *puVar1 = 0;
  }
  arch_walker::ArchWalker::
  walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env_0<system_wall_constructor::construct_lintels::construct_lintels::closure_env_2>_>
            ();
  puVar2 = local_108 + 2;
  if (local_98 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  for (; puVar2 != local_108 + local_100; puVar2 = puVar2 + 1) {
    uVar5 = *puVar2;
    if (uVar5 == 0xffffffffffffffff) goto LAB_141244168;
    if (uVar5 + 1 < uVar4) goto LAB_14124418d;
    if (uVar3 <= uVar5) goto LAB_141244176;
    if (uVar5 < uVar4) {
      uVar6 = 0;
      uVar4 = 0;
    }
    else {
      if (uVar5 - uVar4 == -1) goto LAB_141243dab;
      uVar6 = (uVar5 - uVar4) + 1;
      uVar4 = uVar6 * 4;
      if (uVar6 >> 0x3e != 0) goto LAB_1412441b1;
    }
    if (0x7ffffffffffffffc < uVar4) goto LAB_1412441b1;
    if (uVar4 == 0) {
      puVar1 = (undefined4 *)&DAT_00000004;
      local_98 = 0;
      if (1 < uVar6) goto LAB_141243fe9;
LAB_14124401f:
      if (uVar6 != 0) goto LAB_14124402b;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_00 == (undefined4 *)0x0) goto LAB_1412441b1;
      puVar1 = extraout_RAX_00;
      local_98 = uVar6;
      if (uVar6 < 2) goto LAB_14124401f;
LAB_141243fe9:
      memset(puVar1,0,uVar4 - 4);
      puVar1 = (undefined4 *)((longlong)puVar1 + (uVar4 - 4));
LAB_14124402b:
      *puVar1 = 0;
    }
    arch_walker::ArchWalker::
    walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env_0<system_wall_constructor::construct_lintels::construct_lintels::closure_env_2>_>
              ();
    if (local_98 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    uVar4 = uVar5;
  }
LAB_1412440d6:
  if (local_110 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

