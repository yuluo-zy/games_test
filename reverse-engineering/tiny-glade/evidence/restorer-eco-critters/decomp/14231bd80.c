// 0x14231bd80 sheep_walk_on_terrain
// SIG: void __cdecl sheep_walk_on_terrain(void)

void __cdecl system_critters::sheep::sheep_terrain::sheep_walk_on_terrain(void)

{
  uint *puVar1;
  undefined4 *puVar2;
  uint *puVar3;
  undefined4 *puVar4;
  undefined4 *puVar5;
  char cVar6;
  undefined4 uVar7;
  uint uVar8;
  longlong lVar9;
  longlong lVar10;
  longlong lVar11;
  ulonglong uVar12;
  longlong *in_RCX;
  longlong lVar13;
  longlong lVar14;
  ulonglong uVar15;
  longlong lVar16;
  ulonglong uVar17;
  longlong *in_R8;
  uint *puVar18;
  float fVar19;
  float extraout_XMM0_Da;
  undefined4 extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float fVar20;
  float fVar21;
  float fVar22;
  longlong local_148;
  longlong local_140;
  longlong local_138;
  longlong local_130;
  longlong local_128;
  longlong local_120;
  longlong local_110;
  
  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  fVar19 = expf(*(float *)(*in_R8 + 0x28) * -3.0);
  fVar19 = 1.0 - fVar19;
  lVar9 = *in_RCX;
  lVar10 = in_RCX[1];
  uVar7 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar18 = *(uint **)(lVar9 + 0xf8);
  puVar1 = puVar18 + *(longlong *)(lVar9 + 0x100);
  cVar6 = *(char *)(lVar9 + 0x108);
  local_110 = 8;
  local_138 = 0;
  local_140 = 0;
  local_148 = 0;
  uVar15 = 0;
  uVar17 = 0;
  do {
    if (cVar6 == '\0') {
      if (uVar17 == uVar15) {
        do {
          if (puVar18 == puVar1) {
            return;
          }
          uVar8 = *puVar18;
          puVar18 = puVar18 + 1;
          uVar15 = *(ulonglong *)(*(longlong *)(lVar10 + 0x100) + 0x58 + (ulonglong)uVar8 * 0xa0);
        } while (uVar15 == 0);
        lVar13 = *(longlong *)(lVar10 + 0x100) + (ulonglong)uVar8 * 0xa0;
        uVar17 = (ulonglong)*(uint *)(lVar13 + 0x94);
        lVar11 = *(longlong *)(*(longlong *)(lVar10 + 0x1a8) + 0x18 + uVar17 * 0x48);
        lVar16 = *(longlong *)(*(longlong *)(lVar10 + 0x1a8) + 0x38 + uVar17 * 0x48);
        lVar14 = ~*(ulonglong *)(lVar16 + *(longlong *)(lVar9 + 0x110) * 8) * 0x30;
        local_140 = *(longlong *)(lVar11 + 0x10 + lVar14);
        local_120 = *(longlong *)(lVar11 + 0x28 + lVar14);
        lVar14 = ~*(ulonglong *)(lVar16 + *(longlong *)(lVar9 + 0x118) * 8) * 0x30;
        local_138 = *(longlong *)(lVar11 + 0x10 + lVar14);
        local_128 = *(longlong *)(lVar11 + 0x28 + lVar14);
        lVar16 = ~*(ulonglong *)(lVar16 + *(longlong *)(lVar9 + 0x120) * 8) * 0x30;
        local_148 = *(longlong *)(lVar11 + 0x10 + lVar16);
        local_130 = *(longlong *)(lVar11 + 0x28 + lVar16);
        local_110 = *(longlong *)(lVar13 + 0x50);
        uVar17 = 0;
      }
      uVar12 = (ulonglong)*(uint *)(local_110 + 8 + uVar17 * 0x10);
    }
    else {
      if (uVar17 == uVar15) {
        do {
          if (puVar18 == puVar1) {
            return;
          }
          uVar8 = *puVar18;
          puVar18 = puVar18 + 1;
          uVar15 = *(ulonglong *)(*(longlong *)(lVar10 + 0x1a8) + 0x10 + (ulonglong)uVar8 * 0x48);
        } while (uVar15 == 0);
        lVar11 = *(longlong *)(lVar10 + 0x1a8) + (ulonglong)uVar8 * 0x48;
        lVar16 = *(longlong *)(lVar11 + 0x18);
        lVar11 = *(longlong *)(lVar11 + 0x38);
        lVar13 = ~*(ulonglong *)(lVar11 + *(longlong *)(lVar9 + 0x110) * 8) * 0x30;
        local_140 = *(longlong *)(lVar16 + 0x10 + lVar13);
        local_120 = *(longlong *)(lVar16 + 0x28 + lVar13);
        lVar13 = ~*(ulonglong *)(lVar11 + *(longlong *)(lVar9 + 0x118) * 8) * 0x30;
        local_138 = *(longlong *)(lVar16 + 0x10 + lVar13);
        local_128 = *(longlong *)(lVar16 + 0x28 + lVar13);
        lVar11 = ~*(ulonglong *)(lVar11 + *(longlong *)(lVar9 + 0x120) * 8) * 0x30;
        local_148 = *(longlong *)(lVar16 + 0x10 + lVar11);
        local_130 = *(longlong *)(lVar16 + 0x28 + lVar11);
        uVar17 = 0;
      }
      uVar12 = uVar17 & 0xffffffff;
    }
    lVar11 = local_148 + uVar12 * 0xc;
    lVar16 = uVar12 * 0x30 + local_140;
    puVar2 = (undefined4 *)(local_120 + uVar12 * 4);
    puVar3 = (uint *)(local_138 + uVar12 * 8);
    puVar4 = (undefined4 *)(local_128 + uVar12 * 4);
    puVar5 = (undefined4 *)(local_130 + uVar12 * 4);
    uVar17 = uVar17 + 1;
    country_core::resources::terrain_height::TerrainHeightsData::
    sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
    *puVar4 = uVar7;
    switch(*puVar3) {
    case 2:
      *puVar2 = uVar7;
      *(float *)(lVar16 + 0x14) = extraout_XMM0_Da;
      break;
    case 3:
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    case 4:
      ::utils::time_continuous_lerp();
      *puVar2 = uVar7;
      *(undefined4 *)(lVar16 + 0x14) = extraout_XMM0_Da_00;
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
      if (extraout_XMM0_Da_01 <= 0.0) {
        *puVar4 = uVar7;
        *puVar3 = 2;
      }
      break;
    default:
      fVar20 = extraout_XMM0_Da;
      if ((*puVar3 & 1) != 0) {
        fVar20 = (float)puVar3[1];
      }
      fVar20 = (fVar20 + 4.0) - *(float *)(lVar16 + 0x14);
      if (extraout_XMM0_Da <= *(float *)(lVar16 + 0x14)) {
        fVar21 = -2.0;
        if (-2.0 <= fVar20) {
          fVar21 = fVar20;
        }
        fVar20 = 2.0;
        if (fVar21 <= 2.0) {
          fVar20 = fVar21;
        }
        fVar20 = (fVar20 * 10.0 - *(float *)(lVar11 + 4)) * fVar19;
      }
      else {
        *puVar2 = uVar7;
        *(float *)(lVar16 + 0x14) = extraout_XMM0_Da;
        fVar21 = -5.0;
        if (-5.0 <= fVar20) {
          fVar21 = fVar20;
        }
        fVar20 = 5.0;
        if (fVar21 <= 5.0) {
          fVar20 = fVar21;
        }
        fVar20 = (fVar20 * 10.0 - *(float *)(lVar11 + 4)) * fVar19 * 3.0;
      }
      *puVar5 = uVar7;
      *(float *)(lVar11 + 4) = fVar20 + *(float *)(lVar11 + 4);
      break;
    case 6:
      fVar21 = extraout_XMM0_Da - *(float *)(lVar16 + 0x14);
      fVar20 = -0.3;
      if (-0.3 <= fVar21) {
        fVar20 = fVar21;
      }
      fVar22 = 0.3;
      if (fVar20 <= 0.3) {
        fVar22 = fVar20;
      }
      fVar20 = *(float *)(lVar11 + 4);
      *puVar5 = uVar7;
      *(float *)(lVar11 + 4) = (fVar22 * 20.0 - fVar20) * fVar19 + *(float *)(lVar11 + 4);
      if (-0.05 < fVar21) {
        *puVar4 = uVar7;
        *puVar3 = 2;
        *puVar5 = uVar7;
        *(undefined4 *)(lVar11 + 4) = 0;
      }
    }
  } while( true );
}

