/* run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>),void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>)>_> @0x0x141bb19d0 size 431 */

void __cdecl
bevy_ecs::system::schedule_system::impl$1::
run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>),void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>)>_>
          (void)

{
  int *piVar1;
  int iVar2;
  ulonglong uVar3;
  undefined8 *puVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  longlong *extraout_RAX;
  byte *in_RCX;
  longlong in_RDX;
  char *local_98;
  undefined8 local_90;
  byte *local_88;
  code *local_80;
  char **local_78;
  code *local_70;
  undefined *local_68;
  undefined8 local_60;
  byte **local_58;
  undefined8 local_50;
  undefined8 local_48;
  
  LOCK();
  piVar1 = (int *)(in_RDX + 0x5b0);
  iVar2 = *piVar1;
  *piVar1 = *piVar1 + 1;
  UNLOCK();
  if ((*in_RCX & 1) == 0) {
                    // WARNING: Subroutine does not return
    core::option::expect_failed();
  }
  if ((*(ulonglong *)(in_RCX + 8) < *(ulonglong *)(in_RDX + 0x218)) &&
     (uVar3 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + *(ulonglong *)(in_RCX + 8) * 8),
     uVar3 != 0)) {
    lVar7 = ~uVar3 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar7) != 0) {
      lVar7 = *(longlong *)(in_RDX + 0x1e0) + lVar7;
      puVar4 = *(undefined8 **)(lVar7 + 0x10);
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX != (longlong *)0x0) {
        lVar5 = *extraout_RAX;
        lVar6 = extraout_RAX[1];
        *extraout_RAX = *extraout_RAX + 1;
        *(int *)(lVar7 + 0x5c) = iVar2;
        if ((puVar4[1] != 0) && (puVar4[1] * 0x11 != -0x21)) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        puVar4[2] = 0;
        puVar4[3] = 0;
        *puVar4 = &anon_e77b929c0c47cd0bfa6e3db7ab2d1a37_1_llvm_3084324875566536296;
        puVar4[1] = 0;
        puVar4[4] = lVar5;
        puVar4[5] = lVar6;
        puVar4[6] = 0;
        *(int *)(in_RCX + 0x148) = iVar2;
        return;
      }
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
  }
  local_88 = in_RCX + 0xb0;
  local_98 = 
  "country_core::resources::tree_storage::TreeStorageResource requested by  does not exist: ";
  local_90 = 0x32;
  local_80 = alloc::string::impl$23::fmt;
  local_78 = &local_98;
  local_70 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  local_68 = &DAT_142eef7b0;
  local_60 = 2;
  local_48 = 0;
  local_58 = &local_88;
  local_50 = 2;
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}

