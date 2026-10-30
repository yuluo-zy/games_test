// ADDR 0x1408238a0
// SIG void __cdecl create_or_restore_decorator_backup(void)
// ORIG system::create_or_restore_decorator_backup


void __cdecl system_decorator::backup::create_or_restore_decorator_backup(void)

{
  undefined1 auVar1 [16];
  char cVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 *puVar5;
  longlong lVar6;
  longlong lVar7;
  undefined8 uVar8;
  undefined4 *puVar9;
  longlong *plVar10;
  ushort uVar11;
  uint uVar12;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong extraout_RAX_06;
  longlong in_RDX;
  ulonglong uVar13;
  undefined8 uVar14;
  longlong *in_R8;
  longlong *in_R9;
  longlong lVar15;
  longlong lVar16;
  ulonglong uVar17;
  uint uVar18;
  longlong lVar19;
  undefined1 (*pauVar20) [16];
  bool bVar21;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  longlong *in_stack_00000038;
  longlong *in_stack_00000040;
  longlong in_stack_00000048;
  longlong local_3c8;
  longlong lStack_3c0;
  ulonglong local_3b8;
  longlong lStack_3b0;
  longlong local_3a8;
  undefined1 (*local_3a0) [16];
  ushort local_390;
  longlong local_388;
  undefined8 local_1d8;
  undefined8 uStack_1d0;
  undefined8 local_1a8;
  undefined8 uStack_1a0;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined1 local_148;
  undefined1 local_147;
  undefined1 local_146;
  longlong local_130;
  undefined1 (*local_e8) [16];
  undefined1 (*pauStack_e0) [16];
  undefined1 *local_d8;
  ushort uStack_d0;
  undefined6 uStack_ce;
  longlong local_b0;
  longlong local_68;
  ulonglong local_60;
  longlong local_58;
  
  uVar3 = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
  puVar5 = (undefined4 *)in_stack_00000040[2];
  *puVar5 = uVar3;
  lVar6 = *in_stack_00000040;
  cVar2 = *(char *)(lVar6 + 0x282);
  *(undefined1 *)(lVar6 + 0x282) = 0;
  if (cVar2 != '\0') {
    local_58 = lStack_3b0;
    local_68 = lStack_3c0;
    if (cVar2 == '\x01') {
      country_core::resources::walls::decorator_archivist::DecoratorArchivist::all_decorators();
      uVar12 = (uint)local_390;
      local_60 = local_388;
      uVar13 = local_3b8;
      while (uVar13 != 2) {
        if (local_3a8 == 0) {
joined_r0x000140823adf:
          if ((uVar13 & 1) == 0) {
            local_3a8 = 0;
          }
          else {
            local_3a8 = 0;
            bVar21 = local_58 != 0;
            local_58 = 0;
            if (bVar21) {
              local_3a8 = 0;
              local_58 = 0;
              uVar18 = uVar12;
              goto LAB_1408239a9;
            }
          }
          break;
        }
        if (local_60 == 0) {
          local_60 = 0;
          goto joined_r0x000140823adf;
        }
        if ((short)uVar12 == 0) {
          do {
            auVar1 = *local_3a0;
            uVar11 = (ushort)(SUB161(auVar1 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                     (ushort)(byte)(auVar1[0xf] >> 7) << 0xf;
            local_3a8 = local_3a8 + -0x980;
            local_3a0 = local_3a0 + 1;
          } while (uVar11 == 0xffff);
          uVar12 = ~(uint)uVar11;
          uVar18 = uVar12 & -(uint)uVar11 - 2;
        }
        else {
          uVar18 = uVar12 - 1 & uVar12;
        }
        local_60 = local_60 + -1;
        for (; (uVar12 & 1) == 0; uVar12 = uVar12 >> 1 | 0x80000000) {
        }
        country_core::resources::walls::decorator_storage::impl$16::from();
        uVar12 = uVar18;
        if (extraout_RAX_00 == 3) goto joined_r0x000140823adf;
LAB_1408239a9:
        *puVar5 = uVar3;
        country_core::resources::walls::decorator_storage::DecoratorStorage::get_or_insert_default()
        ;
        country_core::resources::walls::decorator_storage::DecoratorStorage::get();
        if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
        country_core::resources::walls::decorator_storage::Decorators::tracked_clone_from<tuple_<>_>
                  ();
        uVar12 = uVar18;
      }
      if (((byte)local_3c8 & local_68 != 0) != 0) {
        uVar13 = 2;
        local_68 = 0;
        uVar18 = uVar12;
        goto LAB_1408239a9;
      }
      *puVar5 = uVar3;
      lVar7 = *in_R8;
      hashbrown::raw::impl$13::
      clone_from<tuple_<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,tuple_<>_>,alloc::alloc::Global>
                ();
      uVar8 = *(undefined8 *)(lVar7 + 0x28);
      *(undefined8 *)(lVar6 + 0x250) = *(undefined8 *)(lVar7 + 0x20);
      *(undefined8 *)(lVar6 + 600) = uVar8;
      *puVar5 = uVar3;
      lVar7 = *in_R9;
      hashbrown::raw::impl$13::
      clone_from<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::sync::Arc<system_decorator::blueprint::DecoratorBlueprint,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
      uVar8 = *(undefined8 *)(lVar7 + 0x28);
      *(undefined8 *)(lVar6 + 0x1f0) = *(undefined8 *)(lVar7 + 0x20);
      *(undefined8 *)(lVar6 + 0x1f8) = uVar8;
      *puVar5 = uVar3;
      lVar7 = *in_stack_00000028;
      hashbrown::raw::impl$13::
      clone_from<tuple_<country_core::resources::walls::decorator::DecoratorId,utils::components::transform::Transform>,alloc::alloc::Global>
                ();
      uVar8 = *(undefined8 *)(lVar7 + 0x28);
      *(undefined8 *)(lVar6 + 0x220) = *(undefined8 *)(lVar7 + 0x20);
      *(undefined8 *)(lVar6 + 0x228) = uVar8;
      *puVar5 = uVar3;
      country_core::resources::walls::decorator::impl$12::clone();
      if (*(longlong *)(lVar6 + 0x268) != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      *(ulonglong *)(lVar6 + 0x270) = local_3b8;
      *(longlong *)(lVar6 + 0x278) = lStack_3b0;
      *(longlong *)(lVar6 + 0x260) = local_3c8;
      *(longlong *)(lVar6 + 0x268) = lStack_3c0;
      *puVar5 = uVar3;
      lVar7 = *(longlong *)(*in_stack_00000038 + 8);
      uVar13 = *(ulonglong *)(*in_stack_00000038 + 0x10);
      uVar17 = *(ulonglong *)(lVar6 + 0x10);
      if (uVar13 <= *(ulonglong *)(lVar6 + 0x10)) {
        *(ulonglong *)(lVar6 + 0x10) = uVar13;
        uVar17 = uVar13;
      }
      if (uVar17 != 0) {
        lVar16 = *(longlong *)(lVar6 + 8);
        lVar15 = 8;
        do {
          uVar8 = *(undefined8 *)(lVar7 + -8 + lVar15);
          uVar14 = 3;
          switch(uVar8) {
          case 3:
            break;
          case 4:
            uVar14 = 4;
            break;
          case 5:
            uVar14 = 5;
            break;
          default:
            local_3b8 = *(ulonglong *)(lVar7 + 0x10 + lVar15);
            local_3c8 = *(longlong *)(lVar7 + lVar15);
            lStack_3c0 = ((longlong *)(lVar7 + lVar15))[1];
            uVar14 = uVar8;
          }
          *(undefined8 *)(lVar16 + -8 + lVar15) = uVar14;
          *(longlong *)(lVar16 + lVar15) = local_3c8;
          ((longlong *)(lVar16 + lVar15))[1] = lStack_3c0;
          *(ulonglong *)(lVar16 + 0x10 + lVar15) = local_3b8;
          lVar15 = lVar15 + 0x20;
          uVar17 = uVar17 - 1;
        } while (uVar17 != 0);
      }
      alloc::vec::spec_extend::impl$3::
      spec_extend<enum2_<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,core::slice::iter::Iter<enum2_<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>_>,alloc::alloc::Global>
                ();
      *puVar5 = uVar3;
      *(undefined1 *)(lVar6 + 0x281) = 1;
    }
    else if (*(char *)(lVar6 + 0x281) != '\0') {
      lVar7 = *in_stack_00000038;
      cVar2 = *(char *)(lVar7 + 0x48);
      country_core::resources::walls::decorator_archivist::DecoratorArchivist::all_storages();
      uVar12 = (uint)local_390;
      uVar4 = *(undefined4 *)(in_RDX + 0x1c);
      puVar9 = *(undefined4 **)(in_RDX + 0x10);
      lVar16 = local_3c8;
      uVar13 = local_3b8;
      lVar15 = local_388;
      lVar19 = local_3a8;
      pauVar20 = local_3a0;
LAB_140823d50:
      if (uVar13 == 5) {
LAB_140823e10:
        if (lVar16 - 3U < 2) goto LAB_1408240d4;
        lVar16 = 3;
        uVar13 = 5;
      }
      else if (lVar19 == 0) {
LAB_140823de1:
        uVar17 = uVar13 - 3;
        uVar13 = 3;
        lVar19 = 0;
        if (uVar17 < 2) goto LAB_140823e10;
        lVar19 = 0;
      }
      else {
        if (lVar15 == 0) {
          lVar15 = 0;
          goto LAB_140823de1;
        }
        if ((short)uVar12 == 0) {
          do {
            auVar1 = *pauVar20;
            uVar11 = (ushort)(SUB161(auVar1 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                     (ushort)(byte)(auVar1[0xf] >> 7) << 0xf;
            lVar19 = lVar19 + -0x980;
            pauVar20 = pauVar20 + 1;
          } while (uVar11 == 0xffff);
          uVar18 = ~(uint)uVar11;
          uVar12 = uVar18 & -(uint)uVar11 - 2;
        }
        else {
          uVar18 = uVar12;
          uVar12 = uVar12 - 1 & uVar12;
        }
        lVar15 = lVar15 + -1;
        for (; (uVar18 & 1) == 0; uVar18 = uVar18 >> 1 | 0x80000000) {
        }
        country_core::resources::rt::impl$5::from();
        if (extraout_RAX_01 == 3) goto LAB_140823de1;
      }
      *puVar9 = uVar4;
      country_core::resources::walls::decorator_storage::DecoratorStorage::get_mut();
      if (extraout_RAX_02 != 0) {
        country_core::resources::walls::decorator_storage::Decorators::
        clear<country_core::resources::walls::decorator_storage::DecoratorToStorageMap>();
      }
      goto LAB_140823d50;
    }
  }
  cVar2 = *(char *)(lVar6 + 0x280);
joined_r0x000140824613:
  if (cVar2 != '\0') {
    *puVar5 = uVar3;
    impl$3::default();
    country_core::resources::walls::decorator_storage::impl$12::clone_from();
    hashbrown::raw::impl$13::
    clone_from<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::sync::Arc<system_decorator::blueprint::DecoratorBlueprint,alloc::alloc::Global>_>,alloc::alloc::Global>
              ();
    *(undefined8 *)(lVar6 + 0x1f0) = local_1d8;
    *(undefined8 *)(lVar6 + 0x1f8) = uStack_1d0;
    hashbrown::raw::impl$13::
    clone_from<tuple_<country_core::resources::walls::decorator::DecoratorId,utils::components::transform::Transform>,alloc::alloc::Global>
              ();
    *(undefined8 *)(lVar6 + 0x220) = local_1a8;
    *(undefined8 *)(lVar6 + 0x228) = uStack_1a0;
    hashbrown::raw::impl$13::
    clone_from<tuple_<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,tuple_<>_>,alloc::alloc::Global>
              ();
    *(undefined8 *)(lVar6 + 0x250) = local_178;
    *(undefined8 *)(lVar6 + 600) = uStack_170;
    country_core::resources::walls::decorator::impl$12::clone();
    if (*(longlong *)(lVar6 + 0x268) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    *(undefined1 **)(lVar6 + 0x270) = local_d8;
    *(ulonglong *)(lVar6 + 0x278) = CONCAT62(uStack_ce,uStack_d0);
    *(undefined1 (**) [16])(lVar6 + 0x260) = local_e8;
    *(undefined1 (**) [16])(lVar6 + 0x268) = pauStack_e0;
    uVar13 = *(ulonglong *)(lVar6 + 0x10);
    if (local_3b8 <= *(ulonglong *)(lVar6 + 0x10)) {
      *(ulonglong *)(lVar6 + 0x10) = local_3b8;
      uVar13 = local_3b8;
    }
    if (uVar13 != 0) {
      lVar7 = *(longlong *)(lVar6 + 8);
      lVar16 = 8;
      do {
        uVar8 = *(undefined8 *)(lStack_3c0 + -8 + lVar16);
        uVar14 = 3;
        switch(uVar8) {
        case 3:
          break;
        case 4:
          uVar14 = 4;
          break;
        case 5:
          uVar14 = 5;
          break;
        default:
          local_d8 = *(undefined1 **)(lStack_3c0 + 0x10 + lVar16);
          local_e8 = *(undefined1 (**) [16])(lStack_3c0 + lVar16);
          pauStack_e0 = (undefined1 (*) [16])((undefined8 *)(lStack_3c0 + lVar16))[1];
          uVar14 = uVar8;
        }
        *(undefined8 *)(lVar7 + -8 + lVar16) = uVar14;
        *(undefined8 *)(lVar7 + lVar16) = local_e8;
        ((undefined8 *)(lVar7 + lVar16))[1] = pauStack_e0;
        *(undefined1 **)(lVar7 + 0x10 + lVar16) = local_d8;
        lVar16 = lVar16 + 0x20;
        uVar13 = uVar13 - 1;
      } while (uVar13 != 0);
    }
    alloc::vec::spec_extend::impl$3::
    spec_extend<enum2_<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,core::slice::iter::Iter<enum2_<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>_>,alloc::alloc::Global>
              ();
    *(undefined1 *)(lVar6 + 0x282) = local_146;
    *(undefined1 *)(lVar6 + 0x280) = local_148;
    *(undefined1 *)(lVar6 + 0x281) = local_147;
    core::ptr::drop_in_place<system_decorator::backup::DecoratorBackup>();
  }
  return;
LAB_1408240d4:
  country_core::resources::walls::decorator_archivist::DecoratorArchivist::all_decorators();
  local_60 = local_3b8;
  uVar12 = (uint)local_390;
  local_b0 = local_388;
  do {
    if (local_60 == 2) {
LAB_14082426a:
      if (((byte)local_3c8 & local_68 != 0) == 0) {
        *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
        lVar16 = *in_R8;
        hashbrown::raw::impl$13::
        clone_from<tuple_<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,tuple_<>_>,alloc::alloc::Global>
                  ();
        uVar8 = *(undefined8 *)(lVar6 + 600);
        *(undefined8 *)(lVar16 + 0x20) = *(undefined8 *)(lVar6 + 0x250);
        *(undefined8 *)(lVar16 + 0x28) = uVar8;
        *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
        lVar16 = *in_R9;
        hashbrown::raw::impl$13::
        clone_from<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::sync::Arc<system_decorator::blueprint::DecoratorBlueprint,alloc::alloc::Global>_>,alloc::alloc::Global>
                  ();
        uVar8 = *(undefined8 *)(lVar6 + 0x1f8);
        *(undefined8 *)(lVar16 + 0x20) = *(undefined8 *)(lVar6 + 0x1f0);
        *(undefined8 *)(lVar16 + 0x28) = uVar8;
        *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
        lVar16 = *in_stack_00000028;
        hashbrown::raw::impl$13::
        clone_from<tuple_<country_core::resources::walls::decorator::DecoratorId,utils::components::transform::Transform>,alloc::alloc::Global>
                  ();
        uVar8 = *(undefined8 *)(lVar6 + 0x228);
        *(undefined8 *)(lVar16 + 0x20) = *(undefined8 *)(lVar6 + 0x220);
        *(undefined8 *)(lVar16 + 0x28) = uVar8;
        *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
        plVar10 = (longlong *)*in_stack_00000030;
        country_core::resources::walls::decorator::impl$12::clone();
        if (plVar10[1] != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        plVar10[2] = local_3b8;
        plVar10[3] = lStack_3b0;
        *plVar10 = local_3c8;
        plVar10[1] = lStack_3c0;
        *(undefined4 *)in_stack_00000038[2] = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
        lVar16 = *(longlong *)(lVar6 + 8);
        uVar13 = *(ulonglong *)(lVar6 + 0x10);
        uVar17 = *(ulonglong *)(lVar7 + 0x10);
        if (uVar13 <= *(ulonglong *)(lVar7 + 0x10)) {
          *(ulonglong *)(lVar7 + 0x10) = uVar13;
          uVar17 = uVar13;
        }
        if (uVar17 != 0) {
          lVar7 = *(longlong *)(lVar7 + 8);
          lVar15 = 8;
          do {
            uVar8 = *(undefined8 *)(lVar16 + -8 + lVar15);
            uVar14 = 3;
            switch(uVar8) {
            case 3:
              break;
            case 4:
              uVar14 = 4;
              break;
            case 5:
              uVar14 = 5;
              break;
            default:
              local_3b8 = *(ulonglong *)(lVar16 + 0x10 + lVar15);
              local_3c8 = *(longlong *)(lVar16 + lVar15);
              lStack_3c0 = ((longlong *)(lVar16 + lVar15))[1];
              uVar14 = uVar8;
            }
            *(undefined8 *)(lVar7 + -8 + lVar15) = uVar14;
            *(longlong *)(lVar7 + lVar15) = local_3c8;
            ((longlong *)(lVar7 + lVar15))[1] = lStack_3c0;
            *(ulonglong *)(lVar7 + 0x10 + lVar15) = local_3b8;
            lVar15 = lVar15 + 0x20;
            uVar17 = uVar17 - 1;
          } while (uVar17 != 0);
        }
        alloc::vec::spec_extend::impl$3::
        spec_extend<enum2_<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,core::slice::iter::Iter<enum2_<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>_>,alloc::alloc::Global>
                  ();
        **(undefined4 **)(in_stack_00000048 + 0x10) = *(undefined4 *)(in_stack_00000048 + 0x1c);
        country_core::resources::stairs::stairs_state::StairsState::restore_backup();
        cVar2 = *(char *)(lVar6 + 0x280);
        goto joined_r0x000140824613;
      }
      local_60 = 2;
      local_68 = 0;
    }
    else {
      if (local_3a8 == 0) {
joined_r0x000140824263:
        if ((local_60 & 1) == 0) {
          local_3a8 = 0;
        }
        else {
          local_3a8 = 0;
          bVar21 = local_58 != 0;
          local_58 = 0;
          if (bVar21) {
            local_3a8 = 0;
            local_58 = 0;
            goto LAB_1408242aa;
          }
        }
        goto LAB_14082426a;
      }
      if (local_b0 == 0) {
        local_b0 = 0;
        goto joined_r0x000140824263;
      }
      if ((short)uVar12 == 0) {
        do {
          auVar1 = *local_3a0;
          uVar11 = (ushort)(SUB161(auVar1 >> 7,0) & 1) | (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1
                   | (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                   (ushort)(byte)(auVar1[0xf] >> 7) << 0xf;
          local_3a8 = local_3a8 + -0x980;
          local_3a0 = local_3a0 + 1;
        } while (uVar11 == 0xffff);
        uVar12 = ~(uint)uVar11;
        uVar18 = uVar12 & -(uint)uVar11 - 2;
      }
      else {
        uVar18 = uVar12 - 1 & uVar12;
      }
      local_b0 = local_b0 + -1;
      for (; (uVar12 & 1) == 0; uVar12 = uVar12 >> 1 | 0x80000000) {
      }
      country_core::resources::walls::decorator_storage::impl$16::from();
      uVar12 = uVar18;
      if (extraout_RAX_03 == 3) goto joined_r0x000140824263;
    }
LAB_1408242aa:
    *puVar9 = uVar4;
    country_core::resources::walls::decorator_storage::DecoratorStorage::get_mut();
    if (extraout_RAX_04 == 0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    country_core::resources::walls::decorator_storage::DecoratorStorage::get();
    country_core::resources::walls::decorator_storage::DecoMoveToStorage::new();
    country_core::resources::walls::decorator_storage::Decorators::
    tracked_clone_from<country_core::resources::walls::decorator_storage::DecoMoveToStorage>();
    if (cVar2 != '\0') {
      local_e8 = *(undefined1 (**) [16])(extraout_RAX_04 + 0x30);
      local_d8 = *local_e8 + *(longlong *)(extraout_RAX_04 + 0x38) + 1;
      auVar1 = *local_e8;
      uStack_d0 = ~((ushort)(SUB161(auVar1 >> 7,0) & 1) | (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1
                    | (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                    (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                    (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                    (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                    (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                    (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                    (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                    (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                    (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                    (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                    (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                    (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                    (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                   (ushort)(byte)(auVar1[0xf] >> 7) << 0xf);
      pauStack_e0 = local_e8 + 1;
      country_core::resources::walls::decorator_storage::impl$3::next();
      while (local_130 != 0) {
        minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
        if (*(char *)(extraout_RAX_05 + 0x47) == '\x01') {
          country_core::resources::walls::decorator_storage::DecoratorMaybeMut::get_mut();
          *(undefined1 *)(extraout_RAX_06 + 0x47) = 0;
        }
        country_core::resources::walls::decorator_storage::impl$3::next();
      }
    }
  } while( true );
}


