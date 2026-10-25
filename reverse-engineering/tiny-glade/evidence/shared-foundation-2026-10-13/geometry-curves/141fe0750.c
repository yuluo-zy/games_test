
void FUN_141fe0750(float *param_1,longlong param_2,float param_3,longlong *param_4)

{
  undefined4 uVar1;
  float fVar2;
  longlong lVar3;
  longlong lVar4;
  longlong *plVar5;
  longlong lVar6;
  longlong lVar7;
  code *pcVar8;
  longlong lVar9;
  longlong lVar10;
  ulonglong uVar11;
  float fVar12;
  
  if (param_2 != 0) {
    fVar12 = *param_1;
    lVar4 = *param_4;
    plVar5 = (longlong *)param_4[1];
    lVar6 = param_4[2];
    lVar3 = param_4[3];
    if (fVar12 <= param_3) {
joined_r0x000141fe0875:
      if (param_2 != 1) {
        lVar10 = param_2 * 4 + -4;
        uVar11 = 1;
        lVar9 = lVar6 * 0x20;
        do {
          fVar2 = param_1[uVar11];
          if (fVar12 <= param_3) {
            if (param_3 < fVar2) {
              FUN_14123fb70(param_4,uVar11 - 1,(param_3 - fVar12) / (fVar2 - fVar12));
              goto LAB_141fe0903;
            }
          }
          else if (fVar2 <= param_3) {
            FUN_14123fab0(param_4,uVar11 - 1,(param_3 - fVar12) / (fVar2 - fVar12));
          }
          else {
LAB_141fe0903:
            if (*(ulonglong *)(lVar4 + 0x28) <= uVar11) goto LAB_141fe0988;
            uVar1 = *(undefined4 *)(*(longlong *)(lVar4 + 0x20) + uVar11 * 4);
            lVar7 = plVar5[2];
            if (lVar7 == lVar6) {
              if (lVar6 == *plVar5) {
                FUN_140562c30(plVar5);
              }
              lVar7 = plVar5[1];
              *(undefined8 *)(lVar7 + lVar9) = 0;
              ((undefined8 *)(lVar7 + lVar9))[1] = 0;
              *(longlong *)(lVar7 + 0x10 + lVar9) = lVar3;
              *(undefined4 *)(lVar7 + 0x18 + lVar9) = uVar1;
              *(undefined4 *)(lVar7 + 0x1c + lVar9) = 0x3f800000;
              plVar5[2] = lVar6 + 1;
            }
            else {
              if (lVar7 == 0) goto LAB_141fe0994;
              if (plVar5[1] + lVar7 * 0x20 == 0x20) goto LAB_141fe0994;
              *(undefined4 *)(plVar5[1] + lVar7 * 0x20 + -4) = uVar1;
            }
          }
          uVar11 = uVar11 + 1;
          lVar10 = lVar10 + -4;
          fVar12 = fVar2;
        } while (lVar10 != 0);
      }
      return;
    }
    if (*(longlong *)(lVar4 + 0x28) == 0) {
LAB_141fe0988:
      FUN_1428d9310(&UNK_142c1f430);
    }
    else {
      uVar1 = **(undefined4 **)(lVar4 + 0x20);
      lVar10 = plVar5[2];
      if (lVar10 == lVar6) {
        if (lVar6 == *plVar5) {
          FUN_140562c30(plVar5,&UNK_142c1f460);
        }
        lVar10 = plVar5[1];
        lVar9 = lVar6 * 0x20;
        *(undefined8 *)(lVar10 + lVar9) = 0;
        ((undefined8 *)(lVar10 + lVar9))[1] = 0;
        *(longlong *)(lVar10 + 0x10 + lVar9) = lVar3;
        *(undefined4 *)(lVar10 + 0x18 + lVar9) = uVar1;
        *(undefined4 *)(lVar10 + 0x1c + lVar9) = 0x3f800000;
        plVar5[2] = lVar6 + 1;
        goto joined_r0x000141fe0875;
      }
      if (lVar10 != 0) {
        if (plVar5[1] + lVar10 * 0x20 != 0x20) {
          *(undefined4 *)(plVar5[1] + lVar10 * 0x20 + -4) = uVar1;
          goto joined_r0x000141fe0875;
        }
      }
    }
LAB_141fe0994:
    FUN_1428d9310(&UNK_142c1f448);
  }
  FUN_1428d9518(0,0,&UNK_142fcb2c0);
  pcVar8 = (code *)swi(3);
  (*pcVar8)();
  return;
}

