// address 0x141618e00  name new<country_core::render::nani::instance_data::PlantInstanceData>
// signature: void __cdecl new<country_core::render::nani::instance_data::PlantInstanceData>(void)

void __cdecl
country_core::render::ssbo::TypedStorageBuffer::
new<country_core::render::nani::instance_data::PlantInstanceData>(void)

{
  undefined8 *in_RCX;
  longlong in_R8;
  int local_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 local_78;
  
  if (in_R8 == 0) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  rhapsody::raw::buffer::BufferDesc::new();
  rhapsody::raw::buffer::BufferDesc::alignment();
  rhapsody::raw::device::GraphicsDevice::
  create_buffer<enum2_<core::option::Option<ref_<slice2_<u8>_>_>_>_>();
  if (local_a0 == 0xb) {
    in_RCX[4] = local_78;
    in_RCX[2] = uStack_88;
    in_RCX[3] = uStack_80;
    *in_RCX = uStack_98;
    in_RCX[1] = uStack_90;
    return;
  }
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
}


