0x1400ecc50 55 push rbp
0x1400ecc51 56 push rsi
0x1400ecc52 57 push rdi
0x1400ecc53 4881ece0030000 sub rsp, 0x3e0
0x1400ecc5a 488dac2480000000 lea rbp, [rsp + 0x80]
0x1400ecc62 48c78558030000feffffff mov qword ptr [rbp + 0x358], 0xfffffffffffffffe
0x1400ecc6d 4889ce mov rsi, rcx
0x1400ecc70 0f57c0 xorps xmm0, xmm0
0x1400ecc73 0f1145c0 movups xmmword ptr [rbp - 0x40], xmm0
0x1400ecc77 0f1145d8 movups xmmword ptr [rbp - 0x28], xmm0
0x1400ecc7b 0f1145f0 movups xmmword ptr [rbp - 0x10], xmm0
0x1400ecc7f 0f114508 movups xmmword ptr [rbp + 8], xmm0
0x1400ecc83 0f114520 movups xmmword ptr [rbp + 0x20], xmm0
0x1400ecc87 c7453000000000 mov dword ptr [rbp + 0x30], 0
0x1400ecc8e 0f114558 movups xmmword ptr [rbp + 0x58], xmm0
0x1400ecc92 0f114570 movups xmmword ptr [rbp + 0x70], xmm0
0x1400ecc96 0f118588000000 movups xmmword ptr [rbp + 0x88], xmm0
0x1400ecc9d 0f1185a0000000 movups xmmword ptr [rbp + 0xa0], xmm0
0x1400ecca4 0f1185b8000000 movups xmmword ptr [rbp + 0xb8], xmm0
0x1400eccab c785c800000000000000 mov dword ptr [rbp + 0xc8], 0
0x1400eccb5 48c745a000000000 mov qword ptr [rbp - 0x60], 0
0x1400eccbd 48c745a808000000 mov qword ptr [rbp - 0x58], 8
0x1400eccc5 48c745b000000000 mov qword ptr [rbp - 0x50], 0
0x1400ecccd 48c745b810000000 mov qword ptr [rbp - 0x48], 0x10
0x1400eccd5 48c745d010000000 mov qword ptr [rbp - 0x30], 0x10
0x1400eccdd 48c745e810000000 mov qword ptr [rbp - 0x18], 0x10
0x1400ecce5 48c7450010000000 mov qword ptr [rbp], 0x10
0x1400ecced 48c7451810000000 mov qword ptr [rbp + 0x18], 0x10
0x1400eccf5 48b80000000000000080 movabs rax, 0x8000000000000000
0x1400eccff 48894538 mov qword ptr [rbp + 0x38], rax
0x1400ecd03 488d0d28078502 lea rcx, [rip + 0x2850728]
0x1400ecd0a 48894d40 mov qword ptr [rbp + 0x40], rcx
0x1400ecd0e 48c745484d000000 mov qword ptr [rbp + 0x48], 0x4d
0x1400ecd16 48c7455010000000 mov qword ptr [rbp + 0x50], 0x10
0x1400ecd1e 48c7456810000000 mov qword ptr [rbp + 0x68], 0x10
0x1400ecd26 48c7858000000010000000 mov qword ptr [rbp + 0x80], 0x10
0x1400ecd31 48c7859800000010000000 mov qword ptr [rbp + 0x98], 0x10
0x1400ecd3c 48c785b000000010000000 mov qword ptr [rbp + 0xb0], 0x10
0x1400ecd47 c785d000000000000000 mov dword ptr [rbp + 0xd0], 0
0x1400ecd51 66c785d40000000100 mov word ptr [rbp + 0xd4], 1
0x1400ecd5a 488985c8010000 mov qword ptr [rbp + 0x1c8], rax
0x1400ecd61 c7855003000000000000 mov dword ptr [rbp + 0x350], 0
0x1400ecd6b 0fb605977e5403 movzx eax, byte ptr [rip + 0x3547e97]
0x1400ecd72 b9b8030000 mov ecx, 0x3b8
0x1400ecd77 ba08000000 mov edx, 8
0x1400ecd7c e88f6e5200 call 0x140613c10
0x1400ecd81 4885c0 test rax, rax
0x1400ecd84 7435 je 0x1400ecdbb
0x1400ecd86 4889c7 mov rdi, rax
0x1400ecd89 488d55a0 lea rdx, [rbp - 0x60]
0x1400ecd8d 41b8b8030000 mov r8d, 0x3b8
0x1400ecd93 4889c1 mov rcx, rax
0x1400ecd96 e8656e8302 call 0x142923c00
0x1400ecd9b 4c8d054e038502 lea r8, [rip + 0x285034e]
0x1400ecda2 4889f1 mov rcx, rsi
0x1400ecda5 4889fa mov rdx, rdi
0x1400ecda8 e8637dbf00 call 0x140ce4b10
0x1400ecdad 4889f0 mov rax, rsi
0x1400ecdb0 4881c4e0030000 add rsp, 0x3e0
0x1400ecdb7 5f pop rdi
0x1400ecdb8 5e pop rsi
0x1400ecdb9 5d pop rbp
0x1400ecdba c3 ret 
0x1400ecdbb b908000000 mov ecx, 8
0x1400ecdc0 bab8030000 mov edx, 0x3b8
0x1400ecdc5 e836c27e02 call 0x1428d9000
0x1400ecdca 0f0b ud2 
0x1400ecdcc 0f1f4000 nop dword ptr [rax]
0x1400ecdd0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1400ecdd5 55 push rbp
0x1400ecdd6 56 push rsi
0x1400ecdd7 57 push rdi
0x1400ecdd8 4883ec20 sub rsp, 0x20
0x1400ecddc 488daa80000000 lea rbp, [rdx + 0x80]
0x1400ecde3 488d4da0 lea rcx, [rbp - 0x60]
0x1400ecde7 e8c4934e00 call 0x1405d61b0
0x1400ecdec 90 nop 
0x1400ecded 4883c420 add rsp, 0x20
0x1400ecdf1 5f pop rdi
0x1400ecdf2 5e pop rsi
0x1400ecdf3 5d pop rbp
0x1400ecdf4 c3 ret 
