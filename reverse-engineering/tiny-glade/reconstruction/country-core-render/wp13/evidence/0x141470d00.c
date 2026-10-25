// name: parse_saved_game_data size: 714
// void __cdecl parse_saved_game_data(void)

// WARNING: Control flow encountered bad instruction data
// WARNING: Instruction at (ram,0x0001414726e4) overlaps instruction at (ram,0x0001414726df)
// 
// WARNING: Removing unreachable block (ram,0x00014147170f)
// WARNING: Removing unreachable block (ram,0x0001414716f6)
// WARNING: Type propagation algorithm not settling
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl country_core::systems::save::load_game::parse_saved_game_data(void)

{
  char *pcVar1;
  int *piVar2;
  undefined8 *puVar3;
  undefined4 *puVar4;
  undefined8 ****ppppuVar5;
  longlong *plVar6;
  undefined8 ******ppppppuVar7;
  longlong lVar8;
  ushort uVar9;
  undefined8 uVar10;
  undefined8 *****pppppuVar11;
  undefined *puVar12;
  undefined1 *puVar13;
  undefined1 *puVar14;
  undefined1 *puVar15;
  uint uVar16;
  char cVar17;
  byte bVar18;
  undefined4 uVar19;
  undefined8 *****extraout_RAX;
  undefined8 *****extraout_RAX_00;
  undefined1 (*extraout_RAX_01) [16];
  undefined8 *****extraout_RAX_02;
  undefined8 *****extraout_RAX_03;
  undefined8 *****extraout_RAX_04;
  byte *extraout_RAX_05;
  byte *pbVar20;
  undefined1 (*extraout_RAX_06) [16];
  undefined8 *****extraout_RAX_07;
  undefined8 ******extraout_RAX_08;
  undefined8 ******extraout_RAX_09;
  undefined8 ******extraout_RAX_10;
  char *extraout_RAX_11;
  undefined8 ******extraout_RAX_12;
  undefined8 ******extraout_RAX_13;
  undefined8 ******extraout_RAX_14;
  undefined8 ******extraout_RAX_15;
  undefined1 (*pauVar21) [16];
  undefined1 (*pauVar22) [16];
  undefined8 ******ppppppuVar23;
  undefined8 *****pppppuVar24;
  longlong extraout_RAX_16;
  undefined8 ******extraout_RAX_17;
  undefined8 ******extraout_RAX_18;
  undefined8 ******extraout_RAX_19;
  undefined8 *****_Dst;
  undefined8 ******extraout_RAX_20;
  undefined8 ******extraout_RAX_21;
  undefined8 ******in_RCX;
  char *pcVar25;
  code *UNRECOVERED_JUMPTABLE_00;
  ulonglong uVar26;
  undefined8 ******ppppppuVar27;
  undefined8 *****pppppuVar28;
  undefined8 ******ppppppuVar29;
  int iVar30;
  undefined8 ******in_RDX;
  undefined8 ******ppppppuVar31;
  uint uVar32;
  undefined7 uVar34;
  longlong lVar33;
  undefined1 *puVar35;
  undefined1 *puVar36;
  undefined1 *puVar37;
  undefined1 *puVar38;
  undefined1 *puVar39;
  undefined1 *puVar40;
  undefined1 **ppuVar41;
  undefined1 *puVar42;
  undefined1 *puVar43;
  undefined1 *puVar44;
  undefined1 *puVar45;
  undefined8 *******pppppppuVar46;
  undefined8 ******in_R8;
  undefined1 (*pauVar47) [16];
  undefined1 (*pauVar48) [16];
  uint uVar49;
  undefined8 *****in_R9;
  undefined8 ****in_R10;
  undefined1 (*in_R11) [16];
  uint uVar50;
  uint uVar51;
  uint unaff_R13D;
  undefined8 ****ppppuVar52;
  undefined8 *******pppppppuVar53;
  undefined8 ******unaff_R15;
  undefined1 (*pauVar54) [16];
  bool bVar55;
  bool bVar56;
  code *in_MM0;
  undefined1 auVar57 [16];
  float fVar58;
  float fVar59;
  float fVar60;
  undefined1 auVar61 [16];
  undefined1 auVar62 [16];
  float unaff_XMM6_Da;
  undefined4 unaff_XMM6_Db;
  undefined4 unaff_XMM6_Dc;
  undefined4 unaff_XMM6_Dd;
  float unaff_XMM7_Da;
  undefined4 uVar63;
  undefined4 uVar64;
  undefined4 uVar65;
  undefined4 uVar66;
  undefined4 unaff_XMM8_Da;
  undefined4 unaff_XMM8_Db;
  undefined4 unaff_XMM8_Dc;
  undefined4 unaff_XMM8_Dd;
  float unaff_XMM9_Da;
  undefined4 uVar67;
  undefined4 uVar68;
  undefined4 uVar69;
  undefined4 uVar70;
  undefined4 unaff_XMM10_Da;
  undefined4 unaff_XMM10_Db;
  undefined4 unaff_XMM10_Dc;
  undefined4 unaff_XMM10_Dd;
  undefined4 unaff_XMM11_Da;
  undefined4 unaff_XMM12_Da;
  undefined4 unaff_XMM12_Db;
  undefined4 unaff_XMM12_Dc;
  undefined4 unaff_XMM12_Dd;
  undefined4 unaff_XMM13_Da;
  undefined8 ******unaff_XMM14_Qa;
  undefined8 ******unaff_XMM14_Qb;
  float unaff_XMM15_Da;
  undefined4 uVar71;
  undefined1 *puStack_4b0;
  undefined1 auStack_4a8 [32];
  undefined *puStack_488;
  undefined8 uStack_480;
  undefined8 *******local_428;
  undefined1 auStack_420 [80];
  undefined4 local_3d0;
  undefined4 uStack_3cc;
  undefined1 (*local_3c8) [16];
  undefined8 local_3c0;
  undefined8 uStack_3b8;
  undefined1 (*local_330) [16];
  char local_2f0 [8];
  undefined1 (*local_2e8) [16];
  undefined8 local_2e0;
  undefined8 uStack_2d8;
  uint *local_2c8;
  code *pcStack_2c0;
  undefined1 (*local_2b8) [16];
  code *pcStack_2b0;
  undefined8 local_2a8;
  undefined8 uStack_2a0;
  undefined1 (*local_1d8) [16];
  undefined8 local_1b8;
  code *pcStack_1b0;
  undefined1 (*local_1a8) [16];
  code *pcStack_1a0;
  undefined8 local_198;
  undefined8 uStack_190;
  uint auStack_128 [11];
  undefined1 local_fb;
  undefined1 local_fa;
  undefined8 local_f0;
  
  puVar37 = auStack_4a8;
  puVar40 = auStack_4a8;
  puVar35 = auStack_4a8;
  puVar38 = auStack_4a8;
  ppuVar41 = (undefined1 **)auStack_4a8;
  puVar42 = auStack_4a8;
  puVar43 = auStack_4a8;
  puVar45 = auStack_4a8;
  puVar39 = auStack_4a8;
  puVar44 = auStack_4a8;
  puVar36 = auStack_4a8;
  pppppppuVar46 = &local_428;
  local_f0 = 0xfffffffffffffffe;
  puStack_4b0 = (undefined1 *)0x141470d8a;
  pppppuVar28 = in_R9;
  std::fs::read_to_string::inner();
  pauVar21 = local_3c8;
  if (SBORROW8(0,CONCAT44(uStack_3cc,local_3d0))) {
    local_1d8 = local_3c8;
    local_fa = 1;
    puStack_4b0 = (undefined1 *)0x141470db8;
    std::backtrace::Backtrace::capture();
    local_198 = local_2a8;
    uStack_190 = uStack_2a0;
    local_1a8 = local_2b8;
    pcStack_1a0 = pcStack_2b0;
    local_1b8 = local_2c8;
    pcStack_1b0 = pcStack_2c0;
    local_2c8 = (uint *)&DAT_142c64473;
    pcStack_2c0 = (code *)0x11;
    local_2b8 = local_3c8;
    local_fa = 0;
    puStack_4b0 = (undefined1 *)0x141470e1c;
    anyhow::Error::construct<anyhow::error::ContextError<ref_<str_>,std::io::error::Error>_>();
    *in_RCX = extraout_RAX;
    *(undefined1 *)((longlong)in_RCX + 0xf) = 2;
    return;
  }
  local_330 = local_3c8;
  puStack_4b0 = (undefined1 *)0x141470e46;
  serde_json::de::from_str<enum2_<serde_json::value::Value>_>();
  if ((char)local_3d0 == '\x06') {
    local_1d8 = local_3c8;
    local_fb = 1;
    puStack_4b0 = (undefined1 *)0x141470e70;
    std::backtrace::Backtrace::capture();
    local_198 = local_2a8;
    uStack_190 = uStack_2a0;
    local_1a8 = local_2b8;
    pcStack_1a0 = pcStack_2b0;
    local_1b8 = local_2c8;
    pcStack_1b0 = pcStack_2c0;
    local_2c8 = (uint *)&DAT_142c64484;
    pcStack_2c0 = (code *)0xc;
    local_2b8 = local_3c8;
    local_fb = 0;
    puStack_4b0 = (undefined *)0x141470ed4;
    anyhow::Error::construct<anyhow::error::ContextError<ref_<str_>,serde_json::error::Error>_>();
    *in_RCX = extraout_RAX_00;
    *(undefined1 *)((longlong)in_RCX + 0xf) = 2;
    puVar35 = auStack_4a8;
    goto LAB_14147147c;
  }
  local_2f0[4] = (undefined1)uStack_3cc;
  local_2f0._1_3_ = local_3d0._1_3_;
  local_2f0._5_3_ = (undefined3)((uint)uStack_3cc >> 8);
  local_2e0 = local_3c0;
  uStack_2d8 = uStack_3b8;
  local_2f0[0] = (char)local_3d0;
  local_2e8 = local_3c8;
  pauVar47 = (undefined1 (*) [16])local_2f0;
  puStack_4b0 = (undefined *)0x141470f28;
  serde_json::value::index::impl$1::index_into();
  pauVar22 = extraout_RAX_01;
  if (extraout_RAX_01 == (undefined1 (*) [16])0x0) {
    pauVar22 = (undefined1 (*) [16])
               __imp__ZN10serde_json5value5index87__LT_impl_u20_core__ops__index__Index_LT_I_GT__u20_for_u20_serde_json__value__Value_GT_5index4NULL17hd2864d0cd0b4dde5E
    ;
  }
  ppppppuVar31 = (undefined8 ******)&DAT_142c64d28;
  pcVar25 = &DAT_142c64d28 + *(int *)(&DAT_142c64d28 + (ulonglong)(byte)(*pauVar22)[0] * 4);
  cVar17 = (char)pauVar22;
  uVar49 = (uint)pauVar22;
  uVar34 = (undefined7)((ulonglong)in_R8 >> 8);
  puVar13 = auStack_4a8;
  puVar14 = auStack_4a8;
  puVar15 = auStack_4a8;
  ppppppuVar29 = in_RCX;
  pauVar48 = pauVar47;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch((*pauVar22)[0]) {
  case 0:
    local_2c8 = (uint *)((ulonglong)local_2c8._1_7_ << 8);
    break;
  case 1:
    local_2c8 = (uint *)CONCAT71(CONCAT61(local_2c8._2_6_,*(undefined1 *)((longlong)*pauVar22 + 1)),
                                 1);
    break;
  case 2:
    pcStack_2c0 = *(code **)((longlong)*pauVar22 + 8);
    local_2b8 = *(undefined1 (**) [16])pauVar22[1];
    local_2c8 = (uint *)CONCAT71(local_2c8._1_7_,2);
    break;
  case 3:
    puStack_4b0 = &UNK_141470fa2;
    alloc::string::impl$6::clone();
    local_2c8 = (uint *)CONCAT71(local_2c8._1_7_,3);
    break;
  case 4:
    puStack_4b0 = &UNK_141470f6d;
    alloc::vec::impl$11::clone<enum2_<serde_json::value::Value>,alloc::alloc::Global>();
    local_2c8 = (uint *)CONCAT71(local_2c8._1_7_,4);
    break;
  case 5:
    if (*(longlong *)((longlong)pauVar22[1] + 8) == 0) {
      pcStack_2c0 = (code *)0x0;
      pcStack_2b0 = (code *)0x0;
    }
    else {
      if (*(longlong *)((longlong)*pauVar22 + 8) == 0) {
                    // WARNING: Subroutine does not return
        puStack_4b0 = &UNK_14147330c;
        core::option::unwrap_failed();
      }
      puStack_4b0 = &UNK_141470fe2;
      alloc::collections::btree::map::impl$2::clone::
      clone_subtree<alloc::string::String,enum2_<serde_json::value::Value>,alloc::alloc::Global>();
    }
    local_2c8 = (uint *)CONCAT71(local_2c8._1_7_,5);
    break;
  case 6:
    pbVar20 = (byte *)(ulonglong)(uVar49 + 0x21bf5d7);
    goto code_r0x000141471181;
  case 7:
    puStack_4b0 = &UNK_1414711ec;
    alloc::string::impl$6::clone();
    local_2c8 = (uint *)CONCAT71(local_2c8._1_7_,3);
    goto code_r0x000141471222;
  case 8:
    puStack_4b0 = &UNK_1414711ba;
    alloc::vec::impl$11::clone<enum2_<serde_json::value::Value>,alloc::alloc::Global>();
    local_2c8 = (uint *)CONCAT71(local_2c8._1_7_,4);
  case 9:
    goto code_r0x000141471222;
  case 10:
    goto code_r0x00014147118b;
  case 0xb:
    (*pauVar22)[0] = (*pauVar22)[0] + cVar17;
    pppppppuVar46 = (undefined8 *******)(ulonglong)(uint)((int)pppppppuVar46 + (int)in_R8);
    *(char *)((longlong)pppppppuVar46 + 0x161) = cVar17 + -0xf;
    *(undefined1 *)(pppppppuVar46 + 0x2c) = 1;
code_r0x000141471222:
    puStack_4b0 = &UNK_141471235;
    serde_json::value::de::impl$2::
    deserialize_i32<serde::de::impls::impl_54::deserialize::PrimitiveVisitor>();
    if (((ulonglong)pppppppuVar46[0x4e] & 1) == 0) {
      *(undefined4 *)((longlong)pppppppuVar46 + 0x11c) =
           *(undefined4 *)((longlong)pppppppuVar46 + 0x274);
    }
    else {
      pppppppuVar46[0xc] = pppppppuVar46[0x4f];
      *(undefined4 *)(pppppppuVar46 + 0xb) = 1;
      *(undefined4 *)((longlong)pppppppuVar46 + 0x11c) = 0x19;
      puStack_4b0 = &UNK_141471263;
      core::ptr::drop_in_place<serde_json::error::Error>();
    }
code_r0x000141471278:
    pauVar48 = (undefined1 (*) [16])(pppppppuVar46 + 0x27);
code_r0x000141471284:
    puStack_4b0 = &UNK_141471289;
    serde_json::value::index::impl$1::index_into();
    pauVar22 = extraout_RAX_06;
    if (extraout_RAX_06 == (undefined1 (*) [16])0x0) {
      pauVar22 = (undefined1 (*) [16])
                 __imp__ZN10serde_json5value5index87__LT_impl_u20_core__ops__index__Index_LT_I_GT__u20_for_u20_serde_json__value__Value_GT_5index4NULL17hd2864d0cd0b4dde5E
      ;
    }
    ppppppuVar31 = (undefined8 ******)&UNK_142c64d58;
    uVar26 = (ulonglong)*(int *)(&UNK_142c64d58 + (ulonglong)(byte)(*pauVar22)[0] * 4);
    bVar56 = 0xfffffffebd39b2a7 < uVar26;
    pcVar25 = &UNK_142c64d58 + uVar26;
    bVar55 = pcVar25 == (char *)0x0;
    cVar17 = (char)pauVar22;
    puVar35 = auStack_4a8;
    puVar37 = auStack_4a8;
    local_428 = pppppppuVar46;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
    switch((*pauVar22)[0]) {
    case 0:
      *(undefined1 *)(pppppppuVar46 + 0x2c) = 0;
      break;
    case 1:
      *(undefined1 *)((longlong)pppppppuVar46 + 0x161) = *(undefined1 *)((longlong)*pauVar22 + 1);
      *(undefined1 *)(pppppppuVar46 + 0x2c) = 1;
      break;
    case 2:
      ppppppuVar31 = *(undefined8 *******)pauVar22[1];
      pppppppuVar46[0x2d] = *(undefined8 *******)((longlong)*pauVar22 + 8);
      pppppppuVar46[0x2e] = ppppppuVar31;
code_r0x0001414712e4:
      *(undefined1 *)(pppppppuVar46 + 0x2c) = 2;
      break;
    case 3:
      puStack_4b0 = &UNK_141471300;
      alloc::string::impl$6::clone();
      *(undefined1 *)(pppppppuVar46 + 0x2c) = 3;
      break;
    case 4:
code_r0x0001414712bf:
      puStack_4b0 = &UNK_1414712ce;
      alloc::vec::impl$11::clone<enum2_<serde_json::value::Value>,alloc::alloc::Global>();
      *(undefined1 *)(pppppppuVar46 + 0x2c) = 4;
      break;
    case 5:
      puStack_4b0 = &UNK_14147132f;
      alloc::collections::btree::map::impl$2::
      clone<alloc::string::String,enum2_<serde_json::value::Value>,alloc::alloc::Global>();
      *(undefined1 *)(pppppppuVar46 + 0x2c) = 5;
      break;
    case 6:
    case 9:
    case 0xc:
    case 0xd:
      (&UNK_142c64cdb)[uVar26] = (&UNK_142c64cdb)[uVar26] + (char)pcVar25 + bVar56;
      *(byte *)((longlong)in_RCX + 0x42) = *(byte *)((longlong)in_RCX + 0x42) | 0x4d;
code_r0x000141472264:
      in_R11 = (undefined1 (*) [16])&UNK_142c64d70;
      pauVar47 = (undefined1 (*) [16])(longlong)*(int *)(&UNK_142c64d70 + (longlong)pppppuVar28 * 4)
      ;
code_r0x00014147226f:
      UNRECOVERED_JUMPTABLE_00 = (code *)((longlong)*in_R11 + (longlong)*pauVar47);
                    // WARNING: Could not recover jumptable at 0x000141472272. Too many branches
                    // WARNING: Treating indirect jump as call
      (*UNRECOVERED_JUMPTABLE_00)(pcVar25,ppppppuVar31,UNRECOVERED_JUMPTABLE_00);
      return;
    case 7:
      goto code_r0x0001414722b3;
    case 8:
      goto code_r0x000141472278;
    case 10:
    case 0x17:
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    case 0xb:
      (&UNK_142c64cd9)[uVar26] = (&UNK_142c64cd9)[uVar26] + cVar17;
      out(3,(int)pauVar22);
      (*pauVar22)[0] = (*pauVar22)[0] + cVar17;
      puVar14 = auStack_4a8;
      goto code_r0x0001414722b7;
    case 0xe:
      pauVar22 = (undefined1 (*) [16])&DAT_00000008;
      puVar39 = auStack_4a8;
      goto code_r0x000141472278;
    case 0xf:
    case 0x12:
    case 0x15:
    case 0x16:
      ppppppuVar29 = (undefined8 ******)((longlong)in_RCX + 4);
      *(int *)in_RCX = *(int *)*pauVar21;
      *pcVar25 = *pcVar25 + cVar17 + bVar56;
      (*pauVar22)[0] = (*pauVar22)[0] + cVar17;
      puVar36 = auStack_4a8;
      goto code_r0x000141471eae;
    case 0x10:
      (*pauVar22)[0] = (*pauVar22)[0] + 'X' + bVar56;
      goto code_r0x000141472259;
    case 0x11:
      goto code_r0x0001414722b5;
    case 0x13:
      (*pauVar22)[0] = (*pauVar22)[0] + cVar17;
code_r0x0001414722d0:
      in_R8 = (undefined8 ******)CONCAT71(uVar34,(byte)in_R8 + (char)((ulonglong)pcVar25 >> 8));
      *(int *)in_RCX = *(int *)*pauVar21;
      pauVar48 = (undefined1 (*) [16])0x3;
      puVar39 = auStack_4a8;
      in_RCX = (undefined8 ******)((longlong)in_RCX + 4);
      goto code_r0x000141472278;
    case 0x14:
code_r0x00014147229f:
      (*pauVar22)[0] = (*pauVar22)[0] | (byte)ppppppuVar31;
      puVar15 = (undefined1 *)ppuVar41;
code_r0x0001414722a1:
      puVar37 = puVar15;
      goto code_r0x000141471eea;
    case 0x18:
    case 0x1b:
    case 0x1e:
    case 0x1f:
      *(int *)in_RCX = *(int *)in_RCX + -1;
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    case 0x19:
      piVar2 = (int *)((longlong)pauVar22[-8] + 3);
      *piVar2 = *piVar2 + -1;
      (*pauVar22)[0] = (*pauVar22)[0] << 1 | 1;
      puVar35 = auStack_4a8;
      goto code_r0x000141472714;
    case 0x1a:
      if (pcVar25 == (char *)0x0) goto code_r0x000141472758;
      pauVar22 = (undefined1 (*) [16])0xfffffffe;
      goto code_r0x00014147271d;
    case 0x1c:
      pauVar22 = (undefined1 (*) [16])(ulonglong)(uint)((int)pauVar22 - (int)pcVar25);
      puVar44 = auStack_4a8;
      goto code_r0x00014147271d;
    case 0x1d:
      goto code_r0x000141472714;
    case 0x20:
      goto code_r0x00014147272c;
    default:
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    }
    puStack_488 = &anon_0555c73667d6be1dc866b3f0ae0b3f3f_107_llvm_14020587298740829885;
    uStack_480 = 9;
    puStack_4b0 = &UNK_141471368;
    serde_json::value::de::impl$2::
    deserialize_struct<country_core::systems::camera_rig::_::impl_0::deserialize::__Visitor>();
    if (*(int *)(pppppppuVar46 + 0xb) == 1) {
      ppppppuVar31 = pppppppuVar46[0xc];
      pppppppuVar46[0x4a] = ppppppuVar31;
      *(undefined1 *)((longlong)pppppppuVar46 + 0x32c) = 1;
      puStack_4b0 = &UNK_141471390;
      std::backtrace::Backtrace::capture();
      pppppppuVar46[0x52] = pppppppuVar46[0x30];
      pppppppuVar46[0x53] = pppppppuVar46[0x31];
      pppppppuVar46[0x50] = pppppppuVar46[0x2e];
      pppppppuVar46[0x51] = pppppppuVar46[0x2f];
      pppppppuVar46[0x4e] = pppppppuVar46[0x2c];
      pppppppuVar46[0x4f] = pppppppuVar46[0x2d];
      pppppppuVar46[0x2c] = (undefined8 ******)0x142c644b2;
      pppppppuVar46[0x2d] = (undefined8 ******)0xe;
      pppppppuVar46[0x2e] = ppppppuVar31;
      *(undefined1 *)((longlong)pppppppuVar46 + 0x32c) = 0;
      puStack_4b0 = &UNK_1414713f8;
      anyhow::Error::construct<anyhow::error::ContextError<ref_<str_>,serde_json::error::Error>_>();
      pppppuVar28 = extraout_RAX_07;
      goto code_r0x0001414713f8;
    }
    unaff_XMM11_Da = *(undefined4 *)((longlong)pppppppuVar46 + 0x5c);
    pppppppuVar46[3] = pppppppuVar46[0xc];
    pppppppuVar46[2] = pppppppuVar46[0xd];
    unaff_XMM10_Da = *(undefined4 *)(pppppppuVar46 + 0xe);
    unaff_XMM10_Db = *(undefined4 *)((longlong)pppppppuVar46 + 0x74);
    unaff_XMM10_Dc = *(undefined4 *)(pppppppuVar46 + 0xf);
    unaff_XMM10_Dd = *(undefined4 *)((longlong)pppppppuVar46 + 0x7c);
    puStack_4b0 = &UNK_141471532;
    serde_json::value::index::impl$1::index_into();
    puStack_4b0 = &UNK_14147154d;
    serde_json::value::impl$4::clone();
    puStack_488 = &anon_66a3d72518e7f93316b3f3cb8ace3c6b_100_llvm_1564075658915195154;
    uStack_480 = 3;
    puStack_4b0 = &UNK_141471582;
    serde_json::value::de::impl$2::
    deserialize_struct<country_core::resources::time_of_day::_::impl_0::deserialize::__Visitor>();
    if (*(int *)(pppppppuVar46 + 0x2c) == 0) {
      *(undefined4 *)((longlong)pppppppuVar46 + 0x2f4) =
           *(undefined4 *)((longlong)pppppppuVar46 + 0x164);
      unaff_XMM8_Da = *(undefined4 *)(pppppppuVar46 + 0x2d);
      unaff_XMM8_Db = *(undefined4 *)((longlong)pppppppuVar46 + 0x16c);
      unaff_XMM8_Dc = *(undefined4 *)(pppppppuVar46 + 0x2e);
      unaff_XMM8_Dd = *(undefined4 *)((longlong)pppppppuVar46 + 0x174);
      unaff_XMM12_Da = *(undefined4 *)(pppppppuVar46 + 0x2f);
      unaff_XMM12_Db = *(undefined4 *)((longlong)pppppppuVar46 + 0x17c);
      unaff_XMM12_Dc = *(undefined4 *)(pppppppuVar46 + 0x30);
      unaff_XMM12_Dd = *(undefined4 *)((longlong)pppppppuVar46 + 0x184);
      unaff_XMM13_Da = *(undefined4 *)(pppppppuVar46 + 0x31);
      *(undefined2 *)((longlong)pppppppuVar46 + 0x316) =
           *(undefined2 *)((longlong)pppppppuVar46 + 0x18c);
    }
    else {
      puStack_4b0 = &UNK_14147159c;
      core::ptr::drop_in_place<serde_json::error::Error>();
      puStack_4b0 = &UNK_1414715b4;
      serde_json::value::index::impl$1::index_into();
      puStack_4b0 = &UNK_1414715cf;
      serde_json::value::impl$4::clone();
      puStack_4b0 = &UNK_1414715e2;
      serde_json::value::de::impl$2::
      deserialize_f32<serde::de::impls::impl_76::deserialize::PrimitiveVisitor>();
      if (*(int *)(pppppppuVar46 + 0x4e) == 0) {
        unaff_XMM8_Db = *(undefined4 *)((longlong)pppppppuVar46 + 0x274);
        unaff_XMM8_Dc = 0;
                    // WARNING: Read-only address (ram,0x000142aba9c0) is written
        unaff_XMM8_Dd = 0x426d5134;
        unaff_XMM8_Da = 0;
        unaff_XMM12_Da = 0x3f0147ae;
        unaff_XMM12_Db = 0x3f50a3d7;
        unaff_XMM12_Dc = 0x3e19999a;
        unaff_XMM12_Dd = 0;
        *(undefined4 *)((longlong)pppppppuVar46 + 0x2f4) = 0;
        unaff_XMM13_Da = 0;
        *(undefined2 *)((longlong)pppppppuVar46 + 0x316) = 0xcf;
      }
      else {
        puStack_4b0 = &UNK_1414715fb;
        core::ptr::drop_in_place<serde_json::error::Error>();
        uVar26 = *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E;
        unaff_XMM12_Da = 0x3f0147ae;
        unaff_XMM12_Db = 0x3f50a3d7;
        unaff_XMM12_Dc = 0x3e19999a;
        unaff_XMM12_Dd = 0;
        unaff_XMM8_Da = 0;
        unaff_XMM8_Db = 0x3e99999a;
        unaff_XMM8_Dc = 0;
        unaff_XMM8_Dd = 0x426d5134;
        *(undefined4 *)((longlong)pppppppuVar46 + 0x2f4) = 0;
        unaff_XMM13_Da = 0;
        *(undefined2 *)((longlong)pppppppuVar46 + 0x316) = 0xcf;
        if (1 < uVar26) {
          pppppppuVar46[0x2c] = (undefined8 ******)&UNK_142c644f8;
          pppppppuVar46[0x2d] = (undefined8 ******)0x1;
          pppppppuVar46[0x2e] = (undefined8 ******)0x8;
          *(undefined1 (*) [16])(pppppppuVar46 + 0x2f) = (undefined1  [16])0x0;
          puStack_4b0 = &UNK_141471672;
          toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
          pppppppuVar46[0x4e] = (undefined8 ******)&DAT_142c64148;
          pppppppuVar46[0x4f] = (undefined8 ******)0x26;
          pppppppuVar46[0x50] = (undefined8 ******)&DAT_142c64148;
          pppppppuVar46[0x51] = (undefined8 ******)0x26;
          pppppppuVar46[0x52] = extraout_RAX_08;
          puStack_4b0 = &UNK_1414716bc;
          ::log::__private_api::log<tuple_<>,log::__private_api::GlobalLogger>();
        }
      }
    }
    puStack_4b0 = &UNK_14147175b;
    serde_json::value::index::impl$1::index_into();
    puStack_4b0 = &UNK_141471776;
    serde_json::value::impl$4::clone();
    puStack_488 = &anon_0555c73667d6be1dc866b3f0ae0b3f3f_76_llvm_14020587298740829885;
    uStack_480 = 1;
    puStack_4b0 = &UNK_1414717ab;
    serde_json::value::de::impl$2::
    deserialize_struct<country_core::resources::critters::sheep::control::_::impl_0::deserialize::__Visitor>
              ();
    puStack_4b0 = &UNK_1414717be;
    core::result::Result::
    inspect_err<country_core::resources::critters::sheep::control::SheepControlSnapshot,serde_json::error::Error,country_core::systems::save::load_game::parse_saved_game_data::closure_env_0>
              ();
    if (SBORROW8(0,(longlong)pppppppuVar46[0x2c])) {
      pppppppuVar46[8] = (undefined8 ******)0x8000000000000000;
      puStack_4b0 = &UNK_1414717e3;
      core::ptr::drop_in_place<serde_json::error::Error>();
    }
    else {
      pppppppuVar46[10] = pppppppuVar46[0x2e];
      pppppppuVar46[8] = pppppppuVar46[0x2c];
      pppppppuVar46[9] = pppppppuVar46[0x2d];
    }
    puStack_4b0 = &UNK_141471815;
    serde_json::value::index::impl$1::index_into();
    puStack_4b0 = &UNK_141471830;
    serde_json::value::impl$4::clone();
    puStack_488 = &anon_fd26506bd31d5e4909a9375e3935cf05_44_llvm_1874562925060143905;
    uStack_480 = 0xf;
    puStack_4b0 = &UNK_141471865;
    serde_json::value::de::impl$2::
    deserialize_struct<country_core::resources::glade_color_customization::_::impl_0::deserialize::__Visitor>
              ();
    iVar30 = *(int *)(pppppppuVar46 + 0x2c);
    if (iVar30 == 1) {
      puStack_4b0 = &UNK_141471880;
      core::ptr::drop_in_place<serde_json::error::Error>();
      if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
        pppppppuVar46[0x2c] = (undefined8 ******)&UNK_142c645a0;
        pppppppuVar46[0x2d] = (undefined8 ******)0x1;
        pppppppuVar46[0x2e] = (undefined8 ******)0x8;
        *(undefined1 (*) [16])(pppppppuVar46 + 0x2f) = (undefined1  [16])0x0;
        puStack_4b0 = &UNK_1414718d0;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        pppppppuVar46[0x4e] = (undefined8 ******)&DAT_142c64148;
        pppppppuVar46[0x4f] = (undefined8 ******)0x26;
        pppppppuVar46[0x50] = (undefined8 ******)&DAT_142c64148;
        pppppppuVar46[0x51] = (undefined8 ******)0x26;
        pppppppuVar46[0x52] = extraout_RAX_09;
        puStack_4b0 = &UNK_14147191a;
        ::log::__private_api::log<tuple_<>,log::__private_api::GlobalLogger>();
      }
      *(undefined4 *)(pppppppuVar46 + 0x24) = *(undefined4 *)((longlong)in_R9 + 0x14);
      unaff_XMM14_Qa =
           (undefined8 ******)CONCAT44(*(undefined4 *)(in_R9 + 3),*(undefined4 *)(in_R9 + 2));
      unaff_XMM14_Qb = (undefined8 ******)0x0;
      uVar63 = 0;
      uVar64 = 0;
      uVar65 = 0x186a3;
      uVar66 = 0;
      uVar67 = 0;
      uVar68 = 0;
      uVar69 = 0;
      uVar70 = 0;
      *(undefined1 *)((longlong)pppppppuVar46 + 0x32f) = 1;
      uVar71 = 0x3f800000;
      *(undefined4 *)((longlong)pppppppuVar46 + 0x124) = 0;
      *(undefined4 *)(pppppppuVar46 + 0x25) = 0;
      *(undefined4 *)((longlong)pppppppuVar46 + 300) = 0;
      *(undefined4 *)(pppppppuVar46 + 0x26) = 0;
    }
    else {
      uVar63 = *(undefined4 *)((longlong)pppppppuVar46 + 0x164);
      uVar64 = *(undefined4 *)(pppppppuVar46 + 0x2d);
      uVar65 = *(undefined4 *)((longlong)pppppppuVar46 + 0x16c);
      uVar66 = *(undefined4 *)(pppppppuVar46 + 0x2e);
      *(undefined4 *)((longlong)pppppppuVar46 + 0x124) =
           *(undefined4 *)((longlong)pppppppuVar46 + 0x174);
      *(uint *)(pppppppuVar46 + 0x25) = (uint)*(byte *)(pppppppuVar46 + 0x2f);
      *(undefined4 *)(pppppppuVar46 + 0x24) = *(undefined4 *)((longlong)pppppppuVar46 + 0x17c);
      unaff_XMM14_Qa = pppppppuVar46[0x30];
      unaff_XMM14_Qb = pppppppuVar46[0x31];
      uVar67 = *(undefined4 *)(pppppppuVar46 + 0x32);
      uVar68 = *(undefined4 *)((longlong)pppppppuVar46 + 0x194);
      uVar69 = *(undefined4 *)(pppppppuVar46 + 0x33);
      uVar70 = *(undefined4 *)((longlong)pppppppuVar46 + 0x19c);
      uVar71 = *(undefined4 *)(pppppppuVar46 + 0x34);
      *(uint *)((longlong)pppppppuVar46 + 300) = (uint)*(byte *)((longlong)pppppppuVar46 + 0x1a4);
      *(uint *)(pppppppuVar46 + 0x26) = (uint)*(byte *)((longlong)pppppppuVar46 + 0x1a5);
      *(undefined1 *)((longlong)pppppppuVar46 + 0x32f) =
           *(undefined1 *)((longlong)pppppppuVar46 + 0x1a6);
      if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
        *(int *)(pppppppuVar46 + 100) = iVar30;
        pppppppuVar46[0x4e] = (undefined8 ******)&UNK_142c64550;
        pppppppuVar46[0x4f] = (undefined8 ******)0x1;
        pppppppuVar46[0x50] = (undefined8 ******)0x8;
        *(undefined1 (*) [16])(pppppppuVar46 + 0x51) = (undefined1  [16])0x0;
        puStack_4b0 = &UNK_141471a44;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        pppppppuVar46[0xb] = (undefined8 ******)&DAT_142c64148;
        pppppppuVar46[0xc] = (undefined8 ******)0x26;
        pppppppuVar46[0xd] = (undefined8 ******)&DAT_142c64148;
        pppppppuVar46[0xe] = (undefined8 ******)0x26;
        pppppppuVar46[0xf] = extraout_RAX_10;
        puStack_4b0 = &UNK_141471a7c;
        ::log::__private_api::log<tuple_<>,log::__private_api::GlobalLogger>();
        iVar30 = *(int *)(pppppppuVar46 + 100);
      }
      if (iVar30 != 0) {
        puStack_4b0 = &UNK_141471a92;
        core::ptr::drop_in_place<serde_json::error::Error>();
      }
    }
    puStack_4b0 = &UNK_141471aaa;
    serde_json::value::index::impl$1::index_into();
    if (extraout_RAX_11 == (char *)0x0) {
code_r0x000141471aba:
      *(undefined1 *)(pppppppuVar46 + 0x4a) = 0;
    }
    else {
      cVar17 = *extraout_RAX_11;
      *extraout_RAX_11 = '\0';
      if (cVar17 == '\x06') goto code_r0x000141471aba;
      *(char *)(pppppppuVar46 + 0x4a) = cVar17;
      uVar10 = *(undefined8 *)(extraout_RAX_11 + 9);
      ppppppuVar31 = *(undefined8 *******)(extraout_RAX_11 + 0x10);
      ppppppuVar29 = *(undefined8 *******)(extraout_RAX_11 + 0x18);
      *(undefined8 *)((longlong)pppppppuVar46 + 0x251) = *(undefined8 *)(extraout_RAX_11 + 1);
      *(undefined8 *)((longlong)pppppppuVar46 + 0x259) = uVar10;
      pppppppuVar46[0x4c] = ppppppuVar31;
      pppppppuVar46[0x4d] = ppppppuVar29;
    }
    puStack_488 = &anon_66a3d72518e7f93316b3f3cb8ace3c6b_55_llvm_1564075658915195154;
    uStack_480 = 4;
    pppppppuVar53 = pppppppuVar46 + 0x4a;
    puStack_4b0 = &UNK_141471b1b;
    serde_json::value::de::impl$2::
    deserialize_struct<country_core::resources::history::_::impl_0::deserialize::__Visitor>();
    puStack_4b0 = &UNK_141471b3b;
    anyhow::context::impl$0::
    context<country_core::resources::history::History,serde_json::error::Error,ref_<str_>_>();
    ppppppuVar31 = pppppppuVar46[0x4f];
    if (!SBORROW8(0,(longlong)pppppppuVar46[0x4e])) {
      pppppppuVar46[5] = in_R8;
      pppppppuVar46[0x19] = pppppppuVar46[0x5c];
      pppppppuVar46[0x17] = pppppppuVar46[0x5a];
      pppppppuVar46[0x18] = pppppppuVar46[0x5b];
      pppppppuVar46[0x15] = pppppppuVar46[0x58];
      pppppppuVar46[0x16] = pppppppuVar46[0x59];
      pppppppuVar46[0x13] = pppppppuVar46[0x56];
      pppppppuVar46[0x14] = pppppppuVar46[0x57];
      pppppppuVar46[0x11] = pppppppuVar46[0x54];
      pppppppuVar46[0x12] = pppppppuVar46[0x55];
      *(undefined4 *)(pppppppuVar46 + 0xf) = *(undefined4 *)(pppppppuVar46 + 0x52);
      *(undefined4 *)((longlong)pppppppuVar46 + 0x7c) =
           *(undefined4 *)((longlong)pppppppuVar46 + 0x294);
      *(undefined4 *)(pppppppuVar46 + 0x10) = *(undefined4 *)(pppppppuVar46 + 0x53);
      *(undefined4 *)((longlong)pppppppuVar46 + 0x84) =
           *(undefined4 *)((longlong)pppppppuVar46 + 0x29c);
      *(undefined1 (*) [16])(pppppppuVar46 + 0xd) = *(undefined1 (*) [16])(pppppppuVar46 + 0x50);
      pppppppuVar46[0xb] = pppppppuVar46[0x4e];
      pppppppuVar46[0xc] = ppppppuVar31;
      *(undefined4 *)((longlong)pppppppuVar46 + 0xd4) = *(undefined4 *)(pppppppuVar46 + 0x18);
      *(undefined1 *)((longlong)pppppppuVar46 + 0x336) = 0;
      *(undefined4 *)(pppppppuVar46 + 0x5e) = 0;
      uVar19 = 0;
      bVar56 = false;
      if (unaff_R13D == 0x1b) {
code_r0x000141471c23:
        *(undefined4 *)((longlong)pppppppuVar46 + 0x304) = uVar19;
        if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
          pppppppuVar46[0x2c] = (undefined8 ******)&UNK_142c64738;
          pppppppuVar46[0x2d] = (undefined8 ******)0x1;
          pppppppuVar46[0x2e] = (undefined8 ******)0x8;
          pppppppuVar46[0x2f] = (undefined8 ******)0x0;
          pppppppuVar46[0x30] = (undefined8 ******)0x0;
          *(undefined **)(puVar36 + -8) = &UNK_141471c77;
          toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
          pppppppuVar46[0x4e] = (undefined8 ******)&DAT_142c64148;
          pppppppuVar46[0x4f] = (undefined8 ******)0x26;
          pppppppuVar46[0x50] = (undefined8 ******)&DAT_142c64148;
          pppppppuVar46[0x51] = (undefined8 ******)0x26;
          pppppppuVar46[0x52] = extraout_RAX_12;
          *(undefined **)(puVar36 + -8) = &UNK_141471cc1;
          ::log::__private_api::log<tuple_<>,log::__private_api::GlobalLogger>();
        }
      }
      else {
        if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
          pppppppuVar46[0x4a] = pppppppuVar46 + 0x60;
          pppppppuVar46[0x4b] = (undefined8 ******)core::fmt::num::imp::impl$7::fmt;
          pppppppuVar46[0x4c] = (undefined8 ******)&UNK_142c645e0;
          pppppppuVar46[0x4d] = (undefined8 ******)core::fmt::num::imp::impl$7::fmt;
          pppppppuVar46[0x2c] = (undefined8 ******)&UNK_142c64648;
          pppppppuVar46[0x2d] = (undefined8 ******)0x3;
          pppppppuVar46[0x30] = (undefined8 ******)0x0;
          pppppppuVar46[0x2e] = pppppppuVar53;
          pppppppuVar46[0x2f] = (undefined8 ******)0x2;
          puStack_4b0 = &UNK_141471dc3;
          toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
          pppppppuVar46[0x4e] = (undefined8 ******)&DAT_142c64148;
          pppppppuVar46[0x4f] = (undefined8 ******)0x26;
          pppppppuVar46[0x50] = (undefined8 ******)&DAT_142c64148;
          pppppppuVar46[0x51] = (undefined8 ******)0x26;
          pppppppuVar46[0x52] = extraout_RAX_15;
          puStack_4b0 = &UNK_141471e0d;
          ::log::__private_api::log<tuple_<>,log::__private_api::GlobalLogger>();
          ppppppuVar31 = pppppppuVar46[0xc];
        }
        *(undefined4 *)((longlong)pppppppuVar46 + 0x134) = uVar71;
        *(undefined4 *)(pppppppuVar46 + -2) = uVar67;
        *(undefined4 *)((longlong)pppppppuVar46 + -0xc) = uVar68;
        *(undefined4 *)(pppppppuVar46 + -1) = uVar69;
        *(undefined4 *)((longlong)pppppppuVar46 + -4) = uVar70;
        *(undefined4 *)(pppppppuVar46 + -4) = uVar63;
        *(undefined4 *)((longlong)pppppppuVar46 + -0x1c) = uVar64;
        *(undefined4 *)(pppppppuVar46 + -3) = uVar65;
        *(undefined4 *)((longlong)pppppppuVar46 + -0x14) = uVar66;
        pppppppuVar46[7] = in_RCX;
        *(bool *)((longlong)pppppppuVar46 + 0x336) = unaff_R13D < 0x10;
        *(undefined1 *)(pppppppuVar46 + 0x1b) = 1;
        pppppppuVar46[0x16] = (undefined8 ******)0x0;
        pppppppuVar46[4] = ppppppuVar31 + (longlong)pppppppuVar46[0xd] * 5;
        in_R10 = (undefined8 ****)0xffffffffffffffff;
        in_R9 = (undefined8 *****)0x800000000000000e;
        in_RCX = (undefined8 ******)0x2;
        unaff_XMM15_Da = -0.01;
        unaff_XMM6_Da = 1.0;
        unaff_XMM6_Db = 0;
        unaff_XMM6_Dc = 0;
        unaff_XMM6_Dd = 0;
