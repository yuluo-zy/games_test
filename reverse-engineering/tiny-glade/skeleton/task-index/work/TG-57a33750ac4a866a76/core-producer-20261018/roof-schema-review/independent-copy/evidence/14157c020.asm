0x14157c020 55 push rbp
0x14157c021 4157 push r15
0x14157c023 4156 push r14
0x14157c025 4154 push r12
0x14157c027 56 push rsi
0x14157c028 57 push rdi
0x14157c029 53 push rbx
0x14157c02a 4883ec60 sub rsp, 0x60
0x14157c02e 488d6c2460 lea rbp, [rsp + 0x60]
0x14157c033 48c745f8feffffff mov qword ptr [rbp - 8], 0xfffffffffffffffe
0x14157c03b 4889d7 mov rdi, rdx
0x14157c03e 4889ce mov rsi, rcx
0x14157c041 488d1500b46f01 lea rdx, [rip + 0x16fb400]
0x14157c048 488d4dd8 lea rcx, [rbp - 0x28]
0x14157c04c 41b804000000 mov r8d, 4
0x14157c052 41b902000000 mov r9d, 2
0x14157c058 e843a360ff call 0x140b863a0
0x14157c05d 488b4dd8 mov rcx, qword ptr [rbp - 0x28]
0x14157c061 488b45e0 mov rax, qword ptr [rbp - 0x20]
0x14157c065 4889ca mov rdx, rcx
0x14157c068 48f7da neg rdx
0x14157c06b 710c jno 0x14157c079
0x14157c06d 48894608 mov qword ptr [rsi + 8], rax
0x14157c071 c60606 mov byte ptr [rsi], 6
0x14157c074 e92f010000 jmp 0x14157c1a8
0x14157c079 488b5de8 mov rbx, qword ptr [rbp - 0x18]
0x14157c07d 48894dc0 mov qword ptr [rbp - 0x40], rcx
0x14157c081 488945c8 mov qword ptr [rbp - 0x38], rax
0x14157c085 48895dd0 mov qword ptr [rbp - 0x30], rbx
0x14157c089 f30f1007 movss xmm0, dword ptr [rdi]
0x14157c08d f30f5ac0 cvtss2sd xmm0, xmm0
0x14157c091 66480f7ec2 movq rdx, xmm0
0x14157c096 49beffffffffffffff7f movabs r14, 0x7fffffffffffffff
0x14157c0a0 4c21f2 and rdx, r14
0x14157c0a3 49bf000000000000f07f movabs r15, 0x7ff0000000000000
0x14157c0ad 4c39fa cmp rdx, r15
0x14157c0b0 0f9cc2 setl dl
0x14157c0b3 00d2 add dl, dl
0x14157c0b5 8855d8 mov byte ptr [rbp - 0x28], dl
0x14157c0b8 48c745e002000000 mov qword ptr [rbp - 0x20], 2
0x14157c0c0 f20f1145e8 movsd qword ptr [rbp - 0x18], xmm0
0x14157c0c5 4839cb cmp rbx, rcx
0x14157c0c8 7518 jne 0x14157c0e2
0x14157c0ca 488d15c7c16f01 lea rdx, [rip + 0x16fc1c7]
0x14157c0d1 488d4dc0 lea rcx, [rbp - 0x40]
0x14157c0d5 e8865f60ff call 0x140b82060
0x14157c0da 488b4dc0 mov rcx, qword ptr [rbp - 0x40]
0x14157c0de 488b45c8 mov rax, qword ptr [rbp - 0x38]
0x14157c0e2 4889da mov rdx, rbx
0x14157c0e5 48c1e205 shl rdx, 5
0x14157c0e9 0f1045d8 movups xmm0, xmmword ptr [rbp - 0x28]
0x14157c0ed 0f104de8 movups xmm1, xmmword ptr [rbp - 0x18]
0x14157c0f1 0f114c1010 movups xmmword ptr [rax + rdx + 0x10], xmm1
0x14157c0f6 0f110410 movups xmmword ptr [rax + rdx], xmm0
0x14157c0fa 4c8d6301 lea r12, [rbx + 1]
0x14157c0fe 4c8965d0 mov qword ptr [rbp - 0x30], r12
0x14157c102 f30f104704 movss xmm0, dword ptr [rdi + 4]
0x14157c107 f30f5ac0 cvtss2sd xmm0, xmm0
0x14157c10b 66480f7ec2 movq rdx, xmm0
0x14157c110 4c21f2 and rdx, r14
0x14157c113 4c39fa cmp rdx, r15
0x14157c116 0f9cc2 setl dl
0x14157c119 00d2 add dl, dl
0x14157c11b 8855d8 mov byte ptr [rbp - 0x28], dl
0x14157c11e 48c745e002000000 mov qword ptr [rbp - 0x20], 2
0x14157c126 f20f1145e8 movsd qword ptr [rbp - 0x18], xmm0
0x14157c12b 4939cc cmp r12, rcx
0x14157c12e 7514 jne 0x14157c144
0x14157c130 488d1561c16f01 lea rdx, [rip + 0x16fc161]
0x14157c137 488d4dc0 lea rcx, [rbp - 0x40]
0x14157c13b e8205f60ff call 0x140b82060
0x14157c140 488b45c8 mov rax, qword ptr [rbp - 0x38]
0x14157c144 49c1e405 shl r12, 5
0x14157c148 0f1045d8 movups xmm0, xmmword ptr [rbp - 0x28]
0x14157c14c 0f104de8 movups xmm1, xmmword ptr [rbp - 0x18]
0x14157c150 420f114c2010 movups xmmword ptr [rax + r12 + 0x10], xmm1
0x14157c156 420f110420 movups xmmword ptr [rax + r12], xmm0
0x14157c15b 4883c302 add rbx, 2
0x14157c15f 48895dd0 mov qword ptr [rbp - 0x30], rbx
0x14157c163 48895def mov qword ptr [rbp - 0x11], rbx
0x14157c167 0f1045c0 movups xmm0, xmmword ptr [rbp - 0x40]
0x14157c16b 0f1145df movups xmmword ptr [rbp - 0x21], xmm0
0x14157c16f c60604 mov byte ptr [rsi], 4
0x14157c172 0fb645d8 movzx eax, byte ptr [rbp - 0x28]
0x14157c176 8b4dd9 mov ecx, dword ptr [rbp - 0x27]
0x14157c179 0fb755dd movzx edx, word ptr [rbp - 0x23]
0x14157c17d 440fb645df movzx r8d, byte ptr [rbp - 0x21]
0x14157c182 4c8b4de0 mov r9, qword ptr [rbp - 0x20]
0x14157c186 4c8b55e7 mov r10, qword ptr [rbp - 0x19]
0x14157c18a 4c8b5def mov r11, qword ptr [rbp - 0x11]
0x14157c18e 884601 mov byte ptr [rsi + 1], al
0x14157c191 894e02 mov dword ptr [rsi + 2], ecx
0x14157c194 66895606 mov word ptr [rsi + 6], dx
0x14157c198 44884608 mov byte ptr [rsi + 8], r8b
0x14157c19c 4c894e09 mov qword ptr [rsi + 9], r9
0x14157c1a0 4c895610 mov qword ptr [rsi + 0x10], r10
0x14157c1a4 4c895e18 mov qword ptr [rsi + 0x18], r11
0x14157c1a8 4889f0 mov rax, rsi
0x14157c1ab 4883c460 add rsp, 0x60
0x14157c1af 5b pop rbx
0x14157c1b0 5f pop rdi
0x14157c1b1 5e pop rsi
0x14157c1b2 415c pop r12
0x14157c1b4 415e pop r14
0x14157c1b6 415f pop r15
0x14157c1b8 5d pop rbp
0x14157c1b9 c3 ret 
0x14157c1ba 660f1f440000 nop word ptr [rax + rax]
0x14157c1c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14157c1c5 55 push rbp
0x14157c1c6 4157 push r15
0x14157c1c8 4156 push r14
0x14157c1ca 4154 push r12
0x14157c1cc 56 push rsi
0x14157c1cd 57 push rdi
0x14157c1ce 53 push rbx
0x14157c1cf 4883ec20 sub rsp, 0x20
0x14157c1d3 488d6a60 lea rbp, [rdx + 0x60]
0x14157c1d7 488d4dc0 lea rcx, [rbp - 0x40]
0x14157c1db e8b0d130ff call 0x140889390
0x14157c1e0 90 nop 
0x14157c1e1 4883c420 add rsp, 0x20
0x14157c1e5 5b pop rbx
0x14157c1e6 5f pop rdi
0x14157c1e7 5e pop rsi
0x14157c1e8 415c pop r12
0x14157c1ea 415e pop r14
0x14157c1ec 415f pop r15
0x14157c1ee 5d pop rbp
0x14157c1ef c3 ret 
0x14157c1f0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14157c1f5 55 push rbp
0x14157c1f6 4157 push r15
0x14157c1f8 4156 push r14
0x14157c1fa 4154 push r12
0x14157c1fc 56 push rsi
0x14157c1fd 57 push rdi
0x14157c1fe 53 push rbx
0x14157c1ff 4883ec20 sub rsp, 0x20
0x14157c203 488d6a60 lea rbp, [rdx + 0x60]
0x14157c207 488d4dd8 lea rcx, [rbp - 0x28]
0x14157c20b e880182eff call 0x14085da90
0x14157c210 90 nop 
0x14157c211 4883c420 add rsp, 0x20
0x14157c215 5b pop rbx
0x14157c216 5f pop rdi
0x14157c217 5e pop rsi
0x14157c218 415c pop r12
0x14157c21a 415e pop r14
0x14157c21c 415f pop r15
0x14157c21e 5d pop rbp
0x14157c21f c3 ret 
0x14157c220 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14157c225 55 push rbp
0x14157c226 4157 push r15
0x14157c228 4156 push r14
0x14157c22a 4154 push r12
0x14157c22c 56 push rsi
0x14157c22d 57 push rdi
0x14157c22e 53 push rbx
0x14157c22f 4883ec20 sub rsp, 0x20
0x14157c233 488d6a60 lea rbp, [rdx + 0x60]
0x14157c237 488d4dd8 lea rcx, [rbp - 0x28]
0x14157c23b e850182eff call 0x14085da90
0x14157c240 90 nop 
0x14157c241 4883c420 add rsp, 0x20
0x14157c245 5b pop rbx
0x14157c246 5f pop rdi
0x14157c247 5e pop rsi
0x14157c248 415c pop r12
0x14157c24a 415e pop r14
0x14157c24c 415f pop r15
0x14157c24e 5d pop rbp
0x14157c24f c3 ret 
