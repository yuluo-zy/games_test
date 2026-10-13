// addr=0x140960b20 name=cell_from_world_space
// signature=void __cdecl cell_from_world_space(void)


void __cdecl country_core::utils::dirty_tiles::DirtyTiles::cell_from_world_space(void)

{
  bool bVar1;
  uint uVar2;
  uint *in_RCX;
  uint uVar3;
  float fVar4;
  float fVar5;
  
  fVar4 = (float)fmaf();
  fVar5 = (float)fmaf();
  fmaf();
  uVar2 = (uint)fVar5;
  if (2.1474835e+09 < fVar5) {
    uVar2 = 0x7fffffff;
  }
  if (NAN(fVar5)) {
    uVar2 = 0;
  }
  uVar3 = (int)fVar4;
  if (2.1474835e+09 < fVar4) {
    uVar3 = 0x7fffffff;
  }
  if (NAN(fVar4)) {
    uVar3 = 0;
  }
  bVar1 = (uVar2 | uVar3) < 0x40;
  if (bVar1) {
    *(ulonglong *)(in_RCX + 1) = CONCAT44(uVar2,uVar3);
  }
  *in_RCX = (uint)bVar1;
  return;
}


