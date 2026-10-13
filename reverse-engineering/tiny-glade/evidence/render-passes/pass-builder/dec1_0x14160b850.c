// address: 0x14160b850
// pdb name: dispatch_indirect_with<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::group_draw_cmd_prefix_sum::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>_>
// signature: void __cdecl dispatch_indirect_with<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::group_draw_cmd_prefix_sum::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>_>(void)

void __cdecl
country_core::render::pass_builder::compute_pass_builder::ComputePass::
dispatch_indirect_with<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::group_draw_cmd_prefix_sum::closure_env_0<country_core::render::nani::gpu_world::RenderSublistsIter>_>
          (void)

{
  undefined4 uVar1;
  undefined8 uVar2;
  undefined8 *extraout_RAX;
  void *in_RCX;
  longlong in_RDX;
  undefined8 *in_R8;
  undefined8 local_308;
  undefined8 uStack_300;
  undefined8 local_2f8;
  undefined8 local_2f0;
  undefined8 uStack_2e8;
  undefined8 local_2e0;
  undefined8 local_2d8;
  undefined8 uStack_2d0;
  undefined8 local_2c8;
  undefined8 local_2c0;
  undefined8 uStack_2b8;
  undefined8 local_2b0;
  undefined8 local_2a8;
  undefined8 uStack_2a0;
  undefined8 local_298;
  undefined8 uStack_290;
  undefined8 local_288;
  undefined8 uStack_280;
  undefined8 local_278;
  undefined8 uStack_270;
  undefined8 local_268;
  undefined8 uStack_260;
  undefined8 local_258;
  undefined8 uStack_250;
  undefined8 local_248;
  undefined8 uStack_240;
  undefined8 local_238;
  undefined8 uStack_230;
  undefined8 local_228;
  undefined8 uStack_220;
  undefined8 local_218;
  undefined8 uStack_210;
  undefined8 local_208;
  undefined8 uStack_200;
  char local_1d2;
  undefined8 local_1d0;
  undefined8 local_188;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 local_110;
  undefined8 uStack_108;
  undefined8 local_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined8 local_e0;
  undefined8 uStack_d8;
  undefined8 local_d0;
  undefined4 local_c8;
  undefined4 local_c4;
  undefined *local_b8;
  undefined8 local_b0;
  undefined8 uStack_a8;
  undefined8 local_a0;
  undefined8 uStack_98;
  undefined8 local_90;
  undefined8 uStack_88;
  undefined8 local_80;
  undefined8 uStack_78;
  undefined8 local_70;
  undefined8 uStack_68;
  undefined8 local_60;
  undefined8 uStack_58;
  undefined8 local_50;
  undefined8 local_48;
  void *local_30;
  undefined1 local_21;
  undefined8 local_20;
  
  local_20 = 0xfffffffffffffffe;
  local_21 = 1;
  local_30 = in_RCX;
  bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
  uVar1 = *(undefined4 *)(in_RDX + 0x10);
  local_21 = 1;
  rhapsody::rg::pass_builder::PassBuilder::
  add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>();
  memcpy(&local_308,local_30,400);
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX != (undefined8 *)0x0) {
    uVar2 = in_R8[1];
    *extraout_RAX = *in_R8;
    extraout_RAX[1] = uVar2;
    local_138 = local_268;
    uStack_130 = uStack_260;
    local_148 = local_278;
    uStack_140 = uStack_270;
    local_158 = local_288;
    uStack_150 = uStack_280;
    local_168 = local_298;
    uStack_160 = uStack_290;
    local_178 = local_2a8;
    uStack_170 = uStack_2a0;
    local_118 = local_2f8;
    local_128 = local_308;
    uStack_120 = uStack_300;
    local_f8 = local_2f0;
    uStack_f0 = uStack_2e8;
    local_e8 = local_2e0;
    local_110 = local_2d8;
    uStack_108 = uStack_2d0;
    local_100 = local_2c8;
    local_b0 = local_258;
    uStack_a8 = uStack_250;
    local_a0 = local_248;
    uStack_98 = uStack_240;
    local_90 = local_238;
    uStack_88 = uStack_230;
    local_80 = local_228;
    uStack_78 = uStack_220;
    local_70 = local_218;
    uStack_68 = uStack_210;
    local_60 = local_208;
    uStack_58 = uStack_200;
    local_d0 = local_2b0;
    local_e0 = local_2c0;
    uStack_d8 = uStack_2b8;
    local_48 = local_188;
    local_50 = local_1d0;
    local_c8 = 4;
    local_b8 = &anon_0ad13c2f7becfb1cf74d6efc314db65f_28_llvm_14037420088952369931;
    local_c4 = uVar1;
    rhapsody::rg::pass_builder::PassBuilder::
    render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
              ();
    if (local_1d2 != '\b') {
      rhapsody::raw::managed::handle::impl$6::drop();
    }
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::alloc::handle_alloc_error();
}


