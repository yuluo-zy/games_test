
undefined8 FUN_1408c5df0(longlong param_1,longlong param_2)

{
  int *piVar1;
  int iVar2;
  ulonglong uVar3;
  longlong lVar4;
  ulonglong uVar5;
  longlong lVar6;
  code *pcVar7;
  undefined8 uVar8;
  undefined *puStack_128;
  undefined8 uStack_120;
  longlong lStack_118;
  undefined *puStack_110;
  undefined **ppuStack_108;
  undefined *puStack_100;
  undefined *puStack_f8;
  longlong lStack_f0;
  longlong *plStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  longlong lStack_d0;
  longlong lStack_c8;
  undefined4 uStack_c0;
  int iStack_bc;
  undefined8 uStack_b8;
  longlong lStack_b0;
  longlong lStack_a8;
  undefined4 uStack_a0;
  int iStack_9c;
  undefined8 uStack_98;
  longlong lStack_90;
  longlong lStack_88;
  undefined4 uStack_80;
  int iStack_7c;
  undefined8 uStack_78;
  longlong lStack_70;
  longlong lStack_68;
  undefined4 uStack_60;
  int iStack_5c;
  longlong alStack_58 [4];
  
  LOCK();
  piVar1 = (int *)(param_2 + 0x5b0);
  iVar2 = *piVar1;
  *piVar1 = *piVar1 + 1;
  UNLOCK();
  if (SBORROW8(0,*(longlong *)(param_1 + 0x160))) goto code_r0x0001408c61b5;
  uVar3 = *(ulonglong *)(param_2 + 0x218);
  if (*(ulonglong *)(param_1 + 0x138) < uVar3) {
    lVar4 = *(longlong *)(param_2 + 0x210);
    uVar5 = *(ulonglong *)(lVar4 + *(ulonglong *)(param_1 + 0x138) * 8);
    if (uVar5 == 0) goto code_r0x0001408c60b5;
    lVar6 = *(longlong *)(param_2 + 0x1e0);
    lStack_f0 = ~uVar5 * 0x60;
    if (*(longlong *)(lVar6 + 0x20 + lStack_f0) == 0) goto code_r0x0001408c60b5;
    if (((*(ulonglong *)(param_1 + 0x140) < uVar3) &&
        (uVar5 = *(ulonglong *)(lVar4 + *(ulonglong *)(param_1 + 0x140) * 8), uVar5 != 0)) &&
       (lStack_d0 = ~uVar5 * 0x60, *(longlong *)(lVar6 + 0x20 + lStack_d0) != 0)) {
      if (((*(ulonglong *)(param_1 + 0x148) < uVar3) &&
          (uVar5 = *(ulonglong *)(lVar4 + *(ulonglong *)(param_1 + 0x148) * 8), uVar5 != 0)) &&
         (lStack_b0 = ~uVar5 * 0x60, *(longlong *)(lVar6 + 0x20 + lStack_b0) != 0)) {
        if (((*(ulonglong *)(param_1 + 0x150) < uVar3) &&
            (uVar5 = *(ulonglong *)(lVar4 + *(ulonglong *)(param_1 + 0x150) * 8), uVar5 != 0)) &&
           (lStack_90 = ~uVar5 * 0x60, *(longlong *)(lVar6 + 0x20 + lStack_90) != 0)) {
          if (((*(ulonglong *)(param_1 + 0x158) < uVar3) &&
              (uVar3 = *(ulonglong *)(lVar4 + *(ulonglong *)(param_1 + 0x158) * 8), uVar3 != 0)) &&
             (lStack_68 = ~uVar3 * 0x60, *(longlong *)(lVar6 + 0x20 + lStack_68) != 0)) {
            lStack_f0 = lStack_f0 + lVar6;
            puStack_f8 = *(undefined **)(lStack_f0 + 0x10);
            lStack_d0 = lStack_d0 + lVar6;
            uStack_d8 = *(undefined8 *)(lStack_d0 + 0x10);
            lStack_b0 = lStack_b0 + lVar6;
            uStack_b8 = *(undefined8 *)(lStack_b0 + 0x10);
            lStack_90 = lStack_90 + lVar6;
            uStack_98 = *(undefined8 *)(lStack_90 + 0x10);
            lStack_68 = lStack_68 + lVar6;
            uStack_78 = *(undefined8 *)(lStack_68 + 0x10);
            plStack_e8 = (longlong *)(lStack_f0 + 0x5c);
            lStack_f0 = lStack_f0 + 0x58;
            uStack_c0 = *(undefined4 *)(param_1 + 0x130);
            uStack_e0 = CONCAT44(iVar2,uStack_c0);
            lStack_c8 = lStack_d0 + 0x5c;
            lStack_d0 = lStack_d0 + 0x58;
            lStack_a8 = lStack_b0 + 0x5c;
            lStack_b0 = lStack_b0 + 0x58;
            lStack_88 = lStack_90 + 0x5c;
            lStack_90 = lStack_90 + 0x58;
            alStack_58[2] = param_1 + 0x160;
            lStack_70 = lStack_68 + 0x58;
            lStack_68 = lStack_68 + 0x5c;
            alStack_58[1] = 0;
            iStack_bc = iVar2;
            uStack_a0 = uStack_c0;
            iStack_9c = iVar2;
            uStack_80 = uStack_c0;
            iStack_7c = iVar2;
            uStack_60 = uStack_c0;
            iStack_5c = iVar2;
            alStack_58[0] = param_2;
            FUN_140a92670(&puStack_f8,&uStack_d8,&uStack_b8,&uStack_98,&uStack_78,alStack_58);
            *(int *)(param_1 + 0x130) = iVar2;
            return 0;
          }
          puStack_128 = &UNK_142aa26b4;
          uStack_120 = 0x35;
        }
        else {
          puStack_128 = &UNK_142aa271c;
          uStack_120 = 0x41;
        }
      }
      else {
        puStack_128 = &UNK_142aa26e9;
        uStack_120 = 0x33;
      }
    }
    else {
      puStack_128 = &UNK_142aa2527;
      uStack_120 = 0x3d;
    }
  }
  else {
code_r0x0001408c60b5:
    puStack_128 = &UNK_142aa2688;
    uStack_120 = 0x2c;
  }
  lStack_118 = param_1 + 0x98;
  puStack_110 = &UNK_14003bad0;
  ppuStack_108 = &puStack_128;
  puStack_100 = &UNK_140109e90;
  puStack_f8 = &UNK_142aa2590;
  lStack_f0 = 2;
  uStack_d8 = 0;
  plStack_e8 = &lStack_118;
  uStack_e0 = 2;
  FUN_1428d9390(&puStack_f8,&UNK_142aa2618);
code_r0x0001408c61b5:
  FUN_1428d9330(&UNK_142aa2950,0x59,&UNK_142aa29b0);
  pcVar7 = (code *)swi(3);
  uVar8 = (*pcVar7)();
  return uVar8;
}

