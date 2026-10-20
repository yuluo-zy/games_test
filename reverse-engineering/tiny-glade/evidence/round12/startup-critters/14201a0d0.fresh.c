
void FUN_14201a0d0(ulonglong *param_1,undefined8 param_2,longlong param_3,undefined8 param_4,
                  char *param_5,undefined8 param_6,longlong param_7,longlong param_8,
                  undefined8 param_9,uint param_10,undefined4 param_11,undefined8 *param_12,
                  char param_13)

{
  undefined4 uVar1;
  undefined4 uVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  code *pcVar5;
  undefined8 uVar6;
  ulonglong uVar7;
  longlong lVar8;
  undefined8 uStack_b8;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  ulonglong uStack_80;
  
  uStack_b8._0_4_ = *(undefined4 *)(param_12 + 2);
  uStack_b8._4_4_ = *(undefined4 *)((longlong)param_12 + 0x14);
  uStack_b0 = *(undefined4 *)(param_12 + 3);
  uStack_ac = *(undefined4 *)((longlong)param_12 + 0x1c);
  uStack_a8 = *(undefined4 *)(param_12 + 4);
  uStack_a4 = *(undefined4 *)((longlong)param_12 + 0x24);
  uStack_a0 = *(undefined4 *)(param_12 + 5);
  uStack_9c = *(undefined4 *)((longlong)param_12 + 0x2c);
  uStack_98 = *(undefined4 *)(param_12 + 6);
  uStack_94 = *(undefined4 *)((longlong)param_12 + 0x34);
  uStack_90 = *(undefined4 *)(param_12 + 7);
  uStack_8c = *(undefined4 *)((longlong)param_12 + 0x3c);
  uVar7 = *param_1;
  uVar3 = *(ulonglong *)(param_3 + 0x40);
  if ((uVar7 < uVar3) &&
     (uVar4 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar7 * 8), uVar4 != 0)) {
    lVar8 = ~uVar4 * 0x30 + *(longlong *)(param_3 + 0x18);
  }
  else {
    lVar8 = 0;
  }
  uVar1 = *(undefined4 *)(param_12 + 0xb);
  uVar6 = param_12[10];
  uVar2 = *(undefined4 *)(param_12 + 0xd);
  if (*param_5 == '\0') {
    func_0x000142923c10(*(longlong *)(lVar8 + 8) * (ulonglong)param_10 + *(longlong *)(lVar8 + 0x10)
                        ,&uStack_b8);
    *(undefined4 *)(*(longlong *)(lVar8 + 0x20) + (ulonglong)param_10 * 4) = param_11;
LAB_14201a213:
    *(undefined4 *)(*(longlong *)(lVar8 + 0x28) + (ulonglong)param_10 * 4) = param_11;
  }
  else {
    if (param_13 == '\0') {
      FUN_140cff400(lVar8,param_10,&uStack_b8);
      goto LAB_14201a213;
    }
    if (((uVar7 < uVar3) &&
        (uVar7 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar7 * 8), uVar7 != 0)) &&
       (pcVar5 = *(code **)(*(longlong *)(param_3 + 0x18) + 0x18 + ~uVar7 * 0x30),
       pcVar5 != (code *)0x0)) {
      (*pcVar5)(&uStack_b8);
    }
  }
  uStack_b8 = CONCAT44(uStack_b8._4_4_,uVar1);
  uVar7 = param_1[1];
  if ((uVar7 < uVar3) &&
     (uVar4 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar7 * 8), uVar4 != 0)) {
    lVar8 = ~uVar4 * 0x30 + *(longlong *)(param_3 + 0x18);
    if (param_5[1] != '\0') goto LAB_14201a296;
LAB_14201a264:
    func_0x000142923c10(*(longlong *)(lVar8 + 8) * (ulonglong)param_10 + *(longlong *)(lVar8 + 0x10)
                        ,&uStack_b8);
    *(undefined4 *)(*(longlong *)(lVar8 + 0x20) + (ulonglong)param_10 * 4) = param_11;
LAB_14201a2ec:
    *(undefined4 *)(*(longlong *)(lVar8 + 0x28) + (ulonglong)param_10 * 4) = param_11;
  }
  else {
    lVar8 = 0;
    if (param_5[1] == '\0') goto LAB_14201a264;
LAB_14201a296:
    if (param_13 == '\0') {
      FUN_140cff400(lVar8,param_10,&uStack_b8);
      goto LAB_14201a2ec;
    }
    if (((uVar7 < uVar3) &&
        (uVar7 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar7 * 8), uVar7 != 0)) &&
       (pcVar5 = *(code **)(*(longlong *)(param_3 + 0x18) + 0x18 + ~uVar7 * 0x30),
       pcVar5 != (code *)0x0)) {
      (*pcVar5)(&uStack_b8);
    }
  }
  uVar7 = param_1[2];
  if ((uVar7 < uVar3) &&
     (uVar4 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar7 * 8), uVar4 != 0)) {
    lVar8 = ~uVar4 * 0x30 + *(longlong *)(param_3 + 0x18);
    if (param_5[2] != '\0') goto LAB_14201a368;
LAB_14201a336:
    func_0x000142923c10(*(longlong *)(lVar8 + 8) * (ulonglong)param_10 + *(longlong *)(lVar8 + 0x10)
                        ,&uStack_b8);
    *(undefined4 *)(*(longlong *)(lVar8 + 0x20) + (ulonglong)param_10 * 4) = param_11;
LAB_14201a3be:
    *(undefined4 *)(*(longlong *)(lVar8 + 0x28) + (ulonglong)param_10 * 4) = param_11;
  }
  else {
    lVar8 = 0;
    if (param_5[2] == '\0') goto LAB_14201a336;
LAB_14201a368:
    if (param_13 == '\0') {
      FUN_140cff400(lVar8,param_10,&uStack_b8);
      goto LAB_14201a3be;
    }
    if (((uVar7 < uVar3) &&
        (uVar7 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar7 * 8), uVar7 != 0)) &&
       (pcVar5 = *(code **)(*(longlong *)(param_3 + 0x18) + 0x18 + ~uVar7 * 0x30),
       pcVar5 != (code *)0x0)) {
      (*pcVar5)(&uStack_b8);
    }
  }
  uVar7 = param_1[3];
  uStack_b8 = uVar6;
  if ((uVar7 < uVar3) &&
     (uVar4 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar7 * 8), uVar4 != 0)) {
    lVar8 = ~uVar4 * 0x30 + *(longlong *)(param_3 + 0x18);
    if (param_5[3] != '\0') goto LAB_14201a43f;
LAB_14201a412:
    func_0x000142923c10(*(longlong *)(lVar8 + 8) * (ulonglong)param_10 + *(longlong *)(lVar8 + 0x10)
                        ,&uStack_b8);
    *(undefined4 *)(*(longlong *)(lVar8 + 0x20) + (ulonglong)param_10 * 4) = param_11;
LAB_14201a490:
    *(undefined4 *)(*(longlong *)(lVar8 + 0x28) + (ulonglong)param_10 * 4) = param_11;
  }
  else {
    lVar8 = 0;
    if (param_5[3] == '\0') goto LAB_14201a412;
LAB_14201a43f:
    if (param_13 == '\0') {
      FUN_140cff400(lVar8,param_10,&uStack_b8);
      goto LAB_14201a490;
    }
    if (((uVar7 < uVar3) &&
        (uVar7 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar7 * 8), uVar7 != 0)) &&
       (pcVar5 = *(code **)(*(longlong *)(param_3 + 0x18) + 0x18 + ~uVar7 * 0x30),
       pcVar5 != (code *)0x0)) {
      (*pcVar5)(&uStack_b8);
    }
  }
  uStack_b8 = *param_12;
  uStack_b0 = *(undefined4 *)(param_12 + 1);
  uStack_ac = *(undefined4 *)((longlong)param_12 + 0xc);
  uVar7 = param_1[4];
  uStack_80 = uVar3;
  if ((uVar7 < uVar3) &&
     (uVar4 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar7 * 8), uVar4 != 0)) {
    lVar8 = ~uVar4 * 0x30 + *(longlong *)(param_3 + 0x18);
    if (param_5[4] != '\0') goto LAB_14201a51c;
LAB_14201a4eb:
    func_0x000142923c10(*(longlong *)(lVar8 + 8) * (ulonglong)param_10 + *(longlong *)(lVar8 + 0x10)
                        ,&uStack_b8);
    *(undefined4 *)(*(longlong *)(lVar8 + 0x20) + (ulonglong)param_10 * 4) = param_11;
LAB_14201a56f:
    *(undefined4 *)(*(longlong *)(lVar8 + 0x28) + (ulonglong)param_10 * 4) = param_11;
  }
  else {
    lVar8 = 0;
    if (param_5[4] == '\0') goto LAB_14201a4eb;
LAB_14201a51c:
    if (param_13 == '\0') {
      FUN_140cff400(lVar8,param_10,&uStack_b8);
      goto LAB_14201a56f;
    }
    if (((uVar7 < uVar3) &&
        (uVar7 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar7 * 8), uVar7 != 0)) &&
       (pcVar5 = *(code **)(*(longlong *)(param_3 + 0x18) + 0x18 + ~uVar7 * 0x30),
       pcVar5 != (code *)0x0)) {
      (*pcVar5)(&uStack_b8);
    }
  }
  uVar3 = uStack_80;
  uStack_b8 = param_12[8];
  uStack_b0 = *(undefined4 *)(param_12 + 9);
  uStack_ac = *(undefined4 *)((longlong)param_12 + 0x4c);
  uVar7 = param_1[5];
  if ((uVar7 < uStack_80) &&
     (uVar4 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar7 * 8), uVar4 != 0)) {
    lVar8 = ~uVar4 * 0x30 + *(longlong *)(param_3 + 0x18);
    if (param_5[5] != '\0') goto LAB_14201a5f5;
LAB_14201a5c4:
    func_0x000142923c10(*(longlong *)(lVar8 + 8) * (ulonglong)param_10 + *(longlong *)(lVar8 + 0x10)
                        ,&uStack_b8);
    *(undefined4 *)(*(longlong *)(lVar8 + 0x20) + (ulonglong)param_10 * 4) = param_11;
  }
  else {
    lVar8 = 0;
    if (param_5[5] == '\0') goto LAB_14201a5c4;
LAB_14201a5f5:
    if (param_13 != '\0') {
      if (((uVar7 < uStack_80) &&
          (uVar7 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar7 * 8), uVar7 != 0)) &&
         (pcVar5 = *(code **)(*(longlong *)(param_3 + 0x18) + 0x18 + ~uVar7 * 0x30),
         pcVar5 != (code *)0x0)) {
        (*pcVar5)(&uStack_b8);
      }
      goto LAB_14201a64e;
    }
    FUN_140cff400(lVar8,param_10,&uStack_b8);
  }
  *(undefined4 *)(*(longlong *)(lVar8 + 0x28) + (ulonglong)param_10 * 4) = param_11;
