
void __cdecl country_core::game_config::impl$11::clone(void)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined1 uVar8;
  undefined1 uVar9;
  undefined1 uVar10;
  undefined1 uVar11;
  undefined4 uVar12;
  undefined4 uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  undefined8 uVar19;
  undefined8 uVar20;
  undefined8 uVar21;
  undefined8 extraout_RAX;
  void *in_RCX;
  longlong in_RDX;
  uint uVar22;
  bool bVar23;
  undefined4 unaff_XMM6_Da;
  undefined4 unaff_XMM6_Db;
  undefined4 unaff_XMM6_Dc;
  undefined4 unaff_XMM6_Dd;
  undefined4 uVar24;
  undefined8 local_1f8;
  undefined8 uStack_1f0;
  undefined8 local_1e8;
  undefined8 uStack_1e0;
  longlong local_1d8;
  undefined8 uStack_1d0;
  undefined8 local_1c8;
  undefined8 local_1c0;
  undefined8 uStack_1b8;
  undefined8 local_1b0;
  undefined8 uStack_1a8;
  longlong local_1a0;
  undefined8 local_198;
  undefined8 local_190;
  undefined4 local_188;
  undefined4 uStack_184;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  undefined8 local_178;
  uint local_170;
  undefined8 local_16c;
  undefined8 uStack_164;
  undefined1 local_15c;
  undefined1 local_15b;
  undefined4 local_158;
  undefined4 local_154;
  undefined4 local_150;
  undefined4 local_14c;
  undefined4 local_148;
  undefined4 local_144;
  undefined8 local_140;
  undefined8 uStack_138;
  undefined4 local_130;
  undefined1 local_12c;
  undefined8 local_12b;
  undefined8 uStack_123;
  undefined4 local_11b;
  undefined1 local_117;
  undefined1 local_116;
  undefined1 local_115;
  longlong local_110;
  undefined8 uStack_108;
  undefined8 local_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined8 local_d8;
  undefined8 local_d0;
  undefined4 local_c8;
  undefined4 local_c4;
  longlong local_c0;
  undefined4 local_b8;
  undefined1 local_b4;
  undefined1 local_b3;
  undefined1 local_b2;
  undefined1 local_b1;
  undefined8 local_b0;
  
  local_b0 = 0xfffffffffffffffe;
  bVar23 = *(int *)(in_RDX + 0x68) == 1;
  if (bVar23) {
    local_1f8 = *(undefined8 *)(in_RDX + 0x6c);
    uStack_1f0 = *(undefined8 *)(in_RDX + 0x74);
  }
  uVar22 = (uint)bVar23;
  uVar12 = *(undefined4 *)(in_RDX + 0x94);
  uVar8 = *(undefined1 *)(in_RDX + 0x7c);
  uVar9 = *(undefined1 *)(in_RDX + 0x7d);
  uVar14 = *(undefined8 *)(in_RDX + 0x60);
  local_c0 = *(longlong *)(in_RDX + 0x38);
  if (!SBORROW8(0,local_c0)) {
    if (local_c0 == -0x7fffffffffffffff) goto LAB_1405935dc;
    alloc::string::impl$6::clone();
    local_d0 = uStack_1d0;
    local_d8 = local_1c8;
    local_c0 = local_1d8;
  }
  unaff_XMM6_Da = *(undefined4 *)(in_RDX + 0x50);
  unaff_XMM6_Db = *(undefined4 *)(in_RDX + 0x54);
  unaff_XMM6_Dc = *(undefined4 *)(in_RDX + 0x58);
  unaff_XMM6_Dd = *(undefined4 *)(in_RDX + 0x5c);
