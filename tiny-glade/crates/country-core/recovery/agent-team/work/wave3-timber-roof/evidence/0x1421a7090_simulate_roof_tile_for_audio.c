// 0x1421a7090 simulate_roof_tile_for_audio
// sig: void __cdecl simulate_roof_tile_for_audio(void)
// size: 1651


void __cdecl system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio(void)

{
  uint *puVar1;
  float *pfVar2;
  undefined4 *puVar3;
  undefined8 *puVar4;
  char cVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  longlong lVar8;
  longlong lVar9;
  longlong lVar10;
  longlong lVar11;
  undefined4 *puVar12;
  undefined8 uVar13;
  char extraout_AL;
  undefined1 uVar14;
  longlong lVar15;
  ulonglong uVar16;
  longlong *in_RCX;
  longlong *in_RDX;
  longlong lVar17;
  uint *puVar18;
  ulonglong uVar19;
  longlong *in_R8;
  longlong lVar20;
  ulonglong uVar21;
  uint uVar22;
  uint uVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  longlong local_158;
  longlong local_138;
  undefined4 local_12c;
  float fStack_128;
  undefined4 local_124;
  longlong local_110;
  undefined8 local_104;
  undefined3 uStack_fc;
  undefined4 uStack_f9;
  undefined1 uStack_f5;
  
  lVar8 = *in_R8;
  if ((*(float *)(lVar8 + 0x28) == 0.0) && (!NAN(*(float *)(lVar8 + 0x28)))) {
    return;
  }
  lVar9 = *in_RCX;
  lVar10 = in_RCX[1];
  uVar6 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar18 = *(uint **)(lVar9 + 0xf8);
  puVar1 = puVar18 + *(longlong *)(lVar9 + 0x100);
  cVar5 = *(char *)(lVar9 + 0x108);
  uVar7 = *(undefined4 *)((longlong)in_RDX + 0x1c);
  lVar11 = *in_RDX;
  puVar12 = (undefined4 *)in_RDX[2];
  local_110 = 8;
  local_158 = 0;
  uVar19 = 0;
  uVar21 = 0;
  do {
    do {
      if (cVar5 == '\0') {
        if (uVar21 == uVar19) {
          if (puVar18 == puVar1) {
            return;
          }
          while( true ) {
            uVar19 = *(ulonglong *)
                      (*(longlong *)(lVar10 + 0x100) + 0x58 + (ulonglong)*puVar18 * 0xa0);
            if (uVar19 != 0) break;
            puVar18 = puVar18 + 1;
            if (puVar18 == puVar1) {
              return;
            }
          }
          lVar15 = *(longlong *)(lVar10 + 0x100) + (ulonglong)*puVar18 * 0xa0;
          uVar21 = (ulonglong)*(uint *)(lVar15 + 0x94);
          lVar20 = *(longlong *)(*(longlong *)(lVar10 + 0x1a8) + 0x18 + uVar21 * 0x48);
          lVar17 = ~*(ulonglong *)
                     (*(longlong *)(*(longlong *)(lVar10 + 0x1a8) + 0x38 + uVar21 * 0x48) +
                     *(longlong *)(lVar9 + 0x110) * 8) * 0x30;
          local_158 = *(longlong *)(lVar20 + 0x10 + lVar17);
          local_138 = *(longlong *)(lVar20 + 0x28 + lVar17);
          puVar18 = puVar18 + 1;
          local_110 = *(longlong *)(lVar15 + 0x50);
          uVar21 = 0;
        }
        uVar16 = (ulonglong)*(uint *)(local_110 + 8 + uVar21 * 0x10);
      }
      else {
        if (uVar21 == uVar19) {
          if (puVar18 == puVar1) {
            return;
          }
          while( true ) {
            uVar19 = *(ulonglong *)
                      (*(longlong *)(lVar10 + 0x1a8) + 0x10 + (ulonglong)*puVar18 * 0x48);
            if (uVar19 != 0) break;
            puVar18 = puVar18 + 1;
            if (puVar18 == puVar1) {
              return;
            }
          }
          lVar20 = *(longlong *)(lVar10 + 0x1a8) + (ulonglong)*puVar18 * 0x48;
          lVar15 = *(longlong *)(lVar20 + 0x18);
          lVar20 = ~*(ulonglong *)(*(longlong *)(lVar20 + 0x38) + *(longlong *)(lVar9 + 0x110) * 8)
                   * 0x30;
          local_158 = *(longlong *)(lVar15 + 0x10 + lVar20);
          local_138 = *(longlong *)(lVar15 + 0x28 + lVar20);
          puVar18 = puVar18 + 1;
          uVar21 = 0;
        }
        uVar16 = uVar21 & 0xffffffff;
      }
      pfVar2 = (float *)(local_158 + uVar16 * 0x18);
      puVar3 = (undefined4 *)(local_138 + uVar16 * 4);
      uVar21 = uVar21 + 1;
      country_core::resources::roofs::roof_shape::Roof::is_flat();
    } while (extraout_AL != '\0');
    country_core::resources::roofs::roof_shape::Roof::pivot_3d();
    fVar24 = *(float *)(lVar8 + 0x28);
    fVar25 = *pfVar2;
    fVar28 = pfVar2[2];
    uVar22 = -(uint)(fVar25 / fVar24 <= -120.0);
    uVar23 = -(uint)(fStack_128 - fVar25 < fVar28);
    fVar27 = fVar24 * -45.126003 + pfVar2[3];
    fVar28 = fVar24 * fVar27 +
             (float)(uVar23 & (uint)fVar28 |
                    ~uVar23 & (-(uint)(0.0 <= fVar25) & (uint)fVar28 |
                              ~-(uint)(0.0 <= fVar25) &
                              (~uVar22 & (uint)fStack_128 | uVar22 & (uint)fVar28)));
    if (fVar28 <= fStack_128) {
      if (15.0 < fVar27) {
                    // WARNING: Subroutine does not return
        core::f32::impl$0::clamp::do_panic::runtime();
      }
      fVar26 = (fVar25 / fVar24) * 0.166;
      fVar25 = fStack_128;
      if (fStack_128 <= fVar28) {
        fVar25 = fVar28;
      }
      fVar28 = (float)(~-(uint)NAN(fVar28) & (uint)fVar25 | -(uint)NAN(fVar28) & (uint)fStack_128);
      fVar25 = fVar27;
      if (fVar27 <= fVar26) {
        fVar25 = fVar26;
      }
      fVar27 = 15.0;
      if (fVar25 <= 15.0) {
        fVar27 = fVar25;
      }
    }
    fVar24 = expf(fVar24 * -13.300004);
    fVar24 = (1.0 - fVar24) * fStack_128 + (1.0 - (1.0 - fVar24)) * fVar28;
    pfVar2[2] = fVar24;
    *puVar3 = uVar6;
    pfVar2[3] = fVar27;
    fVar24 = ABS(fVar24 - fStack_128);
    if (*(char *)(pfVar2 + 5) == '\0') {
      if (fVar24 < 0.1) {
        pfVar2[4] = 0.0;
        goto LAB_1421a7647;
      }
      *puVar12 = uVar7;
      lVar20 = *(longlong *)(lVar11 + 0x30);
      uVar13 = *(undefined8 *)(lVar11 + 0x40);
      if (lVar20 == *(longlong *)(lVar11 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                  ();
      }
      lVar15 = *(longlong *)(lVar11 + 0x28);
      *(undefined8 *)(lVar15 + lVar20 * 0x28) = uVar13;
      *(undefined1 *)(lVar15 + 8 + lVar20 * 0x28) = 5;
      uStack_f5 = (undefined1)local_12c;
      puVar4 = (undefined8 *)(lVar15 + 0x14 + lVar20 * 0x28);
      *puVar4 = CONCAT44(local_12c,uStack_f9);
      puVar4[1] = CONCAT44(local_124,fStack_128);
      puVar4 = (undefined8 *)(lVar15 + 9 + lVar20 * 0x28);
      *puVar4 = local_104;
      puVar4[1] = CONCAT17(uStack_f5,CONCAT43(uStack_f9,uStack_fc));
      *(longlong *)(lVar11 + 0x30) = lVar20 + 1;
      *(longlong *)(lVar11 + 0x40) = *(longlong *)(lVar11 + 0x40) + 1;
      uVar14 = 1;
    }
    else if (fVar24 <= 0.1) {
      if (0.01 < pfVar2[4]) {
        fVar24 = (pfVar2[4] + -0.01) / 0.19;
        uStack_fc = (undefined3)local_12c;
        uStack_f9 = (undefined4)(CONCAT44(fStack_128,local_12c) >> 0x18);
        uStack_f5 = (undefined1)((uint)fStack_128 >> 0x18);
        *puVar12 = uVar7;
        lVar20 = *(longlong *)(lVar11 + 0x30);
        uVar13 = *(undefined8 *)(lVar11 + 0x40);
        if (lVar20 == *(longlong *)(lVar11 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                    ();
        }
        fVar25 = 0.0;
        if (0.0 <= fVar24) {
          fVar25 = fVar24;
        }
        fVar24 = 1.0;
        if (fVar25 <= 1.0) {
          fVar24 = fVar25;
        }
        lVar15 = *(longlong *)(lVar11 + 0x28);
        *(undefined8 *)(lVar15 + lVar20 * 0x28) = uVar13;
        *(undefined1 *)(lVar15 + 8 + lVar20 * 0x28) = 4;
        *(float *)(lVar15 + 0xc + lVar20 * 0x28) = fVar24;
        *(undefined4 *)(lVar15 + 0x20 + lVar20 * 0x28) = local_124;
        puVar4 = (undefined8 *)(lVar15 + 0x10 + lVar20 * 0x28);
        *puVar4 = local_104;
        puVar4[1] = CONCAT17(uStack_f5,CONCAT43(uStack_f9,uStack_fc));
        *(longlong *)(lVar11 + 0x30) = lVar20 + 1;
        *(longlong *)(lVar11 + 0x40) = *(longlong *)(lVar11 + 0x40) + 1;
      }
LAB_1421a7647:
      uVar14 = 0;
    }
    else {
      uVar14 = 1;
      if (fVar27 < 0.0) {
        fVar24 = *(float *)(lVar8 + 0x28);
        *puVar3 = uVar6;
        pfVar2[4] = fVar24 + pfVar2[4];
      }
    }
    *puVar3 = uVar6;
    *(undefined1 *)(pfVar2 + 5) = uVar14;
  } while( true );
}

