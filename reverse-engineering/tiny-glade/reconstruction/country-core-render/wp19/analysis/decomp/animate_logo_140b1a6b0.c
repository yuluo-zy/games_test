// animate_logo  0x140b1a6b0
// signature: void __cdecl animate_logo(void)
// callees: [{"name": "__rust_alloc", "address": "140613c10"}, {"name": "__rust_dealloc", "address": "140613c20"}, {"name": "drop", "address": "14089a280"}, {"name": "handle_error", "address": "1428d8fe3"}, {"name": "instantiate_audio", "address": "140a54440"}, {"name": "start_if_not_playing", "address": "140a57300"}, {"name": "unwrap_failed", "address": "1428d9760"}]


void __cdecl country_core::resources::logo_animation::animate_logo(void)

{
  float *pfVar1;
  char *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *in_RCX;
  longlong *in_RDX;
  float *pfVar2;
  undefined8 *in_R8;
  float fVar3;
  float fVar4;
  float fVar5;
  longlong local_c8;
  longlong local_b8;
  
  fVar3 = *(float *)(*in_RDX + 0x58);
  pfVar1 = (float *)*in_RCX;
  if (1.0 < *pfVar1 || *pfVar1 == 1.0) {
    if (1.0 < pfVar1[1] || pfVar1[1] == 1.0) {
      if (1.0 < pfVar1[2] || pfVar1[2] == 1.0) {
        if (1.0 < pfVar1[3] || pfVar1[3] == 1.0) {
          if (1.0 < pfVar1[4] || pfVar1[4] == 1.0) {
            return;
          }
          fVar4 = pfVar1[9];
          *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
          fVar5 = pfVar1[4];
          pfVar2 = pfVar1 + 4;
        }
        else {
          fVar4 = pfVar1[8];
          *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
          fVar5 = pfVar1[3];
          pfVar2 = pfVar1 + 3;
        }
      }
      else {
        pfVar2 = pfVar1 + 2;
        fVar4 = pfVar1[7];
        *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
        fVar5 = pfVar1[2];
        if ((fVar5 == 0.0) && (!NAN(fVar5))) {
          if (*(longlong *)*in_R8 == 0) {
            std::alloc::__default_lib_allocator::__rust_alloc();
            if (extraout_RAX_00 == (undefined8 *)0x0) goto LAB_140b1aa79;
            *extraout_RAX_00 = 0x552f3a746e657665;
            extraout_RAX_00[1] = 0x422f6f676f4c2f49;
            *(undefined8 *)((longlong)extraout_RAX_00 + 0xf) = 0x736568636e617242;
            local_b8 = 0x17;
          }
          else {
            audio_resources::AudioCoreInner::instantiate_audio();
            if (local_c8 == 2) {
                    // WARNING: Subroutine does not return
              core::result::unwrap_failed();
            }
          }
          audio_resources::AudioInstance::start_if_not_playing();
          audio_resources::impl$13::drop();
          if (local_b8 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
        }
      }
    }
    else {
      fVar4 = pfVar1[6];
      *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
      fVar5 = pfVar1[1];
      pfVar2 = pfVar1 + 1;
    }
  }
  else {
    fVar4 = pfVar1[5];
    *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
    fVar5 = *pfVar1;
    pfVar2 = pfVar1;
    if ((fVar5 == 0.0) && (!NAN(fVar5))) {
      if (*(longlong *)*in_R8 == 0) {
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX == (char *)0x0) {
LAB_140b1aa79:
                    // WARNING: Subroutine does not return
          alloc::raw_vec::handle_error();
        }
        builtin_strncpy(extraout_RAX,"event:/UI/Logo/Scribble",0x17);
        local_b8 = 0x17;
      }
      else {
        audio_resources::AudioCoreInner::instantiate_audio();
        if (local_c8 == 2) {
                    // WARNING: Subroutine does not return
          core::result::unwrap_failed();
        }
      }
      audio_resources::AudioInstance::start_if_not_playing();
      audio_resources::impl$13::drop();
      if (local_b8 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
  }
  fVar5 = fVar3 / fVar4 + fVar5;
  fVar3 = 1.1754944e-38;
  if (1.1754944e-38 <= fVar5) {
    fVar3 = fVar5;
  }
  fVar4 = 1.0;
  if (fVar3 <= 1.0) {
    fVar4 = fVar3;
  }
  *pfVar2 = fVar4;
  return;
}

