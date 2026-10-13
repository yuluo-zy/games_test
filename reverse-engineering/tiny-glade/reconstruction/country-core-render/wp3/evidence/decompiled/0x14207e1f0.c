// address 0x14207e1f0
// name nani_update_uber
// signature void __cdecl nani_update_uber(void)

// WARNING: Type propagation algorithm not settling

void __cdecl country_core::render::nani::systems::nani_update_uber(void)

{
  byte *pbVar1;
  longlong lVar2;
  char *pcVar3;
  uint *puVar4;
  byte bVar5;
  byte bVar6;
  char cVar7;
  longlong lVar8;
  longlong lVar9;
  longlong lVar10;
  ulonglong uVar11;
  ulonglong *puVar12;
  uint uVar13;
  int iVar14;
  int iVar15;
  int iVar16;
  int iVar17;
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
  char cVar31;
  char cVar32;
  char cVar33;
  bool bVar34;
  char extraout_AL;
  char extraout_AL_00;
  bool bVar35;
  ushort uVar36;
  uint uVar37;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong lVar38;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *in_RCX;
  longlong *plVar39;
  longlong *in_RDX;
  ulonglong uVar40;
  ulonglong uVar41;
  undefined *puVar42;
  ulonglong uVar43;
  uint *puVar44;
  byte bVar45;
  undefined1 auVar46 [16];
  undefined1 unaff_XMM8_Ba;
  undefined1 unaff_XMM8_Bb;
  undefined1 unaff_XMM8_Bc;
  undefined1 unaff_XMM8_Bd;
  undefined1 unaff_XMM8_Be;
  undefined1 unaff_XMM8_Bf;
  undefined1 unaff_XMM8_Bg;
  undefined1 unaff_XMM8_Bh;
  undefined1 unaff_XMM8_Bi;
  undefined1 unaff_XMM8_Bj;
  undefined1 unaff_XMM8_Bk;
  undefined1 unaff_XMM8_Bl;
  undefined1 unaff_XMM8_Bm;
  undefined1 unaff_XMM8_Bn;
  undefined1 unaff_XMM8_Bo;
  undefined1 unaff_XMM8_Bp;
  char cVar47;
  char cVar48;
  char cVar49;
  char cVar50;
  undefined1 auVar51 [16];
  longlong local_110;
  ulonglong *local_f0;
  ulonglong local_c0;
  
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar34 = false;
  }
  else {
    uVar40 = 0x35;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar40 != 0) {
        if (uVar40 < 0x35) {
          if ((char)(&DAT_142fd3048)[uVar40] < -0x40) {
LAB_14207e297:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar40 != 0x35) goto LAB_14207e297;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar40 != 0xfffffffffffffffe)) {
        if (uVar40 + 2 < 0x35) {
          if ((char)(&UNK_142fd304a)[uVar40] < -0x40) {
LAB_14207e2f5:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar40 + 2 != 0x35) goto LAB_14207e2f5;
      }
    }
    puVar42 = &DAT_142fd345e;
    do {
      bVar45 = puVar42[-1];
      uVar37 = (uint)(char)bVar45;
      if ((char)bVar45 < '\0') {
        bVar5 = puVar42[-2];
        if ((char)bVar5 < -0x40) {
          bVar6 = puVar42[-3];
          if ((char)bVar6 < -0x40) {
            pbVar1 = puVar42 + -4;
            puVar42 = puVar42 + -4;
            uVar37 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar42 = puVar42 + -3;
            uVar37 = bVar6 & 0xf;
          }
          uVar37 = bVar5 & 0x3f | uVar37 << 6;
        }
        else {
          puVar42 = puVar42 + -2;
          uVar37 = bVar5 & 0x1f;
        }
        uVar37 = (uint)(bVar45 & 0x3f) | uVar37 << 6;
        if (uVar37 != 0x5c) goto LAB_14207e3b4;
LAB_14207e3c6:
        if (puVar42 == &UNK_142fd342f) break;
        if (puVar42 + -0x142fd342f < (undefined *)0x2e) {
          if (-0x41 < (char)puVar42[1]) break;
        }
        else if (puVar42 + -0x142fd342f == (undefined *)0x2e) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar42 = puVar42 + -1;
      if (uVar37 == 0x5c) goto LAB_14207e3c6;
LAB_14207e3b4:
      if (uVar37 == 0x2f) goto LAB_14207e3c6;
    } while (puVar42 != &DAT_142fd3430);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar39 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar39 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0))
    goto LAB_14207edcb;
    if (*plVar39 != 0) goto LAB_14207edd7;
    *plVar39 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar39 = *plVar39 + 1;
    bVar34 = true;
  }
  gpu_world::GpuWorld::owned_subset_mut();
  lVar8 = *in_RCX;
  lVar9 = in_RCX[1];
  puVar44 = *(uint **)(lVar8 + 0xf8);
  puVar4 = puVar44 + *(longlong *)(lVar8 + 0x100);
  cVar7 = *(char *)(lVar8 + 0x108);
  lVar10 = *in_RDX;
  local_110 = 8;
  uVar40 = 0;
  iVar14 = -(uint)(CONCAT13(unaff_XMM8_Bd,
                            CONCAT12(unaff_XMM8_Bc,CONCAT11(unaff_XMM8_Bb,unaff_XMM8_Ba))) ==
                  CONCAT13(unaff_XMM8_Bd,
                           CONCAT12(unaff_XMM8_Bc,CONCAT11(unaff_XMM8_Bb,unaff_XMM8_Ba))));
  iVar15 = -(uint)(CONCAT13(unaff_XMM8_Bh,
                            CONCAT12(unaff_XMM8_Bg,CONCAT11(unaff_XMM8_Bf,unaff_XMM8_Be))) ==
                  CONCAT13(unaff_XMM8_Bh,
                           CONCAT12(unaff_XMM8_Bg,CONCAT11(unaff_XMM8_Bf,unaff_XMM8_Be))));
  iVar16 = -(uint)(CONCAT13(unaff_XMM8_Bl,
                            CONCAT12(unaff_XMM8_Bk,CONCAT11(unaff_XMM8_Bj,unaff_XMM8_Bi))) ==
                  CONCAT13(unaff_XMM8_Bl,
                           CONCAT12(unaff_XMM8_Bk,CONCAT11(unaff_XMM8_Bj,unaff_XMM8_Bi))));
  iVar17 = -(uint)(CONCAT13(unaff_XMM8_Bp,
                            CONCAT12(unaff_XMM8_Bo,CONCAT11(unaff_XMM8_Bn,unaff_XMM8_Bm))) ==
                  CONCAT13(unaff_XMM8_Bp,
                           CONCAT12(unaff_XMM8_Bo,CONCAT11(unaff_XMM8_Bn,unaff_XMM8_Bm))));
  local_c0 = 0;
  local_f0 = (ulonglong *)0x0;
  bVar35 = false;
  if (cVar7 != '\0') goto LAB_14207e780;
