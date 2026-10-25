
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
  undefined8 *puVar13;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  longlong *in_RCX;
  longlong lVar14;
  ulonglong uVar15;
  ulonglong uVar16;
  longlong lVar17;
  longlong lVar18;
  longlong *in_RDX;
  ulonglong uVar19;
  longlong lVar20;
  ulonglong uVar21;
  longlong lVar22;
  uint *puVar23;
  longlong lVar24;
  longlong lVar25;
  longlong lVar26;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float fVar27;
  float extraout_XMM0_Da_04;
  float extraout_XMM0_Da_05;
  float extraout_XMM0_Da_06;
  float extraout_XMM0_Da_07;
  float extraout_XMM0_Da_08;
  undefined4 extraout_XMM0_Da_09;
  undefined4 extraout_XMM0_Da_10;
  float fVar28;
  undefined4 uVar29;
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
  lVar5 = *in_RCX;
  lVar6 = *in_RDX;
  lVar7 = in_RDX[1];
  puVar23 = *(uint **)(lVar6 + 0xf8);
  puVar1 = puVar23 + *(longlong *)(lVar6 + 0x100);
  cVar2 = *(char *)(lVar6 + 0x108);
  local_180 = 8;
  local_108 = 0;
  local_178 = 8;
  local_100 = 0;
  local_110 = 0;
  uVar21 = 0;
  uVar15 = 0;
