
void __cdecl
bevy_ecs::system::schedule_system::impl$1::
run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void_(*)(bevy_ecs::event::reader::EventReader<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<enum2_<country_core::systems::ui::resources::widget_params::PlayUiSound>_>,bevy_ecs::system::system_param::Local<system_glade_border::feedback_input_out_of_border::InputOfOutBorderCache>),void_(*)(bevy_ecs::event::reader::EventReader<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<enum2_<country_core::systems::ui::resources::widget_params::PlayUiSound>_>,bevy_ecs::system::system_param::Local<system_glade_border::feedback_input_out_of_border::InputOfOutBorderCache>)>_>
          (void)

{
  int *piVar1;
  int iVar2;
  ulonglong uVar3;
  longlong lVar4;
  ulonglong uVar5;
  undefined8 uVar6;
  longlong lVar7;
  longlong in_RCX;
  longlong in_RDX;
  ulonglong uVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  longlong lVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  char *local_90;
  undefined8 local_88;
  longlong local_80;
  code *local_78;
  char **local_70;
  code *local_68;
  undefined *local_60;
  undefined8 local_58;
  longlong *local_50;
  undefined8 local_48;
  undefined8 local_40;
  
  LOCK();
  piVar1 = (int *)(in_RDX + 0x5b0);
  iVar2 = *piVar1;
  *piVar1 = *piVar1 + 1;
  UNLOCK();
  if (*(char *)(in_RCX + 0x158) == '\x02') {
                    // WARNING: Subroutine does not return
    core::option::expect_failed();
  }
  if (*(ulonglong *)(in_RCX + 0x148) < *(ulonglong *)(in_RDX + 0x218)) {
    uVar3 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + *(ulonglong *)(in_RCX + 0x148) * 8);
    if (uVar3 != 0) {
      lVar4 = *(longlong *)(in_RDX + 0x1e0);
      lVar11 = ~uVar3 * 0x60;
      if (*(longlong *)(lVar4 + 0x20 + lVar11) != 0) {
        if (((*(ulonglong *)(in_RCX + 0x150) < *(ulonglong *)(in_RDX + 0x218)) &&
            (uVar3 = *(ulonglong *)
                      (*(longlong *)(in_RDX + 0x210) + *(ulonglong *)(in_RCX + 0x150) * 8),
            uVar3 != 0)) && (lVar7 = ~uVar3 * 0x60, *(longlong *)(lVar4 + 0x20 + lVar7) != 0)) {
          lVar11 = *(longlong *)(lVar11 + lVar4 + 0x10);
          lVar7 = lVar7 + lVar4;
          lVar4 = *(longlong *)(lVar7 + 0x10);
          uVar3 = *(ulonglong *)(in_RCX + 0x140);
          uVar9 = 0;
          uVar10 = uVar3 - *(ulonglong *)(lVar11 + 0x18);
          if (uVar3 < *(ulonglong *)(lVar11 + 0x18)) {
            uVar10 = uVar9;
          }
          uVar8 = uVar3 - *(ulonglong *)(lVar11 + 0x38);
          if (uVar3 < *(ulonglong *)(lVar11 + 0x38)) {
            uVar8 = uVar9;
          }
          uVar3 = *(ulonglong *)(lVar11 + 0x30);
          uVar12 = uVar3 - uVar8;
          if (uVar3 < uVar8) {
            uVar12 = uVar9;
          }
          uVar5 = *(ulonglong *)(lVar11 + 0x10);
          uVar13 = uVar5 - uVar10;
          if (uVar5 < uVar10) {
            uVar13 = uVar9;
          }
          lVar11 = *(longlong *)(lVar11 + 0x40);
          *(ulonglong *)(in_RCX + 0x140) = lVar11 - (uVar13 + uVar12);
          if (uVar5 >= uVar10 && uVar5 - uVar10 != 0 || uVar3 >= uVar8 && uVar3 - uVar8 != 0) {
            *(longlong *)(in_RCX + 0x140) = lVar11;
            uVar9 = 1;
            if (*(char *)(in_RCX + 0x158) == '\0') {
              *(int *)(lVar7 + 0x5c) = iVar2;
              lVar11 = *(longlong *)(lVar4 + 0x30);
              uVar6 = *(undefined8 *)(lVar4 + 0x40);
              if (lVar11 == *(longlong *)(lVar4 + 0x20)) {
                alloc::raw_vec::RawVec::
                grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                          ();
              }
              lVar7 = *(longlong *)(lVar4 + 0x28);
              *(undefined4 *)(lVar7 + lVar11 * 0x20) = 4;
              *(undefined8 *)(lVar7 + 0x18 + lVar11 * 0x20) = uVar6;
              *(longlong *)(lVar4 + 0x30) = lVar11 + 1;
              *(longlong *)(lVar4 + 0x40) = *(longlong *)(lVar4 + 0x40) + 1;
            }
          }
          *(char *)(in_RCX + 0x158) = (char)uVar9;
          *(int *)(in_RCX + 0x130) = iVar2;
          return;
        }
        local_80 = in_RCX + 0x98;
        local_90 = 
        "bevy_ecs::event::collections::Events<country_core::systems::ui::resources::widget_params::PlayUiSound>"
        ;
        local_88 = 0x66;
        local_78 = alloc::string::impl$23::fmt;
        local_70 = &local_90;
        local_68 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
        local_60 = &DAT_142efaa30;
        local_58 = 2;
        local_40 = 0;
        local_50 = &local_80;
        local_48 = 2;
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
    }
  }
  local_80 = in_RCX + 0x98;
  local_90 = 
  "bevy_ecs::event::collections::Events<country_core::resources::glade_border::FeedbackInputOutOfBorder>Resource requested by  does not exist: "
  ;
  local_88 = 0x65;
  local_78 = alloc::string::impl$23::fmt;
  local_70 = &local_90;
  local_68 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  local_60 = &DAT_142efaa30;
  local_58 = 2;
  local_40 = 0;
  local_50 = &local_80;
  local_48 = 2;
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}

