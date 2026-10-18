
void __cdecl
core::ops::function::impls::impl$4::
call_once<tuple_<ref_<country_core::resources::history::wall_compound_edit::ConcurrentEdits>_>,country_core::systems::history::replay_wall::impl_0::new::closure_env_0>
          (void)

{
  undefined1 auVar1 [16];
  ulonglong uVar2;
  ulonglong extraout_RAX;
  ulonglong *in_RCX;
  ulonglong uVar3;
  longlong in_R8;
  
  uVar3 = *(ulonglong *)(in_R8 + 8);
  auVar1._8_8_ = 0;
  auVar1._0_8_ = uVar3;
  uVar2 = SUB168(auVar1 * ZEXT816(0x18),0);
  if ((SUB168(auVar1 * ZEXT816(0x18),8) == 0) && (uVar2 < 0x7ffffffffffffff9)) {
    if (uVar2 == 0) {
      uVar2 = 8;
      uVar3 = 0;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      uVar2 = extraout_RAX;
      if (extraout_RAX == 0) goto LAB_140abf755;
    }
    iter::adapters::map::impl$2::
    fold<country_core::systems::history::replay_wall::WallReplayEditCmd,core::slice::iter::Iter<alloc::sync::Arc<enum2_<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>_>,country_core::systems::history::replay_wall::impl_0::new::closure_0::closure_env_0,tuple_<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env_0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::vec::impl_20::extend_trusted::closure_env_0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2_<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>_>,country_core::systems::history::replay_wall::impl_0::new::closure_0::closure_env_0>_>_>_>
              ();
    in_RCX[2] = 0;
    *in_RCX = uVar3;
    in_RCX[1] = uVar2;
    return;
  }
LAB_140abf755:
                    // WARNING: Subroutine does not return
  ::alloc::raw_vec::handle_error();
}