LAB_140f8fa57:
  do {
    if (cVar2 == '\0') {
      if (uVar15 == uVar21) {
        do {
          if (puVar23 == puVar1) goto LAB_140f900d4;
          uVar4 = *puVar23;
          puVar23 = puVar23 + 1;
          uVar21 = *(ulonglong *)(*(longlong *)(lVar7 + 0x100) + 0x58 + (ulonglong)uVar4 * 0xa0);
        } while (uVar21 == 0);
        lVar25 = *(longlong *)(lVar7 + 0x100) + (ulonglong)uVar4 * 0xa0;
        uVar15 = (ulonglong)*(uint *)(lVar25 + 0x94);
        lVar20 = *(longlong *)(lVar7 + 0x1a8) + uVar15 * 0x48;
        uVar15 = *(ulonglong *)(*(longlong *)(lVar7 + 0x1a8) + 0x40 + uVar15 * 0x48);
        if ((*(ulonglong *)(lVar6 + 0x110) < uVar15) &&
           (uVar16 = *(ulonglong *)
                      (*(longlong *)(lVar20 + 0x38) + *(ulonglong *)(lVar6 + 0x110) * 8),
           uVar16 != 0)) {
          local_108 = *(longlong *)(*(longlong *)(lVar20 + 0x18) + 0x10 + ~uVar16 * 0x30);
          uVar16 = *(ulonglong *)(lVar6 + 0x118);
          if (uVar15 <= uVar16) goto LAB_140f8fccb;
LAB_140f8fc42:
          uVar16 = *(ulonglong *)(*(longlong *)(lVar20 + 0x38) + uVar16 * 8);
          if (uVar16 == 0) goto LAB_140f8fccb;
          local_100 = *(longlong *)(*(longlong *)(lVar20 + 0x18) + 0x10 + ~uVar16 * 0x30);
          uVar16 = *(ulonglong *)(lVar6 + 0x120);
          if (uVar16 < uVar15) goto LAB_140f8fce9;
LAB_140f8fd35:
          local_110 = 0;
        }
        else {
          local_108 = 0;
          uVar16 = *(ulonglong *)(lVar6 + 0x118);
          if (uVar16 < uVar15) goto LAB_140f8fc42;
LAB_140f8fccb:
          local_100 = 0;
          uVar16 = *(ulonglong *)(lVar6 + 0x120);
          if (uVar15 <= uVar16) goto LAB_140f8fd35;
LAB_140f8fce9:
          uVar15 = *(ulonglong *)(*(longlong *)(lVar20 + 0x38) + uVar16 * 8);
          if (uVar15 == 0) goto LAB_140f8fd35;
          local_110 = *(longlong *)(*(longlong *)(lVar20 + 0x18) + 0x10 + ~uVar15 * 0x30);
        }
        local_178 = *(longlong *)(lVar25 + 0x50);
        uVar15 = 0;
      }
      puVar13 = (undefined8 *)(local_178 + uVar15 * 0x10);
      uVar19 = (ulonglong)*(uint *)(local_178 + 8 + uVar15 * 0x10);
      uVar16 = uVar15;
    }
    else {
      if (uVar15 == uVar21) {
        do {
          if (puVar23 == puVar1) goto LAB_140f900d4;
          uVar4 = *puVar23;
          puVar23 = puVar23 + 1;
          uVar21 = *(ulonglong *)(*(longlong *)(lVar7 + 0x1a8) + 0x10 + (ulonglong)uVar4 * 0x48);
        } while (uVar21 == 0);
        lVar20 = *(longlong *)(lVar7 + 0x1a8) + (ulonglong)uVar4 * 0x48;
        uVar15 = *(ulonglong *)(lVar20 + 0x40);
        if ((*(ulonglong *)(lVar6 + 0x110) < uVar15) &&
           (uVar16 = *(ulonglong *)
                      (*(longlong *)(lVar20 + 0x38) + *(ulonglong *)(lVar6 + 0x110) * 8),
           uVar16 != 0)) {
          local_108 = *(longlong *)(*(longlong *)(lVar20 + 0x18) + 0x10 + ~uVar16 * 0x30);
          uVar16 = *(ulonglong *)(lVar6 + 0x118);
          if (uVar15 <= uVar16) goto LAB_140f8fc83;
LAB_140f8fbd8:
          uVar16 = *(ulonglong *)(*(longlong *)(lVar20 + 0x38) + uVar16 * 8);
          if (uVar16 == 0) goto LAB_140f8fc83;
          local_100 = *(longlong *)(*(longlong *)(lVar20 + 0x18) + 0x10 + ~uVar16 * 0x30);
          uVar16 = *(ulonglong *)(lVar6 + 0x120);
          if (uVar16 < uVar15) goto LAB_140f8fca1;
LAB_140f8fd13:
          local_110 = 0;
        }
        else {
          local_108 = 0;
          uVar16 = *(ulonglong *)(lVar6 + 0x118);
          if (uVar16 < uVar15) goto LAB_140f8fbd8;
LAB_140f8fc83:
          local_100 = 0;
          uVar16 = *(ulonglong *)(lVar6 + 0x120);
          if (uVar15 <= uVar16) goto LAB_140f8fd13;
LAB_140f8fca1:
          uVar15 = *(ulonglong *)(*(longlong *)(lVar20 + 0x38) + uVar16 * 8);
          if (uVar15 == 0) goto LAB_140f8fd13;
          local_110 = *(longlong *)(*(longlong *)(lVar20 + 0x18) + 0x10 + ~uVar15 * 0x30);
        }
        local_180 = *(longlong *)(lVar20 + 8);
        uVar15 = 0;
      }
      puVar13 = (undefined8 *)(local_180 + uVar15 * 8);
      uVar19 = uVar15 & 0xffffffff;
      uVar16 = uVar15;
    }
    lVar20 = uVar19 * 0x30 + local_108;
    lVar25 = uVar19 * 0x100 + local_100;
    uVar8 = *puVar13;
    bVar3 = *(byte *)(local_110 + uVar19);
    if (bVar3 != 2) {
      bVar11 = false;
      bVar12 = false;
      if (1 < bVar3) goto LAB_140f8fec1;
LAB_140f8fe9a:
      uVar15 = uVar16 + 1;
      lVar14 = *(longlong *)(lVar25 + 0xa0);
      bVar12 = bVar11;
      if (lVar14 == 0 || lVar14 * 0xc + *(longlong *)(lVar25 + 0x98) == 0xc) goto LAB_140f8fec1;
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
      fVar27 = extraout_XMM0_Da_08 + extraout_XMM0_Da_07 + extraout_XMM0_Da_06 + extraout_XMM0_Da_05
      ;
      fVar28 = 0.0;
      if (0.0 <= fVar27) {
        fVar28 = fVar27;
      }
      fVar27 = 1.0;
      if (fVar28 <= 1.0) {
        fVar27 = fVar28;
      }
      if (bVar11 || (0.9 < fVar27 || extraout_XMM0_Da_04 < 0.1)) {
LAB_140f8ffef:
        uVar15 = uVar16 + 1;
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX == (undefined8 *)0x0) goto LAB_140f9042d;
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_00 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
          alloc::alloc::handle_alloc_error();
        }
        *(undefined4 *)(extraout_RAX_00 + 1) = *(undefined4 *)(lVar20 + 0x18);
        *extraout_RAX_00 = *(undefined8 *)(lVar20 + 0x10);
        *extraout_RAX = 1;
        extraout_RAX[1] = extraout_RAX_00;
        extraout_RAX[2] = 1;
        *(undefined1 *)(extraout_RAX + 3) = 0;
        perchance::PerchanceContext::uniform_f32();
        uVar29 = 0x3f800000;
        ::utils::uniform_sample_circle();
        lVar20 = *(longlong *)(lVar5 + 0x20);
        if (lVar20 == *(longlong *)(lVar5 + 0x10)) {
          alloc::raw_vec::RawVec::
          grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>
                    ();
        }
        lVar25 = *(longlong *)(lVar5 + 0x18);
        lVar14 = lVar20 * 0x30;
        puVar13 = (undefined8 *)(lVar25 + 0x20 + lVar14);
        *puVar13 = uVar8;
        puVar13[1] = (ulonglong)uStack_13c << 0x20;
        puVar13 = (undefined8 *)(lVar25 + 0x10 + lVar14);
        *puVar13 = 1;
        puVar13[1] = CONCAT44(uVar29,extraout_XMM0_Da_09);
        *(undefined8 *)(lVar25 + lVar14) = 1;
        ((undefined8 *)(lVar25 + lVar14))[1] = extraout_RAX;
        *(longlong *)(lVar5 + 0x20) = lVar20 + 1;
      }
      goto LAB_140f8fa57;
    }
    ::utils::float_grid::impl$0::
    bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
    bVar11 = true;
    if (1e-05 <= extraout_XMM0_Da) {
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
      fVar27 = extraout_XMM0_Da_03 + extraout_XMM0_Da_02 + extraout_XMM0_Da_01 + extraout_XMM0_Da_00
      ;
      fVar28 = 0.0;
      if (0.0 <= fVar27) {
        fVar28 = fVar27;
      }
      fVar27 = 1.0;
      if (fVar28 <= 1.0) {
        fVar27 = fVar28;
      }
      bVar11 = 0.99 < fVar27;
    }
    bVar12 = bVar11;
    if (*(byte *)(local_110 + uVar19) < 2) goto LAB_140f8fe9a;
