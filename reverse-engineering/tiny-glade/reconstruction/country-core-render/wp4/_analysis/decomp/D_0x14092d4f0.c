// ADDRESS: 0x14092d4f0
// SIZE: 119
// SIGNATURE: void __cdecl get_handle_by_name(void)

void __cdecl country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_handle_by_name(void)

{
  ulonglong extraout_RAX;
  undefined1 local_50 [16];
  undefined1 *local_40;
  code *local_38;
  undefined **local_30;
  undefined8 local_28;
  undefined1 **local_20;
  undefined8 local_18;
  undefined8 local_10;
  
  try_get_handle_by_name();
  if ((extraout_RAX & 1) != 0) {
    return;
  }
  local_40 = local_50;
  local_38 = core::fmt::impl$74::fmt<str_>;
  local_30 = &anon_03061c64e98a3a4ea7442d952ee9ce0d_45_llvm_2292516692454265093;
  local_28 = 2;
  local_10 = 0;
  local_20 = &local_40;
  local_18 = 1;
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}

