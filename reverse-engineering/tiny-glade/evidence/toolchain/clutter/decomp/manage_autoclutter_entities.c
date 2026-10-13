
/* WARNING: Removing unreachable block (ram,0x0001422d24ea) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

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
  uint uVar15;
  bool bVar16;
  bool bVar17;
  float fVar18;
  float fVar19;
  longlong *plVar20;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  char extraout_AL_03;
  ushort uVar21;
  uint uVar22;
  uint uVar23;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  ulonglong extraout_RAX_02;
  ulonglong extraout_RAX_03;
  undefined1 *puVar24;
  uint *puVar25;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  uint uVar26;
  undefined4 *puVar27;
  longlong *plVar28;
  longlong *in_RDX;
  undefined4 *puVar29;
  ulonglong uVar30;
  char cVar31;
  ulonglong uVar32;
  longlong lVar33;
  undefined *puVar34;
  longlong lVar35;
  ulonglong uVar36;
  longlong lVar37;
  longlong in_R9;
  longlong lVar38;
  longlong *plVar39;
  undefined8 *puVar40;
  undefined1 (*pauVar41) [16];
  ulonglong uVar42;
  longlong lVar43;
  byte bVar44;
  char cVar45;
  char cVar49;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  char cVar47;
  char cVar48;
  undefined1 auVar46 [16];
  float fVar50;
  float fVar51;
  undefined1 auVar52 [16];
  undefined1 auVar53 [16];
  float fVar54;
  float fVar55;
  float fVar56;
  float fVar59;
  undefined1 auVar57 [16];
  undefined1 auVar58 [16];
  float fVar60;
  float fVar61;
  float fVar62;
  float fVar63;
  float fVar64;
  float fVar65;
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
  
  plVar28 = in_RDX;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar16 = false;
  }
  else {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (plVar28 != (longlong *)0x0) {
        if (&DAT_00000054 < plVar28) {
          if (plVar28 != (longlong *)0x55) goto LAB_1422d05ab;
        }
        else if ((char)plVar28[0x28608552] < -0x40) {
LAB_1422d05ab:
                    /* WARNING: Subroutine does not return */
          core::str::slice_error_fail();
        }
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (plVar28 != (longlong *)0xfffffffffffffffe)) {
        if ((undefined1 *)((longlong)plVar28 + 2) < (undefined1 *)0x55) {
          if (*(char *)((longlong)plVar28 + 0x143042a92) < -0x40) {
LAB_1422d05ff:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if ((undefined1 *)((longlong)plVar28 + 2) != (undefined1 *)0x55) goto LAB_1422d05ff;
      }
    }
    puVar34 = &DAT_143042bf5;
    do {
      bVar44 = puVar34[-1];
      uVar22 = (uint)(char)bVar44;
      if ((char)bVar44 < '\0') {
        bVar5 = puVar34[-2];
        if ((char)bVar5 < -0x40) {
          bVar6 = puVar34[-3];
          if ((char)bVar6 < -0x40) {
            pbVar1 = puVar34 + -4;
            puVar34 = puVar34 + -4;
            uVar22 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar34 = puVar34 + -3;
            uVar22 = bVar6 & 0xf;
          }
          uVar22 = bVar5 & 0x3f | uVar22 << 6;
        }
        else {
          puVar34 = puVar34 + -2;
          uVar22 = bVar5 & 0x1f;
        }
        uVar22 = (uint)(bVar44 & 0x3f) | uVar22 << 6;
        if (uVar22 != 0x5c) goto LAB_1422d06c4;
LAB_1422d06d5:
        if (puVar34 == &UNK_143042baf) break;
        if (puVar34 + -0x143042baf < (undefined *)0x45) {
          if (-0x41 < (char)puVar34[1]) break;
        }
        else if (puVar34 + -0x143042baf == (undefined *)0x45) break;
                    /* WARNING: Subroutine does not return */
        core::str::slice_error_fail();
      }
      puVar34 = puVar34 + -1;
      if (uVar22 == 0x5c) goto LAB_1422d06d5;
LAB_1422d06c4:
      if (uVar22 == 0x2f) goto LAB_1422d06d5;
    } while (puVar34 != &DAT_143042bb0);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar28 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar28 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0))
    goto LAB_1422d2634;
    if (*plVar28 != 0) goto LAB_1422d2640;
    *plVar28 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar28 = *plVar28 + 1;
    bVar16 = true;
  }
  std::collections::hash::map::impl$87::
  from_iter<u64,ref_<system_clutter::autoclutter::AutoClutterObject>,std::hash::random::RandomState,core::iter::adapters::map::Map<core::slice::iter::Iter<system_clutter::autoclutter::AutoClutterObject>,system_clutter::autoclutter::manage_autoclutter_entities::manage_autoclutter_entities::closure_env_1>_>
            ();
  fVar62 = __real_80000000;
  fVar19 = __real_3dcccccd;
  fVar18 = __real_3f800000;
  uVar22 = ___xmm_80000000800000008000000080000000;
  fVar56 = __real_3f000000;
  lVar10 = *in_RDX;
  local_248 = in_RDX[1];
  uVar8 = *(undefined4 *)((longlong)in_RDX + 0x14);
  local_238 = (longlong *)(local_248 + 0xf8);
  puVar25 = *(uint **)(lVar10 + 0xf8);
  puVar2 = puVar25 + *(longlong *)(lVar10 + 0x100);
  cVar31 = *(char *)(lVar10 + 0x108);
  local_280 = 8;
  local_1b8 = 0;
  local_110 = 0;
  uVar32 = 0;
  local_278 = 8;
  bVar17 = false;
  local_118 = (longlong *)0x0;
  local_148 = 0;
