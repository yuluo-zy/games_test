
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
  float fVar21;
  char extraout_AL;
  longlong lVar22;
  longlong extraout_RAX;
  longlong lVar23;
  ulonglong uVar24;
  ulonglong extraout_RAX_00;
  longlong extraout_RAX_01;
  void *_Dst;
  uint uVar25;
  undefined8 *in_RCX;
  longlong lVar26;
  ulonglong uVar27;
  longlong lVar28;
  longlong *in_RDX;
  ulonglong uVar29;
  longlong lVar30;
  ulonglong uVar31;
  longlong lVar32;
  longlong lVar33;
  uint *puVar34;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar35;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  longlong local_1e8;
  longlong local_1e0;
  ulonglong local_1c8;
  longlong local_190;
  
  puVar8 = (ulonglong *)*in_RCX;
  lVar9 = in_RCX[1];
  uVar31 = *puVar8;
  uVar29 = 0;
  uVar24 = uVar31 - *(ulonglong *)(lVar9 + 0x18);
  if (uVar31 < *(ulonglong *)(lVar9 + 0x18)) {
    uVar24 = uVar29;
  }
  uVar27 = uVar31 - *(ulonglong *)(lVar9 + 0x38);
  if (uVar31 < *(ulonglong *)(lVar9 + 0x38)) {
    uVar27 = uVar29;
  }
  lVar26 = uVar24 * 0x10 + *(longlong *)(lVar9 + 8);
  uVar31 = *(ulonglong *)(lVar9 + 0x10) - uVar24;
  if (*(ulonglong *)(lVar9 + 0x10) < uVar24) {
    lVar26 = 8;
    uVar31 = uVar29;
  }
  lVar22 = uVar27 * 0x10 + *(longlong *)(lVar9 + 0x28);
  uVar24 = *(ulonglong *)(lVar9 + 0x30) - uVar27;
  if (*(ulonglong *)(lVar9 + 0x30) < uVar27) {
    lVar22 = 8;
    uVar24 = uVar29;
  }
  *puVar8 = (*(longlong *)(lVar9 + 0x40) - uVar24) - uVar31;
  fVar21 = __real_3fc00000;
  lVar32 = uVar31 * 0x10 + lVar26;
  lVar33 = uVar24 * 0x10 + lVar22;
  lVar9 = *in_RDX;
  lVar10 = in_RDX[1];
  lVar11 = *in_stack_00000028;
  lVar12 = in_stack_00000028[1];
  uVar5 = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
  lVar13 = *in_stack_00000030;
  puVar14 = (undefined4 *)in_stack_00000030[2];
