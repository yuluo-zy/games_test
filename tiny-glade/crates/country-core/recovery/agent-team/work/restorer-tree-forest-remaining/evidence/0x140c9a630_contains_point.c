// contains_point 0x140c9a630 size=110
// signature: void __cdecl contains_point(void)


void __cdecl utils::geometry::rectangle::Rectangle2d::contains_point(void)

{
  undefined8 uVar1;
  undefined1 auVar3 [16];
  undefined4 in_EAX;
  undefined8 *in_RCX;
  float fVar4;
  float fVar5;
  float fVar6;
  float in_XMM1_Da;
  float in_XMM2_Da;
  float fVar7;
  float fVar8;
  undefined8 uVar2;
  
  fVar5 = (float)((ulonglong)*in_RCX >> 0x20);
  fVar6 = (float)*in_RCX;
  fVar8 = (float)((ulonglong)in_RCX[1] >> 0x20);
  fVar7 = (float)in_RCX[1];
  uVar1 = in_RCX[2];
  uVar2 = in_RCX[2];
  fVar4 = (in_XMM2_Da - fVar8) * fVar5 + (in_XMM1_Da - fVar7) * fVar6;
  fVar6 = (in_XMM2_Da - fVar8) * fVar6 + (in_XMM1_Da - fVar7) * -fVar5;
  auVar3._4_4_ = -(uint)(-fVar6 <= ((float)((ulonglong)uVar1 >> 0x20) + 0.0) * 0.5);
  auVar3._0_4_ = -(uint)(-fVar4 <= ((float)uVar1 + 0.0) * 0.5);
  auVar3._8_4_ = -(uint)(fVar4 <= ((float)uVar2 + 0.0) * 0.5);
  auVar3._12_4_ = -(uint)(fVar6 <= ((float)((ulonglong)uVar2 >> 0x20) + 0.0) * 0.5);
  movmskps(in_EAX,auVar3);
  return;
}

