
void FUN_1420194d0(ulonglong *param_1,undefined8 param_2,longlong param_3,undefined8 param_4,
                  char *param_5,undefined8 param_6,longlong param_7,longlong param_8,
                  undefined8 param_9,uint param_10,undefined4 param_11,undefined8 *param_12,
                  char param_13)

{
  undefined4 uVar1;
  char cVar2;
  undefined4 uVar3;
  undefined8 uVar4;
  ulonglong uVar5;
  code *pcVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  ulonglong uVar19;
  longlong lVar20;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  ulonglong *puStack_88;
  ulonglong uStack_80;
  undefined1 uStack_71;
  undefined8 uStack_70;
  
  uStack_70 = 0xfffffffffffffffe;
  uStack_b8 = param_12[8];
  uStack_b0 = param_12[9];
  uStack_a8 = param_12[10];
  uStack_a0 = param_12[0xb];
  uStack_98 = param_12[0xc];
  uStack_90 = param_12[0xd];
  uVar9 = *param_12;
  uVar10 = param_12[1];
  uVar15 = param_12[2];
  uVar16 = param_12[3];
  uVar13 = param_12[4];
  uVar14 = param_12[5];
  uVar17 = param_12[0x13];
  uVar11 = param_12[6];
  uVar12 = param_12[7];
  uVar18 = param_12[0xe];
  uVar7 = *(undefined4 *)(param_12 + 0xf);
  uVar8 = *(undefined4 *)((longlong)param_12 + 0x7c);
  uVar3 = *(undefined4 *)(param_12 + 0x15);
  uVar4 = param_12[0x14];
  uVar19 = *param_1;
  uStack_80 = *(ulonglong *)(param_3 + 0x40);
  if ((uVar19 < uStack_80) &&
     (uVar5 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar19 * 8), uVar5 != 0)) {
    lVar20 = ~uVar5 * 0x30 + *(longlong *)(param_3 + 0x18);
  }
  else {
    lVar20 = 0;
  }
  uVar1 = *(undefined4 *)((longlong)param_12 + 0xac);
  puStack_88 = param_1;
  if (*param_5 == '\0') {
    func_0x000142923c10(*(longlong *)(lVar20 + 8) * (ulonglong)param_10 +
                        *(longlong *)(lVar20 + 0x10),&uStack_b8);
    *(undefined4 *)(*(longlong *)(lVar20 + 0x20) + (ulonglong)param_10 * 4) = param_11;
LAB_14201964a:
    *(undefined4 *)(*(longlong *)(lVar20 + 0x28) + (ulonglong)param_10 * 4) = param_11;
  }
  else {
    if (param_13 == '\0') {
      uStack_71 = 1;
      FUN_140cff400(lVar20,param_10,&uStack_b8);
      goto LAB_14201964a;
    }
    if (((uVar19 < uStack_80) &&
        (uVar19 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar19 * 8), uVar19 != 0)) &&
       (pcVar6 = *(code **)(*(longlong *)(param_3 + 0x18) + 0x18 + ~uVar19 * 0x30),
       pcVar6 != (code *)0x0)) {
      uStack_71 = 1;
      (*pcVar6)(&uStack_b8);
    }
  }
  uStack_a8 = param_12[0x12];
  uStack_b8 = param_12[0x10];
  uStack_b0 = param_12[0x11];
  uVar19 = puStack_88[1];
  if ((uVar19 < uStack_80) &&
     (uVar5 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar19 * 8), uVar5 != 0)) {
    lVar20 = ~uVar5 * 0x30 + *(longlong *)(param_3 + 0x18);
    if (param_5[1] == '\0') goto LAB_14201969d;
LAB_1420196d2:
    if (param_13 == '\0') {
      uStack_71 = 1;
      FUN_140cff400(lVar20,param_10,&uStack_b8);
      goto LAB_142019729;
    }
    if (((uVar19 < uStack_80) &&
        (uVar19 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar19 * 8), uVar19 != 0)) &&
       (pcVar6 = *(code **)(*(longlong *)(param_3 + 0x18) + 0x18 + ~uVar19 * 0x30),
       pcVar6 != (code *)0x0)) {
      uStack_71 = 1;
      (*pcVar6)(&uStack_b8);
    }
  }
  else {
    lVar20 = 0;
    if (param_5[1] != '\0') goto LAB_1420196d2;
LAB_14201969d:
    func_0x000142923c10(*(longlong *)(lVar20 + 8) * (ulonglong)param_10 +
                        *(longlong *)(lVar20 + 0x10),&uStack_b8);
    *(undefined4 *)(*(longlong *)(lVar20 + 0x20) + (ulonglong)param_10 * 4) = param_11;
LAB_142019729:
    *(undefined4 *)(*(longlong *)(lVar20 + 0x28) + (ulonglong)param_10 * 4) = param_11;
  }
  uVar19 = puStack_88[2];
  uStack_b8 = uVar9;
  uStack_b0 = uVar10;
  uStack_a8 = uVar15;
  uStack_a0 = uVar16;
  uStack_98 = uVar13;
  uStack_90 = uVar14;
  if ((uVar19 < uStack_80) &&
     (uVar5 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar19 * 8), uVar5 != 0)) {
    lVar20 = ~uVar5 * 0x30 + *(longlong *)(param_3 + 0x18);
    if (param_5[2] == '\0') goto LAB_142019780;
LAB_1420197b5:
    if (param_13 == '\0') {
      uStack_71 = 0;
      FUN_140cff400(lVar20,param_10,&uStack_b8);
      goto LAB_14201981a;
    }
    if (((uVar19 < uStack_80) &&
        (uVar19 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar19 * 8), uVar19 != 0)) &&
       (pcVar6 = *(code **)(*(longlong *)(param_3 + 0x18) + 0x18 + ~uVar19 * 0x30),
       pcVar6 != (code *)0x0)) {
      uStack_71 = 0;
      (*pcVar6)(&uStack_b8);
    }
    uVar19 = puStack_88[3];
    if (uVar19 < uStack_80) goto LAB_142019830;
