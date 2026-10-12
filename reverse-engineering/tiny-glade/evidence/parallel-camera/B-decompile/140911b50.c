
/* WARNING: Instruction at (ram,0x0001409148cd) overlaps instruction at (ram,0x0001409148cb)
    */
/* WARNING: Control flow encountered bad instruction data */
/* WARNING: Removing unreachable block (ram,0x000140913320) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::systems::camera_rig::MainCamera::immediately_finish_interpolating(void)

{
  uint *puVar1;
  byte *pbVar2;
  undefined *puVar3;
  undefined *puVar4;
  ulonglong *puVar5;
  undefined **ppuVar6;
  longlong lVar7;
  ulonglong uVar8;
  undefined8 uVar9;
  undefined4 *puVar10;
  undefined1 auVar11 [16];
  undefined1 auVar12 [16];
  undefined8 uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  undefined8 uVar16;
  undefined1 *puVar17;
  undefined1 *puVar18;
  undefined1 *puVar19;
  undefined1 *puVar20;
  char cVar21;
  int iVar22;
  undefined **ppuVar23;
  undefined8 *puVar24;
  char *pcVar25;
  char *pcVar26;
  byte bVar28;
  undefined **in_RCX;
  ulonglong uVar29;
  undefined **UNRECOVERED_JUMPTABLE_00;
  code *UNRECOVERED_JUMPTABLE;
  undefined **ppuVar30;
  undefined **unaff_RBX;
  undefined1 *puVar31;
  undefined1 *puVar32;
  undefined1 *puVar33;
  undefined1 *puVar34;
  undefined8 *puVar35;
  undefined1 *puVar36;
  undefined1 *puVar37;
  undefined8 *unaff_RBP;
  undefined8 *puVar38;
  longlong lVar39;
  int iVar40;
  uint uVar41;
  undefined **_Dst;
  undefined **in_R8;
  undefined **ppuVar42;
  undefined **in_R9;
  longlong lVar43;
  undefined *in_R11;
  undefined *puVar44;
  undefined **unaff_R12;
  longlong *unaff_R13;
  undefined **unaff_R14;
  undefined **unaff_R15;
  undefined2 in_ES;
  bool bVar45;
  bool bVar46;
  unkbyte10 in_ST0;
  unkbyte10 in_ST2;
  float in_XMM0_Da;
  float fVar47;
  float extraout_XMM0_Da;
  float in_XMM0_Db;
  float extraout_XMM0_Db;
  float in_XMM0_Dc;
  float extraout_XMM0_Dc;
  float in_XMM0_Dd;
  float extraout_XMM0_Dd;
  float in_XMM1_Da;
  float fVar48;
  undefined4 in_XMM1_Db;
  float fVar50;
  undefined4 in_XMM1_Dc;
  undefined4 in_XMM1_Dd;
  undefined1 auVar49 [16];
  float fVar51;
  float fVar52;
  undefined1 in_XMM2 [16];
  undefined1 auVar53 [16];
  float in_XMM3_Da;
  float unaff_XMM6_Da;
  undefined4 unaff_XMM6_Db;
  undefined4 unaff_XMM6_Dc;
  undefined4 unaff_XMM6_Dd;
  float unaff_XMM7_Da;
  float unaff_XMM7_Db;
  float unaff_XMM8_Da;
  float unaff_XMM9_Da;
  float unaff_XMM9_Db;
  float unaff_XMM9_Dc;
  float unaff_XMM9_Dd;
  float unaff_XMM10_Da;
  float unaff_XMM10_Db;
  float unaff_XMM10_Dd;
  float unaff_XMM11_Da;
  undefined4 unaff_XMM11_Db;
  float unaff_XMM12_Da;
  float unaff_XMM13_Da;
  undefined1 uStack_2a1;
  undefined1 auStack_298 [32];
  undefined *puStack_278;
  float fStack_270;
  char acStack_26a [2];
  undefined8 uStack_268;
  undefined *puStack_260;
  float fStack_258;
  undefined4 uStack_254;
  undefined4 uStack_250;
  undefined4 uStack_24c;
  undefined4 uStack_248;
  undefined4 uStack_244;
  undefined4 uStack_240;
  undefined4 uStack_23c;
  undefined4 uStack_238;
  undefined4 uStack_234;
  undefined4 uStack_230;
  float fStack_22c;
  undefined *puStack_228;
  undefined *puStack_220;
  undefined *puStack_218;
  undefined *puStack_210;
  longlong lStack_208;
  undefined8 uStack_200;
  undefined8 uStack_1f8;
  undefined8 uStack_1f0;
  undefined8 uStack_1e8;
  undefined4 uStack_1e0;
  undefined4 uStack_1dc;
  undefined4 uStack_1d8;
  undefined4 uStack_1d4;
  undefined8 uStack_1d0;
  undefined8 uStack_1c0;
  undefined8 uStack_1b0;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined8 uStack_188;
  undefined8 uStack_180;
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  uint uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined *puStack_a8;
  undefined *puStack_a0;
  undefined *puStack_98;
  undefined *puStack_90;
  longlong lStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined8 uStack_58;
  undefined8 uStack_50;
  char cStack_48;
  undefined8 uStack_40;
  char cStack_38;
  undefined8 uStack_30;
  undefined7 uVar27;
  
  iVar40 = (int)in_ST2;
  puVar34 = auStack_298;
  puVar35 = (undefined8 *)auStack_298;
  puVar32 = auStack_298;
  puVar37 = auStack_298;
  puVar36 = auStack_298;
  puVar31 = auStack_298;
  _Dst = in_RCX + 0x20;
  ppuVar23 = &anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885;
  ppuVar30 = (undefined **)&DAT_142aabc70;
  uVar29 = (ulonglong)*(int *)(&DAT_142aabc70 + (ulonglong)*(byte *)(in_RCX + 0x9a) * 4);
  bVar45 = 0xfffffffebd55438f < uVar29;
  UNRECOVERED_JUMPTABLE_00 = (undefined **)(&DAT_142aabc70 + uVar29);
  bVar46 = UNRECOVERED_JUMPTABLE_00 == (undefined **)0x0;
  bVar28 = (byte)UNRECOVERED_JUMPTABLE_00;
  cVar21 = (char)unaff_RBX;
  uVar27 = (undefined7)((ulonglong)unaff_RBX >> 8);
  puVar17 = auStack_298;
  puVar18 = auStack_298;
  puVar19 = auStack_298;
  puVar33 = auStack_298;
  puVar20 = auStack_298;
  puVar38 = (undefined8 *)auStack_298;
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
  switch(*(byte *)(in_RCX + 0x9a)) {
  case 0:
    lVar39 = 0x8c;
    UNRECOVERED_JUMPTABLE_00 = _Dst;
    break;
  case 1:
    lVar39 = 0x144;
    UNRECOVERED_JUMPTABLE_00 = in_RCX + 0x37;
    break;
  case 2:
    ppuVar23 = &anon_0555c73667d6be1dc866b3f0ae0b3f3f_17_llvm_14020587298740829885;
    lVar39 = 0x1c4;
    UNRECOVERED_JUMPTABLE_00 = in_RCX + 0x4e;
    break;
  case 3:
    ppuVar23 = (undefined **)&anon_0555c73667d6be1dc866b3f0ae0b3f3f_16_llvm_14020587298740829885;
    lVar39 = 0x264;
    UNRECOVERED_JUMPTABLE_00 = in_RCX + 0x5e;
    break;
  case 4:
    lVar39 = 0x31c;
    UNRECOVERED_JUMPTABLE_00 = in_RCX + 0x72;
    break;
  case 5:
    ppuVar23 = &anon_0555c73667d6be1dc866b3f0ae0b3f3f_18_llvm_14020587298740829885;
    lVar39 = 0x3a4;
    UNRECOVERED_JUMPTABLE_00 = in_RCX + 0x89;
    break;
  case 6:
    anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_4_ =
         CONCAT31(anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._1_3_,
                  (char)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 + -0x68);
    puVar44 = &DAT_142aabc88;
    goto code_r0x000140911ea1;
  case 7:
                    /* WARNING: Could not emulate address calculation at 0x000140911f0f */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)(&UNK_142aabc98 +
              *(int *)(&UNK_142aabc98 +
                      (ulonglong)
                      *(uint *)((ulonglong)(uint)*(float *)UNRECOVERED_JUMPTABLE_00 + 0x128) * 4)))
              ((ulonglong)(uint)*(float *)UNRECOVERED_JUMPTABLE_00,
               &UNK_142aabc98 +
               *(int *)(&UNK_142aabc98 +
                       (ulonglong)
                       *(uint *)((ulonglong)(uint)*(float *)UNRECOVERED_JUMPTABLE_00 + 0x128) * 4));
    return;
  case 8:
    puVar44 = (undefined *)0x42aabc88;
