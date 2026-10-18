
// WARNING: Function: __chkstk replaced with injection: alloca_probe
// WARNING: Removing unreachable block (ram,0x000140af4616)

void __cdecl country_core::systems::backend::prepare_frame_constants::prepare_frame_constants(void)

{
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  undefined8 uVar16;
  undefined1 auVar17 [16];
  undefined4 uVar18;
  int iVar19;
  longlong lVar20;
  longlong lVar21;
  uint *puVar22;
  undefined4 *puVar23;
  ulonglong *puVar24;
  float *pfVar25;
  uint uVar26;
  uint uVar27;
  double dVar28;
  bool bVar29;
  void *_Src;
  float *pfVar30;
  undefined4 *extraout_RAX;
  longlong lVar31;
  ulonglong extraout_RAX_00;
  longlong *in_RCX;
  longlong lVar32;
  longlong *in_RDX;
  void *pvVar33;
  ulonglong uVar34;
  longlong *in_R8;
  ulonglong uVar35;
  undefined8 *in_R9;
  ulonglong uVar36;
  longlong lVar37;
  uint uVar38;
  ushort uVar39;
  ulonglong uVar40;
  ulonglong uVar41;
  float fVar42;
  float fVar43;
  char cVar44;
  byte bVar45;
  float fVar46;
  float fVar47;
  char cVar51;
  char cVar52;
  char cVar53;
  undefined8 uVar48;
  double dVar49;
  undefined1 auVar50 [16];
  float fVar54;
  undefined1 auVar55 [16];
  double dVar56;
  undefined1 auVar57 [16];
  float fVar58;
  undefined4 uVar59;
  float fVar60;
  float fVar61;
  undefined4 uVar62;
  undefined1 local_31e8 [800];
  undefined1 local_2ec8 [800];
  undefined1 local_2ba8 [800];
  undefined1 local_2888 [800];
  undefined8 local_2568;
  undefined8 uStack_2560;
  undefined8 local_2558;
  undefined8 uStack_2550;
  undefined8 local_2548;
  undefined8 uStack_2540;
  undefined8 local_2538;
  undefined8 uStack_2530;
  undefined8 local_24e8;
  undefined8 uStack_24e0;
  undefined8 local_24d8;
  undefined8 uStack_24d0;
  undefined8 local_24c8;
  undefined8 uStack_24c0;
  undefined8 local_24b8;
  undefined8 uStack_24b0;
  float local_15c8;
  int local_15c4;
  undefined1 local_14c8 [256];
  undefined8 local_13c8;
  undefined1 local_13c0 [16];
  undefined1 local_12a8 [800];
  undefined1 local_f88 [800];
  undefined1 local_c68 [800];
  undefined1 local_948 [800];
  undefined1 local_628 [16];
  undefined8 local_618;
  undefined8 uStack_610;
  undefined8 local_608;
  undefined8 uStack_600;
  undefined8 local_5f8;
  undefined8 uStack_5f0;
  undefined1 local_308 [16];
  undefined8 local_2f8;
  undefined8 uStack_2f0;
  undefined8 local_2e8;
  undefined8 uStack_2e0;
  undefined8 local_2d8;
  undefined8 uStack_2d0;
  undefined8 local_288;
  undefined8 uStack_280;
  undefined8 local_278;
  undefined8 uStack_270;
  undefined8 local_268;
  undefined8 uStack_260;
  undefined8 local_258;
  undefined8 uStack_250;
  undefined1 local_208 [16];
  undefined8 local_1f8;
  undefined8 uStack_1f0;
  undefined8 local_1e8;
  undefined8 uStack_1e0;
  undefined8 local_1d8;
  undefined8 uStack_1d0;
  longlong local_1c0;
  undefined8 local_1b0;
  undefined8 local_1a8;
  undefined8 uStack_1a0;
  undefined1 local_198;
  longlong *local_190;
  longlong local_188;
  ulonglong local_180;
  undefined1 local_178 [16];
  float *local_160;
  longlong *local_158;
  undefined8 uStack_150;
  uint local_148;
  int local_144;
  undefined8 *local_140;
  longlong local_138;
  undefined8 *local_130;
  longlong *local_128;
  void *local_120;
  void *local_118;
  longlong local_110;
  float local_104;
  undefined8 *local_100;
  char local_f1;
  undefined8 local_f0;
  undefined8 uStack_48;
  
  uStack_48 = 0x140af40c6;
  local_f0 = 0xfffffffffffffffe;
  lVar20 = in_RCX[4];
  local_160 = (float *)in_RCX[8];
  local_118 = (void *)in_RCX[0xc];
  local_138 = in_RCX[0x10];
  local_110 = in_RCX[0x14];
  lVar21 = in_RCX[0x18];
  puVar22 = (uint *)in_RCX[0x1c];
  local_130 = (undefined8 *)in_RCX[0x20];
  lVar37 = (ulonglong)*(byte *)(lVar20 + 0xc0) * 0x40;
  local_100 = (undefined8 *)in_RCX[0x24];
  local_128 = (longlong *)in_RCX[0x28];
  local_158 = (longlong *)in_RCX[0x2c];
  local_178._0_8_ = in_RCX[0x30];
  local_188 = in_RCX[0x34];
  local_190 = in_R8;
  local_140 = in_R9;
  if (*(char *)(lVar20 + 0x28 + lVar37) == '\0') {
    lVar37 = lVar37 + lVar20;
    alloc::string::impl$6::clone();
    local_1a8 = *(undefined8 *)(lVar37 + 0x18);
    uStack_1a0 = *(undefined8 *)(lVar37 + 0x20);
    uVar38 = *puVar22;
    pvVar33 = (void *)*in_RDX;
    _Src = local_118;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (undefined4 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    *(undefined4 *)((longlong)extraout_RAX + 3) = 0x746c7561;
    *extraout_RAX = 0x61666564;
    local_1c0 = 7;
    local_1b0 = 7;
    local_1a8 = 0x3f800000;
    uStack_1a0 = 0x3f8000003f800000;
    uVar38 = *puVar22;
    pvVar33 = (void *)*in_RDX;
    _Src = local_118;
  }
  if (1 < uVar38) {
    uVar38 = puVar22[1];
    *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
    *(uint *)((longlong)pvVar33 + 0x108) = uVar38 & 3;
  }
  local_198 = 0;
  local_120 = pvVar33;
  local_118 = _Src;
  if (USE_TAA_JITTER == '\0') {
    uVar38 = *(uint *)((longlong)pvVar33 + 0x108);
    uVar62 = 0;
    uVar59 = 0;
  }
  else {
    uVar35 = *(ulonglong *)(lVar21 + 0x10);
    if (uVar35 == 0) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_const::panic_const_rem_by_zero();
    }
    uVar38 = *(uint *)((longlong)pvVar33 + 0x108);
    uVar41 = (ulonglong)uVar38;
    if (uVar35 <= uVar41) {
      uVar41 = uVar41 % (uVar35 & 0xffffffff);
    }
    uVar9 = *(undefined8 *)(*(longlong *)(lVar21 + 8) + uVar41 * 8);
    uVar62 = (undefined4)uVar9;
    uVar59 = (undefined4)((ulonglong)uVar9 >> 0x20);
  }
  memcpy(&local_15c8,_Src,0x100);
  if (uVar38 == 0) {
    pvVar33 = _Src;
  }
  memcpy(local_14c8,pvVar33,0x100);
  local_13c8 = *(undefined8 *)(local_110 + 0x10);
  local_13c0 = (undefined1  [16])0x0;
  country_core::render::view_constants::ViewportConstantBuilder::build();
  memcpy(&local_15c8,local_118,0x100);
  pvVar33 = local_120;
  uVar18 = *(undefined4 *)((longlong)in_RDX + 0x1c);
  puVar23 = (undefined4 *)in_RDX[2];
  *puVar23 = uVar18;
  memcpy(local_120,&local_15c8,0x100);
  *puVar23 = uVar18;
  *(ulonglong *)((longlong)pvVar33 + 0x100) = CONCAT44(uVar59,uVar62);
  memcpy(local_2ba8,local_2888,800);
  country_core::render::view_constants::ViewConstants::set_pixel_offset();
  lVar37 = (ulonglong)*(byte *)(lVar20 + 0xc0) * 0x40;
  local_104 = (float)CONCAT31(local_104._1_3_,*(undefined1 *)(lVar20 + 0x3d + lVar37));
  puVar24 = (ulonglong *)*local_140;
  lVar21 = local_140[1];
  uVar35 = *puVar24;
  uVar36 = 0;
  uVar41 = uVar35 - *(ulonglong *)(lVar21 + 0x18);
  if (uVar35 < *(ulonglong *)(lVar21 + 0x18)) {
    uVar41 = uVar36;
  }
  uVar40 = uVar35 - *(ulonglong *)(lVar21 + 0x38);
  if (uVar35 < *(ulonglong *)(lVar21 + 0x38)) {
    uVar40 = uVar36;
  }
  lVar31 = uVar41 * 0x18 + *(longlong *)(lVar21 + 8);
  uVar35 = *(ulonglong *)(lVar21 + 0x10) - uVar41;
  if (*(ulonglong *)(lVar21 + 0x10) < uVar41) {
    lVar31 = 8;
    uVar35 = uVar36;
  }
  lVar32 = uVar40 * 0x18 + *(longlong *)(lVar21 + 0x28);
  uVar41 = *(ulonglong *)(lVar21 + 0x30);
  local_180 = (ulonglong)*(byte *)(lVar20 + 0x3c + lVar37);
  local_100 = (undefined8 *)*in_RCX;
  fVar61 = *(float *)(local_100 + 0xb);
  uVar9 = *(undefined8 *)((longlong)local_100 + 0x5c);
  local_144 = (int)*local_128;
  local_f1 = *(char *)((longlong)local_118 + 0x4d0);
  uVar34 = uVar41 - uVar40;
  if (uVar41 < uVar40) {
    uVar34 = uVar36;
  }
  local_140 = (undefined8 *)CONCAT44(local_140._4_4_,*(undefined4 *)(local_110 + 8));
  uVar48 = *local_130;
  uVar62 = *(undefined4 *)(local_130 + 1);
  local_148 = *(uint *)(local_110 + 0x10);
  if (uVar41 < uVar40) {
    lVar32 = 8;
  }
  *puVar24 = (*(longlong *)(lVar21 + 0x40) - uVar34) - uVar35;
  lVar20 = lVar31 + uVar35 * 0x18;
  local_130 = (undefined8 *)(lVar32 + uVar34 * 0x18);
  local_128 = local_158 + 4;
  do {
    do {
      if ((lVar31 == 0) || (lVar31 == lVar20)) {
        fVar42 = 0.0;
        if ((lVar32 == 0) || ((undefined8 *)lVar32 == local_130)) goto LAB_140af46d6;
        *puVar24 = *puVar24 + 1;
        cVar44 = *(char *)(lVar32 + 0x14);
        lVar37 = 0;
        lVar21 = lVar32;
        lVar32 = lVar32 + 0x18;
      }
      else {
        *puVar24 = *puVar24 + 1;
        cVar44 = *(char *)(lVar31 + 0x14);
        lVar37 = lVar31 + 0x18;
        lVar21 = lVar31;
      }
      lVar31 = lVar37;
    } while (cVar44 != '\x03');
    fVar42 = *(float *)(lVar21 + 8);
    iVar19 = *(int *)(lVar21 + 0xc);
    bVar29 = false;
    local_15c8 = fVar42;
    local_15c4 = iVar19;
    if (local_158[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::resources::stairs::stairs_state::StairGraphEdge>_>
                ();
      lVar21 = *local_158;
      bVar45 = (byte)(extraout_RAX_00 >> 0x39);
      auVar50 = ZEXT216(CONCAT11(bVar45,bVar45));
      auVar50 = pshuflw(auVar50,auVar50,0);
      lVar37 = 0;
      uVar35 = extraout_RAX_00;
      while( true ) {
        uVar35 = local_158[1] & uVar35;
        auVar17 = *(undefined1 (*) [16])(lVar21 + uVar35);
        cVar44 = auVar50[0];
        auVar57[0] = -(auVar17[0] == cVar44);
        cVar51 = auVar50[1];
        auVar57[1] = -(auVar17[1] == cVar51);
        cVar52 = auVar50[2];
        auVar57[2] = -(auVar17[2] == cVar52);
        cVar53 = auVar50[3];
        auVar57[3] = -(auVar17[3] == cVar53);
        auVar57[4] = -(auVar17[4] == cVar44);
        auVar57[5] = -(auVar17[5] == cVar51);
        auVar57[6] = -(auVar17[6] == cVar52);
        auVar57[7] = -(auVar17[7] == cVar53);
        auVar57[8] = -(auVar17[8] == cVar44);
        auVar57[9] = -(auVar17[9] == cVar51);
        auVar57[10] = -(auVar17[10] == cVar52);
        auVar57[0xb] = -(auVar17[0xb] == cVar53);
        auVar57[0xc] = -(auVar17[0xc] == cVar44);
        auVar57[0xd] = -(auVar17[0xd] == cVar51);
        auVar57[0xe] = -(auVar17[0xe] == cVar52);
        auVar57[0xf] = -(auVar17[0xf] == cVar53);
        uVar39 = (ushort)(SUB161(auVar57 >> 7,0) & 1) | (ushort)(SUB161(auVar57 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar57 >> 0x17,0) & 1) << 2 |
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
                 (ushort)(SUB161(auVar57 >> 0x77,0) & 1) << 0xe | (ushort)(auVar57[0xf] >> 7) << 0xf
        ;
        uVar38 = (uint)uVar39;
        while (uVar39 != 0) {
          uVar26 = 0;
          for (uVar27 = uVar38; (uVar27 & 1) == 0; uVar27 = uVar27 >> 1 | 0x80000000) {
            uVar26 = uVar26 + 1;
          }
          uVar41 = uVar26 + uVar35 & local_158[1];
          if ((*(float *)(lVar21 + -0xc + uVar41 * -0xc) == fVar42) &&
             (*(int *)(lVar21 + -8 + uVar41 * -0xc) == iVar19)) {
            fVar42 = *(float *)(lVar21 + -4 + uVar41 * -0xc);
            bVar29 = true;
            goto LAB_140af46c0;
          }
          uVar39 = (ushort)(uVar38 - 1) & (ushort)uVar38;
          uVar38 = CONCAT22((short)(uVar38 - 1 >> 0x10),uVar39);
        }
        auVar55[0] = -(auVar17[0] == -1);
        auVar55[1] = -(auVar17[1] == -1);
        auVar55[2] = -(auVar17[2] == -1);
        auVar55[3] = -(auVar17[3] == -1);
        auVar55[4] = -(auVar17[4] == -1);
        auVar55[5] = -(auVar17[5] == -1);
        auVar55[6] = -(auVar17[6] == -1);
        auVar55[7] = -(auVar17[7] == -1);
        auVar55[8] = -(auVar17[8] == -1);
        auVar55[9] = -(auVar17[9] == -1);
        auVar55[10] = -(auVar17[10] == -1);
        auVar55[0xb] = -(auVar17[0xb] == -1);
        auVar55[0xc] = -(auVar17[0xc] == -1);
        auVar55[0xd] = -(auVar17[0xd] == -1);
        auVar55[0xe] = -(auVar17[0xe] == -1);
        auVar55[0xf] = -(auVar17[0xf] == -1);
        if ((((((((((((((((SUB161(auVar55 >> 7,0) & 1) != 0 || (SUB161(auVar55 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar55 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar55 >> 0x1f,0) & 1) != 0) || (SUB161(auVar55 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar55 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar55 >> 0x37,0) & 1) != 0) || (SUB161(auVar55 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar55 >> 0x47,0) & 1) != 0) || (SUB161(auVar55 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar55 >> 0x57,0) & 1) != 0) || (SUB161(auVar55 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar55 >> 0x67,0) & 1) != 0) || (SUB161(auVar55 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar55 >> 0x77,0) & 1) != 0) || auVar55[0xf] < '\0') break;
        uVar35 = uVar35 + lVar37 + 0x10;
        lVar37 = lVar37 + 0x10;
      }
    }
LAB_140af46c0:
    uVar62 = 0;
    uVar48 = 0x3cf5c2903dcccccd;
  } while (!bVar29);
LAB_140af46d6:
  uStack_150 = 0;
  local_158 = (longlong *)uVar48;
  memcpy(local_2ec8,local_2888,800);
  local_628._0_12_ = ZEXT812(0x3c7c0fc1);
  local_628._12_4_ = 0;
  local_618 = 0x3c7c0fc100000000;
  uStack_610 = 0;
  local_608 = 0;
  uStack_600 = 0xba800000;
  local_5f8 = 0x8000000080000000;
  uStack_5f0 = 0x3f8000003f000000;
  glam::f32::sse2::mat4::Mat4::inverse();
  local_24e8 = *local_100;
  uStack_24e0 = local_100[1];
  local_24d8 = local_100[2];
  uStack_24d0 = local_100[3];
  local_24c8 = local_100[4];
  uStack_24c0 = local_100[5];
  local_24b8 = local_100[6];
  uStack_24b0 = local_100[7];
  glam::f32::sse2::mat4::Mat4::inverse();
  local_2538 = local_5f8;
  uStack_2530 = uStack_5f0;
  local_2548 = local_608;
  uStack_2540 = uStack_600;
  local_2558 = local_618;
  uStack_2550 = uStack_610;
  local_2568 = local_628._0_8_;
  uStack_2560 = local_628._8_8_;
  memcpy(&local_15c8,&local_2568,0x100);
  local_13c8 = 0x4b0000004b0;
  memcpy(local_14c8,&local_2568,0x100);
  local_13c0 = (undefined1  [16])0x0;
  country_core::render::view_constants::ViewportConstantBuilder::build();
  local_308._0_12_ = ZEXT812(0x3bfc0fc1);
  local_308._12_4_ = 0;
  local_2f8 = 0x3bfc0fc100000000;
  uStack_2f0 = 0;
  local_2e8 = 0;
  uStack_2e0 = 0xba800000;
  local_2d8 = 0x8000000080000000;
  uStack_2d0 = 0x3f8000003f000000;
  glam::f32::sse2::mat4::Mat4::inverse();
  local_24e8 = *local_100;
  uStack_24e0 = local_100[1];
  local_24d8 = local_100[2];
  uStack_24d0 = local_100[3];
  local_24c8 = local_100[4];
  uStack_24c0 = local_100[5];
  local_24b8 = local_100[6];
  uStack_24b0 = local_100[7];
  glam::f32::sse2::mat4::Mat4::inverse();
  local_2538 = local_2d8;
  uStack_2530 = uStack_2d0;
  local_2548 = local_2e8;
  uStack_2540 = uStack_2e0;
  local_2558 = local_2f8;
  uStack_2550 = uStack_2f0;
  local_2568 = local_308._0_8_;
  uStack_2560 = local_308._8_8_;
  memcpy(&local_15c8,&local_2568,0x100);
  local_13c8 = 0x4b0000004b0;
  memcpy(local_14c8,&local_2568,0x100);
  local_13c0 = (undefined1  [16])0x0;
  country_core::render::view_constants::ViewportConstantBuilder::build();
  local_208._0_12_ = ZEXT812(0x3b100901);
  local_208._12_4_ = 0;
  local_1f8 = 0x3b10090100000000;
  uStack_1f0 = 0;
  local_1e8 = 0;
  uStack_1e0 = 0xba800000;
  local_1d8 = 0x8000000080000000;
  uStack_1d0 = 0x3f8000003f000000;
  glam::f32::sse2::mat4::Mat4::inverse();
  local_288 = *local_100;
  uStack_280 = local_100[1];
  local_278 = local_100[2];
  uStack_270 = local_100[3];
  local_268 = local_100[4];
  uStack_260 = local_100[5];
  local_258 = local_100[6];
  uStack_250 = local_100[7];
  glam::f32::sse2::mat4::Mat4::inverse();
  local_2d8 = local_1d8;
  uStack_2d0 = uStack_1d0;
  local_2e8 = local_1e8;
  uStack_2e0 = uStack_1e0;
  local_2f8 = local_1f8;
  uStack_2f0 = uStack_1f0;
  local_308 = local_208;
  memcpy(&local_15c8,local_308,0x100);
  local_13c8 = 0x4b0000004b0;
  memcpy(local_14c8,local_308,0x100);
  local_13c0 = (undefined1  [16])0x0;
  country_core::render::view_constants::ViewportConstantBuilder::build();
  memcpy(&local_15c8,local_2ec8,800);
  memcpy(local_12a8,local_2ba8,800);
  memcpy(local_f88,local_31e8,800);
  memcpy(local_c68,local_628,800);
  memcpy(local_948,&local_2568,800);
  pfVar30 = local_160;
  local_100 = (undefined8 *)CONCAT44(local_100._4_4_,*(undefined4 *)((longlong)local_120 + 0x108));
  uVar48 = *(undefined8 *)(local_160 + 7);
  uVar10 = *(undefined8 *)(local_160 + 0xd);
  auVar50 = *(undefined1 (*) [16])(local_160 + 0x15);
  local_120 = (void *)CONCAT44(local_120._4_4_,*(undefined4 *)(local_138 + 0x28));
  local_110 = *(undefined8 *)(local_110 + 0x10);
  dVar56 = *(double *)(local_138 + 0x10);
  local_128 = (longlong *)CONCAT44(local_128._4_4_,(uint)*(byte *)((longlong)local_118 + 0x4d0));
  local_138 = CONCAT44(local_138._4_4_,
                       *(undefined4 *)
                        (&DAT_142ae4fc0 + (ulonglong)*(byte *)(local_178._0_8_ + 0x58) * 4));
  fVar1 = (float)local_180;
  fVar43 = fVar1;
  if (local_104._0_1_ != '\0') {
    fVar43 = (float)((int)fVar1 + 2);
  }
  local_118 = (void *)CONCAT44(local_118._4_4_,local_160[0x1e]);
  if (local_f1 == '\x03') {
    fVar43 = fVar1;
  }
  if (local_144 != 0x13) {
    fVar43 = (float)((int)fVar1 + 2);
  }
  local_104 = (float)exp2f();
  local_178._12_4_ = auVar50._0_4_;
  local_178._0_12_ = auVar50._4_12_;
  fVar1 = pfVar30[0xc];
  local_130 = (undefined8 *)CONCAT44(local_130._4_4_,uVar62);
  fVar46 = (float)exp2f();
  pfVar30 = local_160;
  uVar11 = *(undefined8 *)(local_160 + 10);
  fVar2 = local_160[9];
  fVar3 = local_160[6];
  uVar12 = *(undefined8 *)(local_160 + 1);
  uVar13 = *(undefined8 *)(local_160 + 4);
  fVar4 = local_160[3];
  fVar61 = fVar61 * fVar46;
  fVar5 = local_160[0x12];
  fVar6 = local_160[0xf];
  fVar58 = (float)uVar9 * fVar46;
  fVar60 = (float)((ulonglong)uVar9 >> 0x20) * fVar46;
  uVar9 = *(undefined8 *)(local_160 + 0x13);
  uVar14 = *(undefined8 *)(local_160 + 0x10);
  fVar7 = *local_160;
  fVar54 = (float)((ulonglong)local_140 & 0xffffffff);
  if (fVar54 <= 1.0) {
    fVar54 = 1.0;
  }
  fVar47 = (float)log2f((float)local_148 / fVar54);
  fVar54 = pfVar30[0x19];
  fVar8 = *(float *)(local_188 + 8);
  uVar15 = *(undefined8 *)(pfVar30 + 0x1a);
  memcpy(&local_2568,&local_15c8,4000);
  uVar62 = *(undefined4 *)((longlong)local_190 + 0x1c);
  pfVar25 = (float *)*local_190;
  puVar23 = (undefined4 *)local_190[2];
  dVar49 = fmod(dVar56,1.0);
  dVar28 = 0.0;
  if (0.0 <= dVar56) {
    dVar28 = dVar56;
  }
  dVar56 = 4294967295.0;
  if (dVar28 <= 4294967295.0) {
    dVar56 = dVar28;
  }
  uVar16 = *(undefined8 *)(pfVar30 + 0x1c);
  *puVar23 = uVar62;
  *pfVar25 = fVar61 * fVar7 * 30.0;
  pfVar25[1] = fVar58 * (float)uVar12 * 30.0;
  pfVar25[2] = fVar60 * (float)((ulonglong)uVar12 >> 0x20) * 30.0;
  pfVar25[3] = 1.0;
  pfVar25[4] = fVar6 * fVar61 * 8.16;
  pfVar25[5] = (float)uVar14 * fVar58 * 8.16;
  pfVar25[6] = (float)((ulonglong)uVar14 >> 0x20) * fVar60 * 8.16;
  pfVar25[7] = 1.0;
  pfVar25[8] = fVar5 * fVar61 * 2.72;
  pfVar25[9] = (float)uVar9 * fVar58 * 2.72;
  pfVar25[10] = (float)((ulonglong)uVar9 >> 0x20) * fVar60 * 2.72;
  pfVar25[0xb] = 1.0;
  *(undefined8 *)(pfVar25 + 0xc) = local_178._0_8_;
  *(undefined8 *)(pfVar25 + 0xe) = local_178._8_8_;
  pfVar25[0x10] = fVar4 * 30.0 * fVar46;
  pfVar25[0x11] = (float)uVar13 * 30.0 * fVar46;
  pfVar25[0x12] = (float)((ulonglong)uVar13 >> 0x20) * 30.0 * fVar46;
  pfVar25[0x13] = 1.0;
  auVar50._4_4_ = (int)uVar48;
  auVar50._0_4_ = fVar3;
  auVar50._8_4_ = (int)((ulonglong)uVar48 >> 0x20);
  auVar50._12_4_ = 0;
  *(undefined1 (*) [16])(pfVar25 + 0x14) = auVar50;
  pfVar25[0x18] = fVar2 * 30.0 * fVar46;
  pfVar25[0x19] = (float)uVar11 * 30.0 * fVar46;
  pfVar25[0x1a] = (float)((ulonglong)uVar11 >> 0x20) * 30.0 * fVar46;
  pfVar25[0x1b] = 1.0;
  auVar17._4_4_ = (int)uVar10;
  auVar17._0_4_ = fVar1;
  auVar17._8_4_ = (int)((ulonglong)uVar10 >> 0x20);
  auVar17._12_4_ = 0;
  *(undefined1 (*) [16])(pfVar25 + 0x1c) = auVar17;
  pfVar25[0x20] = local_100._0_4_;
  pfVar25[0x21] = local_120._0_4_;
  pfVar25[0x22] = local_104;
  pfVar25[0x23] = (float)local_138;
  *(longlong *)(pfVar25 + 0x24) = local_110;
  pfVar25[0x26] = (float)(longlong)dVar56;
  pfVar25[0x27] = (float)dVar49;
  *(longlong **)(pfVar25 + 0x28) = local_158;
  pfVar25[0x2a] = local_130._0_4_;
  pfVar25[0x2b] = fVar8;
  pfVar25[0x2c] = fVar54;
  pfVar25[0x2d] = (float)uVar15;
  pfVar25[0x2e] = (float)((ulonglong)uVar15 >> 0x20);
  pfVar25[0x2f] = local_128._0_4_;
  *(undefined8 *)(pfVar25 + 0x30) = uVar16;
  pfVar25[0x32] = local_118._0_4_;
  pfVar25[0x33] = fVar43;
  pfVar25[0x34] = fVar47;
  pfVar25[0x35] = fVar42;
  pfVar25[0x36] = 1.4013e-45;
  pfVar25[0x37] = 2.8026e-45;
  *(undefined1 (*) [16])(pfVar25 + 0x38) = ZEXT416(0);
  memcpy(pfVar25 + 0x3c,&local_2568,4000);
  if (local_1c0 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

