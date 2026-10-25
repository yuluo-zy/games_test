
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong FUN_142289c40(longlong param_1,float param_2,undefined8 param_3,undefined **param_4)

{
  longlong lVar1;
  longlong lVar2;
  undefined **ppuVar3;
  code *pcVar4;
  undefined **ppuVar5;
  longlong lVar6;
  undefined **ppuVar7;
  float fVar8;
  ulonglong uVar9;
  undefined1 auVar10 [12];
  undefined4 in_XMM1_Dc;
  undefined4 in_XMM1_Dd;
  undefined1 in_XMM2 [16];
  undefined *puStack_90;
  undefined8 uStack_88;
  undefined4 **ppuStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  float afStack_60 [6];
  float *pfStack_48;
  code *pcStack_40;
  float *pfStack_38;
  code *pcStack_30;
  float afStack_24 [3];
  
  afStack_24[1] = -NAN;
  afStack_24[2] = -NAN;
  if ((uint)ABS(param_2) < 0x7f800000) {
    in_XMM2._4_4_ = 0;
    in_XMM2._0_4_ = param_2;
    in_XMM2._8_4_ = in_XMM1_Dc;
    in_XMM2._12_4_ = in_XMM1_Dd;
    if (_DAT_142925904 <= param_2) {
      ppuVar5 = (undefined **)(*(longlong *)(param_1 + 0x10) + 0xfffffffe);
      fVar8 = _DAT_142925904;
LAB_142289d3e:
      ppuVar5 = (undefined **)((ulonglong)ppuVar5 & 0xffffffff);
      if (*(undefined ***)(param_1 + 0x10) <= ppuVar5) goto LAB_142289ddb;
      param_4 = (undefined **)((longlong)ppuVar5 + 1);
      if (param_4 < *(undefined ***)(param_1 + 0x10)) {
        return (ulonglong)
               (uint)(*(float *)(*(longlong *)(param_1 + 8) + 8 + (longlong)ppuVar5 * 8) * fVar8 +
                     *(float *)(*(longlong *)(param_1 + 8) + (longlong)ppuVar5 * 8) *
                     (_DAT_142925904 - fVar8));
      }
      goto LAB_142289dea;
    }
    if (param_2 <= 0.0) {
      ppuVar5 = (undefined **)0x0;
      fVar8 = 0.0;
      goto LAB_142289d3e;
    }
    lVar2 = *(longlong *)(param_1 + 0x20);
    ppuVar3 = *(undefined ***)(param_1 + 0x28);
    afStack_24[0] = param_2;
    if (ppuVar3 == (undefined **)0x1) {
      lVar6 = 0;
LAB_142289cd8:
      ppuVar7 = (undefined **)((lVar6 + 1) - (ulonglong)(param_2 < *(float *)(lVar2 + lVar6 * 4)));
      if (ppuVar7 < ppuVar3) {
        ppuVar5 = (undefined **)((longlong)ppuVar7 + -1);
        if (ppuVar7 == (undefined **)0x0) {
          FUN_142287ee0(afStack_60,lVar2,lVar2 + (longlong)ppuVar3 * 4);
          pfStack_48 = afStack_24;
          pcStack_40 = FUN_140d8c910;
          pcStack_30 = FUN_141c586d0;
          puStack_90 = &UNK_14303a2b8;
          uStack_88 = 2;
          uStack_70 = 0;
          ppuStack_80 = &pfStack_48;
          uStack_78 = 2;
          pfStack_38 = afStack_60;
          FUN_1428d9390(&puStack_90,&UNK_14303a2d8);
          goto LAB_142289edf;
        }
        if (ppuVar3 <= ppuVar5) {
          FUN_1428d9518(ppuVar5,ppuVar3,&UNK_14303a270);
          pcVar4 = (code *)swi(3);
          uVar9 = (*pcVar4)();
          return uVar9;
        }
        afStack_60[0] = *(float *)(lVar2 + (longlong)ppuVar5 * 4);
        fVar8 = *(float *)(lVar2 + (longlong)ppuVar7 * 4);
        pfStack_48 = (float *)CONCAT44(pfStack_48._4_4_,fVar8);
        if ((afStack_60[0] == fVar8) && (!NAN(afStack_60[0]) && !NAN(fVar8))) goto LAB_142289db4;
        fVar8 = (float)func_0x000140c904a0();
        auVar10 = ZEXT812(0);
        if (0.0 <= fVar8) {
          auVar10._4_8_ = 0;
          auVar10._0_4_ = fVar8;
        }
        in_XMM2._12_4_ = 0;
        in_XMM2._0_12_ = auVar10;
        fVar8 = _DAT_142925904;
        if (auVar10._0_4_ <= _DAT_142925904) {
          fVar8 = auVar10._0_4_;
        }
        goto LAB_142289d3e;
      }
    }
    else if (ppuVar3 != (undefined **)0x0) {
      lVar6 = 0;
      param_4 = ppuVar3;
      do {
        lVar1 = ((ulonglong)param_4 >> 1) + lVar6;
        if (*(float *)(lVar2 + lVar1 * 4) <= param_2) {
          lVar6 = lVar1;
        }
        param_4 = (undefined **)((longlong)param_4 - ((ulonglong)param_4 >> 1));
      } while ((undefined **)0x1 < param_4);
      goto LAB_142289cd8;
    }
  }
  else {
    FUN_1428d9430(&UNK_14303a1dd,0x23,&UNK_14303a200);
LAB_142289db4:
    puStack_90 = (undefined *)0x0;
    param_4 = &puStack_90;
    ppuVar5 = (undefined **)FUN_1428ce04f(1,afStack_60,&pfStack_48,param_4,&UNK_14303a288);
LAB_142289ddb:
    FUN_1428d9518(ppuVar5);
LAB_142289dea:
    FUN_1428d9518(param_4);
    afStack_24[0] = in_XMM2._0_4_;
  }
  FUN_142287ee0(afStack_60);
  pfStack_48 = afStack_24;
  pcStack_40 = FUN_140d8c910;
  pcStack_30 = FUN_141c586d0;
  puStack_90 = &UNK_14303a238;
  uStack_88 = 2;
  uStack_70 = 0;
  ppuStack_80 = &pfStack_48;
  uStack_78 = 2;
  pfStack_38 = afStack_60;
  FUN_1428d9390(&puStack_90,&UNK_14303a258);
LAB_142289edf:
                    /* WARNING: Does not return */
  pcVar4 = (code *)invalidInstructionException();
  (*pcVar4)();
}

