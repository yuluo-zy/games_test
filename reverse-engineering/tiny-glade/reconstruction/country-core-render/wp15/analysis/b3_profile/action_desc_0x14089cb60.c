// addr=0x14089cb60 name=action_desc
// signature=void __cdecl action_desc(void)


// WARNING: Instruction at (ram,0x00014089d3d8) overlaps instruction at (ram,0x00014089d3d5)
// 
// WARNING: Control flow encountered bad instruction data
// WARNING: This function may have set the stack pointer
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl country_core::game_key_bindings::KeyBinding::action_desc(void)

{
  byte *pbVar1;
  char *pcVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  byte bVar5;
  undefined4 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  longlong lVar9;
  code *pcVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  void *_Src;
  char cVar13;
  longlong lVar15;
  int *piVar16;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  void *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  undefined8 *puVar17;
  undefined4 *extraout_RAX_03;
  void *extraout_RAX_04;
  void *extraout_RAX_05;
  void *extraout_RAX_06;
  undefined8 *extraout_RAX_07;
  void *extraout_RAX_08;
  void *extraout_RAX_09;
  undefined4 *extraout_RAX_10;
  undefined4 *extraout_RAX_11;
  undefined4 *puVar18;
  void *extraout_RAX_12;
  void *extraout_RAX_13;
  undefined8 *extraout_RAX_14;
  undefined8 *extraout_RAX_15;
  void *extraout_RAX_16;
  undefined8 extraout_RAX_17;
  undefined8 uVar19;
  undefined8 extraout_RAX_18;
  undefined8 extraout_RAX_19;
  ulonglong extraout_RAX_20;
  longlong lVar20;
  size_t *in_RCX;
  undefined *puVar21;
  byte *in_RDX;
  size_t _Size;
  undefined8 unaff_RBX;
  longlong unaff_RBP;
  longlong lVar22;
  byte *unaff_RDI;
  undefined8 *in_R8;
  undefined8 in_R9;
  void *_Dst;
  longlong *plVar23;
  undefined4 in_XMM0_Da;
  undefined4 in_XMM0_Db;
  undefined4 in_XMM0_Dc;
  undefined4 in_XMM0_Dd;
  undefined4 uVar24;
  undefined1 auStack_b8 [32];
  undefined8 uStack_98;
  undefined1 *puStack_90;
  code *pcStack_88;
  undefined1 auStack_80 [8];
  undefined8 uStack_78;
  size_t sStack_70;
  size_t sStack_68;
  size_t sStack_60;
  undefined *puStack_58;
  undefined8 uStack_50;
  undefined1 **ppuStack_48;
  undefined8 uStack_40;
  undefined8 uStack_38;
  uint uVar14;
  
  puVar21 = &DAT_142a9aa98;
  lVar15 = (longlong)*(int *)(&DAT_142a9aa98 + (ulonglong)*in_RDX * 4);
  piVar16 = (int *)(&DAT_142a9aa98 + lVar15);
  cVar13 = (char)piVar16;
  uVar14 = (uint)piVar16;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(*in_RDX) {
  case 0:
    goto code_r0x00014089d03f;
  case 1:
    _Size = 0x14;
    lang_lib::lookup();
    _Src = extraout_RAX_06;
    goto joined_r0x00014089d188;
  case 2:
    uVar19 = 0x10;
    goto code_r0x00014089cf73;
  case 3:
    uVar19 = 0x10;
    goto code_r0x00014089ce1b;
  case 4:
    uVar19 = 0x10;
    lang_lib::lookup();
    puStack_58 = &UNK_142a990e8;
    uStack_78 = uVar19;
    goto code_r0x00014089cf9f;
  case 5:
    uVar19 = 0x10;
    lang_lib::lookup();
    puStack_58 = &UNK_142a99110;
    uStack_78 = uVar19;
    goto code_r0x00014089cf9f;
  case 6:
    uVar19 = 0x12;
code_r0x00014089cf73:
    lang_lib::lookup();
    puStack_58 = &UNK_142a99098;
    uStack_78 = uVar19;
    goto code_r0x00014089cf9f;
  case 7:
    uVar19 = 0x12;
code_r0x00014089ce1b:
    lang_lib::lookup();
    puStack_58 = &UNK_142a990c0;
    uStack_78 = uVar19;
code_r0x00014089cf9f:
    puStack_90 = auStack_80;
    pcStack_88 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
    uStack_50 = 2;
    ppuStack_48 = &puStack_90;
    uStack_40 = 1;
    uStack_38 = 0;
    alloc::fmt::format::format_inner();
    in_RCX[2] = sStack_60;
    *in_RCX = sStack_70;
    in_RCX[1] = sStack_68;
    goto code_r0x00014089d0a1;
  case 8:
code_r0x00014089d03f:
    _Size = 0x12;
    lang_lib::lookup();
    _Src = extraout_RAX_08;
    goto joined_r0x00014089d188;
  case 9:
    _Size = 0x13;
    lang_lib::lookup();
    _Src = extraout_RAX_04;
    goto joined_r0x00014089d188;
  case 10:
    *in_RCX = 0x8000000000000000;
    in_RCX[3] = (size_t)&UNK_142a99155;
    in_RCX[4] = 0x10;
    return;
  case 0xb:
    *in_RCX = 0x8000000000000000;
    puVar21 = &UNK_142a99165;
    goto code_r0x00014089cec6;
  case 0xc:
    *in_RCX = 0x8000000000000000;
    in_RCX[3] = (size_t)&UNK_142a99171;
    in_RCX[4] = 0x12;
    return;
  case 0xd:
    *in_RCX = 0x8000000000000000;
    in_RCX[3] = (size_t)&UNK_142a99183;
    in_RCX[4] = 0x15;
    return;
  case 0xe:
    *in_RCX = 0x8000000000000000;
    puVar21 = &UNK_142a99198;
    goto code_r0x00014089ccae;
  case 0xf:
    *in_RCX = 0x8000000000000000;
    in_RCX[3] = (size_t)&UNK_142a991a6;
    in_RCX[4] = 0xf;
    return;
  case 0x10:
    *in_RCX = 0x8000000000000000;
    puVar21 = &UNK_142a991b5;
code_r0x00014089ccae:
    in_RCX[3] = (size_t)puVar21;
    in_RCX[4] = 0xe;
    return;
  case 0x11:
    *in_RCX = 0x8000000000000000;
    puVar21 = &UNK_142a991c3;
code_r0x00014089cec6:
    in_RCX[3] = (size_t)puVar21;
    in_RCX[4] = 0xc;
    return;
  case 0x12:
    *in_RCX = 0x8000000000000000;
    in_RCX[3] = (size_t)&UNK_142a991cf;
    in_RCX[4] = 0xd;
    return;
  case 0x13:
    *in_RCX = 0x8000000000000000;
    in_RCX[3] = (size_t)&UNK_142a991dc;
    in_RCX[4] = 0x11;
    return;
  case 0x14:
    *in_RCX = 0x8000000000000000;
    in_RCX[3] = (size_t)&UNK_142a991ed;
    in_RCX[4] = 0x13;
    return;
  case 0x15:
    goto code_r0x00014089d134;
  case 0x16:
code_r0x00014089d134:
    _Size = 0x10;
    lang_lib::lookup();
    _Src = extraout_RAX_12;
    goto joined_r0x00014089d188;
  case 0x17:
    goto code_r0x00014089d178;
  case 0x18:
code_r0x00014089d178:
    _Size = 0x1b;
    lang_lib::lookup();
    _Src = extraout_RAX_13;
    goto joined_r0x00014089d188;
  case 0x19:
    _Size = 0x1a;
    lang_lib::lookup();
    _Src = extraout_RAX_05;
    goto joined_r0x00014089d188;
  case 0x1a:
    _Size = 0x18;
    lang_lib::lookup();
    _Src = extraout_RAX_16;
    goto joined_r0x00014089d188;
  case 0x1b:
    _Size = 0x1d;
    lang_lib::lookup();
    _Src = extraout_RAX_01;
joined_r0x00014089d188:
    if (-1 < (longlong)_Size) {
      if (_Size == 0) {
        _Dst = (void *)0x1;
      }
      else {
        std::alloc::__default_lib_allocator::__rust_alloc();
        _Dst = extraout_RAX_09;
        if (extraout_RAX_09 == (void *)0x0) goto code_r0x00014089d191;
      }
      memcpy(_Dst,_Src,_Size);
      *in_RCX = _Size;
      in_RCX[1] = (size_t)_Dst;
      in_RCX[2] = _Size;
code_r0x00014089d0a1:
      in_RCX[3] = 0;
                    // WARNING: Read-only address (ram,0x000142a9aa19) is written
                    // WARNING: Read-only address (ram,0x000142a9aa1b) is written
                    // WARNING: Read-only address (ram,0x000142a9aa98) is written
      return;
    }
code_r0x00014089d191:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  case 0x1c:
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_14 == (undefined8 *)0x0) goto code_r0x00014089d2cc;
    *extraout_RAX_14 = 0x65636e6143204955;
    *(undefined1 *)(extraout_RAX_14 + 1) = 0x6c;
    *in_RCX = 9;
    in_RCX[1] = (size_t)extraout_RAX_14;
    in_RCX[2] = 9;
    goto code_r0x00014089d0a1;
  case 0x1d:
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_15 != (undefined8 *)0x0) {
      *extraout_RAX_15 = 0x69666e6f43204955;
      *(undefined2 *)(extraout_RAX_15 + 1) = 0x6d72;
      *in_RCX = 10;
      in_RCX[1] = (size_t)extraout_RAX_15;
      in_RCX[2] = 10;
      goto code_r0x00014089d0a1;
    }
    goto code_r0x00014089d2e2;
  case 0x1e:
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_10 == (undefined4 *)0x0) {
code_r0x00014089d26d:
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    *(undefined4 *)((longlong)extraout_RAX_10 + 3) = 0x7466654c;
    *extraout_RAX_10 = 0x4c204955;
    puVar18 = extraout_RAX_10;
    goto code_r0x00014089d115;
  case 0x1f:
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_07 != (undefined8 *)0x0) {
      *extraout_RAX_07 = 0x7468676952204955;
      *in_RCX = 8;
      in_RCX[1] = (size_t)extraout_RAX_07;
      in_RCX[2] = 8;
      goto code_r0x00014089d0a1;
    }
  case 0x47:
