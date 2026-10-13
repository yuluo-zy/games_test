// addr=0x140946620 name=just_pressed
// signature=void __cdecl just_pressed(void)


// WARNING: Instruction at (ram,0x000140947bf2) overlaps instruction at (ram,0x000140947bed)
// 
// WARNING: Control flow encountered bad instruction data

void __cdecl country_core::input::action_set::MainActionSet::just_pressed(void)

{
  uint *puVar1;
  byte *pbVar2;
  undefined8 uVar3;
  byte bVar4;
  code cVar5;
  uint uVar6;
  int iVar7;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  char *pcVar8;
  ulonglong uVar9;
  code *UNRECOVERED_JUMPTABLE;
  undefined7 uVar12;
  longlong lVar10;
  int *piVar11;
  byte bVar13;
  code *in_RCX;
  code *pcVar14;
  byte in_DL;
  undefined *puVar15;
  char cVar16;
  ulonglong unaff_RBX;
  undefined6 uVar17;
  undefined8 *puVar18;
  undefined1 *puVar19;
  undefined8 *unaff_RBP;
  byte *unaff_RSI;
  byte *unaff_RDI;
  longlong in_R8;
  undefined *in_R9;
  undefined *in_R10;
  int unaff_R14D;
  undefined2 in_FS;
  bool bVar20;
  byte bVar21;
  bool bVar22;
  int unaff_XMM6_Da;
  int unaff_XMM6_Db;
  int unaff_XMM6_Dc;
  int unaff_XMM6_Dd;
  uint unaff_XMM13_Da;
  uint unaff_XMM14_Da;
  undefined8 *unaff_retaddr;
  undefined1 auStackX_8 [8];
  undefined *puStackX_10;
  undefined1 auStack_58 [48];
  
  puVar15 = &DAT_142ab2c78;
  uVar9 = (ulonglong)*(int *)(&DAT_142ab2c78 + (ulonglong)in_DL * 4);
  bVar20 = 0xfffffffebd54d387 < uVar9;
  UNRECOVERED_JUMPTABLE = (code *)(&DAT_142ab2c78 + uVar9);
  bVar22 = UNRECOVERED_JUMPTABLE == (code *)0x0;
  bVar13 = (byte)in_RCX;
  bVar4 = (byte)unaff_RBX;
  bVar21 = (byte)(unaff_RBX >> 8);
  uVar17 = (undefined6)(unaff_RBX >> 0x10);
  cVar5 = SUB81(UNRECOVERED_JUMPTABLE,0);
  uVar6 = (uint)UNRECOVERED_JUMPTABLE;
  uVar12 = (undefined7)((ulonglong)UNRECOVERED_JUMPTABLE >> 8);
  cVar16 = (char)bVar21 >> 1;
  puVar19 = (undefined1 *)register0x00000020;
  switch(in_DL) {
  case 0:
    puVar15 = &UNK_142ab3024;
    UNRECOVERED_JUMPTABLE =
         (code *)(longlong)*(int *)(&UNK_142ab3024 + (ulonglong)*(uint *)in_RCX * 4);
  case 0x70:
                    // WARNING: Could not recover jumptable at 0x000140946643. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(UNRECOVERED_JUMPTABLE + (longlong)puVar15))();
    return;
  case 1:
    uVar9 = (ulonglong)*(uint *)(in_RCX + 8);
    puVar15 = &UNK_142ab3010;
    goto code_r0x0001409468ac;
  case 2:
    UNRECOVERED_JUMPTABLE = (code *)(ulonglong)*(uint *)(in_RCX + 0x10);
    puVar15 = &UNK_142ab2ffc;
    goto code_r0x000140946834;
  case 3:
    uVar6 = *(uint *)(in_RCX + 0x18);
    puVar15 = &UNK_142ab2fe8;
    goto code_r0x000140946870;
  case 4:
    uVar6 = *(uint *)(in_RCX + 0x20);
    puVar15 = &UNK_142ab2fd4;
    goto code_r0x000140946792;
  case 5:
    uVar9 = (ulonglong)*(uint *)(in_RCX + 0x28);
    puVar15 = &UNK_142ab2fc0;
    goto code_r0x000140946936;
  case 6:
    puVar15 = &UNK_142ab2fac;
    uVar9 = (ulonglong)*(int *)(&UNK_142ab2fac + (ulonglong)*(uint *)(in_RCX + 0x30) * 4);
    goto code_r0x00014094699a;
  case 7:
                    // WARNING: Could not recover jumptable at 0x000140946895. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&UNK_142ab2f98 + *(int *)(&UNK_142ab2f98 + (ulonglong)*(uint *)(in_RCX + 0x38) * 4)))
              ();
    return;
  case 8:
  case 0x2a:
                    // WARNING: Could not recover jumptable at 0x0001409469fd. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&UNK_142ab2f84 + *(int *)(&UNK_142ab2f84 + (ulonglong)*(uint *)(in_RCX + 0x40) * 4)))
              ();
    return;
  case 9:
    uVar9 = (ulonglong)*(uint *)(in_RCX + 0x48);
    puVar15 = &UNK_142ab2f70;
    goto code_r0x0001409467ce;
  case 10:
    uVar9 = (ulonglong)*(uint *)(in_RCX + 0x50);
    goto code_r0x0001409469d1;
  case 0xb:
                    // WARNING: Could not recover jumptable at 0x00014094677b. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&UNK_142ab2f48 + *(int *)(&UNK_142ab2f48 + (ulonglong)*(uint *)(in_RCX + 0x58) * 4)))
              ();
    return;
  case 0xc:
                    // WARNING: Could not recover jumptable at 0x0001409467b7. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&UNK_142ab2f34 + *(int *)(&UNK_142ab2f34 + (ulonglong)*(uint *)(in_RCX + 0x60) * 4)))
              ();
    return;
  case 0xd:
    uVar6 = *(uint *)(in_RCX + 0x68);
    puVar15 = &UNK_142ab2f20;
    goto code_r0x000140946978;
  case 0xe:
                    // WARNING: Could not recover jumptable at 0x000140946739. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&UNK_142ab2f0c + *(int *)(&UNK_142ab2f0c + (ulonglong)*(uint *)(in_RCX + 0x70) * 4)))
              ();
    return;
  case 0xf:
                    // WARNING: Could not recover jumptable at 0x000140946859. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&UNK_142ab2ef8 + *(int *)(&UNK_142ab2ef8 + (ulonglong)*(uint *)(in_RCX + 0x78) * 4)))
              ();
    return;
  case 0x10:
    uVar6 = *(uint *)(in_RCX + 0x80);
    puVar15 = &UNK_142ab2ee4;
    goto code_r0x000140946711;
  default:
    UNRECOVERED_JUMPTABLE = (code *)(ulonglong)*(uint *)(in_RCX + 0x88);
    goto code_r0x0001409468ea;
  case 0x12:
    uVar6 = *(uint *)(in_RCX + 0x90);
    puVar15 = &UNK_142ab2ebc;
    goto code_r0x0001409469b7;
  case 0x13:
    uVar6 = *(uint *)(in_RCX + 0x98);
    puVar15 = &UNK_142ab2ea8;
    goto code_r0x000140946aa7;
  case 0x14:
    UNRECOVERED_JUMPTABLE = (code *)(ulonglong)*(uint *)(in_RCX + 0xa0);
    goto code_r0x00014094690e;
  case 0x15:
    UNRECOVERED_JUMPTABLE =
         (code *)(&UNK_142ab2e80 +
                 *(int *)(&UNK_142ab2e80 + (ulonglong)*(uint *)(in_RCX + 0xa8) * 4));
    goto code_r0x00014094695e;
  case 0x16:
    puVar15 = &UNK_142ab2e6c;
    UNRECOVERED_JUMPTABLE =
         (code *)(longlong)*(int *)(&UNK_142ab2e6c + (ulonglong)*(uint *)(in_RCX + 0xb0) * 4);
    goto code_r0x000140946a63;
  case 0x17:
    UNRECOVERED_JUMPTABLE = (code *)(ulonglong)*(uint *)(in_RCX + 0xb8);
  case 0x29:
  case 0x2b:
  case 0x2c:
                    // WARNING: Could not recover jumptable at 0x000140946ad2. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&UNK_142ab2e58 + *(int *)(&UNK_142ab2e58 + (longlong)UNRECOVERED_JUMPTABLE * 4)))();
    return;
  case 0x18:
    uVar9 = (ulonglong)*(uint *)(in_RCX + 0xc0);
    puVar15 = &UNK_142ab2e44;
    goto code_r0x000140946813;
  case 0x19:
    uVar9 = (ulonglong)*(uint *)(in_RCX + 200);
    puVar15 = &UNK_142ab2e30;
    goto code_r0x0001409467ef;
  case 0x1a:
                    // WARNING: Could not recover jumptable at 0x000140946b62. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&UNK_142ab2e1c + *(int *)(&UNK_142ab2e1c + (ulonglong)*(uint *)(in_RCX + 0xd0) * 4)))
              ();
    return;
  case 0x1b:
                    // WARNING: Could not recover jumptable at 0x0001409466d0. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&UNK_142ab2e08 + *(int *)(&UNK_142ab2e08 + (ulonglong)*(uint *)(in_RCX + 0xd8) * 4)))
              ();
    return;
  case 0x1c:
                    // WARNING: Could not recover jumptable at 0x000140946af6. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&UNK_142ab2df4 + *(int *)(&UNK_142ab2df4 + (ulonglong)*(uint *)(in_RCX + 0xe0) * 4)))
              ();
    return;
  case 0x1d:
                    // WARNING: Could not recover jumptable at 0x000140946b1a. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&UNK_142ab2de0 + *(int *)(&UNK_142ab2de0 + (ulonglong)*(uint *)(in_RCX + 0xe8) * 4)))
              ();
    return;
  case 0x1e:
    UNRECOVERED_JUMPTABLE = (code *)(ulonglong)*(uint *)(in_RCX + 0xf0);
  case 0x56:
  case 0x58:
  case 0x59:
    puVar15 = &UNK_142ab2dcc;
code_r0x000140946a17:
                    // WARNING: Could not recover jumptable at 0x000140946a1e. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(puVar15 + *(int *)(puVar15 + (longlong)UNRECOVERED_JUMPTABLE * 4)))();
    return;
  case 0x1f:
    puVar15 = &UNK_142ab2db8;
    lVar10 = (longlong)*(int *)(&UNK_142ab2db8 + (ulonglong)*(uint *)(in_RCX + 0xf8) * 4);
    goto code_r0x0001409468d1;
  case 0x20:
    UNRECOVERED_JUMPTABLE = (code *)(ulonglong)*(uint *)(in_RCX + 0x100);
    puVar15 = &UNK_142ab2da4;
    goto code_r0x000140946a3b;
  case 0x21:
    UNRECOVERED_JUMPTABLE =
         (code *)(&UNK_142ab2d90 +
                 *(int *)(&UNK_142ab2d90 + (ulonglong)*(uint *)(in_RCX + 0x108) * 4));
    goto code_r0x00014094675a;
  case 0x22:
    piVar11 = (int *)(ulonglong)*(uint *)(in_RCX + 0x110);
    puVar15 = &UNK_142ab2d7c;
    goto code_r0x0001409466ed;
  case 0x23:
    UNRECOVERED_JUMPTABLE = (code *)(ulonglong)*(uint *)(in_RCX + 0x118);
    puVar15 = &UNK_142ab2d68;
    goto code_r0x000140946681;
  case 0x24:
                    // WARNING: Could not recover jumptable at 0x0001409466ac. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&UNK_142ab2d54 + *(int *)(&UNK_142ab2d54 + (ulonglong)*(uint *)(in_RCX + 0x120) * 4))
    )();
    return;
  case 0x25:
  case 0x75:
    goto code_r0x000140946650;
  case 0x26:
                    // WARNING: Could not recover jumptable at 0x000140946b3e. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&UNK_142ab2d2c + *(int *)(&UNK_142ab2d2c + (ulonglong)*(uint *)(in_RCX + 0x130) * 4))
    )();
    return;
  case 0x27:
    UNRECOVERED_JUMPTABLE = (code *)(ulonglong)*(uint *)(in_RCX + 0x138);
    puVar15 = &UNK_142ab2d18;
    goto code_r0x000140946a83;
  case 0x28:
    DAT_142ab2c78 = 0xbb;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x2d:
    goto code_r0x000140946ab3;
  case 0x2e:
  case 0x30:
  case 0x31:
