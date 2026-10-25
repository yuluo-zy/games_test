0x140ee5d90 55 push rbp
0x140ee5d91 56 push rsi
0x140ee5d92 57 push rdi
0x140ee5d93 4881ec90010000 sub rsp, 0x190
0x140ee5d9a 488dac2480000000 lea rbp, [rsp + 0x80]
0x140ee5da2 48c78508010000feffffff mov qword ptr [rbp + 0x108], 0xfffffffffffffffe
0x140ee5dad 4889ce mov rsi, rcx
0x140ee5db0 0f57c0 xorps xmm0, xmm0
0x140ee5db3 0f1145e8 movups xmmword ptr [rbp - 0x18], xmm0
0x140ee5db7 0f114500 movups xmmword ptr [rbp], xmm0
0x140ee5dbb 0f114518 movups xmmword ptr [rbp + 0x18], xmm0
0x140ee5dbf 0f114530 movups xmmword ptr [rbp + 0x30], xmm0
0x140ee5dc3 0f114548 movups xmmword ptr [rbp + 0x48], xmm0
0x140ee5dc7 c7455800000000 mov dword ptr [rbp + 0x58], 0
0x140ee5dce 0f118580000000 movups xmmword ptr [rbp + 0x80], xmm0
0x140ee5dd5 0f118598000000 movups xmmword ptr [rbp + 0x98], xmm0
0x140ee5ddc 0f1185b0000000 movups xmmword ptr [rbp + 0xb0], xmm0
0x140ee5de3 0f1185c8000000 movups xmmword ptr [rbp + 0xc8], xmm0
0x140ee5dea 0f1185e0000000 movups xmmword ptr [rbp + 0xe0], xmm0
0x140ee5df1 c785f000000000000000 mov dword ptr [rbp + 0xf0], 0
0x140ee5dfb 48c745a000000000 mov qword ptr [rbp - 0x60], 0
0x140ee5e03 48c745c800000000 mov qword ptr [rbp - 0x38], 0
0x140ee5e0b 48c745d008000000 mov qword ptr [rbp - 0x30], 8
0x140ee5e13 48c745d800000000 mov qword ptr [rbp - 0x28], 0
0x140ee5e1b 48c745e010000000 mov qword ptr [rbp - 0x20], 0x10
0x140ee5e23 48c745f810000000 mov qword ptr [rbp - 8], 0x10
0x140ee5e2b 48c7451010000000 mov qword ptr [rbp + 0x10], 0x10
0x140ee5e33 48c7452810000000 mov qword ptr [rbp + 0x28], 0x10
0x140ee5e3b 48c7454010000000 mov qword ptr [rbp + 0x40], 0x10
0x140ee5e43 48b80000000000000080 movabs rax, 0x8000000000000000
0x140ee5e4d 48894560 mov qword ptr [rbp + 0x60], rax
0x140ee5e51 488d052029cb01 lea rax, [rip + 0x1cb2920]
0x140ee5e58 48894568 mov qword ptr [rbp + 0x68], rax
0x140ee5e5c 48c745703c000000 mov qword ptr [rbp + 0x70], 0x3c
0x140ee5e64 48c7457810000000 mov qword ptr [rbp + 0x78], 0x10
0x140ee5e6c 48c7859000000010000000 mov qword ptr [rbp + 0x90], 0x10
0x140ee5e77 48c785a800000010000000 mov qword ptr [rbp + 0xa8], 0x10
0x140ee5e82 48c785c000000010000000 mov qword ptr [rbp + 0xc0], 0x10
0x140ee5e8d 48c785d800000010000000 mov qword ptr [rbp + 0xd8], 0x10
0x140ee5e98 c785f800000000000000 mov dword ptr [rbp + 0xf8], 0
0x140ee5ea2 66c785fc0000000100 mov word ptr [rbp + 0xfc], 1
0x140ee5eab c7850001000000000000 mov dword ptr [rbp + 0x100], 0
0x140ee5eb5 0fb6054ded7402 movzx eax, byte ptr [rip + 0x274ed4d]
0x140ee5ebc b968010000 mov ecx, 0x168
0x140ee5ec1 ba08000000 mov edx, 8
0x140ee5ec6 e845dd72ff call 0x140613c10
0x140ee5ecb 4885c0 test rax, rax
0x140ee5ece 7435 je 0x140ee5f05
0x140ee5ed0 4889c7 mov rdi, rax
0x140ee5ed3 488d55a0 lea rdx, [rbp - 0x60]
0x140ee5ed7 41b868010000 mov r8d, 0x168
0x140ee5edd 4889c1 mov rcx, rax
0x140ee5ee0 e81bdda301 call 0x142923c00
0x140ee5ee5 4c8d056425cb01 lea r8, [rip + 0x1cb2564]
0x140ee5eec 4889f1 mov rcx, rsi
0x140ee5eef 4889fa mov rdx, rdi
0x140ee5ef2 e819ecdfff call 0x140ce4b10
0x140ee5ef7 4889f0 mov rax, rsi
0x140ee5efa 4881c490010000 add rsp, 0x190
0x140ee5f01 5f pop rdi
0x140ee5f02 5e pop rsi
0x140ee5f03 5d pop rbp
0x140ee5f04 c3 ret 
0x140ee5f05 b908000000 mov ecx, 8
0x140ee5f0a ba68010000 mov edx, 0x168
0x140ee5f0f e8ec309f01 call 0x1428d9000
0x140ee5f14 0f0b ud2 
0x140ee5f16 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140ee5f20 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140ee5f25 55 push rbp
0x140ee5f26 56 push rsi
0x140ee5f27 57 push rdi
0x140ee5f28 4883ec20 sub rsp, 0x20
0x140ee5f2c 488daa80000000 lea rbp, [rdx + 0x80]
0x140ee5f33 488d4da0 lea rcx, [rbp - 0x60]
0x140ee5f37 e8d40f70ff call 0x1405e6f10
0x140ee5f3c 90 nop 
0x140ee5f3d 4883c420 add rsp, 0x20
0x140ee5f41 5f pop rdi
0x140ee5f42 5e pop rsi
0x140ee5f43 5d pop rbp
0x140ee5f44 c3 ret 
