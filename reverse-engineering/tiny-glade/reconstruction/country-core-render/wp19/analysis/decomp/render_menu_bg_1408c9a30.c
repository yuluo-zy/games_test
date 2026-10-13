// render_menu_bg  0x1408c9a30
// signature: void __cdecl render_menu_bg(void)
// callees: [{"name": "acquire_resource", "address": "140c81c70"}, {"name": "bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture>_>", "address": "1408fd6c0"}, {"name": "call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64>_>_>_>_>_>_>_>_>", "address": "1400d9b20"}, {"name": "default", "address": "140be6750"}, {"name": "draw_no_vao", "address": "140906a90"}, {"name": "drop", "address": "140be24b0"}, {"name": "extent_2d", "address": "140bd5790"}, {"name": "finish<country_core::systems::main_camera::MainCameraPlugin>", "address": "14003bb80"}, {"name": "get_raster_handle_by_name", "address": "140937aa0"}, {"name": "get_resource_id", "address": "1400dd6f0"}, {"name": "lock_slow", "address": "1428cd210"}, {"name": "memcpy", "address": "142923c00"}, {"name": "new", "address": "140905be0"}, {"name": "new_impl", "address": "140867970"}, {"name": "panic_access_error", "address": "1428d80d0"}, {"name": "release_resource", "address": "140c81e10"}, {"name": "render", "address": "140868370"}, {"name": "unlock_slow", "address": "1428cdb90"}, {"name": "unwrap_failed", "address": "1428d9310"}]


void __cdecl country_core::systems::render_loop::menu_background::render_menu_bg(void)

