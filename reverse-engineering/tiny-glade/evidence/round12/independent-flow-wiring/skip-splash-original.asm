14059d450: push rbp
14059d451: push r15
14059d453: push r14
14059d455: push r13
14059d457: push r12
14059d459: push rsi
14059d45a: push rdi
14059d45b: push rbx
14059d45c: mov eax, 0x37b8
14059d461: call 0x14289aa40 ; __chkstk
14059d466: sub rsp, rax
14059d469: lea rbp, [rsp + 0x80]
14059d471: movaps xmmword ptr [rbp + 0x3720], xmm15
14059d479: movaps xmmword ptr [rbp + 0x3710], xmm14
14059d481: movaps xmmword ptr [rbp + 0x3700], xmm13
14059d489: movaps xmmword ptr [rbp + 0x36f0], xmm12
14059d491: movaps xmmword ptr [rbp + 0x36e0], xmm11
14059d499: movaps xmmword ptr [rbp + 0x36d0], xmm10
14059d4a1: movaps xmmword ptr [rbp + 0x36c0], xmm9
14059d4a9: movaps xmmword ptr [rbp + 0x36b0], xmm8
14059d4b1: movaps xmmword ptr [rbp + 0x36a0], xmm7
14059d4b8: movaps xmmword ptr [rbp + 0x3690], xmm6
14059d4bf: and rsp, 0xffffffffffffff80
14059d4c3: mov rbx, rsp
14059d4c6: mov qword ptr [rbx + 0x3700], rbp
14059d4cd: mov qword ptr [rbp + 0x3688], 0xfffffffffffffffe
14059d4d8: mov qword ptr [rbx + 0xc8], r9
14059d4df: mov rax, qword ptr [rbp + 0x37a0]
14059d4e6: movups xmm0, xmmword ptr [rbp + 0x37d8]
14059d4ed: movups xmm1, xmmword ptr [rbp + 0x37c8]
14059d4f4: mov qword ptr [rbx + 0x2c0], rcx
14059d4fb: mov qword ptr [rbx + 0x2c8], rdx
14059d502: mov qword ptr [rbx + 0x140], r8
14059d509: movups xmmword ptr [rbx + 0x328], xmm1
14059d510: movups xmmword ptr [rbx + 0x160], xmm0
14059d517: movupd xmm0, xmmword ptr [rax + 0xae0]
14059d51f: movups xmm1, xmmword ptr [rax + 0xaf0]
14059d526: movaps xmmword ptr [rbx + 0x360], xmm1
14059d52d: movapd xmmword ptr [rbx + 0x350], xmm0
14059d535: mov rcx, qword ptr [rax + 0xb00]
14059d53c: mov qword ptr [rbx + 0x2b8], rcx
14059d543: mov rcx, qword ptr [rax + 0xb08]
14059d54a: mov qword ptr [rbx + 0x2b0], rcx
14059d551: mov qword ptr [rbx + 0xb00], 1
14059d55c: mov qword ptr [rbx + 0xb08], 1
14059d567: lea rcx, [rbx + 0xb10]
14059d56e: mov r8d, 0xae0
14059d574: mov rdx, rax
14059d577: call 0x142923c00 ; 
14059d57c: movzx eax, byte ptr [rip + 0x3097686]
14059d583: mov ecx, 0xaf0
14059d588: mov edx, 8
14059d58d: call 0x140613c10 ; 
14059d592: test rax, rax
14059d595: je 0x1405a6a26
14059d59b: mov rsi, rax
14059d59e: lea rdx, [rbx + 0xb00]
14059d5a5: mov r8d, 0xaf0
14059d5ab: mov rcx, rax
14059d5ae: call 0x142923c00 ; 
14059d5b3: mov qword ptr [rbx + 0x138], rsi
14059d5ba: cmp byte ptr [rsi + 0xae8], 1
14059d5c1: jne 0x14059d63c
14059d5c3: add rsi, 0x10
14059d5c7: mov byte ptr [rbx + 0xbc], 1
14059d5ce: mov byte ptr [rbx + 0xbd], 1
14059d5d5: mov byte ptr [rbx + 0xbe], 1
14059d5dc: mov byte ptr [rbx + 0xbf], 1
14059d5e3: mov byte ptr [rbx + 0xc0], 1
14059d5ea: mov byte ptr [rbx + 0xc1], 1
14059d5f1: mov rcx, rsi
14059d5f4: call 0x140bf8950 ; rhapsody::raw::device::GraphicsDevice::physical_device
14059d5f9: add rax, 0x68
14059d5fd: mov byte ptr [rbx + 0xbc], 1
14059d604: mov byte ptr [rbx + 0xbd], 1
14059d60b: mov byte ptr [rbx + 0xbe], 1
14059d612: mov byte ptr [rbx + 0xbf], 1
14059d619: mov byte ptr [rbx + 0xc0], 1
14059d620: mov byte ptr [rbx + 0xc1], 1
14059d627: mov rcx, rax
14059d62a: call 0x140c001e0 ; rhapsody::raw::physical_device::DeviceDriverVersion::is_amd
14059d62f: test al, al
14059d631: je 0x14059d646
14059d633: mov rax, qword ptr [rip + 0x308ea8e]
14059d63a: jmp 0x14059d643
14059d63c: mov rax, qword ptr [rip + 0x308ea8d]
14059d643: mov byte ptr [rax], 1
14059d646: mov rcx, qword ptr [rbx + 0x138]
14059d64d: add rcx, 0x10
14059d651: mov byte ptr [rbx + 0xbc], 1
14059d658: mov byte ptr [rbx + 0xbd], 1
14059d65f: mov byte ptr [rbx + 0xbe], 1
14059d666: mov byte ptr [rbx + 0xbf], 1
14059d66d: mov byte ptr [rbx + 0xc0], 1
14059d674: mov byte ptr [rbx + 0xc1], 1
14059d67b: call 0x1405898a0 ; tiny_glade::panic_reporter::panic_screenshot::create_graphics_resources
14059d680: mov rax, qword ptr [rbx + 0x140]
14059d687: mov rcx, qword ptr [rax + 0x18]
14059d68b: xorps xmm6, xmm6
14059d68e: movaps xmmword ptr [rbx + 0x440], xmm6
14059d695: lea rdx, [rbx + 0x440]
14059d69c: call qword ptr [rip + 0x2bf51de]
14059d6a2: test eax, eax
14059d6a4: je 0x1405a6520
14059d6aa: mov edi, dword ptr [rbx + 0x444]
14059d6b0: mov eax, dword ptr [rbx + 0x448]
14059d6b6: sub eax, dword ptr [rbx + 0x440]
14059d6bc: mov r14d, dword ptr [rbx + 0x44c]
14059d6c3: xorps xmm0, xmm0
14059d6c6: cvtsi2sd xmm0, rax
14059d6cb: mov byte ptr [rbx + 0xbc], 1
14059d6d2: mov byte ptr [rbx + 0xbd], 1
14059d6d9: mov byte ptr [rbx + 0xbe], 1
14059d6e0: mov byte ptr [rbx + 0xbf], 1
14059d6e7: mov byte ptr [rbx + 0xc0], 1
14059d6ee: mov byte ptr [rbx + 0xc1], 1
14059d6f5: call 0x1406977f0 ; winit::dpi::impl$2::from_f64
14059d6fa: mov esi, eax
14059d6fc: sub r14d, edi
14059d6ff: xorps xmm0, xmm0
14059d702: cvtsi2sd xmm0, r14
14059d707: mov byte ptr [rbx + 0xbc], 1
14059d70e: mov byte ptr [rbx + 0xbd], 1
14059d715: mov byte ptr [rbx + 0xbe], 1
14059d71c: mov byte ptr [rbx + 0xbf], 1
14059d723: mov byte ptr [rbx + 0xc0], 1
14059d72a: mov byte ptr [rbx + 0xc1], 1
14059d731: call 0x1406977f0 ; winit::dpi::impl$2::from_f64
14059d736: shl rax, 0x20
14059d73a: mov ecx, esi
14059d73c: or rcx, rax
14059d73f: mov byte ptr [rbx + 0xbc], 1
14059d746: mov byte ptr [rbx + 0xbd], 1
14059d74d: mov byte ptr [rbx + 0xbe], 1
14059d754: mov byte ptr [rbx + 0xbf], 1
14059d75b: mov byte ptr [rbx + 0xc0], 1
14059d762: mov byte ptr [rbx + 0xc1], 1
14059d769: call 0x140949bf0 ; country_core::resources::window::WindowSize::new
14059d76e: mov dword ptr [rbx + 0x148], eax
14059d774: mov dword ptr [rbx + 0x14c], edx
14059d77a: mov rcx, qword ptr [rbp + 0x37b0]
14059d781: movzx eax, byte ptr [rcx + 0xf4]
14059d788: movss xmm0, dword ptr [rcx + 0xec]
14059d790: mov dword ptr [rbx + 0xb08], eax
14059d796: movss dword ptr [rbx + 0xb0c], xmm0
14059d79e: movabs r13, 0x8000000000000000
14059d7a8: mov qword ptr [rbx + 0xb00], r13
14059d7af: mov byte ptr [rbx + 0xbc], 1
14059d7b6: mov byte ptr [rbx + 0xbd], 1
14059d7bd: mov byte ptr [rbx + 0xbe], 1
14059d7c4: mov byte ptr [rbx + 0xbf], 1
14059d7cb: mov byte ptr [rbx + 0xc0], 1
14059d7d2: mov byte ptr [rbx + 0xc1], 1
14059d7d9: lea rsi, [rbx + 0x440]
14059d7e0: lea rdx, [rbx + 0x160]
14059d7e7: lea r8, [rbx + 0xb00]
14059d7ee: mov rcx, rsi
14059d7f1: call 0x1403c5a10 ; std::sync::mpmc::Sender::send<enum2$<tiny_glade::event_loop::GameEvent> >
14059d7f6: movabs rax, 0x8000000000000007
14059d800: cmp qword ptr [rbx + 0x440], rax
14059d807: jne 0x1405a6a3a
14059d80d: call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
14059d812: mov rdi, rax
14059d815: test byte ptr [rax], 1
14059d818: je 0x14059d82b
14059d81a: mov rcx, rdi
14059d81d: add rcx, 8
14059d821: mov rax, qword ptr [rdi + 8]
14059d825: mov rdx, qword ptr [rdi + 0x10]
14059d829: jmp 0x14059d870
14059d82b: mov byte ptr [rbx + 0xbc], 1
14059d832: mov byte ptr [rbx + 0xbd], 1
14059d839: mov byte ptr [rbx + 0xbe], 1
14059d840: mov byte ptr [rbx + 0xbf], 1
14059d847: mov byte ptr [rbx + 0xc0], 1
14059d84e: mov byte ptr [rbx + 0xc1], 1
14059d855: call 0x140d64e00 ; std::sys::random::hashmap_random_keys
14059d85a: mov rcx, rdi
14059d85d: add rcx, 8
14059d861: mov qword ptr [rdi], 1
14059d868: mov qword ptr [rdi + 8], rax
14059d86c: mov qword ptr [rdi + 0x10], rdx
14059d870: lea r8, [rax + 1]
14059d874: mov qword ptr [rcx], r8
14059d877: movups xmm6, xmmword ptr [rip + 0x2460e32]
14059d87e: movaps xmmword ptr [rbx + 0xb10], xmm6
14059d885: movups xmm7, xmmword ptr [rip + 0x2460e14]
14059d88c: movaps xmmword ptr [rbx + 0xb00], xmm7
14059d893: mov qword ptr [rbx + 0xb20], rax
14059d89a: mov qword ptr [rbx + 0xb28], rdx
14059d8a1: call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
14059d8a6: mov rdi, rax
14059d8a9: test byte ptr [rax], 1
14059d8ac: je 0x14059d8bf
14059d8ae: mov rcx, rdi
14059d8b1: add rcx, 8
14059d8b5: mov rax, qword ptr [rdi + 8]
14059d8b9: mov rdx, qword ptr [rdi + 0x10]
14059d8bd: jmp 0x14059d8da
14059d8bf: call 0x140d64e00 ; std::sys::random::hashmap_random_keys
14059d8c4: mov rcx, rdi
14059d8c7: add rcx, 8
14059d8cb: mov qword ptr [rdi], 1
14059d8d2: mov qword ptr [rdi + 8], rax
14059d8d6: mov qword ptr [rdi + 0x10], rdx
14059d8da: lea r8, [rax + 1]
14059d8de: mov qword ptr [rcx], r8
14059d8e1: movaps xmm0, xmmword ptr [rbx + 0xb00]
14059d8e8: movaps xmm1, xmmword ptr [rbx + 0xb10]
14059d8ef: movaps xmm2, xmmword ptr [rbx + 0xb20]
14059d8f6: movaps xmmword ptr [rbx + 0xac0], xmm2
14059d8fd: movaps xmmword ptr [rbx + 0xab0], xmm1
14059d904: movaps xmmword ptr [rbx + 0xaa0], xmm0
14059d90b: movaps xmmword ptr [rbx + 0xad0], xmm7
14059d912: movaps xmmword ptr [rbx + 0xae0], xmm6
14059d919: mov qword ptr [rbx + 0xaf0], rax
14059d920: mov qword ptr [rbx + 0xaf8], rdx
14059d927: mov rdx, qword ptr [rbp + 0x37b8]
14059d92e: mov rax, qword ptr [rdx]
14059d931: lock inc qword ptr [rax]
14059d935: jle 0x1405a7040
14059d93b: mov rcx, qword ptr [rdx + 8]
14059d93f: lock inc qword ptr [rcx]
14059d943: jle 0x1405a7040
14059d949: mov rdx, qword ptr [rdx + 0x10]
14059d94d: lock inc qword ptr [rdx]
14059d951: jle 0x1405a7040
14059d957: mov qword ptr [rbx + 0xb00], rax
14059d95e: mov qword ptr [rbx + 0xb08], rcx
14059d965: mov qword ptr [rbx + 0xb10], rdx
14059d96c: xor ecx, ecx
14059d96e: call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > > >
14059d973: test rax, rax
14059d976: je 0x1405a6adc
14059d97c: xorps xmm0, xmm0
14059d97f: movups xmmword ptr [rbx + 0x2108], xmm0
14059d986: movups xmm1, xmmword ptr [rax]
14059d989: inc qword ptr [rax]
14059d98c: mov rax, qword ptr [rbx + 0xb10]
14059d993: mov qword ptr [rbx + 0x2150], rax
14059d99a: movups xmm2, xmmword ptr [rbx + 0xb00]
14059d9a1: movups xmmword ptr [rbx + 0x2140], xmm2
14059d9a8: movups xmm2, xmmword ptr [rip + 0x239d741]
14059d9af: movups xmmword ptr [rbx + 0x2158], xmm2
14059d9b6: movups xmm2, xmmword ptr [rip + 0x239d743]
14059d9bd: movups xmmword ptr [rbx + 0x2168], xmm2
14059d9c4: movups xmmword ptr [rbx + 0x2178], xmm1
14059d9cb: mov qword ptr [rbx + 0x20e0], 0
14059d9d6: mov qword ptr [rbx + 0x20e8], 8
14059d9e1: movups xmmword ptr [rbx + 0x20f0], xmm0
14059d9e8: mov qword ptr [rbx + 0x2100], 8
14059d9f3: mov qword ptr [rbx + 0x2118], 8
14059d9fe: movups xmmword ptr [rbx + 0x2120], xmm0
14059da05: mov qword ptr [rbx + 0x2130], 8
14059da10: mov qword ptr [rbx + 0x2138], 0
14059da1b: mov rax, qword ptr [rip + 0x308e986]
14059da22: mov rax, qword ptr [rax]
14059da25: cmp rax, 3
14059da29: jb 0x14059db6b
14059da2f: lea rax, [rip + 0x2461d52]
14059da36: mov qword ptr [rbx + 0x440], rax
14059da3d: lea rax, [rip - 0x493bb4]
14059da44: mov qword ptr [rbx + 0x448], rax
14059da4b: mov byte ptr [rbx + 0xdc], 1
14059da52: mov byte ptr [rbx + 0xdd], 1
14059da59: mov byte ptr [rbx + 0xde], 1
14059da60: mov byte ptr [rbx + 0xdf], 1
14059da67: mov byte ptr [rbx + 0xe0], 1
14059da6e: mov byte ptr [rbx + 0xe1], 1
14059da75: mov byte ptr [rbx + 0xe2], 1
14059da7c: lea rcx, [rip + 0x2461d35]
14059da83: call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
14059da88: movups xmm0, xmmword ptr [rax]
14059da8b: mov eax, dword ptr [rax + 0x10]
14059da8e: mov qword ptr [rbx + 0xb30], 3
14059da99: lea rcx, [rip + 0x2461950]
14059daa0: mov qword ptr [rbx + 0xb38], rcx
14059daa7: mov qword ptr [rbx + 0xb40], 0x14
14059dab2: lea rdx, [rip + 0x2461cef]
14059dab9: mov qword ptr [rbx + 0xb50], rdx
14059dac0: mov qword ptr [rbx + 0xb58], 1
14059dacb: mov qword ptr [rbx + 0xb60], rsi
14059dad2: mov qword ptr [rbx + 0xb68], 1
14059dadd: mov qword ptr [rbx + 0xb70], 0
14059dae8: mov qword ptr [rbx + 0xb00], 0
14059daf3: mov qword ptr [rbx + 0xb08], rcx
14059dafa: mov qword ptr [rbx + 0xb10], 0x14
14059db05: mov qword ptr [rbx + 0xb18], 0
14059db10: movups xmmword ptr [rbx + 0xb20], xmm0
14059db17: mov dword ptr [rbx + 0xb48], 1
14059db21: mov dword ptr [rbx + 0xb4c], eax
14059db27: mov byte ptr [rbx + 0xdc], 1
14059db2e: mov byte ptr [rbx + 0xdd], 1
14059db35: mov byte ptr [rbx + 0xde], 1
14059db3c: mov byte ptr [rbx + 0xdf], 1
14059db43: mov byte ptr [rbx + 0xe0], 1
14059db4a: mov byte ptr [rbx + 0xe1], 1
14059db51: mov byte ptr [rbx + 0xe2], 1
14059db58: lea rcx, [rbx + 0x370]
14059db5f: lea rdx, [rbx + 0xb00]
14059db66: call 0x140d4a330 ; log::__private_api::impl$2::log
14059db6b: mov byte ptr [rbx + 0xdc], 1
14059db72: mov byte ptr [rbx + 0xdd], 1
14059db79: mov byte ptr [rbx + 0xde], 1
14059db80: mov byte ptr [rbx + 0xdf], 1
14059db87: mov byte ptr [rbx + 0xe0], 1
14059db8e: mov byte ptr [rbx + 0xe1], 1
14059db95: mov byte ptr [rbx + 0xe2], 1
14059db9c: lea rcx, [rbx + 0x148]
14059dba3: call 0x140949c20 ; country_core::resources::window::WindowSize::aspect_ratio
14059dba8: mov byte ptr [rbx + 0xdc], 1
14059dbaf: mov byte ptr [rbx + 0xdd], 1
14059dbb6: mov byte ptr [rbx + 0xde], 1
14059dbbd: mov byte ptr [rbx + 0xdf], 1
14059dbc4: mov byte ptr [rbx + 0xe0], 1
14059dbcb: mov byte ptr [rbx + 0xe1], 1
14059dbd2: mov byte ptr [rbx + 0xe2], 1
14059dbd9: lea rcx, [rbx + 0x3220]
14059dbe0: movaps xmm1, xmm0
14059dbe3: call 0x14090ec70 ; country_core::systems::camera_rig::MainCamera::new
14059dbe8: movzx eax, byte ptr [rip + 0x309701a]
14059dbef: mov ecx, 0xb0
14059dbf4: mov edx, 8
14059dbf9: call 0x140613c10 ; 
14059dbfe: test rax, rax
14059dc01: je 0x1405a6aed
14059dc07: mov qword ptr [rax], 1
14059dc0e: mov qword ptr [rax + 8], 1
14059dc16: mov qword ptr [rax + 0x10], 0
14059dc1e: mov byte ptr [rax + 0x18], 0
14059dc22: mov qword ptr [rax + 0x20], r13
14059dc26: mov byte ptr [rax + 0xa0], 2
14059dc2d: mov byte ptr [rax + 0xa8], 0
14059dc34: mov qword ptr [rbx + 0x158], rax
14059dc3b: mov rax, qword ptr [rbp + 0x37c0]
14059dc42: cmp byte ptr [rax + 0x147], 1
14059dc49: jne 0x14059dc96
14059dc4b: mov byte ptr [rbx + 0xaa], 1
14059dc52: mov byte ptr [rbx + 0xab], 1
14059dc59: mov byte ptr [rbx + 0xac], 1
14059dc60: mov byte ptr [rbx + 0xad], 1
14059dc67: mov byte ptr [rbx + 0xae], 1
14059dc6e: mov byte ptr [rbx + 0xaf], 1
14059dc75: mov byte ptr [rbx + 0xb0], 1
14059dc7c: mov byte ptr [rbx + 0xb1], 1
14059dc83: mov byte ptr [rbx + 0xb2], 1
14059dc8a: mov rcx, qword ptr [rbx + 0xc8]
14059dc91: call 0x140328130 ; bevy_app::sub_app::SubApp::edit_schedule<bevy_app::main_schedule::Update,tiny_glade::run_game::run_game_thread::closure_env$0>
14059dc96: mov qword ptr [rbx + 0x440], 1
14059dca1: mov qword ptr [rbx + 0x448], 0xffffffffffffffff
14059dcac: mov qword ptr [rbx + 0x450], 0
14059dcb7: mov qword ptr [rbx + 0x460], 0
14059dcc2: mov qword ptr [rbx + 0x470], 1
14059dccd: mov qword ptr [rbx + 0x478], 4
14059dcd8: mov dword ptr [rbx + 0x480], 0x3e800000
14059dce2: mov qword ptr [rbx + 0x488], 0
14059dced: mov qword ptr [rbx + 0x498], 0
14059dcf8: mov qword ptr [rbx + 0x4a8], 1
14059dd03: mov qword ptr [rbx + 0x4b0], 4
14059dd0e: mov dword ptr [rbx + 0x4b8], 0x3e800000
14059dd18: mov qword ptr [rbx + 0x4c0], 0
14059dd23: mov qword ptr [rbx + 0x4d0], 0
14059dd2e: mov qword ptr [rbx + 0x4e0], 1
14059dd39: mov qword ptr [rbx + 0x4e8], 0xffffffffffffffff
14059dd44: mov dword ptr [rbx + 0x4f0], 0x3f800000
14059dd4e: mov byte ptr [rbx + 0x110], 1
14059dd55: mov rcx, qword ptr [rbx + 0xc8]
14059dd5c: call 0x140113ff0 ; bevy_app::app::App::plugins_state
14059dd61: cmp al, 1
14059dd63: ja 0x1405a6b39
14059dd69: lea rsi, [rbx + 0xb00]
14059dd70: lea rdx, [rbx + 0x440]
14059dd77: mov r8d, 0xb8
14059dd7d: mov rcx, rsi
14059dd80: call 0x142923c00 ; 
14059dd85: mov byte ptr [rbx + 0x110], 0
14059dd8c: lea r8, [rip + 0x2461a3d]
14059dd93: mov rcx, rsi
14059dd96: mov rsi, qword ptr [rbx + 0xc8]
14059dd9d: mov rdx, rsi
14059dda0: call 0x1400ec810 ; bevy_app::plugin::sealed::impl$6::add_to_app<bevy_app::plugin::sealed::PluginMarker,bevy_app::task_pool_plugin::TaskPoolPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::run_game::InputLitePlugin,bevy_app::plugin::sealed::PluginMarker,bevy_time::TimePlugin,bevy_app::plugin::sealed::PluginMarker,bevy_state::app::StatesPlugin>
14059dda5: mov byte ptr [rbx + 0xaa], 1
14059ddac: mov byte ptr [rbx + 0xab], 1
14059ddb3: mov byte ptr [rbx + 0xac], 1
14059ddba: mov byte ptr [rbx + 0xad], 1
14059ddc1: mov byte ptr [rbx + 0xae], 1
14059ddc8: mov byte ptr [rbx + 0xaf], 1
14059ddcf: mov byte ptr [rbx + 0xb0], 1
14059ddd6: mov byte ptr [rbx + 0xb1], 1
14059dddd: mov byte ptr [rbx + 0xb2], 1
14059dde4: mov rcx, rsi
14059dde7: call 0x140113ff0 ; bevy_app::app::App::plugins_state
14059ddec: cmp al, 1
14059ddee: ja 0x1405a6b86
14059ddf4: mov byte ptr [rbx + 0xaa], 1
14059ddfb: mov byte ptr [rbx + 0xab], 1
14059de02: mov byte ptr [rbx + 0xac], 1
14059de09: mov byte ptr [rbx + 0xad], 1
14059de10: mov byte ptr [rbx + 0xae], 1
14059de17: mov byte ptr [rbx + 0xaf], 1
14059de1e: mov byte ptr [rbx + 0xb0], 1
14059de25: mov byte ptr [rbx + 0xb1], 1
14059de2c: mov byte ptr [rbx + 0xb2], 1
14059de33: lea rdx, [rip + 0x24619ae]
14059de3a: mov rcx, qword ptr [rbx + 0xc8]
14059de41: call 0x14032b660 ; bevy_app::plugin::sealed::impl$15::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::run_game::ReflectionListingPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::game_loop::GameLoopPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::main_camera::MainCameraPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::Ui2dPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::mei::core::MeiPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::ToolbarPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::session::SessionFlowPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::menu::MenuFlowPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::painterly::PainterlyPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::screenshot::ScreenshotPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::lut_gallery::LutGalleryPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::gui::settings::SettingsMenuPlugin>
14059de46: mov byte ptr [rbx + 0xaa], 1
14059de4d: mov byte ptr [rbx + 0xab], 1
14059de54: mov byte ptr [rbx + 0xac], 1
14059de5b: mov byte ptr [rbx + 0xad], 1
14059de62: mov byte ptr [rbx + 0xae], 1
14059de69: mov byte ptr [rbx + 0xaf], 1
14059de70: mov byte ptr [rbx + 0xb0], 1
14059de77: mov byte ptr [rbx + 0xb1], 1
14059de7e: mov byte ptr [rbx + 0xb2], 1
14059de85: mov rcx, qword ptr [rbx + 0xc8]
14059de8c: call 0x140113ff0 ; bevy_app::app::App::plugins_state
14059de91: cmp al, 1
14059de93: ja 0x1405a6c0b
14059de99: mov byte ptr [rbx + 0xaa], 1
14059dea0: mov byte ptr [rbx + 0xab], 1
14059dea7: mov byte ptr [rbx + 0xac], 1
14059deae: mov byte ptr [rbx + 0xad], 1
14059deb5: mov byte ptr [rbx + 0xae], 1
14059debc: mov byte ptr [rbx + 0xaf], 1
14059dec3: mov byte ptr [rbx + 0xb0], 1
14059deca: mov byte ptr [rbx + 0xb1], 1
14059ded1: mov byte ptr [rbx + 0xb2], 1
14059ded8: lea r9, [rip + 0x2461921]
14059dedf: xor ecx, ecx
14059dee1: xor edx, edx
14059dee3: mov r8, qword ptr [rbx + 0xc8]
14059deea: call 0x1405b0140 ; bevy_app::plugin::sealed::impl$16::add_to_app<bevy_app::plugin::sealed::PluginMarker,feedback_reporter::FeedbackReporterPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::feedback_report_resources::FeedbackReportBackgroundPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::menu_background::MenuBackgroundPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::paths::PathPlugin,bevy_app::plugin::sealed::PluginMarker,system_water::WaterPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::garden::GardenPlugin,bevy_app::plugin::sealed::PluginMarker,system_tree::TreePlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::prefab::PrefabPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::vermeer::VermeerPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::draw_shaders::clouds::CloudPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::logo_overlay::LogoOverlayPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::photo_mode::PhotoModePlugin>
14059deef: mov byte ptr [rbx + 0xaa], 1
14059def6: mov byte ptr [rbx + 0xab], 1
14059defd: mov byte ptr [rbx + 0xac], 1
14059df04: mov byte ptr [rbx + 0xad], 1
14059df0b: mov byte ptr [rbx + 0xae], 1
14059df12: mov byte ptr [rbx + 0xaf], 1
14059df19: mov byte ptr [rbx + 0xb0], 1
14059df20: mov byte ptr [rbx + 0xb1], 1
14059df27: mov byte ptr [rbx + 0xb2], 1
14059df2e: mov rcx, qword ptr [rbx + 0xc8]
14059df35: call 0x140113ff0 ; bevy_app::app::App::plugins_state
14059df3a: cmp al, 1
14059df3c: ja 0x1405a6c90
14059df42: mov byte ptr [rbx + 0xaa], 1
14059df49: mov byte ptr [rbx + 0xab], 1
14059df50: mov byte ptr [rbx + 0xac], 1
14059df57: mov byte ptr [rbx + 0xad], 1
14059df5e: mov byte ptr [rbx + 0xae], 1
14059df65: mov byte ptr [rbx + 0xaf], 1
14059df6c: mov byte ptr [rbx + 0xb0], 1
14059df73: mov byte ptr [rbx + 0xb1], 1
14059df7a: mov byte ptr [rbx + 0xb2], 1
14059df81: lea rdx, [rip + 0x2461890]
14059df88: mov rcx, qword ptr [rbx + 0xc8]
14059df8f: call 0x14030a670 ; bevy_app::plugin::sealed::impl$12::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::decorator_plugin::DecoratorPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::backend::icc::IccProfilePlugin,bevy_app::plugin::sealed::PluginMarker,system_critters::CritterPlugin,bevy_app::plugin::sealed::PluginMarker,system_critters::sheep::SheepPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::gpu_lut::GpuLutPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::components::drawable::PrevTransformPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::render::fsr_plugin::plugin_impl::FsrPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::dlss_plugin::plugin_impl::DlssPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::rollercoaster::RollercoasterPlugin>
14059df94: mov byte ptr [rbx + 0xaa], 1
14059df9b: mov byte ptr [rbx + 0xab], 1
14059dfa2: mov byte ptr [rbx + 0xac], 1
14059dfa9: mov byte ptr [rbx + 0xad], 1
14059dfb0: mov byte ptr [rbx + 0xae], 1
14059dfb7: mov byte ptr [rbx + 0xaf], 1
14059dfbe: mov byte ptr [rbx + 0xb0], 1
14059dfc5: mov byte ptr [rbx + 0xb1], 1
14059dfcc: mov byte ptr [rbx + 0xb2], 1
14059dfd3: mov rcx, qword ptr [rbx + 0xc8]
14059dfda: call 0x1403258c0 ; bevy_state::app::impl$0::init_state<country_core::app_loading_state::AppLoadingState>
14059dfdf: mov rax, qword ptr [rbx + 0xc8]
14059dfe6: lea rcx, [rax + 0x38]
14059dfea: mov byte ptr [rbx + 0xaa], 1
14059dff1: mov byte ptr [rbx + 0xab], 1
14059dff8: mov byte ptr [rbx + 0xac], 1
14059dfff: mov byte ptr [rbx + 0xad], 1
14059e006: mov byte ptr [rbx + 0xae], 1
14059e00d: mov byte ptr [rbx + 0xaf], 1
14059e014: mov byte ptr [rbx + 0xb0], 1
14059e01b: mov byte ptr [rbx + 0xb1], 1
14059e022: mov byte ptr [rbx + 0xb2], 1
14059e029: movabs rdx, 0x2e383ebb9376bfa1
14059e033: movabs r8, 0x96b847e2c17c833c
14059e03d: mov qword ptr [rbx + 0xe8], rcx
14059e044: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
14059e049: test rax, rax
14059e04c: je 0x14059e096
14059e04e: mov rax, qword ptr [rbx + 0xc8]
14059e055: cmp qword ptr [rax + 0x218], rdx
14059e05c: jbe 0x14059e096
14059e05e: mov rax, qword ptr [rbx + 0xc8]
14059e065: mov rax, qword ptr [rax + 0x210]
14059e06c: mov rax, qword ptr [rax + rdx*8]
14059e070: test rax, rax
14059e073: je 0x14059e096
14059e075: mov rcx, qword ptr [rbx + 0xc8]
14059e07c: mov rcx, qword ptr [rcx + 0x1e0]
14059e083: not rax
14059e086: lea rax, [rax + rax*2]
14059e08a: shl rax, 5
14059e08e: cmp qword ptr [rcx + rax + 0x20], 0
14059e094: jne 0x14059e0e1
14059e096: mov byte ptr [rbx + 0xaa], 1
14059e09d: mov byte ptr [rbx + 0xab], 1
14059e0a4: mov byte ptr [rbx + 0xac], 1
14059e0ab: mov byte ptr [rbx + 0xad], 1
14059e0b2: mov byte ptr [rbx + 0xae], 1
14059e0b9: mov byte ptr [rbx + 0xaf], 1
14059e0c0: mov byte ptr [rbx + 0xb0], 1
14059e0c7: mov byte ptr [rbx + 0xb1], 1
14059e0ce: mov byte ptr [rbx + 0xb2], 1
14059e0d5: mov rcx, qword ptr [rbx + 0xc8]
14059e0dc: call 0x140566ee0 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::resources::events::ShutdownGameCmd>
14059e0e1: mov byte ptr [rbx + 0xaa], 1
14059e0e8: mov byte ptr [rbx + 0xab], 1
14059e0ef: mov byte ptr [rbx + 0xac], 1
14059e0f6: mov byte ptr [rbx + 0xad], 1
14059e0fd: mov byte ptr [rbx + 0xae], 1
14059e104: mov byte ptr [rbx + 0xaf], 1
14059e10b: mov byte ptr [rbx + 0xb0], 1
14059e112: mov byte ptr [rbx + 0xb1], 1
14059e119: mov byte ptr [rbx + 0xb2], 1
14059e120: movabs rdx, 0x47f704a227dd9e87
14059e12a: movabs r8, 0xc1485275b097c794
14059e134: mov rcx, qword ptr [rbx + 0xe8]
14059e13b: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
14059e140: test rax, rax
14059e143: je 0x14059e18d
14059e145: mov rax, qword ptr [rbx + 0xc8]
14059e14c: cmp qword ptr [rax + 0x218], rdx
14059e153: jbe 0x14059e18d
14059e155: mov rax, qword ptr [rbx + 0xc8]
14059e15c: mov rax, qword ptr [rax + 0x210]
14059e163: mov rax, qword ptr [rax + rdx*8]
14059e167: test rax, rax
14059e16a: je 0x14059e18d
14059e16c: mov rcx, qword ptr [rbx + 0xc8]
14059e173: mov rcx, qword ptr [rcx + 0x1e0]
14059e17a: not rax
14059e17d: lea rax, [rax + rax*2]
14059e181: shl rax, 5
14059e185: cmp qword ptr [rcx + rax + 0x20], 0
14059e18b: jne 0x14059e1d8
14059e18d: mov byte ptr [rbx + 0xaa], 1
14059e194: mov byte ptr [rbx + 0xab], 1
14059e19b: mov byte ptr [rbx + 0xac], 1
14059e1a2: mov byte ptr [rbx + 0xad], 1
14059e1a9: mov byte ptr [rbx + 0xae], 1
14059e1b0: mov byte ptr [rbx + 0xaf], 1
14059e1b7: mov byte ptr [rbx + 0xb0], 1
14059e1be: mov byte ptr [rbx + 0xb1], 1
14059e1c5: mov byte ptr [rbx + 0xb2], 1
14059e1cc: mov rcx, qword ptr [rbx + 0xc8]
14059e1d3: call 0x14056b920 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::resources::window::CursorMovedScreenSpace>
14059e1d8: mov byte ptr [rbx + 0xaa], 1
14059e1df: mov byte ptr [rbx + 0xab], 1
14059e1e6: mov byte ptr [rbx + 0xac], 1
14059e1ed: mov byte ptr [rbx + 0xad], 1
14059e1f4: mov byte ptr [rbx + 0xae], 1
14059e1fb: mov byte ptr [rbx + 0xaf], 1
14059e202: mov byte ptr [rbx + 0xb0], 1
14059e209: mov byte ptr [rbx + 0xb1], 1
14059e210: mov byte ptr [rbx + 0xb2], 1
14059e217: movabs rdx, 0xb5215f98f93b0bd0
14059e221: movabs r8, 0xa52829b46b4ee6bb
14059e22b: mov rcx, qword ptr [rbx + 0xe8]
14059e232: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
14059e237: test rax, rax
14059e23a: je 0x14059e284
14059e23c: mov rax, qword ptr [rbx + 0xc8]
14059e243: cmp qword ptr [rax + 0x218], rdx
14059e24a: jbe 0x14059e284
14059e24c: mov rax, qword ptr [rbx + 0xc8]
14059e253: mov rax, qword ptr [rax + 0x210]
14059e25a: mov rax, qword ptr [rax + rdx*8]
14059e25e: test rax, rax
14059e261: je 0x14059e284
14059e263: mov rcx, qword ptr [rbx + 0xc8]
14059e26a: mov rcx, qword ptr [rcx + 0x1e0]
14059e271: not rax
14059e274: lea rax, [rax + rax*2]
14059e278: shl rax, 5
14059e27c: cmp qword ptr [rcx + rax + 0x20], 0
14059e282: jne 0x14059e2cf
14059e284: mov byte ptr [rbx + 0xaa], 1
14059e28b: mov byte ptr [rbx + 0xab], 1
14059e292: mov byte ptr [rbx + 0xac], 1
14059e299: mov byte ptr [rbx + 0xad], 1
14059e2a0: mov byte ptr [rbx + 0xae], 1
14059e2a7: mov byte ptr [rbx + 0xaf], 1
14059e2ae: mov byte ptr [rbx + 0xb0], 1
14059e2b5: mov byte ptr [rbx + 0xb1], 1
14059e2bc: mov byte ptr [rbx + 0xb2], 1
14059e2c3: mov rcx, qword ptr [rbx + 0xc8]
14059e2ca: call 0x1405857b0 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::resources::window::AudioCursorMoved>
14059e2cf: mov byte ptr [rbx + 0xaa], 1
14059e2d6: mov byte ptr [rbx + 0xab], 1
14059e2dd: mov byte ptr [rbx + 0xac], 1
14059e2e4: mov byte ptr [rbx + 0xad], 1
14059e2eb: mov byte ptr [rbx + 0xae], 1
14059e2f2: mov byte ptr [rbx + 0xaf], 1
14059e2f9: mov byte ptr [rbx + 0xb0], 1
14059e300: mov byte ptr [rbx + 0xb1], 1
14059e307: mov byte ptr [rbx + 0xb2], 1
14059e30e: movabs rdx, 0xbb7c1673aee764cb
14059e318: movabs r8, 0x250aee383945e2d6
14059e322: mov rcx, qword ptr [rbx + 0xe8]
14059e329: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
14059e32e: test rax, rax
14059e331: je 0x14059e37b
14059e333: mov rax, qword ptr [rbx + 0xc8]
14059e33a: cmp qword ptr [rax + 0x218], rdx
14059e341: jbe 0x14059e37b
14059e343: mov rax, qword ptr [rbx + 0xc8]
14059e34a: mov rax, qword ptr [rax + 0x210]
14059e351: mov rax, qword ptr [rax + rdx*8]
14059e355: test rax, rax
14059e358: je 0x14059e37b
14059e35a: mov rcx, qword ptr [rbx + 0xc8]
14059e361: mov rcx, qword ptr [rcx + 0x1e0]
14059e368: not rax
14059e36b: lea rax, [rax + rax*2]
14059e36f: shl rax, 5
14059e373: cmp qword ptr [rcx + rax + 0x20], 0
14059e379: jne 0x14059e3c6
14059e37b: mov byte ptr [rbx + 0xaa], 1
14059e382: mov byte ptr [rbx + 0xab], 1
14059e389: mov byte ptr [rbx + 0xac], 1
14059e390: mov byte ptr [rbx + 0xad], 1
14059e397: mov byte ptr [rbx + 0xae], 1
14059e39e: mov byte ptr [rbx + 0xaf], 1
14059e3a5: mov byte ptr [rbx + 0xb0], 1
14059e3ac: mov byte ptr [rbx + 0xb1], 1
14059e3b3: mov byte ptr [rbx + 0xb2], 1
14059e3ba: mov rcx, qword ptr [rbx + 0xc8]
14059e3c1: call 0x1405687a0 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::resources::window::WindowFocusEvent>
14059e3c6: mov byte ptr [rbx + 0xaa], 1
14059e3cd: mov byte ptr [rbx + 0xab], 1
14059e3d4: mov byte ptr [rbx + 0xac], 1
14059e3db: mov byte ptr [rbx + 0xad], 1
14059e3e2: mov byte ptr [rbx + 0xae], 1
14059e3e9: mov byte ptr [rbx + 0xaf], 1
14059e3f0: mov byte ptr [rbx + 0xb0], 1
14059e3f7: mov byte ptr [rbx + 0xb1], 1
14059e3fe: mov byte ptr [rbx + 0xb2], 1
14059e405: movabs rdx, 0x998e2a38a408421c
14059e40f: movabs r8, 0xf8878e80950c1d68
14059e419: mov rcx, qword ptr [rbx + 0xe8]
14059e420: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
14059e425: test rax, rax
14059e428: je 0x14059e472
14059e42a: mov rax, qword ptr [rbx + 0xc8]
14059e431: cmp qword ptr [rax + 0x218], rdx
14059e438: jbe 0x14059e472
14059e43a: mov rax, qword ptr [rbx + 0xc8]
14059e441: mov rax, qword ptr [rax + 0x210]
14059e448: mov rax, qword ptr [rax + rdx*8]
14059e44c: test rax, rax
14059e44f: je 0x14059e472
14059e451: mov rcx, qword ptr [rbx + 0xc8]
14059e458: mov rcx, qword ptr [rcx + 0x1e0]
14059e45f: not rax
14059e462: lea rax, [rax + rax*2]
14059e466: shl rax, 5
14059e46a: cmp qword ptr [rcx + rax + 0x20], 0
14059e470: jne 0x14059e4bd
14059e472: mov byte ptr [rbx + 0xaa], 1
14059e479: mov byte ptr [rbx + 0xab], 1
14059e480: mov byte ptr [rbx + 0xac], 1
14059e487: mov byte ptr [rbx + 0xad], 1
14059e48e: mov byte ptr [rbx + 0xae], 1
14059e495: mov byte ptr [rbx + 0xaf], 1
14059e49c: mov byte ptr [rbx + 0xb0], 1
14059e4a3: mov byte ptr [rbx + 0xb1], 1
14059e4aa: mov byte ptr [rbx + 0xb2], 1
14059e4b1: mov rcx, qword ptr [rbx + 0xc8]
14059e4b8: call 0x14056d1e0 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::systems::brush_preview::OverwriteBrushPreview>
14059e4bd: mov byte ptr [rbx + 0xaa], 1
14059e4c4: mov byte ptr [rbx + 0xab], 1
14059e4cb: mov byte ptr [rbx + 0xac], 1
14059e4d2: mov byte ptr [rbx + 0xad], 1
14059e4d9: mov byte ptr [rbx + 0xae], 1
14059e4e0: mov byte ptr [rbx + 0xaf], 1
14059e4e7: mov byte ptr [rbx + 0xb0], 1
14059e4ee: mov byte ptr [rbx + 0xb1], 1
14059e4f5: mov byte ptr [rbx + 0xb2], 1
14059e4fc: movabs rdx, 0x5206f06e93414f24
14059e506: movabs r8, 0x39f4e4f76f731964
14059e510: mov rcx, qword ptr [rbx + 0xe8]
14059e517: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
14059e51c: test rax, rax
14059e51f: je 0x14059e569
14059e521: mov rax, qword ptr [rbx + 0xc8]
14059e528: cmp qword ptr [rax + 0x218], rdx
14059e52f: jbe 0x14059e569
14059e531: mov rax, qword ptr [rbx + 0xc8]
14059e538: mov rax, qword ptr [rax + 0x210]
14059e53f: mov rax, qword ptr [rax + rdx*8]
14059e543: test rax, rax
14059e546: je 0x14059e569
14059e548: mov rcx, qword ptr [rbx + 0xc8]
14059e54f: mov rcx, qword ptr [rcx + 0x1e0]
14059e556: not rax
14059e559: lea rax, [rax + rax*2]
14059e55d: shl rax, 5
14059e561: cmp qword ptr [rcx + rax + 0x20], 0
14059e567: jne 0x14059e5b4
14059e569: mov byte ptr [rbx + 0xaa], 1
14059e570: mov byte ptr [rbx + 0xab], 1
14059e577: mov byte ptr [rbx + 0xac], 1
14059e57e: mov byte ptr [rbx + 0xad], 1
14059e585: mov byte ptr [rbx + 0xae], 1
14059e58c: mov byte ptr [rbx + 0xaf], 1
14059e593: mov byte ptr [rbx + 0xb0], 1
14059e59a: mov byte ptr [rbx + 0xb1], 1
14059e5a1: mov byte ptr [rbx + 0xb2], 1
14059e5a8: mov rcx, qword ptr [rbx + 0xc8]
14059e5af: call 0x1405715f0 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::resources::events::ModeJustChanged>
14059e5b4: mov byte ptr [rbx + 0xaa], 1
14059e5bb: mov byte ptr [rbx + 0xab], 1
14059e5c2: mov byte ptr [rbx + 0xac], 1
14059e5c9: mov byte ptr [rbx + 0xad], 1
14059e5d0: mov byte ptr [rbx + 0xae], 1
14059e5d7: mov byte ptr [rbx + 0xaf], 1
14059e5de: mov byte ptr [rbx + 0xb0], 1
14059e5e5: mov byte ptr [rbx + 0xb1], 1
14059e5ec: mov byte ptr [rbx + 0xb2], 1
14059e5f3: movabs rdx, 0x7e476b44c6fa1e68
14059e5fd: movabs r8, 0xada174e41ab1f3e
14059e607: mov rcx, qword ptr [rbx + 0xe8]
14059e60e: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
14059e613: test rax, rax
14059e616: je 0x14059e660
14059e618: mov rax, qword ptr [rbx + 0xc8]
14059e61f: cmp qword ptr [rax + 0x218], rdx
14059e626: jbe 0x14059e660
14059e628: mov rax, qword ptr [rbx + 0xc8]
14059e62f: mov rax, qword ptr [rax + 0x210]
14059e636: mov rax, qword ptr [rax + rdx*8]
14059e63a: test rax, rax
14059e63d: je 0x14059e660
14059e63f: mov rcx, qword ptr [rbx + 0xc8]
14059e646: mov rcx, qword ptr [rcx + 0x1e0]
14059e64d: not rax
14059e650: lea rax, [rax + rax*2]
14059e654: shl rax, 5
14059e658: cmp qword ptr [rcx + rax + 0x20], 0
14059e65e: jne 0x14059e6ab
14059e660: mov byte ptr [rbx + 0xaa], 1
14059e667: mov byte ptr [rbx + 0xab], 1
14059e66e: mov byte ptr [rbx + 0xac], 1
14059e675: mov byte ptr [rbx + 0xad], 1
14059e67c: mov byte ptr [rbx + 0xae], 1
14059e683: mov byte ptr [rbx + 0xaf], 1
14059e68a: mov byte ptr [rbx + 0xb0], 1
14059e691: mov byte ptr [rbx + 0xb1], 1
14059e698: mov byte ptr [rbx + 0xb2], 1
14059e69f: mov rcx, qword ptr [rbx + 0xc8]
14059e6a6: call 0x140572eb0 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::resources::events::ClearCanvasCmd>
14059e6ab: mov byte ptr [rbx + 0xaa], 1
14059e6b2: mov byte ptr [rbx + 0xab], 1
14059e6b9: mov byte ptr [rbx + 0xac], 1
14059e6c0: mov byte ptr [rbx + 0xad], 1
14059e6c7: mov byte ptr [rbx + 0xae], 1
14059e6ce: mov byte ptr [rbx + 0xaf], 1
14059e6d5: mov byte ptr [rbx + 0xb0], 1
14059e6dc: mov byte ptr [rbx + 0xb1], 1
14059e6e3: mov byte ptr [rbx + 0xb2], 1
14059e6ea: movabs rdx, 0x50c7f48d615ae830
14059e6f4: movabs r8, 0x360552332bdc4363
14059e6fe: mov rcx, qword ptr [rbx + 0xe8]
14059e705: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
14059e70a: test rax, rax
14059e70d: je 0x14059e757
14059e70f: mov rax, qword ptr [rbx + 0xc8]
14059e716: cmp qword ptr [rax + 0x218], rdx
14059e71d: jbe 0x14059e757
14059e71f: mov rax, qword ptr [rbx + 0xc8]
14059e726: mov rax, qword ptr [rax + 0x210]
14059e72d: mov rax, qword ptr [rax + rdx*8]
14059e731: test rax, rax
14059e734: je 0x14059e757
14059e736: mov rcx, qword ptr [rbx + 0xc8]
14059e73d: mov rcx, qword ptr [rcx + 0x1e0]
14059e744: not rax
14059e747: lea rax, [rax + rax*2]
14059e74b: shl rax, 5
14059e74f: cmp qword ptr [rcx + rax + 0x20], 0
14059e755: jne 0x14059e7a2
14059e757: mov byte ptr [rbx + 0xaa], 1
14059e75e: mov byte ptr [rbx + 0xab], 1
14059e765: mov byte ptr [rbx + 0xac], 1
14059e76c: mov byte ptr [rbx + 0xad], 1
14059e773: mov byte ptr [rbx + 0xae], 1
14059e77a: mov byte ptr [rbx + 0xaf], 1
14059e781: mov byte ptr [rbx + 0xb0], 1
14059e788: mov byte ptr [rbx + 0xb1], 1
14059e78f: mov byte ptr [rbx + 0xb2], 1
14059e796: mov rcx, qword ptr [rbx + 0xc8]
14059e79d: call 0x140583290 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>
14059e7a2: mov byte ptr [rbx + 0xaa], 1
14059e7a9: mov byte ptr [rbx + 0xab], 1
14059e7b0: mov byte ptr [rbx + 0xac], 1
14059e7b7: mov byte ptr [rbx + 0xad], 1
14059e7be: mov byte ptr [rbx + 0xae], 1
14059e7c5: mov byte ptr [rbx + 0xaf], 1
14059e7cc: mov byte ptr [rbx + 0xb0], 1
14059e7d3: mov byte ptr [rbx + 0xb1], 1
14059e7da: mov byte ptr [rbx + 0xb2], 1
14059e7e1: movabs rdx, 0x76eef710ca5ee797
14059e7eb: movabs r8, 0x804a5169b0f39613
14059e7f5: mov rcx, qword ptr [rbx + 0xe8]
14059e7fc: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
14059e801: test rax, rax
14059e804: je 0x14059e84e
14059e806: mov rax, qword ptr [rbx + 0xc8]
14059e80d: cmp qword ptr [rax + 0x218], rdx
14059e814: jbe 0x14059e84e
14059e816: mov rax, qword ptr [rbx + 0xc8]
14059e81d: mov rax, qword ptr [rax + 0x210]
14059e824: mov rax, qword ptr [rax + rdx*8]
14059e828: test rax, rax
14059e82b: je 0x14059e84e
14059e82d: mov rcx, qword ptr [rbx + 0xc8]
14059e834: mov rcx, qword ptr [rcx + 0x1e0]
14059e83b: not rax
14059e83e: lea rax, [rax + rax*2]
14059e842: shl rax, 5
14059e846: cmp qword ptr [rcx + rax + 0x20], 0
14059e84c: jne 0x14059e899
14059e84e: mov byte ptr [rbx + 0xaa], 1
14059e855: mov byte ptr [rbx + 0xab], 1
14059e85c: mov byte ptr [rbx + 0xac], 1
14059e863: mov byte ptr [rbx + 0xad], 1
14059e86a: mov byte ptr [rbx + 0xae], 1
14059e871: mov byte ptr [rbx + 0xaf], 1
14059e878: mov byte ptr [rbx + 0xb0], 1
14059e87f: mov byte ptr [rbx + 0xb1], 1
14059e886: mov byte ptr [rbx + 0xb2], 1
14059e88d: mov rcx, qword ptr [rbx + 0xc8]
14059e894: call 0x140578b80 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::systems::save::load_game::StartupRevealStarted>
14059e899: mov byte ptr [rbx + 0xaa], 1
14059e8a0: mov byte ptr [rbx + 0xab], 1
14059e8a7: mov byte ptr [rbx + 0xac], 1
14059e8ae: mov byte ptr [rbx + 0xad], 1
14059e8b5: mov byte ptr [rbx + 0xae], 1
14059e8bc: mov byte ptr [rbx + 0xaf], 1
14059e8c3: mov byte ptr [rbx + 0xb0], 1
14059e8ca: mov byte ptr [rbx + 0xb1], 1
14059e8d1: mov byte ptr [rbx + 0xb2], 1
14059e8d8: movabs rdx, 0xd8f93d064ea9d61c
14059e8e2: movabs r8, 0x6447617066d9832f
14059e8ec: mov rcx, qword ptr [rbx + 0xe8]
14059e8f3: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
14059e8f8: test rax, rax
14059e8fb: je 0x14059e945
14059e8fd: mov rax, qword ptr [rbx + 0xc8]
14059e904: cmp qword ptr [rax + 0x218], rdx
14059e90b: jbe 0x14059e945
14059e90d: mov rax, qword ptr [rbx + 0xc8]
14059e914: mov rax, qword ptr [rax + 0x210]
14059e91b: mov rax, qword ptr [rax + rdx*8]
14059e91f: test rax, rax
14059e922: je 0x14059e945
14059e924: mov rcx, qword ptr [rbx + 0xc8]
14059e92b: mov rcx, qword ptr [rcx + 0x1e0]
14059e932: not rax
14059e935: lea rax, [rax + rax*2]
14059e939: shl rax, 5
14059e93d: cmp qword ptr [rcx + rax + 0x20], 0
14059e943: jne 0x14059e990
14059e945: mov byte ptr [rbx + 0xaa], 1
14059e94c: mov byte ptr [rbx + 0xab], 1
14059e953: mov byte ptr [rbx + 0xac], 1
14059e95a: mov byte ptr [rbx + 0xad], 1
14059e961: mov byte ptr [rbx + 0xae], 1
14059e968: mov byte ptr [rbx + 0xaf], 1
14059e96f: mov byte ptr [rbx + 0xb0], 1
14059e976: mov byte ptr [rbx + 0xb1], 1
14059e97d: mov byte ptr [rbx + 0xb2], 1
14059e984: mov rcx, qword ptr [rbx + 0xc8]
14059e98b: call 0x1405734e0 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::systems::save::load_game::StartupRevealFinished>
14059e990: mov byte ptr [rbx + 0xaa], 1
14059e997: mov byte ptr [rbx + 0xab], 1
14059e99e: mov byte ptr [rbx + 0xac], 1
14059e9a5: mov byte ptr [rbx + 0xad], 1
14059e9ac: mov byte ptr [rbx + 0xae], 1
14059e9b3: mov byte ptr [rbx + 0xaf], 1
14059e9ba: mov byte ptr [rbx + 0xb0], 1
14059e9c1: mov byte ptr [rbx + 0xb1], 1
14059e9c8: mov byte ptr [rbx + 0xb2], 1
14059e9cf: movabs rdx, 0x53692c24f21fcb2f
14059e9d9: movabs r8, 0xbc74dd13ef48076b
14059e9e3: mov rcx, qword ptr [rbx + 0xe8]
14059e9ea: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
14059e9ef: test rax, rax
14059e9f2: je 0x14059ea3c
14059e9f4: mov rax, qword ptr [rbx + 0xc8]
14059e9fb: cmp qword ptr [rax + 0x218], rdx
14059ea02: jbe 0x14059ea3c
14059ea04: mov rax, qword ptr [rbx + 0xc8]
14059ea0b: mov rax, qword ptr [rax + 0x210]
14059ea12: mov rax, qword ptr [rax + rdx*8]
14059ea16: test rax, rax
14059ea19: je 0x14059ea3c
14059ea1b: mov rcx, qword ptr [rbx + 0xc8]
14059ea22: mov rcx, qword ptr [rcx + 0x1e0]
14059ea29: not rax
14059ea2c: lea rax, [rax + rax*2]
14059ea30: shl rax, 5
14059ea34: cmp qword ptr [rcx + rax + 0x20], 0
14059ea3a: jne 0x14059ea87
14059ea3c: mov byte ptr [rbx + 0xaa], 1
14059ea43: mov byte ptr [rbx + 0xab], 1
14059ea4a: mov byte ptr [rbx + 0xac], 1
14059ea51: mov byte ptr [rbx + 0xad], 1
14059ea58: mov byte ptr [rbx + 0xae], 1
14059ea5f: mov byte ptr [rbx + 0xaf], 1
14059ea66: mov byte ptr [rbx + 0xb0], 1
14059ea6d: mov byte ptr [rbx + 0xb1], 1
14059ea74: mov byte ptr [rbx + 0xb2], 1
14059ea7b: mov rcx, qword ptr [rbx + 0xc8]
14059ea82: call 0x140570fc0 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>
14059ea87: mov rax, qword ptr [rbp + 0x37b0]
14059ea8e: lea r14, [rax + 0xe0]
14059ea95: mov byte ptr [rbx + 0xaa], 1
14059ea9c: mov byte ptr [rbx + 0xab], 1
14059eaa3: mov byte ptr [rbx + 0xac], 1
14059eaaa: mov byte ptr [rbx + 0xad], 1
14059eab1: mov byte ptr [rbx + 0xae], 1
14059eab8: mov byte ptr [rbx + 0xaf], 1
14059eabf: mov byte ptr [rbx + 0xb0], 1
14059eac6: mov byte ptr [rbx + 0xb1], 1
14059eacd: mov byte ptr [rbx + 0xb2], 1
14059ead4: mov rcx, r14
14059ead7: call 0x14090df40 ; country_core::game_config::DevSettings::live_link
14059eadc: mov byte ptr [rbx + 0xaa], 1
14059eae3: mov byte ptr [rbx + 0xab], 1
14059eaea: mov byte ptr [rbx + 0xac], 1
14059eaf1: mov byte ptr [rbx + 0xad], 1
14059eaf8: mov byte ptr [rbx + 0xae], 1
14059eaff: mov byte ptr [rbx + 0xaf], 1
14059eb06: mov byte ptr [rbx + 0xb0], 1
14059eb0d: mov byte ptr [rbx + 0xb1], 1
14059eb14: mov byte ptr [rbx + 0xb2], 1
14059eb1b: lea rcx, [rbx + 0xb00]
14059eb22: mov edx, eax
14059eb24: call 0x1408ba0f0 ; country_core::asset_libraries::configwatch::ConfigWatch::new
14059eb29: mov byte ptr [rbx + 0xaa], 1
14059eb30: mov byte ptr [rbx + 0xab], 1
14059eb37: mov byte ptr [rbx + 0xac], 1
14059eb3e: mov byte ptr [rbx + 0xad], 1
14059eb45: mov byte ptr [rbx + 0xae], 1
14059eb4c: mov byte ptr [rbx + 0xaf], 1
14059eb53: mov byte ptr [rbx + 0xb0], 1
14059eb5a: mov byte ptr [rbx + 0xb1], 1
14059eb61: mov byte ptr [rbx + 0xb2], 1
14059eb68: lea rdx, [rbx + 0xb00]
14059eb6f: mov rcx, qword ptr [rbx + 0xc8]
14059eb76: call 0x140131d60 ; bevy_app::app::App::insert_resource<country_core::asset_libraries::configwatch::ConfigWatch>
14059eb7b: mov byte ptr [rbx + 0xaa], 1
14059eb82: mov byte ptr [rbx + 0xab], 1
14059eb89: mov byte ptr [rbx + 0xac], 1
14059eb90: mov byte ptr [rbx + 0xad], 1
14059eb97: mov byte ptr [rbx + 0xae], 1
14059eb9e: mov byte ptr [rbx + 0xaf], 1
14059eba5: mov byte ptr [rbx + 0xb0], 1
14059ebac: mov byte ptr [rbx + 0xb1], 1
14059ebb3: mov byte ptr [rbx + 0xb2], 1
14059ebba: mov rcx, rax
14059ebbd: call 0x1409457c0 ; country_core::systems::terrain::impl$0::insert_terrain_resources
14059ebc2: mov byte ptr [rbx + 0xaa], 1
14059ebc9: mov byte ptr [rbx + 0xab], 1
14059ebd0: mov byte ptr [rbx + 0xac], 1
14059ebd7: mov byte ptr [rbx + 0xad], 1
14059ebde: mov byte ptr [rbx + 0xae], 1
14059ebe5: mov byte ptr [rbx + 0xaf], 1
14059ebec: mov byte ptr [rbx + 0xb0], 1
14059ebf3: mov byte ptr [rbx + 0xb1], 1
14059ebfa: mov byte ptr [rbx + 0xb2], 1
14059ec01: mov rcx, rax
14059ec04: call 0x140943e10 ; country_core::systems::history::impl$0::insert_history_resources
14059ec09: mov rdx, qword ptr [rbx + 0x138]
14059ec10: add rdx, 0x10
14059ec14: mov byte ptr [rbx + 0xaa], 1
14059ec1b: mov byte ptr [rbx + 0xab], 1
14059ec22: mov byte ptr [rbx + 0xac], 1
14059ec29: mov byte ptr [rbx + 0xad], 1
14059ec30: mov byte ptr [rbx + 0xae], 1
14059ec37: mov byte ptr [rbx + 0xaf], 1
14059ec3e: mov byte ptr [rbx + 0xb0], 1
14059ec45: mov byte ptr [rbx + 0xb1], 1
14059ec4c: mov byte ptr [rbx + 0xb2], 1
14059ec53: mov rcx, rax
14059ec56: mov r8, qword ptr [rbp + 0x37b0]
14059ec5d: call 0x140940190 ; country_core::resources::impl$0::insert_rendering_resources
14059ec62: mov rsi, rax
14059ec65: mov rdx, qword ptr [rbx + 0x138]
14059ec6c: add rdx, 0x10
14059ec70: mov byte ptr [rbx + 0xaa], 1
14059ec77: mov byte ptr [rbx + 0xab], 1
14059ec7e: mov byte ptr [rbx + 0xac], 1
14059ec85: mov byte ptr [rbx + 0xad], 1
14059ec8c: mov byte ptr [rbx + 0xae], 1
14059ec93: mov byte ptr [rbx + 0xaf], 1
14059ec9a: mov byte ptr [rbx + 0xb0], 1
14059eca1: mov byte ptr [rbx + 0xb1], 1
14059eca8: mov byte ptr [rbx + 0xb2], 1
14059ecaf: lea rcx, [rbx + 0xb00]
14059ecb6: call 0x140c02570 ; rhapsody::raw::dynamic_constants::DynamicConstants::new
14059ecbb: movups xmm0, xmmword ptr [rbx + 0xb40]
14059ecc2: movups xmmword ptr [rbx + 0x487], xmm0
14059ecc9: movups xmm0, xmmword ptr [rbx + 0xb00]
14059ecd0: movups xmm1, xmmword ptr [rbx + 0xb10]
14059ecd7: movups xmm2, xmmword ptr [rbx + 0xb20]
14059ecde: movups xmm3, xmmword ptr [rbx + 0xb30]
14059ece5: movups xmmword ptr [rbx + 0x477], xmm3
14059ecec: movups xmmword ptr [rbx + 0x467], xmm2
14059ecf3: movups xmmword ptr [rbx + 0x457], xmm1
14059ecfa: movups xmmword ptr [rbx + 0x447], xmm0
14059ed01: mov byte ptr [rbx + 0xb00], 0
14059ed08: movups xmm0, xmmword ptr [rbx + 0x440]
14059ed0f: movups xmmword ptr [rbx + 0xb01], xmm0
14059ed16: movups xmm0, xmmword ptr [rbx + 0x450]
14059ed1d: movups xmmword ptr [rbx + 0xb11], xmm0
14059ed24: movups xmm0, xmmword ptr [rbx + 0x460]
14059ed2b: movups xmmword ptr [rbx + 0xb21], xmm0
14059ed32: movups xmm0, xmmword ptr [rbx + 0x470]
14059ed39: movups xmmword ptr [rbx + 0xb31], xmm0
14059ed40: movups xmm0, xmmword ptr [rbx + 0x480]
14059ed47: movups xmmword ptr [rbx + 0xb41], xmm0
14059ed4e: mov rax, qword ptr [rbx + 0x48f]
14059ed55: mov qword ptr [rbx + 0xb50], rax
14059ed5c: mov byte ptr [rbx + 0xaa], 1
14059ed63: mov byte ptr [rbx + 0xab], 1
14059ed6a: mov byte ptr [rbx + 0xac], 1
14059ed71: mov byte ptr [rbx + 0xad], 1
14059ed78: mov byte ptr [rbx + 0xae], 1
14059ed7f: mov byte ptr [rbx + 0xaf], 1
14059ed86: mov byte ptr [rbx + 0xb0], 1
14059ed8d: mov byte ptr [rbx + 0xb1], 1
14059ed94: mov byte ptr [rbx + 0xb2], 1
14059ed9b: lea rdx, [rbx + 0xb00]
14059eda2: mov rcx, rsi
14059eda5: call 0x14011f840 ; bevy_app::app::App::insert_resource<country_core::render::DynamicConstantsResource>
14059edaa: mov rdx, qword ptr [rbx + 0x138]
14059edb1: lock inc qword ptr [rdx]
14059edb5: jle 0x1405a7040
14059edbb: mov byte ptr [rbx + 0xaa], 1
14059edc2: mov byte ptr [rbx + 0xab], 1
14059edc9: mov byte ptr [rbx + 0xac], 1
14059edd0: mov byte ptr [rbx + 0xad], 1
14059edd7: mov byte ptr [rbx + 0xae], 1
14059edde: mov byte ptr [rbx + 0xaf], 1
14059ede5: mov byte ptr [rbx + 0xb0], 1
14059edec: mov byte ptr [rbx + 0xb1], 1
14059edf3: mov byte ptr [rbx + 0xb2], 1
14059edfa: mov rcx, rax
14059edfd: call 0x14011cb90 ; bevy_app::app::App::insert_resource<country_core::render::GraphicsDeviceResource>
14059ee02: mov rsi, rax
14059ee05: mov byte ptr [rbx + 0xaa], 1
14059ee0c: mov byte ptr [rbx + 0xab], 1
14059ee13: mov byte ptr [rbx + 0xac], 1
14059ee1a: mov byte ptr [rbx + 0xad], 1
14059ee21: mov byte ptr [rbx + 0xae], 1
14059ee28: mov byte ptr [rbx + 0xaf], 1
14059ee2f: mov byte ptr [rbx + 0xb0], 0
14059ee36: mov byte ptr [rbx + 0xb1], 1
14059ee3d: mov byte ptr [rbx + 0xb2], 1
14059ee44: mov rcx, qword ptr [rbx + 0x2b8]
14059ee4b: mov rdx, qword ptr [rbx + 0x2b0]
14059ee52: call 0x140987cb0 ; country_core::render::AvailableGraphicsDevices::from_preference_order
14059ee57: mov r8, rdx
14059ee5a: mov byte ptr [rbx + 0xaa], 1
14059ee61: mov byte ptr [rbx + 0xab], 1
14059ee68: mov byte ptr [rbx + 0xac], 1
14059ee6f: mov byte ptr [rbx + 0xad], 1
14059ee76: mov byte ptr [rbx + 0xae], 1
14059ee7d: mov byte ptr [rbx + 0xaf], 1
14059ee84: mov byte ptr [rbx + 0xb0], 0
14059ee8b: mov byte ptr [rbx + 0xb1], 1
14059ee92: mov byte ptr [rbx + 0xb2], 1
14059ee99: mov rcx, rsi
14059ee9c: mov rdx, rax
14059ee9f: call 0x140123e00 ; bevy_app::app::App::insert_resource<country_core::render::AvailableGraphicsDevices>
14059eea4: mov rsi, rax
14059eea7: movaps xmm8, xmmword ptr [rip + 0x245ff51]
14059eeaf: movaps xmmword ptr [rbx + 0x470], xmm8
14059eeb7: movaps xmm9, xmmword ptr [rip + 0x245ff31]
14059eebf: movaps xmmword ptr [rbx + 0x460], xmm9
14059eec7: movaps xmm10, xmmword ptr [rip + 0x245ff11]
14059eecf: movaps xmmword ptr [rbx + 0x450], xmm10
14059eed7: movaps xmm11, xmmword ptr [rip + 0x245fef1]
14059eedf: movaps xmmword ptr [rbx + 0x440], xmm11
14059eee7: movaps xmmword ptr [rbx + 0x480], xmm11
14059eeef: movaps xmmword ptr [rbx + 0x490], xmm10
14059eef7: movaps xmmword ptr [rbx + 0x4a0], xmm9
14059eeff: movaps xmmword ptr [rbx + 0x4b0], xmm8
14059ef07: movaps xmmword ptr [rbx + 0x4c0], xmm11
14059ef0f: movaps xmmword ptr [rbx + 0x4d0], xmm10
14059ef17: movaps xmmword ptr [rbx + 0x4e0], xmm9
14059ef1f: movaps xmmword ptr [rbx + 0x4f0], xmm8
14059ef27: movaps xmmword ptr [rbx + 0x530], xmm8
14059ef2f: movaps xmmword ptr [rbx + 0x520], xmm9
14059ef37: movaps xmmword ptr [rbx + 0x510], xmm10
14059ef3f: movaps xmmword ptr [rbx + 0x500], xmm11
14059ef47: mov dword ptr [rbx + 0xc08], 0
14059ef51: lea rdi, [rbx + 0xb00]
14059ef58: lea rdx, [rbx + 0x440]
14059ef5f: mov r8d, 0x100
14059ef65: mov rcx, rdi
14059ef68: call 0x142923c00 ; 
14059ef6d: mov qword ptr [rbx + 0xc00], 0
14059ef78: mov byte ptr [rbx + 0xaa], 1
14059ef7f: mov byte ptr [rbx + 0xab], 1
14059ef86: mov byte ptr [rbx + 0xac], 1
14059ef8d: mov byte ptr [rbx + 0xad], 1
14059ef94: mov byte ptr [rbx + 0xae], 1
14059ef9b: mov byte ptr [rbx + 0xaf], 1
14059efa2: mov byte ptr [rbx + 0xb0], 0
14059efa9: mov byte ptr [rbx + 0xb1], 1
14059efb0: mov byte ptr [rbx + 0xb2], 1
14059efb7: mov rcx, rsi
14059efba: mov rdx, rdi
14059efbd: call 0x140135cc0 ; bevy_app::app::App::insert_resource<country_core::render::RenderTemporalState>
14059efc2: mov byte ptr [rbx + 0xaa], 1
14059efc9: mov byte ptr [rbx + 0xab], 1
14059efd0: mov byte ptr [rbx + 0xac], 1
14059efd7: mov byte ptr [rbx + 0xad], 1
14059efde: mov byte ptr [rbx + 0xae], 1
14059efe5: mov byte ptr [rbx + 0xaf], 1
14059efec: mov byte ptr [rbx + 0xb0], 0
14059eff3: mov byte ptr [rbx + 0xb1], 1
14059effa: mov byte ptr [rbx + 0xb2], 1
14059f001: mov rcx, rax
14059f004: call 0x1409440c0 ; country_core::systems::ivy::impl$0::insert_ivy_resources
14059f009: mov rsi, rax
14059f00c: mov rax, qword ptr [rbp + 0x37b8]
14059f013: mov rcx, qword ptr [rax]
14059f016: lock inc qword ptr [rcx]
14059f01a: jle 0x1405a7040
14059f020: mov byte ptr [rbx + 0xaa], 1
14059f027: mov byte ptr [rbx + 0xab], 1
14059f02e: mov byte ptr [rbx + 0xac], 1
14059f035: mov byte ptr [rbx + 0xad], 1
14059f03c: mov byte ptr [rbx + 0xae], 1
14059f043: mov byte ptr [rbx + 0xaf], 1
14059f04a: mov byte ptr [rbx + 0xb0], 0
14059f051: mov byte ptr [rbx + 0xb1], 1
14059f058: mov byte ptr [rbx + 0xb2], 1
14059f05f: call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
14059f064: mov byte ptr [rbx + 0xaa], 1
14059f06b: mov byte ptr [rbx + 0xab], 1
14059f072: mov byte ptr [rbx + 0xac], 1
14059f079: mov byte ptr [rbx + 0xad], 1
14059f080: mov byte ptr [rbx + 0xae], 1
14059f087: mov byte ptr [rbx + 0xaf], 1
14059f08e: mov byte ptr [rbx + 0xb0], 0
14059f095: mov byte ptr [rbx + 0xb1], 1
14059f09c: mov byte ptr [rbx + 0xb2], 1
14059f0a3: mov rcx, rsi
14059f0a6: mov rdx, rax
14059f0a9: call 0x140127e40 ; bevy_app::app::App::insert_resource<country_core::resources::gui::PendingEguiContextResource>
14059f0ae: mov rsi, rax
14059f0b1: mov byte ptr [rbx + 0xaa], 1
14059f0b8: mov byte ptr [rbx + 0xab], 1
14059f0bf: mov byte ptr [rbx + 0xac], 1
14059f0c6: mov byte ptr [rbx + 0xad], 1
14059f0cd: mov byte ptr [rbx + 0xae], 1
14059f0d4: mov byte ptr [rbx + 0xaf], 1
14059f0db: mov byte ptr [rbx + 0xb0], 0
14059f0e2: mov byte ptr [rbx + 0xb1], 1
14059f0e9: mov byte ptr [rbx + 0xb2], 1
14059f0f0: lea rcx, [rbx + 0xb00]
14059f0f7: call 0x14093fce0 ; country_core::resources::gui::impl$9::default
14059f0fc: mov byte ptr [rbx + 0xaa], 1
14059f103: mov byte ptr [rbx + 0xab], 1
14059f10a: mov byte ptr [rbx + 0xac], 1
14059f111: mov byte ptr [rbx + 0xad], 1
14059f118: mov byte ptr [rbx + 0xae], 1
14059f11f: mov byte ptr [rbx + 0xaf], 1
14059f126: mov byte ptr [rbx + 0xb0], 0
14059f12d: mov byte ptr [rbx + 0xb1], 1
14059f134: mov byte ptr [rbx + 0xb2], 1
14059f13b: lea rdx, [rbx + 0xb00]
14059f142: mov rcx, rsi
14059f145: call 0x140128a10 ; bevy_app::app::App::insert_resource<country_core::resources::gui::DebugGui>
14059f14a: mov rsi, rax
14059f14d: mov byte ptr [rbx + 0xaa], 1
14059f154: mov byte ptr [rbx + 0xab], 1
14059f15b: mov byte ptr [rbx + 0xac], 1
14059f162: mov byte ptr [rbx + 0xad], 1
14059f169: mov byte ptr [rbx + 0xae], 1
14059f170: mov byte ptr [rbx + 0xaf], 1
14059f177: mov byte ptr [rbx + 0xb0], 0
14059f17e: mov byte ptr [rbx + 0xb1], 1
14059f185: mov byte ptr [rbx + 0xb2], 1
14059f18c: lea rcx, [rbx + 0xb00]
14059f193: mov rdx, qword ptr [rbp + 0x37c0]
14059f19a: call 0x140353390 ; tiny_glade::app_config::CliArgs::to_input_recording_opts
14059f19f: mov byte ptr [rbx + 0xaa], 1
14059f1a6: mov byte ptr [rbx + 0xab], 1
14059f1ad: mov byte ptr [rbx + 0xac], 1
14059f1b4: mov byte ptr [rbx + 0xad], 1
14059f1bb: mov byte ptr [rbx + 0xae], 1
14059f1c2: mov byte ptr [rbx + 0xaf], 1
14059f1c9: mov byte ptr [rbx + 0xb0], 0
14059f1d0: mov byte ptr [rbx + 0xb1], 1
14059f1d7: mov byte ptr [rbx + 0xb2], 1
14059f1de: lea rdx, [rbx + 0xb00]
14059f1e5: lea r8, [rbx + 0x3220]
14059f1ec: mov rcx, rsi
14059f1ef: call 0x140942f80 ; country_core::systems::backend::impl$0::insert_backend_resources
14059f1f4: mov rsi, rax
14059f1f7: lea rdi, [rbx + 0xb00]
14059f1fe: lea rdx, [rbx + 0x3220]
14059f205: mov r8d, 0x4e0
14059f20b: mov rcx, rdi
14059f20e: call 0x142923c00 ; 
14059f213: mov byte ptr [rbx + 0xaa], 1
14059f21a: mov byte ptr [rbx + 0xab], 1
14059f221: mov byte ptr [rbx + 0xac], 1
14059f228: mov byte ptr [rbx + 0xad], 1
14059f22f: mov byte ptr [rbx + 0xae], 0
14059f236: mov byte ptr [rbx + 0xaf], 1
14059f23d: mov byte ptr [rbx + 0xb0], 0
14059f244: mov byte ptr [rbx + 0xb1], 1
14059f24b: mov byte ptr [rbx + 0xb2], 1
14059f252: mov rcx, rsi
14059f255: mov rdx, rdi
14059f258: call 0x140133510 ; bevy_app::app::App::insert_resource<country_core::systems::camera_rig::MainCamera>
14059f25d: mov rsi, rax
14059f260: mov byte ptr [rbx + 0xaa], 1
14059f267: mov byte ptr [rbx + 0xab], 1
14059f26e: mov byte ptr [rbx + 0xac], 1
14059f275: mov byte ptr [rbx + 0xad], 1
14059f27c: mov byte ptr [rbx + 0xae], 0
14059f283: mov byte ptr [rbx + 0xaf], 1
14059f28a: mov byte ptr [rbx + 0xb0], 0
14059f291: mov byte ptr [rbx + 0xb1], 1
14059f298: mov byte ptr [rbx + 0xb2], 1
14059f29f: lea rcx, [rbx + 0xb00]
14059f2a6: call 0x1408ae7f0 ; country_core::resources::image_formation::impl$0::default
14059f2ab: mov byte ptr [rbx + 0xaa], 1
14059f2b2: mov byte ptr [rbx + 0xab], 1
14059f2b9: mov byte ptr [rbx + 0xac], 1
14059f2c0: mov byte ptr [rbx + 0xad], 1
14059f2c7: mov byte ptr [rbx + 0xae], 0
14059f2ce: mov byte ptr [rbx + 0xaf], 1
14059f2d5: mov byte ptr [rbx + 0xb0], 0
14059f2dc: mov byte ptr [rbx + 0xb1], 1
14059f2e3: mov byte ptr [rbx + 0xb2], 1
14059f2ea: lea rdx, [rbx + 0xb00]
14059f2f1: mov rcx, rsi
14059f2f4: call 0x140122c30 ; bevy_app::app::App::insert_resource<country_core::resources::image_formation::ImageFormationConfigs>
14059f2f9: mov rsi, rax
14059f2fc: mov byte ptr [rbx + 0xaa], 1
14059f303: mov byte ptr [rbx + 0xab], 1
14059f30a: mov byte ptr [rbx + 0xac], 1
14059f311: mov byte ptr [rbx + 0xad], 1
14059f318: mov byte ptr [rbx + 0xae], 0
14059f31f: mov byte ptr [rbx + 0xaf], 1
14059f326: mov byte ptr [rbx + 0xb0], 0
14059f32d: mov byte ptr [rbx + 0xb1], 1
14059f334: mov byte ptr [rbx + 0xb2], 1
14059f33b: lea rcx, [rbx + 0xb00]
14059f342: call 0x1408705b0 ; country_core::startup::init_photomode_state::impl$1::default
14059f347: mov byte ptr [rbx + 0xaa], 1
14059f34e: mov byte ptr [rbx + 0xab], 1
14059f355: mov byte ptr [rbx + 0xac], 1
14059f35c: mov byte ptr [rbx + 0xad], 1
14059f363: mov byte ptr [rbx + 0xae], 0
14059f36a: mov byte ptr [rbx + 0xaf], 1
14059f371: mov byte ptr [rbx + 0xb0], 0
14059f378: mov byte ptr [rbx + 0xb1], 1
14059f37f: mov byte ptr [rbx + 0xb2], 1
14059f386: lea rdx, [rbx + 0xb00]
14059f38d: mov rcx, rsi
14059f390: call 0x14012fa90 ; bevy_app::app::App::insert_resource<country_core::startup::init_photomode_state::PhotomodeState>
14059f395: mov byte ptr [rbx + 0xaa], 1
14059f39c: mov byte ptr [rbx + 0xab], 1
14059f3a3: mov byte ptr [rbx + 0xac], 1
14059f3aa: mov byte ptr [rbx + 0xad], 1
14059f3b1: mov byte ptr [rbx + 0xae], 0
14059f3b8: mov byte ptr [rbx + 0xaf], 1
14059f3bf: mov byte ptr [rbx + 0xb0], 0
14059f3c6: mov byte ptr [rbx + 0xb1], 1
14059f3cd: mov byte ptr [rbx + 0xb2], 1
14059f3d4: mov rcx, rax
14059f3d7: xor edx, edx
14059f3d9: call 0x14011d170 ; bevy_app::app::App::insert_resource<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>
14059f3de: mov byte ptr [rbx + 0xaa], 1
14059f3e5: mov byte ptr [rbx + 0xab], 1
14059f3ec: mov byte ptr [rbx + 0xac], 1
14059f3f3: mov byte ptr [rbx + 0xad], 1
14059f3fa: mov byte ptr [rbx + 0xae], 0
14059f401: mov byte ptr [rbx + 0xaf], 1
14059f408: mov byte ptr [rbx + 0xb0], 0
14059f40f: mov byte ptr [rbx + 0xb1], 1
14059f416: mov byte ptr [rbx + 0xb2], 1
14059f41d: xorps xmm1, xmm1
14059f420: xorps xmm2, xmm2
14059f423: mov rcx, rax
14059f426: call 0x140138520 ; bevy_app::app::App::insert_resource<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>
14059f42b: mov rsi, rax
14059f42e: mov rdx, qword ptr [rbx + 0x138]
14059f435: add rdx, 0x10
14059f439: mov byte ptr [rbx + 0xaa], 1
14059f440: mov byte ptr [rbx + 0xab], 1
14059f447: mov byte ptr [rbx + 0xac], 1
14059f44e: mov byte ptr [rbx + 0xad], 1
14059f455: mov byte ptr [rbx + 0xae], 0
14059f45c: mov byte ptr [rbx + 0xaf], 1
14059f463: mov byte ptr [rbx + 0xb0], 0
14059f46a: mov byte ptr [rbx + 0xb1], 1
14059f471: mov byte ptr [rbx + 0xb2], 1
14059f478: lea rcx, [rbx + 0xb00]
14059f47f: call 0x1409917d0 ; country_core::render::nani::gpu_world::GpuWorld::new
14059f484: mov byte ptr [rbx + 0xaa], 1
14059f48b: mov byte ptr [rbx + 0xab], 1
14059f492: mov byte ptr [rbx + 0xac], 1
14059f499: mov byte ptr [rbx + 0xad], 1
14059f4a0: mov byte ptr [rbx + 0xae], 0
14059f4a7: mov byte ptr [rbx + 0xaf], 1
14059f4ae: mov byte ptr [rbx + 0xb0], 0
14059f4b5: mov byte ptr [rbx + 0xb1], 1
14059f4bc: mov byte ptr [rbx + 0xb2], 1
14059f4c3: lea rdx, [rbx + 0xb00]
14059f4ca: mov rcx, rsi
14059f4cd: call 0x1401159d0 ; bevy_app::app::App::insert_resource<country_core::render::nani::gpu_world::GpuWorld>
14059f4d2: mov byte ptr [rbx + 0xaa], 1
14059f4d9: mov byte ptr [rbx + 0xab], 1
14059f4e0: mov byte ptr [rbx + 0xac], 1
14059f4e7: mov byte ptr [rbx + 0xad], 1
14059f4ee: mov byte ptr [rbx + 0xae], 0
14059f4f5: mov byte ptr [rbx + 0xaf], 1
14059f4fc: mov byte ptr [rbx + 0xb0], 0
14059f503: mov byte ptr [rbx + 0xb1], 1
14059f50a: mov byte ptr [rbx + 0xb2], 1
14059f511: mov rcx, rax
14059f514: call 0x1404936b0 ; tiny_glade::app_systems::impl$0::add_wall_events
14059f519: mov byte ptr [rbx + 0xaa], 1
14059f520: mov byte ptr [rbx + 0xab], 1
14059f527: mov byte ptr [rbx + 0xac], 1
14059f52e: mov byte ptr [rbx + 0xad], 1
14059f535: mov byte ptr [rbx + 0xae], 0
14059f53c: mov byte ptr [rbx + 0xaf], 1
14059f543: mov byte ptr [rbx + 0xb0], 0
14059f54a: mov byte ptr [rbx + 0xb1], 1
14059f551: mov byte ptr [rbx + 0xb2], 1
14059f558: mov rcx, rax
14059f55b: call 0x140493120 ; tiny_glade::app_systems::impl$0::add_wall_systems
14059f560: mov byte ptr [rbx + 0xaa], 1
14059f567: mov byte ptr [rbx + 0xab], 1
14059f56e: mov byte ptr [rbx + 0xac], 1
14059f575: mov byte ptr [rbx + 0xad], 1
14059f57c: mov byte ptr [rbx + 0xae], 0
14059f583: mov byte ptr [rbx + 0xaf], 1
14059f58a: mov byte ptr [rbx + 0xb0], 0
14059f591: mov byte ptr [rbx + 0xb1], 1
14059f598: mov byte ptr [rbx + 0xb2], 1
14059f59f: mov rcx, rax
14059f5a2: call 0x140945f50 ; country_core::systems::wall::shapes::impl$0::add_wall_shape_ux
14059f5a7: mov byte ptr [rbx + 0xaa], 1
14059f5ae: mov byte ptr [rbx + 0xab], 1
14059f5b5: mov byte ptr [rbx + 0xac], 1
14059f5bc: mov byte ptr [rbx + 0xad], 1
14059f5c3: mov byte ptr [rbx + 0xae], 0
14059f5ca: mov byte ptr [rbx + 0xaf], 1
14059f5d1: mov byte ptr [rbx + 0xb0], 0
14059f5d8: mov byte ptr [rbx + 0xb1], 1
14059f5df: mov byte ptr [rbx + 0xb2], 1
14059f5e6: mov rcx, rax
14059f5e9: call 0x140945a20 ; country_core::systems::terrain::impl$0::add_terrain_events
14059f5ee: mov byte ptr [rbx + 0xaa], 1
14059f5f5: mov byte ptr [rbx + 0xab], 1
14059f5fc: mov byte ptr [rbx + 0xac], 1
14059f603: mov byte ptr [rbx + 0xad], 1
14059f60a: mov byte ptr [rbx + 0xae], 0
14059f611: mov byte ptr [rbx + 0xaf], 1
14059f618: mov byte ptr [rbx + 0xb0], 0
14059f61f: mov byte ptr [rbx + 0xb1], 1
14059f626: mov byte ptr [rbx + 0xb2], 1
14059f62d: mov rcx, rax
14059f630: call 0x1409455f0 ; country_core::systems::terrain::impl$0::add_terrain_systems
14059f635: mov byte ptr [rbx + 0xaa], 1
14059f63c: mov byte ptr [rbx + 0xab], 1
14059f643: mov byte ptr [rbx + 0xac], 1
14059f64a: mov byte ptr [rbx + 0xad], 1
14059f651: mov byte ptr [rbx + 0xae], 0
14059f658: mov byte ptr [rbx + 0xaf], 1
14059f65f: mov byte ptr [rbx + 0xb0], 0
14059f666: mov byte ptr [rbx + 0xb1], 1
14059f66d: mov byte ptr [rbx + 0xb2], 1
14059f674: mov rcx, rax
14059f677: mov rdx, qword ptr [rbp + 0x37b0]
14059f67e: call 0x140942c80 ; country_core::systems::backend::impl$0::add_backend_systems
14059f683: mov byte ptr [rbx + 0xaa], 1
14059f68a: mov byte ptr [rbx + 0xab], 1
14059f691: mov byte ptr [rbx + 0xac], 1
14059f698: mov byte ptr [rbx + 0xad], 1
14059f69f: mov byte ptr [rbx + 0xae], 0
14059f6a6: mov byte ptr [rbx + 0xaf], 1
14059f6ad: mov byte ptr [rbx + 0xb0], 0
14059f6b4: mov byte ptr [rbx + 0xb1], 1
14059f6bb: mov byte ptr [rbx + 0xb2], 1
14059f6c2: mov rcx, rax
14059f6c5: call 0x140943660 ; country_core::systems::backend::impl$0::add_backend_events
14059f6ca: mov byte ptr [rbx + 0xaa], 1
14059f6d1: mov byte ptr [rbx + 0xab], 1
14059f6d8: mov byte ptr [rbx + 0xac], 1
14059f6df: mov byte ptr [rbx + 0xad], 1
14059f6e6: mov byte ptr [rbx + 0xae], 0
14059f6ed: mov byte ptr [rbx + 0xaf], 1
14059f6f4: mov byte ptr [rbx + 0xb0], 0
14059f6fb: mov byte ptr [rbx + 0xb1], 1
14059f702: mov byte ptr [rbx + 0xb2], 1
14059f709: mov rcx, rax
14059f70c: call 0x1409438a0 ; country_core::systems::history::impl$0::add_history_systems
14059f711: mov byte ptr [rbx + 0xaa], 1
14059f718: mov byte ptr [rbx + 0xab], 1
14059f71f: mov byte ptr [rbx + 0xac], 1
14059f726: mov byte ptr [rbx + 0xad], 1
14059f72d: mov byte ptr [rbx + 0xae], 0
14059f734: mov byte ptr [rbx + 0xaf], 1
14059f73b: mov byte ptr [rbx + 0xb0], 0
14059f742: mov byte ptr [rbx + 0xb1], 1
14059f749: mov byte ptr [rbx + 0xb2], 1
14059f750: mov rcx, rax
14059f753: call 0x140943f40 ; country_core::systems::history::impl$0::add_history_events
14059f758: mov byte ptr [rbx + 0xaa], 1
14059f75f: mov byte ptr [rbx + 0xab], 1
14059f766: mov byte ptr [rbx + 0xac], 1
14059f76d: mov byte ptr [rbx + 0xad], 1
14059f774: mov byte ptr [rbx + 0xae], 0
14059f77b: mov byte ptr [rbx + 0xaf], 1
14059f782: mov byte ptr [rbx + 0xb0], 0
14059f789: mov byte ptr [rbx + 0xb1], 1
14059f790: mov byte ptr [rbx + 0xb2], 1
14059f797: mov rcx, rax
14059f79a: call 0x140944070 ; country_core::systems::ivy::impl$0::add_ivy_systems
14059f79f: mov byte ptr [rbx + 0xaa], 1
14059f7a6: mov byte ptr [rbx + 0xab], 1
14059f7ad: mov byte ptr [rbx + 0xac], 1
14059f7b4: mov byte ptr [rbx + 0xad], 1
14059f7bb: mov byte ptr [rbx + 0xae], 0
14059f7c2: mov byte ptr [rbx + 0xaf], 1
14059f7c9: mov byte ptr [rbx + 0xb0], 0
14059f7d0: mov byte ptr [rbx + 0xb1], 1
14059f7d7: mov byte ptr [rbx + 0xb2], 1
14059f7de: mov rcx, rax
14059f7e1: call 0x140944280 ; country_core::systems::ivy::impl$0::add_ivy_events
14059f7e6: mov byte ptr [rbx + 0xaa], 1
14059f7ed: mov byte ptr [rbx + 0xab], 1
14059f7f4: mov byte ptr [rbx + 0xac], 1
14059f7fb: mov byte ptr [rbx + 0xad], 1
14059f802: mov byte ptr [rbx + 0xae], 0
14059f809: mov byte ptr [rbx + 0xaf], 1
14059f810: mov byte ptr [rbx + 0xb0], 0
14059f817: mov byte ptr [rbx + 0xb1], 1
14059f81e: mov byte ptr [rbx + 0xb2], 1
14059f825: mov rcx, rax
14059f828: call 0x1409411a0 ; country_core::systems::audio::impl$1::add_audio_systems
14059f82d: mov byte ptr [rbx + 0xaa], 1
14059f834: mov byte ptr [rbx + 0xab], 1
14059f83b: mov byte ptr [rbx + 0xac], 1
14059f842: mov byte ptr [rbx + 0xad], 1
14059f849: mov byte ptr [rbx + 0xae], 0
14059f850: mov byte ptr [rbx + 0xaf], 1
14059f857: mov byte ptr [rbx + 0xb0], 0
14059f85e: mov byte ptr [rbx + 0xb1], 1
14059f865: mov byte ptr [rbx + 0xb2], 1
14059f86c: mov rcx, rax
14059f86f: call 0x140941450 ; country_core::systems::audio::impl$1::add_audio_events
14059f874: mov byte ptr [rbx + 0xaa], 1
14059f87b: mov byte ptr [rbx + 0xab], 1
14059f882: mov byte ptr [rbx + 0xac], 1
14059f889: mov byte ptr [rbx + 0xad], 1
14059f890: mov byte ptr [rbx + 0xae], 0
14059f897: mov byte ptr [rbx + 0xaf], 1
14059f89e: mov byte ptr [rbx + 0xb0], 0
14059f8a5: mov byte ptr [rbx + 0xb1], 1
14059f8ac: mov byte ptr [rbx + 0xb2], 1
14059f8b3: mov rcx, rax
14059f8b6: call 0x1409443b0 ; country_core::systems::save::impl$1::add_save_load_systems
14059f8bb: mov byte ptr [rbx + 0xaa], 1
14059f8c2: mov byte ptr [rbx + 0xab], 1
14059f8c9: mov byte ptr [rbx + 0xac], 1
14059f8d0: mov byte ptr [rbx + 0xad], 1
14059f8d7: mov byte ptr [rbx + 0xae], 0
14059f8de: mov byte ptr [rbx + 0xaf], 1
14059f8e5: mov byte ptr [rbx + 0xb0], 0
14059f8ec: mov byte ptr [rbx + 0xb1], 1
14059f8f3: mov byte ptr [rbx + 0xb2], 1
14059f8fa: mov rcx, rax
14059f8fd: call 0x1409447e0 ; country_core::systems::save::impl$1::add_save_load_events_and_resources
14059f902: mov byte ptr [rbx + 0xaa], 1
14059f909: mov byte ptr [rbx + 0xab], 1
14059f910: mov byte ptr [rbx + 0xac], 1
14059f917: mov byte ptr [rbx + 0xad], 1
14059f91e: mov byte ptr [rbx + 0xae], 0
14059f925: mov byte ptr [rbx + 0xaf], 1
14059f92c: mov byte ptr [rbx + 0xb0], 0
14059f933: mov byte ptr [rbx + 0xb1], 1
14059f93a: mov byte ptr [rbx + 0xb2], 1
14059f941: mov rcx, rax
14059f944: call 0x14062fcb0 ; system_roof::impl$0::add_roof_systems
14059f949: mov byte ptr [rbx + 0xaa], 1
14059f950: mov byte ptr [rbx + 0xab], 1
14059f957: mov byte ptr [rbx + 0xac], 1
14059f95e: mov byte ptr [rbx + 0xad], 1
14059f965: mov byte ptr [rbx + 0xae], 0
14059f96c: mov byte ptr [rbx + 0xaf], 1
14059f973: mov byte ptr [rbx + 0xb0], 0
14059f97a: mov byte ptr [rbx + 0xb1], 1
14059f981: mov byte ptr [rbx + 0xb2], 1
14059f988: mov rcx, rax
14059f98b: call 0x140945c60 ; country_core::systems::ui::impl$0::add_ui
14059f990: mov rsi, rax
14059f993: mov byte ptr [rbx + 0xaa], 1
14059f99a: mov byte ptr [rbx + 0xab], 1
14059f9a1: mov byte ptr [rbx + 0xac], 1
14059f9a8: mov byte ptr [rbx + 0xad], 1
14059f9af: mov byte ptr [rbx + 0xae], 0
14059f9b6: mov byte ptr [rbx + 0xaf], 1
14059f9bd: mov byte ptr [rbx + 0xb0], 0
14059f9c4: mov byte ptr [rbx + 0xb1], 1
14059f9cb: mov byte ptr [rbx + 0xb2], 1
14059f9d2: lea rcx, [rbx + 0x2590]
14059f9d9: call 0x1403c9610 ; bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred<tuple$<void (*)(bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::writer::EventWriter<country_core::resources::window::AudioCursorMoved>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::FollowMouse>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),tuple$<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::mouse_capture::CursorCapturedPosition>,bevy_ecs::event::reader::EventReader<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorCaptureCmd> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorVisibilityCmd> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>)> >,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::writer::EventWriter<country_core::resources::window::AudioCursorMoved>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::FollowMouse>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> >,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::mouse_capture::CursorCapturedPosition>,bevy_ecs::event::reader::EventReader<country_core::resources::mouse_capture::CursorCaptureCmd>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorCaptureCmd> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorVisibilityCmd> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>)> > > > >
14059f9de: lea rcx, [rbx + 0x440]
14059f9e5: call 0x1400b18f0 ; bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred<tuple$<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,tuple$<bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast> >,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_edit_focus::ToolSelectionCancelled>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::structure_outline::OutlineWallCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_autoclutter::EvRightClickedAutoClutter>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast> >,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>,bevy_ecs::system::system_param::Local<country_core::systems::ui_edit_focus::TerrainFocusModeOrigin> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::eraser::ui_bulldozer::ChangeDecoratorTypeCmd>),void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::UiHistoryPlaySelectAnimation>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::reader::EventReader<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::event::reader::EventReader<country_core::systems::mode_manager::RequestHistoryCommand>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>),tuple$<void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::PrevAppMode>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::ModeJustChanged>)> >,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,tuple$<bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast> >,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_edit_focus::ToolSelectionCancelled>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::structure_outline::OutlineWallCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_autoclutter::EvRightClickedAutoClutter>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast> >,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>,bevy_ecs::system::system_param::Local<country_core::systems::ui_edit_focus::TerrainFocusModeOrigin> >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestMoveClutter>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::ui_focus_decoclutter::RequestRotateDecorator>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::eraser::ui_bulldozer::ChangeDecoratorTypeCmd>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::UiHistoryPlaySelectAnimation>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::reader::EventReader<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::event::reader::EventReader<country_core::systems::mode_manager::RequestHistoryCommand>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>)> >,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::mode_manager::PrevAppMode>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::ModeJustChanged>)> > > > >
14059f9ea: movups xmm0, xmmword ptr [rbx + 0x25f0]
14059f9f1: movaps xmmword ptr [rbx + 0xb60], xmm0
14059f9f8: movups xmm0, xmmword ptr [rbx + 0x25e0]
14059f9ff: movaps xmmword ptr [rbx + 0xb50], xmm0
14059fa06: movups xmm0, xmmword ptr [rbx + 0x25d0]
14059fa0d: movaps xmmword ptr [rbx + 0xb40], xmm0
14059fa14: movups xmm0, xmmword ptr [rbx + 0x2590]
14059fa1b: movups xmm1, xmmword ptr [rbx + 0x25a0]
14059fa22: movups xmm2, xmmword ptr [rbx + 0x25b0]
14059fa29: movups xmm3, xmmword ptr [rbx + 0x25c0]
14059fa30: movaps xmmword ptr [rbx + 0xb30], xmm3
14059fa37: movaps xmmword ptr [rbx + 0xb20], xmm2
14059fa3e: movaps xmmword ptr [rbx + 0xb10], xmm1
14059fa45: movaps xmmword ptr [rbx + 0xb00], xmm0
14059fa4c: movups xmm0, xmmword ptr [rbx + 0x440]
14059fa53: movups xmm1, xmmword ptr [rbx + 0x450]
14059fa5a: movups xmm2, xmmword ptr [rbx + 0x460]
14059fa61: movups xmm3, xmmword ptr [rbx + 0x470]
14059fa68: movaps xmmword ptr [rbx + 0xb70], xmm0
14059fa6f: movaps xmmword ptr [rbx + 0xb80], xmm1
14059fa76: movaps xmmword ptr [rbx + 0xb90], xmm2
14059fa7d: movaps xmmword ptr [rbx + 0xba0], xmm3
14059fa84: movups xmm0, xmmword ptr [rbx + 0x480]
14059fa8b: movaps xmmword ptr [rbx + 0xbb0], xmm0
14059fa92: movups xmm0, xmmword ptr [rbx + 0x490]
14059fa99: movaps xmmword ptr [rbx + 0xbc0], xmm0
14059faa0: movups xmm0, xmmword ptr [rbx + 0x4a0]
14059faa7: movaps xmmword ptr [rbx + 0xbd0], xmm0
14059faae: mov byte ptr [rbx + 0xaa], 1
14059fab5: mov byte ptr [rbx + 0xab], 1
14059fabc: mov byte ptr [rbx + 0xac], 1
14059fac3: mov byte ptr [rbx + 0xad], 1
14059faca: mov byte ptr [rbx + 0xae], 0
14059fad1: mov byte ptr [rbx + 0xaf], 1
14059fad8: mov byte ptr [rbx + 0xb0], 0
14059fadf: mov byte ptr [rbx + 0xb1], 1
14059fae6: mov byte ptr [rbx + 0xb2], 1
14059faed: lea r9, [rip + 0x245fd3c]
14059faf4: lea rcx, [rbx + 0x440]
14059fafb: lea rdx, [rbx + 0xb00]
14059fb02: mov r8b, 0xa
14059fb05: call 0x1400d6520 ; bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<tuple$<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > >,enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > > >,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<>,tuple$<> >,country_core::systems::GameSystemSet>
14059fb0a: mov byte ptr [rbx + 0xaa], 1
14059fb11: mov byte ptr [rbx + 0xab], 1
14059fb18: mov byte ptr [rbx + 0xac], 1
14059fb1f: mov byte ptr [rbx + 0xad], 1
14059fb26: mov byte ptr [rbx + 0xae], 0
14059fb2d: mov byte ptr [rbx + 0xaf], 1
14059fb34: mov byte ptr [rbx + 0xb0], 0
14059fb3b: mov byte ptr [rbx + 0xb1], 1
14059fb42: mov byte ptr [rbx + 0xb2], 1
14059fb49: lea rcx, [rbx + 0x24e8]
14059fb50: lea rdx, [rbx + 0x440]
14059fb57: call 0x1400d37d0 ; bevy_ecs::schedule::config::impl$3::run_if<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >)>,bool (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >)>
14059fb5c: lea rcx, [rbx + 0xb00]
14059fb63: call 0x1405389b0 ; bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred<tuple$<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::geometry::instanced_wall::InstancedWall> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>),void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::writer::EventWriter<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::BuildingHadArchesLastFrame>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >),void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::LintelSegments>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::geometry::instanced_wall::InstancedWall> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RoofIsInvalid>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::DeleteRoofCmd>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::writer::EventWriter<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::BuildingHadArchesLastFrame>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::LintelSegments>)> > > >
14059fb68: mov byte ptr [rbx + 0x440], 0x16
14059fb6f: mov rcx, qword ptr [rip + 0x308c7fa]
14059fb76: lea r8, [rip + 0x238b41b]
14059fb7d: lea rdx, [rbx + 0x440]
14059fb84: call 0x140364d20 ; bevy_ecs::intern::Interner::intern<dyn$<bevy_ecs::schedule::set::SystemSet> >
14059fb89: mov r8, rdx
14059fb8c: lea rcx, [rbx + 0xb00]
14059fb93: mov rdx, rax
14059fb96: call 0x1400d68c0 ; bevy_ecs::schedule::config::ScheduleConfigs::in_set_inner<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> >
14059fb9b: movups xmm0, xmmword ptr [rbx + 0xb60]
14059fba2: movaps xmmword ptr [rbx + 0x3d0], xmm0
14059fba9: movups xmm0, xmmword ptr [rbx + 0xb50]
14059fbb0: movaps xmmword ptr [rbx + 0x3c0], xmm0
14059fbb7: movups xmm0, xmmword ptr [rbx + 0xb40]
14059fbbe: movaps xmmword ptr [rbx + 0x3b0], xmm0
14059fbc5: movups xmm0, xmmword ptr [rbx + 0xb00]
14059fbcc: movups xmm1, xmmword ptr [rbx + 0xb10]
14059fbd3: movups xmm2, xmmword ptr [rbx + 0xb20]
14059fbda: movups xmm3, xmmword ptr [rbx + 0xb30]
14059fbe1: movaps xmmword ptr [rbx + 0x3a0], xmm3
14059fbe8: movaps xmmword ptr [rbx + 0x390], xmm2
14059fbef: movaps xmmword ptr [rbx + 0x380], xmm1
14059fbf6: movaps xmmword ptr [rbx + 0x370], xmm0
14059fbfd: lea r8, [rip + 0x245fc44]
14059fc04: lea rcx, [rbx + 0xb00]
14059fc0b: mov dl, 0x24
14059fc0d: call 0x1402cc220 ; bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::event::reader::EventReader<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::event::reader::EventReader<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> >,country_core::systems::GameSystemSet>
14059fc12: lea rcx, [rbx + 0x1ff0]
14059fc19: lea rdx, [rbx + 0xb00]
14059fc20: call 0x1400d37d0 ; bevy_ecs::schedule::config::impl$3::run_if<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >)>,bool (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >)>
14059fc25: lea r8, [rip + 0x245fc34]
14059fc2c: lea rcx, [rbx + 0xb00]
14059fc33: mov dl, 0x24
14059fc35: call 0x140524400 ; bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::system::system_param::Local<country_core::systems::save::load_completion_checker::LoadingCompletionState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::system::system_param::Local<country_core::systems::save::load_completion_checker::LoadingCompletionState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> >,country_core::systems::GameSystemSet>
14059fc3a: lea rcx, [rbx + 0x2230]
14059fc41: lea rdx, [rbx + 0xb00]
14059fc48: call 0x1400d37d0 ; bevy_ecs::schedule::config::impl$3::run_if<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >)>,bool (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >)>
14059fc4d: lea r8, [rip + 0x245fc24]
14059fc54: lea rcx, [rbx + 0x2bf0]
14059fc5b: mov dl, 0x24
14059fc5d: call 0x1403429a0 ; bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> >,country_core::systems::GameSystemSet>
14059fc62: lea r8, [rip + 0x245fc27]
14059fc69: lea rcx, [rbx + 0x2590]
14059fc70: mov dl, 0x2c
14059fc72: call 0x1403c7a30 ; bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<tuple$<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::Res<utils::camera::TaaJitterSet>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay> >,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,bevy_ecs::change_detection::ResMut<country_core::render::frame_constants::FrameConstants>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::structure_outline::OutlineStairsCmd>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::Res<country_core::resources::rt::RtBvhBuilderGlueResource>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::event::reader::EventReader<country_core::resources::rt::UpdateRtWorldEvent>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::rt::RtBvhBuilderGlueResource>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::Res<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::Res<utils::camera::TaaJitterSet>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay> >,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,bevy_ecs::change_detection::ResMut<country_core::render::frame_constants::FrameConstants>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::structure_outline::OutlineStairsCmd>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::Res<country_core::resources::rt::RtBvhBuilderGlueResource>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::event::reader::EventReader<country_core::resources::rt::UpdateRtWorldEvent>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::rt::RtBvhBuilderGlueResource>)> > >,country_core::systems::GameSystemSet>
14059fc77: lea r8, [rip + 0x245fc2a]
14059fc7e: lea rcx, [rbx + 0x440]
14059fc85: mov dl, 0xe
14059fc87: call 0x1405c0400 ; bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >,country_core::systems::GameSystemSet>
14059fc8c: movups xmm0, xmmword ptr [rbx + 0x2548]
14059fc93: movaps xmmword ptr [rbx + 0xb60], xmm0
14059fc9a: movups xmm0, xmmword ptr [rbx + 0x2538]
14059fca1: movaps xmmword ptr [rbx + 0xb50], xmm0
14059fca8: movups xmm0, xmmword ptr [rbx + 0x2528]
14059fcaf: movaps xmmword ptr [rbx + 0xb40], xmm0
14059fcb6: movups xmm0, xmmword ptr [rbx + 0x24e8]
14059fcbd: movups xmm1, xmmword ptr [rbx + 0x24f8]
14059fcc4: movups xmm2, xmmword ptr [rbx + 0x2508]
14059fccb: movups xmm3, xmmword ptr [rbx + 0x2518]
14059fcd2: movaps xmmword ptr [rbx + 0xb30], xmm3
14059fcd9: movaps xmmword ptr [rbx + 0xb20], xmm2
14059fce0: movaps xmmword ptr [rbx + 0xb10], xmm1
14059fce7: movaps xmmword ptr [rbx + 0xb00], xmm0
14059fcee: movaps xmm0, xmmword ptr [rbx + 0x370]
14059fcf5: movaps xmm1, xmmword ptr [rbx + 0x380]
14059fcfc: movaps xmm2, xmmword ptr [rbx + 0x390]
14059fd03: movaps xmm3, xmmword ptr [rbx + 0x3a0]
14059fd0a: movaps xmmword ptr [rbx + 0xb70], xmm0
14059fd11: movaps xmmword ptr [rbx + 0xb80], xmm1
14059fd18: movaps xmmword ptr [rbx + 0xb90], xmm2
14059fd1f: movaps xmmword ptr [rbx + 0xba0], xmm3
14059fd26: movaps xmm0, xmmword ptr [rbx + 0x3b0]
14059fd2d: movaps xmmword ptr [rbx + 0xbb0], xmm0
14059fd34: movaps xmm0, xmmword ptr [rbx + 0x3c0]
14059fd3b: movaps xmmword ptr [rbx + 0xbc0], xmm0
14059fd42: movaps xmm0, xmmword ptr [rbx + 0x3d0]
14059fd49: movaps xmmword ptr [rbx + 0xbd0], xmm0
14059fd50: movups xmm0, xmmword ptr [rbx + 0x2050]
14059fd57: movaps xmmword ptr [rbx + 0xc40], xmm0
14059fd5e: movups xmm0, xmmword ptr [rbx + 0x2040]
14059fd65: movaps xmmword ptr [rbx + 0xc30], xmm0
14059fd6c: movups xmm0, xmmword ptr [rbx + 0x2030]
14059fd73: movaps xmmword ptr [rbx + 0xc20], xmm0
14059fd7a: movups xmm0, xmmword ptr [rbx + 0x1ff0]
14059fd81: movups xmm1, xmmword ptr [rbx + 0x2000]
14059fd88: movups xmm2, xmmword ptr [rbx + 0x2010]
14059fd8f: movups xmm3, xmmword ptr [rbx + 0x2020]
14059fd96: movaps xmmword ptr [rbx + 0xc10], xmm3
14059fd9d: movaps xmmword ptr [rbx + 0xc00], xmm2
14059fda4: movaps xmmword ptr [rbx + 0xbf0], xmm1
14059fdab: movaps xmmword ptr [rbx + 0xbe0], xmm0
14059fdb2: movups xmm0, xmmword ptr [rbx + 0x2230]
14059fdb9: movups xmm1, xmmword ptr [rbx + 0x2240]
14059fdc0: movups xmm2, xmmword ptr [rbx + 0x2250]
14059fdc7: movups xmm3, xmmword ptr [rbx + 0x2260]
14059fdce: movaps xmmword ptr [rbx + 0xc50], xmm0
14059fdd5: movaps xmmword ptr [rbx + 0xc60], xmm1
14059fddc: movaps xmmword ptr [rbx + 0xc70], xmm2
14059fde3: movaps xmmword ptr [rbx + 0xc80], xmm3
14059fdea: movups xmm0, xmmword ptr [rbx + 0x2270]
14059fdf1: movaps xmmword ptr [rbx + 0xc90], xmm0
14059fdf8: movups xmm0, xmmword ptr [rbx + 0x2280]
14059fdff: movaps xmmword ptr [rbx + 0xca0], xmm0
14059fe06: movups xmm0, xmmword ptr [rbx + 0x2290]
14059fe0d: movaps xmmword ptr [rbx + 0xcb0], xmm0
14059fe14: movups xmm0, xmmword ptr [rbx + 0x2c50]
14059fe1b: movaps xmmword ptr [rbx + 0xd20], xmm0
14059fe22: movups xmm0, xmmword ptr [rbx + 0x2c40]
14059fe29: movaps xmmword ptr [rbx + 0xd10], xmm0
14059fe30: movups xmm0, xmmword ptr [rbx + 0x2c30]
14059fe37: movaps xmmword ptr [rbx + 0xd00], xmm0
14059fe3e: movups xmm0, xmmword ptr [rbx + 0x2bf0]
14059fe45: movups xmm1, xmmword ptr [rbx + 0x2c00]
14059fe4c: movups xmm2, xmmword ptr [rbx + 0x2c10]
14059fe53: movups xmm3, xmmword ptr [rbx + 0x2c20]
14059fe5a: movaps xmmword ptr [rbx + 0xcf0], xmm3
14059fe61: movaps xmmword ptr [rbx + 0xce0], xmm2
14059fe68: movaps xmmword ptr [rbx + 0xcd0], xmm1
14059fe6f: movaps xmmword ptr [rbx + 0xcc0], xmm0
14059fe76: movups xmm0, xmmword ptr [rbx + 0x25f0]
14059fe7d: movaps xmmword ptr [rbx + 0xd90], xmm0
14059fe84: movups xmm0, xmmword ptr [rbx + 0x25e0]
14059fe8b: movaps xmmword ptr [rbx + 0xd80], xmm0
14059fe92: movups xmm0, xmmword ptr [rbx + 0x25d0]
14059fe99: movaps xmmword ptr [rbx + 0xd70], xmm0
14059fea0: movups xmm0, xmmword ptr [rbx + 0x2590]
14059fea7: movups xmm1, xmmword ptr [rbx + 0x25a0]
14059feae: movups xmm2, xmmword ptr [rbx + 0x25b0]
14059feb5: movups xmm3, xmmword ptr [rbx + 0x25c0]
14059febc: movaps xmmword ptr [rbx + 0xd60], xmm3
14059fec3: movaps xmmword ptr [rbx + 0xd50], xmm2
14059feca: movaps xmmword ptr [rbx + 0xd40], xmm1
14059fed1: movaps xmmword ptr [rbx + 0xd30], xmm0
14059fed8: movups xmm0, xmmword ptr [rbx + 0x4a0]
14059fedf: movaps xmmword ptr [rbx + 0xe00], xmm0
14059fee6: movups xmm0, xmmword ptr [rbx + 0x490]
14059feed: movaps xmmword ptr [rbx + 0xdf0], xmm0
14059fef4: movups xmm0, xmmword ptr [rbx + 0x480]
14059fefb: movaps xmmword ptr [rbx + 0xde0], xmm0
14059ff02: movups xmm0, xmmword ptr [rbx + 0x440]
14059ff09: movups xmm1, xmmword ptr [rbx + 0x450]
14059ff10: movups xmm2, xmmword ptr [rbx + 0x460]
14059ff17: movups xmm3, xmmword ptr [rbx + 0x470]
14059ff1e: movaps xmmword ptr [rbx + 0xdd0], xmm3
14059ff25: movaps xmmword ptr [rbx + 0xdc0], xmm2
14059ff2c: movaps xmmword ptr [rbx + 0xdb0], xmm1
14059ff33: movaps xmmword ptr [rbx + 0xda0], xmm0
14059ff3a: mov byte ptr [rbx + 0xaa], 1
14059ff41: mov byte ptr [rbx + 0xab], 1
14059ff48: mov byte ptr [rbx + 0xac], 1
14059ff4f: mov byte ptr [rbx + 0xad], 1
14059ff56: mov byte ptr [rbx + 0xae], 0
14059ff5d: mov byte ptr [rbx + 0xaf], 1
14059ff64: mov byte ptr [rbx + 0xb0], 0
14059ff6b: mov byte ptr [rbx + 0xb1], 1
14059ff72: mov byte ptr [rbx + 0xb2], 1
14059ff79: lea rdx, [rbx + 0xb00]
14059ff80: mov rcx, rsi
14059ff83: call 0x140327490 ; bevy_app::sub_app::SubApp::add_systems<tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<>,tuple$<>,tuple$<>,tuple$<>,tuple$<>,tuple$<>,tuple$<> >,bevy_app::main_schedule::Update,tuple$<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > >,enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > >,enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > >,enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > >,enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > >,enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > >,enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > > > >
14059ff88: mov rcx, rsi
14059ff8b: add rcx, 0x38
14059ff8f: mov byte ptr [rbx + 0xaa], 1
14059ff96: mov byte ptr [rbx + 0xab], 1
14059ff9d: mov byte ptr [rbx + 0xac], 1
14059ffa4: mov byte ptr [rbx + 0xad], 1
14059ffab: mov byte ptr [rbx + 0xae], 0
14059ffb2: mov byte ptr [rbx + 0xaf], 1
14059ffb9: mov byte ptr [rbx + 0xb0], 0
14059ffc0: mov byte ptr [rbx + 0xb1], 1
14059ffc7: mov byte ptr [rbx + 0xb2], 1
14059ffce: movabs rdx, 0x1379125545270eb2
14059ffd8: movabs r8, 0x4eae26c0c92a587b
14059ffe2: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
14059ffe7: test rax, rax
14059ffea: je 0x1405a001f
14059ffec: cmp qword ptr [rsi + 0x218], rdx
14059fff3: jbe 0x1405a001f
14059fff5: mov rax, qword ptr [rsi + 0x210]
14059fffc: mov rax, qword ptr [rax + rdx*8]
1405a0000: test rax, rax
1405a0003: je 0x1405a001f
1405a0005: mov rcx, qword ptr [rsi + 0x1e0]
1405a000c: not rax
1405a000f: lea rax, [rax + rax*2]
1405a0013: shl rax, 5
1405a0017: cmp qword ptr [rcx + rax + 0x20], 0
1405a001d: jne 0x1405a0066
1405a001f: mov byte ptr [rbx + 0xaa], 1
1405a0026: mov byte ptr [rbx + 0xab], 1
1405a002d: mov byte ptr [rbx + 0xac], 1
1405a0034: mov byte ptr [rbx + 0xad], 1
1405a003b: mov byte ptr [rbx + 0xae], 0
1405a0042: mov byte ptr [rbx + 0xaf], 1
1405a0049: mov byte ptr [rbx + 0xb0], 0
1405a0050: mov byte ptr [rbx + 0xb1], 1
1405a0057: mov byte ptr [rbx + 0xb2], 1
1405a005e: mov rcx, rsi
1405a0061: call 0x14056d810 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::systems::ui_edit_focus::ToolSelectionCancelled>
1405a0066: mov rcx, qword ptr [rbx + 0x158]
1405a006d: lock inc qword ptr [rcx]
1405a0071: jle 0x1405a7040
1405a0077: mov byte ptr [rbx + 0xaa], 1
1405a007e: mov byte ptr [rbx + 0xab], 1
1405a0085: mov byte ptr [rbx + 0xac], 1
1405a008c: mov byte ptr [rbx + 0xad], 1
1405a0093: mov byte ptr [rbx + 0xae], 0
1405a009a: mov byte ptr [rbx + 0xaf], 1
1405a00a1: mov byte ptr [rbx + 0xb0], 0
1405a00a8: mov byte ptr [rbx + 0xb1], 1
1405a00af: mov byte ptr [rbx + 0xb2], 1
1405a00b6: call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
1405a00bb: mov byte ptr [rbx + 0xaa], 1
1405a00c2: mov byte ptr [rbx + 0xab], 1
1405a00c9: mov byte ptr [rbx + 0xac], 1
1405a00d0: mov byte ptr [rbx + 0xad], 1
1405a00d7: mov byte ptr [rbx + 0xae], 0
1405a00de: mov byte ptr [rbx + 0xaf], 1
1405a00e5: mov byte ptr [rbx + 0xb0], 0
1405a00ec: mov byte ptr [rbx + 0xb1], 1
1405a00f3: mov byte ptr [rbx + 0xb2], 1
1405a00fa: mov rcx, rsi
1405a00fd: mov rdx, rax
1405a0100: call 0x140138aa0 ; bevy_app::app::App::insert_resource<country_core::resources::rt::RtBvhBuilderGlueResource>
1405a0105: mov byte ptr [rbx + 0xaa], 1
1405a010c: mov byte ptr [rbx + 0xab], 1
1405a0113: mov byte ptr [rbx + 0xac], 1
1405a011a: mov byte ptr [rbx + 0xad], 1
1405a0121: mov byte ptr [rbx + 0xae], 0
1405a0128: mov byte ptr [rbx + 0xaf], 1
1405a012f: mov byte ptr [rbx + 0xb0], 0
1405a0136: mov byte ptr [rbx + 0xb1], 1
1405a013d: mov byte ptr [rbx + 0xb2], 1
1405a0144: mov rcx, rax
1405a0147: call 0x1406305f0 ; system_roof::impl$0::add_roof_events_and_resources
1405a014c: mov r15, rax
1405a014f: mov byte ptr [rbx + 0xaa], 1
1405a0156: mov byte ptr [rbx + 0xab], 1
1405a015d: mov byte ptr [rbx + 0xac], 1
1405a0164: mov byte ptr [rbx + 0xad], 1
1405a016b: mov byte ptr [rbx + 0xae], 0
1405a0172: mov byte ptr [rbx + 0xaf], 1
1405a0179: mov byte ptr [rbx + 0xb0], 0
1405a0180: mov byte ptr [rbx + 0xb1], 1
1405a0187: mov byte ptr [rbx + 0xb2], 1
1405a018e: lea r8, [rip + 0x245f72b]
1405a0195: lea rcx, [rbx + 0xb00]
1405a019c: mov dl, 7
1405a019e: call 0x140363710 ; bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<utils::components::transform::Transform> > >,ref$<country_core::systems::debug::human_scale_ref::HumanScaleRef> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::brush_preview::BrushPreview>,enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > > >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2DFollowMouse> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref_mut$<country_core::components::ArrowGizmo> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>),void (*)(bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::MeshFollowTerrain> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<utils::components::transform::Transform> > >,ref$<country_core::systems::debug::human_scale_ref::HumanScaleRef> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::brush_preview::BrushPreview>,enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > > >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2DFollowMouse> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref_mut$<country_core::components::ArrowGizmo> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::event::reader::EventReader<country_core::resources::window::CursorMovedScreenSpace>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::MeshFollowTerrain> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)> > >,country_core::systems::GameSystemSet>
1405a01a3: mov byte ptr [rbx + 0xaa], 1
1405a01aa: mov byte ptr [rbx + 0xab], 1
1405a01b1: mov byte ptr [rbx + 0xac], 1
1405a01b8: mov byte ptr [rbx + 0xad], 1
1405a01bf: mov byte ptr [rbx + 0xae], 0
1405a01c6: mov byte ptr [rbx + 0xaf], 1
1405a01cd: mov byte ptr [rbx + 0xb0], 0
1405a01d4: mov byte ptr [rbx + 0xb1], 1
1405a01db: mov byte ptr [rbx + 0xb2], 1
1405a01e2: lea rdx, [rbx + 0xb00]
1405a01e9: mov rcx, r15
1405a01ec: call 0x1403276f0 ; bevy_app::sub_app::SubApp::add_systems<tuple$<>,bevy_app::main_schedule::Update,enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > > >
1405a01f1: mov byte ptr [rbx + 0xaa], 1
1405a01f8: mov byte ptr [rbx + 0xab], 1
1405a01ff: mov byte ptr [rbx + 0xac], 1
1405a0206: mov byte ptr [rbx + 0xad], 1
1405a020d: mov byte ptr [rbx + 0xae], 0
1405a0214: mov byte ptr [rbx + 0xaf], 1
1405a021b: mov byte ptr [rbx + 0xb0], 0
1405a0222: mov byte ptr [rbx + 0xb1], 1
1405a0229: mov byte ptr [rbx + 0xb2], 1
1405a0230: lea rcx, [rbx + 0xb00]
1405a0237: call 0x1403770b0 ; bevy_ecs::schedule::config::IntoScheduleConfigs::run_if<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone1Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone2Raster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::Cobblestone3Raster> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::HalftimberColorIds> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::WallHighlight>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>)> >,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>)>,bool (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>)>
1405a023c: mov byte ptr [rbx + 0x2590], 0x21
1405a0243: mov rcx, qword ptr [rip + 0x308c126]
1405a024a: lea r8, [rip + 0x238ad47]
1405a0251: lea rdx, [rbx + 0x2590]
1405a0258: call 0x140364d20 ; bevy_ecs::intern::Interner::intern<dyn$<bevy_ecs::schedule::set::SystemSet> >
1405a025d: mov r8, rdx
1405a0260: lea rcx, [rbx + 0xb00]
1405a0267: mov rdx, rax
1405a026a: call 0x1400d68c0 ; bevy_ecs::schedule::config::ScheduleConfigs::in_set_inner<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> >
1405a026f: movups xmm0, xmmword ptr [rbx + 0xb60]
1405a0276: movaps xmmword ptr [rbx + 0x4a0], xmm0
1405a027d: movups xmm0, xmmword ptr [rbx + 0xb50]
1405a0284: movaps xmmword ptr [rbx + 0x490], xmm0
1405a028b: movups xmm0, xmmword ptr [rbx + 0xb40]
1405a0292: movaps xmmword ptr [rbx + 0x480], xmm0
1405a0299: movups xmm0, xmmword ptr [rbx + 0xb00]
1405a02a0: movups xmm1, xmmword ptr [rbx + 0xb10]
1405a02a7: movups xmm2, xmmword ptr [rbx + 0xb20]
1405a02ae: movups xmm3, xmmword ptr [rbx + 0xb30]
1405a02b5: movaps xmmword ptr [rbx + 0x470], xmm3
1405a02bc: movaps xmmword ptr [rbx + 0x460], xmm2
1405a02c3: movaps xmmword ptr [rbx + 0x450], xmm1
1405a02ca: movaps xmmword ptr [rbx + 0x440], xmm0
1405a02d1: mov byte ptr [rbx + 0xaa], 1
1405a02d8: mov byte ptr [rbx + 0xab], 1
1405a02df: mov byte ptr [rbx + 0xac], 1
1405a02e6: mov byte ptr [rbx + 0xad], 1
1405a02ed: mov byte ptr [rbx + 0xae], 0
1405a02f4: mov byte ptr [rbx + 0xaf], 1
1405a02fb: mov byte ptr [rbx + 0xb0], 0
1405a0302: mov byte ptr [rbx + 0xb1], 1
1405a0309: mov byte ptr [rbx + 0xb2], 1
1405a0310: lea rdx, [rbx + 0x440]
1405a0317: mov rcx, r15
1405a031a: call 0x1403276f0 ; bevy_app::sub_app::SubApp::add_systems<tuple$<>,bevy_app::main_schedule::Update,enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > > >
1405a031f: mov byte ptr [rbx + 0xaa], 1
1405a0326: mov byte ptr [rbx + 0xab], 1
1405a032d: mov byte ptr [rbx + 0xac], 1
1405a0334: mov byte ptr [rbx + 0xad], 1
1405a033b: mov byte ptr [rbx + 0xae], 0
1405a0342: mov byte ptr [rbx + 0xaf], 1
1405a0349: mov byte ptr [rbx + 0xb0], 0
1405a0350: mov byte ptr [rbx + 0xb1], 1
1405a0357: mov byte ptr [rbx + 0xb2], 1
1405a035e: lea rcx, [rbx + 0x2bf0]
1405a0365: call 0x14016bcd0 ; bevy_ecs::schedule::config::IntoScheduleConfigs::run_if<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > > >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::render::adaptive_render_state::AdaptiveRenderState> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,tuple$<bevy_ecs::event::writer::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::writer::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::TutorialOp>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorVisibilityCmd> > >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::gui::settings::TutorialUiState> >,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiControlsState>,bevy_ecs::event::reader::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::AvailableMonitors>,bevy_ecs::change_detection::Res<country_core::render::AvailableGraphicsDevices>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > > >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::render::adaptive_render_state::AdaptiveRenderState> >,bevy_ecs::change_detection::ResMut<tiny_glade::gui::settings::LastAppliedVideoSettings>,tuple$<bevy_ecs::event::writer::EventWriter<tiny_glade::app_config::ApplyVideoSettingsCmd>,bevy_ecs::event::writer::EventWriter<tiny_glade::app_config::WriteSettingsFileCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::TutorialOp>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorVisibilityCmd> > >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::gui::settings::TutorialUiState> >,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_controls::GuiControlsState>,bevy_ecs::event::reader::EventReader<bevy_input::keyboard::KeyboardInput>,bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseButtonInput>,bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>)> >,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >)>,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >)>
1405a036a: lea rcx, [rbx + 0x2590]
1405a0371: call 0x1405b6ee0 ; bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred<tuple$<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > > >
1405a0376: lea rcx, [rbx + 0xb00]
1405a037d: call 0x1402e8280 ; bevy_ecs::schedule::config::impl$4::into_configs<void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>),tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>)> >
1405a0382: lea rcx, [rbx + 0x440]
1405a0389: lea rdx, [rbx + 0xb00]
1405a0390: call 0x1400d3350 ; bevy_ecs::schedule::config::impl$3::run_if<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(bevy_ecs::event::reader::EventReader<tiny_glade::app_config::WriteSettingsFileCmd>)>,bool (*)(bevy_ecs::event::reader::EventReader<tiny_glade::app_config::WriteSettingsFileCmd>)>
1405a0395: movups xmm0, xmmword ptr [rbx + 0x2bf0]
1405a039c: movups xmm1, xmmword ptr [rbx + 0x2c00]
1405a03a3: movups xmm2, xmmword ptr [rbx + 0x2c10]
1405a03aa: movups xmm3, xmmword ptr [rbx + 0x2c20]
1405a03b1: movaps xmmword ptr [rbx + 0xb00], xmm0
1405a03b8: movaps xmmword ptr [rbx + 0xb10], xmm1
1405a03bf: movaps xmmword ptr [rbx + 0xb20], xmm2
1405a03c6: movaps xmmword ptr [rbx + 0xb30], xmm3
1405a03cd: movups xmm0, xmmword ptr [rbx + 0x2c30]
1405a03d4: movaps xmmword ptr [rbx + 0xb40], xmm0
1405a03db: movups xmm0, xmmword ptr [rbx + 0x2c40]
1405a03e2: movaps xmmword ptr [rbx + 0xb50], xmm0
1405a03e9: movups xmm0, xmmword ptr [rbx + 0x2c50]
1405a03f0: movaps xmmword ptr [rbx + 0xb60], xmm0
1405a03f7: movups xmm0, xmmword ptr [rbx + 0x2590]
1405a03fe: movups xmm1, xmmword ptr [rbx + 0x25a0]
1405a0405: movups xmm2, xmmword ptr [rbx + 0x25b0]
1405a040c: movups xmm3, xmmword ptr [rbx + 0x25c0]
1405a0413: movaps xmmword ptr [rbx + 0xb70], xmm0
1405a041a: movaps xmmword ptr [rbx + 0xb80], xmm1
1405a0421: movaps xmmword ptr [rbx + 0xb90], xmm2
1405a0428: movaps xmmword ptr [rbx + 0xba0], xmm3
1405a042f: movups xmm0, xmmword ptr [rbx + 0x25d0]
1405a0436: movaps xmmword ptr [rbx + 0xbb0], xmm0
1405a043d: movups xmm0, xmmword ptr [rbx + 0x25e0]
1405a0444: movaps xmmword ptr [rbx + 0xbc0], xmm0
1405a044b: movups xmm0, xmmword ptr [rbx + 0x25f0]
1405a0452: movaps xmmword ptr [rbx + 0xbd0], xmm0
1405a0459: movups xmm0, xmmword ptr [rbx + 0x4a0]
1405a0460: movaps xmmword ptr [rbx + 0xc40], xmm0
1405a0467: movups xmm0, xmmword ptr [rbx + 0x490]
1405a046e: movaps xmmword ptr [rbx + 0xc30], xmm0
1405a0475: movups xmm0, xmmword ptr [rbx + 0x480]
1405a047c: movaps xmmword ptr [rbx + 0xc20], xmm0
1405a0483: movups xmm0, xmmword ptr [rbx + 0x440]
1405a048a: movups xmm1, xmmword ptr [rbx + 0x450]
1405a0491: movups xmm2, xmmword ptr [rbx + 0x460]
1405a0498: movups xmm3, xmmword ptr [rbx + 0x470]
1405a049f: movaps xmmword ptr [rbx + 0xc10], xmm3
1405a04a6: movaps xmmword ptr [rbx + 0xc00], xmm2
1405a04ad: movaps xmmword ptr [rbx + 0xbf0], xmm1
1405a04b4: movaps xmmword ptr [rbx + 0xbe0], xmm0
1405a04bb: mov byte ptr [rbx + 0xaa], 1
1405a04c2: mov byte ptr [rbx + 0xab], 1
1405a04c9: mov byte ptr [rbx + 0xac], 1
1405a04d0: mov byte ptr [rbx + 0xad], 1
1405a04d7: mov byte ptr [rbx + 0xae], 0
1405a04de: mov byte ptr [rbx + 0xaf], 1
1405a04e5: mov byte ptr [rbx + 0xb0], 0
1405a04ec: mov byte ptr [rbx + 0xb1], 1
1405a04f3: mov byte ptr [rbx + 0xb2], 1
1405a04fa: lea r9, [rip + 0x245f3d7]
1405a0501: lea rcx, [rbx + 0x440]
1405a0508: lea rdx, [rbx + 0xb00]
1405a050f: mov r8b, 0x27
1405a0512: call 0x140611040 ; bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<tuple$<void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::vermeer::VermeerDebugControl>,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnAllClutterCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnClutterCmd> >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<country_core::systems::backend::icc::ActiveIccProfile>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_debug::AudioDebugState> >,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::systems::main_camera::CameraSnapshot>,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_debug::GpuAllocatorGui>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::logo_animation::LogoAnimation> > >,bevy_ecs::system::system_param::Local<country_core::systems::main_camera::CacheFreeCamRig>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >,tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::system::commands::Commands,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>),void (*)(bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::system::system_param::Local<usize>),enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > >,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_tod::TodButtonState>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >),enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > >,enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > >,void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::backend::grow_grass::GrowGrass>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_link_to_official_game::MessageState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::commands::Commands)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::vermeer::VermeerDebugControl>,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnAllClutterCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnClutterCmd> >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::Res<country_core::resources::grading_lut::GradingLuts>,bevy_ecs::change_detection::Res<country_core::systems::backend::icc::ActiveIccProfile>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_debug::AudioDebugState> >,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::systems::main_camera::CameraSnapshot>,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_debug::GpuAllocatorGui>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::logo_animation::LogoAnimation> > >,bevy_ecs::system::system_param::Local<country_core::systems::main_camera::CacheFreeCamRig>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >,tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::system::commands::Commands,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::system::system_param::Local<usize>)> >,tuple$<>,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_tod::TodButtonState>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)> >,tuple$<>,tuple$<>,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::backend::grow_grass::GrowGrass>,bevy_ecs::system::system_param::Local<tiny_glade::gui::gui_link_to_official_game::MessageState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::commands::Commands)> > >,country_core::systems::GameSystemSet>
1405a0517: mov byte ptr [rbx + 0xaa], 1
1405a051e: mov byte ptr [rbx + 0xab], 1
1405a0525: mov byte ptr [rbx + 0xac], 1
1405a052c: mov byte ptr [rbx + 0xad], 1
1405a0533: mov byte ptr [rbx + 0xae], 0
1405a053a: mov byte ptr [rbx + 0xaf], 1
1405a0541: mov byte ptr [rbx + 0xb0], 0
1405a0548: mov byte ptr [rbx + 0xb1], 1
1405a054f: mov byte ptr [rbx + 0xb2], 1
1405a0556: lea rdx, [rbx + 0x440]
1405a055d: mov rcx, r15
1405a0560: call 0x1403276f0 ; bevy_app::sub_app::SubApp::add_systems<tuple$<>,bevy_app::main_schedule::Update,enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > > >
1405a0565: mov byte ptr [rbx + 0xaa], 1
1405a056c: mov byte ptr [rbx + 0xab], 1
1405a0573: mov byte ptr [rbx + 0xac], 1
1405a057a: mov byte ptr [rbx + 0xad], 1
1405a0581: mov byte ptr [rbx + 0xae], 0
1405a0588: mov byte ptr [rbx + 0xaf], 1
1405a058f: mov byte ptr [rbx + 0xb0], 0
1405a0596: mov byte ptr [rbx + 0xb1], 1
1405a059d: mov byte ptr [rbx + 0xb2], 1
1405a05a4: mov rcx, r15
1405a05a7: call 0x140327d70 ; bevy_app::sub_app::SubApp::add_systems<tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>)> >,bevy_app::main_schedule::PostStartup,void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::input::panic_repro_state::PanicReproState>)>
1405a05ac: mov r12, r15
1405a05af: add r12, 0x38
1405a05b3: mov byte ptr [rbx + 0xaa], 1
1405a05ba: mov byte ptr [rbx + 0xab], 1
1405a05c1: mov byte ptr [rbx + 0xac], 1
1405a05c8: mov byte ptr [rbx + 0xad], 1
1405a05cf: mov byte ptr [rbx + 0xae], 0
1405a05d6: mov byte ptr [rbx + 0xaf], 1
1405a05dd: mov byte ptr [rbx + 0xb0], 0
1405a05e4: mov byte ptr [rbx + 0xb1], 1
1405a05eb: mov byte ptr [rbx + 0xb2], 1
1405a05f2: movabs rdx, 0xb8da66e7e904e3d
1405a05fc: movabs r8, 0x2fbe5d1891a41745
1405a0606: mov rcx, r12
1405a0609: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a060e: test rax, rax
1405a0611: je 0x1405a0646
1405a0613: cmp qword ptr [r15 + 0x218], rdx
1405a061a: jbe 0x1405a0646
1405a061c: mov rax, qword ptr [r15 + 0x210]
1405a0623: mov rax, qword ptr [rax + rdx*8]
1405a0627: test rax, rax
1405a062a: je 0x1405a0646
1405a062c: mov rcx, qword ptr [r15 + 0x1e0]
1405a0633: not rax
1405a0636: lea rax, [rax + rax*2]
1405a063a: shl rax, 5
1405a063e: cmp qword ptr [rcx + rax + 0x20], 0
1405a0644: jne 0x1405a068d
1405a0646: mov byte ptr [rbx + 0xaa], 1
1405a064d: mov byte ptr [rbx + 0xab], 1
1405a0654: mov byte ptr [rbx + 0xac], 1
1405a065b: mov byte ptr [rbx + 0xad], 1
1405a0662: mov byte ptr [rbx + 0xae], 0
1405a0669: mov byte ptr [rbx + 0xaf], 1
1405a0670: mov byte ptr [rbx + 0xb0], 0
1405a0677: mov byte ptr [rbx + 0xb1], 1
1405a067e: mov byte ptr [rbx + 0xb2], 1
1405a0685: mov rcx, r15
1405a0688: call 0x140568170 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>
1405a068d: mov byte ptr [rbx + 0xaa], 1
1405a0694: mov byte ptr [rbx + 0xab], 1
1405a069b: mov byte ptr [rbx + 0xac], 1
1405a06a2: mov byte ptr [rbx + 0xad], 1
1405a06a9: mov byte ptr [rbx + 0xae], 0
1405a06b0: mov byte ptr [rbx + 0xaf], 1
1405a06b7: mov byte ptr [rbx + 0xb0], 0
1405a06be: mov byte ptr [rbx + 0xb1], 1
1405a06c5: mov byte ptr [rbx + 0xb2], 1
1405a06cc: movabs rdx, 0x75bf40b77651060d
1405a06d6: movabs r8, 0xc86ab9d809a19210
1405a06e0: mov rcx, r12
1405a06e3: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a06e8: test rax, rax
1405a06eb: je 0x1405a0720
1405a06ed: cmp qword ptr [r15 + 0x218], rdx
1405a06f4: jbe 0x1405a0720
1405a06f6: mov rax, qword ptr [r15 + 0x210]
1405a06fd: mov rax, qword ptr [rax + rdx*8]
1405a0701: test rax, rax
1405a0704: je 0x1405a0720
1405a0706: mov rcx, qword ptr [r15 + 0x1e0]
1405a070d: not rax
1405a0710: lea rax, [rax + rax*2]
1405a0714: shl rax, 5
1405a0718: cmp qword ptr [rcx + rax + 0x20], 0
1405a071e: jne 0x1405a0767
1405a0720: mov byte ptr [rbx + 0xaa], 1
1405a0727: mov byte ptr [rbx + 0xab], 1
1405a072e: mov byte ptr [rbx + 0xac], 1
1405a0735: mov byte ptr [rbx + 0xad], 1
1405a073c: mov byte ptr [rbx + 0xae], 0
1405a0743: mov byte ptr [rbx + 0xaf], 1
1405a074a: mov byte ptr [rbx + 0xb0], 0
1405a0751: mov byte ptr [rbx + 0xb1], 1
1405a0758: mov byte ptr [rbx + 0xb2], 1
1405a075f: mov rcx, r15
1405a0762: call 0x140569400 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::resources::mouse_capture::CursorCaptureCmd>
1405a0767: mov byte ptr [rbx + 0xaa], 1
1405a076e: mov byte ptr [rbx + 0xab], 1
1405a0775: mov byte ptr [rbx + 0xac], 1
1405a077c: mov byte ptr [rbx + 0xad], 1
1405a0783: mov byte ptr [rbx + 0xae], 0
1405a078a: mov byte ptr [rbx + 0xaf], 1
1405a0791: mov byte ptr [rbx + 0xb0], 0
1405a0798: mov byte ptr [rbx + 0xb1], 1
1405a079f: mov byte ptr [rbx + 0xb2], 1
1405a07a6: movabs rdx, 0xbbdd82cfacba045f
1405a07b0: movabs r8, 0xcaf27ba849498836
1405a07ba: mov rcx, r12
1405a07bd: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a07c2: test rax, rax
1405a07c5: je 0x1405a07fa
1405a07c7: cmp qword ptr [r15 + 0x218], rdx
1405a07ce: jbe 0x1405a07fa
1405a07d0: mov rax, qword ptr [r15 + 0x210]
1405a07d7: mov rax, qword ptr [rax + rdx*8]
1405a07db: test rax, rax
1405a07de: je 0x1405a07fa
1405a07e0: mov rcx, qword ptr [r15 + 0x1e0]
1405a07e7: not rax
1405a07ea: lea rax, [rax + rax*2]
1405a07ee: shl rax, 5
1405a07f2: cmp qword ptr [rcx + rax + 0x20], 0
1405a07f8: jne 0x1405a0841
1405a07fa: mov byte ptr [rbx + 0xaa], 1
1405a0801: mov byte ptr [rbx + 0xab], 1
1405a0808: mov byte ptr [rbx + 0xac], 1
1405a080f: mov byte ptr [rbx + 0xad], 1
1405a0816: mov byte ptr [rbx + 0xae], 0
1405a081d: mov byte ptr [rbx + 0xaf], 1
1405a0824: mov byte ptr [rbx + 0xb0], 0
1405a082b: mov byte ptr [rbx + 0xb1], 1
1405a0832: mov byte ptr [rbx + 0xb2], 1
1405a0839: mov rcx, r15
1405a083c: call 0x140566280 ; bevy_ecs::event::registry::EventRegistry::register_event<enum2$<country_core::resources::mouse_capture::OsCursorCaptureCmd> >
1405a0841: mov byte ptr [rbx + 0xaa], 1
1405a0848: mov byte ptr [rbx + 0xab], 1
1405a084f: mov byte ptr [rbx + 0xac], 1
1405a0856: mov byte ptr [rbx + 0xad], 1
1405a085d: mov byte ptr [rbx + 0xae], 0
1405a0864: mov byte ptr [rbx + 0xaf], 1
1405a086b: mov byte ptr [rbx + 0xb0], 0
1405a0872: mov byte ptr [rbx + 0xb1], 1
1405a0879: mov byte ptr [rbx + 0xb2], 1
1405a0880: movabs rdx, 0x1c5d03917730cc4f
1405a088a: movabs r8, 0x3d29e4fb18bada6c
1405a0894: mov rcx, r12
1405a0897: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a089c: test rax, rax
1405a089f: je 0x1405a08d4
1405a08a1: cmp qword ptr [r15 + 0x218], rdx
1405a08a8: jbe 0x1405a08d4
1405a08aa: mov rax, qword ptr [r15 + 0x210]
1405a08b1: mov rax, qword ptr [rax + rdx*8]
1405a08b5: test rax, rax
1405a08b8: je 0x1405a08d4
1405a08ba: mov rcx, qword ptr [r15 + 0x1e0]
1405a08c1: not rax
1405a08c4: lea rax, [rax + rax*2]
1405a08c8: shl rax, 5
1405a08cc: cmp qword ptr [rcx + rax + 0x20], 0
1405a08d2: jne 0x1405a091b
1405a08d4: mov byte ptr [rbx + 0xaa], 1
1405a08db: mov byte ptr [rbx + 0xab], 1
1405a08e2: mov byte ptr [rbx + 0xac], 1
1405a08e9: mov byte ptr [rbx + 0xad], 1
1405a08f0: mov byte ptr [rbx + 0xae], 0
1405a08f7: mov byte ptr [rbx + 0xaf], 1
1405a08fe: mov byte ptr [rbx + 0xb0], 0
1405a0905: mov byte ptr [rbx + 0xb1], 1
1405a090c: mov byte ptr [rbx + 0xb2], 1
1405a0913: mov rcx, r15
1405a0916: call 0x14057e850 ; bevy_ecs::event::registry::EventRegistry::register_event<enum2$<country_core::resources::mouse_capture::OsCursorVisibilityCmd> >
1405a091b: mov byte ptr [rbx + 0xaa], 1
1405a0922: mov byte ptr [rbx + 0xab], 1
1405a0929: mov byte ptr [rbx + 0xac], 1
1405a0930: mov byte ptr [rbx + 0xad], 1
1405a0937: mov byte ptr [rbx + 0xae], 0
1405a093e: mov byte ptr [rbx + 0xaf], 1
1405a0945: mov byte ptr [rbx + 0xb0], 0
1405a094c: mov byte ptr [rbx + 0xb1], 1
1405a0953: mov byte ptr [rbx + 0xb2], 1
1405a095a: movabs rdx, 0x6ecb4684a4c9e1f9
1405a0964: movabs r8, 0xfae2e21abac81ddc
1405a096e: mov rcx, r12
1405a0971: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a0976: test rax, rax
1405a0979: je 0x1405a09ae
1405a097b: cmp qword ptr [r15 + 0x218], rdx
1405a0982: jbe 0x1405a09ae
1405a0984: mov rax, qword ptr [r15 + 0x210]
1405a098b: mov rax, qword ptr [rax + rdx*8]
1405a098f: test rax, rax
1405a0992: je 0x1405a09ae
1405a0994: mov rcx, qword ptr [r15 + 0x1e0]
1405a099b: not rax
1405a099e: lea rax, [rax + rax*2]
1405a09a2: shl rax, 5
1405a09a6: cmp qword ptr [rcx + rax + 0x20], 0
1405a09ac: jne 0x1405a09f5
1405a09ae: mov byte ptr [rbx + 0xaa], 1
1405a09b5: mov byte ptr [rbx + 0xab], 1
1405a09bc: mov byte ptr [rbx + 0xac], 1
1405a09c3: mov byte ptr [rbx + 0xad], 1
1405a09ca: mov byte ptr [rbx + 0xae], 0
1405a09d1: mov byte ptr [rbx + 0xaf], 1
1405a09d8: mov byte ptr [rbx + 0xb0], 0
1405a09df: mov byte ptr [rbx + 0xb1], 1
1405a09e6: mov byte ptr [rbx + 0xb2], 1
1405a09ed: mov rcx, r15
1405a09f0: call 0x140575a00 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>
1405a09f5: mov byte ptr [rbx + 0xaa], 1
1405a09fc: mov byte ptr [rbx + 0xab], 1
1405a0a03: mov byte ptr [rbx + 0xac], 1
1405a0a0a: mov byte ptr [rbx + 0xad], 1
1405a0a11: mov byte ptr [rbx + 0xae], 0
1405a0a18: mov byte ptr [rbx + 0xaf], 1
1405a0a1f: mov byte ptr [rbx + 0xb0], 0
1405a0a26: mov byte ptr [rbx + 0xb1], 1
1405a0a2d: mov byte ptr [rbx + 0xb2], 1
1405a0a34: movabs rdx, 0xdac2c590678dd6c0
1405a0a3e: movabs r8, 0x5d30eb6733ae4d59
1405a0a48: mov rcx, r12
1405a0a4b: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a0a50: test rax, rax
1405a0a53: je 0x1405a0a88
1405a0a55: cmp qword ptr [r15 + 0x218], rdx
1405a0a5c: jbe 0x1405a0a88
1405a0a5e: mov rax, qword ptr [r15 + 0x210]
1405a0a65: mov rax, qword ptr [rax + rdx*8]
1405a0a69: test rax, rax
1405a0a6c: je 0x1405a0a88
1405a0a6e: mov rcx, qword ptr [r15 + 0x1e0]
1405a0a75: not rax
1405a0a78: lea rax, [rax + rax*2]
1405a0a7c: shl rax, 5
1405a0a80: cmp qword ptr [rcx + rax + 0x20], 0
1405a0a86: jne 0x1405a0acf
1405a0a88: mov byte ptr [rbx + 0xaa], 1
1405a0a8f: mov byte ptr [rbx + 0xab], 1
1405a0a96: mov byte ptr [rbx + 0xac], 1
1405a0a9d: mov byte ptr [rbx + 0xad], 1
1405a0aa4: mov byte ptr [rbx + 0xae], 0
1405a0aab: mov byte ptr [rbx + 0xaf], 1
1405a0ab2: mov byte ptr [rbx + 0xb0], 0
1405a0ab9: mov byte ptr [rbx + 0xb1], 1
1405a0ac0: mov byte ptr [rbx + 0xb2], 1
1405a0ac7: mov rcx, r15
1405a0aca: call 0x140584520 ; bevy_ecs::event::registry::EventRegistry::register_event<enum2$<country_core::resources::window::OsWindowCmd> >
1405a0acf: mov byte ptr [rbx + 0xaa], 1
1405a0ad6: mov byte ptr [rbx + 0xab], 1
1405a0add: mov byte ptr [rbx + 0xac], 1
1405a0ae4: mov byte ptr [rbx + 0xad], 1
1405a0aeb: mov byte ptr [rbx + 0xae], 0
1405a0af2: mov byte ptr [rbx + 0xaf], 1
1405a0af9: mov byte ptr [rbx + 0xb0], 0
1405a0b00: mov byte ptr [rbx + 0xb1], 1
1405a0b07: mov byte ptr [rbx + 0xb2], 1
1405a0b0e: movabs rdx, 0x25b1b2cd3a0e7966
1405a0b18: movabs r8, 0x102186ba43f6414e
1405a0b22: mov rcx, r12
1405a0b25: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a0b2a: test rax, rax
1405a0b2d: je 0x1405a0b62
1405a0b2f: cmp qword ptr [r15 + 0x218], rdx
1405a0b36: jbe 0x1405a0b62
1405a0b38: mov rax, qword ptr [r15 + 0x210]
1405a0b3f: mov rax, qword ptr [rax + rdx*8]
1405a0b43: test rax, rax
1405a0b46: je 0x1405a0b62
1405a0b48: mov rcx, qword ptr [r15 + 0x1e0]
1405a0b4f: not rax
1405a0b52: lea rax, [rax + rax*2]
1405a0b56: shl rax, 5
1405a0b5a: cmp qword ptr [rcx + rax + 0x20], 0
1405a0b60: jne 0x1405a0ba9
1405a0b62: mov byte ptr [rbx + 0xaa], 1
1405a0b69: mov byte ptr [rbx + 0xab], 1
1405a0b70: mov byte ptr [rbx + 0xac], 1
1405a0b77: mov byte ptr [rbx + 0xad], 1
1405a0b7e: mov byte ptr [rbx + 0xae], 0
1405a0b85: mov byte ptr [rbx + 0xaf], 1
1405a0b8c: mov byte ptr [rbx + 0xb0], 0
1405a0b93: mov byte ptr [rbx + 0xb1], 1
1405a0b9a: mov byte ptr [rbx + 0xb2], 1
1405a0ba1: mov rcx, r15
1405a0ba4: call 0x14057cf90 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::resources::gui::EscapeMenuJustOpened>
1405a0ba9: mov byte ptr [rbx + 0xaa], 1
1405a0bb0: mov byte ptr [rbx + 0xab], 1
1405a0bb7: mov byte ptr [rbx + 0xac], 1
1405a0bbe: mov byte ptr [rbx + 0xad], 1
1405a0bc5: mov byte ptr [rbx + 0xae], 0
1405a0bcc: mov byte ptr [rbx + 0xaf], 1
1405a0bd3: mov byte ptr [rbx + 0xb0], 0
1405a0bda: mov byte ptr [rbx + 0xb1], 1
1405a0be1: mov byte ptr [rbx + 0xb2], 1
1405a0be8: movabs rdx, 0x1be1b6f9ce2b6798
1405a0bf2: movabs r8, 0xd606952bfb407b93
1405a0bfc: mov rcx, r12
1405a0bff: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a0c04: test rax, rax
1405a0c07: je 0x1405a0c3c
1405a0c09: cmp qword ptr [r15 + 0x218], rdx
1405a0c10: jbe 0x1405a0c3c
1405a0c12: mov rax, qword ptr [r15 + 0x210]
1405a0c19: mov rax, qword ptr [rax + rdx*8]
1405a0c1d: test rax, rax
1405a0c20: je 0x1405a0c3c
1405a0c22: mov rcx, qword ptr [r15 + 0x1e0]
1405a0c29: not rax
1405a0c2c: lea rax, [rax + rax*2]
1405a0c30: shl rax, 5
1405a0c34: cmp qword ptr [rcx + rax + 0x20], 0
1405a0c3a: jne 0x1405a0c83
1405a0c3c: mov byte ptr [rbx + 0xaa], 1
1405a0c43: mov byte ptr [rbx + 0xab], 1
1405a0c4a: mov byte ptr [rbx + 0xac], 1
1405a0c51: mov byte ptr [rbx + 0xad], 1
1405a0c58: mov byte ptr [rbx + 0xae], 0
1405a0c5f: mov byte ptr [rbx + 0xaf], 1
1405a0c66: mov byte ptr [rbx + 0xb0], 0
1405a0c6d: mov byte ptr [rbx + 0xb1], 1
1405a0c74: mov byte ptr [rbx + 0xb2], 1
1405a0c7b: mov rcx, r15
1405a0c7e: call 0x140571c20 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::resources::gui::EscapeMenuJustClosed>
1405a0c83: mov byte ptr [rbx + 0xaa], 1
1405a0c8a: mov byte ptr [rbx + 0xab], 1
1405a0c91: mov byte ptr [rbx + 0xac], 1
1405a0c98: mov byte ptr [rbx + 0xad], 1
1405a0c9f: mov byte ptr [rbx + 0xae], 0
1405a0ca6: mov byte ptr [rbx + 0xaf], 1
1405a0cad: mov byte ptr [rbx + 0xb0], 0
1405a0cb4: mov byte ptr [rbx + 0xb1], 1
1405a0cbb: mov byte ptr [rbx + 0xb2], 1
1405a0cc2: movabs rdx, 0xcf31a175ed46a479
1405a0ccc: movabs r8, 0x7ed3edadd1d139e1
1405a0cd6: mov rcx, r12
1405a0cd9: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a0cde: test rax, rax
1405a0ce1: je 0x1405a0d16
1405a0ce3: cmp qword ptr [r15 + 0x218], rdx
1405a0cea: jbe 0x1405a0d16
1405a0cec: mov rax, qword ptr [r15 + 0x210]
1405a0cf3: mov rax, qword ptr [rax + rdx*8]
1405a0cf7: test rax, rax
1405a0cfa: je 0x1405a0d16
1405a0cfc: mov rcx, qword ptr [r15 + 0x1e0]
1405a0d03: not rax
1405a0d06: lea rax, [rax + rax*2]
1405a0d0a: shl rax, 5
1405a0d0e: cmp qword ptr [rcx + rax + 0x20], 0
1405a0d14: jne 0x1405a0d5d
1405a0d16: mov byte ptr [rbx + 0xaa], 1
1405a0d1d: mov byte ptr [rbx + 0xab], 1
1405a0d24: mov byte ptr [rbx + 0xac], 1
1405a0d2b: mov byte ptr [rbx + 0xad], 1
1405a0d32: mov byte ptr [rbx + 0xae], 0
1405a0d39: mov byte ptr [rbx + 0xaf], 1
1405a0d40: mov byte ptr [rbx + 0xb0], 0
1405a0d47: mov byte ptr [rbx + 0xb1], 1
1405a0d4e: mov byte ptr [rbx + 0xb2], 1
1405a0d55: mov rcx, r15
1405a0d58: call 0x140586410 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::systems::mode_manager::RequestAppModeChange>
1405a0d5d: mov byte ptr [rbx + 0xaa], 1
1405a0d64: mov byte ptr [rbx + 0xab], 1
1405a0d6b: mov byte ptr [rbx + 0xac], 1
1405a0d72: mov byte ptr [rbx + 0xad], 1
1405a0d79: mov byte ptr [rbx + 0xae], 0
1405a0d80: mov byte ptr [rbx + 0xaf], 1
1405a0d87: mov byte ptr [rbx + 0xb0], 0
1405a0d8e: mov byte ptr [rbx + 0xb1], 1
1405a0d95: mov byte ptr [rbx + 0xb2], 1
1405a0d9c: movabs rdx, 0x2394d67f95c7f498
1405a0da6: movabs r8, 0x800129258aa79cc6
1405a0db0: mov rcx, r12
1405a0db3: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a0db8: test rax, rax
1405a0dbb: je 0x1405a0df0
1405a0dbd: cmp qword ptr [r15 + 0x218], rdx
1405a0dc4: jbe 0x1405a0df0
1405a0dc6: mov rax, qword ptr [r15 + 0x210]
1405a0dcd: mov rax, qword ptr [rax + rdx*8]
1405a0dd1: test rax, rax
1405a0dd4: je 0x1405a0df0
1405a0dd6: mov rcx, qword ptr [r15 + 0x1e0]
1405a0ddd: not rax
1405a0de0: lea rax, [rax + rax*2]
1405a0de4: shl rax, 5
1405a0de8: cmp qword ptr [rcx + rax + 0x20], 0
1405a0dee: jne 0x1405a0e37
1405a0df0: mov byte ptr [rbx + 0xaa], 1
1405a0df7: mov byte ptr [rbx + 0xab], 1
1405a0dfe: mov byte ptr [rbx + 0xac], 1
1405a0e05: mov byte ptr [rbx + 0xad], 1
1405a0e0c: mov byte ptr [rbx + 0xae], 0
1405a0e13: mov byte ptr [rbx + 0xaf], 1
1405a0e1a: mov byte ptr [rbx + 0xb0], 0
1405a0e21: mov byte ptr [rbx + 0xb1], 1
1405a0e28: mov byte ptr [rbx + 0xb2], 1
1405a0e2f: mov rcx, r15
1405a0e32: call 0x14056a060 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::systems::mode_manager::RequestToolbarCategoryChange>
1405a0e37: mov byte ptr [rbx + 0xaa], 1
1405a0e3e: mov byte ptr [rbx + 0xab], 1
1405a0e45: mov byte ptr [rbx + 0xac], 1
1405a0e4c: mov byte ptr [rbx + 0xad], 1
1405a0e53: mov byte ptr [rbx + 0xae], 0
1405a0e5a: mov byte ptr [rbx + 0xaf], 1
1405a0e61: mov byte ptr [rbx + 0xb0], 0
1405a0e68: mov byte ptr [rbx + 0xb1], 1
1405a0e6f: mov byte ptr [rbx + 0xb2], 1
1405a0e76: movabs rdx, 0xd1de6c9a7f2b6c4f
1405a0e80: movabs r8, 0xf073d18fea2ba51e
1405a0e8a: mov rcx, r12
1405a0e8d: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a0e92: test rax, rax
1405a0e95: je 0x1405a0eca
1405a0e97: cmp qword ptr [r15 + 0x218], rdx
1405a0e9e: jbe 0x1405a0eca
1405a0ea0: mov rax, qword ptr [r15 + 0x210]
1405a0ea7: mov rax, qword ptr [rax + rdx*8]
1405a0eab: test rax, rax
1405a0eae: je 0x1405a0eca
1405a0eb0: mov rcx, qword ptr [r15 + 0x1e0]
1405a0eb7: not rax
1405a0eba: lea rax, [rax + rax*2]
1405a0ebe: shl rax, 5
1405a0ec2: cmp qword ptr [rcx + rax + 0x20], 0
1405a0ec8: jne 0x1405a0f11
1405a0eca: mov byte ptr [rbx + 0xaa], 1
1405a0ed1: mov byte ptr [rbx + 0xab], 1
1405a0ed8: mov byte ptr [rbx + 0xac], 1
1405a0edf: mov byte ptr [rbx + 0xad], 1
1405a0ee6: mov byte ptr [rbx + 0xae], 0
1405a0eed: mov byte ptr [rbx + 0xaf], 1
1405a0ef4: mov byte ptr [rbx + 0xb0], 0
1405a0efb: mov byte ptr [rbx + 0xb1], 1
1405a0f02: mov byte ptr [rbx + 0xb2], 1
1405a0f09: mov rcx, r15
1405a0f0c: call 0x140585de0 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::systems::mode_manager::RequestHistoryCommand>
1405a0f11: mov byte ptr [rbx + 0xaa], 1
1405a0f18: mov byte ptr [rbx + 0xab], 1
1405a0f1f: mov byte ptr [rbx + 0xac], 1
1405a0f26: mov byte ptr [rbx + 0xad], 1
1405a0f2d: mov byte ptr [rbx + 0xae], 0
1405a0f34: mov byte ptr [rbx + 0xaf], 1
1405a0f3b: mov byte ptr [rbx + 0xb0], 0
1405a0f42: mov byte ptr [rbx + 0xb1], 1
1405a0f49: mov byte ptr [rbx + 0xb2], 1
1405a0f50: movabs rdx, 0xb4f0d2490f280d80
1405a0f5a: movabs r8, 0xf8449329f12602fe
1405a0f64: mov rcx, r12
1405a0f67: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a0f6c: test rax, rax
1405a0f6f: je 0x1405a0fa4
1405a0f71: cmp qword ptr [r15 + 0x218], rdx
1405a0f78: jbe 0x1405a0fa4
1405a0f7a: mov rax, qword ptr [r15 + 0x210]
1405a0f81: mov rax, qword ptr [rax + rdx*8]
1405a0f85: test rax, rax
1405a0f88: je 0x1405a0fa4
1405a0f8a: mov rcx, qword ptr [r15 + 0x1e0]
1405a0f91: not rax
1405a0f94: lea rax, [rax + rax*2]
1405a0f98: shl rax, 5
1405a0f9c: cmp qword ptr [rcx + rax + 0x20], 0
1405a0fa2: jne 0x1405a0feb
1405a0fa4: mov byte ptr [rbx + 0xaa], 1
1405a0fab: mov byte ptr [rbx + 0xab], 1
1405a0fb2: mov byte ptr [rbx + 0xac], 1
1405a0fb9: mov byte ptr [rbx + 0xad], 1
1405a0fc0: mov byte ptr [rbx + 0xae], 0
1405a0fc7: mov byte ptr [rbx + 0xaf], 1
1405a0fce: mov byte ptr [rbx + 0xb0], 0
1405a0fd5: mov byte ptr [rbx + 0xb1], 1
1405a0fdc: mov byte ptr [rbx + 0xb2], 1
1405a0fe3: mov rcx, r15
1405a0fe6: call 0x1405668b0 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::systems::mode_manager::UiHistoryPlaySelectAnimation>
1405a0feb: mov byte ptr [rbx + 0xaa], 1
1405a0ff2: mov byte ptr [rbx + 0xab], 1
1405a0ff9: mov byte ptr [rbx + 0xac], 1
1405a1000: mov byte ptr [rbx + 0xad], 1
1405a1007: mov byte ptr [rbx + 0xae], 0
1405a100e: mov byte ptr [rbx + 0xaf], 1
1405a1015: mov byte ptr [rbx + 0xb0], 0
1405a101c: mov byte ptr [rbx + 0xb1], 1
1405a1023: mov byte ptr [rbx + 0xb2], 1
1405a102a: movabs rdx, 0xa554fbb3bbf1ca0
1405a1034: movabs r8, 0x29b993a986bb8e9d
1405a103e: mov rcx, r12
1405a1041: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a1046: test rax, rax
1405a1049: je 0x1405a107e
1405a104b: cmp qword ptr [r15 + 0x218], rdx
1405a1052: jbe 0x1405a107e
1405a1054: mov rax, qword ptr [r15 + 0x210]
1405a105b: mov rax, qword ptr [rax + rdx*8]
1405a105f: test rax, rax
1405a1062: je 0x1405a107e
1405a1064: mov rcx, qword ptr [r15 + 0x1e0]
1405a106b: not rax
1405a106e: lea rax, [rax + rax*2]
1405a1072: shl rax, 5
1405a1076: cmp qword ptr [rcx + rax + 0x20], 0
1405a107c: jne 0x1405a10c5
1405a107e: mov byte ptr [rbx + 0xaa], 1
1405a1085: mov byte ptr [rbx + 0xab], 1
1405a108c: mov byte ptr [rbx + 0xac], 1
1405a1093: mov byte ptr [rbx + 0xad], 1
1405a109a: mov byte ptr [rbx + 0xae], 0
1405a10a1: mov byte ptr [rbx + 0xaf], 1
1405a10a8: mov byte ptr [rbx + 0xb0], 0
1405a10af: mov byte ptr [rbx + 0xb1], 1
1405a10b6: mov byte ptr [rbx + 0xb2], 1
1405a10bd: mov rcx, r15
1405a10c0: call 0x1405797e0 ; bevy_ecs::event::registry::EventRegistry::register_event<tiny_glade::app_config::WriteSettingsFileCmd>
1405a10c5: mov byte ptr [rbx + 0xaa], 1
1405a10cc: mov byte ptr [rbx + 0xab], 1
1405a10d3: mov byte ptr [rbx + 0xac], 1
1405a10da: mov byte ptr [rbx + 0xad], 1
1405a10e1: mov byte ptr [rbx + 0xae], 0
1405a10e8: mov byte ptr [rbx + 0xaf], 1
1405a10ef: mov byte ptr [rbx + 0xb0], 0
1405a10f6: mov byte ptr [rbx + 0xb1], 1
1405a10fd: mov byte ptr [rbx + 0xb2], 1
1405a1104: movabs rdx, 0xdb5e6652a838d059
1405a110e: movabs r8, 0x820aea602265b9f1
1405a1118: mov rcx, r12
1405a111b: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a1120: test rax, rax
1405a1123: je 0x1405a1158
1405a1125: cmp qword ptr [r15 + 0x218], rdx
1405a112c: jbe 0x1405a1158
1405a112e: mov rax, qword ptr [r15 + 0x210]
1405a1135: mov rax, qword ptr [rax + rdx*8]
1405a1139: test rax, rax
1405a113c: je 0x1405a1158
1405a113e: mov rcx, qword ptr [r15 + 0x1e0]
1405a1145: not rax
1405a1148: lea rax, [rax + rax*2]
1405a114c: shl rax, 5
1405a1150: cmp qword ptr [rcx + rax + 0x20], 0
1405a1156: jne 0x1405a119f
1405a1158: mov byte ptr [rbx + 0xaa], 1
1405a115f: mov byte ptr [rbx + 0xab], 1
1405a1166: mov byte ptr [rbx + 0xac], 1
1405a116d: mov byte ptr [rbx + 0xad], 1
1405a1174: mov byte ptr [rbx + 0xae], 0
1405a117b: mov byte ptr [rbx + 0xaf], 1
1405a1182: mov byte ptr [rbx + 0xb0], 0
1405a1189: mov byte ptr [rbx + 0xb1], 1
1405a1190: mov byte ptr [rbx + 0xb2], 1
1405a1197: mov rcx, r15
1405a119a: call 0x14057dbf0 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>
1405a119f: mov byte ptr [rbx + 0xaa], 1
1405a11a6: mov byte ptr [rbx + 0xab], 1
1405a11ad: mov byte ptr [rbx + 0xac], 1
1405a11b4: mov byte ptr [rbx + 0xad], 1
1405a11bb: mov byte ptr [rbx + 0xae], 0
1405a11c2: mov byte ptr [rbx + 0xaf], 1
1405a11c9: mov byte ptr [rbx + 0xb0], 0
1405a11d0: mov byte ptr [rbx + 0xb1], 1
1405a11d7: mov byte ptr [rbx + 0xb2], 1
1405a11de: movabs rdx, 0x3a04fec58ff7409b
1405a11e8: movabs r8, 0xccf689c1cb3e03a2
1405a11f2: mov rcx, r12
1405a11f5: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a11fa: test rax, rax
1405a11fd: je 0x1405a1232
1405a11ff: cmp qword ptr [r15 + 0x218], rdx
1405a1206: jbe 0x1405a1232
1405a1208: mov rax, qword ptr [r15 + 0x210]
1405a120f: mov rax, qword ptr [rax + rdx*8]
1405a1213: test rax, rax
1405a1216: je 0x1405a1232
1405a1218: mov rcx, qword ptr [r15 + 0x1e0]
1405a121f: not rax
1405a1222: lea rax, [rax + rax*2]
1405a1226: shl rax, 5
1405a122a: cmp qword ptr [rcx + rax + 0x20], 0
1405a1230: jne 0x1405a1279
1405a1232: mov byte ptr [rbx + 0xaa], 1
1405a1239: mov byte ptr [rbx + 0xab], 1
1405a1240: mov byte ptr [rbx + 0xac], 1
1405a1247: mov byte ptr [rbx + 0xad], 1
1405a124e: mov byte ptr [rbx + 0xae], 0
1405a1255: mov byte ptr [rbx + 0xaf], 1
1405a125c: mov byte ptr [rbx + 0xb0], 0
1405a1263: mov byte ptr [rbx + 0xb1], 1
1405a126a: mov byte ptr [rbx + 0xb2], 1
1405a1271: mov rcx, r15
1405a1274: call 0x1405819d0 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::systems::timelapse_replay::TimelapseReplayFinished>
1405a1279: mov byte ptr [rbx + 0xaa], 1
1405a1280: mov byte ptr [rbx + 0xab], 1
1405a1287: mov byte ptr [rbx + 0xac], 1
1405a128e: mov byte ptr [rbx + 0xad], 1
1405a1295: mov byte ptr [rbx + 0xae], 0
1405a129c: mov byte ptr [rbx + 0xaf], 1
1405a12a3: mov byte ptr [rbx + 0xb0], 0
1405a12aa: mov byte ptr [rbx + 0xb1], 1
1405a12b1: mov byte ptr [rbx + 0xb2], 1
1405a12b8: movabs rdx, 0x2c026e94513be739
1405a12c2: movabs r8, 0xa05489b04f4c6987
1405a12cc: mov rcx, r12
1405a12cf: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a12d4: test rax, rax
1405a12d7: je 0x1405a130c
1405a12d9: cmp qword ptr [r15 + 0x218], rdx
1405a12e0: jbe 0x1405a130c
1405a12e2: mov rax, qword ptr [r15 + 0x210]
1405a12e9: mov rax, qword ptr [rax + rdx*8]
1405a12ed: test rax, rax
1405a12f0: je 0x1405a130c
1405a12f2: mov rcx, qword ptr [r15 + 0x1e0]
1405a12f9: not rax
1405a12fc: lea rax, [rax + rax*2]
1405a1300: shl rax, 5
1405a1304: cmp qword ptr [rcx + rax + 0x20], 0
1405a130a: jne 0x1405a1353
1405a130c: mov byte ptr [rbx + 0xaa], 1
1405a1313: mov byte ptr [rbx + 0xab], 1
1405a131a: mov byte ptr [rbx + 0xac], 1
1405a1321: mov byte ptr [rbx + 0xad], 1
1405a1328: mov byte ptr [rbx + 0xae], 0
1405a132f: mov byte ptr [rbx + 0xaf], 1
1405a1336: mov byte ptr [rbx + 0xb0], 0
1405a133d: mov byte ptr [rbx + 0xb1], 1
1405a1344: mov byte ptr [rbx + 0xb2], 1
1405a134b: mov rcx, r15
1405a134e: call 0x14057d5c0 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::systems::timelapse_replay::InterruptTimelapse>
1405a1353: mov byte ptr [rbx + 0xaa], 1
1405a135a: mov byte ptr [rbx + 0xab], 1
1405a1361: mov byte ptr [rbx + 0xac], 1
1405a1368: mov byte ptr [rbx + 0xad], 1
1405a136f: mov byte ptr [rbx + 0xae], 0
1405a1376: mov byte ptr [rbx + 0xaf], 1
1405a137d: mov byte ptr [rbx + 0xb0], 0
1405a1384: mov byte ptr [rbx + 0xb1], 1
1405a138b: mov byte ptr [rbx + 0xb2], 1
1405a1392: movabs rdx, 0x3b770906f36e3dd8
1405a139c: movabs r8, 0x1bce870c4bce18ee
1405a13a6: mov rcx, r12
1405a13a9: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a13ae: test rax, rax
1405a13b1: je 0x1405a13e6
1405a13b3: cmp qword ptr [r15 + 0x218], rdx
1405a13ba: jbe 0x1405a13e6
1405a13bc: mov rax, qword ptr [r15 + 0x210]
1405a13c3: mov rax, qword ptr [rax + rdx*8]
1405a13c7: test rax, rax
1405a13ca: je 0x1405a13e6
1405a13cc: mov rcx, qword ptr [r15 + 0x1e0]
1405a13d3: not rax
1405a13d6: lea rax, [rax + rax*2]
1405a13da: shl rax, 5
1405a13de: cmp qword ptr [rcx + rax + 0x20], 0
1405a13e4: jne 0x1405a142d
1405a13e6: mov byte ptr [rbx + 0xaa], 1
1405a13ed: mov byte ptr [rbx + 0xab], 1
1405a13f4: mov byte ptr [rbx + 0xac], 1
1405a13fb: mov byte ptr [rbx + 0xad], 1
1405a1402: mov byte ptr [rbx + 0xae], 0
1405a1409: mov byte ptr [rbx + 0xaf], 1
1405a1410: mov byte ptr [rbx + 0xb0], 0
1405a1417: mov byte ptr [rbx + 0xb1], 1
1405a141e: mov byte ptr [rbx + 0xb2], 1
1405a1425: mov rcx, r15
1405a1428: call 0x14057ee80 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::systems::backend::grow_grass::GrowGrass>
1405a142d: mov byte ptr [rbx + 0xaa], 1
1405a1434: mov byte ptr [rbx + 0xab], 1
1405a143b: mov byte ptr [rbx + 0xac], 1
1405a1442: mov byte ptr [rbx + 0xad], 1
1405a1449: mov byte ptr [rbx + 0xae], 0
1405a1450: mov byte ptr [rbx + 0xaf], 1
1405a1457: mov byte ptr [rbx + 0xb0], 0
1405a145e: mov byte ptr [rbx + 0xb1], 1
1405a1465: mov byte ptr [rbx + 0xb2], 1
1405a146c: mov rcx, r15
1405a146f: mov dl, 2
1405a1471: call 0x1401295c0 ; bevy_app::app::App::insert_resource<enum2$<country_core::systems::timelapse_replay::TimelapseState> >
1405a1476: mov rsi, rax
1405a1479: mov qword ptr [rbx + 0x440], 0
1405a1484: xorps xmm0, xmm0
1405a1487: movups xmmword ptr [rbx + 0x450], xmm0
1405a148e: mov qword ptr [rbx + 0x448], 8
1405a1499: call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1405a149e: mov rdi, rax
1405a14a1: test byte ptr [rax], 1
1405a14a4: je 0x1405a14b7
1405a14a6: mov rcx, rdi
1405a14a9: add rcx, 8
1405a14ad: mov rax, qword ptr [rdi + 8]
1405a14b1: mov rdx, qword ptr [rdi + 0x10]
1405a14b5: jmp 0x1405a14d2
1405a14b7: call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1405a14bc: mov rcx, rdi
1405a14bf: add rcx, 8
1405a14c3: mov qword ptr [rdi], 1
1405a14ca: mov qword ptr [rdi + 8], rax
1405a14ce: mov qword ptr [rdi + 0x10], rdx
1405a14d2: lea r8, [rax + 1]
1405a14d6: mov qword ptr [rcx], r8
1405a14d9: movups xmm0, xmmword ptr [rbx + 0x440]
1405a14e0: movups xmm1, xmmword ptr [rbx + 0x450]
1405a14e7: movaps xmmword ptr [rbx + 0xb10], xmm1
1405a14ee: movaps xmmword ptr [rbx + 0xb00], xmm0
1405a14f5: movaps xmmword ptr [rbx + 0xb20], xmm7
1405a14fc: movaps xmmword ptr [rbx + 0xb30], xmm6
1405a1503: mov qword ptr [rbx + 0xb40], rax
1405a150a: mov qword ptr [rbx + 0xb48], rdx
1405a1511: mov byte ptr [rbx + 0xaa], 1
1405a1518: mov byte ptr [rbx + 0xab], 1
1405a151f: mov byte ptr [rbx + 0xac], 1
1405a1526: mov byte ptr [rbx + 0xad], 1
1405a152d: mov byte ptr [rbx + 0xae], 0
1405a1534: mov byte ptr [rbx + 0xaf], 1
1405a153b: mov byte ptr [rbx + 0xb0], 0
1405a1542: mov byte ptr [rbx + 0xb1], 1
1405a1549: mov byte ptr [rbx + 0xb2], 1
1405a1550: lea rdx, [rbx + 0xb00]
1405a1557: mov rcx, rsi
1405a155a: call 0x140118630 ; bevy_app::app::App::insert_resource<country_core::meta_commander::MetaCommander>
1405a155f: mov rsi, rax
1405a1562: mov byte ptr [rbx + 0xaa], 1
1405a1569: mov byte ptr [rbx + 0xab], 1
1405a1570: mov byte ptr [rbx + 0xac], 1
1405a1577: mov byte ptr [rbx + 0xad], 1
1405a157e: mov byte ptr [rbx + 0xae], 0
1405a1585: mov byte ptr [rbx + 0xaf], 1
1405a158c: mov byte ptr [rbx + 0xb0], 0
1405a1593: mov byte ptr [rbx + 0xb1], 1
1405a159a: mov byte ptr [rbx + 0xb2], 1
1405a15a1: lea rcx, [rbx + 0xb00]
1405a15a8: call 0x140a86b30 ; parry3d::shape::shape::Shape::feature_normal_at_point<parry3d::shape::compound::Compound>
1405a15ad: mov byte ptr [rbx + 0xaa], 1
1405a15b4: mov byte ptr [rbx + 0xab], 1
1405a15bb: mov byte ptr [rbx + 0xac], 1
1405a15c2: mov byte ptr [rbx + 0xad], 1
1405a15c9: mov byte ptr [rbx + 0xae], 0
1405a15d0: mov byte ptr [rbx + 0xaf], 1
1405a15d7: mov byte ptr [rbx + 0xb0], 0
1405a15de: mov byte ptr [rbx + 0xb1], 1
1405a15e5: mov byte ptr [rbx + 0xb2], 1
1405a15ec: lea rdx, [rbx + 0xb00]
1405a15f3: mov rcx, rsi
1405a15f6: call 0x1401180f0 ; bevy_app::app::App::insert_resource<country_core::resources::mouse_capture::CursorCapturedPosition>
1405a15fb: mov rsi, rax
1405a15fe: call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1405a1603: mov rdi, rax
1405a1606: test byte ptr [rax], 1
1405a1609: je 0x1405a161c
1405a160b: mov rcx, rdi
1405a160e: add rcx, 8
1405a1612: mov rax, qword ptr [rdi + 8]
1405a1616: mov rdx, qword ptr [rdi + 0x10]
1405a161a: jmp 0x1405a1676
1405a161c: mov byte ptr [rbx + 0xaa], 1
1405a1623: mov byte ptr [rbx + 0xab], 1
1405a162a: mov byte ptr [rbx + 0xac], 1
1405a1631: mov byte ptr [rbx + 0xad], 1
1405a1638: mov byte ptr [rbx + 0xae], 0
1405a163f: mov byte ptr [rbx + 0xaf], 1
1405a1646: mov byte ptr [rbx + 0xb0], 0
1405a164d: mov byte ptr [rbx + 0xb1], 1
1405a1654: mov byte ptr [rbx + 0xb2], 1
1405a165b: call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1405a1660: mov rcx, rdi
1405a1663: add rcx, 8
1405a1667: mov qword ptr [rdi], 1
1405a166e: mov qword ptr [rdi + 8], rax
1405a1672: mov qword ptr [rdi + 0x10], rdx
1405a1676: lea r8, [rax + 1]
1405a167a: mov qword ptr [rcx], r8
1405a167d: movaps xmmword ptr [rbx + 0xb10], xmm6
1405a1684: movaps xmmword ptr [rbx + 0xb00], xmm7
1405a168b: mov qword ptr [rbx + 0xb20], rax
1405a1692: mov qword ptr [rbx + 0xb28], rdx
1405a1699: mov byte ptr [rbx + 0xaa], 1
1405a16a0: mov byte ptr [rbx + 0xab], 1
1405a16a7: mov byte ptr [rbx + 0xac], 1
1405a16ae: mov byte ptr [rbx + 0xad], 1
1405a16b5: mov byte ptr [rbx + 0xae], 0
1405a16bc: mov byte ptr [rbx + 0xaf], 1
1405a16c3: mov byte ptr [rbx + 0xb0], 0
1405a16ca: mov byte ptr [rbx + 0xb1], 1
1405a16d1: mov byte ptr [rbx + 0xb2], 1
1405a16d8: lea rdx, [rbx + 0xb00]
1405a16df: mov rcx, rsi
1405a16e2: call 0x140131170 ; bevy_app::app::App::insert_resource<country_core::resources::walls::wall_holes::WallHoles>
1405a16e7: mov rsi, rax
1405a16ea: call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1405a16ef: mov rdi, rax
1405a16f2: test byte ptr [rax], 1
1405a16f5: je 0x1405a1708
1405a16f7: mov rcx, rdi
1405a16fa: add rcx, 8
1405a16fe: mov rax, qword ptr [rdi + 8]
1405a1702: mov rdx, qword ptr [rdi + 0x10]
1405a1706: jmp 0x1405a1762
1405a1708: mov byte ptr [rbx + 0xaa], 1
1405a170f: mov byte ptr [rbx + 0xab], 1
1405a1716: mov byte ptr [rbx + 0xac], 1
1405a171d: mov byte ptr [rbx + 0xad], 1
1405a1724: mov byte ptr [rbx + 0xae], 0
1405a172b: mov byte ptr [rbx + 0xaf], 1
1405a1732: mov byte ptr [rbx + 0xb0], 0
1405a1739: mov byte ptr [rbx + 0xb1], 1
1405a1740: mov byte ptr [rbx + 0xb2], 1
1405a1747: call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1405a174c: mov rcx, rdi
1405a174f: add rcx, 8
1405a1753: mov qword ptr [rdi], 1
1405a175a: mov qword ptr [rdi + 8], rax
1405a175e: mov qword ptr [rdi + 0x10], rdx
1405a1762: lea r8, [rax + 1]
1405a1766: mov qword ptr [rcx], r8
1405a1769: movaps xmmword ptr [rbx + 0xb10], xmm6
1405a1770: movaps xmmword ptr [rbx + 0xb00], xmm7
1405a1777: mov qword ptr [rbx + 0xb20], rax
1405a177e: mov qword ptr [rbx + 0xb28], rdx
1405a1785: mov byte ptr [rbx + 0xaa], 1
1405a178c: mov byte ptr [rbx + 0xab], 1
1405a1793: mov byte ptr [rbx + 0xac], 1
1405a179a: mov byte ptr [rbx + 0xad], 1
1405a17a1: mov byte ptr [rbx + 0xae], 0
1405a17a8: mov byte ptr [rbx + 0xaf], 1
1405a17af: mov byte ptr [rbx + 0xb0], 0
1405a17b6: mov byte ptr [rbx + 0xb1], 1
1405a17bd: mov byte ptr [rbx + 0xb2], 1
1405a17c4: lea rdx, [rbx + 0xb00]
1405a17cb: mov rcx, rsi
1405a17ce: call 0x14011f240 ; bevy_app::app::App::insert_resource<country_core::resources::walls::wall_holes::PrevWallHoles>
1405a17d3: mov rsi, rax
1405a17d6: mov byte ptr [rbx + 0xaa], 1
1405a17dd: mov byte ptr [rbx + 0xab], 1
1405a17e4: mov byte ptr [rbx + 0xac], 1
1405a17eb: mov byte ptr [rbx + 0xad], 1
1405a17f2: mov byte ptr [rbx + 0xae], 0
1405a17f9: mov byte ptr [rbx + 0xaf], 1
1405a1800: mov byte ptr [rbx + 0xb0], 0
1405a1807: mov byte ptr [rbx + 0xb1], 1
1405a180e: mov byte ptr [rbx + 0xb2], 1
1405a1815: lea rcx, [rbx + 0xb00]
1405a181c: call 0x140af8c50 ; country_core::resources::toolbar::Toolbar::new
1405a1821: mov byte ptr [rbx + 0xaa], 1
1405a1828: mov byte ptr [rbx + 0xab], 1
1405a182f: mov byte ptr [rbx + 0xac], 1
1405a1836: mov byte ptr [rbx + 0xad], 1
1405a183d: mov byte ptr [rbx + 0xae], 0
1405a1844: mov byte ptr [rbx + 0xaf], 1
1405a184b: mov byte ptr [rbx + 0xb0], 0
1405a1852: mov byte ptr [rbx + 0xb1], 1
1405a1859: mov byte ptr [rbx + 0xb2], 1
1405a1860: lea rdx, [rbx + 0xb00]
1405a1867: mov rcx, rsi
1405a186a: call 0x14013f8f0 ; bevy_app::app::App::insert_resource<country_core::resources::toolbar::Toolbar>
1405a186f: mov rsi, rax
1405a1872: mov byte ptr [rbx + 0xaa], 1
1405a1879: mov byte ptr [rbx + 0xab], 1
1405a1880: mov byte ptr [rbx + 0xac], 1
1405a1887: mov byte ptr [rbx + 0xad], 1
1405a188e: mov byte ptr [rbx + 0xae], 0
1405a1895: mov byte ptr [rbx + 0xaf], 1
1405a189c: mov byte ptr [rbx + 0xb0], 0
1405a18a3: mov byte ptr [rbx + 0xb1], 1
1405a18aa: mov byte ptr [rbx + 0xb2], 1
1405a18b1: lea rcx, [rbx + 0xb00]
1405a18b8: call 0x1408c2240 ; country_core::resources::glade_settings::impl$1::default
1405a18bd: mov byte ptr [rbx + 0xaa], 1
1405a18c4: mov byte ptr [rbx + 0xab], 1
1405a18cb: mov byte ptr [rbx + 0xac], 1
1405a18d2: mov byte ptr [rbx + 0xad], 1
1405a18d9: mov byte ptr [rbx + 0xae], 0
1405a18e0: mov byte ptr [rbx + 0xaf], 1
1405a18e7: mov byte ptr [rbx + 0xb0], 0
1405a18ee: mov byte ptr [rbx + 0xb1], 1
1405a18f5: mov byte ptr [rbx + 0xb2], 1
1405a18fc: lea rdx, [rbx + 0xb00]
1405a1903: mov rcx, rsi
1405a1906: call 0x14011d6b0 ; bevy_app::app::App::insert_resource<country_core::resources::glade_settings::GladeSettings>
1405a190b: mov byte ptr [rbx + 0xaa], 1
1405a1912: mov byte ptr [rbx + 0xab], 1
1405a1919: mov byte ptr [rbx + 0xac], 1
1405a1920: mov byte ptr [rbx + 0xad], 1
1405a1927: mov byte ptr [rbx + 0xae], 0
1405a192e: mov byte ptr [rbx + 0xaf], 1
1405a1935: mov byte ptr [rbx + 0xb0], 0
1405a193c: mov byte ptr [rbx + 0xb1], 1
1405a1943: mov byte ptr [rbx + 0xb2], 1
1405a194a: mov rcx, rax
1405a194d: xor edx, edx
1405a194f: call 0x14012c910 ; bevy_app::app::App::insert_resource<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>
1405a1954: mov byte ptr [rbx + 0xaa], 1
1405a195b: mov byte ptr [rbx + 0xab], 1
1405a1962: mov byte ptr [rbx + 0xac], 1
1405a1969: mov byte ptr [rbx + 0xad], 1
1405a1970: mov byte ptr [rbx + 0xae], 0
1405a1977: mov byte ptr [rbx + 0xaf], 1
1405a197e: mov byte ptr [rbx + 0xb0], 0
1405a1985: mov byte ptr [rbx + 0xb1], 1
1405a198c: mov byte ptr [rbx + 0xb2], 1
1405a1993: mov rcx, rax
1405a1996: xor edx, edx
1405a1998: call 0x14011b520 ; bevy_app::app::App::insert_resource<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>
1405a199d: mov rsi, rax
1405a19a0: mov byte ptr [rbx + 0xaa], 1
1405a19a7: mov byte ptr [rbx + 0xab], 1
1405a19ae: mov byte ptr [rbx + 0xac], 1
1405a19b5: mov byte ptr [rbx + 0xad], 1
1405a19bc: mov byte ptr [rbx + 0xae], 0
1405a19c3: mov byte ptr [rbx + 0xaf], 1
1405a19ca: mov byte ptr [rbx + 0xb0], 0
1405a19d1: mov byte ptr [rbx + 0xb1], 1
1405a19d8: mov byte ptr [rbx + 0xb2], 1
1405a19df: call 0x1404b0ec0 ; std::io::util::impl$0::read
1405a19e4: mov r8d, edx
1405a19e7: mov byte ptr [rbx + 0xaa], 1
1405a19ee: mov byte ptr [rbx + 0xab], 1
1405a19f5: mov byte ptr [rbx + 0xac], 1
1405a19fc: mov byte ptr [rbx + 0xad], 1
1405a1a03: mov byte ptr [rbx + 0xae], 0
1405a1a0a: mov byte ptr [rbx + 0xaf], 1
1405a1a11: mov byte ptr [rbx + 0xb0], 0
1405a1a18: mov byte ptr [rbx + 0xb1], 1
1405a1a1f: mov byte ptr [rbx + 0xb2], 1
1405a1a26: and al, 1
1405a1a28: and r8b, 1
1405a1a2c: mov rcx, rsi
1405a1a2f: mov edx, eax
1405a1a31: call 0x14011ecf0 ; bevy_app::app::App::insert_resource<country_core::resources::gui::PhotomodeScreen>
1405a1a36: mov qword ptr [rbx + 0xb05], 0
1405a1a41: mov qword ptr [rbx + 0xb00], 0
1405a1a4c: mov byte ptr [rbx + 0xaa], 1
1405a1a53: mov byte ptr [rbx + 0xab], 1
1405a1a5a: mov byte ptr [rbx + 0xac], 1
1405a1a61: mov byte ptr [rbx + 0xad], 1
1405a1a68: mov byte ptr [rbx + 0xae], 0
1405a1a6f: mov byte ptr [rbx + 0xaf], 1
1405a1a76: mov byte ptr [rbx + 0xb0], 0
1405a1a7d: mov byte ptr [rbx + 0xb1], 1
1405a1a84: mov byte ptr [rbx + 0xb2], 1
1405a1a8b: lea rdx, [rbx + 0xb00]
1405a1a92: mov rcx, rax
1405a1a95: call 0x14012e960 ; bevy_app::app::App::insert_resource<country_core::resources::gui::TimeOfDayScreen>
1405a1a9a: mov rsi, rax
1405a1a9d: call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1405a1aa2: mov rdi, rax
1405a1aa5: test byte ptr [rax], 1
1405a1aa8: je 0x1405a1abb
1405a1aaa: mov rcx, rdi
1405a1aad: add rcx, 8
1405a1ab1: mov rax, qword ptr [rdi + 8]
1405a1ab5: mov rdx, qword ptr [rdi + 0x10]
1405a1ab9: jmp 0x1405a1b15
1405a1abb: mov byte ptr [rbx + 0xaa], 1
1405a1ac2: mov byte ptr [rbx + 0xab], 1
1405a1ac9: mov byte ptr [rbx + 0xac], 1
1405a1ad0: mov byte ptr [rbx + 0xad], 1
1405a1ad7: mov byte ptr [rbx + 0xae], 0
1405a1ade: mov byte ptr [rbx + 0xaf], 1
1405a1ae5: mov byte ptr [rbx + 0xb0], 0
1405a1aec: mov byte ptr [rbx + 0xb1], 1
1405a1af3: mov byte ptr [rbx + 0xb2], 1
1405a1afa: call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1405a1aff: mov rcx, rdi
1405a1b02: add rcx, 8
1405a1b06: mov qword ptr [rdi], 1
1405a1b0d: mov qword ptr [rdi + 8], rax
1405a1b11: mov qword ptr [rdi + 0x10], rdx
1405a1b15: lea r8, [rax + 1]
1405a1b19: mov qword ptr [rcx], r8
1405a1b1c: movaps xmmword ptr [rbx + 0xb10], xmm6
1405a1b23: movaps xmmword ptr [rbx + 0xb00], xmm7
1405a1b2a: mov qword ptr [rbx + 0xb20], rax
1405a1b31: mov qword ptr [rbx + 0xb28], rdx
1405a1b38: mov byte ptr [rbx + 0xaa], 1
1405a1b3f: mov byte ptr [rbx + 0xab], 1
1405a1b46: mov byte ptr [rbx + 0xac], 1
1405a1b4d: mov byte ptr [rbx + 0xad], 1
1405a1b54: mov byte ptr [rbx + 0xae], 0
1405a1b5b: mov byte ptr [rbx + 0xaf], 1
1405a1b62: mov byte ptr [rbx + 0xb0], 0
1405a1b69: mov byte ptr [rbx + 0xb1], 1
1405a1b70: mov byte ptr [rbx + 0xb2], 1
1405a1b77: lea rdx, [rbx + 0xb00]
1405a1b7e: mov rcx, rsi
1405a1b81: call 0x1401272f0 ; bevy_app::app::App::insert_resource<country_core::resources::walls::arches::ArchSegments>
1405a1b86: mov rsi, rax
1405a1b89: call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1405a1b8e: mov rdi, rax
1405a1b91: test byte ptr [rax], 1
1405a1b94: je 0x1405a1ba7
1405a1b96: mov rcx, rdi
1405a1b99: add rcx, 8
1405a1b9d: mov rax, qword ptr [rdi + 8]
1405a1ba1: mov rdx, qword ptr [rdi + 0x10]
1405a1ba5: jmp 0x1405a1c01
1405a1ba7: mov byte ptr [rbx + 0xaa], 1
1405a1bae: mov byte ptr [rbx + 0xab], 1
1405a1bb5: mov byte ptr [rbx + 0xac], 1
1405a1bbc: mov byte ptr [rbx + 0xad], 1
1405a1bc3: mov byte ptr [rbx + 0xae], 0
1405a1bca: mov byte ptr [rbx + 0xaf], 1
1405a1bd1: mov byte ptr [rbx + 0xb0], 0
1405a1bd8: mov byte ptr [rbx + 0xb1], 1
1405a1bdf: mov byte ptr [rbx + 0xb2], 1
1405a1be6: call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1405a1beb: mov rcx, rdi
1405a1bee: add rcx, 8
1405a1bf2: mov qword ptr [rdi], 1
1405a1bf9: mov qword ptr [rdi + 8], rax
1405a1bfd: mov qword ptr [rdi + 0x10], rdx
1405a1c01: lea r8, [rax + 1]
1405a1c05: mov qword ptr [rcx], r8
1405a1c08: movaps xmmword ptr [rbx + 0xb10], xmm6
1405a1c0f: movaps xmmword ptr [rbx + 0xb00], xmm7
1405a1c16: mov qword ptr [rbx + 0xb20], rax
1405a1c1d: mov qword ptr [rbx + 0xb28], rdx
1405a1c24: mov byte ptr [rbx + 0xaa], 1
1405a1c2b: mov byte ptr [rbx + 0xab], 1
1405a1c32: mov byte ptr [rbx + 0xac], 1
1405a1c39: mov byte ptr [rbx + 0xad], 1
1405a1c40: mov byte ptr [rbx + 0xae], 0
1405a1c47: mov byte ptr [rbx + 0xaf], 1
1405a1c4e: mov byte ptr [rbx + 0xb0], 0
1405a1c55: mov byte ptr [rbx + 0xb1], 1
1405a1c5c: mov byte ptr [rbx + 0xb2], 1
1405a1c63: lea rdx, [rbx + 0xb00]
1405a1c6a: mov rcx, rsi
1405a1c6d: call 0x14012acb0 ; bevy_app::app::App::insert_resource<country_core::resources::walls::arches::BuildingHadArchesLastFrame>
1405a1c72: mov rsi, rax
1405a1c75: call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1405a1c7a: mov rdi, rax
1405a1c7d: test byte ptr [rax], 1
1405a1c80: je 0x1405a1c93
1405a1c82: mov rcx, rdi
1405a1c85: add rcx, 8
1405a1c89: mov rax, qword ptr [rdi + 8]
1405a1c8d: mov rdx, qword ptr [rdi + 0x10]
1405a1c91: jmp 0x1405a1ced
1405a1c93: mov byte ptr [rbx + 0xaa], 1
1405a1c9a: mov byte ptr [rbx + 0xab], 1
1405a1ca1: mov byte ptr [rbx + 0xac], 1
1405a1ca8: mov byte ptr [rbx + 0xad], 1
1405a1caf: mov byte ptr [rbx + 0xae], 0
1405a1cb6: mov byte ptr [rbx + 0xaf], 1
1405a1cbd: mov byte ptr [rbx + 0xb0], 0
1405a1cc4: mov byte ptr [rbx + 0xb1], 1
1405a1ccb: mov byte ptr [rbx + 0xb2], 1
1405a1cd2: call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1405a1cd7: mov rcx, rdi
1405a1cda: add rcx, 8
1405a1cde: mov qword ptr [rdi], 1
1405a1ce5: mov qword ptr [rdi + 8], rax
1405a1ce9: mov qword ptr [rdi + 0x10], rdx
1405a1ced: lea r8, [rax + 1]
1405a1cf1: mov qword ptr [rcx], r8
1405a1cf4: movaps xmmword ptr [rbx + 0xb10], xmm6
1405a1cfb: movaps xmmword ptr [rbx + 0xb00], xmm7
1405a1d02: mov qword ptr [rbx + 0xb20], rax
1405a1d09: mov qword ptr [rbx + 0xb28], rdx
1405a1d10: mov byte ptr [rbx + 0xaa], 1
1405a1d17: mov byte ptr [rbx + 0xab], 1
1405a1d1e: mov byte ptr [rbx + 0xac], 1
1405a1d25: mov byte ptr [rbx + 0xad], 1
1405a1d2c: mov byte ptr [rbx + 0xae], 0
1405a1d33: mov byte ptr [rbx + 0xaf], 1
1405a1d3a: mov byte ptr [rbx + 0xb0], 0
1405a1d41: mov byte ptr [rbx + 0xb1], 1
1405a1d48: mov byte ptr [rbx + 0xb2], 1
1405a1d4f: lea rdx, [rbx + 0xb00]
1405a1d56: mov rcx, rsi
1405a1d59: call 0x14012f490 ; bevy_app::app::App::insert_resource<country_core::resources::walls::arches::LintelSegments>
1405a1d5e: mov rsi, rax
1405a1d61: call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1405a1d66: mov rdi, rax
1405a1d69: test byte ptr [rax], 1
1405a1d6c: je 0x1405a1d7f
1405a1d6e: mov rcx, rdi
1405a1d71: add rcx, 8
1405a1d75: mov rax, qword ptr [rdi + 8]
1405a1d79: mov rdx, qword ptr [rdi + 0x10]
1405a1d7d: jmp 0x1405a1dd9
1405a1d7f: mov byte ptr [rbx + 0xaa], 1
1405a1d86: mov byte ptr [rbx + 0xab], 1
1405a1d8d: mov byte ptr [rbx + 0xac], 1
1405a1d94: mov byte ptr [rbx + 0xad], 1
1405a1d9b: mov byte ptr [rbx + 0xae], 0
1405a1da2: mov byte ptr [rbx + 0xaf], 1
1405a1da9: mov byte ptr [rbx + 0xb0], 0
1405a1db0: mov byte ptr [rbx + 0xb1], 1
1405a1db7: mov byte ptr [rbx + 0xb2], 1
1405a1dbe: call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1405a1dc3: mov rcx, rdi
1405a1dc6: add rcx, 8
1405a1dca: mov qword ptr [rdi], 1
1405a1dd1: mov qword ptr [rdi + 8], rax
1405a1dd5: mov qword ptr [rdi + 0x10], rdx
1405a1dd9: lea r8, [rax + 1]
1405a1ddd: mov qword ptr [rcx], r8
1405a1de0: movaps xmmword ptr [rbx + 0xb10], xmm6
1405a1de7: movaps xmmword ptr [rbx + 0xb00], xmm7
1405a1dee: mov qword ptr [rbx + 0xb20], rax
1405a1df5: mov qword ptr [rbx + 0xb28], rdx
1405a1dfc: mov byte ptr [rbx + 0xaa], 1
1405a1e03: mov byte ptr [rbx + 0xab], 1
1405a1e0a: mov byte ptr [rbx + 0xac], 1
1405a1e11: mov byte ptr [rbx + 0xad], 1
1405a1e18: mov byte ptr [rbx + 0xae], 0
1405a1e1f: mov byte ptr [rbx + 0xaf], 1
1405a1e26: mov byte ptr [rbx + 0xb0], 0
1405a1e2d: mov byte ptr [rbx + 0xb1], 1
1405a1e34: mov byte ptr [rbx + 0xb2], 1
1405a1e3b: lea rdx, [rbx + 0xb00]
1405a1e42: mov rcx, rsi
1405a1e45: call 0x14012bdc0 ; bevy_app::app::App::insert_resource<country_core::resources::walls::elevation_support_stats::ElevationSupportStats>
1405a1e4a: mov rsi, rax
1405a1e4d: call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1405a1e52: mov rdi, rax
1405a1e55: test byte ptr [rax], 1
1405a1e58: je 0x1405a1e6b
1405a1e5a: mov rcx, rdi
1405a1e5d: add rcx, 8
1405a1e61: mov rax, qword ptr [rdi + 8]
1405a1e65: mov rdx, qword ptr [rdi + 0x10]
1405a1e69: jmp 0x1405a1ec5
1405a1e6b: mov byte ptr [rbx + 0xaa], 1
1405a1e72: mov byte ptr [rbx + 0xab], 1
1405a1e79: mov byte ptr [rbx + 0xac], 1
1405a1e80: mov byte ptr [rbx + 0xad], 1
1405a1e87: mov byte ptr [rbx + 0xae], 0
1405a1e8e: mov byte ptr [rbx + 0xaf], 1
1405a1e95: mov byte ptr [rbx + 0xb0], 0
1405a1e9c: mov byte ptr [rbx + 0xb1], 1
1405a1ea3: mov byte ptr [rbx + 0xb2], 1
1405a1eaa: call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1405a1eaf: mov rcx, rdi
1405a1eb2: add rcx, 8
1405a1eb6: mov qword ptr [rdi], 1
1405a1ebd: mov qword ptr [rdi + 8], rax
1405a1ec1: mov qword ptr [rdi + 0x10], rdx
1405a1ec5: lea r8, [rax + 1]
1405a1ec9: mov qword ptr [rcx], r8
1405a1ecc: movaps xmmword ptr [rbx + 0xb10], xmm6
1405a1ed3: movaps xmmword ptr [rbx + 0xb00], xmm7
1405a1eda: mov qword ptr [rbx + 0xb20], rax
1405a1ee1: mov qword ptr [rbx + 0xb28], rdx
1405a1ee8: mov byte ptr [rbx + 0xaa], 1
1405a1eef: mov byte ptr [rbx + 0xab], 1
1405a1ef6: mov byte ptr [rbx + 0xac], 1
1405a1efd: mov byte ptr [rbx + 0xad], 1
1405a1f04: mov byte ptr [rbx + 0xae], 0
1405a1f0b: mov byte ptr [rbx + 0xaf], 1
1405a1f12: mov byte ptr [rbx + 0xb0], 0
1405a1f19: mov byte ptr [rbx + 0xb1], 1
1405a1f20: mov byte ptr [rbx + 0xb2], 1
1405a1f27: lea rdx, [rbx + 0xb00]
1405a1f2e: mov rcx, rsi
1405a1f31: call 0x140125010 ; bevy_app::app::App::insert_resource<country_core::resources::walls::wooden_gates::Gates>
1405a1f36: mov rsi, rax
1405a1f39: call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1405a1f3e: mov rdi, rax
1405a1f41: test byte ptr [rax], 1
1405a1f44: je 0x1405a1f57
1405a1f46: mov rcx, rdi
1405a1f49: add rcx, 8
1405a1f4d: mov rax, qword ptr [rdi + 8]
1405a1f51: mov rdx, qword ptr [rdi + 0x10]
1405a1f55: jmp 0x1405a1fb1
1405a1f57: mov byte ptr [rbx + 0xaa], 1
1405a1f5e: mov byte ptr [rbx + 0xab], 1
1405a1f65: mov byte ptr [rbx + 0xac], 1
1405a1f6c: mov byte ptr [rbx + 0xad], 1
1405a1f73: mov byte ptr [rbx + 0xae], 0
1405a1f7a: mov byte ptr [rbx + 0xaf], 1
1405a1f81: mov byte ptr [rbx + 0xb0], 0
1405a1f88: mov byte ptr [rbx + 0xb1], 1
1405a1f8f: mov byte ptr [rbx + 0xb2], 1
1405a1f96: call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1405a1f9b: mov rcx, rdi
1405a1f9e: add rcx, 8
1405a1fa2: mov qword ptr [rdi], 1
1405a1fa9: mov qword ptr [rdi + 8], rax
1405a1fad: mov qword ptr [rdi + 0x10], rdx
1405a1fb1: lea r8, [rax + 1]
1405a1fb5: mov qword ptr [rcx], r8
1405a1fb8: movaps xmmword ptr [rbx + 0x450], xmm6
1405a1fbf: movaps xmmword ptr [rbx + 0x440], xmm7
1405a1fc6: mov qword ptr [rbx + 0x460], rax
1405a1fcd: mov qword ptr [rbx + 0x468], rdx
1405a1fd4: call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1405a1fd9: mov rdi, rax
1405a1fdc: test byte ptr [rax], 1
1405a1fdf: je 0x1405a1ff2
1405a1fe1: mov rcx, rdi
1405a1fe4: add rcx, 8
1405a1fe8: mov rax, qword ptr [rdi + 8]
1405a1fec: mov rdx, qword ptr [rdi + 0x10]
1405a1ff0: jmp 0x1405a200d
1405a1ff2: call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1405a1ff7: mov rcx, rdi
1405a1ffa: add rcx, 8
1405a1ffe: mov qword ptr [rdi], 1
1405a2005: mov qword ptr [rdi + 8], rax
1405a2009: mov qword ptr [rdi + 0x10], rdx
1405a200d: lea r8, [rax + 1]
1405a2011: mov qword ptr [rcx], r8
1405a2014: movaps xmm0, xmmword ptr [rbx + 0x440]
1405a201b: movaps xmm1, xmmword ptr [rbx + 0x450]
1405a2022: movaps xmm2, xmmword ptr [rbx + 0x460]
1405a2029: movaps xmmword ptr [rbx + 0xb20], xmm2
1405a2030: movaps xmmword ptr [rbx + 0xb10], xmm1
1405a2037: movaps xmmword ptr [rbx + 0xb00], xmm0
1405a203e: movaps xmmword ptr [rbx + 0xb30], xmm7
1405a2045: movaps xmmword ptr [rbx + 0xb40], xmm6
1405a204c: mov qword ptr [rbx + 0xb50], rax
1405a2053: mov qword ptr [rbx + 0xb58], rdx
1405a205a: mov byte ptr [rbx + 0xaa], 1
1405a2061: mov byte ptr [rbx + 0xab], 1
1405a2068: mov byte ptr [rbx + 0xac], 1
1405a206f: mov byte ptr [rbx + 0xad], 1
1405a2076: mov byte ptr [rbx + 0xae], 0
1405a207d: mov byte ptr [rbx + 0xaf], 1
1405a2084: mov byte ptr [rbx + 0xb0], 0
1405a208b: mov byte ptr [rbx + 0xb1], 1
1405a2092: mov byte ptr [rbx + 0xb2], 1
1405a2099: lea rdx, [rbx + 0xb00]
1405a20a0: mov rcx, rsi
1405a20a3: call 0x14011fe50 ; bevy_app::app::App::insert_resource<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>
1405a20a8: mov byte ptr [rbx + 0xaa], 1
1405a20af: mov byte ptr [rbx + 0xab], 1
1405a20b6: mov byte ptr [rbx + 0xac], 1
1405a20bd: mov byte ptr [rbx + 0xad], 1
1405a20c4: mov byte ptr [rbx + 0xae], 0
1405a20cb: mov byte ptr [rbx + 0xaf], 1
1405a20d2: mov byte ptr [rbx + 0xb0], 0
1405a20d9: mov byte ptr [rbx + 0xb1], 1
1405a20e0: mov byte ptr [rbx + 0xb2], 1
1405a20e7: mov rcx, rax
1405a20ea: call 0x140651870 ; system_color::impl$0::add_color
1405a20ef: mov byte ptr [rbx + 0xaa], 1
1405a20f6: mov byte ptr [rbx + 0xab], 1
1405a20fd: mov byte ptr [rbx + 0xac], 1
1405a2104: mov byte ptr [rbx + 0xad], 1
1405a210b: mov byte ptr [rbx + 0xae], 0
1405a2112: mov byte ptr [rbx + 0xaf], 1
1405a2119: mov byte ptr [rbx + 0xb0], 0
1405a2120: mov byte ptr [rbx + 0xb1], 1
1405a2127: mov byte ptr [rbx + 0xb2], 1
1405a212e: mov rcx, rax
1405a2131: call 0x140630a20 ; timberframe::impl$0::add_timberframe
1405a2136: mov byte ptr [rbx + 0xaa], 1
1405a213d: mov byte ptr [rbx + 0xab], 1
1405a2144: mov byte ptr [rbx + 0xac], 1
1405a214b: mov byte ptr [rbx + 0xad], 1
1405a2152: mov byte ptr [rbx + 0xae], 0
1405a2159: mov byte ptr [rbx + 0xaf], 1
1405a2160: mov byte ptr [rbx + 0xb0], 0
1405a2167: mov byte ptr [rbx + 0xb1], 1
1405a216e: mov byte ptr [rbx + 0xb2], 1
1405a2175: mov rcx, rax
1405a2178: call 0x140650d50 ; system_onboarding::impl$0::add_onboarding
1405a217d: mov edx, dword ptr [rbx + 0x148]
1405a2183: mov r8d, dword ptr [rbx + 0x14c]
1405a218a: mov byte ptr [rbx + 0xaa], 1
1405a2191: mov byte ptr [rbx + 0xab], 1
1405a2198: mov byte ptr [rbx + 0xac], 1
1405a219f: mov byte ptr [rbx + 0xad], 1
1405a21a6: mov byte ptr [rbx + 0xae], 0
1405a21ad: mov byte ptr [rbx + 0xaf], 1
1405a21b4: mov byte ptr [rbx + 0xb0], 0
1405a21bb: mov byte ptr [rbx + 0xb1], 1
1405a21c2: mov byte ptr [rbx + 0xb2], 1
1405a21c9: mov rcx, rax
1405a21cc: call 0x140940d80 ; country_core::resources::impl$1::insert_resources
1405a21d1: mov rsi, rax
1405a21d4: mov byte ptr [rbx + 0xaa], 1
1405a21db: mov byte ptr [rbx + 0xab], 1
1405a21e2: mov byte ptr [rbx + 0xac], 1
1405a21e9: mov byte ptr [rbx + 0xad], 1
1405a21f0: mov byte ptr [rbx + 0xae], 0
1405a21f7: mov byte ptr [rbx + 0xaf], 1
1405a21fe: mov byte ptr [rbx + 0xb0], 0
1405a2205: mov byte ptr [rbx + 0xb1], 1
1405a220c: mov byte ptr [rbx + 0xb2], 1
1405a2213: lea rcx, [rbx + 0xb00]
1405a221a: mov rdx, qword ptr [rbp + 0x37b0]
1405a2221: call 0x140593510 ; country_core::game_config::impl$11::clone
1405a2226: mov byte ptr [rbx + 0xaa], 1
1405a222d: mov byte ptr [rbx + 0xab], 1
1405a2234: mov byte ptr [rbx + 0xac], 1
1405a223b: mov byte ptr [rbx + 0xad], 1
1405a2242: mov byte ptr [rbx + 0xae], 0
1405a2249: mov byte ptr [rbx + 0xaf], 1
1405a2250: mov byte ptr [rbx + 0xb0], 0
1405a2257: mov byte ptr [rbx + 0xb1], 1
1405a225e: mov byte ptr [rbx + 0xb2], 1
1405a2265: lea rdx, [rbx + 0xb00]
1405a226c: mov rcx, rsi
1405a226f: call 0x140119810 ; bevy_app::app::App::insert_resource<country_core::game_config::GameConfig>
1405a2274: mov byte ptr [rbx + 0xaa], 1
1405a227b: mov byte ptr [rbx + 0xab], 1
1405a2282: mov byte ptr [rbx + 0xac], 1
1405a2289: mov byte ptr [rbx + 0xad], 1
1405a2290: mov byte ptr [rbx + 0xae], 0
1405a2297: mov byte ptr [rbx + 0xaf], 1
1405a229e: mov byte ptr [rbx + 0xb0], 0
1405a22a5: mov byte ptr [rbx + 0xb1], 1
1405a22ac: mov byte ptr [rbx + 0xb2], 1
1405a22b3: movabs rdx, 0xe3601bf355496a1b
1405a22bd: movabs r8, 0x8acc546ccef23be3
1405a22c7: mov rcx, qword ptr [rbx + 0xe8]
1405a22ce: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a22d3: test rax, rax
1405a22d6: mov r8, qword ptr [rbx + 0xc8]
1405a22dd: je 0x1405a249d
1405a22e3: cmp qword ptr [r8 + 0x218], rdx
1405a22ea: jbe 0x1405a249d
1405a22f0: mov rax, qword ptr [r8 + 0x210]
1405a22f7: mov rcx, qword ptr [rax + rdx*8]
1405a22fb: test rcx, rcx
1405a22fe: je 0x1405a249d
1405a2304: mov rax, qword ptr [r8 + 0x1e0]
1405a230b: not rcx
1405a230e: lea rcx, [rcx + rcx*2]
1405a2312: shl rcx, 5
1405a2316: cmp qword ptr [rax + rcx + 0x20], 0
1405a231c: je 0x1405a249d
1405a2322: add rax, rcx
1405a2325: mov rcx, qword ptr [rax + 0x10]
1405a2329: mov byte ptr [rbx + 0xaa], 1
1405a2330: mov byte ptr [rbx + 0xab], 1
1405a2337: mov byte ptr [rbx + 0xac], 1
1405a233e: mov byte ptr [rbx + 0xad], 1
1405a2345: mov byte ptr [rbx + 0xae], 0
1405a234c: mov byte ptr [rbx + 0xaf], 1
1405a2353: mov byte ptr [rbx + 0xb0], 0
1405a235a: mov byte ptr [rbx + 0xb1], 1
1405a2361: mov byte ptr [rbx + 0xb2], 1
1405a2368: call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
1405a236d: mov rsi, rax
1405a2370: call 0x142923570 ; 
1405a2375: mov rcx, qword ptr [rsi]
1405a2378: lock inc qword ptr [rcx]
1405a237c: jle 0x1405a7040
1405a2382: mov qword ptr [rbx + 0xb00], rcx
1405a2389: mov qword ptr [rbx + 0xb08], rax
1405a2390: mov rcx, rax
1405a2393: call 0x1429234f0 ; 
1405a2398: mov esi, eax
1405a239a: mov rax, qword ptr [rbx + 0xb00]
1405a23a1: lock dec qword ptr [rax]
1405a23a5: jne 0x1405a23f2
1405a23a7: mov byte ptr [rbx + 0xaa], 1
1405a23ae: mov byte ptr [rbx + 0xab], 1
1405a23b5: mov byte ptr [rbx + 0xac], 1
1405a23bc: mov byte ptr [rbx + 0xad], 1
1405a23c3: mov byte ptr [rbx + 0xae], 0
1405a23ca: mov byte ptr [rbx + 0xaf], 1
1405a23d1: mov byte ptr [rbx + 0xb0], 0
1405a23d8: mov byte ptr [rbx + 0xb1], 1
1405a23df: mov byte ptr [rbx + 0xb2], 1
1405a23e6: lea rcx, [rbx + 0xb00]
1405a23ed: call 0x140b85350 ; alloc::sync::Arc::drop_slow<steamworks::Inner<steamworks::ClientManager>,alloc::alloc::Global>
1405a23f2: test sil, sil
1405a23f5: je 0x1405a249d
1405a23fb: mov byte ptr [rbx + 0xaa], 1
1405a2402: mov byte ptr [rbx + 0xab], 1
1405a2409: mov byte ptr [rbx + 0xac], 1
1405a2410: mov byte ptr [rbx + 0xad], 1
1405a2417: mov byte ptr [rbx + 0xae], 0
1405a241e: mov byte ptr [rbx + 0xaf], 1
1405a2425: mov byte ptr [rbx + 0xb0], 0
1405a242c: mov byte ptr [rbx + 0xb1], 1
1405a2433: mov byte ptr [rbx + 0xb2], 1
1405a243a: lea rcx, [rbx + 0xb00]
1405a2441: call 0x140958eb0 ; country_core::input::ActiveInputModeTracker::new_default_controller
1405a2446: mov byte ptr [rbx + 0xaa], 1
1405a244d: mov byte ptr [rbx + 0xab], 1
1405a2454: mov byte ptr [rbx + 0xac], 1
1405a245b: mov byte ptr [rbx + 0xad], 1
1405a2462: mov byte ptr [rbx + 0xae], 0
1405a2469: mov byte ptr [rbx + 0xaf], 1
1405a2470: mov byte ptr [rbx + 0xb0], 0
1405a2477: mov byte ptr [rbx + 0xb1], 1
1405a247e: mov byte ptr [rbx + 0xb2], 1
1405a2485: lea rdx, [rbx + 0xb00]
1405a248c: mov rcx, qword ptr [rbx + 0xc8]
1405a2493: call 0x14012b880 ; bevy_app::app::App::insert_resource<country_core::input::ActiveInputModeTracker>
1405a2498: jmp 0x1405a253a
1405a249d: mov byte ptr [rbx + 0xaa], 1
1405a24a4: mov byte ptr [rbx + 0xab], 1
1405a24ab: mov byte ptr [rbx + 0xac], 1
1405a24b2: mov byte ptr [rbx + 0xad], 1
1405a24b9: mov byte ptr [rbx + 0xae], 0
1405a24c0: mov byte ptr [rbx + 0xaf], 1
1405a24c7: mov byte ptr [rbx + 0xb0], 0
1405a24ce: mov byte ptr [rbx + 0xb1], 1
1405a24d5: mov byte ptr [rbx + 0xb2], 1
1405a24dc: lea rcx, [rbx + 0xb00]
1405a24e3: call 0x140958ef0 ; country_core::input::ActiveInputModeTracker::new_default_mouse_keyboard
1405a24e8: mov byte ptr [rbx + 0xaa], 1
1405a24ef: mov byte ptr [rbx + 0xab], 1
1405a24f6: mov byte ptr [rbx + 0xac], 1
1405a24fd: mov byte ptr [rbx + 0xad], 1
1405a2504: mov byte ptr [rbx + 0xae], 0
1405a250b: mov byte ptr [rbx + 0xaf], 1
1405a2512: mov byte ptr [rbx + 0xb0], 0
1405a2519: mov byte ptr [rbx + 0xb1], 1
1405a2520: mov byte ptr [rbx + 0xb2], 1
1405a2527: lea rdx, [rbx + 0xb00]
1405a252e: mov rcx, qword ptr [rbx + 0xc8]
1405a2535: call 0x14012b880 ; bevy_app::app::App::insert_resource<country_core::input::ActiveInputModeTracker>
1405a253a: mov byte ptr [rbx + 0xaa], 1
1405a2541: mov byte ptr [rbx + 0xab], 1
1405a2548: mov byte ptr [rbx + 0xac], 1
1405a254f: mov byte ptr [rbx + 0xad], 1
1405a2556: mov byte ptr [rbx + 0xae], 0
1405a255d: mov byte ptr [rbx + 0xaf], 1
1405a2564: mov byte ptr [rbx + 0xb0], 0
1405a256b: mov byte ptr [rbx + 0xb1], 1
1405a2572: mov byte ptr [rbx + 0xb2], 1
1405a2579: lea rcx, [rbx + 0x148]
1405a2580: call 0x1400885d0 ; tracing_core::callsite::impl$1::metadata
1405a2585: mov rsi, rax
1405a2588: mov byte ptr [rbx + 0xaa], 1
1405a258f: mov byte ptr [rbx + 0xab], 1
1405a2596: mov byte ptr [rbx + 0xac], 1
1405a259d: mov byte ptr [rbx + 0xad], 1
1405a25a4: mov byte ptr [rbx + 0xae], 0
1405a25ab: mov byte ptr [rbx + 0xaf], 1
1405a25b2: mov byte ptr [rbx + 0xb0], 0
1405a25b9: mov byte ptr [rbx + 0xb1], 1
1405a25c0: mov byte ptr [rbx + 0xb2], 1
1405a25c7: lea rcx, [rbx + 0x148]
1405a25ce: call 0x1400885d0 ; tracing_core::callsite::impl$1::metadata
1405a25d3: mov rdi, rax
1405a25d6: mov byte ptr [rbx + 0xaa], 1
1405a25dd: mov byte ptr [rbx + 0xab], 1
1405a25e4: mov byte ptr [rbx + 0xac], 1
1405a25eb: mov byte ptr [rbx + 0xad], 1
1405a25f2: mov byte ptr [rbx + 0xae], 0
1405a25f9: mov byte ptr [rbx + 0xaf], 1
1405a2600: mov byte ptr [rbx + 0xb0], 0
1405a2607: mov byte ptr [rbx + 0xb1], 1
1405a260e: mov byte ptr [rbx + 0xb2], 1
1405a2615: lea rcx, [rbx + 0x148]
1405a261c: call 0x1400885d0 ; tracing_core::callsite::impl$1::metadata
1405a2621: mov r15, rax
1405a2624: mov rax, qword ptr [rbp + 0x37b0]
1405a262b: cmp byte ptr [rax + 0x88], 0
1405a2632: movss xmm0, dword ptr [rip + 0x23832ca]
1405a263a: jne 0x1405a264b
1405a263c: movaps xmm12, xmm0
1405a2640: cmp byte ptr [rax + 0xab], 1
1405a2647: je 0x1405a265d
1405a2649: jmp 0x1405a26a8
1405a264b: movss xmm12, dword ptr [rax + 0x8c]
1405a2654: cmp byte ptr [rax + 0xab], 1
1405a265b: jne 0x1405a26a8
1405a265d: lea rcx, [rax + 0xc3]
1405a2664: mov byte ptr [rbx + 0xaa], 1
1405a266b: mov byte ptr [rbx + 0xab], 1
1405a2672: mov byte ptr [rbx + 0xac], 1
1405a2679: mov byte ptr [rbx + 0xad], 1
1405a2680: mov byte ptr [rbx + 0xae], 0
1405a2687: mov byte ptr [rbx + 0xaf], 1
1405a268e: mov byte ptr [rbx + 0xb0], 0
1405a2695: mov byte ptr [rbx + 0xb1], 1
1405a269c: mov byte ptr [rbx + 0xb2], 1
1405a26a3: call 0x140968bb0 ; country_core::render::render_config::UpscalingPreset::tgsr_resolution_scale
1405a26a8: mov qword ptr [rbx + 0xb00], rsi
1405a26af: mov qword ptr [rbx + 0xb08], rdi
1405a26b6: mov qword ptr [rbx + 0xb10], r15
1405a26bd: mov byte ptr [rbx + 0xb20], 0
1405a26c4: movss dword ptr [rbx + 0xb18], xmm12
1405a26cd: movss dword ptr [rbx + 0xb1c], xmm0
1405a26d5: mov byte ptr [rbx + 0xaa], 1
1405a26dc: mov byte ptr [rbx + 0xab], 1
1405a26e3: mov byte ptr [rbx + 0xac], 1
1405a26ea: mov byte ptr [rbx + 0xad], 1
1405a26f1: mov byte ptr [rbx + 0xae], 0
1405a26f8: mov byte ptr [rbx + 0xaf], 1
1405a26ff: mov byte ptr [rbx + 0xb0], 0
1405a2706: mov byte ptr [rbx + 0xb1], 1
1405a270d: mov byte ptr [rbx + 0xb2], 1
1405a2714: lea rdx, [rbx + 0xb00]
1405a271b: mov rcx, qword ptr [rbx + 0xc8]
1405a2722: call 0x14012e420 ; bevy_app::app::App::insert_resource<country_core::render::render_config::FrameRenderConfig>
1405a2727: mov rax, qword ptr [rbp + 0x37c0]
1405a272e: movzx eax, byte ptr [rax + 0x141]
1405a2735: xor al, 1
1405a2737: mov byte ptr [rbx + 0xb28], al
1405a273d: mov qword ptr [rbx + 0xb00], 0
1405a2748: mov byte ptr [rbx + 0xaa], 1
1405a274f: mov byte ptr [rbx + 0xab], 1
1405a2756: mov byte ptr [rbx + 0xac], 1
1405a275d: mov byte ptr [rbx + 0xad], 1
1405a2764: mov byte ptr [rbx + 0xae], 0
1405a276b: mov byte ptr [rbx + 0xaf], 1
1405a2772: mov byte ptr [rbx + 0xb0], 0
1405a2779: mov byte ptr [rbx + 0xb1], 1
1405a2780: mov byte ptr [rbx + 0xb2], 1
1405a2787: lea rdx, [rbx + 0xb00]
1405a278e: mov rcx, qword ptr [rbx + 0xc8]
1405a2795: call 0x14013d5f0 ; bevy_app::app::App::insert_resource<country_core::render::adaptive_render_state::AdaptiveRenderState>
1405a279a: mov rax, qword ptr [rbx + 0x2c0]
1405a27a1: mov rdx, qword ptr [rbx + 0x2c8]
1405a27a8: test rax, rax
1405a27ab: je 0x1405a280c
1405a27ad: cmp eax, 1
1405a27b0: jne 0x1405a285f
1405a27b6: mov byte ptr [rbx + 0xaa], 1
1405a27bd: mov byte ptr [rbx + 0xab], 1
1405a27c4: mov byte ptr [rbx + 0xac], 1
1405a27cb: mov byte ptr [rbx + 0xad], 1
1405a27d2: mov byte ptr [rbx + 0xae], 0
1405a27d9: mov byte ptr [rbx + 0xaf], 1
1405a27e0: mov byte ptr [rbx + 0xb0], 0
1405a27e7: mov byte ptr [rbx + 0xb1], 1
1405a27ee: mov byte ptr [rbx + 0xb2], 1
1405a27f5: lea rcx, [rbx + 0xb00]
1405a27fc: mov r9d, 0x3b9aca00
1405a2802: call 0x140264130 ; std::sync::mpmc::list::Channel::recv<country_core::systems::audio::AsyncLoadedAudioResources>
1405a2807: jmp 0x1405a28b0
1405a280c: mov byte ptr [rbx + 0xaa], 1
1405a2813: mov byte ptr [rbx + 0xab], 1
1405a281a: mov byte ptr [rbx + 0xac], 1
1405a2821: mov byte ptr [rbx + 0xad], 1
1405a2828: mov byte ptr [rbx + 0xae], 0
1405a282f: mov byte ptr [rbx + 0xaf], 1
1405a2836: mov byte ptr [rbx + 0xb0], 0
1405a283d: mov byte ptr [rbx + 0xb1], 1
1405a2844: mov byte ptr [rbx + 0xb2], 1
1405a284b: lea rcx, [rbx + 0xb00]
1405a2852: mov r9d, 0x3b9aca00
1405a2858: call 0x1404ee940 ; std::sync::mpmc::array::Channel::recv<country_core::systems::audio::AsyncLoadedAudioResources>
1405a285d: jmp 0x1405a28b0
1405a285f: mov byte ptr [rbx + 0xaa], 1
1405a2866: mov byte ptr [rbx + 0xab], 1
1405a286d: mov byte ptr [rbx + 0xac], 1
1405a2874: mov byte ptr [rbx + 0xad], 1
1405a287b: mov byte ptr [rbx + 0xae], 0
1405a2882: mov byte ptr [rbx + 0xaf], 1
1405a2889: mov byte ptr [rbx + 0xb0], 0
1405a2890: mov byte ptr [rbx + 0xb1], 1
1405a2897: mov byte ptr [rbx + 0xb2], 1
1405a289e: lea rcx, [rbx + 0xb00]
1405a28a5: mov r9d, 0x3b9aca00
1405a28ab: call 0x140513570 ; std::sync::mpmc::zero::Channel::recv<country_core::systems::audio::AsyncLoadedAudioResources>
1405a28b0: mov rax, qword ptr [rbx + 0xb00]
1405a28b7: cmp rax, 2
1405a28bb: je 0x1405a6d15
1405a28c1: movups xmm0, xmmword ptr [rbx + 0xb08]
1405a28c8: movups xmm1, xmmword ptr [rbx + 0xb18]
1405a28cf: movups xmmword ptr [rbx + 0x318], xmm1
1405a28d6: movups xmmword ptr [rbx + 0x308], xmm0
1405a28dd: mov qword ptr [rbx + 0x300], rax
1405a28e4: mov rdx, qword ptr [rbx + 0x320]
1405a28eb: test rdx, rdx
1405a28ee: mov rcx, qword ptr [rbx + 0xc8]
1405a28f5: je 0x1405a2901
1405a28f7: lock inc qword ptr [rdx]
1405a28fb: jle 0x1405a7040
1405a2901: mov byte ptr [rbx + 0xb4], 1
1405a2908: mov byte ptr [rbx + 0xb5], 1
1405a290f: mov byte ptr [rbx + 0xb6], 1
1405a2916: mov byte ptr [rbx + 0xb7], 1
1405a291d: mov byte ptr [rbx + 0xb8], 1
1405a2924: mov byte ptr [rbx + 0xb9], 1
1405a292b: mov byte ptr [rbx + 0xba], 1
1405a2932: mov byte ptr [rbx + 0xbb], 1
1405a2939: call 0x140114e10 ; bevy_app::app::App::insert_resource<country_core::resources::audio_resources::AudioCore>
1405a293e: movups xmm0, xmmword ptr [rbx + 0x300]
1405a2945: movups xmm1, xmmword ptr [rbx + 0x310]
1405a294c: movaps xmmword ptr [rbx + 0xb10], xmm1
1405a2953: movaps xmmword ptr [rbx + 0xb00], xmm0
1405a295a: mov byte ptr [rbx + 0xb4], 1
1405a2961: mov byte ptr [rbx + 0xb5], 1
1405a2968: mov byte ptr [rbx + 0xb6], 1
1405a296f: mov byte ptr [rbx + 0xb7], 0
1405a2976: mov byte ptr [rbx + 0xb8], 1
1405a297d: mov byte ptr [rbx + 0xb9], 1
1405a2984: mov byte ptr [rbx + 0xba], 1
1405a298b: mov byte ptr [rbx + 0xbb], 1
1405a2992: lea rdx, [rbx + 0xb00]
1405a2999: mov rcx, qword ptr [rbx + 0xc8]
1405a29a0: call 0x14012b2a0 ; bevy_app::app::App::insert_resource<country_core::resources::audio_resources::AudioSamplesLoadedPromise>
1405a29a5: mov byte ptr [rbx + 0xb4], 1
1405a29ac: mov byte ptr [rbx + 0xb5], 1
1405a29b3: mov byte ptr [rbx + 0xb6], 1
1405a29ba: mov byte ptr [rbx + 0xb7], 0
1405a29c1: mov byte ptr [rbx + 0xb8], 1
1405a29c8: mov byte ptr [rbx + 0xb9], 1
1405a29cf: mov byte ptr [rbx + 0xba], 1
1405a29d6: mov byte ptr [rbx + 0xbb], 1
1405a29dd: mov rcx, qword ptr [rbx + 0xc8]
1405a29e4: call 0x140327eb0 ; bevy_app::sub_app::SubApp::add_systems<tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >)> >,bevy_app::main_schedule::PostStartup,tiny_glade::run_game::run_game_thread::closure_env$2>
1405a29e9: mov byte ptr [rbx + 0xb4], 1
1405a29f0: mov byte ptr [rbx + 0xb5], 1
1405a29f7: mov byte ptr [rbx + 0xb6], 1
1405a29fe: mov byte ptr [rbx + 0xb7], 0
1405a2a05: mov byte ptr [rbx + 0xb8], 1
1405a2a0c: mov byte ptr [rbx + 0xb9], 1
1405a2a13: mov byte ptr [rbx + 0xba], 1
1405a2a1a: mov byte ptr [rbx + 0xbb], 1
1405a2a21: lea rdx, [rip + 0x245cee0]
1405a2a28: mov rcx, qword ptr [rbx + 0xc8]
1405a2a2f: call 0x140113340 ; bevy_app::app::App::add_plugins<bevy_app::plugin::sealed::PluginMarker,system_clutter::ClutterPlugin>
1405a2a34: mov byte ptr [rbx + 0xb4], 1
1405a2a3b: mov byte ptr [rbx + 0xb5], 1
1405a2a42: mov byte ptr [rbx + 0xb6], 1
1405a2a49: mov byte ptr [rbx + 0xb7], 0
1405a2a50: mov byte ptr [rbx + 0xb8], 1
1405a2a57: mov byte ptr [rbx + 0xb9], 1
1405a2a5e: mov byte ptr [rbx + 0xba], 1
1405a2a65: mov byte ptr [rbx + 0xbb], 1
1405a2a6c: lea rdx, [rip + 0x245cead]
1405a2a73: mov rcx, qword ptr [rbx + 0xc8]
1405a2a7a: call 0x1401130e0 ; bevy_app::app::App::add_plugins<bevy_app::plugin::sealed::PluginMarker,country_core::systems::collision::CollisionPlugin>
1405a2a7f: mov byte ptr [rbx + 0xb4], 1
1405a2a86: mov byte ptr [rbx + 0xb5], 1
1405a2a8d: mov byte ptr [rbx + 0xb6], 1
1405a2a94: mov byte ptr [rbx + 0xb7], 0
1405a2a9b: mov byte ptr [rbx + 0xb8], 1
1405a2aa2: mov byte ptr [rbx + 0xb9], 1
1405a2aa9: mov byte ptr [rbx + 0xba], 1
1405a2ab0: mov byte ptr [rbx + 0xbb], 1
1405a2ab7: lea rdx, [rip + 0x245ce7a]
1405a2abe: mov rcx, qword ptr [rbx + 0xc8]
1405a2ac5: call 0x140113210 ; bevy_app::app::App::add_plugins<bevy_app::plugin::sealed::PluginMarker,forest::ForestPlugin>
1405a2aca: mov byte ptr [rbx + 0xb4], 1
1405a2ad1: mov byte ptr [rbx + 0xb5], 1
1405a2ad8: mov byte ptr [rbx + 0xb6], 1
1405a2adf: mov byte ptr [rbx + 0xb7], 0
1405a2ae6: mov byte ptr [rbx + 0xb8], 1
1405a2aed: mov byte ptr [rbx + 0xb9], 1
1405a2af4: mov byte ptr [rbx + 0xba], 1
1405a2afb: mov byte ptr [rbx + 0xbb], 1
1405a2b02: lea rdx, [rip + 0x245ce47]
1405a2b09: mov rcx, qword ptr [rbx + 0xc8]
1405a2b10: call 0x140112fb0 ; bevy_app::app::App::add_plugins<bevy_app::plugin::sealed::PluginMarker,system_glade_border::GladeBorderPlugin>
1405a2b15: mov byte ptr [rbx + 0xb4], 1
1405a2b1c: mov byte ptr [rbx + 0xb5], 1
1405a2b23: mov byte ptr [rbx + 0xb6], 1
1405a2b2a: mov byte ptr [rbx + 0xb7], 0
1405a2b31: mov byte ptr [rbx + 0xb8], 1
1405a2b38: mov byte ptr [rbx + 0xb9], 1
1405a2b3f: mov byte ptr [rbx + 0xba], 1
1405a2b46: mov byte ptr [rbx + 0xbb], 1
1405a2b4d: mov rcx, r14
1405a2b50: call 0x14090df40 ; country_core::game_config::DevSettings::live_link
1405a2b55: mov sil, al
1405a2b58: mov byte ptr [rbx + 0xb4], 1
1405a2b5f: mov byte ptr [rbx + 0xb5], 1
1405a2b66: mov byte ptr [rbx + 0xb6], 1
1405a2b6d: mov byte ptr [rbx + 0xb7], 0
1405a2b74: mov byte ptr [rbx + 0xb8], 1
1405a2b7b: mov byte ptr [rbx + 0xb9], 1
1405a2b82: mov byte ptr [rbx + 0xba], 1
1405a2b89: mov byte ptr [rbx + 0xbb], 1
1405a2b90: mov rcx, qword ptr [rbx + 0xc8]
1405a2b97: call 0x140113ff0 ; bevy_app::app::App::plugins_state
1405a2b9c: cmp al, 1
1405a2b9e: ja 0x1405a6d84
1405a2ba4: mov byte ptr [rbx + 0xb4], 1
1405a2bab: mov byte ptr [rbx + 0xb5], 1
1405a2bb2: mov byte ptr [rbx + 0xb6], 1
1405a2bb9: mov byte ptr [rbx + 0xb7], 0
1405a2bc0: mov byte ptr [rbx + 0xb8], 1
1405a2bc7: mov byte ptr [rbx + 0xb9], 1
1405a2bce: mov byte ptr [rbx + 0xba], 1
1405a2bd5: mov byte ptr [rbx + 0xbb], 1
1405a2bdc: lea r8, [rip + 0x245cd85]
1405a2be3: mov ecx, esi
1405a2be5: mov rdx, qword ptr [rbx + 0xc8]
1405a2bec: call 0x14032afd0 ; bevy_app::plugin::sealed::impl$0::add_to_app<country_core::resources::prefab::PrefabWatchPlugin>
1405a2bf1: mov rax, qword ptr [rbp + 0x37b0]
1405a2bf8: movzx eax, byte ptr [rax + 0xe4]
1405a2bff: mov byte ptr [rbx + 0x107], al
1405a2c05: xor eax, eax
1405a2c07: mov rcx, qword ptr [rbp + 0x37c0]
1405a2c0e: cmp rax, qword ptr [rcx + 0xc8]
1405a2c15: jo 0x1405a30e7
1405a2c1b: add rcx, 0xc8
1405a2c22: mov qword ptr [rbx + 0x338], rcx
1405a2c29: mov byte ptr [rbx + 0xb4], 1
1405a2c30: mov byte ptr [rbx + 0xb5], 1
1405a2c37: mov byte ptr [rbx + 0xb6], 1
1405a2c3e: mov byte ptr [rbx + 0xb7], 0
1405a2c45: mov byte ptr [rbx + 0xb8], 1
1405a2c4c: mov byte ptr [rbx + 0xb9], 1
1405a2c53: mov byte ptr [rbx + 0xba], 1
1405a2c5a: mov byte ptr [rbx + 0xbb], 1
1405a2c61: call 0x140d76ed0 ; alloc::boxed::convert::impl$19::from::impl$0::description
1405a2c66: mov r8, rdx
1405a2c69: mov byte ptr [rbx + 0xb4], 1
1405a2c70: mov byte ptr [rbx + 0xb5], 1
1405a2c77: mov byte ptr [rbx + 0xb6], 1
1405a2c7e: mov byte ptr [rbx + 0xb7], 0
1405a2c85: mov byte ptr [rbx + 0xb8], 1
1405a2c8c: mov byte ptr [rbx + 0xb9], 1
1405a2c93: mov byte ptr [rbx + 0xba], 1
1405a2c9a: mov byte ptr [rbx + 0xbb], 1
1405a2ca1: lea rcx, [rbx + 0xb00]
1405a2ca8: mov rdx, rax
1405a2cab: call 0x140d59140 ; std::fs::read_to_string::inner
1405a2cb0: mov byte ptr [rbx + 0xb4], 1
1405a2cb7: mov byte ptr [rbx + 0xb5], 1
1405a2cbe: mov byte ptr [rbx + 0xb6], 1
1405a2cc5: mov byte ptr [rbx + 0xb7], 0
1405a2ccc: mov byte ptr [rbx + 0xb8], 1
1405a2cd3: mov byte ptr [rbx + 0xb9], 1
1405a2cda: mov byte ptr [rbx + 0xba], 1
1405a2ce1: mov byte ptr [rbx + 0xbb], 1
1405a2ce8: lea rcx, [rbx + 0x2590]
1405a2cef: lea rdx, [rbx + 0xb00]
1405a2cf6: lea r8, [rbx + 0x338]
1405a2cfd: call 0x140311ea0 ; anyhow::context::impl$0::with_context<alloc::string::String,std::io::error::Error,alloc::string::String,tiny_glade::run_game::run_game_thread::closure_env$3>
1405a2d02: mov rax, qword ptr [rbx + 0x2590]
1405a2d09: mov r8, qword ptr [rbx + 0x2598]
1405a2d10: mov qword ptr [rbx + 0x130], rax
1405a2d17: neg rax
1405a2d1a: jo 0x1405a6e72
1405a2d20: mov r9, qword ptr [rbx + 0x25a0]
1405a2d27: mov qword ptr [rbx + 0xb10], 0
1405a2d32: mov qword ptr [rbx + 0xb00], 1
1405a2d3d: mov qword ptr [rbx + 0xb08], 0x80
1405a2d48: lea rcx, [rbx + 0x440]
1405a2d4f: lea rdx, [rbx + 0xb00]
1405a2d56: mov qword ptr [rbx + 0x128], r8
1405a2d5d: call 0x1405233f0 ; ron::options::Options::from_str_seed<core::marker::PhantomData<alloc::collections::vec_deque::VecDeque<enum2$<country_core::meta_commander::MetaCommand>,alloc::alloc::Global> >,alloc::collections::vec_deque::VecDeque<enum2$<country_core::meta_commander::MetaCommand>,alloc::alloc::Global> >
1405a2d62: movzx eax, byte ptr [rbx + 0x440]
1405a2d69: cmp al, 0x32
1405a2d6b: jne 0x1405a3153
1405a2d71: mov r13, qword ptr [rbx + 0x448]
1405a2d78: mov rsi, qword ptr [rbx + 0x450]
1405a2d7f: movups xmm0, xmmword ptr [rbx + 0x458]
1405a2d86: movaps xmmword ptr [rbx + 0xb00], xmm0
1405a2d8d: mov rdx, qword ptr [rbx + 0x130]
1405a2d94: test rdx, rdx
1405a2d97: je 0x1405a2dab
1405a2d99: mov r8d, 1
1405a2d9f: mov rcx, qword ptr [rbx + 0x128]
1405a2da6: call 0x140613c20 ; 
1405a2dab: mov rax, r13
1405a2dae: neg rax
1405a2db1: jo 0x1405a6e6f
1405a2db7: mov qword ptr [rbx + 0x2230], r13
1405a2dbe: mov qword ptr [rbx + 0x2238], rsi
1405a2dc5: movaps xmm0, xmmword ptr [rbx + 0xb00]
1405a2dcc: movups xmmword ptr [rbx + 0x2240], xmm0
1405a2dd3: lea r15, [rbx + 0x2598]
1405a2dda: xor r12d, r12d
1405a2ddd: movabs r13, 0x8000000000000014
1405a2de7: movabs r14, 0x8000000000000000
1405a2df1: xorps xmm12, xmm12
1405a2df5: lea rsi, [rbx + 0xb00]
1405a2dfc: lea rdi, [rbx + 0x2590]
1405a2e03: xorps xmm13, xmm13
1405a2e07: nop word ptr [rax + rax]
1405a2e10: mov rax, qword ptr [rbx + 0x2248]
1405a2e17: test rax, rax
1405a2e1a: je 0x1405a309c
1405a2e20: mov r9, qword ptr [rbx + 0x2240]
1405a2e27: lea r8, [r9 + 1]
1405a2e2b: mov rdx, qword ptr [rbx + 0x2230]
1405a2e32: mov rcx, qword ptr [rbx + 0x2238]
1405a2e39: cmp r8, rdx
1405a2e3c: mov r8, rdx
1405a2e3f: cmovb r8, r12
1405a2e43: neg r8
1405a2e46: add r8, r9
1405a2e49: inc r8
1405a2e4c: mov qword ptr [rbx + 0x2240], r8
1405a2e53: lea r10, [rax - 1]
1405a2e57: mov qword ptr [rbx + 0x2248], r10
1405a2e5e: imul r10, r9, 0x38
1405a2e62: mov r9, qword ptr [rcx + r10]
1405a2e66: movups xmm0, xmmword ptr [rcx + r10 + 8]
1405a2e6c: movaps xmmword ptr [rbx + 0x1ff0], xmm0
1405a2e73: movups xmm0, xmmword ptr [rcx + r10 + 0x18]
1405a2e79: movups xmm1, xmmword ptr [rcx + r10 + 0x28]
1405a2e7f: movaps xmmword ptr [rbx + 0x2bf0], xmm0
1405a2e86: movaps xmmword ptr [rbx + 0x2c00], xmm1
1405a2e8d: cmp r9, r13
1405a2e90: je 0x1405a309c
1405a2e96: mov qword ptr [rbx + 0x2590], r9
1405a2e9d: movaps xmm0, xmmword ptr [rbx + 0x1ff0]
1405a2ea4: movups xmmword ptr [r15], xmm0
1405a2ea8: movaps xmm0, xmmword ptr [rbx + 0x2bf0]
1405a2eaf: movaps xmm1, xmmword ptr [rbx + 0x2c00]
1405a2eb6: movups xmmword ptr [r15 + 0x20], xmm1
1405a2ebb: movups xmmword ptr [r15 + 0x10], xmm0
1405a2ec0: mov r10, r9
1405a2ec3: xor r10, r14
1405a2ec6: cmp r10, 6
1405a2eca: je 0x1405a2f50
1405a2ed0: cmp r10, 7
1405a2ed4: jne 0x1405a3976
1405a2eda: lea rax, [rbx + 0xb10]
1405a2ee1: movups xmmword ptr [rax], xmm13
1405a2ee5: lea rax, [rbx + 0xb30]
1405a2eec: movups xmmword ptr [rax], xmm13
1405a2ef0: mov qword ptr [rbx + 0xb00], 0
1405a2efb: mov qword ptr [rbx + 0xb08], 8
1405a2f06: mov qword ptr [rbx + 0xb20], 0
1405a2f11: mov qword ptr [rbx + 0xb28], 8
1405a2f1c: movabs rax, 0x8000000000000003
1405a2f26: mov qword ptr [rbx + 0xb40], rax
1405a2f2d: mov byte ptr [rbx + 0xd9], 1
1405a2f34: mov byte ptr [rbx + 0xda], 1
1405a2f3b: mov rcx, qword ptr [rbx + 0xc8]
1405a2f42: mov rdx, rsi
1405a2f45: call 0x140137910 ; bevy_app::app::App::insert_resource<country_core::input::input_stream::InputRecordReplayState>
1405a2f4a: jmp 0x1405a3002
1405a2f4f: nop 
1405a2f50: movss xmm0, dword ptr [rbx + 0x2598]
1405a2f58: ucomiss xmm12, xmm0
1405a2f5c: ja 0x1405a65b1
1405a2f62: movd r9d, xmm0
1405a2f67: mov r8d, r9d
1405a2f6a: shr r8d, 0x17
1405a2f6e: movzx ecx, r8b
1405a2f72: xor eax, eax
1405a2f74: cmp cx, 0x60
1405a2f78: jb 0x1405a2fcb
1405a2f7a: mov edx, r9d
1405a2f7d: and edx, 0x7fffff
1405a2f83: or edx, 0x800000
1405a2f89: cmp cx, 0x7f
1405a2f8d: jae 0x1405a3023
1405a2f93: mov eax, edx
1405a2f95: lea ecx, [r8 + 0x2a]
1405a2f99: shl rax, cl
1405a2f9c: mov ecx, 0x3b9aca00
1405a2fa1: mul rcx
1405a2fa4: mov rcx, rax
1405a2fa7: neg rcx
1405a2faa: setno cl
1405a2fad: shr rax, 0x3f
1405a2fb1: mov r8d, edx
1405a2fb4: or r8b, cl
1405a2fb7: and r8b, al
1405a2fba: movzx eax, r8b
1405a2fbe: add eax, edx
1405a2fc0: cmp eax, 0x3b9aca00
1405a2fc5: je 0x1405a65b1
1405a2fcb: xor edx, edx
1405a2fcd: mov qword ptr [rbx + 0xb08], rdx
1405a2fd4: mov dword ptr [rbx + 0xb10], eax
1405a2fda: mov qword ptr [rbx + 0xb00], 2
1405a2fe5: mov byte ptr [rbx + 0xd9], 1
1405a2fec: mov byte ptr [rbx + 0xda], 1
1405a2ff3: mov rcx, qword ptr [rbx + 0xc8]
1405a2ffa: mov rdx, rsi
1405a2ffd: call 0x140135170 ; bevy_app::app::App::insert_resource<enum2$<bevy_time::TimeUpdateStrategy> >
1405a3002: cmp r12, qword ptr [rbx + 0x2590]
1405a3009: jo 0x1405a2e10
1405a300f: mov byte ptr [rbx + 0xf4], 1
1405a3016: mov rcx, rdi
1405a3019: call 0x140592cc0 ; core::ptr::drop_in_place<enum2$<country_core::meta_commander::MetaCommand> >
1405a301e: jmp 0x1405a2e10
1405a3023: movzx ecx, cx
1405a3026: cmp ecx, 0x96
1405a302c: jae 0x1405a3082
1405a302e: lea eax, [r8 + 1]
1405a3032: mov cl, 0x16
1405a3034: sub cl, r8b
1405a3037: shr edx, cl
1405a3039: mov ecx, eax
1405a303b: shl r9d, cl
1405a303e: and r9d, 0x7fffff
1405a3045: imul rax, r9, 0x3b9aca00
1405a304c: mov rcx, rax
1405a304f: shr rcx, 0x17
1405a3053: mov r8d, eax
1405a3056: and r8d, 0x7ffe00
1405a305d: cmp r8d, 0x400000
1405a3064: setne r8b
1405a3068: shr eax, 0x16
1405a306b: and eax, 1
1405a306e: mov r9d, ecx
1405a3071: or r9b, r8b
1405a3074: and r9b, al
1405a3077: movzx eax, r9b
1405a307b: add eax, ecx
1405a307d: jmp 0x1405a2fcd
1405a3082: cmp ecx, 0xbf
1405a3088: jae 0x1405a6ffd
1405a308e: mov edx, edx
1405a3090: lea ecx, [r8 + 0x2a]
1405a3094: shl rdx, cl
1405a3097: jmp 0x1405a2fcd
1405a309c: mov byte ptr [rbx + 0xb4], 1
1405a30a3: mov byte ptr [rbx + 0xb5], 1
1405a30aa: mov byte ptr [rbx + 0xb6], 1
1405a30b1: mov byte ptr [rbx + 0xb7], 0
1405a30b8: mov byte ptr [rbx + 0xb8], 1
1405a30bf: mov byte ptr [rbx + 0xb9], 1
1405a30c6: mov byte ptr [rbx + 0xba], 1
1405a30cd: mov byte ptr [rbx + 0xbb], 1
1405a30d4: lea rcx, [rbx + 0x2230]
1405a30db: call 0x1405d2cf0 ; core::ptr::drop_in_place<alloc::collections::vec_deque::VecDeque<enum2$<country_core::meta_commander::MetaCommand>,alloc::alloc::Global> >
1405a30e0: mov rcx, qword ptr [rbp + 0x37c0]
1405a30e7: xor eax, eax
1405a30e9: cmp rax, qword ptr [rcx + 0xa8]
1405a30f0: jno 0x1405a3140
1405a30f2: mov byte ptr [rbx + 0xb4], 1
1405a30f9: mov byte ptr [rbx + 0xb5], 1
1405a3100: mov byte ptr [rbx + 0xb6], 1
1405a3107: mov byte ptr [rbx + 0xb7], 0
1405a310e: mov byte ptr [rbx + 0xb8], 1
1405a3115: mov byte ptr [rbx + 0xb9], 1
1405a311c: mov byte ptr [rbx + 0xba], 1
1405a3123: mov byte ptr [rbx + 0xbb], 1
1405a312a: mov dil, 1
1405a312d: mov rcx, qword ptr [rbx + 0xc8]
1405a3134: xor edx, edx
1405a3136: call 0x1401209b0 ; bevy_app::app::App::insert_resource<country_core::input::panic_repro_state::PanicReproState>
1405a313b: jmp 0x1405a3e37
1405a3140: cmp byte ptr [rcx + 0x20], 0
1405a3144: je 0x1405a31e2
1405a314a: mov rdi, qword ptr [rcx + 0x28]
1405a314e: jmp 0x1405a31e4
1405a3153: mov ecx, dword ptr [rbx + 0x441]
1405a3159: mov edx, dword ptr [rbx + 0x444]
1405a315f: mov dword ptr [rbx + 0xb04], edx
1405a3165: mov dword ptr [rbx + 0xb01], ecx
1405a316b: movups xmm0, xmmword ptr [rbx + 0x448]
1405a3172: movups xmm1, xmmword ptr [rbx + 0x458]
1405a3179: movaps xmmword ptr [rbx + 0x2bf0], xmm1
1405a3180: movups xmm2, xmmword ptr [rbx + 0x468]
1405a3187: movups xmmword ptr [rbx + 0xb28], xmm2
1405a318e: movups xmm2, xmmword ptr [rbx + 0x478]
1405a3195: movups xmmword ptr [rbx + 0xb38], xmm2
1405a319c: movups xmm2, xmmword ptr [rbx + 0x488]
1405a31a3: movups xmmword ptr [rbx + 0xb48], xmm2
1405a31aa: movups xmmword ptr [rbx + 0xb08], xmm0
1405a31b1: movups xmmword ptr [rbx + 0xb18], xmm1
1405a31b8: mov byte ptr [rbx + 0xb00], al
1405a31be: lea rcx, [rbx + 0xb00]
1405a31c5: call 0x1428a3e00 ; anyhow::error::impl$1::from<ron::error::SpannedError>
1405a31ca: mov rsi, rax
1405a31cd: mov rdx, qword ptr [rbx + 0x130]
1405a31d4: test rdx, rdx
1405a31d7: jne 0x1405a2d99
1405a31dd: jmp 0x1405a2dab
1405a31e2: xor edi, edi
1405a31e4: mov byte ptr [rbx + 0xb4], 1
1405a31eb: mov byte ptr [rbx + 0xb5], 1
1405a31f2: mov byte ptr [rbx + 0xb6], 1
1405a31f9: mov byte ptr [rbx + 0xb7], 0
1405a3200: mov byte ptr [rbx + 0xb8], 1
1405a3207: mov byte ptr [rbx + 0xb9], 1
1405a320e: mov byte ptr [rbx + 0xba], 1
1405a3215: mov byte ptr [rbx + 0xbb], 1
1405a321c: lea rcx, [rbx + 0x2d0]
1405a3223: call 0x140916420 ; country_core::resources::session::SessionId::new
1405a3228: mov byte ptr [rbx + 0xb4], 1
1405a322f: mov byte ptr [rbx + 0xb5], 1
1405a3236: mov byte ptr [rbx + 0xb6], 1
1405a323d: mov byte ptr [rbx + 0xb7], 0
1405a3244: mov byte ptr [rbx + 0xb8], 1
1405a324b: mov byte ptr [rbx + 0xb9], 1
1405a3252: mov byte ptr [rbx + 0xba], 1
1405a3259: mov byte ptr [rbx + 0xbb], 1
1405a3260: lea rcx, [rbx + 0xb00]
1405a3267: lea rdx, [rbx + 0x2d0]
1405a326e: call 0x140b46000 ; country_core::resources::session::SessionId::save_folder_path
1405a3273: mov byte ptr [rbx + 0x440], 1
1405a327a: lea rcx, [rbx + 0xb00]
1405a3281: call 0x140d76ed0 ; alloc::boxed::convert::impl$19::from::impl$0::description
1405a3286: mov r8, rdx
1405a3289: lea rcx, [rbx + 0x440]
1405a3290: mov rdx, rax
1405a3293: call 0x140d5a150 ; std::fs::DirBuilder::_create
1405a3298: mov rsi, rax
1405a329b: mov rdx, qword ptr [rbx + 0xb00]
1405a32a2: test rdx, rdx
1405a32a5: je 0x1405a32b9
1405a32a7: mov rcx, qword ptr [rbx + 0xb08]
1405a32ae: mov r8d, 1
1405a32b4: call 0x140613c20 ; 
1405a32b9: test rsi, rsi
1405a32bc: jne 0x1405a6ea9
1405a32c2: movups xmm0, xmmword ptr [rbx + 0x2d0]
1405a32c9: movaps xmmword ptr [rbx + 0xb00], xmm0
1405a32d0: mov byte ptr [rbx + 0xb4], 1
1405a32d7: mov byte ptr [rbx + 0xb5], 1
1405a32de: mov byte ptr [rbx + 0xb6], 1
1405a32e5: mov byte ptr [rbx + 0xb7], 0
1405a32ec: mov byte ptr [rbx + 0xb8], 1
1405a32f3: mov byte ptr [rbx + 0xb9], 1
1405a32fa: mov byte ptr [rbx + 0xba], 1
1405a3301: mov byte ptr [rbx + 0xbb], 1
1405a3308: lea rcx, [rbx + 0x440]
1405a330f: lea rdx, [rbx + 0xb00]
1405a3316: call 0x140b46920 ; country_core::resources::session::NextSession::load
1405a331b: mov rax, qword ptr [rbp + 0x37c0]
1405a3322: lea rsi, [rax + 0xa8]
1405a3329: mov qword ptr [rbx + 0x2bf8], 0xa
1405a3334: movabs rax, 0x8000000000000005
1405a333e: mov qword ptr [rbx + 0x2bf0], rax
1405a3345: movabs rax, 0x8000000000000001
1405a334f: mov qword ptr [rbx + 0x2590], rax
1405a3356: mov rcx, rsi
1405a3359: call 0x140d76ed0 ; alloc::boxed::convert::impl$19::from::impl$0::description
1405a335e: mov r8, rdx
1405a3361: mov qword ptr [rsp + 0x20], 0xb
1405a336a: lea r9, [rip + 0x245c67f]
1405a3371: lea rcx, [rbx + 0x2230]
1405a3378: mov rdx, rax
1405a337b: call 0x140d62810 ; std::path::Path::_join
1405a3380: movups xmm0, xmmword ptr [rbx + 0x2230]
1405a3387: movups xmm1, xmmword ptr [rbx + 0x2240]
1405a338e: movups xmmword ptr [rbx + 0xb88], xmm1
1405a3395: movups xmmword ptr [rbx + 0xb78], xmm0
1405a339c: movups xmm0, xmmword ptr [rbx + 0x2bf0]
1405a33a3: movups xmm1, xmmword ptr [rbx + 0x2c00]
1405a33aa: movups xmm2, xmmword ptr [rbx + 0x2c10]
1405a33b1: movaps xmmword ptr [rbx + 0xb00], xmm0
1405a33b8: movaps xmmword ptr [rbx + 0xb10], xmm1
1405a33bf: movaps xmmword ptr [rbx + 0xb20], xmm2
1405a33c6: mov rax, qword ptr [rbx + 0x2c20]
1405a33cd: mov qword ptr [rbx + 0xb30], rax
1405a33d4: movups xmm0, xmmword ptr [rbx + 0x2590]
1405a33db: movups xmm1, xmmword ptr [rbx + 0x25a0]
1405a33e2: movups xmm2, xmmword ptr [rbx + 0x25b0]
1405a33e9: movups xmmword ptr [rbx + 0xb38], xmm0
1405a33f0: movups xmmword ptr [rbx + 0xb48], xmm1
1405a33f7: movups xmmword ptr [rbx + 0xb58], xmm2
1405a33fe: mov rax, qword ptr [rbx + 0x25c0]
1405a3405: mov qword ptr [rbx + 0xb68], rax
1405a340c: movabs rax, 0x8000000000000008
1405a3416: mov qword ptr [rbx + 0xb70], rax
1405a341d: mov qword ptr [rbx + 0xb98], rdi
1405a3424: mov byte ptr [rbx + 0x11e], 0
1405a342b: lea rcx, [rbx + 0x2590]
1405a3432: lea rdx, [rbx + 0x440]
1405a3439: lea r8, [rbx + 0xb00]
1405a3440: call 0x1403b5150 ; country_core::resources::session::NextSession::append_on_load<array$<enum2$<country_core::meta_commander::MetaCommand>,3> >
1405a3445: mov byte ptr [rbx + 0xb4], 1
1405a344c: mov byte ptr [rbx + 0xb5], 1
1405a3453: mov byte ptr [rbx + 0xb6], 1
1405a345a: mov byte ptr [rbx + 0xb7], 0
1405a3461: mov byte ptr [rbx + 0xb8], 1
1405a3468: mov byte ptr [rbx + 0xb9], 1
1405a346f: mov byte ptr [rbx + 0xba], 1
1405a3476: mov byte ptr [rbx + 0xbb], 1
1405a347d: lea rdx, [rbx + 0x2590]
1405a3484: mov rcx, qword ptr [rbx + 0xc8]
1405a348b: call 0x14011a3b0 ; bevy_app::app::App::insert_resource<country_core::resources::session::NextSession>
1405a3490: mov byte ptr [rbx + 0xb4], 1
1405a3497: mov byte ptr [rbx + 0xb5], 1
1405a349e: mov byte ptr [rbx + 0xb6], 1
1405a34a5: mov byte ptr [rbx + 0xb7], 0
1405a34ac: mov byte ptr [rbx + 0xb8], 1
1405a34b3: mov byte ptr [rbx + 0xb9], 1
1405a34ba: mov byte ptr [rbx + 0xba], 1
1405a34c1: mov byte ptr [rbx + 0xbb], 1
1405a34c8: mov rcx, rsi
1405a34cb: call 0x140d76ed0 ; alloc::boxed::convert::impl$19::from::impl$0::description
1405a34d0: mov r8, rdx
1405a34d3: mov byte ptr [rbx + 0xb4], 1
1405a34da: mov byte ptr [rbx + 0xb5], 1
1405a34e1: mov byte ptr [rbx + 0xb6], 1
1405a34e8: mov byte ptr [rbx + 0xb7], 0
1405a34ef: mov byte ptr [rbx + 0xb8], 1
1405a34f6: mov byte ptr [rbx + 0xb9], 1
1405a34fd: mov byte ptr [rbx + 0xba], 1
1405a3504: mov byte ptr [rbx + 0xbb], 1
1405a350b: mov qword ptr [rsp + 0x20], 9
1405a3514: lea r9, [rip + 0x245c4e0]
1405a351b: lea rcx, [rbx + 0x2590]
1405a3522: mov rdx, rax
1405a3525: call 0x140d62810 ; std::path::Path::_join
1405a352a: lea rcx, [rbx + 0xb00]
1405a3531: lea rdx, [rbx + 0x2d0]
1405a3538: call 0x140b46000 ; country_core::resources::session::SessionId::save_folder_path
1405a353d: mov byte ptr [rbx + 0x111], 1
1405a3544: lea rcx, [rbx + 0xb00]
1405a354b: call 0x140d76ed0 ; alloc::boxed::convert::impl$19::from::impl$0::description
1405a3550: mov r8, rdx
1405a3553: mov byte ptr [rbx + 0x111], 1
1405a355a: mov qword ptr [rsp + 0x20], 0xc
1405a3563: lea r9, [rip + 0x245c49a]
1405a356a: lea rcx, [rbx + 0x440]
1405a3571: mov rdx, rax
1405a3574: call 0x140d62810 ; std::path::Path::_join
1405a3579: mov byte ptr [rbx + 0x111], 0
1405a3580: lea rdi, [rbx + 0x2590]
1405a3587: lea rdx, [rbx + 0x440]
1405a358e: mov rcx, rdi
1405a3591: call 0x1403107e0 ; std::fs::copy<std::path::PathBuf,std::path::PathBuf>
1405a3596: test al, 1
1405a3598: jne 0x1405a6ee0
1405a359e: mov rdx, qword ptr [rbx + 0xb00]
1405a35a5: test rdx, rdx
1405a35a8: je 0x1405a35bc
1405a35aa: mov rcx, qword ptr [rbx + 0xb08]
1405a35b1: mov r8d, 1
1405a35b7: call 0x140613c20 ; 
1405a35bc: mov byte ptr [rbx + 0xb4], 1
1405a35c3: mov byte ptr [rbx + 0xb5], 1
1405a35ca: mov byte ptr [rbx + 0xb6], 1
1405a35d1: mov byte ptr [rbx + 0xb7], 0
1405a35d8: mov byte ptr [rbx + 0xb8], 1
1405a35df: mov byte ptr [rbx + 0xb9], 1
1405a35e6: mov byte ptr [rbx + 0xba], 1
1405a35ed: mov byte ptr [rbx + 0xbb], 1
1405a35f4: mov rcx, rsi
1405a35f7: call 0x140d76ed0 ; alloc::boxed::convert::impl$19::from::impl$0::description
1405a35fc: mov r8, rdx
1405a35ff: mov byte ptr [rbx + 0xb4], 1
1405a3606: mov byte ptr [rbx + 0xb5], 1
1405a360d: mov byte ptr [rbx + 0xb6], 1
1405a3614: mov byte ptr [rbx + 0xb7], 0
1405a361b: mov byte ptr [rbx + 0xb8], 1
1405a3622: mov byte ptr [rbx + 0xb9], 1
1405a3629: mov byte ptr [rbx + 0xba], 1
1405a3630: mov byte ptr [rbx + 0xbb], 1
1405a3637: mov qword ptr [rsp + 0x20], 9
1405a3640: lea r9, [rip + 0x245c3e1]
1405a3647: lea rcx, [rbx + 0x1ff0]
1405a364e: mov rdx, rax
1405a3651: call 0x140d62810 ; std::path::Path::_join
1405a3656: mov byte ptr [rbx + 0xd3], 1
1405a365d: lea rcx, [rbx + 0x1ff0]
1405a3664: call 0x140d76ed0 ; alloc::boxed::convert::impl$19::from::impl$0::description
1405a3669: mov byte ptr [rbx + 0xd3], 1
1405a3670: mov rcx, rax
1405a3673: call 0x140d62e10 ; std::path::Path::is_dir
1405a3678: test al, al
1405a367a: je 0x1405a3702
1405a3680: mov byte ptr [rbx + 0xd3], 1
1405a3687: lea rcx, [rbx + 0xb00]
1405a368e: lea rdx, [rbx + 0x2d0]
1405a3695: call 0x140b46000 ; country_core::resources::session::SessionId::save_folder_path
1405a369a: lea rcx, [rbx + 0xb00]
1405a36a1: call 0x140d76ed0 ; alloc::boxed::convert::impl$19::from::impl$0::description
1405a36a6: mov r8, rdx
1405a36a9: mov qword ptr [rsp + 0x20], 9
1405a36b2: lea r9, [rip + 0x245c36f]
1405a36b9: lea rcx, [rbx + 0x440]
1405a36c0: mov rdx, rax
1405a36c3: call 0x140d62810 ; std::path::Path::_join
1405a36c8: lea rcx, [rbx + 0x1ff0]
1405a36cf: lea rdx, [rbx + 0x440]
1405a36d6: call 0x140355800 ; tiny_glade::fs_util::copy_folder<ref$<std::path::PathBuf>,std::path::PathBuf>
1405a36db: test rax, rax
1405a36de: jne 0x1405a700e
1405a36e4: mov rdx, qword ptr [rbx + 0xb00]
1405a36eb: test rdx, rdx
1405a36ee: je 0x1405a3702
1405a36f0: mov rcx, qword ptr [rbx + 0xb08]
1405a36f7: mov r8d, 1
1405a36fd: call 0x140613c20 ; 
1405a3702: mov byte ptr [rbx + 0xd3], 1
1405a3709: lea rcx, [rbx + 0xb00]
1405a3710: lea rdx, [rbx + 0x2d0]
1405a3717: call 0x140b46000 ; country_core::resources::session::SessionId::save_folder_path
1405a371c: lea rcx, [rbx + 0xb00]
1405a3723: call 0x140d76ed0 ; alloc::boxed::convert::impl$19::from::impl$0::description
1405a3728: mov r8, rdx
1405a372b: mov qword ptr [rsp + 0x20], 0xc
1405a3734: lea r9, [rip + 0x245c315]
1405a373b: lea rcx, [rbx + 0x440]
1405a3742: mov rdx, rax
1405a3745: call 0x140d62810 ; std::path::Path::_join
1405a374a: lea r14, [rbx + 0x440]
1405a3751: mov rcx, r14
1405a3754: call 0x140d76ed0 ; alloc::boxed::convert::impl$19::from::impl$0::description
1405a3759: lea r8, [rip + 0x245c2fc]
1405a3760: mov r9d, 5
1405a3766: mov rcx, rax
1405a3769: call 0x140d59430 ; std::fs::write::inner
1405a376e: mov r15, rax
1405a3771: mov rdx, qword ptr [rbx + 0x440]
1405a3778: test rdx, rdx
1405a377b: je 0x1405a378f
1405a377d: mov rcx, qword ptr [rbx + 0x448]
1405a3784: mov r8d, 1
1405a378a: call 0x140613c20 ; 
1405a378f: test r15, r15
1405a3792: jne 0x1405a6f17
1405a3798: mov rdx, qword ptr [rbx + 0xb00]
1405a379f: test rdx, rdx
1405a37a2: je 0x1405a37b6
1405a37a4: mov rcx, qword ptr [rbx + 0xb08]
1405a37ab: mov r8d, 1
1405a37b1: call 0x140613c20 ; 
1405a37b6: mov byte ptr [rbx + 0xd3], 1
1405a37bd: mov rcx, rsi
1405a37c0: call 0x140d76ed0 ; alloc::boxed::convert::impl$19::from::impl$0::description
1405a37c5: mov r8, rdx
1405a37c8: mov byte ptr [rbx + 0xd3], 1
1405a37cf: mov qword ptr [rsp + 0x20], 0xc
1405a37d8: lea r9, [rip + 0x245c2a1]
1405a37df: lea rcx, [rbx + 0x2230]
1405a37e6: mov rdx, rax
1405a37e9: call 0x140d62810 ; std::path::Path::_join
1405a37ee: mov byte ptr [rbx + 0xc7], 1
1405a37f5: lea rcx, [rbx + 0x2230]
1405a37fc: call 0x140d76ed0 ; alloc::boxed::convert::impl$19::from::impl$0::description
1405a3801: mov r8, rdx
1405a3804: mov byte ptr [rbx + 0xc7], 1
1405a380b: lea rcx, [rbx + 0xb00]
1405a3812: mov rdx, rax
1405a3815: call 0x140d59140 ; std::fs::read_to_string::inner
1405a381a: mov byte ptr [rbx + 0xc7], 1
1405a3821: lea rcx, [rbx + 0x370]
1405a3828: lea rdx, [rbx + 0xb00]
1405a382f: lea r8, [rbx + 0x2230]
1405a3836: call 0x140311b40 ; anyhow::context::impl$0::with_context<alloc::string::String,std::io::error::Error,alloc::string::String,tiny_glade::run_game::run_game_thread::closure_env$5>
1405a383b: mov rcx, qword ptr [rbx + 0x370]
1405a3842: mov r8, qword ptr [rbx + 0x378]
1405a3849: mov rax, rcx
1405a384c: neg rax
1405a384f: jo 0x1405a3b68
1405a3855: mov qword ptr [rbx + 0x130], rcx
1405a385c: mov r9, qword ptr [rbx + 0x380]
1405a3863: mov qword ptr [rbx + 0x25a0], 0
1405a386e: mov qword ptr [rbx + 0x2590], 1
1405a3879: mov qword ptr [rbx + 0x2598], 0x80
1405a3884: lea rcx, [rbx + 0xb00]
1405a388b: lea rdx, [rbx + 0x2590]
1405a3892: mov qword ptr [rbx + 0x128], r8
1405a3899: call 0x140522ad0 ; ron::options::Options::from_str_seed<core::marker::PhantomData<country_core::game_config::GameConfig>,country_core::game_config::GameConfig>
1405a389e: mov r15, qword ptr [rbx + 0xb00]
1405a38a5: mov rsi, qword ptr [rbx + 0xb08]
1405a38ac: mov rax, r15
1405a38af: neg rax
1405a38b2: movups xmm0, xmmword ptr [rbx + 0xb10]
1405a38b9: movaps xmmword ptr [rbx + 0x440], xmm0
1405a38c0: movups xmm0, xmmword ptr [rbx + 0xb20]
1405a38c7: movaps xmmword ptr [rbx + 0x450], xmm0
1405a38ce: movups xmm0, xmmword ptr [rbx + 0xb30]
1405a38d5: movaps xmmword ptr [rbx + 0x460], xmm0
1405a38dc: movups xmm0, xmmword ptr [rbx + 0xb40]
1405a38e3: movaps xmmword ptr [rbx + 0x470], xmm0
1405a38ea: movups xmm0, xmmword ptr [rbx + 0xb50]
1405a38f1: movaps xmmword ptr [rbx + 0x480], xmm0
1405a38f8: jno 0x1405a3ad5
1405a38fe: mov qword ptr [rbx + 0xb00], rsi
1405a3905: movaps xmm0, xmmword ptr [rbx + 0x440]
1405a390c: movaps xmm1, xmmword ptr [rbx + 0x450]
1405a3913: movaps xmm2, xmmword ptr [rbx + 0x460]
1405a391a: movaps xmm3, xmmword ptr [rbx + 0x470]
1405a3921: movups xmmword ptr [rbx + 0xb08], xmm0
1405a3928: movups xmmword ptr [rbx + 0xb18], xmm1
1405a392f: movups xmmword ptr [rbx + 0xb28], xmm2
1405a3936: movups xmmword ptr [rbx + 0xb38], xmm3
1405a393d: movaps xmm0, xmmword ptr [rbx + 0x480]
1405a3944: movups xmmword ptr [rbx + 0xb48], xmm0
1405a394b: lea rcx, [rbx + 0xb00]
1405a3952: call 0x1428a3e00 ; anyhow::error::impl$1::from<ron::error::SpannedError>
1405a3957: mov rsi, rax
1405a395a: mov rdx, qword ptr [rbx + 0x130]
1405a3961: test rdx, rdx
1405a3964: mov r12, qword ptr [rbp + 0x37b0]
1405a396b: jne 0x1405a3b47
1405a3971: jmp 0x1405a3b59
1405a3976: test r10, r10
1405a3979: jne 0x1405a6441
1405a397f: movaps xmm0, xmmword ptr [rbx + 0x1ff0]
1405a3986: movaps xmmword ptr [rbx + 0x1fe0], xmm0
1405a398d: movzx esi, byte ptr [rbx + 0x25b0]
1405a3994: mov rax, qword ptr [rbx + 0x25a8]
1405a399b: mov qword ptr [rbx + 0x370], rax
1405a39a2: test rax, rax
1405a39a5: jne 0x1405a658d
1405a39ab: mov byte ptr [rbx + 0xe3], 1
1405a39b2: lea rcx, [rbx + 0xb00]
1405a39b9: lea rdx, [rbx + 0x1fe0]
1405a39c0: call 0x140b46920 ; country_core::resources::session::NextSession::load
1405a39c5: mov byte ptr [rbx + 0xe3], 1
1405a39cc: lea rcx, [rbx + 0x440]
1405a39d3: lea rdx, [rbx + 0xb00]
1405a39da: mov r8d, esi
1405a39dd: call 0x140b46a20 ; country_core::resources::session::NextSession::ignore_snapshot
1405a39e2: movups xmm0, xmmword ptr [rbx + 0x2230]
1405a39e9: movups xmm1, xmmword ptr [rbx + 0x2240]
1405a39f0: movaps xmmword ptr [rbx + 0xb10], xmm1
1405a39f7: movaps xmmword ptr [rbx + 0xb00], xmm0
1405a39fe: mov rsi, qword ptr [rbx + 0x450]
1405a3a05: cmp rsi, qword ptr [rbx + 0x440]
1405a3a0c: jne 0x1405a3a21
1405a3a0e: lea rdx, [rip + 0x2429093]
1405a3a15: lea rcx, [rbx + 0x440]
1405a3a1c: call 0x140562c30 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::water::WaterEraseEvent>,alloc::alloc::Global>
1405a3a21: mov rax, qword ptr [rbx + 0x448]
1405a3a28: mov rcx, rsi
1405a3a2b: shl rcx, 5
1405a3a2f: movaps xmm0, xmmword ptr [rbx + 0xb00]
1405a3a36: movaps xmm1, xmmword ptr [rbx + 0xb10]
1405a3a3d: movups xmmword ptr [rax + rcx + 0x10], xmm1
1405a3a42: movups xmmword ptr [rax + rcx], xmm0
1405a3a46: inc rsi
1405a3a49: mov qword ptr [rbx + 0x450], rsi
1405a3a50: lea rsi, [rbx + 0xb00]
1405a3a57: lea rdx, [rbx + 0x440]
1405a3a5e: mov r8d, 0xa0
1405a3a64: mov rcx, rsi
1405a3a67: call 0x142923c00 ; 
1405a3a6c: mov byte ptr [rbx + 0xe3], 0
1405a3a73: mov rcx, qword ptr [rbx + 0xc8]
1405a3a7a: mov rdx, rsi
1405a3a7d: call 0x14011a3b0 ; bevy_app::app::App::insert_resource<country_core::resources::session::NextSession>
1405a3a82: mov rax, qword ptr [rbx + 0x370]
1405a3a89: test rax, rax
1405a3a8c: je 0x1405a3aae
1405a3a8e: lock dec qword ptr [rax]
1405a3a92: jne 0x1405a3aae
1405a3a94: mov byte ptr [rbx + 0xd9], 0
1405a3a9b: mov byte ptr [rbx + 0xda], 1
1405a3aa2: lea rcx, [rbx + 0x370]
1405a3aa9: call 0x1402e5d80 ; alloc::sync::Arc::drop_slow<core::sync::atomic::AtomicBool,alloc::alloc::Global>
1405a3aae: xor eax, eax
1405a3ab0: cmp rax, qword ptr [rbx + 0x2590]
1405a3ab7: jo 0x1405a30e0
1405a3abd: mov byte ptr [rbx + 0xf4], 0
1405a3ac4: lea rcx, [rbx + 0x2590]
1405a3acb: call 0x140592cc0 ; core::ptr::drop_in_place<enum2$<country_core::meta_commander::MetaCommand> >
1405a3ad0: jmp 0x1405a30e0
1405a3ad5: lea rdx, [rbx + 0xb60]
1405a3adc: lea rcx, [rbx + 0x2c40]
1405a3ae3: mov r8d, 0xc8
1405a3ae9: call 0x142923c00 ; 
1405a3aee: movaps xmm0, xmmword ptr [rbx + 0x440]
1405a3af5: movaps xmm1, xmmword ptr [rbx + 0x450]
1405a3afc: movaps xmm2, xmmword ptr [rbx + 0x460]
1405a3b03: movaps xmm3, xmmword ptr [rbx + 0x470]
1405a3b0a: movaps xmmword ptr [rbx + 0x2bf0], xmm0
1405a3b11: movaps xmmword ptr [rbx + 0x2c00], xmm1
1405a3b18: movaps xmmword ptr [rbx + 0x2c10], xmm2
1405a3b1f: movaps xmmword ptr [rbx + 0x2c20], xmm3
1405a3b26: movaps xmm0, xmmword ptr [rbx + 0x480]
1405a3b2d: movaps xmmword ptr [rbx + 0x2c30], xmm0
1405a3b34: mov rdx, qword ptr [rbx + 0x130]
1405a3b3b: test rdx, rdx
1405a3b3e: mov r12, qword ptr [rbp + 0x37b0]
1405a3b45: je 0x1405a3b59
1405a3b47: mov r8d, 1
1405a3b4d: mov rcx, qword ptr [rbx + 0x128]
1405a3b54: call 0x140613c20 ; 
1405a3b59: mov rax, r15
1405a3b5c: neg rax
1405a3b5f: jno 0x1405a3c71
1405a3b65: mov r8, rsi
1405a3b68: mov qword ptr [rbx + 0x2590], r8
1405a3b6f: mov rax, qword ptr [rip + 0x3088832]
1405a3b76: mov rax, qword ptr [rax]
1405a3b79: cmp rax, 2
1405a3b7d: jb 0x1405a3c56
1405a3b83: mov qword ptr [rbx + 0x440], rdi
1405a3b8a: lea rax, [rip + 0x7a908f]
1405a3b91: mov qword ptr [rbx + 0x448], rax
1405a3b98: lea rcx, [rip + 0x245bf21]
1405a3b9f: call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
1405a3ba4: movups xmm0, xmmword ptr [rax]
1405a3ba7: mov eax, dword ptr [rax + 0x10]
1405a3baa: mov qword ptr [rbx + 0xb30], 2
1405a3bb5: lea rcx, [rip + 0x245b834]
1405a3bbc: mov qword ptr [rbx + 0xb38], rcx
1405a3bc3: mov qword ptr [rbx + 0xb40], 0x14
1405a3bce: lea rdx, [rip + 0x245bedb]
1405a3bd5: mov qword ptr [rbx + 0xb50], rdx
1405a3bdc: mov qword ptr [rbx + 0xb58], 1
1405a3be7: mov qword ptr [rbx + 0xb60], r14
1405a3bee: mov qword ptr [rbx + 0xb68], 1
1405a3bf9: mov qword ptr [rbx + 0xb70], 0
1405a3c04: mov qword ptr [rbx + 0xb00], 0
1405a3c0f: mov qword ptr [rbx + 0xb08], rcx
1405a3c16: mov qword ptr [rbx + 0xb10], 0x14
1405a3c21: mov qword ptr [rbx + 0xb18], 0
1405a3c2c: movups xmmword ptr [rbx + 0xb20], xmm0
1405a3c33: mov dword ptr [rbx + 0xb48], 1
1405a3c3d: mov dword ptr [rbx + 0xb4c], eax
1405a3c43: lea rcx, [rbx + 0x370]
1405a3c4a: lea rdx, [rbx + 0xb00]
1405a3c51: call 0x140d4a330 ; log::__private_api::impl$2::log
1405a3c56: mov byte ptr [rbx + 0xc7], 1
1405a3c5d: lea rcx, [rbx + 0x2590]
1405a3c64: call 0x140d4cc30 ; anyhow::error::impl$6::drop
1405a3c69: mov sil, 1
1405a3c6c: jmp 0x1405a3d16
1405a3c71: mov qword ptr [rbx + 0x2590], r15
1405a3c78: mov qword ptr [rbx + 0x2598], rsi
1405a3c7f: lea rcx, [rbx + 0x25a0]
1405a3c86: lea rdx, [rbx + 0x2bf0]
1405a3c8d: mov r8d, 0x118
1405a3c93: call 0x142923c00 ; 
1405a3c98: lea rsi, [rbx + 0x440]
1405a3c9f: mov r8d, 0x128
1405a3ca5: mov rcx, rsi
1405a3ca8: mov rdx, r12
1405a3cab: call 0x142923c00 ; 
1405a3cb0: movups xmm0, xmmword ptr [rbx + 0x2cc8]
1405a3cb7: movups xmmword ptr [rbx + 0x528], xmm0
1405a3cbe: lea rdi, [rbx + 0xb00]
1405a3cc5: mov r8d, 0x128
1405a3ccb: mov rcx, rdi
1405a3cce: mov rdx, rsi
1405a3cd1: call 0x142923c00 ; 
1405a3cd6: mov rcx, qword ptr [rbx + 0xc8]
1405a3cdd: mov rdx, rdi
1405a3ce0: call 0x140119810 ; bevy_app::app::App::insert_resource<country_core::game_config::GameConfig>
1405a3ce5: lea rcx, [rbx + 0x2590]
1405a3cec: call 0x1405fdb80 ; core::ptr::drop_in_place<country_core::render::render_config::VideoSettings>
1405a3cf1: mov rdx, qword ptr [rbx + 0x2658]
1405a3cf8: mov rax, rdx
1405a3cfb: neg rax
1405a3cfe: jo 0x1405a3d14
1405a3d00: jae 0x1405a3d14
1405a3d02: mov rcx, qword ptr [rbx + 0x2660]
1405a3d09: mov r8d, 1
1405a3d0f: call 0x140613c20 ; 
1405a3d14: xor esi, esi
1405a3d16: mov rcx, qword ptr [rbx + 0xc8]
1405a3d1d: mov byte ptr [rbx + 0xc7], sil
1405a3d24: mov dl, 1
1405a3d26: call 0x1401209b0 ; bevy_app::app::App::insert_resource<country_core::input::panic_repro_state::PanicReproState>
1405a3d2b: mov byte ptr [rbx + 0xc7], sil
1405a3d32: lea r8, [rip + 0x245bd9f]
1405a3d39: lea rcx, [rbx + 0xb00]
1405a3d40: mov dl, 1
1405a3d42: call 0x1401525a0 ; bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<void (*)(bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::input::panic_repro_state::PanicReproState>),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::input::panic_repro_state::PanicReproState>)> >,country_core::systems::GameSystemSet>
1405a3d47: mov dword ptr [rbx + 0x130], esi
1405a3d4d: mov rcx, qword ptr [rip + 0x308861c]
1405a3d54: lea r8, [rip + 0x23c7b0d]
1405a3d5b: lea rdx, [rbx + 0x370]
1405a3d62: call 0x140364d20 ; bevy_ecs::intern::Interner::intern<dyn$<bevy_ecs::schedule::set::SystemSet> >
1405a3d67: mov r8, rdx
1405a3d6a: lea rcx, [rbx + 0xb00]
1405a3d71: mov rdx, rax
1405a3d74: call 0x1400d67b0 ; bevy_ecs::schedule::config::ScheduleConfigs::after_inner<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> >
1405a3d79: movups xmm0, xmmword ptr [rbx + 0xb60]
1405a3d80: movaps xmmword ptr [rbx + 0x4a0], xmm0
1405a3d87: movups xmm0, xmmword ptr [rbx + 0xb50]
1405a3d8e: movaps xmmword ptr [rbx + 0x490], xmm0
1405a3d95: movups xmm0, xmmword ptr [rbx + 0xb40]
1405a3d9c: movaps xmmword ptr [rbx + 0x480], xmm0
1405a3da3: movups xmm0, xmmword ptr [rbx + 0xb00]
1405a3daa: movups xmm1, xmmword ptr [rbx + 0xb10]
1405a3db1: movups xmm2, xmmword ptr [rbx + 0xb20]
1405a3db8: movups xmm3, xmmword ptr [rbx + 0xb30]
1405a3dbf: movaps xmmword ptr [rbx + 0x470], xmm3
1405a3dc6: movaps xmmword ptr [rbx + 0x460], xmm2
1405a3dcd: movaps xmmword ptr [rbx + 0x450], xmm1
1405a3dd4: movaps xmmword ptr [rbx + 0x440], xmm0
1405a3ddb: mov edi, dword ptr [rbx + 0x130]
1405a3de1: mov byte ptr [rbx + 0xc7], dil
1405a3de8: lea rdx, [rbx + 0x440]
1405a3def: mov rcx, qword ptr [rbx + 0xc8]
1405a3df6: call 0x1403276f0 ; bevy_app::sub_app::SubApp::add_systems<tuple$<>,bevy_app::main_schedule::Update,enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > > >
1405a3dfb: mov rdx, qword ptr [rbx + 0x2230]
1405a3e02: test rdx, rdx
1405a3e05: je 0x1405a3e19
1405a3e07: mov rcx, qword ptr [rbx + 0x2238]
1405a3e0e: mov r8d, 1
1405a3e14: call 0x140613c20 ; 
1405a3e19: mov rdx, qword ptr [rbx + 0x1ff0]
1405a3e20: test rdx, rdx
1405a3e23: je 0x1405a3e37
1405a3e25: mov rcx, qword ptr [rbx + 0x1ff8]
1405a3e2c: mov r8d, 1
1405a3e32: call 0x140613c20 ; 
1405a3e37: movaps xmm0, xmmword ptr [rbx + 0xaf0]
1405a3e3e: movaps xmmword ptr [rbx + 0xb50], xmm0
1405a3e45: movaps xmm0, xmmword ptr [rbx + 0xae0]
1405a3e4c: movaps xmmword ptr [rbx + 0xb40], xmm0
1405a3e53: movaps xmm0, xmmword ptr [rbx + 0xaa0]
1405a3e5a: movaps xmm1, xmmword ptr [rbx + 0xab0]
1405a3e61: movaps xmm2, xmmword ptr [rbx + 0xac0]
1405a3e68: movaps xmm3, xmmword ptr [rbx + 0xad0]
1405a3e6f: movaps xmmword ptr [rbx + 0xb30], xmm3
1405a3e76: movaps xmmword ptr [rbx + 0xb20], xmm2
1405a3e7d: movaps xmmword ptr [rbx + 0xb10], xmm1
1405a3e84: movaps xmmword ptr [rbx + 0xb00], xmm0
1405a3e8b: call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1405a3e90: mov rsi, rax
1405a3e93: test byte ptr [rax], 1
1405a3e96: mov dword ptr [rbx + 0x130], edi
1405a3e9c: je 0x1405a3eb7
1405a3e9e: mov rcx, rsi
1405a3ea1: add rcx, 8
1405a3ea5: mov rax, qword ptr [rsi + 8]
1405a3ea9: mov rdx, qword ptr [rsi + 0x10]
1405a3ead: movzx r13d, byte ptr [rbx + 0x107]
1405a3eb5: jmp 0x1405a3ed9
1405a3eb7: call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1405a3ebc: mov r13b, byte ptr [rbx + 0x107]
1405a3ec3: mov rcx, rsi
1405a3ec6: add rcx, 8
1405a3eca: mov qword ptr [rsi], 1
1405a3ed1: mov qword ptr [rsi + 8], rax
1405a3ed5: mov qword ptr [rsi + 0x10], rdx
1405a3ed9: lea r8, [rax + 1]
1405a3edd: mov qword ptr [rcx], r8
1405a3ee0: movaps xmm0, xmmword ptr [rbx + 0xb50]
1405a3ee7: movups xmmword ptr [rbx + 0x21f0], xmm0
1405a3eee: movaps xmm0, xmmword ptr [rbx + 0xb40]
1405a3ef5: movups xmmword ptr [rbx + 0x21e0], xmm0
1405a3efc: movaps xmm0, xmmword ptr [rbx + 0xb00]
1405a3f03: movaps xmm1, xmmword ptr [rbx + 0xb10]
1405a3f0a: movaps xmm2, xmmword ptr [rbx + 0xb20]
1405a3f11: movaps xmm3, xmmword ptr [rbx + 0xb30]
1405a3f18: movups xmmword ptr [rbx + 0x21d0], xmm3
1405a3f1f: movups xmmword ptr [rbx + 0x21c0], xmm2
1405a3f26: movups xmmword ptr [rbx + 0x21b0], xmm1
1405a3f2d: movups xmmword ptr [rbx + 0x21a0], xmm0
1405a3f34: movups xmmword ptr [rbx + 0x2200], xmm7
1405a3f3b: movups xmmword ptr [rbx + 0x2210], xmm6
1405a3f42: mov qword ptr [rbx + 0x2220], rax
1405a3f49: mov qword ptr [rbx + 0x2228], rdx
1405a3f50: mov qword ptr [rbx + 0x2188], 0
1405a3f5b: mov qword ptr [rbx + 0x2190], 8
1405a3f66: mov qword ptr [rbx + 0x2198], 0
1405a3f71: movabs rax, 0x8000000000000000
1405a3f7b: mov qword ptr [rbx + 0xb10], rax
1405a3f82: mov qword ptr [rbx + 0xb00], 0
1405a3f8d: mov byte ptr [rbx + 0xb28], 0
1405a3f94: movzx eax, byte ptr [rip + 0x3090c6e]
1405a3f9b: mov ecx, 0x12
1405a3fa0: mov edx, 1
1405a3fa5: call 0x140613c10 ; 
1405a3faa: test rax, rax
1405a3fad: mov qword ptr [rbx + 0x128], rax
1405a3fb4: je 0x1405a6e02
1405a3fba: movups xmm0, xmmword ptr [rip + 0x245bb2f]
1405a3fc1: movups xmmword ptr [rax], xmm0
1405a3fc4: mov word ptr [rax + 0x10], 0x7265
1405a3fca: mov qword ptr [rbx + 0x2590], 0x12
1405a3fd5: mov qword ptr [rbx + 0x2598], rax
1405a3fdc: mov qword ptr [rbx + 0x25a0], 0x12
1405a3fe7: lea rcx, [rbx + 0x440]
1405a3fee: lea rdx, [rbx + 0xb00]
1405a3ff5: lea r14, [rbx + 0x2590]
1405a3ffc: mov r8, r14
1405a3fff: call 0x140d558f0 ; std::thread::Builder::name
1405a4004: mov r8, qword ptr [rbx + 0x158]
1405a400b: mov byte ptr [rbx + 0xd4], 1
1405a4012: mov byte ptr [rbx + 0xd5], 1
1405a4019: mov byte ptr [rbx + 0xd6], 1
1405a4020: mov byte ptr [rbx + 0xd7], 0
1405a4027: mov byte ptr [rbx + 0xd8], 1
1405a402e: lea rcx, [rbx + 0xb00]
1405a4035: lea rdx, [rbx + 0x440]
1405a403c: call 0x14028ae90 ; std::thread::Builder::spawn<tiny_glade::run_game::run_game_thread::closure_env$7,tuple$<> >
1405a4041: cmp qword ptr [rbx + 0xb00], 0
1405a4049: je 0x1405a6e1d
1405a404f: mov rax, qword ptr [rbx + 0xb10]
1405a4056: mov qword ptr [rbx + 0x2a0], rax
1405a405d: movups xmm0, xmmword ptr [rbx + 0xb00]
1405a4064: movaps xmmword ptr [rbx + 0x290], xmm0
1405a406b: mov rax, qword ptr [rbp + 0x37c0]
1405a4072: cmp byte ptr [rax + 0x14a], 0
1405a4079: je 0x1405a409c
1405a407b: mov byte ptr [rbx + 0xe4], 1
1405a4082: mov byte ptr [rbx + 0xe5], 1
1405a4089: mov byte ptr [rbx + 0xe6], 1
1405a4090: mov byte ptr [rbx + 0xe7], 1
1405a4097: call 0x140365be0 ; tiny_glade::hang_watchdog::spawn_hang_watchdog_thread
1405a409c: mov qword ptr [rbx + 0xc80], 1
1405a40a7: mov qword ptr [rbx + 0xc88], 1
1405a40b2: mov byte ptr [rbx + 0xc90], 0
1405a40b9: xorps xmm0, xmm0
1405a40bc: movaps xmmword ptr [rbx + 0xb00], xmm0
1405a40c3: movaps xmmword ptr [rbx + 0xb80], xmm0
1405a40ca: mov word ptr [rbx + 0xc00], 0
1405a40d3: mov qword ptr [rbx + 0xc08], 0
1405a40de: mov qword ptr [rbx + 0xc10], 8
1405a40e9: movups xmmword ptr [rbx + 0xc18], xmm0
1405a40f0: mov qword ptr [rbx + 0xc28], 8
1405a40fb: mov qword ptr [rbx + 0xc30], 0
1405a4106: mov byte ptr [rbx + 0xc38], 1
1405a410d: movzx eax, byte ptr [rip + 0x3090af5]
1405a4114: mov ecx, 0x200
1405a4119: mov edx, 0x80
1405a411e: call 0x140613c10 ; 
1405a4123: test rax, rax
1405a4126: je 0x1405a6e5b
1405a412c: mov rsi, rax
1405a412f: lea rdx, [rbx + 0xb00]
1405a4136: mov r8d, 0x200
1405a413c: mov rcx, rax
1405a413f: call 0x142923c00 ; 
1405a4144: mov qword ptr [rbx + 0x2e0], 1
1405a414f: mov qword ptr [rbx + 0x2e8], rsi
1405a4156: mov qword ptr [rbx + 0x2f0], 1
1405a4161: mov qword ptr [rbx + 0x2f8], rsi
1405a4168: test r13b, r13b
1405a416b: je 0x1405a43f1
1405a4171: mov byte ptr [rbx + 0xc2], 1
1405a4178: mov byte ptr [rbx + 0xc3], 1
1405a417f: mov byte ptr [rbx + 0xc4], 1
1405a4186: mov byte ptr [rbx + 0xc5], 1
1405a418d: mov byte ptr [rbx + 0xc6], 1
1405a4194: movabs rdx, 0xaed7b5777ef68ba3
1405a419e: movabs r8, 0x5834a199b0692b72
1405a41a8: mov rcx, qword ptr [rbx + 0xe8]
1405a41af: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a41b4: test rax, rax
1405a41b7: mov rcx, qword ptr [rbx + 0xc8]
1405a41be: je 0x1405a65f1
1405a41c4: cmp qword ptr [rcx + 0x218], rdx
1405a41cb: jbe 0x1405a65f1
1405a41d1: mov rax, qword ptr [rcx + 0x210]
1405a41d8: mov rax, qword ptr [rax + rdx*8]
1405a41dc: test rax, rax
1405a41df: je 0x1405a65f1
1405a41e5: mov rcx, qword ptr [rcx + 0x1e0]
1405a41ec: not rax
1405a41ef: lea rax, [rax + rax*2]
1405a41f3: shl rax, 5
1405a41f7: cmp qword ptr [rcx + rax + 0x20], 0
1405a41fd: je 0x1405a65f1
1405a4203: add rcx, rax
1405a4206: mov byte ptr [rbx + 0xc2], 1
1405a420d: mov byte ptr [rbx + 0xc3], 1
1405a4214: mov byte ptr [rbx + 0xc4], 1
1405a421b: mov byte ptr [rbx + 0xc5], 1
1405a4222: mov byte ptr [rbx + 0xc6], 1
1405a4229: xor edx, edx
1405a422b: call 0x140cff710 ; bevy_ecs::storage::blob_vec::BlobVec::swap_remove_and_forget_unchecked
1405a4230: mov rcx, qword ptr [rax]
1405a4233: mov byte ptr [rbx + 0xc2], 1
1405a423a: mov byte ptr [rbx + 0xc3], 1
1405a4241: mov byte ptr [rbx + 0xc4], 1
1405a4248: mov byte ptr [rbx + 0xc5], 1
1405a424f: mov byte ptr [rbx + 0xc6], 1
1405a4256: call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
1405a425b: mov byte ptr [rbx + 0xc2], 1
1405a4262: mov byte ptr [rbx + 0xc3], 1
1405a4269: mov byte ptr [rbx + 0xc4], 1
1405a4270: mov byte ptr [rbx + 0xc5], 1
1405a4277: mov byte ptr [rbx + 0xc6], 1
1405a427e: mov rcx, qword ptr [rbx + 0xc8]
1405a4285: mov rdx, rax
1405a4288: call 0x14013c9f0 ; bevy_app::app::App::insert_resource<country_core::resources::gui::EguiContextResource>
1405a428d: mov byte ptr [rbx + 0xc2], 1
1405a4294: mov byte ptr [rbx + 0xc3], 1
1405a429b: mov byte ptr [rbx + 0xc4], 1
1405a42a2: mov byte ptr [rbx + 0xc5], 1
1405a42a9: mov byte ptr [rbx + 0xc6], 1
1405a42b0: movabs rdx, 0xd8f93d064ea9d61c
1405a42ba: movabs r8, 0x6447617066d9832f
1405a42c4: mov rcx, qword ptr [rbx + 0xe8]
1405a42cb: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a42d0: test rax, rax
1405a42d3: mov r8, qword ptr [rbx + 0xc8]
1405a42da: je 0x1405a66af
1405a42e0: cmp qword ptr [r8 + 0x218], rdx
1405a42e7: jbe 0x1405a66af
1405a42ed: mov rax, qword ptr [r8 + 0x210]
1405a42f4: mov rcx, qword ptr [rax + rdx*8]
1405a42f8: test rcx, rcx
1405a42fb: je 0x1405a66af
1405a4301: mov rax, qword ptr [r8 + 0x1e0]
1405a4308: not rcx
1405a430b: lea rcx, [rcx + rcx*2]
1405a430f: shl rcx, 5
1405a4313: cmp qword ptr [rax + rcx + 0x20], 0
1405a4319: je 0x1405a66af
1405a431f: add rax, rcx
1405a4322: mov rsi, qword ptr [rax + 0x10]
1405a4326: mov ecx, dword ptr [r8 + 0x5b0]
1405a432d: mov dword ptr [rax + 0x5c], ecx
1405a4330: mov rdi, qword ptr [rsi + 0x30]
1405a4334: mov r15, qword ptr [rsi + 0x40]
1405a4338: cmp rdi, qword ptr [rsi + 0x20]
1405a433c: jne 0x1405a4371
1405a433e: lea rcx, [rsi + 0x20]
1405a4342: mov byte ptr [rbx + 0xc2], 1
1405a4349: mov byte ptr [rbx + 0xc3], 1
1405a4350: mov byte ptr [rbx + 0xc4], 1
1405a4357: mov byte ptr [rbx + 0xc5], 1
1405a435e: mov byte ptr [rbx + 0xc6], 1
1405a4365: lea rdx, [rip + 0x244edbc]
1405a436c: call 0x140562870 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::glade_border::ShowDemoBorderOverlay>,alloc::alloc::Global>
1405a4371: mov rax, qword ptr [rsi + 0x28]
1405a4375: mov qword ptr [rax + rdi*8], r15
1405a4379: inc rdi
1405a437c: mov qword ptr [rsi + 0x30], rdi
1405a4380: inc qword ptr [rsi + 0x40]
1405a4384: lea rcx, [rbx + 0x440]
1405a438b: mov r8d, 0x658
1405a4391: mov rdx, qword ptr [rbx + 0xc8]
1405a4398: call 0x142923c00 ; 
1405a439d: movups xmm0, xmmword ptr [rbx + 0x2f0]
1405a43a4: movups xmmword ptr [rbx + 0x280], xmm0
1405a43ab: movups xmm0, xmmword ptr [rbx + 0x328]
1405a43b2: movups xmmword ptr [rbx + 0x24e8], xmm0
1405a43b9: mov rax, qword ptr [rbx + 0x160]
1405a43c0: cmp rax, 2
1405a43c4: je 0x1405a5fc5
1405a43ca: cmp eax, 1
1405a43cd: jne 0x1405a5fdc
1405a43d3: mov rcx, qword ptr [rbx + 0x168]
1405a43da: lock inc qword ptr [rcx + 0x180]
1405a43e2: jg 0x1405a5ff1
1405a43e8: mov ecx, 7
1405a43ed: int 0x29
1405a43ef: ud2 
1405a43f1: mov r15, qword ptr [rbx + 0x138]
1405a43f8: lock inc qword ptr [r15]
1405a43fc: jle 0x1405a7040
1405a4402: mov qword ptr [rbx + 0x2230], r15
1405a4409: mov byte ptr [rbx + 0x11f], 1
1405a4410: lea rcx, [rbx + 0x440]
1405a4417: call 0x140cad810 ; bevy_app::app::App::new
1405a441c: mov byte ptr [rbx + 0xb3], 1
1405a4423: lea rcx, [rbx + 0x440]
1405a442a: call 0x140945ed0 ; country_core::systems::utils::impl$0::add_stages
1405a442f: mov byte ptr [rbx + 0xb3], 1
1405a4436: movabs rdx, 0x864ddca98c2312a6
1405a4440: movabs r8, 0x27b0e9369e9a484e
1405a444a: mov rcx, qword ptr [rbx + 0xe8]
1405a4451: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a4456: test rax, rax
1405a4459: mov r8, qword ptr [rbx + 0xc8]
1405a4460: je 0x1405a6625
1405a4466: cmp qword ptr [r8 + 0x218], rdx
1405a446d: jbe 0x1405a6625
1405a4473: mov rax, qword ptr [r8 + 0x210]
1405a447a: mov rcx, qword ptr [rax + rdx*8]
1405a447e: test rcx, rcx
1405a4481: je 0x1405a6625
1405a4487: mov rax, qword ptr [r8 + 0x1e0]
1405a448e: not rcx
1405a4491: lea rcx, [rcx + rcx*2]
1405a4495: shl rcx, 5
1405a4499: cmp qword ptr [rax + rcx + 0x20], 0
1405a449f: je 0x1405a6625
1405a44a5: add rax, rcx
1405a44a8: mov rax, qword ptr [rax + 0x10]
1405a44ac: mov r12d, dword ptr [rax]
1405a44af: mov r13d, dword ptr [rax + 4]
1405a44b3: mov byte ptr [rbx + 0xb3], 1
1405a44ba: lea rcx, [rbx + 0x440]
1405a44c1: call 0x140113ff0 ; bevy_app::app::App::plugins_state
1405a44c6: cmp al, 1
1405a44c8: ja 0x1405a6f4e
1405a44ce: mov byte ptr [rbx + 0xb3], 1
1405a44d5: lea rdx, [rip + 0x245b6a4]
1405a44dc: lea rcx, [rbx + 0x440]
1405a44e3: call 0x14032b3f0 ; bevy_app::plugin::sealed::impl$5::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::run_game::InputLitePlugin,bevy_app::plugin::sealed::PluginMarker,bevy_time::TimePlugin,bevy_app::plugin::sealed::PluginMarker,bevy_state::app::StatesPlugin>
1405a44e8: mov byte ptr [rbx + 0xb3], 1
1405a44ef: lea rcx, [rbx + 0x440]
1405a44f6: call 0x140113ff0 ; bevy_app::app::App::plugins_state
1405a44fb: cmp al, 1
1405a44fd: ja 0x1405a6f9b
1405a4503: mov byte ptr [rbx + 0xb3], 1
1405a450a: lea r9, [rip + 0x245b687]
1405a4511: lea r8, [rbx + 0x440]
1405a4518: mov cl, 1
1405a451a: mov dl, 1
1405a451c: call 0x1400cd450 ; bevy_app::plugin::sealed::impl$9::add_to_app<bevy_app::plugin::sealed::PluginMarker,country_core::startup::MinimalStartupPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::gui::settings::SettingsMenuPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::logo_animation::AnimatedLogo,bevy_app::plugin::sealed::PluginMarker,country_core::resources::loading_screen::LoadingScreen,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::vermeer::VermeerPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::mei::core::MeiPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::menu::SplashScreenPlugin>
1405a4521: lea rdi, [rbx + 0x478]
1405a4528: mov byte ptr [rbx + 0xb3], 1
1405a452f: movabs rdx, 0xbbdd82cfacba045f
1405a4539: movabs r8, 0xcaf27ba849498836
1405a4543: mov rcx, rdi
1405a4546: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a454b: test rax, rax
1405a454e: je 0x1405a4583
1405a4550: cmp qword ptr [rbx + 0x658], rdx
1405a4557: jbe 0x1405a4583
1405a4559: mov rax, qword ptr [rbx + 0x650]
1405a4560: mov rax, qword ptr [rax + rdx*8]
1405a4564: test rax, rax
1405a4567: je 0x1405a4583
1405a4569: mov rcx, qword ptr [rbx + 0x620]
1405a4570: not rax
1405a4573: lea rax, [rax + rax*2]
1405a4577: shl rax, 5
1405a457b: cmp qword ptr [rcx + rax + 0x20], 0
1405a4581: jne 0x1405a4596
1405a4583: mov byte ptr [rbx + 0xb3], 1
1405a458a: lea rcx, [rbx + 0x440]
1405a4591: call 0x140566280 ; bevy_ecs::event::registry::EventRegistry::register_event<enum2$<country_core::resources::mouse_capture::OsCursorCaptureCmd> >
1405a4596: mov byte ptr [rbx + 0xb3], 1
1405a459d: movabs rdx, 0x1c5d03917730cc4f
1405a45a7: movabs r8, 0x3d29e4fb18bada6c
1405a45b1: mov rcx, rdi
1405a45b4: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a45b9: test rax, rax
1405a45bc: je 0x1405a45f1
1405a45be: cmp qword ptr [rbx + 0x658], rdx
1405a45c5: jbe 0x1405a45f1
1405a45c7: mov rax, qword ptr [rbx + 0x650]
1405a45ce: mov rax, qword ptr [rax + rdx*8]
1405a45d2: test rax, rax
1405a45d5: je 0x1405a45f1
1405a45d7: mov rcx, qword ptr [rbx + 0x620]
1405a45de: not rax
1405a45e1: lea rax, [rax + rax*2]
1405a45e5: shl rax, 5
1405a45e9: cmp qword ptr [rcx + rax + 0x20], 0
1405a45ef: jne 0x1405a4604
1405a45f1: mov byte ptr [rbx + 0xb3], 1
1405a45f8: lea rcx, [rbx + 0x440]
1405a45ff: call 0x14057e850 ; bevy_ecs::event::registry::EventRegistry::register_event<enum2$<country_core::resources::mouse_capture::OsCursorVisibilityCmd> >
1405a4604: mov byte ptr [rbx + 0xb3], 1
1405a460b: movabs rdx, 0x6ecb4684a4c9e1f9
1405a4615: movabs r8, 0xfae2e21abac81ddc
1405a461f: mov rcx, rdi
1405a4622: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a4627: test rax, rax
1405a462a: je 0x1405a465f
1405a462c: cmp qword ptr [rbx + 0x658], rdx
1405a4633: jbe 0x1405a465f
1405a4635: mov rax, qword ptr [rbx + 0x650]
1405a463c: mov rax, qword ptr [rax + rdx*8]
1405a4640: test rax, rax
1405a4643: je 0x1405a465f
1405a4645: mov rcx, qword ptr [rbx + 0x620]
1405a464c: not rax
1405a464f: lea rax, [rax + rax*2]
1405a4653: shl rax, 5
1405a4657: cmp qword ptr [rcx + rax + 0x20], 0
1405a465d: jne 0x1405a4672
1405a465f: mov byte ptr [rbx + 0xb3], 1
1405a4666: lea rcx, [rbx + 0x440]
1405a466d: call 0x140575a00 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>
1405a4672: mov byte ptr [rbx + 0xb3], 1
1405a4679: movabs rdx, 0xdac2c590678dd6c0
1405a4683: movabs r8, 0x5d30eb6733ae4d59
1405a468d: mov rcx, rdi
1405a4690: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a4695: test rax, rax
1405a4698: je 0x1405a46cd
1405a469a: cmp qword ptr [rbx + 0x658], rdx
1405a46a1: jbe 0x1405a46cd
1405a46a3: mov rax, qword ptr [rbx + 0x650]
1405a46aa: mov rax, qword ptr [rax + rdx*8]
1405a46ae: test rax, rax
1405a46b1: je 0x1405a46cd
1405a46b3: mov rcx, qword ptr [rbx + 0x620]
1405a46ba: not rax
1405a46bd: lea rax, [rax + rax*2]
1405a46c1: shl rax, 5
1405a46c5: cmp qword ptr [rcx + rax + 0x20], 0
1405a46cb: jne 0x1405a46e0
1405a46cd: mov byte ptr [rbx + 0xb3], 1
1405a46d4: lea rcx, [rbx + 0x440]
1405a46db: call 0x140584520 ; bevy_ecs::event::registry::EventRegistry::register_event<enum2$<country_core::resources::window::OsWindowCmd> >
1405a46e0: mov byte ptr [rbx + 0xb3], 1
1405a46e7: movabs rdx, 0xa554fbb3bbf1ca0
1405a46f1: movabs r8, 0x29b993a986bb8e9d
1405a46fb: mov rcx, rdi
1405a46fe: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a4703: test rax, rax
1405a4706: je 0x1405a473b
1405a4708: cmp qword ptr [rbx + 0x658], rdx
1405a470f: jbe 0x1405a473b
1405a4711: mov rax, qword ptr [rbx + 0x650]
1405a4718: mov rax, qword ptr [rax + rdx*8]
1405a471c: test rax, rax
1405a471f: je 0x1405a473b
1405a4721: mov rcx, qword ptr [rbx + 0x620]
1405a4728: not rax
1405a472b: lea rax, [rax + rax*2]
1405a472f: shl rax, 5
1405a4733: cmp qword ptr [rcx + rax + 0x20], 0
1405a4739: jne 0x1405a474e
1405a473b: mov byte ptr [rbx + 0xb3], 1
1405a4742: lea rcx, [rbx + 0x440]
1405a4749: call 0x1405797e0 ; bevy_ecs::event::registry::EventRegistry::register_event<tiny_glade::app_config::WriteSettingsFileCmd>
1405a474e: mov byte ptr [rbx + 0xb3], 1
1405a4755: movabs rdx, 0xbb7c1673aee764cb
1405a475f: movabs r8, 0x250aee383945e2d6
1405a4769: mov rcx, rdi
1405a476c: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a4771: test rax, rax
1405a4774: je 0x1405a47a9
1405a4776: cmp qword ptr [rbx + 0x658], rdx
1405a477d: jbe 0x1405a47a9
1405a477f: mov rax, qword ptr [rbx + 0x650]
1405a4786: mov rax, qword ptr [rax + rdx*8]
1405a478a: test rax, rax
1405a478d: je 0x1405a47a9
1405a478f: mov rcx, qword ptr [rbx + 0x620]
1405a4796: not rax
1405a4799: lea rax, [rax + rax*2]
1405a479d: shl rax, 5
1405a47a1: cmp qword ptr [rcx + rax + 0x20], 0
1405a47a7: jne 0x1405a47bc
1405a47a9: mov byte ptr [rbx + 0xb3], 1
1405a47b0: lea rcx, [rbx + 0x440]
1405a47b7: call 0x1405687a0 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::resources::window::WindowFocusEvent>
1405a47bc: mov byte ptr [rbx + 0xb3], 1
1405a47c3: movabs rdx, 0x825d6aa59c91551
1405a47cd: movabs r8, 0xe073afcfe6bed5b6
1405a47d7: mov rcx, rdi
1405a47da: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a47df: test rax, rax
1405a47e2: je 0x1405a4817
1405a47e4: cmp qword ptr [rbx + 0x658], rdx
1405a47eb: jbe 0x1405a4817
1405a47ed: mov rax, qword ptr [rbx + 0x650]
1405a47f4: mov rax, qword ptr [rax + rdx*8]
1405a47f8: test rax, rax
1405a47fb: je 0x1405a4817
1405a47fd: mov rcx, qword ptr [rbx + 0x620]
1405a4804: not rax
1405a4807: lea rax, [rax + rax*2]
1405a480b: shl rax, 5
1405a480f: cmp qword ptr [rcx + rax + 0x20], 0
1405a4815: jne 0x1405a482a
1405a4817: mov byte ptr [rbx + 0xb3], 1
1405a481e: lea rcx, [rbx + 0x440]
1405a4825: call 0x1405778f0 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>
1405a482a: mov byte ptr [rbx + 0xb3], 1
1405a4831: movabs rdx, 0x923f1ac331d52636
1405a483b: movabs r8, 0x4114c9eaff932cf6
1405a4845: mov rcx, rdi
1405a4848: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a484d: test rax, rax
1405a4850: je 0x1405a4885
1405a4852: cmp qword ptr [rbx + 0x658], rdx
1405a4859: jbe 0x1405a4885
1405a485b: mov rax, qword ptr [rbx + 0x650]
1405a4862: mov rax, qword ptr [rax + rdx*8]
1405a4866: test rax, rax
1405a4869: je 0x1405a4885
1405a486b: mov rcx, qword ptr [rbx + 0x620]
1405a4872: not rax
1405a4875: lea rax, [rax + rax*2]
1405a4879: shl rax, 5
1405a487d: cmp qword ptr [rcx + rax + 0x20], 0
1405a4883: jne 0x1405a4898
1405a4885: mov byte ptr [rbx + 0xb3], 1
1405a488c: lea rcx, [rbx + 0x440]
1405a4893: call 0x140572250 ; bevy_ecs::event::registry::EventRegistry::register_event<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >
1405a4898: mov byte ptr [rbx + 0xb3], 1
1405a489f: movabs rdx, 0x2e383ebb9376bfa1
1405a48a9: movabs r8, 0x96b847e2c17c833c
1405a48b3: mov rcx, rdi
1405a48b6: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a48bb: test rax, rax
1405a48be: je 0x1405a48f3
1405a48c0: cmp qword ptr [rbx + 0x658], rdx
1405a48c7: jbe 0x1405a48f3
1405a48c9: mov rax, qword ptr [rbx + 0x650]
1405a48d0: mov rax, qword ptr [rax + rdx*8]
1405a48d4: test rax, rax
1405a48d7: je 0x1405a48f3
1405a48d9: mov rcx, qword ptr [rbx + 0x620]
1405a48e0: not rax
1405a48e3: lea rax, [rax + rax*2]
1405a48e7: shl rax, 5
1405a48eb: cmp qword ptr [rcx + rax + 0x20], 0
1405a48f1: jne 0x1405a4906
1405a48f3: mov byte ptr [rbx + 0xb3], 1
1405a48fa: lea rcx, [rbx + 0x440]
1405a4901: call 0x140566ee0 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::resources::events::ShutdownGameCmd>
1405a4906: mov byte ptr [rbx + 0xb3], 1
1405a490d: movabs rdx, 0x9ec27b7f9851aea
1405a4917: movabs r8, 0x1e73f089b9e6883e
1405a4921: mov rcx, rdi
1405a4924: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a4929: test rax, rax
1405a492c: je 0x1405a4961
1405a492e: cmp qword ptr [rbx + 0x658], rdx
1405a4935: jbe 0x1405a4961
1405a4937: mov rax, qword ptr [rbx + 0x650]
1405a493e: mov rax, qword ptr [rax + rdx*8]
1405a4942: test rax, rax
1405a4945: je 0x1405a4961
1405a4947: mov rcx, qword ptr [rbx + 0x620]
1405a494e: not rax
1405a4951: lea rax, [rax + rax*2]
1405a4955: shl rax, 5
1405a4959: cmp qword ptr [rcx + rax + 0x20], 0
1405a495f: jne 0x1405a4974
1405a4961: mov byte ptr [rbx + 0xb3], 1
1405a4968: lea rcx, [rbx + 0x440]
1405a496f: call 0x1405791b0 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::systems::backend::icc::IccProfileChanged>
1405a4974: mov byte ptr [rbx + 0xb3], 1
1405a497b: movabs rdx, 0xfc533cfe7fb82312
1405a4985: movabs r8, 0x2e13a6a03dedb277
1405a498f: mov rcx, rdi
1405a4992: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a4997: test rax, rax
1405a499a: je 0x1405a49cf
1405a499c: cmp qword ptr [rbx + 0x658], rdx
1405a49a3: jbe 0x1405a49cf
1405a49a5: mov rax, qword ptr [rbx + 0x650]
1405a49ac: mov rax, qword ptr [rax + rdx*8]
1405a49b0: test rax, rax
1405a49b3: je 0x1405a49cf
1405a49b5: mov rcx, qword ptr [rbx + 0x620]
1405a49bc: not rax
1405a49bf: lea rax, [rax + rax*2]
1405a49c3: shl rax, 5
1405a49c7: cmp qword ptr [rcx + rax + 0x20], 0
1405a49cd: jne 0x1405a49e2
1405a49cf: mov byte ptr [rbx + 0xb3], 1
1405a49d6: lea rcx, [rbx + 0x440]
1405a49dd: call 0x140570360 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::systems::audio::menu_audio::PlayMenuUiSound>
1405a49e2: mov byte ptr [rbx + 0xb3], 1
1405a49e9: movabs rdx, 0xa4c02661e67b048b
1405a49f3: movabs r8, 0xeae761e6aa2a67ec
1405a49fd: mov rcx, qword ptr [rbx + 0xe8]
1405a4a04: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a4a09: test rax, rax
1405a4a0c: mov r8, qword ptr [rbx + 0xc8]
1405a4a13: je 0x1405a66e3
1405a4a19: cmp qword ptr [r8 + 0x218], rdx
1405a4a20: jbe 0x1405a66e3
1405a4a26: mov rax, qword ptr [r8 + 0x210]
1405a4a2d: mov rcx, qword ptr [rax + rdx*8]
1405a4a31: test rcx, rcx
1405a4a34: je 0x1405a66e3
1405a4a3a: mov rax, qword ptr [r8 + 0x1e0]
1405a4a41: not rcx
1405a4a44: lea rcx, [rcx + rcx*2]
1405a4a48: shl rcx, 5
1405a4a4c: cmp qword ptr [rax + rcx + 0x20], 0
1405a4a52: je 0x1405a66e3
1405a4a58: add rax, rcx
1405a4a5b: mov rax, qword ptr [rax + 0x10]
1405a4a5f: mov rcx, qword ptr [rax + 0x10]
1405a4a63: movzx edx, byte ptr [rax + 0x20]
1405a4a67: movups xmm0, xmmword ptr [rax]
1405a4a6a: movaps xmmword ptr [rbx + 0xb00], xmm0
1405a4a71: mov qword ptr [rbx + 0xb10], rcx
1405a4a78: mov byte ptr [rbx + 0xb20], dl
1405a4a7e: movsd xmm0, qword ptr [rax + 0x18]
1405a4a83: movsd qword ptr [rbx + 0xb18], xmm0
1405a4a8b: mov byte ptr [rbx + 0xb3], 1
1405a4a92: lea rcx, [rbx + 0x440]
1405a4a99: lea rdx, [rbx + 0xb00]
1405a4aa0: call 0x14012e420 ; bevy_app::app::App::insert_resource<country_core::render::render_config::FrameRenderConfig>
1405a4aa5: mov rsi, rax
1405a4aa8: mov byte ptr [rbx + 0xb3], 1
1405a4aaf: movabs rdx, 0x3335386a5fbcee80
1405a4ab9: movabs r8, 0x8f2a59f31a65ec60
1405a4ac3: mov rcx, qword ptr [rbx + 0xe8]
1405a4aca: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a4acf: test rax, rax
1405a4ad2: mov r8, qword ptr [rbx + 0xc8]
1405a4ad9: je 0x1405a676d
1405a4adf: cmp qword ptr [r8 + 0x218], rdx
1405a4ae6: jbe 0x1405a676d
1405a4aec: mov rax, qword ptr [r8 + 0x210]
1405a4af3: mov rcx, qword ptr [rax + rdx*8]
1405a4af7: test rcx, rcx
1405a4afa: je 0x1405a676d
1405a4b00: mov rax, qword ptr [r8 + 0x1e0]
1405a4b07: not rcx
1405a4b0a: lea rcx, [rcx + rcx*2]
1405a4b0e: shl rcx, 5
1405a4b12: cmp qword ptr [rax + rcx + 0x20], 0
1405a4b18: je 0x1405a676d
1405a4b1e: add rax, rcx
1405a4b21: mov rax, qword ptr [rax + 0x10]
1405a4b25: movzx ecx, byte ptr [rax + 0x28]
1405a4b29: mov byte ptr [rbx + 0xb28], cl
1405a4b2f: movups xmm0, xmmword ptr [rax]
1405a4b32: movups xmm1, xmmword ptr [rax + 0x10]
1405a4b36: movaps xmmword ptr [rbx + 0xb00], xmm0
1405a4b3d: movaps xmmword ptr [rbx + 0xb10], xmm1
1405a4b44: mov rax, qword ptr [rax + 0x20]
1405a4b48: mov qword ptr [rbx + 0xb20], rax
1405a4b4f: mov byte ptr [rbx + 0xb3], 1
1405a4b56: lea rdx, [rbx + 0xb00]
1405a4b5d: mov rcx, rsi
1405a4b60: call 0x14013d5f0 ; bevy_app::app::App::insert_resource<country_core::render::adaptive_render_state::AdaptiveRenderState>
1405a4b65: mov byte ptr [rbx + 0xb3], 1
1405a4b6c: mov rcx, rax
1405a4b6f: mov edx, r12d
1405a4b72: mov r8d, r13d
1405a4b75: call 0x140120460 ; bevy_app::app::App::insert_resource<country_core::resources::window::WindowSize>
1405a4b7a: mov rsi, rax
1405a4b7d: mov byte ptr [rbx + 0xb3], 1
1405a4b84: lea rcx, [rbx + 0xb00]
1405a4b8b: call 0x140b64d00 ; country_core::resources::time::impl$0::default
1405a4b90: mov byte ptr [rbx + 0xb3], 1
1405a4b97: lea rdx, [rbx + 0xb00]
1405a4b9e: mov rcx, rsi
1405a4ba1: call 0x14012c3c0 ; bevy_app::app::App::insert_resource<country_core::resources::time::GameTime>
1405a4ba6: mov rsi, rax
1405a4ba9: mov byte ptr [rbx + 0xb3], 1
1405a4bb0: movabs rdx, 0x3d7b56e787d47c40
1405a4bba: movabs r8, 0x67b0b10a8d43bfc7
1405a4bc4: mov rcx, qword ptr [rbx + 0xe8]
1405a4bcb: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a4bd0: test rax, rax
1405a4bd3: mov r8, qword ptr [rbx + 0xc8]
1405a4bda: je 0x1405a67f7
1405a4be0: cmp qword ptr [r8 + 0x218], rdx
1405a4be7: jbe 0x1405a67f7
1405a4bed: mov rax, qword ptr [r8 + 0x210]
1405a4bf4: mov rcx, qword ptr [rax + rdx*8]
1405a4bf8: test rcx, rcx
1405a4bfb: je 0x1405a67f7
1405a4c01: mov rax, qword ptr [r8 + 0x1e0]
1405a4c08: not rcx
1405a4c0b: lea rcx, [rcx + rcx*2]
1405a4c0f: shl rcx, 5
1405a4c13: cmp qword ptr [rax + rcx + 0x20], 0
1405a4c19: je 0x1405a67f7
1405a4c1f: add rax, rcx
1405a4c22: mov rdx, qword ptr [rax + 0x10]
1405a4c26: mov byte ptr [rbx + 0xb3], 1
1405a4c2d: lea rcx, [rbx + 0xb00]
1405a4c34: call 0x140593510 ; country_core::game_config::impl$11::clone
1405a4c39: mov byte ptr [rbx + 0xb3], 1
1405a4c40: lea rdx, [rbx + 0xb00]
1405a4c47: mov rcx, rsi
1405a4c4a: call 0x140119810 ; bevy_app::app::App::insert_resource<country_core::game_config::GameConfig>
1405a4c4f: mov rsi, rax
1405a4c52: mov byte ptr [rbx + 0xb3], 1
1405a4c59: movabs rdx, 0xfdf985a9f052d51c
1405a4c63: movabs r8, 0xd7c2d9b5621db8c6
1405a4c6d: mov rcx, qword ptr [rbx + 0xe8]
1405a4c74: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a4c79: test rax, rax
1405a4c7c: mov r8, qword ptr [rbx + 0xc8]
1405a4c83: je 0x1405a6881
1405a4c89: cmp qword ptr [r8 + 0x218], rdx
1405a4c90: jbe 0x1405a6881
1405a4c96: mov rax, qword ptr [r8 + 0x210]
1405a4c9d: mov rcx, qword ptr [rax + rdx*8]
1405a4ca1: test rcx, rcx
1405a4ca4: je 0x1405a6881
1405a4caa: mov rax, qword ptr [r8 + 0x1e0]
1405a4cb1: not rcx
1405a4cb4: lea rcx, [rcx + rcx*2]
1405a4cb8: shl rcx, 5
1405a4cbc: cmp qword ptr [rax + rcx + 0x20], 0
1405a4cc2: je 0x1405a6881
1405a4cc8: add rax, rcx
1405a4ccb: mov rax, qword ptr [rax + 0x10]
1405a4ccf: mov rdx, qword ptr [rax]
1405a4cd2: test rdx, rdx
1405a4cd5: je 0x1405a4ce1
1405a4cd7: lock inc qword ptr [rdx]
1405a4cdb: jle 0x1405a7040
1405a4ce1: mov byte ptr [rbx + 0xb3], 1
1405a4ce8: mov rcx, rsi
1405a4ceb: call 0x140114e10 ; bevy_app::app::App::insert_resource<country_core::resources::audio_resources::AudioCore>
1405a4cf0: mov rsi, rax
1405a4cf3: mov byte ptr [rbx + 0xb3], 1
1405a4cfa: movabs rdx, 0x8b8bde009a4ee44
1405a4d04: movabs r8, 0x2ad7572b7bbdf9dd
1405a4d0e: mov rcx, qword ptr [rbx + 0xe8]
1405a4d15: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a4d1a: test rax, rax
1405a4d1d: mov r8, qword ptr [rbx + 0xc8]
1405a4d24: je 0x1405a690b
1405a4d2a: cmp qword ptr [r8 + 0x218], rdx
1405a4d31: jbe 0x1405a690b
1405a4d37: mov rax, qword ptr [r8 + 0x210]
1405a4d3e: mov rcx, qword ptr [rax + rdx*8]
1405a4d42: test rcx, rcx
1405a4d45: je 0x1405a690b
1405a4d4b: mov rax, qword ptr [r8 + 0x1e0]
1405a4d52: not rcx
1405a4d55: lea rcx, [rcx + rcx*2]
1405a4d59: shl rcx, 5
1405a4d5d: cmp qword ptr [rax + rcx + 0x20], 0
1405a4d63: je 0x1405a690b
1405a4d69: add rax, rcx
1405a4d6c: mov rax, qword ptr [rax + 0x10]
1405a4d70: movzx edx, byte ptr [rax]
1405a4d73: mov byte ptr [rbx + 0xb3], 1
1405a4d7a: mov rcx, rsi
1405a4d7d: call 0x14013a1d0 ; bevy_app::app::App::insert_resource<country_core::resources::platform::RuntimePlatform>
1405a4d82: mov rsi, rax
1405a4d85: mov byte ptr [rbx + 0xb3], 1
1405a4d8c: lea rcx, [rbx + 0xb00]
1405a4d93: xor edx, edx
1405a4d95: call 0x1408ba0f0 ; country_core::asset_libraries::configwatch::ConfigWatch::new
1405a4d9a: mov byte ptr [rbx + 0xb3], 1
1405a4da1: lea rdx, [rbx + 0xb00]
1405a4da8: mov rcx, rsi
1405a4dab: call 0x140131d60 ; bevy_app::app::App::insert_resource<country_core::asset_libraries::configwatch::ConfigWatch>
1405a4db0: mov rsi, rax
1405a4db3: movaps xmmword ptr [rbx + 0x25c0], xmm8
1405a4dbb: movaps xmmword ptr [rbx + 0x25b0], xmm9
1405a4dc3: movaps xmmword ptr [rbx + 0x25a0], xmm10
1405a4dcb: movaps xmmword ptr [rbx + 0x2590], xmm11
1405a4dd3: movaps xmmword ptr [rbx + 0x25d0], xmm11
1405a4ddb: movaps xmmword ptr [rbx + 0x25e0], xmm10
1405a4de3: movaps xmmword ptr [rbx + 0x25f0], xmm9
1405a4deb: movaps xmmword ptr [rbx + 0x2600], xmm8
1405a4df3: movaps xmmword ptr [rbx + 0x2610], xmm11
1405a4dfb: movaps xmmword ptr [rbx + 0x2620], xmm10
1405a4e03: movaps xmmword ptr [rbx + 0x2630], xmm9
1405a4e0b: movaps xmmword ptr [rbx + 0x2640], xmm8
1405a4e13: movaps xmmword ptr [rbx + 0x2650], xmm11
1405a4e1b: movaps xmmword ptr [rbx + 0x2660], xmm10
1405a4e23: movaps xmmword ptr [rbx + 0x2670], xmm9
1405a4e2b: movaps xmmword ptr [rbx + 0x2680], xmm8
1405a4e33: mov dword ptr [rbx + 0xc08], 0
1405a4e3d: lea rdi, [rbx + 0xb00]
1405a4e44: lea rdx, [rbx + 0x2590]
1405a4e4b: mov r8d, 0x100
1405a4e51: mov rcx, rdi
1405a4e54: call 0x142923c00 ; 
1405a4e59: mov qword ptr [rbx + 0xc00], 0
1405a4e64: mov byte ptr [rbx + 0xb3], 1
1405a4e6b: mov rcx, rsi
1405a4e6e: mov rdx, rdi
1405a4e71: call 0x140135cc0 ; bevy_app::app::App::insert_resource<country_core::render::RenderTemporalState>
1405a4e76: mov rsi, rax
1405a4e79: mov byte ptr [rbx + 0xb3], 1
1405a4e80: lea rcx, [rbx + 0xb00]
1405a4e87: call 0x140936c60 ; country_core::asset_libraries::shader_library::AssetShaderLibrary::new
1405a4e8c: mov byte ptr [rbx + 0xb3], 1
1405a4e93: lea rdx, [rbx + 0xb00]
1405a4e9a: mov rcx, rsi
1405a4e9d: call 0x14012a6e0 ; bevy_app::app::App::insert_resource<country_core::asset_libraries::shader_library::AssetShaderLibrary>
1405a4ea2: mov rdi, rax
1405a4ea5: lea rsi, [r15 + 0x10]
1405a4ea9: mov byte ptr [rbx + 0xb3], 1
1405a4eb0: lea rcx, [rbx + 0xb00]
1405a4eb7: mov rdx, rsi
1405a4eba: call 0x14093bb10 ; country_core::asset_libraries::texture_library::AssetTextureLibrary::new
1405a4ebf: mov byte ptr [rbx + 0xb3], 1
1405a4ec6: lea rdx, [rbx + 0xb00]
1405a4ecd: mov rcx, rdi
1405a4ed0: call 0x140126d20 ; bevy_app::app::App::insert_resource<country_core::asset_libraries::texture_library::AssetTextureLibrary>
1405a4ed5: mov rdi, rax
1405a4ed8: mov byte ptr [rbx + 0xb3], 1
1405a4edf: lea rcx, [rbx + 0xb00]
1405a4ee6: call 0x14092c920 ; country_core::asset_libraries::mesh_library::AssetMeshLibrary::new
1405a4eeb: mov byte ptr [rbx + 0xb3], 1
1405a4ef2: lea rdx, [rbx + 0xb00]
1405a4ef9: mov rcx, rdi
1405a4efc: call 0x140126750 ; bevy_app::app::App::insert_resource<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
1405a4f01: mov rdi, rax
1405a4f04: mov byte ptr [rbx + 0xb3], 1
1405a4f0b: lea rcx, [rbx + 0xb00]
1405a4f12: call 0x14091eb60 ; country_core::asset_libraries::ssbo_library::impl$6::default
1405a4f17: mov byte ptr [rbx + 0xb3], 1
1405a4f1e: lea rdx, [rbx + 0xb00]
1405a4f25: mov rcx, rdi
1405a4f28: call 0x140119210 ; bevy_app::app::App::insert_resource<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
1405a4f2d: mov rdi, rax
1405a4f30: mov qword ptr [rbx + 0xba0], 0
1405a4f3b: mov dword ptr [rbx + 0xba8], 0
1405a4f45: mov qword ptr [rbx + 0xbb0], 0
1405a4f50: mov dword ptr [rbx + 0xbb8], 0
1405a4f5a: mov qword ptr [rbx + 0xbc0], 0
1405a4f65: mov dword ptr [rbx + 0xbc8], 0
1405a4f6f: xorps xmm6, xmm6
1405a4f72: movaps xmmword ptr [rbx + 0xbe0], xmm6
1405a4f79: lea rcx, [rbx + 0xbf0]
1405a4f80: mov byte ptr [rbx + 0xb3], 1
1405a4f87: call 0x1400ef490 ; core::array::impl$56::default<country_core::render::view_constants::ViewConstants>
1405a4f8c: mov dword ptr [rbx + 0xbac], 0
1405a4f96: mov dword ptr [rbx + 0xbbc], 0
1405a4fa0: movups xmmword ptr [rbx + 0xbcc], xmm6
1405a4fa7: mov dword ptr [rbx + 0xbdc], 0
1405a4fb1: movaps xmmword ptr [rbx + 0xb00], xmm6
1405a4fb8: movaps xmmword ptr [rbx + 0xb10], xmm6
1405a4fbf: movaps xmmword ptr [rbx + 0xb20], xmm6
1405a4fc6: movaps xmmword ptr [rbx + 0xb30], xmm6
1405a4fcd: movaps xmmword ptr [rbx + 0xb40], xmm6
1405a4fd4: movaps xmmword ptr [rbx + 0xb50], xmm6
1405a4fdb: movaps xmmword ptr [rbx + 0xb60], xmm6
1405a4fe2: movaps xmmword ptr [rbx + 0xb70], xmm6
1405a4fe9: movaps xmmword ptr [rbx + 0xb80], xmm6
1405a4ff0: movaps xmmword ptr [rbx + 0xb90], xmm6
1405a4ff7: mov byte ptr [rbx + 0xb3], 1
1405a4ffe: lea rdx, [rbx + 0xb00]
1405a5005: mov rcx, rdi
1405a5008: call 0x14012d450 ; bevy_app::app::App::insert_resource<country_core::render::frame_constants::FrameConstants>
1405a500d: mov rdi, rax
1405a5010: mov byte ptr [rbx + 0xb3], 1
1405a5017: lea rcx, [rbx + 0xb00]
1405a501e: xor edx, edx
1405a5020: call 0x1408ba490 ; country_core::asset_libraries::meshwatch::MeshWatch::new
1405a5025: mov byte ptr [rbx + 0xb3], 1
1405a502c: lea rdx, [rbx + 0xb00]
1405a5033: mov rcx, rdi
1405a5036: call 0x140128420 ; bevy_app::app::App::insert_resource<country_core::asset_libraries::meshwatch::MeshWatch>
1405a503b: mov byte ptr [rbx + 0xb3], 1
1405a5042: mov rcx, rax
1405a5045: call 0x140115fa0 ; bevy_app::app::App::insert_resource<country_core::render::shaderwatch::inner::ShaderWatch>
1405a504a: mov rdi, rax
1405a504d: mov byte ptr [rbx + 0xb3], 1
1405a5054: lea rcx, [rbx + 0xb00]
1405a505b: xor edx, edx
1405a505d: call 0x1408bab30 ; country_core::asset_libraries::texture_watch::TextureWatch::new
1405a5062: mov byte ptr [rbx + 0xb3], 1
1405a5069: lea rdx, [rbx + 0xb00]
1405a5070: mov rcx, rdi
1405a5073: call 0x140136de0 ; bevy_app::app::App::insert_resource<country_core::asset_libraries::texture_watch::TextureWatch>
1405a5078: mov rdi, rax
1405a507b: mov byte ptr [rbx + 0xb3], 1
1405a5082: lea rcx, [rbx + 0xb00]
1405a5089: call 0x140924030 ; country_core::asset_libraries::vao_library::AssetVAOLibrary::new
1405a508e: mov byte ptr [rbx + 0xb3], 1
1405a5095: lea rdx, [rbx + 0xb00]
1405a509c: mov rcx, rdi
1405a509f: call 0x1401214f0 ; bevy_app::app::App::insert_resource<country_core::asset_libraries::vao_library::AssetVAOLibrary>
1405a50a4: mov rdi, rax
1405a50a7: mov byte ptr [rbx + 0xb3], 1
1405a50ae: lea rcx, [rbx + 0xb00]
1405a50b5: mov rdx, rsi
1405a50b8: call 0x140c02570 ; rhapsody::raw::dynamic_constants::DynamicConstants::new
1405a50bd: movups xmm0, xmmword ptr [rbx + 0xb40]
1405a50c4: movups xmmword ptr [rbx + 0x25d7], xmm0
1405a50cb: movups xmm0, xmmword ptr [rbx + 0xb00]
1405a50d2: movups xmm1, xmmword ptr [rbx + 0xb10]
1405a50d9: movups xmm2, xmmword ptr [rbx + 0xb20]
1405a50e0: movups xmm3, xmmword ptr [rbx + 0xb30]
1405a50e7: movups xmmword ptr [rbx + 0x25c7], xmm3
1405a50ee: movups xmmword ptr [rbx + 0x25b7], xmm2
1405a50f5: movups xmmword ptr [rbx + 0x25a7], xmm1
1405a50fc: movups xmmword ptr [rbx + 0x2597], xmm0
1405a5103: mov byte ptr [rbx + 0xb00], 0
1405a510a: movups xmm0, xmmword ptr [rbx + 0x2590]
1405a5111: movups xmmword ptr [rbx + 0xb01], xmm0
1405a5118: movups xmm0, xmmword ptr [rbx + 0x25a0]
1405a511f: movups xmmword ptr [rbx + 0xb11], xmm0
1405a5126: movups xmm0, xmmword ptr [rbx + 0x25b0]
1405a512d: movups xmmword ptr [rbx + 0xb21], xmm0
1405a5134: movups xmm0, xmmword ptr [rbx + 0x25c0]
1405a513b: movups xmmword ptr [rbx + 0xb31], xmm0
1405a5142: movups xmm0, xmmword ptr [rbx + 0x25d0]
1405a5149: movups xmmword ptr [rbx + 0xb41], xmm0
1405a5150: mov rax, qword ptr [rbx + 0x25df]
1405a5157: mov qword ptr [rbx + 0xb50], rax
1405a515e: mov byte ptr [rbx + 0xb3], 1
1405a5165: lea rdx, [rbx + 0xb00]
1405a516c: mov rcx, rdi
1405a516f: call 0x14011f840 ; bevy_app::app::App::insert_resource<country_core::render::DynamicConstantsResource>
1405a5174: mov byte ptr [rbx + 0xb3], 0
1405a517b: mov rcx, rax
1405a517e: mov rdx, r15
1405a5181: call 0x14011cb90 ; bevy_app::app::App::insert_resource<country_core::render::GraphicsDeviceResource>
1405a5186: mov rsi, rax
1405a5189: mov byte ptr [rbx + 0xb3], 0
1405a5190: lea rcx, [rbx + 0xb00]
1405a5197: call 0x140924030 ; country_core::asset_libraries::vao_library::AssetVAOLibrary::new
1405a519c: mov byte ptr [rbx + 0xb3], 0
1405a51a3: lea rdx, [rbx + 0xb00]
1405a51aa: mov rcx, rsi
1405a51ad: call 0x1401214f0 ; bevy_app::app::App::insert_resource<country_core::asset_libraries::vao_library::AssetVAOLibrary>
1405a51b2: mov rsi, rax
1405a51b5: mov byte ptr [rbx + 0xb3], 0
1405a51bc: lea rcx, [rbx + 0xb00]
1405a51c3: call 0x140867700 ; country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary::new
1405a51c8: mov byte ptr [rbx + 0xb3], 0
1405a51cf: lea rdx, [rbx + 0xb00]
1405a51d6: mov rcx, rsi
1405a51d9: call 0x1401237f0 ; bevy_app::app::App::insert_resource<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>
1405a51de: mov byte ptr [rbx + 0xb3], 0
1405a51e5: mov rcx, rax
1405a51e8: call 0x140115fa0 ; bevy_app::app::App::insert_resource<country_core::render::shaderwatch::inner::ShaderWatch>
1405a51ed: mov qword ptr [rbx + 0xb00], 0
1405a51f8: mov qword ptr [rbx + 0xb08], 4
1405a5203: mov qword ptr [rbx + 0xb10], 0
1405a520e: mov byte ptr [rbx + 0xb3], 0
1405a5215: lea rdx, [rbx + 0xb00]
1405a521c: mov rcx, rax
1405a521f: call 0x140137f20 ; bevy_app::app::App::insert_resource<country_core::resources::session::SavedSessionList>
1405a5224: mov rsi, rax
1405a5227: mov byte ptr [rbx + 0xb3], 0
1405a522e: lea rcx, [rbx + 0xb00]
1405a5235: call 0x1408ae7f0 ; country_core::resources::image_formation::impl$0::default
1405a523a: mov byte ptr [rbx + 0xb3], 0
1405a5241: lea rdx, [rbx + 0xb00]
1405a5248: mov rcx, rsi
1405a524b: call 0x140122c30 ; bevy_app::app::App::insert_resource<country_core::resources::image_formation::ImageFormationConfigs>
1405a5250: mov qword ptr [rbx + 0xb00], 0
1405a525b: mov qword ptr [rbx + 0xb10], 0
1405a5266: mov dword ptr [rbx + 0xb28], 0x3b9aca00
1405a5270: mov byte ptr [rbx + 0xb30], 0
1405a5277: mov byte ptr [rbx + 0xb3], 0
1405a527e: lea rdx, [rbx + 0xb00]
1405a5285: mov rcx, rax
1405a5288: call 0x14011e1d0 ; bevy_app::app::App::insert_resource<country_core::systems::ui::resources::widget_params::UiNavState>
1405a528d: mov qword ptr [rbx + 0x128], rax
1405a5294: mov byte ptr [rbx + 0xb3], 0
1405a529b: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a52a0: mov dword ptr [rbx + 0x150], eax
1405a52a6: movss dword ptr [rbx + 0x26c], xmm0
1405a52ae: mov byte ptr [rbx + 0xb3], 0
1405a52b5: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a52ba: mov dword ptr [rbx + 0x268], eax
1405a52c0: movss dword ptr [rbx + 0x264], xmm0
1405a52c8: mov byte ptr [rbx + 0xb3], 0
1405a52cf: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a52d4: mov dword ptr [rbx + 0x260], eax
1405a52da: movss dword ptr [rbx + 0x25c], xmm0
1405a52e2: mov byte ptr [rbx + 0xb3], 0
1405a52e9: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a52ee: mov dword ptr [rbx + 0x258], eax
1405a52f4: movss dword ptr [rbx + 0x254], xmm0
1405a52fc: mov byte ptr [rbx + 0xb3], 0
1405a5303: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a5308: mov dword ptr [rbx + 0x250], eax
1405a530e: movss dword ptr [rbx + 0x24c], xmm0
1405a5316: mov byte ptr [rbx + 0xb3], 0
1405a531d: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a5322: mov dword ptr [rbx + 0x248], eax
1405a5328: movss dword ptr [rbx + 0x244], xmm0
1405a5330: mov byte ptr [rbx + 0xb3], 0
1405a5337: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a533c: mov dword ptr [rbx + 0x240], eax
1405a5342: movss dword ptr [rbx + 0x23c], xmm0
1405a534a: mov byte ptr [rbx + 0xb3], 0
1405a5351: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a5356: mov dword ptr [rbx + 0x238], eax
1405a535c: movss dword ptr [rbx + 0x234], xmm0
1405a5364: mov byte ptr [rbx + 0xb3], 0
1405a536b: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a5370: mov dword ptr [rbx + 0x230], eax
1405a5376: movss dword ptr [rbx + 0x22c], xmm0
1405a537e: mov byte ptr [rbx + 0xb3], 0
1405a5385: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a538a: mov dword ptr [rbx + 0x228], eax
1405a5390: movss dword ptr [rbx + 0x224], xmm0
1405a5398: mov byte ptr [rbx + 0xb3], 0
1405a539f: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a53a4: mov dword ptr [rbx + 0x220], eax
1405a53aa: movss dword ptr [rbx + 0x21c], xmm0
1405a53b2: mov byte ptr [rbx + 0xb3], 0
1405a53b9: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a53be: mov dword ptr [rbx + 0x218], eax
1405a53c4: movss dword ptr [rbx + 0x214], xmm0
1405a53cc: mov byte ptr [rbx + 0xb3], 0
1405a53d3: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a53d8: mov dword ptr [rbx + 0x210], eax
1405a53de: movss dword ptr [rbx + 0x20c], xmm0
1405a53e6: mov byte ptr [rbx + 0xb3], 0
1405a53ed: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a53f2: mov dword ptr [rbx + 0x208], eax
1405a53f8: movss dword ptr [rbx + 0x204], xmm0
1405a5400: mov byte ptr [rbx + 0xb3], 0
1405a5407: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a540c: mov dword ptr [rbx + 0x200], eax
1405a5412: movss dword ptr [rbx + 0x1fc], xmm0
1405a541a: mov byte ptr [rbx + 0xb3], 0
1405a5421: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a5426: mov dword ptr [rbx + 0x1f8], eax
1405a542c: movss dword ptr [rbx + 0x1f4], xmm0
1405a5434: mov byte ptr [rbx + 0xb3], 0
1405a543b: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a5440: mov dword ptr [rbx + 0x1f0], eax
1405a5446: movss dword ptr [rbx + 0x1ec], xmm0
1405a544e: mov byte ptr [rbx + 0xb3], 0
1405a5455: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a545a: mov dword ptr [rbx + 0x1e8], eax
1405a5460: movss dword ptr [rbx + 0x1e4], xmm0
1405a5468: mov byte ptr [rbx + 0xb3], 0
1405a546f: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a5474: mov dword ptr [rbx + 0x1e0], eax
1405a547a: movss dword ptr [rbx + 0x1dc], xmm0
1405a5482: mov byte ptr [rbx + 0xb3], 0
1405a5489: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a548e: mov dword ptr [rbx + 0x1d8], eax
1405a5494: movss dword ptr [rbx + 0x1d4], xmm0
1405a549c: mov byte ptr [rbx + 0xb3], 0
1405a54a3: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a54a8: mov dword ptr [rbx + 0x1d0], eax
1405a54ae: movss dword ptr [rbx + 0x1cc], xmm0
1405a54b6: mov byte ptr [rbx + 0xb3], 0
1405a54bd: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a54c2: mov dword ptr [rbx + 0x1c8], eax
1405a54c8: movss dword ptr [rbx + 0x1c4], xmm0
1405a54d0: mov byte ptr [rbx + 0xb3], 0
1405a54d7: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a54dc: mov dword ptr [rbx + 0x1c0], eax
1405a54e2: movss dword ptr [rbx + 0x1bc], xmm0
1405a54ea: mov byte ptr [rbx + 0xb3], 0
1405a54f1: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a54f6: mov dword ptr [rbx + 0x1b8], eax
1405a54fc: movss dword ptr [rbx + 0x1b4], xmm0
1405a5504: mov byte ptr [rbx + 0xb3], 0
1405a550b: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a5510: mov dword ptr [rbx + 0x1b0], eax
1405a5516: movss dword ptr [rbx + 0x1ac], xmm0
1405a551e: mov byte ptr [rbx + 0xb3], 0
1405a5525: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a552a: mov dword ptr [rbx + 0x1a8], eax
1405a5530: movss dword ptr [rbx + 0x1a4], xmm0
1405a5538: mov byte ptr [rbx + 0xb3], 0
1405a553f: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a5544: mov dword ptr [rbx + 0x1a0], eax
1405a554a: movss dword ptr [rbx + 0x19c], xmm0
1405a5552: mov byte ptr [rbx + 0xb3], 0
1405a5559: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a555e: mov dword ptr [rbx + 0x198], eax
1405a5564: movss dword ptr [rbx + 0x194], xmm0
1405a556c: mov byte ptr [rbx + 0xb3], 0
1405a5573: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a5578: mov dword ptr [rbx + 0x190], eax
1405a557e: movss dword ptr [rbx + 0x18c], xmm0
1405a5586: mov byte ptr [rbx + 0xb3], 0
1405a558d: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a5592: mov dword ptr [rbx + 0x188], eax
1405a5598: movaps xmm15, xmm0
1405a559c: mov byte ptr [rbx + 0xb3], 0
1405a55a3: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a55a8: mov dword ptr [rbx + 0x184], eax
1405a55ae: movaps xmm6, xmm0
1405a55b1: mov byte ptr [rbx + 0xb3], 0
1405a55b8: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a55bd: mov dword ptr [rbx + 0x180], eax
1405a55c3: movaps xmm7, xmm0
1405a55c6: mov byte ptr [rbx + 0xb3], 0
1405a55cd: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a55d2: mov dword ptr [rbx + 0x17c], eax
1405a55d8: movaps xmm8, xmm0
1405a55dc: mov byte ptr [rbx + 0xb3], 0
1405a55e3: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a55e8: mov r15d, eax
1405a55eb: movaps xmm9, xmm0
1405a55ef: mov byte ptr [rbx + 0xb3], 0
1405a55f6: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a55fb: mov r12d, eax
1405a55fe: movaps xmm10, xmm0
1405a5602: mov byte ptr [rbx + 0xb3], 0
1405a5609: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a560e: mov esi, eax
1405a5610: movaps xmm11, xmm0
1405a5614: mov byte ptr [rbx + 0xb3], 0
1405a561b: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a5620: mov r13d, eax
1405a5623: movaps xmm12, xmm0
1405a5627: mov byte ptr [rbx + 0xb3], 0
1405a562e: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a5633: mov edi, eax
1405a5635: movaps xmm13, xmm0
1405a5639: mov byte ptr [rbx + 0xb3], 0
1405a5640: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a5645: mov r14d, eax
1405a5648: movaps xmm14, xmm0
1405a564c: mov byte ptr [rbx + 0xb3], 0
1405a5653: call 0x14093fa60 ; country_core::input::action_set::impl$0::default
1405a5658: xorps xmm1, xmm1
1405a565b: movups xmmword ptr [rbx + 0xc60], xmm1
1405a5662: movups xmmword ptr [rbx + 0xc50], xmm1
1405a5669: movups xmmword ptr [rbx + 0xc40], xmm1
1405a5670: mov ecx, dword ptr [rbx + 0x150]
1405a5676: mov dword ptr [rbx + 0xb00], ecx
1405a567c: movss xmm1, dword ptr [rbx + 0x26c]
1405a5684: movss dword ptr [rbx + 0xb04], xmm1
1405a568c: mov ecx, dword ptr [rbx + 0x268]
1405a5692: mov dword ptr [rbx + 0xb08], ecx
1405a5698: movss xmm1, dword ptr [rbx + 0x264]
1405a56a0: movss dword ptr [rbx + 0xb0c], xmm1
1405a56a8: mov ecx, dword ptr [rbx + 0x260]
1405a56ae: mov dword ptr [rbx + 0xb10], ecx
1405a56b4: movss xmm1, dword ptr [rbx + 0x25c]
1405a56bc: movss dword ptr [rbx + 0xb14], xmm1
1405a56c4: mov ecx, dword ptr [rbx + 0x258]
1405a56ca: mov dword ptr [rbx + 0xb18], ecx
1405a56d0: movss xmm1, dword ptr [rbx + 0x254]
1405a56d8: movss dword ptr [rbx + 0xb1c], xmm1
1405a56e0: mov ecx, dword ptr [rbx + 0x250]
1405a56e6: mov dword ptr [rbx + 0xb20], ecx
1405a56ec: movss xmm1, dword ptr [rbx + 0x24c]
1405a56f4: movss dword ptr [rbx + 0xb24], xmm1
1405a56fc: mov ecx, dword ptr [rbx + 0x248]
1405a5702: mov dword ptr [rbx + 0xb28], ecx
1405a5708: movss xmm1, dword ptr [rbx + 0x244]
1405a5710: movss dword ptr [rbx + 0xb2c], xmm1
1405a5718: mov ecx, dword ptr [rbx + 0x240]
1405a571e: mov dword ptr [rbx + 0xb30], ecx
1405a5724: movss xmm1, dword ptr [rbx + 0x23c]
1405a572c: movss dword ptr [rbx + 0xb34], xmm1
1405a5734: mov ecx, dword ptr [rbx + 0x238]
1405a573a: mov dword ptr [rbx + 0xb38], ecx
1405a5740: movss xmm1, dword ptr [rbx + 0x234]
1405a5748: movss dword ptr [rbx + 0xb3c], xmm1
1405a5750: mov ecx, dword ptr [rbx + 0x230]
1405a5756: mov dword ptr [rbx + 0xb40], ecx
1405a575c: movss xmm1, dword ptr [rbx + 0x22c]
1405a5764: movss dword ptr [rbx + 0xb44], xmm1
1405a576c: mov ecx, dword ptr [rbx + 0x228]
1405a5772: mov dword ptr [rbx + 0xb48], ecx
1405a5778: movss xmm1, dword ptr [rbx + 0x224]
1405a5780: movss dword ptr [rbx + 0xb4c], xmm1
1405a5788: mov ecx, dword ptr [rbx + 0x220]
1405a578e: mov dword ptr [rbx + 0xb50], ecx
1405a5794: movss xmm1, dword ptr [rbx + 0x21c]
1405a579c: movss dword ptr [rbx + 0xb54], xmm1
1405a57a4: mov ecx, dword ptr [rbx + 0x218]
1405a57aa: mov dword ptr [rbx + 0xb58], ecx
1405a57b0: movss xmm1, dword ptr [rbx + 0x214]
1405a57b8: movss dword ptr [rbx + 0xb5c], xmm1
1405a57c0: mov ecx, dword ptr [rbx + 0x210]
1405a57c6: mov dword ptr [rbx + 0xb60], ecx
1405a57cc: movss xmm1, dword ptr [rbx + 0x20c]
1405a57d4: movss dword ptr [rbx + 0xb64], xmm1
1405a57dc: mov ecx, dword ptr [rbx + 0x208]
1405a57e2: mov dword ptr [rbx + 0xb68], ecx
1405a57e8: movss xmm1, dword ptr [rbx + 0x204]
1405a57f0: movss dword ptr [rbx + 0xb6c], xmm1
1405a57f8: mov ecx, dword ptr [rbx + 0x200]
1405a57fe: mov dword ptr [rbx + 0xb70], ecx
1405a5804: movss xmm1, dword ptr [rbx + 0x1fc]
1405a580c: movss dword ptr [rbx + 0xb74], xmm1
1405a5814: mov ecx, dword ptr [rbx + 0x1f8]
1405a581a: mov dword ptr [rbx + 0xb78], ecx
1405a5820: movss xmm1, dword ptr [rbx + 0x1f4]
1405a5828: movss dword ptr [rbx + 0xb7c], xmm1
1405a5830: mov ecx, dword ptr [rbx + 0x1f0]
1405a5836: mov dword ptr [rbx + 0xb80], ecx
1405a583c: movss xmm1, dword ptr [rbx + 0x1ec]
1405a5844: movss dword ptr [rbx + 0xb84], xmm1
1405a584c: mov ecx, dword ptr [rbx + 0x1e8]
1405a5852: mov dword ptr [rbx + 0xb88], ecx
1405a5858: movss xmm1, dword ptr [rbx + 0x1e4]
1405a5860: movss dword ptr [rbx + 0xb8c], xmm1
1405a5868: mov ecx, dword ptr [rbx + 0x1e0]
1405a586e: mov dword ptr [rbx + 0xb90], ecx
1405a5874: movss xmm1, dword ptr [rbx + 0x1dc]
1405a587c: movss dword ptr [rbx + 0xb94], xmm1
1405a5884: mov ecx, dword ptr [rbx + 0x1d8]
1405a588a: mov dword ptr [rbx + 0xb98], ecx
1405a5890: movss xmm1, dword ptr [rbx + 0x1d4]
1405a5898: movss dword ptr [rbx + 0xb9c], xmm1
1405a58a0: mov ecx, dword ptr [rbx + 0x1d0]
1405a58a6: mov dword ptr [rbx + 0xba0], ecx
1405a58ac: movss xmm1, dword ptr [rbx + 0x1cc]
1405a58b4: movss dword ptr [rbx + 0xba4], xmm1
1405a58bc: mov ecx, dword ptr [rbx + 0x1c8]
1405a58c2: mov dword ptr [rbx + 0xba8], ecx
1405a58c8: movss xmm1, dword ptr [rbx + 0x1c4]
1405a58d0: movss dword ptr [rbx + 0xbac], xmm1
1405a58d8: mov ecx, dword ptr [rbx + 0x1c0]
1405a58de: mov dword ptr [rbx + 0xbb0], ecx
1405a58e4: movss xmm1, dword ptr [rbx + 0x1bc]
1405a58ec: movss dword ptr [rbx + 0xbb4], xmm1
1405a58f4: mov ecx, dword ptr [rbx + 0x1b8]
1405a58fa: mov dword ptr [rbx + 0xbb8], ecx
1405a5900: movss xmm1, dword ptr [rbx + 0x1b4]
1405a5908: movss dword ptr [rbx + 0xbbc], xmm1
1405a5910: mov ecx, dword ptr [rbx + 0x1b0]
1405a5916: mov dword ptr [rbx + 0xbc0], ecx
1405a591c: movss xmm1, dword ptr [rbx + 0x1ac]
1405a5924: movss dword ptr [rbx + 0xbc4], xmm1
1405a592c: mov ecx, dword ptr [rbx + 0x1a8]
1405a5932: mov dword ptr [rbx + 0xbc8], ecx
1405a5938: movss xmm1, dword ptr [rbx + 0x1a4]
1405a5940: movss dword ptr [rbx + 0xbcc], xmm1
1405a5948: mov ecx, dword ptr [rbx + 0x1a0]
1405a594e: mov dword ptr [rbx + 0xbd0], ecx
1405a5954: movss xmm1, dword ptr [rbx + 0x19c]
1405a595c: movss dword ptr [rbx + 0xbd4], xmm1
1405a5964: mov ecx, dword ptr [rbx + 0x198]
1405a596a: mov dword ptr [rbx + 0xbd8], ecx
1405a5970: movss xmm1, dword ptr [rbx + 0x194]
1405a5978: movss dword ptr [rbx + 0xbdc], xmm1
1405a5980: mov ecx, dword ptr [rbx + 0x190]
1405a5986: mov dword ptr [rbx + 0xbe0], ecx
1405a598c: movss xmm1, dword ptr [rbx + 0x18c]
1405a5994: movss dword ptr [rbx + 0xbe4], xmm1
1405a599c: mov ecx, dword ptr [rbx + 0x188]
1405a59a2: mov dword ptr [rbx + 0xbe8], ecx
1405a59a8: movss dword ptr [rbx + 0xbec], xmm15
1405a59b1: mov ecx, dword ptr [rbx + 0x184]
1405a59b7: mov dword ptr [rbx + 0xbf0], ecx
1405a59bd: movss dword ptr [rbx + 0xbf4], xmm6
1405a59c5: mov ecx, dword ptr [rbx + 0x180]
1405a59cb: mov dword ptr [rbx + 0xbf8], ecx
1405a59d1: movss dword ptr [rbx + 0xbfc], xmm7
1405a59d9: mov ecx, dword ptr [rbx + 0x17c]
1405a59df: mov dword ptr [rbx + 0xc00], ecx
1405a59e5: movss dword ptr [rbx + 0xc04], xmm8
1405a59ee: mov dword ptr [rbx + 0xc08], r15d
1405a59f5: movss dword ptr [rbx + 0xc0c], xmm9
1405a59fe: mov dword ptr [rbx + 0xc10], r12d
1405a5a05: movss dword ptr [rbx + 0xc14], xmm10
1405a5a0e: mov dword ptr [rbx + 0xc18], esi
1405a5a14: movss dword ptr [rbx + 0xc1c], xmm11
1405a5a1d: mov dword ptr [rbx + 0xc20], r13d
1405a5a24: movss dword ptr [rbx + 0xc24], xmm12
1405a5a2d: mov dword ptr [rbx + 0xc28], edi
1405a5a33: movss dword ptr [rbx + 0xc2c], xmm13
1405a5a3c: mov dword ptr [rbx + 0xc30], r14d
1405a5a43: movss dword ptr [rbx + 0xc34], xmm14
1405a5a4c: mov dword ptr [rbx + 0xc38], eax
1405a5a52: movss dword ptr [rbx + 0xc3c], xmm0
1405a5a5a: mov byte ptr [rbx + 0xb3], 0
1405a5a61: lea rdx, [rbx + 0xb00]
1405a5a68: mov rcx, qword ptr [rbx + 0x128]
1405a5a6f: call 0x1401278f0 ; bevy_app::app::App::insert_resource<country_core::input::action_set::ActionSetStates>
1405a5a74: mov rsi, rax
1405a5a77: mov byte ptr [rbx + 0xb3], 0
1405a5a7e: movabs rdx, 0x2b39980e7de6bf8a
1405a5a88: movabs r8, 0x52d9aabfd20d56c5
1405a5a92: mov rcx, qword ptr [rbx + 0xe8]
1405a5a99: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1405a5a9e: test rax, rax
1405a5aa1: mov r8, qword ptr [rbx + 0xc8]
1405a5aa8: je 0x1405a6995
1405a5aae: cmp qword ptr [r8 + 0x218], rdx
1405a5ab5: jbe 0x1405a6995
1405a5abb: mov rax, qword ptr [r8 + 0x210]
1405a5ac2: mov rcx, qword ptr [rax + rdx*8]
1405a5ac6: test rcx, rcx
1405a5ac9: je 0x1405a6995
1405a5acf: mov rax, qword ptr [r8 + 0x1e0]
1405a5ad6: not rcx
1405a5ad9: lea rcx, [rcx + rcx*2]
1405a5add: shl rcx, 5
1405a5ae1: cmp qword ptr [rax + rcx + 0x20], 0
1405a5ae7: je 0x1405a6995
1405a5aed: add rax, rcx
1405a5af0: mov rax, qword ptr [rax + 0x10]
1405a5af4: mov rcx, qword ptr [rax]
1405a5af7: mov edx, dword ptr [rax + 8]
1405a5afa: mov r8, qword ptr [rax + 0x10]
1405a5afe: mov eax, dword ptr [rax + 0x18]
1405a5b01: mov qword ptr [rbx + 0xb00], rcx
1405a5b08: mov dword ptr [rbx + 0xb08], edx
1405a5b0e: mov qword ptr [rbx + 0xb10], r8
1405a5b15: mov dword ptr [rbx + 0xb18], eax
1405a5b1b: mov byte ptr [rbx + 0xb3], 0
1405a5b22: lea rdx, [rbx + 0xb00]
1405a5b29: mov rcx, rsi
1405a5b2c: call 0x14012b880 ; bevy_app::app::App::insert_resource<country_core::input::ActiveInputModeTracker>
1405a5b31: mov rsi, rax
1405a5b34: mov byte ptr [rbx + 0xb3], 0
1405a5b3b: lea rcx, [rbx + 0xb00]
1405a5b42: call 0x1405b7290 ; bevy_ecs::schedule::config::IntoScheduleConfigs::chain<tuple$<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>)>,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>)> > > >
1405a5b47: movups xmm0, xmmword ptr [rbx + 0xb60]
1405a5b4e: movaps xmmword ptr [rbx + 0x25f0], xmm0
1405a5b55: movups xmm0, xmmword ptr [rbx + 0xb50]
1405a5b5c: movaps xmmword ptr [rbx + 0x25e0], xmm0
1405a5b63: movups xmm0, xmmword ptr [rbx + 0xb40]
1405a5b6a: movaps xmmword ptr [rbx + 0x25d0], xmm0
1405a5b71: movups xmm0, xmmword ptr [rbx + 0xb00]
1405a5b78: movups xmm1, xmmword ptr [rbx + 0xb10]
1405a5b7f: movups xmm2, xmmword ptr [rbx + 0xb20]
1405a5b86: movups xmm3, xmmword ptr [rbx + 0xb30]
1405a5b8d: movaps xmmword ptr [rbx + 0x25c0], xmm3
1405a5b94: movaps xmmword ptr [rbx + 0x25b0], xmm2
1405a5b9b: movaps xmmword ptr [rbx + 0x25a0], xmm1
1405a5ba2: movaps xmmword ptr [rbx + 0x2590], xmm0
1405a5ba9: mov byte ptr [rbx + 0xb3], 0
1405a5bb0: lea rdx, [rbx + 0x2590]
1405a5bb7: mov rcx, rsi
1405a5bba: call 0x140326890 ; bevy_app::sub_app::SubApp::add_systems<tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<>,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)> >,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState> > >)> > >,bevy_app::main_schedule::PostUpdate,tuple$<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > >,void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState> > >)> >
1405a5bbf: lea rcx, [rbx + 0x2590]
1405a5bc6: lea rdx, [rbx + 0x440]
1405a5bcd: mov r8d, 0x658
1405a5bd3: call 0x142923c00 ; 
1405a5bd8: mov byte ptr [rbx + 0x112], 1
1405a5bdf: mov byte ptr [rbx + 0x113], 1
1405a5be6: mov byte ptr [rbx + 0x114], 1
1405a5bed: lea rcx, [rbx + 0xb00]
1405a5bf4: call 0x140ca5fd0 ; bevy_app::sub_app::impl$1::default
1405a5bf9: lea rcx, [rbx + 0x2bf0]
1405a5c00: mov r8d, 0x628
1405a5c06: mov rsi, qword ptr [rbx + 0xc8]
1405a5c0d: mov rdx, rsi
1405a5c10: call 0x142923c00 ; 
1405a5c15: lea rdx, [rbx + 0xb00]
1405a5c1c: mov r8d, 0x628
1405a5c22: mov rcx, rsi
1405a5c25: call 0x142923c00 ; 
1405a5c2a: mov rax, qword ptr [rbx + 0x138]
1405a5c31: lock inc qword ptr [rax]
1405a5c35: movabs rcx, 0x8000000000000000
1405a5c3f: jle 0x1405a7040
1405a5c45: mov qword ptr [rbx + 0x1ff0], rax
1405a5c4c: mov qword ptr [rbx + 0xb10], rcx
1405a5c53: mov qword ptr [rbx + 0xb00], 0
1405a5c5e: mov byte ptr [rbx + 0xb28], 0
1405a5c65: movzx eax, byte ptr [rip + 0x308ef9d]
1405a5c6c: mov ecx, 0xa
1405a5c71: mov edx, 1
1405a5c76: call 0x140613c10 ; 
1405a5c7b: test rax, rax
1405a5c7e: mov qword ptr [rbx + 0xe8], rax
1405a5c85: je 0x1405a6fe5
1405a5c8b: movabs rcx, 0x6f6c20636e797361
1405a5c95: mov qword ptr [rax], rcx
1405a5c98: mov word ptr [rax + 8], 0x6461
1405a5c9e: mov qword ptr [rbx + 0x2230], 0xa
1405a5ca9: mov qword ptr [rbx + 0x2238], rax
1405a5cb0: mov qword ptr [rbx + 0x2240], 0xa
1405a5cbb: lea rcx, [rbx + 0x440]
1405a5cc2: lea rdx, [rbx + 0xb00]
1405a5cc9: lea r8, [rbx + 0x2230]
1405a5cd0: call 0x140d558f0 ; std::thread::Builder::name
1405a5cd5: mov r13b, byte ptr [rbx + 0x107]
1405a5cdc: mov rax, qword ptr [rbx + 0x1ff0]
1405a5ce3: mov qword ptr [rbx + 0x1138], rax
1405a5cea: lea rcx, [rbx + 0xb10]
1405a5cf1: lea rdx, [rbx + 0x2bf0]
1405a5cf8: mov r8d, 0x628
1405a5cfe: call 0x142923c00 ; 
1405a5d03: movups xmm0, xmmword ptr [rbx + 0x2e0]
1405a5d0a: movups xmmword ptr [rbx + 0xb00], xmm0
1405a5d11: mov byte ptr [rbx + 0xdb], 0
1405a5d18: lea rsi, [rbx + 0x2230]
1405a5d1f: lea rdx, [rbx + 0x440]
1405a5d26: lea r8, [rbx + 0xb00]
1405a5d2d: mov rcx, rsi
1405a5d30: call 0x14028a680 ; std::thread::Builder::spawn<tiny_glade::run_game::run_game_thread::closure_env$8,tuple$<> >
1405a5d35: mov rdi, qword ptr [rbx + 0x2230]
1405a5d3c: test rdi, rdi
1405a5d3f: je 0x1405a5d86
1405a5d41: mov rcx, qword ptr [rbx + 0x2240]
1405a5d48: call qword ptr [rip + 0x2bec6a2]
1405a5d4e: lock dec qword ptr [rdi]
1405a5d52: jne 0x1405a5d60
1405a5d54: lea rcx, [rbx + 0x2230]
1405a5d5b: call 0x140d53b60 ; alloc::sync::Arc::drop_slow<std::thread::Inner,alloc::alloc::Global>
1405a5d60: mov rax, qword ptr [rbx + 0x2238]
1405a5d67: lock dec qword ptr [rax]
1405a5d6b: jne 0x1405a5e00
1405a5d71: lea rcx, [rbx + 0x2238]
1405a5d78: mov byte ptr [rbx + 0xdb], 0
1405a5d7f: call 0x140d53db0 ; alloc::sync::Arc::drop_slow<std::thread::Packet<tuple$<> >,alloc::alloc::Global>
1405a5d84: jmp 0x1405a5e00
1405a5d86: mov rax, qword ptr [rbx + 0x2238]
1405a5d8d: mov ecx, eax
1405a5d8f: and ecx, 3
1405a5d92: cmp ecx, 1
1405a5d95: jne 0x1405a5e00
1405a5d97: lea rcx, [rax - 1]
1405a5d9b: mov qword ptr [rbx + 0x150], rcx
1405a5da2: mov rcx, qword ptr [rax - 1]
1405a5da6: mov qword ptr [rbx + 0xe8], rcx
1405a5dad: mov rax, qword ptr [rax + 7]
1405a5db1: mov qword ptr [rbx + 0x128], rax
1405a5db8: mov rax, qword ptr [rax]
1405a5dbb: test rax, rax
1405a5dbe: je 0x1405a5dc9
1405a5dc0: mov rcx, qword ptr [rbx + 0xe8]
1405a5dc7: call rax
1405a5dc9: mov rcx, qword ptr [rbx + 0xe8]
1405a5dd0: mov rax, qword ptr [rbx + 0x128]
1405a5dd7: mov rdx, qword ptr [rax + 8]
1405a5ddb: test rdx, rdx
1405a5dde: je 0x1405a5de9
1405a5de0: mov r8, qword ptr [rax + 0x10]
1405a5de4: call 0x140613c20 ; 
1405a5de9: mov edx, 0x18
1405a5dee: mov r8d, 8
1405a5df4: mov rcx, qword ptr [rbx + 0x150]
1405a5dfb: call 0x140613c20 ; 
1405a5e00: lea rcx, [rbx + 0x440]
1405a5e07: lea rdx, [rbx + 0x2590]
1405a5e0e: mov r8d, 0x658
1405a5e14: call 0x142923c00 ; 
1405a5e19: movups xmm0, xmmword ptr [rbx + 0x2f0]
1405a5e20: movups xmmword ptr [rbx + 0x340], xmm0
1405a5e27: movups xmm0, xmmword ptr [rbx + 0x328]
1405a5e2e: movups xmmword ptr [rbx + 0x270], xmm0
1405a5e35: movups xmm0, xmmword ptr [rbx + 0x160]
1405a5e3c: movups xmmword ptr [rbx + 0x280], xmm0
1405a5e43: mov rax, qword ptr [rbx + 0x140]
1405a5e4a: mov qword ptr [rbx + 0x170], rax
1405a5e51: lea rdi, [rbx + 0x24e8]
1405a5e58: lea rdx, [rbx + 0x2188]
1405a5e5f: mov r8d, 0xa8
1405a5e65: mov rcx, rdi
1405a5e68: call 0x142923c00 ; 
1405a5e6d: mov rdx, qword ptr [rbx + 0x138]
1405a5e74: add rdx, 0x10
1405a5e78: mov byte ptr [rbx + 0x105], 1
1405a5e7f: lea rcx, [rbx + 0x370]
1405a5e86: call 0x140bea2d0 ; rhapsody::rg::executor::GraphExecutor::new
1405a5e8b: lea rcx, [rbx + 0x1ff0]
1405a5e92: call 0x140be57d0 ; rhapsody::rg::state::impl$0::default
1405a5e97: lea rcx, [rbx + 0xb00]
1405a5e9e: call 0x140be57d0 ; rhapsody::rg::state::impl$0::default
1405a5ea3: lea r14, [rbx + 0x370]
1405a5eaa: mov r8d, 0xd0
1405a5eb0: mov rcx, rsi
1405a5eb3: mov rdx, r14
1405a5eb6: call 0x142923c00 ; 
1405a5ebb: lea rcx, [rbx + 0x2300]
1405a5ec2: lea r15, [rbx + 0x1ff0]
1405a5ec9: mov r8d, 0xf0
1405a5ecf: mov rdx, r15
1405a5ed2: call 0x142923c00 ; 
1405a5ed7: lea rcx, [rbx + 0x23f0]
1405a5ede: lea rdx, [rbx + 0xb00]
1405a5ee5: mov r8d, 0xf0
1405a5eeb: call 0x142923c00 ; 
1405a5ef0: movaps xmm0, xmmword ptr [rbx + 0x350]
1405a5ef7: movaps xmm1, xmmword ptr [rbx + 0x360]
1405a5efe: movaps xmmword ptr [rbx + 0x380], xmm1
1405a5f05: movaps xmmword ptr [rbx + 0x370], xmm0
1405a5f0c: mov r12, qword ptr [rbp + 0x37a8]
1405a5f13: lea rdx, [rbx + 0x20e0]
1405a5f1a: mov r8d, 0xa8
1405a5f20: mov rcx, r15
1405a5f23: call 0x142923c00 ; 
1405a5f28: mov r8, qword ptr [rbx + 0x340]
1405a5f2f: mov r9, qword ptr [rbx + 0x348]
1405a5f36: movups xmm0, xmmword ptr [rbx + 0x270]
1405a5f3d: movups xmm1, xmmword ptr [rbx + 0x280]
1405a5f44: mov rax, qword ptr [rbx + 0x170]
1405a5f4b: mov byte ptr [rbx + 0x105], 0
1405a5f52: mov qword ptr [rsp + 0x68], r15
1405a5f57: mov qword ptr [rsp + 0x60], r12
1405a5f5c: mov qword ptr [rsp + 0x58], r14
1405a5f61: mov qword ptr [rsp + 0x50], rsi
1405a5f66: mov qword ptr [rsp + 0x48], rdi
1405a5f6b: mov qword ptr [rsp + 0x40], rax
1405a5f70: movups xmmword ptr [rsp + 0x30], xmm1
1405a5f75: movups xmmword ptr [rsp + 0x20], xmm0
1405a5f7a: lea rcx, [rbx + 0xb00]
1405a5f81: lea rdx, [rbx + 0x440]
1405a5f88: call 0x1404d8100 ; tiny_glade::game_loop::GameLoop::new
1405a5f8d: mov byte ptr [rbx + 0xf5], 0
1405a5f94: mov byte ptr [rbx + 0xf6], 0
1405a5f9b: mov byte ptr [rbx + 0xf7], 0
1405a5fa2: lea rcx, [rbx + 0xb00]
1405a5fa9: call 0x1404d8250 ; tiny_glade::game_loop::GameLoop::run
1405a5fae: mov al, 1
1405a5fb0: mov dword ptr [rbx + 0x128], eax
1405a5fb6: mov dword ptr [rbx + 0xe8], 0
1405a5fc0: jmp 0x1405a61bc
1405a5fc5: mov rcx, qword ptr [rbx + 0x168]
1405a5fcc: lock inc qword ptr [rcx + 0x70]
1405a5fd1: jg 0x1405a5ff1
1405a5fd3: mov ecx, 7
1405a5fd8: int 0x29
1405a5fda: ud2 
1405a5fdc: mov rcx, qword ptr [rbx + 0x168]
1405a5fe3: lock inc qword ptr [rcx + 0x200]
1405a5feb: jle 0x1405a7042
1405a5ff1: mov qword ptr [rbx + 0x370], rax
1405a5ff8: mov qword ptr [rbx + 0x378], rcx
1405a5fff: mov rax, qword ptr [rbx + 0x140]
1405a6006: lock inc qword ptr [rax]
1405a600a: jle 0x1405a7040
1405a6010: mov qword ptr [rbx + 0x270], rax
1405a6017: lea rsi, [rbx + 0x1ff0]
1405a601e: lea rdx, [rbx + 0x2188]
1405a6025: mov r8d, 0xa8
1405a602b: mov rcx, rsi
1405a602e: call 0x142923c00 ; 
1405a6033: mov rdx, qword ptr [rbx + 0x138]
1405a603a: add rdx, 0x10
1405a603e: mov byte ptr [rbx + 0x106], 1
1405a6045: lea rcx, [rbx + 0x2230]
1405a604c: call 0x140bea2d0 ; rhapsody::rg::executor::GraphExecutor::new
1405a6051: lea rcx, [rbx + 0x2bf0]
1405a6058: call 0x140be57d0 ; rhapsody::rg::state::impl$0::default
1405a605d: lea rcx, [rbx + 0xb00]
1405a6064: call 0x140be57d0 ; rhapsody::rg::state::impl$0::default
1405a6069: lea rdi, [rbx + 0x2230]
1405a6070: mov r8d, 0xd0
1405a6076: mov rcx, r14
1405a6079: mov rdx, rdi
1405a607c: call 0x142923c00 ; 
1405a6081: lea rcx, [rbx + 0x2660]
1405a6088: lea r15, [rbx + 0x2bf0]
1405a608f: mov r8d, 0xf0
1405a6095: mov rdx, r15
1405a6098: call 0x142923c00 ; 
1405a609d: lea rcx, [rbx + 0x2750]
1405a60a4: lea rdx, [rbx + 0xb00]
1405a60ab: mov r8d, 0xf0
1405a60b1: call 0x142923c00 ; 
1405a60b6: movaps xmm0, xmmword ptr [rbx + 0x350]
1405a60bd: movaps xmm1, xmmword ptr [rbx + 0x360]
1405a60c4: movaps xmmword ptr [rbx + 0x2240], xmm1
1405a60cb: movaps xmmword ptr [rbx + 0x2230], xmm0
1405a60d2: mov r12, qword ptr [rbp + 0x37a8]
1405a60d9: lea rdx, [rbx + 0x20e0]
1405a60e0: mov r8d, 0xa8
1405a60e6: mov rcx, r15
1405a60e9: call 0x142923c00 ; 
1405a60ee: mov r8, qword ptr [rbx + 0x280]
1405a60f5: mov r9, qword ptr [rbx + 0x288]
1405a60fc: movups xmm0, xmmword ptr [rbx + 0x24e8]
1405a6103: movups xmm1, xmmword ptr [rbx + 0x370]
1405a610a: mov rax, qword ptr [rbx + 0x270]
1405a6111: mov byte ptr [rbx + 0x106], 0
1405a6118: mov qword ptr [rsp + 0x68], r15
1405a611d: mov qword ptr [rsp + 0x60], r12
1405a6122: mov qword ptr [rsp + 0x58], rdi
1405a6127: mov qword ptr [rsp + 0x50], r14
1405a612c: mov qword ptr [rsp + 0x48], rsi
1405a6131: mov qword ptr [rsp + 0x40], rax
1405a6136: movups xmmword ptr [rsp + 0x30], xmm1
1405a613b: movups xmmword ptr [rsp + 0x20], xmm0
1405a6140: lea rcx, [rbx + 0xb00]
1405a6147: lea rdx, [rbx + 0x440]
1405a614e: call 0x1404d8100 ; tiny_glade::game_loop::GameLoop::new
1405a6153: mov byte ptr [rbx + 0xc2], 0
1405a615a: mov byte ptr [rbx + 0xc3], 0
1405a6161: mov byte ptr [rbx + 0xc4], 1
1405a6168: mov byte ptr [rbx + 0xc5], 1
1405a616f: mov byte ptr [rbx + 0xc6], 0
1405a6176: lea rcx, [rbx + 0xb00]
1405a617d: call 0x1404d8250 ; tiny_glade::game_loop::GameLoop::run
1405a6182: mov byte ptr [rbx + 0xe4], 0
1405a6189: mov byte ptr [rbx + 0xe5], 0
1405a6190: mov byte ptr [rbx + 0xe6], 1
1405a6197: mov byte ptr [rbx + 0xe7], 0
1405a619e: lea rcx, [rbx + 0x2e0]
1405a61a5: call 0x140601d60 ; core::ptr::drop_in_place<std::sync::mpsc::Sender<bevy_app::sub_app::SubApp> >
1405a61aa: mov al, 1
1405a61ac: mov dword ptr [rbx + 0xe8], eax
1405a61b2: mov dword ptr [rbx + 0x128], 0
1405a61bc: mov rcx, qword ptr [rbx + 0x2a0]
1405a61c3: call qword ptr [rip + 0x2bec227]
1405a61c9: mov rax, qword ptr [rbx + 0x290]
1405a61d0: lock dec qword ptr [rax]
1405a61d4: jne 0x1405a61e2
1405a61d6: lea rcx, [rbx + 0x290]
1405a61dd: call 0x140d53b60 ; alloc::sync::Arc::drop_slow<std::thread::Inner,alloc::alloc::Global>
1405a61e2: mov rax, qword ptr [rbx + 0x298]
1405a61e9: lock dec qword ptr [rax]
1405a61ed: mov edi, dword ptr [rbx + 0xe8]
1405a61f3: mov r14d, dword ptr [rbx + 0x128]
1405a61fa: jne 0x1405a622b
1405a61fc: lea rcx, [rbx + 0x298]
1405a6203: mov byte ptr [rbx + 0xd4], 0
1405a620a: mov byte ptr [rbx + 0xd5], r14b
1405a6211: mov byte ptr [rbx + 0xd6], dil
1405a6218: mov byte ptr [rbx + 0xd7], 0
1405a621f: mov byte ptr [rbx + 0xd8], 0
1405a6226: call 0x140d53db0 ; alloc::sync::Arc::drop_slow<std::thread::Packet<tuple$<> >,alloc::alloc::Global>
1405a622b: mov rax, qword ptr [rbx + 0x320]
1405a6232: test rax, rax
1405a6235: mov esi, dword ptr [rbx + 0x130]
1405a623b: je 0x1405a628e
1405a623d: lock dec qword ptr [rax]
1405a6241: jne 0x1405a628e
1405a6243: mov byte ptr [rbx + 0xaa], r14b
1405a624a: mov byte ptr [rbx + 0xab], sil
1405a6251: mov byte ptr [rbx + 0xac], dil
1405a6258: mov byte ptr [rbx + 0xad], 0
1405a625f: mov byte ptr [rbx + 0xae], 0
1405a6266: mov byte ptr [rbx + 0xaf], 0
1405a626d: mov byte ptr [rbx + 0xb0], 0
1405a6274: mov byte ptr [rbx + 0xb1], 0
1405a627b: mov byte ptr [rbx + 0xb2], 0
1405a6282: lea rcx, [rbx + 0x320]
1405a6289: call 0x1406978e0 ; alloc::sync::Arc::drop_slow<parking::Inner,alloc::alloc::Global>
1405a628e: mov rax, qword ptr [rbx + 0x138]
1405a6295: lock dec qword ptr [rax]
1405a6299: jne 0x1405a62d1
1405a629b: mov byte ptr [rbx + 0xf8], r14b
1405a62a2: mov byte ptr [rbx + 0xf9], sil
1405a62a9: mov byte ptr [rbx + 0xfa], dil
1405a62b0: mov byte ptr [rbx + 0xfb], 0
1405a62b7: mov byte ptr [rbx + 0xfc], 0
1405a62be: mov byte ptr [rbx + 0xfd], 0
1405a62c5: lea rcx, [rbx + 0x138]
1405a62cc: call 0x140b5be90 ; alloc::sync::Arc::drop_slow<rhapsody::raw::device::GraphicsDevice,alloc::alloc::Global>
1405a62d1: test r13b, r13b
1405a62d4: je 0x1405a6305
1405a62d6: mov byte ptr [rbx + 0x115], dil
1405a62dd: mov byte ptr [rbx + 0x116], r14b
1405a62e4: mov byte ptr [rbx + 0x117], sil
1405a62eb: mov byte ptr [rbx + 0x118], 0
1405a62f2: mov byte ptr [rbx + 0x119], 0
1405a62f9: lea rcx, [rbx + 0x160]
1405a6300: call 0x140606870 ; core::ptr::drop_in_place<std::sync::mpsc::Sender<enum2$<tiny_glade::event_loop::GameEvent> > >
1405a6305: mov byte ptr [rbx + 0x120], dil
1405a630c: mov byte ptr [rbx + 0x121], r14b
1405a6313: mov byte ptr [rbx + 0x122], sil
1405a631a: mov byte ptr [rbx + 0x123], 0
1405a6321: mov rcx, qword ptr [rbp + 0x37c0]
1405a6328: call 0x1405ee980 ; core::ptr::drop_in_place<tiny_glade::app_config::CliArgs>
1405a632d: mov byte ptr [rbx + 0x124], dil
1405a6334: mov byte ptr [rbx + 0x125], r14b
1405a633b: mov byte ptr [rbx + 0x126], sil
1405a6342: mov byte ptr [rbx + 0x127], 0
1405a6349: mov rcx, qword ptr [rbp + 0x37b8]
1405a6350: call 0x1405f18c0 ; core::ptr::drop_in_place<tiny_glade::event_loop::EguiEventIo>
1405a6355: test sil, sil
1405a6358: mov rcx, qword ptr [rbp + 0x37b0]
1405a635f: je 0x1405a639d
1405a6361: call 0x1405fdb80 ; core::ptr::drop_in_place<country_core::render::render_config::VideoSettings>
1405a6366: mov rcx, qword ptr [rbp + 0x37b0]
1405a636d: mov rdx, qword ptr [rcx + 0xc8]
1405a6374: mov rax, rdx
1405a6377: neg rax
1405a637a: mov edi, dword ptr [rbx + 0xe8]
1405a6380: mov r14d, dword ptr [rbx + 0x128]
1405a6387: jo 0x1405a639d
1405a6389: jae 0x1405a639d
1405a638b: mov rcx, qword ptr [rcx + 0xd0]
1405a6392: mov r8d, 1
1405a6398: call 0x140613c20 ; 
1405a639d: test r14b, r14b
1405a63a0: je 0x1405a63b5
1405a63a2: mov byte ptr [rbx + 0x11d], dil
1405a63a9: mov rcx, qword ptr [rbx + 0xc8]
1405a63b0: call 0x1405e6cd0 ; core::ptr::drop_in_place<bevy_app::app::App>
1405a63b5: test r13b, r13b
1405a63b8: je 0x1405a63d3
1405a63ba: mov rax, qword ptr [rbx + 0x140]
1405a63c1: lock dec qword ptr [rax]
1405a63c5: jne 0x1405a63d3
1405a63c7: lea rcx, [rbx + 0x140]
1405a63ce: call 0x140631840 ; alloc::sync::Arc::drop_slow<winit::window::Window,alloc::alloc::Global>
1405a63d3: lea rcx, [rbx + 0x2c0]
1405a63da: call 0x1405d3280 ; core::ptr::drop_in_place<std::sync::mpsc::Receiver<country_core::systems::audio::AsyncLoadedAudioResources> >
1405a63df: movaps xmm6, xmmword ptr [rbp + 0x3690]
1405a63e6: movaps xmm7, xmmword ptr [rbp + 0x36a0]
1405a63ed: movaps xmm8, xmmword ptr [rbp + 0x36b0]
1405a63f5: movaps xmm9, xmmword ptr [rbp + 0x36c0]
1405a63fd: movaps xmm10, xmmword ptr [rbp + 0x36d0]
1405a6405: movaps xmm11, xmmword ptr [rbp + 0x36e0]
1405a640d: movaps xmm12, xmmword ptr [rbp + 0x36f0]
1405a6415: movaps xmm13, xmmword ptr [rbp + 0x3700]
1405a641d: movaps xmm14, xmmword ptr [rbp + 0x3710]
1405a6425: movaps xmm15, xmmword ptr [rbp + 0x3720]
1405a642d: lea rsp, [rbp + 0x3738]
1405a6434: pop rbx
1405a6435: pop rdi
1405a6436: pop rsi
1405a6437: pop r12
1405a6439: pop r13
1405a643b: pop r14
1405a643d: pop r15
1405a643f: pop rbp
1405a6440: ret 
1405a6441: movaps xmm0, xmmword ptr [rbx + 0x1ff0]
1405a6448: movaps xmmword ptr [rbx + 0x440], xmm0
1405a644f: movaps xmm0, xmmword ptr [rbx + 0x2bf0]
1405a6456: movaps xmm1, xmmword ptr [rbx + 0x2c00]
1405a645d: movaps xmmword ptr [rbx + 0xb00], xmm0
1405a6464: movaps xmmword ptr [rbx + 0xb10], xmm1
1405a646b: dec r8
1405a646e: add rdx, r8
1405a6471: cmovae rdx, r8
1405a6475: mov qword ptr [rbx + 0x2240], rdx
1405a647c: mov qword ptr [rbx + 0x2248], rax
1405a6483: imul rax, rdx, 0x38
1405a6487: mov qword ptr [rcx + rax], r9
1405a648b: movaps xmm0, xmmword ptr [rbx + 0x440]
1405a6492: movups xmmword ptr [rcx + rax + 8], xmm0
1405a6497: movaps xmm0, xmmword ptr [rbx + 0xb00]
1405a649e: movaps xmm1, xmmword ptr [rbx + 0xb10]
1405a64a5: movups xmmword ptr [rcx + rax + 0x28], xmm1
1405a64aa: movups xmmword ptr [rcx + rax + 0x18], xmm0
1405a64af: movups xmm0, xmmword ptr [rbx + 0x2230]
1405a64b6: mov rax, qword ptr [rbx + 0x2240]
1405a64bd: mov qword ptr [rbx + 0x450], rax
1405a64c4: mov rax, qword ptr [rbx + 0x2248]
1405a64cb: mov qword ptr [rbx + 0x458], rax
1405a64d2: movaps xmmword ptr [rbx + 0x440], xmm0
1405a64d9: mov byte ptr [rbx + 0xd9], 0
1405a64e0: mov byte ptr [rbx + 0xda], 0
1405a64e7: lea rcx, [rbx + 0xb00]
1405a64ee: lea rdx, [rbx + 0x440]
1405a64f5: call 0x14091eff0 ; country_core::meta_commander::MetaCommander::new
1405a64fa: mov byte ptr [rbx + 0xd9], 0
1405a6501: mov byte ptr [rbx + 0xda], 0
1405a6508: lea rdx, [rbx + 0xb00]
1405a650f: mov rcx, qword ptr [rbx + 0xc8]
1405a6516: call 0x140118630 ; bevy_app::app::App::insert_resource<country_core::meta_commander::MetaCommander>
1405a651b: jmp 0x1405a30e0
1405a6520: lea rax, [rip + 0x2458681]
1405a6527: mov qword ptr [rbx + 0xb00], rax
1405a652e: mov qword ptr [rbx + 0xb08], 1
1405a6539: mov qword ptr [rbx + 0xb10], 8
1405a6544: movups xmmword ptr [rbx + 0xb18], xmm6
1405a654b: mov byte ptr [rbx + 0xbc], 1
1405a6552: mov byte ptr [rbx + 0xbd], 1
1405a6559: mov byte ptr [rbx + 0xbe], 1
1405a6560: mov byte ptr [rbx + 0xbf], 1
1405a6567: mov byte ptr [rbx + 0xc0], 1
1405a656e: mov byte ptr [rbx + 0xc1], 1
1405a6575: lea rdx, [rip + 0x245869c]
1405a657c: lea rcx, [rbx + 0xb00]
1405a6583: call 0x1428d9390 ; core::panicking::panic_fmt
1405a6588: jmp 0x1405a7040
1405a658d: mov byte ptr [rbx + 0xe3], 1
1405a6594: lea rcx, [rip + 0x24593fd]
1405a659b: lea r8, [rip + 0x245941e]
1405a65a2: mov edx, 0x21
1405a65a7: call 0x1428d9430 ; core::panicking::panic
1405a65ac: jmp 0x1405a7040
1405a65b1: mov eax, 0x3b
1405a65b6: lea rcx, [rip + 0x2458243]
1405a65bd: mov qword ptr [rbx + 0xb00], rcx
1405a65c4: mov qword ptr [rbx + 0xb08], rax
1405a65cb: mov byte ptr [rbx + 0xd9], 1
1405a65d2: mov byte ptr [rbx + 0xda], 1
1405a65d9: lea rdx, [rip + 0x24582f0]
1405a65e0: lea rcx, [rbx + 0xb00]
1405a65e7: call 0x1428ce090 ; core::time::impl$0::from_secs_f32::panic_cold_display<ref$<str$> >
1405a65ec: jmp 0x1405a7040
1405a65f1: mov byte ptr [rbx + 0xc2], 1
1405a65f8: mov byte ptr [rbx + 0xc3], 1
1405a65ff: mov byte ptr [rbx + 0xc4], 1
1405a6606: mov byte ptr [rbx + 0xc5], 1
1405a660d: mov byte ptr [rbx + 0xc6], 1
1405a6614: lea rcx, [rip + 0x245951d]
1405a661b: call 0x1428d9310 ; core::option::unwrap_failed
1405a6620: jmp 0x1405a7040
1405a6625: lea rax, [rip + 0x2458b74]
1405a662c: mov qword ptr [rbx + 0x2590], rax
1405a6633: mov qword ptr [rbx + 0x2598], 0x2b
1405a663e: mov qword ptr [rbx + 0x2bf0], r14
1405a6645: lea rax, [rip - 0x49c7bc]
1405a664c: mov qword ptr [rbx + 0x2bf8], rax
1405a6653: lea rax, [rip + 0x2458b26]
1405a665a: mov qword ptr [rbx + 0xb00], rax
1405a6661: mov qword ptr [rbx + 0xb08], 2
1405a666c: mov qword ptr [rbx + 0xb20], 0
1405a6677: lea rax, [rbx + 0x2bf0]
1405a667e: mov qword ptr [rbx + 0xb10], rax
1405a6685: mov qword ptr [rbx + 0xb18], 1
1405a6690: mov byte ptr [rbx + 0xb3], 1
1405a6697: lea rdx, [rip + 0x24594ca]
1405a669e: lea rcx, [rbx + 0xb00]
1405a66a5: call 0x1428d9390 ; core::panicking::panic_fmt
1405a66aa: jmp 0x1405a7040
1405a66af: mov byte ptr [rbx + 0xc2], 1
1405a66b6: mov byte ptr [rbx + 0xc3], 1
1405a66bd: mov byte ptr [rbx + 0xc4], 1
1405a66c4: mov byte ptr [rbx + 0xc5], 1
1405a66cb: mov byte ptr [rbx + 0xc6], 1
1405a66d2: lea rcx, [rip + 0x2459477]
1405a66d9: call 0x1428d9310 ; core::option::unwrap_failed
1405a66de: jmp 0x1405a7040
1405a66e3: lea rax, [rip + 0x2458956]
1405a66ea: mov qword ptr [rbx + 0x2590], rax
1405a66f1: mov qword ptr [rbx + 0x2598], 0x36
1405a66fc: mov qword ptr [rbx + 0x2bf0], r14
1405a6703: lea rax, [rip - 0x49c87a]
1405a670a: mov qword ptr [rbx + 0x2bf8], rax
1405a6711: lea rax, [rip + 0x2458a68]
1405a6718: mov qword ptr [rbx + 0xb00], rax
1405a671f: mov qword ptr [rbx + 0xb08], 2
1405a672a: mov qword ptr [rbx + 0xb20], 0
1405a6735: lea rax, [rbx + 0x2bf0]
1405a673c: mov qword ptr [rbx + 0xb10], rax
1405a6743: mov qword ptr [rbx + 0xb18], 1
1405a674e: mov byte ptr [rbx + 0xb3], 1
1405a6755: lea rdx, [rip + 0x2459454]
1405a675c: lea rcx, [rbx + 0xb00]
1405a6763: call 0x1428d9390 ; core::panicking::panic_fmt
1405a6768: jmp 0x1405a7040
1405a676d: lea rax, [rip + 0x2458abc]
1405a6774: mov qword ptr [rbx + 0x2590], rax
1405a677b: mov qword ptr [rbx + 0x2598], 0x40
1405a6786: mov qword ptr [rbx + 0x2bf0], r14
1405a678d: lea rax, [rip - 0x49c904]
1405a6794: mov qword ptr [rbx + 0x2bf8], rax
1405a679b: lea rax, [rip + 0x24589de]
1405a67a2: mov qword ptr [rbx + 0xb00], rax
1405a67a9: mov qword ptr [rbx + 0xb08], 2
1405a67b4: mov qword ptr [rbx + 0xb20], 0
1405a67bf: lea rax, [rbx + 0x2bf0]
1405a67c6: mov qword ptr [rbx + 0xb10], rax
1405a67cd: mov qword ptr [rbx + 0xb18], 1
1405a67d8: mov byte ptr [rbx + 0xb3], 1
1405a67df: lea rdx, [rip + 0x24593e2]
1405a67e6: lea rcx, [rbx + 0xb00]
1405a67ed: call 0x1428d9390 ; core::panicking::panic_fmt
1405a67f2: jmp 0x1405a7040
1405a67f7: lea rax, [rip + 0x2458a9d]
1405a67fe: mov qword ptr [rbx + 0x2590], rax
1405a6805: mov qword ptr [rbx + 0x2598], 0x25
1405a6810: mov qword ptr [rbx + 0x2bf0], r14
1405a6817: lea rax, [rip - 0x49c98e]
1405a681e: mov qword ptr [rbx + 0x2bf8], rax
1405a6825: lea rax, [rip + 0x2458954]
1405a682c: mov qword ptr [rbx + 0xb00], rax
1405a6833: mov qword ptr [rbx + 0xb08], 2
1405a683e: mov qword ptr [rbx + 0xb20], 0
1405a6849: lea rax, [rbx + 0x2bf0]
1405a6850: mov qword ptr [rbx + 0xb10], rax
1405a6857: mov qword ptr [rbx + 0xb18], 1
1405a6862: mov byte ptr [rbx + 0xb3], 1
1405a6869: lea rdx, [rip + 0x2459370]
1405a6870: lea rcx, [rbx + 0xb00]
1405a6877: call 0x1428d9390 ; core::panicking::panic_fmt
1405a687c: jmp 0x1405a7040
1405a6881: lea rax, [rip + 0x2458943]
1405a6888: mov qword ptr [rbx + 0x2590], rax
1405a688f: mov qword ptr [rbx + 0x2598], 0x33
1405a689a: mov qword ptr [rbx + 0x2bf0], r14
1405a68a1: lea rax, [rip - 0x49ca18]
1405a68a8: mov qword ptr [rbx + 0x2bf8], rax
1405a68af: lea rax, [rip + 0x24588ca]
1405a68b6: mov qword ptr [rbx + 0xb00], rax
1405a68bd: mov qword ptr [rbx + 0xb08], 2
1405a68c8: mov qword ptr [rbx + 0xb20], 0
1405a68d3: lea rax, [rbx + 0x2bf0]
1405a68da: mov qword ptr [rbx + 0xb10], rax
1405a68e1: mov qword ptr [rbx + 0xb18], 1
1405a68ec: mov byte ptr [rbx + 0xb3], 1
1405a68f3: lea rdx, [rip + 0x24592fe]
1405a68fa: lea rcx, [rbx + 0xb00]
1405a6901: call 0x1428d9390 ; core::panicking::panic_fmt
1405a6906: jmp 0x1405a7040
1405a690b: lea rax, [rip + 0x24588ec]
1405a6912: mov qword ptr [rbx + 0x2590], rax
1405a6919: mov qword ptr [rbx + 0x2598], 0x32
1405a6924: mov qword ptr [rbx + 0x2bf0], r14
1405a692b: lea rax, [rip - 0x49caa2]
1405a6932: mov qword ptr [rbx + 0x2bf8], rax
1405a6939: lea rax, [rip + 0x2458840]
1405a6940: mov qword ptr [rbx + 0xb00], rax
1405a6947: mov qword ptr [rbx + 0xb08], 2
1405a6952: mov qword ptr [rbx + 0xb20], 0
1405a695d: lea rax, [rbx + 0x2bf0]
1405a6964: mov qword ptr [rbx + 0xb10], rax
1405a696b: mov qword ptr [rbx + 0xb18], 1
1405a6976: mov byte ptr [rbx + 0xb3], 1
1405a697d: lea rdx, [rip + 0x245928c]
1405a6984: lea rcx, [rbx + 0xb00]
1405a698b: call 0x1428d9390 ; core::panicking::panic_fmt
1405a6990: jmp 0x1405a7040
1405a6995: lea rax, [rip + 0x24588d4]
1405a699c: mov qword ptr [rbx + 0x2590], rax
1405a69a3: mov qword ptr [rbx + 0x2598], 0x2b
1405a69ae: lea rax, [rbx + 0x2590]
1405a69b5: mov qword ptr [rbx + 0x2bf0], rax
1405a69bc: lea rax, [rip - 0x49cb33]
1405a69c3: mov qword ptr [rbx + 0x2bf8], rax
1405a69ca: lea rax, [rip + 0x24587af]
1405a69d1: mov qword ptr [rbx + 0xb00], rax
1405a69d8: mov qword ptr [rbx + 0xb08], 2
1405a69e3: mov qword ptr [rbx + 0xb20], 0
1405a69ee: lea rax, [rbx + 0x2bf0]
1405a69f5: mov qword ptr [rbx + 0xb10], rax
1405a69fc: mov qword ptr [rbx + 0xb18], 1
1405a6a07: mov byte ptr [rbx + 0xb3], 0
1405a6a0e: lea rdx, [rip + 0x2459213]
1405a6a15: lea rcx, [rbx + 0xb00]
1405a6a1c: call 0x1428d9390 ; core::panicking::panic_fmt
1405a6a21: jmp 0x1405a7040
1405a6a26: mov ecx, 8
1405a6a2b: mov edx, 0xaf0
1405a6a30: call 0x1428d9000 ; alloc::alloc::handle_alloc_error
1405a6a35: jmp 0x1405a7040
1405a6a3a: mov rax, qword ptr [rbx + 0x4b0]
1405a6a41: mov qword ptr [rbx + 0xb70], rax
1405a6a48: movups xmm0, xmmword ptr [rbx + 0x4a0]
1405a6a4f: movaps xmmword ptr [rbx + 0xb60], xmm0
1405a6a56: movups xmm0, xmmword ptr [rbx + 0x490]
1405a6a5d: movaps xmmword ptr [rbx + 0xb50], xmm0
1405a6a64: movups xmm0, xmmword ptr [rbx + 0x480]
1405a6a6b: movaps xmmword ptr [rbx + 0xb40], xmm0
1405a6a72: movupd xmm0, xmmword ptr [rbx + 0x440]
1405a6a7a: movups xmm1, xmmword ptr [rbx + 0x450]
1405a6a81: movups xmm2, xmmword ptr [rbx + 0x460]
1405a6a88: movups xmm3, xmmword ptr [rbx + 0x470]
1405a6a8f: movaps xmmword ptr [rbx + 0xb30], xmm3
1405a6a96: movaps xmmword ptr [rbx + 0xb20], xmm2
1405a6a9d: movaps xmmword ptr [rbx + 0xb10], xmm1
1405a6aa4: movapd xmmword ptr [rbx + 0xb00], xmm0
1405a6aac: lea rax, [rip + 0x2458ca5]
1405a6ab3: mov qword ptr [rsp + 0x20], rax
1405a6ab8: lea rcx, [rip + 0x2457ef1]
1405a6abf: lea r9, [rip + 0x2457f5a]
1405a6ac6: lea r8, [rbx + 0xb00]
1405a6acd: mov edx, 0x2b
1405a6ad2: call 0x1428d9760 ; core::result::unwrap_failed
1405a6ad7: jmp 0x1405a7040
1405a6adc: lea rcx, [rip + 0x238bf2d]
1405a6ae3: call 0x1428d80d0 ; std::thread::local::panic_access_error
1405a6ae8: jmp 0x1405a7040
1405a6aed: mov byte ptr [rbx + 0x108], 1
1405a6af4: mov byte ptr [rbx + 0x109], 1
1405a6afb: mov byte ptr [rbx + 0x10a], 1
1405a6b02: mov byte ptr [rbx + 0x10b], 1
1405a6b09: mov byte ptr [rbx + 0x10c], 1
1405a6b10: mov byte ptr [rbx + 0x10d], 1
1405a6b17: mov byte ptr [rbx + 0x10e], 1
1405a6b1e: mov byte ptr [rbx + 0x10f], 1
1405a6b25: mov ecx, 8
1405a6b2a: mov edx, 0xb0
1405a6b2f: call 0x1428d9000 ; alloc::alloc::handle_alloc_error
1405a6b34: jmp 0x1405a7040
1405a6b39: lea rax, [rip + 0x23a9cd0]
1405a6b40: mov qword ptr [rbx + 0xb00], rax
1405a6b47: mov qword ptr [rbx + 0xb08], 1
1405a6b52: mov qword ptr [rbx + 0xb10], 8
1405a6b5d: xorps xmm0, xmm0
1405a6b60: movups xmmword ptr [rbx + 0xb18], xmm0
1405a6b67: mov byte ptr [rbx + 0x110], 1
1405a6b6e: lea rdx, [rip + 0x2458c5b]
1405a6b75: lea rcx, [rbx + 0xb00]
1405a6b7c: call 0x1428d9390 ; core::panicking::panic_fmt
1405a6b81: jmp 0x1405a7040
1405a6b86: lea rax, [rip + 0x23a9c83]
1405a6b8d: mov qword ptr [rbx + 0xb00], rax
1405a6b94: mov qword ptr [rbx + 0xb08], 1
1405a6b9f: mov qword ptr [rbx + 0xb10], 8
1405a6baa: xorps xmm0, xmm0
1405a6bad: movups xmmword ptr [rbx + 0xb18], xmm0
1405a6bb4: mov byte ptr [rbx + 0xaa], 1
1405a6bbb: mov byte ptr [rbx + 0xab], 1
1405a6bc2: mov byte ptr [rbx + 0xac], 1
1405a6bc9: mov byte ptr [rbx + 0xad], 1
1405a6bd0: mov byte ptr [rbx + 0xae], 1
1405a6bd7: mov byte ptr [rbx + 0xaf], 1
1405a6bde: mov byte ptr [rbx + 0xb0], 1
1405a6be5: mov byte ptr [rbx + 0xb1], 1
1405a6bec: mov byte ptr [rbx + 0xb2], 1
1405a6bf3: lea rdx, [rip + 0x2458bee]
1405a6bfa: lea rcx, [rbx + 0xb00]
1405a6c01: call 0x1428d9390 ; core::panicking::panic_fmt
1405a6c06: jmp 0x1405a7040
1405a6c0b: lea rax, [rip + 0x23a9bfe]
1405a6c12: mov qword ptr [rbx + 0xb00], rax
1405a6c19: mov qword ptr [rbx + 0xb08], 1
1405a6c24: mov qword ptr [rbx + 0xb10], 8
1405a6c2f: xorps xmm0, xmm0
1405a6c32: movups xmmword ptr [rbx + 0xb18], xmm0
1405a6c39: mov byte ptr [rbx + 0xaa], 1
1405a6c40: mov byte ptr [rbx + 0xab], 1
1405a6c47: mov byte ptr [rbx + 0xac], 1
1405a6c4e: mov byte ptr [rbx + 0xad], 1
1405a6c55: mov byte ptr [rbx + 0xae], 1
1405a6c5c: mov byte ptr [rbx + 0xaf], 1
1405a6c63: mov byte ptr [rbx + 0xb0], 1
1405a6c6a: mov byte ptr [rbx + 0xb1], 1
1405a6c71: mov byte ptr [rbx + 0xb2], 1
1405a6c78: lea rdx, [rip + 0x2458b81]
1405a6c7f: lea rcx, [rbx + 0xb00]
1405a6c86: call 0x1428d9390 ; core::panicking::panic_fmt
1405a6c8b: jmp 0x1405a7040
1405a6c90: lea rax, [rip + 0x23a9b79]
1405a6c97: mov qword ptr [rbx + 0xb00], rax
1405a6c9e: mov qword ptr [rbx + 0xb08], 1
1405a6ca9: mov qword ptr [rbx + 0xb10], 8
1405a6cb4: xorps xmm0, xmm0
1405a6cb7: movups xmmword ptr [rbx + 0xb18], xmm0
1405a6cbe: mov byte ptr [rbx + 0xaa], 1
1405a6cc5: mov byte ptr [rbx + 0xab], 1
1405a6ccc: mov byte ptr [rbx + 0xac], 1
1405a6cd3: mov byte ptr [rbx + 0xad], 1
1405a6cda: mov byte ptr [rbx + 0xae], 1
1405a6ce1: mov byte ptr [rbx + 0xaf], 1
1405a6ce8: mov byte ptr [rbx + 0xb0], 1
1405a6cef: mov byte ptr [rbx + 0xb1], 1
1405a6cf6: mov byte ptr [rbx + 0xb2], 1
1405a6cfd: lea rdx, [rip + 0x2458b14]
1405a6d04: lea rcx, [rbx + 0xb00]
1405a6d0b: call 0x1428d9390 ; core::panicking::panic_fmt
1405a6d10: jmp 0x1405a7040
1405a6d15: mov byte ptr [rbx + 0xaa], 1
1405a6d1c: mov byte ptr [rbx + 0xab], 1
1405a6d23: mov byte ptr [rbx + 0xac], 1
1405a6d2a: mov byte ptr [rbx + 0xad], 1
1405a6d31: mov byte ptr [rbx + 0xae], 0
1405a6d38: mov byte ptr [rbx + 0xaf], 1
1405a6d3f: mov byte ptr [rbx + 0xb0], 0
1405a6d46: mov byte ptr [rbx + 0xb1], 1
1405a6d4d: mov byte ptr [rbx + 0xb2], 1
1405a6d54: lea rax, [rip + 0x2458b95]
1405a6d5b: mov qword ptr [rsp + 0x20], rax
1405a6d60: lea rcx, [rip + 0x2457c49]
1405a6d67: lea r9, [rip + 0x2457c02]
1405a6d6e: lea r8, [rbx + 0x370]
1405a6d75: mov edx, 0x2b
1405a6d7a: call 0x1428d9760 ; core::result::unwrap_failed
1405a6d7f: jmp 0x1405a7040
1405a6d84: lea rax, [rip + 0x23a9a85]
1405a6d8b: mov qword ptr [rbx + 0xb00], rax
1405a6d92: mov qword ptr [rbx + 0xb08], 1
1405a6d9d: mov qword ptr [rbx + 0xb10], 8
1405a6da8: xorps xmm0, xmm0
1405a6dab: movups xmmword ptr [rbx + 0xb18], xmm0
1405a6db2: mov byte ptr [rbx + 0xb4], 1
1405a6db9: mov byte ptr [rbx + 0xb5], 1
1405a6dc0: mov byte ptr [rbx + 0xb6], 1
1405a6dc7: mov byte ptr [rbx + 0xb7], 0
1405a6dce: mov byte ptr [rbx + 0xb8], 1
1405a6dd5: mov byte ptr [rbx + 0xb9], 1
1405a6ddc: mov byte ptr [rbx + 0xba], 1
1405a6de3: mov byte ptr [rbx + 0xbb], 1
1405a6dea: lea rdx, [rip + 0x2458b77]
1405a6df1: lea rcx, [rbx + 0xb00]
1405a6df8: call 0x1428d9390 ; core::panicking::panic_fmt
1405a6dfd: jmp 0x1405a7040
1405a6e02: lea r8, [rip + 0x2457c87]
1405a6e09: mov ecx, 1
1405a6e0e: mov edx, 0x12
1405a6e13: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1405a6e18: jmp 0x1405a7040
1405a6e1d: mov rax, qword ptr [rbx + 0xb08]
1405a6e24: mov qword ptr [rbx + 0x440], rax
1405a6e2b: lea rax, [rip + 0x2458cee]
1405a6e32: mov qword ptr [rsp + 0x20], rax
1405a6e37: lea rcx, [rip + 0x2458cc4]
1405a6e3e: lea r9, [rip + 0x2457acb]
1405a6e45: lea r8, [rbx + 0x440]
1405a6e4c: mov edx, 0x1a
1405a6e51: call 0x1428d9760 ; core::result::unwrap_failed
1405a6e56: jmp 0x1405a7040
1405a6e5b: mov ecx, 0x80
1405a6e60: mov edx, 0x200
1405a6e65: call 0x1428d9000 ; alloc::alloc::handle_alloc_error
1405a6e6a: jmp 0x1405a7040
1405a6e6f: mov r8, rsi
1405a6e72: mov qword ptr [rbx + 0x440], r8
1405a6e79: lea rax, [rip + 0x2458b00]
1405a6e80: mov qword ptr [rsp + 0x20], rax
1405a6e85: lea rcx, [rip + 0x2457b24]
1405a6e8c: lea r9, [rip + 0x2457afd]
1405a6e93: lea r8, [rbx + 0x440]
1405a6e9a: mov edx, 0x2b
1405a6e9f: call 0x1428d9760 ; core::result::unwrap_failed
1405a6ea4: jmp 0x1405a7040
1405a6ea9: mov qword ptr [rbx + 0xb00], rsi
1405a6eb0: lea rax, [rip + 0x2458b21]
1405a6eb7: mov qword ptr [rsp + 0x20], rax
1405a6ebc: lea rcx, [rip + 0x2457aed]
1405a6ec3: lea r9, [rip + 0x2457a46]
1405a6eca: lea r8, [rbx + 0xb00]
1405a6ed1: mov edx, 0x2b
1405a6ed6: call 0x1428d9760 ; core::result::unwrap_failed
1405a6edb: jmp 0x1405a7040
1405a6ee0: mov qword ptr [rbx + 0x440], rdx
1405a6ee7: lea rax, [rip + 0x2458b22]
1405a6eee: mov qword ptr [rsp + 0x20], rax
1405a6ef3: lea rcx, [rip + 0x2457ab6]
1405a6efa: lea r9, [rip + 0x2457a0f]
1405a6f01: lea r8, [rbx + 0x440]
1405a6f08: mov edx, 0x2b
1405a6f0d: call 0x1428d9760 ; core::result::unwrap_failed
1405a6f12: jmp 0x1405a7040
1405a6f17: mov qword ptr [rbx + 0x440], r15
1405a6f1e: lea rax, [rip + 0x2458b43]
1405a6f25: mov qword ptr [rsp + 0x20], rax
1405a6f2a: lea rcx, [rip + 0x2457a7f]
1405a6f31: lea r9, [rip + 0x24579d8]
1405a6f38: lea r8, [rbx + 0x440]
1405a6f3f: mov edx, 0x2b
1405a6f44: call 0x1428d9760 ; core::result::unwrap_failed
1405a6f49: jmp 0x1405a7040
1405a6f4e: lea rax, [rip + 0x23a98bb]
1405a6f55: mov qword ptr [rbx + 0xb00], rax
1405a6f5c: mov qword ptr [rbx + 0xb08], 1
1405a6f67: mov qword ptr [rbx + 0xb10], 8
1405a6f72: xorps xmm0, xmm0
1405a6f75: movups xmmword ptr [rbx + 0xb18], xmm0
1405a6f7c: mov byte ptr [rbx + 0xb3], 1
1405a6f83: lea rdx, [rip + 0x2458bf6]
1405a6f8a: lea rcx, [rbx + 0xb00]
1405a6f91: call 0x1428d9390 ; core::panicking::panic_fmt
1405a6f96: jmp 0x1405a7040
1405a6f9b: lea rax, [rip + 0x23a986e]
1405a6fa2: mov qword ptr [rbx + 0xb00], rax
1405a6fa9: mov qword ptr [rbx + 0xb08], 1
1405a6fb4: mov qword ptr [rbx + 0xb10], 8
1405a6fbf: xorps xmm0, xmm0
1405a6fc2: movups xmmword ptr [rbx + 0xb18], xmm0
1405a6fc9: mov byte ptr [rbx + 0xb3], 1
1405a6fd0: lea rdx, [rip + 0x2458bc1]
1405a6fd7: lea rcx, [rbx + 0xb00]
1405a6fde: call 0x1428d9390 ; core::panicking::panic_fmt
1405a6fe3: jmp 0x1405a7040
1405a6fe5: lea r8, [rip + 0x2457aa4]
1405a6fec: mov ecx, 1
1405a6ff1: mov edx, 0xa
1405a6ff6: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1405a6ffb: jmp 0x1405a7040
1405a6ffd: mov eax, 0x48
1405a7002: lea rcx, [rip + 0x2457832]
1405a7009: jmp 0x1405a65bd
1405a700e: mov qword ptr [rbx + 0x440], rax
1405a7015: lea rax, [rip + 0x2458a1c]
1405a701c: mov qword ptr [rsp + 0x20], rax
1405a7021: lea rcx, [rip + 0x2457988]
1405a7028: lea r9, [rip + 0x2457961]
1405a702f: lea r8, [rbx + 0x440]
1405a7036: mov edx, 0x2b
1405a703b: call 0x1428d9760 ; core::result::unwrap_failed
1405a7040: ud2 
1405a7042: mov ecx, 7
1405a7047: int 0x29
1405a7049: ud2 
1405a704b: nop dword ptr [rax + rax]
1405a7050: mov qword ptr [rsp + 0x10], rdx
1405a7055: push rbp
1405a7056: push r15
1405a7058: push r14
1405a705a: push r13
1405a705c: push r12
1405a705e: push rsi
1405a705f: push rdi
1405a7060: push rbx
1405a7061: sub rsp, 0x118
1405a7068: lea rbp, [rdx + 0x80]
1405a706f: movaps xmmword ptr [rsp + 0x70], xmm15
1405a7075: movaps xmmword ptr [rsp + 0x80], xmm14
1405a707e: movaps xmmword ptr [rsp + 0x90], xmm13
1405a7087: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a7090: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a7099: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a70a2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a70ab: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a70b4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a70bc: movaps xmmword ptr [rsp + 0x100], xmm6
1405a70c4: and rdx, 0xffffffffffffff80
1405a70c8: mov rbx, rdx
1405a70cb: lea rcx, [rbx + 0xb00]
1405a70d2: call 0x1403b60c0 ; core::ptr::drop_in_place<enum2$<country_core::resources::session::SessionOnLoadCmd> >
1405a70d7: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a70df: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a70e7: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a70f0: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a70f9: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a7102: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a710b: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a7114: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a711d: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a7126: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a712c: add rsp, 0x118
1405a7133: pop rbx
1405a7134: pop rdi
1405a7135: pop rsi
1405a7136: pop r12
1405a7138: pop r13
1405a713a: pop r14
1405a713c: pop r15
1405a713e: pop rbp
1405a713f: ret 
1405a7140: mov qword ptr [rsp + 0x10], rdx
1405a7145: push rbp
1405a7146: push r15
1405a7148: push r14
1405a714a: push r13
1405a714c: push r12
1405a714e: push rsi
1405a714f: push rdi
1405a7150: push rbx
1405a7151: sub rsp, 0x118
1405a7158: lea rbp, [rdx + 0x80]
1405a715f: movaps xmmword ptr [rsp + 0x70], xmm15
1405a7165: movaps xmmword ptr [rsp + 0x80], xmm14
1405a716e: movaps xmmword ptr [rsp + 0x90], xmm13
1405a7177: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a7180: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a7189: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a7192: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a719b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a71a4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a71ac: movaps xmmword ptr [rsp + 0x100], xmm6
1405a71b4: and rdx, 0xffffffffffffff80
1405a71b8: mov rbx, rdx
1405a71bb: mov byte ptr [rbx + 0xe3], 0
1405a71c2: lea rcx, [rbx + 0x440]
1405a71c9: call 0x1405f9a70 ; core::ptr::drop_in_place<country_core::resources::session::NextSession>
1405a71ce: mov byte ptr [rbx + 0xe3], 0
1405a71d5: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a71dd: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a71e5: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a71ee: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a71f7: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a7200: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a7209: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a7212: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a721b: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a7224: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a722a: add rsp, 0x118
1405a7231: pop rbx
1405a7232: pop rdi
1405a7233: pop rsi
1405a7234: pop r12
1405a7236: pop r13
1405a7238: pop r14
1405a723a: pop r15
1405a723c: pop rbp
1405a723d: ret 
1405a723e: nop 
1405a7240: mov qword ptr [rsp + 0x10], rdx
1405a7245: push rbp
1405a7246: push r15
1405a7248: push r14
1405a724a: push r13
1405a724c: push r12
1405a724e: push rsi
1405a724f: push rdi
1405a7250: push rbx
1405a7251: sub rsp, 0x118
1405a7258: lea rbp, [rdx + 0x80]
1405a725f: movaps xmmword ptr [rsp + 0x70], xmm15
1405a7265: movaps xmmword ptr [rsp + 0x80], xmm14
1405a726e: movaps xmmword ptr [rsp + 0x90], xmm13
1405a7277: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a7280: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a7289: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a7292: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a729b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a72a4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a72ac: movaps xmmword ptr [rsp + 0x100], xmm6
1405a72b4: and rdx, 0xffffffffffffff80
1405a72b8: mov rbx, rdx
1405a72bb: mov rcx, qword ptr [rbx + 0xe8]
1405a72c2: mov rax, qword ptr [rbx + 0x128]
1405a72c9: mov rdx, qword ptr [rax + 8]
1405a72cd: test rdx, rdx
1405a72d0: je 0x1405a72e2
1405a72d2: mov rax, qword ptr [rbx + 0x128]
1405a72d9: mov r8, qword ptr [rax + 0x10]
1405a72dd: call 0x140613c20 ; 
1405a72e2: mov edx, 0x18
1405a72e7: mov r8d, 8
1405a72ed: mov rcx, qword ptr [rbx + 0x150]
1405a72f4: call 0x140613c20 ; 
1405a72f9: mov byte ptr [rbx + 0xdb], 0
1405a7300: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a7308: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a7310: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a7319: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a7322: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a732b: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a7334: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a733d: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a7346: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a734f: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a7355: add rsp, 0x118
1405a735c: pop rbx
1405a735d: pop rdi
1405a735e: pop rsi
1405a735f: pop r12
1405a7361: pop r13
1405a7363: pop r14
1405a7365: pop r15
1405a7367: pop rbp
1405a7368: ret 
1405a7369: nop dword ptr [rax]
1405a7370: mov qword ptr [rsp + 0x10], rdx
1405a7375: push rbp
1405a7376: push r15
1405a7378: push r14
1405a737a: push r13
1405a737c: push r12
1405a737e: push rsi
1405a737f: push rdi
1405a7380: push rbx
1405a7381: sub rsp, 0x118
1405a7388: lea rbp, [rdx + 0x80]
1405a738f: movaps xmmword ptr [rsp + 0x70], xmm15
1405a7395: movaps xmmword ptr [rsp + 0x80], xmm14
1405a739e: movaps xmmword ptr [rsp + 0x90], xmm13
1405a73a7: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a73b0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a73b9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a73c2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a73cb: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a73d4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a73dc: movaps xmmword ptr [rsp + 0x100], xmm6
1405a73e4: and rdx, 0xffffffffffffff80
1405a73e8: mov rbx, rdx
1405a73eb: mov rdx, qword ptr [rbx + 0x2658]
1405a73f2: lea rax, [rdx*2]
1405a73fa: test rax, rax
1405a73fd: je 0x1405a7411
1405a73ff: mov rcx, qword ptr [rbx + 0x2660]
1405a7406: mov r8d, 1
1405a740c: call 0x140613c20 ; 
1405a7411: mov byte ptr [rbx + 0xc7], 0
1405a7418: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a7420: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a7428: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a7431: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a743a: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a7443: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a744c: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a7455: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a745e: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a7467: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a746d: add rsp, 0x118
1405a7474: pop rbx
1405a7475: pop rdi
1405a7476: pop rsi
1405a7477: pop r12
1405a7479: pop r13
1405a747b: pop r14
1405a747d: pop r15
1405a747f: pop rbp
1405a7480: ret 
1405a7481: nop word ptr cs:[rax + rax]
1405a7490: mov qword ptr [rsp + 0x10], rdx
1405a7495: push rbp
1405a7496: push r15
1405a7498: push r14
1405a749a: push r13
1405a749c: push r12
1405a749e: push rsi
1405a749f: push rdi
1405a74a0: push rbx
1405a74a1: sub rsp, 0x118
1405a74a8: lea rbp, [rdx + 0x80]
1405a74af: movaps xmmword ptr [rsp + 0x70], xmm15
1405a74b5: movaps xmmword ptr [rsp + 0x80], xmm14
1405a74be: movaps xmmword ptr [rsp + 0x90], xmm13
1405a74c7: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a74d0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a74d9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a74e2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a74eb: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a74f4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a74fc: movaps xmmword ptr [rsp + 0x100], xmm6
1405a7504: and rdx, 0xffffffffffffff80
1405a7508: mov rbx, rdx
1405a750b: lea rcx, [rbx + 0x2590]
1405a7512: call 0x1405f2450 ; core::ptr::drop_in_place<country_core::game_config::GameConfig>
1405a7517: mov byte ptr [rbx + 0xc7], 0
1405a751e: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a7526: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a752e: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a7537: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a7540: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a7549: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a7552: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a755b: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a7564: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a756d: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a7573: add rsp, 0x118
1405a757a: pop rbx
1405a757b: pop rdi
1405a757c: pop rsi
1405a757d: pop r12
1405a757f: pop r13
1405a7581: pop r14
1405a7583: pop r15
1405a7585: pop rbp
1405a7586: ret 
1405a7587: nop word ptr [rax + rax]
1405a7590: mov qword ptr [rsp + 0x10], rdx
1405a7595: push rbp
1405a7596: push r15
1405a7598: push r14
1405a759a: push r13
1405a759c: push r12
1405a759e: push rsi
1405a759f: push rdi
1405a75a0: push rbx
1405a75a1: sub rsp, 0x118
1405a75a8: lea rbp, [rdx + 0x80]
1405a75af: movaps xmmword ptr [rsp + 0x70], xmm15
1405a75b5: movaps xmmword ptr [rsp + 0x80], xmm14
1405a75be: movaps xmmword ptr [rsp + 0x90], xmm13
1405a75c7: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a75d0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a75d9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a75e2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a75eb: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a75f4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a75fc: movaps xmmword ptr [rsp + 0x100], xmm6
1405a7604: and rdx, 0xffffffffffffff80
1405a7608: mov rbx, rdx
1405a760b: mov rax, qword ptr [rbx + 0x2238]
1405a7612: lock dec qword ptr [rax]
1405a7616: jne 0x1405a762b
1405a7618: lea rcx, [rbx + 0x2238]
1405a761f: mov byte ptr [rbx + 0xdb], 0
1405a7626: call 0x140d53db0 ; alloc::sync::Arc::drop_slow<std::thread::Packet<tuple$<> >,alloc::alloc::Global>
1405a762b: mov byte ptr [rbx + 0xdb], 0
1405a7632: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a763a: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a7642: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a764b: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a7654: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a765d: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a7666: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a766f: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a7678: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a7681: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a7687: add rsp, 0x118
1405a768e: pop rbx
1405a768f: pop rdi
1405a7690: pop rsi
1405a7691: pop r12
1405a7693: pop r13
1405a7695: pop r14
1405a7697: pop r15
1405a7699: pop rbp
1405a769a: ret 
1405a769b: nop dword ptr [rax + rax]
1405a76a0: mov qword ptr [rsp + 0x10], rdx
1405a76a5: push rbp
1405a76a6: push r15
1405a76a8: push r14
1405a76aa: push r13
1405a76ac: push r12
1405a76ae: push rsi
1405a76af: push rdi
1405a76b0: push rbx
1405a76b1: sub rsp, 0x118
1405a76b8: lea rbp, [rdx + 0x80]
1405a76bf: movaps xmmword ptr [rsp + 0x70], xmm15
1405a76c5: movaps xmmword ptr [rsp + 0x80], xmm14
1405a76ce: movaps xmmword ptr [rsp + 0x90], xmm13
1405a76d7: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a76e0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a76e9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a76f2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a76fb: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a7704: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a770c: movaps xmmword ptr [rsp + 0x100], xmm6
1405a7714: and rdx, 0xffffffffffffff80
1405a7718: mov rbx, rdx
1405a771b: lea rcx, [rbx + 0x440]
1405a7722: call 0x140d4cc30 ; anyhow::error::impl$6::drop
1405a7727: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a772f: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a7737: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a7740: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a7749: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a7752: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a775b: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a7764: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a776d: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a7776: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a777c: add rsp, 0x118
1405a7783: pop rbx
1405a7784: pop rdi
1405a7785: pop rsi
1405a7786: pop r12
1405a7788: pop r13
1405a778a: pop r14
1405a778c: pop r15
1405a778e: pop rbp
1405a778f: ret 
1405a7790: mov qword ptr [rsp + 0x10], rdx
1405a7795: push rbp
1405a7796: push r15
1405a7798: push r14
1405a779a: push r13
1405a779c: push r12
1405a779e: push rsi
1405a779f: push rdi
1405a77a0: push rbx
1405a77a1: sub rsp, 0x118
1405a77a8: lea rbp, [rdx + 0x80]
1405a77af: movaps xmmword ptr [rsp + 0x70], xmm15
1405a77b5: movaps xmmword ptr [rsp + 0x80], xmm14
1405a77be: movaps xmmword ptr [rsp + 0x90], xmm13
1405a77c7: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a77d0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a77d9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a77e2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a77eb: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a77f4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a77fc: movaps xmmword ptr [rsp + 0x100], xmm6
1405a7804: and rdx, 0xffffffffffffff80
1405a7808: mov rbx, rdx
1405a780b: lea rcx, [rbx + 0x2590]
1405a7812: call 0x140d4cc30 ; anyhow::error::impl$6::drop
1405a7817: mov byte ptr [rbx + 0xc7], 1
1405a781e: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a7826: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a782e: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a7837: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a7840: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a7849: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a7852: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a785b: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a7864: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a786d: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a7873: add rsp, 0x118
1405a787a: pop rbx
1405a787b: pop rdi
1405a787c: pop rsi
1405a787d: pop r12
1405a787f: pop r13
1405a7881: pop r14
1405a7883: pop r15
1405a7885: pop rbp
1405a7886: ret 
1405a7887: nop word ptr [rax + rax]
1405a7890: mov qword ptr [rsp + 0x10], rdx
1405a7895: push rbp
1405a7896: push r15
1405a7898: push r14
1405a789a: push r13
1405a789c: push r12
1405a789e: push rsi
1405a789f: push rdi
1405a78a0: push rbx
1405a78a1: sub rsp, 0x118
1405a78a8: lea rbp, [rdx + 0x80]
1405a78af: movaps xmmword ptr [rsp + 0x70], xmm15
1405a78b5: movaps xmmword ptr [rsp + 0x80], xmm14
1405a78be: movaps xmmword ptr [rsp + 0x90], xmm13
1405a78c7: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a78d0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a78d9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a78e2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a78eb: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a78f4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a78fc: movaps xmmword ptr [rsp + 0x100], xmm6
1405a7904: and rdx, 0xffffffffffffff80
1405a7908: mov rbx, rdx
1405a790b: cmp qword ptr [rbx + 0x130], 0
1405a7913: je 0x1405a792e
1405a7915: mov r8d, 1
1405a791b: mov rcx, qword ptr [rbx + 0x128]
1405a7922: mov rdx, qword ptr [rbx + 0x130]
1405a7929: call 0x140613c20 ; 
1405a792e: mov byte ptr [rbx + 0xc7], 1
1405a7935: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a793d: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a7945: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a794e: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a7957: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a7960: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a7969: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a7972: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a797b: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a7984: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a798a: add rsp, 0x118
1405a7991: pop rbx
1405a7992: pop rdi
1405a7993: pop rsi
1405a7994: pop r12
1405a7996: pop r13
1405a7998: pop r14
1405a799a: pop r15
1405a799c: pop rbp
1405a799d: ret 
1405a799e: nop 
1405a79a0: mov qword ptr [rsp + 0x10], rdx
1405a79a5: push rbp
1405a79a6: push r15
1405a79a8: push r14
1405a79aa: push r13
1405a79ac: push r12
1405a79ae: push rsi
1405a79af: push rdi
1405a79b0: push rbx
1405a79b1: sub rsp, 0x118
1405a79b8: lea rbp, [rdx + 0x80]
1405a79bf: movaps xmmword ptr [rsp + 0x70], xmm15
1405a79c5: movaps xmmword ptr [rsp + 0x80], xmm14
1405a79ce: movaps xmmword ptr [rsp + 0x90], xmm13
1405a79d7: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a79e0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a79e9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a79f2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a79fb: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a7a04: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a7a0c: movaps xmmword ptr [rsp + 0x100], xmm6
1405a7a14: and rdx, 0xffffffffffffff80
1405a7a18: mov rbx, rdx
1405a7a1b: lea rcx, [rbx + 0x1ff0]
1405a7a22: call 0x1405f3270 ; core::ptr::drop_in_place<rhapsody::rg::state::RenderGraphState>
1405a7a27: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a7a2f: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a7a37: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a7a40: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a7a49: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a7a52: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a7a5b: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a7a64: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a7a6d: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a7a76: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a7a7c: add rsp, 0x118
1405a7a83: pop rbx
1405a7a84: pop rdi
1405a7a85: pop rsi
1405a7a86: pop r12
1405a7a88: pop r13
1405a7a8a: pop r14
1405a7a8c: pop r15
1405a7a8e: pop rbp
1405a7a8f: ret 
1405a7a90: mov qword ptr [rsp + 0x10], rdx
1405a7a95: push rbp
1405a7a96: push r15
1405a7a98: push r14
1405a7a9a: push r13
1405a7a9c: push r12
1405a7a9e: push rsi
1405a7a9f: push rdi
1405a7aa0: push rbx
1405a7aa1: sub rsp, 0x118
1405a7aa8: lea rbp, [rdx + 0x80]
1405a7aaf: movaps xmmword ptr [rsp + 0x70], xmm15
1405a7ab5: movaps xmmword ptr [rsp + 0x80], xmm14
1405a7abe: movaps xmmword ptr [rsp + 0x90], xmm13
1405a7ac7: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a7ad0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a7ad9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a7ae2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a7aeb: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a7af4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a7afc: movaps xmmword ptr [rsp + 0x100], xmm6
1405a7b04: and rdx, 0xffffffffffffff80
1405a7b08: mov rbx, rdx
1405a7b0b: mov rax, qword ptr [rbp + 0x37b0]
1405a7b12: mov rdx, qword ptr [rax + 0xc8]
1405a7b19: lea rax, [rdx*2]
1405a7b21: test rax, rax
1405a7b24: je 0x1405a7b3f
1405a7b26: mov rax, qword ptr [rbp + 0x37b0]
1405a7b2d: mov rcx, qword ptr [rax + 0xd0]
1405a7b34: mov r8d, 1
1405a7b3a: call 0x140613c20 ; 
1405a7b3f: mov eax, dword ptr [rbx + 0xe8]
1405a7b45: mov byte ptr [rbx + 0x11a], al
1405a7b4b: mov eax, dword ptr [rbx + 0x128]
1405a7b51: mov byte ptr [rbx + 0x11b], al
1405a7b57: mov byte ptr [rbx + 0x11c], 0
1405a7b5e: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a7b66: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a7b6e: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a7b77: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a7b80: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a7b89: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a7b92: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a7b9b: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a7ba4: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a7bad: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a7bb3: add rsp, 0x118
1405a7bba: pop rbx
1405a7bbb: pop rdi
1405a7bbc: pop rsi
1405a7bbd: pop r12
1405a7bbf: pop r13
1405a7bc1: pop r14
1405a7bc3: pop r15
1405a7bc5: pop rbp
1405a7bc6: ret 
1405a7bc7: nop word ptr [rax + rax]
1405a7bd0: mov qword ptr [rsp + 0x10], rdx
1405a7bd5: push rbp
1405a7bd6: push r15
1405a7bd8: push r14
1405a7bda: push r13
1405a7bdc: push r12
1405a7bde: push rsi
1405a7bdf: push rdi
1405a7be0: push rbx
1405a7be1: sub rsp, 0x118
1405a7be8: lea rbp, [rdx + 0x80]
1405a7bef: movaps xmmword ptr [rsp + 0x70], xmm15
1405a7bf5: movaps xmmword ptr [rsp + 0x80], xmm14
1405a7bfe: movaps xmmword ptr [rsp + 0x90], xmm13
1405a7c07: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a7c10: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a7c19: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a7c22: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a7c2b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a7c34: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a7c3c: movaps xmmword ptr [rsp + 0x100], xmm6
1405a7c44: and rdx, 0xffffffffffffff80
1405a7c48: mov rbx, rdx
1405a7c4b: mov rax, qword ptr [rbx + 0x298]
1405a7c52: lock dec qword ptr [rax]
1405a7c56: jne 0x1405a7c91
1405a7c58: lea rcx, [rbx + 0x298]
1405a7c5f: mov byte ptr [rbx + 0xd4], 0
1405a7c66: mov eax, dword ptr [rbx + 0x128]
1405a7c6c: mov byte ptr [rbx + 0xd5], al
1405a7c72: mov eax, dword ptr [rbx + 0xe8]
1405a7c78: mov byte ptr [rbx + 0xd6], al
1405a7c7e: mov byte ptr [rbx + 0xd7], 0
1405a7c85: mov byte ptr [rbx + 0xd8], 0
1405a7c8c: call 0x140d53db0 ; alloc::sync::Arc::drop_slow<std::thread::Packet<tuple$<> >,alloc::alloc::Global>
1405a7c91: mov byte ptr [rbx + 0xd4], 0
1405a7c98: mov eax, dword ptr [rbx + 0x128]
1405a7c9e: mov byte ptr [rbx + 0xd5], al
1405a7ca4: mov eax, dword ptr [rbx + 0xe8]
1405a7caa: mov byte ptr [rbx + 0xd6], al
1405a7cb0: mov byte ptr [rbx + 0xd7], 0
1405a7cb7: mov byte ptr [rbx + 0xd8], 0
1405a7cbe: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a7cc6: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a7cce: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a7cd7: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a7ce0: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a7ce9: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a7cf2: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a7cfb: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a7d04: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a7d0d: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a7d13: add rsp, 0x118
1405a7d1a: pop rbx
1405a7d1b: pop rdi
1405a7d1c: pop rsi
1405a7d1d: pop r12
1405a7d1f: pop r13
1405a7d21: pop r14
1405a7d23: pop r15
1405a7d25: pop rbp
1405a7d26: ret 
1405a7d27: nop word ptr [rax + rax]
1405a7d30: mov qword ptr [rsp + 0x10], rdx
1405a7d35: push rbp
1405a7d36: push r15
1405a7d38: push r14
1405a7d3a: push r13
1405a7d3c: push r12
1405a7d3e: push rsi
1405a7d3f: push rdi
1405a7d40: push rbx
1405a7d41: sub rsp, 0x118
1405a7d48: lea rbp, [rdx + 0x80]
1405a7d4f: movaps xmmword ptr [rsp + 0x70], xmm15
1405a7d55: movaps xmmword ptr [rsp + 0x80], xmm14
1405a7d5e: movaps xmmword ptr [rsp + 0x90], xmm13
1405a7d67: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a7d70: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a7d79: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a7d82: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a7d8b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a7d94: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a7d9c: movaps xmmword ptr [rsp + 0x100], xmm6
1405a7da4: and rdx, 0xffffffffffffff80
1405a7da8: mov rbx, rdx
1405a7dab: lea rcx, [rbx + 0x2bf0]
1405a7db2: call 0x1405f3270 ; core::ptr::drop_in_place<rhapsody::rg::state::RenderGraphState>
1405a7db7: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a7dbf: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a7dc7: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a7dd0: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a7dd9: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a7de2: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a7deb: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a7df4: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a7dfd: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a7e06: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a7e0c: add rsp, 0x118
1405a7e13: pop rbx
1405a7e14: pop rdi
1405a7e15: pop rsi
1405a7e16: pop r12
1405a7e18: pop r13
1405a7e1a: pop r14
1405a7e1c: pop r15
1405a7e1e: pop rbp
1405a7e1f: ret 
1405a7e20: mov qword ptr [rsp + 0x10], rdx
1405a7e25: push rbp
1405a7e26: push r15
1405a7e28: push r14
1405a7e2a: push r13
1405a7e2c: push r12
1405a7e2e: push rsi
1405a7e2f: push rdi
1405a7e30: push rbx
1405a7e31: sub rsp, 0x118
1405a7e38: lea rbp, [rdx + 0x80]
1405a7e3f: movaps xmmword ptr [rsp + 0x70], xmm15
1405a7e45: movaps xmmword ptr [rsp + 0x80], xmm14
1405a7e4e: movaps xmmword ptr [rsp + 0x90], xmm13
1405a7e57: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a7e60: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a7e69: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a7e72: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a7e7b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a7e84: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a7e8c: movaps xmmword ptr [rsp + 0x100], xmm6
1405a7e94: and rdx, 0xffffffffffffff80
1405a7e98: mov rbx, rdx
1405a7e9b: lea rcx, [rbx + 0x440]
1405a7ea2: call 0x1405e7a00 ; core::ptr::drop_in_place<std::io::error::Error>
1405a7ea7: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a7eaf: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a7eb7: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a7ec0: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a7ec9: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a7ed2: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a7edb: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a7ee4: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a7eed: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a7ef6: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a7efc: add rsp, 0x118
1405a7f03: pop rbx
1405a7f04: pop rdi
1405a7f05: pop rsi
1405a7f06: pop r12
1405a7f08: pop r13
1405a7f0a: pop r14
1405a7f0c: pop r15
1405a7f0e: pop rbp
1405a7f0f: ret 
1405a7f10: mov qword ptr [rsp + 0x10], rdx
1405a7f15: push rbp
1405a7f16: push r15
1405a7f18: push r14
1405a7f1a: push r13
1405a7f1c: push r12
1405a7f1e: push rsi
1405a7f1f: push rdi
1405a7f20: push rbx
1405a7f21: sub rsp, 0x118
1405a7f28: lea rbp, [rdx + 0x80]
1405a7f2f: movaps xmmword ptr [rsp + 0x70], xmm15
1405a7f35: movaps xmmword ptr [rsp + 0x80], xmm14
1405a7f3e: movaps xmmword ptr [rsp + 0x90], xmm13
1405a7f47: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a7f50: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a7f59: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a7f62: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a7f6b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a7f74: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a7f7c: movaps xmmword ptr [rsp + 0x100], xmm6
1405a7f84: and rdx, 0xffffffffffffff80
1405a7f88: mov rbx, rdx
1405a7f8b: lea rcx, [rbx + 0xb00]
1405a7f92: call 0x1405f6770 ; core::ptr::drop_in_place<rhapsody::rg::resource_pool::ResourcePool>
1405a7f97: mov byte ptr [rbx + 0xb4], 1
1405a7f9e: mov eax, dword ptr [rbx + 0x130]
1405a7fa4: mov byte ptr [rbx + 0xb5], al
1405a7faa: mov byte ptr [rbx + 0xb6], 1
1405a7fb1: mov byte ptr [rbx + 0xb7], 0
1405a7fb8: mov byte ptr [rbx + 0xb8], 1
1405a7fbf: mov byte ptr [rbx + 0xb9], 0
1405a7fc6: mov byte ptr [rbx + 0xba], 1
1405a7fcd: mov byte ptr [rbx + 0xbb], 1
1405a7fd4: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a7fdc: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a7fe4: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a7fed: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a7ff6: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a7fff: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a8008: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a8011: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a801a: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a8023: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a8029: add rsp, 0x118
1405a8030: pop rbx
1405a8031: pop rdi
1405a8032: pop rsi
1405a8033: pop r12
1405a8035: pop r13
1405a8037: pop r14
1405a8039: pop r15
1405a803b: pop rbp
1405a803c: ret 
1405a803d: nop dword ptr [rax]
1405a8040: mov qword ptr [rsp + 0x10], rdx
1405a8045: push rbp
1405a8046: push r15
1405a8048: push r14
1405a804a: push r13
1405a804c: push r12
1405a804e: push rsi
1405a804f: push rdi
1405a8050: push rbx
1405a8051: sub rsp, 0x118
1405a8058: lea rbp, [rdx + 0x80]
1405a805f: movaps xmmword ptr [rsp + 0x70], xmm15
1405a8065: movaps xmmword ptr [rsp + 0x80], xmm14
1405a806e: movaps xmmword ptr [rsp + 0x90], xmm13
1405a8077: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a8080: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a8089: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a8092: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a809b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a80a4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a80ac: movaps xmmword ptr [rsp + 0x100], xmm6
1405a80b4: and rdx, 0xffffffffffffff80
1405a80b8: mov rbx, rdx
1405a80bb: lea rcx, [rbx + 0x440]
1405a80c2: call 0x1405e7a00 ; core::ptr::drop_in_place<std::io::error::Error>
1405a80c7: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a80cf: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a80d7: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a80e0: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a80e9: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a80f2: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a80fb: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a8104: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a810d: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a8116: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a811c: add rsp, 0x118
1405a8123: pop rbx
1405a8124: pop rdi
1405a8125: pop rsi
1405a8126: pop r12
1405a8128: pop r13
1405a812a: pop r14
1405a812c: pop r15
1405a812e: pop rbp
1405a812f: ret 
1405a8130: mov qword ptr [rsp + 0x10], rdx
1405a8135: push rbp
1405a8136: push r15
1405a8138: push r14
1405a813a: push r13
1405a813c: push r12
1405a813e: push rsi
1405a813f: push rdi
1405a8140: push rbx
1405a8141: sub rsp, 0x118
1405a8148: lea rbp, [rdx + 0x80]
1405a814f: movaps xmmword ptr [rsp + 0x70], xmm15
1405a8155: movaps xmmword ptr [rsp + 0x80], xmm14
1405a815e: movaps xmmword ptr [rsp + 0x90], xmm13
1405a8167: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a8170: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a8179: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a8182: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a818b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a8194: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a819c: movaps xmmword ptr [rsp + 0x100], xmm6
1405a81a4: and rdx, 0xffffffffffffff80
1405a81a8: mov rbx, rdx
1405a81ab: mov rdx, qword ptr [rbx + 0x2590]
1405a81b2: test rdx, rdx
1405a81b5: je 0x1405a81c9
1405a81b7: mov rcx, qword ptr [rbx + 0x2598]
1405a81be: mov r8d, 1
1405a81c4: call 0x140613c20 ; 
1405a81c9: mov byte ptr [rbx + 0xb4], 1
1405a81d0: mov byte ptr [rbx + 0xb5], 1
1405a81d7: mov byte ptr [rbx + 0xb6], 1
1405a81de: mov byte ptr [rbx + 0xb7], 0
1405a81e5: mov byte ptr [rbx + 0xb8], 1
1405a81ec: mov byte ptr [rbx + 0xb9], 1
1405a81f3: mov byte ptr [rbx + 0xba], 1
1405a81fa: mov byte ptr [rbx + 0xbb], 1
1405a8201: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a8209: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a8211: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a821a: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a8223: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a822c: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a8235: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a823e: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a8247: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a8250: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a8256: add rsp, 0x118
1405a825d: pop rbx
1405a825e: pop rdi
1405a825f: pop rsi
1405a8260: pop r12
1405a8262: pop r13
1405a8264: pop r14
1405a8266: pop r15
1405a8268: pop rbp
1405a8269: ret 
1405a826a: nop word ptr [rax + rax]
1405a8270: mov qword ptr [rsp + 0x10], rdx
1405a8275: push rbp
1405a8276: push r15
1405a8278: push r14
1405a827a: push r13
1405a827c: push r12
1405a827e: push rsi
1405a827f: push rdi
1405a8280: push rbx
1405a8281: sub rsp, 0x118
1405a8288: lea rbp, [rdx + 0x80]
1405a828f: movaps xmmword ptr [rsp + 0x70], xmm15
1405a8295: movaps xmmword ptr [rsp + 0x80], xmm14
1405a829e: movaps xmmword ptr [rsp + 0x90], xmm13
1405a82a7: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a82b0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a82b9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a82c2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a82cb: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a82d4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a82dc: movaps xmmword ptr [rsp + 0x100], xmm6
1405a82e4: and rdx, 0xffffffffffffff80
1405a82e8: mov rbx, rdx
1405a82eb: mov byte ptr [rbx + 0xb4], 1
1405a82f2: mov byte ptr [rbx + 0xb5], 1
1405a82f9: mov byte ptr [rbx + 0xb6], 1
1405a8300: mov byte ptr [rbx + 0xb7], 0
1405a8307: mov byte ptr [rbx + 0xb8], 1
1405a830e: mov byte ptr [rbx + 0xb9], 1
1405a8315: mov byte ptr [rbx + 0xba], 1
1405a831c: mov byte ptr [rbx + 0xbb], 1
1405a8323: lea rcx, [rbx + 0xb00]
1405a832a: call 0x1405e7a00 ; core::ptr::drop_in_place<std::io::error::Error>
1405a832f: mov byte ptr [rbx + 0xb4], 1
1405a8336: mov byte ptr [rbx + 0xb5], 1
1405a833d: mov byte ptr [rbx + 0xb6], 1
1405a8344: mov byte ptr [rbx + 0xb7], 0
1405a834b: mov byte ptr [rbx + 0xb8], 1
1405a8352: mov byte ptr [rbx + 0xb9], 1
1405a8359: mov byte ptr [rbx + 0xba], 1
1405a8360: mov byte ptr [rbx + 0xbb], 1
1405a8367: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a836f: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a8377: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a8380: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a8389: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a8392: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a839b: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a83a4: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a83ad: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a83b6: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a83bc: add rsp, 0x118
1405a83c3: pop rbx
1405a83c4: pop rdi
1405a83c5: pop rsi
1405a83c6: pop r12
1405a83c8: pop r13
1405a83ca: pop r14
1405a83cc: pop r15
1405a83ce: pop rbp
1405a83cf: ret 
1405a83d0: mov qword ptr [rsp + 0x10], rdx
1405a83d5: push rbp
1405a83d6: push r15
1405a83d8: push r14
1405a83da: push r13
1405a83dc: push r12
1405a83de: push rsi
1405a83df: push rdi
1405a83e0: push rbx
1405a83e1: sub rsp, 0x118
1405a83e8: lea rbp, [rdx + 0x80]
1405a83ef: movaps xmmword ptr [rsp + 0x70], xmm15
1405a83f5: movaps xmmword ptr [rsp + 0x80], xmm14
1405a83fe: movaps xmmword ptr [rsp + 0x90], xmm13
1405a8407: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a8410: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a8419: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a8422: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a842b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a8434: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a843c: movaps xmmword ptr [rsp + 0x100], xmm6
1405a8444: and rdx, 0xffffffffffffff80
1405a8448: mov rbx, rdx
1405a844b: lea rcx, [rbx + 0x440]
1405a8452: mov r8d, 0x20
1405a8458: mov r9d, 0x10
1405a845e: lea rdx, [rbx + 0x460]
1405a8465: call 0x140051fa0 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::walls::WallId,alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global> >,alloc::alloc::Global>
1405a846a: mov byte ptr [rbx + 0xaa], 1
1405a8471: mov byte ptr [rbx + 0xab], 1
1405a8478: mov byte ptr [rbx + 0xac], 1
1405a847f: mov byte ptr [rbx + 0xad], 1
1405a8486: mov byte ptr [rbx + 0xae], 0
1405a848d: mov byte ptr [rbx + 0xaf], 1
1405a8494: mov byte ptr [rbx + 0xb0], 0
1405a849b: mov byte ptr [rbx + 0xb1], 1
1405a84a2: mov byte ptr [rbx + 0xb2], 1
1405a84a9: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a84b1: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a84b9: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a84c2: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a84cb: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a84d4: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a84dd: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a84e6: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a84ef: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a84f8: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a84fe: add rsp, 0x118
1405a8505: pop rbx
1405a8506: pop rdi
1405a8507: pop rsi
1405a8508: pop r12
1405a850a: pop r13
1405a850c: pop r14
1405a850e: pop r15
1405a8510: pop rbp
1405a8511: ret 
1405a8512: nop word ptr cs:[rax + rax]
1405a8520: mov qword ptr [rsp + 0x10], rdx
1405a8525: push rbp
1405a8526: push r15
1405a8528: push r14
1405a852a: push r13
1405a852c: push r12
1405a852e: push rsi
1405a852f: push rdi
1405a8530: push rbx
1405a8531: sub rsp, 0x118
1405a8538: lea rbp, [rdx + 0x80]
1405a853f: movaps xmmword ptr [rsp + 0x70], xmm15
1405a8545: movaps xmmword ptr [rsp + 0x80], xmm14
1405a854e: movaps xmmword ptr [rsp + 0x90], xmm13
1405a8557: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a8560: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a8569: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a8572: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a857b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a8584: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a858c: movaps xmmword ptr [rsp + 0x100], xmm6
1405a8594: and rdx, 0xffffffffffffff80
1405a8598: mov rbx, rdx
1405a859b: mov byte ptr [rbx + 0xaa], 1
1405a85a2: mov byte ptr [rbx + 0xab], 1
1405a85a9: mov byte ptr [rbx + 0xac], 1
1405a85b0: mov byte ptr [rbx + 0xad], 1
1405a85b7: mov byte ptr [rbx + 0xae], 0
1405a85be: mov byte ptr [rbx + 0xaf], 1
1405a85c5: mov byte ptr [rbx + 0xb0], 0
1405a85cc: mov byte ptr [rbx + 0xb1], 1
1405a85d3: mov byte ptr [rbx + 0xb2], 1
1405a85da: lea rcx, [rbx + 0x440]
1405a85e1: call 0x1405d2cf0 ; core::ptr::drop_in_place<alloc::collections::vec_deque::VecDeque<enum2$<country_core::meta_commander::MetaCommand>,alloc::alloc::Global> >
1405a85e6: mov byte ptr [rbx + 0xaa], 1
1405a85ed: mov byte ptr [rbx + 0xab], 1
1405a85f4: mov byte ptr [rbx + 0xac], 1
1405a85fb: mov byte ptr [rbx + 0xad], 1
1405a8602: mov byte ptr [rbx + 0xae], 0
1405a8609: mov byte ptr [rbx + 0xaf], 1
1405a8610: mov byte ptr [rbx + 0xb0], 0
1405a8617: mov byte ptr [rbx + 0xb1], 1
1405a861e: mov byte ptr [rbx + 0xb2], 1
1405a8625: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a862d: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a8635: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a863e: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a8647: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a8650: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a8659: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a8662: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a866b: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a8674: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a867a: add rsp, 0x118
1405a8681: pop rbx
1405a8682: pop rdi
1405a8683: pop rsi
1405a8684: pop r12
1405a8686: pop r13
1405a8688: pop r14
1405a868a: pop r15
1405a868c: pop rbp
1405a868d: ret 
1405a868e: nop 
1405a8690: mov qword ptr [rsp + 0x10], rdx
1405a8695: push rbp
1405a8696: push r15
1405a8698: push r14
1405a869a: push r13
1405a869c: push r12
1405a869e: push rsi
1405a869f: push rdi
1405a86a0: push rbx
1405a86a1: sub rsp, 0x118
1405a86a8: lea rbp, [rdx + 0x80]
1405a86af: movaps xmmword ptr [rsp + 0x70], xmm15
1405a86b5: movaps xmmword ptr [rsp + 0x80], xmm14
1405a86be: movaps xmmword ptr [rsp + 0x90], xmm13
1405a86c7: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a86d0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a86d9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a86e2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a86eb: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a86f4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a86fc: movaps xmmword ptr [rsp + 0x100], xmm6
1405a8704: and rdx, 0xffffffffffffff80
1405a8708: mov rbx, rdx
1405a870b: mov byte ptr [rbx + 0xbc], 1
1405a8712: mov byte ptr [rbx + 0xbd], 1
1405a8719: mov byte ptr [rbx + 0xbe], 1
1405a8720: mov byte ptr [rbx + 0xbf], 1
1405a8727: mov byte ptr [rbx + 0xc0], 1
1405a872e: mov byte ptr [rbx + 0xc1], 1
1405a8735: lea rcx, [rbx + 0xb00]
1405a873c: mov r8d, 0x38
1405a8742: mov r9d, 0x10
1405a8748: lea rdx, [rbx + 0xb20]
1405a874f: call 0x140053060 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<rhapsody::raw::texture::TextureDesc,alloc::vec::Vec<rhapsody::raw::texture::Texture,alloc::alloc::Global> >,alloc::alloc::Global>
1405a8754: mov byte ptr [rbx + 0xbc], 1
1405a875b: mov byte ptr [rbx + 0xbd], 1
1405a8762: mov byte ptr [rbx + 0xbe], 1
1405a8769: mov byte ptr [rbx + 0xbf], 1
1405a8770: mov byte ptr [rbx + 0xc0], 1
1405a8777: mov byte ptr [rbx + 0xc1], 1
1405a877e: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a8786: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a878e: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a8797: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a87a0: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a87a9: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a87b2: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a87bb: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a87c4: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a87cd: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a87d3: add rsp, 0x118
1405a87da: pop rbx
1405a87db: pop rdi
1405a87dc: pop rsi
1405a87dd: pop r12
1405a87df: pop r13
1405a87e1: pop r14
1405a87e3: pop r15
1405a87e5: pop rbp
1405a87e6: ret 
1405a87e7: nop word ptr [rax + rax]
1405a87f0: mov qword ptr [rsp + 0x10], rdx
1405a87f5: push rbp
1405a87f6: push r15
1405a87f8: push r14
1405a87fa: push r13
1405a87fc: push r12
1405a87fe: push rsi
1405a87ff: push rdi
1405a8800: push rbx
1405a8801: sub rsp, 0x118
1405a8808: lea rbp, [rdx + 0x80]
1405a880f: movaps xmmword ptr [rsp + 0x70], xmm15
1405a8815: movaps xmmword ptr [rsp + 0x80], xmm14
1405a881e: movaps xmmword ptr [rsp + 0x90], xmm13
1405a8827: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a8830: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a8839: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a8842: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a884b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a8854: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a885c: movaps xmmword ptr [rsp + 0x100], xmm6
1405a8864: and rdx, 0xffffffffffffff80
1405a8868: mov rbx, rdx
1405a886b: cmp qword ptr [rbx + 0x130], 0
1405a8873: je 0x1405a888e
1405a8875: mov r8d, 1
1405a887b: mov rcx, qword ptr [rbx + 0x128]
1405a8882: mov rdx, qword ptr [rbx + 0x130]
1405a8889: call 0x140613c20 ; 
1405a888e: mov byte ptr [rbx + 0xb4], 1
1405a8895: mov byte ptr [rbx + 0xb5], 1
1405a889c: mov byte ptr [rbx + 0xb6], 1
1405a88a3: mov byte ptr [rbx + 0xb7], 0
1405a88aa: mov byte ptr [rbx + 0xb8], 1
1405a88b1: mov byte ptr [rbx + 0xb9], 1
1405a88b8: mov byte ptr [rbx + 0xba], 1
1405a88bf: mov byte ptr [rbx + 0xbb], 1
1405a88c6: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a88ce: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a88d6: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a88df: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a88e8: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a88f1: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a88fa: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a8903: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a890c: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a8915: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a891b: add rsp, 0x118
1405a8922: pop rbx
1405a8923: pop rdi
1405a8924: pop rsi
1405a8925: pop r12
1405a8927: pop r13
1405a8929: pop r14
1405a892b: pop r15
1405a892d: pop rbp
1405a892e: ret 
1405a892f: nop 
1405a8930: mov qword ptr [rsp + 0x10], rdx
1405a8935: push rbp
1405a8936: push r15
1405a8938: push r14
1405a893a: push r13
1405a893c: push r12
1405a893e: push rsi
1405a893f: push rdi
1405a8940: push rbx
1405a8941: sub rsp, 0x118
1405a8948: lea rbp, [rdx + 0x80]
1405a894f: movaps xmmword ptr [rsp + 0x70], xmm15
1405a8955: movaps xmmword ptr [rsp + 0x80], xmm14
1405a895e: movaps xmmword ptr [rsp + 0x90], xmm13
1405a8967: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a8970: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a8979: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a8982: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a898b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a8994: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a899c: movaps xmmword ptr [rsp + 0x100], xmm6
1405a89a4: and rdx, 0xffffffffffffff80
1405a89a8: mov rbx, rdx
1405a89ab: mov byte ptr [rbx + 0x105], 1
1405a89b2: lea rcx, [rbx + 0x370]
1405a89b9: call 0x1405f2ea0 ; core::ptr::drop_in_place<rhapsody::rg::executor::GraphExecutor>
1405a89be: mov byte ptr [rbx + 0x105], 1
1405a89c5: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a89cd: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a89d5: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a89de: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a89e7: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a89f0: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a89f9: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a8a02: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a8a0b: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a8a14: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a8a1a: add rsp, 0x118
1405a8a21: pop rbx
1405a8a22: pop rdi
1405a8a23: pop rsi
1405a8a24: pop r12
1405a8a26: pop r13
1405a8a28: pop r14
1405a8a2a: pop r15
1405a8a2c: pop rbp
1405a8a2d: ret 
1405a8a2e: nop 
1405a8a30: mov qword ptr [rsp + 0x10], rdx
1405a8a35: push rbp
1405a8a36: push r15
1405a8a38: push r14
1405a8a3a: push r13
1405a8a3c: push r12
1405a8a3e: push rsi
1405a8a3f: push rdi
1405a8a40: push rbx
1405a8a41: sub rsp, 0x118
1405a8a48: lea rbp, [rdx + 0x80]
1405a8a4f: movaps xmmword ptr [rsp + 0x70], xmm15
1405a8a55: movaps xmmword ptr [rsp + 0x80], xmm14
1405a8a5e: movaps xmmword ptr [rsp + 0x90], xmm13
1405a8a67: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a8a70: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a8a79: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a8a82: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a8a8b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a8a94: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a8a9c: movaps xmmword ptr [rsp + 0x100], xmm6
1405a8aa4: and rdx, 0xffffffffffffff80
1405a8aa8: mov rbx, rdx
1405a8aab: cmp qword ptr [rbx + 0xe8], 0
1405a8ab3: jne 0x1405a8b4f
1405a8ab9: mov rdx, qword ptr [rbx + 0xb10]
1405a8ac0: lea rax, [rdx*2]
1405a8ac8: test rax, rax
1405a8acb: je 0x1405a8adf
1405a8acd: mov rcx, qword ptr [rbx + 0xb18]
1405a8ad4: mov r8d, 1
1405a8ada: call 0x140613c20 ; 
1405a8adf: mov byte ptr [rbx + 0xdb], 1
1405a8ae6: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a8aee: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a8af6: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a8aff: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a8b08: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a8b11: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a8b1a: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a8b23: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a8b2c: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a8b35: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a8b3b: add rsp, 0x118
1405a8b42: pop rbx
1405a8b43: pop rdi
1405a8b44: pop rsi
1405a8b45: pop r12
1405a8b47: pop r13
1405a8b49: pop r14
1405a8b4b: pop r15
1405a8b4d: pop rbp
1405a8b4e: ret 
1405a8b4f: mov byte ptr [rbx + 0xdb], 1
1405a8b56: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a8b5e: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a8b66: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a8b6f: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a8b78: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a8b81: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a8b8a: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a8b93: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a8b9c: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a8ba5: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a8bab: add rsp, 0x118
1405a8bb2: pop rbx
1405a8bb3: pop rdi
1405a8bb4: pop rsi
1405a8bb5: pop r12
1405a8bb7: pop r13
1405a8bb9: pop r14
1405a8bbb: pop r15
1405a8bbd: pop rbp
1405a8bbe: ret 
1405a8bbf: nop 
1405a8bc0: mov qword ptr [rsp + 0x10], rdx
1405a8bc5: push rbp
1405a8bc6: push r15
1405a8bc8: push r14
1405a8bca: push r13
1405a8bcc: push r12
1405a8bce: push rsi
1405a8bcf: push rdi
1405a8bd0: push rbx
1405a8bd1: sub rsp, 0x118
1405a8bd8: lea rbp, [rdx + 0x80]
1405a8bdf: movaps xmmword ptr [rsp + 0x70], xmm15
1405a8be5: movaps xmmword ptr [rsp + 0x80], xmm14
1405a8bee: movaps xmmword ptr [rsp + 0x90], xmm13
1405a8bf7: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a8c00: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a8c09: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a8c12: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a8c1b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a8c24: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a8c2c: movaps xmmword ptr [rsp + 0x100], xmm6
1405a8c34: and rdx, 0xffffffffffffff80
1405a8c38: mov rbx, rdx
1405a8c3b: mov byte ptr [rbx + 0x106], 1
1405a8c42: lea rcx, [rbx + 0x2230]
1405a8c49: call 0x1405f2ea0 ; core::ptr::drop_in_place<rhapsody::rg::executor::GraphExecutor>
1405a8c4e: mov byte ptr [rbx + 0x106], 1
1405a8c55: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a8c5d: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a8c65: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a8c6e: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a8c77: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a8c80: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a8c89: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a8c92: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a8c9b: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a8ca4: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a8caa: add rsp, 0x118
1405a8cb1: pop rbx
1405a8cb2: pop rdi
1405a8cb3: pop rsi
1405a8cb4: pop r12
1405a8cb6: pop r13
1405a8cb8: pop r14
1405a8cba: pop r15
1405a8cbc: pop rbp
1405a8cbd: ret 
1405a8cbe: nop 
1405a8cc0: mov qword ptr [rsp + 0x10], rdx
1405a8cc5: push rbp
1405a8cc6: push r15
1405a8cc8: push r14
1405a8cca: push r13
1405a8ccc: push r12
1405a8cce: push rsi
1405a8ccf: push rdi
1405a8cd0: push rbx
1405a8cd1: sub rsp, 0x118
1405a8cd8: lea rbp, [rdx + 0x80]
1405a8cdf: movaps xmmword ptr [rsp + 0x70], xmm15
1405a8ce5: movaps xmmword ptr [rsp + 0x80], xmm14
1405a8cee: movaps xmmword ptr [rsp + 0x90], xmm13
1405a8cf7: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a8d00: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a8d09: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a8d12: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a8d1b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a8d24: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a8d2c: movaps xmmword ptr [rsp + 0x100], xmm6
1405a8d34: and rdx, 0xffffffffffffff80
1405a8d38: mov rbx, rdx
1405a8d3b: mov esi, dword ptr [rbx + 0x130]
1405a8d41: mov byte ptr [rbx + 0xc7], sil
1405a8d48: lea rcx, [rbx + 0xb00]
1405a8d4f: call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > > >
1405a8d54: mov byte ptr [rbx + 0xc7], sil
1405a8d5b: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a8d63: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a8d6b: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a8d74: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a8d7d: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a8d86: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a8d8f: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a8d98: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a8da1: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a8daa: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a8db0: add rsp, 0x118
1405a8db7: pop rbx
1405a8db8: pop rdi
1405a8db9: pop rsi
1405a8dba: pop r12
1405a8dbc: pop r13
1405a8dbe: pop r14
1405a8dc0: pop r15
1405a8dc2: pop rbp
1405a8dc3: ret 
1405a8dc4: nop word ptr cs:[rax + rax]
1405a8dd0: mov qword ptr [rsp + 0x10], rdx
1405a8dd5: push rbp
1405a8dd6: push r15
1405a8dd8: push r14
1405a8dda: push r13
1405a8ddc: push r12
1405a8dde: push rsi
1405a8ddf: push rdi
1405a8de0: push rbx
1405a8de1: sub rsp, 0x118
1405a8de8: lea rbp, [rdx + 0x80]
1405a8def: movaps xmmword ptr [rsp + 0x70], xmm15
1405a8df5: movaps xmmword ptr [rsp + 0x80], xmm14
1405a8dfe: movaps xmmword ptr [rsp + 0x90], xmm13
1405a8e07: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a8e10: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a8e19: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a8e22: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a8e2b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a8e34: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a8e3c: movaps xmmword ptr [rsp + 0x100], xmm6
1405a8e44: and rdx, 0xffffffffffffff80
1405a8e48: mov rbx, rdx
1405a8e4b: mov rdx, qword ptr [rbx + 0x440]
1405a8e52: test rdx, rdx
1405a8e55: je 0x1405a8e69
1405a8e57: mov rcx, qword ptr [rbx + 0x448]
1405a8e5e: mov r8d, 1
1405a8e64: call 0x140613c20 ; 
1405a8e69: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a8e71: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a8e79: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a8e82: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a8e8b: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a8e94: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a8e9d: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a8ea6: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a8eaf: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a8eb8: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a8ebe: add rsp, 0x118
1405a8ec5: pop rbx
1405a8ec6: pop rdi
1405a8ec7: pop rsi
1405a8ec8: pop r12
1405a8eca: pop r13
1405a8ecc: pop r14
1405a8ece: pop r15
1405a8ed0: pop rbp
1405a8ed1: ret 
1405a8ed2: nop word ptr cs:[rax + rax]
1405a8ee0: mov qword ptr [rsp + 0x10], rdx
1405a8ee5: push rbp
1405a8ee6: push r15
1405a8ee8: push r14
1405a8eea: push r13
1405a8eec: push r12
1405a8eee: push rsi
1405a8eef: push rdi
1405a8ef0: push rbx
1405a8ef1: sub rsp, 0x118
1405a8ef8: lea rbp, [rdx + 0x80]
1405a8eff: movaps xmmword ptr [rsp + 0x70], xmm15
1405a8f05: movaps xmmword ptr [rsp + 0x80], xmm14
1405a8f0e: movaps xmmword ptr [rsp + 0x90], xmm13
1405a8f17: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a8f20: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a8f29: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a8f32: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a8f3b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a8f44: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a8f4c: movaps xmmword ptr [rsp + 0x100], xmm6
1405a8f54: and rdx, 0xffffffffffffff80
1405a8f58: mov rbx, rdx
1405a8f5b: mov byte ptr [rbx + 0xe4], 1
1405a8f62: mov byte ptr [rbx + 0xe5], 1
1405a8f69: mov byte ptr [rbx + 0xe6], 1
1405a8f70: mov byte ptr [rbx + 0xe7], 1
1405a8f77: lea rcx, [rbx + 0xb00]
1405a8f7e: call 0x1405d8140 ; core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<bevy_app::sub_app::SubApp> > >
1405a8f83: mov byte ptr [rbx + 0xe4], 1
1405a8f8a: mov byte ptr [rbx + 0xe5], 1
1405a8f91: mov byte ptr [rbx + 0xe6], 1
1405a8f98: mov byte ptr [rbx + 0xe7], 1
1405a8f9f: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a8fa7: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a8faf: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a8fb8: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a8fc1: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a8fca: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a8fd3: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a8fdc: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a8fe5: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a8fee: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a8ff4: add rsp, 0x118
1405a8ffb: pop rbx
1405a8ffc: pop rdi
1405a8ffd: pop rsi
1405a8ffe: pop r12
1405a9000: pop r13
1405a9002: pop r14
1405a9004: pop r15
1405a9006: pop rbp
1405a9007: ret 
1405a9008: nop dword ptr [rax + rax]
1405a9010: mov qword ptr [rsp + 0x10], rdx
1405a9015: push rbp
1405a9016: push r15
1405a9018: push r14
1405a901a: push r13
1405a901c: push r12
1405a901e: push rsi
1405a901f: push rdi
1405a9020: push rbx
1405a9021: sub rsp, 0x118
1405a9028: lea rbp, [rdx + 0x80]
1405a902f: movaps xmmword ptr [rsp + 0x70], xmm15
1405a9035: movaps xmmword ptr [rsp + 0x80], xmm14
1405a903e: movaps xmmword ptr [rsp + 0x90], xmm13
1405a9047: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a9050: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a9059: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a9062: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a906b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a9074: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a907c: movaps xmmword ptr [rsp + 0x100], xmm6
1405a9084: and rdx, 0xffffffffffffff80
1405a9088: mov rbx, rdx
1405a908b: mov byte ptr [rbx + 0xd4], 1
1405a9092: mov byte ptr [rbx + 0xd5], 1
1405a9099: mov byte ptr [rbx + 0xd6], 1
1405a90a0: mov byte ptr [rbx + 0xd7], 0
1405a90a7: mov byte ptr [rbx + 0xd8], 1
1405a90ae: lea rcx, [rbx + 0x440]
1405a90b5: call 0x1405e7a00 ; core::ptr::drop_in_place<std::io::error::Error>
1405a90ba: mov byte ptr [rbx + 0xd4], 1
1405a90c1: mov byte ptr [rbx + 0xd5], 1
1405a90c8: mov byte ptr [rbx + 0xd6], 1
1405a90cf: mov byte ptr [rbx + 0xd7], 0
1405a90d6: mov byte ptr [rbx + 0xd8], 1
1405a90dd: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a90e5: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a90ed: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a90f6: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a90ff: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a9108: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a9111: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a911a: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a9123: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a912c: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a9132: add rsp, 0x118
1405a9139: pop rbx
1405a913a: pop rdi
1405a913b: pop rsi
1405a913c: pop r12
1405a913e: pop r13
1405a9140: pop r14
1405a9142: pop r15
1405a9144: pop rbp
1405a9145: ret 
1405a9146: nop word ptr cs:[rax + rax]
1405a9150: mov qword ptr [rsp + 0x10], rdx
1405a9155: push rbp
1405a9156: push r15
1405a9158: push r14
1405a915a: push r13
1405a915c: push r12
1405a915e: push rsi
1405a915f: push rdi
1405a9160: push rbx
1405a9161: sub rsp, 0x118
1405a9168: lea rbp, [rdx + 0x80]
1405a916f: movaps xmmword ptr [rsp + 0x70], xmm15
1405a9175: movaps xmmword ptr [rsp + 0x80], xmm14
1405a917e: movaps xmmword ptr [rsp + 0x90], xmm13
1405a9187: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a9190: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a9199: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a91a2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a91ab: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a91b4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a91bc: movaps xmmword ptr [rsp + 0x100], xmm6
1405a91c4: and rdx, 0xffffffffffffff80
1405a91c8: mov rbx, rdx
1405a91cb: mov rdx, qword ptr [rbx + 0xb00]
1405a91d2: test rdx, rdx
1405a91d5: je 0x1405a91e9
1405a91d7: mov rcx, qword ptr [rbx + 0xb08]
1405a91de: mov r8d, 1
1405a91e4: call 0x140613c20 ; 
1405a91e9: mov byte ptr [rbx + 0xd3], 1
1405a91f0: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a91f8: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a9200: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a9209: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a9212: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a921b: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a9224: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a922d: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a9236: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a923f: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a9245: add rsp, 0x118
1405a924c: pop rbx
1405a924d: pop rdi
1405a924e: pop rsi
1405a924f: pop r12
1405a9251: pop r13
1405a9253: pop r14
1405a9255: pop r15
1405a9257: pop rbp
1405a9258: ret 
1405a9259: nop dword ptr [rax]
1405a9260: mov qword ptr [rsp + 0x10], rdx
1405a9265: push rbp
1405a9266: push r15
1405a9268: push r14
1405a926a: push r13
1405a926c: push r12
1405a926e: push rsi
1405a926f: push rdi
1405a9270: push rbx
1405a9271: sub rsp, 0x118
1405a9278: lea rbp, [rdx + 0x80]
1405a927f: movaps xmmword ptr [rsp + 0x70], xmm15
1405a9285: movaps xmmword ptr [rsp + 0x80], xmm14
1405a928e: movaps xmmword ptr [rsp + 0x90], xmm13
1405a9297: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a92a0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a92a9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a92b2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a92bb: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a92c4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a92cc: movaps xmmword ptr [rsp + 0x100], xmm6
1405a92d4: and rdx, 0xffffffffffffff80
1405a92d8: mov rbx, rdx
1405a92db: lea rcx, [rbx + 0x2590]
1405a92e2: call 0x140592cc0 ; core::ptr::drop_in_place<enum2$<country_core::meta_commander::MetaCommand> >
1405a92e7: lea rcx, [rbx + 0x2bf0]
1405a92ee: call 0x140592cc0 ; core::ptr::drop_in_place<enum2$<country_core::meta_commander::MetaCommand> >
1405a92f3: mov byte ptr [rbx + 0x11e], 1
1405a92fa: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a9302: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a930a: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a9313: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a931c: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a9325: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a932e: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a9337: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a9340: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a9349: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a934f: add rsp, 0x118
1405a9356: pop rbx
1405a9357: pop rdi
1405a9358: pop rsi
1405a9359: pop r12
1405a935b: pop r13
1405a935d: pop r14
1405a935f: pop r15
1405a9361: pop rbp
1405a9362: ret 
1405a9363: nop word ptr cs:[rax + rax]
1405a9370: mov qword ptr [rsp + 0x10], rdx
1405a9375: push rbp
1405a9376: push r15
1405a9378: push r14
1405a937a: push r13
1405a937c: push r12
1405a937e: push rsi
1405a937f: push rdi
1405a9380: push rbx
1405a9381: sub rsp, 0x118
1405a9388: lea rbp, [rdx + 0x80]
1405a938f: movaps xmmword ptr [rsp + 0x70], xmm15
1405a9395: movaps xmmword ptr [rsp + 0x80], xmm14
1405a939e: movaps xmmword ptr [rsp + 0x90], xmm13
1405a93a7: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a93b0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a93b9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a93c2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a93cb: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a93d4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a93dc: movaps xmmword ptr [rsp + 0x100], xmm6
1405a93e4: and rdx, 0xffffffffffffff80
1405a93e8: mov rbx, rdx
1405a93eb: mov rdx, qword ptr [rbx + 0xb00]
1405a93f2: test rdx, rdx
1405a93f5: je 0x1405a9409
1405a93f7: mov rcx, qword ptr [rbx + 0xb08]
1405a93fe: mov r8d, 1
1405a9404: call 0x140613c20 ; 
1405a9409: mov byte ptr [rbx + 0xb4], 1
1405a9410: mov byte ptr [rbx + 0xb5], 1
1405a9417: mov byte ptr [rbx + 0xb6], 1
1405a941e: mov byte ptr [rbx + 0xb7], 0
1405a9425: mov byte ptr [rbx + 0xb8], 1
1405a942c: mov byte ptr [rbx + 0xb9], 1
1405a9433: mov byte ptr [rbx + 0xba], 1
1405a943a: mov byte ptr [rbx + 0xbb], 1
1405a9441: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a9449: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a9451: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a945a: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a9463: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a946c: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a9475: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a947e: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a9487: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a9490: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a9496: add rsp, 0x118
1405a949d: pop rbx
1405a949e: pop rdi
1405a949f: pop rsi
1405a94a0: pop r12
1405a94a2: pop r13
1405a94a4: pop r14
1405a94a6: pop r15
1405a94a8: pop rbp
1405a94a9: ret 
1405a94aa: nop word ptr [rax + rax]
1405a94b0: mov qword ptr [rsp + 0x10], rdx
1405a94b5: push rbp
1405a94b6: push r15
1405a94b8: push r14
1405a94ba: push r13
1405a94bc: push r12
1405a94be: push rsi
1405a94bf: push rdi
1405a94c0: push rbx
1405a94c1: sub rsp, 0x118
1405a94c8: lea rbp, [rdx + 0x80]
1405a94cf: movaps xmmword ptr [rsp + 0x70], xmm15
1405a94d5: movaps xmmword ptr [rsp + 0x80], xmm14
1405a94de: movaps xmmword ptr [rsp + 0x90], xmm13
1405a94e7: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a94f0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a94f9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a9502: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a950b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a9514: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a951c: movaps xmmword ptr [rsp + 0x100], xmm6
1405a9524: and rdx, 0xffffffffffffff80
1405a9528: mov rbx, rdx
1405a952b: mov byte ptr [rbx + 0xb4], 1
1405a9532: mov byte ptr [rbx + 0xb5], 1
1405a9539: mov byte ptr [rbx + 0xb6], 1
1405a9540: mov byte ptr [rbx + 0xb7], 0
1405a9547: mov byte ptr [rbx + 0xb8], 1
1405a954e: mov byte ptr [rbx + 0xb9], 1
1405a9555: mov byte ptr [rbx + 0xba], 1
1405a955c: mov byte ptr [rbx + 0xbb], 1
1405a9563: lea rcx, [rbx + 0x440]
1405a956a: call 0x140d4cc30 ; anyhow::error::impl$6::drop
1405a956f: mov byte ptr [rbx + 0xb4], 1
1405a9576: mov byte ptr [rbx + 0xb5], 1
1405a957d: mov byte ptr [rbx + 0xb6], 1
1405a9584: mov byte ptr [rbx + 0xb7], 0
1405a958b: mov byte ptr [rbx + 0xb8], 1
1405a9592: mov byte ptr [rbx + 0xb9], 1
1405a9599: mov byte ptr [rbx + 0xba], 1
1405a95a0: mov byte ptr [rbx + 0xbb], 1
1405a95a7: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a95af: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a95b7: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a95c0: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a95c9: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a95d2: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a95db: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a95e4: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a95ed: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a95f6: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a95fc: add rsp, 0x118
1405a9603: pop rbx
1405a9604: pop rdi
1405a9605: pop rsi
1405a9606: pop r12
1405a9608: pop r13
1405a960a: pop r14
1405a960c: pop r15
1405a960e: pop rbp
1405a960f: ret 
1405a9610: mov qword ptr [rsp + 0x10], rdx
1405a9615: push rbp
1405a9616: push r15
1405a9618: push r14
1405a961a: push r13
1405a961c: push r12
1405a961e: push rsi
1405a961f: push rdi
1405a9620: push rbx
1405a9621: sub rsp, 0x118
1405a9628: lea rbp, [rdx + 0x80]
1405a962f: movaps xmmword ptr [rsp + 0x70], xmm15
1405a9635: movaps xmmword ptr [rsp + 0x80], xmm14
1405a963e: movaps xmmword ptr [rsp + 0x90], xmm13
1405a9647: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a9650: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a9659: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a9662: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a966b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a9674: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a967c: movaps xmmword ptr [rsp + 0x100], xmm6
1405a9684: and rdx, 0xffffffffffffff80
1405a9688: mov rbx, rdx
1405a968b: lea rcx, [rbx + 0x2590]
1405a9692: call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > > >
1405a9697: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a969f: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a96a7: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a96b0: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a96b9: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a96c2: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a96cb: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a96d4: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a96dd: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a96e6: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a96ec: add rsp, 0x118
1405a96f3: pop rbx
1405a96f4: pop rdi
1405a96f5: pop rsi
1405a96f6: pop r12
1405a96f8: pop r13
1405a96fa: pop r14
1405a96fc: pop r15
1405a96fe: pop rbp
1405a96ff: ret 
1405a9700: mov qword ptr [rsp + 0x10], rdx
1405a9705: push rbp
1405a9706: push r15
1405a9708: push r14
1405a970a: push r13
1405a970c: push r12
1405a970e: push rsi
1405a970f: push rdi
1405a9710: push rbx
1405a9711: sub rsp, 0x118
1405a9718: lea rbp, [rdx + 0x80]
1405a971f: movaps xmmword ptr [rsp + 0x70], xmm15
1405a9725: movaps xmmword ptr [rsp + 0x80], xmm14
1405a972e: movaps xmmword ptr [rsp + 0x90], xmm13
1405a9737: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a9740: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a9749: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a9752: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a975b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a9764: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a976c: movaps xmmword ptr [rsp + 0x100], xmm6
1405a9774: and rdx, 0xffffffffffffff80
1405a9778: mov rbx, rdx
1405a977b: lea rcx, [rbx + 0x2590]
1405a9782: call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > > >
1405a9787: mov byte ptr [rbx + 0xaa], 1
1405a978e: mov byte ptr [rbx + 0xab], 1
1405a9795: mov byte ptr [rbx + 0xac], 1
1405a979c: mov byte ptr [rbx + 0xad], 1
1405a97a3: mov byte ptr [rbx + 0xae], 0
1405a97aa: mov byte ptr [rbx + 0xaf], 1
1405a97b1: mov byte ptr [rbx + 0xb0], 0
1405a97b8: mov byte ptr [rbx + 0xb1], 1
1405a97bf: mov byte ptr [rbx + 0xb2], 1
1405a97c6: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a97ce: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a97d6: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a97df: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a97e8: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a97f1: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a97fa: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a9803: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a980c: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a9815: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a981b: add rsp, 0x118
1405a9822: pop rbx
1405a9823: pop rdi
1405a9824: pop rsi
1405a9825: pop r12
1405a9827: pop r13
1405a9829: pop r14
1405a982b: pop r15
1405a982d: pop rbp
1405a982e: ret 
1405a982f: nop 
1405a9830: mov qword ptr [rsp + 0x10], rdx
1405a9835: push rbp
1405a9836: push r15
1405a9838: push r14
1405a983a: push r13
1405a983c: push r12
1405a983e: push rsi
1405a983f: push rdi
1405a9840: push rbx
1405a9841: sub rsp, 0x118
1405a9848: lea rbp, [rdx + 0x80]
1405a984f: movaps xmmword ptr [rsp + 0x70], xmm15
1405a9855: movaps xmmword ptr [rsp + 0x80], xmm14
1405a985e: movaps xmmword ptr [rsp + 0x90], xmm13
1405a9867: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a9870: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a9879: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a9882: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a988b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a9894: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a989c: movaps xmmword ptr [rsp + 0x100], xmm6
1405a98a4: and rdx, 0xffffffffffffff80
1405a98a8: mov rbx, rdx
1405a98ab: mov byte ptr [rbx + 0xbc], 1
1405a98b2: mov byte ptr [rbx + 0xbd], 1
1405a98b9: mov byte ptr [rbx + 0xbe], 1
1405a98c0: mov byte ptr [rbx + 0xbf], 1
1405a98c7: mov byte ptr [rbx + 0xc0], 1
1405a98ce: mov byte ptr [rbx + 0xc1], 1
1405a98d5: lea rcx, [rbx + 0xb00]
1405a98dc: call 0x140608a80 ; core::ptr::drop_in_place<std::sync::mpsc::SendError<enum2$<tiny_glade::event_loop::GameEvent> > >
1405a98e1: mov byte ptr [rbx + 0xbc], 1
1405a98e8: mov byte ptr [rbx + 0xbd], 1
1405a98ef: mov byte ptr [rbx + 0xbe], 1
1405a98f6: mov byte ptr [rbx + 0xbf], 1
1405a98fd: mov byte ptr [rbx + 0xc0], 1
1405a9904: mov byte ptr [rbx + 0xc1], 1
1405a990b: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a9913: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a991b: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a9924: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a992d: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a9936: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a993f: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a9948: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a9951: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a995a: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a9960: add rsp, 0x118
1405a9967: pop rbx
1405a9968: pop rdi
1405a9969: pop rsi
1405a996a: pop r12
1405a996c: pop r13
1405a996e: pop r14
1405a9970: pop r15
1405a9972: pop rbp
1405a9973: ret 
1405a9974: nop word ptr cs:[rax + rax]
1405a9980: mov qword ptr [rsp + 0x10], rdx
1405a9985: push rbp
1405a9986: push r15
1405a9988: push r14
1405a998a: push r13
1405a998c: push r12
1405a998e: push rsi
1405a998f: push rdi
1405a9990: push rbx
1405a9991: sub rsp, 0x118
1405a9998: lea rbp, [rdx + 0x80]
1405a999f: movaps xmmword ptr [rsp + 0x70], xmm15
1405a99a5: movaps xmmword ptr [rsp + 0x80], xmm14
1405a99ae: movaps xmmword ptr [rsp + 0x90], xmm13
1405a99b7: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a99c0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a99c9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a99d2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a99db: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a99e4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a99ec: movaps xmmword ptr [rsp + 0x100], xmm6
1405a99f4: and rdx, 0xffffffffffffff80
1405a99f8: mov rbx, rdx
1405a99fb: mov byte ptr [rbx + 0xf8], 1
1405a9a02: mov byte ptr [rbx + 0xf9], 1
1405a9a09: mov byte ptr [rbx + 0xfa], 1
1405a9a10: mov byte ptr [rbx + 0xfb], 1
1405a9a17: mov byte ptr [rbx + 0xfc], 1
1405a9a1e: mov byte ptr [rbx + 0xfd], 1
1405a9a25: lea rcx, [rbx + 0xb10]
1405a9a2c: call 0x1405f27d0 ; core::ptr::drop_in_place<rhapsody::raw::device::GraphicsDevice>
1405a9a31: mov byte ptr [rbx + 0xf8], 1
1405a9a38: mov byte ptr [rbx + 0xf9], 1
1405a9a3f: mov byte ptr [rbx + 0xfa], 1
1405a9a46: mov byte ptr [rbx + 0xfb], 1
1405a9a4d: mov byte ptr [rbx + 0xfc], 1
1405a9a54: mov byte ptr [rbx + 0xfd], 1
1405a9a5b: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a9a63: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a9a6b: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a9a74: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a9a7d: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a9a86: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a9a8f: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a9a98: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a9aa1: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a9aaa: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a9ab0: add rsp, 0x118
1405a9ab7: pop rbx
1405a9ab8: pop rdi
1405a9ab9: pop rsi
1405a9aba: pop r12
1405a9abc: pop r13
1405a9abe: pop r14
1405a9ac0: pop r15
1405a9ac2: pop rbp
1405a9ac3: ret 
1405a9ac4: nop word ptr cs:[rax + rax]
1405a9ad0: mov qword ptr [rsp + 0x10], rdx
1405a9ad5: push rbp
1405a9ad6: push r15
1405a9ad8: push r14
1405a9ada: push r13
1405a9adc: push r12
1405a9ade: push rsi
1405a9adf: push rdi
1405a9ae0: push rbx
1405a9ae1: sub rsp, 0x118
1405a9ae8: lea rbp, [rdx + 0x80]
1405a9aef: movaps xmmword ptr [rsp + 0x70], xmm15
1405a9af5: movaps xmmword ptr [rsp + 0x80], xmm14
1405a9afe: movaps xmmword ptr [rsp + 0x90], xmm13
1405a9b07: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a9b10: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a9b19: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a9b22: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a9b2b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a9b34: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a9b3c: movaps xmmword ptr [rsp + 0x100], xmm6
1405a9b44: and rdx, 0xffffffffffffff80
1405a9b48: mov rbx, rdx
1405a9b4b: cmp byte ptr [rbx + 0x111], 0
1405a9b52: je 0x1405a9bdf
1405a9b58: mov rdx, qword ptr [rbx + 0x2590]
1405a9b5f: test rdx, rdx
1405a9b62: je 0x1405a9b76
1405a9b64: mov rcx, qword ptr [rbx + 0x2598]
1405a9b6b: mov r8d, 1
1405a9b71: call 0x140613c20 ; 
1405a9b76: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a9b7e: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a9b86: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a9b8f: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a9b98: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a9ba1: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a9baa: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a9bb3: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a9bbc: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a9bc5: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a9bcb: add rsp, 0x118
1405a9bd2: pop rbx
1405a9bd3: pop rdi
1405a9bd4: pop rsi
1405a9bd5: pop r12
1405a9bd7: pop r13
1405a9bd9: pop r14
1405a9bdb: pop r15
1405a9bdd: pop rbp
1405a9bde: ret 
1405a9bdf: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a9be7: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a9bef: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a9bf8: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a9c01: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a9c0a: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a9c13: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a9c1c: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a9c25: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a9c2e: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a9c34: add rsp, 0x118
1405a9c3b: pop rbx
1405a9c3c: pop rdi
1405a9c3d: pop rsi
1405a9c3e: pop r12
1405a9c40: pop r13
1405a9c42: pop r14
1405a9c44: pop r15
1405a9c46: pop rbp
1405a9c47: ret 
1405a9c48: nop dword ptr [rax + rax]
1405a9c50: mov qword ptr [rsp + 0x10], rdx
1405a9c55: push rbp
1405a9c56: push r15
1405a9c58: push r14
1405a9c5a: push r13
1405a9c5c: push r12
1405a9c5e: push rsi
1405a9c5f: push rdi
1405a9c60: push rbx
1405a9c61: sub rsp, 0x118
1405a9c68: lea rbp, [rdx + 0x80]
1405a9c6f: movaps xmmword ptr [rsp + 0x70], xmm15
1405a9c75: movaps xmmword ptr [rsp + 0x80], xmm14
1405a9c7e: movaps xmmword ptr [rsp + 0x90], xmm13
1405a9c87: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a9c90: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a9c99: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a9ca2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a9cab: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a9cb4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a9cbc: movaps xmmword ptr [rsp + 0x100], xmm6
1405a9cc4: and rdx, 0xffffffffffffff80
1405a9cc8: mov rbx, rdx
1405a9ccb: cmp byte ptr [rbx + 0x11e], 0
1405a9cd2: je 0x1405a9d85
1405a9cd8: lea rcx, [rbx + 0x440]
1405a9cdf: call 0x1405f9a70 ; core::ptr::drop_in_place<country_core::resources::session::NextSession>
1405a9ce4: mov byte ptr [rbx + 0xb4], 1
1405a9ceb: mov byte ptr [rbx + 0xb5], 1
1405a9cf2: mov byte ptr [rbx + 0xb6], 1
1405a9cf9: mov byte ptr [rbx + 0xb7], 0
1405a9d00: mov byte ptr [rbx + 0xb8], 1
1405a9d07: mov byte ptr [rbx + 0xb9], 1
1405a9d0e: mov byte ptr [rbx + 0xba], 1
1405a9d15: mov byte ptr [rbx + 0xbb], 1
1405a9d1c: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a9d24: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a9d2c: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a9d35: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a9d3e: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a9d47: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a9d50: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a9d59: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a9d62: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a9d6b: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a9d71: add rsp, 0x118
1405a9d78: pop rbx
1405a9d79: pop rdi
1405a9d7a: pop rsi
1405a9d7b: pop r12
1405a9d7d: pop r13
1405a9d7f: pop r14
1405a9d81: pop r15
1405a9d83: pop rbp
1405a9d84: ret 
1405a9d85: mov byte ptr [rbx + 0xb4], 1
1405a9d8c: mov byte ptr [rbx + 0xb5], 1
1405a9d93: mov byte ptr [rbx + 0xb6], 1
1405a9d9a: mov byte ptr [rbx + 0xb7], 0
1405a9da1: mov byte ptr [rbx + 0xb8], 1
1405a9da8: mov byte ptr [rbx + 0xb9], 1
1405a9daf: mov byte ptr [rbx + 0xba], 1
1405a9db6: mov byte ptr [rbx + 0xbb], 1
1405a9dbd: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a9dc5: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a9dcd: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a9dd6: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a9ddf: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a9de8: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a9df1: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a9dfa: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a9e03: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a9e0c: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a9e12: add rsp, 0x118
1405a9e19: pop rbx
1405a9e1a: pop rdi
1405a9e1b: pop rsi
1405a9e1c: pop r12
1405a9e1e: pop r13
1405a9e20: pop r14
1405a9e22: pop r15
1405a9e24: pop rbp
1405a9e25: ret 
1405a9e26: nop word ptr cs:[rax + rax]
1405a9e30: mov qword ptr [rsp + 0x10], rdx
1405a9e35: push rbp
1405a9e36: push r15
1405a9e38: push r14
1405a9e3a: push r13
1405a9e3c: push r12
1405a9e3e: push rsi
1405a9e3f: push rdi
1405a9e40: push rbx
1405a9e41: sub rsp, 0x118
1405a9e48: lea rbp, [rdx + 0x80]
1405a9e4f: movaps xmmword ptr [rsp + 0x70], xmm15
1405a9e55: movaps xmmword ptr [rsp + 0x80], xmm14
1405a9e5e: movaps xmmword ptr [rsp + 0x90], xmm13
1405a9e67: movaps xmmword ptr [rsp + 0xa0], xmm12
1405a9e70: movaps xmmword ptr [rsp + 0xb0], xmm11
1405a9e79: movaps xmmword ptr [rsp + 0xc0], xmm10
1405a9e82: movaps xmmword ptr [rsp + 0xd0], xmm9
1405a9e8b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405a9e94: movaps xmmword ptr [rsp + 0xf0], xmm7
1405a9e9c: movaps xmmword ptr [rsp + 0x100], xmm6
1405a9ea4: and rdx, 0xffffffffffffff80
1405a9ea8: mov rbx, rdx
1405a9eab: movzx esi, byte ptr [rbx + 0xdb]
1405a9eb2: test sil, 1
1405a9eb6: je 0x1405a9f53
1405a9ebc: mov rax, qword ptr [rbx + 0x1ff0]
1405a9ec3: lock dec qword ptr [rax]
1405a9ec7: jne 0x1405a9ed5
1405a9ec9: lea rcx, [rbx + 0x1ff0]
1405a9ed0: call 0x140b5be90 ; alloc::sync::Arc::drop_slow<rhapsody::raw::device::GraphicsDevice,alloc::alloc::Global>
1405a9ed5: mov byte ptr [rbx + 0xf5], sil
1405a9edc: mov byte ptr [rbx + 0xf6], 1
1405a9ee3: mov byte ptr [rbx + 0xf7], 1
1405a9eea: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a9ef2: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a9efa: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a9f03: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a9f0c: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a9f15: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a9f1e: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a9f27: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a9f30: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a9f39: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a9f3f: add rsp, 0x118
1405a9f46: pop rbx
1405a9f47: pop rdi
1405a9f48: pop rsi
1405a9f49: pop r12
1405a9f4b: pop r13
1405a9f4d: pop r14
1405a9f4f: pop r15
1405a9f51: pop rbp
1405a9f52: ret 
1405a9f53: mov byte ptr [rbx + 0xf5], sil
1405a9f5a: mov byte ptr [rbx + 0xf6], 1
1405a9f61: mov byte ptr [rbx + 0xf7], 1
1405a9f68: movaps xmm6, xmmword ptr [rsp + 0x100]
1405a9f70: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405a9f78: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405a9f81: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405a9f8a: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405a9f93: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405a9f9c: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405a9fa5: movaps xmm13, xmmword ptr [rsp + 0x90]
1405a9fae: movaps xmm14, xmmword ptr [rsp + 0x80]
1405a9fb7: movaps xmm15, xmmword ptr [rsp + 0x70]
1405a9fbd: add rsp, 0x118
1405a9fc4: pop rbx
1405a9fc5: pop rdi
1405a9fc6: pop rsi
1405a9fc7: pop r12
1405a9fc9: pop r13
1405a9fcb: pop r14
1405a9fcd: pop r15
1405a9fcf: pop rbp
1405a9fd0: ret 
1405a9fd1: nop word ptr cs:[rax + rax]
1405a9fe0: mov qword ptr [rsp + 0x10], rdx
1405a9fe5: push rbp
1405a9fe6: push r15
1405a9fe8: push r14
1405a9fea: push r13
1405a9fec: push r12
1405a9fee: push rsi
1405a9fef: push rdi
1405a9ff0: push rbx
1405a9ff1: sub rsp, 0x118
1405a9ff8: lea rbp, [rdx + 0x80]
1405a9fff: movaps xmmword ptr [rsp + 0x70], xmm15
1405aa005: movaps xmmword ptr [rsp + 0x80], xmm14
1405aa00e: movaps xmmword ptr [rsp + 0x90], xmm13
1405aa017: movaps xmmword ptr [rsp + 0xa0], xmm12
1405aa020: movaps xmmword ptr [rsp + 0xb0], xmm11
1405aa029: movaps xmmword ptr [rsp + 0xc0], xmm10
1405aa032: movaps xmmword ptr [rsp + 0xd0], xmm9
1405aa03b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405aa044: movaps xmmword ptr [rsp + 0xf0], xmm7
1405aa04c: movaps xmmword ptr [rsp + 0x100], xmm6
1405aa054: and rdx, 0xffffffffffffff80
1405aa058: mov rbx, rdx
1405aa05b: movzx esi, byte ptr [rbx + 0x105]
1405aa062: test sil, 1
1405aa066: je 0x1405aa13f
1405aa06c: lea rcx, [rbx + 0x24e8]
1405aa073: call 0x1405f7630 ; core::ptr::drop_in_place<tiny_glade::run_game::RenderResourceCache>
1405aa078: mov rax, qword ptr [rbx + 0x170]
1405aa07f: lock dec qword ptr [rax]
1405aa083: jne 0x1405aa091
1405aa085: lea rcx, [rbx + 0x170]
1405aa08c: call 0x140631840 ; alloc::sync::Arc::drop_slow<winit::window::Window,alloc::alloc::Global>
1405aa091: lea rcx, [rbx + 0x280]
1405aa098: call 0x140606870 ; core::ptr::drop_in_place<std::sync::mpsc::Sender<enum2$<tiny_glade::event_loop::GameEvent> > >
1405aa09d: lea rcx, [rbx + 0x270]
1405aa0a4: call 0x14060a2c0 ; core::ptr::drop_in_place<std::sync::mpsc::Receiver<enum2$<tiny_glade::event_loop::OsWindowEvent> > >
1405aa0a9: lea rcx, [rbx + 0x340]
1405aa0b0: call 0x140602c80 ; core::ptr::drop_in_place<std::sync::mpsc::Receiver<bevy_app::sub_app::SubApp> >
1405aa0b5: lea rcx, [rbx + 0x440]
1405aa0bc: call 0x1405e6cd0 ; core::ptr::drop_in_place<bevy_app::app::App>
1405aa0c1: mov byte ptr [rbx + 0xf5], 0
1405aa0c8: mov byte ptr [rbx + 0xf6], 0
1405aa0cf: mov byte ptr [rbx + 0xf7], sil
1405aa0d6: movaps xmm6, xmmword ptr [rsp + 0x100]
1405aa0de: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405aa0e6: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405aa0ef: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405aa0f8: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405aa101: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405aa10a: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405aa113: movaps xmm13, xmmword ptr [rsp + 0x90]
1405aa11c: movaps xmm14, xmmword ptr [rsp + 0x80]
1405aa125: movaps xmm15, xmmword ptr [rsp + 0x70]
1405aa12b: add rsp, 0x118
1405aa132: pop rbx
1405aa133: pop rdi
1405aa134: pop rsi
1405aa135: pop r12
1405aa137: pop r13
1405aa139: pop r14
1405aa13b: pop r15
1405aa13d: pop rbp
1405aa13e: ret 
1405aa13f: mov byte ptr [rbx + 0xf5], 0
1405aa146: mov byte ptr [rbx + 0xf6], 0
1405aa14d: mov byte ptr [rbx + 0xf7], sil
1405aa154: movaps xmm6, xmmword ptr [rsp + 0x100]
1405aa15c: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405aa164: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405aa16d: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405aa176: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405aa17f: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405aa188: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405aa191: movaps xmm13, xmmword ptr [rsp + 0x90]
1405aa19a: movaps xmm14, xmmword ptr [rsp + 0x80]
1405aa1a3: movaps xmm15, xmmword ptr [rsp + 0x70]
1405aa1a9: add rsp, 0x118
1405aa1b0: pop rbx
1405aa1b1: pop rdi
1405aa1b2: pop rsi
1405aa1b3: pop r12
1405aa1b5: pop r13
1405aa1b7: pop r14
1405aa1b9: pop r15
1405aa1bb: pop rbp
1405aa1bc: ret 
1405aa1bd: nop dword ptr [rax]
1405aa1c0: mov qword ptr [rsp + 0x10], rdx
1405aa1c5: push rbp
1405aa1c6: push r15
1405aa1c8: push r14
1405aa1ca: push r13
1405aa1cc: push r12
1405aa1ce: push rsi
1405aa1cf: push rdi
1405aa1d0: push rbx
1405aa1d1: sub rsp, 0x118
1405aa1d8: lea rbp, [rdx + 0x80]
1405aa1df: movaps xmmword ptr [rsp + 0x70], xmm15
1405aa1e5: movaps xmmword ptr [rsp + 0x80], xmm14
1405aa1ee: movaps xmmword ptr [rsp + 0x90], xmm13
1405aa1f7: movaps xmmword ptr [rsp + 0xa0], xmm12
1405aa200: movaps xmmword ptr [rsp + 0xb0], xmm11
1405aa209: movaps xmmword ptr [rsp + 0xc0], xmm10
1405aa212: movaps xmmword ptr [rsp + 0xd0], xmm9
1405aa21b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405aa224: movaps xmmword ptr [rsp + 0xf0], xmm7
1405aa22c: movaps xmmword ptr [rsp + 0x100], xmm6
1405aa234: and rdx, 0xffffffffffffff80
1405aa238: mov rbx, rdx
1405aa23b: movzx eax, byte ptr [rbx + 0x106]
1405aa242: mov byte ptr [rbx + 0xe8], al
1405aa248: test al, 1
1405aa24a: je 0x1405aa2de
1405aa250: lea rcx, [rbx + 0x1ff0]
1405aa257: call 0x1405f7630 ; core::ptr::drop_in_place<tiny_glade::run_game::RenderResourceCache>
1405aa25c: mov rax, qword ptr [rbx + 0x270]
1405aa263: lock dec qword ptr [rax]
1405aa267: jne 0x1405aa275
1405aa269: lea rcx, [rbx + 0x270]
1405aa270: call 0x140631840 ; alloc::sync::Arc::drop_slow<winit::window::Window,alloc::alloc::Global>
1405aa275: movaps xmm6, xmmword ptr [rsp + 0x100]
1405aa27d: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405aa285: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405aa28e: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405aa297: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405aa2a0: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405aa2a9: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405aa2b2: movaps xmm13, xmmword ptr [rsp + 0x90]
1405aa2bb: movaps xmm14, xmmword ptr [rsp + 0x80]
1405aa2c4: movaps xmm15, xmmword ptr [rsp + 0x70]
1405aa2ca: add rsp, 0x118
1405aa2d1: pop rbx
1405aa2d2: pop rdi
1405aa2d3: pop rsi
1405aa2d4: pop r12
1405aa2d6: pop r13
1405aa2d8: pop r14
1405aa2da: pop r15
1405aa2dc: pop rbp
1405aa2dd: ret 
1405aa2de: movaps xmm6, xmmword ptr [rsp + 0x100]
1405aa2e6: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405aa2ee: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405aa2f7: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405aa300: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405aa309: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405aa312: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405aa31b: movaps xmm13, xmmword ptr [rsp + 0x90]
1405aa324: movaps xmm14, xmmword ptr [rsp + 0x80]
1405aa32d: movaps xmm15, xmmword ptr [rsp + 0x70]
1405aa333: add rsp, 0x118
1405aa33a: pop rbx
1405aa33b: pop rdi
1405aa33c: pop rsi
1405aa33d: pop r12
1405aa33f: pop r13
1405aa341: pop r14
1405aa343: pop r15
1405aa345: pop rbp
1405aa346: ret 
1405aa347: nop word ptr [rax + rax]
1405aa350: mov qword ptr [rsp + 0x10], rdx
1405aa355: push rbp
1405aa356: push r15
1405aa358: push r14
1405aa35a: push r13
1405aa35c: push r12
1405aa35e: push rsi
1405aa35f: push rdi
1405aa360: push rbx
1405aa361: sub rsp, 0x118
1405aa368: lea rbp, [rdx + 0x80]
1405aa36f: movaps xmmword ptr [rsp + 0x70], xmm15
1405aa375: movaps xmmword ptr [rsp + 0x80], xmm14
1405aa37e: movaps xmmword ptr [rsp + 0x90], xmm13
1405aa387: movaps xmmword ptr [rsp + 0xa0], xmm12
1405aa390: movaps xmmword ptr [rsp + 0xb0], xmm11
1405aa399: movaps xmmword ptr [rsp + 0xc0], xmm10
1405aa3a2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405aa3ab: movaps xmmword ptr [rsp + 0xe0], xmm8
1405aa3b4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405aa3bc: movaps xmmword ptr [rsp + 0x100], xmm6
1405aa3c4: and rdx, 0xffffffffffffff80
1405aa3c8: mov rbx, rdx
1405aa3cb: test byte ptr [rbx + 0xe8], 1
1405aa3d2: je 0x1405aa449
1405aa3d4: lea rcx, [rbx + 0x370]
1405aa3db: call 0x140606870 ; core::ptr::drop_in_place<std::sync::mpsc::Sender<enum2$<tiny_glade::event_loop::GameEvent> > >
1405aa3e0: movaps xmm6, xmmword ptr [rsp + 0x100]
1405aa3e8: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405aa3f0: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405aa3f9: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405aa402: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405aa40b: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405aa414: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405aa41d: movaps xmm13, xmmword ptr [rsp + 0x90]
1405aa426: movaps xmm14, xmmword ptr [rsp + 0x80]
1405aa42f: movaps xmm15, xmmword ptr [rsp + 0x70]
1405aa435: add rsp, 0x118
1405aa43c: pop rbx
1405aa43d: pop rdi
1405aa43e: pop rsi
1405aa43f: pop r12
1405aa441: pop r13
1405aa443: pop r14
1405aa445: pop r15
1405aa447: pop rbp
1405aa448: ret 
1405aa449: movaps xmm6, xmmword ptr [rsp + 0x100]
1405aa451: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405aa459: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405aa462: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405aa46b: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405aa474: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405aa47d: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405aa486: movaps xmm13, xmmword ptr [rsp + 0x90]
1405aa48f: movaps xmm14, xmmword ptr [rsp + 0x80]
1405aa498: movaps xmm15, xmmword ptr [rsp + 0x70]
1405aa49e: add rsp, 0x118
1405aa4a5: pop rbx
1405aa4a6: pop rdi
1405aa4a7: pop rsi
1405aa4a8: pop r12
1405aa4aa: pop r13
1405aa4ac: pop r14
1405aa4ae: pop r15
1405aa4b0: pop rbp
1405aa4b1: ret 
1405aa4b2: nop word ptr cs:[rax + rax]
1405aa4c0: mov qword ptr [rsp + 0x10], rdx
1405aa4c5: push rbp
1405aa4c6: push r15
1405aa4c8: push r14
1405aa4ca: push r13
1405aa4cc: push r12
1405aa4ce: push rsi
1405aa4cf: push rdi
1405aa4d0: push rbx
1405aa4d1: sub rsp, 0x118
1405aa4d8: lea rbp, [rdx + 0x80]
1405aa4df: movaps xmmword ptr [rsp + 0x70], xmm15
1405aa4e5: movaps xmmword ptr [rsp + 0x80], xmm14
1405aa4ee: movaps xmmword ptr [rsp + 0x90], xmm13
1405aa4f7: movaps xmmword ptr [rsp + 0xa0], xmm12
1405aa500: movaps xmmword ptr [rsp + 0xb0], xmm11
1405aa509: movaps xmmword ptr [rsp + 0xc0], xmm10
1405aa512: movaps xmmword ptr [rsp + 0xd0], xmm9
1405aa51b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405aa524: movaps xmmword ptr [rsp + 0xf0], xmm7
1405aa52c: movaps xmmword ptr [rsp + 0x100], xmm6
1405aa534: and rdx, 0xffffffffffffff80
1405aa538: mov rbx, rdx
1405aa53b: test byte ptr [rbx + 0xe8], 1
1405aa542: je 0x1405aa5fe
1405aa548: lea rcx, [rbx + 0x24e8]
1405aa54f: call 0x14060a2c0 ; core::ptr::drop_in_place<std::sync::mpsc::Receiver<enum2$<tiny_glade::event_loop::OsWindowEvent> > >
1405aa554: lea rcx, [rbx + 0x280]
1405aa55b: call 0x140602c80 ; core::ptr::drop_in_place<std::sync::mpsc::Receiver<bevy_app::sub_app::SubApp> >
1405aa560: lea rcx, [rbx + 0x440]
1405aa567: call 0x1405e6cd0 ; core::ptr::drop_in_place<bevy_app::app::App>
1405aa56c: mov byte ptr [rbx + 0xc2], 0
1405aa573: mov byte ptr [rbx + 0xc3], 0
1405aa57a: mov byte ptr [rbx + 0xc4], 1
1405aa581: mov byte ptr [rbx + 0xc5], 1
1405aa588: movzx eax, byte ptr [rbx + 0xe8]
1405aa58f: mov byte ptr [rbx + 0xc6], al
1405aa595: movaps xmm6, xmmword ptr [rsp + 0x100]
1405aa59d: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405aa5a5: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405aa5ae: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405aa5b7: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405aa5c0: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405aa5c9: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405aa5d2: movaps xmm13, xmmword ptr [rsp + 0x90]
1405aa5db: movaps xmm14, xmmword ptr [rsp + 0x80]
1405aa5e4: movaps xmm15, xmmword ptr [rsp + 0x70]
1405aa5ea: add rsp, 0x118
1405aa5f1: pop rbx
1405aa5f2: pop rdi
1405aa5f3: pop rsi
1405aa5f4: pop r12
1405aa5f6: pop r13
1405aa5f8: pop r14
1405aa5fa: pop r15
1405aa5fc: pop rbp
1405aa5fd: ret 
1405aa5fe: mov byte ptr [rbx + 0xc2], 0
1405aa605: mov byte ptr [rbx + 0xc3], 0
1405aa60c: mov byte ptr [rbx + 0xc4], 1
1405aa613: mov byte ptr [rbx + 0xc5], 1
1405aa61a: movzx eax, byte ptr [rbx + 0xe8]
1405aa621: mov byte ptr [rbx + 0xc6], al
1405aa627: movaps xmm6, xmmword ptr [rsp + 0x100]
1405aa62f: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405aa637: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405aa640: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405aa649: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405aa652: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405aa65b: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405aa664: movaps xmm13, xmmword ptr [rsp + 0x90]
1405aa66d: movaps xmm14, xmmword ptr [rsp + 0x80]
1405aa676: movaps xmm15, xmmword ptr [rsp + 0x70]
1405aa67c: add rsp, 0x118
1405aa683: pop rbx
1405aa684: pop rdi
1405aa685: pop rsi
1405aa686: pop r12
1405aa688: pop r13
1405aa68a: pop r14
1405aa68c: pop r15
1405aa68e: pop rbp
1405aa68f: ret 
1405aa690: mov qword ptr [rsp + 0x10], rdx
1405aa695: push rbp
1405aa696: push r15
1405aa698: push r14
1405aa69a: push r13
1405aa69c: push r12
1405aa69e: push rsi
1405aa69f: push rdi
1405aa6a0: push rbx
1405aa6a1: sub rsp, 0x118
1405aa6a8: lea rbp, [rdx + 0x80]
1405aa6af: movaps xmmword ptr [rsp + 0x70], xmm15
1405aa6b5: movaps xmmword ptr [rsp + 0x80], xmm14
1405aa6be: movaps xmmword ptr [rsp + 0x90], xmm13
1405aa6c7: movaps xmmword ptr [rsp + 0xa0], xmm12
1405aa6d0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405aa6d9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405aa6e2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405aa6eb: movaps xmmword ptr [rsp + 0xe0], xmm8
1405aa6f4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405aa6fc: movaps xmmword ptr [rsp + 0x100], xmm6
1405aa704: and rdx, 0xffffffffffffff80
1405aa708: mov rbx, rdx
1405aa70b: cmp qword ptr [rbx + 0x128], 0
1405aa713: jne 0x1405aa7cb
1405aa719: mov rdx, qword ptr [rbx + 0xb10]
1405aa720: lea rax, [rdx*2]
1405aa728: test rax, rax
1405aa72b: je 0x1405aa73f
1405aa72d: mov rcx, qword ptr [rbx + 0xb18]
1405aa734: mov r8d, 1
1405aa73a: call 0x140613c20 ; 
1405aa73f: mov byte ptr [rbx + 0xd4], 1
1405aa746: mov byte ptr [rbx + 0xd5], 1
1405aa74d: mov byte ptr [rbx + 0xd6], 1
1405aa754: mov byte ptr [rbx + 0xd7], 1
1405aa75b: mov byte ptr [rbx + 0xd8], 1
1405aa762: movaps xmm6, xmmword ptr [rsp + 0x100]
1405aa76a: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405aa772: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405aa77b: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405aa784: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405aa78d: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405aa796: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405aa79f: movaps xmm13, xmmword ptr [rsp + 0x90]
1405aa7a8: movaps xmm14, xmmword ptr [rsp + 0x80]
1405aa7b1: movaps xmm15, xmmword ptr [rsp + 0x70]
1405aa7b7: add rsp, 0x118
1405aa7be: pop rbx
1405aa7bf: pop rdi
1405aa7c0: pop rsi
1405aa7c1: pop r12
1405aa7c3: pop r13
1405aa7c5: pop r14
1405aa7c7: pop r15
1405aa7c9: pop rbp
1405aa7ca: ret 
1405aa7cb: mov byte ptr [rbx + 0xd4], 1
1405aa7d2: mov byte ptr [rbx + 0xd5], 1
1405aa7d9: mov byte ptr [rbx + 0xd6], 1
1405aa7e0: mov byte ptr [rbx + 0xd7], 1
1405aa7e7: mov byte ptr [rbx + 0xd8], 1
1405aa7ee: movaps xmm6, xmmword ptr [rsp + 0x100]
1405aa7f6: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405aa7fe: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405aa807: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405aa810: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405aa819: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405aa822: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405aa82b: movaps xmm13, xmmword ptr [rsp + 0x90]
1405aa834: movaps xmm14, xmmword ptr [rsp + 0x80]
1405aa83d: movaps xmm15, xmmword ptr [rsp + 0x70]
1405aa843: add rsp, 0x118
1405aa84a: pop rbx
1405aa84b: pop rdi
1405aa84c: pop rsi
1405aa84d: pop r12
1405aa84f: pop r13
1405aa851: pop r14
1405aa853: pop r15
1405aa855: pop rbp
1405aa856: ret 
1405aa857: nop word ptr [rax + rax]
1405aa860: mov qword ptr [rsp + 0x10], rdx
1405aa865: push rbp
1405aa866: push r15
1405aa868: push r14
1405aa86a: push r13
1405aa86c: push r12
1405aa86e: push rsi
1405aa86f: push rdi
1405aa870: push rbx
1405aa871: sub rsp, 0x118
1405aa878: lea rbp, [rdx + 0x80]
1405aa87f: movaps xmmword ptr [rsp + 0x70], xmm15
1405aa885: movaps xmmword ptr [rsp + 0x80], xmm14
1405aa88e: movaps xmmword ptr [rsp + 0x90], xmm13
1405aa897: movaps xmmword ptr [rsp + 0xa0], xmm12
1405aa8a0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405aa8a9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405aa8b2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405aa8bb: movaps xmmword ptr [rsp + 0xe0], xmm8
1405aa8c4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405aa8cc: movaps xmmword ptr [rsp + 0x100], xmm6
1405aa8d4: and rdx, 0xffffffffffffff80
1405aa8d8: mov rbx, rdx
1405aa8db: mov rdx, qword ptr [rbx + 0xb00]
1405aa8e2: test rdx, rdx
1405aa8e5: je 0x1405aa8f9
1405aa8e7: mov rcx, qword ptr [rbx + 0xb08]
1405aa8ee: mov r8d, 1
1405aa8f4: call 0x140613c20 ; 
1405aa8f9: mov byte ptr [rbx + 0xb4], 1
1405aa900: mov byte ptr [rbx + 0xb5], 1
1405aa907: mov byte ptr [rbx + 0xb6], 1
1405aa90e: mov byte ptr [rbx + 0xb7], 0
1405aa915: mov byte ptr [rbx + 0xb8], 1
1405aa91c: mov byte ptr [rbx + 0xb9], 1
1405aa923: mov byte ptr [rbx + 0xba], 1
1405aa92a: mov byte ptr [rbx + 0xbb], 1
1405aa931: movaps xmm6, xmmword ptr [rsp + 0x100]
1405aa939: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405aa941: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405aa94a: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405aa953: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405aa95c: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405aa965: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405aa96e: movaps xmm13, xmmword ptr [rsp + 0x90]
1405aa977: movaps xmm14, xmmword ptr [rsp + 0x80]
1405aa980: movaps xmm15, xmmword ptr [rsp + 0x70]
1405aa986: add rsp, 0x118
1405aa98d: pop rbx
1405aa98e: pop rdi
1405aa98f: pop rsi
1405aa990: pop r12
1405aa992: pop r13
1405aa994: pop r14
1405aa996: pop r15
1405aa998: pop rbp
1405aa999: ret 
1405aa99a: nop word ptr [rax + rax]
1405aa9a0: mov qword ptr [rsp + 0x10], rdx
1405aa9a5: push rbp
1405aa9a6: push r15
1405aa9a8: push r14
1405aa9aa: push r13
1405aa9ac: push r12
1405aa9ae: push rsi
1405aa9af: push rdi
1405aa9b0: push rbx
1405aa9b1: sub rsp, 0x118
1405aa9b8: lea rbp, [rdx + 0x80]
1405aa9bf: movaps xmmword ptr [rsp + 0x70], xmm15
1405aa9c5: movaps xmmword ptr [rsp + 0x80], xmm14
1405aa9ce: movaps xmmword ptr [rsp + 0x90], xmm13
1405aa9d7: movaps xmmword ptr [rsp + 0xa0], xmm12
1405aa9e0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405aa9e9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405aa9f2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405aa9fb: movaps xmmword ptr [rsp + 0xe0], xmm8
1405aaa04: movaps xmmword ptr [rsp + 0xf0], xmm7
1405aaa0c: movaps xmmword ptr [rsp + 0x100], xmm6
1405aaa14: and rdx, 0xffffffffffffff80
1405aaa18: mov rbx, rdx
1405aaa1b: lea rcx, [rbx + 0x2590]
1405aaa22: call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > > >
1405aaa27: movaps xmm6, xmmword ptr [rsp + 0x100]
1405aaa2f: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405aaa37: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405aaa40: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405aaa49: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405aaa52: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405aaa5b: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405aaa64: movaps xmm13, xmmword ptr [rsp + 0x90]
1405aaa6d: movaps xmm14, xmmword ptr [rsp + 0x80]
1405aaa76: movaps xmm15, xmmword ptr [rsp + 0x70]
1405aaa7c: add rsp, 0x118
1405aaa83: pop rbx
1405aaa84: pop rdi
1405aaa85: pop rsi
1405aaa86: pop r12
1405aaa88: pop r13
1405aaa8a: pop r14
1405aaa8c: pop r15
1405aaa8e: pop rbp
1405aaa8f: ret 
1405aaa90: mov qword ptr [rsp + 0x10], rdx
1405aaa95: push rbp
1405aaa96: push r15
1405aaa98: push r14
1405aaa9a: push r13
1405aaa9c: push r12
1405aaa9e: push rsi
1405aaa9f: push rdi
1405aaaa0: push rbx
1405aaaa1: sub rsp, 0x118
1405aaaa8: lea rbp, [rdx + 0x80]
1405aaaaf: movaps xmmword ptr [rsp + 0x70], xmm15
1405aaab5: movaps xmmword ptr [rsp + 0x80], xmm14
1405aaabe: movaps xmmword ptr [rsp + 0x90], xmm13
1405aaac7: movaps xmmword ptr [rsp + 0xa0], xmm12
1405aaad0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405aaad9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405aaae2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405aaaeb: movaps xmmword ptr [rsp + 0xe0], xmm8
1405aaaf4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405aaafc: movaps xmmword ptr [rsp + 0x100], xmm6
1405aab04: and rdx, 0xffffffffffffff80
1405aab08: mov rbx, rdx
1405aab0b: mov byte ptr [rbx + 0xaa], 1
1405aab12: mov byte ptr [rbx + 0xab], 1
1405aab19: mov byte ptr [rbx + 0xac], 1
1405aab20: mov byte ptr [rbx + 0xad], 1
1405aab27: mov byte ptr [rbx + 0xae], 0
1405aab2e: mov byte ptr [rbx + 0xaf], 1
1405aab35: mov byte ptr [rbx + 0xb0], 0
1405aab3c: mov byte ptr [rbx + 0xb1], 1
1405aab43: mov byte ptr [rbx + 0xb2], 1
1405aab4a: lea rcx, [rbx + 0xb00]
1405aab51: call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > > >
1405aab56: mov byte ptr [rbx + 0xaa], 1
1405aab5d: mov byte ptr [rbx + 0xab], 1
1405aab64: mov byte ptr [rbx + 0xac], 1
1405aab6b: mov byte ptr [rbx + 0xad], 1
1405aab72: mov byte ptr [rbx + 0xae], 0
1405aab79: mov byte ptr [rbx + 0xaf], 1
1405aab80: mov byte ptr [rbx + 0xb0], 0
1405aab87: mov byte ptr [rbx + 0xb1], 1
1405aab8e: mov byte ptr [rbx + 0xb2], 1
1405aab95: movaps xmm6, xmmword ptr [rsp + 0x100]
1405aab9d: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405aaba5: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405aabae: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405aabb7: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405aabc0: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405aabc9: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405aabd2: movaps xmm13, xmmword ptr [rsp + 0x90]
1405aabdb: movaps xmm14, xmmword ptr [rsp + 0x80]
1405aabe4: movaps xmm15, xmmword ptr [rsp + 0x70]
1405aabea: add rsp, 0x118
1405aabf1: pop rbx
1405aabf2: pop rdi
1405aabf3: pop rsi
1405aabf4: pop r12
1405aabf6: pop r13
1405aabf8: pop r14
1405aabfa: pop r15
1405aabfc: pop rbp
1405aabfd: ret 
1405aabfe: nop 
1405aac00: mov qword ptr [rsp + 0x10], rdx
1405aac05: push rbp
1405aac06: push r15
1405aac08: push r14
1405aac0a: push r13
1405aac0c: push r12
1405aac0e: push rsi
1405aac0f: push rdi
1405aac10: push rbx
1405aac11: sub rsp, 0x118
1405aac18: lea rbp, [rdx + 0x80]
1405aac1f: movaps xmmword ptr [rsp + 0x70], xmm15
1405aac25: movaps xmmword ptr [rsp + 0x80], xmm14
1405aac2e: movaps xmmword ptr [rsp + 0x90], xmm13
1405aac37: movaps xmmword ptr [rsp + 0xa0], xmm12
1405aac40: movaps xmmword ptr [rsp + 0xb0], xmm11
1405aac49: movaps xmmword ptr [rsp + 0xc0], xmm10
1405aac52: movaps xmmword ptr [rsp + 0xd0], xmm9
1405aac5b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405aac64: movaps xmmword ptr [rsp + 0xf0], xmm7
1405aac6c: movaps xmmword ptr [rsp + 0x100], xmm6
1405aac74: and rdx, 0xffffffffffffff80
1405aac78: mov rbx, rdx
1405aac7b: lea rcx, [rbx + 0x2bf0]
1405aac82: call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > > >
1405aac87: movaps xmm6, xmmword ptr [rsp + 0x100]
1405aac8f: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405aac97: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405aaca0: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405aaca9: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405aacb2: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405aacbb: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405aacc4: movaps xmm13, xmmword ptr [rsp + 0x90]
1405aaccd: movaps xmm14, xmmword ptr [rsp + 0x80]
1405aacd6: movaps xmm15, xmmword ptr [rsp + 0x70]
1405aacdc: add rsp, 0x118
1405aace3: pop rbx
1405aace4: pop rdi
1405aace5: pop rsi
1405aace6: pop r12
1405aace8: pop r13
1405aacea: pop r14
1405aacec: pop r15
1405aacee: pop rbp
1405aacef: ret 
1405aacf0: mov qword ptr [rsp + 0x10], rdx
1405aacf5: push rbp
1405aacf6: push r15
1405aacf8: push r14
1405aacfa: push r13
1405aacfc: push r12
1405aacfe: push rsi
1405aacff: push rdi
1405aad00: push rbx
1405aad01: sub rsp, 0x118
1405aad08: lea rbp, [rdx + 0x80]
1405aad0f: movaps xmmword ptr [rsp + 0x70], xmm15
1405aad15: movaps xmmword ptr [rsp + 0x80], xmm14
1405aad1e: movaps xmmword ptr [rsp + 0x90], xmm13
1405aad27: movaps xmmword ptr [rsp + 0xa0], xmm12
1405aad30: movaps xmmword ptr [rsp + 0xb0], xmm11
1405aad39: movaps xmmword ptr [rsp + 0xc0], xmm10
1405aad42: movaps xmmword ptr [rsp + 0xd0], xmm9
1405aad4b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405aad54: movaps xmmword ptr [rsp + 0xf0], xmm7
1405aad5c: movaps xmmword ptr [rsp + 0x100], xmm6
1405aad64: and rdx, 0xffffffffffffff80
1405aad68: mov rbx, rdx
1405aad6b: lea rcx, [rbx + 0xb00]
1405aad72: call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > > >
1405aad77: movaps xmm6, xmmword ptr [rsp + 0x100]
1405aad7f: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405aad87: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405aad90: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405aad99: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405aada2: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405aadab: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405aadb4: movaps xmm13, xmmword ptr [rsp + 0x90]
1405aadbd: movaps xmm14, xmmword ptr [rsp + 0x80]
1405aadc6: movaps xmm15, xmmword ptr [rsp + 0x70]
1405aadcc: add rsp, 0x118
1405aadd3: pop rbx
1405aadd4: pop rdi
1405aadd5: pop rsi
1405aadd6: pop r12
1405aadd8: pop r13
1405aadda: pop r14
1405aaddc: pop r15
1405aadde: pop rbp
1405aaddf: ret 
1405aade0: mov qword ptr [rsp + 0x10], rdx
1405aade5: push rbp
1405aade6: push r15
1405aade8: push r14
1405aadea: push r13
1405aadec: push r12
1405aadee: push rsi
1405aadef: push rdi
1405aadf0: push rbx
1405aadf1: sub rsp, 0x118
1405aadf8: lea rbp, [rdx + 0x80]
1405aadff: movaps xmmword ptr [rsp + 0x70], xmm15
1405aae05: movaps xmmword ptr [rsp + 0x80], xmm14
1405aae0e: movaps xmmword ptr [rsp + 0x90], xmm13
1405aae17: movaps xmmword ptr [rsp + 0xa0], xmm12
1405aae20: movaps xmmword ptr [rsp + 0xb0], xmm11
1405aae29: movaps xmmword ptr [rsp + 0xc0], xmm10
1405aae32: movaps xmmword ptr [rsp + 0xd0], xmm9
1405aae3b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405aae44: movaps xmmword ptr [rsp + 0xf0], xmm7
1405aae4c: movaps xmmword ptr [rsp + 0x100], xmm6
1405aae54: and rdx, 0xffffffffffffff80
1405aae58: mov rbx, rdx
1405aae5b: mov byte ptr [rbx + 0xfe], 1
1405aae62: mov byte ptr [rbx + 0xff], 1
1405aae69: mov byte ptr [rbx + 0x100], 1
1405aae70: mov byte ptr [rbx + 0x101], 1
1405aae77: mov byte ptr [rbx + 0x102], 1
1405aae7e: mov byte ptr [rbx + 0x103], 1
1405aae85: mov byte ptr [rbx + 0x104], 1
1405aae8c: lea rcx, [rbx + 0xb00]
1405aae93: call 0x1405f18c0 ; core::ptr::drop_in_place<tiny_glade::event_loop::EguiEventIo>
1405aae98: mov byte ptr [rbx + 0xfe], 1
1405aae9f: mov byte ptr [rbx + 0xff], 1
1405aaea6: mov byte ptr [rbx + 0x100], 1
1405aaead: mov byte ptr [rbx + 0x101], 1
1405aaeb4: mov byte ptr [rbx + 0x102], 1
1405aaebb: mov byte ptr [rbx + 0x103], 1
1405aaec2: mov byte ptr [rbx + 0x104], 1
1405aaec9: movaps xmm6, xmmword ptr [rsp + 0x100]
1405aaed1: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405aaed9: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405aaee2: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405aaeeb: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405aaef4: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405aaefd: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405aaf06: movaps xmm13, xmmword ptr [rsp + 0x90]
1405aaf0f: movaps xmm14, xmmword ptr [rsp + 0x80]
1405aaf18: movaps xmm15, xmmword ptr [rsp + 0x70]
1405aaf1e: add rsp, 0x118
1405aaf25: pop rbx
1405aaf26: pop rdi
1405aaf27: pop rsi
1405aaf28: pop r12
1405aaf2a: pop r13
1405aaf2c: pop r14
1405aaf2e: pop r15
1405aaf30: pop rbp
1405aaf31: ret 
1405aaf32: nop word ptr cs:[rax + rax]
1405aaf40: mov qword ptr [rsp + 0x10], rdx
1405aaf45: push rbp
1405aaf46: push r15
1405aaf48: push r14
1405aaf4a: push r13
1405aaf4c: push r12
1405aaf4e: push rsi
1405aaf4f: push rdi
1405aaf50: push rbx
1405aaf51: sub rsp, 0x118
1405aaf58: lea rbp, [rdx + 0x80]
1405aaf5f: movaps xmmword ptr [rsp + 0x70], xmm15
1405aaf65: movaps xmmword ptr [rsp + 0x80], xmm14
1405aaf6e: movaps xmmword ptr [rsp + 0x90], xmm13
1405aaf77: movaps xmmword ptr [rsp + 0xa0], xmm12
1405aaf80: movaps xmmword ptr [rsp + 0xb0], xmm11
1405aaf89: movaps xmmword ptr [rsp + 0xc0], xmm10
1405aaf92: movaps xmmword ptr [rsp + 0xd0], xmm9
1405aaf9b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405aafa4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405aafac: movaps xmmword ptr [rsp + 0x100], xmm6
1405aafb4: and rdx, 0xffffffffffffff80
1405aafb8: mov rbx, rdx
1405aafbb: mov rdx, qword ptr [rbx + 0xb00]
1405aafc2: test rdx, rdx
1405aafc5: je 0x1405aafd9
1405aafc7: mov rcx, qword ptr [rbx + 0xb08]
1405aafce: mov r8d, 1
1405aafd4: call 0x140613c20 ; 
1405aafd9: mov byte ptr [rbx + 0xd3], 1
1405aafe0: movaps xmm6, xmmword ptr [rsp + 0x100]
1405aafe8: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405aaff0: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405aaff9: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ab002: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ab00b: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ab014: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ab01d: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ab026: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ab02f: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ab035: add rsp, 0x118
1405ab03c: pop rbx
1405ab03d: pop rdi
1405ab03e: pop rsi
1405ab03f: pop r12
1405ab041: pop r13
1405ab043: pop r14
1405ab045: pop r15
1405ab047: pop rbp
1405ab048: ret 
1405ab049: nop dword ptr [rax]
1405ab050: mov qword ptr [rsp + 0x10], rdx
1405ab055: push rbp
1405ab056: push r15
1405ab058: push r14
1405ab05a: push r13
1405ab05c: push r12
1405ab05e: push rsi
1405ab05f: push rdi
1405ab060: push rbx
1405ab061: sub rsp, 0x118
1405ab068: lea rbp, [rdx + 0x80]
1405ab06f: movaps xmmword ptr [rsp + 0x70], xmm15
1405ab075: movaps xmmword ptr [rsp + 0x80], xmm14
1405ab07e: movaps xmmword ptr [rsp + 0x90], xmm13
1405ab087: movaps xmmword ptr [rsp + 0xa0], xmm12
1405ab090: movaps xmmword ptr [rsp + 0xb0], xmm11
1405ab099: movaps xmmword ptr [rsp + 0xc0], xmm10
1405ab0a2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405ab0ab: movaps xmmword ptr [rsp + 0xe0], xmm8
1405ab0b4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405ab0bc: movaps xmmword ptr [rsp + 0x100], xmm6
1405ab0c4: and rdx, 0xffffffffffffff80
1405ab0c8: mov rbx, rdx
1405ab0cb: lea rcx, [rbx + 0x2bf0]
1405ab0d2: call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > > >
1405ab0d7: mov byte ptr [rbx + 0xaa], 1
1405ab0de: mov byte ptr [rbx + 0xab], 1
1405ab0e5: mov byte ptr [rbx + 0xac], 1
1405ab0ec: mov byte ptr [rbx + 0xad], 1
1405ab0f3: mov byte ptr [rbx + 0xae], 0
1405ab0fa: mov byte ptr [rbx + 0xaf], 1
1405ab101: mov byte ptr [rbx + 0xb0], 0
1405ab108: mov byte ptr [rbx + 0xb1], 1
1405ab10f: mov byte ptr [rbx + 0xb2], 1
1405ab116: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ab11e: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ab126: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ab12f: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ab138: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ab141: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ab14a: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ab153: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ab15c: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ab165: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ab16b: add rsp, 0x118
1405ab172: pop rbx
1405ab173: pop rdi
1405ab174: pop rsi
1405ab175: pop r12
1405ab177: pop r13
1405ab179: pop r14
1405ab17b: pop r15
1405ab17d: pop rbp
1405ab17e: ret 
1405ab17f: nop 
1405ab180: mov qword ptr [rsp + 0x10], rdx
1405ab185: push rbp
1405ab186: push r15
1405ab188: push r14
1405ab18a: push r13
1405ab18c: push r12
1405ab18e: push rsi
1405ab18f: push rdi
1405ab190: push rbx
1405ab191: sub rsp, 0x118
1405ab198: lea rbp, [rdx + 0x80]
1405ab19f: movaps xmmword ptr [rsp + 0x70], xmm15
1405ab1a5: movaps xmmword ptr [rsp + 0x80], xmm14
1405ab1ae: movaps xmmword ptr [rsp + 0x90], xmm13
1405ab1b7: movaps xmmword ptr [rsp + 0xa0], xmm12
1405ab1c0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405ab1c9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405ab1d2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405ab1db: movaps xmmword ptr [rsp + 0xe0], xmm8
1405ab1e4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405ab1ec: movaps xmmword ptr [rsp + 0x100], xmm6
1405ab1f4: and rdx, 0xffffffffffffff80
1405ab1f8: mov rbx, rdx
1405ab1fb: lea rcx, [rbx + 0x2230]
1405ab202: call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > > >
1405ab207: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ab20f: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ab217: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ab220: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ab229: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ab232: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ab23b: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ab244: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ab24d: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ab256: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ab25c: add rsp, 0x118
1405ab263: pop rbx
1405ab264: pop rdi
1405ab265: pop rsi
1405ab266: pop r12
1405ab268: pop r13
1405ab26a: pop r14
1405ab26c: pop r15
1405ab26e: pop rbp
1405ab26f: ret 
1405ab270: mov qword ptr [rsp + 0x10], rdx
1405ab275: push rbp
1405ab276: push r15
1405ab278: push r14
1405ab27a: push r13
1405ab27c: push r12
1405ab27e: push rsi
1405ab27f: push rdi
1405ab280: push rbx
1405ab281: sub rsp, 0x118
1405ab288: lea rbp, [rdx + 0x80]
1405ab28f: movaps xmmword ptr [rsp + 0x70], xmm15
1405ab295: movaps xmmword ptr [rsp + 0x80], xmm14
1405ab29e: movaps xmmword ptr [rsp + 0x90], xmm13
1405ab2a7: movaps xmmword ptr [rsp + 0xa0], xmm12
1405ab2b0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405ab2b9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405ab2c2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405ab2cb: movaps xmmword ptr [rsp + 0xe0], xmm8
1405ab2d4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405ab2dc: movaps xmmword ptr [rsp + 0x100], xmm6
1405ab2e4: and rdx, 0xffffffffffffff80
1405ab2e8: mov rbx, rdx
1405ab2eb: cmp byte ptr [rbx + 0x110], 0
1405ab2f2: je 0x1405ab33f
1405ab2f4: mov byte ptr [rbx + 0xaa], 1
1405ab2fb: mov byte ptr [rbx + 0xab], 1
1405ab302: mov byte ptr [rbx + 0xac], 1
1405ab309: mov byte ptr [rbx + 0xad], 1
1405ab310: mov byte ptr [rbx + 0xae], 1
1405ab317: mov byte ptr [rbx + 0xaf], 1
1405ab31e: mov byte ptr [rbx + 0xb0], 1
1405ab325: mov byte ptr [rbx + 0xb1], 1
1405ab32c: mov byte ptr [rbx + 0xb2], 1
1405ab333: lea rcx, [rbx + 0x440]
1405ab33a: call 0x1405f79e0 ; core::ptr::drop_in_place<bevy_app::task_pool_plugin::TaskPoolPlugin>
1405ab33f: mov byte ptr [rbx + 0xaa], 1
1405ab346: mov byte ptr [rbx + 0xab], 1
1405ab34d: mov byte ptr [rbx + 0xac], 1
1405ab354: mov byte ptr [rbx + 0xad], 1
1405ab35b: mov byte ptr [rbx + 0xae], 1
1405ab362: mov byte ptr [rbx + 0xaf], 1
1405ab369: mov byte ptr [rbx + 0xb0], 1
1405ab370: mov byte ptr [rbx + 0xb1], 1
1405ab377: mov byte ptr [rbx + 0xb2], 1
1405ab37e: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ab386: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ab38e: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ab397: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ab3a0: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ab3a9: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ab3b2: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ab3bb: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ab3c4: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ab3cd: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ab3d3: add rsp, 0x118
1405ab3da: pop rbx
1405ab3db: pop rdi
1405ab3dc: pop rsi
1405ab3dd: pop r12
1405ab3df: pop r13
1405ab3e1: pop r14
1405ab3e3: pop r15
1405ab3e5: pop rbp
1405ab3e6: ret 
1405ab3e7: nop word ptr [rax + rax]
1405ab3f0: mov qword ptr [rsp + 0x10], rdx
1405ab3f5: push rbp
1405ab3f6: push r15
1405ab3f8: push r14
1405ab3fa: push r13
1405ab3fc: push r12
1405ab3fe: push rsi
1405ab3ff: push rdi
1405ab400: push rbx
1405ab401: sub rsp, 0x118
1405ab408: lea rbp, [rdx + 0x80]
1405ab40f: movaps xmmword ptr [rsp + 0x70], xmm15
1405ab415: movaps xmmword ptr [rsp + 0x80], xmm14
1405ab41e: movaps xmmword ptr [rsp + 0x90], xmm13
1405ab427: movaps xmmword ptr [rsp + 0xa0], xmm12
1405ab430: movaps xmmword ptr [rsp + 0xb0], xmm11
1405ab439: movaps xmmword ptr [rsp + 0xc0], xmm10
1405ab442: movaps xmmword ptr [rsp + 0xd0], xmm9
1405ab44b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405ab454: movaps xmmword ptr [rsp + 0xf0], xmm7
1405ab45c: movaps xmmword ptr [rsp + 0x100], xmm6
1405ab464: and rdx, 0xffffffffffffff80
1405ab468: mov rbx, rdx
1405ab46b: movzx esi, byte ptr [rbx + 0xf7]
1405ab472: movzx edi, byte ptr [rbx + 0xf6]
1405ab479: movzx r14d, byte ptr [rbx + 0xf5]
1405ab481: test r14b, 1
1405ab485: je 0x1405ab515
1405ab48b: lea rcx, [rbx + 0x2bf0]
1405ab492: call 0x1405e8b60 ; core::ptr::drop_in_place<bevy_app::sub_app::SubApp>
1405ab497: mov byte ptr [rbx + 0x112], r14b
1405ab49e: mov byte ptr [rbx + 0x113], dil
1405ab4a5: mov byte ptr [rbx + 0x114], sil
1405ab4ac: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ab4b4: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ab4bc: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ab4c5: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ab4ce: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ab4d7: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ab4e0: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ab4e9: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ab4f2: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ab4fb: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ab501: add rsp, 0x118
1405ab508: pop rbx
1405ab509: pop rdi
1405ab50a: pop rsi
1405ab50b: pop r12
1405ab50d: pop r13
1405ab50f: pop r14
1405ab511: pop r15
1405ab513: pop rbp
1405ab514: ret 
1405ab515: mov byte ptr [rbx + 0x112], r14b
1405ab51c: mov byte ptr [rbx + 0x113], dil
1405ab523: mov byte ptr [rbx + 0x114], sil
1405ab52a: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ab532: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ab53a: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ab543: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ab54c: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ab555: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ab55e: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ab567: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ab570: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ab579: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ab57f: add rsp, 0x118
1405ab586: pop rbx
1405ab587: pop rdi
1405ab588: pop rsi
1405ab589: pop r12
1405ab58b: pop r13
1405ab58d: pop r14
1405ab58f: pop r15
1405ab591: pop rbp
1405ab592: ret 
1405ab593: nop word ptr cs:[rax + rax]
1405ab5a0: mov qword ptr [rsp + 0x10], rdx
1405ab5a5: push rbp
1405ab5a6: push r15
1405ab5a8: push r14
1405ab5aa: push r13
1405ab5ac: push r12
1405ab5ae: push rsi
1405ab5af: push rdi
1405ab5b0: push rbx
1405ab5b1: sub rsp, 0x118
1405ab5b8: lea rbp, [rdx + 0x80]
1405ab5bf: movaps xmmword ptr [rsp + 0x70], xmm15
1405ab5c5: movaps xmmword ptr [rsp + 0x80], xmm14
1405ab5ce: movaps xmmword ptr [rsp + 0x90], xmm13
1405ab5d7: movaps xmmword ptr [rsp + 0xa0], xmm12
1405ab5e0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405ab5e9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405ab5f2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405ab5fb: movaps xmmword ptr [rsp + 0xe0], xmm8
1405ab604: movaps xmmword ptr [rsp + 0xf0], xmm7
1405ab60c: movaps xmmword ptr [rsp + 0x100], xmm6
1405ab614: and rdx, 0xffffffffffffff80
1405ab618: mov rbx, rdx
1405ab61b: movzx esi, byte ptr [rbx + 0x114]
1405ab622: movzx r14d, byte ptr [rbx + 0x113]
1405ab62a: movzx edi, byte ptr [rbx + 0x112]
1405ab631: test r14b, 1
1405ab635: je 0x1405ab6d3
1405ab63b: lea rcx, [rbx + 0x2590]
1405ab642: call 0x1405e6cd0 ; core::ptr::drop_in_place<bevy_app::app::App>
1405ab647: mov byte ptr [rbx + 0xc2], r14b
1405ab64e: mov byte ptr [rbx + 0xc3], 1
1405ab655: mov byte ptr [rbx + 0xc4], r14b
1405ab65c: mov byte ptr [rbx + 0xc5], dil
1405ab663: mov byte ptr [rbx + 0xc6], sil
1405ab66a: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ab672: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ab67a: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ab683: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ab68c: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ab695: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ab69e: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ab6a7: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ab6b0: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ab6b9: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ab6bf: add rsp, 0x118
1405ab6c6: pop rbx
1405ab6c7: pop rdi
1405ab6c8: pop rsi
1405ab6c9: pop r12
1405ab6cb: pop r13
1405ab6cd: pop r14
1405ab6cf: pop r15
1405ab6d1: pop rbp
1405ab6d2: ret 
1405ab6d3: mov byte ptr [rbx + 0xc2], r14b
1405ab6da: mov byte ptr [rbx + 0xc3], 1
1405ab6e1: mov byte ptr [rbx + 0xc4], r14b
1405ab6e8: mov byte ptr [rbx + 0xc5], dil
1405ab6ef: mov byte ptr [rbx + 0xc6], sil
1405ab6f6: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ab6fe: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ab706: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ab70f: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ab718: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ab721: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ab72a: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ab733: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ab73c: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ab745: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ab74b: add rsp, 0x118
1405ab752: pop rbx
1405ab753: pop rdi
1405ab754: pop rsi
1405ab755: pop r12
1405ab757: pop r13
1405ab759: pop r14
1405ab75b: pop r15
1405ab75d: pop rbp
1405ab75e: ret 
1405ab75f: nop 
1405ab760: mov qword ptr [rsp + 0x10], rdx
1405ab765: push rbp
1405ab766: push r15
1405ab768: push r14
1405ab76a: push r13
1405ab76c: push r12
1405ab76e: push rsi
1405ab76f: push rdi
1405ab770: push rbx
1405ab771: sub rsp, 0x118
1405ab778: lea rbp, [rdx + 0x80]
1405ab77f: movaps xmmword ptr [rsp + 0x70], xmm15
1405ab785: movaps xmmword ptr [rsp + 0x80], xmm14
1405ab78e: movaps xmmword ptr [rsp + 0x90], xmm13
1405ab797: movaps xmmword ptr [rsp + 0xa0], xmm12
1405ab7a0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405ab7a9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405ab7b2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405ab7bb: movaps xmmword ptr [rsp + 0xe0], xmm8
1405ab7c4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405ab7cc: movaps xmmword ptr [rsp + 0x100], xmm6
1405ab7d4: and rdx, 0xffffffffffffff80
1405ab7d8: mov rbx, rdx
1405ab7db: lea rcx, [rbx + 0x1ff0]
1405ab7e2: call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > > >
1405ab7e7: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ab7ef: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ab7f7: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ab800: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ab809: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ab812: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ab81b: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ab824: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ab82d: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ab836: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ab83c: add rsp, 0x118
1405ab843: pop rbx
1405ab844: pop rdi
1405ab845: pop rsi
1405ab846: pop r12
1405ab848: pop r13
1405ab84a: pop r14
1405ab84c: pop r15
1405ab84e: pop rbp
1405ab84f: ret 
1405ab850: mov qword ptr [rsp + 0x10], rdx
1405ab855: push rbp
1405ab856: push r15
1405ab858: push r14
1405ab85a: push r13
1405ab85c: push r12
1405ab85e: push rsi
1405ab85f: push rdi
1405ab860: push rbx
1405ab861: sub rsp, 0x118
1405ab868: lea rbp, [rdx + 0x80]
1405ab86f: movaps xmmword ptr [rsp + 0x70], xmm15
1405ab875: movaps xmmword ptr [rsp + 0x80], xmm14
1405ab87e: movaps xmmword ptr [rsp + 0x90], xmm13
1405ab887: movaps xmmword ptr [rsp + 0xa0], xmm12
1405ab890: movaps xmmword ptr [rsp + 0xb0], xmm11
1405ab899: movaps xmmword ptr [rsp + 0xc0], xmm10
1405ab8a2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405ab8ab: movaps xmmword ptr [rsp + 0xe0], xmm8
1405ab8b4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405ab8bc: movaps xmmword ptr [rsp + 0x100], xmm6
1405ab8c4: and rdx, 0xffffffffffffff80
1405ab8c8: mov rbx, rdx
1405ab8cb: movzx esi, byte ptr [rbx + 0xc7]
1405ab8d2: mov rdx, qword ptr [rbx + 0x2230]
1405ab8d9: test rdx, rdx
1405ab8dc: je 0x1405ab8f0
1405ab8de: mov rcx, qword ptr [rbx + 0x2238]
1405ab8e5: mov r8d, 1
1405ab8eb: call 0x140613c20 ; 
1405ab8f0: mov byte ptr [rbx + 0xd3], sil
1405ab8f7: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ab8ff: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ab907: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ab910: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ab919: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ab922: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ab92b: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ab934: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ab93d: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ab946: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ab94c: add rsp, 0x118
1405ab953: pop rbx
1405ab954: pop rdi
1405ab955: pop rsi
1405ab956: pop r12
1405ab958: pop r13
1405ab95a: pop r14
1405ab95c: pop r15
1405ab95e: pop rbp
1405ab95f: ret 
1405ab960: mov qword ptr [rsp + 0x10], rdx
1405ab965: push rbp
1405ab966: push r15
1405ab968: push r14
1405ab96a: push r13
1405ab96c: push r12
1405ab96e: push rsi
1405ab96f: push rdi
1405ab970: push rbx
1405ab971: sub rsp, 0x118
1405ab978: lea rbp, [rdx + 0x80]
1405ab97f: movaps xmmword ptr [rsp + 0x70], xmm15
1405ab985: movaps xmmword ptr [rsp + 0x80], xmm14
1405ab98e: movaps xmmword ptr [rsp + 0x90], xmm13
1405ab997: movaps xmmword ptr [rsp + 0xa0], xmm12
1405ab9a0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405ab9a9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405ab9b2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405ab9bb: movaps xmmword ptr [rsp + 0xe0], xmm8
1405ab9c4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405ab9cc: movaps xmmword ptr [rsp + 0x100], xmm6
1405ab9d4: and rdx, 0xffffffffffffff80
1405ab9d8: mov rbx, rdx
1405ab9db: lea rcx, [rbx + 0x370]
1405ab9e2: call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > > >
1405ab9e7: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ab9ef: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ab9f7: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405aba00: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405aba09: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405aba12: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405aba1b: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405aba24: movaps xmm13, xmmword ptr [rsp + 0x90]
1405aba2d: movaps xmm14, xmmword ptr [rsp + 0x80]
1405aba36: movaps xmm15, xmmword ptr [rsp + 0x70]
1405aba3c: add rsp, 0x118
1405aba43: pop rbx
1405aba44: pop rdi
1405aba45: pop rsi
1405aba46: pop r12
1405aba48: pop r13
1405aba4a: pop r14
1405aba4c: pop r15
1405aba4e: pop rbp
1405aba4f: ret 
1405aba50: mov qword ptr [rsp + 0x10], rdx
1405aba55: push rbp
1405aba56: push r15
1405aba58: push r14
1405aba5a: push r13
1405aba5c: push r12
1405aba5e: push rsi
1405aba5f: push rdi
1405aba60: push rbx
1405aba61: sub rsp, 0x118
1405aba68: lea rbp, [rdx + 0x80]
1405aba6f: movaps xmmword ptr [rsp + 0x70], xmm15
1405aba75: movaps xmmword ptr [rsp + 0x80], xmm14
1405aba7e: movaps xmmword ptr [rsp + 0x90], xmm13
1405aba87: movaps xmmword ptr [rsp + 0xa0], xmm12
1405aba90: movaps xmmword ptr [rsp + 0xb0], xmm11
1405aba99: movaps xmmword ptr [rsp + 0xc0], xmm10
1405abaa2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405abaab: movaps xmmword ptr [rsp + 0xe0], xmm8
1405abab4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405ababc: movaps xmmword ptr [rsp + 0x100], xmm6
1405abac4: and rdx, 0xffffffffffffff80
1405abac8: mov rbx, rdx
1405abacb: lea rcx, [rbx + 0x24e8]
1405abad2: call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> > > >
1405abad7: mov byte ptr [rbx + 0xaa], 1
1405abade: mov byte ptr [rbx + 0xab], 1
1405abae5: mov byte ptr [rbx + 0xac], 1
1405abaec: mov byte ptr [rbx + 0xad], 1
1405abaf3: mov byte ptr [rbx + 0xae], 0
1405abafa: mov byte ptr [rbx + 0xaf], 1
1405abb01: mov byte ptr [rbx + 0xb0], 0
1405abb08: mov byte ptr [rbx + 0xb1], 1
1405abb0f: mov byte ptr [rbx + 0xb2], 1
1405abb16: movaps xmm6, xmmword ptr [rsp + 0x100]
1405abb1e: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405abb26: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405abb2f: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405abb38: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405abb41: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405abb4a: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405abb53: movaps xmm13, xmmword ptr [rsp + 0x90]
1405abb5c: movaps xmm14, xmmword ptr [rsp + 0x80]
1405abb65: movaps xmm15, xmmword ptr [rsp + 0x70]
1405abb6b: add rsp, 0x118
1405abb72: pop rbx
1405abb73: pop rdi
1405abb74: pop rsi
1405abb75: pop r12
1405abb77: pop r13
1405abb79: pop r14
1405abb7b: pop r15
1405abb7d: pop rbp
1405abb7e: ret 
1405abb7f: nop 
1405abb80: mov qword ptr [rsp + 0x10], rdx
1405abb85: push rbp
1405abb86: push r15
1405abb88: push r14
1405abb8a: push r13
1405abb8c: push r12
1405abb8e: push rsi
1405abb8f: push rdi
1405abb90: push rbx
1405abb91: sub rsp, 0x118
1405abb98: lea rbp, [rdx + 0x80]
1405abb9f: movaps xmmword ptr [rsp + 0x70], xmm15
1405abba5: movaps xmmword ptr [rsp + 0x80], xmm14
1405abbae: movaps xmmword ptr [rsp + 0x90], xmm13
1405abbb7: movaps xmmword ptr [rsp + 0xa0], xmm12
1405abbc0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405abbc9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405abbd2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405abbdb: movaps xmmword ptr [rsp + 0xe0], xmm8
1405abbe4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405abbec: movaps xmmword ptr [rsp + 0x100], xmm6
1405abbf4: and rdx, 0xffffffffffffff80
1405abbf8: mov rbx, rdx
1405abbfb: movzx esi, byte ptr [rbx + 0xd3]
1405abc02: mov rdx, qword ptr [rbx + 0x1ff0]
1405abc09: test rdx, rdx
1405abc0c: je 0x1405abc20
1405abc0e: mov rcx, qword ptr [rbx + 0x1ff8]
1405abc15: mov r8d, 1
1405abc1b: call 0x140613c20 ; 
1405abc20: mov byte ptr [rbx + 0xb4], 1
1405abc27: mov byte ptr [rbx + 0xb5], sil
1405abc2e: mov byte ptr [rbx + 0xb6], 1
1405abc35: mov byte ptr [rbx + 0xb7], 0
1405abc3c: mov byte ptr [rbx + 0xb8], 1
1405abc43: mov byte ptr [rbx + 0xb9], 1
1405abc4a: mov byte ptr [rbx + 0xba], 1
1405abc51: mov byte ptr [rbx + 0xbb], 1
1405abc58: movaps xmm6, xmmword ptr [rsp + 0x100]
1405abc60: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405abc68: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405abc71: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405abc7a: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405abc83: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405abc8c: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405abc95: movaps xmm13, xmmword ptr [rsp + 0x90]
1405abc9e: movaps xmm14, xmmword ptr [rsp + 0x80]
1405abca7: movaps xmm15, xmmword ptr [rsp + 0x70]
1405abcad: add rsp, 0x118
1405abcb4: pop rbx
1405abcb5: pop rdi
1405abcb6: pop rsi
1405abcb7: pop r12
1405abcb9: pop r13
1405abcbb: pop r14
1405abcbd: pop r15
1405abcbf: pop rbp
1405abcc0: ret 
1405abcc1: nop word ptr cs:[rax + rax]
1405abcd0: mov qword ptr [rsp + 0x10], rdx
1405abcd5: push rbp
1405abcd6: push r15
1405abcd8: push r14
1405abcda: push r13
1405abcdc: push r12
1405abcde: push rsi
1405abcdf: push rdi
1405abce0: push rbx
1405abce1: sub rsp, 0x118
1405abce8: lea rbp, [rdx + 0x80]
1405abcef: movaps xmmword ptr [rsp + 0x70], xmm15
1405abcf5: movaps xmmword ptr [rsp + 0x80], xmm14
1405abcfe: movaps xmmword ptr [rsp + 0x90], xmm13
1405abd07: movaps xmmword ptr [rsp + 0xa0], xmm12
1405abd10: movaps xmmword ptr [rsp + 0xb0], xmm11
1405abd19: movaps xmmword ptr [rsp + 0xc0], xmm10
1405abd22: movaps xmmword ptr [rsp + 0xd0], xmm9
1405abd2b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405abd34: movaps xmmword ptr [rsp + 0xf0], xmm7
1405abd3c: movaps xmmword ptr [rsp + 0x100], xmm6
1405abd44: and rdx, 0xffffffffffffff80
1405abd48: mov rbx, rdx
1405abd4b: movzx esi, byte ptr [rbx + 0xe3]
1405abd52: mov rax, qword ptr [rbx + 0x370]
1405abd59: test rax, rax
1405abd5c: je 0x1405abd70
1405abd5e: lock dec qword ptr [rax]
1405abd62: jne 0x1405abd70
1405abd64: lea rcx, [rbx + 0x370]
1405abd6b: call 0x1402e5d80 ; alloc::sync::Arc::drop_slow<core::sync::atomic::AtomicBool,alloc::alloc::Global>
1405abd70: and sil, 1
1405abd74: mov byte ptr [rbx + 0xd9], sil
1405abd7b: mov byte ptr [rbx + 0xda], 1
1405abd82: movaps xmm6, xmmword ptr [rsp + 0x100]
1405abd8a: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405abd92: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405abd9b: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405abda4: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405abdad: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405abdb6: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405abdbf: movaps xmm13, xmmword ptr [rsp + 0x90]
1405abdc8: movaps xmm14, xmmword ptr [rsp + 0x80]
1405abdd1: movaps xmm15, xmmword ptr [rsp + 0x70]
1405abdd7: add rsp, 0x118
1405abdde: pop rbx
1405abddf: pop rdi
1405abde0: pop rsi
1405abde1: pop r12
1405abde3: pop r13
1405abde5: pop r14
1405abde7: pop r15
1405abde9: pop rbp
1405abdea: ret 
1405abdeb: nop dword ptr [rax + rax]
1405abdf0: mov qword ptr [rsp + 0x10], rdx
1405abdf5: push rbp
1405abdf6: push r15
1405abdf8: push r14
1405abdfa: push r13
1405abdfc: push r12
1405abdfe: push rsi
1405abdff: push rdi
1405abe00: push rbx
1405abe01: sub rsp, 0x118
1405abe08: lea rbp, [rdx + 0x80]
1405abe0f: movaps xmmword ptr [rsp + 0x70], xmm15
1405abe15: movaps xmmword ptr [rsp + 0x80], xmm14
1405abe1e: movaps xmmword ptr [rsp + 0x90], xmm13
1405abe27: movaps xmmword ptr [rsp + 0xa0], xmm12
1405abe30: movaps xmmword ptr [rsp + 0xb0], xmm11
1405abe39: movaps xmmword ptr [rsp + 0xc0], xmm10
1405abe42: movaps xmmword ptr [rsp + 0xd0], xmm9
1405abe4b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405abe54: movaps xmmword ptr [rsp + 0xf0], xmm7
1405abe5c: movaps xmmword ptr [rsp + 0x100], xmm6
1405abe64: and rdx, 0xffffffffffffff80
1405abe68: mov rbx, rdx
1405abe6b: movzx esi, byte ptr [rbx + 0xd9]
1405abe72: cmp byte ptr [rbx + 0xda], 0
1405abe79: je 0x1405abf0e
1405abe7f: xor eax, eax
1405abe81: cmp rax, qword ptr [rbx + 0x2590]
1405abe88: jo 0x1405abf82
1405abe8e: lea rcx, [rbx + 0x2590]
1405abe95: call 0x140592cc0 ; core::ptr::drop_in_place<enum2$<country_core::meta_commander::MetaCommand> >
1405abe9a: and sil, 1
1405abe9e: mov byte ptr [rbx + 0xf4], sil
1405abea5: movaps xmm6, xmmword ptr [rsp + 0x100]
1405abead: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405abeb5: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405abebe: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405abec7: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405abed0: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405abed9: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405abee2: movaps xmm13, xmmword ptr [rsp + 0x90]
1405abeeb: movaps xmm14, xmmword ptr [rsp + 0x80]
1405abef4: movaps xmm15, xmmword ptr [rsp + 0x70]
1405abefa: add rsp, 0x118
1405abf01: pop rbx
1405abf02: pop rdi
1405abf03: pop rsi
1405abf04: pop r12
1405abf06: pop r13
1405abf08: pop r14
1405abf0a: pop r15
1405abf0c: pop rbp
1405abf0d: ret 
1405abf0e: and sil, 1
1405abf12: mov byte ptr [rbx + 0xf4], sil
1405abf19: movaps xmm6, xmmword ptr [rsp + 0x100]
1405abf21: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405abf29: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405abf32: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405abf3b: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405abf44: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405abf4d: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405abf56: movaps xmm13, xmmword ptr [rsp + 0x90]
1405abf5f: movaps xmm14, xmmword ptr [rsp + 0x80]
1405abf68: movaps xmm15, xmmword ptr [rsp + 0x70]
1405abf6e: add rsp, 0x118
1405abf75: pop rbx
1405abf76: pop rdi
1405abf77: pop rsi
1405abf78: pop r12
1405abf7a: pop r13
1405abf7c: pop r14
1405abf7e: pop r15
1405abf80: pop rbp
1405abf81: ret 
1405abf82: and sil, 1
1405abf86: mov byte ptr [rbx + 0xf4], sil
1405abf8d: movaps xmm6, xmmword ptr [rsp + 0x100]
1405abf95: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405abf9d: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405abfa6: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405abfaf: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405abfb8: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405abfc1: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405abfca: movaps xmm13, xmmword ptr [rsp + 0x90]
1405abfd3: movaps xmm14, xmmword ptr [rsp + 0x80]
1405abfdc: movaps xmm15, xmmword ptr [rsp + 0x70]
1405abfe2: add rsp, 0x118
1405abfe9: pop rbx
1405abfea: pop rdi
1405abfeb: pop rsi
1405abfec: pop r12
1405abfee: pop r13
1405abff0: pop r14
1405abff2: pop r15
1405abff4: pop rbp
1405abff5: ret 
1405abff6: nop word ptr cs:[rax + rax]
1405ac000: mov qword ptr [rsp + 0x10], rdx
1405ac005: push rbp
1405ac006: push r15
1405ac008: push r14
1405ac00a: push r13
1405ac00c: push r12
1405ac00e: push rsi
1405ac00f: push rdi
1405ac010: push rbx
1405ac011: sub rsp, 0x118
1405ac018: lea rbp, [rdx + 0x80]
1405ac01f: movaps xmmword ptr [rsp + 0x70], xmm15
1405ac025: movaps xmmword ptr [rsp + 0x80], xmm14
1405ac02e: movaps xmmword ptr [rsp + 0x90], xmm13
1405ac037: movaps xmmword ptr [rsp + 0xa0], xmm12
1405ac040: movaps xmmword ptr [rsp + 0xb0], xmm11
1405ac049: movaps xmmword ptr [rsp + 0xc0], xmm10
1405ac052: movaps xmmword ptr [rsp + 0xd0], xmm9
1405ac05b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405ac064: movaps xmmword ptr [rsp + 0xf0], xmm7
1405ac06c: movaps xmmword ptr [rsp + 0x100], xmm6
1405ac074: and rdx, 0xffffffffffffff80
1405ac078: mov rbx, rdx
1405ac07b: cmp byte ptr [rbx + 0xf4], 0
1405ac082: je 0x1405ac135
1405ac088: lea rcx, [rbx + 0x2230]
1405ac08f: call 0x1405d2cf0 ; core::ptr::drop_in_place<alloc::collections::vec_deque::VecDeque<enum2$<country_core::meta_commander::MetaCommand>,alloc::alloc::Global> >
1405ac094: mov byte ptr [rbx + 0xb4], 1
1405ac09b: mov byte ptr [rbx + 0xb5], 1
1405ac0a2: mov byte ptr [rbx + 0xb6], 1
1405ac0a9: mov byte ptr [rbx + 0xb7], 0
1405ac0b0: mov byte ptr [rbx + 0xb8], 1
1405ac0b7: mov byte ptr [rbx + 0xb9], 1
1405ac0be: mov byte ptr [rbx + 0xba], 1
1405ac0c5: mov byte ptr [rbx + 0xbb], 1
1405ac0cc: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ac0d4: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ac0dc: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ac0e5: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ac0ee: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ac0f7: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ac100: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ac109: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ac112: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ac11b: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ac121: add rsp, 0x118
1405ac128: pop rbx
1405ac129: pop rdi
1405ac12a: pop rsi
1405ac12b: pop r12
1405ac12d: pop r13
1405ac12f: pop r14
1405ac131: pop r15
1405ac133: pop rbp
1405ac134: ret 
1405ac135: mov byte ptr [rbx + 0xb4], 1
1405ac13c: mov byte ptr [rbx + 0xb5], 1
1405ac143: mov byte ptr [rbx + 0xb6], 1
1405ac14a: mov byte ptr [rbx + 0xb7], 0
1405ac151: mov byte ptr [rbx + 0xb8], 1
1405ac158: mov byte ptr [rbx + 0xb9], 1
1405ac15f: mov byte ptr [rbx + 0xba], 1
1405ac166: mov byte ptr [rbx + 0xbb], 1
1405ac16d: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ac175: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ac17d: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ac186: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ac18f: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ac198: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ac1a1: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ac1aa: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ac1b3: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ac1bc: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ac1c2: add rsp, 0x118
1405ac1c9: pop rbx
1405ac1ca: pop rdi
1405ac1cb: pop rsi
1405ac1cc: pop r12
1405ac1ce: pop r13
1405ac1d0: pop r14
1405ac1d2: pop r15
1405ac1d4: pop rbp
1405ac1d5: ret 
1405ac1d6: nop word ptr cs:[rax + rax]
1405ac1e0: mov qword ptr [rsp + 0x10], rdx
1405ac1e5: push rbp
1405ac1e6: push r15
1405ac1e8: push r14
1405ac1ea: push r13
1405ac1ec: push r12
1405ac1ee: push rsi
1405ac1ef: push rdi
1405ac1f0: push rbx
1405ac1f1: sub rsp, 0x118
1405ac1f8: lea rbp, [rdx + 0x80]
1405ac1ff: movaps xmmword ptr [rsp + 0x70], xmm15
1405ac205: movaps xmmword ptr [rsp + 0x80], xmm14
1405ac20e: movaps xmmword ptr [rsp + 0x90], xmm13
1405ac217: movaps xmmword ptr [rsp + 0xa0], xmm12
1405ac220: movaps xmmword ptr [rsp + 0xb0], xmm11
1405ac229: movaps xmmword ptr [rsp + 0xc0], xmm10
1405ac232: movaps xmmword ptr [rsp + 0xd0], xmm9
1405ac23b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405ac244: movaps xmmword ptr [rsp + 0xf0], xmm7
1405ac24c: movaps xmmword ptr [rsp + 0x100], xmm6
1405ac254: and rdx, 0xffffffffffffff80
1405ac258: mov rbx, rdx
1405ac25b: movzx esi, byte ptr [rbx + 0xb3]
1405ac262: lea rcx, [rbx + 0x440]
1405ac269: call 0x1405e6cd0 ; core::ptr::drop_in_place<bevy_app::app::App>
1405ac26e: mov byte ptr [rbx + 0x11f], sil
1405ac275: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ac27d: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ac285: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ac28e: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ac297: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ac2a0: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ac2a9: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ac2b2: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ac2bb: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ac2c4: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ac2ca: add rsp, 0x118
1405ac2d1: pop rbx
1405ac2d2: pop rdi
1405ac2d3: pop rsi
1405ac2d4: pop r12
1405ac2d6: pop r13
1405ac2d8: pop r14
1405ac2da: pop r15
1405ac2dc: pop rbp
1405ac2dd: ret 
1405ac2de: nop 
1405ac2e0: mov qword ptr [rsp + 0x10], rdx
1405ac2e5: push rbp
1405ac2e6: push r15
1405ac2e8: push r14
1405ac2ea: push r13
1405ac2ec: push r12
1405ac2ee: push rsi
1405ac2ef: push rdi
1405ac2f0: push rbx
1405ac2f1: sub rsp, 0x118
1405ac2f8: lea rbp, [rdx + 0x80]
1405ac2ff: movaps xmmword ptr [rsp + 0x70], xmm15
1405ac305: movaps xmmword ptr [rsp + 0x80], xmm14
1405ac30e: movaps xmmword ptr [rsp + 0x90], xmm13
1405ac317: movaps xmmword ptr [rsp + 0xa0], xmm12
1405ac320: movaps xmmword ptr [rsp + 0xb0], xmm11
1405ac329: movaps xmmword ptr [rsp + 0xc0], xmm10
1405ac332: movaps xmmword ptr [rsp + 0xd0], xmm9
1405ac33b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405ac344: movaps xmmword ptr [rsp + 0xf0], xmm7
1405ac34c: movaps xmmword ptr [rsp + 0x100], xmm6
1405ac354: and rdx, 0xffffffffffffff80
1405ac358: mov rbx, rdx
1405ac35b: cmp byte ptr [rbx + 0x11f], 0
1405ac362: je 0x1405ac3a0
1405ac364: mov rax, qword ptr [rbx + 0x2230]
1405ac36b: lock dec qword ptr [rax]
1405ac36f: jne 0x1405ac3a0
1405ac371: mov byte ptr [rbx + 0xc2], 1
1405ac378: mov byte ptr [rbx + 0xc3], 1
1405ac37f: mov byte ptr [rbx + 0xc4], 1
1405ac386: mov byte ptr [rbx + 0xc5], 1
1405ac38d: mov byte ptr [rbx + 0xc6], 1
1405ac394: lea rcx, [rbx + 0x2230]
1405ac39b: call 0x140b5be90 ; alloc::sync::Arc::drop_slow<rhapsody::raw::device::GraphicsDevice,alloc::alloc::Global>
1405ac3a0: mov byte ptr [rbx + 0xc2], 1
1405ac3a7: mov byte ptr [rbx + 0xc3], 1
1405ac3ae: mov byte ptr [rbx + 0xc4], 1
1405ac3b5: mov byte ptr [rbx + 0xc5], 1
1405ac3bc: mov byte ptr [rbx + 0xc6], 1
1405ac3c3: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ac3cb: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ac3d3: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ac3dc: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ac3e5: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ac3ee: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ac3f7: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ac400: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ac409: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ac412: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ac418: add rsp, 0x118
1405ac41f: pop rbx
1405ac420: pop rdi
1405ac421: pop rsi
1405ac422: pop r12
1405ac424: pop r13
1405ac426: pop r14
1405ac428: pop r15
1405ac42a: pop rbp
1405ac42b: ret 
1405ac42c: nop dword ptr [rax]
1405ac430: mov qword ptr [rsp + 0x10], rdx
1405ac435: push rbp
1405ac436: push r15
1405ac438: push r14
1405ac43a: push r13
1405ac43c: push r12
1405ac43e: push rsi
1405ac43f: push rdi
1405ac440: push rbx
1405ac441: sub rsp, 0x118
1405ac448: lea rbp, [rdx + 0x80]
1405ac44f: movaps xmmword ptr [rsp + 0x70], xmm15
1405ac455: movaps xmmword ptr [rsp + 0x80], xmm14
1405ac45e: movaps xmmword ptr [rsp + 0x90], xmm13
1405ac467: movaps xmmword ptr [rsp + 0xa0], xmm12
1405ac470: movaps xmmword ptr [rsp + 0xb0], xmm11
1405ac479: movaps xmmword ptr [rsp + 0xc0], xmm10
1405ac482: movaps xmmword ptr [rsp + 0xd0], xmm9
1405ac48b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405ac494: movaps xmmword ptr [rsp + 0xf0], xmm7
1405ac49c: movaps xmmword ptr [rsp + 0x100], xmm6
1405ac4a4: and rdx, 0xffffffffffffff80
1405ac4a8: mov rbx, rdx
1405ac4ab: movzx esi, byte ptr [rbx + 0xc6]
1405ac4b2: movzx r12d, byte ptr [rbx + 0xc5]
1405ac4ba: movzx edi, byte ptr [rbx + 0xc4]
1405ac4c1: movzx r14d, byte ptr [rbx + 0xc3]
1405ac4c9: movzx r15d, byte ptr [rbx + 0xc2]
1405ac4d1: test r15b, 1
1405ac4d5: je 0x1405ac4e3
1405ac4d7: lea rcx, [rbx + 0x2f0]
1405ac4de: call 0x140602c80 ; core::ptr::drop_in_place<std::sync::mpsc::Receiver<bevy_app::sub_app::SubApp> >
1405ac4e3: test r12b, 1
1405ac4e7: je 0x1405ac57e
1405ac4ed: lea rcx, [rbx + 0x2e0]
1405ac4f4: call 0x140601d60 ; core::ptr::drop_in_place<std::sync::mpsc::Sender<bevy_app::sub_app::SubApp> >
1405ac4f9: mov byte ptr [rbx + 0xe4], r15b
1405ac500: mov byte ptr [rbx + 0xe5], r14b
1405ac507: mov byte ptr [rbx + 0xe6], dil
1405ac50e: mov byte ptr [rbx + 0xe7], sil
1405ac515: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ac51d: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ac525: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ac52e: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ac537: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ac540: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ac549: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ac552: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ac55b: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ac564: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ac56a: add rsp, 0x118
1405ac571: pop rbx
1405ac572: pop rdi
1405ac573: pop rsi
1405ac574: pop r12
1405ac576: pop r13
1405ac578: pop r14
1405ac57a: pop r15
1405ac57c: pop rbp
1405ac57d: ret 
1405ac57e: mov byte ptr [rbx + 0xe4], r15b
1405ac585: mov byte ptr [rbx + 0xe5], r14b
1405ac58c: mov byte ptr [rbx + 0xe6], dil
1405ac593: mov byte ptr [rbx + 0xe7], sil
1405ac59a: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ac5a2: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ac5aa: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ac5b3: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ac5bc: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ac5c5: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ac5ce: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ac5d7: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ac5e0: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ac5e9: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ac5ef: add rsp, 0x118
1405ac5f6: pop rbx
1405ac5f7: pop rdi
1405ac5f8: pop rsi
1405ac5f9: pop r12
1405ac5fb: pop r13
1405ac5fd: pop r14
1405ac5ff: pop r15
1405ac601: pop rbp
1405ac602: ret 
1405ac603: nop word ptr cs:[rax + rax]
1405ac610: mov qword ptr [rsp + 0x10], rdx
1405ac615: push rbp
1405ac616: push r15
1405ac618: push r14
1405ac61a: push r13
1405ac61c: push r12
1405ac61e: push rsi
1405ac61f: push rdi
1405ac620: push rbx
1405ac621: sub rsp, 0x118
1405ac628: lea rbp, [rdx + 0x80]
1405ac62f: movaps xmmword ptr [rsp + 0x70], xmm15
1405ac635: movaps xmmword ptr [rsp + 0x80], xmm14
1405ac63e: movaps xmmword ptr [rsp + 0x90], xmm13
1405ac647: movaps xmmword ptr [rsp + 0xa0], xmm12
1405ac650: movaps xmmword ptr [rsp + 0xb0], xmm11
1405ac659: movaps xmmword ptr [rsp + 0xc0], xmm10
1405ac662: movaps xmmword ptr [rsp + 0xd0], xmm9
1405ac66b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405ac674: movaps xmmword ptr [rsp + 0xf0], xmm7
1405ac67c: movaps xmmword ptr [rsp + 0x100], xmm6
1405ac684: and rdx, 0xffffffffffffff80
1405ac688: mov rbx, rdx
1405ac68b: movzx esi, byte ptr [rbx + 0xe7]
1405ac692: movzx edi, byte ptr [rbx + 0xe6]
1405ac699: movzx r14d, byte ptr [rbx + 0xe5]
1405ac6a1: movzx r15d, byte ptr [rbx + 0xe4]
1405ac6a9: lea rcx, [rbx + 0x290]
1405ac6b0: call 0x1405f5c20 ; core::ptr::drop_in_place<std::thread::JoinHandle<tuple$<> > >
1405ac6b5: mov byte ptr [rbx + 0xd4], r15b
1405ac6bc: mov byte ptr [rbx + 0xd5], r14b
1405ac6c3: mov byte ptr [rbx + 0xd6], dil
1405ac6ca: mov byte ptr [rbx + 0xd7], 0
1405ac6d1: mov byte ptr [rbx + 0xd8], sil
1405ac6d8: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ac6e0: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ac6e8: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ac6f1: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ac6fa: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ac703: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ac70c: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ac715: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ac71e: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ac727: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ac72d: add rsp, 0x118
1405ac734: pop rbx
1405ac735: pop rdi
1405ac736: pop rsi
1405ac737: pop r12
1405ac739: pop r13
1405ac73b: pop r14
1405ac73d: pop r15
1405ac73f: pop rbp
1405ac740: ret 
1405ac741: nop word ptr cs:[rax + rax]
1405ac750: mov qword ptr [rsp + 0x10], rdx
1405ac755: push rbp
1405ac756: push r15
1405ac758: push r14
1405ac75a: push r13
1405ac75c: push r12
1405ac75e: push rsi
1405ac75f: push rdi
1405ac760: push rbx
1405ac761: sub rsp, 0x118
1405ac768: lea rbp, [rdx + 0x80]
1405ac76f: movaps xmmword ptr [rsp + 0x70], xmm15
1405ac775: movaps xmmword ptr [rsp + 0x80], xmm14
1405ac77e: movaps xmmword ptr [rsp + 0x90], xmm13
1405ac787: movaps xmmword ptr [rsp + 0xa0], xmm12
1405ac790: movaps xmmword ptr [rsp + 0xb0], xmm11
1405ac799: movaps xmmword ptr [rsp + 0xc0], xmm10
1405ac7a2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405ac7ab: movaps xmmword ptr [rsp + 0xe0], xmm8
1405ac7b4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405ac7bc: movaps xmmword ptr [rsp + 0x100], xmm6
1405ac7c4: and rdx, 0xffffffffffffff80
1405ac7c8: mov rbx, rdx
1405ac7cb: movzx esi, byte ptr [rbx + 0xd8]
1405ac7d2: movzx r14d, byte ptr [rbx + 0xd7]
1405ac7da: movzx r15d, byte ptr [rbx + 0xd6]
1405ac7e2: movzx r12d, byte ptr [rbx + 0xd5]
1405ac7ea: movzx edi, byte ptr [rbx + 0xd4]
1405ac7f1: test dil, 1
1405ac7f5: je 0x1405ac8b1
1405ac7fb: lea rcx, [rbx + 0x2188]
1405ac802: call 0x1405f7630 ; core::ptr::drop_in_place<tiny_glade::run_game::RenderResourceCache>
1405ac807: mov byte ptr [rbx + 0xb4], r12b
1405ac80e: mov eax, dword ptr [rbx + 0x130]
1405ac814: mov byte ptr [rbx + 0xb5], al
1405ac81a: mov byte ptr [rbx + 0xb6], r15b
1405ac821: mov byte ptr [rbx + 0xb7], 0
1405ac828: and r14b, 1
1405ac82c: mov byte ptr [rbx + 0xb8], r14b
1405ac833: mov byte ptr [rbx + 0xb9], 0
1405ac83a: mov byte ptr [rbx + 0xba], dil
1405ac841: mov byte ptr [rbx + 0xbb], sil
1405ac848: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ac850: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ac858: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ac861: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ac86a: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ac873: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ac87c: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ac885: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ac88e: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ac897: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ac89d: add rsp, 0x118
1405ac8a4: pop rbx
1405ac8a5: pop rdi
1405ac8a6: pop rsi
1405ac8a7: pop r12
1405ac8a9: pop r13
1405ac8ab: pop r14
1405ac8ad: pop r15
1405ac8af: pop rbp
1405ac8b0: ret 
1405ac8b1: mov byte ptr [rbx + 0xb4], r12b
1405ac8b8: mov eax, dword ptr [rbx + 0x130]
1405ac8be: mov byte ptr [rbx + 0xb5], al
1405ac8c4: mov byte ptr [rbx + 0xb6], r15b
1405ac8cb: mov byte ptr [rbx + 0xb7], 0
1405ac8d2: and r14b, 1
1405ac8d6: mov byte ptr [rbx + 0xb8], r14b
1405ac8dd: mov byte ptr [rbx + 0xb9], 0
1405ac8e4: mov byte ptr [rbx + 0xba], dil
1405ac8eb: mov byte ptr [rbx + 0xbb], sil
1405ac8f2: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ac8fa: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ac902: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ac90b: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ac914: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ac91d: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ac926: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ac92f: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ac938: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ac941: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ac947: add rsp, 0x118
1405ac94e: pop rbx
1405ac94f: pop rdi
1405ac950: pop rsi
1405ac951: pop r12
1405ac953: pop r13
1405ac955: pop r14
1405ac957: pop r15
1405ac959: pop rbp
1405ac95a: ret 
1405ac95b: nop dword ptr [rax + rax]
1405ac960: mov qword ptr [rsp + 0x10], rdx
1405ac965: push rbp
1405ac966: push r15
1405ac968: push r14
1405ac96a: push r13
1405ac96c: push r12
1405ac96e: push rsi
1405ac96f: push rdi
1405ac970: push rbx
1405ac971: sub rsp, 0x118
1405ac978: lea rbp, [rdx + 0x80]
1405ac97f: movaps xmmword ptr [rsp + 0x70], xmm15
1405ac985: movaps xmmword ptr [rsp + 0x80], xmm14
1405ac98e: movaps xmmword ptr [rsp + 0x90], xmm13
1405ac997: movaps xmmword ptr [rsp + 0xa0], xmm12
1405ac9a0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405ac9a9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405ac9b2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405ac9bb: movaps xmmword ptr [rsp + 0xe0], xmm8
1405ac9c4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405ac9cc: movaps xmmword ptr [rsp + 0x100], xmm6
1405ac9d4: and rdx, 0xffffffffffffff80
1405ac9d8: mov rbx, rdx
1405ac9db: movzx eax, byte ptr [rbx + 0xbb]
1405ac9e2: mov byte ptr [rbx + 0xe8], al
1405ac9e8: movzx eax, byte ptr [rbx + 0xba]
1405ac9ef: mov byte ptr [rbx + 0x130], al
1405ac9f5: movzx r14d, byte ptr [rbx + 0xb9]
1405ac9fd: movzx r15d, byte ptr [rbx + 0xb8]
1405aca05: movzx edi, byte ptr [rbx + 0xb7]
1405aca0c: movzx r12d, byte ptr [rbx + 0xb6]
1405aca14: movzx r13d, byte ptr [rbx + 0xb5]
1405aca1c: movzx esi, byte ptr [rbx + 0xb4]
1405aca23: mov rax, qword ptr [rbx + 0x320]
1405aca2a: test rax, rax
1405aca2d: je 0x1405aca41
1405aca2f: lock dec qword ptr [rax]
1405aca33: jne 0x1405aca41
1405aca35: lea rcx, [rbx + 0x320]
1405aca3c: call 0x1406978e0 ; alloc::sync::Arc::drop_slow<parking::Inner,alloc::alloc::Global>
1405aca41: test dil, 1
1405aca45: je 0x1405acb13
1405aca4b: lea rcx, [rbx + 0x300]
1405aca52: call 0x1406087e0 ; core::ptr::drop_in_place<country_core::resources::audio_resources::AudioSamplesLoadedPromise>
1405aca57: mov byte ptr [rbx + 0xaa], sil
1405aca5e: mov byte ptr [rbx + 0xab], r13b
1405aca65: mov byte ptr [rbx + 0xac], r12b
1405aca6c: and r15b, 1
1405aca70: mov byte ptr [rbx + 0xad], r15b
1405aca77: mov byte ptr [rbx + 0xae], 0
1405aca7e: and r14b, 1
1405aca82: mov byte ptr [rbx + 0xaf], r14b
1405aca89: mov byte ptr [rbx + 0xb0], 0
1405aca90: movzx eax, byte ptr [rbx + 0x130]
1405aca97: mov byte ptr [rbx + 0xb1], al
1405aca9d: movzx eax, byte ptr [rbx + 0xe8]
1405acaa4: mov byte ptr [rbx + 0xb2], al
1405acaaa: movaps xmm6, xmmword ptr [rsp + 0x100]
1405acab2: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405acaba: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405acac3: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405acacc: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405acad5: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405acade: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405acae7: movaps xmm13, xmmword ptr [rsp + 0x90]
1405acaf0: movaps xmm14, xmmword ptr [rsp + 0x80]
1405acaf9: movaps xmm15, xmmword ptr [rsp + 0x70]
1405acaff: add rsp, 0x118
1405acb06: pop rbx
1405acb07: pop rdi
1405acb08: pop rsi
1405acb09: pop r12
1405acb0b: pop r13
1405acb0d: pop r14
1405acb0f: pop r15
1405acb11: pop rbp
1405acb12: ret 
1405acb13: mov byte ptr [rbx + 0xaa], sil
1405acb1a: mov byte ptr [rbx + 0xab], r13b
1405acb21: mov byte ptr [rbx + 0xac], r12b
1405acb28: and r15b, 1
1405acb2c: mov byte ptr [rbx + 0xad], r15b
1405acb33: mov byte ptr [rbx + 0xae], 0
1405acb3a: and r14b, 1
1405acb3e: mov byte ptr [rbx + 0xaf], r14b
1405acb45: mov byte ptr [rbx + 0xb0], 0
1405acb4c: movzx eax, byte ptr [rbx + 0x130]
1405acb53: mov byte ptr [rbx + 0xb1], al
1405acb59: movzx eax, byte ptr [rbx + 0xe8]
1405acb60: mov byte ptr [rbx + 0xb2], al
1405acb66: movaps xmm6, xmmword ptr [rsp + 0x100]
1405acb6e: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405acb76: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405acb7f: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405acb88: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405acb91: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405acb9a: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405acba3: movaps xmm13, xmmword ptr [rsp + 0x90]
1405acbac: movaps xmm14, xmmword ptr [rsp + 0x80]
1405acbb5: movaps xmm15, xmmword ptr [rsp + 0x70]
1405acbbb: add rsp, 0x118
1405acbc2: pop rbx
1405acbc3: pop rdi
1405acbc4: pop rsi
1405acbc5: pop r12
1405acbc7: pop r13
1405acbc9: pop r14
1405acbcb: pop r15
1405acbcd: pop rbp
1405acbce: ret 
1405acbcf: nop 
1405acbd0: mov qword ptr [rsp + 0x10], rdx
1405acbd5: push rbp
1405acbd6: push r15
1405acbd8: push r14
1405acbda: push r13
1405acbdc: push r12
1405acbde: push rsi
1405acbdf: push rdi
1405acbe0: push rbx
1405acbe1: sub rsp, 0x118
1405acbe8: lea rbp, [rdx + 0x80]
1405acbef: movaps xmmword ptr [rsp + 0x70], xmm15
1405acbf5: movaps xmmword ptr [rsp + 0x80], xmm14
1405acbfe: movaps xmmword ptr [rsp + 0x90], xmm13
1405acc07: movaps xmmword ptr [rsp + 0xa0], xmm12
1405acc10: movaps xmmword ptr [rsp + 0xb0], xmm11
1405acc19: movaps xmmword ptr [rsp + 0xc0], xmm10
1405acc22: movaps xmmword ptr [rsp + 0xd0], xmm9
1405acc2b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405acc34: movaps xmmword ptr [rsp + 0xf0], xmm7
1405acc3c: movaps xmmword ptr [rsp + 0x100], xmm6
1405acc44: and rdx, 0xffffffffffffff80
1405acc48: mov rbx, rdx
1405acc4b: movzx ecx, byte ptr [rbx + 0xb2]
1405acc52: movzx edx, byte ptr [rbx + 0xb1]
1405acc59: movzx r14d, byte ptr [rbx + 0xb0]
1405acc61: movzx r15d, byte ptr [rbx + 0xaf]
1405acc69: movzx r12d, byte ptr [rbx + 0xae]
1405acc71: movzx r13d, byte ptr [rbx + 0xac]
1405acc79: movzx esi, byte ptr [rbx + 0xab]
1405acc80: movzx edi, byte ptr [rbx + 0xaa]
1405acc87: cmp byte ptr [rbx + 0xad], 0
1405acc8e: je 0x1405acd72
1405acc94: mov rax, qword ptr [rbx + 0x158]
1405acc9b: lock dec qword ptr [rax]
1405acc9f: jne 0x1405accc7
1405acca1: mov byte ptr [rbx + 0xe8], cl
1405acca7: lea rcx, [rbx + 0x158]
1405accae: mov byte ptr [rbx + 0x130], dl
1405accb4: call 0x140b5c7a0 ; alloc::sync::Arc::drop_slow<std::sync::poison::rwlock::RwLock<country_core::resources::rt::bvh_builder::RtBvhBuilderGlue>,alloc::alloc::Global>
1405accb9: movzx edx, byte ptr [rbx + 0x130]
1405accc0: movzx ecx, byte ptr [rbx + 0xe8]
1405accc7: mov byte ptr [rbx + 0x108], dil
1405accce: mov byte ptr [rbx + 0x109], sil
1405accd5: mov byte ptr [rbx + 0x10a], r13b
1405accdc: and r12b, 1
1405acce0: mov byte ptr [rbx + 0x10b], r12b
1405acce7: and r15b, 1
1405acceb: mov byte ptr [rbx + 0x10c], r15b
1405accf2: and r14b, 1
1405accf6: mov byte ptr [rbx + 0x10d], r14b
1405accfd: mov byte ptr [rbx + 0x10e], dl
1405acd03: mov byte ptr [rbx + 0x10f], cl
1405acd09: movaps xmm6, xmmword ptr [rsp + 0x100]
1405acd11: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405acd19: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405acd22: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405acd2b: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405acd34: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405acd3d: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405acd46: movaps xmm13, xmmword ptr [rsp + 0x90]
1405acd4f: movaps xmm14, xmmword ptr [rsp + 0x80]
1405acd58: movaps xmm15, xmmword ptr [rsp + 0x70]
1405acd5e: add rsp, 0x118
1405acd65: pop rbx
1405acd66: pop rdi
1405acd67: pop rsi
1405acd68: pop r12
1405acd6a: pop r13
1405acd6c: pop r14
1405acd6e: pop r15
1405acd70: pop rbp
1405acd71: ret 
1405acd72: mov byte ptr [rbx + 0x108], dil
1405acd79: mov byte ptr [rbx + 0x109], sil
1405acd80: mov byte ptr [rbx + 0x10a], r13b
1405acd87: and r12b, 1
1405acd8b: mov byte ptr [rbx + 0x10b], r12b
1405acd92: and r15b, 1
1405acd96: mov byte ptr [rbx + 0x10c], r15b
1405acd9d: and r14b, 1
1405acda1: mov byte ptr [rbx + 0x10d], r14b
1405acda8: mov byte ptr [rbx + 0x10e], dl
1405acdae: mov byte ptr [rbx + 0x10f], cl
1405acdb4: movaps xmm6, xmmword ptr [rsp + 0x100]
1405acdbc: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405acdc4: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405acdcd: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405acdd6: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405acddf: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405acde8: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405acdf1: movaps xmm13, xmmword ptr [rsp + 0x90]
1405acdfa: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ace03: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ace09: add rsp, 0x118
1405ace10: pop rbx
1405ace11: pop rdi
1405ace12: pop rsi
1405ace13: pop r12
1405ace15: pop r13
1405ace17: pop r14
1405ace19: pop r15
1405ace1b: pop rbp
1405ace1c: ret 
1405ace1d: nop dword ptr [rax]
1405ace20: mov qword ptr [rsp + 0x10], rdx
1405ace25: push rbp
1405ace26: push r15
1405ace28: push r14
1405ace2a: push r13
1405ace2c: push r12
1405ace2e: push rsi
1405ace2f: push rdi
1405ace30: push rbx
1405ace31: sub rsp, 0x118
1405ace38: lea rbp, [rdx + 0x80]
1405ace3f: movaps xmmword ptr [rsp + 0x70], xmm15
1405ace45: movaps xmmword ptr [rsp + 0x80], xmm14
1405ace4e: movaps xmmword ptr [rsp + 0x90], xmm13
1405ace57: movaps xmmword ptr [rsp + 0xa0], xmm12
1405ace60: movaps xmmword ptr [rsp + 0xb0], xmm11
1405ace69: movaps xmmword ptr [rsp + 0xc0], xmm10
1405ace72: movaps xmmword ptr [rsp + 0xd0], xmm9
1405ace7b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405ace84: movaps xmmword ptr [rsp + 0xf0], xmm7
1405ace8c: movaps xmmword ptr [rsp + 0x100], xmm6
1405ace94: and rdx, 0xffffffffffffff80
1405ace98: mov rbx, rdx
1405ace9b: movzx eax, byte ptr [rbx + 0x10f]
1405acea2: mov byte ptr [rbx + 0xe8], al
1405acea8: movzx edi, byte ptr [rbx + 0x10e]
1405aceaf: movzx r14d, byte ptr [rbx + 0x10d]
1405aceb7: movzx r15d, byte ptr [rbx + 0x10c]
1405acebf: movzx r12d, byte ptr [rbx + 0x10a]
1405acec7: movzx r13d, byte ptr [rbx + 0x109]
1405acecf: movzx esi, byte ptr [rbx + 0x108]
1405aced6: cmp byte ptr [rbx + 0x10b], 0
1405acedd: je 0x1405acf97
1405acee3: lea rcx, [rbx + 0x3220]
1405aceea: call 0x1405f9ce0 ; core::ptr::drop_in_place<country_core::systems::camera_rig::MainCamera>
1405aceef: mov byte ptr [rbx + 0xdc], sil
1405acef6: mov byte ptr [rbx + 0xdd], r13b
1405acefd: mov byte ptr [rbx + 0xde], r12b
1405acf04: and r15b, 1
1405acf08: mov byte ptr [rbx + 0xdf], r15b
1405acf0f: and r14b, 1
1405acf13: mov byte ptr [rbx + 0xe0], r14b
1405acf1a: mov byte ptr [rbx + 0xe1], dil
1405acf21: movzx eax, byte ptr [rbx + 0xe8]
1405acf28: mov byte ptr [rbx + 0xe2], al
1405acf2e: movaps xmm6, xmmword ptr [rsp + 0x100]
1405acf36: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405acf3e: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405acf47: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405acf50: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405acf59: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405acf62: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405acf6b: movaps xmm13, xmmword ptr [rsp + 0x90]
1405acf74: movaps xmm14, xmmword ptr [rsp + 0x80]
1405acf7d: movaps xmm15, xmmword ptr [rsp + 0x70]
1405acf83: add rsp, 0x118
1405acf8a: pop rbx
1405acf8b: pop rdi
1405acf8c: pop rsi
1405acf8d: pop r12
1405acf8f: pop r13
1405acf91: pop r14
1405acf93: pop r15
1405acf95: pop rbp
1405acf96: ret 
1405acf97: mov byte ptr [rbx + 0xdc], sil
1405acf9e: mov byte ptr [rbx + 0xdd], r13b
1405acfa5: mov byte ptr [rbx + 0xde], r12b
1405acfac: and r15b, 1
1405acfb0: mov byte ptr [rbx + 0xdf], r15b
1405acfb7: and r14b, 1
1405acfbb: mov byte ptr [rbx + 0xe0], r14b
1405acfc2: mov byte ptr [rbx + 0xe1], dil
1405acfc9: movzx eax, byte ptr [rbx + 0xe8]
1405acfd0: mov byte ptr [rbx + 0xe2], al
1405acfd6: movaps xmm6, xmmword ptr [rsp + 0x100]
1405acfde: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405acfe6: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405acfef: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405acff8: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ad001: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ad00a: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ad013: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ad01c: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ad025: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ad02b: add rsp, 0x118
1405ad032: pop rbx
1405ad033: pop rdi
1405ad034: pop rsi
1405ad035: pop r12
1405ad037: pop r13
1405ad039: pop r14
1405ad03b: pop r15
1405ad03d: pop rbp
1405ad03e: ret 
1405ad03f: nop 
1405ad040: mov qword ptr [rsp + 0x10], rdx
1405ad045: push rbp
1405ad046: push r15
1405ad048: push r14
1405ad04a: push r13
1405ad04c: push r12
1405ad04e: push rsi
1405ad04f: push rdi
1405ad050: push rbx
1405ad051: sub rsp, 0x118
1405ad058: lea rbp, [rdx + 0x80]
1405ad05f: movaps xmmword ptr [rsp + 0x70], xmm15
1405ad065: movaps xmmword ptr [rsp + 0x80], xmm14
1405ad06e: movaps xmmword ptr [rsp + 0x90], xmm13
1405ad077: movaps xmmword ptr [rsp + 0xa0], xmm12
1405ad080: movaps xmmword ptr [rsp + 0xb0], xmm11
1405ad089: movaps xmmword ptr [rsp + 0xc0], xmm10
1405ad092: movaps xmmword ptr [rsp + 0xd0], xmm9
1405ad09b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405ad0a4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405ad0ac: movaps xmmword ptr [rsp + 0x100], xmm6
1405ad0b4: and rdx, 0xffffffffffffff80
1405ad0b8: mov rbx, rdx
1405ad0bb: movzx esi, byte ptr [rbx + 0xe2]
1405ad0c2: movzx eax, byte ptr [rbx + 0xe1]
1405ad0c9: mov byte ptr [rbx + 0xe8], al
1405ad0cf: movzx r14d, byte ptr [rbx + 0xe0]
1405ad0d7: movzx r15d, byte ptr [rbx + 0xdf]
1405ad0df: movzx r12d, byte ptr [rbx + 0xde]
1405ad0e7: movzx r13d, byte ptr [rbx + 0xdd]
1405ad0ef: movzx edi, byte ptr [rbx + 0xdc]
1405ad0f6: test sil, 1
1405ad0fa: je 0x1405ad1b4
1405ad100: lea rcx, [rbx + 0x20e0]
1405ad107: call 0x1405f37f0 ; core::ptr::drop_in_place<tiny_glade::egui_backend::EguiBackend>
1405ad10c: mov byte ptr [rbx + 0xfe], dil
1405ad113: mov byte ptr [rbx + 0xff], r13b
1405ad11a: mov byte ptr [rbx + 0x100], r12b
1405ad121: and r15b, 1
1405ad125: mov byte ptr [rbx + 0x101], r15b
1405ad12c: and r14b, 1
1405ad130: mov byte ptr [rbx + 0x102], r14b
1405ad137: movzx eax, byte ptr [rbx + 0xe8]
1405ad13e: mov byte ptr [rbx + 0x103], al
1405ad144: mov byte ptr [rbx + 0x104], sil
1405ad14b: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ad153: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ad15b: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ad164: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ad16d: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ad176: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ad17f: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ad188: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ad191: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ad19a: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ad1a0: add rsp, 0x118
1405ad1a7: pop rbx
1405ad1a8: pop rdi
1405ad1a9: pop rsi
1405ad1aa: pop r12
1405ad1ac: pop r13
1405ad1ae: pop r14
1405ad1b0: pop r15
1405ad1b2: pop rbp
1405ad1b3: ret 
1405ad1b4: mov byte ptr [rbx + 0xfe], dil
1405ad1bb: mov byte ptr [rbx + 0xff], r13b
1405ad1c2: mov byte ptr [rbx + 0x100], r12b
1405ad1c9: and r15b, 1
1405ad1cd: mov byte ptr [rbx + 0x101], r15b
1405ad1d4: and r14b, 1
1405ad1d8: mov byte ptr [rbx + 0x102], r14b
1405ad1df: movzx eax, byte ptr [rbx + 0xe8]
1405ad1e6: mov byte ptr [rbx + 0x103], al
1405ad1ec: mov byte ptr [rbx + 0x104], sil
1405ad1f3: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ad1fb: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ad203: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ad20c: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ad215: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ad21e: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ad227: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ad230: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ad239: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ad242: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ad248: add rsp, 0x118
1405ad24f: pop rbx
1405ad250: pop rdi
1405ad251: pop rsi
1405ad252: pop r12
1405ad254: pop r13
1405ad256: pop r14
1405ad258: pop r15
1405ad25a: pop rbp
1405ad25b: ret 
1405ad25c: nop dword ptr [rax]
1405ad260: mov qword ptr [rsp + 0x10], rdx
1405ad265: push rbp
1405ad266: push r15
1405ad268: push r14
1405ad26a: push r13
1405ad26c: push r12
1405ad26e: push rsi
1405ad26f: push rdi
1405ad270: push rbx
1405ad271: sub rsp, 0x118
1405ad278: lea rbp, [rdx + 0x80]
1405ad27f: movaps xmmword ptr [rsp + 0x70], xmm15
1405ad285: movaps xmmword ptr [rsp + 0x80], xmm14
1405ad28e: movaps xmmword ptr [rsp + 0x90], xmm13
1405ad297: movaps xmmword ptr [rsp + 0xa0], xmm12
1405ad2a0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405ad2a9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405ad2b2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405ad2bb: movaps xmmword ptr [rsp + 0xe0], xmm8
1405ad2c4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405ad2cc: movaps xmmword ptr [rsp + 0x100], xmm6
1405ad2d4: and rdx, 0xffffffffffffff80
1405ad2d8: mov rbx, rdx
1405ad2db: movzx esi, byte ptr [rbx + 0x104]
1405ad2e2: movzx edi, byte ptr [rbx + 0x103]
1405ad2e9: movzx r14d, byte ptr [rbx + 0x102]
1405ad2f1: movzx r15d, byte ptr [rbx + 0x100]
1405ad2f9: movzx r12d, byte ptr [rbx + 0xff]
1405ad301: movzx r13d, byte ptr [rbx + 0xfe]
1405ad309: cmp byte ptr [rbx + 0x101], 0
1405ad310: je 0x1405ad3b9
1405ad316: lea rcx, [rbx + 0xaa0]
1405ad31d: call 0x1405f6770 ; core::ptr::drop_in_place<rhapsody::rg::resource_pool::ResourcePool>
1405ad322: mov byte ptr [rbx + 0xbc], r13b
1405ad329: mov byte ptr [rbx + 0xbd], r12b
1405ad330: mov byte ptr [rbx + 0xbe], r15b
1405ad337: and r14b, 1
1405ad33b: mov byte ptr [rbx + 0xbf], r14b
1405ad342: mov byte ptr [rbx + 0xc0], dil
1405ad349: mov byte ptr [rbx + 0xc1], sil
1405ad350: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ad358: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ad360: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ad369: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ad372: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ad37b: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ad384: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ad38d: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ad396: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ad39f: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ad3a5: add rsp, 0x118
1405ad3ac: pop rbx
1405ad3ad: pop rdi
1405ad3ae: pop rsi
1405ad3af: pop r12
1405ad3b1: pop r13
1405ad3b3: pop r14
1405ad3b5: pop r15
1405ad3b7: pop rbp
1405ad3b8: ret 
1405ad3b9: mov byte ptr [rbx + 0xbc], r13b
1405ad3c0: mov byte ptr [rbx + 0xbd], r12b
1405ad3c7: mov byte ptr [rbx + 0xbe], r15b
1405ad3ce: and r14b, 1
1405ad3d2: mov byte ptr [rbx + 0xbf], r14b
1405ad3d9: mov byte ptr [rbx + 0xc0], dil
1405ad3e0: mov byte ptr [rbx + 0xc1], sil
1405ad3e7: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ad3ef: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ad3f7: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ad400: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ad409: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ad412: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ad41b: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ad424: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ad42d: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ad436: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ad43c: add rsp, 0x118
1405ad443: pop rbx
1405ad444: pop rdi
1405ad445: pop rsi
1405ad446: pop r12
1405ad448: pop r13
1405ad44a: pop r14
1405ad44c: pop r15
1405ad44e: pop rbp
1405ad44f: ret 
1405ad450: mov qword ptr [rsp + 0x10], rdx
1405ad455: push rbp
1405ad456: push r15
1405ad458: push r14
1405ad45a: push r13
1405ad45c: push r12
1405ad45e: push rsi
1405ad45f: push rdi
1405ad460: push rbx
1405ad461: sub rsp, 0x118
1405ad468: lea rbp, [rdx + 0x80]
1405ad46f: movaps xmmword ptr [rsp + 0x70], xmm15
1405ad475: movaps xmmword ptr [rsp + 0x80], xmm14
1405ad47e: movaps xmmword ptr [rsp + 0x90], xmm13
1405ad487: movaps xmmword ptr [rsp + 0xa0], xmm12
1405ad490: movaps xmmword ptr [rsp + 0xb0], xmm11
1405ad499: movaps xmmword ptr [rsp + 0xc0], xmm10
1405ad4a2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405ad4ab: movaps xmmword ptr [rsp + 0xe0], xmm8
1405ad4b4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405ad4bc: movaps xmmword ptr [rsp + 0x100], xmm6
1405ad4c4: and rdx, 0xffffffffffffff80
1405ad4c8: mov rbx, rdx
1405ad4cb: movzx esi, byte ptr [rbx + 0xc1]
1405ad4d2: movzx edi, byte ptr [rbx + 0xc0]
1405ad4d9: movzx r14d, byte ptr [rbx + 0xbf]
1405ad4e1: movzx r15d, byte ptr [rbx + 0xbe]
1405ad4e9: movzx r12d, byte ptr [rbx + 0xbd]
1405ad4f1: movzx r13d, byte ptr [rbx + 0xbc]
1405ad4f9: mov rax, qword ptr [rbx + 0x138]
1405ad500: lock dec qword ptr [rax]
1405ad504: jne 0x1405ad512
1405ad506: lea rcx, [rbx + 0x138]
1405ad50d: call 0x140b5be90 ; alloc::sync::Arc::drop_slow<rhapsody::raw::device::GraphicsDevice,alloc::alloc::Global>
1405ad512: mov byte ptr [rbx + 0xf8], r13b
1405ad519: mov byte ptr [rbx + 0xf9], r12b
1405ad520: mov byte ptr [rbx + 0xfa], r15b
1405ad527: and r14b, 1
1405ad52b: mov byte ptr [rbx + 0xfb], r14b
1405ad532: mov byte ptr [rbx + 0xfc], dil
1405ad539: mov byte ptr [rbx + 0xfd], sil
1405ad540: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ad548: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ad550: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ad559: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ad562: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ad56b: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ad574: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ad57d: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ad586: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ad58f: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ad595: add rsp, 0x118
1405ad59c: pop rbx
1405ad59d: pop rdi
1405ad59e: pop rsi
1405ad59f: pop r12
1405ad5a1: pop r13
1405ad5a3: pop r14
1405ad5a5: pop r15
1405ad5a7: pop rbp
1405ad5a8: ret 
1405ad5a9: nop dword ptr [rax]
1405ad5b0: mov qword ptr [rsp + 0x10], rdx
1405ad5b5: push rbp
1405ad5b6: push r15
1405ad5b8: push r14
1405ad5ba: push r13
1405ad5bc: push r12
1405ad5be: push rsi
1405ad5bf: push rdi
1405ad5c0: push rbx
1405ad5c1: sub rsp, 0x118
1405ad5c8: lea rbp, [rdx + 0x80]
1405ad5cf: movaps xmmword ptr [rsp + 0x70], xmm15
1405ad5d5: movaps xmmword ptr [rsp + 0x80], xmm14
1405ad5de: movaps xmmword ptr [rsp + 0x90], xmm13
1405ad5e7: movaps xmmword ptr [rsp + 0xa0], xmm12
1405ad5f0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405ad5f9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405ad602: movaps xmmword ptr [rsp + 0xd0], xmm9
1405ad60b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405ad614: movaps xmmword ptr [rsp + 0xf0], xmm7
1405ad61c: movaps xmmword ptr [rsp + 0x100], xmm6
1405ad624: and rdx, 0xffffffffffffff80
1405ad628: mov rbx, rdx
1405ad62b: movzx esi, byte ptr [rbx + 0xfd]
1405ad632: movzx edi, byte ptr [rbx + 0xfc]
1405ad639: movzx r15d, byte ptr [rbx + 0xfa]
1405ad641: movzx r14d, byte ptr [rbx + 0xf9]
1405ad649: movzx r12d, byte ptr [rbx + 0xf8]
1405ad651: cmp byte ptr [rbx + 0xfb], 0
1405ad658: je 0x1405ad71d
1405ad65e: mov rcx, qword ptr [rbx + 0x2b8]
1405ad665: mov rdx, qword ptr [rbx + 0x2b0]
1405ad66c: call 0x140592970 ; core::ptr::drop_in_place<alloc::boxed::Box<slice2$<rhapsody::raw::init::AvailableGraphicsDevice>,alloc::alloc::Global> >
1405ad671: test sil, 1
1405ad675: jne 0x1405ad727
1405ad67b: test r15b, 1
1405ad67f: je 0x1405ad73d
1405ad685: lea rcx, [rbx + 0x160]
1405ad68c: call 0x140606870 ; core::ptr::drop_in_place<std::sync::mpsc::Sender<enum2$<tiny_glade::event_loop::GameEvent> > >
1405ad691: mov byte ptr [rbx + 0x115], r15b
1405ad698: mov byte ptr [rbx + 0x116], r12b
1405ad69f: mov byte ptr [rbx + 0x117], r14b
1405ad6a6: mov byte ptr [rbx + 0x118], dil
1405ad6ad: mov byte ptr [rbx + 0x119], sil
1405ad6b4: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ad6bc: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ad6c4: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ad6cd: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ad6d6: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ad6df: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ad6e8: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ad6f1: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ad6fa: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ad703: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ad709: add rsp, 0x118
1405ad710: pop rbx
1405ad711: pop rdi
1405ad712: pop rsi
1405ad713: pop r12
1405ad715: pop r13
1405ad717: pop r14
1405ad719: pop r15
1405ad71b: pop rbp
1405ad71c: ret 
1405ad71d: test sil, 1
1405ad721: je 0x1405ad67b
1405ad727: lea rcx, [rbx + 0x350]
1405ad72e: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
1405ad733: test r15b, 1
1405ad737: jne 0x1405ad685
1405ad73d: mov byte ptr [rbx + 0x115], r15b
1405ad744: mov byte ptr [rbx + 0x116], r12b
1405ad74b: mov byte ptr [rbx + 0x117], r14b
1405ad752: mov byte ptr [rbx + 0x118], dil
1405ad759: mov byte ptr [rbx + 0x119], sil
1405ad760: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ad768: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ad770: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ad779: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ad782: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ad78b: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ad794: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ad79d: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ad7a6: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ad7af: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ad7b5: add rsp, 0x118
1405ad7bc: pop rbx
1405ad7bd: pop rdi
1405ad7be: pop rsi
1405ad7bf: pop r12
1405ad7c1: pop r13
1405ad7c3: pop r14
1405ad7c5: pop r15
1405ad7c7: pop rbp
1405ad7c8: ret 
1405ad7c9: nop dword ptr [rax]
1405ad7d0: mov qword ptr [rsp + 0x10], rdx
1405ad7d5: push rbp
1405ad7d6: push r15
1405ad7d8: push r14
1405ad7da: push r13
1405ad7dc: push r12
1405ad7de: push rsi
1405ad7df: push rdi
1405ad7e0: push rbx
1405ad7e1: sub rsp, 0x118
1405ad7e8: lea rbp, [rdx + 0x80]
1405ad7ef: movaps xmmword ptr [rsp + 0x70], xmm15
1405ad7f5: movaps xmmword ptr [rsp + 0x80], xmm14
1405ad7fe: movaps xmmword ptr [rsp + 0x90], xmm13
1405ad807: movaps xmmword ptr [rsp + 0xa0], xmm12
1405ad810: movaps xmmword ptr [rsp + 0xb0], xmm11
1405ad819: movaps xmmword ptr [rsp + 0xc0], xmm10
1405ad822: movaps xmmword ptr [rsp + 0xd0], xmm9
1405ad82b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405ad834: movaps xmmword ptr [rsp + 0xf0], xmm7
1405ad83c: movaps xmmword ptr [rsp + 0x100], xmm6
1405ad844: and rdx, 0xffffffffffffff80
1405ad848: mov rbx, rdx
1405ad84b: movzx esi, byte ptr [rbx + 0x119]
1405ad852: movzx edi, byte ptr [rbx + 0x117]
1405ad859: movzx r14d, byte ptr [rbx + 0x116]
1405ad861: movzx r15d, byte ptr [rbx + 0x115]
1405ad869: test byte ptr [rbx + 0x118], 1
1405ad870: je 0x1405ad87e
1405ad872: lea rcx, [rbx + 0x328]
1405ad879: call 0x14060a2c0 ; core::ptr::drop_in_place<std::sync::mpsc::Receiver<enum2$<tiny_glade::event_loop::OsWindowEvent> > >
1405ad87e: mov rcx, qword ptr [rbp + 0x37c0]
1405ad885: call 0x1405ee980 ; core::ptr::drop_in_place<tiny_glade::app_config::CliArgs>
1405ad88a: mov byte ptr [rbx + 0x120], r15b
1405ad891: mov byte ptr [rbx + 0x121], r14b
1405ad898: mov byte ptr [rbx + 0x122], dil
1405ad89f: mov byte ptr [rbx + 0x123], sil
1405ad8a6: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ad8ae: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ad8b6: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ad8bf: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ad8c8: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ad8d1: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ad8da: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ad8e3: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ad8ec: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ad8f5: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ad8fb: add rsp, 0x118
1405ad902: pop rbx
1405ad903: pop rdi
1405ad904: pop rsi
1405ad905: pop r12
1405ad907: pop r13
1405ad909: pop r14
1405ad90b: pop r15
1405ad90d: pop rbp
1405ad90e: ret 
1405ad90f: nop 
1405ad910: mov qword ptr [rsp + 0x10], rdx
1405ad915: push rbp
1405ad916: push r15
1405ad918: push r14
1405ad91a: push r13
1405ad91c: push r12
1405ad91e: push rsi
1405ad91f: push rdi
1405ad920: push rbx
1405ad921: sub rsp, 0x118
1405ad928: lea rbp, [rdx + 0x80]
1405ad92f: movaps xmmword ptr [rsp + 0x70], xmm15
1405ad935: movaps xmmword ptr [rsp + 0x80], xmm14
1405ad93e: movaps xmmword ptr [rsp + 0x90], xmm13
1405ad947: movaps xmmword ptr [rsp + 0xa0], xmm12
1405ad950: movaps xmmword ptr [rsp + 0xb0], xmm11
1405ad959: movaps xmmword ptr [rsp + 0xc0], xmm10
1405ad962: movaps xmmword ptr [rsp + 0xd0], xmm9
1405ad96b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405ad974: movaps xmmword ptr [rsp + 0xf0], xmm7
1405ad97c: movaps xmmword ptr [rsp + 0x100], xmm6
1405ad984: and rdx, 0xffffffffffffff80
1405ad988: mov rbx, rdx
1405ad98b: movzx esi, byte ptr [rbx + 0x123]
1405ad992: movzx edi, byte ptr [rbx + 0x122]
1405ad999: movzx r14d, byte ptr [rbx + 0x121]
1405ad9a1: movzx r15d, byte ptr [rbx + 0x120]
1405ad9a9: mov rcx, qword ptr [rbp + 0x37b8]
1405ad9b0: call 0x1405f18c0 ; core::ptr::drop_in_place<tiny_glade::event_loop::EguiEventIo>
1405ad9b5: mov byte ptr [rbx + 0x124], r15b
1405ad9bc: mov byte ptr [rbx + 0x125], r14b
1405ad9c3: mov byte ptr [rbx + 0x126], dil
1405ad9ca: mov byte ptr [rbx + 0x127], sil
1405ad9d1: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ad9d9: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ad9e1: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ad9ea: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ad9f3: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ad9fc: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ada05: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ada0e: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ada17: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ada20: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ada26: add rsp, 0x118
1405ada2d: pop rbx
1405ada2e: pop rdi
1405ada2f: pop rsi
1405ada30: pop r12
1405ada32: pop r13
1405ada34: pop r14
1405ada36: pop r15
1405ada38: pop rbp
1405ada39: ret 
1405ada3a: nop word ptr [rax + rax]
1405ada40: mov qword ptr [rsp + 0x10], rdx
1405ada45: push rbp
1405ada46: push r15
1405ada48: push r14
1405ada4a: push r13
1405ada4c: push r12
1405ada4e: push rsi
1405ada4f: push rdi
1405ada50: push rbx
1405ada51: sub rsp, 0x118
1405ada58: lea rbp, [rdx + 0x80]
1405ada5f: movaps xmmword ptr [rsp + 0x70], xmm15
1405ada65: movaps xmmword ptr [rsp + 0x80], xmm14
1405ada6e: movaps xmmword ptr [rsp + 0x90], xmm13
1405ada77: movaps xmmword ptr [rsp + 0xa0], xmm12
1405ada80: movaps xmmword ptr [rsp + 0xb0], xmm11
1405ada89: movaps xmmword ptr [rsp + 0xc0], xmm10
1405ada92: movaps xmmword ptr [rsp + 0xd0], xmm9
1405ada9b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405adaa4: movaps xmmword ptr [rsp + 0xf0], xmm7
1405adaac: movaps xmmword ptr [rsp + 0x100], xmm6
1405adab4: and rdx, 0xffffffffffffff80
1405adab8: mov rbx, rdx
1405adabb: movzx esi, byte ptr [rbx + 0x127]
1405adac2: movzx edi, byte ptr [rbx + 0x125]
1405adac9: movzx r14d, byte ptr [rbx + 0x124]
1405adad1: test byte ptr [rbx + 0x126], 1
1405adad8: je 0x1405adb68
1405adade: mov rcx, qword ptr [rbp + 0x37b0]
1405adae5: call 0x1405f2450 ; core::ptr::drop_in_place<country_core::game_config::GameConfig>
1405adaea: mov byte ptr [rbx + 0x11a], r14b
1405adaf1: mov byte ptr [rbx + 0x11b], dil
1405adaf8: mov byte ptr [rbx + 0x11c], sil
1405adaff: movaps xmm6, xmmword ptr [rsp + 0x100]
1405adb07: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405adb0f: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405adb18: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405adb21: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405adb2a: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405adb33: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405adb3c: movaps xmm13, xmmword ptr [rsp + 0x90]
1405adb45: movaps xmm14, xmmword ptr [rsp + 0x80]
1405adb4e: movaps xmm15, xmmword ptr [rsp + 0x70]
1405adb54: add rsp, 0x118
1405adb5b: pop rbx
1405adb5c: pop rdi
1405adb5d: pop rsi
1405adb5e: pop r12
1405adb60: pop r13
1405adb62: pop r14
1405adb64: pop r15
1405adb66: pop rbp
1405adb67: ret 
1405adb68: mov byte ptr [rbx + 0x11a], r14b
1405adb6f: mov byte ptr [rbx + 0x11b], dil
1405adb76: mov byte ptr [rbx + 0x11c], sil
1405adb7d: movaps xmm6, xmmword ptr [rsp + 0x100]
1405adb85: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405adb8d: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405adb96: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405adb9f: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405adba8: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405adbb1: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405adbba: movaps xmm13, xmmword ptr [rsp + 0x90]
1405adbc3: movaps xmm14, xmmword ptr [rsp + 0x80]
1405adbcc: movaps xmm15, xmmword ptr [rsp + 0x70]
1405adbd2: add rsp, 0x118
1405adbd9: pop rbx
1405adbda: pop rdi
1405adbdb: pop rsi
1405adbdc: pop r12
1405adbde: pop r13
1405adbe0: pop r14
1405adbe2: pop r15
1405adbe4: pop rbp
1405adbe5: ret 
1405adbe6: nop word ptr cs:[rax + rax]
1405adbf0: mov qword ptr [rsp + 0x10], rdx
1405adbf5: push rbp
1405adbf6: push r15
1405adbf8: push r14
1405adbfa: push r13
1405adbfc: push r12
1405adbfe: push rsi
1405adbff: push rdi
1405adc00: push rbx
1405adc01: sub rsp, 0x118
1405adc08: lea rbp, [rdx + 0x80]
1405adc0f: movaps xmmword ptr [rsp + 0x70], xmm15
1405adc15: movaps xmmword ptr [rsp + 0x80], xmm14
1405adc1e: movaps xmmword ptr [rsp + 0x90], xmm13
1405adc27: movaps xmmword ptr [rsp + 0xa0], xmm12
1405adc30: movaps xmmword ptr [rsp + 0xb0], xmm11
1405adc39: movaps xmmword ptr [rsp + 0xc0], xmm10
1405adc42: movaps xmmword ptr [rsp + 0xd0], xmm9
1405adc4b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405adc54: movaps xmmword ptr [rsp + 0xf0], xmm7
1405adc5c: movaps xmmword ptr [rsp + 0x100], xmm6
1405adc64: and rdx, 0xffffffffffffff80
1405adc68: mov rbx, rdx
1405adc6b: movzx edi, byte ptr [rbx + 0x11b]
1405adc72: movzx esi, byte ptr [rbx + 0x11a]
1405adc79: mov rax, qword ptr [rbp + 0x37a8]
1405adc80: test rax, rax
1405adc83: setne cl
1405adc86: test byte ptr [rbx + 0x11c], cl
1405adc8c: je 0x1405adca0
1405adc8e: lock dec qword ptr [rax]
1405adc92: jne 0x1405adca0
1405adc94: lea rcx, [rbp + 0x37a8]
1405adc9b: call 0x140b85350 ; alloc::sync::Arc::drop_slow<steamworks::Inner<steamworks::ClientManager>,alloc::alloc::Global>
1405adca0: test dil, 1
1405adca4: je 0x1405add22
1405adca6: mov rcx, qword ptr [rbx + 0xc8]
1405adcad: call 0x1405e6cd0 ; core::ptr::drop_in_place<bevy_app::app::App>
1405adcb2: mov byte ptr [rbx + 0x11d], sil
1405adcb9: movaps xmm6, xmmword ptr [rsp + 0x100]
1405adcc1: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405adcc9: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405adcd2: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405adcdb: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405adce4: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405adced: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405adcf6: movaps xmm13, xmmword ptr [rsp + 0x90]
1405adcff: movaps xmm14, xmmword ptr [rsp + 0x80]
1405add08: movaps xmm15, xmmword ptr [rsp + 0x70]
1405add0e: add rsp, 0x118
1405add15: pop rbx
1405add16: pop rdi
1405add17: pop rsi
1405add18: pop r12
1405add1a: pop r13
1405add1c: pop r14
1405add1e: pop r15
1405add20: pop rbp
1405add21: ret 
1405add22: mov byte ptr [rbx + 0x11d], sil
1405add29: movaps xmm6, xmmword ptr [rsp + 0x100]
1405add31: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405add39: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405add42: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405add4b: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405add54: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405add5d: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405add66: movaps xmm13, xmmword ptr [rsp + 0x90]
1405add6f: movaps xmm14, xmmword ptr [rsp + 0x80]
1405add78: movaps xmm15, xmmword ptr [rsp + 0x70]
1405add7e: add rsp, 0x118
1405add85: pop rbx
1405add86: pop rdi
1405add87: pop rsi
1405add88: pop r12
1405add8a: pop r13
1405add8c: pop r14
1405add8e: pop r15
1405add90: pop rbp
1405add91: ret 
1405add92: nop word ptr cs:[rax + rax]
1405adda0: mov qword ptr [rsp + 0x10], rdx
1405adda5: push rbp
1405adda6: push r15
1405adda8: push r14
1405addaa: push r13
1405addac: push r12
1405addae: push rsi
1405addaf: push rdi
1405addb0: push rbx
1405addb1: sub rsp, 0x118
1405addb8: lea rbp, [rdx + 0x80]
1405addbf: movaps xmmword ptr [rsp + 0x70], xmm15
1405addc5: movaps xmmword ptr [rsp + 0x80], xmm14
1405addce: movaps xmmword ptr [rsp + 0x90], xmm13
1405addd7: movaps xmmword ptr [rsp + 0xa0], xmm12
1405adde0: movaps xmmword ptr [rsp + 0xb0], xmm11
1405adde9: movaps xmmword ptr [rsp + 0xc0], xmm10
1405addf2: movaps xmmword ptr [rsp + 0xd0], xmm9
1405addfb: movaps xmmword ptr [rsp + 0xe0], xmm8
1405ade04: movaps xmmword ptr [rsp + 0xf0], xmm7
1405ade0c: movaps xmmword ptr [rsp + 0x100], xmm6
1405ade14: and rdx, 0xffffffffffffff80
1405ade18: mov rbx, rdx
1405ade1b: test byte ptr [rbx + 0x11d], 1
1405ade22: je 0x1405adeaa
1405ade28: mov rax, qword ptr [rbx + 0x140]
1405ade2f: lock dec qword ptr [rax]
1405ade33: jne 0x1405ade41
1405ade35: lea rcx, [rbx + 0x140]
1405ade3c: call 0x140631840 ; alloc::sync::Arc::drop_slow<winit::window::Window,alloc::alloc::Global>
1405ade41: movaps xmm6, xmmword ptr [rsp + 0x100]
1405ade49: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405ade51: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405ade5a: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405ade63: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405ade6c: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405ade75: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405ade7e: movaps xmm13, xmmword ptr [rsp + 0x90]
1405ade87: movaps xmm14, xmmword ptr [rsp + 0x80]
1405ade90: movaps xmm15, xmmword ptr [rsp + 0x70]
1405ade96: add rsp, 0x118
1405ade9d: pop rbx
1405ade9e: pop rdi
1405ade9f: pop rsi
1405adea0: pop r12
1405adea2: pop r13
1405adea4: pop r14
1405adea6: pop r15
1405adea8: pop rbp
1405adea9: ret 
1405adeaa: movaps xmm6, xmmword ptr [rsp + 0x100]
1405adeb2: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405adeba: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405adec3: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405adecc: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405aded5: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405adede: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405adee7: movaps xmm13, xmmword ptr [rsp + 0x90]
1405adef0: movaps xmm14, xmmword ptr [rsp + 0x80]
1405adef9: movaps xmm15, xmmword ptr [rsp + 0x70]
1405adeff: add rsp, 0x118
1405adf06: pop rbx
1405adf07: pop rdi
1405adf08: pop rsi
1405adf09: pop r12
1405adf0b: pop r13
1405adf0d: pop r14
1405adf0f: pop r15
1405adf11: pop rbp
1405adf12: ret 
1405adf13: nop word ptr cs:[rax + rax]
1405adf20: mov qword ptr [rsp + 0x10], rdx
1405adf25: push rbp
1405adf26: push r15
1405adf28: push r14
1405adf2a: push r13
1405adf2c: push r12
1405adf2e: push rsi
1405adf2f: push rdi
1405adf30: push rbx
1405adf31: sub rsp, 0x118
1405adf38: lea rbp, [rdx + 0x80]
1405adf3f: movaps xmmword ptr [rsp + 0x70], xmm15
1405adf45: movaps xmmword ptr [rsp + 0x80], xmm14
1405adf4e: movaps xmmword ptr [rsp + 0x90], xmm13
1405adf57: movaps xmmword ptr [rsp + 0xa0], xmm12
1405adf60: movaps xmmword ptr [rsp + 0xb0], xmm11
1405adf69: movaps xmmword ptr [rsp + 0xc0], xmm10
1405adf72: movaps xmmword ptr [rsp + 0xd0], xmm9
1405adf7b: movaps xmmword ptr [rsp + 0xe0], xmm8
1405adf84: movaps xmmword ptr [rsp + 0xf0], xmm7
1405adf8c: movaps xmmword ptr [rsp + 0x100], xmm6
1405adf94: and rdx, 0xffffffffffffff80
1405adf98: mov rbx, rdx
1405adf9b: lea rcx, [rbx + 0x2c0]
1405adfa2: call 0x1405d3280 ; core::ptr::drop_in_place<std::sync::mpsc::Receiver<country_core::systems::audio::AsyncLoadedAudioResources> >
1405adfa7: movaps xmm6, xmmword ptr [rsp + 0x100]
1405adfaf: movaps xmm7, xmmword ptr [rsp + 0xf0]
1405adfb7: movaps xmm8, xmmword ptr [rsp + 0xe0]
1405adfc0: movaps xmm9, xmmword ptr [rsp + 0xd0]
1405adfc9: movaps xmm10, xmmword ptr [rsp + 0xc0]
1405adfd2: movaps xmm11, xmmword ptr [rsp + 0xb0]
1405adfdb: movaps xmm12, xmmword ptr [rsp + 0xa0]
1405adfe4: movaps xmm13, xmmword ptr [rsp + 0x90]
1405adfed: movaps xmm14, xmmword ptr [rsp + 0x80]
1405adff6: movaps xmm15, xmmword ptr [rsp + 0x70]
1405adffc: add rsp, 0x118
1405ae003: pop rbx
1405ae004: pop rdi
1405ae005: pop rsi
1405ae006: pop r12
1405ae008: pop r13
1405ae00a: pop r14
1405ae00c: pop r15
1405ae00e: pop rbp
1405ae00f: ret 