LAB_14201987e:
    lVar20 = 0;
    if (param_5[3] == '\0') goto LAB_142019859;
LAB_14201988e:
    if (param_13 == '\0') {
      uStack_71 = 0;
      FUN_140cff400(lVar20,param_10,&uStack_b8);
      goto LAB_1420198e5;
    }
    if (((uVar19 < uStack_80) &&
        (uVar19 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar19 * 8), uVar19 != 0)) &&
       (pcVar6 = *(code **)(*(longlong *)(param_3 + 0x18) + 0x18 + ~uVar19 * 0x30),
       pcVar6 != (code *)0x0)) {
      uStack_71 = 0;
      (*pcVar6)(&uStack_b8);
    }
  }
  else {
    lVar20 = 0;
    if (param_5[2] != '\0') goto LAB_1420197b5;
LAB_142019780:
    func_0x000142923c10(*(longlong *)(lVar20 + 8) * (ulonglong)param_10 +
                        *(longlong *)(lVar20 + 0x10),&uStack_b8);
    *(undefined4 *)(*(longlong *)(lVar20 + 0x20) + (ulonglong)param_10 * 4) = param_11;
LAB_14201981a:
    *(undefined4 *)(*(longlong *)(lVar20 + 0x28) + (ulonglong)param_10 * 4) = param_11;
    uVar19 = puStack_88[3];
    if (uStack_80 <= uVar19) goto LAB_14201987e;
LAB_142019830:
    uVar5 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar19 * 8);
    if (uVar5 == 0) goto LAB_14201987e;
    lVar20 = ~uVar5 * 0x30 + *(longlong *)(param_3 + 0x18);
    if (param_5[3] != '\0') goto LAB_14201988e;
LAB_142019859:
    func_0x000142923c10(*(longlong *)(lVar20 + 8) * (ulonglong)param_10 +
                        *(longlong *)(lVar20 + 0x10),&uStack_b8);
    *(undefined4 *)(*(longlong *)(lVar20 + 0x20) + (ulonglong)param_10 * 4) = param_11;
