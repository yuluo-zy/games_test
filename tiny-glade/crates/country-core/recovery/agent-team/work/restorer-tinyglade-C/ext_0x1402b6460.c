
void __cdecl
alloc::vec::Vec::
dedup_by<enum2_<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global,alloc::vec::impl_5::dedup::closure_env_0<enum2_<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>_>
          (void)

{
  undefined8 *puVar1;
  ulonglong uVar2;
  longlong lVar3;
  ulonglong uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  int iVar8;
  longlong in_RCX;
  int *piVar9;
  ulonglong uVar10;
  
  uVar2 = *(ulonglong *)(in_RCX + 0x10);
  if (1 < uVar2) {
    lVar3 = *(longlong *)(in_RCX + 8);
    piVar9 = (int *)(lVar3 + 0x40);
    uVar10 = 1;
    do {
      if (((piVar9[-8] == piVar9[-0x10]) && (*(size_t *)(piVar9 + -2) == *(size_t *)(piVar9 + -10)))
         && (iVar8 = memcmp(*(void **)(piVar9 + -4),*(void **)(piVar9 + -0xc),
                            *(size_t *)(piVar9 + -2)), iVar8 == 0)) {
        uVar4 = uVar10;
        if (*(longlong *)(piVar9 + -6) != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        while (uVar4 + 1 < uVar2) {
          puVar1 = (undefined8 *)(lVar3 + uVar10 * 0x20);
          if (((*piVar9 == *(int *)(lVar3 + -0x20 + uVar10 * 0x20)) &&
              (*(size_t *)(piVar9 + 6) == puVar1[-1])) &&
             (iVar8 = memcmp(*(void **)(piVar9 + 4),(void *)puVar1[-2],*(size_t *)(piVar9 + 6)),
             iVar8 == 0)) {
            if (*(longlong *)(piVar9 + 2) != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
          }
          else {
            uVar5 = *(undefined8 *)piVar9;
            uVar6 = *(undefined8 *)(piVar9 + 2);
            uVar7 = *(undefined8 *)(piVar9 + 6);
            puVar1[2] = *(undefined8 *)(piVar9 + 4);
            puVar1[3] = uVar7;
            *puVar1 = uVar5;
            puVar1[1] = uVar6;
            uVar10 = uVar10 + 1;
          }
          piVar9 = piVar9 + 8;
          uVar4 = uVar4 + 1;
        }
        *(ulonglong *)(in_RCX + 0x10) = uVar10;
        return;
      }
      uVar10 = uVar10 + 1;
      piVar9 = piVar9 + 8;
    } while (uVar2 != uVar10);
  }
  return;
}

