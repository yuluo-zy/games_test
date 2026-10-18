140b3c930: 4157 push r15
140b3c932: 4156 push r14
140b3c934: 4155 push r13
140b3c936: 4154 push r12
140b3c938: 56 push rsi
140b3c939: 57 push rdi
140b3c93a: 53 push rbx
140b3c93b: 4881ec80000000 sub rsp, 0x80
140b3c942: bb01000000 mov ebx, 1
140b3c947: f00fc19ab0050000 lock xadd dword ptr [rdx + 0x5b0], ebx
140b3c94f: f60101 test byte ptr [rcx], 1
140b3c952: 0f84ad010000 je 0x140b3cb05
140b3c958: 4889ce mov rsi, rcx
140b3c95b: 488b4108 mov rax, qword ptr [rcx + 8]
140b3c95f: 48398218020000 cmp qword ptr [rdx + 0x218], rax
140b3c966: 0f86f6000000 jbe 0x140b3ca62
140b3c96c: 488b8a10020000 mov rcx, qword ptr [rdx + 0x210]
140b3c973: 488b0cc1 mov rcx, qword ptr [rcx + rax*8]
140b3c977: 4885c9 test rcx, rcx
140b3c97a: 0f84e2000000 je 0x140b3ca62
140b3c980: 488b82e0010000 mov rax, qword ptr [rdx + 0x1e0]
140b3c987: 48f7d1 not rcx
140b3c98a: 488d0c49 lea rcx, [rcx + rcx*2]
140b3c98e: 48c1e105 shl rcx, 5
140b3c992: 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
140b3c998: 0f84c4000000 je 0x140b3ca62
140b3c99e: 4801c8 add rax, rcx
140b3c9a1: 4c8b7010 mov r14, qword ptr [rax + 0x10]
140b3c9a5: 8b8ab0050000 mov ecx, dword ptr [rdx + 0x5b0]
140b3c9ab: 488b4e10 mov rcx, qword ptr [rsi + 0x10]
140b3c9af: 48398a18020000 cmp qword ptr [rdx + 0x218], rcx
140b3c9b6: 0f86c4000000 jbe 0x140b3ca80
140b3c9bc: 4c8b8210020000 mov r8, qword ptr [rdx + 0x210]
140b3c9c3: 498b0cc8 mov rcx, qword ptr [r8 + rcx*8]
140b3c9c7: 4885c9 test rcx, rcx
140b3c9ca: 0f84b0000000 je 0x140b3ca80
140b3c9d0: 4c8bbae0010000 mov r15, qword ptr [rdx + 0x1e0]
140b3c9d7: 48f7d1 not rcx
140b3c9da: 488d0c49 lea rcx, [rcx + rcx*2]
140b3c9de: 48c1e105 shl rcx, 5
140b3c9e2: 49837c0f2000 cmp qword ptr [r15 + rcx + 0x20], 0
140b3c9e8: 0f8492000000 je 0x140b3ca80
140b3c9ee: 4901cf add r15, rcx
140b3c9f1: 498b7f10 mov rdi, qword ptr [r15 + 0x10]
140b3c9f5: 8b8ab0050000 mov ecx, dword ptr [rdx + 0x5b0]
140b3c9fb: 89585c mov dword ptr [rax + 0x5c], ebx
140b3c9fe: 4d8b6630 mov r12, qword ptr [r14 + 0x30]
140b3ca02: 4d8b6e40 mov r13, qword ptr [r14 + 0x40]
140b3ca06: 4d3b6620 cmp r12, qword ptr [r14 + 0x20]
140b3ca0a: 7510 jne 0x140b3ca1c
140b3ca0c: 498d4e20 lea rcx, [r14 + 0x20]
140b3ca10: 488d1531d70f02 lea rdx, [rip + 0x20fd731]
140b3ca17: e8145ca2ff call 0x140562630 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<iced_core::element::Element<enum2$<tg_gpu_selector::vulkan_config::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<std::sync::mpsc::Receiver<enum2$<tg_package_integrity::verify_package_manifest::CheckOutcome> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<system_critters::sheep::sheep_pet_animation::PlaySheepPetAnimation>,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstan
140b3ca1c: 498b4628 mov rax, qword ptr [r14 + 0x28]
140b3ca20: 4c89e1 mov rcx, r12
140b3ca23: 48c1e104 shl rcx, 4
140b3ca27: 4c892c08 mov qword ptr [rax + rcx], r13
140b3ca2b: c644080800 mov byte ptr [rax + rcx + 8], 0
140b3ca30: 49ffc4 inc r12
140b3ca33: 4d896630 mov qword ptr [r14 + 0x30], r12
140b3ca37: 49ff4640 inc qword ptr [r14 + 0x40]
140b3ca3b: 41895f5c mov dword ptr [r15 + 0x5c], ebx
140b3ca3f: 4889f9 mov rcx, rdi
140b3ca42: e8f96df0ff call 0x140a43840 ; country_core::systems::save::save_game::AutoSaveState::close_file
140b3ca47: 899e50010000 mov dword ptr [rsi + 0x150], ebx
140b3ca4d: 31c0 xor eax, eax
140b3ca4f: 4881c480000000 add rsp, 0x80
140b3ca56: 5b pop rbx
140b3ca57: 5f pop rdi
140b3ca58: 5e pop rsi
140b3ca59: 415c pop r12
140b3ca5b: 415d pop r13
140b3ca5d: 415e pop r14
140b3ca5f: 415f pop r15
140b3ca61: c3 ret 
140b3ca62: 4881c6b8000000 add rsi, 0xb8
140b3ca69: 488d055c1cfb01 lea rax, [rip + 0x1fb1c5c] ; 0x142aee6cc 'bevy_ecs::event::collections::Events<country_core::resources::events::ClearCanvasCmd>'
140b3ca70: 4889442420 mov qword ptr [rsp + 0x20], rax
140b3ca75: 48c744242855000000 mov qword ptr [rsp + 0x28], 0x55
140b3ca7e: eb1c jmp 0x140b3ca9c
140b3ca80: 4881c6b8000000 add rsi, 0xb8
140b3ca87: 488d05e71cfb01 lea rax, [rip + 0x1fb1ce7] ; 0x142aee775 'country_core::systems::save::save_game::AutoSaveStateSystem built with a different world than the one it was added to.'
140b3ca8e: 4889442420 mov qword ptr [rsp + 0x20], rax
140b3ca93: 48c744242835000000 mov qword ptr [rsp + 0x28], 0x35
140b3ca9c: 4889742430 mov qword ptr [rsp + 0x30], rsi
140b3caa1: 488d0528f04fff lea rax, [rip - 0xb00fd8]
140b3caa8: 4889442438 mov qword ptr [rsp + 0x38], rax
140b3caad: 488d442420 lea rax, [rsp + 0x20]
140b3cab2: 4889442440 mov qword ptr [rsp + 0x40], rax
140b3cab7: 488d05d2d35cff lea rax, [rip - 0xa32c2e]
140b3cabe: 4889442448 mov qword ptr [rsp + 0x48], rax
140b3cac3: 488d059e1afb01 lea rax, [rip + 0x1fb1a9e]
140b3caca: 4889442450 mov qword ptr [rsp + 0x50], rax
140b3cacf: 48c744245802000000 mov qword ptr [rsp + 0x58], 2
140b3cad8: 48c744247000000000 mov qword ptr [rsp + 0x70], 0
140b3cae1: 488d442430 lea rax, [rsp + 0x30]
140b3cae6: 4889442460 mov qword ptr [rsp + 0x60], rax
140b3caeb: 48c744246802000000 mov qword ptr [rsp + 0x68], 2
140b3caf4: 488d152d1cfb01 lea rdx, [rip + 0x1fb1c2d]
140b3cafb: 488d4c2450 lea rcx, [rsp + 0x50]
140b3cb00: e88bc8d901 call 0x1428d9390 ; core::panicking::panic_fmt
140b3cb05: 488d0d7c1dfb01 lea rcx, [rip + 0x1fb1d7c] ; 0x142aee888 "System's state was not found. Did you forget to initialize this system before running it?"
140b3cb0c: 4c8d05d51dfb01 lea r8, [rip + 0x1fb1dd5]
140b3cb13: ba59000000 mov edx, 0x59
140b3cb18: e813c8d901 call 0x1428d9330 ; core::option::expect_failed
140b3cb1d: cc int3 