LAB_140f8fec1:
    uVar15 = uVar16 + 1;
    if (bVar12) goto LAB_140f8ffef;
  } while( true );
LAB_140f900d4:
  puVar9 = (ulonglong *)*in_stack_00000028;
  lVar20 = in_stack_00000028[1];
  uVar21 = *puVar9;
  uVar16 = 0;
  uVar15 = uVar21 - *(ulonglong *)(lVar20 + 0x18);
  if (uVar21 < *(ulonglong *)(lVar20 + 0x18)) {
    uVar15 = uVar16;
  }
  uVar19 = uVar21 - *(ulonglong *)(lVar20 + 0x38);
  if (uVar21 < *(ulonglong *)(lVar20 + 0x38)) {
    uVar19 = uVar16;
  }
  lVar25 = uVar15 * 0x10 + *(longlong *)(lVar20 + 8);
  uVar21 = *(ulonglong *)(lVar20 + 0x10) - uVar15;
  if (*(ulonglong *)(lVar20 + 0x10) < uVar15) {
    lVar25 = 8;
    uVar21 = uVar16;
  }
  lVar14 = uVar19 * 0x10 + *(longlong *)(lVar20 + 0x28);
  uVar15 = *(ulonglong *)(lVar20 + 0x30) - uVar19;
  if (*(ulonglong *)(lVar20 + 0x30) < uVar19) {
    lVar14 = 8;
    uVar15 = uVar16;
  }
  *puVar9 = (*(longlong *)(lVar20 + 0x40) - uVar15) - uVar21;
  lVar20 = lVar25;
  lVar26 = lVar14;
