// addr=0x140a6c120 name=mesh_from_pts
// signature=void __cdecl mesh_from_pts(void)


// WARNING: Removing unreachable block (ram,0x000140a6c3b9)

void __cdecl country_core::utils::geo::mesh_from_pts(void)

{
  int *piVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  ulonglong *extraout_RAX;
  longlong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  ulonglong *in_RCX;
  ulonglong uVar8;
  ulonglong in_RDX;
  ulonglong uVar9;
  ulonglong uVar10;
  ulonglong in_R8;
  longlong lVar11;
  longlong in_R9;
  int iVar12;
  int iVar13;
  int iVar14;
  int iVar15;
  longlong in_stack_00000028;
  ulonglong local_c0;
  ulonglong uStack_b8;
  ulonglong uStack_b0;
  
  uVar9 = in_stack_00000028 - in_R9;
  if (uVar9 == 0) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX == (ulonglong *)0x0) {
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  uVar6 = *extraout_RAX;
  uVar7 = extraout_RAX[1];
  *extraout_RAX = *extraout_RAX + 1;
  if ((0x7ffffffffffffffc < uVar9) ||
     (std::alloc::__default_lib_allocator::__rust_alloc(), extraout_RAX_00 == 0)) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  uVar10 = (ulonglong)(in_stack_00000028 - in_R9) / 0xc;
  if (uVar9 - 0xc < 0xc) {
    uVar8 = 0;
  }
  else {
    lVar11 = 0x14;
    uVar8 = 0;
    do {
      uVar4 = *(undefined4 *)(in_R9 + -0xc + lVar11);
      *(undefined8 *)(extraout_RAX_00 + -0x14 + lVar11) = *(undefined8 *)(in_R9 + -0x14 + lVar11);
      *(undefined4 *)(extraout_RAX_00 + -0xc + lVar11) = uVar4;
      uVar4 = *(undefined4 *)(in_R9 + lVar11);
      puVar2 = (undefined4 *)(in_R9 + -8 + lVar11);
      uVar5 = puVar2[1];
      puVar3 = (undefined4 *)(extraout_RAX_00 + -8 + lVar11);
      *puVar3 = *puVar2;
      puVar3[1] = uVar5;
      *(undefined4 *)(extraout_RAX_00 + lVar11) = uVar4;
      uVar8 = uVar8 + 2;
      lVar11 = lVar11 + 0x18;
    } while ((uVar10 & 0xffffffffffffffe) != uVar8);
  }
  if ((uVar10 & 1) != 0) {
    lVar11 = uVar8 * 0xc;
    uVar4 = *(undefined4 *)(in_R9 + 8 + lVar11);
    uVar5 = ((undefined4 *)(in_R9 + lVar11))[1];
    *(undefined4 *)(extraout_RAX_00 + lVar11) = *(undefined4 *)(in_R9 + lVar11);
    ((undefined4 *)(extraout_RAX_00 + lVar11))[1] = uVar5;
    *(undefined4 *)(extraout_RAX_00 + 8 + lVar11) = uVar4;
  }
  country_core::render::mesh::Mesh::
  set_attribute<alloc::vec::Vec<array_<f32,3>,alloc::alloc::Global>_>();
  alloc::vec::from_elem<array_<f32,3>_>();
  country_core::render::mesh::Mesh::
  set_attribute<alloc::vec::Vec<array_<f32,3>,alloc::alloc::Global>_>();
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_01 == 0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  if (uVar9 < 0x60) {
    uVar9 = 0;
  }
  else {
    uVar9 = uVar10 & 0xffffffffffffff8;
    iVar12 = 0;
    iVar13 = 1;
    iVar14 = 2;
    iVar15 = 3;
    uVar8 = 0;
    do {
      piVar1 = (int *)(extraout_RAX_01 + uVar8 * 4);
      *piVar1 = iVar12;
      piVar1[1] = iVar13;
      piVar1[2] = iVar14;
      piVar1[3] = iVar15;
      piVar1 = (int *)(extraout_RAX_01 + 0x10 + uVar8 * 4);
      *piVar1 = iVar12 + 4;
      piVar1[1] = iVar13 + 4;
      piVar1[2] = iVar14 + 4;
      piVar1[3] = iVar15 + 4;
      uVar8 = uVar8 + 8;
      iVar12 = iVar12 + 8;
      iVar13 = iVar13 + 8;
      iVar14 = iVar14 + 8;
      iVar15 = iVar15 + 8;
    } while (uVar9 != uVar8);
    if (uVar10 == uVar9) goto LAB_140a6c3cc;
  }
  do {
    *(int *)(extraout_RAX_01 + uVar9 * 4) = (int)uVar9;
    uVar9 = uVar9 + 1;
  } while (uVar10 != uVar9);
LAB_140a6c3cc:
  in_RCX[4] = local_c0;
  in_RCX[5] = uStack_b8;
  in_RCX[6] = uStack_b0;
  in_RCX[7] = (ulonglong)&anon_cc634b371bd8d6b6c1e8ab9dcbca5d5b_40_llvm_9402988993337875171;
  in_RCX[8] = 0;
  in_RCX[9] = 0;
  in_RCX[10] = 0;
  in_RCX[0xb] = uVar6;
  in_RCX[0xc] = uVar7;
  in_RCX[0xd] = in_RDX;
  in_RCX[0xe] = in_R8;
  *in_RCX = uVar10;
  in_RCX[1] = extraout_RAX_01;
  in_RCX[2] = uVar10;
  in_RCX[3] = 0x8000000000000000;
  return;
}


