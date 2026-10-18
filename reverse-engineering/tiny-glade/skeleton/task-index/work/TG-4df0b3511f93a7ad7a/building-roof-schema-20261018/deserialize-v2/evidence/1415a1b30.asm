0x1415a1b30 55 push rbp
0x1415a1b31 56 push rsi
0x1415a1b32 4881ec88000000 sub rsp, 0x88
0x1415a1b39 488dac2480000000 lea rbp, [rsp + 0x80]
0x1415a1b41 48c74500feffffff mov qword ptr [rbp], 0xfffffffffffffffe
0x1415a1b49 4889ce mov rsi, rcx
0x1415a1b4c 0fb602 movzx eax, byte ptr [rdx]
0x1415a1b4f 83f804 cmp eax, 4
0x1415a1b52 488955f0 mov qword ptr [rbp - 0x10], rdx
0x1415a1b56 7460 je 0x1415a1bb8
0x1415a1b58 83f805 cmp eax, 5
0x1415a1b5b 0f85c2000000 jne 0x1415a1c23
0x1415a1b61 488b4218 mov rax, qword ptr [rdx + 0x18]
0x1415a1b65 488945b0 mov qword ptr [rbp - 0x50], rax
0x1415a1b69 0f104208 movups xmm0, xmmword ptr [rdx + 8]
0x1415a1b6d 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
0x1415a1b71 c645ff00 mov byte ptr [rbp - 1], 0
0x1415a1b75 c645fe01 mov byte ptr [rbp - 2], 1
0x1415a1b79 488d55a0 lea rdx, [rbp - 0x60]
0x1415a1b7d 4889f1 mov rcx, rsi
0x1415a1b80 e84b630200 call 0x1415c7ed0
0x1415a1b85 b101 mov cl, 1
0x1415a1b87 31c0 xor eax, eax
0x1415a1b89 4c8b45f0 mov r8, qword ptr [rbp - 0x10]
0x1415a1b8d 410fb610 movzx edx, byte ptr [r8]
0x1415a1b91 83fa04 cmp edx, 4
0x1415a1b94 7457 je 0x1415a1bed
0x1415a1b96 83fa05 cmp edx, 5
0x1415a1b99 0f85c0000000 jne 0x1415a1c5f
0x1415a1b9f 84c0 test al, al
0x1415a1ba1 0f84d5000000 je 0x1415a1c7c
0x1415a1ba7 4983c008 add r8, 8
0x1415a1bab 4c89c1 mov rcx, r8
0x1415a1bae e88d732fff call 0x140898f40
0x1415a1bb3 e9c4000000 jmp 0x1415a1c7c
0x1415a1bb8 488b4218 mov rax, qword ptr [rdx + 0x18]
0x1415a1bbc 488945d0 mov qword ptr [rbp - 0x30], rax
0x1415a1bc0 0f104208 movups xmm0, xmmword ptr [rdx + 8]
0x1415a1bc4 0f2945c0 movaps xmmword ptr [rbp - 0x40], xmm0
0x1415a1bc8 c645ff01 mov byte ptr [rbp - 1], 1
0x1415a1bcc c645fe00 mov byte ptr [rbp - 2], 0
0x1415a1bd0 488d55c0 lea rdx, [rbp - 0x40]
0x1415a1bd4 4889f1 mov rcx, rsi
0x1415a1bd7 e8449a0000 call 0x1415ab620
0x1415a1bdc b001 mov al, 1
0x1415a1bde 31c9 xor ecx, ecx
0x1415a1be0 4c8b45f0 mov r8, qword ptr [rbp - 0x10]
0x1415a1be4 410fb610 movzx edx, byte ptr [r8]
0x1415a1be8 83fa04 cmp edx, 4
0x1415a1beb 75a9 jne 0x1415a1b96
0x1415a1bed 84c9 test cl, cl
0x1415a1bef 0f8487000000 je 0x1415a1c7c
0x1415a1bf5 498b4810 mov rcx, qword ptr [r8 + 0x10]
0x1415a1bf9 498b5018 mov rdx, qword ptr [r8 + 0x18]
0x1415a1bfd 48894de0 mov qword ptr [rbp - 0x20], rcx
0x1415a1c01 e86a712eff call 0x140888d70
0x1415a1c06 488b45f0 mov rax, qword ptr [rbp - 0x10]
0x1415a1c0a 488b5008 mov rdx, qword ptr [rax + 8]
0x1415a1c0e 4885d2 test rdx, rdx
0x1415a1c11 7469 je 0x1415a1c7c
0x1415a1c13 48c1e205 shl rdx, 5
0x1415a1c17 41b808000000 mov r8d, 8
0x1415a1c1d 488b4de0 mov rcx, qword ptr [rbp - 0x20]
0x1415a1c21 eb54 jmp 0x1415a1c77
0x1415a1c23 c645ff01 mov byte ptr [rbp - 1], 1
0x1415a1c27 c645fe01 mov byte ptr [rbp - 2], 1
0x1415a1c2b 4c8d05a6776d01 lea r8, [rip + 0x16d77a6]
0x1415a1c32 4889d1 mov rcx, rdx
0x1415a1c35 488d55ef lea rdx, [rbp - 0x11]
0x1415a1c39 e8227e3501 call 0x1428f9a60
0x1415a1c3e 48894608 mov qword ptr [rsi + 8], rax
0x1415a1c42 c70601000000 mov dword ptr [rsi], 1
0x1415a1c48 b001 mov al, 1
0x1415a1c4a b101 mov cl, 1
0x1415a1c4c 4c8b45f0 mov r8, qword ptr [rbp - 0x10]
0x1415a1c50 410fb610 movzx edx, byte ptr [r8]
0x1415a1c54 83fa04 cmp edx, 4
0x1415a1c57 0f8539ffffff jne 0x1415a1b96
0x1415a1c5d eb8e jmp 0x1415a1bed
0x1415a1c5f 80fa03 cmp dl, 3
0x1415a1c62 7218 jb 0x1415a1c7c
0x1415a1c64 498b5008 mov rdx, qword ptr [r8 + 8]
0x1415a1c68 4885d2 test rdx, rdx
0x1415a1c6b 740f je 0x1415a1c7c
0x1415a1c6d 498b4810 mov rcx, qword ptr [r8 + 0x10]
0x1415a1c71 41b801000000 mov r8d, 1
0x1415a1c77 e8a41f07ff call 0x140613c20
0x1415a1c7c 4889f0 mov rax, rsi
0x1415a1c7f 4881c488000000 add rsp, 0x88
0x1415a1c86 5e pop rsi
0x1415a1c87 5d pop rbp
0x1415a1c88 c3 ret 
0x1415a1c89 0f1f8000000000 nop dword ptr [rax]
0x1415a1c90 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415a1c95 55 push rbp
0x1415a1c96 56 push rsi
0x1415a1c97 4883ec28 sub rsp, 0x28
0x1415a1c9b 488daa80000000 lea rbp, [rdx + 0x80]
0x1415a1ca2 488b45f0 mov rax, qword ptr [rbp - 0x10]
0x1415a1ca6 488b5008 mov rdx, qword ptr [rax + 8]
0x1415a1caa 4885d2 test rdx, rdx
0x1415a1cad 7413 je 0x1415a1cc2
0x1415a1caf 48c1e205 shl rdx, 5
0x1415a1cb3 41b808000000 mov r8d, 8
0x1415a1cb9 488b4de0 mov rcx, qword ptr [rbp - 0x20]
0x1415a1cbd e85e1f07ff call 0x140613c20
0x1415a1cc2 90 nop 
0x1415a1cc3 4883c428 add rsp, 0x28
0x1415a1cc7 5e pop rsi
0x1415a1cc8 5d pop rbp
0x1415a1cc9 c3 ret 
0x1415a1cca 660f1f440000 nop word ptr [rax + rax]
0x1415a1cd0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415a1cd5 55 push rbp
0x1415a1cd6 56 push rsi
0x1415a1cd7 4883ec28 sub rsp, 0x28
0x1415a1cdb 488daa80000000 lea rbp, [rdx + 0x80]
0x1415a1ce2 488b45f0 mov rax, qword ptr [rbp - 0x10]
0x1415a1ce6 0fb600 movzx eax, byte ptr [rax]
0x1415a1ce9 83f804 cmp eax, 4
0x1415a1cec 741a je 0x1415a1d08
0x1415a1cee 83f805 cmp eax, 5
0x1415a1cf1 7522 jne 0x1415a1d15
0x1415a1cf3 f645ff01 test byte ptr [rbp - 1], 1
0x1415a1cf7 7415 je 0x1415a1d0e
0x1415a1cf9 488b4df0 mov rcx, qword ptr [rbp - 0x10]
0x1415a1cfd 4883c108 add rcx, 8
0x1415a1d01 e83a722fff call 0x140898f40
0x1415a1d06 eb06 jmp 0x1415a1d0e
0x1415a1d08 f645fe01 test byte ptr [rbp - 2], 1
0x1415a1d0c 7512 jne 0x1415a1d20
0x1415a1d0e 4883c428 add rsp, 0x28
0x1415a1d12 5e pop rsi
0x1415a1d13 5d pop rbp
0x1415a1d14 c3 ret 
0x1415a1d15 488b4df0 mov rcx, qword ptr [rbp - 0x10]
0x1415a1d19 e872bd2bff call 0x14085da90
0x1415a1d1e ebee jmp 0x1415a1d0e
0x1415a1d20 488b4df0 mov rcx, qword ptr [rbp - 0x10]
0x1415a1d24 4883c108 add rcx, 8
0x1415a1d28 e863762eff call 0x140889390
0x1415a1d2d ebdf jmp 0x1415a1d0e
