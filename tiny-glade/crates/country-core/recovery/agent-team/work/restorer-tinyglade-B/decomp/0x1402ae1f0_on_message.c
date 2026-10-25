// on_message @ 0x1402ae1f0
// sig: void __cdecl on_message(void)


void __cdecl tiny_glade::crash_pickup::server::impl$1::on_message(void)

{
  longlong lVar1;
  longlong in_R8;
  undefined8 local_718 [128];
  longlong local_318;
  undefined8 local_310;
  undefined8 local_68;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  local_68 = *(undefined8 *)(in_R8 + 8);
  serde_json::de::from_slice<enum2_<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage>_>()
  ;
  if (local_318 != -0x7ffffffffffffffc) {
    memcpy(local_718,(void *)((longlong)&local_310 + 2),0x20e);
    lVar1 = 0;
    if (local_318 < -0x7ffffffffffffffc) {
      lVar1 = local_318 + -0x7fffffffffffffff;
    }
                    // WARNING: Could not emulate address calculation at 0x0001402ae299
                    // WARNING: Treating indirect jump as call
    (*(code *)((longlong)&DAT_1429ad390 + (longlong)(int)(&DAT_1429ad390)[lVar1]))
              ((code *)((longlong)&DAT_1429ad390 + (longlong)(int)(&DAT_1429ad390)[lVar1]));
    return;
  }
  local_718[0] = local_310;
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
}

