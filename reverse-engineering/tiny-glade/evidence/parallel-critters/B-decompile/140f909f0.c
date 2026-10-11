
void __cdecl system_critters::fish::FishSpawnLocations::new(void)

{
  float fVar1;
  float fVar2;
  float fVar3;
  undefined8 *in_RCX;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  float fVar8;
  float fVar9;
  
  fVar3 = __real_42500000;
  fVar2 = __real_41c00000;
  fVar1 = __real_3f800000;
  lVar6 = 0;
  lVar7 = 0;
  do {
    fVar8 = (float)lVar7;
    lVar7 = lVar7 + 1;
    fVar8 = fVar8 / fVar2;
    lVar4 = lVar6 * 0xc;
    lVar5 = 0;
    do {
      fVar9 = (float)lVar5 / fVar2;
      if (lVar6 == 0) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                  ();
      }
      lVar5 = lVar5 + 1;
      *(float *)(lVar4 + 4) = fVar8 * fVar3 - (fVar1 - fVar8) * fVar3;
      *(float *)(lVar4 + 8) = fVar9 * fVar3 - (fVar1 - fVar9) * fVar3;
      *(undefined1 *)(lVar4 + 0xc) = 0;
      lVar6 = lVar6 + 1;
      lVar4 = lVar4 + 0xc;
    } while (lVar5 != 0x19);
  } while (lVar7 != 0x19);
  in_RCX[2] = lVar6;
  *in_RCX = 0;
  in_RCX[1] = 4;
  return;
}

