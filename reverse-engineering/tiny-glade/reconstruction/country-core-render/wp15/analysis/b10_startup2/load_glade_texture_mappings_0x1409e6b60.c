// addr=0x1409e6b60 name=load_glade_texture_mappings
// signature=void __cdecl load_glade_texture_mappings(void)


void __cdecl country_core::startup::load_textures::load_glade_texture_mappings(void)

{
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  undefined8 *extraout_RAX_03;
  undefined8 *extraout_RAX_04;
  undefined8 *extraout_RAX_05;
  undefined8 *extraout_RAX_06;
  undefined8 *extraout_RAX_07;
  longlong in_RDX;
  longlong local_270;
  undefined8 local_268;
  undefined8 local_260;
  undefined8 local_250;
  undefined8 local_248;
  undefined1 local_240;
  undefined8 local_238;
  undefined8 local_228;
  undefined8 local_218;
  undefined8 local_210;
  undefined1 local_208;
  undefined *local_198;
  undefined8 local_190;
  undefined ***local_188;
  undefined8 local_180;
  longlong *local_178;
  undefined8 local_170;
  undefined1 local_168;
  undefined8 local_160;
  undefined8 local_150;
  undefined8 local_140;
  undefined8 local_138;
  undefined1 local_130;
  undefined *local_c0 [4];
  undefined **local_a0;
  code *local_98;
  undefined8 *local_90;
  code *local_88;
  undefined **local_80;
  code *local_78;
  undefined8 **local_70;
  code *local_68;
  undefined **local_60;
  undefined8 local_58;
  undefined8 local_50;
  undefined8 *local_48;
  undefined8 local_40 [2];
  undefined8 local_30;
  
  local_30 = 0xfffffffffffffffe;
  std::path::Path::_join();
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::path::Path::_join();
  if (local_270 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::fs::read_to_string::inner();
  local_48 = local_a0;
  if (SBORROW8(0,(longlong)local_a0)) {
    local_40[0] = local_98;
    local_78 = std::path::impl$44::fmt;
    local_70 = (undefined8 **)local_40;
    local_68 = std::io::error::impl$9::fmt;
    local_270 = 2;
    local_260 = 2;
    local_250 = 0;
    local_248 = 0x20;
    local_240 = 3;
    local_238 = 2;
    local_228 = 2;
    local_218 = 1;
    local_210 = 0x400000020;
    local_208 = 3;
    local_198 = &DAT_142ac4860;
    local_190 = 2;
    local_170 = 2;
    local_188 = &local_80;
    local_180 = 2;
    local_178 = &local_270;
    local_80 = local_c0;
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  local_50 = local_98;
  serde_json::de::from_str<country_core::startup::load_textures::GladeTextureMappings>();
  if (SBORROW8(0,local_270)) {
    local_40[0] = local_268;
    local_98 = std::path::impl$44::fmt;
    local_90 = local_40;
    local_88 = serde_json::error::impl$5::fmt;
    local_198 = (undefined *)0x2;
    local_188 = (undefined ***)0x2;
    local_178 = (longlong *)0x0;
    local_170 = 0x20;
    local_168 = 3;
    local_160 = 2;
    local_150 = 2;
    local_140 = 1;
    local_138 = 0x400000020;
    local_130 = 3;
    local_80 = &PTR_s_Couldn_t_deserialize_142ac48b0;
    local_78 = (code *)0x2;
    local_60 = &local_198;
    local_58 = 2;
    local_68 = (code *)0x2;
    local_a0 = local_c0;
    local_70 = &local_a0;
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  memcpy(&local_198,&local_270,0xd8);
  **(undefined4 **)(in_RDX + 0x10) = *(undefined4 *)(in_RDX + 0x1c);
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  *extraout_RAX = 0x76693a6564616c67;
  extraout_RAX[1] = 0x635f6661656c5f79;
  *(undefined4 *)(extraout_RAX + 2) = 0x726f6c6f;
  asset_libraries::texture_library::AssetTextureLibrary::create_name_mapping();
  std::alloc::__default_lib_allocator::__rust_dealloc();
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  *(undefined8 *)((longlong)extraout_RAX_00 + 10) = 0x65776f6c665f7970;
  *(undefined8 *)((longlong)extraout_RAX_00 + 0x12) = 0x68747065645f7372;
  *extraout_RAX_00 = 0x61633a6564616c67;
  extraout_RAX_00[1] = 0x6f6c665f79706f6e;
  asset_libraries::texture_library::AssetTextureLibrary::create_name_mapping();
  std::alloc::__default_lib_allocator::__rust_dealloc();
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_01 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  *(undefined8 *)((longlong)extraout_RAX_01 + 10) = 0x6c6369747261705f;
  *(undefined8 *)((longlong)extraout_RAX_01 + 0x12) = 0x73726f6c6f635f65;
  *extraout_RAX_01 = 0x656c3a6564616c67;
  extraout_RAX_01[1] = 0x69747261705f6661;
  asset_libraries::texture_library::AssetTextureLibrary::create_name_mapping();
  std::alloc::__default_lib_allocator::__rust_dealloc();
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_02 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  *extraout_RAX_02 = 0x65743a6564616c67;
  extraout_RAX_02[1] = 0x65745f6e69617272;
  *(undefined8 *)((longlong)extraout_RAX_02 + 0xd) = 0x657275747865745f;
  asset_libraries::texture_library::AssetTextureLibrary::create_name_mapping();
  std::alloc::__default_lib_allocator::__rust_dealloc();
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_03 != (undefined8 *)0x0) {
    *extraout_RAX_03 = 0x72673a6564616c67;
    extraout_RAX_03[1] = 0x637461705f737361;
    *(undefined1 *)(extraout_RAX_03 + 2) = 0x68;
    asset_libraries::texture_library::AssetTextureLibrary::create_name_mapping();
    std::alloc::__default_lib_allocator::__rust_dealloc();
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_04 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    *(undefined8 *)((longlong)extraout_RAX_04 + 5) = 0x317265776f6c663a;
    *extraout_RAX_04 = 0x6c663a6564616c67;
    asset_libraries::texture_library::AssetTextureLibrary::create_name_mapping();
    std::alloc::__default_lib_allocator::__rust_dealloc();
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_05 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    *extraout_RAX_05 = 0x696c3a6564616c67;
    extraout_RAX_05[1] = 0x6c665f646170796c;
    *(undefined4 *)(extraout_RAX_05 + 2) = 0x7265776f;
    asset_libraries::texture_library::AssetTextureLibrary::create_name_mapping();
    std::alloc::__default_lib_allocator::__rust_dealloc();
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_06 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    *extraout_RAX_06 = 0x61633a6564616c67;
    extraout_RAX_06[1] = 0x6c6f635f79706f6e;
    *(undefined2 *)(extraout_RAX_06 + 2) = 0x726f;
    asset_libraries::texture_library::AssetTextureLibrary::create_name_mapping();
    std::alloc::__default_lib_allocator::__rust_dealloc();
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_07 != (undefined8 *)0x0) {
      *extraout_RAX_07 = 0x61623a6564616c67;
      extraout_RAX_07[1] = 0x726f6c6f635f6b72;
      *(undefined8 *)((longlong)extraout_RAX_07 + 0xe) = 0x64656b616e5f726f;
      asset_libraries::texture_library::AssetTextureLibrary::create_name_mapping();
      std::alloc::__default_lib_allocator::__rust_dealloc();
      core::ptr::drop_in_place<country_core::startup::load_textures::GladeTextureMappings>();
      if ((undefined **)local_48 != (undefined **)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (local_c0[0] != (undefined *)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      return;
    }
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}


