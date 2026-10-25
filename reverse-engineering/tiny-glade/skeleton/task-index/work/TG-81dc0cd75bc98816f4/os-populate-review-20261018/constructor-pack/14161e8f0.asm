14161e8f0: push rbp
14161e8f1: push r15
14161e8f3: push r14
14161e8f5: push r12
14161e8f7: push rsi
14161e8f8: push rdi
14161e8f9: push rbx
14161e8fa: sub rsp, 0x130
14161e901: lea rbp, [rsp + 0x80]
14161e909: mov qword ptr [rbp + 0xa8], 0xfffffffffffffffe
14161e914: mov rdi, rdx
14161e917: mov rsi, rcx
14161e91a: test r8, r8
14161e91d: je 0x14161e956
14161e91f: mov rbx, r9
14161e922: mov rax, r9
14161e925: and rax, 0xf
14161e929: mov qword ptr [rbp - 0x40], rax
14161e92d: jne 0x14161eb3d
14161e933: mov r15, r8
14161e936: mov rax, r8
14161e939: and rax, 3
14161e93d: mov qword ptr [rbp - 0x40], rax
14161e941: jne 0x14161eb4e
14161e947: cmp rbx, 0x50
14161e94b: jae 0x14161ebe9
14161e951: xor r12d, r12d
14161e954: jmp 0x14161e992
14161e956: movzx eax, byte ptr [rip + 0x20162ac]
14161e95d: mov ebx, 0x40
14161e962: mov ecx, 0x40
14161e967: mov edx, 4
14161e96c: call 0x140613c10
14161e971: test rax, rax
14161e974: je 0x14161ec53
14161e97a: mov r15, rax
14161e97d: xorps xmm0, xmm0
14161e980: movups xmmword ptr [rax + 0x30], xmm0
14161e984: movups xmmword ptr [rax + 0x20], xmm0
14161e988: movups xmmword ptr [rax + 0x10], xmm0
14161e98c: movups xmmword ptr [rax], xmm0
14161e98f: mov r12b, 1
14161e992: mov qword ptr [rbp + 0x98], r15
14161e999: mov byte ptr [rbp + 0xa7], r12b
14161e9a0: lea rcx, [rbp + 0x10]
14161e9a4: mov edx, 0x40
14161e9a9: mov r8d, 0x22
14161e9af: mov r9b, 4
14161e9b2: call 0x140bd4de0
14161e9b7: mov qword ptr [rbp + 0x98], r15
14161e9be: mov byte ptr [rbp + 0xa7], r12b
14161e9c5: lea r14, [rbp + 0x60]
14161e9c9: lea rdx, [rbp + 0x10]
14161e9cd: mov r8d, 4
14161e9d3: mov rcx, r14
14161e9d6: call 0x140bd4f20
14161e9db: mov qword ptr [rbp + 0x98], r15
14161e9e2: mov byte ptr [rbp + 0xa7], r12b
14161e9e9: mov qword ptr [rsp + 0x30], rbx
14161e9ee: mov qword ptr [rsp + 0x28], r15
14161e9f3: mov qword ptr [rsp + 0x20], r14
14161e9f8: lea r8, [rip + 0x1660e92]
14161e9ff: lea rcx, [rbp - 0x40]
14161ea03: mov r9d, 0xf
14161ea09: mov rdx, rdi
14161ea0c: call 0x1416a2dc0
14161ea11: cmp dword ptr [rbp - 0x40], 0xb
14161ea15: jne 0x14161eb78
14161ea1b: mov rax, qword ptr [rbp - 0x18]
14161ea1f: mov qword ptr [rbp + 0x80], rax
14161ea26: movups xmm0, xmmword ptr [rbp - 0x38]
14161ea2a: movups xmm1, xmmword ptr [rbp - 0x28]
14161ea2e: movaps xmmword ptr [rbp + 0x70], xmm1
14161ea32: movaps xmmword ptr [rbp + 0x60], xmm0
14161ea36: test r12b, r12b
14161ea39: je 0x14161ea4e
14161ea3b: mov edx, 0x40
14161ea40: mov r8d, 4
14161ea46: mov rcx, r15
14161ea49: call 0x140613c20
14161ea4e: lea rcx, [rbp + 0x60]
14161ea52: mov rdx, rdi
14161ea55: call 0x140bd4cd0
14161ea5a: test rax, rax
14161ea5d: je 0x14161ebd8
14161ea63: cmp rdx, qword ptr [rbp + 0x78]
14161ea67: jb 0x14161eac2
14161ea69: mov rcx, rdx
14161ea6c: and rcx, 0xf
14161ea70: mov qword ptr [rbp - 0x40], rcx
14161ea74: jne 0x14161eadf
14161ea76: mov rcx, rax
14161ea79: and rcx, 3
14161ea7d: mov qword ptr [rbp - 0x40], rcx
14161ea81: jne 0x14161eb0e
14161ea87: mov rcx, qword ptr [rbp + 0x80]
14161ea8e: mov qword ptr [rsi + 0x30], rcx
14161ea92: movaps xmm0, xmmword ptr [rbp + 0x60]
14161ea96: movaps xmm1, xmmword ptr [rbp + 0x70]
14161ea9a: movups xmmword ptr [rsi + 0x20], xmm1
14161ea9e: movups xmmword ptr [rsi + 0x10], xmm0
14161eaa2: shr rdx, 4
14161eaa6: mov qword ptr [rsi], rax
14161eaa9: mov qword ptr [rsi + 8], rdx
14161eaad: mov rax, rsi
14161eab0: add rsp, 0x130
14161eab7: pop rbx
14161eab8: pop rdi
14161eab9: pop rsi
14161eaba: pop r12
14161eabc: pop r14
14161eabe: pop r15
14161eac0: pop rbp
14161eac1: ret 
14161eac2: lea rcx, [rip + 0x1661727]
14161eac9: lea r8, [rip + 0x1661758]
14161ead0: mov edx, 0x37
14161ead5: call 0x1428d9430
14161eada: jmp 0x14161ec70
14161eadf: mov qword ptr [rbp + 0x10], 0
14161eae7: lea rax, [rip + 0x166175a]
14161eaee: mov qword ptr [rsp + 0x20], rax
14161eaf3: lea r8, [rip + 0x1661746]
14161eafa: lea rdx, [rbp - 0x40]
14161eafe: lea r9, [rbp + 0x10]
14161eb02: xor ecx, ecx
14161eb04: call 0x1428d9596
14161eb09: jmp 0x14161ec70
14161eb0e: mov qword ptr [rbp + 0x10], 0
14161eb16: lea rax, [rip + 0x1661743]
14161eb1d: mov qword ptr [rsp + 0x20], rax
14161eb22: lea r8, [rip + 0x1661717]
14161eb29: lea rdx, [rbp - 0x40]
14161eb2d: lea r9, [rbp + 0x10]
14161eb31: xor ecx, ecx
14161eb33: call 0x1428d9596
14161eb38: jmp 0x14161ec70
14161eb3d: mov qword ptr [rbp + 0x10], 0
14161eb45: lea rax, [rip + 0x16ab3fc]
14161eb4c: jmp 0x14161eb5d
14161eb4e: mov qword ptr [rbp + 0x10], 0
14161eb56: lea rax, [rip + 0x16ab403]
14161eb5d: mov qword ptr [rsp + 0x20], rax
14161eb62: lea r8, [rip + 0x16ab3d7]
14161eb69: lea rdx, [rbp - 0x40]
14161eb6d: lea r9, [rbp + 0x10]
14161eb71: xor ecx, ecx
14161eb73: call 0x1428d9596
14161eb78: mov dword ptr [rbp + 0x94], r12d
14161eb7f: mov qword ptr [rbp - 0x48], r15
14161eb83: movups xmm0, xmmword ptr [rbp]
14161eb87: movaps xmmword ptr [rbp + 0x50], xmm0
14161eb8b: movups xmm0, xmmword ptr [rbp - 0x40]
14161eb8f: movups xmm1, xmmword ptr [rbp - 0x30]
14161eb93: movups xmm2, xmmword ptr [rbp - 0x20]
14161eb97: movups xmm3, xmmword ptr [rbp - 0x10]
14161eb9b: movaps xmmword ptr [rbp + 0x40], xmm3
14161eb9f: movaps xmmword ptr [rbp + 0x30], xmm2
14161eba3: movaps xmmword ptr [rbp + 0x20], xmm1
14161eba7: movaps xmmword ptr [rbp + 0x10], xmm0
14161ebab: lea rax, [rip + 0x16615f6]
14161ebb2: mov qword ptr [rsp + 0x20], rax
14161ebb7: lea rcx, [rip + 0x165ff2a]
14161ebbe: lea r9, [rip + 0x165ff03]
14161ebc5: lea r8, [rbp + 0x10]
14161ebc9: mov edx, 0x2b
14161ebce: call 0x1428d9760
14161ebd3: jmp 0x14161ec70
14161ebd8: lea rcx, [rip + 0x16615f9]
14161ebdf: call 0x1428d9310
14161ebe4: jmp 0x14161ec70
14161ebe9: lea rax, [rip + 0x147f286]
14161ebf0: mov qword ptr [rbp - 0x40], rax
14161ebf4: mov qword ptr [rbp - 0x38], 0x47
14161ebfc: lea rax, [rbp - 0x40]
14161ec00: mov qword ptr [rbp + 0x60], rax
14161ec04: lea rax, [rip - 0x1514d7b]
14161ec0b: mov qword ptr [rbp + 0x68], rax
14161ec0f: lea rax, [rip + 0x1660bb2]
14161ec16: mov qword ptr [rbp + 0x10], rax
14161ec1a: mov qword ptr [rbp + 0x18], 1
14161ec22: mov qword ptr [rbp + 0x30], 0
14161ec2a: lea rax, [rbp + 0x60]
14161ec2e: mov qword ptr [rbp + 0x20], rax
14161ec32: mov qword ptr [rbp + 0x28], 1
14161ec3a: mov byte ptr [rbp + 0xa7], 0
14161ec41: lea rdx, [rip + 0x1660b90]
14161ec48: lea rcx, [rbp + 0x10]
14161ec4c: call 0x1428d9390
14161ec51: jmp 0x14161ec70
14161ec53: mov byte ptr [rbp + 0xa7], 0
14161ec5a: lea r8, [rip + 0x165fe4f]
14161ec61: mov ecx, 4
14161ec66: mov edx, 0x40
14161ec6b: call 0x1428d8fe3
14161ec70: ud2 
14161ec72: nop word ptr cs:[rax + rax]
14161ec80: mov qword ptr [rsp + 0x10], rdx
14161ec85: push rbp
14161ec86: push r15
14161ec88: push r14
14161ec8a: push r12
14161ec8c: push rsi
14161ec8d: push rdi
14161ec8e: push rbx
14161ec8f: sub rsp, 0x40
14161ec93: lea rbp, [rdx + 0x80]
14161ec9a: mov rsi, qword ptr [rbp - 0x48]
14161ec9e: mov qword ptr [rbp + 0x98], rsi
14161eca5: mov edi, dword ptr [rbp + 0x94]
14161ecab: mov byte ptr [rbp + 0xa7], dil
14161ecb2: lea rcx, [rbp + 0x10]
14161ecb6: call 0x1407cea70
14161ecbb: mov qword ptr [rbp + 0x98], rsi
14161ecc2: mov byte ptr [rbp + 0xa7], dil
14161ecc9: add rsp, 0x40
14161eccd: pop rbx
14161ecce: pop rdi
14161eccf: pop rsi
14161ecd0: pop r12
14161ecd2: pop r14
14161ecd4: pop r15
14161ecd6: pop rbp
14161ecd7: ret 
14161ecd8: nop dword ptr [rax + rax]
14161ece0: mov qword ptr [rsp + 0x10], rdx
14161ece5: push rbp
14161ece6: push r15
14161ece8: push r14
14161ecea: push r12
14161ecec: push rsi
14161eced: push rdi
14161ecee: push rbx
14161ecef: sub rsp, 0x40
14161ecf3: lea rbp, [rdx + 0x80]
14161ecfa: test byte ptr [rbp + 0xa7], 1
14161ed01: je 0x14161ed1a
14161ed03: mov rcx, qword ptr [rbp + 0x98]
14161ed0a: mov edx, 0x40
14161ed0f: mov r8d, 4
14161ed15: call 0x140613c20
14161ed1a: nop 
14161ed1b: add rsp, 0x40
14161ed1f: pop rbx
14161ed20: pop rdi
14161ed21: pop rsi
14161ed22: pop r12
14161ed24: pop r14
14161ed26: pop r15
14161ed28: pop rbp
14161ed29: ret 
14161ed2a: nop word ptr [rax + rax]
14161ed30: mov qword ptr [rsp + 0x10], rdx
14161ed35: push rbp
14161ed36: push r15
14161ed38: push r14
14161ed3a: push r12
14161ed3c: push rsi
14161ed3d: push rdi
14161ed3e: push rbx
14161ed3f: sub rsp, 0x40
14161ed43: lea rbp, [rdx + 0x80]
14161ed4a: lea rcx, [rbp + 0x60]
14161ed4e: call 0x140be24b0
14161ed53: nop 
14161ed54: add rsp, 0x40
14161ed58: pop rbx
14161ed59: pop rdi
14161ed5a: pop rsi
14161ed5b: pop r12
14161ed5d: pop r14
14161ed5f: pop r15
14161ed61: pop rbp
14161ed62: ret 
