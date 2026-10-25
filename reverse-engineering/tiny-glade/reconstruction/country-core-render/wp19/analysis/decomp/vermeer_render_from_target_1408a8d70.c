// vermeer_render_from_target  0x1408a8d70
// signature: void __cdecl vermeer_render_from_target(void)
// callees: [{"name": "bind_texture_to_compute_pass<ref$<rhapsody::raw::texture::Texture>_>", "address": "1408fcd40"}, {"name": "bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>", "address": "1408ee240"}, {"name": "bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>", "address": "140908610"}, {"name": "bind_texture_to_draw_builder<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>", "address": "1408ee800"}, {"name": "blur_pyramid", "address": "1409ee1e0"}, {"name": "call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64>_>_>_>_>_>_>_>_>", "address": "1400d9b20"}, {"name": "create_raw_resource", "address": "140be94d0"}, {"name": "default", "address": "140be6750"}, {"name": "draw_no_vao", "address": "140906a90"}, {"name": "drop", "address": "140be24b0"}, {"name": "extent_2d", "address": "140bd5790"}, {"name": "from", "address": "140bd6ce0"}, {"name": "get_by_name", "address": "14093ca90"}, {"name": "get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$>_>", "address": "14096c310"}, {"name": "get_raster_handle_by_name", "address": "140937aa0"}, {"name": "get_resource_id", "address": "1400dd6f0"}, {"name": "into_sampled_rg_texture", "address": "140bd6470"}, {"name": "memcpy", "address": "142923c00"}, {"name": "new", "address": "140bd4de0"}, {"name": "new", "address": "140905be0"}, {"name": "new<country_core::systems::render_loop::vermeer::GenerateStrokes>", "address": "1415fea40"}, {"name": "new_2d", "address": "140bd52a0"}, {"name": "new_impl", "address": "140867970"}, {"name": "optional_uniform<array$<u32,2>_>", "address": "141629d20"}, {"name": "panic_access_error", "address": "1428d80d0"}, {"name": "read_rg_storage_buffer", "address": "140906150"}, {"name": "render", "address": "140868370"}, {"name": "render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>", "address": "1409dd4d0"}, {"name": "unwrap_failed", "address": "1428d9760"}, {"name": "unwrap_failed", "address": "1428d9310"}, {"name": "write_rg_storage_buffer", "address": "14096dd60"}, {"name": "write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<f32>,f32>", "address": "14096ba30"}, {"name": "write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>", "address": "14096b570"}, {"name": "write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<bool>,bool>", "address": "14096b850"}]


void __cdecl country_core::systems::render_loop::vermeer::vermeer_render_from_target(void)

