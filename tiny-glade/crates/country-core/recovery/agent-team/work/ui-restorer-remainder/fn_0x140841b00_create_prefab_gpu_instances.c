void __cdecl create_prefab_gpu_instances(void)


void __cdecl country_core::systems::prefab::create_prefab_gpu_instances(void)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 *puVar3;
  undefined2 extraout_AX;
  undefined4 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *in_RCX;
  undefined4 *puVar4;
  longlong in_R8;
  undefined8 *in_stack_00000028;
  longlong in_stack_00000030;
  undefined1 *local_140;
  code *local_138;
  undefined1 *local_130;
  code *local_128;
  undefined8 local_120;
  undefined8 local_118;
  undefined8 local_110;
  undefined8 local_108;
  undefined8 local_100;
  undefined8 uStack_f8;
  undefined8 local_f0;
  char *local_e8;
  undefined8 local_e0;
  undefined4 local_d8;
  undefined4 local_d4;
  undefined **local_d0;
  undefined8 local_c8;
  undefined1 **local_c0;
  undefined8 local_b8;
  undefined8 local_b0;
  undefined1 local_a0 [16];
  undefined1 local_90 [8];
  undefined8 *local_88;
  undefined8 local_80;
  undefined8 uStack_78;
  undefined8 local_70;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  local_88 = in_RCX;
  resources::render::mesh_atlas_library::MeshAtlasLibrary::
  get_by_name<ref_<asset::prefab::PrefabNameRef>_>();
  local_80 = 0;
  uStack_78 = 4;
  local_70 = 0;
  if (4 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
    local_140 = local_90;
    local_138 = core::fmt::num::imp::impl$9::fmt;
    local_130 = local_a0;
    local_128 = core::fmt::impl$74::fmt<asset::prefab::PrefabNameRef>;
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    local_100 = *extraout_RAX_00;
    uStack_f8 = extraout_RAX_00[1];
    local_d4 = *(undefined4 *)(extraout_RAX_00 + 2);
    local_f0 = 5;
    local_e8 = "country_core::systems::prefab";
    local_e0 = 0x1d;
    local_d0 = &PTR_s_Creating_parts_for_prefab_142a942f0;
    local_c8 = 2;
    local_c0 = &local_140;
    local_b8 = 2;
    local_b0 = 0;
    local_120 = 0;
    local_118 = "country_core::systems::prefab";
    local_110 = 0x1d;
    local_108 = 0;
    local_d8 = 1;
    ::log::__private_api::impl$2::log();
  }
  uVar1 = *(undefined4 *)(in_stack_00000030 + 0x1c);
  puVar3 = *(undefined4 **)(in_stack_00000030 + 0x10);
  for (puVar4 = extraout_RAX; puVar4 != extraout_RAX + in_R8 * 2; puVar4 = puVar4 + 2) {
    uVar2 = *puVar4;
    half::binary16::f16::from_f32();
    *puVar3 = uVar1;
    local_120 = *in_stack_00000028;
    local_118 = (char *)CONCAT26((short)uVar2,
                                 CONCAT24(extraout_AX,*(undefined4 *)(in_stack_00000028 + 1)));
    country_core::render::nani::gpu_world::GpuWorld::
    create_instances<country_core::resources::placement_data::PlacementDataSsbo,country_core::systems::prefab::create_prefab_gpu_instances::closure_env_0>
              ();
  }
  local_88[2] = local_70;
  *local_88 = local_80;
  local_88[1] = uStack_78;
  return;
}