LAB_140f901a0:
  do {
    do {
      if ((lVar20 == 0) || (lVar20 == uVar21 * 0x10 + lVar25)) {
        if ((lVar26 == 0) || (lVar26 == uVar15 * 0x10 + lVar14)) {
          return;
        }
        lVar17 = lVar26 + 0x10;
        lVar18 = 0;
        lVar24 = lVar26;
      }
      else {
        lVar18 = lVar20 + 0x10;
        lVar24 = lVar20;
        lVar17 = lVar26;
      }
      *puVar9 = *puVar9 + 1;
      uVar16 = *(ulonglong *)(lVar24 + 8) & 0xffffffff;
      lVar20 = lVar18;
      lVar26 = lVar17;
    } while ((*(ulonglong *)(lVar7 + 0x10) <= uVar16) ||
            (*(int *)(*(longlong *)(lVar7 + 8) + uVar16 * 0x14) !=
             (int)(*(ulonglong *)(lVar24 + 8) >> 0x20)));
    lVar18 = *(longlong *)(lVar7 + 8) + uVar16 * 0x14;
    uVar16 = (ulonglong)*(uint *)(lVar18 + 4);
  } while ((uVar16 == 0xffffffff) ||
          ((*(ulonglong *)(lVar6 + 0x38) <= uVar16 ||
           ((*(ulonglong *)
              (*(longlong *)(lVar6 + 0x28) + (ulonglong)(*(uint *)(lVar18 + 4) >> 6) * 8) >>
             (uVar16 & 0x3f) & 1) == 0))));
  uVar16 = (ulonglong)*(uint *)(lVar18 + 0xc);
  lVar22 = *(longlong *)(lVar7 + 0x1a8);
  lVar17 = lVar22 + uVar16 * 0x48;
  lVar10 = *(longlong *)(lVar22 + 0x38 + uVar16 * 0x48);
  if (*(ulonglong *)(lVar6 + 0x110) < *(ulonglong *)(lVar22 + 0x40 + uVar16 * 0x48)) {
    uVar16 = *(ulonglong *)(lVar10 + *(ulonglong *)(lVar6 + 0x110) * 8);
    lVar17 = *(longlong *)(lVar17 + 0x18);
    if (uVar16 == 0) goto LAB_140f9028b;
    lVar22 = *(longlong *)(lVar17 + 0x10 + ~uVar16 * 0x30);
  }
  else {
    lVar17 = *(longlong *)(lVar17 + 0x18);
LAB_140f9028b:
    lVar22 = 0;
  }
  uVar4 = *(uint *)(lVar18 + 0x10);
  if (*(char *)(*(longlong *)
                 (lVar17 + 0x10 + ~*(ulonglong *)(lVar10 + *(longlong *)(lVar6 + 0x120) * 8) * 0x30)
               + (ulonglong)uVar4) != '\x03') {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_01 == (undefined8 *)0x0) {
LAB_140f9042d:
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_02 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    lVar18 = (ulonglong)uVar4 * 0x30;
    *(undefined4 *)(extraout_RAX_02 + 1) = *(undefined4 *)(lVar22 + 0x18 + lVar18);
    *extraout_RAX_02 = *(undefined8 *)(lVar22 + 0x10 + lVar18);
    *extraout_RAX_01 = 1;
    extraout_RAX_01[1] = extraout_RAX_02;
    extraout_RAX_01[2] = 1;
    *(undefined1 *)(extraout_RAX_01 + 3) = 0;
    perchance::PerchanceContext::uniform_f32();
    uVar29 = 0x3f800000;
    ::utils::uniform_sample_circle();
    uVar8 = *(undefined8 *)(lVar24 + 8);
    lVar18 = *(longlong *)(lVar5 + 0x20);
    if (lVar18 == *(longlong *)(lVar5 + 0x10)) {
      alloc::raw_vec::RawVec::
      grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>();
    }
    lVar24 = *(longlong *)(lVar5 + 0x18);
    lVar17 = lVar18 * 0x30;
    puVar13 = (undefined8 *)(lVar24 + 0x20 + lVar17);
    *puVar13 = uVar8;
    puVar13[1] = (ulonglong)uStack_13c << 0x20;
    puVar13 = (undefined8 *)(lVar24 + 0x10 + lVar17);
    *puVar13 = 1;
    puVar13[1] = CONCAT44(uVar29,extraout_XMM0_Da_10);
    *(undefined8 *)(lVar24 + lVar17) = 1;
    ((undefined8 *)(lVar24 + lVar17))[1] = extraout_RAX_01;
    *(longlong *)(lVar5 + 0x20) = lVar18 + 1;
  }
  goto LAB_140f901a0;
}

