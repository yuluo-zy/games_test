
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong FUN_141467ce0(longlong param_1,float param_2)

{
  longlong lVar1;
  longlong lVar2;
  ulonglong uVar3;
  code *pcVar4;
  longlong lVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  undefined4 in_XMM1_Dc;
  undefined4 in_XMM1_Dd;
  undefined1 in_XMM2 [16];
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
    in_XMM2._4_4_ = 0;
    in_XMM2._0_4_ = param_2;
    in_XMM2._8_4_ = in_XMM1_Dc;
    in_XMM2._12_4_ = in_XMM1_Dd;
    if (_DAT_142925904 <= param_2) {
      uVar7 = (ulonglong)(*(int *)(param_1 + 0x10) - 2);
      goto LAB_141467dda;
    }
    uVar7 = 0;
    if (param_2 <= 0.0) goto LAB_141467dda;
    lVar2 = *(longlong *)(param_1 + 0x20);
    uVar3 = *(ulonglong *)(param_1 + 0x28);
    afStack_1c[0] = param_2;
    if (uVar3 == 1) {
      lVar5 = 0;
LAB_141467d80:
      uVar6 = (lVar5 + 1) - (ulonglong)(param_2 < *(float *)(lVar2 + lVar5 * 4));
      if (uVar6 < uVar3) {
        uVar7 = uVar6 - 1;
        if (uVar6 == 0) {
          FUN_14147b5c0(auStack_90,lVar2,lVar2 + uVar3 * 4);
          pfStack_48 = afStack_1c;
          pcStack_40 = FUN_140d8c910;
          pcStack_30 = FUN_1412f7440;
          puStack_78 = &UNK_142c635a0;
          uStack_70 = 2;
          uStack_58 = 0;
          ppfStack_68 = &pfStack_48;
          uStack_60 = 2;
          puStack_38 = auStack_90;
          FUN_1428d9390(&puStack_78,&UNK_142c635c0);
          goto LAB_141467f0b;
        }
        if (uVar3 <= uVar7) {
          FUN_1428d9518(uVar7,uVar3,&UNK_142c63558);
          pcVar4 = (code *)swi(3);
          uVar7 = (*pcVar4)();
          return uVar7;
        }
        fStack_20 = *(float *)(lVar2 + uVar7 * 4);
        fStack_24 = *(float *)(lVar2 + uVar6 * 4);
        if ((fStack_20 != fStack_24) || (NAN(fStack_20) || NAN(fStack_24))) {
          func_0x000140c904a0();
LAB_141467dda:
          return uVar7 & 0xffffffff;
        }
        goto LAB_141467dfe;
      }
    }
    else if (uVar3 != 0) {
      lVar5 = 0;
      uVar7 = uVar3;
      do {
        lVar1 = (uVar7 >> 1) + lVar5;
        if (*(float *)(lVar2 + lVar1 * 4) <= param_2) {
          lVar5 = lVar1;
        }
        uVar7 = uVar7 - (uVar7 >> 1);
      } while (1 < uVar7);
      goto LAB_141467d80;
    }
  }
  else {
    FUN_1428d9430(&UNK_142c634c0,0x23,&UNK_142c634e8);
LAB_141467dfe:
    puStack_78 = (undefined *)0x0;
    FUN_1428ce04f(1,&fStack_20,&fStack_24,&puStack_78,&UNK_142c63570);
    afStack_1c[0] = in_XMM2._0_4_;
  }
  FUN_14147b5c0(auStack_90);
  pfStack_48 = afStack_1c;
  pcStack_40 = FUN_140d8c910;
  pcStack_30 = FUN_1412f7440;
  puStack_78 = &UNK_142c63520;
  uStack_70 = 2;
  uStack_58 = 0;
  ppfStack_68 = &pfStack_48;
  uStack_60 = 2;
  puStack_38 = auStack_90;
  FUN_1428d9390(&puStack_78,&UNK_142c63540);
LAB_141467f0b:
                    /* WARNING: Does not return */
  pcVar4 = (code *)invalidInstructionException();
  (*pcVar4)();
}

