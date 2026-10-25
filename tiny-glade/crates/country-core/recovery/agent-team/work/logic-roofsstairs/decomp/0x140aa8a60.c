
void __cdecl country_core::resources::stairs::visual::StairsVisualState::reset_edge(void)

{
  char *pcVar1;
  undefined8 *puVar2;
  undefined8 uVar3;
  undefined8 *puVar4;
  undefined8 uVar5;
  undefined1 (*pauVar6) [16];
  uint uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  undefined8 uVar19;
  longlong *plVar20;
  ulonglong extraout_RAX;
  int *piVar21;
  longlong lVar22;
  longlong *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  longlong *extraout_RAX_03;
  int iVar23;
  int iVar24;
  undefined8 *in_RCX;
  uint uVar25;
  longlong *in_RDX;
  ulonglong uVar26;
  ulonglong uVar27;
  byte bVar28;
  int in_R8D;
  uint uVar29;
  longlong lVar30;
  int in_R9D;
  ushort uVar31;
  longlong lVar32;
  undefined1 (*pauVar33) [16];
  longlong lVar34;
  bool bVar35;
  byte bVar36;
  char cVar37;
  char cVar39;
  char cVar40;
  char cVar41;
  undefined1 auVar38 [16];
  undefined1 auVar42 [16];
  undefined1 auVar43 [16];
  longlong in_stack_00000030;
  longlong in_stack_00000040;
  undefined4 uStack_c4;
  undefined4 uStack_6c;
  
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::resources::stairs::stairs_state::StairGraphEdge>_>
            ();
  lVar22 = *in_RDX;
  bVar28 = (byte)(extraout_RAX >> 0x38);
  bVar36 = bVar28 >> 1;
  auVar38 = ZEXT216(CONCAT11(bVar36,bVar36));
  auVar38 = pshuflw(auVar38,auVar38,0);
  lVar30 = 0;
  uVar26 = extraout_RAX;
  while( true ) {
    uVar26 = in_RDX[1] & uVar26;
    pcVar1 = (char *)(lVar22 + uVar26);
    cVar37 = auVar38[0];
    auVar43[0] = -(*pcVar1 == cVar37);
    cVar39 = auVar38[1];
    auVar43[1] = -(pcVar1[1] == cVar39);
    cVar40 = auVar38[2];
    auVar43[2] = -(pcVar1[2] == cVar40);
    cVar41 = auVar38[3];
    auVar43[3] = -(pcVar1[3] == cVar41);
    auVar43[4] = -(pcVar1[4] == cVar37);
    auVar43[5] = -(pcVar1[5] == cVar39);
    auVar43[6] = -(pcVar1[6] == cVar40);
    auVar43[7] = -(pcVar1[7] == cVar41);
    auVar43[8] = -(pcVar1[8] == cVar37);
    auVar43[9] = -(pcVar1[9] == cVar39);
    auVar43[10] = -(pcVar1[10] == cVar40);
    auVar43[0xb] = -(pcVar1[0xb] == cVar41);
    auVar43[0xc] = -(pcVar1[0xc] == cVar37);
    auVar43[0xd] = -(pcVar1[0xd] == cVar39);
    auVar43[0xe] = -(pcVar1[0xe] == cVar40);
    auVar43[0xf] = -(pcVar1[0xf] == cVar41);
    uVar31 = (ushort)(SUB161(auVar43 >> 7,0) & 1) | (ushort)(SUB161(auVar43 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar43 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar43 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar43 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar43 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar43 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar43 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar43 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar43 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar43 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar43 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar43 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar43 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar43 >> 0x77,0) & 1) << 0xe | (ushort)(auVar43[0xf] >> 7) << 0xf;
    uVar29 = (uint)uVar31;
    while (uVar31 != 0) {
      uVar25 = 0;
      for (uVar7 = uVar29; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
        uVar25 = uVar25 + 1;
      }
      lVar32 = (uVar25 + uVar26 & in_RDX[1]) * -0x110;
      if ((*(int *)(lVar22 + -0x110 + lVar32) == in_R8D) &&
         (*(int *)(lVar22 + -0x10c + lVar32) == in_R9D)) {
        pauVar33 = (undefined1 (*) [16])(lVar22 + lVar32);
        lVar22 = *(longlong *)pauVar33[-3];
        goto joined_r0x000140aa8b81;
      }
      uVar31 = (ushort)(uVar29 - 1) & (ushort)uVar29;
      uVar29 = CONCAT22((short)(uVar29 - 1 >> 0x10),uVar31);
    }
    auVar42[0] = -(*pcVar1 == -1);
    auVar42[1] = -(pcVar1[1] == -1);
    auVar42[2] = -(pcVar1[2] == -1);
    auVar42[3] = -(pcVar1[3] == -1);
    auVar42[4] = -(pcVar1[4] == -1);
    auVar42[5] = -(pcVar1[5] == -1);
    auVar42[6] = -(pcVar1[6] == -1);
    auVar42[7] = -(pcVar1[7] == -1);
    auVar42[8] = -(pcVar1[8] == -1);
    auVar42[9] = -(pcVar1[9] == -1);
    auVar42[10] = -(pcVar1[10] == -1);
    auVar42[0xb] = -(pcVar1[0xb] == -1);
    auVar42[0xc] = -(pcVar1[0xc] == -1);
    auVar42[0xd] = -(pcVar1[0xd] == -1);
    auVar42[0xe] = -(pcVar1[0xe] == -1);
    auVar42[0xf] = -(pcVar1[0xf] == -1);
    if ((((((((((((((((SUB161(auVar42 >> 7,0) & 1) != 0 || (SUB161(auVar42 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar42 >> 0x17,0) & 1) != 0) || (SUB161(auVar42 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar42 >> 0x27,0) & 1) != 0) || (SUB161(auVar42 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar42 >> 0x37,0) & 1) != 0) || (SUB161(auVar42 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar42 >> 0x47,0) & 1) != 0) || (SUB161(auVar42 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar42 >> 0x57,0) & 1) != 0) || (SUB161(auVar42 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar42 >> 0x67,0) & 1) != 0) || (SUB161(auVar42 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar42 >> 0x77,0) & 1) != 0) || auVar42[0xf] < '\0') break;
    uVar26 = uVar26 + lVar30 + 0x10;
    lVar30 = lVar30 + 0x10;
  }
  if (in_RDX[2] == 0) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::visual::StairsEdgeVisualState>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::visual::StairsEdgeVisualState,std::hash::random::RandomState>_>
              ();
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    plVar20 = extraout_RAX_03;
  }
  else {
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    plVar20 = extraout_RAX_00;
  }
  if (plVar20 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  lVar22 = *plVar20;
  lVar30 = plVar20[1];
  *plVar20 = *plVar20 + 1;
  pauVar6 = (undefined1 (*) [16])*in_RDX;
  uVar26 = in_RDX[1];
  uVar27 = extraout_RAX & uVar26;
  auVar38 = *(undefined1 (*) [16])(*pauVar6 + uVar27);
  uVar31 = (ushort)(SUB161(auVar38 >> 7,0) & 1) | (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe | (ushort)(byte)(auVar38[0xf] >> 7) << 0xf
  ;
  uVar29 = (uint)uVar31;
  if (uVar31 == 0) {
    lVar32 = 0x10;
    do {
      uVar27 = uVar27 + lVar32 & uVar26;
      auVar38 = *(undefined1 (*) [16])(*pauVar6 + uVar27);
      lVar32 = lVar32 + 0x10;
      uVar29 = (uint)(ushort)((ushort)(SUB161(auVar38 >> 7,0) & 1) |
                              (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1 |
                              (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
                              (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
                              (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
                              (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
                              (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
                              (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
                              (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
                              (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
                              (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
                              (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
                              (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
                              (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
                              (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe |
                             (ushort)(byte)(auVar38[0xf] >> 7) << 0xf);
    } while (uVar29 == 0);
  }
  uVar25 = 0;
  for (; (uVar29 & 1) == 0; uVar29 = uVar29 >> 1 | 0x80000000) {
    uVar25 = uVar25 + 1;
  }
  uVar27 = uVar25 + uVar27 & uVar26;
  bVar36 = (*pauVar6)[uVar27];
  if (-1 < (char)bVar36) {
    auVar38 = *pauVar6;
    uVar29 = 0;
    for (uVar25 = (uint)(ushort)((ushort)(SUB161(auVar38 >> 7,0) & 1) |
                                 (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1 |
                                 (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
                                 (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
                                 (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
                                 (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
                                 (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
                                 (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
                                 (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
                                 (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
                                 (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
                                 (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
                                 (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
                                 (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
                                 (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe |
                                (ushort)(byte)(auVar38[0xf] >> 7) << 0xf); (uVar25 & 1) == 0;
        uVar25 = uVar25 >> 1 | 0x80000000) {
      uVar29 = uVar29 + 1;
    }
    uVar27 = (ulonglong)uVar29;
    bVar36 = (*pauVar6)[uVar27];
  }
  bVar28 = bVar28 >> 1;
  (*pauVar6)[uVar27] = bVar28;
  pauVar6[1][uVar27 - 0x10 & uVar26] = bVar28;
  in_RDX[2] = in_RDX[2] - (ulonglong)(bVar36 & 1);
  pauVar33 = pauVar6 + uVar27 * -0x11;
  *(int *)pauVar6[uVar27 * -0x11 + -0x11] = in_R8D;
  *(int *)(pauVar6[uVar27 * -0x11 + -0x11] + 4) = in_R9D;
  *(undefined8 *)(pauVar6[uVar27 * -0x11 + -0x11] + 8) = 0;
  *(undefined8 *)pauVar6[uVar27 * -0x11 + -0x10] = 8;
  *(undefined1 (*) [16])(pauVar6[uVar27 * -0x11 + -0x10] + 8) = (undefined1  [16])0x0;
  *(undefined8 *)(pauVar6[uVar27 * -0x11 + -0xf] + 8) = 0x10;
  pauVar6[uVar27 * -0x11 + -0xe] = (undefined1  [16])0x0;
  *(undefined8 *)pauVar6[uVar27 * -0x11 + -0xd] = 8;
  *(undefined1 (*) [16])(pauVar6[uVar27 * -0x11 + -0xd] + 8) = (undefined1  [16])0x0;
  *(undefined8 *)(pauVar6[uVar27 * -0x11 + -0xc] + 8) = 0x10;
  *(undefined8 *)pauVar6[uVar27 * -0x11 + -0xb] = 0;
  *(undefined4 *)(pauVar6[uVar27 * -0x11 + -0xb] + 8) = 0;
  *(undefined8 *)pauVar6[uVar27 * -0x11 + -10] = 0;
  *(undefined8 *)pauVar6[uVar27 * -0x11 + -9] = 0;
  pauVar6[uVar27 * -0x11 + -8][0] = 0;
  *(undefined8 *)(pauVar6[uVar27 * -0x11 + -8] + 8) = 0;
  *(undefined8 *)pauVar6[uVar27 * -0x11 + -7] = 4;
  *(undefined8 *)(pauVar6[uVar27 * -0x11 + -7] + 8) = 0;
  *(undefined **)pauVar6[uVar27 * -0x11 + -6] =
       &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
  *(undefined8 *)(pauVar6[uVar27 * -0x11 + -6] + 8) = 0;
  *(undefined8 *)pauVar6[uVar27 * -0x11 + -5] = 0;
  *(undefined8 *)(pauVar6[uVar27 * -0x11 + -5] + 8) = 0;
  *(longlong *)pauVar6[uVar27 * -0x11 + -4] = lVar22;
  *(longlong *)(pauVar6[uVar27 * -0x11 + -4] + 8) = lVar30;
  *(undefined8 *)pauVar6[uVar27 * -0x11 + -3] = 0;
  *(undefined8 *)pauVar6[uVar27 * -0x11 + -2] = 0;
  *(undefined8 *)pauVar6[uVar27 * -0x11 + -1] = 0;
  *(undefined2 *)(pauVar6[uVar27 * -0x11 + -1] + 8) = 0;
  in_RDX[3] = in_RDX[3] + 1;
  lVar22 = *(longlong *)pauVar33[-3];
joined_r0x000140aa8b81:
  if (lVar22 == 0) {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_01 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX_01 = 0;
    extraout_RAX_01[1] = 4;
    extraout_RAX_01[2] = 0;
    *(undefined1 *)(extraout_RAX_01 + 3) = 0;
    lVar22 = *(longlong *)(in_stack_00000030 + 0x60);
    do {
      lVar30 = *(longlong *)(lVar22 + 8);
      while (lVar30 != -1) {
        if (lVar30 < 0) {
                    // WARNING: Subroutine does not return
          alloc::sync::impl$21::downgrade::panic_cold_display<ref_<str_>_>();
        }
        LOCK();
        lVar32 = *(longlong *)(lVar22 + 8);
        bVar35 = lVar30 == lVar32;
        if (bVar35) {
          *(longlong *)(lVar22 + 8) = lVar30 + 1;
          lVar32 = lVar30;
        }
        UNLOCK();
        lVar30 = lVar32;
        if (bVar35) {
          *(longlong *)pauVar33[-3] = lVar22;
          *(undefined8 **)(pauVar33[-3] + 8) = extraout_RAX_01;
          puVar2 = extraout_RAX_01;
          goto joined_r0x000140aa9106;
        }
      }
    } while( true );
  }
  puVar2 = *(undefined8 **)(pauVar33[-3] + 8);
joined_r0x000140aa9106:
  if (puVar2 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  LOCK();
  bVar35 = *(char *)(puVar2 + 3) == '\0';
  if (bVar35) {
    *(char *)(puVar2 + 3) = '\x01';
  }
  UNLOCK();
  if (!bVar35) {
    parking_lot::raw_mutex::RawMutex::lock_slow();
  }
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  piVar21 = (int *)(puVar2[2] * 4 + puVar2[1] + -4);
  if (piVar21 == (int *)0x0 || puVar2[2] == 0) {
    lVar22 = *(longlong *)(in_stack_00000030 + 0x28);
    if (lVar22 == 0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    iVar23 = (int)lVar22 + -1;
  }
  else {
    iVar23 = *piVar21;
    lVar22 = *(longlong *)(in_stack_00000030 + 0x28);
  }
  uVar3 = *(undefined8 *)(in_stack_00000030 + 0x20);
  uVar8 = *(undefined8 *)(in_stack_00000030 + 0x30);
  uVar9 = *(undefined8 *)(in_stack_00000030 + 0x38);
  uVar10 = *(undefined8 *)(in_stack_00000030 + 0x40);
  uVar11 = *(undefined8 *)(in_stack_00000030 + 0x48);
  uVar12 = *(undefined8 *)(in_stack_00000030 + 0x50);
  uVar13 = *(undefined8 *)(in_stack_00000030 + 0x58);
  country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
  clear<country_core::geometry::instanced_wall::BrickTransformSsbo>();
  MagicSupports::clear();
  if (*(longlong *)pauVar33[-2] == 0) {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_02 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX_02 = 0;
    extraout_RAX_02[1] = 4;
    extraout_RAX_02[2] = 0;
    *(undefined1 *)(extraout_RAX_02 + 3) = 0;
    lVar30 = *(longlong *)(in_stack_00000040 + 0x60);
    do {
      lVar32 = *(longlong *)(lVar30 + 8);
      while (lVar32 != -1) {
        if (lVar32 < 0) {
                    // WARNING: Subroutine does not return
          alloc::sync::impl$21::downgrade::panic_cold_display<ref_<str_>_>();
        }
        LOCK();
        lVar34 = *(longlong *)(lVar30 + 8);
        bVar35 = lVar32 == lVar34;
        if (bVar35) {
          *(longlong *)(lVar30 + 8) = lVar32 + 1;
          lVar34 = lVar32;
        }
        UNLOCK();
        lVar32 = lVar34;
        if (bVar35) {
          *(longlong *)pauVar33[-2] = lVar30;
          *(undefined8 **)(pauVar33[-2] + 8) = extraout_RAX_02;
          puVar4 = extraout_RAX_02;
          goto joined_r0x000140aa9192;
        }
      }
    } while( true );
  }
  puVar4 = *(undefined8 **)(pauVar33[-2] + 8);
joined_r0x000140aa9192:
  if (puVar4 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  LOCK();
  bVar35 = *(char *)(puVar4 + 3) == '\0';
  if (bVar35) {
    *(char *)(puVar4 + 3) = '\x01';
  }
  UNLOCK();
  if (!bVar35) {
    parking_lot::raw_mutex::RawMutex::lock_slow();
  }
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  piVar21 = (int *)(puVar4[2] * 4 + puVar4[1] + -4);
  if (piVar21 == (int *)0x0 || puVar4[2] == 0) {
    lVar30 = *(longlong *)(in_stack_00000040 + 0x28);
    if (lVar30 == 0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    iVar24 = (int)lVar30 + -1;
  }
  else {
    iVar24 = *piVar21;
    lVar30 = *(longlong *)(in_stack_00000040 + 0x28);
  }
  uVar5 = *(undefined8 *)(in_stack_00000040 + 0x20);
  uVar14 = *(undefined8 *)(in_stack_00000040 + 0x30);
  uVar15 = *(undefined8 *)(in_stack_00000040 + 0x38);
  uVar16 = *(undefined8 *)(in_stack_00000040 + 0x40);
  uVar17 = *(undefined8 *)(in_stack_00000040 + 0x48);
  uVar18 = *(undefined8 *)(in_stack_00000040 + 0x50);
  uVar19 = *(undefined8 *)(in_stack_00000040 + 0x58);
  country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
  clear<country_core::geometry::instanced_wall::BrickTransformSsbo>();
  lVar32 = *(longlong *)pauVar33[-0x10];
  lVar34 = *(longlong *)(pauVar33[-0x10] + 8);
  *(undefined8 *)(pauVar33[-0x10] + 8) = 0;
  if (lVar34 != 0) {
    lVar32 = lVar32 + 8;
    lVar34 = lVar34 << 3;
    do {
      if (*(ulonglong *)(lVar32 + -8) >> 0x20 == 0) break;
      bevy_ecs::system::commands::EntityCommands::despawn();
      lVar32 = lVar32 + 8;
      lVar34 = lVar34 + -8;
    } while (lVar34 != 0);
  }
  *(undefined8 *)pauVar33[-0xe] = 0;
  *(undefined8 *)(pauVar33[-0xd] + 8) = 0;
  if (*(longlong *)(pauVar33[-5] + 8) != 0) {
    uVar26 = *(ulonglong *)(pauVar33[-6] + 8);
    if (uVar26 != 0) {
      memset(*(void **)pauVar33[-6],(int)CONCAT71((int7)((ulonglong)lVar32 >> 8),0xff),uVar26 + 0x11
            );
    }
    *(undefined8 *)(pauVar33[-5] + 8) = 0;
    uVar27 = (uVar26 + 1 & 0xfffffffffffffff8) - (uVar26 + 1 >> 3);
    if (uVar26 < 8) {
      uVar27 = uVar26;
    }
    *(ulonglong *)pauVar33[-5] = uVar27;
  }
  *(undefined8 *)(pauVar33[-7] + 8) = 0;
  *(undefined2 *)(pauVar33[-1] + 8) = 0;
  *(undefined8 *)pauVar33[-1] = 0;
  in_RCX[10] = CONCAT44(uStack_c4,iVar23);
  in_RCX[8] = uVar12;
  in_RCX[9] = uVar13;
  in_RCX[6] = uVar11;
  in_RCX[7] = in_stack_00000030;
  in_RCX[4] = puVar2;
  in_RCX[5] = uVar10;
  in_RCX[2] = uVar8;
  in_RCX[3] = uVar9;
  *in_RCX = uVar3;
  in_RCX[1] = lVar22;
  in_RCX[0xb] = uVar5;
  in_RCX[0xc] = lVar30;
  in_RCX[0xd] = uVar14;
  in_RCX[0xe] = uVar15;
  in_RCX[0xf] = puVar4;
  in_RCX[0x10] = uVar16;
  in_RCX[0x11] = uVar17;
  in_RCX[0x12] = in_stack_00000040;
  in_RCX[0x13] = uVar18;
  in_RCX[0x14] = uVar19;
  in_RCX[0x15] = CONCAT44(uStack_6c,iVar24);
  in_RCX[0x16] = pauVar33[-0x11] + 8;
  in_RCX[0x17] = pauVar33 + -6;
  in_RCX[0x18] = pauVar33 + -0xf;
  in_RCX[0x19] = pauVar33[-0xe] + 8;
  return;
}

