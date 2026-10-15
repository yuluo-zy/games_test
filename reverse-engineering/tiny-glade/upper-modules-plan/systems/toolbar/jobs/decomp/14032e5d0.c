// 0x14032e5d0 pill_button<ref$<str$>_>
// signature: void __cdecl pill_button<ref$<str$>_>(void)

void __cdecl toolbar::mei::widgets::impl$52::pill_button<ref_<str_>_>(void)

{
  undefined1 auVar1 [16];
  undefined8 uVar2;
  longlong *plVar3;
  code *pcVar4;
  uint uVar5;
  uint uVar6;
  undefined8 extraout_RAX;
  ulonglong extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined **ppuVar7;
  longlong lVar8;
  longlong in_RDX;
  ulonglong uVar9;
  longlong in_R8;
  ulonglong uVar10;
  longlong in_R9;
  longlong lVar11;
  uint uVar12;
  ushort uVar13;
  longlong lVar14;
  byte bVar15;
  char cVar16;
  char cVar18;
  char cVar19;
  char cVar20;
  undefined1 auVar17 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  undefined8 in_stack_00000028;
  undefined8 in_stack_00000030;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined8 local_58;
  
  core::impl$0::as_id();
  ::utils::calculate_hash<tuple_<u64,core::any::TypeId>_>();
  uVar2 = *(undefined8 *)(in_RDX + 8);
  ::utils::calculate_hash<tuple_<u64,u64>_>();
  core::NodeContext::child_with_global_id();
  if (*(longlong *)(in_R8 + 0x1b8) != 0) {
    ::core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar8 = *(longlong *)(in_R8 + 0x1a0);
    uVar9 = *(ulonglong *)(in_R8 + 0x1a8);
    bVar15 = (byte)(extraout_RAX_00 >> 0x39);
    auVar17 = ZEXT216(CONCAT11(bVar15,bVar15));
    auVar17 = pshuflw(auVar17,auVar17,0);
    lVar11 = 0;
    uVar10 = extraout_RAX_00;
    while( true ) {
      uVar10 = uVar9 & uVar10;
      auVar1 = *(undefined1 (*) [16])(lVar8 + uVar10);
      cVar16 = auVar17[0];
      auVar22[0] = -(auVar1[0] == cVar16);
      cVar18 = auVar17[1];
      auVar22[1] = -(auVar1[1] == cVar18);
      cVar19 = auVar17[2];
      auVar22[2] = -(auVar1[2] == cVar19);
      cVar20 = auVar17[3];
      auVar22[3] = -(auVar1[3] == cVar20);
      auVar22[4] = -(auVar1[4] == cVar16);
      auVar22[5] = -(auVar1[5] == cVar18);
      auVar22[6] = -(auVar1[6] == cVar19);
      auVar22[7] = -(auVar1[7] == cVar20);
      auVar22[8] = -(auVar1[8] == cVar16);
      auVar22[9] = -(auVar1[9] == cVar18);
      auVar22[10] = -(auVar1[10] == cVar19);
      auVar22[0xb] = -(auVar1[0xb] == cVar20);
      auVar22[0xc] = -(auVar1[0xc] == cVar16);
      auVar22[0xd] = -(auVar1[0xd] == cVar18);
      auVar22[0xe] = -(auVar1[0xe] == cVar19);
      auVar22[0xf] = -(auVar1[0xf] == cVar20);
      uVar13 = (ushort)(SUB161(auVar22 >> 7,0) & 1) | (ushort)(SUB161(auVar22 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar22 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar22 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar22 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar22 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar22 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar22 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar22 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar22 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar22 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar22 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar22 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar22 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar22 >> 0x77,0) & 1) << 0xe | (ushort)(auVar22[0xf] >> 7) << 0xf;
      uVar12 = (uint)uVar13;
      while (uVar13 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar12; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        lVar14 = (uVar5 + uVar10 & uVar9) * -0xc0;
        if (in_R9 == *(longlong *)(lVar8 + -0xc0 + lVar14)) {
          lVar11 = lVar8 + lVar14;
          if (*(longlong *)(lVar8 + -0x38 + lVar14) == 0) {
            plVar3 = *(longlong **)(in_R8 + 0x260);
            LOCK();
            lVar8 = *plVar3;
            *plVar3 = *plVar3 + 1;
            UNLOCK();
            if (*plVar3 == 0 || SCARRY8(lVar8,1) != *plVar3 < 0) {
                    // WARNING: Does not return
              pcVar4 = (code *)invalidInstructionException();
              (*pcVar4)();
            }
            widgets::impl$24::create_node_content();
            uVar9 = 8;
            std::alloc::__default_lib_allocator::__rust_alloc();
            if (extraout_RAX_01 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
              alloc::alloc::handle_alloc_error();
            }
            extraout_RAX_01[8] = local_58;
            extraout_RAX_01[6] = local_68;
            extraout_RAX_01[7] = uStack_60;
            extraout_RAX_01[4] = local_78;
            extraout_RAX_01[5] = uStack_70;
            extraout_RAX_01[2] = 0xa6ddf7d2d741de4d;
            extraout_RAX_01[3] = uStack_80;
            *extraout_RAX_01 = uVar2;
            extraout_RAX_01[1] = extraout_RAX;
            *(undefined8 **)(lVar11 + -0x38) = extraout_RAX_01;
            ppuVar7 = &PTR_drop_in_place<egui::grid::State>_1429bf628;
            *(undefined ***)(lVar11 + -0x30) = &PTR_drop_in_place<egui::grid::State>_1429bf628;
          }
          else {
            ppuVar7 = *(undefined ***)(lVar11 + -0x30);
          }
          lVar8 = (*(code *)ppuVar7[5])();
          lVar11 = (**(code **)(uVar9 + 0x18))(lVar8);
          if (((lVar8 != 0) && (lVar11 == -0x7931e0e99eebebe0)) && (uVar9 == 0xa6ddf7d2d741de4d)) {
            *(undefined8 *)(lVar8 + 0x30) = in_stack_00000028;
            *(undefined8 *)(lVar8 + 0x38) = in_stack_00000030;
            core::NodeContext::with_nav();
            layout::impl$16::default();
            layout::LayoutStyle::align_center();
            layout::LayoutStyle::xalign_center();
            layout::LayoutStyle::size();
            core::NodeContext::layout();
            audio::impl$0::default();
            core::NodeContext::sounds();
            return;
          }
                    // WARNING: Subroutine does not return
          ::core::option::expect_failed();
        }
        uVar13 = (ushort)(uVar12 - 1) & (ushort)uVar12;
        uVar12 = CONCAT22((short)(uVar12 - 1 >> 0x10),uVar13);
      }
      auVar21[0] = -(auVar1[0] == -1);
      auVar21[1] = -(auVar1[1] == -1);
      auVar21[2] = -(auVar1[2] == -1);
      auVar21[3] = -(auVar1[3] == -1);
      auVar21[4] = -(auVar1[4] == -1);
      auVar21[5] = -(auVar1[5] == -1);
      auVar21[6] = -(auVar1[6] == -1);
      auVar21[7] = -(auVar1[7] == -1);
      auVar21[8] = -(auVar1[8] == -1);
      auVar21[9] = -(auVar1[9] == -1);
      auVar21[10] = -(auVar1[10] == -1);
      auVar21[0xb] = -(auVar1[0xb] == -1);
      auVar21[0xc] = -(auVar1[0xc] == -1);
      auVar21[0xd] = -(auVar1[0xd] == -1);
      auVar21[0xe] = -(auVar1[0xe] == -1);
      auVar21[0xf] = -(auVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar21 >> 7,0) & 1) != 0 || (SUB161(auVar21 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar21 >> 0x17,0) & 1) != 0) || (SUB161(auVar21 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar21 >> 0x27,0) & 1) != 0) || (SUB161(auVar21 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar21 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar21 >> 0x3f,0) & 1) != 0) || (SUB161(auVar21 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar21 >> 0x4f,0) & 1) != 0) || (SUB161(auVar21 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar21 >> 0x5f,0) & 1) != 0) || (SUB161(auVar21 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar21 >> 0x6f,0) & 1) != 0) || (SUB161(auVar21 >> 0x77,0) & 1) != 0) ||
          auVar21[0xf] < '\0') break;
      uVar10 = uVar10 + lVar11 + 0x10;
      lVar11 = lVar11 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  ::core::option::expect_failed();
}