code_r0x000141471e83:
        unaff_XMM7_Da = 0.01;
        unaff_XMM9_Da = INFINITY;
        *(undefined4 *)((longlong)pppppppuVar46 + 0x304) = 0;
        *(undefined4 *)(pppppppuVar46 + 0x5e) = 0;
        *(uint *)(pppppppuVar46 + 0x66) = unaff_R13D;
        pppppppuVar46[0x22] = in_RDX;
        puVar36 = auStack_4a8;
        ppppppuVar29 = in_RCX;
code_r0x000141471eae:
        if (ppppppuVar31 != pppppppuVar46[4]) {
          pppppppuVar46[0x5f] = ppppppuVar31;
          puVar35 = puVar36;
          if (ppppppuVar31[2] != (undefined8 *****)0x0) {
            in_R8 = (undefined8 ******)pppppppuVar46[0x5f][1];
            unaff_R15 = in_R8 + (longlong)ppppppuVar31[2] * 3;
            pppppppuVar46[1] = unaff_R15;
            puVar37 = puVar36;
            do {
              pppppuVar28 = *in_R8;
              if (pppppuVar28 == (undefined8 *****)0x7) {
                pppppuVar28 = in_R8[2];
                if (pppppuVar28 != (undefined8 *****)0x0) {
                  *pppppppuVar46 = in_R8;
                  pppppuVar24 = in_R8[1];
                  pppppuVar28 = pppppuVar24 + (longlong)pppppuVar28 * 2;
                  do {
                    ppppuVar52 = pppppuVar24[1];
                    if (ppppuVar52 != (undefined8 ****)0x0) {
                      ppppuVar5 = *pppppuVar24;
                      lVar33 = 0;
                      do {
                        plVar6 = *(longlong **)((longlong)ppppuVar5 + lVar33);
                        LOCK();
                        bVar56 = plVar6[1] == 1;
                        if (bVar56) {
                          plVar6[1] = (longlong)in_R10;
                        }
                        UNLOCK();
                        if ((!bVar56) || (plVar6[1] = 1, *plVar6 != 1)) {
                          *(undefined1 *)((longlong)pppppppuVar46 + 0x337) = 1;
                    // WARNING: Subroutine does not return
                          *(undefined **)(puVar37 + -8) = &UNK_1414732cb;
                          core::option::unwrap_failed();
                        }
                        ppppppuVar31 = *(undefined8 *******)((longlong)ppppuVar5 + lVar33);
                        if (ppppppuVar31[2] == in_R9) {
                          pppppppuVar46[0x20] = (undefined8 ******)ppppppuVar31[3];
                          uVar71 = *(undefined4 *)(ppppppuVar31 + 4);
                          pppppppuVar46[99] = (undefined8 ******)((longlong)ppppppuVar31 + 0x24);
                          bVar56 = *(char *)((longlong)ppppppuVar31 + 0x3c) != '\0';
                          uVar63 = uVar71;
                          pppppppuVar53 = pppppppuVar46 + 0x2c;
                          if (bVar56) {
                            uVar63 = (int)ppppppuVar29;
                            pppppppuVar53 = pppppppuVar46 + 0x4e;
                          }
                          *(undefined4 *)(pppppppuVar46 + 0x2b) = uVar63;
                          if (!bVar56) {
                            uVar71 = (int)ppppppuVar29;
                          }
                          *(undefined4 *)(pppppppuVar46 + 100) = uVar71;
                          pppppppuVar53[2] = *(undefined8 *******)((longlong)ppppppuVar31 + 0x34);
                          ppppppuVar23 = *(undefined8 *******)((longlong)ppppppuVar31 + 0x2c);
                          *pppppppuVar53 = *(undefined8 *******)((longlong)ppppppuVar31 + 0x24);
                          pppppppuVar53[1] = ppppppuVar23;
                          pppppppuVar46[0x61] = ppppppuVar31;
                          *(undefined **)(puVar37 + -8) = &UNK_141472012;
                          core::ptr::
                          drop_in_place<enum2_<country_core::resources::history::wall_compound_edit::WallHistoryEdit>_>
                                    ();
                          ppppppuVar31 = pppppppuVar46[0x61];
                          ppppppuVar31[2] = (undefined8 *****)0x8000000000000003;
                          ppppppuVar31[3] = pppppppuVar46[0x20];
                          *(undefined4 *)(ppppppuVar31 + 4) = *(undefined4 *)(pppppppuVar46 + 0x2b);
                          ppppppuVar23 = pppppppuVar46[99];
                          ppppppuVar23[2] = pppppppuVar46[0x2e];
                          ppppppuVar27 = pppppppuVar46[0x2d];
                          *ppppppuVar23 = pppppppuVar46[0x2c];
                          ppppppuVar23[1] = ppppppuVar27;
                          *(undefined4 *)((longlong)ppppppuVar31 + 0x3c) =
                               *(undefined4 *)(pppppppuVar46 + 100);
                          *(undefined1 (*) [16])(ppppppuVar31 + 8) =
                               *(undefined1 (*) [16])(pppppppuVar46 + 0x4e);
                          ppppppuVar31[10] = pppppppuVar46[0x50];
                          bVar18 = 1;
                          in_R10 = (undefined8 ****)0xffffffffffffffff;
                        }
                        else {
                          bVar18 = 0;
                          if ((-0x7ffffffffffffff1 < (longlong)ppppppuVar31[2]) &&
                             (*(uint *)(ppppppuVar31 + 0x1f) < 2)) {
                            fVar58 = *(float *)((longlong)ppppppuVar31 + 0xfc);
                            if ((fVar58 != *(float *)(ppppppuVar31 + 0x20)) ||
                               (NAN(fVar58) || NAN(*(float *)(ppppppuVar31 + 0x20)))) {
                              bVar18 = 0;
                            }
                            else {
                              fVar58 = fVar58 + unaff_XMM15_Da;
                              if (NAN(fVar58)) {
                                *(undefined1 *)((longlong)pppppppuVar46 + 0x337) = 1;
                    // WARNING: Subroutine does not return
                                *(undefined **)(puVar37 + -8) = &UNK_141472f11;
                                core::panicking::panic();
                              }
                              if (ABS(fVar58) == INFINITY) {
                                *(undefined1 *)((longlong)pppppppuVar46 + 0x337) = 1;
                    // WARNING: Subroutine does not return
                                *(undefined **)(puVar37 + -8) = &UNK_141472f35;
                                core::panicking::panic();
                              }
                              fVar59 = 0.0;
                              if (0.0 <= fVar58) {
                                fVar59 = fVar58;
                              }
                              auVar57._4_4_ = unaff_XMM6_Db;
                              auVar57._0_4_ = unaff_XMM6_Da;
                              auVar57._8_4_ = unaff_XMM6_Dc;
                              auVar57._12_4_ = unaff_XMM6_Dd;
                              if (fVar59 <= unaff_XMM6_Da) {
                                auVar57._0_4_ = fVar59;
                              }
                              *(float *)((longlong)ppppppuVar31 + 0xfc) = auVar57._0_4_;
                              fVar58 = auVar57._0_4_ + unaff_XMM7_Da;
                              if (unaff_XMM9_Da <= fVar58) {
                                *(undefined1 *)((longlong)pppppppuVar46 + 0x337) = 1;
                    // WARNING: Subroutine does not return
                                *(undefined **)(puVar37 + -8) = &UNK_141472f59;
                                core::panicking::panic();
                              }
                              fVar59 = unaff_XMM6_Da;
                              if (fVar58 <= unaff_XMM6_Da) {
                                fVar59 = fVar58;
                              }
                              *(float *)(ppppppuVar31 + 0x20) = fVar59;
                              bVar18 = 1;
                            }
                          }
                        }
                        *(byte *)((longlong)pppppppuVar46 + 0x336) =
                             *(byte *)((longlong)pppppppuVar46 + 0x336) | bVar18;
                        lVar33 = lVar33 + 8;
                      } while ((longlong)ppppuVar52 << 3 != lVar33);
                    }
                    pppppuVar24 = pppppuVar24 + 2;
                  } while (pppppuVar24 != pppppuVar28);
                  in_R8 = *pppppppuVar46;
                  pppppuVar28 = *in_R8;
                  unaff_R13D = *(uint *)(pppppppuVar46 + 0x66);
                  unaff_R15 = pppppppuVar46[1];
                  goto code_r0x000141472147;
                }
              }
              else {
code_r0x000141472147:
                if (pppppuVar28 == (undefined8 *****)0x0) {
                  pppppuVar28 = in_R8[1];
                  LOCK();
                  bVar56 = pppppuVar28[1] == (undefined8 ****)0x1;
                  if (bVar56) {
                    pppppuVar28[1] = in_R10;
                  }
                  UNLOCK();
                  if ((!bVar56) ||
                     (pppppuVar28[1] = (undefined8 ****)0x1, *pppppuVar28 != (undefined8 ****)0x1))
                  {
                    *(undefined1 *)((longlong)pppppppuVar46 + 0x337) = 1;
                    // WARNING: Subroutine does not return
                    *(undefined **)(puVar37 + -8) = &UNK_1414732e3;
                    core::option::unwrap_failed();
                  }
                  pppppuVar28 = in_R8[1];
                  if ((SBORROW8(0,(longlong)pppppuVar28[2])) &&
                     (pppppuVar28[5] == (undefined8 ****)0x2)) {
                    ppppuVar52 = pppppuVar28[4];
                    fVar58 = *(float *)(ppppuVar52 + 1) - *(float *)((longlong)ppppuVar52 + 0x14);
                    fVar59 = SUB84(*ppppuVar52,0) - *(float *)((longlong)ppppuVar52 + 0xc);
                    fVar60 = (float)((ulonglong)*ppppuVar52 >> 0x20) - *(float *)(ppppuVar52 + 2);
                    fVar58 = SQRT(fVar58 * fVar58 + fVar60 * fVar60 + fVar59 * fVar59);
                    if ((fVar58 == 0.0) && (!NAN(fVar58))) {
                      *(float *)((longlong)ppppuVar52 + 0xc) =
                           *(float *)((longlong)ppppuVar52 + 0xc) + 0.001;
                      *(float *)(ppppuVar52 + 2) = *(float *)(ppppuVar52 + 2) + 0.0;
                      *(float *)((longlong)ppppuVar52 + 0x14) =
                           *(float *)((longlong)ppppuVar52 + 0x14) + 0.0;
                      *(undefined1 *)((longlong)pppppppuVar46 + 0x336) = 1;
                    }
                  }
                }
              }
              if ((unaff_R13D < 0x14) && (*(int *)in_R8 == 9)) {
                pppppuVar28 = in_R8[1];
                LOCK();
                bVar56 = pppppuVar28[1] == (undefined8 ****)0x1;
                if (bVar56) {
                  pppppuVar28[1] = in_R10;
                }
                UNLOCK();
                if ((!bVar56) ||
                   (pppppuVar28[1] = (undefined8 ****)0x1, *pppppuVar28 != (undefined8 ****)0x1)) {
                  *(undefined1 *)((longlong)pppppppuVar46 + 0x337) = 1;
                    // WARNING: Subroutine does not return
                  *(undefined **)(puVar37 + -8) = &UNK_1414732fb;
                  core::option::unwrap_failed();
                }
code_r0x00014147222d:
                pauVar22 = (undefined1 (*) [16])in_R8[1];
                pcVar25 = (char *)(ulonglong)*(byte *)((longlong)pauVar22[1] + 8);
                in_RCX = ppppppuVar29;
                if (*(byte *)((longlong)pauVar22[1] + 8) == 4) {
                  pcVar25 = &DAT_0000001c;
                  ppppppuVar31 = (undefined8 ******)0x18;
                  puVar38 = puVar37;
                }
                else {
code_r0x00014147223a:
                  ppppppuVar29 = in_RCX;
                  if ((int)pcVar25 != 7) goto code_r0x000141471eea;
                  pcVar25 = &DAT_00000014;
                  ppppppuVar31 = (undefined8 ******)0x10;
                  puVar38 = puVar37;
                }
code_r0x000141472259:
                uVar49 = *(uint *)((longlong)pauVar22[1] + (longlong)ppppppuVar31);
                pppppuVar28 = (undefined8 *****)(ulonglong)uVar49;
                if (pppppuVar28 < (undefined8 *****)0x9) goto code_r0x000141472264;
                pauVar48 = (undefined1 (*) [16])0x1;
                bVar55 = uVar49 == 999;
                puVar42 = puVar38;
code_r0x0001414722b3:
                puVar43 = puVar42;
                if (bVar55) {
                  pauVar48 = (undefined1 (*) [16])((ulonglong)pppppuVar28 & 0xffffffff);
                  puVar39 = puVar42;
                }
                else {
code_r0x0001414722b5:
                  puVar39 = puVar43;
                }
code_r0x000141472278:
                *(int *)((longlong)pauVar22[1] + (longlong)ppppppuVar31) = (int)pauVar48;
                pauVar48 = (undefined1 (*) [16])
                           (ulonglong)*(uint *)(pcVar25 + (longlong)pauVar22[1]);
                bVar56 = pauVar48 < (undefined1 (*) [16])&DAT_00000008;
                bVar55 = pauVar48 == (undefined1 (*) [16])&DAT_00000008;
                puVar40 = puVar39;
code_r0x000141472286:
                puVar14 = puVar40;
                pauVar47 = pauVar48;
                if (bVar56 || bVar55) goto code_r0x000141472288;
code_r0x0001414722b7:
                puVar37 = puVar14;
                ppppppuVar29 = in_RCX;
                if ((int)pauVar48 == 999) {
                  pcVar25 = pcVar25 + (longlong)pauVar22[1];
                  pcVar25[0] = -0x19;
                  pcVar25[1] = '\x03';
                  pcVar25[2] = '\0';
                  pcVar25[3] = '\0';
                }
                else {
                  pcVar25 = pcVar25 + (longlong)pauVar22[1];
                  pcVar25[0] = '\x01';
                  pcVar25[1] = '\0';
                  pcVar25[2] = '\0';
                  pcVar25[3] = '\0';
                }
              }
code_r0x000141471eea:
              in_R8 = in_R8 + 3;
              puVar35 = puVar37;
            } while (in_R8 != unaff_R15);
          }
          if (unaff_R13D < 0x10) {
            ppppppuVar31 = pppppppuVar46[0x5f];
            LOCK();
            bVar56 = *ppppppuVar31[3] == (undefined8 ****)0x1;
            if (bVar56) {
              *ppppppuVar31[3] = (undefined8 ****)0x0;
            }
            UNLOCK();
            ppppppuVar23 = (undefined8 ******)ppppppuVar31[3];
            if (bVar56) {
              if (ppppppuVar23[1] == (undefined8 *****)0x1) {
                *ppppppuVar23 = (undefined8 *****)0x1;
              }
              else {
                pppppppuVar46[99] = ppppppuVar23 + 1;
                pppppppuVar46[100] = ppppppuVar23;
                *(undefined **)(puVar35 + -8) = &UNK_1414727df;
                alloc::sync::UniqueArcUninit::
                new<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>
                          ();
                ppppppuVar27 = pppppppuVar46[0x2c];
                ppppppuVar7 = pppppppuVar46[0x2e];
                lVar33 = (-(longlong)ppppppuVar27 & (longlong)ppppppuVar27 + 0xfU) - 0x10;
                if (((ulonglong)ppppppuVar27 ^ (longlong)ppppppuVar27 - 1U) <=
                    (longlong)ppppppuVar27 - 1U) {
                  lVar33 = -1;
                }
                pppppuVar28 = ppppppuVar23[2];
                pppppuVar24 = ppppppuVar23[3];
                uVar71 = *(undefined4 *)(ppppppuVar23 + 4);
                uVar63 = *(undefined4 *)((longlong)ppppppuVar23 + 0x24);
                uVar64 = *(undefined4 *)(ppppppuVar23 + 5);
                uVar65 = *(undefined4 *)((longlong)ppppppuVar23 + 0x2c);
                pppppuVar11 = ppppppuVar23[7];
                puVar3 = (undefined8 *)((longlong)ppppppuVar7 + lVar33 + 0x30);
                *puVar3 = ppppppuVar23[6];
                puVar3[1] = pppppuVar11;
                puVar4 = (undefined4 *)((longlong)ppppppuVar7 + lVar33 + 0x20);
                *puVar4 = uVar71;
                puVar4[1] = uVar63;
                puVar4[2] = uVar64;
                puVar4[3] = uVar65;
                puVar3 = (undefined8 *)((longlong)ppppppuVar7 + lVar33 + 0x10);
                *puVar3 = pppppuVar28;
                puVar3[1] = pppppuVar24;
                if (((ulonglong)pppppppuVar46[0x2f] & 1) == 0) {
                    // WARNING: Subroutine does not return
                  *(undefined **)(puVar35 + -8) = &UNK_141473359;
                  core::option::unwrap_failed();
                }
                ppppppuVar31[3] = ppppppuVar7;
                if (pppppppuVar46[100] != (undefined8 ******)0xffffffffffffffff) {
                  ppppppuVar31 = pppppppuVar46[99];
                  LOCK();
                  *ppppppuVar31 = (undefined8 *****)((longlong)*ppppppuVar31 + -1);
                  UNLOCK();
                  if (*ppppppuVar31 == (undefined8 *****)0x0) {
                    *(undefined **)(puVar35 + -8) = &UNK_141472872;
                    std::alloc::__default_lib_allocator::__rust_dealloc();
                  }
                }
              }
            }
            else {
              pppppppuVar46[99] = ppppppuVar31 + 3;
              *(undefined1 *)((longlong)pppppppuVar46 + 0x337) = 1;
              pppppppuVar46[0x61] = ppppppuVar23 + 2;
              *(undefined **)(puVar35 + -8) = &UNK_14147239b;
              alloc::sync::UniqueArcUninit::
              new<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>
                        ();
              *(undefined4 *)(pppppppuVar46 + 0x20) = unaff_XMM8_Da;
              *(undefined4 *)((longlong)pppppppuVar46 + 0x104) = unaff_XMM8_Db;
              *(undefined4 *)(pppppppuVar46 + 0x21) = unaff_XMM8_Dc;
              *(undefined4 *)((longlong)pppppppuVar46 + 0x10c) = unaff_XMM8_Dd;
              ppppppuVar31 = pppppppuVar46[0x4e];
              ppppppuVar27 = pppppppuVar46[0x50];
              pppppppuVar46[100] = ppppppuVar23;
              ppppppuVar7 = (undefined8 ******)ppppppuVar23[6];
              ppppppuVar23 = (undefined8 ******)ppppppuVar23[7];
              *(undefined1 *)((longlong)pppppppuVar46 + 0x335) = 1;
              *(undefined **)(puVar35 + -8) = &UNK_1414723ec;
              hashbrown::raw::impl$13::
              clone<tuple_<enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>
                        ();
              lVar33 = (-(longlong)ppppppuVar31 & (longlong)ppppppuVar31 + 0xfU) - 0x10;
              if (((ulonglong)ppppppuVar31 ^ (longlong)ppppppuVar31 - 1U) <=
                  (longlong)ppppppuVar31 - 1U) {
                lVar33 = -1;
              }
              pppppppuVar46[0x30] = ppppppuVar7;
              pppppppuVar46[0x31] = ppppppuVar23;
              ppppppuVar31 = pppppppuVar46[0x31];
              puVar3 = (undefined8 *)((longlong)ppppppuVar27 + lVar33 + 0x30);
              *puVar3 = pppppppuVar46[0x30];
              puVar3[1] = ppppppuVar31;
              ppppppuVar31 = pppppppuVar46[0x2c];
              ppppppuVar23 = pppppppuVar46[0x2d];
              uVar71 = *(undefined4 *)((longlong)pppppppuVar46 + 0x174);
              uVar63 = *(undefined4 *)(pppppppuVar46 + 0x2f);
              uVar64 = *(undefined4 *)((longlong)pppppppuVar46 + 0x17c);
              puVar4 = (undefined4 *)((longlong)ppppppuVar27 + lVar33 + 0x20);
              *puVar4 = *(undefined4 *)(pppppppuVar46 + 0x2e);
              puVar4[1] = uVar71;
              puVar4[2] = uVar63;
              puVar4[3] = uVar64;
              pppppppuVar46[0x61] = ppppppuVar27;
              puVar3 = (undefined8 *)((longlong)ppppppuVar27 + lVar33 + 0x10);
              *puVar3 = ppppppuVar31;
              puVar3[1] = ppppppuVar23;
              if (((ulonglong)pppppppuVar46[0x51] & 1) == 0) {
                *(undefined1 *)((longlong)pppppppuVar46 + 0x335) = 0;
                    // WARNING: Subroutine does not return
                *(undefined **)(puVar35 + -8) = &UNK_14147334b;
                core::option::unwrap_failed();
              }
              ppppppuVar31 = pppppppuVar46[100];
              LOCK();
              *ppppppuVar31 = (undefined8 *****)((longlong)*ppppppuVar31 + -1);
              UNLOCK();
              if (*ppppppuVar31 == (undefined8 *****)0x0) {
                *(undefined **)(puVar35 + -8) = &UNK_14147246f;
                alloc::sync::Arc::
                drop_slow<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>
                          ();
              }
              *pppppppuVar46[99] = pppppppuVar46[0x61];
              unaff_XMM8_Da = *(undefined4 *)(pppppppuVar46 + 0x20);
              unaff_XMM8_Db = *(undefined4 *)((longlong)pppppppuVar46 + 0x104);
              unaff_XMM8_Dc = *(undefined4 *)(pppppppuVar46 + 0x21);
              unaff_XMM8_Dd = *(undefined4 *)((longlong)pppppppuVar46 + 0x10c);
            }
            *(undefined1 *)((longlong)pppppppuVar46 + 0x337) = 1;
            *(undefined **)(puVar35 + -8) = &UNK_1414724ac;
            hashbrown::raw::RawTable::
            clear<tuple_<enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>
                      ();
          }
          *(undefined1 *)((longlong)pppppppuVar46 + 0x337) = 1;
          *(undefined **)(puVar35 + -8) = &UNK_1414724cd;
          alloc::vec::Vec::
          retain<enum2_<country_core::resources::history::EditType>,alloc::alloc::Global,country_core::systems::save::load_game::parse_saved_game_data::closure_env_2>
                    ();
          ppppppuVar23 = pppppppuVar46[0x5f];
          ppppppuVar31 = ppppppuVar23 + 5;
          if (unaff_R13D < 0x14) goto code_r0x0001414724e2;
          goto code_r0x00014147287e;
        }
        bVar56 = (int)unaff_R13D < 0x13;
        if ((int)unaff_R13D < 0x13) {
          if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
            pppppppuVar46[0x4a] = (undefined8 ******)&UNK_142c64690;
            pppppppuVar46[0x4b] = (undefined8 ******)core::fmt::num::imp::impl$7::fmt;
            pppppppuVar46[0x2c] = (undefined8 ******)&UNK_142c646d0;
            pppppppuVar46[0x2d] = (undefined8 ******)0x2;
            pppppppuVar46[0x30] = (undefined8 ******)0x0;
            pppppppuVar46[0x2e] = pppppppuVar46 + 0x4a;
            pppppppuVar46[0x2f] = (undefined8 ******)0x1;
            *(undefined1 *)((longlong)pppppppuVar46 + 0x337) = 1;
            *(undefined **)(puVar36 + -8) = &UNK_141472feb;
            toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
            pppppppuVar46[0x4e] = (undefined8 ******)&DAT_142c64148;
            pppppppuVar46[0x4f] = (undefined8 ******)0x26;
            pppppppuVar46[0x50] = (undefined8 ******)&DAT_142c64148;
            pppppppuVar46[0x51] = (undefined8 ******)0x26;
            pppppppuVar46[0x52] = extraout_RAX_20;
            *(undefined1 *)((longlong)pppppppuVar46 + 0x337) = 1;
            *(undefined **)(puVar36 + -8) = &UNK_14147303c;
            ::log::__private_api::log<tuple_<>,log::__private_api::GlobalLogger>();
          }
          auVar57 = ___xmm_000000000000000041a0000041a00000;
          if ((int)unaff_R13D < 0x11) {
            pppppppuVar46[0x16] = (undefined8 ******)0x0;
            if (pppppppuVar46[0xd] != (undefined8 ******)0x0) {
              ppppppuVar29 = pppppppuVar46[0xc];
              ppppppuVar31 = ppppppuVar29 + (longlong)pppppppuVar46[0xd] * 5;
              do {
                pppppuVar28 = ppppppuVar29[2];
                if (pppppuVar28 != (undefined8 *****)0x0) {
                  pppppuVar24 = ppppppuVar29[1];
                  lVar33 = 0;
                  do {
                    if (*(longlong *)((longlong)pppppuVar24 + lVar33) == 0) {
                      plVar6 = *(longlong **)((longlong)pppppuVar24 + lVar33 + 8);
                      LOCK();
                      bVar55 = plVar6[1] == 1;
                      if (bVar55) {
                        plVar6[1] = -1;
                      }
                      UNLOCK();
                      if ((!bVar55) || (plVar6[1] = 1, *plVar6 != 1)) {
                        *(undefined1 *)((longlong)pppppppuVar46 + 0x337) = 1;
                    // WARNING: Subroutine does not return
                        *(undefined **)(puVar36 + -8) = &UNK_141473321;
                        core::option::unwrap_failed();
                      }
                      lVar8 = *(longlong *)((longlong)pppppuVar24 + lVar33 + 8);
                      if (SBORROW8(0,*(longlong *)(lVar8 + 0x10))) {
                        fVar59 = SQRT(*(float *)(lVar8 + 0x34)) * 10.0;
                        fVar58 = 0.0;
                        if (0.0 <= fVar59) {
                          fVar58 = fVar59;
                        }
                        fVar59 = 20.0;
                        if (fVar58 <= 20.0) {
                          fVar59 = fVar58;
                        }
                        *(float *)(lVar8 + 0x34) = fVar59;
                        *(undefined1 *)((longlong)pppppppuVar46 + 0x336) = 1;
                      }
                      if (*(longlong *)((longlong)pppppuVar24 + lVar33) == 0) {
                        plVar6 = *(longlong **)((longlong)pppppuVar24 + lVar33 + 8);
                        LOCK();
                        bVar55 = plVar6[1] == 1;
                        if (bVar55) {
                          plVar6[1] = -1;
                        }
                        UNLOCK();
                        if ((!bVar55) || (plVar6[1] = 1, *plVar6 != 1)) {
                          *(undefined1 *)((longlong)pppppppuVar46 + 0x337) = 1;
                    // WARNING: Subroutine does not return
                          *(undefined **)(puVar36 + -8) = &UNK_141473336;
                          core::option::unwrap_failed();
                        }
                        lVar8 = *(longlong *)((longlong)pppppuVar24 + lVar33 + 8);
                        if (*(longlong *)(lVar8 + 0x10) == -0x7fffffffffffffff) {
                          auVar61._8_8_ = 0;
                          auVar61._0_8_ = *(ulonglong *)(lVar8 + 0x1c);
                          auVar61 = sqrtps(auVar61,auVar61);
                          auVar62._0_4_ = auVar61._0_4_ * 10.0;
                          auVar62._4_4_ = auVar61._4_4_ * 10.0;
                          auVar62._8_4_ = auVar61._8_4_ * 0.0;
                          auVar62._12_4_ = auVar61._12_4_ * 0.0;
                          auVar61 = maxps(ZEXT816(0),auVar62);
                          auVar61 = minps(auVar57,auVar61);
                          *(longlong *)(lVar8 + 0x1c) = auVar61._0_8_;
                          *(undefined1 *)((longlong)pppppppuVar46 + 0x336) = 1;
                        }
                      }
                    }
                    lVar33 = lVar33 + 0x18;
                  } while ((longlong)pppppuVar28 * 0x18 != lVar33);
                }
                ppppppuVar29 = ppppppuVar29 + 5;
              } while (ppppppuVar29 != ppppppuVar31);
            }
          }
        }
        if (*(char *)((longlong)pppppppuVar46 + 0x336) != '\x01') {
          *(undefined1 *)(pppppppuVar46 + 0x1b) = 0;
        }
        *(undefined1 *)((longlong)pppppppuVar46 + 0x337) = 0;
        *(undefined **)(puVar36 + -8) = &UNK_1414731c9;
        resources::history::History::finish_potential_mutation();
        if ((*(byte *)((longlong)pppppppuVar46 + 0x336) & 1) == 0) {
          in_RCX = pppppppuVar46[7];
          uVar63 = *(undefined4 *)(pppppppuVar46 + -4);
          uVar64 = *(undefined4 *)((longlong)pppppppuVar46 + -0x1c);
          uVar65 = *(undefined4 *)(pppppppuVar46 + -3);
          uVar66 = *(undefined4 *)((longlong)pppppppuVar46 + -0x14);
          uVar67 = *(undefined4 *)(pppppppuVar46 + -2);
          uVar68 = *(undefined4 *)((longlong)pppppppuVar46 + -0xc);
          uVar69 = *(undefined4 *)(pppppppuVar46 + -1);
          uVar70 = *(undefined4 *)((longlong)pppppppuVar46 + -4);
          uVar71 = *(undefined4 *)((longlong)pppppppuVar46 + 0x134);
          uVar19 = *(undefined4 *)((longlong)pppppppuVar46 + 0x304);
          pppppppuVar53 = pppppppuVar46 + 0x4a;
          goto code_r0x000141471c23;
        }
        in_RCX = pppppppuVar46[7];
        uVar63 = *(undefined4 *)(pppppppuVar46 + -4);
        uVar64 = *(undefined4 *)((longlong)pppppppuVar46 + -0x1c);
        uVar65 = *(undefined4 *)(pppppppuVar46 + -3);
        uVar66 = *(undefined4 *)((longlong)pppppppuVar46 + -0x14);
        uVar67 = *(undefined4 *)(pppppppuVar46 + -2);
        uVar68 = *(undefined4 *)((longlong)pppppppuVar46 + -0xc);
        uVar69 = *(undefined4 *)(pppppppuVar46 + -1);
        uVar70 = *(undefined4 *)((longlong)pppppppuVar46 + -4);
        uVar71 = *(undefined4 *)((longlong)pppppppuVar46 + 0x134);
        pppppppuVar53 = pppppppuVar46 + 0x4a;
        if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
          pppppppuVar46[0x2c] = (undefined8 ******)&UNK_142c647a0;
          pppppppuVar46[0x2d] = (undefined8 ******)0x1;
          pppppppuVar46[0x2e] = (undefined8 ******)0x8;
          pppppppuVar46[0x2f] = (undefined8 ******)0x0;
          pppppppuVar46[0x30] = (undefined8 ******)0x0;
          *(undefined **)(puVar36 + -8) = &UNK_141473241;
          toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
          pppppppuVar46[0x4e] = (undefined8 ******)&DAT_142c64148;
          pppppppuVar46[0x4f] = (undefined8 ******)0x26;
          pppppppuVar46[0x50] = (undefined8 ******)&DAT_142c64148;
          pppppppuVar46[0x51] = (undefined8 ******)0x26;
          pppppppuVar46[0x52] = extraout_RAX_21;
          *(undefined **)(puVar36 + -8) = &UNK_14147328b;
          ::log::__private_api::log<tuple_<>,log::__private_api::GlobalLogger>();
        }
      }
      *(undefined **)(puVar36 + -8) = &UNK_141471cd9;
      serde_json::value::index::impl$1::index_into();
      pppppppuVar46[0x2c] = (undefined8 ******)&UNK_142c647e0;
      pppppppuVar46[0x2d] = (undefined8 ******)0x1;
      pppppppuVar46[0x2e] = (undefined8 ******)0x8;
      pppppppuVar46[0x2f] = (undefined8 ******)0x0;
      pppppppuVar46[0x30] = (undefined8 ******)0x0;
      *(undefined **)(puVar36 + -8) = &UNK_141471d16;
      anyhow::__private::format_err();
      pppppppuVar46[0x2c] = extraout_RAX_14;
      ppppppuVar31 = extraout_RAX_14;
      if (extraout_RAX_13 != (undefined8 ******)0x0) {
        *(undefined **)(puVar36 + -8) = &UNK_141471d32;
        anyhow::error::impl$6::drop();
        ppppppuVar31 = extraout_RAX_13;
      }
      *(undefined **)(puVar36 + -8) = &UNK_141472aeb;
      core::result::Result::
      and_then<ref_<enum2_<serde_json::value::Value>_>,anyhow::Error,country_core::resources::history::HistoryIdentity,country_core::systems::save::load_game::parse_saved_game_data::closure_env_4>
                ();
      puVar12 = __imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E;
      if (extraout_RAX_16 != 0) {
        pppppppuVar46[0x1b] = ppppppuVar31;
        if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
          pppppppuVar46[0x4a] = pppppppuVar46 + 0x1b;
          pppppppuVar46[0x4b] = (undefined8 ******)anyhow::error::impl$5::fmt;
          pppppppuVar46[0x2c] = (undefined8 ******)&UNK_142c64810;
          pppppppuVar46[0x2d] = (undefined8 ******)0x2;
          pppppppuVar46[0x30] = (undefined8 ******)0x0;
          pppppppuVar46[0x2e] = pppppppuVar53;
          pppppppuVar46[0x2f] = (undefined8 ******)0x1;
          *(undefined **)(puVar36 + -8) = &UNK_141472b70;
          toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
          pppppppuVar46[0x4e] = (undefined8 ******)&DAT_142c64148;
          pppppppuVar46[0x4f] = (undefined8 ******)0x26;
          pppppppuVar46[0x50] = (undefined8 ******)&DAT_142c64148;
          pppppppuVar46[0x51] = (undefined8 ******)0x26;
          pppppppuVar46[0x52] = extraout_RAX_17;
          *(undefined **)(puVar36 + -8) = &UNK_141472bba;
          ::log::__private_api::log<tuple_<>,log::__private_api::GlobalLogger>();
        }
        *(undefined **)(puVar36 + -8) = &UNK_141472bc6;
        save_file_identity::derive_fallback_save_file_identity();
        pppppppuVar46[6] = extraout_RAX_18;
        ppppppuVar31 = extraout_RAX_18;
        if (2 < *(ulonglong *)puVar12) {
          pppppppuVar46[0x4a] = pppppppuVar46 + 6;
          pppppppuVar46[0x4b] = (undefined8 ******)core::fmt::num::imp::impl$9::fmt;
          pppppppuVar46[0x2c] = (undefined8 ******)&UNK_142c64870;
          pppppppuVar46[0x2d] = (undefined8 ******)0x1;
          pppppppuVar46[0x30] = (undefined8 ******)0x0;
          pppppppuVar46[0x2e] = pppppppuVar53;
          pppppppuVar46[0x2f] = (undefined8 ******)0x1;
          *(undefined **)(puVar36 + -8) = &UNK_141472c35;
          toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
          pppppppuVar46[0x4e] = (undefined8 ******)&DAT_142c64148;
          pppppppuVar46[0x4f] = (undefined8 ******)0x26;
          pppppppuVar46[0x50] = (undefined8 ******)&DAT_142c64148;
          pppppppuVar46[0x51] = (undefined8 ******)0x26;
          pppppppuVar46[0x52] = extraout_RAX_19;
          *(undefined **)(puVar36 + -8) = &UNK_141472c7f;
          ::log::__private_api::log<tuple_<>,log::__private_api::GlobalLogger>();
          ppppppuVar31 = pppppppuVar46[6];
        }
        *(undefined **)(puVar36 + -8) = &UNK_141472c8f;
        anyhow::error::impl$6::drop();
      }
      ppppppuVar29 = pppppppuVar46[0x18];
      *(undefined **)(puVar36 + -8) = &UNK_141472cba;
      alloc::vec::spec_from_iter::impl$0::
      from_iter<country_core::resources::history::MultiEdit,core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<country_core::resources::history::MultiEdit>_>_>
                ();
      *(undefined **)(puVar36 + -8) = &UNK_141472cdf;
      alloc::vec::spec_from_iter::impl$0::
      from_iter<country_core::resources::history::ResolvedHistoryChainSnapshot,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::history::HistoryChainSnapshot>,country_core::resources::history::impl_1::serialize::closure_env_0<ref_mut_<serde_json::ser::Serializer<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>_>_>_>
                ();
      ppppppuVar23 = pppppppuVar46[8];
      if (SBORROW8(0,(longlong)ppppppuVar23)) {
        pppppppuVar46[0x4e] = (undefined8 ******)0x8000000000000000;
      }
      else {
        *(undefined **)(puVar36 + -8) = &UNK_141472d1b;
        alloc::vec::impl$11::
        clone<country_core::resources::critters::sheep::control::SheepSnapshot,alloc::alloc::Global>
                  ();
      }
      pppppppuVar46[0x35] = ppppppuVar31;
      *(undefined4 *)(pppppppuVar46 + 0x44) = *(undefined4 *)((longlong)pppppppuVar46 + 0x11c);
      *(undefined4 *)((longlong)pppppppuVar46 + 0x224) = unaff_XMM11_Da;
      *(int *)(pppppppuVar46 + 0x45) = (int)pppppppuVar46[3];
      *(int *)((longlong)pppppppuVar46 + 0x22c) = (int)((ulonglong)pppppppuVar46[3] >> 0x20);
      pppppppuVar46[0x46] = pppppppuVar46[2];
      *(undefined4 *)(pppppppuVar46 + 0x47) = unaff_XMM10_Da;
      *(undefined4 *)((longlong)pppppppuVar46 + 0x23c) = unaff_XMM10_Db;
      *(undefined4 *)(pppppppuVar46 + 0x48) = unaff_XMM10_Dc;
      *(undefined4 *)((longlong)pppppppuVar46 + 0x244) = unaff_XMM10_Dd;
      *(undefined4 *)((longlong)pppppppuVar46 + 500) =
           *(undefined4 *)((longlong)pppppppuVar46 + 0x2f4);
      *(undefined4 *)(pppppppuVar46 + 0x3f) = unaff_XMM8_Da;
      *(undefined4 *)((longlong)pppppppuVar46 + 0x1fc) = unaff_XMM8_Db;
      *(undefined4 *)(pppppppuVar46 + 0x40) = unaff_XMM8_Dc;
      *(undefined4 *)((longlong)pppppppuVar46 + 0x204) = unaff_XMM8_Dd;
      *(undefined4 *)(pppppppuVar46 + 0x41) = unaff_XMM12_Da;
      *(undefined4 *)((longlong)pppppppuVar46 + 0x20c) = unaff_XMM12_Db;
      *(undefined4 *)(pppppppuVar46 + 0x42) = unaff_XMM12_Dc;
      *(undefined4 *)((longlong)pppppppuVar46 + 0x214) = unaff_XMM12_Dd;
      *(undefined4 *)(pppppppuVar46 + 0x43) = unaff_XMM13_Da;
      *(undefined2 *)((longlong)pppppppuVar46 + 0x21c) =
           *(undefined2 *)((longlong)pppppppuVar46 + 0x316);
      pppppppuVar46[0x49] = ppppppuVar29;
      pppppppuVar46[0x2c] = pppppppuVar46[0x1b];
      pppppppuVar46[0x2d] = pppppppuVar46[0x1c];
      pppppppuVar46[0x2e] = pppppppuVar46[0x1d];
      pppppppuVar46[0x2f] = pppppppuVar46[0x4a];
      pppppppuVar46[0x30] = pppppppuVar46[0x4b];
      pppppppuVar46[0x31] = pppppppuVar46[0x4c];
      pppppppuVar46[0x34] = pppppppuVar46[0x50];
      pppppppuVar46[0x32] = pppppppuVar46[0x4e];
      pppppppuVar46[0x33] = pppppppuVar46[0x4f];
      *(undefined4 *)(pppppppuVar46 + 0x36) = uVar63;
      *(undefined4 *)((longlong)pppppppuVar46 + 0x1b4) = uVar64;
      *(undefined4 *)(pppppppuVar46 + 0x37) = uVar65;
      *(undefined4 *)((longlong)pppppppuVar46 + 0x1bc) = uVar66;
      *(undefined4 *)(pppppppuVar46 + 0x38) = *(undefined4 *)((longlong)pppppppuVar46 + 0x124);
      *(char *)((longlong)pppppppuVar46 + 0x1c4) = (char)*(undefined4 *)(pppppppuVar46 + 0x25);
      *(undefined4 *)(pppppppuVar46 + 0x39) = *(undefined4 *)(pppppppuVar46 + 0x24);
      *(undefined8 *******)((longlong)pppppppuVar46 + 0x1cc) = unaff_XMM14_Qa;
      *(undefined8 *******)((longlong)pppppppuVar46 + 0x1d4) = unaff_XMM14_Qb;
      *(undefined4 *)((longlong)pppppppuVar46 + 0x1dc) = uVar67;
      *(undefined4 *)(pppppppuVar46 + 0x3c) = uVar68;
      *(undefined4 *)((longlong)pppppppuVar46 + 0x1e4) = uVar69;
      *(undefined4 *)(pppppppuVar46 + 0x3d) = uVar70;
      *(undefined4 *)((longlong)pppppppuVar46 + 0x1ec) = uVar71;
      *(char *)(pppppppuVar46 + 0x3e) = (char)*(undefined4 *)((longlong)pppppppuVar46 + 300);
      *(char *)((longlong)pppppppuVar46 + 0x1f1) = (char)*(undefined4 *)(pppppppuVar46 + 0x26);
      *(undefined1 *)((longlong)pppppppuVar46 + 0x1f2) =
           *(undefined1 *)((longlong)pppppppuVar46 + 0x32f);
      *(undefined **)(puVar36 + -8) = &UNK_141472e5e;
      alloc::alloc::exchange_malloc();
      *(undefined **)(puVar36 + -8) = &UNK_141472e76;
      memcpy(_Dst,pppppppuVar46 + 0x2c,0xf0);
      bVar18 = *(byte *)((longlong)pppppppuVar46 + 0x336);
      uVar71 = *(undefined4 *)(pppppppuVar46 + 0x5e);
      uVar63 = *(undefined4 *)((longlong)pppppppuVar46 + 0x304);
      *in_RCX = _Dst;
      *(int *)(in_RCX + 1) = *(int *)((longlong)pppppppuVar46 + 0xd4);
      *(byte *)((longlong)in_RCX + 0xc) = unaff_R13D != 0x1b | bVar18;
      *(byte *)((longlong)in_RCX + 0xd) = (byte)uVar71 & 1;
      *(byte *)((longlong)in_RCX + 0xe) = (byte)uVar63 & 1;
      *(bool *)((longlong)in_RCX + 0xf) = bVar56;
      *(undefined **)(puVar36 + -8) = &UNK_141472ec0;
      core::ptr::drop_in_place<country_core::resources::history::History>();
      if ((!SBORROW8(0,(longlong)ppppppuVar23)) && (ppppppuVar23 != (undefined8 ******)0x0)) {
        *(undefined **)(puVar36 + -8) = &UNK_141472ee1;
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      *(undefined **)(puVar36 + -8) = &UNK_141472eed;
      core::ptr::drop_in_place<enum2_<serde_json::value::Value>_>();
      puVar35 = puVar36;
      goto LAB_14147147c;
    }
    *in_RCX = ppppppuVar31;
    *(undefined1 *)((longlong)in_RCX + 0xf) = 2;
    if ((!SBORROW8(0,(longlong)pppppppuVar46[8])) && (pppppppuVar46[8] != (undefined8 ******)0x0)) {
      puStack_4b0 = &UNK_141471b85;
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    goto code_r0x0001414713ff;
  case 0xc:
    goto code_r0x000141471278;
  case 0xd:
    out(*(undefined4 *)*pauVar21,0x4d28);
    in_R8 = (undefined8 ******)CONCAT71(uVar34,(byte)in_R8 | (char)pauVar21 + 4U);
    pcStack_2c0 = in_MM0;
    goto code_r0x0001414712e4;
  case 0xe:
    if (pcVar25 != (char *)0x0) {
      (*pauVar22)[0] = (*pauVar22)[0] + cVar17;
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    }
    (*pauVar22)[0] = (*pauVar22)[0] + cVar17;
    goto code_r0x000141471278;
  case 0xf:
    (*pauVar22)[0] = (*pauVar22)[0] + cVar17;
    goto code_r0x0001414712bf;
  case 0x10:
    goto code_r0x000141471284;
  case 0x11:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x12:
  case 0x15:
  case 0x18:
  case 0x19:
    goto code_r0x00014147222d;
  case 0x13:
    bVar56 = uVar49 < 8;
    bVar55 = uVar49 == 8;
    goto code_r0x000141472286;
  case 0x14:
    goto code_r0x0001414722a1;
  case 0x16:
  case 0x23:
    goto code_r0x0001414722b7;
  case 0x17:
    goto code_r0x000141472288;
  case 0x1a:
    goto code_r0x0001414722d0;
  case 0x1b:
  case 0x1e:
  case 0x21:
  case 0x22:
    goto code_r0x000141471e83;
  case 0x1c:
    pcVar1 = (char *)((longlong)pauVar22[-8] + 0xb);
    *pcVar1 = *pcVar1 + (byte)pcVar25;
    *(byte *)unaff_R15 = *(byte *)unaff_R15 | (byte)pcVar25;
    *(byte *)((longlong)in_R8 + 0x157404f9) = *(byte *)((longlong)in_R8 + 0x157404f9) - cVar17;
    puVar37 = auStack_4a8;
    goto code_r0x00014147223a;
  case 0x1d:
    *(byte *)((longlong)in_RCX + 0x2f) = *(byte *)((longlong)in_RCX + 0x2f) | 0x4d;
code_r0x000141472288:
                    // WARNING: Could not recover jumptable at 0x000141472296. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&UNK_142c64d94 + *(int *)(&UNK_142c64d94 + (longlong)pauVar47 * 4)))
              (pcVar25,&UNK_142c64d94 + *(int *)(&UNK_142c64d94 + (longlong)pauVar47 * 4));
    return;
  case 0x1f:
    ppuVar41 = &puStack_4b0;
    puStack_4b0 = auStack_4a8;
    goto code_r0x00014147229f;
  case 0x20:
    goto code_r0x00014147226f;
  case 0x24:
  case 0x27:
  case 0x2a:
  case 0x2b:
    goto code_r0x0001414726c3;
  case 0x25:
    goto code_r0x0001414726dc;
  case 0x26:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x28:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x29:
    piVar2 = (int *)((longlong)pauVar22[-8] + 5);
    *piVar2 = *piVar2 + -1;
    puVar35 = auStack_420;
    goto code_r0x0001414726e5;
  case 0x2c:
    (*pauVar22)[0] = (*pauVar22)[0] + cVar17;
    (*pauVar22)[0] = (*pauVar22)[0] + cVar17;
    puVar35 = auStack_4a8;
    local_428 = pppppppuVar46;
    goto code_r0x000141472700;
  default:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  }
  puStack_4b0 = &UNK_141471017;
  serde_json::value::de::impl$2::
  deserialize_i32<serde::de::impls::impl_54::deserialize::PrimitiveVisitor>();
  if (((ulonglong)local_1b8 & 1) == 0) {
    auStack_128[0] = local_1b8._4_4_;
    if ((int)local_1b8._4_4_ < 3) {
      local_1b8 = auStack_128;
      pcStack_1b0 = core::fmt::num::imp::impl$7::fmt;
      local_1a8 = (undefined1 (*) [16])&UNK_142c649a8;
      pcStack_1a0 = core::fmt::num::imp::impl$7::fmt;
      local_2c8 = (uint *)&UNK_142c649e8;
      pcStack_2c0 = (code *)0x2;
      pcStack_2b0 = (code *)0x2;
      local_2a8 = 0;
      puStack_4b0 = &UNK_1414710c0;
      local_2b8 = (undefined1 (*) [16])&local_1b8;
      alloc::fmt::format::format_inner();
      puStack_4b0 = &UNK_1414710c9;
      anyhow::Error::msg<alloc::string::String>();
      pppppuVar28 = extraout_RAX_03;
    }
    else {
      if (local_1b8._4_4_ < 0x1c) {
        puStack_4b0 = &UNK_141471175;
        serde_json::value::index::impl$1::index_into();
        pbVar20 = extraout_RAX_05;
        if (extraout_RAX_05 == (byte *)0x0) {
          pbVar20 = 
          __imp__ZN10serde_json5value5index87__LT_impl_u20_core__ops__index__Index_LT_I_GT__u20_for_u20_serde_json__value__Value_GT_5index4NULL17hd2864d0cd0b4dde5E
          ;
        }
code_r0x000141471181:
        pcVar25 = (char *)(ulonglong)*pbVar20;
        ppppppuVar31 = (undefined8 ******)&UNK_142c64d40;
code_r0x00014147118b:
        UNRECOVERED_JUMPTABLE_00 =
             (code *)((longlong)*(int *)((longlong)ppppppuVar31 + (longlong)pcVar25 * 4) +
                     (longlong)ppppppuVar31);
                    // WARNING: Could not recover jumptable at 0x000141471192. Too many branches
                    // WARNING: Treating indirect jump as call
        (*UNRECOVERED_JUMPTABLE_00)(UNRECOVERED_JUMPTABLE_00);
        return;
      }
      local_1b8 = auStack_128;
      pcStack_1b0 = core::fmt::num::imp::impl$7::fmt;
      local_1a8 = (undefined1 (*) [16])&UNK_142c645e0;
      pcStack_1a0 = core::fmt::num::imp::impl$7::fmt;
      local_2c8 = (uint *)&UNK_142c64988;
      pcStack_2c0 = (code *)0x2;
      pcStack_2b0 = (code *)0x2;
      local_2a8 = 0;
      puStack_4b0 = &UNK_14147114f;
      local_2b8 = (undefined1 (*) [16])&local_1b8;
      alloc::fmt::format::format_inner();
      puStack_4b0 = &UNK_141471158;
      anyhow::Error::msg<alloc::string::String>();
      pppppuVar28 = extraout_RAX_04;
    }
  }
  else {
    puStack_4b0 = &UNK_14147102c;
    anyhow::error::impl$1::from<serde_json::error::Error>();
    pppppuVar28 = extraout_RAX_02;
  }
