// address 0x14168f870  name new<country_core::render::nani::instance_data::SolidAlbedoMapInstanceData>
// signature: void __cdecl new<country_core::render::nani::instance_data::SolidAlbedoMapInstanceData>(void)

void __cdecl
country_core::render::shadowed_buffer::ShadowedBuffer::
new<country_core::render::nani::instance_data::SolidAlbedoMapInstanceData>(void)

{
  ulonglong uVar1;
  ulonglong extraout_RAX;
  undefined8 *in_RCX;
  ulonglong uVar2;
  ulonglong *in_R8;
  undefined8 local_80;
  undefined8 uStack_78;
  undefined8 local_70;
  undefined8 uStack_68;
  undefined8 local_60;
  
  uVar2 = in_R8[1];
  uVar1 = in_R8[2];
  ssbo::TypedStorageBuffer::
  new<country_core::render::nani::instance_data::SolidAlbedoMapInstanceData>();
  if (uVar1 < *in_R8) {
    if (uVar1 == 0) {
      uVar2 = 4;
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    else {
      std::alloc::__default_lib_allocator::__rust_realloc();
      uVar2 = extraout_RAX;
      if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
    }
  }
  in_RCX[8] = local_60;
  in_RCX[6] = local_70;
  in_RCX[7] = uStack_68;
  in_RCX[4] = local_80;
  in_RCX[5] = uStack_78;
  in_RCX[2] = uVar2;
  in_RCX[3] = uVar1;
  *in_RCX = 0xffffffffffffffff;
  in_RCX[1] = 0;
  return;
}


