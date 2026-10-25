
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong FUN_141940780(longlong param_1,float param_2)

{
  longlong lVar1;
  longlong lVar2;
  code *pcVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  longlong lVar7;
  undefined1 auStack_90 [24];
  undefined *puStack_78;
  undefined8 uStack_70;
  float **ppfStack_68;
  undefined8 uStack_60;
  undefined8 uStack_58;
  float *pfStack_48;
  code *pcStack_40;
  undefined1 *puStack_38;
  code *pcStack_30;
  float fStack_24;
  float fStack_20;
  float afStack_1c [3];
  
  afStack_1c[1] = -NAN;
  afStack_1c[2] = -NAN;
  if ((uint)ABS(param_2) < 0x7f800000) {
    if (_DAT_142925904 <= param_2) {
      return (ulonglong)(*(int *)(param_1 + 0x10) - 2);
    }
    if (param_2 <= 0.0) {
      return 0;
    }
    lVar2 = *(longlong *)(param_1 + 0x20);
    uVar6 = *(ulonglong *)(param_1 + 0x28);
    if (uVar6 == 1) {
      lVar7 = 0;
LAB_141940810:
      uVar4 = (lVar7 + 1) - (ulonglong)(param_2 < *(float *)(lVar2 + lVar7 * 4));
      if (uVar4 < uVar6) {
        uVar5 = uVar4 - 1;
        if (uVar4 == 0) {
          afStack_1c[0] = param_2;
          FUN_1421801e0(auStack_90,lVar2,lVar2 + uVar6 * 4);
          pfStack_48 = afStack_1c;
          pcStack_40 = FUN_140d8c910;
          pcStack_30 = FUN_141941c50;
          puStack_78 = &UNK_142e715f0;
          uStack_70 = 2;
          uStack_58 = 0;
          ppfStack_68 = &pfStack_48;
          uStack_60 = 2;
          puStack_38 = auStack_90;
          FUN_1428d9390(&puStack_78,&UNK_142e71610);
          goto LAB_1419409a0;
        }
        if (uVar6 <= uVar5) {
          FUN_1428d9518(uVar5,uVar6,&UNK_142e715a8);
          pcVar3 = (code *)swi(3);
          uVar6 = (*pcVar3)();
          return uVar6;
        }
        fStack_20 = *(float *)(lVar2 + uVar5 * 4);
        fStack_24 = *(float *)(lVar2 + uVar4 * 4);
        if ((fStack_20 != fStack_24) || (NAN(fStack_20) || NAN(fStack_24))) {
          return uVar5;
        }
        goto LAB_141940893;
      }
    }
    else if (uVar6 != 0) {
      lVar7 = 0;
      uVar4 = uVar6;
      do {
        lVar1 = (uVar4 >> 1) + lVar7;
        if (*(float *)(lVar2 + lVar1 * 4) <= param_2) {
          lVar7 = lVar1;
        }
        uVar4 = uVar4 - (uVar4 >> 1);
      } while (1 < uVar4);
      goto LAB_141940810;
    }
  }
  else {
    FUN_1428d9430(&UNK_142e71510,0x23,&UNK_142e71538);
LAB_141940893:
    puStack_78 = (undefined *)0x0;
    FUN_1428ce04f(1,&fStack_20,&fStack_24,&puStack_78,&UNK_142e715c0);
  }
  afStack_1c[0] = param_2;
  FUN_1421801e0(auStack_90);
  pfStack_48 = afStack_1c;
  pcStack_40 = FUN_140d8c910;
  pcStack_30 = FUN_141941c50;
  puStack_78 = &UNK_142e71570;
  uStack_70 = 2;
  uStack_58 = 0;
  ppfStack_68 = &pfStack_48;
  uStack_60 = 2;
  puStack_38 = auStack_90;
  FUN_1428d9390(&puStack_78,&UNK_142e71590);
LAB_1419409a0:
                    /* WARNING: Does not return */
  pcVar3 = (code *)invalidInstructionException();
  (*pcVar3)();
}

