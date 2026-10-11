
void __cdecl country_core::systems::camera_rig::free::FreeCamera::get_position(void)

{
  undefined8 *puVar1;
  undefined4 uVar2;
  undefined8 *puVar3;
  longlong lVar4;
  undefined8 *in_RCX;
  longlong in_RDX;
  longlong lVar5;
  undefined8 *puVar6;
  char **local_80;
  code *local_78;
  char *local_70;
  undefined8 local_68;
  undefined *local_60;
  undefined8 local_58;
  undefined8 **local_50;
  undefined8 local_48;
  undefined8 local_40;
  
  lVar5 = *(longlong *)(in_RDX + 0x10) << 4;
  puVar6 = *(undefined8 **)(in_RDX + 8);
  do {
    if (lVar5 == 0) {
      local_70 = "dolly::drivers::position::PositionNo  driver found in the CameraRig";
      local_68 = 0x22;
      local_80 = &local_70;
      local_78 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
      local_60 = &anon_f854cc98b5e1efe54f6ab98d53b9f85b_10_llvm_5637972437638668905;
      local_58 = 2;
      local_40 = 0;
      local_50 = &local_80;
      local_48 = 1;
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_fmt();
    }
    puVar1 = puVar6 + 2;
    puVar3 = (undefined8 *)(**(code **)(puVar6[1] + 0x38))(*puVar6);
    lVar4 = (**(code **)(in_RDX + 0x18))(puVar3);
    lVar5 = lVar5 + -0x10;
    puVar6 = puVar1;
  } while (((puVar3 == (undefined8 *)0x0) || (lVar4 != 0x6b8463577afc1a66)) ||
          (in_RDX != 0xe4b9f294168d7e3));
  uVar2 = *(undefined4 *)(puVar3 + 1);
  *in_RCX = *puVar3;
  *(undefined4 *)(in_RCX + 1) = uVar2;
  return;
}

