// address: 0x14096e880
// pdb name: dispatch_indirect_raw
// signature: void __cdecl dispatch_indirect_raw(void)

void __cdecl
country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_indirect_raw(void)

{
  undefined8 *puVar1;
  undefined4 uVar2;
  undefined8 *puVar3;
  undefined8 *in_RCX;
  longlong in_RDX;
  undefined1 local_308 [310];
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
  undefined8 *local_40;
  undefined1 local_31;
  undefined8 local_30;
  
  local_30 = 0xfffffffffffffffe;
  local_31 = 1;
  local_40 = in_RCX;
  bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
  puVar3 = local_40;
  puVar1 = local_40 + 0xc;
  uVar2 = *(undefined4 *)(in_RDX + 0x10);
  local_31 = 1;
  rhapsody::rg::pass_builder::PassBuilder::
  add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>();
  memcpy(local_308,local_40,400);
  local_138 = puVar3[0x14];
  uStack_130 = puVar3[0x15];
  local_178 = *puVar1;
  uStack_170 = puVar3[0xd];
  local_168 = puVar3[0xe];
  uStack_160 = puVar3[0xf];
  local_158 = puVar3[0x10];
  uStack_150 = puVar3[0x11];
  local_148 = puVar3[0x12];
  uStack_140 = puVar3[0x13];
  local_118 = local_40[2];
  local_128 = *local_40;
  uStack_120 = local_40[1];
  local_e8 = local_40[5];
  local_f8 = local_40[3];
  uStack_f0 = local_40[4];
  local_100 = local_40[8];
  local_110 = local_40[6];
  uStack_108 = local_40[7];
  local_b0 = local_40[0x16];
  uStack_a8 = local_40[0x17];
  local_a0 = local_40[0x18];
  uStack_98 = local_40[0x19];
  local_90 = local_40[0x1a];
  uStack_88 = local_40[0x1b];
  local_80 = local_40[0x1c];
  uStack_78 = local_40[0x1d];
  local_70 = local_40[0x1e];
  uStack_68 = local_40[0x1f];
  local_60 = local_40[0x20];
  uStack_58 = local_40[0x21];
  local_d0 = local_40[0xb];
  local_e0 = local_40[9];
  uStack_d8 = local_40[10];
  local_48 = local_188;
  local_50 = local_1d0;
  local_c8 = 2;
  local_c4 = uVar2;
  rhapsody::rg::pass_builder::PassBuilder::
  render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
            ();
  if (local_1d2 != '\b') {
    local_31 = 0;
    rhapsody::raw::managed::handle::impl$6::drop();
  }
  return;
}


