
longlong * asset_pipe__texture__with_rgba8(longlong *param_1,longlong param_2,int *param_3,undefined8 *param_4)
{
  char *pcVar1;
  char *pcVar2;
  undefined8 uVar3;
  undefined1 auVar4 [16];
  code *pcVar5;
  ulonglong uVar6;
  longlong lVar7;
  undefined4 *puVar8;
  undefined8 uVar9;
  undefined4 uVar10;
  undefined8 uStack_a8;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  uint uStack_88;
  uint uStack_84;
  longlong lStack_80;
  undefined8 uStack_78;
  longlong lStack_70;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  uint uStack_48;
  uint uStack_44;
  undefined1 uStack_39;
  undefined8 uStack_38;
  uStack_38 = 0xfffffffffffffffe;
  if (*(char *)(param_2 + 0x4c) == '\0') {
    if (*param_3 == 3) {
      if (*(char *)*param_4 != '\0') {
        if (*(char *)param_4[2] != '\x03') {
          asset_pipe__texture__compress(param_1,param_3 + 2,param_4[1]);
          return param_1;
        }
        goto LAB_1420e32f7;
      }
      param_4 = *(undefined8 **)(param_3 + 6);
      if ((longlong)param_4 < 0) goto LAB_1420e3303;
      uVar3 = *(undefined8 *)(param_3 + 4);
      if (param_4 == (undefined8 *)0x0) {
        lVar7 = 1;
LAB_1420e325f:
        func_0x000142923c00(lVar7,uVar3,param_4);
        *param_1 = (longlong)param_4;
        param_1[1] = lVar7;
        param_1[2] = (longlong)param_4;
        return param_1;
      }
      uVar9 = 1;
      lVar7 = func_0x000140613c10(param_4,1);
      if (lVar7 != 0) goto LAB_1420e325f;
    }
    else {
      FUN_1428d9310(&UNK_142fdec70);
LAB_1420e32f7:
      FUN_1428d9310(&UNK_142fdec40);
LAB_1420e3303:
      uVar9 = 0;
    }
    FUN_1428d8fe3(uVar9,param_4,&UNK_142fdec08);
  }
  else {
    FUN_140bad960(&uStack_60,param_3);
    auVar4 = ZEXT416(uStack_48) * (undefined1  [16])0x4 * ZEXT416(uStack_44);
    uVar6 = auVar4._0_8_;
    if (auVar4._8_8_ != 0) {
      uStack_39 = 1;
      FUN_1428d9310(&UNK_142fded60);
      goto LAB_1420e333b;
    }
    if (CONCAT44(uStack_4c,uStack_50) < uVar6) {
      uStack_39 = 1;
      FUN_1428d9840(uVar6,CONCAT44(uStack_4c,uStack_50),&UNK_142fded78);
      goto LAB_1420e333b;
    }
    if (uVar6 != 0) {
      puVar8 = (undefined4 *)CONCAT44(uStack_54,uStack_58);
      do {
        uStack_39 = 1;
        uVar10 = asset_pipe__texture__srgb_oetf(*puVar8);
        *puVar8 = uVar10;
        uStack_39 = 1;
        uVar10 = asset_pipe__texture__srgb_oetf(puVar8[1]);
        puVar8[1] = uVar10;
        uStack_39 = 1;
        uVar10 = asset_pipe__texture__srgb_oetf(puVar8[2]);
        puVar8[2] = uVar10;
        puVar8 = puVar8 + 4;
        uVar6 = uVar6 - 4;
      } while (uVar6 != 0);
    }
    pcVar1 = (char *)*param_4;
    uVar9 = param_4[1];
    pcVar2 = (char *)param_4[2];
    uStack_90 = uStack_50;
    uStack_8c = uStack_4c;
    uStack_88 = uStack_48;
    uStack_84 = uStack_44;
    uStack_a0 = uStack_60;
    uStack_9c = uStack_5c;
    uStack_98 = uStack_58;
    uStack_94 = uStack_54;
    uStack_a8 = 9;
    uStack_39 = 0;
    FUN_140bad9f0(&lStack_80,&uStack_a8);
    uVar3 = uStack_78;
    if (*pcVar1 == '\0') {
      if (-1 < lStack_70) {
        if (lStack_70 == 0) {
          lVar7 = 1;
        }
        else {
          uVar9 = 1;
          lVar7 = func_0x000140613c10(lStack_70,1);
          if (lVar7 == 0) goto LAB_1420e3329;
        }
        func_0x000142923c00(lVar7,uVar3,lStack_70);
        *param_1 = lStack_70;
        param_1[1] = lVar7;
        param_1[2] = lStack_70;
joined_r0x0001420e32a0:
        if (lStack_80 != 0) {
          func_0x000140613c20(uStack_78,lStack_80,1);
        }
        return param_1;
      }
      uVar9 = 0;
LAB_1420e3329:
      FUN_1428d8fe3(uVar9,lStack_70,&UNK_142fdec08);
      goto LAB_1420e333b;
    }
    if (*pcVar2 != '\x03') {
      asset_pipe__texture__compress(param_1,&lStack_80,uVar9);
      goto joined_r0x0001420e32a0;
    }
  }
  FUN_1428d9310(&UNK_142fdec40);
LAB_1420e333b:
                    /* WARNING: Does not return */
  pcVar5 = (code *)invalidInstructionException();
  (*pcVar5)();
}
