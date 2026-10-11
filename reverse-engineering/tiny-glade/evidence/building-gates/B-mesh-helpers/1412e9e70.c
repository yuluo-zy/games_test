
void __cdecl
country_core::components::transient_mesh::TransientMeshRegistry::
drain_meshes_to_delete<country_core::systems::backend::transient_mesh::delete_dropped_transient_meshes::closure_env_0>
          (void)

{
  char cVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  longlong lVar4;
  undefined4 *puVar5;
  undefined4 *puVar6;
  byte extraout_AL;
  byte bVar7;
  char extraout_AL_00;
  char *in_RCX;
  longlong in_RDX;
  longlong in_R8;
  longlong lVar8;
  
  LOCK();
  cVar1 = *in_RCX;
  if (cVar1 == '\0') {
    *in_RCX = '\x01';
  }
  UNLOCK();
  if (cVar1 != '\0') {
    std::sys::sync::mutex::futex::Mutex::lock_contended();
  }
  if ((*(ulonglong *)__imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E &
      0x7fffffffffffffff) == 0) {
    bVar7 = 0;
    cVar1 = in_RCX[1];
  }
  else {
    std::panicking::panic_count::is_zero_slow_path();
    bVar7 = extraout_AL ^ 1;
    cVar1 = in_RCX[1];
  }
  if (cVar1 != '\0') {
                    /* WARNING: Subroutine does not return */
    core::result::unwrap_failed();
  }
  lVar4 = *(longlong *)(in_RCX + 0x18);
  if (lVar4 != 0) {
    uVar2 = *(undefined4 *)(in_RDX + 0x1c);
    puVar5 = *(undefined4 **)(in_RDX + 0x10);
    uVar3 = *(undefined4 *)(in_R8 + 0x1c);
    puVar6 = *(undefined4 **)(in_R8 + 0x10);
    lVar8 = 0;
    do {
      if (4 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        ::log::__private_api::impl$2::log();
      }
      *puVar5 = uVar2;
      asset_libraries::mesh_library::AssetMeshLibrary::remove();
      *puVar6 = uVar3;
      asset_libraries::vao_library::AssetVAOLibrary::remove();
      lVar8 = lVar8 + 8;
    } while (lVar4 << 3 != lVar8);
  }
  in_RCX[0x18] = '\0';
  in_RCX[0x19] = '\0';
  in_RCX[0x1a] = '\0';
  in_RCX[0x1b] = '\0';
  in_RCX[0x1c] = '\0';
  in_RCX[0x1d] = '\0';
  in_RCX[0x1e] = '\0';
  in_RCX[0x1f] = '\0';
  if (((bVar7 == 0) &&
      ((*(ulonglong *)__imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E &
       0x7fffffffffffffff) != 0)) &&
     (std::panicking::panic_count::is_zero_slow_path(), extraout_AL_00 == '\0')) {
    in_RCX[1] = '\x01';
  }
  LOCK();
  cVar1 = *in_RCX;
  *in_RCX = '\0';
  UNLOCK();
  if (cVar1 == '\x02') {
    std::sys::sync::mutex::futex::Mutex::wake();
  }
  return;
}

