// addr 0x1416191a0 sig void __cdecl new<country_core::resources::roof_tile_ssbo::RoofTileSsbo>(void) size 391

void __cdecl
country_core::render::ssbo::TypedStorageBuffer::
new<country_core::resources::roof_tile_ssbo::RoofTileSsbo>(void)

{
  undefined8 *in_RCX;
  longlong in_R8;
  ulonglong in_R9;
  undefined *local_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 local_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined **local_a8;
  undefined8 uStack_a0;
  undefined8 **local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined **local_50;
  code *local_48;
  undefined8 local_40;
  
  local_40 = 0xfffffffffffffffe;
  if (0x40000 < in_R9 && in_R8 != 0) {
    local_f8 = &anon_107183261458dea85466985370debbb6_82_llvm_3641509820080252961;
    uStack_f0 = 0x35;
    local_50 = &local_f8;
    local_48 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
    local_a8 = &PTR_DAT_142c7f7c8;
    uStack_a0 = 1;
    local_88 = 0;
    local_98 = &local_50;
    uStack_90 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  rhapsody::raw::buffer::BufferDesc::new();
  rhapsody::raw::buffer::BufferDesc::alignment();
  rhapsody::raw::device::GraphicsDevice::
  create_buffer<enum2_<core::option::Option<ref_<slice2_<u8>_>_>_>_>();
  if ((int)local_f8 == 0xb) {
    in_RCX[4] = local_d0;
    in_RCX[2] = uStack_e0;
    in_RCX[3] = uStack_d8;
    *in_RCX = uStack_f0;
    in_RCX[1] = uStack_e8;
    return;
  }
  local_68 = local_b8;
  uStack_60 = uStack_b0;
  local_78 = local_c8;
  uStack_70 = uStack_c0;
  local_88 = uStack_d8;
  uStack_80 = local_d0;
  local_98 = (undefined8 **)uStack_e8;
  uStack_90 = uStack_e0;
  local_a8 = (undefined **)local_f8;
  uStack_a0 = uStack_f0;
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
}


// CALLEES: [{"address": "140bd4f20", "name": "alignment"}, {"address": "1416a2dc0", "name": "create_buffer<enum2$<core::option::Option<ref$<slice2$<u8>_>_>_>_>"}, {"address": "140bd4de0", "name": "new"}, {"address": "1428d9390", "name": "panic_fmt"}, {"address": "1428d9760", "name": "unwrap_failed"}]