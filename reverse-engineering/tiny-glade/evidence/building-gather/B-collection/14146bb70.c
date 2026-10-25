
void __cdecl
utils::curve::Curve::
crashy_resample_remap_per_segment_2<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,country_core::systems::wall::wall_state::publish_public_wall_state::process::closure_env_0>
          (void)

{
  longlong *plVar1;
  undefined1 *puVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  bool bVar8;
  longlong *extraout_RAX;
  ulonglong uVar9;
  byte bVar10;
  longlong *in_RCX;
  longlong *plVar11;
  longlong in_RDX;
  longlong *plVar12;
  longlong *plVar13;
  byte in_R8B;
  ulonglong uVar14;
  float fVar15;
  float fVar16;
  float in_XMM3_Da;
  undefined1 local_1a8 [8];
  longlong lStack_1a0;
  longlong local_190;
  undefined4 uStack_188;
  undefined4 uStack_184;
  longlong local_180;
  longlong lStack_178;
  longlong local_170;
  longlong lStack_168;
  longlong local_160;
  longlong *local_158;
  longlong local_150;
  undefined **local_148;
  longlong *plStack_140;
  undefined1 **local_138;
  undefined8 local_130;
  undefined8 local_128;
  longlong *local_118;
  longlong *local_110;
  undefined1 *local_108;
  code *pcStack_100;
  undefined1 *local_f8;
  code *local_f0;
  byte local_e2;
  undefined1 local_e1;
  undefined8 local_e0;
  
  local_e0 = 0xfffffffffffffffe;
  puVar2 = *(undefined1 **)(in_RDX + 0x10);
  if (puVar2 < (undefined1 *)0x2) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  if ((in_XMM3_Da == 0.0) && (!NAN(in_XMM3_Da))) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  if ((*(float *)(in_RDX + 0x30) == 0.0) && (!NAN(*(float *)(in_RDX + 0x30)))) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  if (((undefined1 *)0xaaaaaaaaaaaaaaa < puVar2) ||
     (local_158 = in_RCX, local_e2 = in_R8B, std::alloc::__default_lib_allocator::__rust_alloc(),
     extraout_RAX == (longlong *)0x0)) {
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error();
  }
  local_f8 = (undefined1 *)0x0;
  plVar12 = *(longlong **)(in_RDX + 8);
  local_148 = (undefined **)*plVar12;
  local_e1 = 1;
  local_150 = in_RDX;
  local_108 = puVar2;
  pcStack_100 = (code *)extraout_RAX;
  country_core::resources::terrain_height::TerrainHeightsData::
  sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
  local_e1 = 1;
  plVar11 = &local_190;
  swizzlers::impl$1::xvy();
  *(undefined4 *)(extraout_RAX + 1) = uStack_188;
  *extraout_RAX = local_190;
  fVar7 = __real_bf800000;
  fVar6 = __real_5f7fffff;
  fVar5 = __real_5f000000;
  fVar4 = __real_322bcc77;
  fVar3 = __real_3f800000;
  local_f8 = (undefined1 *)0x1;
  local_110 = plVar12 + (longlong)puVar2;
  bVar8 = true;
  while (plVar12 != local_110) {
    plVar1 = plVar12 + 1;
    if (bVar8) {
      if (plVar1 == local_110) break;
      local_118 = plVar12 + 2;
      plVar13 = plVar12;
      plVar11 = plVar1;
    }
    else {
      local_118 = plVar1;
      plVar13 = plVar11;
      plVar11 = plVar12;
    }
    fVar15 = (float)*plVar11 - (float)*plVar13;
    fVar16 = (float)((ulonglong)*plVar11 >> 0x20) - (float)((ulonglong)*plVar13 >> 0x20);
    fVar15 = SQRT(fVar16 * fVar16 + fVar15 * fVar15);
    if (fVar15 <= fVar4) {
      fVar15 = fVar4;
    }
    fVar16 = in_XMM3_Da / fVar15;
    if (fVar3 <= in_XMM3_Da / fVar15) {
      fVar16 = fVar3;
    }
    fVar15 = (float)roundf(fVar3 / fVar16);
    uVar9 = (longlong)(fVar15 - fVar5) & (longlong)fVar15 >> 0x3f | (longlong)fVar15;
    if (fVar15 < 0.0) {
      uVar9 = 0;
    }
    if (fVar6 < fVar15) {
      uVar9 = 0xffffffffffffffff;
    }
    uVar9 = uVar9 + 1;
    if (uVar9 < 3) {
      uVar9 = 2;
    }
    bVar10 = (uVar9 & 1) == 0 & local_e2;
    uVar14 = bVar10 | uVar9;
    if ((longlong)uVar14 < 0) {
      fVar15 = (float)(uVar9 >> 1 | (ulonglong)((uint)bVar10 | (uint)uVar9 & 1));
      fVar15 = fVar15 + fVar15;
    }
    else {
      fVar15 = (float)(longlong)uVar14;
    }
    for (uVar9 = 1; uVar9 < uVar14; uVar9 = uVar9 + 1) {
      fVar16 = (float)uVar9 / (fVar15 + fVar7);
      local_148 = (undefined **)
                  CONCAT44(fVar16 * (float)((ulonglong)*plVar11 >> 0x20) +
                           (fVar3 - fVar16) * (float)((ulonglong)*plVar13 >> 0x20),
                           fVar16 * (float)*plVar11 + (fVar3 - fVar16) * (float)*plVar13);
      local_e1 = 1;
      country_core::resources::terrain_height::TerrainHeightsData::
      sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
      local_e1 = 1;
      swizzlers::impl$1::xvy();
      puVar2 = local_f8;
      if (local_f8 == local_108) {
        local_e1 = 1;
        alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
      }
      *(undefined4 *)((longlong)pcStack_100 + (longlong)puVar2 * 0xc + 8) = uStack_188;
      *(longlong *)((longlong)pcStack_100 + (longlong)puVar2 * 0xc) = local_190;
      local_f8 = puVar2 + 1;
    }
    bVar8 = false;
    plVar12 = local_118;
  }
  local_138 = (undefined1 **)local_f8;
  local_148 = (undefined **)local_108;
  plStack_140 = (longlong *)pcStack_100;
  local_e1 = 0;
  try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>();
  if (!SBORROW8(0,local_190)) {
    local_158[6] = local_160;
    local_158[4] = local_170;
    local_158[5] = lStack_168;
    local_158[2] = local_180;
    local_158[3] = lStack_178;
    *local_158 = local_190;
    local_158[1] = CONCAT44(uStack_184,uStack_188);
    return;
  }
  lStack_1a0 = local_180;
  local_108 = local_1a8;
  pcStack_100 = impl$46::fmt;
  local_f8 = (undefined1 *)local_150;
  local_f0 = alloc::vec::impl$27::fmt<glam::f32::vec2::Vec2,alloc::alloc::Global>;
  local_148 = &PTR_s_Curve_per_segment_resampling_fai_142c63ab8;
  plStack_140 = (longlong *)0x2;
  local_128 = 0;
  local_138 = &local_108;
  local_130 = 2;
                    /* WARNING: Subroutine does not return */
  core::panicking::panic_fmt();
}

