/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
float ******
FUN_140c94a10(float ******param_1,float ******param_2,ulonglong param_3,float ******param_4,
             float ******param_5,float ******param_6,float ******param_7,float ******param_8,
             float ******param_9,int param_10,undefined4 param_11,int param_12,undefined4 param_13)
{
  float fVar1;
  float *****pppppfVar2;
  code *pcVar3;
  float ******ppppppfVar4;
  float ******ppppppfVar5;
  float ******ppppppfVar6;
  float ****ppppfVar7;
  float ******unaff_RSI;
  float ******ppppppfVar8;
  float ******unaff_RDI;
  ulonglong uVar9;
  float ******ppppppfVar10;
  float ******in_R10;
  float ****ppppfVar11;
  float ****ppppfVar12;
  float fVar13;
  undefined4 uVar14;
  float *****pppppfStack_118;
  int iStack_108;
  undefined4 uStack_104;
  ulonglong uStack_100;
  int iStack_f8;
  undefined4 uStack_f4;
  float *****pppppfStack_f0;
  float *****pppppfStack_e8;
  float *****pppppfStack_e0;
  float *****pppppfStack_d8;
  float *****pppppfStack_d0;
  float *****pppppfStack_c8;
  float *****pppppfStack_c0;
  float *****pppppfStack_b8;
  float ***pppfStack_b0;
  float *****pppppfStack_a8;
  float *****pppppfStack_a0;
  ulonglong uStack_98;
  float *****pppppfStack_90;
  float *****pppppfStack_88;
  longlong lStack_80;
  float fStack_54;
  float fStack_50;
  float fStack_4c;
  float fStack_48;
  float fStack_44;
  ppppppfVar5 = param_9;
  iStack_108 = param_10;
  uStack_104 = param_11;
  iStack_f8 = param_12;
  uStack_f4 = param_13;
  pppppfStack_e0 = (float *****)&iStack_108;
  pppppfStack_d8 = (float *****)param_8;
  pppppfStack_d0 = (float *****)param_9;
  pppppfStack_c8 = (float *****)param_6;
  pppppfStack_c0 = (float *****)param_7;
  pppppfStack_b8 = (float *****)&param_9;
  pppfStack_b0 = (float ***)&iStack_f8;
  pppppfStack_a8 = (float *****)0x0;
  pppppfStack_a0 = (float *****)param_9;
  ppppppfVar10 = param_8;
  uStack_100 = param_3;
  pppppfStack_e8 = (float *****)param_2;
  if ((ulonglong)param_9 >> 0x3d == 0) {
    pppppfVar2 = param_1[2];
    ppppfVar11 = (float ****)((ulonglong)pppppfVar2[4] & 0xfffffffffffffffc);
    if ((ppppfVar11 < *pppppfVar2) ||
       ((ulonglong)((longlong)ppppfVar11 - (longlong)*pppppfVar2) <=
        (ulonglong)((longlong)param_9 * 4) &&
        (longlong)param_9 * 4 - ((longlong)ppppfVar11 - (longlong)*pppppfVar2) != 0)) {
LAB_140c95023:
      ppppfVar11 = (float ****)FUN_1428d7780(param_1);
      if (ppppfVar11 == (float ****)0x0) goto LAB_140c95099;
    }
    else {
      ppppfVar11 = (float ****)((longlong)ppppfVar11 + (longlong)param_9 * -4);
      pppppfVar2[4] = ppppfVar11;
      if (ppppfVar11 == (float ****)0x0) goto LAB_140c95023;
    }
    ppppppfVar8 = unaff_RSI;
    if (ppppppfVar5 != (float ******)0x0) {
      ppppppfVar8 = &pppppfStack_e0;
      unaff_RDI = (float ******)0x0;
      do {
        unaff_RSI = ppppppfVar8;
        if (pppppfStack_a0 <= pppppfStack_a8) goto LAB_140c9509e;
        ppppppfVar6 = (float ******)((longlong)unaff_RDI + 1);
        pppppfStack_a8 = (float *****)((longlong)pppppfStack_a8 + 1);
        fVar13 = (float)FUN_141941180(ppppppfVar8);
        *(float *)((longlong)ppppfVar11 + (longlong)unaff_RDI * 4) = fVar13;
        unaff_RDI = ppppppfVar6;
      } while (ppppppfVar5 != ppppppfVar6);
    }
    ppppppfVar6 = param_9;
    unaff_RSI = ppppppfVar8;
    if ((ulonglong)param_9 >> 0x3d != 0) goto LAB_140c95099;
    unaff_RSI = (float ******)((longlong)param_9 * 4);
    pppppfVar2 = param_1[2];
    ppppfVar7 = (float ****)((ulonglong)pppppfVar2[4] & 0xfffffffffffffffc);
    if ((ppppfVar7 < *pppppfVar2) ||
       ((float ******)((longlong)ppppfVar7 - (longlong)*pppppfVar2) <= unaff_RSI &&
        (longlong)unaff_RSI - ((longlong)ppppfVar7 - (longlong)*pppppfVar2) != 0)) {
LAB_140c9503e:
      ppppfVar7 = (float ****)FUN_1428d7780(param_1);
      ppppppfVar8 = unaff_RSI;
      if (ppppfVar7 == (float ****)0x0) goto LAB_140c95099;
    }
    else {
      ppppfVar7 = (float ****)((longlong)ppppfVar7 + (longlong)param_9 * -4);
      pppppfVar2[4] = ppppfVar7;
      if (ppppfVar7 == (float ****)0x0) goto LAB_140c9503e;
    }
    if (ppppppfVar6 != (float ******)0x0) {
      func_0x000142923c20(ppppfVar7,0,unaff_RSI);
    }
    unaff_RDI = param_9;
    unaff_RSI = ppppppfVar8;
    if ((ulonglong)param_9 >> 0x3d != 0) goto LAB_140c95099;
    unaff_RSI = (float ******)((longlong)param_9 * 4);
    pppppfVar2 = param_1[2];
    ppppfVar12 = (float ****)((ulonglong)pppppfVar2[4] & 0xfffffffffffffffc);
    if ((ppppfVar12 < *pppppfVar2) ||
       ((float ******)((longlong)ppppfVar12 - (longlong)*pppppfVar2) <= unaff_RSI &&
        (longlong)unaff_RSI - ((longlong)ppppfVar12 - (longlong)*pppppfVar2) != 0)) {
LAB_140c9505f:
      ppppfVar12 = (float ****)FUN_1428d7780();
      ppppppfVar8 = unaff_RSI;
      if (ppppfVar12 == (float ****)0x0) goto LAB_140c95099;
    }
    else {
      ppppfVar12 = (float ****)((longlong)ppppfVar12 + (longlong)param_9 * -4);
      pppppfVar2[4] = ppppfVar12;
      if (ppppfVar12 == (float ****)0x0) goto LAB_140c9505f;
    }
    if (unaff_RDI != (float ******)0x0) {
      func_0x000142923c20(ppppfVar12,0,unaff_RSI);
    }
    if (ppppppfVar6 == (float ******)0x0) goto LAB_140c9514a;
    uVar14 = 0;
    if (iStack_108 != 0) {
      uVar14 = _DAT_142925870;
    }
    *(undefined4 *)ppppfVar7 = uVar14;
    if (ppppppfVar5 == (float ******)0x0) goto LAB_140c9515a;
    if (iStack_108 == 0) {
      fVar13 = _DAT_142925904;
      if (unaff_RDI != (float ******)0x0) goto LAB_140c94c8b;
LAB_140c94e63:
      ppppppfVar10 = (float ******)0x0;
      ppppppfVar6 = (float ******)0x0;
      pppppfStack_f0 = (float *****)param_1;
      fVar13 = (float)FUN_1428d9518(0,0,&UNK_142b2dd50);
LAB_140c94e73:
      if (unaff_RDI <= ppppppfVar8) goto LAB_140c951e2;
      *(int *)((longlong)ppppfVar12 + ((longlong)ppppppfVar10 + -1) * 4) = 0;
      if (ppppppfVar8 == (float ******)0x0) {
LAB_140c94edc:
        pppppfStack_118 = (float *****)ppppppfVar5;
        pppppfStack_e0 = (float *****)param_8;
        pppppfStack_d0 = (float *****)param_6;
        pppppfStack_c8 = (float *****)param_7;
        pppppfStack_b8 = (float *****)param_5;
        pppppfStack_a8 = (float *****)unaff_RDI;
        pppppfStack_a0 = pppppfStack_e8;
        uStack_98 = uStack_100;
        unaff_RSI = ppppppfVar8;
        ppppppfVar10 = (float ******)pppppfStack_f0;
        in_R10 = ppppppfVar5;
        pppppfStack_d8 = pppppfStack_118;
        pppppfStack_c0 = (float *****)param_4;
        pppfStack_b0 = (float ***)ppppfVar12;
        pppppfStack_88 = (float *****)ppppppfVar8;
        if (ppppppfVar8 < (float ******)0x666666666666667) {
          pppppfVar2 = (float *****)pppppfStack_f0[2];
          ppppfVar11 = (float ****)((ulonglong)pppppfVar2[4] & 0xfffffffffffffffc);
          if ((*pppppfVar2 <= ppppfVar11) &&
             ((ulonglong)((longlong)ppppppfVar8 * 0x14) <=
              (ulonglong)((longlong)ppppfVar11 - (longlong)*pppppfVar2))) {
            unaff_RDI = (float ******)((longlong)ppppfVar11 + (longlong)ppppppfVar8 * -0x14);
            pppppfVar2[4] = (float ****)unaff_RDI;
            if (unaff_RDI != (float ******)0x0) goto LAB_140c94fa1;
          }
          unaff_RDI = (float ******)FUN_1428d7780(pppppfStack_f0,4);
          in_R10 = ppppppfVar5;
          if (unaff_RDI != (float ******)0x0) {
LAB_140c94fa1:
            if (ppppppfVar8 != (float ******)0x0) {
              ppppppfVar5 = (float ******)0x0;
              ppppppfVar6 = unaff_RDI;
              do {
                if (ppppppfVar8 <= ppppppfVar5) goto LAB_140c9509e;
                ppppppfVar5 = (float ******)((longlong)ppppppfVar5 + 1);
                pppppfStack_90 = (float *****)ppppppfVar5;
                FUN_141941510(&fStack_54,&pppppfStack_e0);
                *(float *)(ppppppfVar6 + 2) = fStack_44;
                *(float *)ppppppfVar6 = fStack_54;
                *(float *)((longlong)ppppppfVar6 + 4) = fStack_50;
                *(float *)(ppppppfVar6 + 1) = fStack_4c;
                *(float *)((longlong)ppppppfVar6 + 0xc) = fStack_48;
                ppppppfVar6 = (float ******)((longlong)ppppppfVar6 + 0x14);
              } while (ppppppfVar5 != ppppppfVar8);
            }
            return unaff_RDI;
          }
        }
        goto LAB_140c95099;
      }
LAB_140c94e94:
      pppppfStack_118 = (float *****)ppppppfVar5;
      ppppppfVar4 = ppppppfVar8;
      if ((float ******)((longlong)ppppppfVar10 + -2) < ppppppfVar6) {
        do {
          ppppppfVar10 = (float ******)pppppfStack_f0;
          in_R10 = ppppppfVar5;
          if (unaff_RDI <= ppppppfVar4) goto LAB_140c950b6;
          fVar13 = *(float *)((longlong)ppppfVar7 + ((longlong)ppppppfVar4 + -1) * 4) *
                   *(float *)((longlong)ppppfVar12 + (longlong)ppppppfVar4 * 4);
          *(float *)((longlong)ppppfVar12 + ((longlong)ppppppfVar4 + -1) * 4) =
               *(float *)((longlong)ppppfVar12 + ((longlong)ppppppfVar4 + -1) * 4) - fVar13;
          ppppppfVar4 = (float ******)((longlong)ppppppfVar4 + -1);
        } while (ppppppfVar4 != (float ******)0x0);
        goto LAB_140c94edc;
      }
      goto LAB_140c951c4;
    }
    in_R10 = param_5;
    if (param_5 == (float ******)0x0) goto LAB_140c9516a;
    fVar13 = *(float *)param_4 + *(float *)param_4;
    if (unaff_RDI == (float ******)0x0) goto LAB_140c94e63;
LAB_140c94c8b:
    pppppfStack_118 = (float *****)ppppppfVar5;
    fVar13 = *(float *)ppppfVar11 / fVar13;
    *(float *)ppppfVar12 = fVar13;
    ppppppfVar8 = (float ******)((longlong)param_9 - 1);
    if ((float ******)0x1 < ppppppfVar8) {
      uVar9 = 2;
      if (2 < uStack_100) {
        uVar9 = uStack_100;
      }
      lStack_80 = (longlong)param_5 + (ulonglong)(param_5 == (float ******)0x0);
      ppppppfVar10 = (float ******)0x0;
      do {
        in_R10 = param_5;
        pppppfStack_f0 = (float *****)param_1;
        if ((float ******)(uVar9 - 2) == ppppppfVar10) goto LAB_140c950fa;
        if (param_5 == ppppppfVar10) goto LAB_140c95112;
        if (ppppppfVar6 == ppppppfVar10) goto LAB_140c950d4;
        if ((float ******)(lStack_80 + -1) == ppppppfVar10) goto LAB_140c950e3;
        if ((float ******)((longlong)ppppppfVar6 + -1) == ppppppfVar10) goto LAB_140c950c5;
        fVar13 = *(float *)((longlong)pppppfStack_e8 + ((longlong)ppppppfVar10 + 2) * 4) -
                 *(float *)((longlong)pppppfStack_e8 + (longlong)ppppppfVar10 * 4);
        fVar1 = *(float *)((longlong)param_4 + (longlong)ppppppfVar10 * 4);
        fVar13 = (fVar13 + fVar13) -
                 *(float *)((longlong)ppppfVar7 + (longlong)ppppppfVar10 * 4) * fVar1;
        *(float *)((longlong)ppppfVar7 + ((longlong)ppppppfVar10 + 1) * 4) =
             *(float *)((longlong)param_4 + ((longlong)ppppppfVar10 + 1) * 4) / fVar13;
        in_R10 = param_4;
        if ((float ******)((longlong)ppppppfVar5 + -1) == ppppppfVar10) goto LAB_140c95124;
        in_R10 = (float ******)0x0;
        if ((1 - (longlong)unaff_RDI) + (longlong)ppppppfVar10 == 0) goto LAB_140c95138;
        *(float *)((longlong)ppppfVar12 + ((longlong)ppppppfVar10 + 1) * 4) =
             (*(float *)((longlong)ppppfVar11 + ((longlong)ppppppfVar10 + 1) * 4) -
             fVar1 * *(float *)((longlong)ppppfVar12 + (longlong)ppppppfVar10 * 4)) / fVar13;
        ppppppfVar10 = (float ******)((longlong)ppppppfVar10 + 1);
      } while ((float ******)((longlong)param_9 + -2) != ppppppfVar10);
    }
    ppppppfVar10 = param_9;
    pppppfStack_f0 = (float *****)param_1;
    if (iStack_f8 == 0) goto LAB_140c94e73;
    ppppppfVar4 = (float ******)((longlong)param_9 - 2);
    if (param_5 <= ppppppfVar4) goto LAB_140c9517a;
    if (ppppppfVar6 <= ppppppfVar4) goto LAB_140c95191;
    if (ppppppfVar5 <= ppppppfVar8) goto LAB_140c951a0;
    if (unaff_RDI <= ppppppfVar4) goto LAB_140c951b2;
    if (ppppppfVar8 < unaff_RDI) {
      fVar1 = *(float *)((longlong)param_4 + ((longlong)param_9 + -2) * 4);
      fVar13 = (_DAT_14292e6f0 - *(float *)((longlong)ppppfVar7 + ((longlong)param_9 + -2) * 4)) *
               fVar1;
      *(float *)((longlong)ppppfVar12 + ((longlong)param_9 + -1) * 4) =
           (*(float *)((longlong)ppppfVar11 + ((longlong)param_9 + -1) * 4) -
           fVar1 * *(float *)((longlong)ppppfVar12 + ((longlong)param_9 + -2) * 4)) / fVar13;
      if (ppppppfVar8 != (float ******)0x0) goto LAB_140c94e94;
      goto LAB_140c94edc;
    }
  }
  else {
LAB_140c95099:
    FUN_1428d79d0();
LAB_140c9509e:
    fVar13 = (float)FUN_1428d9330(&UNK_142b2db40,0x22,&UNK_142b2dbc0);
    ppppppfVar8 = unaff_RSI;
LAB_140c950b6:
    ppppppfVar6 = unaff_RDI;
    FUN_1428d9518(fVar13,unaff_RDI,&UNK_142b2de10);
LAB_140c950c5:
    FUN_1428d9518(ppppppfVar6);
LAB_140c950d4:
    FUN_1428d9518(ppppppfVar10);
LAB_140c950e3:
    FUN_1428d9518(lStack_80,in_R10,&UNK_142b2de70);
LAB_140c950fa:
    ppppppfVar10 = (float ******)((longlong)ppppppfVar10 + 2);
    FUN_1428d9518(ppppppfVar10,uStack_100,&UNK_142b2de28);
LAB_140c95112:
    FUN_1428d9518(ppppppfVar10,in_R10,&UNK_142b2de40);
LAB_140c95124:
    FUN_1428d9518(pppppfStack_118,pppppfStack_118,&UNK_142b2dea0);
LAB_140c95138:
    FUN_1428d9518(unaff_RDI,unaff_RDI,&UNK_142b2deb8);
LAB_140c9514a:
    FUN_1428d9518(0,0,&UNK_142b2dd08);
LAB_140c9515a:
    FUN_1428d9518(0,0,&UNK_142b2dd20);
LAB_140c9516a:
    ppppppfVar5 = in_R10;
    ppppppfVar4 = (float ******)FUN_1428d9518(0,0,&UNK_142b2dd38);
LAB_140c9517a:
    ppppppfVar4 = (float ******)FUN_1428d9518(ppppppfVar4,param_5,&UNK_142b2dd80);
LAB_140c95191:
    FUN_1428d9518(ppppppfVar4);
LAB_140c951a0:
    ppppppfVar4 = (float ******)FUN_1428d9518(ppppppfVar8,ppppppfVar5,&UNK_142b2ddb0);
LAB_140c951b2:
    FUN_1428d9518(ppppppfVar4,unaff_RDI,&UNK_142b2ddc8);
LAB_140c951c4:
    FUN_1428d9518();
  }
  FUN_1428d9518(ppppppfVar8,unaff_RDI,&UNK_142b2dde0);
LAB_140c951e2:
  FUN_1428d9518(ppppppfVar8,unaff_RDI,&UNK_142b2dd68);
  pcVar3 = (code *)swi(3);
  ppppppfVar5 = (float ******)(*pcVar3)();
  return ppppppfVar5;
}
