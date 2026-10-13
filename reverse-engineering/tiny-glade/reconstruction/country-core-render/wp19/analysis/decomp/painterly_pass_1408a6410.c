// painterly_pass  0x1408a6410
// signature: void __cdecl painterly_pass(void)
// callees: [{"name": "__rust_alloc", "address": "140613c10"}, {"name": "__rust_dealloc", "address": "140613c20"}, {"name": "as_inner_mut", "address": "14067dd80"}, {"name": "bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>", "address": "1408ee240"}, {"name": "bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>", "address": "1408edd00"}, {"name": "bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>", "address": "140908610"}, {"name": "call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64>_>_>_>_>_>_>_>_>", "address": "1400d9b20"}, {"name": "create_raw_resource", "address": "140be94d0"}, {"name": "default", "address": "140be6750"}, {"name": "draw_no_vao", "address": "140906a90"}, {"name": "drop", "address": "140be24b0"}, {"name": "extent_2d", "address": "140bd5790"}, {"name": "from", "address": "140bd6ce0"}, {"name": "get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$>_>", "address": "14096c310"}, {"name": "get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$>_>", "address": "14096c890"}, {"name": "get_raster_handle_by_name", "address": "140937aa0"}, {"name": "get_resource_id", "address": "1400dd6f0"}, {"name": "handle_error", "address": "1428d8fe3"}, {"name": "log", "address": "140d4a330"}, {"name": "memcpy", "address": "142923c00"}, {"name": "new", "address": "140bd4de0"}, {"name": "new", "address": "140905be0"}, {"name": "new<ref$<str$>_>", "address": "1416032a0"}, {"name": "new_2d", "address": "140bd52a0"}, {"name": "new_impl", "address": "140867970"}, {"name": "optional_uniform<array$<u32,2>_>", "address": "141629d20"}, {"name": "panic_access_error", "address": "1428d80d0"}, {"name": "panic_const_div_by_zero", "address": "1428d99e0"}, {"name": "read_rg_storage_buffer", "address": "140906150"}, {"name": "render", "address": "140868370"}, {"name": "render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>", "address": "1409dd4d0"}, {"name": "sampled", "address": "140bd5370"}, {"name": "storage", "address": "140bd5390"}, {"name": "unwrap_failed", "address": "1428d9760"}, {"name": "unwrap_failed", "address": "1428d9310"}, {"name": "write_rg_storage_buffer", "address": "14096dd60"}, {"name": "write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<f32>,f32>", "address": "14096ba30"}]


void __cdecl country_core::systems::render_loop::painterly::painterly_pass(void)

