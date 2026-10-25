
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141207a10(longlong *param_1,float param_2)

{
  undefined8 *puVar1;
  code *pcVar2;
  float fVar3;
  undefined4 uVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  undefined1 uStack_109;
  uint auStack_108 [10];
  undefined8 auStack_e0 [5];
  longlong *plStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined4 uStack_98;
  undefined4 uStack_94;
  ulonglong uStack_90;
  ulonglong uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined1 *puStack_68;
  undefined1 uStack_60;
  
  if (param_2 < 0.0) {
    FUN_1428d9430(&UNK_142c1a090,0x21,&UNK_142c1a0e8);
    pcVar2 = (code *)swi(3);
    (*pcVar2)();
    return;
  }
  fVar6 = *(float *)((longlong)param_1 + 0x1c);
  uStack_90 = (ulonglong)*(uint *)(param_1 + 3);
  puVar1 = (undefined8 *)*param_1;
  uStack_b0 = *puVar1;
  uStack_a8 = puVar1[1];
  uStack_a0 = *(undefined4 *)(puVar1 + 2);
  uStack_9c = *(undefined4 *)((longlong)puVar1 + 0x14);
  uStack_98 = *(undefined4 *)(puVar1 + 3);
  uStack_94 = *(undefined4 *)((longlong)puVar1 + 0x1c);
  uStack_80 = 0;
  uStack_78 = 0;
  uStack_70 = 0;
  plStack_b8 = param_1 + 1;
  auStack_e0[0] = 0;
  puStack_68 = &uStack_109;
  uStack_60 = 0;
  uStack_88 = uStack_90;
  FUN_141238280(auStack_108,auStack_e0);
  fVar3 = _DAT_142925904;
  if (auStack_108[0] == 1) {
    uVar4 = (undefined4)auStack_108._8_8_;
    fVar5 = (float)auStack_108._32_8_ - (float)auStack_108._16_8_;
    fVar7 = SUB84(auStack_108._32_8_,4) - SUB84(auStack_108._16_8_,4);
    fVar5 = SQRT(fVar7 * fVar7 + fVar5 * fVar5);
    param_2 = (_DAT_142925904 - fVar6) * fVar5 - param_2;
    if (0.0 <= param_2) {
LAB_141207bac:
      *(undefined4 *)(param_1 + 3) = uVar4;
      *(float *)((longlong)param_1 + 0x1c) = fVar3 - param_2 / fVar5;
    }
    else {
      *(undefined4 *)(param_1 + 3) = uVar4;
      *(undefined4 *)((longlong)param_1 + 0x1c) = 0x3f800000;
      FUN_141238280(auStack_108,auStack_e0);
      if (auStack_108[0] == 1) {
        do {
          uVar4 = (undefined4)auStack_108._8_8_;
          fVar6 = (float)auStack_108._32_8_ - (float)auStack_108._16_8_;
          fVar5 = SUB84(auStack_108._32_8_,4) - SUB84(auStack_108._16_8_,4);
          fVar5 = SQRT(fVar5 * fVar5 + fVar6 * fVar6);
          param_2 = param_2 + fVar5;
          if (0.0 <= param_2) goto LAB_141207bac;
          *(undefined4 *)(param_1 + 3) = uVar4;
          *(undefined4 *)((longlong)param_1 + 0x1c) = 0x3f800000;
          FUN_141238280(auStack_108,auStack_e0);
        } while ((auStack_108[0] & 1) != 0);
      }
    }
  }
  return;
}

