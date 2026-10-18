
void __cdecl system_decorator::decorator_visual::flag_connections::clothline(void)

{
  undefined1 auVar1 [16];
  ulonglong uVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  bool bVar7;
  float fVar8;
  undefined1 auVar9 [16];
  undefined1 auVar10 [16];
  float fVar11;
  float fVar12;
  float in_XMM2_Da;
  undefined1 auVar13 [16];
  int iVar14;
  longlong in_stack_00000030;
  longlong in_stack_00000038;
  
  fVar8 = *(float *)(in_stack_00000030 + 0x18) - *(float *)(in_stack_00000038 + 0x18);
  fVar11 = (float)*(undefined8 *)(in_stack_00000030 + 0x10) -
           (float)*(undefined8 *)(in_stack_00000038 + 0x10);
  fVar12 = (float)((ulonglong)*(undefined8 *)(in_stack_00000030 + 0x10) >> 0x20) -
           (float)((ulonglong)*(undefined8 *)(in_stack_00000038 + 0x10) >> 0x20);
  fVar8 = ceilf(SQRT(fVar8 * fVar8 + fVar12 * fVar12 + fVar11 * fVar11) / in_XMM2_Da);
  uVar6 = 0;
  uVar3 = (longlong)(fVar8 - 9.223372e+18) & (longlong)fVar8 >> 0x3f | (longlong)fVar8;
  if (fVar8 < 0.0) {
    uVar3 = uVar6;
  }
  uVar2 = 0xffffffffffffffff;
  if (fVar8 <= 1.8446743e+19) {
    uVar2 = uVar3;
  }
  uVar3 = 2;
  if (2 < uVar2) {
    uVar3 = uVar2;
  }
  auVar1._4_4_ = (float)uVar3;
  auVar1._0_4_ = (float)uVar3;
  auVar13 = (undefined1  [16])0x0;
  do {
    iVar14 = auVar13._12_4_;
    auVar9._0_8_ = auVar13._8_8_;
    uVar2 = auVar13._0_8_;
    if (auVar9._0_8_ < uVar3) {
      uVar4 = auVar9._0_8_ + 1;
      if (uVar6 == 0) {
        bVar7 = uVar3 <= uVar4;
        uVar5 = auVar9._0_8_ + 2;
        if (bVar7) {
          uVar5 = uVar4;
        }
        auVar9._8_8_ = uVar4;
        uVar6 = 1;
        uVar2 = auVar9._0_8_;
        uVar4 = uVar5;
        auVar13 = auVar9;
        if (-1 < iVar14) goto LAB_1407e2a57;
        goto LAB_1407e2abd;
      }
      bVar7 = false;
      if ((longlong)uVar2 < 0) goto LAB_1407e2abd;
LAB_1407e2a57:
      fVar8 = (float)(longlong)uVar2;
      uVar2 = auVar13._8_8_;
      if (auVar13._12_4_ < 0) goto LAB_1407e2a6f;
LAB_1407e2ae5:
      fVar11 = (float)(longlong)uVar2;
    }
    else {
      bVar7 = true;
      if (uVar6 == 0) {
        return;
      }
      uVar4 = auVar9._0_8_;
      if (-1 < (longlong)uVar2) goto LAB_1407e2a57;
LAB_1407e2abd:
      fVar8 = (float)uVar2;
      uVar2 = auVar13._8_8_;
      if (-1 < auVar13._12_4_) goto LAB_1407e2ae5;
LAB_1407e2a6f:
      fVar11 = (float)uVar2;
    }
    auVar10._4_4_ = fVar11;
    auVar10._0_4_ = fVar8;
    auVar10._8_8_ = 0;
    auVar1._8_8_ = 0;
    divps(auVar10,auVar1);
    system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::
    build<system_decorator::decorator_visual_state::DecoratorTransientVisualState>();
    auVar13._8_8_ = uVar4;
    auVar13._0_8_ = uVar2;
    if (bVar7 || uVar3 < uVar4) {
      return;
    }
  } while( true );
}

