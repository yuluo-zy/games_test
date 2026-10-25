
void __cdecl
bevy_ecs::system::schedule_system::impl$1::
run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void_(*)(bevy_ecs::change_detection::ResMut<enum2_<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState>_>_>),tiny_glade::run_game::run_game_thread::closure_env_2>_>
          (void)

{
  int *piVar1;
  int iVar2;
  ulonglong uVar3;
  byte *in_RCX;
  longlong in_RDX;
  longlong lVar4;
  char *local_60;
  undefined8 local_58;
  byte *local_50;
  code *local_48;
  char **local_40;
  code *local_38;
  undefined *local_30;
  undefined8 local_28;
  byte **local_20;
  undefined8 local_18;
  undefined8 local_10;
  
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
    lVar4 = ~uVar3 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar4) != 0) {
      lVar4 = *(longlong *)(in_RDX + 0x1e0) + lVar4;
      *(int *)(lVar4 + 0x5c) = iVar2;
      **(undefined1 **)(lVar4 + 0x10) = 1;
      *(int *)(in_RCX + 0x148) = iVar2;
      return;
    }
  }
  local_50 = in_RCX + 0xb0;
  local_60 = 
  "bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState>Resource requested by  does not exist: "
  ;
  local_58 = 0x59;
  local_48 = alloc::string::impl$23::fmt;
  local_40 = &local_60;
  local_38 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  local_30 = &DAT_1429be840;
  local_28 = 2;
  local_10 = 0;
  local_20 = &local_50;
  local_18 = 2;
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}