code_r0x000140946ab3:
    DAT_142ab2c78 = 0xbb;
    return;
  case 0x2f:
    DAT_142ab2c78 = 0xbb;
    return;
  case 0x32:
    goto code_r0x000140946a9f;
  case 0x33:
  case 0x35:
  case 0x36:
    goto code_r0x000140946a9c;
  case 0x34:
    goto code_r0x00014094659e;
  case 0x37:
    in_RCX = in_RCX + -1;
    if (in_RCX == (code *)0x0 || bVar22) {
      *unaff_RDI = *unaff_RDI >> 6 | *unaff_RDI << 2;
      *(int *)(uVar9 + 0x18aab2c78) =
           (*(int *)(uVar9 + 0x18aab2c78) + 0x3bfeea73) - (uint)((bVar4 >> 5 & 1) != 0);
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    }
    goto code_r0x000140946a9c;
  case 0x38:
  case 0x3a:
  case 0x3b:
    UNRECOVERED_JUMPTABLE = (code *)(ulonglong)(uVar6 + 0x42ab2c78);
    goto code_r0x000140946a83;
  case 0x39:
    (&UNK_142ab2c05)[uVar9] = (&UNK_142ab2c05)[uVar9] + bVar13;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x3c:
    goto code_r0x000140946a77;
  case 0x3d:
  case 0x3f:
  case 0x40:
    goto code_r0x000140946a77;
  case 0x3e:
    goto code_r0x00014094659a;
  case 0x41:
    goto code_r0x000140946a63;
  case 0x42:
  case 0x44:
  case 0x45:
    UNRECOVERED_JUMPTABLE =
         (code *)(ulonglong)*(uint *)(&DAT_142ab2c78 + (longlong)UNRECOVERED_JUMPTABLE * 4);
    goto code_r0x000140946a63;
  case 0x43:
    register0x00000020 = (BADSPACEBASE *)&stack0xffffffffffffffe8;
  case 0xd4:
    *(byte **)((longlong)register0x00000020 + -8) = unaff_RDI;
    *(undefined8 *)((longlong)register0x00000020 + -0x30) = 0x14094660f;
    core::ptr::
    drop_in_place<enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>
              ();
    return;
  case 0x46:
    puVar1 = (uint *)((unaff_RBX & 0xffffffff) + 0x48000000);
    *puVar1 = *puVar1 ^ 0xc40d158d;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x47:
  case 0x49:
  case 0x4a:
    *UNRECOVERED_JUMPTABLE = (code)((char)*UNRECOVERED_JUMPTABLE + (char)cVar5);
    return;
  case 0x48:
    in_RCX = in_RCX + -1;
    if (in_RCX == (code *)0x0 || bVar22) {
      *unaff_RDI = *unaff_RDI >> 6 | *unaff_RDI << 2;
      *(int *)(unaff_RBX & 0xffffffff) = *(int *)(unaff_RBX & 0xffffffff) + -0x48000001;
    }
    else {
      (&UNK_142ab2c05)[uVar9] = (&UNK_142ab2c05)[uVar9] + (char)in_RCX;
    }
code_r0x000140946650:
                    // WARNING: Could not recover jumptable at 0x000140946664. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&UNK_142ab2d40 + *(int *)(&UNK_142ab2d40 + (ulonglong)*(uint *)(in_RCX + 0x128) * 4))
    )();
    return;
  case 0x4b:
    goto code_r0x000140946a3b;
  case 0x4c:
  case 0x4e:
  case 0x4f:
    DAT_142ab2c78 = 0xbb;
    return;
  case 0x4d:
    DAT_142ab2c78 = 0xbb;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x50:
    *UNRECOVERED_JUMPTABLE = (code)((char)*UNRECOVERED_JUMPTABLE + (char)cVar5);
    *unaff_RDI = *unaff_RDI + bVar13;
  case 0x51:
  case 0x53:
  case 0x54:
    UNRECOVERED_JUMPTABLE = (code *)(unaff_RBX & 0xffffffff);
    *(uint *)UNRECOVERED_JUMPTABLE = *(uint *)UNRECOVERED_JUMPTABLE + 0x48000001;
    puVar15 = (undefined *)0x42ab2da4;
code_r0x000140946a3b:
                    // WARNING: Could not recover jumptable at 0x000140946a42. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(puVar15 + *(int *)(puVar15 + (longlong)UNRECOVERED_JUMPTABLE * 4)))();
    return;
  case 0x52:
    goto code_r0x000140946796;
  case 0x55:
    DAT_142ab2c78 = 0xbb;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x57:
    lVar10 = CONCAT71(uVar12,(char)cVar5 + 0x82);
code_r0x0001409468d1:
                    // WARNING: Could not recover jumptable at 0x0001409468d4. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(puVar15 + lVar10))();
    return;
  case 0x5a:
    DAT_142ab2c78 = 0xbb;
    return;
  case 0x5b:
  case 0x5d:
  case 0x5e:
    if (in_RCX + -1 == (code *)0x0 || cVar16 == '\0') {
      *unaff_RDI = *unaff_RDI >> 6 | *unaff_RDI << 2;
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    }
    pbVar2 = &UNK_142ab2c05 + uVar9;
    bVar13 = *pbVar2;
    bVar4 = (byte)(in_RCX + -1);
    *pbVar2 = *pbVar2 + bVar4;
    UNRECOVERED_JUMPTABLE = (code *)(ulonglong)(uVar6 + 0x216c3b5 + (uint)CARRY1(bVar13,bVar4));
    goto code_r0x000140946a17;
  case 0x5c:
    DAT_142ab2c78 = 0xbb;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x5f:
    goto code_r0x0001409469eb;
  case 0x60:
  case 0x62:
  case 99:
