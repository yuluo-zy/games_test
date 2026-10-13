// address 0x142077d20
// name nani_update_curved_halftimber
// signature void __cdecl nani_update_curved_halftimber(void)

void __cdecl country_core::render::nani::systems::nani_update_curved_halftimber(void)

{
  byte *pbVar1;
  uint *puVar2;
  char *pcVar3;
  byte bVar4;
  byte bVar5;
  char cVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  longlong lVar9;
  longlong lVar10;
  longlong lVar11;
  longlong lVar12;
  longlong lVar13;
  longlong lVar14;
  ulonglong uVar15;
  uint uVar16;
  uint uVar17;
  int iVar18;
  int iVar19;
  int iVar20;
  int iVar21;
  bool bVar22;
  char extraout_AL;
  ushort uVar23;
  uint uVar24;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong lVar25;
  ulonglong uVar26;
  ulonglong *puVar27;
  ulonglong extraout_RAX_03;
  longlong lVar28;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *in_RCX;
  longlong lVar29;
  longlong *plVar30;
  ulonglong uVar31;
  ulonglong uVar32;
  longlong *in_R8;
  undefined *puVar33;
  longlong *in_R9;
  longlong lVar34;
  ulonglong uVar35;
  ulonglong uVar36;
  uint *puVar37;
  undefined4 *puVar38;
  byte bVar39;
  char cVar40;
  char cVar42;
  char cVar43;
  char cVar44;
  undefined1 auVar41 [16];
  undefined1 auVar45 [16];
  undefined1 auVar46 [16];
  undefined1 unaff_XMM7_Ba;
  undefined1 unaff_XMM7_Bb;
  undefined1 unaff_XMM7_Bc;
  undefined1 unaff_XMM7_Bd;
  undefined1 unaff_XMM7_Be;
  undefined1 unaff_XMM7_Bf;
  undefined1 unaff_XMM7_Bg;
  undefined1 unaff_XMM7_Bh;
  undefined1 unaff_XMM7_Bi;
  undefined1 unaff_XMM7_Bj;
  undefined1 unaff_XMM7_Bk;
  undefined1 unaff_XMM7_Bl;
  undefined1 unaff_XMM7_Bm;
  undefined1 unaff_XMM7_Bn;
  undefined1 unaff_XMM7_Bo;
  undefined1 unaff_XMM7_Bp;
  longlong *in_stack_00000028;
  longlong local_108;
  longlong local_e0;
  longlong local_c0;
  longlong local_98;
  longlong local_78;
  
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar22 = false;
  }
  else {
    uVar31 = 0x42;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar31 != 0) {
        if (uVar31 < 0x42) {
          if ((char)(&DAT_142fd3245)[uVar31] < -0x40) {
LAB_142077da7:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar31 != 0x42) goto LAB_142077da7;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar31 != 0xfffffffffffffffe)) {
        if (uVar31 + 2 < 0x42) {
          if ((char)(&UNK_142fd3247)[uVar31] < -0x40) {
LAB_142077e05:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar31 + 2 != 0x42) goto LAB_142077e05;
      }
    }
    puVar33 = &DAT_142fd345e;
    do {
      bVar39 = puVar33[-1];
      uVar24 = (uint)(char)bVar39;
      if ((char)bVar39 < '\0') {
        bVar4 = puVar33[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar33[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar33 + -4;
            puVar33 = puVar33 + -4;
            uVar24 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar33 = puVar33 + -3;
            uVar24 = bVar5 & 0xf;
          }
          uVar24 = bVar4 & 0x3f | uVar24 << 6;
        }
        else {
          puVar33 = puVar33 + -2;
          uVar24 = bVar4 & 0x1f;
        }
        uVar24 = (uint)(bVar39 & 0x3f) | uVar24 << 6;
        if (uVar24 != 0x5c) goto LAB_142077ec4;
LAB_142077ed5:
        if (puVar33 == &UNK_142fd342f) break;
        if (puVar33 + -0x142fd342f < (undefined *)0x2e) {
          if (-0x41 < (char)puVar33[1]) break;
        }
        else if (puVar33 + -0x142fd342f == (undefined *)0x2e) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar33 = puVar33 + -1;
      if (uVar24 == 0x5c) goto LAB_142077ed5;
LAB_142077ec4:
      if (uVar24 == 0x2f) goto LAB_142077ed5;
    } while (puVar33 != &DAT_142fd3430);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar30 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar30 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0))
    goto LAB_1420785cb;
    if (*plVar30 != 0) goto LAB_1420785d7;
    *plVar30 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar30 = *plVar30 + 1;
    bVar22 = true;
  }
  gpu_world::GpuWorld::owned_subset_mut();
  lVar9 = *in_RCX;
  lVar10 = in_RCX[1];
  uVar7 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar37 = *(uint **)(lVar9 + 0xf8);
  puVar2 = puVar37 + *(longlong *)(lVar9 + 0x100);
  cVar6 = *(char *)(lVar9 + 0x108);
  lVar11 = *in_R9;
  lVar12 = *in_R8;
  lVar13 = *in_stack_00000028;
  lVar14 = in_stack_00000028[1];
  local_108 = 8;
  local_98 = 0;
  iVar18 = -(uint)(CONCAT13(unaff_XMM7_Bd,
                            CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))) ==
                  CONCAT13(unaff_XMM7_Bd,
                           CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))));
  iVar19 = -(uint)(CONCAT13(unaff_XMM7_Bh,
                            CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))) ==
                  CONCAT13(unaff_XMM7_Bh,
                           CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))));
  iVar20 = -(uint)(CONCAT13(unaff_XMM7_Bl,
                            CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))) ==
                  CONCAT13(unaff_XMM7_Bl,
                           CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))));
  iVar21 = -(uint)(CONCAT13(unaff_XMM7_Bp,
                            CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))) ==
                  CONCAT13(unaff_XMM7_Bp,
                           CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))));
  local_78 = 0;
  uVar31 = 0;
  uVar32 = 0;
  if (cVar6 != '\0') goto LAB_142078190;
