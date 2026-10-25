0x1403cbe60 55 push rbp
0x1403cbe61 56 push rsi
0x1403cbe62 57 push rdi
0x1403cbe63 4881ecf0010000 sub rsp, 0x1f0
0x1403cbe6a 488dac2480000000 lea rbp, [rsp + 0x80]
0x1403cbe72 48c78568010000feffffff mov qword ptr [rbp + 0x168], 0xfffffffffffffffe
0x1403cbe7d 4889ce mov rsi, rcx
0x1403cbe80 0f57c0 xorps xmm0, xmm0
0x1403cbe83 0f114548 movups xmmword ptr [rbp + 0x48], xmm0
0x1403cbe87 0f114560 movups xmmword ptr [rbp + 0x60], xmm0
0x1403cbe8b 0f114578 movups xmmword ptr [rbp + 0x78], xmm0
0x1403cbe8f 0f118590000000 movups xmmword ptr [rbp + 0x90], xmm0
0x1403cbe96 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
0x1403cbe9d c785b800000000000000 mov dword ptr [rbp + 0xb8], 0
0x1403cbea7 0f1185e0000000 movups xmmword ptr [rbp + 0xe0], xmm0
0x1403cbeae 0f1185f8000000 movups xmmword ptr [rbp + 0xf8], xmm0
0x1403cbeb5 0f118510010000 movups xmmword ptr [rbp + 0x110], xmm0
0x1403cbebc 0f118528010000 movups xmmword ptr [rbp + 0x128], xmm0
0x1403cbec3 0f118540010000 movups xmmword ptr [rbp + 0x140], xmm0
0x1403cbeca c7855001000000000000 mov dword ptr [rbp + 0x150], 0
0x1403cbed4 48c745a800000000 mov qword ptr [rbp - 0x58], 0
0x1403cbedc 48c7452800000000 mov qword ptr [rbp + 0x28], 0
0x1403cbee4 48c7453008000000 mov qword ptr [rbp + 0x30], 8
0x1403cbeec 48c7453800000000 mov qword ptr [rbp + 0x38], 0
0x1403cbef4 48c7454010000000 mov qword ptr [rbp + 0x40], 0x10
0x1403cbefc 48c7455810000000 mov qword ptr [rbp + 0x58], 0x10
0x1403cbf04 48c7457010000000 mov qword ptr [rbp + 0x70], 0x10
0x1403cbf0c 48c7858800000010000000 mov qword ptr [rbp + 0x88], 0x10
0x1403cbf17 48c785a000000010000000 mov qword ptr [rbp + 0xa0], 0x10
0x1403cbf22 48b80000000000000080 movabs rax, 0x8000000000000000
0x1403cbf2c 488985c0000000 mov qword ptr [rbp + 0xc0], rax
0x1403cbf33 488d05434d6002 lea rax, [rip + 0x2604d43]
0x1403cbf3a 488985c8000000 mov qword ptr [rbp + 0xc8], rax
0x1403cbf41 48c785d000000035000000 mov qword ptr [rbp + 0xd0], 0x35
0x1403cbf4c 48c785d800000010000000 mov qword ptr [rbp + 0xd8], 0x10
0x1403cbf57 48c785f000000010000000 mov qword ptr [rbp + 0xf0], 0x10
0x1403cbf62 48c7850801000010000000 mov qword ptr [rbp + 0x108], 0x10
0x1403cbf6d 48c7852001000010000000 mov qword ptr [rbp + 0x120], 0x10
0x1403cbf78 48c7853801000010000000 mov qword ptr [rbp + 0x138], 0x10
0x1403cbf83 c7855801000000000000 mov dword ptr [rbp + 0x158], 0
0x1403cbf8d 66c7855c0100000100 mov word ptr [rbp + 0x15c], 1
0x1403cbf96 c7856001000000000000 mov dword ptr [rbp + 0x160], 0
0x1403cbfa0 0fb605628c2603 movzx eax, byte ptr [rip + 0x3268c62]
0x1403cbfa7 b9c0010000 mov ecx, 0x1c0
0x1403cbfac ba08000000 mov edx, 8
0x1403cbfb1 e85a7c2400 call 0x140613c10
0x1403cbfb6 4885c0 test rax, rax
0x1403cbfb9 7435 je 0x1403cbff0
0x1403cbfbb 4889c7 mov rdi, rax
0x1403cbfbe 488d55a8 lea rdx, [rbp - 0x58]
0x1403cbfc2 41b8c0010000 mov r8d, 0x1c0
0x1403cbfc8 4889c1 mov rcx, rax
0x1403cbfcb e8307c5502 call 0x142923c00
0x1403cbfd0 4c8d05294b6002 lea r8, [rip + 0x2604b29]
0x1403cbfd7 4889f1 mov rcx, rsi
0x1403cbfda 4889fa mov rdx, rdi
0x1403cbfdd e82e8b9100 call 0x140ce4b10
0x1403cbfe2 4889f0 mov rax, rsi
0x1403cbfe5 4881c4f0010000 add rsp, 0x1f0
0x1403cbfec 5f pop rdi
0x1403cbfed 5e pop rsi
0x1403cbfee 5d pop rbp
0x1403cbfef c3 ret 
0x1403cbff0 b908000000 mov ecx, 8
0x1403cbff5 bac0010000 mov edx, 0x1c0
0x1403cbffa e801d05002 call 0x1428d9000
0x1403cbfff 0f0b ud2 
0x1403cc001 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1403cc010 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1403cc015 55 push rbp
0x1403cc016 56 push rsi
0x1403cc017 57 push rdi
0x1403cc018 4883ec20 sub rsp, 0x20
0x1403cc01c 488daa80000000 lea rbp, [rdx + 0x80]
0x1403cc023 488d4da8 lea rcx, [rbp - 0x58]
0x1403cc027 e8a4192100 call 0x1405dd9d0
0x1403cc02c 90 nop 
0x1403cc02d 4883c420 add rsp, 0x20
0x1403cc031 5f pop rdi
0x1403cc032 5e pop rsi
0x1403cc033 5d pop rbp
0x1403cc034 c3 ret 
