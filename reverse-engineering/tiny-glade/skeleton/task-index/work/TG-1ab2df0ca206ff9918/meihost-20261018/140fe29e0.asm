0x140fe29e0 push rsi
0x140fe29e1 sub rsp, 0x30
0x140fe29e5 lea rax, [r8 - 3]
0x140fe29e9 cmp rax, 4
0x140fe29ed ja 0x140fe2a80
0x140fe29f3 lea r9, [rip + 0x1bdf8b2] ; 0x142bc22ac af0742fed40742fe570742fe730742fe f32=-6.447753167049102e+37
0x140fe29fa movsxd rax, dword ptr [r9 + rax*4]
0x140fe29fe add rax, r9
0x140fe2a01 jmp rax
0x140fe2a03 mov eax, 0x72617453
0x140fe2a08 xor eax, dword ptr [rdx]
0x140fe2a0a movzx r9d, byte ptr [rdx + 4]
0x140fe2a0f xor r9d, 0x74
0x140fe2a13 or r9d, eax
0x140fe2a16 jne 0x140fe2a80
0x140fe2a18 mov word ptr [rcx], 0x132
0x140fe2a1d jmp 0x140fe2a77
0x140fe2a1f mov eax, 0x746e6543
0x140fe2a24 xor eax, dword ptr [rdx]
0x140fe2a26 movzx r9d, word ptr [rdx + 4]
0x140fe2a2b xor r9d, 0x7265
0x140fe2a32 or r9d, eax
0x140fe2a35 jne 0x140fe2a80
0x140fe2a37 mov word ptr [rcx], 0x232
0x140fe2a3c jmp 0x140fe2a77
0x140fe2a3e mov eax, 0x65727453
0x140fe2a43 xor eax, dword ptr [rdx]
0x140fe2a45 mov r9d, 0x68637465
0x140fe2a4b xor r9d, dword ptr [rdx + 3]
0x140fe2a4f or r9d, eax
0x140fe2a52 jne 0x140fe2a80
0x140fe2a54 mov word ptr [rcx], 0x32
0x140fe2a59 jmp 0x140fe2a77
0x140fe2a5b movzx eax, word ptr [rdx]
0x140fe2a5e xor eax, 0x6e45
0x140fe2a63 movzx r9d, byte ptr [rdx + 2]
0x140fe2a68 xor r9d, 0x64
0x140fe2a6c or r9w, ax
0x140fe2a70 jne 0x140fe2a80
0x140fe2a72 mov word ptr [rcx], 0x332
0x140fe2a77 mov rax, rcx
0x140fe2a7a add rsp, 0x30
0x140fe2a7e pop rsi
0x140fe2a7f ret 
0x140fe2a80 mov qword ptr [rsp + 0x20], 4
0x140fe2a89 lea r9, [rip + 0x1bdf548] ; 0x142bc1fd8 ce1fbc42010000000700000000000000 f32=94.06211853027344
0x140fe2a90 mov rsi, rcx
0x140fe2a93 call 0x1428ca640
0x140fe2a98 mov rax, rsi
0x140fe2a9b add rsp, 0x30
0x140fe2a9f pop rsi
0x140fe2aa0 ret 