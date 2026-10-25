
// WARNING: Removing unreachable block (ram,0x000140944e6f)

void __cdecl country_core::systems::save::impl$1::add_save_load_events_and_resources(void)

{
  longlong lVar1;
  code *pcVar2;
  void *_Dst;
  void *_Dst_00;
  longlong *extraout_RAX;
  undefined8 *extraout_RAX_00;
  longlong extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong extraout_RAX_06;
  longlong in_RCX;
  ulonglong uVar3;
  undefined1 local_2f8 [8];
  undefined8 local_2f0;
  undefined8 local_2e0;
  undefined8 local_2d0;
  undefined8 local_2c0;
  undefined8 local_2b0;
  undefined8 uStack_2a8;
  undefined8 local_2a0;
  undefined8 uStack_298;
  undefined8 local_290;
  undefined8 uStack_288;
  undefined8 local_280;
  undefined8 uStack_278;
  undefined8 local_270;
  undefined8 local_268;
  undefined8 uStack_260;
  ulonglong local_258;
  undefined8 uStack_250;
  undefined8 local_248;
  undefined8 uStack_240;
  undefined1 local_238;
  undefined8 local_200;
  undefined8 uStack_1f8;
  ulonglong local_180;
  undefined8 uStack_178;
  undefined8 local_170;
  undefined8 local_168;
  undefined8 uStack_160;
  ulonglong local_158;
  undefined8 local_150;
  undefined1 local_148;
  undefined7 uStack_147;
  undefined4 local_140;
  undefined8 local_13c;
  undefined8 local_100;
  undefined8 local_f8;
  undefined1 local_f0;
  undefined1 *local_60;
  undefined8 local_50;
  
  local_60 = local_2f8;
  local_50 = 0xfffffffffffffffe;
  local_100 = 1;
  local_f8 = 1;
  local_f0 = 0;
  local_280 = 0;
  uStack_278 = 0;
  local_200 = 0;
  uStack_1f8 = 0;
  local_180 = local_180 & 0xffffffffffff0000;
  uStack_178 = 0;
  local_170 = 8;
  local_168 = 0;
  uStack_160 = 0;
  local_158 = 8;
  local_150 = 0;
  local_148 = 1;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (_Dst == (void *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  memcpy(_Dst,&local_280,0x200);
  local_2f0 = 1;
  local_2e0 = 1;
  local_100 = 1;
  local_f8 = 1;
  local_f0 = 0;
  local_280 = 0;
  uStack_278 = 0;
  local_200 = 0;
  uStack_1f8 = 0;
  local_180 = local_180 & 0xffffffffffff0000;
  uStack_178 = 0;
  local_170 = 8;
  local_168 = 0;
  uStack_160 = 0;
  local_158 = 8;
  local_150 = 0;
  _local_148 = CONCAT71(uStack_147,1);
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (_Dst_00 == (void *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  memcpy(_Dst_00,&local_280,0x200);
  local_2d0 = 1;
  local_2c0 = 1;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  *extraout_RAX = 1;
  extraout_RAX[1] = 1;
  *(undefined1 *)(extraout_RAX + 2) = 0;
  extraout_RAX[3] = 0;
  extraout_RAX[4] = 8;
  extraout_RAX[5] = 0;
  LOCK();
  lVar1 = *extraout_RAX;
  *extraout_RAX = *extraout_RAX + 1;
  UNLOCK();
  if (*extraout_RAX == 0 || SCARRY8(lVar1,1) != *extraout_RAX < 0) {
                    // WARNING: Does not return
    pcVar2 = (code *)invalidInstructionException();
    (*pcVar2)();
  }
  local_270 = 0x8000000000000000;
  local_280 = 0;
  local_258 = local_258 & 0xffffffffffffff00;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  *extraout_RAX_00 = 0x617320636e797341;
  *(undefined2 *)(extraout_RAX_00 + 1) = 0x6576;
  std::thread::Builder::name();
  local_280 = 1;
  local_270 = 1;
  std::thread::Builder::
  spawn<country_core::systems::save::impl_1::add_save_load_events_and_resources::closure_env_0,tuple_<>_>
            ();
  uVar3 = 0x4ab328be485143c6;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_01 == 0) || (*(ulonglong *)(in_RCX + 0x218) <= uVar3)) ||
      (uVar3 = *(ulonglong *)(*(longlong *)(in_RCX + 0x210) + uVar3 * 8), uVar3 == 0)) ||
     (*(longlong *)(*(longlong *)(in_RCX + 0x1e0) + 0x20 + ~uVar3 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::save::save_game::SaveGameCmd>();
  }
  local_2b0 = 0x8000000000000000;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_02 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  *(undefined4 *)(extraout_RAX_02 + 1) = 0;
  *extraout_RAX_02 = 0x8000000000000000;
  local_248 = local_290;
  uStack_240 = uStack_288;
  local_258 = local_2a0;
  uStack_250 = uStack_298;
  local_268 = local_2b0;
  uStack_260 = uStack_2a8;
  local_238 = 2;
  local_280 = 1;
  local_270 = 1;
  bevy_app::app::App::insert_resource<country_core::systems::save::load_game::LoadGameManager>();
  local_170 = 10;
  local_180 = 10;
  uStack_160 = 0;
  local_280 = 1;
  local_158 = local_158 & 0xffffffffffffff00;
  local_150 = 1;
  local_270 = 0;
  local_268 = 8;
  uStack_260 = 0;
  local_258 = 0x8000000000000000;
  local_140 = 0;
  local_13c = 0x501502f900000000;
  bevy_app::app::App::insert_resource<country_core::systems::save::save_game::AutoSaveState>();
  uVar3 = 0x4d00324fb817a8a1;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_04 == 0) || (*(ulonglong *)(extraout_RAX_03 + 0x218) <= uVar3)) ||
     ((uVar3 = *(ulonglong *)(*(longlong *)(extraout_RAX_03 + 0x210) + uVar3 * 8), uVar3 == 0 ||
      (*(longlong *)(*(longlong *)(extraout_RAX_03 + 0x1e0) + 0x20 + ~uVar3 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::save::load_game::LoadingGameComplete>();
  }
  uVar3 = 0x400333187b2ad3ac;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_05 == 0) || (*(ulonglong *)(extraout_RAX_03 + 0x218) <= uVar3)) ||
     ((uVar3 = *(ulonglong *)(*(longlong *)(extraout_RAX_03 + 0x210) + uVar3 * 8), uVar3 == 0 ||
      (*(longlong *)(*(longlong *)(extraout_RAX_03 + 0x1e0) + 0x20 + ~uVar3 * 0x60) == 0)))) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::save::save_game::SavingGameComplete>();
  }
  uVar3 = 0xeb08402348b616b;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_06 == 0) || (*(ulonglong *)(extraout_RAX_03 + 0x218) <= uVar3)) ||
      (uVar3 = *(ulonglong *)(*(longlong *)(extraout_RAX_03 + 0x210) + uVar3 * 8), uVar3 == 0)) ||
     (*(longlong *)(*(longlong *)(extraout_RAX_03 + 0x1e0) + 0x20 + ~uVar3 * 0x60) == 0)) {
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::game_profile::SaveGameProfileCmd>();
  }
  return;
}

