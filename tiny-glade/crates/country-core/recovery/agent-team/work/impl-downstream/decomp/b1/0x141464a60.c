
void __cdecl
country_core::systems::wall::shapes::rectangle::finish_edit_rectangle::finish_edit_rectangle(void)

{
  char *pcVar1;
  int *piVar2;
  undefined8 *puVar3;
  int iVar4;
  float fVar5;
  float fVar6;
  undefined8 uVar7;
  int *piVar8;
  undefined8 uVar9;
  ulonglong *puVar10;
  ulonglong uVar11;
  longlong *plVar12;
  uint uVar13;
  uint uVar14;
  longlong lVar15;
  ulonglong extraout_RAX;
  longlong *in_RCX;
  longlong lVar16;
  longlong lVar17;
  undefined8 *in_RDX;
  ulonglong uVar18;
  longlong lVar19;
  longlong in_R8;
  ulonglong uVar20;
  longlong lVar21;
  ulonglong uVar22;
  longlong in_R9;
  ulonglong uVar23;
  longlong lVar24;
  uint uVar25;
  ulonglong uVar26;
  longlong lVar27;
  ushort uVar28;
  ulonglong uVar29;
  longlong lVar30;
  undefined4 uVar31;
  longlong lVar32;
  byte bVar33;
  char cVar34;
  char cVar38;
  char cVar39;
  float fVar35;
  float fVar36;
  char cVar40;
  float fVar41;
  undefined1 auVar37 [16];
  undefined1 uVar42;
  undefined1 uVar43;
  undefined1 uVar44;
  undefined1 uVar45;
  undefined1 uVar46;
  undefined1 uVar47;
  undefined1 uVar48;
  undefined1 uVar49;
  undefined1 uVar50;
  undefined1 uVar51;
  undefined1 uVar52;
  undefined1 uVar53;
  undefined1 uVar54;
  undefined1 uVar55;
  undefined1 uVar56;
  undefined1 uVar57;
  undefined1 auVar58 [16];
  undefined1 auVar59 [16];
  int iVar60;
  int iVar61;
  int iVar62;
  int iVar63;
  int iVar64;
  undefined4 uVar65;
  longlong in_stack_00000028;
  undefined8 *in_stack_00000030;
  longlong *in_stack_00000038;
  longlong *in_stack_00000040;
  longlong *in_stack_00000048;
  undefined8 local_238;
  undefined8 uStack_230;
  undefined8 local_228;
  undefined1 auStack_220 [16];
  
  piVar8 = (int *)*in_RDX;
  if (*piVar8 != 0) {
    if (*piVar8 == 2) {
      iVar64 = (int)*(undefined8 *)(piVar8 + 2);
      uVar65 = (undefined4)((ulonglong)*(undefined8 *)(piVar8 + 2) >> 0x20);
      iVar60 = piVar8[4];
      iVar61 = piVar8[5];
      iVar62 = piVar8[6];
      iVar63 = piVar8[7];
      local_238 = *(undefined8 *)(piVar8 + 8);
      uStack_230 = *(undefined8 *)(piVar8 + 10);
      local_228 = *(undefined8 *)(piVar8 + 0xc);
      auStack_220 = *(undefined1 (*) [16])(piVar8 + 0xe);
      uVar31 = 2;
    }
    else {
      iVar60 = piVar8[4];
      iVar61 = piVar8[5];
      iVar62 = piVar8[6];
      iVar63 = piVar8[7];
      local_238 = *(undefined8 *)(piVar8 + 8);
      uStack_230 = *(undefined8 *)(piVar8 + 10);
      local_228 = *(undefined8 *)(piVar8 + 0xc);
      auStack_220 = *(undefined1 (*) [16])(piVar8 + 0xe);
      iVar64 = piVar8[2];
      uVar65 = 0;
      uVar31 = 1;
    }
    iVar4 = piVar8[1];
    uVar7 = *(undefined8 *)(piVar8 + 0x12);
    *(undefined4 *)in_stack_00000038[2] = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
    lVar15 = *in_stack_00000038;
    lVar17 = *(longlong *)(lVar15 + 0x30);
    uVar9 = *(undefined8 *)(lVar15 + 0x40);
    if (lVar17 == *(longlong *)(lVar15 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<gpu_profiler::backend::ash::ProfilerOutputBuffer<rhapsody::raw::profiler::ProfilerBuffer>,alloc::alloc::Global>
                ();
    }
    lVar19 = *(longlong *)(lVar15 + 0x28);
    lVar16 = lVar17 * 0x58;
    *(undefined4 *)(lVar19 + lVar16) = uVar31;
    *(int *)(lVar19 + 4 + lVar16) = iVar4;
    *(ulonglong *)(lVar19 + 8 + lVar16) = CONCAT44(uVar65,iVar64);
    piVar2 = (int *)(lVar19 + 0x10 + lVar16);
    *piVar2 = iVar60;
    piVar2[1] = iVar61;
    piVar2[2] = iVar62;
    piVar2[3] = iVar63;
    puVar3 = (undefined8 *)(lVar19 + 0x20 + lVar16);
    *puVar3 = local_238;
    puVar3[1] = uStack_230;
    puVar3 = (undefined8 *)(lVar19 + 0x30 + lVar16);
    *puVar3 = local_228;
    puVar3[1] = auStack_220._0_8_;
    *(undefined8 *)(lVar19 + 0x40 + lVar16) = auStack_220._8_8_;
    *(undefined8 *)(lVar19 + 0x48 + lVar16) = uVar7;
    *(undefined8 *)(lVar19 + 0x50 + lVar16) = uVar9;
    *(longlong *)(lVar15 + 0x30) = lVar17 + 1;
    *(longlong *)(lVar15 + 0x40) = *(longlong *)(lVar15 + 0x40) + 1;
  }
  puVar10 = (ulonglong *)*in_stack_00000030;
  lVar15 = in_stack_00000030[1];
  uVar22 = *puVar10;
  uVar20 = 0;
  uVar23 = uVar22 - *(ulonglong *)(lVar15 + 0x18);
  if (uVar22 < *(ulonglong *)(lVar15 + 0x18)) {
    uVar23 = uVar20;
  }
  uVar18 = uVar22 - *(ulonglong *)(lVar15 + 0x38);
  if (uVar22 < *(ulonglong *)(lVar15 + 0x38)) {
    uVar18 = uVar20;
  }
  uVar22 = *(ulonglong *)(lVar15 + 0x30);
  uVar26 = uVar22 - uVar18;
  if (uVar22 < uVar18) {
    uVar26 = uVar20;
  }
  uVar11 = *(ulonglong *)(lVar15 + 0x10);
  uVar29 = uVar11 - uVar23;
  if (uVar11 < uVar23) {
    uVar29 = uVar20;
  }
  uVar20 = *(ulonglong *)(lVar15 + 0x40);
  *puVar10 = uVar20 - (uVar29 + uVar26);
  if ((uVar11 < uVar23 || uVar11 - uVar23 == 0) && (uVar22 < uVar18 || uVar22 - uVar18 == 0)) {
    return;
  }
  *puVar10 = uVar20;
  if (*piVar8 == 1) {
    lVar17 = 0x4c;
    lVar19 = 0x48;
    lVar15 = 0x3c;
    lVar21 = 0x38;
    lVar24 = 0x34;
    lVar27 = 0x30;
    lVar32 = 0x24;
    lVar16 = 0x20;
    lVar30 = 0x10;
  }
  else {
    if (*piVar8 != 2) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    lVar17 = 0x44;
    lVar19 = 0x40;
    lVar15 = 0x34;
    lVar21 = 0x30;
    lVar24 = 0x2c;
    lVar27 = 0x28;
    lVar32 = 0x1c;
    lVar16 = 0x18;
    lVar30 = 8;
  }
  lVar30 = *(longlong *)((longlong)piVar8 + lVar30);
  fVar36 = *(float *)((longlong)piVar8 + lVar27);
  fVar41 = *(float *)((longlong)piVar8 + lVar24);
  fVar5 = *(float *)((longlong)piVar8 + lVar17);
  fVar6 = *(float *)((longlong)piVar8 + lVar19);
  uVar43 = 0;
  uVar45 = 0;
  uVar47 = 0;
  uVar49 = 0;
  uVar50 = 0;
  uVar51 = 0;
  uVar52 = 0;
  uVar53 = 0;
  uVar54 = 0;
  uVar55 = 0;
  uVar56 = 0;
  uVar57 = 0;
  fVar35 = *(float *)((longlong)piVar8 + lVar21) * *(float *)((longlong)piVar8 + lVar16);
  uVar42 = SUB41(fVar35,0);
  uVar44 = (undefined1)((uint)fVar35 >> 8);
  uVar46 = (undefined1)((uint)fVar35 >> 0x10);
  uVar48 = (undefined1)((uint)fVar35 >> 0x18);
  fVar35 = acosf(*(float *)((longlong)piVar8 + lVar15) * *(float *)((longlong)piVar8 + lVar32) +
                 fVar35);
  if (ABS(fVar35) * 57.29578 <= 0.01) {
    fVar36 = fVar36 - fVar6;
    fVar41 = fVar41 - fVar5;
    fVar41 = fVar41 * fVar41;
    uVar43 = SUB41(fVar41,0);
    uVar45 = (undefined1)((uint)fVar41 >> 8);
    uVar47 = (undefined1)((uint)fVar41 >> 0x10);
    uVar49 = (undefined1)((uint)fVar41 >> 0x18);
    uVar50 = 0;
    uVar51 = 0;
    uVar52 = 0;
    uVar53 = 0;
    fVar41 = fVar41 + fVar36 * fVar36;
    uVar42 = SUB41(fVar41,0);
    uVar44 = (undefined1)((uint)fVar41 >> 8);
    uVar46 = (undefined1)((uint)fVar41 >> 0x10);
    uVar48 = (undefined1)((uint)fVar41 >> 0x18);
    uVar54 = uVar50;
    uVar55 = uVar51;
    uVar56 = uVar52;
    uVar57 = uVar53;
    if (SQRT(fVar41) <= 0.01) {
      **(undefined4 **)(in_stack_00000028 + 0x10) = *(undefined4 *)(in_stack_00000028 + 0x1c);
      **(undefined4 **)(in_R9 + 0x10) = *(undefined4 *)(in_R9 + 0x1c);
      resources::walls::decorator_archivist::DecoratorArchivist::extract_and_restore_to_storage();
LAB_14146504f:
      *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
      *piVar8 = 0;
      return;
    }
  }
  plVar12 = (longlong *)*in_RCX;
  if (plVar12[3] != 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar15 = *plVar12;
    bVar33 = (byte)(extraout_RAX >> 0x39);
    auVar37 = ZEXT216(CONCAT11(bVar33,bVar33));
    auVar37 = pshuflw(auVar37,auVar37,0);
    lVar17 = 0;
    iVar64 = -(uint)(CONCAT13(uVar48,CONCAT12(uVar46,CONCAT11(uVar44,uVar42))) ==
                    CONCAT13(uVar48,CONCAT12(uVar46,CONCAT11(uVar44,uVar42))));
    iVar60 = -(uint)(CONCAT13(uVar49,CONCAT12(uVar47,CONCAT11(uVar45,uVar43))) ==
                    CONCAT13(uVar49,CONCAT12(uVar47,CONCAT11(uVar45,uVar43))));
    iVar61 = -(uint)(CONCAT13(uVar53,CONCAT12(uVar52,CONCAT11(uVar51,uVar50))) ==
                    CONCAT13(uVar53,CONCAT12(uVar52,CONCAT11(uVar51,uVar50))));
    iVar62 = -(uint)(CONCAT13(uVar57,CONCAT12(uVar56,CONCAT11(uVar55,uVar54))) ==
                    CONCAT13(uVar57,CONCAT12(uVar56,CONCAT11(uVar55,uVar54))));
    uVar22 = extraout_RAX;
    while( true ) {
      uVar22 = plVar12[1] & uVar22;
      pcVar1 = (char *)(lVar15 + uVar22);
      cVar34 = auVar37[0];
      auVar59[0] = -(*pcVar1 == cVar34);
      cVar38 = auVar37[1];
      auVar59[1] = -(pcVar1[1] == cVar38);
      cVar39 = auVar37[2];
      auVar59[2] = -(pcVar1[2] == cVar39);
      cVar40 = auVar37[3];
      auVar59[3] = -(pcVar1[3] == cVar40);
      auVar59[4] = -(pcVar1[4] == cVar34);
      auVar59[5] = -(pcVar1[5] == cVar38);
      auVar59[6] = -(pcVar1[6] == cVar39);
      auVar59[7] = -(pcVar1[7] == cVar40);
      auVar59[8] = -(pcVar1[8] == cVar34);
      auVar59[9] = -(pcVar1[9] == cVar38);
      auVar59[10] = -(pcVar1[10] == cVar39);
      auVar59[0xb] = -(pcVar1[0xb] == cVar40);
      auVar59[0xc] = -(pcVar1[0xc] == cVar34);
      auVar59[0xd] = -(pcVar1[0xd] == cVar38);
      auVar59[0xe] = -(pcVar1[0xe] == cVar39);
      auVar59[0xf] = -(pcVar1[0xf] == cVar40);
      uVar28 = (ushort)(SUB161(auVar59 >> 7,0) & 1) | (ushort)(SUB161(auVar59 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar59 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar59 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar59 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar59 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar59 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar59 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar59 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar59 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar59 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar59 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar59 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar59 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar59 >> 0x77,0) & 1) << 0xe | (ushort)(auVar59[0xf] >> 7) << 0xf;
      uVar25 = (uint)uVar28;
      while (uVar28 != 0) {
        uVar13 = 0;
        for (uVar14 = uVar25; (uVar14 & 1) == 0; uVar14 = uVar14 >> 1 | 0x80000000) {
          uVar13 = uVar13 + 1;
        }
        lVar19 = (uVar13 + uVar22 & plVar12[1]) * -0x78;
        if (*(longlong *)(lVar15 + -0x78 + lVar19) == lVar30) {
          uVar25 = *(uint *)(lVar15 + -0x38 + lVar19);
          if (1 < uVar25) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_fmt();
          }
          if (uVar25 != 1) {
                    // WARNING: Subroutine does not return
            core::panicking::panic();
          }
          resources::history::wall_compound_edit::CompoundEditBuilder::next();
          **(undefined4 **)(in_R8 + 0x10) = *(undefined4 *)(in_R8 + 0x1c);
          alloc::boxed::iter::impl$13::
          from_iter<country_core::resources::history::wall_compound_edit::ConcurrentEdits,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2_<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl_0::new::closure_env_0>_>
                    ();
          resources::history::History::add();
          *(undefined4 *)in_stack_00000040[2] = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
          lVar15 = *in_stack_00000040;
          lVar17 = *(longlong *)(lVar15 + 0x30);
          uVar7 = *(undefined8 *)(lVar15 + 0x40);
          if (lVar17 == *(longlong *)(lVar15 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                      ();
          }
          *(undefined8 *)(*(longlong *)(lVar15 + 0x28) + lVar17 * 8) = uVar7;
          *(longlong *)(lVar15 + 0x30) = lVar17 + 1;
          *(longlong *)(lVar15 + 0x40) = *(longlong *)(lVar15 + 0x40) + 1;
          *(undefined4 *)in_stack_00000048[2] = *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
          lVar15 = *in_stack_00000048;
          lVar17 = *(longlong *)(lVar15 + 0x30);
          uVar7 = *(undefined8 *)(lVar15 + 0x40);
          if (lVar17 == *(longlong *)(lVar15 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                      ();
          }
          lVar19 = *(longlong *)(lVar15 + 0x28);
          *(undefined8 *)(lVar19 + lVar17 * 0x10) = uVar7;
          *(longlong *)(lVar19 + 8 + lVar17 * 0x10) = lVar30;
          *(longlong *)(lVar15 + 0x30) = lVar17 + 1;
          *(longlong *)(lVar15 + 0x40) = *(longlong *)(lVar15 + 0x40) + 1;
          goto LAB_14146504f;
        }
        uVar28 = (ushort)(uVar25 - 1) & (ushort)uVar25;
        uVar25 = CONCAT22((short)(uVar25 - 1 >> 0x10),uVar28);
      }
      auVar58[0] = -(*pcVar1 == (char)iVar64);
      auVar58[1] = -(pcVar1[1] == (char)((uint)iVar64 >> 8));
      auVar58[2] = -(pcVar1[2] == (char)((uint)iVar64 >> 0x10));
      auVar58[3] = -(pcVar1[3] == (char)((uint)iVar64 >> 0x18));
      auVar58[4] = -(pcVar1[4] == (char)iVar60);
      auVar58[5] = -(pcVar1[5] == (char)((uint)iVar60 >> 8));
      auVar58[6] = -(pcVar1[6] == (char)((uint)iVar60 >> 0x10));
      auVar58[7] = -(pcVar1[7] == (char)((uint)iVar60 >> 0x18));
      auVar58[8] = -(pcVar1[8] == (char)iVar61);
      auVar58[9] = -(pcVar1[9] == (char)((uint)iVar61 >> 8));
      auVar58[10] = -(pcVar1[10] == (char)((uint)iVar61 >> 0x10));
      auVar58[0xb] = -(pcVar1[0xb] == (char)((uint)iVar61 >> 0x18));
      auVar58[0xc] = -(pcVar1[0xc] == (char)iVar62);
      auVar58[0xd] = -(pcVar1[0xd] == (char)((uint)iVar62 >> 8));
      auVar58[0xe] = -(pcVar1[0xe] == (char)((uint)iVar62 >> 0x10));
      auVar58[0xf] = -(pcVar1[0xf] == (char)((uint)iVar62 >> 0x18));
      if ((((((((((((((((SUB161(auVar58 >> 7,0) & 1) != 0 || (SUB161(auVar58 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar58 >> 0x17,0) & 1) != 0) || (SUB161(auVar58 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar58 >> 0x27,0) & 1) != 0) || (SUB161(auVar58 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar58 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar58 >> 0x3f,0) & 1) != 0) || (SUB161(auVar58 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar58 >> 0x4f,0) & 1) != 0) || (SUB161(auVar58 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar58 >> 0x5f,0) & 1) != 0) || (SUB161(auVar58 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar58 >> 0x6f,0) & 1) != 0) || (SUB161(auVar58 >> 0x77,0) & 1) != 0) ||
          auVar58[0xf] < '\0') break;
      uVar22 = uVar22 + lVar17 + 0x10;
      lVar17 = lVar17 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

