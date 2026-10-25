0x1405b0b70 55 push rbp
0x1405b0b71 56 push rsi
0x1405b0b72 57 push rdi
0x1405b0b73 4881ece0010000 sub rsp, 0x1e0
0x1405b0b7a 488dac2480000000 lea rbp, [rsp + 0x80]
0x1405b0b82 48c78558010000feffffff mov qword ptr [rbp + 0x158], 0xfffffffffffffffe
0x1405b0b8d 4889ce mov rsi, rcx
0x1405b0b90 0f57c0 xorps xmm0, xmm0
0x1405b0b93 0f114538 movups xmmword ptr [rbp + 0x38], xmm0
0x1405b0b97 0f114550 movups xmmword ptr [rbp + 0x50], xmm0
0x1405b0b9b 0f114568 movups xmmword ptr [rbp + 0x68], xmm0
0x1405b0b9f 0f118580000000 movups xmmword ptr [rbp + 0x80], xmm0
0x1405b0ba6 0f118598000000 movups xmmword ptr [rbp + 0x98], xmm0
0x1405b0bad c785a800000000000000 mov dword ptr [rbp + 0xa8], 0
0x1405b0bb7 0f1185d0000000 movups xmmword ptr [rbp + 0xd0], xmm0
0x1405b0bbe 0f1185e8000000 movups xmmword ptr [rbp + 0xe8], xmm0
0x1405b0bc5 0f118500010000 movups xmmword ptr [rbp + 0x100], xmm0
0x1405b0bcc 0f118518010000 movups xmmword ptr [rbp + 0x118], xmm0
0x1405b0bd3 0f118530010000 movups xmmword ptr [rbp + 0x130], xmm0
0x1405b0bda c7854001000000000000 mov dword ptr [rbp + 0x140], 0
0x1405b0be4 48c745a000000000 mov qword ptr [rbp - 0x60], 0
0x1405b0bec 48c7451800000000 mov qword ptr [rbp + 0x18], 0
0x1405b0bf4 48c7452008000000 mov qword ptr [rbp + 0x20], 8
0x1405b0bfc 48c7452800000000 mov qword ptr [rbp + 0x28], 0
0x1405b0c04 48c7453010000000 mov qword ptr [rbp + 0x30], 0x10
0x1405b0c0c 48c7454810000000 mov qword ptr [rbp + 0x48], 0x10
0x1405b0c14 48c7456010000000 mov qword ptr [rbp + 0x60], 0x10
0x1405b0c1c 48c7457810000000 mov qword ptr [rbp + 0x78], 0x10
0x1405b0c24 48c7859000000010000000 mov qword ptr [rbp + 0x90], 0x10
0x1405b0c2f 48b80000000000000080 movabs rax, 0x8000000000000000
0x1405b0c39 488985b0000000 mov qword ptr [rbp + 0xb0], rax
0x1405b0c40 488d0597ff4402 lea rax, [rip + 0x244ff97]
0x1405b0c47 488985b8000000 mov qword ptr [rbp + 0xb8], rax
0x1405b0c4e 48c785c00000003f000000 mov qword ptr [rbp + 0xc0], 0x3f
0x1405b0c59 48c785c800000010000000 mov qword ptr [rbp + 0xc8], 0x10
0x1405b0c64 48c785e000000010000000 mov qword ptr [rbp + 0xe0], 0x10
0x1405b0c6f 48c785f800000010000000 mov qword ptr [rbp + 0xf8], 0x10
0x1405b0c7a 48c7851001000010000000 mov qword ptr [rbp + 0x110], 0x10
0x1405b0c85 48c7852801000010000000 mov qword ptr [rbp + 0x128], 0x10
0x1405b0c90 c7854801000000000000 mov dword ptr [rbp + 0x148], 0
0x1405b0c9a 66c7854c0100000100 mov word ptr [rbp + 0x14c], 1
0x1405b0ca3 c7855001000000000000 mov dword ptr [rbp + 0x150], 0
0x1405b0cad 0fb605553f0803 movzx eax, byte ptr [rip + 0x3083f55]
0x1405b0cb4 b9b8010000 mov ecx, 0x1b8
0x1405b0cb9 ba08000000 mov edx, 8
0x1405b0cbe e84d2f0600 call 0x140613c10
0x1405b0cc3 4885c0 test rax, rax
0x1405b0cc6 7435 je 0x1405b0cfd
0x1405b0cc8 4889c7 mov rdi, rax
0x1405b0ccb 488d55a0 lea rdx, [rbp - 0x60]
0x1405b0ccf 41b8b8010000 mov r8d, 0x1b8
0x1405b0cd5 4889c1 mov rcx, rax
0x1405b0cd8 e8232f3702 call 0x142923c00
0x1405b0cdd 4c8d05dcfd4402 lea r8, [rip + 0x244fddc]
0x1405b0ce4 4889f1 mov rcx, rsi
0x1405b0ce7 4889fa mov rdx, rdi
0x1405b0cea e8213e7300 call 0x140ce4b10
0x1405b0cef 4889f0 mov rax, rsi
0x1405b0cf2 4881c4e0010000 add rsp, 0x1e0
0x1405b0cf9 5f pop rdi
0x1405b0cfa 5e pop rsi
0x1405b0cfb 5d pop rbp
0x1405b0cfc c3 ret 
0x1405b0cfd b908000000 mov ecx, 8
0x1405b0d02 bab8010000 mov edx, 0x1b8
0x1405b0d07 e8f4823202 call 0x1428d9000
0x1405b0d0c 0f0b ud2 
0x1405b0d0e 6690 nop 
0x1405b0d10 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1405b0d15 55 push rbp
0x1405b0d16 56 push rsi
0x1405b0d17 57 push rdi
0x1405b0d18 4883ec20 sub rsp, 0x20
0x1405b0d1c 488daa80000000 lea rbp, [rdx + 0x80]
0x1405b0d23 488d4da0 lea rcx, [rbp - 0x60]
0x1405b0d27 e8d4a90200 call 0x1405db700
0x1405b0d2c 90 nop 
0x1405b0d2d 4883c420 add rsp, 0x20
0x1405b0d31 5f pop rdi
0x1405b0d32 5e pop rsi
0x1405b0d33 5d pop rbp
0x1405b0d34 c3 ret 
