
void __cdecl
utils::curve::curve2_serialized_as_curve3::deserialize<enum2_<serde_json::value::Value>_>(void)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  longlong extraout_RAX;
  ulonglong uVar6;
  longlong *in_RCX;
  ulonglong uVar7;
  ulonglong uVar8;
  longlong lVar9;
  longlong local_d0;
  longlong lStack_c8;
  longlong local_c0;
  longlong lStack_b8;
  longlong local_b0;
  ulonglong uStack_a8;
  undefined4 local_a0;
  
  serde_json::value::de::impl$2::
  deserialize_struct<utils::curve::_::impl_0::deserialize::__Visitor<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>_>
            ();
  if (SBORROW8(0,local_d0)) {
    in_RCX[1] = lStack_c8;
    *in_RCX = -0x8000000000000000;
    return;
  }
  alloc::vec::spec_from_iter::impl$0::
  from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3>_>,glam::f32::vec2::Vec2_(*)(glam::f32::vec3::Vec3)>_>
            ();
  if (uStack_a8 == 0) {
    lVar9 = 4;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    lVar9 = extraout_RAX;
    if (uStack_a8 < 8 || (ulonglong)(extraout_RAX - local_b0) < 0x20) {
      uVar6 = 0;
    }
    else {
      uVar6 = uStack_a8 & 0xfffffffffffffff8;
      uVar8 = 0;
      do {
        puVar1 = (undefined8 *)(local_b0 + uVar8 * 4);
        uVar3 = puVar1[1];
        puVar2 = (undefined8 *)(local_b0 + 0x10 + uVar8 * 4);
        uVar4 = *puVar2;
        uVar5 = puVar2[1];
        puVar2 = (undefined8 *)(extraout_RAX + uVar8 * 4);
        *puVar2 = *puVar1;
        puVar2[1] = uVar3;
        puVar1 = (undefined8 *)(extraout_RAX + 0x10 + uVar8 * 4);
        *puVar1 = uVar4;
        puVar1[1] = uVar5;
        uVar8 = uVar8 + 8;
      } while (uVar6 != uVar8);
      if (uStack_a8 == uVar6) goto LAB_14128592b;
    }
    uVar7 = uVar6;
    for (uVar8 = uStack_a8 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
      *(undefined4 *)(extraout_RAX + uVar7 * 4) = *(undefined4 *)(local_b0 + uVar7 * 4);
      uVar7 = uVar7 + 1;
    }
    if (uVar6 - uStack_a8 < 0xfffffffffffffffd) {
      do {
        *(undefined4 *)(extraout_RAX + uVar7 * 4) = *(undefined4 *)(local_b0 + uVar7 * 4);
        *(undefined4 *)(extraout_RAX + 4 + uVar7 * 4) = *(undefined4 *)(local_b0 + 4 + uVar7 * 4);
        *(undefined4 *)(extraout_RAX + 8 + uVar7 * 4) = *(undefined4 *)(local_b0 + 8 + uVar7 * 4);
        *(undefined4 *)(extraout_RAX + 0xc + uVar7 * 4) =
             *(undefined4 *)(local_b0 + 0xc + uVar7 * 4);
        uVar7 = uVar7 + 4;
      } while (uStack_a8 != uVar7);
    }
  }
LAB_14128592b:
  if (local_d0 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (lStack_b8 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  in_RCX[2] = local_c0;
  *in_RCX = local_d0;
  in_RCX[1] = lStack_c8;
  in_RCX[3] = uStack_a8;
  in_RCX[4] = lVar9;
  in_RCX[5] = uStack_a8;
  *(undefined4 *)(in_RCX + 6) = local_a0;
  return;
}

