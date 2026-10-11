
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
core::iter::adapters::map::impl$2::
fold<f32,core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3>_>,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env_1,tuple_<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env_0<f32,alloc::vec::impl_20::extend_trusted::closure_env_0<f32,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3>_>,system_wall_constructor::construct_arches::profile::create_stone_arch_profile_impl::closure_env_1>_>_>_>
          (void)

{
  float fVar1;
  float fVar2;
  float fVar3;
  char cVar4;
  longlong lVar5;
  longlong lVar6;
  longlong *plVar7;
  float *pfVar8;
  float *pfVar9;
  longlong lVar10;
  longlong lVar11;
  float *pfVar12;
  char *pcVar13;
  longlong lVar14;
  float fVar15;
  float fVar16;
  uint uVar17;
  float fVar18;
  float fVar19;
  longlong *in_RCX;
  undefined8 *in_RDX;
  ulonglong uVar20;
  longlong lVar21;
  longlong lVar22;
  float fVar23;
  float extraout_XMM0_Da;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  
  fVar19 = __real_c0600000;
  fVar18 = __real_bf800000;
  uVar17 = ___xmm_7fffffff7fffffff7fffffff7fffffff;
  fVar16 = __real_3f800000;
  fVar15 = __real_3f000000;
  lVar5 = *in_RCX;
  lVar6 = in_RCX[1];
  plVar7 = (longlong *)*in_RDX;
  lVar21 = in_RDX[1];
  if (lVar5 != lVar6) {
    uVar20 = in_RCX[2];
    pfVar8 = (float *)in_RCX[3];
    pfVar9 = (float *)in_RCX[4];
    lVar10 = in_RCX[5];
    lVar11 = in_RCX[6];
    pfVar12 = (float *)in_RCX[7];
    pcVar13 = (char *)in_RCX[8];
    lVar14 = in_RDX[2];
    lVar22 = 0;
    do {
      if (*(ulonglong *)(lVar10 + 0x10) <= uVar20) {
                    /* WARNING: Subroutine does not return */
        panicking::panic_bounds_check();
      }
      fVar1 = *pfVar8;
      fVar23 = *(float *)(*(longlong *)(lVar10 + 8) + uVar20 * 4) / *pfVar12;
      fVar26 = fVar1 + fVar19;
      fVar2 = 0.0;
      if (0.0 <= fVar23) {
        fVar2 = fVar23;
      }
      fVar23 = fVar16;
      if (fVar2 <= fVar16) {
        fVar23 = fVar2;
      }
      fVar2 = *(float *)(lVar5 + 4 + lVar22 * 0xc);
      fVar3 = *pfVar9;
      fVar23 = fVar23 * fVar15 + (fVar16 - fVar23) * 0.0;
      fVar27 = (float)uVar20 / ((float)*(longlong *)(lVar11 + 0x10) + fVar18);
      if (*pcVar13 == '\0') {
        fVar27 = (float)((uint)(fVar27 + __real_bf000000) & uVar17);
        fVar27 = fVar27 + fVar27;
        fVar23 = fVar16 - (fVar16 - fVar23) * fVar27 * fVar27 * fVar27;
      }
      else {
        cVar4 = pcVar13[1];
        ::utils::inverse_lerp();
        if (cVar4 == '\0') {
          fVar28 = fVar27 * fVar27;
        }
        else {
          fVar28 = (fVar16 - fVar27) * (fVar16 - fVar27);
        }
        fVar23 = fVar16 - fVar23;
        fVar24 = (float)((uint)(fVar27 + __real_bf000000) & uVar17);
        fVar24 = fVar24 + fVar24;
        fVar27 = 0.0;
        if (0.0 <= extraout_XMM0_Da) {
          fVar27 = extraout_XMM0_Da;
        }
        fVar25 = fVar16;
        if (fVar27 <= fVar16) {
          fVar25 = fVar27;
        }
        fVar23 = (fVar16 - fVar25) *
                 (__xmm_00000000000000003f8000003f800000._4_4_ - fVar23 * fVar24 * fVar24 * fVar24)
                 + fVar25 * ((float)__xmm_00000000000000003f8000003f800000 - fVar23 * fVar28);
      }
      fVar27 = fVar26;
      if (fVar26 <= fVar3) {
        fVar27 = fVar3;
      }
      fVar26 = ((float)(~-(uint)NAN(fVar3) & (uint)fVar27 | -(uint)NAN(fVar3) & (uint)fVar26) +
               fVar18) - fVar2;
      if (fVar26 <= 0.0) {
        fVar26 = 0.0;
      }
      *(float *)(lVar14 + lVar21 * 4) = (fVar1 - fVar2) * fVar23 + (fVar16 - fVar23) * fVar26;
      lVar21 = lVar21 + 1;
      uVar20 = uVar20 + 1;
      lVar22 = lVar22 + 1;
    } while (lVar22 != ((ulonglong)(lVar6 - lVar5) >> 2) * -0x5555555555555555);
  }
  *plVar7 = lVar21;
  return;
}

