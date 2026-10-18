
void __cdecl
country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::remove_wall_id(void)

{
  longlong lVar1;
  undefined1 auVar2 [16];
  ushort uVar3;
  int iVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  uint uVar10;
  uint uVar11;
  longlong in_RCX;
  longlong in_RDX;
  longlong lVar12;
  longlong *plVar13;
  longlong lVar14;
  ulonglong uVar15;
  undefined8 *puVar16;
  longlong *plVar17;
  undefined1 (*pauVar18) [16];
  longlong lVar19;
  longlong lVar20;
  longlong lVar21;
  undefined1 (*local_c0) [16];
  longlong local_b8;
  longlong local_a8;
  longlong local_58;
  
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  hashbrown::raw::RawTable::
  remove_entry<tuple_<country_core::resources::walls::WallId,std::collections::hash::set::HashSet<usize,std::hash::random::RandomState>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,std::collections::hash::set::HashSet<usize,std::hash::random::RandomState>_>_>
            ();
  if (local_c0 != (undefined1 (*) [16])0x0) {
    if (local_b8 != 0) {
      local_58 = local_b8 + (local_b8 * 8 + 0x17U & 0xfffffffffffffff0) + 0x11;
    }
    if (local_a8 != 0) {
      auVar2 = *local_c0;
      uVar10 = ~(uint)(ushort)((ushort)(SUB161(auVar2 >> 7,0) & 1) |
                               (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                               (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                               (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                               (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                               (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                               (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                               (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                               (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                               (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                               (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                               (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                               (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                               (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                               (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                              (ushort)(byte)(auVar2[0xf] >> 7) << 0xf);
      pauVar18 = local_c0 + 1;
      do {
        if ((short)uVar10 == 0) {
          do {
            auVar2 = *pauVar18;
            uVar3 = (ushort)(SUB161(auVar2 >> 7,0) & 1) | (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1
                    | (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                    (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                    (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                    (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                    (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                    (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                    (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                    (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                    (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                    (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                    (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                    (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                    (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                    (ushort)(byte)(auVar2[0xf] >> 7) << 0xf;
            local_c0 = local_c0 + -8;
            pauVar18 = pauVar18 + 1;
          } while (uVar3 == 0xffff);
          uVar11 = ~(uint)uVar3;
          uVar10 = uVar11 & -(uint)uVar3 - 2;
        }
        else {
          uVar11 = uVar10;
          uVar10 = uVar10 - 1 & uVar10;
        }
        iVar4 = 0;
        for (; (uVar11 & 1) == 0; uVar11 = uVar11 >> 1 | 0x80000000) {
          iVar4 = iVar4 + 1;
        }
        uVar15 = *(ulonglong *)((longlong)local_c0 + (-8 - (ulonglong)(uint)(iVar4 << 3)));
        if (*(ulonglong *)(in_RCX + 0x10) <= uVar15) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        lVar19 = uVar15 * 0x30;
        lVar1 = *(longlong *)(in_RCX + 8) + lVar19;
        lVar19 = *(longlong *)(*(longlong *)(in_RCX + 8) + 0x10 + lVar19);
        if (lVar19 != 0) {
          lVar21 = *(longlong *)(lVar1 + 8);
          lVar12 = 1;
          lVar20 = 1;
          if (in_RDX != *(longlong *)(lVar21 + 0x28)) {
            plVar13 = (longlong *)(lVar21 + 0x60);
            do {
              if (lVar19 == lVar20) {
                lVar12 = 0;
                goto LAB_140aa555a;
              }
              lVar20 = lVar20 + 1;
              lVar14 = *plVar13;
              plVar13 = plVar13 + 7;
            } while (in_RDX != lVar14);
          }
          lVar14 = lVar19 - lVar20;
          if (lVar14 != 0) {
            puVar16 = (undefined8 *)(lVar21 + lVar20 * 0x38);
            lVar12 = 1;
            do {
              while (in_RDX == puVar16[5]) {
                lVar12 = lVar12 + 1;
                puVar16 = puVar16 + 7;
                lVar14 = lVar14 + -1;
                if (lVar14 == 0) goto LAB_140aa555a;
              }
              puVar16[lVar12 * -7 + 6] = puVar16[6];
              uVar5 = *puVar16;
              uVar6 = puVar16[1];
              uVar7 = puVar16[2];
              uVar8 = puVar16[3];
              uVar9 = puVar16[5];
              puVar16[lVar12 * -7 + 4] = puVar16[4];
              (puVar16 + lVar12 * -7 + 4)[1] = uVar9;
              puVar16[lVar12 * -7 + 2] = uVar7;
              (puVar16 + lVar12 * -7 + 2)[1] = uVar8;
              puVar16[lVar12 * -7] = uVar5;
              (puVar16 + lVar12 * -7)[1] = uVar6;
              puVar16 = puVar16 + 7;
              lVar14 = lVar14 + -1;
            } while (lVar14 != 0);
          }
LAB_140aa555a:
          *(longlong *)(lVar1 + 0x10) = lVar19 - lVar12;
        }
        lVar19 = *(longlong *)(lVar1 + 0x28);
        if (lVar19 != 0) {
          plVar13 = *(longlong **)(lVar1 + 0x20);
          lVar12 = 1;
          lVar21 = 1;
          if (in_RDX != *plVar13) {
            lVar20 = 1;
            do {
              if (lVar19 == lVar20) {
                lVar12 = 0;
                goto LAB_140aa5429;
              }
              lVar21 = lVar20 + 1;
              plVar17 = plVar13 + lVar20;
              lVar20 = lVar21;
            } while (in_RDX != *plVar17);
          }
          uVar15 = lVar19 - lVar21;
          if (uVar15 != 0) {
            lVar12 = 1;
            if (lVar19 != lVar21 + 1) {
              plVar17 = plVar13 + lVar21 + 1;
              lVar20 = 0;
              do {
                while (lVar14 = lVar20, in_RDX != plVar17[-1]) {
                  plVar17[-1 - lVar12] = plVar17[-1];
                  lVar20 = *plVar17;
                  if (in_RDX != lVar20) goto LAB_140aa563b;
LAB_140aa55f0:
                  lVar12 = lVar12 + 1;
                  plVar17 = plVar17 + 2;
                  lVar20 = lVar14 + -2;
                  if (-(lVar14 + -2) == (uVar15 & 0xfffffffffffffffe)) goto LAB_140aa565a;
                }
                lVar12 = lVar12 + 1;
                lVar20 = *plVar17;
                if (in_RDX == lVar20) goto LAB_140aa55f0;
LAB_140aa563b:
                plVar17[-lVar12] = lVar20;
                plVar17 = plVar17 + 2;
                lVar20 = lVar14 + -2;
              } while (-(lVar14 + -2) != (uVar15 & 0xfffffffffffffffe));
LAB_140aa565a:
              lVar21 = lVar21 - (lVar14 + -2);
            }
            if ((uVar15 & 1) != 0) {
              if (in_RDX == plVar13[lVar21]) {
                lVar12 = lVar12 + 1;
              }
              else {
                plVar13[lVar21 - lVar12] = plVar13[lVar21];
              }
            }
          }
LAB_140aa5429:
          *(longlong *)(lVar1 + 0x28) = lVar19 - lVar12;
        }
        local_a8 = local_a8 + -1;
      } while (local_a8 != 0);
    }
    if ((local_b8 != 0) && (local_58 != 0)) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  return;
}

