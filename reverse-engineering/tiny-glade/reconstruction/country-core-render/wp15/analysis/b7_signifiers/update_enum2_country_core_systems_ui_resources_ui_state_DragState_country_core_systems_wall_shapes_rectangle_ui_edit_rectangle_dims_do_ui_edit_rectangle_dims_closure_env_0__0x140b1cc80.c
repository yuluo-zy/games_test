// addr=0x140b1cc80 name=update<enum2$<country_core::systems::ui::resources::ui_state::DragState>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims::closure_env$0>
// signature=void __cdecl update<enum2$<country_core::systems::ui::resources::ui_state::DragState>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims::closure_env$0>(void)


void __cdecl
country_core::systems::ui::resources::slider3d::Slider3d::
update<enum2_<country_core::systems::ui::resources::ui_state::DragState>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims::closure_env_0>
          (void)

{
  float fVar1;
  undefined8 uVar2;
  undefined8 *puVar3;
  undefined8 uVar4;
  uint *in_RCX;
  uint uVar5;
  undefined8 *in_RDX;
  char in_R8B;
  longlong *in_R9;
  byte bVar6;
  float *pfVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float *in_stack_00000028;
  float local_90;
  float fStack_8c;
  float local_88;
  
  puVar3 = (undefined8 *)*in_RDX;
  bVar6 = 3;
  if ((byte)(in_R8B - 2U) < 3) {
    bVar6 = in_R8B - 2U;
  }
  uVar5 = (uint)bVar6;
  if (bVar6 - 2 < 2) {
    fVar12 = *in_stack_00000028;
    fVar11 = in_stack_00000028[1];
    ::utils::mouse_ss_to_cs();
    country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::project_onto_ray();
    if (fVar11 < fVar12) {
                    // WARNING: Subroutine does not return
      core::f32::impl$0::clamp::do_panic::runtime();
    }
    fVar1 = *(float *)((longlong)puVar3 + 0x14);
    fVar8 = (float)*(undefined8 *)((longlong)puVar3 + 0xc);
    fVar9 = (float)((ulonglong)*(undefined8 *)((longlong)puVar3 + 0xc) >> 0x20);
    fVar10 = (local_88 * fVar1 + fStack_8c * fVar9 + local_90 * fVar8) - *(float *)(puVar3 + 3);
    if (fVar12 <= fVar10) {
      fVar12 = fVar10;
    }
    if (fVar12 <= fVar11) {
      fVar11 = fVar12;
    }
    fVar12 = *(float *)(puVar3 + 1);
    *(ulonglong *)(in_RCX + 1) =
         CONCAT44((float)((ulonglong)*puVar3 >> 0x20) + fVar11 * fVar9,
                  (float)*puVar3 + fVar11 * fVar8);
    in_RCX[3] = (uint)(fVar1 * fVar11 + fVar12);
    in_RCX[4] = (uint)fVar11;
    uVar5 = 1;
  }
  else if (bVar6 != 0) {
    pfVar7 = (float *)in_R9[4];
    if ((*(longlong *)in_R9[2] - 1U & 0xfffffffffffffffd) != 0) {
      pfVar7 = (float *)in_R9[3];
    }
    fVar12 = *pfVar7;
    fVar11 = *(float *)((undefined8 *)in_R9[1] + 1);
    uVar2 = *(undefined8 *)in_R9[1];
    uVar4 = *(undefined8 *)*in_R9;
    *(undefined4 *)(puVar3 + 1) = *(undefined4 *)((undefined8 *)*in_R9 + 1);
    *puVar3 = uVar4;
    *(undefined8 *)((longlong)puVar3 + 0xc) = uVar2;
    *(float *)((longlong)puVar3 + 0x14) = fVar11;
    ::utils::mouse_ss_to_cs();
    country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::project_onto_ray();
    *(float *)(puVar3 + 3) =
         (fVar11 * local_88 +
         (float)((ulonglong)uVar2 >> 0x20) * fStack_8c + local_90 * (float)uVar2) - fVar12;
    uVar5 = 0;
  }
  *in_RCX = uVar5;
  return;
}