LAB_1405935dc:
  local_c8 = *(undefined4 *)(in_RDX + 0x80);
  uVar1 = *(undefined4 *)(in_RDX + 0x84);
  local_c4 = *(undefined4 *)(in_RDX + 0x88);
  uVar2 = *(undefined4 *)(in_RDX + 0x8c);
  uVar24 = (undefined4)*(undefined8 *)(in_RDX + 0xa8);
  local_b4 = *(undefined1 *)(in_RDX + 0xc3);
  uVar16 = *(undefined8 *)(in_RDX + 0x98);
  uVar17 = *(undefined8 *)(in_RDX + 0xa0);
  alloc::vec::impl$11::
  clone<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>();
  if (*(char *)(in_RDX + 0xac) == '\x01') {
    local_1e8 = *(undefined8 *)(in_RDX + 0xad);
    uStack_1e0 = *(undefined8 *)(in_RDX + 0xb5);
    local_b8 = (undefined4)CONCAT71((int7)((ulonglong)extraout_RAX >> 8),1);
  }
  else {
    local_b8 = 0;
  }
  uVar3 = *(undefined4 *)(in_RDX + 0xbd);
  local_b2 = *(undefined1 *)(in_RDX + 0xc1);
  local_b1 = *(undefined1 *)(in_RDX + 0xc2);
  uVar13 = *(undefined4 *)(in_RDX + 0x30);
  local_b3 = uVar9;
  alloc::vec::impl$11::
  clone<enum2_<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>();
  local_1a0 = local_c0;
  local_198 = local_d0;
  local_190 = local_d8;
  local_16c = local_1f8;
  uStack_164 = uStack_1f0;
  local_15b = local_b3;
  local_158 = local_c8;
  local_150 = local_c4;
  local_115 = local_b4;
  local_1c8 = local_100;
  local_1d8 = local_110;
  uStack_1d0 = uStack_108;
  local_12c = (undefined1)local_b8;
  local_148 = *(undefined4 *)(in_RDX + 0x90);
  local_12b = local_1e8;
  uStack_123 = uStack_1e0;
  local_117 = local_b2;
  local_116 = local_b1;
  uStack_1a8 = CONCAT44(uStack_dc,uVar13);
  local_1c0 = local_f8;
  uStack_1b8 = uStack_f0;
  local_1b0 = local_e8;
  uVar18 = *(undefined8 *)(in_RDX + 0xf8);
  uVar19 = *(undefined8 *)(in_RDX + 0x100);
  uVar6 = *(undefined8 *)(in_RDX + 0xf0);
  uVar9 = *(undefined1 *)(in_RDX + 0xf4);
  uVar7 = *(undefined8 *)(in_RDX + 0xe8);
  uVar15 = *(undefined8 *)(in_RDX + 0xe0);
  uVar4 = *(undefined4 *)(in_RDX + 0x11c);
  uVar20 = *(undefined8 *)(in_RDX + 0x108);
  uVar21 = *(undefined8 *)(in_RDX + 0x110);
  uVar5 = *(undefined4 *)(in_RDX + 0x118);
  uVar10 = *(undefined1 *)(in_RDX + 0x120);
  uVar11 = *(undefined1 *)(in_RDX + 0x121);
  local_188 = unaff_XMM6_Da;
  uStack_184 = unaff_XMM6_Db;
  uStack_180 = unaff_XMM6_Dc;
  uStack_17c = unaff_XMM6_Dd;
  local_178 = uVar14;
  local_170 = uVar22;
  local_15c = uVar8;
  local_154 = uVar1;
  local_14c = uVar2;
  local_144 = uVar12;
  local_140 = uVar16;
  uStack_138 = uVar17;
  local_130 = uVar24;
  local_11b = uVar3;
  uStack_e0 = uVar13;
  if (SBORROW8(0,*(longlong *)(in_RDX + 200))) {
    local_f8 = 0x8000000000000000;
  }
  else {
    alloc::string::impl$6::clone();
  }
  memcpy(in_RCX,&local_1d8,200);
  *(undefined8 *)((longlong)in_RCX + 0xf8) = uVar18;
  *(undefined8 *)((longlong)in_RCX + 0x100) = uVar19;
  *(undefined8 *)((longlong)in_RCX + 0xe8) = uVar7;
  *(int *)((longlong)in_RCX + 0xf0) = (int)uVar6;
  *(undefined1 *)((longlong)in_RCX + 0xf4) = uVar9;
  *(undefined8 *)((longlong)in_RCX + 0xe0) = uVar15;
  *(undefined8 *)((longlong)in_RCX + 0x108) = uVar20;
  *(undefined8 *)((longlong)in_RCX + 0x110) = uVar21;
  *(undefined4 *)((longlong)in_RCX + 0x118) = uVar5;
  *(undefined4 *)((longlong)in_RCX + 0x11c) = uVar4;
  *(undefined1 *)((longlong)in_RCX + 0x120) = uVar10;
  *(undefined1 *)((longlong)in_RCX + 0x121) = uVar11;
  *(undefined8 *)((longlong)in_RCX + 200) = local_f8;
  *(undefined8 *)((longlong)in_RCX + 0xd0) = uStack_f0;
  *(undefined8 *)((longlong)in_RCX + 0xd8) = local_e8;
  return;
}

