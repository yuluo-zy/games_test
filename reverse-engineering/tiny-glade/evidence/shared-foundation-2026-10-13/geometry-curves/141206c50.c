
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141206c50(longlong *param_1,float param_2)

{
  float fVar1;
  uint uVar2;
  undefined4 *puVar3;
  code *pcVar4;
  int iVar5;
  longlong lVar6;
  float fVar7;
  undefined1 uStack_141;
  uint auStack_140 [2];
  longlong lStack_138;
  float fStack_130;
  float fStack_12c;
  float fStack_120;
  float fStack_11c;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined8 auStack_e0 [5];
  longlong *plStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined8 uStack_80;
  longlong lStack_78;
  undefined1 *puStack_70;
  undefined8 uStack_68;
  longlong lStack_60;
  undefined1 uStack_58;
  undefined1 uStack_50;
  
  if (param_2 < 0.0) {
    FUN_1428d9430(&UNK_142c1a090,0x21,&UNK_142c1a0b8);
LAB_141206eb7:
    FUN_1428d9310(&UNK_142c1a0d0);
    pcVar4 = (code *)swi(3);
    (*pcVar4)();
    return;
  }
  puVar3 = (undefined4 *)*param_1;
  lVar6 = func_0x000141200350(puVar3 + 4);
  if (lVar6 != 0) {
    fVar1 = *(float *)((longlong)param_1 + 0x14);
    uVar2 = *(uint *)(param_1 + 2);
    uStack_100 = *puVar3;
    uStack_fc = puVar3[1];
    uStack_f8 = puVar3[2];
    uStack_f4 = puVar3[3];
    uStack_f0 = puVar3[4];
    uStack_ec = puVar3[5];
    uStack_e8 = puVar3[6];
    uStack_e4 = puVar3[7];
    lVar6 = func_0x000141200350(puVar3 + 4);
    lStack_78 = (lVar6 - (ulonglong)uVar2) + -2;
    uStack_118 = 0;
    uStack_110 = 0;
    uStack_108 = 0;
    plStack_b8 = param_1 + 1;
    auStack_e0[0] = 0;
    uStack_b0 = 0;
    uStack_a8 = 0;
    uStack_a0 = 0;
    uStack_98 = CONCAT44(uStack_fc,uStack_100);
    uStack_90 = uStack_f8;
    uStack_8c = uStack_f4;
    uStack_88 = uStack_f0;
    uStack_84 = uStack_ec;
    uStack_80 = CONCAT44(uStack_e4,uStack_e8);
    puStack_70 = &uStack_141;
    uStack_68 = 0;
    uStack_58 = 0;
    uStack_50 = 0;
    lStack_60 = (ulonglong)uVar2 + 1;
    FUN_1412380b0(auStack_140,auStack_e0);
    if (auStack_140[0] == 1) {
      iVar5 = (int)lStack_138 + -1;
      if (lStack_138 == 0) goto LAB_141206eb7;
      fVar7 = SQRT((fStack_11c - fStack_12c) * (fStack_11c - fStack_12c) +
                   (fStack_120 - fStack_130) * (fStack_120 - fStack_130));
      param_2 = (fVar1 + _DAT_14295a0e4 + _DAT_142925904) * fVar7 - param_2;
      if (0.0 <= param_2) {
LAB_141206e74:
        *(int *)(param_1 + 2) = iVar5;
        *(float *)((longlong)param_1 + 0x14) = param_2 / fVar7;
      }
      else {
        *(int *)(param_1 + 2) = iVar5;
        *(undefined4 *)((longlong)param_1 + 0x14) = 0;
        FUN_1412380b0(auStack_140,auStack_e0);
        if (auStack_140[0] == 1) {
          do {
            iVar5 = (int)lStack_138 + -1;
            if (lStack_138 == 0) goto LAB_141206eb7;
            fVar7 = SQRT((fStack_11c - fStack_12c) * (fStack_11c - fStack_12c) +
                         (fStack_120 - fStack_130) * (fStack_120 - fStack_130));
            param_2 = param_2 + fVar7;
            if (0.0 <= param_2) goto LAB_141206e74;
            *(int *)(param_1 + 2) = iVar5;
            *(undefined4 *)((longlong)param_1 + 0x14) = 0;
            FUN_1412380b0(auStack_140,auStack_e0);
          } while ((auStack_140[0] & 1) != 0);
        }
      }
    }
  }
  return;
}