code_r0x0001409469eb:
    DAT_142ab2c78 = 0xbb;
    return;
  case 0x61:
    goto code_r0x00014094697c;
  case 100:
    uVar9 = CONCAT71(uVar12,(char)cVar5 + 0x82);
    goto code_r0x0001409469d1;
  case 0x65:
  case 0x67:
  case 0x68:
    DAT_142ab2c78 = 0xbb;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x66:
    *UNRECOVERED_JUMPTABLE = (code)((char)*UNRECOVERED_JUMPTABLE + (char)cVar5);
    goto code_r0x00014094651d;
  case 0x69:
    goto code_r0x0001409469c3;
  case 0x6a:
  case 0x6c:
  case 0x6d:
code_r0x0001409469c3:
    DAT_142ab2c78 = 0xbb;
    return;
  case 0x6b:
    *(undefined1 *)((ulonglong)&stack0x00000000 & 0xffffffff) =
         *(undefined1 *)((ulonglong)&stack0x00000000 & 0xffffffff);
    *unaff_RDI = *unaff_RDI + bVar13;
    uVar9 = unaff_RBX & 0xffffffff;
    *(longlong *)(((ulonglong)UNRECOVERED_JUMPTABLE & 0xffffffff) - 8) = in_R8;
code_r0x0001409469d1:
                    // WARNING: Could not recover jumptable at 0x0001409469df. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&UNK_142ab2f5c + *(int *)(&UNK_142ab2f5c + uVar9 * 4)))();
    return;
  case 0x6e:
    goto code_r0x0001409469af;
  case 0x6f:
  case 0x71:
  case 0x72:
    *UNRECOVERED_JUMPTABLE = (code)((char)*UNRECOVERED_JUMPTABLE + (char)cVar5);
code_r0x0001409469af:
    pbVar2 = &UNK_142ab2c05 + uVar9;
    bVar4 = *pbVar2;
    *pbVar2 = *pbVar2 + bVar13;
    uVar6 = uVar6 + 0x216c505 + (uint)CARRY1(bVar4,bVar13);
code_r0x0001409469b7:
                    // WARNING: Could not recover jumptable at 0x0001409469be. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(puVar15 + *(int *)(puVar15 + (ulonglong)uVar6 * 4)))();
    return;
  case 0x73:
    uVar9 = (ulonglong)(uVar6 + 0x42ab2c78);
    goto code_r0x00014094699a;
  case 0x74:
  case 0x76:
  case 0x77:
    uVar9 = CONCAT71(uVar12,(char)cVar5 + 0x82);
code_r0x00014094699a:
                    // WARNING: Could not recover jumptable at 0x00014094699d. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(puVar15 + uVar9))();
    return;
  case 0x78:
    bVar13 = in(0x2c78);
    *unaff_RDI = bVar13;
  case 0x79:
  case 0x7b:
  case 0x7c:
    return;
  case 0x7a:
    DAT_142ab2c78 = 0xbb;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x7d:
    uVar6 = uVar6 + 0x216c5a8 + (uint)bVar20;
code_r0x000140946978:
    UNRECOVERED_JUMPTABLE = (code *)(longlong)*(int *)(puVar15 + (ulonglong)uVar6 * 4);
code_r0x00014094697c:
                    // WARNING: Could not recover jumptable at 0x00014094697f. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(UNRECOVERED_JUMPTABLE + (longlong)puVar15))();
    return;
  case 0x7e:
  case 0x80:
  case 0x81:
    goto code_r0x000140946970;
  case 0x7f:
    goto code_r0x000140946874;
  case 0x82:
    if (in_RCX == (code *)0x1 || bVar22) {
      register0x00000020 = (BADSPACEBASE *)&stack0xfffffffffffffff8;
      *unaff_RDI = *unaff_RDI >> 6 | *unaff_RDI << 2;
      *(int *)(UNRECOVERED_JUMPTABLE + uVar9 + 0xd5ba2c78) =
           *(int *)(UNRECOVERED_JUMPTABLE + uVar9 + 0xd5ba2c78) + -0x418bc3c0;
    }
code_r0x000140946970:
    *(undefined8 *)((longlong)register0x00000020 + -8) = 0xffffffffa8158d48;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x83:
  case 0x85:
  case 0x86:
    UNRECOVERED_JUMPTABLE = (code *)(ulonglong)(uVar6 + 0x42ab2c78);
code_r0x00014094695e:
                    // WARNING: Could not recover jumptable at 0x00014094695e. Too many branches
                    // WARNING: Treating indirect jump as call
    (*UNRECOVERED_JUMPTABLE)();
    return;
  case 0x84:
    UNRECOVERED_JUMPTABLE = (code *)(ulonglong)(uVar6 + 0x42ab2c78);
code_r0x00014094675a:
                    // WARNING: Could not recover jumptable at 0x00014094675a. Too many branches
                    // WARNING: Treating indirect jump as call
    (*UNRECOVERED_JUMPTABLE)();
    return;
  case 0x87:
    goto code_r0x00014094694b;
  case 0x88:
  case 0x8a:
  case 0x8b:
code_r0x00014094694b:
    *(int *)(uVar9 + 0x18aab2c78) = *(int *)(uVar9 + 0x18aab2c78) + 0x3ad6ea73;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x89:
    goto code_r0x000140946702;
  case 0x8c:
    uVar9 = (ulonglong)*(uint *)(&DAT_142ab2c78 + (longlong)UNRECOVERED_JUMPTABLE * 4);
    goto code_r0x000140946936;
  case 0x8d:
  case 0x8f:
  case 0x90:
    DAT_142ab2c78 = 0xbb;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x8e:
  case 0xba:
  case 0xbc:
  case 0xbd:
    DAT_142ab2c78 = 0xbb;
    return;
  case 0x91:
    goto code_r0x000140946923;
  case 0x92:
  case 0x94:
  case 0x95:
    *unaff_RDI = *unaff_RDI >> 6 | *unaff_RDI << 2;
code_r0x000140946923:
    *(uint *)(UNRECOVERED_JUMPTABLE + uVar9 + 0xd5ba2c78) =
         *(uint *)(UNRECOVERED_JUMPTABLE + uVar9 + 0xd5ba2c78) & 0x418bc3c0;
    pbVar2 = &UNK_142ab2c05 + uVar9;
    bVar4 = *pbVar2;
    *pbVar2 = *pbVar2 - bVar13;
    uVar9 = (ulonglong)(uVar6 + 0x216c68a + (uint)(bVar4 < bVar13));
code_r0x000140946936:
                    // WARNING: Could not recover jumptable at 0x00014094693d. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(puVar15 + *(int *)(puVar15 + uVar9 * 4)))();
    return;
  case 0x93:
    if (in_RCX == (code *)0x1 || bVar22) {
      *unaff_RDI = *unaff_RDI >> 6 | *unaff_RDI << 2;
      return;
    }
    uVar6 = uVar6 + 0x216c842 + (uint)bVar20;
code_r0x000140946792:
    UNRECOVERED_JUMPTABLE = (code *)(longlong)*(int *)(puVar15 + (ulonglong)uVar6 * 4);
code_r0x000140946796:
                    // WARNING: Could not recover jumptable at 0x000140946799. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(UNRECOVERED_JUMPTABLE + (longlong)puVar15))();
    return;
  case 0x96:
    puVar15 = (undefined *)0x42ab2e94;
    goto code_r0x000140946915;
  case 0x97:
  case 0x99:
  case 0x9a:
    *UNRECOVERED_JUMPTABLE = (code)((char)*UNRECOVERED_JUMPTABLE + (char)cVar5);
code_r0x00014094690e:
    puVar15 = &UNK_142ab2e94;
