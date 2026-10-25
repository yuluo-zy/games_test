// snapping_audio addr=0x140a6a480 size=223 sig=void __cdecl snapping_audio(void)

void __cdecl country_core::systems::wall::snapping::snapping_audio::snapping_audio(void)

{
  undefined8 *puVar1;
  char *pcVar2;
  longlong lVar3;
  longlong lVar4;
  undefined8 uVar5;
  longlong lVar6;
  undefined8 *in_RCX;
  longlong lVar7;
  longlong *in_RDX;
  undefined1 local_2c;
  undefined7 uStack_2b;
  undefined8 uStack_24;
  undefined4 local_1c;
  
  pcVar2 = (char *)*in_RCX;
  if (*pcVar2 == '\x02') {
    if (pcVar2[0x20] == '\x02') {
      return;
    }
    ::utils::swizzlers::impl$1::xvy();
    local_2c = 1;
  }
  else {
    if (*pcVar2 != '\x03') {
      return;
    }
    if (1 < (byte)pcVar2[0x20]) {
      return;
    }
    ::utils::swizzlers::impl$1::xvy();
    local_2c = 2;
  }
  *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
  lVar3 = *in_RDX;
  lVar4 = *(longlong *)(lVar3 + 0x30);
  uVar5 = *(undefined8 *)(lVar3 + 0x40);
  if (lVar4 == *(longlong *)(lVar3 + 0x20)) {
    alloc::raw_vec::RawVec::
    grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
              ();
  }
  lVar6 = *(longlong *)(lVar3 + 0x28);
  lVar7 = lVar4 * 0x20;
  *(undefined8 *)(lVar6 + lVar7) = uVar5;
  *(undefined4 *)(lVar6 + 0x18 + lVar7) = local_1c;
  puVar1 = (undefined8 *)(lVar6 + 8 + lVar7);
  *puVar1 = CONCAT71(uStack_2b,local_2c);
  puVar1[1] = uStack_24;
  *(longlong *)(lVar3 + 0x30) = lVar4 + 1;
  *(longlong *)(lVar3 + 0x40) = *(longlong *)(lVar3 + 0x40) + 1;
  return;
}

