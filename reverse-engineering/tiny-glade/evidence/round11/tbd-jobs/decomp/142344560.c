// 0x142344560 undefined FUN_142344560(void) body_bytes=201

/* WARNING: Instruction at (ram,0x000142344d3b) overlaps instruction at (ram,0x000142344d3a)
    */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_142344560(float param_1,undefined8 *param_2,byte *param_3,ulonglong *param_4)

{
  int *piVar1;
  byte bVar2;
  uint uVar3;
  uint uVar4;
  int iVar5;
  longlong *plVar6;
  code *pcVar7;
  ulonglong uVar8;
  float fVar9;
  float *pfVar10;
  byte bVar11;
  char cVar12;
  byte **ppbVar13;
  byte *pbVar14;
  longlong lVar15;
  undefined8 *in_RCX;
  byte *pbVar16;
  longlong *plVar17;
  longlong lVar18;
  undefined *puVar19;
  byte *pbVar20;
  undefined4 *puVar21;
  byte *pbVar22;
  undefined8 uVar23;
  longlong *unaff_RBP;
  longlong lVar24;
  byte *pbVar25;
  longlong lVar26;
  byte *pbVar27;
  uint uVar28;
  uint uVar29;
  undefined4 uVar30;
  byte *pbVar31;
  longlong *unaff_R15;
  bool bVar32;
  float fVar33;
  int iVar34;
  undefined8 in_XMM1_Qa;
  int iVar35;
  undefined1 in_XMM2 [16];
  undefined1 auVar36 [16];
  float fVar38;
  float unaff_XMM9_Da;
  float fVar39;
  float unaff_XMM11_Da;
  undefined8 *in_stack_00000080;
  longlong *in_stack_00000098;
  int iStack_1e8;
  undefined4 uStack_1e4;
  undefined4 uStack_1e0;
  undefined4 uStack_1dc;
  undefined8 uStack_1d8;
  undefined8 uStack_1d0;
  int iStack_1c8;
  undefined4 uStack_1c4;
  undefined4 uStack_1c0;
  undefined4 uStack_1bc;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  undefined1 auStack_1a8 [280];
  undefined *puStack_90;
  float fStack_68;
  uint uStack_64;
  undefined8 uStack_60;
  byte *pbStack_58;
  byte *pbStack_50;
  byte *pbStack_48;
  undefined4 uVar37;
  
  iVar35 = (int)((ulonglong)in_XMM1_Qa >> 0x20);
  iVar34 = (int)in_XMM1_Qa;
  pbVar20 = *(byte **)param_3;
  pbStack_48 = (byte *)*param_2;
  pbStack_50 = pbVar20;
  fStack_68 = *(float *)((longlong)param_4 + 0x1c);
  plVar6 = (longlong *)*param_4;
  uStack_60 = (float *)param_4[2];
  in_RCX = (undefined8 *)*in_RCX;
  ppbVar13 = &pbStack_48;
  pbStack_58 = (byte *)0x0;
  do {
    pfVar10 = uStack_60;
    fVar39 = fStack_68;
    uVar23 = in_XMM2._0_8_;
    pbVar22 = *ppbVar13;
    uVar29 = *(uint *)(pbVar22 + 0x18);
    pbVar31 = (byte *)(ulonglong)uVar29;
    uVar3 = *(uint *)(pbVar22 + 0x1c);
    pbVar25 = (byte *)(ulonglong)uVar3;
    uStack_64 = *(uint *)(pbVar22 + 0x20);
    uVar4 = *(uint *)(pbVar22 + 0x24);
    pbVar27 = (byte *)(ulonglong)uVar4;
    *uStack_60 = fStack_68;
    if (plVar6[0x15] != 0) {
      lVar24 = plVar6[0x14];
      pbVar20 = (byte *)(plVar6[0x15] * 0x28);
      param_3 = (byte *)0x0;
      do {
        if (param_3[lVar24 + 0x24] != 4) {
          uVar28 = 0x1020300 >> ((*(byte *)(in_RCX + 0x1e) & 3) << 3);
          param_4 = (ulonglong *)(ulonglong)uVar28;
          param_3[lVar24 + 0x24] = (byte)uVar28;
        }
        param_3 = param_3 + 0x28;
      } while (pbVar20 != param_3);
    }
    uVar30 = SUB84(param_4,0);
    pbVar16 = &UNK_14304e6d8;
    pbVar14 = &UNK_14304e6d8 + *(int *)(&UNK_14304e6d8 + (ulonglong)*(byte *)(in_RCX + 0x1e) * 4);
    bVar32 = pbVar14 == (byte *)0x0;
    bVar11 = (byte)pbVar14;
    plVar17 = plVar6;
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
    switch(*(byte *)(in_RCX + 0x1e)) {
    case 0:
      *uStack_60 = fStack_68;
      puStack_90 = &UNK_142344662;
      FUN_142376050(plVar6,pbVar31,pbVar25);
      *pfVar10 = fVar39;
      puStack_90 = &UNK_142344673;
      FUN_142375ce0(plVar6,uVar29,pbVar25);
      uVar29 = uStack_64;
      *pfVar10 = fVar39;
      puStack_90 = &UNK_142344687;
      FUN_142375b40(plVar6,uStack_64,pbVar27);
      *pfVar10 = fVar39;
      pbVar20 = (byte *)(ulonglong)uVar29;
      param_3 = (byte *)(ulonglong)uVar4;
      puStack_90 = &UNK_142344697;
      param_1 = (float)FUN_142375ce0(plVar6);
      break;
    case 1:
      *uStack_60 = fStack_68;
      puStack_90 = &UNK_14234477a;
      FUN_142375c10(plVar6,pbVar31,pbVar25);
      *pfVar10 = fVar39;
      puStack_90 = &UNK_14234478b;
      FUN_142375f80(plVar6,pbVar31,pbVar25);
      uVar29 = uStack_64;
      *pfVar10 = fVar39;
      puStack_90 = &UNK_14234479f;
      FUN_142375eb0(plVar6,uStack_64,pbVar27);
      *pfVar10 = fVar39;
      pbVar20 = (byte *)(ulonglong)uVar29;
      puStack_90 = &UNK_1423447af;
      param_1 = (float)FUN_142376300(plVar6);
      param_3 = pbVar27;
      break;
    case 2:
      *uStack_60 = fStack_68;
      puStack_90 = &UNK_1423446ba;
      FUN_142376230(plVar6,pbVar31,pbVar25);
      *pfVar10 = fVar39;
      puStack_90 = &UNK_1423446cb;
      FUN_142375f80(plVar6,pbVar31,pbVar25);
      uVar29 = uStack_64;
      *pfVar10 = fVar39;
      puStack_90 = &UNK_1423446df;
      FUN_142375eb0(plVar6,uStack_64,pbVar27);
      *pfVar10 = fVar39;
      pbVar20 = (byte *)(ulonglong)uVar29;
      puStack_90 = &UNK_1423446ef;
      param_1 = (float)FUN_142375ce0(plVar6);
      param_3 = pbVar27;
      break;
    case 3:
      *uStack_60 = fStack_68;
      puStack_90 = &UNK_14234471a;
      FUN_142376050(plVar6,pbVar31,pbVar25);
      *pfVar10 = fVar39;
      puStack_90 = &UNK_14234472b;
      FUN_142376300(plVar6,pbVar31,pbVar25);
      uVar29 = uStack_64;
      *pfVar10 = fVar39;
      puStack_90 = &UNK_14234473f;
      FUN_142375b40(plVar6,uStack_64,pbVar27);
      *pfVar10 = fVar39;
      pbVar20 = (byte *)(ulonglong)uVar29;
      puStack_90 = &UNK_14234474f;
      param_1 = (float)FUN_142376300(plVar6);
      param_3 = pbVar27;
      break;
    default:
code_r0x00014234485d:
                    /* WARNING: Could not recover jumptable at 0x00014234486b. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      (*(code *)(&UNK_14304e6e8 + *(int *)(&UNK_14304e6e8 + (longlong)pbVar14 * 4)))();
      return;
    case 5:
    case 7:
      out(0xeb,bVar11);
      pbVar20 = pbVar25;
      goto code_r0x0001423447d0;
    case 8:
      pbVar22 = pbVar22 + 0x50c139bda;
      bVar2 = *pbVar22;
      *pbVar22 = *pbVar22 + bVar11;
      *(int *)pbVar27 = (int)unaff_R15;
                    /* WARNING: Could not recover jumptable at 0x0001423448e4. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      (*(code *)(&UNK_14304e6f8 +
                *(int *)(&UNK_14304e6f8 +
                        CONCAT71((int7)((ulonglong)pbVar14 >> 8),
                                 (bVar11 + 0xb4) - CARRY1(bVar2,bVar11)) * 4)))();
      return;
    case 9:
      *(char *)((longlong)unaff_RBP + -0x77) = *(char *)((longlong)unaff_RBP + -0x77) + bVar11;
      goto code_r0x000142375b40;
    case 10:
      *(char *)((longlong)unaff_RBP + -0x77) = *(char *)((longlong)unaff_RBP + -0x77) + bVar11;
code_r0x000142375b40:
      pbStack_50 = (byte *)CONCAT44(uStack_64,fStack_68);
      pbStack_48 = pbStack_58;
      FUN_140fe7cb0(&iStack_1e8,pbVar22,uVar3,(ulonglong)plVar6 & 0xffffffff);
      if (iStack_1e8 == 4) {
        FUN_142374fd0(auStack_1a8,CONCAT44(uStack_1dc,uStack_1e0));
        uStack_60._0_4_ = (uint)(uint3)uStack_60;
        FUN_140fe7b10(&iStack_1e8,pbVar22,uVar3,(ulonglong)plVar6 & 0xffffffff,auStack_1a8);
        if (iStack_1e8 == 4) {
          return;
        }
        uStack_1b8 = uStack_1d8;
        uStack_1b0 = uStack_1d0;
        iStack_1c8 = iStack_1e8;
        uStack_1c4 = uStack_1e4;
        uStack_1c0 = uStack_1e0;
        uStack_1bc = uStack_1dc;
        puVar19 = &UNK_143052e68;
      }
      else {
        uStack_1b8 = uStack_1d8;
        uStack_1b0 = uStack_1d0;
        iStack_1c8 = iStack_1e8;
        uStack_1c4 = uStack_1e4;
        uStack_1c0 = uStack_1e0;
        uStack_1bc = uStack_1dc;
        puVar19 = &UNK_143052e50;
      }
      FUN_1428d9760(&UNK_143052c50,0x2b,&iStack_1c8,&UNK_143052c30,puVar19);
      pcVar7 = (code *)swi(3);
      (*pcVar7)();
      return;
    case 0xb:
      *(char *)((longlong)unaff_RBP + -0x77) = *(char *)((longlong)unaff_RBP + -0x77) + bVar11;
      pbStack_50 = (byte *)CONCAT44(uStack_64,fStack_68);
      pbStack_48 = pbStack_58;
      FUN_140fe7cb0(&iStack_1e8,pbVar22,pbVar25,(ulonglong)plVar6 & 0xffffffff);
      if (iStack_1e8 == 4) {
        FUN_142374fd0(auStack_1a8,CONCAT44(uStack_1dc,uStack_1e0));
        uStack_60._0_4_ = CONCAT13(1,(uint3)uStack_60);
        FUN_140fe7b10(&iStack_1e8,pbVar22,pbVar25,(ulonglong)plVar6 & 0xffffffff,auStack_1a8);
        if (iStack_1e8 == 4) {
          return;
        }
        uStack_1b8 = uStack_1d8;
        uStack_1b0 = uStack_1d0;
        iStack_1c8 = iStack_1e8;
        uStack_1c4 = uStack_1e4;
        uStack_1c0 = uStack_1e0;
        uStack_1bc = uStack_1dc;
        puVar19 = &UNK_143052e68;
      }
      else {
        uStack_1b8 = uStack_1d8;
        uStack_1b0 = uStack_1d0;
        iStack_1c8 = iStack_1e8;
        uStack_1c4 = uStack_1e4;
        uStack_1c0 = uStack_1e0;
        uStack_1bc = uStack_1dc;
        puVar19 = &UNK_143052e50;
      }
      FUN_1428d9760(&UNK_143052c50,0x2b,&iStack_1c8,&UNK_143052c30,puVar19);
      pcVar7 = (code *)swi(3);
      (*pcVar7)();
      return;
    case 0xc:
      *(int *)in_RCX[2] = (int)pbVar14;
      pbVar27 = (byte *)*in_RCX;
      pbVar27[0x10] = 0;
      pbVar27[0x11] = 0;
      pbVar27[0x12] = 0;
      pbVar27[0x13] = 0;
    case 0xe:
LAB_142344cd5:
      fVar39 = unaff_XMM9_Da * _DAT_14292c76c;
      puStack_90 = (undefined *)0x142344cf1;
      param_1 = (float)FUN_140c95ef0(&UNK_14304e668);
      param_1 = param_1 * fVar39;
      *(undefined4 *)unaff_R15[2] = *(undefined4 *)((longlong)unaff_R15 + 0x1c);
      iVar34 = *(int *)pbVar27;
      iVar35 = *(int *)(pbVar27 + 4);
      pbVar14 = *(byte **)(*unaff_R15 + 0xa0);
      puVar19 = (undefined *)(*(longlong *)(*unaff_R15 + 0xa8) << 3);
code_r0x000142344d1b:
      pbVar20 = (byte *)((longlong)puVar19 * 5);
      pbVar16 = (byte *)0x0;
      param_3 = pbVar14;
      do {
        uVar30 = SUB84(param_4,0);
        bVar32 = pbVar20 == pbVar16;
code_r0x000142344d33:
        if (bVar32) {
          return;
        }
        pbVar22 = param_3 + 0x10;
        param_3 = param_3 + 0x28;
        uVar23 = CONCAT44(-(uint)((int)((ulonglong)*(undefined8 *)pbVar22 >> 0x20) == iVar35),
                          -(uint)((int)*(undefined8 *)pbVar22 == iVar34));
code_r0x000142344d43:
        auVar36._0_4_ = (undefined4)uVar23;
        uVar37 = (undefined4)((ulonglong)uVar23 >> 0x20);
        auVar36._4_4_ = auVar36._0_4_;
        auVar36._8_4_ = uVar37;
        auVar36._12_4_ = uVar37;
        uVar29 = movmskpd(uVar30,auVar36);
        param_4 = (ulonglong *)(ulonglong)uVar29;
        pbVar16 = pbVar16 + 0x28;
      } while (uVar29 != 3);
      fVar39 = 0.0;
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
      switch(pbVar31[0xf0]) {
      case 0:
        param_1 = (float)((uint)param_1 ^ _UNK_1429258f0);
      case 3:
        fVar33 = 0.0;
        fVar39 = param_1;
        break;
      case 2:
        param_1 = (float)((uint)param_1 ^ _UNK_1429258f0);
      case 1:
        fVar33 = param_1;
      }
      *(float *)(pbVar14 + -0x20 + (longlong)pbVar16) = fVar33;
      *(float *)(pbVar14 + -0x1c + (longlong)pbVar16) = fVar39;
      return;
    case 0xd:
      pbVar27 = (byte *)*in_RCX;
      fVar39 = *(float *)(pbVar27 + 0x10);
      puStack_90 = &UNK_142344b5a;
      cVar12 = func_0x00014093fbe0(*(undefined8 *)pbVar22);
      if (cVar12 == '\0') {
code_r0x000142344be2:
        fVar33 = *(float *)(*plVar6 + 0x58) / _DAT_1429258d8;
code_r0x000142344bf2:
        uVar30 = *(undefined4 *)((longlong)in_RCX + 0x1c);
        puVar21 = (undefined4 *)in_RCX[2];
        *puVar21 = uVar30;
        fVar33 = fVar33 + *(float *)(pbVar27 + 0x10);
      }
      else {
        fStack_68 = _UNK_1429a0788 * unaff_XMM9_Da + _UNK_1429a0788 * unaff_XMM9_Da + unaff_XMM11_Da
        ;
        puStack_90 = &UNK_142344b9a;
        FUN_14123fff0(&pbStack_58);
        puStack_90 = &UNK_142344bae;
        cVar12 = func_0x000141240140(&pbStack_58);
        if (cVar12 == '\0') goto code_r0x000142344be2;
        puStack_90 = &UNK_142344bc9;
        cVar12 = func_0x00014093fa70(*(undefined4 *)*in_stack_00000080);
        if (cVar12 == '\0') {
          fVar33 = *(float *)(*plVar6 + 0x58) / _UNK_142925900;
          goto code_r0x000142344bf2;
        }
        fVar33 = *(float *)(pbVar27 + 0x10);
        uVar30 = *(undefined4 *)((longlong)in_RCX + 0x1c);
        puVar21 = (undefined4 *)in_RCX[2];
      }
      fVar9 = _DAT_142925904;
      fVar38 = 0.0;
      if (0.0 <= fVar33) {
        fVar38 = fVar33;
      }
      fVar33 = _DAT_142925904;
      if (fVar38 <= _DAT_142925904) {
        fVar33 = fVar38;
      }
      *puVar21 = uVar30;
      *(float *)(pbVar27 + 0x10) = fVar33;
      if ((fVar39 != 0.0) || (fVar33 <= 0.0)) {
        if ((fVar39 != _DAT_142925904) ||
           ((NAN(fVar39) || NAN(_DAT_142925904) || (fVar9 <= fVar33)))) goto LAB_142344cd5;
        *(undefined4 *)in_stack_00000098[2] = *(undefined4 *)((longlong)in_stack_00000098 + 0x1c);
        lVar24 = *in_stack_00000098;
        lVar26 = *(longlong *)(lVar24 + 0x30);
        uVar23 = *(undefined8 *)(lVar24 + 0x40);
        if (lVar26 == *(longlong *)(lVar24 + 0x20)) {
          puStack_90 = &UNK_142344cb3;
          FUN_140562c30(lVar24 + 0x20,&UNK_142bb5270);
        }
        lVar15 = *(longlong *)(lVar24 + 0x28);
        lVar18 = lVar26 * 0x20;
        *(undefined4 *)(lVar15 + lVar18) = 0x11;
      }
      else {
        *(undefined4 *)in_stack_00000098[2] = *(undefined4 *)((longlong)in_stack_00000098 + 0x1c);
        lVar24 = *in_stack_00000098;
        lVar26 = *(longlong *)(lVar24 + 0x30);
        uVar23 = *(undefined8 *)(lVar24 + 0x40);
        if (lVar26 == *(longlong *)(lVar24 + 0x20)) {
          puStack_90 = &UNK_142344c64;
          FUN_140562c30(lVar24 + 0x20,&UNK_142bb5270);
        }
        lVar15 = *(longlong *)(lVar24 + 0x28);
        lVar18 = lVar26 * 0x20;
        *(undefined4 *)(lVar15 + lVar18) = 0x10;
      }
      *(undefined8 *)(lVar15 + 0x18 + lVar18) = uVar23;
      *(longlong *)(lVar24 + 0x30) = lVar26 + 1;
      *(longlong *)(lVar24 + 0x40) = *(longlong *)(lVar24 + 0x40) + 1;
      goto LAB_142344cd5;
    case 0xf:
      *pbVar14 = *pbVar14 << 1 | (char)*pbVar14 < '\0';
                    /* WARNING: Could not emulate address calculation at 0x000142344aea */
                    /* WARNING: Treating indirect jump as call */
      (*(code *)(&UNK_14304e6d8 + *(int *)(&UNK_14304e6d8 + (longlong)pbVar14 * 4)))();
      return;
    case 0x10:
      goto code_r0x000142344d33;
    case 0x11:
      uVar30 = movmskps(uVar30,in_XMM2);
      goto code_r0x000142344d43;
    case 0x12:
      piVar1 = (int *)((longlong)plVar6 + 0xf);
      iVar5 = *piVar1;
      *piVar1 = *piVar1 << 1;
      if (iVar5 < 0 == *piVar1 < 0) goto code_r0x000142344d43;
      puVar19 = &UNK_14304e6d7;
      if (*piVar1 == 0) {
        _UNK_14304e6d7 = (int)plVar6;
      }
      goto code_r0x000142344d1b;
    case 0x13:
      UNK_14304e65b =
           UNK_14304e65b + -0x28 +
           (0xfffffffebcfb1927U <
           (ulonglong)(longlong)*(int *)(&UNK_14304e6d8 + (ulonglong)*(byte *)(in_RCX + 0x1e) * 4));
      *pbVar14 = *pbVar14 >> 6;
      goto code_r0x000142344d43;
    }
    pbVar27 = *(byte **)(pbVar22 + 8);
    unaff_RBP = (longlong *)(*(longlong *)(pbVar22 + 0x10) << 5);
    pbVar31 = pbVar27;
    for (unaff_R15 = (longlong *)0x0; unaff_RBP != unaff_R15; unaff_R15 = unaff_R15 + 4) {
      uVar29 = *(uint *)(pbVar31 + 0x10);
      if (uVar29 != 0) {
        uVar3 = *(uint *)(pbVar27 + 0x14 + (longlong)unaff_R15);
        pbVar22 = (byte *)(ulonglong)uVar3;
        cVar12 = *(char *)(in_RCX + 0x1e);
        *uStack_60 = fStack_68;
        if (1 < (byte)(cVar12 - 1U)) {
          puStack_90 = &UNK_142344855;
          FUN_142375b40(plVar6,(byte *)(ulonglong)uVar29,uVar3);
          pbVar14 = (byte *)(ulonglong)*(byte *)(in_RCX + 0x1e);
          goto code_r0x00014234485d;
        }
        puStack_90 = &UNK_14234482c;
        FUN_142375eb0();
        pbVar20 = (byte *)(ulonglong)uVar29;
        switch(*(undefined1 *)(in_RCX + 0x1e)) {
        case 1:
code_r0x0001423447d0:
          *uStack_60 = fStack_68;
          param_3 = (byte *)((ulonglong)pbVar22 & 0xffffffff);
          puStack_90 = &UNK_1423447e8;
          plVar17 = plVar6;
          param_1 = (float)FUN_142376300(plVar6);
          break;
        case 2:
          *uStack_60 = fStack_68;
          pbVar20 = (byte *)(ulonglong)uVar29;
          param_3 = (byte *)(ulonglong)uVar3;
          puStack_90 = &UNK_142344885;
          plVar17 = plVar6;
          param_1 = (float)FUN_142375ce0(plVar6);
        }
      }
      pbVar31 = pbVar31 + 0x20;
    }
    ppbVar13 = &pbStack_50;
    uVar8 = (ulonglong)pbStack_58 & 1;
    pbStack_58 = (byte *)CONCAT71((int7)((ulonglong)plVar17 >> 8),1);
    if (uVar8 != 0) {
      return;
    }
  } while( true );
}

