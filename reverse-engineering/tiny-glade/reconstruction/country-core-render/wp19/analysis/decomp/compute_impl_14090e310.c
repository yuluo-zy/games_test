// compute_impl  0x14090e310
// signature: void __cdecl compute_impl(void)
// callees: [{"name": "__rust_dealloc", "address": "140613c20"}, {"name": "grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>", "address": "140562870"}, {"name": "memcpy", "address": "142923c00"}, {"name": "new<ref$<dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource>_>_>", "address": "141600570"}]


void __cdecl country_core::resources::gpu_lut::impl$3::render_luts::impl$0::compute_impl(void)

{
  longlong *plVar1;
  longlong lVar2;
  undefined8 uVar3;
  longlong in_RCX;
  longlong in_R8;
  undefined8 in_R9;
  longlong in_stack_00000028;
  undefined1 local_378 [400];
  undefined1 local_1e8 [400];
  undefined8 local_58;
  longlong local_50;
  undefined1 local_41;
  undefined8 local_40;
  
  local_40 = 0xfffffffffffffffe;
  local_50 = in_stack_00000028;
  plVar1 = *(longlong **)(in_RCX + 8);
  local_41 = 1;
  local_58 = in_R9;
  uVar3 = (**(code **)(in_R8 + 0x18))();
  lVar2 = plVar1[2];
  if (lVar2 == *plVar1) {
    local_41 = 1;
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
              ();
  }
  *(undefined8 *)(plVar1[1] + lVar2 * 8) = uVar3;
  plVar1[2] = lVar2 + 1;
  local_41 = 1;
  country_core::render::pass_builder::compute_pass_builder::ComputePass::
  new<ref_<dyn_<country_core::render::pass_builder::shader_uri::AsComputeShaderSource>_>_>();
  memcpy(local_378,local_1e8,400);
  (**(code **)(local_50 + 0x18))(local_58,local_378);
  if (*(longlong *)(local_50 + 8) != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
    return;
  }
  return;
}

