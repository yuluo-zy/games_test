// calculate_resolution_scale_mem_estimate_text @ 0x1404f8240
// sig: void __cdecl calculate_resolution_scale_mem_estimate_text(void)


void __cdecl tiny_glade::vram::calculate_resolution_scale_mem_estimate_text(void)

{
  longlong extraout_RAX;
  undefined8 extraout_RAX_00;
  undefined8 extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong lVar1;
  ulonglong uVar2;
  ulonglong uVar3;
  ulonglong in_R8;
  double dVar4;
  undefined1 auVar5 [16];
  undefined1 auVar6 [16];
  undefined1 auVar7 [16];
  undefined1 auVar8 [16];
  undefined1 auVar9 [16];
  double dVar10;
  double dVar11;
  undefined1 auVar12 [16];
  float in_XMM3_Da;
  double dVar13;
  undefined **local_1c8;
  undefined **local_1c0;
  undefined *local_1b8;
  double local_1b0;
  undefined **local_1a8;
  code *local_1a0;
  undefined ***local_198;
  code *local_190;
  double *local_188;
  code *local_180;
  double *local_178;
  code *local_170;
  undefined **local_168;
  code *local_160;
  undefined ***local_158;
  code *local_150;
  undefined8 *local_148;
  undefined8 local_140;
  double local_138 [4];
  undefined8 local_118;
  undefined8 local_110;
  undefined1 local_108;
  undefined8 local_100;
  undefined8 local_f8;
  undefined8 local_f0;
  undefined8 local_e0;
  undefined8 local_d8;
  undefined1 local_d0;
  undefined8 local_c8;
  undefined8 local_c0;
  undefined8 local_b8;
  undefined8 local_a8;
  undefined8 local_a0;
  undefined1 local_98;
  undefined8 local_90;
  undefined8 local_88;
  undefined8 local_80;
  undefined8 local_70;
  undefined8 local_68;
  undefined1 local_60;
  
  rhapsody::raw::device::GraphicsDevice::physical_device();
  dVar10 = (double)(in_R8 & 0xffffffff) * (double)(in_R8 >> 0x20);
  dVar13 = dVar10 * 47.7399274329577;
  dVar4 = (double)(in_R8 & 0xffffffff) * (double)in_XMM3_Da * (double)(in_R8 >> 0x20) *
          (double)in_XMM3_Da;
  dVar11 = dVar4 * 146.1091661136655;
  if (*(int *)(extraout_RAX + 0xd8) == 2) {
    dVar11 = dVar11 + dVar13 + 992811760.0;
    uVar3 = (longlong)(dVar11 - 9.223372036854776e+18) & (longlong)dVar11 >> 0x3f | (longlong)dVar11
    ;
    if (dVar11 < 0.0) {
      uVar3 = 0;
    }
    if (1.844674407370955e+19 < dVar11) {
      uVar3 = 0xffffffffffffffff;
    }
    auVar12._8_4_ = (int)(uVar3 >> 0x20);
    auVar12._0_8_ = uVar3;
    auVar12._12_4_ = 0x45300000;
    local_1b8 = (undefined *)
                (((auVar12._8_8_ - 1.9342813113834067e+25) +
                 ((double)CONCAT44(0x43300000,(int)uVar3) - 4503599627370496.0)) *
                9.313225746154785e-10);
    dVar4 = dVar4 * 0.0 + dVar10 * 0.0 + 372245456.0;
    uVar3 = (longlong)(dVar4 - 9.223372036854776e+18) & (longlong)dVar4 >> 0x3f | (longlong)dVar4;
    if (dVar4 < 0.0) {
      uVar3 = 0;
    }
    if (1.844674407370955e+19 < dVar4) {
      uVar3 = 0xffffffffffffffff;
    }
    auVar5._8_4_ = (int)(uVar3 >> 0x20);
    auVar5._0_8_ = uVar3;
    auVar5._12_4_ = 0x45300000;
    local_1b0 = ((auVar5._8_8_ - 1.9342813113834067e+25) +
                ((double)CONCAT44(0x43300000,(int)uVar3) - 4503599627370496.0)) *
                9.313225746154785e-10;
    rhapsody::raw::physical_device::PhysicalDevice::device_local_memory_budget_bytes();
    auVar6._8_4_ = (int)((ulonglong)extraout_RAX_00 >> 0x20);
    auVar6._0_8_ = extraout_RAX_00;
    auVar6._12_4_ = 0x45300000;
    local_1c8 = (undefined **)
                (((auVar6._8_8_ - 1.9342813113834067e+25) +
                 ((double)CONCAT44(0x43300000,(int)extraout_RAX_00) - 4503599627370496.0)) *
                9.313225746154785e-10);
    rhapsody::raw::physical_device::PhysicalDevice::host_memory_budget_bytes();
    auVar7._8_4_ = (int)((ulonglong)extraout_RAX_01 >> 0x20);
    auVar7._0_8_ = extraout_RAX_01;
    auVar7._12_4_ = 0x45300000;
    dVar4 = (auVar7._8_8_ - 1.9342813113834067e+25) +
            ((double)CONCAT44(0x43300000,(int)extraout_RAX_01) - 4503599627370496.0);
    local_1a8 = &local_1b8;
    local_1a0 = core::fmt::float::impl$9::fmt;
    local_198 = &local_1c8;
    local_190 = core::fmt::float::impl$9::fmt;
    local_188 = &local_1b0;
    local_180 = core::fmt::float::impl$9::fmt;
    local_178 = (double *)&local_1c0;
    local_170 = core::fmt::float::impl$9::fmt;
    local_c8 = 0;
    local_c0 = 2;
    local_b8 = 2;
    local_a8 = 2;
    local_a0 = 0x20;
    local_98 = 3;
    local_90 = 0;
    local_88 = 2;
    local_80 = 2;
    local_70 = 3;
    local_68 = 0x20;
    local_60 = 3;
    local_168 = &PTR_DAT_1429ed2c0;
    local_160 = (code *)0x5;
    local_158 = &local_1a8;
    local_150 = (code *)0x4;
    local_148 = local_138;
    local_140 = 4;
  }
  else {
    dVar11 = dVar11 + dVar13 + 1365057216.0;
    uVar3 = 0;
    if (0.0 <= dVar11) {
      uVar3 = (longlong)(dVar11 - 9.223372036854776e+18) & (longlong)dVar11 >> 0x3f |
              (longlong)dVar11;
    }
    uVar2 = 0xffffffffffffffff;
    if (dVar11 <= 1.844674407370955e+19) {
      uVar2 = uVar3;
    }
    auVar8._8_4_ = (int)(uVar2 >> 0x20);
    auVar8._0_8_ = uVar2;
    auVar8._12_4_ = 0x45300000;
    local_1c8 = (undefined **)
                (((auVar8._8_8_ - 1.9342813113834067e+25) +
                 ((double)CONCAT44(0x43300000,(int)uVar2) - 4503599627370496.0)) *
                9.313225746154785e-10);
    rhapsody::raw::physical_device::PhysicalDevice::device_local_memory_budget_bytes();
    rhapsody::raw::physical_device::PhysicalDevice::host_memory_budget_bytes();
    lVar1 = extraout_RAX_03 + extraout_RAX_02;
    auVar9._8_4_ = (int)((ulonglong)lVar1 >> 0x20);
    auVar9._0_8_ = lVar1;
    auVar9._12_4_ = 0x45300000;
    dVar4 = (auVar9._8_8_ - 1.9342813113834067e+25) +
            ((double)CONCAT44(0x43300000,(int)lVar1) - 4503599627370496.0);
    local_168 = (undefined **)&local_1c8;
    local_160 = core::fmt::float::impl$9::fmt;
    local_158 = &local_1c0;
    local_150 = core::fmt::float::impl$9::fmt;
    local_1a8 = &PTR_DAT_1429ed280;
    local_1a0 = (code *)0x3;
    local_198 = &local_168;
    local_190 = (code *)0x2;
    local_188 = local_138;
    local_180 = (code *)0x2;
  }
  local_1c0 = (undefined **)(dVar4 * 9.313225746154785e-10);
  local_d0 = 3;
  local_d8 = 0x20;
  local_e0 = 1;
  local_f0 = 2;
  local_f8 = 2;
  local_100 = 0;
  local_108 = 3;
  local_110 = 0x20;
  local_118 = 0;
  local_138[2] = 9.88131291682493e-324;
  local_138[1] = 9.88131291682493e-324;
  local_138[0] = 0.0;
  alloc::fmt::format::format_inner();
  return;
}

