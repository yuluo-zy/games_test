
void __cdecl system_critters::spawn_ducks::DuckSpawnLocations::new(void)

{
  float fVar1;
  float fVar2;
  float fVar3;
  undefined8 *in_RCX;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  float fVar7;
  
  fVar3 = __real_42500000;
  fVar2 = __real_41100000;
  fVar1 = __real_3f800000;
  lVar5 = 0;
  lVar6 = 0;
  lVar4 = 0;
  do {
    if (lVar6 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    fVar7 = ((float)lVar4 / fVar2) * fVar3 - (fVar1 - (float)lVar4 / fVar2) * fVar3;
    *(float *)(lVar5 + 4) = fVar7;
    *(undefined4 *)(lVar5 + 8) = 0xc2500000;
    *(undefined1 *)(lVar5 + 0xc) = 0;
    if (lVar6 == -1) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(float *)(lVar5 + 0x10) = fVar7;
    *(undefined4 *)(lVar5 + 0x14) = 0xc221c71c;
    *(undefined1 *)(lVar5 + 0x18) = 0;
    if (lVar6 == -2) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(float *)(lVar5 + 0x1c) = fVar7;
    *(undefined4 *)(lVar5 + 0x20) = 0xc1e71c73;
    *(undefined1 *)(lVar5 + 0x24) = 0;
    if (lVar6 == -3) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(float *)(lVar5 + 0x28) = fVar7;
    *(undefined4 *)(lVar5 + 0x2c) = 0xc18aaaa9;
    *(undefined1 *)(lVar5 + 0x30) = 0;
    if (lVar6 == -4) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(float *)(lVar5 + 0x34) = fVar7;
    *(undefined4 *)(lVar5 + 0x38) = 0xc0b8e390;
    *(undefined1 *)(lVar5 + 0x3c) = 0;
    if (lVar6 == -5) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(float *)(lVar5 + 0x40) = fVar7;
    *(undefined4 *)(lVar5 + 0x44) = 0x40b8e390;
    *(undefined1 *)(lVar5 + 0x48) = 0;
    if (lVar6 == -6) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(float *)(lVar5 + 0x4c) = fVar7;
    *(undefined4 *)(lVar5 + 0x50) = 0x418aaaac;
    *(undefined1 *)(lVar5 + 0x54) = 0;
    if (lVar6 == -7) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(float *)(lVar5 + 0x58) = fVar7;
    *(undefined4 *)(lVar5 + 0x5c) = 0x41e71c73;
    *(undefined1 *)(lVar5 + 0x60) = 0;
    if (lVar6 == -8) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(float *)(lVar5 + 100) = fVar7;
    *(undefined4 *)(lVar5 + 0x68) = 0x4221c71c;
    *(undefined1 *)(lVar5 + 0x6c) = 0;
    if (lVar6 == -9) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(float *)(lVar5 + 0x70) = fVar7;
    *(undefined4 *)(lVar5 + 0x74) = 0x42500000;
    *(undefined1 *)(lVar5 + 0x78) = 0;
    lVar6 = lVar6 + 10;
    lVar5 = lVar5 + 0x78;
    lVar4 = lVar4 + 1;
  } while (lVar5 != 0x4b0);
  in_RCX[2] = lVar6;
  *in_RCX = 0;
  in_RCX[1] = 4;
  *(undefined4 *)(in_RCX + 3) = 0x41c80000;
  return;
}

