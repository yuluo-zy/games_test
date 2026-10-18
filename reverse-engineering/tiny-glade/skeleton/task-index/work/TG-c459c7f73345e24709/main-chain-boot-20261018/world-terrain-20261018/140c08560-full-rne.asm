140c08560: push r15
140c08562: push r14
140c08564: push r13
140c08566: push r12
140c08568: push rsi
140c08569: push rdi
140c0856a: push rbp
140c0856b: push rbx
140c0856c: sub rsp, 0xc8
140c08573: movaps xmmword ptr [rsp + 0xb0], xmm6
140c0857b: mov rbx, rcx
140c0857e: mov rax, qword ptr [rcx]
140c08581: mov rcx, qword ptr [rax + 8]
140c08585: mov r8d, dword ptr [rcx + 0xc]
140c08589: cmp r8d, dword ptr [rax + 0x10]
140c0858d: jne 0x140c085b1
140c0858f: cmp dword ptr [rdx], 1
140c08592: jne 0x140c087d1
140c08598: mov r8, qword ptr [rdx + 8]
140c0859c: mov rax, qword ptr [rcx]
140c0859f: nop 
140c085a0: cmp rax, r8
140c085a3: mov r9, r8
140c085a6: cmova r9, rax
140c085aa: lock cmpxchg qword ptr [rcx], r9
140c085af: jne 0x140c085a0
140c085b1: mov r14, qword ptr [rbx + 8]
140c085b5: mov r15, qword ptr [rbx + 0x10]
140c085b9: mov rax, qword ptr [r14 + 0x30]
140c085bd: mov ecx, dword ptr [r15]
140c085c0: mov r12, qword ptr [rbx + 0x18]
140c085c4: lea rsi, [r12 + 0x78]
140c085c9: mov rdi, qword ptr [rdx + 0x48]
140c085cd: mov qword ptr [rsp + 0x90], 0
140c085d9: mov byte ptr [rsp + 0x46], 0
140c085de: mov byte ptr [rsp + 0x47], 0x34
140c085e3: lea rdx, [rsp + 0x46]
140c085e8: mov qword ptr [rsp + 0x48], rdx
140c085ed: mov qword ptr [rsp + 0x50], 1
140c085f6: lea r13, [rsp + 0x47]
140c085fb: mov qword ptr [rsp + 0x58], r13
140c08600: mov qword ptr [rsp + 0x60], 1
140c08609: mov qword ptr [rsp + 0x68], rax
140c0860e: mov qword ptr [rsp + 0x70], 0xffffffffffffffff
140c08617: mov dword ptr [rsp + 0x78], ecx
140c0861b: movaps xmm6, xmmword ptr [rip + 0x1f0791e] ; 0x142b0ff40 b'\x00\x00\x00\x00\xff\xff\xff\xff\x00\x00\x00\x00\xff\xff\xff\xff\x8f\xff\xff\xff\x8f\xff\xff\xff'
140c08622: movups xmmword ptr [rsp + 0x7c], xmm6
140c08627: mov word ptr [rsp + 0x8c], 0
140c08631: mov byte ptr [rsp + 0x8e], 1
140c08639: lea rbp, [rsp + 0x48]
140c0863e: mov qword ptr [rsp + 0x28], rbp
140c08643: mov qword ptr [rsp + 0x30], 1
140c0864c: mov qword ptr [rsp + 0x20], 0
140c08655: lea r8, [rsp + 0x90]
140c0865d: mov r9d, 8
140c08663: mov rcx, rsi
140c08666: mov rdx, rdi
140c08669: call 0x1418afd20 ; vk_sync_fork::cmd::pipeline_barrier
140c0866e: mov eax, dword ptr [r15]
140c08671: mov rdx, qword ptr [r14 + 0x30]
140c08675: test al, 6
140c08677: je 0x140c086cd
140c08679: mov qword ptr [rsp + 0x90], 0
140c08685: mov rcx, qword ptr [rbx + 0x20]
140c08689: mov r8, qword ptr [rbx + 0x28]
140c0868d: mov r8d, dword ptr [r8]
140c08690: shl r8, 0x20
140c08694: movzx ecx, byte ptr [rcx]
140c08697: mov qword ptr [rsp + 0x48], rax
140c0869c: mov dword ptr [rsp + 0x50], ecx
140c086a0: mov qword ptr [rsp + 0x54], r8
140c086a5: mov qword ptr [rsp + 0x28], rbp
140c086aa: mov dword ptr [rsp + 0x20], 1
140c086b2: lea r9, [rsp + 0x90]
140c086ba: mov rcx, rdi
140c086bd: mov r8d, 7
140c086c3: call qword ptr [r12 + 0x3b0]
140c086cb: jmp 0x140c0871e
140c086cd: xorps xmm0, xmm0
140c086d0: movaps xmmword ptr [rsp + 0x90], xmm0
140c086d8: mov rcx, qword ptr [rbx + 0x20]
140c086dc: mov r8, qword ptr [rbx + 0x28]
140c086e0: mov r8d, dword ptr [r8]
140c086e3: shl r8, 0x20
140c086e7: movzx ecx, byte ptr [rcx]
140c086ea: mov qword ptr [rsp + 0x48], rax
140c086ef: mov dword ptr [rsp + 0x50], ecx
140c086f3: mov qword ptr [rsp + 0x54], r8
140c086f8: mov qword ptr [rsp + 0x28], rbp
140c086fd: mov dword ptr [rsp + 0x20], 1
140c08705: lea r9, [rsp + 0x90]
140c0870d: mov rcx, rdi
140c08710: mov r8d, 7
140c08716: call qword ptr [r12 + 0x3a8]
140c0871e: mov rax, qword ptr [r14 + 0x30]
140c08722: mov ecx, dword ptr [r15]
140c08725: mov qword ptr [rsp + 0x90], 0
140c08731: mov byte ptr [rsp + 0x46], 0x34
140c08736: mov byte ptr [rsp + 0x47], 0x22
140c0873b: lea rdx, [rsp + 0x46]
140c08740: mov qword ptr [rsp + 0x48], rdx
140c08745: mov qword ptr [rsp + 0x50], 1
140c0874e: mov qword ptr [rsp + 0x58], r13
140c08753: mov qword ptr [rsp + 0x60], 1
140c0875c: mov qword ptr [rsp + 0x68], rax
140c08761: mov qword ptr [rsp + 0x70], 0xffffffffffffffff
140c0876a: mov dword ptr [rsp + 0x78], ecx
140c0876e: movups xmmword ptr [rsp + 0x7c], xmm6
140c08773: mov word ptr [rsp + 0x8c], 0
140c0877d: mov byte ptr [rsp + 0x8e], 0
140c08785: mov qword ptr [rsp + 0x28], rbp
140c0878a: mov qword ptr [rsp + 0x30], 1
140c08793: mov qword ptr [rsp + 0x20], 0
140c0879c: lea r8, [rsp + 0x90]
140c087a4: mov r9d, 8
140c087aa: mov rcx, rsi
140c087ad: mov rdx, rdi
140c087b0: call 0x1418afd20 ; vk_sync_fork::cmd::pipeline_barrier
140c087b5: movaps xmm6, xmmword ptr [rsp + 0xb0]
140c087bd: add rsp, 0xc8
140c087c4: pop rbx
140c087c5: pop rbp
140c087c6: pop rdi
140c087c7: pop rsi
140c087c8: pop r12
140c087ca: pop r13
140c087cc: pop r14
140c087ce: pop r15
140c087d0: ret 
140c087d1: mov qword ptr [rsp + 0x90], rdx
140c087d9: lea rax, [rip - 0x1b1c0] ; 0x140bed620 b'H\x83\xechH\x89\xd0H\x8b\x11L\x8d\x05?x\xf2\x01Ic\x14\x90L\x01\xc2'
140c087e0: mov qword ptr [rsp + 0x98], rax
140c087e8: lea rax, [rip + 0x1f0c3c1] ; 0x142b14bb0 b'{K\xb1B\x01\x00\x00\x003\x00\x00\x00\x00\x00\x00\x00crates/r'
140c087ef: mov qword ptr [rsp + 0x48], rax
140c087f4: mov qword ptr [rsp + 0x50], 1
140c087fd: mov qword ptr [rsp + 0x68], 0
140c08806: lea rax, [rsp + 0x90]
140c0880e: mov qword ptr [rsp + 0x58], rax
140c08813: mov qword ptr [rsp + 0x60], 1
140c0881c: lea rdx, [rip + 0x1f0c3cd] ; 0x142b14bf0 b'\xc0K\xb1B\x01\x00\x00\x00)\x00\x00\x00\x00\x00\x00\x00<\x00\x00\x00\x11\x00\x00\x00'
140c08823: lea rcx, [rsp + 0x48]
140c08828: call 0x1428d9390 ; core::panicking::panic_fmt
140c0882d: int3 
