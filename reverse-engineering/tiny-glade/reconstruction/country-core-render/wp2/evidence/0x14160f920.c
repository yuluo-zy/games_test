// address 0x14160f920  name new_storage_buffer<country_core::systems::render_loop::resources::critters::SheepAnimationSsbo>
// signature: void __cdecl new_storage_buffer<country_core::systems::render_loop::resources::critters::SheepAnimationSsbo>(void)

// WARNING: Removing unreachable block (ram,0x00014160f9f1)

void __cdecl
country_core::render::ssbo::impl$0::
new_storage_buffer<country_core::systems::render_loop::resources::critters::SheepAnimationSsbo>
          (void)

{
  ulonglong in_R8;
  ulonglong in_R9;
  
  if (in_R8 == 0) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  if ((in_R9 % 0x3e0 == 0) && ((in_R8 & 3) == 0)) {
    rhapsody::raw::buffer::BufferDesc::new();
    rhapsody::raw::buffer::BufferDesc::alignment();
    rhapsody::raw::device::GraphicsDevice::
    create_buffer<enum2_<core::option::Option<ref_<slice2_<u8>_>_>_>_>();
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
                    // WARNING: Subroutine does not return
  core::panicking::assert_failed<usize,usize>();
}


