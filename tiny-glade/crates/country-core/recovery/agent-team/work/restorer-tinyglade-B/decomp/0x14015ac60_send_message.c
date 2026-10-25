// send_message @ 0x14015ac60
// sig: void __cdecl send_message(void)


// WARNING: Removing unreachable block (ram,0x00014015ad2c)
// WARNING: Removing unreachable block (ram,0x00014015ad92)

void __cdecl tiny_glade::crash_pickup::ClientHandle::send_message(void)

{
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined8 *in_RCX;
  undefined8 uStack_40;
  undefined8 local_38;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  crash_payload::_::impl$0::
  serialize<ref_mut_<serde_json::ser::Serializer<ref_mut_<alloc::vec::Vec<u8,alloc::alloc::Global>_>,serde_json::ser::CompactFormatter>_>_>
            ();
  if (extraout_RAX_00 == 0) {
    minidumper::ipc::client::Client::send_message_impl();
    std::alloc::__default_lib_allocator::__rust_dealloc();
    in_RCX[4] = local_38;
    in_RCX[2] = 0;
    in_RCX[3] = uStack_40;
    *in_RCX = 0x80;
    in_RCX[1] = extraout_RAX;
    core::ptr::drop_in_place<enum2_<tiny_glade::crash_pickup::crash_payload::CrashPickupMessage>_>()
    ;
    return;
  }
  std::alloc::__default_lib_allocator::__rust_dealloc();
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
}