code_r0x00014089d2bb:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  case 0x20:
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_11 == (undefined4 *)0x0) goto code_r0x00014089d26d;
    *(undefined4 *)((longlong)extraout_RAX_11 + 3) = 0x6e776f44;
    *extraout_RAX_11 = 0x44204955;
    puVar18 = extraout_RAX_11;
code_r0x00014089d115:
    *in_RCX = 7;
    in_RCX[1] = (size_t)puVar18;
    in_RCX[2] = 7;
    goto code_r0x00014089d0a1;
  case 0x21:
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_03 != (undefined4 *)0x0) {
      *(undefined1 *)(extraout_RAX_03 + 1) = 0x70;
      *extraout_RAX_03 = 0x55204955;
      *in_RCX = 5;
      in_RCX[1] = (size_t)extraout_RAX_03;
      in_RCX[2] = 5;
      goto code_r0x00014089d0a1;
    }
    goto code_r0x00014089d299;
  case 0x22:
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_02 == (undefined8 *)0x0) {
code_r0x00014089d257:
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    *extraout_RAX_02 = 0x5020626154204955;
    *(undefined4 *)((longlong)extraout_RAX_02 + 7) = 0x76657250;
    puVar17 = extraout_RAX_02;
    goto code_r0x00014089cc6c;
  case 0x23:
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (undefined8 *)0x0) goto code_r0x00014089d257;
    *extraout_RAX = 0x4e20626154204955;
    *(undefined4 *)((longlong)extraout_RAX + 7) = 0x7478654e;
    puVar17 = extraout_RAX;
