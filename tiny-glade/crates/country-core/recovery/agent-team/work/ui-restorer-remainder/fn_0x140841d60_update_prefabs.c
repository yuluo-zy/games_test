void __cdecl update_prefabs(void)


void __cdecl country_core::systems::prefab::update_prefabs(void)

{
  byte *pbVar1;
  uint *puVar2;
  float fVar3;
  longlong lVar4;
  byte bVar5;
  byte bVar6;
  char cVar7;
  int iVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  longlong lVar11;
  longlong lVar12;
  undefined8 uVar13;
  undefined4 *puVar14;
  longlong lVar15;
  undefined4 *puVar16;
  uint uVar17;
  uint uVar18;
  int iVar19;
  int iVar20;
  int iVar21;
  int iVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  bool bVar26;
  char extraout_AL;
  uint uVar27;
  uint uVar28;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong *plVar29;
  longlong extraout_RAX_03;
  ulonglong extraout_RAX_04;
  longlong lVar30;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  float *extraout_RAX_07;
  undefined8 *puVar31;
  longlong *extraout_RAX_08;
  longlong *extraout_RAX_09;
  longlong *extraout_RAX_10;
  undefined4 uVar32;
  longlong *in_RCX;
  longlong *plVar33;
  longlong in_RDX;
  ulonglong uVar34;
  longlong lVar35;
  uint *puVar36;
  undefined8 *in_R8;
  undefined *puVar37;
  ulonglong uVar38;
  longlong *in_R9;
  ushort uVar39;
  longlong lVar40;
  float *pfVar41;
  byte bVar42;
  char cVar43;
  char cVar52;
  float fVar44;
  float fVar45;
  char cVar50;
  char cVar51;
  undefined1 auVar46 [16];
  undefined1 auVar47 [16];
  undefined1 auVar48 [16];
  undefined1 auVar49 [16];
  float fVar53;
  undefined1 auVar54 [16];
  float fVar55;
  float fVar57;
  undefined1 auVar56 [16];
  float fVar58;
  float fVar59;
  float fVar60;
  float fVar61;
  float fVar62;
  undefined1 unaff_XMM11_Ba;
  undefined1 unaff_XMM11_Bb;
  undefined1 unaff_XMM11_Bc;
  undefined1 unaff_XMM11_Bd;
  undefined1 unaff_XMM11_Be;
  undefined1 unaff_XMM11_Bf;
  undefined1 unaff_XMM11_Bg;
  undefined1 unaff_XMM11_Bh;
  undefined1 unaff_XMM11_Bi;
  undefined1 unaff_XMM11_Bj;
  undefined1 unaff_XMM11_Bk;
  undefined1 unaff_XMM11_Bl;
  undefined1 unaff_XMM11_Bm;
  undefined1 unaff_XMM11_Bn;
  undefined1 unaff_XMM11_Bo;
  undefined1 unaff_XMM11_Bp;
  float fVar63;
  float fVar64;
  float fVar65;
  float fVar66;
  longlong *in_stack_00000028;
  undefined8 local_208;
  undefined8 uStack_200;
  undefined8 local_1f8;
  longlong local_170;
  longlong local_158;
  longlong local_150;
  ulonglong local_148;
  longlong local_120;
  longlong local_118;
  
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar26 = false;
  }
  else {
    uVar34 = 0x2d;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar34 != 0) {
        if (uVar34 < 0x2d) {
          if ((char)(&DAT_142a93f50)[uVar34] < -0x40) {
LAB_140841e29:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar34 != 0x2d) goto LAB_140841e29;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar34 != 0xfffffffffffffffe)) {
        if (uVar34 + 2 < 0x2d) {
          if ((char)(&UNK_142a93f52)[uVar34] < -0x40) {
LAB_140841e87:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar34 + 2 != 0x2d) goto LAB_140841e87;
      }
    }
    puVar37 = &DAT_142a94181;
    do {
      bVar42 = puVar37[-1];
      uVar27 = (uint)(char)bVar42;
      if ((char)bVar42 < '\0') {
        bVar5 = puVar37[-2];
        if ((char)bVar5 < -0x40) {
          bVar6 = puVar37[-3];
          if ((char)bVar6 < -0x40) {
            pbVar1 = puVar37 + -4;
            puVar37 = puVar37 + -4;
            uVar27 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar37 = puVar37 + -3;
            uVar27 = bVar6 & 0xf;
          }
          uVar27 = bVar5 & 0x3f | uVar27 << 6;
        }
        else {
          puVar37 = puVar37 + -2;
          uVar27 = bVar5 & 0x1f;
        }
        uVar27 = (uint)(bVar42 & 0x3f) | uVar27 << 6;
        if (uVar27 != 0x5c) goto LAB_140841f54;
LAB_140841f66:
        if (puVar37 == &UNK_142a94157) break;
        if (puVar37 + -0x142a94157 < (undefined *)0x29) {
          if (-0x41 < (char)puVar37[1]) break;
        }
        else if (puVar37 + -0x142a94157 == (undefined *)0x29) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar37 = puVar37 + -1;
      if (uVar27 == 0x5c) goto LAB_140841f66;
LAB_140841f54:
      if (uVar27 == 0x2f) goto LAB_140841f66;
    } while (puVar37 != &DAT_142a94158);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar33 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar33 = extraout_RAX_10, extraout_RAX_10 == (longlong *)0x0)) {
LAB_1408429ab:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar33 != 0) {
LAB_1408429b7:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar33 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar33 = *plVar33 + 1;
    bVar26 = true;
  }
  **(undefined4 **)(in_RDX + 0x10) = *(undefined4 *)(in_RDX + 0x1c);
  lVar11 = *in_RCX;
  lVar12 = in_RCX[1];
  iVar8 = *(int *)((longlong)in_RCX + 0x14);
  puVar36 = *(uint **)(lVar11 + 0xf8);
  puVar2 = puVar36 + *(longlong *)(lVar11 + 0x100);
  plVar33 = (longlong *)*in_stack_00000028;
  uVar9 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  uVar13 = *in_R8;
  puVar14 = (undefined4 *)in_R8[2];
  uVar28 = iVar8 - (int)in_RCX[2];
  uVar27 = 0xc233b000;
  if (uVar28 < 0xc233b000) {
    uVar27 = uVar28;
  }
  uVar10 = *(undefined4 *)((longlong)in_R9 + 0x1c);
  lVar15 = *in_R9;
  puVar16 = (undefined4 *)in_R9[2];
  iVar19 = -(uint)(CONCAT13(unaff_XMM11_Bd,
                            CONCAT12(unaff_XMM11_Bc,CONCAT11(unaff_XMM11_Bb,unaff_XMM11_Ba))) ==
                  CONCAT13(unaff_XMM11_Bd,
                           CONCAT12(unaff_XMM11_Bc,CONCAT11(unaff_XMM11_Bb,unaff_XMM11_Ba))));
  iVar20 = -(uint)(CONCAT13(unaff_XMM11_Bh,
                            CONCAT12(unaff_XMM11_Bg,CONCAT11(unaff_XMM11_Bf,unaff_XMM11_Be))) ==
                  CONCAT13(unaff_XMM11_Bh,
                           CONCAT12(unaff_XMM11_Bg,CONCAT11(unaff_XMM11_Bf,unaff_XMM11_Be))));
  iVar21 = -(uint)(CONCAT13(unaff_XMM11_Bl,
                            CONCAT12(unaff_XMM11_Bk,CONCAT11(unaff_XMM11_Bj,unaff_XMM11_Bi))) ==
                  CONCAT13(unaff_XMM11_Bl,
                           CONCAT12(unaff_XMM11_Bk,CONCAT11(unaff_XMM11_Bj,unaff_XMM11_Bi))));
  iVar22 = -(uint)(CONCAT13(unaff_XMM11_Bp,
                            CONCAT12(unaff_XMM11_Bo,CONCAT11(unaff_XMM11_Bn,unaff_XMM11_Bm))) ==
                  CONCAT13(unaff_XMM11_Bp,
                           CONCAT12(unaff_XMM11_Bo,CONCAT11(unaff_XMM11_Bn,unaff_XMM11_Bm))));
  cVar7 = *(char *)(lVar11 + 0x108);
  local_170 = 8;
  local_158 = 8;
  local_120 = 0;
  local_118 = 0;
  local_150 = 0;
  local_148 = 0;
  uVar34 = 0;