LAB_14207803c:
  if (uVar32 != uVar31) goto LAB_142078176;
  if (puVar37 != puVar2) {
    while( true ) {
      uVar31 = *(ulonglong *)(*(longlong *)(lVar10 + 0x100) + 0x58 + (ulonglong)*puVar37 * 0xa0);
      if (uVar31 != 0) break;
      puVar37 = puVar37 + 1;
      if (puVar37 == puVar2) goto LAB_142078510;
    }
    lVar25 = *(longlong *)(lVar10 + 0x100) + (ulonglong)*puVar37 * 0xa0;
    uVar32 = (ulonglong)*(uint *)(lVar25 + 0x94);
    lVar29 = *(longlong *)(lVar10 + 0x1a8);
    lVar28 = lVar29 + uVar32 * 0x48;
    lVar34 = *(longlong *)(lVar29 + 0x38 + uVar32 * 0x48);
    uVar32 = *(ulonglong *)(lVar29 + 0x40 + uVar32 * 0x48);
    if (*(ulonglong *)(lVar9 + 0x110) < uVar32) {
      lVar28 = *(longlong *)(lVar28 + 0x18);
      if (*(longlong *)(lVar34 + *(ulonglong *)(lVar9 + 0x110) * 8) == 0) goto LAB_1420780fd;
      uVar26 = *(ulonglong *)(lVar9 + 0x120);
      if (uVar26 < uVar32) goto LAB_142078115;
LAB_142078134:
      local_78 = 0;
    }
    else {
      lVar28 = *(longlong *)(lVar28 + 0x18);
LAB_1420780fd:
      uVar26 = *(ulonglong *)(lVar9 + 0x120);
      if (uVar32 <= uVar26) goto LAB_142078134;
LAB_142078115:
      uVar32 = *(ulonglong *)(lVar34 + uVar26 * 8);
      if (uVar32 == 0) goto LAB_142078134;
      local_78 = *(longlong *)(lVar28 + 0x10 + ~uVar32 * 0x30);
    }
    puVar37 = puVar37 + 1;
    lVar29 = ~*(ulonglong *)(lVar34 + *(longlong *)(lVar9 + 0x118) * 8) * 0x30;
    local_98 = *(longlong *)(lVar28 + 0x10 + lVar29);
    local_c0 = *(longlong *)(lVar28 + 0x28 + lVar29);
    local_108 = *(longlong *)(lVar25 + 0x50);
    uVar32 = 0;
LAB_142078176:
    uVar26 = (ulonglong)*(uint *)(local_108 + 8 + uVar32 * 0x10);
    do {
      puVar27 = (ulonglong *)(uVar26 * 0x10 + local_78);
      uVar15 = *puVar27;
      if ((char)puVar27[1] == '\0') {
        if (*(longlong *)(lVar12 + 0x48) != 0) {
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
          lVar28 = *(longlong *)(lVar12 + 0x30);
          bVar39 = (byte)(extraout_RAX_03 >> 0x39);
          auVar41 = ZEXT216(CONCAT11(bVar39,bVar39));
          auVar41 = pshuflw(auVar41,auVar41,0);
          local_e0 = 0;
          uVar35 = extraout_RAX_03;
          do {
            uVar35 = *(ulonglong *)(lVar12 + 0x38) & uVar35;
            pcVar3 = (char *)(lVar28 + uVar35);
            cVar40 = auVar41[0];
            auVar46[0] = -(*pcVar3 == cVar40);
            cVar42 = auVar41[1];
            auVar46[1] = -(pcVar3[1] == cVar42);
            cVar43 = auVar41[2];
            auVar46[2] = -(pcVar3[2] == cVar43);
            cVar44 = auVar41[3];
            auVar46[3] = -(pcVar3[3] == cVar44);
            auVar46[4] = -(pcVar3[4] == cVar40);
            auVar46[5] = -(pcVar3[5] == cVar42);
            auVar46[6] = -(pcVar3[6] == cVar43);
            auVar46[7] = -(pcVar3[7] == cVar44);
            auVar46[8] = -(pcVar3[8] == cVar40);
            auVar46[9] = -(pcVar3[9] == cVar42);
            auVar46[10] = -(pcVar3[10] == cVar43);
            auVar46[0xb] = -(pcVar3[0xb] == cVar44);
            auVar46[0xc] = -(pcVar3[0xc] == cVar40);
            auVar46[0xd] = -(pcVar3[0xd] == cVar42);
            auVar46[0xe] = -(pcVar3[0xe] == cVar43);
            auVar46[0xf] = -(pcVar3[0xf] == cVar44);
            uVar23 = (ushort)(SUB161(auVar46 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
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
                     (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                     (ushort)(auVar46[0xf] >> 7) << 0xf;
            uVar24 = (uint)uVar23;
            while (uVar23 != 0) {
              uVar16 = 0;
              for (uVar17 = uVar24; (uVar17 & 1) == 0; uVar17 = uVar17 >> 1 | 0x80000000) {
                uVar16 = uVar16 + 1;
              }
              uVar36 = uVar16 + uVar35 & *(ulonglong *)(lVar12 + 0x38);
              if (*(ulonglong *)(lVar28 + -0x10 + uVar36 * -0x10) == uVar15) {
                uVar35 = *(ulonglong *)(lVar28 + -8 + uVar36 * -0x10);
                if (((uVar35 >> 0x20 == 0) ||
                    (uVar36 = uVar35 & 0xffffffff, *(ulonglong *)(lVar14 + 0x10) <= uVar36)) ||
                   (*(int *)(*(longlong *)(lVar14 + 8) + uVar36 * 0x14) != (int)(uVar35 >> 0x20)))
                goto LAB_1420784b0;
                lVar28 = *(longlong *)(lVar14 + 8) + uVar36 * 0x14;
                uVar35 = (ulonglong)*(uint *)(lVar28 + 4);
                if (((uVar35 == 0xffffffff) || (*(ulonglong *)(lVar13 + 0x38) <= uVar35)) ||
                   ((*(ulonglong *)
                      (*(longlong *)(lVar13 + 0x28) + (ulonglong)(*(uint *)(lVar28 + 4) >> 6) * 8)
                     >> (uVar35 & 0x3f) & 1) == 0)) goto LAB_1420784b0;
                if (*(ulonglong *)(lVar13 + 0x110) <
                    *(ulonglong *)
                     (*(longlong *)(lVar14 + 0x1a8) + 0x40 +
                     (ulonglong)*(uint *)(lVar28 + 0xc) * 0x48)) {
                  lVar29 = *(longlong *)(lVar14 + 0x1a8) + (ulonglong)*(uint *)(lVar28 + 0xc) * 0x48
                  ;
                  uVar35 = *(ulonglong *)
                            (*(longlong *)(lVar29 + 0x38) + *(ulonglong *)(lVar13 + 0x110) * 8);
                  if (uVar35 == 0) goto LAB_14207848f;
                  lVar29 = *(longlong *)(*(longlong *)(lVar29 + 0x18) + 0x10 + ~uVar35 * 0x30);
                }
                else {
LAB_14207848f:
                  lVar29 = 0;
                }
                lVar28 = (ulonglong)*(uint *)(lVar28 + 0x10) * 0x58;
                if (*(char *)(lVar29 + 0x40 + lVar28) != '\x02') goto LAB_1420784b0;
                puVar38 = (undefined4 *)(lVar29 + lVar28 + 0x44);
                goto LAB_1420784c9;
              }
              uVar23 = (ushort)(uVar24 - 1) & (ushort)uVar24;
              uVar24 = CONCAT22((short)(uVar24 - 1 >> 0x10),uVar23);
            }
            auVar45[0] = -(*pcVar3 == (char)iVar18);
            auVar45[1] = -(pcVar3[1] == (char)((uint)iVar18 >> 8));
            auVar45[2] = -(pcVar3[2] == (char)((uint)iVar18 >> 0x10));
            auVar45[3] = -(pcVar3[3] == (char)((uint)iVar18 >> 0x18));
            auVar45[4] = -(pcVar3[4] == (char)iVar19);
            auVar45[5] = -(pcVar3[5] == (char)((uint)iVar19 >> 8));
            auVar45[6] = -(pcVar3[6] == (char)((uint)iVar19 >> 0x10));
            auVar45[7] = -(pcVar3[7] == (char)((uint)iVar19 >> 0x18));
            auVar45[8] = -(pcVar3[8] == (char)iVar20);
            auVar45[9] = -(pcVar3[9] == (char)((uint)iVar20 >> 8));
            auVar45[10] = -(pcVar3[10] == (char)((uint)iVar20 >> 0x10));
            auVar45[0xb] = -(pcVar3[0xb] == (char)((uint)iVar20 >> 0x18));
            auVar45[0xc] = -(pcVar3[0xc] == (char)iVar21);
            auVar45[0xd] = -(pcVar3[0xd] == (char)((uint)iVar21 >> 8));
            auVar45[0xe] = -(pcVar3[0xe] == (char)((uint)iVar21 >> 0x10));
            auVar45[0xf] = -(pcVar3[0xf] == (char)((uint)iVar21 >> 0x18));
            if ((((((((((((((((SUB161(auVar45 >> 7,0) & 1) != 0 ||
                             (SUB161(auVar45 >> 0xf,0) & 1) != 0) ||
                            (SUB161(auVar45 >> 0x17,0) & 1) != 0) ||
                           (SUB161(auVar45 >> 0x1f,0) & 1) != 0) ||
                          (SUB161(auVar45 >> 0x27,0) & 1) != 0) ||
                         (SUB161(auVar45 >> 0x2f,0) & 1) != 0) ||
                        (SUB161(auVar45 >> 0x37,0) & 1) != 0) ||
                       (SUB161(auVar45 >> 0x3f,0) & 1) != 0) || (SUB161(auVar45 >> 0x47,0) & 1) != 0
                      ) || (SUB161(auVar45 >> 0x4f,0) & 1) != 0) ||
                    (SUB161(auVar45 >> 0x57,0) & 1) != 0) || (SUB161(auVar45 >> 0x5f,0) & 1) != 0)
                  || (SUB161(auVar45 >> 0x67,0) & 1) != 0) || (SUB161(auVar45 >> 0x6f,0) & 1) != 0)
                || (SUB161(auVar45 >> 0x77,0) & 1) != 0) || auVar45[0xf] < '\0') break;
            uVar35 = uVar35 + local_e0 + 0x10;
            local_e0 = local_e0 + 0x10;
          } while( true );
        }
LAB_1420784b0:
        if (*(ulonglong *)(lVar11 + 0x10) <= uVar15) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
      }
      else if (*(ulonglong *)(lVar11 + 0x10) <= uVar15) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      puVar38 = (undefined4 *)(uVar15 * 4 + *(longlong *)(lVar11 + 8));
LAB_1420784c9:
      uVar8 = *puVar38;
      *(undefined4 *)(local_c0 + uVar26 * 4) = uVar7;
      *(undefined4 *)(uVar26 * 0x40 + local_98 + 0xc) = uVar8;
      gpu_world::OwnedGpuWorldSubset::
      update_instance_data<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>
                ();
      uVar32 = uVar32 + 1;
      if (cVar6 == '\0') goto LAB_14207803c;
LAB_142078190:
      if (uVar32 == uVar31) {
        if (puVar37 == puVar2) break;
        while( true ) {
          uVar31 = *(ulonglong *)(*(longlong *)(lVar10 + 0x1a8) + 0x10 + (ulonglong)*puVar37 * 0x48)
          ;
          if (uVar31 != 0) break;
          puVar37 = puVar37 + 1;
          if (puVar37 == puVar2) goto LAB_142078510;
        }
        lVar28 = *(longlong *)(lVar10 + 0x1a8) + (ulonglong)*puVar37 * 0x48;
        if (*(ulonglong *)(lVar9 + 0x110) < *(ulonglong *)(lVar28 + 0x40)) {
          lVar29 = *(longlong *)(lVar28 + 0x18);
        }
        else {
          lVar29 = *(longlong *)(lVar28 + 0x18);
        }
        puVar37 = puVar37 + 1;
        lVar34 = ~*(ulonglong *)(*(longlong *)(lVar28 + 0x38) + *(longlong *)(lVar9 + 0x118) * 8) *
                 0x30;
        local_98 = *(longlong *)(lVar29 + 0x10 + lVar34);
        local_c0 = *(longlong *)(lVar29 + 0x28 + lVar34);
        if ((*(ulonglong *)(lVar9 + 0x120) < *(ulonglong *)(lVar28 + 0x40)) &&
           (uVar32 = *(ulonglong *)
                      (*(longlong *)(lVar28 + 0x38) + *(ulonglong *)(lVar9 + 0x120) * 8),
           uVar32 != 0)) {
          local_78 = *(longlong *)(lVar29 + 0x10 + ~uVar32 * 0x30);
        }
        else {
          local_78 = 0;
        }
        uVar32 = 0;
      }
      uVar26 = uVar32 & 0xffffffff;
    } while( true );
  }
LAB_142078510:
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  lVar9 = *extraout_RAX_02;
  if (lVar9 == 8) {
    *extraout_RAX_02 = 0;
  }
  UNLOCK();
  if (lVar9 != 8) {
    parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
  }
  if (bVar22) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_04 == 1) {
      plVar30 = extraout_RAX_04 + 1;
    }
    else if ((*extraout_RAX_04 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar30 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0)) {
LAB_1420785cb:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar30 != 0) {
LAB_1420785d7:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar30 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar30 = *plVar30 + 1;
  }
  return;
}

