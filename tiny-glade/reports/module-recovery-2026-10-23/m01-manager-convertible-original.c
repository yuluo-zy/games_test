
// WARNING: Removing unreachable block (ram,0x0001422d24ea)

void __cdecl
system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities(void)

{
  byte *pbVar1;
  uint *puVar2;
  char *pcVar3;
  longlong *plVar4;
  byte bVar5;
  byte bVar6;
  undefined1 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  longlong lVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  code *pcVar13;
  int iVar14;
  bool bVar15;
  bool bVar16;
  longlong *plVar17;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  char extraout_AL_03;
  ushort uVar18;
  uint uVar19;
  uint uVar20;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  ulonglong extraout_RAX_02;
  ulonglong extraout_RAX_03;
  undefined1 *puVar21;
  uint *puVar22;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  uint uVar23;
  undefined4 *puVar24;
  longlong *plVar25;
  longlong *in_RDX;
  undefined4 *puVar26;
  ulonglong uVar27;
  char cVar28;
  ulonglong uVar29;
  longlong lVar30;
  undefined *puVar31;
  longlong lVar32;
  ulonglong uVar33;
  longlong lVar34;
  longlong in_R9;
  longlong lVar35;
  longlong *plVar36;
  undefined8 *puVar37;
  undefined1 (*pauVar38) [16];
  ulonglong uVar39;
  longlong lVar40;
  byte bVar41;
  char cVar42;
  char cVar46;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  char cVar44;
  char cVar45;
  undefined1 auVar43 [16];
  float fVar47;
  float fVar48;
  undefined1 auVar49 [16];
  undefined1 auVar50 [16];
  float fVar51;
  float fVar52;
  float fVar55;
  undefined1 auVar53 [16];
  undefined1 auVar54 [16];
  float fVar56;
  float fVar57;
  float fVar58;
  float fVar59;
  float fVar60;
  undefined8 *in_stack_00000028;
  longlong local_318;
  longlong local_310;
  ulonglong local_308;
  undefined1 (*local_2f0) [16];
  ulonglong local_2e8;
  longlong local_2d8;
  undefined4 *local_2c0;
  longlong local_280;
  longlong local_278;
  longlong local_268;
  undefined8 *local_258;
  ulonglong local_250;
  longlong local_248;
  longlong *plStack_240;
  longlong *local_238;
  undefined8 local_228;
  longlong local_1b8;
  longlong local_170;
  longlong local_148;
  longlong local_130;
  longlong *local_120;
  longlong *local_118;
  ulonglong local_110;
  undefined4 *local_100;
  
  plVar25 = in_RDX;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar15 = false;
  }
  else {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (plVar25 != (longlong *)0x0) {
        if (&DAT_00000054 < plVar25) {
          if (plVar25 != (longlong *)0x55) goto LAB_1422d05ab;
        }
        else if ((char)plVar25[0x28608552] < -0x40) {
LAB_1422d05ab:
                    // WARNING: Subroutine does not return
          core::str::slice_error_fail();
        }
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (plVar25 != (longlong *)0xfffffffffffffffe)) {
        if ((undefined1 *)((longlong)plVar25 + 2) < (undefined1 *)0x55) {
          if (*(char *)((longlong)plVar25 + 0x143042a92) < -0x40) {
LAB_1422d05ff:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if ((undefined1 *)((longlong)plVar25 + 2) != (undefined1 *)0x55) goto LAB_1422d05ff;
      }
    }
    puVar31 = &DAT_143042bf5;
    do {
      bVar41 = puVar31[-1];
      uVar19 = (uint)(char)bVar41;
      if ((char)bVar41 < '\0') {
        bVar5 = puVar31[-2];
        if ((char)bVar5 < -0x40) {
          bVar6 = puVar31[-3];
          if ((char)bVar6 < -0x40) {
            pbVar1 = puVar31 + -4;
            puVar31 = puVar31 + -4;
            uVar19 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar31 = puVar31 + -3;
            uVar19 = bVar6 & 0xf;
          }
          uVar19 = bVar5 & 0x3f | uVar19 << 6;
        }
        else {
          puVar31 = puVar31 + -2;
          uVar19 = bVar5 & 0x1f;
        }
        uVar19 = (uint)(bVar41 & 0x3f) | uVar19 << 6;
        if (uVar19 != 0x5c) goto LAB_1422d06c4;
LAB_1422d06d5:
        if (puVar31 == &UNK_143042baf) break;
        if (puVar31 + -0x143042baf < (undefined *)0x45) {
          if (-0x41 < (char)puVar31[1]) break;
        }
        else if (puVar31 + -0x143042baf == (undefined *)0x45) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar31 = puVar31 + -1;
      if (uVar19 == 0x5c) goto LAB_1422d06d5;
LAB_1422d06c4:
      if (uVar19 == 0x2f) goto LAB_1422d06d5;
    } while (puVar31 != &DAT_143042bb0);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar25 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar25 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0))
    goto LAB_1422d2634;
    if (*plVar25 != 0) goto LAB_1422d2640;
    *plVar25 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar25 = *plVar25 + 1;
    bVar15 = true;
  }
  std::collections::hash::map::impl$87::
  from_iter<u64,ref_<system_clutter::autoclutter::AutoClutterObject>,std::hash::random::RandomState,core::iter::adapters::map::Map<core::slice::iter::Iter<system_clutter::autoclutter::AutoClutterObject>,system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure_env_1>_>
            ();
  lVar10 = *in_RDX;
  local_248 = in_RDX[1];
  uVar8 = *(undefined4 *)((longlong)in_RDX + 0x14);
  local_238 = (longlong *)(local_248 + 0xf8);
  puVar22 = *(uint **)(lVar10 + 0xf8);
  puVar2 = puVar22 + *(longlong *)(lVar10 + 0x100);
  cVar28 = *(char *)(lVar10 + 0x108);
  local_280 = 8;
  local_1b8 = 0;
  local_110 = 0;
  uVar29 = 0;
  local_278 = 8;
  bVar16 = false;
  local_118 = (longlong *)0x0;
  local_148 = 0;
