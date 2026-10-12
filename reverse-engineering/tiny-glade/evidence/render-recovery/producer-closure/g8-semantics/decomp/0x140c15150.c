
void FUN_140c15150(longlong *param_1,longlong param_2)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  ulonglong uVar3;
  undefined8 *puVar4;
  undefined8 *puVar5;
  longlong lVar6;
  longlong *plVar7;
  longlong lVar8;
  longlong lVar9;
  longlong lVar10;
  longlong lVar11;
  longlong lStack_e8;
  undefined8 uStack_e0;
  ulonglong uStack_d8;
  longlong lStack_d0;
  longlong lStack_c8;
  longlong lStack_c0;
  longlong lStack_b8;
  longlong lStack_b0;
  longlong *plStack_a8;
  longlong alStack_a0 [2];
  longlong lStack_90;
  longlong lStack_88;
  longlong lStack_80;
  longlong lStack_78;
  longlong lStack_70;
  longlong lStack_68;
  longlong lStack_60;
  longlong lStack_58;
  undefined8 uStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  lStack_68 = 0;
  lStack_60 = 8;
  lStack_58 = 0;
  lStack_80 = 0;
  lStack_78 = 8;
  lStack_70 = 0;
  lStack_c0 = *(longlong *)(param_2 + 8);
  lStack_b0 = *(longlong *)(param_2 + 0x10) * 0x38 + lStack_c0;
  lStack_88 = 8;
  lVar8 = 0;
  lStack_c8 = 0;
  plStack_a8 = param_1;
  while (lStack_c0 != lStack_b0) {
    puVar1 = *(undefined8 **)(lStack_c0 + 8);
    lVar6 = *(longlong *)(lStack_c0 + 0x10);
    puVar4 = puVar1;
    lVar9 = lStack_c0;
    lStack_c0 = lStack_c0 + 0x38;
    while (puVar5 = puVar4, puVar5 != puVar1 + lVar6 * 6) {
      puVar4 = puVar5 + 6;
      if (*(char *)(puVar5 + 5) != '\x02') {
        lStack_c8 = lStack_c8 + puVar5[3];
        (**(code **)(puVar5[1] + 0x60))(&lStack_e8,*puVar5,*(byte *)(lVar9 + 0x30) & 1);
        uVar3 = uStack_d8;
        uVar2 = uStack_e0;
        lVar11 = uStack_d8 * 0x38;
        lStack_90 = lStack_e8;
        lVar10 = lVar8;
        lStack_b8 = lVar9;
        if ((ulonglong)(lStack_68 - lVar8) < uStack_d8) {
          FUN_142909820(&lStack_68,lVar8,uStack_d8,8,0x38);
          lStack_88 = lStack_60;
          lVar10 = lStack_58;
        }
        func_0x000142923c00(lVar10 * 0x38 + lStack_88,uVar2,lVar11);
        lVar10 = uVar3 + lVar10;
        uStack_50 = uVar2;
        lStack_58 = lVar10;
        FUN_1418bf2d0(uVar2,0);
        lVar9 = lStack_b8;
        if (lStack_90 == 0) {
          plVar7 = alStack_a0;
          lVar11 = 0;
        }
        else {
          lVar11 = lStack_90 * 0x38;
          alStack_a0[0] = 8;
          plVar7 = &lStack_d0;
        }
        *plVar7 = lVar11;
        if ((alStack_a0[0] != 0) && (lStack_d0 != 0)) {
          func_0x000140613c20(uStack_50);
        }
        lVar11 = lStack_70;
        uVar2 = puVar5[3];
        if (lStack_70 == lStack_80) {
          FUN_1406ae3b0(&lStack_80,&UNK_142b1b5c0);
        }
        *(longlong *)(lStack_78 + lVar11 * 0x18) = lVar8;
        *(longlong *)(lStack_78 + 8 + lVar11 * 0x18) = lVar10;
        *(undefined8 *)(lStack_78 + 0x10 + lVar11 * 0x18) = uVar2;
        lStack_70 = lVar11 + 1;
        lVar8 = lVar10;
      }
    }
  }
  if (lVar8 == 0) {
    lVar9 = 0;
    lVar6 = 0;
  }
  else {
    lVar10 = 0;
    lVar6 = 0;
    lVar9 = 0;
    do {
      lVar11 = *(longlong *)(lStack_60 + 0x28 + lVar10);
      lVar9 = lVar9 + lVar11;
      if (*(char *)(lStack_60 + 0x30 + lVar10) == '\0') {
        lVar11 = 0;
      }
      lVar6 = lVar6 + lVar11;
      lVar10 = lVar10 + 0x38;
    } while (lVar8 * 0x38 - lVar10 != 0);
  }
  plStack_a8[2] = lStack_58;
  *plStack_a8 = lStack_68;
  plStack_a8[1] = lStack_60;
  plStack_a8[3] = lStack_80;
  plStack_a8[4] = lStack_78;
  plStack_a8[5] = lStack_70;
  plStack_a8[6] = lVar9;
  plStack_a8[7] = lStack_c8;
  plStack_a8[8] = lVar6;
  plStack_a8[9] = lVar9 - lVar6;
  return;
}

