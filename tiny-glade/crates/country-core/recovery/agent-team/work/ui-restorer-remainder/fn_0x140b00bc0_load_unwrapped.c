void __cdecl load_unwrapped(void)


// WARNING: Removing unreachable block (ram,0x000140b00d9c)
// WARNING: Removing unreachable block (ram,0x000140b00deb)
// WARNING: Removing unreachable block (ram,0x000140b00dfd)

void __cdecl country_core::resources::grading_lut::GradingLut::load_unwrapped(void)

{
  undefined1 *in_R8;
  longlong local_120;
  
  alloc::fmt::format::format_inner();
  image::io::reader::Reader::open_impl();
  if (local_120 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if ((in_R8 != (undefined1 *)0x2) &&
     (image::io::reader::Reader::decode<std::io::buffered::bufreader::BufReader<std::fs::File>_>(),
     in_R8 == &DAT_800000000000000a)) {
    image::dynimage::DynamicImage::to_rgba8();
    core::ptr::drop_in_place<enum2_<image::dynimage::DynamicImage>_>();
    rhapsody::raw::texture::TextureDesc::new_3d();
    rhapsody::raw::texture::TextureDesc::sampled();
    rhapsody::raw::device::GraphicsDevice::create_texture<ref_<slice2_<u8>_>_>();
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
}

