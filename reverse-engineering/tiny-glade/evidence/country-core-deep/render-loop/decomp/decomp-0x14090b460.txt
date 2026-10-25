
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_14090b460(undefined8 param_1,longlong param_2,undefined4 *param_3,longlong param_4)

{
  float fVar1;
  char cVar2;
  undefined8 uVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  undefined4 auStack_3a8 [62];
  undefined1 auStack_2b0 [184];
  undefined1 auStack_1f8 [38];
  char cStack_1d2;
  longlong lStack_1c8;
  float fStack_1b8;
  float fStack_1b4;
  float fStack_1b0;
  float fStack_1ac;
  float fStack_1a8;
  float fStack_1a4;
  float fStack_1a0;
  float fStack_19c;
  float fStack_198;
  float fStack_194;
  float fStack_190;
  float fStack_18c;
  undefined4 uStack_188;
  undefined4 uStack_184;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  undefined1 auStack_100 [38];
  char cStack_da;
  longlong lStack_d0;
  undefined1 uStack_b1;
  undefined8 uStack_b0;
  
  uStack_b0 = 0xfffffffffffffffe;
  uVar3 = *(undefined8 *)(*(longlong *)(param_2 + 0xe0) + 0x1f8);
  cVar2 = *(char *)(param_3 + 1);
  *(char *)(param_2 + 0xf0) = cVar2;
  if (cVar2 == '\0') {
    func_0x000142923c00(&fStack_1b8,param_2,0xf8);
    if (cStack_da != '\b') {
      FUN_14096b850(auStack_100,lStack_d0 + 0xe8,0x418a98441f59a54c,1);
    }
    func_0x000142923c00(auStack_2b0,&fStack_1b8,0xf8);
    uStack_b1 = 0x11;
    auStack_3a8[0] = 2;
    FUN_140908c50(&uStack_b1,auStack_2b0,0x11651f4dc9841aa4,0x1d,auStack_3a8);
  }
  else {
    func_0x000142923c00(&fStack_1b8,param_2,0xf8);
    if (cStack_da != '\b') {
      FUN_14096b850(auStack_100,lStack_d0 + 0xe8,0x418a98441f59a54c,0);
    }
    func_0x000142923c00(auStack_2b0,&fStack_1b8,0xf8);
    uStack_b1 = 0x12;
    auStack_3a8[0] = 2;
    FUN_140908c50(&uStack_b1,auStack_2b0,0x11651f4dc9841aa4,0x1d,auStack_3a8);
  }
  func_0x000142923c00(auStack_3a8,auStack_2b0,0xf8);
  fVar4 = *(float *)(param_4 + 0x20);
  fVar5 = *(float *)(param_4 + 0x24);
  fVar6 = *(float *)(param_4 + 0x2c);
  fVar1 = *(float *)(param_4 + 0x28);
  fVar10 = fVar1 + fVar1;
  fVar9 = fVar5 + fVar5;
  fVar7 = fVar4 * (fVar4 + fVar4);
  fVar8 = (fVar4 + fVar4) * fVar6;
  fStack_1ac = *(float *)(param_4 + 0x3c);
  fStack_19c = *(float *)(param_4 + 0x40);
  fStack_18c = *(float *)(param_4 + 0x44);
  fStack_1b8 = fStack_1ac * (_UNK_142925904 - (fVar5 * fVar9 + fVar1 * fVar10));
  fStack_1b4 = fStack_1ac * (fVar4 * fVar9 + fVar10 * fVar6);
  fStack_1b0 = fStack_1ac * (fVar4 * fVar10 - fVar9 * fVar6);
  fStack_1ac = fStack_1ac * 0.0;
  fStack_1a8 = fStack_19c * (fVar4 * fVar9 - fVar10 * fVar6);
  fStack_1a4 = fStack_19c * (_UNK_142925904 - (fVar1 * fVar10 + fVar7));
  fStack_1a0 = fStack_19c * (fVar5 * fVar10 + fVar8);
  fStack_19c = fStack_19c * 0.0;
  uStack_180 = *(undefined4 *)(param_4 + 0x38);
  fStack_198 = fStack_18c * (fVar9 * fVar6 + fVar4 * fVar10);
  fStack_194 = fStack_18c * (fVar5 * fVar10 - fVar8);
  fStack_190 = fStack_18c * (_UNK_142925904 - (fVar7 + fVar5 * fVar9));
  fStack_18c = fStack_18c * 0.0;
  uStack_188 = *(undefined4 *)(param_4 + 0x30);
  uStack_184 = *(undefined4 *)(param_4 + 0x34);
  uStack_17c = (undefined4)((ulonglong)_UNK_14293e0c8 >> 0x20);
  FUN_141629b90(auStack_2b0,auStack_3a8,0x14658de104bef99d,&fStack_1b8);
  if (cStack_1d2 != '\b') {
    FUN_14096b570(auStack_1f8,lStack_1c8 + 0xe8,0xc5ab673fb08651f5,cVar2);
  }
  func_0x000142923c00(&fStack_1b8,auStack_2b0,0xf8);
  FUN_141629a20(auStack_2b0,&fStack_1b8,0x1e5485ef2e9f9384,*param_3);
  FUN_141629d20(&fStack_1b8,auStack_2b0,0x531bb9318eea4971,uVar3);
  FUN_1416286f0(param_1,&fStack_1b8);
  return param_1;
}