LAB_14201a64e:
  uStack_b0 = *(undefined4 *)((longlong)param_12 + 100);
  uStack_b8 = *(undefined8 *)((longlong)param_12 + 0x5c);
  uVar7 = param_1[6];
  if ((uVar7 < uVar3) &&
     (uVar4 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar7 * 8), uVar4 != 0)) {
    lVar8 = ~uVar4 * 0x30 + *(longlong *)(param_3 + 0x18);
  }
  else {
    lVar8 = 0;
  }
  if (param_5[6] == '\0') {
    uVar7 = (ulonglong)param_10;
    func_0x000142923c10(*(longlong *)(lVar8 + 8) * uVar7 + *(longlong *)(lVar8 + 0x10),&uStack_b8);
    *(undefined4 *)(*(longlong *)(lVar8 + 0x20) + uVar7 * 4) = param_11;
    *(undefined4 *)(*(longlong *)(lVar8 + 0x28) + uVar7 * 4) = param_11;
  }
  else if (param_13 == '\0') {
    FUN_140cff400(lVar8,(ulonglong)param_10,&uStack_b8);
    *(undefined4 *)(*(longlong *)(lVar8 + 0x28) + (ulonglong)param_10 * 4) = param_11;
  }
  else if (((uVar7 < uVar3) &&
           (uVar7 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar7 * 8), uVar7 != 0)) &&
          (pcVar5 = *(code **)(*(longlong *)(param_3 + 0x18) + 0x18 + ~uVar7 * 0x30),
          pcVar5 != (code *)0x0)) {
    (*pcVar5)(&uStack_b8);
  }
  uStack_b8 = CONCAT44(uStack_b8._4_4_,uVar2);
  uVar7 = param_1[7];
  if ((uVar7 < uVar3) &&
     (uVar4 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar7 * 8), uVar4 != 0)) {
    lVar8 = ~uVar4 * 0x30 + *(longlong *)(param_3 + 0x18);
  }
  else {
    lVar8 = 0;
  }
  if (param_5[7] == '\0') {
    uVar7 = (ulonglong)param_10;
    func_0x000142923c10(*(longlong *)(lVar8 + 8) * uVar7 + *(longlong *)(lVar8 + 0x10),&uStack_b8);
    *(undefined4 *)(*(longlong *)(lVar8 + 0x20) + uVar7 * 4) = param_11;
    *(undefined4 *)(*(longlong *)(lVar8 + 0x28) + uVar7 * 4) = param_11;
  }
  else if (param_13 == '\0') {
    FUN_140cff400(lVar8,(ulonglong)param_10,&uStack_b8);
    *(undefined4 *)(*(longlong *)(lVar8 + 0x28) + (ulonglong)param_10 * 4) = param_11;
  }
  else if (((uVar7 < uVar3) &&
           (uVar7 = *(ulonglong *)(*(longlong *)(param_3 + 0x38) + uVar7 * 8), uVar7 != 0)) &&
          (pcVar5 = *(code **)(*(longlong *)(param_3 + 0x18) + 0x18 + ~uVar7 * 0x30),
          pcVar5 != (code *)0x0)) {
    (*pcVar5)(&uStack_b8);
  }
  for (; param_7 != param_8; param_7 = param_7 + 0x10) {
    func_0x000140d40890(param_7,param_3,param_4,param_11,param_10,param_9);
  }
  return;
}

