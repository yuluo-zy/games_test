// addr=0x140b405e0 name=load_shader_into_library_replace_previous<ref$<str$>_>
// signature=void __cdecl load_shader_into_library_replace_previous<ref$<str$>_>(void)


void __cdecl country_core::startup::load_shader_into_library_replace_previous<ref_<str_>_>(void)

{
  void *extraout_RAX;
  void *in_RDX;
  size_t in_R8;
  void *_Dst;
  longlong local_198;
  
  if ((longlong)in_R8 < 0) {
LAB_140b40879:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  if (in_R8 == 0) {
    _Dst = (void *)0x1;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    _Dst = extraout_RAX;
    if (extraout_RAX == (void *)0x0) goto LAB_140b40879;
  }
  memcpy(_Dst,in_RDX,in_R8);
  alloc::string::impl$6::clone();
  country_core::render::shader::compile_shader_variants();
  if (local_198 == 0) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  if (SBORROW8(0,local_198)) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  asset_libraries::shader_library::AssetShaderLibrary::add_raster();
  return;
}


