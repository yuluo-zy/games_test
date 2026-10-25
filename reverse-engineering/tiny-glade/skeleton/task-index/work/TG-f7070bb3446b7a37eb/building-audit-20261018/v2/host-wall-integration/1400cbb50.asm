0x1400cbb50 55 push rbp
0x1400cbb51 56 push rsi
0x1400cbb52 57 push rdi
0x1400cbb53 4881ec00030000 sub rsp, 0x300
0x1400cbb5a 488dac2480000000 lea rbp, [rsp + 0x80]
0x1400cbb62 48c78578020000feffffff mov qword ptr [rbp + 0x278], 0xfffffffffffffffe
0x1400cbb6d 4889ce mov rsi, rcx
0x1400cbb70 0f57c0 xorps xmm0, xmm0
0x1400cbb73 0f1145c0 movups xmmword ptr [rbp - 0x40], xmm0
0x1400cbb77 0f1145d8 movups xmmword ptr [rbp - 0x28], xmm0
0x1400cbb7b 0f1145f0 movups xmmword ptr [rbp - 0x10], xmm0
0x1400cbb7f 0f114508 movups xmmword ptr [rbp + 8], xmm0
0x1400cbb83 0f114520 movups xmmword ptr [rbp + 0x20], xmm0
0x1400cbb87 c7453000000000 mov dword ptr [rbp + 0x30], 0
0x1400cbb8e 0f114558 movups xmmword ptr [rbp + 0x58], xmm0
0x1400cbb92 0f114570 movups xmmword ptr [rbp + 0x70], xmm0
0x1400cbb96 0f118588000000 movups xmmword ptr [rbp + 0x88], xmm0
0x1400cbb9d 0f1185a0000000 movups xmmword ptr [rbp + 0xa0], xmm0
0x1400cbba4 0f1185b8000000 movups xmmword ptr [rbp + 0xb8], xmm0
0x1400cbbab c785c800000000000000 mov dword ptr [rbp + 0xc8], 0
0x1400cbbb5 48c745a000000000 mov qword ptr [rbp - 0x60], 0
0x1400cbbbd 48c745a808000000 mov qword ptr [rbp - 0x58], 8
0x1400cbbc5 48c745b000000000 mov qword ptr [rbp - 0x50], 0
0x1400cbbcd 48c745b810000000 mov qword ptr [rbp - 0x48], 0x10
0x1400cbbd5 48c745d010000000 mov qword ptr [rbp - 0x30], 0x10
0x1400cbbdd 48c745e810000000 mov qword ptr [rbp - 0x18], 0x10
0x1400cbbe5 48c7450010000000 mov qword ptr [rbp], 0x10
0x1400cbbed 48c7451810000000 mov qword ptr [rbp + 0x18], 0x10
0x1400cbbf5 48b80000000000000080 movabs rax, 0x8000000000000000
0x1400cbbff 48894538 mov qword ptr [rbp + 0x38], rax
0x1400cbc03 488d0dc1ba8602 lea rcx, [rip + 0x286bac1]
0x1400cbc0a 48894d40 mov qword ptr [rbp + 0x40], rcx
0x1400cbc0e 48c7454867000000 mov qword ptr [rbp + 0x48], 0x67
0x1400cbc16 48c7455010000000 mov qword ptr [rbp + 0x50], 0x10
0x1400cbc1e 48c7456810000000 mov qword ptr [rbp + 0x68], 0x10
0x1400cbc26 48c7858000000010000000 mov qword ptr [rbp + 0x80], 0x10
0x1400cbc31 48c7859800000010000000 mov qword ptr [rbp + 0x98], 0x10
0x1400cbc3c 48c785b000000010000000 mov qword ptr [rbp + 0xb0], 0x10
0x1400cbc47 c785d000000000000000 mov dword ptr [rbp + 0xd0], 0
0x1400cbc51 66c785d40000000100 mov word ptr [rbp + 0xd4], 1
0x1400cbc5a 488985d0010000 mov qword ptr [rbp + 0x1d0], rax
0x1400cbc61 c7857002000000000000 mov dword ptr [rbp + 0x270], 0
0x1400cbc6b 0fb605978f5603 movzx eax, byte ptr [rip + 0x3568f97]
0x1400cbc72 b9d8020000 mov ecx, 0x2d8
0x1400cbc77 ba08000000 mov edx, 8
0x1400cbc7c e88f7f5400 call 0x140613c10
0x1400cbc81 4885c0 test rax, rax
0x1400cbc84 7435 je 0x1400cbcbb
0x1400cbc86 4889c7 mov rdi, rax
0x1400cbc89 488d55a0 lea rdx, [rbp - 0x60]
0x1400cbc8d 41b8d8020000 mov r8d, 0x2d8
0x1400cbc93 4889c1 mov rcx, rax
0x1400cbc96 e8657f8502 call 0x142923c00
0x1400cbc9b 4c8d05e6b88602 lea r8, [rip + 0x286b8e6]
0x1400cbca2 4889f1 mov rcx, rsi
0x1400cbca5 4889fa mov rdx, rdi
0x1400cbca8 e8638ec100 call 0x140ce4b10
0x1400cbcad 4889f0 mov rax, rsi
0x1400cbcb0 4881c400030000 add rsp, 0x300
0x1400cbcb7 5f pop rdi
0x1400cbcb8 5e pop rsi
0x1400cbcb9 5d pop rbp
0x1400cbcba c3 ret 
0x1400cbcbb b908000000 mov ecx, 8
0x1400cbcc0 bad8020000 mov edx, 0x2d8
0x1400cbcc5 e836d38002 call 0x1428d9000
0x1400cbcca 0f0b ud2 
0x1400cbccc 0f1f4000 nop dword ptr [rax]
0x1400cbcd0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1400cbcd5 55 push rbp
0x1400cbcd6 56 push rsi
0x1400cbcd7 57 push rdi
0x1400cbcd8 4883ec20 sub rsp, 0x20
0x1400cbcdc 488daa80000000 lea rbp, [rdx + 0x80]
0x1400cbce3 488d4da0 lea rcx, [rbp - 0x60]
0x1400cbce7 e8c4365000 call 0x1405cf3b0
0x1400cbcec 90 nop 
0x1400cbced 4883c420 add rsp, 0x20
0x1400cbcf1 5f pop rdi
0x1400cbcf2 5e pop rsi
0x1400cbcf3 5d pop rbp
0x1400cbcf4 c3 ret 
