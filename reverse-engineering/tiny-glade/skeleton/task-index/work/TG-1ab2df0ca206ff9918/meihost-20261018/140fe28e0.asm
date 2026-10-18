0x140fe28e0 push rsi
0x140fe28e1 sub rsp, 0x30
0x140fe28e5 lea rax, [r8 - 3]
0x140fe28e9 cmp rax, 9
0x140fe28ed ja 0x140fe29b5
0x140fe28f3 lea r9, [rip + 0x1bdf98a] ; 0x142bc2284 7f0642fe310742fee70642fea40642fe f32=-6.447599020736114e+37
0x140fe28fa movsxd rax, dword ptr [r9 + rax*4]
0x140fe28fe add rax, r9
0x140fe2901 jmp rax
0x140fe2903 movzx eax, word ptr [rdx]
0x140fe2906 xor eax, 0x6e45
0x140fe290b movzx r9d, byte ptr [rdx + 2]
0x140fe2910 xor r9d, 0x64
0x140fe2914 or r9w, ax
0x140fe2918 jne 0x140fe29b5
0x140fe291e mov word ptr [rcx], 0x232
0x140fe2923 jmp 0x140fe29ac
0x140fe2928 mov eax, 0x746e6543
0x140fe292d xor eax, dword ptr [rdx]
0x140fe292f movzx r9d, word ptr [rdx + 4]
0x140fe2934 xor r9d, 0x7265
0x140fe293b or r9d, eax
0x140fe293e jne 0x140fe29b5
0x140fe2940 mov word ptr [rcx], 0x132
0x140fe2945 jmp 0x140fe29ac
0x140fe2947 movabs rax, 0x7465426563617053
0x140fe2951 xor rax, qword ptr [rdx]
0x140fe2954 mov r9d, dword ptr [rdx + 8]
0x140fe2958 xor r9, 0x6e656577
0x140fe295f or r9, rax
0x140fe2962 jne 0x140fe29b5
0x140fe2964 mov word ptr [rcx], 0x332
0x140fe2969 jmp 0x140fe29ac
0x140fe296b mov eax, 0x72617453
0x140fe2970 xor eax, dword ptr [rdx]
0x140fe2972 movzx r9d, byte ptr [rdx + 4]
0x140fe2977 xor r9d, 0x74
0x140fe297b or r9d, eax
0x140fe297e jne 0x140fe29b5
0x140fe2980 mov word ptr [rcx], 0x32
0x140fe2985 jmp 0x140fe29ac
0x140fe2987 movabs rax, 0x6576456563617053
0x140fe2991 xor rax, qword ptr [rdx]
0x140fe2994 movabs r9, 0x796c6e6576456563
0x140fe299e xor r9, qword ptr [rdx + 3]
0x140fe29a2 or r9, rax
0x140fe29a5 jne 0x140fe29b5
0x140fe29a7 mov word ptr [rcx], 0x432
0x140fe29ac mov rax, rcx
0x140fe29af add rsp, 0x30
0x140fe29b3 pop rsi
0x140fe29b4 ret 
0x140fe29b5 mov qword ptr [rsp + 0x20], 5
0x140fe29be lea r9, [rip + 0x1bdf5b3] ; 0x142bc1f78 4d1fbc42010000000500000000000000 f32=94.0611343383789
0x140fe29c5 mov rsi, rcx
0x140fe29c8 call 0x1428ca640
0x140fe29cd mov rax, rsi
0x140fe29d0 add rsp, 0x30
0x140fe29d4 pop rsi
0x140fe29d5 ret 