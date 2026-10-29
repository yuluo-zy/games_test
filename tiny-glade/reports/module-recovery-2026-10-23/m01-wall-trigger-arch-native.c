
void __cdecl country_core::systems::collision::world::construct_arch_geo(void)

{
  ulonglong uVar1;
  longlong *plVar2;
  undefined4 uVar3;
  longlong lVar4;
  longlong lVar5;
  undefined8 uVar6;
  longlong lVar7;
  longlong lVar8;
  ulonglong uVar9;
  undefined8 *puVar10;
  longlong in_RCX;
  ulonglong uVar11;
  longlong lVar12;
  longlong *in_RDX;
  longlong *in_R8;
  bool bVar13;
  longlong local_188;
  undefined4 uStack_17c;
  undefined4 uStack_178;
  undefined4 uStack_174;
  undefined8 *puStack_170;
  longlong local_d0;
  longlong local_c0;
  undefined8 local_a0;
  undefined8 uStack_98;
  undefined8 local_90;
  undefined8 uStack_88;
  undefined4 local_80;
  longlong local_7c;
  longlong lStack_74;
  undefined8 local_6c;
  undefined8 *puStack_64;
  undefined4 local_5c;
  
  lVar4 = *(longlong *)(in_RCX + 8);
  lVar5 = *(longlong *)(in_RCX + 0x20);
  uVar11 = *(ulonglong *)(in_RCX + 0x28);
  if (*(ulonglong *)(in_RCX + 0x10) <= *(ulonglong *)(in_RCX + 0x28)) {
    uVar11 = *(ulonglong *)(in_RCX + 0x10);
  }
  local_d0 = 0;
  if (uVar11 != 0) {
    uVar9 = 0;
    do {
      uVar1 = uVar9 + 1;
      lVar12 = uVar9 * 4 + lVar4;
      puVar10 = (undefined8 *)(uVar9 * 0xc + lVar5);
      bVar13 = local_d0 == 0;
      local_d0 = local_c0;
      if (bVar13) {
        itertools::tuple_impl::impl$31::
        collect_from_iter_no_buf<tuple_<ref_<f32>,ref_<glam::f32::vec3::Vec3>_>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple_<ref_<f32>,ref_<glam::f32::vec3::Vec3>_>_>,ref_mut_<core::iter::adapters::zip::Zip<core::slice::iter::Iter<f32>,core::slice::iter::Iter<glam::f32::vec3::Vec3>_>_>_>_>
                  ();
        lVar12 = CONCAT44(uStack_174,uStack_178);
        puVar10 = puStack_170;
        local_d0 = local_188;
        if (local_188 == 0) {
          return;
        }
      }
      uVar3 = *(undefined4 *)(puVar10 + 1);
      uVar6 = *puVar10;
      ::utils::geometry::intersect::Triangle::new();
      ::utils::geometry::intersect::Triangle::normal();
      uStack_17c = (undefined4)uVar6;
      uStack_178 = (undefined4)((ulonglong)uVar6 >> 0x20);
      lVar7 = in_RDX[2];
      if (lVar7 == *in_RDX) {
        alloc::raw_vec::RawVec::grow_one<array_<glam::f32::vec3::Vec3,3>,alloc::alloc::Global>();
      }
      lVar8 = in_RDX[1];
      *(undefined4 *)(lVar8 + 0x20 + lVar7 * 0x24) = uVar3;
      puVar10 = (undefined8 *)(lVar8 + 0x10 + lVar7 * 0x24);
      *puVar10 = CONCAT44(uVar3,uStack_178);
      puVar10[1] = uVar6;
      puVar10 = (undefined8 *)(lVar8 + lVar7 * 0x24);
      *puVar10 = uVar6;
      puVar10[1] = CONCAT44(uStack_17c,uVar3);
      in_RDX[2] = lVar7 + 1;
      lVar7 = in_R8[2];
      if (lVar7 == *in_R8) {
        alloc::raw_vec::RawVec::grow_one<array_<glam::f32::vec3::Vec3,3>,alloc::alloc::Global>();
      }
      lVar8 = in_R8[1];
      *(undefined4 *)(lVar8 + 0x20 + lVar7 * 0x24) = local_80;
      puVar10 = (undefined8 *)(lVar8 + 0x10 + lVar7 * 0x24);
      *puVar10 = local_90;
      puVar10[1] = uStack_88;
      puVar10 = (undefined8 *)(lVar8 + lVar7 * 0x24);
      *puVar10 = local_a0;
      puVar10[1] = uStack_98;
      in_R8[2] = lVar7 + 1;
      ::utils::geometry::intersect::Triangle::new();
      ::utils::geometry::intersect::Triangle::normal();
      lVar7 = in_RDX[2];
      if (lVar7 == *in_RDX) {
        alloc::raw_vec::RawVec::grow_one<array_<glam::f32::vec3::Vec3,3>,alloc::alloc::Global>();
      }
      lVar8 = in_RDX[1];
      *(undefined4 *)(lVar8 + 0x20 + lVar7 * 0x24) = uVar3;
      puVar10 = (undefined8 *)(lVar8 + 0x10 + lVar7 * 0x24);
      *puVar10 = CONCAT44(uVar3,uStack_178);
      puVar10[1] = uVar6;
      puVar10 = (undefined8 *)(lVar8 + lVar7 * 0x24);
      *puVar10 = uVar6;
      puVar10[1] = CONCAT44(uStack_17c,uVar3);
      in_RDX[2] = lVar7 + 1;
      uStack_178 = (undefined4)local_6c;
      uStack_174 = (undefined4)((ulonglong)local_6c >> 0x20);
      lVar7 = in_R8[2];
      if (lVar7 == *in_R8) {
        alloc::raw_vec::RawVec::grow_one<array_<glam::f32::vec3::Vec3,3>,alloc::alloc::Global>();
      }
      puStack_170 = puStack_64;
      local_188 = local_7c;
      lVar8 = in_R8[1];
      *(undefined4 *)(lVar8 + 0x20 + lVar7 * 0x24) = local_5c;
      puVar10 = (undefined8 *)(lVar8 + 0x10 + lVar7 * 0x24);
      *puVar10 = local_6c;
      puVar10[1] = puStack_64;
      plVar2 = (longlong *)(lVar8 + lVar7 * 0x24);
      *plVar2 = local_7c;
      plVar2[1] = lStack_74;
      in_R8[2] = lVar7 + 1;
      uVar9 = uVar1;
      local_c0 = lVar12;
    } while (uVar1 < uVar11);
  }
  return;
}


