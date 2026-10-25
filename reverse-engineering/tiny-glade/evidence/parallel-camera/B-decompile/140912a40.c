
void __cdecl country_core::systems::main_camera::reset_camera(void)

{
  void *pvVar1;
  longlong *plVar2;
  longlong lVar3;
  longlong lVar4;
  undefined8 uVar5;
  undefined8 *extraout_RAX;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  longlong *in_R8;
  undefined8 local_518;
  undefined *local_510;
  undefined8 local_508;
  undefined8 local_500;
  undefined8 local_4f8;
  undefined8 uStack_4f0;
  undefined8 local_4e8;
  undefined *local_4e0;
  undefined8 local_4d8;
  undefined4 local_4d0;
  undefined4 local_4cc;
  undefined **local_4c8;
  undefined8 local_4c0;
  undefined8 local_4b8;
  undefined8 local_4b0;
  undefined8 uStack_4a8;
  void *local_38;
  undefined8 local_28;
  
  local_28 = 0xfffffffffffffffe;
  if (4 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    local_4f8 = *extraout_RAX;
    uStack_4f0 = extraout_RAX[1];
    local_4cc = *(undefined4 *)(extraout_RAX + 2);
    local_4e8 = 5;
    local_4e0 = &anon_434755572596c75c61f150b2c3bca76e_52_llvm_17334014131709226316;
    local_4d8 = 0x22;
    local_4c8 = &anon_434755572596c75c61f150b2c3bca76e_50_llvm_17334014131709226316;
    local_4c0 = 1;
    local_4b8 = 8;
    local_518 = 0;
    local_4b0 = 0;
    uStack_4a8 = 0;
    local_510 = &anon_434755572596c75c61f150b2c3bca76e_52_llvm_17334014131709226316;
    local_508 = 0x22;
    local_500 = 0;
    local_4d0 = 1;
    ::log::__private_api::impl$2::log();
  }
  *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
  pvVar1 = (void *)*in_RCX;
  camera_rig::MainCamera::new();
  local_38 = pvVar1;
  core::ptr::drop_in_place<country_core::systems::camera_rig::MainCamera>();
  memcpy(local_38,&local_518,0x4e0);
  *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
  plVar2 = (longlong *)*in_RDX;
  if (*plVar2 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  *plVar2 = 0;
  plVar2[1] = 4;
  plVar2[2] = 0;
  *(undefined4 *)(plVar2 + 3) = 0;
  plVar2[4] = 0;
  *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
  lVar3 = *in_R8;
  lVar4 = *(longlong *)(lVar3 + 0x30);
  uVar5 = *(undefined8 *)(lVar3 + 0x40);
  if (lVar4 == *(longlong *)(lVar3 + 0x20)) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
              ();
  }
  *(undefined8 *)(*(longlong *)(lVar3 + 0x28) + lVar4 * 8) = uVar5;
  *(longlong *)(lVar3 + 0x30) = lVar4 + 1;
  *(longlong *)(lVar3 + 0x40) = *(longlong *)(lVar3 + 0x40) + 1;
  return;
}

