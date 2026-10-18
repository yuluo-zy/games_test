// new 0x140f909f0 sig=void __cdecl new(void)

void __cdecl system_critters::fish::FishSpawnLocations::new(void)

{
  undefined8 *in_RCX;
  longlong lVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  float fVar5;
  float fVar6;
  
  lVar3 = 0;
  lVar4 = 0;
  do {
    fVar5 = (float)lVar4;
    lVar4 = lVar4 + 1;
    lVar1 = lVar3 * 0xc;
    lVar2 = 0;
    do {
      fVar6 = (float)lVar2;
      if (lVar3 == 0) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                  ();
      }
      lVar2 = lVar2 + 1;
      *(float *)(lVar1 + 4) = (fVar5 / 24.0) * 52.0 - (1.0 - fVar5 / 24.0) * 52.0;
      *(float *)(lVar1 + 8) = (fVar6 / 24.0) * 52.0 - (1.0 - fVar6 / 24.0) * 52.0;
      *(undefined1 *)(lVar1 + 0xc) = 0;
      lVar3 = lVar3 + 1;
      lVar1 = lVar1 + 0xc;
    } while (lVar2 != 0x19);
  } while (lVar4 != 0x19);
  in_RCX[2] = lVar3;
  *in_RCX = 0;
  in_RCX[1] = 4;
  return;
}

