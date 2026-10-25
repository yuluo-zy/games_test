0x140174db0 55 push rbp
0x140174db1 56 push rsi
0x140174db2 57 push rdi
0x140174db3 4881eca0010000 sub rsp, 0x1a0
0x140174dba 488dac2480000000 lea rbp, [rsp + 0x80]
0x140174dc2 48c78518010000feffffff mov qword ptr [rbp + 0x118], 0xfffffffffffffffe
0x140174dcd 4889ce mov rsi, rcx
0x140174dd0 0f57c0 xorps xmm0, xmm0
0x140174dd3 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
0x140174dd7 0f114510 movups xmmword ptr [rbp + 0x10], xmm0
0x140174ddb 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
0x140174ddf 0f114540 movups xmmword ptr [rbp + 0x40], xmm0
0x140174de3 0f114558 movups xmmword ptr [rbp + 0x58], xmm0
0x140174de7 c7456800000000 mov dword ptr [rbp + 0x68], 0
0x140174dee 0f118590000000 movups xmmword ptr [rbp + 0x90], xmm0
0x140174df5 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
0x140174dfc 0f1185c0000000 movups xmmword ptr [rbp + 0xc0], xmm0
0x140174e03 0f1185d8000000 movups xmmword ptr [rbp + 0xd8], xmm0
0x140174e0a 0f1185f0000000 movups xmmword ptr [rbp + 0xf0], xmm0
0x140174e11 c7850001000000000000 mov dword ptr [rbp + 0x100], 0
0x140174e1b 48c745a000000000 mov qword ptr [rbp - 0x60], 0
0x140174e23 48c745d800000000 mov qword ptr [rbp - 0x28], 0
0x140174e2b 48c745e008000000 mov qword ptr [rbp - 0x20], 8
0x140174e33 48c745e800000000 mov qword ptr [rbp - 0x18], 0
0x140174e3b 48c745f010000000 mov qword ptr [rbp - 0x10], 0x10
0x140174e43 48c7450810000000 mov qword ptr [rbp + 8], 0x10
0x140174e4b 48c7452010000000 mov qword ptr [rbp + 0x20], 0x10
0x140174e53 48c7453810000000 mov qword ptr [rbp + 0x38], 0x10
0x140174e5b 48c7455010000000 mov qword ptr [rbp + 0x50], 0x10
0x140174e63 48b80000000000000080 movabs rax, 0x8000000000000000
0x140174e6d 48894570 mov qword ptr [rbp + 0x70], rax
0x140174e71 488d05e1af7e02 lea rax, [rip + 0x27eafe1]
0x140174e78 48894578 mov qword ptr [rbp + 0x78], rax
0x140174e7c 48c7858000000069000000 mov qword ptr [rbp + 0x80], 0x69
0x140174e87 48c7858800000010000000 mov qword ptr [rbp + 0x88], 0x10
0x140174e92 48c785a000000010000000 mov qword ptr [rbp + 0xa0], 0x10
0x140174e9d 48c785b800000010000000 mov qword ptr [rbp + 0xb8], 0x10
0x140174ea8 48c785d000000010000000 mov qword ptr [rbp + 0xd0], 0x10
0x140174eb3 48c785e800000010000000 mov qword ptr [rbp + 0xe8], 0x10
0x140174ebe c7850801000000000000 mov dword ptr [rbp + 0x108], 0
0x140174ec8 66c7850c0100000100 mov word ptr [rbp + 0x10c], 1
0x140174ed1 c7851001000000000000 mov dword ptr [rbp + 0x110], 0
0x140174edb 0fb60527fd4b03 movzx eax, byte ptr [rip + 0x34bfd27]
0x140174ee2 b978010000 mov ecx, 0x178
0x140174ee7 ba08000000 mov edx, 8
0x140174eec e81fed4900 call 0x140613c10
0x140174ef1 4885c0 test rax, rax
0x140174ef4 7435 je 0x140174f2b
0x140174ef6 4889c7 mov rdi, rax
0x140174ef9 488d55a0 lea rdx, [rbp - 0x60]
0x140174efd 41b878010000 mov r8d, 0x178
0x140174f03 4889c1 mov rcx, rax
0x140174f06 e8f5ec7a02 call 0x142923c00
0x140174f0b 4c8d05e6ac7e02 lea r8, [rip + 0x27eace6]
0x140174f12 4889f1 mov rcx, rsi
0x140174f15 4889fa mov rdx, rdi
0x140174f18 e8f3fbb600 call 0x140ce4b10
0x140174f1d 4889f0 mov rax, rsi
0x140174f20 4881c4a0010000 add rsp, 0x1a0
0x140174f27 5f pop rdi
0x140174f28 5e pop rsi
0x140174f29 5d pop rbp
0x140174f2a c3 ret 
0x140174f2b b908000000 mov ecx, 8
0x140174f30 ba78010000 mov edx, 0x178
0x140174f35 e8c6407602 call 0x1428d9000
0x140174f3a 0f0b ud2 
0x140174f3c 0f1f4000 nop dword ptr [rax]
0x140174f40 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140174f45 55 push rbp
0x140174f46 56 push rsi
0x140174f47 57 push rdi
0x140174f48 4883ec20 sub rsp, 0x20
0x140174f4c 488daa80000000 lea rbp, [rdx + 0x80]
0x140174f53 488d4da0 lea rcx, [rbp - 0x60]
0x140174f57 e8944b4700 call 0x1405e9af0
0x140174f5c 90 nop 
0x140174f5d 4883c420 add rsp, 0x20
0x140174f61 5f pop rdi
0x140174f62 5e pop rsi
0x140174f63 5d pop rbp
0x140174f64 c3 ret 
