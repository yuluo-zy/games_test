0x141ceee20 55 push rbp
0x141ceee21 56 push rsi
0x141ceee22 57 push rdi
0x141ceee23 4881ecc0020000 sub rsp, 0x2c0
0x141ceee2a 488dac2480000000 lea rbp, [rsp + 0x80]
0x141ceee32 48c78538020000feffffff mov qword ptr [rbp + 0x238], 0xfffffffffffffffe
0x141ceee3d 4889ce mov rsi, rcx
0x141ceee40 0f57c0 xorps xmm0, xmm0
0x141ceee43 0f1145c8 movups xmmword ptr [rbp - 0x38], xmm0
0x141ceee47 0f1145e0 movups xmmword ptr [rbp - 0x20], xmm0
0x141ceee4b 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
0x141ceee4f 0f114510 movups xmmword ptr [rbp + 0x10], xmm0
0x141ceee53 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
0x141ceee57 c7453800000000 mov dword ptr [rbp + 0x38], 0
0x141ceee5e 0f114560 movups xmmword ptr [rbp + 0x60], xmm0
0x141ceee62 0f114578 movups xmmword ptr [rbp + 0x78], xmm0
0x141ceee66 0f118590000000 movups xmmword ptr [rbp + 0x90], xmm0
0x141ceee6d 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
0x141ceee74 0f1185c0000000 movups xmmword ptr [rbp + 0xc0], xmm0
0x141ceee7b c785d000000000000000 mov dword ptr [rbp + 0xd0], 0
0x141ceee85 48c745a800000000 mov qword ptr [rbp - 0x58], 0
0x141ceee8d 48c745b008000000 mov qword ptr [rbp - 0x50], 8
0x141ceee95 48c745b800000000 mov qword ptr [rbp - 0x48], 0
0x141ceee9d 48c745c010000000 mov qword ptr [rbp - 0x40], 0x10
0x141ceeea5 48c745d810000000 mov qword ptr [rbp - 0x28], 0x10
0x141ceeead 48c745f010000000 mov qword ptr [rbp - 0x10], 0x10
0x141ceeeb5 48c7450810000000 mov qword ptr [rbp + 8], 0x10
0x141ceeebd 48c7452010000000 mov qword ptr [rbp + 0x20], 0x10
0x141ceeec5 48b80000000000000080 movabs rax, 0x8000000000000000
0x141ceeecf 48894540 mov qword ptr [rbp + 0x40], rax
0x141ceeed3 488d0d7e8c2401 lea rcx, [rip + 0x1248c7e]
0x141ceeeda 48894d48 mov qword ptr [rbp + 0x48], rcx
0x141ceeede 48c7455052000000 mov qword ptr [rbp + 0x50], 0x52
0x141ceeee6 48c7455810000000 mov qword ptr [rbp + 0x58], 0x10
0x141ceeeee 48c7457010000000 mov qword ptr [rbp + 0x70], 0x10
0x141ceeef6 48c7858800000010000000 mov qword ptr [rbp + 0x88], 0x10
0x141ceef01 48c785a000000010000000 mov qword ptr [rbp + 0xa0], 0x10
0x141ceef0c 48c785b800000010000000 mov qword ptr [rbp + 0xb8], 0x10
0x141ceef17 c785d800000000000000 mov dword ptr [rbp + 0xd8], 0
0x141ceef21 66c785dc0000000100 mov word ptr [rbp + 0xdc], 1
0x141ceef2a 488985d8010000 mov qword ptr [rbp + 0x1d8], rax
0x141ceef31 c7853002000000000000 mov dword ptr [rbp + 0x230], 0
0x141ceef3b 0fb605c75c9401 movzx eax, byte ptr [rip + 0x1945cc7]
0x141ceef42 b990020000 mov ecx, 0x290
0x141ceef47 ba08000000 mov edx, 8
0x141ceef4c e8bf4c92fe call 0x140613c10
0x141ceef51 4885c0 test rax, rax
0x141ceef54 7435 je 0x141ceef8b
0x141ceef56 4889c7 mov rdi, rax
0x141ceef59 488d55a8 lea rdx, [rbp - 0x58]
0x141ceef5d 41b890020000 mov r8d, 0x290
0x141ceef63 4889c1 mov rcx, rax
0x141ceef66 e8954cc300 call 0x142923c00
0x141ceef6b 4c8d052e8b2401 lea r8, [rip + 0x1248b2e]
0x141ceef72 4889f1 mov rcx, rsi
0x141ceef75 4889fa mov rdx, rdi
0x141ceef78 e8935bfffe call 0x140ce4b10
0x141ceef7d 4889f0 mov rax, rsi
0x141ceef80 4881c4c0020000 add rsp, 0x2c0
0x141ceef87 5f pop rdi
0x141ceef88 5e pop rsi
0x141ceef89 5d pop rbp
0x141ceef8a c3 ret 
0x141ceef8b b908000000 mov ecx, 8
0x141ceef90 ba90020000 mov edx, 0x290
0x141ceef95 e866a0be00 call 0x1428d9000
0x141ceef9a 0f0b ud2 
0x141ceef9c 0f1f4000 nop dword ptr [rax]
0x141ceefa0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141ceefa5 55 push rbp
0x141ceefa6 56 push rsi
0x141ceefa7 57 push rdi
0x141ceefa8 4883ec20 sub rsp, 0x20
0x141ceefac 488daa80000000 lea rbp, [rdx + 0x80]
0x141ceefb3 488d4da8 lea rcx, [rbp - 0x58]
0x141ceefb7 e8f4038efe call 0x1405cf3b0
0x141ceefbc 90 nop 
0x141ceefbd 4883c420 add rsp, 0x20
0x141ceefc1 5f pop rdi
0x141ceefc2 5e pop rsi
0x141ceefc3 5d pop rbp
0x141ceefc4 c3 ret 
