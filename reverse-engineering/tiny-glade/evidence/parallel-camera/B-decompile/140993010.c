
void __cdecl country_core::systems::camera_rig::free::FreeCamera::get_yaw_pitch(void)

{
  undefined8 *puVar1;
  longlong lVar2;
  longlong lVar3;
  longlong in_RCX;
  longlong in_RDX;
  longlong lVar4;
  undefined8 *puVar5;
  undefined **local_78;
  code *local_70;
  undefined *local_68;
  undefined8 local_60;
  undefined *local_58;
  undefined8 local_50;
  undefined8 **local_48;
  undefined8 local_40;
  undefined8 local_38;
  
  lVar4 = *(longlong *)(in_RCX + 0x10) << 4;
  puVar5 = *(undefined8 **)(in_RCX + 8);
  do {
    if (lVar4 == 0) {
      local_68 = &anon_f854cc98b5e1efe54f6ab98d53b9f85b_13_llvm_5637972437638668905;
      local_60 = 0x23;
      local_78 = &local_68;
      local_70 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
      local_58 = &anon_f854cc98b5e1efe54f6ab98d53b9f85b_10_llvm_5637972437638668905;
      local_50 = 2;
      local_38 = 0;
      local_48 = &local_78;
      local_40 = 1;
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_fmt();
    }
    puVar1 = puVar5 + 2;
    lVar2 = (**(code **)(puVar5[1] + 0x38))(*puVar5);
    lVar3 = (**(code **)(in_RDX + 0x18))(lVar2);
    lVar4 = lVar4 + -0x10;
    puVar5 = puVar1;
  } while (((lVar2 == 0) || (lVar3 != 0x718e4fd258330fe5)) || (in_RDX != 0x7f9f9b0da2744bb));
  return;
}

