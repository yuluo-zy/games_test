0x1415d26d0 55 push rbp
0x1415d26d1 4156 push r14
0x1415d26d3 56 push rsi
0x1415d26d4 57 push rdi
0x1415d26d5 53 push rbx
0x1415d26d6 4881ecb0000000 sub rsp, 0xb0
0x1415d26dd 488dac2480000000 lea rbp, [rsp + 0x80]
0x1415d26e5 48c74528feffffff mov qword ptr [rbp + 0x28], 0xfffffffffffffffe
0x1415d26ed 4889ce mov rsi, rcx
0x1415d26f0 0f104218 movups xmm0, xmmword ptr [rdx + 0x18]
0x1415d26f4 0f104a28 movups xmm1, xmmword ptr [rdx + 0x28]
0x1415d26f8 0f294d00 movaps xmmword ptr [rbp], xmm1
0x1415d26fc 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
0x1415d2700 4c8b0a mov r9, qword ptr [rdx]
0x1415d2703 488b4a08 mov rcx, qword ptr [rdx + 8]
0x1415d2707 488b5210 mov rdx, qword ptr [rdx + 0x10]
0x1415d270b 4883fa05 cmp rdx, 5
0x1415d270f 752d jne 0x1415d273e
0x1415d2711 8b01 mov eax, dword ptr [rcx]
0x1415d2713 33058fd16b01 xor eax, dword ptr [rip + 0x16bd18f]
0x1415d2719 440fb64104 movzx r8d, byte ptr [rcx + 4]
0x1415d271e 44320587d16b01 xor r8b, byte ptr [rip + 0x16bd187]
0x1415d2725 450fb6c0 movzx r8d, r8b
0x1415d2729 4109c0 or r8d, eax
0x1415d272c 0f859e000000 jne 0x1415d27d0
0x1415d2732 31db xor ebx, ebx
0x1415d2734 4531f6 xor r14d, r14d
0x1415d2737 4d85c9 test r9, r9
0x1415d273a 7520 jne 0x1415d275c
0x1415d273c eb2c jmp 0x1415d276a
0x1415d273e 4883fa04 cmp rdx, 4
0x1415d2742 0f8588000000 jne 0x1415d27d0
0x1415d2748 8b01 mov eax, dword ptr [rcx]
0x1415d274a 3b055dd16b01 cmp eax, dword ptr [rip + 0x16bd15d]
0x1415d2750 757e jne 0x1415d27d0
0x1415d2752 b301 mov bl, 1
0x1415d2754 4531f6 xor r14d, r14d
0x1415d2757 4d85c9 test r9, r9
0x1415d275a 740e je 0x1415d276a
0x1415d275c 41b801000000 mov r8d, 1
0x1415d2762 4c89ca mov rdx, r9
0x1415d2765 e8b61404ff call 0x140613c20
0x1415d276a 0f2845f0 movaps xmm0, xmmword ptr [rbp - 0x10]
0x1415d276e 0f284d00 movaps xmm1, xmmword ptr [rbp]
0x1415d2772 0f294db0 movaps xmmword ptr [rbp - 0x50], xmm1
0x1415d2776 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
0x1415d277a 4584f6 test r14b, r14b
0x1415d277d 7416 je 0x1415d2795
0x1415d277f 48897e08 mov qword ptr [rsi + 8], rdi
0x1415d2783 c60602 mov byte ptr [rsi], 2
0x1415d2786 c6452700 mov byte ptr [rbp + 0x27], 0
0x1415d278a 488d4da0 lea rcx, [rbp - 0x60]
0x1415d278e e89dbc2bff call 0x14088e430
0x1415d2793 eb2a jmp 0x1415d27bf
0x1415d2795 0f2845f0 movaps xmm0, xmmword ptr [rbp - 0x10]
0x1415d2799 0f284d00 movaps xmm1, xmmword ptr [rbp]
0x1415d279d 0f114de0 movups xmmword ptr [rbp - 0x20], xmm1
0x1415d27a1 0f1145d0 movups xmmword ptr [rbp - 0x30], xmm0
0x1415d27a5 881e mov byte ptr [rsi], bl
0x1415d27a7 0f1045c9 movups xmm0, xmmword ptr [rbp - 0x37]
0x1415d27ab 0f104dd9 movups xmm1, xmmword ptr [rbp - 0x27]
0x1415d27af 0f114601 movups xmmword ptr [rsi + 1], xmm0
0x1415d27b3 0f114e11 movups xmmword ptr [rsi + 0x11], xmm1
0x1415d27b7 488b45e8 mov rax, qword ptr [rbp - 0x18]
0x1415d27bb 48894620 mov qword ptr [rsi + 0x20], rax
0x1415d27bf 4889f0 mov rax, rsi
0x1415d27c2 4881c4b0000000 add rsp, 0xb0
0x1415d27c9 5b pop rbx
0x1415d27ca 5f pop rdi
0x1415d27cb 5e pop rsi
0x1415d27cc 415e pop r14
0x1415d27ce 5d pop rbp
0x1415d27cf c3 ret 
0x1415d27d0 4c894d18 mov qword ptr [rbp + 0x18], r9
0x1415d27d4 4c8d05ddd06b01 lea r8, [rip + 0x16bd0dd]
0x1415d27db 41b902000000 mov r9d, 2
0x1415d27e1 48894d10 mov qword ptr [rbp + 0x10], rcx
0x1415d27e5 e836763201 call 0x1428f9e20
0x1415d27ea 4889c7 mov rdi, rax
0x1415d27ed 41b601 mov r14b, 1
0x1415d27f0 4c8b4d18 mov r9, qword ptr [rbp + 0x18]
0x1415d27f4 488b4d10 mov rcx, qword ptr [rbp + 0x10]
0x1415d27f8 4d85c9 test r9, r9
0x1415d27fb 0f855bffffff jne 0x1415d275c
0x1415d2801 e964ffffff jmp 0x1415d276a
0x1415d2806 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1415d2810 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415d2815 55 push rbp
0x1415d2816 4156 push r14
0x1415d2818 56 push rsi
0x1415d2819 57 push rdi
0x1415d281a 53 push rbx
0x1415d281b 4883ec20 sub rsp, 0x20
0x1415d281f 488daa80000000 lea rbp, [rdx + 0x80]
0x1415d2826 48837d1800 cmp qword ptr [rbp + 0x18], 0
0x1415d282b 7413 je 0x1415d2840
0x1415d282d 41b801000000 mov r8d, 1
0x1415d2833 488b4d10 mov rcx, qword ptr [rbp + 0x10]
0x1415d2837 488b5518 mov rdx, qword ptr [rbp + 0x18]
0x1415d283b e8e01304ff call 0x140613c20
0x1415d2840 c6452701 mov byte ptr [rbp + 0x27], 1
0x1415d2844 4883c420 add rsp, 0x20
0x1415d2848 5b pop rbx
0x1415d2849 5f pop rdi
0x1415d284a 5e pop rsi
0x1415d284b 415e pop r14
0x1415d284d 5d pop rbp
0x1415d284e c3 ret 
0x1415d284f 90 nop 
0x1415d2850 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415d2855 55 push rbp
0x1415d2856 4156 push r14
0x1415d2858 56 push rsi
0x1415d2859 57 push rdi
0x1415d285a 53 push rbx
0x1415d285b 4883ec20 sub rsp, 0x20
0x1415d285f 488daa80000000 lea rbp, [rdx + 0x80]
0x1415d2866 807d2700 cmp byte ptr [rbp + 0x27], 0
0x1415d286a 7409 je 0x1415d2875
0x1415d286c 488d4df0 lea rcx, [rbp - 0x10]
0x1415d2870 e8bbbb2bff call 0x14088e430
0x1415d2875 90 nop 
0x1415d2876 4883c420 add rsp, 0x20
0x1415d287a 5b pop rbx
0x1415d287b 5f pop rdi
0x1415d287c 5e pop rsi
0x1415d287d 415e pop r14
0x1415d287f 5d pop rbp
0x1415d2880 c3 ret 