code_r0x000140946915:
                    // WARNING: Could not recover jumptable at 0x00014094691c. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(puVar15 + *(int *)(puVar15 + (longlong)UNRECOVERED_JUMPTABLE * 4)))();
    return;
  case 0x98:
    DAT_142ab2c78 = 0xbb;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x9b:
    *unaff_RDI = *unaff_RDI >> 6 | *unaff_RDI << 2;
    *(uint *)(UNRECOVERED_JUMPTABLE + uVar9 + 0xd5ba2c78) =
         *(uint *)(UNRECOVERED_JUMPTABLE + uVar9 + 0xd5ba2c78) | 0x818bc3c0;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x9c:
  case 0x9e:
  case 0x9f:
    goto code_r0x0001409468f8;
  case 0x9d:
    UNRECOVERED_JUMPTABLE = (code *)swi(1);
    (*UNRECOVERED_JUMPTABLE)();
    return;
  case 0xa0:
    *UNRECOVERED_JUMPTABLE = (code)((char)*UNRECOVERED_JUMPTABLE + (char)cVar5);
    pbVar2 = &UNK_142ab2c05 + uVar9;
    bVar4 = *pbVar2;
    *pbVar2 = *pbVar2 + bVar13;
    UNRECOVERED_JUMPTABLE = (code *)(ulonglong)(uVar6 + 0x216c5df + (uint)CARRY1(bVar4,bVar13));
    goto code_r0x0001409468f1;
  case 0xa2:
    (**(code **)(uVar9 + 0x1636fafc0))();
    return;
  case 0xa5:
    if (in_RCX == (code *)0x1 || cVar16 == '\0') {
      *unaff_RDI = *unaff_RDI >> 6 | *unaff_RDI << 2;
      puVar1 = (uint *)((CONCAT62(uVar17,CONCAT11(cVar16,bVar4)) & 0xffffffff) + 0x48000000);
      *puVar1 = *puVar1 | 0xc5df158d;
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    }
    *UNRECOVERED_JUMPTABLE = cVar5;
    *UNRECOVERED_JUMPTABLE = (code)((char)*UNRECOVERED_JUMPTABLE + (char)cVar5);
code_r0x0001409468ea:
    puVar15 = &UNK_142ab2ed0;
code_r0x0001409468f1:
    UNRECOVERED_JUMPTABLE =
         (code *)(puVar15 + *(int *)(puVar15 + (longlong)UNRECOVERED_JUMPTABLE * 4));
code_r0x0001409468f8:
                    // WARNING: Could not recover jumptable at 0x0001409468f8. Too many branches
                    // WARNING: Treating indirect jump as call
    (*UNRECOVERED_JUMPTABLE)();
    return;
  case 0xa6:
  case 0xa8:
  case 0xa9:
    DAT_142ab2c78 = 0xbb;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0xa7:
    *(uint *)UNRECOVERED_JUMPTABLE = *(uint *)UNRECOVERED_JUMPTABLE + (int)&stack0x00000000;
    *(char *)(unaff_RBP + 1) = *(char *)(unaff_RBP + 1) + ',';
    goto LAB_1409464ac;
  case 0xaa:
    goto code_r0x0001409468bf;
  case 0xab:
  case 0xad:
  case 0xae:
code_r0x0001409468bf:
    DAT_142ab2c78 = 0xbb;
    return;
  case 0xac:
    *UNRECOVERED_JUMPTABLE = (code)((char)*UNRECOVERED_JUMPTABLE + (char)cVar5);
    *unaff_RDI = *unaff_RDI + bVar13;
    piVar11 = (int *)(unaff_RBX & 0xffffffff);
    *piVar11 = *piVar11 + 0x48000001 + (uint)(((byte)cVar5 >> 5 & 1) != 0);
    puVar15 = (undefined *)0x42ab2d7c;
code_r0x0001409466ed:
                    // WARNING: Could not recover jumptable at 0x0001409466f4. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(puVar15 + *(int *)(puVar15 + (longlong)piVar11 * 4)))();
    return;
  case 0xaf:
    uVar9 = CONCAT71(uVar12,(char)cVar5 + 0x82);
    goto code_r0x0001409468ac;
  case 0xb0:
  case 0xb2:
  case 0xb3:
    DAT_142ab2c78 = 0xbb;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0xb1:
    DAT_142ab2c78 = 0xbb;
    return;
  case 0xb4:
    DAT_142ab2c78 = 0xbb;
    return;
  case 0xb5:
  case 0xb7:
  case 0xb8:
    bVar21 = bVar21 & 1;
    if (in_RCX + -1 == (code *)0x0 || cVar16 == '\0') {
      *unaff_RDI = *unaff_RDI >> 6 | *unaff_RDI << 2;
      UNRECOVERED_JUMPTABLE = (code *)(CONCAT62(uVar17,CONCAT11(cVar16,bVar4)) & 0xffffffff);
      bVar21 = 0;
      *(byte *)(in_R8 + -0x73) = *(byte *)(in_R8 + -0x73) | (byte)(in_RCX + -1);
    }
    uVar9 = (ulonglong)((int)UNRECOVERED_JUMPTABLE + 0x216c764 + (uint)bVar21);
code_r0x0001409468ac:
                    // WARNING: Could not recover jumptable at 0x0001409468b3. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(puVar15 + *(int *)(puVar15 + uVar9 * 4)))();
    return;
  case 0xb6:
LAB_1409464ac:
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::DoUiEditRectangleRotate>
              ();
    break;
  case 0xb9:
    DAT_142ab2c78 = 0xbb;
    return;
  case 0xbb:
    *(uint *)UNRECOVERED_JUMPTABLE = *(uint *)UNRECOVERED_JUMPTABLE + uVar6;
    *unaff_RDI = *unaff_RDI + bVar13;
    UNRECOVERED_JUMPTABLE = (code *)(unaff_RBX & 0xffffffff);
code_r0x000140946702:
    *(uint *)(UNRECOVERED_JUMPTABLE + 0x48000000) =
         *(uint *)(UNRECOVERED_JUMPTABLE + 0x48000000) + 0xc7d3158d;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0xbe:
code_r0x000140946872:
    UNRECOVERED_JUMPTABLE = (code *)CONCAT71(uVar12,(char)cVar5 + 0x82);
    goto code_r0x000140946874;
  case 0xbf:
  case 0xc1:
  case 0xc2:
    if (-1 < (longlong)UNRECOVERED_JUMPTABLE) {
      DAT_142ab2c78 = 0xbb;
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    }
    uVar9 = (ulonglong)*(uint *)(&DAT_142ab2c78 + (longlong)UNRECOVERED_JUMPTABLE * 4);
    goto code_r0x000140946838;
  case 0xc0:
    *UNRECOVERED_JUMPTABLE = (code)((char)*UNRECOVERED_JUMPTABLE + (char)cVar5);
    (&UNK_142ab2c01)[uVar9] = (&UNK_142ab2c01)[uVar9] + bVar13;
    unaff_RDI[0x100ff300] = unaff_RDI[0x100ff300] - bVar21;
    break;
  case 0xc3:
    DAT_142ab2c78 = 0xbb;
    return;
  case 0xc4:
  case 0xc6:
  case 199:
    bVar20 = (bool)(bVar21 & 1);
    if (in_RCX + -1 == (code *)0x0 || cVar16 == '\0') {
      *unaff_RDI = *unaff_RDI >> 6 | *unaff_RDI << 2;
      if (cVar16 < '\0') goto code_r0x000140946872;
      UNRECOVERED_JUMPTABLE = (code *)(CONCAT62(uVar17,CONCAT11(cVar16,bVar4)) & 0xffffffff);
      bVar22 = ((byte)cVar5 >> 5 & 1) != 0;
      pbVar2 = (byte *)(in_R8 + -0x73);
      bVar4 = (byte)(in_RCX + -1);
      bVar13 = *pbVar2 - bVar4;
      bVar20 = *pbVar2 < bVar4 || bVar13 < bVar22;
      *pbVar2 = bVar13 - bVar22;
    }
    uVar6 = (int)UNRECOVERED_JUMPTABLE + 0x216c778 + (uint)bVar20;
code_r0x000140946870:
    UNRECOVERED_JUMPTABLE = (code *)(longlong)*(int *)(puVar15 + (ulonglong)uVar6 * 4);
code_r0x000140946874:
                    // WARNING: Could not recover jumptable at 0x000140946877. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(UNRECOVERED_JUMPTABLE + (longlong)puVar15))();
    return;
  case 0xc5:
    pcVar14 = in_RCX + -1;
    if (pcVar14 == (code *)0x0 || cVar16 == '\0') {
      *unaff_RDI = *unaff_RDI >> 6 | *unaff_RDI << 2;
      *(uint *)(pcVar14 + (longlong)UNRECOVERED_JUMPTABLE) =
           *(int *)(pcVar14 + (longlong)UNRECOVERED_JUMPTABLE) + -0x6cf10000 +
           (uint)((char)*unaff_RDI < '\0');
      goto code_r0x000140946702;
    }
    *UNRECOVERED_JUMPTABLE = *UNRECOVERED_JUMPTABLE;
    pbVar2 = &UNK_142ab2c05 + uVar9;
    bVar13 = *pbVar2;
    *pbVar2 = *pbVar2 + (byte)pcVar14;
    uVar6 = uVar6 + 0x216c7d3 + (uint)CARRY1(bVar13,(byte)pcVar14);
code_r0x000140946711:
                    // WARNING: Could not recover jumptable at 0x000140946718. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(puVar15 + *(int *)(puVar15 + (ulonglong)uVar6 * 4)))();
    return;
  case 200:
    goto code_r0x000140946847;
  case 0xc9:
  case 0xcb:
  case 0xcc:
