// address 0x14156ebd0
// name update_instance_data<country_core::render::nani::instance_data::GothicWindowBricksInstanceData>
// signature void __cdecl update_instance_data<country_core::render::nani::instance_data::GothicWindowBricksInstanceData>(void)

void __cdecl
country_core::render::nani::gpu_world::OwnedGpuWorldSubset::
update_instance_data<country_core::render::nani::instance_data::GothicWindowBricksInstanceData>
          (void)

{
  char *pcVar1;
  undefined8 *puVar2;
  undefined4 uVar3;
  uint uVar4;
  uint uVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  undefined8 uVar16;
  undefined2 extraout_AX;
  ulonglong extraout_RAX;
  longlong lVar17;
  longlong lVar18;
  ulonglong uVar19;
  longlong in_RCX;
  longlong lVar20;
  ulonglong uVar21;
  int *in_RDX;
  undefined8 *in_R8;
  ulonglong uVar22;
  uint uVar23;
  ushort uVar24;
  ulonglong uVar25;
  bool bVar26;
  byte bVar27;
  char cVar28;
  char cVar30;
  char cVar31;
  char cVar32;
  undefined1 auVar29 [16];
  undefined1 in_XMM1_Ba;
  undefined1 in_XMM1_Bb;
  undefined1 in_XMM1_Bc;
  undefined1 in_XMM1_Bd;
  undefined1 in_XMM1_Be;
  undefined1 in_XMM1_Bf;
  undefined1 in_XMM1_Bg;
  undefined1 in_XMM1_Bh;
  undefined1 in_XMM1_Bi;
  undefined1 in_XMM1_Bj;
  undefined1 in_XMM1_Bk;
  undefined1 in_XMM1_Bl;
  undefined1 in_XMM1_Bm;
  undefined1 in_XMM1_Bn;
  undefined1 in_XMM1_Bo;
  undefined1 in_XMM1_Bp;
  undefined1 auVar33 [16];
  undefined1 auVar34 [16];
  undefined8 *in_stack_00000028;
  undefined *local_c8;
  undefined8 local_c0;
  undefined8 local_b8;
  ulonglong local_b0;
  int *local_a8;
  code *local_a0;
  undefined **local_98;
  code *local_90;
  undefined8 *local_88;
  code *local_80;
  undefined *local_78;
  undefined8 local_70;
  int **local_68;
  undefined8 local_60;
  undefined8 local_58;
  
  uVar22 = (ulonglong)(uint)in_RDX[1];
  if ((uVar22 < *(ulonglong *)(in_RCX + 0x10)) &&
     (*(int *)(*(longlong *)(in_RCX + 8) + uVar22 * 4) == *in_RDX)) {
LAB_14156eccd:
    uVar25 = *(ulonglong *)(in_RCX + 0x48);
    do {
      uVar21 = uVar22;
      if (uVar25 <= uVar22) {
        uVar21 = uVar25;
      }
      LOCK();
      uVar19 = *(ulonglong *)(in_RCX + 0x48);
      bVar26 = uVar25 == uVar19;
      if (bVar26) {
        *(ulonglong *)(in_RCX + 0x48) = uVar21;
        uVar19 = uVar25;
      }
      UNLOCK();
      uVar25 = uVar19;
    } while (!bVar26);
    uVar25 = *(ulonglong *)(in_RCX + 0x50);
    do {
      uVar21 = uVar22;
      if (uVar22 < uVar25) {
        uVar21 = uVar25;
      }
      LOCK();
      uVar19 = *(ulonglong *)(in_RCX + 0x50);
      bVar26 = uVar25 == uVar19;
      if (bVar26) {
        *(ulonglong *)(in_RCX + 0x50) = uVar21;
        uVar19 = uVar25;
      }
      UNLOCK();
      uVar25 = uVar19;
    } while (!bVar26);
    uVar25 = *(ulonglong *)(in_RCX + 0x60);
    if (uVar25 <= uVar22) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    lVar18 = *(longlong *)(in_RCX + 0x58);
    lVar17 = uVar22 * 0x10;
    uVar3 = *(undefined4 *)(in_R8 + 1);
    *(undefined8 *)(lVar18 + lVar17) = *in_R8;
    *(undefined4 *)(lVar18 + 8 + lVar17) = uVar3;
    half::binary16::f16::from_f32();
    *(undefined2 *)(lVar18 + 0xc + lVar17) = extraout_AX;
    lVar18 = (**(code **)(*(longlong *)(in_RCX + 0x98) + 0x28))(*(undefined8 *)(in_RCX + 0x90));
    lVar17 = (**(code **)(uVar25 + 0x18))(lVar18);
    if (((lVar18 != 0) && (lVar17 == -0x6ecec704ae7d393)) && (uVar25 == 0xeb18a26409362d7c)) {
      if (*(ulonglong *)(lVar18 + 0x30) <= uVar22) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      lVar17 = *(longlong *)(lVar18 + 0x28);
      lVar20 = uVar22 * 0x70;
      uVar10 = in_stack_00000028[0xd];
      puVar2 = (undefined8 *)(lVar17 + 0x60 + lVar20);
      *puVar2 = in_stack_00000028[0xc];
      puVar2[1] = uVar10;
      uVar10 = in_stack_00000028[0xb];
      puVar2 = (undefined8 *)(lVar17 + 0x50 + lVar20);
      *puVar2 = in_stack_00000028[10];
      puVar2[1] = uVar10;
      uVar10 = in_stack_00000028[9];
      puVar2 = (undefined8 *)(lVar17 + 0x40 + lVar20);
      *puVar2 = in_stack_00000028[8];
      puVar2[1] = uVar10;
      uVar10 = *in_stack_00000028;
      uVar11 = in_stack_00000028[1];
      uVar12 = in_stack_00000028[2];
      uVar13 = in_stack_00000028[3];
      uVar14 = in_stack_00000028[4];
      uVar15 = in_stack_00000028[5];
      uVar16 = in_stack_00000028[7];
      puVar2 = (undefined8 *)(lVar17 + 0x30 + lVar20);
      *puVar2 = in_stack_00000028[6];
      puVar2[1] = uVar16;
      puVar2 = (undefined8 *)(lVar17 + 0x20 + lVar20);
      *puVar2 = uVar14;
      puVar2[1] = uVar15;
      puVar2 = (undefined8 *)(lVar17 + 0x10 + lVar20);
      *puVar2 = uVar12;
      puVar2[1] = uVar13;
      *(undefined8 *)(lVar17 + lVar20) = uVar10;
      ((undefined8 *)(lVar17 + lVar20))[1] = uVar11;
      uVar25 = *(ulonglong *)(lVar18 + 0x18);
      do {
        uVar21 = uVar22;
        if (uVar25 <= uVar22) {
          uVar21 = uVar25;
        }
        LOCK();
        uVar19 = *(ulonglong *)(lVar18 + 0x18);
        bVar26 = uVar25 == uVar19;
        if (bVar26) {
          *(ulonglong *)(lVar18 + 0x18) = uVar21;
          uVar19 = uVar25;
        }
        UNLOCK();
        uVar25 = uVar19;
      } while (!bVar26);
      uVar25 = *(ulonglong *)(lVar18 + 0x20);
      do {
        uVar21 = uVar22;
        if (uVar22 < uVar25) {
          uVar21 = uVar25;
        }
        LOCK();
        uVar19 = *(ulonglong *)(lVar18 + 0x20);
        bVar26 = uVar25 == uVar19;
        if (bVar26) {
          *(ulonglong *)(lVar18 + 0x20) = uVar21;
          uVar19 = uVar25;
        }
        UNLOCK();
        uVar25 = uVar19;
      } while (!bVar26);
      return;
    }
    local_c8 = &anon_107183261458dea85466985370debbb6_86_llvm_3641509820080252961;
    local_c0 = 0x49;
    local_b8 = (**(code **)(*(longlong *)(in_RCX + 0x98) + 0x18))(*(undefined8 *)(in_RCX + 0x90));
    local_a0 = gpu_world::impl$17::fmt;
    local_98 = &local_c8;
    local_90 = core::fmt::impl$74::fmt<str_>;
    local_88 = &local_b8;
    local_80 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
    local_78 = &anon_1c1a639a7b8c984c3b37b13664f64d34_58_llvm_14765964051527200705;
    local_70 = 3;
    local_58 = 0;
    local_68 = &local_a8;
    local_60 = 3;
    local_b0 = uVar25;
    local_a8 = in_RDX + 2;
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  if (*(longlong *)(in_RCX + 0x30) != 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
    lVar18 = *(longlong *)(in_RCX + 0x18);
    bVar27 = (byte)(extraout_RAX >> 0x39);
    auVar29 = ZEXT216(CONCAT11(bVar27,bVar27));
    auVar29 = pshuflw(auVar29,auVar29,0);
    lVar17 = 0;
    iVar6 = -(uint)(CONCAT13(in_XMM1_Bd,CONCAT12(in_XMM1_Bc,CONCAT11(in_XMM1_Bb,in_XMM1_Ba))) ==
                   CONCAT13(in_XMM1_Bd,CONCAT12(in_XMM1_Bc,CONCAT11(in_XMM1_Bb,in_XMM1_Ba))));
    iVar7 = -(uint)(CONCAT13(in_XMM1_Bh,CONCAT12(in_XMM1_Bg,CONCAT11(in_XMM1_Bf,in_XMM1_Be))) ==
                   CONCAT13(in_XMM1_Bh,CONCAT12(in_XMM1_Bg,CONCAT11(in_XMM1_Bf,in_XMM1_Be))));
    iVar8 = -(uint)(CONCAT13(in_XMM1_Bl,CONCAT12(in_XMM1_Bk,CONCAT11(in_XMM1_Bj,in_XMM1_Bi))) ==
                   CONCAT13(in_XMM1_Bl,CONCAT12(in_XMM1_Bk,CONCAT11(in_XMM1_Bj,in_XMM1_Bi))));
    iVar9 = -(uint)(CONCAT13(in_XMM1_Bp,CONCAT12(in_XMM1_Bo,CONCAT11(in_XMM1_Bn,in_XMM1_Bm))) ==
                   CONCAT13(in_XMM1_Bp,CONCAT12(in_XMM1_Bo,CONCAT11(in_XMM1_Bn,in_XMM1_Bm))));
    uVar22 = extraout_RAX;
    while( true ) {
      uVar22 = *(ulonglong *)(in_RCX + 0x20) & uVar22;
      pcVar1 = (char *)(lVar18 + uVar22);
      cVar28 = auVar29[0];
      auVar34[0] = -(*pcVar1 == cVar28);
      cVar30 = auVar29[1];
      auVar34[1] = -(pcVar1[1] == cVar30);
      cVar31 = auVar29[2];
      auVar34[2] = -(pcVar1[2] == cVar31);
      cVar32 = auVar29[3];
      auVar34[3] = -(pcVar1[3] == cVar32);
      auVar34[4] = -(pcVar1[4] == cVar28);
      auVar34[5] = -(pcVar1[5] == cVar30);
      auVar34[6] = -(pcVar1[6] == cVar31);
      auVar34[7] = -(pcVar1[7] == cVar32);
      auVar34[8] = -(pcVar1[8] == cVar28);
      auVar34[9] = -(pcVar1[9] == cVar30);
      auVar34[10] = -(pcVar1[10] == cVar31);
      auVar34[0xb] = -(pcVar1[0xb] == cVar32);
      auVar34[0xc] = -(pcVar1[0xc] == cVar28);
      auVar34[0xd] = -(pcVar1[0xd] == cVar30);
      auVar34[0xe] = -(pcVar1[0xe] == cVar31);
      auVar34[0xf] = -(pcVar1[0xf] == cVar32);
      uVar24 = (ushort)(SUB161(auVar34 >> 7,0) & 1) | (ushort)(SUB161(auVar34 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar34 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar34 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar34 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar34 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar34 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar34 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar34 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar34 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar34 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar34 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar34 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar34 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar34 >> 0x77,0) & 1) << 0xe | (ushort)(auVar34[0xf] >> 7) << 0xf;
      uVar23 = (uint)uVar24;
      if (uVar24 != 0) {
        do {
          uVar4 = 0;
          for (uVar5 = uVar23; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
            uVar4 = uVar4 + 1;
          }
          uVar25 = uVar4 + uVar22 & *(ulonglong *)(in_RCX + 0x20);
          if (*in_RDX == *(int *)(lVar18 + uVar25 * -8 + -8)) {
            uVar23 = *(uint *)(lVar18 + -4 + uVar25 * -8);
            uVar22 = (ulonglong)uVar23;
            in_RDX[1] = uVar23;
            goto LAB_14156eccd;
          }
          uVar24 = (ushort)(uVar23 - 1) & (ushort)uVar23;
          uVar23 = CONCAT22((short)(uVar23 - 1 >> 0x10),uVar24);
        } while (uVar24 != 0);
      }
      auVar33[0] = -(*pcVar1 == (char)iVar6);
      auVar33[1] = -(pcVar1[1] == (char)((uint)iVar6 >> 8));
      auVar33[2] = -(pcVar1[2] == (char)((uint)iVar6 >> 0x10));
      auVar33[3] = -(pcVar1[3] == (char)((uint)iVar6 >> 0x18));
      auVar33[4] = -(pcVar1[4] == (char)iVar7);
      auVar33[5] = -(pcVar1[5] == (char)((uint)iVar7 >> 8));
      auVar33[6] = -(pcVar1[6] == (char)((uint)iVar7 >> 0x10));
      auVar33[7] = -(pcVar1[7] == (char)((uint)iVar7 >> 0x18));
      auVar33[8] = -(pcVar1[8] == (char)iVar8);
      auVar33[9] = -(pcVar1[9] == (char)((uint)iVar8 >> 8));
      auVar33[10] = -(pcVar1[10] == (char)((uint)iVar8 >> 0x10));
      auVar33[0xb] = -(pcVar1[0xb] == (char)((uint)iVar8 >> 0x18));
      auVar33[0xc] = -(pcVar1[0xc] == (char)iVar9);
      auVar33[0xd] = -(pcVar1[0xd] == (char)((uint)iVar9 >> 8));
      auVar33[0xe] = -(pcVar1[0xe] == (char)((uint)iVar9 >> 0x10));
      auVar33[0xf] = -(pcVar1[0xf] == (char)((uint)iVar9 >> 0x18));
      if ((((((((((((((((SUB161(auVar33 >> 7,0) & 1) != 0 || (SUB161(auVar33 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar33 >> 0x17,0) & 1) != 0) || (SUB161(auVar33 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar33 >> 0x27,0) & 1) != 0) || (SUB161(auVar33 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar33 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar33 >> 0x3f,0) & 1) != 0) || (SUB161(auVar33 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar33 >> 0x4f,0) & 1) != 0) || (SUB161(auVar33 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar33 >> 0x5f,0) & 1) != 0) || (SUB161(auVar33 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar33 >> 0x6f,0) & 1) != 0) || (SUB161(auVar33 >> 0x77,0) & 1) != 0) ||
          auVar33[0xf] < '\0') break;
      uVar22 = uVar22 + lVar17 + 0x10;
      lVar17 = lVar17 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