code_r0x0001414713f8:
  *in_RCX = pppppuVar28;
  *(undefined1 *)((longlong)in_RCX + 0xf) = 2;
code_r0x0001414713ff:
  bVar18 = *(byte *)(pppppppuVar46 + 0x27);
  puVar35 = auStack_4a8;
  if (2 < bVar18) {
    if (bVar18 == 3) {
      puVar35 = auStack_4a8;
      if (pppppppuVar46[0x28] != (undefined8 ******)0x0) {
        puStack_4b0 = &UNK_14147146e;
        std::alloc::__default_lib_allocator::__rust_dealloc();
        puVar35 = auStack_4a8;
      }
    }
    else if (bVar18 == 4) {
      pppppppuVar46[100] = pppppppuVar46[0x29];
      puStack_4b0 = &UNK_14147142c;
      core::ptr::drop_in_place<slice2_<enum2_<serde_json::value::Value>_>_>();
      puVar35 = auStack_4a8;
      if (pppppppuVar46[0x28] != (undefined8 ******)0x0) {
        puStack_4b0 = &UNK_14147144e;
        std::alloc::__default_lib_allocator::__rust_dealloc();
        puVar35 = auStack_4a8;
      }
    }
    else {
      puStack_4b0 = (undefined *)0x14147147c;
      core::ptr::
      drop_in_place<serde_json::map::Map<alloc::string::String,enum2_<serde_json::value::Value>_>_>
                ();
      puVar35 = auStack_4a8;
    }
  }