LAB_1422d088d:
  do {
    if (cVar31 == '\0') {
      if (local_110 == uVar32) {
        do {
          if (puVar25 == puVar2) goto LAB_1422d1323;
          uVar26 = *puVar25;
          puVar25 = puVar25 + 1;
          uVar32 = *(ulonglong *)
                    (*(longlong *)(local_248 + 0x100) + 0x58 + (ulonglong)uVar26 * 0xa0);
        } while (uVar32 == 0);
        lVar43 = *(longlong *)(local_248 + 0x100) + (ulonglong)uVar26 * 0xa0;
        lVar38 = *(longlong *)(local_248 + 0x1a8) + (ulonglong)*(uint *)(lVar43 + 0x94) * 0x48;
        if ((*(ulonglong *)(lVar10 + 0x110) <
             *(ulonglong *)
              (*(longlong *)(local_248 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar43 + 0x94) * 0x48)
            ) && (uVar30 = *(ulonglong *)
                            (*(longlong *)(lVar38 + 0x38) + *(ulonglong *)(lVar10 + 0x110) * 8),
                 uVar30 != 0)) {
          local_118 = *(longlong **)(*(longlong *)(lVar38 + 0x18) + 0x10 + ~uVar30 * 0x30);
        }
        else {
          local_118 = (longlong *)0x0;
        }
        uVar30 = *(ulonglong *)(lVar10 + 0x118);
        if (uVar30 < *(ulonglong *)(lVar43 + 0x88)) {
          lVar35 = *(longlong *)(lVar38 + 0x18);
          if (*(longlong *)(*(longlong *)(lVar43 + 0x80) + uVar30 * 8) == 0) goto LAB_1422d0ada;
          lVar38 = *(longlong *)(lVar38 + 0x38);
          lVar33 = ~*(ulonglong *)(lVar38 + uVar30 * 8) * 0x30;
          local_1b8 = *(longlong *)(lVar35 + 0x10 + lVar33);
          local_268 = *(longlong *)(lVar35 + 0x28 + lVar33);
          bVar17 = true;
        }
        else {
          lVar35 = *(longlong *)(lVar38 + 0x18);
LAB_1422d0ada:
          lVar38 = *(longlong *)(lVar38 + 0x38);
          bVar17 = false;
        }
        lVar38 = ~*(ulonglong *)(lVar38 + *(longlong *)(lVar10 + 0x120) * 8) * 0x30;
        local_148 = *(longlong *)(lVar35 + 0x10 + lVar38);
        local_170 = *(longlong *)(lVar35 + 0x28 + lVar38);
        local_280 = *(longlong *)(lVar43 + 0x50);
        local_110 = 0;
      }
      uVar30 = (ulonglong)*(uint *)(local_280 + 8 + local_110 * 0x10);
      if (bVar17) {
        pauVar41 = (undefined1 (*) [16])(uVar30 * 0x20 + local_1b8);
        puVar27 = (undefined4 *)(local_268 + uVar30 * 4);
      }
      else {
        pauVar41 = (undefined1 (*) [16])0x0;
        puVar27 = local_100;
      }
      local_130 = *(undefined8 *)(local_280 + local_110 * 0x10);
      plVar28 = (longlong *)((longlong)local_118 + uVar30 * 0x18);
      puVar40 = (undefined8 *)(uVar30 * 0x30 + local_148);
      puVar29 = (undefined4 *)(local_170 + uVar30 * 4);
      local_100 = puVar27;
    }
    else {
      if (local_110 == uVar32) {
        do {
          if (puVar25 == puVar2) goto LAB_1422d1323;
          uVar26 = *puVar25;
          puVar25 = puVar25 + 1;
          uVar32 = *(ulonglong *)
                    (*(longlong *)(local_248 + 0x1a8) + 0x10 + (ulonglong)uVar26 * 0x48);
        } while (uVar32 == 0);
        lVar38 = *(longlong *)(local_248 + 0x1a8) + (ulonglong)uVar26 * 0x48;
        if (*(ulonglong *)(lVar10 + 0x110) < *(ulonglong *)(lVar38 + 0x40)) {
          uVar30 = *(ulonglong *)(*(longlong *)(lVar38 + 0x38) + *(ulonglong *)(lVar10 + 0x110) * 8)
          ;
          if (uVar30 == 0) {
            local_118 = (longlong *)0x0;
          }
          else {
            local_118 = *(longlong **)(*(longlong *)(lVar38 + 0x18) + 0x10 + ~uVar30 * 0x30);
          }
        }
        else {
          local_118 = (longlong *)0x0;
        }
        if (*(ulonglong *)(lVar10 + 0x118) < *(ulonglong *)(lVar38 + 0x40)) {
          uVar30 = *(ulonglong *)(*(longlong *)(lVar38 + 0x38) + *(ulonglong *)(lVar10 + 0x118) * 8)
          ;
          lVar43 = *(longlong *)(lVar38 + 0x18);
          if (uVar30 == 0) goto LAB_1422d0bef;
          lVar35 = ~uVar30 * 0x30;
          local_1b8 = *(longlong *)(lVar43 + 0x10 + lVar35);
          local_268 = *(longlong *)(lVar43 + 0x28 + lVar35);
          bVar17 = true;
        }
        else {
          lVar43 = *(longlong *)(lVar38 + 0x18);
LAB_1422d0bef:
          bVar17 = false;
        }
        lVar35 = ~*(ulonglong *)(*(longlong *)(lVar38 + 0x38) + *(longlong *)(lVar10 + 0x120) * 8) *
                 0x30;
        local_148 = *(longlong *)(lVar43 + 0x10 + lVar35);
        local_170 = *(longlong *)(lVar43 + 0x28 + lVar35);
        local_278 = *(longlong *)(lVar38 + 8);
        uVar30 = 0;
        local_110 = uVar30;
        if (!bVar17) goto LAB_1422d0c4d;
LAB_1422d0a1b:
        pauVar41 = (undefined1 (*) [16])(uVar30 * 0x20 + local_1b8);
        puVar27 = (undefined4 *)(local_268 + uVar30 * 4);
      }
      else {
        uVar30 = local_110 & 0xffffffff;
        if (bVar17) goto LAB_1422d0a1b;
LAB_1422d0c4d:
        pauVar41 = (undefined1 (*) [16])0x0;
        puVar27 = local_2c0;
      }
      local_130 = *(undefined8 *)(local_278 + local_110 * 8);
      plVar28 = (longlong *)((longlong)local_118 + uVar30 * 0x18);
      puVar40 = (undefined8 *)(uVar30 * 0x30 + local_148);
      puVar29 = (undefined4 *)(local_170 + uVar30 * 4);
      local_2c0 = puVar27;
    }
    local_110 = local_110 + 1;
    if (*(char *)((longlong)plVar28 + 0x15) == '\0') {
      if (local_2d8 != 0) {
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
        bVar44 = (byte)(extraout_RAX_03 >> 0x39);
        auVar46 = ZEXT216(CONCAT11(bVar44,bVar44));
        auVar46 = pshuflw(auVar46,auVar46,0);
        lVar38 = 0;
        uVar30 = extraout_RAX_03;
        while( true ) {
          uVar30 = local_2e8 & uVar30;
          pcVar3 = *local_2f0 + uVar30;
          cVar45 = auVar46[0];
          auVar58[0] = -(*pcVar3 == cVar45);
          cVar47 = auVar46[1];
          auVar58[1] = -(pcVar3[1] == cVar47);
          cVar48 = auVar46[2];
          auVar58[2] = -(pcVar3[2] == cVar48);
          cVar49 = auVar46[3];
          auVar58[3] = -(pcVar3[3] == cVar49);
          auVar58[4] = -(pcVar3[4] == cVar45);
          auVar58[5] = -(pcVar3[5] == cVar47);
          auVar58[6] = -(pcVar3[6] == cVar48);
          auVar58[7] = -(pcVar3[7] == cVar49);
          auVar58[8] = -(pcVar3[8] == cVar45);
          auVar58[9] = -(pcVar3[9] == cVar47);
          auVar58[10] = -(pcVar3[10] == cVar48);
          auVar58[0xb] = -(pcVar3[0xb] == cVar49);
          auVar58[0xc] = -(pcVar3[0xc] == cVar45);
          auVar58[0xd] = -(pcVar3[0xd] == cVar47);
          auVar58[0xe] = -(pcVar3[0xe] == cVar48);
          auVar58[0xf] = -(pcVar3[0xf] == cVar49);
          uVar21 = (ushort)(SUB161(auVar58 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar58 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar58 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar58 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar58 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar58 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar58 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar58 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar58 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar58 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar58 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar58 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar58 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar58 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar58 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar58[0xf] >> 7) << 0xf;
          uVar26 = (uint)uVar21;
          if (uVar21 != 0) {
            do {
              uVar23 = 0;
              for (uVar15 = uVar26; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
                uVar23 = uVar23 + 1;
              }
              uVar42 = uVar23 + uVar30 & local_2e8;
              if (*plVar28 == *(longlong *)local_2f0[-1 - uVar42]) {
                lVar38 = *(longlong *)(local_2f0[-1 - uVar42] + 8);
                fVar51 = *(float *)(lVar38 + 0x4c) - *(float *)((longlong)puVar40 + 0x14);
                fVar55 = *(float *)(lVar38 + 0x48) - *(float *)(puVar40 + 2);
                fVar59 = *(float *)(lVar38 + 0x50) - *(float *)(puVar40 + 3);
                if (fVar19 < SQRT(fVar59 * fVar59 + fVar51 * fVar51 + fVar55 * fVar55))
                goto LAB_1422d10ff;
                goto LAB_1422d088d;
              }
              uVar21 = (ushort)(uVar26 - 1) & (ushort)uVar26;
              uVar26 = CONCAT22((short)(uVar26 - 1 >> 0x10),uVar21);
            } while (uVar21 != 0);
          }
          auVar53[0] = -(*pcVar3 == -1);
          auVar53[1] = -(pcVar3[1] == -1);
          auVar53[2] = -(pcVar3[2] == -1);
          auVar53[3] = -(pcVar3[3] == -1);
          auVar53[4] = -(pcVar3[4] == -1);
          auVar53[5] = -(pcVar3[5] == -1);
          auVar53[6] = -(pcVar3[6] == -1);
          auVar53[7] = -(pcVar3[7] == -1);
          auVar53[8] = -(pcVar3[8] == -1);
          auVar53[9] = -(pcVar3[9] == -1);
          auVar53[10] = -(pcVar3[10] == -1);
          auVar53[0xb] = -(pcVar3[0xb] == -1);
          auVar53[0xc] = -(pcVar3[0xc] == -1);
          auVar53[0xd] = -(pcVar3[0xd] == -1);
          auVar53[0xe] = -(pcVar3[0xe] == -1);
          auVar53[0xf] = -(pcVar3[0xf] == -1);
          if ((((((((((((((((SUB161(auVar53 >> 7,0) & 1) != 0 || (SUB161(auVar53 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar53 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar53 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar53 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar53 >> 0x2f,0) & 1) != 0) || (SUB161(auVar53 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar53 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar53 >> 0x47,0) & 1) != 0) || (SUB161(auVar53 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar53 >> 0x57,0) & 1) != 0) || (SUB161(auVar53 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar53 >> 0x67,0) & 1) != 0) || (SUB161(auVar53 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar53 >> 0x77,0) & 1) != 0) || auVar53[0xf] < '\0') break;
          uVar30 = uVar30 + lVar38 + 0x10;
          lVar38 = lVar38 + 0x10;
        }
      }
LAB_1422d10ff:
      plVar28 = *(longlong **)(in_R9 + 8);
      if (plVar28 == (longlong *)0x0) {
        plVar28 = *(longlong **)(in_R9 + 0x10);
      }
      lVar38 = plVar28[2];
      if ((ulonglong)(*plVar28 - lVar38) < 0x20) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
    }
    else {
      if (local_2d8 != 0) {
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
        bVar44 = (byte)(extraout_RAX_02 >> 0x39);
        auVar46 = ZEXT216(CONCAT11(bVar44,bVar44));
        auVar46 = pshuflw(auVar46,auVar46,0);
        lVar38 = 0;
        uVar30 = extraout_RAX_02;
        do {
          uVar30 = local_2e8 & uVar30;
          auVar58 = *(undefined1 (*) [16])(*local_2f0 + uVar30);
          cVar45 = auVar46[0];
          auVar57[0] = -(auVar58[0] == cVar45);
          cVar47 = auVar46[1];
          auVar57[1] = -(auVar58[1] == cVar47);
          cVar48 = auVar46[2];
          auVar57[2] = -(auVar58[2] == cVar48);
          cVar49 = auVar46[3];
          auVar57[3] = -(auVar58[3] == cVar49);
          auVar57[4] = -(auVar58[4] == cVar45);
          auVar57[5] = -(auVar58[5] == cVar47);
          auVar57[6] = -(auVar58[6] == cVar48);
          auVar57[7] = -(auVar58[7] == cVar49);
          auVar57[8] = -(auVar58[8] == cVar45);
          auVar57[9] = -(auVar58[9] == cVar47);
          auVar57[10] = -(auVar58[10] == cVar48);
          auVar57[0xb] = -(auVar58[0xb] == cVar49);
          auVar57[0xc] = -(auVar58[0xc] == cVar45);
          auVar57[0xd] = -(auVar58[0xd] == cVar47);
          auVar57[0xe] = -(auVar58[0xe] == cVar48);
          auVar57[0xf] = -(auVar58[0xf] == cVar49);
          uVar21 = (ushort)(SUB161(auVar57 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar57 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar57 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar57 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar57 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar57 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar57 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar57 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar57 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar57 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar57 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar57 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar57 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar57 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar57 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar57[0xf] >> 7) << 0xf;
          uVar26 = (uint)uVar21;
          if (uVar21 != 0) {
            do {
              uVar23 = 0;
              for (uVar15 = uVar26; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
                uVar23 = uVar23 + 1;
              }
              uVar42 = uVar23 + uVar30 & local_2e8;
              if (*plVar28 == *(longlong *)local_2f0[-1 - uVar42]) {
                uVar9 = *(undefined4 *)(*(longlong *)(local_2f0[-1 - uVar42] + 8) + 0x50);
                plStack_240 = (longlong *)CONCAT44(plStack_240._4_4_,uVar9);
                uVar11 = *(undefined8 *)(*(longlong *)(local_2f0[-1 - uVar42] + 8) + 0x48);
                *puVar29 = uVar8;
                *(undefined4 *)(puVar40 + 3) = uVar9;
                puVar40[2] = uVar11;
                manage_autoclutter_entities::closure_0();
                *puVar29 = uVar8;
                *puVar40 = uVar11;
                puVar40[1] = plStack_240;
                if (pauVar41 == (undefined1 (*) [16])0x0) goto LAB_1422d088d;
                lVar38 = *(longlong *)(local_2f0[-1 - uVar42] + 8);
                fVar59 = (float)((ulonglong)*(undefined8 *)(lVar38 + 0x54) >> 0x20);
                fVar51 = *(float *)(lVar38 + 0x5c);
                fVar60 = fVar59 * fVar62;
                fVar55 = (float)*(undefined8 *)(lVar38 + 0x54);
                fVar50 = fVar55 * 0.0 + fVar51 * fVar62;
                fVar61 = (float)((uint)fVar55 ^ uVar22) - fVar60;
                fVar60 = fVar60 + fVar51;
                fVar54 = fVar18 / SQRT(fVar61 * fVar61 + fVar50 * fVar50 + fVar60 * fVar60);
                fVar60 = fVar60 * fVar54;
                fVar50 = fVar50 * fVar54;
                fVar54 = fVar54 * fVar61;
                fVar61 = fVar59 * fVar54 - fVar51 * fVar50;
                fVar63 = fVar51 * fVar60 - fVar55 * fVar54;
                fVar65 = fVar55 * fVar50 - fVar59 * fVar60;
                fVar64 = fVar18 / SQRT(fVar65 * fVar65 + fVar63 * fVar63 + fVar61 * fVar61);
                fVar61 = fVar61 * fVar64;
                fVar65 = fVar64 * fVar65;
                if (fVar51 <= 0.0) {
                  fVar60 = fVar63 * fVar64 - fVar60;
                  if (0.0 < fVar60) {
                    fVar63 = fVar50 + fVar61;
                    fVar64 = (fVar18 - fVar51) + fVar60;
                    fVar60 = fVar59 + fVar65;
                    fVar50 = fVar55 - fVar54;
                    fVar51 = fVar64;
                    goto LAB_1422d11e3;
                  }
                  fVar60 = (fVar18 - fVar51) - fVar60;
                  fVar51 = fVar56 / SQRT(fVar60);
                  auVar46._0_4_ = fVar51 * fVar60;
                  auVar46._4_4_ = fVar51 * (fVar50 + fVar61);
                  auVar46._8_4_ = fVar51 * (fVar55 + fVar54);
                  auVar46._12_4_ = fVar51 * (fVar65 - fVar59);
                }
                else {
                  fVar60 = fVar60 + fVar63 * fVar64;
                  if (fVar60 <= 0.0) {
                    fVar63 = fVar55 + fVar54;
                    fVar64 = fVar59 + fVar65;
                    fVar60 = (fVar51 + fVar18) - fVar60;
                    fVar50 = fVar50 - fVar61;
                    fVar51 = fVar60;
LAB_1422d11e3:
                    fVar51 = fVar56 / SQRT(fVar51);
                    auVar46._0_4_ = fVar51 * fVar63;
                    auVar46._4_4_ = fVar51 * fVar64;
                    auVar46._8_4_ = fVar51 * fVar60;
                    auVar46._12_4_ = fVar51 * fVar50;
                  }
                  else {
                    fVar60 = fVar60 + fVar51 + fVar18;
                    fVar51 = fVar56 / SQRT(fVar60);
                    auVar46._0_4_ = fVar51 * (fVar65 - fVar59);
                    auVar46._4_4_ = fVar51 * (fVar55 - fVar54);
                    auVar46._8_4_ = fVar51 * (fVar50 - fVar61);
                    auVar46._12_4_ = fVar51 * fVar60;
                  }
                }
                *puVar27 = uVar8;
                *pauVar41 = auVar46;
                bVar44 = *(char *)(lVar38 + 0x30) - 3;
                if ((*(char *)(lVar38 + 0x30) == '\x02') || (bVar44 < 4 && bVar44 != 1))
                goto LAB_1422d088d;
                plVar28 = *(longlong **)(in_R9 + 8);
                if (plVar28 == (longlong *)0x0) {
                  plVar28 = *(longlong **)(in_R9 + 0x10);
                }
                uVar9 = *(undefined4 *)(lVar38 + 0x34);
                lVar38 = plVar28[2];
                if ((ulonglong)(*plVar28 - lVar38) < 0x20) {
                  alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
                }
                lVar43 = plVar28[1];
                *(code **)(lVar43 + lVar38) =
                     core::ops::function::FnOnce::
                     call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::color_id::ColorId>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
                ;
                *(code **)(lVar43 + 8 + lVar38) = bevy_ecs::error::handler::panic;
                *(longlong *)(lVar43 + 0x10 + lVar38) = local_130;
                *(undefined4 *)(lVar43 + 0x18 + lVar38) = uVar9;
                puVar24 = (undefined1 *)(lVar43 + lVar38 + 0x1c);
                goto LAB_1422d1160;
              }
              uVar21 = (ushort)(uVar26 - 1) & (ushort)uVar26;
              uVar26 = CONCAT22((short)(uVar26 - 1 >> 0x10),uVar21);
            } while (uVar21 != 0);
          }
          auVar52[0] = -(auVar58[0] == -1);
          auVar52[1] = -(auVar58[1] == -1);
          auVar52[2] = -(auVar58[2] == -1);
          auVar52[3] = -(auVar58[3] == -1);
          auVar52[4] = -(auVar58[4] == -1);
          auVar52[5] = -(auVar58[5] == -1);
          auVar52[6] = -(auVar58[6] == -1);
          auVar52[7] = -(auVar58[7] == -1);
          auVar52[8] = -(auVar58[8] == -1);
          auVar52[9] = -(auVar58[9] == -1);
          auVar52[10] = -(auVar58[10] == -1);
          auVar52[0xb] = -(auVar58[0xb] == -1);
          auVar52[0xc] = -(auVar58[0xc] == -1);
          auVar52[0xd] = -(auVar58[0xd] == -1);
          auVar52[0xe] = -(auVar58[0xe] == -1);
          auVar52[0xf] = -(auVar58[0xf] == -1);
          if ((((((((((((((((SUB161(auVar52 >> 7,0) & 1) != 0 || (SUB161(auVar52 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar52 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar52 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar52 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar52 >> 0x2f,0) & 1) != 0) || (SUB161(auVar52 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar52 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar52 >> 0x47,0) & 1) != 0) || (SUB161(auVar52 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar52 >> 0x57,0) & 1) != 0) || (SUB161(auVar52 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar52 >> 0x67,0) & 1) != 0) || (SUB161(auVar52 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar52 >> 0x77,0) & 1) != 0) || auVar52[0xf] < '\0') break;
          uVar30 = uVar30 + lVar38 + 0x10;
          lVar38 = lVar38 + 0x10;
        } while( true );
      }
      plVar28 = *(longlong **)(in_R9 + 8);
      if (plVar28 == (longlong *)0x0) {
        plVar28 = *(longlong **)(in_R9 + 0x10);
      }
      lVar38 = plVar28[2];
      if ((ulonglong)(*plVar28 - lVar38) < 0x20) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
    }
    lVar43 = plVar28[1];
    *(code **)(lVar43 + lVar38) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::clutter_meshes::ClutterMarkedForDeletion>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar43 + 8 + lVar38) = bevy_ecs::error::handler::panic;
    *(longlong *)(lVar43 + 0x10 + lVar38) = local_130;
    puVar24 = (undefined1 *)(lVar43 + lVar38 + 0x18);
LAB_1422d1160:
    *puVar24 = 0;
    plVar28[2] = lVar38 + 0x20;
  } while( true );
LAB_1422d16bd:
  do {
    if ((char)local_120[0x21] == '\0') {
      if (uVar42 == uVar30) {
        do {
          if (puVar25 == puVar2) goto LAB_1422d1810;
          uVar23 = *puVar25;
          puVar25 = puVar25 + 1;
          uVar30 = *(ulonglong *)(*(longlong *)(lVar10 + 0x100) + 0x58 + (ulonglong)uVar23 * 0xa0);
        } while (uVar30 == 0);
        lVar33 = *(longlong *)(lVar10 + 0x100) + (ulonglong)uVar23 * 0xa0;
        uVar42 = (ulonglong)*(uint *)(lVar33 + 0x94);
        if ((ulonglong)local_120[0x22] <
            *(ulonglong *)(*(longlong *)(lVar10 + 0x1a8) + 0x40 + uVar42 * 0x48)) {
          lVar35 = *(longlong *)(lVar10 + 0x1a8) + uVar42 * 0x48;
          uVar42 = *(ulonglong *)(*(longlong *)(lVar35 + 0x38) + local_120[0x22] * 8);
          if (uVar42 == 0) goto LAB_1422d17e8;
          lVar35 = *(longlong *)(*(longlong *)(lVar35 + 0x18) + 0x10 + ~uVar42 * 0x30);
        }
        else {
LAB_1422d17e8:
          lVar35 = 0;
        }
        lVar33 = *(longlong *)(lVar33 + 0x50);
        uVar42 = 0;
      }
      uVar36 = (ulonglong)*(uint *)(lVar33 + 8 + uVar42 * 0x10);
    }
    else {
      if (uVar42 == uVar30) {
        do {
          if (puVar25 == puVar2) goto LAB_1422d1810;
          uVar23 = *puVar25;
          puVar25 = puVar25 + 1;
          uVar30 = *(ulonglong *)(*(longlong *)(lVar10 + 0x1a8) + 0x10 + (ulonglong)uVar23 * 0x48);
        } while (uVar30 == 0);
        lVar35 = *(longlong *)(lVar10 + 0x1a8) + (ulonglong)uVar23 * 0x48;
        if (((ulonglong)local_120[0x22] < *(ulonglong *)(lVar35 + 0x40)) &&
           (uVar42 = *(ulonglong *)(*(longlong *)(lVar35 + 0x38) + local_120[0x22] * 8), uVar42 != 0
           )) {
          lVar35 = *(longlong *)(*(longlong *)(lVar35 + 0x18) + 0x10 + ~uVar42 * 0x30);
        }
        else {
          lVar35 = 0;
        }
        uVar42 = 0;
      }
      uVar36 = uVar42 & 0xffffffff;
    }
    lVar37 = uVar36 * 0x30 + lVar35;
    uVar42 = uVar42 + 1;
    fVar56 = *(float *)(lVar37 + 0x14) - *(float *)(lVar43 + 0x4c);
    fVar62 = *(float *)(lVar37 + 0x10) - *(float *)(lVar43 + 0x48);
    fVar51 = *(float *)(lVar37 + 0x18) - *(float *)(lVar43 + 0x50);
  } while (*(float *)(*(longlong *)(lVar43 + 0x38) + 0x40) + fVar18 <=
           SQRT(fVar51 * fVar51 + fVar56 * fVar56 + fVar62 * fVar62));
  goto LAB_1422d14de;
LAB_1422d1810:
  if (local_250 == 0) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
              ();
  }
  *(longlong *)(local_250 * 0x10 + 8) = lVar38;
  *(longlong *)(local_250 * 0x10 + 0x10) = lVar43;
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
  auVar46 = *local_2f0;
  uVar22 = ~(uint)(ushort)((ushort)(SUB161(auVar46 >> 7,0) & 1) |
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
                          (ushort)(byte)(auVar46[0xf] >> 7) << 0xf);
  pauVar41 = local_2f0 + 1;
  local_120 = (longlong *)*in_stack_00000028;
  lVar10 = in_stack_00000028[1];
  uVar32 = local_308 & 0xfffffffffffffff8;
  uVar26 = (uint)local_308 & 7;
  local_118 = (longlong *)(ulonglong)uVar26;
LAB_1422d14de:
  fVar51 = __real_3f4ccccd;
  fVar62 = __real_3e4ccccd;
  fVar56 = __real_3e99999a;
  if (local_130 != 0) {
    if ((short)uVar22 == 0) {
      do {
        auVar46 = *pauVar41;
        uVar21 = (ushort)(SUB161(auVar46 >> 7,0) & 1) | (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1
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
                 (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar46[0xf] >> 7) << 0xf;
        local_2f0 = local_2f0 + -0x10;
        pauVar41 = pauVar41 + 1;
      } while (uVar21 == 0xffff);
      uVar23 = ~(uint)uVar21;
      uVar22 = uVar23 & -(uint)uVar21 - 2;
    }
    else {
      uVar23 = uVar22;
      uVar22 = uVar22 - 1 & uVar22;
    }
    local_130 = local_130 + -1;
    iVar14 = 0;
    for (; (uVar23 & 1) == 0; uVar23 = uVar23 >> 1 | 0x80000000) {
      iVar14 = iVar14 + 1;
    }
    lVar38 = *(longlong *)((longlong)local_2f0 + (-0x10 - (ulonglong)(uint)(iVar14 << 4)));
    lVar43 = *(longlong *)((longlong)local_2f0 + (-8 - (ulonglong)(uint)(iVar14 << 4)));
    uVar30 = 0;
    while (uVar32 != uVar30) {
      lVar35 = uVar30 * 8;
      if (((((*(longlong *)(local_310 + uVar30 * 8) == lVar38) ||
            (*(longlong *)(local_310 + 8 + uVar30 * 8) == lVar38)) ||
           (*(longlong *)(local_310 + 0x10 + uVar30 * 8) == lVar38)) ||
          ((*(longlong *)(local_310 + 0x18 + uVar30 * 8) == lVar38 ||
           (*(longlong *)(local_310 + 0x20 + uVar30 * 8) == lVar38)))) ||
         ((*(longlong *)(local_310 + 0x28 + uVar30 * 8) == lVar38 ||
          ((*(longlong *)(local_310 + 0x30 + uVar30 * 8) == lVar38 ||
           (uVar30 = uVar30 + 8, *(longlong *)(local_310 + 0x38 + lVar35) == lVar38))))))
      goto LAB_1422d14de;
    }
    if ((local_118 == (longlong *)0x0) ||
       ((*(longlong *)(local_310 + uVar32 * 8) != lVar38 &&
        ((uVar26 == 1 ||
         ((*(longlong *)(local_310 + 8 + uVar32 * 8) != lVar38 &&
          ((uVar26 == 2 ||
           ((*(longlong *)(local_310 + 0x10 + uVar32 * 8) != lVar38 &&
            ((uVar26 == 3 ||
             ((*(longlong *)(local_310 + 0x18 + uVar32 * 8) != lVar38 &&
              ((uVar26 == 4 ||
               ((*(longlong *)(local_310 + 0x20 + uVar32 * 8) != lVar38 &&
                ((uVar26 == 5 ||
                 ((*(longlong *)(local_310 + 0x28 + uVar32 * 8) != lVar38 &&
                  ((uVar26 == 6 ||
                   ((*(longlong *)(local_310 + 0x30 + uVar32 * 8) != lVar38 && (uVar26 == 7)))))))))
                ))))))))))))))))))) {
      puVar25 = (uint *)local_120[0x1f];
      puVar2 = puVar25 + local_120[0x20];
      lVar33 = 8;
      uVar42 = 0;
      uVar30 = 0;
      lVar35 = 0;
      goto LAB_1422d16bd;
    }
    goto LAB_1422d14de;
  }
  plStack_240 = (longlong *)(local_248 + 0x1a0);
  for (; uVar11 = local_228, plVar20 = local_238, plVar28 = plStack_240, lVar10 = local_248,
      local_258 != (undefined8 *)(local_250 * 0x10 + 8); local_258 = local_258 + 2) {
    uVar12 = *local_258;
    puVar40 = (undefined8 *)local_258[1];
    manage_autoclutter_entities::closure_0();
    bVar44 = *(char *)(puVar40 + 6) - 3;
    if (bVar44 == 1 || 3 < bVar44) {
      fastrand::Rng::f32();
      fVar59 = (fVar18 - extraout_XMM0_Da) * 0.0;
      fVar55 = extraout_XMM0_Da * fVar19;
    }
    else {
      if (*(char *)(puVar40 + 6) == '\x06') {
        fastrand::Rng::f32();
        fVar59 = (fVar18 - extraout_XMM0_Da_00) * fVar62;
        fVar55 = extraout_XMM0_Da_00;
      }
      else {
        if (4 < local_250) {
          fastrand::Rng::f32();
          fVar59 = (fVar18 - extraout_XMM0_Da_02) * fVar56;
          fVar55 = extraout_XMM0_Da_02 * fVar51;
          goto LAB_1422d1af0;
        }
        fastrand::Rng::f32();
        fVar59 = (fVar18 - extraout_XMM0_Da_01) * fVar19;
        fVar55 = extraout_XMM0_Da_01;
      }
      fVar55 = fVar55 * fVar56;
    }
LAB_1422d1af0:
    cVar31 = *(char *)(puVar40 + 6);
    uVar7 = *(undefined1 *)(puVar40[7] + 0x44);
    uVar8 = *(undefined4 *)(puVar40 + 0xc);
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar39 = plStack_240;
    if (plStack_240 == (longlong *)0x0) {
      plVar39 = local_238;
    }
    lVar38 = plVar39[2];
    if ((ulonglong)(*plVar39 - lVar38) < 0x38) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar43 = plVar39[1];
    *(code **)(lVar43 + lVar38) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::AutoClutter>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar43 + 8 + lVar38) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar43 + 0x10 + lVar38) = local_228;
    *(undefined8 *)(lVar43 + 0x18 + lVar38) = uVar12;
    *(float *)(lVar43 + 0x20 + lVar38) = fVar55 + fVar59;
    *(undefined4 *)(lVar43 + 0x24 + lVar38) = 0;
    *(undefined4 *)(lVar43 + 0x28 + lVar38) = uVar8;
    *(undefined1 *)(lVar43 + 0x2c + lVar38) = uVar7;
    *(bool *)(lVar43 + 0x2d + lVar38) = 3 < (byte)(cVar31 - 3U) || (byte)(cVar31 - 3U) == 1;
    *(undefined1 *)(lVar43 + 0x30 + lVar38) = 0;
    plVar39[2] = lVar38 + 0x38;
    if ((*(byte *)(puVar40 + 6) & 6) != 4 && 2 < *(byte *)(puVar40 + 6)) {
      switch(*puVar40) {
      case 3:
        local_118 = (longlong *)puVar40[1];
        local_120 = (longlong *)
                    ((ulonglong)local_120 & 0xffffffff00000000 | (ulonglong)*(uint *)(puVar40 + 2));
        break;
      case 4:
        local_118 = (longlong *)puVar40[1];
        local_120 = (longlong *)puVar40[2];
        LOCK();
        lVar38 = *local_118;
        *local_118 = *local_118 + 1;
        UNLOCK();
        if (*local_118 == 0 || SCARRY8(lVar38,1) != *local_118 < 0) {
                    /* WARNING: Does not return */
          pcVar13 = (code *)invalidInstructionException();
          (*pcVar13)();
        }
        break;
      case 5:
        break;
      default:
        local_118 = (longlong *)puVar40[1];
        local_120 = (longlong *)
                    ((ulonglong)local_120 & 0xffffffff00000000 | (ulonglong)*(uint *)(puVar40 + 2));
      }
      local_228 = puVar40[4];
      bevy_ecs::system::commands::EntityCommands::
      insert<country_core::components::AutoClutterConvertableToDecoClutter>();
      plStack_240 = local_118;
      local_238 = local_120;
    }
    lVar38 = puVar40[7];
    plVar39 = plVar28;
    if (*(char *)(lVar38 + 0x45) == '\t') {
      cVar31 = *(char *)(puVar40 + 6);
      if ((cVar31 == '\x02') || ((byte)(cVar31 - 3U) < 4 && (byte)(cVar31 - 3U) != 1)) {
        if (cVar31 == '\x05') {
          uVar12 = *(undefined8 *)(lVar38 + 0x38);
          uVar8 = *(undefined4 *)(puVar40 + 10);
          plStack_240 = (longlong *)CONCAT44(plStack_240._4_4_,uVar8);
          lVar38 = puVar40[9];
          bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
          if (plVar28 == (longlong *)0x0) {
            plVar39 = plVar20;
          }
          lVar43 = plVar39[2];
          if ((ulonglong)(*plVar39 - lVar43) < 0x88) {
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          lVar35 = plVar39[1];
          *(code **)(lVar35 + lVar43) =
               core::ops::function::FnOnce::
               call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::PlantInstanceData>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          plVar4 = (longlong *)(lVar35 + 8 + lVar43);
          *plVar4 = local_248;
          plVar4[1] = (longlong)plVar28;
          *(longlong *)(lVar35 + 0x18 + lVar43) = lVar38;
          *(undefined4 *)(lVar35 + 0x20 + lVar43) = uVar8;
          *(undefined8 *)(lVar35 + 0x24 + lVar43) = 0;
          *(undefined4 *)(lVar35 + 0x2c + lVar43) = 0;
          *(undefined4 *)(lVar35 + 0x38 + lVar43) = 0;
          *(undefined8 *)(lVar35 + 0x40 + lVar43) = uVar12;
          *(undefined1 *)(lVar35 + 0x48 + lVar43) = 10;
          local_248 = lVar38;
        }
        else {
          uVar12 = *(undefined8 *)(lVar38 + 0x38);
          uVar8 = *(undefined4 *)(puVar40 + 10);
          plStack_240 = (longlong *)CONCAT44(plStack_240._4_4_,uVar8);
          lVar38 = puVar40[9];
          bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
          if (plVar28 == (longlong *)0x0) {
            plVar39 = plVar20;
          }
          lVar43 = plVar39[2];
          if ((ulonglong)(*plVar39 - lVar43) < 0x88) {
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          lVar35 = plVar39[1];
          *(code **)(lVar35 + lVar43) =
               core::ops::function::FnOnce::
               call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SolidVertexColorInstanceData>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          plVar4 = (longlong *)(lVar35 + 8 + lVar43);
          *plVar4 = local_248;
          plVar4[1] = (longlong)plVar28;
          *(longlong *)(lVar35 + 0x18 + lVar43) = lVar38;
          *(undefined4 *)(lVar35 + 0x20 + lVar43) = uVar8;
          *(undefined8 *)(lVar35 + 0x24 + lVar43) = 0;
          *(undefined4 *)(lVar35 + 0x2c + lVar43) = 0;
          *(undefined4 *)(lVar35 + 0x38 + lVar43) = 0;
          *(undefined8 *)(lVar35 + 0x40 + lVar43) = uVar12;
          *(undefined1 *)(lVar35 + 0x48 + lVar43) = 3;
          local_248 = lVar38;
        }
      }
      else {
        uVar12 = *(undefined8 *)(lVar38 + 0x38);
        uVar8 = *(undefined4 *)(puVar40 + 10);
        plStack_240 = (longlong *)CONCAT44(plStack_240._4_4_,uVar8);
        lVar38 = puVar40[9];
        bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
        if (plVar28 == (longlong *)0x0) {
          plVar39 = plVar20;
        }
        lVar43 = plVar39[2];
        if ((ulonglong)(*plVar39 - lVar43) < 0x88) {
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
        lVar35 = plVar39[1];
        *(code **)(lVar35 + lVar43) =
             core::ops::function::FnOnce::
             call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
        ;
        plVar4 = (longlong *)(lVar35 + 8 + lVar43);
        *plVar4 = local_248;
        plVar4[1] = (longlong)plVar28;
        *(longlong *)(lVar35 + 0x18 + lVar43) = lVar38;
        *(undefined4 *)(lVar35 + 0x20 + lVar43) = uVar8;
        *(undefined8 *)(lVar35 + 0x24 + lVar43) = 0;
        *(undefined4 *)(lVar35 + 0x2c + lVar43) = 0;
        *(undefined4 *)(lVar35 + 0x38 + lVar43) = 0;
        *(undefined8 *)(lVar35 + 0x40 + lVar43) = uVar12;
        *(undefined1 *)(lVar35 + 0x48 + lVar43) = 4;
        local_248 = lVar38;
      }
      *(undefined1 *)(lVar35 + 0x58 + lVar43) = 0;
      *(undefined8 *)(lVar35 + 0x68 + lVar43) = uVar11;
      *(code **)(lVar35 + 0x78 + lVar43) = bevy_ecs::error::handler::panic;
      lVar43 = lVar43 + 0x88;
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
        uVar12 = *(undefined8 *)(puVar40[7] + 0x38);
        uVar8 = *(undefined4 *)(puVar40 + 10);
        lVar38 = puVar40[9];
        bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
        cVar31 = *(char *)(puVar40[7] + 0x45);
        if (cVar31 == '\t') {
                    /* WARNING: Subroutine does not return */
          core::option::unwrap_failed();
        }
        if (plVar28 == (longlong *)0x0) {
          plVar39 = plVar20;
        }
        lVar43 = plVar39[2];
        if ((ulonglong)(*plVar39 - lVar43) < 0x98) {
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
        lVar35 = plVar39[1];
        *(code **)(lVar35 + lVar43) =
             core::ops::function::FnOnce::
             call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SolidAlbedoMapInstanceData>,country_core::components::drawable::NaniSolidTexture>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
        ;
        plVar4 = (longlong *)(lVar35 + 8 + lVar43);
        *plVar4 = local_248;
        plVar4[1] = (longlong)plVar28;
        *(longlong *)(lVar35 + 0x18 + lVar43) = lVar38;
        *(undefined4 *)(lVar35 + 0x20 + lVar43) = uVar8;
        *(undefined8 *)(lVar35 + 0x24 + lVar43) = 0;
        *(undefined4 *)(lVar35 + 0x2c + lVar43) = 0;
        *(undefined4 *)(lVar35 + 0x38 + lVar43) = 0;
        *(undefined8 *)(lVar35 + 0x40 + lVar43) = uVar12;
        *(undefined1 *)(lVar35 + 0x48 + lVar43) = 0xf;
        local_248 = lVar38;
      }
      else {
LAB_1422d1f30:
        uVar12 = *(undefined8 *)(puVar40[7] + 0x38);
        uVar8 = *(undefined4 *)(puVar40 + 10);
        lVar38 = puVar40[9];
        bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
        cVar31 = *(char *)(puVar40[7] + 0x45);
        if (cVar31 == '\t') {
                    /* WARNING: Subroutine does not return */
          core::option::unwrap_failed();
        }
        if (plVar28 == (longlong *)0x0) {
          plVar39 = plVar20;
        }
        lVar43 = plVar39[2];
        if ((ulonglong)(*plVar39 - lVar43) < 0x98) {
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
        lVar35 = plVar39[1];
        *(code **)(lVar35 + lVar43) =
             core::ops::function::FnOnce::
             call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SolidAlbedoMapEmissiveInstanceData>,country_core::components::drawable::NaniSolidTexture>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
        ;
        plVar4 = (longlong *)(lVar35 + 8 + lVar43);
        *plVar4 = local_248;
        plVar4[1] = (longlong)plVar28;
        *(longlong *)(lVar35 + 0x18 + lVar43) = lVar38;
        *(undefined4 *)(lVar35 + 0x20 + lVar43) = uVar8;
        *(undefined8 *)(lVar35 + 0x24 + lVar43) = 0;
        *(undefined4 *)(lVar35 + 0x2c + lVar43) = 0;
        *(undefined4 *)(lVar35 + 0x38 + lVar43) = 0;
        *(undefined8 *)(lVar35 + 0x40 + lVar43) = uVar12;
        *(undefined1 *)(lVar35 + 0x48 + lVar43) = 0x10;
        local_248 = lVar38;
      }
      plStack_240 = (longlong *)CONCAT44(plStack_240._4_4_,uVar8);
      *(char *)(lVar35 + 0x58 + lVar43) = cVar31;
      *(undefined1 *)(lVar35 + 0x68 + lVar43) = 0;
      *(undefined8 *)(lVar35 + 0x78 + lVar43) = uVar11;
      *(code **)(lVar35 + 0x88 + lVar43) = bevy_ecs::error::handler::panic;
      lVar43 = lVar43 + 0x98;
    }
    plVar39[2] = lVar43;
    cVar31 = *(char *)(puVar40 + 6);
    if ((cVar31 != '\x02') && (3 < (byte)(cVar31 - 3U) || (byte)(cVar31 - 3U) == 1)) {
      plVar39 = plVar28;
      if (plVar28 == (longlong *)0x0) {
        plVar39 = plVar20;
      }
      lVar38 = plVar39[2];
      if ((ulonglong)(*plVar39 - lVar38) < 0x58) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar43 = plVar39[1];
      *(code **)(lVar43 + lVar38) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<system_clutter::autoclutter::WindowShutter>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      plVar4 = (longlong *)(lVar43 + 8 + lVar38);
      *plVar4 = lVar10;
      plVar4[1] = (longlong)plVar28;
      *(char *)(lVar43 + 0x18 + lVar38) = cVar31;
      *(undefined1 *)(lVar43 + 0x28 + lVar38) = 0;
      *(undefined8 *)(lVar43 + 0x38 + lVar38) = uVar11;
      *(code **)(lVar43 + 0x48 + lVar38) = bevy_ecs::error::handler::panic;
      plVar39[2] = lVar38 + 0x58;
      if (plVar28 == (longlong *)0x0) {
        plVar28 = plVar20;
      }
      uVar8 = *(undefined4 *)((longlong)puVar40 + 0x34);
      lVar10 = plVar28[2];
      if ((ulonglong)(*plVar28 - lVar10) < 0x20) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar38 = plVar28[1];
      *(code **)(lVar38 + lVar10) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::color_id::ColorId>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      *(code **)(lVar38 + 8 + lVar10) = bevy_ecs::error::handler::panic;
      *(undefined8 *)(lVar38 + 0x10 + lVar10) = uVar11;
      *(undefined4 *)(lVar38 + 0x18 + lVar10) = uVar8;
      *(undefined1 *)(lVar38 + 0x1c + lVar10) = 0;
      plVar28[2] = lVar10 + 0x20;
    }
  }
  if (local_318 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if ((local_2e8 != 0) && (local_2e8 * 0x11 != -0x21)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (bVar16) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_04 == 1) {
      plVar28 = extraout_RAX_04 + 1;
    }
    else if ((*extraout_RAX_04 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar28 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0)) {
LAB_1422d2634:
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    if (*plVar28 != 0) {
LAB_1422d2640:
                    /* WARNING: Subroutine does not return */
      core::cell::panic_already_borrowed();
    }
    *plVar28 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar28 = *plVar28 + 1;
  }
  return;
}