code_r0x000140911ea1:
                    /* WARNING: Could not recover jumptable at 0x000140911ea8. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)(puVar44 + *(int *)(puVar44 + (longlong)in_R8 * 4)))
              (UNRECOVERED_JUMPTABLE_00,puVar44,in_R8,
               puVar44 + *(int *)(puVar44 + (longlong)in_R8 * 4));
    return;
  case 9:
                    /* WARNING: Could not emulate address calculation at 0x000140911ed7 */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)(&UNK_142aabc98 +
              *(int *)(&UNK_142aabc98 +
                      (ulonglong)
                      *(uint *)((ulonglong)(uint)*(float *)UNRECOVERED_JUMPTABLE_00 + 0x128) * 4)))
              ((ulonglong)(uint)*(float *)UNRECOVERED_JUMPTABLE_00,
               &UNK_142aabc98 +
               *(int *)(&UNK_142aabc98 +
                       (ulonglong)
                       *(uint *)((ulonglong)(uint)*(float *)UNRECOVERED_JUMPTABLE_00 + 0x128) * 4));
    return;
  case 10:
    *(int *)(ulonglong)*(uint *)(CONCAT71(0x142c91c,'P' - bVar45) + 0x10) =
         (int)UNRECOVERED_JUMPTABLE_00;
    lVar39 = *unaff_R13;
    *unaff_R13 = 0;
                    /* WARNING: Could not recover jumptable at 0x000140911fb5. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)(&UNK_142aabcc0 + *(int *)(&UNK_142aabcc0 + lVar39 * 4)))();
    return;
  default:
    *(float *)((longlong)in_RCX + 0x11c) =
         (float)((int)*(float *)((longlong)in_RCX + 0x11c) + 0x42c91c98 + (uint)bVar45);
    goto code_r0x000140911f53;
  case 0xc:
    *(float *)(in_RCX + 0x23) = 2.8026e-45;
    *(float *)((longlong)in_RCX + 0x11c) = in_XMM0_Da;
code_r0x000140911f53:
    UNRECOVERED_JUMPTABLE_00 = (undefined **)*UNRECOVERED_JUMPTABLE_00;
code_r0x000140911f56:
    in_R8 = (undefined **)&UNK_142aabc98;
    ppuVar30 = (undefined **)
               (longlong)
               *(int *)(&UNK_142aabc98 +
                       (ulonglong)(uint)*(float *)(UNRECOVERED_JUMPTABLE_00 + 0x25) * 4);
code_r0x000140911f67:
                    /* WARNING: Could not recover jumptable at 0x000140911f6a. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)((longlong)ppuVar30 + (longlong)in_R8))
              (UNRECOVERED_JUMPTABLE_00,(code *)((longlong)ppuVar30 + (longlong)in_R8));
    return;
  case 0xf:
    anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_4_ =
         CONCAT31(anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._1_3_,
                  (char)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 + -0x68);
  case 0x11:
    memmove(UNRECOVERED_JUMPTABLE_00,unaff_R15,(size_t)in_R8);
    *(float *)(unaff_R15 + 4) = (float)lStack_208;
    unaff_R15[2] = puStack_218;
    unaff_R15[3] = puStack_210;
    *unaff_R15 = puStack_228;
    unaff_R15[1] = puStack_220;
    in_RCX[0x22] = (byte *)((longlong)unaff_R14 + 1);
code_r0x000140912047:
    in_RCX[0x24] = (undefined *)unaff_RBX;
code_r0x00014091218f:
    fVar50 = *(float *)(in_RCX + 0x23);
    if (fVar50 == 0.0) {
code_r0x0001409124c4:
      return;
    }
    if (fVar50 == 4.2039e-45) {
code_r0x00014091221f:
      ::utils::lerp();
    }
    else if (fVar50 != 2.8026e-45) {
      *(float *)in_RCX[2] = *(float *)((longlong)in_RCX + 0x1c);
      _Dst = (undefined **)(*in_RCX + 0x100);
      UNRECOVERED_JUMPTABLE_00 = (undefined **)(ulonglong)(byte)(*in_RCX)[0x4d0];
code_r0x0001409121f7:
      UNRECOVERED_JUMPTABLE_00 =
           (undefined **)
           (&UNK_142aabcd4 + *(int *)(&UNK_142aabcd4 + (longlong)UNRECOVERED_JUMPTABLE_00 * 4));
code_r0x000140912205:
                    /* WARNING: Could not recover jumptable at 0x000140912208. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      (*(code *)UNRECOVERED_JUMPTABLE_00)(UNRECOVERED_JUMPTABLE_00,_Dst);
      return;
    }
code_r0x00014091222c:
    *(float *)in_RCX[2] = *(float *)((longlong)in_RCX + 0x1c);
    _Dst = (undefined **)(*in_RCX + 0x100);
    UNRECOVERED_JUMPTABLE_00 = (undefined **)(ulonglong)(byte)(*in_RCX)[0x4d0];
    ppuVar30 = (undefined **)&UNK_142aabcd4;
code_r0x00014091225d:
    UNRECOVERED_JUMPTABLE =
         (code *)((longlong)
                  (int)*(float *)((longlong)ppuVar30 + (longlong)UNRECOVERED_JUMPTABLE_00 * 4) +
                 (longlong)ppuVar30);
                    /* WARNING: Could not recover jumptable at 0x000140912267. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*UNRECOVERED_JUMPTABLE)(UNRECOVERED_JUMPTABLE,_Dst);
    return;
  case 0x10:
  case 0x12:
  case 0x13:
    UNRECOVERED_JUMPTABLE_00 = (undefined **)(ulonglong)(uint)*(float *)UNRECOVERED_JUMPTABLE_00;
    goto code_r0x000140911f56;
  case 0x14:
                    /* WARNING: Bad instruction - Truncating control flow here */
    halt_baddata();
  case 0x15:
    goto code_r0x000140911f67;
  case 0x16:
    uVar29 = (ulonglong)(uint)*(float *)(in_RCX + 0x24) + 1;
    if ((uVar29 | (ulonglong)UNRECOVERED_JUMPTABLE_00) >> 0x20 == 0) {
      in_RCX[0x24] = (undefined *)
                     ((uVar29 & 0xffffffff) % ((ulonglong)UNRECOVERED_JUMPTABLE_00 & 0xffffffff));
    }
    else {
      in_RCX[0x24] = (undefined *)(uVar29 % (ulonglong)UNRECOVERED_JUMPTABLE_00);
    }
    in_RCX[0x23] = (undefined *)0x1;
    *(float *)in_RCX[2] = *(float *)((longlong)in_RCX + 0x1c);
    load_parameters();
    goto code_r0x00014091218f;
  case 0x17:
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_bounds_check();
  case 0x18:
    if (!bVar45 && !bVar46) {
      anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_4_ =
           (int)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 + 0x42c91c98;
      *(byte *)_Dst = *(byte *)_Dst + bVar28;
                    /* WARNING: Bad instruction - Truncating control flow here */
      halt_baddata();
    }
    goto code_r0x000140912047;
  case 0x19:
    (*pcRam0000000042c91cc8)(&puStack_228);
    fStack_270 = (float)puStack_220._0_4_;
    puStack_278 = puStack_228;
    fVar50 = expf(unaff_XMM6_Da / __real_be4ccccd);
    *(float *)(in_RCX + 0x49) =
         (__real_3f800000 - fVar50) * (float)((ulonglong)in_RCX[0x49] >> 0x20) +
         (__real_3f800000 - (__real_3f800000 - fVar50)) * SUB84(in_RCX[0x49],0);
    uStack_268 = puStack_218;
    puStack_260 = puStack_210;
    fStack_258 = *(float *)(in_RCX + 0x9b);
    fStack_22c = *(float *)((longlong)in_RCX + 0x26c);
    uStack_234 = (undefined4)*(undefined8 *)((longlong)in_RCX + 0x264);
    uStack_230 = (undefined4)((ulonglong)*(undefined8 *)((longlong)in_RCX + 0x264) >> 0x20);
    uStack_244 = (undefined4)*(undefined8 *)((longlong)in_RCX + 0x254);
    uStack_240 = (undefined4)((ulonglong)*(undefined8 *)((longlong)in_RCX + 0x254) >> 0x20);
    uStack_23c = (undefined4)*(undefined8 *)((longlong)in_RCX + 0x25c);
    uStack_238 = (undefined4)((ulonglong)*(undefined8 *)((longlong)in_RCX + 0x25c) >> 0x20);
    uStack_254 = (undefined4)*(undefined8 *)((longlong)in_RCX + 0x244);
    uStack_250 = (undefined4)((ulonglong)*(undefined8 *)((longlong)in_RCX + 0x244) >> 0x20);
    uStack_24c = (undefined4)*(undefined8 *)((longlong)in_RCX + 0x24c);
    uStack_248 = (undefined4)((ulonglong)*(undefined8 *)((longlong)in_RCX + 0x24c) >> 0x20);
    ::utils::camera::impl$2::into_camera_body_matrices();
    ::utils::camera::CameraProjectionParams::calc_matrices();
    uStack_1f8 = uStack_78;
    uStack_1f0 = uStack_70;
    lStack_208 = lStack_88;
    uStack_200 = uStack_80;
    puStack_218 = puStack_98;
    puStack_210 = puStack_90;
    puStack_228 = puStack_a8;
    puStack_220 = puStack_a0;
    uStack_1e8 = uStack_68;
    uStack_1e0 = (undefined4)uStack_60;
    uStack_1dc = (undefined4)((ulonglong)uStack_60 >> 0x20);
    uStack_1d8 = (undefined4)uStack_58;
    uStack_1d4 = (undefined4)((ulonglong)uStack_58 >> 0x20);
    uStack_1d0 = uStack_50;
    uStack_1c0 = uStack_40;
    uStack_1b0 = uStack_30;
    uStack_1a8 = uStack_128;
    uStack_1a0 = uStack_120;
    uStack_198 = uStack_118;
    uStack_190 = uStack_110;
    uStack_188 = uStack_108;
    uStack_180 = uStack_100;
    uStack_178 = uStack_f8;
    uStack_170 = uStack_f0;
    uStack_138 = uStack_b8;
    uStack_130 = uStack_b0;
    uStack_148 = uStack_c8;
    uStack_140 = uStack_c0;
    uStack_158 = uStack_d8;
    uStack_168 = uStack_e8;
    uStack_160 = uStack_e0;
    ppuVar30 = &puStack_228;
    in_R8 = (undefined **)&DAT_00000100;
    UNRECOVERED_JUMPTABLE_00 = in_RCX;
  case 0x1f:
    memcpy(UNRECOVERED_JUMPTABLE_00,ppuVar30,(size_t)in_R8);
    return;
  case 0x1a:
    goto code_r0x000140912205;
  case 0x1b:
                    /* WARNING: Bad instruction - Truncating control flow here */
    halt_baddata();
  case 0x1c:
    bRam0000000285923930 = bRam0000000285923930 << 1 | (char)bRam0000000285923930 < '\0';
    PTR_update_142c91c20._5_1_ = PTR_update_142c91c20._5_1_ + bVar28;
    goto code_r0x00014091222c;
  case 0x1d:
    goto code_r0x00014091221f;
  case 0x1e:
    if (bVar45 || bVar46) {
      _DAT_142aabc70 = (_DAT_142aabc70 + -0x42c91c98) - (uint)bVar45;
      goto code_r0x00014091225d;
    }
    anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_4_ =
         CONCAT31(anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._1_3_,
                  (char)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 + -0x68);
    goto code_r0x0001409121f7;
  case 0x20:
                    /* WARNING: Subroutine does not return */
    alloc::vec::impl$1::insert::assert_failed();
  case 0x21:
                    /* WARNING: Bad instruction - Truncating control flow here */
    halt_baddata();
  case 0x22:
    goto code_r0x0001409124c4;
  case 0x23:
    return;
  case 0x24:
                    /* WARNING: Subroutine does not return */
    alloc::vec::impl$1::remove::assert_failed();
  case 0x25:
    *(float *)UNRECOVERED_JUMPTABLE_00 = SUB84(auStack_298,0);
    *(byte *)((longlong)in_RCX + 0x444) = 0x98;
    uStack_268 = (undefined *)CONCAT17(*(undefined1 *)(lStack_208 + 0x103),(undefined7)uStack_268);
    bVar28 = *(byte *)(CONCAT44(uStack_23c,uStack_240) +
                      (0x11d - (ulonglong)(*(uint *)(unaff_RBP + 2) < 3)));
    uVar41 = (uint)bVar28;
    if (bVar28 == 6) {
      uVar41 = 0;
    }
    fStack_22c = *(float *)(CONCAT44(uStack_254,fStack_258) +
                           *(longlong *)(&DAT_142aabbf8 + (uVar41 << 3)));
    if (*(float *)(unaff_RBP + 0x2d) == 0.0) {
                    /* WARNING: Could not recover jumptable at 0x000140913152. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      (*(code *)(&DAT_142aabd04 + *(int *)(&DAT_142aabd04 + (ulonglong)*(uint *)(unaff_RBP + 2) * 4)
                ))(&DAT_142aabd04 +
                   *(int *)(&DAT_142aabd04 + (ulonglong)*(uint *)(unaff_RBP + 2) * 4));
      return;
    }
    fStack_270 = (float)(uint)*(float *)((longlong)in_RCX + 4);
    puStack_278 = (undefined *)CONCAT44(puStack_278._4_4_,(float)(uint)*(float *)in_RCX);
    ::utils::geometry::ray::Ray::from_mouse_ss();
    fVar50 = (__real_3f800000 - *(float *)((longlong)unaff_R14 + 4)) /
             (float)(uint)*(float *)((longlong)in_RCX + 4);
    fVar52 = *(float *)unaff_R14 / (float)(uint)*(float *)in_RCX +
             *(float *)unaff_R14 / (float)(uint)*(float *)in_RCX + __real_bf800000;
    fVar50 = fVar50 + fVar50 + __real_3f800000;
    fVar47 = fVar50 * *(float *)(in_RCX + 0x2a) + fVar52 * *(float *)(in_RCX + 0x28);
    fVar48 = fVar50 * *(float *)((longlong)in_RCX + 0x154) +
             fVar52 * *(float *)((longlong)in_RCX + 0x144);
    fVar51 = fVar50 * *(float *)(in_RCX + 0x2b) + fVar52 * *(float *)(in_RCX + 0x29);
    fVar50 = fVar50 * *(float *)((longlong)in_RCX + 0x15c) +
             fVar52 * *(float *)((longlong)in_RCX + 0x14c);
    auVar53._0_4_ = *(float *)(in_RCX + 0x2c) + fVar47 + *(float *)(in_RCX + 0x2e);
    auVar53._4_4_ =
         *(float *)((longlong)in_RCX + 0x164) + fVar48 + *(float *)((longlong)in_RCX + 0x174);
    auVar53._8_4_ = *(float *)(in_RCX + 0x2d) + fVar51 + *(float *)(in_RCX + 0x2f);
    auVar53._12_4_ =
         *(float *)((longlong)in_RCX + 0x16c) + fVar50 + *(float *)((longlong)in_RCX + 0x17c);
    auVar12._4_4_ = auVar53._12_4_;
    auVar12._0_4_ = auVar53._12_4_;
    auVar12._8_4_ = auVar53._12_4_;
    auVar12._12_4_ = auVar53._12_4_;
    divps(auVar53,auVar12);
    auVar49._0_4_ =
         *(float *)(in_RCX + 0x2c) * ___xmm_3c23d70a3c23d70a3c23d70a3c23d70a + fVar47 +
         *(float *)(in_RCX + 0x2e);
    auVar49._4_4_ =
         *(float *)((longlong)in_RCX + 0x164) * _UNK_142aab4e4 + fVar48 +
         *(float *)((longlong)in_RCX + 0x174);
    auVar49._8_4_ = *(float *)(in_RCX + 0x2d) * _UNK_142aab4e8 + fVar51 + *(float *)(in_RCX + 0x2f);
    auVar49._12_4_ =
         *(float *)((longlong)in_RCX + 0x16c) * _UNK_142aab4ec + fVar50 +
         *(float *)((longlong)in_RCX + 0x17c);
    auVar11._4_4_ = auVar49._12_4_;
    auVar11._0_4_ = auVar49._12_4_;
    auVar11._8_4_ = auVar49._12_4_;
    auVar11._12_4_ = auVar49._12_4_;
    divps(auVar49,auVar11);
                    /* WARNING: Could not recover jumptable at 0x000140913048. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)(&DAT_142aabd30 + DAT_142aabd44))();
    return;
  case 0x26:
  case 0x28:
  case 0x29:
    unaff_RBX = (undefined **)CONCAT71(uVar27,cVar21 + -0x44);
    uStack_1dc = (undefined4)uStack_d8;
    uStack_1d8 = (undefined4)((ulonglong)uStack_d8 >> 0x20);
    ppuVar23 = (undefined **)(ulonglong)uStack_d0;
    goto code_r0x0001409130e2;
  case 0x27:
    goto code_r0x0001409131cc;
  case 0x2a:
    goto code_r0x00014091333f;
  case 0x2b:
                    /* WARNING: Bad instruction - Truncating control flow here */
    halt_baddata();
  case 0x2c:
    goto code_r0x0001409143ec;
  case 0x2d:
    PTR_update_142c91c20._3_1_ = PTR_update_142c91c20._3_1_ + bVar28;
    unaff_RBX = (undefined **)CONCAT62((int6)((ulonglong)unaff_RBX >> 0x10),CONCAT11(0x58,cVar21));
    PTR_update_142c91c58._1_1_ = PTR_update_142c91c58._1_1_ + bVar28;
    out(4,0x98);
    puVar19 = &stack0xfffffffffffffd60;
    goto code_r0x0001409143ec;
  case 0x2e:
    ppuVar23 = (undefined **)((ulonglong)auStack_298 & 0xffffffff);
    goto code_r0x0001409143dc;
  case 0x2f:
    goto LAB_140914430;
  case 0x30:
    puVar32 = auStack_298;
    goto code_r0x0001409130f9;
  case 0x31:
    _Dst = (undefined **)((ulonglong)auStack_298 & 0xffffffff);
    puVar32 = auStack_298;
    goto code_r0x0001409130f9;
  case 0x32:
    anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_4_ =
         CONCAT31(anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._1_3_,
                  (char)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 + -0x68);
    PTR_update_142c91c20._3_1_ = PTR_update_142c91c20._3_1_ + bVar28;
    iVar40 = (int)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 + 0x42c91c98;
    PTR_update_142c91c20._1_1_ = PTR_update_142c91c20._1_1_ + bVar28;
    anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_1_ = (char)iVar40;
    anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_1_ =
         (char)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 + -0x68;
    anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._1_3_ =
         (undefined3)((uint)iVar40 >> 8);
    *(byte *)((longlong)unaff_RBX + 0x1c82484) = *(byte *)((longlong)unaff_RBX + 0x1c82484) + bVar28
    ;
    anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_4_ =
         CONCAT31(anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._1_3_,
                  (char)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 + -0x68);
    puVar31 = auStack_298;
    goto code_r0x0001409130e2;
  case 0x33:
    unaff_RBP = (undefined8 *)&stack0xfffffffffffffd60;
    puVar31 = &uStack_2a1;
code_r0x0001409130e2:
    *(int *)(puVar31 + 0xc4) = (int)ppuVar23;
    puVar32 = puVar31;
    if ((char)unaff_R13 != '\x04') {
      unaff_R15 = unaff_R12;
    }
code_r0x0001409130f9:
    *(undefined8 *)(puVar32 + -8) = 0x140913111;
    in_R8 = unaff_R15;
    resources::terrain_height::TerrainHeightsData::
    intersect_ray<country_core::resources::terrain_height::TerrainHeightSsbo>();
    in_RCX = (undefined **)0x0;
    bVar46 = puVar32[0x1c0] == '\0';
    unaff_XMM12_Da = 0.0;
    if (bVar46) {
      unaff_XMM13_Da = 0.0;
      uVar41 = *(uint *)(puVar32 + 0x38);
      in_XMM0_Da = unaff_XMM12_Da;
    }
    else {
      unaff_XMM13_Da = *(float *)(puVar32 + 0x1d0);
code_r0x00014091316c:
      uVar41 = *(uint *)(puVar32 + 0x38);
      in_XMM0_Da = unaff_XMM12_Da;
    }
    unaff_R12 = (undefined **)(ulonglong)uVar41;
    puVar38 = (undefined8 *)puVar32;
    unaff_XMM12_Da = in_XMM0_Da;
    if (!bVar46) {
      in_RCX = (undefined **)(ulonglong)*(uint *)(puVar32 + 0x1cc);
      in_XMM0_Da = *(float *)(puVar32 + 0x1c8);
      unaff_XMM12_Da = in_XMM0_Da;
    }
code_r0x0001409131cc:
    puVar35 = puVar38;
    fVar50 = SUB84(unaff_R12,0);
    if (__real_43430000 <= SQRT(unaff_XMM13_Da * unaff_XMM13_Da + in_XMM0_Da * unaff_XMM12_Da)) {
      if (unaff_XMM9_Da != 0.0) goto LAB_140913467;
      goto LAB_1409135c3;
    }
    *(float *)puVar35[9] = fVar50;
    puVar35[-1] = 0x14091320a;
    ppuVar30 = _Dst;
    ::utils::camera::CameraMatrices::eye_position();
    iVar40 = (int)in_ST2;
    in_XMM0_Da = *(float *)((longlong)puVar35 + 0xb4);
    in_XMM0_Db = 0.0;
    in_XMM0_Dc = 0.0;
    in_XMM0_Dd = 0.0;
    UNRECOVERED_JUMPTABLE_00 = (undefined **)&DAT_142aabd48;
    uVar29 = (ulonglong)*(int *)(&DAT_142aabd48 + (ulonglong)*(byte *)(_Dst + 0x9a) * 4);
    ppuVar23 = (undefined **)(&DAT_142aabd48 + uVar29);
    bVar28 = (byte)ppuVar23;
    iVar22 = (int)ppuVar23;
    puVar38 = unaff_RBP;
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
    switch(*(byte *)(_Dst + 0x9a)) {
    case 0:
      ppuVar23 = _Dst + 0x20;
      goto code_r0x00014091340f;
    case 1:
      ppuVar23 = _Dst + 0x37;
      goto code_r0x00014091340f;
    default:
      if (unaff_XMM9_Da != 0.0) goto LAB_140913467;
      goto code_r0x0001409134dc;
    case 4:
      ppuVar23 = _Dst + 0x72;
code_r0x00014091340f:
      fVar50 = in_XMM0_Da - SUB84(in_RCX,0);
      in_XMM0_Da = SQRT((*(float *)(puVar35 + 0x17) - unaff_XMM13_Da) *
                        (*(float *)(puVar35 + 0x17) - unaff_XMM13_Da) +
                        fVar50 * fVar50 +
                        (*(float *)(puVar35 + 0x16) - unaff_XMM12_Da) *
                        (*(float *)(puVar35 + 0x16) - unaff_XMM12_Da));
      *(float *)(ppuVar23 + 3) = 1.4013e-45;
      *(float *)((longlong)ppuVar23 + 0x1c) = in_XMM0_Da;
code_r0x000140913448:
      *(float *)(ppuVar23 + 4) = in_XMM0_Da;
      ((float *)((longlong)ppuVar23 + 0x24))[0] = 0.0;
      ((float *)((longlong)ppuVar23 + 0x24))[1] = 0.0;
      *(ulonglong *)((longlong)ppuVar23 + 0x2c) = CONCAT44(unaff_XMM11_Db,unaff_XMM11_Da);
      *(float *)((longlong)ppuVar23 + 0x34) = unaff_XMM10_Da;
      if (unaff_XMM9_Da == 0.0) {
code_r0x0001409134dc:
        puVar38 = *(undefined8 **)((longlong)puVar35 + 0x80);
        *(undefined4 *)puVar38[2] = *(undefined4 *)((longlong)puVar38 + 0x1c);
        in_RCX = (undefined **)*puVar38;
        unaff_R14 = (undefined **)in_RCX[6];
        unaff_R15 = (undefined **)in_RCX[8];
        if (unaff_R14 == (undefined **)in_RCX[4]) {
          *(undefined **)((longlong)puVar35 + -8) = &UNK_14091350e;
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
code_r0x00014091350e:
        puVar44 = in_RCX[5];
        *(undefined ***)(puVar44 + (longlong)unaff_R14 * 0x10) = unaff_R15;
        puVar44[(longlong)unaff_R14 * 0x10 + 8] = 0;
        unaff_R14 = (undefined **)((longlong)unaff_R14 + 1);
        in_RCX[6] = (undefined *)unaff_R14;
        in_RCX[8] = in_RCX[8] + 1;
        goto LAB_1409135c3;
      }
LAB_140913467:
      in_XMM0_Dd = 0.0;
      in_XMM0_Dc = 0.0;
      in_XMM0_Db = 0.0;
      in_XMM0_Da = 0.0;
      puVar17 = (undefined1 *)puVar35;
      if (*(byte *)(_Dst + 0x9a) != 0) goto LAB_1409135c3;
code_r0x000140913474:
      puVar35 = (undefined8 *)puVar17;
      **(undefined4 **)((longlong)puVar35 + 0x48) = (int)unaff_R12;
      unaff_RBX = *(undefined ***)((longlong)puVar35 + 0x78);
      if (0x7f7fffff <
          (uint)ABS(*(float *)(unaff_RBP + 0x2d) * __real_3e800000 *
                    *(float *)(*(longlong *)((longlong)puVar35 + 0x58) + 0x108)))
      goto LAB_140913578;
code_r0x0001409134b2:
      iVar40 = (int)in_ST2;
      fVar50 = SUB84(unaff_R12,0);
      ppuVar23 = &anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885;
      ppuVar30 = (undefined **)&DAT_142aabd60;
      uVar29 = (ulonglong)*(int *)(&DAT_142aabd60 + (ulonglong)*(byte *)(_Dst + 0x9a) * 4);
      UNRECOVERED_JUMPTABLE_00 = (undefined **)(&DAT_142aabd60 + uVar29);
      bVar28 = (byte)UNRECOVERED_JUMPTABLE_00;
      puVar38 = unaff_RBP;
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
      switch(*(byte *)(_Dst + 0x9a)) {
      case 0:
        UNRECOVERED_JUMPTABLE_00 = _Dst + 0x20;
        break;
      case 1:
        UNRECOVERED_JUMPTABLE_00 = _Dst + 0x37;
        break;
      case 2:
code_r0x00014091353b:
        UNRECOVERED_JUMPTABLE_00 = _Dst + 0x4e;
        ppuVar23 = &anon_0555c73667d6be1dc866b3f0ae0b3f3f_17_llvm_14020587298740829885;
        break;
      case 3:
code_r0x00014091354b:
        UNRECOVERED_JUMPTABLE_00 = _Dst + 0x5e;
        ppuVar23 = (undefined **)&anon_0555c73667d6be1dc866b3f0ae0b3f3f_16_llvm_14020587298740829885
        ;
        break;
      case 4:
code_r0x000140913532:
        UNRECOVERED_JUMPTABLE_00 = _Dst + 0x72;
        break;
      case 5:
        UNRECOVERED_JUMPTABLE_00 = _Dst + 0x89;
        ppuVar23 = &anon_0555c73667d6be1dc866b3f0ae0b3f3f_18_llvm_14020587298740829885;
        break;
      case 6:
        PTR_update_142c91c20._5_1_ = PTR_update_142c91c20._5_1_ + bVar28;
        goto code_r0x0001409139a0;
      case 7:
        goto code_r0x0001409139d0;
      case 8:
        uVar41 = (int)_Dst >> 1;
        _Dst = (undefined **)(ulonglong)uVar41;
        UNRECOVERED_JUMPTABLE_00 =
             (undefined **)
             (CONCAT71((int7)((ulonglong)UNRECOVERED_JUMPTABLE_00 >> 8),
                       anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._1_1_) +
             -1);
        puVar34 = (undefined1 *)puVar35;
        if (UNRECOVERED_JUMPTABLE_00 == (undefined **)0x0 || uVar41 != 0) goto code_r0x0001409139be;
        goto code_r0x000140913a07;
      case 9:
        goto code_r0x0001409139c7;
      case 10:
        goto code_r0x0001409139a0;
      case 0xb:
        goto code_r0x0001409139e0;
      case 0xc:
        goto code_r0x000140913bf4;
      case 0xd:
        ppuVar23 = (undefined **)((ulonglong)in_RCX & 0xffffffff);
        goto code_r0x000140913ccf;
      case 0xe:
code_r0x000140913ca1:
        *(byte *)ppuVar23 = *(byte *)ppuVar23 + (char)ppuVar23;
        goto code_r0x000140913ca3;
      case 0xf:
        goto code_r0x000140913cb7;
      case 0x10:
                    /* WARNING: Bad instruction - Truncating control flow here */
        halt_baddata();
      case 0x11:
        goto code_r0x000140913cdf;
      case 0x12:
        uVar29 = (ulonglong)*(uint *)(&DAT_142aabd60 + (longlong)UNRECOVERED_JUMPTABLE_00 * 4);
        goto code_r0x000140913f4a;
      case 0x13:
        *(byte *)UNRECOVERED_JUMPTABLE_00 = *(byte *)UNRECOVERED_JUMPTABLE_00;
        *(byte *)_Dst = *(byte *)_Dst + bVar28;
        *(int *)((longlong)puVar35 + 0x170) = *(int *)((longlong)puVar35 + 0x170) + -0x42c91c98;
        goto code_r0x000140913f13;
      case 0x14:
        anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_4_ =
             CONCAT31(anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._1_3_,
                      (char)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 +
                      -0x68);
        goto code_r0x000140913f1b;
      case 0x15:
        goto code_r0x000140913f29;
      case 0x16:
        anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_4_ =
             (int)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 + 0x42c91c98;
        PTR_update_142c91c20._5_1_ = PTR_update_142c91c20._5_1_ + bVar28;
        ppuVar23 = (undefined **)((ulonglong)puVar35 & 0xffffffb0);
        cVar21 = (char)ppuVar23;
        *(char *)ppuVar23 = *(char *)ppuVar23 + cVar21;
        (&UNK_142aabd18)[uVar29] = (&UNK_142aabd18)[uVar29] + cVar21;
        *(byte *)UNRECOVERED_JUMPTABLE_00 = *(byte *)UNRECOVERED_JUMPTABLE_00 + cVar21;
        puVar35 = (undefined8 *)(undefined1 *)0x42c91c98;
        goto code_r0x000140913f1f;
      case 0x17:
        ppuVar30 = (undefined **)&UNK_142aa8f60;
        bRam0000000285923930 = bRam0000000285923930 << 1 | (char)bRam0000000285923930 < '\0';
        bVar46 = CARRY1(PTR_update_142c91c20._5_1_,bVar28);
        PTR_update_142c91c20._5_1_ = PTR_update_142c91c20._5_1_ + bVar28;
        goto code_r0x000140913f41;
      case 0x18:
code_r0x0001409145a1:
        *(undefined ***)((longlong)puVar35 + -8) = _Dst;
        *(undefined ***)((longlong)puVar35 + -0x10) = unaff_RBX;
        *(float *)((longlong)puVar35 + -0x20) = unaff_XMM6_Da;
        *(undefined4 *)((longlong)puVar35 + -0x1c) = unaff_XMM6_Db;
        *(undefined4 *)((longlong)puVar35 + -0x18) = unaff_XMM6_Dc;
        *(undefined4 *)((longlong)puVar35 + -0x14) = unaff_XMM6_Dd;
        goto code_r0x0001409145b2;
      case 0x19:
        goto code_r0x0001409145e3;
      case 0x1a:
        goto code_r0x0001409145b9;
      case 0x1b:
        goto code_r0x0001409145ce;
      case 0x1c:
        *(int *)((longlong)puVar35 + 0x270) = *(int *)((longlong)puVar35 + 0x270) - (int)in_RCX;
code_r0x0001409145b2:
        ppuVar23 = (undefined **)*ppuVar30;
        in_XMM0_Da = (float)(uint)*(float *)ppuVar23;
        goto code_r0x0001409145bc;
      case 0x1d:
        goto code_r0x0001409145f1;
      case 0x1e:
        ppuVar23 = (undefined **)&UNK_142c91c00;
        *(int *)UNRECOVERED_JUMPTABLE_00 = *(int *)UNRECOVERED_JUMPTABLE_00 + 0x42c91c00;
        *(byte *)UNRECOVERED_JUMPTABLE_00 = *(byte *)UNRECOVERED_JUMPTABLE_00;
        goto code_r0x000140914900;
      case 0x1f:
        goto code_r0x0001409149d9;
      case 0x20:
        anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_4_ =
             CONCAT31(anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._1_3_,
                      (char)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 +
                      -0x68);
                    /* WARNING: Bad instruction - Truncating control flow here */
        halt_baddata();
      case 0x21:
        anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_4_ =
             CONCAT31(anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._1_3_,
                      (char)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 +
                      -0x68);
        PTR_translate_screen_space_142c91c18._3_1_ =
             PTR_translate_screen_space_142c91c18._3_1_ + bVar28;
                    /* WARNING: Bad instruction - Truncating control flow here */
        halt_baddata();
      case 0x22:
        _UNK_142c91da0 = _UNK_142c91da0 + 0x42c91c98 + (uint)(0xfffffffebd55429f < uVar29);
code_r0x000140914900:
        ppuVar23[0x12] = (undefined *)0x0;
        *(undefined1 (*) [16])(ppuVar23 + 0x13) = ZEXT416(__real_42700000);
        *(float *)(ppuVar23 + 0x15) = 9.21956e-41;
        *(byte *)((longlong)ppuVar23 + 0xac) = 0;
        uVar9 = *(undefined8 *)((longlong)puVar35 + 10);
        *(undefined8 *)((longlong)ppuVar23 + 0xad) = *(undefined8 *)((longlong)puVar35 + 2);
        *(undefined8 *)((longlong)ppuVar23 + 0xb5) = uVar9;
        *(undefined8 *)((longlong)ppuVar23 + 0xbb) = *(undefined8 *)((longlong)puVar35 + 0x10);
        *(byte *)((longlong)ppuVar23 + 0xc3) = 3;
        puVar44 = (undefined *)CONCAT44(_UNK_14292593c,_UNK_142925938);
        ppuVar23[0x1f] =
             (undefined *)CONCAT44(_UNK_142925934,___xmm_3f8000003f8000003f8000003f800000);
        ppuVar23[0x20] = puVar44;
        ppuVar23[0x1d] = __xmm_000000000000000041c000003f800000;
        *(float *)(ppuVar23 + 0x1e) = 2.3693558e-38;
        *(byte *)((longlong)ppuVar23 + 0xf4) = 0;
        ppuVar23[0x1c] = (undefined *)0x1000101;
        puVar44 = _UNK_142aab528;
        ppuVar23[0x21] = ___xmm_423400003e99999a3f8000003f800000;
        ppuVar23[0x22] = puVar44;
        *(float *)(ppuVar23 + 0x23) = 1.0;
        *(undefined2 *)((longlong)ppuVar23 + 0x11c) = 0x606;
        goto code_r0x0001409149a5;
      case 0x23:
        goto code_r0x0001409149ed;
      }
      UNRECOVERED_JUMPTABLE = (code *)ppuVar23[3];
      *(undefined1 **)((longlong)puVar35 + -8) = &LAB_140913578;
      (*UNRECOVERED_JUMPTABLE)(UNRECOVERED_JUMPTABLE_00);
LAB_140913578:
      **(undefined4 **)((longlong)puVar35 + 0x70) = *(undefined4 *)((longlong)puVar35 + 0x3c);
      puVar44 = unaff_RBX[6];
      unaff_R14 = (undefined **)unaff_RBX[8];
      if (puVar44 == unaff_RBX[4]) {
        *(undefined1 **)((longlong)puVar35 + -8) = &LAB_1409135a1;
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      puVar3 = unaff_RBX[5];
      *(undefined4 *)(puVar3 + (longlong)puVar44 * 0x10) = 2;
      *(undefined ***)(puVar3 + (longlong)puVar44 * 0x10 + 8) = unaff_R14;
      in_RCX = (undefined **)(puVar44 + 1);
      unaff_RBX[6] = (undefined *)in_RCX;
      unaff_RBX[8] = unaff_RBX[8] + 1;
LAB_1409135c3:
      if (*(int *)(unaff_RBP + 2) == 3) {
        puVar38 = *(undefined8 **)((longlong)puVar35 + 0x80);
        *(undefined4 *)puVar38[2] = *(undefined4 *)((longlong)puVar38 + 0x1c);
        in_RCX = (undefined **)*puVar38;
        puVar44 = in_RCX[6];
        puVar3 = in_RCX[8];
        if (puVar44 == in_RCX[4]) {
          *(undefined1 **)((longlong)puVar35 + -8) = &LAB_1409135fb;
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        puVar4 = in_RCX[5];
        *(undefined **)(puVar4 + (longlong)puVar44 * 0x10) = puVar3;
        puVar4[(longlong)puVar44 * 0x10 + 8] = 1;
        unaff_R14 = (undefined **)(puVar44 + 1);
        in_RCX[6] = (undefined *)unaff_R14;
        in_RCX[8] = in_RCX[8] + 1;
        **(undefined4 **)((longlong)puVar35 + 0x48) = (int)unaff_R12;
        if (*(byte *)(_Dst + 0x9a) - 2 < 4) goto LAB_140913635;
        if (*(byte *)(_Dst + 0x9a) == 0) {
          unaff_R14 = _Dst + 0x20;
          bVar28 = *(byte *)(_Dst + 0x23);
          *(float *)(_Dst + 0x23) = 0.0;
        }
        else {
          unaff_R14 = _Dst + 0x37;
          bVar28 = *(byte *)(_Dst + 0x3a);
          *(float *)(_Dst + 0x3a) = 0.0;
        }
        if ((bVar28 & 1) == 0) goto LAB_140913635;
        uVar9 = *(undefined8 *)((longlong)unaff_R14 + 0x1c);
        uVar13 = *(undefined8 *)((longlong)unaff_R14 + 0x24);
        puVar44 = unaff_R14[6];
        *(undefined **)((longlong)puVar35 + 0xbc) = unaff_R14[5];
        *(undefined **)((longlong)puVar35 + 0xc4) = puVar44;
        *(undefined8 *)((longlong)puVar35 + 0xb0) = uVar9;
        *(undefined8 *)((longlong)puVar35 + 0xb8) = uVar13;
        puVar44 = unaff_R14[2];
        if (puVar44 == *unaff_R14) {
          *(undefined1 **)((longlong)puVar35 + -8) = &LAB_140913839;
          alloc::raw_vec::RawVec::
          grow_one<system_wall_constructor::place_pillars::PillarPlacementProposal,alloc::alloc::Global>
                    ();
        }
        puVar3 = unaff_R14[1];
        uVar9 = *(undefined8 *)((longlong)puVar35 + 0xc4);
        puVar38 = (undefined8 *)(puVar3 + (longlong)puVar44 * 0x1c + 0xc);
        *puVar38 = *(undefined8 *)((longlong)puVar35 + 0xbc);
        puVar38[1] = uVar9;
        uVar9 = *(undefined8 *)((longlong)puVar35 + 0xb8);
        puVar38 = (undefined8 *)(puVar3 + (longlong)puVar44 * 0x1c);
        *puVar38 = *(undefined8 *)((longlong)puVar35 + 0xb0);
        puVar38[1] = uVar9;
        in_RCX = (undefined **)(puVar44 + 1);
        unaff_R14[2] = (undefined *)in_RCX;
        if (2 < *(uint *)(unaff_RBP + 3)) goto LAB_14091387a;
LAB_140913647:
        unaff_XMM10_Dd = 0.0;
        unaff_XMM10_Db = 0.0;
        unaff_XMM10_Da = 0.0;
        unaff_XMM9_Da = 0.0;
        puVar5 = (ulonglong *)**(undefined8 **)((longlong)puVar35 + 0x88);
        lVar39 = (*(undefined8 **)((longlong)puVar35 + 0x88))[1];
        uVar29 = *puVar5;
        UNRECOVERED_JUMPTABLE_00 = (undefined **)0x0;
        ppuVar23 = (undefined **)(uVar29 - *(ulonglong *)(lVar39 + 0x18));
        if (uVar29 < *(ulonglong *)(lVar39 + 0x18)) {
          ppuVar23 = UNRECOVERED_JUMPTABLE_00;
        }
        ppuVar30 = (undefined **)(uVar29 - *(ulonglong *)(lVar39 + 0x38));
        if (uVar29 < *(ulonglong *)(lVar39 + 0x38)) {
          ppuVar30 = UNRECOVERED_JUMPTABLE_00;
        }
        lVar43 = *(longlong *)(lVar39 + 8);
        ppuVar42 = *(undefined ***)(lVar39 + 0x30);
        *(undefined ***)((longlong)puVar35 + 0x40) = ppuVar42;
        bVar46 = ppuVar30 <= ppuVar42;
        ppuVar42 = (undefined **)((longlong)ppuVar42 - (longlong)ppuVar30);
        in_R8 = ppuVar42;
        if (!bVar46) {
          in_R8 = UNRECOVERED_JUMPTABLE_00;
        }
        ppuVar6 = *(undefined ***)(lVar39 + 0x10);
        unaff_RBX = (undefined **)
                    CONCAT71((int7)((ulonglong)unaff_RBX >> 8),
                             bVar46 && ppuVar42 != (undefined **)0x0);
        unaff_R14 = (undefined **)((longlong)ppuVar6 - (longlong)ppuVar23);
        if (ppuVar6 < ppuVar23) {
          unaff_R14 = UNRECOVERED_JUMPTABLE_00;
        }
        in_RCX = *(undefined ***)(lVar39 + 0x28);
        in_R11 = *(undefined **)(lVar39 + 0x40);
        unaff_R12 = (undefined **)(ulonglong)*(uint *)((longlong)puVar35 + 0x38);
        *puVar5 = (ulonglong)(in_R11 + (-(longlong)unaff_R14 - (longlong)in_R8));
        unaff_R13 = *(longlong **)((longlong)puVar35 + 0x78);
        in_R9 = _Dst;
        if (ppuVar6 >= ppuVar23 &&
            (undefined **)((longlong)ppuVar6 - (longlong)ppuVar23) != (undefined **)0x0 ||
            bVar46 && ppuVar42 != (undefined **)0x0) {
          in_RCX = in_RCX + (longlong)ppuVar30 * 2;
          in_R8 = (undefined **)((longlong)in_R8 << 4);
          lVar43 = lVar43 + (longlong)ppuVar23 * 0x10;
          unaff_R14 = (undefined **)((longlong)unaff_R14 * 0x10);
          if (ppuVar6 < ppuVar23) {
            lVar43 = 8;
          }
          unaff_RBX = (undefined **)0x0;
          if (ppuVar23 < ppuVar6) {
            unaff_RBX = (undefined **)(lVar43 + -0x10 + (longlong)unaff_R14);
          }
          if (*(undefined ***)((longlong)puVar35 + 0x40) < ppuVar30) {
            in_RCX = (undefined **)&DAT_00000008;
          }
          *puVar5 = (ulonglong)in_R11;
          *(undefined8 *)((longlong)puVar35 + -8) = 0x140913723;
          ::utils::swizzlers::impl$0::x0y();
          fVar50 = *(float *)(*(longlong *)((longlong)puVar35 + 0x58) + 0x10c);
          fVar47 = *(float *)((longlong)puVar35 + 0xb0) * __real_bd23d70a * fVar50;
          fVar48 = fVar50 * (float)*(undefined8 *)((longlong)puVar35 + 0xb4) *
                            ___xmm_0000000000000000bd23d70abd23d70a;
          fVar50 = fVar50 * (float)((ulonglong)*(undefined8 *)((longlong)puVar35 + 0xb4) >> 0x20) *
                            _UNK_142aab504;
          fVar51 = _UNK_142aab50c * 0.0 * 0.0;
          if ((*(byte *)(_Dst + 0x9a) == 4) || (*(byte *)(_Dst + 0x9a) == 1)) {
            fVar47 = fVar47 * __real_3f000000;
            fVar48 = fVar48 * (float)__xmm_00000000000000003f0000003f000000;
            fVar50 = fVar50 * __xmm_00000000000000003f0000003f000000._4_4_;
            fVar51 = fVar51 * _UNK_1429258ec;
          }
          unaff_XMM9_Da = fVar47 + 0.0;
          unaff_XMM10_Da = fVar48 + 0.0;
          unaff_XMM10_Db = fVar50 + 0.0;
          unaff_XMM10_Dd = fVar51 + 0.0;
        }
        bVar28 = *(byte *)(_Dst + 0x9a);
        unaff_R15 = *(undefined ***)((longlong)puVar35 + 0x48);
      }
      else {
LAB_140913635:
        if (*(uint *)(unaff_RBP + 3) < 3) goto LAB_140913647;
LAB_14091387a:
        unaff_XMM10_Dd = 0.0;
        unaff_XMM10_Db = 0.0;
        unaff_XMM10_Da = 0.0;
        unaff_XMM9_Da = 0.0;
        unaff_R13 = *(longlong **)((longlong)puVar35 + 0x78);
        bVar28 = *(byte *)(_Dst + 0x9a);
        unaff_R15 = *(undefined ***)((longlong)puVar35 + 0x48);
      }
      unaff_XMM11_Da = __real_3e800000;
      if ((bVar28 != 1) && (unaff_XMM11_Da = __real_40000000, bVar28 != 2)) {
        unaff_XMM11_Da = *(float *)(&__real_3e4ccccd3f800000 + (ulonglong)(bVar28 == 4) * 4);
      }
LAB_14091389d:
      *(undefined8 *)((longlong)puVar35 + -8) = 0x1409138b1;
      ::utils::swizzlers::impl$0::x0y();
      iVar40 = (int)in_ST2;
      in_XMM0_Da = *(float *)((longlong)puVar35 + 0xb4) * unaff_XMM11_Da;
      puVar33 = (undefined1 *)puVar35;
code_r0x0001409138bf:
      in_XMM1_Da = *(float *)(puVar33 + 0xb8);
      puVar20 = puVar33;
      goto code_r0x0001409138c8;
    case 6:
      (&UNK_142aabcd5)[uVar29] = (&UNK_142aabcd5)[uVar29] + 'H';
      goto code_r0x0001409134b2;
    case 7:
      goto code_r0x00014091353b;
    case 8:
      in_RCX = (undefined **)(ulonglong)((int)SUB84(in_RCX,0) + 1);
      goto code_r0x00014091350e;
    case 9:
      goto code_r0x000140913532;
    case 10:
      *(int *)(uVar29 + 0x285557a90) = (int)_Dst;
      goto code_r0x00014091350e;
    case 0xb:
      goto code_r0x00014091354b;
    case 0xc:
      puVar35[0x16] = CONCAT44(unaff_XMM11_Db,unaff_XMM11_Da);
      goto code_r0x000140913997;
    case 0xd:
      *ppuVar23 = (undefined *)*puVar35;
      *(int *)ppuVar23 = (int)*(float *)ppuVar23 + iVar22;
                    /* WARNING: Bad instruction - Truncating control flow here */
      halt_baddata();
    case 0xe:
      goto code_r0x0001409139a0;
    case 0xf:
      goto code_r0x0001409139a0;
    case 0x10:
      goto code_r0x000140913997;
    case 0x11:
                    /* WARNING: Bad instruction - Truncating control flow here */
      halt_baddata();
    case 0x12:
      *(byte *)ppuVar23 = *(byte *)ppuVar23 + bVar28;
      *(byte *)_Dst = *(byte *)_Dst + 0x48;
      uVar41 = iVar22 + _DAT_142aabd48;
      *(char *)(ulonglong)uVar41 = *(char *)(ulonglong)uVar41 + (char)uVar41;
      goto LAB_140913bd4;
    case 0x13:
      *(byte *)ppuVar23 = *(byte *)ppuVar23 | (byte)ppuVar30;
      goto code_r0x000140913cb7;
    case 0x14:
      goto code_r0x000140913c6f;
    case 0x15:
      goto code_r0x000140913ca1;
    case 0x16:
                    /* WARNING: Bad instruction - Truncating control flow here */
      halt_baddata();
    case 0x17:
      in_RCX = (undefined **)0x31c;
      goto code_r0x000140913cbf;
    case 0x18:
      ppuVar23 = (undefined **)((ulonglong)ppuVar23 & 0xffffffffffffffb0);
      goto code_r0x000140913f31;
    case 0x19:
      *(byte *)ppuVar30 = *(byte *)ppuVar30 ^ bVar28;
      *(byte *)ppuVar23 = *(byte *)ppuVar23 + bVar28;
      break;
    case 0x1a:
      *(byte *)ppuVar23 = *(byte *)ppuVar23 + bVar28;
      break;
    case 0x1b:
      *(byte *)ppuVar23 = *(byte *)ppuVar23 + bVar28;
      goto code_r0x000140913f13;
    case 0x1c:
      *(int *)ppuVar23 = (int)*(float *)ppuVar23 + iVar22;
      *(byte *)_Dst = *(byte *)_Dst + 0x48;
      *(int *)(puVar35 + 0x32) = *(int *)(puVar35 + 0x32) - (int)ppuVar30;
      break;
    case 0x1d:
      goto code_r0x000140913f21;
    case 0x1e:
      goto LAB_140914526;
    case 0x1f:
      goto code_r0x0001409145cb;
    case 0x20:
      goto code_r0x0001409145a1;
    case 0x21:
      in_XMM0_Da = (float)iVar40;
      goto code_r0x0001409145bc;
    case 0x22:
      *(undefined1 *)((ulonglong)ppuVar23 & 0xffffffffffffff00) =
           *(undefined1 *)((ulonglong)ppuVar23 & 0xffffffffffffff00);
LAB_140914526:
      *(undefined1 **)((longlong)puVar35 + 0x1c0) = (undefined1 *)((longlong)puVar35 + 0x240);
      *(code **)((longlong)puVar35 + 0x1c8) =
           anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
      *(undefined **)((longlong)puVar35 + 0xb0) =
           &anon_f854cc98b5e1efe54f6ab98d53b9f85b_10_llvm_5637972437638668905;
      *(undefined8 *)((longlong)puVar35 + 0xb8) = 2;
      *(undefined8 *)((longlong)puVar35 + 0xd0) = 0;
      *(undefined1 **)((longlong)puVar35 + 0xc0) = (undefined1 *)((longlong)puVar35 + 0x1c0);
      *(undefined8 *)((longlong)puVar35 + 200) = 1;
                    /* WARNING: Subroutine does not return */
      *(undefined **)((longlong)puVar35 + -8) = &UNK_14091459c;
      core::panicking::panic_fmt();
    case 0x23:
      cRam0000000101380549 = cRam0000000101380549 + 'H';
      _DAT_142aabd48 = CONCAT31(_UNK_142aabd49,DAT_142aabd48 + bVar28);
      *(byte *)ppuVar23 = *(byte *)ppuVar23 + bVar28;
      goto code_r0x0001409145e3;
    case 0x24:
      *(byte *)ppuVar23 = *(byte *)ppuVar23 + bVar28;
      goto code_r0x000140914900;
    case 0x25:
      return;
    case 0x26:
      *(uint *)ppuVar30 = ((int)*(float *)ppuVar30 - iVar22) - (uint)(0xfffffffebd5542b7 < uVar29);
      goto code_r0x000140914900;
    case 0x27:
      goto code_r0x0001409149a5;
    case 0x28:
      do {
      } while( true );
    case 0x29:
                    /* WARNING: Bad instruction - Truncating control flow here */
      halt_baddata();
    }
    goto code_r0x000140913e4b;
  case 0x34:
  case 0x35:
    return;
  case 0x36:
    UNRECOVERED_JUMPTABLE_00 = unaff_RBX;
    _UNK_142c91c00 = in_R11;
  case 0x37:
    in_XMM1_Da = *(float *)(UNRECOVERED_JUMPTABLE_00 + 1);
    in_XMM2 = ZEXT416((uint)*(float *)((longlong)UNRECOVERED_JUMPTABLE_00 + 0xc));
    in_R8 = (undefined **)CONCAT44(uStack_23c,uStack_240);
code_r0x00014091333f:
    *(int *)CONCAT44(uStack_24c,uStack_250) = (int)unaff_R12;
    ppuVar23 = in_R8 + 0x21;
    fVar50 = ABS(in_XMM1_Da * __real_bf99999a * __real_3e800000 * *(float *)ppuVar23);
    in_R8 = (undefined **)(ulonglong)(uint)fVar50;
    puVar35 = (undefined8 *)auStack_298;
    if (((uint)fVar50 < 0x7f800000) &&
       (puVar35 = (undefined8 *)auStack_298,
       (uint)ABS(in_XMM2._0_4_ * __real_be800000 * *(float *)ppuVar23) < 0x7f800000)) {
      ppuVar30 = (undefined **)&UNK_142aabd18;
      UNRECOVERED_JUMPTABLE_00 =
           (undefined **)
           (longlong)*(int *)(&UNK_142aabd18 + (ulonglong)*(byte *)(in_RCX + 0xba) * 4);
code_r0x0001409133e2:
                    /* WARNING: Could not recover jumptable at 0x0001409133e5. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      (*(code *)((longlong)UNRECOVERED_JUMPTABLE_00 + (longlong)ppuVar30))
                ((code *)((longlong)UNRECOVERED_JUMPTABLE_00 + (longlong)ppuVar30));
      return;
    }
    goto LAB_1409135c3;
  case 0x38:
  case 0x39:
  case 0x3b:
    goto code_r0x00014091316c;
  case 0x3a:
                    /* WARNING: Bad instruction - Truncating control flow here */
    halt_baddata();
  case 0x3c:
    goto code_r0x0001409133e2;
  case 0x3d:
    anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_1_ =
         (char)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 + -0x68;
    *(byte *)_Dst = *(byte *)_Dst + bVar28;
    anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_4_ =
         CONCAT31(anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._1_3_,
                  (char)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 + -0x68);
    puVar17 = auStack_298;
    goto code_r0x000140913474;
  case 0x3e:
    *(byte *)_Dst = *(byte *)_Dst & bVar28;
    pbVar2 = &UNK_142aabc7f + uVar29;
    *pbVar2 = *pbVar2 << 6 | *pbVar2 >> 2;
    return;
  case 0x3f:
    PTR_update_142c91cc8._4_4_ = PTR_update_142c91cc8._4_4_ + 0x42aabc70 + (uint)bVar45;
    puVar35 = (undefined8 *)auStack_298;
    goto code_r0x000140913448;
  case 0x40:
    unaff_RBX = (undefined **)CONCAT71(uVar27,cVar21 + -0x44);
    puVar35 = (undefined8 *)auStack_298;
    PTR_rotate_yaw_pitch_142c91cb0._4_4_ = in_XMM0_Da;
    PTR_constrain_to_be_above_terrain_142c91cb8._0_4_ = in_XMM0_Db;
    PTR_constrain_to_be_above_terrain_142c91cb8._4_4_ = in_XMM0_Dc;
    PTR_translate_screen_space_142c91cc0._0_4_ = in_XMM0_Dd;
    goto code_r0x000140913448;
  case 0x41:
    goto code_r0x000140913474;
  case 0x42:
    UNK_142c90098 = UNK_142c90098 + -0x68;
    goto LAB_14091389d;
  case 0x43:
    in_XMM1_Da = in_XMM1_Da * in_XMM3_Da;
    puVar20 = auStack_298;
  case 0x44:
    goto code_r0x0001409138c8;
  case 0x45:
    puVar20 = auStack_298;
    goto code_r0x0001409138c8;
  case 0x46:
    goto code_r0x0001409138bf;
  case 0x47:
    puVar20 = auStack_298;
code_r0x0001409138c8:
    in_XMM0_Db = unaff_XMM10_Db;
    puVar34 = puVar20;
    fVar50 = *(float *)(*(longlong *)(puVar34 + 0x58) + 0x10c);
    fVar47 = in_XMM0_Da * fVar50;
    in_XMM0_Da = in_XMM0_Db + in_XMM1_Da * unaff_XMM11_Da * fVar50;
    fVar48 = fVar50 * *(float *)(puVar34 + 0xb0) * unaff_XMM11_Da + unaff_XMM9_Da;
    fVar50 = unaff_XMM11_Da * *(float *)(unaff_RBP + 0x2b) * fVar50 + fVar47 + unaff_XMM10_Da;
    if ((((fVar48 != 0.0) || (fVar50 != 0.0)) || (NAN(fVar50))) ||
       ((in_XMM0_Da != 0.0 || (NAN(in_XMM0_Da))))) {
      if ((0x7f7fffff < (uint)ABS(fVar48)) ||
         ((0x7f7fffff < (uint)ABS(fVar50) || (0x7f7fffff < (uint)ABS(in_XMM0_Da)))))
      goto LAB_140913a6d;
      *(float *)unaff_R15 = SUB84(unaff_R12,0);
      *(ulonglong *)(puVar34 + 0xb0) = CONCAT44(fVar50,fVar48);
      puVar35 = (undefined8 *)puVar34;
      in_XMM0_Dd = unaff_XMM10_Dd;
code_r0x000140913997:
      in_XMM0_Dc = in_XMM0_Dd;
      *(float *)((longlong)puVar35 + 0xb8) = in_XMM0_Da;
      puVar38 = unaff_RBP;
      in_XMM0_Dd = in_XMM0_Dc;
code_r0x0001409139a0:
      fVar50 = SUB84(unaff_R12,0);
      iVar22 = (int)in_ST0;
      ppuVar23 = &anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885;
      ppuVar30 = (undefined **)&DAT_142aabd78;
      bVar46 = 0xfffffffebd554287 <
               (ulonglong)(longlong)*(int *)(&DAT_142aabd78 + (ulonglong)*(byte *)(_Dst + 0x9a) * 4)
      ;
      UNRECOVERED_JUMPTABLE_00 =
           (undefined **)
           (&DAT_142aabd78 + *(int *)(&DAT_142aabd78 + (ulonglong)*(byte *)(_Dst + 0x9a) * 4));
      unaff_RBP = puVar38;
      puVar34 = (undefined1 *)puVar35;
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
      switch(*(byte *)(_Dst + 0x9a)) {
      case 0:
code_r0x0001409139be:
        ppuVar23 = &anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885;
        UNRECOVERED_JUMPTABLE_00 = _Dst + 0x20;
        puVar34 = (undefined1 *)puVar35;
        break;
      case 1:
        UNRECOVERED_JUMPTABLE_00 = _Dst + 0x37;
        break;
      case 2:
code_r0x0001409139d0:
        UNRECOVERED_JUMPTABLE_00 = _Dst + 0x4e;
        ppuVar23 = &anon_0555c73667d6be1dc866b3f0ae0b3f3f_17_llvm_14020587298740829885;
        puVar34 = (undefined1 *)puVar35;
        break;
      case 3:
code_r0x0001409139e0:
        UNRECOVERED_JUMPTABLE_00 = _Dst + 0x5e;
        ppuVar23 = (undefined **)&anon_0555c73667d6be1dc866b3f0ae0b3f3f_16_llvm_14020587298740829885
        ;
        puVar34 = (undefined1 *)puVar35;
        break;
      case 4:
code_r0x0001409139c7:
        ppuVar23 = &anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885;
        UNRECOVERED_JUMPTABLE_00 = _Dst + 0x72;
        puVar34 = (undefined1 *)puVar35;
        break;
      case 5:
        UNRECOVERED_JUMPTABLE_00 = _Dst + 0x89;
        ppuVar23 = &anon_0555c73667d6be1dc866b3f0ae0b3f3f_18_llvm_14020587298740829885;
        break;
      case 6:
        PTR_update_142c91c20._5_1_ = PTR_update_142c91c20._5_1_ + (char)UNRECOVERED_JUMPTABLE_00;
        goto code_r0x000140913bf4;
      case 7:
        goto code_r0x000140913ce5;
      case 8:
        unaff_RBP = (undefined8 *)*puVar38;
        puVar35 = puVar38 + 1;
        goto code_r0x000140913cb7;
      case 9:
        goto code_r0x000140913ccf;
      case 10:
        goto code_r0x000140913ca7;
      case 0xb:
        goto code_r0x000140913cf7;
      case 0xc:
        goto code_r0x000140913f5f;
      case 0xd:
code_r0x000140913f1f:
        *(char *)ppuVar23 = *(char *)ppuVar23 + (char)ppuVar23;
        goto code_r0x000140913f21;
      case 0xe:
code_r0x000140913f31:
        *(int *)ppuVar23 = *(int *)ppuVar23 + (int)ppuVar23;
        *(byte *)_Dst = *(byte *)_Dst + (char)UNRECOVERED_JUMPTABLE_00;
        goto code_r0x000140913f37;
      case 0xf:
code_r0x000140913f41:
        ppuVar23 = (undefined **)(ulonglong)(bVar46 + 0x44e29afa);
        goto code_r0x000140913f46;
      case 0x10:
                    /* WARNING: Bad instruction - Truncating control flow here */
        halt_baddata();
      case 0x11:
                    /* WARNING: Bad instruction - Truncating control flow here */
        halt_baddata();
      case 0x12:
code_r0x0001409145b9:
        ppuVar23 = &anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885;
        in_XMM0_Da = (float)iVar40;
code_r0x0001409145bc:
        fVar50 = (float)(uint)*(float *)((longlong)ppuVar23 + 4);
        goto code_r0x0001409145c4;
      case 0x13:
        UNRECOVERED_JUMPTABLE_00 =
             (undefined **)
             CONCAT71((int7)((ulonglong)UNRECOVERED_JUMPTABLE_00 >> 8),
                      anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._1_1_);
        goto code_r0x0001409145fe;
      case 0x14:
        goto code_r0x0001409145d1;
      case 0x15:
        bRam0000000285923930 = bRam0000000285923930 << 1 | (char)bRam0000000285923930 < '\0';
        goto code_r0x0001409145e9;
      case 0x16:
        puVar24 = (undefined8 *)((longlong)puVar35 + -8);
        *(undefined8 **)((longlong)puVar35 + -8) = puVar38;
        cVar21 = '\x1d';
        do {
          puVar38 = puVar38 + -1;
          puVar24 = puVar24 + -1;
          *puVar24 = *puVar38;
          cVar21 = cVar21 + -1;
        } while ('\0' < cVar21);
        *(undefined1 **)((longlong)puVar35 + -0xf8) = (undefined1 *)((longlong)puVar35 + -8);
        puVar1 = (uint *)((longlong)unaff_RBX + -0x74b7e3bf);
        *puVar1 = *puVar1 >> 0x11 | *puVar1 << 0xf;
        goto code_r0x0001409145ce;
      case 0x17:
        goto code_r0x000140914609;
      case 0x18:
        anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_4_ =
             CONCAT31(anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._1_3_,
                      ((char)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 -
                      bVar46) + '/');
        _UNK_142c91db4 = 0x80c70606;
                    /* WARNING: Bad instruction - Truncating control flow here */
        halt_baddata();
      case 0x19:
        (*(code *)&anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885)();
        return;
      case 0x1a:
        goto code_r0x0001409149b9;
      case 0x1b:
        *(byte *)unaff_RBX = *(byte *)unaff_RBX << 1 | (char)*(byte *)unaff_RBX < '\0';
code_r0x0001409149d9:
        anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_4_ =
             CONCAT31(anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._1_3_,
                      (char)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 +
                      -0x68);
        goto code_r0x0001409149db;
      case 0x1c:
                    /* WARNING: Bad instruction - Truncating control flow here */
        halt_baddata();
      case 0x1d:
                    /* WARNING: Bad instruction - Truncating control flow here */
        halt_baddata();
      }
code_r0x000140913a07:
      *(undefined ***)(puVar34 + 0x20) = _Dst;
      in_R9 = (undefined **)(puVar34 + 0x310);
      UNRECOVERED_JUMPTABLE = (code *)ppuVar23[5];
      *(undefined **)(puVar34 + -8) = &UNK_140913a22;
      (*UNRECOVERED_JUMPTABLE)(UNRECOVERED_JUMPTABLE_00);
      **(undefined4 **)(puVar34 + 0x70) = *(undefined4 *)(puVar34 + 0x3c);
      lVar39 = unaff_R13[6];
      lVar43 = unaff_R13[8];
      if (lVar39 == unaff_R13[4]) {
        *(undefined **)(puVar34 + -8) = &UNK_140913a4b;
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar7 = unaff_R13[5];
      *(undefined4 *)(lVar7 + lVar39 * 0x10) = 0;
      *(longlong *)(lVar7 + 8 + lVar39 * 0x10) = lVar43;
      unaff_R13[6] = lVar39 + 1;
      unaff_R13[8] = unaff_R13[8] + 1;
    }
LAB_140913a6d:
    fVar50 = *(float *)(unaff_RBP + 0x2c);
    if (fVar50 == 0.0) {
LAB_140913ada:
      puVar35 = (undefined8 *)puVar34;
      if (puVar34[0x37] == '\x05') goto LAB_140913ae5;
LAB_140913bd4:
      *(float *)unaff_R15 = SUB84(unaff_R12,0);
      *(byte *)(_Dst + 0x9a) = *(byte *)(*(longlong *)((longlong)puVar35 + 0x90) + 0x103);
    }
    else {
      bVar28 = *(byte *)(_Dst + 0x9a);
      if ((1 < bVar28) && (bVar28 != 4)) {
        if (bVar28 == 5) {
          fVar50 = (float)((uint)fVar50 & ___xmm_80000000800000008000000080000000 |
                          ___xmm_3f8000003f8000003f8000003f800000);
          iVar40 = 0x7fffffff;
          if (fVar50 <= __real_4effffff) {
            iVar40 = (int)fVar50;
          }
          if (NAN(fVar50)) {
            iVar40 = 0;
          }
          fVar50 = (float)((int)*(float *)((longlong)_Dst + 0x484) - iVar40);
          if ((int)fVar50 < 1) {
            fVar50 = 0.0;
          }
          *(float *)unaff_R15 = SUB84(unaff_R12,0);
          *(float *)((longlong)_Dst + 0x484) = fVar50;
        }
        goto LAB_140913ada;
      }
      unaff_XMM9_Da = (float)(___xmm_7fffffff7fffffff7fffffff7fffffff & (uint)fVar50);
      *(float *)unaff_R15 = SUB84(unaff_R12,0);
      *(float *)(puVar34 + 0xb0) = unaff_XMM8_Da;
      *(undefined4 *)(puVar34 + 0xb4) = *(undefined4 *)(puVar34 + 0x6c);
      *(float *)(puVar34 + 0xb8) = unaff_XMM7_Da;
      in_R8 = (undefined **)(puVar34 + 0xb0);
      *(undefined8 *)(puVar34 + -8) = 0x140913b77;
      zoom();
      **(undefined4 **)(puVar34 + 0x70) = *(undefined4 *)(puVar34 + 0x3c);
      in_RCX = (undefined **)unaff_R13[6];
      unaff_R14 = (undefined **)unaff_R13[8];
      if (in_RCX == (undefined **)unaff_R13[4]) {
        *(undefined1 **)(puVar34 + -8) = &LAB_140913ba0;
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
LAB_140913ba0:
      lVar39 = unaff_R13[5];
      lVar43 = (longlong)in_RCX * 0x10;
      *(undefined4 *)(lVar39 + lVar43) = 1;
      *(float *)(lVar39 + 4 + lVar43) = unaff_XMM9_Da;
      *(undefined ***)(lVar39 + 8 + lVar43) = unaff_R14;
      unaff_R13[6] = (longlong)((longlong)in_RCX + 1);
      unaff_R13[8] = unaff_R13[8] + 1;
      puVar35 = (undefined8 *)puVar34;
      if (puVar34[0x37] != '\x05') goto LAB_140913bd4;
LAB_140913ae5:
      lVar39 = *(longlong *)(puVar34 + 0x60);
      ppuVar23 = *(undefined ***)(lVar39 + 0xf8);
      UNRECOVERED_JUMPTABLE_00 =
           (undefined **)((longlong)ppuVar23 + *(longlong *)(lVar39 + 0x100) * 4);
      in_R8 = (undefined **)(ulonglong)*(byte *)(lVar39 + 0x108);
      puVar18 = puVar34;
code_r0x000140913b04:
      puVar35 = (undefined8 *)puVar18;
      fVar50 = SUB84(unaff_R12,0);
      ppuVar30 = (undefined **)(ulonglong)(uint)*(float *)((longlong)_Dst + 0x484);
      if (ppuVar30 == (undefined **)0x0) {
        if ((char)in_R8 != '\0') {
LAB_140913c3e:
          do {
            in_R9 = *(undefined ***)((longlong)puVar35 + 0x50);
            if (ppuVar23 == UNRECOVERED_JUMPTABLE_00) goto code_r0x000140913bf4;
            fVar47 = *(float *)ppuVar23;
            ppuVar23 = (undefined **)((longlong)ppuVar23 + 4);
            ppuVar30 = (undefined **)(*(undefined ***)((longlong)puVar35 + 0x50))[0x35];
            in_R8 = (undefined **)((ulonglong)(uint)fVar47 * 9);
          } while (ppuVar30[(ulonglong)(uint)fVar47 * 9 + 2] == (undefined *)0x0);
code_r0x000140913c6f:
          ppuVar23 = ppuVar30 + (longlong)in_R8;
          in_R9 = (undefined **)0x0;
          in_R11 = (undefined *)0x0;
          if (*(undefined **)(*(longlong *)((longlong)puVar35 + 0x60) + 0x110) < ppuVar23[8]) {
            ppuVar30 = (undefined **)ppuVar23[7];
            UNRECOVERED_JUMPTABLE_00 =
                 (undefined **)
                 ppuVar30[(longlong)*(undefined **)(*(longlong *)((longlong)puVar35 + 0x60) + 0x110)
                         ];
            if (UNRECOVERED_JUMPTABLE_00 == (undefined **)0x0) {
              in_R9 = (undefined **)0x0;
              in_R11 = (undefined *)0x0;
            }
            else {
code_r0x000140913ca3:
              ppuVar23 = (undefined **)ppuVar23[3];
code_r0x000140913ca7:
              in_R11 = ppuVar23[~(ulonglong)UNRECOVERED_JUMPTABLE_00 * 6 + 2];
code_r0x000140913cb7:
              in_R9 = (undefined **)0x0;
            }
          }
          goto LAB_1409142e5;
        }
LAB_14091423a:
        in_R9 = *(undefined ***)((longlong)puVar35 + 0x50);
        do {
          if (ppuVar23 == UNRECOVERED_JUMPTABLE_00) goto code_r0x000140913bf4;
          fVar47 = *(float *)ppuVar23;
          ppuVar23 = (undefined **)((longlong)ppuVar23 + 4);
          in_R8 = (undefined **)((ulonglong)(uint)fVar47 * 0xa0);
        } while (*(longlong *)(in_R9[0x20] + 0x58 + (longlong)in_R8) == 0);
        ppuVar30 = (undefined **)(in_R9[0x20] + (longlong)in_R8);
        uVar29 = (ulonglong)(uint)*(float *)((longlong)ppuVar30 + 0x94);
        puVar44 = in_R9[0x35];
        if ((*(ulonglong *)(*(longlong *)((longlong)puVar35 + 0x60) + 0x110) <
             *(ulonglong *)(puVar44 + uVar29 * 0x48 + 0x40)) &&
           (uVar8 = *(ulonglong *)
                     (*(longlong *)(puVar44 + uVar29 * 0x48 + 0x38) +
                     *(ulonglong *)(*(longlong *)((longlong)puVar35 + 0x60) + 0x110) * 8),
           uVar8 != 0)) {
          in_R9 = *(undefined ***)
                   (*(longlong *)(puVar44 + uVar29 * 0x48 + 0x18) + 0x10 + ~uVar8 * 0x30);
        }
        else {
          in_R9 = (undefined **)0x0;
        }
        puVar44 = ppuVar30[10];
        lVar43 = 0;
LAB_1409142be:
        ppuVar23 = in_R9 + (ulonglong)*(uint *)(puVar44 + lVar43 * 0x10 + 8) * 6;
        puVar36 = (undefined1 *)puVar35;
      }
      else {
        if ((char)in_R8 == '\0') {
          puVar44 = (undefined *)0x8;
          lVar43 = 0;
          lVar39 = 0;
          in_R9 = (undefined **)0x0;
          in_R8 = (undefined **)0x0;
          do {
            if (lVar43 == lVar39) {
              ppuVar42 = in_R9;
              do {
                in_R9 = ppuVar42;
                if (ppuVar23 == UNRECOVERED_JUMPTABLE_00) goto code_r0x000140913bf4;
                fVar47 = *(float *)ppuVar23;
                ppuVar23 = (undefined **)((longlong)ppuVar23 + 4);
                lVar43 = *(longlong *)(*(longlong *)((longlong)puVar35 + 0x50) + 0x100);
                ppuVar42 = (undefined **)((ulonglong)(uint)fVar47 * 0xa0);
                lVar39 = *(longlong *)(lVar43 + 0x58 + (longlong)ppuVar42);
              } while (lVar39 == 0);
              uVar29 = (ulonglong)*(uint *)((longlong)ppuVar42 + lVar43 + 0x94);
              lVar7 = *(longlong *)(*(longlong *)((longlong)puVar35 + 0x50) + 0x1a8);
              if (*(ulonglong *)(*(longlong *)((longlong)puVar35 + 0x60) + 0x110) <
                  *(ulonglong *)(lVar7 + 0x40 + uVar29 * 0x48)) {
                lVar7 = lVar7 + uVar29 * 0x48;
                uVar29 = *(ulonglong *)
                          (*(longlong *)(lVar7 + 0x38) +
                          *(ulonglong *)(*(longlong *)((longlong)puVar35 + 0x60) + 0x110) * 8);
                if (uVar29 == 0) goto LAB_1409140e3;
                in_R9 = *(undefined ***)(*(longlong *)(lVar7 + 0x18) + 0x10 + ~uVar29 * 0x30);
              }
              else {
LAB_1409140e3:
                in_R9 = (undefined **)0x0;
              }
              puVar44 = *(undefined **)((longlong)ppuVar42 + lVar43 + 0x50);
              lVar43 = 0;
            }
            in_R8 = (undefined **)((longlong)in_R8 + 1);
            lVar43 = lVar43 + 1;
          } while (in_R8 != ppuVar30);
          if (lVar43 == lVar39) goto LAB_14091423a;
          goto LAB_1409142be;
        }
        in_R9 = (undefined **)0x0;
        ppuVar42 = (undefined **)0x0;
        in_R11 = (undefined *)0x0;
        in_R8 = (undefined **)0x0;
        lVar39 = *(longlong *)((longlong)puVar35 + 0x50);
        do {
          if (in_R9 == ppuVar42) {
            do {
              if (ppuVar23 == UNRECOVERED_JUMPTABLE_00) goto code_r0x000140913bf4;
              uVar29 = (ulonglong)(uint)*(float *)ppuVar23;
              ppuVar23 = (undefined **)((longlong)ppuVar23 + 4);
              in_R9 = *(undefined ***)(lVar39 + 0x1a8);
              ppuVar42 = (undefined **)in_R9[uVar29 * 9 + 2];
            } while (ppuVar42 == (undefined **)0x0);
            if (*(undefined **)(*(longlong *)((longlong)puVar35 + 0x60) + 0x110) <
                in_R9[uVar29 * 9 + 8]) {
              if (*(ulonglong *)
                   (in_R9[uVar29 * 9 + 7] +
                   (longlong)*(undefined **)(*(longlong *)((longlong)puVar35 + 0x60) + 0x110) * 8)
                  == 0) {
                in_R11 = (undefined *)0x0;
              }
              else {
                in_R11 = *(undefined **)
                          (in_R9[uVar29 * 9 + 3] +
                          ~*(ulonglong *)
                            (in_R9[uVar29 * 9 + 7] +
                            (longlong)
                            *(undefined **)(*(longlong *)((longlong)puVar35 + 0x60) + 0x110) * 8) *
                          0x30 + 0x10);
              }
              in_R9 = (undefined **)0x0;
              lVar39 = *(longlong *)((longlong)puVar35 + 0x50);
            }
            else {
              in_R9 = (undefined **)0x0;
              in_R11 = (undefined *)0x0;
            }
          }
          in_R8 = (undefined **)((longlong)in_R8 + 1);
          in_R9 = (undefined **)((longlong)in_R9 + 1);
        } while (in_R8 != ppuVar30);
        if (in_R9 == ppuVar42) goto LAB_140913c3e;
LAB_1409142e5:
        ppuVar23 = (undefined **)(in_R11 + ((ulonglong)in_R9 & 0xffffffff) * 0x30);
        puVar36 = (undefined1 *)puVar35;
      }
      *(float *)unaff_R15 = fVar50;
      puVar44 = ppuVar23[2];
      unaff_XMM7_Da = SUB84(puVar44,0);
      unaff_XMM7_Db = (float)((ulonglong)puVar44 >> 0x20);
      unaff_XMM8_Da = *(float *)(ppuVar23 + 3);
      unaff_XMM9_Da = *(float *)ppuVar23;
      unaff_XMM9_Db = *(float *)((longlong)ppuVar23 + 4);
      unaff_XMM9_Dc = *(float *)(ppuVar23 + 1);
      unaff_XMM9_Dd = *(float *)((longlong)ppuVar23 + 0xc);
      *(byte *)(_Dst + 0x9a) = 5;
      lVar39 = (longlong)_Dst[0x8b] << 4;
      puVar35 = (undefined8 *)_Dst[0x8a];
      do {
        if (lVar39 == 0) {
          pcVar26 = "dolly::drivers::position::PositionNo  driver found in the CameraRig";
          goto LAB_1409144f5;
        }
        puVar38 = puVar35 + 2;
        uVar9 = *puVar35;
        UNRECOVERED_JUMPTABLE = *(code **)(puVar35[1] + 0x40);
        *(undefined8 *)(puVar36 + -8) = 0x140914357;
        puVar24 = (undefined8 *)(*UNRECOVERED_JUMPTABLE)(uVar9);
        UNRECOVERED_JUMPTABLE = (code *)ppuVar30[3];
        *(undefined8 *)(puVar36 + -8) = 0x140914360;
        lVar43 = (*UNRECOVERED_JUMPTABLE)(puVar24);
        lVar39 = lVar39 + -0x10;
        puVar35 = puVar38;
      } while (((puVar24 == (undefined8 *)0x0) || (lVar43 != 0x6b8463577afc1a66)) ||
              (ppuVar30 != (undefined **)0xe4b9f294168d7e3));
      *puVar24 = puVar44;
      *(float *)(puVar24 + 1) = unaff_XMM8_Da;
      in_RCX = (undefined **)((longlong)_Dst[0x8b] << 4);
      unaff_R14 = (undefined **)0x9f8cbdc83fdf50d6;
      unaff_R15 = (undefined **)0xc69d4571dbc0c107;
      ppuVar30 = (undefined **)0xe4b9f294168d7e3;
      unaff_R12 = (undefined **)_Dst[0x8a];
      do {
        do {
          if (in_RCX == (undefined **)0x0) {
            pcVar26 = "dolly::drivers::rotation::Rotationdolly::drivers::look_at::LookAt";
LAB_1409144f5:
            *(char **)(puVar36 + 0x240) = pcVar26;
            *(undefined8 *)(puVar36 + 0x248) = 0x22;
            puVar35 = (undefined8 *)puVar36;
            goto LAB_140914526;
          }
          UNRECOVERED_JUMPTABLE_00 = unaff_R12 + 2;
          puVar44 = *unaff_R12;
          UNRECOVERED_JUMPTABLE = *(code **)(unaff_R12[1] + 0x40);
          *(undefined8 *)(puVar36 + -8) = 0x1409143c7;
          unaff_RBX = (undefined **)(*UNRECOVERED_JUMPTABLE)(puVar44);
          UNRECOVERED_JUMPTABLE = (code *)ppuVar30[3];
          *(undefined8 *)(puVar36 + -8) = 0x1409143d0;
          ppuVar23 = (undefined **)(*UNRECOVERED_JUMPTABLE)(unaff_RBX);
          in_RCX = in_RCX + -2;
          unaff_R12 = UNRECOVERED_JUMPTABLE_00;
        } while (unaff_RBX == (undefined **)0x0);
code_r0x0001409143dc:
      } while ((ppuVar23 != unaff_R14) || (puVar19 = puVar36, ppuVar30 != unaff_R15));
code_r0x0001409143ec:
      puVar37 = puVar19;
      unaff_XMM7_Da = unaff_XMM7_Da + (float)__xmm_00000000000000003f80000000000000;
      unaff_XMM7_Db = unaff_XMM7_Db + __xmm_00000000000000003f80000000000000._4_4_;
      *(float *)unaff_RBX = unaff_XMM9_Da;
      *(float *)((longlong)unaff_RBX + 4) = unaff_XMM9_Db;
      *(float *)(unaff_RBX + 1) = unaff_XMM9_Dc;
      *(float *)((longlong)unaff_RBX + 0xc) = unaff_XMM9_Dd;
      unaff_XMM8_Da = unaff_XMM8_Da + 0.0;
      in_RCX = (undefined **)((longlong)_Dst[0x8b] << 4);
      unaff_R14 = (undefined **)0x601d3feb7ea275df;
      unaff_R15 = (undefined **)0xcb8ec583f4439065;
      in_R8 = (undefined **)_Dst[0x8a];
LAB_140914430:
      do {
        if (in_RCX == (undefined **)0x0) {
          pcVar26 = "dolly::drivers::look_at::LookAt";
          goto code_r0x000140914512;
        }
        ppuVar23 = in_R8 + 2;
        puVar44 = *in_R8;
        UNRECOVERED_JUMPTABLE = *(code **)(in_R8[1] + 0x40);
        *(undefined8 *)(puVar37 + -8) = 0x140914447;
        lVar39 = (*UNRECOVERED_JUMPTABLE)(puVar44);
        UNRECOVERED_JUMPTABLE = (code *)ppuVar30[3];
        *(undefined8 *)(puVar37 + -8) = 0x140914450;
        UNRECOVERED_JUMPTABLE_00 = (undefined **)(*UNRECOVERED_JUMPTABLE)(lVar39);
        in_RCX = in_RCX + -2;
        in_R8 = ppuVar23;
      } while (((lVar39 == 0) || (UNRECOVERED_JUMPTABLE_00 != unaff_R14)) || (ppuVar30 != unaff_R15)
              );
      *(ulonglong *)(lVar39 + 0x14) = CONCAT44(unaff_XMM7_Db,unaff_XMM7_Da);
      *(float *)(lVar39 + 0x1c) = unaff_XMM8_Da;
      unaff_R12 = (undefined **)(ulonglong)*(uint *)(puVar37 + 0x38);
      unaff_R15 = *(undefined ***)(puVar37 + 0x48);
      puVar35 = (undefined8 *)puVar37;
    }
code_r0x000140913bf4:
    *(undefined8 *)((longlong)puVar35 + -8) = 0x140913bfc;
    ppuVar30 = _Dst;
    ::utils::camera::CameraMatrices::eye_position();
    iVar22 = (int)in_ST0;
    fVar50 = SUB84(unaff_R12,0);
    *(float *)unaff_R15 = fVar50;
    unaff_RBX = _Dst + 0x20;
    in_RCX = &anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885;
    UNRECOVERED_JUMPTABLE_00 = (undefined **)&DAT_142aabd90;
    uVar29 = (ulonglong)*(int *)(&DAT_142aabd90 + (ulonglong)*(byte *)(_Dst + 0x9a) * 4);
    ppuVar23 = (undefined **)(&DAT_142aabd90 + uVar29);
    cVar21 = (char)ppuVar23;
    in_XMM0_Da = extraout_XMM0_Da;
    in_XMM0_Db = extraout_XMM0_Db;
    in_XMM0_Dc = extraout_XMM0_Dc;
    in_XMM0_Dd = extraout_XMM0_Dd;
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
    switch(*(byte *)(_Dst + 0x9a)) {
    case 0:
      unaff_R14 = (undefined **)&DAT_0000008c;
      ppuVar23 = &anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885;
      ppuVar30 = unaff_RBX;
      break;
    case 1:
code_r0x000140913cf7:
      ppuVar23 = in_RCX;
      unaff_RBX = (undefined **)
                  CONCAT71((int7)((ulonglong)unaff_RBX >> 8),
                           (char)unaff_RBX + (char)((ulonglong)UNRECOVERED_JUMPTABLE_00 >> 8));
      unaff_R14 = (undefined **)&DAT_00000144;
      ppuVar30 = _Dst + 0x37;
      in_RCX = ppuVar23;
      break;
    case 2:
      ppuVar30 = _Dst + 0x4e;
      ppuVar23 = &anon_0555c73667d6be1dc866b3f0ae0b3f3f_17_llvm_14020587298740829885;
code_r0x000140913cdf:
      unaff_R14 = (undefined **)&DAT_000001c4;
code_r0x000140913ce5:
      break;
    case 3:
      ppuVar23 = (undefined **)&anon_0555c73667d6be1dc866b3f0ae0b3f3f_16_llvm_14020587298740829885;
      unaff_R14 = (undefined **)0x264;
      ppuVar30 = _Dst + 0x5e;
      break;
    case 4:
code_r0x000140913cbf:
      ppuVar23 = in_RCX;
      ppuVar30 = _Dst + 0x72;
      unaff_R14 = (undefined **)0x31c;
      in_RCX = ppuVar23;
code_r0x000140913ccf:
      break;
    case 5:
      ppuVar23 = &anon_0555c73667d6be1dc866b3f0ae0b3f3f_18_llvm_14020587298740829885;
      unaff_R14 = (undefined **)0x3a4;
      ppuVar30 = _Dst + 0x89;
      break;
    case 6:
      (&UNK_142aabd1d)[uVar29] = (&UNK_142aabd1d)[uVar29] + -0x70;
      ppuVar23 = (undefined **)(ulonglong)((uint)ppuVar23 ^ 0x237dc71);
      goto code_r0x000140913f8f;
    case 7:
code_r0x000140913f37:
      puVar35 = (undefined8 *)((longlong)puVar35 + 8);
      goto code_r0x000140913f29;
    case 8:
      iVar40 = (int)_Dst >> 1;
      lVar39 = CONCAT71(0x142aabd,(&UNK_142aabd91)[uVar29]) + -1;
      if (lVar39 != 0 && iVar40 == 0) {
        pcVar26 = (char *)((ulonglong)
                           (uint)((int)(int *)((ulonglong)ppuVar23 & 0xffffffffffffff10) +
                                 *(int *)((ulonglong)ppuVar23 & 0xffffffffffffff10)) - 0x77);
        *pcVar26 = *pcVar26 + (char)lVar39;
                    /* WARNING: Bad instruction - Truncating control flow here */
        halt_baddata();
      }
      unaff_RBX = (undefined **)(ulonglong)(iVar40 + 0x1b8);
      goto code_r0x000140913f8f;
    case 9:
      unaff_RBX = (undefined **)(ulonglong)((int)_Dst + 0x390);
      goto code_r0x000140913f5f;
    case 10:
      ppuVar30 = (undefined **)0x42aabda8;
      goto code_r0x000140913f46;
    case 0xb:
      in_RCX = (undefined **)0x42c91c28;
      goto code_r0x000140913f8f;
    case 0xc:
code_r0x0001409145d1:
      *(float *)(in_RCX + 0x9b) = in_XMM0_Da;
      *(float *)((longlong)in_RCX + 0x4dc) = in_XMM0_Db;
      *(float *)(in_RCX + 0x9c) = in_XMM0_Dc;
      *(float *)((longlong)in_RCX + 0x4e4) = in_XMM0_Dd;
code_r0x0001409145ce:
      puVar10 = (undefined4 *)UNRECOVERED_JUMPTABLE_00[2];
      *(float *)(in_RCX + 0x9b) = in_XMM0_Da;
      *puVar10 = (int)ppuVar23;
      goto code_r0x0001409145e3;
    case 0xd:
      cVar21 = (cVar21 + -3) - (0xfffffffebd55426f < uVar29);
      ppuVar23 = (undefined **)CONCAT71((int7)((ulonglong)ppuVar23 >> 8),cVar21);
      *(byte *)ppuVar23 = *(byte *)ppuVar23 + cVar21;
      goto code_r0x000140914664;
    case 0xe:
      goto code_r0x0001409145e9;
    case 0xf:
code_r0x0001409145fe:
      uVar41 = (int)_Dst >> 1;
      _Dst = (undefined **)(ulonglong)uVar41;
      UNRECOVERED_JUMPTABLE_00 = (undefined **)((longlong)UNRECOVERED_JUMPTABLE_00 + -1);
      if (UNRECOVERED_JUMPTABLE_00 != (undefined **)0x0 && uVar41 == 0) {
        fVar50 = (float)iVar22;
code_r0x0001409145c4:
        in_XMM0_Da = in_XMM0_Da / fVar50;
        ppuVar23 = (undefined **)(ulonglong)*(uint *)((longlong)UNRECOVERED_JUMPTABLE_00 + 0x1c);
code_r0x0001409145cb:
        in_RCX = (undefined **)*UNRECOVERED_JUMPTABLE_00;
        goto code_r0x0001409145ce;
      }
      *(undefined2 *)ppuVar23 = in_ES;
      *(byte *)ppuVar23 = *(byte *)ppuVar23 + (char)ppuVar23;
      goto code_r0x000140914609;
    case 0x10:
      *(int *)(&UNK_142aabd1d + uVar29) = *(int *)(&UNK_142aabd1d + uVar29) + 0x42aabd90;
      in_RCX = (undefined **)&DAT_00000100;
code_r0x0001409145e3:
      UNRECOVERED_JUMPTABLE_00 = (undefined **)(ulonglong)*(byte *)(in_RCX + 0x9a);
      goto code_r0x0001409145f1;
    case 0x11:
      ppuVar23 = (undefined **)0x42c91c28;
      unaff_RBX = (undefined **)&DAT_000001c4;
      goto code_r0x000140914664;
    case 0x12:
      *(byte *)ppuVar23 = *(byte *)ppuVar23 + cVar21;
      *(byte *)ppuVar23 = *(byte *)ppuVar23 + cVar21;
code_r0x0001409149a5:
      pbVar2 = (byte *)((longlong)ppuVar23 + 0x11e);
      pbVar2[0] = 0;
      pbVar2[1] = 0;
      pbVar2[2] = 0;
      pbVar2[3] = 0;
      UNRECOVERED_JUMPTABLE_00 = (undefined **)&DAT_8000000000000000;
      goto code_r0x0001409149b9;
    case 0x13:
      *(byte *)ppuVar23 = *(byte *)ppuVar23 + cVar21;
      goto code_r0x000140914a0b;
    case 0x14:
      in_R8 = (undefined **)(ulonglong)*(byte *)(ppuVar30 + 0x7a);
      goto code_r0x0001409149db;
    case 0x15:
      goto code_r0x0001409149ed;
    case 0x16:
      *(undefined8 **)((longlong)puVar35 + -8) = unaff_RBP;
code_r0x0001409149b9:
      ppuVar23[0x19] = (undefined *)UNRECOVERED_JUMPTABLE_00;
      return;
    case 0x17:
      goto code_r0x000140914a1e;
    }
    UNRECOVERED_JUMPTABLE = (code *)ppuVar23[6];
    *(undefined **)((longlong)puVar35 + -8) = &UNK_140913d31;
    (*UNRECOVERED_JUMPTABLE)((undefined1 *)((longlong)puVar35 + 0xb0),ppuVar30);
    *(undefined4 *)((longlong)puVar35 + 0x2c8) = *(undefined4 *)((longlong)puVar35 + 0xb8);
    *(undefined8 *)((longlong)puVar35 + 0x2c0) = *(undefined8 *)((longlong)puVar35 + 0xb0);
    uVar13 = *(undefined8 *)((longlong)puVar35 + 0xc0);
    uVar14 = *(undefined8 *)((longlong)puVar35 + 200);
    *(undefined **)((longlong)puVar35 + -8) = &UNK_140913d67;
    fVar50 = expf(unaff_XMM6_Da / __real_be4ccccd);
    uVar9 = *(undefined8 *)((longlong)((longlong)_Dst + 0x104) + (longlong)unaff_R14);
    *(float *)((longlong)((longlong)_Dst + 0x104) + (longlong)unaff_R14) =
         (__real_3f800000 - fVar50) * (float)((ulonglong)uVar9 >> 0x20) +
         (__real_3f800000 - (__real_3f800000 - fVar50)) * (float)uVar9;
    *(undefined8 *)((longlong)puVar35 + 0x2d0) = uVar13;
    *(undefined8 *)((longlong)puVar35 + 0x2d8) = uVar14;
    fVar50 = *(float *)(_Dst + 0x9b);
    uVar13 = *(undefined8 *)((longlong)unaff_RBX + (longlong)unaff_R14);
    uVar14 = *(undefined8 *)((byte *)((longlong)unaff_RBX + (longlong)unaff_R14) + 8);
    uVar15 = *(undefined8 *)((longlong)(unaff_RBX + 2) + (longlong)unaff_R14);
    uVar16 = *(undefined8 *)((byte *)((longlong)(unaff_RBX + 2) + (longlong)unaff_R14) + 8);
    uVar9 = *(undefined8 *)((longlong)(unaff_RBX + 4) + (longlong)unaff_R14);
    *(undefined4 *)((longlong)puVar35 + 0x30c) =
         *(undefined4 *)((longlong)(unaff_RBX + 5) + (longlong)unaff_R14);
    *(undefined8 *)((longlong)puVar35 + 0x304) = uVar9;
    *(undefined8 *)((longlong)puVar35 + 0x2f4) = uVar15;
    *(undefined8 *)((longlong)puVar35 + 0x2fc) = uVar16;
    *(undefined8 *)((longlong)puVar35 + 0x2e4) = uVar13;
    *(undefined8 *)((longlong)puVar35 + 0x2ec) = uVar14;
    *(float *)((longlong)puVar35 + 0x2e0) = fVar50;
    *(undefined **)((longlong)puVar35 + -8) = &UNK_140913dfe;
    ::utils::camera::impl$2::into_camera_body_matrices();
    *(undefined **)((longlong)puVar35 + -8) = &UNK_140913e13;
    ::utils::camera::CameraProjectionParams::calc_matrices();
    in_XMM0_Da = *(float *)((longlong)puVar35 + 0x240);
    in_XMM0_Db = *(float *)((longlong)puVar35 + 0x244);
    in_XMM0_Dc = *(float *)((longlong)puVar35 + 0x248);
    in_XMM0_Dd = *(float *)((longlong)puVar35 + 0x24c);
    in_XMM1_Da = *(float *)((longlong)puVar35 + 0x250);
    in_XMM1_Db = *(undefined4 *)((longlong)puVar35 + 0x254);
    in_XMM1_Dc = *(undefined4 *)((longlong)puVar35 + 600);
    in_XMM1_Dd = *(undefined4 *)((longlong)puVar35 + 0x25c);
code_r0x000140913e23:
    in_XMM2 = *(undefined1 (*) [16])((longlong)puVar35 + 0x260);
code_r0x000140913e2b:
    *(undefined8 *)((longlong)puVar35 + 0xe0) = *(undefined8 *)((longlong)puVar35 + 0x270);
    *(undefined8 *)((longlong)puVar35 + 0xe8) = *(undefined8 *)((longlong)puVar35 + 0x278);
    *(undefined1 (*) [16])((longlong)puVar35 + 0xd0) = in_XMM2;
    *(float *)((longlong)puVar35 + 0xc0) = in_XMM1_Da;
    *(undefined4 *)((longlong)puVar35 + 0xc4) = in_XMM1_Db;
    *(undefined4 *)((longlong)puVar35 + 200) = in_XMM1_Dc;
    *(undefined4 *)((longlong)puVar35 + 0xcc) = in_XMM1_Dd;
code_r0x000140913e4b:
    fVar50 = SUB84(unaff_R12,0);
    *(float *)((longlong)puVar35 + 0xb0) = in_XMM0_Da;
    *(float *)((longlong)puVar35 + 0xb4) = in_XMM0_Db;
    *(float *)((longlong)puVar35 + 0xb8) = in_XMM0_Dc;
    *(float *)((longlong)puVar35 + 0xbc) = in_XMM0_Dd;
    *(undefined8 *)((longlong)puVar35 + 0xf0) = *(undefined8 *)((longlong)puVar35 + 0x280);
    *(undefined8 *)((longlong)puVar35 + 0xf8) = *(undefined8 *)((longlong)puVar35 + 0x288);
    *(undefined8 *)((longlong)puVar35 + 0x100) = *(undefined8 *)((longlong)puVar35 + 0x290);
    *(undefined8 *)((longlong)puVar35 + 0x108) = *(undefined8 *)((longlong)puVar35 + 0x298);
    *(undefined8 *)((longlong)puVar35 + 0x110) = *(undefined8 *)((longlong)puVar35 + 0x2a0);
    *(undefined8 *)((longlong)puVar35 + 0x118) = *(undefined8 *)((longlong)puVar35 + 0x2a8);
    *(undefined8 *)((longlong)puVar35 + 0x120) = *(undefined8 *)((longlong)puVar35 + 0x2b0);
    *(undefined8 *)((longlong)puVar35 + 0x128) = *(undefined8 *)((longlong)puVar35 + 0x2b8);
    *(undefined8 *)((longlong)puVar35 + 0x130) = *(undefined8 *)((longlong)puVar35 + 0x1c0);
    *(undefined8 *)((longlong)puVar35 + 0x138) = *(undefined8 *)((longlong)puVar35 + 0x1c8);
    *(undefined8 *)((longlong)puVar35 + 0x140) = *(undefined8 *)((longlong)puVar35 + 0x1d0);
    *(undefined8 *)((longlong)puVar35 + 0x148) = *(undefined8 *)((longlong)puVar35 + 0x1d8);
    *(undefined8 *)((longlong)puVar35 + 0x150) = *(undefined8 *)((longlong)puVar35 + 0x1e0);
    *(undefined8 *)((longlong)puVar35 + 0x158) = *(undefined8 *)((longlong)puVar35 + 0x1e8);
    *(undefined8 *)((longlong)puVar35 + 0x160) = *(undefined8 *)((longlong)puVar35 + 0x1f0);
    *(undefined8 *)((longlong)puVar35 + 0x168) = *(undefined8 *)((longlong)puVar35 + 0x1f8);
    *(undefined8 *)((longlong)puVar35 + 0x1a0) = *(undefined8 *)((longlong)puVar35 + 0x230);
    *(undefined8 *)((longlong)puVar35 + 0x1a8) = *(undefined8 *)((longlong)puVar35 + 0x238);
    *(undefined8 *)((longlong)puVar35 + 400) = *(undefined8 *)((longlong)puVar35 + 0x220);
    *(undefined8 *)((longlong)puVar35 + 0x198) = *(undefined8 *)((longlong)puVar35 + 0x228);
    *(undefined8 *)((longlong)puVar35 + 0x180) = *(undefined8 *)((longlong)puVar35 + 0x210);
    *(undefined8 *)((longlong)puVar35 + 0x188) = *(undefined8 *)((longlong)puVar35 + 0x218);
    *(undefined8 *)((longlong)puVar35 + 0x170) = *(undefined8 *)((longlong)puVar35 + 0x200);
    *(undefined8 *)((longlong)puVar35 + 0x178) = *(undefined8 *)((longlong)puVar35 + 0x208);
code_r0x000140913f13:
    ppuVar30 = (undefined **)((longlong)puVar35 + 0xb0);
code_r0x000140913f1b:
    in_R8 = (undefined **)&DAT_00000100;
code_r0x000140913f21:
    *(undefined **)((longlong)puVar35 + -8) = &UNK_140913f29;
    memcpy(_Dst,ppuVar30,(size_t)in_R8);
code_r0x000140913f29:
    *(float *)unaff_R15 = fVar50;
    ppuVar23 = (undefined **)(ulonglong)*(byte *)(*(longlong *)((longlong)puVar35 + 0x1b0) + 0x58);
    UNRECOVERED_JUMPTABLE_00 = (undefined **)(ulonglong)*(byte *)(_Dst + 0x9a);
    ppuVar30 = (undefined **)&UNK_142aabda8;
code_r0x000140913f46:
    uVar29 = (ulonglong)(int)*(float *)((longlong)ppuVar30 + (longlong)UNRECOVERED_JUMPTABLE_00 * 4)
    ;
code_r0x000140913f4a:
    UNRECOVERED_JUMPTABLE_00 = (undefined **)(uVar29 + (longlong)ppuVar30);
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
    switch(UNRECOVERED_JUMPTABLE_00) {
    case (undefined **)0x140913f4f:
      unaff_RBX = _Dst + 0x37;
      break;
    case (undefined **)0x140913f58:
      unaff_RBX = _Dst + 0x72;
code_r0x000140913f5f:
      break;
    case (undefined **)0x140913f61:
      unaff_RBX = _Dst + 0x4e;
      in_RCX = &anon_0555c73667d6be1dc866b3f0ae0b3f3f_17_llvm_14020587298740829885;
      break;
    case (undefined **)0x140913f71:
      unaff_RBX = _Dst + 0x5e;
      in_RCX = (undefined **)&anon_0555c73667d6be1dc866b3f0ae0b3f3f_16_llvm_14020587298740829885;
      break;
    case (undefined **)0x140913f81:
      unaff_RBX = _Dst + 0x89;
      in_RCX = &anon_0555c73667d6be1dc866b3f0ae0b3f3f_18_llvm_14020587298740829885;
      break;
    case (undefined **)0x1409145e9:
code_r0x0001409145e9:
      *(byte *)((longlong)ppuVar23 + -0x73) =
           *(byte *)((longlong)ppuVar23 + -0x73) + (char)UNRECOVERED_JUMPTABLE_00;
      goto code_r0x0001409145f1;
    case (undefined **)0x1409145f3:
code_r0x0001409145f1:
                    /* WARNING: Could not recover jumptable at 0x0001409145ff. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      (*(code *)(&DAT_142aabdc0 + *(int *)(&DAT_142aabdc0 + (longlong)UNRECOVERED_JUMPTABLE_00 * 4))
      )(&DAT_142aabdc0 + *(int *)(&DAT_142aabdc0 + (longlong)UNRECOVERED_JUMPTABLE_00 * 4));
      return;
    case (undefined **)0x140914601:
      unaff_RBX = (undefined **)&DAT_0000008c;
code_r0x000140914609:
code_r0x000140914664:
      UNRECOVERED_JUMPTABLE = (code *)ppuVar23[6];
      *(undefined **)((longlong)puVar35 + -8) = &UNK_140914672;
      (*UNRECOVERED_JUMPTABLE)((undefined1 *)((longlong)puVar35 + 0x70));
      *(undefined4 *)((longlong)puVar35 + 0x28) = *(undefined4 *)((longlong)puVar35 + 0x78);
      *(undefined8 *)((longlong)puVar35 + 0x20) = *(undefined8 *)((longlong)puVar35 + 0x70);
      uVar9 = *(undefined8 *)((longlong)puVar35 + 0x80);
      uVar13 = *(undefined8 *)((longlong)puVar35 + 0x88);
      *(float *)((longlong)((longlong)in_RCX + 0x104) + (longlong)unaff_RBX) =
           *(float *)((longlong)(in_RCX + 0x21) + (longlong)unaff_RBX) * 0.0 +
           *(float *)((longlong)((longlong)in_RCX + 0x104) + (longlong)unaff_RBX);
      *(undefined8 *)((longlong)puVar35 + 0x30) = uVar9;
      *(undefined8 *)((longlong)puVar35 + 0x38) = uVar13;
      fVar50 = *(float *)(in_RCX + 0x9b);
      uVar13 = *(undefined8 *)((longlong)_Dst + (longlong)unaff_RBX);
      uVar14 = *(undefined8 *)((byte *)((longlong)_Dst + (longlong)unaff_RBX) + 8);
      uVar15 = *(undefined8 *)((longlong)(_Dst + 2) + (longlong)unaff_RBX);
      uVar16 = *(undefined8 *)((byte *)((longlong)(_Dst + 2) + (longlong)unaff_RBX) + 8);
      uVar9 = *(undefined8 *)((longlong)(_Dst + 4) + (longlong)unaff_RBX);
      *(undefined4 *)((longlong)puVar35 + 0x6c) =
           *(undefined4 *)((longlong)(_Dst + 5) + (longlong)unaff_RBX);
      *(undefined8 *)((longlong)puVar35 + 100) = uVar9;
      *(undefined8 *)((longlong)puVar35 + 0x54) = uVar15;
      *(undefined8 *)((longlong)puVar35 + 0x5c) = uVar16;
      *(undefined8 *)((longlong)puVar35 + 0x44) = uVar13;
      *(undefined8 *)((longlong)puVar35 + 0x4c) = uVar14;
      *(float *)((longlong)puVar35 + 0x40) = fVar50;
      *(undefined **)((longlong)puVar35 + -8) = &UNK_1409146f1;
      ::utils::camera::impl$2::into_camera_body_matrices();
      *(undefined **)((longlong)puVar35 + -8) = &UNK_140914703;
      ::utils::camera::CameraProjectionParams::calc_matrices();
      *(undefined8 *)((longlong)puVar35 + 0xa0) = *(undefined8 *)((longlong)puVar35 + 0x220);
      *(undefined8 *)((longlong)puVar35 + 0xa8) = *(undefined8 *)((longlong)puVar35 + 0x228);
      *(undefined8 *)((longlong)puVar35 + 0x90) = *(undefined8 *)((longlong)puVar35 + 0x210);
      *(undefined8 *)((longlong)puVar35 + 0x98) = *(undefined8 *)((longlong)puVar35 + 0x218);
      *(undefined8 *)((longlong)puVar35 + 0x80) = *(undefined8 *)((longlong)puVar35 + 0x200);
      *(undefined8 *)((longlong)puVar35 + 0x88) = *(undefined8 *)((longlong)puVar35 + 0x208);
      *(undefined8 *)((longlong)puVar35 + 0x70) = *(undefined8 *)((longlong)puVar35 + 0x1f0);
      *(undefined8 *)((longlong)puVar35 + 0x78) = *(undefined8 *)((longlong)puVar35 + 0x1f8);
      *(undefined8 *)((longlong)puVar35 + 0xb0) = *(undefined8 *)((longlong)puVar35 + 0x230);
      *(undefined8 *)((longlong)puVar35 + 0xb8) = *(undefined8 *)((longlong)puVar35 + 0x238);
      *(undefined8 *)((longlong)puVar35 + 0xc0) = *(undefined8 *)((longlong)puVar35 + 0x240);
      *(undefined8 *)((longlong)puVar35 + 200) = *(undefined8 *)((longlong)puVar35 + 0x248);
      *(undefined8 *)((longlong)puVar35 + 0xd0) = *(undefined8 *)((longlong)puVar35 + 0x250);
      *(undefined8 *)((longlong)puVar35 + 0xd8) = *(undefined8 *)((longlong)puVar35 + 600);
      *(undefined8 *)((longlong)puVar35 + 0xe0) = *(undefined8 *)((longlong)puVar35 + 0x260);
      *(undefined8 *)((longlong)puVar35 + 0xe8) = *(undefined8 *)((longlong)puVar35 + 0x268);
      *(undefined8 *)((longlong)puVar35 + 0xf0) = *(undefined8 *)((longlong)puVar35 + 0x170);
      *(undefined8 *)((longlong)puVar35 + 0xf8) = *(undefined8 *)((longlong)puVar35 + 0x178);
      *(undefined8 *)((longlong)puVar35 + 0x100) = *(undefined8 *)((longlong)puVar35 + 0x180);
      *(undefined8 *)((longlong)puVar35 + 0x108) = *(undefined8 *)((longlong)puVar35 + 0x188);
      *(undefined8 *)((longlong)puVar35 + 0x110) = *(undefined8 *)((longlong)puVar35 + 400);
      *(undefined8 *)((longlong)puVar35 + 0x118) = *(undefined8 *)((longlong)puVar35 + 0x198);
      *(undefined8 *)((longlong)puVar35 + 0x120) = *(undefined8 *)((longlong)puVar35 + 0x1a0);
      *(undefined8 *)((longlong)puVar35 + 0x128) = *(undefined8 *)((longlong)puVar35 + 0x1a8);
      *(undefined8 *)((longlong)puVar35 + 0x160) = *(undefined8 *)((longlong)puVar35 + 0x1e0);
      *(undefined8 *)((longlong)puVar35 + 0x168) = *(undefined8 *)((longlong)puVar35 + 0x1e8);
      *(undefined8 *)((longlong)puVar35 + 0x150) = *(undefined8 *)((longlong)puVar35 + 0x1d0);
      *(undefined8 *)((longlong)puVar35 + 0x158) = *(undefined8 *)((longlong)puVar35 + 0x1d8);
      *(undefined8 *)((longlong)puVar35 + 0x140) = *(undefined8 *)((longlong)puVar35 + 0x1c0);
      *(undefined8 *)((longlong)puVar35 + 0x148) = *(undefined8 *)((longlong)puVar35 + 0x1c8);
      *(undefined8 *)((longlong)puVar35 + 0x130) = *(undefined8 *)((longlong)puVar35 + 0x1b0);
      *(undefined8 *)((longlong)puVar35 + 0x138) = *(undefined8 *)((longlong)puVar35 + 0x1b8);
      *(undefined **)((longlong)puVar35 + -8) = &UNK_140914813;
      memcpy(in_RCX,(undefined1 *)((longlong)puVar35 + 0x70),0x100);
      return;
    case (undefined **)0x140914616:
                    /* WARNING: Bad instruction - Truncating control flow here */
      halt_baddata();
    case (undefined **)0x14091462b:
      ppuVar23 = (undefined **)&anon_0555c73667d6be1dc866b3f0ae0b3f3f_16_llvm_14020587298740829885;
      unaff_RBX = (undefined **)0x264;
      goto code_r0x000140914664;
    case (undefined **)0x140914639:
                    /* WARNING: Bad instruction - Truncating control flow here */
      halt_baddata();
    case (undefined **)0x1409149c3:
      UNRECOVERED_JUMPTABLE = (code *)swi(3);
      (*UNRECOVERED_JUMPTABLE)();
      return;
    case (undefined **)0x1409149d4:
code_r0x0001409149db:
      UNRECOVERED_JUMPTABLE_00 = &anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885
      ;
      in_R9 = (undefined **)&DAT_142aabdd8;
    case (undefined **)0x1409149e9:
      goto code_r0x0001409149e9;
    case (undefined **)0x140914a05:
      ppuVar23 = (undefined **)(ulonglong)((int)ppuVar30 + 0x290);
code_r0x000140914a0b:
      lVar39 = 0x300;
      lVar43 = 0x31c;
      goto code_r0x000140914a80;
    case (undefined **)0x140914a21:
code_r0x000140914a1e:
      *(byte *)ppuVar23 = *(byte *)ppuVar23 + (char)ppuVar23;
      UNRECOVERED_JUMPTABLE_00 = &anon_0555c73667d6be1dc866b3f0ae0b3f3f_17_llvm_14020587298740829885
      ;
      lVar39 = 0x1a8;
      lVar43 = 0x1c4;
      goto code_r0x000140914a80;
    case (undefined **)0x140914a36:
      ppuVar23 = (undefined **)(ulonglong)((int)ppuVar30 + 0x1f0);
      UNRECOVERED_JUMPTABLE_00 =
           (undefined **)&anon_0555c73667d6be1dc866b3f0ae0b3f3f_16_llvm_14020587298740829885;
      lVar39 = 0x248;
      lVar43 = 0x264;
code_r0x000140914a80:
      *ppuVar23 = (undefined *)in_R8;
      ppuVar23[1] = (undefined *)UNRECOVERED_JUMPTABLE_00;
      ppuVar23[2] = (byte *)(lVar43 + (longlong)ppuVar30);
      ppuVar23[3] = (byte *)(lVar39 + (longlong)ppuVar30);
      return;
    }
code_r0x000140913f8f:
    UNRECOVERED_JUMPTABLE = (code *)in_RCX[4];
    *(undefined **)((longlong)puVar35 + -8) = &UNK_140913fab;
    (*UNRECOVERED_JUMPTABLE)
              (unaff_RBX,(undefined1 *)((longlong)puVar35 + 0x310),
               *(undefined8 *)((longlong)puVar35 + 0x1b8),(char)ppuVar23 == '\x04');
    *(undefined **)((longlong)puVar35 + -8) = &UNK_140913fbb;
    ::utils::camera::CameraMatrices::eye_position();
    if ((*(uint *)(unaff_RBP + 2) < 3) &&
       (unaff_XMM6_Da * __real_3e4ccccd <
        SQRT((*(float *)((longlong)puVar35 + 0xa0) - *(float *)((longlong)puVar35 + 0xac)) *
             (*(float *)((longlong)puVar35 + 0xa0) - *(float *)((longlong)puVar35 + 0xac)) +
             (*(float *)((longlong)puVar35 + 0x9c) - *(float *)((longlong)puVar35 + 0xa8)) *
             (*(float *)((longlong)puVar35 + 0x9c) - *(float *)((longlong)puVar35 + 0xa8)) +
             (*(float *)((longlong)puVar35 + 0x98) - *(float *)((longlong)puVar35 + 0xa4)) *
             (*(float *)((longlong)puVar35 + 0x98) - *(float *)((longlong)puVar35 + 0xa4))))) {
      **(undefined4 **)((longlong)puVar35 + 0x70) = *(undefined4 *)((longlong)puVar35 + 0x3c);
      lVar39 = unaff_R13[6];
      lVar43 = unaff_R13[8];
      if (lVar39 == unaff_R13[4]) {
        *(undefined **)((longlong)puVar35 + -8) = &UNK_140914054;
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar7 = unaff_R13[5];
      *(undefined4 *)(lVar7 + lVar39 * 0x10) = 2;
      *(longlong *)(lVar7 + 8 + lVar39 * 0x10) = lVar43;
      unaff_R13[6] = lVar39 + 1;
      unaff_R13[8] = unaff_R13[8] + 1;
    }
    return;
  case 0x48:
    goto code_r0x000140913b04;
  case 0x49:
    anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_4_ =
         CONCAT31(anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._1_3_,
                  (char)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 + -0x68);
    puVar35 = (undefined8 *)auStack_298;
    goto LAB_140913bd4;
  case 0x4a:
    *(float *)(uVar29 + 0x28573d90c) = unaff_XMM9_Da;
    *(float *)(uVar29 + 0x28573d910) = unaff_XMM9_Db;
    *(float *)(uVar29 + 0x28573d914) = unaff_XMM9_Dc;
    *(float *)(uVar29 + 0x28573d918) = unaff_XMM9_Dd;
    goto LAB_140913ba0;
  case 0x4b:
                    /* WARNING: Bad instruction - Truncating control flow here */
    halt_baddata();
  case 0x4c:
    *(int *)(&UNK_142aabbfb + uVar29) = *(int *)(&UNK_142aabbfb + uVar29) + -1;
    *(byte *)((longlong)in_R8 + -0x77) = *(byte *)((longlong)in_R8 + -0x77) - (char)in_R9;
    UNRECOVERED_JUMPTABLE = (code *)swi(1);
    (*UNRECOVERED_JUMPTABLE)();
    return;
  case 0x4d:
    anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_4_ =
         CONCAT31(anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._1_3_,
                  (char)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 + -0x68);
    PTR_update_142c91c20._1_1_ = PTR_update_142c91c20._1_1_ + bVar28;
    puVar35 = (undefined8 *)auStack_298;
    goto code_r0x000140913bf4;
  case 0x4e:
    *(byte *)_Dst = *(byte *)_Dst + bVar28;
    uStack_1a8 = CONCAT44(uStack_1a8._4_4_,(int)uStack_1a8 + -0x42c91c98);
    puVar35 = (undefined8 *)auStack_298;
    goto code_r0x000140913e4b;
  case 0x4f:
    *(byte *)_Dst = *(byte *)_Dst + bVar28;
    cStack_48 = cStack_48 - bVar28;
    puVar35 = (undefined8 *)auStack_298;
    goto code_r0x000140913e23;
  case 0x50:
    anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_4_ =
         CONCAT31(anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._1_3_,
                  (char)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 + -0x68);
    puVar35 = (undefined8 *)auStack_298;
    goto code_r0x000140913e2b;
  case 0x51:
    anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_4_ =
         CONCAT31(anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._1_3_,
                  (char)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 + -0x68);
    puVar35 = (undefined8 *)auStack_298;
    goto code_r0x000140913e2b;
  case 0x52:
    *(byte *)_Dst = *(byte *)_Dst + bVar28;
    cStack_38 = cStack_38 + -0x70;
    puVar35 = (undefined8 *)auStack_298;
    goto code_r0x000140913e2b;
  case 0x53:
    anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_4_ =
         CONCAT31(anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._1_3_,
                  (char)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 + -0x68);
    puVar35 = (undefined8 *)auStack_298;
    goto code_r0x000140913e4b;
  case 0x54:
    (*(code *)PTR_rotate_yaw_pitch_142c91c08)();
  case 0x58:
    puVar35 = (undefined8 *)auStack_298;
    goto LAB_1409135c3;
  case 0x55:
    pcVar26 = (char *)((ulonglong)(0xbe3fd496 - bVar45) & 0xffffffffffffff40);
    uVar27 = (undefined7)((ulonglong)pcVar26 >> 8);
    cVar21 = (char)pcVar26 + *pcVar26;
    pcVar25 = (char *)CONCAT71(uVar27,cVar21);
    pcVar26 = pcVar25 + -0x39;
    *pcVar26 = *pcVar26 + bVar28;
    ppuVar23 = (undefined **)CONCAT71(uVar27,cVar21 + *pcVar25);
  case 0x59:
    cVar21 = (char)ppuVar23 + *(char *)ppuVar23;
    pcVar26 = (char *)CONCAT71((int7)((ulonglong)ppuVar23 >> 8),cVar21);
    _DAT_142aabc70 = CONCAT31(_UNK_142aabc71,DAT_142aabc70 + (char)((ulonglong)ppuVar23 >> 8));
    *pcVar26 = *pcVar26 + cVar21;
    pcVar26 = (char *)(ulonglong)((int)pcVar26 + 0x21d88f3);
    puVar37 = auStack_298;
code_r0x000140914512:
    *(char **)(puVar37 + 0x240) = pcVar26;
    *(undefined8 *)(puVar37 + 0x248) = 0x1f;
    puVar35 = (undefined8 *)puVar37;
    goto LAB_140914526;
  case 0x56:
    do {
                    /* WARNING: Do nothing block with infinite loop */
    } while( true );
  case 0x57:
                    /* WARNING: Bad instruction - Truncating control flow here */
    halt_baddata();
  case 0x5a:
                    /* WARNING: Bad instruction - Truncating control flow here */
    halt_baddata();
  case 0x5b:
    if (-0x142aabc71 < (longlong)uVar29) {
      anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_4_ =
           CONCAT31(anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._1_3_,
                    (char)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 + -0x68
                   );
    }
    goto code_r0x0001409148ec;
  case 0x5c:
    goto code_r0x0001409148b1;
  case 0x5d:
    _anon_0555c73667d6be1dc866b3f0ae0b3f3f_21_llvm_14020587298740829885 = UNRECOVERED_JUMPTABLE_00;
    goto code_r0x0001409148b1;
  case 0x5e:
    PTR_update_142c91c58._7_1_ = PTR_update_142c91c58._7_1_ + bVar28 * '\x02' + bVar45;
    iVar40 = (int)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 + 0x42c91c98;
    anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_1_ = (char)iVar40;
    anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_1_ =
         (char)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 + -0x68;
    anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._1_3_ =
         (undefined3)((uint)iVar40 >> 8);
    *(byte *)in_R8 = *(byte *)in_R8 | bVar28;
    anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_4_ =
         CONCAT31(anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._1_3_,
                  (char)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 + -0x68);
    PTR_update_142c91c58._7_1_ = PTR_update_142c91c58._7_1_ + bVar28;
    *(byte *)in_R8 = *(byte *)in_R8 & bVar28;
code_r0x0001409148b1:
    PTR_update_142c91c58._7_1_ = PTR_update_142c91c58._7_1_ + bVar28;
    anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_1_ =
         (char)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 + -0x68;
    *(byte *)in_R8 = *(byte *)in_R8 + 0x68;
    anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_1_ =
         (char)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 + -0x68;
    *(byte *)in_R8 = *(byte *)in_R8 ^ 0x98;
    anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_4_ =
         CONCAT31(anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._1_3_,
                  (char)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 + -0x68);
    PTR_translate_screen_space_142c91c50._1_1_ = PTR_translate_screen_space_142c91c50._1_1_ + bVar28
    ;
    iVar40 = (int)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 + 0x42c91c98;
    anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_1_ = (char)iVar40;
    anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._1_3_ =
         (undefined3)((uint)iVar40 >> 8);
    anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_4_ =
         CONCAT31(anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._1_3_,
                  (char)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 + '0');
    cRam000000017b11a5e0 = cRam000000017b11a5e0 + -0x68;
    _UNK_142c91cf8 = 0x43800000780;
    _UNK_142c91d00 = 0;
    _UNK_142c91d14 = 1;
code_r0x0001409148ec:
    _UNK_142c91d18 = 0;
    _UNK_142c91d20 = 0;
    puVar35 = (undefined8 *)auStack_298;
    goto code_r0x000140914900;
  case 0x5f:
    anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._0_4_ =
         CONCAT31(anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885._1_3_,
                  (char)anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885 + -0x68);
    puVar35 = (undefined8 *)auStack_298;
    goto code_r0x000140914900;
  }
  (*(code *)ppuVar23[6])(&puStack_228,UNRECOVERED_JUMPTABLE_00);
  fStack_270 = (float)puStack_220._0_4_;
  puStack_278 = puStack_228;
  fVar50 = expf(___real_c8f42400);
  uVar9 = *(undefined8 *)((longlong)in_RCX + lVar39 + 0x104);
  *(float *)((longlong)in_RCX + lVar39 + 0x104) =
       (__real_3f800000 - fVar50) * (float)((ulonglong)uVar9 >> 0x20) +
       (__real_3f800000 - (__real_3f800000 - fVar50)) * (float)uVar9;
  uStack_268 = puStack_218;
  puStack_260 = puStack_210;
  fStack_258 = *(float *)(in_RCX + 0x9b);
  uVar9 = *(undefined8 *)((longlong)_Dst + lVar39);
  uVar13 = *(undefined8 *)((byte *)((longlong)_Dst + lVar39) + 8);
  pbVar2 = (byte *)((longlong)in_RCX + lVar39 + 0x110);
  uVar14 = *(undefined8 *)pbVar2;
  uVar15 = *(undefined8 *)(pbVar2 + 8);
  pbVar2 = (byte *)((longlong)in_RCX + lVar39 + 0x11c);
  uVar16 = *(undefined8 *)(pbVar2 + 8);
  uStack_234 = (undefined4)((ulonglong)*(undefined8 *)pbVar2 >> 0x20);
  uStack_230 = (undefined4)uVar16;
  fStack_22c = (float)((ulonglong)uVar16 >> 0x20);
  uStack_244 = (undefined4)uVar14;
  uStack_240 = (undefined4)((ulonglong)uVar14 >> 0x20);
  uStack_23c = (undefined4)uVar15;
  uStack_238 = (undefined4)((ulonglong)uVar15 >> 0x20);
  uStack_254 = (undefined4)uVar9;
  uStack_250 = (undefined4)((ulonglong)uVar9 >> 0x20);
  uStack_24c = (undefined4)uVar13;
  uStack_248 = (undefined4)((ulonglong)uVar13 >> 0x20);
  ::utils::camera::impl$2::into_camera_body_matrices();
  ::utils::camera::CameraProjectionParams::calc_matrices();
  uStack_1f8 = uStack_78;
  uStack_1f0 = uStack_70;
  lStack_208 = lStack_88;
  uStack_200 = uStack_80;
  puStack_218 = puStack_98;
  puStack_210 = puStack_90;
  puStack_228 = puStack_a8;
  puStack_220 = puStack_a0;
  uStack_1e8 = uStack_68;
  uStack_1e0 = (undefined4)uStack_60;
  uStack_1dc = (undefined4)((ulonglong)uStack_60 >> 0x20);
  uStack_1d8 = (undefined4)uStack_58;
  uStack_1d4 = (undefined4)((ulonglong)uStack_58 >> 0x20);
  uStack_1d0 = uStack_50;
  uStack_1c0 = uStack_40;
  uStack_1b0 = uStack_30;
  uStack_1a8 = uStack_128;
  uStack_1a0 = uStack_120;
  uStack_198 = uStack_118;
  uStack_190 = uStack_110;
  uStack_188 = uStack_108;
  uStack_180 = uStack_100;
  uStack_178 = uStack_f8;
  uStack_170 = uStack_f0;
  uStack_138 = uStack_b8;
  uStack_130 = uStack_b0;
  uStack_148 = uStack_c8;
  uStack_140 = uStack_c0;
  uStack_158 = uStack_d8;
  uStack_168 = uStack_e8;
  uStack_160 = uStack_e0;
  memcpy(in_RCX,&puStack_228,0x100);
  return;
code_r0x0001409149e9:
  in_R8 = (undefined **)(longlong)(int)*(float *)((longlong)in_R9 + (longlong)in_R8 * 4);
code_r0x0001409149ed:
                    /* WARNING: Could not recover jumptable at 0x0001409149f0. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (*(code *)((longlong)in_R8 + (longlong)in_R9))(UNRECOVERED_JUMPTABLE_00);
  return;
}