code_r0x000140946847:
    DAT_142ab2c78 = 0xbb;
    return;
  case 0xca:
    *UNRECOVERED_JUMPTABLE = (code)((char)*UNRECOVERED_JUMPTABLE + (char)cVar5);
    (&UNK_142ab2c01)[uVar9] = (&UNK_142ab2c01)[uVar9] + bVar13;
    pcVar8 = (char *)(ulonglong)(uVar6 >> 0x1e);
    pcVar8[-0x39] = pcVar8[-0x39] + bVar13;
    bVar4 = (byte)((ulonglong)UNRECOVERED_JUMPTABLE >> 0x18);
    bVar21 = bVar4 >> 6;
    *pcVar8 = *pcVar8 + bVar21;
    *pcVar8 = *pcVar8 + bVar21;
    bVar4 = bVar4 >> 6;
    *pcVar8 = *pcVar8 + bVar4;
    *pcVar8 = *pcVar8 + bVar4;
    *pcVar8 = *pcVar8 + bVar4;
    *pcVar8 = *pcVar8 + bVar4;
    pcVar8[-0x73] = pcVar8[-0x73] + bVar13;
    UNRECOVERED_JUMPTABLE = (code *)((ulonglong)unaff_RBP & 0xffffffff);
code_r0x00014094659a:
    LOCK();
    *UNRECOVERED_JUMPTABLE = (code)((char)*UNRECOVERED_JUMPTABLE + (char)UNRECOVERED_JUMPTABLE);
    UNLOCK();
    UNRECOVERED_JUMPTABLE[-0x77] = (code)((char)UNRECOVERED_JUMPTABLE[-0x77] + bVar13);
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0xcd:
    uVar9 = CONCAT71(uVar12,(char)cVar5 + 0x82);
    goto code_r0x000140946838;
  case 0xce:
  case 0xd0:
  case 0xd1:
    puVar18 = (undefined8 *)&stack0xfffffffffffffff8;
    cVar16 = '\x01';
    do {
      unaff_RBP = unaff_RBP + -1;
      puVar18 = puVar18 + -1;
      *puVar18 = *unaff_RBP;
      cVar16 = cVar16 + -1;
    } while ('\0' < cVar16);
code_r0x000140946834:
    uVar9 = (ulonglong)*(int *)(puVar15 + (longlong)UNRECOVERED_JUMPTABLE * 4);
code_r0x000140946838:
                    // WARNING: Could not recover jumptable at 0x00014094683b. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(puVar15 + uVar9))();
    return;
  case 0xcf:
    *(int *)(in_RCX + (longlong)UNRECOVERED_JUMPTABLE) =
         *(int *)(in_RCX + (longlong)UNRECOVERED_JUMPTABLE) + 0x6cf10000;
    *(uint *)UNRECOVERED_JUMPTABLE =
         (*(uint *)UNRECOVERED_JUMPTABLE + 0xb7ffffff) - (uint)((bVar4 >> 5 & 1) != 0);
    puVar15 = (undefined *)0x42ab2d68;
code_r0x000140946681:
                    // WARNING: Could not recover jumptable at 0x000140946688. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(puVar15 + *(int *)(puVar15 + (longlong)UNRECOVERED_JUMPTABLE * 4)))();
    return;
  case 0xd2:
    goto code_r0x00014094681f;
  case 0xd3:
  case 0xd5:
  case 0xd6:
code_r0x00014094681f:
    DAT_142ab2c78 = 0xbb;
    return;
  case 0xd7:
    goto code_r0x00014094680b;
  case 0xd8:
  case 0xda:
  case 0xdb:
    goto code_r0x000140946808;
  case 0xd9:
    pbVar2 = &UNK_142ab2c05 + uVar9;
    *pbVar2 = *pbVar2 >> 1 | *pbVar2 << 7;
    uVar9 = CONCAT71(uVar12,(char)cVar5 + 0x40);
code_r0x000140946436:
    if (*(longlong *)(*(longlong *)(unaff_RSI + 0x1e0) + 0x20 + ~uVar9 * 0x60) == 0)
    goto LAB_140946450;
    goto LAB_140946458;
  case 0xdc:
    in_RCX = in_RCX + -1;
    if (in_RCX == (code *)0x0 || bVar22) {
      *unaff_RDI = *unaff_RDI >> 6 | *unaff_RDI << 2;
      uVar9 = (ulonglong)((int)unaff_RBX + 0x48000000);
      puVar15 = (undefined *)0x42ab2e44;
      goto code_r0x000140946813;
    }
code_r0x000140946808:
    *UNRECOVERED_JUMPTABLE = *UNRECOVERED_JUMPTABLE;
code_r0x00014094680b:
    pbVar2 = &UNK_142ab2c05 + uVar9;
    bVar13 = *pbVar2;
    *pbVar2 = *pbVar2 + (byte)in_RCX;
    uVar9 = (ulonglong)(uVar6 + 0x216c631 + (uint)CARRY1(bVar13,(byte)in_RCX));
code_r0x000140946813:
                    // WARNING: Could not recover jumptable at 0x00014094681a. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(puVar15 + *(int *)(puVar15 + uVar9 * 4)))();
    return;
  case 0xdd:
  case 0xdf:
  case 0xe0:
    uVar9 = (ulonglong)(uVar6 + 0x42ab2c78);
    goto code_r0x0001409467ef;
  case 0xde:
    goto code_r0x000140946509;
  case 0xe1:
    goto code_r0x0001409467e3;
  case 0xe2:
  case 0xe4:
  case 0xe5:
code_r0x0001409467e3:
    uVar9 = (ulonglong)(uVar6 | 0x48000000);
    puVar15 = (undefined *)0x42ab2e30;
code_r0x0001409467ef:
                    // WARNING: Could not recover jumptable at 0x0001409467f6. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(puVar15 + *(int *)(puVar15 + uVar9 * 4)))();
    return;
  case 0xe3:
                    // WARNING (jumptable): Read-only address (ram,0x000142ab2c78) is written
                    // WARNING: Read-only address (ram,0x000142ab2c78) is written
    DAT_142ab2c78 = (char)cVar5 + 0xbb;
    break;
  case 0xe6:
    uVar9 = (ulonglong)*(uint *)(&DAT_142ab2c78 + (longlong)UNRECOVERED_JUMPTABLE * 4);
    goto code_r0x0001409467ce;
  case 0xe7:
  case 0xe9:
  case 0xea:
    DAT_142ab2c78 = 0xbb;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0xe8:
    goto code_r0x00014094651d;
  case 0xeb:
code_r0x0001409467bb:
    *unaff_RDI = *unaff_RDI >> 6 | *unaff_RDI << 2;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0xec:
  case 0xee:
  case 0xef:
    if (in_RCX == (code *)0x1 || bVar22) goto code_r0x0001409467bb;
    uVar9 = (ulonglong)(uVar6 + 0x216c7a2 + (uint)bVar20);
code_r0x0001409467ce:
                    // WARNING: Could not recover jumptable at 0x0001409467d5. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(puVar15 + *(int *)(puVar15 + uVar9 * 4)))();
    return;
  case 0xed:
    *unaff_RSI = (byte)((ulonglong)in_RCX >> 8);
    in_RCX[0x16] = (code)((byte)in_RCX[0x16] & bVar4);
    bevy_ecs::component::Components::get_resource_id();
    if ((((extraout_RAX == 0) || (*(undefined **)(unaff_RSI + 0x218) <= puVar15)) ||
        (uVar9 = *(ulonglong *)(*(longlong *)(unaff_RSI + 0x210) + (longlong)puVar15 * 8),
        uVar9 == 0)) ||
       (*(longlong *)(*(longlong *)(unaff_RSI + 0x1e0) + 0x20 + ~uVar9 * 0x60) == 0)) {
      bevy_ecs::event::registry::EventRegistry::
      register_event<country_core::systems::wall::shapes::rectangle::FinishEditRectangleCmd>();
    }
    uVar9 = 0x4a1db5837d11c93f;
    bevy_ecs::component::Components::get_resource_id();
    if (((extraout_RAX_00 == 0) || (*(ulonglong *)(unaff_RSI + 0x218) <= uVar9)) ||
       ((uVar9 = *(ulonglong *)(*(longlong *)(unaff_RSI + 0x210) + uVar9 * 8), uVar9 == 0 ||
        (*(longlong *)(*(longlong *)(unaff_RSI + 0x1e0) + 0x20 + ~uVar9 * 0x60) == 0)))) {
      bevy_ecs::event::registry::EventRegistry::
      register_event<country_core::systems::wall::shapes::circle::create_circle::CreateCircleCmd>();
    }
    uVar9 = 0x58adde1121b70c78;
    bevy_ecs::component::Components::get_resource_id();
    if (((extraout_RAX_01 == 0) || (*(ulonglong *)(unaff_RSI + 0x218) <= uVar9)) ||
       ((uVar9 = *(ulonglong *)(*(longlong *)(unaff_RSI + 0x210) + uVar9 * 8), uVar9 == 0 ||
        (*(longlong *)(*(longlong *)(unaff_RSI + 0x1e0) + 0x20 + ~uVar9 * 0x60) == 0)))) {
      bevy_ecs::event::registry::EventRegistry::
      register_event<country_core::systems::wall::shapes::shape_placement::DontRecalculateShapePlacement>
                ();
    }
    uVar9 = 0xf4e90bdaaff68cb;
    bevy_ecs::component::Components::get_resource_id();
    if ((((extraout_RAX_02 == 0) || (*(ulonglong *)(unaff_RSI + 0x218) <= uVar9)) ||
        (uVar9 = *(ulonglong *)(*(longlong *)(unaff_RSI + 0x210) + uVar9 * 8), uVar9 == 0)) ||
       (*(longlong *)(*(longlong *)(unaff_RSI + 0x1e0) + 0x20 + ~uVar9 * 0x60) == 0)) {
      bevy_ecs::event::registry::EventRegistry::
      register_event<country_core::systems::ui_raise_shape::HandleRaiseShapeUiInteraction>();
    }
    uVar9 = 0xfbdd85d7ac4b26cb;
    bevy_ecs::component::Components::get_resource_id();
    if (((extraout_RAX_03 != 0) && (uVar9 < *(ulonglong *)(unaff_RSI + 0x218))) &&
       (uVar9 = *(ulonglong *)(*(longlong *)(unaff_RSI + 0x210) + uVar9 * 8), uVar9 != 0))
    goto code_r0x000140946436;