LAB_1420198e5:
    *(undefined4 *)(*(longlong *)(lVar20 + 0x28) + (ulonglong)param_10 * 4) = param_11;
  }
  uVar19 = puStack_88[4];
  uStack_b8 = uVar17;
  if ((uVar19 < uStack_80) &&
     (uVar5 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar19 * 8), uVar5 != 0)) {
    lVar20 = ~uVar5 * 0x30 + *(longlong *)(param_3 + 0x18);
    if (param_5[4] == '\0') goto LAB_14201992f;
LAB_142019964:
    if (param_13 == '\0') {
      uStack_71 = 0;
      FUN_140cff400(lVar20,param_10,&uStack_b8);
      goto LAB_1420199bb;
    }
    if (((uVar19 < uStack_80) &&
        (uVar19 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar19 * 8), uVar19 != 0)) &&
       (pcVar6 = *(code **)(*(longlong *)(param_3 + 0x18) + 0x18 + ~uVar19 * 0x30),
       pcVar6 != (code *)0x0)) {
      uStack_71 = 0;
      (*pcVar6)(&uStack_b8);
    }
  }
  else {
    lVar20 = 0;
    if (param_5[4] != '\0') goto LAB_142019964;
LAB_14201992f:
    func_0x000142923c10(*(longlong *)(lVar20 + 8) * (ulonglong)param_10 +
                        *(longlong *)(lVar20 + 0x10),&uStack_b8);
    *(undefined4 *)(*(longlong *)(lVar20 + 0x20) + (ulonglong)param_10 * 4) = param_11;
LAB_1420199bb:
    *(undefined4 *)(*(longlong *)(lVar20 + 0x28) + (ulonglong)param_10 * 4) = param_11;
  }
  uVar19 = puStack_88[5];
  uStack_b8 = uVar11;
  uStack_b0 = uVar12;
  if ((uVar19 < uStack_80) &&
     (uVar5 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar19 * 8), uVar5 != 0)) {
    lVar20 = ~uVar5 * 0x30 + *(longlong *)(param_3 + 0x18);
    if (param_5[5] == '\0') goto LAB_142019a02;
LAB_142019a37:
    if (param_13 == '\0') {
      uStack_71 = 0;
      FUN_140cff400(lVar20,param_10,&uStack_b8);
      goto LAB_142019a8e;
    }
    if (((uVar19 < uStack_80) &&
        (uVar19 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar19 * 8), uVar19 != 0)) &&
       (pcVar6 = *(code **)(*(longlong *)(param_3 + 0x18) + 0x18 + ~uVar19 * 0x30),
       pcVar6 != (code *)0x0)) {
      uStack_71 = 0;
      (*pcVar6)(&uStack_b8);
    }
  }
  else {
    lVar20 = 0;
    if (param_5[5] != '\0') goto LAB_142019a37;
LAB_142019a02:
    func_0x000142923c10(*(longlong *)(lVar20 + 8) * (ulonglong)param_10 +
                        *(longlong *)(lVar20 + 0x10),&uStack_b8);
    *(undefined4 *)(*(longlong *)(lVar20 + 0x20) + (ulonglong)param_10 * 4) = param_11;
LAB_142019a8e:
    *(undefined4 *)(*(longlong *)(lVar20 + 0x28) + (ulonglong)param_10 * 4) = param_11;
  }
  uVar19 = puStack_88[6];
  uStack_b8 = uVar18;
  uStack_b0._4_4_ = uVar8;
  uStack_b0._0_4_ = uVar7;
  if ((uVar19 < uStack_80) &&
     (uVar5 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar19 * 8), uVar5 != 0)) {
    lVar20 = ~uVar5 * 0x30 + *(longlong *)(param_3 + 0x18);
    if (param_5[6] == '\0') goto LAB_142019ad5;
LAB_142019b0a:
    if (param_13 == '\0') {
      uStack_71 = 0;
      FUN_140cff400(lVar20,param_10,&uStack_b8);
      goto LAB_142019b61;
    }
    if (((uVar19 < uStack_80) &&
        (uVar19 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar19 * 8), uVar19 != 0)) &&
       (pcVar6 = *(code **)(*(longlong *)(param_3 + 0x18) + 0x18 + ~uVar19 * 0x30),
       pcVar6 != (code *)0x0)) {
      uStack_71 = 0;
      (*pcVar6)(&uStack_b8);
    }
  }
  else {
    lVar20 = 0;
    if (param_5[6] != '\0') goto LAB_142019b0a;
LAB_142019ad5:
    func_0x000142923c10(*(longlong *)(lVar20 + 8) * (ulonglong)param_10 +
                        *(longlong *)(lVar20 + 0x10),&uStack_b8);
    *(undefined4 *)(*(longlong *)(lVar20 + 0x20) + (ulonglong)param_10 * 4) = param_11;
LAB_142019b61:
    *(undefined4 *)(*(longlong *)(lVar20 + 0x28) + (ulonglong)param_10 * 4) = param_11;
  }
  uStack_b0 = CONCAT44(uStack_b0._4_4_,uVar3);
  uVar19 = puStack_88[7];
  uStack_b8 = uVar4;
  if ((uVar19 < uStack_80) &&
     (uVar5 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar19 * 8), uVar5 != 0)) {
    lVar20 = ~uVar5 * 0x30 + *(longlong *)(param_3 + 0x18);
    if (param_5[7] == '\0') goto LAB_142019bae;
LAB_142019be3:
    if (param_13 == '\0') {
      uStack_71 = 0;
      FUN_140cff400(lVar20,param_10,&uStack_b8);
      goto LAB_142019c3a;
    }
    if (((uVar19 < uStack_80) &&
        (uVar19 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar19 * 8), uVar19 != 0)) &&
       (pcVar6 = *(code **)(*(longlong *)(param_3 + 0x18) + 0x18 + ~uVar19 * 0x30),
       pcVar6 != (code *)0x0)) {
      uStack_71 = 0;
      (*pcVar6)(&uStack_b8);
    }
  }
  else {
    lVar20 = 0;
    if (param_5[7] != '\0') goto LAB_142019be3;
LAB_142019bae:
    func_0x000142923c10(*(longlong *)(lVar20 + 8) * (ulonglong)param_10 +
                        *(longlong *)(lVar20 + 0x10),&uStack_b8);
    *(undefined4 *)(*(longlong *)(lVar20 + 0x20) + (ulonglong)param_10 * 4) = param_11;
LAB_142019c3a:
    *(undefined4 *)(*(longlong *)(lVar20 + 0x28) + (ulonglong)param_10 * 4) = param_11;
  }
  uStack_b8 = CONCAT44(uStack_b8._4_4_,uVar1);
  uVar19 = puStack_88[8];
  if (uVar19 < uStack_80) {
    uVar5 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar19 * 8);
    if (uVar5 == 0) {
      cVar2 = param_5[8];
      goto joined_r0x000142019cc3;
    }
    lVar20 = ~uVar5 * 0x30 + *(longlong *)(param_3 + 0x18);
    if (param_5[8] == '\0') goto LAB_142019c89;
