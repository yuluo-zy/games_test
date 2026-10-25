// check_for_disable_snapping addr=0x140a6a560 size=230 sig=void __cdecl check_for_disable_snapping(void)

void __cdecl
country_core::systems::wall::snapping::snapping_maintain::check_for_disable_snapping(void)

{
  char cVar1;
  char *pcVar2;
  longlong lVar3;
  longlong lVar4;
  undefined8 uVar5;
  longlong lVar6;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  longlong *in_R8;
  
  pcVar2 = (char *)*in_RCX;
  cVar1 = *pcVar2;
  if (cVar1 == '\x02') {
    if (*(char *)*in_RDX == '\0') {
LAB_140a6a5f0:
      *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
      lVar3 = *in_R8;
      lVar4 = *(longlong *)(lVar3 + 0x30);
      uVar5 = *(undefined8 *)(lVar3 + 0x40);
      if (lVar4 == *(longlong *)(lVar3 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar6 = *(longlong *)(lVar3 + 0x28);
      *(undefined8 *)(lVar6 + lVar4 * 0x10) = uVar5;
      *(undefined1 *)(lVar6 + 8 + lVar4 * 0x10) = 0;
      *(longlong *)(lVar3 + 0x30) = lVar4 + 1;
      *(longlong *)(lVar3 + 0x40) = *(longlong *)(lVar3 + 0x40) + 1;
      return;
    }
  }
  else {
    if (*(char *)*in_RDX == '\0') {
      if (cVar1 != '\x03') {
        return;
      }
      goto LAB_140a6a5f0;
    }
    if (cVar1 != '\x03') goto LAB_140a6a5de;
  }
  *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
  lVar3 = *in_R8;
  lVar4 = *(longlong *)(lVar3 + 0x30);
  uVar5 = *(undefined8 *)(lVar3 + 0x40);
  if (lVar4 == *(longlong *)(lVar3 + 0x20)) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
              ();
  }
  lVar6 = *(longlong *)(lVar3 + 0x28);
  *(undefined8 *)(lVar6 + lVar4 * 0x10) = uVar5;
  *(undefined1 *)(lVar6 + 8 + lVar4 * 0x10) = 1;
  *(longlong *)(lVar3 + 0x30) = lVar4 + 1;
  *(longlong *)(lVar3 + 0x40) = *(longlong *)(lVar3 + 0x40) + 1;
LAB_140a6a5de:
  *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
  *pcVar2 = '\0';
  return;
}