{
  undefined4 uVar1;
  undefined4 extraout_EAX;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong extraout_RAX_03;
  ulonglong extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong in_RDX;
  ulonglong uVar2;
  char *pcVar3;
  undefined1 *in_R8;
  longlong in_R9;
  undefined4 uVar4;
  longlong in_stack_00000028;
  char in_stack_00000040;
  char in_stack_00000048;
  undefined1 local_cf8 [400];
  undefined1 *local_b68;
  undefined8 uStack_b60;
  undefined8 local_b58;
  undefined8 uStack_b50;
  undefined8 uStack_b48;
  undefined8 local_b40;
  undefined8 local_b38;
  undefined8 uStack_b30;
  undefined8 local_b28;
  undefined8 local_b20;
  undefined8 uStack_b18;
  undefined8 local_b10;
  undefined1 *local_b08;
  undefined8 uStack_b00;
  undefined8 local_af8;
  undefined8 uStack_af0;
  undefined8 local_ae8;
  undefined8 uStack_ae0;
  undefined8 local_ad8;
  undefined8 uStack_ad0;
  undefined8 local_ac8;
  undefined8 uStack_ac0;
  undefined8 local_ab8;
  undefined8 uStack_ab0;
  undefined8 local_aa8;
  undefined8 uStack_aa0;
  undefined8 local_a98;
  undefined8 uStack_a90;
  undefined8 local_a88;
  undefined8 uStack_a80;
  undefined8 local_a78;
  undefined8 uStack_a70;
  undefined8 local_a68;
  undefined8 uStack_a60;
  char local_a32;
  undefined8 local_a30;
  undefined8 local_9e8;
  undefined1 *local_778;
  undefined8 uStack_770;
  undefined8 local_768;
  undefined8 uStack_760;
  undefined8 local_758;
  undefined8 uStack_750;
  undefined8 local_748;
  undefined8 uStack_740;
  undefined8 local_738;
  undefined8 uStack_730;
  byte local_6d0;
  byte local_6cd;
  undefined4 local_678;
  undefined4 uStack_674;
  undefined4 uStack_670;
  undefined4 uStack_66c;
  undefined4 local_668;
  undefined4 uStack_664;
  undefined1 *puStack_660;
  undefined8 uStack_658;
  undefined8 local_650;
  undefined8 local_648;
  undefined8 uStack_640;
  undefined8 local_638;
  undefined8 local_630;
  undefined8 uStack_628;
  undefined8 local_620;
  undefined4 local_618;
  undefined4 local_614;
  undefined8 local_610;
  undefined8 local_600;
  undefined8 uStack_5f8;
  undefined8 local_5f0;
  undefined8 uStack_5e8;
  undefined8 local_5e0;
  undefined8 uStack_5d8;
  byte local_5d0;
  undefined2 uStack_5cf;
  byte bStack_5cd;
  undefined4 uStack_5cc;
  undefined4 uStack_5c8;
  undefined4 uStack_5c4;
  undefined4 local_5c0;
  undefined4 uStack_5bc;
  undefined4 uStack_5b8;
  undefined4 uStack_5b4;
  undefined4 local_5b0;
  undefined4 uStack_5ac;
  undefined4 uStack_5a8;
  undefined4 uStack_5a4;
  undefined4 local_5a0;
  undefined2 uStack_59c;
  char cStack_59a;
  undefined1 uStack_599;
  undefined4 uStack_598;
  undefined4 uStack_594;
  undefined8 uStack_588;
  char local_542;
  undefined1 local_28c [36];
  undefined8 local_268;
  undefined8 uStack_260;
  undefined8 uStack_250;
  undefined1 local_244 [4];
  undefined1 *local_240;
  undefined8 uStack_238;
  undefined8 local_230;
  undefined8 uStack_228;
  undefined8 local_220;
  undefined8 uStack_218;
  undefined8 local_210;
  undefined8 uStack_208;
  undefined8 local_200;
  undefined8 local_1f8;
  undefined8 local_1f0;
  undefined8 local_1e8;
  undefined8 local_1e0;
  undefined *local_1b0;
  undefined8 uStack_1a8;
  undefined8 local_1a0;
  undefined8 uStack_198;
  longlong local_190;
  longlong lStack_188;
  char *local_180;
  undefined4 local_178;
  undefined4 local_174;
  undefined2 uStack_170;
  undefined2 uStack_16e;
  undefined8 uStack_16c;
  undefined8 local_164;
  undefined8 uStack_15c;
  undefined8 local_154;
  undefined8 uStack_14c;
  char cStack_142;
  undefined5 uStack_141;
  undefined8 uStack_130;
  longlong local_90;
  undefined1 local_84;
  undefined1 local_83;
  undefined1 local_82;
  undefined1 local_81;
  undefined8 local_80;
  
  local_80 = 0xfffffffffffffffe;
  uVar4 = 0x8000;
  if (in_stack_00000040 != '\0') {
    uVar4 = 0xa00;
  }
  if (in_R9 == 0) {
    uVar2 = 0xa68561b396784eaa;
    bevy_ecs::component::Components::get_resource_id();
    if ((((extraout_RAX == 0) || (*(ulonglong *)(in_RDX + 0x218) <= uVar2)) ||
        (uVar2 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar2 * 8), uVar2 == 0)) ||
       (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + ~uVar2 * 0x60) == 0)) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    asset_libraries::texture_library::AssetTextureLibrary::get_by_name();
    rhapsody::rg::import_export::impl$2::into_sampled_rg_texture();
  }
  compute_passes::blur_pyramid::blur_pyramid();
  local_90 = in_stack_00000028 + 4;
  rhapsody::raw::texture::TextureDesc::extent_2d();
  rhapsody::raw::texture::TextureDesc::new_2d();
  local_b58 = local_210;
  uStack_b50 = uStack_208;
  local_b68 = local_220;
  uStack_b60 = uStack_218;
  rhapsody::rg::resource::impl$4::from();
  local_678 = 0x42a9bc70;
  uStack_674 = 1;
  uStack_670 = 0xe;
  uStack_66c = 0;
  rhapsody::rg::graph::RenderGraph::create_raw_resource();
  local_240 = local_220;
  uStack_238 = uStack_218;
  local_230 = local_210;
  uStack_228 = uStack_208;
  rhapsody::raw::buffer::BufferDesc::new();
  rhapsody::rg::temporal::impl$12::
  get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref_<str_>_>
            ();
  if ((char)puStack_660 == '\x02') {
    local_b68 = (undefined1 *)CONCAT44(uStack_674,local_678);
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_268 = CONCAT44(uStack_674,local_678);
  uStack_260 = CONCAT44(uStack_66c,uStack_670);
  uStack_250 = puStack_660;
  if (in_stack_00000048 == '\0') {
    country_core::render::pass_builder::compute_pass_builder::ComputePass::
    new<country_core::systems::render_loop::vermeer::GenerateStrokes>();
    if (in_R8 == (undefined1 *)0x0) {
      local_b68 = local_28c;
      local_678 = 2;
      country_core::render::pass_builder::compute_pass_builder::impl$4::
      bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
                ();
    }
    else {
      local_678 = 2;
      local_b68 = in_R8;
      country_core::render::pass_builder::compute_pass_builder::impl$4::
      bind_texture_to_compute_pass<ref_<rhapsody::raw::texture::Texture>_>();
    }
    memcpy(&local_220,local_cf8,400);
    memcpy(&local_678,&local_220,400);
    local_778 = local_28c;
    local_b68 = (undefined1 *)CONCAT44(local_b68._4_4_,2);
    country_core::render::pass_builder::compute_pass_builder::impl$4::
    bind_texture_to_compute_pass<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
              ();
    memcpy(&local_b68,&local_678,400);
    country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer()
    ;
    if (local_542 != '\b') {
      country_core::render::pass_builder::uniform_binder::UniformBinder::
      write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<f32>,f32>
                ();
    }
    memcpy(&local_b68,&local_678,400);
    if (local_a32 != '\b') {
      country_core::render::pass_builder::uniform_binder::UniformBinder::
      write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<i32>,i32>
                ();
    }
    memcpy(&local_678,&local_b68,400);
    if (local_542 != '\b') {
      country_core::render::pass_builder::uniform_binder::UniformBinder::
      write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<f32>,f32>
                ();
    }
    memcpy(&local_b68,&local_678,400);
    if (local_a32 != '\b') {
      country_core::render::pass_builder::uniform_binder::UniformBinder::
      write_uniform<country_core::render::pass_builder::draw_builder::impl_2::uniform::closure_env_0<bool>,bool>
                ();
    }
    memcpy(&local_678,&local_b68,400);
    if (local_542 != '\b') {
      country_core::render::pass_builder::uniform_binder::UniformBinder::
      write_uniform<country_core::render::pass_builder::draw_builder::impl_2::uniform::closure_env_0<bool>,bool>
                ();
    }
    memcpy(&local_b68,&local_678,400);
    local_738 = local_ac8;
    uStack_730 = uStack_ac0;
    local_748 = local_ad8;
    uStack_740 = uStack_ad0;
    local_758 = local_ae8;
    uStack_750 = uStack_ae0;
    local_768 = local_af8;
    uStack_760 = uStack_af0;
    local_778 = local_b08;
    uStack_770 = uStack_b00;
    local_668 = (undefined4)local_b58;
    uStack_664 = (undefined4)((ulonglong)local_b58 >> 0x20);
    local_678 = SUB84(local_b68,0);
    uStack_674 = (undefined4)((ulonglong)local_b68 >> 0x20);
    uStack_670 = (undefined4)uStack_b60;
    uStack_66c = (undefined4)((ulonglong)uStack_b60 >> 0x20);
    local_648 = uStack_b50;
    uStack_640 = uStack_b48;
    local_638 = local_b40;
    puStack_660 = (undefined1 *)local_b38;
    uStack_658 = uStack_b30;
    local_650 = local_b28;
    local_600 = local_ab8;
    uStack_5f8 = uStack_ab0;
    local_5f0 = local_aa8;
    uStack_5e8 = uStack_aa0;
    local_5e0 = local_a98;
    uStack_5d8 = uStack_a90;
    local_5d0 = (byte)local_a88;
    uStack_5cf = (undefined2)((ulonglong)local_a88 >> 8);
    bStack_5cd = (byte)((ulonglong)local_a88 >> 0x18);
    uStack_5cc = (undefined4)((ulonglong)local_a88 >> 0x20);
    uStack_5c8 = (undefined4)uStack_a80;
    uStack_5c4 = (undefined4)((ulonglong)uStack_a80 >> 0x20);
    local_5c0 = (undefined4)local_a78;
    uStack_5bc = (undefined4)((ulonglong)local_a78 >> 0x20);
    uStack_5b8 = (undefined4)uStack_a70;
    uStack_5b4 = (undefined4)((ulonglong)uStack_a70 >> 0x20);
    local_5b0 = (undefined4)local_a68;
    uStack_5ac = (undefined4)((ulonglong)local_a68 >> 0x20);
    uStack_5a8 = (undefined4)uStack_a60;
    uStack_5a4 = (undefined4)((ulonglong)uStack_a60 >> 0x20);
    local_620 = local_b10;
    local_630 = local_b20;
    uStack_628 = uStack_b18;
    uStack_598 = (undefined4)local_9e8;
    uStack_594 = (undefined4)((ulonglong)local_9e8 >> 0x20);
    local_5a0 = (undefined4)local_a30;
    uStack_59c = (undefined2)((ulonglong)local_a30 >> 0x20);
    cStack_59a = (char)((ulonglong)local_a30 >> 0x30);
    uStack_599 = (undefined1)((ulonglong)local_a30 >> 0x38);
    local_618 = 1;
    local_610 = 0x100000001;
    local_614 = uVar4;
    rhapsody::rg::pass_builder::PassBuilder::
    render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
              ();
    if (local_a32 != '\b') {
      rhapsody::raw::managed::handle::impl$6::drop();
    }
  }
  uVar2 = 0x8597ee0727b461b3;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_00 == 0) || (*(ulonglong *)(in_RDX + 0x218) <= uVar2)) ||
     ((uVar2 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar2 * 8), uVar2 == 0 ||
      (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + ~uVar2 * 0x60) == 0)))) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  pcVar3 = "vermeer_raster_strokes";
  asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
  if ((extraout_RAX_01 & 1) == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  rhapsody::raw::texture::TextureDesc::extent_2d();
  local_678 = 1;
  uStack_674 = 0;
  uStack_670 = 0;
  uStack_66c = 0;
  local_668 = 0;
  uStack_664 = 0;
  puStack_660 = local_244;
  local_220 = (undefined1 *)CONCAT44(local_220._4_4_,2);
  country_core::render::pass_builder::draw_stream::RasterPass::new_impl();
  local_82 = 1;
  rhapsody::raw::shader::impl$7::default();
  local_82 = 1;
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_02 == (longlong *)0x0) {
    local_82 = 1;
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  uStack_170 = 0;
  local_174 = 0x3f800000;
  local_220 = (undefined1 *)0x0;
  local_210 = 0;
  uStack_208 = 4;
  local_200 = 0;
  local_190 = *extraout_RAX_02;
  lStack_188 = extraout_RAX_02[1];
  *extraout_RAX_02 = *extraout_RAX_02 + 1;
  local_1a0 = 0;
  uStack_198 = 0;
  local_1b0 = &anon_65ceffb69360efd323e48f4f8756afaa_2_llvm_13316938065337964131;
  uStack_1a8 = 0;
  local_1e0 = 0x100000003;
  local_1f8 = 0;
  local_1f0 = 1;
  local_1e8 = 0;
  local_82 = 1;
  local_180 = pcVar3;
  country_core::render::pass_builder::draw_builder::DrawBuilder::new();
  local_83 = 1;
  rhapsody::raw::texture::TextureDesc::extent_2d();
  local_83 = 0;
  country_core::render::pass_builder::draw_builder::impl$2::optional_uniform<array_<u32,2>_>();
  if (cStack_142 != '\b') {
    country_core::render::pass_builder::uniform_binder::UniformBinder::
    write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<f32>,f32>
              ();
  }
  memcpy(&local_678,&local_220,0xf8);
  if (cStack_59a != '\b') {
    country_core::render::pass_builder::uniform_binder::UniformBinder::
    write_uniform<country_core::render::pass_builder::draw_builder::impl_2::uniform::closure_env_0<bool>,bool>
              ();
  }
  memcpy(&local_220,&local_678,0xf8);
  if (cStack_142 != '\b') {
    country_core::render::pass_builder::uniform_binder::UniformBinder::
    write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<i32>,i32>
              ();
  }
  memcpy(&local_678,&local_220,0xf8);
  local_778 = (undefined1 *)0x76ef6f6cc99f108b;
  local_220._0_4_ = 2;
  country_core::render::pass_builder::draw_builder::impl$3::
  bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
  memcpy(&local_220,&local_678,0xf8);
  local_82 = 1;
  country_core::render::pass_builder::draw_builder::DrawBuilder::read_rg_storage_buffer();
  local_6d0 = local_6d0 & 0x7f;
  local_6cd = local_6cd & 0x9f | 0x40;
  memcpy(&local_678,&local_778,0xf8);
  local_82 = 1;
  country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao();
  memcpy(&local_678,&local_b68,1000);
  local_82 = 0;
  country_core::render::pass_builder::draw_stream::RasterPass::render();
  uVar2 = 0x8597ee0727b461b3;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_03 == 0) || (*(ulonglong *)(in_RDX + 0x218) <= uVar2)) ||
     ((uVar2 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar2 * 8), uVar2 == 0 ||
      (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + ~uVar2 * 0x60) == 0)))) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  pcVar3 = "vermeer_combine";
  asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
  if ((extraout_RAX_04 & 1) == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  rhapsody::raw::texture::TextureDesc::extent_2d();
  local_678 = 0;
  uStack_670 = (undefined4)in_stack_00000028;
  uStack_66c = (undefined4)((ulonglong)in_stack_00000028 >> 0x20);
  local_220 = (undefined1 *)CONCAT44(local_220._4_4_,2);
  country_core::render::pass_builder::draw_stream::RasterPass::new_impl();
  local_81 = 1;
  rhapsody::raw::shader::impl$7::default();
  local_81 = 1;
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_05 != (longlong *)0x0) {
    uStack_170 = 0;
    local_174 = 0x3f800000;
    local_220 = (undefined1 *)0x0;
    local_210 = 0;
    uStack_208 = 4;
    local_200 = 0;
    local_190 = *extraout_RAX_05;
    lStack_188 = extraout_RAX_05[1];
    *extraout_RAX_05 = *extraout_RAX_05 + 1;
    local_1a0 = 0;
    uStack_198 = 0;
    local_1b0 = &anon_65ceffb69360efd323e48f4f8756afaa_2_llvm_13316938065337964131;
    uStack_1a8 = 0;
    local_1e0 = 0x100000003;
    local_1f8 = 0;
    local_1f0 = 1;
    local_1e8 = 0;
    local_81 = 1;
    local_180 = pcVar3;
    local_178 = extraout_EAX;
    country_core::render::pass_builder::draw_builder::DrawBuilder::new();
    local_84 = 1;
    rhapsody::raw::texture::TextureDesc::extent_2d();
    local_84 = 0;
    country_core::render::pass_builder::draw_builder::impl$2::optional_uniform<array_<u32,2>_>();
    if (cStack_142 != '\b') {
      country_core::render::pass_builder::uniform_binder::UniformBinder::
      write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<f32>,f32>
                ();
    }
    memcpy(&local_678,&local_220,0xf8);
    if (cStack_59a != '\b') {
      country_core::render::pass_builder::uniform_binder::UniformBinder::
      write_uniform<country_core::render::pass_builder::draw_builder::impl_2::uniform::closure_env_0<bool>,bool>
                ();
    }
    memcpy(&local_220,&local_678,0xf8);
    if (cStack_142 != '\b') {
      country_core::render::pass_builder::uniform_binder::UniformBinder::
      write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<i32>,i32>
                ();
    }
    memcpy(&local_678,&local_220,0xf8);
    local_778 = local_28c;
    local_220._0_4_ = 2;
    country_core::render::pass_builder::draw_builder::impl$3::
    bind_texture_to_draw_builder<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
              ();
    memcpy(&local_220,&local_678,0xf8);
    local_778 = local_244;
    local_678 = 2;
    country_core::render::pass_builder::draw_builder::impl$3::
    bind_texture_to_draw_builder<ref_<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>_>_>
              ();
    uVar1 = local_220._4_4_;
    uVar4 = (undefined4)local_220;
    memcpy(&uStack_670,&uStack_218,0xa0);
    uStack_5cc = local_174;
    uStack_5c8 = (undefined4)(CONCAT26(uStack_16e,CONCAT24(uStack_170,local_174)) >> 0x20);
    uStack_5c4 = (undefined4)uStack_16c;
    local_5c0 = (undefined4)((ulonglong)uStack_16c >> 0x20);
    uStack_5bc = (undefined4)local_164;
    uStack_5b8 = (undefined4)((ulonglong)local_164 >> 0x20);
    uStack_5b4 = (undefined4)uStack_15c;
    local_5b0 = (undefined4)((ulonglong)uStack_15c >> 0x20);
    uStack_5ac = (undefined4)local_154;
    uStack_5a8 = (undefined4)((ulonglong)local_154 >> 0x20);
    uStack_5a4 = (undefined4)uStack_14c;
    local_5a0 = (undefined4)((ulonglong)uStack_14c >> 0x20);
    uStack_599 = (undefined1)uStack_141;
    uStack_598 = (undefined4)((uint5)uStack_141 >> 8);
    uStack_588 = uStack_130;
    local_5d0 = (byte)local_178 & 0x7f;
    bStack_5cd = local_178._3_1_ & 0x9f | 0x40;
    local_678 = uVar4;
    uStack_674 = uVar1;
    uStack_5cf = local_178._1_2_;
    local_81 = 1;
    country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao();
    memcpy(&local_678,&local_b68,1000);
    local_81 = 0;
    country_core::render::pass_builder::draw_stream::RasterPass::render();
    return;
  }
  local_81 = 1;
                    // WARNING: Subroutine does not return
  std::thread::local::panic_access_error();
}

