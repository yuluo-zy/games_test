// 0x14067da90 increase
// signature: void __cdecl increase(void)

void __cdecl toolbar::mei::widgets::tweaker::impl$4::increase(void)

{
  float fVar1;
  float *pfVar2;
  undefined8 *in_RCX;
  float fVar3;
  float fVar4;
  float fVar5;
  
  fVar5 = *(float *)((longlong)in_RCX + 0xc);
  fVar4 = *(float *)(in_RCX + 2);
  fVar1 = *(float *)(in_RCX + 1);
  pfVar2 = (float *)*in_RCX;
  fVar3 = (float)roundf((*pfVar2 + fVar1) / fVar1);
  fVar3 = fVar3 * fVar1;
  *pfVar2 = fVar3;
  if (fVar5 <= fVar4) {
    if (fVar5 <= fVar3) {
      fVar5 = fVar3;
    }
    if (fVar5 <= fVar4) {
      fVar4 = fVar5;
    }
    *pfVar2 = fVar4;
    return;
  }
                    // WARNING: Subroutine does not return
  ::core::f32::impl$0::clamp::do_panic::runtime();
}

