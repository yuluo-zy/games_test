
void __cdecl
country_core::resources::glade_border::GladeBorder::search_lerped_shape_that_fits_into_glade(void)

{
  uint *puVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  uint *in_RCX;
  longlong lVar6;
  byte *pbVar7;
  uint *in_R8;
  byte *in_R9;
  uint uVar8;
  undefined1 local_118 [8];
  uint *local_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  undefined8 local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined4 local_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined8 local_b8;
  undefined4 local_b0;
  undefined4 uStack_ac;
  undefined8 uStack_a8;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  
  uVar8 = *in_R8;
  puVar1 = in_R8 + 1;
  local_d8 = *(undefined8 *)(in_R8 + 5);
  local_e8 = *(undefined8 *)(in_R8 + 1);
  uStack_e0 = *(undefined8 *)(in_R8 + 3);
  pbVar7 = in_R9 + 4;
  local_110 = in_RCX;
  if ((uVar8 & 1) == 0) {
    if ((*in_R9 & 1) != 0) goto LAB_140ad3029;
    local_98 = *(undefined8 *)puVar1;
    uStack_90 = *(undefined8 *)(in_R8 + 3);
    local_78 = *(undefined8 *)pbVar7;
    uStack_70 = *(undefined8 *)(in_R9 + 0xc);
    ::utils::geometry::circle::Circle2d::lerp();
    local_b8._0_4_ = uStack_bc;
    uVar4 = uStack_c0;
    uVar3 = uStack_c4;
    uVar2 = local_c8;
    local_108 = CONCAT44(uStack_c4,local_c8);
    uStack_100 = CONCAT44(uStack_bc,uStack_c0);
    local_c8 = 0;
    local_b8._4_4_ = (undefined4)local_f8;
    local_b0 = (undefined4)((ulonglong)local_f8 >> 0x20);
    uStack_c4 = uVar2;
    uStack_c0 = uVar3;
    uStack_bc = uVar4;
    is_shape_inside();
    if (extraout_AL_01 != '\0') {
      lVar6 = 1;
      do {
        local_98 = *(undefined8 *)puVar1;
        uStack_90 = *(undefined8 *)(in_R8 + 3);
        local_78 = *(undefined8 *)pbVar7;
        uStack_70 = *(undefined8 *)(in_R9 + 0xc);
        local_e8 = local_108;
        uStack_e0 = uStack_100;
        local_d8 = local_f8;
        ::utils::geometry::circle::Circle2d::lerp();
        local_b8._0_4_ = uStack_bc;
        uVar4 = uStack_c0;
        uVar3 = uStack_c4;
        uVar2 = local_c8;
        local_108 = CONCAT44(uStack_c4,local_c8);
        uStack_100 = CONCAT44(uStack_bc,uStack_c0);
        local_c8 = 0;
        local_b8._4_4_ = (undefined4)local_f8;
        local_b0 = (undefined4)((ulonglong)local_f8 >> 0x20);
        uStack_c4 = uVar2;
        uStack_c0 = uVar3;
        uStack_bc = uVar4;
        is_shape_inside();
        if (extraout_AL_02 == '\0') break;
        lVar6 = lVar6 + 1;
        local_e8 = local_108;
        uStack_e0 = uStack_100;
        local_d8 = local_f8;
      } while (lVar6 != 0x14);
      uVar8 = 0;
      in_RCX = local_110;
    }
  }
  else {
    if ((*in_R9 & 1) == 0) {
LAB_140ad3029:
      local_c8 = 0x42ae1408;
      uStack_c4 = 1;
      uStack_c0 = 1;
      uStack_bc = 0;
      local_b8 = local_118;
      local_b0 = 0;
      uStack_ac = 0;
      uStack_a8 = 0;
                    // WARNING: Subroutine does not return
      core::panicking::panic_fmt();
    }
    local_88 = *(undefined8 *)(in_R8 + 5);
    local_98 = *(undefined8 *)puVar1;
    uStack_90 = *(undefined8 *)(in_R8 + 3);
    local_68 = *(undefined8 *)(in_R9 + 0x14);
    local_78 = *(undefined8 *)pbVar7;
    uStack_70 = *(undefined8 *)(in_R9 + 0xc);
    ::utils::geometry::rectangle::Rectangle2d::lerp();
    local_b0 = local_b8._4_4_;
    uVar5 = uStack_bc;
    uVar4 = uStack_c0;
    uVar3 = uStack_c4;
    uVar2 = local_c8;
    local_f8 = CONCAT44(local_b8._4_4_,(undefined4)local_b8);
    local_108 = CONCAT44(uStack_c4,local_c8);
    uStack_100 = CONCAT44(uStack_bc,uStack_c0);
    local_c8 = 1;
    local_b8._4_4_ = (undefined4)local_b8;
    uStack_c4 = uVar2;
    uStack_c0 = uVar3;
    uStack_bc = uVar4;
    local_b8._0_4_ = uVar5;
    is_shape_inside();
    if (extraout_AL != '\0') {
      lVar6 = 1;
      do {
        local_88 = *(undefined8 *)(in_R8 + 5);
        local_98 = *(undefined8 *)puVar1;
        uStack_90 = *(undefined8 *)(in_R8 + 3);
        local_68 = *(undefined8 *)(in_R9 + 0x14);
        local_78 = *(undefined8 *)pbVar7;
        uStack_70 = *(undefined8 *)(in_R9 + 0xc);
        local_e8 = local_108;
        uStack_e0 = uStack_100;
        local_d8 = local_f8;
        ::utils::geometry::rectangle::Rectangle2d::lerp();
        local_b0 = local_b8._4_4_;
        uVar5 = uStack_bc;
        uVar4 = uStack_c0;
        uVar3 = uStack_c4;
        uVar2 = local_c8;
        local_f8 = CONCAT44(local_b8._4_4_,(undefined4)local_b8);
        local_108 = CONCAT44(uStack_c4,local_c8);
        uStack_100 = CONCAT44(uStack_bc,uStack_c0);
        local_c8 = 1;
        local_b8._4_4_ = (undefined4)local_b8;
        uStack_c4 = uVar2;
        uStack_c0 = uVar3;
        uStack_bc = uVar4;
        local_b8._0_4_ = uVar5;
        is_shape_inside();
        if (extraout_AL_00 == '\0') break;
        lVar6 = lVar6 + 1;
        local_e8 = local_108;
        uStack_e0 = uStack_100;
        local_d8 = local_f8;
      } while (lVar6 != 0x14);
      uVar8 = 1;
      in_RCX = local_110;
    }
  }
  *in_RCX = uVar8;
  *(undefined8 *)(in_RCX + 1) = local_e8;
  *(undefined8 *)(in_RCX + 3) = uStack_e0;
  *(undefined8 *)(in_RCX + 5) = local_d8;
  return;
}


