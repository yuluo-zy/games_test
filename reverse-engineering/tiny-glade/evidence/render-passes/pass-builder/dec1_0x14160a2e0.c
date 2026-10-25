// address: 0x14160a2e0
// pdb name: dispatch_with<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket::closure_env$1>
// signature: void __cdecl dispatch_with<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket::closure_env$1>(void)

void __cdecl
country_core::render::pass_builder::compute_pass_builder::ComputePass::
dispatch_with<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket::closure_env_1>
          (void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 *extraout_RAX;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  undefined1 local_2f8 [310];
  char local_1c2;
  undefined8 local_1c0;
  undefined8 local_178;
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
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 local_100;
  undefined8 uStack_f8;
  undefined8 local_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined8 local_d0;
  undefined8 uStack_c8;
  undefined8 local_c0;
  undefined4 local_b8;
  undefined **local_a8;
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
  undefined8 uStack_48;
  undefined8 local_40;
  undefined8 local_38;
  undefined8 local_20;
  
  local_20 = 0xfffffffffffffffe;
  memcpy(local_2f8,in_RCX,400);
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX != (undefined8 *)0x0) {
    uVar1 = *in_RDX;
    uVar2 = in_RDX[1];
    uVar3 = in_RDX[2];
    uVar4 = in_RDX[3];
    uVar5 = in_RDX[5];
    extraout_RAX[4] = in_RDX[4];
    extraout_RAX[5] = uVar5;
    extraout_RAX[2] = uVar3;
    extraout_RAX[3] = uVar4;
    *extraout_RAX = uVar1;
    extraout_RAX[1] = uVar2;
    local_128 = in_RCX[0x14];
    uStack_120 = in_RCX[0x15];
    local_168 = in_RCX[0xc];
    uStack_160 = in_RCX[0xd];
    local_158 = in_RCX[0xe];
    uStack_150 = in_RCX[0xf];
    local_148 = in_RCX[0x10];
    uStack_140 = in_RCX[0x11];
    local_138 = in_RCX[0x12];
    uStack_130 = in_RCX[0x13];
    local_108 = in_RCX[2];
    local_118 = *in_RCX;
    uStack_110 = in_RCX[1];
    local_d8 = in_RCX[5];
    local_e8 = in_RCX[3];
    uStack_e0 = in_RCX[4];
    local_f0 = in_RCX[8];
    local_100 = in_RCX[6];
    uStack_f8 = in_RCX[7];
    local_a0 = in_RCX[0x16];
    uStack_98 = in_RCX[0x17];
    local_90 = in_RCX[0x18];
    uStack_88 = in_RCX[0x19];
    local_80 = in_RCX[0x1a];
    uStack_78 = in_RCX[0x1b];
    local_70 = in_RCX[0x1c];
    uStack_68 = in_RCX[0x1d];
    local_60 = in_RCX[0x1e];
    uStack_58 = in_RCX[0x1f];
    local_50 = in_RCX[0x20];
    uStack_48 = in_RCX[0x21];
    local_c0 = in_RCX[0xb];
    local_d0 = in_RCX[9];
    uStack_c8 = in_RCX[10];
    local_38 = local_178;
    local_40 = local_1c0;
    local_b8 = 3;
    local_a8 = &anon_0ad13c2f7becfb1cf74d6efc314db65f_20_llvm_14037420088952369931;
    rhapsody::rg::pass_builder::PassBuilder::
    render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
              ();
    if (local_1c2 != '\b') {
      rhapsody::raw::managed::handle::impl$6::drop();
    }
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::alloc::handle_alloc_error();
}