code_r0x00014089cc6c:
    *in_RCX = 0xb;
    in_RCX[1] = (size_t)puVar17;
    in_RCX[2] = 0xb;
    goto code_r0x00014089d0a1;
  case 0x24:
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_00 != (undefined8 *)0x0) {
      *extraout_RAX_00 = 0x6143202950495728;
      extraout_RAX_00[1] = 0x616e73206172656d;
      *(undefined8 *)((longlong)extraout_RAX_00 + 0xe) = 0x73746f687370616e;
      *in_RCX = 0x16;
      in_RCX[1] = (size_t)extraout_RAX_00;
      in_RCX[2] = 0x16;
      goto code_r0x00014089d0a1;
    }
    goto code_r0x00014089d28f;
  case 0x25:
    UNK_142a9aa19 = 0x1b;
    UNK_142a9aa1b = 0xfd;
    _DAT_142a9aa98 = 0xfde020ec;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x26:
    *(char *)(lVar15 + 0x2d13c5530) = *(char *)(lVar15 + 0x2d13c5530) << 1;
    *(char *)piVar16 = (char)*piVar16 + cVar13;
code_r0x00014089d32b:
    in_R8 = in_R8 + 0x36;
    break;
  case 0x27:
    goto code_r0x00014089d336;
  case 0x28:
    pcVar10 = (code *)swi(3);
    (*pcVar10)();
    return;
  case 0x29:
    *(char *)piVar16 = (char)*piVar16 + cVar13;
code_r0x00014089d3e8:
    in_R8 = (undefined8 *)((longlong)in_R8 + 0xcc);
    break;
  case 0x2a:
    pbVar1 = (byte *)(lVar15 + 0x2893c5530);
    *pbVar1 = *pbVar1 >> 1 | *pbVar1 << 7;
    goto code_r0x00014089d371;
  case 0x2b:
    (&DAT_142a9aa98)[unaff_RBP * 8] = 0;
    *(char *)piVar16 = (char)*piVar16 + cVar13;
  case 0x37:
                    // WARNING (jumptable): Read-only address (ram,0x000142a9aa19) is written
                    // WARNING: Read-only address (ram,0x000142a9aa19) is written
    UNK_142a9aa19 = 0xb3;
    bVar5 = *(byte *)(lVar15 + 0x12ba9aa98);
    *(byte *)(lVar15 + 0x12ba9aa98) = bVar5 << 2 | bVar5 >> 7;
    *(char *)piVar16 = (char)*piVar16 + cVar13;
                    // WARNING (jumptable): Read-only address (ram,0x000142a9aa1b) is written
                    // WARNING: Read-only address (ram,0x000142a9aa1b) is written
    UNK_142a9aa1b = 0x95;
    goto code_r0x00014089d396;
  case 0x2c:
    piVar16 = (int *)(ulonglong)(uVar14 ^ 0xfde020ec);
    *(char *)piVar16 = (char)*piVar16 + (char)(uVar14 ^ 0xfde020ec);
