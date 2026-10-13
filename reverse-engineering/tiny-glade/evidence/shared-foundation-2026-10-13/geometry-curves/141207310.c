
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141207310(longlong *param_1,float param_2,undefined8 param_3,undefined8 param_4)

{
  float fVar1;
  uint uVar2;
  undefined4 *puVar3;
  code *pcVar4;
  int iVar5;
  longlong lVar6;
  float fVar7;
  uint auStack_170 [2];
  longlong lStack_168;
  float fStack_160;
  float fStack_15c;
  float fStack_158;
  float fStack_148;
  float fStack_144;
  float fStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined4 uStack_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined8 auStack_f0 [7];
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
  undefined8 *puStack_70;
  undefined8 uStack_68;
  longlong lStack_60;
  undefined1 uStack_58;
  undefined1 uStack_50;
  
  uStack_138 = param_3;
  uStack_130 = param_4;
  if (param_2 < 0.0) {
    FUN_1428d9430(&UNK_142c1a090,0x21,&UNK_142c1a0b8);
LAB_1412075d3:
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
    uStack_110 = *puVar3;
    uStack_10c = puVar3[1];
    uStack_108 = puVar3[2];
    uStack_104 = puVar3[3];
    uStack_100 = puVar3[4];
    uStack_fc = puVar3[5];
    uStack_f8 = puVar3[6];
    uStack_f4 = puVar3[7];
    lVar6 = func_0x000141200350(puVar3 + 4);
    lStack_78 = (lVar6 - (ulonglong)uVar2) + -2;
    uStack_128 = 0;
    uStack_120 = 0;
    uStack_118 = 0;
    plStack_b8 = param_1 + 1;
    auStack_f0[0] = 0;
    uStack_b0 = 0;
    uStack_a8 = 0;
    uStack_a0 = 0;
    uStack_98 = CONCAT44(uStack_10c,uStack_110);
    uStack_90 = uStack_108;
    uStack_8c = uStack_104;
    uStack_88 = uStack_100;
    uStack_84 = uStack_fc;
    uStack_80 = CONCAT44(uStack_f4,uStack_f8);
    puStack_70 = &uStack_138;
    uStack_68 = 0;
    uStack_58 = 0;
    uStack_50 = 0;
    lStack_60 = (ulonglong)uVar2 + 1;
    FUN_141237c20(auStack_170,auStack_f0);
    if (auStack_170[0] == 1) {
      iVar5 = (int)lStack_168 + -1;
      if (lStack_168 == 0) goto LAB_1412075d3;
      fVar7 = SQRT((fStack_140 - fStack_158) * (fStack_140 - fStack_158) +
                   (fStack_144 - fStack_15c) * (fStack_144 - fStack_15c) +
                   (fStack_148 - fStack_160) * (fStack_148 - fStack_160));
      param_2 = (fVar1 + _DAT_14295a0e4 + _DAT_142925904) * fVar7 - param_2;
      if (0.0 <= param_2) {
LAB_141207590:
        *(int *)(param_1 + 2) = iVar5;
        *(float *)((longlong)param_1 + 0x14) = param_2 / fVar7;
      }
      else {
        *(int *)(param_1 + 2) = iVar5;
        *(undefined4 *)((longlong)param_1 + 0x14) = 0;
        FUN_141237c20(auStack_170,auStack_f0);
        if (auStack_170[0] == 1) {
          do {
            iVar5 = (int)lStack_168 + -1;
            if (lStack_168 == 0) goto LAB_1412075d3;
            fVar7 = SQRT((fStack_140 - fStack_158) * (fStack_140 - fStack_158) +
                         (fStack_144 - fStack_15c) * (fStack_144 - fStack_15c) +
                         (fStack_148 - fStack_160) * (fStack_148 - fStack_160));
            param_2 = param_2 + fVar7;
            if (0.0 <= param_2) goto LAB_141207590;
            *(int *)(param_1 + 2) = iVar5;
            *(undefined4 *)((longlong)param_1 + 0x14) = 0;
            FUN_141237c20(auStack_170,auStack_f0);
          } while ((auStack_170[0] & 1) != 0);
        }
      }
    }
  }
  return;
}