LAB_1422d088d:
  do {
    if (cVar28 == '\0') {
      if (local_110 == uVar29) {
        do {
          if (puVar22 == puVar2) goto LAB_1422d1323;
          uVar19 = *puVar22;
          puVar22 = puVar22 + 1;
          uVar29 = *(ulonglong *)
                    (*(longlong *)(local_248 + 0x100) + 0x58 + (ulonglong)uVar19 * 0xa0);
        } while (uVar29 == 0);
        lVar40 = *(longlong *)(local_248 + 0x100) + (ulonglong)uVar19 * 0xa0;
        lVar35 = *(longlong *)(local_248 + 0x1a8) + (ulonglong)*(uint *)(lVar40 + 0x94) * 0x48;
        if ((*(ulonglong *)(lVar10 + 0x110) <
             *(ulonglong *)
              (*(longlong *)(local_248 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar40 + 0x94) * 0x48)
            ) && (uVar27 = *(ulonglong *)
                            (*(longlong *)(lVar35 + 0x38) + *(ulonglong *)(lVar10 + 0x110) * 8),
                 uVar27 != 0)) {
          local_118 = *(longlong **)(*(longlong *)(lVar35 + 0x18) + 0x10 + ~uVar27 * 0x30);
        }
        else {
          local_118 = (longlong *)0x0;
        }
        uVar27 = *(ulonglong *)(lVar10 + 0x118);
        if (uVar27 < *(ulonglong *)(lVar40 + 0x88)) {
          lVar32 = *(longlong *)(lVar35 + 0x18);
          if (*(longlong *)(*(longlong *)(lVar40 + 0x80) + uVar27 * 8) == 0) goto LAB_1422d0ada;
          lVar35 = *(longlong *)(lVar35 + 0x38);
          lVar30 = ~*(ulonglong *)(lVar35 + uVar27 * 8) * 0x30;
          local_1b8 = *(longlong *)(lVar32 + 0x10 + lVar30);
          local_268 = *(longlong *)(lVar32 + 0x28 + lVar30);
          bVar16 = true;
        }
        else {
          lVar32 = *(longlong *)(lVar35 + 0x18);
LAB_1422d0ada:
          lVar35 = *(longlong *)(lVar35 + 0x38);
          bVar16 = false;
        }
        lVar35 = ~*(ulonglong *)(lVar35 + *(longlong *)(lVar10 + 0x120) * 8) * 0x30;
        local_148 = *(longlong *)(lVar32 + 0x10 + lVar35);
        local_170 = *(longlong *)(lVar32 + 0x28 + lVar35);
        local_280 = *(longlong *)(lVar40 + 0x50);
        local_110 = 0;
      }
      uVar27 = (ulonglong)*(uint *)(local_280 + 8 + local_110 * 0x10);
      if (bVar16) {
        pauVar38 = (undefined1 (*) [16])(uVar27 * 0x20 + local_1b8);
        puVar24 = (undefined4 *)(local_268 + uVar27 * 4);
      }
      else {
        pauVar38 = (undefined1 (*) [16])0x0;
        puVar24 = local_100;
      }
      local_130 = *(undefined8 *)(local_280 + local_110 * 0x10);
      plVar25 = (longlong *)((longlong)local_118 + uVar27 * 0x18);
      puVar37 = (undefined8 *)(uVar27 * 0x30 + local_148);
      puVar26 = (undefined4 *)(local_170 + uVar27 * 4);
      local_100 = puVar24;
    }
    else {
      if (local_110 == uVar29) {
        do {
          if (puVar22 == puVar2) goto LAB_1422d1323;
          uVar19 = *puVar22;
          puVar22 = puVar22 + 1;
          uVar29 = *(ulonglong *)
                    (*(longlong *)(local_248 + 0x1a8) + 0x10 + (ulonglong)uVar19 * 0x48);
        } while (uVar29 == 0);
        lVar35 = *(longlong *)(local_248 + 0x1a8) + (ulonglong)uVar19 * 0x48;
        if (*(ulonglong *)(lVar10 + 0x110) < *(ulonglong *)(lVar35 + 0x40)) {
          uVar27 = *(ulonglong *)(*(longlong *)(lVar35 + 0x38) + *(ulonglong *)(lVar10 + 0x110) * 8)
          ;
          if (uVar27 == 0) {
            local_118 = (longlong *)0x0;
          }
          else {
            local_118 = *(longlong **)(*(longlong *)(lVar35 + 0x18) + 0x10 + ~uVar27 * 0x30);
          }
        }
        else {
          local_118 = (longlong *)0x0;
        }
        if (*(ulonglong *)(lVar10 + 0x118) < *(ulonglong *)(lVar35 + 0x40)) {
          uVar27 = *(ulonglong *)(*(longlong *)(lVar35 + 0x38) + *(ulonglong *)(lVar10 + 0x118) * 8)
          ;
          lVar40 = *(longlong *)(lVar35 + 0x18);
          if (uVar27 == 0) goto LAB_1422d0bef;
          lVar32 = ~uVar27 * 0x30;
          local_1b8 = *(longlong *)(lVar40 + 0x10 + lVar32);
          local_268 = *(longlong *)(lVar40 + 0x28 + lVar32);
          bVar16 = true;
        }
        else {
          lVar40 = *(longlong *)(lVar35 + 0x18);
LAB_1422d0bef:
          bVar16 = false;
        }
        lVar32 = ~*(ulonglong *)(*(longlong *)(lVar35 + 0x38) + *(longlong *)(lVar10 + 0x120) * 8) *
                 0x30;
        local_148 = *(longlong *)(lVar40 + 0x10 + lVar32);
        local_170 = *(longlong *)(lVar40 + 0x28 + lVar32);
        local_278 = *(longlong *)(lVar35 + 8);
        uVar27 = 0;
        local_110 = uVar27;
        if (!bVar16) goto LAB_1422d0c4d;
LAB_1422d0a1b:
        pauVar38 = (undefined1 (*) [16])(uVar27 * 0x20 + local_1b8);
        puVar24 = (undefined4 *)(local_268 + uVar27 * 4);
      }
      else {
        uVar27 = local_110 & 0xffffffff;
        if (bVar16) goto LAB_1422d0a1b;
LAB_1422d0c4d:
        pauVar38 = (undefined1 (*) [16])0x0;
        puVar24 = local_2c0;
      }
      local_130 = *(undefined8 *)(local_278 + local_110 * 8);
      plVar25 = (longlong *)((longlong)local_118 + uVar27 * 0x18);
      puVar37 = (undefined8 *)(uVar27 * 0x30 + local_148);
      puVar26 = (undefined4 *)(local_170 + uVar27 * 4);
      local_2c0 = puVar24;
    }
    local_110 = local_110 + 1;
    if (*(char *)((longlong)plVar25 + 0x15) == '\0') {
      if (local_2d8 != 0) {
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
        bVar41 = (byte)(extraout_RAX_03 >> 0x39);
        auVar43 = ZEXT216(CONCAT11(bVar41,bVar41));
        auVar43 = pshuflw(auVar43,auVar43,0);
        lVar35 = 0;
        uVar27 = extraout_RAX_03;
        while( true ) {
          uVar27 = local_2e8 & uVar27;
          pcVar3 = *local_2f0 + uVar27;
          cVar42 = auVar43[0];
          auVar54[0] = -(*pcVar3 == cVar42);
          cVar44 = auVar43[1];
          auVar54[1] = -(pcVar3[1] == cVar44);
          cVar45 = auVar43[2];
          auVar54[2] = -(pcVar3[2] == cVar45);
          cVar46 = auVar43[3];
          auVar54[3] = -(pcVar3[3] == cVar46);
          auVar54[4] = -(pcVar3[4] == cVar42);
          auVar54[5] = -(pcVar3[5] == cVar44);
          auVar54[6] = -(pcVar3[6] == cVar45);
          auVar54[7] = -(pcVar3[7] == cVar46);
          auVar54[8] = -(pcVar3[8] == cVar42);
          auVar54[9] = -(pcVar3[9] == cVar44);
          auVar54[10] = -(pcVar3[10] == cVar45);
          auVar54[0xb] = -(pcVar3[0xb] == cVar46);
          auVar54[0xc] = -(pcVar3[0xc] == cVar42);
          auVar54[0xd] = -(pcVar3[0xd] == cVar44);
          auVar54[0xe] = -(pcVar3[0xe] == cVar45);
          auVar54[0xf] = -(pcVar3[0xf] == cVar46);
          uVar18 = (ushort)(SUB161(auVar54 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar54 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar54 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar54 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar54 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar54 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar54 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar54 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar54 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar54 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar54 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar54 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar54 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar54 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar54 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar54[0xf] >> 7) << 0xf;
          uVar19 = (uint)uVar18;
          if (uVar18 != 0) {
            do {
              uVar23 = 0;
              for (uVar20 = uVar19; (uVar20 & 1) == 0; uVar20 = uVar20 >> 1 | 0x80000000) {
                uVar23 = uVar23 + 1;
              }
              uVar39 = uVar23 + uVar27 & local_2e8;
              if (*plVar25 == *(longlong *)local_2f0[-1 - uVar39]) {
                lVar35 = *(longlong *)(local_2f0[-1 - uVar39] + 8);
                fVar48 = *(float *)(lVar35 + 0x4c) - *(float *)((longlong)puVar37 + 0x14);
                fVar52 = *(float *)(lVar35 + 0x48) - *(float *)(puVar37 + 2);
                fVar55 = *(float *)(lVar35 + 0x50) - *(float *)(puVar37 + 3);
                if (0.1 < SQRT(fVar55 * fVar55 + fVar48 * fVar48 + fVar52 * fVar52))
                goto LAB_1422d10ff;
                goto LAB_1422d088d;
              }
              uVar18 = (ushort)(uVar19 - 1) & (ushort)uVar19;
              uVar19 = CONCAT22((short)(uVar19 - 1 >> 0x10),uVar18);
            } while (uVar18 != 0);
          }
          auVar50[0] = -(*pcVar3 == -1);
          auVar50[1] = -(pcVar3[1] == -1);
          auVar50[2] = -(pcVar3[2] == -1);
          auVar50[3] = -(pcVar3[3] == -1);
          auVar50[4] = -(pcVar3[4] == -1);
          auVar50[5] = -(pcVar3[5] == -1);
          auVar50[6] = -(pcVar3[6] == -1);
          auVar50[7] = -(pcVar3[7] == -1);
          auVar50[8] = -(pcVar3[8] == -1);
          auVar50[9] = -(pcVar3[9] == -1);
          auVar50[10] = -(pcVar3[10] == -1);
          auVar50[0xb] = -(pcVar3[0xb] == -1);
          auVar50[0xc] = -(pcVar3[0xc] == -1);
          auVar50[0xd] = -(pcVar3[0xd] == -1);
          auVar50[0xe] = -(pcVar3[0xe] == -1);
          auVar50[0xf] = -(pcVar3[0xf] == -1);
          if ((((((((((((((((SUB161(auVar50 >> 7,0) & 1) != 0 || (SUB161(auVar50 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar50 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar50 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar50 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar50 >> 0x2f,0) & 1) != 0) || (SUB161(auVar50 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar50 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar50 >> 0x47,0) & 1) != 0) || (SUB161(auVar50 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar50 >> 0x57,0) & 1) != 0) || (SUB161(auVar50 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar50 >> 0x67,0) & 1) != 0) || (SUB161(auVar50 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar50 >> 0x77,0) & 1) != 0) || auVar50[0xf] < '\0') break;
          uVar27 = uVar27 + lVar35 + 0x10;
          lVar35 = lVar35 + 0x10;
        }
      }
LAB_1422d10ff:
      plVar25 = *(longlong **)(in_R9 + 8);
      if (plVar25 == (longlong *)0x0) {
        plVar25 = *(longlong **)(in_R9 + 0x10);
      }
      lVar35 = plVar25[2];
      if ((ulonglong)(*plVar25 - lVar35) < 0x20) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
    }
    else {
      if (local_2d8 != 0) {
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
        bVar41 = (byte)(extraout_RAX_02 >> 0x39);
        auVar43 = ZEXT216(CONCAT11(bVar41,bVar41));
        auVar43 = pshuflw(auVar43,auVar43,0);
        lVar35 = 0;
        uVar27 = extraout_RAX_02;
        do {
          uVar27 = local_2e8 & uVar27;
          auVar54 = *(undefined1 (*) [16])(*local_2f0 + uVar27);
          cVar42 = auVar43[0];
          auVar53[0] = -(auVar54[0] == cVar42);
          cVar44 = auVar43[1];
          auVar53[1] = -(auVar54[1] == cVar44);
          cVar45 = auVar43[2];
          auVar53[2] = -(auVar54[2] == cVar45);
          cVar46 = auVar43[3];
          auVar53[3] = -(auVar54[3] == cVar46);
          auVar53[4] = -(auVar54[4] == cVar42);
          auVar53[5] = -(auVar54[5] == cVar44);
          auVar53[6] = -(auVar54[6] == cVar45);
          auVar53[7] = -(auVar54[7] == cVar46);
          auVar53[8] = -(auVar54[8] == cVar42);
          auVar53[9] = -(auVar54[9] == cVar44);
          auVar53[10] = -(auVar54[10] == cVar45);
          auVar53[0xb] = -(auVar54[0xb] == cVar46);
          auVar53[0xc] = -(auVar54[0xc] == cVar42);
          auVar53[0xd] = -(auVar54[0xd] == cVar44);
          auVar53[0xe] = -(auVar54[0xe] == cVar45);
          auVar53[0xf] = -(auVar54[0xf] == cVar46);
          uVar18 = (ushort)(SUB161(auVar53 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar53 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar53 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar53 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar53 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar53 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar53 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar53 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar53 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar53 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar53 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar53 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar53 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar53 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar53 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar53[0xf] >> 7) << 0xf;
          uVar19 = (uint)uVar18;
          if (uVar18 != 0) {
            do {
              uVar23 = 0;
              for (uVar20 = uVar19; (uVar20 & 1) == 0; uVar20 = uVar20 >> 1 | 0x80000000) {
                uVar23 = uVar23 + 1;
              }
              uVar39 = uVar23 + uVar27 & local_2e8;
              if (*plVar25 == *(longlong *)local_2f0[-1 - uVar39]) {
                uVar9 = *(undefined4 *)(*(longlong *)(local_2f0[-1 - uVar39] + 8) + 0x50);
                plStack_240 = (longlong *)CONCAT44(plStack_240._4_4_,uVar9);
                uVar11 = *(undefined8 *)(*(longlong *)(local_2f0[-1 - uVar39] + 8) + 0x48);
                *puVar26 = uVar8;
                *(undefined4 *)(puVar37 + 3) = uVar9;
                puVar37[2] = uVar11;
                manage_autoclutter_entities::closure_0();
                *puVar26 = uVar8;
                *puVar37 = uVar11;
                puVar37[1] = plStack_240;
                if (pauVar38 == (undefined1 (*) [16])0x0) goto LAB_1422d088d;
                lVar35 = *(longlong *)(local_2f0[-1 - uVar39] + 8);
                fVar55 = (float)((ulonglong)*(undefined8 *)(lVar35 + 0x54) >> 0x20);
                fVar48 = *(float *)(lVar35 + 0x5c);
                fVar52 = (float)*(undefined8 *)(lVar35 + 0x54);
                fVar47 = fVar52 * 0.0 + fVar48 * -0.0;
                fVar57 = -fVar52 - fVar55 * -0.0;
                fVar56 = fVar55 * -0.0 + fVar48;
                fVar51 = 1.0 / SQRT(fVar57 * fVar57 + fVar47 * fVar47 + fVar56 * fVar56);
                fVar56 = fVar56 * fVar51;
                fVar47 = fVar47 * fVar51;
                fVar51 = fVar51 * fVar57;
                fVar57 = fVar55 * fVar51 - fVar48 * fVar47;
                fVar58 = fVar48 * fVar56 - fVar52 * fVar51;
                fVar60 = fVar52 * fVar47 - fVar55 * fVar56;
                fVar59 = 1.0 / SQRT(fVar60 * fVar60 + fVar58 * fVar58 + fVar57 * fVar57);
                fVar57 = fVar57 * fVar59;
                fVar60 = fVar59 * fVar60;
                if (fVar48 <= 0.0) {
                  fVar56 = fVar58 * fVar59 - fVar56;
                  if (0.0 < fVar56) {
                    fVar58 = fVar47 + fVar57;
                    fVar59 = (1.0 - fVar48) + fVar56;
                    fVar56 = fVar55 + fVar60;
                    fVar47 = fVar52 - fVar51;
                    fVar48 = fVar59;
                    goto LAB_1422d11e3;
                  }
                  fVar56 = (1.0 - fVar48) - fVar56;
                  fVar48 = 0.5 / SQRT(fVar56);
                  auVar43._0_4_ = fVar48 * fVar56;
                  auVar43._4_4_ = fVar48 * (fVar47 + fVar57);
                  auVar43._8_4_ = fVar48 * (fVar52 + fVar51);
                  auVar43._12_4_ = fVar48 * (fVar60 - fVar55);
                }
                else {
                  fVar56 = fVar56 + fVar58 * fVar59;
                  if (fVar56 <= 0.0) {
                    fVar58 = fVar52 + fVar51;
                    fVar59 = fVar55 + fVar60;
                    fVar56 = (fVar48 + 1.0) - fVar56;
                    fVar47 = fVar47 - fVar57;
                    fVar48 = fVar56;
LAB_1422d11e3:
                    fVar48 = 0.5 / SQRT(fVar48);
                    auVar43._0_4_ = fVar48 * fVar58;
                    auVar43._4_4_ = fVar48 * fVar59;
                    auVar43._8_4_ = fVar48 * fVar56;
                    auVar43._12_4_ = fVar48 * fVar47;
                  }
                  else {
                    fVar56 = fVar56 + fVar48 + 1.0;
                    fVar48 = 0.5 / SQRT(fVar56);
                    auVar43._0_4_ = fVar48 * (fVar60 - fVar55);
                    auVar43._4_4_ = fVar48 * (fVar52 - fVar51);
                    auVar43._8_4_ = fVar48 * (fVar47 - fVar57);
                    auVar43._12_4_ = fVar48 * fVar56;
                  }
                }
                *puVar24 = uVar8;
                *pauVar38 = auVar43;
                bVar41 = *(char *)(lVar35 + 0x30) - 3;
                if ((*(char *)(lVar35 + 0x30) == '\x02') || (bVar41 < 4 && bVar41 != 1))
                goto LAB_1422d088d;
                plVar25 = *(longlong **)(in_R9 + 8);
                if (plVar25 == (longlong *)0x0) {
                  plVar25 = *(longlong **)(in_R9 + 0x10);
                }
                uVar9 = *(undefined4 *)(lVar35 + 0x34);
                lVar35 = plVar25[2];
                if ((ulonglong)(*plVar25 - lVar35) < 0x20) {
                  alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
                }
                lVar40 = plVar25[1];
                *(code **)(lVar40 + lVar35) =
                     core::ops::function::FnOnce::
                     call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::color_id::ColorId>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
                ;
                *(code **)(lVar40 + 8 + lVar35) = bevy_ecs::error::handler::panic;
                *(longlong *)(lVar40 + 0x10 + lVar35) = local_130;
                *(undefined4 *)(lVar40 + 0x18 + lVar35) = uVar9;
                puVar21 = (undefined1 *)(lVar40 + lVar35 + 0x1c);
                goto LAB_1422d1160;
              }
              uVar18 = (ushort)(uVar19 - 1) & (ushort)uVar19;
              uVar19 = CONCAT22((short)(uVar19 - 1 >> 0x10),uVar18);
            } while (uVar18 != 0);
          }
          auVar49[0] = -(auVar54[0] == -1);
          auVar49[1] = -(auVar54[1] == -1);
          auVar49[2] = -(auVar54[2] == -1);
          auVar49[3] = -(auVar54[3] == -1);
          auVar49[4] = -(auVar54[4] == -1);
          auVar49[5] = -(auVar54[5] == -1);
          auVar49[6] = -(auVar54[6] == -1);
          auVar49[7] = -(auVar54[7] == -1);
          auVar49[8] = -(auVar54[8] == -1);
          auVar49[9] = -(auVar54[9] == -1);
          auVar49[10] = -(auVar54[10] == -1);
          auVar49[0xb] = -(auVar54[0xb] == -1);
          auVar49[0xc] = -(auVar54[0xc] == -1);
          auVar49[0xd] = -(auVar54[0xd] == -1);
          auVar49[0xe] = -(auVar54[0xe] == -1);
          auVar49[0xf] = -(auVar54[0xf] == -1);
          if ((((((((((((((((SUB161(auVar49 >> 7,0) & 1) != 0 || (SUB161(auVar49 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar49 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar49 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar49 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar49 >> 0x2f,0) & 1) != 0) || (SUB161(auVar49 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar49 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar49 >> 0x47,0) & 1) != 0) || (SUB161(auVar49 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar49 >> 0x57,0) & 1) != 0) || (SUB161(auVar49 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar49 >> 0x67,0) & 1) != 0) || (SUB161(auVar49 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar49 >> 0x77,0) & 1) != 0) || auVar49[0xf] < '\0') break;
          uVar27 = uVar27 + lVar35 + 0x10;
          lVar35 = lVar35 + 0x10;
        } while( true );
      }
      plVar25 = *(longlong **)(in_R9 + 8);
      if (plVar25 == (longlong *)0x0) {
        plVar25 = *(longlong **)(in_R9 + 0x10);
      }
      lVar35 = plVar25[2];
      if ((ulonglong)(*plVar25 - lVar35) < 0x20) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
    }
    lVar40 = plVar25[1];
    *(code **)(lVar40 + lVar35) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::clutter_meshes::ClutterMarkedForDeletion>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar40 + 8 + lVar35) = bevy_ecs::error::handler::panic;
    *(longlong *)(lVar40 + 0x10 + lVar35) = local_130;
    puVar21 = (undefined1 *)(lVar40 + lVar35 + 0x18);
LAB_1422d1160:
    *puVar21 = 0;
    plVar25[2] = lVar35 + 0x20;
  } while( true );
LAB_1422d16bd:
  do {
    if ((char)local_120[0x21] == '\0') {
      if (uVar39 == uVar27) {
        do {
          if (puVar22 == puVar2) goto LAB_1422d1810;
          uVar20 = *puVar22;
          puVar22 = puVar22 + 1;
          uVar27 = *(ulonglong *)(*(longlong *)(lVar10 + 0x100) + 0x58 + (ulonglong)uVar20 * 0xa0);
        } while (uVar27 == 0);
        lVar30 = *(longlong *)(lVar10 + 0x100) + (ulonglong)uVar20 * 0xa0;
        uVar39 = (ulonglong)*(uint *)(lVar30 + 0x94);
        if ((ulonglong)local_120[0x22] <
            *(ulonglong *)(*(longlong *)(lVar10 + 0x1a8) + 0x40 + uVar39 * 0x48)) {
          lVar32 = *(longlong *)(lVar10 + 0x1a8) + uVar39 * 0x48;
          uVar39 = *(ulonglong *)(*(longlong *)(lVar32 + 0x38) + local_120[0x22] * 8);
          if (uVar39 == 0) goto LAB_1422d17e8;
          lVar32 = *(longlong *)(*(longlong *)(lVar32 + 0x18) + 0x10 + ~uVar39 * 0x30);
        }
        else {
LAB_1422d17e8:
          lVar32 = 0;
        }
        lVar30 = *(longlong *)(lVar30 + 0x50);
        uVar39 = 0;
      }
      uVar33 = (ulonglong)*(uint *)(lVar30 + 8 + uVar39 * 0x10);
    }
    else {
      if (uVar39 == uVar27) {
        do {
          if (puVar22 == puVar2) goto LAB_1422d1810;
          uVar20 = *puVar22;
          puVar22 = puVar22 + 1;
          uVar27 = *(ulonglong *)(*(longlong *)(lVar10 + 0x1a8) + 0x10 + (ulonglong)uVar20 * 0x48);
        } while (uVar27 == 0);
        lVar32 = *(longlong *)(lVar10 + 0x1a8) + (ulonglong)uVar20 * 0x48;
        if (((ulonglong)local_120[0x22] < *(ulonglong *)(lVar32 + 0x40)) &&
           (uVar39 = *(ulonglong *)(*(longlong *)(lVar32 + 0x38) + local_120[0x22] * 8), uVar39 != 0
           )) {
          lVar32 = *(longlong *)(*(longlong *)(lVar32 + 0x18) + 0x10 + ~uVar39 * 0x30);
        }
        else {
          lVar32 = 0;
        }
        uVar39 = 0;
      }
      uVar33 = uVar39 & 0xffffffff;
    }
    lVar34 = uVar33 * 0x30 + lVar32;
    uVar39 = uVar39 + 1;
    fVar48 = *(float *)(lVar34 + 0x14) - *(float *)(lVar40 + 0x4c);
    fVar52 = *(float *)(lVar34 + 0x10) - *(float *)(lVar40 + 0x48);
    fVar55 = *(float *)(lVar34 + 0x18) - *(float *)(lVar40 + 0x50);
  } while (*(float *)(*(longlong *)(lVar40 + 0x38) + 0x40) + 1.0 <=
           SQRT(fVar55 * fVar55 + fVar48 * fVar48 + fVar52 * fVar52));
  goto LAB_1422d14de;
LAB_1422d1810:
  if (local_250 == 0) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
              ();
  }
  *(longlong *)(local_250 * 0x10 + 8) = lVar35;
  *(longlong *)(local_250 * 0x10 + 0x10) = lVar40;
  local_250 = local_250 + 1;
  goto LAB_1422d14de;
LAB_1422d1323:
  local_258 = (undefined8 *)&DAT_00000008;
  local_250 = 0;
  ::utils::rng::init_rng();
  local_228 = *(undefined8 *)(lVar10 + 0xf8);
  alloc::vec::spec_from_iter::impl$0::
  from_iter<u64,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple_<bevy_ecs::entity::Entity,ref_<country_core::components::AutoClutter>,enum2_<core::option::Option<ref_<system_clutter::autoclutter::WindowShutter>_>_>,ref_<utils::components::transform::Transform>_>,bevy_ecs::query::filter::Without<country_core::resources::clutter_meshes::ClutterMarkedForDeletion>_>,system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure_env_4>_>
            ();
  local_130 = local_2d8;
  auVar43 = *local_2f0;
  uVar19 = ~(uint)(ushort)((ushort)(SUB161(auVar43 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar43 >> 0xf,0) & 1) << 1 |
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
                           (ushort)(SUB161(auVar43 >> 0x77,0) & 1) << 0xe |
                          (ushort)(byte)(auVar43[0xf] >> 7) << 0xf);
  pauVar38 = local_2f0 + 1;
  local_120 = (longlong *)*in_stack_00000028;
  lVar10 = in_stack_00000028[1];
  uVar29 = local_308 & 0xfffffffffffffff8;
  uVar23 = (uint)local_308 & 7;
  local_118 = (longlong *)(ulonglong)uVar23;
LAB_1422d14de:
  if (local_130 != 0) {
    if ((short)uVar19 == 0) {
      do {
        auVar43 = *pauVar38;
        uVar18 = (ushort)(SUB161(auVar43 >> 7,0) & 1) | (ushort)(SUB161(auVar43 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar43 >> 0x17,0) & 1) << 2 |
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
                 (ushort)(SUB161(auVar43 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar43[0xf] >> 7) << 0xf;
        local_2f0 = local_2f0 + -0x10;
        pauVar38 = pauVar38 + 1;
      } while (uVar18 == 0xffff);
      uVar20 = ~(uint)uVar18;
      uVar19 = uVar20 & -(uint)uVar18 - 2;
    }
    else {
      uVar20 = uVar19;
      uVar19 = uVar19 - 1 & uVar19;
    }
    local_130 = local_130 + -1;
    iVar14 = 0;
    for (; (uVar20 & 1) == 0; uVar20 = uVar20 >> 1 | 0x80000000) {
      iVar14 = iVar14 + 1;
    }
    lVar35 = *(longlong *)((longlong)local_2f0 + (-0x10 - (ulonglong)(uint)(iVar14 << 4)));
    lVar40 = *(longlong *)((longlong)local_2f0 + (-8 - (ulonglong)(uint)(iVar14 << 4)));
    uVar27 = 0;
    while (uVar29 != uVar27) {
      lVar32 = uVar27 * 8;
      if (((((*(longlong *)(local_310 + uVar27 * 8) == lVar35) ||
            (*(longlong *)(local_310 + 8 + uVar27 * 8) == lVar35)) ||
           (*(longlong *)(local_310 + 0x10 + uVar27 * 8) == lVar35)) ||
          ((*(longlong *)(local_310 + 0x18 + uVar27 * 8) == lVar35 ||
           (*(longlong *)(local_310 + 0x20 + uVar27 * 8) == lVar35)))) ||
         ((*(longlong *)(local_310 + 0x28 + uVar27 * 8) == lVar35 ||
          ((*(longlong *)(local_310 + 0x30 + uVar27 * 8) == lVar35 ||
           (uVar27 = uVar27 + 8, *(longlong *)(local_310 + 0x38 + lVar32) == lVar35))))))
      goto LAB_1422d14de;
    }
    if ((local_118 == (longlong *)0x0) ||
       ((*(longlong *)(local_310 + uVar29 * 8) != lVar35 &&
        ((uVar23 == 1 ||
         ((*(longlong *)(local_310 + 8 + uVar29 * 8) != lVar35 &&
          ((uVar23 == 2 ||
           ((*(longlong *)(local_310 + 0x10 + uVar29 * 8) != lVar35 &&
            ((uVar23 == 3 ||
             ((*(longlong *)(local_310 + 0x18 + uVar29 * 8) != lVar35 &&
              ((uVar23 == 4 ||
               ((*(longlong *)(local_310 + 0x20 + uVar29 * 8) != lVar35 &&
                ((uVar23 == 5 ||
                 ((*(longlong *)(local_310 + 0x28 + uVar29 * 8) != lVar35 &&
                  ((uVar23 == 6 ||
                   ((*(longlong *)(local_310 + 0x30 + uVar29 * 8) != lVar35 && (uVar23 == 7)))))))))
                ))))))))))))))))))) {
      puVar22 = (uint *)local_120[0x1f];
      puVar2 = puVar22 + local_120[0x20];
      lVar30 = 8;
      uVar39 = 0;
      uVar27 = 0;
      lVar32 = 0;
      goto LAB_1422d16bd;
    }
    goto LAB_1422d14de;
  }
  plStack_240 = (longlong *)(local_248 + 0x1a0);
  for (; uVar11 = local_228, plVar17 = local_238, plVar25 = plStack_240, lVar10 = local_248,
      local_258 != (undefined8 *)(local_250 * 0x10 + 8); local_258 = local_258 + 2) {
    uVar12 = *local_258;
    puVar37 = (undefined8 *)local_258[1];
    manage_autoclutter_entities::closure_0();
    bVar41 = *(char *)(puVar37 + 6) - 3;
    if (bVar41 == 1 || 3 < bVar41) {
      fastrand::Rng::f32();
      fVar52 = (1.0 - extraout_XMM0_Da) * 0.0;
      fVar48 = extraout_XMM0_Da * 0.1;
    }
    else {
      if (*(char *)(puVar37 + 6) == '\x06') {
        fastrand::Rng::f32();
        fVar52 = (1.0 - extraout_XMM0_Da_00) * 0.2;
        fVar48 = extraout_XMM0_Da_00;
      }
      else {
        if (4 < local_250) {
          fastrand::Rng::f32();
          fVar52 = (1.0 - extraout_XMM0_Da_02) * 0.3;
          fVar48 = extraout_XMM0_Da_02 * 0.8;
          goto LAB_1422d1af0;
        }
        fastrand::Rng::f32();
        fVar52 = (1.0 - extraout_XMM0_Da_01) * 0.1;
        fVar48 = extraout_XMM0_Da_01;
      }
      fVar48 = fVar48 * 0.3;
    }
LAB_1422d1af0:
    cVar28 = *(char *)(puVar37 + 6);
    uVar7 = *(undefined1 *)(puVar37[7] + 0x44);
    uVar8 = *(undefined4 *)(puVar37 + 0xc);
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar36 = plStack_240;
    if (plStack_240 == (longlong *)0x0) {
      plVar36 = local_238;
    }
    lVar35 = plVar36[2];
    if ((ulonglong)(*plVar36 - lVar35) < 0x38) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar40 = plVar36[1];
    *(code **)(lVar40 + lVar35) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::AutoClutter>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar40 + 8 + lVar35) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar40 + 0x10 + lVar35) = local_228;
    *(undefined8 *)(lVar40 + 0x18 + lVar35) = uVar12;
    *(float *)(lVar40 + 0x20 + lVar35) = fVar48 + fVar52;
    *(undefined4 *)(lVar40 + 0x24 + lVar35) = 0;
    *(undefined4 *)(lVar40 + 0x28 + lVar35) = uVar8;
    *(undefined1 *)(lVar40 + 0x2c + lVar35) = uVar7;
    *(bool *)(lVar40 + 0x2d + lVar35) = 3 < (byte)(cVar28 - 3U) || (byte)(cVar28 - 3U) == 1;
    *(undefined1 *)(lVar40 + 0x30 + lVar35) = 0;
    plVar36[2] = lVar35 + 0x38;
    if ((*(byte *)(puVar37 + 6) & 6) != 4 && 2 < *(byte *)(puVar37 + 6)) {
      switch(*puVar37) {
      case 3:
        local_118 = (longlong *)puVar37[1];
        local_120 = (longlong *)
                    ((ulonglong)local_120 & 0xffffffff00000000 | (ulonglong)*(uint *)(puVar37 + 2));
        break;
      case 4:
        local_118 = (longlong *)puVar37[1];
        local_120 = (longlong *)puVar37[2];
        LOCK();
        lVar35 = *local_118;
        *local_118 = *local_118 + 1;
        UNLOCK();
        if (*local_118 == 0 || SCARRY8(lVar35,1) != *local_118 < 0) {
                    // WARNING: Does not return
          pcVar13 = (code *)invalidInstructionException();
          (*pcVar13)();
        }
        break;
      case 5:
        break;
      default:
        local_118 = (longlong *)puVar37[1];
        local_120 = (longlong *)
                    ((ulonglong)local_120 & 0xffffffff00000000 | (ulonglong)*(uint *)(puVar37 + 2));
      }
      local_228 = puVar37[4];
      bevy_ecs::system::commands::EntityCommands::
      insert<country_core::components::AutoClutterConvertableToDecoClutter>();
      plStack_240 = local_118;
      local_238 = local_120;
    }
    lVar35 = puVar37[7];
    plVar36 = plVar25;
    if (*(char *)(lVar35 + 0x45) == '\t') {
      cVar28 = *(char *)(puVar37 + 6);
      if ((cVar28 == '\x02') || ((byte)(cVar28 - 3U) < 4 && (byte)(cVar28 - 3U) != 1)) {
        if (cVar28 == '\x05') {
          uVar12 = *(undefined8 *)(lVar35 + 0x38);
          uVar8 = *(undefined4 *)(puVar37 + 10);
          plStack_240 = (longlong *)CONCAT44(plStack_240._4_4_,uVar8);
          lVar35 = puVar37[9];
          bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
          if (plVar25 == (longlong *)0x0) {
            plVar36 = plVar17;
          }
          lVar40 = plVar36[2];
          if ((ulonglong)(*plVar36 - lVar40) < 0x88) {
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          lVar32 = plVar36[1];
          *(code **)(lVar32 + lVar40) =
               core::ops::function::FnOnce::
               call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::PlantInstanceData>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          plVar4 = (longlong *)(lVar32 + 8 + lVar40);
          *plVar4 = local_248;
          plVar4[1] = (longlong)plVar25;
          *(longlong *)(lVar32 + 0x18 + lVar40) = lVar35;
          *(undefined4 *)(lVar32 + 0x20 + lVar40) = uVar8;
          *(undefined8 *)(lVar32 + 0x24 + lVar40) = 0;
          *(undefined4 *)(lVar32 + 0x2c + lVar40) = 0;
          *(undefined4 *)(lVar32 + 0x38 + lVar40) = 0;
          *(undefined8 *)(lVar32 + 0x40 + lVar40) = uVar12;
          *(undefined1 *)(lVar32 + 0x48 + lVar40) = 10;
          local_248 = lVar35;
        }
        else {
          uVar12 = *(undefined8 *)(lVar35 + 0x38);
          uVar8 = *(undefined4 *)(puVar37 + 10);
          plStack_240 = (longlong *)CONCAT44(plStack_240._4_4_,uVar8);
          lVar35 = puVar37[9];
          bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
          if (plVar25 == (longlong *)0x0) {
            plVar36 = plVar17;
          }
          lVar40 = plVar36[2];
          if ((ulonglong)(*plVar36 - lVar40) < 0x88) {
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          lVar32 = plVar36[1];
          *(code **)(lVar32 + lVar40) =
               core::ops::function::FnOnce::
               call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SolidVertexColorInstanceData>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          plVar4 = (longlong *)(lVar32 + 8 + lVar40);
          *plVar4 = local_248;
          plVar4[1] = (longlong)plVar25;
          *(longlong *)(lVar32 + 0x18 + lVar40) = lVar35;
          *(undefined4 *)(lVar32 + 0x20 + lVar40) = uVar8;
          *(undefined8 *)(lVar32 + 0x24 + lVar40) = 0;
          *(undefined4 *)(lVar32 + 0x2c + lVar40) = 0;
          *(undefined4 *)(lVar32 + 0x38 + lVar40) = 0;
          *(undefined8 *)(lVar32 + 0x40 + lVar40) = uVar12;
          *(undefined1 *)(lVar32 + 0x48 + lVar40) = 3;
          local_248 = lVar35;
        }
      }
      else {
        uVar12 = *(undefined8 *)(lVar35 + 0x38);
        uVar8 = *(undefined4 *)(puVar37 + 10);
        plStack_240 = (longlong *)CONCAT44(plStack_240._4_4_,uVar8);
        lVar35 = puVar37[9];
        bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
        if (plVar25 == (longlong *)0x0) {
          plVar36 = plVar17;
        }
        lVar40 = plVar36[2];
        if ((ulonglong)(*plVar36 - lVar40) < 0x88) {
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
        lVar32 = plVar36[1];
        *(code **)(lVar32 + lVar40) =
             core::ops::function::FnOnce::
             call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
        ;
        plVar4 = (longlong *)(lVar32 + 8 + lVar40);
        *plVar4 = local_248;
        plVar4[1] = (longlong)plVar25;
        *(longlong *)(lVar32 + 0x18 + lVar40) = lVar35;
        *(undefined4 *)(lVar32 + 0x20 + lVar40) = uVar8;
        *(undefined8 *)(lVar32 + 0x24 + lVar40) = 0;
        *(undefined4 *)(lVar32 + 0x2c + lVar40) = 0;
        *(undefined4 *)(lVar32 + 0x38 + lVar40) = 0;
        *(undefined8 *)(lVar32 + 0x40 + lVar40) = uVar12;
        *(undefined1 *)(lVar32 + 0x48 + lVar40) = 4;
        local_248 = lVar35;
      }
      *(undefined1 *)(lVar32 + 0x58 + lVar40) = 0;
      *(undefined8 *)(lVar32 + 0x68 + lVar40) = uVar11;
      *(code **)(lVar32 + 0x78 + lVar40) = bevy_ecs::error::handler::panic;
      lVar40 = lVar40 + 0x88;
    }
    else {
      ::utils::name_hash::NameHash::compute();
      ::utils::name_hash::impl$2::eq();
      if (extraout_AL_00 == '\0') {
        ::utils::name_hash::NameHash::compute();
        ::utils::name_hash::impl$2::eq();
        if (extraout_AL_01 != '\0') goto LAB_1422d1f30;
        ::utils::name_hash::NameHash::compute();
        ::utils::name_hash::impl$2::eq();
        if (extraout_AL_02 != '\0') goto LAB_1422d1f30;
        ::utils::name_hash::NameHash::compute();
        ::utils::name_hash::impl$2::eq();
        if (extraout_AL_03 != '\0') goto LAB_1422d1f30;
        uVar12 = *(undefined8 *)(puVar37[7] + 0x38);
        uVar8 = *(undefined4 *)(puVar37 + 10);
        lVar35 = puVar37[9];
        bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
        cVar28 = *(char *)(puVar37[7] + 0x45);
        if (cVar28 == '\t') {
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
        if (plVar25 == (longlong *)0x0) {
          plVar36 = plVar17;
        }
        lVar40 = plVar36[2];
        if ((ulonglong)(*plVar36 - lVar40) < 0x98) {
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
        lVar32 = plVar36[1];
        *(code **)(lVar32 + lVar40) =
             core::ops::function::FnOnce::
             call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SolidAlbedoMapInstanceData>,country_core::components::drawable::NaniSolidTexture>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
        ;
        plVar4 = (longlong *)(lVar32 + 8 + lVar40);
        *plVar4 = local_248;
        plVar4[1] = (longlong)plVar25;
        *(longlong *)(lVar32 + 0x18 + lVar40) = lVar35;
        *(undefined4 *)(lVar32 + 0x20 + lVar40) = uVar8;
        *(undefined8 *)(lVar32 + 0x24 + lVar40) = 0;
        *(undefined4 *)(lVar32 + 0x2c + lVar40) = 0;
        *(undefined4 *)(lVar32 + 0x38 + lVar40) = 0;
        *(undefined8 *)(lVar32 + 0x40 + lVar40) = uVar12;
        *(undefined1 *)(lVar32 + 0x48 + lVar40) = 0xf;
        local_248 = lVar35;
      }
      else {
LAB_1422d1f30:
        uVar12 = *(undefined8 *)(puVar37[7] + 0x38);
        uVar8 = *(undefined4 *)(puVar37 + 10);
        lVar35 = puVar37[9];
        bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
        cVar28 = *(char *)(puVar37[7] + 0x45);
        if (cVar28 == '\t') {
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
        if (plVar25 == (longlong *)0x0) {
          plVar36 = plVar17;
        }
        lVar40 = plVar36[2];
        if ((ulonglong)(*plVar36 - lVar40) < 0x98) {
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
        lVar32 = plVar36[1];
        *(code **)(lVar32 + lVar40) =
             core::ops::function::FnOnce::
             call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SolidAlbedoMapEmissiveInstanceData>,country_core::components::drawable::NaniSolidTexture>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
        ;
        plVar4 = (longlong *)(lVar32 + 8 + lVar40);
        *plVar4 = local_248;
        plVar4[1] = (longlong)plVar25;
        *(longlong *)(lVar32 + 0x18 + lVar40) = lVar35;
        *(undefined4 *)(lVar32 + 0x20 + lVar40) = uVar8;
        *(undefined8 *)(lVar32 + 0x24 + lVar40) = 0;
        *(undefined4 *)(lVar32 + 0x2c + lVar40) = 0;
        *(undefined4 *)(lVar32 + 0x38 + lVar40) = 0;
        *(undefined8 *)(lVar32 + 0x40 + lVar40) = uVar12;
        *(undefined1 *)(lVar32 + 0x48 + lVar40) = 0x10;
        local_248 = lVar35;
      }
      plStack_240 = (longlong *)CONCAT44(plStack_240._4_4_,uVar8);
      *(char *)(lVar32 + 0x58 + lVar40) = cVar28;
      *(undefined1 *)(lVar32 + 0x68 + lVar40) = 0;
      *(undefined8 *)(lVar32 + 0x78 + lVar40) = uVar11;
      *(code **)(lVar32 + 0x88 + lVar40) = bevy_ecs::error::handler::panic;
      lVar40 = lVar40 + 0x98;
    }
    plVar36[2] = lVar40;
    cVar28 = *(char *)(puVar37 + 6);
    if ((cVar28 != '\x02') && (3 < (byte)(cVar28 - 3U) || (byte)(cVar28 - 3U) == 1)) {
      plVar36 = plVar25;
      if (plVar25 == (longlong *)0x0) {
        plVar36 = plVar17;
      }
      lVar35 = plVar36[2];
      if ((ulonglong)(*plVar36 - lVar35) < 0x58) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar40 = plVar36[1];
      *(code **)(lVar40 + lVar35) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<system_clutter::autoclutter::WindowShutter>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      plVar4 = (longlong *)(lVar40 + 8 + lVar35);
      *plVar4 = lVar10;
      plVar4[1] = (longlong)plVar25;
      *(char *)(lVar40 + 0x18 + lVar35) = cVar28;
      *(undefined1 *)(lVar40 + 0x28 + lVar35) = 0;
      *(undefined8 *)(lVar40 + 0x38 + lVar35) = uVar11;
      *(code **)(lVar40 + 0x48 + lVar35) = bevy_ecs::error::handler::panic;
      plVar36[2] = lVar35 + 0x58;
      if (plVar25 == (longlong *)0x0) {
        plVar25 = plVar17;
      }
      uVar8 = *(undefined4 *)((longlong)puVar37 + 0x34);
      lVar10 = plVar25[2];
      if ((ulonglong)(*plVar25 - lVar10) < 0x20) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar35 = plVar25[1];
      *(code **)(lVar35 + lVar10) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::color_id::ColorId>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      *(code **)(lVar35 + 8 + lVar10) = bevy_ecs::error::handler::panic;
      *(undefined8 *)(lVar35 + 0x10 + lVar10) = uVar11;
      *(undefined4 *)(lVar35 + 0x18 + lVar10) = uVar8;
      *(undefined1 *)(lVar35 + 0x1c + lVar10) = 0;
      plVar25[2] = lVar10 + 0x20;
    }
  }
  if (local_318 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if ((local_2e8 != 0) && (local_2e8 * 0x11 != -0x21)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (bVar15) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_04 == 1) {
      plVar25 = extraout_RAX_04 + 1;
    }
    else if ((*extraout_RAX_04 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar25 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0)) {
LAB_1422d2634:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar25 != 0) {
LAB_1422d2640:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar25 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar25 = *plVar25 + 1;
  }
  return;
}


