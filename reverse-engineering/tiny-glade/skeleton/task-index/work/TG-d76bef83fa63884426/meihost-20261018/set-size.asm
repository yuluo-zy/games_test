0x142376120 push r15
0x142376122 push r14
0x142376124 push r12
0x142376126 push rsi
0x142376127 push rdi
0x142376128 push rbx
0x142376129 sub rsp, 0x1c8
0x142376130 mov r14, r9
0x142376133 mov esi, r8d
0x142376136 mov edi, edx
0x142376138 mov rbx, rcx
0x14237613b lea rcx, [rsp + 0x30]
0x142376140 mov rdx, rbx
0x142376143 mov r8d, edi
0x142376146 mov r9d, esi
0x142376149 call 0x140fe7cb0
0x14237614e cmp dword ptr [rsp + 0x30], 4
0x142376153 jne 0x1423761d2
0x142376155 mov r12, qword ptr [rsp + 0x220]
0x14237615d mov rdx, qword ptr [rsp + 0x38]
0x142376162 lea r15, [rsp + 0x70]
0x142376167 mov rcx, r15
0x14237616a call 0x142374fd0
0x14237616f movss xmm0, dword ptr [r14]
0x142376174 movss xmm1, dword ptr [r12]
0x14237617a mov dword ptr [rsp + 0x148], 0
0x142376185 movss dword ptr [rsp + 0x14c], xmm0
0x14237618e mov dword ptr [rsp + 0x150], 0
0x142376199 movss dword ptr [rsp + 0x154], xmm1
0x1423761a2 mov qword ptr [rsp + 0x20], r15
0x1423761a7 lea rcx, [rsp + 0x30]
0x1423761ac mov rdx, rbx
0x1423761af mov r8d, edi
0x1423761b2 mov r9d, esi
0x1423761b5 call 0x140fe7b10
0x1423761ba cmp dword ptr [rsp + 0x30], 4
0x1423761bf jne 0x1423761ef
0x1423761c1 add rsp, 0x1c8
0x1423761c8 pop rbx
0x1423761c9 pop rdi
0x1423761ca pop rsi
0x1423761cb pop r12
0x1423761cd pop r14
0x1423761cf pop r15
0x1423761d1 ret 
0x1423761d2 movups xmm0, xmmword ptr [rsp + 0x30]
0x1423761d7 movups xmm1, xmmword ptr [rsp + 0x40]
0x1423761dc movaps xmmword ptr [rsp + 0x60], xmm1
0x1423761e1 movaps xmmword ptr [rsp + 0x50], xmm0
0x1423761e6 lea rax, [rip + 0xcdcc63]
0x1423761ed jmp 0x14237620a
0x1423761ef movups xmm0, xmmword ptr [rsp + 0x30]
0x1423761f4 movups xmm1, xmmword ptr [rsp + 0x40]
0x1423761f9 movaps xmmword ptr [rsp + 0x60], xmm1
0x1423761fe movaps xmmword ptr [rsp + 0x50], xmm0
0x142376203 lea rax, [rip + 0xcdcc5e]
0x14237620a mov qword ptr [rsp + 0x20], rax
0x14237620f lea rcx, [rip + 0xcdca3a]
0x142376216 lea r9, [rip + 0xcdca13]
0x14237621d lea r8, [rsp + 0x50]
0x142376222 mov edx, 0x2b
0x142376227 call 0x1428d9760
0x14237622c int3 
