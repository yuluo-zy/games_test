// ui_adjust_wall_height_signifier_animation  size=3414
// void __cdecl ui_adjust_wall_height_signifier_animation(void)

void __cdecl
country_core::systems::wall::adjust_wall_height::signifier_animation::
ui_adjust_wall_height_signifier_animation(void)

{
  uint *puVar1;
  char *pcVar2;
  undefined4 *puVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  longlong lVar6;
  longlong lVar7;
  longlong lVar8;
  longlong *plVar9;
  undefined4 *puVar10;
  undefined8 ***pppuVar11;
  uint uVar12;
  uint uVar13;
  int iVar14;
  int iVar15;
  int iVar16;
  int iVar17;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  longlong *in_RCX;
  longlong lVar18;
  longlong in_RDX;
  ulonglong uVar19;
  ulonglong uVar20;
  longlong lVar21;
  longlong lVar22;
  ulonglong **ppuVar23;
  longlong *in_R8;
  longlong lVar24;
  uint uVar25;
  ushort uVar26;
  longlong *plVar27;
  ulonglong uVar28;
  uint *puVar29;
  char cVar30;
  byte bVar31;
  char cVar33;
  char cVar34;
  char cVar35;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  undefined1 auVar32 [16];
  float fVar36;
  float fVar37;
  undefined1 auVar38 [16];
  undefined1 auVar39 [16];
  float fVar40;
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
  float fVar41;
  longlong *in_stack_00000028;
  longlong local_1f8;
  longlong local_1b8;
  undefined8 ***local_1a8;
  ulonglong **ppuStack_1a0;
  longlong local_198;
  longlong lStack_190;
  longlong local_188;
  longlong lStack_180;
  longlong local_178;
  ulonglong **local_168;
  ulonglong **ppuStack_160;
  ulonglong ***local_158;
  undefined4 local_150;
  undefined4 uStack_14c;
  undefined4 uStack_148;
  undefined4 uStack_144;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  longlong local_138;
  longlong local_130;
  ulonglong **local_128;
  longlong lStack_120;
  undefined8 **local_118;
  longlong local_110;
  undefined8 local_108;
  undefined4 local_fc;
  uint local_f8;
  char local_f2;
  undefined1 local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  lVar6 = *in_RCX;
  lVar7 = in_RCX[1];
  local_fc = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar29 = *(uint **)(lVar6 + 0xf8);
  puVar1 = puVar29 + *(longlong *)(lVar6 + 0x100);
  local_f2 = *(char *)(lVar6 + 0x108);
  lVar8 = *in_stack_00000028;
  plVar9 = (longlong *)*in_R8;
  uVar4 = *(undefined4 *)(in_RDX + 0x1c);
  puVar10 = *(undefined4 **)(in_RDX + 0x10);
  iVar14 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                            CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                  CONCAT13(unaff_XMM6_Bd,
                           CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
  iVar15 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                            CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                  CONCAT13(unaff_XMM6_Bh,
                           CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
  iVar16 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                            CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                  CONCAT13(unaff_XMM6_Bl,
                           CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
  iVar17 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                            CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                  CONCAT13(unaff_XMM6_Bp,
                           CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
  local_1f8 = 8;
  local_130 = 0;
  uVar19 = 0;
  uVar28 = 0;
  local_110 = 0;
  local_108 = 0;
  local_f8 = 0;
  cVar30 = local_f2;
LAB_14085c2b0:
  do {
    if (cVar30 == '\0') {
      if (uVar19 == uVar28) {
        if (puVar29 == puVar1) {
          return;
        }
        while( true ) {
          uVar28 = *(ulonglong *)(*(longlong *)(lVar7 + 0x100) + 0x58 + (ulonglong)*puVar29 * 0xa0);
          if (uVar28 != 0) break;
          puVar29 = puVar29 + 1;
          if (puVar29 == puVar1) {
            return;
          }
        }
        lVar24 = *(longlong *)(lVar7 + 0x100) + (ulonglong)*puVar29 * 0xa0;
        uVar20 = (ulonglong)*(uint *)(lVar24 + 0x94);
        lVar18 = *(longlong *)(lVar7 + 0x1a8);
        uVar19 = *(ulonglong *)(lVar18 + 0x40 + uVar20 * 0x48);
        lVar21 = *(longlong *)(lVar18 + 0x18 + uVar20 * 0x48);
        lVar18 = *(longlong *)(lVar18 + 0x38 + uVar20 * 0x48);
        if ((*(ulonglong *)(lVar6 + 0x118) < uVar19) &&
           (uVar20 = *(ulonglong *)(lVar18 + *(ulonglong *)(lVar6 + 0x118) * 8), uVar20 != 0)) {
          local_110 = *(longlong *)(lVar21 + 0x10 + ~uVar20 * 0x30);
          uVar20 = *(ulonglong *)(lVar6 + 0x120);
          if (uVar19 <= uVar20) goto LAB_14085c50e;
LAB_14085c4d2:
          uVar19 = *(ulonglong *)(lVar18 + uVar20 * 8);
          if (uVar19 == 0) goto LAB_14085c50e;
          local_108 = *(undefined8 *)(lVar21 + 0x10 + ~uVar19 * 0x30);
          uVar19 = *(ulonglong *)(lVar6 + 0x128);
          if (*(ulonglong *)(lVar24 + 0x88) <= uVar19) goto LAB_14085c502;
LAB_14085c529:
          local_f8 = (uint)CONCAT71((int7)(uVar19 >> 8),
                                    *(longlong *)(*(longlong *)(lVar24 + 0x80) + uVar19 * 8) != 0);
        }
        else {
          local_110 = 0;
          uVar20 = *(ulonglong *)(lVar6 + 0x120);
          if (uVar20 < uVar19) goto LAB_14085c4d2;
LAB_14085c50e:
          local_108 = 0;
          uVar19 = *(ulonglong *)(lVar6 + 0x128);
          if (uVar19 < *(ulonglong *)(lVar24 + 0x88)) goto LAB_14085c529;
LAB_14085c502:
          local_f8 = 0;
        }
        lVar18 = ~*(ulonglong *)(lVar18 + *(longlong *)(lVar6 + 0x110) * 8) * 0x30;
        local_130 = *(longlong *)(lVar21 + 0x10 + lVar18);
        local_1b8 = *(longlong *)(lVar21 + 0x28 + lVar18);
        puVar29 = puVar29 + 1;
        local_1f8 = *(longlong *)(lVar24 + 0x50);
        uVar19 = 0;
      }
      uVar20 = (ulonglong)*(uint *)(local_1f8 + 8 + uVar19 * 0x10);
    }
    else {
      if (uVar19 == uVar28) {
        if (puVar29 == puVar1) {
          return;
        }
        while( true ) {
          uVar28 = *(ulonglong *)(*(longlong *)(lVar7 + 0x1a8) + 0x10 + (ulonglong)*puVar29 * 0x48);
          if (uVar28 != 0) break;
          puVar29 = puVar29 + 1;
          if (puVar29 == puVar1) {
            return;
          }
        }
        lVar18 = *(longlong *)(lVar7 + 0x1a8) + (ulonglong)*puVar29 * 0x48;
        uVar19 = *(ulonglong *)(lVar18 + 0x40);
        lVar21 = *(longlong *)(lVar18 + 0x18);
        lVar18 = *(longlong *)(lVar18 + 0x38);
        if ((*(ulonglong *)(lVar6 + 0x118) < uVar19) &&
           (uVar20 = *(ulonglong *)(lVar18 + *(ulonglong *)(lVar6 + 0x118) * 8), uVar20 != 0)) {
          local_110 = *(longlong *)(lVar21 + 0x10 + ~uVar20 * 0x30);
          uVar20 = *(ulonglong *)(lVar6 + 0x120);
          if (uVar19 <= uVar20) goto LAB_14085c45e;
LAB_14085c426:
          uVar20 = *(ulonglong *)(lVar18 + uVar20 * 8);
          if (uVar20 == 0) goto LAB_14085c45e;
          local_108 = *(undefined8 *)(lVar21 + 0x10 + ~uVar20 * 0x30);
          uVar20 = *(ulonglong *)(lVar6 + 0x128);
          if (uVar19 <= uVar20) goto LAB_14085c452;
LAB_14085c475:
          local_f8 = (uint)CONCAT71((int7)(uVar19 >> 8),*(longlong *)(lVar18 + uVar20 * 8) != 0);
        }
        else {
          local_110 = 0;
          uVar20 = *(ulonglong *)(lVar6 + 0x120);
          if (uVar20 < uVar19) goto LAB_14085c426;
LAB_14085c45e:
          local_108 = 0;
          uVar20 = *(ulonglong *)(lVar6 + 0x128);
          if (uVar20 < uVar19) goto LAB_14085c475;
LAB_14085c452:
          local_f8 = 0;
        }
        lVar18 = ~*(ulonglong *)(lVar18 + *(longlong *)(lVar6 + 0x110) * 8) * 0x30;
        local_130 = *(longlong *)(lVar21 + 0x10 + lVar18);
        local_1b8 = *(longlong *)(lVar21 + 0x28 + lVar18);
        puVar29 = puVar29 + 1;
        uVar19 = 0;
      }
      uVar20 = uVar19 & 0xffffffff;
    }
    plVar27 = (longlong *)(uVar20 * 0x60 + local_130);
    puVar3 = (undefined4 *)(local_1b8 + uVar20 * 4);
    lVar18 = uVar20 * 0x1c + local_110;
    if (*(char *)(lVar8 + 0x18) != '\0') {
      ppuStack_160 = *(ulonglong ***)(lVar8 + 0x34);
      local_158 = *(ulonglong ****)(lVar8 + 0x3c);
      uStack_148 = (undefined4)((ulonglong)*(undefined8 *)(lVar8 + 0x48) >> 0x20);
      uStack_144 = (undefined4)*(undefined8 *)(lVar8 + 0x50);
      uStack_140 = (undefined4)((ulonglong)*(undefined8 *)(lVar8 + 0x50) >> 0x20);
      local_150 = (undefined4)*(undefined8 *)(lVar8 + 0x44);
      uStack_14c = (undefined4)((ulonglong)*(undefined8 *)(lVar8 + 0x44) >> 0x20);
      if (*(int *)(lVar8 + 0x28) != 2) {
        *puVar3 = local_fc;
        *(undefined4 *)(plVar27 + 9) = 1;
        *(ulonglong *)((longlong)plVar27 + 0x4c) = CONCAT44(uStack_144,uStack_148);
        *(undefined4 *)((longlong)plVar27 + 0x54) = uStack_140;
      }
    }
    uVar19 = uVar19 + 1;
    pppuVar11 = (undefined8 ***)plVar27[0xb];
    if (plVar9[3] != 0) {
      local_168 = (ulonglong **)pppuVar11;
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      lVar21 = *plVar9;
      bVar31 = (byte)(extraout_RAX_01 >> 0x39);
      auVar32 = ZEXT216(CONCAT11(bVar31,bVar31));
      auVar32 = pshuflw(auVar32,auVar32,0);
      lVar24 = 0;
      uVar20 = extraout_RAX_01;
      while( true ) {
        uVar20 = plVar9[1] & uVar20;
        pcVar2 = (char *)(lVar21 + uVar20);
        cVar30 = auVar32[0];
        auVar39[0] = -(*pcVar2 == cVar30);
        cVar33 = auVar32[1];
        auVar39[1] = -(pcVar2[1] == cVar33);
        cVar34 = auVar32[2];
        auVar39[2] = -(pcVar2[2] == cVar34);
        cVar35 = auVar32[3];
        auVar39[3] = -(pcVar2[3] == cVar35);
        auVar39[4] = -(pcVar2[4] == cVar30);
        auVar39[5] = -(pcVar2[5] == cVar33);
        auVar39[6] = -(pcVar2[6] == cVar34);
        auVar39[7] = -(pcVar2[7] == cVar35);
        auVar39[8] = -(pcVar2[8] == cVar30);
        auVar39[9] = -(pcVar2[9] == cVar33);
        auVar39[10] = -(pcVar2[10] == cVar34);
        auVar39[0xb] = -(pcVar2[0xb] == cVar35);
        auVar39[0xc] = -(pcVar2[0xc] == cVar30);
        auVar39[0xd] = -(pcVar2[0xd] == cVar33);
        auVar39[0xe] = -(pcVar2[0xe] == cVar34);
        auVar39[0xf] = -(pcVar2[0xf] == cVar35);
        uVar26 = (ushort)(SUB161(auVar39 >> 7,0) & 1) | (ushort)(SUB161(auVar39 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar39 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar39 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar39 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar39 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar39 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar39 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar39 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar39 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar39 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar39 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar39 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar39 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar39 >> 0x77,0) & 1) << 0xe | (ushort)(auVar39[0xf] >> 7) << 0xf
        ;
        uVar25 = (uint)uVar26;
        if (uVar26 != 0) break;
LAB_14085bd20:
        auVar38[0] = -(*pcVar2 == (char)iVar14);
        auVar38[1] = -(pcVar2[1] == (char)((uint)iVar14 >> 8));
        auVar38[2] = -(pcVar2[2] == (char)((uint)iVar14 >> 0x10));
        auVar38[3] = -(pcVar2[3] == (char)((uint)iVar14 >> 0x18));
        auVar38[4] = -(pcVar2[4] == (char)iVar15);
        auVar38[5] = -(pcVar2[5] == (char)((uint)iVar15 >> 8));
        auVar38[6] = -(pcVar2[6] == (char)((uint)iVar15 >> 0x10));
        auVar38[7] = -(pcVar2[7] == (char)((uint)iVar15 >> 0x18));
        auVar38[8] = -(pcVar2[8] == (char)iVar16);
        auVar38[9] = -(pcVar2[9] == (char)((uint)iVar16 >> 8));
        auVar38[10] = -(pcVar2[10] == (char)((uint)iVar16 >> 0x10));
        auVar38[0xb] = -(pcVar2[0xb] == (char)((uint)iVar16 >> 0x18));
        auVar38[0xc] = -(pcVar2[0xc] == (char)iVar17);
        auVar38[0xd] = -(pcVar2[0xd] == (char)((uint)iVar17 >> 8));
        auVar38[0xe] = -(pcVar2[0xe] == (char)((uint)iVar17 >> 0x10));
        auVar38[0xf] = -(pcVar2[0xf] == (char)((uint)iVar17 >> 0x18));
        cVar30 = local_f2;
        if ((((((((((((((((SUB161(auVar38 >> 7,0) & 1) != 0 || (SUB161(auVar38 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar38 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar38 >> 0x1f,0) & 1) != 0) || (SUB161(auVar38 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar38 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar38 >> 0x37,0) & 1) != 0) || (SUB161(auVar38 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar38 >> 0x47,0) & 1) != 0) || (SUB161(auVar38 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar38 >> 0x57,0) & 1) != 0) || (SUB161(auVar38 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar38 >> 0x67,0) & 1) != 0) || (SUB161(auVar38 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar38 >> 0x77,0) & 1) != 0) || auVar38[0xf] < '\0') goto LAB_14085c2b0;
        uVar20 = uVar20 + lVar24 + 0x10;
        lVar24 = lVar24 + 0x10;
      }
      while( true ) {
        uVar12 = 0;
        for (uVar13 = uVar25; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
          uVar12 = uVar12 + 1;
        }
        lVar22 = (uVar12 + uVar20 & plVar9[1]) * -0x2a8;
        if (*(undefined8 ****)(lVar21 + -0x2a8 + lVar22) == pppuVar11) break;
        uVar26 = (ushort)(uVar25 - 1) & (ushort)uVar25;
        uVar25 = CONCAT22((short)(uVar25 - 1 >> 0x10),uVar26);
        if (uVar26 == 0) goto LAB_14085bd20;
      }
      if ((*(ulonglong *)(lVar21 + -0x290 + lVar22) < 2) ||
         (*(float *)(lVar21 + -0x270 + lVar22) <= 0.0)) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      resources::walls::public_walls::PublicWalls::roof_component();
      if (extraout_RAX == 0) {
        fVar41 = 0.0;
      }
      else {
        fVar41 = 0.0;
        if ((*(float *)(extraout_RAX + 0x30) <= 0.1) && (*(char *)(extraout_RAX + 0x3d) != '\0')) {
          fVar41 = -0.29000002;
        }
      }
      lVar21 = lVar21 + lVar22;
      ppuVar23 = *(ulonglong ***)(lVar21 + -0x208);
      if (*(ulonglong ***)(lVar21 + -0x1b8) < *(ulonglong ***)(lVar21 + -0x208)) {
        ppuVar23 = *(ulonglong ***)(lVar21 + -0x1b8);
      }
      if ((ulonglong **)0xaaaaaaaaaaaaaaa < ppuVar23) {
LAB_14085c85e:
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      if (ppuVar23 == (ulonglong **)0x0) {
        lStack_120 = 4;
      }
      else {
        lVar24 = *(longlong *)(lVar21 + -0x210);
        lVar21 = *(longlong *)(lVar21 + -0x1c0);
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_00 == 0) goto LAB_14085c85e;
        if (ppuVar23 == (ulonglong **)0x1) {
          uVar20 = 0;
        }
        else {
          lVar22 = 5;
          uVar20 = 0;
          do {
            uVar5 = *(undefined4 *)(lVar24 + -0xc + lVar22 * 4);
            fVar36 = *(float *)(lVar21 + uVar20 * 4);
            *(undefined4 *)(extraout_RAX_00 + -0x14 + lVar22 * 4) =
                 *(undefined4 *)(lVar24 + -0x14 + lVar22 * 4);
            *(float *)(extraout_RAX_00 + -0x10 + lVar22 * 4) = fVar36 + fVar41;
            *(undefined4 *)(extraout_RAX_00 + -0xc + lVar22 * 4) = uVar5;
            uVar5 = *(undefined4 *)(lVar24 + lVar22 * 4);
            fVar36 = *(float *)(lVar21 + 4 + uVar20 * 4);
            uVar20 = uVar20 + 2;
            *(undefined4 *)(extraout_RAX_00 + -8 + lVar22 * 4) =
                 *(undefined4 *)(lVar24 + -8 + lVar22 * 4);
            *(float *)(extraout_RAX_00 + -4 + lVar22 * 4) = fVar36 + fVar41;
            *(undefined4 *)(extraout_RAX_00 + lVar22 * 4) = uVar5;
            lVar22 = lVar22 + 6;
          } while (((ulonglong)ppuVar23 & 0xffffffffffffffe) != uVar20);
        }
        lStack_120 = extraout_RAX_00;
        if (((ulonglong)ppuVar23 & 1) != 0) {
          lVar22 = uVar20 * 0xc;
          fVar36 = *(float *)(lVar21 + uVar20 * 4);
          uVar5 = *(undefined4 *)(lVar24 + 8 + lVar22);
          *(undefined4 *)(extraout_RAX_00 + lVar22) = *(undefined4 *)(lVar24 + lVar22);
          *(float *)(extraout_RAX_00 + 4 + lVar22) = fVar41 + fVar36;
          *(undefined4 *)(extraout_RAX_00 + 8 + lVar22) = uVar5;
        }
      }
      local_128 = ppuVar23;
      local_118 = ppuVar23;
      ::utils::curve::Curve::
      try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>();
      if (SBORROW8(0,(longlong)local_168)) {
        local_128 = ppuStack_160;
        lStack_120 = (longlong)local_158;
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      local_178 = local_138;
      lStack_190 = CONCAT44(uStack_14c,local_150);
      local_188 = CONCAT44(uStack_144,uStack_148);
      lStack_180 = CONCAT44(uStack_13c,uStack_140);
      local_198 = (longlong)local_158;
      local_1a8 = (undefined8 ***)local_168;
      ppuStack_1a0 = ppuStack_160;
      *puVar3 = local_fc;
      if (!SBORROW8(0,*plVar27)) {
        if (*plVar27 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        if (plVar27[3] != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
      }
      plVar27[6] = local_178;
      plVar27[4] = local_188;
      plVar27[5] = lStack_180;
      plVar27[2] = local_198;
      plVar27[3] = lStack_190;
      *plVar27 = (longlong)local_1a8;
      plVar27[1] = (longlong)ppuStack_1a0;
      if ((*(byte *)(plVar27 + 7) & 1) == 0) {
        fVar41 = 0.5;
        if ((*(byte *)(plVar27 + 9) & 1) != 0) {
          if (SBORROW8(0,*plVar27)) {
                    // WARNING: Subroutine does not return
            core::option::unwrap_failed();
          }
          ::utils::curve::Curve::
          get_approx_u_from_pos<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>();
          fVar36 = 0.0;
          if (0.0 <= extraout_XMM0_Da) {
            fVar36 = extraout_XMM0_Da;
          }
          fVar41 = 1.0;
          if (fVar36 <= 1.0) {
            fVar41 = fVar36;
          }
        }
        *puVar3 = local_fc;
        *(undefined4 *)(plVar27 + 7) = 1;
        *(float *)((longlong)plVar27 + 0x3c) = fVar41;
        fVar41 = *(float *)(lVar18 + 0x18);
        if (0.95 < fVar41) goto LAB_14085c051;
LAB_14085c09a:
        if ((*(byte *)(plVar27 + 8) & 1) == 0) {
          if ((*(byte *)(plVar27 + 7) & 1) == 0) {
            fVar36 = 0.5;
          }
          else {
            fVar36 = *(float *)((longlong)plVar27 + 0x3c);
          }
        }
        else {
          fVar36 = *(float *)((longlong)plVar27 + 0x44);
        }
      }
      else {
        fVar41 = *(float *)(lVar18 + 0x18);
        if (fVar41 <= 0.95) goto LAB_14085c09a;
LAB_14085c051:
        if ((local_f8 & 1) == 0) goto LAB_14085c09a;
        if ((*(byte *)(plVar27 + 9) & 1) == 0) {
          fVar36 = *(float *)((longlong)plVar27 + 0x3c);
        }
        else {
          if (SBORROW8(0,*plVar27)) {
                    // WARNING: Subroutine does not return
            core::option::unwrap_failed();
          }
          ::utils::curve::Curve::
          get_approx_u_from_pos<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>();
          fVar41 = 0.0;
          if (0.0 <= extraout_XMM0_Da_00) {
            fVar41 = extraout_XMM0_Da_00;
          }
          fVar36 = 1.0;
          if (fVar41 <= 1.0) {
            fVar36 = fVar41;
          }
        }
        *puVar3 = local_fc;
        *(undefined4 *)(plVar27 + 8) = 1;
        *(float *)((longlong)plVar27 + 0x44) = fVar36;
        fVar41 = *(float *)(lVar18 + 0x18);
      }
      fVar37 = 0.0;
      if (0.0 <= fVar36 + -0.01) {
        fVar37 = fVar36 + -0.01;
      }
      fVar40 = 1.0;
      if (fVar37 <= 1.0) {
        fVar40 = fVar37;
      }
      fVar37 = fVar41 * 0.0 + fVar40 * (1.0 - fVar41);
      local_128 = (ulonglong **)CONCAT44(local_128._4_4_,fVar37);
      if ((fVar37 < 0.0) || (1.0 < fVar37)) {
        local_1a8 = &local_128;
        ppuStack_1a0 = (ulonglong **)core::fmt::float::impl$5::fmt;
        local_168 = (ulonglong **)&anon_12ce4fde5425384d5f438f5122902e1a_13_llvm_3277990281340110158
        ;
        ppuStack_160 = (ulonglong **)0x1;
        uStack_148 = 0;
        uStack_144 = 0;
        local_158 = (ulonglong ***)&local_1a8;
        local_150 = 1;
        uStack_14c = 0;
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
      if (NAN(fVar37)) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      fVar37 = 0.0;
      if (0.0 <= fVar36 + 0.01) {
        fVar37 = fVar36 + 0.01;
      }
      fVar36 = 1.0;
      if (fVar37 <= 1.0) {
        fVar36 = fVar37;
      }
      fVar41 = fVar41 + (1.0 - fVar41) * fVar36;
      local_128 = (ulonglong **)CONCAT44(local_128._4_4_,fVar41);
      if ((fVar41 < 0.0) || (1.0 < fVar41)) {
        local_1a8 = &local_128;
        ppuStack_1a0 = (ulonglong **)core::fmt::float::impl$5::fmt;
        local_168 = (ulonglong **)&anon_12ce4fde5425384d5f438f5122902e1a_13_llvm_3277990281340110158
        ;
        ppuStack_160 = (ulonglong **)0x1;
        uStack_148 = 0;
        uStack_144 = 0;
        local_158 = (ulonglong ***)&local_1a8;
        local_150 = 1;
        uStack_14c = 0;
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
      if (NAN(fVar41)) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      if (SBORROW8(0,*plVar27)) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      local_128 = (ulonglong **)0x0;
      lStack_120 = 4;
      local_118 = (undefined8 **)0x0;
      local_f1 = 1;
      ::utils::curve::Curve::
      slice_into<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,alloc::vec::Vec<glam::f32::vec3::Vec3,alloc::alloc::Global>_>
                ();
      local_198 = (longlong)local_118;
      local_1a8 = (undefined8 ***)local_128;
      ppuStack_1a0 = (ulonglong **)lStack_120;
      local_f1 = 0;
      ::utils::curve::Curve::
      try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>();
      if (SBORROW8(0,(longlong)local_168)) {
        local_1a8 = (undefined8 ***)ppuStack_160;
        ppuStack_1a0 = (ulonglong **)local_158;
        local_f1 = 0;
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      local_178 = local_138;
      lStack_190 = CONCAT44(uStack_14c,local_150);
      local_188 = CONCAT44(uStack_144,uStack_148);
      lStack_180 = CONCAT44(uStack_13c,uStack_140);
      local_198 = (longlong)local_158;
      local_1a8 = (undefined8 ***)local_168;
      ppuStack_1a0 = ppuStack_160;
      *puVar10 = uVar4;
      country_core::utils::geo::
      update_curve_mesh<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3>_>_>
                ();
      if (local_1a8 != (undefined8 ***)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      cVar30 = local_f2;
      if (lStack_190 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
        cVar30 = local_f2;
      }
    }
  } while( true );
}



// CALLEES: [{'name': '__rust_alloc', 'address': '140613c10'}, {'name': '__rust_dealloc', 'address': '140613c20'}, {'name': 'get_approx_u_from_pos<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>', 'address': '141469950'}, {'name': 'handle_error', 'address': '1428d8fe3'}, {'name': 'hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId>_>', 'address': '1405b0d70'}, {'name': 'panic', 'address': '1428d9430'}, {'name': 'panic_fmt', 'address': '1428d9390'}, {'name': 'roof_component', 'address': '140a923a0'}, {'name': 'slice_into<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,alloc::vec::Vec<glam::f32::vec3::Vec3,alloc::alloc::Global>_>', 'address': '141466300'}, {'name': 'try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>', 'address': '141469230'}, {'name': 'unwrap_failed', 'address': '1428d9760'}, {'name': 'unwrap_failed', 'address': '1428d9310'}, {'name': 'update_curve_mesh<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3>_>_>', 'address': '14157f130'}]