
// WARNING: Type propagation algorithm not settling
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl country_core::input::action_set::MainActionSet::deconstruct(void)

{
  int iVar1;
  undefined8 uVar2;
  int iVar3;
  undefined8 *in_RCX;
  int *in_RDX;
  longlong lVar4;
  float fVar5;
  longlong local_48;
  
  local_48 = 0;
  uVar2 = *(undefined8 *)(in_RDX + 0x50);
  fVar5 = (float)((ulonglong)uVar2 >> 0x20);
  if (((((float)uVar2 != 0.0) || (NAN((float)uVar2))) || (fVar5 != 0.0)) || (NAN(fVar5))) {
    alloc::raw_vec::RawVec::
    grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
              ();
    _DAT_00000004 = 0;
    local_48 = 1;
    lVar4 = 1;
    _DAT_00000008 = uVar2;
  }
  else {
    lVar4 = 0;
  }
  uVar2 = *(undefined8 *)(in_RDX + 0x52);
  fVar5 = (float)((ulonglong)uVar2 >> 0x20);
  if ((((float)uVar2 != 0.0) || (NAN((float)uVar2))) || ((fVar5 != 0.0 || (NAN(fVar5))))) {
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x100;
    *(undefined8 *)(lVar4 * 0xc + 8) = uVar2;
    lVar4 = lVar4 + 1;
    local_48 = lVar4;
  }
  uVar2 = *(undefined8 *)(in_RDX + 0x54);
  fVar5 = (float)((ulonglong)uVar2 >> 0x20);
  if ((((float)uVar2 != 0.0) || (NAN((float)uVar2))) || ((fVar5 != 0.0 || (NAN(fVar5))))) {
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x200;
    *(undefined8 *)(lVar4 * 0xc + 8) = uVar2;
    lVar4 = lVar4 + 1;
    local_48 = lVar4;
  }
  uVar2 = *(undefined8 *)(in_RDX + 0x56);
  fVar5 = (float)((ulonglong)uVar2 >> 0x20);
  if (((((float)uVar2 != 0.0) || (NAN((float)uVar2))) || (fVar5 != 0.0)) || (NAN(fVar5))) {
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x300;
    *(undefined8 *)(lVar4 * 0xc + 8) = uVar2;
    lVar4 = lVar4 + 1;
    local_48 = lVar4;
  }
  uVar2 = *(undefined8 *)(in_RDX + 0x58);
  fVar5 = (float)((ulonglong)uVar2 >> 0x20);
  if ((((float)uVar2 != 0.0) || (NAN((float)uVar2))) || ((fVar5 != 0.0 || (NAN(fVar5))))) {
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x400;
    *(undefined8 *)(lVar4 * 0xc + 8) = uVar2;
    lVar4 = lVar4 + 1;
    local_48 = lVar4;
  }
  uVar2 = *(undefined8 *)(in_RDX + 0x5a);
  fVar5 = (float)((ulonglong)uVar2 >> 0x20);
  if ((((float)uVar2 != 0.0) || (NAN((float)uVar2))) || ((fVar5 != 0.0 || (NAN(fVar5))))) {
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x500;
    *(undefined8 *)(lVar4 * 0xc + 8) = uVar2;
    lVar4 = lVar4 + 1;
    iVar3 = *in_RDX;
    local_48 = lVar4;
  }
  else {
    iVar3 = *in_RDX;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[2];
  }
  else {
    iVar1 = in_RDX[1];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 1;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[2];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[4];
  }
  else {
    iVar1 = in_RDX[3];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x101;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[4];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[6];
  }
  else {
    iVar1 = in_RDX[5];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x201;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[6];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[8];
  }
  else {
    iVar1 = in_RDX[7];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x301;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[8];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[10];
  }
  else {
    iVar1 = in_RDX[9];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x401;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[10];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0xc];
  }
  else {
    iVar1 = in_RDX[0xb];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x501;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0xc];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0xe];
  }
  else {
    iVar1 = in_RDX[0xd];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x601;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0xe];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x10];
  }
  else {
    iVar1 = in_RDX[0xf];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x701;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x10];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x12];
  }
  else {
    iVar1 = in_RDX[0x11];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x801;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x12];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x14];
  }
  else {
    iVar1 = in_RDX[0x13];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x901;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x14];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x16];
  }
  else {
    iVar1 = in_RDX[0x15];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0xa01;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x16];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x18];
  }
  else {
    iVar1 = in_RDX[0x17];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0xb01;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x18];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x1a];
  }
  else {
    iVar1 = in_RDX[0x19];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0xc01;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x1a];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x1c];
  }
  else {
    iVar1 = in_RDX[0x1b];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0xd01;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x1c];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x1e];
  }
  else {
    iVar1 = in_RDX[0x1d];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0xe01;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x1e];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x20];
  }
  else {
    iVar1 = in_RDX[0x1f];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0xf01;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x20];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x22];
  }
  else {
    iVar1 = in_RDX[0x21];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x1001;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x22];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x24];
  }
  else {
    iVar1 = in_RDX[0x23];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x1101;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x24];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x26];
  }
  else {
    iVar1 = in_RDX[0x25];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x1201;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x26];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x28];
  }
  else {
    iVar1 = in_RDX[0x27];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x1301;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x28];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x2a];
  }
  else {
    iVar1 = in_RDX[0x29];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x1401;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x2a];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x2c];
  }
  else {
    iVar1 = in_RDX[0x2b];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x1501;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x2c];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x2e];
  }
  else {
    iVar1 = in_RDX[0x2d];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x1601;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x2e];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x30];
  }
  else {
    iVar1 = in_RDX[0x2f];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x1701;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x30];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x32];
  }
  else {
    iVar1 = in_RDX[0x31];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x1801;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x32];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x34];
  }
  else {
    iVar1 = in_RDX[0x33];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x1901;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x34];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x36];
  }
  else {
    iVar1 = in_RDX[0x35];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x1a01;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x36];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x38];
  }
  else {
    iVar1 = in_RDX[0x37];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x1b01;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x38];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x3a];
  }
  else {
    iVar1 = in_RDX[0x39];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x1c01;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x3a];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x3c];
  }
  else {
    iVar1 = in_RDX[0x3b];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x1d01;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x3c];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x3e];
  }
  else {
    iVar1 = in_RDX[0x3d];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x1e01;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x3e];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x40];
  }
  else {
    iVar1 = in_RDX[0x3f];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x1f01;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x40];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x42];
  }
  else {
    iVar1 = in_RDX[0x41];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x2001;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x42];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x44];
  }
  else {
    iVar1 = in_RDX[0x43];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x2101;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x44];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x46];
  }
  else {
    iVar1 = in_RDX[0x45];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x2201;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x46];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x48];
  }
  else {
    iVar1 = in_RDX[0x47];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x2301;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x48];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x4a];
  }
  else {
    iVar1 = in_RDX[0x49];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x2401;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x4a];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x4c];
  }
  else {
    iVar1 = in_RDX[0x4b];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x2501;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x4c];
    local_48 = lVar4;
  }
  if (iVar3 == 4) {
    iVar3 = in_RDX[0x4e];
  }
  else {
    iVar1 = in_RDX[0x4d];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x2601;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    lVar4 = lVar4 + 1;
    iVar3 = in_RDX[0x4e];
    local_48 = lVar4;
  }
  if (iVar3 != 4) {
    iVar1 = in_RDX[0x4f];
    if (lVar4 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                ();
    }
    *(undefined2 *)(lVar4 * 0xc + 4) = 0x2701;
    *(int *)(lVar4 * 0xc + 8) = iVar3;
    *(int *)(lVar4 * 0xc + 0xc) = iVar1;
    local_48 = lVar4 + 1;
  }
  in_RCX[2] = local_48;
  *in_RCX = 0;
  in_RCX[1] = 4;
  return;
}

