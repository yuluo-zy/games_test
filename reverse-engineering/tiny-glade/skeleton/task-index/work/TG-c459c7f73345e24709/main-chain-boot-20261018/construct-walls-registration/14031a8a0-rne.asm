0x14031a8a0 55 push rbp
0x14031a8a1 56 push rsi
0x14031a8a2 57 push rdi
0x14031a8a3 4881ec00020000 sub rsp, 0x200
0x14031a8aa 488dac2480000000 lea rbp, [rsp + 0x80]
0x14031a8b2 48c78578010000feffffff mov qword ptr [rbp + 0x178], 0xfffffffffffffffe
0x14031a8bd 4889ce mov rsi, rcx
0x14031a8c0 0f57c0 xorps xmm0, xmm0
0x14031a8c3 0f1145c0 movups xmmword ptr [rbp - 0x40], xmm0
0x14031a8c7 0f1145d8 movups xmmword ptr [rbp - 0x28], xmm0
0x14031a8cb 0f1145f0 movups xmmword ptr [rbp - 0x10], xmm0
0x14031a8cf 0f114508 movups xmmword ptr [rbp + 8], xmm0
0x14031a8d3 0f114520 movups xmmword ptr [rbp + 0x20], xmm0
0x14031a8d7 c7453000000000 mov dword ptr [rbp + 0x30], 0
0x14031a8de 0f114558 movups xmmword ptr [rbp + 0x58], xmm0
0x14031a8e2 0f114570 movups xmmword ptr [rbp + 0x70], xmm0
0x14031a8e6 0f118588000000 movups xmmword ptr [rbp + 0x88], xmm0
0x14031a8ed 0f1185a0000000 movups xmmword ptr [rbp + 0xa0], xmm0
0x14031a8f4 0f1185b8000000 movups xmmword ptr [rbp + 0xb8], xmm0
0x14031a8fb c785c800000000000000 mov dword ptr [rbp + 0xc8], 0
0x14031a905 48c745a000000000 mov qword ptr [rbp - 0x60], 0
0x14031a90d 48c745a808000000 mov qword ptr [rbp - 0x58], 8
0x14031a915 48c745b000000000 mov qword ptr [rbp - 0x50], 0
0x14031a91d 48c745b810000000 mov qword ptr [rbp - 0x48], 0x10
0x14031a925 48c745d010000000 mov qword ptr [rbp - 0x30], 0x10
0x14031a92d 48c745e810000000 mov qword ptr [rbp - 0x18], 0x10
0x14031a935 48c7450010000000 mov qword ptr [rbp], 0x10
0x14031a93d 48c7451810000000 mov qword ptr [rbp + 0x18], 0x10
0x14031a945 48b80000000000000080 movabs rax, 0x8000000000000000
0x14031a94f 48894538 mov qword ptr [rbp + 0x38], rax
0x14031a953 488d0d231a6a02 lea rcx, [rip + 0x26a1a23]
0x14031a95a 48894d40 mov qword ptr [rbp + 0x40], rcx
0x14031a95e 48c745483b000000 mov qword ptr [rbp + 0x48], 0x3b
0x14031a966 48c7455010000000 mov qword ptr [rbp + 0x50], 0x10
0x14031a96e 48c7456810000000 mov qword ptr [rbp + 0x68], 0x10
0x14031a976 48c7858000000010000000 mov qword ptr [rbp + 0x80], 0x10
0x14031a981 48c7859800000010000000 mov qword ptr [rbp + 0x98], 0x10
0x14031a98c 48c785b000000010000000 mov qword ptr [rbp + 0xb0], 0x10
0x14031a997 c785d000000000000000 mov dword ptr [rbp + 0xd0], 0
0x14031a9a1 66c785d40000000100 mov word ptr [rbp + 0xd4], 1
0x14031a9aa 48898538010000 mov qword ptr [rbp + 0x138], rax
0x14031a9b1 c7857001000000000000 mov dword ptr [rbp + 0x170], 0
0x14031a9bb 0fb60547a23103 movzx eax, byte ptr [rip + 0x331a247]
0x14031a9c2 b9d8010000 mov ecx, 0x1d8
0x14031a9c7 ba08000000 mov edx, 8
0x14031a9cc e83f922f00 call 0x140613c10
0x14031a9d1 4885c0 test rax, rax
0x14031a9d4 7435 je 0x14031aa0b
0x14031a9d6 4889c7 mov rdi, rax
0x14031a9d9 488d55a0 lea rdx, [rbp - 0x60]
0x14031a9dd 41b8d8010000 mov r8d, 0x1d8
0x14031a9e3 4889c1 mov rcx, rax
0x14031a9e6 e815926002 call 0x142923c00
0x14031a9eb 4c8d05b6186a02 lea r8, [rip + 0x26a18b6]
0x14031a9f2 4889f1 mov rcx, rsi
0x14031a9f5 4889fa mov rdx, rdi
0x14031a9f8 e813a19c00 call 0x140ce4b10
0x14031a9fd 4889f0 mov rax, rsi
0x14031aa00 4881c400020000 add rsp, 0x200
0x14031aa07 5f pop rdi
0x14031aa08 5e pop rsi
0x14031aa09 5d pop rbp
0x14031aa0a c3 ret 
0x14031aa0b b908000000 mov ecx, 8
0x14031aa10 bad8010000 mov edx, 0x1d8
0x14031aa15 e8e6e55b02 call 0x1428d9000
0x14031aa1a 0f0b ud2 
0x14031aa1c 0f1f4000 nop dword ptr [rax]
0x14031aa20 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14031aa25 55 push rbp
0x14031aa26 56 push rsi
0x14031aa27 57 push rdi
0x14031aa28 4883ec20 sub rsp, 0x20
0x14031aa2c 488daa80000000 lea rbp, [rdx + 0x80]
0x14031aa33 488d4da0 lea rcx, [rbp - 0x60]
0x14031aa37 e804292e00 call 0x1405fd340
0x14031aa3c 90 nop 
0x14031aa3d 4883c420 add rsp, 0x20
0x14031aa41 5f pop rdi
0x14031aa42 5e pop rsi
0x14031aa43 5d pop rbp
0x14031aa44 c3 ret 
