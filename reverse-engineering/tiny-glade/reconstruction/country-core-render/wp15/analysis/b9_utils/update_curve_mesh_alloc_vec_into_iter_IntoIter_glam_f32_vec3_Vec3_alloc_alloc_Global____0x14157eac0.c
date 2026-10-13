// addr=0x14157eac0 name=update_curve_mesh<alloc::vec::into_iter::IntoIter<glam::f32::vec3::Vec3,alloc::alloc::Global>_>
// signature=void __cdecl update_curve_mesh<alloc::vec::into_iter::IntoIter<glam::f32::vec3::Vec3,alloc::alloc::Global>_>(void)


void __cdecl
country_core::utils::geo::
update_curve_mesh<alloc::vec::into_iter::IntoIter<glam::f32::vec3::Vec3,alloc::alloc::Global>_>
          (void)

{
  int *piVar1;
  undefined4 uVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  ulonglong *extraout_RAX;
  ulonglong extraout_RAX_00;
  longlong *in_RCX;
  ulonglong uVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  longlong lVar9;
  ulonglong uVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  int iVar14;
  
  lVar3 = in_RCX[1];
  lVar4 = in_RCX[3];
  uVar7 = lVar4 - lVar3;
  asset_libraries::mesh_library::AssetMeshLibrary::get_mut();
  if (extraout_RAX == (ulonglong *)0x0) {
                    // WARNING: Subroutine does not return
    core::option::expect_failed();
  }
  uVar8 = uVar7 / 0xc;
  lVar5 = *in_RCX;
  if (lVar4 != lVar3) {
    if (uVar7 - 0xc < 0xc) {
      uVar10 = 0;
    }
    else {
      lVar9 = 0x14;
      uVar10 = 0;
      do {
        uVar2 = *(undefined4 *)(lVar3 + -0xc + lVar9);
        *(undefined8 *)(lVar5 + -0x14 + lVar9) = *(undefined8 *)(lVar3 + -0x14 + lVar9);
        *(undefined4 *)(lVar5 + -0xc + lVar9) = uVar2;
        uVar10 = uVar10 + 2;
        uVar2 = *(undefined4 *)(lVar3 + lVar9);
        *(undefined8 *)(lVar5 + -8 + lVar9) = *(undefined8 *)(lVar3 + -8 + lVar9);
        *(undefined4 *)(lVar5 + lVar9) = uVar2;
        lVar9 = lVar9 + 0x18;
      } while ((uVar8 & 0xfffffffffffffffe) != uVar10);
    }
    if ((uVar8 & 1) != 0) {
      lVar9 = uVar10 * 0xc;
      uVar2 = *(undefined4 *)(lVar3 + 8 + lVar9);
      *(undefined8 *)(lVar5 + lVar9) = *(undefined8 *)(lVar3 + lVar9);
      *(undefined4 *)(lVar5 + 8 + lVar9) = uVar2;
    }
  }
  country_core::render::mesh::Mesh::
  set_attribute<alloc::vec::Vec<array_<f32,3>,alloc::alloc::Global>_>();
  alloc::vec::from_elem<array_<f32,3>_>();
  country_core::render::mesh::Mesh::
  set_attribute<alloc::vec::Vec<array_<f32,3>,alloc::alloc::Global>_>();
  if (lVar4 == lVar3) {
    uVar8 = 0;
    uVar10 = 4;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_00 == 0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    uVar10 = extraout_RAX_00;
    if (uVar7 < 0x60) {
      uVar7 = 0;
    }
    else {
      uVar7 = uVar8 & 0xfffffffffffffff8;
      iVar11 = 0;
      iVar12 = 1;
      iVar13 = 2;
      iVar14 = 3;
      uVar6 = 0;
      do {
        piVar1 = (int *)(extraout_RAX_00 + uVar6 * 4);
        *piVar1 = iVar11;
        piVar1[1] = iVar12;
        piVar1[2] = iVar13;
        piVar1[3] = iVar14;
        piVar1 = (int *)(extraout_RAX_00 + 0x10 + uVar6 * 4);
        *piVar1 = iVar11 + 4;
        piVar1[1] = iVar12 + 4;
        piVar1[2] = iVar13 + 4;
        piVar1[3] = iVar14 + 4;
        uVar6 = uVar6 + 8;
        iVar11 = iVar11 + 8;
        iVar12 = iVar12 + 8;
        iVar13 = iVar13 + 8;
        iVar14 = iVar14 + 8;
      } while (uVar7 != uVar6);
      if (uVar8 == uVar7) goto LAB_14157ece0;
    }
    do {
      *(int *)(extraout_RAX_00 + uVar7 * 4) = (int)uVar7;
      uVar7 = uVar7 + 1;
    } while (uVar8 != uVar7);
  }
LAB_14157ece0:
  if (*extraout_RAX != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  *extraout_RAX = uVar8;
  extraout_RAX[1] = uVar10;
  extraout_RAX[2] = uVar8;
  return;
}


