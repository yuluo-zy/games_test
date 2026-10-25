142080640: 55 push rbp
142080641: 4157 push r15
142080643: 4156 push r14
142080645: 4154 push r12
142080647: 56 push rsi
142080648: 57 push rdi
142080649: 53 push rbx
14208064a: 4883ec70 sub rsp, 0x70
14208064e: 488d6c2470 lea rbp, [rsp + 0x70]
142080653: 48c745f8feffffff mov qword ptr [rbp - 8], 0xfffffffffffffffe
14208065b: 4889d6 mov rsi, rdx
14208065e: 4889cf mov rdi, rcx
142080661: e89a63b9fe call 0x140c16a00 ; puffin::are_scopes_on
142080666: 84c0 test al, al
142080668: 8845f0 mov byte ptr [rbp - 0x10], al
14208066b: 745f je 0x1420806cc
14208066d: 4c8d35162cf500 lea r14, [rip + 0xf52c16]
142080674: bb40000000 mov ebx, 0x40
142080679: ba40000000 mov edx, 0x40
14208067e: 4c89f1 mov rcx, r14
142080681: e8eae7feff call 0x14206ee70 ; core::str::impl$0::rfind<ref$<str$> >
142080686: 4883f801 cmp rax, 1
14208068a: 0f85b1000000 jne 0x142080741
142080690: 4989d1 mov r9, rdx
142080693: 4885d2 test rdx, rdx
142080696: 743d je 0x1420806d5
142080698: 4983f940 cmp r9, 0x40
14208069c: 7335 jae 0x1420806d3
14208069e: 488d05e52bf500 lea rax, [rip + 0xf52be5]
1420806a5: 41803c01bf cmp byte ptr [r9 + rax], 0xbf
1420806aa: 7f29 jg 0x1420806d5
1420806ac: 488d05252df500 lea rax, [rip + 0xf52d25]
1420806b3: 4889442420 mov qword ptr [rsp + 0x20], rax
1420806b8: 488d0dcb2bf500 lea rcx, [rip + 0xf52bcb]
1420806bf: ba40000000 mov edx, 0x40
1420806c4: 4531c0 xor r8d, r8d
1420806c7: e874928500 call 0x1428d9940 ; core::str::slice_error_fail
1420806cc: 31c0 xor eax, eax
1420806ce: e9ba010000 jmp 0x14208088d ; unknown
1420806d3: 75d7 jne 0x1420806ac
1420806d5: 4c8d35ae2bf500 lea r14, [rip + 0xf52bae]
1420806dc: 4c89f1 mov rcx, r14
1420806df: 4c89ca mov rdx, r9
1420806e2: e889e7feff call 0x14206ee70 ; core::str::impl$0::rfind<ref$<str$> >
1420806e7: 4883f801 cmp rax, 1
1420806eb: 7554 jne 0x142080741
1420806ed: 4989d0 mov r8, rdx
1420806f0: 4983c002 add r8, 2
1420806f4: 7439 je 0x14208072f
1420806f6: 4983f840 cmp r8, 0x40
1420806fa: 7331 jae 0x14208072d
1420806fc: 488d05872bf500 lea rax, [rip + 0xf52b87]
142080703: 41803c00bf cmp byte ptr [r8 + rax], 0xbf
142080708: 7f25 jg 0x14208072f
14208070a: 488d05df2cf500 lea rax, [rip + 0xf52cdf]
142080711: 4889442420 mov qword ptr [rsp + 0x20], rax
142080716: 488d0d6d2bf500 lea rcx, [rip + 0xf52b6d]
14208071d: ba40000000 mov edx, 0x40
142080722: 41b940000000 mov r9d, 0x40
142080728: e813928500 call 0x1428d9940 ; core::str::slice_error_fail
14208072d: 75db jne 0x14208070a
14208072f: bb3e000000 mov ebx, 0x3e
142080734: 4829d3 sub rbx, rdx
142080737: 4c8d354c2bf500 lea r14, [rip + 0xf52b4c]
14208073e: 4d01c6 add r14, r8
142080741: 4c8d05162df500 lea r8, [rip + 0xf52d16]
142080748: 4c8d3de12cf500 lea r15, [rip + 0xf52ce1]
14208074f: 90 nop 
142080750: 410fbe40ff movsx eax, byte ptr [r8 - 1]
142080755: 85c0 test eax, eax
142080757: 7817 js 0x142080770
142080759: 49ffc8 dec r8
14208075c: 83f85c cmp eax, 0x5c
14208075f: 7563 jne 0x1420807c4
142080761: eb73 jmp 0x1420807d6 ; unknown
142080763: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142080770: 410fb648fe movzx ecx, byte ptr [r8 - 2]
142080775: 80f9c0 cmp cl, 0xc0
142080778: 7d23 jge 0x14208079d
14208077a: 410fb650fd movzx edx, byte ptr [r8 - 3]
14208077f: 80fac0 cmp dl, 0xc0
142080782: 7d22 jge 0x1420807a6
142080784: 450fb648fc movzx r9d, byte ptr [r8 - 4]
142080789: 4983c0fc add r8, -4
14208078d: 4183e107 and r9d, 7
142080791: 41c1e106 shl r9d, 6
142080795: 83e23f and edx, 0x3f
142080798: 4409ca or edx, r9d
14208079b: eb10 jmp 0x1420807ad ; unknown
14208079d: 4983c0fe add r8, -2
1420807a1: 83e11f and ecx, 0x1f
1420807a4: eb0f jmp 0x1420807b5 ; unknown
1420807a6: 4983c0fd add r8, -3
1420807aa: 83e20f and edx, 0xf
1420807ad: c1e206 shl edx, 6
1420807b0: 83e13f and ecx, 0x3f
1420807b3: 09d1 or ecx, edx
1420807b5: c1e106 shl ecx, 6
1420807b8: 243f and al, 0x3f
1420807ba: 0fb6c0 movzx eax, al
1420807bd: 09c8 or eax, ecx
1420807bf: 83f85c cmp eax, 0x5c
1420807c2: 7412 je 0x1420807d6
1420807c4: 83f82f cmp eax, 0x2f
1420807c7: 740d je 0x1420807d6
1420807c9: 4d39f8 cmp r8, r15
1420807cc: 7582 jne 0x142080750
1420807ce: 41bc2e000000 mov r12d, 0x2e
1420807d4: eb4d jmp 0x142080823 ; unknown
1420807d6: 4c8d3d532cf500 lea r15, [rip + 0xf52c53]
1420807dd: 4d29f8 sub r8, r15
1420807e0: 49ffc0 inc r8
1420807e3: 7432 je 0x142080817
1420807e5: 4983f82e cmp r8, 0x2e
1420807e9: 732a jae 0x142080815
1420807eb: 43803c38bf cmp byte ptr [r8 + r15], 0xbf
1420807f0: 7f25 jg 0x142080817
1420807f2: 488d05bf2bf500 lea rax, [rip + 0xf52bbf]
1420807f9: 4889442420 mov qword ptr [rsp + 0x20], rax
1420807fe: 488d0d2b2cf500 lea rcx, [rip + 0xf52c2b]
142080805: ba2e000000 mov edx, 0x2e
14208080a: 41b92e000000 mov r9d, 0x2e
142080810: e82b918500 call 0x1428d9940 ; core::str::slice_error_fail
142080815: 75db jne 0x1420807f2
142080817: 41bc2e000000 mov r12d, 0x2e
14208081d: 4d29c4 sub r12, r8
142080820: 4d01c7 add r15, r8
142080823: e80879b9fe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
142080828: 4889c1 mov rcx, rax
14208082b: 488b00 mov rax, qword ptr [rax]
14208082e: 4883f801 cmp rax, 1
142080832: 0f8523010000 jne 0x14208095b
142080838: 4883c108 add rcx, 8
14208083c: 48833900 cmp qword ptr [rcx], 0
142080840: 0f854f010000 jne 0x142080995
142080846: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
14208084d: 48894de8 mov qword ptr [rbp - 0x18], rcx
142080851: 4883c108 add rcx, 8
142080855: 4c89642420 mov qword ptr [rsp + 0x20], r12
14208085a: 48c744243000000000 mov qword ptr [rsp + 0x30], 0
142080863: 48c744242801000000 mov qword ptr [rsp + 0x28], 1
14208086c: 4c89f2 mov rdx, r14
14208086f: 4989d8 mov r8, rbx
142080872: 4d89f9 mov r9, r15
142080875: e8c664b9fe call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
14208087a: 4889c3 mov rbx, rax
14208087d: 488b45e8 mov rax, qword ptr [rbp - 0x18]
142080881: 48ff00 inc qword ptr [rax]
142080884: 48895dd0 mov qword ptr [rbp - 0x30], rbx
142080888: b801000000 mov eax, 1
14208088d: 488945c8 mov qword ptr [rbp - 0x38], rax
142080891: 8b461c mov eax, dword ptr [rsi + 0x1c]
142080894: 488b4e10 mov rcx, qword ptr [rsi + 0x10]
142080898: 8901 mov dword ptr [rcx], eax
14208089a: 488b07 mov rax, qword ptr [rdi]
14208089d: 488b38 mov rdi, qword ptr [rax]
1420808a0: 4883c710 add rdi, 0x10
1420808a4: 4c8b36 mov r14, qword ptr [rsi]
1420808a7: 498d7650 lea rsi, [r14 + 0x50]
1420808ab: 41bf40140000 mov r15d, 0x1440
1420808b1: 49bc0200000000000080 movabs r12, 0x8000000000000002
1420808bb: eb11 jmp 0x1420808ce ; unknown
1420808bd: 0f1f00 nop dword ptr [rax]
1420808c0: 4881c6c0000000 add rsi, 0xc0
1420808c7: 4981c740ffffff add r15, -0xc0
1420808ce: 4d85ff test r15, r15
1420808d1: 7421 je 0x1420808f4
1420808d3: 4c3966b8 cmp qword ptr [rsi - 0x48], r12
1420808d7: 7ce7 jl 0x1420808c0
1420808d9: 4889f1 mov rcx, rsi
1420808dc: 4889fa mov rdx, rdi
1420808df: e8ec2b61ff call 0x1416934d0 ; country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::buffers::GpuInstance>
1420808e4: 488b4e48 mov rcx, qword ptr [rsi + 0x48]
1420808e8: 488b4650 mov rax, qword ptr [rsi + 0x50]
1420808ec: 4889fa mov rdx, rdi
1420808ef: ff5030 call qword ptr [rax + 0x30]
1420808f2: ebcc jmp 0x1420808c0 ; unknown
1420808f4: 498b8640140000 mov rax, qword ptr [r14 + 0x1440]
1420808fb: 49898648140000 mov qword ptr [r14 + 0x1448], rax
142080902: 49c7864014000000000000 mov qword ptr [r14 + 0x1440], 0
14208090d: 807df000 cmp byte ptr [rbp - 0x10], 0
142080911: 7439 je 0x14208094c
142080913: e81878b9fe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
142080918: 4889c1 mov rcx, rax
14208091b: 488b00 mov rax, qword ptr [rax]
14208091e: 4883f801 cmp rax, 1
142080922: 7551 jne 0x142080975
142080924: 4883c108 add rcx, 8
142080928: 48833900 cmp qword ptr [rcx], 0
14208092c: 7567 jne 0x142080995
14208092e: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
142080935: 48894df0 mov qword ptr [rbp - 0x10], rcx
142080939: 4883c108 add rcx, 8
14208093d: 4889da mov rdx, rbx
142080940: e8ab67b9fe call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
142080945: 488b45f0 mov rax, qword ptr [rbp - 0x10]
142080949: 48ff00 inc qword ptr [rax]
14208094c: 4883c470 add rsp, 0x70
142080950: 5b pop rbx
142080951: 5f pop rdi
142080952: 5e pop rsi
142080953: 415c pop r12
142080955: 415e pop r14
142080957: 415f pop r15
142080959: 5d pop rbp
14208095a: c3 ret 
14208095b: 4885c0 test rax, rax
14208095e: 7529 jne 0x142080989
142080960: 31d2 xor edx, edx
142080962: e8697e8700 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
142080967: 4889c1 mov rcx, rax
14208096a: 4885c0 test rax, rax
14208096d: 0f85c9feffff jne 0x14208083c
142080973: eb14 jmp 0x142080989 ; unknown
142080975: 4885c0 test rax, rax
142080978: 750f jne 0x142080989
14208097a: 31d2 xor edx, edx
14208097c: e84f7e8700 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
142080981: 4889c1 mov rcx, rax
142080984: 4885c0 test rax, rax
142080987: 759f jne 0x142080928
142080989: 488d0d10a1bb00 lea rcx, [rip + 0xbba110]
142080990: e83b778500 call 0x1428d80d0 ; std::thread::local::panic_access_error
142080995: 488d0dd4a1bb00 lea rcx, [rip + 0xbba1d4]
14208099c: e8af888500 call 0x1428d9250 ; core::cell::panic_already_borrowed
1420809a1: cc int3 
1420809a2: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1420809b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1420809b5: 55 push rbp
1420809b6: 4157 push r15
1420809b8: 4156 push r14
1420809ba: 4154 push r12
1420809bc: 56 push rsi
1420809bd: 57 push rdi
1420809be: 53 push rbx
1420809bf: 4883ec40 sub rsp, 0x40
1420809c3: 488d6a70 lea rbp, [rdx + 0x70]
1420809c7: 488b45e8 mov rax, qword ptr [rbp - 0x18]
1420809cb: 48ff00 inc qword ptr [rax]
1420809ce: 4883c440 add rsp, 0x40
1420809d2: 5b pop rbx
1420809d3: 5f pop rdi
1420809d4: 5e pop rsi
1420809d5: 415c pop r12
1420809d7: 415e pop r14
1420809d9: 415f pop r15
1420809db: 5d pop rbp
1420809dc: c3 ret 
1420809dd: 0f1f00 nop dword ptr [rax]
1420809e0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1420809e5: 55 push rbp
1420809e6: 4157 push r15
1420809e8: 4156 push r14
1420809ea: 4154 push r12
1420809ec: 56 push rsi
1420809ed: 57 push rdi
1420809ee: 53 push rbx
1420809ef: 4883ec40 sub rsp, 0x40
1420809f3: 488d6a70 lea rbp, [rdx + 0x70]
1420809f7: 807df000 cmp byte ptr [rbp - 0x10], 0
1420809fb: 7420 je 0x142080a1d
1420809fd: 488d45d0 lea rax, [rbp - 0x30]
142080a01: 488945e0 mov qword ptr [rbp - 0x20], rax
142080a05: 488d45e0 lea rax, [rbp - 0x20]
142080a09: 488945d8 mov qword ptr [rbp - 0x28], rax
142080a0d: 488d0d4429f500 lea rcx, [rip + 0xf52944]
142080a14: 488d55d8 lea rdx, [rbp - 0x28]
142080a18: e8b3e82aff call 0x14132f2d0 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
142080a1d: 90 nop 
142080a1e: 4883c440 add rsp, 0x40
142080a22: 5b pop rbx
142080a23: 5f pop rdi
142080a24: 5e pop rsi
142080a25: 415c pop r12
142080a27: 415e pop r14
142080a29: 415f pop r15
142080a2b: 5d pop rbp
142080a2c: c3 ret 
142080a2d: 0f1f00 nop dword ptr [rax]
142080a30: 4889542410 mov qword ptr [rsp + 0x10], rdx
142080a35: 55 push rbp
142080a36: 4157 push r15
142080a38: 4156 push r14
142080a3a: 4154 push r12
142080a3c: 56 push rsi
142080a3d: 57 push rdi
142080a3e: 53 push rbx
142080a3f: 4883ec40 sub rsp, 0x40
142080a43: 488d6a70 lea rbp, [rdx + 0x70]
142080a47: 488b45f0 mov rax, qword ptr [rbp - 0x10]
142080a4b: 48ff00 inc qword ptr [rax]
142080a4e: 4883c440 add rsp, 0x40
142080a52: 5b pop rbx
142080a53: 5f pop rdi
142080a54: 5e pop rsi
142080a55: 415c pop r12
142080a57: 415e pop r14
142080a59: 415f pop r15
142080a5b: 5d pop rbp
142080a5c: c3 ret 
