
void __cdecl
core::ops::function::impls::impl$3::
call_mut<tuple_<tuple_<ref_<country_core::resources::roofs::roof_shape::Roof>,ref_<country_core::resources::roofs::roof_animation::RoofAnimation>_>_>,system_roof::visual::place_spires::place_roof_spires::closure_env_0>
          (void)

{
  undefined4 uVar1;
  undefined4 uVar2;
  float fVar3;
  char extraout_AL;
  undefined8 extraout_RAX;
  undefined8 extraout_RAX_00;
  undefined8 *in_RCX;
  undefined8 uVar4;
  longlong in_R8;
  longlong in_R9;
  float extraout_XMM0_Da;
  float fVar5;
  undefined1 auVar6 [16];
  float fVar7;
  float local_68;
  float fStack_64;
  float fStack_60;
  float local_54;
  undefined8 local_50;
  
  uVar1 = *(undefined4 *)(in_R9 + 4);
  country_core::resources::roofs::roof_shape::Roof::ty();
  uVar4 = 0;
  if (extraout_AL != '\0') {
    country_core::resources::roofs::roof_ridge::RoofRidgeDims::from();
    local_68 = (float)extraout_RAX;
    fStack_64 = (float)((ulonglong)extraout_RAX >> 0x20);
    minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
    fVar5 = (float)((ulonglong)extraout_RAX_00 >> 0x20);
    fVar7 = (float)extraout_RAX_00 / fVar5;
    fVar5 = fVar5 / (float)extraout_RAX_00;
    fVar3 = fVar5;
    if (fVar5 <= fVar7) {
      fVar3 = fVar7;
    }
    uVar4 = 0;
    if ((float)(~-(uint)NAN(fVar7) & (uint)fVar3 | -(uint)NAN(fVar7) & (uint)fVar5) <
        __real_3f8147ae) {
      country_core::resources::roofs::roof_shape::Roof::pivot_3d();
      country_core::resources::roofs::roof_shape::Roof::roof_tip_offset_xz();
      ::utils::swizzlers::impl$0::x0y();
      country_core::resources::roofs::roof_shape::Roof::height();
      auVar6._4_4_ = extraout_XMM0_Da + fStack_64 + (float)local_50;
      auVar6._0_4_ = local_54 + local_68 + extraout_XMM0_Da * 0.0;
      auVar6._8_4_ = extraout_XMM0_Da * 0.0 + fStack_60 + (float)((ulonglong)local_50 >> 0x20);
      auVar6._12_4_ = 0;
      uVar2 = *(undefined4 *)(in_R8 + 0x2c);
      *(undefined1 (*) [16])(in_RCX + 2) = auVar6;
      *(undefined4 *)(in_RCX + 4) = uVar1;
      *(undefined4 *)((longlong)in_RCX + 0x24) = uVar2;
      in_RCX[5] = 0x3e800000;
      uVar4 = 1;
    }
  }
  *in_RCX = uVar4;
  in_RCX[1] = 0;
  return;
}

