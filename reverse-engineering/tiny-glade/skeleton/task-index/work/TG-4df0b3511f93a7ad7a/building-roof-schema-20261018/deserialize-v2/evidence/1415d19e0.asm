0x1415d19e0 55 push rbp
0x1415d19e1 4156 push r14
0x1415d19e3 56 push rsi
0x1415d19e4 57 push rdi
0x1415d19e5 53 push rbx
0x1415d19e6 4881ecb0000000 sub rsp, 0xb0
0x1415d19ed 488dac2480000000 lea rbp, [rsp + 0x80]
0x1415d19f5 48c74528feffffff mov qword ptr [rbp + 0x28], 0xfffffffffffffffe
0x1415d19fd 4889ce mov rsi, rcx
0x1415d1a00 0f104218 movups xmm0, xmmword ptr [rdx + 0x18]
0x1415d1a04 0f104a28 movups xmm1, xmmword ptr [rdx + 0x28]
0x1415d1a08 0f294d00 movaps xmmword ptr [rbp], xmm1
0x1415d1a0c 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
0x1415d1a10 4c8b0a mov r9, qword ptr [rdx]
0x1415d1a13 488b4a08 mov rcx, qword ptr [rdx + 8]
0x1415d1a17 488b5210 mov rdx, qword ptr [rdx + 0x10]
0x1415d1a1b 4883fa03 cmp rdx, 3
0x1415d1a1f 7530 jne 0x1415d1a51
0x1415d1a21 0fb701 movzx eax, word ptr [rcx]
0x1415d1a24 663305adde6b01 xor ax, word ptr [rip + 0x16bdead]
0x1415d1a2b 440fb64102 movzx r8d, byte ptr [rcx + 2]
0x1415d1a30 443205a3de6b01 xor r8b, byte ptr [rip + 0x16bdea3]
0x1415d1a37 450fb6c0 movzx r8d, r8b
0x1415d1a3b 664109c0 or r8w, ax
0x1415d1a3f 0f85b1000000 jne 0x1415d1af6
0x1415d1a45 31db xor ebx, ebx
0x1415d1a47 4531f6 xor r14d, r14d
0x1415d1a4a 4d85c9 test r9, r9
0x1415d1a4d 7533 jne 0x1415d1a82
0x1415d1a4f eb3f jmp 0x1415d1a90
0x1415d1a51 4883fa05 cmp rdx, 5
0x1415d1a55 0f859b000000 jne 0x1415d1af6
0x1415d1a5b 8b01 mov eax, dword ptr [rcx]
0x1415d1a5d 330578de6b01 xor eax, dword ptr [rip + 0x16bde78]
0x1415d1a63 440fb64104 movzx r8d, byte ptr [rcx + 4]
0x1415d1a68 44320570de6b01 xor r8b, byte ptr [rip + 0x16bde70]
0x1415d1a6f 450fb6c0 movzx r8d, r8b
0x1415d1a73 4109c0 or r8d, eax
0x1415d1a76 757e jne 0x1415d1af6
0x1415d1a78 b301 mov bl, 1
0x1415d1a7a 4531f6 xor r14d, r14d
0x1415d1a7d 4d85c9 test r9, r9
0x1415d1a80 740e je 0x1415d1a90
0x1415d1a82 41b801000000 mov r8d, 1
0x1415d1a88 4c89ca mov rdx, r9
0x1415d1a8b e8902104ff call 0x140613c20
0x1415d1a90 0f2845f0 movaps xmm0, xmmword ptr [rbp - 0x10]
0x1415d1a94 0f284d00 movaps xmm1, xmmword ptr [rbp]
0x1415d1a98 0f294db0 movaps xmmword ptr [rbp - 0x50], xmm1
0x1415d1a9c 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
0x1415d1aa0 4584f6 test r14b, r14b
0x1415d1aa3 7416 je 0x1415d1abb
0x1415d1aa5 48897e08 mov qword ptr [rsi + 8], rdi
0x1415d1aa9 c60602 mov byte ptr [rsi], 2
0x1415d1aac c6452700 mov byte ptr [rbp + 0x27], 0
0x1415d1ab0 488d4da0 lea rcx, [rbp - 0x60]
0x1415d1ab4 e877c92bff call 0x14088e430
0x1415d1ab9 eb2a jmp 0x1415d1ae5
0x1415d1abb 0f2845f0 movaps xmm0, xmmword ptr [rbp - 0x10]
0x1415d1abf 0f284d00 movaps xmm1, xmmword ptr [rbp]
0x1415d1ac3 0f114de0 movups xmmword ptr [rbp - 0x20], xmm1
0x1415d1ac7 0f1145d0 movups xmmword ptr [rbp - 0x30], xmm0
0x1415d1acb 881e mov byte ptr [rsi], bl
0x1415d1acd 0f1045c9 movups xmm0, xmmword ptr [rbp - 0x37]
0x1415d1ad1 0f104dd9 movups xmm1, xmmword ptr [rbp - 0x27]
0x1415d1ad5 0f114601 movups xmmword ptr [rsi + 1], xmm0
0x1415d1ad9 0f114e11 movups xmmword ptr [rsi + 0x11], xmm1
0x1415d1add 488b45e8 mov rax, qword ptr [rbp - 0x18]
0x1415d1ae1 48894620 mov qword ptr [rsi + 0x20], rax
0x1415d1ae5 4889f0 mov rax, rsi
0x1415d1ae8 4881c4b0000000 add rsp, 0xb0
0x1415d1aef 5b pop rbx
0x1415d1af0 5f pop rdi
0x1415d1af1 5e pop rsi
0x1415d1af2 415e pop r14
0x1415d1af4 5d pop rbp
0x1415d1af5 c3 ret 
0x1415d1af6 4c894d18 mov qword ptr [rbp + 0x18], r9
0x1415d1afa 4c8d05dfdd6b01 lea r8, [rip + 0x16bdddf]
0x1415d1b01 41b902000000 mov r9d, 2
0x1415d1b07 48894d10 mov qword ptr [rbp + 0x10], rcx
0x1415d1b0b e810833201 call 0x1428f9e20
0x1415d1b10 4889c7 mov rdi, rax
0x1415d1b13 41b601 mov r14b, 1
0x1415d1b16 4c8b4d18 mov r9, qword ptr [rbp + 0x18]
0x1415d1b1a 488b4d10 mov rcx, qword ptr [rbp + 0x10]
0x1415d1b1e 4d85c9 test r9, r9
0x1415d1b21 0f855bffffff jne 0x1415d1a82
0x1415d1b27 e964ffffff jmp 0x1415d1a90
0x1415d1b2c 0f1f4000 nop dword ptr [rax]
0x1415d1b30 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415d1b35 55 push rbp
0x1415d1b36 4156 push r14
0x1415d1b38 56 push rsi
0x1415d1b39 57 push rdi
0x1415d1b3a 53 push rbx
0x1415d1b3b 4883ec20 sub rsp, 0x20
0x1415d1b3f 488daa80000000 lea rbp, [rdx + 0x80]
0x1415d1b46 48837d1800 cmp qword ptr [rbp + 0x18], 0
0x1415d1b4b 7413 je 0x1415d1b60
0x1415d1b4d 41b801000000 mov r8d, 1
0x1415d1b53 488b4d10 mov rcx, qword ptr [rbp + 0x10]
0x1415d1b57 488b5518 mov rdx, qword ptr [rbp + 0x18]
0x1415d1b5b e8c02004ff call 0x140613c20
0x1415d1b60 c6452701 mov byte ptr [rbp + 0x27], 1
0x1415d1b64 4883c420 add rsp, 0x20
0x1415d1b68 5b pop rbx
0x1415d1b69 5f pop rdi
0x1415d1b6a 5e pop rsi
0x1415d1b6b 415e pop r14
0x1415d1b6d 5d pop rbp
0x1415d1b6e c3 ret 
0x1415d1b6f 90 nop 
0x1415d1b70 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415d1b75 55 push rbp
0x1415d1b76 4156 push r14
0x1415d1b78 56 push rsi
0x1415d1b79 57 push rdi
0x1415d1b7a 53 push rbx
0x1415d1b7b 4883ec20 sub rsp, 0x20
0x1415d1b7f 488daa80000000 lea rbp, [rdx + 0x80]
0x1415d1b86 807d2700 cmp byte ptr [rbp + 0x27], 0
0x1415d1b8a 7409 je 0x1415d1b95
0x1415d1b8c 488d4df0 lea rcx, [rbp - 0x10]
0x1415d1b90 e89bc82bff call 0x14088e430
0x1415d1b95 90 nop 
0x1415d1b96 4883c420 add rsp, 0x20
0x1415d1b9a 5b pop rbx
0x1415d1b9b 5f pop rdi
0x1415d1b9c 5e pop rsi
0x1415d1b9d 415e pop r14
0x1415d1b9f 5d pop rbp
0x1415d1ba0 c3 ret 
