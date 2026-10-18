141613db0: push rbp
141613db1: push r15
141613db3: push r14
141613db5: push r12
141613db7: push rsi
141613db8: push rdi
141613db9: push rbx
141613dba: sub rsp, 0x120
141613dc1: lea rbp, [rsp + 0x80]
141613dc9: mov qword ptr [rbp + 0x98], 0xfffffffffffffffe
141613dd4: mov rdi, rdx
141613dd7: mov rsi, rcx
141613dda: test r8, r8
141613ddd: je 0x141613e19
141613ddf: mov rbx, r9
141613de2: mov rax, r9
141613de5: and rax, 0xf
141613de9: mov qword ptr [rbp - 0x40], rax
141613ded: jne 0x141613f32
141613df3: mov r15, r8
141613df6: mov rax, r8
141613df9: and rax, 3
141613dfd: mov qword ptr [rbp - 0x40], rax
141613e01: jne 0x141613f43
141613e07: cmp rbx, 0x90
141613e0e: jae 0x141613fcd
141613e14: xor r12d, r12d
141613e17: jmp 0x141613e69
141613e19: movzx eax, byte ptr [rip + 0x2020de9]
141613e20: mov ebx, 0x80
141613e25: mov ecx, 0x80
141613e2a: mov edx, 4
141613e2f: call 0x140613c10
141613e34: test rax, rax
141613e37: je 0x141614037
141613e3d: mov r15, rax
141613e40: movaps xmm0, xmmword ptr [rip + 0x1496399]
141613e47: movups xmmword ptr [rax], xmm0
141613e4a: movups xmmword ptr [rax + 0x10], xmm0
141613e4e: movups xmmword ptr [rax + 0x20], xmm0
141613e52: movups xmmword ptr [rax + 0x30], xmm0
141613e56: movups xmmword ptr [rax + 0x40], xmm0
141613e5a: movups xmmword ptr [rax + 0x50], xmm0
141613e5e: movups xmmword ptr [rax + 0x60], xmm0
141613e62: movups xmmword ptr [rax + 0x70], xmm0
141613e66: mov r12b, 1
141613e69: mov qword ptr [rbp + 0x88], r15
141613e70: mov byte ptr [rbp + 0x97], r12b
141613e77: lea rcx, [rbp + 0x10]
141613e7b: mov edx, 0x80
141613e80: mov r8d, 0x122
141613e86: mov r9b, 1
141613e89: call 0x140bd4de0
141613e8e: mov qword ptr [rbp + 0x88], r15
141613e95: mov byte ptr [rbp + 0x97], r12b
141613e9c: lea r14, [rbp + 0x68]
141613ea0: lea rdx, [rbp + 0x10]
141613ea4: mov r8d, 4
141613eaa: mov rcx, r14
141613ead: call 0x140bd4f20
141613eb2: mov qword ptr [rbp + 0x88], r15
141613eb9: mov byte ptr [rbp + 0x97], r12b
141613ec0: mov qword ptr [rsp + 0x30], rbx
141613ec5: mov qword ptr [rsp + 0x28], r15
141613eca: mov qword ptr [rsp + 0x20], r14
141613ecf: lea r8, [rip + 0x166c096]
141613ed6: lea rcx, [rbp - 0x40]
141613eda: mov r9d, 0x16
141613ee0: mov rdx, rdi
141613ee3: call 0x1416a2dc0
141613ee8: cmp dword ptr [rbp - 0x40], 0xb
141613eec: jne 0x141613f6d
141613eee: mov rax, qword ptr [rbp - 0x18]
141613ef2: mov qword ptr [rsi + 0x20], rax
141613ef6: movups xmm0, xmmword ptr [rbp - 0x38]
141613efa: movups xmm1, xmmword ptr [rbp - 0x28]
141613efe: movups xmmword ptr [rsi + 0x10], xmm1
141613f02: movups xmmword ptr [rsi], xmm0
141613f05: test r12b, r12b
141613f08: je 0x141613f1d
141613f0a: mov edx, 0x80
141613f0f: mov r8d, 4
141613f15: mov rcx, r15
141613f18: call 0x140613c20
141613f1d: mov rax, rsi
141613f20: add rsp, 0x120
141613f27: pop rbx
141613f28: pop rdi
141613f29: pop rsi
141613f2a: pop r12
141613f2c: pop r14
141613f2e: pop r15
141613f30: pop rbp
141613f31: ret 
141613f32: mov qword ptr [rbp + 0x10], 0
141613f3a: lea rax, [rip + 0x16b6007]
141613f41: jmp 0x141613f52
141613f43: mov qword ptr [rbp + 0x10], 0
141613f4b: lea rax, [rip + 0x16b600e]
141613f52: mov qword ptr [rsp + 0x20], rax
141613f57: lea r8, [rip + 0x16b5fe2]
141613f5e: lea rdx, [rbp - 0x40]
141613f62: lea r9, [rbp + 0x10]
141613f66: xor ecx, ecx
141613f68: call 0x1428d9596
141613f6d: mov dword ptr [rbp + 0x84], r12d
141613f74: mov qword ptr [rbp + 0x78], r15
141613f78: movups xmm0, xmmword ptr [rbp]
141613f7c: movaps xmmword ptr [rbp + 0x50], xmm0
141613f80: movups xmm0, xmmword ptr [rbp - 0x40]
141613f84: movups xmm1, xmmword ptr [rbp - 0x30]
141613f88: movups xmm2, xmmword ptr [rbp - 0x20]
141613f8c: movups xmm3, xmmword ptr [rbp - 0x10]
141613f90: movaps xmmword ptr [rbp + 0x40], xmm3
141613f94: movaps xmmword ptr [rbp + 0x30], xmm2
141613f98: movaps xmmword ptr [rbp + 0x20], xmm1
141613f9c: movaps xmmword ptr [rbp + 0x10], xmm0
141613fa0: lea rax, [rip + 0x166c201]
141613fa7: mov qword ptr [rsp + 0x20], rax
141613fac: lea rcx, [rip + 0x166ab35]
141613fb3: lea r9, [rip + 0x166ab0e]
141613fba: lea r8, [rbp + 0x10]
141613fbe: mov edx, 0x2b
141613fc3: call 0x1428d9760
141613fc8: jmp 0x141614054
141613fcd: lea rax, [rip + 0x1489d2b]
141613fd4: mov qword ptr [rbp - 0x40], rax
141613fd8: mov qword ptr [rbp - 0x38], 0x4b
141613fe0: lea rax, [rbp - 0x40]
141613fe4: mov qword ptr [rbp + 0x68], rax
141613fe8: lea rax, [rip - 0x150a15f]
141613fef: mov qword ptr [rbp + 0x70], rax
141613ff3: lea rax, [rip + 0x166b7ce]
141613ffa: mov qword ptr [rbp + 0x10], rax
141613ffe: mov qword ptr [rbp + 0x18], 1
141614006: mov qword ptr [rbp + 0x30], 0
14161400e: lea rax, [rbp + 0x68]
141614012: mov qword ptr [rbp + 0x20], rax
141614016: mov qword ptr [rbp + 0x28], 1
14161401e: mov byte ptr [rbp + 0x97], 0
141614025: lea rdx, [rip + 0x166b7ac]
14161402c: lea rcx, [rbp + 0x10]
141614030: call 0x1428d9390
141614035: jmp 0x141614054
141614037: mov byte ptr [rbp + 0x97], 0
14161403e: lea r8, [rip + 0x166aa6b]
141614045: mov ecx, 4
14161404a: mov edx, 0x80
14161404f: call 0x1428d8fe3
141614054: ud2 
141614056: nop word ptr cs:[rax + rax]
141614060: mov qword ptr [rsp + 0x10], rdx
141614065: push rbp
141614066: push r15
141614068: push r14
14161406a: push r12
14161406c: push rsi
14161406d: push rdi
14161406e: push rbx
14161406f: sub rsp, 0x40
141614073: lea rbp, [rdx + 0x80]
14161407a: mov rsi, qword ptr [rbp + 0x78]
14161407e: mov qword ptr [rbp + 0x88], rsi
141614085: mov edi, dword ptr [rbp + 0x84]
14161408b: mov byte ptr [rbp + 0x97], dil
141614092: lea rcx, [rbp + 0x10]
141614096: call 0x1407cea70
14161409b: mov qword ptr [rbp + 0x88], rsi
1416140a2: mov byte ptr [rbp + 0x97], dil
1416140a9: add rsp, 0x40
1416140ad: pop rbx
1416140ae: pop rdi
1416140af: pop rsi
1416140b0: pop r12
1416140b2: pop r14
1416140b4: pop r15
1416140b6: pop rbp
1416140b7: ret 
1416140b8: nop dword ptr [rax + rax]
1416140c0: mov qword ptr [rsp + 0x10], rdx
1416140c5: push rbp
1416140c6: push r15
1416140c8: push r14
1416140ca: push r12
1416140cc: push rsi
1416140cd: push rdi
1416140ce: push rbx
1416140cf: sub rsp, 0x40
1416140d3: lea rbp, [rdx + 0x80]
1416140da: test byte ptr [rbp + 0x97], 1
1416140e1: je 0x1416140fa
1416140e3: mov rcx, qword ptr [rbp + 0x88]
1416140ea: mov edx, 0x80
1416140ef: mov r8d, 4
1416140f5: call 0x140613c20
1416140fa: nop 
1416140fb: add rsp, 0x40
1416140ff: pop rbx
141614100: pop rdi
141614101: pop rsi
141614102: pop r12
141614104: pop r14
141614106: pop r15
141614108: pop rbp
141614109: ret 
