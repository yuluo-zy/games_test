
void __cdecl
bevy_ecs::system::schedule_system::impl$1::
run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void_(*)(bevy_ecs::system::query::Query<tuple_<ref_mut_<utils::components::transform::Transform>,ref_<country_core::systems::look_at_camera::LookAtCamera>_>,tuple_<>_>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void_(*)(bevy_ecs::system::query::Query<tuple_<ref_mut_<utils::components::transform::Transform>,ref_<country_core::systems::look_at_camera::LookAtCamera>_>,tuple_<>_>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)>_>
          (void)

{
  int *piVar1;
  int iVar2;
  ulonglong uVar3;
  longlong in_RCX;
  longlong in_RDX;
  char *local_40;
  undefined8 local_38;
  longlong local_30;
  code *local_28;
  char **local_20;
  code *local_18;
  
  LOCK();
  piVar1 = (int *)(in_RDX + 0x5b0);
  iVar2 = *piVar1;
  *piVar1 = *piVar1 + 1;
  UNLOCK();
  if (SBORROW8(0,*(longlong *)(in_RCX + 0x230))) {
                    // WARNING: Subroutine does not return
    core::option::expect_failed();
  }
  if ((*(ulonglong *)(in_RCX + 0x260) < *(ulonglong *)(in_RDX + 0x218)) &&
     (uVar3 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + *(ulonglong *)(in_RCX + 0x260) * 8),
     uVar3 != 0)) {
    if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + ~uVar3 * 0x60) != 0) {
      country_core::systems::look_at_camera::look_at_camera();
      *(int *)(in_RCX + 0x130) = iVar2;
      return;
    }
  }
  local_30 = in_RCX + 0x98;
  local_40 = "country_core::systems::camera_rig::MainCameraResource requested by  does not exist: ";
  local_38 = 0x2d;
  local_28 = alloc::string::impl$23::fmt;
  local_20 = &local_40;
  local_18 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}

