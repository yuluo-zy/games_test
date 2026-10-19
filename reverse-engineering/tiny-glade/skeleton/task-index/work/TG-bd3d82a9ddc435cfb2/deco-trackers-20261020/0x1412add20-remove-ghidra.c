
void __cdecl
system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker::
remove<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams>
          (void)

{
  char *pcVar1;
  longlong *plVar2;
  uint uVar3;
  uint uVar4;
  longlong extraout_RAX;
  ulonglong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  longlong *in_RCX;
  ulonglong uVar5;
  longlong lVar6;
  ulonglong uVar7;
  int in_R9D;
  uint uVar8;
  longlong lVar9;
  ushort uVar10;
  longlong lVar11;
  ulonglong uVar12;
  byte bVar13;
  char cVar14;
  char cVar16;
  char cVar17;
  char cVar18;
  undefined1 auVar15 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  undefined **local_d0;
  code *local_c8;
  undefined ***local_c0;
  code *local_b8;
  undefined8 local_b0;
  undefined1 local_a0 [8];
  undefined *local_98;
  longlong lStack_90;
  undefined8 local_88;
  undefined **local_78;
  code *local_70;
  undefined ***local_68;
  code *local_60;
  undefined1 *local_58;
  code *local_50;
  undefined1 local_44 [4];
  undefined8 local_40;
  
  local_40 = 0xfffffffffffffffe;
  country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall();
  if (in_RCX[3] != 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar9 = *in_RCX;
    bVar13 = (byte)(extraout_RAX_00 >> 0x39);
    auVar15 = ZEXT216(CONCAT11(bVar13,bVar13));
    auVar15 = pshuflw(auVar15,auVar15,0);
    lVar6 = 0;
    uVar5 = extraout_RAX_00;
    while( true ) {
      uVar5 = in_RCX[1] & uVar5;
      pcVar1 = (char *)(lVar9 + uVar5);
      cVar14 = auVar15[0];
      auVar21[0] = -(*pcVar1 == cVar14);
      cVar16 = auVar15[1];
      auVar21[1] = -(pcVar1[1] == cVar16);
      cVar17 = auVar15[2];
      auVar21[2] = -(pcVar1[2] == cVar17);
      cVar18 = auVar15[3];
      auVar21[3] = -(pcVar1[3] == cVar18);
      auVar21[4] = -(pcVar1[4] == cVar14);
      auVar21[5] = -(pcVar1[5] == cVar16);
      auVar21[6] = -(pcVar1[6] == cVar17);
      auVar21[7] = -(pcVar1[7] == cVar18);
      auVar21[8] = -(pcVar1[8] == cVar14);
      auVar21[9] = -(pcVar1[9] == cVar16);
      auVar21[10] = -(pcVar1[10] == cVar17);
      auVar21[0xb] = -(pcVar1[0xb] == cVar18);
      auVar21[0xc] = -(pcVar1[0xc] == cVar14);
      auVar21[0xd] = -(pcVar1[0xd] == cVar16);
      auVar21[0xe] = -(pcVar1[0xe] == cVar17);
      auVar21[0xf] = -(pcVar1[0xf] == cVar18);
      uVar10 = (ushort)(SUB161(auVar21 >> 7,0) & 1) | (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar21 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar21 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar21 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar21 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar21 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar21 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar21 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar21 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar21 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar21 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar21 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar21 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar21 >> 0x77,0) & 1) << 0xe | (ushort)(auVar21[0xf] >> 7) << 0xf;
      uVar8 = (uint)uVar10;
      while (uVar10 != 0) {
        uVar3 = 0;
        for (uVar4 = uVar8; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
          uVar3 = uVar3 + 1;
        }
        lVar11 = (uVar3 + uVar5 & in_RCX[1]) * -0x38;
        if (extraout_RAX == *(longlong *)(lVar9 + -0x38 + lVar11)) {
          if (*(longlong *)(lVar9 + -0x18 + lVar11) != 0) {
            core::hash::BuildHasher::
            hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>
                      ();
            lVar6 = *(longlong *)(lVar9 + lVar11 + -0x30);
            uVar5 = *(ulonglong *)(lVar9 + lVar11 + -0x28);
            bVar13 = (byte)(extraout_RAX_01 >> 0x39);
            auVar15 = ZEXT216(CONCAT11(bVar13,bVar13));
            auVar15 = pshuflw(auVar15,auVar15,0);
            lVar9 = 0;
            uVar7 = extraout_RAX_01;
            while( true ) {
              uVar7 = uVar5 & uVar7;
              auVar21 = *(undefined1 (*) [16])(lVar6 + uVar7);
              cVar14 = auVar15[0];
              auVar22[0] = -(auVar21[0] == cVar14);
              cVar16 = auVar15[1];
              auVar22[1] = -(auVar21[1] == cVar16);
              cVar17 = auVar15[2];
              auVar22[2] = -(auVar21[2] == cVar17);
              cVar18 = auVar15[3];
              auVar22[3] = -(auVar21[3] == cVar18);
              auVar22[4] = -(auVar21[4] == cVar14);
              auVar22[5] = -(auVar21[5] == cVar16);
              auVar22[6] = -(auVar21[6] == cVar17);
              auVar22[7] = -(auVar21[7] == cVar18);
              auVar22[8] = -(auVar21[8] == cVar14);
              auVar22[9] = -(auVar21[9] == cVar16);
              auVar22[10] = -(auVar21[10] == cVar17);
              auVar22[0xb] = -(auVar21[0xb] == cVar18);
              auVar22[0xc] = -(auVar21[0xc] == cVar14);
              auVar22[0xd] = -(auVar21[0xd] == cVar16);
              auVar22[0xe] = -(auVar21[0xe] == cVar17);
              auVar22[0xf] = -(auVar21[0xf] == cVar18);
              uVar10 = (ushort)(SUB161(auVar22 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar22 >> 0xf,0) & 1) << 1 |
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
                       (ushort)(SUB161(auVar22 >> 0x77,0) & 1) << 0xe |
                       (ushort)(auVar22[0xf] >> 7) << 0xf;
              uVar8 = (uint)uVar10;
              while (uVar10 != 0) {
                uVar3 = 0;
                for (uVar4 = uVar8; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
                  uVar3 = uVar3 + 1;
                }
                uVar12 = uVar3 + uVar7 & uVar5;
                if (*(int *)(lVar6 + -0x20 + uVar12 * -0x20) == in_R9D) {
                  lVar9 = uVar12 * -0x20;
                  local_88 = *(undefined8 *)(lVar6 + -8 + lVar9);
                  plVar2 = (longlong *)(lVar6 + -0x18 + lVar9);
                  local_98 = (undefined *)*plVar2;
                  lStack_90 = plVar2[1];
                  *(undefined8 *)(lVar6 + -0x18 + lVar9) = 4;
                    // WARNING: Could not recover jumptable at 0x0001412adf0b. Too many branches
                    // WARNING: Treating indirect jump as call
                  (*(code *)(&DAT_142c2b710 + *(int *)(&DAT_142c2b710 + (longlong)local_98 * 4)))
                            (&DAT_142c2b710 + *(int *)(&DAT_142c2b710 + (longlong)local_98 * 4));
                  return;
                }
                uVar10 = (ushort)(uVar8 - 1) & (ushort)uVar8;
                uVar8 = CONCAT22((short)(uVar8 - 1 >> 0x10),uVar10);
              }
              auVar20[0] = -(auVar21[0] == -1);
              auVar20[1] = -(auVar21[1] == -1);
              auVar20[2] = -(auVar21[2] == -1);
              auVar20[3] = -(auVar21[3] == -1);
              auVar20[4] = -(auVar21[4] == -1);
              auVar20[5] = -(auVar21[5] == -1);
              auVar20[6] = -(auVar21[6] == -1);
              auVar20[7] = -(auVar21[7] == -1);
              auVar20[8] = -(auVar21[8] == -1);
              auVar20[9] = -(auVar21[9] == -1);
              auVar20[10] = -(auVar21[10] == -1);
              auVar20[0xb] = -(auVar21[0xb] == -1);
              auVar20[0xc] = -(auVar21[0xc] == -1);
              auVar20[0xd] = -(auVar21[0xd] == -1);
              auVar20[0xe] = -(auVar21[0xe] == -1);
              auVar20[0xf] = -(auVar21[0xf] == -1);
              if ((((((((((((((((SUB161(auVar20 >> 7,0) & 1) != 0 ||
                               (SUB161(auVar20 >> 0xf,0) & 1) != 0) ||
                              (SUB161(auVar20 >> 0x17,0) & 1) != 0) ||
                             (SUB161(auVar20 >> 0x1f,0) & 1) != 0) ||
                            (SUB161(auVar20 >> 0x27,0) & 1) != 0) ||
                           (SUB161(auVar20 >> 0x2f,0) & 1) != 0) ||
                          (SUB161(auVar20 >> 0x37,0) & 1) != 0) ||
                         (SUB161(auVar20 >> 0x3f,0) & 1) != 0) ||
                        (SUB161(auVar20 >> 0x47,0) & 1) != 0) ||
                       (SUB161(auVar20 >> 0x4f,0) & 1) != 0) || (SUB161(auVar20 >> 0x57,0) & 1) != 0
                      ) || (SUB161(auVar20 >> 0x5f,0) & 1) != 0) ||
                    (SUB161(auVar20 >> 0x67,0) & 1) != 0) || (SUB161(auVar20 >> 0x6f,0) & 1) != 0)
                  || (SUB161(auVar20 >> 0x77,0) & 1) != 0) || auVar20[0xf] < '\0') break;
              uVar7 = uVar7 + lVar9 + 0x10;
              lVar9 = lVar9 + 0x10;
            }
          }
          local_98 = &anon_8b5f636f5ecef8ae31f545d005a6bf08_4_llvm_10917514084104779652;
          lStack_90 = 0x59;
          local_70 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>
          ;
          local_60 = country_core::resources::walls::impl$7::fmt;
          local_58 = local_44;
          local_50 = country_core::resources::walls::decorator::impl$27::fmt;
          local_d0 = (undefined **)&DAT_142c2b5e8;
          local_c8 = (code *)0x3;
          local_b0 = 0;
          local_c0 = &local_78;
          local_b8 = (code *)0x3;
          local_78 = &local_98;
          local_68 = (undefined ***)local_a0;
                    // WARNING: Subroutine does not return
          core::panicking::panic_fmt();
        }
        uVar10 = (ushort)(uVar8 - 1) & (ushort)uVar8;
        uVar8 = CONCAT22((short)(uVar8 - 1 >> 0x10),uVar10);
      }
      auVar19[0] = -(*pcVar1 == -1);
      auVar19[1] = -(pcVar1[1] == -1);
      auVar19[2] = -(pcVar1[2] == -1);
      auVar19[3] = -(pcVar1[3] == -1);
      auVar19[4] = -(pcVar1[4] == -1);
      auVar19[5] = -(pcVar1[5] == -1);
      auVar19[6] = -(pcVar1[6] == -1);
      auVar19[7] = -(pcVar1[7] == -1);
      auVar19[8] = -(pcVar1[8] == -1);
      auVar19[9] = -(pcVar1[9] == -1);
      auVar19[10] = -(pcVar1[10] == -1);
      auVar19[0xb] = -(pcVar1[0xb] == -1);
      auVar19[0xc] = -(pcVar1[0xc] == -1);
      auVar19[0xd] = -(pcVar1[0xd] == -1);
      auVar19[0xe] = -(pcVar1[0xe] == -1);
      auVar19[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar19 >> 7,0) & 1) != 0 || (SUB161(auVar19 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar19 >> 0x17,0) & 1) != 0) || (SUB161(auVar19 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar19 >> 0x27,0) & 1) != 0) || (SUB161(auVar19 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar19 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar19 >> 0x3f,0) & 1) != 0) || (SUB161(auVar19 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar19 >> 0x4f,0) & 1) != 0) || (SUB161(auVar19 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar19 >> 0x5f,0) & 1) != 0) || (SUB161(auVar19 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar19 >> 0x6f,0) & 1) != 0) || (SUB161(auVar19 >> 0x77,0) & 1) != 0) ||
          auVar19[0xf] < '\0') break;
      uVar5 = uVar5 + lVar6 + 0x10;
      lVar6 = lVar6 + 0x10;
    }
  }
  local_98 = &anon_8b5f636f5ecef8ae31f545d005a6bf08_4_llvm_10917514084104779652;
  lStack_90 = 0x59;
  local_c8 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  local_c0 = (undefined ***)local_a0;
  local_b8 = country_core::resources::walls::impl$7::fmt;
  local_78 = (undefined **)&DAT_142c2b648;
  local_70 = (code *)0x2;
  local_58 = (undefined1 *)0x0;
  local_68 = &local_d0;
  local_60 = (code *)0x2;
  local_d0 = &local_98;
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}

