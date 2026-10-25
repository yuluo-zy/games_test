
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
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  longlong lVar17;
  ulonglong uVar18;
  longlong *in_RCX;
  longlong lVar19;
  longlong lVar20;
  ulonglong uVar21;
  longlong lVar22;
  ulonglong uVar23;
  longlong *in_R8;
  uint *puVar24;
  float fVar25;
  float extraout_XMM0_Da;
  undefined4 extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float fVar26;
  float fVar27;
  float fVar28;
  longlong local_148;
  longlong local_140;
  longlong local_138;
  longlong local_130;
  longlong local_128;
  longlong local_120;
  longlong local_110;
  
  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  fVar25 = expf(*(float *)(*in_R8 + 0x28) * __real_c0400000);
  fVar16 = __real_41a00000;
  fVar15 = __real_bd4ccccd;
  fVar14 = __real_40800000;
  fVar13 = __real_41200000;
  fVar12 = __real_be99999a;
  fVar11 = __real_3e99999a;
  fVar25 = __real_3f800000 - fVar25;
  lVar9 = *in_RCX;
  lVar10 = in_RCX[1];
  uVar7 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar24 = *(uint **)(lVar9 + 0xf8);
  puVar1 = puVar24 + *(longlong *)(lVar9 + 0x100);
  cVar6 = *(char *)(lVar9 + 0x108);
  local_110 = 8;
  local_138 = 0;
  local_140 = 0;
  local_148 = 0;
  uVar21 = 0;
  uVar23 = 0;
  do {
    if (cVar6 == '\0') {
      if (uVar23 == uVar21) {
        do {
          if (puVar24 == puVar1) {
            return;
          }
          uVar8 = *puVar24;
          puVar24 = puVar24 + 1;
          uVar21 = *(ulonglong *)(*(longlong *)(lVar10 + 0x100) + 0x58 + (ulonglong)uVar8 * 0xa0);
        } while (uVar21 == 0);
        lVar19 = *(longlong *)(lVar10 + 0x100) + (ulonglong)uVar8 * 0xa0;
        uVar23 = (ulonglong)*(uint *)(lVar19 + 0x94);
        lVar17 = *(longlong *)(*(longlong *)(lVar10 + 0x1a8) + 0x18 + uVar23 * 0x48);
        lVar22 = *(longlong *)(*(longlong *)(lVar10 + 0x1a8) + 0x38 + uVar23 * 0x48);
        lVar20 = ~*(ulonglong *)(lVar22 + *(longlong *)(lVar9 + 0x110) * 8) * 0x30;
        local_140 = *(longlong *)(lVar17 + 0x10 + lVar20);
        local_120 = *(longlong *)(lVar17 + 0x28 + lVar20);
        lVar20 = ~*(ulonglong *)(lVar22 + *(longlong *)(lVar9 + 0x118) * 8) * 0x30;
        local_138 = *(longlong *)(lVar17 + 0x10 + lVar20);
        local_128 = *(longlong *)(lVar17 + 0x28 + lVar20);
        lVar22 = ~*(ulonglong *)(lVar22 + *(longlong *)(lVar9 + 0x120) * 8) * 0x30;
        local_148 = *(longlong *)(lVar17 + 0x10 + lVar22);
        local_130 = *(longlong *)(lVar17 + 0x28 + lVar22);
        local_110 = *(longlong *)(lVar19 + 0x50);
        uVar23 = 0;
      }
      uVar18 = (ulonglong)*(uint *)(local_110 + 8 + uVar23 * 0x10);
    }
    else {
      if (uVar23 == uVar21) {
        do {
          if (puVar24 == puVar1) {
            return;
          }
          uVar8 = *puVar24;
          puVar24 = puVar24 + 1;
          uVar21 = *(ulonglong *)(*(longlong *)(lVar10 + 0x1a8) + 0x10 + (ulonglong)uVar8 * 0x48);
        } while (uVar21 == 0);
        lVar17 = *(longlong *)(lVar10 + 0x1a8) + (ulonglong)uVar8 * 0x48;
        lVar22 = *(longlong *)(lVar17 + 0x18);
        lVar17 = *(longlong *)(lVar17 + 0x38);
        lVar19 = ~*(ulonglong *)(lVar17 + *(longlong *)(lVar9 + 0x110) * 8) * 0x30;
        local_140 = *(longlong *)(lVar22 + 0x10 + lVar19);
        local_120 = *(longlong *)(lVar22 + 0x28 + lVar19);
        lVar19 = ~*(ulonglong *)(lVar17 + *(longlong *)(lVar9 + 0x118) * 8) * 0x30;
        local_138 = *(longlong *)(lVar22 + 0x10 + lVar19);
        local_128 = *(longlong *)(lVar22 + 0x28 + lVar19);
        lVar17 = ~*(ulonglong *)(lVar17 + *(longlong *)(lVar9 + 0x120) * 8) * 0x30;
        local_148 = *(longlong *)(lVar22 + 0x10 + lVar17);
        local_130 = *(longlong *)(lVar22 + 0x28 + lVar17);
        uVar23 = 0;
      }
      uVar18 = uVar23 & 0xffffffff;
    }
    lVar17 = local_148 + uVar18 * 0xc;
    lVar22 = uVar18 * 0x30 + local_140;
    puVar2 = (undefined4 *)(local_120 + uVar18 * 4);
    puVar3 = (uint *)(local_138 + uVar18 * 8);
    puVar4 = (undefined4 *)(local_128 + uVar18 * 4);
    puVar5 = (undefined4 *)(local_130 + uVar18 * 4);
    uVar23 = uVar23 + 1;
    country_core::resources::terrain_height::TerrainHeightsData::
    sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
    *puVar4 = uVar7;
    switch(*puVar3) {
    case 2:
      *puVar2 = uVar7;
      *(float *)(lVar22 + 0x14) = extraout_XMM0_Da;
      break;
    case 3:
                    /* WARNING: Subroutine does not return */
      core::panicking::panic();
    case 4:
      ::utils::time_continuous_lerp();
      *puVar2 = uVar7;
      *(undefined4 *)(lVar22 + 0x14) = extraout_XMM0_Da_00;
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
      if (extraout_XMM0_Da_01 <= 0.0) {
        *puVar4 = uVar7;
        *puVar3 = 2;
      }
      break;
    default:
      fVar26 = extraout_XMM0_Da;
      if ((*puVar3 & 1) != 0) {
        fVar26 = (float)puVar3[1];
      }
      fVar26 = (fVar26 + fVar14) - *(float *)(lVar22 + 0x14);
      if (extraout_XMM0_Da <= *(float *)(lVar22 + 0x14)) {
        fVar27 = __real_c0000000;
        if (__real_c0000000 <= fVar26) {
          fVar27 = fVar26;
        }
        fVar26 = __real_40000000;
        if (fVar27 <= __real_40000000) {
          fVar26 = fVar27;
        }
        fVar26 = (fVar26 * fVar13 - *(float *)(lVar17 + 4)) * fVar25;
      }
      else {
        *puVar2 = uVar7;
        *(float *)(lVar22 + 0x14) = extraout_XMM0_Da;
        fVar27 = __real_c0a00000;
        if (__real_c0a00000 <= fVar26) {
          fVar27 = fVar26;
        }
        fVar26 = __real_40a00000;
        if (fVar27 <= __real_40a00000) {
          fVar26 = fVar27;
        }
        fVar26 = (fVar26 * fVar13 - *(float *)(lVar17 + 4)) * fVar25 * __real_40400000;
      }
      *puVar5 = uVar7;
      *(float *)(lVar17 + 4) = fVar26 + *(float *)(lVar17 + 4);
      break;
    case 6:
      fVar27 = extraout_XMM0_Da - *(float *)(lVar22 + 0x14);
      fVar26 = fVar12;
      if (fVar12 <= fVar27) {
        fVar26 = fVar27;
      }
      fVar28 = fVar11;
      if (fVar26 <= fVar11) {
        fVar28 = fVar26;
      }
      fVar26 = *(float *)(lVar17 + 4);
      *puVar5 = uVar7;
      *(float *)(lVar17 + 4) = (fVar28 * fVar16 - fVar26) * fVar25 + *(float *)(lVar17 + 4);
      if (fVar15 < fVar27) {
        *puVar4 = uVar7;
        *puVar3 = 2;
        *puVar5 = uVar7;
        *(undefined4 *)(lVar17 + 4) = 0;
      }
    }
  } while( true );
}

