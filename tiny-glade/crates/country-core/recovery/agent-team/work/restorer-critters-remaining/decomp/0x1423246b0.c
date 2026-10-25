
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
  char extraout_AL;
  char extraout_AL_00;
  int iVar12;
  undefined8 *extraout_RAX;
  longlong lVar13;
  ulonglong uVar14;
  longlong lVar15;
  longlong lVar16;
  ulonglong uVar17;
  uint uVar18;
  longlong *in_RCX;
  longlong lVar19;
  longlong lVar20;
  ulonglong uVar21;
  uint *puVar22;
  ulonglong *puVar23;
  ulonglong uVar24;
  byte *pbVar25;
  ulonglong uVar26;
  ulonglong uVar27;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float fVar28;
  float fVar29;
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
  
  puVar23 = (ulonglong *)*in_stack_00000040;
  lVar13 = in_stack_00000040[1];
  uVar27 = *puVar23;
  uVar17 = 0;
  uVar14 = uVar27 - *(ulonglong *)(lVar13 + 0x18);
  if (uVar27 < *(ulonglong *)(lVar13 + 0x18)) {
    uVar14 = uVar17;
  }
  uVar21 = uVar27 - *(ulonglong *)(lVar13 + 0x38);
  if (uVar27 < *(ulonglong *)(lVar13 + 0x38)) {
    uVar21 = uVar17;
  }
  uVar27 = *(ulonglong *)(lVar13 + 0x30);
  uVar24 = uVar27 - uVar21;
  if (uVar27 < uVar21) {
    uVar24 = uVar17;
  }
  uVar9 = *(ulonglong *)(lVar13 + 0x10);
  uVar26 = uVar9 - uVar14;
  if (uVar9 < uVar14) {
    uVar26 = uVar17;
  }
  uVar17 = *(ulonglong *)(lVar13 + 0x40);
  *puVar23 = uVar17 - (uVar26 + uVar24);
  if ((uVar9 < uVar14 || uVar9 - uVar14 == 0) && (uVar27 < uVar21 || uVar27 - uVar21 == 0)) {
    puVar23 = (ulonglong *)*in_stack_00000048;
    lVar13 = in_stack_00000048[1];
    uVar27 = *puVar23;
    uVar17 = 0;
    uVar14 = uVar27 - *(ulonglong *)(lVar13 + 0x18);
    if (uVar27 < *(ulonglong *)(lVar13 + 0x18)) {
      uVar14 = uVar17;
    }
    uVar21 = uVar27 - *(ulonglong *)(lVar13 + 0x38);
    if (uVar27 < *(ulonglong *)(lVar13 + 0x38)) {
      uVar21 = uVar17;
    }
    uVar27 = *(ulonglong *)(lVar13 + 0x30);
    uVar24 = uVar27 - uVar21;
    if (uVar27 < uVar21) {
      uVar24 = uVar17;
    }
    uVar9 = *(ulonglong *)(lVar13 + 0x10);
    uVar26 = uVar9 - uVar14;
    if (uVar9 < uVar14) {
      uVar26 = uVar17;
    }
    uVar17 = *(ulonglong *)(lVar13 + 0x40);
    *puVar23 = uVar17 - (uVar26 + uVar24);
    if ((uVar9 < uVar14 || uVar9 - uVar14 == 0) && (uVar27 < uVar21 || uVar27 - uVar21 == 0)) {
      if (*in_stack_00000038 == '\0') {
        *in_stack_00000038 = '\x01';
        return;
      }
      frozen::FrozenMap::
      get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
                ();
      if (extraout_RAX != (undefined8 *)0x0) {
        if ((extraout_RAX[3] != 0x3f) ||
           (iVar12 = memcmp(&anon_75d1013106e08612ad9ec328065f3af1_1_llvm_9468065370954387650,
                            (void *)extraout_RAX[2],0x3f), iVar12 != 0)) {
                    // WARNING: Subroutine does not return
          core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
        }
        lVar13 = (**(code **)(extraout_RAX[1] + 0x18))(*extraout_RAX);
        if (lVar13 != 0) {
          toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
          lVar13 = *in_RCX;
          lVar10 = in_RCX[1];
          uVar7 = *(undefined4 *)((longlong)in_RCX + 0x14);
          puVar22 = *(uint **)(lVar13 + 0xf8);
          puVar1 = puVar22 + *(longlong *)(lVar13 + 0x100);
          cVar5 = *(char *)(lVar13 + 0x108);
          lVar11 = *in_stack_00000028;
          local_170 = 8;
          local_1c0 = 0;
          local_1a0 = 0;
          local_1a8 = 0;
          uVar27 = 0;
          uVar14 = 0;
          do {
            if (cVar5 == '\0') {
              if (uVar14 == uVar27) {
                do {
                  if (puVar22 == puVar1) {
                    return;
                  }
                  uVar8 = *puVar22;
                  puVar22 = puVar22 + 1;
                  uVar27 = *(ulonglong *)
                            (*(longlong *)(lVar10 + 0x100) + 0x58 + (ulonglong)uVar8 * 0xa0);
                } while (uVar27 == 0);
                lVar15 = *(longlong *)(lVar10 + 0x100) + (ulonglong)uVar8 * 0xa0;
                uVar14 = (ulonglong)*(uint *)(lVar15 + 0x94);
                lVar16 = *(longlong *)(lVar10 + 0x1a8);
                lVar19 = lVar16 + uVar14 * 0x48;
                lVar20 = *(longlong *)(lVar16 + 0x38 + uVar14 * 0x48);
                if (*(ulonglong *)(lVar13 + 0x110) < *(ulonglong *)(lVar16 + 0x40 + uVar14 * 0x48))
                {
                  uVar14 = *(ulonglong *)(lVar20 + *(ulonglong *)(lVar13 + 0x110) * 8);
                  lVar19 = *(longlong *)(lVar19 + 0x18);
                  if (uVar14 == 0) goto LAB_142324b31;
                  local_1c0 = *(longlong *)(lVar19 + 0x10 + ~uVar14 * 0x30);
                }
                else {
                  lVar19 = *(longlong *)(lVar19 + 0x18);
LAB_142324b31:
                  local_1c0 = 0;
                }
                lVar16 = ~*(ulonglong *)(lVar20 + *(longlong *)(lVar13 + 0x118) * 8) * 0x30;
                local_1a0 = *(longlong *)(lVar19 + 0x10 + lVar16);
                local_190 = *(longlong *)(lVar19 + 0x28 + lVar16);
                lVar16 = ~*(ulonglong *)(lVar20 + *(longlong *)(lVar13 + 0x120) * 8) * 0x30;
                local_1a8 = *(longlong *)(lVar19 + 0x10 + lVar16);
                local_198 = *(longlong *)(lVar19 + 0x28 + lVar16);
                local_170 = *(longlong *)(lVar15 + 0x50);
                uVar14 = 0;
              }
              uVar17 = (ulonglong)*(uint *)(local_170 + 8 + uVar14 * 0x10);
              local_180 = uVar14;
            }
            else {
              if (uVar14 == uVar27) {
                do {
                  if (puVar22 == puVar1) {
                    return;
                  }
                  uVar8 = *puVar22;
                  puVar22 = puVar22 + 1;
                  uVar27 = *(ulonglong *)
                            (*(longlong *)(lVar10 + 0x1a8) + 0x10 + (ulonglong)uVar8 * 0x48);
                } while (uVar27 == 0);
                lVar19 = *(longlong *)(lVar10 + 0x1a8) + (ulonglong)uVar8 * 0x48;
                lVar16 = *(longlong *)(lVar19 + 0x38);
                if (*(ulonglong *)(lVar13 + 0x110) < *(ulonglong *)(lVar19 + 0x40)) {
                  uVar14 = *(ulonglong *)(lVar16 + *(ulonglong *)(lVar13 + 0x110) * 8);
                  lVar19 = *(longlong *)(lVar19 + 0x18);
                  if (uVar14 == 0) goto LAB_142324abd;
                  local_1c0 = *(longlong *)(lVar19 + 0x10 + ~uVar14 * 0x30);
                }
                else {
                  lVar19 = *(longlong *)(lVar19 + 0x18);
LAB_142324abd:
                  local_1c0 = 0;
                }
                lVar20 = ~*(ulonglong *)(lVar16 + *(longlong *)(lVar13 + 0x118) * 8) * 0x30;
                local_1a0 = *(longlong *)(lVar19 + 0x10 + lVar20);
                local_190 = *(longlong *)(lVar19 + 0x28 + lVar20);
                lVar16 = ~*(ulonglong *)(lVar16 + *(longlong *)(lVar13 + 0x120) * 8) * 0x30;
                local_1a8 = *(longlong *)(lVar19 + 0x10 + lVar16);
                local_198 = *(longlong *)(lVar19 + 0x28 + lVar16);
                uVar14 = 0;
              }
              uVar17 = uVar14 & 0xffffffff;
              local_180 = uVar14;
            }
            puVar2 = (uint *)(local_1a0 + uVar17 * 8);
            puVar3 = (undefined4 *)(local_190 + uVar17 * 4);
            pbVar25 = (byte *)(uVar17 * 0x68 + local_1a8);
            puVar4 = (undefined4 *)(local_198 + uVar17 * 4);
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
              uVar18 = uVar8 - 2;
              if (4 < uVar18) {
                uVar18 = 3;
              }
              if (uVar18 == 0) {
                if (extraout_XMM0_Da <= 0.0) goto LAB_142324dd2;
                country_core::systems::save::save_backup_list::BackupSaveFileKind::is_corrupted();
                *puVar3 = uVar7;
                if (extraout_AL_00 == '\0') {
                  *puVar2 = 4;
                  goto LAB_142324cfe;
                }
                goto LAB_142324cd1;
              }
              if ((uVar18 == 3) && (extraout_AL == '\0' && extraout_XMM0_Da <= 0.0)) {
                fVar29 = *(float *)(uVar17 * 0x30 + local_1c0 + 0x14);
                country_core::resources::terrain_height::TerrainHeightsData::
                sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
                if (fVar29 <= extraout_XMM0_Da_00) {
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
              fVar29 = *(float *)(pbVar25 + 4);
              bVar6 = *pbVar25;
              country_core::resources::terrain_height::TerrainHeightsData::
              sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
              fVar28 = extraout_XMM0_Da_01;
              if (extraout_XMM0_Da_01 <= 0.5) {
                fVar28 = 0.5;
              }
              if ((((bVar6 & 1) != 0) &&
                  (fVar29 = (fVar28 - fVar29) / *(float *)(lVar11 + 0x28),
                  (uint)ABS(fVar29) < 0x7f800000)) && (10.0 < ABS(fVar29))) {
                *puVar3 = uVar7;
                *puVar2 = 0;
              }
              *puVar4 = uVar7;
              pbVar25[0] = 1;
              pbVar25[1] = 0;
              pbVar25[2] = 0;
              pbVar25[3] = 0;
              *(float *)(pbVar25 + 4) = fVar28;
              uVar8 = *puVar2;
            }
            else {
              *puVar3 = uVar7;
LAB_142324cd1:
              *puVar2 = 0;
LAB_142324cfe:
              *puVar4 = uVar7;
              pbVar25[0] = 0;
              pbVar25[1] = 0;
              pbVar25[2] = 0;
              pbVar25[3] = 0;
              uVar8 = *puVar2;
            }
            uVar14 = local_180 + 1;
            if (uVar8 != 2) {
              uVar14 = local_180 + 1;
              *puVar4 = uVar7;
              if (pbVar25[0x30] == 0) {
                pbVar25[0x34] = 0;
                pbVar25[0x35] = 0;
                pbVar25[0x36] = 0;
                pbVar25[0x37] = 0;
              }
            }
          } while( true );
        }
      }
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
  }
  *puVar23 = uVar17;
  *in_stack_00000038 = '\0';
  return;
}