joined_r0x000140842124:
  do {
    if (cVar7 == '\0') {
      if (uVar34 != local_148) goto LAB_1408422aa;
      while (puVar36 != puVar2) {
        uVar28 = *puVar36;
        puVar36 = puVar36 + 1;
        if (*(longlong *)(*(longlong *)(lVar12 + 0x100) + 0x58 + (ulonglong)uVar28 * 0xa0) == 0) {
          do {
            if (puVar36 == puVar2) goto LAB_1408428c8;
            uVar28 = *puVar36;
            puVar36 = puVar36 + 1;
          } while (*(longlong *)(*(longlong *)(lVar12 + 0x100) + 0x58 + (ulonglong)uVar28 * 0xa0) ==
                   0);
          lVar40 = *(longlong *)(lVar12 + 0x100) + (ulonglong)uVar28 * 0xa0;
        }
        else {
          lVar40 = *(longlong *)(lVar12 + 0x100) + (ulonglong)uVar28 * 0xa0;
        }
        lVar35 = *(longlong *)(lVar12 + 0x1a8) + (ulonglong)*(uint *)(lVar40 + 0x94) * 0x48;
        uVar34 = *(ulonglong *)
                  (*(longlong *)(lVar12 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar40 + 0x94) * 0x48
                  );
        if ((*(ulonglong *)(lVar11 + 0x110) < uVar34) &&
           (uVar38 = *(ulonglong *)
                      (*(longlong *)(lVar35 + 0x38) + *(ulonglong *)(lVar11 + 0x110) * 8),
           uVar38 != 0)) {
          local_120 = *(longlong *)(*(longlong *)(lVar35 + 0x18) + 0x10 + ~uVar38 * 0x30);
          uVar38 = *(ulonglong *)(lVar11 + 0x118);
          if (uVar34 <= uVar38) goto LAB_140842270;
LAB_14084223e:
          uVar34 = *(ulonglong *)(*(longlong *)(lVar35 + 0x38) + uVar38 * 8);
          if (uVar34 == 0) goto LAB_140842270;
          local_118 = *(longlong *)(*(longlong *)(lVar35 + 0x18) + 0x10 + ~uVar34 * 0x30);
        }
        else {
          local_120 = 0;
          uVar38 = *(ulonglong *)(lVar11 + 0x118);
          if (uVar38 < uVar34) goto LAB_14084223e;
LAB_140842270:
          local_118 = 0;
        }
        bevy_ecs::storage::table::Table::get_changed_ticks_slice_for();
        local_158 = *(longlong *)(lVar40 + 0x50);
        local_148 = *(ulonglong *)(lVar40 + 0x58);
        uVar34 = 0;
        local_150 = extraout_RAX_02;
LAB_1408422aa:
        do {
          uVar38 = (ulonglong)*(uint *)(local_158 + 8 + uVar34 * 0x10);
          if ((uint)(iVar8 - *(int *)(local_150 + uVar38 * 4)) < uVar27) {
            plVar29 = (longlong *)(uVar34 * 0x10 + local_158);
            goto LAB_140842487;
          }
          uVar34 = uVar34 + 1;
        } while (uVar34 != local_148);
      }
LAB_1408428c8:
      if (bVar26) {
        puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
        if (*extraout_RAX_08 == 1) {
          plVar33 = extraout_RAX_08 + 1;
        }
        else if ((*extraout_RAX_08 != 0) ||
                (std::sys::thread_local::native::lazy::Storage::
                 initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                           (), plVar33 = extraout_RAX_09, extraout_RAX_09 == (longlong *)0x0))
        goto LAB_1408429ab;
        if (*plVar33 != 0) goto LAB_1408429b7;
        *plVar33 = -1;
        puffin::ThreadProfiler::end_scope();
        *plVar33 = *plVar33 + 1;
      }
      return;
    }
    if (uVar34 != local_148) goto LAB_140842454;
LAB_140842300:
    if (puVar36 == puVar2) goto LAB_1408428c8;
    uVar34 = (ulonglong)*puVar36;
    puVar36 = puVar36 + 1;
    lVar40 = *(longlong *)(lVar12 + 0x1a8);
    if (*(longlong *)(lVar40 + 0x10 + uVar34 * 0x48) == 0) {
      do {
        if (puVar36 == puVar2) goto LAB_1408428c8;
        uVar28 = *puVar36;
        puVar36 = puVar36 + 1;
      } while (*(longlong *)(*(longlong *)(lVar12 + 0x1a8) + 0x10 + (ulonglong)uVar28 * 0x48) == 0);
      lVar35 = *(longlong *)(lVar12 + 0x1a8) + (ulonglong)uVar28 * 0x48;
      lVar40 = lVar35;
    }
    else {
      lVar35 = lVar40 + uVar34 * 0x48;
      lVar40 = lVar40 + uVar34 * 0x48;
    }
    uVar34 = *(ulonglong *)(lVar35 + 0x40);
    if ((*(ulonglong *)(lVar11 + 0x110) < uVar34) &&
       (uVar38 = *(ulonglong *)(*(longlong *)(lVar35 + 0x38) + *(ulonglong *)(lVar11 + 0x110) * 8),
       uVar38 != 0)) {
      local_120 = *(longlong *)(*(longlong *)(lVar35 + 0x18) + 0x10 + ~uVar38 * 0x30);
      uVar38 = *(ulonglong *)(lVar11 + 0x118);
      if (uVar34 <= uVar38) goto LAB_140842410;
LAB_1408423de:
      uVar34 = *(ulonglong *)(*(longlong *)(lVar35 + 0x38) + uVar38 * 8);
      if (uVar34 == 0) goto LAB_140842410;
      local_118 = *(longlong *)(*(longlong *)(lVar35 + 0x18) + 0x10 + ~uVar34 * 0x30);
    }
    else {
      local_120 = 0;
      uVar38 = *(ulonglong *)(lVar11 + 0x118);
      if (uVar38 < uVar34) goto LAB_1408423de;
LAB_140842410:
      local_118 = 0;
    }
    bevy_ecs::storage::table::Table::get_changed_ticks_slice_for();
    local_170 = *(longlong *)(lVar35 + 8);
    local_148 = *(ulonglong *)(lVar40 + 0x10);
    uVar34 = 0;
    local_150 = extraout_RAX_03;
LAB_140842454:
    while (uVar38 = uVar34 & 0xffffffff, uVar27 <= (uint)(iVar8 - *(int *)(local_150 + uVar38 * 4)))
    {
      uVar34 = uVar34 + 1;
      if (uVar34 == local_148) goto LAB_140842300;
    }
    plVar29 = (longlong *)(local_170 + uVar34 * 8);
LAB_140842487:
    lVar35 = uVar38 * 0x58 + local_120;
    pfVar41 = (float *)(uVar38 * 0x30 + local_118);
    lVar40 = *plVar29;
    uVar34 = uVar34 + 1;
    if (plVar33[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      bVar42 = (byte)(extraout_RAX_04 >> 0x39);
      auVar46 = ZEXT216(CONCAT11(bVar42,bVar42));
      auVar46 = pshuflw(auVar46,auVar46,0);
      lVar30 = 0;
      uVar38 = extraout_RAX_04;
      while( true ) {
        uVar38 = plVar33[1] & uVar38;
        auVar47 = *(undefined1 (*) [16])(*plVar33 + uVar38);
        cVar43 = auVar46[0];
        auVar56[0] = -(auVar47[0] == cVar43);
        cVar50 = auVar46[1];
        auVar56[1] = -(auVar47[1] == cVar50);
        cVar51 = auVar46[2];
        auVar56[2] = -(auVar47[2] == cVar51);
        cVar52 = auVar46[3];
        auVar56[3] = -(auVar47[3] == cVar52);
        auVar56[4] = -(auVar47[4] == cVar43);
        auVar56[5] = -(auVar47[5] == cVar50);
        auVar56[6] = -(auVar47[6] == cVar51);
        auVar56[7] = -(auVar47[7] == cVar52);
        auVar56[8] = -(auVar47[8] == cVar43);
        auVar56[9] = -(auVar47[9] == cVar50);
        auVar56[10] = -(auVar47[10] == cVar51);
        auVar56[0xb] = -(auVar47[0xb] == cVar52);
        auVar56[0xc] = -(auVar47[0xc] == cVar43);
        auVar56[0xd] = -(auVar47[0xd] == cVar50);
        auVar56[0xe] = -(auVar47[0xe] == cVar51);
        auVar56[0xf] = -(auVar47[0xf] == cVar52);
        uVar39 = (ushort)(SUB161(auVar56 >> 7,0) & 1) | (ushort)(SUB161(auVar56 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar56 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar56 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar56 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar56 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar56 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar56 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar56 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar56 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar56 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar56 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar56 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar56 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar56 >> 0x77,0) & 1) << 0xe | (ushort)(auVar56[0xf] >> 7) << 0xf
        ;
        uVar28 = (uint)uVar39;
        while (uVar39 != 0) {
          uVar17 = 0;
          for (uVar18 = uVar28; (uVar18 & 1) == 0; uVar18 = uVar18 >> 1 | 0x80000000) {
            uVar17 = uVar17 + 1;
          }
          if (lVar40 == *(longlong *)(*plVar33 + (uVar17 + uVar38 & plVar33[1]) * -8 + -8))
          goto joined_r0x000140842124;
          uVar39 = (ushort)(uVar28 - 1) & (ushort)uVar28;
          uVar28 = CONCAT22((short)(uVar28 - 1 >> 0x10),uVar39);
        }
        auVar54[0] = -(auVar47[0] == (char)iVar19);
        auVar54[1] = -(auVar47[1] == (char)((uint)iVar19 >> 8));
        auVar54[2] = -(auVar47[2] == (char)((uint)iVar19 >> 0x10));
        auVar54[3] = -(auVar47[3] == (char)((uint)iVar19 >> 0x18));
        auVar54[4] = -(auVar47[4] == (char)iVar20);
        auVar54[5] = -(auVar47[5] == (char)((uint)iVar20 >> 8));
        auVar54[6] = -(auVar47[6] == (char)((uint)iVar20 >> 0x10));
        auVar54[7] = -(auVar47[7] == (char)((uint)iVar20 >> 0x18));
        auVar54[8] = -(auVar47[8] == (char)iVar21);
        auVar54[9] = -(auVar47[9] == (char)((uint)iVar21 >> 8));
        auVar54[10] = -(auVar47[10] == (char)((uint)iVar21 >> 0x10));
        auVar54[0xb] = -(auVar47[0xb] == (char)((uint)iVar21 >> 0x18));
        auVar54[0xc] = -(auVar47[0xc] == (char)iVar22);
        auVar54[0xd] = -(auVar47[0xd] == (char)((uint)iVar22 >> 8));
        auVar54[0xe] = -(auVar47[0xe] == (char)((uint)iVar22 >> 0x10));
        auVar54[0xf] = -(auVar47[0xf] == (char)((uint)iVar22 >> 0x18));
        if ((((((((((((((((SUB161(auVar54 >> 7,0) & 1) != 0 || (SUB161(auVar54 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar54 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar54 >> 0x1f,0) & 1) != 0) || (SUB161(auVar54 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar54 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar54 >> 0x37,0) & 1) != 0) || (SUB161(auVar54 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar54 >> 0x47,0) & 1) != 0) || (SUB161(auVar54 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar54 >> 0x57,0) & 1) != 0) || (SUB161(auVar54 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar54 >> 0x67,0) & 1) != 0) || (SUB161(auVar54 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar54 >> 0x77,0) & 1) != 0) || auVar54[0xf] < '\0') break;
        uVar38 = uVar38 + lVar30 + 0x10;
        lVar30 = lVar30 + 0x10;
      }
    }
    lVar30 = *(longlong *)(lVar35 + 8);
    lVar40 = lVar30 + *(longlong *)(lVar35 + 0x10) * 0xc;
    while (lVar30 != lVar40) {
      country_core::render::nani::gpu_world::GpuWorld::owned_subset_mut();
      country_core::render::nani::gpu_world::OwnedGpuWorldSubset::
      update_instance_data<country_core::resources::placement_data::PlacementDataSsbo>();
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      lVar30 = lVar30 + 0xc;
      LOCK();
      lVar4 = *extraout_RAX_05;
      if (lVar4 == 8) {
        *extraout_RAX_05 = 0;
      }
      UNLOCK();
      if (lVar4 != 8) {
        parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
      }
    }
    lVar40 = *(longlong *)(lVar35 + 0x20);
    lVar30 = *(longlong *)(lVar35 + 0x28) * 0x10 + lVar40;
LAB_140842630:
    if (lVar40 != lVar30) {
      *puVar14 = uVar9;
      uVar32 = (undefined4)uVar13;
      resources::rt::RtWorld::get<enum2_<country_core::resources::rt::RtMeshHandle>_>();
      lVar40 = lVar40 + 0x10;
      fVar3 = pfVar41[4];
      fVar44 = sinf(fVar3 * 0.5);
      fVar45 = cosf(fVar3 * 0.5);
      fVar53 = fVar44 * 0.0;
      fVar3 = *pfVar41;
      fVar23 = pfVar41[1];
      fVar24 = pfVar41[2];
      fVar25 = pfVar41[3];
      fVar55 = fVar44 + fVar44;
      fVar57 = fVar53 + fVar53;
      fVar59 = fVar53 * fVar57;
      fVar58 = fVar57 * fVar45;
      fVar60 = 1.0 - (fVar44 * fVar55 + fVar59);
      fVar65 = fVar59 - fVar55 * fVar45;
      fVar66 = fVar44 * fVar57 + fVar58;
      fVar63 = fVar25 * (fVar53 * fVar55 + fVar58);
      fVar64 = fVar25 * fVar65;
      fVar65 = fVar25 * fVar65;
      fVar53 = fVar25 * (fVar53 * fVar55 - fVar58);
      fVar61 = fVar25 * (1.0 - (fVar59 + fVar59));
      fVar62 = fVar25 * fVar66;
      fVar66 = fVar25 * fVar66;
      fVar45 = fVar25 * (fVar55 * fVar45 + fVar59);
      fVar44 = fVar25 * (fVar44 * fVar57 - fVar58);
      lVar4 = *extraout_RAX_06;
      auVar46._4_4_ = -(uint)(*(float *)(lVar4 + 0x14) == fVar63);
      auVar46._0_4_ = -(uint)(*(float *)(lVar4 + 0x10) == fVar25 * fVar60);
      auVar46._8_4_ = -(uint)(*(float *)(lVar4 + 0x18) == fVar64);
      auVar46._12_4_ = -(uint)(*(float *)(lVar4 + 0x1c) == fVar65);
      uVar32 = movmskps(uVar32,auVar46);
      if ((~(byte)uVar32 & 7) == 0) break;
      goto LAB_140842790;
    }
    lVar30 = *(longlong *)(lVar35 + 0x38);
    lVar40 = lVar30 + *(longlong *)(lVar35 + 0x40) * 0x28;
    for (; lVar30 != lVar40; lVar30 = lVar30 + 0x28) {
      ::utils::geometry::aabb::Aabb3::scaled();
      ::utils::geometry::aabb::Aabb3::translated();
      *puVar16 = uVar10;
      if ((*(byte *)(lVar30 + 0x18) & 1) != 0) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      if (*(ulonglong *)(lVar15 + 0x10) <= (ulonglong)*(uint *)(lVar30 + 0x24)) {
LAB_1408429c3:
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      lVar35 = *(longlong *)(lVar15 + 8);
      lVar4 = (ulonglong)*(uint *)(lVar30 + 0x24) * 0x1c;
      if (*(int *)(lVar35 + 0x18 + lVar4) != *(int *)(lVar30 + 0x20)) goto LAB_1408429c3;
      puVar31 = (undefined8 *)(lVar35 + lVar4);
      puVar31[2] = local_1f8;
      *puVar31 = local_208;
      puVar31[1] = uStack_200;
    }
  } while( true );
  auVar47._4_4_ = -(uint)(*(float *)(lVar4 + 0x24) == fVar61);
  auVar47._0_4_ = -(uint)(*(float *)(lVar4 + 0x20) == fVar53);
  auVar47._8_4_ = -(uint)(*(float *)(lVar4 + 0x28) == fVar62);
  auVar47._12_4_ = -(uint)(*(float *)(lVar4 + 0x2c) == fVar66);
  uVar32 = movmskps(CONCAT31((int3)((uint)uVar32 >> 8),~(byte)uVar32),auVar47);
  if ((((~(byte)uVar32 & 7) != 0) ||
      (auVar48._4_4_ = -(uint)(*(float *)(lVar4 + 0x34) == fVar44),
      auVar48._0_4_ = -(uint)(*(float *)(lVar4 + 0x30) == fVar45),
      auVar48._8_4_ = -(uint)(*(float *)(lVar4 + 0x38) == fVar25 * fVar60),
      auVar48._12_4_ = -(uint)(*(float *)(lVar4 + 0x3c) == fVar25 * fVar60),
      uVar32 = movmskps(CONCAT31((int3)((uint)uVar32 >> 8),~(byte)uVar32),auVar48),
      (~(byte)uVar32 & 7) != 0)) ||
     (auVar49._4_4_ = -(uint)(*(float *)(lVar4 + 0x44) == fVar23),
     auVar49._0_4_ = -(uint)(*(float *)(lVar4 + 0x40) == fVar3),
     auVar49._8_4_ = -(uint)(*(float *)(lVar4 + 0x48) == fVar24),
     auVar49._12_4_ = -(uint)(*(float *)(lVar4 + 0x4c) == fVar24),
     uVar32 = movmskps((int)lVar4,auVar49), (~(byte)uVar32 & 7) != 0)) {
LAB_140842790:
    resources::rt::DirtyCow::get_mut<country_core::resources::rt::RtMesh>();
    *extraout_RAX_07 = fVar25 * fVar60;
    extraout_RAX_07[1] = fVar63;
    extraout_RAX_07[2] = fVar64;
    extraout_RAX_07[3] = fVar65;
    extraout_RAX_07[4] = fVar53;
    extraout_RAX_07[5] = fVar61;
    extraout_RAX_07[6] = fVar62;
    extraout_RAX_07[7] = fVar66;
    extraout_RAX_07[8] = fVar45;
    extraout_RAX_07[9] = fVar44;
    extraout_RAX_07[10] = fVar25 * fVar60;
    extraout_RAX_07[0xb] = fVar25 * fVar60;
    extraout_RAX_07[0xc] = fVar3;
    extraout_RAX_07[0xd] = fVar23;
    extraout_RAX_07[0xe] = fVar24;
    extraout_RAX_07[0xf] = fVar24;
  }
  goto LAB_140842630;
}

