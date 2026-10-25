0x1408ae240 55 push rbp
0x1408ae241 56 push rsi
0x1408ae242 57 push rdi
0x1408ae243 4881ec90010000 sub rsp, 0x190
0x1408ae24a 488dac2480000000 lea rbp, [rsp + 0x80]
0x1408ae252 48c78508010000feffffff mov qword ptr [rbp + 0x108], 0xfffffffffffffffe
0x1408ae25d 4889ce mov rsi, rcx
0x1408ae260 0f57c0 xorps xmm0, xmm0
0x1408ae263 0f1145e8 movups xmmword ptr [rbp - 0x18], xmm0
0x1408ae267 0f114500 movups xmmword ptr [rbp], xmm0
0x1408ae26b 0f114518 movups xmmword ptr [rbp + 0x18], xmm0
0x1408ae26f 0f114530 movups xmmword ptr [rbp + 0x30], xmm0
0x1408ae273 0f114548 movups xmmword ptr [rbp + 0x48], xmm0
0x1408ae277 c7455800000000 mov dword ptr [rbp + 0x58], 0
0x1408ae27e 0f118580000000 movups xmmword ptr [rbp + 0x80], xmm0
0x1408ae285 0f118598000000 movups xmmword ptr [rbp + 0x98], xmm0
0x1408ae28c 0f1185b0000000 movups xmmword ptr [rbp + 0xb0], xmm0
0x1408ae293 0f1185c8000000 movups xmmword ptr [rbp + 0xc8], xmm0
0x1408ae29a 0f1185e0000000 movups xmmword ptr [rbp + 0xe0], xmm0
0x1408ae2a1 c785f000000000000000 mov dword ptr [rbp + 0xf0], 0
0x1408ae2ab 48c745a000000000 mov qword ptr [rbp - 0x60], 0
0x1408ae2b3 48c745c800000000 mov qword ptr [rbp - 0x38], 0
0x1408ae2bb 48c745d008000000 mov qword ptr [rbp - 0x30], 8
0x1408ae2c3 48c745d800000000 mov qword ptr [rbp - 0x28], 0
0x1408ae2cb 48c745e010000000 mov qword ptr [rbp - 0x20], 0x10
0x1408ae2d3 48c745f810000000 mov qword ptr [rbp - 8], 0x10
0x1408ae2db 48c7451010000000 mov qword ptr [rbp + 0x10], 0x10
0x1408ae2e3 48c7452810000000 mov qword ptr [rbp + 0x28], 0x10
0x1408ae2eb 48c7454010000000 mov qword ptr [rbp + 0x40], 0x10
0x1408ae2f3 48b80000000000000080 movabs rax, 0x8000000000000000
0x1408ae2fd 48894560 mov qword ptr [rbp + 0x60], rax
0x1408ae301 488d05dde71e02 lea rax, [rip + 0x21ee7dd]
0x1408ae308 48894568 mov qword ptr [rbp + 0x68], rax
0x1408ae30c 48c7457033000000 mov qword ptr [rbp + 0x70], 0x33
0x1408ae314 48c7457810000000 mov qword ptr [rbp + 0x78], 0x10
0x1408ae31c 48c7859000000010000000 mov qword ptr [rbp + 0x90], 0x10
0x1408ae327 48c785a800000010000000 mov qword ptr [rbp + 0xa8], 0x10
0x1408ae332 48c785c000000010000000 mov qword ptr [rbp + 0xc0], 0x10
0x1408ae33d 48c785d800000010000000 mov qword ptr [rbp + 0xd8], 0x10
0x1408ae348 c785f800000000000000 mov dword ptr [rbp + 0xf8], 0
0x1408ae352 66c785fc0000000100 mov word ptr [rbp + 0xfc], 1
0x1408ae35b c7850001000000000000 mov dword ptr [rbp + 0x100], 0
0x1408ae365 0fb6059d68d802 movzx eax, byte ptr [rip + 0x2d8689d]
0x1408ae36c b968010000 mov ecx, 0x168
0x1408ae371 ba08000000 mov edx, 8
0x1408ae376 e89558d6ff call 0x140613c10
0x1408ae37b 4885c0 test rax, rax
0x1408ae37e 7435 je 0x1408ae3b5
0x1408ae380 4889c7 mov rdi, rax
0x1408ae383 488d55a0 lea rdx, [rbp - 0x60]
0x1408ae387 41b868010000 mov r8d, 0x168
0x1408ae38d 4889c1 mov rcx, rax
0x1408ae390 e86b580702 call 0x142923c00
0x1408ae395 4c8d0504e51e02 lea r8, [rip + 0x21ee504]
0x1408ae39c 4889f1 mov rcx, rsi
0x1408ae39f 4889fa mov rdx, rdi
0x1408ae3a2 e869674300 call 0x140ce4b10
0x1408ae3a7 4889f0 mov rax, rsi
0x1408ae3aa 4881c490010000 add rsp, 0x190
0x1408ae3b1 5f pop rdi
0x1408ae3b2 5e pop rsi
0x1408ae3b3 5d pop rbp
0x1408ae3b4 c3 ret 
0x1408ae3b5 b908000000 mov ecx, 8
0x1408ae3ba ba68010000 mov edx, 0x168
0x1408ae3bf e83cac0202 call 0x1428d9000
0x1408ae3c4 0f0b ud2 
0x1408ae3c6 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1408ae3d0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1408ae3d5 55 push rbp
0x1408ae3d6 56 push rsi
0x1408ae3d7 57 push rdi
0x1408ae3d8 4883ec20 sub rsp, 0x20
0x1408ae3dc 488daa80000000 lea rbp, [rdx + 0x80]
0x1408ae3e3 488d4da0 lea rcx, [rbp - 0x60]
0x1408ae3e7 e8248bd3ff call 0x1405e6f10
0x1408ae3ec 90 nop 
0x1408ae3ed 4883c420 add rsp, 0x20
0x1408ae3f1 5f pop rdi
0x1408ae3f2 5e pop rsi
0x1408ae3f3 5d pop rbp
0x1408ae3f4 c3 ret 