{
  double dVar1;
  int *piVar2;
  double dVar3;
  byte bVar4;
  uint uVar5;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  ulonglong extraout_RAX_03;
  undefined8 *extraout_RAX_04;
  undefined8 extraout_RAX_05;
  longlong extraout_RAX_06;
  undefined4 *extraout_RAX_07;
  longlong extraout_RAX_08;
  ulonglong extraout_RAX_09;
  longlong *extraout_RAX_10;
  longlong lVar6;
  longlong in_RDX;
  ulonglong uVar7;
  undefined *puVar8;
  double *pdVar9;
  undefined **in_R8;
  undefined4 uVar10;
  undefined4 uVar11;
  ulonglong uVar12;
  double dVar13;
  char in_stack_00000030;
  undefined8 local_af0;
  undefined4 uStack_ae8;
  undefined8 uStack_ae4;
  undefined8 local_adc;
  undefined8 uStack_ad4;
  byte local_a48;
  byte local_a45;
  undefined *local_9f8;
  undefined8 uStack_9f0;
  undefined8 local_9e8;
  undefined8 uStack_9e0;
  undefined *puStack_9d8;
  undefined8 local_9d0;
  ulonglong local_9c8;
  undefined8 uStack_9c0;
  undefined8 local_9b8;
  longlong local_9b0;
  undefined **ppuStack_9a8;
  undefined8 local_9a0;
  undefined4 local_998;
  undefined4 uStack_994;
  undefined8 uStack_990;
  undefined8 local_988;
  undefined8 uStack_980;
  undefined8 local_978;
  undefined8 uStack_970;
  undefined8 local_968;
  undefined8 uStack_960;
  undefined8 local_958;
  undefined8 uStack_950;
  undefined8 local_948;
  undefined8 uStack_940;
  undefined8 local_938;
  undefined8 uStack_930;
  undefined8 local_928;
  undefined8 uStack_920;
  undefined8 local_918;
  undefined8 uStack_910;
  undefined8 local_908;
  undefined8 uStack_900;
  undefined8 local_8f8;
  undefined8 uStack_8f0;
  char local_8c2;
  undefined8 local_8c0;
  undefined8 local_878;
  undefined *local_608;
  undefined8 uStack_600;
  undefined8 local_5f8;
  ulonglong uStack_5f0;
  undefined4 local_5e8;
  undefined *local_5d8;
  undefined **ppuStack_5d0;
  undefined8 local_5c8;
  ulonglong uStack_5c0;
  undefined8 local_5b8;
  undefined8 uStack_5b0;
  undefined8 local_5a8;
  undefined *puStack_5a0;
  undefined8 local_598;
  undefined8 local_590;
  undefined **ppuStack_588;
  undefined8 local_580;
  undefined8 local_578;
  undefined8 uStack_570;
  undefined8 uStack_568;
  undefined8 uStack_560;
  undefined8 uStack_558;
  undefined8 local_550;
  undefined8 uStack_548;
  undefined8 local_540;
  undefined8 uStack_538;
  undefined8 local_530;
  undefined8 uStack_528;
  undefined8 local_520;
  undefined8 uStack_518;
  undefined8 local_510;
  undefined8 uStack_508;
  undefined8 local_500;
  undefined8 local_4f8;
  undefined8 uStack_4f0;
  undefined8 local_4e8;
  undefined8 uStack_4e0;
  undefined8 local_4d8;
  undefined8 uStack_4d0;
  char local_4a2;
  undefined8 local_4a0;
  undefined8 local_458;
  undefined *local_1e8;
  undefined8 uStack_1e0;
  undefined8 local_1d8;
  undefined4 local_1c8;
  undefined4 uStack_1c4;
  undefined4 uStack_1c0;
  undefined4 uStack_1bc;
  undefined4 local_1b8;
  undefined4 uStack_1b4;
  undefined4 uStack_1b0;
  undefined4 uStack_1ac;
  undefined4 local_1a8;
  longlong local_1a0;
  undefined **local_198;
  undefined8 uStack_190;
  undefined8 local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined *local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  longlong local_108;
  longlong lStack_100;
  undefined *local_f8;
  undefined4 local_ec;
  undefined2 local_e8;
  char local_ba;
  undefined **local_98;
  ulonglong local_90;
  double *local_88;
  undefined1 local_73;
  char local_72;
  undefined1 local_71;
  undefined8 local_70;
  
  local_70 = 0xfffffffffffffffe;
  uVar7 = 0xd38dae617fb79c7a;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX == 0) || (*(ulonglong *)(in_RDX + 0x218) <= uVar7)) ||
     (uVar7 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar7 * 8), uVar7 == 0)) {
LAB_1408a77b9:
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  lVar6 = ~uVar7 * 0x60;
  if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar6) == 0) goto LAB_1408a77b9;
  piVar2 = *(int **)(*(longlong *)(in_RDX + 0x1e0) + lVar6 + 0x10);
  if ((*piVar2 == 7) && ((char)piVar2[2] == '\x01')) {
    return;
  }
  uVar7 = 0x1f18d9c9bdba6c60;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_00 == 0) || (*(ulonglong *)(in_RDX + 0x218) <= uVar7)) ||
     (uVar7 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar7 * 8), uVar7 == 0)) {
LAB_1408a77c5:
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  lVar6 = ~uVar7 * 0x60;
  if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar6) == 0) goto LAB_1408a77c5;
  lVar6 = *(longlong *)(in_RDX + 0x1e0) + lVar6;
  pdVar9 = *(double **)(lVar6 + 0x10);
  *(undefined4 *)(lVar6 + 0x5c) = *(undefined4 *)(in_RDX + 0x5b0);
  uVar7 = 0x48b321ef779dc301;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_01 == 0) || (*(ulonglong *)(in_RDX + 0x218) <= uVar7)) ||
     (uVar7 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar7 * 8), uVar7 == 0)) {
LAB_1408a77d1:
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  lVar6 = ~uVar7 * 0x60;
  if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar6) == 0) goto LAB_1408a77d1;
  if (*(char *)(*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + lVar6 + 0x10) + 0xff) == '\0') {
    uVar7 = 0x8fd9dc19c7973bca;
    bevy_ecs::component::Components::get_resource_id();
    if (((extraout_RAX_02 == 0) || (*(ulonglong *)(in_RDX + 0x218) <= uVar7)) ||
       (uVar7 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar7 * 8), uVar7 == 0)) {
LAB_1408a77dd:
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    lVar6 = ~uVar7 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar6) == 0) goto LAB_1408a77dd;
    local_72 = *(int *)(*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + lVar6 + 0x10) + 0x80) == 0;
  }
  else {
    local_72 = true;
  }
  if (in_stack_00000030 == '\0') {
    *(undefined4 *)(pdVar9 + 1) = 2;
    return;
  }
  uVar5 = *(uint *)(pdVar9 + 1);
  if (uVar5 == 2) {
    *pdVar9 = 0.0;
    *(undefined4 *)(pdVar9 + 1) = 0;
    *(undefined4 *)((longlong)pdVar9 + 0x14) = 0;
    uVar5 = 0;
  }
  local_1a0 = in_RDX + 0x38;
  local_98 = in_R8;
  local_88 = pdVar9;
  if ((bool)local_72) {
    *(undefined4 *)(pdVar9 + 1) = 0;
    *pdVar9 = 0.0;
    *(undefined4 *)((longlong)pdVar9 + 0x14) = 0;
  }
  else if ((uVar5 & 1) != 0) {
    uVar7 = *(ulonglong *)((longlong)pdVar9 + 0xc);
    goto LAB_1408a66ee;
  }
  rhapsody::raw::texture::TextureDesc::extent_2d();
  if (extraout_RAX_03 >> 0x20 == 0) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_const::panic_const_div_by_zero();
  }
  uVar7 = (ulonglong)(uint)((int)extraout_RAX_03 << 8) / (extraout_RAX_03 >> 0x20) | 0x10000000000;