LAB_14231f7a7:
  do {
    if ((lVar26 == 0) || (lVar26 == lVar32)) {
      if ((lVar22 == 0) || (lVar22 == lVar33)) {
        return;
      }
      lVar22 = lVar22 + 0x10;
      lVar26 = 0;
    }
    else {
      lVar26 = lVar26 + 0x10;
    }
    *puVar8 = *puVar8 + 1;
    country_core::resources::walls::public_walls::PublicWalls::get();
    if (extraout_RAX == 0) {
                    /* WARNING: Subroutine does not return */
      core::option::unwrap_failed();
    }
  } while ((1 < *(uint *)(extraout_RAX + 600)) || (fVar21 < *(float *)(extraout_RAX + 0x280)));
  country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape();
  puVar34 = *(uint **)(lVar9 + 0xf8);
  puVar1 = puVar34 + *(longlong *)(lVar9 + 0x100);
  cVar4 = *(char *)(lVar9 + 0x108);
  local_190 = 8;
  local_1e0 = 0;
  local_1e8 = 0;
  uVar31 = 0;
  local_1c8 = 0;
  do {
    if (cVar4 == '\0') {
      if (local_1c8 == uVar31) {
        do {
          if (puVar34 == puVar1) goto LAB_14231f7a7;
          uVar25 = *puVar34;
          puVar34 = puVar34 + 1;
          uVar31 = *(ulonglong *)(*(longlong *)(lVar10 + 0x100) + 0x58 + (ulonglong)uVar25 * 0xa0);
        } while (uVar31 == 0);
        lVar23 = *(longlong *)(lVar10 + 0x100) + (ulonglong)uVar25 * 0xa0;
        uVar24 = (ulonglong)*(uint *)(lVar23 + 0x94);
        lVar30 = *(longlong *)(lVar10 + 0x1a8) + uVar24 * 0x48;
        uVar24 = *(ulonglong *)(*(longlong *)(lVar10 + 0x1a8) + 0x40 + uVar24 * 0x48);
        if ((*(ulonglong *)(lVar9 + 0x110) < uVar24) &&
           (*(longlong *)(*(longlong *)(lVar30 + 0x38) + *(ulonglong *)(lVar9 + 0x110) * 8) != 0)) {
          uVar29 = *(ulonglong *)(lVar9 + 0x118);
          if (uVar24 <= uVar29) goto LAB_14231fb13;
LAB_14231fa94:
          uVar29 = *(ulonglong *)(*(longlong *)(lVar30 + 0x38) + uVar29 * 8);
          if (uVar29 == 0) goto LAB_14231fb13;
          local_1e0 = *(longlong *)(*(longlong *)(lVar30 + 0x18) + 0x10 + ~uVar29 * 0x30);
          uVar29 = *(ulonglong *)(lVar9 + 0x120);
          if (uVar29 < uVar24) goto LAB_14231fb2d;
LAB_14231fb69:
          local_1e8 = 0;
        }
        else {
          uVar29 = *(ulonglong *)(lVar9 + 0x118);
          if (uVar29 < uVar24) goto LAB_14231fa94;
LAB_14231fb13:
          local_1e0 = 0;
          uVar29 = *(ulonglong *)(lVar9 + 0x120);
          if (uVar24 <= uVar29) goto LAB_14231fb69;
LAB_14231fb2d:
          uVar24 = *(ulonglong *)(*(longlong *)(lVar30 + 0x38) + uVar29 * 8);
          if (uVar24 == 0) goto LAB_14231fb69;
          local_1e8 = *(longlong *)(*(longlong *)(lVar30 + 0x18) + 0x10 + ~uVar24 * 0x30);
        }
        local_190 = *(longlong *)(lVar23 + 0x50);
        local_1c8 = 0;
      }
      uVar24 = (ulonglong)*(uint *)(local_190 + 8 + local_1c8 * 0x10);
    }
    else {
      if (local_1c8 == uVar31) {
        do {
          if (puVar34 == puVar1) goto LAB_14231f7a7;
          uVar25 = *puVar34;
          puVar34 = puVar34 + 1;
          uVar31 = *(ulonglong *)(*(longlong *)(lVar10 + 0x1a8) + 0x10 + (ulonglong)uVar25 * 0x48);
        } while (uVar31 == 0);
        lVar30 = *(longlong *)(lVar10 + 0x1a8) + (ulonglong)uVar25 * 0x48;
        uVar24 = *(ulonglong *)(lVar30 + 0x40);
        if ((*(ulonglong *)(lVar9 + 0x110) < uVar24) &&
           (*(longlong *)(*(longlong *)(lVar30 + 0x38) + *(ulonglong *)(lVar9 + 0x110) * 8) != 0)) {
          uVar29 = *(ulonglong *)(lVar9 + 0x118);
          if (uVar24 <= uVar29) goto LAB_14231fad1;
LAB_14231fa39:
          uVar29 = *(ulonglong *)(*(longlong *)(lVar30 + 0x38) + uVar29 * 8);
          if (uVar29 == 0) goto LAB_14231fad1;
          local_1e0 = *(longlong *)(*(longlong *)(lVar30 + 0x18) + 0x10 + ~uVar29 * 0x30);
          uVar29 = *(ulonglong *)(lVar9 + 0x120);
          if (uVar29 < uVar24) goto LAB_14231faeb;
LAB_14231fb55:
          local_1e8 = 0;
        }
        else {
          uVar29 = *(ulonglong *)(lVar9 + 0x118);
          if (uVar29 < uVar24) goto LAB_14231fa39;
LAB_14231fad1:
          local_1e0 = 0;
          uVar29 = *(ulonglong *)(lVar9 + 0x120);
          if (uVar24 <= uVar29) goto LAB_14231fb55;
LAB_14231faeb:
          uVar24 = *(ulonglong *)(*(longlong *)(lVar30 + 0x38) + uVar29 * 8);
          if (uVar24 == 0) goto LAB_14231fb55;
          local_1e8 = *(longlong *)(*(longlong *)(lVar30 + 0x18) + 0x10 + ~uVar24 * 0x30);
        }
        local_1c8 = 0;
      }
      uVar24 = local_1c8 & 0xffffffff;
    }
    lVar30 = uVar24 * 0x30 + local_1e0;
    fVar2 = *(float *)(extraout_RAX + 0x280);
    country_core::resources::walls::public_wall_state::PublicWallState::max_y();
    country_core::resources::walls::public_walls::PublicWalls::roof_entity();
    fVar35 = 0.0;
    if (((extraout_RAX_00 >> 0x20 != 0) &&
        (uVar29 = extraout_RAX_00 & 0xffffffff, uVar29 < *(ulonglong *)(lVar12 + 0x10))) &&
       (lVar23 = *(longlong *)(lVar12 + 8),
       *(int *)(lVar23 + uVar29 * 0x14) == (int)(extraout_RAX_00 >> 0x20))) {
      uVar25 = *(uint *)(lVar23 + uVar29 * 0x14 + 4);
      uVar29 = (ulonglong)uVar25;
      if (((uVar29 != 0xffffffff) && (uVar29 < *(ulonglong *)(lVar11 + 0x38))) &&
         ((*(ulonglong *)(*(longlong *)(lVar11 + 0x28) + (ulonglong)(uVar25 >> 6) * 8) >>
           (uVar29 & 0x3f) & 1) != 0)) {
        country_core::resources::roofs::roof_shape::Roof::height();
        fVar35 = extraout_XMM0_Da_00;
      }
    }
    local_1c8 = local_1c8 + 1;
    country_core::resources::glade_border::GladeBorder::is_pos_inside();
    if (((extraout_AL != '\0') && (fVar2 <= *(float *)(lVar30 + 0x14))) &&
       (*(float *)(lVar30 + 0x14) <= extraout_XMM0_Da + fVar35)) {
      ::utils::rng::init_rng();
      auVar17._8_8_ = 0;
      auVar17._0_8_ = extraout_RAX_01 + 0xa0761d6478bd642fU ^ 0xe7037ed1a0b428db;
      auVar19._8_8_ = 0;
      auVar19._0_8_ = extraout_RAX_01 + 0xa0761d6478bd642fU;
      uVar29 = SUB168(auVar17 * auVar19,8) ^ SUB168(auVar17 * auVar19,0);
      if ((uint)((int)uVar29 * 7) < 4) {
        uVar27 = extraout_RAX_01 + 0x40ec3ac8f17ac85e;
        do {
          auVar18._8_8_ = 0;
          auVar18._0_8_ = uVar27 ^ 0xe7037ed1a0b428db;
          auVar20._8_8_ = 0;
          auVar20._0_8_ = uVar27;
          uVar29 = SUB168(auVar18 * auVar20,8) ^ SUB168(auVar18 * auVar20,0);
          uVar27 = uVar27 + 0xa0761d6478bd642f;
        } while ((uint)((int)uVar29 * 7) < 4);
      }
      uVar29 = (uVar29 & 0xffffffff) * 7;
      uVar25 = (int)(uVar29 >> 0x20) * 8;
      iVar6 = *(int *)(&DAT_14304b8c8 + (uVar29 >> 0x20) * 4);
      _Size = *(size_t *)(&DAT_14304b8e8 + uVar25);
      uVar7 = *(undefined4 *)(lVar30 + 0x18);
      uVar15 = *(undefined8 *)(lVar30 + 0x10);
      uVar3 = *(undefined4 *)(uVar24 * 0x38 + local_1e8 + 0x30);
      *puVar14 = uVar5;
      lVar30 = *(longlong *)(lVar13 + 0x30);
      uVar16 = *(undefined8 *)(lVar13 + 0x40);
      if (lVar30 == *(longlong *)(lVar13 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                  ();
      }
      lVar23 = *(longlong *)(lVar13 + 0x28);
      lVar28 = lVar30 * 0x20;
      *(undefined8 *)(lVar23 + lVar28) = uVar16;
      *(undefined8 *)(lVar23 + 8 + lVar28) = uVar15;
      *(undefined4 *)(lVar23 + 0x10 + lVar28) = uVar7;
      *(undefined4 *)(lVar23 + 0x14 + lVar28) = uVar3;
      *(char *)(lVar23 + 0x18 + lVar28) = (char)(0x7060502030100 >> (sbyte)uVar25);
      *(longlong *)(lVar13 + 0x30) = lVar30 + 1;
      *(longlong *)(lVar13 + 0x40) = *(longlong *)(lVar13 + 0x40) + 1;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (_Dst == (void *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error();
      }
      memcpy(_Dst,&DAT_14304b8c8 + iVar6,_Size);
      bevy_ecs::system::commands::Commands::
      spawn<tuple_<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,system_critters::sheep::sheep_react_to_being_inside_buildings::SheepReactionEmoji>_>
                ();
    }
  } while( true );
}

