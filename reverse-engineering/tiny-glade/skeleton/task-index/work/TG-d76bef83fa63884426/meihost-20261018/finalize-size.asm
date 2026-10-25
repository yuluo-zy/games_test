0x142340890 push r15
0x142340892 push r14
0x142340894 push r13
0x142340896 push r12
0x142340898 push rsi
0x142340899 push rdi
0x14234089a push rbp
0x14234089b push rbx
0x14234089c sub rsp, 0xb8
0x1423408a3 movaps xmmword ptr [rsp + 0xa0], xmm7
0x1423408ab movaps xmmword ptr [rsp + 0x90], xmm6
0x1423408b3 mov r10, qword ptr [rcx]
0x1423408b6 mov r14, qword ptr [rcx + 8]
0x1423408ba mov eax, dword ptr [rcx + 0x14]
0x1423408bd mov dword ptr [rsp + 0x5c], eax
0x1423408c1 mov r15, qword ptr [r10 + 0xf8]
0x1423408c8 mov rax, qword ptr [r10 + 0x100]
0x1423408cf lea r12, [r15 + rax*4]
0x1423408d3 mov qword ptr [rsp + 0x60], r10
0x1423408d8 movzx eax, byte ptr [r10 + 0x108]
0x1423408e0 mov byte ptr [rsp + 0x37], al
0x1423408e4 mov rax, qword ptr [r8]
0x1423408e7 mov qword ptr [rsp + 0x70], rax
0x1423408ec mov rax, qword ptr [r9]
0x1423408ef mov qword ptr [rsp + 0x88], rax
0x1423408f7 mov eax, dword ptr [rdx + 0x1c]
0x1423408fa mov dword ptr [rsp + 0x58], eax
0x1423408fe mov rax, qword ptr [rdx]
0x142340901 mov qword ptr [rsp + 0x80], rax
0x142340909 mov rax, qword ptr [rdx + 0x10]
0x14234090d mov qword ptr [rsp + 0x78], rax
0x142340912 mov eax, 8
0x142340917 mov qword ptr [rsp + 0x68], rax
0x14234091c movss xmm6, dword ptr [rip + 0x5e4fe0]
0x142340924 mov dword ptr [rsp + 0x30], 0
0x14234092c mov qword ptr [rsp + 0x40], 0
0x142340935 xor ebp, ebp
0x142340937 xor esi, esi
0x142340939 xor r13d, r13d
0x14234093c mov qword ptr [rsp + 0x38], 0
0x142340945 mov dword ptr [rsp + 0x2c], 0
0x14234094d jmp 0x14234098d
0x14234094f nop 
0x142340950 inc rbp
0x142340953 mov eax, dword ptr [rsp + 0x58]
0x142340957 mov rcx, qword ptr [rsp + 0x78]
0x14234095c mov dword ptr [rcx], eax
0x14234095e mov edx, dword ptr [rbx]
0x142340960 mov r8d, dword ptr [rbx + 4]
0x142340964 lea rax, [rsp + 0x54]
0x142340969 mov qword ptr [rsp + 0x20], rax
0x14234096e mov rcx, qword ptr [rsp + 0x80]
0x142340976 lea r9, [rsp + 0x50]
0x14234097b call 0x142376120
0x142340980 mov eax, dword ptr [rsp + 0x5c]
0x142340984 mov dword ptr [rdi], eax
0x142340986 mov dword ptr [rbx + 0x10], 0x3f800000
0x14234098d cmp byte ptr [rsp + 0x37], 0
0x142340992 je 0x142340a30
0x142340998 cmp rbp, rsi
0x14234099b jne 0x142340b77
0x1423409a1 nop word ptr cs:[rax + rax]
0x1423409b0 cmp r15, r12
0x1423409b3 je 0x142340cf4
0x1423409b9 mov ecx, dword ptr [r15]
0x1423409bc add r15, 4
0x1423409c0 mov rax, qword ptr [r14 + 0x1a8]
0x1423409c7 lea rcx, [rcx + rcx*8]
0x1423409cb mov rsi, qword ptr [rax + rcx*8 + 0x10]
0x1423409d0 test rsi, rsi
0x1423409d3 je 0x1423409b0
0x1423409d5 lea rcx, [rax + rcx*8]
0x1423409d9 mov rdx, qword ptr [rcx + 0x40]
0x1423409dd mov rax, qword ptr [rcx + 0x18]
0x1423409e1 mov rcx, qword ptr [rcx + 0x38]
0x1423409e5 mov r9, qword ptr [rsp + 0x60]
0x1423409ea mov r8, qword ptr [r9 + 0x118]
0x1423409f1 cmp rdx, r8
0x1423409f4 jbe 0x142340b0c
0x1423409fa mov r8, qword ptr [rcx + r8*8]
0x1423409fe test r8, r8
0x142340a01 je 0x142340b0c
0x142340a07 not r8
0x142340a0a lea r8, [r8 + r8*2]
0x142340a0e shl r8, 4
0x142340a12 mov r8, qword ptr [rax + r8 + 0x10]
0x142340a17 mov qword ptr [rsp + 0x40], r8
0x142340a1c mov r8b, 1
0x142340a1f mov dword ptr [rsp + 0x30], r8d
0x142340a24 jmp 0x142340b14
0x142340a29 nop dword ptr [rax]
0x142340a30 cmp rbp, rsi
0x142340a33 jne 0x142340c2b
0x142340a39 cmp r15, r12
0x142340a3c je 0x142340cf4
0x142340a42 mov rax, qword ptr [r14 + 0x100]
0x142340a49 nop dword ptr [rax]
0x142340a50 mov ecx, dword ptr [r15]
0x142340a53 lea rcx, [rcx + rcx*4]
0x142340a57 shl rcx, 5
0x142340a5b mov rsi, qword ptr [rax + rcx + 0x58]
0x142340a60 test rsi, rsi
0x142340a63 jne 0x142340a73
0x142340a65 add r15, 4
0x142340a69 cmp r15, r12
0x142340a6c jne 0x142340a50
0x142340a6e jmp 0x142340cf4
0x142340a73 add rax, rcx
0x142340a76 mov ecx, dword ptr [rax + 0x94]
0x142340a7c mov rdx, qword ptr [r14 + 0x1a8]
0x142340a83 lea r9, [rcx + rcx*8]
0x142340a87 mov r8, qword ptr [rdx + r9*8 + 0x40]
0x142340a8c mov rcx, qword ptr [rdx + r9*8 + 0x18]
0x142340a91 mov rdx, qword ptr [rdx + r9*8 + 0x38]
0x142340a96 mov rdi, qword ptr [rsp + 0x60]
0x142340a9b mov r10, qword ptr [rdi + 0x118]
0x142340aa2 mov r9, qword ptr [rax + 0x88]
0x142340aa9 cmp r9, r10
0x142340aac jbe 0x142340b7e
0x142340ab2 mov r11, qword ptr [rax + 0x80]
0x142340ab9 cmp qword ptr [r11 + r10*8], 0
0x142340abe je 0x142340b7e
0x142340ac4 mov r11b, 1
0x142340ac7 mov dword ptr [rsp + 0x30], r11d
0x142340acc cmp r8, r10
0x142340acf jbe 0x142340b94
0x142340ad5 mov r10, qword ptr [rdx + r10*8]
0x142340ad9 test r10, r10
0x142340adc je 0x142340b94
0x142340ae2 not r10
0x142340ae5 lea r10, [r10 + r10*2]
0x142340ae9 shl r10, 4
0x142340aed mov r10, qword ptr [rcx + r10 + 0x10]
0x142340af2 mov qword ptr [rsp + 0x40], r10
0x142340af7 mov r10, qword ptr [rdi + 0x120]
0x142340afe cmp r9, r10
0x142340b01 ja 0x142340ba9
0x142340b07 jmp 0x142340be4
0x142340b0c mov dword ptr [rsp + 0x30], 0
0x142340b14 mov r8, qword ptr [r9 + 0x110]
0x142340b1b mov r8, qword ptr [rcx + r8*8]
0x142340b1f not r8
0x142340b22 lea r8, [r8 + r8*2]
0x142340b26 shl r8, 4
0x142340b2a mov r13, qword ptr [rax + r8 + 0x10]
0x142340b2f mov r8, qword ptr [rax + r8 + 0x28]
0x142340b34 mov qword ptr [rsp + 0x48], r8
0x142340b39 mov r8, qword ptr [r9 + 0x120]
0x142340b40 cmp rdx, r8
0x142340b43 jbe 0x142340b6d
0x142340b45 mov rcx, qword ptr [rcx + r8*8]
0x142340b49 test rcx, rcx
0x142340b4c je 0x142340b6d
0x142340b4e not rcx
0x142340b51 lea rcx, [rcx + rcx*2]
0x142340b55 shl rcx, 4
0x142340b59 mov rax, qword ptr [rax + rcx + 0x10]
0x142340b5e mov qword ptr [rsp + 0x38], rax
0x142340b63 mov al, 1
0x142340b65 mov dword ptr [rsp + 0x2c], eax
0x142340b69 xor ebp, ebp
0x142340b6b jmp 0x142340b77
0x142340b6d xor ebp, ebp
0x142340b6f mov dword ptr [rsp + 0x2c], 0
0x142340b77 mov ecx, ebp
0x142340b79 jmp 0x142340c3b
0x142340b7e mov dword ptr [rsp + 0x30], 0
0x142340b86 mov r10, qword ptr [rdi + 0x120]
0x142340b8d cmp r9, r10
0x142340b90 ja 0x142340ba9
0x142340b92 jmp 0x142340be4
0x142340b94 mov qword ptr [rsp + 0x40], 0
0x142340b9d mov r10, qword ptr [rdi + 0x120]
0x142340ba4 cmp r9, r10
0x142340ba7 jbe 0x142340be4
0x142340ba9 mov r9, qword ptr [rax + 0x80]
0x142340bb0 cmp qword ptr [r9 + r10*8], 0
0x142340bb5 je 0x142340be4
0x142340bb7 mov r9b, 1
0x142340bba mov dword ptr [rsp + 0x2c], r9d
0x142340bbf cmp r8, r10
0x142340bc2 jbe 0x142340bee
0x142340bc4 mov r8, qword ptr [rdx + r10*8]
0x142340bc8 test r8, r8
0x142340bcb je 0x142340bee
0x142340bcd not r8
0x142340bd0 lea r8, [r8 + r8*2]
0x142340bd4 shl r8, 4
0x142340bd8 mov r8, qword ptr [rcx + r8 + 0x10]
0x142340bdd mov qword ptr [rsp + 0x38], r8
0x142340be2 jmp 0x142340bf7
0x142340be4 mov dword ptr [rsp + 0x2c], 0
0x142340bec jmp 0x142340bf7
0x142340bee mov qword ptr [rsp + 0x38], 0
0x142340bf7 mov r8, qword ptr [rdi + 0x110]
0x142340bfe mov rdx, qword ptr [rdx + r8*8]
0x142340c02 not rdx
0x142340c05 lea rdx, [rdx + rdx*2]
0x142340c09 shl rdx, 4
0x142340c0d mov r13, qword ptr [rcx + rdx + 0x10]
0x142340c12 mov rcx, qword ptr [rcx + rdx + 0x28]
0x142340c17 mov qword ptr [rsp + 0x48], rcx
0x142340c1c add r15, 4
0x142340c20 mov rax, qword ptr [rax + 0x50]
0x142340c24 mov qword ptr [rsp + 0x68], rax
0x142340c29 xor ebp, ebp
0x142340c2b mov rax, rbp
0x142340c2e shl rax, 4
0x142340c32 mov rcx, qword ptr [rsp + 0x68]
0x142340c37 mov ecx, dword ptr [rcx + rax + 8]
0x142340c3b lea rax, [rcx + rcx*4]
0x142340c3f lea rbx, [rax*4]
0x142340c47 add rbx, r13
0x142340c4a mov rax, qword ptr [rsp + 0x48]
0x142340c4f lea rdi, [rax + rcx*4]
0x142340c53 test byte ptr [rsp + 0x30], 1
0x142340c58 mov eax, 0
0x142340c5d cmovne rax, qword ptr [rsp + 0x40]
0x142340c63 imul rcx, rcx, 0x258
0x142340c6a add rcx, qword ptr [rsp + 0x38]
0x142340c6f test byte ptr [rsp + 0x2c], 1
0x142340c74 mov edx, 0
0x142340c79 cmove rcx, rdx
0x142340c7d movaps xmm0, xmm6
0x142340c80 test rax, rax
0x142340c83 je 0x142340c92
0x142340c85 mov rax, qword ptr [rsp + 0x70]
0x142340c8a movss xmm0, dword ptr [rax + 0xe8]
0x142340c92 movsd xmm1, qword ptr [rbx + 8]
0x142340c97 movss xmm2, dword ptr [rbx + 0x10]
0x142340c9c movsldup xmm2, xmm2
0x142340ca0 mulps xmm2, xmm1
0x142340ca3 movsldup xmm0, xmm0
0x142340ca7 mulps xmm0, xmm2
0x142340caa mov rax, qword ptr [rsp + 0x88]
0x142340cb2 movss xmm1, dword ptr [rax]
0x142340cb6 movsldup xmm7, xmm1
0x142340cba mulps xmm7, xmm0
0x142340cbd movlps qword ptr [rsp + 0x50], xmm7
0x142340cc2 test rcx, rcx
0x142340cc5 je 0x142340950
0x142340ccb call 0x140b90f50
0x142340cd0 movsd xmm0, qword ptr [rax]
0x142340cd4 movaps xmm1, xmm7
0x142340cd7 cmpunordps xmm1, xmm7
0x142340cdb movaps xmm2, xmm0
0x142340cde andps xmm2, xmm1
0x142340ce1 maxps xmm0, xmm7
0x142340ce4 andnps xmm1, xmm0
0x142340ce7 orps xmm1, xmm2
0x142340cea movlps qword ptr [rsp + 0x50], xmm1
0x142340cef jmp 0x142340950
0x142340cf4 movaps xmm6, xmmword ptr [rsp + 0x90]
0x142340cfc movaps xmm7, xmmword ptr [rsp + 0xa0]
0x142340d04 add rsp, 0xb8
0x142340d0b pop rbx
0x142340d0c pop rbp
0x142340d0d pop rdi
0x142340d0e pop rsi
0x142340d0f pop r12
0x142340d11 pop r13
0x142340d13 pop r14
0x142340d15 pop r15
0x142340d17 ret 
