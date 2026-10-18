
void __cdecl country_core::render::pass_builder::draw_stream::RasterPass::render(void)

{
  undefined8 *puVar1;
  undefined1 auVar2 [16];
  char cVar3;
  undefined8 uVar4;
  undefined1 (*pauVar5) [16];
  ulonglong uVar6;
  ushort uVar7;
  undefined **ppuVar8;
  char cVar9;
  char extraout_AL;
  byte bVar10;
  uint uVar11;
  undefined8 *in_RCX;
  ulonglong uVar12;
  uint uVar13;
  longlong lVar14;
  undefined8 *puVar15;
  byte bVar16;
  int iVar17;
  undefined8 local_5f0;
  undefined4 uStack_5e8;
  undefined4 uStack_5e4;
  undefined8 local_5e0;
  undefined **local_568;
  undefined8 uStack_560;
  undefined8 local_558;
  undefined1 auStack_550 [16];
  undefined8 uStack_540;
  undefined8 local_538;
  undefined8 uStack_530;
  undefined8 local_528;
  undefined8 uStack_520;
  undefined8 local_518;
  undefined8 uStack_510;
  undefined8 local_508;
  undefined8 uStack_500;
  undefined8 local_4f8;
  undefined8 uStack_4f0;
  int local_4e8;
  undefined4 uStack_4e4;
  undefined4 uStack_4e0;
  undefined4 uStack_4dc;
  undefined4 local_4d8;
  undefined4 uStack_4d4;
  undefined8 uStack_4d0;
  undefined8 local_4c8;
  undefined8 uStack_4c0;
  undefined8 local_4b8;
  undefined8 uStack_4b0;
  undefined8 local_4a8;
  undefined8 uStack_4a0;
  undefined8 local_498;
  undefined8 uStack_490;
  undefined8 local_488;
  undefined8 uStack_480;
  undefined8 local_478;
  undefined8 local_470;
  undefined8 uStack_468;
  undefined8 local_460;
  undefined8 local_458;
  undefined1 local_450 [488];
  undefined **local_268;
  undefined8 *puStack_260;
  undefined **local_250;
  undefined8 *puStack_248;
  undefined8 local_240;
  undefined4 local_238;
  char local_234;
  undefined2 local_233;
  undefined1 local_231 [25];
  longlong local_218;
  longlong local_1e8;
  uint local_1c0;
  undefined8 local_1bc;
  undefined8 uStack_1b4;
  char local_1ac;
  undefined2 local_1ab;
  undefined1 local_1a9;
  undefined **local_1a8;
  undefined8 *puStack_1a0;
  undefined8 local_198;
  undefined8 uStack_190;
  undefined8 local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  int local_a8;
  undefined **local_a4;
  undefined8 *puStack_9c;
  undefined1 local_94;
  undefined8 *local_88;
  undefined8 local_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 *local_58;
  undefined1 local_4a;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  *(undefined1 *)((longlong)in_RCX + 0x3e4) = 1;
  puVar15 = in_RCX + 0x40;
  uVar13 = *(uint *)(in_RCX + 0x7c);
  local_88 = puVar15;
  local_58 = in_RCX;
  if ((ulonglong)uVar13 == 0) {
    iVar17 = 0;
  }
  else {
    iVar17 = 0;
    lVar14 = 0;
    do {
      local_49 = 1;
      rhapsody::raw::shader::RenderPassAttachmentDesc::new();
      local_49 = 1;
      rhapsody::raw::shader::RenderPassAttachmentDesc::sample_count();
      local_128 = (undefined **)CONCAT44(*(undefined4 *)(puVar15 + 0xb),(undefined4)local_128);
      uStack_5e8 = (undefined4)uStack_120;
      local_5f0 = local_128;
      local_1a8._0_3_ = (undefined3)((ulonglong)uStack_120 >> 0x28);
      if (uStack_120._4_1_ == '\0') break;
      if (lVar14 == 0x80) {
        local_49 = 1;
                    // WARNING: Subroutine does not return
        arrayvec::arrayvec::extend_panic();
      }
      puVar15 = puVar15 + 0xc;
      *(undefined4 *)((longlong)&uStack_560 + lVar14) = (undefined4)uStack_120;
      *(undefined ***)((longlong)&local_568 + lVar14) = local_128;
      *(char *)((longlong)&uStack_560 + lVar14 + 4) = uStack_120._4_1_;
      *(undefined1 *)((longlong)&uStack_560 + lVar14 + 7) = uStack_120._7_1_;
      *(undefined2 *)((longlong)&uStack_560 + lVar14 + 5) = uStack_120._5_2_;
      lVar14 = lVar14 + 0x10;
      iVar17 = iVar17 + 1;
    } while (puVar15 != in_RCX + (ulonglong)uVar13 * 0xc + 0x40);
  }
  local_1a8 = local_568;
  puStack_1a0 = uStack_560;
  local_198 = local_558;
  uStack_190 = auStack_550._0_8_;
  local_188 = auStack_550._8_8_;
  uStack_180 = uStack_540;
  local_178 = local_538;
  uStack_170 = uStack_530;
  local_168 = local_528;
  uStack_160 = uStack_520;
  local_158 = local_518;
  uStack_150 = uStack_510;
  local_148 = local_508;
  uStack_140 = uStack_500;
  local_138 = local_4f8;
  uStack_130 = uStack_4f0;
  local_94 = 3;
  if (*(int *)(local_58 + 0x1c) != 2) {
    local_49 = 1;
    rhapsody::raw::shader::RenderPassAttachmentDesc::new();
    local_49 = 1;
    rhapsody::raw::shader::RenderPassAttachmentDesc::sample_count();
    if (*(int *)((longlong)local_58 + 0x11c) != 0) {
      local_568 = local_250;
      uStack_560 = puStack_248;
      local_49 = 1;
      rhapsody::raw::shader::RenderPassAttachmentDesc::clear_input();
    }
    local_94 = *(undefined1 *)((longlong)local_58 + 300);
    local_268 = local_250;
    puStack_260 = puStack_248;
  }
  local_128 = local_1a8;
  uStack_120 = puStack_1a0;
  local_118 = local_198;
  uStack_110 = uStack_190;
  local_108 = local_188;
  uStack_100 = uStack_180;
  local_f8 = local_178;
  uStack_f0 = uStack_170;
  local_e8 = local_168;
  uStack_e0 = uStack_160;
  local_d8 = local_158;
  uStack_d0 = uStack_150;
  local_c8 = local_148;
  uStack_c0 = uStack_140;
  local_b8 = local_138;
  uStack_b0 = uStack_130;
  local_a4 = local_268;
  puStack_9c = puStack_260;
  local_49 = 1;
  local_a8 = iVar17;
  hashbrown::map::HashMap::
  rustc_entry<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState,alloc::alloc::Global>
            ();
  puVar15 = uStack_560;
  ppuVar8 = local_568;
  cVar9 = local_4c8._4_1_;
  if (local_4c8._4_1_ == '\x04') {
    local_49 = 1;
    rhapsody::raw::managed::handle::impl$5::clone();
  }
  else {
    memcpy(&local_240,&local_558,0x94);
    local_1ab = local_4c8._5_2_;
    local_1a9 = local_4c8._7_1_;
    local_250 = ppuVar8;
    puStack_248 = puVar15;
    local_1ac = cVar9;
    local_80 = local_58[0x3c];
    if ((ulonglong)local_1c0 == 0) {
      iVar17 = 0;
    }
    else {
      lVar14 = 0;
      iVar17 = 0;
      puVar15 = &local_240;
      do {
        cVar3 = *(char *)((longlong)puVar15 + 0xc);
        if (cVar3 == '\0') break;
        if (lVar14 == 0x80) {
          local_4a = 1;
                    // WARNING: Subroutine does not return
          arrayvec::arrayvec::extend_panic();
        }
        uVar4 = *puVar15;
        puVar1 = puVar15 + 2;
        *(undefined4 *)((longlong)&uStack_560 + lVar14) = *(undefined4 *)(puVar15 + 1);
        *(undefined8 *)((longlong)&local_568 + lVar14) = uVar4;
        *(char *)((longlong)&uStack_560 + lVar14 + 4) = cVar3;
        *(undefined1 *)((longlong)&uStack_560 + lVar14 + 7) =
             *(undefined1 *)((longlong)puVar15 + 0xf);
        *(undefined2 *)((longlong)&uStack_560 + lVar14 + 5) =
             *(undefined2 *)((longlong)puVar15 + 0xd);
        lVar14 = lVar14 + 0x10;
        iVar17 = iVar17 + 1;
        puVar15 = puVar1;
      } while (puVar1 != &local_240 + (ulonglong)local_1c0 * 2);
    }
    local_4e8 = iVar17;
    memcpy(&local_5f0,&local_568,0x84);
    if (cVar9 != '\x03') {
      local_78 = local_1bc;
      uStack_70 = uStack_1b4;
    }
    memcpy(&local_568,&local_5f0,0x84);
    uStack_4e4 = (undefined4)local_78;
    uStack_4e0 = (undefined4)((ulonglong)local_78 >> 0x20);
    uStack_4dc = (undefined4)uStack_70;
    local_4d8 = (undefined4)((ulonglong)uStack_70 >> 0x20);
    uStack_4d4 = CONCAT31(uStack_4d4._1_3_,cVar9);
    local_4a = 1;
    rhapsody::raw::device::GraphicsDevice::create_render_pass();
    puVar15 = puStack_248;
    ppuVar8 = local_250;
    memcpy(&local_568,&local_240,0x98);
    uStack_4c0 = local_5e0;
    local_4c8 = CONCAT44(uStack_5e4,uStack_5e8);
    uStack_4d0 = local_5f0;
    pauVar5 = (undefined1 (*) [16])*puVar15;
    uVar6 = puVar15[1];
    uVar12 = uVar6 & (ulonglong)ppuVar8;
    auVar2 = *(undefined1 (*) [16])(*pauVar5 + uVar12);
    uVar7 = (ushort)(SUB161(auVar2 >> 7,0) & 1) | (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
            (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
            (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
            (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
            (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
            (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
            (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
            (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
            (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
            (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
            (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
            (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
            (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
            (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe | (ushort)(byte)(auVar2[0xf] >> 7) << 0xf;
    uVar13 = (uint)uVar7;
    if (uVar7 == 0) {
      lVar14 = 0x10;
      do {
        uVar12 = uVar12 + lVar14 & uVar6;
        auVar2 = *(undefined1 (*) [16])(*pauVar5 + uVar12);
        lVar14 = lVar14 + 0x10;
        uVar13 = (uint)(ushort)((ushort)(SUB161(auVar2 >> 7,0) & 1) |
                                (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                                (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                                (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                                (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                                (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                                (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                                (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                                (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                                (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                                (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                                (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                                (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                                (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                                (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                               (ushort)(byte)(auVar2[0xf] >> 7) << 0xf);
      } while (uVar13 == 0);
    }
    uVar11 = 0;
    for (; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
      uVar11 = uVar11 + 1;
    }
    uVar12 = uVar11 + uVar12 & uVar6;
    bVar10 = (*pauVar5)[uVar12];
    if (-1 < (char)bVar10) {
      auVar2 = *pauVar5;
      uVar13 = 0;
      for (uVar11 = (uint)(ushort)((ushort)(SUB161(auVar2 >> 7,0) & 1) |
                                   (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                                   (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                                   (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                                   (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                                   (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                                   (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                                   (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                                   (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                                   (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                                   (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                                   (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                                   (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                                   (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                                   (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                                  (ushort)(byte)(auVar2[0xf] >> 7) << 0xf); (uVar11 & 1) == 0;
          uVar11 = uVar11 >> 1 | 0x80000000) {
        uVar13 = uVar13 + 1;
      }
      uVar12 = (ulonglong)uVar13;
      bVar10 = (*pauVar5)[uVar12];
    }
    bVar16 = (byte)((ulonglong)ppuVar8 >> 0x39);
    (*pauVar5)[uVar12] = bVar16;
    pauVar5[1][uVar12 - 0x10 & uVar6] = bVar16;
    puVar15[2] = puVar15[2] - (ulonglong)(bVar10 & 1);
    memcpy(pauVar5 + uVar12 * -0xb + -0xb,&local_568,0xb0);
    puVar15[3] = puVar15[3] + 1;
    local_4a = 0;
    rhapsody::raw::managed::handle::impl$5::clone();
  }
  puVar1 = local_58;
  puVar15 = local_58 + 0x1c;
  memcpy(&local_250,local_58,0x90);
  local_e8 = puVar1[0x1a];
  uStack_e0 = puVar1[0x1b];
  local_128 = (undefined **)puVar1[0x12];
  uStack_120 = (undefined8 *)puVar1[0x13];
  local_118 = puVar1[0x14];
  uStack_110 = puVar1[0x15];
  local_108 = puVar1[0x16];
  uStack_100 = puVar1[0x17];
  local_f8 = puVar1[0x18];
  uStack_f0 = puVar1[0x19];
  local_478 = local_68;
  local_488 = local_78;
  uStack_480 = uStack_70;
  uStack_4d0 = puVar1[0x29];
  local_4c8 = puVar1[0x2a];
  uStack_4c0 = puVar1[0x2b];
  local_4b8 = puVar1[0x2c];
  uStack_4b0 = puVar1[0x2d];
  local_4e8 = (int)puVar1[0x26];
  uStack_4e4 = (undefined4)((ulonglong)puVar1[0x26] >> 0x20);
  uStack_4e0 = (undefined4)puVar1[0x27];
  uStack_4dc = (undefined4)((ulonglong)puVar1[0x27] >> 0x20);
  local_4d8 = (undefined4)puVar1[0x28];
  uStack_4d4 = (undefined4)((ulonglong)puVar1[0x28] >> 0x20);
  local_4a8 = puVar1[0x2e];
  uStack_4a0 = puVar1[0x2f];
  local_498 = puVar1[0x30];
  uStack_490 = puVar1[0x31];
  local_470 = puVar1[0x32];
  uStack_468 = puVar1[0x33];
  local_460 = puVar1[0x34];
  memcpy(local_450,local_88,0x1e4);
  local_538 = *puVar15;
  uStack_530 = puVar1[0x1d];
  local_528 = puVar1[0x1e];
  uStack_520 = puVar1[0x1f];
  local_518 = puVar1[0x20];
  uStack_510 = puVar1[0x21];
  local_508 = puVar1[0x22];
  uStack_500 = puVar1[0x23];
  local_4f8 = puVar1[0x24];
  uStack_4f0 = puVar1[0x25];
  local_458 = puVar1[0x3f];
  local_558 = puVar1[2];
  local_568 = (undefined **)*puVar1;
  uStack_560 = (undefined8 *)puVar1[1];
  auStack_550 = *(undefined1 (*) [16])(puVar1 + 3);
  uStack_540 = puVar1[5];
  rhapsody::rg::pass_builder::PassBuilder::
  render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::draw_stream::impl_11::render::closure_env_0>
            ();
  if ((local_218 != 0) && (local_218 * 0x11 != -0x21)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if ((local_1e8 != 0) &&
     (local_1e8 + ((local_1e8 + 1) * 0x18 + 0xfU & 0xfffffffffffffff0) != -0x11)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if ((*(char *)((longlong)local_58 + 0x3e4) == '\0') &&
     (((*(ulonglong *)__imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E &
       0x7fffffffffffffff) == 0 ||
      (std::panicking::panic_count::is_zero_slow_path(), extraout_AL != '\0')))) {
    local_568 = &anon_fd8aea26a9a0442e2a915fc8441cc25b_46_llvm_12095432060108768980;
    uStack_560 = (undefined8 *)0x1;
    local_558 = 8;
    auStack_550 = (undefined1  [16])0x0;
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  return;
}

