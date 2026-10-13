// address 0x140909690  name build_vao

void __cdecl country_core::render::vao::build_vao(void)

{
  undefined8 *in_RCX;
  undefined8 local_120;
  undefined8 uStack_118;
  undefined8 local_110;
  int local_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined8 local_e0;
  
  rhapsody::raw::buffer::BufferDesc::new_gpu_only();
  rhapsody::raw::device::GraphicsDevice::
  create_buffer<enum2_<core::option::Option<ref_<slice2_<u8>_>_>_>_>();
  if (local_108 == 0xb) {
    alloc::vec::spec_from_iter::impl$0::
    from_iter<country_core::render::vao::VaoBinding,core::iter::adapters::map::Map<std::collections::hash::map::Iter<alloc::string::String,enum2_<country_core::render::mesh::VertexAttributeValues>_>,country_core::render::vao::build_vao::closure_env_0>_>
              ();
    in_RCX[7] = local_e0;
    in_RCX[5] = uStack_f0;
    in_RCX[6] = uStack_e8;
    in_RCX[3] = uStack_100;
    in_RCX[4] = uStack_f8;
    *in_RCX = local_120;
    in_RCX[1] = uStack_118;
    in_RCX[2] = local_110;
    return;
  }
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
}


