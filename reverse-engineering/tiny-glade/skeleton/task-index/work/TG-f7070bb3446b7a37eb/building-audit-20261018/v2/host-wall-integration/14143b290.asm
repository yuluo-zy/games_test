0x14143b290 55 push rbp
0x14143b291 56 push rsi
0x14143b292 57 push rdi
0x14143b293 4881ec90010000 sub rsp, 0x190
0x14143b29a 488dac2480000000 lea rbp, [rsp + 0x80]
0x14143b2a2 48c78508010000feffffff mov qword ptr [rbp + 0x108], 0xfffffffffffffffe
0x14143b2ad 4889ce mov rsi, rcx
0x14143b2b0 0f57c0 xorps xmm0, xmm0
0x14143b2b3 0f1145e8 movups xmmword ptr [rbp - 0x18], xmm0
0x14143b2b7 0f114500 movups xmmword ptr [rbp], xmm0
0x14143b2bb 0f114518 movups xmmword ptr [rbp + 0x18], xmm0
0x14143b2bf 0f114530 movups xmmword ptr [rbp + 0x30], xmm0
0x14143b2c3 0f114548 movups xmmword ptr [rbp + 0x48], xmm0
0x14143b2c7 c7455800000000 mov dword ptr [rbp + 0x58], 0
0x14143b2ce 0f118580000000 movups xmmword ptr [rbp + 0x80], xmm0
0x14143b2d5 0f118598000000 movups xmmword ptr [rbp + 0x98], xmm0
0x14143b2dc 0f1185b0000000 movups xmmword ptr [rbp + 0xb0], xmm0
0x14143b2e3 0f1185c8000000 movups xmmword ptr [rbp + 0xc8], xmm0
0x14143b2ea 0f1185e0000000 movups xmmword ptr [rbp + 0xe0], xmm0
0x14143b2f1 c785f000000000000000 mov dword ptr [rbp + 0xf0], 0
0x14143b2fb 48c745a000000000 mov qword ptr [rbp - 0x60], 0
0x14143b303 48c745c800000000 mov qword ptr [rbp - 0x38], 0
0x14143b30b 48c745d008000000 mov qword ptr [rbp - 0x30], 8
0x14143b313 48c745d800000000 mov qword ptr [rbp - 0x28], 0
0x14143b31b 48c745e010000000 mov qword ptr [rbp - 0x20], 0x10
0x14143b323 48c745f810000000 mov qword ptr [rbp - 8], 0x10
0x14143b32b 48c7451010000000 mov qword ptr [rbp + 0x10], 0x10
0x14143b333 48c7452810000000 mov qword ptr [rbp + 0x28], 0x10
0x14143b33b 48c7454010000000 mov qword ptr [rbp + 0x40], 0x10
0x14143b343 48b80000000000000080 movabs rax, 0x8000000000000000
0x14143b34d 48894560 mov qword ptr [rbp + 0x60], rax
0x14143b351 488d05f23b8201 lea rax, [rip + 0x1823bf2]
0x14143b358 48894568 mov qword ptr [rbp + 0x68], rax
0x14143b35c 48c745704d000000 mov qword ptr [rbp + 0x70], 0x4d
0x14143b364 48c7457810000000 mov qword ptr [rbp + 0x78], 0x10
0x14143b36c 48c7859000000010000000 mov qword ptr [rbp + 0x90], 0x10
0x14143b377 48c785a800000010000000 mov qword ptr [rbp + 0xa8], 0x10
0x14143b382 48c785c000000010000000 mov qword ptr [rbp + 0xc0], 0x10
0x14143b38d 48c785d800000010000000 mov qword ptr [rbp + 0xd8], 0x10
0x14143b398 c785f800000000000000 mov dword ptr [rbp + 0xf8], 0
0x14143b3a2 66c785fc0000000100 mov word ptr [rbp + 0xfc], 1
0x14143b3ab c7850001000000000000 mov dword ptr [rbp + 0x100], 0
0x14143b3b5 0fb6054d981f02 movzx eax, byte ptr [rip + 0x21f984d]
0x14143b3bc b968010000 mov ecx, 0x168
0x14143b3c1 ba08000000 mov edx, 8
0x14143b3c6 e845881dff call 0x140613c10
0x14143b3cb 4885c0 test rax, rax
0x14143b3ce 7435 je 0x14143b405
0x14143b3d0 4889c7 mov rdi, rax
0x14143b3d3 488d55a0 lea rdx, [rbp - 0x60]
0x14143b3d7 41b868010000 mov r8d, 0x168
0x14143b3dd 4889c1 mov rcx, rax
0x14143b3e0 e81b884e01 call 0x142923c00
0x14143b3e5 4c8d0514388201 lea r8, [rip + 0x1823814]
0x14143b3ec 4889f1 mov rcx, rsi
0x14143b3ef 4889fa mov rdx, rdi
0x14143b3f2 e819978aff call 0x140ce4b10
0x14143b3f7 4889f0 mov rax, rsi
0x14143b3fa 4881c490010000 add rsp, 0x190
0x14143b401 5f pop rdi
0x14143b402 5e pop rsi
0x14143b403 5d pop rbp
0x14143b404 c3 ret 
0x14143b405 b908000000 mov ecx, 8
0x14143b40a ba68010000 mov edx, 0x168
0x14143b40f e8ecdb4901 call 0x1428d9000
0x14143b414 0f0b ud2 
0x14143b416 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14143b420 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14143b425 55 push rbp
0x14143b426 56 push rsi
0x14143b427 57 push rdi
0x14143b428 4883ec20 sub rsp, 0x20
0x14143b42c 488daa80000000 lea rbp, [rdx + 0x80]
0x14143b433 488d4da0 lea rcx, [rbp - 0x60]
0x14143b437 e8d4ba1aff call 0x1405e6f10
0x14143b43c 90 nop 
0x14143b43d 4883c420 add rsp, 0x20
0x14143b441 5f pop rdi
0x14143b442 5e pop rsi
0x14143b443 5d pop rbp
0x14143b444 c3 ret 
