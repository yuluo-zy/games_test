
void __cdecl
system_critters::sheep::sheep_react_to_being_inside_buildings::react_to_being_inside_buildings(void)

{
  uint *puVar1;
  float fVar2;
  undefined4 uVar3;
  char cVar4;
  undefined4 uVar5;
  int iVar6;
  undefined4 uVar7;
  ulonglong *puVar8;
  longlong lVar9;
  longlong lVar10;
  longlong lVar11;
  longlong lVar12;
  longlong lVar13;
  undefined4 *puVar14;
  size_t _Size;
  undefined8 uVar15;
  undefined8 uVar16;
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  char extraout_AL;
  longlong lVar21;
  longlong extraout_RAX;
  longlong lVar22;
  ulonglong uVar23;
  ulonglong extraout_RAX_00;
  longlong extraout_RAX_01;
  void *_Dst;
  uint uVar24;
  undefined8 *in_RCX;
  longlong lVar25;
  ulonglong uVar26;
  longlong lVar27;
  longlong *in_RDX;
  ulonglong uVar28;
  longlong lVar29;
  ulonglong uVar30;
  longlong lVar31;
  longlong lVar32;
  uint *puVar33;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar34;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  longlong local_1e8;
  longlong local_1e0;
  ulonglong local_1c8;
  longlong local_190;
  
  puVar8 = (ulonglong *)*in_RCX;
  lVar9 = in_RCX[1];
  uVar30 = *puVar8;
  uVar28 = 0;
  uVar23 = uVar30 - *(ulonglong *)(lVar9 + 0x18);
  if (uVar30 < *(ulonglong *)(lVar9 + 0x18)) {
    uVar23 = uVar28;
  }
  uVar26 = uVar30 - *(ulonglong *)(lVar9 + 0x38);
  if (uVar30 < *(ulonglong *)(lVar9 + 0x38)) {
    uVar26 = uVar28;
  }
  lVar25 = uVar23 * 0x10 + *(longlong *)(lVar9 + 8);
  uVar30 = *(ulonglong *)(lVar9 + 0x10) - uVar23;
  if (*(ulonglong *)(lVar9 + 0x10) < uVar23) {
    lVar25 = 8;
    uVar30 = uVar28;
  }
  lVar21 = uVar26 * 0x10 + *(longlong *)(lVar9 + 0x28);
  uVar23 = *(ulonglong *)(lVar9 + 0x30) - uVar26;
  if (*(ulonglong *)(lVar9 + 0x30) < uVar26) {
    lVar21 = 8;
    uVar23 = uVar28;
  }
  *puVar8 = (*(longlong *)(lVar9 + 0x40) - uVar23) - uVar30;
  lVar31 = uVar30 * 0x10 + lVar25;
  lVar32 = uVar23 * 0x10 + lVar21;
  lVar9 = *in_RDX;
  lVar10 = in_RDX[1];
  lVar11 = *in_stack_00000028;
  lVar12 = in_stack_00000028[1];
  uVar5 = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
  lVar13 = *in_stack_00000030;
  puVar14 = (undefined4 *)in_stack_00000030[2];
LAB_14231f7a7:
  do {
    if ((lVar25 == 0) || (lVar25 == lVar31)) {
      if ((lVar21 == 0) || (lVar21 == lVar32)) {
        return;
      }
      lVar21 = lVar21 + 0x10;
      lVar25 = 0;
    }
    else {
      lVar25 = lVar25 + 0x10;
    }
    *puVar8 = *puVar8 + 1;
    country_core::resources::walls::public_walls::PublicWalls::get();
    if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
  } while ((1 < *(uint *)(extraout_RAX + 600)) || (1.5 < *(float *)(extraout_RAX + 0x280)));
  country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape();
  puVar33 = *(uint **)(lVar9 + 0xf8);
  puVar1 = puVar33 + *(longlong *)(lVar9 + 0x100);
  cVar4 = *(char *)(lVar9 + 0x108);
  local_190 = 8;
  local_1e0 = 0;
  local_1e8 = 0;
  uVar30 = 0;
  local_1c8 = 0;
  do {
    if (cVar4 == '\0') {
      if (local_1c8 == uVar30) {
        do {
          if (puVar33 == puVar1) goto LAB_14231f7a7;
          uVar24 = *puVar33;
          puVar33 = puVar33 + 1;
          uVar30 = *(ulonglong *)(*(longlong *)(lVar10 + 0x100) + 0x58 + (ulonglong)uVar24 * 0xa0);
        } while (uVar30 == 0);
        lVar22 = *(longlong *)(lVar10 + 0x100) + (ulonglong)uVar24 * 0xa0;
        uVar23 = (ulonglong)*(uint *)(lVar22 + 0x94);
        lVar29 = *(longlong *)(lVar10 + 0x1a8) + uVar23 * 0x48;
        uVar23 = *(ulonglong *)(*(longlong *)(lVar10 + 0x1a8) + 0x40 + uVar23 * 0x48);
        if ((*(ulonglong *)(lVar9 + 0x110) < uVar23) &&
           (*(longlong *)(*(longlong *)(lVar29 + 0x38) + *(ulonglong *)(lVar9 + 0x110) * 8) != 0)) {
          uVar28 = *(ulonglong *)(lVar9 + 0x118);
          if (uVar23 <= uVar28) goto LAB_14231fb13;
LAB_14231fa94:
          uVar28 = *(ulonglong *)(*(longlong *)(lVar29 + 0x38) + uVar28 * 8);
          if (uVar28 == 0) goto LAB_14231fb13;
          local_1e0 = *(longlong *)(*(longlong *)(lVar29 + 0x18) + 0x10 + ~uVar28 * 0x30);
          uVar28 = *(ulonglong *)(lVar9 + 0x120);
          if (uVar28 < uVar23) goto LAB_14231fb2d;
LAB_14231fb69:
          local_1e8 = 0;
        }
        else {
          uVar28 = *(ulonglong *)(lVar9 + 0x118);
          if (uVar28 < uVar23) goto LAB_14231fa94;
LAB_14231fb13:
          local_1e0 = 0;
          uVar28 = *(ulonglong *)(lVar9 + 0x120);
          if (uVar23 <= uVar28) goto LAB_14231fb69;
LAB_14231fb2d:
          uVar23 = *(ulonglong *)(*(longlong *)(lVar29 + 0x38) + uVar28 * 8);
          if (uVar23 == 0) goto LAB_14231fb69;
          local_1e8 = *(longlong *)(*(longlong *)(lVar29 + 0x18) + 0x10 + ~uVar23 * 0x30);
        }
        local_190 = *(longlong *)(lVar22 + 0x50);
        local_1c8 = 0;
      }
      uVar23 = (ulonglong)*(uint *)(local_190 + 8 + local_1c8 * 0x10);
    }
    else {
      if (local_1c8 == uVar30) {
        do {
          if (puVar33 == puVar1) goto LAB_14231f7a7;
          uVar24 = *puVar33;
          puVar33 = puVar33 + 1;
          uVar30 = *(ulonglong *)(*(longlong *)(lVar10 + 0x1a8) + 0x10 + (ulonglong)uVar24 * 0x48);
        } while (uVar30 == 0);
        lVar29 = *(longlong *)(lVar10 + 0x1a8) + (ulonglong)uVar24 * 0x48;
        uVar23 = *(ulonglong *)(lVar29 + 0x40);
        if ((*(ulonglong *)(lVar9 + 0x110) < uVar23) &&
           (*(longlong *)(*(longlong *)(lVar29 + 0x38) + *(ulonglong *)(lVar9 + 0x110) * 8) != 0)) {
          uVar28 = *(ulonglong *)(lVar9 + 0x118);
          if (uVar23 <= uVar28) goto LAB_14231fad1;
LAB_14231fa39:
          uVar28 = *(ulonglong *)(*(longlong *)(lVar29 + 0x38) + uVar28 * 8);
          if (uVar28 == 0) goto LAB_14231fad1;
          local_1e0 = *(longlong *)(*(longlong *)(lVar29 + 0x18) + 0x10 + ~uVar28 * 0x30);
          uVar28 = *(ulonglong *)(lVar9 + 0x120);
          if (uVar28 < uVar23) goto LAB_14231faeb;
LAB_14231fb55:
          local_1e8 = 0;
        }
        else {
          uVar28 = *(ulonglong *)(lVar9 + 0x118);
          if (uVar28 < uVar23) goto LAB_14231fa39;
LAB_14231fad1:
          local_1e0 = 0;
          uVar28 = *(ulonglong *)(lVar9 + 0x120);
          if (uVar23 <= uVar28) goto LAB_14231fb55;
LAB_14231faeb:
          uVar23 = *(ulonglong *)(*(longlong *)(lVar29 + 0x38) + uVar28 * 8);
          if (uVar23 == 0) goto LAB_14231fb55;
          local_1e8 = *(longlong *)(*(longlong *)(lVar29 + 0x18) + 0x10 + ~uVar23 * 0x30);
        }
        local_1c8 = 0;
      }
      uVar23 = local_1c8 & 0xffffffff;
    }
    lVar29 = uVar23 * 0x30 + local_1e0;
    fVar2 = *(float *)(extraout_RAX + 0x280);
    country_core::resources::walls::public_wall_state::PublicWallState::max_y();
    country_core::resources::walls::public_walls::PublicWalls::roof_entity();
    fVar34 = 0.0;
    if (((extraout_RAX_00 >> 0x20 != 0) &&
        (uVar28 = extraout_RAX_00 & 0xffffffff, uVar28 < *(ulonglong *)(lVar12 + 0x10))) &&
       (lVar22 = *(longlong *)(lVar12 + 8),
       *(int *)(lVar22 + uVar28 * 0x14) == (int)(extraout_RAX_00 >> 0x20))) {
      uVar24 = *(uint *)(lVar22 + uVar28 * 0x14 + 4);
      uVar28 = (ulonglong)uVar24;
      if (((uVar28 != 0xffffffff) && (uVar28 < *(ulonglong *)(lVar11 + 0x38))) &&
         ((*(ulonglong *)(*(longlong *)(lVar11 + 0x28) + (ulonglong)(uVar24 >> 6) * 8) >>
           (uVar28 & 0x3f) & 1) != 0)) {
        country_core::resources::roofs::roof_shape::Roof::height();
        fVar34 = extraout_XMM0_Da_00;
      }
    }
    local_1c8 = local_1c8 + 1;
    country_core::resources::glade_border::GladeBorder::is_pos_inside();
    if (((extraout_AL != '\0') && (fVar2 <= *(float *)(lVar29 + 0x14))) &&
       (*(float *)(lVar29 + 0x14) <= extraout_XMM0_Da + fVar34)) {
      ::utils::rng::init_rng();
      auVar17._8_8_ = 0;
      auVar17._0_8_ = extraout_RAX_01 + 0xa0761d6478bd642fU ^ 0xe7037ed1a0b428db;
      auVar19._8_8_ = 0;
      auVar19._0_8_ = extraout_RAX_01 + 0xa0761d6478bd642fU;
      uVar28 = SUB168(auVar17 * auVar19,8) ^ SUB168(auVar17 * auVar19,0);
      if ((uint)((int)uVar28 * 7) < 4) {
        uVar26 = extraout_RAX_01 + 0x40ec3ac8f17ac85e;
        do {
          auVar18._8_8_ = 0;
          auVar18._0_8_ = uVar26 ^ 0xe7037ed1a0b428db;
          auVar20._8_8_ = 0;
          auVar20._0_8_ = uVar26;
          uVar28 = SUB168(auVar18 * auVar20,8) ^ SUB168(auVar18 * auVar20,0);
          uVar26 = uVar26 + 0xa0761d6478bd642f;
        } while ((uint)((int)uVar28 * 7) < 4);
      }
      uVar28 = (uVar28 & 0xffffffff) * 7;
      uVar24 = (int)(uVar28 >> 0x20) * 8;
      iVar6 = *(int *)(&DAT_14304b8c8 + (uVar28 >> 0x20) * 4);
      _Size = *(size_t *)(&DAT_14304b8e8 + uVar24);
      uVar7 = *(undefined4 *)(lVar29 + 0x18);
      uVar15 = *(undefined8 *)(lVar29 + 0x10);
      uVar3 = *(undefined4 *)(uVar23 * 0x38 + local_1e8 + 0x30);
      *puVar14 = uVar5;
      lVar29 = *(longlong *)(lVar13 + 0x30);
      uVar16 = *(undefined8 *)(lVar13 + 0x40);
      if (lVar29 == *(longlong *)(lVar13 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                  ();
      }
      lVar22 = *(longlong *)(lVar13 + 0x28);
      lVar27 = lVar29 * 0x20;
      *(undefined8 *)(lVar22 + lVar27) = uVar16;
      *(undefined8 *)(lVar22 + 8 + lVar27) = uVar15;
      *(undefined4 *)(lVar22 + 0x10 + lVar27) = uVar7;
      *(undefined4 *)(lVar22 + 0x14 + lVar27) = uVar3;
      *(char *)(lVar22 + 0x18 + lVar27) = (char)(0x7060502030100 >> (sbyte)uVar24);
      *(longlong *)(lVar13 + 0x30) = lVar29 + 1;
      *(longlong *)(lVar13 + 0x40) = *(longlong *)(lVar13 + 0x40) + 1;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (_Dst == (void *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      memcpy(_Dst,&DAT_14304b8c8 + iVar6,_Size);
      bevy_ecs::system::commands::Commands::
      spawn<tuple_<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>_>
                ();
    }
  } while( true );
}

