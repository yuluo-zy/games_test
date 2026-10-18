
// WARNING: Control flow encountered bad instruction data
// WARNING: Instruction at (ram,0x0001416ec5da) overlaps instruction at (ram,0x0001416ec5d9)
// 
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl
country_core::systems::wall::valid_enclosure::_::impl$0::
deserialize<enum2_<serde_json::value::Value>_>(void)

{
  undefined4 uVar1;
  undefined8 uVar2;
  code *pcVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  longlong lVar6;
  longlong lVar7;
  char cVar8;
  int iVar9;
  int iVar10;
  ulonglong uVar11;
  longlong *plVar12;
  longlong extraout_RAX;
  ulonglong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  byte bVar13;
  uint uVar14;
  longlong *in_RCX;
  undefined *puVar15;
  byte *in_RDX;
  longlong lVar16;
  longlong unaff_RBX;
  undefined1 *puVar17;
  undefined1 *puVar18;
  undefined1 *puVar19;
  undefined1 *puVar20;
  undefined1 *puVar21;
  undefined4 *unaff_RDI;
  ulonglong in_R9;
  longlong unaff_GS_OFFSET;
  bool bVar22;
  undefined1 auStack_148 [48];
  undefined1 *puStack_118;
  undefined1 auStack_108 [32];
  undefined **ppuStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  ulonglong uStack_a8;
  undefined8 uStack_a0;
  byte bStack_98;
  undefined6 uStack_97;
  undefined1 uStack_91;
  char cStack_90;
  undefined6 uStack_8f;
  undefined1 uStack_89;
  undefined1 local_88;
  undefined6 uStack_87;
  undefined1 uStack_81;
  undefined7 uStack_80;
  undefined1 uStack_79;
  char cStack_78;
  undefined4 uStack_77;
  undefined3 uStack_73;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  char cStack_58;
  byte bStack_57;
  undefined2 uStack_56;
  int iStack_54;
  char cStack_50;
  byte bStack_4f;
  undefined2 uStack_4e;
  int iStack_4c;
  undefined1 uStack_48;
  undefined7 uStack_47;
  undefined1 uStack_40;
  undefined7 uStack_3f;
  undefined8 uStack_38;
  longlong *plStack_30;
  longlong *local_28;
  
  puVar20 = auStack_108;
  puVar21 = auStack_108;
  puVar19 = auStack_108;
  puVar18 = auStack_108;
  puVar17 = auStack_108;
  local_28 = (longlong *)0xfffffffffffffffe;
  puVar15 = &DAT_142c8fe00;
  uVar11 = (ulonglong)*(int *)(&DAT_142c8fe00 + (ulonglong)*in_RDX * 4);
  plVar12 = (longlong *)(&DAT_142c8fe00 + uVar11);
  bVar22 = plVar12 == (longlong *)0x0;
  cVar8 = (char)plVar12;
  uVar14 = (uint)plVar12;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(*in_RDX) {
  case 0:
    cVar8 = '\x12';
    break;
  case 1:
    bStack_4f = in_RDX[1];
    cVar8 = '\0';
    break;
  case 2:
    uStack_48 = (undefined1)*(undefined8 *)(in_RDX + 0x10);
    uStack_47 = (undefined7)((ulonglong)*(undefined8 *)(in_RDX + 0x10) >> 8);
    cVar8 = (char)(0xa0804 >> ((byte)(*(int *)(in_RDX + 8) << 3) & 0x1f));
    break;
  case 3:
    uVar2 = *(undefined8 *)(in_RDX + 8);
    uVar4 = *(undefined8 *)(in_RDX + 0x10);
    uStack_91 = (undefined1)uVar2;
    cStack_90 = (char)((ulonglong)uVar2 >> 8);
    uStack_8f = (undefined6)((ulonglong)uVar2 >> 0x10);
    uStack_89 = (undefined1)uVar4;
    local_88 = (undefined1)((ulonglong)uVar4 >> 8);
    uStack_87 = (undefined6)((ulonglong)uVar4 >> 0x10);
    uStack_38 = *(undefined8 *)(in_RDX + 0x18);
    uStack_81 = (undefined1)uStack_38;
    uStack_80 = (undefined7)((ulonglong)uStack_38 >> 8);
    uStack_47 = (undefined7)((ulonglong)uVar2 >> 8);
    bStack_4f = bStack_98;
    uStack_4e = (undefined2)uStack_97;
    iStack_4c = (int)((uint6)uStack_97 >> 0x10);
    uStack_48 = uStack_91;
    uStack_3f = (undefined7)((ulonglong)uVar4 >> 8);
    cVar8 = '\f';
    uStack_40 = uStack_89;
    break;
  case 4:
    uStack_a8 = *(ulonglong *)(in_RDX + 0x18);
    uStack_b8 = *(undefined8 *)(in_RDX + 8);
    uStack_b0 = *(undefined8 *)(in_RDX + 0x10);
    serde_json::value::de::visit_array<serde::__private::de::content::ContentVisitor>();
    cVar8 = cStack_50;
    if (cStack_50 != '\x16') break;
code_r0x0001416ebbc2:
    lVar16 = CONCAT71(uStack_47,uStack_48);
    goto code_r0x0001416ebda3;
  case 5:
    uStack_c8 = *(undefined8 *)(in_RDX + 0x18);
    uStack_d8 = *(undefined8 *)(in_RDX + 8);
    uStack_d0 = *(undefined8 *)(in_RDX + 0x10);
    serde_json::value::de::visit_object<serde::__private::de::content::ContentVisitor>();
    cVar8 = cStack_50;
    if (cStack_50 == '\x16') goto code_r0x0001416ebbc2;
    break;
  case 6:
    LOCK();
    *(uint *)(unaff_RBX + 0x4c000000) = uVar14 | 0xf;
    UNLOCK();
    puVar15 = (undefined *)(longlong)*(int *)(in_R9 + 0x10b23f860);
    goto code_r0x0001416ebec1;
  case 7:
    puVar17 = &stack0xfffffffffffffef0;
    if (uVar11 < 0xfffffffebd370200) {
      uVar1 = in((short)in_RDX);
      *unaff_RDI = uVar1;
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    }
    goto LAB_1416ebf39;
  default:
    if ((in_R9 ^ *(ulonglong *)(in_RDX + unaff_GS_OFFSET + 6) | 0x142c8fe00) == 0) {
      (&UNK_142c8fe01)[uVar11] = 2;
code_r0x0001416ebf2f:
      cVar8 = '\0';
      goto LAB_1416ebf31;
    }
    goto LAB_1416ebf39;
  case 0x10:
                    // WARNING (jumptable): Read-only address (ram,0x000142c8fe63) is written
                    // WARNING: Read-only address (ram,0x000142c8fe63) is written
    _UNK_142c8fe63 = 0xe88b5afe;
code_r0x0001416ebec1:
                    // WARNING: Could not recover jumptable at 0x0001416ebec4. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(puVar15 + in_R9))(puVar15 + in_R9);
    return;
  case 0x12:
    *(char *)(unaff_GS_OFFSET + (longlong)plVar12) =
         *(char *)(unaff_GS_OFFSET + (longlong)plVar12) + cVar8;
    puVar17 = auStack_108;
    if ((ushort)(in_RDX[2] ^ 0x73 | 0xfe00) == 0) {
      (&UNK_142c8fe01)[uVar11] = 1;
      goto code_r0x0001416ebf2f;
    }
LAB_1416ebf39:
    *(undefined8 *)(puVar17 + -8) = 0x1416ebf57;
    serde::de::Error::unknown_variant<serde_json::error::Error>();
    uVar11 = extraout_RAX_00;
code_r0x0001416ebf5a:
    plVar12[1] = uVar11;
    cVar8 = '\x01';
LAB_1416ebf31:
    *(char *)plVar12 = cVar8;
    return;
  case 0x13:
    uVar11 = (ulonglong)plVar12 & 0xffffffff;
    plVar12 = in_RCX;
    goto code_r0x0001416ebf5a;
  case 0x14:
    in_RDX = (byte *)CONCAT62((int6)((ulonglong)in_RDX >> 0x10),(ushort)(byte)in_RDX);
    *(char *)plVar12 = (char)*plVar12 + cVar8;
    puVar19 = auStack_108;
    if ((in_RDX[5] == 100) && (puVar19 = auStack_108, in_RDX[6] == 0x65)) {
      bVar22 = in_RDX[7] == 0x53;
      goto code_r0x0001416ec00a;
    }
    goto LAB_1416ec0a8;
  case 0x15:
  case 0x16:
  case 0x17:
  case 0x18:
  case 0x19:
  case 0x1a:
  case 0x1b:
  case 0x1c:
  case 0x1e:
                    // WARNING (jumptable): Read-only address (ram,0x000142c8fe75) is written
                    // WARNING: Read-only address (ram,0x000142c8fe75) is written
    _UNK_142c8fe75 = (int)auStack_108 + 0x5cfea5c1;
    puVar19 = auStack_108;
    if ((((((in_RDX[2] != 0x6c) || (puVar19 = auStack_108, in_RDX[3] != 0x69)) ||
          (puVar19 = auStack_108, in_RDX[4] != 100)) ||
         ((puVar19 = auStack_108, in_RDX[5] != 0x45 || (puVar19 = auStack_108, in_RDX[6] != 0x6e))))
        || ((puVar19 = auStack_108, in_RDX[7] != 99 ||
            ((puVar19 = auStack_108, in_RDX[8] != 0x6c || (puVar19 = auStack_108, in_RDX[9] != 0x6f)
             ))))) ||
       ((puVar19 = auStack_108, in_RDX[10] != 0x73 ||
        ((puVar19 = auStack_108, in_RDX[0xb] != 0x75 || (puVar19 = auStack_108, in_RDX[0xc] != 0x72)
         ))))) goto LAB_1416ec0a8;
    bVar13 = in_RDX[0xd];
    puVar18 = auStack_108;
joined_r0x0001416ec0a6:
    puVar19 = puVar18;
    if (bVar13 == 0x65) {
                    // WARNING: Read-only address (ram,0x000142c8fdb9) is written
                    // WARNING: Read-only address (ram,0x000142c8fe63) is written
                    // WARNING: Read-only address (ram,0x000142c8fe75) is written
      *(undefined2 *)in_RCX = 0x200;
      return;
    }
    goto LAB_1416ec0a8;
  case 0x1d:
    UNK_142c8fdb9 = 0xac;
    _UNK_142c8fe63 = 0xa5c25cfe;
    _UNK_142c8fe75 = 0x5cfea5c1;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x1f:
    puVar18 = &stack0xfffffffffffffef0;
code_r0x0001416ec00a:
    puVar19 = puVar18;
    if ((((bVar22) && (in_RDX[8] == 0x68)) && (in_RDX[9] == 0x61)) && (in_RDX[10] == 0x70)) {
      bVar13 = in_RDX[0xb];
      goto joined_r0x0001416ec0a6;
    }
LAB_1416ec0a8:
    *(undefined8 *)(puVar19 + -8) = 0x1416ec0b1;
    serde::__private::string::from_utf8_lossy();
    cStack_90 = (char)uStack_a0;
    uStack_8f = (undefined6)((ulonglong)uStack_a0 >> 8);
    uStack_89 = (undefined1)((ulonglong)uStack_a0 >> 0x38);
    *(undefined8 *)(puVar19 + -8) = 0x1416ec0cf;
    serde::de::Error::unknown_variant<serde_json::error::Error>();
    in_RCX[1] = extraout_RAX_01;
    *(char *)in_RCX = '\x01';
    if ((uStack_a8 & 0x7fffffffffffffff) != 0) {
code_r0x0001416ec0f1:
      *(undefined8 *)(puVar19 + -8) = 0x1416ec0f6;
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
                    // WARNING: Read-only address (ram,0x000142c8fdb9) is written
                    // WARNING: Read-only address (ram,0x000142c8fe63) is written
                    // WARNING: Read-only address (ram,0x000142c8fe75) is written
    return;
  case 0x20:
    (&UNK_142c8fd8b)[uVar11] = (&UNK_142c8fd8b)[uVar11];
    goto code_r0x0001416ec0f1;
  case 0x21:
    (&UNK_142c8fd8b)[uVar11] = (&UNK_142c8fd8b)[uVar11];
    goto code_r0x0001416ec120;
  case 0x22:
    puVar20 = auStack_148;
    plVar12 = (longlong *)(*(longlong *)(in_RDX + 0x20) * 2);
    puStack_118 = &local_88;
    goto code_r0x0001416ec11b;
  case 0x23:
    *(char *)plVar12 = (char)*plVar12 + cVar8;
code_r0x0001416ec11b:
    puVar21 = puVar20;
    if (plVar12 != (longlong *)0x0) {
code_r0x0001416ec120:
      *(undefined8 *)(puVar21 + -8) = 0x1416ec12f;
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    return;
  case 0x24:
    *(uint *)plVar12 = (int)*plVar12 + uVar14;
    *(char *)plVar12 = (char)*plVar12 + cVar8;
    goto LAB_1416ec5cb;
  case 0x25:
    if (SCARRY8(uVar11,0x142c8fe00)) {
      if (!bVar22) goto code_r0x0001416ec67b;
      if ((longlong)uVar11 < -0x142c8fe00) {
        pcVar3 = (code *)swi(3);
        (*pcVar3)();
        return;
      }
      (&UNK_142c8fd8b)[uVar11] = (&UNK_142c8fd8b)[uVar11];
      (&UNK_142c8fd89)[uVar11] = (&UNK_142c8fd89)[uVar11];
      bVar13 = 0xff;
code_r0x0001416ec69c:
      cStack_78 = (char)(0xa0804 >> (bVar13 & 0x1f));
      goto code_r0x0001416ec70a;
    }
    in_RCX = (longlong *)&DAT_142c8fe00;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
    switch(*in_RDX) {
    case 0:
      cStack_78 = '\x12';
      break;
    case 1:
      bStack_57 = in_RDX[1];
      cStack_78 = '\0';
      break;
    case 2:
      uVar2 = *(undefined8 *)(in_RDX + 0x10);
      cStack_50 = (char)uVar2;
      bStack_4f = (byte)((ulonglong)uVar2 >> 8);
      uStack_4e = (undefined2)((ulonglong)uVar2 >> 0x10);
      iStack_4c = (int)((ulonglong)uVar2 >> 0x20);
      bVar13 = (byte)(*(int *)(in_RDX + 8) << 3);
      goto code_r0x0001416ec69c;
    case 3:
      uVar4 = *(undefined8 *)(in_RDX + 8);
      uVar5 = *(undefined8 *)(in_RDX + 0x10);
      cStack_90 = (char)uVar4;
      uStack_8f = (undefined6)((ulonglong)uVar4 >> 8);
      uStack_89 = (undefined1)((ulonglong)uVar4 >> 0x38);
      local_88 = (undefined1)uVar5;
      uStack_87 = (undefined6)((ulonglong)uVar5 >> 8);
      uStack_81 = (undefined1)((ulonglong)uVar5 >> 0x38);
      uVar2 = *(undefined8 *)(in_RDX + 0x18);
      uStack_80 = (undefined7)uVar2;
      uStack_79 = (undefined1)((ulonglong)uVar2 >> 0x38);
      bStack_57 = (byte)uStack_97;
      uStack_56 = (undefined2)((uint6)uStack_97 >> 8);
      iStack_54 = (int)(CONCAT16(uStack_91,uStack_97) >> 0x18);
      cStack_50 = cStack_90;
      bStack_4f = (byte)((ulonglong)uVar4 >> 8);
      uStack_4e = (undefined2)((ulonglong)uVar4 >> 0x10);
      iStack_4c = (int)((ulonglong)uVar4 >> 0x20);
      uStack_47 = (undefined7)((ulonglong)uVar5 >> 8);
      uStack_40 = (undefined1)uVar2;
      uStack_3f = (undefined7)((ulonglong)uVar2 >> 8);
      cStack_78 = '\f';
      uStack_48 = local_88;
      break;
    case 4:
      uStack_a8 = *(ulonglong *)(in_RDX + 0x18);
      uStack_b8 = *(undefined8 *)(in_RDX + 8);
      uStack_b0 = *(undefined8 *)(in_RDX + 0x10);
code_r0x0001416ec67b:
      serde_json::value::de::visit_array<serde::__private::de::content::ContentVisitor>();
      cStack_78 = cStack_58;
      if (cStack_58 != '\x16') break;
code_r0x0001416ec701:
      lVar16 = CONCAT44(iStack_4c,CONCAT22(uStack_4e,CONCAT11(bStack_4f,cStack_50)));
      goto code_r0x0001416ec8e1;
    case 5:
      uStack_c8 = *(undefined8 *)(in_RDX + 0x18);
      uStack_d8 = *(undefined8 *)(in_RDX + 8);
      uStack_d0 = *(undefined8 *)(in_RDX + 0x10);
      serde_json::value::de::visit_object<serde::__private::de::content::ContentVisitor>();
      cStack_78 = cStack_58;
      if (cStack_58 == '\x16') goto code_r0x0001416ec701;
    }
code_r0x0001416ec70a:
    uStack_77 = CONCAT13((undefined1)iStack_54,CONCAT21(uStack_56,bStack_57));
    uStack_73 = (undefined3)((uint)iStack_54 >> 8);
    uStack_70 = CONCAT44(iStack_4c,CONCAT22(uStack_4e,CONCAT11(bStack_4f,cStack_50)));
    uStack_68 = CONCAT71(uStack_47,uStack_48);
    uStack_60 = CONCAT71(uStack_3f,uStack_40);
    ppuStack_e8 = &anon_8317efdc76ced74d096183bd3b10db80_113_llvm_10055580569595727681;
    uStack_e0 = 2;
    serde::__private::de::content::impl$21::
    deserialize_enum<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl_0::deserialize::__Visitor>
              ();
    iVar10 = iStack_54;
    if (CONCAT22(uStack_56,CONCAT11(bStack_57,cStack_58)) == 1) {
      local_28 = (longlong *)CONCAT44(iStack_4c,CONCAT22(uStack_4e,CONCAT11(bStack_4f,cStack_50)));
      if (*local_28 == 1) {
        core::ptr::drop_in_place<std::io::error::Error>();
      }
      else if ((*local_28 == 0) && (local_28[2] != 0)) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      std::alloc::__default_lib_allocator::__rust_dealloc();
      ppuStack_e8 = &anon_8317efdc76ced74d096183bd3b10db80_113_llvm_10055580569595727681;
      uStack_e0 = 2;
      serde::__private::de::content::impl$21::
      deserialize_enum<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl_0::deserialize::__Visitor>
                ();
      if (CONCAT22(uStack_56,CONCAT11(bStack_57,cStack_58)) == 2) {
        local_28 = (longlong *)CONCAT44(iStack_4c,CONCAT22(uStack_4e,CONCAT11(bStack_4f,cStack_50)))
        ;
        if (*local_28 == 1) {
          core::ptr::drop_in_place<std::io::error::Error>();
        }
        else if ((*local_28 == 0) && (local_28[2] != 0)) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        std::alloc::__default_lib_allocator::__rust_dealloc();
        serde_json::error::impl$8::custom<ref_<str_>_>();
        core::ptr::drop_in_place<enum2_<serde::__private::de::content::Content>_>();
        lVar16 = extraout_RAX_02;
code_r0x0001416ec8e1:
        in_RCX[1] = lVar16;
        iVar10 = 1;
        goto code_r0x0001416ec8ea;
      }
      lVar16 = CONCAT44(iStack_54,CONCAT22(uStack_56,CONCAT11(bStack_57,cStack_58)));
      core::ptr::drop_in_place<enum2_<serde::__private::de::content::Content>_>();
code_r0x0001416ec880:
      if ((int)lVar16 != 0) {
        cStack_58 = 0xb0;
        bStack_57 = 0x99;
        uStack_56 = 0x42c1;
        iStack_54 = 1;
        cStack_50 = 1;
        bStack_4f = 0;
        uStack_4e = 0;
        iStack_4c = 0;
        uStack_48 = 8;
        uStack_47 = 0;
        uStack_40 = 0;
        uStack_3f = 0;
        uStack_38 = 0;
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
      ::utils::geometry::circle::Circle2d::new();
code_r0x0001416ec89f:
      iVar10 = 0;
    }
    else {
      lVar16 = CONCAT44(iStack_4c,CONCAT22(uStack_4e,CONCAT11(bStack_4f,cStack_50)));
      lVar6 = CONCAT71(uStack_47,uStack_48);
      lVar7 = CONCAT71(uStack_3f,uStack_40);
      core::ptr::drop_in_place<enum2_<serde::__private::de::content::Content>_>();
      if (iVar10 == 2) goto code_r0x0001416ec880;
      in_RCX[1] = lVar16;
      in_RCX[2] = lVar6;
      if (iVar10 == 0) goto code_r0x0001416ec89f;
      in_RCX[3] = lVar7;
      iVar10 = 1;
    }
    *(int *)((longlong)in_RCX + 4) = iVar10;
    iVar10 = 0;
code_r0x0001416ec8ea:
    *(int *)in_RCX = iVar10;
    return;
  case 0x26:
    *(char *)plVar12 = (char)*plVar12 + cVar8;
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    lVar16 = *(longlong *)(unaff_RDI + 4);
    *(undefined1 *)(*(longlong *)(unaff_RDI + 2) + lVar16) = 0x7d;
    *(longlong *)(unaff_RDI + 4) = lVar16 + 1;
    in_RCX = (longlong *)*in_RCX;
    lVar16 = in_RCX[2];
    if (*in_RCX != lVar16) goto LAB_1416ec385;
LAB_1416ec5cb:
    ppuStack_e8 = (undefined **)0x1;
code_r0x0001416ec5e0:
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    lVar16 = in_RCX[2];
LAB_1416ec385:
    *(undefined1 *)(in_RCX[1] + lVar16) = 0x7d;
    in_RCX[2] = lVar16 + 1;
    return;
  case 0x27:
    UNK_142c8fdb9 = 0xac;
    _UNK_142c8fe63 = 0xa5c25cfe;
    _UNK_142c8fe75 = 0x5cfea5c1;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x28:
    *(char *)plVar12 = (char)*plVar12 + cVar8;
                    // WARNING (jumptable): Read-only address (ram,0x000142c8fdb9) is written
                    // WARNING: Read-only address (ram,0x000142c8fdb9) is written
    UNK_142c8fdb9 = cVar8 + -0x54;
    *(uint *)plVar12 = (int)*plVar12 + uVar14;
    *(char *)plVar12 = (char)*plVar12 + cVar8;
    goto code_r0x0001416ec5e0;
  case 0x29:
    UNK_142c8fdb9 = 0xac;
    _UNK_142c8fe63 = 0xa5c25cfe;
    _UNK_142c8fe75 = 0x5cfea5c1;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  }
  uStack_70._5_3_ = (undefined3)((uint)iStack_4c >> 8);
  uStack_68 = CONCAT71(uStack_47,uStack_48);
  uStack_60 = CONCAT71(uStack_3f,uStack_40);
  cStack_58 = (char)uStack_38;
  bStack_57 = (byte)((ulonglong)uStack_38 >> 8);
  uStack_56 = (undefined2)((ulonglong)uStack_38 >> 0x10);
  iStack_54 = (int)((ulonglong)uStack_38 >> 0x20);
  uStack_70 = CONCAT71(CONCAT34(uStack_70._5_3_,
                                CONCAT13((undefined1)iStack_4c,CONCAT21(uStack_4e,bStack_4f))),cVar8
                      );
  ppuStack_e8 = &anon_8317efdc76ced74d096183bd3b10db80_97_llvm_10055580569595727681;
  uStack_e0 = 3;
  serde::__private::de::content::impl$21::
  deserialize_enum<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl_0::deserialize::__Visitor>
            ();
  iVar10 = iStack_4c;
  if (CONCAT22(uStack_4e,CONCAT11(bStack_4f,cStack_50)) == 1) {
    plStack_30 = (longlong *)CONCAT71(uStack_47,uStack_48);
    if (*plStack_30 == 1) {
      core::ptr::drop_in_place<std::io::error::Error>();
    }
    else if ((*plStack_30 == 0) && (plStack_30[2] != 0)) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    std::alloc::__default_lib_allocator::__rust_dealloc();
    ppuStack_e8 = &anon_8317efdc76ced74d096183bd3b10db80_103_llvm_10055580569595727681;
    uStack_e0 = 2;
    serde::__private::de::content::impl$21::
    deserialize_enum<serde_json::error::Error,country_core::systems::wall::valid_enclosure::_::impl_0::deserialize::__Visitor>
              ();
    iVar10 = iStack_4c;
    if (CONCAT22(uStack_4e,CONCAT11(bStack_4f,cStack_50)) == 1) {
      plStack_30 = (longlong *)CONCAT71(uStack_47,uStack_48);
      if (*plStack_30 == 1) {
        core::ptr::drop_in_place<std::io::error::Error>();
      }
      else if ((*plStack_30 == 0) && (plStack_30[2] != 0)) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      std::alloc::__default_lib_allocator::__rust_dealloc();
      serde_json::error::impl$8::custom<ref_<str_>_>();
      core::ptr::drop_in_place<enum2_<serde::__private::de::content::Content>_>();
      lVar16 = extraout_RAX;
code_r0x0001416ebda3:
      in_RCX[1] = lVar16;
      iVar10 = 1;
      goto code_r0x0001416ebdac;
    }
    lVar16 = CONCAT71(uStack_47,uStack_48);
    bStack_98 = uStack_40;
    uStack_97 = (undefined6)uStack_3f;
    uStack_91 = (undefined1)((uint7)uStack_3f >> 0x30);
    cStack_90 = (char)uStack_38;
    uStack_8f = (undefined6)((ulonglong)uStack_38 >> 8);
    uStack_89 = (undefined1)((ulonglong)uStack_38 >> 0x38);
    core::ptr::drop_in_place<enum2_<serde::__private::de::content::Content>_>();
    iVar9 = 2;
    if (iVar10 != 2) {
      in_RCX[1] = lVar16;
      in_RCX[2] = CONCAT17(uStack_91,CONCAT61(uStack_97,bStack_98));
      in_RCX[3] = CONCAT17(uStack_89,CONCAT61(uStack_8f,cStack_90));
      iVar9 = iVar10;
    }
  }
  else {
    lVar16 = CONCAT71(uStack_47,uStack_48);
    bStack_98 = uStack_40;
    uStack_97 = (undefined6)uStack_3f;
    uStack_91 = (undefined1)((uint7)uStack_3f >> 0x30);
    cStack_90 = (char)uStack_38;
    uStack_8f = (undefined6)((ulonglong)uStack_38 >> 8);
    uStack_89 = (undefined1)((ulonglong)uStack_38 >> 0x38);
    core::ptr::drop_in_place<enum2_<serde::__private::de::content::Content>_>();
    uVar14 = iVar10 - 2;
    if (1 < uVar14) {
      uVar14 = 2;
    }
    iVar9 = 2;
    if (uVar14 != 0) {
      if (uVar14 == 1) {
        iVar9 = 3;
      }
      else {
        in_RCX[1] = lVar16;
        in_RCX[2] = CONCAT17(uStack_91,CONCAT61(uStack_97,bStack_98));
        in_RCX[3] = CONCAT17(uStack_89,CONCAT61(uStack_8f,cStack_90));
        iVar9 = iVar10;
      }
    }
  }
  *(int *)((longlong)in_RCX + 4) = iVar9;
  iVar10 = 0;
code_r0x0001416ebdac:
  *(int *)in_RCX = iVar10;
  return;
}

