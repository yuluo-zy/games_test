// address 0x1412e3c00  name hide<country_core::geometry::instanced_wall::BrickTransformSsbo>
// signature: void __cdecl hide<country_core::geometry::instanced_wall::BrickTransformSsbo>(void)

void __cdecl
country_core::render::sparse_instance_buffer::SparseInstanceBuffer::
hide<country_core::geometry::instanced_wall::BrickTransformSsbo>(void)

{
  ulonglong *puVar1;
  uint uVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  ulonglong uVar6;
  byte bVar7;
  ulonglong uVar8;
  longlong in_RCX;
  longlong in_RDX;
  longlong lVar9;
  
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
    lVar9 = 0;
    do {
      uVar2 = *(uint *)(lVar3 + lVar9);
      uVar8 = (ulonglong)(uVar2 >> 6);
      if (uVar6 <= uVar8) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      bVar7 = (byte)uVar2 & 0x3f;
      LOCK();
      puVar1 = (ulonglong *)(lVar5 + uVar8 * 8);
      *puVar1 = *puVar1 & (-2L << bVar7 | 0xfffffffffffffffeU >> 0x40 - bVar7);
      UNLOCK();
      lVar9 = lVar9 + 4;
    } while (lVar4 << 2 != lVar9);
  }
  return;
}


