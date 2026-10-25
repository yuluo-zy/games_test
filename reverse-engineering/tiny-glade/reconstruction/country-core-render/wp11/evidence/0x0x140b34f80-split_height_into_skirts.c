// 0x140b34f80 split_height_into_skirts
// sig: void __cdecl split_height_into_skirts(void)

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl country_core::resources::walls::split_height_into_skirts(void)

{
  undefined8 *in_RCX;
  longlong lVar1;
  float in_XMM1_Da;
  float fVar2;
  
  if (5.9 <= in_XMM1_Da) {
    alloc::raw_vec::RawVec::grow_one<f32,alloc::alloc::Global>();
    _DAT_00000004 = 0x40900000;
    fVar2 = 4.5;
    lVar1 = 1;
    while( true ) {
      fVar2 = fVar2 + 3.5;
      if (in_XMM1_Da <= fVar2 + 1.4) break;
      if (lVar1 == 0) {
        alloc::raw_vec::RawVec::grow_one<f32,alloc::alloc::Global>();
      }
      *(float *)(lVar1 * 4 + 4) = fVar2;
      lVar1 = lVar1 + 1;
    }
    in_RCX[2] = lVar1;
    *in_RCX = 0;
    in_RCX[1] = 4;
  }
  else {
    in_RCX[2] = 0;
    *in_RCX = 0;
    in_RCX[1] = 4;
  }
  return;
}

