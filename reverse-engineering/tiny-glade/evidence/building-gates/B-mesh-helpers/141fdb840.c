
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
core::ops::function::impls::impl$4::
call_once<tuple_<tuple_<usize,ref_<f32>_>_>,system_wall_constructor::construct_gates::door_mesh_gap::closure_env_0>
          (void)

{
  ulonglong uVar1;
  float fVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  longlong lVar5;
  undefined8 *puVar6;
  ulonglong uVar7;
  uint extraout_EAX;
  undefined8 *in_RCX;
  ulonglong uVar8;
  longlong *in_RDX;
  longlong in_R8;
  float *in_R9;
  float extraout_XMM0_Da;
  float fVar9;
  float fVar10;
  float fVar11;
  float local_7c;
  float *local_78;
  code *local_70;
  undefined **local_68;
  undefined8 local_60;
  undefined4 **local_58;
  undefined8 local_50;
  undefined8 local_48;
  
  lVar5 = *in_RDX;
  puVar6 = (undefined8 *)in_RDX[1];
  fVar2 = *in_R9;
  local_7c = fVar2;
  if ((fVar2 < 0.0) || (__real_3f800000 < fVar2)) {
    local_78 = &local_7c;
    local_70 = core::fmt::float::impl$5::fmt;
    local_68 = &anon_95101f4d2e0328d894274322bc39dc37_3_llvm_14090887955549727255;
    local_60 = 1;
    local_48 = 0;
    local_58 = &local_78;
    local_50 = 1;
                    /* WARNING: Subroutine does not return */
    panicking::panic_fmt();
  }
  if (NAN(fVar2)) {
                    /* WARNING: Subroutine does not return */
    panicking::panic();
  }
  ::utils::curve::Curve::
  get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
            ();
  uVar7 = *(ulonglong *)(lVar5 + 0x10);
  uVar8 = (ulonglong)extraout_EAX;
  if (uVar8 < uVar7) {
    uVar1 = uVar8 + 1;
    if (uVar1 < uVar7) {
      lVar5 = *(longlong *)(lVar5 + 8);
      fVar9 = *(float *)(lVar5 + 8 + uVar8 * 0xc);
      uVar3 = *(undefined8 *)(lVar5 + uVar8 * 0xc);
      uVar4 = *(undefined8 *)(lVar5 + uVar1 * 0xc);
      fVar10 = (float)uVar3;
      fVar11 = (float)((ulonglong)uVar3 >> 0x20);
      fVar10 = extraout_XMM0_Da * ((float)uVar4 - fVar10) + fVar10;
      fVar11 = extraout_XMM0_Da * ((float)((ulonglong)uVar4 >> 0x20) - fVar11) + fVar11;
      fVar9 = (*(float *)(lVar5 + 8 + uVar1 * 0xc) - fVar9) * extraout_XMM0_Da + fVar9;
      if (in_R8 == 1) {
        uVar3 = *puVar6;
        fVar10 = fVar10 + (float)uVar3 * ___xmm_0000000000000000bdcccccdbdcccccd;
        fVar11 = fVar11 + (float)((ulonglong)uVar3 >> 0x20) * _UNK_142c1c4f4;
        fVar9 = fVar9 + *(float *)(puVar6 + 1) * __real_bdcccccd;
      }
      *in_RCX = CONCAT44(fVar11,fVar10);
      *(float *)(in_RCX + 1) = fVar9;
      *(float *)((longlong)in_RCX + 0xc) = fVar2;
      return;
    }
                    /* WARNING: Subroutine does not return */
    panicking::panic_bounds_check();
  }
                    /* WARNING: Subroutine does not return */
  panicking::panic_bounds_check();
}

