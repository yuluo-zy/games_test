0x1400ecfc0 55 push rbp
0x1400ecfc1 56 push rsi
0x1400ecfc2 57 push rdi
0x1400ecfc3 4881ec90010000 sub rsp, 0x190
0x1400ecfca 488dac2480000000 lea rbp, [rsp + 0x80]
0x1400ecfd2 48c78508010000feffffff mov qword ptr [rbp + 0x108], 0xfffffffffffffffe
0x1400ecfdd 4889ce mov rsi, rcx
0x1400ecfe0 0f57c0 xorps xmm0, xmm0
0x1400ecfe3 0f1145e8 movups xmmword ptr [rbp - 0x18], xmm0
0x1400ecfe7 0f114500 movups xmmword ptr [rbp], xmm0
0x1400ecfeb 0f114518 movups xmmword ptr [rbp + 0x18], xmm0
0x1400ecfef 0f114530 movups xmmword ptr [rbp + 0x30], xmm0
0x1400ecff3 0f114548 movups xmmword ptr [rbp + 0x48], xmm0
0x1400ecff7 c7455800000000 mov dword ptr [rbp + 0x58], 0
0x1400ecffe 0f118580000000 movups xmmword ptr [rbp + 0x80], xmm0
0x1400ed005 0f118598000000 movups xmmword ptr [rbp + 0x98], xmm0
0x1400ed00c 0f1185b0000000 movups xmmword ptr [rbp + 0xb0], xmm0
0x1400ed013 0f1185c8000000 movups xmmword ptr [rbp + 0xc8], xmm0
0x1400ed01a 0f1185e0000000 movups xmmword ptr [rbp + 0xe0], xmm0
0x1400ed021 c785f000000000000000 mov dword ptr [rbp + 0xf0], 0
0x1400ed02b 48c745a000000000 mov qword ptr [rbp - 0x60], 0
0x1400ed033 48c745c800000000 mov qword ptr [rbp - 0x38], 0
0x1400ed03b 48c745d008000000 mov qword ptr [rbp - 0x30], 8
0x1400ed043 48c745d800000000 mov qword ptr [rbp - 0x28], 0
0x1400ed04b 48c745e010000000 mov qword ptr [rbp - 0x20], 0x10
0x1400ed053 48c745f810000000 mov qword ptr [rbp - 8], 0x10
0x1400ed05b 48c7451010000000 mov qword ptr [rbp + 0x10], 0x10
0x1400ed063 48c7452810000000 mov qword ptr [rbp + 0x28], 0x10
0x1400ed06b 48c7454010000000 mov qword ptr [rbp + 0x40], 0x10
0x1400ed073 48b80000000000000080 movabs rax, 0x8000000000000000
0x1400ed07d 48894560 mov qword ptr [rbp + 0x60], rax
0x1400ed081 488d05f7038502 lea rax, [rip + 0x28503f7]
0x1400ed088 48894568 mov qword ptr [rbp + 0x68], rax
0x1400ed08c 48c7457054000000 mov qword ptr [rbp + 0x70], 0x54
0x1400ed094 48c7457810000000 mov qword ptr [rbp + 0x78], 0x10
0x1400ed09c 48c7859000000010000000 mov qword ptr [rbp + 0x90], 0x10
0x1400ed0a7 48c785a800000010000000 mov qword ptr [rbp + 0xa8], 0x10
0x1400ed0b2 48c785c000000010000000 mov qword ptr [rbp + 0xc0], 0x10
0x1400ed0bd 48c785d800000010000000 mov qword ptr [rbp + 0xd8], 0x10
0x1400ed0c8 c785f800000000000000 mov dword ptr [rbp + 0xf8], 0
0x1400ed0d2 66c785fc0000000100 mov word ptr [rbp + 0xfc], 1
0x1400ed0db c7850001000000000000 mov dword ptr [rbp + 0x100], 0
0x1400ed0e5 0fb6051d7b5403 movzx eax, byte ptr [rip + 0x3547b1d]
0x1400ed0ec b968010000 mov ecx, 0x168
0x1400ed0f1 ba08000000 mov edx, 8
0x1400ed0f6 e8156b5200 call 0x140613c10
0x1400ed0fb 4885c0 test rax, rax
0x1400ed0fe 7435 je 0x1400ed135
0x1400ed100 4889c7 mov rdi, rax
0x1400ed103 488d55a0 lea rdx, [rbp - 0x60]
0x1400ed107 41b868010000 mov r8d, 0x168
0x1400ed10d 4889c1 mov rcx, rax
0x1400ed110 e8eb6a8302 call 0x142923c00
0x1400ed115 4c8d0544018502 lea r8, [rip + 0x2850144]
0x1400ed11c 4889f1 mov rcx, rsi
0x1400ed11f 4889fa mov rdx, rdi
0x1400ed122 e8e979bf00 call 0x140ce4b10
0x1400ed127 4889f0 mov rax, rsi
0x1400ed12a 4881c490010000 add rsp, 0x190
0x1400ed131 5f pop rdi
0x1400ed132 5e pop rsi
0x1400ed133 5d pop rbp
0x1400ed134 c3 ret 
0x1400ed135 b908000000 mov ecx, 8
0x1400ed13a ba68010000 mov edx, 0x168
0x1400ed13f e8bcbe7e02 call 0x1428d9000
0x1400ed144 0f0b ud2 
0x1400ed146 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1400ed150 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1400ed155 55 push rbp
0x1400ed156 56 push rsi
0x1400ed157 57 push rdi
0x1400ed158 4883ec20 sub rsp, 0x20
0x1400ed15c 488daa80000000 lea rbp, [rdx + 0x80]
0x1400ed163 488d4da0 lea rcx, [rbp - 0x60]
0x1400ed167 e8a49d4f00 call 0x1405e6f10
0x1400ed16c 90 nop 
0x1400ed16d 4883c420 add rsp, 0x20
0x1400ed171 5f pop rdi
0x1400ed172 5e pop rsi
0x1400ed173 5d pop rbp
0x1400ed174 c3 ret 
