
void __cdecl
utils::curve::Curve::
crashy_resample_remap_per_segment_2<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,country_core::systems::wall::wall_state::publish_public_wall_state::process::closure_env_0>
          (void)

{
  longlong *plVar1;
  undefined1 *puVar2;
  bool bVar3;
  longlong *extraout_RAX;
  ulonglong uVar4;
  byte bVar5;
  longlong *in_RCX;
  longlong *plVar6;
  longlong in_RDX;
  longlong *plVar7;
  longlong *plVar8;
  byte in_R8B;
  ulonglong uVar9;
  float fVar10;
  float fVar11;
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
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  if ((in_XMM3_Da == 0.0) && (!NAN(in_XMM3_Da))) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  if ((*(float *)(in_RDX + 0x30) == 0.0) && (!NAN(*(float *)(in_RDX + 0x30)))) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  if (((undefined1 *)0xaaaaaaaaaaaaaaa < puVar2) ||
     (local_158 = in_RCX, local_e2 = in_R8B, std::alloc::__default_lib_allocator::__rust_alloc(),
     extraout_RAX == (longlong *)0x0)) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  local_f8 = (undefined1 *)0x0;
  plVar7 = *(longlong **)(in_RDX + 8);
  local_148 = (undefined **)*plVar7;
  local_e1 = 1;
  local_150 = in_RDX;
  local_108 = puVar2;
  pcStack_100 = (code *)extraout_RAX;
  country_core::resources::terrain_height::TerrainHeightsData::
  sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
  local_e1 = 1;
  plVar6 = &local_190;
  swizzlers::impl$1::xvy();
  *(undefined4 *)(extraout_RAX + 1) = uStack_188;
  *extraout_RAX = local_190;
  local_f8 = (undefined1 *)0x1;
  local_110 = plVar7 + (longlong)puVar2;
  bVar3 = true;
  while (plVar7 != local_110) {
    plVar1 = plVar7 + 1;
    if (bVar3) {
      if (plVar1 == local_110) break;
      local_118 = plVar7 + 2;
      plVar8 = plVar7;
      plVar6 = plVar1;
    }
    else {
      local_118 = plVar1;
      plVar8 = plVar6;
      plVar6 = plVar7;
    }
    fVar10 = (float)*plVar6 - (float)*plVar8;
    fVar11 = (float)((ulonglong)*plVar6 >> 0x20) - (float)((ulonglong)*plVar8 >> 0x20);
    fVar10 = SQRT(fVar11 * fVar11 + fVar10 * fVar10);
    if (fVar10 <= 1e-08) {
      fVar10 = 1e-08;
    }
    fVar10 = in_XMM3_Da / fVar10;
    if (1.0 <= fVar10) {
      fVar10 = 1.0;
    }
    fVar10 = (float)roundf(1.0 / fVar10);
    uVar4 = (longlong)(fVar10 - 9.223372e+18) & (longlong)fVar10 >> 0x3f | (longlong)fVar10;
    if (fVar10 < 0.0) {
      uVar4 = 0;
    }
    if (1.8446743e+19 < fVar10) {
      uVar4 = 0xffffffffffffffff;
    }
    uVar4 = uVar4 + 1;
    if (uVar4 < 3) {
      uVar4 = 2;
    }
    bVar5 = (uVar4 & 1) == 0 & local_e2;
    uVar9 = bVar5 | uVar4;
    if ((longlong)uVar9 < 0) {
      fVar10 = (float)(uVar4 >> 1 | (ulonglong)((uint)bVar5 | (uint)uVar4 & 1));
      fVar10 = fVar10 + fVar10;
    }
    else {
      fVar10 = (float)(longlong)uVar9;
    }
    for (uVar4 = 1; uVar4 < uVar9; uVar4 = uVar4 + 1) {
      fVar11 = (float)uVar4 / (fVar10 + -1.0);
      local_148 = (undefined **)
                  CONCAT44(fVar11 * (float)((ulonglong)*plVar6 >> 0x20) +
                           (1.0 - fVar11) * (float)((ulonglong)*plVar8 >> 0x20),
                           fVar11 * (float)*plVar6 + (1.0 - fVar11) * (float)*plVar8);
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
    bVar3 = false;
    plVar7 = local_118;
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
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}