code_r0x00014089d343:
    in_R8 = in_R8 + 0x33;
    break;
  case 0x2d:
    goto code_r0x00014089d396;
  case 0x2e:
    piVar16 = (int *)&DAT_142a9aa98;
    puVar21 = &DAT_142a9ab2c + *(int *)(&DAT_142a9ab2c + (ulonglong)*in_RDX * 4);
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Read-only address (ram,0x000142a9aa99) is written
                    // WARNING: Could not find normalized switch variable to match jumptable
    switch(*in_RDX) {
    case 0:
      in_R8 = in_R8 + 6;
      break;
    case 1:
      in_R8 = (undefined8 *)((longlong)in_R8 + 0x3c);
      break;
    case 2:
      goto code_r0x00014089d3ca;
    case 3:
      in_R8 = (undefined8 *)((longlong)in_R8 + 0x24);
      break;
    case 5:
      in_R8 = (undefined8 *)((longlong)in_R8 + 0xc);
      break;
    case 6:
      in_R8 = in_R8 + 9;
      break;
    case 7:
      in_R8 = (undefined8 *)((longlong)in_R8 + 0x54);
      break;
    case 8:
      in_R8 = in_R8 + 0xc;
      break;
    case 9:
      goto code_r0x00014089d394;
    case 10:
      in_R8 = in_R8 + 0xf;
      break;
    case 0xb:
      goto code_r0x00014089d373;
    case 0xc:
      in_R8 = in_R8 + 0x12;
      break;
    case 0xd:
      in_R8 = (undefined8 *)((longlong)in_R8 + 0x9c);
      break;
    case 0xe:
      in_R8 = in_R8 + 0x15;
      break;
    case 0xf:
      in_R8 = (undefined8 *)((longlong)in_R8 + 0xb4);
      break;
    case 0x10:
      in_R8 = in_R8 + 0x18;
      break;
    case 0x11:
      goto code_r0x00014089d3e8;
    case 0x12:
      in_R8 = in_R8 + 0x1b;
      break;
    case 0x13:
      in_R8 = (undefined8 *)((longlong)in_R8 + 0xe4);
      break;
    case 0x14:
      in_R8 = in_R8 + 0x1e;
      break;
    case 0x15:
      in_R8 = (undefined8 *)((longlong)in_R8 + 0xfc);
      break;
    case 0x16:
      in_R8 = in_R8 + 0x21;
      break;
    case 0x17:
      in_R8 = (undefined8 *)((longlong)in_R8 + 0x114);
      break;
    case 0x18:
      goto code_r0x00014089d3a9;
    case 0x19:
      goto code_r0x00014089d39d;
    case 0x1a:
      in_R8 = in_R8 + 0x27;
      break;
    case 0x1b:
      in_R8 = (undefined8 *)((longlong)in_R8 + 0x144);
      break;
    case 0x1c:
      in_R8 = in_R8 + 0x2a;
      break;
    case 0x1d:
      in_R8 = (undefined8 *)((longlong)in_R8 + 0x15c);
      break;
    case 0x1e:
      in_R8 = in_R8 + 0x2d;
      break;
    case 0x1f:
      goto code_r0x00014089d3dc;
    case 0x20:
      in_R8 = in_R8 + 0x30;
      break;
    case 0x21:
      in_R8 = (undefined8 *)((longlong)in_R8 + 0x18c);
      break;
    case 0x22:
      goto code_r0x00014089d343;
    case 0x23:
      in_R8 = (undefined8 *)((longlong)in_R8 + 0x1a4);
      break;
    case 0x24:
      goto code_r0x00014089d32b;
    case 0x25:
      pcVar10 = (code *)swi(3);
      (*pcVar10)();
      return;
    case 0x26:
                    // WARNING (jumptable): Read-only address (ram,0x000142a9aa98) is written
                    // WARNING: Read-only address (ram,0x000142a9aa98) is written
      _DAT_142a9aa98 = 0xfde02084;
      goto code_r0x00014089e0a5;
    case 0x27:
                    // WARNING (jumptable): Read-only address (ram,0x000142a9aa98) is written
                    // WARNING: Read-only address (ram,0x000142a9aa98) is written
      _DAT_142a9aa98 = 0x4089cb84;
      bVar5 = *unaff_RDI;
      *unaff_RDI = *unaff_RDI + (byte)puVar21;
      *(char *)(unaff_RBP + 0x70) =
           *(char *)(unaff_RBP + 0x70) + -0x68 + CARRY1(bVar5,(byte)puVar21);
      *(undefined4 *)(unaff_RBP + 0x100) = in_XMM0_Da;
      *(undefined4 *)(unaff_RBP + 0x104) = in_XMM0_Db;
      *(undefined4 *)(unaff_RBP + 0x108) = in_XMM0_Dc;
      *(undefined4 *)(unaff_RBP + 0x10c) = in_XMM0_Dd;
      *(undefined8 *)(unaff_RBP + 0xf0) = *(undefined8 *)(unaff_RBP + 0x60);
      *(undefined8 *)(unaff_RBP + 0xf8) = *(undefined8 *)(unaff_RBP + 0x68);
      *(undefined8 *)(unaff_RBP + 0xe0) = *(undefined8 *)(unaff_RBP + 0x50);
      *(undefined8 *)(unaff_RBP + 0xe8) = *(undefined8 *)(unaff_RBP + 0x58);
      *(undefined8 *)(unaff_RBP + 0xd0) = *(undefined8 *)(unaff_RBP + 0x40);
      *(undefined8 *)(unaff_RBP + 0xd8) = *(undefined8 *)(unaff_RBP + 0x48);
      *(undefined8 *)(unaff_RBP + 0xc0) = *(undefined8 *)(unaff_RBP + 0x30);
      *(undefined8 *)(unaff_RBP + 200) = *(undefined8 *)(unaff_RBP + 0x38);
      *(byte **)(unaff_RBP + 0x138) = unaff_RDI;
      asset_libraries::mesh_library::AssetMeshLibrary::add();
      *(undefined8 *)(unaff_RBP + 0x160) = extraout_RAX_17;
      goto code_r0x00014089df4a;
    case 0x28:
      *(undefined4 *)(unaff_RBP + 0x110) = in_XMM0_Da;
      *(undefined4 *)(unaff_RBP + 0x114) = in_XMM0_Db;
      *(undefined4 *)(unaff_RBP + 0x118) = in_XMM0_Dc;
      *(undefined4 *)(unaff_RBP + 0x11c) = in_XMM0_Dd;
      *(undefined8 *)(unaff_RBP + 0x100) = *(undefined8 *)(unaff_RBP + -0x18);
      *(undefined8 *)(unaff_RBP + 0x108) = *(undefined8 *)(unaff_RBP + -0x10);
      *(undefined8 *)(unaff_RBP + 0xf0) = *(undefined8 *)(unaff_RBP + -0x28);
      *(undefined8 *)(unaff_RBP + 0xf8) = *(undefined8 *)(unaff_RBP + -0x20);
      *(undefined8 *)(unaff_RBP + 0xe0) = *(undefined8 *)(unaff_RBP + -0x38);
      *(undefined8 *)(unaff_RBP + 0xe8) = *(undefined8 *)(unaff_RBP + -0x30);
      *(undefined8 *)(unaff_RBP + 0xd0) = *(undefined8 *)(unaff_RBP + -0x48);
      *(undefined8 *)(unaff_RBP + 0xd8) = *(undefined8 *)(unaff_RBP + -0x40);
      *(undefined8 *)(unaff_RBP + 0xc0) = *(undefined8 *)(unaff_RBP + -0x58);
      *(undefined8 *)(unaff_RBP + 200) = *(undefined8 *)(unaff_RBP + -0x50);
      *(byte **)(unaff_RBP + 0x138) = unaff_RDI;
      asset_libraries::mesh_library::AssetMeshLibrary::add();
      *(undefined8 *)(unaff_RBP + 0x160) = extraout_RAX_18;
      uVar19 = extraout_RAX_18;
      if (*(longlong *)(unaff_RBP + 0x170) != 0) {
        *(int *)(unaff_RBP + 0x178) = (int)CONCAT71((int7)((ulonglong)extraout_RAX_18 >> 8),1);
        goto code_r0x00014089e05f;
      }
      goto LAB_14089e28e;
    case 0x29:
                    // WARNING (jumptable): Read-only address (ram,0x000142a9aa98) is written
                    // WARNING: Read-only address (ram,0x000142a9aa98) is written
      _DAT_142a9aa98 = 0xfde02084;
      *(undefined **)(unaff_RBP + 0xa8) = puVar21;
      lVar15 = (longlong)iRam000000064d505620;
      *(undefined **)(unaff_RBP + 0x158) = &DAT_142a9ab2c;
      *(undefined8 *)(unaff_RBP + 0xb0) = unaff_RBX;
                    // WARNING: Could not recover jumptable at 0x00014089dddf. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9abc0 + lVar15))(&DAT_142a9abc0 + lVar15);
      return;
    }
    break;
  case 0x2f:
    *(char *)piVar16 = (char)*piVar16 + cVar13;
