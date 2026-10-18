/* despawn_trees @0x0x1422686d0 size 736 */

void __cdecl system_tree::process_tree_cmds::despawn_trees(void)

{
  uint *puVar1;
  float *pfVar2;
  undefined4 uVar3;
  uint uVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  undefined4 *puVar8;
  ulonglong uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  longlong lVar12;
  ulonglong uVar13;
  longlong lVar14;
  longlong *in_RCX;
  longlong *in_RDX;
  ulonglong uVar15;
  longlong lVar16;
  longlong lVar17;
  longlong lVar18;
  longlong lVar19;
  uint *puVar20;
  
  lVar5 = *in_RCX;
  lVar6 = in_RCX[1];
  puVar20 = *(uint **)(lVar5 + 0xf8);
  puVar1 = puVar20 + *(longlong *)(lVar5 + 0x100);
  uVar3 = *(undefined4 *)((longlong)in_RDX + 0x1c);
  lVar7 = *in_RDX;
  puVar8 = (undefined4 *)in_RDX[2];
  if (*(char *)(lVar5 + 0x108) == '\0') {
    lVar19 = 8;
    lVar18 = 0;
    lVar17 = 0;
    lVar16 = 0;
    do {
      if (lVar16 == lVar17) {
        do {
          if (puVar20 == puVar1) {
            return;
          }
          uVar4 = *puVar20;
          puVar20 = puVar20 + 1;
          lVar17 = *(longlong *)(*(longlong *)(lVar6 + 0x100) + 0x58 + (ulonglong)uVar4 * 0xa0);
        } while (lVar17 == 0);
        lVar19 = *(longlong *)(lVar6 + 0x100) + (ulonglong)uVar4 * 0xa0;
        uVar15 = (ulonglong)*(uint *)(lVar19 + 0x94);
        lVar18 = *(longlong *)(lVar6 + 0x1a8);
        if (*(ulonglong *)(lVar5 + 0x110) < *(ulonglong *)(lVar18 + 0x40 + uVar15 * 0x48)) {
          uVar9 = *(ulonglong *)
                   (*(longlong *)(lVar18 + 0x38 + uVar15 * 0x48) + *(ulonglong *)(lVar5 + 0x110) * 8
                   );
          if (uVar9 == 0) {
            lVar18 = 0;
          }
          else {
            lVar18 = *(longlong *)
                      (*(longlong *)(lVar18 + uVar15 * 0x48 + 0x18) + 0x10 + ~uVar9 * 0x30);
          }
        }
        else {
          lVar18 = 0;
        }
        lVar19 = *(longlong *)(lVar19 + 0x50);
        lVar16 = 0;
      }
      lVar14 = lVar16 * 0x10;
      lVar16 = lVar16 + 1;
      pfVar2 = (float *)(lVar18 + 4 + (ulonglong)*(uint *)(lVar19 + 8 + lVar14) * 0x10);
      if (*pfVar2 <= 0.01 && *pfVar2 != 0.01) {
        uVar10 = *(undefined8 *)(lVar19 + lVar14);
        *puVar8 = uVar3;
        lVar14 = *(longlong *)(lVar7 + 0x30);
        uVar11 = *(undefined8 *)(lVar7 + 0x40);
        if (lVar14 == *(longlong *)(lVar7 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        lVar12 = *(longlong *)(lVar7 + 0x28);
        *(undefined8 *)(lVar12 + lVar14 * 0x10) = uVar11;
        *(undefined8 *)(lVar12 + 8 + lVar14 * 0x10) = uVar10;
        *(longlong *)(lVar7 + 0x30) = lVar14 + 1;
        *(longlong *)(lVar7 + 0x40) = *(longlong *)(lVar7 + 0x40) + 1;
      }
    } while( true );
  }
  lVar18 = 8;
  lVar17 = 0;
  uVar15 = 0;
  uVar9 = 0;
  do {
    uVar13 = uVar9;
    if (uVar13 == uVar15) {
      do {
        if (puVar20 == puVar1) {
          return;
        }
        uVar4 = *puVar20;
        puVar20 = puVar20 + 1;
        uVar15 = *(ulonglong *)(*(longlong *)(lVar6 + 0x1a8) + 0x10 + (ulonglong)uVar4 * 0x48);
      } while (uVar15 == 0);
      lVar18 = *(longlong *)(lVar6 + 0x1a8) + (ulonglong)uVar4 * 0x48;
      if ((*(ulonglong *)(lVar5 + 0x110) < *(ulonglong *)(lVar18 + 0x40)) &&
         (uVar9 = *(ulonglong *)(*(longlong *)(lVar18 + 0x38) + *(ulonglong *)(lVar5 + 0x110) * 8),
         uVar9 != 0)) {
        lVar17 = *(longlong *)(*(longlong *)(lVar18 + 0x18) + 0x10 + ~uVar9 * 0x30);
      }
      else {
        lVar17 = 0;
      }
      lVar18 = *(longlong *)(lVar18 + 8);
      uVar13 = 0;
    }
    pfVar2 = (float *)(lVar17 + 4 + (uVar13 & 0xffffffff) * 0x10);
    uVar9 = uVar13 + 1;
    if (*pfVar2 <= 0.01 && *pfVar2 != 0.01) {
      uVar10 = *(undefined8 *)(lVar18 + uVar13 * 8);
      *puVar8 = uVar3;
      lVar19 = *(longlong *)(lVar7 + 0x30);
      uVar11 = *(undefined8 *)(lVar7 + 0x40);
      if (lVar19 == *(longlong *)(lVar7 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar16 = *(longlong *)(lVar7 + 0x28);
      *(undefined8 *)(lVar16 + lVar19 * 0x10) = uVar11;
      *(undefined8 *)(lVar16 + 8 + lVar19 * 0x10) = uVar10;
      *(longlong *)(lVar7 + 0x30) = lVar19 + 1;
      *(longlong *)(lVar7 + 0x40) = *(longlong *)(lVar7 + 0x40) + 1;
    }
  } while( true );
}

