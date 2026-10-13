utils::geometry::rectangle::Rectangle2d::contains_point_padding

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4
FUN_140c9a6a0(undefined8 *param_1,float param_2,float param_3,float param_4,float param_5)

{
  undefined8 uVar1;
  undefined1 auVar3 [16];
  undefined4 in_EAX;
  int iVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  undefined8 uVar2;
  
  fVar7 = (float)((ulonglong)*param_1 >> 0x20);
  fVar6 = (float)*param_1;
  fVar9 = (float)((ulonglong)param_1[1] >> 0x20);
  fVar8 = (float)param_1[1];
  uVar1 = param_1[2];
  uVar2 = param_1[2];
  fVar5 = (param_3 - fVar9) * fVar7 + (param_2 - fVar8) * fVar6;
  fVar6 = (param_3 - fVar9) * fVar6 + (param_2 - fVar8) * (float)((uint)fVar7 ^ _DAT_1429258f0);
  auVar3._4_4_ = -(uint)((float)(_UNK_1429258f4 ^ (uint)fVar6) <=
                        ((float)((ulonglong)uVar1 >> 0x20) + param_5) * _UNK_14293ac64);
  auVar3._0_4_ = -(uint)((float)(_DAT_1429258f0 ^ (uint)fVar5) <=
                        ((float)uVar1 + param_4) * _DAT_14293ac60);
  auVar3._8_4_ = -(uint)(fVar5 <= ((float)uVar2 + param_4) * _UNK_14293ac68);
  auVar3._12_4_ = -(uint)(fVar6 <= ((float)((ulonglong)uVar2 >> 0x20) + param_5) * _UNK_14293ac6c);
  iVar4 = movmskps(in_EAX,auVar3);
  return CONCAT31((int3)((uint)iVar4 >> 8),iVar4 == 0xf);
}

