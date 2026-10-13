// address 0x14207c0a0
// name nani_update_cloth
// signature void __cdecl nani_update_cloth(void)

void __cdecl country_core::render::nani::systems::nani_update_cloth(void)

{
  byte *pbVar1;
  uint *puVar2;
  char *pcVar3;
  byte bVar4;
  byte bVar5;
  char cVar6;
  longlong lVar7;
  longlong lVar8;
  longlong lVar9;
  ulonglong uVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  int iVar14;
  char cVar15;
  char cVar16;
  char cVar17;
  char cVar18;
  char cVar19;
  char cVar20;
  char cVar21;
  char cVar22;
  char cVar23;
  char cVar24;
  char cVar25;
  char cVar26;
  char cVar27;
  char cVar28;
  char cVar29;
  char cVar30;
  bool bVar31;
  char extraout_AL;
  char extraout_AL_00;
  ushort uVar32;
  uint uVar33;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong lVar34;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *in_RCX;
  longlong *plVar35;
  longlong *in_RDX;
  ulonglong uVar36;
  undefined *puVar37;
  longlong lVar38;
  ulonglong uVar39;
  uint uVar40;
  ulonglong uVar41;
  uint *puVar42;
  byte bVar43;
  undefined1 auVar44 [16];
  undefined1 unaff_XMM6_Ba;
  undefined1 unaff_XMM6_Bb;
  undefined1 unaff_XMM6_Bc;
  undefined1 unaff_XMM6_Bd;
  undefined1 unaff_XMM6_Be;
  undefined1 unaff_XMM6_Bf;
  undefined1 unaff_XMM6_Bg;
  undefined1 unaff_XMM6_Bh;
  undefined1 unaff_XMM6_Bi;
  undefined1 unaff_XMM6_Bj;
  undefined1 unaff_XMM6_Bk;
  undefined1 unaff_XMM6_Bl;
  undefined1 unaff_XMM6_Bm;
  undefined1 unaff_XMM6_Bn;
  undefined1 unaff_XMM6_Bo;
  undefined1 unaff_XMM6_Bp;
  char cVar45;
  char cVar46;
  char cVar47;
  char cVar48;
  undefined1 auVar49 [16];
  longlong local_108;
  longlong local_d0;
  
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar31 = false;
  }
  else {
    uVar36 = 0x36;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar36 != 0) {
        if (uVar36 < 0x36) {
          if ((char)(&DAT_142fd30fc)[uVar36] < -0x40) {
LAB_14207c147:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar36 != 0x36) goto LAB_14207c147;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar36 != 0xfffffffffffffffe)) {
        if (uVar36 + 2 < 0x36) {
          if ((char)(&UNK_142fd30fe)[uVar36] < -0x40) {
LAB_14207c1a5:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar36 + 2 != 0x36) goto LAB_14207c1a5;
      }
    }
    puVar37 = &DAT_142fd345e;
    do {
      bVar43 = puVar37[-1];
      uVar33 = (uint)(char)bVar43;
      if ((char)bVar43 < '\0') {
        bVar4 = puVar37[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar37[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar37 + -4;
            puVar37 = puVar37 + -4;
            uVar33 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar37 = puVar37 + -3;
            uVar33 = bVar5 & 0xf;
          }
          uVar33 = bVar4 & 0x3f | uVar33 << 6;
        }
        else {
          puVar37 = puVar37 + -2;
          uVar33 = bVar4 & 0x1f;
        }
        uVar33 = (uint)(bVar43 & 0x3f) | uVar33 << 6;
        if (uVar33 != 0x5c) goto LAB_14207c264;
LAB_14207c276:
        if (puVar37 == &UNK_142fd342f) break;
        if (puVar37 + -0x142fd342f < (undefined *)0x2e) {
          if (-0x41 < (char)puVar37[1]) break;
        }
        else if (puVar37 + -0x142fd342f == (undefined *)0x2e) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar37 = puVar37 + -1;
      if (uVar33 == 0x5c) goto LAB_14207c276;
LAB_14207c264:
      if (uVar33 == 0x2f) goto LAB_14207c276;
    } while (puVar37 != &DAT_142fd3430);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar35 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar35 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0))
    goto LAB_14207caca;
    if (*plVar35 != 0) goto LAB_14207cad6;
    *plVar35 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar35 = *plVar35 + 1;
    bVar31 = true;
  }
  gpu_world::GpuWorld::owned_subset_mut();
  lVar7 = *in_RCX;
  lVar8 = in_RCX[1];
  puVar42 = *(uint **)(lVar7 + 0xf8);
  puVar2 = puVar42 + *(longlong *)(lVar7 + 0x100);
  cVar6 = *(char *)(lVar7 + 0x108);
  lVar9 = *in_RDX;
  local_108 = 8;
  iVar11 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                            CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                  CONCAT13(unaff_XMM6_Bd,
                           CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
  iVar12 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                            CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                  CONCAT13(unaff_XMM6_Bh,
                           CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
  iVar13 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                            CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                  CONCAT13(unaff_XMM6_Bl,
                           CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
  iVar14 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                            CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                  CONCAT13(unaff_XMM6_Bp,
                           CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
  lVar38 = 0;
  uVar36 = 0;
  uVar39 = 0;
  if (cVar6 != '\0') goto LAB_14207c530;
LAB_14207c3b0:
  if (uVar39 != uVar36) goto LAB_14207c512;
  do {
    if (puVar42 == puVar2) goto LAB_14207c9dd;
    uVar33 = *puVar42;
    puVar42 = puVar42 + 1;
    uVar36 = *(ulonglong *)(*(longlong *)(lVar8 + 0x100) + 0x58 + (ulonglong)uVar33 * 0xa0);
  } while (uVar36 == 0);
  lVar34 = *(longlong *)(lVar8 + 0x100) + (ulonglong)uVar33 * 0xa0;
  uVar39 = (ulonglong)*(uint *)(lVar34 + 0x94);
  lVar38 = *(longlong *)(lVar8 + 0x1a8) + uVar39 * 0x48;
  uVar39 = *(ulonglong *)(*(longlong *)(lVar8 + 0x1a8) + 0x40 + uVar39 * 0x48);
  if ((*(ulonglong *)(lVar7 + 0x110) < uVar39) &&
     (*(longlong *)(*(longlong *)(lVar38 + 0x38) + *(ulonglong *)(lVar7 + 0x110) * 8) != 0)) {
    uVar41 = *(ulonglong *)(lVar7 + 0x118);
    if (uVar39 <= uVar41) goto LAB_14207c482;
LAB_14207c453:
    if (*(longlong *)(*(longlong *)(lVar38 + 0x38) + uVar41 * 8) == 0) goto LAB_14207c482;
    uVar41 = *(ulonglong *)(lVar7 + 0x120);
    if (uVar41 < uVar39) goto LAB_14207c491;
LAB_14207c4c7:
    uVar41 = *(ulonglong *)(lVar7 + 0x128);
    if (uVar39 <= uVar41) goto LAB_14207c501;
LAB_14207c4de:
    uVar39 = *(ulonglong *)(*(longlong *)(lVar38 + 0x38) + uVar41 * 8);
    if (uVar39 == 0) goto LAB_14207c501;
    lVar38 = *(longlong *)(*(longlong *)(lVar38 + 0x18) + 0x10 + ~uVar39 * 0x30);
  }
  else {
    uVar41 = *(ulonglong *)(lVar7 + 0x118);
    if (uVar41 < uVar39) goto LAB_14207c453;
LAB_14207c482:
    uVar41 = *(ulonglong *)(lVar7 + 0x120);
    if (uVar39 <= uVar41) goto LAB_14207c4c7;
LAB_14207c491:
    if (*(longlong *)(*(longlong *)(lVar38 + 0x38) + uVar41 * 8) == 0) goto LAB_14207c4c7;
    uVar41 = *(ulonglong *)(lVar7 + 0x128);
    if (uVar41 < uVar39) goto LAB_14207c4de;
LAB_14207c501:
    lVar38 = 0;
  }
  local_108 = *(longlong *)(lVar34 + 0x50);
  uVar39 = 0;
LAB_14207c512:
  uVar41 = (ulonglong)*(uint *)(local_108 + 8 + uVar39 * 0x10);
  while (uVar41 = *(ulonglong *)(lVar38 + uVar41 * 8), *(longlong *)(lVar9 + 0x48) != 0) {
    uVar39 = uVar39 + 1;
    lVar34 = *(longlong *)(lVar9 + 0x30);
    uVar10 = *(ulonglong *)(lVar9 + 0x38);
    bVar43 = (byte)(uVar41 >> 0x39);
    auVar44 = ZEXT216(CONCAT11(bVar43,bVar43));
    auVar44 = pshuflw(auVar44,auVar44,0);
    cVar45 = auVar44[0];
    cVar46 = auVar44[1];
    cVar47 = auVar44[2];
    cVar48 = auVar44[3];
    local_d0 = 0;
    while( true ) {
      uVar41 = uVar10 & uVar41;
      pcVar3 = (char *)(lVar34 + uVar41);
      cVar15 = *pcVar3;
      cVar16 = pcVar3[1];
      cVar17 = pcVar3[2];
      cVar18 = pcVar3[3];
      cVar19 = pcVar3[4];
      cVar20 = pcVar3[5];
      cVar21 = pcVar3[6];
      cVar22 = pcVar3[7];
      cVar23 = pcVar3[8];
      cVar24 = pcVar3[9];
      cVar25 = pcVar3[10];
      cVar26 = pcVar3[0xb];
      cVar27 = pcVar3[0xc];
      cVar28 = pcVar3[0xd];
      cVar29 = pcVar3[0xe];
      cVar30 = pcVar3[0xf];
      auVar44[0] = -(cVar15 == cVar45);
      auVar44[1] = -(cVar16 == cVar46);
      auVar44[2] = -(cVar17 == cVar47);
      auVar44[3] = -(cVar18 == cVar48);
      auVar44[4] = -(cVar19 == cVar45);
      auVar44[5] = -(cVar20 == cVar46);
      auVar44[6] = -(cVar21 == cVar47);
      auVar44[7] = -(cVar22 == cVar48);
      auVar44[8] = -(cVar23 == cVar45);
      auVar44[9] = -(cVar24 == cVar46);
      auVar44[10] = -(cVar25 == cVar47);
      auVar44[0xb] = -(cVar26 == cVar48);
      auVar44[0xc] = -(cVar27 == cVar45);
      auVar44[0xd] = -(cVar28 == cVar46);
      auVar44[0xe] = -(cVar29 == cVar47);
      auVar44[0xf] = -(cVar30 == cVar48);
      uVar32 = (ushort)(SUB161(auVar44 >> 7,0) & 1) | (ushort)(SUB161(auVar44 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar44 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar44 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar44 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar44 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar44 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar44 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar44 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar44 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar44 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar44 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar44 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar44 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar44 >> 0x77,0) & 1) << 0xe | (ushort)(auVar44[0xf] >> 7) << 0xf;
      uVar33 = (uint)uVar32;
      uVar40 = uVar33;
      if (uVar32 != 0) break;
LAB_14207c7b0:
      auVar49[0] = -(cVar15 == (char)iVar11);
      auVar49[1] = -(cVar16 == (char)((uint)iVar11 >> 8));
      auVar49[2] = -(cVar17 == (char)((uint)iVar11 >> 0x10));
      auVar49[3] = -(cVar18 == (char)((uint)iVar11 >> 0x18));
      auVar49[4] = -(cVar19 == (char)iVar12);
      auVar49[5] = -(cVar20 == (char)((uint)iVar12 >> 8));
      auVar49[6] = -(cVar21 == (char)((uint)iVar12 >> 0x10));
      auVar49[7] = -(cVar22 == (char)((uint)iVar12 >> 0x18));
      auVar49[8] = -(cVar23 == (char)iVar13);
      auVar49[9] = -(cVar24 == (char)((uint)iVar13 >> 8));
      auVar49[10] = -(cVar25 == (char)((uint)iVar13 >> 0x10));
      auVar49[0xb] = -(cVar26 == (char)((uint)iVar13 >> 0x18));
      auVar49[0xc] = -(cVar27 == (char)iVar14);
      auVar49[0xd] = -(cVar28 == (char)((uint)iVar14 >> 8));
      auVar49[0xe] = -(cVar29 == (char)((uint)iVar14 >> 0x10));
      auVar49[0xf] = -(cVar30 == (char)((uint)iVar14 >> 0x18));
      if ((((((((((((((((SUB161(auVar49 >> 7,0) & 1) != 0 || (SUB161(auVar49 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar49 >> 0x17,0) & 1) != 0) || (SUB161(auVar49 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar49 >> 0x27,0) & 1) != 0) || (SUB161(auVar49 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar49 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar49 >> 0x3f,0) & 1) != 0) || (SUB161(auVar49 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar49 >> 0x4f,0) & 1) != 0) || (SUB161(auVar49 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar49 >> 0x5f,0) & 1) != 0) || (SUB161(auVar49 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar49 >> 0x6f,0) & 1) != 0) || (SUB161(auVar49 >> 0x77,0) & 1) != 0) ||
          auVar49[0xf] < '\0') goto LAB_14207c9cf;
      uVar41 = uVar41 + local_d0 + 0x10;
      local_d0 = local_d0 + 0x10;
    }
    while( true ) {
      for (; (uVar33 & 1) == 0; uVar33 = uVar33 >> 1 | 0x80000000) {
      }
      ::utils::name_hash::impl$2::eq();
      if (extraout_AL_00 != '\0') break;
      uVar32 = (ushort)(uVar40 - 1) & (ushort)uVar40;
      uVar33 = CONCAT22((short)(uVar40 - 1 >> 0x10),uVar32);
      uVar40 = uVar33;
      if (uVar32 == 0) goto LAB_14207c7b0;
    }
    asset_libraries::texture_library::AssetTextureLibrary::handle_to_bindless_index();
    gpu_world::OwnedGpuWorldSubset::
    update_instance_data<country_core::render::nani::instance_data::ClothInstanceData>();
    if (cVar6 == '\0') goto LAB_14207c3b0;
LAB_14207c530:
    if (uVar39 == uVar36) {
      while (puVar42 != puVar2) {
        uVar33 = *puVar42;
        puVar42 = puVar42 + 1;
        uVar36 = *(ulonglong *)(*(longlong *)(lVar8 + 0x1a8) + 0x10 + (ulonglong)uVar33 * 0x48);
        if (uVar36 != 0) {
          lVar38 = *(longlong *)(lVar8 + 0x1a8) + (ulonglong)uVar33 * 0x48;
          uVar39 = *(ulonglong *)(lVar38 + 0x40);
          if ((*(ulonglong *)(lVar7 + 0x110) < uVar39) &&
             (*(longlong *)(*(longlong *)(lVar38 + 0x38) + *(ulonglong *)(lVar7 + 0x110) * 8) != 0))
          {
            uVar41 = *(ulonglong *)(lVar7 + 0x118);
            if (uVar39 <= uVar41) goto LAB_14207c5e9;
LAB_14207c5ba:
            if (*(longlong *)(*(longlong *)(lVar38 + 0x38) + uVar41 * 8) == 0) goto LAB_14207c5e9;
            uVar41 = *(ulonglong *)(lVar7 + 0x120);
            if (uVar41 < uVar39) goto LAB_14207c5f8;
LAB_14207c62e:
            uVar41 = *(ulonglong *)(lVar7 + 0x128);
            if (uVar39 <= uVar41) goto LAB_14207c668;
LAB_14207c645:
            uVar39 = *(ulonglong *)(*(longlong *)(lVar38 + 0x38) + uVar41 * 8);
            if (uVar39 == 0) goto LAB_14207c668;
            lVar38 = *(longlong *)(*(longlong *)(lVar38 + 0x18) + 0x10 + ~uVar39 * 0x30);
          }
          else {
            uVar41 = *(ulonglong *)(lVar7 + 0x118);
            if (uVar41 < uVar39) goto LAB_14207c5ba;
LAB_14207c5e9:
            uVar41 = *(ulonglong *)(lVar7 + 0x120);
            if (uVar39 <= uVar41) goto LAB_14207c62e;
LAB_14207c5f8:
            if (*(longlong *)(*(longlong *)(lVar38 + 0x38) + uVar41 * 8) == 0) goto LAB_14207c62e;
            uVar41 = *(ulonglong *)(lVar7 + 0x128);
            if (uVar41 < uVar39) goto LAB_14207c645;
LAB_14207c668:
            lVar38 = 0;
          }
          uVar39 = 0;
          goto LAB_14207c671;
        }
      }
      goto LAB_14207c9dd;
    }
LAB_14207c671:
    uVar41 = uVar39 & 0xffffffff;
  }
LAB_14207c9cf:
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
LAB_14207c9dd:
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  lVar7 = *extraout_RAX_02;
  if (lVar7 == 8) {
    *extraout_RAX_02 = 0;
  }
  UNLOCK();
  if (lVar7 != 8) {
    parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
  }
  if (bVar31) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_03 == 1) {
      plVar35 = extraout_RAX_03 + 1;
    }
    else if ((*extraout_RAX_03 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar35 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0)) {
LAB_14207caca:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar35 != 0) {
LAB_14207cad6:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar35 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar35 = *plVar35 + 1;
  }
  return;
}

