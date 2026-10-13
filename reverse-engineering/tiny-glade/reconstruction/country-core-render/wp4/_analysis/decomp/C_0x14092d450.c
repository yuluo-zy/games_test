// ADDRESS: 0x14092d450
// SIZE: 144
// SIGNATURE: void __cdecl get_by_name(void)

void __cdecl country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_by_name(void)

{
  ulonglong extraout_RAX;
  undefined1 local_58 [16];
  undefined1 *local_48;
  code *local_40;
  undefined **local_38;
  undefined8 local_30;
  undefined1 **local_28;
  undefined8 local_20;
  undefined8 local_18;
  
  try_get_handle_by_name();
  if ((extraout_RAX & 1) != 0) {
    frozen::FrozenMap::
    get<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>_>
              ();
    return;
  }
  local_48 = local_58;
  local_40 = core::fmt::impl$74::fmt<str_>;
  local_38 = &anon_03061c64e98a3a4ea7442d952ee9ce0d_45_llvm_2292516692454265093;
  local_30 = 2;
  local_18 = 0;
  local_28 = &local_48;
  local_20 = 1;
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}

