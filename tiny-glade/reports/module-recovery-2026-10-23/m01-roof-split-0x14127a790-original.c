
// WARNING: Type propagation algorithm not settling

void __cdecl
system_decorator::ui_systems::handle_splitting_roof_decorators::
try_display_roof_decorator_parts_and_get_hovered_part_id(void)

{
  longlong lVar1;
  float fVar2;
  ulonglong ***pppuVar3;
  byte bVar4;
  uint uVar5;
  longlong lVar6;
  longlong lVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  ulonglong extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  undefined8 *extraout_RAX_03;
  int iVar11;
  longlong in_RCX;
  longlong in_RDX;
  ulonglong uVar12;
  ulonglong uVar13;
  ulonglong **ppuVar14;
  float *in_R8;
  float fVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  float fVar19;
  float fVar20;
  float fVar21;
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  float fVar30;
  float fVar31;
  longlong *in_stack_00000048;
  longlong *in_stack_00000050;
  longlong local_200;
  ulonglong **local_1f8;
  longlong local_1e8;
  longlong local_1d0;
  ulonglong **local_190;
  ulonglong ***local_188;
  float local_180;
  ulonglong **local_178;
  uint local_170;
  float fStack_16c;
  float fStack_168;
  float fStack_164;
  ulonglong ****local_160;
  undefined8 uStack_158;
  undefined8 local_150;
  float local_13c;
  undefined8 ****local_138;
  code *pcStack_130;
  ulonglong ****local_128;
  code *pcStack_120;
  undefined8 local_118;
  longlong local_110;
  undefined1 local_104 [4];
  longlong local_100;
  byte local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall();
  country_core::resources::walls::public_walls::PublicWalls::roof_entity();
  if (extraout_RAX >> 0x20 == 0) {
    return;
  }
  lVar6 = in_stack_00000048[1];
  uVar12 = extraout_RAX & 0xffffffff;
  if (*(ulonglong *)(lVar6 + 0x10) <= uVar12) {
    return;
  }
  iVar11 = (int)(extraout_RAX >> 0x20);
  if (*(int *)(*(longlong *)(lVar6 + 8) + uVar12 * 0x14) != iVar11) {
    return;
  }
  lVar1 = *(longlong *)(lVar6 + 8) + uVar12 * 0x14;
  uVar13 = (ulonglong)*(uint *)(lVar1 + 4);
  if (uVar13 == 0xffffffff) {
    return;
  }
  lVar7 = *in_stack_00000048;
  if (*(ulonglong *)(lVar7 + 0x38) <= uVar13) {
    return;
  }
  if ((*(ulonglong *)(*(longlong *)(lVar7 + 0x28) + (ulonglong)(*(uint *)(lVar1 + 4) >> 6) * 8) >>
       (uVar13 & 0x3f) & 1) == 0) {
    return;
  }
  if (*(ulonglong *)(lVar7 + 0x110) <
      *(ulonglong *)(*(longlong *)(lVar6 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar1 + 0xc) * 0x48)
     ) {
    lVar6 = *(longlong *)(lVar6 + 0x1a8) + (ulonglong)*(uint *)(lVar1 + 0xc) * 0x48;
    uVar13 = *(ulonglong *)(*(longlong *)(lVar6 + 0x38) + *(ulonglong *)(lVar7 + 0x110) * 8);
    if (uVar13 != 0) {
      local_110 = *(longlong *)(*(longlong *)(lVar6 + 0x18) + 0x10 + ~uVar13 * 0x30);
      goto LAB_14127a8fd;
    }
  }
  local_110 = 0;
LAB_14127a8fd:
  if ((uVar12 < *(ulonglong *)(in_stack_00000050[1] + 0x10)) &&
     (lVar6 = *(longlong *)(in_stack_00000050[1] + 8), *(int *)(lVar6 + uVar12 * 0x14) == iVar11)) {
    uVar5 = *(uint *)(lVar6 + uVar12 * 0x14 + 4);
    uVar12 = (ulonglong)uVar5;
    if ((uVar12 != 0xffffffff) &&
       ((uVar12 < *(ulonglong *)(*in_stack_00000050 + 0x38) &&
        ((*(ulonglong *)(*(longlong *)(*in_stack_00000050 + 0x28) + (ulonglong)(uVar5 >> 6) * 8) >>
          (uVar12 & 0x3f) & 1) != 0)))) {
      local_110 = local_110 + (ulonglong)*(uint *)(lVar1 + 0x10) * 0x58;
      fVar8 = *in_R8;
      fVar9 = in_R8[1];
      fVar10 = in_R8[2];
      fVar23 = in_R8[3];
      fVar28 = fVar23 * fVar23;
      fVar19 = fVar10 * fVar10 + fVar9 * fVar9 + fVar8 * fVar8;
      fVar20 = fVar10 * 1.0 + fVar9 * 0.0 + fVar8 * 0.0;
      fVar20 = fVar20 + fVar20;
      fVar23 = fVar23 + fVar23;
      fVar15 = (fVar9 * 1.0 - fVar10 * 0.0) * fVar23 + fVar20 * fVar8 + (fVar28 - fVar19) * 0.0;
      fVar17 = (fVar10 * 0.0 - fVar8 * 1.0) * fVar23 + fVar20 * fVar9 + (fVar28 - fVar19) * 0.0;
      fVar18 = (fVar8 * 0.0 - fVar9 * 0.0) * fVar23 + fVar20 * fVar10 + (fVar28 - fVar19) * 1.0;
      fVar20 = *(float *)(in_RCX + 0x18);
      pppuVar3 = *(ulonglong ****)(in_R8 + 4);
      fVar2 = in_R8[6];
      fVar25 = (float)*(undefined8 *)(in_RCX + 0x1c);
      fVar26 = (float)((ulonglong)*(undefined8 *)(in_RCX + 0x1c) >> 0x20);
      fVar24 = fVar18 * (fVar26 - (float)((ulonglong)*(undefined8 *)(in_R8 + 5) >> 0x20)) +
               (fVar20 - SUB84(pppuVar3,0)) * fVar15 +
               fVar17 * (fVar25 - (float)*(undefined8 *)(in_R8 + 5));
      bVar4 = *(byte *)(in_RDX + 0x44);
      uVar12 = (longlong)(ulonglong)bVar4 - 1;
      if ((longlong)uVar12 < 0) {
        fVar31 = (float)uVar12;
      }
      else {
        fVar31 = (float)(int)uVar12;
      }
      local_f1 = *(byte *)(local_110 + 8);
      local_13c = 100.0;
      ppuVar14 = (ulonglong **)0x0;
      local_188 = pppuVar3;
      local_180 = fVar2;
      while (ppuVar14 < (ulonglong **)(ulonglong)bVar4) {
        local_190 = ppuVar14;
        if (2 < (byte)(bVar4 - 1)) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
        fVar29 = ((float)(longlong)ppuVar14 / fVar31 + -0.5) *
                 *(float *)(&DAT_142c23fa0 + (ulonglong)(byte)(bVar4 - 1) * 4) *
                 (1.0 - 1.0 / (float)bVar4);
        local_138 = (undefined8 ****)pppuVar3;
        if ((local_f1 & 1) == 0) {
          pcStack_130 = (code *)CONCAT44(pcStack_130._4_4_,fVar2);
          country_core::resources::roofs::roof_shape::Roof::get_roof_shape_from_pos_ws_clamped();
          country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter();
        }
        else {
          pcStack_130 = (code *)CONCAT44(pcStack_130._4_4_,fVar2);
          country_core::resources::roofs::roof_shape::Roof::get_roof_shape_from_pos_ws_clamped();
          country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle();
          local_138 = (undefined8 ****)*extraout_RAX_00;
          pcStack_130 = (code *)extraout_RAX_00[1];
          local_128 = (ulonglong ****)extraout_RAX_00[2];
          ::utils::geometry::rectangle::Rectangle2d::get_closest_edge_id();
          ::utils::geometry::rectangle::Rectangle2d::edge_length();
        }
        country_core::resources::walls::decorator::DecoratorInfo::dst_as_roof();
        country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume::
        get_position_from_mesh_uv();
        fVar27 = fStack_164;
        fVar30 = fStack_16c;
        fVar16 = fStack_168;
        if ((local_170 & 1) == 0) {
          fVar16 = fVar29 * 0.0;
          fVar21 = fVar10 * fVar16 + fVar9 * fVar16 + fVar8 * fVar29;
          fVar21 = fVar21 + fVar21;
          fVar27 = (fVar8 * fVar16 - fVar29 * fVar9) * fVar23 +
                   fVar21 * fVar10 + fVar16 * (fVar28 - fVar19) + fVar2;
          fVar30 = SUB84(pppuVar3,0) +
                   (fVar9 * fVar16 - fVar16 * fVar10) * fVar23 +
                   fVar21 * fVar8 + fVar29 * (fVar28 - fVar19);
          fVar16 = (float)((ulonglong)pppuVar3 >> 0x20) +
                   (fVar10 * fVar29 - fVar16 * fVar8) * fVar23 +
                   fVar21 * fVar9 + fVar16 * (fVar28 - fVar19);
        }
        ppuVar14 = (ulonglong **)((longlong)ppuVar14 + 1);
        if ((bVar4 == 3) && (local_190 == (ulonglong **)0x1)) {
          local_138 = (undefined8 ****)&local_190;
          pcStack_130 = core::fmt::num::imp::impl$9::fmt;
          local_128 = (ulonglong ****)local_104;
          pcStack_120 = country_core::resources::walls::decorator::impl$2::fmt;
          local_170 = 0x42c23d40;
          fStack_16c = 1.4013e-45;
          fStack_168 = 2.8026e-45;
          fStack_164 = 0.0;
          uStack_158 = 2;
          local_150 = 0;
          local_178 = ppuVar14;
          local_160 = (ulonglong ****)&local_138;
          alloc::fmt::format::format_inner();
          local_100 = local_1e8;
          std::alloc::__default_lib_allocator::__rust_alloc();
          if (extraout_RAX_01 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
            alloc::alloc::handle_alloc_error();
          }
          *extraout_RAX_01 = 1;
          extraout_RAX_01[1] = 1;
          extraout_RAX_01[2] = CONCAT44(fVar16 + 0.5,fVar30 + 0.0);
          *(float *)(extraout_RAX_01 + 3) = fVar27 + 0.0;
          country_core::systems::ui::signifiers::SignifierFactory::new();
          local_118 = local_150;
          local_138 = (undefined8 ****)CONCAT44(fStack_16c,local_170);
          pcStack_130 = (code *)CONCAT44(fStack_164,fStack_168);
          local_128 = local_160;
          pcStack_120 = (code *)uStack_158;
          country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write();
          ppuVar14 = local_178;
          if (local_100 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
        }
        else {
          fVar29 = fVar16 - (fVar25 - fVar17 * fVar24);
          fVar21 = fVar30 - (fVar20 - fVar24 * fVar15);
          fVar22 = fVar27 - (fVar26 - fVar24 * fVar18);
          fVar29 = SQRT(fVar22 * fVar22 + fVar29 * fVar29 + fVar21 * fVar21);
          if (fVar29 < local_13c) {
            local_188 = (ulonglong ***)CONCAT44(fVar16 + 0.5,fVar30 + 0.0);
            local_180 = fVar27 + 0.0;
            local_13c = fVar29;
          }
          local_138 = (undefined8 ****)&local_190;
          pcStack_130 = core::fmt::num::imp::impl$9::fmt;
          local_128 = (ulonglong ****)local_104;
          pcStack_120 = country_core::resources::walls::decorator::impl$2::fmt;
          local_170 = 0x42c23d40;
          fStack_16c = 1.4013e-45;
          fStack_168 = 2.8026e-45;
          fStack_164 = 0.0;
          uStack_158 = 2;
          local_150 = 0;
          local_160 = (ulonglong ****)&local_138;
          alloc::fmt::format::format_inner();
          local_100 = local_200;
          std::alloc::__default_lib_allocator::__rust_alloc();
          local_178 = local_1f8;
          if (extraout_RAX_02 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
            alloc::alloc::handle_alloc_error();
          }
          *extraout_RAX_02 = 1;
          extraout_RAX_02[1] = 1;
          extraout_RAX_02[2] = CONCAT44(fVar16 + 0.5,fVar30 + 0.0);
          *(float *)(extraout_RAX_02 + 3) = fVar27 + 0.0;
          country_core::systems::ui::signifiers::SignifierFactory::new();
          local_118 = local_150;
          local_138 = (undefined8 ****)CONCAT44(fStack_16c,local_170);
          pcStack_130 = (code *)CONCAT44(fStack_164,fStack_168);
          local_128 = local_160;
          pcStack_120 = (code *)uStack_158;
          country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write();
          if (local_100 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
        }
      }
      local_138 = &local_188;
      pcStack_130 = glam::f32::vec3::impl$92::fmt;
      local_128 = (ulonglong ****)local_104;
      pcStack_120 = country_core::resources::walls::decorator::impl$2::fmt;
      local_170 = 0x42c23d40;
      fStack_16c = 1.4013e-45;
      fStack_168 = 2.8026e-45;
      fStack_164 = 0.0;
      uStack_158 = 2;
      local_150 = 0;
      local_160 = (ulonglong ****)&local_138;
      alloc::fmt::format::format_inner();
      fStack_168 = local_180;
      local_170 = (uint)local_188;
      fStack_16c = (float)((ulonglong)local_188 >> 0x20);
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_03 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      *extraout_RAX_03 = 1;
      extraout_RAX_03[1] = 1;
      extraout_RAX_03[2] = CONCAT44(fStack_16c,local_170);
      *(float *)(extraout_RAX_03 + 3) = fStack_168;
      country_core::systems::ui::signifiers::SignifierFactory::new();
      local_118 = local_150;
      local_138 = (undefined8 ****)CONCAT44(fStack_16c,local_170);
      pcStack_130 = (code *)CONCAT44(fStack_164,fStack_168);
      local_128 = local_160;
      pcStack_120 = (code *)uStack_158;
      country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write();
      if (local_1d0 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
  }
  return;
}