code_r0x00014089d394:
    in_R8 = (undefined8 *)((longlong)in_R8 + 0x6c);
    break;
  case 0x30:
    UNK_142a9aa19 = 0x1b;
    UNK_142a9aa1b = 0xfd;
    _DAT_142a9aa98 = 0xfde020ec;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x31:
    pcVar10 = (code *)swi(3);
    (*pcVar10)();
    return;
  case 0x32:
    *(char *)piVar16 = (char)*piVar16 + cVar13;
    goto code_r0x00014089d373;
  case 0x33:
code_r0x00014089d2cc:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  case 0x34:
    *(undefined1 *)(lVar15 + 0x12ba9aa99) = 0;
    *piVar16 = *piVar16 + uVar14;
    goto code_r0x00014089d336;
  case 0x35:
    goto code_r0x00014089d2bb;
  case 0x36:
    *(char *)piVar16 = (char)*piVar16 + cVar13;
    break;
  case 0x38:
    (&DAT_142a9aa98)[unaff_RBP * 8] = (char)(&DAT_142a9aa98)[unaff_RBP * 8] >> 0x18;
    *(char *)piVar16 = (char)*piVar16 + cVar13;
                    // WARNING (jumptable): Read-only address (ram,0x000142a9aa19) is written
                    // WARNING: Read-only address (ram,0x000142a9aa19) is written
    UNK_142a9aa19 = 0xb3;
  case 0x3c:
    pcVar2 = (char *)(lVar15 + 0xef92aa98 + (longlong)piVar16);
    *pcVar2 = *pcVar2;
    *(char *)piVar16 = (char)*piVar16 + cVar13;
code_r0x00014089d3ca:
    in_R8 = in_R8 + 3;
    break;
  case 0x39:
    goto code_r0x00014089d360;
  case 0x3a:
    piVar16 = (int *)(ulonglong)(uVar14 + 0xfde020ec);
code_r0x00014089d371:
    *(char *)piVar16 = (char)*piVar16 + (char)piVar16;
code_r0x00014089d373:
    in_R8 = (undefined8 *)((longlong)in_R8 + 0x84);
    break;
  case 0x3b:
    *(char *)piVar16 = (char)*piVar16 + cVar13;
    goto code_r0x00014089d3a9;
  case 0x3d:
    if (piVar16 != (int *)0x0) {
      piVar16 = (int *)(ulonglong)(uVar14 + 0x30);
      break;
    }
    goto code_r0x00014089d360;
  case 0x3e:
    UNK_142a9aa19 = 0x1b;
    UNK_142a9aa1b = 0xfd;
    _DAT_142a9aa98 = 0xfde020ec;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x3f:
                    // WARNING (jumptable): Read-only address (ram,0x000142a9aa19) is written
                    // WARNING: Read-only address (ram,0x000142a9aa19) is written
    UNK_142a9aa19 = 0xb3;
    *(undefined1 *)(lVar15 + 0x285535530) = *(undefined1 *)(lVar15 + 0x285535530);
    goto code_r0x00014089d3dc;
  case 0x40:
    *(char *)piVar16 = (char)*piVar16 + cVar13;
    goto code_r0x00014089d299;
  case 0x41:
                    // WARNING (jumptable): Read-only address (ram,0x000142a9aa1b) is written
                    // WARNING: Read-only address (ram,0x000142a9aa1b) is written
    UNK_142a9aa1b = 0x95;
    *(char *)piVar16 = (char)*piVar16;
    uRame954c08349000000 = uVar14;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x42:
                    // WARNING (jumptable): Read-only address (ram,0x000142a9aa1b) is written
                    // WARNING: Read-only address (ram,0x000142a9aa1b) is written
    UNK_142a9aa1b = 0x95;
    (&UNK_142a9aa32)[unaff_RBP * 8] = (&UNK_142a9aa32)[unaff_RBP * 8];
    *(char *)piVar16 = (char)*piVar16 + cVar13;
