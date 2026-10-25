// address 0x1412a90c0
// name attrib<glam::f32::vec3::Vec3>
// signature void __cdecl attrib<glam::f32::vec3::Vec3>(void)

void __cdecl
country_core::render::nani::trimesh::TrimeshBuilder::attrib<glam::f32::vec3::Vec3>(void)

{
  longlong lVar1;
  longlong lVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  longlong lVar10;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  undefined8 in_R8;
  longlong in_R9;
  
  if (in_R9 == 0) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  if (in_R9 <= (longlong)in_RDX[8]) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  if (in_RDX[9] == 0) {
    in_RDX[9] = in_R9;
  }
  else if (in_RDX[9] != in_R9) {
                    // WARNING: Subroutine does not return
    core::panicking::assert_failed<usize,usize>();
  }
  lVar10 = in_RDX[6];
  lVar2 = in_RDX[7];
  lVar1 = lVar10 + lVar2 * 0x18;
  if ((lVar1 != 0x18 && lVar2 != 0) && (*(byte *)(lVar1 + -6) < 4)) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  if (lVar2 == in_RDX[5]) {
    alloc::raw_vec::RawVec::
    grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
              ();
    lVar10 = in_RDX[6];
  }
  *(undefined8 *)(lVar10 + lVar2 * 0x18) = in_R8;
  *(longlong *)(lVar10 + 8 + lVar2 * 0x18) = in_R9 * 0xc;
  *(undefined2 *)(lVar10 + 0x10 + lVar2 * 0x18) = 0xc;
  *(undefined1 *)(lVar10 + 0x12 + lVar2 * 0x18) = 4;
  in_RDX[7] = lVar2 + 1;
  uVar3 = *in_RDX;
  uVar4 = in_RDX[1];
  uVar5 = in_RDX[2];
  uVar6 = in_RDX[3];
  uVar7 = in_RDX[4];
  uVar8 = in_RDX[5];
  uVar9 = in_RDX[9];
  in_RCX[8] = in_RDX[8];
  in_RCX[9] = uVar9;
  in_RCX[4] = uVar7;
  in_RCX[5] = uVar8;
  in_RCX[2] = uVar5;
  in_RCX[3] = uVar6;
  *in_RCX = uVar3;
  in_RCX[1] = uVar4;
  in_RCX[6] = in_RDX[6];
  in_RCX[7] = in_RDX[7];
  return;
}

