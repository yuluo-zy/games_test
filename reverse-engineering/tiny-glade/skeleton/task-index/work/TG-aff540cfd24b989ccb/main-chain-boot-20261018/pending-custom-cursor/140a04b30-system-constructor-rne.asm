0x140a04b30 55 push rbp
0x140a04b31 56 push rsi
0x140a04b32 57 push rdi
0x140a04b33 4881ec90010000 sub rsp, 0x190
0x140a04b3a 488dac2480000000 lea rbp, [rsp + 0x80]
0x140a04b42 48c78508010000feffffff mov qword ptr [rbp + 0x108], 0xfffffffffffffffe
0x140a04b4d 4889ce mov rsi, rcx
0x140a04b50 0f57c0 xorps xmm0, xmm0
0x140a04b53 0f1145e8 movups xmmword ptr [rbp - 0x18], xmm0
0x140a04b57 0f114500 movups xmmword ptr [rbp], xmm0
0x140a04b5b 0f114518 movups xmmword ptr [rbp + 0x18], xmm0
0x140a04b5f 0f114530 movups xmmword ptr [rbp + 0x30], xmm0
0x140a04b63 0f114548 movups xmmword ptr [rbp + 0x48], xmm0
0x140a04b67 c7455800000000 mov dword ptr [rbp + 0x58], 0
0x140a04b6e 0f118580000000 movups xmmword ptr [rbp + 0x80], xmm0
0x140a04b75 0f118598000000 movups xmmword ptr [rbp + 0x98], xmm0
0x140a04b7c 0f1185b0000000 movups xmmword ptr [rbp + 0xb0], xmm0
0x140a04b83 0f1185c8000000 movups xmmword ptr [rbp + 0xc8], xmm0
0x140a04b8a 0f1185e0000000 movups xmmword ptr [rbp + 0xe0], xmm0
0x140a04b91 c785f000000000000000 mov dword ptr [rbp + 0xf0], 0
0x140a04b9b 48c745a800000000 mov qword ptr [rbp - 0x58], 0
0x140a04ba3 48c745c800000000 mov qword ptr [rbp - 0x38], 0
0x140a04bab 48c745d008000000 mov qword ptr [rbp - 0x30], 8
0x140a04bb3 48c745d800000000 mov qword ptr [rbp - 0x28], 0
0x140a04bbb 48c745e010000000 mov qword ptr [rbp - 0x20], 0x10
0x140a04bc3 48c745f810000000 mov qword ptr [rbp - 8], 0x10
0x140a04bcb 48c7451010000000 mov qword ptr [rbp + 0x10], 0x10
0x140a04bd3 48c7452810000000 mov qword ptr [rbp + 0x28], 0x10
0x140a04bdb 48c7454010000000 mov qword ptr [rbp + 0x40], 0x10
0x140a04be3 48b80000000000000080 movabs rax, 0x8000000000000000
0x140a04bed 48894560 mov qword ptr [rbp + 0x60], rax
0x140a04bf1 488d0515180c02 lea rax, [rip + 0x20c1815]
0x140a04bf8 48894568 mov qword ptr [rbp + 0x68], rax
0x140a04bfc 48c7457031000000 mov qword ptr [rbp + 0x70], 0x31
0x140a04c04 48c7457810000000 mov qword ptr [rbp + 0x78], 0x10
0x140a04c0c 48c7859000000010000000 mov qword ptr [rbp + 0x90], 0x10
0x140a04c17 48c785a800000010000000 mov qword ptr [rbp + 0xa8], 0x10
0x140a04c22 48c785c000000010000000 mov qword ptr [rbp + 0xc0], 0x10
0x140a04c2d 48c785d800000010000000 mov qword ptr [rbp + 0xd8], 0x10
0x140a04c38 c785f800000000000000 mov dword ptr [rbp + 0xf8], 0
0x140a04c42 66c785fc0000000100 mov word ptr [rbp + 0xfc], 1
0x140a04c4b c7850001000000000000 mov dword ptr [rbp + 0x100], 0
0x140a04c55 0fb605adffc202 movzx eax, byte ptr [rip + 0x2c2ffad]
0x140a04c5c b960010000 mov ecx, 0x160
0x140a04c61 ba08000000 mov edx, 8
0x140a04c66 e8a5efc0ff call 0x140613c10
0x140a04c6b 4885c0 test rax, rax
0x140a04c6e 7435 je 0x140a04ca5
0x140a04c70 4889c7 mov rdi, rax
0x140a04c73 488d55a8 lea rdx, [rbp - 0x58]
0x140a04c77 41b860010000 mov r8d, 0x160
0x140a04c7d 4889c1 mov rcx, rax
0x140a04c80 e87beff101 call 0x142923c00
0x140a04c85 4c8d058c150c02 lea r8, [rip + 0x20c158c]
0x140a04c8c 4889f1 mov rcx, rsi
0x140a04c8f 4889fa mov rdx, rdi
0x140a04c92 e879fe2d00 call 0x140ce4b10
0x140a04c97 4889f0 mov rax, rsi
0x140a04c9a 4881c490010000 add rsp, 0x190
0x140a04ca1 5f pop rdi
0x140a04ca2 5e pop rsi
0x140a04ca3 5d pop rbp
0x140a04ca4 c3 ret 
0x140a04ca5 b908000000 mov ecx, 8
0x140a04caa ba60010000 mov edx, 0x160
0x140a04caf e84c43ed01 call 0x1428d9000
0x140a04cb4 0f0b ud2 
0x140a04cb6 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a04cc0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a04cc5 55 push rbp
0x140a04cc6 56 push rsi
0x140a04cc7 57 push rdi
0x140a04cc8 4883ec20 sub rsp, 0x20
0x140a04ccc 488daa80000000 lea rbp, [rdx + 0x80]
0x140a04cd3 488d4da8 lea rcx, [rbp - 0x58]
0x140a04cd7 e89411beff call 0x1405e5e70
0x140a04cdc 90 nop 
0x140a04cdd 4883c420 add rsp, 0x20
0x140a04ce1 5f pop rdi
0x140a04ce2 5e pop rsi
0x140a04ce3 5d pop rbp
0x140a04ce4 c3 ret 
