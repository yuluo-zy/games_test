14161f660: push rbp
14161f661: push r15
14161f663: push r14
14161f665: push r12
14161f667: push rsi
14161f668: push rdi
14161f669: push rbx
14161f66a: sub rsp, 0x130
14161f671: lea rbp, [rsp + 0x80]
14161f679: mov qword ptr [rbp + 0xa8], 0xfffffffffffffffe
14161f684: mov rdi, rdx
14161f687: mov rsi, rcx
14161f68a: test r8, r8
14161f68d: je 0x14161f6c9
14161f68f: mov rbx, r9
14161f692: mov rax, r9
14161f695: and rax, 3
14161f699: mov qword ptr [rbp - 0x40], rax
14161f69d: jne 0x14161f8ee
14161f6a3: mov r15, r8
14161f6a6: mov rax, r8
14161f6a9: and rax, 3
14161f6ad: mov qword ptr [rbp - 0x40], rax
14161f6b1: jne 0x14161f8ff
14161f6b7: cmp rbx, 0x8004
14161f6be: jae 0x14161f99a
14161f6c4: xor r12d, r12d
14161f6c7: jmp 0x14161f743
14161f6c9: movzx eax, byte ptr [rip + 0x2015539]
14161f6d0: mov ecx, 0x8000
14161f6d5: mov edx, 4
14161f6da: call 0x140613c10
14161f6df: test rax, rax
14161f6e2: je 0x14161fa04
14161f6e8: mov r15, rax
14161f6eb: mov eax, 0x1c
14161f6f0: xorps xmm0, xmm0
14161f6f3: nop word ptr cs:[rax + rax]
14161f700: movups xmmword ptr [r15 + rax*4 - 0x70], xmm0
14161f706: movups xmmword ptr [r15 + rax*4 - 0x60], xmm0
14161f70c: movups xmmword ptr [r15 + rax*4 - 0x50], xmm0
14161f712: movups xmmword ptr [r15 + rax*4 - 0x40], xmm0
14161f718: movups xmmword ptr [r15 + rax*4 - 0x30], xmm0
14161f71e: movups xmmword ptr [r15 + rax*4 - 0x20], xmm0
14161f724: movups xmmword ptr [r15 + rax*4 - 0x10], xmm0
14161f72a: movups xmmword ptr [r15 + rax*4], xmm0
14161f72f: add rax, 0x20
14161f733: cmp rax, 0x201c
14161f739: jne 0x14161f700
14161f73b: mov r12b, 1
14161f73e: mov ebx, 0x8000
14161f743: mov qword ptr [rbp + 0x98], r15
14161f74a: mov byte ptr [rbp + 0xa7], r12b
14161f751: lea rcx, [rbp + 0x10]
14161f755: mov edx, 0x8000
14161f75a: mov r8d, 0x22
14161f760: mov r9b, 4
14161f763: call 0x140bd4de0
14161f768: mov qword ptr [rbp + 0x98], r15
14161f76f: mov byte ptr [rbp + 0xa7], r12b
14161f776: lea r14, [rbp + 0x60]
14161f77a: lea rdx, [rbp + 0x10]
14161f77e: mov r8d, 4
14161f784: mov rcx, r14
14161f787: call 0x140bd4f20
14161f78c: mov qword ptr [rbp + 0x98], r15
14161f793: mov byte ptr [rbp + 0xa7], r12b
14161f79a: mov qword ptr [rsp + 0x30], rbx
14161f79f: mov qword ptr [rsp + 0x28], r15
14161f7a4: mov qword ptr [rsp + 0x20], r14
14161f7a9: lea r8, [rip + 0x1660278]
14161f7b0: lea rcx, [rbp - 0x40]
14161f7b4: mov r9d, 0xa
14161f7ba: mov rdx, rdi
14161f7bd: call 0x1416a2dc0
14161f7c2: cmp dword ptr [rbp - 0x40], 0xb
14161f7c6: jne 0x14161f929
14161f7cc: mov rax, qword ptr [rbp - 0x18]
14161f7d0: mov qword ptr [rbp + 0x80], rax
14161f7d7: movups xmm0, xmmword ptr [rbp - 0x38]
14161f7db: movups xmm1, xmmword ptr [rbp - 0x28]
14161f7df: movaps xmmword ptr [rbp + 0x70], xmm1
14161f7e3: movaps xmmword ptr [rbp + 0x60], xmm0
14161f7e7: test r12b, r12b
14161f7ea: je 0x14161f7ff
14161f7ec: mov edx, 0x8000
14161f7f1: mov r8d, 4
14161f7f7: mov rcx, r15
14161f7fa: call 0x140613c20
14161f7ff: lea rcx, [rbp + 0x60]
14161f803: mov rdx, rdi
14161f806: call 0x140bd4cd0
14161f80b: test rax, rax
14161f80e: je 0x14161f989
14161f814: cmp rdx, qword ptr [rbp + 0x78]
14161f818: jb 0x14161f873
14161f81a: mov rcx, rdx
14161f81d: and rcx, 3
14161f821: mov qword ptr [rbp - 0x40], rcx
14161f825: jne 0x14161f890
14161f827: mov rcx, rax
14161f82a: and rcx, 3
14161f82e: mov qword ptr [rbp - 0x40], rcx
14161f832: jne 0x14161f8bf
14161f838: mov rcx, qword ptr [rbp + 0x80]
14161f83f: mov qword ptr [rsi + 0x30], rcx
14161f843: movaps xmm0, xmmword ptr [rbp + 0x60]
14161f847: movaps xmm1, xmmword ptr [rbp + 0x70]
14161f84b: movups xmmword ptr [rsi + 0x20], xmm1
14161f84f: movups xmmword ptr [rsi + 0x10], xmm0
14161f853: shr rdx, 2
14161f857: mov qword ptr [rsi], rax
14161f85a: mov qword ptr [rsi + 8], rdx
14161f85e: mov rax, rsi
14161f861: add rsp, 0x130
14161f868: pop rbx
14161f869: pop rdi
14161f86a: pop rsi
14161f86b: pop r12
14161f86d: pop r14
14161f86f: pop r15
14161f871: pop rbp
14161f872: ret 
14161f873: lea rcx, [rip + 0x1660976]
14161f87a: lea r8, [rip + 0x16609a7]
14161f881: mov edx, 0x37
14161f886: call 0x1428d9430
14161f88b: jmp 0x14161fa21
14161f890: mov qword ptr [rbp + 0x10], 0
14161f898: lea rax, [rip + 0x16609a9]
14161f89f: mov qword ptr [rsp + 0x20], rax
14161f8a4: lea r8, [rip + 0x1660995]
14161f8ab: lea rdx, [rbp - 0x40]
14161f8af: lea r9, [rbp + 0x10]
14161f8b3: xor ecx, ecx
14161f8b5: call 0x1428d9596
14161f8ba: jmp 0x14161fa21
14161f8bf: mov qword ptr [rbp + 0x10], 0
14161f8c7: lea rax, [rip + 0x1660992]
14161f8ce: mov qword ptr [rsp + 0x20], rax
14161f8d3: lea r8, [rip + 0x1660966]
14161f8da: lea rdx, [rbp - 0x40]
14161f8de: lea r9, [rbp + 0x10]
14161f8e2: xor ecx, ecx
14161f8e4: call 0x1428d9596
14161f8e9: jmp 0x14161fa21
14161f8ee: mov qword ptr [rbp + 0x10], 0
14161f8f6: lea rax, [rip + 0x16aa64b]
14161f8fd: jmp 0x14161f90e
14161f8ff: mov qword ptr [rbp + 0x10], 0
14161f907: lea rax, [rip + 0x16aa652]
14161f90e: mov qword ptr [rsp + 0x20], rax
14161f913: lea r8, [rip + 0x16aa626]
14161f91a: lea rdx, [rbp - 0x40]
14161f91e: lea r9, [rbp + 0x10]
14161f922: xor ecx, ecx
14161f924: call 0x1428d9596
14161f929: mov dword ptr [rbp + 0x94], r12d
14161f930: mov qword ptr [rbp - 0x48], r15
14161f934: movups xmm0, xmmword ptr [rbp]
14161f938: movaps xmmword ptr [rbp + 0x50], xmm0
14161f93c: movups xmm0, xmmword ptr [rbp - 0x40]
14161f940: movups xmm1, xmmword ptr [rbp - 0x30]
14161f944: movups xmm2, xmmword ptr [rbp - 0x20]
14161f948: movups xmm3, xmmword ptr [rbp - 0x10]
14161f94c: movaps xmmword ptr [rbp + 0x40], xmm3
14161f950: movaps xmmword ptr [rbp + 0x30], xmm2
14161f954: movaps xmmword ptr [rbp + 0x20], xmm1
14161f958: movaps xmmword ptr [rbp + 0x10], xmm0
14161f95c: lea rax, [rip + 0x1660845]
14161f963: mov qword ptr [rsp + 0x20], rax
14161f968: lea rcx, [rip + 0x165f179]
14161f96f: lea r9, [rip + 0x165f152]
14161f976: lea r8, [rbp + 0x10]
14161f97a: mov edx, 0x2b
14161f97f: call 0x1428d9760
14161f984: jmp 0x14161fa21
14161f989: lea rcx, [rip + 0x1660848]
14161f990: call 0x1428d9310
14161f995: jmp 0x14161fa21
14161f99a: lea rax, [rip + 0x147efa0]
14161f9a1: mov qword ptr [rbp - 0x40], rax
14161f9a5: mov qword ptr [rbp - 0x38], 0x34
14161f9ad: lea rax, [rbp - 0x40]
14161f9b1: mov qword ptr [rbp + 0x60], rax
14161f9b5: lea rax, [rip - 0x1515b2c]
14161f9bc: mov qword ptr [rbp + 0x68], rax
14161f9c0: lea rax, [rip + 0x165fe01]
14161f9c7: mov qword ptr [rbp + 0x10], rax
14161f9cb: mov qword ptr [rbp + 0x18], 1
14161f9d3: mov qword ptr [rbp + 0x30], 0
14161f9db: lea rax, [rbp + 0x60]
14161f9df: mov qword ptr [rbp + 0x20], rax
14161f9e3: mov qword ptr [rbp + 0x28], 1
14161f9eb: mov byte ptr [rbp + 0xa7], 0
14161f9f2: lea rdx, [rip + 0x165fddf]
14161f9f9: lea rcx, [rbp + 0x10]
14161f9fd: call 0x1428d9390
14161fa02: jmp 0x14161fa21
14161fa04: mov byte ptr [rbp + 0xa7], 0
14161fa0b: lea r8, [rip + 0x165f09e]
14161fa12: mov ecx, 4
14161fa17: mov edx, 0x8000
14161fa1c: call 0x1428d8fe3
14161fa21: ud2 
14161fa23: nop word ptr cs:[rax + rax]
14161fa30: mov qword ptr [rsp + 0x10], rdx
14161fa35: push rbp
14161fa36: push r15
14161fa38: push r14
14161fa3a: push r12
14161fa3c: push rsi
14161fa3d: push rdi
14161fa3e: push rbx
14161fa3f: sub rsp, 0x40
14161fa43: lea rbp, [rdx + 0x80]
14161fa4a: mov rsi, qword ptr [rbp - 0x48]
14161fa4e: mov qword ptr [rbp + 0x98], rsi
14161fa55: mov edi, dword ptr [rbp + 0x94]
14161fa5b: mov byte ptr [rbp + 0xa7], dil
14161fa62: lea rcx, [rbp + 0x10]
14161fa66: call 0x1407cea70
14161fa6b: mov qword ptr [rbp + 0x98], rsi
14161fa72: mov byte ptr [rbp + 0xa7], dil
14161fa79: add rsp, 0x40
14161fa7d: pop rbx
14161fa7e: pop rdi
14161fa7f: pop rsi
14161fa80: pop r12
14161fa82: pop r14
14161fa84: pop r15
14161fa86: pop rbp
14161fa87: ret 
14161fa88: nop dword ptr [rax + rax]
14161fa90: mov qword ptr [rsp + 0x10], rdx
14161fa95: push rbp
14161fa96: push r15
14161fa98: push r14
14161fa9a: push r12
14161fa9c: push rsi
14161fa9d: push rdi
14161fa9e: push rbx
14161fa9f: sub rsp, 0x40
14161faa3: lea rbp, [rdx + 0x80]
14161faaa: test byte ptr [rbp + 0xa7], 1
14161fab1: je 0x14161faca
14161fab3: mov rcx, qword ptr [rbp + 0x98]
14161faba: mov edx, 0x8000
14161fabf: mov r8d, 4
14161fac5: call 0x140613c20
14161faca: nop 
14161facb: add rsp, 0x40
14161facf: pop rbx
14161fad0: pop rdi
14161fad1: pop rsi
14161fad2: pop r12
14161fad4: pop r14
14161fad6: pop r15
14161fad8: pop rbp
14161fad9: ret 
14161fada: nop word ptr [rax + rax]
14161fae0: mov qword ptr [rsp + 0x10], rdx
14161fae5: push rbp
14161fae6: push r15
14161fae8: push r14
14161faea: push r12
14161faec: push rsi
14161faed: push rdi
14161faee: push rbx
14161faef: sub rsp, 0x40
14161faf3: lea rbp, [rdx + 0x80]
14161fafa: lea rcx, [rbp + 0x60]
14161fafe: call 0x140be24b0
14161fb03: nop 
14161fb04: add rsp, 0x40
14161fb08: pop rbx
14161fb09: pop rdi
14161fb0a: pop rsi
14161fb0b: pop r12
14161fb0d: pop r14
14161fb0f: pop r15
14161fb11: pop rbp
14161fb12: ret 
