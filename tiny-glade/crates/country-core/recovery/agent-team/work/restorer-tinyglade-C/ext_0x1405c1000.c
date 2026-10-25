
void __cdecl
core::slice::sort::shared::smallsort::
insert_tail<enum2_<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool_(*)(ref_<enum2_<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>_>,ref_<enum2_<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>_>)>
          (void)

{
  longlong *plVar1;
  longlong lVar2;
  ulonglong uVar3;
  int iVar4;
  longlong *plVar5;
  longlong *in_RCX;
  longlong *in_RDX;
  longlong lVar6;
  void *_Buf1;
  ulonglong uVar7;
  longlong lVar8;
  ulonglong uVar9;
  
  lVar2 = *in_RDX;
  if (lVar2 == in_RDX[-4]) {
    _Buf1 = (void *)in_RDX[2];
    uVar9 = in_RDX[3];
    uVar3 = in_RDX[-1];
    uVar7 = uVar3;
    if (uVar9 < uVar3) {
      uVar7 = uVar9;
    }
    iVar4 = memcmp(_Buf1,(void *)in_RDX[-2],uVar7);
    lVar8 = uVar9 - uVar3;
    if (iVar4 != 0) {
      lVar8 = (longlong)iVar4;
    }
    if (-1 < lVar8) {
      return;
    }
  }
  else {
    if ((uint)in_RDX[-4] <= (uint)lVar2) {
      return;
    }
    _Buf1 = (void *)in_RDX[2];
    uVar9 = in_RDX[3];
  }
  plVar5 = in_RDX + -4;
  lVar8 = in_RDX[1];
  in_RDX[2] = in_RDX[-2];
  in_RDX[3] = in_RDX[-1];
  *in_RDX = *plVar5;
  in_RDX[1] = in_RDX[-3];
  if (plVar5 != in_RCX) {
    if (lVar2 == 0) {
      do {
        if (plVar5[-4] == 0) {
          uVar3 = plVar5[-1];
          uVar7 = uVar3;
          if (uVar9 < uVar3) {
            uVar7 = uVar9;
          }
          iVar4 = memcmp(_Buf1,(void *)plVar5[-2],uVar7);
          lVar6 = uVar9 - uVar3;
          if (iVar4 != 0) {
            lVar6 = (longlong)iVar4;
          }
          if (-1 < lVar6) break;
        }
        else if ((int)plVar5[-4] == 0) break;
        plVar1 = plVar5 + -4;
        plVar5[2] = plVar5[-2];
        plVar5[3] = plVar5[-1];
        *plVar5 = *plVar1;
        plVar5[1] = plVar5[-3];
        plVar5 = plVar1;
      } while (plVar1 != in_RCX);
    }
    else {
      do {
        if (plVar5[-4] == 0) break;
        uVar3 = plVar5[-1];
        uVar7 = uVar3;
        if (uVar9 < uVar3) {
          uVar7 = uVar9;
        }
        iVar4 = memcmp(_Buf1,(void *)plVar5[-2],uVar7);
        lVar6 = uVar9 - uVar3;
        if (iVar4 != 0) {
          lVar6 = (longlong)iVar4;
        }
        if (-1 < lVar6) break;
        plVar1 = plVar5 + -4;
        plVar5[2] = plVar5[-2];
        plVar5[3] = plVar5[-1];
        *plVar5 = *plVar1;
        plVar5[1] = plVar5[-3];
        plVar5 = plVar1;
      } while (plVar1 != in_RCX);
    }
  }
  *plVar5 = lVar2;
  plVar5[1] = lVar8;
  plVar5[2] = (longlong)_Buf1;
  plVar5[3] = uVar9;
  return;
}

