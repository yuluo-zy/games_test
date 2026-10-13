// addr=0x140b1aca0 name=load_mesh_into_library
// signature=void __cdecl load_mesh_into_library(void)


void __cdecl country_core::startup::load_mesh_into_library(void)

{
  void *extraout_RAX;
  undefined8 *in_RCX;
  void *in_RDX;
  size_t in_R8;
  undefined1 local_168 [144];
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 local_60;
  void *local_58;
  undefined8 *local_40;
  undefined1 local_31;
  undefined8 local_30;
  
  local_30 = 0xfffffffffffffffe;
  local_31 = 1;
  local_40 = in_RCX;
  asset_libraries::meshwatch::MeshWatch::watch();
  local_68 = local_40[0xe];
  local_78 = local_40[0xc];
  uStack_70 = local_40[0xd];
  local_88 = local_40[10];
  uStack_80 = local_40[0xb];
  local_98 = local_40[8];
  uStack_90 = local_40[9];
  local_d8 = *local_40;
  uStack_d0 = local_40[1];
  local_c8 = local_40[2];
  uStack_c0 = local_40[3];
  local_b8 = local_40[4];
  uStack_b0 = local_40[5];
  local_a8 = local_40[6];
  uStack_a0 = local_40[7];
  local_60 = 0x8000000000000000;
  if ((longlong)in_R8 < 0) {
LAB_140b1add7:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  if (in_R8 == 0) {
    local_58 = (void *)0x1;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    local_58 = extraout_RAX;
    if (extraout_RAX == (void *)0x0) goto LAB_140b1add7;
  }
  memcpy(local_58,in_RDX,in_R8);
  memcpy(local_168,&local_d8,0x90);
  local_31 = 0;
  asset_libraries::mesh_library::AssetMeshLibrary::add();
  return;
}