LAB_140946450:
    bevy_ecs::event::registry::EventRegistry::
    register_event<country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::DoUiEditRectangleDims>
              ();
LAB_140946458:
    uVar9 = 0x23ea1ac5bdb3f981;
    bevy_ecs::component::Components::get_resource_id();
    if (((extraout_RAX_04 == 0) || (*(ulonglong *)(unaff_RSI + 0x218) <= uVar9)) ||
       ((uVar9 = *(ulonglong *)(*(longlong *)(unaff_RSI + 0x210) + uVar9 * 8), uVar9 == 0 ||
        (*(longlong *)(*(longlong *)(unaff_RSI + 0x1e0) + 0x20 + ~uVar9 * 0x60) == 0))))
    goto LAB_1409464ac;
    break;
  case 0xf0:
    piVar11 = (int *)(&UNK_142ab2c80 + (longlong)in_RCX * 4 + uVar9);
    *piVar11 = unaff_XMM6_Da;
    piVar11[1] = unaff_XMM6_Db;
    piVar11[2] = unaff_XMM6_Dc;
    piVar11[3] = unaff_XMM6_Dd;
    goto LAB_140947afb;
  case 0xf1:
    DAT_142ab2c78 = 0xbb;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0xf2:
    puVar19 = auStackX_8;
    register0x00000020 = (BADSPACEBASE *)auStackX_8;
    if (in_RCX + -1 != (code *)0x0 && !bVar22) {
      bVar13 = (byte)(in_RCX + -1);
      if (!bVar22) {
        (&UNK_142ab2c77)[uVar9] = (&UNK_142ab2c77)[uVar9] | bVar13;
        return;
      }
      (&UNK_142ab2c05)[uVar9] = (&UNK_142ab2c05)[uVar9] + bVar13 + bVar20;
code_r0x000140947aeb:
      *(undefined8 *)(puVar19 + -8) = 0x140947afb;
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
      register0x00000020 = (BADSPACEBASE *)puVar19;
      unaff_RBP = unaff_retaddr;
      goto LAB_140947afb;
    }
    unaff_RSI = (byte *)(ulonglong)*(uint *)(unaff_RDI + 0x128);
    unaff_RBP = unaff_retaddr;
    goto joined_r0x000140946fb4;
  case 0xf3:
    in_RCX[-0x7d] = (code)((char)in_RCX[-0x7d] + (char)cVar5);
    unaff_RDI[(longlong)in_RCX] = unaff_RDI[(longlong)in_RCX] + 1;
    unaff_XMM6_Da = *(int *)(unaff_RDI + 300);
    goto code_r0x000140947b34;
  case 0xf4:
    uVar9 = (ulonglong)UNRECOVERED_JUMPTABLE | 0x5b;
    *(undefined2 *)(uVar9 + (longlong)in_RCX * 4) = 0x2301;
    *(int *)(uVar9 + 4 + (longlong)in_RCX * 4) = unaff_R14D;
    *(int *)(uVar9 + 8 + (longlong)in_RCX * 4) = unaff_XMM6_Da;
    unaff_RBX = unaff_RBX + 1;
    unaff_RBP[-4] = unaff_RBX;
    unaff_R14D = *(int *)(unaff_RDI + 0x120);
    if (unaff_R14D == 4) {
      unaff_R14D = *(int *)(unaff_RDI + 0x128);
      goto joined_r0x000140946fb4;
    }
    unaff_XMM6_Da = *(int *)(unaff_RDI + 0x124);
    unaff_retaddr = unaff_RBP;
    if (unaff_RBX == unaff_RBP[-6]) goto code_r0x000140947aeb;
LAB_140947afb:
    lVar10 = unaff_RBP[-5];
    *(undefined2 *)(lVar10 + unaff_RBX * 0xc) = 0x2401;
    *(int *)(lVar10 + 4 + unaff_RBX * 0xc) = unaff_R14D;
    *(int *)(lVar10 + 8 + unaff_RBX * 0xc) = unaff_XMM6_Da;
    unaff_RBX = unaff_RBX + 1;
    unaff_RBP[-4] = unaff_RBX;
    unaff_R14D = *(int *)(unaff_RDI + 0x128);
joined_r0x000140946fb4:
    if (unaff_R14D != 4) {
      unaff_XMM6_Da = *(int *)(unaff_RDI + 300);
      goto code_r0x000140947b34;
    }
    unaff_R14D = *(int *)(unaff_RDI + 0x130);
    unaff_retaddr = unaff_RBP;
    goto joined_r0x000140947b75;
  case 0xf5:
    *UNRECOVERED_JUMPTABLE = (code)((char)*UNRECOVERED_JUMPTABLE + (char)cVar5);
