// 0x141d87140 butterflies_send_to_gpu
// SIG: void __cdecl butterflies_send_to_gpu(void)

void __cdecl system_critters::butterfly::butterflies_send_to_gpu(void)

{
  int iVar1;
  undefined8 *extraout_RAX;
  longlong lVar2;
  longlong local_38;
  longlong local_28;
  
  alloc::vec::spec_from_iter::impl$0::
  from_iter<country_core::systems::render_loop::resources::critters::ButterflySsbo,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple_<ref_<utils::components::transform::Transform>,ref_<country_core::resources::critters::Butterfly>_>,tuple_<>_>,system_critters::butterfly::butterflies_send_to_gpu::closure_env_0>_>
            ();
  if (local_28 != 0) {
    if (local_28 != 0x18) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    frozen::FrozenMap::
    get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
              ();
    if (extraout_RAX != (undefined8 *)0x0) {
      if (extraout_RAX[3] == 0x46) {
        iVar1 = memcmp(&anon_75d1013106e08612ad9ec328065f3af1_2_llvm_9468065370954387650,
                       (void *)extraout_RAX[2],0x46);
        if (iVar1 == 0) {
          lVar2 = (**(code **)(extraout_RAX[1] + 0x18))(*extraout_RAX);
          if (lVar2 != 0) {
            country_core::render::ssbo::impl$6::
            cpu_write<country_core::systems::render_loop::resources::critters::ButterflySsbo>();
          }
          goto LAB_141d8726c;
        }
      }
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
  }
LAB_141d8726c:
  if (local_38 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