LAB_1408a66ee:
  rhapsody::raw::texture::TextureDesc::new_2d();
  rhapsody::raw::texture::TextureDesc::storage();
  rhapsody::raw::texture::TextureDesc::sampled();
  rhapsody::rg::temporal::impl$11::
  get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref_<str_>_>
            ();
  bVar4 = local_5b8._4_1_;
  if (local_5b8._4_1_ == 2) {
    local_9f8 = local_5d8;
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_1a8 = (undefined4)local_5b8;
  local_1b8 = (undefined4)local_5c8;
  uStack_1b4 = (undefined4)((ulonglong)local_5c8 >> 0x20);
  uStack_1b0 = (undefined4)uStack_5c0;
  uStack_1ac = (undefined4)(uStack_5c0 >> 0x20);
  local_1c8 = SUB84(local_5d8,0);
  uStack_1c4 = (undefined4)((ulonglong)local_5d8 >> 0x20);
  uStack_1c0 = SUB84(ppuStack_5d0,0);
  uStack_1bc = (undefined4)((ulonglong)ppuStack_5d0 >> 0x20);
  local_90 = uVar7;
  rhapsody::raw::texture::TextureDesc::new_2d();
  rhapsody::raw::texture::TextureDesc::storage();
  rhapsody::raw::texture::TextureDesc::sampled();
  rhapsody::rg::temporal::impl$11::
  get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref_<str_>_>
            ();
  if (local_5b8._4_1_ == 2) {
    local_9f8 = local_5d8;
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_5e8 = (undefined4)local_5b8;
  local_5f8 = local_5c8;
  uStack_5f0 = uStack_5c0;
  local_608 = local_5d8;
  uStack_600 = ppuStack_5d0;
  if ((*(int *)(local_88 + 1) == 0) || (pdVar9 = local_88, (local_5b8._4_1_ & bVar4 & 1) == 0)) {
    local_9f8 = (undefined *)CONCAT44(uStack_1c0,uStack_1c4);
    uStack_ae4 = CONCAT44(local_1b8,uStack_1bc);
    local_adc = CONCAT44(uStack_1b0,uStack_1b4);
    uStack_ad4 = CONCAT44(local_1a8,uStack_1ac);
    local_af0._4_4_ = uStack_1c4;
    uStack_ae8 = uStack_1c0;
    uStack_9f0 = uStack_ae4;
    local_9e8 = local_adc;
    uStack_9e0 = uStack_ad4;
    rhapsody::rg::resource::impl$4::from();
    local_5d8 = &DAT_142a9b730;
    ppuStack_5d0 = (undefined **)0xd;
    rhapsody::rg::graph::RenderGraph::create_raw_resource();
    if (4 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      local_5b8 = *extraout_RAX_04;
      uStack_5b0 = extraout_RAX_04[1];
      local_5a8 = 5;
      puStack_5a0 = &DAT_142a9b780;
      local_598 = 0x2d;
      ppuStack_588 = &PTR_s_Acquiring_painterly_target_142a9b758;
      local_580 = 1;
      local_578 = (undefined **)0x8;
      local_5d8 = (undefined *)0x0;
      uStack_570 = 0;
      uStack_568 = 0;
      ppuStack_5d0 = (undefined **)&DAT_142a9b780;
      local_5c8 = 0x2d;
      uStack_5c0 = 0;
      local_590 = CONCAT44(*(undefined4 *)(extraout_RAX_04 + 2),1);
      ::log::__private_api::impl$2::log();
    }
    country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref_<str_>_>();
    local_198 = local_98;
    local_9f8._0_4_ = 2;
    country_core::render::pass_builder::compute_pass_builder::impl$4::
    bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
              ();
    uVar7 = local_90;
    memcpy(&local_9f8,&local_5d8,400);
    local_5d8._0_4_ = 2;
    country_core::render::pass_builder::compute_pass_builder::impl$4::
    bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
              ();
    memcpy(&local_5d8,&local_9f8,400);
    local_9f8._0_4_ = 2;
    country_core::render::pass_builder::compute_pass_builder::impl$4::
    bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
              ();
    memcpy(&local_9f8,&local_5d8,400);
    local_5d8 = (undefined *)CONCAT44(local_5d8._4_4_,2);
    local_198 = (undefined **)&local_af0;
    country_core::render::pass_builder::compute_pass_builder::impl$4::
    bind_texture_to_compute_pass<ref_mut_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
              ();
    memcpy(&local_5d8,&local_9f8,400);
    local_198 = &local_608;
    local_9f8 = (undefined *)CONCAT44(local_9f8._4_4_,2);
    country_core::render::pass_builder::compute_pass_builder::impl$4::
    bind_texture_to_compute_pass<ref_mut_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
              ();
    memcpy(&local_9f8,&local_5d8,400);
    local_158 = local_958;
    uStack_150 = uStack_950;
    local_168 = local_968;
    uStack_160 = uStack_960;
    local_178 = local_978;
    uStack_170 = uStack_970;
    local_188 = local_988;
    uStack_180 = uStack_980;
    uStack_190 = uStack_990;
    local_5c8 = local_9e8;
    local_5d8 = local_9f8;
    ppuStack_5d0 = (undefined **)uStack_9f0;
    local_5a8 = uStack_9e0;
    puStack_5a0 = puStack_9d8;
    local_598 = local_9d0;
    uStack_5c0 = local_9c8;
    local_5b8 = uStack_9c0;
    uStack_5b0 = local_9b8;
    uStack_560 = local_948;
    uStack_558 = uStack_940;
    local_550 = local_938;
    uStack_548 = uStack_930;
    local_540 = local_928;
    uStack_538 = uStack_920;
    local_530 = local_918;
    uStack_528 = uStack_910;
    local_520 = local_908;
    uStack_518 = uStack_900;
    local_510 = local_8f8;
    uStack_508 = uStack_8f0;
    local_580 = local_9a0;
    local_590 = local_9b0;
    ppuStack_588 = ppuStack_9a8;
    local_4f8 = local_878;
    local_500 = local_8c0;
    uVar11 = (undefined4)uVar7;
    local_578 = (undefined **)CONCAT44(uVar11,1);
    uVar10 = (undefined4)(uVar7 >> 0x20);
    uStack_570 = CONCAT44(1,uVar10);
    rhapsody::rg::pass_builder::PassBuilder::
    render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
              ();
    if (local_8c2 != '\b') {
      rhapsody::raw::managed::handle::impl$6::drop();
    }
    country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref_<str_>_>();
    local_9f8 = (undefined *)CONCAT44(local_9f8._4_4_,2);
    local_198 = (undefined **)&local_af0;
    country_core::render::pass_builder::compute_pass_builder::impl$4::
    bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
              ();
    memcpy(&local_9f8,&local_5d8,400);
    local_198 = (undefined **)&local_1c8;
    local_5d8 = (undefined *)CONCAT44(local_5d8._4_4_,2);
    country_core::render::pass_builder::compute_pass_builder::impl$4::
    bind_texture_to_compute_pass<ref_mut_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
              ();
    memcpy(&local_5d8,&local_9f8,400);
    local_158 = uStack_538;
    uStack_150 = local_530;
    local_168 = uStack_548;
    uStack_160 = local_540;
    local_178 = uStack_558;
    uStack_170 = local_550;
    local_188 = uStack_568;
    uStack_180 = uStack_560;
    local_198 = local_578;
    uStack_190 = uStack_570;
    local_9e8 = local_5c8;
    local_9f8 = local_5d8;
    uStack_9f0 = ppuStack_5d0;
    local_9c8 = uStack_5c0;
    uStack_9c0 = local_5b8;
    local_9b8 = uStack_5b0;
    uStack_9e0 = local_5a8;
    puStack_9d8 = puStack_5a0;
    local_9d0 = local_598;
    uStack_980 = uStack_528;
    local_978 = local_520;
    uStack_970 = uStack_518;
    local_968 = local_510;
    uStack_960 = uStack_508;
    local_958 = local_500;
    uStack_950 = local_4f8;
    local_948 = uStack_4f0;
    uStack_940 = local_4e8;
    local_938 = uStack_4e0;
    uStack_930 = local_4d8;
    local_928 = uStack_4d0;
    local_9a0 = local_580;
    local_9b0 = local_590;
    ppuStack_9a8 = ppuStack_588;
    local_918 = local_458;
    uStack_920 = local_4a0;
    local_998 = 1;
    uStack_990 = CONCAT44(1,uVar10);
    uStack_994 = uVar11;
    rhapsody::rg::pass_builder::PassBuilder::
    render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
              ();
    pdVar9 = local_88;
    if (local_4a2 != '\b') {
      rhapsody::raw::managed::handle::impl$6::drop();
    }
    rhapsody::raw::texture::TextureDesc::extent_2d();
    *(undefined4 *)(pdVar9 + 1) = 1;
    *(undefined8 *)((longlong)pdVar9 + 0xc) = extraout_RAX_05;
  }
  rhapsody::raw::buffer::BufferDesc::new();
  rhapsody::rg::temporal::impl$12::
  get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref_<str_>_>
            ();
  if ((char)uStack_5c0 == '\x02') {
    local_9f8 = local_5d8;
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_1d8 = local_5c8;
  local_1e8 = local_5d8;
  uStack_1e0 = ppuStack_5d0;
  if ((uStack_5c0 & 1) == 0) {
    *pdVar9 = 0.0;
  }
  uVar7 = 0x109ca79d5cccc2d1;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_06 != 0) && (uVar7 < *(ulonglong *)(in_RDX + 0x218))) &&
     (uVar7 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar7 * 8), uVar7 != 0)) {
    lVar6 = ~uVar7 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar6) != 0) {
      dVar1 = *pdVar9;
      uVar7 = (longlong)dVar1 >> 0x3f & (longlong)(dVar1 - 9.223372036854776e+18) | (longlong)dVar1;
      if (dVar1 < 0.0) {
        uVar7 = 0;
      }
      if (1.844674407370955e+19 < dVar1) {
        uVar7 = 0xffffffffffffffff;
      }
      local_90 = *(longlong *)(*(longlong *)(in_RDX + 0x1e0) + lVar6 + 0x10);
      dVar13 = 1024.0;
      if (local_72 == '\0') {
        dVar13 = *(double *)(local_90 + 0x40) * 1638.4;
      }
      dVar3 = dVar1 + dVar13;
      if (1024.0 <= dVar1 + dVar13) {
        dVar3 = 1024.0;
      }
      uVar12 = (longlong)(dVar3 - 9.223372036854776e+18) & (longlong)dVar3 >> 0x3f | (longlong)dVar3
      ;
      if (dVar3 < 0.0) {
        uVar12 = 0;
      }
      if (1.844674407370955e+19 < dVar3) {
        uVar12 = 0xffffffffffffffff;
      }
      if (uVar12 - uVar7 != 0) {
        country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref_<str_>_>();
        local_198 = (undefined **)&local_1c8;
        local_9f8 = (undefined *)CONCAT44(local_9f8._4_4_,2);
        country_core::render::pass_builder::compute_pass_builder::impl$4::
        bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                  ();
        memcpy(&local_9f8,&local_5d8,400);
        local_198 = &local_608;
        local_5d8 = (undefined *)CONCAT44(local_5d8._4_4_,2);
        country_core::render::pass_builder::compute_pass_builder::impl$4::
        bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                  ();
        memcpy(&local_5d8,&local_9f8,400);
        country_core::render::pass_builder::compute_pass_builder::ComputePass::
        write_rg_storage_buffer();
        uVar10 = *(undefined4 *)((longlong)local_88 + 0x14);
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_07 == (undefined4 *)0x0) {
                    // WARNING: Subroutine does not return
          alloc::raw_vec::handle_error();
        }
        *extraout_RAX_07 = (int)uVar7;
        uVar11 = (undefined4)(uVar12 - uVar7);
        extraout_RAX_07[1] = uVar11;
        extraout_RAX_07[2] = uVar10;
        if (local_9b0 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        local_9b0 = 0xc;
        local_9a0 = 0xc;
        memcpy(&local_5d8,&local_9f8,400);
        local_158 = uStack_538;
        uStack_150 = local_530;
        local_168 = uStack_548;
        uStack_160 = local_540;
        local_178 = uStack_558;
        uStack_170 = local_550;
        local_188 = uStack_568;
        uStack_180 = uStack_560;
        local_198 = local_578;
        uStack_190 = uStack_570;
        local_9e8 = local_5c8;
        local_9f8 = local_5d8;
        uStack_9f0 = ppuStack_5d0;
        local_9c8 = uStack_5c0;
        uStack_9c0 = local_5b8;
        local_9b8 = uStack_5b0;
        uStack_9e0 = local_5a8;
        puStack_9d8 = puStack_5a0;
        local_9d0 = local_598;
        uStack_980 = uStack_528;
        local_978 = local_520;
        uStack_970 = uStack_518;
        local_968 = local_510;
        uStack_960 = uStack_508;
        local_958 = local_500;
        uStack_950 = local_4f8;
        local_948 = uStack_4f0;
        uStack_940 = local_4e8;
        local_938 = uStack_4e0;
        uStack_930 = local_4d8;
        local_928 = uStack_4d0;
        local_9a0 = local_580;
        local_9b0 = local_590;
        ppuStack_9a8 = ppuStack_588;
        local_918 = local_458;
        uStack_920 = local_4a0;
        local_998 = 1;
        uStack_990 = 0x100000001;
        uStack_994 = uVar11;
        rhapsody::rg::pass_builder::PassBuilder::
        render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
                  ();
        if (local_4a2 != '\b') {
          rhapsody::raw::managed::handle::impl$6::drop();
        }
      }
      pdVar9 = local_88;
      *local_88 = dVar3;
      uVar7 = 0x8597ee0727b461b3;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_08 == 0) || (*(ulonglong *)(in_RDX + 0x218) <= uVar7)) ||
         ((uVar7 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar7 * 8), uVar7 == 0 ||
          (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + ~uVar7 * 0x60) == 0)))) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      puVar8 = &anon_71a698368e7604435180484d48b7967e_89_llvm_7739737788480422836;
      asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
      if ((extraout_RAX_09 & 1) == 0) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      rhapsody::raw::texture::TextureDesc::extent_2d();
      local_5d8 = (undefined *)((ulonglong)local_5d8 & 0xffffffff00000000);
      ppuStack_5d0 = local_98;
      local_198 = (undefined **)CONCAT44(local_198._4_4_,2);
      country_core::render::pass_builder::draw_stream::RasterPass::new_impl();
      local_71 = 1;
      rhapsody::raw::shader::impl$7::default();
      local_71 = 1;
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_10 != (longlong *)0x0) {
        local_e8 = 0;
        local_ec = 0x3f800000;
        local_198 = (undefined **)0x0;
        local_188 = 0;
        uStack_180 = 4;
        local_178 = 0;
        local_108 = *extraout_RAX_10;
        lStack_100 = extraout_RAX_10[1];
        *extraout_RAX_10 = *extraout_RAX_10 + 1;
        local_118 = 0;
        uStack_110 = 0;
        local_128 = &anon_65ceffb69360efd323e48f4f8756afaa_2_llvm_13316938065337964131;
        uStack_120 = 0;
        local_158 = 0x100000003;
        uStack_170 = 0;
        local_168 = 1;
        uStack_160 = 0;
        local_71 = 1;
        local_f8 = puVar8;
        country_core::render::pass_builder::draw_builder::DrawBuilder::new();
        local_73 = 1;
        rhapsody::raw::texture::TextureDesc::extent_2d();
        local_73 = 0;
        country_core::render::pass_builder::draw_builder::impl$2::optional_uniform<array_<u32,2>_>()
        ;
        if (local_ba != '\b') {
          country_core::render::pass_builder::uniform_binder::UniformBinder::
          write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<f32>,f32>
                    ();
        }
        memcpy(&local_5d8,&local_198,0xf8);
        local_af0._0_4_ = 0xc99f108b;
        local_af0._4_4_ = 0x76ef6f6c;
        local_198 = (undefined **)CONCAT44(local_198._4_4_,2);
        country_core::render::pass_builder::draw_builder::impl$3::
        bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
        memcpy(&local_198,&local_5d8,0xf8);
        local_71 = 1;
        country_core::render::pass_builder::draw_builder::DrawBuilder::read_rg_storage_buffer();
        local_a48 = local_a48 & 0x7f;
        local_a45 = local_a45 & 0x9f | 0x40;
        memcpy(&local_5d8,&local_af0,0xf8);
        local_71 = 1;
        country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao();
        memcpy(&local_5d8,&local_9f8,1000);
        local_71 = 0;
        country_core::render::pass_builder::draw_stream::RasterPass::render();
        *(float *)((longlong)pdVar9 + 0x14) =
             *(float *)(local_90 + 0x58) + *(float *)((longlong)pdVar9 + 0x14);
        return;
      }
      local_71 = 1;
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

