0x14179fa20 55 push rbp
0x14179fa21 56 push rsi
0x14179fa22 57 push rdi
0x14179fa23 4881eca0010000 sub rsp, 0x1a0
0x14179fa2a 488dac2480000000 lea rbp, [rsp + 0x80]
0x14179fa32 48c78518010000feffffff mov qword ptr [rbp + 0x118], 0xfffffffffffffffe
0x14179fa3d 4889ce mov rsi, rcx
0x14179fa40 0f57c0 xorps xmm0, xmm0
0x14179fa43 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
0x14179fa47 0f114510 movups xmmword ptr [rbp + 0x10], xmm0
0x14179fa4b 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
0x14179fa4f 0f114540 movups xmmword ptr [rbp + 0x40], xmm0
0x14179fa53 0f114558 movups xmmword ptr [rbp + 0x58], xmm0
0x14179fa57 c7456800000000 mov dword ptr [rbp + 0x68], 0
0x14179fa5e 0f118590000000 movups xmmword ptr [rbp + 0x90], xmm0
0x14179fa65 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
0x14179fa6c 0f1185c0000000 movups xmmword ptr [rbp + 0xc0], xmm0
0x14179fa73 0f1185d8000000 movups xmmword ptr [rbp + 0xd8], xmm0
0x14179fa7a 0f1185f0000000 movups xmmword ptr [rbp + 0xf0], xmm0
0x14179fa81 c7850001000000000000 mov dword ptr [rbp + 0x100], 0
0x14179fa8b 48c745a800000000 mov qword ptr [rbp - 0x58], 0
0x14179fa93 48c745d800000000 mov qword ptr [rbp - 0x28], 0
0x14179fa9b 48c745e008000000 mov qword ptr [rbp - 0x20], 8
0x14179faa3 48c745e800000000 mov qword ptr [rbp - 0x18], 0
0x14179faab 48c745f010000000 mov qword ptr [rbp - 0x10], 0x10
0x14179fab3 48c7450810000000 mov qword ptr [rbp + 8], 0x10
0x14179fabb 48c7452010000000 mov qword ptr [rbp + 0x20], 0x10
0x14179fac3 48c7453810000000 mov qword ptr [rbp + 0x38], 0x10
0x14179facb 48c7455010000000 mov qword ptr [rbp + 0x50], 0x10
0x14179fad3 48b80000000000000080 movabs rax, 0x8000000000000000
0x14179fadd 48894570 mov qword ptr [rbp + 0x70], rax
0x14179fae1 488d055ec55301 lea rax, [rip + 0x153c55e]
0x14179fae8 48894578 mov qword ptr [rbp + 0x78], rax
0x14179faec 48c7858000000031000000 mov qword ptr [rbp + 0x80], 0x31
0x14179faf7 48c7858800000010000000 mov qword ptr [rbp + 0x88], 0x10
0x14179fb02 48c785a000000010000000 mov qword ptr [rbp + 0xa0], 0x10
0x14179fb0d 48c785b800000010000000 mov qword ptr [rbp + 0xb8], 0x10
0x14179fb18 48c785d000000010000000 mov qword ptr [rbp + 0xd0], 0x10
0x14179fb23 48c785e800000010000000 mov qword ptr [rbp + 0xe8], 0x10
0x14179fb2e c7850801000000000000 mov dword ptr [rbp + 0x108], 0
0x14179fb38 66c7850c0100000100 mov word ptr [rbp + 0x10c], 1
0x14179fb41 c7851001000000000000 mov dword ptr [rbp + 0x110], 0
0x14179fb4b 0fb605b750e901 movzx eax, byte ptr [rip + 0x1e950b7]
0x14179fb52 b970010000 mov ecx, 0x170
0x14179fb57 ba08000000 mov edx, 8
0x14179fb5c e8af40e7fe call 0x140613c10
0x14179fb61 4885c0 test rax, rax
0x14179fb64 7435 je 0x14179fb9b
0x14179fb66 4889c7 mov rdi, rax
0x14179fb69 488d55a8 lea rdx, [rbp - 0x58]
0x14179fb6d 41b870010000 mov r8d, 0x170
0x14179fb73 4889c1 mov rcx, rax
0x14179fb76 e885401801 call 0x142923c00
0x14179fb7b 4c8d0516c35301 lea r8, [rip + 0x153c316]
0x14179fb82 4889f1 mov rcx, rsi
0x14179fb85 4889fa mov rdx, rdi
0x14179fb88 e8834f54ff call 0x140ce4b10
0x14179fb8d 4889f0 mov rax, rsi
0x14179fb90 4881c4a0010000 add rsp, 0x1a0
0x14179fb97 5f pop rdi
0x14179fb98 5e pop rsi
0x14179fb99 5d pop rbp
0x14179fb9a c3 ret 
0x14179fb9b b908000000 mov ecx, 8
0x14179fba0 ba70010000 mov edx, 0x170
0x14179fba5 e856941301 call 0x1428d9000
0x14179fbaa 0f0b ud2 
0x14179fbac 0f1f4000 nop dword ptr [rax]
0x14179fbb0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14179fbb5 55 push rbp
0x14179fbb6 56 push rsi
0x14179fbb7 57 push rdi
0x14179fbb8 4883ec20 sub rsp, 0x20
0x14179fbbc 488daa80000000 lea rbp, [rdx + 0x80]
0x14179fbc3 488d4da8 lea rcx, [rbp - 0x58]
0x14179fbc7 e8c4efe4fe call 0x1405eeb90
0x14179fbcc 90 nop 
0x14179fbcd 4883c420 add rsp, 0x20
0x14179fbd1 5f pop rdi
0x14179fbd2 5e pop rsi
0x14179fbd3 5d pop rbp
0x14179fbd4 c3 ret 