{
  longlong *plVar1;
  longlong lVar2;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  ulonglong extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong lVar3;
  longlong in_RDX;
  ulonglong uVar4;
  undefined *puVar5;
  bool bVar6;
  longlong local_938;
  longlong lStack_930;
  longlong local_928;
  longlong lStack_920;
  longlong local_918;
  longlong lStack_910;
  undefined8 local_908;
  longlong local_548;
  longlong lStack_540;
  longlong local_538;
  longlong lStack_530;
  longlong local_528;
  longlong lStack_520;
  longlong local_518;
  undefined8 local_510;
  undefined8 local_508;
  undefined *local_4d8;
  undefined8 uStack_4d0;
  undefined8 local_4c8;
  undefined8 uStack_4c0;
  longlong local_4b8;
  longlong lStack_4b0;
  undefined *local_4a8;
  uint local_4a0;
  undefined4 uStack_49c;
  undefined1 uStack_498;
  undefined1 uStack_497;
  undefined7 uStack_496;
  undefined8 local_48f;
  undefined8 uStack_487;
  undefined8 local_47f;
  undefined8 uStack_477;
  undefined8 local_46f;
  undefined8 uStack_458;
  undefined4 local_160;
  undefined4 uStack_15c;
  undefined1 local_158 [160];
  undefined1 local_b8;
  undefined8 local_b7;
  undefined8 uStack_af;
  undefined8 local_a7;
  undefined8 uStack_9f;
  undefined8 local_97;
  undefined8 uStack_8f;
  undefined8 local_87;
  undefined8 uStack_70;
  longlong *local_68;
  longlong *local_58;
  undefined1 local_4a;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  uVar4 = 0xd38dae617fb79c7a;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX != 0) && (uVar4 < *(ulonglong *)(in_RDX + 0x218))) &&
     (uVar4 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar4 * 8), uVar4 != 0)) {
    lVar3 = ~uVar4 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar3) != 0) {
      plVar1 = *(longlong **)(*(longlong *)(in_RDX + 0x1e0) + lVar3 + 0x10);
      lVar3 = *plVar1;
      if ((lVar3 != 2) && (((int)lVar3 != 10 || ((char)plVar1[1] == '\x01')))) {
        uVar4 = 4;
        if (lVar3 - 2U < 9) {
          uVar4 = lVar3 - 2U;
        }
        if ((((0xafUL >> (uVar4 & 0x3f) & 1) != 0) ||
            (((0x50UL >> (uVar4 & 0x3f) & 1) == 0 && ((char)plVar1[1] == '\0')))) &&
           (((int)lVar3 != 7 || ((char)plVar1[1] != '\x01')))) {
          uVar4 = 0xfb32e4dcc8287bca;
          bevy_ecs::component::Components::get_resource_id();
          if (((extraout_RAX_00 != 0) && (uVar4 < *(ulonglong *)(in_RDX + 0x218))) &&
             (uVar4 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar4 * 8), uVar4 != 0)) {
            lVar3 = ~uVar4 * 0x60;
            if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar3) != 0) {
              lVar3 = *(longlong *)(in_RDX + 0x1e0) + lVar3;
              plVar1 = *(longlong **)(lVar3 + 0x10);
              local_58 = (longlong *)CONCAT44(local_58._4_4_,*(undefined4 *)(in_RDX + 0x5b0));
              lVar2 = *plVar1;
              LOCK();
              bVar6 = *(char *)(lVar2 + 0x10) == '\0';
              if (bVar6) {
                *(char *)(lVar2 + 0x10) = '\x01';
              }
              UNLOCK();
              if (!bVar6) {
                parking_lot::raw_mutex::RawMutex::lock_slow();
              }
              parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
              local_908 = *(longlong *)(lVar2 + 0x48);
              local_938 = *(longlong *)(lVar2 + 0x18);
              lStack_930 = *(longlong *)(lVar2 + 0x20);
              local_928 = *(longlong *)(lVar2 + 0x28);
              lStack_920 = *(longlong *)(lVar2 + 0x30);
              local_918 = *(longlong *)(lVar2 + 0x38);
              lStack_910 = *(longlong *)(lVar2 + 0x40);
              *(undefined1 *)(lVar2 + 0x4e) = 7;
              local_4a = 1;
              parking_lot_core::parking_lot::deadlock_impl::release_resource();
              LOCK();
              bVar6 = *(char *)(lVar2 + 0x10) == '\x01';
              if (bVar6) {
                *(char *)(lVar2 + 0x10) = '\0';
              }
              UNLOCK();
              if (!bVar6) {
                local_4a = 1;
                parking_lot::raw_mutex::RawMutex::unlock_slow();
              }
              if (local_908._6_1_ != '\a') {
                local_518 = local_908;
                local_528 = local_918;
                lStack_520 = lStack_910;
                local_538 = local_928;
                lStack_530 = lStack_920;
                local_548 = local_938;
                lStack_540 = lStack_930;
                *(undefined4 *)(lVar3 + 0x5c) = local_58._0_4_;
                local_58 = plVar1 + 1;
                if (*(char *)((longlong)plVar1 + 0x3e) != '\a') {
                  bevy_app::plugin::Plugin::
                  finish<country_core::systems::main_camera::MainCameraPlugin>();
                  rhapsody::raw::managed::handle::impl$6::drop();
                }
                local_58[6] = local_518;
                local_58[4] = local_528;
                local_58[5] = lStack_520;
                local_58[2] = local_538;
                local_58[3] = lStack_530;
                *local_58 = local_548;
                local_58[1] = lStack_540;
              }
              if (*(char *)((longlong)plVar1 + 0x3e) == '\a') {
                return;
              }
              uVar4 = 0x8597ee0727b461b3;
              bevy_ecs::component::Components::get_resource_id();
              if (((extraout_RAX_01 == 0) || (*(ulonglong *)(in_RDX + 0x218) <= uVar4)) ||
                 ((uVar4 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar4 * 8), uVar4 == 0 ||
                  (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + ~uVar4 * 0x60) == 0)))) {
                    // WARNING: Subroutine does not return
                core::option::unwrap_failed();
              }
              puVar5 = &DAT_142aa31e0;
              asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
              if ((extraout_RAX_02 & 1) == 0) {
                    // WARNING: Subroutine does not return
                core::option::unwrap_failed();
              }
              rhapsody::raw::texture::TextureDesc::extent_2d();
              local_548 = CONCAT44(local_548._4_4_,2);
              local_160 = 2;
              country_core::render::pass_builder::draw_stream::RasterPass::new_impl();
              local_49 = 1;
              rhapsody::raw::shader::impl$7::default();
              local_49 = 1;
              core::ops::function::FnOnce::
              call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                        ();
              if (extraout_RAX_03 != (longlong *)0x0) {
                uStack_498 = 0;
                uStack_497 = 0;
                uStack_49c = 0x3f800000;
                local_548 = 0;
                local_538 = 0;
                lStack_530 = 4;
                local_528 = 0;
                local_4b8 = *extraout_RAX_03;
                lStack_4b0 = extraout_RAX_03[1];
                *extraout_RAX_03 = *extraout_RAX_03 + 1;
                local_4c8 = 0;
                uStack_4c0 = 0;
                local_4d8 = &anon_65ceffb69360efd323e48f4f8756afaa_2_llvm_13316938065337964131;
                uStack_4d0 = 0;
                local_508 = 0x100000003;
                lStack_520 = 0;
                local_518 = 1;
                local_510 = 0;
                local_49 = 1;
                local_4a8 = puVar5;
                country_core::render::pass_builder::draw_builder::DrawBuilder::new();
                local_68 = plVar1 + 1;
                local_548 = CONCAT44(local_548._4_4_,2);
                country_core::render::pass_builder::draw_builder::impl$3::
                bind_texture_to_draw_builder<ref_<rhapsody::raw::texture::Texture>_>();
                lVar3 = CONCAT44(uStack_15c,local_160);
                memcpy(&lStack_540,local_158,0xa0);
                local_4a0._1_3_ = (undefined3)local_b7;
                uStack_49c = (undefined4)((ulonglong)local_b7 >> 0x18);
                uStack_498 = (undefined1)((ulonglong)local_b7 >> 0x38);
                uStack_497 = (undefined1)uStack_af;
                uStack_496 = (undefined7)((ulonglong)uStack_af >> 8);
                local_48f = local_a7;
                uStack_487 = uStack_9f;
                local_47f = local_97;
                uStack_477 = uStack_8f;
                local_46f = local_87;
                uStack_458 = uStack_70;
                local_4a0 = CONCAT31(local_4a0._1_3_,local_b8) & 0xffffff7f;
                local_49 = 1;
                local_548 = lVar3;
                country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao();
                memcpy(&local_548,&local_938,1000);
                local_49 = 0;
                country_core::render::pass_builder::draw_stream::RasterPass::render();
                return;
              }
              local_49 = 1;
                    // WARNING: Subroutine does not return
              std::thread::local::panic_access_error();
            }
          }
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
      }
      return;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

