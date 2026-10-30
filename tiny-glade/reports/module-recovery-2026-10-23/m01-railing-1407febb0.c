
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
system_decorator::decorator_visual::stairs_railings_assemble::add_corner_stone_railing_piece
          (ulonglong p1,uint p2,uint p3,void *p4,float *p5,ulonglong p6,float *p7,ulonglong p8,
          float *p9,ulonglong p10,uchar p11,uchar p12,float p13,float p14)

{
  undefined4 uVar1;
  uint uVar2;
  ulonglong uVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float local_1d4;
  undefined4 local_1d0;
  undefined4 local_1cc;
  undefined8 local_1c8;
  float local_1c0;
  float local_1b8;
  float fStack_1b4;
  float local_1b0;
  undefined8 local_1a8;
  float local_1a0;
  undefined8 local_198;
  float local_190;
  undefined8 local_188;
  float local_180;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined4 local_140;
  float local_128;
  float local_124;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined4 local_e0;
  undefined4 local_dc;
  uint local_d8;
  undefined8 local_d4;
  undefined4 local_cc;
  
  if (p8 == 0) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_bounds_check();
  }
  fVar5 = (float)((ulonglong)*(undefined8 *)p7 >> 0x20) * p14 + (float)*(undefined8 *)p7 * p13;
  uVar3 = 0;
  if (0.0 < fVar5) {
    uVar3 = p6 - 1;
  }
  if (p6 <= uVar3) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_bounds_check();
  }
  if (p8 <= uVar3) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_bounds_check();
  }
  fVar6 = p5[uVar3];
  fVar7 = p7[uVar3 * 2];
  fVar9 = p7[uVar3 * 2 + 1];
  fVar8 = __real_3f800000 / SQRT(fVar7 * fVar7 + fVar9 * fVar9);
  local_188._0_4_ = (float)(___xmm_80000000800000008000000080000000 ^ (uint)fVar9) * fVar8;
  local_188._4_4_ = fVar7 * fVar8;
  ::utils::swizzlers::impl$0::x0y(&local_1b8,(float *)&local_188);
  if (uVar3 < p10) {
    fVar4 = fVar6 + ___real_be89374c;
    if (fVar6 + ___real_be89374c <= __real_3dcccccd) {
      fVar4 = __real_3dcccccd;
    }
    fVar6 = p9[uVar3 * 3 + 2];
    fVar5 = *(float *)(&__real_bf8000003f800000 + (ulonglong)(0.0 < fVar5) * 4);
    local_124 = fVar9 * fVar8;
    local_128 = fVar7 * fVar8;
    ::utils::swizzlers::impl$0::x0y(&local_188,&local_128);
    fVar6 = fVar5 * local_180 * __real_3e8a3d71 * __real_3f000000 + fVar6;
    fVar9 = (float)local_188 * fVar5 * ___xmm_00000000000000003e8a3d713e8a3d71 *
            (float)__xmm_00000000000000003f0000003f000000 + (float)*(undefined8 *)(p9 + uVar3 * 3);
    fVar8 = local_188._4_4_ * fVar5 * _UNK_142a8c724 * __xmm_00000000000000003f0000003f000000._4_4_
            + (float)((ulonglong)*(undefined8 *)(p9 + uVar3 * 3) >> 0x20);
    uVar1 = *(undefined4 *)(&DAT_142a8cc30 + ((ulonglong)p3 & 0xff) * 4);
    fVar10 = local_1b8 - fStack_1b4 * 0.0;
    fVar5 = fStack_1b4 * 0.0 - local_1b0;
    fVar7 = local_1b0 * 0.0 - local_1b8 * 0.0;
    if (p11 == '\0') {
      local_1a0 = fVar4 * local_1b0 * __real_bf000000 + fVar6;
      local_1a8 = CONCAT44(fVar4 * fStack_1b4 * _UNK_142a171f4 + fVar8,
                           fVar4 * local_1b8 * ___xmm_0000000000000000bf000000bf000000 + fVar9);
      local_1c0 = local_1b0 * __real_3e8a3d71;
      local_1c8 = CONCAT44(fStack_1b4 * _UNK_142a8c724,
                           local_1b8 * ___xmm_00000000000000003e8a3d713e8a3d71);
      local_1d4 = 0.0;
      local_1cc = 0;
      local_180 = __real_3e8a3d71 * fVar10;
      local_188 = CONCAT44(_UNK_142a8c724 * fVar7,___xmm_00000000000000003e8a3d713e8a3d71 * fVar5);
      local_1d0 = uVar1;
      country_core::resources::render::Affine3Packed::from_cols
                (&local_128,(float *)&local_1c8,&local_1d4,(float *)&local_188,(float *)&local_1a8);
      uVar2 = country_core::geometry::instanced_wall::BrickSourceId::from_wall_id(p2);
      country_core::geometry::instanced_wall::impl$3::default_brick(&local_188);
      local_f8 = local_158;
      uStack_f0 = uStack_150;
      local_e8 = local_148;
      local_e0 = local_140;
      local_d4 = __xmm_00000000000000003f80000000000000;
      local_cc = 0x40;
      local_dc = (int)uVar3;
      local_d8 = uVar2;
      country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push_brick
                (p4,&local_128);
    }
    if (p12 == '\0') {
      local_190 = fVar6 + fVar4 * local_1b0 * __real_3f000000;
      local_198 = CONCAT44(fVar8 + fVar4 * fStack_1b4 * __xmm_00000000000000003f0000003f000000._4_4_
                           ,fVar9 + fVar4 * local_1b8 *
                                    (float)__xmm_00000000000000003f0000003f000000);
      local_1c0 = local_1b0 * __real_3e8a3d71;
      local_1c8 = CONCAT44(fStack_1b4 * _UNK_142a8c724,
                           local_1b8 * ___xmm_00000000000000003e8a3d713e8a3d71);
      local_1d4 = 0.0;
      local_1cc = 0;
      local_180 = fVar10 * __real_3e8a3d71;
      local_188 = CONCAT44(fVar7 * _UNK_142a8c724,fVar5 * ___xmm_00000000000000003e8a3d713e8a3d71);
      local_1d0 = uVar1;
      country_core::resources::render::Affine3Packed::from_cols
                (&local_128,(float *)&local_1c8,&local_1d4,(float *)&local_188,(float *)&local_198);
      uVar2 = country_core::geometry::instanced_wall::BrickSourceId::from_wall_id(p2);
      country_core::geometry::instanced_wall::impl$3::default_brick(&local_188);
      local_f8 = local_158;
      uStack_f0 = uStack_150;
      local_e8 = local_148;
      local_e0 = local_140;
      local_d4 = __xmm_00000000000000003f80000000000000;
      local_cc = 0x40;
      local_dc = (int)uVar3;
      local_d8 = uVar2;
      country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push_brick
                (p4,&local_128);
    }
    return;
  }
                    /* WARNING: Subroutine does not return */
  core::panicking::panic_bounds_check();
}

