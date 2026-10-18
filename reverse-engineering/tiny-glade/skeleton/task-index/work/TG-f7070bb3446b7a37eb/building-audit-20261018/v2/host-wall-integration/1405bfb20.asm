0x1405bfb20 55 push rbp
0x1405bfb21 56 push rsi
0x1405bfb22 57 push rdi
0x1405bfb23 4881ec90010000 sub rsp, 0x190
0x1405bfb2a 488dac2480000000 lea rbp, [rsp + 0x80]
0x1405bfb32 48c78508010000feffffff mov qword ptr [rbp + 0x108], 0xfffffffffffffffe
0x1405bfb3d 4889ce mov rsi, rcx
0x1405bfb40 0f57c0 xorps xmm0, xmm0
0x1405bfb43 0f1145e8 movups xmmword ptr [rbp - 0x18], xmm0
0x1405bfb47 0f114500 movups xmmword ptr [rbp], xmm0
0x1405bfb4b 0f114518 movups xmmword ptr [rbp + 0x18], xmm0
0x1405bfb4f 0f114530 movups xmmword ptr [rbp + 0x30], xmm0
0x1405bfb53 0f114548 movups xmmword ptr [rbp + 0x48], xmm0
0x1405bfb57 c7455800000000 mov dword ptr [rbp + 0x58], 0
0x1405bfb5e 0f118580000000 movups xmmword ptr [rbp + 0x80], xmm0
0x1405bfb65 0f118598000000 movups xmmword ptr [rbp + 0x98], xmm0
0x1405bfb6c 0f1185b0000000 movups xmmword ptr [rbp + 0xb0], xmm0
0x1405bfb73 0f1185c8000000 movups xmmword ptr [rbp + 0xc8], xmm0
0x1405bfb7a 0f1185e0000000 movups xmmword ptr [rbp + 0xe0], xmm0
0x1405bfb81 c785f000000000000000 mov dword ptr [rbp + 0xf0], 0
0x1405bfb8b 48c745a800000000 mov qword ptr [rbp - 0x58], 0
0x1405bfb93 48c745c800000000 mov qword ptr [rbp - 0x38], 0
0x1405bfb9b 48c745d008000000 mov qword ptr [rbp - 0x30], 8
0x1405bfba3 48c745d800000000 mov qword ptr [rbp - 0x28], 0
0x1405bfbab 48c745e010000000 mov qword ptr [rbp - 0x20], 0x10
0x1405bfbb3 48c745f810000000 mov qword ptr [rbp - 8], 0x10
0x1405bfbbb 48c7451010000000 mov qword ptr [rbp + 0x10], 0x10
0x1405bfbc3 48c7452810000000 mov qword ptr [rbp + 0x28], 0x10
0x1405bfbcb 48c7454010000000 mov qword ptr [rbp + 0x40], 0x10
0x1405bfbd3 48b80000000000000080 movabs rax, 0x8000000000000000
0x1405bfbdd 48894560 mov qword ptr [rbp + 0x60], rax
0x1405bfbe1 488d05d5594402 lea rax, [rip + 0x24459d5]
0x1405bfbe8 48894568 mov qword ptr [rbp + 0x68], rax
0x1405bfbec 48c7457045000000 mov qword ptr [rbp + 0x70], 0x45
0x1405bfbf4 48c7457810000000 mov qword ptr [rbp + 0x78], 0x10
0x1405bfbfc 48c7859000000010000000 mov qword ptr [rbp + 0x90], 0x10
0x1405bfc07 48c785a800000010000000 mov qword ptr [rbp + 0xa8], 0x10
0x1405bfc12 48c785c000000010000000 mov qword ptr [rbp + 0xc0], 0x10
0x1405bfc1d 48c785d800000010000000 mov qword ptr [rbp + 0xd8], 0x10
0x1405bfc28 c785f800000000000000 mov dword ptr [rbp + 0xf8], 0
0x1405bfc32 66c785fc0000000100 mov word ptr [rbp + 0xfc], 1
0x1405bfc3b c7850001000000000000 mov dword ptr [rbp + 0x100], 0
0x1405bfc45 0fb605bd4f0703 movzx eax, byte ptr [rip + 0x3074fbd]
0x1405bfc4c b960010000 mov ecx, 0x160
0x1405bfc51 ba08000000 mov edx, 8
0x1405bfc56 e8b53f0500 call 0x140613c10
0x1405bfc5b 4885c0 test rax, rax
0x1405bfc5e 7435 je 0x1405bfc95
0x1405bfc60 4889c7 mov rdi, rax
0x1405bfc63 488d55a8 lea rdx, [rbp - 0x58]
0x1405bfc67 41b860010000 mov r8d, 0x160
0x1405bfc6d 4889c1 mov rcx, rax
0x1405bfc70 e88b3f3602 call 0x142923c00
0x1405bfc75 4c8d0514574402 lea r8, [rip + 0x2445714]
0x1405bfc7c 4889f1 mov rcx, rsi
0x1405bfc7f 4889fa mov rdx, rdi
0x1405bfc82 e8894e7200 call 0x140ce4b10
0x1405bfc87 4889f0 mov rax, rsi
0x1405bfc8a 4881c490010000 add rsp, 0x190
0x1405bfc91 5f pop rdi
0x1405bfc92 5e pop rsi
0x1405bfc93 5d pop rbp
0x1405bfc94 c3 ret 
0x1405bfc95 b908000000 mov ecx, 8
0x1405bfc9a ba60010000 mov edx, 0x160
0x1405bfc9f e85c933102 call 0x1428d9000
0x1405bfca4 0f0b ud2 
0x1405bfca6 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1405bfcb0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1405bfcb5 55 push rbp
0x1405bfcb6 56 push rsi
0x1405bfcb7 57 push rdi
0x1405bfcb8 4883ec20 sub rsp, 0x20
0x1405bfcbc 488daa80000000 lea rbp, [rdx + 0x80]
0x1405bfcc3 488d4da8 lea rcx, [rbp - 0x58]
0x1405bfcc7 e8a4610200 call 0x1405e5e70
0x1405bfccc 90 nop 
0x1405bfccd 4883c420 add rsp, 0x20
0x1405bfcd1 5f pop rdi
0x1405bfcd2 5e pop rsi
0x1405bfcd3 5d pop rbp
0x1405bfcd4 c3 ret 
