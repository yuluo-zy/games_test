
undefined8 * FUN_140909310(undefined8 *param_1,longlong param_2)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  code *pcVar7;
  longlong lVar8;
  longlong lVar9;
  undefined8 *puVar10;
  ulonglong uVar11;
  undefined8 uVar12;
  
  lVar9 = param_2 + 0x38;
  uVar11 = 0x8597ee0727b461b3;
  lVar8 = FUN_1400dd6f0(lVar9,0x8597ee0727b461b3,0x8b297ec5ae15f15c);
  if (((lVar8 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar11)) ||
     (uVar11 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar11 * 8), uVar11 == 0)) {
code_r0x000140909644:
    FUN_1428d9310(&UNK_142aaad28);
code_r0x000140909650:
    FUN_1428d9310(&UNK_142aaad40);
code_r0x00014090965c:
    FUN_1428d9310(&UNK_142aaad58);
code_r0x000140909668:
    FUN_1428d9310(&UNK_142aaad70);
  }
  else {
    lVar8 = ~uVar11 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar8) == 0)
    goto code_r0x000140909644;
    uVar1 = *(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar8 + 0x10);
    uVar11 = 0xf0b963d163fe760f;
    lVar8 = FUN_1400dd6f0(lVar9,0xf0b963d163fe760f,0x7cb1ed0b9f8556a8);
    if (((lVar8 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar11)) ||
       (uVar11 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar11 * 8), uVar11 == 0)) {
code_r0x0001409093f2:
      uVar12 = 0;
    }
    else {
      lVar8 = ~uVar11 * 0x60;
      if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar8) == 0)
      goto code_r0x0001409093f2;
      uVar12 = *(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar8 + 0x10);
    }
    uVar11 = 0xa68561b396784eaa;
    lVar8 = FUN_1400dd6f0(lVar9,0xa68561b396784eaa,0xf5fd74e6faf4ed14);
    if (((lVar8 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar11)) ||
       (uVar11 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar11 * 8), uVar11 == 0))
    goto code_r0x000140909650;
    lVar8 = ~uVar11 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar8) == 0)
    goto code_r0x000140909650;
    uVar2 = *(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar8 + 0x10);
    uVar11 = 0x8226cc786e679355;
    lVar8 = FUN_1400dd6f0(lVar9,0x8226cc786e679355,0x65cfda6a55a841ac);
    if (((lVar8 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar11)) ||
       (uVar11 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar11 * 8), uVar11 == 0))
    goto code_r0x00014090965c;
    lVar8 = ~uVar11 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar8) == 0)
    goto code_r0x00014090965c;
    uVar3 = *(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar8 + 0x10);
    uVar11 = 0x874b1541877dacfd;
    lVar8 = FUN_1400dd6f0(lVar9,0x874b1541877dacfd,0x2394c835ed9c6f89);
    if (((lVar8 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar11)) ||
       (uVar11 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar11 * 8), uVar11 == 0))
    goto code_r0x000140909668;
    lVar8 = ~uVar11 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar8) == 0)
    goto code_r0x000140909668;
    uVar4 = *(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar8 + 0x10);
    uVar11 = 0x47c94cb78283a72e;
    lVar8 = FUN_1400dd6f0(lVar9,0x47c94cb78283a72e,0x8c173ded97323e9d);
    if (((lVar8 != 0) && (uVar11 < *(ulonglong *)(param_2 + 0x218))) &&
       (uVar11 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar11 * 8), uVar11 != 0)) {
      lVar8 = ~uVar11 * 0x60;
      if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar8) != 0) {
        uVar5 = *(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar8 + 0x10);
        uVar11 = 0xb65bf3141f93e938;
        lVar9 = FUN_1400dd6f0(lVar9,0xb65bf3141f93e938,0xffcf78a6ae974587);
        if (((lVar9 != 0) && (uVar11 < *(ulonglong *)(param_2 + 0x218))) &&
           (uVar11 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar11 * 8), uVar11 != 0)) {
          lVar9 = ~uVar11 * 0x60;
          if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar9) != 0) {
            uVar6 = *(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar9 + 0x10);
            param_1[6] = uVar12;
            *param_1 = uVar1;
            param_1[1] = uVar2;
            param_1[2] = uVar3;
            param_1[3] = uVar4;
            param_1[4] = uVar6;
            param_1[5] = uVar5;
            return param_1;
          }
        }
        goto code_r0x000140909680;
      }
    }
  }
  FUN_1428d9310(&UNK_142aaad88);
code_r0x000140909680:
  FUN_1428d9310(&UNK_142aaada0);
  pcVar7 = (code *)swi(3);
  puVar10 = (undefined8 *)(*pcVar7)();
  return puVar10;
}

