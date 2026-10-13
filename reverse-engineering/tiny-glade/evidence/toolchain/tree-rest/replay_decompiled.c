
/* WARNING: Removing unreachable block (ram,0x000141bc3d32) */

void __cdecl system_tree::replay_tree::replay_tree(void)

{
  char *pcVar1;
  float fVar2;
  float fVar3;
  undefined1 auVar4 [16];
  undefined1 auVar5 [16];
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  char *pcVar10;
  longlong lVar11;
  undefined4 *puVar12;
  longlong lVar13;
  undefined4 *puVar14;
  longlong lVar15;
  undefined4 *puVar16;
  longlong lVar17;
  longlong *plVar18;
  longlong *plVar19;
  float fVar20;
  longlong extraout_RAX;
  longlong *in_RCX;
  longlong lVar21;
  ulonglong uVar22;
  longlong *in_RDX;
  ulonglong uVar23;
  longlong lVar24;
  undefined8 uVar25;
  longlong lVar26;
  longlong *in_R8;
  undefined1 (*pauVar27) [16];
  undefined8 *puVar28;
  longlong lVar29;
  longlong *in_R9;
  ulonglong uVar30;
  undefined8 uVar31;
  longlong lVar32;
  longlong lVar33;
  char *pcVar34;
  undefined1 auVar35 [16];
  undefined1 auVar36 [16];
  float fVar37;
  undefined1 auVar38 [16];
  undefined1 auVar39 [16];
  longlong *in_stack_00000028;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined4 uStack_f0;
  undefined7 uStack_e7;
  ulonglong local_b8;
  ulonglong *local_b0;
  
  lVar24 = *in_RCX;
  country_core::input::input_stream::InputStream::sync_point_count();
  fVar2 = *(float *)(lVar24 + 0x28);
  *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
  country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket();
  fVar20 = __real_3d4ccccd;
  if (extraout_RAX == 0) {
    return;
  }
  local_b8 = 0;
  pcVar10 = *(char **)(extraout_RAX + 8);
  lVar24 = *(longlong *)(extraout_RAX + 0x10);
  lVar29 = *in_stack_00000028;
  uVar6 = *(undefined4 *)((longlong)in_R9 + 0x1c);
  lVar11 = *in_R9;
  puVar12 = (undefined4 *)in_R9[2];
  uVar7 = *(undefined4 *)((longlong)in_RDX + 0x1c);
  lVar13 = *in_RDX;
  puVar14 = (undefined4 *)in_RDX[2];
  uVar8 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  lVar15 = *in_R8;
  puVar16 = (undefined4 *)in_R8[2];
  lVar33 = 0;
  pcVar1 = pcVar10;
  while (pcVar34 = pcVar1, lVar32 = lVar33, pcVar34 != pcVar10 + lVar24 * 0x30) {
    pcVar1 = pcVar34 + 0x30;
    lVar33 = lVar32 + 1;
    if (*pcVar34 == '\b') {
      lVar26 = *(longlong *)(pcVar34 + 0x10);
      if (*(int *)(lVar26 + 0x10) == 2) {
        if ((*(float *)(pcVar34 + 0x18) == 0.0) && (!NAN(*(float *)(pcVar34 + 0x18)))) {
          if (pcVar34[0x1c] == '\0') {
            uVar25 = *(undefined8 *)(lVar26 + 0x18);
            *puVar12 = uVar6;
            lVar26 = *(longlong *)(lVar11 + 0x30);
            uVar31 = *(undefined8 *)(lVar11 + 0x40);
            if (lVar26 == *(longlong *)(lVar11 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                        ();
            }
            goto LAB_141bc38ac;
          }
          uVar31 = *(undefined8 *)(lVar26 + 0x18);
          uStack_100 = *(undefined8 *)(lVar26 + 0x38);
          uVar25 = *(undefined8 *)(lVar26 + 0x44);
          local_178 = *(undefined8 *)(lVar26 + 0x40);
          uVar9 = *(undefined4 *)(lVar26 + 0x4c);
          alloc::string::impl$6::clone();
          uStack_f0 = (undefined4)((ulonglong)uVar25 >> 0x20);
          uStack_170 = CONCAT44(uVar9,uStack_f0);
          local_168 = CONCAT71(uStack_e7,*(undefined1 *)(lVar26 + 0x50));
          *puVar14 = uVar7;
          lVar26 = *(longlong *)(lVar13 + 0x30);
          uVar25 = *(undefined8 *)(lVar13 + 0x40);
          if (lVar26 == *(longlong *)(lVar13 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<indexmap::Bucket<tuple_<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>,alloc::alloc::Global>
                      ();
          }
LAB_141bc3975:
          lVar17 = *(longlong *)(lVar13 + 0x28);
          *(undefined8 *)(lVar17 + lVar26 * 0x48) = local_118;
          *(undefined8 *)(lVar17 + 8 + lVar26 * 0x48) = uStack_110;
          auVar4._8_8_ = uStack_100;
          auVar4._0_8_ = local_108;
          auVar5._8_8_ = uStack_170;
          auVar5._0_8_ = local_178;
          *(undefined1 (*) [16])(lVar17 + 0x10 + lVar26 * 0x48) = auVar4;
          *(undefined1 (*) [16])(lVar17 + 0x20 + lVar26 * 0x48) = auVar5;
          *(undefined8 *)(lVar17 + 0x30 + lVar26 * 0x48) = local_168;
          *(undefined8 *)(lVar17 + 0x38 + lVar26 * 0x48) = uVar31;
          *(undefined8 *)(lVar17 + 0x40 + lVar26 * 0x48) = uVar25;
          *(longlong *)(lVar13 + 0x30) = lVar26 + 1;
          lVar26 = lVar13;
LAB_141bc39c1:
          *(longlong *)(lVar26 + 0x40) = *(longlong *)(lVar26 + 0x40) + 1;
        }
      }
      else if (*(int *)(lVar26 + 0x10) == 1) {
        if ((*(float *)(pcVar34 + 0x18) == 0.0) && (!NAN(*(float *)(pcVar34 + 0x18)))) {
          if (pcVar34[0x1c] == '\0') {
            uVar25 = *(undefined8 *)(lVar26 + 0x30);
            uStack_110 = CONCAT44(uStack_110._4_4_,*(undefined4 *)(lVar26 + 0x28));
            local_118 = *(undefined8 *)(lVar26 + 0x20);
            *puVar16 = uVar8;
            lVar26 = *(longlong *)(lVar15 + 0x30);
            uVar31 = *(undefined8 *)(lVar15 + 0x40);
            if (lVar26 == *(longlong *)(lVar15 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                        ();
            }
          }
          else {
            uVar25 = *(undefined8 *)(lVar26 + 0x30);
            uStack_110 = CONCAT44(uStack_110._4_4_,*(undefined4 *)(lVar26 + 0x1c));
            local_118 = *(undefined8 *)(lVar26 + 0x14);
            *puVar16 = uVar8;
            lVar26 = *(longlong *)(lVar15 + 0x30);
            uVar31 = *(undefined8 *)(lVar15 + 0x40);
            if (lVar26 == *(longlong *)(lVar15 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                        ();
            }
          }
          lVar17 = *(longlong *)(lVar15 + 0x28);
          lVar21 = lVar26 * 0x20;
          *(undefined8 *)(lVar17 + lVar21) = uVar31;
          *(undefined8 *)(lVar17 + 8 + lVar21) = uVar25;
          *(undefined4 *)(lVar17 + 0x18 + lVar21) = (undefined4)uStack_110;
          *(undefined8 *)(lVar17 + 0x10 + lVar21) = local_118;
          *(longlong *)(lVar15 + 0x30) = lVar26 + 1;
          lVar26 = lVar15;
          goto LAB_141bc39c1;
        }
      }
      else if ((*(float *)(pcVar34 + 0x18) == 0.0) && (!NAN(*(float *)(pcVar34 + 0x18)))) {
        if (pcVar34[0x1c] == '\0') {
          uVar31 = *(undefined8 *)(lVar26 + 0x18);
          uStack_100 = *(undefined8 *)(lVar26 + 0x38);
          uVar25 = *(undefined8 *)(lVar26 + 0x44);
          local_178 = *(undefined8 *)(lVar26 + 0x40);
          uVar9 = *(undefined4 *)(lVar26 + 0x4c);
          alloc::string::impl$6::clone();
          uStack_f0 = (undefined4)((ulonglong)uVar25 >> 0x20);
          uStack_170 = CONCAT44(uVar9,uStack_f0);
          local_168 = CONCAT71(uStack_e7,*(undefined1 *)(lVar26 + 0x50));
          *puVar14 = uVar7;
          lVar26 = *(longlong *)(lVar13 + 0x30);
          uVar25 = *(undefined8 *)(lVar13 + 0x40);
          if (lVar26 == *(longlong *)(lVar13 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<indexmap::Bucket<tuple_<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>,alloc::alloc::Global>
                      ();
          }
          goto LAB_141bc3975;
        }
        uVar25 = *(undefined8 *)(lVar26 + 0x18);
        *puVar12 = uVar6;
        lVar26 = *(longlong *)(lVar11 + 0x30);
        uVar31 = *(undefined8 *)(lVar11 + 0x40);
        if (lVar26 == *(longlong *)(lVar11 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
LAB_141bc38ac:
        lVar17 = *(longlong *)(lVar11 + 0x28);
        *(undefined8 *)(lVar17 + lVar26 * 0x10) = uVar31;
        *(undefined8 *)(lVar17 + 8 + lVar26 * 0x10) = uVar25;
        *(longlong *)(lVar11 + 0x30) = lVar26 + 1;
        *(longlong *)(lVar11 + 0x40) = *(longlong *)(lVar11 + 0x40) + 1;
      }
      if ((pcVar34[8] & 1U) == 0) {
        if (local_b8 == 0) {
          alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
        }
        *(longlong *)(local_b8 * 8 + 8) = lVar32;
        local_b8 = local_b8 + 1;
      }
      else {
        fVar37 = *(float *)(pcVar34 + 0xc) / fVar2;
        fVar3 = *(float *)(pcVar34 + 0x18);
        if (fVar37 <= fVar20) {
          fVar37 = fVar20;
        }
        if (fVar37 < fVar3) {
          if (local_b8 == 0) {
            alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
          }
          *(longlong *)(local_b8 * 8 + 8) = lVar32;
          local_b8 = local_b8 + 1;
          fVar3 = *(float *)(pcVar34 + 0x18);
        }
        *(float *)(pcVar34 + 0x18) = fVar3 + *(float *)(lVar29 + 0x28);
      }
    }
  }
  if (1 < local_b8) {
    uVar22 = local_b8 >> 1;
    if (local_b8 < 8) {
      uVar23 = 0;
    }
    else {
      uVar23 = uVar22 & 0xfffffffffffffffc;
      pauVar27 = (undefined1 (*) [16])(local_b8 * 8 + -8);
      uVar30 = 0;
      do {
        auVar4 = *(undefined1 (*) [16])(uVar30 * 8 + 8);
        auVar5 = *(undefined1 (*) [16])(uVar30 * 8 + 0x18);
        auVar35 = pauVar27[-1];
        auVar36 = *pauVar27;
        auVar39._0_8_ = auVar36._8_8_;
        auVar39._8_4_ = auVar36._0_4_;
        auVar39._12_4_ = auVar36._4_4_;
        auVar38._0_8_ = auVar35._8_8_;
        auVar38._8_4_ = auVar35._0_4_;
        auVar38._12_4_ = auVar35._4_4_;
        *(undefined1 (*) [16])(uVar30 * 8 + 8) = auVar39;
        *(undefined1 (*) [16])(uVar30 * 8 + 0x18) = auVar38;
        auVar35._0_8_ = auVar4._8_8_;
        auVar35._8_4_ = auVar4._0_4_;
        auVar35._12_4_ = auVar4._4_4_;
        *pauVar27 = auVar35;
        auVar36._0_8_ = auVar5._8_8_;
        auVar36._8_4_ = auVar5._0_4_;
        auVar36._12_4_ = auVar5._4_4_;
        pauVar27[-1] = auVar36;
        uVar30 = uVar30 + 4;
        pauVar27 = pauVar27 + -2;
      } while (uVar23 != uVar30);
      if (uVar22 == uVar23) goto joined_r0x000141bc3b54;
    }
    puVar28 = (undefined8 *)(uVar23 * 8);
    lVar24 = -uVar23;
    do {
      puVar28 = puVar28 + 1;
      uVar25 = *puVar28;
      *puVar28 = *(undefined8 *)(local_b8 * 8 + lVar24 * 8);
      *(undefined8 *)(local_b8 * 8 + lVar24 * 8) = uVar25;
      lVar24 = lVar24 + -1;
    } while (-lVar24 != uVar22);
  }
joined_r0x000141bc3b54:
  if (local_b8 != 0) {
    local_b0 = (ulonglong *)&DAT_00000008;
    do {
      uVar22 = *local_b0;
      uVar23 = *(ulonglong *)(extraout_RAX + 0x10);
      if (uVar23 <= uVar22) {
                    /* WARNING: Subroutine does not return */
        alloc::vec::impl$1::remove::assert_failed();
      }
      lVar24 = *(longlong *)(extraout_RAX + 8);
      lVar29 = uVar22 * 0x30;
      uVar30 = *(ulonglong *)(lVar24 + lVar29);
      plVar18 = (longlong *)((ulonglong *)(lVar24 + lVar29))[1];
      plVar19 = *(longlong **)(lVar24 + 0x10 + lVar29);
      memmove((void *)(lVar24 + lVar29),(void *)(lVar24 + lVar29 + 0x30),(~uVar22 + uVar23) * 0x30);
      *(ulonglong *)(extraout_RAX + 0x10) = uVar23 - 1;
      switch(uVar30 & 0xff) {
      case 0:
        LOCK();
        *plVar19 = *plVar19 + -1;
        UNLOCK();
        if (*plVar19 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>();
        }
        break;
      case 1:
        LOCK();
        *plVar19 = *plVar19 + -1;
        UNLOCK();
        if (*plVar19 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>();
        }
        break;
      case 2:
        LOCK();
        *plVar19 = *plVar19 + -1;
        UNLOCK();
        if (*plVar19 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>();
        }
        break;
      case 3:
        LOCK();
        *plVar18 = *plVar18 + -1;
        UNLOCK();
        if (*plVar18 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 4:
        core::ptr::drop_in_place<country_core::systems::history::replay_wall::WallReplayCmd>();
        break;
      case 5:
        LOCK();
        *plVar19 = *plVar19 + -1;
        UNLOCK();
        if (*plVar19 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::roof_edit::RoofEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 6:
        LOCK();
        *plVar19 = *plVar19 + -1;
        UNLOCK();
        if (*plVar19 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 7:
        LOCK();
        *plVar19 = *plVar19 + -1;
        UNLOCK();
        if (*plVar19 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 8:
        LOCK();
        *plVar19 = *plVar19 + -1;
        UNLOCK();
        if (*plVar19 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::tree_edit::TreeEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 9:
        LOCK();
        *plVar19 = *plVar19 + -1;
        UNLOCK();
        if (*plVar19 == 0) {
          alloc::sync::Arc::
          drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo>,alloc::alloc::Global>
                    ();
        }
        break;
      default:
        LOCK();
        *plVar19 = *plVar19 + -1;
        UNLOCK();
        if (*plVar19 == 0) {
          alloc::sync::Arc::
          drop_slow<std::sync::poison::mutex::Mutex<winit::dpi::PhysicalSize<u32>_>,alloc::alloc::Global>
                    ();
        }
      }
      local_b0 = local_b0 + 1;
    } while (local_b0 != (ulonglong *)(local_b8 * 8 + 8));
  }
  return;
}