LAB_14207e539:
  if (local_c0 != uVar40) goto LAB_14207e757;
  do {
    if (puVar44 == puVar4) goto LAB_14207ecd0;
    uVar37 = *puVar44;
    puVar44 = puVar44 + 1;
    uVar40 = *(ulonglong *)(*(longlong *)(lVar9 + 0x100) + 0x58 + (ulonglong)uVar37 * 0xa0);
  } while (uVar40 == 0);
  lVar38 = *(longlong *)(lVar9 + 0x100) + (ulonglong)uVar37 * 0xa0;
  lVar2 = *(longlong *)(lVar9 + 0x1a8) + (ulonglong)*(uint *)(lVar38 + 0x94) * 0x48;
  uVar41 = *(ulonglong *)
            (*(longlong *)(lVar9 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar38 + 0x94) * 0x48);
  uVar43 = *(ulonglong *)(lVar8 + 0x120);
  uVar11 = *(ulonglong *)(lVar38 + 0x88);
  if ((uVar43 < uVar11) && (*(longlong *)(*(longlong *)(lVar38 + 0x80) + uVar43 * 8) != 0)) {
    if ((uVar41 <= uVar43) || (*(longlong *)(*(longlong *)(lVar2 + 0x38) + uVar43 * 8) == 0)) {
      uVar43 = *(ulonglong *)(lVar8 + 0x128);
      goto joined_r0x00014207e6cc;
    }
    uVar43 = *(ulonglong *)(lVar8 + 0x128);
    if (uVar43 < uVar11) goto LAB_14207e6ee;
LAB_14207e735:
    bVar35 = false;
  }
  else {
    uVar43 = *(ulonglong *)(lVar8 + 0x128);
joined_r0x00014207e6cc:
    if (uVar11 <= uVar43) goto LAB_14207e735;
LAB_14207e6ee:
    if (*(longlong *)(*(longlong *)(lVar38 + 0x80) + uVar43 * 8) == 0) goto LAB_14207e735;
    bVar35 = true;
    if ((uVar43 < uVar41) &&
       (uVar41 = *(ulonglong *)(*(longlong *)(lVar2 + 0x38) + uVar43 * 8), uVar41 != 0)) {
      local_f0 = *(ulonglong **)(*(longlong *)(lVar2 + 0x18) + 0x10 + ~uVar41 * 0x30);
    }
    else {
      local_f0 = (ulonglong *)0x0;
    }
  }
  local_110 = *(longlong *)(lVar38 + 0x50);
  local_c0 = 0;
