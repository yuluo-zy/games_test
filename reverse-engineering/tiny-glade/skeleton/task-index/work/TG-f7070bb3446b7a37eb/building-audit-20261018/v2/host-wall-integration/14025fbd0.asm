0x14025fbd0 55 push rbp
0x14025fbd1 56 push rsi
0x14025fbd2 57 push rdi
0x14025fbd3 4881eca0010000 sub rsp, 0x1a0
0x14025fbda 488dac2480000000 lea rbp, [rsp + 0x80]
0x14025fbe2 48c78518010000feffffff mov qword ptr [rbp + 0x118], 0xfffffffffffffffe
0x14025fbed 4889ce mov rsi, rcx
0x14025fbf0 0f57c0 xorps xmm0, xmm0
0x14025fbf3 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
0x14025fbf7 0f114510 movups xmmword ptr [rbp + 0x10], xmm0
0x14025fbfb 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
0x14025fbff 0f114540 movups xmmword ptr [rbp + 0x40], xmm0
0x14025fc03 0f114558 movups xmmword ptr [rbp + 0x58], xmm0
0x14025fc07 c7456800000000 mov dword ptr [rbp + 0x68], 0
0x14025fc0e 0f118590000000 movups xmmword ptr [rbp + 0x90], xmm0
0x14025fc15 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
0x14025fc1c 0f1185c0000000 movups xmmword ptr [rbp + 0xc0], xmm0
0x14025fc23 0f1185d8000000 movups xmmword ptr [rbp + 0xd8], xmm0
0x14025fc2a 0f1185f0000000 movups xmmword ptr [rbp + 0xf0], xmm0
0x14025fc31 c7850001000000000000 mov dword ptr [rbp + 0x100], 0
0x14025fc3b 48c745a000000000 mov qword ptr [rbp - 0x60], 0
0x14025fc43 48c745d800000000 mov qword ptr [rbp - 0x28], 0
0x14025fc4b 48c745e008000000 mov qword ptr [rbp - 0x20], 8
0x14025fc53 48c745e800000000 mov qword ptr [rbp - 0x18], 0
0x14025fc5b 48c745f010000000 mov qword ptr [rbp - 0x10], 0x10
0x14025fc63 48c7450810000000 mov qword ptr [rbp + 8], 0x10
0x14025fc6b 48c7452010000000 mov qword ptr [rbp + 0x20], 0x10
0x14025fc73 48c7453810000000 mov qword ptr [rbp + 0x38], 0x10
0x14025fc7b 48c7455010000000 mov qword ptr [rbp + 0x50], 0x10
0x14025fc83 48b80000000000000080 movabs rax, 0x8000000000000000
0x14025fc8d 48894570 mov qword ptr [rbp + 0x70], rax
0x14025fc91 488d0525647402 lea rax, [rip + 0x2746425]
0x14025fc98 48894578 mov qword ptr [rbp + 0x78], rax
0x14025fc9c 48c785800000003b000000 mov qword ptr [rbp + 0x80], 0x3b
0x14025fca7 48c7858800000010000000 mov qword ptr [rbp + 0x88], 0x10
0x14025fcb2 48c785a000000010000000 mov qword ptr [rbp + 0xa0], 0x10
0x14025fcbd 48c785b800000010000000 mov qword ptr [rbp + 0xb8], 0x10
0x14025fcc8 48c785d000000010000000 mov qword ptr [rbp + 0xd0], 0x10
0x14025fcd3 48c785e800000010000000 mov qword ptr [rbp + 0xe8], 0x10
0x14025fcde c7850801000000000000 mov dword ptr [rbp + 0x108], 0
0x14025fce8 66c7850c0100000100 mov word ptr [rbp + 0x10c], 1
0x14025fcf1 c7851001000000000000 mov dword ptr [rbp + 0x110], 0
0x14025fcfb 0fb605074f3d03 movzx eax, byte ptr [rip + 0x33d4f07]
0x14025fd02 b978010000 mov ecx, 0x178
0x14025fd07 ba08000000 mov edx, 8
0x14025fd0c e8ff3e3b00 call 0x140613c10
0x14025fd11 4885c0 test rax, rax
0x14025fd14 7435 je 0x14025fd4b
0x14025fd16 4889c7 mov rdi, rax
0x14025fd19 488d55a0 lea rdx, [rbp - 0x60]
0x14025fd1d 41b878010000 mov r8d, 0x178
0x14025fd23 4889c1 mov rcx, rax
0x14025fd26 e8d53e6c02 call 0x142923c00
0x14025fd2b 4c8d05e6617402 lea r8, [rip + 0x27461e6]
0x14025fd32 4889f1 mov rcx, rsi
0x14025fd35 4889fa mov rdx, rdi
0x14025fd38 e8d34da800 call 0x140ce4b10
0x14025fd3d 4889f0 mov rax, rsi
0x14025fd40 4881c4a0010000 add rsp, 0x1a0
0x14025fd47 5f pop rdi
0x14025fd48 5e pop rsi
0x14025fd49 5d pop rbp
0x14025fd4a c3 ret 
0x14025fd4b b908000000 mov ecx, 8
0x14025fd50 ba78010000 mov edx, 0x178
0x14025fd55 e8a6926702 call 0x1428d9000
0x14025fd5a 0f0b ud2 
0x14025fd5c 0f1f4000 nop dword ptr [rax]
0x14025fd60 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14025fd65 55 push rbp
0x14025fd66 56 push rsi
0x14025fd67 57 push rdi
0x14025fd68 4883ec20 sub rsp, 0x20
0x14025fd6c 488daa80000000 lea rbp, [rdx + 0x80]
0x14025fd73 488d4da0 lea rcx, [rbp - 0x60]
0x14025fd77 e8749d3800 call 0x1405e9af0
0x14025fd7c 90 nop 
0x14025fd7d 4883c420 add rsp, 0x20
0x14025fd81 5f pop rdi
0x14025fd82 5e pop rsi
0x14025fd83 5d pop rbp
0x14025fd84 c3 ret 
