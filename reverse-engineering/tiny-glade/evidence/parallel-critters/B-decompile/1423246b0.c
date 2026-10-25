
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl system_critters::sheep::sheep_unstuck::sheep_unstuck(void)

{
  uint *puVar1;
  uint *puVar2;
  undefined4 *puVar3;
  undefined4 *puVar4;
  char cVar5;
  byte bVar6;
  undefined4 uVar7;
  uint uVar8;
  ulonglong uVar9;
  longlong lVar10;
  longlong lVar11;
  float fVar12;
  uint uVar13;
  float fVar14;
  char extraout_AL;
  char extraout_AL_00;
  int iVar15;
  undefined8 *extraout_RAX;
  longlong lVar16;
  ulonglong uVar17;
  longlong lVar18;
  longlong lVar19;
  ulonglong uVar20;
  uint uVar21;
  longlong *in_RCX;
  longlong lVar22;
  longlong lVar23;
  ulonglong uVar24;
  uint *puVar25;
  ulonglong *puVar26;
  ulonglong uVar27;
  byte *pbVar28;
  ulonglong uVar29;
  ulonglong uVar30;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float fVar31;
  float fVar32;
  longlong *in_stack_00000028;
  char *in_stack_00000038;
  undefined8 *in_stack_00000040;
  undefined8 *in_stack_00000048;
  longlong local_1c0;
  longlong local_1a8;
  longlong local_1a0;
  longlong local_198;
  longlong local_190;
  ulonglong local_180;
  longlong local_170;
  uint local_fc;
  int local_e8;
  
  puVar26 = (ulonglong *)*in_stack_00000040;
  lVar16 = in_stack_00000040[1];
  uVar30 = *puVar26;
  uVar20 = 0;
  uVar17 = uVar30 - *(ulonglong *)(lVar16 + 0x18);
  if (uVar30 < *(ulonglong *)(lVar16 + 0x18)) {
    uVar17 = uVar20;
  }
  uVar24 = uVar30 - *(ulonglong *)(lVar16 + 0x38);
  if (uVar30 < *(ulonglong *)(lVar16 + 0x38)) {
    uVar24 = uVar20;
  }
  uVar30 = *(ulonglong *)(lVar16 + 0x30);
  uVar27 = uVar30 - uVar24;
  if (uVar30 < uVar24) {
    uVar27 = uVar20;
  }
  uVar9 = *(ulonglong *)(lVar16 + 0x10);
  uVar29 = uVar9 - uVar17;
  if (uVar9 < uVar17) {
    uVar29 = uVar20;
  }
  uVar20 = *(ulonglong *)(lVar16 + 0x40);
  *puVar26 = uVar20 - (uVar29 + uVar27);
  if ((uVar9 < uVar17 || uVar9 - uVar17 == 0) && (uVar30 < uVar24 || uVar30 - uVar24 == 0)) {
    puVar26 = (ulonglong *)*in_stack_00000048;
    lVar16 = in_stack_00000048[1];
    uVar30 = *puVar26;
    uVar20 = 0;
    uVar17 = uVar30 - *(ulonglong *)(lVar16 + 0x18);
    if (uVar30 < *(ulonglong *)(lVar16 + 0x18)) {
      uVar17 = uVar20;
    }
    uVar24 = uVar30 - *(ulonglong *)(lVar16 + 0x38);
    if (uVar30 < *(ulonglong *)(lVar16 + 0x38)) {
      uVar24 = uVar20;
    }
    uVar30 = *(ulonglong *)(lVar16 + 0x30);
    uVar27 = uVar30 - uVar24;
    if (uVar30 < uVar24) {
      uVar27 = uVar20;
    }
    uVar9 = *(ulonglong *)(lVar16 + 0x10);
    uVar29 = uVar9 - uVar17;
    if (uVar9 < uVar17) {
      uVar29 = uVar20;
    }
    uVar20 = *(ulonglong *)(lVar16 + 0x40);
    *puVar26 = uVar20 - (uVar29 + uVar27);
    if ((uVar9 < uVar17 || uVar9 - uVar17 == 0) && (uVar30 < uVar24 || uVar30 - uVar24 == 0)) {
      if (*in_stack_00000038 == '\0') {
        *in_stack_00000038 = '\x01';
        return;
      }
      frozen::FrozenMap::
      get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
                ();
      if (extraout_RAX != (undefined8 *)0x0) {
        if ((extraout_RAX[3] != 0x3f) ||
           (iVar15 = memcmp(&anon_75d1013106e08612ad9ec328065f3af1_1_llvm_9468065370954387650,
                            (void *)extraout_RAX[2],0x3f), iVar15 != 0)) {
                    /* WARNING: Subroutine does not return */
          core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
        }
        lVar16 = (**(code **)(extraout_RAX[1] + 0x18))(*extraout_RAX);
        if (lVar16 != 0) {
          toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
          fVar14 = __real_41200000;
          uVar13 = ___xmm_7fffffff7fffffff7fffffff7fffffff;
          fVar12 = __real_3f000000;
          lVar16 = *in_RCX;
          lVar10 = in_RCX[1];
          uVar7 = *(undefined4 *)((longlong)in_RCX + 0x14);
          puVar25 = *(uint **)(lVar16 + 0xf8);
          puVar1 = puVar25 + *(longlong *)(lVar16 + 0x100);
          cVar5 = *(char *)(lVar16 + 0x108);
          lVar11 = *in_stack_00000028;
          local_170 = 8;
          local_1c0 = 0;
          local_1a0 = 0;
          local_1a8 = 0;
          uVar30 = 0;
          uVar17 = 0;
          do {
            if (cVar5 == '\0') {
              if (uVar17 == uVar30) {
                do {
                  if (puVar25 == puVar1) {
                    return;
                  }
                  uVar8 = *puVar25;
                  puVar25 = puVar25 + 1;
                  uVar30 = *(ulonglong *)
                            (*(longlong *)(lVar10 + 0x100) + 0x58 + (ulonglong)uVar8 * 0xa0);
                } while (uVar30 == 0);
                lVar18 = *(longlong *)(lVar10 + 0x100) + (ulonglong)uVar8 * 0xa0;
                uVar17 = (ulonglong)*(uint *)(lVar18 + 0x94);
                lVar19 = *(longlong *)(lVar10 + 0x1a8);
                lVar22 = lVar19 + uVar17 * 0x48;
                lVar23 = *(longlong *)(lVar19 + 0x38 + uVar17 * 0x48);
                if (*(ulonglong *)(lVar16 + 0x110) < *(ulonglong *)(lVar19 + 0x40 + uVar17 * 0x48))
                {
                  uVar17 = *(ulonglong *)(lVar23 + *(ulonglong *)(lVar16 + 0x110) * 8);
                  lVar22 = *(longlong *)(lVar22 + 0x18);
                  if (uVar17 == 0) goto LAB_142324b31;
                  local_1c0 = *(longlong *)(lVar22 + 0x10 + ~uVar17 * 0x30);
                }
                else {
                  lVar22 = *(longlong *)(lVar22 + 0x18);
LAB_142324b31:
                  local_1c0 = 0;
                }
                lVar19 = ~*(ulonglong *)(lVar23 + *(longlong *)(lVar16 + 0x118) * 8) * 0x30;
                local_1a0 = *(longlong *)(lVar22 + 0x10 + lVar19);
                local_190 = *(longlong *)(lVar22 + 0x28 + lVar19);
                lVar19 = ~*(ulonglong *)(lVar23 + *(longlong *)(lVar16 + 0x120) * 8) * 0x30;
                local_1a8 = *(longlong *)(lVar22 + 0x10 + lVar19);
                local_198 = *(longlong *)(lVar22 + 0x28 + lVar19);
                local_170 = *(longlong *)(lVar18 + 0x50);
                uVar17 = 0;
              }
              uVar20 = (ulonglong)*(uint *)(local_170 + 8 + uVar17 * 0x10);
              local_180 = uVar17;
            }
            else {
              if (uVar17 == uVar30) {
                do {
                  if (puVar25 == puVar1) {
                    return;
                  }
                  uVar8 = *puVar25;
                  puVar25 = puVar25 + 1;
                  uVar30 = *(ulonglong *)
                            (*(longlong *)(lVar10 + 0x1a8) + 0x10 + (ulonglong)uVar8 * 0x48);
                } while (uVar30 == 0);
                lVar22 = *(longlong *)(lVar10 + 0x1a8) + (ulonglong)uVar8 * 0x48;
                lVar19 = *(longlong *)(lVar22 + 0x38);
                if (*(ulonglong *)(lVar16 + 0x110) < *(ulonglong *)(lVar22 + 0x40)) {
                  uVar17 = *(ulonglong *)(lVar19 + *(ulonglong *)(lVar16 + 0x110) * 8);
                  lVar22 = *(longlong *)(lVar22 + 0x18);
                  if (uVar17 == 0) goto LAB_142324abd;
                  local_1c0 = *(longlong *)(lVar22 + 0x10 + ~uVar17 * 0x30);
                }
                else {
                  lVar22 = *(longlong *)(lVar22 + 0x18);
LAB_142324abd:
                  local_1c0 = 0;
                }
                lVar23 = ~*(ulonglong *)(lVar19 + *(longlong *)(lVar16 + 0x118) * 8) * 0x30;
                local_1a0 = *(longlong *)(lVar22 + 0x10 + lVar23);
                local_190 = *(longlong *)(lVar22 + 0x28 + lVar23);
                lVar19 = ~*(ulonglong *)(lVar19 + *(longlong *)(lVar16 + 0x120) * 8) * 0x30;
                local_1a8 = *(longlong *)(lVar22 + 0x10 + lVar19);
                local_198 = *(longlong *)(lVar22 + 0x28 + lVar19);
                uVar17 = 0;
              }
              uVar20 = uVar17 & 0xffffffff;
              local_180 = uVar17;
            }
            puVar2 = (uint *)(local_1a0 + uVar20 * 8);
            puVar3 = (undefined4 *)(local_190 + uVar20 * 4);
            pbVar28 = (byte *)(uVar20 * 0x68 + local_1a8);
            puVar4 = (undefined4 *)(local_198 + uVar20 * 4);
            country_core::systems::collision::world::RaycastWorld::legacy_spherecast();
            if (local_e8 != 2) {
              *puVar3 = uVar7;
              *puVar2 = 1;
              puVar2[1] = local_fc;
              goto LAB_142324cfe;
            }
            country_core::resources::terrain_rock_mask::impl$1::sample_at_world_xz();
            ::utils::float_grid::impl$0::
            bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                      ();
            uVar8 = *puVar2;
            if (((extraout_AL == '\0') || (6 < uVar8)) || ((0x54U >> (uVar8 & 0x1f) & 1) == 0)) {
              uVar21 = uVar8 - 2;
              if (4 < uVar21) {
                uVar21 = 3;
              }
              if (uVar21 == 0) {
                if (extraout_XMM0_Da <= 0.0) goto LAB_142324dd2;
                country_core::systems::save::save_backup_list::BackupSaveFileKind::is_corrupted();
                *puVar3 = uVar7;
                if (extraout_AL_00 == '\0') {
                  *puVar2 = 4;
                  goto LAB_142324cfe;
                }
                goto LAB_142324cd1;
              }
              if ((uVar21 == 3) && (extraout_AL == '\0' && extraout_XMM0_Da <= 0.0)) {
                fVar32 = *(float *)(uVar20 * 0x30 + local_1c0 + 0x14);
                country_core::resources::terrain_height::TerrainHeightsData::
                sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
                if (fVar32 <= extraout_XMM0_Da_00) {
                  if (*puVar2 == 2) goto LAB_142324ddb;
                }
                else {
                  *puVar3 = uVar7;
                  *puVar2 = 6;
                }
                goto LAB_142324cfe;
              }
LAB_142324dd2:
              if (uVar8 != 2) goto LAB_142324cfe;
LAB_142324ddb:
              fVar32 = *(float *)(pbVar28 + 4);
              bVar6 = *pbVar28;
              country_core::resources::terrain_height::TerrainHeightsData::
              sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
              fVar31 = extraout_XMM0_Da_01;
              if (extraout_XMM0_Da_01 <= fVar12) {
                fVar31 = fVar12;
              }
              if ((((bVar6 & 1) != 0) &&
                  (fVar32 = (fVar31 - fVar32) / *(float *)(lVar11 + 0x28),
                  (uint)ABS(fVar32) < 0x7f800000)) && (fVar14 < (float)((uint)fVar32 & uVar13))) {
                *puVar3 = uVar7;
                *puVar2 = 0;
              }
              *puVar4 = uVar7;
              pbVar28[0] = 1;
              pbVar28[1] = 0;
              pbVar28[2] = 0;
              pbVar28[3] = 0;
              *(float *)(pbVar28 + 4) = fVar31;
              uVar8 = *puVar2;
            }
            else {
              *puVar3 = uVar7;
LAB_142324cd1:
              *puVar2 = 0;
LAB_142324cfe:
              *puVar4 = uVar7;
              pbVar28[0] = 0;
              pbVar28[1] = 0;
              pbVar28[2] = 0;
              pbVar28[3] = 0;
              uVar8 = *puVar2;
            }
            uVar17 = local_180 + 1;
            if (uVar8 != 2) {
              uVar17 = local_180 + 1;
              *puVar4 = uVar7;
              if (pbVar28[0x30] == 0) {
                pbVar28[0x34] = 0;
                pbVar28[0x35] = 0;
                pbVar28[0x36] = 0;
                pbVar28[0x37] = 0;
              }
            }
          } while( true );
        }
      }
                    /* WARNING: Subroutine does not return */
      core::option::unwrap_failed();
    }
  }
  *puVar26 = uVar20;
  *in_stack_00000038 = '\0';
  return;
}

