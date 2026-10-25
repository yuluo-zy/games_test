// tod_circle<u64> @ 0x140334130
// sig: void __cdecl tod_circle<u64>(void)


void __cdecl tiny_glade::gui::gui_tod::impl$5::tod_circle<u64>(void)

{
  char *pcVar1;
  longlong *plVar2;
  code *pcVar3;
  uint uVar4;
  uint uVar5;
  longlong extraout_RAX;
  ulonglong extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined *puVar6;
  longlong lVar7;
  longlong lVar8;
  longlong in_RDX;
  ulonglong uVar9;
  ushort uVar10;
  ulonglong uVar11;
  longlong lVar12;
  uint uVar13;
  longlong lVar14;
  byte bVar15;
  char cVar16;
  char cVar18;
  char cVar19;
  char cVar20;
  undefined1 auVar17 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  uint local_78;
  
  minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
  ::utils::calculate_hash<tuple_<u64,core::any::TypeId>_>();
  lVar7 = *(longlong *)(in_RDX + 8);
  ::utils::calculate_hash<tuple_<u64,u64>_>();
  toolbar::mei::core::NodeContext::child_with_global_id();
  if (*(longlong *)(lVar7 + 0x1b8) != 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar8 = *(longlong *)(lVar7 + 0x1a0);
    uVar9 = *(ulonglong *)(lVar7 + 0x1a8);
    bVar15 = (byte)(extraout_RAX_00 >> 0x39);
    auVar17 = ZEXT216(CONCAT11(bVar15,bVar15));
    auVar17 = pshuflw(auVar17,auVar17,0);
    lVar12 = 0;
    uVar11 = extraout_RAX_00;
    while( true ) {
      uVar11 = uVar9 & uVar11;
      pcVar1 = (char *)(lVar8 + uVar11);
      cVar16 = auVar17[0];
      auVar22[0] = -(*pcVar1 == cVar16);
      cVar18 = auVar17[1];
      auVar22[1] = -(pcVar1[1] == cVar18);
      cVar19 = auVar17[2];
      auVar22[2] = -(pcVar1[2] == cVar19);
      cVar20 = auVar17[3];
      auVar22[3] = -(pcVar1[3] == cVar20);
      auVar22[4] = -(pcVar1[4] == cVar16);
      auVar22[5] = -(pcVar1[5] == cVar18);
      auVar22[6] = -(pcVar1[6] == cVar19);
      auVar22[7] = -(pcVar1[7] == cVar20);
      auVar22[8] = -(pcVar1[8] == cVar16);
      auVar22[9] = -(pcVar1[9] == cVar18);
      auVar22[10] = -(pcVar1[10] == cVar19);
      auVar22[0xb] = -(pcVar1[0xb] == cVar20);
      auVar22[0xc] = -(pcVar1[0xc] == cVar16);
      auVar22[0xd] = -(pcVar1[0xd] == cVar18);
      auVar22[0xe] = -(pcVar1[0xe] == cVar19);
      auVar22[0xf] = -(pcVar1[0xf] == cVar20);
      uVar10 = (ushort)(SUB161(auVar22 >> 7,0) & 1) | (ushort)(SUB161(auVar22 >> 0xf,0) & 1) << 1 |
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
      uVar13 = (uint)uVar10;
      while (uVar10 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar13; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        lVar14 = (uVar4 + uVar11 & uVar9) * -0xc0;
        if (extraout_RAX == *(longlong *)(lVar8 + -0xc0 + lVar14)) {
          lVar12 = lVar8 + lVar14;
          if (*(longlong *)(lVar8 + -0x38 + lVar14) == 0) {
            plVar2 = *(longlong **)(lVar7 + 0x260);
            LOCK();
            lVar7 = *plVar2;
            *plVar2 = *plVar2 + 1;
            UNLOCK();
            if (*plVar2 == 0 || SCARRY8(lVar7,1) != *plVar2 < 0) {
                    // WARNING: Does not return
              pcVar3 = (code *)invalidInstructionException();
              (*pcVar3)();
            }
            local_78 = local_78 & 0xffffff00;
            LOCK();
            *plVar2 = *plVar2 + -1;
            UNLOCK();
            if (*plVar2 == 0) {
              alloc::sync::Arc::drop_slow<tg_text::font_table::FontTable,alloc::alloc::Global>();
            }
            uVar9 = 4;
            std::alloc::__default_lib_allocator::__rust_alloc();
            if (extraout_RAX_01 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
              alloc::alloc::handle_alloc_error();
            }
            *(uint *)(extraout_RAX_01 + 1) = local_78;
            *extraout_RAX_01 = 0;
            *(undefined8 **)(lVar12 + -0x38) = extraout_RAX_01;
            puVar6 = &DAT_1429bf380;
            *(undefined **)(lVar12 + -0x30) = &DAT_1429bf380;
          }
          else {
            puVar6 = *(undefined **)(lVar12 + -0x30);
          }
          lVar7 = (**(code **)(puVar6 + 0x28))();
          lVar8 = (**(code **)(uVar9 + 0x18))(lVar7);
          if (((lVar7 != 0) && (lVar8 == -0x1fd4fbc507badf2e)) && (uVar9 == 0x96b64fdf1b70bc63)) {
            toolbar::mei::layout::impl$16::default();
            toolbar::mei::layout::LayoutStyle::expand();
            toolbar::mei::core::NodeContext::layout();
            return;
          }
                    // WARNING: Subroutine does not return
          core::option::expect_failed();
        }
        uVar10 = (ushort)(uVar13 - 1) & (ushort)uVar13;
        uVar13 = CONCAT22((short)(uVar13 - 1 >> 0x10),uVar10);
      }
      auVar21[0] = -(*pcVar1 == -1);
      auVar21[1] = -(pcVar1[1] == -1);
      auVar21[2] = -(pcVar1[2] == -1);
      auVar21[3] = -(pcVar1[3] == -1);
      auVar21[4] = -(pcVar1[4] == -1);
      auVar21[5] = -(pcVar1[5] == -1);
      auVar21[6] = -(pcVar1[6] == -1);
      auVar21[7] = -(pcVar1[7] == -1);
      auVar21[8] = -(pcVar1[8] == -1);
      auVar21[9] = -(pcVar1[9] == -1);
      auVar21[10] = -(pcVar1[10] == -1);
      auVar21[0xb] = -(pcVar1[0xb] == -1);
      auVar21[0xc] = -(pcVar1[0xc] == -1);
      auVar21[0xd] = -(pcVar1[0xd] == -1);
      auVar21[0xe] = -(pcVar1[0xe] == -1);
      auVar21[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar21 >> 7,0) & 1) != 0 || (SUB161(auVar21 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar21 >> 0x17,0) & 1) != 0) || (SUB161(auVar21 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar21 >> 0x27,0) & 1) != 0) || (SUB161(auVar21 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar21 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar21 >> 0x3f,0) & 1) != 0) || (SUB161(auVar21 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar21 >> 0x4f,0) & 1) != 0) || (SUB161(auVar21 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar21 >> 0x5f,0) & 1) != 0) || (SUB161(auVar21 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar21 >> 0x6f,0) & 1) != 0) || (SUB161(auVar21 >> 0x77,0) & 1) != 0) ||
          auVar21[0xf] < '\0') break;
      uVar11 = uVar11 + lVar12 + 0x10;
      lVar12 = lVar12 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::expect_failed();
}

