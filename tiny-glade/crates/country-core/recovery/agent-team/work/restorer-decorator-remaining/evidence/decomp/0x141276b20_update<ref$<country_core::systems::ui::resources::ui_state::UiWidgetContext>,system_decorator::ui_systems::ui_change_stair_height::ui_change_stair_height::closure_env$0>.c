// 0x141276b20
// callees: [{"name": "into_drag_state", "address": "14092df10"}, {"name": "project_cursor_onto_line", "address": "140a9bd40"}, {"name": "runtime", "address": "140d8b480"}]

void __cdecl
country_core::systems::ui::resources::slider3d::Slider3d::
update<ref_<country_core::systems::ui::resources::ui_state::UiWidgetContext>,system_decorator::ui_systems::ui_change_stair_height::ui_change_stair_height::closure_env_0>
          (void)

{
  float fVar1;
  undefined8 *puVar2;
  char extraout_AL;
  byte bVar3;
  uint uVar4;
  uint *in_RCX;
  undefined8 *in_RDX;
  undefined8 *in_R9;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float *in_stack_00000028;
  float *in_stack_00000030;
  float local_58;
  float fStack_54;
  float local_50;
  
  puVar2 = (undefined8 *)*in_RDX;
  slider3d::impl$1::into_drag_state();
  bVar3 = 3;
  if ((byte)(extraout_AL - 2U) < 3) {
    bVar3 = extraout_AL - 2U;
  }
  uVar4 = (uint)bVar3;
  if (bVar3 - 2 < 2) {
    fVar9 = *in_stack_00000030;
    fVar8 = in_stack_00000030[1];
    Slider3dState::project_cursor_onto_line();
    if (fVar8 < fVar9) {
                    // WARNING: Subroutine does not return
      core::f32::impl$0::clamp::do_panic::runtime();
    }
    fVar1 = *(float *)((longlong)puVar2 + 0x14);
    fVar5 = (float)*(undefined8 *)((longlong)puVar2 + 0xc);
    fVar6 = (float)((ulonglong)*(undefined8 *)((longlong)puVar2 + 0xc) >> 0x20);
    fVar7 = (local_50 * fVar1 + fStack_54 * fVar6 + local_58 * fVar5) - *(float *)(puVar2 + 3);
    if (fVar9 <= fVar7) {
      fVar9 = fVar7;
    }
    if (fVar9 <= fVar8) {
      fVar8 = fVar9;
    }
    fVar9 = *(float *)(puVar2 + 1);
    *(ulonglong *)(in_RCX + 1) =
         CONCAT44((float)((ulonglong)*puVar2 >> 0x20) + fVar8 * fVar6,(float)*puVar2 + fVar8 * fVar5
                 );
    in_RCX[3] = (uint)(fVar1 * fVar8 + fVar9);
    in_RCX[4] = (uint)fVar8;
    uVar4 = 1;
  }
  else if (bVar3 != 0) {
    fVar9 = *in_stack_00000028;
    *(undefined4 *)(puVar2 + 1) = *(undefined4 *)(in_R9 + 1);
    *puVar2 = *in_R9;
    *(undefined8 *)((longlong)puVar2 + 0xc) = 0x3f80000000000000;
    *(undefined4 *)((longlong)puVar2 + 0x14) = 0;
    Slider3dState::project_cursor_onto_line();
    *(float *)(puVar2 + 3) = (local_50 * 0.0 + local_58 * 0.0 + fStack_54) - fVar9;
    uVar4 = 0;
  }
  *in_RCX = uVar4;
  return;
}

