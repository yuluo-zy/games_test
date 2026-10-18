0x1404e8010 55 push rbp
0x1404e8011 4157 push r15
0x1404e8013 4156 push r14
0x1404e8015 4155 push r13
0x1404e8017 4154 push r12
0x1404e8019 56 push rsi
0x1404e801a 57 push rdi
0x1404e801b 53 push rbx
0x1404e801c b808120000 mov eax, 0x1208
0x1404e8021 e81a2a3b02 call 0x14289aa40 ; __chkstk
0x1404e8026 4829c4 sub rsp, rax
0x1404e8029 488dac2480000000 lea rbp, [rsp + 0x80]
0x1404e8031 440f299d70110000 movaps xmmword ptr [rbp + 0x1170], xmm11
0x1404e8039 440f299560110000 movaps xmmword ptr [rbp + 0x1160], xmm10
0x1404e8041 440f298d50110000 movaps xmmword ptr [rbp + 0x1150], xmm9
0x1404e8049 440f298540110000 movaps xmmword ptr [rbp + 0x1140], xmm8
0x1404e8051 0f29bd30110000 movaps xmmword ptr [rbp + 0x1130], xmm7
0x1404e8058 0f29b520110000 movaps xmmword ptr [rbp + 0x1120], xmm6
0x1404e805f 48c78518110000feffffff mov qword ptr [rbp + 0x1118], 0xfffffffffffffffe
0x1404e806a 4c898de0100000 mov qword ptr [rbp + 0x10e0], r9
0x1404e8071 4c8985a8100000 mov qword ptr [rbp + 0x10a8], r8
0x1404e8078 4889d6 mov rsi, rdx
0x1404e807b 4989cc mov r12, rcx
0x1404e807e e87de97200 call 0x140c16a00 ; puffin::are_scopes_on
0x1404e8083 84c0 test al, al
0x1404e8085 4889b5d8100000 mov qword ptr [rbp + 0x10d8], rsi
0x1404e808c 745c je 0x1404e80ea
0x1404e808e be1f000000 mov esi, 0x1f
0x1404e8093 b91f000000 mov ecx, 0x1f
0x1404e8098 e8c3faffff call 0x1404e7b60 ; core::str::impl$0::rfind<ref$<str$> >
0x1404e809d 488d1d50385002 lea rbx, [rip + 0x2503850]
0x1404e80a4 4883f801 cmp rax, 1
0x1404e80a8 0f85a7000000 jne 0x1404e8155
0x1404e80ae 4989d1 mov r9, rdx
0x1404e80b1 4885d2 test rdx, rdx
0x1404e80b4 743d je 0x1404e80f3
0x1404e80b6 4983f91f cmp r9, 0x1f
0x1404e80ba 7335 jae 0x1404e80f1
0x1404e80bc 488d0531385002 lea rax, [rip + 0x2503831]
0x1404e80c3 41803c01bf cmp byte ptr [r9 + rax], 0xbf
0x1404e80c8 7f29 jg 0x1404e80f3
0x1404e80ca 488d05c7385002 lea rax, [rip + 0x25038c7]
0x1404e80d1 4889442420 mov qword ptr [rsp + 0x20], rax
0x1404e80d6 488d0d17385002 lea rcx, [rip + 0x2503817]
0x1404e80dd ba1f000000 mov edx, 0x1f
0x1404e80e2 4531c0 xor r8d, r8d
0x1404e80e5 e856183f02 call 0x1428d9940 ; core::str::slice_error_fail
0x1404e80ea 31c0 xor eax, eax
0x1404e80ec e9c7010000 jmp 0x1404e82b8
0x1404e80f1 75d7 jne 0x1404e80ca
0x1404e80f3 4c89c9 mov rcx, r9
0x1404e80f6 e865faffff call 0x1404e7b60 ; core::str::impl$0::rfind<ref$<str$> >
0x1404e80fb 4883f801 cmp rax, 1
0x1404e80ff 7554 jne 0x1404e8155
0x1404e8101 4989d0 mov r8, rdx
0x1404e8104 4983c002 add r8, 2
0x1404e8108 7439 je 0x1404e8143
0x1404e810a 4983f81f cmp r8, 0x1f
0x1404e810e 7331 jae 0x1404e8141
0x1404e8110 488d05dd375002 lea rax, [rip + 0x25037dd]
0x1404e8117 41803c00bf cmp byte ptr [r8 + rax], 0xbf
0x1404e811c 7f25 jg 0x1404e8143
0x1404e811e 488d058b385002 lea rax, [rip + 0x250388b]
0x1404e8125 4889442420 mov qword ptr [rsp + 0x20], rax
0x1404e812a 488d0dc3375002 lea rcx, [rip + 0x25037c3]
0x1404e8131 ba1f000000 mov edx, 0x1f
0x1404e8136 41b91f000000 mov r9d, 0x1f
0x1404e813c e8ff173f02 call 0x1428d9940 ; core::str::slice_error_fail
0x1404e8141 75db jne 0x1404e811e
0x1404e8143 be1d000000 mov esi, 0x1d
0x1404e8148 4829d6 sub rsi, rdx
0x1404e814b 488d1da2375002 lea rbx, [rip + 0x25037a2]
0x1404e8152 4c01c3 add rbx, r8
0x1404e8155 4c8d05f8395002 lea r8, [rip + 0x25039f8]
0x1404e815c 4c8d3dcd395002 lea r15, [rip + 0x25039cd]
0x1404e8163 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404e8170 410fbe40ff movsx eax, byte ptr [r8 - 1]
0x1404e8175 85c0 test eax, eax
0x1404e8177 7817 js 0x1404e8190
0x1404e8179 49ffc8 dec r8
0x1404e817c 83f85c cmp eax, 0x5c
0x1404e817f 7563 jne 0x1404e81e4
0x1404e8181 eb72 jmp 0x1404e81f5
0x1404e8183 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404e8190 410fb648fe movzx ecx, byte ptr [r8 - 2]
0x1404e8195 80f9c0 cmp cl, 0xc0
0x1404e8198 7d23 jge 0x1404e81bd
0x1404e819a 410fb650fd movzx edx, byte ptr [r8 - 3]
0x1404e819f 80fac0 cmp dl, 0xc0
0x1404e81a2 7d22 jge 0x1404e81c6
0x1404e81a4 450fb648fc movzx r9d, byte ptr [r8 - 4]
0x1404e81a9 4983c0fc add r8, -4
0x1404e81ad 4183e107 and r9d, 7
0x1404e81b1 41c1e106 shl r9d, 6
0x1404e81b5 83e23f and edx, 0x3f
0x1404e81b8 4409ca or edx, r9d
0x1404e81bb eb10 jmp 0x1404e81cd
0x1404e81bd 4983c0fe add r8, -2
0x1404e81c1 83e11f and ecx, 0x1f
0x1404e81c4 eb0f jmp 0x1404e81d5
0x1404e81c6 4983c0fd add r8, -3
0x1404e81ca 83e20f and edx, 0xf
0x1404e81cd c1e206 shl edx, 6
0x1404e81d0 83e13f and ecx, 0x3f
0x1404e81d3 09d1 or ecx, edx
0x1404e81d5 c1e106 shl ecx, 6
0x1404e81d8 243f and al, 0x3f
0x1404e81da 0fb6c0 movzx eax, al
0x1404e81dd 09c8 or eax, ecx
0x1404e81df 83f85c cmp eax, 0x5c
0x1404e81e2 7411 je 0x1404e81f5
0x1404e81e4 83f82f cmp eax, 0x2f
0x1404e81e7 740c je 0x1404e81f5
0x1404e81e9 4d39f8 cmp r8, r15
0x1404e81ec 7582 jne 0x1404e8170
0x1404e81ee bf24000000 mov edi, 0x24
0x1404e81f3 eb4c jmp 0x1404e8241
0x1404e81f5 4c8d3d34395002 lea r15, [rip + 0x2503934]
0x1404e81fc 4d29f8 sub r8, r15
0x1404e81ff 49ffc0 inc r8
0x1404e8202 7432 je 0x1404e8236
0x1404e8204 4983f824 cmp r8, 0x24
0x1404e8208 732a jae 0x1404e8234
0x1404e820a 43803c38bf cmp byte ptr [r8 + r15], 0xbf
0x1404e820f 7f25 jg 0x1404e8236
0x1404e8211 488d0560375002 lea rax, [rip + 0x2503760]
0x1404e8218 4889442420 mov qword ptr [rsp + 0x20], rax
0x1404e821d 488d0d0c395002 lea rcx, [rip + 0x250390c]
0x1404e8224 ba24000000 mov edx, 0x24
0x1404e8229 41b924000000 mov r9d, 0x24
0x1404e822f e80c173f02 call 0x1428d9940 ; core::str::slice_error_fail
0x1404e8234 75db jne 0x1404e8211
0x1404e8236 bf24000000 mov edi, 0x24
0x1404e823b 4c29c7 sub rdi, r8
0x1404e823e 4d01c7 add r15, r8
0x1404e8241 e8eafe7200 call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
0x1404e8246 4889c1 mov rcx, rax
0x1404e8249 488b00 mov rax, qword ptr [rax]
0x1404e824c 4883f801 cmp rax, 1
0x1404e8250 0f852c230000 jne 0x1404ea582
0x1404e8256 4883c108 add rcx, 8
0x1404e825a 48833900 cmp qword ptr [rcx], 0
0x1404e825e 0f8542230000 jne 0x1404ea5a6
0x1404e8264 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x1404e826b 48898d00110000 mov qword ptr [rbp + 0x1100], rcx
0x1404e8272 4883c108 add rcx, 8
0x1404e8276 48897c2420 mov qword ptr [rsp + 0x20], rdi
0x1404e827b 48c744243000000000 mov qword ptr [rsp + 0x30], 0
0x1404e8284 48c744242801000000 mov qword ptr [rsp + 0x28], 1
0x1404e828d 4889da mov rdx, rbx
0x1404e8290 4989f0 mov r8, rsi
0x1404e8293 4d89f9 mov r9, r15
0x1404e8296 e8a5ea7200 call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
0x1404e829b 488b8d00110000 mov rcx, qword ptr [rbp + 0x1100]
0x1404e82a2 48ff01 inc qword ptr [rcx]
0x1404e82a5 48898588100000 mov qword ptr [rbp + 0x1088], rax
0x1404e82ac b801000000 mov eax, 1
0x1404e82b1 488bb5d8100000 mov rsi, qword ptr [rbp + 0x10d8]
0x1404e82b8 48898580100000 mov qword ptr [rbp + 0x1080], rax
0x1404e82bf 488d5e38 lea rbx, [rsi + 0x38]
0x1404e82c3 48bacfde921b5889d12e movabs rdx, 0x2ed189581b92decf
0x1404e82cd 49b8385f069693d9d315 movabs r8, 0x15d3d99396065f38
0x1404e82d7 4889d9 mov rcx, rbx
0x1404e82da e81154bfff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404e82df 4885c0 test rax, rax
0x1404e82e2 0f84ca220000 je 0x1404ea5b2
0x1404e82e8 48399618020000 cmp qword ptr [rsi + 0x218], rdx
0x1404e82ef 0f86bd220000 jbe 0x1404ea5b2
0x1404e82f5 488b8610020000 mov rax, qword ptr [rsi + 0x210]
0x1404e82fc 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404e8300 4885c9 test rcx, rcx
0x1404e8303 0f84a9220000 je 0x1404ea5b2
0x1404e8309 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
0x1404e8310 48f7d1 not rcx
0x1404e8313 488d0c49 lea rcx, [rcx + rcx*2]
0x1404e8317 48c1e105 shl rcx, 5
0x1404e831b 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404e8321 0f848b220000 je 0x1404ea5b2
0x1404e8327 4801c8 add rax, rcx
0x1404e832a 488b4010 mov rax, qword ptr [rax + 0x10]
0x1404e832e 488b38 mov rdi, qword ptr [rax]
0x1404e8331 f048ff07 lock inc qword ptr [rdi]
0x1404e8335 0f8e92240000 jle 0x1404ea7cd
0x1404e833b 4889bda0100000 mov qword ptr [rbp + 0x10a0], rdi
0x1404e8342 48ba407cd487e7567b3d movabs rdx, 0x3d7b56e787d47c40
0x1404e834c 49b8c7bf438d0ab1b067 movabs r8, 0x67b0b10a8d43bfc7
0x1404e8356 4889d9 mov rcx, rbx
0x1404e8359 e89253bfff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404e835e 4885c0 test rax, rax
0x1404e8361 0f845c220000 je 0x1404ea5c3
0x1404e8367 48399618020000 cmp qword ptr [rsi + 0x218], rdx
0x1404e836e 0f864f220000 jbe 0x1404ea5c3
0x1404e8374 488b8610020000 mov rax, qword ptr [rsi + 0x210]
0x1404e837b 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404e837f 4885c9 test rcx, rcx
0x1404e8382 0f843b220000 je 0x1404ea5c3
0x1404e8388 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
0x1404e838f 48f7d1 not rcx
0x1404e8392 488d0c49 lea rcx, [rcx + rcx*2]
0x1404e8396 48c1e105 shl rcx, 5
0x1404e839a 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404e83a0 0f841d220000 je 0x1404ea5c3
0x1404e83a6 4801c8 add rax, rcx
0x1404e83a9 488b7010 mov rsi, qword ptr [rax + 0x10]
0x1404e83ad 4531ff xor r15d, r15d
0x1404e83b0 837e6801 cmp dword ptr [rsi + 0x68], 1
0x1404e83b4 7513 jne 0x1404e83c9
0x1404e83b6 660f10466c movupd xmm0, xmmword ptr [rsi + 0x6c]
0x1404e83bb 660f2985000b0000 movapd xmmword ptr [rbp + 0xb00], xmm0
0x1404e83c3 41bf01000000 mov r15d, 1
0x1404e83c9 8b8694000000 mov eax, dword ptr [rsi + 0x94]
0x1404e83cf 8985fc100000 mov dword ptr [rbp + 0x10fc], eax
0x1404e83d5 440fb6767c movzx r14d, byte ptr [rsi + 0x7c]
0x1404e83da 440fb66e7d movzx r13d, byte ptr [rsi + 0x7d]
0x1404e83df 488b4660 mov rax, qword ptr [rsi + 0x60]
0x1404e83e3 488985b8100000 mov qword ptr [rbp + 0x10b8], rax
0x1404e83ea 488b4e38 mov rcx, qword ptr [rsi + 0x38]
0x1404e83ee 4889c8 mov rax, rcx
0x1404e83f1 48f7d8 neg rax
0x1404e83f4 48b80100000000000080 movabs rax, 0x8000000000000001
0x1404e83fe 7025 jo 0x1404e8425
0x1404e8400 4839c1 cmp rcx, rax
0x1404e8403 7509 jne 0x1404e840e
0x1404e8405 48898d00110000 mov qword ptr [rbp + 0x1100], rcx
0x1404e840c eb3e jmp 0x1404e844c
0x1404e840e 488d5638 lea rdx, [rsi + 0x38]
0x1404e8412 488d8d200d0000 lea rcx, [rbp + 0xd20]
0x1404e8419 e872398900 call 0x140d7bd90 ; alloc::string::impl$6::clone
0x1404e841e 488b8d200d0000 mov rcx, qword ptr [rbp + 0xd20]
0x1404e8425 48898d00110000 mov qword ptr [rbp + 0x1100], rcx
0x1404e842c 0f107650 movups xmm6, xmmword ptr [rsi + 0x50]
0x1404e8430 488b85280d0000 mov rax, qword ptr [rbp + 0xd28]
0x1404e8437 488985f0100000 mov qword ptr [rbp + 0x10f0], rax
0x1404e843e 488b85300d0000 mov rax, qword ptr [rbp + 0xd30]
0x1404e8445 488985f00a0000 mov qword ptr [rbp + 0xaf0], rax
0x1404e844c 660f2885000b0000 movapd xmm0, xmmword ptr [rbp + 0xb00]
0x1404e8454 660f298590000000 movapd xmmword ptr [rbp + 0x90], xmm0
0x1404e845c 8b8680000000 mov eax, dword ptr [rsi + 0x80]
0x1404e8462 8985c8100000 mov dword ptr [rbp + 0x10c8], eax
0x1404e8468 f30f10be84000000 movss xmm7, dword ptr [rsi + 0x84]
0x1404e8470 8b8688000000 mov eax, dword ptr [rsi + 0x88]
0x1404e8476 8985d0100000 mov dword ptr [rbp + 0x10d0], eax
0x1404e847c f3440f10868c000000 movss xmm8, dword ptr [rsi + 0x8c]
0x1404e8485 f2440f108ea8000000 movsd xmm9, qword ptr [rsi + 0xa8]
0x1404e848e 0fb686c3000000 movzx eax, byte ptr [rsi + 0xc3]
0x1404e8495 8885c0100000 mov byte ptr [rbp + 0x10c0], al
0x1404e849b 440f109698000000 movups xmm10, xmmword ptr [rsi + 0x98]
0x1404e84a3 4c8d0596355002 lea r8, [rip + 0x2503596]
0x1404e84aa 488d8d10030000 lea rcx, [rbp + 0x310]
0x1404e84b1 4889f2 mov rdx, rsi
0x1404e84b4 e847ffdcff call 0x1402b8400 ; alloc::vec::impl$11::clone<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>
0x1404e84b9 4c89a5f80a0000 mov qword ptr [rbp + 0xaf8], r12
0x1404e84c0 80beac00000001 cmp byte ptr [rsi + 0xac], 1
0x1404e84c7 4889bd90100000 mov qword ptr [rbp + 0x1090], rdi
0x1404e84ce 48899d98100000 mov qword ptr [rbp + 0x1098], rbx
0x1404e84d5 751a jne 0x1404e84f1
0x1404e84d7 660f1086ad000000 movupd xmm0, xmmword ptr [rsi + 0xad]
0x1404e84df 660f298530050000 movapd xmmword ptr [rbp + 0x530], xmm0
0x1404e84e7 b001 mov al, 1
0x1404e84e9 8985e8100000 mov dword ptr [rbp + 0x10e8], eax
0x1404e84ef eb0a jmp 0x1404e84fb
0x1404e84f1 c785e810000000000000 mov dword ptr [rbp + 0x10e8], 0
0x1404e84fb f3440f109ebd000000 movss xmm11, dword ptr [rsi + 0xbd]
0x1404e8504 448aa6c1000000 mov r12b, byte ptr [rsi + 0xc1]
0x1404e850b 408abec2000000 mov dil, byte ptr [rsi + 0xc2]
0x1404e8512 488d5618 lea rdx, [rsi + 0x18]
0x1404e8516 8b5e30 mov ebx, dword ptr [rsi + 0x30]
0x1404e8519 4c8d0588355002 lea r8, [rip + 0x2503588]
0x1404e8520 488d8d000b0000 lea rcx, [rbp + 0xb00]
0x1404e8527 e864f3dcff call 0x1402b7890 ; alloc::vec::impl$11::clone<enum2$<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>
0x1404e852c 899d180b0000 mov dword ptr [rbp + 0xb18], ebx
0x1404e8532 8b8690000000 mov eax, dword ptr [rsi + 0x90]
0x1404e8538 8b8dfc100000 mov ecx, dword ptr [rbp + 0x10fc]
0x1404e853e 898db40d0000 mov dword ptr [rbp + 0xdb4], ecx
0x1404e8544 488b8d00110000 mov rcx, qword ptr [rbp + 0x1100]
0x1404e854b 48898d580d0000 mov qword ptr [rbp + 0xd58], rcx
0x1404e8552 488b8df0100000 mov rcx, qword ptr [rbp + 0x10f0]
0x1404e8559 48898d600d0000 mov qword ptr [rbp + 0xd60], rcx
0x1404e8560 488b8df00a0000 mov rcx, qword ptr [rbp + 0xaf0]
0x1404e8567 48898d680d0000 mov qword ptr [rbp + 0xd68], rcx
0x1404e856e 0f29b5700d0000 movaps xmmword ptr [rbp + 0xd70], xmm6
0x1404e8575 488b8db8100000 mov rcx, qword ptr [rbp + 0x10b8]
0x1404e857c 48898d800d0000 mov qword ptr [rbp + 0xd80], rcx
0x1404e8583 4489bd880d0000 mov dword ptr [rbp + 0xd88], r15d
0x1404e858a 0f288590000000 movaps xmm0, xmmword ptr [rbp + 0x90]
0x1404e8591 0f11858c0d0000 movups xmmword ptr [rbp + 0xd8c], xmm0
0x1404e8598 4488b59c0d0000 mov byte ptr [rbp + 0xd9c], r14b
0x1404e859f 4488ad9d0d0000 mov byte ptr [rbp + 0xd9d], r13b
0x1404e85a6 8b8dc8100000 mov ecx, dword ptr [rbp + 0x10c8]
0x1404e85ac 898da00d0000 mov dword ptr [rbp + 0xda0], ecx
0x1404e85b2 f30f11bda40d0000 movss dword ptr [rbp + 0xda4], xmm7
0x1404e85ba 8b8dd0100000 mov ecx, dword ptr [rbp + 0x10d0]
0x1404e85c0 898da80d0000 mov dword ptr [rbp + 0xda8], ecx
0x1404e85c6 f3440f1185ac0d0000 movss dword ptr [rbp + 0xdac], xmm8
0x1404e85cf f3440f118dc80d0000 movss dword ptr [rbp + 0xdc8], xmm9
0x1404e85d8 0fb68dc0100000 movzx ecx, byte ptr [rbp + 0x10c0]
0x1404e85df 888de30d0000 mov byte ptr [rbp + 0xde3], cl
0x1404e85e5 440f1195b80d0000 movups xmmword ptr [rbp + 0xdb8], xmm10
0x1404e85ed 488b8d20030000 mov rcx, qword ptr [rbp + 0x320]
0x1404e85f4 48898d300d0000 mov qword ptr [rbp + 0xd30], rcx
0x1404e85fb 0f108510030000 movups xmm0, xmmword ptr [rbp + 0x310]
0x1404e8602 0f2985200d0000 movaps xmmword ptr [rbp + 0xd20], xmm0
0x1404e8609 8b8de8100000 mov ecx, dword ptr [rbp + 0x10e8]
0x1404e860f 888dcc0d0000 mov byte ptr [rbp + 0xdcc], cl
0x1404e8615 0f288530050000 movaps xmm0, xmmword ptr [rbp + 0x530]
0x1404e861c 0f1185cd0d0000 movups xmmword ptr [rbp + 0xdcd], xmm0
0x1404e8623 f3440f119ddd0d0000 movss dword ptr [rbp + 0xddd], xmm11
0x1404e862c 4488a5e10d0000 mov byte ptr [rbp + 0xde1], r12b
0x1404e8633 4088bde20d0000 mov byte ptr [rbp + 0xde2], dil
0x1404e863a 660f1085000b0000 movupd xmm0, xmmword ptr [rbp + 0xb00]
0x1404e8642 0f108d100b0000 movups xmm1, xmmword ptr [rbp + 0xb10]
0x1404e8649 660f1185380d0000 movupd xmmword ptr [rbp + 0xd38], xmm0
0x1404e8651 0f118d480d0000 movups xmmword ptr [rbp + 0xd48], xmm1
0x1404e8658 8985b00d0000 mov dword ptr [rbp + 0xdb0], eax
0x1404e865e 440f1086f8000000 movups xmm8, xmmword ptr [rsi + 0xf8]
0x1404e8666 f30f10bef0000000 movss xmm7, dword ptr [rsi + 0xf0]
0x1404e866e 0fb69ef4000000 movzx ebx, byte ptr [rsi + 0xf4]
0x1404e8675 f2440f109ee8000000 movsd xmm11, qword ptr [rsi + 0xe8]
0x1404e867e 488bbee0000000 mov rdi, qword ptr [rsi + 0xe0]
0x1404e8685 f30f10b61c010000 movss xmm6, dword ptr [rsi + 0x11c]
0x1404e868d 440f108e08010000 movups xmm9, xmmword ptr [rsi + 0x108]
0x1404e8695 f3440f109618010000 movss xmm10, dword ptr [rsi + 0x118]
0x1404e869e 31c0 xor eax, eax
0x1404e86a0 483b86c8000000 cmp rax, qword ptr [rsi + 0xc8]
0x1404e86a7 440fb6b620010000 movzx r14d, byte ptr [rsi + 0x120]
0x1404e86af 440fb6be21010000 movzx r15d, byte ptr [rsi + 0x121]
0x1404e86b7 7113 jno 0x1404e86cc
0x1404e86b9 48b80000000000000080 movabs rax, 0x8000000000000000
0x1404e86c3 488985000b0000 mov qword ptr [rbp + 0xb00], rax
0x1404e86ca eb16 jmp 0x1404e86e2
0x1404e86cc 4881c6c8000000 add rsi, 0xc8
0x1404e86d3 488d8d000b0000 lea rcx, [rbp + 0xb00]
0x1404e86da 4889f2 mov rdx, rsi
0x1404e86dd e8ae368900 call 0x140d7bd90 ; alloc::string::impl$6::clone
0x1404e86e2 488d8d88070000 lea rcx, [rbp + 0x788]
0x1404e86e9 488db5200d0000 lea rsi, [rbp + 0xd20]
0x1404e86f0 41b8c8000000 mov r8d, 0xc8
0x1404e86f6 4889f2 mov rdx, rsi
0x1404e86f9 e802b54302 call 0x142923c00 ; memcpy
0x1404e86fe 440f118580080000 movups xmmword ptr [rbp + 0x880], xmm8
0x1404e8706 440f139d70080000 movlps qword ptr [rbp + 0x870], xmm11
0x1404e870e f30f11bd78080000 movss dword ptr [rbp + 0x878], xmm7
0x1404e8716 889d7c080000 mov byte ptr [rbp + 0x87c], bl
0x1404e871c 4889bd68080000 mov qword ptr [rbp + 0x868], rdi
0x1404e8723 440f118d90080000 movups xmmword ptr [rbp + 0x890], xmm9
0x1404e872b f3440f1195a0080000 movss dword ptr [rbp + 0x8a0], xmm10
0x1404e8734 f30f11b5a4080000 movss dword ptr [rbp + 0x8a4], xmm6
0x1404e873c 4488b5a8080000 mov byte ptr [rbp + 0x8a8], r14b
0x1404e8743 4488bda9080000 mov byte ptr [rbp + 0x8a9], r15b
0x1404e874a 660f1085000b0000 movupd xmm0, xmmword ptr [rbp + 0xb00]
0x1404e8752 660f118550080000 movupd xmmword ptr [rbp + 0x850], xmm0
0x1404e875a 488b85100b0000 mov rax, qword ptr [rbp + 0xb10]
0x1404e8761 48898560080000 mov qword ptr [rbp + 0x860], rax
0x1404e8768 0fb68548080000 movzx eax, byte ptr [rbp + 0x848]
0x1404e876f 488b0d6a381403 mov rcx, qword ptr [rip + 0x314386a]
0x1404e8776 8801 mov byte ptr [rcx], al
0x1404e8778 e883e27200 call 0x140c16a00 ; puffin::are_scopes_on
0x1404e877d 8885b8100000 mov byte ptr [rbp + 0x10b8], al
0x1404e8783 408abd08120000 mov dil, byte ptr [rbp + 0x1208]
0x1404e878a 4080ff02 cmp dil, 2
0x1404e878e 410f94c6 sete r14b
0x1404e8792 488d8d90000000 lea rcx, [rbp + 0x90]
0x1404e8799 e8a21b4d00 call 0x1409ba340 ; country_core::systems::render_loop::draw_passes::draw_lists::DrawLists::new
0x1404e879e c6851711000001 mov byte ptr [rbp + 0x1117], 1
0x1404e87a5 488d9590000000 lea rdx, [rbp + 0x90]
0x1404e87ac 488b8dd8100000 mov rcx, qword ptr [rbp + 0x10d8]
0x1404e87b3 e8881d4d00 call 0x1409ba540 ; country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists
0x1404e87b8 4c8bbde0100000 mov r15, qword ptr [rbp + 0x10e0]
0x1404e87bf c6851711000001 mov byte ptr [rbp + 0x1117], 1
0x1404e87c6 488d8d90000000 lea rcx, [rbp + 0x90]
0x1404e87cd e8de1c4d00 call 0x1409ba4b0 ; country_core::systems::render_loop::draw_passes::draw_lists::DrawLists::sort
0x1404e87d2 488b8df8110000 mov rcx, qword ptr [rbp + 0x11f8]
0x1404e87d9 488b8590100000 mov rax, qword ptr [rbp + 0x1090]
0x1404e87e0 4883c010 add rax, 0x10
0x1404e87e4 48898500110000 mov qword ptr [rbp + 0x1100], rax
0x1404e87eb 4885c9 test rcx, rcx
0x1404e87ee 743c je 0x1404e882c
0x1404e87f0 c6851711000001 mov byte ptr [rbp + 0x1117], 1
0x1404e87f7 e8e45f7200 call 0x140c0e7e0 ; rhapsody::raw::swapchain::Swapchain::format
0x1404e87fc 89c3 mov ebx, eax
0x1404e87fe c6851711000001 mov byte ptr [rbp + 0x1117], 1
0x1404e8805 488b8df8110000 mov rcx, qword ptr [rbp + 0x11f8]
0x1404e880c 488b9500110000 mov rdx, qword ptr [rbp + 0x1100]
0x1404e8813 e878627200 call 0x140c0ea90 ; rhapsody::raw::swapchain::Swapchain::extent
0x1404e8818 899d74100000 mov dword ptr [rbp + 0x1074], ebx
0x1404e881e 48898578100000 mov qword ptr [rbp + 0x1078], rax
0x1404e8825 b801000000 mov eax, 1
0x1404e882a eb02 jmp 0x1404e882e
0x1404e882c 31c0 xor eax, eax
0x1404e882e 4c8badf0110000 mov r13, qword ptr [rbp + 0x11f0]
0x1404e8835 898570100000 mov dword ptr [rbp + 0x1070], eax
0x1404e883b 4084ff test dil, dil
0x1404e883e 488b9dd8100000 mov rbx, qword ptr [rbp + 0x10d8]
0x1404e8845 0f8417010000 je 0x1404e8962
0x1404e884b 488bbd00120000 mov rdi, qword ptr [rbp + 0x1200]
0x1404e8852 4d8da7d0000000 lea r12, [r15 + 0xd0]
0x1404e8859 488d9590000000 lea rdx, [rbp + 0x90]
0x1404e8860 41b8f8010000 mov r8d, 0x1f8
0x1404e8866 4889f1 mov rcx, rsi
0x1404e8869 e892b34302 call 0x142923c00 ; memcpy
0x1404e886e 660f108570100000 movupd xmm0, xmmword ptr [rbp + 0x1070]
0x1404e8876 660f298530050000 movapd xmmword ptr [rbp + 0x530], xmm0
0x1404e887e 488b85f80a0000 mov rax, qword ptr [rbp + 0xaf8]
0x1404e8885 48898500100000 mov qword ptr [rbp + 0x1000], rax
0x1404e888c 4d8d4d18 lea r9, [r13 + 0x18]
0x1404e8890 498d4578 lea rax, [r13 + 0x78]
0x1404e8894 48899d000b0000 mov qword ptr [rbp + 0xb00], rbx
0x1404e889b 4c8b8500110000 mov r8, qword ptr [rbp + 0x1100]
0x1404e88a2 4c8985080b0000 mov qword ptr [rbp + 0xb08], r8
0x1404e88a9 488985c8100000 mov qword ptr [rbp + 0x10c8], rax
0x1404e88b0 488985100b0000 mov qword ptr [rbp + 0xb10], rax
0x1404e88b7 488d8500100000 lea rax, [rbp + 0x1000]
0x1404e88be 488985180b0000 mov qword ptr [rbp + 0xb18], rax
0x1404e88c5 4889b5200b0000 mov qword ptr [rbp + 0xb20], rsi
0x1404e88cc 488d8588070000 lea rax, [rbp + 0x788]
0x1404e88d3 488985280b0000 mov qword ptr [rbp + 0xb28], rax
0x1404e88da 488d8530050000 lea rax, [rbp + 0x530]
0x1404e88e1 488985300b0000 mov qword ptr [rbp + 0xb30], rax
0x1404e88e8 4889bd380b0000 mov qword ptr [rbp + 0xb38], rdi
0x1404e88ef 488d85000b0000 lea rax, [rbp + 0xb00]
0x1404e88f6 4889442420 mov qword ptr [rsp + 0x20], rax
0x1404e88fb 488d8d10030000 lea rcx, [rbp + 0x310]
0x1404e8902 4c89a5c0100000 mov qword ptr [rbp + 0x10c0], r12
0x1404e8909 4c89e2 mov rdx, r12
0x1404e890c 4d89cc mov r12, r9
0x1404e890f e8bc93c7ff call 0x140161cd0 ; rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0>
0x1404e8914 c6851711000000 mov byte ptr [rbp + 0x1117], 0
0x1404e891b 488d8d200d0000 lea rcx, [rbp + 0xd20]
0x1404e8922 e839161200 call 0x140609f60 ; core::ptr::drop_in_place<country_core::systems::render_loop::draw_passes::draw_lists::DrawLists>
0x1404e8927 31c0 xor eax, eax
0x1404e8929 483b8510030000 cmp rax, qword ptr [rbp + 0x310]
0x1404e8930 0f9185fc100000 setno byte ptr [rbp + 0x10fc]
0x1404e8937 0f8000010000 jo 0x1404e8a3d
0x1404e893d 488d8d30050000 lea rcx, [rbp + 0x530]
0x1404e8944 488d9510030000 lea rdx, [rbp + 0x310]
0x1404e894b 41b818020000 mov r8d, 0x218
0x1404e8951 e8aab24302 call 0x142923c00 ; memcpy
0x1404e8956 448ba528050000 mov r12d, dword ptr [rbp + 0x528]
0x1404e895d e9d40b0000 jmp 0x1404e9536
0x1404e8962 85c0 test eax, eax
0x1404e8964 0f84ad1c0000 je 0x1404ea617
0x1404e896a 488bbd78100000 mov rdi, qword ptr [rbp + 0x1078]
0x1404e8971 498db7d0000000 lea rsi, [r15 + 0xd0]
0x1404e8978 488d9590000000 lea rdx, [rbp + 0x90]
0x1404e897f 41b8f8010000 mov r8d, 0x1f8
0x1404e8985 4c8da5000b0000 lea r12, [rbp + 0xb00]
0x1404e898c 4c89e1 mov rcx, r12
0x1404e898f e86cb24302 call 0x142923c00 ; memcpy
0x1404e8994 4889bd30050000 mov qword ptr [rbp + 0x530], rdi
0x1404e899b 4d8d4d18 lea r9, [r13 + 0x18]
0x1404e899f 498d4578 lea rax, [r13 + 0x78]
0x1404e89a3 48899d10030000 mov qword ptr [rbp + 0x310], rbx
0x1404e89aa 4c8b8500110000 mov r8, qword ptr [rbp + 0x1100]
0x1404e89b1 4c898518030000 mov qword ptr [rbp + 0x318], r8
0x1404e89b8 48898520030000 mov qword ptr [rbp + 0x320], rax
0x1404e89bf 488d8530050000 lea rax, [rbp + 0x530]
0x1404e89c6 48898528030000 mov qword ptr [rbp + 0x328], rax
0x1404e89cd 4c89a530030000 mov qword ptr [rbp + 0x330], r12
0x1404e89d4 488d8510030000 lea rax, [rbp + 0x310]
0x1404e89db 4889442420 mov qword ptr [rsp + 0x20], rax
0x1404e89e0 488d8d200d0000 lea rcx, [rbp + 0xd20]
0x1404e89e7 4889f2 mov rdx, rsi
0x1404e89ea e8c1bdc7ff call 0x1401647b0 ; rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0>
0x1404e89ef c6851711000000 mov byte ptr [rbp + 0x1117], 0
0x1404e89f6 488d8d000b0000 lea rcx, [rbp + 0xb00]
0x1404e89fd e85e151200 call 0x140609f60 ; core::ptr::drop_in_place<country_core::systems::render_loop::draw_passes::draw_lists::DrawLists>
0x1404e8a02 31c0 xor eax, eax
0x1404e8a04 483b85200d0000 cmp rax, qword ptr [rbp + 0xd20]
0x1404e8a0b 0f802a1c0000 jo 0x1404ea63b
0x1404e8a11 488d8d30050000 lea rcx, [rbp + 0x530]
0x1404e8a18 488d95200d0000 lea rdx, [rbp + 0xd20]
0x1404e8a1f 41b818020000 mov r8d, 0x218
0x1404e8a25 e8d6b14302 call 0x142923c00 ; memcpy
0x1404e8a2a 448ba5380f0000 mov r12d, dword ptr [rbp + 0xf38]
0x1404e8a31 c685fc10000001 mov byte ptr [rbp + 0x10fc], 1
0x1404e8a38 e9f90a0000 jmp 0x1404e9536
0x1404e8a3d c6851711000000 mov byte ptr [rbp + 0x1117], 0
0x1404e8a44 48ba8b047be66126c0a4 movabs rdx, 0xa4c02661e67b048b
0x1404e8a4e 49b8ec672aaae661e7ea movabs r8, 0xeae761e6aa2a67ec
0x1404e8a58 488b8d98100000 mov rcx, qword ptr [rbp + 0x1098]
0x1404e8a5f e88c4cbfff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404e8a64 4885c0 test rax, rax
0x1404e8a67 488b8dd8100000 mov rcx, qword ptr [rbp + 0x10d8]
0x1404e8a6e 0f846c1b0000 je 0x1404ea5e0
0x1404e8a74 48399118020000 cmp qword ptr [rcx + 0x218], rdx
0x1404e8a7b 0f865f1b0000 jbe 0x1404ea5e0
0x1404e8a81 488b8110020000 mov rax, qword ptr [rcx + 0x210]
0x1404e8a88 488b04d0 mov rax, qword ptr [rax + rdx*8]
0x1404e8a8c 4885c0 test rax, rax
0x1404e8a8f 0f844b1b0000 je 0x1404ea5e0
0x1404e8a95 488bb1e0010000 mov rsi, qword ptr [rcx + 0x1e0]
0x1404e8a9c 48f7d0 not rax
0x1404e8a9f 488d0440 lea rax, [rax + rax*2]
0x1404e8aa3 48c1e005 shl rax, 5
0x1404e8aa7 48837c062000 cmp qword ptr [rsi + rax + 0x20], 0
0x1404e8aad 0f842d1b0000 je 0x1404ea5e0
0x1404e8ab3 4801c6 add rsi, rax
0x1404e8ab6 488b7e10 mov rdi, qword ptr [rsi + 0x10]
0x1404e8aba 8b99b0050000 mov ebx, dword ptr [rcx + 0x5b0]
0x1404e8ac0 f30f104718 movss xmm0, dword ptr [rdi + 0x18]
0x1404e8ac5 4883c718 add rdi, 0x18
0x1404e8ac9 f30f5805272c5002 addss xmm0, dword ptr [rip + 0x2502c27]
0x1404e8ad1 f30f1185b4100000 movss dword ptr [rbp + 0x10b4], xmm0
0x1404e8ad9 488b05c8381403 mov rax, qword ptr [rip + 0x31438c8]
0x1404e8ae0 488b00 mov rax, qword ptr [rax]
0x1404e8ae3 4885c0 test rax, rax
0x1404e8ae6 0f84a7010000 je 0x1404e8c93
0x1404e8aec 4889bd30050000 mov qword ptr [rbp + 0x530], rdi
0x1404e8af3 488d05163e8a00 lea rax, [rip + 0x8a3e16]
0x1404e8afa 48898538050000 mov qword ptr [rbp + 0x538], rax
0x1404e8b01 488d8db4100000 lea rcx, [rbp + 0x10b4]
0x1404e8b08 48898d40050000 mov qword ptr [rbp + 0x540], rcx
0x1404e8b0f 48898548050000 mov qword ptr [rbp + 0x548], rax
0x1404e8b16 48c785000b000000000000 mov qword ptr [rbp + 0xb00], 0
0x1404e8b21 48c785080b000001000000 mov qword ptr [rbp + 0xb08], 1
0x1404e8b2c 48c785100b000000000000 mov qword ptr [rbp + 0xb10], 0
0x1404e8b37 48c785180b000001000000 mov qword ptr [rbp + 0xb18], 1
0x1404e8b42 48c785200b000000000000 mov qword ptr [rbp + 0xb20], 0
0x1404e8b4d 48c785280b000020000000 mov qword ptr [rbp + 0xb28], 0x20
0x1404e8b58 c685300b000003 mov byte ptr [rbp + 0xb30], 3
0x1404e8b5f 48c785380b000000000000 mov qword ptr [rbp + 0xb38], 0
0x1404e8b6a 48c785400b000001000000 mov qword ptr [rbp + 0xb40], 1
0x1404e8b75 48c785480b000000000000 mov qword ptr [rbp + 0xb48], 0
0x1404e8b80 48c785500b000001000000 mov qword ptr [rbp + 0xb50], 1
0x1404e8b8b 48c785580b000001000000 mov qword ptr [rbp + 0xb58], 1
0x1404e8b96 48c785600b000020000000 mov qword ptr [rbp + 0xb60], 0x20
0x1404e8ba1 c685680b000003 mov byte ptr [rbp + 0xb68], 3
0x1404e8ba8 c6851711000000 mov byte ptr [rbp + 0x1117], 0
0x1404e8baf 488d0d1a315002 lea rcx, [rip + 0x250311a]
0x1404e8bb6 e8c5511900 call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
0x1404e8bbb 660f1000 movupd xmm0, xmmword ptr [rax]
0x1404e8bbf 8b4010 mov eax, dword ptr [rax + 0x10]
0x1404e8bc2 48c785500d000001000000 mov qword ptr [rbp + 0xd50], 1
0x1404e8bcd 488d0d9c2f5002 lea rcx, [rip + 0x2502f9c]
0x1404e8bd4 48898d580d0000 mov qword ptr [rbp + 0xd58], rcx
0x1404e8bdb 48c785600d000017000000 mov qword ptr [rbp + 0xd60], 0x17
0x1404e8be6 488d15c3305002 lea rdx, [rip + 0x25030c3]
0x1404e8bed 488995700d0000 mov qword ptr [rbp + 0xd70], rdx
0x1404e8bf4 48c785780d000002000000 mov qword ptr [rbp + 0xd78], 2
0x1404e8bff 488d9530050000 lea rdx, [rbp + 0x530]
0x1404e8c06 488995800d0000 mov qword ptr [rbp + 0xd80], rdx
0x1404e8c0d 48c785880d000002000000 mov qword ptr [rbp + 0xd88], 2
0x1404e8c18 488d95000b0000 lea rdx, [rbp + 0xb00]
0x1404e8c1f 488995900d0000 mov qword ptr [rbp + 0xd90], rdx
0x1404e8c26 48c785980d000002000000 mov qword ptr [rbp + 0xd98], 2
0x1404e8c31 48c785200d000000000000 mov qword ptr [rbp + 0xd20], 0
0x1404e8c3c 48898d280d0000 mov qword ptr [rbp + 0xd28], rcx
0x1404e8c43 48c785300d000017000000 mov qword ptr [rbp + 0xd30], 0x17
0x1404e8c4e 48c785380d000000000000 mov qword ptr [rbp + 0xd38], 0
0x1404e8c59 660f1185400d0000 movupd xmmword ptr [rbp + 0xd40], xmm0
0x1404e8c61 c785680d000001000000 mov dword ptr [rbp + 0xd68], 1
0x1404e8c6b 89856c0d0000 mov dword ptr [rbp + 0xd6c], eax
0x1404e8c71 c6851711000000 mov byte ptr [rbp + 0x1117], 0
0x1404e8c78 488d8d00100000 lea rcx, [rbp + 0x1000]
0x1404e8c7f 488d95200d0000 lea rdx, [rbp + 0xd20]
0x1404e8c86 e8a5168600 call 0x140d4a330 ; log::__private_api::impl$2::log
0x1404e8c8b f30f1085b4100000 movss xmm0, dword ptr [rbp + 0x10b4]
0x1404e8c93 0f57c9 xorps xmm1, xmm1
0x1404e8c96 0f2ec8 ucomiss xmm1, xmm0
0x1404e8c99 0f83d3190000 jae 0x1404ea672
0x1404e8c9f 895e5c mov dword ptr [rsi + 0x5c], ebx
0x1404e8ca2 f30f1107 movss dword ptr [rdi], xmm0
0x1404e8ca6 c6851711000000 mov byte ptr [rbp + 0x1117], 0
0x1404e8cad 4c89e1 mov rcx, r12
0x1404e8cb0 e83bea6f00 call 0x140be76f0 ; rhapsody::rg::resource_pool::ResourcePool::clear
0x1404e8cb5 e8864a8600 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
0x1404e8cba 4889c6 mov rsi, rax
0x1404e8cbd f60001 test byte ptr [rax], 1
0x1404e8cc0 7411 je 0x1404e8cd3
0x1404e8cc2 4889f1 mov rcx, rsi
0x1404e8cc5 4883c108 add rcx, 8
0x1404e8cc9 488b4608 mov rax, qword ptr [rsi + 8]
0x1404e8ccd 488b5610 mov rdx, qword ptr [rsi + 0x10]
0x1404e8cd1 eb22 jmp 0x1404e8cf5
0x1404e8cd3 c6851711000000 mov byte ptr [rbp + 0x1117], 0
0x1404e8cda e821c18700 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
0x1404e8cdf 4889f1 mov rcx, rsi
0x1404e8ce2 4883c108 add rcx, 8
0x1404e8ce6 48c70601000000 mov qword ptr [rsi], 1
0x1404e8ced 48894608 mov qword ptr [rsi + 8], rax
0x1404e8cf1 48895610 mov qword ptr [rsi + 0x10], rdx
0x1404e8cf5 4c8d4001 lea r8, [rax + 1]
0x1404e8cf9 4c8901 mov qword ptr [rcx], r8
0x1404e8cfc 0f1035ed2d5002 movups xmm6, xmmword ptr [rip + 0x2502ded]
0x1404e8d03 0f29b5100b0000 movaps xmmword ptr [rbp + 0xb10], xmm6
0x1404e8d0a 0f103dcf2d5002 movups xmm7, xmmword ptr [rip + 0x2502dcf]
0x1404e8d11 0f29bd000b0000 movaps xmmword ptr [rbp + 0xb00], xmm7
0x1404e8d18 488985200b0000 mov qword ptr [rbp + 0xb20], rax
0x1404e8d1f 488995280b0000 mov qword ptr [rbp + 0xb28], rdx
0x1404e8d26 e8154a8600 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
0x1404e8d2b 4889c6 mov rsi, rax
0x1404e8d2e f60001 test byte ptr [rax], 1
0x1404e8d31 7411 je 0x1404e8d44
0x1404e8d33 4889f0 mov rax, rsi
0x1404e8d36 4883c008 add rax, 8
0x1404e8d3a 488b5e08 mov rbx, qword ptr [rsi + 8]
0x1404e8d3e 488b7e10 mov rdi, qword ptr [rsi + 0x10]
0x1404e8d42 eb21 jmp 0x1404e8d65
0x1404e8d44 e8b7c08700 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
0x1404e8d49 4889c3 mov rbx, rax
0x1404e8d4c 4889d7 mov rdi, rdx
0x1404e8d4f 4889f0 mov rax, rsi
0x1404e8d52 4883c008 add rax, 8
0x1404e8d56 48c70601000000 mov qword ptr [rsi], 1
0x1404e8d5d 48895e08 mov qword ptr [rsi + 8], rbx
0x1404e8d61 48895610 mov qword ptr [rsi + 0x10], rdx
0x1404e8d65 488d4b01 lea rcx, [rbx + 1]
0x1404e8d69 488908 mov qword ptr [rax], rcx
0x1404e8d6c 0f2885000b0000 movaps xmm0, xmmword ptr [rbp + 0xb00]
0x1404e8d73 0f288d100b0000 movaps xmm1, xmmword ptr [rbp + 0xb10]
0x1404e8d7a 0f2895200b0000 movaps xmm2, xmmword ptr [rbp + 0xb20]
0x1404e8d81 0f2995400d0000 movaps xmmword ptr [rbp + 0xd40], xmm2
0x1404e8d88 0f29bd300b0000 movaps xmmword ptr [rbp + 0xb30], xmm7
0x1404e8d8f 0f29b5400b0000 movaps xmmword ptr [rbp + 0xb40], xmm6
0x1404e8d96 0f2985000b0000 movaps xmmword ptr [rbp + 0xb00], xmm0
0x1404e8d9d 0f298d100b0000 movaps xmmword ptr [rbp + 0xb10], xmm1
0x1404e8da4 0f2995200b0000 movaps xmmword ptr [rbp + 0xb20], xmm2
0x1404e8dab 498db760010000 lea rsi, [r15 + 0x160]
0x1404e8db2 4983bf6001000000 cmp qword ptr [r15 + 0x160], 0
0x1404e8dba 0f8484000000 je 0x1404e8e44
0x1404e8dc0 48899de8100000 mov qword ptr [rbp + 0x10e8], rbx
0x1404e8dc7 4889bdd0100000 mov qword ptr [rbp + 0x10d0], rdi
0x1404e8dce 498d9780010000 lea rdx, [r15 + 0x180]
0x1404e8dd5 41b868000000 mov r8d, 0x68
0x1404e8ddb 41b910000000 mov r9d, 0x10
0x1404e8de1 4889b5f0100000 mov qword ptr [rbp + 0x10f0], rsi
0x1404e8de8 4889f1 mov rcx, rsi
0x1404e8deb e8c040b6ff call 0x14004ceb0 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue>,alloc::alloc::Global>
0x1404e8df0 4c8bbde0100000 mov r15, qword ptr [rbp + 0x10e0]
0x1404e8df7 498b9798010000 mov rdx, qword ptr [r15 + 0x198]
0x1404e8dfe 4885d2 test rdx, rdx
0x1404e8e01 488bbdd0100000 mov rdi, qword ptr [rbp + 0x10d0]
0x1404e8e08 488b9de8100000 mov rbx, qword ptr [rbp + 0x10e8]
0x1404e8e0f 488bb5f0100000 mov rsi, qword ptr [rbp + 0x10f0]
0x1404e8e16 742c je 0x1404e8e44
0x1404e8e18 4889d0 mov rax, rdx
0x1404e8e1b 48c1e004 shl rax, 4
0x1404e8e1f 4801c2 add rdx, rax
0x1404e8e22 4883c221 add rdx, 0x21
0x1404e8e26 741c je 0x1404e8e44
0x1404e8e28 498b8f90010000 mov rcx, qword ptr [r15 + 0x190]
0x1404e8e2f 48f7d8 neg rax
0x1404e8e32 4801c1 add rcx, rax
0x1404e8e35 4883c1f0 add rcx, -0x10
0x1404e8e39 41b810000000 mov r8d, 0x10
0x1404e8e3f e8dcad1200 call 0x140613c20 ; __rust_dealloc
0x1404e8e44 0f2885400b0000 movaps xmm0, xmmword ptr [rbp + 0xb40]
0x1404e8e4b 0f114640 movups xmmword ptr [rsi + 0x40], xmm0
0x1404e8e4f 660f2885000b0000 movapd xmm0, xmmword ptr [rbp + 0xb00]
0x1404e8e57 0f288d100b0000 movaps xmm1, xmmword ptr [rbp + 0xb10]
0x1404e8e5e 660f2895200b0000 movapd xmm2, xmmword ptr [rbp + 0xb20]
0x1404e8e66 660f289d300b0000 movapd xmm3, xmmword ptr [rbp + 0xb30]
0x1404e8e6e 660f115e30 movupd xmmword ptr [rsi + 0x30], xmm3
0x1404e8e73 660f115620 movupd xmmword ptr [rsi + 0x20], xmm2
0x1404e8e78 0f114e10 movups xmmword ptr [rsi + 0x10], xmm1
0x1404e8e7c 660f1106 movupd xmmword ptr [rsi], xmm0
0x1404e8e80 49899fb0010000 mov qword ptr [r15 + 0x1b0], rbx
0x1404e8e87 4989bfb8010000 mov qword ptr [r15 + 0x1b8], rdi
0x1404e8e8e e8ad488600 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
0x1404e8e93 4889c6 mov rsi, rax
0x1404e8e96 f60001 test byte ptr [rax], 1
0x1404e8e99 7411 je 0x1404e8eac
0x1404e8e9b 4889f1 mov rcx, rsi
0x1404e8e9e 4883c108 add rcx, 8
0x1404e8ea2 488b4608 mov rax, qword ptr [rsi + 8]
0x1404e8ea6 488b5610 mov rdx, qword ptr [rsi + 0x10]
0x1404e8eaa eb22 jmp 0x1404e8ece
0x1404e8eac c6851711000000 mov byte ptr [rbp + 0x1117], 0
0x1404e8eb3 e848bf8700 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
0x1404e8eb8 4889f1 mov rcx, rsi
0x1404e8ebb 4883c108 add rcx, 8
0x1404e8ebf 48c70601000000 mov qword ptr [rsi], 1
0x1404e8ec6 48894608 mov qword ptr [rsi + 8], rax
0x1404e8eca 48895610 mov qword ptr [rsi + 0x10], rdx
0x1404e8ece 4c8d4001 lea r8, [rax + 1]
0x1404e8ed2 4c8901 mov qword ptr [rcx], r8
0x1404e8ed5 0f29b5100b0000 movaps xmmword ptr [rbp + 0xb10], xmm6
0x1404e8edc 0f29bd000b0000 movaps xmmword ptr [rbp + 0xb00], xmm7
0x1404e8ee3 488985200b0000 mov qword ptr [rbp + 0xb20], rax
0x1404e8eea 488995280b0000 mov qword ptr [rbp + 0xb28], rdx
0x1404e8ef1 e84a488600 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
0x1404e8ef6 4889c6 mov rsi, rax
0x1404e8ef9 f60001 test byte ptr [rax], 1
0x1404e8efc 7411 je 0x1404e8f0f
0x1404e8efe 4889f0 mov rax, rsi
0x1404e8f01 4883c008 add rax, 8
0x1404e8f05 488b5e08 mov rbx, qword ptr [rsi + 8]
0x1404e8f09 488b7e10 mov rdi, qword ptr [rsi + 0x10]
0x1404e8f0d eb21 jmp 0x1404e8f30
0x1404e8f0f e8ecbe8700 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
0x1404e8f14 4889c3 mov rbx, rax
0x1404e8f17 4889d7 mov rdi, rdx
0x1404e8f1a 4889f0 mov rax, rsi
0x1404e8f1d 4883c008 add rax, 8
0x1404e8f21 48c70601000000 mov qword ptr [rsi], 1
0x1404e8f28 48895e08 mov qword ptr [rsi + 8], rbx
0x1404e8f2c 48895610 mov qword ptr [rsi + 0x10], rdx
0x1404e8f30 488d4b01 lea rcx, [rbx + 1]
0x1404e8f34 488908 mov qword ptr [rax], rcx
0x1404e8f37 0f2885000b0000 movaps xmm0, xmmword ptr [rbp + 0xb00]
0x1404e8f3e 0f288d100b0000 movaps xmm1, xmmword ptr [rbp + 0xb10]
0x1404e8f45 0f2895200b0000 movaps xmm2, xmmword ptr [rbp + 0xb20]
0x1404e8f4c 0f2995400d0000 movaps xmmword ptr [rbp + 0xd40], xmm2
0x1404e8f53 0f29bd300b0000 movaps xmmword ptr [rbp + 0xb30], xmm7
0x1404e8f5a 0f29b5400b0000 movaps xmmword ptr [rbp + 0xb40], xmm6
0x1404e8f61 0f2985000b0000 movaps xmmword ptr [rbp + 0xb00], xmm0
0x1404e8f68 0f298d100b0000 movaps xmmword ptr [rbp + 0xb10], xmm1
0x1404e8f6f 0f2995200b0000 movaps xmmword ptr [rbp + 0xb20], xmm2
0x1404e8f76 498db750020000 lea rsi, [r15 + 0x250]
0x1404e8f7d 4983bf5002000000 cmp qword ptr [r15 + 0x250], 0
0x1404e8f85 0f8484000000 je 0x1404e900f
0x1404e8f8b 48899de8100000 mov qword ptr [rbp + 0x10e8], rbx
0x1404e8f92 4889bdd0100000 mov qword ptr [rbp + 0x10d0], rdi
0x1404e8f99 498d9770020000 lea rdx, [r15 + 0x270]
0x1404e8fa0 41b868000000 mov r8d, 0x68
0x1404e8fa6 41b910000000 mov r9d, 0x10
0x1404e8fac 4889b5f0100000 mov qword ptr [rbp + 0x10f0], rsi
0x1404e8fb3 4889f1 mov rcx, rsi
0x1404e8fb6 e8f53eb6ff call 0x14004ceb0 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue>,alloc::alloc::Global>
0x1404e8fbb 4c8bbde0100000 mov r15, qword ptr [rbp + 0x10e0]
0x1404e8fc2 498b9788020000 mov rdx, qword ptr [r15 + 0x288]
0x1404e8fc9 4885d2 test rdx, rdx
0x1404e8fcc 488bbdd0100000 mov rdi, qword ptr [rbp + 0x10d0]
0x1404e8fd3 488b9de8100000 mov rbx, qword ptr [rbp + 0x10e8]
0x1404e8fda 488bb5f0100000 mov rsi, qword ptr [rbp + 0x10f0]
0x1404e8fe1 742c je 0x1404e900f
0x1404e8fe3 4889d0 mov rax, rdx
0x1404e8fe6 48c1e004 shl rax, 4
0x1404e8fea 4801c2 add rdx, rax
0x1404e8fed 4883c221 add rdx, 0x21
0x1404e8ff1 741c je 0x1404e900f
0x1404e8ff3 498b8f80020000 mov rcx, qword ptr [r15 + 0x280]
0x1404e8ffa 48f7d8 neg rax
0x1404e8ffd 4801c1 add rcx, rax
0x1404e9000 4883c1f0 add rcx, -0x10
0x1404e9004 41b810000000 mov r8d, 0x10
0x1404e900a e811ac1200 call 0x140613c20 ; __rust_dealloc
0x1404e900f 0f2885400b0000 movaps xmm0, xmmword ptr [rbp + 0xb40]
0x1404e9016 0f114640 movups xmmword ptr [rsi + 0x40], xmm0
0x1404e901a 660f2885000b0000 movapd xmm0, xmmword ptr [rbp + 0xb00]
0x1404e9022 0f288d100b0000 movaps xmm1, xmmword ptr [rbp + 0xb10]
0x1404e9029 660f2895200b0000 movapd xmm2, xmmword ptr [rbp + 0xb20]
0x1404e9031 660f289d300b0000 movapd xmm3, xmmword ptr [rbp + 0xb30]
0x1404e9039 660f115e30 movupd xmmword ptr [rsi + 0x30], xmm3
0x1404e903e 660f115620 movupd xmmword ptr [rsi + 0x20], xmm2
0x1404e9043 0f114e10 movups xmmword ptr [rsi + 0x10], xmm1
0x1404e9047 660f1106 movupd xmmword ptr [rsi], xmm0
0x1404e904b 49899fa0020000 mov qword ptr [r15 + 0x2a0], rbx
0x1404e9052 4989bfa8020000 mov qword ptr [r15 + 0x2a8], rdi
0x1404e9059 c6851711000000 mov byte ptr [rbp + 0x1117], 0
0x1404e9060 488b8d00110000 mov rcx, qword ptr [rbp + 0x1100]
0x1404e9067 e8f4fd7000 call 0x140bf8e60 ; rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources
0x1404e906c c6851711000000 mov byte ptr [rbp + 0x1117], 0
0x1404e9073 48ba80eebc5f6a383533 movabs rdx, 0x3335386a5fbcee80
0x1404e907d 49b860ec651af3592a8f movabs r8, 0x8f2a59f31a65ec60
0x1404e9087 488b8d98100000 mov rcx, qword ptr [rbp + 0x1098]
0x1404e908e e85d46bfff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404e9093 4885c0 test rax, rax
0x1404e9096 4c8b85d8100000 mov r8, qword ptr [rbp + 0x10d8]
0x1404e909d 744b je 0x1404e90ea
0x1404e909f 49399018020000 cmp qword ptr [r8 + 0x218], rdx
0x1404e90a6 7642 jbe 0x1404e90ea
0x1404e90a8 498b8010020000 mov rax, qword ptr [r8 + 0x210]
0x1404e90af 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404e90b3 4885c9 test rcx, rcx
0x1404e90b6 7432 je 0x1404e90ea
0x1404e90b8 498b80e0010000 mov rax, qword ptr [r8 + 0x1e0]
0x1404e90bf 48f7d1 not rcx
0x1404e90c2 488d0c49 lea rcx, [rcx + rcx*2]
0x1404e90c6 48c1e105 shl rcx, 5
0x1404e90ca 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404e90d0 7418 je 0x1404e90ea
0x1404e90d2 4801c8 add rax, rcx
0x1404e90d5 488b4810 mov rcx, qword ptr [rax + 0x10]
0x1404e90d9 418b90b0050000 mov edx, dword ptr [r8 + 0x5b0]
0x1404e90e0 89505c mov dword ptr [rax + 0x5c], edx
0x1404e90e3 48c70102000000 mov qword ptr [rcx], 2
0x1404e90ea 488b05b7321403 mov rax, qword ptr [rip + 0x31432b7]
0x1404e90f1 488b00 mov rax, qword ptr [rax]
0x1404e90f4 4883f803 cmp rax, 3
0x1404e90f8 0f82cd000000 jb 0x1404e91cb
0x1404e90fe c6851711000000 mov byte ptr [rbp + 0x1117], 0
0x1404e9105 488d0d2c2c5002 lea rcx, [rip + 0x2502c2c]
0x1404e910c e86f4c1900 call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
0x1404e9111 660f1000 movupd xmm0, xmmword ptr [rax]
0x1404e9115 8b4010 mov eax, dword ptr [rax + 0x10]
0x1404e9118 48c785500d000003000000 mov qword ptr [rbp + 0xd50], 3
0x1404e9123 488d0d462a5002 lea rcx, [rip + 0x2502a46]
0x1404e912a 48898d580d0000 mov qword ptr [rbp + 0xd58], rcx
0x1404e9131 48c785600d000017000000 mov qword ptr [rbp + 0xd60], 0x17
0x1404e913c 488d15e52b5002 lea rdx, [rip + 0x2502be5]
0x1404e9143 488995700d0000 mov qword ptr [rbp + 0xd70], rdx
0x1404e914a 48c785780d000001000000 mov qword ptr [rbp + 0xd78], 1
0x1404e9155 48c785800d000008000000 mov qword ptr [rbp + 0xd80], 8
0x1404e9160 48c785200d000000000000 mov qword ptr [rbp + 0xd20], 0
0x1404e916b 0f57c9 xorps xmm1, xmm1
0x1404e916e 0f118d880d0000 movups xmmword ptr [rbp + 0xd88], xmm1
0x1404e9175 48898d280d0000 mov qword ptr [rbp + 0xd28], rcx
0x1404e917c 48c785300d000017000000 mov qword ptr [rbp + 0xd30], 0x17
0x1404e9187 48c785380d000000000000 mov qword ptr [rbp + 0xd38], 0
0x1404e9192 660f1185400d0000 movupd xmmword ptr [rbp + 0xd40], xmm0
0x1404e919a c785680d000001000000 mov dword ptr [rbp + 0xd68], 1
0x1404e91a4 89856c0d0000 mov dword ptr [rbp + 0xd6c], eax
0x1404e91aa c6851711000000 mov byte ptr [rbp + 0x1117], 0
0x1404e91b1 488d8d00100000 lea rcx, [rbp + 0x1000]
0x1404e91b8 488d95200d0000 lea rdx, [rbp + 0xd20]
0x1404e91bf e86c118600 call 0x140d4a330 ; log::__private_api::impl$2::log
0x1404e91c4 4c8b85d8100000 mov r8, qword ptr [rbp + 0x10d8]
0x1404e91cb c7850010000000000000 mov dword ptr [rbp + 0x1000], 0
0x1404e91d5 48c785000b000000000000 mov qword ptr [rbp + 0xb00], 0
0x1404e91e0 48c785080b000010000000 mov qword ptr [rbp + 0xb08], 0x10
0x1404e91eb 660f57c0 xorpd xmm0, xmm0
0x1404e91ef 660f1185100b0000 movupd xmmword ptr [rbp + 0xb10], xmm0
0x1404e91f7 48c785200b000010000000 mov qword ptr [rbp + 0xb20], 0x10
0x1404e9202 660f1185280b0000 movupd xmmword ptr [rbp + 0xb28], xmm0
0x1404e920a 48c785380b000010000000 mov qword ptr [rbp + 0xb38], 0x10
0x1404e9215 660f1185400b0000 movupd xmmword ptr [rbp + 0xb40], xmm0
0x1404e921d 48c785500b000010000000 mov qword ptr [rbp + 0xb50], 0x10
0x1404e9228 660f1185580b0000 movupd xmmword ptr [rbp + 0xb58], xmm0
0x1404e9230 48c785680b000010000000 mov qword ptr [rbp + 0xb68], 0x10
0x1404e923b 660f1185700b0000 movupd xmmword ptr [rbp + 0xb70], xmm0
0x1404e9243 48c785800b000010000000 mov qword ptr [rbp + 0xb80], 0x10
0x1404e924e 660f1185880b0000 movupd xmmword ptr [rbp + 0xb88], xmm0
0x1404e9256 48c785980b000010000000 mov qword ptr [rbp + 0xb98], 0x10
0x1404e9261 660f1185a00b0000 movupd xmmword ptr [rbp + 0xba0], xmm0
0x1404e9269 48c785b00b000010000000 mov qword ptr [rbp + 0xbb0], 0x10
0x1404e9274 660f1185b80b0000 movupd xmmword ptr [rbp + 0xbb8], xmm0
0x1404e927c 48c785c80b000010000000 mov qword ptr [rbp + 0xbc8], 0x10
0x1404e9287 660f1185d00b0000 movupd xmmword ptr [rbp + 0xbd0], xmm0
0x1404e928f 48c785e00b000010000000 mov qword ptr [rbp + 0xbe0], 0x10
0x1404e929a 660f1185e80b0000 movupd xmmword ptr [rbp + 0xbe8], xmm0
0x1404e92a2 48c785f80b000010000000 mov qword ptr [rbp + 0xbf8], 0x10
0x1404e92ad 660f1185000c0000 movupd xmmword ptr [rbp + 0xc00], xmm0
0x1404e92b5 48c785100c000010000000 mov qword ptr [rbp + 0xc10], 0x10
0x1404e92c0 660f1185180c0000 movupd xmmword ptr [rbp + 0xc18], xmm0
0x1404e92c8 48c785280c000010000000 mov qword ptr [rbp + 0xc28], 0x10
0x1404e92d3 660f1185300c0000 movupd xmmword ptr [rbp + 0xc30], xmm0
0x1404e92db 48c785400c000010000000 mov qword ptr [rbp + 0xc40], 0x10
0x1404e92e6 660f1185480c0000 movupd xmmword ptr [rbp + 0xc48], xmm0
0x1404e92ee 48c785580c000010000000 mov qword ptr [rbp + 0xc58], 0x10
0x1404e92f9 660f1185600c0000 movupd xmmword ptr [rbp + 0xc60], xmm0
0x1404e9301 48c785700c000010000000 mov qword ptr [rbp + 0xc70], 0x10
0x1404e930c 660f1185780c0000 movupd xmmword ptr [rbp + 0xc78], xmm0
0x1404e9314 48c785880c000010000000 mov qword ptr [rbp + 0xc88], 0x10
0x1404e931f 660f1185900c0000 movupd xmmword ptr [rbp + 0xc90], xmm0
0x1404e9327 48c785a00c000010000000 mov qword ptr [rbp + 0xca0], 0x10
0x1404e9332 660f1185a80c0000 movupd xmmword ptr [rbp + 0xca8], xmm0
0x1404e933a 48c785b80c000010000000 mov qword ptr [rbp + 0xcb8], 0x10
0x1404e9345 660f1185c00c0000 movupd xmmword ptr [rbp + 0xcc0], xmm0
0x1404e934d 48c785d00c000010000000 mov qword ptr [rbp + 0xcd0], 0x10
0x1404e9358 660f1185d80c0000 movupd xmmword ptr [rbp + 0xcd8], xmm0
0x1404e9360 48c785e80c000010000000 mov qword ptr [rbp + 0xce8], 0x10
0x1404e936b 48c785f00c000000000000 mov qword ptr [rbp + 0xcf0], 0
0x1404e9376 48b82000000020000000 movabs rax, 0x2000000020
0x1404e9380 488985c0090000 mov qword ptr [rbp + 0x9c0], rax
0x1404e9387 4c898530050000 mov qword ptr [rbp + 0x530], r8
0x1404e938e 4c8b8500110000 mov r8, qword ptr [rbp + 0x1100]
0x1404e9395 4c898538050000 mov qword ptr [rbp + 0x538], r8
0x1404e939c 488b85c8100000 mov rax, qword ptr [rbp + 0x10c8]
0x1404e93a3 48898540050000 mov qword ptr [rbp + 0x540], rax
0x1404e93aa 488d85c0090000 lea rax, [rbp + 0x9c0]
0x1404e93b1 48898548050000 mov qword ptr [rbp + 0x548], rax
0x1404e93b8 488d85000b0000 lea rax, [rbp + 0xb00]
0x1404e93bf 48898550050000 mov qword ptr [rbp + 0x550], rax
0x1404e93c6 488d8588070000 lea rax, [rbp + 0x788]
0x1404e93cd 48898558050000 mov qword ptr [rbp + 0x558], rax
0x1404e93d4 488d8500100000 lea rax, [rbp + 0x1000]
0x1404e93db 48898560050000 mov qword ptr [rbp + 0x560], rax
0x1404e93e2 48c7856805000000000000 mov qword ptr [rbp + 0x568], 0
0x1404e93ed 488d8530050000 lea rax, [rbp + 0x530]
0x1404e93f4 4889442420 mov qword ptr [rsp + 0x20], rax
0x1404e93f9 488d8d200d0000 lea rcx, [rbp + 0xd20]
0x1404e9400 488b95c0100000 mov rdx, qword ptr [rbp + 0x10c0]
0x1404e9407 4d89e1 mov r9, r12
0x1404e940a e8c188c7ff call 0x140161cd0 ; rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0>
0x1404e940f c6851711000000 mov byte ptr [rbp + 0x1117], 0
0x1404e9416 488d8d000b0000 lea rcx, [rbp + 0xb00]
0x1404e941d e83e0b1200 call 0x140609f60 ; core::ptr::drop_in_place<country_core::systems::render_loop::draw_passes::draw_lists::DrawLists>
0x1404e9422 31c0 xor eax, eax
0x1404e9424 483b85200d0000 cmp rax, qword ptr [rbp + 0xd20]
0x1404e942b 0f80b9120000 jo 0x1404ea6ea
0x1404e9431 488d8d000b0000 lea rcx, [rbp + 0xb00]
0x1404e9438 488d95200d0000 lea rdx, [rbp + 0xd20]
0x1404e943f 41b820020000 mov r8d, 0x220
0x1404e9445 e8b6a74302 call 0x142923c00 ; memcpy
0x1404e944a 488b05572f1403 mov rax, qword ptr [rip + 0x3142f57]
0x1404e9451 488b00 mov rax, qword ptr [rax]
0x1404e9454 4883f803 cmp rax, 3
0x1404e9458 0f82b8000000 jb 0x1404e9516
0x1404e945e 488d0d7b295002 lea rcx, [rip + 0x250297b]
0x1404e9465 e816491900 call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
0x1404e946a 660f1000 movupd xmm0, xmmword ptr [rax]
0x1404e946e 8b4010 mov eax, dword ptr [rax + 0x10]
0x1404e9471 48c785500d000003000000 mov qword ptr [rbp + 0xd50], 3
0x1404e947c 488d0ded265002 lea rcx, [rip + 0x25026ed]
0x1404e9483 48898d580d0000 mov qword ptr [rbp + 0xd58], rcx
0x1404e948a 48c785600d000017000000 mov qword ptr [rbp + 0xd60], 0x17
0x1404e9495 488d1534295002 lea rdx, [rip + 0x2502934]
0x1404e949c 488995700d0000 mov qword ptr [rbp + 0xd70], rdx
0x1404e94a3 48c785780d000001000000 mov qword ptr [rbp + 0xd78], 1
0x1404e94ae 48c785800d000008000000 mov qword ptr [rbp + 0xd80], 8
0x1404e94b9 48c785200d000000000000 mov qword ptr [rbp + 0xd20], 0
0x1404e94c4 0f57c9 xorps xmm1, xmm1
0x1404e94c7 0f118d880d0000 movups xmmword ptr [rbp + 0xd88], xmm1
0x1404e94ce 48898d280d0000 mov qword ptr [rbp + 0xd28], rcx
0x1404e94d5 48c785300d000017000000 mov qword ptr [rbp + 0xd30], 0x17
0x1404e94e0 48c785380d000000000000 mov qword ptr [rbp + 0xd38], 0
0x1404e94eb 660f1185400d0000 movupd xmmword ptr [rbp + 0xd40], xmm0
0x1404e94f3 c785680d000001000000 mov dword ptr [rbp + 0xd68], 1
0x1404e94fd 89856c0d0000 mov dword ptr [rbp + 0xd6c], eax
0x1404e9503 488d8d00100000 lea rcx, [rbp + 0x1000]
0x1404e950a 488d95200d0000 lea rdx, [rbp + 0xd20]
0x1404e9511 e81a0e8600 call 0x140d4a330 ; log::__private_api::impl$2::log
0x1404e9516 488d8d30050000 lea rcx, [rbp + 0x530]
0x1404e951d 488d95000b0000 lea rdx, [rbp + 0xb00]
0x1404e9524 41b818020000 mov r8d, 0x218
0x1404e952a e8d1a64302 call 0x142923c00 ; memcpy
0x1404e952f 448ba5180d0000 mov r12d, dword ptr [rbp + 0xd18]
0x1404e9536 488d8d000b0000 lea rcx, [rbp + 0xb00]
0x1404e953d 488d9530050000 lea rdx, [rbp + 0x530]
0x1404e9544 41b818020000 mov r8d, 0x218
0x1404e954a e8b1a64302 call 0x142923c00 ; memcpy
0x1404e954f 8b35931b1403 mov esi, dword ptr [rip + 0x3141b93]
0x1404e9555 81fe00ca9a3b cmp esi, 0x3b9aca00
0x1404e955b 7514 jne 0x1404e9571
0x1404e955d 31ff xor edi, edi
0x1404e955f 31f6 xor esi, esi
0x1404e9561 b002 mov al, 2
0x1404e9563 4422b5b8100000 and r14b, byte ptr [rbp + 0x10b8]
0x1404e956a 7541 jne 0x1404e95ad
0x1404e956c e93f010000 jmp 0x1404e96b0
0x1404e9571 488b1d38b61403 mov rbx, qword ptr [rip + 0x314b638]
0x1404e9578 c6851011000001 mov byte ptr [rbp + 0x1110], 1
0x1404e957f e85ca08700 call 0x140d635e0 ; std::time::Instant::now
0x1404e9584 c6851011000001 mov byte ptr [rbp + 0x1110], 1
0x1404e958b 4889c1 mov rcx, rax
0x1404e958e 4989d8 mov r8, rbx
0x1404e9591 4189f1 mov r9d, esi
0x1404e9594 e867a58700 call 0x140d63b00 ; std::time::impl$5::sub
0x1404e9599 4889c7 mov rdi, rax
0x1404e959c 89d6 mov esi, edx
0x1404e959e b002 mov al, 2
0x1404e95a0 4422b5b8100000 and r14b, byte ptr [rbp + 0x10b8]
0x1404e95a7 0f8403010000 je 0x1404e96b0
0x1404e95ad 498b4510 mov rax, qword ptr [r13 + 0x10]
0x1404e95b1 4885c0 test rax, rax
0x1404e95b4 7418 je 0x1404e95ce
0x1404e95b6 48ffc8 dec rax
0x1404e95b9 49894510 mov qword ptr [r13 + 0x10], rax
0x1404e95bd 498b4d08 mov rcx, qword ptr [r13 + 8]
0x1404e95c1 486bd058 imul rdx, rax, 0x58
0x1404e95c5 0fb644114c movzx eax, byte ptr [rcx + rdx + 0x4c]
0x1404e95ca 3c02 cmp al, 2
0x1404e95cc 7523 jne 0x1404e95f1
0x1404e95ce c6851011000001 mov byte ptr [rbp + 0x1110], 1
0x1404e95d5 488d8d200d0000 lea rcx, [rbp + 0xd20]
0x1404e95dc 488b9500110000 mov rdx, qword ptr [rbp + 0x1100]
0x1404e95e3 e8b85e7100 call 0x140bff4a0 ; rhapsody::raw::device::GraphicsDevice::create_gpu_profiler_output_buffer
0x1404e95e8 0fb6856c0d0000 movzx eax, byte ptr [rbp + 0xd6c]
0x1404e95ef eb59 jmp 0x1404e964a
0x1404e95f1 4801d1 add rcx, rdx
0x1404e95f4 0f10413c movups xmm0, xmmword ptr [rcx + 0x3c]
0x1404e95f8 0f11855c0d0000 movups xmmword ptr [rbp + 0xd5c], xmm0
0x1404e95ff 660f1001 movupd xmm0, xmmword ptr [rcx]
0x1404e9603 0f104910 movups xmm1, xmmword ptr [rcx + 0x10]
0x1404e9607 660f105120 movupd xmm2, xmmword ptr [rcx + 0x20]
0x1404e960c 660f105930 movupd xmm3, xmmword ptr [rcx + 0x30]
0x1404e9611 660f299d500d0000 movapd xmmword ptr [rbp + 0xd50], xmm3
0x1404e9619 660f2995400d0000 movapd xmmword ptr [rbp + 0xd40], xmm2
0x1404e9621 0f298d300d0000 movaps xmmword ptr [rbp + 0xd30], xmm1
0x1404e9628 660f2985200d0000 movapd xmmword ptr [rbp + 0xd20], xmm0
0x1404e9630 88856c0d0000 mov byte ptr [rbp + 0xd6c], al
0x1404e9636 488b514d mov rdx, qword ptr [rcx + 0x4d]
0x1404e963a 4889956d0d0000 mov qword ptr [rbp + 0xd6d], rdx
0x1404e9641 8b4954 mov ecx, dword ptr [rcx + 0x54]
0x1404e9644 898d740d0000 mov dword ptr [rbp + 0xd74], ecx
0x1404e964a 0f10855c0d0000 movups xmm0, xmmword ptr [rbp + 0xd5c]
0x1404e9651 0f11851c090000 movups xmmword ptr [rbp + 0x91c], xmm0
0x1404e9658 660f2885200d0000 movapd xmm0, xmmword ptr [rbp + 0xd20]
0x1404e9660 0f288d300d0000 movaps xmm1, xmmword ptr [rbp + 0xd30]
0x1404e9667 660f2895400d0000 movapd xmm2, xmmword ptr [rbp + 0xd40]
0x1404e966f 660f289d500d0000 movapd xmm3, xmmword ptr [rbp + 0xd50]
0x1404e9677 660f299d10090000 movapd xmmword ptr [rbp + 0x910], xmm3
0x1404e967f 660f299500090000 movapd xmmword ptr [rbp + 0x900], xmm2
0x1404e9687 0f298df0080000 movaps xmmword ptr [rbp + 0x8f0], xmm1
0x1404e968e 660f2985e0080000 movapd xmmword ptr [rbp + 0x8e0], xmm0
0x1404e9696 488b8d6d0d0000 mov rcx, qword ptr [rbp + 0xd6d]
0x1404e969d 48898df00f0000 mov qword ptr [rbp + 0xff0], rcx
0x1404e96a4 8b8d740d0000 mov ecx, dword ptr [rbp + 0xd74]
0x1404e96aa 898df70f0000 mov dword ptr [rbp + 0xff7], ecx
0x1404e96b0 4d8d4d18 lea r9, [r13 + 0x18]
0x1404e96b4 0fb68d48080000 movzx ecx, byte ptr [rbp + 0x848]
0x1404e96bb 0f10851c090000 movups xmm0, xmmword ptr [rbp + 0x91c]
0x1404e96c2 0f11854c030000 movups xmmword ptr [rbp + 0x34c], xmm0
0x1404e96c9 660f2885e0080000 movapd xmm0, xmmword ptr [rbp + 0x8e0]
0x1404e96d1 660f288df0080000 movapd xmm1, xmmword ptr [rbp + 0x8f0]
0x1404e96d9 660f289500090000 movapd xmm2, xmmword ptr [rbp + 0x900]
0x1404e96e1 660f289d10090000 movapd xmm3, xmmword ptr [rbp + 0x910]
0x1404e96e9 660f299d40030000 movapd xmmword ptr [rbp + 0x340], xmm3
0x1404e96f1 660f299530030000 movapd xmmword ptr [rbp + 0x330], xmm2
0x1404e96f9 660f298d20030000 movapd xmmword ptr [rbp + 0x320], xmm1
0x1404e9701 660f298510030000 movapd xmmword ptr [rbp + 0x310], xmm0
0x1404e9709 88855c030000 mov byte ptr [rbp + 0x35c], al
0x1404e970f 488b85f00f0000 mov rax, qword ptr [rbp + 0xff0]
0x1404e9716 4889855d030000 mov qword ptr [rbp + 0x35d], rax
0x1404e971d 8b85f70f0000 mov eax, dword ptr [rbp + 0xff7]
0x1404e9723 898564030000 mov dword ptr [rbp + 0x364], eax
0x1404e9729 c6851011000001 mov byte ptr [rbp + 0x1110], 1
0x1404e9730 488d8510030000 lea rax, [rbp + 0x310]
0x1404e9737 4889442430 mov qword ptr [rsp + 0x30], rax
0x1404e973c 884c2428 mov byte ptr [rsp + 0x28], cl
0x1404e9740 884c2420 mov byte ptr [rsp + 0x20], cl
0x1404e9744 488d8d200d0000 lea rcx, [rbp + 0xd20]
0x1404e974b 4c89fa mov rdx, r15
0x1404e974e 4c8b8500110000 mov r8, qword ptr [rbp + 0x1100]
0x1404e9755 e876147000 call 0x140beabd0 ; rhapsody::rg::executor::GraphExecutor::begin_frame
0x1404e975a 4489a5b8100000 mov dword ptr [rbp + 0x10b8], r12d
0x1404e9761 4889bdc8100000 mov qword ptr [rbp + 0x10c8], rdi
0x1404e9768 89b5c0100000 mov dword ptr [rbp + 0x10c0], esi
0x1404e976e 488d8d10030000 lea rcx, [rbp + 0x310]
0x1404e9775 4c8da5200d0000 lea r12, [rbp + 0xd20]
0x1404e977c 41b890010000 mov r8d, 0x190
0x1404e9782 4c89e2 mov rdx, r12
0x1404e9785 e876a44302 call 0x142923c00 ; memcpy
0x1404e978a 488d95b00e0000 lea rdx, [rbp + 0xeb0]
0x1404e9791 488d9d38050000 lea rbx, [rbp + 0x538]
0x1404e9798 41b808010000 mov r8d, 0x108
0x1404e979e 4889d9 mov rcx, rbx
0x1404e97a1 e85aa44302 call 0x142923c00 ; memcpy
0x1404e97a6 48c7853005000000000000 mov qword ptr [rbp + 0x530], 0
0x1404e97b1 488dbd98020000 lea rdi, [rbp + 0x298]
0x1404e97b8 31c9 xor ecx, ecx
0x1404e97ba 4c8dbd90020000 lea r15, [rbp + 0x290]
0x1404e97c1 4c8db5c0090000 lea r14, [rbp + 0x9c0]
0x1404e97c8 eb0d jmp 0x1404e97d7
0x1404e97ca 660f1f440000 nop word ptr [rax + rax]
0x1404e97d0 488b8d30050000 mov rcx, qword ptr [rbp + 0x530]
0x1404e97d7 8b8538060000 mov eax, dword ptr [rbp + 0x638]
0x1404e97dd 4839c1 cmp rcx, rax
0x1404e97e0 0f841d020000 je 0x1404e9a03
0x1404e97e6 4889c8 mov rax, rcx
0x1404e97e9 48ffc1 inc rcx
0x1404e97ec 48898d30050000 mov qword ptr [rbp + 0x530], rcx
0x1404e97f3 48c1e007 shl rax, 7
0x1404e97f7 488b1403 mov rdx, qword ptr [rbx + rax]
0x1404e97fb 4989d0 mov r8, rdx
0x1404e97fe 49f7d8 neg r8
0x1404e9801 70d4 jo 0x1404e97d7
0x1404e9803 48b90100000000000080 movabs rcx, 0x8000000000000001
0x1404e980d 4839ca cmp rdx, rcx
0x1404e9810 0f84ed010000 je 0x1404e9a03
0x1404e9816 4801d8 add rax, rbx
0x1404e9819 48899590020000 mov qword ptr [rbp + 0x290], rdx
0x1404e9820 488b4878 mov rcx, qword ptr [rax + 0x78]
0x1404e9824 48894f70 mov qword ptr [rdi + 0x70], rcx
0x1404e9828 0f104068 movups xmm0, xmmword ptr [rax + 0x68]
0x1404e982c 0f114760 movups xmmword ptr [rdi + 0x60], xmm0
0x1404e9830 0f104058 movups xmm0, xmmword ptr [rax + 0x58]
0x1404e9834 0f114750 movups xmmword ptr [rdi + 0x50], xmm0
0x1404e9838 0f104048 movups xmm0, xmmword ptr [rax + 0x48]
0x1404e983c 0f114740 movups xmmword ptr [rdi + 0x40], xmm0
0x1404e9840 660f104008 movupd xmm0, xmmword ptr [rax + 8]
0x1404e9845 660f104818 movupd xmm1, xmmword ptr [rax + 0x18]
0x1404e984a 660f105028 movupd xmm2, xmmword ptr [rax + 0x28]
0x1404e984f 660f105838 movupd xmm3, xmmword ptr [rax + 0x38]
0x1404e9854 660f115f30 movupd xmmword ptr [rdi + 0x30], xmm3
0x1404e9859 660f115720 movupd xmmword ptr [rdi + 0x20], xmm2
0x1404e985e 660f114f10 movupd xmmword ptr [rdi + 0x10], xmm1
0x1404e9863 660f1107 movupd xmmword ptr [rdi], xmm0
0x1404e9867 4c89e1 mov rcx, r12
0x1404e986a 4c89fa mov rdx, r15
0x1404e986d e87eb4daff call 0x140294cf0 ; gpu_profiler::backend::ash::ProfilerSubmittedFrame::retrieve_results<rhapsody::raw::profiler::ProfilerBuffer>
0x1404e9872 488b85500d0000 mov rax, qword ptr [rbp + 0xd50]
0x1404e9879 48898530100000 mov qword ptr [rbp + 0x1030], rax
0x1404e9880 0f1085200d0000 movups xmm0, xmmword ptr [rbp + 0xd20]
0x1404e9887 0f108d300d0000 movups xmm1, xmmword ptr [rbp + 0xd30]
0x1404e988e 0f1095400d0000 movups xmm2, xmmword ptr [rbp + 0xd40]
0x1404e9895 0f299520100000 movaps xmmword ptr [rbp + 0x1020], xmm2
0x1404e989c 0f298d10100000 movaps xmmword ptr [rbp + 0x1010], xmm1
0x1404e98a3 0f298500100000 movaps xmmword ptr [rbp + 0x1000], xmm0
0x1404e98aa 410f10442438 movups xmm0, xmmword ptr [r12 + 0x38]
0x1404e98b0 66410f104c2448 movupd xmm1, xmmword ptr [r12 + 0x48]
0x1404e98b7 66410f10542458 movupd xmm2, xmmword ptr [r12 + 0x58]
0x1404e98be 66410f105c2468 movupd xmm3, xmmword ptr [r12 + 0x68]
0x1404e98c5 0f298560090000 movaps xmmword ptr [rbp + 0x960], xmm0
0x1404e98cc 660f298d70090000 movapd xmmword ptr [rbp + 0x970], xmm1
0x1404e98d4 660f299580090000 movapd xmmword ptr [rbp + 0x980], xmm2
0x1404e98dc 660f299d90090000 movapd xmmword ptr [rbp + 0x990], xmm3
0x1404e98e4 66410f10442478 movupd xmm0, xmmword ptr [r12 + 0x78]
0x1404e98eb 660f2985a0090000 movapd xmmword ptr [rbp + 0x9a0], xmm0
0x1404e98f3 498b842488000000 mov rax, qword ptr [r12 + 0x88]
0x1404e98fb 488985b0090000 mov qword ptr [rbp + 0x9b0], rax
0x1404e9902 498b7510 mov rsi, qword ptr [r13 + 0x10]
0x1404e9906 493b7500 cmp rsi, qword ptr [r13]
0x1404e990a 750f jne 0x1404e991b
0x1404e990c 4c89e9 mov rcx, r13
0x1404e990f 488d1552255002 lea rdx, [rip + 0x2502552]
0x1404e9916 e815960700 call 0x140562f30 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<system_onboarding::hint_orchestrator::DisplayHintRequest>,alloc::alloc::Global>
0x1404e991b 498b4508 mov rax, qword ptr [r13 + 8]
0x1404e991f 486bce58 imul rcx, rsi, 0x58
0x1404e9923 0f288560090000 movaps xmm0, xmmword ptr [rbp + 0x960]
0x1404e992a 0f288d70090000 movaps xmm1, xmmword ptr [rbp + 0x970]
0x1404e9931 0f289580090000 movaps xmm2, xmmword ptr [rbp + 0x980]
0x1404e9938 660f289d90090000 movapd xmm3, xmmword ptr [rbp + 0x990]
0x1404e9940 660f28a5a0090000 movapd xmm4, xmmword ptr [rbp + 0x9a0]
0x1404e9948 488b95b0090000 mov rdx, qword ptr [rbp + 0x9b0]
0x1404e994f 4889540850 mov qword ptr [rax + rcx + 0x50], rdx
0x1404e9954 660f11640840 movupd xmmword ptr [rax + rcx + 0x40], xmm4
0x1404e995a 660f115c0830 movupd xmmword ptr [rax + rcx + 0x30], xmm3
0x1404e9960 0f11540820 movups xmmword ptr [rax + rcx + 0x20], xmm2
0x1404e9965 0f114c0810 movups xmmword ptr [rax + rcx + 0x10], xmm1
0x1404e996a 0f110408 movups xmmword ptr [rax + rcx], xmm0
0x1404e996e 48ffc6 inc rsi
0x1404e9971 49897510 mov qword ptr [r13 + 0x10], rsi
0x1404e9975 488b8530100000 mov rax, qword ptr [rbp + 0x1030]
0x1404e997c 488985500d0000 mov qword ptr [rbp + 0xd50], rax
0x1404e9983 660f288500100000 movapd xmm0, xmmword ptr [rbp + 0x1000]
0x1404e998b 660f288d10100000 movapd xmm1, xmmword ptr [rbp + 0x1010]
0x1404e9993 660f289520100000 movapd xmm2, xmmword ptr [rbp + 0x1020]
0x1404e999b 660f2995400d0000 movapd xmmword ptr [rbp + 0xd40], xmm2
0x1404e99a3 660f298d300d0000 movapd xmmword ptr [rbp + 0xd30], xmm1
0x1404e99ab 660f2985200d0000 movapd xmmword ptr [rbp + 0xd20], xmm0
0x1404e99b3 c6850d11000000 mov byte ptr [rbp + 0x110d], 0
0x1404e99ba 4c89f1 mov rcx, r14
0x1404e99bd 4c89e2 mov rdx, r12
0x1404e99c0 e8eba26e00 call 0x140bd3cb0 ; rhapsody::raw::profiler::TimedFrame::new
0x1404e99c5 4c89f1 mov rcx, r14
0x1404e99c8 488b95a8100000 mov rdx, qword ptr [rbp + 0x10a8]
0x1404e99cf e85ca66e00 call 0x140bd4030 ; rhapsody::raw::profiler::TimedFrame::send_to_puffin
0x1404e99d4 488b85c0090000 mov rax, qword ptr [rbp + 0x9c0]
0x1404e99db 4885c0 test rax, rax
0x1404e99de 0f84ecfdffff je 0x1404e97d0
0x1404e99e4 488b8dc8090000 mov rcx, qword ptr [rbp + 0x9c8]
0x1404e99eb 48c1e003 shl rax, 3
0x1404e99ef 488d1440 lea rdx, [rax + rax*2]
0x1404e99f3 41b808000000 mov r8d, 8
0x1404e99f9 e822a21200 call 0x140613c20 ; __rust_dealloc
0x1404e99fe e9cdfdffff jmp 0x1404e97d0
0x1404e9a03 c6851611000001 mov byte ptr [rbp + 0x1116], 1
0x1404e9a0a c6851511000001 mov byte ptr [rbp + 0x1115], 1
0x1404e9a11 488d8d30050000 lea rcx, [rbp + 0x530]
0x1404e9a18 e8737a0e00 call 0x1405d1490 ; core::ptr::drop_in_place<arrayvec::arrayvec::IntoIter<rhapsody::rg::executor::FinishedFrame,2> >
0x1404e9a1d 488bbde0100000 mov rdi, qword ptr [rbp + 0x10e0]
0x1404e9a24 488b9dd8100000 mov rbx, qword ptr [rbp + 0x10d8]
0x1404e9a2b 4c8ba5f8110000 mov r12, qword ptr [rbp + 0x11f8]
0x1404e9a32 c6851611000001 mov byte ptr [rbp + 0x1116], 1
0x1404e9a39 c6851511000001 mov byte ptr [rbp + 0x1115], 1
0x1404e9a40 e89b9b8700 call 0x140d635e0 ; std::time::Instant::now
0x1404e9a45 488985d00a0000 mov qword ptr [rbp + 0xad0], rax
0x1404e9a4c 8995d80a0000 mov dword ptr [rbp + 0xad8], edx
0x1404e9a52 488d8d200d0000 lea rcx, [rbp + 0xd20]
0x1404e9a59 488d95000b0000 lea rdx, [rbp + 0xb00]
0x1404e9a60 41b818020000 mov r8d, 0x218
0x1404e9a66 e895a14302 call 0x142923c00 ; memcpy
0x1404e9a6b c6852810000002 mov byte ptr [rbp + 0x1028], 2
0x1404e9a72 c6850c11000001 mov byte ptr [rbp + 0x110c], 1
0x1404e9a79 488db530050000 lea rsi, [rbp + 0x530]
0x1404e9a80 4889f1 mov rcx, rsi
0x1404e9a83 4889da mov rdx, rbx
0x1404e9a86 e885f84100 call 0x140909310 ; country_core::render::pass_builder::pass_resources::PassResources::new_from
0x1404e9a8b c6850c11000000 mov byte ptr [rbp + 0x110c], 0
0x1404e9a92 4889742420 mov qword ptr [rsp + 0x20], rsi
0x1404e9a97 488d4db8 lea rcx, [rbp - 0x48]
0x1404e9a9b 488d9510030000 lea rdx, [rbp + 0x310]
0x1404e9aa2 4c8d85200d0000 lea r8, [rbp + 0xd20]
0x1404e9aa9 4c8d8d00100000 lea r9, [rbp + 0x1000]
0x1404e9ab0 e8db06c1ff call 0x1400fa190 ; rhapsody::rg::executor::GraphExecutionRecorder::execute_compiled<country_core::render::pass_builder::pass_resources::PassResources>
0x1404e9ab5 4c8b8590040000 mov r8, qword ptr [rbp + 0x490]
0x1404e9abc c6851611000001 mov byte ptr [rbp + 0x1116], 1
0x1404e9ac3 c6851511000000 mov byte ptr [rbp + 0x1115], 0
0x1404e9aca 488d8d200d0000 lea rcx, [rbp + 0xd20]
0x1404e9ad1 488d55b8 lea rdx, [rbp - 0x48]
0x1404e9ad5 e826027000 call 0x140be9d00 ; rhapsody::rg::executor::ExecutedRenderGraph::retire
0x1404e9ada 488d8d30050000 lea rcx, [rbp + 0x530]
0x1404e9ae1 488d95200d0000 lea rdx, [rbp + 0xd20]
0x1404e9ae8 41b890000000 mov r8d, 0x90
0x1404e9aee e80da14302 call 0x142923c00 ; memcpy
0x1404e9af3 660f1085b00d0000 movupd xmm0, xmmword ptr [rbp + 0xdb0]
0x1404e9afb 660f108dc00d0000 movupd xmm1, xmmword ptr [rbp + 0xdc0]
0x1404e9b03 660f1095d00d0000 movupd xmm2, xmmword ptr [rbp + 0xdd0]
0x1404e9b0b 660f2995c00a0000 movapd xmmword ptr [rbp + 0xac0], xmm2
0x1404e9b13 660f298db00a0000 movapd xmmword ptr [rbp + 0xab0], xmm1
0x1404e9b1b 660f2985a00a0000 movapd xmmword ptr [rbp + 0xaa0], xmm0
0x1404e9b23 c6851211000001 mov byte ptr [rbp + 0x1112], 1
0x1404e9b2a c6851111000001 mov byte ptr [rbp + 0x1111], 1
0x1404e9b31 488d8d200d0000 lea rcx, [rbp + 0xd20]
0x1404e9b38 488d95a00a0000 lea rdx, [rbp + 0xaa0]
0x1404e9b3f 448b85b8100000 mov r8d, dword ptr [rbp + 0x10b8]
0x1404e9b46 e835017000 call 0x140be9c80 ; rhapsody::rg::executor::ExportedRenderGraphResources::take_resource
0x1404e9b4b c6851211000001 mov byte ptr [rbp + 0x1112], 1
0x1404e9b52 c6851111000001 mov byte ptr [rbp + 0x1111], 1
0x1404e9b59 488d8d280a0000 lea rcx, [rbp + 0xa28]
0x1404e9b60 488d95200d0000 lea rdx, [rbp + 0xd20]
0x1404e9b67 e884c86e00 call 0x140bd63f0 ; rhapsody::rg::import_export::impl$0::from_owned_rg_resource
0x1404e9b6c 488db7d0000000 lea rsi, [rdi + 0xd0]
0x1404e9b73 488d8d200d0000 lea rcx, [rbp + 0xd20]
0x1404e9b7a 41b890000000 mov r8d, 0x90
0x1404e9b80 4889f2 mov rdx, rsi
0x1404e9b83 e878a04302 call 0x142923c00 ; memcpy
0x1404e9b88 488d9530050000 lea rdx, [rbp + 0x530]
0x1404e9b8f 41b890000000 mov r8d, 0x90
0x1404e9b95 4889f1 mov rcx, rsi
0x1404e9b98 e863a04302 call 0x142923c00 ; memcpy
0x1404e9b9d 31c0 xor eax, eax
0x1404e9b9f 483b85200d0000 cmp rax, qword ptr [rbp + 0xd20]
0x1404e9ba6 701a jo 0x1404e9bc2
0x1404e9ba8 c6851411000001 mov byte ptr [rbp + 0x1114], 1
0x1404e9baf c6851311000001 mov byte ptr [rbp + 0x1113], 1
0x1404e9bb6 488d8d200d0000 lea rcx, [rbp + 0xd20]
0x1404e9bbd e84e561000 call 0x1405ef210 ; core::ptr::drop_in_place<rhapsody::rg::graph::RenderGraph>
0x1404e9bc2 4d85e4 test r12, r12
0x1404e9bc5 0f94c0 sete al
0x1404e9bc8 0fb68dfc100000 movzx ecx, byte ptr [rbp + 0x10fc]
0x1404e9bcf 80f101 xor cl, 1
0x1404e9bd2 41b601 mov r14b, 1
0x1404e9bd5 08c1 or cl, al
0x1404e9bd7 752d jne 0x1404e9c06
0x1404e9bd9 c6851411000001 mov byte ptr [rbp + 0x1114], 1
0x1404e9be0 c6851311000001 mov byte ptr [rbp + 0x1113], 1
0x1404e9be7 488d8dc0090000 lea rcx, [rbp + 0x9c0]
0x1404e9bee 488b9500110000 mov rdx, qword ptr [rbp + 0x1100]
0x1404e9bf5 4d89e0 mov r8, r12
0x1404e9bf8 e8d33d7100 call 0x140bfd9d0 ; rhapsody::raw::device::GraphicsDevice::validate_swapchain_and_acquire_image
0x1404e9bfd 80bd1e0a000007 cmp byte ptr [rbp + 0xa1e], 7
0x1404e9c04 7508 jne 0x1404e9c0e
0x1404e9c06 4531ff xor r15d, r15d
0x1404e9c09 e908030000 jmp 0x1404e9f16
0x1404e9c0e 488b85200a0000 mov rax, qword ptr [rbp + 0xa20]
0x1404e9c15 48898560100000 mov qword ptr [rbp + 0x1060], rax
0x1404e9c1c 0f1085100a0000 movups xmm0, xmmword ptr [rbp + 0xa10]
0x1404e9c23 0f298550100000 movaps xmmword ptr [rbp + 0x1050], xmm0
0x1404e9c2a 0f1085000a0000 movups xmm0, xmmword ptr [rbp + 0xa00]
0x1404e9c31 0f298540100000 movaps xmmword ptr [rbp + 0x1040], xmm0
0x1404e9c38 660f1085c0090000 movupd xmm0, xmmword ptr [rbp + 0x9c0]
0x1404e9c40 660f108dd0090000 movupd xmm1, xmmword ptr [rbp + 0x9d0]
0x1404e9c48 660f1095e0090000 movupd xmm2, xmmword ptr [rbp + 0x9e0]
0x1404e9c50 660f109df0090000 movupd xmm3, xmmword ptr [rbp + 0x9f0]
0x1404e9c58 660f299d30100000 movapd xmmword ptr [rbp + 0x1030], xmm3
0x1404e9c60 660f299520100000 movapd xmmword ptr [rbp + 0x1020], xmm2
0x1404e9c68 660f298d10100000 movapd xmmword ptr [rbp + 0x1010], xmm1
0x1404e9c70 660f298500100000 movapd xmmword ptr [rbp + 0x1000], xmm0
0x1404e9c78 c6850f11000001 mov byte ptr [rbp + 0x110f], 1
0x1404e9c7f c6850e11000001 mov byte ptr [rbp + 0x110e], 1
0x1404e9c86 488d9d200d0000 lea rbx, [rbp + 0xd20]
0x1404e9c8d 488db500100000 lea rsi, [rbp + 0x1000]
0x1404e9c94 4889d9 mov rcx, rbx
0x1404e9c97 4889f2 mov rdx, rsi
0x1404e9c9a e881547200 call 0x140c0f120 ; rhapsody::raw::swapchain::TrackedSwapchainTextureLease::image
0x1404e9c9f 488bbd200d0000 mov rdi, qword ptr [rbp + 0xd20]
0x1404e9ca6 0f1085480d0000 movups xmm0, xmmword ptr [rbp + 0xd48]
0x1404e9cad 0f298550090000 movaps xmmword ptr [rbp + 0x950], xmm0
0x1404e9cb4 0f1085380d0000 movups xmm0, xmmword ptr [rbp + 0xd38]
0x1404e9cbb 0f298540090000 movaps xmmword ptr [rbp + 0x940], xmm0
0x1404e9cc2 0f1085280d0000 movups xmm0, xmmword ptr [rbp + 0xd28]
0x1404e9cc9 0f298530090000 movaps xmmword ptr [rbp + 0x930], xmm0
0x1404e9cd0 488b85580a0000 mov rax, qword ptr [rbp + 0xa58]
0x1404e9cd7 488985900a0000 mov qword ptr [rbp + 0xa90], rax
0x1404e9cde 660f1085280a0000 movupd xmm0, xmmword ptr [rbp + 0xa28]
0x1404e9ce6 660f108d380a0000 movupd xmm1, xmmword ptr [rbp + 0xa38]
0x1404e9cee 660f1095480a0000 movupd xmm2, xmmword ptr [rbp + 0xa48]
0x1404e9cf6 660f2995800a0000 movapd xmmword ptr [rbp + 0xa80], xmm2
0x1404e9cfe 660f298d700a0000 movapd xmmword ptr [rbp + 0xa70], xmm1
0x1404e9d06 660f2985600a0000 movapd xmmword ptr [rbp + 0xa60], xmm0
0x1404e9d0e 4c8bb588040000 mov r14, qword ptr [rbp + 0x488]
0x1404e9d15 c6850b11000001 mov byte ptr [rbp + 0x110b], 1
0x1404e9d1c 488d8d50070000 lea rcx, [rbp + 0x750]
0x1404e9d23 4c8bbdd8100000 mov r15, qword ptr [rbp + 0x10d8]
0x1404e9d2a 4c89fa mov rdx, r15
0x1404e9d2d e8def54100 call 0x140909310 ; country_core::render::pass_builder::pass_resources::PassResources::new_from
0x1404e9d32 488b85e0100000 mov rax, qword ptr [rbp + 0x10e0]
0x1404e9d39 4c8d88c0010000 lea r9, [rax + 0x1c0]
0x1404e9d40 4983c578 add r13, 0x78
0x1404e9d44 0f288530090000 movaps xmm0, xmmword ptr [rbp + 0x930]
0x1404e9d4b 0f288d40090000 movaps xmm1, xmmword ptr [rbp + 0x940]
0x1404e9d52 0f289550090000 movaps xmm2, xmmword ptr [rbp + 0x950]
0x1404e9d59 0f2995e00f0000 movaps xmmword ptr [rbp + 0xfe0], xmm2
0x1404e9d60 0f298dd00f0000 movaps xmmword ptr [rbp + 0xfd0], xmm1
0x1404e9d67 0f2985c00f0000 movaps xmmword ptr [rbp + 0xfc0], xmm0
0x1404e9d6e 4c89bd200d0000 mov qword ptr [rbp + 0xd20], r15
0x1404e9d75 4c89b5280d0000 mov qword ptr [rbp + 0xd28], r14
0x1404e9d7c 4c89ad300d0000 mov qword ptr [rbp + 0xd30], r13
0x1404e9d83 660f2885600a0000 movapd xmm0, xmmword ptr [rbp + 0xa60]
0x1404e9d8b 660f288d700a0000 movapd xmm1, xmmword ptr [rbp + 0xa70]
0x1404e9d93 660f2895800a0000 movapd xmm2, xmmword ptr [rbp + 0xa80]
0x1404e9d9b 660f1185480d0000 movupd xmmword ptr [rbp + 0xd48], xmm0
0x1404e9da3 660f118d580d0000 movupd xmmword ptr [rbp + 0xd58], xmm1
0x1404e9dab 660f1195680d0000 movupd xmmword ptr [rbp + 0xd68], xmm2
0x1404e9db3 488b85900a0000 mov rax, qword ptr [rbp + 0xa90]
0x1404e9dba 488985780d0000 mov qword ptr [rbp + 0xd78], rax
0x1404e9dc1 4889bd380d0000 mov qword ptr [rbp + 0xd38], rdi
0x1404e9dc8 488d8588070000 lea rax, [rbp + 0x788]
0x1404e9dcf 488985400d0000 mov qword ptr [rbp + 0xd40], rax
0x1404e9dd6 c6850b11000000 mov byte ptr [rbp + 0x110b], 0
0x1404e9ddd 48895c2428 mov qword ptr [rsp + 0x28], rbx
0x1404e9de2 488d85c00f0000 lea rax, [rbp + 0xfc0]
0x1404e9de9 4889442420 mov qword ptr [rsp + 0x20], rax
0x1404e9dee 488d8db0080000 lea rcx, [rbp + 0x8b0]
0x1404e9df5 488d9510030000 lea rdx, [rbp + 0x310]
0x1404e9dfc 488dbd50070000 lea rdi, [rbp + 0x750]
0x1404e9e03 4989f8 mov r8, rdi
0x1404e9e06 e8a517c1ff call 0x1400fb5b0 ; rhapsody::rg::executor::GraphExecutionRecorder::execute<tuple$<>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0>
0x1404e9e0b 4883bdb008000000 cmp qword ptr [rbp + 0x8b0], 0
0x1404e9e13 0f8408090000 je 0x1404ea721
0x1404e9e19 660f1085b0080000 movupd xmm0, xmmword ptr [rbp + 0x8b0]
0x1404e9e21 660f108dc0080000 movupd xmm1, xmmword ptr [rbp + 0x8c0]
0x1404e9e29 660f1095d0080000 movupd xmm2, xmmword ptr [rbp + 0x8d0]
0x1404e9e31 660f2985c00f0000 movapd xmmword ptr [rbp + 0xfc0], xmm0
0x1404e9e39 660f2995e00f0000 movapd xmmword ptr [rbp + 0xfe0], xmm2
0x1404e9e41 660f298dd00f0000 movapd xmmword ptr [rbp + 0xfd0], xmm1
0x1404e9e49 4883bdc00f000000 cmp qword ptr [rbp + 0xfc0], 0
0x1404e9e51 0f84d5080000 je 0x1404ea72c
0x1404e9e57 488b8560100000 mov rax, qword ptr [rbp + 0x1060]
0x1404e9e5e 488985800d0000 mov qword ptr [rbp + 0xd80], rax
0x1404e9e65 0f288550100000 movaps xmm0, xmmword ptr [rbp + 0x1050]
0x1404e9e6c 0f2985700d0000 movaps xmmword ptr [rbp + 0xd70], xmm0
0x1404e9e73 0f288540100000 movaps xmm0, xmmword ptr [rbp + 0x1040]
0x1404e9e7a 0f2985600d0000 movaps xmmword ptr [rbp + 0xd60], xmm0
0x1404e9e81 660f288500100000 movapd xmm0, xmmword ptr [rbp + 0x1000]
0x1404e9e89 660f288d10100000 movapd xmm1, xmmword ptr [rbp + 0x1010]
0x1404e9e91 660f289520100000 movapd xmm2, xmmword ptr [rbp + 0x1020]
0x1404e9e99 660f289d30100000 movapd xmm3, xmmword ptr [rbp + 0x1030]
0x1404e9ea1 660f299d500d0000 movapd xmmword ptr [rbp + 0xd50], xmm3
0x1404e9ea9 660f2995400d0000 movapd xmmword ptr [rbp + 0xd40], xmm2
0x1404e9eb1 660f298d300d0000 movapd xmmword ptr [rbp + 0xd30], xmm1
0x1404e9eb9 660f2985200d0000 movapd xmmword ptr [rbp + 0xd20], xmm0
0x1404e9ec1 c6850a11000000 mov byte ptr [rbp + 0x110a], 0
0x1404e9ec8 488d8d200d0000 lea rcx, [rbp + 0xd20]
0x1404e9ecf e89c527200 call 0x140c0f170 ; rhapsody::raw::swapchain::TrackedSwapchainTextureLease::into_present_source
0x1404e9ed4 4889c7 mov rdi, rax
0x1404e9ed7 89d6 mov esi, edx
0x1404e9ed9 4883bdc00f000000 cmp qword ptr [rbp + 0xfc0], 0
0x1404e9ee1 742d je 0x1404e9f10
0x1404e9ee3 488d95e00f0000 lea rdx, [rbp + 0xfe0]
0x1404e9eea c6850f11000000 mov byte ptr [rbp + 0x110f], 0
0x1404e9ef1 c6850e11000000 mov byte ptr [rbp + 0x110e], 0
0x1404e9ef8 488d8dc00f0000 lea rcx, [rbp + 0xfc0]
0x1404e9eff 41b848000000 mov r8d, 0x48
0x1404e9f05 41b910000000 mov r9d, 0x10
0x1404e9f0b e88057b6ff call 0x14004f690 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<rhapsody::rg::resource::ResourceHandle,enum2$<rhapsody::rg::resource::RuntimeResource> >,alloc::alloc::Global>
0x1404e9f10 41b701 mov r15b, 1
0x1404e9f13 4531f6 xor r14d, r14d
0x1404e9f16 488d9d200d0000 lea rbx, [rbp + 0xd20]
0x1404e9f1d 488d9510030000 lea rdx, [rbp + 0x310]
0x1404e9f24 41b890010000 mov r8d, 0x190
0x1404e9f2a 4889d9 mov rcx, rbx
0x1404e9f2d e8ce9c4302 call 0x142923c00 ; memcpy
0x1404e9f32 4488b514110000 mov byte ptr [rbp + 0x1114], r14b
0x1404e9f39 c6851311000000 mov byte ptr [rbp + 0x1113], 0
0x1404e9f40 4889d9 mov rcx, rbx
0x1404e9f43 e8b8247000 call 0x140bec400 ; rhapsody::rg::executor::GraphExecutionRecorder::end_frame
0x1404e9f48 4584ff test r15b, r15b
0x1404e9f4b 742c je 0x1404e9f79
0x1404e9f4d 4d85e4 test r12, r12
0x1404e9f50 0f84a2060000 je 0x1404ea5f8
0x1404e9f56 4488b514110000 mov byte ptr [rbp + 0x1114], r14b
0x1404e9f5d c6851311000000 mov byte ptr [rbp + 0x1113], 0
0x1404e9f64 4c89e1 mov rcx, r12
0x1404e9f67 488b9500110000 mov rdx, qword ptr [rbp + 0x1100]
0x1404e9f6e 4989f8 mov r8, rdi
0x1404e9f71 4189f1 mov r9d, esi
0x1404e9f74 e807507200 call 0x140c0ef80 ; rhapsody::raw::swapchain::Swapchain::present
0x1404e9f79 4488b514110000 mov byte ptr [rbp + 0x1114], r14b
0x1404e9f80 c6851311000000 mov byte ptr [rbp + 0x1113], 0
0x1404e9f87 488b8d00110000 mov rcx, qword ptr [rbp + 0x1100]
0x1404e9f8e e88d4ae3ff call 0x14031ea20 ; tiny_glade::screenshot::finish_screenshot_capture
0x1404e9f93 488b9590100000 mov rdx, qword ptr [rbp + 0x1090]
0x1404e9f9a 4881c2b00a0000 add rdx, 0xab0
0x1404e9fa1 4488b514110000 mov byte ptr [rbp + 0x1114], r14b
0x1404e9fa8 c6851311000000 mov byte ptr [rbp + 0x1113], 0
0x1404e9faf 488d8d200d0000 lea rcx, [rbp + 0xd20]
0x1404e9fb6 e8e58d7100 call 0x140c02da0 ; rhapsody::raw::stats::RenderStats::snapshot_and_reset
0x1404e9fbb 488b85300d0000 mov rax, qword ptr [rbp + 0xd30]
0x1404e9fc2 488905dfab1403 mov qword ptr [rip + 0x314abdf], rax
0x1404e9fc9 660f1085200d0000 movupd xmm0, xmmword ptr [rbp + 0xd20]
0x1404e9fd1 660f1105bfab1403 movupd xmmword ptr [rip + 0x314abbf], xmm0
0x1404e9fd9 4584f6 test r14b, r14b
0x1404e9fdc 488b9de0100000 mov rbx, qword ptr [rbp + 0x10e0]
0x1404e9fe3 7426 je 0x1404ea00b
0x1404e9fe5 488d8d280a0000 lea rcx, [rbp + 0xa28]
0x1404e9fec e88f1bb5ff call 0x14003bb80 ; core::error::Error::provide<enum2$<bevy_ecs::world::error::EntityMutableFetchError> >
0x1404e9ff1 c6851211000000 mov byte ptr [rbp + 0x1112], 0
0x1404e9ff8 c6851111000000 mov byte ptr [rbp + 0x1111], 0
0x1404e9fff 488d8d280a0000 lea rcx, [rbp + 0xa28]
0x1404ea006 e8a5846f00 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
0x1404ea00b 488d95c00a0000 lea rdx, [rbp + 0xac0]
0x1404ea012 c6850911000000 mov byte ptr [rbp + 0x1109], 0
0x1404ea019 c6850811000000 mov byte ptr [rbp + 0x1108], 0
0x1404ea020 488d8da00a0000 lea rcx, [rbp + 0xaa0]
0x1404ea027 41b848000000 mov r8d, 0x48
0x1404ea02d 41b910000000 mov r9d, 0x10
0x1404ea033 e85856b6ff call 0x14004f690 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<rhapsody::rg::resource::ResourceHandle,enum2$<rhapsody::rg::resource::RuntimeResource> >,alloc::alloc::Global>
0x1404ea038 c6851611000000 mov byte ptr [rbp + 0x1116], 0
0x1404ea03f c6851511000000 mov byte ptr [rbp + 0x1115], 0
0x1404ea046 488b8d00110000 mov rcx, qword ptr [rbp + 0x1100]
0x1404ea04d e82eef7000 call 0x140bf8f80 ; rhapsody::raw::device::GraphicsDevice::refresh_memory_budget
0x1404ea052 80bd7a08000000 cmp byte ptr [rbp + 0x87a], 0
0x1404ea059 0f840e040000 je 0x1404ea46d
0x1404ea05f c6851611000000 mov byte ptr [rbp + 0x1116], 0
0x1404ea066 c6851511000000 mov byte ptr [rbp + 0x1115], 0
0x1404ea06d 488d8dd00a0000 lea rcx, [rbp + 0xad0]
0x1404ea074 e897988700 call 0x140d63910 ; std::time::Instant::elapsed
0x1404ea079 4889c7 mov rdi, rax
0x1404ea07c 89d6 mov esi, edx
0x1404ea07e c6851611000000 mov byte ptr [rbp + 0x1116], 0
0x1404ea085 c6851511000000 mov byte ptr [rbp + 0x1115], 0
0x1404ea08c 4889d9 mov rcx, rbx
0x1404ea08f 488b9500110000 mov rdx, qword ptr [rbp + 0x1100]
0x1404ea096 e8450a7000 call 0x140beaae0 ; rhapsody::rg::executor::GraphExecutor::last_known_frame_duration_ns
0x1404ea09b 4883f801 cmp rax, 1
0x1404ea09f 0f85c8030000 jne 0x1404ea46d
0x1404ea0a5 66480f6ec7 movq xmm0, rdi
0x1404ea0aa f30f7e157e484402 movq xmm2, qword ptr [rip + 0x244487e]
0x1404ea0b2 660f62c2 punpckldq xmm0, xmm2
0x1404ea0b6 660f281d82484402 movapd xmm3, xmmword ptr [rip + 0x2444882]
0x1404ea0be 660f5cc3 subpd xmm0, xmm3
0x1404ea0c2 660f28e0 movapd xmm4, xmm0
0x1404ea0c6 660f15e0 unpckhpd xmm4, xmm0
0x1404ea0ca f20f2ace cvtsi2sd xmm1, esi
0x1404ea0ce f20f58e0 addsd xmm4, xmm0
0x1404ea0d2 f20f102dce844b02 movsd xmm5, qword ptr [rip + 0x24b84ce]
0x1404ea0da f20f5ecd divsd xmm1, xmm5
0x1404ea0de f20f58cc addsd xmm1, xmm4
0x1404ea0e2 f30f7e85c8100000 movq xmm0, qword ptr [rbp + 0x10c8]
0x1404ea0ea 660f62c2 punpckldq xmm0, xmm2
0x1404ea0ee 660f5cc3 subpd xmm0, xmm3
0x1404ea0f2 660f28d0 movapd xmm2, xmm0
0x1404ea0f6 660f15d0 unpckhpd xmm2, xmm0
0x1404ea0fa f20f58d0 addsd xmm2, xmm0
0x1404ea0fe 0f57c0 xorps xmm0, xmm0
0x1404ea101 f20f2a85c0100000 cvtsi2sd xmm0, dword ptr [rbp + 0x10c0]
0x1404ea109 f20f5ec5 divsd xmm0, xmm5
0x1404ea10d 66480f6eda movq xmm3, rdx
0x1404ea112 660f621d16484402 punpckldq xmm3, xmmword ptr [rip + 0x2444816]
0x1404ea11a f20f58c2 addsd xmm0, xmm2
0x1404ea11e 660f5c1d1a484402 subpd xmm3, xmmword ptr [rip + 0x244481a]
0x1404ea126 660f28d3 movapd xmm2, xmm3
0x1404ea12a 660f15d3 unpckhpd xmm2, xmm3
0x1404ea12e f20f58d3 addsd xmm2, xmm3
0x1404ea132 f20f5e156e844b02 divsd xmm2, qword ptr [rip + 0x24b846e]
0x1404ea13a f20f58c1 addsd xmm0, xmm1
0x1404ea13e 4c8b0593aa1403 mov r8, qword ptr [rip + 0x314aa93]
0x1404ea145 488d056caa1403 lea rax, [rip + 0x314aa6c]
0x1404ea14c 4489c1 mov ecx, r8d
0x1404ea14f 83e103 and ecx, 3
0x1404ea152 f20f1114c8 movsd qword ptr [rax + rcx*8], xmm2
0x1404ea157 488b0da2aa1403 mov rcx, qword ptr [rip + 0x314aaa2]
0x1404ea15e 4189c9 mov r9d, ecx
0x1404ea161 4183e103 and r9d, 3
0x1404ea165 488d4101 lea rax, [rcx + 1]
0x1404ea169 4c89c2 mov rdx, r8
0x1404ea16c 4c8d156daa1403 lea r10, [rip + 0x314aa6d]
0x1404ea173 f2430f1104ca movsd qword ptr [r10 + r9*8], xmm0
0x1404ea179 48890580aa1403 mov qword ptr [rip + 0x314aa80], rax
0x1404ea180 48ffc2 inc rdx
0x1404ea183 4889154eaa1403 mov qword ptr [rip + 0x314aa4e], rdx
0x1404ea18a 750d jne 0x1404ea199
0x1404ea18c f20f1005545d4402 movsd xmm0, qword ptr [rip + 0x2445d54]
0x1404ea194 e99c000000 jmp 0x1404ea235
0x1404ea199 f20f100517aa1403 movsd xmm0, qword ptr [rip + 0x314aa17]
0x1404ea1a1 f20f5d053f5d4402 minsd xmm0, qword ptr [rip + 0x2445d3f]
0x1404ea1a9 4d85c0 test r8, r8
0x1404ea1ac 0f8483000000 je 0x1404ea235
0x1404ea1b2 f20f101506aa1403 movsd xmm2, qword ptr [rip + 0x314aa06]
0x1404ea1ba 660f28c8 movapd xmm1, xmm0
0x1404ea1be f20fc2c803 cmpunordsd xmm1, xmm0
0x1404ea1c3 660f28d9 movapd xmm3, xmm1
0x1404ea1c7 660f54da andpd xmm3, xmm2
0x1404ea1cb f20f5dd0 minsd xmm2, xmm0
0x1404ea1cf 660f55ca andnpd xmm1, xmm2
0x1404ea1d3 660f56cb orpd xmm1, xmm3
0x1404ea1d7 660f28c1 movapd xmm0, xmm1
0x1404ea1db 4883fa02 cmp rdx, 2
0x1404ea1df 7454 je 0x1404ea235
0x1404ea1e1 f20f1015dfa91403 movsd xmm2, qword ptr [rip + 0x314a9df]
0x1404ea1e9 660f28c1 movapd xmm0, xmm1
0x1404ea1ed f20fc2c103 cmpunordsd xmm0, xmm1
0x1404ea1f2 660f28d8 movapd xmm3, xmm0
0x1404ea1f6 660f54da andpd xmm3, xmm2
0x1404ea1fa f20f5dd1 minsd xmm2, xmm1
0x1404ea1fe 660f55c2 andnpd xmm0, xmm2
0x1404ea202 660f56c3 orpd xmm0, xmm3
0x1404ea206 4883fa03 cmp rdx, 3
0x1404ea20a 7429 je 0x1404ea235
0x1404ea20c f20f100dbca91403 movsd xmm1, qword ptr [rip + 0x314a9bc]
0x1404ea214 660f28d0 movapd xmm2, xmm0
0x1404ea218 f20fc2d003 cmpunordsd xmm2, xmm0
0x1404ea21d 660f28da movapd xmm3, xmm2
0x1404ea221 660f54d9 andpd xmm3, xmm1
0x1404ea225 f20f5dc8 minsd xmm1, xmm0
0x1404ea229 660f55d1 andnpd xmm2, xmm1
0x1404ea22d 660f56d3 orpd xmm2, xmm3
0x1404ea231 660f28c2 movapd xmm0, xmm2
0x1404ea235 4885c0 test rax, rax
0x1404ea238 0f849e000000 je 0x1404ea2dc
0x1404ea23e f20f100d9aa91403 movsd xmm1, qword ptr [rip + 0x314a99a]
0x1404ea246 f20f5f0db2145002 maxsd xmm1, qword ptr [rip + 0x25014b2]
0x1404ea24e 4885c9 test rcx, rcx
0x1404ea251 0f848d000000 je 0x1404ea2e4
0x1404ea257 f20f101d89a91403 movsd xmm3, qword ptr [rip + 0x314a989]
0x1404ea25f 660f28d1 movapd xmm2, xmm1
0x1404ea263 f20fc2d103 cmpunordsd xmm2, xmm1
0x1404ea268 660f28e2 movapd xmm4, xmm2
0x1404ea26c 660f54e3 andpd xmm4, xmm3
0x1404ea270 f20f5fd9 maxsd xmm3, xmm1
0x1404ea274 660f55d3 andnpd xmm2, xmm3
0x1404ea278 660f56d4 orpd xmm2, xmm4
0x1404ea27c 660f28ca movapd xmm1, xmm2
0x1404ea280 4883f802 cmp rax, 2
0x1404ea284 745e je 0x1404ea2e4
0x1404ea286 f20f101d62a91403 movsd xmm3, qword ptr [rip + 0x314a962]
0x1404ea28e 660f28ca movapd xmm1, xmm2
0x1404ea292 f20fc2ca03 cmpunordsd xmm1, xmm2
0x1404ea297 660f28e1 movapd xmm4, xmm1
0x1404ea29b 660f54e3 andpd xmm4, xmm3
0x1404ea29f f20f5fda maxsd xmm3, xmm2
0x1404ea2a3 660f55cb andnpd xmm1, xmm3
0x1404ea2a7 660f56cc orpd xmm1, xmm4
0x1404ea2ab 4883f803 cmp rax, 3
0x1404ea2af 7433 je 0x1404ea2e4
0x1404ea2b1 f20f10153fa91403 movsd xmm2, qword ptr [rip + 0x314a93f]
0x1404ea2b9 660f28d9 movapd xmm3, xmm1
0x1404ea2bd f20fc2d903 cmpunordsd xmm3, xmm1
0x1404ea2c2 660f28e3 movapd xmm4, xmm3
0x1404ea2c6 660f54e2 andpd xmm4, xmm2
0x1404ea2ca f20f5fd1 maxsd xmm2, xmm1
0x1404ea2ce 660f55da andnpd xmm3, xmm2
0x1404ea2d2 660f56dc orpd xmm3, xmm4
0x1404ea2d6 660f28cb movapd xmm1, xmm3
0x1404ea2da eb08 jmp 0x1404ea2e4
0x1404ea2dc f20f100d1c145002 movsd xmm1, qword ptr [rip + 0x250141c]
0x1404ea2e4 f20f5cc1 subsd xmm0, xmm1
0x1404ea2e8 660f57c9 xorpd xmm1, xmm1
0x1404ea2ec f20f5fc1 maxsd xmm0, xmm1
0x1404ea2f0 66480f7ec2 movq rdx, xmm0
0x1404ea2f5 4889d1 mov rcx, rdx
0x1404ea2f8 48c1e934 shr rcx, 0x34
0x1404ea2fc 4189c8 mov r8d, ecx
0x1404ea2ff 4181e0ff070000 and r8d, 0x7ff
0x1404ea306 4531c9 xor r9d, r9d
0x1404ea309 4181f8e0030000 cmp r8d, 0x3e0
0x1404ea310 7308 jae 0x1404ea31a
0x1404ea312 4531c0 xor r8d, r8d
0x1404ea315 e936010000 jmp 0x1404ea450
0x1404ea31a 49baffffffffffff0f00 movabs r10, 0xfffffffffffff
0x1404ea324 4989d3 mov r11, rdx
0x1404ea327 4d21d3 and r11, r10
0x1404ea32a 48b80000000000001000 movabs rax, 0x10000000000000
0x1404ea334 4c09d8 or rax, r11
0x1404ea337 450fb7c0 movzx r8d, r8w
0x1404ea33b 4181f8ff030000 cmp r8d, 0x3ff
0x1404ea342 736f jae 0x1404ea3b3
0x1404ea344 80c12d add cl, 0x2d
0x1404ea347 31d2 xor edx, edx
0x1404ea349 480fa5c2 shld rdx, rax, cl
0x1404ea34d 48d3e0 shl rax, cl
0x1404ea350 4531d2 xor r10d, r10d
0x1404ea353 f6c140 test cl, 0x40
0x1404ea356 480f45d0 cmovne rdx, rax
0x1404ea35a 490f45c2 cmovne rax, r10
0x1404ea35e 4869ca00ca9a3b imul rcx, rdx, 0x3b9aca00
0x1404ea365 ba00ca9a3b mov edx, 0x3b9aca00
0x1404ea36a 48f7e2 mul rdx
0x1404ea36d 4801ca add rdx, rcx
0x1404ea370 4889d1 mov rcx, rdx
0x1404ea373 48c1e920 shr rcx, 0x20
0x1404ea377 4189d0 mov r8d, edx
0x1404ea37a 41b900000080 mov r9d, 0x80000000
0x1404ea380 4d31c1 xor r9, r8
0x1404ea383 4909c1 or r9, rax
0x1404ea386 0f95c0 setne al
0x1404ea389 c1ea1f shr edx, 0x1f
0x1404ea38c 4189c8 mov r8d, ecx
0x1404ea38f 4108c0 or r8b, al
0x1404ea392 4120d0 and r8b, dl
0x1404ea395 450fb6c8 movzx r9d, r8b
0x1404ea399 4101c9 add r9d, ecx
0x1404ea39c 4531c0 xor r8d, r8d
0x1404ea39f 4181f900ca9a3b cmp r9d, 0x3b9aca00
0x1404ea3a6 410f94c0 sete r8b
0x1404ea3aa 450f44ca cmove r9d, r10d
0x1404ea3ae e99d000000 jmp 0x1404ea450
0x1404ea3b3 4181f833040000 cmp r8d, 0x433
0x1404ea3ba 737e jae 0x1404ea43a
0x1404ea3bc 4c8d4101 lea r8, [rcx + 1]
0x1404ea3c0 41b133 mov r9b, 0x33
0x1404ea3c3 4128c9 sub r9b, cl
0x1404ea3c6 4489c9 mov ecx, r9d
0x1404ea3c9 48d3e8 shr rax, cl
0x1404ea3cc 4489c1 mov ecx, r8d
0x1404ea3cf 48d3e2 shl rdx, cl
0x1404ea3d2 4c21d2 and rdx, r10
0x1404ea3d5 41b800ca9a3b mov r8d, 0x3b9aca00
0x1404ea3db 4889c1 mov rcx, rax
0x1404ea3de 4889d0 mov rax, rdx
0x1404ea3e1 49f7e0 mul r8
0x1404ea3e4 c1e20c shl edx, 0xc
0x1404ea3e7 4989c0 mov r8, rax
0x1404ea3ea 49c1e834 shr r8, 0x34
0x1404ea3ee 4409c2 or edx, r8d
0x1404ea3f1 49b900feffffffff0f00 movabs r9, 0xffffffffffe00
0x1404ea3fb 4921c1 and r9, rax
0x1404ea3fe 49ba0000000000000800 movabs r10, 0x8000000000000
0x1404ea408 4d39d1 cmp r9, r10
0x1404ea40b 410f95c1 setne r9b
0x1404ea40f 48c1e833 shr rax, 0x33
0x1404ea413 83e001 and eax, 1
0x1404ea416 4508c8 or r8b, r9b
0x1404ea419 4120c0 and r8b, al
0x1404ea41c 410fb6c0 movzx eax, r8b
0x1404ea420 01d0 add eax, edx
0x1404ea422 4531c9 xor r9d, r9d
0x1404ea425 4531c0 xor r8d, r8d
0x1404ea428 3d00ca9a3b cmp eax, 0x3b9aca00
0x1404ea42d 410f94c0 sete r8b
0x1404ea431 440f45c8 cmovne r9d, eax
0x1404ea435 4901c8 add r8, rcx
0x1404ea438 eb16 jmp 0x1404ea450
0x1404ea43a 4181f83f040000 cmp r8d, 0x43f
0x1404ea441 0f834c030000 jae 0x1404ea793
0x1404ea447 80c10d add cl, 0xd
0x1404ea44a 48d3e0 shl rax, cl
0x1404ea44d 4989c0 mov r8, rax
0x1404ea450 c6851611000000 mov byte ptr [rbp + 0x1116], 0
0x1404ea457 c6851511000000 mov byte ptr [rbp + 0x1115], 0
0x1404ea45e 4889d9 mov rcx, rbx
0x1404ea461 488b9500110000 mov rdx, qword ptr [rbp + 0x1100]
0x1404ea468 e813027000 call 0x140bea680 ; rhapsody::rg::executor::GraphExecutor::wait_gpu_queue_trim
0x1404ea46d c6851611000000 mov byte ptr [rbp + 0x1116], 0
0x1404ea474 c6851511000000 mov byte ptr [rbp + 0x1115], 0
0x1404ea47b e860918700 call 0x140d635e0 ; std::time::Instant::now
0x1404ea480 48890529a71403 mov qword ptr [rip + 0x314a729], rax
0x1404ea487 89155b0c1403 mov dword ptr [rip + 0x3140c5b], edx
0x1404ea48d 488d8d88070000 lea rcx, [rbp + 0x788]
0x1404ea494 e8e7361100 call 0x1405fdb80 ; core::ptr::drop_in_place<country_core::render::render_config::VideoSettings>
0x1404ea499 488b9550080000 mov rdx, qword ptr [rbp + 0x850]
0x1404ea4a0 4889d0 mov rax, rdx
0x1404ea4a3 48f7d8 neg rax
0x1404ea4a6 7014 jo 0x1404ea4bc
0x1404ea4a8 7312 jae 0x1404ea4bc
0x1404ea4aa 488b8d58080000 mov rcx, qword ptr [rbp + 0x858]
0x1404ea4b1 41b801000000 mov r8d, 1
0x1404ea4b7 e864971200 call 0x140613c20 ; __rust_dealloc
0x1404ea4bc 488b85a0100000 mov rax, qword ptr [rbp + 0x10a0]
0x1404ea4c3 f048ff08 lock dec qword ptr [rax]
0x1404ea4c7 750c jne 0x1404ea4d5
0x1404ea4c9 488d8da0100000 lea rcx, [rbp + 0x10a0]
0x1404ea4d0 e8bb196700 call 0x140b5be90 ; alloc::sync::Arc::drop_slow<rhapsody::raw::device::GraphicsDevice,alloc::alloc::Global>
0x1404ea4d5 4883bd8010000000 cmp qword ptr [rbp + 0x1080], 0
0x1404ea4dd 7447 je 0x1404ea526
0x1404ea4df e84cdc7200 call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
0x1404ea4e4 4889c1 mov rcx, rax
0x1404ea4e7 488b00 mov rax, qword ptr [rax]
0x1404ea4ea 4883f801 cmp rax, 1
0x1404ea4ee 7578 jne 0x1404ea568
0x1404ea4f0 4883c108 add rcx, 8
0x1404ea4f4 48833900 cmp qword ptr [rcx], 0
0x1404ea4f8 0f85a8000000 jne 0x1404ea5a6
0x1404ea4fe 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x1404ea505 48898da8100000 mov qword ptr [rbp + 0x10a8], rcx
0x1404ea50c 4883c108 add rcx, 8
0x1404ea510 488b9588100000 mov rdx, qword ptr [rbp + 0x1088]
0x1404ea517 e8d4cb7200 call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
0x1404ea51c 488b85a8100000 mov rax, qword ptr [rbp + 0x10a8]
0x1404ea523 48ff00 inc qword ptr [rax]
0x1404ea526 0f28b520110000 movaps xmm6, xmmword ptr [rbp + 0x1120]
0x1404ea52d 0f28bd30110000 movaps xmm7, xmmword ptr [rbp + 0x1130]
0x1404ea534 440f288540110000 movaps xmm8, xmmword ptr [rbp + 0x1140]
0x1404ea53c 440f288d50110000 movaps xmm9, xmmword ptr [rbp + 0x1150]
0x1404ea544 440f289560110000 movaps xmm10, xmmword ptr [rbp + 0x1160]
0x1404ea54c 440f289d70110000 movaps xmm11, xmmword ptr [rbp + 0x1170]
0x1404ea554 4881c408120000 add rsp, 0x1208
0x1404ea55b 5b pop rbx
0x1404ea55c 5f pop rdi
0x1404ea55d 5e pop rsi
0x1404ea55e 415c pop r12
0x1404ea560 415d pop r13
0x1404ea562 415e pop r14
0x1404ea564 415f pop r15
0x1404ea566 5d pop rbp
0x1404ea567 c3 ret 
0x1404ea568 4885c0 test rax, rax
0x1404ea56b 752d jne 0x1404ea59a
0x1404ea56d 31d2 xor edx, edx
0x1404ea56f e8cc703b02 call 0x1428a1640 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
0x1404ea574 4889c1 mov rcx, rax
0x1404ea577 4885c0 test rax, rax
0x1404ea57a 0f8574ffffff jne 0x1404ea4f4
0x1404ea580 eb18 jmp 0x1404ea59a
0x1404ea582 4885c0 test rax, rax
0x1404ea585 7513 jne 0x1404ea59a
0x1404ea587 31d2 xor edx, edx
0x1404ea589 e8b2703b02 call 0x1428a1640 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
0x1404ea58e 4889c1 mov rcx, rax
0x1404ea591 4885c0 test rax, rax
0x1404ea594 0f85c0dcffff jne 0x1404e825a
0x1404ea59a 488d0d6f844402 lea rcx, [rip + 0x244846f]
0x1404ea5a1 e82adb3e02 call 0x1428d80d0 ; std::thread::local::panic_access_error
0x1404ea5a6 488d0d0b854402 lea rcx, [rip + 0x244850b]
0x1404ea5ad e89eec3e02 call 0x1428d9250 ; core::cell::panic_already_borrowed
0x1404ea5b2 488d0dcf155002 lea rcx, [rip + 0x25015cf]
0x1404ea5b9 e852ed3e02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404ea5be e90a020000 jmp 0x1404ea7cd
0x1404ea5c3 488d0dd6155002 lea rcx, [rip + 0x25015d6]
0x1404ea5ca 4c8d05df155002 lea r8, [rip + 0x25015df]
0x1404ea5d1 ba0a000000 mov edx, 0xa
0x1404ea5d6 e855ed3e02 call 0x1428d9330 ; core::option::expect_failed
0x1404ea5db e9ed010000 jmp 0x1404ea7cd
0x1404ea5e0 c6851711000000 mov byte ptr [rbp + 0x1117], 0
0x1404ea5e7 488d0d52165002 lea rcx, [rip + 0x2501652]
0x1404ea5ee e81ded3e02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404ea5f3 e9d5010000 jmp 0x1404ea7cd
0x1404ea5f8 4488b514110000 mov byte ptr [rbp + 0x1114], r14b
0x1404ea5ff c6851311000000 mov byte ptr [rbp + 0x1113], 0
0x1404ea606 488d0d43185002 lea rcx, [rip + 0x2501843]
0x1404ea60d e8feec3e02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404ea612 e9b6010000 jmp 0x1404ea7cd
0x1404ea617 c6851711000001 mov byte ptr [rbp + 0x1117], 1
0x1404ea61e 488d0da3155002 lea rcx, [rip + 0x25015a3]
0x1404ea625 4c8d05c4155002 lea r8, [rip + 0x25015c4]
0x1404ea62c ba28000000 mov edx, 0x28
0x1404ea631 e8faec3e02 call 0x1428d9330 ; core::option::expect_failed
0x1404ea636 e992010000 jmp 0x1404ea7cd
0x1404ea63b c6851711000000 mov byte ptr [rbp + 0x1117], 0
0x1404ea642 488d05df155002 lea rax, [rip + 0x25015df]
0x1404ea649 4889442420 mov qword ptr [rsp + 0x20], rax
0x1404ea64e 488d0db3155002 lea rcx, [rip + 0x25015b3]
0x1404ea655 4c8d0d04125002 lea r9, [rip + 0x2501204]
0x1404ea65c 4c8d8500100000 lea r8, [rbp + 0x1000]
0x1404ea663 ba1b000000 mov edx, 0x1b
0x1404ea668 e8f3f03e02 call 0x1428d9760 ; core::result::unwrap_failed
0x1404ea66d e95b010000 jmp 0x1404ea7cd
0x1404ea672 488d8570100000 lea rax, [rbp + 0x1070]
0x1404ea679 488985000b0000 mov qword ptr [rbp + 0xb00], rax
0x1404ea680 488d0509d9ffff lea rax, [rip - 0x26f7]
0x1404ea687 488985080b0000 mov qword ptr [rbp + 0xb08], rax
0x1404ea68e 488d0513185002 lea rax, [rip + 0x2501813]
0x1404ea695 488985200d0000 mov qword ptr [rbp + 0xd20], rax
0x1404ea69c 48c785280d000001000000 mov qword ptr [rbp + 0xd28], 1
0x1404ea6a7 48c785400d000000000000 mov qword ptr [rbp + 0xd40], 0
0x1404ea6b2 488d85000b0000 lea rax, [rbp + 0xb00]
0x1404ea6b9 488985300d0000 mov qword ptr [rbp + 0xd30], rax
0x1404ea6c0 48c785380d000001000000 mov qword ptr [rbp + 0xd38], 1
0x1404ea6cb c6851711000000 mov byte ptr [rbp + 0x1117], 0
0x1404ea6d2 488d15df175002 lea rdx, [rip + 0x25017df]
0x1404ea6d9 488d8d200d0000 lea rcx, [rbp + 0xd20]
0x1404ea6e0 e8abec3e02 call 0x1428d9390 ; core::panicking::panic_fmt
0x1404ea6e5 e9e3000000 jmp 0x1404ea7cd
0x1404ea6ea c6851711000000 mov byte ptr [rbp + 0x1117], 0
0x1404ea6f1 488d0590165002 lea rax, [rip + 0x2501690]
0x1404ea6f8 4889442420 mov qword ptr [rsp + 0x20], rax
0x1404ea6fd 488d0d4c165002 lea rcx, [rip + 0x250164c]
0x1404ea704 4c8d0d55115002 lea r9, [rip + 0x2501155]
0x1404ea70b 4c8d8500100000 lea r8, [rbp + 0x1000]
0x1404ea712 ba37000000 mov edx, 0x37
0x1404ea717 e844f03e02 call 0x1428d9760 ; core::result::unwrap_failed
0x1404ea71c e9ac000000 jmp 0x1404ea7cd
0x1404ea721 48c785c00f000000000000 mov qword ptr [rbp + 0xfc0], 0
0x1404ea72c 4889b550070000 mov qword ptr [rbp + 0x750], rsi
0x1404ea733 488d05b6d8ffff lea rax, [rip - 0x274a]
0x1404ea73a 48898558070000 mov qword ptr [rbp + 0x758], rax
0x1404ea741 488d05e0165002 lea rax, [rip + 0x25016e0]
0x1404ea748 488985200d0000 mov qword ptr [rbp + 0xd20], rax
0x1404ea74f 48c785280d000001000000 mov qword ptr [rbp + 0xd28], 1
0x1404ea75a 48c785400d000000000000 mov qword ptr [rbp + 0xd40], 0
0x1404ea765 4889bd300d0000 mov qword ptr [rbp + 0xd30], rdi
0x1404ea76c 48c785380d000001000000 mov qword ptr [rbp + 0xd38], 1
0x1404ea777 c6850a11000001 mov byte ptr [rbp + 0x110a], 1
0x1404ea77e 488d15b3165002 lea rdx, [rip + 0x25016b3]
0x1404ea785 488d8d200d0000 lea rcx, [rbp + 0xd20]
0x1404ea78c e8ffeb3e02 call 0x1428d9390 ; core::panicking::panic_fmt