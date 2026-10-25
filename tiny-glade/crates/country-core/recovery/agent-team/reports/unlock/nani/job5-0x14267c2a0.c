
void __cdecl
country_core::components::drawable::impl$3::
component_ids<bevy_ecs::bundle::impl_8::register_info::closure_0::closure_env_0<country_core::components::drawable::DrawableMeshBundle>_>
          (void)

{
  longlong *plVar1;
  longlong lVar2;
  undefined8 extraout_RAX;
  undefined8 extraout_RAX_00;
  undefined8 extraout_RAX_01;
  longlong *in_RDX;
  
  bevy_ecs::component::ComponentsRegistrator::
  register_component<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>_>();
  plVar1 = (longlong *)*in_RDX;
  lVar2 = plVar1[2];
  if (lVar2 == *plVar1) {
    alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
  }
  *(undefined8 *)(plVar1[1] + lVar2 * 8) = extraout_RAX;
  plVar1[2] = lVar2 + 1;
  bevy_ecs::component::ComponentsRegistrator::
  register_component<country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>_>
            ();
  lVar2 = plVar1[2];
  if (lVar2 == *plVar1) {
    alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
  }
  *(undefined8 *)(plVar1[1] + lVar2 * 8) = extraout_RAX_00;
  plVar1[2] = lVar2 + 1;
  bevy_ecs::component::ComponentsRegistrator::
  register_component<utils::components::transform::Transform>();
  lVar2 = plVar1[2];
  if (lVar2 == *plVar1) {
    alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
  }
  *(undefined8 *)(plVar1[1] + lVar2 * 8) = extraout_RAX_01;
  plVar1[2] = lVar2 + 1;
  return;
}

