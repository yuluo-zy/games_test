// name: get_window_aabb
// addr: 0x141221da0
// size: 381
// signature: void __cdecl get_window_aabb(void)


void __cdecl system_wall_constructor::utils::half_timber::get_window_aabb(void)

{
  undefined8 uVar1;
  undefined8 uVar3;
  undefined8 *extraout_RAX;
  undefined8 *puVar4;
  undefined8 *in_RCX;
  longlong in_RDX;
  char in_R8B;
  float fVar5;
  float local_78;
  float fStack_74;
  float fStack_70;
  float fStack_6c;
  longlong local_60;
  longlong local_58;
  undefined4 local_50;
  undefined4 uStack_4c;
  undefined8 uVar2;
  
  if (*(char *)(in_RDX + 0x46) == '9') {
    country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall();
    uVar1 = *extraout_RAX;
    uVar2 = *extraout_RAX;
    if (in_R8B == '\t') {
      country_core::resources::gothic_window_profile::gothic_door_profile_curve();
    }
    else {
      country_core::resources::gothic_window_profile::gothic_half_dormer_profile_curve();
    }
    if ((CONCAT44(uStack_4c,local_50) == 0) ||
       (puVar4 = (undefined8 *)(local_58 + CONCAT44(uStack_4c,local_50) * 8 + -8),
       puVar4 == (undefined8 *)0x0)) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    uVar3 = *puVar4;
    fVar5 = (float)uVar3;
    local_78 = (float)uVar1 - fVar5;
    fStack_74 = (float)((ulonglong)uVar3 >> 0x20) + (float)((ulonglong)uVar1 >> 0x20);
    fStack_70 = fVar5 + (float)uVar2;
    fStack_6c = *(float *)(local_58 + 4) + 0.093 + (float)((ulonglong)uVar2 >> 0x20);
    if (local_60 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  else {
    country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::
    bbx_ls<country_core::resources::walls::decorator_subtype::DecoratorSubtype>();
    country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall();
    ::utils::geometry::aabb::Aabb2::from_center_dims();
  }
  *in_RCX = CONCAT44(fStack_74,local_78);
  in_RCX[1] = CONCAT44(fStack_6c,fStack_70);
  return;
}