code_r0x00014089d3dc:
    in_R8 = (undefined8 *)((longlong)in_R8 + 0x174);
    break;
  case 0x43:
                    // WARNING (jumptable): Read-only address (ram,0x000142a9aa19) is written
                    // WARNING: Read-only address (ram,0x000142a9aa19) is written
    UNK_142a9aa19 = 0xb3;
    *(undefined1 *)(lVar15 + 0x285535530) = *(undefined1 *)(lVar15 + 0x285535530);
    puVar21 = &UNK_142a9aa42;
  case 0x45:
    pcVar10 = (code *)swi(0);
    piVar16 = (int *)(*pcVar10)(puVar21);
    *(char *)piVar16 = (char)*piVar16 + (char)piVar16;
code_r0x00014089d3a9:
    in_R8 = in_R8 + 0x24;
    break;
  case 0x44:
    *(char *)piVar16 = (char)*piVar16 + cVar13;
    break;
  case 0x46:
    *(char *)piVar16 = (char)*piVar16 + cVar13;
    *(byte *)(unaff_RBP + -0x72b3fdfd) =
         *(byte *)(unaff_RBP + -0x72b3fdfd) | (byte)((ulonglong)unaff_RBX >> 8);
code_r0x00014089d2e2:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  case 0x48:
    *piVar16 = *piVar16 + uVar14;
    *(char *)piVar16 = (char)*piVar16 + cVar13;
code_r0x00014089d28f:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  case 0x49:
code_r0x00014089d299:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  case 0x4a:
    pcVar10 = (code *)swi(3);
    (*pcVar10)();
    return;
  case 0x4b:
code_r0x00014089e0a5:
    *(char *)piVar16 = (char)*piVar16 + (char)piVar16;
    *(char *)((longlong)piVar16 + -0x77) = *(char *)((longlong)piVar16 + -0x77) + (char)puVar21;
    *(undefined8 *)(unaff_RBP + 0xb8) = in_R9;
    country_core::utils::geo::mesh_from_pts();
    *(undefined8 *)(unaff_RBP + 0xc0) = *(undefined8 *)(unaff_RBP + 0x170);
    *(undefined8 *)(unaff_RBP + 200) = *(undefined8 *)(unaff_RBP + 0x178);
    *(size_t **)(unaff_RBP + 0xd0) = in_RCX;
    *(undefined1 *)(unaff_RBP + 0x186) = 1;
    country_core::render::mesh::Mesh::
    set_attribute<alloc::vec::Vec<array_<f32,3>,alloc::alloc::Global>_>();
    *(undefined8 *)(unaff_RBP + 0x130) = *(undefined8 *)(unaff_RBP + 0xa0);
    *(undefined8 *)(unaff_RBP + 0x120) = *(undefined8 *)(unaff_RBP + 0x90);
    *(undefined8 *)(unaff_RBP + 0x128) = *(undefined8 *)(unaff_RBP + 0x98);
    *(undefined8 *)(unaff_RBP + 0x110) = *(undefined8 *)(unaff_RBP + 0x80);
    *(undefined8 *)(unaff_RBP + 0x118) = *(undefined8 *)(unaff_RBP + 0x88);
    *(undefined8 *)(unaff_RBP + 0x100) = *(undefined8 *)(unaff_RBP + 0x70);
    *(undefined8 *)(unaff_RBP + 0x108) = *(undefined8 *)(unaff_RBP + 0x78);
    *(undefined8 *)(unaff_RBP + 0xf0) = *(undefined8 *)(unaff_RBP + 0x60);
    *(undefined8 *)(unaff_RBP + 0xf8) = *(undefined8 *)(unaff_RBP + 0x68);
    *(undefined8 *)(unaff_RBP + 0xe0) = *(undefined8 *)(unaff_RBP + 0x50);
    *(undefined8 *)(unaff_RBP + 0xe8) = *(undefined8 *)(unaff_RBP + 0x58);
    *(undefined8 *)(unaff_RBP + 0xd0) = *(undefined8 *)(unaff_RBP + 0x40);
    *(undefined8 *)(unaff_RBP + 0xd8) = *(undefined8 *)(unaff_RBP + 0x48);
    *(undefined8 *)(unaff_RBP + 0xc0) = *(undefined8 *)(unaff_RBP + 0x30);
    *(undefined8 *)(unaff_RBP + 200) = *(undefined8 *)(unaff_RBP + 0x38);
    *(byte **)(unaff_RBP + 0x138) = unaff_RDI;
    *(undefined1 *)(unaff_RBP + 0x186) = 0;
    asset_libraries::mesh_library::AssetMeshLibrary::add();
    *(undefined8 *)(unaff_RBP + 0x160) = extraout_RAX_19;
    uVar19 = extraout_RAX_19;
    if (*(longlong *)(unaff_RBP + 0x168) != 0) {
      *(int *)(unaff_RBP + 0x178) = (int)CONCAT71((int7)((ulonglong)extraout_RAX_19 >> 8),1);
      std::alloc::__default_lib_allocator::__rust_dealloc();
      goto LAB_14089e296;
    }
    goto LAB_14089e28e;
  case 0x4c:
    (&UNK_142a9aa21)[lVar15] = (&UNK_142a9aa21)[lVar15] + -0x68;
    return;
  case 0x4d:
    *(char *)piVar16 = (char)*piVar16 + cVar13;