code_r0x000140947b34:
    if (unaff_RBX == unaff_RBP[-6]) {
      *(undefined8 *)((longlong)register0x00000020 + -8) = 0x140947b4a;
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    lVar10 = unaff_RBP[-5];
    *(undefined2 *)(lVar10 + unaff_RBX * 0xc) = 0x2501;
    *(int *)(lVar10 + 4 + unaff_RBX * 0xc) = unaff_R14D;
    *(int *)(lVar10 + 8 + unaff_RBX * 0xc) = unaff_XMM6_Da;
    unaff_RBX = unaff_RBX + 1;
    unaff_RBP[-4] = unaff_RBX;
    unaff_R14D = *(int *)(unaff_RDI + 0x130);
    unaff_retaddr = unaff_RBP;
joined_r0x000140947b75:
    if (unaff_R14D == 4) {
                    // WARNING: Read-only address (ram,0x000142ab2c78) is written
      unaff_R14D = *(int *)(unaff_RDI + 0x138);
    }
    else {
                    // WARNING: Read-only address (ram,0x000142ab2c78) is written
      unaff_XMM6_Da = *(int *)(unaff_RDI + 0x134);
      if (unaff_RBX == unaff_retaddr[-6]) {
code_r0x000140947b89:
        *(undefined8 *)((longlong)register0x00000020 + -8) = 0x140947b99;
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                  ();
      }
      lVar10 = unaff_retaddr[-5];
      *(undefined2 *)(lVar10 + unaff_RBX * 0xc) = 0x2601;
      *(int *)(lVar10 + 4 + unaff_RBX * 0xc) = unaff_R14D;
      *(int *)(lVar10 + 8 + unaff_RBX * 0xc) = unaff_XMM6_Da;
      unaff_RBX = unaff_RBX + 1;
      unaff_retaddr[-4] = unaff_RBX;
      unaff_R14D = *(int *)(unaff_RDI + 0x138);
    }
    if (unaff_R14D != 4) {
      unaff_XMM6_Da = *(int *)(unaff_RDI + 0x13c);
      if (unaff_RBX == unaff_retaddr[-6]) {
code_r0x000140947bd4:
        *(undefined8 *)((longlong)register0x00000020 + -8) = 0x140947be4;
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                  ();
      }
      lVar10 = unaff_retaddr[-5];
      *(undefined2 *)(lVar10 + unaff_RBX * 0xc) = 0x2701;
      *(int *)(lVar10 + 4 + unaff_RBX * 0xc) = unaff_R14D;
      *(int *)(lVar10 + 8 + unaff_RBX * 0xc) = unaff_XMM6_Da;
code_r0x000140947bfd:
      unaff_retaddr[-4] = unaff_RBX + 1;
    }
    *(undefined8 *)(unaff_RSI + 0x10) = unaff_retaddr[-4];
    uVar3 = unaff_retaddr[-5];
    *(undefined8 *)unaff_RSI = unaff_retaddr[-6];
    *(undefined8 *)(unaff_RSI + 8) = uVar3;
    return;
  case 0xf6:
    do {
                    // WARNING: Do nothing block with infinite loop
    } while( true );
  case 0xf7:
    *(int *)unaff_RDI = *(int *)unaff_RDI + -1;
    return;
  case 0xf8:
    puStackX_10 = &DAT_142ab2c78;
    register0x00000020 = (BADSPACEBASE *)auStack_58;
    goto code_r0x000140947c59;
  case 0xf9:
    if (bVar22) {
      DAT_142ab2c78 = 0xbb;
      return;
    }
    (&UNK_142ab2c03)[uVar9] = (&UNK_142ab2c03)[uVar9] & bVar13;
    pbVar2 = (byte *)(in_R8 + -0x7b);
    *pbVar2 = *pbVar2 >> 1 | *pbVar2 << 7;
    unaff_RDI[0x142ab2cc0] = 0;
    goto code_r0x000140947c59;
  case 0xfa:
    unaff_RBX = CONCAT62(uVar17,CONCAT11(0x3c,bVar4));
    *(uint *)UNRECOVERED_JUMPTABLE = *(uint *)UNRECOVERED_JUMPTABLE + uVar6;
    (&UNK_142ab2cb3)[uVar9] = (&UNK_142ab2cb3)[uVar9] + bVar13;
    register0x00000020 = (BADSPACEBASE *)auStackX_8;
    *(char *)(unaff_retaddr + 2) = *(char *)(unaff_retaddr + 2) << 1;
    goto code_r0x000140947bd4;
  case 0xfb:
    pbVar2 = unaff_RDI + 1;
    *unaff_RDI = *unaff_RSI;
    piVar11 = (int *)((ulonglong)UNRECOVERED_JUMPTABLE & 0xffffffffffffff80);
    *piVar11 = *piVar11 + (int)piVar11;
    (unaff_RDI + 0x2a)[(longlong)in_RCX] = (unaff_RDI + 0x2a)[(longlong)in_RCX] + (char)piVar11;
    piVar11 = (int *)((ulonglong)UNRECOVERED_JUMPTABLE & 0xffffffffffffff00);
    *piVar11 = *piVar11 + (int)piVar11;
    (unaff_RDI + 0x2a)[(longlong)in_RCX] = (unaff_RDI + 0x2a)[(longlong)in_RCX];
    puVar19 = (undefined1 *)((ulonglong)UNRECOVERED_JUMPTABLE & 0xffffff00);
    piVar11 = (int *)((ulonglong)&stack0xfffffffffffffff8 & 0xffffff60);
    iVar7 = (int)piVar11;
    *piVar11 = *piVar11 + iVar7;
    (unaff_RDI + 0x2a)[(longlong)in_RCX] = (unaff_RDI + 0x2a)[(longlong)in_RCX] + (char)piVar11;
    *(undefined2 *)(piVar11 + 0xa155963c) = in_FS;
    *piVar11 = *piVar11 + iVar7;
    (unaff_RDI + 0x2a)[(longlong)in_RCX] = (unaff_RDI + 0x2a)[(longlong)in_RCX] + (char)piVar11;
    *piVar11 = *piVar11 + iVar7;
    *pbVar2 = *pbVar2 + bVar13;
    *(int *)(puVar19 + 0x130) = *(int *)(puVar19 + 0x130) - (int)pbVar2;
    *(int *)(puVar19 + 0x120) = unaff_XMM6_Da;
    *(int *)(puVar19 + 0x124) = unaff_XMM6_Db;
    *(int *)(puVar19 + 0x128) = unaff_XMM6_Dc;
    *(int *)(puVar19 + 300) = unaff_XMM6_Dd;
    UNRECOVERED_JUMPTABLE = in_RCX;
  case 0xfc:
    *(uint *)UNRECOVERED_JUMPTABLE = *(uint *)UNRECOVERED_JUMPTABLE + (int)UNRECOVERED_JUMPTABLE;
    pcVar14 = UNRECOVERED_JUMPTABLE + -0x77;
    *pcVar14 = (code)((char)*pcVar14 + bVar13);
    cVar5 = *pcVar14;
    *(undefined8 **)(puVar19 + -8) = unaff_RBP;
    register0x00000020 = (BADSPACEBASE *)(puVar19 + -0x8555);
    if (cVar5 == (code)0x0) {
      *(undefined4 *)(puVar19 + -0x84d1) = 4;
      *(undefined4 *)(puVar19 + -0x84d5) = 4;
      *(undefined4 *)(puVar19 + -0x84d9) = 4;
      *(undefined4 *)(puVar19 + -0x84dd) = 4;
      *(undefined4 *)(puVar19 + -0x84e1) = 4;
      *(undefined4 *)(puVar19 + -0x84e5) = 4;
      *(undefined4 *)(puVar19 + -0x84e9) = 4;
      *(undefined4 *)(puVar19 + -0x84ed) = 4;
      *(undefined4 *)(puVar19 + -0x84f1) = 4;
      *(undefined4 *)(puVar19 + -0x84f5) = 4;
      *(undefined4 *)(puVar19 + -0x84f9) = 4;
      *(undefined4 *)(puVar19 + -0x84fd) = 4;
      *(undefined4 *)(puVar19 + -0x8501) = 4;
      *(undefined4 *)(puVar19 + -0x8505) = 4;
      *(undefined4 *)(puVar19 + -0x8509) = 4;
      *(undefined4 *)(puVar19 + -0x850d) = 4;
      *(undefined4 *)(puVar19 + -0x8511) = 4;
      *(undefined4 *)(puVar19 + -0x8515) = 4;
      *(undefined4 *)(puVar19 + -0x8519) = 4;
      *(undefined4 *)(puVar19 + -0x851d) = 4;
      *(undefined4 *)(puVar19 + -0x8521) = 4;
      *(undefined4 *)(puVar19 + -0x8525) = 4;
      *(undefined4 *)(puVar19 + -0x8529) = 4;
      *(undefined4 *)(puVar19 + -0x852d) = 4;
      *(undefined4 *)(puVar19 + -0x8531) = 4;
      *(undefined4 *)(puVar19 + -0x8535) = 4;
      *(undefined4 *)(puVar19 + -0x8539) = 4;
      *(undefined4 *)(puVar19 + -0x8541) = 4;
      *(undefined4 *)(puVar19 + -0x853d) = 4;
      *(undefined4 *)(puVar19 + -0x8545) = 4;
      *(undefined4 *)(puVar19 + -0x8549) = 4;
      *(undefined4 *)(puVar19 + -0x854d) = 4;
      *(uint *)UNRECOVERED_JUMPTABLE = 4;
      *(uint *)(UNRECOVERED_JUMPTABLE + 4) = *(uint *)(puVar19 + -0x84cd);
      *(uint *)(UNRECOVERED_JUMPTABLE + 8) = 4;
      *(uint *)(UNRECOVERED_JUMPTABLE + 0xc) = *(uint *)(puVar19 + -0x84c5);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x10) = 4;
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x14) = *(uint *)(puVar19 + -0x84c1);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x18) = 4;
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x1c) = *(uint *)(puVar19 + -0x84b9);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x20) = 4;
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x24) = *(uint *)(puVar19 + -0x84b1);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x28) = 4;
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x2c) = *(uint *)(puVar19 + -0x84a9);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x30) = 4;
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x34) = *(uint *)(puVar19 + -0x84a5);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x38) = 4;
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x3c) = *(uint *)(puVar19 + -0x849d);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x40) = *(uint *)(puVar19 + -0x854d);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x44) = *(uint *)(puVar19 + -0x8495);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x48) = *(uint *)(puVar19 + -0x8549);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x4c) = *(uint *)(puVar19 + -0x848d);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x50) = *(uint *)(puVar19 + -0x8545);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x54) = *(uint *)(puVar19 + -0x8489);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x58) = *(uint *)(puVar19 + -0x853d);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x5c) = *(uint *)(puVar19 + -0x8481);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x60) = *(uint *)(puVar19 + -0x8541);
      *(uint *)(UNRECOVERED_JUMPTABLE + 100) = *(uint *)(puVar19 + -0x8479);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x68) = *(uint *)(puVar19 + -0x8539);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x6c) = *(uint *)(puVar19 + -0x8471);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x70) = *(uint *)(puVar19 + -0x8535);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x74) = *(uint *)(puVar19 + -0x846d);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x78) = *(uint *)(puVar19 + -0x8531);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x7c) = unaff_XMM14_Da;
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x80) = *(uint *)(puVar19 + -0x852d);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x84) = unaff_XMM13_Da;
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x88) = *(uint *)(puVar19 + -0x8529);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x8c) = *(uint *)(puVar19 + -0x84c9);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x90) = *(uint *)(puVar19 + -0x8525);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x94) = *(uint *)(puVar19 + -0x84bd);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x98) = *(uint *)(puVar19 + -0x8521);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x9c) = *(uint *)(puVar19 + -0x84b5);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0xa0) = *(uint *)(puVar19 + -0x851d);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0xa4) = *(uint *)(puVar19 + -0x84ad);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0xa8) = *(uint *)(puVar19 + -0x8519);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0xac) = *(uint *)(puVar19 + -0x84a1);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0xb0) = *(uint *)(puVar19 + -0x8515);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0xb4) = *(uint *)(puVar19 + -0x8499);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0xb8) = *(uint *)(puVar19 + -0x8511);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0xbc) = *(uint *)(puVar19 + -0x8491);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0xc0) = *(uint *)(puVar19 + -0x850d);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0xc4) = *(uint *)(puVar19 + -0x8485);
      *(uint *)(UNRECOVERED_JUMPTABLE + 200) = *(uint *)(puVar19 + -0x8509);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0xcc) = *(uint *)(puVar19 + -0x847d);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0xd0) = *(uint *)(puVar19 + -0x8505);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0xd4) = *(uint *)(puVar19 + -0x8475);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0xd8) = *(uint *)(puVar19 + -0x8501);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0xdc) = *(uint *)(puVar19 + -0x8469);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0xe0) = *(uint *)(puVar19 + -0x84fd);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0xe4) = *(uint *)(puVar19 + -0x8465);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0xe8) = *(uint *)(puVar19 + -0x84f9);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0xec) = *(uint *)(puVar19 + -0x8461);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0xf0) = *(uint *)(puVar19 + -0x84f5);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0xf4) = *(uint *)(puVar19 + -0x845d);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0xf8) = *(uint *)(puVar19 + -0x84f1);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0xfc) = *(uint *)(puVar19 + -0x8459);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x100) = *(uint *)(puVar19 + -0x84ed);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x104) = *(uint *)(puVar19 + -0x8455);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x108) = *(uint *)(puVar19 + -0x84e9);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x10c) = *(uint *)(puVar19 + -0x8451);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x110) = *(uint *)(puVar19 + -0x84e5);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x114) = *(uint *)(puVar19 + -0x844d);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x118) = *(uint *)(puVar19 + -0x84e1);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x11c) = *(uint *)(puVar19 + -0x8449);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x120) = *(uint *)(puVar19 + -0x84dd);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x124) = *(uint *)(puVar19 + -0x8445);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x128) = *(uint *)(puVar19 + -0x84d9);
      *(uint *)(UNRECOVERED_JUMPTABLE + 300) = *(uint *)(puVar19 + -0x8441);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x130) = *(uint *)(puVar19 + -0x84d5);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x134) = *(uint *)(puVar19 + -0x843d);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x138) = *(uint *)(puVar19 + -0x84d1);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x13c) = *(uint *)(puVar19 + -0x8439);
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x140) = 0;
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x144) = 0;
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x148) = 0;
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x14c) = 0;
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x150) = 0;
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x154) = 0;
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x158) = 0;
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x15c) = 0;
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x160) = 0;
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x164) = 0;
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x168) = 0;
      *(uint *)(UNRECOVERED_JUMPTABLE + 0x16c) = 0;
      return;
    }
    puVar15 = &UNK_142ab2c7c;
    in_R9 = &DAT_142ab3050;
    in_R10 = &DAT_142ab3038;
