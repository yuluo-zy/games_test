0x141594b70 55 push rbp
0x141594b71 4157 push r15
0x141594b73 4156 push r14
0x141594b75 4154 push r12
0x141594b77 56 push rsi
0x141594b78 57 push rdi
0x141594b79 53 push rbx
0x141594b7a 4881ec90000000 sub rsp, 0x90
0x141594b81 488dac2480000000 lea rbp, [rsp + 0x80]
0x141594b89 48c74508feffffff mov qword ptr [rbp + 8], 0xfffffffffffffffe
0x141594b91 4d89c4 mov r12, r8
0x141594b94 4d85c0 test r8, r8
0x141594b97 0f88a4010000 js 0x141594d41
0x141594b9d 4c89cf mov rdi, r9
0x141594ba0 4889ce mov rsi, rcx
0x141594ba3 7444 je 0x141594be9
0x141594ba5 4989d6 mov r14, rdx
0x141594ba8 0fb6055a000a02 movzx eax, byte ptr [rip + 0x20a005a]
0x141594baf 41bf01000000 mov r15d, 1
0x141594bb5 ba01000000 mov edx, 1
0x141594bba 4c89e1 mov rcx, r12
0x141594bbd e84ef007ff call 0x140613c10
0x141594bc2 4885c0 test rax, rax
0x141594bc5 0f8479010000 je 0x141594d44
0x141594bcb 4889c3 mov rbx, rax
0x141594bce 4889c1 mov rcx, rax
0x141594bd1 4c89f2 mov rdx, r14
0x141594bd4 4d89e0 mov r8, r12
0x141594bd7 e824f03801 call 0x142923c00
0x141594bdc 488b16 mov rdx, qword ptr [rsi]
0x141594bdf 4889d0 mov rax, rdx
0x141594be2 48f7d8 neg rax
0x141594be5 7112 jno 0x141594bf9
0x141594be7 eb21 jmp 0x141594c0a
0x141594be9 bb01000000 mov ebx, 1
0x141594bee 488b16 mov rdx, qword ptr [rsi]
0x141594bf1 4889d0 mov rax, rdx
0x141594bf4 48f7d8 neg rax
0x141594bf7 7011 jo 0x141594c0a
0x141594bf9 730f jae 0x141594c0a
0x141594bfb 488b4e08 mov rcx, qword ptr [rsi + 8]
0x141594bff 41b801000000 mov r8d, 1
0x141594c05 e816f007ff call 0x140613c20
0x141594c0a 48895e08 mov qword ptr [rsi + 8], rbx
0x141594c0e 4c896610 mov qword ptr [rsi + 0x10], r12
0x141594c12 48b80000000000000080 movabs rax, 0x8000000000000000
0x141594c1c 488906 mov qword ptr [rsi], rax
0x141594c1f 488b4608 mov rax, qword ptr [rsi + 8]
0x141594c23 488945d0 mov qword ptr [rbp - 0x30], rax
0x141594c27 488b4610 mov rax, qword ptr [rsi + 0x10]
0x141594c2b 488945d8 mov qword ptr [rbp - 0x28], rax
0x141594c2f 4c8965c8 mov qword ptr [rbp - 0x38], r12
0x141594c33 803f00 cmp byte ptr [rdi], 0
0x141594c36 0fb605ccff0902 movzx eax, byte ptr [rip + 0x209ffcc]
0x141594c3d 7430 je 0x141594c6f
0x141594c3f bf06000000 mov edi, 6
0x141594c44 b906000000 mov ecx, 6
0x141594c49 ba01000000 mov edx, 1
0x141594c4e e8bdef07ff call 0x140613c10
0x141594c53 4885c0 test rax, rax
0x141594c56 0f84fa000000 je 0x141594d56
0x141594c5c 0fb70d6eae6f01 movzx ecx, word ptr [rip + 0x16fae6e]
0x141594c63 66894804 mov word ptr [rax + 4], cx
0x141594c67 8b0d60ae6f01 mov ecx, dword ptr [rip + 0x16fae60]
0x141594c6d eb2d jmp 0x141594c9c
0x141594c6f bf05000000 mov edi, 5
0x141594c74 b905000000 mov ecx, 5
0x141594c79 ba01000000 mov edx, 1
0x141594c7e e88def07ff call 0x140613c10
0x141594c83 4885c0 test rax, rax
0x141594c86 0f84e6000000 je 0x141594d72
0x141594c8c 0fb60d39ae6f01 movzx ecx, byte ptr [rip + 0x16fae39]
0x141594c93 884804 mov byte ptr [rax + 4], cl
0x141594c96 8b0d2cae6f01 mov ecx, dword ptr [rip + 0x16fae2c]
0x141594c9c 8908 mov dword ptr [rax], ecx
0x141594c9e c645a803 mov byte ptr [rbp - 0x58], 3
0x141594ca2 48897db0 mov qword ptr [rbp - 0x50], rdi
0x141594ca6 488945b8 mov qword ptr [rbp - 0x48], rax
0x141594caa 48897dc0 mov qword ptr [rbp - 0x40], rdi
0x141594cae 4883c618 add rsi, 0x18
0x141594cb2 488d4de0 lea rcx, [rbp - 0x20]
0x141594cb6 4c8d45c8 lea r8, [rbp - 0x38]
0x141594cba 4c8d4da8 lea r9, [rbp - 0x58]
0x141594cbe 4889f2 mov rdx, rsi
0x141594cc1 e86a6deaff call 0x14143ba30
0x141594cc6 0fb645e0 movzx eax, byte ptr [rbp - 0x20]
0x141594cca 4883f806 cmp rax, 6
0x141594cce 7754 ja 0x141594d24
0x141594cd0 488d0d59396e01 lea rcx, [rip + 0x16e3959]
0x141594cd7 48630481 movsxd rax, dword ptr [rcx + rax*4]
0x141594cdb 4801c8 add rax, rcx
0x141594cde ffe0 jmp rax
0x141594ce0 488b55e8 mov rdx, qword ptr [rbp - 0x18]
0x141594ce4 4885d2 test rdx, rdx
0x141594ce7 7444 je 0x141594d2d
0x141594ce9 488b4df0 mov rcx, qword ptr [rbp - 0x10]
0x141594ced 41b801000000 mov r8d, 1
0x141594cf3 eb28 jmp 0x141594d1d
0x141594cf5 488b4df0 mov rcx, qword ptr [rbp - 0x10]
0x141594cf9 488b55f8 mov rdx, qword ptr [rbp - 8]
0x141594cfd 48894d00 mov qword ptr [rbp], rcx
0x141594d01 e86a402fff call 0x140888d70
0x141594d06 488b55e8 mov rdx, qword ptr [rbp - 0x18]
0x141594d0a 4885d2 test rdx, rdx
0x141594d0d 741e je 0x141594d2d
0x141594d0f 48c1e205 shl rdx, 5
0x141594d13 41b808000000 mov r8d, 8
0x141594d19 488b4d00 mov rcx, qword ptr [rbp]
0x141594d1d e8feee07ff call 0x140613c20
0x141594d22 eb09 jmp 0x141594d2d
0x141594d24 488d4de8 lea rcx, [rbp - 0x18]
0x141594d28 e8134230ff call 0x140898f40
0x141594d2d 31c0 xor eax, eax
0x141594d2f 4881c490000000 add rsp, 0x90
0x141594d36 5b pop rbx
0x141594d37 5f pop rdi
0x141594d38 5e pop rsi
0x141594d39 415c pop r12
0x141594d3b 415e pop r14
0x141594d3d 415f pop r15
0x141594d3f 5d pop rbp
0x141594d40 c3 ret 
0x141594d41 4531ff xor r15d, r15d
0x141594d44 4c8d058d346e01 lea r8, [rip + 0x16e348d]
0x141594d4b 4c89f9 mov rcx, r15
0x141594d4e 4c89e2 mov rdx, r12
0x141594d51 e88d423401 call 0x1428d8fe3
0x141594d56 4c896500 mov qword ptr [rbp], r12
0x141594d5a 4c8d05a7a56f01 lea r8, [rip + 0x16fa5a7]
0x141594d61 b901000000 mov ecx, 1
0x141594d66 ba06000000 mov edx, 6
0x141594d6b e873423401 call 0x1428d8fe3
0x141594d70 eb1a jmp 0x141594d8c
0x141594d72 4c896500 mov qword ptr [rbp], r12
0x141594d76 4c8d058ba56f01 lea r8, [rip + 0x16fa58b]
0x141594d7d b901000000 mov ecx, 1
0x141594d82 ba05000000 mov edx, 5
0x141594d87 e857423401 call 0x1428d8fe3
0x141594d8c 0f0b ud2 
0x141594d8e 6690 nop 
0x141594d90 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141594d95 55 push rbp
0x141594d96 4157 push r15
0x141594d98 4156 push r14
0x141594d9a 4154 push r12
0x141594d9c 56 push rsi
0x141594d9d 57 push rdi
0x141594d9e 53 push rbx
0x141594d9f 4883ec20 sub rsp, 0x20
0x141594da3 488daa80000000 lea rbp, [rdx + 0x80]
0x141594daa 48837d0000 cmp qword ptr [rbp], 0
0x141594daf 7413 je 0x141594dc4
0x141594db1 488b4dd0 mov rcx, qword ptr [rbp - 0x30]
0x141594db5 41b801000000 mov r8d, 1
0x141594dbb 488b5500 mov rdx, qword ptr [rbp]
0x141594dbf e85cee07ff call 0x140613c20
0x141594dc4 90 nop 
0x141594dc5 4883c420 add rsp, 0x20
0x141594dc9 5b pop rbx
0x141594dca 5f pop rdi
0x141594dcb 5e pop rsi
0x141594dcc 415c pop r12
0x141594dce 415e pop r14
0x141594dd0 415f pop r15
0x141594dd2 5d pop rbp
0x141594dd3 c3 ret 
0x141594dd4 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x141594de0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141594de5 55 push rbp
0x141594de6 4157 push r15
0x141594de8 4156 push r14
0x141594dea 4154 push r12
0x141594dec 56 push rsi
0x141594ded 57 push rdi
0x141594dee 53 push rbx
0x141594def 4883ec20 sub rsp, 0x20
0x141594df3 488daa80000000 lea rbp, [rdx + 0x80]
0x141594dfa 488b55e8 mov rdx, qword ptr [rbp - 0x18]
0x141594dfe 4885d2 test rdx, rdx
0x141594e01 7413 je 0x141594e16
0x141594e03 48c1e205 shl rdx, 5
0x141594e07 41b808000000 mov r8d, 8
0x141594e0d 488b4d00 mov rcx, qword ptr [rbp]
0x141594e11 e80aee07ff call 0x140613c20
0x141594e16 90 nop 
0x141594e17 4883c420 add rsp, 0x20
0x141594e1b 5b pop rbx
0x141594e1c 5f pop rdi
0x141594e1d 5e pop rsi
0x141594e1e 415c pop r12
0x141594e20 415e pop r14
0x141594e22 415f pop r15
0x141594e24 5d pop rbp
0x141594e25 c3 ret 