LAB_142019cc5:
    if (param_13 != '\0') {
      if (((uVar19 < uStack_80) &&
          (uVar19 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar19 * 8), uVar19 != 0)) &&
         (pcVar6 = *(code **)(*(longlong *)(param_3 + 0x18) + 0x18 + ~uVar19 * 0x30),
         pcVar6 != (code *)0x0)) {
        uStack_71 = 0;
        (*pcVar6)(&uStack_b8);
      }
      goto joined_r0x000142019d30;
    }
    uStack_71 = 0;
    FUN_140cff400(lVar20,param_10,&uStack_b8);
  }
  else {
    cVar2 = param_5[8];
joined_r0x000142019cc3:
    lVar20 = 0;
    if (cVar2 != '\0') goto LAB_142019cc5;
LAB_142019c89:
    func_0x000142923c10(*(longlong *)(lVar20 + 8) * (ulonglong)param_10 +
                        *(longlong *)(lVar20 + 0x10),&uStack_b8);
    *(undefined4 *)(*(longlong *)(lVar20 + 0x20) + (ulonglong)param_10 * 4) = param_11;
  }
  *(undefined4 *)(*(longlong *)(lVar20 + 0x28) + (ulonglong)param_10 * 4) = param_11;
joined_r0x000142019d30:
  for (; param_7 != param_8; param_7 = param_7 + 0x10) {
    func_0x000140d40890(param_7,param_3,param_4,param_11,param_10,param_9);
  }
  return;
}

