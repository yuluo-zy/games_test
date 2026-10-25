0x140a9f240 55 push rbp
0x140a9f241 56 push rsi
0x140a9f242 57 push rdi
0x140a9f243 4881ece0010000 sub rsp, 0x1e0
0x140a9f24a 488dac2480000000 lea rbp, [rsp + 0x80]
0x140a9f252 48c78558010000feffffff mov qword ptr [rbp + 0x158], 0xfffffffffffffffe
0x140a9f25d 4889ce mov rsi, rcx
0x140a9f260 0f57c0 xorps xmm0, xmm0
0x140a9f263 0f114538 movups xmmword ptr [rbp + 0x38], xmm0
0x140a9f267 0f114550 movups xmmword ptr [rbp + 0x50], xmm0
0x140a9f26b 0f114568 movups xmmword ptr [rbp + 0x68], xmm0
0x140a9f26f 0f118580000000 movups xmmword ptr [rbp + 0x80], xmm0
0x140a9f276 0f118598000000 movups xmmword ptr [rbp + 0x98], xmm0
0x140a9f27d c785a800000000000000 mov dword ptr [rbp + 0xa8], 0
0x140a9f287 0f1185d0000000 movups xmmword ptr [rbp + 0xd0], xmm0
0x140a9f28e 0f1185e8000000 movups xmmword ptr [rbp + 0xe8], xmm0
0x140a9f295 0f118500010000 movups xmmword ptr [rbp + 0x100], xmm0
0x140a9f29c 0f118518010000 movups xmmword ptr [rbp + 0x118], xmm0
0x140a9f2a3 0f118530010000 movups xmmword ptr [rbp + 0x130], xmm0
0x140a9f2aa c7854001000000000000 mov dword ptr [rbp + 0x140], 0
0x140a9f2b4 48c745a000000000 mov qword ptr [rbp - 0x60], 0
0x140a9f2bc 48c7451800000000 mov qword ptr [rbp + 0x18], 0
0x140a9f2c4 48c7452008000000 mov qword ptr [rbp + 0x20], 8
0x140a9f2cc 48c7452800000000 mov qword ptr [rbp + 0x28], 0
0x140a9f2d4 48c7453010000000 mov qword ptr [rbp + 0x30], 0x10
0x140a9f2dc 48c7454810000000 mov qword ptr [rbp + 0x48], 0x10
0x140a9f2e4 48c7456010000000 mov qword ptr [rbp + 0x60], 0x10
0x140a9f2ec 48c7457810000000 mov qword ptr [rbp + 0x78], 0x10
0x140a9f2f4 48c7859000000010000000 mov qword ptr [rbp + 0x90], 0x10
0x140a9f2ff 48b80000000000000080 movabs rax, 0x8000000000000000
0x140a9f309 488985b0000000 mov qword ptr [rbp + 0xb0], rax
0x140a9f310 488d0583ce0302 lea rax, [rip + 0x203ce83]
0x140a9f317 488985b8000000 mov qword ptr [rbp + 0xb8], rax
0x140a9f31e 48c785c000000038000000 mov qword ptr [rbp + 0xc0], 0x38
0x140a9f329 48c785c800000010000000 mov qword ptr [rbp + 0xc8], 0x10
0x140a9f334 48c785e000000010000000 mov qword ptr [rbp + 0xe0], 0x10
0x140a9f33f 48c785f800000010000000 mov qword ptr [rbp + 0xf8], 0x10
0x140a9f34a 48c7851001000010000000 mov qword ptr [rbp + 0x110], 0x10
0x140a9f355 48c7852801000010000000 mov qword ptr [rbp + 0x128], 0x10
0x140a9f360 c7854801000000000000 mov dword ptr [rbp + 0x148], 0
0x140a9f36a 66c7854c0100000100 mov word ptr [rbp + 0x14c], 1
0x140a9f373 c7855001000000000000 mov dword ptr [rbp + 0x150], 0
0x140a9f37d 0fb6058558b902 movzx eax, byte ptr [rip + 0x2b95885]
0x140a9f384 b9b8010000 mov ecx, 0x1b8
0x140a9f389 ba08000000 mov edx, 8
0x140a9f38e e87d48b7ff call 0x140613c10
0x140a9f393 4885c0 test rax, rax
0x140a9f396 7435 je 0x140a9f3cd
0x140a9f398 4889c7 mov rdi, rax
0x140a9f39b 488d55a0 lea rdx, [rbp - 0x60]
0x140a9f39f 41b8b8010000 mov r8d, 0x1b8
0x140a9f3a5 4889c1 mov rcx, rax
0x140a9f3a8 e85348e801 call 0x142923c00
0x140a9f3ad 4c8d0574cb0302 lea r8, [rip + 0x203cb74]
0x140a9f3b4 4889f1 mov rcx, rsi
0x140a9f3b7 4889fa mov rdx, rdi
0x140a9f3ba e851572400 call 0x140ce4b10
0x140a9f3bf 4889f0 mov rax, rsi
0x140a9f3c2 4881c4e0010000 add rsp, 0x1e0
0x140a9f3c9 5f pop rdi
0x140a9f3ca 5e pop rsi
0x140a9f3cb 5d pop rbp
0x140a9f3cc c3 ret 
0x140a9f3cd b908000000 mov ecx, 8
0x140a9f3d2 bab8010000 mov edx, 0x1b8
0x140a9f3d7 e8249ce301 call 0x1428d9000
0x140a9f3dc 0f0b ud2 
0x140a9f3de 6690 nop 
0x140a9f3e0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a9f3e5 55 push rbp
0x140a9f3e6 56 push rsi
0x140a9f3e7 57 push rdi
0x140a9f3e8 4883ec20 sub rsp, 0x20
0x140a9f3ec 488daa80000000 lea rbp, [rdx + 0x80]
0x140a9f3f3 488d4da0 lea rcx, [rbp - 0x60]
0x140a9f3f7 e804c3b3ff call 0x1405db700
0x140a9f3fc 90 nop 
0x140a9f3fd 4883c420 add rsp, 0x20
0x140a9f401 5f pop rdi
0x140a9f402 5e pop rsi
0x140a9f403 5d pop rbp
0x140a9f404 c3 ret 
