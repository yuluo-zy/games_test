
/* WARNING: Removing unreachable block (ram,0x000140a6c3b9) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::utils::geo::mesh_from_pts(void)

{
  int *piVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  int iVar14;
  int iVar15;
  int iVar16;
  int iVar17;
  int iVar18;
  int iVar19;
  int iVar20;
  int iVar21;
  ulonglong *extraout_RAX;
  longlong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  ulonglong *in_RCX;
  ulonglong uVar22;
  ulonglong in_RDX;
  ulonglong uVar23;
  ulonglong uVar24;
  ulonglong in_R8;
  longlong lVar25;
  longlong in_R9;
  int iVar26;
  int iVar27;
  int iVar28;
  int iVar29;
  longlong in_stack_00000028;
  ulonglong local_c0;
  ulonglong uStack_b8;
  ulonglong uStack_b0;
  
  uVar23 = in_stack_00000028 - in_R9;
  if (uVar23 == 0) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX == (ulonglong *)0x0) {
                    /* WARNING: Subroutine does not return */
    std::thread::local::panic_access_error();
  }
  uVar6 = *extraout_RAX;
  uVar7 = extraout_RAX[1];
  *extraout_RAX = *extraout_RAX + 1;
  uVar13 = _UNK_142ac64d8;
  uVar12 = _DAT_142ac64d0;
  uVar11 = _UNK_142ac64cc;
  uVar10 = _UNK_142ac64c8;
  uVar9 = anon_cc634b371bd8d6b6c1e8ab9dcbca5d5b_41_llvm_9402988993337875171._4_4_;
  uVar8 = anon_cc634b371bd8d6b6c1e8ab9dcbca5d5b_41_llvm_9402988993337875171._0_4_;
  if ((0x7ffffffffffffffc < uVar23) ||
     (std::alloc::__default_lib_allocator::__rust_alloc(), extraout_RAX_00 == 0)) {
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error();
  }
  uVar24 = (ulonglong)(in_stack_00000028 - in_R9) / 0xc;
  if (uVar23 - 0xc < 0xc) {
    uVar22 = 0;
  }
  else {
    lVar25 = 0x14;
    uVar22 = 0;
    do {
      uVar4 = *(undefined4 *)(in_R9 + -0xc + lVar25);
      *(undefined8 *)(extraout_RAX_00 + -0x14 + lVar25) = *(undefined8 *)(in_R9 + -0x14 + lVar25);
      *(undefined4 *)(extraout_RAX_00 + -0xc + lVar25) = uVar4;
      uVar4 = *(undefined4 *)(in_R9 + lVar25);
      puVar2 = (undefined4 *)(in_R9 + -8 + lVar25);
      uVar5 = puVar2[1];
      puVar3 = (undefined4 *)(extraout_RAX_00 + -8 + lVar25);
      *puVar3 = *puVar2;
      puVar3[1] = uVar5;
      *(undefined4 *)(extraout_RAX_00 + lVar25) = uVar4;
      uVar22 = uVar22 + 2;
      lVar25 = lVar25 + 0x18;
    } while ((uVar24 & 0xffffffffffffffe) != uVar22);
  }
  if ((uVar24 & 1) != 0) {
    lVar25 = uVar22 * 0xc;
    uVar4 = *(undefined4 *)(in_R9 + 8 + lVar25);
    uVar5 = ((undefined4 *)(in_R9 + lVar25))[1];
    *(undefined4 *)(extraout_RAX_00 + lVar25) = *(undefined4 *)(in_R9 + lVar25);
    ((undefined4 *)(extraout_RAX_00 + lVar25))[1] = uVar5;
    *(undefined4 *)(extraout_RAX_00 + 8 + lVar25) = uVar4;
  }
  country_core::render::mesh::Mesh::
  set_attribute<alloc::vec::Vec<array_<f32,3>,alloc::alloc::Global>_>();
  alloc::vec::from_elem<array_<f32,3>_>();
  country_core::render::mesh::Mesh::
  set_attribute<alloc::vec::Vec<array_<f32,3>,alloc::alloc::Global>_>();
  std::alloc::__default_lib_allocator::__rust_alloc();
  iVar21 = _UNK_142ad498c;
  iVar20 = _UNK_142ad4988;
  iVar19 = _UNK_142ad4984;
  iVar18 = ___xmm_00000008000000080000000800000008;
  iVar17 = _UNK_142ad497c;
  iVar16 = _UNK_142ad4978;
  iVar15 = _UNK_142ad4974;
  iVar14 = ___xmm_00000004000000040000000400000004;
  if (extraout_RAX_01 == 0) {
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error();
  }
  if (uVar23 < 0x60) {
    uVar23 = 0;
  }
  else {
    uVar23 = uVar24 & 0xffffffffffffff8;
    uVar22 = 0;
    iVar26 = ___xmm_00000003000000020000000100000000;
    iVar27 = _UNK_142ad4964;
    iVar28 = _UNK_142ad4968;
    iVar29 = _UNK_142ad496c;
    do {
      piVar1 = (int *)(extraout_RAX_01 + uVar22 * 4);
      *piVar1 = iVar26;
      piVar1[1] = iVar27;
      piVar1[2] = iVar28;
      piVar1[3] = iVar29;
      piVar1 = (int *)(extraout_RAX_01 + 0x10 + uVar22 * 4);
      *piVar1 = iVar26 + iVar14;
      piVar1[1] = iVar27 + iVar15;
      piVar1[2] = iVar28 + iVar16;
      piVar1[3] = iVar29 + iVar17;
      uVar22 = uVar22 + 8;
      iVar26 = iVar26 + iVar18;
      iVar27 = iVar27 + iVar19;
      iVar28 = iVar28 + iVar20;
      iVar29 = iVar29 + iVar21;
    } while (uVar23 != uVar22);
    if (uVar24 == uVar23) goto LAB_140a6c3b0;
  }
  do {
    *(int *)(extraout_RAX_01 + uVar23 * 4) = (int)uVar23;
    uVar23 = uVar23 + 1;
  } while (uVar24 != uVar23);
LAB_140a6c3b0:
  in_RCX[4] = local_c0;
  in_RCX[5] = uStack_b8;
  in_RCX[6] = uStack_b0;
  in_RCX[7] = CONCAT44(uVar9,uVar8);
  in_RCX[8] = CONCAT44(uVar11,uVar10);
  in_RCX[9] = uVar12;
  in_RCX[10] = uVar13;
  in_RCX[0xb] = uVar6;
  in_RCX[0xc] = uVar7;
  in_RCX[0xd] = in_RDX;
  in_RCX[0xe] = in_R8;
  *in_RCX = uVar24;
  in_RCX[1] = extraout_RAX_01;
  in_RCX[2] = uVar24;
  in_RCX[3] = 0x8000000000000000;
  return;
}