code_r0x00014089df4a:
    uVar19 = 0;
    if ((code *)**(undefined8 **)(unaff_RBP + 0x170) != (code *)0x0) {
      uVar19 = (*(code *)**(undefined8 **)(unaff_RBP + 0x170))(*(undefined8 *)(unaff_RBP + 0x168));
    }
    if (*(longlong *)(*(longlong *)(unaff_RBP + 0x170) + 8) != 0) {
      *(int *)(unaff_RBP + 0x178) = (int)CONCAT71((int7)((ulonglong)uVar19 >> 8),1);
code_r0x00014089e05f:
      std::alloc::__default_lib_allocator::__rust_dealloc();
      goto LAB_14089e296;
    }
LAB_14089e28e:
    *(int *)(unaff_RBP + 0x178) = (int)CONCAT71((int7)((ulonglong)uVar19 >> 8),1);
LAB_14089e296:
    bevy_ecs::system::commands::Commands::spawn_empty();
    lVar15 = *(longlong *)(unaff_RBP + 0x158);
    if (*(char *)(lVar15 + 0x50) == '\0') {
      uVar24 = 0x3f800000;
      cVar13 = *(char *)(lVar15 + 0x40);
    }
    else {
      uVar24 = *(undefined4 *)(lVar15 + 0x54);
      cVar13 = *(char *)(lVar15 + 0x40);
    }
    if (cVar13 == '\0') {
      *(undefined1 *)(lVar15 + 0x40) = 0;
      *(undefined4 *)(lVar15 + 0x44) = uVar24;
      uVar24 = 0;
    }
    uVar19 = *(undefined8 *)(lVar15 + 0x68);
    asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
    if ((extraout_RAX_20 & 1) == 0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    plVar23 = *(longlong **)(unaff_RBP + 200);
    if (plVar23 == (longlong *)0x0) {
      plVar23 = *(longlong **)(unaff_RBP + 0xd0);
    }
    puVar17 = *(undefined8 **)(unaff_RBP + 0x158);
    uVar11 = *puVar17;
    uVar12 = puVar17[1];
    uVar3 = *(undefined4 *)(puVar17 + 0xf);
    uVar6 = *(undefined4 *)(puVar17 + 9);
    uVar4 = *(undefined4 *)((longlong)puVar17 + 0x4c);
    uVar7 = puVar17[8];
    uVar8 = *(undefined8 *)(unaff_RBP + 0xe0);
    lVar15 = plVar23[2];
    if ((ulonglong)(*plVar23 - lVar15) < 0x98) {
      uStack_98 = 1;
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar22 = *(longlong *)(unaff_RBP + 0x158);
    lVar20 = plVar23[1];
    *(code **)(lVar20 + lVar15) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::systems::ui::signifiers::SignifierAnimation>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    puVar17 = (undefined8 *)(lVar20 + 8 + lVar15);
    *puVar17 = uVar11;
    puVar17[1] = uVar12;
    *(undefined8 *)(lVar20 + 0x18 + lVar15) = *(undefined8 *)(lVar22 + 0x7c);
    *(undefined4 *)(lVar20 + 0x20 + lVar15) = *(undefined4 *)(lVar22 + 0x84);
    *(undefined4 *)(lVar20 + 0x24 + lVar15) = uVar24;
    *(undefined4 *)(lVar20 + 0x28 + lVar15) = uVar24;
    *(undefined4 *)(lVar20 + 0x2c + lVar15) = uVar24;
    *(undefined8 *)(lVar20 + 0x38 + lVar15) = *(undefined8 *)(unaff_RBP + 0x160);
    *(undefined8 *)(lVar20 + 0x40 + lVar15) = uVar19;
    *(undefined4 *)(lVar20 + 0x48 + lVar15) = uVar6;
    *(undefined4 *)(lVar20 + 0x4c + lVar15) = uVar4;
    *(undefined8 *)(lVar20 + 0x50 + lVar15) = uVar7;
    *(undefined4 *)(lVar20 + 0x58 + lVar15) = uVar3;
    *(undefined8 *)(lVar20 + 0x5c + lVar15) = 0;
    *(undefined1 *)(lVar20 + 0x68 + lVar15) = 0;
    *(undefined8 *)(lVar20 + 0x78 + lVar15) = uVar8;
    *(code **)(lVar20 + 0x88 + lVar15) = bevy_ecs::error::handler::panic;
    lVar20 = lVar15 + 0x98;
    plVar23[2] = lVar20;
    if (*(char *)(lVar22 + 0x89) == '\0') {
      if (*(char *)(lVar22 + 0x8a) == '\0') {
        if ((ulonglong)(*plVar23 - lVar20) < 0x20) {
          uStack_98 = 1;
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
        lVar20 = plVar23[1];
        *(code **)(lVar20 + 0x98 + lVar15) =
             core::ops::function::FnOnce::
             call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::UiPass>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
        ;
        *(code **)(lVar20 + 0xa0 + lVar15) = bevy_ecs::error::handler::panic;
        *(undefined8 *)(lVar20 + 0xa8 + lVar15) = uVar8;
        *(undefined1 *)(lVar20 + 0xb0 + lVar15) = 0;
        lVar20 = lVar15 + 0xb8;
        plVar23[2] = lVar20;
        lVar22 = 0xd8;
        if ((ulonglong)(*plVar23 - lVar20) < 0x20) {
          uStack_98 = 1;
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
      }
      else {
        lVar22 = 0xb8;
        if ((ulonglong)(*plVar23 - lVar20) < 0x20) {
          uStack_98 = 1;
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
      }
    }
    else {
      if ((ulonglong)(*plVar23 - lVar20) < 0x20) {
        uStack_98 = 1;
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar20 = plVar23[1];
      *(code **)(lVar20 + 0x98 + lVar15) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::WireframePass>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      *(code **)(lVar20 + 0xa0 + lVar15) = bevy_ecs::error::handler::panic;
      *(undefined8 *)(lVar20 + 0xa8 + lVar15) = uVar8;
      *(undefined1 *)(lVar20 + 0xb0 + lVar15) = 0;
      lVar20 = lVar15 + 0xb8;
      plVar23[2] = lVar20;
      lVar22 = 0xd8;
      if ((ulonglong)(*plVar23 - lVar20) < 0x20) {
        uStack_98 = 1;
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
    }
    lVar9 = plVar23[1];
    *(code **)(lVar9 + lVar20) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::DontCastShadow>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar9 + 8 + lVar20) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar9 + 0x10 + lVar20) = uVar8;
    *(undefined1 *)(lVar9 + 0x18 + lVar20) = 0;
    lVar15 = lVar15 + lVar22;
    plVar23[2] = lVar15;
    lVar20 = *(longlong *)(unaff_RBP + 0x158);
    if ((*(byte *)(lVar20 + 0x58) & 1) != 0) {
      uVar24 = *(undefined4 *)(lVar20 + 0x5c);
      if ((ulonglong)(*plVar23 - lVar15) < 0x28) {
        uStack_98 = 1;
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar20 = plVar23[1];
      *(code **)(lVar20 + lVar15) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::systems::ui::signifiers::SignifierMinimumLifespan>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      *(code **)(lVar20 + 8 + lVar15) = bevy_ecs::error::handler::panic;
      *(undefined8 *)(lVar20 + 0x10 + lVar15) = uVar8;
      *(undefined4 *)(lVar20 + 0x18 + lVar15) = 0;
      *(undefined4 *)(lVar20 + 0x1c + lVar15) = uVar24;
      *(undefined1 *)(lVar20 + 0x20 + lVar15) = 0;
      lVar15 = lVar15 + 0x28;
      plVar23[2] = lVar15;
      lVar20 = *(longlong *)(unaff_RBP + 0x158);
    }
    if (*(char *)(lVar20 + 0x88) != '\0') {
      if ((ulonglong)(*plVar23 - lVar15) < 0x20) {
        uStack_98 = 1;
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar20 = plVar23[1];
      *(code **)(lVar20 + lVar15) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::systems::ui::signifiers::SignifierParentToVolumeOrigin>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      *(code **)(lVar20 + 8 + lVar15) = bevy_ecs::error::handler::panic;
      *(undefined8 *)(lVar20 + 0x10 + lVar15) = uVar8;
      *(undefined1 *)(lVar20 + 0x18 + lVar15) = 0;
      plVar23[2] = lVar15 + 0x20;
    }
    if (*(char *)(unaff_RBP + 0x178) != '\0') {
      plVar23 = *(longlong **)(*(longlong *)(unaff_RBP + 0xb0) + 0x18);
      LOCK();
      lVar15 = *plVar23;
      *plVar23 = *plVar23 + 1;
      UNLOCK();
      if (*plVar23 == 0 || SCARRY8(lVar15,1) != *plVar23 < 0) {
                    // WARNING: Does not return
        pcVar10 = (code *)invalidInstructionException();
        (*pcVar10)();
      }
      bevy_ecs::system::commands::EntityCommands::
      insert<country_core::components::transient_mesh::TransientMesh>();
    }
    puVar17 = *(undefined8 **)(unaff_RBP + 0xa8);
    puVar17[4] = *(undefined8 *)(unaff_RBP + 0xe0);
    uVar19 = *(undefined8 *)(unaff_RBP + 0xc0);
    uVar7 = *(undefined8 *)(unaff_RBP + 200);
    uVar8 = *(undefined8 *)(unaff_RBP + 0xd8);
    puVar17[2] = *(undefined8 *)(unaff_RBP + 0xd0);
    puVar17[3] = uVar8;
    *puVar17 = uVar19;
    puVar17[1] = uVar7;
    if (*(longlong *)(*(longlong *)(unaff_RBP + 0x158) + 0x60) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    return;
  case 0x4e:
    in_RDX[0x88] = 1;
    *(undefined8 *)((ulonglong)((int)auStack_b8 - 0x20) - 8) = 0x14089dd3a;
    memcpy(&DAT_142a9aa98,in_RDX,0x90);
    return;
  }
code_r0x00014089d476:
  piVar16[2] = *(int *)(in_R8 + 1);
  *(undefined8 *)piVar16 = *in_R8;
  return;
code_r0x00014089d360:
  *(char *)piVar16 = (char)*piVar16 + cVar13;
  goto code_r0x00014089d476;
code_r0x00014089d396:
  (&UNK_142a9aa71)[unaff_RBP * 8] = (&UNK_142a9aa71)[unaff_RBP * 8];
  *(char *)piVar16 = (char)*piVar16 + cVar13;
code_r0x00014089d39d:
  in_R8 = (undefined8 *)((longlong)in_R8 + 300);
  goto code_r0x00014089d476;
code_r0x00014089d336:
                    // WARNING (jumptable): Read-only address (ram,0x000142a9aa19) is written
                    // WARNING: Read-only address (ram,0x000142a9aa19) is written
  UNK_142a9aa19 = 0xb3;
  *(undefined1 *)(lVar15 + 0x285535530) = *(undefined1 *)(lVar15 + 0x285535530);
  goto code_r0x00014089d476;
}


