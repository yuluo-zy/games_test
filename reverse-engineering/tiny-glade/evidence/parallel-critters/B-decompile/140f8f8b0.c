
void __cdecl system_critters::spawn_ducks::despawn_ducks(void)

{
  uint *puVar1;
  char cVar2;
  byte bVar3;
  uint uVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  undefined8 uVar8;
  ulonglong *puVar9;
  longlong lVar10;
  bool bVar11;
  bool bVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  undefined8 *puVar16;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  longlong *in_RCX;
  longlong lVar17;
  ulonglong uVar18;
  ulonglong uVar19;
  longlong lVar20;
  longlong lVar21;
  longlong *in_RDX;
  ulonglong uVar22;
  longlong lVar23;
  ulonglong uVar24;
  longlong lVar25;
  uint *puVar26;
  longlong lVar27;
  longlong lVar28;
  longlong lVar29;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float fVar30;
  float extraout_XMM0_Da_04;
  float extraout_XMM0_Da_05;
  float extraout_XMM0_Da_06;
  float extraout_XMM0_Da_07;
  float extraout_XMM0_Da_08;
  undefined4 extraout_XMM0_Da_09;
  undefined4 extraout_XMM0_Da_10;
  float fVar31;
  float fVar32;
  float fVar33;
  undefined8 *in_stack_00000028;
  longlong local_180;
  longlong local_178;
  uint uStack_13c;
  longlong local_110;
  longlong local_108;
  longlong local_100;
  
  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  country_core::systems::paths::draw_path::PathRastersResources::as_float_grid();
  *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
  fVar15 = __real_3f7d70a4;
  fVar14 = __real_3f666666;
  fVar13 = __real_3727c5ac;
  fVar32 = __real_3dcccccd;
  lVar5 = *in_RCX;
  lVar6 = *in_RDX;
  lVar7 = in_RDX[1];
  puVar26 = *(uint **)(lVar6 + 0xf8);
  puVar1 = puVar26 + *(longlong *)(lVar6 + 0x100);
  cVar2 = *(char *)(lVar6 + 0x108);
  local_180 = 8;
  local_108 = 0;
  local_178 = 8;
  local_100 = 0;
  local_110 = 0;
  uVar24 = 0;
  uVar18 = 0;
  fVar33 = __real_3f800000;
LAB_140f8fa57:
  do {
    if (cVar2 == '\0') {
      if (uVar18 == uVar24) {
        do {
          if (puVar26 == puVar1) goto LAB_140f900d4;
          uVar4 = *puVar26;
          puVar26 = puVar26 + 1;
          uVar24 = *(ulonglong *)(*(longlong *)(lVar7 + 0x100) + 0x58 + (ulonglong)uVar4 * 0xa0);
        } while (uVar24 == 0);
        lVar28 = *(longlong *)(lVar7 + 0x100) + (ulonglong)uVar4 * 0xa0;
        uVar18 = (ulonglong)*(uint *)(lVar28 + 0x94);
        lVar23 = *(longlong *)(lVar7 + 0x1a8) + uVar18 * 0x48;
        uVar18 = *(ulonglong *)(*(longlong *)(lVar7 + 0x1a8) + 0x40 + uVar18 * 0x48);
        if ((*(ulonglong *)(lVar6 + 0x110) < uVar18) &&
           (uVar19 = *(ulonglong *)
                      (*(longlong *)(lVar23 + 0x38) + *(ulonglong *)(lVar6 + 0x110) * 8),
           uVar19 != 0)) {
          local_108 = *(longlong *)(*(longlong *)(lVar23 + 0x18) + 0x10 + ~uVar19 * 0x30);
          uVar19 = *(ulonglong *)(lVar6 + 0x118);
          if (uVar18 <= uVar19) goto LAB_140f8fccb;
LAB_140f8fc42:
          uVar19 = *(ulonglong *)(*(longlong *)(lVar23 + 0x38) + uVar19 * 8);
          if (uVar19 == 0) goto LAB_140f8fccb;
          local_100 = *(longlong *)(*(longlong *)(lVar23 + 0x18) + 0x10 + ~uVar19 * 0x30);
          uVar19 = *(ulonglong *)(lVar6 + 0x120);
          if (uVar19 < uVar18) goto LAB_140f8fce9;
LAB_140f8fd35:
          local_110 = 0;
        }
        else {
          local_108 = 0;
          uVar19 = *(ulonglong *)(lVar6 + 0x118);
          if (uVar19 < uVar18) goto LAB_140f8fc42;
LAB_140f8fccb:
          local_100 = 0;
          uVar19 = *(ulonglong *)(lVar6 + 0x120);
          if (uVar18 <= uVar19) goto LAB_140f8fd35;
LAB_140f8fce9:
          uVar18 = *(ulonglong *)(*(longlong *)(lVar23 + 0x38) + uVar19 * 8);
          if (uVar18 == 0) goto LAB_140f8fd35;
          local_110 = *(longlong *)(*(longlong *)(lVar23 + 0x18) + 0x10 + ~uVar18 * 0x30);
        }
        local_178 = *(longlong *)(lVar28 + 0x50);
        uVar18 = 0;
      }
      puVar16 = (undefined8 *)(local_178 + uVar18 * 0x10);
      uVar22 = (ulonglong)*(uint *)(local_178 + 8 + uVar18 * 0x10);
      uVar19 = uVar18;
    }
    else {
      if (uVar18 == uVar24) {
        do {
          if (puVar26 == puVar1) goto LAB_140f900d4;
          uVar4 = *puVar26;
          puVar26 = puVar26 + 1;
          uVar24 = *(ulonglong *)(*(longlong *)(lVar7 + 0x1a8) + 0x10 + (ulonglong)uVar4 * 0x48);
        } while (uVar24 == 0);
        lVar23 = *(longlong *)(lVar7 + 0x1a8) + (ulonglong)uVar4 * 0x48;
        uVar18 = *(ulonglong *)(lVar23 + 0x40);
        if ((*(ulonglong *)(lVar6 + 0x110) < uVar18) &&
           (uVar19 = *(ulonglong *)
                      (*(longlong *)(lVar23 + 0x38) + *(ulonglong *)(lVar6 + 0x110) * 8),
           uVar19 != 0)) {
          local_108 = *(longlong *)(*(longlong *)(lVar23 + 0x18) + 0x10 + ~uVar19 * 0x30);
          uVar19 = *(ulonglong *)(lVar6 + 0x118);
          if (uVar18 <= uVar19) goto LAB_140f8fc83;
LAB_140f8fbd8:
          uVar19 = *(ulonglong *)(*(longlong *)(lVar23 + 0x38) + uVar19 * 8);
          if (uVar19 == 0) goto LAB_140f8fc83;
          local_100 = *(longlong *)(*(longlong *)(lVar23 + 0x18) + 0x10 + ~uVar19 * 0x30);
          uVar19 = *(ulonglong *)(lVar6 + 0x120);
          if (uVar19 < uVar18) goto LAB_140f8fca1;
LAB_140f8fd13:
          local_110 = 0;
        }
        else {
          local_108 = 0;
          uVar19 = *(ulonglong *)(lVar6 + 0x118);
          if (uVar19 < uVar18) goto LAB_140f8fbd8;
LAB_140f8fc83:
          local_100 = 0;
          uVar19 = *(ulonglong *)(lVar6 + 0x120);
          if (uVar18 <= uVar19) goto LAB_140f8fd13;
LAB_140f8fca1:
          uVar18 = *(ulonglong *)(*(longlong *)(lVar23 + 0x38) + uVar19 * 8);
          if (uVar18 == 0) goto LAB_140f8fd13;
          local_110 = *(longlong *)(*(longlong *)(lVar23 + 0x18) + 0x10 + ~uVar18 * 0x30);
        }
        local_180 = *(longlong *)(lVar23 + 8);
        uVar18 = 0;
      }
      puVar16 = (undefined8 *)(local_180 + uVar18 * 8);
      uVar22 = uVar18 & 0xffffffff;
      uVar19 = uVar18;
    }
    lVar23 = uVar22 * 0x30 + local_108;
    lVar28 = uVar22 * 0x100 + local_100;
    uVar8 = *puVar16;
    bVar3 = *(byte *)(local_110 + uVar22);
    if (bVar3 != 2) {
      bVar11 = false;
      bVar12 = false;
      if (1 < bVar3) goto LAB_140f8fec1;
LAB_140f8fe9a:
      uVar18 = uVar19 + 1;
      lVar17 = *(longlong *)(lVar28 + 0xa0);
      bVar12 = bVar11;
      if (lVar17 == 0 || lVar17 * 0xc + *(longlong *)(lVar28 + 0x98) == 0xc) goto LAB_140f8fec1;
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
      fVar33 = __real_3f800000;
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
      fVar30 = extraout_XMM0_Da_08 + extraout_XMM0_Da_07 + extraout_XMM0_Da_06 + extraout_XMM0_Da_05
      ;
      fVar31 = 0.0;
      if (0.0 <= fVar30) {
        fVar31 = fVar30;
      }
      fVar30 = fVar33;
      if (fVar31 <= fVar33) {
        fVar30 = fVar31;
      }
      if (bVar11 || (fVar14 < fVar30 || extraout_XMM0_Da_04 < fVar32)) {
LAB_140f8ffef:
        uVar18 = uVar19 + 1;
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX == (undefined8 *)0x0) goto LAB_140f9042d;
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_00 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::alloc::handle_alloc_error();
        }
        *(undefined4 *)(extraout_RAX_00 + 1) = *(undefined4 *)(lVar23 + 0x18);
        *extraout_RAX_00 = *(undefined8 *)(lVar23 + 0x10);
        *extraout_RAX = 1;
        extraout_RAX[1] = extraout_RAX_00;
        extraout_RAX[2] = 1;
        *(undefined1 *)(extraout_RAX + 3) = 0;
        perchance::PerchanceContext::uniform_f32();
        fVar31 = fVar33;
        ::utils::uniform_sample_circle();
        lVar23 = *(longlong *)(lVar5 + 0x20);
        if (lVar23 == *(longlong *)(lVar5 + 0x10)) {
          alloc::raw_vec::RawVec::
          grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>
                    ();
        }
        lVar28 = *(longlong *)(lVar5 + 0x18);
        lVar17 = lVar23 * 0x30;
        puVar16 = (undefined8 *)(lVar28 + 0x20 + lVar17);
        *puVar16 = uVar8;
        puVar16[1] = (ulonglong)uStack_13c << 0x20;
        puVar16 = (undefined8 *)(lVar28 + 0x10 + lVar17);
        *puVar16 = 1;
        puVar16[1] = CONCAT44(fVar31,extraout_XMM0_Da_09);
        *(undefined8 *)(lVar28 + lVar17) = 1;
        ((undefined8 *)(lVar28 + lVar17))[1] = extraout_RAX;
        *(longlong *)(lVar5 + 0x20) = lVar23 + 1;
      }
      goto LAB_140f8fa57;
    }
    ::utils::float_grid::impl$0::
    bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
    bVar11 = true;
    if (fVar13 <= extraout_XMM0_Da) {
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
      fVar30 = extraout_XMM0_Da_03 + extraout_XMM0_Da_02 + extraout_XMM0_Da_01 + extraout_XMM0_Da_00
      ;
      fVar31 = 0.0;
      if (0.0 <= fVar30) {
        fVar31 = fVar30;
      }
      fVar30 = fVar33;
      if (fVar31 <= fVar33) {
        fVar30 = fVar31;
      }
      bVar11 = fVar15 < fVar30;
    }
    bVar12 = bVar11;
    if (*(byte *)(local_110 + uVar22) < 2) goto LAB_140f8fe9a;
