
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140911520(longlong param_1,undefined8 *param_2,char param_3)

{
  float fVar1;
  float fVar2;
  undefined8 uVar3;
  code *pcVar4;
  ulonglong uVar5;
  undefined8 *puVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  undefined4 uVar13;
  float fStack_58;
  float fStack_54;
  float fStack_50;
  float fStack_4c;
  
  *(undefined1 *)(param_1 + 0x4d0) = 0;
  fVar1 = *(float *)(param_2 + 2);
  fVar11 = (float)*param_2;
  fVar12 = (float)((ulonglong)*param_2 >> 0x20);
  FUN_140d4b440(&fStack_58,2,CONCAT44(fVar12,fVar11 * _UNK_142aab4b0),fVar12 * _UNK_142aab4b0,0);
  fVar2 = *(float *)((longlong)param_2 + 0x14);
  fVar9 = fVar2 * 0.0;
  fVar8 = fStack_4c * fStack_4c;
  fVar10 = fStack_50 * fStack_50 + fStack_54 * fStack_54 + fStack_58 * fStack_58;
  fVar7 = fStack_50 * fVar2 + fStack_54 * fVar9 + fStack_58 * fVar9;
  fVar7 = fVar7 + fVar7;
  fStack_4c = fStack_4c + fStack_4c;
  *(ulonglong *)(param_1 + 0x144) =
       CONCAT44(((float)((ulonglong)param_2[1] >> 0x20) +
                fStack_4c * (fStack_50 * fVar9 - fVar2 * fStack_58) +
                fVar7 * fStack_54 + (fVar8 - fVar10) * fVar9) -
                (float)((ulonglong)*(undefined8 *)(param_1 + 0x138) >> 0x20),
                ((float)param_2[1] +
                fStack_4c * (fStack_54 * fVar2 - fVar9 * fStack_50) +
                fVar7 * fStack_58 + (fVar8 - fVar10) * fVar9) -
                (float)*(undefined8 *)(param_1 + 0x138));
  *(float *)(param_1 + 0x14c) =
       (fStack_4c * (fStack_58 * fVar9 - fVar9 * fStack_54) +
        fVar7 * fStack_50 + (fVar8 - fVar10) * fVar2 + fVar1) - *(float *)(param_1 + 0x140);
  *(undefined4 *)(param_1 + 0x118) = 0;
  puVar6 = (undefined8 *)func_0x000140613c10(0x1c,4);
  if (puVar6 != (undefined8 *)0x0) {
    *puVar6 = 0;
    uVar3 = *(undefined8 *)(param_1 + 0x150);
    uVar13 = (undefined4)_UNK_1429cd330;
    uVar5 = (ulonglong)_UNK_1429cd330 >> 0x20;
    *(float *)(puVar6 + 1) = fVar12 - (float)uVar3;
    *(float *)((longlong)puVar6 + 0xc) = fVar11 - (float)((ulonglong)uVar3 >> 0x20);
    *(undefined4 *)(puVar6 + 2) = uVar13;
    *(int *)((longlong)puVar6 + 0x14) = (int)uVar5;
    *(undefined4 *)(puVar6 + 3) = 0xbf800000;
    if (*(longlong *)(param_1 + 0x100) != 0) {
      func_0x000140613c20(*(undefined8 *)(param_1 + 0x108),*(longlong *)(param_1 + 0x100) * 0x1c,4);
    }
    *(undefined8 *)(param_1 + 0x100) = 1;
    *(undefined8 **)(param_1 + 0x108) = puVar6;
    *(undefined8 *)(param_1 + 0x110) = 1;
    if (param_3 == '\0') {
      *(undefined4 *)(param_1 + 0x194) = *(undefined4 *)(param_2 + 3);
    }
    return;
  }
  FUN_1428d9000(4,0x1c);
  pcVar4 = (code *)swi(3);
  (*pcVar4)();
  return;
}

