// 0x14098c680 read_terrain_data size=4998

void __cdecl
country_core::systems::render_loop::compute_passes::read_terrain_data::read_terrain_data(void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  longlong *plVar3;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  longlong extraout_RAX_03;
  undefined8 *extraout_RAX_04;
  longlong extraout_RAX_05;
  undefined8 *extraout_RAX_06;
  longlong extraout_RAX_07;
  undefined4 *extraout_RAX_08;
  longlong *extraout_RAX_09;
  ulonglong uVar4;
  longlong extraout_RAX_10;
  undefined8 *extraout_RAX_11;
  longlong extraout_RAX_12;
  undefined8 *extraout_RAX_13;
  longlong extraout_RAX_14;
  undefined8 *extraout_RAX_15;
  longlong extraout_RAX_16;
  undefined4 *extraout_RAX_17;
  longlong *extraout_RAX_18;
  longlong extraout_RAX_19;
  longlong lVar5;
  longlong in_RDX;
  ulonglong uVar6;
  undefined4 extraout_XMM0_Da;
  undefined4 extraout_XMM0_Da_00;
  undefined4 extraout_XMM0_Da_01;
  undefined4 extraout_XMM0_Da_02;
  ulonglong local_a58;
  undefined8 uStack_a50;
  undefined8 local_a48;
  undefined8 local_a40;
  undefined8 uStack_a38;
  undefined8 local_a30;
  undefined8 local_a28;
  undefined8 uStack_a20;
  undefined8 local_a18;
  undefined8 local_a10;
  undefined8 uStack_a08;
  undefined8 local_a00;
  undefined8 local_9f8;
  undefined8 uStack_9f0;
  undefined8 local_9e8;
  undefined8 uStack_9e0;
  undefined8 local_9d8;
  undefined8 uStack_9d0;
  undefined8 local_9c8;
  undefined8 uStack_9c0;
  undefined8 local_9b8;
  undefined8 uStack_9b0;
  undefined8 local_9a8;
  undefined8 uStack_9a0;
  undefined8 local_998;
  undefined8 uStack_990;
  undefined8 local_988;
  undefined8 uStack_980;
  undefined8 local_978;
  undefined8 uStack_970;
  undefined8 local_968;
  undefined8 uStack_960;
  undefined8 local_958;
  undefined8 uStack_950;
  char local_922;
  undefined8 local_920;
  undefined8 local_8d8;
  undefined1 local_66c [36];
  ulonglong local_648;
  undefined1 *puStack_640;
  undefined8 local_638;
  undefined8 local_630;
  undefined8 uStack_628;
  undefined8 local_620;
  undefined8 local_618;
  undefined8 uStack_610;
  undefined8 local_608;
  undefined8 local_600;
  undefined8 uStack_5f8;
  undefined8 local_5f0;
  undefined8 local_5e8;
  undefined8 uStack_5e0;
  undefined8 uStack_5d8;
  undefined8 local_5d0;
  undefined8 uStack_5c8;
  undefined8 local_5c0;
  undefined8 uStack_5b8;
  undefined8 local_5b0;
  undefined8 uStack_5a8;
  undefined8 local_5a0;
  undefined8 uStack_598;
  undefined8 local_590;
  undefined8 uStack_588;
  undefined8 local_580;
  undefined8 uStack_578;
  undefined8 local_570;
  undefined8 local_568;
  undefined8 uStack_560;
  undefined8 local_558;
  undefined8 uStack_550;
  undefined8 local_548;
  undefined8 uStack_540;
  char local_512;
  undefined8 local_510;
  undefined8 local_4c8;
  undefined8 local_258;
  ulonglong uStack_250;
  char *local_248;
  undefined8 uStack_240;
  undefined8 local_238;
  undefined8 uStack_230;
  undefined8 local_228;
  undefined8 uStack_220;
  undefined8 local_218;
  undefined8 uStack_210;
  undefined8 local_208;
  undefined8 local_200;
  undefined4 local_1f8;
  undefined1 local_1f4;
  undefined *local_1e8;
  undefined8 uStack_1e0;
  undefined8 local_1d8;
  undefined8 uStack_1d0;
  longlong local_1c8;
  longlong lStack_1c0;
  undefined8 local_1b8;
  undefined4 local_1ac;
  undefined2 local_1a8;
  ulonglong *local_c0;
  undefined4 local_b0;
  undefined4 uStack_ac;
  ulonglong uStack_a8;
  undefined4 local_a0;
  undefined4 uStack_9c;
  ulonglong uStack_98;
  undefined8 *local_90;
  ulonglong local_88;
  longlong local_80;
  undefined1 local_71;
  undefined8 local_70;
  
  local_70 = 0xfffffffffffffffe;
  local_80 = in_RDX + 0x38;
  uVar6 = 0xa68561b396784eaa;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX == 0) || (*(ulonglong *)(in_RDX + 0x218) <= uVar6)) ||
      (uVar6 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar6 * 8), uVar6 == 0)) ||
     (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + ~uVar6 * 0x60) == 0)) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  asset_libraries::texture_library::AssetTextureLibrary::get_rg_texture();
  rhapsody::raw::texture::TextureDesc::extent_2d();
  local_648 = 0;
  local_638 = 1;
  puStack_640 = local_66c;
  country_core::render::pass_builder::draw_stream::RasterPass::new_impl();
  local_71 = 1;
  uVar6 = 0xd3bda3878f7a9d01;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_00 != 0) && (uVar6 < *(ulonglong *)(in_RDX + 0x218))) &&
     (uVar6 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar6 * 8), uVar6 != 0)) {
    lVar5 = ~uVar6 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar5) != 0) {
      local_c0 = *(ulonglong **)(*(longlong *)(in_RDX + 0x1e0) + lVar5 + 0x10);
      local_88 = *local_c0;
      uVar6 = local_88 & 0xffffffff;
      if (((*(ulonglong *)(in_RDX + 0x10) <= uVar6) ||
          (*(int *)(*(longlong *)(in_RDX + 8) + uVar6 * 0x14) != (int)(local_88 >> 0x20))) ||
         (lVar5 = *(longlong *)(in_RDX + 8) + uVar6 * 0x14, *(int *)(lVar5 + 4) == -1)) {
        local_71 = 1;
        local_648 = local_88;
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      uVar6 = (ulonglong)*(uint *)(lVar5 + 0xc);
      local_71 = 1;
      bevy_ecs::component::Components::get_id();
      if ((extraout_RAX_01 != 0) && (uVar6 < *(ulonglong *)(in_RDX + 0x1b0))) {
        local_71 = 1;
        bevy_ecs::storage::table::Table::get_component();
        if (extraout_RAX_02 != (undefined8 *)0x0) {
          local_71 = 1;
          local_90 = extraout_RAX_02;
          bevy_ecs::component::Components::get_id();
          if ((extraout_RAX_03 != 0) && (uVar6 < *(ulonglong *)(in_RDX + 0x1b0))) {
            local_71 = 1;
            bevy_ecs::storage::table::Table::get_component();
            if (extraout_RAX_04 != (undefined8 *)0x0) {
              local_71 = 1;
              bevy_ecs::component::Components::get_id();
              if ((extraout_RAX_05 != 0) && (uVar6 < *(ulonglong *)(in_RDX + 0x1b0))) {
                local_71 = 1;
                bevy_ecs::storage::table::Table::get_component();
                if (extraout_RAX_06 != (undefined8 *)0x0) {
                  local_71 = 1;
                  bevy_ecs::component::Components::get_id();
                  local_a0 = 2;
                  uStack_9c = extraout_XMM0_Da;
                  if ((extraout_RAX_07 != 0) && (uVar6 < *(ulonglong *)(in_RDX + 0x1b0))) {
                    local_71 = 1;
                    bevy_ecs::storage::table::Table::get_component();
                    uStack_9c = extraout_XMM0_Da_00;
                    if (extraout_RAX_08 != (undefined4 *)0x0) {
                      local_a0 = *extraout_RAX_08;
                      uStack_9c = extraout_RAX_08[1];
                    }
                  }
                  uStack_98 = local_88;
                  uVar1 = *extraout_RAX_04;
                  uVar2 = *local_90;
                  local_71 = 1;
                  rhapsody::raw::shader::impl$7::default();
                  local_71 = 1;
                  core::ops::function::FnOnce::
                  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                            ();
                  if (extraout_RAX_09 == (longlong *)0x0) {
                    local_71 = 1;
                    // WARNING: Subroutine does not return
                    std::thread::local::panic_access_error();
                  }
                  local_1a8 = 0;
                  local_1ac = 0x3f800000;
                  local_258 = 1;
                  local_248 = (char *)0x0;
                  uStack_240 = 4;
                  local_238 = 0;
                  local_1c8 = *extraout_RAX_09;
                  lStack_1c0 = extraout_RAX_09[1];
                  *extraout_RAX_09 = *extraout_RAX_09 + 1;
                  local_1d8 = 0;
                  uStack_1d0 = 0;
                  local_1e8 = &anon_65ceffb69360efd323e48f4f8756afaa_2_llvm_13316938065337964131;
                  uStack_1e0 = 0;
                  local_218 = 0x100000003;
                  uStack_230 = 0;
                  local_228 = 1;
                  uStack_220 = 0;
                  local_71 = 1;
                  uStack_250 = uVar2;
                  local_1b8 = uVar1;
                  country_core::render::pass_builder::draw_builder::DrawBuilder::new();
                  local_208 = *local_90;
                  local_200 = *extraout_RAX_04;
                  local_238 = *extraout_RAX_06;
                  uStack_230 = extraout_RAX_06[1];
                  local_228 = extraout_RAX_06[2];
                  uStack_220 = extraout_RAX_06[3];
                  local_218 = extraout_RAX_06[4];
                  uStack_210 = extraout_RAX_06[5];
                  local_258 = CONCAT44(uStack_9c,local_a0);
                  uStack_250 = uStack_98;
                  local_1f4 = 0;
                  local_248 = "terrain_shader";
                  uStack_240 = 0xe;
                  local_1f8 = 0;
                  local_71 = 1;
                  country_core::render::pass_builder::draw_builder::DrawBuilder::bind_common();
                  local_71 = 1;
                  draw_passes::draw_shaders::floor_shader::draw_floor_shader();
                  uVar6 = local_c0[1];
                  uVar4 = uVar6 & 0xffffffff;
                  if (((*(ulonglong *)(in_RDX + 0x10) <= uVar4) ||
                      (*(int *)(*(longlong *)(in_RDX + 8) + uVar4 * 0x14) != (int)(uVar6 >> 0x20)))
                     || (lVar5 = *(longlong *)(in_RDX + 8) + uVar4 * 0x14, *(int *)(lVar5 + 4) == -1
                        )) {
                    local_71 = 1;
                    local_648 = uVar6;
                    // WARNING: Subroutine does not return
                    core::result::unwrap_failed();
                  }
                  uVar4 = (ulonglong)*(uint *)(lVar5 + 0xc);
                  local_71 = 1;
                  local_88 = uVar6;
                  bevy_ecs::component::Components::get_id();
                  if ((extraout_RAX_10 != 0) && (uVar4 < *(ulonglong *)(in_RDX + 0x1b0))) {
                    local_71 = 1;
                    bevy_ecs::storage::table::Table::get_component();
                    if (extraout_RAX_11 != (undefined8 *)0x0) {
                      local_71 = 1;
                      local_90 = extraout_RAX_11;
                      bevy_ecs::component::Components::get_id();
                      if ((extraout_RAX_12 != 0) && (uVar4 < *(ulonglong *)(in_RDX + 0x1b0))) {
                        local_71 = 1;
                        bevy_ecs::storage::table::Table::get_component();
                        if (extraout_RAX_13 != (undefined8 *)0x0) {
                          local_71 = 1;
                          bevy_ecs::component::Components::get_id();
                          if ((extraout_RAX_14 != 0) && (uVar4 < *(ulonglong *)(in_RDX + 0x1b0))) {
                            local_71 = 1;
                            bevy_ecs::storage::table::Table::get_component();
                            if (extraout_RAX_15 != (undefined8 *)0x0) {
                              local_71 = 1;
                              bevy_ecs::component::Components::get_id();
                              local_b0 = 2;
                              uStack_ac = extraout_XMM0_Da_01;
                              if ((extraout_RAX_16 != 0) && (uVar4 < *(ulonglong *)(in_RDX + 0x1b0))
                                 ) {
                                local_71 = 1;
                                bevy_ecs::storage::table::Table::get_component();
                                uStack_ac = extraout_XMM0_Da_02;
                                if (extraout_RAX_17 != (undefined4 *)0x0) {
                                  local_b0 = *extraout_RAX_17;
                                  uStack_ac = extraout_RAX_17[1];
                                }
                              }
                              uStack_a8 = local_88;
                              uVar1 = *extraout_RAX_13;
                              uVar2 = *local_90;
                              local_71 = 1;
                              rhapsody::raw::shader::impl$7::default();
                              local_71 = 1;
                              core::ops::function::FnOnce::
                              call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                                        ();
                              if (extraout_RAX_18 == (longlong *)0x0) {
                                local_71 = 1;
                    // WARNING: Subroutine does not return
                                std::thread::local::panic_access_error();
                              }
                              local_1a8 = 0;
                              local_1ac = 0x3f800000;
                              local_258 = 1;
                              local_248 = (char *)0x0;
                              uStack_240 = 4;
                              local_238 = 0;
                              local_1c8 = *extraout_RAX_18;
                              lStack_1c0 = extraout_RAX_18[1];
                              *extraout_RAX_18 = *extraout_RAX_18 + 1;
                              local_1d8 = 0;
                              uStack_1d0 = 0;
                              local_1e8 = &
                                          anon_65ceffb69360efd323e48f4f8756afaa_2_llvm_13316938065337964131
                              ;
                              uStack_1e0 = 0;
                              local_218 = 0x100000003;
                              uStack_230 = 0;
                              local_228 = 1;
                              uStack_220 = 0;
                              local_71 = 1;
                              uStack_250 = uVar2;
                              local_1b8 = uVar1;
                              country_core::render::pass_builder::draw_builder::DrawBuilder::new();
                              local_208 = *local_90;
                              local_200 = *extraout_RAX_13;
                              local_238 = *extraout_RAX_15;
                              uStack_230 = extraout_RAX_15[1];
                              local_228 = extraout_RAX_15[2];
                              uStack_220 = extraout_RAX_15[3];
                              local_218 = extraout_RAX_15[4];
                              uStack_210 = extraout_RAX_15[5];
                              local_258 = CONCAT44(uStack_ac,local_b0);
                              uStack_250 = uStack_a8;
                              local_1f4 = 0;
                              local_248 = "rocky_terrain";
                              uStack_240 = 0xd;
                              local_1f8 = 0;
                              local_71 = 1;
                              country_core::render::pass_builder::draw_builder::DrawBuilder::
                              bind_common();
                              local_71 = 1;
                              draw_passes::draw_shaders::rocky_terrain_draw::rocky_terrain_shader();
                              memcpy(&local_648,&local_a58,1000);
                              local_71 = 0;
                              country_core::render::pass_builder::draw_stream::RasterPass::render();
                              country_core::render::pass_builder::compute_pass_builder::ComputePass
                              ::new<ref_<str_>_>();
                              local_258 = CONCAT71(local_258._1_7_,9);
                              local_648 = CONCAT44(local_648._4_4_,2);
                              country_core::render::pass_builder::compute_pass_builder::impl$4::
                              bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
                                        ();
                              memcpy(&local_648,&local_a58,400);
                              local_66c[0] = 10;
                              local_258 = CONCAT44(local_258._4_4_,2);
                              country_core::render::pass_builder::compute_pass_builder::impl$4::
                              bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
                                        ();
                              memcpy(&local_258,&local_648,400);
                              country_core::render::pass_builder::compute_pass_builder::ComputePass
                              ::
                              write_storage_buffer<country_core::resources::terrain_height::TerrainHeightSsbo>
                                        ();
                              country_core::render::pass_builder::compute_pass_builder::ComputePass
                              ::
                              write_storage_buffer<country_core::resources::terrain_height::VisualTerrainHeightSsbo>
                                        ();
                              local_218 = local_9b8;
                              uStack_210 = uStack_9b0;
                              local_228 = local_9c8;
                              uStack_220 = uStack_9c0;
                              local_238 = local_9d8;
                              uStack_230 = uStack_9d0;
                              local_248 = (char *)local_9e8;
                              uStack_240 = uStack_9e0;
                              local_258 = local_9f8;
                              uStack_250 = uStack_9f0;
                              local_638 = local_a48;
                              local_648 = local_a58;
                              puStack_640 = (undefined1 *)uStack_a50;
                              local_618 = local_a40;
                              uStack_610 = uStack_a38;
                              local_608 = local_a30;
                              local_630 = local_a28;
                              uStack_628 = uStack_a20;
                              local_620 = local_a18;
                              local_5d0 = local_9a8;
                              uStack_5c8 = uStack_9a0;
                              local_5c0 = local_998;
                              uStack_5b8 = uStack_990;
                              local_5b0 = local_988;
                              uStack_5a8 = uStack_980;
                              local_5a0 = local_978;
                              uStack_598 = uStack_970;
                              local_590 = local_968;
                              uStack_588 = uStack_960;
                              local_580 = local_958;
                              uStack_578 = uStack_950;
                              local_5f0 = local_a00;
                              local_600 = local_a10;
                              uStack_5f8 = uStack_a08;
                              local_568 = local_8d8;
                              local_570 = local_920;
                              local_5e8 = 0x2000000000001;
                              uStack_5e0 = 0x100000001;
                              rhapsody::rg::pass_builder::PassBuilder::
                              render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
                                        ();
                              if (local_922 != '\b') {
                                rhapsody::raw::managed::handle::impl$6::drop();
                              }
                              country_core::render::pass_builder::compute_pass_builder::ComputePass
                              ::new<ref_<str_>_>();
                              local_258 = CONCAT71(local_258._1_7_,0xe);
                              local_a58 = CONCAT44(local_a58._4_4_,2);
                              country_core::render::pass_builder::compute_pass_builder::impl$4::
                              bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
                                        ();
                              memcpy(&local_a58,&local_648,400);
                              country_core::render::pass_builder::compute_pass_builder::ComputePass
                              ::
                              write_storage_buffer<country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo>
                                        ();
                              local_218 = uStack_5a8;
                              uStack_210 = local_5a0;
                              local_228 = uStack_5b8;
                              uStack_220 = local_5b0;
                              local_238 = uStack_5c8;
                              uStack_230 = local_5c0;
                              local_248 = (char *)uStack_5d8;
                              uStack_240 = local_5d0;
                              local_258 = local_5e8;
                              uStack_250 = uStack_5e0;
                              local_a48 = local_638;
                              local_a58 = local_648;
                              uStack_a50 = puStack_640;
                              local_a28 = local_630;
                              uStack_a20 = uStack_628;
                              local_a18 = local_620;
                              local_a40 = local_618;
                              uStack_a38 = uStack_610;
                              local_a30 = local_608;
                              uStack_9e0 = uStack_598;
                              local_9d8 = local_590;
                              uStack_9d0 = uStack_588;
                              local_9c8 = local_580;
                              uStack_9c0 = uStack_578;
                              local_9b8 = local_570;
                              uStack_9b0 = local_568;
                              local_9a8 = uStack_560;
                              uStack_9a0 = local_558;
                              local_998 = uStack_550;
                              uStack_990 = local_548;
                              local_988 = uStack_540;
                              local_a00 = local_5f0;
                              local_a10 = local_600;
                              uStack_a08 = uStack_5f8;
                              local_978 = local_4c8;
                              uStack_980 = local_510;
                              local_9f8 = 0x200000000001;
                              uStack_9f0 = 0x100000001;
                              rhapsody::rg::pass_builder::PassBuilder::
                              render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
                                        ();
                              if (local_512 != '\b') {
                                rhapsody::raw::managed::handle::impl$6::drop();
                              }
                              if (_DEV_OVERRIDE_IS_DEMO_MODE == '\0') {
                                country_core::render::pass_builder::compute_pass_builder::
                                ComputePass::new<ref_<str_>_>();
                                local_258 = CONCAT71(local_258._1_7_,0x13);
                                local_a58 = CONCAT44(local_a58._4_4_,2);
                                country_core::render::pass_builder::compute_pass_builder::impl$4::
                                bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
                                          ();
                                memcpy(&local_a58,&local_648,400);
                                country_core::render::pass_builder::compute_pass_builder::
                                ComputePass::
                                write_storage_buffer<country_core::resources::water_mask::WaterMaskSsbo>
                                          ();
                                country_core::render::pass_builder::compute_pass_builder::
                                ComputePass::
                                write_storage_buffer<country_core::resources::water_exclude_clutter_mask::WaterExcludeClutterMaskSsbo>
                                          ();
                                local_218 = local_9b8;
                                uStack_210 = uStack_9b0;
                                local_228 = local_9c8;
                                uStack_220 = uStack_9c0;
                                local_238 = local_9d8;
                                uStack_230 = uStack_9d0;
                                local_248 = (char *)local_9e8;
                                uStack_240 = uStack_9e0;
                                local_258 = local_9f8;
                                uStack_250 = uStack_9f0;
                                local_638 = local_a48;
                                local_648 = local_a58;
                                puStack_640 = (undefined1 *)uStack_a50;
                                local_618 = local_a40;
                                uStack_610 = uStack_a38;
                                local_608 = local_a30;
                                local_630 = local_a28;
                                uStack_628 = uStack_a20;
                                local_620 = local_a18;
                                local_5d0 = local_9a8;
                                uStack_5c8 = uStack_9a0;
                                local_5c0 = local_998;
                                uStack_5b8 = uStack_990;
                                local_5b0 = local_988;
                                uStack_5a8 = uStack_980;
                                local_5a0 = local_978;
                                uStack_598 = uStack_970;
                                local_590 = local_968;
                                uStack_588 = uStack_960;
                                local_580 = local_958;
                                uStack_578 = uStack_950;
                                local_5f0 = local_a00;
                                local_600 = local_a10;
                                uStack_5f8 = uStack_a08;
                                local_568 = local_8d8;
                                local_570 = local_920;
                                local_5e8 = 0x200000000001;
                                uStack_5e0 = 0x100000001;
                                rhapsody::rg::pass_builder::PassBuilder::
                                render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
                                          ();
                                if (local_922 != '\b') {
                                  rhapsody::raw::managed::handle::impl$6::drop();
                                }
                              }
                              country_core::render::pass_builder::compute_pass_builder::ComputePass
                              ::new<ref_<str_>_>();
                              local_258 = CONCAT71(local_258._1_7_,0x14);
                              local_a58 = CONCAT44(local_a58._4_4_,2);
                              country_core::render::pass_builder::compute_pass_builder::impl$4::
                              bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
                                        ();
                              memcpy(&local_a58,&local_648,400);
                              country_core::render::pass_builder::compute_pass_builder::ComputePass
                              ::
                              write_storage_buffer<country_core::resources::garden_mask::GardenMaskSsbo>
                                        ();
                              local_218 = uStack_5a8;
                              uStack_210 = local_5a0;
                              local_228 = uStack_5b8;
                              uStack_220 = local_5b0;
                              local_238 = uStack_5c8;
                              uStack_230 = local_5c0;
                              local_248 = (char *)uStack_5d8;
                              uStack_240 = local_5d0;
                              local_258 = local_5e8;
                              uStack_250 = uStack_5e0;
                              local_a48 = local_638;
                              local_a58 = local_648;
                              uStack_a50 = puStack_640;
                              local_a28 = local_630;
                              uStack_a20 = uStack_628;
                              local_a18 = local_620;
                              local_a40 = local_618;
                              uStack_a38 = uStack_610;
                              local_a30 = local_608;
                              uStack_9e0 = uStack_598;
                              local_9d8 = local_590;
                              uStack_9d0 = uStack_588;
                              local_9c8 = local_580;
                              uStack_9c0 = uStack_578;
                              local_9b8 = local_570;
                              uStack_9b0 = local_568;
                              local_9a8 = uStack_560;
                              uStack_9a0 = local_558;
                              local_998 = uStack_550;
                              uStack_990 = local_548;
                              local_988 = uStack_540;
                              local_a00 = local_5f0;
                              local_a10 = local_600;
                              uStack_a08 = uStack_5f8;
                              local_978 = local_4c8;
                              uStack_980 = local_510;
                              local_9f8 = 0x200000000001;
                              uStack_9f0 = 0x100000001;
                              rhapsody::rg::pass_builder::PassBuilder::
                              render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
                                        ();
                              if (local_512 != '\b') {
                                rhapsody::raw::managed::handle::impl$6::drop();
                              }
                              uVar6 = 0xfc8691076ab4d826;
                              bevy_ecs::component::Components::get_resource_id();
                              if (((extraout_RAX_19 != 0) &&
                                  (uVar6 < *(ulonglong *)(in_RDX + 0x218))) &&
                                 (uVar6 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar6 * 8),
                                 uVar6 != 0)) {
                                lVar5 = ~uVar6 * 0x60;
                                if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar5) != 0
                                   ) {
                                  lVar5 = *(longlong *)(in_RDX + 0x1e0) + lVar5;
                                  plVar3 = *(longlong **)(lVar5 + 0x10);
                                  *(undefined4 *)(lVar5 + 0x5c) = *(undefined4 *)(in_RDX + 0x5b0);
                                  lVar5 = *plVar3;
                                  LOCK();
                                  if (lVar5 == *plVar3) {
                                    *plVar3 = 0;
                                  }
                                  UNLOCK();
                                  *(byte *)(plVar3 + 1) = *(byte *)(plVar3 + 1) | lVar5 != 0;
                                  return;
                                }
                              }
                    // WARNING: Subroutine does not return
                              core::option::unwrap_failed();
                            }
                          }
                          local_71 = 1;
                    // WARNING: Subroutine does not return
                          core::option::unwrap_failed();
                        }
                      }
                      local_71 = 1;
                    // WARNING: Subroutine does not return
                      core::option::unwrap_failed();
                    }
                  }
                  local_71 = 1;
                    // WARNING: Subroutine does not return
                  core::option::unwrap_failed();
                }
              }
              local_71 = 1;
                    // WARNING: Subroutine does not return
              core::option::unwrap_failed();
            }
          }
          local_71 = 1;
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
      }
      local_71 = 1;
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
  }
  local_71 = 1;
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

