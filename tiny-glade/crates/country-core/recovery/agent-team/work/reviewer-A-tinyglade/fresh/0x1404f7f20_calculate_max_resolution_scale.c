// calculate_max_resolution_scale @ 0x1404f7f20 size=796
// sig: void __cdecl calculate_max_resolution_scale(void)


void __cdecl tiny_glade::vram::calculate_max_resolution_scale(void)

{
  int iVar1;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined8 extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong lVar2;
  ulonglong in_RDX;
  double dVar3;
  undefined1 auVar4 [16];
  undefined1 auVar5 [16];
  undefined1 auVar6 [16];
  double dVar7;
  double dVar8;
  double dVar9;
  double dVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  
  rhapsody::raw::device::GraphicsDevice::physical_device();
  iVar1 = *(int *)(extraout_RAX + 0xd8);
  rhapsody::raw::physical_device::PhysicalDevice::device_local_memory_budget_bytes();
  if (iVar1 == 2) {
    auVar4._8_4_ = (int)((ulonglong)extraout_RAX_00 >> 0x20);
    auVar4._0_8_ = extraout_RAX_00;
    auVar4._12_4_ = 0x45300000;
    dVar10 = (double)(in_RDX & 0xffffffff);
    dVar9 = (double)(in_RDX >> 0x20);
    dVar3 = (dVar10 * -47.7399274329577 * dVar9 +
            (auVar4._8_8_ - 1.9342813113834067e+25) +
            ((double)CONCAT44(0x43300000,(int)extraout_RAX_00) - 4503599627370496.0) + -992811760.0)
            / 146.1091661136655;
    fVar11 = 0.0;
    fVar13 = 0.0;
    if (0.0 < dVar3) {
      dVar3 = dVar3 / (dVar10 * dVar9);
      if (dVar3 <= 0.0) {
        dVar3 = 0.0;
      }
      dVar7 = dVar9;
      if (dVar10 <= dVar9) {
        dVar7 = dVar10;
      }
      dVar7 = (double)(~-(ulonglong)NAN(dVar10) & (ulonglong)dVar7 |
                      -(ulonglong)NAN(dVar10) & (ulonglong)dVar9);
      if (dVar7 <= 1.0) {
        dVar7 = 1.0;
      }
      dVar3 = SQRT(dVar3);
      dVar7 = 10912.0 / dVar7;
      dVar8 = dVar7;
      if (dVar3 <= dVar7) {
        dVar8 = dVar3;
      }
      fVar13 = (float)(double)(~-(ulonglong)NAN(dVar3) & (ulonglong)dVar8 |
                              -(ulonglong)NAN(dVar3) & (ulonglong)dVar7);
    }
    rhapsody::raw::physical_device::PhysicalDevice::host_memory_budget_bytes();
    auVar6._8_4_ = (int)((ulonglong)extraout_RAX_01 >> 0x20);
    auVar6._0_8_ = extraout_RAX_01;
    auVar6._12_4_ = 0x45300000;
    dVar3 = ((auVar6._8_8_ - 1.9342813113834067e+25) +
             ((double)CONCAT44(0x43300000,(int)extraout_RAX_01) - 4503599627370496.0) + -372245456.0
            ) / 0.0;
    if (0.0 < dVar3) {
      dVar3 = dVar3 / (dVar10 * dVar9);
      if (dVar3 <= 0.0) {
        dVar3 = 0.0;
      }
      dVar7 = dVar9;
      if (dVar10 <= dVar9) {
        dVar7 = dVar10;
      }
      dVar9 = (double)(~-(ulonglong)NAN(dVar10) & (ulonglong)dVar7 |
                      -(ulonglong)NAN(dVar10) & (ulonglong)dVar9);
      if (dVar9 <= 1.0) {
        dVar9 = 1.0;
      }
      dVar3 = SQRT(dVar3);
      dVar9 = 10912.0 / dVar9;
      dVar10 = dVar9;
      if (dVar3 <= dVar9) {
        dVar10 = dVar3;
      }
      fVar11 = (float)(double)(~-(ulonglong)NAN(dVar3) & (ulonglong)dVar10 |
                              -(ulonglong)NAN(dVar3) & (ulonglong)dVar9);
    }
    fVar12 = fVar11;
    if (fVar13 <= fVar11) {
      fVar12 = fVar13;
    }
    fVar11 = (float)(~-(uint)NAN(fVar13) & (uint)fVar12 | -(uint)NAN(fVar13) & (uint)fVar11);
  }
  else {
    rhapsody::raw::physical_device::PhysicalDevice::host_memory_budget_bytes();
    lVar2 = extraout_RAX_02 + extraout_RAX_00;
    auVar5._8_4_ = (int)((ulonglong)lVar2 >> 0x20);
    auVar5._0_8_ = lVar2;
    auVar5._12_4_ = 0x45300000;
    dVar3 = (double)(in_RDX & 0xffffffff);
    dVar10 = (double)(in_RDX >> 0x20);
    dVar9 = (dVar3 * -47.7399274329577 * dVar10 +
            (auVar5._8_8_ - 1.9342813113834067e+25) +
            ((double)CONCAT44(0x43300000,(int)lVar2) - 4503599627370496.0) + -1365057216.0) /
            146.1091661136655;
    fVar11 = 0.0;
    if (0.0 < dVar9) {
      dVar9 = dVar9 / (dVar3 * dVar10);
      if (dVar9 <= 0.0) {
        dVar9 = 0.0;
      }
      dVar7 = dVar10;
      if (dVar3 <= dVar10) {
        dVar7 = dVar3;
      }
      dVar10 = (double)(~-(ulonglong)NAN(dVar3) & (ulonglong)dVar7 |
                       -(ulonglong)NAN(dVar3) & (ulonglong)dVar10);
      if (dVar10 <= 1.0) {
        dVar10 = 1.0;
      }
      dVar9 = SQRT(dVar9);
      dVar10 = 10912.0 / dVar10;
      dVar3 = dVar10;
      if (dVar9 <= dVar10) {
        dVar3 = dVar9;
      }
      fVar11 = (float)(double)(~-(ulonglong)NAN(dVar9) & (ulonglong)dVar3 |
                              -(ulonglong)NAN(dVar9) & (ulonglong)dVar10);
    }
  }
  floorf(fVar11 * 10.0);
  return;
}


