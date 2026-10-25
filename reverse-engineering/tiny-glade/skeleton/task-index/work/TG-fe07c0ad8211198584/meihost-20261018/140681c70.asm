0x140681c70 push r14
0x140681c72 push rsi
0x140681c73 push rdi
0x140681c74 push rbx
0x140681c75 sub rsp, 0x48
0x140681c79 movaps xmmword ptr [rsp + 0x30], xmm6
0x140681c7e mov rsi, rcx
0x140681c81 movss xmm0, dword ptr [rdx + 8]
0x140681c86 movss xmm1, dword ptr [rdx + 0xc]
0x140681c8b movss xmm2, dword ptr [rip + 0x22a3bdd] ; 0x142925870 0000003fc10ffc3e34a6c93900000042 f32=0.5 str=b''
0x140681c93 movss xmm3, dword ptr [rdx + 0x10]
0x140681c98 mulss xmm3, xmm2
0x140681c9c mulss xmm2, dword ptr [rdx + 0x14]
0x140681ca1 movaps xmm4, xmm0
0x140681ca4 subss xmm4, xmm3
0x140681ca8 movaps xmm5, xmm1
0x140681cab subss xmm5, xmm2
0x140681caf addss xmm3, xmm0
0x140681cb3 addss xmm2, xmm1
0x140681cb7 mov rdi, qword ptr [rdx]
0x140681cba movd ebx, xmm4
0x140681cbe movd eax, xmm5
0x140681cc2 shl rax, 0x20
0x140681cc6 or rbx, rax
0x140681cc9 subss xmm3, xmm4
0x140681ccd subss xmm2, xmm5
0x140681cd1 movd r14d, xmm3
0x140681cd6 movd eax, xmm2
0x140681cda shl rax, 0x20
0x140681cde or r14, rax
0x140681ce1 movss xmm6, dword ptr [rdx + 0x18]
0x140681ce6 mov qword ptr [rsp + 0x20], rcx
0x140681ceb mov rcx, rdi
0x140681cee mov rdx, rbx
0x140681cf1 mov r8, r14
0x140681cf4 movaps xmm3, xmm6
0x140681cf7 call 0x140b8f510
0x140681cfc add rsi, 0x18
0x140681d00 mov qword ptr [rsp + 0x20], rsi
0x140681d05 mov rcx, rdi
0x140681d08 mov rdx, rbx
0x140681d0b mov r8, r14
0x140681d0e movaps xmm3, xmm6
0x140681d11 call 0x140b8f510
0x140681d16 movaps xmm6, xmmword ptr [rsp + 0x30]
0x140681d1b add rsp, 0x48
0x140681d1f pop rbx
0x140681d20 pop rdi
0x140681d21 pop rsi
0x140681d22 pop r14
0x140681d24 ret 