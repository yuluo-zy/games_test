1417e335f: 48c7454003000000 mov qword ptr [rbp + 0x40], 3
1417e3367: c7454832000000 mov dword ptr [rbp + 0x48], 0x32
1417e336e: 488d0553fa4701 lea rax, [rip + 0x147fa53]
1417e3375: 48894530 mov qword ptr [rbp + 0x30], rax
1417e3379: 48c745381d000000 mov qword ptr [rbp + 0x38], 0x1d
1417e3381: 488d4df8 lea rcx, [rbp - 8]
1417e3385: 4c8d4530 lea r8, [rbp + 0x30]
1417e3389: e8925e27ff call 0x140a59220
1417e338e: 31c0 xor eax, eax
1417e3390: 483b45f8 cmp rax, qword ptr [rbp - 8]
1417e3394: 7056 jo 0x1417e33ec
1417e3396: 488d4dd0 lea rcx, [rbp - 0x30]
1417e339a: 488b4528 mov rax, qword ptr [rbp + 0x28]
1417e339e: 48894560 mov qword ptr [rbp + 0x60], rax
1417e33a2: 0f1045f8 movups xmm0, xmmword ptr [rbp - 8]
1417e33a6: 0f104d08 movups xmm1, xmmword ptr [rbp + 8]
1417e33aa: 0f105518 movups xmm2, xmmword ptr [rbp + 0x18]
1417e33ae: 0f295550 movaps xmmword ptr [rbp + 0x50], xmm2
1417e33b2: 0f294d40 movaps xmmword ptr [rbp + 0x40], xmm1
1417e33b6: 0f294530 movaps xmmword ptr [rbp + 0x30], xmm0
1417e33ba: 48c7456800000000 mov qword ptr [rbp + 0x68], 0
1417e33c2: 48c7457008000000 mov qword ptr [rbp + 0x70], 8
1417e33ca: 48c7457800000000 mov qword ptr [rbp + 0x78], 0
1417e33d2: 4c8d0567fa4701 lea r8, [rip + 0x147fa67]
1417e33d9: 488d5530 lea rdx, [rbp + 0x30]
1417e33dd: e83ec9afff call 0x1412dfd20
