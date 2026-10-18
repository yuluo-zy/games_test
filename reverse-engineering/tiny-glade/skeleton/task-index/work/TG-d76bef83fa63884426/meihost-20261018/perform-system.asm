0x141e9eb40 push r15
0x141e9eb42 push r14
0x141e9eb44 push rsi
0x141e9eb45 push rdi
0x141e9eb46 push rbx
0x141e9eb47 sub rsp, 0x80
0x141e9eb4e mov ebx, 1
0x141e9eb53 lock xadd dword ptr [rdx + 0x5b0], ebx
0x141e9eb5b test byte ptr [rcx], 1
0x141e9eb5e je 0x141e9edf5
0x141e9eb64 mov rsi, rcx
0x141e9eb67 mov rax, qword ptr [rcx + 8]
0x141e9eb6b cmp qword ptr [rdx + 0x218], rax
0x141e9eb72 jbe 0x141e9ecaf
0x141e9eb78 mov rcx, qword ptr [rdx + 0x210]
0x141e9eb7f mov rcx, qword ptr [rcx + rax*8]
0x141e9eb83 test rcx, rcx
0x141e9eb86 je 0x141e9ecaf
0x141e9eb8c mov rax, qword ptr [rdx + 0x1e0]
0x141e9eb93 not rcx
0x141e9eb96 lea rcx, [rcx + rcx*2]
0x141e9eb9a shl rcx, 5
0x141e9eb9e cmp qword ptr [rax + rcx + 0x20], 0
0x141e9eba4 je 0x141e9ecaf
0x141e9ebaa add rax, rcx
0x141e9ebad mov rdi, qword ptr [rax + 0x10]
0x141e9ebb1 mov ecx, dword ptr [rdx + 0x5b0]
0x141e9ebb7 mov rcx, qword ptr [rsi + 0x10]
0x141e9ebbb mov r8, qword ptr [rdx + 0x218]
0x141e9ebc2 cmp r8, rcx
0x141e9ebc5 jbe 0x141e9ed34
0x141e9ebcb mov r9, qword ptr [rdx + 0x210]
0x141e9ebd2 mov rcx, qword ptr [r9 + rcx*8]
0x141e9ebd6 test rcx, rcx
0x141e9ebd9 je 0x141e9ed34
0x141e9ebdf mov rdx, qword ptr [rdx + 0x1e0]
0x141e9ebe6 not rcx
0x141e9ebe9 lea rcx, [rcx + rcx*2]
0x141e9ebed shl rcx, 5
0x141e9ebf1 cmp qword ptr [rdx + rcx + 0x20], 0
0x141e9ebf7 je 0x141e9ed34
0x141e9ebfd mov r10, qword ptr [rsi + 0x18]
0x141e9ec01 cmp r8, r10
0x141e9ec04 jbe 0x141e9ed52
0x141e9ec0a mov r10, qword ptr [r9 + r10*8]
0x141e9ec0e test r10, r10
0x141e9ec11 je 0x141e9ed52
0x141e9ec17 not r10
0x141e9ec1a lea r10, [r10 + r10*2]
0x141e9ec1e shl r10, 5
0x141e9ec22 cmp qword ptr [rdx + r10 + 0x20], 0
0x141e9ec28 je 0x141e9ed52
0x141e9ec2e mov r11, qword ptr [rsi + 0x20]
0x141e9ec32 cmp r8, r11
0x141e9ec35 jbe 0x141e9ed70
0x141e9ec3b mov r8, qword ptr [r9 + r11*8]
0x141e9ec3f test r8, r8
0x141e9ec42 je 0x141e9ed70
0x141e9ec48 not r8
0x141e9ec4b lea r8, [r8 + r8*2]
0x141e9ec4f shl r8, 5
0x141e9ec53 cmp qword ptr [rdx + r8 + 0x20], 0
0x141e9ec59 je 0x141e9ed70
0x141e9ec5f add rcx, rdx
0x141e9ec62 mov rcx, qword ptr [rcx + 0x10]
0x141e9ec66 add r10, rdx
0x141e9ec69 mov r14, qword ptr [r10 + 0x10]
0x141e9ec6d add r8, rdx
0x141e9ec70 mov r15, qword ptr [r8 + 0x10]
0x141e9ec74 mov dword ptr [rax + 0x5c], ebx
0x141e9ec77 call 0x140949c10
0x141e9ec7c movaps xmm2, xmm1
0x141e9ec7f movss xmm3, dword ptr [r14 + 0xe8]
0x141e9ec88 mulss xmm3, dword ptr [r15]
0x141e9ec8d mov rcx, rdi
0x141e9ec90 movaps xmm1, xmm0
0x141e9ec93 call 0x141e4c6c0
0x141e9ec98 mov dword ptr [rsi + 0x160], ebx
0x141e9ec9e xor eax, eax
0x141e9eca0 add rsp, 0x80
0x141e9eca7 pop rbx
0x141e9eca8 pop rdi
0x141e9eca9 pop rsi
0x141e9ecaa pop r14
0x141e9ecac pop r15
0x141e9ecae ret 
0x141e9ecaf add rsi, 0xc8
0x141e9ecb6 lea rax, [rip + 0x10f6982]
0x141e9ecbd mov qword ptr [rsp + 0x20], rax
0x141e9ecc2 mov qword ptr [rsp + 0x28], 0x3b
0x141e9eccb mov qword ptr [rsp + 0x30], rsi
0x141e9ecd0 lea rax, [rip - 0x1e63207]
0x141e9ecd7 mov qword ptr [rsp + 0x38], rax
0x141e9ecdc lea rax, [rsp + 0x20]
0x141e9ece1 mov qword ptr [rsp + 0x40], rax
0x141e9ece6 lea rax, [rip - 0x1d94e5d]
0x141e9eced mov qword ptr [rsp + 0x48], rax
0x141e9ecf2 lea rax, [rip + 0x10f69af]
0x141e9ecf9 mov qword ptr [rsp + 0x50], rax
0x141e9ecfe mov qword ptr [rsp + 0x58], 2
0x141e9ed07 mov qword ptr [rsp + 0x70], 0
0x141e9ed10 lea rax, [rsp + 0x30]
0x141e9ed15 mov qword ptr [rsp + 0x60], rax
0x141e9ed1a mov qword ptr [rsp + 0x68], 2
0x141e9ed23 lea rdx, [rip + 0x10f6a96]
0x141e9ed2a lea rcx, [rsp + 0x50]
0x141e9ed2f call 0x1428d9390
0x141e9ed34 add rsi, 0xc8
0x141e9ed3b lea rax, [rip + 0x10f6a06]
0x141e9ed42 mov qword ptr [rsp + 0x20], rax
0x141e9ed47 mov qword ptr [rsp + 0x28], 0x2b
0x141e9ed50 jmp 0x141e9ed8c
0x141e9ed52 add rsi, 0xc8
0x141e9ed59 lea rax, [rip + 0x10f6a3b]
0x141e9ed60 mov qword ptr [rsp + 0x20], rax
0x141e9ed65 mov qword ptr [rsp + 0x28], 0x25
0x141e9ed6e jmp 0x141e9ed8c
0x141e9ed70 add rsi, 0xc8
0x141e9ed77 lea rax, [rip + 0x10f69f5]
0x141e9ed7e mov qword ptr [rsp + 0x20], rax
0x141e9ed83 mov qword ptr [rsp + 0x28], 0x28
0x141e9ed8c mov qword ptr [rsp + 0x30], rsi
0x141e9ed91 lea rax, [rip - 0x1e632c8]
0x141e9ed98 mov qword ptr [rsp + 0x38], rax
0x141e9ed9d lea rax, [rsp + 0x20]
0x141e9eda2 mov qword ptr [rsp + 0x40], rax
0x141e9eda7 lea rax, [rip - 0x1d94f1e]
0x141e9edae mov qword ptr [rsp + 0x48], rax
0x141e9edb3 lea rax, [rip + 0x10f68ee]
0x141e9edba mov qword ptr [rsp + 0x50], rax
0x141e9edbf mov qword ptr [rsp + 0x58], 2
0x141e9edc8 mov qword ptr [rsp + 0x70], 0
0x141e9edd1 lea rax, [rsp + 0x30]
0x141e9edd6 mov qword ptr [rsp + 0x60], rax
0x141e9eddb mov qword ptr [rsp + 0x68], 2
0x141e9ede4 lea rdx, [rip + 0x10f6945]
0x141e9edeb lea rcx, [rsp + 0x50]
0x141e9edf0 call 0x1428d9390
0x141e9edf5 lea rcx, [rip + 0x10f6abc]
0x141e9edfc lea r8, [rip + 0x10f6b15]
0x141e9ee03 mov edx, 0x59
0x141e9ee08 call 0x1428d9330
0x141e9ee0d int3 