LAB_140f8fec1:
    uVar18 = uVar19 + 1;
    if (bVar12) goto LAB_140f8ffef;
  } while( true );
LAB_140f900d4:
  puVar9 = (ulonglong *)*in_stack_00000028;
  lVar23 = in_stack_00000028[1];
  uVar24 = *puVar9;
  uVar19 = 0;
  uVar18 = uVar24 - *(ulonglong *)(lVar23 + 0x18);
  if (uVar24 < *(ulonglong *)(lVar23 + 0x18)) {
    uVar18 = uVar19;
  }
  uVar22 = uVar24 - *(ulonglong *)(lVar23 + 0x38);
  if (uVar24 < *(ulonglong *)(lVar23 + 0x38)) {
    uVar22 = uVar19;
  }
  lVar28 = uVar18 * 0x10 + *(longlong *)(lVar23 + 8);
  uVar24 = *(ulonglong *)(lVar23 + 0x10) - uVar18;
  if (*(ulonglong *)(lVar23 + 0x10) < uVar18) {
    lVar28 = 8;
    uVar24 = uVar19;
  }
  lVar17 = uVar22 * 0x10 + *(longlong *)(lVar23 + 0x28);
  uVar18 = *(ulonglong *)(lVar23 + 0x30) - uVar22;
  if (*(ulonglong *)(lVar23 + 0x30) < uVar22) {
    lVar17 = 8;
    uVar18 = uVar19;
  }
  *puVar9 = (*(longlong *)(lVar23 + 0x40) - uVar18) - uVar24;
  fVar33 = __real_3f800000;
  lVar23 = lVar28;
  lVar29 = lVar17;
