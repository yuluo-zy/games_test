// addr=0x1408bf9c0 name=load_glade_grading
// signature=void __cdecl load_glade_grading(void)


void __cdecl country_core::startup::load_glade_grading::load_glade_grading(void)

{
  longlong lVar1;
  char *pcVar2;
  longlong *plVar3;
  undefined1 uVar4;
  undefined1 uVar5;
  undefined1 uVar6;
  longlong lVar7;
  undefined8 *puVar8;
  code *pcVar9;
  longlong lVar10;
  longlong lVar11;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined4 *extraout_RAX_02;
  undefined8 *in_RCX;
  longlong *in_RDX;
  longlong *plVar12;
  undefined4 uVar13;
  longlong lVar14;
  undefined4 *puVar15;
  longlong lVar16;
  longlong local_128;
  longlong local_120;
  longlong local_110;
  longlong local_e8;
  longlong lStack_d0;
  longlong lStack_c8;
  undefined1 local_c0;
  longlong local_b0;
  longlong lStack_a8;
  longlong lStack_a0;
  longlong local_98;
  longlong lStack_90;
  undefined1 local_88;
  undefined4 local_87;
  undefined2 local_83;
  undefined1 local_81;
  longlong local_78;
  
  lVar7 = *in_RDX;
  lVar16 = 0;
  do {
    if (*(longlong *)(lVar7 + 0x10) == lVar16) {
      std::path::Path::_join();
      std::sys::os_str::wtf8::Buf::as_mut_slice();
      std::path::Path::_join();
      std::sys::os_str::wtf8::Buf::as_mut_slice();
      std::fs::read_to_string::inner();
      if (local_b0 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (SBORROW8(0,local_128)) {
        if (((uint)local_120 & 3) == 1) {
          puVar8 = *(undefined8 **)(local_120 + 7);
          pcVar9 = (code *)*puVar8;
          if (pcVar9 != (code *)0x0) {
            (*pcVar9)(*(undefined8 *)(local_120 + -1));
          }
          if (puVar8[1] != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        if (local_110 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_02 == (undefined4 *)0x0) {
                    // WARNING: Subroutine does not return
          alloc::raw_vec::handle_error();
        }
        *(undefined4 *)((longlong)extraout_RAX_02 + 3) = 0x746c7561;
        *extraout_RAX_02 = 0x61666564;
        local_c0 = 0;
        local_e8 = 7;
        lStack_d0 = 0x3f800000;
        lStack_c8 = 0x3f8000003f800000;
      }
      else {
        serde_json::de::from_str<country_core::resources::grading_lut::ColorGradingState>();
        if (SBORROW8(0,local_b0)) {
                    // WARNING: Subroutine does not return
          core::result::unwrap_failed();
        }
        local_c0 = local_88;
        lStack_c8 = lStack_90;
        lStack_d0 = local_98;
        local_e8 = local_b0;
        if (local_128 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        if (local_110 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
      }
      uVar13 = *(undefined4 *)((longlong)in_RCX + 0x1c);
      puVar15 = (undefined4 *)in_RCX[2];
      *puVar15 = uVar13;
      plVar12 = (longlong *)*in_RCX;
      alloc::string::impl$6::clone();
      if (*plVar12 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      plVar12[4] = lStack_c8;
      plVar12[5] = CONCAT17(local_81,CONCAT25(local_83,CONCAT41(local_87,local_c0)));
      plVar12[2] = lStack_a0;
      plVar12[3] = lStack_d0;
      *plVar12 = local_b0;
      plVar12[1] = lStack_a8;
      *(undefined8 *)((longlong)plVar12 + 0x36) = 0;
      plVar12[6] = 0;
      alloc::string::impl$6::clone();
      if (plVar12[8] != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      plVar12[0xc] = lStack_c8;
      *(undefined1 *)(plVar12 + 0xd) = local_c0;
      *(undefined4 *)((longlong)plVar12 + 0x69) = local_87;
      *(undefined2 *)((longlong)plVar12 + 0x6d) = local_83;
      *(undefined1 *)((longlong)plVar12 + 0x6f) = local_81;
      plVar12[10] = lStack_a0;
      plVar12[0xb] = lStack_d0;
      plVar12[8] = local_b0;
      plVar12[9] = lStack_a8;
      *(undefined8 *)((longlong)plVar12 + 0x76) = 0;
      plVar12[0xe] = 0;
      alloc::string::impl$6::clone();
      if (plVar12[0x10] != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      plVar12[0x14] = lStack_c8;
      *(undefined1 *)(plVar12 + 0x15) = local_c0;
      *(undefined4 *)((longlong)plVar12 + 0xa9) = local_87;
      *(undefined2 *)((longlong)plVar12 + 0xad) = local_83;
      *(undefined1 *)((longlong)plVar12 + 0xaf) = local_81;
      plVar12[0x12] = lStack_a0;
      plVar12[0x13] = lStack_d0;
      plVar12[0x10] = local_b0;
      plVar12[0x11] = lStack_a8;
      *(undefined8 *)((longlong)plVar12 + 0xb6) = 0;
      plVar12[0x16] = 0;
      if (local_e8 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      goto LAB_1408bff1d;
    }
    pcVar2 = (char *)(*(longlong *)(lVar7 + 8) + lVar16);
    lVar16 = lVar16 + 1;
  } while (*pcVar2 != '\0');
  uVar13 = *(undefined4 *)((longlong)in_RCX + 0x1c);
  puVar15 = (undefined4 *)in_RCX[2];
  *puVar15 = uVar13;
  plVar12 = (longlong *)*in_RCX;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX != (undefined8 *)0x0) {
    *extraout_RAX = 0x61654d7473616f52;
    extraout_RAX[1] = 0x616d656e69435f74;
    *(undefined8 *)((longlong)extraout_RAX + 0xe) = 0x315f3930375f616d;
    if (*plVar12 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    *plVar12 = 0x16;
    plVar12[1] = (longlong)extraout_RAX;
    plVar12[2] = 0x16;
    plVar12[3] = 0x3f400000;
    plVar12[4] = 0x3f87ae143f800000;
    *(undefined1 *)(plVar12 + 5) = 0;
    plVar12[6] = 0;
    *(undefined8 *)((longlong)plVar12 + 0x36) = 0;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_00 != (undefined8 *)0x0) {
      *extraout_RAX_00 = 0x61654d7473616f52;
      extraout_RAX_00[1] = 0x616d656e69435f74;
      *(undefined8 *)((longlong)extraout_RAX_00 + 0xe) = 0x315f3930375f616d;
      if (plVar12[8] != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      plVar12[8] = 0x16;
      plVar12[9] = (longlong)extraout_RAX_00;
      plVar12[10] = 0x16;
      plVar12[0xb] = 0x3f400000;
      plVar12[0xc] = 0x3f87ae143f800000;
      *(undefined1 *)(plVar12 + 0xd) = 0;
      plVar12[0xe] = 0;
      *(undefined8 *)((longlong)plVar12 + 0x76) = 0;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_01 != (undefined8 *)0x0) {
        *extraout_RAX_01 = 0x61654d7473616f52;
        extraout_RAX_01[1] = 0x616d656e69435f74;
        *(undefined8 *)((longlong)extraout_RAX_01 + 0xe) = 0x315f3930375f616d;
        if (plVar12[0x10] != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        plVar12[0x10] = 0x16;
        plVar12[0x11] = (longlong)extraout_RAX_01;
        plVar12[0x12] = 0x16;
        plVar12[0x13] = 0x3f400000;
        plVar12[0x14] = 0x3f87ae143f800000;
        *(undefined1 *)(plVar12 + 0x15) = 0;
        plVar12[0x16] = 0;
        *(undefined8 *)((longlong)plVar12 + 0xb6) = 0;
LAB_1408bff1d:
        if (!SBORROW8(0,*(longlong *)(lVar7 + 0x18))) {
          *puVar15 = uVar13;
          local_78 = 0;
          lVar16 = 0;
          do {
            lVar1 = local_78 + 0x40;
            lVar14 = local_78;
            if (lVar16 == 0) {
              if (lVar1 == 0xc0) {
                return;
              }
              lVar16 = 1;
              lVar14 = lVar1;
              lVar1 = local_78 + 0x80;
            }
            local_78 = lVar1;
            plVar3 = (longlong *)((longlong)plVar12 + lVar14);
            uVar4 = *(undefined1 *)(lVar7 + 0x40);
            alloc::string::impl$6::clone();
            lVar10 = *(longlong *)(lVar7 + 0x30);
            lVar11 = *(longlong *)(lVar7 + 0x38);
            lVar1 = *(longlong *)(lVar7 + 0x48);
            uVar13 = *(undefined4 *)(lVar7 + 0x50);
            uVar5 = *(undefined1 *)(lVar7 + 0x54);
            uVar6 = *(undefined1 *)(lVar7 + 0x55);
            if (*(longlong *)((longlong)plVar12 + lVar14) != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            lVar16 = lVar16 + 1;
            plVar3[4] = lVar11;
            plVar3[5] = CONCAT17(local_81,CONCAT25(local_83,CONCAT41(local_87,uVar4)));
            plVar3[2] = lStack_a0;
            plVar3[3] = lVar10;
            *plVar3 = local_b0;
            plVar3[1] = lStack_a8;
            plVar3[6] = lVar1;
            *(undefined4 *)(plVar3 + 7) = uVar13;
            *(undefined1 *)((longlong)plVar3 + 0x3c) = uVar5;
            *(undefined1 *)((longlong)plVar3 + 0x3d) = uVar6;
          } while (local_78 != 0xc0);
        }
        return;
      }
    }
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}