LAB_14207e757:
  uVar41 = (ulonglong)*(uint *)(local_110 + 8 + local_c0 * 0x10);
LAB_14207e829:
  if (!bVar35) goto LAB_14207eb22;
  puVar12 = local_f0 + uVar41;
joined_r0x00014207e875:
  if (puVar12 != (ulonglong *)0x0) {
    uVar41 = *puVar12;
    if (*(longlong *)(lVar10 + 0x48) != 0) {
      lVar2 = *(longlong *)(lVar10 + 0x30);
      uVar43 = *(ulonglong *)(lVar10 + 0x38);
      bVar45 = (byte)(uVar41 >> 0x39);
      auVar46 = ZEXT216(CONCAT11(bVar45,bVar45));
      auVar46 = pshuflw(auVar46,auVar46,0);
      cVar47 = auVar46[0];
      cVar48 = auVar46[1];
      cVar49 = auVar46[2];
      cVar50 = auVar46[3];
      lVar38 = 0;
      while( true ) {
        uVar41 = uVar43 & uVar41;
        pcVar3 = (char *)(lVar2 + uVar41);
        cVar18 = *pcVar3;
        cVar19 = pcVar3[1];
        cVar20 = pcVar3[2];
        cVar21 = pcVar3[3];
        cVar22 = pcVar3[4];
        cVar23 = pcVar3[5];
        cVar24 = pcVar3[6];
        cVar25 = pcVar3[7];
        cVar26 = pcVar3[8];
        cVar27 = pcVar3[9];
        cVar28 = pcVar3[10];
        cVar29 = pcVar3[0xb];
        cVar30 = pcVar3[0xc];
        cVar31 = pcVar3[0xd];
        cVar32 = pcVar3[0xe];
        cVar33 = pcVar3[0xf];
        auVar46[0] = -(cVar18 == cVar47);
        auVar46[1] = -(cVar19 == cVar48);
        auVar46[2] = -(cVar20 == cVar49);
        auVar46[3] = -(cVar21 == cVar50);
        auVar46[4] = -(cVar22 == cVar47);
        auVar46[5] = -(cVar23 == cVar48);
        auVar46[6] = -(cVar24 == cVar49);
        auVar46[7] = -(cVar25 == cVar50);
        auVar46[8] = -(cVar26 == cVar47);
        auVar46[9] = -(cVar27 == cVar48);
        auVar46[10] = -(cVar28 == cVar49);
        auVar46[0xb] = -(cVar29 == cVar50);
        auVar46[0xc] = -(cVar30 == cVar47);
        auVar46[0xd] = -(cVar31 == cVar48);
        auVar46[0xe] = -(cVar32 == cVar49);
        auVar46[0xf] = -(cVar33 == cVar50);
        uVar36 = (ushort)(SUB161(auVar46 >> 7,0) & 1) | (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar46 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar46 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar46 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar46 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar46 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar46 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar46 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar46 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar46 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar46 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar46 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe | (ushort)(auVar46[0xf] >> 7) << 0xf
        ;
        uVar37 = (uint)uVar36;
        while (uVar13 = uVar37, uVar36 != 0) {
          for (; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
          }
          ::utils::name_hash::impl$2::eq();
          if (extraout_AL_00 != '\0') {
            asset_libraries::texture_library::AssetTextureLibrary::handle_to_bindless_index();
            goto LAB_14207eb22;
          }
          uVar36 = (ushort)(uVar37 - 1) & (ushort)uVar37;
          uVar37 = CONCAT22((short)(uVar37 - 1 >> 0x10),uVar36);
        }
        auVar51[0] = -(cVar18 == (char)iVar14);
        auVar51[1] = -(cVar19 == (char)((uint)iVar14 >> 8));
        auVar51[2] = -(cVar20 == (char)((uint)iVar14 >> 0x10));
        auVar51[3] = -(cVar21 == (char)((uint)iVar14 >> 0x18));
        auVar51[4] = -(cVar22 == (char)iVar15);
        auVar51[5] = -(cVar23 == (char)((uint)iVar15 >> 8));
        auVar51[6] = -(cVar24 == (char)((uint)iVar15 >> 0x10));
        auVar51[7] = -(cVar25 == (char)((uint)iVar15 >> 0x18));
        auVar51[8] = -(cVar26 == (char)iVar16);
        auVar51[9] = -(cVar27 == (char)((uint)iVar16 >> 8));
        auVar51[10] = -(cVar28 == (char)((uint)iVar16 >> 0x10));
        auVar51[0xb] = -(cVar29 == (char)((uint)iVar16 >> 0x18));
        auVar51[0xc] = -(cVar30 == (char)iVar17);
        auVar51[0xd] = -(cVar31 == (char)((uint)iVar17 >> 8));
        auVar51[0xe] = -(cVar32 == (char)((uint)iVar17 >> 0x10));
        auVar51[0xf] = -(cVar33 == (char)((uint)iVar17 >> 0x18));
        if ((((((((((((((((SUB161(auVar51 >> 7,0) & 1) != 0 || (SUB161(auVar51 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar51 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar51 >> 0x1f,0) & 1) != 0) || (SUB161(auVar51 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar51 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar51 >> 0x37,0) & 1) != 0) || (SUB161(auVar51 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar51 >> 0x47,0) & 1) != 0) || (SUB161(auVar51 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar51 >> 0x57,0) & 1) != 0) || (SUB161(auVar51 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar51 >> 0x67,0) & 1) != 0) || (SUB161(auVar51 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar51 >> 0x77,0) & 1) != 0) || auVar51[0xf] < '\0') break;
        uVar41 = uVar41 + lVar38 + 0x10;
        lVar38 = lVar38 + 0x10;
      }
    }
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
LAB_14207eb22:
  do {
    gpu_world::OwnedGpuWorldSubset::
    update_instance_data<country_core::render::nani::instance_data::UberInstanceData>();
    local_c0 = local_c0 + 1;
    if (cVar7 == '\0') goto LAB_14207e539;
LAB_14207e780:
    if (local_c0 != uVar40) {
      uVar41 = local_c0 & 0xffffffff;
      goto LAB_14207e829;
    }
    do {
      if (puVar44 == puVar4) goto LAB_14207ecd0;
      uVar37 = *puVar44;
      puVar44 = puVar44 + 1;
      uVar40 = *(ulonglong *)(*(longlong *)(lVar9 + 0x1a8) + 0x10 + (ulonglong)uVar37 * 0x48);
    } while (uVar40 == 0);
    lVar2 = *(longlong *)(lVar9 + 0x1a8) + (ulonglong)uVar37 * 0x48;
    uVar41 = *(ulonglong *)(lVar2 + 0x40);
    if ((*(ulonglong *)(lVar8 + 0x110) < uVar41) &&
       (*(longlong *)(*(longlong *)(lVar2 + 0x38) + *(ulonglong *)(lVar8 + 0x110) * 8) != 0)) {
      uVar43 = *(ulonglong *)(lVar8 + 0x118);
      if (uVar41 <= uVar43) goto LAB_14207e9de;
LAB_14207e9a8:
      if (*(longlong *)(*(longlong *)(lVar2 + 0x38) + uVar43 * 8) == 0) goto LAB_14207e9de;
      uVar43 = *(ulonglong *)(lVar8 + 0x120);
      if (uVar43 < uVar41) goto LAB_14207e9f3;
LAB_14207ea3e:
      uVar43 = *(ulonglong *)(lVar8 + 0x128);
    }
    else {
      uVar43 = *(ulonglong *)(lVar8 + 0x118);
      if (uVar43 < uVar41) goto LAB_14207e9a8;
LAB_14207e9de:
      uVar43 = *(ulonglong *)(lVar8 + 0x120);
      if (uVar41 <= uVar43) goto LAB_14207ea3e;
LAB_14207e9f3:
      if (*(longlong *)(*(longlong *)(lVar2 + 0x38) + uVar43 * 8) == 0) goto LAB_14207ea3e;
      uVar43 = *(ulonglong *)(lVar8 + 0x128);
    }
    if ((uVar43 < uVar41) &&
       (uVar41 = *(ulonglong *)(*(longlong *)(lVar2 + 0x38) + uVar43 * 8), uVar41 != 0)) break;
    local_c0 = 0;
    bVar35 = false;
  } while( true );
  puVar12 = *(ulonglong **)(*(longlong *)(lVar2 + 0x18) + 0x10 + ~uVar41 * 0x30);
  bVar35 = true;
  local_c0 = 0;
  local_f0 = puVar12;
  goto joined_r0x00014207e875;
LAB_14207ecd0:
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  lVar8 = *extraout_RAX_02;
  if (lVar8 == 8) {
    *extraout_RAX_02 = 0;
  }
  UNLOCK();
  if (lVar8 != 8) {
    parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
  }
  if (bVar34) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_03 == 1) {
      plVar39 = extraout_RAX_03 + 1;
    }
    else if ((*extraout_RAX_03 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar39 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0)) {
LAB_14207edcb:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar39 != 0) {
LAB_14207edd7:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar39 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar39 = *plVar39 + 1;
  }
  return;
}