LAB_140f901a0:
  do {
    do {
      if ((lVar23 == 0) || (lVar23 == uVar24 * 0x10 + lVar28)) {
        if ((lVar29 == 0) || (lVar29 == uVar18 * 0x10 + lVar17)) {
          return;
        }
        lVar20 = lVar29 + 0x10;
        lVar21 = 0;
        lVar27 = lVar29;
      }
      else {
        lVar21 = lVar23 + 0x10;
        lVar27 = lVar23;
        lVar20 = lVar29;
      }
      *puVar9 = *puVar9 + 1;
      uVar19 = *(ulonglong *)(lVar27 + 8) & 0xffffffff;
      lVar23 = lVar21;
      lVar29 = lVar20;
    } while ((*(ulonglong *)(lVar7 + 0x10) <= uVar19) ||
            (*(int *)(*(longlong *)(lVar7 + 8) + uVar19 * 0x14) !=
             (int)(*(ulonglong *)(lVar27 + 8) >> 0x20)));
    lVar21 = *(longlong *)(lVar7 + 8) + uVar19 * 0x14;
    uVar19 = (ulonglong)*(uint *)(lVar21 + 4);
  } while ((uVar19 == 0xffffffff) ||
          ((*(ulonglong *)(lVar6 + 0x38) <= uVar19 ||
           ((*(ulonglong *)
              (*(longlong *)(lVar6 + 0x28) + (ulonglong)(*(uint *)(lVar21 + 4) >> 6) * 8) >>
             (uVar19 & 0x3f) & 1) == 0))));
  uVar19 = (ulonglong)*(uint *)(lVar21 + 0xc);
  lVar25 = *(longlong *)(lVar7 + 0x1a8);
  lVar20 = lVar25 + uVar19 * 0x48;
  lVar10 = *(longlong *)(lVar25 + 0x38 + uVar19 * 0x48);
  if (*(ulonglong *)(lVar6 + 0x110) < *(ulonglong *)(lVar25 + 0x40 + uVar19 * 0x48)) {
    uVar19 = *(ulonglong *)(lVar10 + *(ulonglong *)(lVar6 + 0x110) * 8);
    lVar20 = *(longlong *)(lVar20 + 0x18);
    if (uVar19 == 0) goto LAB_140f9028b;
    lVar25 = *(longlong *)(lVar20 + 0x10 + ~uVar19 * 0x30);
  }
  else {
    lVar20 = *(longlong *)(lVar20 + 0x18);
LAB_140f9028b:
    lVar25 = 0;
  }
  uVar4 = *(uint *)(lVar21 + 0x10);
  if (*(char *)(*(longlong *)
                 (lVar20 + 0x10 + ~*(ulonglong *)(lVar10 + *(longlong *)(lVar6 + 0x120) * 8) * 0x30)
               + (ulonglong)uVar4) != '\x03') {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_01 == (undefined8 *)0x0) {
LAB_140f9042d:
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error();
    }
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_02 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error();
    }
    lVar21 = (ulonglong)uVar4 * 0x30;
    *(undefined4 *)(extraout_RAX_02 + 1) = *(undefined4 *)(lVar25 + 0x18 + lVar21);
    *extraout_RAX_02 = *(undefined8 *)(lVar25 + 0x10 + lVar21);
    *extraout_RAX_01 = 1;
    extraout_RAX_01[1] = extraout_RAX_02;
    extraout_RAX_01[2] = 1;
    *(undefined1 *)(extraout_RAX_01 + 3) = 0;
    perchance::PerchanceContext::uniform_f32();
    fVar32 = fVar33;
    ::utils::uniform_sample_circle();
    uVar8 = *(undefined8 *)(lVar27 + 8);
    lVar21 = *(longlong *)(lVar5 + 0x20);
    if (lVar21 == *(longlong *)(lVar5 + 0x10)) {
      alloc::raw_vec::RawVec::
      grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>();
    }
    lVar27 = *(longlong *)(lVar5 + 0x18);
    lVar20 = lVar21 * 0x30;
    puVar16 = (undefined8 *)(lVar27 + 0x20 + lVar20);
    *puVar16 = uVar8;
    puVar16[1] = (ulonglong)uStack_13c << 0x20;
    puVar16 = (undefined8 *)(lVar27 + 0x10 + lVar20);
    *puVar16 = 1;
    puVar16[1] = CONCAT44(fVar32,extraout_XMM0_Da_10);
    *(undefined8 *)(lVar27 + lVar20) = 1;
    ((undefined8 *)(lVar27 + lVar20))[1] = extraout_RAX_01;
    *(longlong *)(lVar5 + 0x20) = lVar21 + 1;
  }
  goto LAB_140f901a0;
}