LAB_14147147c:
  if (pppppppuVar46[0x1e] != (undefined8 ******)0x0) {
    *(undefined8 *)(puVar35 + -8) = 0x14147149a;
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
                    // WARNING: Read-only address (ram,0x000142aba9c0) is written
  return;
code_r0x0001414724e2:
  pppppuVar28 = ppppppuVar23[3];
  in_R10 = (undefined8 ****)0xffffffffffffffff;
  puVar36 = puVar35;
  if (pppppuVar28[5] != (undefined8 ****)0x0) {
    LOCK();
    bVar56 = pppppuVar28[1] == (undefined8 ****)0x1;
    if (bVar56) {
      pppppuVar28[1] = (undefined8 ****)0xffffffffffffffff;
    }
    UNLOCK();
    if ((!bVar56) || (pppppuVar28[1] = (undefined8 ****)0x1, *pppppuVar28 != (undefined8 ****)0x1))
    {
      *(undefined1 *)((longlong)pppppppuVar46 + 0x337) = 1;
                    // WARNING: Subroutine does not return
      *(undefined **)(puVar35 + -8) = &UNK_14147336e;
      core::option::unwrap_failed();
    }
    ppppppuVar27 = (undefined8 ******)pppppppuVar46[0x5f][3][5];
    if (ppppppuVar27 == (undefined8 ******)0x0) {
      ppppppuVar23 = pppppppuVar46[0x5f];
    }
    else {
      pauVar48 = (undefined1 (*) [16])pppppppuVar46[0x5f][3][2];
      pppppppuVar46[0x2b] = (undefined8 ******)(pauVar48 + 1);
      auVar57 = *pauVar48;
      pppppppuVar46[100] =
           (undefined8 ******)
           (ulonglong)
           ~(uint)(ushort)((ushort)(SUB161(auVar57 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar57 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar57 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar57 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar57 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar57 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar57 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar57 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar57 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar57 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar57 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar57 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar57 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar57 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar57 >> 0x77,0) & 1) << 0xe |
                          (ushort)(byte)(auVar57[0xf] >> 7) << 0xf);
      pppppppuVar46[0x20] = ppppppuVar31;
      local_428 = pppppppuVar46;
      do {
        uVar49 = (uint)local_428[100];
        local_428[99] = ppppppuVar27;
        if ((short)local_428[100] == 0) {
          pauVar21 = (undefined1 (*) [16])local_428[0x2b];
          do {
            auVar57 = *pauVar21;
            uVar9 = (ushort)(SUB161(auVar57 >> 7,0) & 1) |
                    (ushort)(SUB161(auVar57 >> 0xf,0) & 1) << 1 |
                    (ushort)(SUB161(auVar57 >> 0x17,0) & 1) << 2 |
                    (ushort)(SUB161(auVar57 >> 0x1f,0) & 1) << 3 |
                    (ushort)(SUB161(auVar57 >> 0x27,0) & 1) << 4 |
                    (ushort)(SUB161(auVar57 >> 0x2f,0) & 1) << 5 |
                    (ushort)(SUB161(auVar57 >> 0x37,0) & 1) << 6 |
                    (ushort)(SUB161(auVar57 >> 0x3f,0) & 1) << 7 |
                    (ushort)(SUB161(auVar57 >> 0x47,0) & 1) << 8 |
                    (ushort)(SUB161(auVar57 >> 0x4f,0) & 1) << 9 |
                    (ushort)(SUB161(auVar57 >> 0x57,0) & 1) << 10 |
                    (ushort)(SUB161(auVar57 >> 0x5f,0) & 1) << 0xb |
                    (ushort)(SUB161(auVar57 >> 0x67,0) & 1) << 0xc |
                    (ushort)(SUB161(auVar57 >> 0x6f,0) & 1) << 0xd |
                    (ushort)(SUB161(auVar57 >> 0x77,0) & 1) << 0xe |
                    (ushort)(byte)(auVar57[0xf] >> 7) << 0xf;
            pauVar48 = pauVar48 + -0x70;
            pauVar21 = pauVar21 + 1;
          } while (uVar9 == 0xffff);
          local_428[0x2b] = (undefined8 ******)pauVar21;
          uVar49 = ~(uint)uVar9;
          uVar50 = uVar49 & -(uint)uVar9 - 2;
        }
        else {
          uVar50 = uVar49 - 1 & uVar49;
        }
        uVar51 = 0;
        for (; (uVar49 & 1) == 0; uVar49 = uVar49 >> 1 | 0x80000000) {
          uVar51 = uVar51 + 1;
        }
        uVar26 = (ulonglong)uVar51;
        local_428[100] = (undefined8 ******)(ulonglong)uVar50;
        pauVar47 = *(undefined1 (**) [16])pauVar48[uVar26 * -7 + -6];
        in_R10 = *(undefined8 *****)(pauVar48[uVar26 * -7 + -5] + 8);
        auVar57 = *pauVar47;
        ppppppuVar31 = (undefined8 ******)
                       (ulonglong)
                       ~(uint)(ushort)((ushort)(SUB161(auVar57 >> 7,0) & 1) |
                                       (ushort)(SUB161(auVar57 >> 0xf,0) & 1) << 1 |
                                       (ushort)(SUB161(auVar57 >> 0x17,0) & 1) << 2 |
                                       (ushort)(SUB161(auVar57 >> 0x1f,0) & 1) << 3 |
                                       (ushort)(SUB161(auVar57 >> 0x27,0) & 1) << 4 |
                                       (ushort)(SUB161(auVar57 >> 0x2f,0) & 1) << 5 |
                                       (ushort)(SUB161(auVar57 >> 0x37,0) & 1) << 6 |
                                       (ushort)(SUB161(auVar57 >> 0x3f,0) & 1) << 7 |
                                       (ushort)(SUB161(auVar57 >> 0x47,0) & 1) << 8 |
                                       (ushort)(SUB161(auVar57 >> 0x4f,0) & 1) << 9 |
                                       (ushort)(SUB161(auVar57 >> 0x57,0) & 1) << 10 |
                                       (ushort)(SUB161(auVar57 >> 0x5f,0) & 1) << 0xb |
                                       (ushort)(SUB161(auVar57 >> 0x67,0) & 1) << 0xc |
                                       (ushort)(SUB161(auVar57 >> 0x6f,0) & 1) << 0xd |
                                       (ushort)(SUB161(auVar57 >> 0x77,0) & 1) << 0xe |
                                      (ushort)(byte)(auVar57[0xf] >> 7) << 0xf);
        in_R11 = pauVar47 + 1;
        pauVar21 = *(undefined1 (**) [16])pauVar48[uVar26 * -7 + -3];
        auVar57 = *pauVar21;
        pppppuVar28 = (undefined8 *****)
                      (ulonglong)
                      ~(uint)(ushort)((ushort)(SUB161(auVar57 >> 7,0) & 1) |
                                      (ushort)(SUB161(auVar57 >> 0xf,0) & 1) << 1 |
                                      (ushort)(SUB161(auVar57 >> 0x17,0) & 1) << 2 |
                                      (ushort)(SUB161(auVar57 >> 0x1f,0) & 1) << 3 |
                                      (ushort)(SUB161(auVar57 >> 0x27,0) & 1) << 4 |
                                      (ushort)(SUB161(auVar57 >> 0x2f,0) & 1) << 5 |
                                      (ushort)(SUB161(auVar57 >> 0x37,0) & 1) << 6 |
                                      (ushort)(SUB161(auVar57 >> 0x3f,0) & 1) << 7 |
                                      (ushort)(SUB161(auVar57 >> 0x47,0) & 1) << 8 |
                                      (ushort)(SUB161(auVar57 >> 0x4f,0) & 1) << 9 |
                                      (ushort)(SUB161(auVar57 >> 0x57,0) & 1) << 10 |
                                      (ushort)(SUB161(auVar57 >> 0x5f,0) & 1) << 0xb |
                                      (ushort)(SUB161(auVar57 >> 0x67,0) & 1) << 0xc |
                                      (ushort)(SUB161(auVar57 >> 0x6f,0) & 1) << 0xd |
                                      (ushort)(SUB161(auVar57 >> 0x77,0) & 1) << 0xe |
                                     (ushort)(byte)(auVar57[0xf] >> 7) << 0xf);
        pauVar22 = pauVar21 + 1;
        local_428[0x61] = (undefined8 ******)pauVar48;
        pcVar25 = *(char **)(pauVar48[uVar26 * -7 + -2] + 8);
        pppppppuVar46 = local_428;
        if (pauVar47 != (undefined1 (*) [16])0x0) goto code_r0x00014147267e;
code_r0x0001414726dc:
        local_428 = pppppppuVar46;
        in_RCX = ppppppuVar29;
        if (pauVar21 != (undefined1 (*) [16])0x0) {
code_r0x0001414726e5:
          uVar49 = (uint)pppppuVar28;
          ppppppuVar29 = in_RCX;
          if (pcVar25 == (char *)0x0) goto code_r0x00014147255f;
          if ((short)pppppuVar28 == 0) {
code_r0x000141472700:
            do {
              auVar57 = *pauVar22;
              pppppuVar28 = (undefined8 *****)
                            (ulonglong)
                            (ushort)((ushort)(SUB161(auVar57 >> 7,0) & 1) |
                                     (ushort)(SUB161(auVar57 >> 0xf,0) & 1) << 1 |
                                     (ushort)(SUB161(auVar57 >> 0x17,0) & 1) << 2 |
                                     (ushort)(SUB161(auVar57 >> 0x1f,0) & 1) << 3 |
                                     (ushort)(SUB161(auVar57 >> 0x27,0) & 1) << 4 |
                                     (ushort)(SUB161(auVar57 >> 0x2f,0) & 1) << 5 |
                                     (ushort)(SUB161(auVar57 >> 0x37,0) & 1) << 6 |
                                     (ushort)(SUB161(auVar57 >> 0x3f,0) & 1) << 7 |
                                     (ushort)(SUB161(auVar57 >> 0x47,0) & 1) << 8 |
                                     (ushort)(SUB161(auVar57 >> 0x4f,0) & 1) << 9 |
                                     (ushort)(SUB161(auVar57 >> 0x57,0) & 1) << 10 |
                                     (ushort)(SUB161(auVar57 >> 0x5f,0) & 1) << 0xb |
                                     (ushort)(SUB161(auVar57 >> 0x67,0) & 1) << 0xc |
                                     (ushort)(SUB161(auVar57 >> 0x6f,0) & 1) << 0xd |
                                     (ushort)(SUB161(auVar57 >> 0x77,0) & 1) << 0xe |
                                    (ushort)(byte)(auVar57[0xf] >> 7) << 0xf);
              pauVar21 = pauVar21 + -0x58;
              pauVar22 = pauVar22 + 1;
code_r0x000141472714:
              puVar44 = puVar35;
              pppppppuVar46 = local_428;
            } while ((int)pppppuVar28 == 0xffff);
code_r0x00014147271d:
            pauVar48 = (undefined1 (*) [16])(ulonglong)(-(uint)pppppuVar28 - 2);
            uVar49 = ~(uint)pppppuVar28;
            pppppuVar28 = (undefined8 *****)(ulonglong)uVar49;
            in_RDX = (undefined8 ******)(ulonglong)uVar49;
            puVar37 = puVar44;
            local_428 = pppppppuVar46;
code_r0x00014147272c:
            puVar35 = puVar37;
            uVar49 = (uint)pppppuVar28;
            uVar50 = (uint)in_RDX & (uint)pauVar48;
            pauVar47 = pauVar21;
            ppppppuVar29 = in_RCX;
          }
          else {
            uVar50 = uVar49 - 1 & uVar49;
            pauVar47 = pauVar21;
          }
          pcVar25 = pcVar25 + -1;
          uVar26 = (ulonglong)uVar49;
          pppppuVar28 = (undefined8 *****)(ulonglong)uVar50;
          pauVar21 = pauVar47;
          pauVar48 = (undefined1 (*) [16])0x0;
code_r0x000141472652:
          uVar49 = 0;
          for (uVar50 = (uint)uVar26; (uVar50 & 1) == 0; uVar50 = uVar50 >> 1 | 0x80000000) {
            uVar49 = uVar49 + 1;
          }
          if ((byte)(*(char *)((longlong)pauVar47 + (ulonglong)uVar49 * -0x58 + -6) - 0x38U) < 3) {
            in_R8 = (undefined8 ******)((longlong)pauVar47 + (ulonglong)uVar49 * -0x58);
            uVar49 = *(uint *)(in_R8 + -5);
            in_RDX = (undefined8 ******)(ulonglong)uVar49;
            puVar45 = puVar35;
            pppppppuVar46 = local_428;
            in_RCX = ppppppuVar29;
            switch(in_RDX) {
            case (undefined8 ******)0x0:
            case (undefined8 ******)0x3:
            case (undefined8 ******)0x6:
            case (undefined8 ******)0x7:
              break;
            case (undefined8 ******)0x1:
              uVar49 = 2;
              break;
            case (undefined8 ******)0x2:
              uVar49 = 5;
              break;
            case (undefined8 ******)0x4:
              uVar49 = 8;
              break;
            case (undefined8 ******)0x5:
              uVar49 = 3;
              break;
            case (undefined8 ******)0x8:
              uVar49 = 4;
              break;
            default:
code_r0x000141472758:
              uVar49 = 1;
              puVar35 = puVar45;
              ppppppuVar29 = in_RCX;
              if ((int)in_RDX == 999) {
                uVar49 = 999;
              }
              goto code_r0x000141472799;
            }
            unaff_R13D = *(uint *)(local_428 + 0x66);
code_r0x000141472799:
            *(uint *)(in_R8 + -5) = uVar49;
            *(int *)(pppppppuVar46 + 0x5e) = (int)CONCAT71((int7)((ulonglong)in_R8 >> 8),1);
            local_428 = pppppppuVar46;
          }
          pauVar47 = pauVar48;
          pppppppuVar46 = local_428;
          if (pauVar47 == (undefined1 (*) [16])0x0) goto code_r0x0001414726dc;
code_r0x00014147267e:
          pppppppuVar46 = local_428;
          if (in_R10 != (undefined8 ****)0x0) {
            if ((short)ppppppuVar31 == 0) {
              do {
                auVar57 = *in_R11;
                uVar9 = (ushort)(SUB161(auVar57 >> 7,0) & 1) |
                        (ushort)(SUB161(auVar57 >> 0xf,0) & 1) << 1 |
                        (ushort)(SUB161(auVar57 >> 0x17,0) & 1) << 2 |
                        (ushort)(SUB161(auVar57 >> 0x1f,0) & 1) << 3 |
                        (ushort)(SUB161(auVar57 >> 0x27,0) & 1) << 4 |
                        (ushort)(SUB161(auVar57 >> 0x2f,0) & 1) << 5 |
                        (ushort)(SUB161(auVar57 >> 0x37,0) & 1) << 6 |
                        (ushort)(SUB161(auVar57 >> 0x3f,0) & 1) << 7 |
                        (ushort)(SUB161(auVar57 >> 0x47,0) & 1) << 8 |
                        (ushort)(SUB161(auVar57 >> 0x4f,0) & 1) << 9 |
                        (ushort)(SUB161(auVar57 >> 0x57,0) & 1) << 10 |
                        (ushort)(SUB161(auVar57 >> 0x5f,0) & 1) << 0xb |
                        (ushort)(SUB161(auVar57 >> 0x67,0) & 1) << 0xc |
                        (ushort)(SUB161(auVar57 >> 0x6f,0) & 1) << 0xd |
                        (ushort)(SUB161(auVar57 >> 0x77,0) & 1) << 0xe |
                        (ushort)(byte)(auVar57[0xf] >> 7) << 0xf;
                unaff_R15 = (undefined8 ******)(ulonglong)uVar9;
                pauVar47 = pauVar47 + -0x58;
                in_R11 = in_R11 + 1;
              } while (uVar9 == 0xffff);
              in_R8 = (undefined8 ******)0xfffffffe;
              puVar13 = puVar35;
code_r0x0001414726c3:
              local_428 = pppppppuVar46;
              puVar35 = puVar13;
              uVar49 = ~(uint)unaff_R15;
              uVar26 = (ulonglong)uVar49;
              ppppppuVar31 = (undefined8 ******)(ulonglong)(uVar49 & (int)in_R8 - (uint)unaff_R15);
              in_R10 = (undefined8 ****)((longlong)in_R10 + -1);
              pauVar48 = pauVar47;
            }
            else {
              in_R10 = (undefined8 ****)((longlong)in_R10 + -1);
              uVar26 = (ulonglong)ppppppuVar31 & 0xffffffff;
              ppppppuVar31 = (undefined8 ******)
                             (ulonglong)((uint)ppppppuVar31 - 1 & (uint)ppppppuVar31);
              pauVar48 = pauVar47;
            }
            goto code_r0x000141472652;
          }
          in_R10 = (undefined8 ****)0x0;
          goto code_r0x0001414726dc;
        }
code_r0x00014147255f:
        ppppppuVar27 = (undefined8 ******)((longlong)local_428[99] + -1);
        ppppppuVar23 = local_428[0x5f];
        ppppppuVar31 = local_428[0x20];
        pauVar48 = (undefined1 (*) [16])local_428[0x61];
        pppppppuVar46 = local_428;
      } while (ppppppuVar27 != (undefined8 ******)0x0);
code_r0x00014147287e:
      in_R10 = (undefined8 ****)0xffffffffffffffff;
      puVar36 = puVar35;
      if (0x14 < unaff_R13D) goto code_r0x000141471eae;
    }
    in_R10 = (undefined8 ****)0xffffffffffffffff;
    pppppuVar28 = ppppppuVar23[3];
    puVar36 = puVar35;
    if (pppppuVar28[5] != (undefined8 ****)0x0) {
      LOCK();
      bVar56 = pppppuVar28[1] == (undefined8 ****)0x1;
      if (bVar56) {
        pppppuVar28[1] = (undefined8 ****)0xffffffffffffffff;
      }
      UNLOCK();
      if ((!bVar56) || (pppppuVar28[1] = (undefined8 ****)0x1, *pppppuVar28 != (undefined8 ****)0x1)
         ) {
        *(undefined1 *)((longlong)pppppppuVar46 + 0x337) = 1;
                    // WARNING: Subroutine does not return
        *(undefined **)(puVar35 + -8) = &UNK_141473383;
        core::option::unwrap_failed();
      }
      pppppuVar28 = pppppppuVar46[0x5f][3];
      ppppuVar52 = pppppuVar28[5];
      if (ppppuVar52 != (undefined8 ****)0x0) {
        pppppppuVar46[0x20] = ppppppuVar31;
        pauVar21 = (undefined1 (*) [16])pppppuVar28[2];
        pppppppuVar46[0x61] = (undefined8 ******)(pauVar21 + 1);
        pppppppuVar46[99] = (undefined8 ******)pauVar21;
        auVar57 = *pauVar21;
        pppppppuVar46[100] =
             (undefined8 ******)
             (ulonglong)
             ~(uint)(ushort)((ushort)(SUB161(auVar57 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar57 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar57 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar57 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar57 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar57 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar57 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar57 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar57 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar57 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar57 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar57 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar57 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar57 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar57 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar57[0xf] >> 7) << 0xf);
code_r0x00014147291a:
        uVar49 = (uint)pppppppuVar46[100];
        if ((short)pppppppuVar46[100] == 0) {
          ppppppuVar31 = pppppppuVar46[99];
          pauVar21 = (undefined1 (*) [16])pppppppuVar46[0x61];
          do {
            auVar57 = *pauVar21;
            uVar9 = (ushort)(SUB161(auVar57 >> 7,0) & 1) |
                    (ushort)(SUB161(auVar57 >> 0xf,0) & 1) << 1 |
                    (ushort)(SUB161(auVar57 >> 0x17,0) & 1) << 2 |
                    (ushort)(SUB161(auVar57 >> 0x1f,0) & 1) << 3 |
                    (ushort)(SUB161(auVar57 >> 0x27,0) & 1) << 4 |
                    (ushort)(SUB161(auVar57 >> 0x2f,0) & 1) << 5 |
                    (ushort)(SUB161(auVar57 >> 0x37,0) & 1) << 6 |
                    (ushort)(SUB161(auVar57 >> 0x3f,0) & 1) << 7 |
                    (ushort)(SUB161(auVar57 >> 0x47,0) & 1) << 8 |
                    (ushort)(SUB161(auVar57 >> 0x4f,0) & 1) << 9 |
                    (ushort)(SUB161(auVar57 >> 0x57,0) & 1) << 10 |
                    (ushort)(SUB161(auVar57 >> 0x5f,0) & 1) << 0xb |
                    (ushort)(SUB161(auVar57 >> 0x67,0) & 1) << 0xc |
                    (ushort)(SUB161(auVar57 >> 0x6f,0) & 1) << 0xd |
                    (ushort)(SUB161(auVar57 >> 0x77,0) & 1) << 0xe |
                    (ushort)(byte)(auVar57[0xf] >> 7) << 0xf;
            ppppppuVar31 = ppppppuVar31 + -0xe0;
            pauVar21 = pauVar21 + 1;
          } while (uVar9 == 0xffff);
          pppppppuVar46[0x61] = (undefined8 ******)pauVar21;
          pppppppuVar46[99] = ppppppuVar31;
          uVar49 = ~(uint)uVar9;
          uVar50 = uVar49 & -(uint)uVar9 - 2;
        }
        else {
          uVar50 = uVar49 - 1 & uVar49;
        }
        uVar51 = 0;
        for (; (uVar49 & 1) == 0; uVar49 = uVar49 >> 1 | 0x80000000) {
          uVar51 = uVar51 + 1;
        }
        uVar26 = (ulonglong)uVar51;
        pppppppuVar46[100] = (undefined8 ******)(ulonglong)uVar50;
        ppppppuVar31 = pppppppuVar46[99];
        pauVar47 = (undefined1 (*) [16])ppppppuVar31[uVar26 * -0xe + -0xc];
        pppppuVar28 = ppppppuVar31[uVar26 * -0xe + -9];
        auVar57 = *pauVar47;
        uVar49 = ~(uint)(ushort)((ushort)(SUB161(auVar57 >> 7,0) & 1) |
                                 (ushort)(SUB161(auVar57 >> 0xf,0) & 1) << 1 |
                                 (ushort)(SUB161(auVar57 >> 0x17,0) & 1) << 2 |
                                 (ushort)(SUB161(auVar57 >> 0x1f,0) & 1) << 3 |
                                 (ushort)(SUB161(auVar57 >> 0x27,0) & 1) << 4 |
                                 (ushort)(SUB161(auVar57 >> 0x2f,0) & 1) << 5 |
                                 (ushort)(SUB161(auVar57 >> 0x37,0) & 1) << 6 |
                                 (ushort)(SUB161(auVar57 >> 0x3f,0) & 1) << 7 |
                                 (ushort)(SUB161(auVar57 >> 0x47,0) & 1) << 8 |
                                 (ushort)(SUB161(auVar57 >> 0x4f,0) & 1) << 9 |
                                 (ushort)(SUB161(auVar57 >> 0x57,0) & 1) << 10 |
                                 (ushort)(SUB161(auVar57 >> 0x5f,0) & 1) << 0xb |
                                 (ushort)(SUB161(auVar57 >> 0x67,0) & 1) << 0xc |
                                 (ushort)(SUB161(auVar57 >> 0x6f,0) & 1) << 0xd |
                                 (ushort)(SUB161(auVar57 >> 0x77,0) & 1) << 0xe |
                                (ushort)(byte)(auVar57[0xf] >> 7) << 0xf);
        pauVar21 = pauVar47 + 1;
        pauVar48 = (undefined1 (*) [16])ppppppuVar31[uVar26 * -0xe + -6];
        auVar57 = *pauVar48;
        uVar50 = ~(uint)(ushort)((ushort)(SUB161(auVar57 >> 7,0) & 1) |
                                 (ushort)(SUB161(auVar57 >> 0xf,0) & 1) << 1 |
                                 (ushort)(SUB161(auVar57 >> 0x17,0) & 1) << 2 |
                                 (ushort)(SUB161(auVar57 >> 0x1f,0) & 1) << 3 |
                                 (ushort)(SUB161(auVar57 >> 0x27,0) & 1) << 4 |
                                 (ushort)(SUB161(auVar57 >> 0x2f,0) & 1) << 5 |
                                 (ushort)(SUB161(auVar57 >> 0x37,0) & 1) << 6 |
                                 (ushort)(SUB161(auVar57 >> 0x3f,0) & 1) << 7 |
                                 (ushort)(SUB161(auVar57 >> 0x47,0) & 1) << 8 |
                                 (ushort)(SUB161(auVar57 >> 0x4f,0) & 1) << 9 |
                                 (ushort)(SUB161(auVar57 >> 0x57,0) & 1) << 10 |
                                 (ushort)(SUB161(auVar57 >> 0x5f,0) & 1) << 0xb |
                                 (ushort)(SUB161(auVar57 >> 0x67,0) & 1) << 0xc |
                                 (ushort)(SUB161(auVar57 >> 0x6f,0) & 1) << 0xd |
                                 (ushort)(SUB161(auVar57 >> 0x77,0) & 1) << 0xe |
                                (ushort)(byte)(auVar57[0xf] >> 7) << 0xf);
        pauVar22 = pauVar48 + 1;
        pppppuVar24 = ppppppuVar31[uVar26 * -0xe + -3];
        if (pauVar47 != (undefined1 (*) [16])0x0) goto code_r0x0001414729fb;
code_r0x000141472a4f:
        do {
          if ((pauVar48 == (undefined1 (*) [16])0x0) || (pppppuVar24 == (undefined8 *****)0x0))
          goto code_r0x000141472911;
          if ((short)uVar50 == 0) {
            do {
              auVar57 = *pauVar22;
              uVar9 = (ushort)(SUB161(auVar57 >> 7,0) & 1) |
                      (ushort)(SUB161(auVar57 >> 0xf,0) & 1) << 1 |
                      (ushort)(SUB161(auVar57 >> 0x17,0) & 1) << 2 |
                      (ushort)(SUB161(auVar57 >> 0x1f,0) & 1) << 3 |
                      (ushort)(SUB161(auVar57 >> 0x27,0) & 1) << 4 |
                      (ushort)(SUB161(auVar57 >> 0x2f,0) & 1) << 5 |
                      (ushort)(SUB161(auVar57 >> 0x37,0) & 1) << 6 |
                      (ushort)(SUB161(auVar57 >> 0x3f,0) & 1) << 7 |
                      (ushort)(SUB161(auVar57 >> 0x47,0) & 1) << 8 |
                      (ushort)(SUB161(auVar57 >> 0x4f,0) & 1) << 9 |
                      (ushort)(SUB161(auVar57 >> 0x57,0) & 1) << 10 |
                      (ushort)(SUB161(auVar57 >> 0x5f,0) & 1) << 0xb |
                      (ushort)(SUB161(auVar57 >> 0x67,0) & 1) << 0xc |
                      (ushort)(SUB161(auVar57 >> 0x6f,0) & 1) << 0xd |
                      (ushort)(SUB161(auVar57 >> 0x77,0) & 1) << 0xe |
                      (ushort)(byte)(auVar57[0xf] >> 7) << 0xf;
              pauVar48 = pauVar48 + -0x58;
              pauVar22 = pauVar22 + 1;
            } while (uVar9 == 0xffff);
            uVar50 = ~(uint)uVar9;
            uVar51 = uVar50 & -(uint)uVar9 - 2;
          }
          else {
            uVar51 = uVar50 - 1 & uVar50;
          }
          pppppuVar24 = (undefined8 *****)((longlong)pppppuVar24 + -1);
          pauVar47 = (undefined1 (*) [16])0x0;
          pauVar54 = pauVar48;
          uVar16 = uVar49;
          uVar32 = uVar50;
          while( true ) {
            uVar50 = uVar51;
            uVar49 = uVar16;
            uVar51 = 0;
            for (; (uVar32 & 1) == 0; uVar32 = uVar32 >> 1 | 0x80000000) {
              uVar51 = uVar51 + 1;
            }
            lVar33 = (ulonglong)uVar51 * -0x58;
            if (*(char *)((longlong)pauVar54 + lVar33 + -6) == ':') {
              *(undefined4 *)((longlong)pauVar54 + lVar33 + -0x28) = 999;
              *(int *)((longlong)pppppppuVar46 + 0x304) =
                   (int)CONCAT71((int7)((ulonglong)lVar33 >> 8),1);
            }
            if (pauVar47 == (undefined1 (*) [16])0x0) goto code_r0x000141472a4f;
code_r0x0001414729fb:
            if (pppppuVar28 == (undefined8 *****)0x0) break;
            uVar51 = uVar50;
            if ((short)uVar49 == 0) {
              do {
                auVar57 = *pauVar21;
                uVar9 = (ushort)(SUB161(auVar57 >> 7,0) & 1) |
                        (ushort)(SUB161(auVar57 >> 0xf,0) & 1) << 1 |
                        (ushort)(SUB161(auVar57 >> 0x17,0) & 1) << 2 |
                        (ushort)(SUB161(auVar57 >> 0x1f,0) & 1) << 3 |
                        (ushort)(SUB161(auVar57 >> 0x27,0) & 1) << 4 |
                        (ushort)(SUB161(auVar57 >> 0x2f,0) & 1) << 5 |
                        (ushort)(SUB161(auVar57 >> 0x37,0) & 1) << 6 |
                        (ushort)(SUB161(auVar57 >> 0x3f,0) & 1) << 7 |
                        (ushort)(SUB161(auVar57 >> 0x47,0) & 1) << 8 |
                        (ushort)(SUB161(auVar57 >> 0x4f,0) & 1) << 9 |
                        (ushort)(SUB161(auVar57 >> 0x57,0) & 1) << 10 |
                        (ushort)(SUB161(auVar57 >> 0x5f,0) & 1) << 0xb |
                        (ushort)(SUB161(auVar57 >> 0x67,0) & 1) << 0xc |
                        (ushort)(SUB161(auVar57 >> 0x6f,0) & 1) << 0xd |
                        (ushort)(SUB161(auVar57 >> 0x77,0) & 1) << 0xe |
                        (ushort)(byte)(auVar57[0xf] >> 7) << 0xf;
                pauVar47 = pauVar47 + -0x58;
                pauVar21 = pauVar21 + 1;
              } while (uVar9 == 0xffff);
              uVar32 = ~(uint)uVar9;
              pppppuVar28 = (undefined8 *****)((longlong)pppppuVar28 + -1);
              pauVar54 = pauVar47;
              uVar16 = uVar32 & -(uint)uVar9 - 2;
            }
            else {
              pppppuVar28 = (undefined8 *****)((longlong)pppppuVar28 + -1);
              pauVar54 = pauVar47;
              uVar16 = uVar49 - 1 & uVar49;
              uVar32 = uVar49;
            }
          }
          pppppuVar28 = (undefined8 *****)0x0;
        } while( true );
      }
    }
  }
  goto code_r0x000141471eae;
code_r0x000141472911:
  ppppuVar52 = (undefined8 ****)((longlong)ppppuVar52 + -1);
  if (ppppuVar52 == (undefined8 ****)0x0) goto code_r0x000141472abd;
  goto code_r0x00014147291a;
code_r0x000141472abd:
  unaff_R13D = *(uint *)(pppppppuVar46 + 0x66);
  in_R10 = (undefined8 ****)0xffffffffffffffff;
  ppppppuVar31 = pppppppuVar46[0x20];
  goto code_r0x000141471eae;
}


