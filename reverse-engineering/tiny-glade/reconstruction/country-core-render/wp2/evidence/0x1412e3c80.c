// address 0x1412e3c80  name show<country_core::geometry::instanced_wall::BrickTransformSsbo>
// signature: void __cdecl show<country_core::geometry::instanced_wall::BrickTransformSsbo>(void)

void __cdecl
country_core::render::sparse_instance_buffer::SparseInstanceBuffer::
show<country_core::geometry::instanced_wall::BrickTransformSsbo>(void)

{
  ulonglong *puVar1;
  uint uVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  longlong in_RCX;
  longlong in_RDX;
  longlong lVar8;
  
  lVar3 = *(longlong *)(in_RDX + 8);
  if (lVar3 == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  lVar4 = *(longlong *)(lVar3 + 0x10);
  if (lVar4 != 0) {
    lVar3 = *(longlong *)(lVar3 + 8);
    lVar5 = *(longlong *)(in_RCX + 0x30);
    uVar6 = *(ulonglong *)(in_RCX + 0x38);
    lVar8 = 0;
    do {
      uVar2 = *(uint *)(lVar3 + lVar8);
      uVar7 = (ulonglong)(uVar2 >> 6);
      if (uVar6 <= uVar7) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      LOCK();
      puVar1 = (ulonglong *)(lVar5 + uVar7 * 8);
      *puVar1 = *puVar1 | 1L << ((byte)uVar2 & 0x3f);
      UNLOCK();
      lVar8 = lVar8 + 4;
    } while (lVar4 << 2 != lVar8);
  }
  return;
}


