// new 0x140f8ee90 sig=void __cdecl new(void)

void __cdecl system_critters::spawn_ducks::DuckSpawnLocations::new(void)

{
  undefined8 *in_RCX;
  longlong lVar1;
  longlong lVar2;
  longlong lVar3;
  float fVar4;
  
  lVar2 = 0;
  lVar3 = 0;
  lVar1 = 0;
  do {
    if (lVar3 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    fVar4 = ((float)lVar1 / 9.0) * 52.0 - (1.0 - (float)lVar1 / 9.0) * 52.0;
    *(float *)(lVar2 + 4) = fVar4;
    *(undefined4 *)(lVar2 + 8) = 0xc2500000;
    *(undefined1 *)(lVar2 + 0xc) = 0;
    if (lVar3 == -1) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(float *)(lVar2 + 0x10) = fVar4;
    *(undefined4 *)(lVar2 + 0x14) = 0xc221c71c;
    *(undefined1 *)(lVar2 + 0x18) = 0;
    if (lVar3 == -2) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(float *)(lVar2 + 0x1c) = fVar4;
    *(undefined4 *)(lVar2 + 0x20) = 0xc1e71c73;
    *(undefined1 *)(lVar2 + 0x24) = 0;
    if (lVar3 == -3) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(float *)(lVar2 + 0x28) = fVar4;
    *(undefined4 *)(lVar2 + 0x2c) = 0xc18aaaa9;
    *(undefined1 *)(lVar2 + 0x30) = 0;
    if (lVar3 == -4) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(float *)(lVar2 + 0x34) = fVar4;
    *(undefined4 *)(lVar2 + 0x38) = 0xc0b8e390;
    *(undefined1 *)(lVar2 + 0x3c) = 0;
    if (lVar3 == -5) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(float *)(lVar2 + 0x40) = fVar4;
    *(undefined4 *)(lVar2 + 0x44) = 0x40b8e390;
    *(undefined1 *)(lVar2 + 0x48) = 0;
    if (lVar3 == -6) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(float *)(lVar2 + 0x4c) = fVar4;
    *(undefined4 *)(lVar2 + 0x50) = 0x418aaaac;
    *(undefined1 *)(lVar2 + 0x54) = 0;
    if (lVar3 == -7) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(float *)(lVar2 + 0x58) = fVar4;
    *(undefined4 *)(lVar2 + 0x5c) = 0x41e71c73;
    *(undefined1 *)(lVar2 + 0x60) = 0;
    if (lVar3 == -8) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(float *)(lVar2 + 100) = fVar4;
    *(undefined4 *)(lVar2 + 0x68) = 0x4221c71c;
    *(undefined1 *)(lVar2 + 0x6c) = 0;
    if (lVar3 == -9) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(float *)(lVar2 + 0x70) = fVar4;
    *(undefined4 *)(lVar2 + 0x74) = 0x42500000;
    *(undefined1 *)(lVar2 + 0x78) = 0;
    lVar3 = lVar3 + 10;
    lVar2 = lVar2 + 0x78;
    lVar1 = lVar1 + 1;
  } while (lVar2 != 0x4b0);
  in_RCX[2] = lVar3;
  *in_RCX = 0;
  in_RCX[1] = 4;
  *(undefined4 *)(in_RCX + 3) = 0x41c80000;
  return;
}