code_r0x000140947d1f:
    *(undefined4 *)((longlong)register0x00000020 + 8) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0xc) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x10) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x18) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x14) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x1c) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x20) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x24) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x28) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x2c) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x30) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x34) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x38) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x3c) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x40) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x44) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x48) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x4c) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x50) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x54) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x58) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x5c) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x60) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 100) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x68) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x6c) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x70) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x74) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x78) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x7c) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x80) = 4;
    *(undefined4 *)((longlong)register0x00000020 + 0x84) = 4;
    if (puVar15[-4] == '\0') {
                    // WARNING: Could not recover jumptable at 0x000140947ecd. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(in_R10 + *(int *)(in_R10 + (ulonglong)(byte)puVar15[-3] * 4)))();
      return;
    }
                    // WARNING: Could not recover jumptable at 0x000140947eae. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(in_R9 + *(int *)(in_R9 + (ulonglong)(byte)puVar15[-3] * 4)))();
    return;
  case 0xfd:
code_r0x000140947c59:
    *(undefined8 *)((longlong)register0x00000020 + -8) = 0x140947c68;
    std::alloc::__default_lib_allocator::__rust_dealloc();
                    // WARNING: Read-only address (ram,0x000142ab2c78) is written
                    // WARNING: Read-only address (ram,0x000142ab2c78) is written
    return;
  case 0xfe:
    goto code_r0x000140947d1f;
  case 0xff:
    *(undefined4 *)(UNRECOVERED_JUMPTABLE + (longlong)in_RCX * 4) = 0x89442701;
    if (bVar22) {
      unaff_RBX = CONCAT62(uVar17,CONCAT11(0x34,bVar4));
      (&UNK_142ab2cb3)[uVar9] = (&UNK_142ab2cb3)[uVar9] + bVar13;
      register0x00000020 = (BADSPACEBASE *)auStackX_8;
      *(char *)(unaff_retaddr + 2) = *(char *)(unaff_retaddr + 2) << 1;
      goto code_r0x000140947b89;
    }
    piVar11 = (int *)(CONCAT71(uVar12,(char)cVar5 - 0xd) + 8 + (longlong)in_RCX * 4);
    *piVar11 = unaff_XMM6_Da;
    piVar11[1] = unaff_XMM6_Db;
    piVar11[2] = unaff_XMM6_Dc;
    piVar11[3] = unaff_XMM6_Dd;
    unaff_retaddr = unaff_RBP;
    goto code_r0x000140947bfd;
  }
  *(undefined1 *)(unaff_RBP + 0x25) = 2;
  bevy_app::app::App::
  insert_resource<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>
            ();
  unaff_RBP[0x1e] = 0x8000000000000000;
  bevy_app::app::App::
  insert_resource<country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::UiEditRectangleActiveWidgetResource>
            ();
  unaff_RBP[0x20] = 0x3f800000;
  unaff_RBP[0x21] = 0;
  unaff_RBP[0x22] = 0x40c0000040c00000;
code_r0x000140946509:
  *(undefined4 *)(unaff_RBP + 0x23) = 0;
  *(undefined4 *)(unaff_RBP + 0x1e) = 0;
code_r0x00014094651d:
  bevy_app::app::App::
  insert_resource<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>
            ();
  unaff_RBP[0x20] = 0;
  unaff_RBP[0x21] = 0x40400000;
  *(undefined4 *)(unaff_RBP + 0x22) = 0;
  *(undefined4 *)(unaff_RBP + 0x1e) = 0;
  bevy_app::app::App::
  insert_resource<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>();
  *(undefined8 *)((longlong)unaff_RBP + 0xf5) = 0;
  unaff_RBP[0x1e] = 0;
  bevy_app::app::App::
  insert_resource<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::ShapesDragMouseMoved>
            ();
  unaff_RBP[0x1f] = 0;
  *(undefined4 *)(unaff_RBP + 0x1e) = 0;
code_r0x00014094659e:
  bevy_app::app::App::
  insert_resource<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>
            ();
  return;
code_r0x000140946a63:
                    // WARNING: Could not recover jumptable at 0x000140946a66. Too many branches
                    // WARNING: Treating indirect jump as call
  (*(UNRECOVERED_JUMPTABLE + (longlong)puVar15))();
  return;
code_r0x000140946a9c:
  UNRECOVERED_JUMPTABLE = (code *)(ulonglong)(uint)(int)(short)UNRECOVERED_JUMPTABLE;
  *UNRECOVERED_JUMPTABLE = (code)((char)*UNRECOVERED_JUMPTABLE + (char)cVar5);
code_r0x000140946a9f:
  pcVar14 = UNRECOVERED_JUMPTABLE + -0x73;
  cVar5 = *pcVar14;
  *pcVar14 = (code)((char)*pcVar14 + (byte)in_RCX);
  uVar6 = (int)UNRECOVERED_JUMPTABLE + 0x216c401 + (uint)CARRY1((byte)cVar5,(byte)in_RCX);
code_r0x000140946aa7:
                    // WARNING: Could not recover jumptable at 0x000140946aae. Too many branches
                    // WARNING: Treating indirect jump as call
  (*(code *)(puVar15 + *(int *)(puVar15 + (ulonglong)uVar6 * 4)))();
  return;
code_r0x000140946a77:
  puVar15 = (undefined *)0x42ab2d18;
code_r0x000140946a83:
                    // WARNING: Could not recover jumptable at 0x000140946a8a. Too many branches
                    // WARNING: Treating indirect jump as call
  (*(code *)(puVar15 + *(int *)(puVar15 + (longlong)UNRECOVERED_JUMPTABLE * 4)))();
  return;
}


