// ADDR 0x140822e00
// SIG void __cdecl op_preview_audio(void)
// ORIG system::op_preview_audio


// WARNING: Control flow encountered bad instruction data
// WARNING: Instruction at (ram,0x000142a91168) overlaps instruction at (ram,0x000142a91167)
// 

void __cdecl system_decorator::audio::op_preview_audio::op_preview_audio(void)

{
  byte *pbVar1;
  char *pcVar2;
  undefined8 *puVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  char cVar6;
  ulonglong *puVar7;
  ulonglong uVar8;
  longlong lVar9;
  undefined8 uVar10;
  longlong lVar11;
  bool bVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  byte bVar15;
  uint uVar16;
  longlong lVar17;
  longlong extraout_RAX;
  uint *extraout_RAX_00;
  uint *puVar18;
  int *piVar19;
  ulonglong uVar20;
  byte bVar21;
  undefined8 *in_RCX;
  uint *puVar22;
  byte bVar23;
  undefined8 *in_RDX;
  longlong lVar24;
  uint *puVar25;
  ulonglong uVar26;
  ulonglong uVar27;
  longlong *plVar28;
  undefined1 *puVar29;
  longlong *plVar32;
  ulonglong uVar33;
  longlong *plVar34;
  longlong *plVar35;
  ulonglong uVar36;
  undefined4 *puVar37;
  undefined4 uVar38;
  undefined8 *in_R8;
  undefined *puVar39;
  undefined7 uVar40;
  longlong *in_R9;
  ulonglong uVar41;
  longlong *plVar42;
  undefined4 *puVar43;
  longlong *plVar44;
  longlong *plVar45;
  longlong *plVar46;
  unkbyte10 in_ST0;
  unkbyte10 Var47;
  unkbyte10 in_ST1;
  longlong *in_stack_00000028;
  undefined8 *in_stack_00000030;
  undefined1 auStack_f8 [44];
  undefined4 local_cc;
  undefined4 local_78;
  undefined4 local_74;
  longlong local_70;
  undefined8 local_68;
  longlong local_60;
  longlong *plVar30;
  longlong *plVar31;
  
  puVar29 = auStack_f8;
  puVar7 = (ulonglong *)*in_RDX;
  lVar24 = in_RDX[1];
  uVar20 = *puVar7;
  uVar27 = 0;
  uVar33 = uVar20 - *(ulonglong *)(lVar24 + 0x18);
  if (uVar20 < *(ulonglong *)(lVar24 + 0x18)) {
    uVar33 = uVar27;
  }
  uVar41 = uVar20 - *(ulonglong *)(lVar24 + 0x38);
  if (uVar20 < *(ulonglong *)(lVar24 + 0x38)) {
    uVar41 = uVar27;
  }
  uVar20 = *(ulonglong *)(lVar24 + 0x30);
  uVar36 = uVar20 - uVar41;
  if (uVar41 > uVar20) {
    uVar36 = uVar27;
  }
  uVar8 = *(ulonglong *)(lVar24 + 0x10);
  uVar26 = uVar8 - uVar33;
  if (uVar33 > uVar8) {
    uVar26 = uVar27;
  }
  uVar27 = *(ulonglong *)(lVar24 + 0x40);
  *puVar7 = uVar27 - (uVar26 + uVar36);
  if (uVar33 <= uVar8 && uVar8 - uVar33 != 0 || uVar41 <= uVar20 && uVar20 - uVar41 != 0) {
    *puVar7 = uVar27;
    return;
  }
  puVar7 = (ulonglong *)*in_RCX;
  lVar24 = in_RCX[1];
  uVar20 = *puVar7;
  uVar27 = 0;
  uVar33 = uVar20 - *(ulonglong *)(lVar24 + 0x18);
  if (uVar20 < *(ulonglong *)(lVar24 + 0x18)) {
    uVar33 = uVar27;
  }
  uVar41 = uVar20 - *(ulonglong *)(lVar24 + 0x38);
  if (uVar20 < *(ulonglong *)(lVar24 + 0x38)) {
    uVar41 = uVar27;
  }
  plVar44 = (longlong *)(uVar33 * 0x90 + *(longlong *)(lVar24 + 8));
  uVar20 = *(ulonglong *)(lVar24 + 0x10) - uVar33;
  if (*(ulonglong *)(lVar24 + 0x10) < uVar33) {
    plVar44 = (longlong *)&DAT_00000008;
    uVar20 = uVar27;
  }
  plVar32 = (longlong *)(uVar41 * 0x90 + *(longlong *)(lVar24 + 0x28));
  uVar33 = *(ulonglong *)(lVar24 + 0x30) - uVar41;
  if (*(ulonglong *)(lVar24 + 0x30) < uVar41) {
    plVar32 = (longlong *)&DAT_00000008;
    uVar33 = uVar27;
  }
  *puVar7 = (*(longlong *)(lVar24 + 0x40) - uVar33) - uVar20;
  plVar45 = plVar44 + uVar20 * 0x12;
  plVar46 = plVar32 + uVar33 * 0x12;
  lVar24 = *in_stack_00000028;
  plVar42 = (longlong *)*in_R8;
  puVar25 = (uint *)in_R8[2];
  puVar39 = (undefined *)(ulonglong)*(uint *)((longlong)in_R8 + 0x1c);
  local_68 = *in_stack_00000030;
  local_74 = *(undefined4 *)((longlong)in_R9 + 0x1c);
  puVar43 = (undefined4 *)in_R9[2];
  local_70 = *in_R9;
  local_60 = *in_R9 + 0x20;
  local_78 = (undefined4)local_60;
  local_cc = local_78;
LAB_140822f6f:
  while( true ) {
    do {
      if ((plVar44 == (longlong *)0x0) || (plVar44 == plVar45)) {
        if (plVar32 == (longlong *)0x0) {
          return;
        }
        if (plVar32 == plVar46) {
          return;
        }
        *puVar7 = *puVar7 + 1;
        cVar6 = *(char *)((longlong)plVar32 + 0x7c);
        plVar28 = (longlong *)0x0;
        plVar34 = plVar32 + 0x12;
        plVar30 = plVar32;
      }
      else {
        *puVar7 = *puVar7 + 1;
        cVar6 = *(char *)((longlong)plVar44 + 0x7c);
        plVar28 = plVar44 + 0x12;
        plVar34 = plVar32;
        plVar30 = plVar44;
      }
      plVar32 = plVar34;
      plVar44 = plVar28;
    } while ((cVar6 == '\x02') || (*(char *)((longlong)plVar30 + 0x81) == '?'));
    plVar28 = plVar30 + 0xe;
    uVar4 = *(undefined4 *)(lVar24 + 0x20 + (ulonglong)*(byte *)(plVar30 + 0x10) * 4);
    lVar17 = *plVar30 + -2;
    if (1 < *plVar30 - 3U) {
      lVar17 = 0;
    }
    uVar38 = SUB84(puVar39,0);
    if (lVar17 != 0) break;
    if ((char)plVar30[0xc] != '\0') goto code_r0x000140823106;
    lVar17 = plVar30[2];
    if (lVar17 != 3) {
      *(longlong **)(puVar29 + 0x30) = in_R9;
      *(longlong *)(puVar29 + 0x58) = lVar24;
      *(longlong **)(puVar29 + 0x40) = plVar42;
      *(uint **)(puVar29 + 0x50) = puVar25;
      *(undefined4 *)(puVar29 + 0x28) = uVar38;
      *(undefined4 **)(puVar29 + 0x48) = puVar43;
      if (lVar17 != 0) {
        *(longlong **)(puVar29 + 0x38) = plVar30 + 2;
LAB_1408233a5:
        bVar21 = *(byte *)((longlong)plVar30 + 0x42);
        plVar35 = plVar30;
code_r0x0001408233a9:
        bVar23 = *(char *)((longlong)plVar35 + 0x81) - 0x38;
        if (7 < bVar23) {
          bVar23 = 6;
        }
        bVar15 = 0x16;
        puVar39 = &DAT_142a91198 + *(int *)(&DAT_142a91198 + (ulonglong)bVar23 * 4);
        uVar40 = (undefined7)((ulonglong)puVar39 >> 8);
                    // WARNING: Could not find normalized switch variable to match jumptable
        switch(bVar23) {
        default:
          uVar38 = (undefined4)CONCAT71(uVar40,bVar21 == 2);
          bVar15 = 7;
          plVar34 = *(longlong **)(puVar29 + 0x38);
          break;
        case 3:
          uVar38 = (undefined4)CONCAT71(uVar40,bVar21 == 2);
          plVar34 = *(longlong **)(puVar29 + 0x38);
          break;
        case 4:
          uVar38 = (undefined4)CONCAT71(uVar40,bVar21 == 3);
          bVar15 = 0xc;
          plVar34 = *(longlong **)(puVar29 + 0x38);
          break;
        case 5:
          puVar18 = (uint *)(ulonglong)(byte)(bVar21 - 2);
code_r0x000140823487:
          bVar15 = 0x1c;
          plVar34 = *(longlong **)(puVar29 + 0x38);
          switch(puVar18) {
          case (uint *)0x0:
          case (uint *)0x2:
            uVar38 = (undefined4)CONCAT71((int7)((ulonglong)puVar39 >> 8),1);
            break;
          case (uint *)0x1:
            bVar15 = 0x1d;
            uVar38 = *(undefined4 *)(puVar29 + 0x80);
            break;
          default:
            uVar38 = 0;
          }
          break;
        case 6:
        case 0xbad1abe1bad1abe1:
          plVar34 = *(longlong **)(puVar29 + 0x38);
          if (*(char *)((longlong)plVar35 + 0x81) == '\t') {
            uVar38 = (undefined4)CONCAT71(uVar40,bVar21 == 3);
            bVar15 = 0xc;
          }
          else {
            uVar38 = (undefined4)CONCAT71(uVar40,bVar21 != 3);
            bVar15 = 0x24;
          }
          break;
        case 7:
          uVar38 = 0;
          plVar34 = *(longlong **)(puVar29 + 0x38);
          bVar15 = 0x16;
        }
        goto LAB_1408235d0;
      }
      *(longlong **)(puVar29 + 0x38) = plVar30 + 2;
      if (*(char *)((longlong)plVar30 + 0x42) != '\x02') goto LAB_1408233a5;
      *(undefined8 *)(puVar29 + -8) = 0x1408231df;
      country_core::resources::walls::public_walls::PublicWalls::roof_component();
      if (extraout_RAX == 0) goto LAB_1408233a5;
      *(undefined8 *)(puVar29 + -8) = 0x1408231f3;
      country_core::resources::roofs::roof_shape::Roof::is_flat();
      if (extraout_AL == '\0') goto LAB_1408233a5;
      *(undefined8 *)(puVar29 + -8) = 0x14082320b;
      country_core::resources::roofs::roof_shape::FlatRoofStyle::is_wooden();
      if (extraout_AL_00 == '\0') goto LAB_1408233a5;
      *(int *)(puVar29 + 0x78) = (int)plVar30[0xf];
      *(longlong *)(puVar29 + 0x70) = *plVar28;
      puVar29[0x60] = 6;
      *(undefined4 *)(puVar29 + 100) = uVar4;
      puVar29[0x7c] = 0;
      *(undefined8 *)(puVar29 + -8) = 0x14082324a;
      bevy_ecs::event::writer::EventWriter::
      write<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>();
      plVar34 = *(longlong **)(puVar29 + 0x38);
      goto LAB_140823611;
    }
  }
  if (lVar17 == 2) {
    if ((int)*plVar42 != 7) {
      *(int *)(puVar29 + 0x77) = (int)plVar30[0xf];
      *(longlong *)(puVar29 + 0x6f) = *plVar28;
      *(undefined4 **)(puVar29 + 0x48) = puVar43;
      *puVar43 = *(undefined4 *)(puVar29 + 0x84);
      lVar17 = *(longlong *)(puVar29 + 0x88);
      lVar9 = *(longlong *)(lVar17 + 0x30);
      uVar10 = *(undefined8 *)(lVar17 + 0x40);
      if (lVar9 == *(longlong *)(lVar17 + 0x20)) {
        *(longlong *)(puVar29 + 0x58) = lVar24;
        *(longlong **)(puVar29 + 0x30) = in_R9;
        *(uint **)(puVar29 + 0x50) = puVar25;
        *(longlong **)(puVar29 + 0x40) = plVar42;
        *(undefined4 *)(puVar29 + 0x28) = uVar38;
        *(undefined8 *)(puVar29 + -8) = 0x14082308a;
        alloc::raw_vec::RawVec::
        grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                  ();
        puVar39 = (undefined *)(ulonglong)*(uint *)(puVar29 + 0x28);
        plVar42 = *(longlong **)(puVar29 + 0x40);
        lVar24 = *(longlong *)(puVar29 + 0x58);
        puVar25 = *(uint **)(puVar29 + 0x50);
        in_R9 = *(longlong **)(puVar29 + 0x30);
      }
      lVar17 = *(longlong *)(puVar29 + 0x88);
      lVar11 = *(longlong *)(lVar17 + 0x28);
      *(undefined8 *)(lVar11 + lVar9 * 0x28) = uVar10;
      *(undefined1 *)(lVar11 + 8 + lVar9 * 0x28) = 5;
      uVar10 = *(undefined8 *)(puVar29 + 0x68);
      uVar13 = *(undefined8 *)(puVar29 + 0x6b);
      uVar14 = *(undefined8 *)(puVar29 + 0x73);
      puVar3 = (undefined8 *)(lVar11 + 9 + lVar9 * 0x28);
      *puVar3 = *(undefined8 *)(puVar29 + 0x60);
      puVar3[1] = uVar10;
      puVar3 = (undefined8 *)(lVar11 + 0x14 + lVar9 * 0x28);
      *puVar3 = uVar13;
      puVar3[1] = uVar14;
      *(undefined1 *)(lVar11 + 0x24 + lVar9 * 0x28) = 0;
      *(longlong *)(lVar17 + 0x30) = lVar9 + 1;
      *(longlong *)(lVar17 + 0x40) = *(longlong *)(lVar17 + 0x40) + 1;
      puVar43 = *(undefined4 **)(puVar29 + 0x48);
    }
    *puVar25 = (uint)puVar39;
    *plVar42 = 7;
    goto LAB_140822f6f;
  }
  *(uint **)(puVar29 + 0x50) = puVar25;
  *(longlong *)(puVar29 + 0x58) = lVar24;
  if ((plVar30[1] == 0) && (*(char *)((longlong)plVar30 + 0x3a) == '\x02')) {
    *(undefined4 *)(puVar29 + 0x28) = uVar38;
    *(longlong **)(puVar29 + 0x30) = in_R9;
    *(longlong **)(puVar29 + 0x40) = plVar42;
    *(undefined4 **)(puVar29 + 0x48) = puVar43;
    *(undefined8 *)(puVar29 + -8) = 0x1408232a0;
    country_core::resources::walls::public_walls::PublicWalls::roof_component();
    puVar43 = *(undefined4 **)(puVar29 + 0x48);
    puVar39 = (undefined *)(ulonglong)*(uint *)(puVar29 + 0x28);
    plVar42 = *(longlong **)(puVar29 + 0x40);
    in_R9 = *(longlong **)(puVar29 + 0x30);
    if (extraout_RAX_00 == (uint *)0x0) goto LAB_140823356;
    *(undefined8 *)(puVar29 + -8) = 0x1408232c8;
    country_core::resources::roofs::roof_shape::Roof::is_flat();
    puVar43 = *(undefined4 **)(puVar29 + 0x48);
    puVar39 = (undefined *)(ulonglong)*(uint *)(puVar29 + 0x28);
    plVar42 = *(longlong **)(puVar29 + 0x40);
    in_R9 = *(longlong **)(puVar29 + 0x30);
    puVar25 = extraout_RAX_00;
    if (extraout_AL_01 == '\0') goto LAB_140823356;
    puVar25 = extraout_RAX_00 + 0x15;
    *(undefined8 *)(puVar29 + -8) = 0x1408232f0;
    country_core::resources::roofs::roof_shape::FlatRoofStyle::is_wooden();
    puVar43 = *(undefined4 **)(puVar29 + 0x48);
    puVar39 = (undefined *)(ulonglong)*(uint *)(puVar29 + 0x28);
    plVar42 = *(longlong **)(puVar29 + 0x40);
    in_R9 = *(longlong **)(puVar29 + 0x30);
    if (extraout_AL_02 == '\0') goto LAB_140823356;
    *(int *)(puVar29 + 0x78) = (int)plVar30[0xf];
    *(longlong *)(puVar29 + 0x70) = *plVar28;
    puVar29[0x60] = 6;
    *(undefined4 *)(puVar29 + 100) = uVar4;
    puVar29[0x7c] = 0;
    *(undefined8 *)(puVar29 + -8) = 0x14082333d;
    bevy_ecs::event::writer::EventWriter::
    write<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>();
    puVar43 = *(undefined4 **)(puVar29 + 0x48);
    puVar39 = (undefined *)(ulonglong)*(uint *)(puVar29 + 0x28);
    plVar42 = *(longlong **)(puVar29 + 0x40);
    in_R9 = *(longlong **)(puVar29 + 0x30);
    goto LAB_1408235a8;
  }
LAB_140823356:
  uVar38 = SUB84(puVar39,0);
  bVar15 = *(byte *)((longlong)plVar30 + 0x3a);
  puVar18 = (uint *)(ulonglong)bVar15;
  uVar16 = (uint)bVar15;
  bVar21 = *(byte *)((longlong)plVar30 + 0x81);
  puVar22 = (uint *)(ulonglong)bVar21;
  bVar23 = bVar21 - 0x38;
  if (7 < bVar23) {
    bVar23 = 6;
  }
  puVar37 = (undefined4 *)CONCAT71((int7)((ulonglong)puVar25 >> 8),0x16);
  plVar34 = (longlong *)&DAT_142a91168;
  puVar25 = (uint *)(&DAT_142a91168 + *(int *)(&DAT_142a91168 + (ulonglong)bVar23 * 4));
  plVar35 = plVar34;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(bVar23) {
  case 0:
  case 1:
  case 2:
    *(uint *)(puVar29 + 0x2c) = (uint)(bVar15 == 2);
    puVar37 = (undefined4 *)&DAT_00000007;
    break;
  case 3:
    *(uint *)(puVar29 + 0x2c) = (uint)(bVar15 == 2);
    break;
  case 4:
    goto code_r0x00014082343f;
  case 5:
    puVar18 = (uint *)(ulonglong)(byte)(bVar15 - 2);
    puVar22 = (uint *)((longlong)puVar18 + 1);
    puVar25 = (uint *)0x0;
  case 9:
  case 0xb:
    *(char *)((longlong)puVar18 + -0x73) = *(char *)((longlong)puVar18 + -0x73);
    uVar16 = *puVar22;
    *puVar22 = *puVar22 + 0x42a91168;
    pcVar2 = (char *)((longlong)plVar28 + (longlong)puVar18);
    bVar21 = (byte)puVar22 & 0x1f;
    cVar6 = *pcVar2;
    *pcVar2 = *pcVar2 >> bVar21;
    bVar12 = ((ulonglong)puVar22 & 0x1f) == 0;
    if (!(bool)(bVar12 * (0xbd56ee97 < uVar16) | !bVar12 * ((cVar6 >> bVar21 - 1 & 1U) != 0))) {
      puVar22 = puVar25;
    }
                    // WARNING: Could not recover jumptable at 0x00014082341c. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&UNK_142a91188 + *(int *)(&UNK_142a91188 + (longlong)puVar22 * 4)))();
    return;
  case 6:
    if (bVar21 == 9) goto code_r0x00014082343f;
    *(uint *)(puVar29 + 0x2c) = (uint)(bVar15 != 3);
    puVar37 = (undefined4 *)&DAT_00000024;
    break;
  case 7:
    *(undefined4 *)(puVar29 + 0x2c) = 0;
    break;
  case 8:
    return;
  case 10:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0xc:
  case 0xd:
  case 0xe:
    goto code_r0x0001408233a9;
  case 0xf:
    *(byte *)puVar18 = (char)*puVar18 + bVar15;
    *(char *)((longlong)puVar18 + -0x7af0f607) = *(char *)((longlong)puVar18 + -0x7af0f607) + bVar15
    ;
    uVar16 = *puVar18;
    puVar18 = (uint *)(ulonglong)uVar16;
    *(char *)puVar18 = (char)*puVar18 + (char)uVar16;
    goto code_r0x00014082343f;
  case 0x10:
    *(uint *)(puVar29 + 0x2c) = (uint)bVar15;
    break;
  case 0x11:
    uVar16 = (uint)bVar15 + *(int *)((longlong)puVar22 + 0xf);
    piVar19 = (int *)((ulonglong)puVar29 & 0xffffffff);
    *(char *)(piVar19 + 0x1d22d203) = (char)piVar19[0x1d22d203] << 4;
    uVar5 = in((short)puVar25);
    *puVar37 = uVar5;
    *piVar19 = *piVar19 + (int)puVar29;
    *(char *)((longlong)piVar19 + 0xf4102f9) =
         *(char *)((longlong)piVar19 + 0xf4102f9) + (char)piVar19;
    puVar29 = (undefined1 *)((ulonglong)puVar29 & 0xffffffff);
    pbVar1 = (byte *)((ulonglong)uVar16 - 0x75);
    *pbVar1 = *pbVar1 >> 4 | *pbVar1 << 4;
    bVar15 = (byte)uVar16 & 0x38;
    goto LAB_1408235d0;
  case 0x12:
    goto code_r0x000140823487;
  case 0x13:
    goto code_r0x000140823444;
  case 0x14:
    *(undefined1 **)(puVar29 + -8) = puVar29;
    uVar20 = (ulonglong)puVar18 & 0xffffffffffffff38;
    puVar29 = puVar29 + -8;
    goto code_r0x000140823574;
  case 0x15:
  case 0x17:
    plVar34 = (longlong *)(ulonglong)*(uint *)(puVar29 + 0x38);
LAB_1408235d0:
    *(int *)(puVar29 + 0x78) = (int)plVar30[0xf];
    *(longlong *)(puVar29 + 0x70) = *plVar28;
    puVar29[0x60] = bVar15;
    *(undefined4 *)(puVar29 + 0x80) = uVar38;
    puVar29[0x61] = (char)uVar38;
    *(undefined4 *)(puVar29 + 100) = uVar4;
    puVar29[0x7c] = 0;
    *(undefined8 *)(puVar29 + -8) = 0x140823611;
    bevy_ecs::event::writer::EventWriter::
    write<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>();
LAB_140823611:
    lVar24 = *plVar34;
    lVar17 = plVar34[1];
    puVar25 = *(uint **)(puVar29 + 0x50);
    puVar39 = (undefined *)(ulonglong)*(uint *)(puVar29 + 0x28);
    *puVar25 = *(uint *)(puVar29 + 0x28);
    plVar42 = *(longlong **)(puVar29 + 0x40);
    *plVar42 = lVar24;
    plVar42[1] = lVar17;
    in_R9 = *(longlong **)(puVar29 + 0x30);
    lVar24 = *(longlong *)(puVar29 + 0x58);
    puVar43 = *(undefined4 **)(puVar29 + 0x48);
    goto LAB_140822f6f;
  case 0x16:
    plVar30 = (longlong *)(puVar29 + -8);
    plVar31 = (longlong *)(puVar29 + -8);
    *(longlong **)(puVar29 + -8) = plVar32;
    cVar6 = '\x13';
    do {
      plVar32 = plVar32 + -1;
      plVar30 = plVar30 + -1;
      *plVar30 = *plVar32;
      cVar6 = cVar6 + -1;
    } while ('\0' < cVar6);
    *(undefined1 **)(puVar29 + -0xa8) = puVar29 + -8;
    puVar29 = puVar29 + -0x8bb0;
    uVar20 = (ulonglong)puVar18 & 0xffffffffffffff2c;
    plVar32 = plVar31;
    goto code_r0x00014082357c;
  case 0x18:
  case 0x19:
    *(char *)puVar25 = '\0';
    in_ST0 = fscale(in_ST0,in_ST1);
    goto code_r0x000142a9116c;
  default:
    *(char *)puVar18 = (char)*puVar18;
code_r0x000142a9116c:
    *(char *)puVar25 = '\0';
    Var47 = fscale(in_ST0,in_ST1);
    *(char *)puVar25 = '\0';
    Var47 = fscale(Var47,in_ST1);
    fscale(Var47,in_ST1);
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  }
LAB_1408234e9:
  puVar29[0x38] = (char)puVar37;
  *(int *)(puVar29 + 0x70) = (int)plVar30[0xf];
  *(longlong *)(puVar29 + 0x68) = *plVar28;
  *puVar43 = *(undefined4 *)(puVar29 + 0x84);
  lVar24 = *(longlong *)(puVar29 + 0x88);
  plVar28 = *(longlong **)(lVar24 + 0x30);
  uVar10 = *(undefined8 *)(lVar24 + 0x40);
  if (plVar28 == *(longlong **)(lVar24 + 0x20)) {
    *(longlong **)(puVar29 + 0x30) = in_R9;
    *(longlong **)(puVar29 + 0x40) = plVar42;
    *(undefined4 *)(puVar29 + 0x28) = uVar38;
    *(undefined4 **)(puVar29 + 0x48) = puVar43;
    *(undefined8 *)(puVar29 + -8) = 0x140823547;
    alloc::raw_vec::RawVec::
    grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
              ();
    puVar43 = *(undefined4 **)(puVar29 + 0x48);
    puVar39 = (undefined *)(ulonglong)*(uint *)(puVar29 + 0x28);
    plVar42 = *(longlong **)(puVar29 + 0x40);
    in_R9 = *(longlong **)(puVar29 + 0x30);
  }
  puVar37 = *(undefined4 **)(puVar29 + 0x88);
  uVar20 = *(ulonglong *)(puVar37 + 10);
  puVar22 = (uint *)((longlong)plVar28 * 5);
  *(undefined8 *)(uVar20 + (longlong)plVar28 * 0x28) = uVar10;
  puVar25 = (uint *)(ulonglong)(byte)puVar29[0x38];
code_r0x000140823574:
  *(char *)(uVar20 + 8 + (longlong)puVar22 * 8) = (char)puVar25;
  puVar25 = (uint *)(ulonglong)*(uint *)(puVar29 + 0x2c);
code_r0x00014082357c:
  *(char *)(uVar20 + 9 + (longlong)puVar22 * 8) = (char)puVar25;
  *(undefined4 *)(uVar20 + 0xc + (longlong)puVar22 * 8) = uVar4;
  uVar10 = *(undefined8 *)(puVar29 + 0x68);
  puVar3 = (undefined8 *)(uVar20 + 0x10 + (longlong)puVar22 * 8);
  *puVar3 = *(undefined8 *)(puVar29 + 0x60);
  puVar3[1] = uVar10;
  *(undefined4 *)(uVar20 + 0x20 + (longlong)puVar22 * 8) = *(undefined4 *)(puVar29 + 0x70);
  *(undefined1 *)(uVar20 + 0x24 + (longlong)puVar22 * 8) = 0;
  *(undefined1 **)(puVar37 + 0xc) = (undefined1 *)((longlong)plVar28 + 1);
  *(longlong *)(puVar37 + 0x10) = *(longlong *)(puVar37 + 0x10) + 1;
LAB_1408235a8:
  puVar25 = *(uint **)(puVar29 + 0x50);
  *puVar25 = (uint)puVar39;
  *plVar42 = 5;
  lVar24 = *(longlong *)(puVar29 + 0x58);
  goto LAB_140822f6f;
code_r0x000140823106:
  if ((int)*plVar42 != 6) {
    *(int *)(puVar29 + 0x78) = (int)plVar30[0xf];
    *(longlong *)(puVar29 + 0x70) = *plVar28;
    puVar29[0x60] = 3;
    puVar29[0x7c] = 0;
    *(longlong *)(puVar29 + 0x58) = lVar24;
    *(undefined4 *)(puVar29 + 0x28) = uVar38;
    *(longlong **)(puVar29 + 0x30) = in_R9;
    *(uint **)(puVar29 + 0x50) = puVar25;
    *(undefined8 *)(puVar29 + -8) = 0x140823155;
    bevy_ecs::event::writer::EventWriter::
    write<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>();
    puVar39 = (undefined *)(ulonglong)*(uint *)(puVar29 + 0x28);
    lVar24 = *(longlong *)(puVar29 + 0x58);
    puVar25 = *(uint **)(puVar29 + 0x50);
    in_R9 = *(longlong **)(puVar29 + 0x30);
  }
  *puVar25 = (uint)puVar39;
  *plVar42 = 6;
  goto LAB_140822f6f;
code_r0x00014082343f:
  uVar16 = (uint)CONCAT71((int7)((ulonglong)puVar18 >> 8),(char)puVar18 == '\x03');
code_r0x000140823444:
  *(uint *)(puVar29 + 0x2c) = uVar16;
  puVar37 = (undefined4 *)&DAT_0000000c;
  goto LAB_1408234e9;
}


