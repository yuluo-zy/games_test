utils::geometry::rectangle::Rectangle3d::contains_point

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 tg_math_140c985a0(undefined8 *param_1,float *param_2)

{
  undefined8 uVar1;
  undefined1 auVar3 [16];
  int iVar4;
  undefined4 uVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  undefined8 uVar2;
  
  uVar5 = 0;
  if ((*(float *)(param_1 + 3) <= param_2[1]) && (param_2[1] < *(float *)((longlong)param_1 + 0x1c))
     ) {
    fVar6 = (float)((ulonglong)*param_1 >> 0x20);
    fVar10 = (float)*param_1;
    fVar9 = (float)((ulonglong)param_1[1] >> 0x20);
    fVar7 = (float)param_1[1];
    fVar8 = (param_2[2] - fVar9) * fVar6 + (*param_2 - fVar7) * fVar10;
    fVar10 = (param_2[2] - fVar9) * fVar10 +
             (*param_2 - fVar7) * (float)((uint)fVar6 ^ _DAT_1429258f0);
    uVar1 = param_1[2];
    uVar2 = param_1[2];
    auVar3._4_4_ = -(uint)((float)(_UNK_1429258f4 ^ (uint)fVar10) <=
                          ((float)((ulonglong)uVar1 >> 0x20) + 0.0) * _UNK_14293ac64);
    auVar3._0_4_ = -(uint)((float)(_DAT_1429258f0 ^ (uint)fVar8) <=
                          ((float)uVar1 + 0.0) * _DAT_14293ac60);
    auVar3._8_4_ = -(uint)(fVar8 <= ((float)uVar2 + 0.0) * _UNK_14293ac68);
    auVar3._12_4_ = -(uint)(fVar10 <= ((float)((ulonglong)uVar2 >> 0x20) + 0.0) * _UNK_14293ac6c);
    iVar4 = movmskps(0,auVar3);
    uVar5 = CONCAT31((int3)((uint)iVar4 >> 8),iVar4 == 0xf);
  }
  return uVar5;
}

