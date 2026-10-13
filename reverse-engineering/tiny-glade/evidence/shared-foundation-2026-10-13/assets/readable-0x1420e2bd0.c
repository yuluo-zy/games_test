
void asset_pipe__texture__with_rgba8(longlong param_1,int *param_2,undefined8 *param_3)
{
  char *pcVar1;
  longlong *plVar2;
  char *pcVar3;
  undefined1 auVar4 [16];
  code *pcVar5;
  undefined8 uVar6;
  ulonglong uVar7;
  undefined4 *puVar8;
  longlong lVar9;
  longlong lVar10;
  undefined8 uVar11;
  undefined4 uVar12;
  longlong lStack_b8;
  undefined8 uStack_b0;
  ulonglong uStack_a8;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  uint uStack_80;
  uint uStack_7c;
  longlong lStack_78;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  uint uStack_58;
  uint uStack_54;
  undefined8 uStack_50;
  longlong lStack_48;
  undefined1 uStack_39;
  undefined8 uStack_38;
  uStack_38 = 0xfffffffffffffffe;
  if (*(char *)(param_1 + 0x4c) == '\0') {
    if (*param_2 == 3) {
      plVar2 = (longlong *)param_3[1];
      if (*(char *)*param_3 == '\0') {
        uVar11 = *(undefined8 *)(param_2 + 4);
        uVar7 = *(ulonglong *)(param_2 + 6);
        lVar9 = plVar2[2];
        if ((ulonglong)(*plVar2 - lVar9) < uVar7) {
          alloc__raw_vec__impl$4__reserve__do_reserve_and_handle(plVar2,lVar9,uVar7,1,1);
          lVar9 = plVar2[2];
        }
        func_0x000142923c00(plVar2[1] + lVar9,uVar11,uVar7);
        plVar2[2] = lVar9 + uVar7;
        return;
      }
      if (*(char *)param_3[3] != '\x03') {
        asset_pipe__texture__compress(&lStack_78,param_2 + 2,param_3[2]);
        uVar7 = CONCAT44(uStack_64,uStack_68);
        lVar9 = plVar2[2];
        lVar10 = lStack_78;
        uVar11 = CONCAT44(uStack_6c,uStack_70);
        if ((ulonglong)(*plVar2 - lVar9) < uVar7) {
          lStack_48 = lStack_78;
          uStack_50 = CONCAT44(uStack_6c,uStack_70);
          alloc__raw_vec__impl$4__reserve__do_reserve_and_handle(plVar2,lVar9,uVar7,1,1);
          lVar9 = plVar2[2];
          lVar10 = lStack_48;
          uVar11 = uStack_50;
        }
        func_0x000142923c00(plVar2[1] + lVar9,uVar11,uVar7);
        plVar2[2] = lVar9 + uVar7;
        lStack_b8 = lVar10;
        uStack_b0 = uVar11;
        goto joined_r0x0001420e2dab;
      }
    }
    else {
      FUN_1428d9310(&UNK_142fdec70);
    }
    FUN_1428d9310(&UNK_142fdec58);
  }
  else {
    FUN_140bad960(&uStack_98);
    auVar4 = ZEXT416(uStack_80) * (undefined1  [16])0x4 * ZEXT416(uStack_7c);
    uVar7 = auVar4._0_8_;
    if (auVar4._8_8_ != 0) {
      uStack_39 = 1;
      FUN_1428d9310(&UNK_142fded60);
      goto LAB_1420e2f58;
    }
    if (CONCAT44(uStack_84,uStack_88) < uVar7) {
      uStack_39 = 1;
      FUN_1428d9840(uVar7,CONCAT44(uStack_84,uStack_88),&UNK_142fded78);
      goto LAB_1420e2f58;
    }
    if (uVar7 != 0) {
      puVar8 = (undefined4 *)CONCAT44(uStack_8c,uStack_90);
      do {
        uStack_39 = 1;
        uVar12 = asset_pipe__texture__srgb_oetf(*puVar8);
        *puVar8 = uVar12;
        uStack_39 = 1;
        uVar12 = asset_pipe__texture__srgb_oetf(puVar8[1]);
        puVar8[1] = uVar12;
        uStack_39 = 1;
        uVar12 = asset_pipe__texture__srgb_oetf(puVar8[2]);
        puVar8[2] = uVar12;
        puVar8 = puVar8 + 4;
        uVar7 = uVar7 - 4;
      } while (uVar7 != 0);
    }
    pcVar1 = (char *)*param_3;
    plVar2 = (longlong *)param_3[1];
    uVar11 = param_3[2];
    pcVar3 = (char *)param_3[3];
    uStack_60 = uStack_88;
    uStack_5c = uStack_84;
    uStack_58 = uStack_80;
    uStack_54 = uStack_7c;
    uStack_70 = uStack_98;
    uStack_6c = uStack_94;
    uStack_68 = uStack_90;
    uStack_64 = uStack_8c;
    lStack_78 = 9;
    uStack_39 = 0;
    FUN_140bad9f0(&lStack_b8,&lStack_78);
    uVar6 = uStack_b0;
    if (*pcVar1 == '\0') {
      lVar9 = plVar2[2];
      if ((ulonglong)(*plVar2 - lVar9) < uStack_a8) {
        alloc__raw_vec__impl$4__reserve__do_reserve_and_handle(plVar2,lVar9,uStack_a8,1,1);
        lVar9 = plVar2[2];
      }
      func_0x000142923c00(plVar2[1] + lVar9,uVar6,uStack_a8);
      plVar2[2] = lVar9 + uStack_a8;
joined_r0x0001420e2dab:
      if (lStack_b8 != 0) {
        func_0x000140613c20(uStack_b0,lStack_b8,1);
      }
      return;
    }
    if (*pcVar3 != '\x03') {
      asset_pipe__texture__compress(&lStack_78,&lStack_b8,uVar11);
      uVar7 = CONCAT44(uStack_64,uStack_68);
      lVar9 = plVar2[2];
      lVar10 = lStack_78;
      uVar11 = CONCAT44(uStack_6c,uStack_70);
      if ((ulonglong)(*plVar2 - lVar9) < uVar7) {
        lStack_48 = lStack_78;
        uStack_50 = CONCAT44(uStack_6c,uStack_70);
        alloc__raw_vec__impl$4__reserve__do_reserve_and_handle(plVar2,lVar9,uVar7,1,1);
        lVar9 = plVar2[2];
        lVar10 = lStack_48;
        uVar11 = uStack_50;
      }
      func_0x000142923c00(plVar2[1] + lVar9,uVar11,uVar7);
      plVar2[2] = lVar9 + uVar7;
      if (lVar10 != 0) {
        func_0x000140613c20(uVar11,lVar10,1);
      }
      goto joined_r0x0001420e2dab;
    }
  }
  FUN_1428d9310(&UNK_142fdec58);
LAB_1420e2f58:
                    /* WARNING: Does not return */
  pcVar5 = (code *)invalidInstructionException();
  (*pcVar5)();
}
