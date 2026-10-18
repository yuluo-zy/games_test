// ADDRESS 0x141bc34f0 SIZE 1918 SIGNATURE void __cdecl replay_tree(void)

// WARNING: Removing unreachable block (ram,0x000141bc3d32)

void __cdecl system_tree::replay_tree::replay_tree(void)

{
  char *pcVar1;
  undefined4 *puVar2;
  float fVar3;
  float fVar4;
  undefined1 auVar5 [16];
  undefined1 auVar6 [16];
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
  longlong extraout_RAX;
  longlong *in_RCX;
  longlong lVar20;
  ulonglong uVar21;
  longlong *in_RDX;
  ulonglong uVar22;
  longlong lVar23;
  undefined8 uVar24;
  longlong lVar25;
  longlong *in_R8;
  undefined1 (*pauVar26) [16];
  undefined8 *puVar27;
  longlong lVar28;
  longlong *in_R9;
  ulonglong uVar29;
  undefined8 uVar30;
  longlong lVar31;
  longlong lVar32;
  char *pcVar33;
  undefined1 auVar34 [16];
  undefined1 auVar35 [16];
  float fVar36;
  undefined1 auVar37 [16];
  undefined1 auVar38 [16];
  longlong *in_stack_00000028;
  undefined8 local_178;
  undefined4 uStack_170;
  undefined4 uStack_16c;
  undefined8 local_168;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined4 uStack_f0;
  undefined7 uStack_e7;
  ulonglong local_b8;
  ulonglong *local_b0;
  
  lVar23 = *in_RCX;
  country_core::input::input_stream::InputStream::sync_point_count();
  fVar3 = *(float *)(lVar23 + 0x28);
  *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
  country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket();
  if (extraout_RAX == 0) {
    return;
  }
  local_b8 = 0;
  pcVar10 = *(char **)(extraout_RAX + 8);
  lVar23 = *(longlong *)(extraout_RAX + 0x10);
  lVar28 = *in_stack_00000028;
  uVar7 = *(undefined4 *)((longlong)in_R9 + 0x1c);
  lVar11 = *in_R9;
  puVar12 = (undefined4 *)in_R9[2];
  uVar8 = *(undefined4 *)((longlong)in_RDX + 0x1c);
  lVar13 = *in_RDX;
  puVar14 = (undefined4 *)in_RDX[2];
  uVar9 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  lVar15 = *in_R8;
  puVar16 = (undefined4 *)in_R8[2];
  lVar32 = 0;
  pcVar1 = pcVar10;
  while (pcVar33 = pcVar1, lVar31 = lVar32, pcVar33 != pcVar10 + lVar23 * 0x30) {
    pcVar1 = pcVar33 + 0x30;
    lVar32 = lVar31 + 1;
    if (*pcVar33 == '\b') {
      lVar25 = *(longlong *)(pcVar33 + 0x10);
      if (*(int *)(lVar25 + 0x10) == 2) {
        if ((*(float *)(pcVar33 + 0x18) == 0.0) && (!NAN(*(float *)(pcVar33 + 0x18)))) {
          if (pcVar33[0x1c] == '\0') {
            uVar24 = *(undefined8 *)(lVar25 + 0x18);
            *puVar12 = uVar7;
            lVar25 = *(longlong *)(lVar11 + 0x30);
            uVar30 = *(undefined8 *)(lVar11 + 0x40);
            if (lVar25 == *(longlong *)(lVar11 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                        ();
            }
            goto LAB_141bc38ac;
          }
          uVar30 = *(undefined8 *)(lVar25 + 0x18);
          uStack_100 = *(undefined8 *)(lVar25 + 0x38);
          uVar24 = *(undefined8 *)(lVar25 + 0x44);
          local_178 = *(undefined8 *)(lVar25 + 0x40);
          uStack_16c = *(undefined4 *)(lVar25 + 0x4c);
          alloc::string::impl$6::clone();
          uStack_f0 = (undefined4)((ulonglong)uVar24 >> 0x20);
          uStack_170 = uStack_f0;
          local_168 = CONCAT71(uStack_e7,*(undefined1 *)(lVar25 + 0x50));
          *puVar14 = uVar8;
          lVar25 = *(longlong *)(lVar13 + 0x30);
          uVar24 = *(undefined8 *)(lVar13 + 0x40);
          if (lVar25 == *(longlong *)(lVar13 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<indexmap::Bucket<tuple_<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>,alloc::alloc::Global>
                      ();
          }
LAB_141bc3975:
          lVar17 = *(longlong *)(lVar13 + 0x28);
          *(undefined8 *)(lVar17 + lVar25 * 0x48) = local_118;
          *(undefined8 *)(lVar17 + 8 + lVar25 * 0x48) = uStack_110;
          auVar5._8_8_ = uStack_100;
          auVar5._0_8_ = local_108;
          *(undefined1 (*) [16])(lVar17 + 0x10 + lVar25 * 0x48) = auVar5;
          puVar2 = (undefined4 *)(lVar17 + 0x20 + lVar25 * 0x48);
          *puVar2 = (undefined4)local_178;
          puVar2[1] = local_178._4_4_;
          puVar2[2] = uStack_170;
          puVar2[3] = uStack_16c;
          *(undefined8 *)(lVar17 + 0x30 + lVar25 * 0x48) = local_168;
          *(undefined8 *)(lVar17 + 0x38 + lVar25 * 0x48) = uVar30;
          *(undefined8 *)(lVar17 + 0x40 + lVar25 * 0x48) = uVar24;
          *(longlong *)(lVar13 + 0x30) = lVar25 + 1;
          lVar25 = lVar13;
LAB_141bc39c1:
          *(longlong *)(lVar25 + 0x40) = *(longlong *)(lVar25 + 0x40) + 1;
        }
      }
      else if (*(int *)(lVar25 + 0x10) == 1) {
        if ((*(float *)(pcVar33 + 0x18) == 0.0) && (!NAN(*(float *)(pcVar33 + 0x18)))) {
          if (pcVar33[0x1c] == '\0') {
            uVar24 = *(undefined8 *)(lVar25 + 0x30);
            uStack_110 = CONCAT44(uStack_110._4_4_,*(undefined4 *)(lVar25 + 0x28));
            local_118 = *(undefined8 *)(lVar25 + 0x20);
            *puVar16 = uVar9;
            lVar25 = *(longlong *)(lVar15 + 0x30);
            uVar30 = *(undefined8 *)(lVar15 + 0x40);
            if (lVar25 == *(longlong *)(lVar15 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                        ();
            }
          }
          else {
            uVar24 = *(undefined8 *)(lVar25 + 0x30);
            uStack_110 = CONCAT44(uStack_110._4_4_,*(undefined4 *)(lVar25 + 0x1c));
            local_118 = *(undefined8 *)(lVar25 + 0x14);
            *puVar16 = uVar9;
            lVar25 = *(longlong *)(lVar15 + 0x30);
            uVar30 = *(undefined8 *)(lVar15 + 0x40);
            if (lVar25 == *(longlong *)(lVar15 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                        ();
            }
          }
          lVar17 = *(longlong *)(lVar15 + 0x28);
          lVar20 = lVar25 * 0x20;
          *(undefined8 *)(lVar17 + lVar20) = uVar30;
          *(undefined8 *)(lVar17 + 8 + lVar20) = uVar24;
          *(undefined4 *)(lVar17 + 0x18 + lVar20) = (undefined4)uStack_110;
          *(undefined8 *)(lVar17 + 0x10 + lVar20) = local_118;
          *(longlong *)(lVar15 + 0x30) = lVar25 + 1;
          lVar25 = lVar15;
          goto LAB_141bc39c1;
        }
      }
      else if ((*(float *)(pcVar33 + 0x18) == 0.0) && (!NAN(*(float *)(pcVar33 + 0x18)))) {
        if (pcVar33[0x1c] == '\0') {
          uVar30 = *(undefined8 *)(lVar25 + 0x18);
          uStack_100 = *(undefined8 *)(lVar25 + 0x38);
          uVar24 = *(undefined8 *)(lVar25 + 0x44);
          local_178 = *(undefined8 *)(lVar25 + 0x40);
          uStack_16c = *(undefined4 *)(lVar25 + 0x4c);
          alloc::string::impl$6::clone();
          uStack_f0 = (undefined4)((ulonglong)uVar24 >> 0x20);
          uStack_170 = uStack_f0;
          local_168 = CONCAT71(uStack_e7,*(undefined1 *)(lVar25 + 0x50));
          *puVar14 = uVar8;
          lVar25 = *(longlong *)(lVar13 + 0x30);
          uVar24 = *(undefined8 *)(lVar13 + 0x40);
          if (lVar25 == *(longlong *)(lVar13 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<indexmap::Bucket<tuple_<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>,alloc::alloc::Global>
                      ();
          }
          goto LAB_141bc3975;
        }
        uVar24 = *(undefined8 *)(lVar25 + 0x18);
        *puVar12 = uVar7;
        lVar25 = *(longlong *)(lVar11 + 0x30);
        uVar30 = *(undefined8 *)(lVar11 + 0x40);
        if (lVar25 == *(longlong *)(lVar11 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
LAB_141bc38ac:
        lVar17 = *(longlong *)(lVar11 + 0x28);
        *(undefined8 *)(lVar17 + lVar25 * 0x10) = uVar30;
        *(undefined8 *)(lVar17 + 8 + lVar25 * 0x10) = uVar24;
        *(longlong *)(lVar11 + 0x30) = lVar25 + 1;
        *(longlong *)(lVar11 + 0x40) = *(longlong *)(lVar11 + 0x40) + 1;
      }
      if ((pcVar33[8] & 1U) == 0) {
        if (local_b8 == 0) {
          alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
        }
        *(longlong *)(local_b8 * 8 + 8) = lVar31;
        local_b8 = local_b8 + 1;
      }
      else {
        fVar36 = *(float *)(pcVar33 + 0xc) / fVar3;
        fVar4 = *(float *)(pcVar33 + 0x18);
        if (fVar36 <= 0.05) {
          fVar36 = 0.05;
        }
        if (fVar36 < fVar4) {
          if (local_b8 == 0) {
            alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
          }
          *(longlong *)(local_b8 * 8 + 8) = lVar31;
          local_b8 = local_b8 + 1;
          fVar4 = *(float *)(pcVar33 + 0x18);
        }
        *(float *)(pcVar33 + 0x18) = fVar4 + *(float *)(lVar28 + 0x28);
      }
    }
  }
  if (1 < local_b8) {
    uVar21 = local_b8 >> 1;
    if (local_b8 < 8) {
      uVar22 = 0;
    }
    else {
      uVar22 = uVar21 & 0xfffffffffffffffc;
      pauVar26 = (undefined1 (*) [16])(local_b8 * 8 + -8);
      uVar29 = 0;
      do {
        auVar5 = *(undefined1 (*) [16])(uVar29 * 8 + 8);
        auVar6 = *(undefined1 (*) [16])(uVar29 * 8 + 0x18);
        auVar34 = pauVar26[-1];
        auVar35 = *pauVar26;
        auVar38._0_8_ = auVar35._8_8_;
        auVar38._8_4_ = auVar35._0_4_;
        auVar38._12_4_ = auVar35._4_4_;
        auVar37._0_8_ = auVar34._8_8_;
        auVar37._8_4_ = auVar34._0_4_;
        auVar37._12_4_ = auVar34._4_4_;
        *(undefined1 (*) [16])(uVar29 * 8 + 8) = auVar38;
        *(undefined1 (*) [16])(uVar29 * 8 + 0x18) = auVar37;
        auVar34._0_8_ = auVar5._8_8_;
        auVar34._8_4_ = auVar5._0_4_;
        auVar34._12_4_ = auVar5._4_4_;
        *pauVar26 = auVar34;
        auVar35._0_8_ = auVar6._8_8_;
        auVar35._8_4_ = auVar6._0_4_;
        auVar35._12_4_ = auVar6._4_4_;
        pauVar26[-1] = auVar35;
        uVar29 = uVar29 + 4;
        pauVar26 = pauVar26 + -2;
      } while (uVar22 != uVar29);
      if (uVar21 == uVar22) goto joined_r0x000141bc3b54;
    }
    puVar27 = (undefined8 *)(uVar22 * 8);
    lVar23 = -uVar22;
    do {
      puVar27 = puVar27 + 1;
      uVar24 = *puVar27;
      *puVar27 = *(undefined8 *)(local_b8 * 8 + lVar23 * 8);
      *(undefined8 *)(local_b8 * 8 + lVar23 * 8) = uVar24;
      lVar23 = lVar23 + -1;
    } while (-lVar23 != uVar21);
  }
joined_r0x000141bc3b54:
  if (local_b8 != 0) {
    local_b0 = (ulonglong *)&DAT_00000008;
    do {
      uVar21 = *local_b0;
      uVar22 = *(ulonglong *)(extraout_RAX + 0x10);
      if (uVar22 <= uVar21) {
                    // WARNING: Subroutine does not return
        alloc::vec::impl$1::remove::assert_failed();
      }
      lVar23 = *(longlong *)(extraout_RAX + 8);
      lVar28 = uVar21 * 0x30;
      uVar29 = *(ulonglong *)(lVar23 + lVar28);
      plVar18 = (longlong *)((ulonglong *)(lVar23 + lVar28))[1];
      plVar19 = *(longlong **)(lVar23 + 0x10 + lVar28);
      memmove((void *)(lVar23 + lVar28),(void *)(lVar23 + lVar28 + 0x30),(~uVar21 + uVar22) * 0x30);
      *(ulonglong *)(extraout_RAX + 0x10) = uVar22 - 1;
      switch(uVar29 & 0xff) {
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


// LOCALS:
[
 {
  "name": "uStack_100",
  "type": "undefined8",
  "storage": "Stack[-0x100]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uStack_f0",
  "type": "undefined4",
  "storage": "Stack[-0xf0]:4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uStack_110",
  "type": "undefined8",
  "storage": "Stack[-0x110]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uStack_170",
  "type": "undefined4",
  "storage": "Stack[-0x170]:4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uStack_16c",
  "type": "undefined4",
  "storage": "Stack[-0x16c]:4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "fVar3",
  "type": "float",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "fVar4",
  "type": "float",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "pcVar1",
  "type": "char *",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "puVar2",
  "type": "undefined4 *",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uStack_e7",
  "type": "undefined7",
  "storage": "Stack[-0xe7]:7",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_168",
  "type": "undefined8",
  "storage": "Stack[-0x168]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_178",
  "type": "undefined8",
  "storage": "Stack[-0x178]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_b0",
  "type": "ulonglong *",
  "storage": "Stack[-0xb0]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar24",
  "type": "undefined8",
  "storage": "RBX:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "lVar25",
  "type": "longlong",
  "storage": "RBX:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_b8",
  "type": "ulonglong",
  "storage": "Stack[-0xb8]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar22",
  "type": "ulonglong",
  "storage": "RDX:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "lVar23",
  "type": "longlong",
  "storage": "RDX:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar21",
  "type": "ulonglong",
  "storage": "RCX:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "in_RDX",
  "type": "longlong *",
  "storage": "RDX:8",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "in_RCX",
  "type": "longlong *",
  "storage": "RCX:8",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "lVar20",
  "type": "longlong",
  "storage": "RCX:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar30",
  "type": "undefined8",
  "storage": "R12:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "lVar31",
  "type": "longlong",
  "storage": "R13:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "in_R9",
  "type": "longlong *",
  "storage": "R9:8",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "local_108",
  "type": "undefined8",
  "storage": "Stack[-0x108]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_118",
  "type": "undefined8",
  "storage": "Stack[-0x118]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar29",
  "type": "ulonglong",
  "storage": "R9:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "puVar27",
  "type": "undefined8 *",
  "storage": "R8:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "lVar28",
  "type": "longlong",
  "storage": "R8:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "in_R8",
  "type": "longlong *",
  "storage": "R8:8",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "pauVar26",
  "type": "undefined1[16] *",
  "storage": "R8:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "lVar11",
  "type": "longlong",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "puVar12",
  "type": "undefined4 *",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar9",
  "type": "undefined4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "pcVar10",
  "type": "char *",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar7",
  "type": "undefined4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar8",
  "type": "undefined4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "auVar5",
  "type": "undefined1[16]",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "auVar6",
  "type": "undefined1[16]",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "plVar19",
  "type": "longlong *",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "extraout_RAX",
  "type": "longlong",
  "storage": "RAX:8",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "lVar17",
  "type": "longlong",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "plVar18",
  "type": "longlong *",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "lVar15",
  "type": "longlong",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "puVar16",
  "type": "undefined4 *",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "lVar13",
  "type": "longlong",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "puVar14",
  "type": "undefined4 *",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "auVar38",
  "type": "undefined1[16]",
  "storage": "XMM3:16",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "in_stack_00000028",
  "type": "longlong *",
  "storage": "Stack[0x28]:8",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "fVar36",
  "type": "float",
  "storage": "XMM1_Da:4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "auVar37",
  "type": "undefined1[16]",
  "storage": "XMM2:16",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "auVar34",
  "type": "undefined1[16]",
  "storage": "XMM0:16",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "auVar35",
  "type": "undefined1[16]",
  "storage": "XMM0:16",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "lVar32",
  "type": "longlong",
  "storage": "R13:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "pcVar33",
  "type": "char *",
  "storage": "R15:8",
  "is_phantom": false,
  "in_decompiled_code": true
 }
]