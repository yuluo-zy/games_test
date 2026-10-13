// address 0x14161ad70  name new_storage_buffer<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo>
// signature: void __cdecl new_storage_buffer<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo>(void)

void __cdecl
country_core::render::ssbo::impl$4::
new_storage_buffer<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo>(void)

{
  void *_Dst;
  longlong extraout_RAX;
  longlong *in_RCX;
  ulonglong in_RDX;
  longlong in_R8;
  ulonglong in_R9;
  char cVar1;
  undefined4 uVar2;
  undefined8 unaff_R12;
  undefined *local_110;
  longlong lStack_108;
  longlong lStack_100;
  longlong lStack_f8;
  ulonglong uStack_f0;
  longlong local_e8;
  undefined8 local_e0;
  undefined8 uStack_d8;
  undefined8 local_d0;
  undefined8 uStack_c8;
  undefined **local_c0;
  code *local_b8;
  undefined **local_a8;
  longlong lStack_a0;
  undefined8 **local_98;
  ulonglong uStack_90;
  ulonglong local_88;
  longlong lStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined4 local_54;
  char local_41;
  undefined8 local_40;
  
  local_40 = 0xfffffffffffffffe;
  if (in_R8 == 0) {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (_Dst == (void *)0x0) {
      local_41 = 0;
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    memset(_Dst,0,0x40000);
    uVar2 = (undefined4)CONCAT71((int7)((ulonglong)unaff_R12 >> 8),1);
  }
  else {
    if (0x40000 < in_R9) {
      local_110 = &anon_107183261458dea85466985370debbb6_27_llvm_3641509820080252961;
      lStack_108 = 0x49;
      local_c0 = &local_110;
      local_b8 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
      local_a8 = &PTR_DAT_142c7f7c8;
      lStack_a0 = 1;
      local_88 = 0;
      local_98 = &local_c0;
      uStack_90 = 1;
      local_41 = 0;
                    // WARNING: Subroutine does not return
      core::panicking::panic_fmt();
    }
    uVar2 = 0;
  }
  cVar1 = (char)uVar2;
  local_41 = cVar1;
  rhapsody::raw::buffer::BufferDesc::new();
  local_41 = cVar1;
  rhapsody::raw::buffer::BufferDesc::alignment();
  local_41 = cVar1;
  rhapsody::raw::device::GraphicsDevice::
  create_buffer<enum2_<core::option::Option<ref_<slice2_<u8>_>_>_>_>();
  if ((int)local_110 == 0xb) {
    local_88 = local_e8;
    local_98 = (undefined8 **)lStack_f8;
    uStack_90 = uStack_f0;
    local_a8 = (undefined **)lStack_108;
    lStack_a0 = lStack_100;
    if (cVar1 != '\0') {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    rhapsody::raw::buffer::Buffer::mapped_slice_mut();
    if (extraout_RAX != 0) {
      if (uStack_90 <= in_RDX) {
        in_RCX[6] = local_88;
        in_RCX[4] = (longlong)local_98;
        in_RCX[5] = uStack_90;
        in_RCX[2] = (longlong)local_a8;
        in_RCX[3] = lStack_a0;
        *in_RCX = extraout_RAX;
        in_RCX[1] = in_RDX;
        return;
      }
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  local_68 = local_d0;
  uStack_60 = uStack_c8;
  local_78 = local_e0;
  uStack_70 = uStack_d8;
  local_88 = uStack_f0;
  lStack_80 = local_e8;
  local_98 = (undefined8 **)lStack_100;
  uStack_90 = lStack_f8;
  local_a8 = (undefined **)local_110;
  lStack_a0 = lStack_108;
  local_54 = uVar2;
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
}


